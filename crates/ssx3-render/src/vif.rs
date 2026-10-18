//! VIF1 command interpretation into typed calls (stage 0 of
//! `docs/investigation/phase3-render.md`).
//!
//! The same semantics as the hardware model (`ssx3-ps2` `vif/mod.rs`:
//! UNPACK with STCYCL/STMASK/STROW/STCOL/STMOD, BASE/OFFSET double
//! buffering, ITOP), but into a virtual VU1 data memory, and without running
//! anything: every microprogram start becomes a [`VuCall`] carrying the
//! quadwords written since the previous start, i.e. the call's input.

/// VU1 data memory: 16 KiB = 1024 quadwords.
pub const VU1_QWORDS: usize = 1024;

/// VIF1 registers that persist between transfers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VifRegs {
    pub cycle: u32,
    pub mode: u32,
    pub mask: u32,
    pub row: [u32; 4],
    pub col: [u32; 4],
    pub base: u32,
    pub ofst: u32,
    pub tops: u32,
    pub itops: u32,
    pub dbf: bool,
}

/// One microprogram start.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VuCall {
    /// Program address in bytes (MSCAL imm × 8); `None` for MSCNT (continue).
    pub entry: Option<u32>,
    /// VIF1 TOP / ITOP the program reads (`XTOP`, `XITOP`).
    pub top: u32,
    pub itop: u32,
    /// Quadwords written by UNPACK since the previous start: (address, value),
    /// in address order; the call's input buffer plus any fixed-area uploads.
    pub input: Vec<(u16, u128)>,
}

/// A GIF stream sent through DIRECT/DIRECTHL (PATH2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Direct {
    pub qwords: Vec<u128>,
}

/// What the VIF did, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Call(VuCall),
    Direct(Direct),
    /// MPG upload (program code): address in bytes and size in bytes.
    Microcode { addr: u32, bytes: u32 },
    /// MSKPATH3.
    MaskPath3(bool),
}

pub struct Vif1 {
    pub regs: VifRegs,
    pub mem: Box<[u8; VU1_QWORDS * 16]>,
    dirty: Box<[bool; VU1_QWORDS]>,
    /// The qwords in `dirty`, in write order.
    dirty_list: Vec<u16>,
    fifo: Vec<u32>,
    pub events: Vec<Event>,
    /// Unknown VIFcodes seen.
    pub errors: u32,
}

impl Default for Vif1 {
    fn default() -> Self {
        Self::new(VifRegs::default())
    }
}

impl Vif1 {
    pub fn new(regs: VifRegs) -> Self {
        Self {
            regs,
            mem: Box::new([0; VU1_QWORDS * 16]),
            dirty: Box::new([false; VU1_QWORDS]),
            dirty_list: Vec::new(),
            fifo: Vec::new(),
            events: Vec::new(),
            errors: 0,
        }
    }

    /// Feed quadwords (one DMA transfer).
    pub fn write(&mut self, data: &[u128]) {
        let mut words = std::mem::take(&mut self.fifo);
        words.reserve(data.len() * 4);
        for q in data {
            words.extend_from_slice(&[*q as u32, (*q >> 32) as u32, (*q >> 64) as u32, (*q >> 96) as u32]);
        }
        let mut pos = 0;
        while pos < words.len() {
            let cmd = words[pos];
            let need = self.data_words(cmd);
            if pos + 1 + need > words.len() {
                break;
            }
            self.execute(cmd, &words[pos + 1..pos + 1 + need]);
            pos += 1 + need;
        }
        // (What did not complete waits for the next transfer.)
        words.drain(..pos);
        self.fifo = words;
    }

    fn cl_wl(&self) -> (u32, u32) {
        (self.regs.cycle & 0xff, (self.regs.cycle >> 8) & 0xff)
    }

    fn unpack_words(&self, cmd: u32) -> usize {
        let vn = (cmd >> 26) & 3;
        let vl = (cmd >> 24) & 3;
        let num = match (cmd >> 16) & 0xff {
            0 => 256,
            n => n,
        };
        let (cl, wl) = self.cl_wl();
        let data = if wl > cl && wl != 0 { (num / wl) * cl + (num % wl).min(cl) } else { num };
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

    fn execute(&mut self, cmd: u32, data: &[u32]) {
        let op = (cmd >> 24) & 0x7f;
        let imm = cmd & 0xffff;
        let r = &mut self.regs;
        match op {
            0x00 | 0x07 | 0x10 | 0x11 | 0x13 => {}
            0x01 => r.cycle = imm,
            0x02 => {
                r.ofst = imm & 0x3ff;
                r.dbf = false;
                r.tops = r.base;
            }
            0x03 => r.base = imm & 0x3ff,
            0x04 => r.itops = imm & 0x3ff,
            0x05 => r.mode = imm & 3,
            0x06 => self.events.push(Event::MaskPath3((imm >> 15) & 1 != 0)),
            0x14 | 0x15 => self.call(Some(imm * 8)),
            0x17 => self.call(None),
            0x20 => r.mask = data[0],
            0x30 => r.row.copy_from_slice(&data[..4]),
            0x31 => r.col.copy_from_slice(&data[..4]),
            0x4a => self.events.push(Event::Microcode { addr: imm * 8, bytes: data.len() as u32 * 4 }),
            0x50 | 0x51 => {
                let qwords = data
                    .chunks_exact(4)
                    .map(|c| c[0] as u128 | (c[1] as u128) << 32 | (c[2] as u128) << 64 | (c[3] as u128) << 96)
                    .collect();
                self.events.push(Event::Direct(Direct { qwords }));
            }
            _ if op & 0x60 == 0x60 => self.unpack(cmd, data),
            _ => self.errors += 1,
        }
    }

    /// MSCAL/MSCALF/MSCNT: TOP/ITOP latch and the TOPS double buffer, as the
    /// VIF does when it starts the VU.
    fn call(&mut self, entry: Option<u32>) {
        let r = &mut self.regs;
        let top = r.tops & 0x3ff;
        if r.dbf {
            r.tops = r.base;
            r.dbf = false;
        } else {
            r.tops = r.base + r.ofst;
            r.dbf = true;
        }
        let itop = r.itops;
        self.dirty_list.sort_unstable();
        let mut input = Vec::with_capacity(self.dirty_list.len());
        for &a in &self.dirty_list {
            let a = a as usize;
            self.dirty[a] = false;
            input.push((a as u16, u128::from_le_bytes(self.mem[a * 16..a * 16 + 16].try_into().unwrap())));
        }
        self.dirty_list.clear();
        self.events.push(Event::Call(VuCall { entry, top, itop, input }));
    }

    fn put(&mut self, addr: u32, j: usize, v: u32) {
        let a = (addr as usize) & (VU1_QWORDS - 1);
        self.mem[a * 16 + j * 4..a * 16 + j * 4 + 4].copy_from_slice(&v.to_le_bytes());
        if !self.dirty[a] {
            self.dirty[a] = true;
            self.dirty_list.push(a as u16);
        }
    }

    fn unpack(&mut self, cmd: u32, data: &[u32]) {
        let vn = (cmd >> 26) & 3;
        let vl = (cmd >> 24) & 3;
        let masked = (cmd >> 28) & 1 != 0;
        let usn = (cmd >> 14) & 1 != 0;
        let num = match (cmd >> 16) & 0xff {
            0 => 256,
            n => n,
        };
        let mut addr = cmd & 0x3ff;
        if (cmd >> 15) & 1 != 0 {
            addr += self.regs.tops;
        }
        let (cl, wl) = self.cl_wl();
        let fill_mode = wl > cl;
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
        let mut cyc = 0u32;
        for _ in 0..num {
            let fill = fill_mode && cyc >= cl;
            let mut v = [0u32; 4];
            if !fill {
                if vl == 3 {
                    let c = read(16);
                    v = [(c & 0x1f) << 3, ((c >> 5) & 0x1f) << 3, ((c >> 10) & 0x1f) << 3, ((c >> 15) & 1) << 7];
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
            for (j, val) in v.iter().enumerate() {
                let mut m = if masked { (self.regs.mask >> ((mrow * 4 + j as u32) * 2)) & 3 } else { 0 };
                if fill && m == 0 {
                    m = 1;
                }
                let out = match m {
                    3 => continue,
                    1 => self.regs.row[j],
                    2 => self.regs.col[mrow as usize],
                    _ => match self.regs.mode {
                        1 => val.wrapping_add(self.regs.row[j]),
                        2 => {
                            self.regs.row[j] = val.wrapping_add(self.regs.row[j]);
                            self.regs.row[j]
                        }
                        _ => *val,
                    },
                };
                self.put(addr, j, out);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(w: [u32; 4]) -> u128 {
        w[0] as u128 | (w[1] as u128) << 32 | (w[2] as u128) << 64 | (w[3] as u128) << 96
    }

    #[test]
    fn unpack_v4_32_then_mscal_reports_input_and_double_buffers() {
        let mut v = Vif1::new(VifRegs { base: 0x10, ofst: 0x20, ..Default::default() });
        // STCYCL 1,1; OFFSET 0x20 (tops = base); UNPACK V4-32 2 qwords at 0 + TOPS; MSCAL 0x40.
        v.write(&[
            q([0x0100_0101, 0x0200_0020, 0x6c02_8000, 1]),
            q([2, 3, 4, 5]),
            q([6, 7, 8, 0x1400_0040]),
        ]);
        let Event::Call(c) = &v.events[0] else { panic!("{:?}", v.events) };
        assert_eq!(c.entry, Some(0x200));
        assert_eq!(c.top, 0x10);
        assert_eq!(c.input, vec![(0x10, q([1, 2, 3, 4])), (0x11, q([5, 6, 7, 8]))]);
        assert_eq!(v.regs.tops, 0x30);
    }
}
