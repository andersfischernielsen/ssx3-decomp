//! SPU2: two cores of 24 voices, 2 MiB of sound RAM, AutoDMA input, IRQ
//! address, output at 48 kHz.
//!
//! Voices decode PS-ADPCM (16-byte blocks of 28 samples), run the PSX/PS2
//! ADSR envelope, resample by `pitch` (0x1000 = 48 kHz) with linear
//! interpolation, and are mixed with their L/R volumes. AutoDMA (`ADMAS`)
//! streams interleaved PCM from IOP RAM into the core input (512 bytes left,
//! 512 bytes right per 256 frames), mixed at `BVOL`. Reverb is not modelled.
//!
//! Each core also writes its dry output and the voice 1/3 outputs into the
//! fixed "ring" areas of sound RAM like the hardware does (games can clock
//! themselves off IRQs on those addresses).
//!
//! Ported from ps2-web-recomp `runtime/src/iop/spu2.cpp` (MIT, Copyright (c)
//! 2026 Samir Nuri), restructured for this crate.

use std::collections::VecDeque;

use crate::hw::LINE_SPU2;
use crate::kernel::Timed;
use crate::Iop;

const RAM_WORDS: usize = 1 << 20;
const RAM_MASK: u32 = (RAM_WORDS - 1) as u32;
/// IOP cycles per output sample (36.864 MHz / 48 kHz).
pub const CYCLES_PER_SAMPLE: u64 = 768;
pub const BATCH: usize = 32;
const INPUT_FRAMES: usize = 512;
const INPUT_BASE: [u32; 2] = [0x2000, 0x2400];
const RING_WORDS: u32 = 0x200;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Phase {
    #[default]
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Envelope {
    counter: i32,
    pub level: i32,
}

impl Envelope {
    fn step(&mut self, rate: i32, decrease: bool, exponential: bool) {
        let shift = rate >> 2;
        let mut cycles = 1 << (shift - 11).max(0);
        if exponential && !decrease && self.level > 0x6000 {
            cycles *= 4;
        }
        self.counter += 1;
        if self.counter < cycles {
            return;
        }
        self.counter = 0;
        let mut delta = (if decrease { -8 + (rate & 3) } else { 7 - (rate & 3) }) << (11 - shift).max(0);
        if exponential && decrease {
            delta = delta * self.level / 0x8000;
        }
        self.level = (self.level + delta).clamp(0, 0x7FFF);
    }
}

#[derive(Clone, Default, Debug)]
pub struct Voice {
    pub voll: u16,
    pub volr: u16,
    pub pitch: u16,
    pub adsr1: u16,
    pub adsr2: u16,
    vol: [Envelope; 2],
    pub ssa: u32,
    pub lsax: u32,
    next: u32,
    block: [u32; 2],
    block_end: [bool; 2],
    lsax_written: bool,
    key_on: bool,
    pub end: bool,
    pub phase: Phase,
    pub env: Envelope,
    buffer: Buf,
    s1: i32,
    s2: i32,
    position: u32,
}

#[derive(Clone, Copy, Debug)]
struct Buf([i16; 56]);

impl Default for Buf {
    fn default() -> Self {
        Buf([0; 56])
    }
}

#[derive(Clone, Copy, Debug)]
struct Transfer {
    madr: u32,
    bytes: u32,
    channel: u32,
}

pub struct Core {
    pub voices: Vec<Voice>,
    pub attr: u16,
    pub admas: u16,
    pub irqa: u32,
    pub tsa: u32,
    pub mvol: [u16; 2],
    pub bvol: [u16; 2],
    pub avol: [u16; 2],
    pub evol: [u16; 2],
    pub irq: bool,
    input: Box<[[i16; 2]; INPUT_FRAMES]>,
    input_read: usize,
    input_count: usize,
    transfers: VecDeque<Transfer>,
    regs: Vec<u16>,
}

impl Core {
    fn new() -> Self {
        Self {
            voices: vec![Voice::default(); 24],
            attr: 0,
            admas: 0,
            irqa: 0x800,
            tsa: 0,
            mvol: [0x3FFF; 2],
            bvol: [0x7FFF; 2],
            avol: [0x7FFF; 2],
            evol: [0; 2],
            irq: false,
            input: Box::new([[0; 2]; INPUT_FRAMES]),
            input_read: 0,
            input_count: 0,
            transfers: VecDeque::new(),
            regs: vec![0; 0x200],
        }
    }
}

pub struct Spu2 {
    pub ram: Vec<u16>,
    pub cores: [Core; 2],
    spdif_out: u16,
    spdif_mode: u16,
    spdif_media: u16,
    /// IOP cycle of the next output batch.
    pub next_tick: u64,
    ticks: u64,
    pub sink: Option<crate::audio::Sink>,
    pub samples_out: u64,
    pub trace: bool,
    /// Pending interrupt raise (IRQ address hit).
    raise_irq: bool,
    pub stats: [u64; 4],
}

impl Spu2 {
    pub fn new() -> Self {
        Self {
            ram: vec![0; RAM_WORDS],
            cores: [Core::new(), Core::new()],
            spdif_out: 0,
            spdif_mode: 0,
            spdif_media: 0,
            next_tick: BATCH as u64 * CYCLES_PER_SAMPLE,
            ticks: 0,
            sink: None,
            samples_out: 0,
            trace: std::env::var("SSX3_TRACE_SPU2").is_ok(),
            raise_irq: false,
            stats: [0; 4],
        }
    }

    fn check_irq(&mut self, address: u32, words: u32) {
        for k in self.cores.iter_mut() {
            if k.attr & 0x40 == 0 || k.irq {
                continue;
            }
            if (k.irqa.wrapping_sub(address) & RAM_MASK) >= words {
                continue;
            }
            k.irq = true;
            self.raise_irq = true;
        }
    }

    fn decode_block(&mut self, c: usize, v: usize, half: usize) {
        const FILTER: [[i32; 2]; 5] = [[0, 0], [60, 0], [115, -52], [98, -55], [122, -60]];
        let at = self.cores[c].voices[v].next & RAM_MASK;
        let header = self.ram[at as usize];
        let shift = (header & 0xF) as i32;
        let filter = ((header >> 4) & 0xF).min(4) as usize;
        let flags = header >> 8;
        let mut words = [0u16; 7];
        for (i, w) in words.iter_mut().enumerate() {
            *w = self.ram[((at + 1 + i as u32) & RAM_MASK) as usize];
        }
        let voice = &mut self.cores[c].voices[v];
        for i in 0..28 {
            let nib = ((words[i / 4] >> ((i & 3) * 4)) & 0xF) as i32;
            let mut sample = (((nib << 12) as i16) as i32) >> shift;
            sample += (voice.s1 * FILTER[filter][0] + voice.s2 * FILTER[filter][1] + 32) >> 6;
            let sample = sample.clamp(-32768, 32767);
            voice.s2 = voice.s1;
            voice.s1 = sample;
            voice.buffer.0[half * 28 + i] = sample as i16;
        }
        voice.block[half] = at;
        voice.block_end[half] = flags & 1 != 0;
        if flags & 4 != 0 && !voice.lsax_written {
            voice.lsax = at;
        }
        voice.next = (at + 8) & RAM_MASK;
        if flags & 1 != 0 {
            voice.next = voice.lsax;
            if flags & 2 == 0 {
                voice.phase = Phase::Release;
                voice.env.level = 0;
            }
        }
    }

    fn key_on(&mut self, c: usize, v: usize) {
        {
            let voice = &mut self.cores[c].voices[v];
            voice.key_on = false;
            voice.lsax_written = false;
            voice.end = false;
            voice.next = voice.ssa;
            voice.s1 = 0;
            voice.s2 = 0;
            voice.position = 0;
            voice.phase = Phase::Attack;
            voice.env = Envelope::default();
        }
        self.decode_block(c, v, 0);
        self.decode_block(c, v, 1);
        let b = self.cores[c].voices[v].block[0];
        self.check_irq(b, 8);
    }

    fn render_voice(&mut self, c: usize, v: usize, sum: &mut [[i32; 2]; BATCH], raw: Option<&mut [i16; BATCH]>) {
        if self.cores[c].voices[v].key_on {
            self.key_on(c, v);
        }
        let mut raw = raw;
        for n in 0..BATCH {
            if self.cores[c].voices[v].phase == Phase::Off {
                break;
            }
            let (sample, wrapped) = {
                let voice = &mut self.cores[c].voices[v];
                let step = (voice.pitch as u32).min(0x3FFF);
                let index = (voice.position >> 12) as usize;
                let frac = (voice.position & 0xFFF) as i32;
                let s = (voice.buffer.0[index] as i32 * (0x1000 - frac) + voice.buffer.0[index + 1] as i32 * frac) >> 12;
                voice.position += step;
                (s, voice.position >> 12 >= 28)
            };
            if wrapped {
                let mut guard = 0;
                while self.cores[c].voices[v].position >> 12 >= 28 && guard < 4 {
                    guard += 1;
                    let voice = &mut self.cores[c].voices[v];
                    voice.position -= 28 << 12;
                    voice.buffer.0.copy_within(28..56, 0);
                    voice.end |= voice.block_end[0];
                    voice.block[0] = voice.block[1];
                    voice.block_end[0] = voice.block_end[1];
                    let b = voice.block[0];
                    self.check_irq(b, 8);
                    self.decode_block(c, v, 1);
                }
            }
            let voice = &mut self.cores[c].voices[v];
            update_envelope(voice);
            let out = sample * voice.env.level >> 15;
            if let Some(r) = raw.as_deref_mut() {
                r[n] = clamp16(out);
            }
            let l = volume(voice.voll, &mut voice.vol[0]);
            let r = volume(voice.volr, &mut voice.vol[1]);
            sum[n][0] += out * l >> 15;
            sum[n][1] += out * r >> 15;
        }
    }

    /// Move queued AutoDMA data into the core's input ring. Returns channels
    /// whose transfer was fully consumed.
    fn feed_input(&mut self, core: usize, ram: &[u8], done: &mut Vec<u32>) {
        loop {
            let k = &mut self.cores[core];
            if k.input_count + 256 > INPUT_FRAMES {
                return;
            }
            let Some(t) = k.transfers.front_mut() else { return };
            let chunk = t.bytes.min(1024);
            let frames = (chunk / 4).max(1) as usize;
            let src = (t.madr & 0x1F_FFFF) as usize;
            for i in 0..frames {
                let rd = |o: usize| -> i16 {
                    let o = (src + o) & 0x1F_FFFF;
                    i16::from_le_bytes([ram[o], ram[(o + 1) & 0x1F_FFFF]])
                };
                let l = rd(i * 2);
                let r = rd(chunk as usize / 2 + i * 2);
                let slot = (k.input_read + k.input_count + i) % INPUT_FRAMES;
                k.input[slot] = [l, r];
                self.ram[(INPUT_BASE[core] as usize + slot) & RAM_MASK as usize] = l as u16;
                self.ram[(INPUT_BASE[core] as usize + 0x200 + slot) & RAM_MASK as usize] = r as u16;
            }
            let k = &mut self.cores[core];
            k.input_count += frames;
            let t = k.transfers.front_mut().unwrap();
            t.madr += chunk;
            t.bytes -= chunk;
            if t.bytes == 0 {
                let ch = t.channel;
                k.transfers.pop_front();
                done.push(ch);
            }
        }
    }

    fn render_input(&mut self, core: usize, left: &mut i32, right: &mut i32) -> (i16, i16) {
        let k = &mut self.cores[core];
        if k.input_count == 0 {
            return (0, 0);
        }
        let slot = k.input_read;
        let [l, r] = k.input[slot];
        *left += l as i32 * (k.bvol[0] as i16 as i32) >> 15;
        *right += r as i32 * (k.bvol[1] as i16 as i32) >> 15;
        k.input_read = (k.input_read + 1) % INPUT_FRAMES;
        k.input_count -= 1;
        self.check_irq(INPUT_BASE[core] + slot as u32, 1);
        (l, r)
    }

    fn write_rings(&mut self, core: usize, rings: &[[i16; BATCH]; 8]) {
        let pos = ((self.ticks % (RING_WORDS as u64 / BATCH as u64)) * BATCH as u64) as u32;
        for (r, ring) in rings.iter().enumerate() {
            let r = r as u32;
            let at = (if r < 4 { 0 } else { 0x1000 }) + core as u32 * 0x800 + (r & 3) * RING_WORDS + pos;
            for (n, s) in ring.iter().enumerate() {
                self.ram[(at as usize + n) & RAM_MASK as usize] = *s as u16;
            }
            self.check_irq(at, BATCH as u32);
        }
    }

    pub fn read(&mut self, p: u32) -> u32 {
        let off = p & 0x7FE;
        if off < 0x760 {
            return self.read_core(((off & 0x400) != 0) as usize, off & 0x3FE);
        }
        if off < 0x7B0 {
            return self.master_reg((off - 0x760) / 0x28, (off - 0x760) % 0x28).map(|r| *r as u32).unwrap_or(0);
        }
        match off {
            0x7C0 => self.spdif_out as u32,
            0x7C2 => (if self.cores[0].irq { 4 } else { 0 }) | (if self.cores[1].irq { 8 } else { 0 }),
            0x7C6 => self.spdif_mode as u32,
            0x7C8 => self.spdif_media as u32,
            _ => 0,
        }
    }

    fn master_reg(&mut self, core: u32, off: u32) -> Option<&mut u16> {
        let k = &mut self.cores[core as usize & 1];
        Some(match off {
            0x0 => &mut k.mvol[0],
            0x2 => &mut k.mvol[1],
            0x4 => &mut k.evol[0],
            0x6 => &mut k.evol[1],
            0x8 => &mut k.avol[0],
            0xA => &mut k.avol[1],
            0xC => &mut k.bvol[0],
            0xE => &mut k.bvol[1],
            _ => return None,
        })
    }

    fn read_core(&mut self, core: usize, off: u32) -> u32 {
        let k = &self.cores[core];
        if off < 0x180 {
            let v = &k.voices[(off >> 4) as usize];
            return match off & 0xF {
                0x0 => v.voll as u32,
                0x2 => v.volr as u32,
                0x4 => v.pitch as u32,
                0x6 => v.adsr1 as u32,
                0x8 => v.adsr2 as u32,
                0xA => v.env.level as u32,
                0xC => v.vol[0].level as u32,
                _ => v.vol[1].level as u32,
            };
        }
        if (0x1C0..0x2E0).contains(&off) {
            let v = &k.voices[((off - 0x1C0) / 12) as usize];
            let nax = (v.block[0] + 1 + (v.position >> 14)) & RAM_MASK;
            return match (off - 0x1C0) % 12 {
                0 => v.ssa >> 16,
                2 => v.ssa & 0xFFFF,
                4 => v.lsax >> 16,
                6 => v.lsax & 0xFFFF,
                8 => nax >> 16,
                _ => nax & 0xFFFF,
            };
        }
        match off {
            0x19A => k.attr as u32,
            0x19C => k.irqa >> 16,
            0x19E => k.irqa & 0xFFFF,
            0x1A8 => k.tsa >> 16,
            0x1AA => k.tsa & 0xFFFF,
            0x1B0 => k.admas as u32,
            0x340 | 0x342 => {
                let mut endx = 0u32;
                for (i, v) in k.voices.iter().enumerate() {
                    if v.end {
                        endx |= 1 << i;
                    }
                }
                if off == 0x340 {
                    endx & 0xFFFF
                } else {
                    endx >> 16
                }
            }
            // STATX: bit 7 = DMA busy (while the ATTR DMA mode bits are set),
            // bit 10 = DMA request ready.
            0x344 => {
                if k.attr & 0x30 != 0 {
                    0x80
                } else {
                    0
                }
            }
            _ => k.regs[(off >> 1) as usize] as u32,
        }
    }
}

impl Default for Spu2 {
    fn default() -> Self {
        Self::new()
    }
}

fn update_envelope(v: &mut Voice) {
    let e = &mut v.env;
    match v.phase {
        Phase::Attack => {
            e.step(((v.adsr1 >> 8) & 0x7F) as i32, false, v.adsr1 & 0x8000 != 0);
            if e.level >= 0x7FFF {
                v.phase = Phase::Decay;
            }
        }
        Phase::Decay => {
            e.step((((v.adsr1 >> 4) & 0xF) << 2) as i32, true, true);
            if e.level <= ((v.adsr1 & 0xF) as i32 + 1) * 0x800 {
                v.phase = Phase::Sustain;
            }
        }
        Phase::Sustain => e.step(((v.adsr2 >> 6) & 0x7F) as i32, v.adsr2 & 0x4000 != 0, v.adsr2 & 0x8000 != 0),
        Phase::Release => {
            e.step(((v.adsr2 & 0x1F) << 2) as i32, true, v.adsr2 & 0x20 != 0);
            if e.level <= 0 {
                v.phase = Phase::Off;
            }
        }
        Phase::Off => {}
    }
}

fn volume(reg: u16, e: &mut Envelope) -> i32 {
    if reg & 0x8000 == 0 {
        return (reg << 1) as i16 as i32;
    }
    e.step((reg & 0x7F) as i32, reg & 0x2000 != 0, reg & 0x4000 != 0);
    e.level
}

fn clamp16(v: i32) -> i16 {
    v.clamp(-32768, 32767) as i16
}

fn master(reg: u16) -> i32 {
    if reg & 0x8000 != 0 {
        0x7FFF
    } else {
        (reg << 1) as i16 as i32
    }
}

fn set_hi(a: u32, v: u32) -> u32 {
    ((v & 0xF) << 16) | (a & 0xFFFF)
}
fn set_lo(a: u32, v: u32) -> u32 {
    (a & 0xF_0000) | (v & 0xFFFF)
}

fn flush_irq(c: &mut Iop) {
    if c.spu.raise_irq {
        c.spu.raise_irq = false;
        if c.spu.trace {
            eprintln!("[spu2] irq at {}", c.cycles);
        }
        c.hw.raise(LINE_SPU2);
    }
}

/// Register write (`p` is the physical address, 16-bit value).
pub fn write_reg(c: &mut Iop, p: u32, value: u32) {
    let off = p & 0x7FE;
    let value = value & 0xFFFF;
    let s = &mut c.spu;
    if off >= 0x760 {
        if off < 0x7B0 {
            if let Some(r) = s.master_reg((off - 0x760) / 0x28, (off - 0x760) % 0x28) {
                *r = value as u16;
            }
        } else {
            match off {
                0x7C0 => s.spdif_out = value as u16,
                0x7C6 => s.spdif_mode = value as u16,
                0x7C8 => s.spdif_media = value as u16,
                _ => {}
            }
        }
        return;
    }
    let core = ((off & 0x400) != 0) as usize;
    let off = off & 0x3FE;
    if s.trace && (0x180..0x1C0).contains(&off) || s.trace && off >= 0x2E0 {
        eprintln!("[spu2] core {core} reg {off:#05x} = {value:#06x}");
    }
    let k = &mut s.cores[core];
    if off < 0x180 {
        let v = &mut k.voices[(off >> 4) as usize];
        match off & 0xF {
            0x0 => {
                v.voll = value as u16;
                if value & 0x8000 == 0 {
                    v.vol[0].level = (value << 1) as i16 as i32;
                }
            }
            0x2 => {
                v.volr = value as u16;
                if value & 0x8000 == 0 {
                    v.vol[1].level = (value << 1) as i16 as i32;
                }
            }
            0x4 => v.pitch = value as u16,
            0x6 => v.adsr1 = value as u16,
            0x8 => v.adsr2 = value as u16,
            0xA => v.env.level = value as i16 as i32,
            _ => {}
        }
        return;
    }
    if (0x1C0..0x2E0).contains(&off) {
        let v = &mut k.voices[((off - 0x1C0) / 12) as usize];
        match (off - 0x1C0) % 12 {
            0 => v.ssa = set_hi(v.ssa, value),
            2 => v.ssa = set_lo(v.ssa, value),
            4 => {
                v.lsax = set_hi(v.lsax, value);
                v.lsax_written = true;
            }
            6 => {
                v.lsax = set_lo(v.lsax, value);
                v.lsax_written = true;
            }
            8 => v.next = set_hi(v.next, value),
            _ => v.next = set_lo(v.next, value),
        }
        return;
    }
    k.regs[(off >> 1) as usize] = value as u16;
    let key = |k: &mut Core, mask: u32, on: bool| {
        for i in 0..24 {
            if mask & (1 << i) == 0 {
                continue;
            }
            let v = &mut k.voices[i];
            if on {
                v.key_on = true;
            } else if v.key_on {
                v.key_on = false;
            } else if v.phase != Phase::Off {
                v.phase = Phase::Release;
            }
        }
    };
    match off {
        0x19A => {
            k.attr = value as u16;
            if value & 0x40 == 0 {
                k.irq = false;
            }
            if value & 0x8000 != 0 {
                // Core reset bit.
                k.irq = false;
            }
        }
        0x19C => k.irqa = set_hi(k.irqa, value),
        0x19E => k.irqa = set_lo(k.irqa, value),
        0x1A0 => key(k, value, true),
        0x1A2 => key(k, value << 16, true),
        0x1A4 => key(k, value, false),
        0x1A6 => key(k, value << 16, false),
        0x1A8 => k.tsa = set_hi(k.tsa, value),
        0x1AA => k.tsa = set_lo(k.tsa, value),
        0x1AC => {
            let tsa = k.tsa;
            k.tsa = (k.tsa + 1) & RAM_MASK;
            s.ram[tsa as usize] = value as u16;
            s.check_irq(tsa, 1);
        }
        0x1B0 => {
            k.admas = value as u16;
            if value == 0 {
                k.input_count = 0;
                k.input_read = 0;
            }
        }
        0x340 => (0..16).for_each(|i| k.voices[i].end = false),
        0x342 => (16..24).for_each(|i| k.voices[i].end = false),
        _ => {}
    }
    flush_irq(c);
}

/// IOP DMA channel 4 (core 0) / 7 (core 1).
pub fn dma(c: &mut Iop, core: usize, madr: u32, words: u32, to_spu: bool, channel: u32) {
    let s = &mut c.spu;
    if s.trace {
        eprintln!(
            "[spu2] dma core {core} {} madr {madr:#x} words {words:#x} tsa {:#x} admas {:#x}",
            if to_spu { "write" } else { "read" },
            s.cores[core].tsa,
            s.cores[core].admas
        );
    }
    if to_spu && s.cores[core].admas & (1 << core) != 0 {
        s.cores[core].transfers.push_back(Transfer { madr, bytes: words * 4, channel });
        let mut done = Vec::new();
        s.feed_input(core, &c.ram, &mut done);
        for ch in done {
            let at = c.cycles + 32;
            c.k.schedule(at, Timed::DmaDone(ch));
        }
        return;
    }
    let halfwords = words * 2;
    let tsa = s.cores[core].tsa;
    if to_spu {
        s.check_irq(tsa, halfwords);
    }
    for i in 0..halfwords {
        let sa = ((tsa + i) & RAM_MASK) as usize;
        let ia = ((madr + i * 2) & 0x1F_FFFF) as usize;
        if to_spu {
            s.ram[sa] = u16::from_le_bytes([c.ram[ia], c.ram[ia + 1]]);
        } else {
            c.ram[ia..ia + 2].copy_from_slice(&s.ram[sa].to_le_bytes());
        }
    }
    s.cores[core].tsa = (tsa + halfwords) & RAM_MASK;
    let at = c.cycles + 64 + words as u64 / 4;
    c.k.schedule(at, Timed::DmaDone(channel));
    flush_irq(c);
}

/// Produce one batch of output samples.
pub fn tick(c: &mut Iop) {
    let s = &mut c.spu;
    let mut sum = [[[0i32; 2]; BATCH]; 2];
    let mut rings = [[[0i16; BATCH]; 8]; 2];
    for core in 0..2 {
        let mut sc = sum[core];
        for v in 0..24 {
            let voice = &s.cores[core].voices[v];
            if !voice.key_on && voice.phase == Phase::Off {
                continue;
            }
            s.stats[0] += 1;
            match v {
                1 => {
                    let mut r = rings[core][2];
                    s.render_voice(core, v, &mut sc, Some(&mut r));
                    rings[core][2] = r;
                }
                3 => {
                    let mut r = rings[core][3];
                    s.render_voice(core, v, &mut sc, Some(&mut r));
                    rings[core][3] = r;
                }
                _ => s.render_voice(core, v, &mut sc, None),
            }
        }
        for n in 0..BATCH {
            rings[core][4][n] = clamp16(sc[n][0]);
            rings[core][5][n] = clamp16(sc[n][1]);
        }
        for n in 0..BATCH {
            let (mut l, mut r) = (sc[n][0], sc[n][1]);
            let (il, ir) = s.render_input(core, &mut l, &mut r);
            sc[n] = [l, r];
            if core == 0 {
                rings[0][0][n] = il;
                rings[0][1][n] = ir;
            }
        }
        sum[core] = sc;
    }
    let mut out = [0i16; BATCH * 2];
    for n in 0..BATCH {
        let mut mix = [0i32; 2];
        for core in 0..2 {
            let l = sum[core][n][0] * master(s.cores[core].mvol[0]) >> 15;
            let r = sum[core][n][1] * master(s.cores[core].mvol[1]) >> 15;
            if core == 0 {
                rings[1][0][n] = clamp16(l);
                rings[1][1][n] = clamp16(r);
            }
            mix[0] += l;
            mix[1] += r;
        }
        out[n * 2] = clamp16(mix[0]);
        out[n * 2 + 1] = clamp16(mix[1]);
    }
    s.write_rings(0, &rings[0]);
    s.write_rings(1, &rings[1]);
    if let Some(sink) = s.sink.as_mut() {
        sink.push(&out);
    }
    s.samples_out += BATCH as u64;
    let mut done = Vec::new();
    s.feed_input(0, &c.ram, &mut done);
    s.feed_input(1, &c.ram, &mut done);
    s.ticks += 1;
    s.next_tick += BATCH as u64 * CYCLES_PER_SAMPLE;
    for ch in done {
        let at = c.cycles + 32;
        c.k.schedule(at, Timed::DmaDone(ch));
    }
    flush_irq(c);
}

/// One-line voice summary for diagnostics.
pub fn describe(s: &Spu2) -> String {
    let mut out = String::new();
    for (ci, k) in s.cores.iter().enumerate() {
        let active = k.voices.iter().filter(|v| v.phase != Phase::Off).count();
        out += &format!(
            "core{ci}: attr {:#x} admas {:#x} mvol {:04x}/{:04x} bvol {:04x}/{:04x} voices {active} input {} queued {} | ",
            k.attr,
            k.admas,
            k.mvol[0],
            k.mvol[1],
            k.bvol[0],
            k.bvol[1],
            k.input_count,
            k.transfers.len()
        );
    }
    out
}
