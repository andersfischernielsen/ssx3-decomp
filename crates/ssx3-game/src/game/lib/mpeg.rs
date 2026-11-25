//! `src/lib/mpeg.cpp`: hand-written MPEG (`sceMpeg*`) replacements.

use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0040_2a58,
        sce_mpeg_get_picture_raw8,
        "sceMpegGetPictureRAW8",
    ),
    (
        0x0040_2aa0,
        sce_mpeg_get_picture_raw8xy,
        "sceMpegGetPictureRAW8xy",
    ),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `libmpeg_GetPictureCommon` (`0x0040_2e10`).
    pub(crate) const GET_PICTURE_COMMON: Callee = Callee {
        addr: 0x0040_2e10,
        f: ee::s15::libmpeg_GetPictureCommon,
    };
}

/// `int sceMpegGetPictureRAW8(sys, a1, a2)` (0x402a58): latch the picture
/// request words into the work area (`+0xe4` the raw value, `+0xd8` flags,
/// `+0xdc`/`+0xb0`/`+0xe0` cleared), then `libmpeg_GetPictureCommon`.
pub fn sce_mpeg_get_picture_raw8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0fff_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] &= c.g[V0][0];
        c.g[V1][0] = 0x2000_0000;
        c.g[A3][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] |= c.g[V1][0];
        Addr((c.g[A3][0] as u32).wrapping_add(0xe4)).sw(c, c.g[A2][0]);
        Addr((c.g[A3][0] as u32).wrapping_add(0xd8)).sw(c, c.g[A1][0]);
        Addr((c.g[A3][0] as u32).wrapping_add(0xdc)).sw(c, 0);
        Addr((c.g[A3][0] as u32).wrapping_add(0xb0)).sw(c, 0);
        c.g[RA][0] = sx(0x0040_2a90);
        Addr((c.g[A3][0] as u32).wrapping_add(0xe0)).sw(c, 0);
        callee::GET_PICTURE_COMMON.call(c, 0x0040_2a90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int sceMpegGetPictureRAW8xy(sys, a1, x, y)` (0x402aa0): latch the
/// picture request words into the work area (`+0xd8` flags, `+0xe4` the
/// mult result, `+0xdc`/`+0xe0` x/y scaled by 16), clear `+0xb0`, then
/// `libmpeg_GetPictureCommon`.
pub fn sce_mpeg_get_picture_raw8xy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0fff_0000;
        frame.save_ra(c, 0);
        c.mult(0, 8, 6, 7);
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_shl(4));
        c.g[A1][0] &= c.g[V0][0];
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = 0x2000_0000;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_shl(4));
        c.g[A1][0] |= c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0xe0)).sw(c, c.g[A3][0]);
        Addr((c.g[V1][0] as u32).wrapping_add(0xd8)).sw(c, c.g[A1][0]);
        Addr((c.g[V1][0] as u32).wrapping_add(0xe4)).sw(c, c.g[T0][0]);
        Addr((c.g[V1][0] as u32).wrapping_add(0xdc)).sw(c, c.g[A2][0]);
        c.g[RA][0] = sx(0x0040_2ae4);
        Addr((c.g[V1][0] as u32).wrapping_add(0xb0)).sw(c, 0);
        callee::GET_PICTURE_COMMON.call(c, 0x0040_2ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::require_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee_rt::Cpu;

    /// GetPictureRAW8: same fast-`Errorf` steering as the xy form.
    #[test]
    fn sce_mpeg_get_picture_raw8_matches() {
        let mpeg_get_picture_raw8_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg_setup(c, seed, 0x0040_2a58);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] =
                ((seed % 63) + 1) as u64 | (seed as u64).wrapping_mul(0x1f3b_52d5) & 0x0fff_ffc0;
            c.g[A2][0] = seed as u64 ^ 0x8765_4321;
        };
        check(
            ee::s15::f_00402a58__generated,
            sce_mpeg_get_picture_raw8,
            200,
            mpeg_get_picture_raw8_call,
        );
        require_paths(
            ee::s15::f_00402a58__generated,
            200,
            &[253],
            mpeg_get_picture_raw8_call,
        );
    }

    /// GetPictureRAW8xy: the masked-flags leg always has low 6 bits set,
    /// so `GetPictureCommon` takes the fast `Errorf` return; the stores
    /// and the mult vary by seed. The header/decode legs need a live
    /// stream and are not covered.
    #[test]
    fn sce_mpeg_get_picture_raw8xy_matches() {
        let mpeg_get_picture_raw8xy_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mpeg_setup(c, seed, 0x0040_2aa0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] =
                ((seed % 63) + 1) as u64 | (seed as u64).wrapping_mul(0x1f3b_52d5) & 0x0fff_ffc0;
            c.g[A2][0] = seed as u64 ^ 0x1234_5678;
            c.g[A3][0] = (seed as u64).wrapping_mul(0x85eb_ca6b);
            c.g[T0][0] = (seed as u64).wrapping_mul(0xc2b2_ae35);
        };
        check_lohi(
            ee::s15::f_00402aa0__generated,
            sce_mpeg_get_picture_raw8xy,
            200,
            mpeg_get_picture_raw8xy_call,
        );
        require_paths(
            ee::s15::f_00402aa0__generated,
            200,
            &[256],
            mpeg_get_picture_raw8xy_call,
        );
    }

    /// MPEG picture-request fixture: the sys object at DATA with its work
    /// area at DATA+0x400. `+0x858` is 0 so the `libmpeg_Error` leg takes
    /// the fast `DefaultErrorPrint` path, and the printf format words are
    /// NULled so `sprintf`/`scePrintf` return at once.
    pub(crate) fn mpeg_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x0040_2a58 ^ (seed as u64) << 16 ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        c.bus.write32(DATA + 0x40, DATA + 0x400);
        c.bus.write32(DATA + 0x400 + 0x858, 0);
        c.bus.write32(0x0049_7c68, 0);
        c.bus.write32(0x0049_7ce8, 0);
    }
}
