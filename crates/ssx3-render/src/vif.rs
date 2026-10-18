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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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
#[derive(Clone, Debug)]
pub struct VuCall {
    /// Program address in bytes (MSCAL imm × 8); `None` for MSCNT (continue).
    pub entry: Option<u32>,
    /// VIF1 TOP / ITOP the program reads (`XTOP`, `XITOP`).
    pub top: u32,
    pub itop: u32,
    /// Quadwords written by UNPACK since the previous start: (address, value),
    /// in address order; the call's input buffer plus any fixed-area uploads.
    pub input: Vec<(u16, u128)>,
    /// More input, shared with the chain recording it was replayed from
    /// ([`Recording`]): applied after `input` (where both write a qword,
    /// they hold the same value). See [`VuCall::writes`].
    pub shared: Option<std::sync::Arc<[(u16, u128)]>>,
    /// A stable key of where the input came from, when it is the same every
    /// time (a replayed mesh chain: [`Recording::id`] and the start's
    /// index), so its vertex data can stay on the GPU. Not part of equality.
    pub src: Option<u64>,
}

impl VuCall {
    /// Every input qword, in the order to apply them.
    pub fn writes(&self) -> impl Iterator<Item = &(u16, u128)> {
        self.input.iter().chain(self.shared.as_deref().unwrap_or(&[]))
    }

    /// The input as one list in address order.
    pub fn merged_input(&self) -> std::borrow::Cow<'_, [(u16, u128)]> {
        match &self.shared {
            None => std::borrow::Cow::Borrowed(&self.input),
            Some(_) => {
                let m: std::collections::BTreeMap<u16, u128> = self.writes().copied().collect();
                std::borrow::Cow::Owned(m.into_iter().collect())
            }
        }
    }
}

impl PartialEq for VuCall {
    fn eq(&self, o: &Self) -> bool {
        (self.entry, self.top, self.itop) == (o.entry, o.top, o.itop) && self.merged_input() == o.merged_input()
    }
}
impl Eq for VuCall {}

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
    /// A packet the game's draw function described natively
    /// (`crate::direct`): the capture's index in the frame's list, the VIF
    /// registers and VU1 qw0-5 (the header the state change uploaded) at
    /// the packet. Not decoded; its program starts only advance the double
    /// buffering ([`Vif1::skip_starts`]).
    Native { index: u32, regs: VifRegs, header: [u128; 6] },
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
    /// Recording a stretch of input ([`Vif1::record`]).
    rec: Option<Rec>,
}

/// A stretch of VIF input as replayable effects: per microprogram start,
/// the registers just before it and the qwords the stretch wrote since the
/// previous one; then the qwords written after the last start and the
/// registers at the end.
#[derive(Clone, Debug)]
pub struct Recording {
    /// Unique among recordings (set by whoever keeps it; 0: none).
    pub id: u64,
    pub calls: Vec<(VifRegs, Option<u32>, std::sync::Arc<[(u16, u128)]>)>,
    pub tail: Vec<(u16, u128)>,
    pub regs: VifRegs,
}

struct Rec {
    /// Lanes written per qword since the last start (bit j: lane j).
    lanes: Box<[u8; VU1_QWORDS]>,
    order: Vec<u16>,
    calls: Vec<(VifRegs, Option<u32>, std::sync::Arc<[(u16, u128)]>)>,
    /// A qword written in part, or an event other than a start: not
    /// replayable.
    bad: bool,
}

impl Rec {
    fn take(&mut self, mem: &[u8; VU1_QWORDS * 16]) -> Vec<(u16, u128)> {
        let mut v = Vec::with_capacity(self.order.len());
        self.order.sort_unstable();
        for &a in &self.order {
            let a = a as usize;
            if self.lanes[a] != 0xf {
                self.bad = true;
            }
            self.lanes[a] = 0;
            v.push((a as u16, u128::from_le_bytes(mem[a * 16..a * 16 + 16].try_into().unwrap())));
        }
        self.order.clear();
        v
    }
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
            rec: None,
        }
    }

    /// A copy of the state (memory, registers; no events), for checks.
    pub fn clone_state(&self) -> Vif1 {
        Vif1 {
            regs: self.regs,
            mem: self.mem.clone(),
            dirty: self.dirty.clone(),
            dirty_list: self.dirty_list.clone(),
            fifo: self.fifo.clone(),
            events: Vec::new(),
            errors: 0,
            rec: None,
        }
    }

    /// No VIFcode is waiting for data (a recording can start or replay).
    pub fn idle(&self) -> bool {
        self.fifo.is_empty()
    }

    /// Start recording the effects of the input that follows.
    pub fn record(&mut self) {
        self.rec = Some(Rec { lanes: Box::new([0; VU1_QWORDS]), order: Vec::new(), calls: Vec::new(), bad: false });
    }

    /// Stop recording: the effects since [`Vif1::record`], or `None` when
    /// they cannot be replayed (a VIFcode still waiting for data, a qword
    /// written in part, events other than microprogram starts).
    pub fn recorded(&mut self) -> Option<Recording> {
        let mut r = self.rec.take()?;
        let tail = r.take(&self.mem);
        (!r.bad && self.fifo.is_empty()).then(|| Recording { id: 0, calls: r.calls, tail, regs: self.regs })
    }

    /// Apply a recording as if its input were fed again (the registers must
    /// be the ones it was recorded from).
    pub fn replay(&mut self, r: &Recording) {
        for (k, (regs, entry, writes)) in r.calls.iter().enumerate() {
            // (The call's input: what was pending before, plus the
            // recorded writes, shared.)
            for &(a, v) in writes.iter() {
                let a = a as usize & (VU1_QWORDS - 1);
                self.mem[a * 16..a * 16 + 16].copy_from_slice(&v.to_le_bytes());
            }
            self.regs = *regs;
            self.call(*entry);
            if let Some(Event::Call(c)) = self.events.last_mut() {
                c.shared = Some(writes.clone());
                if r.id != 0 {
                    c.src = Some(r.id << 16 | k as u64);
                }
            }
        }
        self.put_qwords(&r.tail);
        self.regs = r.regs;
    }

    fn put_qwords(&mut self, w: &[(u16, u128)]) {
        for &(a, v) in w {
            let a = a as usize & (VU1_QWORDS - 1);
            self.mem[a * 16..a * 16 + 16].copy_from_slice(&v.to_le_bytes());
            if !self.dirty[a] {
                self.dirty[a] = true;
                self.dirty_list.push(a as u16);
            }
        }
    }

    fn push_event(&mut self, e: Event) {
        if let Some(r) = self.rec.as_mut() {
            r.bad = true;
        }
        self.events.push(e);
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
            0x06 => self.push_event(Event::MaskPath3((imm >> 15) & 1 != 0)),
            0x14 | 0x15 => self.call(Some(imm * 8)),
            0x17 => self.call(None),
            0x20 => r.mask = data[0],
            0x30 => r.row.copy_from_slice(&data[..4]),
            0x31 => r.col.copy_from_slice(&data[..4]),
            0x4a => self.push_event(Event::Microcode { addr: imm * 8, bytes: data.len() as u32 * 4 }),
            0x50 | 0x51 => {
                let qwords = data
                    .chunks_exact(4)
                    .map(|c| c[0] as u128 | (c[1] as u128) << 32 | (c[2] as u128) << 64 | (c[3] as u128) << 96)
                    .collect();
                self.push_event(Event::Direct(Direct { qwords }));
            }
            _ if op & 0x60 == 0x60 => self.unpack(cmd, data),
            _ => self.errors += 1,
        }
    }

    /// MSCAL/MSCALF/MSCNT: TOP/ITOP latch and the TOPS double buffer, as the
    /// VIF does when it starts the VU.
    /// `n` program starts made by a packet that is not decoded (a native
    /// draw): only the double buffering moves on. Pending input stays for
    /// the next decoded start.
    pub fn skip_starts(&mut self, n: u32) {
        for _ in 0..n {
            let r = &mut self.regs;
            r.tops = if r.dbf { r.base } else { r.base + r.ofst };
            r.dbf = !r.dbf;
        }
    }

    fn call(&mut self, entry: Option<u32>) {
        if let Some(rec) = self.rec.as_mut() {
            let w = rec.take(&self.mem);
            rec.calls.push((self.regs, entry, w.into()));
        }
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
        self.events.push(Event::Call(VuCall { entry, top, itop, input, shared: None, src: None }));
    }

    fn put(&mut self, addr: u32, j: usize, v: u32) {
        let a = (addr as usize) & (VU1_QWORDS - 1);
        self.mem[a * 16 + j * 4..a * 16 + j * 4 + 4].copy_from_slice(&v.to_le_bytes());
        if !self.dirty[a] {
            self.dirty[a] = true;
            self.dirty_list.push(a as u16);
        }
        if let Some(r) = self.rec.as_mut() {
            if r.lanes[a] == 0 {
                r.order.push(a as u16);
            }
            r.lanes[a] |= 1 << j;
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
