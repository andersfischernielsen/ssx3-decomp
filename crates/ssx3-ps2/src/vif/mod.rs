//! VIF0/VIF1: the command stream in front of each VU (UNPACK, MPG, MSCAL,
//! DIRECT, STROW/STCOL/STMASK, ...).
//!
//! INTERFACE CONTRACT:
//! - `Vif::new(index)`.
//! - `write(data, vu, gs)`: consume whole quadwords from DMA (or the FIFO);
//!   command state persists across calls. `vu` is the VU this VIF feeds. For
//!   VIF1, DIRECT/DIRECTHL go to `gs.gif_write(2, ..)` and the VU's XGKICK goes
//!   to `gs.gif_write(1, ..)`.
//! - `read_reg(off)` / `write_reg(off, val)`: VIFn registers (`off` relative to
//!   `0x1000_3800` for VIF0, `0x1000_3C00` for VIF1).
//! - `stalled()`: an interrupt bit (i) or MARK stalled the stream.
//! - `take_irq()`: true once per VIF interrupt (INTC VIF0/VIF1 bit).
//!
//! Notes:
//! - The stream is buffered as 32-bit words; a command whose data has not
//!   fully arrived waits for the next `write`.
//! - MSCAL/MSCALF/MSCNT run the VU program to completion synchronously
//!   (translated microcode via `Vu::run`, budget `vu_budget` pairs), so the
//!   FLUSH* commands are no-ops and STAT.VEW is never observed set.
//! - A VIFcode with the i bit (and ERR.MII clear) sets STAT.INT/VIS after it
//!   executes and stalls the stream; `take_irq()` reports it once. Writing
//!   FBRST.STC clears the stall; buffered data resumes on the next `write`
//!   (or call `resume`). MARK does not stall (it only sets STAT.MRK).
//!
//! UNPACK ported in spirit from ps2-web-recomp `runtime/src/vif.cpp` (MIT,
//! https://github.com/snuri00/ps2-web-recomp) with PCSX2 semantics.

#[cfg(test)]
mod tests;

use crate::gs::Mtgs as Gs;
use crate::vu::Vu;
use crate::Qword;

/// STAT bits.
pub const STAT_MRK: u32 = 1 << 6;
pub const STAT_DBF: u32 = 1 << 7;
pub const STAT_VSS: u32 = 1 << 8;
pub const STAT_VFS: u32 = 1 << 9;
pub const STAT_VIS: u32 = 1 << 10;
pub const STAT_INT: u32 = 1 << 11;
pub const STAT_ER0: u32 = 1 << 12;
pub const STAT_ER1: u32 = 1 << 13;

pub struct Vif {
    pub index: usize,
    pub stat: u32,
    pub err: u32,
    pub mark: u32,
    pub cycle: u32,
    pub mode: u32,
    pub num: u32,
    pub mask: u32,
    pub code: u32,
    pub itops: u32,
    pub base: u32,
    pub ofst: u32,
    pub tops: u32,
    pub itop: u32,
    pub top: u32,
    pub row: [u32; 4],
    pub col: [u32; 4],
    pub dbf: bool,
    /// MSKPATH3 state (VIF1).
    pub path3_masked: bool,
    /// Pair budget for one MSCAL.
    pub vu_budget: u64,
    /// Per-command execution counts (diagnostics).
    pub commands: [u64; 128],
    pub errors: u64,
    fifo: Vec<u32>,
    stalled: bool,
    irq: bool,
}

/// UNPACK of format V(VN+1)-(32>>VL) (VL 3: V4-5) without mask, mode or
/// cycle skipping, to `num` consecutive qwords from `addr` (wrapping):
/// the same element decoding as the general loop in `Vif::unpack`.
fn unpack_plain<const VN: u32, const VL: u32>(data: &[u32], num: usize, addr: u32, qwords: u32, usn: bool, dst: &mut [u8]) {
    let n = VN as usize + 1;
    let elem = |e: usize| -> u32 {
        match VL {
            0 => data[e],
            1 => {
                let h = (data[e / 2] >> (16 * (e % 2))) as u16;
                if usn {
                    h as u32
                } else {
                    h as i16 as i32 as u32
                }
            }
            _ => {
                let x = (data[e / 4] >> (8 * (e % 4))) as u8;
                if usn {
                    x as u32
                } else {
                    x as i8 as i32 as u32
                }
            }
        }
    };
    for i in 0..num {
        let v: [u32; 4] = if VL == 3 {
            let c = (data[i / 2] >> (16 * (i % 2))) & 0xffff;
            [(c & 0x1f) << 3, ((c >> 5) & 0x1f) << 3, ((c >> 10) & 0x1f) << 3, ((c >> 15) & 1) << 7]
        } else {
            let b = i * n;
            match n {
                1 => [elem(b); 4],
                2 => {
                    let (x, y) = (elem(b), elem(b + 1));
                    [x, y, x, y]
                }
                3 => [elem(b), elem(b + 1), elem(b + 2), 0],
                _ => [elem(b), elem(b + 1), elem(b + 2), elem(b + 3)],
            }
        };
        let o = (((addr + i as u32) & (qwords - 1)) * 16) as usize;
        let q: &mut [u8; 16] = (&mut dst[o..o + 16]).try_into().unwrap();
        for j in 0..4 {
            q[j * 4..j * 4 + 4].copy_from_slice(&v[j].to_le_bytes());
        }
    }
}

impl Vif {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            stat: 0,
            err: 0,
            mark: 0,
            cycle: 0,
            mode: 0,
            num: 0,
            mask: 0,
            code: 0,
            itops: 0,
            base: 0,
            ofst: 0,
            tops: 0,
            itop: 0,
            top: 0,
            row: [0; 4],
            col: [0; 4],
            dbf: false,
            path3_masked: false,
            vu_budget: 2_000_000,
            commands: [0; 128],
            errors: 0,
            fifo: Vec::new(),
            stalled: false,
            irq: false,
        }
    }

    pub fn write(&mut self, data: &[Qword], vu: &mut Vu, gs: &mut Gs) {
        self.fifo.reserve(data.len() * 4);
        for q in data {
            for k in 0..4 {
                self.fifo.push((*q >> (32 * k)) as u32);
            }
        }
        self.process(vu, gs);
    }

    /// Continue processing buffered data (e.g. after FBRST.STC).
    pub fn resume(&mut self, vu: &mut Vu, gs: &mut Gs) {
        self.process(vu, gs);
    }

    /// Words buffered but not yet consumed.
    pub fn pending_words(&self) -> usize {
        self.fifo.len()
    }

    fn process(&mut self, vu: &mut Vu, gs: &mut Gs) {
        let fifo = std::mem::take(&mut self.fifo);
        let mut pos = 0;
        while pos < fifo.len() && !self.stalled {
            let cmd = fifo[pos];
            let need = self.data_words(cmd);
            if pos + 1 + need > fifo.len() {
                self.num = (cmd >> 16) & 0xff;
                break;
            }
            self.execute(cmd, &fifo[pos + 1..pos + 1 + need], vu, gs);
            pos += 1 + need;
            self.num = 0;
        }
        self.fifo = fifo;
        self.fifo.drain(..pos);
    }

    fn cl_wl(&self) -> (u32, u32) {
        (self.cycle & 0xff, (self.cycle >> 8) & 0xff)
    }

    fn unpack_words(&self, cmd: u32) -> usize {
        let vn = (cmd >> 26) & 3;
        let vl = (cmd >> 24) & 3;
        let num = match (cmd >> 16) & 0xff {
            0 => 256,
            n => n,
        };
        let (cl, wl) = self.cl_wl();
        let data = if wl > cl && wl != 0 {
            (num / wl) * cl + (num % wl).min(cl)
        } else {
            num
        };
        let bits = if vl == 3 { 16 } else { (vn + 1) * (32 >> vl) };
        ((data * bits) as usize).div_ceil(32)
    }

    fn data_words(&self, cmd: u32) -> usize {
        let op = (cmd >> 24) & 0x7f;
        let imm = cmd & 0xffff;
        match op {
            0x20 => 1,
            0x30 | 0x31 => 4,
            0x4a => {
                let n = (cmd >> 16) & 0xff;
                (if n == 0 { 256 } else { n }) as usize * 2
            }
            0x50 | 0x51 => (if imm == 0 { 0x10000 } else { imm }) as usize * 4,
            _ if op & 0x60 == 0x60 => self.unpack_words(cmd),
            _ => 0,
        }
    }

    fn execute(&mut self, cmd: u32, data: &[u32], vu: &mut Vu, gs: &mut Gs) {
        let op = (cmd >> 24) & 0x7f;
        let imm = cmd & 0xffff;
        self.code = cmd;
        self.commands[op as usize] += 1;
        if self.index == 1 && op >= 0x10 && !matches!(op, 0x20 | 0x30 | 0x31) {
            // PATH3 runs alongside the VIF: while it is unmasked, waiting
            // GIF DMA data goes before the VIF's next transfer, flush or
            // microprogram. NOPs and register settings take a cycle or two:
            // SSX 3 puts `MSKPATH3 0`, NOPs and `MSKPATH3 1` together to
            // release a single packet.
            gs.path3_flow();
        }
        if crate::trace_path3() && self.index == 1 && matches!(op, 0x06 | 0x10 | 0x11 | 0x13 | 0x14 | 0x15 | 0x17 | 0x50 | 0x51) {
            eprintln!("[p3] vif1 cmd {cmd:#010x}");
        }
        match op {
            0x00 => {}
            0x01 => self.cycle = imm,
            0x02 => {
                self.ofst = imm & 0x3ff;
                self.dbf = false;
                self.tops = self.base;
            }
            0x03 => self.base = imm & 0x3ff,
            0x04 => self.itops = imm & 0x3ff,
            0x05 => self.mode = imm & 3,
            0x06 => {
                self.path3_masked = (imm >> 15) & 1 != 0;
                if self.index == 1 {
                    gs.set_path3_mask(self.path3_masked);
                }
            }
            0x07 => {
                self.mark = imm;
                self.stat |= STAT_MRK;
            }
            0x10 | 0x11 | 0x13 => {}
            0x14 | 0x15 => self.run_vu(imm * 8, vu, gs),
            0x17 => {
                let pc = vu.tpc;
                self.run_vu(pc, vu, gs)
            }
            0x20 => self.mask = data[0],
            0x30 => self.row.copy_from_slice(&data[..4]),
            0x31 => self.col.copy_from_slice(&data[..4]),
            0x4a => {
                let bytes: Vec<u8> = data.iter().flat_map(|w| w.to_le_bytes()).collect();
                vu.write_code(imm * 8, &bytes);
            }
            0x50 | 0x51 => {
                if self.index == 1 {
                    let qws: Vec<Qword> = data
                        .chunks_exact(4)
                        .map(|c| {
                            c[0] as u128
                                | (c[1] as u128) << 32
                                | (c[2] as u128) << 64
                                | (c[3] as u128) << 96
                        })
                        .collect();
                    gs.gif_write(2, &qws);
                } else {
                    self.error(cmd);
                }
            }
            _ if op & 0x60 == 0x60 => self.unpack(cmd, data, vu),
            _ => self.error(cmd),
        }
        if cmd & 0x8000_0000 != 0 && self.err & 1 == 0 {
            self.stat |= STAT_INT | STAT_VIS;
            self.stalled = true;
            self.irq = true;
        }
    }

    fn error(&mut self, cmd: u32) {
        self.errors += 1;
        self.stat |= STAT_ER1;
        if self.errors <= 32 {
            eprintln!("[vif{}] unsupported VIFcode {cmd:#010x}", self.index);
        }
    }

    fn run_vu(&mut self, pc: u32, vu: &mut Vu, gs: &mut Gs) {
        if self.index == 1 {
            self.top = self.tops & 0x3ff;
            if self.dbf {
                self.tops = self.base;
                self.dbf = false;
            } else {
                self.tops = self.base + self.ofst;
                self.dbf = true;
            }
        }
        self.itop = self.itops;
        if self.index == 1 && crate::render_verify::enabled() {
            crate::render_verify::program_start(Some(pc), self.top, self.itop, &vu.data);
        }
        vu.vif_top = self.top;
        vu.vif_itop = self.itop;
        vu.start(pc);
        let mut cap = crate::vu::capture::begin(vu, pc);
        let vif1 = self.index == 1;
        // SSX3_VERIFY_MAT3: native type 3 program vs the translated one.
        let mut mat3 = if vif1 { crate::mat3_verify::begin(vu, pc, self.top, self.itop) } else { None };
        // SSX3_VERIFY_MAT6: native rider program vs the translated one.
        let mut mat6 = if vif1 { crate::mat6_verify::begin(vu, pc, self.top, self.itop) } else { None };
        let watch = crate::hw::watch_qword_pub();
        let mut hit: Option<(usize, u128)> = None;
        let done = vu.run(self.vu_budget, &mut |p: &[Qword]| {
            if let Some(c) = cap.as_mut() {
                c.kicks.push(p.to_vec());
            }
            if let Some(m) = mat3.as_mut() {
                m.kick(p);
            }
            if let Some(m) = mat6.as_mut() {
                m.kick(p);
            }
            if let Some(w) = watch {
                if let Some((k, q)) = p.iter().enumerate().find(|(_, q)| **q as u64 == w) {
                    if hit.is_none() {
                        static ONCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                        if !ONCE.swap(true, std::sync::atomic::Ordering::Relaxed) {
                            for (i, x) in p.iter().enumerate().take(48) {
                                eprintln!("[vu-pkt] {i:3}: {x:#034x}");
                            }
                        }
                    }
                    hit = Some((k, *q));
                }
            }
            if vif1 {
                if crate::gifstats::enabled() {
                    crate::gifstats::scan(p);
                }
                gs.gif_write(1, p)
            }
        });
        if let Some((k, q)) = hit {
            eprintln!("[vu-watch] vu{} program entry {pc:#06x} hash {:#018x} kicked {q:#034x} at packet qword {k}", self.index, vu.segment_hash_pub(pc));
        }
        if !done {
            eprintln!(
                "[vif{}] VU program at {pc:#06x} exceeded {} pairs; stopped",
                self.index, self.vu_budget
            );
            vu.flush();
            vu.running = false;
            self.errors += 1;
        }
        if let Some(c) = cap {
            crate::vu::capture::end(c, vu);
        }
        if let (Some(m), true) = (mat3, done) {
            crate::mat3_verify::end(m, vu);
        }
        if let (Some(m), true) = (mat6, done) {
            crate::mat6_verify::end(m, vu);
        }
    }

    fn unpack(&mut self, cmd: u32, data: &[u32], vu: &mut Vu) {
        let vn = (cmd >> 26) & 3;
        let vl = (cmd >> 24) & 3;
        let masked = (cmd >> 28) & 1 != 0;
        let usn = (cmd >> 14) & 1 != 0;
        let num = match (cmd >> 16) & 0xff {
            0 => 256,
            n => n,
        };
        let mut addr = cmd & 0x3ff;
        if self.index == 1 && (cmd >> 15) & 1 != 0 {
            addr += self.tops;
        }
        let (cl, wl) = self.cl_wl();
        let fill_mode = wl > cl;
        let qwords = (vu.data.len() / 16) as u32;
        // Elements are read straight from the little-endian words (they are
        // naturally aligned: 32-bit at word, 16-bit at half-word offsets).
        let mut bitpos = 0usize;
        let elem_bits = 32usize >> vl;
        let mut read = |bits: usize| -> u32 {
            let b = bitpos / 8;
            bitpos += bits;
            let w = data[b / 4] >> (8 * (b % 4));
            match bits {
                32 => w,
                16 => {
                    let h = w as u16;
                    if usn || vl == 3 {
                        h as u32
                    } else {
                        h as i16 as i32 as u32
                    }
                }
                _ => {
                    let x = w as u8;
                    if usn {
                        x as u32
                    } else {
                        x as i8 as i32 as u32
                    }
                }
            }
        };
        if !masked && self.mode == 0 && cl == wl {
            // Unmasked, plain mode, consecutive qwords (the common case):
            // decoding monomorphised per format.
            let f = match (vn, vl) {
                (0, 0) => unpack_plain::<0, 0>,
                (0, 1) => unpack_plain::<0, 1>,
                (0, 2) => unpack_plain::<0, 2>,
                (1, 0) => unpack_plain::<1, 0>,
                (1, 1) => unpack_plain::<1, 1>,
                (1, 2) => unpack_plain::<1, 2>,
                (2, 0) => unpack_plain::<2, 0>,
                (2, 1) => unpack_plain::<2, 1>,
                (2, 2) => unpack_plain::<2, 2>,
                (3, 0) => unpack_plain::<3, 0>,
                (3, 1) => unpack_plain::<3, 1>,
                (3, 2) => unpack_plain::<3, 2>,
                (3, 3) => unpack_plain::<3, 3>,
                _ => unpack_plain::<0, 3>,
            };
            if vl != 3 || vn == 3 {
                f(data, num as usize, addr, qwords, usn, &mut vu.data);
                return;
            }
        }
        let mut cyc = 0u32;
        for _ in 0..num {
            let fill = fill_mode && cyc >= cl;
            let mut v = [0u32; 4];
            if !fill {
                if vl == 3 {
                    // V4-5: RGBA 5:5:5:1.
                    let c = read(16);
                    v = [
                        (c & 0x1f) << 3,
                        ((c >> 5) & 0x1f) << 3,
                        ((c >> 10) & 0x1f) << 3,
                        ((c >> 15) & 1) << 7,
                    ];
                } else {
                    let n = vn as usize + 1;
                    for x in v.iter_mut().take(n) {
                        *x = read(elem_bits);
                    }
                    match n {
                        1 => v = [v[0]; 4],
                        2 => v = [v[0], v[1], v[0], v[1]],
                        _ => {}
                    }
                }
            }
            let mrow = cyc.min(3);
            let o = ((addr & (qwords - 1)) * 16) as usize;
            for (j, val) in v.iter().enumerate() {
                let mut m = if masked {
                    (self.mask >> ((mrow * 4 + j as u32) * 2)) & 3
                } else {
                    0
                };
                if fill && m == 0 {
                    m = 1;
                }
                let out = match m {
                    3 => continue,
                    1 => self.row[j],
                    2 => self.col[mrow as usize],
                    _ => match self.mode {
                        1 => val.wrapping_add(self.row[j]),
                        2 => {
                            self.row[j] = val.wrapping_add(self.row[j]);
                            self.row[j]
                        }
                        _ => *val,
                    },
                };
                vu.data[o + j * 4..o + j * 4 + 4].copy_from_slice(&out.to_le_bytes());
            }
            addr += 1;
            cyc += 1;
            if !fill_mode {
                if wl != 0 && cyc >= wl {
                    addr += cl - wl;
                    cyc = 0;
                }
            } else if cyc >= wl {
                cyc = 0;
            }
        }
    }

    pub fn read_reg(&self, off: u32) -> u32 {
        match off {
            0x00 => {
                let fqc = (self.fifo.len() / 4).min(16) as u32;
                let vps = if self.fifo.is_empty() { 0 } else { 1 };
                (self.stat & !(STAT_DBF | 0x1f00_0003))
                    | vps
                    | ((self.dbf as u32) << 7)
                    | (fqc << 24)
            }
            0x20 => self.err,
            0x30 => self.mark,
            0x40 => self.cycle,
            0x50 => self.mode,
            0x60 => self.num,
            0x70 => self.mask,
            0x80 => self.code,
            0x90 => self.itops,
            0xa0 => self.base,
            0xb0 => self.ofst,
            0xc0 => self.tops,
            0xd0 => self.itop,
            0xe0 => self.top,
            0x100..=0x13f => self.row[((off - 0x100) >> 4) as usize],
            0x140..=0x17f => self.col[((off - 0x140) >> 4) as usize],
            _ => 0,
        }
    }

    pub fn write_reg(&mut self, off: u32, val: u32) {
        match off {
            0x10 => {
                if val & 1 != 0 {
                    let (index, budget) = (self.index, self.vu_budget);
                    *self = Vif::new(index);
                    self.vu_budget = budget;
                }
                if val & 4 != 0 {
                    self.stat |= STAT_VSS;
                    self.stalled = true;
                }
                if val & 8 != 0 {
                    self.stat &= !(STAT_VSS | STAT_VFS | STAT_VIS | STAT_INT | STAT_ER0 | STAT_ER1);
                    self.stalled = false;
                }
            }
            0x20 => self.err = val & 7,
            0x30 => {
                self.mark = val & 0xffff;
                self.stat &= !STAT_MRK;
            }
            0x100..=0x13f => self.row[((off - 0x100) >> 4) as usize] = val,
            0x140..=0x17f => self.col[((off - 0x140) >> 4) as usize] = val,
            _ => {}
        }
    }

    pub fn stalled(&self) -> bool {
        self.stalled
    }

    pub fn take_irq(&mut self) -> bool {
        std::mem::take(&mut self.irq)
    }
}
