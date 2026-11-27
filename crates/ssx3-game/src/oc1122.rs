//! Staging module oc1122 (wave wave270): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1122`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_7630, libmpeg_picture_header, "libmpeg_PictureHeader"),
    (
        0x0040_7820,
        libmpeg_picture_coding_extension,
        "libmpeg_PictureCodingExtension",
    ),
    (
        0x0040_7a10,
        libmpeg_extra_bit_information,
        "libmpeg_ExtraBitInformation",
    ),
    (
        0x0040_7a58,
        libmpeg_update_temporal_ref,
        "libmpeg_UpdateTemporalRef",
    ),
    (
        0x0040_7ad0,
        libmpeg_group_of_pictures_header,
        "libmpeg_GroupOfPicturesHeader",
    ),
    (
        0x0040_7b70,
        libmpeg_quant_matrix_extension,
        "libmpeg_QuantMatrixExtension",
    ),
    (
        0x0040_7c38,
        libmpeg_picture_display_extension,
        "libmpeg_PictureDisplayExtension",
    ),
    (
        0x0040_7d30,
        libmpeg_copyright_extension,
        "libmpeg_CopyrightExtension",
    ),
];

/// The MPEG decoder's direct callees (their generated aliases dispatch
/// through the override table, as the generated call sites do).
const OC1122_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::libmpeg_GetBits,
};
const OC1122_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::libmpeg_FlushBits,
};
const OC1122_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};
const OC1122_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::libmpeg_IpuCommand,
};
const OC1122_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};
const OC1122_EXTRA_BIT_INFO: Callee = Callee {
    addr: 0x0040_7a10,
    f: ee::s15::libmpeg_ExtraBitInformation,
};
const OC1122_EXT_USER_DATA: Callee = Callee {
    addr: 0x0040_76f8,
    f: ee::s15::libmpeg_ExtensionAndUserData,
};

/// `GetBits(sys, n)` returning to `ret_to`: the call sites set `a0 = sys`
/// and `a1 = n`.
fn oc1122_get_bits(c: &mut Cpu, sys: u64, n: u64, ret_to: u32) -> Flow {
    c.g[A0][0] = sys;
    c.g[A1][0] = n;
    OC1122_GET_BITS.call(c, ret_to)
}

/// `IpuWaitBusy(sys)` returning to `ret_to`.
fn oc1122_ipu_wait_busy(c: &mut Cpu, sys: u64, ret_to: u32) -> Flow {
    c.g[A0][0] = sys;
    OC1122_IPU_WAIT_BUSY.call(c, ret_to)
}

/// `IpuCommand(sys, cmd)` returning to `ret_to`.
fn oc1122_ipu_command(c: &mut Cpu, sys: u64, cmd: u64, ret_to: u32) -> Flow {
    c.g[A0][0] = sys;
    c.g[A1][0] = cmd;
    OC1122_IPU_COMMAND.call(c, ret_to)
}

/// `libmpeg_PictureHeader(sys)` (0x407630): reads the 10-bit temporal
/// reference (kept in `s1`), the 3-bit picture coding type (`sys+0x150`),
/// the 16-bit vbv delay, and for coding types 2 and 3 the forward (and,
/// for type 3, backward) fields into `+0x154`..`+0x160`. Then it runs
/// `ExtraBitInformation`, `ExtensionAndUserData`, and tail-calls
/// `UpdateTemporalRef(sys, temporalRef)`.
pub fn libmpeg_picture_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0xa;
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);
        let slot = frame.at(c, 0x10);
        c.sd(slot, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];

        // temporal reference = GetBits(sys, 10)
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 0xa, 0x0040_764c)?;
        c.g[S1][0] = c.g[V0][0];
        // picture coding type
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 3, 0x0040_765c)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x150)).sw(c, v0);
        // vbv delay
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 0x10, 0x0040_766c)?;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x150)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S0][0];
        if !t {
            c.cycles += 8;
            let s0 = c.g[S0][0];
            oc1122_get_bits(c, s0, 1, 0x0040_7688)?;
            let v0 = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x154)).sw(c, v0);
            let s0 = c.g[S0][0];
            oc1122_get_bits(c, s0, 3, 0x0040_7698)?;
            let v0 = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x158)).sw(c, v0);
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x150)).lw(c);
        }

        // Backward fields, for picture coding type 3 only.
        c.cycles += 3;
        c.g[V0][0] = 3;
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if !t {
            c.cycles += 7;
            let s0 = c.g[S0][0];
            oc1122_get_bits(c, s0, 1, 0x0040_76b4)?;
            let v0 = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x15c)).sw(c, v0);
            let s0 = c.g[S0][0];
            oc1122_get_bits(c, s0, 3, 0x0040_76c4)?;
            let v0 = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x160)).sw(c, v0);
        }

        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        OC1122_EXTRA_BIT_INFO.call(c, 0x0040_76d0)?;
        c.g[A0][0] = c.g[S0][0];
        OC1122_EXT_USER_DATA.call(c, 0x0040_76d8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        frame.restore_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = 0x0040_7a58;
        ee::s15::libmpeg_UpdateTemporalRef(c);
        Ok(())
    })
}

/// `libmpeg_PictureCodingExtension(sys)` (0x407820): the four `f_code`
/// nibbles into `+0x164`..`+0x170`, the picture structure and the
/// top/bottom and frame-prediction flags into `+0x174`, `+0x178`..`+0x188`,
/// the intra and non-intra quantiser matrix flags, and the composite-
/// display words written to the IPU control register (`0x10002010`).
pub fn libmpeg_picture_coding_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 4;
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];

        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 4, 0x0040_7838)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x164)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 4, 0x0040_7848)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x168)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 4, 0x0040_7858)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x16c)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 4, 0x0040_7868)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x170)).sw(c, v0);

        // Composite display flag into the IPU control word.
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 2, 0x0040_7878)?;
        c.g[A3][0] = 0x1000_0000;
        c.g[A2][0] = 0xffff_ffff_fffc_0000;
        c.g[A3][0] |= 0x2010;
        c.g[A2][0] |= 0xffff;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        c.g[V1][0] &= c.g[A2][0];
        c.g[V1][0] |= c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(c.g[A3][0] as u32).sw(c, v1);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 2, 0x0040_78a8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xd4)).lw(c);
        let t = c.g[V0][0] != 0u64;
        let v1 = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x174)).sw(c, v1);
        if !t {
            c.cycles += 1;
            Addr((c.g[S0][0] as u32).wrapping_add(0xd4)).sw(c, v1);
        }

        // Flags: one bit each, the IPU control word takes the top bits.
        c.cycles += 64;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_78c8)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x178)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_78d8)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x17c)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_78e8)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x180)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_78f8)?;

        c.g[A2][0] = 0x1000_0000;
        c.g[A2][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x2010)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffbf_0000;
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 22);
        c.g[A2][0] &= c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] |= c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[AT][0] = 0x1000_0000;
        let a2 = c.g[A2][0];
        Addr((c.g[AT][0] as u32).wrapping_add(0x2010)).sw(c, a2);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7928)?;

        c.g[A2][0] = 0x1000_0000;
        c.g[A2][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x2010)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffdf_0000;
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 21);
        c.g[A2][0] &= c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] |= c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[AT][0] = 0x1000_0000;
        let a2 = c.g[A2][0];
        Addr((c.g[AT][0] as u32).wrapping_add(0x2010)).sw(c, a2);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7958)?;

        c.g[A2][0] = 0x1000_0000;
        c.g[A2][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x2010)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffef_0000;
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
        c.g[A2][0] &= c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] |= c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[AT][0] = 0x1000_0000;
        let a2 = c.g[A2][0];
        Addr((c.g[AT][0] as u32).wrapping_add(0x2010)).sw(c, a2);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7988)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x184)).sw(c, v0);

        c.g[A0][0] = c.g[S0][0];
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7998)?;
        c.g[A0][0] = c.g[S0][0];
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_79a4)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x188)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_79b4)?;

        let t = c.g[V0][0] == 0u64;
        c.g[RA][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x10)).ld(c);
        if t {
            c.cycles += 3;
            let a = frame.at(c, 0);
            c.g[S0][0] = c.ld(a);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = ra;
            return Ok(());
        }
        c.cycles += 18;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_79c8)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 3, 0x0040_79d4)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_79e0)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 7, 0x0040_79ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x10)).ld(c);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        c.g[A1][0] = 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = 0x0040_72e8;
        ee::s15::libmpeg_GetBits(c);
        Ok(())
    })
}

/// `libmpeg_ExtraBitInformation(sys)` (0x407a10): skips the extra
/// information bits. While a one-bit flag reads as set, it flushes one
/// byte (`FlushBits(sys, 8)`) and reads the next flag. Delivers a due event
/// at the loop's back edge.
pub fn libmpeg_extra_bit_information(c: &mut Cpu) {
    if c.pc != 0x0040_7a10 {
        // A re-entry at a loop head after an unwound event: the generated
        // dispatcher carries its own resume table.
        return ee::s15::f_00407a10__generated(c);
    }
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        loop {
            c.cycles += 6;
            let s0 = c.g[S0][0];
            oc1122_get_bits(c, s0, 1, 0x0040_7a3c)?;
            c.g[A0][0] = c.g[S0][0];
            let more = c.g[V0][0] != 0u64;
            c.g[A1][0] = 8;
            if !more {
                break;
            }
            if back_edge_unwinds(c, 0x0040_7a28) {
                return Ok(());
            }
            c.cycles += 2;
            OC1122_FLUSH_BITS.call(c, 0x0040_7a30)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `libmpeg_UpdateTemporalRef(sys, temporalRef)` (0x407a58): no calls.
/// Keeps the picture's absolute temporal reference (`sys+0x1ac`: the
/// reference plus the group base `sys+0x84c`, plus 0x400 after a wrap, the
/// wrap flag in `sys+0x854`), and the running maximum `sys+0x850`, which it
/// returns in v0.
pub fn libmpeg_update_temporal_ref(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A2][0] = c.g[A0][0];
    c.g[A3][0] = 0;
    let sys = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(sys.wrapping_add(0x150)).lw(c);
    c.g[V0][0] = 3;
    let is_three = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = 0;
    // `exit_to_v0`: the paths that leave through the shared `+1` exit below.
    let mut exit_to_v0 = is_three;
    if !is_three {
        c.cycles += 2;
        if c.g[A1][0] == 0u64 {
            c.g[V0][0] = Addr(sys.wrapping_add(0x84c)).lw(c);
        } else {
            c.cycles += 2;
            if (c.g[A1][0] as i64) >= 0 {
                Addr(sys.wrapping_add(0x854)).sw(c, 0);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(sys.wrapping_add(0x854)).lw(c);
                c.g[A3][0] = (c.g[V0][0] < 0x1u64) as u64;
                Addr(sys.wrapping_add(0x854)).sw(c, 0);
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[A1][0];
            exit_to_v0 = true;
        }
    }
    if exit_to_v0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(sys.wrapping_add(0x84c)).lw(c);
    }

    c.cycles += 3;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let v1 = c.g[V1][0];
    let a3_zero = c.g[A3][0] == 0;
    Addr(sys.wrapping_add(0x1ac)).sw(c, v1);
    let mut returned_early = false;
    if !a3_zero {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(sys.wrapping_add(0x850)).lw(c);
            returned_early = true;
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx((v1 as u32).wrapping_add(0x400));
            let v0 = c.g[V0][0];
            Addr(sys.wrapping_add(0x1ac)).sw(c, v0);
        }
    }
    if !returned_early {
        c.cycles += 1;
        c.g[V0][0] = Addr(sys.wrapping_add(0x850)).lw(c);
    }

    c.cycles += 5;
    c.g[A0][0] = Addr(sys.wrapping_add(0x1ac)).lw(c);
    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    let v0 = c.g[V0][0];
    Addr(sys.wrapping_add(0x850)).sw(c, v0);
    ret(c);
}

/// `libmpeg_GroupOfPicturesHeader(sys)` (0x407ad0): clears the picture
/// count word, bumps `sys+0x84c` (the group base), then reads the group's
/// time code and closed/broken-link flags (`+0x1a4`, `+0x1a8`) and runs
/// `ExtensionAndUserData`.
pub fn libmpeg_group_of_pictures_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xe8)).sw(c, 0);
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x850)).lw(c);
        let v1 = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x854)).sw(c, v1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x84c)).sw(c, v0);

        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7b00)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 5, 0x0040_7b0c)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 6, 0x0040_7b18)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7b24)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 6, 0x0040_7b30)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 6, 0x0040_7b3c)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7b48)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x1a4)).sw(c, v0);
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7b58)?;
        let v0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x1a8)).sw(c, v0);

        c.g[A0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = 0x0040_76f8;
        ee::s15::libmpeg_ExtensionAndUserData(c);
        Ok(())
    })
}

/// `libmpeg_QuantMatrixExtension(sys)` (0x407b70): the load-intra and
/// load-non-intra quantiser matrix flags (`+0x840`, `+0x844`); each set
/// flag sends a matrix to the IPU through `IpuWaitBusy` and `IpuCommand`.
/// A set flag in the third position is a reserved extension and reports
/// through `libmpeg_Error`.
pub fn libmpeg_quant_matrix_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];

        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7b88)?;
        let v0 = c.g[V0][0];
        let t = v0 == 0u64;
        Addr((c.g[S0][0] as u32).wrapping_add(0x840)).sw(c, v0);
        if !t {
            c.cycles += 7;
            let s0 = c.g[S0][0];
            oc1122_ipu_wait_busy(c, s0, 0x0040_7b98)?;
            let s0 = c.g[S0][0];
            oc1122_ipu_command(c, s0, 0x5000_0000, 0x0040_7ba4)?;
            let s0 = c.g[S0][0];
            oc1122_ipu_wait_busy(c, s0, 0x0040_7bac)?;
        }

        c.cycles += 5;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7bb8)?;
        let v0 = c.g[V0][0];
        let t = v0 == 0u64;
        Addr((c.g[S0][0] as u32).wrapping_add(0x844)).sw(c, v0);
        if !t {
            c.cycles += 7;
            let s0 = c.g[S0][0];
            oc1122_ipu_wait_busy(c, s0, 0x0040_7bc8)?;
            let s0 = c.g[S0][0];
            oc1122_ipu_command(c, s0, 0x5800_0000, 0x0040_7bd4)?;
            let s0 = c.g[S0][0];
            oc1122_ipu_wait_busy(c, s0, 0x0040_7bdc)?;
        }

        c.cycles += 5;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7be8)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = 0x4a_0000;
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7ff0));
            OC1122_ERROR.call(c, 0x0040_7bfc)?;
        }

        c.cycles += 5;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7c08)?;
        let t = c.g[V0][0] == 0u64;
        frame.restore_ra(c, 0x10);
        if t {
            c.cycles += 3;
            let a = frame.at(c, 0);
            c.g[S0][0] = c.ld(a);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4a_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7fc8));
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = 0x0040_34a8;
        ee::s15::libmpeg_Error(c);
        Ok(())
    })
}

/// `libmpeg_PictureDisplayExtension(sys)` (0x407c38): the picture
/// structure's display-extension words. It works out the number of
/// display offsets (`s3`: 1 to 3) from the extension flags, then reads, per
/// offset, a 16-bit horizontal and a 16-bit vertical value into the offset
/// tables at `sys+0x18c` and `sys+0x198`.
pub fn libmpeg_picture_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let slot = frame.at(c, 0x10);
        c.sd(slot, c.g[S1][0]);
        frame.save_ra(c, 0x60);
        c.g[S1][0] = c.g[A0][0];
        let slot = frame.at(c, 0x50);
        c.sd(slot, c.g[S5][0]);
        let slot = frame.at(c, 0x40);
        c.sd(slot, c.g[S4][0]);
        let slot = frame.at(c, 0x30);
        c.sd(slot, c.g[S3][0]);
        let slot = frame.at(c, 0x20);
        c.sd(slot, c.g[S2][0]);
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);

        let sys = c.g[S1][0] as u32;
        c.g[V0][0] = Addr(sys.wrapping_add(0x13c)).lw(c);
        // `to_one`: the paths that leave through the shared `S3 = 1` exit.
        let mut to_one = false;
        if c.g[V0][0] == 0u64 {
            c.g[V1][0] = Addr(sys.wrapping_add(0x174)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = 3;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = Addr(sys.wrapping_add(0x184)).lw(c);
                c.cycles += 2;
                c.g[S3][0] = 2;
                if c.g[V0][0] != 0 {
                    c.g[S3][0] = c.g[V1][0];
                }
            } else {
                to_one = true;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(sys.wrapping_add(0x184)).lw(c);
            let t = c.g[V0][0] == 0u64;
            c.g[S3][0] = 2;
            if t {
                to_one = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(sys.wrapping_add(0x178)).lw(c);
                c.g[V0][0] = 3;
                if c.g[V1][0] != 0 {
                    c.g[S3][0] = c.g[V0][0];
                }
            }
        }
        if to_one {
            c.cycles += 2;
            c.g[S3][0] = 1;
        }

        c.cycles += 2;
        let t = (c.g[S3][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !t {
            c.cycles += 4;
            c.g[S5][0] = sx(sys.wrapping_add(0x18c));
            c.g[S4][0] = sx(sys.wrapping_add(0x198));
            c.g[A0][0] = c.g[S1][0];
            loop {
                c.cycles += 20;
                let s1 = c.g[S1][0];
                oc1122_get_bits(c, s1, 0x10, 0x0040_7cc0)?;
                c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[A0][0] = c.g[S1][0];
                let (addr, v0) = (c.g[V1][0] as u32, c.g[V0][0]);
                Addr(addr).sw(c, v0);
                oc1122_get_bits(c, s1, 1, 0x0040_7cd8)?;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.g[S1][0];
                oc1122_get_bits(c, s1, 0x10, 0x0040_7ce8)?;
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[A0][0] = c.g[S1][0];
                let (addr, v0) = (c.g[S0][0] as u32, c.g[V0][0]);
                Addr(addr).sw(c, v0);
                oc1122_get_bits(c, s1, 1, 0x0040_7cfc)?;
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let more = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_7cb8);
                }
            }
        }

        c.cycles += 9;
        frame.restore_ra(c, 0x60);
        let a = frame.at(c, 0x50);
        c.g[S5][0] = c.ld(a);
        let a = frame.at(c, 0x40);
        c.g[S4][0] = c.ld(a);
        let a = frame.at(c, 0x30);
        c.g[S3][0] = c.ld(a);
        let a = frame.at(c, 0x20);
        c.g[S2][0] = c.ld(a);
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `libmpeg_CopyrightExtension(sys)` (0x407d30): reads the copyright
/// extension's fields (the copyright flag, the 8-bit manufacturer code, the
/// 7-bit reserved field, and the 0x14- and 0x16-bit copyright numbers),
/// then tail-calls `GetBits(sys, 22)` for the last one.
pub fn libmpeg_copyright_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        let slot = frame.at(c, 0);
        c.sd(slot, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];

        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7d48)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 8, 0x0040_7d54)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7d60)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 7, 0x0040_7d6c)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7d78)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 0x14, 0x0040_7d84)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7d90)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 0x16, 0x0040_7d9c)?;
        let s0 = c.g[S0][0];
        oc1122_get_bits(c, s0, 1, 0x0040_7da8)?;

        c.g[A0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        c.g[A1][0] = 0x16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = 0x0040_72e8;
        ee::s15::libmpeg_GetBits(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the MPEG callee fakes: they answer only while this word of the
    /// fixture's RAM holds `OC1122_MAGIC` (otherwise the generated body runs).
    const OC1122_CTRL: u32 = DATA + 0x0002_0000;
    const OC1122_MAGIC: u32 = 0x1122_0407;
    /// The 64 `GetBits` results of a fixture, one word per fake call; zero
    /// past the end, so every bit loop terminates.
    const OC1122_BITS: u32 = DATA + 0x0002_0100;

    static OC1122_PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);
    static OC1122_PREV_FLUSH_BITS: AtomicUsize = AtomicUsize::new(0);
    static OC1122_PREV_IPU_WAIT: AtomicUsize = AtomicUsize::new(0);
    static OC1122_PREV_IPU_COMMAND: AtomicUsize = AtomicUsize::new(0);
    static OC1122_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);
    static OC1122_PREV_EXT_USER_DATA: AtomicUsize = AtomicUsize::new(0);

    fn oc1122_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1122_CTRL) == OC1122_MAGIC
    }

    /// `libmpeg_GetBits`: logs the call and returns the next seeded result.
    fn oc1122_fake_get_bits(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_GET_BITS, ee::s15::f_004072e8__generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let bits = if n < 64 {
            c.bus.read32(OC1122_BITS + 4 * n)
        } else {
            0
        };
        fakes::fake_call(c, 0x0040_72e8);
        c.g[V0][0] = bits as u64;
    }

    fn oc1122_fake_flush_bits(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_FLUSH_BITS, ee::s15::f_004071f8__generated);
        }
        fakes::fake_call(c, 0x0040_71f8);
    }

    fn oc1122_fake_ipu_wait(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_IPU_WAIT, ee::s15::f_00406e20__generated);
        }
        fakes::fake_call(c, 0x0040_6e20);
    }

    fn oc1122_fake_ipu_command(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_IPU_COMMAND, ee::s15::f_00406df0__generated);
        }
        fakes::fake_call(c, 0x0040_6df0);
    }

    fn oc1122_fake_error(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_ERROR, ee::s15::f_004034a8__generated);
        }
        fakes::fake_call(c, 0x0040_34a8);
    }

    fn oc1122_fake_ext_user_data(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(
                c,
                &OC1122_PREV_EXT_USER_DATA,
                ee::s15::f_004076f8__generated,
            );
        }
        fakes::fake_call(c, 0x0040_76f8);
    }

    fn oc1122_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1122_fake_get_bits,
            &OC1122_PREV_GET_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            oc1122_fake_flush_bits,
            &OC1122_PREV_FLUSH_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406E20),
            oc1122_fake_ipu_wait,
            &OC1122_PREV_IPU_WAIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406DF0),
            oc1122_fake_ipu_command,
            &OC1122_PREV_IPU_COMMAND,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1122_fake_error,
            &OC1122_PREV_ERROR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004076F8),
            oc1122_fake_ext_user_data,
            &OC1122_PREV_EXT_USER_DATA,
        );
    }

    /// MPEG fixture: the callee fakes are armed, the bit results are seeded
    /// (values 0 to 7), the sys object at DATA holds random words, and `c.pc`
    /// is the entry.
    fn oc1122_mpeg_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        oc1122_install_fakes();
        let mut r = fakes::machine(c, seed, 0x1122_0000 ^ entry as u64, 0);
        c.bus.write32(OC1122_CTRL, OC1122_MAGIC);
        for k in 0..64 {
            c.bus.write32(OC1122_BITS + 4 * k, r.next_u32() % 8);
        }
        fill(c, &mut r, DATA, 0x400);
        c.g[A0][0] = sx(DATA);
        c.pc = entry;
        r
    }

    #[test]
    fn picture_header_matches() {
        check(
            ee::s15::f_00407630__generated,
            libmpeg_picture_header,
            200,
            &|c, seed| {
                oc1122_mpeg_setup(c, seed, 0x0040_7630);
            },
        );
    }

    #[test]
    fn picture_coding_extension_matches() {
        check(
            ee::s15::f_00407820__generated,
            libmpeg_picture_coding_extension,
            200,
            &|c, seed| {
                oc1122_mpeg_setup(c, seed, 0x0040_7820);
            },
        );
    }

    #[test]
    fn extra_bit_information_matches() {
        check(
            ee::s15::f_00407a10__generated,
            libmpeg_extra_bit_information,
            200,
            &|c, seed| {
                oc1122_mpeg_setup(c, seed, 0x0040_7a10);
            },
        );
    }

    #[test]
    fn update_temporal_ref_matches() {
        check(
            ee::s15::f_00407a58__generated,
            libmpeg_update_temporal_ref,
            200,
            &|c, seed| {
                let mut r = oc1122_mpeg_setup(c, seed, 0x0040_7a58);
                // Coding type 3 for a quarter of the seeds, and every
                // temporal-reference sign.
                c.bus.write32(DATA + 0x150, r.next_u32() % 5);
                c.bus.write32(
                    DATA + 0x854,
                    if seed % 3 == 0 { 0 } else { r.next_u32() % 9 },
                );
                c.bus.write32(DATA + 0x850, r.next_u32() % 4000);
                c.bus.write32(DATA + 0x84c, r.next_u32() % 4000);
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    _ => sx(r.next_u32() % 4000),
                };
            },
        );
    }

    #[test]
    fn group_of_pictures_header_matches() {
        check(
            ee::s15::f_00407ad0__generated,
            libmpeg_group_of_pictures_header,
            200,
            &|c, seed| {
                oc1122_mpeg_setup(c, seed, 0x0040_7ad0);
            },
        );
    }

    #[test]
    fn quant_matrix_extension_matches() {
        check(
            ee::s15::f_00407b70__generated,
            libmpeg_quant_matrix_extension,
            200,
            &|c, seed| {
                oc1122_mpeg_setup(c, seed, 0x0040_7b70);
            },
        );
    }

    #[test]
    fn picture_display_extension_matches() {
        check(
            ee::s15::f_00407c38__generated,
            libmpeg_picture_display_extension,
            200,
            &|c, seed| {
                let mut r = oc1122_mpeg_setup(c, seed, 0x0040_7c38);
                // Small flags so each branch of the display-offset count is taken.
                c.bus.write32(DATA + 0x13c, r.next_u32() % 2);
                c.bus.write32(DATA + 0x174, r.next_u32() % 4);
                c.bus.write32(DATA + 0x178, r.next_u32() % 2);
                c.bus.write32(DATA + 0x184, r.next_u32() % 2);
            },
        );
    }

    #[test]
    fn copyright_extension_matches() {
        check(
            ee::s15::f_00407d30__generated,
            libmpeg_copyright_extension,
            200,
            &|c, seed| {
                oc1122_mpeg_setup(c, seed, 0x0040_7d30);
            },
        );
    }
}
