//! SNDDRV's output stage: resampling of the EE PCM block and the IOP mix bus
//! into one AutoDMA half (`docs/investigation/phase3-sound.md` §3).
//!
//! Literal ports of `SNDIOP_ee36_iop24` (0x3510c, the SSX 3 path),
//! `SNDIOP_ee24_iop24` (0x34e04), `SNDIOP_ee36_iop36` (0x354bc) and
//! `SNDIOP_ee48_iop48` (0x35824), plus the stereo part of the dispatch in
//! `SNDIOP_mix` (0x36b74). The `*_spu48` (DTS, 4+ channels) variants are not
//! ported.
//!
//! Layouts (all verified against the decompile and the tables in SNDDRV's
//! data at 0x3a3ec..0x3a40c):
//! - EE block: planar s16, plane 0 = R, plane 1 = L, `N` frames per plane
//!   (N = 256 / 384 / 512 for 24 / 36 / 48 kHz).
//! - IOP buses: `iop_bus[0]` = R, `iop_bus[1]` = L, int32, scale x128
//!   (the driver's `b3a0[ch]`). Every element that is read is cleared.
//! - Output half (0x800 bytes): `[L 256][R 256][L 256][R 256]` s16, i.e.
//!   sub-block 0 then sub-block 1, each with its L block first.
//!
//! Per sub-block `s` and output block `c` (0 = L, 1 = R) the routines use
//! these offsets (in elements):
//!
//! | routine | EE table (s0 L, s0 R, s1 L, s1 R) | IOP bus | IOP offset per s |
//! |---|---|---|---|
//! | ee24_iop24 (0x3a3ec) | 256, 0, 384, 128 | L→1, R→0 (0x3a3f4) | 128 |
//! | ee36_iop24 (0x3a3fc) | 384, 0, 576, 192 | L→1, R→0 | 128 |
//! | ee36_iop36 (0x3a3fc) | 384, 0, 576, 192 | L→1, R→0 | 192 |
//! | ee48_iop48 (0x3a404) | 512, 0, 768, 256 | L→1, R→0 | 256 |
//!
//! Mono (`channels == 1`): `SNDIOP_mix` copies IOP bus 0 over bus 1 before
//! resampling (`memcpy(b3a4, b3a0, 256*4)`); that copy is the caller's job.

/// EE sample offsets, indexed `[s][c]` (c: 0 = L block, 1 = R block).
const EE24: [[usize; 2]; 2] = [[0x100, 0x000], [0x180, 0x080]];
const EE36: [[usize; 2]; 2] = [[0x180, 0x000], [0x240, 0x0c0]];
const EE48: [[usize; 2]; 2] = [[0x200, 0x000], [0x300, 0x100]];
/// IOP bus index per output block (0x3a3f4): L block reads bus 1, R bus 0.
const IOP_BUS: [usize; 2] = [1, 0];

/// `x < -0x8000 ? -0x8000 : x < 0x8000 ? x : 0x7fff`, as in every routine.
#[inline]
fn clamp16(x: i32) -> i16 {
    if x < -0x8000 {
        -0x8000
    } else if x < 0x8000 {
        x as i16
    } else {
        0x7fff
    }
}

/// Persistent state of the output resamplers: SNDDRV's `b2f8[2]` (s16, last
/// EE sample or last combined sample) and `b304[2]` (i32, last IOP sample),
/// indexed by output block (0 = L, 1 = R). Initialised to 0 (BSS).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Resampler {
    pub pe: [i16; 2],
    pub pp: [i32; 2],
}

impl Resampler {
    pub fn new() -> Self {
        Self::default()
    }

    /// The stereo dispatch of `SNDIOP_mix` (0x36b74): picks the routine for
    /// `(ee_rate, iop_rate)` and writes one AutoDMA half.
    ///
    /// `ee_pcm` is the planar EE block (plane 0 = R, plane 1 = L), `iop_bus`
    /// the IOP buses (0 = R, 1 = L; 256 ints for a 24 kHz IOP, 384 for 36 kHz,
    /// 512 for 48 kHz), cleared as they are read. Returns false, touching
    /// nothing, for a combination SNDDRV does not handle (the original then
    /// leaves the half as it was).
    pub fn mix(
        &mut self,
        ee_rate: u32,
        iop_rate: u32,
        ee_pcm: &[i16],
        iop_bus: &mut [Vec<i32>],
        half: &mut [i16; 1024],
    ) -> bool {
        match (ee_rate, iop_rate) {
            (36000, 24000) => self.ee36_iop24(ee_pcm, iop_bus, half),
            (36000, 36000) => self.ee36_iop36(ee_pcm, iop_bus, half),
            (48000, 48000) => ee48_iop48(ee_pcm, iop_bus, half),
            (24000, 24000) => self.ee24_iop24(ee_pcm, iop_bus, half),
            _ => return false,
        }
        true
    }

    /// `SNDIOP_ee36_iop24` (0x3510c). EE 3:4 and IOP 1:2 linear
    /// interpolation with a one-input-sample delay. `ee_pcm`: 2 x 384 s16;
    /// buses: 256 ints each.
    ///
    /// With `e[-1] = pe[c]`, `p[-1] = pp[c]`, for j in 0..64:
    /// ```text
    /// out[4j  ] = clamp( e[3j-1]                     + (p[2j-1] >> 7) )
    /// out[4j+1] = clamp( ((e[3j-1] + 3*e[3j]) >> 2)   + ((p[2j-1] + p[2j]) >> 8) )
    /// out[4j+2] = clamp( ((e[3j] + e[3j+1]) >> 1)     + (p[2j] >> 7) )
    /// out[4j+3] = clamp( ((3*e[3j+1] + e[3j+2]) >> 2) + ((p[2j] + p[2j+1]) >> 8) )
    /// ```
    /// then `pe[c] = e[191]` (raw), `pp[c] = p[127]`, `p[0..128] = 0`.
    pub fn ee36_iop24(&mut self, ee_pcm: &[i16], iop_bus: &mut [Vec<i32>], half: &mut [i16; 1024]) {
        for s in 0..2 {
            for c in 0..2 {
                let e = &ee_pcm[EE36[s][c]..EE36[s][c] + 192];
                let p = &mut iop_bus[IOP_BUS[c]][s * 128..s * 128 + 128];
                let out = &mut half[(s * 2 + c) * 256..(s * 2 + c) * 256 + 256];
                let pe = self.pe[c] as i32;
                let pp = self.pp[c];
                let e0 = e[0] as i32;
                let e1 = e[1] as i32;
                let e2 = e[2] as i32;
                out[0] = clamp16(pe.wrapping_add(pp >> 7));
                out[1] = clamp16(((pe + e0 * 3) >> 2).wrapping_add(pp.wrapping_add(p[0]) >> 8));
                out[2] = clamp16(((e0 + e1) >> 1).wrapping_add(p[0] >> 7));
                out[3] = clamp16(((e1 * 3 + e2) >> 2).wrapping_add(p[0].wrapping_add(p[1]) >> 8));
                self.pe[c] = e[0xbf];
                self.pp[c] = p[0x7f];
                p[0] = 0;
                // Loop: e from index 2, p from index 1, out from 4.
                let mut ei = 2usize;
                let mut pi = 1usize;
                let mut o = 4usize;
                while o < 256 {
                    let a = e[ei] as i32;
                    let b = e[ei + 1] as i32;
                    let cc = e[ei + 2] as i32;
                    let d = e[ei + 3] as i32;
                    let p0 = p[pi];
                    let p1 = p[pi + 1];
                    let p2 = p[pi + 2];
                    out[o] = clamp16(a.wrapping_add(p0 >> 7));
                    out[o + 1] = clamp16(((a + b * 3) >> 2).wrapping_add(p0.wrapping_add(p1) >> 8));
                    out[o + 2] = clamp16(((b + cc) >> 1).wrapping_add(p1 >> 7));
                    out[o + 3] = clamp16(((cc * 3 + d) >> 2).wrapping_add(p1.wrapping_add(p2) >> 8));
                    ei += 3;
                    p[pi] = 0;
                    p[pi + 1] = 0;
                    o += 4;
                    pi += 2;
                }
                p[0x7f] = 0;
            }
        }
    }

    /// `SNDIOP_ee24_iop24` (0x34e04). 1:2 linear interpolation of the
    /// combined signal `x[i] = e[i] + (p[i] >> 7)` (unclamped), with
    /// `x[-1] = pe[c]` (stored clamped):
    /// `out[2i] = clamp(x[i-1])`, `out[2i+1] = clamp((x[i-1] + x[i]) >> 1)`,
    /// then `pe[c] = clamp(x[127])`. `ee_pcm`: 2 x 256; buses: 256 ints.
    pub fn ee24_iop24(&mut self, ee_pcm: &[i16], iop_bus: &mut [Vec<i32>], half: &mut [i16; 1024]) {
        for s in 0..2 {
            for c in 0..2 {
                let e = &ee_pcm[EE24[s][c]..EE24[s][c] + 128];
                let p = &mut iop_bus[IOP_BUS[c]][s * 128..s * 128 + 128];
                let out = &mut half[(s * 2 + c) * 256..(s * 2 + c) * 256 + 256];
                let pe = self.pe[c] as i32;
                out[0] = self.pe[c];
                let x0 = (e[0] as i32).wrapping_add(p[0] >> 7);
                out[1] = clamp16(pe.wrapping_add(x0) >> 1);
                p[0] = 0;
                out[2] = clamp16(x0);
                out[3] = clamp16(x0.wrapping_add(e[1] as i32).wrapping_add(p[1] >> 7) >> 1);
                self.pe[c] = clamp16((e[0x7f] as i32).wrapping_add(p[0x7f] >> 7));
                let mut i = 1usize;
                let mut o = 4usize;
                while o < 256 {
                    let x1 = (e[i] as i32).wrapping_add(p[i] >> 7);
                    out[o] = clamp16(x1);
                    p[i] = 0;
                    let x2 = (e[i + 1] as i32).wrapping_add(p[i + 1] >> 7);
                    out[o + 1] = clamp16(x1.wrapping_add(x2) >> 1);
                    p[i + 1] = 0;
                    out[o + 2] = clamp16(x2);
                    out[o + 3] =
                        clamp16(x2.wrapping_add(e[i + 2] as i32).wrapping_add(p[i + 2] >> 7) >> 1);
                    i += 2;
                    o += 4;
                }
                p[0x7f] = 0;
            }
        }
    }

    /// `SNDIOP_ee36_iop36` (0x354bc). 3:4 linear interpolation of the
    /// combined signal `x[i] = e[i] + (p[i] >> 7)` with `x[-1] = pe[c]`
    /// (stored clamped), for j in 0..64:
    /// ```text
    /// out[4j  ] = clamp(x[3j-1])            (j = 0: pe itself)
    /// out[4j+1] = clamp((x[3j-1] + 3*x[3j]) >> 2)
    /// out[4j+2] = clamp((x[3j] + x[3j+1]) >> 1)
    /// out[4j+3] = clamp((3*x[3j+1] + x[3j+2]) >> 2)
    /// ```
    /// then `pe[c] = clamp(x[191])`. `ee_pcm`: 2 x 384; buses: 384 ints.
    pub fn ee36_iop36(&mut self, ee_pcm: &[i16], iop_bus: &mut [Vec<i32>], half: &mut [i16; 1024]) {
        for s in 0..2 {
            for c in 0..2 {
                let e = &ee_pcm[EE36[s][c]..EE36[s][c] + 192];
                let p = &mut iop_bus[IOP_BUS[c]][s * 192..s * 192 + 192];
                let out = &mut half[(s * 2 + c) * 256..(s * 2 + c) * 256 + 256];
                let pe = self.pe[c] as i32;
                out[0] = self.pe[c];
                let x0 = (e[0] as i32).wrapping_add(p[0] >> 7);
                out[1] = clamp16(pe.wrapping_add(x0.wrapping_mul(3)) >> 2);
                p[0] = 0;
                let x1 = (e[1] as i32).wrapping_add(p[1] >> 7);
                out[2] = clamp16(x0.wrapping_add(x1) >> 1);
                p[1] = 0;
                out[3] = clamp16(
                    x1.wrapping_mul(3).wrapping_add(e[2] as i32).wrapping_add(p[2] >> 7) >> 2,
                );
                self.pe[c] = clamp16((e[0xbf] as i32).wrapping_add(p[0xbf] >> 7));
                let mut i = 2usize;
                let mut o = 4usize;
                while o < 256 {
                    let a = (e[i] as i32).wrapping_add(p[i] >> 7);
                    out[o] = clamp16(a);
                    p[i] = 0;
                    let b = (e[i + 1] as i32).wrapping_add(p[i + 1] >> 7);
                    out[o + 1] = clamp16(a.wrapping_add(b.wrapping_mul(3)) >> 2);
                    p[i + 1] = 0;
                    let cc = (e[i + 2] as i32).wrapping_add(p[i + 2] >> 7);
                    out[o + 2] = clamp16(b.wrapping_add(cc) >> 1);
                    p[i + 2] = 0;
                    out[o + 3] = clamp16(
                        cc.wrapping_mul(3).wrapping_add(e[i + 3] as i32).wrapping_add(p[i + 3] >> 7)
                            >> 2,
                    );
                    i += 3;
                    o += 4;
                }
                p[0xbf] = 0;
            }
        }
    }
}

/// `SNDIOP_ee48_iop48` (0x35824). No interpolation and no state:
/// `out[i] = clamp(e[i] + (p[i] >> 7))`. `ee_pcm`: 2 x 512; buses: 512 ints.
pub fn ee48_iop48(ee_pcm: &[i16], iop_bus: &mut [Vec<i32>], half: &mut [i16; 1024]) {
    for s in 0..2 {
        for c in 0..2 {
            let e = &ee_pcm[EE48[s][c]..EE48[s][c] + 256];
            let p = &mut iop_bus[IOP_BUS[c]][s * 256..s * 256 + 256];
            let out = &mut half[(s * 2 + c) * 256..(s * 2 + c) * 256 + 256];
            for i in 0..256 {
                out[i] = clamp16((e[i] as i32).wrapping_add(p[i] >> 7));
                p[i] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buses(n: usize) -> [Vec<i32>; 2] {
        [vec![0; n], vec![0; n]]
    }

    /// Straightforward re-statement of spec §3.3, as an oracle.
    fn oracle_ee36_iop24(
        pe: &mut [i16; 2],
        pp: &mut [i32; 2],
        ee: &[i16],
        bus: &[Vec<i32>; 2],
    ) -> [i16; 1024] {
        let mut half = [0i16; 1024];
        for s in 0..2 {
            for c in 0..2 {
                let eo = EE36[s][c];
                let pb = &bus[IOP_BUS[c]][s * 128..s * 128 + 128];
                let ev = |i: isize| -> i32 { if i < 0 { pe[c] as i32 } else { ee[eo + i as usize] as i32 } };
                let pv = |i: isize| -> i32 { if i < 0 { pp[c] } else { pb[i as usize] } };
                for j in 0..64isize {
                    let k = (s * 2 + c) * 256 + 4 * j as usize;
                    half[k] = clamp16(ev(3 * j - 1) + (pv(2 * j - 1) >> 7));
                    half[k + 1] = clamp16(((ev(3 * j - 1) + 3 * ev(3 * j)) >> 2) + ((pv(2 * j - 1) + pv(2 * j)) >> 8));
                    half[k + 2] = clamp16(((ev(3 * j) + ev(3 * j + 1)) >> 1) + (pv(2 * j) >> 7));
                    half[k + 3] = clamp16(((3 * ev(3 * j + 1) + ev(3 * j + 2)) >> 2) + ((pv(2 * j) + pv(2 * j + 1)) >> 8));
                }
                pe[c] = ee[eo + 191];
                pp[c] = pb[127];
            }
        }
        half
    }

    #[test]
    fn ee36_iop24_ramp_first_values() {
        // EE ramp: R plane value i, L plane value 1000 + i; IOP buses silent.
        let mut ee = vec![0i16; 768];
        for i in 0..384 {
            ee[i] = i as i16;
            ee[384 + i] = 1000 + i as i16;
        }
        let mut r = Resampler::new();
        let mut b = buses(256);
        let mut half = [0i16; 1024];
        assert!(r.mix(36000, 24000, &ee, &mut b, &mut half));
        // L block of sub-block 0: e = 1000, 1001, ...; e[-1] = 0.
        // out0 = 0, out1 = (0 + 3*1000)>>2 = 750, out2 = (1000+1001)>>1 = 1000,
        // out3 = (3*1001 + 1002)>>2 = 1001, out4 = e[2] = 1002.
        assert_eq!(&half[0..5], &[0, 750, 1000, 1001, 1002]);
        // R block of sub-block 0: e = 0, 1, 2...
        assert_eq!(&half[256..261], &[0, 0, 0, 1, 2]);
        // L block of sub-block 1 starts with pe = e[191] of the s0 L block (1191),
        // its first EE sample is 1000 + 192.
        assert_eq!(half[512], 1191);
        assert_eq!(half[513], ((1191 + 3 * 1192) >> 2) as i16);
        // State after: pe = last EE sample of the s1 blocks.
        assert_eq!(r.pe, [1383, 383]);
    }

    #[test]
    fn ee36_iop24_matches_oracle_and_clears_bus() {
        let mut seed = 0x1234_5678u32;
        let mut rnd = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed
        };
        let mut r = Resampler::new();
        let (mut pe, mut pp) = ([0i16; 2], [0i32; 2]);
        for _ in 0..3 {
            let ee: Vec<i16> = (0..768).map(|_| rnd() as i16).collect();
            let mut b = buses(256);
            for ch in 0..2 {
                for v in b[ch].iter_mut() {
                    *v = (rnd() as i32) >> 6; // large, so clamps happen
                }
            }
            let want = oracle_ee36_iop24(&mut pe, &mut pp, &ee, &b);
            let mut half = [0i16; 1024];
            r.ee36_iop24(&ee, &mut b, &mut half);
            assert_eq!(half.to_vec(), want.to_vec());
            assert_eq!((r.pe, r.pp), (pe, pp));
            assert!(b[0].iter().chain(b[1].iter()).all(|&x| x == 0));
        }
    }

    #[test]
    fn ee36_iop24_iop_bus_and_clamp() {
        let ee = vec![0i16; 768];
        let mut b = buses(256);
        // L bus (1): p[0] = 128*100, p[1] = 128*200 -> out2 = 100, out1 = (0+12800)>>8 = 50,
        // out3 = (12800 + 25600) >> 8 = 150.
        b[1][0] = 12800;
        b[1][1] = 25600;
        // R bus (0): huge positive and negative values clamp.
        b[0][0] = 0x7fff_ff00;
        b[0][1] = -0x7fff_ff00;
        let mut r = Resampler::new();
        let mut half = [0i16; 1024];
        r.ee36_iop24(&ee, &mut b, &mut half);
        assert_eq!(&half[0..4], &[0, 50, 100, 150]);
        assert_eq!(half[256 + 2], 0x7fff);
        assert_eq!(half[256 + 3], 0); // (p0 + p1) >> 8 = 0
        assert_eq!(half[256 + 4], -0x8000); // p1 >> 7
        // The history keeps the raw last IOP sample.
        let mut b2 = buses(256);
        b2[1][127] = 1 << 20;
        b2[1][255] = 77 << 7;
        let mut r2 = Resampler::new();
        r2.ee36_iop24(&ee, &mut b2, &mut half);
        // After the call pp holds sub-block 1's last raw IOP sample.
        assert_eq!(r2.pp[0], 77 << 7);
        // ...and the next sub-block (s1, L) starts from it: out0 = pp >> 7.
        assert_eq!(half[512], (1 << 13) as i16);
    }

    #[test]
    fn ee24_iop24_interpolates_and_keeps_clamped_history() {
        let mut ee = vec![0i16; 512];
        for i in 0..256 {
            ee[i] = 10 * i as i16; // R plane
            ee[256 + i] = 32000; // L plane
        }
        let mut b = buses(256);
        b[1][0] = 128 * 1000; // L: x0 = 33000 -> clamps
        let mut r = Resampler::new();
        let mut half = [0i16; 1024];
        assert!(r.mix(24000, 24000, &ee, &mut b, &mut half));
        // L s0: out0 = pe 0, out1 = (0 + 33000)>>1 = 16500, out2 = clamp(33000),
        // out3 = (33000 + 32000) >> 1 = 32500.
        assert_eq!(&half[0..4], &[0, 16500, 0x7fff, 32500]);
        // R s0: x = 0, 10, 20...: 0, 0, 0, 5, 10, 15, 20
        assert_eq!(&half[256..263], &[0, 0, 0, 5, 10, 15, 20]);
        // s1 R starts from pe = x[127] of s0 R = 1270.
        assert_eq!(half[768], 1270);
        assert_eq!(half[769], (1270 + 1280) / 2);
        assert!(b[1].iter().all(|&x| x == 0));
    }

    #[test]
    fn ee36_iop36_pattern() {
        let mut ee = vec![0i16; 768];
        for i in 0..384 {
            ee[i] = 4 * i as i16;
        }
        let mut b = buses(384);
        let mut r = Resampler::new();
        let mut half = [0i16; 1024];
        assert!(r.mix(36000, 36000, &ee, &mut b, &mut half));
        // R s0: x = 0,4,8,12,...; out = 0, (0+0)>>2=0, (0+4)>>1=2, (12+8)>>2=5, 8, (8+36)>>2=11
        assert_eq!(&half[256..262], &[0, 0, 2, 5, 8, 11]);
        assert_eq!(r.pe[1], 4 * 383);
    }

    #[test]
    fn ee48_iop48_adds_and_clears() {
        let mut ee = vec![0i16; 1024];
        ee[512] = 5; // L s0 first sample
        ee[0] = -7; // R s0
        ee[768 + 255] = 100; // L s1 last
        let mut b = buses(512);
        b[1][0] = 128 * 3;
        b[0][256] = 128 * 9; // R s1 first
        let mut r = Resampler::new();
        let mut half = [0i16; 1024];
        assert!(r.mix(48000, 48000, &ee, &mut b, &mut half));
        assert_eq!(half[0], 8);
        assert_eq!(half[256], -7);
        assert_eq!(half[767], 100);
        assert_eq!(half[768], 9);
        assert!(b[0].iter().chain(b[1].iter()).all(|&x| x == 0));
    }

    #[test]
    fn unknown_rate_pair_is_ignored() {
        let mut r = Resampler::new();
        let mut b = buses(256);
        b[0][0] = 5;
        let mut half = [7i16; 1024];
        assert!(!r.mix(44100, 24000, &[0; 1024], &mut b, &mut half));
        assert_eq!(half[0], 7);
        assert_eq!(b[0][0], 5);
    }
}
