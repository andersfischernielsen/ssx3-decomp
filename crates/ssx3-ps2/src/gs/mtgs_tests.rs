//! MTGS: the threaded GS must be indistinguishable from the direct one.
//! Random streams of GIF packets (all three paths, PATH3 behind masks),
//! privileged register traffic, VBlanks and readbacks are run through
//! `Mtgs` with and without the GS thread; every value the guest observes
//! and the final VRAM must match.

use super::diff_tests::Rng;
use super::{Gs, Mtgs};
use crate::Qword;

fn giftag(nloop: u32, eop: bool, flg: u32, nreg: u32, regs: u64) -> Qword {
    let lo = nloop as u64 & 0x7FFF
        | (eop as u64) << 15
        | (flg as u64) << 58
        | ((nreg as u64) & 0xF) << 60;
    lo as u128 | (regs as u128) << 64
}

fn ad_q(addr: u8, value: u64) -> Qword {
    value as u128 | (addr as u128) << 64
}

fn ad_packet(list: &[(u8, u64)], eop: bool) -> Vec<Qword> {
    let mut q = vec![giftag(list.len() as u32, eop, 0, 1, 0xE)];
    q.extend(list.iter().map(|&(a, v)| ad_q(a, v)));
    q
}

fn xyz(x: u32, y: u32, z: u32) -> u64 {
    (x * 16) as u64 | ((y * 16) as u64) << 16 | (z as u64) << 32
}

/// A random GIF packet: drawing, a texture upload, SIGNAL/FINISH/LABEL, or
/// a local->host transfer.
fn random_packet(r: &mut Rng) -> Vec<Qword> {
    match r.below(11) {
        10 => {
            // REGLIST: PRIM, RGBAQ, XYZ2 x n (an odd register count pads).
            let n = r.below(5) as u32 + 1;
            let mut q = vec![giftag(n, true, 1, 3, 0x510)];
            let mut halves = vec![];
            for _ in 0..n {
                halves.push(4u64);
                halves.push(r.next() & 0xFFFF_FFFF | (1.0f32.to_bits() as u64) << 32);
                halves.push(xyz(r.below(320) as u32, r.below(224) as u32, 0));
            }
            if halves.len() % 2 == 1 {
                halves.push(0);
            }
            q.extend(
                halves
                    .chunks(2)
                    .map(|c| c[0] as u128 | (c[1] as u128) << 64),
            );
            q
        }
        0..=4 => {
            // A few primitives with random colours (flat or Gouraud).
            let prim = r.pick(&[3u64, 4, 6, 6 | 1 << 3]);
            let mut list = vec![(0x00u8, prim)];
            for _ in 0..r.below(12) + 3 {
                list.push((
                    0x01,
                    r.next() & 0xFFFF_FFFF | (1.0f32.to_bits() as u64) << 32,
                ));
                list.push((
                    0x05,
                    xyz(
                        r.below(320) as u32,
                        r.below(224) as u32,
                        r.below(1 << 20) as u32,
                    ),
                ));
            }
            ad_packet(&list, true)
        }
        5..=6 => {
            // CT32/CT16/T8 upload of a small rectangle, the IMAGE data
            // possibly in several tags.
            let psm = r.pick(&[0u64, 2, 0x13]);
            let (w, h) = (8 * (r.below(4) + 1), 2 * (r.below(8) + 1));
            let dbp = 0x1000 + r.below(64) * 32;
            let mut q = ad_packet(
                &[
                    (0x50, dbp << 32 | 2u64 << 48 | psm << 56),
                    (0x51, r.below(16) << 32 | r.below(16) << 48),
                    (0x52, w | h << 32),
                    (0x53, 0),
                ],
                false,
            );
            let bpp = match psm {
                0 => 32,
                2 => 16,
                _ => 8,
            };
            let mut n = (w * h * bpp / 128) as u32;
            while n > 0 {
                let k = (r.below(n as u64) + 1) as u32;
                n -= k;
                q.push(giftag(k, n == 0, 2, 0, 0));
                for _ in 0..k {
                    q.push(r.next() as u128 | (r.next() as u128) << 64);
                }
            }
            q
        }
        7 => ad_packet(
            &[
                (0x60, r.next() & 0xFFFF_0000_FFFF),
                (0x62, r.next() & 0xFFFF_0000_FFFF),
            ],
            true,
        ),
        8 => ad_packet(&[(0x61, 0)], true),
        _ => ad_packet(
            &[
                (0x50, 0 | 10 << 16),
                (0x51, r.below(64) | r.below(64) << 16),
                (0x52, 8 | 4u64 << 32),
                (0x53, 1),
            ],
            true,
        ),
    }
}

fn setup() -> Vec<Qword> {
    ad_packet(
        &[
            (0x4C, 10 << 16),
            (0x40, 639 << 16 | 447 << 48),
            (0x18, 0),
            (0x47, 0),
            (0x4E, 0x80 << 0 | 0 << 24),
        ],
        true,
    )
}

/// The machine-facing GS API, for running one session on `Gs` itself and
/// on `Mtgs` (direct and threaded).
trait Dev {
    fn gif_write(&mut self, path: usize, d: &[Qword]);
    fn path3_dma(&mut self, d: &[Qword]);
    fn set_path3_mask(&mut self, m: bool);
    fn path3_flow(&mut self);
    fn set_path3_mask_reg(&mut self, m: bool);
    fn path3_pending(&self) -> usize;
    fn gif_stat(&self) -> u32;
    fn read_priv(&mut self, o: u32) -> u64;
    fn write_priv(&mut self, o: u32, v: u64);
    fn write_priv32(&mut self, o: u32, hi: bool, v: u32);
    fn irq_pending(&mut self) -> bool;
    fn vblank(&mut self, f: u32);
    fn sync(&mut self);
    fn read_local(&mut self, n: usize) -> Vec<Qword>;
    fn finish(&mut self) -> (u64, Vec<u32>);
}

macro_rules! fwd {
    () => {
        fn gif_write(&mut self, path: usize, d: &[Qword]) {
            self.gif_write(path, d)
        }
        fn path3_dma(&mut self, d: &[Qword]) {
            self.path3_dma(d)
        }
        fn set_path3_mask(&mut self, m: bool) {
            self.set_path3_mask(m)
        }
        fn path3_flow(&mut self) {
            self.path3_flow()
        }
        fn set_path3_mask_reg(&mut self, m: bool) {
            self.set_path3_mask_reg(m)
        }
        fn path3_pending(&self) -> usize {
            self.path3_pending()
        }
        fn gif_stat(&self) -> u32 {
            self.gif_stat()
        }
        fn read_priv(&mut self, o: u32) -> u64 {
            self.read_priv(o)
        }
        fn write_priv(&mut self, o: u32, v: u64) {
            self.write_priv(o, v)
        }
        fn vblank(&mut self, f: u32) {
            self.vblank(f)
        }
        fn sync(&mut self) {
            self.sync()
        }
        fn read_local(&mut self, n: usize) -> Vec<Qword> {
            self.read_local(n)
        }
    };
}

impl Dev for Gs {
    fwd!();
    fn write_priv32(&mut self, o: u32, hi: bool, v: u32) {
        // What the bus did before MTGS: read, merge, write.
        let cur = self.read_priv(o);
        let nv = if hi {
            (cur & 0xffff_ffff) | (v as u64) << 32
        } else {
            (cur & !0xffff_ffff) | v as u64
        };
        self.write_priv(o, nv);
    }
    fn irq_pending(&mut self) -> bool {
        Gs::irq_pending(self)
    }
    fn finish(&mut self) -> (u64, Vec<u32>) {
        self.sync();
        (self.prims_drawn, self.vram.clone())
    }
}

impl Dev for Mtgs {
    fwd!();
    fn write_priv32(&mut self, o: u32, hi: bool, v: u32) {
        self.write_priv32(o, hi, v)
    }
    fn irq_pending(&mut self) -> bool {
        Mtgs::irq_pending(self)
    }
    fn finish(&mut self) -> (u64, Vec<u32>) {
        self.sync();
        let mut core = self.core();
        core.sync();
        (core.prims_drawn, core.vram.clone())
    }
}

/// Run a random session and return everything the guest observed.
/// `mode`: 0 = `Gs`, 1 = `Mtgs` direct, 2 = `Mtgs` threaded.
fn session(seed: u64, mode: u32) -> (Vec<u64>, Vec<u32>) {
    let mut gs = Gs::new_cpu();
    gs.set_deferred(true);
    match mode {
        0 => run_session(seed, &mut gs),
        _ => run_session(seed, &mut Mtgs::with_thread(gs, mode == 2)),
    }
}

fn run_session(seed: u64, m: &mut impl Dev) -> (Vec<u64>, Vec<u32>) {
    let mut r = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut seen = Vec::new();
    m.gif_write(2, &setup());
    for step in 0..600 {
        match r.below(20) {
            0..=5 => m.gif_write(1, &random_packet(&mut r)),
            6..=7 => m.gif_write(2, &random_packet(&mut r)),
            8..=10 => m.path3_dma(&random_packet(&mut r)),
            11 => m.set_path3_mask(r.chance(50)),
            12 => m.path3_flow(),
            13 => m.set_path3_mask_reg(r.chance(20)),
            14 => {
                seen.push(m.read_priv(0x1000));
                seen.push(m.read_priv(0x1080));
            }
            15 => {
                // IMR (sometimes unmasking SIGNAL/FINISH) and CSR acks.
                let imr = 0x7F00 & !(r.below(0x20) << 8);
                m.write_priv(0x1010, imr);
                m.write_priv32(0x1000, false, r.below(0x20) as u32);
            }
            16 => {
                m.vblank(step & 1);
                m.sync();
            }
            17 => {
                let q = m.read_local(r.below(4) as usize + 1);
                seen.extend(q.iter().map(|&q| q as u64 ^ (q >> 64) as u64));
                seen.push(q.len() as u64);
            }
            18 => m.write_priv(r.pick(&[0x00u32, 0x70, 0x90, 0xE0, 0x1040]), r.next()),
            _ => seen
                .push(m.read_priv(r.pick(&[0x00u32, 0x70, 0x90, 0xE0, 0x1010, 0x1040])) ^ 0x5555),
        }
        seen.push(m.irq_pending() as u64);
        seen.push(m.gif_stat() as u64);
        seen.push(m.path3_pending() as u64);
    }
    let (prims, vram) = m.finish();
    seen.push(prims);
    (seen, vram)
}

#[test]
fn threaded_matches_direct() {
    for seed in 0..12 {
        let (a, va) = session(seed, 0);
        assert!(a.iter().any(|&v| v == 1), "some interrupt is seen");
        for mode in [1, 2] {
            let (b, vb) = session(seed, mode);
            assert_eq!(a.len(), b.len(), "seed {seed} mode {mode}");
            if let Some(i) = (0..a.len()).find(|&i| a[i] != b[i]) {
                panic!(
                    "seed {seed} mode {mode}: observation {i} differs: {:#x} vs {:#x}",
                    a[i], b[i]
                );
            }
            assert!(va == vb, "seed {seed} mode {mode}: VRAM differs");
        }
    }
}

#[test]
fn path3_mask_through_mtgs() {
    // The PATH3 arbitration lives on the guest side; check the hardware
    // outcome of a mask pulse through the threaded GS.
    let mut m = Mtgs::with_thread(Gs::new_cpu(), true);
    let up = |dbp: u64, c: u32| {
        let mut q = ad_packet(
            &[
                (0x50, dbp << 32 | 1u64 << 48),
                (0x51, 0),
                (0x52, 8 | 8u64 << 32),
                (0x53, 0),
            ],
            false,
        );
        q.push(giftag(16, true, 2, 0, 0));
        let w = c as u128;
        q.extend((0..16).map(|_| w | w << 32 | w << 64 | w << 96));
        q
    };
    let texel = |m: &mut Mtgs| {
        super::swizzle::read_pixel(&m.core().vram, super::swizzle::PSMCT32, 0x100, 1, 3, 3)
    };
    m.set_path3_mask(true);
    let mut q = up(0x100, 0x6666_6666);
    q.extend(up(0x100, 0x7777_7777));
    m.path3_dma(&q);
    assert_eq!(texel(&mut m), 0);
    assert_ne!(m.gif_stat() & (1 << 6), 0, "P3Q");
    m.set_path3_mask(false);
    m.set_path3_mask(true);
    assert_eq!(texel(&mut m), 0x6666_6666);
    m.set_path3_mask(false);
    m.set_path3_mask(true);
    assert_eq!(texel(&mut m), 0x7777_7777);
    assert_eq!(m.path3_pending(), 0);
}
