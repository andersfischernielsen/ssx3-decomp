//! SIO2: the IOP's serial controller for the controller and memory card
//! ports, with the devices plugged into it ([`crate::ds2`] on port 0,
//! [`crate::memcard`] in port 0 / slot 0).
//!
//! SIO2MAN drives it like this (per transfer, see `sio2_transfer` in the
//! recompiled SIO2MAN at `0x1d4`/`0x2fc`/`0x324`):
//!
//! 1. `CTRL |= 0xC` (reset the FIFOs and the command queue);
//! 2. `SEND1/SEND2` per port (baud rate / timing, ignored here) and
//!    `SEND3[0..16]`: the command queue, one word per packet:
//!    bits 0..1 port (0/1 pads, 2/3 memory cards), bits 8..17 packet length
//!    in bytes, bits 18..27 response length;
//! 3. packet bytes into the TX FIFO, either by `DATA_IN` writes (PIO) or by
//!    DMA channel 11 (one DMA block per packet);
//! 4. DMA channel 12 armed to drain the RX FIFO into IOP RAM;
//! 5. `CTRL |= 1` (start): the controller sends the queue and raises
//!    INTC line 17 when done; the handler acknowledges `ISTAT` and wakes
//!    the waiting thread, which reads `RECV1/2/3` (connection status) and,
//!    for PIO, the RX FIFO through `DATA_OUT`.
//!
//! The first byte of each packet selects the device class: `0x01` controller,
//! `0x21` multitap, `0x61` infrared, `0x81` memory card. The response has one
//! byte per packet byte (the device answers while the host clocks bytes
//! out). Packet framing, the RECV1 status values and the DMA block alignment
//! of the RX FIFO follow PCSX2's `Sio2.cpp` (behaviour, not code); the
//! register map follows ps2-web-recomp `runtime/src/iop/sio2.cpp` (MIT).
//!
//! Timing is not modelled: a transfer completes as soon as it is started.

use std::collections::VecDeque;

use crate::ds2::DualShock2;
use crate::memcard::MemoryCard;

pub const LINE_SIO2: u32 = 17;
pub const BASE: u32 = 0x1F80_8200;
pub const END: u32 = 0x1F80_8290;

/// RECV1 status bits.
pub mod stat {
    pub const CONNECTED: u32 = 0x1100;
    pub const DISCONNECTED: u32 = 0x1_D100;
    pub const NO_DEVICES_MISSING: u32 = 0x1000;
    pub const PORT_1_MISSING: u32 = 0x1_D000;
    pub const PORT_2_MISSING: u32 = 0x2_D000;
    pub const ONE_PORT_OPEN: u32 = 0x100;
    pub const TWO_PORTS_OPEN: u32 = 0x200;
}

pub struct Sio2 {
    pub send3: [u32; 16],
    pub send1: [u32; 4],
    pub send2: [u32; 4],
    pub ctrl: u32,
    pub recv1: u32,
    pub recv2: u32,
    pub recv3: u32,
    pub istat: u32,
    tx: Vec<u8>,
    rx: VecDeque<u8>,
    /// Command queue position and the length of the packet being received.
    queue_pos: usize,
    cmd_len: usize,
    cmd_port: u32,
    in_packet: bool,
    queue_done: bool,
    /// Bytes per DMA block while channel 11 feeds the TX FIFO (0 for PIO).
    dma_block: usize,
    pub pads: [DualShock2; 2],
    /// Memory card in port 0 (slot 0); `None`: empty slot.
    pub card: Option<MemoryCard>,
    pub trace: bool,
    pub transfers: u64,
    pub packets: u64,
}

impl Default for Sio2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sio2 {
    pub fn new() -> Self {
        let mut pads = [DualShock2::new(), DualShock2::new()];
        // Pad 2 is unplugged unless SSX3_PAD2=1 (Multi Play needs it; the
        // windowed front end maps a second game controller to it).
        pads[1].connected = std::env::var_os("SSX3_PAD2").is_some_and(|v| v != "0");
        Self {
            send3: [0; 16],
            send1: [0; 4],
            send2: [0; 4],
            ctrl: 0x3BC,
            recv1: stat::DISCONNECTED,
            recv2: 0xF,
            recv3: 0,
            istat: 0,
            tx: Vec::new(),
            rx: VecDeque::new(),
            queue_pos: 0,
            cmd_len: 0,
            cmd_port: 0,
            in_packet: false,
            queue_done: false,
            dma_block: 0,
            pads,
            card: None,
            trace: std::env::var("SSX3_TRACE_SIO2").is_ok(),
            transfers: 0,
            packets: 0,
        }
    }

    /// New command queue (SEND3[0] written, or CTRL reset).
    fn queue_reset(&mut self) {
        self.queue_pos = 0;
        self.cmd_len = 0;
        self.in_packet = false;
        self.queue_done = false;
        self.dma_block = 0;
        self.tx.clear();
        self.recv1 = 0;
    }

    /// One byte into the TX FIFO.
    pub fn write_byte(&mut self, b: u8) {
        if !self.in_packet {
            if self.queue_pos >= self.send3.len() {
                return;
            }
            let cmd = self.send3[self.queue_pos];
            self.cmd_port = cmd & 3;
            self.cmd_len = ((cmd >> 8) & 0x1FF) as usize;
            self.in_packet = true;
            self.queue_done = self.cmd_len == 0;
            self.tx.clear();
        }
        if self.queue_done {
            return;
        }
        self.tx.push(b);
        let want = if self.dma_block > 0 { self.dma_block } else { self.cmd_len };
        if self.tx.len() >= want {
            self.in_packet = false;
            self.queue_pos += 1;
            let packet = std::mem::take(&mut self.tx);
            // A DMA block may be longer than the packet: only the packet
            // length goes to the device.
            let n = self.cmd_len.min(packet.len()).max(1);
            self.packet(&packet[..n]);
            if self.dma_block > 0 {
                let pad = (self.dma_block - self.rx.len() % self.dma_block) % self.dma_block;
                self.rx.extend(std::iter::repeat(0).take(pad));
            }
        }
    }

    /// Dispatch one complete packet to the addressed device.
    fn packet(&mut self, p: &[u8]) {
        self.packets += 1;
        let port = self.cmd_port;
        let out: Vec<u8> = match p[0] {
            0x01 => {
                if self.recv1 & stat::ONE_PORT_OPEN != 0 {
                    self.recv1 = (self.recv1 & !stat::ONE_PORT_OPEN) | stat::TWO_PORTS_OPEN;
                } else {
                    self.recv1 |= stat::ONE_PORT_OPEN;
                }
                self.recv1 |= stat::NO_DEVICES_MISSING;
                let pad = &mut self.pads[(port & 1) as usize];
                if !pad.connected {
                    self.recv1 |= if port & 1 == 0 { stat::PORT_1_MISSING } else { stat::PORT_2_MISSING };
                }
                pad.transfer(p)
            }
            0x81 => {
                // Memory cards: port 2 is the card in controller port 1.
                let card = if port & 1 == 0 { self.card.as_mut() } else { None };
                match card {
                    Some(card) => {
                        self.recv1 = stat::CONNECTED;
                        card.transfer(p)
                    }
                    None => {
                        self.recv1 = stat::DISCONNECTED;
                        vec![0xFF; p.len()]
                    }
                }
            }
            _ => {
                // Multitap (0x21), infrared (0x61): not attached.
                self.recv1 = stat::DISCONNECTED;
                vec![0xFF; p.len()]
            }
        };
        if self.trace {
            let hex = |v: &[u8]| v.iter().take(24).map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
            eprintln!("[sio2] port {port} {} -> {} (recv1 {:#x})", hex(p), hex(&out), self.recv1);
        }
        self.rx.extend(out);
    }

    pub fn read_byte(&mut self) -> u8 {
        self.rx.pop_front().unwrap_or(0xFF)
    }

    /// Register read; `None` outside the SIO2 range.
    pub fn read(&mut self, a: u32) -> Option<u32> {
        if !(BASE..END).contains(&a) {
            return None;
        }
        let o = a - BASE;
        if self.trace && (o == 0x6C || o == 0x64 || o == 0x74) {
            eprintln!("[sio2] read {o:#x} (rx left {})", self.rx.len());
        }
        Some(match o {
            0x00..=0x3F => self.send3[(o / 4) as usize],
            0x40..=0x5F => {
                let i = ((o - 0x40) / 8) as usize;
                if o & 4 != 0 {
                    self.send2[i]
                } else {
                    self.send1[i]
                }
            }
            0x64 => self.read_byte() as u32,
            0x68 => self.ctrl,
            0x6C => self.recv1,
            0x70 => self.recv2,
            0x74 => self.recv3,
            0x80 => self.istat,
            _ => 0,
        })
    }

    /// Register write. Returns true when the write started a transfer (the
    /// caller raises the interrupt).
    pub fn write(&mut self, a: u32, v: u32) -> bool {
        let o = a - BASE;
        match o {
            0x00..=0x3F => {
                let i = (o / 4) as usize;
                self.send3[i] = v;
                if i == 0 {
                    self.queue_reset();
                }
            }
            0x40..=0x5F => {
                let i = ((o - 0x40) / 8) as usize;
                if o & 4 != 0 {
                    self.send2[i] = v;
                } else {
                    self.send1[i] = v;
                }
            }
            0x60 => self.write_byte(v as u8),
            0x68 => {
                self.ctrl = v;
                // SIO2MAN sets the reset bits (0xC) before a transfer and
                // leaves them set in the start write (`ctrl | 1`): only a
                // write without the start bit resets.
                if v & 0xC == 0xC && v & 1 == 0 {
                    self.queue_reset();
                    self.rx.clear();
                }
                if v & 1 != 0 {
                    self.transfers += 1;
                    self.istat |= 1;
                    return true;
                }
            }
            0x6C => self.recv1 = v,
            0x70 => self.recv2 = v,
            0x74 => self.recv3 = v,
            0x80 => self.istat &= !v,
            _ => {}
        }
        false
    }

    /// DMA channel 11 (memory → SIO2): `blocks` blocks of `words` words.
    pub fn dma_in(&mut self, data: &[u8], block_bytes: usize) {
        self.dma_block = block_bytes;
        for &b in data {
            self.write_byte(b);
        }
    }

    /// DMA channel 12 (SIO2 → memory).
    pub fn dma_out(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.read_byte()).collect()
    }

    pub fn describe(&self) -> String {
        let card = match &self.card {
            Some(c) => format!("card {}", c.describe()),
            None => "no card".into(),
        };
        format!("sio2 {} xfers {} pkts, pad0 mode {:#x}, {card}", self.transfers, self.packets, self.pads[0].mode_id())
    }
}
