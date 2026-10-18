//! IPU — the EE's Image Processing Unit (MPEG-2 macroblock decoder).
//!
//! SSX 3 plays its movies (`DATA/MOVIES/*.MPC`, EA "Gimex" chunks around an
//! MPEG-2 video elementary stream) through Sony's libmpeg 2.7, which parses
//! headers with FDEC/VDEC, decodes macroblocks with BDEC, does motion
//! compensation on the EE and converts finished pictures with CSC.
//!
//! Model: a byte-stream input FIFO fed by toIPU DMA (ch4, *pulled* as the
//! decoder needs data — see [`dma`]) or by 128-bit writes to `0x1000_7010`,
//! and an output queue drained into fromIPU DMA (ch3) or read from
//! `0x1000_7000`. Commands execute synchronously when written; a command
//! that runs out of input rolls back to its last macroblock boundary, stays
//! BUSY, and resumes whenever more data arrives. On completion INTC bit 8
//! (IPU) is raised.
//!
//! Port of ps2-web-recomp's `runtime/src/ipu/ipu.cpp` (MIT, Copyright (c)
//! 2026 Samir Nuri), with PCSX2's CSC/dither/VQ/threshold semantics. See
//! `docs/investigation/ipu.md`.

pub mod dma;

pub use ssx3_mpeg::{csc, idct, tables};
use ssx3_mpeg::{dequantise, luts, DctLookup, Lookup, DEFAULT_INTRA, NON_LINEAR_SCALE};

#[cfg(test)]
pub(crate) mod tests;


use crate::Qword;

/// The IPU's view of its DMA channels.
pub trait IpuDma {
    /// Pull one quadword from toIPU (ch4); `None` when it has nothing to give.
    fn pull(&mut self) -> Option<Qword>;
    /// Offer one quadword to fromIPU (ch3); `false` if it can't take it now.
    fn push(&mut self, q: Qword) -> bool;
}

/// No DMA connected (CPU FIFO access, tests).
pub struct NoDma;

impl IpuDma for NoDma {
    fn pull(&mut self) -> Option<Qword> {
        None
    }
    fn push(&mut self, _: Qword) -> bool {
        false
    }
}

/// Register offsets from `0x1000_2000`.
pub mod reg {
    pub const CMD: u32 = 0x00;
    pub const CMD_BUSY: u32 = 0x04;
    pub const CTRL: u32 = 0x10;
    pub const BP: u32 = 0x20;
    pub const TOP: u32 = 0x30;
    pub const TOP_BUSY: u32 = 0x34;
}

/// Command codes (IPU_CMD bits 28-31).
pub mod cmd {
    pub const BCLR: u32 = 0;
    pub const IDEC: u32 = 1;
    pub const BDEC: u32 = 2;
    pub const VDEC: u32 = 3;
    pub const FDEC: u32 = 4;
    pub const SETIQ: u32 = 5;
    pub const SETVQ: u32 = 6;
    pub const CSC: u32 = 7;
    pub const PACK: u32 = 8;
    pub const SETTH: u32 = 9;
}

/// IPU_CTRL bits.
pub mod ctrl {
    pub const ECD: u32 = 1 << 14;
    pub const SCD: u32 = 1 << 15;
    pub const AS: u32 = 1 << 20;
    pub const IVF: u32 = 1 << 21;
    pub const QST: u32 = 1 << 22;
    pub const MP1: u32 = 1 << 23;
    pub const RST: u32 = 1 << 30;
    pub const BUSY: u32 = 1 << 31;
    /// Bits written by the CPU: IDP, AS, IVF, QST, MP1, PCT.
    pub const WRITABLE: u32 = 0x3FFF_0000;
}

/// Input FIFO depth in bytes (8 quadwords).
const FIFO_BYTES: usize = 128;

// -------------------------------------------------------------------- state

/// Why a decode step stopped early.
enum Stop {
    /// Input ran out; roll back and retry when more arrives.
    NeedData,
    /// Invalid VLC: the command ends with ECD set.
    BadCode,
}

type R<T> = Result<T, Stop>;

/// Command counters, for diagnostics.
#[derive(Default, Clone, Debug)]
pub struct IpuStats {
    pub commands: [u64; 16],
    pub stalls: u64,
    pub out_qwords: u64,
    pub bad_codes: u64,
}

pub struct Ipu {
    /// Input bytes as they arrived (quadword-granular); `bit` is the read
    /// position. Whole consumed quadwords are dropped by `commit`.
    inb: Vec<u8>,
    bit: usize,
    /// Output bytes not yet delivered; `out_pos` is the delivered prefix.
    out: Vec<u8>,
    out_pos: usize,
    ctrl: u32,
    result: u32,
    command: u32,
    busy: bool,
    decoding: bool,
    starved: bool,
    /// IDEC sub-state and per-command scratch.
    phase: u8,
    qsc: u32,
    count: u32,
    first: bool,
    intra_iq: [u8; 64],
    nonintra_iq: [u8; 64],
    vqclut: [u16; 16],
    th0: u16,
    th1: u16,
    dc: [i32; 3],
    irq: bool,
    /// See [`dma::HwPort`]: toIPU's current tag was its last.
    pub(crate) dma4_tag_end: bool,
    pub stats: IpuStats,
    /// Trace commands to stderr (`SSX3_TRACE_IPU`).
    pub trace: bool,
}

impl Default for Ipu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ipu {
    pub fn new() -> Self {
        Self {
            inb: Vec::new(),
            bit: 0,
            out: Vec::new(),
            out_pos: 0,
            ctrl: 0,
            result: 0,
            command: 0,
            busy: false,
            decoding: false,
            starved: false,
            phase: 0,
            qsc: 0,
            count: 0,
            first: true,
            intra_iq: DEFAULT_INTRA,
            nonintra_iq: [16; 64],
            vqclut: [0; 16],
            th0: 0,
            th1: 0,
            dc: [0; 3],
            irq: false,
            dma4_tag_end: false,
            stats: IpuStats::default(),
            trace: std::env::var_os("SSX3_TRACE_IPU").is_some(),
        }
    }

    /// Whether a command is still executing (or its output is undelivered).
    pub fn busy(&self) -> bool {
        self.busy
    }

    /// A command completed since the last call (INTC IPU).
    pub fn take_irq(&mut self) -> bool {
        std::mem::take(&mut self.irq)
    }

    /// Quadwords of output waiting to be read.
    pub fn out_pending(&self) -> usize {
        (self.out.len() - self.out_pos) / 16
    }

    /// Unconsumed input quadwords, counting a partially consumed one.
    fn in_qwords(&self) -> usize {
        (self.inb.len() / 16).saturating_sub(self.bit / 128)
    }

    /// Unread input bits.
    fn avail(&self) -> usize {
        (self.inb.len() * 8).saturating_sub(self.bit)
    }

    /// Up to 32 bits at `bit`, MSB first, zero-padded past the end.
    fn bits_at(&self, bit: usize, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let byte = bit >> 3;
        let mut v = 0u64;
        for i in 0..8 {
            v = v << 8 | *self.inb.get(byte + i).unwrap_or(&0) as u64;
        }
        ((v << (bit & 7)) >> (64 - n)) as u32
    }

    /// The next 32 bits as currently buffered (IPU_TOP / idle IPU_CMD).
    fn peek_now(&self) -> u32 {
        self.bits_at(self.bit, 32)
    }

    // ------------------------------------------------------------ registers

    /// Read an IPU register (`off` relative to `0x1000_2000`).
    pub fn read32(&self, off: u32) -> u32 {
        match off & 0xFFC {
            reg::CMD => {
                let last = self.command >> 28;
                if last == cmd::VDEC || last == cmd::FDEC {
                    self.result
                } else {
                    self.peek_now()
                }
            }
            reg::CMD_BUSY | reg::TOP_BUSY if self.busy => ctrl::BUSY,
            reg::TOP_BUSY if self.avail() < 32 => ctrl::BUSY,
            reg::CTRL => {
                let ifc = self.in_qwords().min(8) as u32;
                let ofc = self.out_pending().min(8) as u32;
                let cbp = self.ctrl & 0x3F00;
                (self.ctrl & !0x8000_3FFF) | cbp | ifc | ofc << 4 | (self.busy as u32) << 31
            }
            reg::BP => {
                // BP: bit offset into the current quadword; FP: quadwords in
                // the bit-pointer stage; IFC: the rest of the FIFO.
                let bp = (self.bit & 127) as u32;
                let mut ifc = self.in_qwords() as u32;
                let fp = (bp != 0 && ifc != 0) as u32;
                ifc -= fp;
                fp << 16 | ifc.min(15) << 8 | bp
            }
            reg::TOP => self.peek_now(),
            _ => 0,
        }
    }

    /// Write an IPU register. Writing IPU_CMD starts a command.
    pub fn write32(&mut self, off: u32, v: u32, dma: &mut dyn IpuDma) {
        match off & 0xFFC {
            reg::CMD => {
                self.ctrl &= !(ctrl::ECD | ctrl::SCD);
                self.starved = false;
                self.stats.commands[(v >> 28) as usize] += 1;
                self.command = v;
                self.busy = true;
                self.decoding = true;
                self.phase = 0;
                self.qsc = (v >> 16) & 0x1F;
                self.first = true;
                self.count = v & 0x7FF;
                self.step(dma);
                if self.trace {
                    eprintln!(
                        "[ipu] cmd {v:08X} -> result {:08X} ctrl {:08X} busy {} bit {} next {:08X}",
                        self.result,
                        self.ctrl,
                        self.busy,
                        self.bit,
                        self.peek_now()
                    );
                }
            }
            reg::CTRL => {
                if v & ctrl::RST != 0 {
                    self.inb.clear();
                    self.out.clear();
                    self.bit = 0;
                    self.out_pos = 0;
                    self.busy = false;
                    self.decoding = false;
                    self.starved = false;
                    self.ctrl = 0;
                    self.command = 0;
                    self.result = 0;
                    self.th0 = 0;
                    self.th1 = 0;
                }
                self.ctrl = (self.ctrl & !ctrl::WRITABLE) | (v & ctrl::WRITABLE);
            }
            _ => {}
        }
    }

    /// Quadwords written to the input FIFO (`0x1000_7010`).
    pub fn write_fifo(&mut self, qs: &[Qword], dma: &mut dyn IpuDma) {
        for q in qs {
            self.inb.extend_from_slice(&q.to_le_bytes());
        }
        self.step(dma);
    }

    /// Read up to `n` quadwords from the output FIFO (`0x1000_7000`).
    pub fn read_fifo(&mut self, n: usize, dma: &mut dyn IpuDma) -> Vec<Qword> {
        let mut v = Vec::with_capacity(n);
        while v.len() < n && self.out_pos + 16 <= self.out.len() {
            let q = &self.out[self.out_pos..self.out_pos + 16];
            v.push(u128::from_le_bytes(q.try_into().unwrap()));
            self.out_pos += 16;
        }
        self.step(dma);
        v
    }

    /// Advance: resume a starved command, deliver output, refill the FIFO.
    /// Call whenever the DMA channels change (start of ch3/ch4).
    pub fn step(&mut self, dma: &mut dyn IpuDma) {
        if self.busy && self.decoding {
            let mut ctx = Ctx { ipu: self, dma };
            match ctx.run_command() {
                Ok(true) => self.decoding = false,
                Ok(false) => self.starved = true,
                Err(_) => {
                    self.stats.bad_codes += 1;
                    if self.trace {
                        eprintln!("[ipu] bad code in command {:08X}", self.command);
                    }
                    self.ctrl |= ctrl::ECD;
                    self.decoding = false;
                }
            }
            if !self.decoding {
                self.starved = false;
            }
        }
        // Deliver output to fromIPU.
        while self.out_pos + 16 <= self.out.len() {
            let q = u128::from_le_bytes(
                self.out[self.out_pos..self.out_pos + 16]
                    .try_into()
                    .unwrap(),
            );
            if !dma.push(q) {
                break;
            }
            self.out_pos += 16;
            self.stats.out_qwords += 1;
        }
        if self.out_pos == self.out.len() {
            self.out.clear();
            self.out_pos = 0;
        }
        if self.busy && !self.decoding && self.out.is_empty() {
            self.busy = false;
            self.irq = true;
        }
        // Keep the input FIFO topped up from toIPU.
        while self.in_qwords() < FIFO_BYTES / 16 {
            match dma.pull() {
                Some(q) => self.inb.extend_from_slice(&q.to_le_bytes()),
                None => break,
            }
        }
    }

    fn dc_precision(&self) -> u32 {
        (self.ctrl >> 16) & 3
    }

    fn mpeg1(&self) -> bool {
        self.ctrl & ctrl::MP1 != 0
    }

    fn picture_type(&self) -> u32 {
        (self.ctrl >> 24) & 7
    }
}

// ------------------------------------------------------------------ decoder

/// The IPU with its DMA attached, for the duration of one step.
struct Ctx<'a> {
    ipu: &'a mut Ipu,
    dma: &'a mut dyn IpuDma,
}

impl Ctx<'_> {
    fn pull(&mut self) -> bool {
        match self.dma.pull() {
            Some(q) => {
                self.ipu.inb.extend_from_slice(&q.to_le_bytes());
                true
            }
            None => false,
        }
    }

    /// Ensure `n` unread bits are buffered, pulling from DMA as needed.
    fn have(&mut self, n: usize) -> bool {
        while self.ipu.avail() < n {
            if !self.pull() {
                return false;
            }
        }
        true
    }

    fn peek(&mut self, n: u32) -> R<u32> {
        if !self.have(n as usize) {
            return Err(Stop::NeedData);
        }
        Ok(self.ipu.bits_at(self.ipu.bit, n))
    }

    fn get(&mut self, n: u32) -> R<u32> {
        let v = self.peek(n)?;
        self.ipu.bit += n as usize;
        Ok(v)
    }

    fn skip(&mut self, n: u32) -> R<()> {
        if !self.have(n as usize) {
            return Err(Stop::NeedData);
        }
        self.ipu.bit += n as usize;
        Ok(())
    }

    /// Peek `n` bits without failing; returns (bits zero-padded, available).
    fn peek_soft(&mut self, n: u32) -> (u32, usize) {
        self.have(n as usize);
        (self.ipu.bits_at(self.ipu.bit, n), self.ipu.avail())
    }

    fn decode(&mut self, l: &Lookup) -> R<u32> {
        let (idx, avail) = self.peek_soft(l.bits);
        let (len, value) = l.entries[idx as usize];
        if len == 0 {
            return Err(if avail < l.bits as usize {
                Stop::NeedData
            } else {
                Stop::BadCode
            });
        }
        if len as usize > avail {
            return Err(Stop::NeedData);
        }
        self.ipu.bit += len as usize;
        Ok(value)
    }

    /// Next 23 bits are zero: a start code (`00 00 01`) follows.
    fn start_code_follows(&mut self) -> bool {
        let (v, avail) = self.peek_soft(23);
        avail >= 23 && v == 0
    }

    /// Drop whole consumed quadwords from the input buffer.
    fn commit(&mut self) {
        let q = (self.ipu.bit / 128).min(self.ipu.inb.len() / 16);
        if q > 0 {
            self.ipu.inb.drain(..q * 16);
            self.ipu.bit -= q * 128;
        }
    }

    /// Run `f`; if it runs out of input, undo its effects and return `None`.
    fn atomic<T>(&mut self, f: impl FnOnce(&mut Self) -> R<T>) -> R<Option<T>> {
        let (bit, dc, out) = (self.ipu.bit, self.ipu.dc, self.ipu.out.len());
        match f(self) {
            Ok(v) => Ok(Some(v)),
            Err(Stop::NeedData) => {
                self.ipu.stats.stalls += 1;
                self.ipu.bit = bit;
                self.ipu.dc = dc;
                self.ipu.out.truncate(out);
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    // ----------------------------------------------------------- blocks

    fn read_dc_diff(&mut self, chroma: bool) -> R<i32> {
        let l = luts();
        let size = self.decode(if chroma { &l.dc_chroma } else { &l.dc_luma })? & 0xFFFF;
        if size == 0 {
            return Ok(0);
        }
        let v = self.get(size)? as i32;
        Ok(if v < 1 << (size - 1) {
            v + 1 - (1 << size)
        } else {
            v
        })
    }

    /// One run/level pair (the caller has already ruled out end-of-block).
    fn read_coefficient(&mut self, l: &DctLookup) -> R<(usize, i32)> {
        let (idx, avail) = self.peek_soft(l.bits);
        let (len, run, level) = l.entries[idx as usize];
        if len == 0 {
            return Err(if avail < l.bits as usize {
                Stop::NeedData
            } else {
                Stop::BadCode
            });
        }
        if len as usize > avail {
            return Err(Stop::NeedData);
        }
        self.ipu.bit += len as usize;
        if run == tables::ESCAPE {
            let run = self.get(6)? as usize;
            let level = if self.ipu.mpeg1() {
                let v = self.get(8)? as i32;
                match v {
                    0 => self.get(8)? as i32,
                    128 => self.get(8)? as i32 - 256,
                    v if v > 128 => v - 256,
                    v => v,
                }
            } else {
                let v = self.get(12)? as i32;
                if v & 0x800 != 0 {
                    v - 0x1000
                } else {
                    v
                }
            };
            return Ok((run, level));
        }
        let level = level as i32;
        Ok((run as usize, if self.get(1)? != 0 { -level } else { level }))
    }

    /// Decode, dequantise and inverse-transform one 8x8 block.
    fn decode_block(&mut self, out: &mut [i16; 64], comp: usize, intra: bool, qsc: u32) -> R<()> {
        let mut coef = [0i32; 64];
        let mut index = 0usize;
        let table_one = intra && self.ipu.ctrl & ctrl::IVF != 0;
        let l = luts();
        let table = if table_one { &l.dct1 } else { &l.dct0 };
        if intra {
            self.ipu.dc[comp] += self.read_dc_diff(comp != 0)?;
            coef[0] = self.ipu.dc[comp];
            index = 1;
        } else if self.peek(1)? != 0 {
            // First coefficient of a non-intra block: "1s" is run 0, level ±1.
            coef[0] = if self.get(2)? & 1 != 0 { -1 } else { 1 };
            index = 1;
        }
        loop {
            if index > 0 {
                let eob = if table_one {
                    self.peek(4)? == 0b0110
                } else {
                    self.peek(2)? == 0b10
                };
                if eob {
                    self.ipu.bit += if table_one { 4 } else { 2 };
                    break;
                }
            }
            let (run, level) = self.read_coefficient(table)?;
            index += run;
            if index >= 64 {
                return Err(Stop::BadCode);
            }
            coef[index] = level;
            index += 1;
        }
        let ipu = &*self.ipu;
        let scan = if ipu.ctrl & ctrl::AS != 0 {
            &tables::ALTERNATE
        } else {
            &tables::ZIGZAG
        };
        let weights = if intra {
            &ipu.intra_iq
        } else {
            &ipu.nonintra_iq
        };
        let mpeg1 = ipu.mpeg1();
        let scale = if mpeg1 || ipu.ctrl & ctrl::QST == 0 {
            qsc as i32 * 2
        } else {
            NON_LINEAR_SCALE[(qsc & 31) as usize] as i32
        };
        let f = dequantise(&coef, scan, weights, intra, scale, 8 >> ipu.dc_precision(), mpeg1);
        idct::idct(&f, out);
        Ok(())
    }

    /// Decode one macroblock's six blocks into RAW16 layout (256 Y in
    /// frame order, 64 Cb, 64 Cr). Returns the coded block pattern.
    fn decode_macroblock(
        &mut self,
        raw: &mut [i16; 384],
        intra: bool,
        field: bool,
        dc_reset: bool,
        qsc: u32,
    ) -> R<u32> {
        let cbp = if intra {
            0x3F
        } else {
            self.decode(&luts().cbp)? & 0xFF
        };
        if dc_reset {
            let v = 128 << self.ipu.dc_precision();
            self.ipu.dc = [v; 3];
        }
        let mut blocks = [[0i16; 64]; 6];
        for (b, blk) in blocks.iter_mut().enumerate() {
            if cbp & (32 >> b) != 0 {
                self.decode_block(blk, b.saturating_sub(3), intra, qsc)?;
            }
        }
        for i in 0..8 {
            let (top, bottom) = if field {
                (2 * i, 2 * i + 1)
            } else {
                (i, i + 8)
            };
            raw[top * 16..top * 16 + 8].copy_from_slice(&blocks[0][i * 8..i * 8 + 8]);
            raw[top * 16 + 8..top * 16 + 16].copy_from_slice(&blocks[1][i * 8..i * 8 + 8]);
            raw[bottom * 16..bottom * 16 + 8].copy_from_slice(&blocks[2][i * 8..i * 8 + 8]);
            raw[bottom * 16 + 8..bottom * 16 + 16].copy_from_slice(&blocks[3][i * 8..i * 8 + 8]);
        }
        raw[256..320].copy_from_slice(&blocks[4]);
        raw[320..384].copy_from_slice(&blocks[5]);
        Ok(cbp)
    }

    // ---------------------------------------------------------- output

    fn emit_raw16(&mut self, raw: &[i16; 384]) {
        for v in raw {
            self.ipu.out.extend_from_slice(&v.to_le_bytes());
        }
    }

    /// CSC one RAW8 macroblock to RGB32 (OFM=0) or RGB16 (OFM=1).
    fn emit_csc(&mut self, raw: &csc::Raw8, rgb16: bool, dte: bool, sgn: bool) {
        let rgb = csc::ycbcr_to_rgb32(raw, self.ipu.th0, self.ipu.th1, sgn);
        if rgb16 {
            for p in csc::rgb32_to_rgb16(&rgb, dte) {
                self.ipu.out.extend_from_slice(&p.to_le_bytes());
            }
        } else {
            for p in rgb {
                self.ipu.out.extend_from_slice(&p.to_le_bytes());
            }
        }
    }

    // --------------------------------------------------------- commands

    /// Run (or resume) the current command. `Ok(false)`: starved.
    fn run_command(&mut self) -> R<bool> {
        let v = self.ipu.command;
        let fb = v & 0x3F;
        match v >> 28 {
            cmd::BCLR => {
                self.ipu.inb.clear();
                self.ipu.bit = (v & 0x7F) as usize;
                Ok(true)
            }
            cmd::IDEC => self.run_idec(),
            cmd::BDEC => {
                let intra = v & (1 << 27) != 0;
                let dcr = v & (1 << 26) != 0;
                let dt = v & (1 << 25) != 0;
                let qsc = (v >> 16) & 0x1F;
                let r = self.atomic(|c| {
                    c.skip(fb)?;
                    let mut raw = [0i16; 384];
                    let cbp = c.decode_macroblock(&mut raw, intra, dt, dcr, qsc)?;
                    c.emit_raw16(&raw);
                    Ok(cbp)
                })?;
                let Some(cbp) = r else { return Ok(false) };
                self.ipu.ctrl = (self.ipu.ctrl & !0x3F00) | cbp << 8;
                self.commit();
                if self.start_code_follows() {
                    self.ipu.ctrl |= ctrl::SCD;
                }
                Ok(true)
            }
            cmd::VDEC => {
                let tbl = (v >> 26) & 3;
                let pct = self.ipu.picture_type();
                let r = self.atomic(|c| {
                    c.skip(fb)?;
                    let l = luts();
                    let table = match tbl {
                        0 => &l.mbai,
                        1 => match pct {
                            1 => &l.mb_i,
                            2 => &l.mb_p,
                            3 => &l.mb_b,
                            _ => return Err(Stop::BadCode),
                        },
                        2 => &l.motion,
                        _ => &l.dmv,
                    };
                    match c.decode(table) {
                        Err(Stop::BadCode) => Ok(None),
                        r => r.map(Some),
                    }
                })?;
                let Some(res) = r else { return Ok(false) };
                self.commit();
                match res {
                    Some(val) => self.ipu.result = val,
                    None => {
                        // Not a valid code: a start code (end of slice) or an error.
                        self.ipu.result = 0;
                        if self.start_code_follows() {
                            self.ipu.ctrl |= ctrl::SCD;
                        } else {
                            self.ipu.ctrl |= ctrl::ECD;
                        }
                    }
                }
                Ok(true)
            }
            cmd::FDEC => {
                let r = self.atomic(|c| {
                    c.skip(fb)?;
                    c.peek(32)
                })?;
                let Some(val) = r else { return Ok(false) };
                self.ipu.result = val;
                self.commit();
                if val >> 8 == 1 {
                    self.ipu.ctrl |= ctrl::SCD;
                }
                Ok(true)
            }
            cmd::SETIQ => {
                let r = self.atomic(|c| {
                    c.skip(fb)?;
                    let mut m = [0u8; 64];
                    for x in m.iter_mut() {
                        *x = c.get(8)? as u8;
                    }
                    Ok(m)
                })?;
                let Some(m) = r else { return Ok(false) };
                // The matrix arrives in zigzag order; store it by raster position.
                let dst = if v & (1 << 27) != 0 {
                    &mut self.ipu.nonintra_iq
                } else {
                    &mut self.ipu.intra_iq
                };
                for (k, &x) in m.iter().enumerate() {
                    dst[tables::ZIGZAG[k] as usize] = x;
                }
                self.commit();
                Ok(true)
            }
            cmd::SETVQ => {
                let r = self.atomic(|c| {
                    c.skip(fb)?;
                    let mut t = [0u16; 16];
                    for x in t.iter_mut() {
                        // Little-endian halfwords in memory order.
                        let lo = c.get(8)?;
                        let hi = c.get(8)?;
                        *x = (lo | hi << 8) as u16;
                    }
                    Ok(t)
                })?;
                let Some(t) = r else { return Ok(false) };
                self.ipu.vqclut = t;
                self.commit();
                Ok(true)
            }
            cmd::CSC => {
                let rgb16 = v & (1 << 27) != 0;
                let dte = v & (1 << 26) != 0;
                while self.ipu.count > 0 {
                    let r = self.atomic(|c| {
                        let mut raw = [0u8; 384];
                        for x in raw.iter_mut() {
                            *x = c.get(8)? as u8;
                        }
                        c.emit_csc(&raw, rgb16, dte, false);
                        Ok(())
                    })?;
                    if r.is_none() {
                        return Ok(false);
                    }
                    self.commit();
                    self.ipu.count -= 1;
                }
                Ok(true)
            }
            cmd::PACK => {
                let rgb16 = v & (1 << 27) != 0;
                let dte = v & (1 << 26) != 0;
                while self.ipu.count > 0 {
                    let r = self.atomic(|c| {
                        let mut rgb = [0u32; 256];
                        for x in rgb.iter_mut() {
                            let b = [c.get(8)?, c.get(8)?, c.get(8)?, c.get(8)?];
                            *x = b[0] | b[1] << 8 | b[2] << 16 | b[3] << 24;
                        }
                        Ok(rgb)
                    })?;
                    let Some(rgb) = r else { return Ok(false) };
                    let p16 = csc::rgb32_to_rgb16(&rgb, dte);
                    if rgb16 {
                        for p in p16 {
                            self.ipu.out.extend_from_slice(&p.to_le_bytes());
                        }
                    } else {
                        let idx = csc::rgb16_to_indx4(&p16, &self.ipu.vqclut);
                        self.ipu.out.extend_from_slice(&idx);
                    }
                    self.commit();
                    self.ipu.count -= 1;
                }
                Ok(true)
            }
            cmd::SETTH => {
                self.ipu.th0 = (v & 0x1FF) as u16;
                self.ipu.th1 = ((v >> 16) & 0x1FF) as u16;
                Ok(true)
            }
            _ => {
                if self.ipu.trace {
                    eprintln!("[ipu] unknown command {v:08X}");
                }
                Ok(true)
            }
        }
    }

    /// IDEC: decode an intra slice from macroblock_type onwards (the first
    /// macroblock_address_increment has been consumed by the caller), CSC
    /// each macroblock, and stop at the next start code with SCD set.
    fn run_idec(&mut self) -> R<bool> {
        let v = self.ipu.command;
        let dtd = v & (1 << 24) != 0;
        let sgn = v & (1 << 25) != 0;
        let dte = v & (1 << 26) != 0;
        let rgb16 = v & (1 << 27) != 0;
        loop {
            let ok = match self.ipu.phase {
                0 => {
                    let r = self.atomic(|c| c.skip(v & 0x3F))?;
                    if r.is_some() {
                        self.ipu.phase = 1;
                    }
                    r.is_some()
                }
                1 => {
                    let first = self.ipu.first;
                    let r = self.atomic(|c| {
                        let ty = c.decode(&luts().mb_i)? & 0xFFFF;
                        let field = dtd && c.get(1)? != 0;
                        let mut qsc = c.ipu.qsc;
                        if ty & 0x10 != 0 {
                            qsc = c.get(5)?;
                        }
                        let mut raw = [0i16; 384];
                        c.decode_macroblock(&mut raw, true, field, first, qsc)?;
                        let mut raw8 = [0u8; 384];
                        for (d, s) in raw8.iter_mut().zip(raw.iter()) {
                            *d = (*s).clamp(0, 255) as u8;
                        }
                        c.emit_csc(&raw8, rgb16, dte, sgn);
                        c.ipu.qsc = qsc;
                        Ok(())
                    })?;
                    if r.is_some() {
                        self.ipu.first = false;
                        self.ipu.phase = 2;
                    }
                    r.is_some()
                }
                2 => {
                    // Start code next (end of slice), or the next macroblock's
                    // address increment (skipping stuffing / escapes).
                    let r = self.atomic(|c| {
                        let (_, avail) = c.peek_soft(23);
                        if avail < 23 {
                            return Err(Stop::NeedData);
                        }
                        if c.start_code_follows() {
                            c.ipu.bit = (c.ipu.bit + 7) & !7;
                            return Ok(3);
                        }
                        loop {
                            let inc = c.decode(&luts().mbai)? & 0xFFFF;
                            if inc != 0x22 && inc != 0x23 {
                                break;
                            }
                        }
                        Ok(1)
                    })?;
                    if let Some(p) = r {
                        self.ipu.phase = p;
                    }
                    r.is_some()
                }
                3 => {
                    // Consume zero bytes up to the 00 00 01 prefix.
                    let r = self.atomic(|c| loop {
                        let (code, avail) = c.peek_soft(24);
                        if avail < 24 {
                            return Err(Stop::NeedData);
                        }
                        if code == 1 {
                            return Ok(());
                        }
                        if code != 0 {
                            return Err(Stop::BadCode);
                        }
                        c.ipu.bit += 8;
                    })?;
                    if r.is_some() {
                        self.ipu.phase = 4;
                    }
                    r.is_some()
                }
                _ => {
                    self.ipu.ctrl |= ctrl::SCD;
                    return Ok(true);
                }
            };
            if !ok {
                return Ok(false);
            }
            self.commit();
        }
    }
}
