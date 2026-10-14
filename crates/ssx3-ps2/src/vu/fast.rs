//! Helpers for statically translated microcode (`generated/vu/`).
//!
//! The reference interpreter and macro mode drive the per-pair model in
//! `mod.rs` (`begin_pair` / staged upper / `end_pair`, every pipeline
//! advanced every pair). Translated code does the same work, but the
//! translator (`recomp.rs`) decides at translate time what each pair
//! actually needs, and calls these small `#[inline(always)]` pieces:
//!
//! - **Stalls:** only the VF fields that can still be in flight are checked
//!   (`stall`). A field written more than 3 pairs back is always ready.
//! - **Results:** the upper op is computed into locals (`fmac`, `plain`),
//!   the lower op runs against the old state, then the upper result is
//!   written (`put_vf`). No staging struct.
//! - **Ready times** (`vf_ready`, `ready_max`) are written only when a read
//!   within 3 pairs, a program end or a dynamic jump can observe them.
//! - **Q, P and the flag pipelines** are retired lazily, right before an op
//!   that observes them (`q_sync`, `p_sync`, `flags_sync`). A flag result
//!   that no flag op can observe within its latency is made visible at once
//!   (`flags_eager`) instead of being queued.
//! - **VI backup** (`vi_bk`, `vi_brf`): the backed-up register carries the
//!   index of the pair that wrote it, instead of being cleared every pair.
//!
//! The result is the same architectural state, cycle count, flags and memory
//! as the per-pair model (checked by the oracle test and `vu_diff digest`).

use super::float::{self, Exc, ABS, SIGN};
use super::{FlagSnap, Vu, CLIP_LATENCY, FMAC_LATENCY, STATUS_DIV, STATUS_FMAC};

#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline(always)]
fn lane_mac(l: u32, v: u32, e: Exc) -> u32 {
    let b = 3 - l;
    (((v & ABS == 0) as u32) << b)
        | (((v & SIGN != 0) as u32) << (b + 4))
        | ((e.u as u32) << (b + 8))
        | ((e.o as u32) << (b + 12))
}

/// MAC flags of the lanes in `D` (bit 3 = x) from per-lane results.
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline(always)]
fn mac_of<const D: u8>(v: &[u32; 4], e: &[Exc; 4]) -> u32 {
    let mut mac = 0;
    for l in 0..4 {
        if D & (8 >> l) != 0 {
            mac |= lane_mac(l as u32, v[l], e[l]);
        }
    }
    mac
}

// FMAC ops on all four lanes (straight-line, so LLVM can vectorise them);
// only the lanes in `D` are flagged and later written. Broadcast, Q and I
// operands are passed as `[x; 4]`.
#[allow(unused_macros)]
macro_rules! fmac2 {
    ($name:ident, $f:path) => {
        #[inline(always)]
        pub fn $name<const D: u8, const NS: bool, const NT: bool>(s: &[u32; 4], t: &[u32; 4]) -> ([u32; 4], u32) {
            let mut v = [0u32; 4];
            let mut e = [Exc::default(); 4];
            for l in 0..4 {
                let (x, y) = $f(s[l], t[l]);
                v[l] = x;
                e[l] = y;
            }
            (v, mac_of::<D>(&v, &e))
        }
    };
}
#[allow(unused_macros)]
macro_rules! fmac3 {
    ($name:ident, $f:path) => {
        #[inline(always)]
        pub fn $name<const D: u8, const NA: bool, const NS: bool, const NT: bool>(
            a: &[u32; 4],
            s: &[u32; 4],
            t: &[u32; 4],
        ) -> ([u32; 4], u32) {
            let mut v = [0u32; 4];
            let mut e = [Exc::default(); 4];
            for l in 0..4 {
                let (x, y) = $f(a[l], s[l], t[l]);
                v[l] = x;
                e[l] = y;
            }
            (v, mac_of::<D>(&v, &e))
        }
    };
}
#[cfg(not(target_arch = "aarch64"))]
fmac2!(add4, float::add);
#[cfg(not(target_arch = "aarch64"))]
fmac2!(sub4, float::sub);
#[cfg(not(target_arch = "aarch64"))]
fmac2!(mul4, float::mul);
#[cfg(not(target_arch = "aarch64"))]
fmac3!(madd4, float::madd);
#[cfg(not(target_arch = "aarch64"))]
fmac3!(msub4, float::msub);

// NEON: four lanes per instruction (`simd.rs`), same bits as the scalar code.
#[cfg(target_arch = "aarch64")]
mod neon_ops {
    use super::super::simd::{self, load, store};
    // `NS`, `NT`, `NA`: whether an operand must be normalised (false when
    // the translator knows it holds an FMAC/ITOF result).
    macro_rules! neon2 {
        ($name:ident, |$s:ident, $t:ident| $e:expr) => {
            #[inline(always)]
            pub fn $name<const D: u8, const NS: bool, const NT: bool>(s: &[u32; 4], t: &[u32; 4]) -> ([u32; 4], u32) {
                let ($s, $t) = (load(s), load(t));
                let r = $e;
                (store(r.v), simd::mac::<D>(&r))
            }
        };
    }
    macro_rules! neon3 {
        ($name:ident, $f:ident) => {
            #[inline(always)]
            pub fn $name<const D: u8, const NA: bool, const NS: bool, const NT: bool>(
                a: &[u32; 4],
                s: &[u32; 4],
                t: &[u32; 4],
            ) -> ([u32; 4], u32) {
                let r = simd::$f::<NA, NS, NT>(load(a), load(s), load(t));
                (store(r.v), simd::mac::<D>(&r))
            }
        };
    }
    neon2!(add4, |s, t| simd::add_k::<NS, NT>(s, t));
    // (Negating keeps a normal operand normal.)
    neon2!(sub4, |s, t| simd::add_k::<NS, NT>(s, simd::neg(t)));
    neon2!(mul4, |s, t| simd::mul_k::<NS, NT>(s, t));
    neon3!(madd4, madd_k);
    neon3!(msub4, msub_k);

    #[inline(always)]
    pub fn itof4<const F: i32>(s: &[u32; 4]) -> [u32; 4] {
        store(simd::itof::<F>(load(s)))
    }
    #[inline(always)]
    pub fn ftoi4<const F: i32>(s: &[u32; 4]) -> [u32; 4] {
        store(simd::ftoi::<F>(load(s)))
    }
}
#[cfg(target_arch = "aarch64")]
pub use neon_ops::{add4, ftoi4, itof4, madd4, msub4, mul4, sub4};

#[inline(always)]
pub fn max4(s: &[u32; 4], t: &[u32; 4]) -> [u32; 4] {
    std::array::from_fn(|l| float::max(s[l], t[l]))
}
#[inline(always)]
pub fn min4(s: &[u32; 4], t: &[u32; 4]) -> [u32; 4] {
    std::array::from_fn(|l| float::min(s[l], t[l]))
}
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn itof4<const F: i32>(s: &[u32; 4]) -> [u32; 4] {
    std::array::from_fn(|l| float::from_fixed(s[l] as i32, F))
}
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn ftoi4<const F: i32>(s: &[u32; 4]) -> [u32; 4] {
    std::array::from_fn(|l| float::to_fixed(s[l], F))
}
#[inline(always)]
pub fn abs4(s: &[u32; 4]) -> [u32; 4] {
    std::array::from_fn(|l| s[l] & ABS)
}

/// Out-of-line copies for the rarely run single-pair arms (keeps the
/// translated functions small).
pub mod cold {
    macro_rules! cold2 {
        ($name:ident) => {
            #[inline(never)]
            pub fn $name<const D: u8>(s: &[u32; 4], t: &[u32; 4]) -> ([u32; 4], u32) {
                super::$name::<D, true, true>(s, t)
            }
        };
    }
    macro_rules! cold3 {
        ($name:ident) => {
            #[inline(never)]
            pub fn $name<const D: u8>(a: &[u32; 4], s: &[u32; 4], t: &[u32; 4]) -> ([u32; 4], u32) {
                super::$name::<D, true, true, true>(a, s, t)
            }
        };
    }
    cold2!(add4);
    cold2!(sub4);
    cold2!(mul4);
    cold3!(madd4);
    cold3!(msub4);
    #[inline(never)]
    pub fn max4(s: &[u32; 4], t: &[u32; 4]) -> [u32; 4] {
        super::max4(s, t)
    }
    #[inline(never)]
    pub fn min4(s: &[u32; 4], t: &[u32; 4]) -> [u32; 4] {
        super::min4(s, t)
    }
    #[inline(never)]
    pub fn itof4<const F: i32>(s: &[u32; 4]) -> [u32; 4] {
        super::itof4::<F>(s)
    }
    #[inline(never)]
    pub fn ftoi4<const F: i32>(s: &[u32; 4]) -> [u32; 4] {
        super::ftoi4::<F>(s)
    }
    #[inline(never)]
    pub fn abs4(s: &[u32; 4]) -> [u32; 4] {
        super::abs4(s)
    }
}

/// `d[l] = v[l]` for the lanes in `D`. On AArch64 a partial write is one
/// 128-bit read-select-write, so a later full-register load forwards from
/// a single store (partial lane stores would stall it).
#[inline(always)]
fn merge<const D: u8>(d: &mut [u32; 4], v: &[u32; 4]) {
    #[cfg(target_arch = "aarch64")]
    {
        if D == 0b1111 {
            *d = *v;
        } else if D != 0 {
            *d = super::simd::select::<D>(v, d);
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        for l in 0..4 {
            if D & (8 >> l) != 0 {
                d[l] = v[l];
            }
        }
    }
}

/// CLIP judgement bits of `s.xyz` against `|w|` (see `Vu::clipw`).
#[inline(always)]
pub fn clip(s: [u32; 4], w: u32) -> u32 {
    let w = float::norm_in(w) & ABS;
    let wk = w as i32;
    let mut flags = 0u32;
    for l in 0..3 {
        let x = float::norm_in(s[l]);
        let k = if x & SIGN != 0 {
            -((x & ABS) as i32)
        } else {
            (x & ABS) as i32
        };
        if k > wk {
            flags |= 1 << (l * 2);
        }
        if k < -wk {
            flags |= 2 << (l * 2);
        }
    }
    flags
}

impl Vu {
    /// Stall until cycle `need` (the max of the ready times of the fields
    /// this pair reads). A stalling pair sees no VI backup (microVU
    /// `analyzeBranchVI`), as in `begin_pair`.
    #[inline(always)]
    pub fn stall(&mut self, need: u64) {
        if need > self.cycle {
            self.cycle = need;
            self.fbk_reg = 0;
        }
    }

    /// Write lanes `D` of VF `r` (r != 0).
    #[inline(always)]
    pub fn put_vf<const D: u8>(&mut self, r: usize, v: &[u32; 4]) {
        merge::<D>(&mut self.vf[r & 31], v);
    }

    #[inline(always)]
    pub fn put_acc<const D: u8>(&mut self, v: &[u32; 4]) {
        merge::<D>(&mut self.acc, v);
    }

    /// The fields `F` of VF `r` are ready FMAC_LATENCY cycles from now.
    #[inline(always)]
    pub fn ready<const F: u8>(&mut self, r: usize) {
        let t = self.cycle + FMAC_LATENCY;
        let d = &mut self.vf_ready[r & 31];
        if F & 8 != 0 {
            d[0] = t;
        }
        if F & 4 != 0 {
            d[1] = t;
        }
        if F & 2 != 0 {
            d[2] = t;
        }
        if F & 1 != 0 {
            d[3] = t;
        }
    }

    /// A VF write near a program end or dynamic jump: `flush` must see it.
    #[inline(always)]
    pub fn ready_max_now(&mut self) {
        let t = self.cycle + FMAC_LATENCY;
        if t > self.ready_max {
            self.ready_max = t;
        }
    }

    /// Commit an FMAC op's MAC flags and the status bits derived from them.
    #[inline(always)]
    pub fn set_mac(&mut self, mac: u32) {
        self.mac = mac;
        let s = ((mac & 0x000f != 0) as u32)
            | (((mac & 0x00f0 != 0) as u32) << 1)
            | (((mac & 0x0f00 != 0) as u32) << 2)
            | (((mac & 0xf000 != 0) as u32) << 3);
        self.status = (self.status & !0xf) | s | (s << 6);
    }

    /// Commit a run of FMAC ops at once: `last` is the final MAC, `acc` the
    /// OR of all their MACs (the sticky status bits are an OR, so this equals
    /// committing them one by one).
    #[inline(always)]
    pub fn mac_commit(&mut self, last: u32, acc: u32) {
        let nib = |m: u32| {
            ((m & 0x000f != 0) as u32)
                | (((m & 0x00f0 != 0) as u32) << 1)
                | (((m & 0x0f00 != 0) as u32) << 2)
                | (((m & 0xf000 != 0) as u32) << 3)
        };
        self.mac = last;
        self.status = (self.status & !0xf) | nib(last) | (nib(acc) << 6);
    }

    /// Flags no flag op can observe before they would be visible anyway:
    /// make them visible now. Older queued snapshots are superseded.
    #[inline(always)]
    pub fn flags_eager(&mut self) {
        self.vis_mac = self.mac;
        self.vis_status = (self.vis_status & STATUS_DIV) | (self.status & STATUS_FMAC);
        if !self.flag_pipe.is_empty() {
            self.flag_pipe.clear();
        }
    }

    /// Queue the current flags, visible FMAC_LATENCY cycles from now.
    #[inline(always)]
    pub fn flags_push(&mut self) {
        self.retire_flags();
        self.flag_pipe.push_back(FlagSnap {
            ready: self.cycle + FMAC_LATENCY,
            mac: self.mac,
            status: self.status,
            clip: self.clip,
        });
    }

    /// CLIP: shift in the judgement bits; queue them or make them visible.
    #[inline(always)]
    pub fn clip_commit(&mut self, c: u32, push: bool) {
        self.clip = ((self.clip << 6) | c) & 0xff_ffff;
        if push {
            self.retire_clip();
            self.clip_pipe.push_back((self.cycle + CLIP_LATENCY, self.clip));
            self.flags_push();
        } else {
            self.vis_clip = self.clip;
            if !self.clip_pipe.is_empty() {
                self.clip_pipe.clear();
            }
            self.flags_eager();
        }
    }

    #[inline(always)]
    fn retire_flags(&mut self) {
        while let Some(f) = self.flag_pipe.front() {
            if f.ready > self.cycle {
                break;
            }
            let f = *f;
            self.flag_pipe.pop_front();
            self.vis_mac = f.mac;
            self.vis_status = (self.vis_status & STATUS_DIV) | (f.status & STATUS_FMAC);
        }
    }

    #[inline(always)]
    fn retire_clip(&mut self) {
        while let Some(&(ready, c)) = self.clip_pipe.front() {
            if ready > self.cycle {
                break;
            }
            self.clip_pipe.pop_front();
            self.vis_clip = c;
        }
    }

    /// Before a flag op: retire the flag snapshots that are due.
    #[inline(always)]
    pub fn flags_sync(&mut self) {
        self.retire_flags();
        self.retire_clip();
    }

    /// Before an op that reads Q or the status flags: land a due DIV result.
    #[inline(always)]
    pub fn q_sync(&mut self) {
        if let Some(p) = self.q_pend {
            if p.ready <= self.cycle {
                self.q_pend = None;
                self.apply_q(p);
            }
        }
    }

    /// Before MFP: land a due EFU result.
    #[inline(always)]
    pub fn p_sync(&mut self) {
        if let Some(p) = self.p_pend {
            if p.ready <= self.cycle {
                self.p_pend = None;
                self.p = p.value;
            }
        }
    }

    /// VI write that the next pair's conditional branch reads as the old
    /// value (`set_vi_bk`). `tag` is this pair's index (`pairs` numbering).
    #[inline(always)]
    pub fn vi_bk(&mut self, r: u8, v: u16, tag: u64) {
        let r = r & 15;
        let old = if self.fbk_reg == r && self.fbk_tag.wrapping_add(1) == tag {
            self.fbk_old
        } else {
            self.vi[r as usize]
        };
        self.fbk_reg = r;
        self.fbk_old = old;
        self.fbk_tag = tag;
        self.vi[r as usize] = v;
    }

    /// VI `r` (!= 0) as read by the conditional branch of pair `tag`.
    #[inline(always)]
    pub fn vi_brf(&self, r: u8, tag: u64) -> u16 {
        let r = r & 15;
        if self.fbk_reg == r && self.fbk_tag.wrapping_add(1) == tag && super::vi_backup_enabled() {
            self.fbk_old
        } else {
            self.vi[r as usize]
        }
    }

    /// Plain VI write (ILW, flag ops, links, XTOP): no backup.
    #[inline(always)]
    pub fn vi_set(&mut self, r: u8, v: u16) {
        self.set_vi(r, v);
    }

    /// Byte offset of data qword `a` (wrapping at the end of data memory).
    #[inline(always)]
    fn qoff(&self, a: u32) -> usize {
        ((a & (self.data_qwords() - 1)) as usize) * 16
    }

    /// Load the qword at qword address `a`.
    #[inline(always)]
    pub fn ldq(&self, a: u32) -> [u32; 4] {
        let o = self.qoff(a);
        let b: &[u8; 16] = self.data[o..o + 16].try_into().unwrap();
        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        ]
    }

    /// Store lanes `D` of `v` at qword address `a`.
    #[inline(always)]
    pub fn stq<const D: u8>(&mut self, a: u32, v: &[u32; 4]) {
        let o = self.qoff(a);
        let b: &mut [u8; 16] = (&mut self.data[o..o + 16]).try_into().unwrap();
        if D & 8 != 0 {
            b[0..4].copy_from_slice(&v[0].to_le_bytes());
        }
        if D & 4 != 0 {
            b[4..8].copy_from_slice(&v[1].to_le_bytes());
        }
        if D & 2 != 0 {
            b[8..12].copy_from_slice(&v[2].to_le_bytes());
        }
        if D & 1 != 0 {
            b[12..16].copy_from_slice(&v[3].to_le_bytes());
        }
    }

    /// RNEXT: step the LFSR and return R.
    #[inline(always)]
    pub fn r_next(&mut self) -> u32 {
        self.lfsr_step();
        self.r
    }
}
