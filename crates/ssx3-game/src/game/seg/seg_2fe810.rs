//! `src/seg/seg_2fe810.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_e878, ieee754_rem_pio2f, "__ieee754_rem_pio2f"),
    (0x0040_ec58, kernel_rem_pio2f, "__kernel_rem_pio2f"),
    (
        0x0040_2c30,
        libmpeg_dispatch_callback,
        "libmpeg_DispatchCallback",
    ),
    (
        0x0040_2c80,
        libmpeg_callback_nodata,
        "libmpeg_CallbackNodata",
    ),
    (0x0040_2d50, libmpeg_work_area_init, "libmpeg_WorkAreaInit"),
    (0x0040_2d68, libmpeg_work_area_mark, "libmpeg_WorkAreaMark"),
    (
        0x0040_2d78,
        libmpeg_work_area_restore,
        "libmpeg_WorkAreaRestore",
    ),
    (
        0x0040_2d88,
        libmpeg_work_area_alloc,
        "libmpeg_WorkAreaAlloc",
    ),
    (
        0x0040_2e10,
        libmpeg_get_picture_common,
        "libmpeg_GetPictureCommon",
    ),
    (
        0x0040_2f80,
        libmpeg_decode_frame_pic,
        "libmpeg_DecodeFramePic",
    ),
    (0x0040_3098, libmpeg_decode_picture, "libmpeg_DecodePicture"),
    (
        0x0040_30e0,
        libmpeg_decode_field_pic,
        "libmpeg_DecodeFieldPic",
    ),
    (
        0x0040_32a8,
        libmpeg_reset_mpeg1_flag,
        "libmpeg_ResetMpeg1Flag",
    ),
    (
        0x0040_32b8,
        libmpeg_output_last_picture,
        "libmpeg_OutputLastPicture",
    ),
    (
        0x0040_3338,
        libmpeg_init_spr_buffers,
        "libmpeg_InitSprBuffers",
    ),
    (0x0040_3398, libmpeg_reset_ipu, "libmpeg_ResetIpu"),
    (
        0x0040_3460,
        libmpeg_default_error_print,
        "libmpeg_DefaultErrorPrint",
    ),
    (0x0040_3470, libmpeg_errorf, "libmpeg_Errorf"),
    (0x0040_34a8, libmpeg_error, "libmpeg_Error"),
    (
        0x0040_3500,
        libmpeg_add_bs_dma_chain,
        "libmpeg_AddBsDmaChain",
    ),
    (
        0x0040_3600,
        libmpeg_set_picture_size,
        "libmpeg_SetPictureSize",
    ),
    (
        0x0040_3620,
        lib_mpeg_sequence_header,
        "libmpeg_SequenceHeader",
    ),
    (
        0x0040_3bb0,
        libmpeg_sequence_extension,
        "libmpeg_SequenceExtension",
    ),
    (
        0x0040_3ce0,
        libmpeg_sequence_display_extension,
        "libmpeg_SequenceDisplayExtension",
    ),
    (
        0x0040_3dd8,
        libmpeg_restart_dma_callback,
        "libmpeg_RestartDmaCallback",
    ),
    (
        0x0040_3e00,
        lib_mpeg_setup_macroblock,
        "libmpeg_SetupMacroblock",
    ),
    (
        0x0040_4050,
        lib_mpeg_form_predictions,
        "libmpeg_FormPredictions",
    ),
    (
        0x0040_4758,
        lib_mpeg_setup_pred_block,
        "libmpeg_SetupPredBlock",
    ),
    (
        0x0040_4b78,
        libmpeg_add_pred_and_store,
        "libmpeg_AddPredAndStore",
    ),
    (
        0x0040_5b48,
        libmpeg_set_ipu_mpeg1_bit,
        "libmpeg_SetIpuMpeg1Bit",
    ),
    (0x0040_5b70, lib_mpeg_wait_bdec, "libmpeg_WaitBdec"),
    (
        0x0040_5e10,
        lib_mpeg_decode_dm_vector,
        "libmpeg_DecodeDmVector",
    ),
    (
        0x0040_5e30,
        lib_mpeg_dual_prime_arithmetic,
        "libmpeg_DualPrimeArithmetic",
    ),
    (
        0x0040_5fb8,
        lib_mpeg_macroblock_address_increment,
        "libmpeg_MacroblockAddressIncrement",
    ),
    (0x0040_60c8, libmpeg_decode_slices, "libmpeg_DecodeSlices"),
    (0x0040_6318, libmpeg_decode_slice, "libmpeg_DecodeSlice"),
    (
        0x0040_6510,
        libmpeg_skipped_macroblock,
        "libmpeg_SkippedMacroblock",
    ),
    (
        0x0040_65d0,
        libmpeg_decode_macroblock,
        "libmpeg_DecodeMacroblock",
    ),
    (
        0x0040_6a78,
        libmpeg_decode_motion_vector,
        "libmpeg_DecodeMotionVector",
    ),
    (0x0040_6b00, libmpeg_motion_vectors, "libmpeg_MotionVectors"),
    (0x0040_6ca0, libmpeg_motion_vector, "libmpeg_MotionVector"),
    (0x0040_6ec8, libmpeg_ipu_wait_cmd, "libmpeg_IpuWaitCmd"),
    (
        0x0040_7438,
        libmpeg_next_start_code,
        "libmpeg_NextStartCode",
    ),
    (
        0x0040_74b8,
        libmpeg_slice_header_tail,
        "libmpeg_SliceHeaderTail",
    ),
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
    (
        0x0040_81d8,
        oc1123_check_output_buffer,
        "libmpeg_CheckOutputBuffer",
    ),
    (
        0x0040_8278,
        libmpeg_copy_output_raw,
        "libmpeg_CopyOutputRaw",
    ),
    (0x0040_8530, oc1123_compute_pts, "libmpeg_ComputePts"),
    (
        0x0040_8af8,
        oc1123_ipu_from_dma_start,
        "libmpeg_IpuFromDmaStart",
    ),
    (0x0040_8b80, oc1123_ipu_csc, "libmpeg_IpuCsc"),
    (0x0040_8df0, oc1123_ipu_csc_dma, "libmpeg_IpuCscDma"),
    (
        0x0040_9088,
        libmpeg_convert_picture_csc,
        "libmpeg_ConvertPictureCsc",
    ),
    (
        0x0040_2bb0,
        libmpeg_clear_frame_flags,
        "libmpeg_ClearFrameFlags",
    ),
];

// ---------------------------------------------------------------------------
// __ieee754_rem_pio2f (0x40e878)

/// `npio2_hw` (0x499540) and `two_over_pi` (0x499228) of newlib's
/// `e_rem_pio2f.c`.
pub(crate) const REM_PIO2F_NPIO2_HW: u32 = 0x0049_9540;

pub(crate) const REM_PIO2F_TWO_OVER_PI: u32 = 0x0049_9228;

pub(crate) const FABSF: Callee = Callee {
    addr: 0x0040_f8a0,
    f: ee::s15::fabsf,
};

pub(crate) const KERNEL_REM_PIO2F: Callee = Callee {
    addr: 0x0040_ec58,
    f: ee::s15::__kernel_rem_pio2f,
};

/// `f = bits` through AT (`lui/ori at; mtc1 at, f`).
#[inline]
pub(crate) fn rem_pio2f_const(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
}

/// The range test of one case: `cycles`, v0 = `v0` (the next constant the
/// original builds), `ix <= limit`.
pub(crate) fn rem_pio2f_below(c: &mut Cpu, cycles: u64, v0: u64, ix: i64, limit: i64) -> bool {
    c.cycles += cycles;
    c.g[V0][0] = v0;
    ix <= limit
}

/// `x -/+ pio2_1` (and the tail) for |x| < 3pi/4: subtracts for x > 0,
/// else adds. y[0] = f1, y[1] = f0; v0 = n = 1 or -1.
pub(crate) fn rem_pio2f_small(c: &mut Cpu, positive: bool) {
    let op = |c: &mut Cpu, fd: usize, fs: usize, ft: usize| {
        if positive {
            c.sub_s(fd, fs, ft)
        } else {
            c.add_s(fd, fs, ft)
        }
    };
    c.cycles += 9;
    rem_pio2f_const(c, 0, 0x3fc9_0f80); // pio2_1
    c.g[V0][0] = c.g[S0][0] & 0xffff_ffff_ffff_fff0;
    c.g[V1][0] = 0x3fc9_0fd0;
    op(c, 12, 12, 0);
    if c.g[V0][0] != c.g[V1][0] {
        c.cycles += 5;
        rem_pio2f_const(c, 2, 0x3735_4443); // pio2_1t
        op(c, 1, 12, 2);
    } else {
        // Near pi/2: 33+53 bits of pi.
        c.cycles += 8;
        rem_pio2f_const(c, 0, 0x3735_4400); // pio2_2
        rem_pio2f_const(c, 2, 0x2e85_a308); // pio2_2t
        op(c, 12, 12, 0);
        op(c, 1, 12, 2);
    }
    c.cycles += 6;
    let y = c.g[S1][0] as u32;
    c.sub_s(0, 12, 1);
    c.swc1(1, y);
    op(c, 0, 0, 2);
    c.swc1(0, y + 4);
    c.g[V0][0] = if positive { 1 } else { u64::MAX };
}

/// `int __ieee754_rem_pio2f(float x, float* y)` (0x40e878): x reduced by
/// n * pi/2 into y[0] + y[1]; returns n (mod 8 is all callers use). Small
/// cases inline, medium by n = round(|x| * 2/pi) with up to three pi/2
/// parts, huge ones through `__kernel_rem_pio2f`.
pub fn ieee754_rem_pio2f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        let sp = sp(c);
        c.sd(sp + 0x20, c.g[S1][0]);
        c.sd(sp + 0x40, c.g[RA][0]);
        c.sd(sp + 0x30, c.g[S2][0]);
        c.sd(sp + 0x10, c.g[S0][0]);
        c.g[S2][0] = sx(c.f[12]); // hx
        c.g[V1][0] = 0x7fff_ffff;
        c.g[S0][0] = c.g[S2][0] & 0x7fff_ffff; // ix
        c.g[S1][0] = c.g[A0][0];
        let ix = c.g[S0][0] as i64;
        let y = c.g[S1][0] as u32;
        if ix <= 0x3f49_0fd8 {
            // |x| <= pi/4: y = (x, 0), n = 0.
            c.g[V0][0] = 0;
            c.cycles += 4;
            c.swc1(12, y);
            Addr(y + 4).sw(c, 0);
        } else if rem_pio2f_below(c, 5, 0x4349_0000, ix, 0x4016_cbe3) {
            // |x| < 3pi/4.
            c.cycles += 2;
            c.g[V1][0] = 0x3fc9_0000;
            rem_pio2f_small(c, (c.g[S2][0] as i64) > 0);
        } else if rem_pio2f_below(c, 4, 0x7f7f_0000, ix, 0x4349_0f80) {
            // |x| <= 2^7 * pi/2: medium.
            c.cycles += 25;
            FABSF.call(c, 0x0040_e9dc)?;
            rem_pio2f_const(c, 1, 0x3f22_f984); // invpio2
            c.mov_s(5, 0); // t = |x|
            rem_pio2f_const(c, 2, 0x3f00_0000);
            c.mul_s(1, 5, 1);
            rem_pio2f_const(c, 0, 0x3fc9_0f80); // pio2_1
            rem_pio2f_const(c, 3, 0x3735_4443); // pio2_1t
            c.add_s(1, 1, 2);
            c.cvt_w_s(2, 1);
            c.mfc1(A1, 2); // n
            c.mtc1(A1, 6);
            c.cvt_s_w(6, 6); // fn
            let n = c.g[A1][0];
            c.g[V0][0] = ((n as i64) < 32) as u64;
            c.mul_s(0, 6, 0);
            c.mul_s(3, 6, 3);
            c.sub_s(4, 5, 0); // r = t - fn * pio2_1
            let y = c.g[S1][0] as u32;
            let mut quick = false;
            if (n as i64) < 32 {
                c.cycles += 11;
                let at = REM_PIO2F_NPIO2_HW.wrapping_add(((n as u32).wrapping_sub(1)) << 2);
                c.g[V1][0] = sx(at);
                c.g[A0][0] = Addr(at).lw(c);
                c.g[V0][0] = c.g[S0][0] & 0xffff_ffff_ffff_ff00;
                c.sub_s(0, 4, 3);
                if c.g[V0][0] != c.g[A0][0] {
                    c.cycles += 2;
                    c.swc1(0, y);
                    c.cycles += 1;
                    c.lwc1(1, y);
                    quick = true;
                }
            } else {
                c.cycles += 1;
                c.sub_s(0, 4, 3);
            }
            if !quick {
                c.cycles += 9;
                c.g[A0][0] = sx(((c.g[S0][0] as u32) as i32 >> 23) as u32); // j
                c.mfc1(V0, 0);
                Addr(y).sw(c, c.g[V0][0]);
                let i = rem_pio2f_exponent_gap(c);
                c.g[V0][0] = (i < 9) as u64;
                if i < 9 {
                    c.lwc1(1, y);
                } else {
                    // Second iteration: 2nd 24 bits of pi/2.
                    c.cycles += 22;
                    rem_pio2f_const(c, 0, 0x3735_4400); // pio2_2
                    c.mov_s(5, 4);
                    rem_pio2f_const(c, 1, 0x2e85_a308); // pio2_2t
                    c.mul_s(3, 6, 0);
                    c.mul_s(1, 6, 1);
                    c.sub_s(4, 4, 3);
                    c.sub_s(0, 5, 4);
                    c.sub_s(0, 0, 3);
                    c.sub_s(3, 1, 0);
                    c.sub_s(2, 4, 3);
                    c.mfc1(V0, 2);
                    Addr(y).sw(c, c.g[V0][0]);
                    let i = rem_pio2f_exponent_gap(c);
                    c.g[V0][0] = (i < 26) as u64;
                    if i < 26 {
                        c.lwc1(1, y);
                    } else {
                        // Third iteration.
                        c.cycles += 15;
                        rem_pio2f_const(c, 0, 0x2e85_a300); // pio2_3
                        c.mov_s(5, 4);
                        rem_pio2f_const(c, 2, 0x248d_3132); // pio2_3t
                        c.mul_s(3, 6, 0);
                        c.mul_s(2, 6, 2);
                        c.sub_s(4, 4, 3);
                        c.sub_s(0, 5, 4);
                        c.sub_s(0, 0, 3);
                        c.sub_s(3, 2, 0);
                        c.sub_s(1, 4, 3);
                        c.swc1(1, y);
                        c.cycles += 1;
                        c.lwc1(1, y);
                    }
                }
            }
            // y[1] = (r - y[0]) - w; negate for x < 0.
            c.cycles += 4;
            c.sub_s(0, 4, 1);
            c.sub_s(0, 0, 3);
            c.swc1(0, y + 4);
            if (c.g[S2][0] as i64) >= 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
            } else {
                c.cycles += 3;
                c.neg_s(1, 1);
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                rem_pio2f_store_negated(c);
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x7f7f_ffff;
            let nan = ix > 0x7f7f_ffff;
            c.g[V0][0] = 0;
            if nan {
                // Inf or NaN: y = (x - x, x - x), n = 0.
                c.cycles += 4;
                c.sub_s(0, 12, 12);
                c.swc1(0, y);
                c.swc1(0, y + 4);
            } else {
                rem_pio2f_large(c)?;
            }
        }
        c.cycles += 6;
        let sp = self::sp(c);
        c.g[RA][0] = c.ld(sp + 0x40);
        c.g[S2][0] = c.ld(sp + 0x30);
        c.g[S1][0] = c.ld(sp + 0x20);
        c.g[S0][0] = c.ld(sp + 0x10);
        c.g[SP][0] = sx(sp.wrapping_add(0x50));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `i = j - ((y[0] >> 23) & 0xff)` from the word in v0 (a0 = j); v1 = i.
pub(crate) fn rem_pio2f_exponent_gap(c: &mut Cpu) -> i64 {
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 23) & 0xff;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V1][0] as i64
}

/// x < 0: y[0] = -f1 (already negated), y[1] = -f0.
pub(crate) fn rem_pio2f_store_negated(c: &mut Cpu) {
    c.cycles += 4;
    let y = c.g[S1][0] as u32;
    c.neg_s(0, 0);
    c.swc1(1, y);
    c.swc1(0, y + 4);
}

/// |x| > 2^7 * pi/2: split x into three 8-bit-exponent floats on the stack
/// and hand them to `__kernel_rem_pio2f`.
pub(crate) fn rem_pio2f_large(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 10;
    let sp = sp(c);
    c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 23) as u32);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x86)); // e0
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 23);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.mtc1(S0, 12); // z
    rem_pio2f_const(c, 2, 0x4380_0000); // two8
    c.g[V1][0] = 1;
    c.g[A0][0] = sx(sp);
    loop {
        c.cycles += 10;
        c.cvt_w_s(0, 12);
        c.mfc1(V0, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(1, 12, 0);
        c.swc1(0, c.g[A0][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        c.mul_s(12, 1, 2);
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_eb90);
        }
    }
    // tx[2] = z; skip trailing zeros (nx).
    c.cycles += 6;
    c.f[1] = 0;
    c.g[A3][0] = 3;
    c.c_eq_s(12, 1);
    c.swc1(12, sp + 8);
    if c.fcc() {
        c.cycles += 2;
        c.g[A0][0] = 2;
        c.g[A3][0] = 2;
        loop {
            c.cycles += 9;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx(sp.wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, c.g[V0][0] as u32);
            c.c_eq_s(0, 1);
            if !c.fcc() {
                break;
            }
            c.g[A3][0] = c.g[A0][0];
            if event_due(c) {
                deliver_event_here(c, 0x0040_ebd8);
            }
        }
    }
    c.cycles += 8;
    c.g[T1][0] = sx(REM_PIO2F_TWO_OVER_PI);
    c.g[A0][0] = sx(sp);
    c.g[A1][0] = c.g[S1][0];
    c.g[T0][0] = 2;
    KERNEL_REM_PIO2F.call(c, 0x0040_ec14)?;
    c.g[A1][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) >= 0 {
        c.cycles += 1;
        c.g[V0][0] = c.g[A1][0];
    } else {
        c.cycles += 4;
        let y = c.g[S1][0] as u32;
        c.lwc1(1, y);
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
        c.lwc1(0, y + 4);
        c.neg_s(1, 1);
        rem_pio2f_store_negated(c);
    }
    Ok(())
}

pub(crate) mod callee {
    use super::*;

    /// `float scalbnf(float x, int n)`.
    pub(crate) const SCALBNF: Callee = Callee {
        addr: 0x0040_f9f8,
        f: ee::s15::scalbnf,
    };

    /// `float floorf(float x)`.
    pub(crate) const FLOORF: Callee = Callee {
        addr: 0x0040_f8e8,
        f: ee::s15::floorf,
    };
}

pub(crate) mod kernel_rem_pio2f_reg {
    use ssx3_ee_rt::cpu::reg::*;

    /// `jz`: the number of 8-bit chunks in use.
    pub(crate) const JZ: usize = S0;

    /// `jx = nx - 1`; from the chop on, the result `n & 7`.
    pub(crate) const JX: usize = S1;

    /// `q0`: the binary exponent of the lowest chunk.
    pub(crate) const Q0: usize = S2;

    /// `n`: the integer part of x * 2/pi.
    pub(crate) const N: usize = S3;

    /// `jk = init_jk[prec]` (also used as `jp`).
    pub(crate) const JK: usize = S4;

    /// `&q[0]`.
    pub(crate) const QS: usize = S5;

    /// `jv`: the first `ipio2` chunk needed.
    pub(crate) const JV: usize = S6;

    /// `y`.
    pub(crate) const Y: usize = S7;

    /// `4 * jz` as of the last distill.
    pub(crate) const JZ4: usize = FP;

    /// FPU f20: `z`.
    pub(crate) const Z: usize = 20;

    /// FPU f2: `fw`, the running sum.
    pub(crate) const FW: usize = 2;
}

pub(crate) mod kernel_rem_pio2f_slot {

    pub(crate) const SIZE: u32 = 0x210;

    /// `int iq[20]`: the fraction of x * 2/pi in 8-bit chunks, lowest first.
    pub(crate) const IQ: u32 = 0x00;

    /// `float f[20]`: the `ipio2` chunks the products use, as floats.
    pub(crate) const F: u32 = 0x50;

    /// `float fq[20]`: the `PIo2[] * q[]` sums.
    pub(crate) const FQ: u32 = 0xa0;

    /// `float q[20]`: the `x[] * f[]` sums, later the scaled chunks.
    pub(crate) const Q: u32 = 0xf0;

    /// The arguments x, prec and ipio2.
    pub(crate) const X: u32 = 0x140;

    pub(crate) const PREC: u32 = 0x144;

    pub(crate) const IPIO2: u32 = 0x148;

    /// `ih`: 0, 1 (q > 0.5 by the chunk bits) or 2 (by z >= 0.5).
    pub(crate) const IH: u32 = 0x14c;

    /// `prec < 3`, for the final switch.
    pub(crate) const PREC_BELOW_3: u32 = 0x150;

    /// `jk < 0` (no q terms and empty PIo2 sums).
    pub(crate) const JK_NEGATIVE: u32 = 0x154;

    pub(crate) const RA: u32 = 0x1f0;

    pub(crate) const F20: u32 = 0x200;
}

pub(crate) mod kernel_rem_pio2f_head {

    /// The `f[]` fill.
    pub(crate) const FILL_F: u32 = 0x0040_ed08;

    /// The first `q[i]` terms, i = 0..=jk, and their x * f products.
    pub(crate) const Q_TERM: u32 = 0x0040_ed78;

    pub(crate) const Q_DOT: u32 = 0x0040_ed98;

    /// `recompute:`.
    pub(crate) const RECOMPUTE: u32 = 0x0040_edf4;

    /// q[] into iq[] (most significant last).
    pub(crate) const DISTILL: u32 = 0x0040_ee28;

    /// 1 - q, chunk by chunk with the borrow.
    pub(crate) const CARRY: u32 = 0x0040_ef58;

    /// `j |= iq[i]` for i = jz-1 down to jk.
    pub(crate) const ZERO_SCAN: u32 = 0x0040_f028;

    /// `for (k = 1; iq[jk - k] == 0; k++)`.
    pub(crate) const K_SCAN: u32 = 0x0040_f068;

    /// The k extra terms and their products.
    pub(crate) const NEW_TERM: u32 = 0x0040_f0b0;

    pub(crate) const NEW_DOT: u32 = 0x0040_f0f0;

    /// Dropping zero chunks at the top.
    pub(crate) const CHOP: u32 = 0x0040_f180;

    /// `q[i] = fw * iq[i]`.
    pub(crate) const SCALE: u32 = 0x0040_f260;

    /// The `fq[jz - i]` sums and their PIo2 * q products.
    pub(crate) const FQ_TERM: u32 = 0x0040_f298;

    pub(crate) const FQ_DOT: u32 = 0x0040_f2c0;

    /// prec 0: the sum of fq[].
    pub(crate) const SUM0: u32 = 0x0040_f378;

    /// prec 1 and 2: the sum of fq[], then fq[1..=jz] onto fq[0] - sum.
    pub(crate) const SUM12: u32 = 0x0040_f3c8;

    pub(crate) const SUM12_TAIL: u32 = 0x0040_f418;

    /// prec 3: the two renormalising passes and the sum of fq[2..].
    pub(crate) const FOLD3_PASS1: u32 = 0x0040_f468;

    pub(crate) const FOLD3_PASS2: u32 = 0x0040_f4b8;

    pub(crate) const SUM3: u32 = 0x0040_f510;
}

// ---------------------------------------------------------------------------
// __kernel_rem_pio2f (0x40ec58)

/// `init_jk[] = {4, 7, 9}` (0x4995e8), the number of terms per precision;
/// `init_jk[3]` reads the zero word after it.
pub(crate) const KERNEL_REM_PIO2F_INIT_JK: u32 = 0x0049_95e8;

/// `PIo2[]` (0x4995f8): pi/2 in 8-bit pieces. The loop addresses it as
/// `lui t3, 0x4a` minus 0x6a08 and keeps the upper half in t3.
pub(crate) const KERNEL_REM_PIO2F_PIO2: u32 = 0x0049_95f8;

pub(crate) const KERNEL_REM_PIO2F_PIO2_HI: u32 = 0x004a_0000;

/// The float literals, built with `lui at` (`two8`, `twon8`, 0.125, 8,
/// 0.5, 1).
pub(crate) const KERNEL_REM_PIO2F_TWO8: u32 = 0x4380_0000;

pub(crate) const KERNEL_REM_PIO2F_TWON8: u32 = 0x3b80_0000;

pub(crate) const KERNEL_REM_PIO2F_EIGHTH: u32 = 0x3e00_0000;

pub(crate) const KERNEL_REM_PIO2F_EIGHT: u32 = 0x4100_0000;

pub(crate) const KERNEL_REM_PIO2F_HALF: u32 = 0x3f00_0000;

pub(crate) const KERNEL_REM_PIO2F_ONE: u32 = 0x3f80_0000;

/// The callee-saved GPRs and their `sd` slots.
pub(crate) const KERNEL_REM_PIO2F_SAVED: [(usize, u32); 9] = [
    (S0, 0x160),
    (S1, 0x170),
    (S2, 0x180),
    (S3, 0x190),
    (S4, 0x1a0),
    (S5, 0x1b0),
    (S6, 0x1c0),
    (S7, 0x1d0),
    (FP, 0x1e0),
];

/// `sp + off` in the frame.
#[inline]
pub(crate) fn kernel_rem_pio2f_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `(char *) iq + byte_off`.
#[inline]
pub(crate) fn kernel_rem_pio2f_iq(c: &Cpu, byte_off: u32) -> u32 {
    kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::IQ).wrapping_add(byte_off)
}

/// A back-edge event check at `head`.
#[inline]
pub(crate) fn kernel_rem_pio2f_back_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// `int __kernel_rem_pio2f(float *x, float *y, int e0, int nx, int prec,
/// const __int32_t *ipio2)` (0x40ec58): fdlibm's reduction of a huge float
/// by pi/2. x[0..nx] are the argument's 8-bit pieces (x[0] * 2^e0 the
/// leading one); the product with the 2/pi chunks in `ipio2` is distilled
/// into 8-bit chunks
/// (adding terms while the fraction comes out all zero), the fraction is
/// multiplied by pi/2 and folded into y[] (one float for prec 0, two for 1
/// and 2, three for 3; nothing for others). Returns n & 7, the quadrant.
pub fn kernel_rem_pio2f(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    guest(c, |c| {
        kernel_rem_pio2f_setup(c);
        kernel_rem_pio2f_fill_f(c);
        kernel_rem_pio2f_first_terms(c);
        c.cycles += 1;
        c.g[JZ][0] = c.g[JK][0];
        loop {
            kernel_rem_pio2f_distill(c);
            kernel_rem_pio2f_take_n(c)?;
            kernel_rem_pio2f_round_up(c)?;
            if !kernel_rem_pio2f_add_terms(c) {
                break;
            }
            kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::RECOMPUTE);
        }
        kernel_rem_pio2f_chop(c)?;
        kernel_rem_pio2f_scale(c)?;
        kernel_rem_pio2f_pio2_sums(c);
        kernel_rem_pio2f_fold(c);
        kernel_rem_pio2f_epilogue(c);
        Ok(())
    })
}

/// The prologue, the argument spills and `jv`, `q0`, `jx`, `jk`. Leaves
/// a1 = j = jv - jx, a2 = i = 0, a3 = m = jx + jk.
pub(crate) fn kernel_rem_pio2f_setup(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    c.cycles += 37;
    c.g[SP][0] = sx(sp(c).wrapping_sub(slot::SIZE));
    for (reg, off) in KERNEL_REM_PIO2F_SAVED {
        let at = kernel_rem_pio2f_at(c, off);
        c.sd(at, c.g[reg][0]);
    }
    c.sd(kernel_rem_pio2f_at(c, slot::RA), c.g[RA][0]);
    c.swc1(Z, kernel_rem_pio2f_at(c, slot::F20));
    c.sw(kernel_rem_pio2f_at(c, slot::PREC), c.g[T0][0]);
    // jv = (e0 - 3) / 8 rounded toward zero ((e0 + 4) >> 3 below zero),
    // at least 0.
    c.g[T4][0] = sx(r32(c, A2).wrapping_sub(3));
    c.g[T3][0] = u64::MAX;
    c.g[V0][0] = sx(r32(c, A2).wrapping_add(4));
    c.g[T2][0] = ((c.g[T3][0] as i64) < (c.g[T4][0] as i64)) as u64;
    if c.g[T2][0] != 0 {
        c.g[V0][0] = c.g[T4][0];
    }
    c.g[JV][0] = sx((r32(c, V0) as i32 >> 3) as u32);
    c.g[T3][0] = ((c.g[T3][0] as i64) < (c.g[JV][0] as i64)) as u64;
    if c.g[T3][0] == 0 {
        c.g[JV][0] = 0;
    }
    // &init_jk[prec]
    c.g[V1][0] = sx(KERNEL_REM_PIO2F_INIT_JK);
    c.g[V0][0] = sx(r32(c, T0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    // q0 = e0 - 8 * (jv + 1)
    c.g[V1][0] = sx(r32(c, JV).wrapping_add(1));
    c.g[V1][0] = sx(r32(c, V1) << 3);
    c.g[JX][0] = sx(r32(c, A3).wrapping_sub(1));
    c.g[Q0][0] = sx(r32(c, A2).wrapping_sub(r32(c, V1)));
    c.g[Y][0] = c.g[A1][0];
    c.g[A1][0] = sx(r32(c, JV).wrapping_sub(r32(c, JX)));
    c.g[A2][0] = 0;
    c.g[JK][0] = c.lw(r32(c, V0));
    c.sw(kernel_rem_pio2f_at(c, slot::X), c.g[A0][0]);
    c.g[A3][0] = sx(r32(c, JX).wrapping_add(r32(c, JK)));
    c.sw(kernel_rem_pio2f_at(c, slot::IPIO2), c.g[T1][0]);
}

/// `f[i] = j < 0 ? 0 : (float) ipio2[j]` for i = 0..=m (j = jv - jx + i),
/// and the `jk < 0` / `prec < 3` flags. Leaves t0 = (jk < 0).
pub(crate) fn kernel_rem_pio2f_fill_f(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    if (c.g[A3][0] as i64) < 0 {
        // m < 0: nothing to fill.
        c.cycles += 7;
        c.g[A2][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC));
        c.g[A1][0] = ((c.g[JK][0] as i64) < 0) as u64;
        c.sw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE), c.g[A1][0]);
        c.g[QS][0] = sx(kernel_rem_pio2f_at(c, slot::Q));
        c.g[A2][0] = ((c.g[A2][0] as i64) < 3) as u64;
        c.sw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3), c.g[A2][0]);
        c.g[T0][0] = c.lw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE));
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = ((c.g[JK][0] as i64) < 0) as u64;
    c.g[V1][0] = ((c.g[T0][0] as i64) < 3) as u64;
    c.sw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE), c.g[V0][0]);
    c.g[T1][0] = sx(kernel_rem_pio2f_at(c, slot::F));
    c.sw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3), c.g[V1][0]);
    c.g[QS][0] = sx(kernel_rem_pio2f_at(c, slot::Q));
    loop {
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V0))); // &f[i]
        if (c.g[A1][0] as i64) < 0 {
            c.cycles += 1;
            c.sw(r32(c, V1), 0);
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.lw(kernel_rem_pio2f_at(c, slot::IPIO2));
            c.g[V0][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.lwc1(0, r32(c, V0));
            c.cvt_s_w(0, 0);
            c.swc1(0, r32(c, V1));
        }
        c.cycles += 4;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        if c.g[V0][0] != 0 {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::FILL_F);
    }
    c.cycles += 2;
    c.g[T0][0] = c.lw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE));
}

/// `fw = sum of x[j] * f[base - j]` for j = 0..=jx onto f2, with a1 = j,
/// `base` in register `index`, t1 = f, a3 = 4 * i; leaves v1 = &q[i].
pub(crate) fn kernel_rem_pio2f_dot(c: &mut Cpu, index: usize, head: u32) {
    use kernel_rem_pio2f_reg::*;
    loop {
        c.cycles += 13;
        c.g[V0][0] = sx(r32(c, index).wrapping_sub(r32(c, A1)));
        c.g[A2][0] = c.lw(kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::X));
        c.g[V1][0] = sx(r32(c, A1) << 2);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
        c.lwc1(1, r32(c, V0)); // f[base - j]
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.lwc1(0, r32(c, V1)); // x[j]
        c.g[V0][0] = ((c.g[JX][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.mul_s(0, 0, 1);
        c.add_s(FW, FW, 0);
        if c.g[V0][0] != 0 {
            break;
        }
        kernel_rem_pio2f_back_edge(c, head);
    }
    c.cycles += 2;
    c.g[V1][0] = sx(r32(c, QS).wrapping_add(r32(c, A3)));
}

/// jx < 0: no products; t0 = i + 1, v1 = &q[i].
pub(crate) fn kernel_rem_pio2f_no_products(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 3;
    c.g[T0][0] = sx(r32(c, A2).wrapping_add(1));
    c.g[A3][0] = sx(r32(c, A2) << 2);
    c.g[V1][0] = sx(r32(c, QS).wrapping_add(r32(c, A3)));
}

/// `q[i] = fw` and i = t0; true once i passes the register `last`.
pub(crate) fn kernel_rem_pio2f_store_term(c: &mut Cpu, last: usize) -> bool {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 4;
    c.g[A2][0] = c.g[T0][0];
    c.g[V0][0] = ((c.g[last][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.swc1(FW, r32(c, V1));
    c.g[V0][0] != 0
}

/// `q[i] = sum of x[j] * f[jx + i - j]` for i = 0..=jk (none when jk < 0,
/// t0 set).
pub(crate) fn kernel_rem_pio2f_first_terms(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 2;
    c.g[A2][0] = 0;
    if c.g[T0][0] != 0 {
        return;
    }
    c.cycles += 2;
    c.g[T3][0] = ((c.g[JX][0] as i64) < 0) as u64;
    loop {
        c.cycles += 3;
        c.mtc1(ZERO, FW);
        c.g[A1][0] = 0;
        if c.g[T3][0] != 0 {
            kernel_rem_pio2f_no_products(c);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(r32(c, JX).wrapping_add(r32(c, A2)));
            c.g[A3][0] = sx(r32(c, A2) << 2);
            c.g[T1][0] = sx(kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::F));
            c.g[T0][0] = sx(r32(c, A2).wrapping_add(1));
            kernel_rem_pio2f_dot(c, A0, kernel_rem_pio2f_head::Q_DOT);
        }
        if kernel_rem_pio2f_store_term(c, JK) {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::Q_TERM);
    }
}

/// Distil q[0..=jz] into iq[]: from the top term down, `fw = (int)(z /
/// 256)`, `iq[i] = (int)(z - 256 * fw)`, `z = q[j - 1] + fw`. z ends as
/// the integer part (q[0] plus the carry).
pub(crate) fn kernel_rem_pio2f_distill(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 5;
    c.g[JZ4][0] = sx(r32(c, JZ) << 2);
    c.g[A1][0] = c.g[JZ][0];
    c.g[V0][0] = sx(r32(c, QS).wrapping_add(r32(c, JZ4)));
    c.lwc1(Z, r32(c, V0)); // z = q[jz]
    if (c.g[JZ][0] as i64) <= 0 {
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = sx(r32(c, JZ4).wrapping_sub(4));
    rem_pio2f_const(c, 4, KERNEL_REM_PIO2F_TWON8);
    rem_pio2f_const(c, 3, KERNEL_REM_PIO2F_TWO8);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, QS))); // &q[jz - 1]
    c.g[V1][0] = c.g[SP][0]; // &iq[0]
    loop {
        c.cycles += 15;
        c.mul_s(0, Z, 4);
        c.lwc1(1, r32(c, A0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        c.cvt_w_s(2, 0);
        c.mfc1(V0, 2);
        c.mtc1(V0, 2);
        c.cvt_s_w(FW, 2);
        c.mul_s(0, FW, 3);
        c.sub_s(0, Z, 0);
        c.add_s(Z, 1, FW);
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, V1));
        let more = (c.g[A1][0] as i64) > 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        if !more {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::DISTILL);
    }
}

/// `z = scalbnf(z, q0)`, `z -= 8 * floorf(z / 8)`, `n = (int) z`, `z -= n`,
/// then `ih`: from the chunk below the point when q0 >= 0 (moving its bits
/// above the point into n when q0 > 0), else 2 when z >= 0.5.
pub(crate) fn kernel_rem_pio2f_take_n(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    let ih_at = kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::IH);
    c.cycles += 19;
    c.mov_s(12, Z);
    c.g[A0][0] = c.g[Q0][0];
    callee::SCALBNF.call(c, 0x0040_ee70)?;
    rem_pio2f_const(c, 12, KERNEL_REM_PIO2F_EIGHTH);
    c.mov_s(Z, 0);
    c.sw(ih_at, 0);
    c.mul_s(12, Z, 12);
    callee::FLOORF.call(c, 0x0040_ee88)?;
    rem_pio2f_const(c, 1, KERNEL_REM_PIO2F_EIGHT);
    c.mul_s(0, 0, 1);
    c.sub_s(Z, Z, 0);
    c.cvt_w_s(0, Z);
    c.mfc1(N, 0);
    c.mtc1(N, 0);
    c.cvt_s_w(0, 0);
    let q0 = c.g[Q0][0] as i64;
    c.sub_s(Z, Z, 0);
    if q0 > 0 {
        // i = iq[jz-1] >> (8 - q0); n += i; iq[jz-1] -= i << (8 - q0);
        // ih = iq[jz-1] >> (7 - q0).
        c.cycles += 16;
        c.g[V0][0] = sx(r32(c, JZ).wrapping_sub(1));
        c.g[V1][0] = 8;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, Q0)));
        c.g[A1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz - 1]
        c.g[A0][0] = 7;
        c.g[V0][0] = c.lw(r32(c, A1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, Q0)));
        c.g[A2][0] = sx((r32(c, V0) as i32 >> (r32(c, V1) & 31)) as u32);
        c.g[V1][0] = sx(r32(c, A2) << (r32(c, V1) & 31));
        c.g[N][0] = sx(r32(c, N).wrapping_add(r32(c, A2)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        c.g[A0][0] = sx((r32(c, V0) as i32 >> (r32(c, A0) & 31)) as u32);
        c.sw(ih_at, c.g[A0][0]);
        c.sw(r32(c, A1), c.g[V0][0]);
        return Ok(());
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, JZ).wrapping_sub(1));
    if c.g[Q0][0] == 0 {
        // ih = iq[jz-1] >> 8.
        c.cycles += 6;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        c.g[A0][0] = sx((r32(c, A0) as i32 >> 8) as u32);
        c.sw(ih_at, c.g[A0][0]);
    } else {
        c.cycles += 6;
        rem_pio2f_const(c, 0, KERNEL_REM_PIO2F_HALF);
        c.c_le_s(0, Z);
        c.g[V0][0] = 2;
        if c.fcc() {
            c.cycles += 1;
            c.sw(ih_at, c.g[V0][0]);
        }
    }
    Ok(())
}

/// ih > 0 (q > 0.5): n += 1 and the chunks become 1 - q (the lowest
/// non-zero one `0x100 - j`, every one above it `0xff - j`; a3 = carry),
/// the bits above the point cleared again for q0 = 1, 2; for ih = 2 also
/// `z = 1 - z`, less `scalbnf(1, q0)` when a chunk was non-zero.
pub(crate) fn kernel_rem_pio2f_round_up(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    let ih_at = kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::IH);
    c.cycles += 3;
    c.g[V1][0] = c.lw(ih_at);
    if (c.g[V1][0] as i64) <= 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[N][0] = sx(r32(c, N).wrapping_add(1));
    c.g[A3][0] = 0;
    if (c.g[JZ][0] as i64) > 0 {
        c.cycles += 5;
        c.g[T0][0] = 0x100;
        c.g[A0][0] = 0xff;
        c.g[V1][0] = c.g[SP][0]; // &iq[0]
        c.g[A2][0] = c.g[JZ][0];
        loop {
            c.cycles += 2;
            c.g[A1][0] = c.lw(r32(c, V1)); // j = iq[i]
            let store = if c.g[A3][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A1)));
                true
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, T0).wrapping_sub(r32(c, A1)));
                if c.g[A1][0] != 0 {
                    c.cycles += 2;
                    c.g[A3][0] = 1;
                    true
                } else {
                    false
                }
            };
            if store {
                c.cycles += 1;
                c.sw(r32(c, V1), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            let more = c.g[A2][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if !more {
                break;
            }
            kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::CARRY);
        }
    }
    // switch (q0): case 1: iq[jz-1] &= 0x7f; case 2: iq[jz-1] &= 0x3f.
    c.cycles += 2;
    c.g[V0][0] = 1;
    let mut ih_loaded = false;
    if (c.g[Q0][0] as i64) > 0 {
        c.cycles += 2;
        let q0_is_1 = c.g[Q0][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        let mask = if q0_is_1 {
            c.cycles += 6;
            Some(0x7fu64)
        } else {
            c.cycles += 2;
            let q0_is_2 = c.g[Q0][0] == c.g[V0][0];
            c.g[A0][0] = c.lw(ih_at);
            if q0_is_2 {
                c.cycles += 5;
                Some(0x3fu64)
            } else {
                c.cycles += 2;
                ih_loaded = true;
                None
            }
        };
        if let Some(mask) = mask {
            c.g[V0][0] = sx(r32(c, JZ).wrapping_sub(1));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[A0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz - 1]
            c.g[V1][0] = c.lw(r32(c, A0)) & mask;
            c.cycles += 1;
            c.sw(r32(c, A0), c.g[V1][0]);
        }
    }
    if !ih_loaded {
        c.cycles += 2;
        c.g[A0][0] = c.lw(ih_at);
        c.g[V0][0] = 2;
    }
    c.cycles += 2;
    if c.g[A0][0] != c.g[V0][0] {
        return Ok(());
    }
    // ih == 2
    c.cycles += 4;
    rem_pio2f_const(c, 12, KERNEL_REM_PIO2F_ONE);
    c.sub_s(Z, 12, Z);
    if c.g[A3][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[Q0][0];
    callee::SCALBNF.call(c, 0x0040_effc)?;
    c.sub_s(Z, Z, 0);
    Ok(())
}

/// z == 0 and the chunks `iq[jk..jz-1]` all zero: the fraction needs more
/// terms. Adds k of them (k - 1 = the zero chunks below `iq[jk]`), filling
/// `f[jx + i]` and `q[i]` for i = jz+1..=jz+k, and returns true to distil
/// again. Otherwise leaves fcc = (z == 0) and returns false.
pub(crate) fn kernel_rem_pio2f_add_terms(c: &mut Cpu) -> bool {
    use kernel_rem_pio2f_head as head;
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    c.cycles += 5;
    c.mtc1(ZERO, 0);
    c.c_eq_s(Z, 0);
    c.g[A2][0] = sx(r32(c, JZ).wrapping_sub(1));
    if !c.fcc() {
        return false;
    }
    // j = iq[jz-1] | ... | iq[jk] (a1)
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[JK][0] as i64)) as u64;
    c.g[A1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[A0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz - 1]
        loop {
            c.cycles += 7;
            c.g[V1][0] = c.lw(r32(c, A0));
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[JK][0] as i64)) as u64;
            c.g[A1][0] |= c.g[V1][0];
            if c.g[V0][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::ZERO_SCAN);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, JK).wrapping_sub(1));
    if c.g[A1][0] != 0 {
        c.cycles += 3;
        c.mtc1(ZERO, 0);
        c.c_eq_s(Z, 0);
        return false;
    }
    // k (t0): 1 + the zero chunks iq[jk-1], iq[jk-2], ...
    c.cycles += 5;
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
    c.g[A0][0] = c.lw(r32(c, V1));
    c.g[T0][0] = 1;
    c.cycles += 2;
    c.g[A1][0] = sx(r32(c, JZ).wrapping_add(1));
    if c.g[A0][0] == 0 {
        loop {
            c.cycles += 7;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, JK).wrapping_sub(r32(c, T0)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            if c.g[A0][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::K_SCAN);
        }
        c.cycles += 2;
    }
    c.g[A0][0] = sx(r32(c, JZ).wrapping_add(r32(c, T0))); // the new jz
    c.cycles += 4;
    c.g[A2][0] = c.g[A1][0]; // i = jz + 1
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.g[JZ][0] = c.g[A0][0];
    if c.g[V0][0] != 0 {
        // No new term (cannot happen: k >= 1).
        return true;
    }
    c.cycles += 3;
    c.g[T1][0] = sx(kernel_rem_pio2f_at(c, slot::F));
    c.g[T3][0] = ((c.g[JX][0] as i64) < 0) as u64;
    loop {
        // f[jx + i] = (float) ipio2[jv + i]
        c.cycles += 13;
        c.g[A1][0] = c.lw(kernel_rem_pio2f_at(c, slot::IPIO2));
        c.g[V0][0] = sx(r32(c, JV).wrapping_add(r32(c, A2)));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[T2][0] = sx(r32(c, JX).wrapping_add(r32(c, A2)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.g[V1][0] = sx(r32(c, T2) << 2);
        c.lwc1(0, r32(c, V0));
        c.cvt_s_w(0, 0);
        c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V1)));
        c.g[A1][0] = 0;
        c.mtc1(ZERO, FW);
        c.swc1(0, r32(c, V1));
        if c.g[T3][0] != 0 {
            kernel_rem_pio2f_no_products(c);
        } else {
            c.cycles += 3;
            c.g[T0][0] = sx(r32(c, A2).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, A2) << 2);
            kernel_rem_pio2f_dot(c, T2, head::NEW_DOT);
        }
        if kernel_rem_pio2f_store_term(c, A0) {
            break;
        }
        kernel_rem_pio2f_back_edge(c, head::NEW_TERM);
    }
    c.cycles += 2;
    c.g[JZ][0] = c.g[A0][0];
    true
}

/// Chop: z == 0 drops the zero chunks at the top (jz -= 1, q0 -= 8 each);
/// otherwise `z = scalbnf(z, -q0)` becomes the top chunk, split in two
/// when it is 256 or more. Sets s1 = n & 7.
pub(crate) fn kernel_rem_pio2f_chop(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 2;
    let zero = c.fcc();
    c.mov_s(12, Z);
    if zero {
        c.cycles += 6;
        c.g[JZ][0] = sx(r32(c, JZ).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, JZ) << 2);
        c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1)); // iq[jz]
        c.g[Q0][0] = sx(r32(c, Q0).wrapping_sub(8));
        c.cycles += 1;
        c.g[JX][0] = c.g[N][0] & 7;
        if c.g[A0][0] == 0 {
            loop {
                c.cycles += 7;
                c.g[JZ][0] = sx(r32(c, JZ).wrapping_sub(1));
                c.g[Q0][0] = sx(r32(c, Q0).wrapping_sub(8));
                c.g[V0][0] = sx(r32(c, JZ) << 2);
                c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                if c.g[A0][0] != 0 {
                    break;
                }
                kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::CHOP);
            }
            c.cycles += 2;
        }
        return Ok(());
    }
    c.cycles += 9;
    c.g[A0][0] = sx(0u32.wrapping_sub(r32(c, Q0)));
    callee::SCALBNF.call(c, 0x0040_f1ac)?;
    rem_pio2f_const(c, 1, KERNEL_REM_PIO2F_TWO8);
    c.mov_s(Z, 0);
    c.c_le_s(1, Z);
    c.g[A0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, JZ4))); // &iq[jz]
    if c.fcc() {
        // z >= two8: iq[jz] = low 8 bits, iq[jz + 1] = the rest.
        c.cycles += 19;
        rem_pio2f_const(c, 0, KERNEL_REM_PIO2F_TWON8);
        c.g[JZ][0] = sx(r32(c, JZ).wrapping_add(1));
        c.g[V0][0] = sx(r32(c, JZ) << 2);
        c.mul_s(0, Z, 0);
        c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
        c.g[JX][0] = c.g[N][0] & 7;
        c.cvt_w_s(2, 0);
        c.mfc1(V0, 2);
        c.mtc1(V0, 2);
        c.cvt_s_w(FW, 2);
        c.mul_s(0, FW, 1);
        c.sub_s(0, Z, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, A0));
        c.cvt_w_s(0, FW);
        c.swc1(0, r32(c, V1));
        c.g[Q0][0] = sx(r32(c, Q0).wrapping_add(8));
    } else {
        c.cycles += 5;
        c.g[V0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, JZ4)));
        c.cvt_w_s(0, Z);
        c.swc1(0, r32(c, V0));
        c.g[JX][0] = c.g[N][0] & 7;
    }
    Ok(())
}

/// `fw = scalbnf(1, q0)`; `q[i] = fw * iq[i]`, `fw *= twon8` for i = jz
/// down to 0. Leaves a2 = jz.
pub(crate) fn kernel_rem_pio2f_scale(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 7;
    rem_pio2f_const(c, 12, KERNEL_REM_PIO2F_ONE);
    c.g[A0][0] = c.g[Q0][0];
    callee::SCALBNF.call(c, 0x0040_f23c)?;
    c.g[A2][0] = c.g[JZ][0];
    c.mov_s(FW, 0);
    if (c.g[A2][0] as i64) < 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = sx(r32(c, A2) << 2);
    rem_pio2f_const(c, 1, KERNEL_REM_PIO2F_TWON8);
    c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz]
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, QS))); // &q[jz]
    loop {
        c.cycles += 9;
        c.lwc1(0, r32(c, V1));
        c.cvt_s_w(0, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(4));
        c.mul_s(0, FW, 0);
        c.mul_s(FW, FW, 1);
        c.swc1(0, r32(c, V0));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::SCALE);
    }
    c.cycles += 1;
    c.g[A2][0] = c.g[JZ][0];
    Ok(())
}

/// The fq slot of term i (v0 = 4 * (jz - i), t1 = jz - i) and the next i
/// (a2 = a1 = i - 1).
pub(crate) fn kernel_rem_pio2f_fq_next(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, T1) << 2);
    c.g[A2][0] = c.g[A1][0];
}

/// `fq[jz - i] = sum of PIo2[k] * q[i + k]` for k = 0..=min(jp, jz - i),
/// i = jz down to 0 (a2 = i, t0 = k, a3 = fq). Leaves v1 = (prec < 3).
pub(crate) fn kernel_rem_pio2f_pio2_sums(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    c.cycles += 2;
    if (c.g[A2][0] as i64) < 0 {
        c.g[V1][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3));
        return;
    }
    c.cycles += 2;
    c.g[A3][0] = sx(kernel_rem_pio2f_at(c, slot::FQ));
    loop {
        c.cycles += 4;
        c.g[V0][0] = c.lw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE));
        c.mtc1(ZERO, FW);
        c.g[T0][0] = 0;
        if c.g[V0][0] != 0 {
            // jp < 0: no products.
            c.cycles += 2;
            c.g[T1][0] = sx(r32(c, JZ).wrapping_sub(r32(c, A2)));
            c.g[A1][0] = sx(r32(c, A2).wrapping_sub(1));
            kernel_rem_pio2f_fq_next(c);
        } else {
            c.cycles += 3;
            c.g[T1][0] = sx(r32(c, JZ).wrapping_sub(r32(c, A2)));
            c.g[T2][0] = c.g[T1][0];
            if (c.g[T1][0] as i64) < 0 {
                // Cannot happen: i <= jz.
                c.cycles += 1;
                c.g[A1][0] = sx(r32(c, A2).wrapping_sub(1));
                kernel_rem_pio2f_fq_next(c);
            } else {
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[T3][0] = sx(KERNEL_REM_PIO2F_PIO2_HI);
                loop {
                    c.cycles += 13;
                    c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, T0)));
                    c.g[A0][0] = sx(r32(c, T0) << 2);
                    c.g[V0][0] = sx(KERNEL_REM_PIO2F_PIO2);
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, QS).wrapping_add(r32(c, V1)));
                    c.lwc1(0, r32(c, A0)); // PIo2[k]
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    c.lwc1(1, r32(c, V1)); // q[i + k]
                    c.g[V0][0] = ((c.g[JK][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.mul_s(0, 0, 1);
                    c.add_s(FW, FW, 0);
                    if c.g[V0][0] != 0 {
                        // k > jp
                        kernel_rem_pio2f_fq_next(c);
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    let more = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, T1) << 2);
                    if !more {
                        // k > jz - i
                        c.cycles += 2;
                        c.g[A2][0] = c.g[A1][0];
                        break;
                    }
                    kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::FQ_DOT);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        let more = (c.g[A2][0] as i64) >= 0;
        c.swc1(FW, r32(c, V0));
        if !more {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::FQ_TERM);
    }
    c.cycles += 1;
    c.g[V1][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3));
}

/// `fw += fq[i]` for i = a2 down to 0 (a3 = fq).
pub(crate) fn kernel_rem_pio2f_sum_down(c: &mut Cpu, head: u32) {
    use kernel_rem_pio2f_reg::*;
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        c.lwc1(0, r32(c, V0));
        c.add_s(FW, FW, 0);
        if (c.g[A2][0] as i64) < 0 {
            break;
        }
        kernel_rem_pio2f_back_edge(c, head);
    }
}

/// `switch (prec)`: y[] from fq[] (negated when ih != 0), v0 = n & 7, and
/// ra reloaded for the epilogue.
pub(crate) fn kernel_rem_pio2f_fold(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let ra_at = kernel_rem_pio2f_at(c, slot::RA);
    c.cycles += 2;
    c.g[A0][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC));
    if c.g[V1][0] == 0 {
        // prec >= 3
        c.cycles += 4;
        c.g[A1][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC));
        c.g[V0][0] = 3;
        let three = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[JX][0];
        if !three {
            c.cycles += 2;
            c.g[RA][0] = c.ld(ra_at);
            return;
        }
        kernel_rem_pio2f_fold3(c);
    } else {
        c.cycles += 2;
        if (c.g[A0][0] as i64) > 0 {
            c.g[A2][0] = c.g[JZ][0];
            kernel_rem_pio2f_fold12(c);
        } else {
            c.cycles += 2;
            let zero = c.g[A0][0] == 0;
            c.g[V0][0] = c.g[JX][0];
            if !zero {
                c.cycles += 2;
                c.g[RA][0] = c.ld(ra_at);
                return;
            }
            kernel_rem_pio2f_fold0(c);
        }
    }
    c.cycles += 1;
    c.g[RA][0] = c.ld(ra_at);
}

/// prec 0: y[0] = the sum of fq[jz..0].
pub(crate) fn kernel_rem_pio2f_fold0(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let y = r32(c, Y);
    c.cycles += 4;
    c.g[A2][0] = c.g[JZ][0];
    c.mtc1(ZERO, FW);
    if (c.g[A2][0] as i64) < 0 {
        c.swc1(FW, y);
    } else {
        c.cycles += 2;
        c.g[A3][0] = sx(kernel_rem_pio2f_at(c, slot::FQ));
        kernel_rem_pio2f_sum_down(c, kernel_rem_pio2f_head::SUM0);
        c.cycles += 1;
        c.swc1(FW, y);
    }
    c.cycles += 3;
    c.g[A2][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    let ih = c.g[A2][0] != 0;
    c.g[V0][0] = c.g[JX][0];
    if ih {
        c.cycles += 3;
        c.neg_s(0, FW);
        c.swc1(0, y);
    }
}

/// prec 1 and 2: y[0] = the sum of fq[jz..0], y[1] = (fq[0] - y[0]) plus
/// fq[1..=jz] (a2 = jz).
pub(crate) fn kernel_rem_pio2f_fold12(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let y = r32(c, Y);
    let fq = kernel_rem_pio2f_at(c, slot::FQ);
    c.cycles += 3;
    c.mtc1(ZERO, FW);
    let empty = (c.g[A2][0] as i64) < 0;
    c.lwc1(3, fq);
    if empty {
        c.cycles += 2;
        c.g[V1][0] = ((c.g[A2][0] as i64) < 1) as u64;
        c.swc1(FW, y);
    } else {
        c.cycles += 3;
        c.g[A3][0] = sx(fq);
        c.g[V1][0] = ((c.g[A2][0] as i64) < 1) as u64;
        kernel_rem_pio2f_sum_down(c, kernel_rem_pio2f_head::SUM12);
        c.cycles += 2;
        c.swc1(FW, y);
    }
    c.cycles += 3;
    c.g[T0][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    if c.g[T0][0] != 0 {
        c.cycles += 3;
        c.neg_s(0, FW);
        c.swc1(0, y);
    }
    c.sub_s(FW, 3, FW);
    c.cycles += 2;
    c.g[A2][0] = 1;
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[A3][0] = sx(fq);
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.lwc1(0, r32(c, V0));
            c.g[V1][0] = ((c.g[JZ][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.add_s(FW, FW, 0);
            if c.g[V1][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::SUM12_TAIL);
        }
    }
    c.cycles += 4;
    c.swc1(FW, y.wrapping_add(4));
    c.g[V0][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    let ih = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[JX][0];
    if ih {
        c.cycles += 3;
        c.neg_s(0, FW);
        c.swc1(0, y.wrapping_add(4));
    }
}

/// One renormalising step of prec 3 at i (a2, a3 = fq): `fw = fq[i-1] +
/// fq[i]; fq[i] += fq[i-1] - fw; fq[i-1] = fw`; a2 = i - 1.
pub(crate) fn kernel_rem_pio2f_fold3_step(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.g[A0][0] = sx(r32(c, A2).wrapping_sub(1));
    c.g[V0][0] = sx(r32(c, A2) << 2);
    c.g[V1][0] = sx(r32(c, A0) << 2);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
    c.lwc1(1, r32(c, V0)); // fq[i]
    c.lwc1(0, r32(c, V1)); // fq[i - 1]
    c.g[A2][0] = c.g[A0][0];
    c.add_s(FW, 0, 1);
    c.sub_s(0, 0, FW);
    c.add_s(1, 1, 0);
    c.swc1(1, r32(c, V0));
    c.swc1(FW, r32(c, V1));
}

/// prec 3: two renormalising passes over fq[] (down to i = 1, then 2),
/// then y = (fq[0], fq[1], the sum of fq[jz..2]).
pub(crate) fn kernel_rem_pio2f_fold3(c: &mut Cpu) {
    use kernel_rem_pio2f_head as head;
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let y = r32(c, Y);
    let fq = kernel_rem_pio2f_at(c, slot::FQ);
    c.cycles += 3;
    c.g[A2][0] = c.g[JZ][0];
    let skip = (c.g[A2][0] as i64) <= 0;
    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    if !skip {
        c.cycles += 2;
        c.g[A3][0] = sx(fq);
        loop {
            c.cycles += 14;
            kernel_rem_pio2f_fold3_step(c);
            if (c.g[A2][0] as i64) <= 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::FOLD3_PASS1);
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[JZ][0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[A2][0] = c.g[JZ][0];
    } else {
        c.cycles += 2;
        c.g[A3][0] = sx(fq);
        loop {
            c.cycles += 15;
            kernel_rem_pio2f_fold3_step(c);
            c.g[A1][0] = ((c.g[A2][0] as i64) < 2) as u64;
            if c.g[A1][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::FOLD3_PASS2);
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[JZ][0];
    }
    // fw = fq[jz] + ... + fq[2]
    c.cycles += 4;
    c.mtc1(ZERO, FW);
    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    let short = c.g[V0][0] != 0;
    c.lwc1(3, fq);
    c.cycles += 2;
    if short {
        c.lwc1(1, fq.wrapping_add(4));
    } else {
        c.g[A3][0] = sx(fq);
        c.lwc1(1, fq.wrapping_add(4));
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.lwc1(0, r32(c, V0));
            c.g[V1][0] = ((c.g[A2][0] as i64) < 2) as u64;
            c.add_s(FW, FW, 0);
            if c.g[V1][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::SUM3);
        }
        c.cycles += 2;
    }
    c.g[V1][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.neg_s(0, 3);
        c.cycles += 5;
        c.neg_s(1, 1);
        c.neg_s(FW, FW);
        c.swc1(0, y);
        c.swc1(1, y.wrapping_add(4));
        c.swc1(FW, y.wrapping_add(8));
    } else {
        c.cycles += 4;
        c.swc1(FW, y.wrapping_add(8));
        c.swc1(3, y);
        c.swc1(1, y.wrapping_add(4));
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[JX][0];
}

/// Restore the saved registers and return.
pub(crate) fn kernel_rem_pio2f_epilogue(c: &mut Cpu) {
    c.cycles += 12;
    for (reg, off) in KERNEL_REM_PIO2F_SAVED {
        c.g[reg][0] = c.ld(kernel_rem_pio2f_at(c, off));
    }
    c.lwc1(20, kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::F20));
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(kernel_rem_pio2f_slot::SIZE));
    c.pc = ra;
}

/// `libmpeg_SetupPredBlock` (`0x0040_4758`).
pub(crate) const FP_SETUP_PRED_BLOCK: Callee = Callee {
    addr: 0x0040_4758,
    f: ee::s15::f_00404758,
};

/// `libmpeg_DualPrimeArithmetic` (`0x0040_5e30`).
pub(crate) const FP_DUAL_PRIME: Callee = Callee {
    addr: 0x0040_5e30,
    f: ee::s15::f_00405e30,
};

/// `libmpeg_Errorf` (`0x0040_3470`).
pub(crate) const FP_ERRORF: Callee = Callee {
    addr: 0x0040_3470,
    f: ee::s15::f_00403470,
};

/// `libmpeg_FormPredictions(dec, bx, by, mbtype, mtype, PMV, mvfs, dmv)`
/// (0x404050): MPEG-2 motion compensation of one macroblock. `dec` (a0) is the
/// decoder state, `bx`/`by` (a1/a2) the block position, `mbtype` (a3) the
/// macroblock type flags (bit 3 forward, bit 2 backward), `mtype` (t0) the
/// motion type (1 field, 2 frame, 3 dual prime), `PMV` (t1) the motion
/// vectors, `mvfs` (t2) the field selects, `dmv` (t3) the dual-prime deltas.
/// Clears the per-buffer flag at `dec + dec[0x810] * 0x140 + 0x6bc`, then
/// makes one or more `libmpeg_SetupPredBlock` calls for the forward and the
/// backward prediction (`Errorf` on an unknown motion type), averaging the
/// second one in.
pub fn lib_mpeg_form_predictions(c: &mut Cpu) {
    guest(c, |c| {
        'b_404538: {
            'b_4040cc: {
                c.cycles += 27;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                c.g[V1][0] = 0x140;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[FP][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[S6][0]);
                c.g[FP][0] = c.g[T2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
                c.g[S6][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S4][0]);
                c.g[S5][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S3][0]);
                c.g[S4][0] = c.g[T0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S2][0]);
                c.g[S3][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S1][0]);
                c.g[S2][0] = c.g[T1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                c.g[S0][0] = 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[S7][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x810));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                c.mult(0, 4, 2, 3);
                c.g[T4][0] = c.g[A3][0] & 0x8;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                let t = c.g[T4][0] != 0u64;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6bc), 0u64);
                if t {
                    break 'b_4040cc;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_404538;
                    } else {
                        break 'b_4040cc;
                    }
                }
            }
            'b_404534: {
                'b_404520: {
                    'b_404510: {
                        'b_4042e4: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                            c.g[V0][0] = 0x3;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = c.g[V1][0] ^ 0x2;
                            if t {
                                break 'b_4042e4;
                            } else {
                                'b_404128: {
                                    'b_4040f4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        if c.g[S4][0] == c.g[V0][0] {
                                            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                            break 'b_4040f4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[T4][0] != 0u64;

                                            if t {
                                                break 'b_404128;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                                break 'b_4040f4;
                                            }
                                        }
                                    }
                                    c.cycles += 13;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = 0u64;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                    c.g[A3][0] = 0u64;
                                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                    c.g[T0][0] = 0u64;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.g[T1][0] = 0x10;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                    c.g[T2][0] = c.g[S5][0];
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                    break 'b_404510;
                                }
                                'b_4041b4: {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] != c.g[S3][0];
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_4041b4;
                                    } else {
                                        c.cycles += 33;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                        c.g[A3][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.g[A2][0] = c.lw(c.g[FP][0] as u32);
                                        c.g[T0][0] = 0u64;
                                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        FP_SETUP_PRED_BLOCK.call(c, 0x0040416c)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A3][0] = 0x1;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                                        c.g[T0][0] = 0u64;
                                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        FP_SETUP_PRED_BLOCK.call(c, 0x004041ac)?;
                                        c.g[S0][0] = 0x1;
                                        break 'b_404534;
                                    }
                                }
                                'b_4042cc: {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] != c.g[V1][0];
                                    c.g[A1][0] = 0x490000;
                                    if t {
                                        break 'b_4042cc;
                                    } else {
                                        c.cycles += 68;
                                        c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A2][0] = c.g[T3][0];
                                        c.g[A3][0] = c.lw(c.g[S2][0] as u32);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                        FP_DUAL_PRIME.call(c, 0x004041d8)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                        c.g[A2][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                        c.g[A3][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.g[T3][0] = c.g[S6][0];
                                        FP_SETUP_PRED_BLOCK.call(c, 0x00404218)?;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.g[A2][0] = 0x1;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[A3][0] = 0u64;
                                        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        FP_SETUP_PRED_BLOCK.call(c, 0x00404254)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                        c.g[A2][0] = 0x1;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                        c.g[A3][0] = 0x1;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.g[T3][0] = c.g[S6][0];
                                        FP_SETUP_PRED_BLOCK.call(c, 0x00404294)?;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                        c.g[A2][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[A3][0] = 0x1;
                                        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        break 'b_404510;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e38));
                                c.g[A2][0] = c.g[S4][0];
                                FP_ERRORF.call(c, 0x004042dc)?;
                                c.g[S0][0] = 0x1;
                                break 'b_404534;
                            }
                        }
                        'b_404334: {
                            'b_404330: {
                                c.cycles += 13;
                                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c8));
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d8));
                                c.g[S7][0] = (c.g[V0][0] < 0x1u64) as u64;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                c.g[A2][0] = 0x2;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                c.g[S3][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[A1][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                                let t = c.g[V0][0] != c.g[A2][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                                if t {
                                    break 'b_404330;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x120));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_404334;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                                        c.g[V0][0] = c.g[S7][0] ^ c.g[V0][0];
                                        c.g[S3][0] = (0u64 < c.g[V0][0]) as u64;
                                        break 'b_404330;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_404334;
                        }
                        'b_404504: {
                            'b_404384: {
                                'b_404348: {
                                    c.cycles += 2;
                                    if c.g[S4][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                                        break 'b_404348;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[T4][0] != 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_404384;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                                            break 'b_404348;
                                        }
                                    }
                                }
                                c.cycles += 15;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.sw(c.g[SP][0] as u32, c.g[A2][0]);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V1][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                break 'b_404504;
                            }
                            'b_404454: {
                                c.cycles += 2;
                                let t = c.g[S4][0] != c.g[V0][0];
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_404454;
                                } else {
                                    'b_40440c: {
                                        'b_404408: {
                                            c.cycles += 24;
                                            c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                                            c.g[A1][0] = sx((c.g[S3][0] as u32) << 3);
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[A3][0] = 0u64;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                            c.g[T0][0] = 0u64;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                            c.g[T1][0] = 0x8;
                                            c.g[T2][0] = c.g[S5][0];
                                            c.g[T3][0] = c.g[S6][0];
                                            FP_SETUP_PRED_BLOCK.call(c, 0x004043e0)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                            let t = c.g[V0][0] != c.g[S4][0];
                                            c.g[S3][0] = 0u64;
                                            if t {
                                                break 'b_404408;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x120));
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] =
                                                    c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_40440c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[S3][0] = 0x1;
                                                    c.g[V0][0] = c.g[S7][0] ^ c.g[V1][0];
                                                    if c.g[V0][0] == 0 {
                                                        c.g[S3][0] = 0u64;
                                                    }
                                                    break 'b_40440c;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                                        break 'b_40440c;
                                    }
                                    c.cycles += 18;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[A3][0] = 0u64;
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[T0][0] = 0x8;
                                    c.sw(c.g[SP][0] as u32, c.g[A2][0]);
                                    c.g[T1][0] = 0x8;
                                    c.g[A2][0] = 0u64;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                    c.g[T2][0] = c.g[S5][0];
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                    break 'b_404510;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S4][0] != c.g[V0][0];
                            c.g[A1][0] = 0x490000;
                            if t {
                                break 'b_404520;
                            } else {
                                c.cycles += 42;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x120));
                                c.g[S3][0] = 0x1;
                                c.g[A3][0] = c.lw(c.g[S2][0] as u32);
                                c.g[A2][0] = c.g[T3][0];
                                c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                if c.g[V0][0] == 0 {
                                    c.g[S3][0] = 0u64;
                                }
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                FP_DUAL_PRIME.call(c, 0x00404480)?;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                                c.g[A3][0] = 0u64;
                                c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                c.g[T0][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[T1][0] = 0x10;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                c.g[T2][0] = c.g[S5][0];
                                c.g[T3][0] = c.g[S6][0];
                                FP_SETUP_PRED_BLOCK.call(c, 0x004044c8)?;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                if c.g[S7][0] != 0 {
                                    c.g[V0][0] = c.g[V1][0];
                                }
                                c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] = 0x1;
                                c.g[A0][0] = c.g[S1][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[A3][0] = 0u64;
                                break 'b_404504;
                            }
                        }
                        c.cycles += 3;
                        c.g[T0][0] = 0u64;
                        c.g[T1][0] = 0x10;
                        c.g[T2][0] = c.g[S5][0];
                        break 'b_404510;
                    }
                    c.cycles += 4;
                    c.g[T3][0] = c.g[S6][0];
                    FP_SETUP_PRED_BLOCK.call(c, 0x00404518)?;
                    c.g[S0][0] = 0x1;
                    break 'b_404534;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e58));
                c.g[A2][0] = c.g[S4][0];
                FP_ERRORF.call(c, 0x00404530)?;
                c.g[S0][0] = 0x1;
                break 'b_404534;
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_404538;
        }
        'b_404728: {
            'b_404724: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0x4;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x3;
                if t {
                    break 'b_404724;
                } else {
                    'b_40461c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_40461c;
                        } else {
                            'b_40460c: {
                                'b_404594: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[S4][0] != c.g[V0][0];
                                    c.g[S3][0] = 0x1;
                                    if t {
                                        break 'b_404594;
                                    } else {
                                        c.cycles += 14;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                        c.g[A2][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                        c.g[A3][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                        c.g[T0][0] = 0u64;
                                        c.g[T1][0] = 0x10;
                                        break 'b_40460c;
                                    }
                                }
                                c.cycles += 30;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A3][0] = 0u64;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1bc));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                                c.g[T0][0] = 0u64;
                                c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                c.g[T1][0] = 0x8;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[T2][0] = c.g[S5][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                c.g[T3][0] = c.g[S6][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                FP_SETUP_PRED_BLOCK.call(c, 0x004045d4)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                c.g[T2][0] = c.g[S5][0];
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                                c.g[T3][0] = c.g[S6][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[A3][0] = 0x1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                c.g[T0][0] = 0u64;
                                c.g[T1][0] = 0x8;
                                c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
                                break 'b_40460c;
                            }
                            c.cycles += 4;
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
                            FP_SETUP_PRED_BLOCK.call(c, 0x00404614)?;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                            break 'b_404728;
                        }
                    }
                    'b_404674: {
                        c.cycles += 2;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_404674;
                        } else {
                            'b_404634: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                    break 'b_404634;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                    break 'b_404634;
                                }
                            }
                            c.cycles += 16;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[T2][0] = c.g[S5][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[T3][0] = c.g[S6][0];
                            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                            c.g[A3][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0x10;
                            FP_SETUP_PRED_BLOCK.call(c, 0x0040466c)?;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                            break 'b_404728;
                        }
                    }
                    'b_404714: {
                        c.cycles += 2;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[A1][0] = 0x490000;
                        if t {
                            break 'b_404714;
                        } else {
                            'b_40468c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                    break 'b_40468c;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                    break 'b_40468c;
                                }
                            }
                            'b_4046d4: {
                                c.cycles += 17;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                c.g[A2][0] = 0u64;
                                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[T0][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[T1][0] = 0x8;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                c.g[T2][0] = c.g[S5][0];
                                c.g[T3][0] = c.g[S6][0];
                                FP_SETUP_PRED_BLOCK.call(c, 0x004046c4)?;
                                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                    break 'b_4046d4;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                    break 'b_4046d4;
                                }
                            }
                            c.cycles += 16;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                            c.g[T2][0] = c.g[S5][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[T3][0] = c.g[S6][0];
                            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                            c.g[A3][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                            c.g[T0][0] = 0x8;
                            c.g[T1][0] = 0x8;
                            FP_SETUP_PRED_BLOCK.call(c, 0x0040470c)?;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                            break 'b_404728;
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e78));
                    c.g[A2][0] = c.g[S4][0];
                    FP_ERRORF.call(c, 0x00404724)?;
                    break 'b_404724;
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
            break 'b_404728;
        }
        c.cycles += 11;
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        Ok(())
    })
}

/// `int libmpeg_GetPictureCommon(Mpeg *m)` (0x00402e10): the MPEG picture
/// pump. With `d = m->dec` (+0x40) it clears `d[0]` and requires
/// `d[54] & 0x3f == 0` (else `libmpeg_Errorf`, returns -1); then loops: read
/// headers (`libmpeg_GetHeader`) until the picture type is 0, the header
/// counters (`d[93]`, `d[53]`) agree or `d[530]` is clear, and dispatch on the
/// type (0: flush the pending picture and set `d[0]`; 1/2/3/4: decode a
/// picture into the I/P/B slot counters `d[40..42]`). Returns 1 once `d[520]`
/// or `d[0]` is set. Converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn libmpeg_get_picture_common(c: &mut Cpu) {
    if c.pc != 0x00402e10 {
        return ee::s15::f_00402e10__generated(c);
    }
    guest(c, |c| {
        'b_402f64: {
            'b_402e60: {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.g[S3][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.g[S2][0] = 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                c.g[V0][0] = c.g[A2][0] & 0x3f;
                let t = c.g[V0][0] == 0u64;
                c.sw(c.g[S0][0] as u32, 0u64);
                if t {
                    break 'b_402e60;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00402e58);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7c68));
                    Callee {
                        addr: 0x00403470,
                        f: ee::s15::libmpeg_Errorf,
                    }
                    .call(c, 0x00402e58)?;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_402f64;
                    } else {
                        break 'b_402e60;
                    }
                }
            }
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x820), 0u64);
            c.g[V0][0] = 0xffffffffffffffff;
            'l_402e68: loop {
                'b_402ea0: {
                    c.cycles += 2;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                    if t {
                        break 'b_402ea0;
                    } else {
                        'l_402e70: loop {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x00402e78);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x00407520,
                                f: ee::s15::libmpeg_GetHeader,
                            }
                            .call(c, 0x00402e78)?;
                            c.g[S3][0] = c.g[V0][0];
                            let t = c.g[S3][0] == 0u64;
                            c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                            if t {
                                break 'b_402ea0;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd4));
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                                if t {
                                    break 'b_402ea0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00402e70);
                                        }
                                        continue 'l_402e70;
                                    } else {
                                        break 'b_402ea0;
                                    }
                                }
                            }
                        }
                    }
                }
                'b_402f48: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        break 'b_402f48;
                    } else {
                        'b_402f28: {
                            'b_402f04: {
                                'b_402ed4: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7cb0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x00402ec0 => {
                                            c.cycles += 5;
                                            c.g[RA][0] = sx(0x00402ec8);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x00403240,
                                                f: ee::s15::libmpeg_FlushPendingPicture,
                                            }
                                            .call(c, 0x00402ec8)?;
                                            c.g[V1][0] = 0x1;
                                            let t = true;
                                            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                                            if t {
                                                break 'b_402f48;
                                            } else {
                                                break 'b_402ed4;
                                            }
                                        }
                                        0x00402ed4 => {
                                            break 'b_402ed4;
                                        }
                                        0x00402f04 => {
                                            break 'b_402f04;
                                        }
                                        0x00402f28 => {
                                            break 'b_402f28;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x00402e10, 0x00402f80);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 12;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), 0u64);
                                c.g[A0][0] = c.g[S1][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), 0u64);
                                c.g[A1][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), 0u64);
                                c.g[RA][0] = sx(0x00402ef0);
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x94));
                                Callee {
                                    addr: 0x00403098,
                                    f: ee::s15::libmpeg_DecodePicture,
                                }
                                .call(c, 0x00402ef0)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                                if t {
                                    break 'b_402f48;
                                } else {
                                    break 'b_402f04;
                                }
                            }
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00402f14);
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                            Callee {
                                addr: 0x00403098,
                                f: ee::s15::libmpeg_DecodePicture,
                            }
                            .call(c, 0x00402f14)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V1][0]);
                            if t {
                                break 'b_402f48;
                            } else {
                                break 'b_402f28;
                            }
                        }
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00402f38);
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                        Callee {
                            addr: 0x00403098,
                            f: ee::s15::libmpeg_DecodePicture,
                        }
                        .call(c, 0x00402f38)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
                        break 'b_402f48;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x820));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_402f64;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00402e68);
                        }
                        continue 'l_402e68;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x1;
                        break 'b_402f64;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void libmpeg_AddPredAndStore(Mpeg *m, int blk)` (0x00404b78): MPEG block
/// reconstruction. Runs the two per-component function-pointer tables of
/// block `blk` (stride 0x140) `count` times, reports an error if the block
/// has both flags set (`libmpeg_Error`), then adds the prediction (second
/// source) to the 24 residual quad-words (or just clamps them, without a
/// prediction) with packed halfword add/min/max and stores them as packed
/// bytes. Converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn libmpeg_add_pred_and_store(c: &mut Cpu) {
    if c.pc != 0x00404b78 {
        return ee::s15::f_00404b78__generated(c);
    }
    guest(c, |c| {
        'b_404c7c: {
            'b_404c78: {
                'b_404c6c: {
                    'b_404c5c: {
                        c.cycles += 19;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                        c.g[V0][0] = 0x140;
                        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                        c.mult(0, 5, 5, 2);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
                        c.g[S4][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c8));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6bc));
                        if t {
                            break 'b_404c5c;
                        } else {
                            c.cycles += 5;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) <= 0;
                            c.g[S5][0] = 0u64;
                            if t {
                                break 'b_404c6c;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
                                c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5b8));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c8));
                                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b8));
                                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                'l_404bf0: loop {
                                    c.cycles += 25;
                                    c.g[S1][0] = 0x140;
                                    c.g[S3][0] = 0x1c;
                                    c.g[S0][0] = sx((c.g[S5][0] as u32) << 2);
                                    c.mult(0, 17, 2, 17);
                                    c.mult(1, 19, 21, 19);
                                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x590));
                                    c.g[V0][0] =
                                        sx((c.g[S7][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                    c.g[S2][0] =
                                        sx((c.g[S4][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00404c28);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00404c28)?;
                                    c.g[S0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xb8));
                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00404c3c);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    call_ptr(c, t, 0x00404c3c)?;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[S1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00404bf0);
                                        }
                                        continue 'l_404bf0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                        if t {
                                            break 'b_404c7c;
                                        } else {
                                            break 'b_404c5c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
                    c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b8));
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        break 'b_404c78;
                    } else {
                        break 'b_404c6c;
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b8));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_404c78;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            break 'b_404c7c;
        }
        'b_404cb0: {
            c.cycles += 7;
            c.g[V0][0] = 0x140;
            c.mult(0, 4, 3, 2);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if t {
                break 'b_404cb0;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6cc));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = 0x490000;
                if t {
                    break 'b_404cb0;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00404cb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e98));
                    Callee {
                        addr: 0x004034a8,
                        f: ee::s15::libmpeg_Error,
                    }
                    .call(c, 0x00404cb0)?;
                    break 'b_404cb0;
                }
            }
        }
        'b_405ae8: {
            c.cycles += 8;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = 0x140;
            c.mult(0, 5, 3, 2);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if t {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6cc));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if t {
                    c.cycles += 16;
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x81c));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x594));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.cycles += 4;
                    c.g[T4][0] = 0x18;
                    c.g[T2][0] = 0x400000;
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5b30));
                    c.lq(11, c.g[T2][0] as u32);
                    'l_405a98: loop {
                        c.cycles += 17;
                        c.lq(8, c.g[A1][0] as u32);
                        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
                        c.lq(13, c.g[A2][0] as u32);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.lq(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.paddh(8, 8, 13);
                        c.lq(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.pminh(8, 8, 11);
                        c.paddh(9, 9, 2);
                        c.pmaxh(8, 8, 0);
                        c.pminh(9, 9, 11);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        c.pmaxh(9, 9, 0);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        c.ppacb(10, 9, 8);
                        let t = c.g[T4][0] != 0u64;
                        c.sq(10, (c.g[A0][0] as u32).wrapping_sub(0x10));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00405a98);
                            }
                            continue 'l_405a98;
                        } else {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.pc = t;
                            return Ok(());
                        }
                    }
                } else {
                    c.cycles += 14;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x81c));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    break 'b_405ae8;
                }
            } else {
                c.cycles += 16;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x594));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                break 'b_405ae8;
            }
        }
        c.cycles += 4;
        c.g[T4][0] = 0x18;
        c.g[T2][0] = 0x400000;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5b30));
        c.lq(11, c.g[T2][0] as u32);
        'l_405af8: loop {
            c.cycles += 12;
            c.lq(8, c.g[A1][0] as u32);
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
            c.pminh(8, 8, 11);
            c.lq(9, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.pmaxh(8, 8, 0);
            c.pminh(9, 9, 11);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            c.pmaxh(9, 9, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            c.ppacb(10, 9, 8);
            let t = c.g[T4][0] != 0u64;
            c.sq(10, (c.g[A0][0] as u32).wrapping_sub(0x10));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00405af8);
                }
                continue 'l_405af8;
            } else {
                c.cycles += 8;
                let t = c.g[RA][0] as u32;
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `libmpeg_SequenceHeader(dec)` (0x403620): parses an MPEG sequence header
/// from the bitstream: width/height (`dec+0x124/0x128`, `Error` when the width
/// field is 2801 or more), frame-rate and bit-rate words (`+0x134/0x138`), then
/// the intra and non-intra quantiser matrices (loaded from the stream through
/// the IPU, or the default tables), and the extension/user data. Then it
/// falls into the frame-buffer set-up of the decoder context at
/// `dec[0x858]->[0x40]`: macroblock dimensions (`+0x12c/0x130`) and, when the
/// picture size changed, the work-area allocations, `SetupFrameBuffers` and
/// the `SetPictureSize` calls (the last one a tail call).
pub fn lib_mpeg_sequence_header(c: &mut Cpu) {
    guest(c, |c| {
        'b_40366c: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[A1][0] = 0x20;
            c.sd(c.g[SP][0] as u32, c.g[S0][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x0040363c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xd4), 0u64);
            Callee {
                addr: 0x004072e8,
                f: ee::s15::libmpeg_GetBits,
            }
            .call(c, 0x0040363c)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            c.g[V0][0] = c.g[V0][0] & 0xfff;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x124), c.g[V1][0]);
            c.g[A0][0] = ((c.g[V0][0] as i64) < 2801i64) as u64;
            let t = c.g[A0][0] != 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x128), c.g[V0][0]);
            if t {
                break 'b_40366c;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0040366c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7cf8));
                Callee {
                    addr: 0x004034a8,
                    f: ee::s15::libmpeg_Error,
                }
                .call(c, 0x0040366c)?;
                break 'b_40366c;
            }
        }
        'b_4036e0: {
            'b_4036c8: {
                c.cycles += 14;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00403678);
                c.g[A1][0] = 0x1e;
                Callee {
                    addr: 0x004072e8,
                    f: ee::s15::libmpeg_GetBits,
                }
                .call(c, 0x00403678)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
                c.g[A1][0] = 0x1;
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 12);
                c.g[V0][0] = c.g[V0][0] & 0x3ff;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x134), c.g[V1][0]);
                c.g[RA][0] = sx(0x0040369c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x138), c.g[V0][0]);
                Callee {
                    addr: 0x004072e8,
                    f: ee::s15::libmpeg_GetBits,
                }
                .call(c, 0x0040369c)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x840), c.g[V0][0]);
                if t {
                    break 'b_4036c8;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x004036ac);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x004036ac)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x004036b8);
                    c.g[A1][0] = 0x50000000;
                    Callee {
                        addr: 0x00406df0,
                        f: ee::s15::libmpeg_IpuCommand,
                    }
                    .call(c, 0x004036b8)?;
                    c.g[RA][0] = sx(0x004036c0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x004036c0)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_4036e0;
                    } else {
                        break 'b_4036c8;
                    }
                }
            }
            c.cycles += 6;
            c.g[A2][0] = 0x450000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3200));
            c.g[RA][0] = sx(0x004036dc);
            c.g[A1][0] = 0x50000000;
            Callee {
                addr: 0x00403ad0,
                f: ee::s15::libmpeg_LoadIqMatrix,
            }
            .call(c, 0x004036dc)?;
            c.g[A0][0] = c.g[S0][0];
            break 'b_4036e0;
        }
        'b_403728: {
            'b_403714: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x004036e8);
                c.g[A1][0] = 0x1;
                Callee {
                    addr: 0x004072e8,
                    f: ee::s15::libmpeg_GetBits,
                }
                .call(c, 0x004036e8)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x844), c.g[V0][0]);
                if t {
                    break 'b_403714;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x004036f8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x004036f8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00403704);
                    c.g[A1][0] = 0x58000000;
                    Callee {
                        addr: 0x00406df0,
                        f: ee::s15::libmpeg_IpuCommand,
                    }
                    .call(c, 0x00403704)?;
                    c.g[RA][0] = sx(0x0040370c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x0040370c)?;
                    let t = true;

                    if t {
                        break 'b_403728;
                    } else {
                        break 'b_403714;
                    }
                }
            }
            c.cycles += 5;
            c.g[A2][0] = 0x450000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3240));
            c.g[RA][0] = sx(0x00403728);
            c.g[A1][0] = 0x58000000;
            Callee {
                addr: 0x00403ad0,
                f: ee::s15::libmpeg_LoadIqMatrix,
            }
            .call(c, 0x00403728)?;
            break 'b_403728;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x00403730);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x004076f8,
            f: ee::s15::libmpeg_ExtensionAndUserData,
        }
        .call(c, 0x00403730)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        'b_4037b0: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
            c.g[A1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[S7][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[S6][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[FP][0]);
            c.g[FP][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x848));
            if c.g[A2][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x124));
                break 'b_4037b0;
            } else {
                c.cycles += 10;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x5;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x174), c.g[V1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x188), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x124));
                break 'b_4037b0;
            }
        }
        'b_4037e8: {
            'b_4037e0: {
                'b_4037dc: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    let t = c.g[A2][0] == 0u64;
                    c.sw((c.g[FP][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
                    if t {
                        break 'b_4037dc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x13c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x128));
                        if t {
                            break 'b_4037e0;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f));
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                            let t = true;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            if t {
                                break 'b_4037e8;
                            } else {
                                break 'b_4037dc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x128));
                break 'b_4037e0;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
            break 'b_4037e8;
        }
        'b_403810: {
            c.cycles += 7;
            c.sw((c.g[FP][0] as u32).wrapping_add(0x130), c.g[V0][0]);
            c.g[S6][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x12c));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[S7][0] = sx((c.g[V0][0] as u32) << 4);
            let t = c.g[S7][0] != c.g[V1][0];
            c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x528));
            if t {
                break 'b_403810;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                let t = c.g[S6][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x528));
                if t {
                    c.cycles += 12;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.pc = t;
                    return Ok(());
                } else {
                    break 'b_403810;
                }
            }
        }
        c.cycles += 108;
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S6][0]);
        c.g[S0][0] = 0x180;
        c.sw(c.g[A1][0] as u32, c.g[S7][0]);
        c.mult(0, 16, 22, 16);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x108));
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x320));
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
        c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1e8));
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x388));
        c.mult(0, 16, 23, 16);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.g[S4][0] = sx((c.g[FP][0] as u32).wrapping_add(0x250));
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3f0));
        c.g[S5][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2b8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S2][0] = sx(((c.g[S6][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x458));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4c0));
        c.g[RA][0] = sx(0x00403870);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        Callee {
            addr: 0x00402d78,
            f: ee::s15::libmpeg_WorkAreaRestore,
        }
        .call(c, 0x00403870)?;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00403884);
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x00402d88,
            f: ee::s15::libmpeg_WorkAreaAlloc,
        }
        .call(c, 0x00403884)?;
        c.sw((c.g[FP][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0040389c);
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x00402d88,
            f: ee::s15::libmpeg_WorkAreaAlloc,
        }
        .call(c, 0x0040389c)?;
        c.sw((c.g[FP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[FP][0];
        c.g[RA][0] = sx(0x004038b4);
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x00402d88,
            f: ee::s15::libmpeg_WorkAreaAlloc,
        }
        .call(c, 0x004038b4)?;
        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[A0][0] = c.g[S3][0];
        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
        c.g[A1][0] = c.g[S4][0];
        c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
        c.g[A2][0] = c.g[S5][0];
        c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xfc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x100));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x104));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S7][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[S6][0]);
        c.g[RA][0] = sx(0x00403904);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        Callee {
            addr: 0x004039f0,
            f: ee::s15::libmpeg_SetupFrameBuffers,
        }
        .call(c, 0x00403904)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403914);
        c.g[A2][0] = c.g[S6][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403914)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403924);
        c.g[A2][0] = c.g[S6][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403924)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403934);
        c.g[A2][0] = c.g[S6][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403934)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403944);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403944)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403954);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403954)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403964);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403964)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403974);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403974)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403984);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403984)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = 0x00403600;
        ee::s15::libmpeg_SetPictureSize(c);
        return Ok(());
    });
}

/// `int libmpeg_SetupMacroblock(dec, mbAddr, b, flags, mtype)` (0x403e00): splits
/// the macroblock address by the picture width in macroblocks (`dec+0x12c`)
/// into column and row, waits for the IPU-side SPR DMA (`0x1000d400` bit 8),
/// and for a non-intra block (`flags & 1` clear) forms the predictions
/// (`Errorf` and 0 for a bad motion type 0 or above 3, else `FormPredictions`,
/// the DMA wait again, `FetchRefBlocks`). Then it records the per-buffer
/// entry of `dec[0x6b8..0x6c8 + cur * 0x140]`: done flag, two flags and the
/// macroblock's buffer address in the current frame (picture structure
/// `dec+0x174`: 3 frame, 2 the `+0x1e0` field buffer, else `+0x1d0`).
pub fn lib_mpeg_setup_macroblock(c: &mut Cpu) {
    guest(c, |c| {
        'b_403e50: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
            c.g[FP][0] = c.g[A2][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
            c.g[S5][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
            c.g[V1][0] = c.g[S5][0] & 0x1;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
            c.div(0, 5, 2);
            if c.g[V0][0] == 0u64 {
                break 'b_403e50;
            } else {
                break 'b_403e50;
            }
        }
        'b_404020: {
            'b_403f44: {
                'b_403eb0: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = c.hi[0];
                    c.g[S7][0] = c.lo[0];
                    c.g[S6][0] = c.g[V1][0];
                    c.g[A2][0] = sx((c.g[S7][0] as u32) << 4);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S6][0] as u32) << 4);
                    if t {
                        break 'b_403eb0;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c4));
                        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
                        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b8));
                        'l_403e80: loop {
                            c.cycles += 7;
                            c.g[V0][0] = 0x10000000;
                            c.g[V0][0] = c.g[V0][0] | 0xd400;
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                            c.g[V1][0] = c.g[V1][0] & 0x1;
                            let t = c.g[V1][0] != 0u64;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00403e80);
                                }
                                continue 'l_403e80;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x140;
                                c.mult(0, 3, 4, 2);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                let t = true;
                                c.sw(c.g[V0][0] as u32, 0u64);
                                if t {
                                    break 'b_403f44;
                                } else {
                                    break 'b_403eb0;
                                }
                            }
                        }
                    }
                }
                'b_403ee4: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_403ee4;
                    } else {
                        c.cycles += 9;
                        c.g[A1][0] = 0x490000;
                        c.g[A2][0] = c.g[T0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e10));
                        c.g[RA][0] = sx(0x00403ed4);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00403470,
                            f: ee::s15::libmpeg_Errorf,
                        }
                        .call(c, 0x00403ed4)?;
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = 0u64;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                        if t {
                            break 'b_404020;
                        } else {
                            break 'b_403ee4;
                        }
                    }
                }
                c.cycles += 9;
                c.g[RA][0] = sx(0x00403eec);
                c.g[A3][0] = c.g[S5][0];
                Callee {
                    addr: 0x00404050,
                    f: ee::s15::libmpeg_FormPredictions,
                }
                .call(c, 0x00403eec)?;
                c.g[V1][0] = 0x10000000;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c4));
                c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
                c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b8));
                c.g[V1][0] = c.g[V1][0] | 0xd400;
                'l_403f08: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00403f08);
                        }
                        continue 'l_403f08;
                    } else {
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x00403f2c);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00408988,
                            f: ee::s15::libmpeg_FetchRefBlocks,
                        }
                        .call(c, 0x00403f2c)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[V1][0] = 0x140;
                        c.g[A0][0] = 0x1;
                        c.mult(0, 5, 2, 3);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                        break 'b_403f44;
                    }
                }
            }
            'b_403f84: {
                'b_403f74: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[FP][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        break 'b_403f74;
                    } else {
                        'b_403f70: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S5][0] & 0x2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x140;
                            if t {
                                break 'b_403f70;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                                c.mult(0, 4, 2, 3);
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                let t = true;
                                c.sw(c.g[V0][0] as u32, c.g[FP][0]);
                                if t {
                                    break 'b_403f84;
                                } else {
                                    break 'b_403f70;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        break 'b_403f74;
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = 0x140;
                c.mult(0, 4, 2, 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw(c.g[V0][0] as u32, 0u64);
                break 'b_403f84;
            }
            'b_40401c: {
                'b_403fe0: {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                    c.g[A3][0] = 0x140;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = 0x3;
                    c.mult(0, 4, 2, 7);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_403fe0;
                    } else {
                        c.cycles += 13;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[A2][0] = 0x180;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                        c.mult(0, 3, 4, 7);
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.mult(0, 5, 22, 2);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                        c.mult(0, 2, 2, 6);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let t = true;
                        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                        if t {
                            break 'b_40401c;
                        } else {
                            break 'b_403fe0;
                        }
                    }
                }
                'b_403fec: {
                    c.cycles += 2;
                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d0));
                        break 'b_403fec;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e0));
                        break 'b_403fec;
                    }
                }
                c.cycles += 12;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = 0x180;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                c.g[A1][0] = 0x140;
                c.mult(0, 7, 22, 3);
                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.mult(0, 7, 4, 5);
                c.mult(0, 3, 3, 6);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                break 'b_40401c;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_404020;
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `libmpeg_SetupPredBlock(dec, ref, a2, a3, t0, h, bx, by, dx, dy, fld, avg)`
/// (0x404758): appends the luma and the chroma prediction block descriptors
/// for one motion vector (`dx`, `dy` in half pixels, at the stack) to the
/// current buffer entry of `dec` (`dec[0x6bc + cur * 0x140]` holds the count
/// `k`): block address inside the reference picture, the fraction-dependent
/// row split (`+8`/`+0xc` rows before/after the macroblock edge), buffer
/// pointers, and the interpolation-routine selectors from the tables at
/// `0x453280`/`0x4532a0`. Leaf, 8 stack arguments.
pub fn lib_mpeg_setup_pred_block(c: &mut Cpu) {
    'b_404818: {
        'b_404810: {
            c.cycles += 42;
            c.g[T8][0] = c.g[A0][0];
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(0x6bc));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[FP][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
            c.g[V1][0] = 0x140;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
            c.g[FP][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
            c.g[T6][0] = c.g[T1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S1][0] = c.g[A2][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[S0][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S7][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x81c));
            c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[A0][0] = 0x1c;
            c.g[T5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T2][0]);
            c.mult(0, 2, 2, 3);
            c.sw(c.g[SP][0] as u32, c.g[A3][0]);
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[T2][0] = sx(((c.g[S6][0] as u32) as i32 >> 1) as u32);
            c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb8));
            c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S7][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x590));
            c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.mult(0, 4, 23, 4);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
            c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let t = c.g[T5][0] == 0u64;
            c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if t {
                break 'b_404810;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                let t = true;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                if t {
                    break 'b_404818;
                } else {
                    break 'b_404810;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        break 'b_404818;
    }
    'b_4048e0: {
        'b_4048dc: {
            'b_4048a8: {
                c.cycles += 22;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                c.g[S4][0] = sx(((c.g[T0][0] as u32) as i32 >> 4) as u32);
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
                c.mult(0, 5, 20, 5);
                c.g[S5][0] = sx(((c.g[A2][0] as u32) as i32 >> 4) as u32);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                c.sw((c.g[T4][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[V0][0] = sx((c.g[S5][0] as u32) << 4);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.sw(c.g[T4][0] as u32, c.g[A0][0]);
                c.g[T9][0] = c.g[S2][0] & 0x1;
                let t = c.g[T9][0] == 0u64;
                c.g[S3][0] = c.g[S6][0] & 0x1;
                if t {
                    break 'b_4048a8;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[T6][0] as u32) << ((c.g[T5][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 16i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                        break 'b_4048dc;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = 0x10;
                        c.g[V1][0] =
                            sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] =
                            sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[T6][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = true;
                        c.sw((c.g[T4][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        if t {
                            break 'b_4048e0;
                        } else {
                            break 'b_4048a8;
                        }
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T6][0] as u32) << ((c.g[T5][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                break 'b_4048dc;
            } else {
                c.cycles += 8;
                c.g[V0][0] = 0x10;
                c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                let t = true;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                if t {
                    break 'b_4048e0;
                } else {
                    break 'b_4048dc;
                }
            }
        }
        c.cycles += 1;
        c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
        break 'b_4048e0;
    }
    'b_4049a8: {
        'b_404994: {
            c.cycles += 38;
            c.g[A1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
            c.g[V0][0] = 0x140;
            c.g[A3][0] = 0x600;
            c.g[A0][0] = sx((c.g[S3][0] as u32) << 1);
            c.mult(0, 3, 5, 2);
            c.mult(0, 7, 23, 7);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc8));
            c.g[T1][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x300));
            c.g[A2][0] = 0x10;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T5][0] as u32) & 31));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T8][0] as u32));
            c.g[V0][0] = sx((c.g[S2][0] as u32) >> 31);
            c.g[V1][0] = sx((c.g[S6][0] as u32) >> 31);
            c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x590));
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S3][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[S6][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
            c.g[A0][0] = c.g[A3][0] | c.g[A0][0];
            c.g[A0][0] = c.g[A0][0] | c.g[T9][0];
            c.g[A3][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.sw((c.g[T4][0] as u32).wrapping_add(0x14), c.g[T1][0]);
            c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
            c.sw((c.g[T4][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[T4][0] as u32).wrapping_add(0x18), c.g[T2][0]);
            let t = c.g[T5][0] == 0u64;
            c.g[T1][0] = sx(((c.g[T6][0] as u32) as i32 >> 1) as u32);
            if t {
                break 'b_404994;
            } else {
                c.cycles += 7;
                c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
                c.g[A0][0] = sx(((c.g[T3][0] as u32) as i32 >> 1) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let t = true;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if t {
                    break 'b_4049a8;
                } else {
                    break 'b_404994;
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
        c.g[V1][0] = sx(((c.g[T3][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        break 'b_4049a8;
    }
    'b_404a5c: {
        'b_404a58: {
            'b_404a24: {
                c.cycles += 17;
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[T2][0] = sx(((c.g[A2][0] as u32) as i32 >> 3) as u32);
                c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[T2][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[S3][0] = c.g[S3][0] & 0x1;
                c.g[A3][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x200));
                c.g[V1][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[T9][0] = c.g[S6][0] & 0x1;
                c.sw((c.g[T7][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                let t = c.g[T9][0] == 0u64;
                c.sw(c.g[T7][0] as u32, c.g[V0][0]);
                if t {
                    break 'b_404a24;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[T5][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 8i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                        break 'b_404a58;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = 0x8;
                        c.g[V1][0] =
                            sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] =
                            sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = true;
                        c.sw((c.g[T7][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        if t {
                            break 'b_404a5c;
                        } else {
                            break 'b_404a24;
                        }
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[T5][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 9i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                break 'b_404a58;
            } else {
                c.cycles += 8;
                c.g[V0][0] = 0x8;
                c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                let t = true;
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                if t {
                    break 'b_404a5c;
                } else {
                    break 'b_404a58;
                }
            }
        }
        c.cycles += 1;
        c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
        break 'b_404a5c;
    }
    c.cycles += 70;
    c.g[V0][0] = 0x8;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[S4][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[T5][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[S5][0] as u32));
    c.sw((c.g[T7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A1][0] = 0x140;
    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x450000;
    c.g[T0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
    c.g[T2][0] = 0x180;
    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3280));
    c.mult(0, 8, 8, 5);
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mult(0, 5, 4, 10);
    c.mult(0, 13, 12, 10);
    c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
    c.g[T4][0] = c.lw(c.g[V1][0] as u32);
    c.g[A3][0] = sx((c.g[S7][0] as u32) << 2);
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S2][0] as u32));
    c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
    c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T3][0] as u32));
    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x5b8), c.g[T4][0]);
    c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x400));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
    c.g[T0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x100));
    c.mult(0, 12, 11, 10);
    c.g[A1][0] = 0x450000;
    c.g[T1][0] = c.lw(c.g[FP][0] as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x32a0));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.sw((c.g[T7][0] as u32).wrapping_add(0x14), c.g[A0][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.sw((c.g[T7][0] as u32).wrapping_add(0x18), c.g[V1][0]);
    c.g[A0][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[T3][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T1][0] as u32));
    c.g[A1][0] = c.lw(c.g[T0][0] as u32);
    c.g[V1][0] = c.g[A0][0];
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x598), c.g[T1][0]);
    c.g[A3][0] = c.g[V1][0];
    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x5c8), c.g[A2][0]);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x5a8), c.g[T3][0]);
    c.sw(c.g[T0][0] as u32, c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.pc = t;
    return;
}

/// `int libmpeg_WaitBdec(dec)` (0x405b70): waits for the IPU decode to finish.
/// Loops while the IPU-from DMA has quadwords queued (`0x1000b020`) and no
/// error is flagged in `IPU_CTRL` (`0x10002010`, ECD = bit 14), feeding the
/// stalled to-IPU DMA through the callback `dec[0x858]` (`DispatchCallback`,
/// command 1). With `dec[0x10a0]` set it aborts instead (interrupts off, DMA
/// registers cleared, `IPU_CTRL = 0x40000000`, returns 0). Otherwise it
/// stores `IPU_TOP` and the bit-position remainder (`dec+0x838/0x83c`) and,
/// when ECD is set, reports the error (`Error`, callbacks 2 and 3), resets the
/// DMA and returns 0; else 1.
pub fn lib_mpeg_wait_bdec(c: &mut Cpu) {
    guest(c, |c| {
        'b_405c38: {
            'b_405c34: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S2][0]);
                c.g[S2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S6][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S5][0]);
                c.g[S6][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                c.g[RA][0] = sx(0x00405ba0);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S0][0]);
                Callee {
                    addr: 0x00406e20,
                    f: ee::s15::libmpeg_IpuWaitBusy,
                }
                .call(c, 0x00405ba0)?;
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0xb020;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x10000000;
                if t {
                    break 'b_405c34;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[V0][0] | 0x2010;
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.g[V1][0] & 0x4000;
                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
                        break 'b_405c38;
                    } else {
                        c.cycles += 10;
                        c.g[S4][0] = 0x10000000;
                        c.g[S3][0] = 0x10000000;
                        c.g[S1][0] = 0x10000000;
                        c.g[S0][0] = 0x10000000;
                        c.g[S4][0] = c.g[S4][0] | 0xb420;
                        c.g[S3][0] = c.g[S3][0] | 0xb400;
                        c.g[S5][0] = 0x1;
                        c.g[S1][0] = c.g[S1][0] | 0xb020;
                        c.g[S0][0] = c.g[S0][0] | 0x2010;
                        'l_405bf0: loop {
                            'b_405c18: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_405c18;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] & 0x100;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[SP][0];
                                    if t {
                                        break 'b_405c18;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
                                        c.g[RA][0] = sx(0x00405c18);
                                        c.sw(c.g[SP][0] as u32, c.g[S5][0]);
                                        Callee {
                                            addr: 0x00402c30,
                                            f: ee::s15::libmpeg_DispatchCallback,
                                        }
                                        .call(c, 0x00405c18)?;
                                        break 'b_405c18;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
                                break 'b_405c38;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x4000;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00405bf0);
                                    }
                                    continue 'l_405bf0;
                                } else {
                                    break 'b_405c34;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
            break 'b_405c38;
        }
        'b_405de8: {
            'b_405cd8: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x10000000;
                if t {
                    break 'b_405cd8;
                } else {
                    'b_405ca8: {
                        c.cycles += 23;
                        c.g[RA][0] = sx(0x00405c48);
                        Callee {
                            addr: 0x0042c078,
                            f: ee::s15::DIntr,
                        }
                        .call(c, 0x00405c48)?;
                        c.g[A3][0] = 0x10000000;
                        c.g[A0][0] = 0x10000;
                        c.g[A3][0] = c.g[A3][0] | 0xf520;
                        c.g[T0][0] = 0x10000000;
                        c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                        c.g[T0][0] = c.g[T0][0] | 0xf590;
                        c.g[A1][0] = 0x10000000;
                        c.g[A2][0] = 0x10000000;
                        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                        c.g[A1][0] = c.g[A1][0] | 0xb400;
                        c.sw(c.g[T0][0] as u32, c.g[V1][0]);
                        c.g[A2][0] = c.g[A2][0] | 0xb000;
                        c.sw(c.g[A1][0] as u32, 0u64);
                        c.g[A0][0] = 0xfffffffffffe0000;
                        c.sw(c.g[A2][0] as u32, 0u64);
                        c.g[A0][0] = c.g[A0][0] | 0xffff;
                        c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                        c.sw(c.g[T0][0] as u32, c.g[V1][0]);
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x10000000;
                        if t {
                            break 'b_405ca8;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x00405ca4);
                            Callee {
                                addr: 0x0042c0c0,
                                f: ee::s15::EIntr,
                            }
                            .call(c, 0x00405ca4)?;
                            c.g[V0][0] = 0x10000000;
                            break 'b_405ca8;
                        }
                    }
                    c.cycles += 12;
                    c.g[V1][0] = 0x10000000;
                    c.g[V0][0] = c.g[V0][0] | 0xb020;
                    c.g[V1][0] = c.g[V1][0] | 0xb420;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[A0][0] = 0x10000000;
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[A0][0] = c.g[A0][0] | 0x2010;
                    c.g[V1][0] = 0x40000000;
                    c.g[V0][0] = 0u64;
                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        break 'b_405de8;
                    } else {
                        break 'b_405cd8;
                    }
                }
            }
            'b_405d18: {
                'b_405d14: {
                    c.cycles += 8;
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x2030));
                    c.g[V0][0] = c.g[V0][0] | 0x2020;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.g[A0][0] << 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    let t = (c.g[A0][0] as i64) >= 0;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x838), c.g[V1][0]);
                    if t {
                        break 'b_405d14;
                    } else {
                        'b_405d0c: {
                            c.cycles += 3;
                            c.g[V1][0] = c.g[V0][0] & 0x1f;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x20;
                            if t {
                                break 'b_405d0c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                if t {
                                    break 'b_405d18;
                                } else {
                                    break 'b_405d0c;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_405d18;
                        } else {
                            break 'b_405d14;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x20;
                break 'b_405d18;
            }
            'b_405de0: {
                c.cycles += 7;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x83c), c.g[V0][0]);
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x2010;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.g[V1][0] & 0x4000;
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = 0x490000;
                if t {
                    break 'b_405de0;
                } else {
                    'b_405dd4: {
                        c.cycles += 37;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00405d40);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7eb0));
                        Callee {
                            addr: 0x004034a8,
                            f: ee::s15::libmpeg_Error,
                        }
                        .call(c, 0x00405d40)?;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00405d58);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        Callee {
                            addr: 0x00402c30,
                            f: ee::s15::libmpeg_DispatchCallback,
                        }
                        .call(c, 0x00405d58)?;
                        c.g[V1][0] = 0x40000000;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
                        c.g[AT][0] = 0x10000000;
                        c.sw((c.g[AT][0] as u32).wrapping_add(0x2010), c.g[V1][0]);
                        c.g[V0][0] = 0x3;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        c.g[RA][0] = sx(0x00405d78);
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00402c30,
                            f: ee::s15::libmpeg_DispatchCallback,
                        }
                        .call(c, 0x00405d78)?;
                        c.g[RA][0] = sx(0x00405d80);
                        Callee {
                            addr: 0x0042c078,
                            f: ee::s15::DIntr,
                        }
                        .call(c, 0x00405d80)?;
                        c.g[A2][0] = 0x10000000;
                        c.g[A3][0] = 0x10000000;
                        c.g[A2][0] = c.g[A2][0] | 0xf520;
                        c.g[T0][0] = 0x10000;
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        c.g[A3][0] = c.g[A3][0] | 0xf590;
                        c.g[A0][0] = 0x10000000;
                        c.g[A1][0] = 0xfffffffffffe0000;
                        c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
                        c.g[A0][0] = c.g[A0][0] | 0xb000;
                        c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                        c.g[A1][0] = c.g[A1][0] | 0xffff;
                        c.sw(c.g[A0][0] as u32, 0u64);
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                        c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x10000000;
                        if t {
                            break 'b_405dd4;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x00405dd0);
                            Callee {
                                addr: 0x0042c0c0,
                                f: ee::s15::EIntr,
                            }
                            .call(c, 0x00405dd0)?;
                            c.g[V0][0] = 0x10000000;
                            break 'b_405dd4;
                        }
                    }
                    c.cycles += 3;
                    c.g[S6][0] = 0u64;
                    c.g[V0][0] = c.g[V0][0] | 0xb020;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    break 'b_405de0;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S6][0];
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_405de8;
        }
        c.cycles += 9;
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `int libmpeg_DispatchCallback(mpeg, cbData, ...)` (0x402c30): calls the
/// callback of the mpeg system's entry `*cbData` (entries at `sys + 8 * idx`,
/// the function at +0xc, its argument at +0x10) and returns its result, or 0
/// when there is no mpeg, system, or callback.
pub fn libmpeg_dispatch_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        // The exit block (+1, ra reload) runs unless the call was skipped.
        let mut exit_block = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(r32(c, A0)).off(0x40).lw(c);
            frame.restore_ra(c, 0);
            exit_block = false;
            if c.g[A2][0] != 0 {
                c.cycles += 6;
                let idx = Addr(r32(c, A1)).lw(c) as u32;
                c.g[V0][0] = sx(idx << 3);
                c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V1)).off(0xc).lw(c);
                let has_fn = c.g[V1][0] != 0;
                c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
                if has_fn {
                    c.cycles += 3;
                    let target = r32(c, V1);
                    c.g[A2][0] = Addr(r32(c, V0)).off(0x10).lw(c);
                    call_ptr(c, target, 0x0040_2c6c)?;
                    c.g[A3][0] = c.g[V0][0];
                    exit_block = true;
                }
            }
        }
        if exit_block {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A3][0];
        frame.close(c);
        Ok(())
    })
}

// Direct-call targets of this module (address, readable alias in the shard).
pub(crate) const CALL_DISPATCH_CALLBACK: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::libmpeg_DispatchCallback,
};

pub(crate) const CALL_CALLBACK_NODATA: Callee = Callee {
    addr: 0x0040_2c80,
    f: ee::s15::libmpeg_CallbackNodata,
};

pub(crate) const CALL_LIBMPEG_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};

pub(crate) const CALL_SETUP_REF_FRAMES: Callee = Callee {
    addr: 0x0040_7f20,
    f: ee::s15::libmpeg_SetupRefFrames,
};

pub(crate) const CALL_DECODE_PICTURE_SLICES: Callee = Callee {
    addr: 0x0040_7dc0,
    f: ee::s15::libmpeg_DecodePictureSlices,
};

pub(crate) const CALL_GET_HEADER: Callee = Callee {
    addr: 0x0040_7520,
    f: ee::s15::libmpeg_GetHeader,
};

pub(crate) const CALL_FLUSH_PENDING: Callee = Callee {
    addr: 0x0040_3240,
    f: ee::s15::libmpeg_FlushPendingPicture,
};

pub(crate) const CALL_OUTPUT_PICTURE: Callee = Callee {
    addr: 0x0040_7e90,
    f: ee::s15::libmpeg_OutputPicture,
};

pub(crate) const CALL_DECODE_FIELD_PIC: Callee = Callee {
    addr: 0x0040_30e0,
    f: ee::s15::libmpeg_DecodeFieldPic,
};

pub(crate) const CALL_DECODE_FRAME_PIC: Callee = Callee {
    addr: 0x0040_2f80,
    f: ee::s15::libmpeg_DecodeFramePic,
};

/// `void libmpeg_CallbackNodata(mpeg)` (0x402c80): registers the one-entry
/// callback table `{1}` with `libmpeg_DispatchCallback` and returns 1.
pub fn libmpeg_callback_nodata(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0];
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        CALL_DISPATCH_CALLBACK.call(c, 0x0040_2c98)?;
        frame.restore_ra(c, 0x20);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_WorkAreaInit(area, base, size)` (0x402d50): a work area is
/// `{base, size, cursor, mark}` at +0, +4, +8 and +0xc; the cursor and the
/// mark start at `base`.
pub fn libmpeg_work_area_init(c: &mut Cpu) {
    c.cycles += 5;
    let area = Addr(c.g[A0][0] as u32);
    area.off(0xc).sw(c, c.g[A1][0]);
    area.off(4).sw(c, c.g[A2][0]);
    area.sw(c, c.g[A1][0]);
    area.off(8).sw(c, c.g[A1][0]);
    ret(c);
}

/// `void libmpeg_WorkAreaMark(area)` (0x402d68): records the cursor (+8) as
/// the mark (+0xc), so a later restore can rewind to it.
pub fn libmpeg_work_area_mark(c: &mut Cpu) {
    c.cycles += 3;
    let area = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = area.off(8).lw(c);
    area.off(0xc).sw(c, c.g[V0][0]);
    ret(c);
}

/// `void libmpeg_WorkAreaRestore(area)` (0x402d78): rewinds the cursor (+8)
/// to the mark (+0xc).
pub fn libmpeg_work_area_restore(c: &mut Cpu) {
    c.cycles += 3;
    let area = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = area.off(0xc).lw(c);
    area.off(8).sw(c, c.g[V0][0]);
    ret(c);
}

/// `void* libmpeg_WorkAreaAlloc(dec, pool, size, align)` (0x402d88): bump
/// allocates `size` bytes at an `align` boundary from the pool `{base, size,
/// cursor}`. Returns the block start; when the block would run past the
/// pool's end it reports through `libmpeg_Error` (the 0x497c48 message) and
/// returns 0.
pub fn libmpeg_work_area_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.cycles += 13;
        let pool = Addr(c.g[A1][0] as u32);
        c.g[V0][0] = pool.off(8).lw(c);
        c.g[A0][0] = pool.off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = pool.lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.divu(0, 2, 7);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lo[0];
        c.mult(0, 2, 2, 7);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        if c.g[V1][0] != 0 {
            c.g[A1][0] = 0x0049_0000;
            c.cycles += 4;
            c.g[A0][0] = c.g[T0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7c48));
            CALL_LIBMPEG_ERROR.call(c, 0x0040_2de4)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            pool.off(8).sw(c, c.g[A0][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_DecodeFramePic(mpeg, arg1, arg2)` (0x402f80): decodes one
/// frame picture. With the second limit (`arg2`) not -1 and `arg1 >= arg2`
/// the picture is skipped: reference frames are set up and the callback is
/// registered (`S3 = 1`). Otherwise the decoder's counters are set up, the
/// reference frames are set up and the slices are decoded. Then the
/// picture is output, the counters advance, and `S2` (the picture result)
/// is returned.
pub fn libmpeg_decode_frame_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = u64::MAX;
        Addr(frame.at(c, 0x30)).sd(c, c.g[S3][0]);
        Addr(frame.at(c, 0x10)).sd(c, c.g[S1][0]);
        c.g[S3][0] = 0;
        Addr(frame.at(c, 0x40)).sd(c, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        Addr(frame.at(c, 0x20)).sd(c, c.g[S2][0]);
        Addr(frame.at(c, 0)).sd(c, c.g[S0][0]);
        let no_limit = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        let mut skipped = false;
        if !no_limit {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            skipped = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
        }
        if skipped {
            c.cycles += 6;
            c.g[A1][0] = 0;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_2ffc)?;
            c.g[S3][0] = 1;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            CALL_CALLBACK_NODATA.call(c, 0x0040_300c)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
            let counted = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !counted {
                c.cycles += 3;
                Addr(c.g[S1][0] as u32).off(8).sw(c, 0);
                c.g[V0][0] = 1;
                Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[A1][0] = 0;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_2fd8)?;
            let no_slices = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !no_slices {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                CALL_DECODE_PICTURE_SLICES.call(c, 0x0040_2fe8)?;
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            c.g[S2][0] = c.g[V1][0];
        }
        // Output and counters.
        c.cycles += 8;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        CALL_OUTPUT_PICTURE.call(c, 0x0040_301c)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x174).lw(c);
        c.g[V0][0] = 3;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
        } else {
            c.cycles += 2;
            if c.g[S3][0] != 0 {
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x120).lw(c);
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                Addr(c.g[S0][0] as u32).off(0x120).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        Addr(c.g[S1][0] as u32).off(8).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x120).lw(c);
        c.g[V0][0] = c.g[S2][0];
        if c.g[V1][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            Addr(c.g[S0][0] as u32).off(0x118).sw(c, c.g[V0][0]);
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 7;
        c.g[RA][0] = Addr(frame.at(c, 0x40)).ld(c);
        c.g[S3][0] = Addr(frame.at(c, 0x30)).ld(c);
        c.g[S2][0] = Addr(frame.at(c, 0x20)).ld(c);
        c.g[S1][0] = Addr(frame.at(c, 0x10)).ld(c);
        c.g[S0][0] = Addr(frame.at(c, 0)).ld(c);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_DecodePicture(mpeg)` (0x403098): a progressive picture
/// (`decoder+0x174 == 3`) goes to `libmpeg_DecodeFramePic`, anything else to
/// `libmpeg_DecodeFieldPic`, both with the mpeg object.
pub fn libmpeg_decode_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 3;
        c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x40).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x174).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[A3][0];
            CALL_DECODE_FRAME_PIC.call(c, 0x0040_30d0)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[A3][0];
            CALL_DECODE_FIELD_PIC.call(c, 0x0040_30c0)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_DecodeFieldPic(mpeg, arg1, arg2)` (0x4030e0): decodes one
/// field picture. The header comes from `libmpeg_GetHeader`; a failed header
/// flushes the pending picture and returns 0. A header that matches the
/// decoder's expected picture kind sets up the reference frames and slices,
/// outputs the picture, advances the counters and returns `S4`.
pub fn libmpeg_decode_field_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        'epilogue: {
            c.g[V0][0] = u64::MAX;
            Addr(frame.at(c, 0x30)).sd(c, c.g[S3][0]);
            Addr(frame.at(c, 0x20)).sd(c, c.g[S2][0]);
            c.g[S3][0] = 0;
            Addr(frame.at(c, 0x50)).sd(c, c.g[RA][0]);
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x40)).sd(c, c.g[S4][0]);
            Addr(frame.at(c, 0x10)).sd(c, c.g[S1][0]);
            Addr(frame.at(c, 0)).sd(c, c.g[S0][0]);
            c.g[S0][0] = Addr(c.g[S2][0] as u32).off(0x40).lw(c);
            let mut s3_set = c.g[A2][0] == c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            if !s3_set {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                s3_set = c.g[V0][0] != 0;
            }
            if s3_set {
                c.cycles += 2;
                c.g[S3][0] = 1;
            }
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
            c.cycles += 2;
            let counted = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !counted {
                c.cycles += 3;
                Addr(c.g[S2][0] as u32).off(8).sw(c, 0);
                c.g[V0][0] = 1;
                Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
            }
            // The reference frames, and the slices when the field has them.
            c.cycles += 4;
            c.g[A1][0] = 0;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_3144)?;
            c.g[S1][0] = 1;
            let mut to_tail = true;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if c.g[S3][0] == 0 {
                    to_tail = false;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    CALL_DECODE_PICTURE_SLICES.call(c, 0x0040_315c)?;
                    c.g[S1][0] = 1;
                }
            }
            if to_tail {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            }
            // The picture header.
            c.cycles += 4;
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, c.g[S1][0]);
            CALL_GET_HEADER.call(c, 0x0040_316c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xd4).lw(c);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                CALL_FLUSH_PENDING.call(c, 0x0040_317c)?;
                Addr(c.g[S0][0] as u32).sw(c, c.g[S1][0]);
                c.g[V0][0] = 0;
                break 'epilogue;
            }
            c.cycles += 6;
            c.g[V1][0] = 2;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x174).lw(c);
            c.g[V0][0] ^= 1;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
            let mismatch = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = u64::MAX;
            if mismatch {
                break 'epilogue;
            }
            // The field's slices, then its output.
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            CALL_SETUP_REF_FRAMES.call(c, 0x0040_31ac)?;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S1][0];
            }
            let no_slices = c.g[V1][0] == 0;
            c.g[S4][0] = 0;
            let mut to_output = false;
            if !no_slices {
                c.cycles += 2;
                if c.g[S3][0] == 0 {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
                    to_output = true;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    CALL_DECODE_PICTURE_SLICES.call(c, 0x0040_31cc)?;
                    if c.g[V0][0] != 0 {
                        c.g[S4][0] = c.g[S1][0];
                    }
                }
            }
            if !to_output {
                c.cycles += 1;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            }
            c.cycles += 15;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            CALL_OUTPUT_PICTURE.call(c, 0x0040_31e0)?;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            Addr(c.g[S2][0] as u32).off(8).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(c.g[S0][0] as u32).off(0x118).sw(c, c.g[V1][0]);
            let picture_skipped = c.g[S3][0] != 0;
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V0][0]);
            if !picture_skipped {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                CALL_CALLBACK_NODATA.call(c, 0x0040_3218)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
        }
        c.cycles += 8;
        c.g[RA][0] = Addr(frame.at(c, 0x50)).ld(c);
        c.g[S4][0] = Addr(frame.at(c, 0x40)).ld(c);
        c.g[S3][0] = Addr(frame.at(c, 0x30)).ld(c);
        c.g[S2][0] = Addr(frame.at(c, 0x20)).ld(c);
        c.g[S1][0] = Addr(frame.at(c, 0x10)).ld(c);
        c.g[S0][0] = Addr(frame.at(c, 0)).ld(c);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_FlushPendingPicture(Mpeg *m)` (0x00403240): when the decoder
/// `d = m->dec` (+0x40) has a picture pending (`d[1]` and `d[2]` set), output it
/// with `libmpeg_OutputLastPicture`, set `m[2] = d[70] - d[43]` and clear
/// `d[1]`. Returns 1 when it flushed, else 0.
#[allow(dead_code)]
pub fn libmpeg_flush_pending_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        let pending = c.g[V0][0] != 0;
        c.g[A0][0] = 0;
        let mut fall_through = true;
        if pending {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
            let has_picture = c.g[V0][0] != 0;
            frame.restore_ra(c, 0x20);
            fall_through = has_picture;
            if has_picture {
                c.cycles += 8;
                Callee {
                    addr: 0x0040_32b8,
                    f: ee::s15::f_004032b8,
                }
                .call(c, 0x0040_3278)?;
                // The state is re-read from the callee-saved registers after the call.
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
                c.g[A0][0] = 1;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                let frames = c.g[V0][0];
                Addr(c.g[S1][0] as u32).off(0x8).sw(c, frames);
                Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
            }
        }
        if fall_through {
            c.cycles += 1;
            frame.restore_ra(c, 0x20);
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[A0][0];
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_ResetMpeg1Flag(Mpeg *m)` (0x004032a8): clears `m->mpeg1` (+0x848)
/// and tail-calls `libmpeg_SetIpuMpeg1Bit(1)`.
pub fn libmpeg_reset_mpeg1_flag(c: &mut Cpu) {
    c.cycles += 3;
    Addr(c.g[A0][0] as u32).off(0x848).sw(c, 0);
    c.g[A0][0] = 1;
    c.pc = 0x0040_5b48;
    ee::s15::f_00405b48(c);
}

/// `void libmpeg_OutputLastPicture(Mpeg *dec)` (0x004032b8): the pending
/// picture's output. A non-zero `dec[72]` (+0x120) is an error (`libmpeg_Error`
/// with the message at 0x497cc8) and clears it. Otherwise it outputs a frame
/// (`libmpeg_OutputFrame`, when `dec[93]` (+0x174) is 3) or a field pair
/// (`libmpeg_OutputFieldPair`), then clears `dec[72]`.
pub fn libmpeg_output_last_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let pending_error = Addr(c.g[S0][0] as u32).off(0x120).lw(c);
        c.g[V0][0] = pending_error;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        if pending_error != 0 {
            c.cycles += 5;
            c.g[A1][0] = sx(0x0049_7cc8);
            Callee {
                addr: 0x0040_34a8,
                f: ee::s15::f_004034a8,
            }
            .call(c, 0x0040_32e0)?;
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x174).lw(c);
            c.g[V0][0] = 3;
            c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            if c.g[V1][0] == 3 {
                c.cycles += 6;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x1bc).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0040_86c0,
                    f: ee::s15::f_004086c0,
                }
                .call(c, 0x0040_3308)?;
                Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            } else {
                c.cycles += 5;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x1cc).lw(c);
                c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x1dc).lw(c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0040_87d0,
                    f: ee::s15::f_004087d0,
                }
                .call(c, 0x0040_3320)?;
                Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_InitSprBuffers(Mpeg *m)` (0x00403338): switches the IPU to
/// MPEG-1 (`libmpeg_SetIpuMpeg1Bit(1)`), then points the four scratchpad
/// buffer slots of `m` at 0x70000000 and its +0x1800/+0x1b00/+0x3300 offsets
/// and clears the word at +0x810.
pub fn libmpeg_init_spr_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 1;
        Callee {
            addr: 0x0040_5b48,
            f: ee::s15::f_00405b48,
        }
        .call(c, 0x0040_3354)?;
        c.g[S1][0] = 0x7000_0000;
        c.g[V0][0] = 0x7000_1800;
        c.g[V1][0] = 0x7000_1b00;
        c.g[A0][0] = 0x7000_3300;
        let m = Addr(c.g[S0][0] as u32);
        let base = c.g[S1][0];
        m.off(0x590).sw(c, base);
        let second = c.g[V0][0];
        m.off(0x594).sw(c, second);
        let third = c.g[V1][0];
        m.off(0x6d0).sw(c, third);
        let fourth = c.g[A0][0];
        m.off(0x6d4).sw(c, fourth);
        m.off(0x810).sw(c, 0);
        frame.restore_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_ResetIpu(Mpeg *m)` (0x00403398): sets `m[+0x818] = 1`,
/// clears `m[+0x1b0]`, disables interrupts (`DIntr`, its state in `v0`),
/// resets the IPU and DMA registers (0x1000b000.., 0x1000d400..) and the
/// DMA/IPU control words, re-enables them (`EIntr`) when they were enabled,
/// clears the IPU status registers and tail-calls `sceIpuSync(0, 0)` after
/// writing 0x40000000 to the IPU control register.
pub fn libmpeg_reset_ipu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        let v = c.g[V0][0];
        Addr(c.g[A0][0] as u32).off(0x818).sw(c, v);
        Addr(c.g[A0][0] as u32).off(0x1b0).sw(c, 0);
        Callee {
            addr: 0x0042_c078,
            f: ee::s15::f_0042c078,
        }
        .call(c, 0x0040_33b0)?;
        // `v0` is the interrupt state DIntr returned.
        let intr_on = c.g[V0][0] != 0;
        c.g[A2][0] = 0x1000_f520;
        c.g[T0][0] = 0x1_0000;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A3][0] = 0x1000_f590;
        c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
        let v = c.g[V1][0];
        Addr(c.g[A3][0] as u32).sw(c, v);
        c.g[A0][0] = 0x1000_b000;
        c.g[A1][0] = 0x1000_b400;
        Addr(c.g[A0][0] as u32).sw(c, 0);
        Addr(c.g[A1][0] as u32).sw(c, 0);
        c.g[V1][0] = 0x1000_d400;
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.g[A0][0] = 0xffff_ffff_fffe_ffff;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c) & c.g[A0][0];
        let v = c.g[V1][0];
        Addr(c.g[A3][0] as u32).sw(c, v);
        c.g[V0][0] = 0x1000_0000;
        if intr_on {
            c.cycles += 3;
            Callee {
                addr: 0x0042_c0c0,
                f: ee::s15::f_0042c0c0,
            }
            .call(c, 0x0040_3418)?;
            c.g[V0][0] = 0x1000_0000;
        }
        c.cycles += 17;
        c.g[V0][0] = 0x1000_b020;
        c.g[A0][0] = 0x1000_b420;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        c.g[V1][0] = 0x1000_d420;
        Addr(c.g[A0][0] as u32).sw(c, 0);
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.g[V0][0] = 0x1000_2010;
        c.g[V1][0] = 0x4000_0000;
        let v = c.g[V1][0];
        Addr(c.g[V0][0] as u32).sw(c, v);
        frame.restore_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = 0x0040_9638;
        ee::s15::f_00409638(c);
        Ok(())
    })
}

/// `void libmpeg_DefaultErrorPrint(const char *msg)` (0x00403460): prints the
/// message through the format at 0x497ce8 (tail call to `scePrintf`).
pub fn libmpeg_default_error_print(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A0][0];
    c.g[A0][0] = sx(0x0049_7ce8);
    c.pc = 0x0042_5c60;
    ee::s15::f_00425c60(c);
}

/// `void libmpeg_Errorf(Mpeg *m, const char *fmt, ...)` (0x00403470): formats
/// the message into a 0x120-byte stack buffer (`sprintf`) and reports it with
/// `libmpeg_Error(m, buf)`.
pub fn libmpeg_errorf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x120);
        let a = frame.at(c, 0x100);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x110);
        c.g[A0][0] = c.g[SP][0];
        Callee {
            addr: 0x0041_7828,
            f: ee::s15::f_00417828,
        }
        .call(c, 0x0040_3488)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x0040_34a8,
            f: ee::s15::f_004034a8,
        }
        .call(c, 0x0040_3494)?;
        frame.restore_ra(c, 0x110);
        let a = frame.at(c, 0x100);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_Error(Mpeg *m, const char *msg)` (0x004034a8): with a
/// decoder `m` whose callback table is set (`m[+0x858]`, its `+0xc` non-zero),
/// queues the message through `libmpeg_DispatchCallback` (message in a
/// two-word block on the stack, type 0); otherwise prints it with
/// `libmpeg_DefaultErrorPrint`.
pub fn libmpeg_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x858).lw(c);
        let mut dispatched = false;
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
                let has_callback = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[V1][0];
                if has_callback {
                    c.cycles += 6;
                    let msg_slot = frame.at(c, 4);
                    let msg = c.g[A1][0];
                    Addr(msg_slot).sw(c, msg);
                    let type_slot = frame.at(c, 0);
                    Addr(type_slot).sw(c, 0);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x0040_2c30,
                        f: ee::s15::f_00402c30,
                    }
                    .call(c, 0x0040_34e0)?;
                    frame.restore_ra(c, 0x10);
                    dispatched = true;
                }
            }
        }
        if !dispatched {
            c.cycles += 3;
            c.g[A0][0] = c.g[A1][0];
            Callee {
                addr: 0x0040_3460,
                f: ee::s15::f_00403460,
            }
            .call(c, 0x0040_34f0)?;
            frame.restore_ra(c, 0x10);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// Direct-call targets of this module's functions (guest address, readable alias
// in the generated shard).
pub(crate) const LIBMPEG_CALLEE_DISPATCH_CALLBACK: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::libmpeg_DispatchCallback,
};

pub(crate) const LIBMPEG_CALLEE_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};

pub(crate) const LIBMPEG_CALLEE_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::libmpeg_IpuCommand,
};

pub(crate) const LIBMPEG_CALLEE_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};

pub(crate) const LIBMPEG_CALLEE_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::libmpeg_GetBits,
};

pub(crate) const LIBMPEG_CALLEE_SET_IPU_MPEG1_BIT: Callee = Callee {
    addr: 0x0040_5b48,
    f: ee::s15::libmpeg_SetIpuMpeg1Bit,
};

pub(crate) const LIBMPEG_CALLEE_IPU_RESTART_DMA: Callee = Callee {
    addr: 0x0040_94e8,
    f: ee::s15::sceIpuRestartDMA,
};

pub(crate) const LIBMPEG_CALLEE_FLUSH_CACHE: Callee = Callee {
    addr: 0x0042_4020,
    f: ee::s15::FlushCache,
};

pub(crate) const LIBMPEG_CALLEE_DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::DIntr,
};

pub(crate) const LIBMPEG_CALLEE_EINTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::EIntr,
};

/// `void libmpeg_AddBsDmaChain(base, addr, n)` (0x403500): build a DMA tag chain
/// at `base` (`(base + 0x88f)` rounded down to 16 bytes): one 16-byte tag per
/// chunk of at most 0xfff40 bytes, with id 3 on every chunk but the last, then
/// flush the cache, mask interrupts around the channel start (`0x105` to the
/// channel control), and restore the interrupt state.
pub fn libmpeg_add_bs_dma_chain(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let f = Frame::open(c, 0x20);
        let a = f.at(c, 0x00);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x88f));
        c.g[A0][0] = sx((c.g[A0][0] as u32) >> 4);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 4);
        f.save_ra(c, 0x10);
        c.g[T3][0] = c.g[A1][0];
        c.g[T0][0] = c.g[A2][0];
        c.g[T2][0] = c.g[S0][0];
        if (c.g[T0][0] as i64) > 0 {
            c.cycles += 6;
            c.g[T1][0] = 0x000f_ff40;
            c.g[T4][0] = 0x0fff_ffff;
            c.g[T6][0] = 3;
            c.g[T5][0] = u64::MAX;
            loop {
                c.cycles += 23;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                c.g[A3][0] = c.g[T1][0];
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[T0][0];
                }
                c.g[V1][0] = 0;
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xf));
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1e));
                c.g[A1][0] = ((c.g[T5][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[A2][0];
                }
                c.g[A0][0] = c.g[T3][0] & c.g[T4][0];
                if c.g[T0][0] != 0 {
                    c.g[V1][0] = c.g[T6][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                c.g[A0][0] <<= 32;
                c.g[V1][0] <<= 28;
                c.g[V0][0] <<= 32;
                c.g[A0][0] |= c.g[V1][0];
                c.g[V0][0] >>= 32;
                c.g[A0][0] |= c.g[V0][0];
                let a = c.g[T2][0] as u32;
                let v = c.g[A0][0];
                c.sd(a, v);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A3][0] as u32));
                let more = (c.g[T0][0] as i64) > 0;
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_3540);
                }
            }
        }
        c.cycles += 19;
        c.g[A0][0] = 0;
        LIBMPEG_CALLEE_FLUSH_CACHE.call(c, 0x0040_35a4)?;
        LIBMPEG_CALLEE_DINTR.call(c, 0x0040_35ac)?;
        c.g[V1][0] = 0x0fff_ffff;
        c.g[A1][0] = 0x1000_b430;
        c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
        c.g[A0][0] = 0x1000_0000;
        let a = c.g[A1][0] as u32;
        let v = c.g[V1][0];
        Addr(a).sw(c, v);
        c.g[A0][0] |= 0xb420;
        let a = c.g[A0][0] as u32;
        Addr(a).sw(c, 0);
        c.g[V1][0] = 0x1000_b400;
        c.g[A0][0] = 0x105;
        let a = c.g[V1][0] as u32;
        let v = c.g[A0][0];
        Addr(a).sw(c, v);
        f.restore_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let a = f.at(c, 0x00);
            c.g[S0][0] = Addr(a).ld(c);
            f.close(c);
        } else {
            c.cycles += 3;
            let a = f.at(c, 0x00);
            c.g[S0][0] = Addr(a).ld(c);
            c.g[SP][0] = sx(sp(c).wrapping_add(f.size));
            c.pc = 0x0042_c0c0;
            ee::s15::EIntr(c);
        }
        Ok(())
    })
}

/// `void libmpeg_SetPictureSize(pic, width, height)` (0x403600): store the
/// size and the size in 16-pixel macroblocks (`>> 4`) into the picture record
/// (+4 width, +8 height, +0xc/+0x10 macroblocks); returns 1.
pub fn libmpeg_set_picture_size(c: &mut Cpu) {
    c.cycles += 8;
    let pic = c.g[A0][0] as u32;
    let width = c.g[A1][0];
    let height = c.g[A2][0];
    c.g[V0][0] = sx(((width as u32) as i32 >> 4) as u32);
    c.g[V1][0] = sx(((height as u32) as i32 >> 4) as u32);
    let v = c.g[V0][0];
    Addr(pic.wrapping_add(0xc)).sw(c, v);
    let v = c.g[V1][0];
    Addr(pic.wrapping_add(0x10)).sw(c, v);
    c.g[V0][0] = 1;
    Addr(pic.wrapping_add(0x4)).sw(c, width);
    Addr(pic.wrapping_add(0x8)).sw(c, height);
    ret(c);
}

/// `void libmpeg_SetupFrameBuffers(y, uBuffer, vBuffer, ..., w, height)`
/// (0x4039f0): stores the Y, U and V output pointers of three frame-buffer
/// sets (the first two at the plane start, the third advanced by
/// `w * height / 512` in the reference). Its arguments are partly on the
/// stack (the 9th to 14th words); the register form follows the generated
/// body exactly, including the 32-bit multiplies (`mult`) and the scratch
/// registers they leave.
#[allow(dead_code)]
pub fn libmpeg_setup_frame_buffers(c: &mut Cpu) {
    c.cycles += 56;
    let f = Frame::open(c, 0x50);
    c.g[T6][0] = 0x0fff_0000;
    let a = f.at(c, 0x70);
    c.g[V1][0] = Addr(a).lw(c);
    c.g[T6][0] |= 0xffff;
    let a = f.at(c, 0x00);
    let v = c.g[S0][0];
    c.sd(a, v);
    c.mult(0, 3, 3, 2);
    c.g[S0][0] = u64::MAX;
    let a = f.at(c, 0x20);
    let v = c.g[S2][0];
    c.sd(a, v);
    c.g[S2][0] = 0x180;
    let a = f.at(c, 0x58);
    c.g[T4][0] = Addr(a).lw(c);
    let a = f.at(c, 0x10);
    let v = c.g[S1][0];
    c.sd(a, v);
    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ff));
    if c.g[S0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[S1][0] = c.g[T4][0] & c.g[T6][0];
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
    let a = f.at(c, 0x30);
    let v = c.g[S3][0];
    c.sd(a, v);
    c.mult(0, 3, 2, 18);
    let a = f.at(c, 0x60);
    c.g[T5][0] = Addr(a).lw(c);
    c.g[S3][0] = 0x2000_0000;
    let a = f.at(c, 0x40);
    let v = c.g[S4][0];
    c.sd(a, v);
    let a = f.at(c, 0x68);
    c.g[S4][0] = Addr(a).lw(c);
    c.g[S1][0] |= c.g[S3][0];
    c.g[T7][0] = c.g[T5][0] & c.g[T6][0];
    let a = c.g[A0][0] as u32;
    let v = c.g[S1][0];
    Addr(a).sw(c, v);
    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
    c.g[T7][0] |= c.g[S3][0];
    c.mult(0, 3, 2, 18);
    c.mult(0, 4, 2, 18);
    let a = c.g[A1][0] as u32;
    let v = c.g[T7][0];
    Addr(a).sw(c, v);
    c.g[T4][0] &= c.g[T6][0];
    c.g[T4][0] |= c.g[S3][0];
    let a = f.at(c, 0x20);
    c.g[S2][0] = Addr(a).ld(c);
    let a = f.at(c, 0x00);
    c.g[S0][0] = Addr(a).ld(c);
    c.g[T5][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[V1][0] = c.g[S4][0] & c.g[T6][0];
    c.g[T5][0] &= c.g[T6][0];
    c.g[V1][0] |= c.g[S3][0];
    c.g[V0][0] &= c.g[T6][0];
    let a = c.g[A2][0] as u32;
    let v = c.g[V1][0];
    Addr(a).sw(c, v);
    c.g[T5][0] |= c.g[S3][0];
    let a = c.g[A3][0] as u32;
    let v = c.g[S1][0];
    Addr(a).sw(c, v);
    c.g[V0][0] |= c.g[S3][0];
    let a = c.g[T0][0] as u32;
    let v = c.g[T7][0];
    Addr(a).sw(c, v);
    let a = c.g[T1][0] as u32;
    let v = c.g[V1][0];
    Addr(a).sw(c, v);
    let a = c.g[T2][0] as u32;
    let v = c.g[T4][0];
    Addr(a).sw(c, v);
    let a = f.at(c, 0x50);
    c.g[V1][0] = Addr(a).lw(c);
    let a = c.g[T3][0] as u32;
    let v = c.g[T5][0];
    Addr(a).sw(c, v);
    let a = f.at(c, 0x40);
    c.g[S4][0] = Addr(a).ld(c);
    let a = f.at(c, 0x30);
    c.g[S3][0] = Addr(a).ld(c);
    let a = f.at(c, 0x10);
    c.g[S1][0] = Addr(a).ld(c);
    let a = c.g[V1][0] as u32;
    let v = c.g[V0][0];
    Addr(a).sw(c, v);
    f.close(c);
}

/// `void libmpeg_LoadIqMatrix(dec, srcAddr, ipuCmd)` (0x403ad0): wait for the
/// dispatch callback and the IPU, start the IPU source DMA at `srcAddr` with
/// interrupts masked around the channel setup, run the IPU command `ipuCmd`,
/// wait again and dispatch the final callback (state 3).
#[allow(dead_code)]
pub fn libmpeg_load_iq_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let f = Frame::open(c, 0x60);
        c.g[V0][0] = 2;
        let a = f.at(c, 0x40);
        let v = c.g[S2][0];
        c.sd(a, v);
        let a = f.at(c, 0x30);
        let v = c.g[S1][0];
        c.sd(a, v);
        c.g[S2][0] = c.g[A1][0];
        let a = f.at(c, 0x20);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[S1][0] = c.g[A0][0];
        let a = f.at(c, 0x50);
        let v = c.g[RA][0];
        c.sd(a, v);
        c.g[S0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[SP][0];
        let a = (c.g[S1][0] as u32).wrapping_add(0x858);
        c.g[A0][0] = Addr(a).lw(c);
        let a = f.at(c, 0x00);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        LIBMPEG_CALLEE_DISPATCH_CALLBACK.call(c, 0x0040_3b04)?;
        c.g[A0][0] = c.g[S1][0];
        LIBMPEG_CALLEE_IPU_WAIT_BUSY.call(c, 0x0040_3b0c)?;
        c.g[V0][0] = 0x1000_2000;
        c.g[A0][0] = c.g[S1][0];
        Addr(0x1000_2000).sw(c, 0);
        LIBMPEG_CALLEE_IPU_WAIT_BUSY.call(c, 0x0040_3b20)?;
        LIBMPEG_CALLEE_DINTR.call(c, 0x0040_3b28)?;
        c.g[V1][0] = 0x0fff_ffff;
        c.g[A0][0] = 0x1000_b410;
        c.g[S0][0] &= c.g[V1][0];
        c.g[A1][0] = 0x1000_0000;
        let a = c.g[A0][0] as u32;
        let v = c.g[S0][0];
        Addr(a).sw(c, v);
        c.g[A1][0] |= 0xb420;
        c.g[A0][0] = 4;
        c.g[V1][0] = 0x1000_0000;
        let a = c.g[A1][0] as u32;
        let v = c.g[A0][0];
        Addr(a).sw(c, v);
        c.g[V1][0] |= 0xb400;
        c.g[A0][0] = 0x101;
        let a = c.g[V1][0] as u32;
        let v = c.g[A0][0];
        Addr(a).sw(c, v);
        if c.g[V0][0] == 0 {
            c.g[A1][0] = c.g[S2][0];
        } else {
            c.cycles += 3;
            LIBMPEG_CALLEE_EINTR.call(c, 0x0040_3b70)?;
            c.g[A1][0] = c.g[S2][0];
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S1][0];
        LIBMPEG_CALLEE_IPU_COMMAND.call(c, 0x0040_3b7c)?;
        c.g[A0][0] = c.g[S1][0];
        LIBMPEG_CALLEE_IPU_WAIT_BUSY.call(c, 0x0040_3b84)?;
        let a = (c.g[S1][0] as u32).wrapping_add(0x858);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[V0][0] = 3;
        let a = f.at(c, 0x00);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        c.g[A1][0] = c.g[SP][0];
        LIBMPEG_CALLEE_DISPATCH_CALLBACK.call(c, 0x0040_3b98)?;
        f.restore_ra(c, 0x50);
        let a = f.at(c, 0x40);
        c.g[S2][0] = Addr(a).ld(c);
        let a = f.at(c, 0x30);
        c.g[S1][0] = Addr(a).ld(c);
        let a = f.at(c, 0x20);
        c.g[S0][0] = Addr(a).ld(c);
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_SequenceExtension(dec)` (0x403bb0): clear the IPU MPEG-1 bit,
/// read the 28-bit sequence-extension word, check its profile field (bits
/// 17-18 must be 1, else `libmpeg_Error`), keep the fields in the decoder
/// (+0x140, +0x124..+0x138 accumulators), and read the 16-bit chroma/level
/// word whose code (bits 20-27 of the first word, 0x48/0x58/0x44) is checked
/// the same way.
pub fn libmpeg_sequence_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let f = Frame::open(c, 0x70);
        let a = f.at(c, 0x10);
        let v = c.g[S1][0];
        c.sd(a, v);
        let a = f.at(c, 0x00);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[S1][0] = c.g[A0][0];
        let a = f.at(c, 0x50);
        let v = c.g[S5][0];
        c.sd(a, v);
        c.g[S0][0] = 1;
        let a = f.at(c, 0x40);
        let v = c.g[S4][0];
        c.sd(a, v);
        c.g[A0][0] = 0;
        let a = f.at(c, 0x30);
        let v = c.g[S3][0];
        c.sd(a, v);
        let a = f.at(c, 0x20);
        let v = c.g[S2][0];
        c.sd(a, v);
        f.save_ra(c, 0x60);
        let a = (c.g[S1][0] as u32).wrapping_add(0x848);
        let v = c.g[S0][0];
        Addr(a).sw(c, v);
        LIBMPEG_CALLEE_SET_IPU_MPEG1_BIT.call(c, 0x0040_3be4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1c;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3bf0)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[S2][0] as u32) >> 1);
        c.g[V0][0] = sx((c.g[S2][0] as u32) >> 17);
        c.g[S5][0] = c.g[V1][0] & 0xfff;
        c.g[V0][0] &= 3;
        c.g[A0][0] = sx((c.g[S2][0] as u32) >> 13);
        c.g[V1][0] = sx((c.g[S2][0] as u32) >> 15);
        c.g[S4][0] = c.g[A0][0] & 3;
        c.g[S3][0] = c.g[V1][0] & 3;
        let t = c.g[V0][0] == c.g[S0][0];
        let a = (c.g[S1][0] as u32).wrapping_add(0x140);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        if !t {
            c.cycles += 4;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7d10));
            LIBMPEG_CALLEE_ERROR.call(c, 0x0040_3c2c)?;
        }
        c.cycles += 11;
        c.g[V0][0] = sx((c.g[S2][0] as u32) >> 19);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] &= 1;
        c.g[A1][0] = 0x10;
        let a = (c.g[S1][0] as u32).wrapping_add(0x13c);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        c.g[S0][0] = sx((c.g[S2][0] as u32) >> 20);
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3c48)?;
        c.g[S2][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[V0][0] = 0x48;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x58;
        if !t {
            c.cycles += 2;
            let t = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = 0x44;
            if !t {
                c.cycles += 2;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[A1][0] = 0x490000;
                if !t {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7d38));
                    LIBMPEG_CALLEE_ERROR.call(c, 0x0040_3c74)?;
                }
            }
        }
        c.cycles += 27;
        let a = (c.g[S1][0] as u32).wrapping_add(0x124);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[T0][0] = sx((c.g[S5][0] as u32) << 18);
        let a = (c.g[S1][0] as u32).wrapping_add(0x128);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[T1][0] = sx((c.g[S2][0] as u32) << 10);
        let a = (c.g[S1][0] as u32).wrapping_add(0x134);
        c.g[A2][0] = Addr(a).lw(c);
        c.g[A3][0] = sx((c.g[S3][0] as u32) << 12);
        let a = (c.g[S1][0] as u32).wrapping_add(0x138);
        c.g[V0][0] = Addr(a).lw(c);
        c.g[A1][0] = sx((c.g[S4][0] as u32) << 12);
        c.g[A0][0] &= 0xfff;
        c.g[V1][0] &= 0xfff;
        c.g[A3][0] |= c.g[A0][0];
        c.g[A1][0] |= c.g[V1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        let a = (c.g[S1][0] as u32).wrapping_add(0x138);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        let a = (c.g[S1][0] as u32).wrapping_add(0x124);
        let v = c.g[A3][0];
        Addr(a).sw(c, v);
        let a = (c.g[S1][0] as u32).wrapping_add(0x128);
        let v = c.g[A1][0];
        Addr(a).sw(c, v);
        let a = (c.g[S1][0] as u32).wrapping_add(0x134);
        let v = c.g[A2][0];
        Addr(a).sw(c, v);
        f.restore_ra(c, 0x60);
        let a = f.at(c, 0x50);
        c.g[S5][0] = Addr(a).ld(c);
        let a = f.at(c, 0x40);
        c.g[S4][0] = Addr(a).ld(c);
        let a = f.at(c, 0x30);
        c.g[S3][0] = Addr(a).ld(c);
        let a = f.at(c, 0x20);
        c.g[S2][0] = Addr(a).ld(c);
        let a = f.at(c, 0x10);
        c.g[S1][0] = Addr(a).ld(c);
        let a = f.at(c, 0x00);
        c.g[S0][0] = Addr(a).ld(c);
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_SequenceDisplayExtension(dec)` (0x403ce0): read the display
/// extension: a 1-bit flag, then three 8-bit colour fields (+0x144) when set,
/// then the 14-bit frame width (+0x148), a 1-bit flag and a 14-bit height
/// (+0x14c).
pub fn libmpeg_sequence_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 3;
        let a = f.at(c, 0x00);
        let v = c.g[S0][0];
        c.sd(a, v);
        f.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d04)?;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            for ret_addr in [0x0040_3d14u32, 0x0040_3d20, 0x0040_3d2c] {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 8;
                LIBMPEG_CALLEE_GET_BITS.call(c, ret_addr)?;
            }
            let a = (c.g[S0][0] as u32).wrapping_add(0x144);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xe;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d3c)?;
        let a = (c.g[S0][0] as u32).wrapping_add(0x148);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xe;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d58)?;
        let a = (c.g[S0][0] as u32).wrapping_add(0x14c);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        f.restore_ra(c, 0x10);
        let a = f.at(c, 0x00);
        c.g[S0][0] = Addr(a).ld(c);
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_RestartDmaCallback(dec)` (0x403dd8): restart the IPU source
/// DMA of the decoder's stream block (`dec+0x40`, at +0x4c), return 1.
pub fn libmpeg_restart_dma_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let f = Frame::open(c, 0x10);
        f.save_ra(c, 0x00);
        let a = (c.g[A0][0] as u32).wrapping_add(0x40);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
        LIBMPEG_CALLEE_IPU_RESTART_DMA.call(c, 0x0040_3dec)?;
        f.restore_ra(c, 0x00);
        c.g[V0][0] = 1;
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_SetIpuMpeg1Bit(flag)` (0x405b48): write the argument's low
/// bits (`flag << 23`) into the IPU control word at `0x10002010`, keeping
/// every other bit except bit 23, which takes the flag.
pub fn libmpeg_set_ipu_mpeg1_bit(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A1][0] = 0x1000_0000;
    c.g[V1][0] = 0xffff_ffff_ff7f_0000;
    c.g[A1][0] |= 0x2010;
    c.g[V1][0] |= 0xffff;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 23);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A0][0];
    let a = c.g[A1][0] as u32;
    let v = c.g[V0][0];
    Addr(a).sw(c, v);
    ret(c);
}

/// `libmpeg_IpuVdec` (0x0040_6f78): the IPU variable-length decode; the code
/// it decodes comes back in v0.
pub(crate) const MPEG_IPU_VDEC: Callee = Callee {
    addr: 0x0040_6f78,
    f: ee::s15::f_00406f78,
};

/// `libmpeg_ShowBits` (0x0040_70f0): the next bits of the stream, not consumed.
pub(crate) const MPEG_SHOW_BITS: Callee = Callee {
    addr: 0x0040_70f0,
    f: ee::s15::f_004070f0,
};

/// `libmpeg_FlushBits` (0x0040_71f8): consumes bits of the stream.
pub(crate) const MPEG_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::f_004071f8,
};

/// `libmpeg_Errorf` (0x0040_3470): reports a decode error.
pub(crate) const MPEG_ERRORF: Callee = Callee {
    addr: 0x0040_3470,
    f: ee::s15::f_00403470,
};

/// `libmpeg_DecodeDmVector(dec)` (0x0040_5e10): decodes one dual-prime
/// differential motion vector. It is the IPU variable-length decode with
/// table 3 (`a1 = 3`) on the decoder state, and returns what that decoded.
pub fn lib_mpeg_decode_dm_vector(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x3;
        MPEG_IPU_VDEC.call(c, 0x0040_5e20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// H1 of the reference's `calc_DMV`: `(x + (x > 0)) >> 1` on the 32-bit word,
/// with `pos` = `x > 0` (the 64-bit sign test of the register).
pub(crate) fn dp_half(x: u32, pos: bool) -> u32 {
    ((x.wrapping_add(pos as u32)) as i32 >> 1) as u32
}

/// H3 of the reference's `calc_DMV`: `(3x + (x > 0)) >> 1`.
pub(crate) fn dp_third(x: u32, pos: bool) -> u32 {
    ((x.wrapping_mul(3).wrapping_add(pos as u32)) as i32 >> 1) as u32
}

/// Frame picture with the bottom field first (`dec+0x178 == 0`): the vectors
/// come out as `H3(a)`, `H3(b)`, `H1(a)`, `H1(b)` with the field offsets.
pub(crate) fn dp_bottom_first(c: &mut Cpu, out: u32, inp: u32, a: u64, b: u64) {
    let (a32, b32) = (a as u32, b as u32);
    let (a_pos, b_pos) = ((a as i64) > 0, (b as i64) > 0);
    c.cycles += 3;
    let in0 = Addr(inp).lw(c);
    if a_pos {
        c.cycles += 1;
    }
    c.cycles += 7;
    Addr(out).sw(c, sx(dp_third(a32, a_pos).wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    if b_pos {
        c.cycles += 1;
    }
    c.cycles += 6;
    let out1 = dp_third(b32, b_pos)
        .wrapping_add(in1 as u32)
        .wrapping_sub(1);
    Addr(out.wrapping_add(4)).sw(c, sx(out1));
    let in0 = Addr(inp).lw(c);
    c.cycles += if a_pos { 3 } else { 1 };
    let h1a = dp_half(a32, a_pos);
    c.cycles += 4;
    Addr(out.wrapping_add(8)).sw(c, sx(h1a.wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    c.cycles += if b_pos { 3 } else { 2 };
    let h1b = dp_half(b32, b_pos);
    c.cycles += 4;
    let out3 = sx(h1b.wrapping_add(in1 as u32).wrapping_add(1));
    Addr(out.wrapping_add(0xc)).sw(c, out3);
    c.g[V0][0] = out3;
    c.g[V1][0] = in0;
    c.g[A2][0] = in1;
}

/// Frame picture with the top field first (`dec+0x178 != 0`): the vectors
/// come out as `H1(a)`, `H1(b)`, `H3(a)`, `H3(b)` with the field offsets.
pub(crate) fn dp_top_first(c: &mut Cpu, out: u32, inp: u32, a: u64, b: u64) {
    let (a32, b32) = (a as u32, b as u32);
    let (a_pos, b_pos) = ((a as i64) > 0, (b as i64) > 0);
    c.cycles += 2;
    let in0 = Addr(inp).lw(c);
    c.cycles += if a_pos { 3 } else { 1 };
    let h1a = dp_half(a32, a_pos);
    c.cycles += 4;
    Addr(out).sw(c, sx(h1a.wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    c.cycles += if b_pos { 3 } else { 1 };
    let h1b = dp_half(b32, b_pos);
    c.cycles += 7;
    let out1 = h1b.wrapping_add(in1 as u32).wrapping_sub(1);
    Addr(out.wrapping_add(4)).sw(c, sx(out1));
    let in0 = Addr(inp).lw(c);
    if a_pos {
        c.cycles += 1;
    }
    c.cycles += 7;
    Addr(out.wrapping_add(8)).sw(c, sx(dp_third(a32, a_pos).wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    if b_pos {
        c.cycles += 1;
    }
    c.cycles += 1;
    let h3b = dp_third(b32, b_pos);
    c.cycles += 4;
    let out3 = sx(h3b.wrapping_add(in1 as u32).wrapping_add(1));
    Addr(out.wrapping_add(0xc)).sw(c, out3);
    c.g[V0][0] = out3;
    c.g[V1][0] = in0;
    c.g[A2][0] = in1;
}

/// Field picture (`dec+0x174 != 3`): one vector `H1(a)`, `H1(b)` corrected
/// by -1 when the picture structure is 1 (top field) and by +1 otherwise.
pub(crate) fn dp_field(c: &mut Cpu, dec: u32, out: u32, inp: u32, a: u64, b: u64) {
    let (a32, b32) = (a as u32, b as u32);
    let (a_pos, b_pos) = ((a as i64) > 0, (b as i64) > 0);
    c.cycles += 2;
    let in0 = Addr(inp).lw(c);
    c.cycles += if a_pos { 3 } else { 1 };
    let h1a = dp_half(a32, a_pos);
    c.cycles += 4;
    Addr(out).sw(c, sx(h1a.wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    c.cycles += if b_pos { 3 } else { 1 };
    let h1b = dp_half(b32, b_pos);
    c.cycles += 6;
    let sum = sx(h1b.wrapping_add(in1 as u32));
    Addr(out.wrapping_add(4)).sw(c, sum);
    c.g[A2][0] = sum;
    c.g[V1][0] = 1;
    let structure = Addr(dec.wrapping_add(0x174)).lw(c);
    let not_top = structure != c.g[V1][0];
    c.g[V0][0] = sx((sum as u32).wrapping_add(1));
    if not_top {
        c.cycles += 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx((sum as u32).wrapping_sub(1));
    }
    let v = c.g[V0][0];
    Addr(out.wrapping_add(4)).sw(c, v);
}

/// `libmpeg_DualPrimeArithmetic(dec, out, in, a, b)` (0x0040_5e30): the
/// dual-prime derived motion vectors (MSSG's `calc_DMV`). `dec` (a0) is the
/// decoder state: the picture structure at `+0x174` (3 = frame) and the
/// top-field-first flag at `+0x178`. `out` (a1) receives four words, `inp`
/// (a2) holds the differential vector, `a` (a3) and `b` (t0) the horizontal
/// and vertical motion vectors. Frame pictures derive both field vectors,
/// scaled by 1/2 and 3/2 with rounding away from zero, in the order the
/// top-field-first flag gives, with the vertical -1 / +1 field corrections.
pub fn lib_mpeg_dual_prime_arithmetic(c: &mut Cpu) {
    let dec = c.g[A0][0] as u32;
    let out = c.g[A1][0] as u32;
    let inp = c.g[A2][0] as u32;
    let a = c.g[A3][0];
    let b = c.g[T0][0];
    c.cycles += 4;
    let structure = Addr(dec.wrapping_add(0x174)).lw(c);
    if structure == 3 {
        c.cycles += 3;
        if Addr(dec.wrapping_add(0x178)).lw(c) == 0 {
            dp_bottom_first(c, out, inp, a, b);
        } else {
            dp_top_first(c, out, inp, a, b);
        }
    } else {
        dp_field(c, dec, out, inp, a, b);
    }
    ret(c);
}

/// What one pass of the MacroblockAddressIncrement loop decided.
#[derive(Clone, Copy)]
pub(crate) enum MbiStep {
    /// Another pass (`again` set).
    Again,
    /// The code's value is added to the sum and the loop ends.
    Done,
    /// `Errorf` was reported: the function returns 1 at once.
    Errored,
}

/// Saves a register at `off(sp)` with `sd`, the width the generated code uses.
pub(crate) fn mbi_save(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    let v = c.g[reg][0];
    c.sd(a, v);
}

/// Restores a register from `off(sp)` with `ld`.
pub(crate) fn mbi_restore(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    c.g[reg][0] = c.ld(a);
}

/// One pass of the loop: the IPU VDEC call and the switch on the code it
/// returns (the generated body's blocks, one per case).
pub(crate) fn mbi_pass(c: &mut Cpu) -> Result<MbiStep, Unwound> {
    c.cycles += 5;
    c.g[A1][0] = 0;
    MPEG_IPU_VDEC.call(c, 0x0040_6008)?;
    c.g[S0][0] = c.g[V0][0];
    // Case 34 (stuffing): another pass.
    if c.g[S0][0] == c.g[S6][0] {
        c.cycles += 2;
        c.g[V0][0] = 1;
        return Ok(MbiStep::Again);
    }
    c.g[V0][0] = (c.g[S0][0] < 0x23u64) as u64;
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // Codes from 35 up: 35 is the escape (+33, another pass); the rest
        // take the default arm.
        c.cycles += 2;
        if c.g[S0][0] != c.g[S5][0] {
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            return Ok(MbiStep::Done);
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x21));
        return Ok(MbiStep::Again);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S1][0];
    if c.g[S0][0] != 0 {
        // Codes 1..33: the default arm.
        c.cycles += 2;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
        return Ok(MbiStep::Done);
    }
    // Code 0: look at the next 11 bits. The 11-bit stuffing code (15) is
    // skipped when the flag at +0x848 is set; anything else is an error.
    c.cycles += 6;
    c.g[A1][0] = 0xb;
    MPEG_SHOW_BITS.call(c, 0x0040_6048)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x848)).lw(c);
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7ed0));
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[A2][0] = c.g[S0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = c.g[S0][0];
        if c.g[V1][0] == c.g[S3][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0xb;
            MPEG_FLUSH_BITS.call(c, 0x0040_606c)?;
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(MbiStep::Again);
        }
    }
    // Error: report it, mark the state, return 1.
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    MPEG_ERRORF.call(c, 0x0040_6080)?;
    let flag = c.g[S7][0];
    Addr((c.g[S1][0] as u32).wrapping_add(0x11c)).sw(c, flag);
    c.g[V0][0] = 1;
    Ok(MbiStep::Errored)
}

/// `libmpeg_MacroblockAddressIncrement(dec)` (0x0040_5fb8): reads the
/// macroblock address increment from the stream: the variable-length codes
/// are summed (34 is stuffing, 35 the escape worth 33), an error code is
/// reported through `Errorf` and returns 1. Returns the sum.
pub fn lib_mpeg_macroblock_address_increment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        mbi_save(c, frame, S7, 0x70);
        mbi_save(c, frame, S6, 0x60);
        c.g[S7][0] = 1;
        mbi_save(c, frame, S5, 0x50);
        c.g[S6][0] = 0x22;
        mbi_save(c, frame, S4, 0x40);
        c.g[S5][0] = 0x23;
        mbi_save(c, frame, S3, 0x30);
        c.g[S4][0] = 0x0049_0000;
        mbi_save(c, frame, S2, 0x20);
        c.g[S3][0] = 0xf;
        mbi_save(c, frame, S1, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A0][0];
        mbi_save(c, frame, S0, 0);
        c.g[A0][0] = c.g[S1][0];

        loop {
            match mbi_pass(c)? {
                MbiStep::Again => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_6000);
                    }
                }
                MbiStep::Done => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    break;
                }
                MbiStep::Errored => break,
            }
        }

        c.cycles += 11;
        frame.restore_ra(c, 0x80);
        mbi_restore(c, frame, S7, 0x70);
        mbi_restore(c, frame, S6, 0x60);
        mbi_restore(c, frame, S5, 0x50);
        mbi_restore(c, frame, S4, 0x40);
        mbi_restore(c, frame, S3, 0x30);
        mbi_restore(c, frame, S2, 0x20);
        mbi_restore(c, frame, S1, 0x10);
        mbi_restore(c, frame, S0, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_DecodeSlice(DecState* dec, int n)` (0x00406318): decodes one
/// MPEG slice of `n` macroblocks. Reads the slice header (`SliceHeader`, a
/// non-zero result is returned at once), then loops per macroblock: waits for
/// the IPU (`WaitBdec`; on failure returns 4 when the abort flag `dec[0x10a0]`
/// was set, else 2), reads the address increment when it is 0 (`ShowBits(23)`,
/// `GetMbIncrement`; returns 3/1 on a flagged error `dec[0x11c]`), reports a
/// macroblock past the end (`Errorf`, 2), decodes a coded macroblock
/// (`DecodeMacroblock`, increment 1) or a skipped one (`SkippedMacroblock`),
/// sets it up (`SetupMacroblock`) and stores the previous one's prediction
/// (`AddPredAndStore`), then toggles the buffer index `dec[0x810]`. Returns 0
/// when all `n` macroblocks are done. Converted from the generated body (a
/// `match pc` body: every register, cycle and call kept); any other `pc` (a
/// resume after an unwound callee or event) runs the generated body (at
/// install: its `__generated` alias).
pub fn libmpeg_decode_slice(c: &mut Cpu) {
    if c.pc != 0x00406318 {
        return ee::s15::f_00406318__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00406318;
        loop {
            match pc {
                0x00406318 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S3][0]);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S0][0]);
                    c.g[S3][0] = c.g[A1][0];
                    c.g[S0][0] = c.g[A0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x44), 0u64);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[T0][0] = c.g[SP][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S2][0]);
                    c.g[RA][0] = sx(0x00406350);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                    Callee {
                        addr: 0x004061e8,
                        f: ee::s15::libmpeg_SliceHeader,
                    }
                    .call(c, 0x00406350)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x004064f8;
                        continue;
                    }
                    pc = 0x00406358;
                    continue;
                }
                0x00406358 => {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                    pc = 0x00406360;
                    continue;
                }
                0x00406360 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        pc = 0x00406378;
                        continue;
                    }
                    pc = 0x00406370;
                    continue;
                }
                0x00406370 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x004064f4;
                        continue;
                    }
                    pc = 0x00406378;
                    continue;
                }
                0x00406378 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0x140;
                    c.g[A0][0] = c.g[S0][0];
                    c.mult(0, 5, 2, 3);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x00406390);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6cc), 0u64);
                    Callee {
                        addr: 0x00405b70,
                        f: ee::s15::libmpeg_WaitBdec,
                    }
                    .call(c, 0x00406390)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                    if t {
                        pc = 0x004063ac;
                        continue;
                    }
                    pc = 0x00406398;
                    continue;
                }
                0x00406398 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10a0));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x00406414;
                        continue;
                    }
                    pc = 0x004063a4;
                    continue;
                }
                0x004063a4 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10a0), 0u64);
                    if t {
                        pc = 0x004064f4;
                        continue;
                    }
                    pc = 0x004063ac;
                    continue;
                }
                0x004063ac => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x004063f8;
                        continue;
                    }
                    pc = 0x004063b4;
                    continue;
                }
                0x004063b4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x004063c0);
                    c.g[A1][0] = 0x17;
                    Callee {
                        addr: 0x004070f0,
                        f: ee::s15::libmpeg_ShowBits,
                    }
                    .call(c, 0x004063c0)?;

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                        pc = 0x004063d8;
                        continue;
                    }
                    pc = 0x004063c8;
                    continue;
                }
                0x004063c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x004063e0;
                        continue;
                    }
                    pc = 0x004063d4;
                    continue;
                }
                0x004063d4 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                    pc = 0x004063d8;
                    continue;
                }
                0x004063d8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x004064f4;
                        continue;
                    }
                    pc = 0x004063e0;
                    continue;
                }
                0x004063e0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x004063e8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00405fb8,
                        f: ee::s15::libmpeg_MacroblockAddressIncrement,
                    }
                    .call(c, 0x004063e8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                    let t = c.g[V1][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                    if t {
                        pc = 0x00406454;
                        continue;
                    }
                    pc = 0x004063f4;
                    continue;
                }
                0x004063f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x004063f8;
                    continue;
                }
                0x004063f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                    if t {
                        pc = 0x0040641c;
                        continue;
                    }
                    pc = 0x00406404;
                    continue;
                }
                0x00406404 => {
                    c.cycles += 4;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00406414);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f70));
                    Callee {
                        addr: 0x004034a8,
                        f: ee::s15::libmpeg_Error,
                    }
                    .call(c, 0x00406414)?;
                    pc = 0x00406414;
                    continue;
                }
                0x00406414 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x004064f4;
                        continue;
                    }
                    pc = 0x0040641c;
                    continue;
                }
                0x0040641c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x00406460;
                        continue;
                    }
                    pc = 0x00406428;
                    continue;
                }
                0x00406428 => {
                    c.cycles += 11;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0040644c);
                    c.g[T2][0] = c.g[S1][0];
                    Callee {
                        addr: 0x004065d0,
                        f: ee::s15::libmpeg_DecodeMacroblock,
                    }
                    .call(c, 0x0040644c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x00406484;
                        continue;
                    }
                    pc = 0x00406454;
                    continue;
                }
                0x00406454 => {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x004064f4;
                        continue;
                    }
                    pc = 0x00406460;
                    continue;
                }
                0x00406460 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.g[A3][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00406478);
                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                    Callee {
                        addr: 0x00406510,
                        f: ee::s15::libmpeg_SkippedMacroblock,
                    }
                    .call(c, 0x00406478)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x004064ac;
                        continue;
                    }
                    pc = 0x00406480;
                    continue;
                }
                0x00406480 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x00406484;
                    continue;
                }
                0x00406484 => {
                    c.cycles += 10;
                    c.g[T2][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[T3][0] = c.g[S1][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.g[RA][0] = sx(0x004064a4);
                    c.g[T1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x00403e00,
                        f: ee::s15::libmpeg_SetupMacroblock,
                    }
                    .call(c, 0x004064a4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x004064b4;
                        continue;
                    }
                    pc = 0x004064ac;
                    continue;
                }
                0x004064ac => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                    if t {
                        if back_edge_unwinds(c, 0x00406414) {
                            return Ok(());
                        }
                        pc = 0x00406414;
                        continue;
                    }
                    pc = 0x004064b4;
                    continue;
                }
                0x004064b4 => {
                    c.cycles += 2;

                    if c.g[A0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        pc = 0x004064d4;
                        continue;
                    }
                    pc = 0x004064bc;
                    continue;
                }
                0x004064bc => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x004064cc);
                    c.g[A1][0] = c.g[A1][0] ^ 0x1;
                    Callee {
                        addr: 0x00404b78,
                        f: ee::s15::libmpeg_AddPredAndStore,
                    }
                    .call(c, 0x004064cc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                    pc = 0x004064d4;
                    continue;
                }
                0x004064d4 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x810), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                    if t {
                        if back_edge_unwinds(c, 0x00406360) {
                            return Ok(());
                        }
                        pc = 0x00406360;
                        continue;
                    }
                    pc = 0x004064f4;
                    continue;
                }
                0x004064f4 => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    pc = 0x004064f8;
                    continue;
                }
                0x004064f8 => {
                    c.cycles += 6;
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00406318, 0x00406510);
                    return Ok(());
                }
            }
        }
    });
}

/// `int libmpeg_DecodeMacroblock(dec, int* mbtype, int* motion_type,
/// int* dct_type, int* pmv, int* field_select, int* dm_vector)` (0x004065d0):
/// parses one coded macroblock header. Sets the IPU quantiser-scale field of
/// `IPU_CTRL`, reads the macroblock type (the macroblock-type reader, 0 reports an
/// error and sets `dec[0x11c]`), the motion type and DCT type bits, the
/// quantiser code, then the forward / backward motion vectors (the MPEG-1
/// reader `0x406ca0` or the MPEG-2 reader `0x406b00`, chosen by `dec[0x848]`),
/// and for blocks with coefficients the IPU command (clear the block buffer,
/// wait, issue the macroblock command). Returns 0 on an error
/// (`dec[0x11c]` set) else 1, after resetting the motion-vector predictors
/// for intra and skip cases. Converted from the generated labelled-block body.
pub fn libmpeg_decode_macroblock(c: &mut Cpu) {
    guest(c, |c| {
        'b_406a48: {
            'b_406678: {
                c.cycles += 34;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[T3][0] = 0x10000000;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S6][0]);
                c.g[T3][0] = c.g[T3][0] | 0x2010;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S5][0]);
                c.g[V0][0] = 0xfffffffff8ff0000;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S2][0]);
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S1][0]);
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S0][0]);
                c.g[S5][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[FP][0]);
                c.g[S1][0] = c.g[T0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S7][0]);
                c.g[S6][0] = c.g[T2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S4][0]);
                c.g[A1][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S3][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                c.g[V1][0] = c.lw(c.g[T3][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.sw(c.g[T3][0] as u32, c.g[V1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A3][0]);
                c.g[RA][0] = sx(0x0040664c);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[T1][0]);
                Callee {
                    addr: 0x00406f78,
                    f: ee::s15::libmpeg_IpuVdec,
                }
                .call(c, 0x0040664c)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64;
                c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                if t {
                    break 'b_406678;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00406668);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7fc0));
                    Callee {
                        addr: 0x004034a8,
                        f: ee::s15::libmpeg_Error,
                    }
                    .call(c, 0x00406668)?;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0u64;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                    if t {
                        break 'b_406a48;
                    } else {
                        break 'b_406678;
                    }
                }
            }
            'b_4066e4: {
                'b_4066e0: {
                    'b_4066b4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V1][0] & 0xc;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x3;
                        if t {
                            break 'b_4066b4;
                        } else {
                            'b_4066a4: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_4066a4;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_4066a4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                                        if t {
                                            break 'b_4066e0;
                                        } else {
                                            break 'b_4066a4;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x004066ac);
                            c.g[A1][0] = 0x2;
                            Callee {
                                addr: 0x004072e8,
                                f: ee::s15::libmpeg_GetBits,
                            }
                            .call(c, 0x004066ac)?;
                            let t = true;
                            c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_4066e0;
                            } else {
                                break 'b_4066b4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0x1;
                    if c.g[V0][0] == 0u64 {
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                        break 'b_4066e4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'b_4066e0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                            c.g[A0][0] = 0x2;
                            c.g[V0][0] = c.g[V0][0] ^ 0x3;
                            if c.g[V0][0] == 0 {
                                c.g[V1][0] = c.g[A0][0];
                            }
                            c.sw(c.g[S5][0] as u32, c.g[V1][0]);
                            break 'b_4066e0;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                break 'b_4066e4;
            }
            'b_406720: {
                'b_40670c: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[A1][0] = c.lw(c.g[S5][0] as u32);
                    if t {
                        break 'b_40670c;
                    } else {
                        c.cycles += 7;
                        c.g[S3][0] = 0x1;
                        c.g[V0][0] = 0x2;
                        c.g[V1][0] = c.g[A1][0] ^ 0x1;
                        c.g[A0][0] = c.g[A1][0] ^ 0x2;
                        if c.g[V1][0] == 0 {
                            c.g[S3][0] = c.g[V0][0];
                        }
                        let t = true;
                        c.g[S4][0] = (c.g[A0][0] < 0x1u64) as u64;
                        if t {
                            break 'b_406720;
                        } else {
                            break 'b_40670c;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = 0x2;
                c.g[S3][0] = 0x1;
                c.g[S4][0] = 0u64;
                c.g[V0][0] = c.g[A1][0] ^ 0x2;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = c.g[V1][0];
                }
                break 'b_406720;
            }
            'b_406738: {
                c.cycles += 4;
                c.g[V0][0] = c.g[A1][0] ^ 0x3;
                c.g[S7][0] = 0u64;
                let t = c.g[S4][0] != 0u64;
                c.g[FP][0] = (c.g[V0][0] < 0x1u64) as u64;
                if t {
                    break 'b_406738;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0] ^ 0x3;
                    c.g[S7][0] = (c.g[V0][0] < 0x1u64) as u64;
                    break 'b_406738;
                }
            }
            'b_406778: {
                'b_406774: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_406774;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_406774;
                        } else {
                            'b_406770: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x3;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_406770;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00406768);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x004072e8,
                                        f: ee::s15::libmpeg_GetBits,
                                    }
                                    .call(c, 0x00406768)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_406778;
                                    } else {
                                        break 'b_406770;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0u64;
                            break 'b_406774;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_406778;
            }
            'b_40679c: {
                c.cycles += 5;
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.g[V0][0] = c.g[V1][0] & 0x10;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_40679c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00406794);
                    c.g[A1][0] = 0x5;
                    Callee {
                        addr: 0x004072e8,
                        f: ee::s15::libmpeg_GetBits,
                    }
                    .call(c, 0x00406794)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1b4), c.g[V0][0]);
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    break 'b_40679c;
                }
            }
            'b_406834: {
                'b_4067c4: {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0x8;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
                        break 'b_4067c4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V1][0] & 0x1;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                            break 'b_406834;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                                break 'b_406834;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
                                break 'b_4067c4;
                            }
                        }
                    }
                }
                'b_40680c: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_40680c;
                    } else {
                        c.cycles += 16;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x164));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[FP][0]);
                        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
                        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                        c.g[A2][0] = c.g[S6][0];
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S7][0]);
                        c.g[T0][0] = 0u64;
                        c.g[T1][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00406804);
                        c.g[T2][0] = c.g[S4][0];
                        Callee {
                            addr: 0x00406b00,
                            f: ee::s15::libmpeg_MotionVectors,
                        }
                        .call(c, 0x00406804)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                        if t {
                            break 'b_406834;
                        } else {
                            break 'b_40680c;
                        }
                    }
                }
                c.cycles += 10;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x158));
                c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x154));
                c.g[A1][0] = c.g[S1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                c.g[A2][0] = c.g[S6][0];
                c.g[T1][0] = 0u64;
                c.g[T0][0] = c.g[A3][0];
                c.g[RA][0] = sx(0x00406830);
                c.g[T2][0] = 0u64;
                Callee {
                    addr: 0x00406ca0,
                    f: ee::s15::libmpeg_MotionVector,
                }
                .call(c, 0x00406830)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                break 'b_406834;
            }
            c.cycles += 2;
            let t = c.g[V1][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_406a48;
            } else {
                'b_4068c0: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        break 'b_4068c0;
                    } else {
                        'b_406898: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A2][0] = c.g[S6][0];
                            if t {
                                break 'b_406898;
                            } else {
                                c.cycles += 16;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x170));
                                c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                                c.g[T1][0] = c.g[S3][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S7][0]);
                                c.g[T2][0] = c.g[S4][0];
                                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x00406890);
                                c.g[T0][0] = 0x1;
                                Callee {
                                    addr: 0x00406b00,
                                    f: ee::s15::libmpeg_MotionVectors,
                                }
                                .call(c, 0x00406890)?;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                                if t {
                                    break 'b_4068c0;
                                } else {
                                    break 'b_406898;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x160));
                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x15c));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[T1][0] = 0u64;
                        c.g[T0][0] = c.g[A3][0];
                        c.g[RA][0] = sx(0x004068bc);
                        c.g[T2][0] = 0u64;
                        Callee {
                            addr: 0x00406ca0,
                            f: ee::s15::libmpeg_MotionVector,
                        }
                        .call(c, 0x004068bc)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                        break 'b_4068c0;
                    }
                }
                c.cycles += 2;
                let t = c.g[V1][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_406a48;
                } else {
                    'b_4068f4: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = c.g[V1][0] & 0x1;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[V1][0] & 0x3;
                        if t {
                            break 'b_4068f4;
                        } else {
                            'b_4068f0: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_4068f0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x004068ec);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x004071f8,
                                        f: ee::s15::libmpeg_FlushBits,
                                    }
                                    .call(c, 0x004068ec)?;
                                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                    break 'b_4068f0;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.g[V1][0] & 0x3;
                            break 'b_4068f4;
                        }
                    }
                    'b_406980: {
                        'b_406968: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x140;
                            if t {
                                break 'b_406968;
                            } else {
                                c.cycles += 27;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                                c.g[A1][0] = 0x300;
                                c.mult(0, 4, 2, 3);
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                c.g[RA][0] = sx(0x00406914);
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x594));
                                Callee {
                                    addr: 0x00408af8,
                                    f: ee::s15::libmpeg_IpuFromDmaStart,
                                }
                                .call(c, 0x00406914)?;
                                c.g[RA][0] = sx(0x0040691c);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00406e20,
                                    f: ee::s15::libmpeg_IpuWaitBusy,
                                }
                                .call(c, 0x0040691c)?;
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                c.g[A3][0] = 0x20000000;
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b0));
                                c.g[A1][0] = c.g[A1][0] & 0x1;
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[A1][0] = sx((c.g[A1][0] as u32) << 27);
                                c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 26);
                                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                                c.g[A1][0] = c.g[A1][0] | c.g[A2][0];
                                c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
                                c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 25);
                                c.g[RA][0] = sx(0x00406960);
                                c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                Callee {
                                    addr: 0x00406df0,
                                    f: ee::s15::libmpeg_IpuCommand,
                                }
                                .call(c, 0x00406960)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                                if t {
                                    break 'b_406980;
                                } else {
                                    break 'b_406968;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[A0][0] = 0x1;
                        c.mult(0, 5, 2, 3);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6cc), c.g[A0][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                        break 'b_406980;
                    }
                    'b_406990: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1b0), 0u64);
                        if t {
                            break 'b_406990;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_406a48;
                            } else {
                                break 'b_406990;
                            }
                        }
                    }
                    'b_4069e8: {
                        'b_4069bc: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
                                break 'b_4069bc;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1b0), c.g[V1][0]);
                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                                    break 'b_4069e8;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
                                    break 'b_4069bc;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                            break 'b_4069e8;
                        } else {
                            c.cycles += 9;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                            c.sw(c.g[S1][0] as u32, 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                            break 'b_4069e8;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_406a48;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = c.g[V0][0] & 0x9;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_406a48;
                        } else {
                            'b_406a44: {
                                'b_406a2c: {
                                    c.cycles += 8;
                                    c.sw(c.g[S1][0] as u32, 0u64);
                                    c.g[V1][0] = 0x3;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V1][0] = 0x1;
                                    if t {
                                        break 'b_406a2c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.sw(c.g[S5][0] as u32, c.g[A0][0]);
                                        if t {
                                            break 'b_406a44;
                                        } else {
                                            break 'b_406a2c;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.sw(c.g[S5][0] as u32, c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                c.sw(c.g[T0][0] as u32, c.g[V0][0]);
                                break 'b_406a44;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_406a48;
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void libmpeg_CopyOutputRaw(DecState* dec, Picture* pic)` (0x00408278):
/// copies a decoded raw picture out through the scratchpad: for each of the
/// 1 or 2 fields and each macroblock row `pic[3]`, DMA `qwc` quadwords of
/// `pic[0]` into the scratchpad (channel 9, `0x1000d400`), wait, DMA them
/// back out to the destination row at `dec[0xd8]` (channel 8, `0x1000d000`),
/// wait for the queue to drain, and advance the row by the stride from
/// `dec[0xe0]`. Interrupts are masked around each kick (`DIntr`/`EIntr`).
/// Converted from the generated labelled-block body.
pub fn libmpeg_copy_output_raw(c: &mut Cpu) {
    guest(c, |c| {
        'b_40834c: {
            'b_408324: {
                'b_4082f8: {
                    'b_4082ec: {
                        c.cycles += 24;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                        c.g[V0][0] = 0xfff0000;
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                        c.g[S6][0] = c.g[A1][0];
                        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                        c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V1][0] = c.lw(c.g[S6][0] as u32);
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd8));
                        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x174));
                        c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
                        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
                        c.g[V1][0] = 0x3;
                        let t = c.g[A1][0] == c.g[V1][0];
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                        if t {
                            break 'b_4082ec;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe0));
                            let t = c.g[A0][0] != 0u64;
                            c.g[V1][0] = c.g[A0][0];
                            if t {
                                break 'b_408324;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_4082f8;
                                } else {
                                    break 'b_4082ec;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A3][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe0));
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                    break 'b_4082f8;
                }
                'b_408318: {
                    'b_408314: {
                        c.cycles += 4;
                        c.g[A0][0] = 0x180;
                        c.mult(0, 21, 2, 4);
                        let t = c.g[V1][0] == 0u64;
                        c.g[S4][0] = sx(((c.g[S5][0] as u32) as i32 >> 4) as u32);
                        if t {
                            break 'b_408314;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
                            let t = true;
                            c.mult(0, 30, 2, 4);
                            if t {
                                break 'b_408318;
                            } else {
                                break 'b_408314;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[FP][0] = c.g[S5][0];
                    break 'b_408318;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = true;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                if t {
                    break 'b_40834c;
                } else {
                    break 'b_408324;
                }
            }
            c.cycles += 10;
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = 0x180;
            c.g[V1][0] = 0xc0;
            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 4) as u32);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.mult(0, 30, 4, 3);
            c.mult(1, 21, 2, 5);
            c.g[V1][0] = 0x2;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[S4][0] = sx(((c.g[S5][0] as u32) as i32 >> 4) as u32);
            break 'b_40834c;
        }
        'b_4084d8: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_4084d8;
            } else {
                c.cycles += 2;
                c.g[A2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                'l_408360: loop {
                    'b_4084ac: {
                        'b_4084a4: {
                            c.cycles += 3;
                            c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            let t = (c.g[A2][0] as i64) <= 0;
                            c.g[S0][0] = 0u64;
                            if t {
                                break 'b_4084a4;
                            } else {
                                c.cycles += 1;
                                c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                'l_408370: loop {
                                    'b_4083c0: {
                                        c.cycles += 17;
                                        c.g[RA][0] = sx(0x00408378);
                                        Callee {
                                            addr: 0x0042c078,
                                            f: ee::s15::DIntr,
                                        }
                                        .call(c, 0x00408378)?;
                                        c.g[V1][0] = 0x10000000;
                                        c.g[A1][0] = 0x10000000;
                                        c.g[V1][0] = c.g[V1][0] | 0xd480;
                                        c.g[A1][0] = c.g[A1][0] | 0xd410;
                                        c.sw(c.g[V1][0] as u32, 0u64);
                                        c.g[A0][0] = 0x10000000;
                                        c.sw(c.g[A1][0] as u32, c.g[S2][0]);
                                        c.g[A0][0] = c.g[A0][0] | 0xd420;
                                        c.g[A1][0] = 0x10000000;
                                        c.sw(c.g[A0][0] as u32, c.g[S4][0]);
                                        c.g[A1][0] = c.g[A1][0] | 0xd400;
                                        c.g[V1][0] = 0x101;
                                        c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V1][0] = 0x10000000;
                                        if t {
                                            break 'b_4083c0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[RA][0] = sx(0x004083bc);
                                            Callee {
                                                addr: 0x0042c0c0,
                                                f: ee::s15::EIntr,
                                            }
                                            .call(c, 0x004083bc)?;
                                            c.g[V1][0] = 0x10000000;
                                            break 'b_4083c0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S3][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[S2][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                                    c.g[V1][0] = c.g[V1][0] | 0xd400;
                                    'l_4083d0: loop {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                        c.g[V0][0] = c.g[V0][0] & 0x100;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x004083d0);
                                            }
                                            continue 'l_4083d0;
                                        } else {
                                            'b_40843c: {
                                                c.cycles += 17;
                                                c.g[RA][0] = sx(0x004083f4);
                                                Callee {
                                                    addr: 0x0042c078,
                                                    f: ee::s15::DIntr,
                                                }
                                                .call(c, 0x004083f4)?;
                                                c.g[V1][0] = 0x10000000;
                                                c.g[A1][0] = 0x10000000;
                                                c.g[V1][0] = c.g[V1][0] | 0xd080;
                                                c.g[A1][0] = c.g[A1][0] | 0xd010;
                                                c.sw(c.g[V1][0] as u32, 0u64);
                                                c.g[A0][0] = 0x10000000;
                                                c.sw(c.g[A1][0] as u32, c.g[S1][0]);
                                                c.g[A0][0] = c.g[A0][0] | 0xd020;
                                                c.g[A1][0] = 0x10000000;
                                                c.sw(c.g[A0][0] as u32, c.g[S4][0]);
                                                c.g[A1][0] = c.g[A1][0] | 0xd000;
                                                c.g[V1][0] = 0x100;
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] = 0x10000000;
                                                if t {
                                                    break 'b_40843c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x00408438);
                                                    Callee {
                                                        addr: 0x0042c0c0,
                                                        f: ee::s15::EIntr,
                                                    }
                                                    .call(c, 0x00408438)?;
                                                    c.g[V1][0] = 0x10000000;
                                                    break 'b_40843c;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[A2][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                            c.g[V1][0] = c.g[V1][0] | 0xd000;
                                            'l_408448: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                c.g[V0][0] = c.g[V0][0] & 0x100;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00408448);
                                                    }
                                                    continue 'l_408448;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] = 0x10000000;
                                                    c.g[V1][0] = c.g[V1][0] | 0xd020;
                                                    'l_408470: loop {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                        let t = c.g[V0][0] != 0u64;

                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00408470);
                                                            }
                                                            continue 'l_408470;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[S1][0] = c.g[S2][0];
                                                            c.g[V0][0] = ((c.g[S0][0] as i64)
                                                                < (c.g[A2][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[S2][0] = c.g[S3][0];
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00408370,
                                                                    );
                                                                }
                                                                continue 'l_408370;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A3][0] =
                                                                    c.lw(c.g[SP][0] as u32);
                                                                if t {
                                                                    break 'b_4084ac;
                                                                } else {
                                                                    break 'b_4084a4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[A3][0] = c.lw(c.g[SP][0] as u32);
                        break 'b_4084ac;
                    }
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S7][0];
                    c.g[V1][0] = 0xc0;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe4));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.lo[0] = c.g[A3][0];
                    c.madd(0, 0, 2, 3);
                    c.g[A3][0] = c.lo[0];
                    let t = c.g[A0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00408360);
                        }
                        continue 'l_408360;
                    } else {
                        break 'b_4084d8;
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void libmpeg_ConvertPictureCsc(DecState* dec, Picture* pic)` (0x00409088):
/// converts a picture to RGB through the IPU colour-space converter. Tells
/// the callback chain (`DispatchCallback` command 2), clears an IPU error,
/// waits for the IPU, issues the `IpuCommand`, then feeds the source
/// `pic[0]` (`pic[3] * pic[4]` macroblocks, 0x18 quadwords each) through the
/// to-IPU DMA: in one go when it fits 0xffff quadwords, else in 0xffff-qwc
/// pieces with a DMAC handler (`AddDmacHandler`/`EnableDmac`, `DisableDmac`,
/// `RemoveDmacHandler`). Fewer than 1024 macroblocks use `IpuCsc`, more
/// `IpuCscDma`; finally the callback chain gets command 3. Converted from the
/// generated labelled-block body.
pub fn libmpeg_convert_picture_csc(c: &mut Cpu) {
    guest(c, |c| {
        'b_4090f0: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[A2][0] = 0x2;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S0][0]);
            c.g[S2][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S5][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S1][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
            c.mult(0, 19, 3, 2);
            c.g[RA][0] = sx(0x004090d0);
            c.sw(c.g[SP][0] as u32, c.g[A2][0]);
            Callee {
                addr: 0x00402c30,
                f: ee::s15::libmpeg_DispatchCallback,
            }
            .call(c, 0x004090d0)?;
            c.g[V1][0] = 0x10000000;
            c.g[V1][0] = c.g[V1][0] | 0x2010;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] & 0x4000;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x40000000;
            if t {
                break 'b_4090f0;
            } else {
                c.cycles += 2;
                c.g[AT][0] = 0x10000000;
                c.sw((c.g[AT][0] as u32).wrapping_add(0x2010), c.g[V0][0]);
                break 'b_4090f0;
            }
        }
        c.cycles += 4;
        c.g[V1][0] = 0x10000000;
        c.g[S5][0] = ((c.g[S3][0] as i64) < 1024i64) as u64;
        c.g[V1][0] = c.g[V1][0] | 0x2010;
        'l_409100: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let t = (c.g[V0][0] as i64) < 0;

            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00409100);
                }
                continue 'l_409100;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00409128);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x00406df0,
                    f: ee::s15::libmpeg_IpuCommand,
                }
                .call(c, 0x00409128)?;
                c.g[A0][0] = 0x10000000;
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.g[A0][0] = c.g[A0][0] | 0x2010;
                'l_409138: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    let t = (c.g[V0][0] as i64) < 0;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00409138);
                        }
                        continue 'l_409138;
                    } else {
                        'b_4092dc: {
                            'b_409260: {
                                c.cycles += 10;
                                c.g[V0][0] = 0x18;
                                c.g[S1][0] = 0xfff0000;
                                c.mult(0, 2, 19, 2);
                                c.g[S1][0] = c.g[S1][0] | 0xffff;
                                c.g[V1][0] = c.g[V1][0] & c.g[S1][0];
                                c.g[S4][0] = 0xffff;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                                c.g[A0][0] = (c.g[S4][0] < c.g[V0][0]) as u64;
                                let t = c.g[A0][0] == 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                if t {
                                    break 'b_409260;
                                } else {
                                    'b_4091f8: {
                                        c.cycles += 28;
                                        c.g[A1][0] = 0x410000;
                                        c.g[A0][0] = 0x4;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7050));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x00409194);
                                        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                        Callee {
                                            addr: 0x00423ac0,
                                            f: ee::s15::f_00423ac0,
                                        }
                                        .call(c, 0x00409194)?;
                                        c.g[S2][0] = c.g[V0][0];
                                        c.g[V1][0] = 0x10;
                                        c.g[V0][0] = 0x10000000;
                                        c.g[A0][0] = 0x4;
                                        c.g[V0][0] = c.g[V0][0] | 0xe010;
                                        c.g[RA][0] = sx(0x004091b0);
                                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                                        Callee {
                                            addr: 0x004249b8,
                                            f: ee::s15::EnableDmac,
                                        }
                                        .call(c, 0x004091b0)?;
                                        c.g[RA][0] = sx(0x004091b8);
                                        Callee {
                                            addr: 0x0042c078,
                                            f: ee::s15::DIntr,
                                        }
                                        .call(c, 0x004091b8)?;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = 0x10000000;
                                        c.g[V1][0] = c.g[V1][0] | 0xb410;
                                        c.g[A0][0] = 0x10000000;
                                        c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                                        c.g[A0][0] = c.g[A0][0] | 0xb420;
                                        c.sw(c.g[A0][0] as u32, c.g[S4][0]);
                                        c.g[V1][0] = 0x10000000;
                                        c.g[V1][0] = c.g[V1][0] | 0xb400;
                                        c.g[A0][0] = 0x101;
                                        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_4091f8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[RA][0] = sx(0x004091f4);
                                            Callee {
                                                addr: 0x0042c0c0,
                                                f: ee::s15::EIntr,
                                            }
                                            .call(c, 0x004091f4)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                            break 'b_4091f8;
                                        }
                                    }
                                    'b_409244: {
                                        'b_409234: {
                                            c.cycles += 9;
                                            c.g[V1][0] = 0xf0000;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.g[V1][0] = c.g[V1][0] | 0xfff0;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_sub(c.g[S4][0] as u32));
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x24),
                                                c.g[V0][0],
                                            );
                                            let t = c.g[S5][0] == 0u64;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x20),
                                                c.g[A0][0],
                                            );
                                            if t {
                                                break 'b_409234;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                                                c.g[A2][0] = c.g[S3][0];
                                                c.g[RA][0] = sx(0x0040922c);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x00408b80,
                                                    f: ee::s15::libmpeg_IpuCsc,
                                                }
                                                .call(c, 0x0040922c)?;
                                                let t = true;

                                                if t {
                                                    break 'b_409244;
                                                } else {
                                                    break 'b_409234;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                                        c.g[A2][0] = c.g[S3][0];
                                        c.g[RA][0] = sx(0x00409244);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00408df0,
                                            f: ee::s15::libmpeg_IpuCscDma,
                                        }
                                        .call(c, 0x00409244)?;
                                        break 'b_409244;
                                    }
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x0040924c);
                                    c.g[A0][0] = 0x4;
                                    Callee {
                                        addr: 0x00424950,
                                        f: ee::s15::DisableDmac,
                                    }
                                    .call(c, 0x0040924c)?;
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x00409258);
                                    c.g[A0][0] = 0x4;
                                    Callee {
                                        addr: 0x00423ad0,
                                        f: ee::s15::RemoveDmacHandler,
                                    }
                                    .call(c, 0x00409258)?;
                                    let t = true;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
                                    if t {
                                        break 'b_4092dc;
                                    } else {
                                        break 'b_409260;
                                    }
                                }
                            }
                            'b_4092a8: {
                                c.cycles += 16;
                                c.g[RA][0] = sx(0x00409268);
                                Callee {
                                    addr: 0x0042c078,
                                    f: ee::s15::DIntr,
                                }
                                .call(c, 0x00409268)?;
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                c.g[V1][0] = 0x10000000;
                                c.g[V1][0] = c.g[V1][0] | 0xb410;
                                c.g[A2][0] = 0x10000000;
                                c.g[A1][0] = c.g[A1][0] & c.g[S1][0];
                                c.g[A2][0] = c.g[A2][0] | 0xb420;
                                c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                                c.g[A0][0] = 0x10000000;
                                c.g[A0][0] = c.g[A0][0] | 0xb400;
                                c.g[A1][0] = 0x101;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                                let t = c.g[V0][0] == 0u64;
                                c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                                if t {
                                    break 'b_4092a8;
                                } else {
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x004092a8);
                                    Callee {
                                        addr: 0x0042c0c0,
                                        f: ee::s15::EIntr,
                                    }
                                    .call(c, 0x004092a8)?;
                                    break 'b_4092a8;
                                }
                            }
                            'b_4092c8: {
                                c.cycles += 2;
                                let t = c.g[S5][0] == 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                                if t {
                                    break 'b_4092c8;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                                    c.g[A2][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x004092c0);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00408b80,
                                        f: ee::s15::libmpeg_IpuCsc,
                                    }
                                    .call(c, 0x004092c0)?;
                                    let t = true;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
                                    if t {
                                        break 'b_4092dc;
                                    } else {
                                        break 'b_4092c8;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                            c.g[A2][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x004092d8);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x00408df0,
                                f: ee::s15::libmpeg_IpuCscDma,
                            }
                            .call(c, 0x004092d8)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
                            break 'b_4092dc;
                        }
                        c.cycles += 13;
                        c.g[V0][0] = 0x3;
                        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                        c.g[RA][0] = sx(0x004092ec);
                        c.g[A1][0] = c.g[SP][0];
                        Callee {
                            addr: 0x00402c30,
                            f: ee::s15::libmpeg_DispatchCallback,
                        }
                        .call(c, 0x004092ec)?;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `sd reg, off(sp)` into the frame (the value is read before the store).
macro_rules! oc1121_sd {
    ($c:ident, $frame:expr, $off:expr, $reg:expr) => {{
        let v = $c.g[$reg][0];
        let a = $frame.at($c, $off);
        Addr(a).sd($c, v);
    }};
}

/// `ld reg, off(sp)` from the frame.
macro_rules! oc1121_ld {
    ($c:ident, $frame:expr, $off:expr, $reg:expr) => {{
        let a = $frame.at($c, $off);
        $c.g[$reg][0] = Addr(a).ld($c);
    }};
}

/// `libmpeg_DecodeSlice(d, n)` (0x406318): one slice; 1 and 3 ask for a re-run.
pub(crate) const OC1121_DECODE_SLICE: Callee = Callee {
    addr: 0x0040_6318,
    f: ee::s15::f_00406318,
};

/// `libmpeg_IpuWaitBusy(d)` (0x406e20).
pub(crate) const OC1121_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::f_00406e20,
};

/// `libmpeg_WaitBdec(d)` (0x405b70): nonzero when the bitstream decoder failed.
pub(crate) const OC1121_WAIT_BDEC: Callee = Callee {
    addr: 0x0040_5b70,
    f: ee::s15::f_00405b70,
};

/// `libmpeg_AddPredAndStore(d, skipped)` (0x404b78).
pub(crate) const OC1121_ADD_PRED_AND_STORE: Callee = Callee {
    addr: 0x0040_4b78,
    f: ee::s15::f_00404b78,
};

/// `libmpeg_Error(d, msg)` (0x4034a8).
pub(crate) const OC1121_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::f_004034a8,
};

/// `libmpeg_GetBits(d, n)` (0x4072e8): reads `n` bits from the stream.
pub(crate) const OC1121_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::f_004072e8,
};

/// `libmpeg_FlushBits(d, n)` (0x4071f8).
pub(crate) const OC1121_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::f_004071f8,
};

/// `libmpeg_IpuVdec(d, table)` (0x406f78): one variable-length code.
pub(crate) const OC1121_IPU_VDEC: Callee = Callee {
    addr: 0x0040_6f78,
    f: ee::s15::f_00406f78,
};

/// `libmpeg_DecodeDmVector(d)` (0x405e10).
pub(crate) const OC1121_DECODE_DM_VECTOR: Callee = Callee {
    addr: 0x0040_5e10,
    f: ee::s15::f_00405e10,
};

/// `libmpeg_MotionVector(...)` (0x406ca0): both vectors of one block.
pub(crate) const OC1121_MOTION_VECTOR: Callee = Callee {
    addr: 0x0040_6ca0,
    f: ee::s15::f_00406ca0,
};

/// `libmpeg_DecodeMotionVector(...)` (0x406a78): one vector component.
pub(crate) const OC1121_DECODE_MOTION_VECTOR: Callee = Callee {
    addr: 0x0040_6a78,
    f: ee::s15::f_00406a78,
};

/// `CallbackNodata(d)` (0x402c80): the host callback when the IPU stalls.
pub(crate) const OC1121_CALLBACK_NODATA: Callee = Callee {
    addr: 0x0040_2c80,
    f: ee::s15::f_00402c80,
};

/// `libmpeg_ExtraBitInformation(d)` (0x407a10).
pub(crate) const OC1121_EXTRA_BIT_INFO: Callee = Callee {
    addr: 0x0040_7a10,
    f: ee::s15::f_00407a10,
};

/// `int libmpeg_DecodeSlices(MpegDec* d)` (0x4060c8): decodes the slices of a
/// picture. The slice count `n` (`d->w * d->h`, halved unless the picture is
/// a frame) drives `libmpeg_DecodeSlice` until it stops asking for a re-run
/// (result 1 or 3). Then it waits for the IPU and the bitstream decoder,
/// retries a failed slice once, reports errors through `libmpeg_Error`, and
/// returns whether the slice result was 0.
pub fn libmpeg_decode_slices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        oc1121_sd!(c, frame, 0x40, S4);
        oc1121_sd!(c, frame, 0x30, S3);
        c.g[S4][0] = 0x1;
        oc1121_sd!(c, frame, 0x20, S2);
        c.g[S3][0] = 0x3;
        oc1121_sd!(c, frame, 0x10, S1);
        c.g[S2][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0x50, RA);
        oc1121_sd!(c, frame, 0x0, S0);

        let st = c.g[S2][0] as u32;
        Addr(st.wrapping_add(0x810)).sw(c, 0);
        c.g[V0][0] = Addr(st.wrapping_add(0x130)).lw(c);
        c.g[A0][0] = Addr(st.wrapping_add(0x12c)).lw(c);
        c.g[V1][0] = Addr(st.wrapping_add(0x174)).lw(c);
        c.mult(0, S1, A0, V0);
        Addr(st.wrapping_add(0x814)).sw(c, 0);
        c.g[V1][0] ^= 0x3;
        c.g[V0][0] = sx(((c.g[S1][0] as u32) as i32 >> 1) as u32);
        if c.g[V1][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }

        // `'l_406118`: the outer loop (result 3 re-runs it).
        loop {
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            // `'l_40611c`: the inner loop (result 1 re-runs it).
            let rerun_outer = loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                OC1121_DECODE_SLICE.call(c, 0x0040_6124)?;
                c.g[S0][0] = c.g[V0][0];
                let again = c.g[S0][0] == c.g[S4][0];
                c.g[A0][0] = c.g[S2][0];
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_611c);
                    }
                    continue;
                }
                c.cycles += 2;
                if c.g[S0][0] == c.g[S3][0] {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_6118);
                    }
                    break true;
                }
                break false;
            };
            if !rerun_outer {
                break;
            }
        }

        // `'b_40616c`: wait for the IPU, then for the bitstream decoder.
        c.cycles += 6;
        c.g[A0][0] = c.g[S2][0];
        OC1121_IPU_WAIT_BUSY.call(c, 0x0040_6140)?;
        c.g[A0][0] = c.g[S2][0];
        OC1121_WAIT_BDEC.call(c, 0x0040_6148)?;
        let bdec_failed = c.g[V0][0] != 0;
        c.g[V1][0] = 0x1000_0000;
        if !bdec_failed {
            // `'b_406164`: a slice that failed earlier is retried once.
            c.cycles += 3;
            c.g[V0][0] = Addr(st.wrapping_add(0x10a0)).lw(c);
            let retry_clear = c.g[V0][0] == 0;
            c.g[S0][0] = 0x4;
            if retry_clear {
                c.cycles += 2;
                c.g[S0][0] = 0x2;
                c.g[V1][0] = 0x1000_0000;
            } else {
                c.cycles += 2;
                Addr(st.wrapping_add(0x10a0)).sw(c, 0);
            }
        }

        // Slice result flags and the status-register wait.
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[S3][0] = (c.g[S0][0] < 0x1) as u64;
        c.g[V1][0] |= 0xd400;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
            c.g[V0][0] &= 0x1;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0040_6178);
            }
        }

        c.cycles += 2;
        let s0_nonzero = c.g[S0][0] != 0;
        c.g[V0][0] = (c.g[S1][0] < 0x2) as u64;
        if !s0_nonzero {
            c.cycles += 5;
            c.g[A1][0] = Addr(st.wrapping_add(0x810)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = (c.g[A1][0] < 0x1) as u64;
            OC1121_ADD_PRED_AND_STORE.call(c, 0x0040_61ac)?;
            c.g[V0][0] = (c.g[S1][0] < 0x2) as u64;
        }

        // `'b_4061c4`: report a slice that stayed broken.
        c.cycles += 2;
        let broken = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if broken {
            c.cycles += 3;
            c.g[A1][0] = 0x0049_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f08));
            OC1121_ERROR.call(c, 0x0040_61c4)?;
        }

        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        oc1121_ld!(c, frame, 0x50, RA);
        oc1121_ld!(c, frame, 0x40, S4);
        oc1121_ld!(c, frame, 0x30, S3);
        oc1121_ld!(c, frame, 0x20, S2);
        oc1121_ld!(c, frame, 0x10, S1);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_SkippedMacroblock(MpegDec* d, int* pmv, int* motionType,
/// int* fieldSelect, int* mbType)` (0x406510): sets up a skipped macroblock:
/// marks it, zeroes the predictors when the picture is a P field, picks the
/// motion type and field select from the picture structure, and clears the
/// macroblock's intra bit. Returns `ok` (0 after a field error, which is
/// reported through `libmpeg_Error`).
pub fn libmpeg_skipped_macroblock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x140;
        oc1121_sd!(c, frame, 0x0, S0);
        c.g[T1][0] = 0x1;
        oc1121_sd!(c, frame, 0x10, RA);
        c.g[S0][0] = c.g[T0][0];
        c.g[T0][0] = 0x2;
        let st = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(st.wrapping_add(0x810)).lw(c);
        c.mult(0, T2, V0, V1);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(st));
        let one = c.g[T1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x6cc)).sw(c, one);
        let one = c.g[T1][0];
        Addr(st.wrapping_add(0x1b0)).sw(c, one);
        c.g[V0][0] = Addr(st.wrapping_add(0x150)).lw(c);
        if c.g[V0][0] != c.g[T0][0] {
            c.g[V1][0] = Addr(st.wrapping_add(0x174)).lw(c);
        } else {
            c.cycles += 5;
            let pmv = c.g[A1][0] as u32;
            Addr(pmv).sw(c, 0);
            Addr(pmv.wrapping_add(0x14)).sw(c, 0);
            Addr(pmv.wrapping_add(0x10)).sw(c, 0);
            Addr(pmv.wrapping_add(0x4)).sw(c, 0);
            c.g[V1][0] = Addr(st.wrapping_add(0x174)).lw(c);
        }

        // `'b_406588`: motion type, then the field select.
        let field_select_done = 'motion: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if c.g[V1][0] != c.g[V0][0] {
                let two = c.g[T1][0];
                Addr(c.g[A2][0] as u32).sw(c, two);
                break 'motion false;
            }
            c.cycles += 2;
            let two = c.g[T0][0];
            Addr(c.g[A2][0] as u32).sw(c, two);
            true
        };
        if !field_select_done {
            c.cycles += 5;
            c.g[V0][0] = Addr(st.wrapping_add(0x174)).lw(c);
            c.g[V0][0] ^= 0x2;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
            let bottom = c.g[V0][0];
            Addr(c.g[A3][0] as u32).sw(c, bottom);
            Addr((c.g[A3][0] as u32).wrapping_add(0x4)).sw(c, bottom);
        }

        // `'b_4065ac`: a field error is reported once.
        c.cycles += 4;
        c.g[V1][0] = Addr(st.wrapping_add(0x150)).lw(c);
        c.g[V0][0] = 0x1;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x0049_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f90));
            OC1121_ERROR.call(c, 0x0040_65a4)?;
            c.g[T1][0] = 0;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        }

        c.cycles += 8;
        c.g[A0][0] = 0xffff_ffff_ffff_fffe;
        oc1121_ld!(c, frame, 0x10, RA);
        c.g[V0][0] = c.g[T1][0];
        let mask = c.g[A0][0];
        c.g[V1][0] &= mask;
        let flags = c.g[V1][0];
        Addr(c.g[S0][0] as u32).sw(c, flags);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_DecodeMotionVector(int* pred, int r_size, int motion_code,
/// int motion_residual, int full_pel)` (0x406a78): one motion vector
/// component. The predictor is halved for full-pel vectors, moved by the
/// coded difference (wrapped into the range `[-lim, lim)` of the r_size),
/// and stored back doubled for full-pel vectors.
pub fn libmpeg_decode_motion_vector(c: &mut Cpu) {
    c.cycles += 7;
    let pred_ptr = c.g[A0][0];
    c.g[T2][0] = pred_ptr;
    c.g[V0][0] = 0x10;
    c.g[A0][0] = Addr(pred_ptr as u32).lw(c);
    let r = (c.g[A1][0] as u32) & 31;
    c.g[T1][0] = sx(0x10u32 << r);
    let lim = (c.g[T1][0] as u32) as i32;
    let half = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
    c.g[V1][0] = half;
    if c.g[T0][0] != 0 {
        c.g[A0][0] = half;
    }
    let vec = (c.g[A0][0] as u32) as i32;
    let mc = c.g[A2][0] as u32 as i32;
    let res = c.g[A3][0] as u32;
    let mut v = vec;
    // The wrap-around paths share one tail charge (`'b_406af0`).
    let mut wrapped = false;
    if (c.g[A2][0] as i64) > 0 {
        // motion_code > 0: add, wrap down from `lim`.
        c.cycles += 8;
        let pre = (vec as u32)
            .wrapping_add(1)
            .wrapping_add((mc as u32).wrapping_sub(1) << r)
            .wrapping_add(res) as i32;
        c.g[V1][0] = (pre < lim) as u64;
        v = pre;
        if pre >= lim {
            c.cycles += 3;
            v = pre.wrapping_sub(lim.wrapping_mul(2));
            wrapped = true;
        }
    } else {
        c.cycles += 2;
        if (c.g[A2][0] as i64) < 0 {
            // motion_code < 0: subtract, wrap up from `-lim`.
            c.cycles += 9;
            let shifted = (!(mc as u32)) << r;
            let pre = (vec as u32)
                .wrapping_sub(1)
                .wrapping_sub(shifted.wrapping_add(res)) as i32;
            let neg_lim = (lim as u32).wrapping_neg() as i32;
            c.g[V1][0] = (pre < neg_lim) as u64;
            v = pre;
            if pre < neg_lim {
                c.cycles += 1;
                v = pre.wrapping_add(lim.wrapping_mul(2));
            }
            wrapped = true;
        }
    }
    if wrapped {
        c.cycles += 1;
    }
    c.cycles += 3;
    c.g[A0][0] = sx(v as u32);
    c.g[V0][0] = sx(v as u32);
    if c.g[T0][0] != 0 {
        c.g[V0][0] = sx((v as u32) << 1);
    }
    let out = c.g[V0][0];
    Addr(pred_ptr as u32).sw(c, out);
    abi::ret(c);
}

/// `void libmpeg_MotionVectors(MpegDec* d, int* x, int* out, int* fieldSelect,
/// int idx, int mvCount, int k, rSizeH, rSizeV, dualPrime, mvScale)`
/// (0x406b00): the motion vectors of one partition. With one vector it reads
/// the field select (unless `k` or `dualPrime` decide it) and decodes one
/// vector; with two it reads both field selects and decodes both.
pub fn libmpeg_motion_vectors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x1;
        oc1121_sd!(c, frame, 0x90, FP);
        oc1121_sd!(c, frame, 0x80, S7);
        oc1121_sd!(c, frame, 0x70, S6);
        c.g[S7][0] = c.g[A2][0];
        oc1121_sd!(c, frame, 0x60, S5);
        oc1121_sd!(c, frame, 0x50, S4);
        c.g[S5][0] = c.g[T3][0];
        oc1121_sd!(c, frame, 0x40, S3);
        c.g[S4][0] = c.g[A1][0];
        oc1121_sd!(c, frame, 0x30, S2);
        oc1121_sd!(c, frame, 0x10, S0);
        c.g[S2][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0xa0, RA);
        c.g[S0][0] = c.g[T0][0];
        oc1121_sd!(c, frame, 0x20, S1);
        let field = c.g[A3][0];
        Addr(frame.at(c, 0)).sw(c, field);
        c.g[S6][0] = Addr(frame.at(c, 0xb0)).lw(c);
        c.g[S3][0] = Addr(frame.at(c, 0xb8)).lw(c);
        let two_vectors = c.g[T1][0] != c.g[V0][0];
        c.g[FP][0] = Addr(frame.at(c, 0xc0)).lw(c);

        if two_vectors {
            // Two vectors: both field selects, then both vectors (tail call).
            c.cycles += 42;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            OC1121_GET_BITS.call(c, 0x0040_6b94)?;
            c.g[S1][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = c.g[S7][0];
            let bit = c.g[V0][0];
            Addr(c.g[S0][0] as u32).sw(c, bit);
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S6][0];
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = c.g[FP][0];
            c.g[T3][0] = 0;
            OC1121_MOTION_VECTOR.call(c, 0x0040_6bcc)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            OC1121_GET_BITS.call(c, 0x0040_6bd8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
            let bit = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, bit);
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S6][0];
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = c.g[FP][0];
            oc1121_ld!(c, frame, 0xa0, RA);
            oc1121_ld!(c, frame, 0x90, FP);
            c.g[T3][0] = 0;
            oc1121_ld!(c, frame, 0x80, S7);
            oc1121_ld!(c, frame, 0x70, S6);
            oc1121_ld!(c, frame, 0x60, S5);
            oc1121_ld!(c, frame, 0x50, S4);
            oc1121_ld!(c, frame, 0x40, S3);
            oc1121_ld!(c, frame, 0x30, S2);
            oc1121_ld!(c, frame, 0x20, S1);
            oc1121_ld!(c, frame, 0x10, S0);
            c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
            c.pc = 0x0040_6ca0;
            (OC1121_MOTION_VECTOR.f)(c);
            return Ok(());
        }

        // One vector: the field select, unless `k` or `dualPrime` decide it.
        c.cycles += 2;
        if c.g[T2][0] != 0 {
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
        } else {
            c.cycles += 2;
            if c.g[S3][0] != 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
            } else {
                c.cycles += 8;
                c.g[A1][0] = 0x1;
                OC1121_GET_BITS.call(c, 0x0040_6b70)?;
                c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let bit = c.g[V0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, bit);
                let bit = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, bit);
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
            }
        }

        // The vector itself, then its copy into the predictor slot (tail).
        c.cycles += 26;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = c.g[S5][0];
        c.g[T0][0] = c.g[S6][0];
        c.g[T1][0] = c.g[S3][0];
        c.g[T2][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[T3][0] = 0;
        OC1121_MOTION_VECTOR.call(c, 0x0040_6c5c)?;
        let vector = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(vector).lw(c);
        c.g[V1][0] = Addr(vector.wrapping_add(0x4)).lw(c);
        let word = c.g[V0][0];
        Addr(vector.wrapping_add(0x10)).sw(c, word);
        let word = c.g[V1][0];
        Addr(vector.wrapping_add(0x14)).sw(c, word);
        oc1121_ld!(c, frame, 0xa0, RA);
        oc1121_ld!(c, frame, 0x90, FP);
        oc1121_ld!(c, frame, 0x80, S7);
        oc1121_ld!(c, frame, 0x70, S6);
        oc1121_ld!(c, frame, 0x60, S5);
        oc1121_ld!(c, frame, 0x50, S4);
        oc1121_ld!(c, frame, 0x40, S3);
        oc1121_ld!(c, frame, 0x30, S2);
        oc1121_ld!(c, frame, 0x20, S1);
        oc1121_ld!(c, frame, 0x10, S0);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_MotionVector(MpegDec* d, int* pred, int* dmv, int r_size1,
/// int r_size2, int has_dmv, int full_pel_flag, int full_pel)` (0x406ca0):
/// decodes both motion vector components of one vector (the second one with
/// its own motion code and residual), halves or doubles the predictor for
/// field pictures, and reads the dual-prime vector when present.
pub fn libmpeg_motion_vector(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        oc1121_sd!(c, frame, 0x30, S3);
        oc1121_sd!(c, frame, 0x20, S2);
        c.g[S3][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0x80, FP);
        c.g[S2][0] = c.g[A1][0];
        oc1121_sd!(c, frame, 0x70, S7);
        c.g[FP][0] = c.g[A2][0];
        oc1121_sd!(c, frame, 0x60, S6);
        c.g[S7][0] = c.g[T1][0];
        oc1121_sd!(c, frame, 0x50, S5);
        c.g[S6][0] = c.g[T2][0];
        oc1121_sd!(c, frame, 0x40, S4);
        c.g[S5][0] = c.g[T3][0];
        oc1121_sd!(c, frame, 0x10, S1);
        c.g[S4][0] = c.g[T0][0];
        oc1121_sd!(c, frame, 0x0, S0);
        c.g[S1][0] = c.g[A3][0];
        oc1121_sd!(c, frame, 0x90, RA);

        // The motion code of the first component.
        c.g[RA][0] = sx(0x0040_6cf4);
        c.g[A1][0] = 0x2;
        OC1121_IPU_VDEC.call(c, 0x0040_6cf4)?;
        c.g[S0][0] = c.g[V0][0];

        // The residual of the first component (`r_size` bits).
        let mut have_residual = false;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[S0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                OC1121_GET_BITS.call(c, 0x0040_6d0c)?;
                c.g[A3][0] = c.g[V0][0];
                have_residual = true;
            }
        }
        if !have_residual {
            c.cycles += 1;
            c.g[A3][0] = 0;
        }

        // `'b_406d44`: the first component, then the dual-prime vector.
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S5][0];
        OC1121_DECODE_MOTION_VECTOR.call(c, 0x0040_6d2c)?;
        c.g[A0][0] = c.g[S3][0];
        if c.g[S7][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            OC1121_DECODE_DM_VECTOR.call(c, 0x0040_6d3c)?;
            let dmv = c.g[V0][0];
            Addr(c.g[FP][0] as u32).sw(c, dmv);
            c.g[A0][0] = c.g[S3][0];
        }

        // `'b_406d70`: the second component's motion code and residual.
        c.cycles += 4;
        c.g[RA][0] = sx(0x0040_6d4c);
        c.g[A1][0] = 0x2;
        OC1121_IPU_VDEC.call(c, 0x0040_6d4c)?;
        c.g[S0][0] = c.g[V0][0];
        let mut have_second_residual = false;
        if c.g[S4][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[S0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S4][0];
                OC1121_GET_BITS.call(c, 0x0040_6d64)?;
                c.g[A3][0] = c.g[V0][0];
                have_second_residual = true;
            }
        }
        if !have_second_residual {
            c.cycles += 1;
            c.g[A3][0] = 0;
        }

        // `'b_406d84`: the predictor of the field picture is halved.
        c.cycles += 2;
        let half_pel = c.g[S6][0] == 0;
        c.g[A1][0] = c.g[S4][0];
        if !half_pel {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            let word = c.g[V0][0];
            Addr((c.g[S2][0] as u32).wrapping_add(0x4)).sw(c, word);
        }

        // `'b_406da8`: the second component, then the field doubling.
        c.cycles += 6;
        c.g[A2][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S5][0];
        c.g[RA][0] = sx(0x0040_6d94);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
        OC1121_DECODE_MOTION_VECTOR.call(c, 0x0040_6d94)?;
        if c.g[S6][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            let word = c.g[V0][0];
            Addr((c.g[S2][0] as u32).wrapping_add(0x4)).sw(c, word);
        }

        // `'b_406dc0`: the dual-prime vector of the second component.
        c.cycles += 2;
        let no_dmv = c.g[S7][0] == 0;
        oc1121_ld!(c, frame, 0x90, RA);
        if !no_dmv {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            OC1121_DECODE_DM_VECTOR.call(c, 0x0040_6db8)?;
            let dmv = c.g[V0][0];
            Addr((c.g[FP][0] as u32).wrapping_add(0x4)).sw(c, dmv);
            oc1121_ld!(c, frame, 0x90, RA);
        }

        c.cycles += 11;
        oc1121_ld!(c, frame, 0x80, FP);
        oc1121_ld!(c, frame, 0x70, S7);
        oc1121_ld!(c, frame, 0x60, S6);
        oc1121_ld!(c, frame, 0x50, S5);
        oc1121_ld!(c, frame, 0x40, S4);
        oc1121_ld!(c, frame, 0x30, S3);
        oc1121_ld!(c, frame, 0x20, S2);
        oc1121_ld!(c, frame, 0x10, S1);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_IpuWaitCmd(MpegDec* d)` (0x406ec8): waits until the IPU
/// command register has a command (its top bit clear) or reports an error
/// (bit 14 of the control register). Stalled waits call the host callback
/// every 5001 polls. Returns the command word.
pub fn libmpeg_ipu_wait_cmd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x1000_0000;
        oc1121_sd!(c, frame, 0x20, S2);
        c.g[V0][0] |= 0x2000;
        oc1121_sd!(c, frame, 0x30, RA);
        c.g[S2][0] = c.g[A0][0];
        oc1121_sd!(c, frame, 0x10, S1);
        c.g[A1][0] = 0;
        oc1121_sd!(c, frame, 0x0, S0);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).ld(c);
        let ready = (c.g[A0][0] as i64) >= 0;
        oc1121_ld!(c, frame, 0x30, RA);
        // Whether the tail (`V0 = cmd`, one cycle) runs: not on the error exit.
        let mut tail = true;
        if !ready {
            c.cycles += 6;
            c.g[V0][0] = 0x1000_2010;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] &= 0x4000;
            let error_flag = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[A0][0];
            if error_flag {
                tail = false;
            } else {
                c.cycles += 6;
                c.g[S1][0] = 0x1000_2000;
                c.g[S0][0] = 0x1000_2010;
                c.g[V0][0] = c.g[A1][0];
                loop {
                    // `'b_406f40`: count the poll, call back every 5001 polls.
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 5001) as u64;
                    let below = c.g[V0][0] != 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    if !below {
                        c.cycles += 3;
                        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x858)).lw(c);
                        OC1121_CALLBACK_NODATA.call(c, 0x0040_6f3c)?;
                        c.g[A1][0] = 0;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).ld(c);
                    let cmd_ready = (c.g[A0][0] as i64) >= 0;
                    oc1121_ld!(c, frame, 0x30, RA);
                    if cmd_ready {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] &= 0x4000;
                    let idle = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[A1][0];
                    if idle {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_6f28);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    oc1121_ld!(c, frame, 0x30, RA);
                    break;
                }
            }
        }
        if tail {
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0];
        }
        c.cycles += 5;
        oc1121_ld!(c, frame, 0x20, S2);
        oc1121_ld!(c, frame, 0x10, S1);
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// `int libmpeg_SliceHeaderTail(MpegDec* d)` (0x4074b8): reads the last
/// fields of a slice header (5 bits into `d+0x1b4`, then one flag bit); when
/// the flag is set it skips 1 bit, flushes 7 bits and reads the extra
/// information. Returns 0.
pub fn libmpeg_slice_header_tail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x5;
        oc1121_sd!(c, frame, 0x0, S0);
        oc1121_sd!(c, frame, 0x10, RA);
        c.g[S0][0] = c.g[A0][0];
        OC1121_GET_BITS.call(c, 0x0040_74d0)?;
        let bits = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x1b4)).sw(c, bits);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        OC1121_GET_BITS.call(c, 0x0040_74e0)?;
        let flag_clear = c.g[V0][0] == 0;
        oc1121_ld!(c, frame, 0x10, RA);
        if !flag_clear {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1;
            OC1121_GET_BITS.call(c, 0x0040_74f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x7;
            OC1121_FLUSH_BITS.call(c, 0x0040_7500)?;
            c.g[A0][0] = c.g[S0][0];
            OC1121_EXTRA_BIT_INFO.call(c, 0x0040_7508)?;
            oc1121_ld!(c, frame, 0x10, RA);
        }
        c.cycles += 4;
        c.g[V0][0] = 0;
        oc1121_ld!(c, frame, 0x0, S0);
        frame.close(c);
        Ok(())
    })
}

/// The MPEG decoder's direct callees (their generated aliases dispatch
/// through the override table, as the generated call sites do).
pub(crate) const OC1122_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::libmpeg_GetBits,
};

pub(crate) const OC1122_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::libmpeg_FlushBits,
};

pub(crate) const OC1122_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};

pub(crate) const OC1122_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::libmpeg_IpuCommand,
};

pub(crate) const OC1122_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};

pub(crate) const OC1122_EXTRA_BIT_INFO: Callee = Callee {
    addr: 0x0040_7a10,
    f: ee::s15::libmpeg_ExtraBitInformation,
};

pub(crate) const OC1122_EXT_USER_DATA: Callee = Callee {
    addr: 0x0040_76f8,
    f: ee::s15::libmpeg_ExtensionAndUserData,
};

/// `GetBits(sys, n)` returning to `ret_to`: the call sites set `a0 = sys`
/// and `a1 = n`.
pub(crate) fn oc1122_get_bits(c: &mut Cpu, sys: u64, n: u64, ret_to: u32) -> Flow {
    c.g[A0][0] = sys;
    c.g[A1][0] = n;
    OC1122_GET_BITS.call(c, ret_to)
}

/// `IpuWaitBusy(sys)` returning to `ret_to`.
pub(crate) fn oc1122_ipu_wait_busy(c: &mut Cpu, sys: u64, ret_to: u32) -> Flow {
    c.g[A0][0] = sys;
    OC1122_IPU_WAIT_BUSY.call(c, ret_to)
}

/// `IpuCommand(sys, cmd)` returning to `ret_to`.
pub(crate) fn oc1122_ipu_command(c: &mut Cpu, sys: u64, cmd: u64, ret_to: u32) -> Flow {
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

/// `sprintf` (0x417828).
pub(crate) const OC1123_SPRINTF: Callee = Callee {
    addr: 0x0041_7828,
    f: ee::s15::f_00417828,
};

/// `libmpeg_Error` (0x4034a8).
pub(crate) const OC1123_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::f_004034a8,
};

/// `DIntr` (0x42c078): disables interrupts, returns the previous state.
pub(crate) const OC1123_DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::f_0042c078,
};

/// `EIntr` (0x42c0c0): re-enables interrupts.
pub(crate) const OC1123_EINTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::f_0042c0c0,
};

/// `__muldi3` (0x4114d0): libgcc's 64-bit multiply.
pub(crate) const OC1123_MULDI3: Callee = Callee {
    addr: 0x0041_14d0,
    f: ee::s15::f_004114d0,
};

/// `libmpeg_IpuCommand` (0x406df0).
pub(crate) const OC1123_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::f_00406df0,
};

/// `libmpeg_DispatchCallback` (0x402c30).
pub(crate) const OC1123_DISPATCH: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::f_00402c30,
};

/// `AddDmacHandler` (0x423ac0): registers a DMA channel handler, returns its id.
pub(crate) const OC1123_ADD_HANDLER: Callee = Callee {
    addr: 0x0042_3ac0,
    f: ee::s15::f_00423ac0,
};

/// `RemoveDmacHandler` (0x423ad0).
pub(crate) const OC1123_REMOVE_HANDLER: Callee = Callee {
    addr: 0x0042_3ad0,
    f: ee::s15::f_00423ad0,
};

/// `EnableDmac` (0x4249b8).
pub(crate) const OC1123_ENABLE_DMAC: Callee = Callee {
    addr: 0x0042_49b8,
    f: ee::s15::f_004249b8,
};

/// `DisableDmac` (0x424950).
pub(crate) const OC1123_DISABLE_DMAC: Callee = Callee {
    addr: 0x0042_4950,
    f: ee::s15::f_00424950,
};

/// `int libmpeg_CheckOutputBuffer(MpegDec* dec, int* picSize)` (0x4081d8, 160
/// bytes). Whether the picture fits the decoder's output buffer: with
/// `dec[0xE0]` rows, the buffer is `dec[0xDC]` wide and `dec[0xE0]` high and the
/// picture `picSize[1]` x `picSize[2]`; without rows (`dec[0xE0] == 0`) the
/// buffer is `dec[0xE4]` bytes and the picture `picSize[3] * picSize[4]`. When
/// it does not fit, the required size goes to `libmpeg_Error` through a 256-byte
/// stack buffer. Returns the fits flag in v0.
pub fn oc1123_check_output_buffer(c: &mut Cpu) {
    guest(c, oc1123_check_output_buffer_body);
}

pub(crate) fn oc1123_check_output_buffer_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x130);
    let sp0 = sp(c);
    c.sd(sp0 + 0x100, c.g[S0][0]);
    c.sd(sp0 + 0x120, c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.sd(sp0 + 0x110, c.g[S1][0]);
    let dec = c.g[S0][0] as u32;
    c.g[A0][0] = Addr(dec + 0xe0).lw(c);
    c.g[A2][0] = c.g[A1][0];
    let pic = c.g[A2][0] as u32;
    if c.g[A0][0] == 0 {
        // Flat buffer: dec[0xE4] against picSize[3] * picSize[4].
        c.cycles += 6;
        c.g[V1][0] = Addr(pic + 0xc).lw(c);
        c.g[A0][0] = Addr(pic + 0x10).lw(c);
        c.g[V0][0] = Addr(dec + 0xe4).lw(c);
        c.mult(0, V1, V1, A0);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[S1][0] = c.g[V0][0] ^ 0x1;
    } else {
        // Rows: the width against picSize[1], then the rows against picSize[2].
        c.cycles += 5;
        c.g[V0][0] = Addr(dec + 0xdc).lw(c);
        c.g[V1][0] = Addr(pic + 0x4).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[S1][0] = 0;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(pic + 0x8).lw(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[S1][0] = c.g[V0][0] ^ 0x1;
        }
    }

    c.cycles += 2;
    c.g[V0][0] = c.g[S1][0];
    if c.g[S1][0] == 0 {
        // Does not fit: report the picture size.
        c.cycles += 10;
        c.g[A3][0] = Addr(pic + 0x8).lw(c);
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_sub(0x7f58));
        c.g[A2][0] = Addr(pic + 0x4).lw(c);
        c.g[A0][0] = sx(sp0);
        OC1123_SPRINTF.call(c, 0x0040_8254)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(sp0);
        OC1123_ERROR.call(c, 0x0040_8260)?;
        c.g[V0][0] = c.g[S1][0];
    }
    c.cycles += 5;
    c.g[RA][0] = c.ld(sp0 + 0x120);
    c.g[S1][0] = c.ld(sp0 + 0x110);
    c.g[S0][0] = c.ld(sp0 + 0x100);
    frame.close(c);
    Ok(())
}

/// `long libmpeg_ComputePts(MpegDec* dec, PicRecord* pic, long* ptsOut,
/// long* dtsOut, long* flagsOut)` (0x408530, 396 bytes). Presentation and
/// decode timestamps of a picture and its flag word. The PTS comes from the
/// picture record, or is extrapolated from the last PTS (`dec[0x80]`) with the
/// frame rate (`dec[0x78]`, `dec[0x88]`, the round counter `dec[0x90]`) when the
/// record has none. A pending PTS hand-off (`dec[0xF8] == 2`) overrides it.
/// Returns the flag word in v0.
pub fn oc1123_compute_pts(c: &mut Cpu) {
    guest(c, oc1123_compute_pts_body);
}

pub(crate) fn oc1123_compute_pts_body(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0xb0);
    let sp0 = sp(c);
    c.sd(sp0 + 0x90, c.g[FP][0]);
    c.sd(sp0 + 0x60, c.g[S5][0]);
    c.g[FP][0] = c.g[A3][0];
    c.sd(sp0 + 0x50, c.g[S4][0]);
    c.g[S5][0] = c.g[A2][0];
    c.sd(sp0 + 0x40, c.g[S3][0]);
    c.g[S4][0] = c.g[A1][0];
    c.sd(sp0 + 0xa0, c.g[RA][0]);
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp0 + 0x80, c.g[S7][0]);
    c.sd(sp0 + 0x70, c.g[S6][0]);
    c.sd(sp0 + 0x30, c.g[S2][0]);
    c.sd(sp0 + 0x20, c.g[S1][0]);
    c.sd(sp0 + 0x10, c.g[S0][0]);

    let dec = c.g[S3][0] as u32;
    let pic = c.g[S4][0] as u32;
    let pts_out = c.g[S5][0] as u32;
    c.g[V0][0] = Addr(dec + 0x70).lw(c);
    let no_state = c.g[V0][0] == 0;
    // The fifth argument (flagsOut) is in t0; it is stored in the caller's
    // argument slot and read back at the end.
    c.sw(sp0, c.g[T0][0]);
    if no_state {
        c.cycles += 2;
        c.g[V0][0] = Addr(pic + 0x18).ld(c);
        c.sd(pts_out, c.g[V0][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(pic + 0x18).ld(c);
        if (c.g[V0][0] as i64) >= 0 {
            c.sd(pts_out, c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[S7][0] = Addr(dec + 0x80).lw(c);
            if (c.g[S7][0] as i64) < 0 {
                c.sd(pts_out, c.g[V0][0]);
            } else {
                oc1123_pts_extrapolate(c, dec, pts_out)?;
            }
        }
    }

    // A pending hand-off (fF8 == 2 with fF0 >= 0) overrides the PTS and
    // clears itself. The flags are taken from the picture either way.
    c.cycles += 4;
    c.g[V1][0] = Addr(dec + 0xf8).lw(c);
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[A1][0] = Addr(pic + 0x40).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(dec + 0xf0).ld(c);
        if (c.g[V0][0] as i64) < 0 {
            c.g[A1][0] = Addr(pic + 0x40).lw(c);
        } else {
            c.cycles += 5;
            c.sd(pts_out, c.g[V0][0]);
            c.g[V0][0] = u64::MAX;
            c.sw(dec + 0xf8, 0);
            c.sd(dec + 0xf0, c.g[V0][0]);
            c.g[A1][0] = Addr(pic + 0x40).lw(c);
        }
    }

    c.cycles += 31;
    c.g[A0][0] = Addr(pic + 0x3c).lw(c);
    c.g[V0][0] = Addr(pic + 0x34).lw(c);
    c.g[A1][0] = c.g[A1][0] << 5;
    c.g[A0][0] = c.g[A0][0] << 6;
    c.g[A2][0] = Addr(pic + 0x30).lw(c);
    c.g[A3][0] = Addr(pic + 0x2c).lw(c);
    c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
    c.g[V1][0] = Addr(pic + 0x38).lw(c);
    c.g[V0][0] = c.g[V0][0] << 8;
    c.g[A0][0] = Addr(pic + 0x20).ld(c);
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.g[A2][0] = c.g[A2][0] << 3;
    c.g[V1][0] = c.g[V1][0] << 7;
    // *dtsOut = picRecord->dts
    c.sd(c.g[FP][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    c.g[RA][0] = c.ld(sp0 + 0xa0);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[FP][0] = c.ld(sp0 + 0x90);
    c.g[V1][0] = Addr(sp0).lw(c);
    c.g[S7][0] = c.ld(sp0 + 0x80);
    c.g[S6][0] = c.ld(sp0 + 0x70);
    c.g[S5][0] = c.ld(sp0 + 0x60);
    c.g[S4][0] = c.ld(sp0 + 0x50);
    c.g[S3][0] = c.ld(sp0 + 0x40);
    c.g[S2][0] = c.ld(sp0 + 0x30);
    c.g[S1][0] = c.ld(sp0 + 0x20);
    c.g[S0][0] = c.ld(sp0 + 0x10);
    // *flagsOut = picFlags (the flags word, the return value too)
    c.sd(c.g[V1][0] as u32, c.g[V0][0]);
    frame.close(c);
    Ok(())
}

/// The extrapolated PTS of `libmpeg_ComputePts` (its branch with a last PTS
/// and no PTS on the picture): `lastPts + (f78 * x >> 1) + y`, with `x` =
/// `dec[0x88]`, `y` = the odd-round correction, and the round counter bumped
/// when both frame-rate parities are odd. Writes the PTS to `pts_out`.
pub(crate) fn oc1123_pts_extrapolate(c: &mut Cpu, dec: u32, pts_out: u32) -> Flow {
    c.cycles += 28;
    c.g[S0][0] = Addr(dec + 0x88).ld(c);
    c.g[A1][0] = Addr(dec + 0x78).ld(c);
    c.g[S0][0] = c.g[S0][0] << 32;
    c.g[S0][0] = ((c.g[S0][0] as i64) >> 32) as u64;
    c.g[S2][0] = c.g[S0][0] & 0x1;
    c.g[A1][0] &= 0x1;
    c.g[A0][0] = c.g[S2][0];
    // bothOdd = (x & 1) * (f78 & 1)
    OC1123_MULDI3.call(c, 0x0040_85b0)?;
    c.g[S6][0] = Addr(dec + 0x90).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S6][0] & 0x1;
    // y = bothOdd * (roundCount & 1)
    OC1123_MULDI3.call(c, 0x0040_85c0)?;
    c.g[A0][0] = Addr(dec + 0x78).ld(c);
    c.g[S1][0] = c.g[V0][0] << 32;
    c.g[S1][0] = ((c.g[S1][0] as i64) >> 32) as u64;
    c.g[A1][0] = c.g[S0][0];
    // f78 * x, halved (bits 1..32) and added to lastPts and y
    OC1123_MULDI3.call(c, 0x0040_85d4)?;
    c.g[V0][0] = c.g[V0][0] << 31;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[A0][0] = c.g[S2][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sd(pts_out, c.g[V0][0]);
    // (x & 1) * (f78 & 1): the round counter moves when it is non-zero
    c.g[A1][0] = Addr(dec + 0x78).ld(c);
    c.g[A1][0] &= 0x1;
    OC1123_MULDI3.call(c, 0x0040_85f8)?;
    let bump = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
    if bump {
        c.cycles += 2;
        c.sw(dec + 0x90, c.g[V0][0]);
    }
    Ok(())
}

/// `void libmpeg_IpuFromDmaStart(u32 dst, int byteCount)` (0x408af8, 132
/// bytes). Starts a DMA transfer from the IPU into RAM (channel 3, the
/// `0x1000B000` block: address, quadword count, start) with the interrupts
/// off, then restores the interrupt state (EIntr as a tail call when they were
/// on).
pub fn oc1123_ipu_from_dma_start(c: &mut Cpu) {
    guest(c, oc1123_ipu_from_dma_start_body);
}

pub(crate) fn oc1123_ipu_from_dma_start_body(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x30);
    let sp0 = sp(c);
    c.sd(sp0 + 0x10, c.g[S1][0]);
    c.sd(sp0, c.g[S0][0]);
    c.g[S1][0] = c.g[A1][0];
    c.sd(sp0 + 0x20, c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    OC1123_DINTR.call(c, 0x0040_8b14)?;
    c.g[V1][0] = 0x0fff_ffff;
    c.g[A2][0] = 0xffff_ffff_8000_0000;
    c.g[S0][0] &= c.g[V1][0];
    c.g[A1][0] = 0x1000_b010;
    c.g[S0][0] |= c.g[A2][0];
    // MADR = (dst & 0x0fffffff) | 0x80000000 (the uncached mirror)
    c.sw(c.g[A1][0] as u32, c.g[S0][0]);
    c.g[A0][0] = 0x1000_b020;
    c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 4) as u32);
    c.g[V1][0] = 0x1000_0000;
    // QWC = byteCount / 16
    c.sw(c.g[A0][0] as u32, c.g[S1][0]);
    c.g[V1][0] = 0x1000_b000;
    c.g[A0][0] = 0x100;
    c.sw(c.g[V1][0] as u32, c.g[A0][0]);
    let intr_was_off = c.g[V0][0] == 0;
    c.g[RA][0] = c.ld(sp0 + 0x20);
    if intr_was_off {
        c.cycles += 4;
        c.g[S1][0] = c.ld(sp0 + 0x10);
        c.g[S0][0] = c.ld(sp0);
        frame.close(c);
    } else {
        c.cycles += 4;
        c.g[S1][0] = c.ld(sp0 + 0x10);
        c.g[S0][0] = c.ld(sp0);
        // Tail call: EIntr returns straight to our caller.
        c.g[SP][0] = sx(sp0.wrapping_add(0x30));
        c.pc = 0x0042_c0c0;
        (OC1123_EINTR.f)(c);
    }
    Ok(())
}

/// `void libmpeg_IpuCsc(MpegDec* dec, u32 addr, int n)` (0x408b80, 296 bytes).
/// Converts `n` macroblocks to RGB with the IPU, the DMA writing to `addr`
/// (channel 3, quadword count `n << 6`). It waits for the IPU to go idle, runs
/// the IPU command `n | 0x70000000` through `libmpeg_IpuCommand`, then waits
/// for the DMA to finish. Interrupts are off during the start (EIntr when they
/// were on).
pub fn oc1123_ipu_csc(c: &mut Cpu) {
    guest(c, oc1123_ipu_csc_body);
}

pub(crate) fn oc1123_ipu_csc_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x70);
    let sp0 = sp(c);
    c.g[V1][0] = 0x1000_2010;
    c.sd(sp0 + 0x50, c.g[S3][0]);
    c.sd(sp0 + 0x40, c.g[S2][0]);
    c.g[S3][0] = c.g[A2][0];
    c.sd(sp0 + 0x30, c.g[S1][0]);
    c.g[S2][0] = c.g[A0][0];
    c.sd(sp0 + 0x20, c.g[S0][0]);
    c.g[S1][0] = sx((c.g[S3][0] as u32) << 6);
    c.sd(sp0 + 0x60, c.g[RA][0]);
    c.g[S0][0] = c.g[A1][0];

    // Wait while the IPU is busy (IPU control word, sign bit).
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8bb0);
        }
    }

    c.cycles += 17;
    OC1123_DINTR.call(c, 0x0040_8bd4)?;
    c.g[V1][0] = 0x0fff_ffff;
    c.g[A1][0] = 0x1000_b010;
    c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
    c.g[A0][0] = 0x1000_0000;
    // MADR = addr & 0x0fffffff
    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
    c.g[A0][0] = 0x1000_b020;
    // QWC = n << 6
    c.sw(c.g[A0][0] as u32, c.g[S1][0]);
    c.g[V1][0] = 0x1000_b000;
    c.g[A0][0] = 0x100;
    c.sw(c.g[V1][0] as u32, c.g[A0][0]);
    if c.g[V0][0] == 0 {
        c.g[A1][0] = 0x7000_0000;
    } else {
        c.cycles += 3;
        OC1123_EINTR.call(c, 0x0040_8c18)?;
        c.g[A1][0] = 0x7000_0000;
    }

    c.cycles += 11;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S3][0] | c.g[A1][0];
    OC1123_IPU_COMMAND.call(c, 0x0040_8c28)?;
    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x858)).lw(c);
    c.g[V0][0] = 0x4;
    c.sw(sp0, c.g[V0][0]);
    c.g[A1][0] = sx(sp0);
    OC1123_DISPATCH.call(c, 0x0040_8c3c)?;
    c.g[V1][0] = 0x1000_b000;

    // Wait for the DMA channel to finish (STR, bit 8 of CHCR).
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_b000).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[V0][0] &= 0x1;
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8c48);
        }
    }

    c.cycles += 3;
    c.g[V1][0] = 0x1000_2010;
    // Then wait for the IPU to go idle again.
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8c70);
        }
    }

    c.cycles += 7;
    c.g[RA][0] = c.ld(sp0 + 0x60);
    c.g[S3][0] = c.ld(sp0 + 0x50);
    c.g[S2][0] = c.ld(sp0 + 0x40);
    c.g[S1][0] = c.ld(sp0 + 0x30);
    c.g[S0][0] = c.ld(sp0 + 0x20);
    frame.close(c);
    Ok(())
}

/// `void libmpeg_IpuCscDma(MpegDec* dec, u32 addr, int size)` (0x408df0, 448
/// bytes). The long-picture variant of `libmpeg_IpuCsc`: it registers a DMA
/// chunk handler (`0x408ca8`) that feeds the transfer in 1023-quadword chunks
/// (`n = size / 1023 + 1` of them, counted in `cnt` in the transfer state),
/// starts the IPU CSC command, waits for all chunks, and reports a handler
/// stop through `libmpeg_Error`. Interrupts are off during the start.
pub fn oc1123_ipu_csc_dma(c: &mut Cpu) {
    guest(c, oc1123_ipu_csc_dma_body);
}

pub(crate) fn oc1123_ipu_csc_dma_body(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A3][0] = 0x3ff;
    let frame = Frame::open(c, 0x80);
    c.div(0, A2, A3);
    let sp0 = sp(c);
    c.sd(sp0 + 0x40, c.g[S0][0]);
    // addr + 0xFFC00, masked to 28 bits
    c.g[V0][0] = 0x000f_fc00;
    c.g[S0][0] = c.g[A1][0];
    c.g[V1][0] = 0x0fff_ffff;
    c.sd(sp0 + 0x50, c.g[S1][0]);
    c.sd(sp0 + 0x70, c.g[RA][0]);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sd(sp0 + 0x60, c.g[S2][0]);
    c.g[V0][0] &= c.g[V1][0];

    // The transfer state on the stack: cnt (+0x20), stop (+0x24), size
    // (+0x28), addr (+0x2c), n (+0x30).
    c.cycles += 12;
    c.g[V1][0] = 0x1000_0000;
    c.sw(sp0 + 0x28, c.g[A2][0]);
    c.g[S1][0] = c.g[A0][0];
    c.sw(sp0 + 0x2c, c.g[V0][0]);
    c.g[A3][0] = sx(sp0 + 0x20);
    c.sw(sp0 + 0x24, 0);
    c.g[V1][0] |= 0x2010;
    c.sw(sp0 + 0x20, 0);
    c.g[T0][0] = c.lo[0];
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
    c.sw(sp0 + 0x30, c.g[T0][0]);

    // Wait while the IPU is busy.
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8e60);
        }
    }

    c.cycles += 30;
    c.g[A1][0] = 0x0041_0000;
    c.g[A0][0] = 0x3;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7358));
    c.g[A2][0] = 0;
    // Chunk handler 0x408ca8 on DMA channel 3.
    OC1123_ADD_HANDLER.call(c, 0x0040_8e90)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = 0x8;
    c.g[V0][0] = 0x1000_e010;
    c.g[A0][0] = 0x3;
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    OC1123_ENABLE_DMAC.call(c, 0x0040_8eac)?;
    OC1123_DINTR.call(c, 0x0040_8eb4)?;
    c.g[V1][0] = 0x0fff_ffff;
    c.g[A0][0] = 0x1000_b010;
    c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
    c.g[A1][0] = 0x1000_0000;
    // MADR = addr & 0x0fffffff
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.g[A1][0] |= 0xb020;
    c.g[V1][0] = 0xffc0;
    c.g[A0][0] = 0x1000_0000;
    // QWC = 0xffc0 (one chunk)
    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
    c.g[A0][0] |= 0xb000;
    c.g[V1][0] = 0x100;
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    let intr_was_off = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1000_0000;
    if !intr_was_off {
        c.cycles += 3;
        OC1123_EINTR.call(c, 0x0040_8efc)?;
        c.g[V0][0] = 0x1000_0000;
    }

    // Start the IPU CSC command (0x70000000 | 3 ... the DMA-driven form).
    c.cycles += 12;
    c.g[V1][0] = 0x7000_0000;
    c.g[V0][0] |= 0x2000;
    c.g[V1][0] |= 0x3ff;
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    c.g[A2][0] = 0x4;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x858)).lw(c);
    c.g[A1][0] = sx(sp0);
    c.sw(sp0, c.g[A2][0]);
    OC1123_DISPATCH.call(c, 0x0040_8f24)?;
    c.g[A0][0] = Addr(sp0 + 0x24).lw(c);
    c.g[V1][0] = Addr(sp0 + 0x30).lw(c);

    // Wait until every chunk has been fed (cnt < n).
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(sp0 + 0x20).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8f30);
        }
    }

    // A handler stop is reported through libmpeg_Error.
    c.cycles += 2;
    let stopped = c.g[A0][0] != 0;
    c.g[A0][0] = c.g[S1][0];
    if stopped {
        c.cycles += 3;
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_sub(0x7f28));
        OC1123_ERROR.call(c, 0x0040_8f60)?;
    }

    c.cycles += 2;
    c.g[V1][0] = 0x1000_2010;
    // Wait for the IPU to go idle.
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(0x1000_2010).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_8f68);
        }
    }

    c.cycles += 11;
    c.g[A0][0] = 0x3;
    OC1123_DISABLE_DMAC.call(c, 0x0040_8f8c)?;
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = 0x3;
    OC1123_REMOVE_HANDLER.call(c, 0x0040_8f98)?;
    c.g[RA][0] = c.ld(sp0 + 0x70);
    c.g[S2][0] = c.ld(sp0 + 0x60);
    c.g[S1][0] = c.ld(sp0 + 0x50);
    c.g[S0][0] = c.ld(sp0 + 0x40);
    frame.close(c);
    Ok(())
}

pub(crate) const OC1137_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};

pub(crate) const OC1137_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::libmpeg_FlushBits,
};

pub(crate) const OC1137_SHOW_BITS: Callee = Callee {
    addr: 0x0040_70f0,
    f: ee::s15::libmpeg_ShowBits,
};

/// `libmpeg_NextStartCode(sys)` (0x407438): byte-aligns the bit stream (the
/// IPU's bit position `0x10002020 & 7`, `FlushBits` by the missing bits),
/// then flushes bytes until `ShowBits(sys, 24)` is 1 (the `00 00 01` start
/// code prefix). Delivers a due event at the loop's back edge; a resume at
/// that head (`0x407480`) runs through the generated body.
pub fn libmpeg_next_start_code(c: &mut Cpu) {
    if c.pc != 0x0040_7438 {
        return ee::s15::f_00407438__generated(c);
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        OC1137_IPU_WAIT_BUSY.call(c, 0x0040_7450)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[V0][0] |= 0x2020;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] &= 7;
        c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
        c.g[A1][0] = c.g[V1][0] & 7;
        let aligned = c.g[A1][0] == 0;
        c.g[S1][0] = 1;
        if !aligned {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            OC1137_FLUSH_BITS.call(c, 0x0040_7478)?;
            c.g[S1][0] = 1;
        }
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x18;
            OC1137_SHOW_BITS.call(c, 0x0040_7494)?;
            c.g[A0][0] = c.g[S0][0];
            let more = c.g[V0][0] != c.g[S1][0];
            c.g[A1][0] = 8;
            if !more {
                break;
            }
            if back_edge_unwinds(c, 0x0040_7480) {
                return Ok(());
            }
            c.cycles += 2;
            OC1137_FLUSH_BITS.call(c, 0x0040_7488)?;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// Offsets used by `libmpeg_ClearFrameFlags` (0x402bb0). `mpeg[16]` (the
/// MPEG system block) sits at +0x40 of the argument; the picture records
/// hang off it in six slots, and each record's flag word is at +0x28.
pub(crate) const CLEAR_FRAME_SYS: u32 = 0x40;

pub(crate) const CLEAR_FRAME_FLAGS: u32 = 0x28;

/// `int libmpeg_ClearFrameFlags(mpeg)` (0x402bb0, `sceMpegClearRefBuff`):
/// clears the flag word of each reference picture record that is present,
/// then returns 1. The six slots are read in the original's order.
pub fn libmpeg_clear_frame_flags(c: &mut Cpu) {
    c.cycles += 4;
    let mpeg = c.g[A0][0] as u32;
    let sys = Addr(mpeg).off(CLEAR_FRAME_SYS).lw(c);
    c.g[V1][0] = sys;
    clear_frame_slot(c, sys, 0x1b8);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1c8);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1d8);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1bc);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1cc);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1dc);
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// One slot of `libmpeg_ClearFrameFlags`: `v0 = sys[slot]`, then clear the
/// flag word of that record if it is present.
pub(crate) fn clear_frame_slot(c: &mut Cpu, sys: u64, slot: u32) {
    let record = Addr(sys as u32).off(slot).lw(c);
    c.g[V0][0] = record;
    if record != 0u64 {
        Addr(record as u32).off(CLEAR_FRAME_FLAGS).sw(c, 0);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// A machine with the game image loaded (jump tables, `.lit4`), `gp`
    /// set and `pc` at `entry` (the generated slot dispatches on it).
    pub(crate) fn oc383_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = Rng(0x0383_0000_0000 ^ salt ^ (seed as u64) << 20);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = entry;
        r
    }

    #[test]
    fn ieee754_rem_pio2f_matches() {
        check(
            ee::s15::f_0040e878__generated,
            ieee754_rem_pio2f,
            400,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0xe878, 0x0040_e878);
                let mag = match seed % 10 {
                    0 => r.next_u32() % 0x3f49_0fd9,              // |x| <= pi/4
                    1 => 0x3f49_0fd9 + r.next_u32() % 0xcd_bc0b,  // < 3pi/4
                    2 => 0x3fc9_0fd0 + r.next_u32() % 0x10,       // ~pi/2
                    3 => 0x4016_cbe4 + r.next_u32() % 0x1b2_4000, // medium
                    4 => (n_pio2(r.next_u32() % 31 + 1)) ^ (r.next_u32() % 0x100), // near n*pi/2
                    5 => 0x4349_0f81 + r.next_u32() % 0x3b00_0000, // large
                    6 => [0x7f80_0000, 0x7fc0_0000, 0x7f7f_ffff, 0x4349_0f80]
                        [(r.next_u32() % 4) as usize],
                    7 => 0x4400_0000 + r.next_u32() % 0x0800_0000, // large, few bits
                    _ => r.next_u32() & 0x7fff_ffff,
                };
                // Large values with trailing zero parts (the nx scan).
                let mag = if seed % 10 == 7 {
                    mag & 0xffff_0000
                } else {
                    mag
                };
                let sign = if r.next_u32() & 1 == 0 {
                    0
                } else {
                    0x8000_0000
                };
                c.f[12] = mag | sign;
                fill(c, &mut r, DATA, 4);
                c.g[A0][0] = sx(DATA);
            },
        );

        /// n * pi/2 as a float's bits (the npio2_hw entries' neighbourhood).
        fn n_pio2(n: u32) -> u32 {
            (n as f32 * std::f32::consts::FRAC_PI_2).to_bits()
        }
    }

    use crate::difftest::check_paths;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::Events;
    use crate::game::misc::tests::event_at;
    use crate::game::misc::tests::require_heads;
    use crate::game::seg::seg_2fe810::REM_PIO2F_TWO_OVER_PI;

    /// The generated body the tests compare against.
    pub(crate) const KERNEL_REM_PIO2F_GENERATED: GuestFn = ee::s15::f_0040ec58__generated;

    pub(crate) const KERNEL_REM_PIO2F_ENTRY: u32 = 0x0040_ec58;

    /// The x pieces, y[] (pre-filled) and a made-up `ipio2` table.
    pub(crate) const KERNEL_REM_PIO2F_X_AT: u32 = DATA + 0x24;

    pub(crate) const KERNEL_REM_PIO2F_Y_AT: u32 = DATA + 0x134;

    pub(crate) const KERNEL_REM_PIO2F_TABLE_AT: u32 = DATA + 0x404;

    /// The function's frame below the test stack.
    pub(crate) const KERNEL_REM_PIO2F_FRAME_AT: u32 =
        crate::difftest::SP - kernel_rem_pio2f_slot::SIZE;

    /// A machine with registers scrambled, the game image loaded (init_jk,
    /// PIo2, two_over_pi), gp set and pc at the entry.
    pub(crate) fn kernel_rem_pio2f_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x0040_ec58_0000 ^ salt ^ (seed as u64) << 20);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = KERNEL_REM_PIO2F_ENTRY;
        r
    }

    /// One call: x pieces (float bits), e0, prec, the `ipio2` table, and a
    /// replacement for `init_jk[prec]` (its word written over in RAM).
    pub(crate) struct KernelRemPio2fCall {
        pub(crate) x: Vec<u32>,
        pub(crate) e0: i32,
        pub(crate) prec: i32,
        pub(crate) ipio2: u32,
        pub(crate) jk: Option<i32>,
    }

    /// Place `call`: y[] and the frame pre-filled with junk, the three words
    /// below the frame (`iq[-1..-3]`, read when jk is 0 or below) set from
    /// `below`, the arguments in a0-a3/t0/t1.
    pub(crate) fn kernel_rem_pio2f_place(
        c: &mut Cpu,
        r: &mut Rng,
        call: &KernelRemPio2fCall,
        below: [u32; 3],
    ) {
        fill(
            c,
            r,
            KERNEL_REM_PIO2F_FRAME_AT - 0x40,
            (kernel_rem_pio2f_slot::SIZE + 0x40) / 4,
        );
        for (i, w) in below.iter().enumerate() {
            c.bus
                .write32(KERNEL_REM_PIO2F_FRAME_AT - 4 * (i as u32 + 1), *w);
        }
        fill(c, r, KERNEL_REM_PIO2F_Y_AT, 4);
        for (i, w) in call.x.iter().enumerate() {
            c.bus.write32(KERNEL_REM_PIO2F_X_AT + 4 * i as u32, *w);
        }
        if let Some(jk) = call.jk {
            let at = KERNEL_REM_PIO2F_INIT_JK.wrapping_add((call.prec as u32) << 2);
            c.bus.write32(at, jk as u32);
        }
        c.g[A0][0] = sx(KERNEL_REM_PIO2F_X_AT);
        c.g[A1][0] = sx(KERNEL_REM_PIO2F_Y_AT);
        c.g[A2][0] = sx(call.e0 as u32);
        c.g[A3][0] = sx(call.x.len() as u32);
        c.g[T0][0] = sx(call.prec as u32);
        c.g[T1][0] = sx(call.ipio2);
    }

    /// A made-up `ipio2` table of chunks 1..=254 with `zeros` cleared.
    pub(crate) fn kernel_rem_pio2f_table(c: &mut Cpu, r: &mut Rng, first: u32, zeros: &[usize]) {
        for i in 0..64u32 {
            let v = if i == 0 {
                first
            } else {
                1 + r.next_u32() % 254
            };
            c.bus.write32(KERNEL_REM_PIO2F_TABLE_AT + 4 * i, v);
        }
        for &i in zeros {
            c.bus.write32(KERNEL_REM_PIO2F_TABLE_AT + 4 * i as u32, 0);
        }
    }

    /// The caller's x pieces: x[0] in 128..=255, the others 0..=255, the
    /// last non-zero, each times `scale`.
    pub(crate) fn kernel_rem_pio2f_pieces(r: &mut Rng, nx: usize, scale: f32) -> Vec<u32> {
        let mut x: Vec<u32> = (0..nx)
            .map(|i| {
                if i == 0 {
                    128 + r.next_u32() % 128
                } else {
                    r.next_u32() % 256
                }
            })
            .collect();
        if x[nx - 1] == 0 {
            x[nx - 1] = 1;
        }
        x.iter().map(|&v| (v as f32 * scale).to_bits()).collect()
    }

    /// A seeded call: precision 0-3, q0 = -5..2 (e0 - 3 mod 8), and an
    /// input kind: the caller's pieces with the game's two_over_pi table
    /// (kinds 0, 1), pieces that are multiples of 256 (a zero low chunk,
    /// kind 2), x = 1 with a made-up table whose chunks 1 (and 2)
    /// are zero (recomputation, kind 3), e0 below 3 (jv clamped, z >= 256,
    /// kind 4). `wide` picks a large jv and, for prec 3, init_jk[3] = 4..9
    /// instead of the 0 in the image. Every digit is independent of the
    /// others: the seed is `((((wide * 5 + kind) * 8 + residue) * 4 + prec)
    /// * 3 + nx) * 3 + below`.
    pub(crate) fn kernel_rem_pio2f_case(c: &mut Cpu, seed: u32) {
        let mut r = kernel_rem_pio2f_machine(c, seed, 0xec58);
        let prec = (seed / 9 % 4) as i32;
        let residue = (seed / 36 % 8) as i32;
        let kind = seed / 288 % 5;
        let wide = seed / 1440 % 2 == 1;
        let nx = 1 + (seed / 3 % 3) as usize;
        let below = match seed % 3 {
            0 => [0, 0, 1 + r.next_u32() % 255],
            1 => [0, 1 + r.next_u32() % 255, 1 + r.next_u32() % 255],
            _ => [
                1 + r.next_u32() % 255,
                1 + r.next_u32() % 255,
                1 + r.next_u32() % 255,
            ],
        };
        let jv = (if wide {
            2 + r.next_u32() % 12
        } else {
            r.next_u32() % 2
        }) as i32;
        let jk = if prec == 3 && wide {
            Some(4 + (r.next_u32() % 6) as i32)
        } else {
            None
        };
        let mut call = KernelRemPio2fCall {
            x: Vec::new(),
            e0: 3 + 8 * jv + residue,
            prec,
            ipio2: REM_PIO2F_TWO_OVER_PI,
            jk,
        };
        match kind {
            0 | 1 => call.x = kernel_rem_pio2f_pieces(&mut r, nx, 1.0),
            2 => call.x = kernel_rem_pio2f_pieces(&mut r, nx, 256.0),
            3 => {
                call.x = [1.0f32, 0.0, 0.0][..nx]
                    .iter()
                    .map(|v| v.to_bits())
                    .collect();
                call.e0 = 3 + residue;
                call.ipio2 = KERNEL_REM_PIO2F_TABLE_AT;
                let first = 1 + r.next_u32() % 254;
                let zeros: &[usize] = if wide { &[1, 2][..] } else { &[1][..] };
                kernel_rem_pio2f_table(c, &mut r, first, zeros);
            }
            _ => {
                call.x = kernel_rem_pio2f_pieces(&mut r, nx, 1.0);
                call.e0 = -12 + 2 * residue;
            }
        }
        kernel_rem_pio2f_place(c, &mut r, &call, below);
    }

    /// `__kernel_rem_pio2f` over every precision, q0 class and input kind:
    /// the game's 2/pi table, recomputation with more terms, zero chunks,
    /// rounding up with the borrow, z >= 256, init_jk[3] as in the image.
    #[test]
    fn kernel_rem_pio2f_matches() {
        check(
            KERNEL_REM_PIO2F_GENERATED,
            kernel_rem_pio2f,
            2880,
            &|c, seed| kernel_rem_pio2f_case(c, seed),
        );
    }

    /// Short calls with init_jk[prec] = -1 (m < 0, no terms, z read from
    /// `q[-1]`, the chunk below the frame as `iq[jz-1]`), and prec 15 (the
    /// image's 0 after init_jk, an unknown precision):
    /// 0: prec 0, q0 = 1, ih = 1 (mask 0x7f), y[0] = -0: 273 cycles;
    /// 1: prec 1, q0 = 2, ih = 1 (mask 0x3f), both y negated: 287;
    /// 2: prec 3, q0 = 0, ih = 1 from iq[jz-1] >> 8, y negated: 265;
    /// 3: prec 3, q0 = -5, z >= 0.5 (ih = 2, no borrow), y negated: 270;
    /// 4: prec 3, q0 = -5, ih = 0: 255;
    /// 5: prec 0, e0 = -6 (jv clamped to 0, q0 = -14), ih = 2, z >= 256
    ///    split, one chunk scaled, jp < 0 sum, y[0] summed, negated: 315;
    /// 6: prec 15 (jk = 0), one term and one PIo2 product, no y: 330;
    /// 7: prec -2, no y: 238.
    #[test]
    fn kernel_rem_pio2f_short_paths_match() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = kernel_rem_pio2f_machine(c, seed, 0xec59);
            // (prec, init_jk patch, e0, z = q[-1], iq[-2])
            let (prec, jk, e0, z, iq2): (i32, Option<i32>, i32, f32, u32) = match seed % 8 {
                0 => (0, Some(-1), 9, 3.25, 0x1c5),
                1 => (1, Some(-1), 10, 3.375, 0xe1),
                2 => (3, Some(-1), 8, 3.25, 0x1ff),
                3 => (3, Some(-1), 3, 20.0, 0x11),
                4 => (3, Some(-1), 3, 8.0, 0x11),
                5 => (0, Some(-1), -6, 12288.0, 0x11),
                6 => (15, None, 3, 0.0, 0x11),
                _ => (-2, Some(-1), 3, 8.0, 0x11),
            };
            let call = KernelRemPio2fCall {
                x: vec![1.0f32.to_bits()],
                e0,
                prec,
                ipio2: REM_PIO2F_TWO_OVER_PI,
                jk,
            };
            kernel_rem_pio2f_place(c, &mut r, &call, [0x33, iq2, 0x55]);
            // q[-1] is fq[19], read as z when jz = -1.
            let q_minus_1 = KERNEL_REM_PIO2F_FRAME_AT + kernel_rem_pio2f_slot::Q - 4;
            c.bus.write32(q_minus_1, z.to_bits());
        };
        check_paths(
            KERNEL_REM_PIO2F_GENERATED,
            kernel_rem_pio2f,
            64,
            &[273, 287, 265, 270, 255, 315, 330, 238],
            &setup,
        );
    }

    /// The main paths, pinned with the generated body's cycle totals: one
    /// seed per precision, input kind, table width, piece count, chunk
    /// shape and q0 class (see `kernel_rem_pio2f_case`).
    #[test]
    fn kernel_rem_pio2f_paths_match() {
        const SEEDS: [u32; 16] = [
            36, 81, 126, 171, 72, 432, 792, 864, 1224, 1854, 21, 24, 19, 20, 180, 252,
        ];
        const TOTALS: [u64; 16] = [
            1150, 2069, 2084, 775, 1320, 1320, 1284, 1009, 1131, 2621, 2228, 2366, 2205, 2205,
            1150, 1166,
        ];
        check_paths(
            KERNEL_REM_PIO2F_GENERATED,
            kernel_rem_pio2f,
            SEEDS.len() as u32,
            &TOTALS,
            &|c, seed| kernel_rem_pio2f_case(c, SEEDS[seed as usize]),
        );
    }

    /// The event-test calls (x = 1, 0, 0 with a made-up table, so the
    /// paths are known): 0: prec 2, q0 = 1, chunks 1 and 2 zero (three more
    /// terms, the zero scan, two chopped chunks); 1: prec 0, q0 = -5,
    /// z = 0.5 (ih = 2) and chunk 4 zero (the borrow loop's three arms);
    /// 2: prec 3 with init_jk[3] = 4 and ih = 2 (both prec-3 passes).
    pub(crate) fn kernel_rem_pio2f_event_call(c: &mut Cpu, seed: u32, base: u32) {
        let mut r = kernel_rem_pio2f_machine(c, seed, 0xec5a);
        let mut t = Rng(0x0040_ec58 + base as u64);
        let (prec, jk, e0, first, zeros): (i32, Option<i32>, i32, u32, &[usize]) = match base {
            0 => (2, None, 9, 0x55, &[1, 2][..]),
            1 => (0, None, 3, 0x70, &[4][..]),
            _ => (3, Some(4), 3, 0x51, &[][..]),
        };
        kernel_rem_pio2f_table(c, &mut t, first, zeros);
        let call = KernelRemPio2fCall {
            x: [1.0f32, 0.0, 0.0].iter().map(|v| v.to_bits()).collect(),
            e0,
            prec,
            ipio2: KERNEL_REM_PIO2F_TABLE_AT,
            jk,
        };
        kernel_rem_pio2f_place(c, &mut r, &call, [0x33, 0x44, 0x55]);
    }

    thread_local! {
        /// The back-edge checks of a spied run: (loop head, cycles).
        static KERNEL_REM_PIO2F_CHECKS: std::cell::RefCell<Vec<(u32, u64)>> =
            const { std::cell::RefCell::new(Vec::new()) };
        /// Per event-test call: each loop head once, with the cycles of its
        /// first back-edge check after entering at cycle 1000.
        static KERNEL_REM_PIO2F_HEADS: std::cell::RefCell<Vec<Vec<(u32, u64)>>> =
            const { std::cell::RefCell::new(Vec::new()) };
    }

    /// Event hook that notes every back-edge check and delivers nothing.
    pub(crate) fn kernel_rem_pio2f_note_check(c: &mut Cpu) -> bool {
        assert!(
            c.cycles < crate::difftest::CYCLE_BUDGET,
            "runaway at {:#x}",
            c.pc
        );
        KERNEL_REM_PIO2F_CHECKS.with(|v| v.borrow_mut().push((c.pc, c.cycles)));
        c.next_event = 0;
        false
    }

    /// The loop heads of event-test call `base` and the cycle offset (from
    /// 1000) of each one's first back-edge check, from a spied generated
    /// run (the paths do not depend on the scrambled registers or junk).
    pub(crate) fn kernel_rem_pio2f_heads(base: u32) -> Vec<(u32, u64)> {
        let cached = KERNEL_REM_PIO2F_HEADS.with(|h| h.borrow().get(base as usize).cloned());
        if let Some(heads) = cached {
            return heads;
        }
        let mut all = Vec::new();
        for b in 0..3 {
            let mut c = crate::difftest::fresh();
            kernel_rem_pio2f_event_call(&mut c, 0, b);
            let _ = event_at(&mut c, KERNEL_REM_PIO2F_ENTRY, 0, 2);
            KERNEL_REM_PIO2F_CHECKS.with(|v| v.borrow_mut().clear());
            c.hooks.event = kernel_rem_pio2f_note_check;
            c.next_event = 0;
            KERNEL_REM_PIO2F_GENERATED(&mut c);
            let checks = KERNEL_REM_PIO2F_CHECKS.with(|v| std::mem::take(&mut *v.borrow_mut()));
            let mut heads: Vec<(u32, u64)> = Vec::new();
            for (pc, cycles) in checks {
                if !heads.iter().any(|&(h, _)| h == pc) {
                    heads.push((pc, cycles - 1000));
                }
            }
            all.push(heads);
        }
        KERNEL_REM_PIO2F_HEADS.with(|h| *h.borrow_mut() = all.clone());
        all[base as usize].clone()
    }

    /// Seeds per event-test call: two modes per targeted head.
    pub(crate) const KERNEL_REM_PIO2F_EVENT_SLOTS: u32 = 32;

    /// An event-test call with the first event due at the first back-edge
    /// check of one loop head in turn, unwinding (even slots) or not.
    pub(crate) fn kernel_rem_pio2f_events_setup(c: &mut Cpu, seed: u32) -> Events {
        let base = seed / KERNEL_REM_PIO2F_EVENT_SLOTS % 3;
        let slot = seed % KERNEL_REM_PIO2F_EVENT_SLOTS;
        let heads = kernel_rem_pio2f_heads(base);
        kernel_rem_pio2f_event_call(c, seed, base);
        let (_, at) = heads[(slot / 2) as usize % heads.len()];
        event_at(c, KERNEL_REM_PIO2F_ENTRY, at, slot % 2)
    }

    driven!(kernel_rem_pio2f_gen_driven, KERNEL_REM_PIO2F_GENERATED);

    driven!(kernel_rem_pio2f_hand_driven, kernel_rem_pio2f);

    /// `__kernel_rem_pio2f` with the first event at each of its 20 loop
    /// heads (resumes expected: none, every loop is structured).
    #[test]
    fn kernel_rem_pio2f_events() {
        use kernel_rem_pio2f_head as h;
        let seeds = 3 * KERNEL_REM_PIO2F_EVENT_SLOTS;
        check_events(
            kernel_rem_pio2f_gen_driven,
            kernel_rem_pio2f_hand_driven,
            seeds,
            &kernel_rem_pio2f_events_setup,
        );
        require_heads(
            KERNEL_REM_PIO2F_GENERATED,
            seeds,
            &[
                h::FILL_F,
                h::Q_TERM,
                h::Q_DOT,
                h::RECOMPUTE,
                h::DISTILL,
                h::CARRY,
                h::ZERO_SCAN,
                h::K_SCAN,
                h::NEW_TERM,
                h::NEW_DOT,
                h::CHOP,
                h::SCALE,
                h::FQ_TERM,
                h::FQ_DOT,
                h::SUM0,
                h::SUM12,
                h::SUM12_TAIL,
                h::FOLD3_PASS1,
                h::FOLD3_PASS2,
                h::SUM3,
            ],
            &kernel_rem_pio2f_events_setup,
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    use crate::difftest::check_lohi;
    use crate::difftest::SP as STACK_TOP;

    pub(crate) use crate::game::lib::snd::tests::SEEDS;

    /// Gate word of the oc1107 callee fakes (in the machine's RAM).
    pub(crate) const FP_CTRL: u32 = DATA + 0xf_8000;

    pub(crate) const FP_MAGIC: u32 = 0x1107_c0de;

    pub(crate) static FP_PREV_SETUP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static FP_PREV_DUAL: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static FP_PREV_ERRORF: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_SetupPredBlock`, gated: logs the call and returns.
    pub(crate) fn fp_fake_setup(c: &mut Cpu) {
        if c.bus.read32(FP_CTRL) != FP_MAGIC {
            return fakes::fall_back(c, &FP_PREV_SETUP, ee::s15::f_00404758__generated);
        }
        fakes::fake_call(c, 0x0040_4758)
    }

    /// `libmpeg_DualPrimeArithmetic`, gated: logs the call, then fills the
    /// four-word `DMV` output (a1) from the call's inputs.
    pub(crate) fn fp_fake_dual(c: &mut Cpu) {
        if c.bus.read32(FP_CTRL) != FP_MAGIC {
            return fakes::fall_back(c, &FP_PREV_DUAL, ee::s15::f_00405e30__generated);
        }
        let out = c.g[A1][0] as u32;
        let h = fakes::mix(c.g[A2][0] ^ c.g[A3][0].rotate_left(9) ^ c.g[T0][0]);
        fakes::fake_call(c, 0x0040_5e30);
        for i in 0..4u32 {
            let v = fakes::mix(h + i as u64) as u32 % 64;
            c.bus.write32(out + 4 * i, v);
        }
    }

    /// `libmpeg_Errorf`, gated: logs the call and returns.
    pub(crate) fn fp_fake_errorf(c: &mut Cpu) {
        if c.bus.read32(FP_CTRL) != FP_MAGIC {
            return fakes::fall_back(c, &FP_PREV_ERRORF, ee::s15::f_00403470__generated);
        }
        fakes::fake_call(c, 0x0040_3470)
    }

    pub(crate) fn fp_hook_callees() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00404758),
            fp_fake_setup,
            &FP_PREV_SETUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405E30),
            fp_fake_dual,
            &FP_PREV_DUAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            fp_fake_errorf,
            &FP_PREV_ERRORF,
        );
    }

    pub(crate) const FP_DEC: u32 = DATA + 0x1000;

    pub(crate) const FP_PMV: u32 = DATA + 0x4000;

    pub(crate) const FP_MVFS: u32 = DATA + 0x4100;

    pub(crate) const FP_DMV: u32 = DATA + 0x4200;

    /// The fixture: the seed walks the combinations of macroblock type (none,
    /// backward, forward, both), frame/field decoder mode (`dec+0x174`),
    /// picture structure (`dec+0x150`), motion type 0..4, the `dec+0x120`
    /// flag; the field selects, vectors and the
    /// reference pointers are random.
    pub(crate) fn fp_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x404050, 6);
        c.bus.write32(FP_CTRL, FP_MAGIC);
        let mut k = seed;
        let mut pick = |n: u32| {
            let v = k % n;
            k /= n;
            v
        };
        let mbtype = [0u32, 4, 8, 12][pick(4) as usize] | (r.next_u32() & 0x13) << 4;
        let frame_mode = [3u32, 1][pick(2) as usize];
        let structure = [1u32, 2, 3][pick(3) as usize];
        let mtype = pick(5);
        let flag120 = pick(2);
        fill(c, &mut r, FP_DEC, 0x900);
        c.bus.write32(FP_DEC + 0x174, frame_mode);
        c.bus.write32(FP_DEC + 0x150, structure);
        c.bus.write32(FP_DEC + 0x120, flag120);
        c.bus.write32(FP_DEC + 0x810, r.next_u32() % 3);
        for off in [0x1b8, 0x1bc, 0x1c8, 0x1cc, 0x1d8, 0x1dc] {
            c.bus
                .write32(FP_DEC + off, DATA + 0x6000 + 0x100 * (off & 0xff));
        }
        for i in 0..8 {
            c.bus.write32(FP_PMV + 4 * i, r.next_u32() % 64);
        }
        for i in 0..4 {
            c.bus.write32(FP_MVFS + 4 * i, r.next_u32() & 1);
        }
        c.g[A0][0] = FP_DEC as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = mbtype as u64;
        c.g[T0][0] = mtype as u64;
        c.g[T1][0] = FP_PMV as u64;
        c.g[T2][0] = FP_MVFS as u64;
        c.g[T3][0] = FP_DMV as u64;
    }

    /// `FormPredictions`: all the motion-compensation legs (frame and field
    /// pictures, forward/backward/both, dual prime, unknown motion type) with
    /// the three callees faked (gated) and unwinding at a random call.
    #[test]
    fn lib_mpeg_form_predictions_matches() {
        fp_hook_callees();
        check(
            ee::s15::f_00404050__generated,
            lib_mpeg_form_predictions,
            960,
            &fp_setup,
        );
        // Many distinct call sequences must really be reached.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..960 {
            let mut c = crate::difftest::fresh();
            fp_setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s15::f_00404050__generated);
            seen.insert(fakes::logged_returns(&mut c));
        }
        assert!(seen.len() >= 20, "only {} call sequences", seen.len());
    }

    /// Gate word, result mailboxes and call counters of this module's fakes.
    pub(crate) const OC1108_CTRL: u32 = DATA + 0xf_e000;

    pub(crate) const OC1108_MAGIC: u32 = 0x4d50_3147;

    /// Header types the `libmpeg_GetHeader` fake returns, in call order.
    pub(crate) const OC1108_HDR: u32 = OC1108_CTRL + 0x100;

    /// Results of the `libmpeg_DecodePicture` fake, in call order.
    pub(crate) const OC1108_DEC: u32 = OC1108_CTRL + 0x200;

    pub(crate) static OC1108_PREV_ERRORF: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1108_PREV_HDR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1108_PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1108_PREV_DEC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1108_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);

    /// Log the call; the header and decode fakes answer from the mailboxes
    /// (the list is exhausted: 0), the decode fake may set `d[520]`.
    pub(crate) fn oc1108_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let (counter, list) = match addr {
            0x0040_7520 => (OC1108_CTRL + 0x20, OC1108_HDR),
            0x0040_3098 => (OC1108_CTRL + 0x24, OC1108_DEC),
            _ => return,
        };
        let n = c.bus.read32(counter);
        c.bus.write32(counter, n + 1);
        let v = if n < 8 { c.bus.read32(list + 4 * n) } else { 0 };
        c.g[V0][0] = sx(v);
        if addr == 0x0040_3098 && n == c.bus.read32(OC1108_CTRL + 0x30) {
            let dec = c.bus.read32(OC1108_CTRL + 0x34);
            c.bus.write32(dec + 0x820, 1);
        }
    }

    macro_rules! oc1108_gated {
        ($name:ident, $addr:expr, $prev:ident, $gen:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1108_CTRL) != OC1108_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc1108_fake(c, $addr)
            }
        };
    }

    oc1108_gated!(
        oc1108_fake_errorf,
        0x0040_3470,
        OC1108_PREV_ERRORF,
        ee::s15::f_00403470__generated
    );

    oc1108_gated!(
        oc1108_fake_hdr,
        0x0040_7520,
        OC1108_PREV_HDR,
        ee::s15::f_00407520__generated
    );

    oc1108_gated!(
        oc1108_fake_flush,
        0x0040_3240,
        OC1108_PREV_FLUSH,
        ee::s15::f_00403240__generated
    );

    oc1108_gated!(
        oc1108_fake_dec,
        0x0040_3098,
        OC1108_PREV_DEC,
        ee::s15::f_00403098__generated
    );

    oc1108_gated!(
        oc1108_fake_error,
        0x0040_34a8,
        OC1108_PREV_ERROR,
        ee::s15::f_004034a8__generated
    );

    pub(crate) fn oc1108_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            oc1108_fake_errorf,
            &OC1108_PREV_ERRORF,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407520),
            oc1108_fake_hdr,
            &OC1108_PREV_HDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403240),
            oc1108_fake_flush,
            &OC1108_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403098),
            oc1108_fake_dec,
            &OC1108_PREV_DEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1108_fake_error,
            &OC1108_PREV_ERROR,
        );
    }

    pub(crate) const OC1108_MPEG: u32 = DATA + 0x2000;

    pub(crate) const OC1108_DECODER: u32 = DATA + 0x3000;

    /// `GetPictureCommon`: the error exit (`d[54] & 0x3f`), header loops
    /// (types 0-5, matching counters, `d[530]` set), decodes returning -1 or 0
    /// (-1 re-dispatches the same type), the exit through `d[520]` set by a
    /// decode, and unwinds in the fakes.
    #[test]
    fn libmpeg_get_picture_common_matches() {
        oc1108_install_fakes();
        check(
            ee::s15::f_00402e10__generated,
            libmpeg_get_picture_common,
            400,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402e10, 5);
                elf_image(c);
                c.bus.write32(OC1108_CTRL, OC1108_MAGIC);
                c.bus.write32(OC1108_MPEG + 0x40, OC1108_DECODER);
                let d = OC1108_DECODER;
                let bad = seed % 11 == 0;
                c.bus
                    .write32(d + 0xd8, if bad { 0x41 } else { 0x40 * (seed % 2) });
                let counter = r.next_u32() % 3;
                c.bus.write32(d + 0xd4, counter);
                c.bus
                    .write32(d + 0x174, if seed % 4 == 0 { counter } else { counter + 1 });
                c.bus.write32(d + 0x848, (seed % 3 == 0) as u32);
                for k in 0..3u32 {
                    c.bus.write32(d + 0x94 + 4 * k, r.next_u32());
                }
                for n in 0..8u32 {
                    let t = if n < 1 + seed % 6 {
                        r.next_u32() % 6
                    } else {
                        0
                    };
                    c.bus.write32(OC1108_HDR + 4 * n, t);
                    let res = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        0
                    };
                    c.bus.write32(OC1108_DEC + 4 * n, res);
                }
                c.bus.write32(
                    OC1108_CTRL + 0x30,
                    if seed % 5 == 0 { r.next_u32() % 3 } else { 99 },
                );
                c.bus.write32(OC1108_CTRL + 0x34, d);
                c.g[A0][0] = sx(OC1108_MPEG);
                c.pc = 0x0040_2e10;
            },
        );
    }

    pub(crate) const OC1108_SRC: u32 = DATA + 0x6000;

    pub(crate) const OC1108_PRED: u32 = DATA + 0x7000;

    pub(crate) const OC1108_DST: u32 = DATA + 0x8000;

    /// `AddPredAndStore`: blocks 0-2, no / some table calls (counts -1, 0, 1-4
    /// through the fake method address), both flag combinations (error report)
    /// and the clamp-only against the add-prediction tails.
    #[test]
    fn libmpeg_add_pred_and_store_matches() {
        oc1108_install_fakes();
        check(
            ee::s15::f_00404b78__generated,
            libmpeg_add_pred_and_store,
            400,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x404b78, 9);
                elf_image(c);
                c.bus.write32(OC1108_CTRL, OC1108_MAGIC);
                let m = OC1108_MPEG;
                let idx = seed % 3;
                let blk = 0x140 * idx;
                for b in 0..3u32 {
                    let o = 0x140 * b;
                    for k in 0..4u32 {
                        c.bus.write32(m + 0x5b8 + o + 4 * k, fakes::FAKE + 4 * k);
                        c.bus
                            .write32(m + 0x5c8 + o + 4 * k, fakes::FAKE + 0x10 + 4 * k);
                    }
                    let count = [0xffff_ffffu32, 0, 1, 3, 4][(r.next_u32() % 5) as usize];
                    c.bus.write32(m + 0x6bc + o, count);
                    c.bus.write32(m + 0x6c8 + o, (seed / 3 % 2) as u32);
                    c.bus.write32(m + 0x6c0 + o, (seed / 6 % 2) as u32);
                    c.bus.write32(m + 0x6cc + o, (r.next_u32() % 2) as u32);
                    c.bus.write32(m + 0x594 + o, OC1108_PRED);
                    c.bus.write32(m + 0x6b8 + o, OC1108_DST);
                }
                c.bus.write32(m + 0x81c, OC1108_SRC);
                for k in 0..0xc0u32 {
                    c.bus.write32(OC1108_SRC + 4 * k, r.next_u32());
                    c.bus.write32(OC1108_PRED + 4 * k, r.next_u32());
                }
                let _ = blk;
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(idx);
                c.pc = 0x0040_4b78;
            },
        );
    }

    /// Gate word of the oc1110 callee fakes (in the machine's RAM): with it
    /// unset every fake runs the body it replaced.
    pub(crate) const OC1110_CTRL: u32 = DATA + 0xf_8100;

    pub(crate) const OC1110_MAGIC: u32 = 0x1110_c0de;

    /// Mailbox words of the fakes (`DATA + 0xf_8110 + 4 * i`).
    pub(crate) const OC1110_MB: u32 = DATA + 0xf_8110;

    /// GetBits results: 32-bit header, 30-bit header, first and second 1-bit flag, 1-bit calls so far.
    pub(crate) const MB_HDR1: u32 = 0;

    pub(crate) const MB_HDR2: u32 = 1;

    pub(crate) const MB_FLAG1: u32 = 2;

    pub(crate) const MB_FLAG2: u32 = 3;

    pub(crate) const MB_BITS1: u32 = 4;

    /// WaitBdec callback fake: calls so far, the call that acts, what it does
    /// (0: clears the IPU-from DMA qwc, else raises ECD), and DIntr's result.
    pub(crate) const MB_CB_COUNT: u32 = 8;

    pub(crate) const MB_CB_AT: u32 = 9;

    pub(crate) const MB_CB_KIND: u32 = 10;

    pub(crate) const MB_DINTR: u32 = 12;

    /// AssignVariations fakes: BuildSentencePart result mask and counter,
    /// DecodeSampleEntry counter, size seed and result mask.
    pub(crate) const MB_BUILD_MASK: u32 = 16;

    pub(crate) const MB_BUILD_N: u32 = 17;

    pub(crate) const MB_DECODE_N: u32 = 18;

    pub(crate) const MB_DECODE_SEED: u32 = 19;

    pub(crate) const MB_DECODE_MASK: u32 = 20;

    pub(crate) fn mb_rd(c: &mut Cpu, i: u32) -> u32 {
        c.bus.read32(OC1110_MB + 4 * i)
    }

    pub(crate) fn mb_wr(c: &mut Cpu, i: u32, v: u32) {
        c.bus.write32(OC1110_MB + 4 * i, v);
    }

    /// Gated fakes that only log the call (all registers, cycles, target).
    macro_rules! oc1110_simple_fakes {
        ($(($fake:ident, $slot:ident, $gen:ident, $addr:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::s15::$gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc1110_hook_simple() {
                $(fakes::hook(addr_of_mut!(ee::s15::$slot), $fake, fakes::prev_cell($fake));)*
            }
        };
    }

    oc1110_simple_fakes!(
        (
            oc1110_fake_error,
            OVERRIDE_004034A8,
            f_004034a8__generated,
            0x0040_34a8
        ),
        (
            oc1110_fake_ipu_wait,
            OVERRIDE_00406E20,
            f_00406e20__generated,
            0x0040_6e20
        ),
        (
            oc1110_fake_ipu_cmd,
            OVERRIDE_00406DF0,
            f_00406df0__generated,
            0x0040_6df0
        ),
        (
            oc1110_fake_load_iq,
            OVERRIDE_00403AD0,
            f_00403ad0__generated,
            0x0040_3ad0
        ),
        (
            oc1110_fake_ext_user,
            OVERRIDE_004076F8,
            f_004076f8__generated,
            0x0040_76f8
        ),
        (
            oc1110_fake_wa_restore,
            OVERRIDE_00402D78,
            f_00402d78__generated,
            0x0040_2d78
        ),
        (
            oc1110_fake_wa_alloc,
            OVERRIDE_00402D88,
            f_00402d88__generated,
            0x0040_2d88
        ),
        (
            oc1110_fake_setup_fb,
            OVERRIDE_004039F0,
            f_004039f0__generated,
            0x0040_39f0
        ),
        (
            oc1110_fake_set_pic_size,
            OVERRIDE_00403600,
            f_00403600__generated,
            0x0040_3600
        ),
        (
            oc1110_fake_errorf,
            OVERRIDE_00403470,
            f_00403470__generated,
            0x0040_3470
        ),
        (
            oc1110_fake_form_pred,
            OVERRIDE_00404050,
            f_00404050__generated,
            0x0040_4050
        ),
        (
            oc1110_fake_fetch_ref,
            OVERRIDE_00408988,
            f_00408988__generated,
            0x0040_8988
        ),
        (
            oc1110_fake_eintr,
            OVERRIDE_0042C0C0,
            f_0042c0c0__generated,
            0x0042_c0c0
        ),
    );

    /// `libmpeg_GetBits(dec, n)`: logs, then returns the mailbox value for `n`
    /// (32: header word, 30: second header word, 1: the two matrix flags).
    pub(crate) fn oc1110_fake_get_bits(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_get_bits),
                ee::s15::f_004072e8__generated,
            );
        }
        let bits = c.g[A1][0] as u32;
        fakes::fake_call(c, 0x0040_72e8);
        let v = match bits {
            0x20 => mb_rd(c, MB_HDR1),
            0x1e => mb_rd(c, MB_HDR2),
            _ => {
                let n = mb_rd(c, MB_BITS1);
                mb_wr(c, MB_BITS1, n + 1);
                mb_rd(c, MB_FLAG1 + (n & 1))
            }
        };
        c.g[V0][0] = v as u64;
    }

    /// `libmpeg_DispatchCallback(ctx, cmd)`: logs, counts the call and, on the
    /// mailbox's call, finishes the IPU wait: clears the from-IPU qwc, or
    /// raises ECD (an invalid VDEC) in `IPU_CTRL`.
    pub(crate) fn oc1110_fake_dispatch(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_dispatch),
                ee::s15::f_00402c30__generated,
            );
        }
        fakes::fake_call(c, 0x0040_2c30);
        let n = mb_rd(c, MB_CB_COUNT) + 1;
        mb_wr(c, MB_CB_COUNT, n);
        if n == mb_rd(c, MB_CB_AT) {
            if mb_rd(c, MB_CB_KIND) == 0 {
                c.bus.write32(0x1000_b020, 0);
            } else {
                c.bus.write32(0x1000_2000, 0x3400_0000);
            }
        }
    }

    /// `DIntr()`: logs, returns the mailbox's "interrupts were enabled" word.
    pub(crate) fn oc1110_fake_dintr(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_dintr),
                ee::s15::f_0042c078__generated,
            );
        }
        fakes::fake_call(c, 0x0042_c078);
        let v = mb_rd(c, MB_DINTR);
        c.g[V0][0] = v as u64;
    }

    /// `SPCH_DecodeSampleEntry(sample, idx, int* size)`: logs, stores a size
    /// (0..39) through a2 and returns a flag from the mailbox's mask.
    pub(crate) fn oc1110_fake_decode_entry(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_decode_entry),
                ee::s14::f_003db838__generated,
            );
        }
        let out = c.g[A2][0] as u32;
        fakes::fake_call(c, 0x003d_b838);
        let n = mb_rd(c, MB_DECODE_N);
        mb_wr(c, MB_DECODE_N, n + 1);
        let seed = mb_rd(c, MB_DECODE_SEED) as u64;
        let size = (fakes::mix(seed + n as u64) % 40) as u32;
        c.bus.write32(out, size);
        let mask = mb_rd(c, MB_DECODE_MASK);
        c.g[V0][0] = ((mask >> (n & 31)) & 1) as u64;
    }

    /// `SPCH_BuildSentencePart(sel, phrase)`: logs, returns a flag from the
    /// mailbox's mask.
    pub(crate) fn oc1110_fake_build_part(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_build_part),
                ee::s14::f_003d8e58__generated,
            );
        }
        fakes::fake_call(c, 0x003d_8e58);
        let n = mb_rd(c, MB_BUILD_N);
        mb_wr(c, MB_BUILD_N, n + 1);
        let mask = mb_rd(c, MB_BUILD_MASK);
        c.g[V0][0] = ((mask >> (n & 31)) & 1) as u64;
    }

    pub(crate) fn oc1110_hook_all() {
        oc1110_hook_simple();
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1110_fake_get_bits,
            fakes::prev_cell(oc1110_fake_get_bits),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1110_fake_dispatch,
            fakes::prev_cell(oc1110_fake_dispatch),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C078),
            oc1110_fake_dintr,
            fakes::prev_cell(oc1110_fake_dintr),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DB838),
            oc1110_fake_decode_entry,
            fakes::prev_cell(oc1110_fake_decode_entry),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D8E58),
            oc1110_fake_build_part,
            fakes::prev_cell(oc1110_fake_build_part),
        );
    }

    /// A machine with the fakes on, the mailbox cleared.
    pub(crate) fn oc1110_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1110_hook_all();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1110_CTRL, OC1110_MAGIC);
        for i in 0..24 {
            mb_wr(c, i, 0);
        }
        r
    }

    pub(crate) const OC1110_DEC: u32 = DATA + 0x1000;

    // ------------------------------------------------------------------
    // libmpeg_SequenceHeader

    /// Sequence header bits and the previous picture size (`dec[0x858][0..8]`),
    /// the decoder context being `dec` itself (`dec[0x858][0x40] = dec`).
    pub(crate) fn seq_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x403620, 30);
        fill(c, &mut r, OC1110_DEC, 0x600);
        let b = DATA + 0x4000;
        fill(c, &mut r, b, 0x20);
        let height = 16 + r.next_u32() % 700;
        let w = if seed % 7 == 0 {
            2801 + r.next_u32() % 100
        } else {
            16 + r.next_u32() % 700
        };
        mb_wr(
            c,
            MB_HDR1,
            height << 20 | w << 8 | (r.next_u32() & 0xff) | 1,
        );
        mb_wr(c, MB_HDR2, r.next_u32() | 1);
        mb_wr(c, MB_FLAG1, seed / 2 % 2);
        mb_wr(c, MB_FLAG2, seed / 4 % 2);
        c.bus.write32(OC1110_DEC + 0x858, b);
        c.bus.write32(b + 0x40, OC1110_DEC);
        let flag848 = seed / 8 % 2;
        c.bus
            .write32(OC1110_DEC + 0x848, flag848 * (1 + r.next_u32() % 5));
        let f13c = if flag848 == 0 { 1 } else { seed / 16 % 2 };
        c.bus.write32(OC1110_DEC + 0x13c, f13c);
        // The size the header will give: S7 = mbw << 4, S6 = mbh << 4.
        let mbw = (height + 15) >> 4;
        let mbh = if f13c != 0 {
            (w + 15) >> 4
        } else {
            ((w + 31) >> 5) << 1
        };
        let (s7, s6) = (mbw << 4, mbh << 4);
        let (w0, w1) = match seed / 32 % 4 {
            0 => (s7, s6),
            1 => (s7, s6 + 16),
            2 => (s7 + 16, s6),
            _ => (r.next_u32() % 900, r.next_u32() % 900),
        };
        c.bus.write32(b, w0);
        c.bus.write32(b + 4, w1);
        c.g[A0][0] = OC1110_DEC as u64;
    }

    /// Both quantiser-matrix sources, the width limit, an unchanged picture
    /// size (early return), a changed one (allocations, frame buffers and the
    /// `SetPictureSize` tail call); fakes unwind at random calls.
    #[test]
    fn lib_mpeg_sequence_header_matches() {
        check_lohi(
            ee::s15::f_00403620__generated,
            lib_mpeg_sequence_header,
            512,
            &seq_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_SetupMacroblock

    pub(crate) fn smb_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x403e00, 3);
        fill(c, &mut r, OC1110_DEC, 0x600);
        c.bus.write32(OC1110_DEC + 0x12c, 1 + r.next_u32() % 30);
        c.bus.write32(OC1110_DEC + 0x810, seed % 3);
        c.bus
            .write32(OC1110_DEC + 0x174, [1u32, 2, 3][(seed / 3 % 3) as usize]);
        for (i, off) in [0x1c0u32, 0x1d0, 0x1e0].into_iter().enumerate() {
            let p = DATA + 0x3000 + 0x40 * i as u32;
            c.bus.write32(OC1110_DEC + off, p);
            c.bus.write32(p, r.next_u32());
            c.bus.write32(p + 0x10, r.next_u32() % 50);
        }
        let flags = [0u32, 1, 2, 3][(seed / 9 % 4) as usize] | (r.next_u32() & 0xc);
        c.g[A0][0] = OC1110_DEC as u64;
        c.g[A1][0] = (r.next_u32() % 3000) as u64;
        c.g[A2][0] = [0u32, 1, 2][(seed / 36 % 3) as usize] as u64;
        c.g[A3][0] = flags as u64;
        c.g[T0][0] = (seed / 108 % 6) as u64;
    }

    /// Intra (no prediction) and inter macroblocks, motion types 0..5 (0, 4
    /// and 5 report the error), the three picture structures, the flag
    /// combinations of the `b == 1 && flags & 2` field.
    #[test]
    fn lib_mpeg_setup_macroblock_matches() {
        check_lohi(
            ee::s15::f_00403e00__generated,
            lib_mpeg_setup_macroblock,
            648,
            &smb_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_SetupPredBlock

    pub(crate) fn spb_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1110_4758 ^ (seed as u64) << 9);
        scramble(c, &mut r);
        fill(c, &mut r, OC1110_DEC, 0x400);
        let cur = seed % 3;
        let k = seed / 3 % 4;
        c.bus.write32(OC1110_DEC + 0x810, cur);
        c.bus.write32(OC1110_DEC + 0x6bc + cur * 0x140, k);
        let rf = DATA + 0x3000;
        c.bus.write32(rf, r.next_u32());
        c.bus.write32(rf + 0x10, 1 + r.next_u32() % 40);
        for i in 0..8 {
            c.bus.write32(0x0045_3280 + 4 * i, r.next_u32());
            c.bus.write32(0x0045_32a0 + 4 * i, r.next_u32());
        }
        let signed = |r: &mut Rng| (r.next_u32() % 81) as i32 - 40;
        c.bus.write32(STACK_TOP, signed(&mut r) as u32);
        c.bus.write32(STACK_TOP + 8, signed(&mut r) as u32);
        c.bus.write32(STACK_TOP + 0x10, seed / 12 % 2);
        c.bus.write32(STACK_TOP + 0x18, seed / 24 % 2);
        c.g[A0][0] = OC1110_DEC as u64;
        c.g[A1][0] = rf as u64;
        c.g[A2][0] = (r.next_u32() % 24) as u64;
        c.g[A3][0] = (r.next_u32() % 2) as u64;
        c.g[T0][0] = (r.next_u32() % 8) as u64;
        c.g[T1][0] = (1 + r.next_u32() % 16) as u64;
        c.g[T2][0] = (r.next_u32() % 100) as u64;
        c.g[T3][0] = (r.next_u32() % 100) as u64;
    }

    /// Luma and chroma block descriptors for every vector parity, frame and
    /// field prediction, both buffer entries and block counts.
    #[test]
    fn lib_mpeg_setup_pred_block_matches() {
        check_lohi(
            ee::s15::f_00404758__generated,
            lib_mpeg_setup_pred_block,
            1200,
            &spb_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_WaitBdec

    /// `seed % 6`: the IPU state the wait starts in (idle, ECD with and
    /// without queued DMA, queued DMA ended by the callback at the 1st or 2nd
    /// call or by an error raised in the callback); then the abort flag
    /// `dec[0x10a0]` and `DIntr`'s result.
    pub(crate) fn wbd_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x405b70, 8);
        fill(c, &mut r, OC1110_DEC, 0x440);
        let mode = seed % 6;
        let abort = seed / 6 % 2;
        c.bus
            .write32(OC1110_DEC + 0x10a0, abort * (1 + r.next_u32() % 9));
        mb_wr(c, MB_DINTR, seed / 12 % 2);
        let queued = mode >= 2;
        c.bus
            .write32(0x1000_b020, if queued { 1 + r.next_u32() % 20 } else { 0 });
        if mode == 1 || mode == 2 {
            c.bus.write32(0x1000_2000, 0x3400_0000);
        }
        match mode {
            3 => {
                mb_wr(c, MB_CB_AT, 1);
                mb_wr(c, MB_CB_KIND, 0);
            }
            4 => {
                mb_wr(c, MB_CB_AT, 2);
                mb_wr(c, MB_CB_KIND, 0);
            }
            5 => {
                mb_wr(c, MB_CB_AT, 1);
                mb_wr(c, MB_CB_KIND, 1);
            }
            _ => {}
        }
        c.g[A0][0] = OC1110_DEC as u64;
    }

    /// The IPU wait loop (ended by the callback clearing the DMA, or by ECD),
    /// the abort path with `DIntr` returning 0 and 1, and the ECD report with
    /// its two callbacks. Not reached: a to-IPU DMA already queued
    /// (`0x1000b420`) or running (`0x1000b400` bit 8) inside the loop (the loop
    /// would never end on the test machine) and IPU_TOP with bit 63 set (the
    /// model's TOP reads zero in the high word).
    #[test]
    fn lib_mpeg_wait_bdec_matches() {
        check(
            ee::s15::f_00405b70__generated,
            lib_mpeg_wait_bdec,
            288,
            &wbd_setup,
        );
    }

    #[test]
    fn libmpeg_dispatch_callback_matches() {
        check(
            ssx3_ee::s15::f_00402c30__generated,
            libmpeg_dispatch_callback,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0040_2c30, 0);
                let mpeg = DATA + 0x1000;
                let sys = DATA + 0x2000;
                let cb = DATA + 0x3000;
                c.bus
                    .write32(mpeg + 0x40, if seed % 6 == 0 { 0 } else { sys });
                c.bus.write32(cb, seed % 4);
                for k in 0..4u32 {
                    let e = sys + 8 * k;
                    let func = if (seed + k) % 5 == 0 {
                        0
                    } else {
                        fakes::FAKE + 0x40 + 0x10 * k
                    };
                    c.bus.write32(e + 0xc, func);
                    c.bus.write32(e + 0x10, r.next_u32());
                }
                c.g[A0][0] = if seed % 7 == 0 { 0 } else { mpeg as u64 };
                c.g[A1][0] = cb as u64;
            },
        );
    }

    /// Gate word, result mailboxes of this module's fakes.
    pub(crate) const OC1116_CTRL: u32 = DATA + 0xf_c000;

    pub(crate) const OC1116_MAGIC: u32 = 0x4d50_3163;

    /// Mailbox word `k` (one per fake, in the `oc1116_fake_*` order below).
    pub(crate) const OC1116_RET: u32 = OC1116_CTRL + 0x100;

    /// The mpeg object (its decoder pointer at +0x40) and the decoder state.
    pub(crate) const OC1116_MPEG: u32 = DATA + 0x2000;

    pub(crate) const OC1116_DEC: u32 = DATA + 0x3000;

    /// A work area and a bump pool `{base, size, cursor}`.
    pub(crate) const OC1116_AREA: u32 = DATA + 0x5000;

    pub(crate) const OC1116_POOL: u32 = DATA + 0x6000;

    pub(crate) static OC1116_PREV_SETUP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1116_PREV_SLICES: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1116_PREV_HEADER: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1116_PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1116_PREV_OUTPUT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1116_PREV_FIELD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1116_PREV_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// Log the call and return the mailbox word `k` (zero-extended into `v0`).
    pub(crate) fn oc1116_fake(c: &mut Cpu, addr: u32, k: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(OC1116_RET + 4 * k));
        }
    }

    macro_rules! oc1116_gated {
        ($name:ident, $addr:expr, $k:expr, $prev:ident, $gen:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1116_CTRL) != OC1116_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc1116_fake(c, $addr, $k)
            }
        };
    }

    oc1116_gated!(
        oc1116_fake_setup,
        0x0040_7f20,
        0,
        OC1116_PREV_SETUP,
        ee::s15::f_00407f20__generated
    );

    oc1116_gated!(
        oc1116_fake_slices,
        0x0040_7dc0,
        1,
        OC1116_PREV_SLICES,
        ee::s15::f_00407dc0__generated
    );

    oc1116_gated!(
        oc1116_fake_header,
        0x0040_7520,
        2,
        OC1116_PREV_HEADER,
        ee::s15::f_00407520__generated
    );

    oc1116_gated!(
        oc1116_fake_flush,
        0x0040_3240,
        3,
        OC1116_PREV_FLUSH,
        ee::s15::f_00403240__generated
    );

    oc1116_gated!(
        oc1116_fake_output,
        0x0040_7e90,
        4,
        OC1116_PREV_OUTPUT,
        ee::s15::f_00407e90__generated
    );

    oc1116_gated!(
        oc1116_fake_field,
        0x0040_30e0,
        5,
        OC1116_PREV_FIELD,
        ee::s15::f_004030e0__generated
    );

    oc1116_gated!(
        oc1116_fake_frame,
        0x0040_2f80,
        6,
        OC1116_PREV_FRAME,
        ee::s15::f_00402f80__generated
    );

    pub(crate) fn oc1116_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407F20),
            oc1116_fake_setup,
            &OC1116_PREV_SETUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407DC0),
            oc1116_fake_slices,
            &OC1116_PREV_SLICES,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407520),
            oc1116_fake_header,
            &OC1116_PREV_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403240),
            oc1116_fake_flush,
            &OC1116_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407E90),
            oc1116_fake_output,
            &OC1116_PREV_OUTPUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004030E0),
            oc1116_fake_field,
            &OC1116_PREV_FIELD,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402F80),
            oc1116_fake_frame,
            &OC1116_PREV_FRAME,
        );
    }

    /// The mailboxes of the fakes, in the `k` order above.
    pub(crate) fn oc1116_rets(c: &mut Cpu, rets: [u32; 7]) {
        for (k, v) in rets.iter().enumerate() {
            c.bus.write32(OC1116_RET + 4 * k as u32, *v);
        }
    }

    /// A decoder state the field and frame paths read: the mpeg object at
    /// `MPEG` with its decoder at `+0x40`. The `DispatchCallback` table word
    /// (`DEC+0x14`) is zero, so the real `CallbackNodata` returns at once.
    pub(crate) fn oc1116_decoder(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(OC1116_CTRL, OC1116_MAGIC);
        c.bus.write32(OC1116_MPEG + 0x40, OC1116_DEC);
        c.bus.write32(OC1116_MPEG + 8, r.next_u32());
        let d = OC1116_DEC;
        c.bus.write32(d, r.next_u32() % 0x1000);
        c.bus.write32(d + 4, r.next_u32() % 0x10);
        c.bus.write32(
            d + 8,
            if seed % 2 == 0 {
                0
            } else {
                1 + r.next_u32() % 3
            },
        );
        c.bus.write32(d + 0x14, 0);
        c.bus.write32(d + 0xac, r.next_u32() % 4);
        c.bus.write32(d + 0x118, r.next_u32() % 5);
        c.bus.write32(d + 0x120, (seed / 3) % 2);
        c.bus.write32(d + 0xd4, seed % 3);
        c.bus.write32(d + 0x174, 1 + (seed / 2) % 2);
    }

    /// A full 64-bit register value from the generator.
    pub(crate) fn oc1116_wide(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    /// `CallbackNodata`: with the decoder's callback table empty the real
    /// `DispatchCallback` returns at once; the null mpeg and the null
    /// decoder take its early exits.
    #[test]
    fn libmpeg_callback_nodata_matches() {
        check(
            ee::s15::f_00402c80__generated,
            libmpeg_callback_nodata,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402c80, 0);
                c.bus.write32(OC1116_DEC + 0x14, 0);
                c.bus.write32(
                    OC1116_MPEG + 0x40,
                    if seed % 3 == 0 { 0 } else { OC1116_DEC },
                );
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { sx(OC1116_MPEG) };
                c.g[A1][0] = oc1116_wide(&mut r);
                c.pc = 0x0040_2c80;
            },
        );
    }

    /// `WorkAreaInit`: the four words of the area, from arbitrary 64-bit
    /// arguments (only the low 32 bits are stored).
    #[test]
    fn libmpeg_work_area_init_matches() {
        check(
            ee::s15::f_00402d50__generated,
            libmpeg_work_area_init,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d50, 0);
                c.g[A0][0] = sx(OC1116_AREA);
                c.g[A1][0] = oc1116_wide(&mut r);
                c.g[A2][0] = oc1116_wide(&mut r);
                for k in 0..4u32 {
                    c.bus.write32(OC1116_AREA + 4 * k, r.next_u32());
                }
                c.pc = 0x0040_2d50;
            },
        );
    }

    /// `WorkAreaMark`: the cursor (+8) is copied to the mark (+0xc).
    #[test]
    fn libmpeg_work_area_mark_matches() {
        check(
            ee::s15::f_00402d68__generated,
            libmpeg_work_area_mark,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d68, 0);
                c.g[A0][0] = sx(OC1116_AREA);
                c.bus.write32(OC1116_AREA + 8, r.next_u32());
                c.bus.write32(OC1116_AREA + 0xc, r.next_u32());
                c.pc = 0x0040_2d68;
            },
        );
    }

    /// `WorkAreaRestore`: the mark (+0xc) is copied back to the cursor (+8).
    #[test]
    fn libmpeg_work_area_restore_matches() {
        check(
            ee::s15::f_00402d78__generated,
            libmpeg_work_area_restore,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d78, 0);
                c.g[A0][0] = sx(OC1116_AREA);
                c.bus.write32(OC1116_AREA + 8, r.next_u32());
                c.bus.write32(OC1116_AREA + 0xc, r.next_u32());
                c.pc = 0x0040_2d78;
            },
        );
    }

    /// `WorkAreaAlloc`: the bump pool `{base 0x1000, size, cursor}` with
    /// power-of-two alignments. Small pools overflow (the `libmpeg_Error`
    /// leg, which takes the fast `DefaultErrorPrint` path because the
    /// decoder's `+0x858` is zero and the printf words are null); big pools
    /// take the success leg and advance the cursor.
    #[test]
    fn libmpeg_work_area_alloc_matches() {
        check(
            ee::s15::f_00402d88__generated,
            libmpeg_work_area_alloc,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402d88, 0);
                c.bus.write32(OC1116_DEC + 0x858, 0);
                c.bus.write32(0x0049_7c68, 0);
                c.bus.write32(0x0049_7ce8, 0);
                let base = 0x1000u32;
                let pool_size: u32 = if seed % 3 == 0 { 0x100 } else { 0x1000 };
                let cursor = base + r.next_u32() % (pool_size + 0x40);
                let align = 1u32 << (seed % 5);
                let size = 1 + r.next_u32() % 0x80;
                c.bus.write32(OC1116_POOL, base);
                c.bus.write32(OC1116_POOL + 4, pool_size);
                c.bus.write32(OC1116_POOL + 8, cursor);
                c.g[A0][0] = sx(OC1116_DEC);
                c.g[A1][0] = sx(OC1116_POOL);
                c.g[A2][0] = size as u64;
                c.g[A3][0] = align as u64;
                c.pc = 0x0040_2d88;
            },
        );
    }

    /// `DecodePicture`: the progressive case (`+0x174 == 3`) and the field
    /// case, each through its gated fake (the callee bodies are not run).
    #[test]
    fn libmpeg_decode_picture_matches() {
        oc1116_install_fakes();
        check(
            ee::s15::f_00403098__generated,
            libmpeg_decode_picture,
            200,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x403098, 0);
                oc1116_decoder(c, &mut r, seed);
                c.bus.write32(
                    OC1116_DEC + 0x174,
                    if seed % 2 == 0 { 3 } else { 1 + (seed / 2) % 2 },
                );
                oc1116_rets(c, [0; 7]);
                c.g[A0][0] = sx(OC1116_MPEG);
                c.pc = 0x0040_3098;
            },
        );
    }

    /// `DecodeFramePic`: the skipped picture (`arg1 >= arg2`, or `arg2 == -1`
    /// with the limit off), the counted and uncounted decoder, the slices
    /// with and without the reference frames, and both tail counters.
    #[test]
    fn libmpeg_decode_frame_pic_matches() {
        oc1116_install_fakes();
        check(
            ee::s15::f_00402f80__generated,
            libmpeg_decode_frame_pic,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402f80, 0);
                oc1116_decoder(c, &mut r, seed);
                oc1116_rets(c, [(seed % 3 != 0) as u32, (seed / 3) % 2, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(OC1116_MPEG);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = if seed % 4 == 0 {
                    u64::MAX
                } else {
                    (seed / 4 % 3) as u64
                };
                c.pc = 0x0040_2f80;
            },
        );
    }

    /// `DecodeFieldPic`: every header result (failed, nonzero, the
    /// `+0xd4 == 1` case), the limit and the reference/slice legs, and the
    /// output tail. `CallbackNodata` runs for real.
    #[test]
    fn libmpeg_decode_field_pic_matches() {
        oc1116_install_fakes();
        check(
            ee::s15::f_004030e0__generated,
            libmpeg_decode_field_pic,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x4030e0, 0);
                oc1116_decoder(c, &mut r, seed);
                oc1116_rets(
                    c,
                    [
                        (seed % 3 != 0) as u32,
                        (seed / 3) % 2,
                        (seed % 5 != 0) as u32,
                        0,
                        0,
                        0,
                        0,
                    ],
                );
                c.g[A0][0] = sx(OC1116_MPEG);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = if seed % 7 == 0 {
                    u64::MAX
                } else {
                    (seed / 7 % 3) as u64
                };
                c.pc = 0x0040_30e0;
            },
        );
    }

    /// Gate word of this module's fakes (the magic at `OC1117_CTRL` in the machine's RAM).
    pub(crate) const OC1117_CTRL: u32 = DATA + 0x0b_4000;

    pub(crate) const OC1117_MAGIC: u32 = 0x1117_6e5a;

    /// The MPEG state and its decoder in the test machine.
    pub(crate) const OC1117_MPEG: u32 = DATA + 0x0b_0000;

    pub(crate) const OC1117_DEC: u32 = DATA + 0x0b_2000;

    pub(crate) static OC1117_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1117_PREV_OUTPUT_FRAME: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1117_PREV_FIELD_PAIR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1117_PREV_DISPATCH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1117_PREV_PRINTF: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1117_PREV_SPRINTF: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake's answer: return at once, with `v0` zero.
    pub(crate) fn oc1117_leaf(c: &mut Cpu) {
        c.g[V0][0] = 0;
        crate::abi::ret(c);
    }

    macro_rules! oc1117_gated {
        ($name:ident, $prev:ident, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1117_CTRL) != OC1117_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc1117_leaf(c)
            }
        };
    }

    oc1117_gated!(
        oc1117_fake_error,
        OC1117_PREV_ERROR,
        ee::s15::f_004034a8__generated
    );

    oc1117_gated!(
        oc1117_fake_output_frame,
        OC1117_PREV_OUTPUT_FRAME,
        ee::s15::f_004086c0__generated
    );

    oc1117_gated!(
        oc1117_fake_field_pair,
        OC1117_PREV_FIELD_PAIR,
        ee::s15::f_004087d0__generated
    );

    oc1117_gated!(
        oc1117_fake_dispatch,
        OC1117_PREV_DISPATCH,
        ee::s15::f_00402c30__generated
    );

    oc1117_gated!(
        oc1117_fake_printf,
        OC1117_PREV_PRINTF,
        ee::s15::f_00425c60__generated
    );

    oc1117_gated!(
        oc1117_fake_sprintf,
        OC1117_PREV_SPRINTF,
        ee::s15::f_00417828__generated
    );

    /// Hooks this module's gated fakes into the override slots (once per process).
    pub(crate) fn oc1117_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1117_fake_error,
            &OC1117_PREV_ERROR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004086C0),
            oc1117_fake_output_frame,
            &OC1117_PREV_OUTPUT_FRAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004087D0),
            oc1117_fake_field_pair,
            &OC1117_PREV_FIELD_PAIR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1117_fake_dispatch,
            &OC1117_PREV_DISPATCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00425C60),
            oc1117_fake_printf,
            &OC1117_PREV_PRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1117_fake_sprintf,
            &OC1117_PREV_SPRINTF,
        );
    }

    /// `ResetMpeg1Flag`: clears the flag at +0x848 and runs `SetIpuMpeg1Bit`.
    #[test]
    fn libmpeg_reset_mpeg1_flag_matches() {
        check(
            ee::s15::f_004032a8__generated,
            libmpeg_reset_mpeg1_flag,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x4032a8, 0);
                c.bus
                    .write32(OC1117_MPEG + 0x848, seed.wrapping_mul(0x9e37_79b9) | 1);
                c.g[A0][0] = sx(OC1117_MPEG);
                c.pc = 0x0040_32a8;
            },
        );
    }

    /// `OutputLastPicture`: the error exit (`dec[72]` set), the frame output
    /// (`dec[93] == 3`) and the field-pair output.
    #[test]
    fn libmpeg_output_last_picture_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_004032b8__generated,
            libmpeg_output_last_picture,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x4032b8, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                let d = OC1117_DEC;
                c.bus
                    .write32(d + 0x120, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(d + 0x118, r.next_u32());
                c.bus.write32(d + 0xac, r.next_u32());
                c.bus
                    .write32(d + 0x174, if seed % 2 == 0 { 3 } else { r.next_u32() % 5 });
                c.bus.write32(d + 0x1bc, r.next_u32());
                c.bus.write32(d + 0x1cc, r.next_u32());
                c.bus.write32(d + 0x1dc, r.next_u32());
                c.g[A0][0] = sx(d);
                c.pc = 0x0040_32b8;
            },
        );
    }

    /// `InitSprBuffers`: the IPU switch call, then the scratchpad buffer words.
    #[test]
    fn libmpeg_init_spr_buffers_matches() {
        check(
            ee::s15::f_00403338__generated,
            libmpeg_init_spr_buffers,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x403338, 0);
                c.g[A0][0] = sx(OC1117_DEC);
                c.pc = 0x0040_3338;
            },
        );
    }

    /// `ResetIpu`: with interrupts off (`cop0` status bit 16 clear) or on
    /// (`DIntr` returns 1, `EIntr` runs), then the IPU reset tail.
    #[test]
    fn libmpeg_reset_ipu_matches() {
        check(
            ee::s15::f_00403398__generated,
            libmpeg_reset_ipu,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x403398, 0);
                c.cop0[12] = if seed % 2 == 0 { 0x1_0000 } else { 0 };
                c.g[A0][0] = sx(OC1117_DEC);
                c.pc = 0x0040_3398;
            },
        );
    }

    /// `DefaultErrorPrint`: a tail call to `scePrintf` with the format and the message.
    #[test]
    fn libmpeg_default_error_print_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_00403460__generated,
            libmpeg_default_error_print,
            50,
            &|c, seed| {
                let _ = machine(c, seed, 0x403460, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                c.pc = 0x0040_3460;
            },
        );
    }

    /// `Errorf`: the `sprintf` into the stack buffer, then `libmpeg_Error`
    /// with the decoder and the buffer.
    #[test]
    fn libmpeg_errorf_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_00403470__generated,
            libmpeg_errorf,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x403470, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                c.g[A0][0] = sx(OC1117_DEC);
                c.g[A1][0] = sx(OC1117_MPEG + 0x100);
                c.pc = 0x0040_3470;
            },
        );
    }

    /// `Error`: no decoder, no callback table, a callback (`DispatchCallback`)
    /// and the default print.
    #[test]
    fn libmpeg_error_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_004034a8__generated,
            libmpeg_error,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x4034a8, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                let st = OC1117_MPEG;
                c.g[A0][0] = if seed % 9 == 0 { 0 } else { sx(st) };
                c.bus
                    .write32(st + 0x858, if seed % 4 == 0 { 0 } else { OC1117_DEC });
                c.bus.write32(st + 0xc, if seed % 5 == 0 { 0 } else { 1 });
                c.bus
                    .write32(OC1117_DEC + 0xc, if seed % 3 == 0 { 0 } else { 1 });
                c.g[A1][0] = sx(OC1117_MPEG + 0x200);
                c.pc = 0x0040_34a8;
            },
        );
    }

    /// Gate word of the test-only `libmpeg_GetBits` fake: the fake answers only
    /// in a machine that wrote this magic here.
    pub(crate) const OC1118_GATE: u32 = DATA + 0x000f_1118;

    pub(crate) const OC1118_MAGIC: u32 = 0x1118_6e75;

    /// Number of fake calls so far in this machine.
    pub(crate) const OC1118_CALLS: u32 = DATA + 0x000f_1120;

    /// The queue of words the fake returns, one per call (16 words).
    pub(crate) const OC1118_WORDS: u32 = DATA + 0x000f_1180;

    /// Previous override-slot body, run when the gate is off.
    pub(crate) static OC1118_PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_GetBits(dec, nbits)` fake: returns the next queued word masked
    /// to `nbits`; with the gate off it runs the real body.
    pub(crate) fn oc1118_get_bits_fake(c: &mut Cpu) {
        if c.bus.read32(OC1118_GATE) != OC1118_MAGIC {
            return fakes::fall_back(c, &OC1118_PREV_GET_BITS, ee::s15::f_004072e8__generated);
        }
        let k = c.bus.read32(OC1118_CALLS);
        c.bus.write32(OC1118_CALLS, k.wrapping_add(1));
        let bits = c.g[A1][0] as u32;
        let word = c.bus.read32(OC1118_WORDS + 4 * (k & 15));
        let mask = if bits >= 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };
        c.g[V0][0] = sx(word & mask);
        ret(c);
    }

    pub(crate) fn oc1118_hook_get_bits() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1118_get_bits_fake,
            &OC1118_PREV_GET_BITS,
        );
    }

    /// Arm the GetBits fake in this machine with a queue of words.
    pub(crate) fn oc1118_get_bits_queue(c: &mut Cpu, words: &[u32]) {
        c.bus.write32(OC1118_GATE, OC1118_MAGIC);
        c.bus.write32(OC1118_CALLS, 0);
        for (i, w) in words.iter().enumerate() {
            c.bus.write32(OC1118_WORDS + 4 * i as u32, *w);
        }
    }

    /// The MPEG error path prints nothing: its format words are NULled (as in
    /// `game/lib/mpeg.rs`).
    pub(crate) fn oc1118_quiet_printf(c: &mut Cpu) {
        c.bus.write32(0x0049_7c68, 0);
        c.bus.write32(0x0049_7ce8, 0);
    }

    #[test]
    fn libmpeg_set_picture_size_matches() {
        check(
            ee::s15::f_00403600__generated,
            libmpeg_set_picture_size,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3600 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_set_ipu_mpeg1_bit_matches() {
        check(
            ee::s15::f_00405b48__generated,
            libmpeg_set_ipu_mpeg1_bit,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_5b48 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.bus.write32(0x1000_2010, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_restart_dma_callback_matches() {
        check(
            ee::s15::f_00403dd8__generated,
            libmpeg_restart_dma_callback,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3dd8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x40, DATA + 0x200);
                // sceIpuRestartDMA's count word (+0x10 of the block at +0x4c) is 0:
                // its short path.
                c.bus.write32(DATA + 0x200 + 0x4c + 0x10, 0);
                c.bus.write32(DATA + 0x200 + 0x4c + 0x1c, r.next_u32());
                c.bus.write32(DATA + 0x200 + 0x4c, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_add_bs_dma_chain_matches() {
        check(
            ee::s15::f_00403500__generated,
            libmpeg_add_bs_dma_chain,
            200,
            &|c, seed| {
                // The machine's syscall hook answers FlushCache.
                let _ = fakes::machine(c, seed, 0x0040_3500, 0);
                let mut r = Rng(0x0040_3500 ^ (seed as u64) << 16);
                c.g[A0][0] = (DATA + 0x1000 * (seed % 8)) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                let n = match seed % 3 {
                    0 => 0,
                    1 => 0x0010_0000 + r.next_u32() % 0x1000,
                    _ => r.next_u32() % 0x0004_0000 + 1,
                };
                c.g[A2][0] = sx(n);
                c.cop0[12] = if seed % 2 == 0 { 0x0001_0000 } else { 0 };
            },
        );
    }

    #[test]
    fn libmpeg_sequence_extension_matches() {
        oc1118_hook_get_bits();
        check(
            ee::s15::f_00403bb0__generated,
            libmpeg_sequence_extension,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3bb0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1118_quiet_printf(c);
                // First word (28 bits): bits 17-18 == 1 take the no-error leg
                // (seed % 5 != 0); bits 20-27 select the case.
                let code = [0x48u32, 0x58, 0x44, 0x10][(seed as usize / 5) % 4];
                let mut w0 = r.next_u32();
                w0 &= !(3u32 << 17);
                if seed % 5 != 0 {
                    w0 |= 1u32 << 17;
                }
                w0 &= !(0xffu32 << 20);
                w0 |= code << 20;
                let w1 = r.next_u32();
                oc1118_get_bits_queue(c, &[w0, w1]);
            },
        );
    }

    #[test]
    fn libmpeg_sequence_display_extension_matches() {
        oc1118_hook_get_bits();
        check(
            ee::s15::f_00403ce0__generated,
            libmpeg_sequence_display_extension,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3ce0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let words: [u32; 16] = std::array::from_fn(|_| r.next_u32());
                oc1118_get_bits_queue(c, &words);
            },
        );
    }

    /// The dec struct of the dual-prime test, and its out and in words.
    pub(crate) const DP_DEC: u32 = DATA + 0x1000;

    pub(crate) const DP_OUT: u32 = DATA + 0x1100;

    pub(crate) const DP_IN: u32 = DATA + 0x1200;

    /// A motion vector operand: zero, a small negative or positive value, a
    /// random 32-bit word, or a random 64-bit value (its sign may sit in the
    /// high half only).
    pub(crate) fn dp_operand(r: &mut Rng, kind: u32) -> u64 {
        match kind {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 300) as i64)) as u64,
            2 => (1 + r.next_u32() % 300) as u64,
            3 => sx(r.next_u32()),
            _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
        }
    }

    /// The fixture of the dual-prime test: the picture structure (frame or
    /// field), the top-field-first flag, the operand kinds for a and b. The
    /// seed walks all 300 combinations (mixed radix 4 x 3 x 5 x 5).
    pub(crate) fn dp_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x405e30, 0);
        let structure = [3u32, 3, 1, 2][(seed % 4) as usize];
        let top_first = [0u32, 1, 2][((seed / 4) % 3) as usize];
        let a_kind = (seed / 12) % 5;
        let b_kind = (seed / 60) % 5;
        c.bus.write32(DP_DEC + 0x174, structure);
        c.bus.write32(DP_DEC + 0x178, top_first);
        fill(c, &mut r, DP_IN, 2);
        fill(c, &mut r, DP_OUT, 4);
        c.g[A0][0] = sx(DP_DEC);
        c.g[A1][0] = sx(DP_OUT);
        c.g[A2][0] = sx(DP_IN);
        c.g[A3][0] = dp_operand(&mut r, a_kind);
        c.g[T0][0] = dp_operand(&mut r, b_kind);
    }

    /// `libmpeg_DualPrimeArithmetic` against the generated body: frame
    /// pictures with either field first, field pictures, both signs and zero
    /// for both vectors.
    #[test]
    fn lib_mpeg_dual_prime_arithmetic_matches() {
        check(
            ee::s15::f_00405e30__generated,
            lib_mpeg_dual_prime_arithmetic,
            300,
            &dp_setup,
        );
    }

    /// Gate word of the DecodeDmVector test's VDEC fake (in the machine's RAM).
    pub(crate) const DM_CTRL: u32 = DATA + 0x3_0400;

    pub(crate) const DM_MAGIC: u32 = 0x1119_d0c5;

    pub(crate) static DM_PREV_VDEC: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_IpuVdec`, gated: logs the call and returns a hashed result.
    pub(crate) fn dm_fake_vdec(c: &mut Cpu) {
        if c.bus.read32(DM_CTRL) != DM_MAGIC {
            return fakes::fall_back(c, &DM_PREV_VDEC, ee::s15::f_00406f78__generated);
        }
        fakes::fake_call(c, 0x0040_6f78)
    }

    pub(crate) fn dm_setup(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x405e10, 4);
        c.bus.write32(DM_CTRL, DM_MAGIC);
    }

    /// `libmpeg_DecodeDmVector` against the generated body: the IPU call with
    /// table 3, the return address and the callee's logged registers, with
    /// the VDEC fake unwinding at a random call.
    #[test]
    fn lib_mpeg_decode_dm_vector_matches() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406F78),
            dm_fake_vdec,
            &DM_PREV_VDEC,
        );
        check(
            ee::s15::f_00405e10__generated,
            lib_mpeg_decode_dm_vector,
            200,
            &dm_setup,
        );
    }

    /// Gate word of the MacroblockAddressIncrement test's fakes.
    pub(crate) const MBI_CTRL: u32 = DATA + 0x3_0000;

    pub(crate) const MBI_MAGIC: u32 = 0x1119_0b1a;

    /// Cursors into the code list and the peek list (each fake call takes
    /// the next entry; past the end the code is 40 and the peek is 15).
    pub(crate) const MBI_CUR: u32 = DATA + 0x3_0004;

    pub(crate) const MBI_PEEK_CUR: u32 = DATA + 0x3_0008;

    pub(crate) const MBI_CODES: u32 = DATA + 0x3_0100;

    pub(crate) const MBI_PEEKS: u32 = DATA + 0x3_0200;

    /// The decoder state of the fixture.
    pub(crate) const MBI_DEC: u32 = DATA + 0x1000;

    /// Codes the fixture draws: stuffing 34, escape 35, 0 (error or peek),
    /// the default range, and 36 (beyond the escape).
    pub(crate) const MBI_VLC: [u32; 9] = [34, 35, 0, 1, 7, 33, 36, 40, 35];

    /// The 11-bit peeks the fixture draws (15 is the stuffing code).
    pub(crate) const MBI_PEEK_BITS: [u32; 6] = [15, 15, 0, 3, 14, 16];

    pub(crate) static MBI_PREV_VDEC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static MBI_PREV_SHOW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static MBI_PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static MBI_PREV_ERRORF: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_IpuVdec`, gated: logs the call and returns the next code.
    pub(crate) fn mbi_fake_vdec(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_VDEC, ee::s15::f_00406f78__generated);
        }
        fakes::fake_call(c, 0x0040_6f78);
        let i = c.bus.read32(MBI_CUR);
        c.bus.write32(MBI_CUR, i + 1);
        c.g[V0][0] = if i < 16 {
            c.bus.read32(MBI_CODES + 4 * i) as u64
        } else {
            40
        };
    }

    /// `libmpeg_ShowBits`, gated: logs the call and returns the next peek.
    pub(crate) fn mbi_fake_show(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_SHOW, ee::s15::f_004070f0__generated);
        }
        fakes::fake_call(c, 0x0040_70f0);
        let i = c.bus.read32(MBI_PEEK_CUR);
        c.bus.write32(MBI_PEEK_CUR, i + 1);
        c.g[V0][0] = if i < 16 {
            c.bus.read32(MBI_PEEKS + 4 * i) as u64
        } else {
            15
        };
    }

    /// `libmpeg_FlushBits`, gated: logs the call and returns.
    pub(crate) fn mbi_fake_flush(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_FLUSH, ee::s15::f_004071f8__generated);
        }
        fakes::fake_call(c, 0x0040_71f8)
    }

    /// `libmpeg_Errorf`, gated: logs the call and returns.
    pub(crate) fn mbi_fake_errorf(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_ERRORF, ee::s15::f_00403470__generated);
        }
        fakes::fake_call(c, 0x0040_3470)
    }

    pub(crate) fn mbi_hook_callees() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406F78),
            mbi_fake_vdec,
            &MBI_PREV_VDEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004070F0),
            mbi_fake_show,
            &MBI_PREV_SHOW,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            mbi_fake_flush,
            &MBI_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            mbi_fake_errorf,
            &MBI_PREV_ERRORF,
        );
    }

    /// The fixture: sixteen random codes and peeks, the decoder state with
    /// the stuffing flag at +0x848 set or clear, and random words around it.
    pub(crate) fn mbi_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x405fb8, 10);
        c.bus.write32(MBI_CTRL, MBI_MAGIC);
        c.bus.write32(MBI_CUR, 0);
        c.bus.write32(MBI_PEEK_CUR, 0);
        for i in 0..16 {
            let code = MBI_VLC[(r.next_u32() % MBI_VLC.len() as u32) as usize];
            c.bus.write32(MBI_CODES + 4 * i, code);
            let peek = MBI_PEEK_BITS[(r.next_u32() % MBI_PEEK_BITS.len() as u32) as usize];
            c.bus.write32(MBI_PEEKS + 4 * i, peek);
        }
        fill(c, &mut r, MBI_DEC, 0x240);
        c.bus
            .write32(MBI_DEC + 0x848, if seed % 3 == 0 { 0 } else { 1 });
        c.g[A0][0] = sx(MBI_DEC);
    }

    /// `libmpeg_MacroblockAddressIncrement` against the generated body: all
    /// the codes (stuffing, escape, error, peek branches, default), the
    /// stuffing flag, and the fakes unwinding at random calls.
    #[test]
    fn lib_mpeg_macroblock_address_increment_matches() {
        mbi_hook_callees();
        check(
            ee::s15::f_00405fb8__generated,
            lib_mpeg_macroblock_address_increment,
            300,
            &mbi_setup,
        );
    }

    pub(crate) use crate::game::sound::bankmonitor::tests::oc1137_setup;

    /// Gate word of this module's callee fakes (in the machine's RAM): with it
    /// unset every fake runs the body it replaced.
    pub(crate) const OC1120_CTRL: u32 = DATA + 0xf_9000;

    pub(crate) const OC1120_MAGIC: u32 = 0x1120_c0de;

    /// Mailbox of fake `k`: call count, 8 results, 8 side stores, two outputs.
    pub(crate) const OC1120_MB: u32 = OC1120_CTRL + 0x100;

    pub(crate) const OC1120_MB_SIZE: u32 = 0x80;

    /// The decoder state (`dec`) and the picture record of the fixtures.
    pub(crate) const OC1120_DEC: u32 = DATA + 0x2000;

    pub(crate) const OC1120_REC: u32 = DATA + 0x6000;

    /// Source and destination pictures of the DMA fixtures.
    pub(crate) const OC1120_SRC: u32 = DATA + 0x1_0000;

    pub(crate) const OC1120_DST: u32 = DATA + 0x2_0000;

    /// Output areas of `DecodeMacroblock` (mbtype, motion type, dct type, pmv,
    /// field select, dm vector).
    pub(crate) const OC1120_OUT: u32 = DATA + 0x8000;

    /// `dec[0x11c]`: the error flag the callees may set.
    pub(crate) const OC1120_ERR: u32 = 0x11c;

    /// The index of each fake's mailbox.
    pub(crate) const K_SLICE_HEADER: u32 = 0;

    pub(crate) const K_WAIT_BDEC: u32 = 1;

    pub(crate) const K_SHOW_BITS: u32 = 2;

    pub(crate) const K_MB_INCREMENT: u32 = 3;

    pub(crate) const K_ERRORF: u32 = 4;

    pub(crate) const K_DECODE_MB: u32 = 5;

    pub(crate) const K_SKIPPED_MB: u32 = 6;

    pub(crate) const K_SETUP_MB: u32 = 7;

    pub(crate) const K_ADD_PRED: u32 = 8;

    pub(crate) const K_GET_BITS: u32 = 9;

    pub(crate) const K_MV1: u32 = 10;

    pub(crate) const K_MV2: u32 = 11;

    pub(crate) const K_CLEAR: u32 = 12;

    pub(crate) const K_71F8: u32 = 13;

    pub(crate) const K_MB_TYPE: u32 = 14;

    pub(crate) const K_BLOCK_WAIT: u32 = 15;

    pub(crate) const K_IPU_COMMAND: u32 = 16;

    pub(crate) const K_DINTR: u32 = 17;

    pub(crate) const K_EINTR: u32 = 18;

    pub(crate) const K_CSC_DMA: u32 = 19;

    pub(crate) const K_CSC: u32 = 20;

    pub(crate) const K_DISPATCH: u32 = 21;

    pub(crate) const K_ENABLE_DMAC: u32 = 22;

    pub(crate) const K_DISABLE_DMAC: u32 = 23;

    pub(crate) const K_REMOVE_HANDLER: u32 = 24;

    pub(crate) const K_ADD_HANDLER: u32 = 25;

    /// A callee fake: logs the call (every register, cycles, target), then
    /// takes its result from the mailbox's next entry (zero-extended like the
    /// plain fakes), optionally stores the entry's side word minus one to
    /// `a0[0x11c]` (0: no store) and, for `SliceHeader`, the two output words
    /// through `a2` and `a3`.
    pub(crate) fn oc1120_fake(c: &mut Cpu, addr: u32, k: u32, side: bool, outs: bool) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let a3 = c.g[A3][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let mb = OC1120_MB + OC1120_MB_SIZE * k;
        let n = c.bus.read32(mb);
        c.bus.write32(mb, n + 1);
        let i = 4 * (n % 8);
        let result = c.bus.read32(mb + 4 + i);
        c.g[V0][0] = result as u64;
        if side {
            let s = c.bus.read32(mb + 0x24 + i);
            if s != 0 {
                c.bus.write32(a0 + OC1120_ERR, s - 1);
            }
        }
        if outs {
            let (o0, o1) = (c.bus.read32(mb + 0x44), c.bus.read32(mb + 0x48));
            c.bus.write32(a2, o0);
            c.bus.write32(a3, o1);
        }
    }

    macro_rules! oc1120_fakes {
        ($(($fake:ident, $addr:expr, $slot:ident, $gen:ident, $k:expr, $side:expr, $outs:expr)),* $(,)?) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1120_CTRL) != OC1120_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::s15::$gen);
                    }
                    oc1120_fake(c, $addr, $k, $side, $outs)
                }
            )*

            /// Put every fake in its override slot (each once per process).
            pub(crate) fn oc1120_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::s15::$slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc1120_fakes!(
        (
            oc1120_fake_slice_header,
            0x004061e8,
            OVERRIDE_004061E8,
            f_004061e8__generated,
            K_SLICE_HEADER,
            false,
            true
        ),
        (
            oc1120_fake_wait_bdec,
            0x00405b70,
            OVERRIDE_00405B70,
            f_00405b70__generated,
            K_WAIT_BDEC,
            false,
            false
        ),
        (
            oc1120_fake_show_bits,
            0x004070f0,
            OVERRIDE_004070F0,
            f_004070f0__generated,
            K_SHOW_BITS,
            true,
            false
        ),
        (
            oc1120_fake_mb_increment,
            0x00405fb8,
            OVERRIDE_00405FB8,
            f_00405fb8__generated,
            K_MB_INCREMENT,
            true,
            false
        ),
        (
            oc1120_fake_errorf,
            0x004034a8,
            OVERRIDE_004034A8,
            f_004034a8__generated,
            K_ERRORF,
            false,
            false
        ),
        (
            oc1120_fake_decode_mb,
            0x004065d0,
            OVERRIDE_004065D0,
            f_004065d0__generated,
            K_DECODE_MB,
            false,
            false
        ),
        (
            oc1120_fake_skipped_mb,
            0x00406510,
            OVERRIDE_00406510,
            f_00406510__generated,
            K_SKIPPED_MB,
            false,
            false
        ),
        (
            oc1120_fake_setup_mb,
            0x00403e00,
            OVERRIDE_00403E00,
            f_00403e00__generated,
            K_SETUP_MB,
            false,
            false
        ),
        (
            oc1120_fake_add_pred,
            0x00404b78,
            OVERRIDE_00404B78,
            f_00404b78__generated,
            K_ADD_PRED,
            false,
            false
        ),
        (
            oc1120_fake_get_bits,
            0x004072e8,
            OVERRIDE_004072E8,
            f_004072e8__generated,
            K_GET_BITS,
            false,
            false
        ),
        (
            oc1120_fake_mv1,
            0x00406ca0,
            OVERRIDE_00406CA0,
            f_00406ca0__generated,
            K_MV1,
            true,
            false
        ),
        (
            oc1120_fake_mv2,
            0x00406b00,
            OVERRIDE_00406B00,
            f_00406b00__generated,
            K_MV2,
            true,
            false
        ),
        (
            oc1120_fake_clear,
            0x00408af8,
            OVERRIDE_00408AF8,
            f_00408af8__generated,
            K_CLEAR,
            false,
            false
        ),
        (
            oc1120_fake_71f8,
            0x004071f8,
            OVERRIDE_004071F8,
            f_004071f8__generated,
            K_71F8,
            false,
            false
        ),
        (
            oc1120_fake_mb_type,
            0x00406f78,
            OVERRIDE_00406F78,
            f_00406f78__generated,
            K_MB_TYPE,
            false,
            false
        ),
        (
            oc1120_fake_block_wait,
            0x00406e20,
            OVERRIDE_00406E20,
            f_00406e20__generated,
            K_BLOCK_WAIT,
            true,
            false
        ),
        (
            oc1120_fake_ipu_command,
            0x00406df0,
            OVERRIDE_00406DF0,
            f_00406df0__generated,
            K_IPU_COMMAND,
            false,
            false
        ),
        (
            oc1120_fake_dintr,
            0x0042c078,
            OVERRIDE_0042C078,
            f_0042c078__generated,
            K_DINTR,
            false,
            false
        ),
        (
            oc1120_fake_eintr,
            0x0042c0c0,
            OVERRIDE_0042C0C0,
            f_0042c0c0__generated,
            K_EINTR,
            false,
            false
        ),
        (
            oc1120_fake_csc_dma,
            0x00408df0,
            OVERRIDE_00408DF0,
            f_00408df0__generated,
            K_CSC_DMA,
            false,
            false
        ),
        (
            oc1120_fake_csc,
            0x00408b80,
            OVERRIDE_00408B80,
            f_00408b80__generated,
            K_CSC,
            false,
            false
        ),
        (
            oc1120_fake_dispatch,
            0x00402c30,
            OVERRIDE_00402C30,
            f_00402c30__generated,
            K_DISPATCH,
            false,
            false
        ),
        (
            oc1120_fake_enable_dmac,
            0x004249b8,
            OVERRIDE_004249B8,
            f_004249b8__generated,
            K_ENABLE_DMAC,
            false,
            false
        ),
        (
            oc1120_fake_disable_dmac,
            0x00424950,
            OVERRIDE_00424950,
            f_00424950__generated,
            K_DISABLE_DMAC,
            false,
            false
        ),
        (
            oc1120_fake_remove_handler,
            0x00423ad0,
            OVERRIDE_00423AD0,
            f_00423ad0__generated,
            K_REMOVE_HANDLER,
            false,
            false
        ),
        (
            oc1120_fake_add_handler,
            0x00423ac0,
            OVERRIDE_00423AC0,
            f_00423ac0__generated,
            K_ADD_HANDLER,
            false,
            false
        ),
    );

    /// A machine with the fakes on and every mailbox cleared.
    pub(crate) fn oc1120_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1120_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1120_CTRL, OC1120_MAGIC);
        for k in 0..26 {
            for w in 0..0x20 {
                c.bus.write32(OC1120_MB + OC1120_MB_SIZE * k + 4 * w, 0);
            }
        }
        r
    }

    /// Fill fake `k`'s eight (result, side) entries from `f(i)`.
    pub(crate) fn oc1120_mb(c: &mut Cpu, k: u32, mut f: impl FnMut(u32) -> (u32, u32)) {
        let mb = OC1120_MB + OC1120_MB_SIZE * k;
        for i in 0..8 {
            let (result, side) = f(i);
            c.bus.write32(mb + 4 + 4 * i, result);
            c.bus.write32(mb + 0x24 + 4 * i, side);
        }
    }

    /// The two output words `SliceHeader` stores (mbAddress, mbIncrement).
    pub(crate) fn oc1120_mb_outs(c: &mut Cpu, k: u32, o0: u32, o1: u32) {
        let mb = OC1120_MB + OC1120_MB_SIZE * k;
        c.bus.write32(mb + 0x44, o0);
        c.bus.write32(mb + 0x48, o1);
    }

    /// `true` for about `num` in `den` draws.
    pub(crate) fn oc1120_chance(r: &mut Rng, num: u32, den: u32) -> bool {
        r.next_u32() % den < num
    }

    /// How many different cycle totals the generated body reaches over the
    /// seeds (each total is a different path through the function).
    pub(crate) fn oc1120_paths(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    // ------------------------------------------------------------------
    // libmpeg_DecodeSlice

    /// `SliceHeader` returns 0 (rarely an error) and stores the first
    /// macroblock address (0..n+1) and increment (0..3); then each macroblock
    /// draws its callee results (success 4 in 5), `ShowBits` and
    /// `GetMbIncrement` may flag an error in `dec[0x11c]`.
    pub(crate) fn oc1120_slice_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x406318, 14);
        fill(c, &mut r, OC1120_DEC, 0x440);
        let n = 1 + seed % 5;
        c.bus.write32(OC1120_DEC + 0x810, seed / 5 % 2);
        let abort = if seed / 10 % 3 == 0 {
            1 + r.next_u32() % 9
        } else {
            0
        };
        c.bus.write32(OC1120_DEC + 0x10a0, abort);
        oc1120_mb(c, K_SLICE_HEADER, |_| {
            (
                if oc1120_chance(&mut r, 1, 6) {
                    1 + r.next_u32() % 4
                } else {
                    0
                },
                0,
            )
        });
        let (a, inc) = (r.next_u32() % (n + 2), r.next_u32() % 4);
        oc1120_mb_outs(c, K_SLICE_HEADER, a, inc);
        oc1120_mb(c, K_WAIT_BDEC, |_| (oc1120_chance(&mut r, 4, 5) as u32, 0));
        oc1120_mb(c, K_SHOW_BITS, |_| {
            let side = [0, 0, 0, 1, 2][(r.next_u32() % 5) as usize];
            (oc1120_chance(&mut r, 4, 5) as u32, side)
        });
        oc1120_mb(c, K_MB_INCREMENT, |_| {
            let side = [0, 0, 0, 1, 2][(r.next_u32() % 5) as usize];
            ([0, 1, 1, 2, 3][(r.next_u32() % 5) as usize], side)
        });
        oc1120_mb(c, K_DECODE_MB, |_| (oc1120_chance(&mut r, 5, 6) as u32, 0));
        oc1120_mb(c, K_SKIPPED_MB, |_| (oc1120_chance(&mut r, 5, 6) as u32, 0));
        oc1120_mb(c, K_SETUP_MB, |_| (oc1120_chance(&mut r, 5, 6) as u32, 0));
        c.g[A0][0] = OC1120_DEC as u64;
        c.g[A1][0] = n as u64;
        c.pc = 0x0040_6318;
    }

    /// Slice-header error, the loop ending by count, a failed `WaitBdec`
    /// (with and without the abort flag), `ShowBits` zero or flagged, the
    /// increment read (and its flag), a macroblock past the end, coded and
    /// skipped macroblocks failing or not, setup failing, and the
    /// `AddPredAndStore` call from the second macroblock on.
    #[test]
    fn libmpeg_decode_slice_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_00406318__generated,
            libmpeg_decode_slice,
            600,
            &oc1120_slice_setup,
        );
        let paths = oc1120_paths(ee::s15::f_00406318__generated, 600, &oc1120_slice_setup);
        assert!(paths >= 10, "only {paths} paths");
    }

    // ------------------------------------------------------------------
    // libmpeg_DecodeMacroblock

    pub(crate) fn oc1120_mb_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x4065d0, 14);
        fill(c, &mut r, OC1120_DEC, 0x440);
        fill(c, &mut r, OC1120_OUT, 0x100);
        let d = OC1120_DEC;
        let ptype = 1 + seed % 3;
        c.bus.write32(d + 0x150, ptype);
        c.bus.write32(d + 0x174, 1 + seed / 3 % 3);
        c.bus.write32(d + 0x17c, seed / 9 % 2);
        c.bus.write32(d + 0x180, seed / 18 % 2);
        c.bus.write32(d + 0x848, seed / 36 % 2);
        c.bus.write32(d + 0x1b0, seed / 72 % 2);
        c.bus.write32(d + 0x810, seed / 144 % 2);
        c.bus.write32(d + OC1120_ERR, 0);
        for off in [0x154u32, 0x158, 0x15c, 0x160, 0x164, 0x168, 0x16c, 0x170] {
            c.bus.write32(d + off, 1 + r.next_u32() % 9);
        }
        oc1120_mb(c, K_MB_TYPE, |_| {
            let t = if oc1120_chance(&mut r, 1, 8) {
                0
            } else {
                r.next_u32() % 0x20
            };
            (t, 0)
        });
        oc1120_mb(c, K_GET_BITS, |i| {
            (
                [r.next_u32() % 4, r.next_u32() % 2, r.next_u32() % 32][(i % 3) as usize],
                0,
            )
        });
        for k in [K_MV1, K_MV2, K_BLOCK_WAIT] {
            oc1120_mb(c, k, |_| {
                (
                    r.next_u32(),
                    if oc1120_chance(&mut r, 1, 6) { 2 } else { 0 },
                )
            });
        }
        c.g[A0][0] = d as u64;
        c.g[A1][0] = (OC1120_OUT) as u64;
        c.g[A2][0] = (OC1120_OUT + 0x10) as u64;
        c.g[A3][0] = (OC1120_OUT + 0x20) as u64;
        c.g[T0][0] = (OC1120_OUT + 0x40) as u64;
        c.g[T1][0] = (OC1120_OUT + 0x80) as u64;
        c.g[T2][0] = (OC1120_OUT + 0xa0) as u64;
    }

    /// Every macroblock type (0 reports the error), the three picture
    /// structures, both MV readers, the motion-type / DCT / quantiser reads,
    /// intra and inter blocks, the flags set by the readers' side stores, and
    /// the predictor resets (the `ptype == 2` and non-intra-no-motion cases).
    #[test]
    fn libmpeg_decode_macroblock_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_004065d0__generated,
            libmpeg_decode_macroblock,
            1200,
            &oc1120_mb_setup,
        );
        let paths = oc1120_paths(ee::s15::f_004065d0__generated, 1200, &oc1120_mb_setup);
        assert!(paths >= 12, "only {paths} paths");
    }

    // ------------------------------------------------------------------
    // libmpeg_CopyOutputRaw

    pub(crate) fn oc1120_copy_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x408278, 0);
        fill(c, &mut r, OC1120_SRC, 0x800);
        fill(c, &mut r, OC1120_DST, 0x800);
        let d = OC1120_DEC;
        c.bus.write32(d + 0xd8, OC1120_DST + 16 * (seed % 4));
        c.bus.write32(d + 0x174, 1 + seed / 4 % 3);
        c.bus
            .write32(d + 0xe0, [0u32, 16, 32, 48][(seed / 12 % 4) as usize]);
        c.bus.write32(d + 0xe4, seed / 48 % 4);
        let rec = OC1120_REC;
        c.bus.write32(rec, OC1120_SRC + 16 * (seed % 3));
        c.bus.write32(rec + 0xc, seed / 3 % 4);
        c.bus
            .write32(rec + 0x10, [1u32, 2, 3, 4][(seed / 5 % 4) as usize]);
        oc1120_mb(c, K_DINTR, |_| (oc1120_chance(&mut r, 1, 2) as u32, 0));
        c.g[A0][0] = d as u64;
        c.g[A1][0] = rec as u64;
    }

    /// One and two fields (`dec[0x174] == 3` or `dec[0xe0] == 0` means one),
    /// with and without a destination stride, 0 to 3 rows, interrupts
    /// enabled or not when masked.
    #[test]
    fn libmpeg_copy_output_raw_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_00408278__generated,
            libmpeg_copy_output_raw,
            384,
            &oc1120_copy_setup,
        );
        let paths = oc1120_paths(ee::s15::f_00408278__generated, 384, &oc1120_copy_setup);
        assert!(paths >= 6, "only {paths} paths");
    }

    // ------------------------------------------------------------------
    // libmpeg_ConvertPictureCsc

    pub(crate) fn oc1120_csc_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x409088, 14);
        fill(c, &mut r, OC1120_DEC, 0x440);
        let rec = OC1120_REC;
        let count = [5u32, 1023, 1024, 2730, 2731, 5000][(seed % 6) as usize];
        c.bus.write32(rec, OC1120_SRC + 16 * (seed / 6 % 4));
        c.bus.write32(rec + 0xc, count);
        c.bus.write32(rec + 0x10, 1);
        if seed / 12 % 2 == 1 {
            // An invalid VDEC command leaves ECD set in IPU_CTRL.
            c.bus.write32(0x1000_2000, 0x3400_0000);
        }
        let intr = seed / 24 % 2;
        oc1120_mb(c, K_DINTR, |_| (intr, 0));
        oc1120_mb(c, K_ADD_HANDLER, |_| (1 + r.next_u32() % 100, 0));
        c.g[A0][0] = OC1120_DEC as u64;
        c.g[A1][0] = rec as u64;
    }

    /// Fewer and more than 1023 macroblocks (`IpuCsc` / `IpuCscDma`), a
    /// transfer within 0xffff quadwords and one split with a DMAC handler,
    /// interrupts enabled or not, an IPU error cleared first.
    #[test]
    fn libmpeg_convert_picture_csc_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_00409088__generated,
            libmpeg_convert_picture_csc,
            288,
            &oc1120_csc_setup,
        );
        let paths = oc1120_paths(ee::s15::f_00409088__generated, 288, &oc1120_csc_setup);
        assert!(paths >= 6, "only {paths} paths");
    }

    /// Gate of this module's callee fakes: they answer only while this word
    /// holds `OC1121_MAGIC` (otherwise the body they replaced runs).
    pub(crate) const OC1121_CTRL: u32 = DATA + 0x000f_0000;

    pub(crate) const OC1121_MAGIC: u32 = 0x3131_4c43;

    /// Results the fakes return, one word per logged fake call.
    pub(crate) const OC1121_MBOX: u32 = DATA + 0x000f_1000;

    pub(crate) static OC1121_PREV_DECODE_SLICE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_IPU_WAIT_BUSY: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_WAIT_BDEC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_ADD_PRED: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_FLUSH_BITS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_IPU_VDEC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_DECODE_DM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_MOTION_VECTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_DECODE_MV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_CALLBACK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1121_PREV_EXTRA_BIT: AtomicUsize = AtomicUsize::new(0);

    /// Log the call, return the next mailbox word in v0.
    pub(crate) fn oc1121_answer(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = c.bus.read32(OC1121_MBOX + 4 * (n % 64));
        fakes::fake_call(c, target);
        c.g[V0][0] = result as u64;
    }

    /// A gated callee fake: answers from the mailbox while the gate is on,
    /// else runs the body it replaced.
    macro_rules! oc1121_fake {
        ($fake:ident, $prev:ident, $target:expr, $body:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1121_CTRL) != OC1121_MAGIC {
                    return fakes::fall_back(c, &$prev, $body);
                }
                oc1121_answer(c, $target);
            }
        };
    }

    oc1121_fake!(
        oc1121_fake_decode_slice,
        OC1121_PREV_DECODE_SLICE,
        0x0040_6318,
        ee::s15::f_00406318__generated
    );

    oc1121_fake!(
        oc1121_fake_ipu_wait_busy,
        OC1121_PREV_IPU_WAIT_BUSY,
        0x0040_6e20,
        ee::s15::f_00406e20__generated
    );

    oc1121_fake!(
        oc1121_fake_wait_bdec,
        OC1121_PREV_WAIT_BDEC,
        0x0040_5b70,
        ee::s15::f_00405b70__generated
    );

    oc1121_fake!(
        oc1121_fake_add_pred,
        OC1121_PREV_ADD_PRED,
        0x0040_4b78,
        ee::s15::f_00404b78__generated
    );

    oc1121_fake!(
        oc1121_fake_error,
        OC1121_PREV_ERROR,
        0x0040_34a8,
        ee::s15::f_004034a8__generated
    );

    oc1121_fake!(
        oc1121_fake_get_bits,
        OC1121_PREV_GET_BITS,
        0x0040_72e8,
        ee::s15::f_004072e8__generated
    );

    oc1121_fake!(
        oc1121_fake_flush_bits,
        OC1121_PREV_FLUSH_BITS,
        0x0040_71f8,
        ee::s15::f_004071f8__generated
    );

    oc1121_fake!(
        oc1121_fake_ipu_vdec,
        OC1121_PREV_IPU_VDEC,
        0x0040_6f78,
        ee::s15::f_00406f78__generated
    );

    oc1121_fake!(
        oc1121_fake_decode_dm,
        OC1121_PREV_DECODE_DM,
        0x0040_5e10,
        ee::s15::f_00405e10__generated
    );

    oc1121_fake!(
        oc1121_fake_motion_vector,
        OC1121_PREV_MOTION_VECTOR,
        0x0040_6ca0,
        ee::s15::f_00406ca0__generated
    );

    oc1121_fake!(
        oc1121_fake_decode_mv,
        OC1121_PREV_DECODE_MV,
        0x0040_6a78,
        ee::s15::f_00406a78__generated
    );

    oc1121_fake!(
        oc1121_fake_extra_bit,
        OC1121_PREV_EXTRA_BIT,
        0x0040_7a10,
        ee::s15::f_00407a10__generated
    );

    /// The host callback: logs the call and makes the IPU command ready, so
    /// the poll loop ends.
    pub(crate) fn oc1121_fake_callback(c: &mut Cpu) {
        if c.bus.read32(OC1121_CTRL) != OC1121_MAGIC {
            return fakes::fall_back(c, &OC1121_PREV_CALLBACK, ee::s15::f_00402c80__generated);
        }
        fakes::fake_call(c, 0x0040_2c80);
        c.bus.write64(0x1000_2000, 0x5);
    }

    /// Hook this module's fakes into their override slots (once per process).
    pub(crate) fn oc1121_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406318),
            oc1121_fake_decode_slice,
            &OC1121_PREV_DECODE_SLICE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406E20),
            oc1121_fake_ipu_wait_busy,
            &OC1121_PREV_IPU_WAIT_BUSY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405B70),
            oc1121_fake_wait_bdec,
            &OC1121_PREV_WAIT_BDEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00404B78),
            oc1121_fake_add_pred,
            &OC1121_PREV_ADD_PRED,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1121_fake_error,
            &OC1121_PREV_ERROR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1121_fake_get_bits,
            &OC1121_PREV_GET_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            oc1121_fake_flush_bits,
            &OC1121_PREV_FLUSH_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406F78),
            oc1121_fake_ipu_vdec,
            &OC1121_PREV_IPU_VDEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405E10),
            oc1121_fake_decode_dm,
            &OC1121_PREV_DECODE_DM,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406CA0),
            oc1121_fake_motion_vector,
            &OC1121_PREV_MOTION_VECTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406A78),
            oc1121_fake_decode_mv,
            &OC1121_PREV_DECODE_MV,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C80),
            oc1121_fake_callback,
            &OC1121_PREV_CALLBACK,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407A10),
            oc1121_fake_extra_bit,
            &OC1121_PREV_EXTRA_BIT,
        );
    }

    /// Gate this module's fakes on in the test machine and fill the mailbox.
    pub(crate) fn oc1121_arm(c: &mut Cpu, results: &[u32]) {
        c.bus.write32(OC1121_CTRL, OC1121_MAGIC);
        for (k, v) in results.iter().enumerate() {
            c.bus.write32(OC1121_MBOX + 4 * k as u32, *v);
        }
    }

    #[test]
    fn libmpeg_decode_slices_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_004060c8__generated,
            libmpeg_decode_slices,
            300,
            &|c, seed| {
                let mut r = Rng(0x4060_c800 ^ seed as u64);
                scramble(c, &mut r);
                // Slice results: re-runs (1, 3), a clean one (0), errors (2, 4).
                let mut results = [0u32; 8];
                for v in results.iter_mut() {
                    *v = [0, 1, 2, 3, 4][(r.next_u32() % 5) as usize];
                }
                oc1121_arm(c, &results);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x12c, 2 + r.next_u32() % 6);
                c.bus.write32(DATA + 0x130, 1 + r.next_u32() % 4);
                c.bus.write32(DATA + 0x174, [3, 2, 1][seed as usize % 3]);
                c.bus
                    .write32(DATA + 0x10a0, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(0x1000_d400, 0);
            },
        );
    }

    #[test]
    fn libmpeg_skipped_macroblock_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406510__generated,
            libmpeg_skipped_macroblock,
            300,
            &|c, seed| {
                let mut r = Rng(0x4065_1000 ^ seed as u64);
                scramble(c, &mut r);
                oc1121_arm(c, &[]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2_0000);
                c.g[A2][0] = sx(DATA + 0x2_0100);
                c.g[A3][0] = sx(DATA + 0x2_0200);
                c.g[T0][0] = sx(DATA + 0x2_0300);
                c.bus.write32(DATA + 0x810, r.next_u32() % 4);
                c.bus.write32(DATA + 0x150, [0, 1, 2, 3][seed as usize % 4]);
                c.bus
                    .write32(DATA + 0x174, [1, 2, 3][(seed / 4) as usize % 3]);
                c.bus.write32(DATA + 0x2_0300, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_decode_motion_vector_matches() {
        check(
            ee::s15::f_00406a78__generated,
            libmpeg_decode_motion_vector,
            300,
            &|c, seed| {
                let mut r = Rng(0x6a78_0001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let mc = (r.next_u32() % 33) as i32 - 16;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(mc as u32);
                c.g[A3][0] = sx(r.next_u32() % 9);
                c.g[T0][0] = (seed % 2) as u64;
                c.bus
                    .write32(DATA, ((r.next_u32() % 400) as i32 - 200) as u32);
            },
        );
    }

    #[test]
    fn libmpeg_motion_vectors_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406b00__generated,
            libmpeg_motion_vectors,
            300,
            &|c, seed| {
                let mut r = Rng(0x6b00_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let results: Vec<u32> = (0..16).map(|_| r.next_u32() % 3).collect();
                oc1121_arm(c, &results);
                let stack = crate::difftest::SP;
                c.g[SP][0] = sx(stack);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3_0000);
                c.g[A2][0] = sx(DATA + 0x3_1000);
                c.g[A3][0] = sx(DATA + 0x3_2000);
                c.g[T0][0] = (seed % 3) as u64;
                c.g[T1][0] = 1 + (seed % 2) as u64;
                c.g[T2][0] = ((seed / 2) % 2) as u64;
                c.g[T3][0] = (r.next_u32() % 4) as u64;
                c.bus.write32(stack + 0xb0, r.next_u32() % 4);
                c.bus.write32(stack + 0xb8, (seed / 4) % 2);
                c.bus.write32(stack + 0xc0, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn libmpeg_motion_vector_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406ca0__generated,
            libmpeg_motion_vector,
            300,
            &|c, seed| {
                let mut r = Rng(0x6ca0_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let results: Vec<u32> = (0..16).map(|_| r.next_u32() % 5).collect();
                oc1121_arm(c, &results);
                c.g[SP][0] = sx(crate::difftest::SP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2_0000);
                c.g[A2][0] = sx(DATA + 0x2_1000);
                c.g[A3][0] = sx(r.next_u32() % 3);
                c.g[T0][0] = (r.next_u32() % 3) as u64;
                c.g[T1][0] = (seed % 2) as u64;
                c.g[T2][0] = ((seed / 2) % 2) as u64;
                c.g[T3][0] = ((seed / 4) % 2) as u64;
                c.bus.write32(DATA + 0x2_0004, r.next_u32() % 64);
            },
        );
    }

    #[test]
    fn libmpeg_ipu_wait_cmd_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_00406ec8__generated,
            libmpeg_ipu_wait_cmd,
            200,
            &|c, seed| {
                let mut r = Rng(0x6ec8_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                oc1121_arm(c, &[]);
                c.g[A0][0] = sx(DATA);
                // Ready (seed % 3 == 0), error flag set (1), or polled to the callback (2).
                let cmd = if seed % 3 == 0 {
                    (r.next_u32() & 0x7fff_ffff) as u64
                } else {
                    (r.next_u32() as u64) | 0x8000_0000_0000_0000
                };
                c.bus.write64(0x1000_2000, cmd);
                c.bus
                    .write32(0x1000_2010, if seed % 3 == 1 { 0x4000 } else { 0 });
            },
        );
    }

    #[test]
    fn libmpeg_slice_header_tail_matches() {
        oc1121_install_fakes();
        check(
            ee::s15::f_004074b8__generated,
            libmpeg_slice_header_tail,
            200,
            &|c, seed| {
                let mut r = Rng(0x74b8_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                // First bits (any), the flag (zero or not), then the tail's results.
                let mut results = vec![r.next_u32(), seed % 2];
                results.extend((0..6).map(|_| r.next_u32() % 3));
                oc1121_arm(c, &results);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// Gate of the MPEG callee fakes: they answer only while this word of the
    /// fixture's RAM holds `OC1122_MAGIC` (otherwise the generated body runs).
    pub(crate) const OC1122_CTRL: u32 = DATA + 0x0002_0000;

    pub(crate) const OC1122_MAGIC: u32 = 0x1122_0407;

    /// The 64 `GetBits` results of a fixture, one word per fake call; zero
    /// past the end, so every bit loop terminates.
    pub(crate) const OC1122_BITS: u32 = DATA + 0x0002_0100;

    pub(crate) static OC1122_PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1122_PREV_FLUSH_BITS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1122_PREV_IPU_WAIT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1122_PREV_IPU_COMMAND: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1122_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1122_PREV_EXT_USER_DATA: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1122_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1122_CTRL) == OC1122_MAGIC
    }

    /// `libmpeg_GetBits`: logs the call and returns the next seeded result.
    pub(crate) fn oc1122_fake_get_bits(c: &mut Cpu) {
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

    pub(crate) fn oc1122_fake_flush_bits(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_FLUSH_BITS, ee::s15::f_004071f8__generated);
        }
        fakes::fake_call(c, 0x0040_71f8);
    }

    pub(crate) fn oc1122_fake_ipu_wait(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_IPU_WAIT, ee::s15::f_00406e20__generated);
        }
        fakes::fake_call(c, 0x0040_6e20);
    }

    pub(crate) fn oc1122_fake_ipu_command(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_IPU_COMMAND, ee::s15::f_00406df0__generated);
        }
        fakes::fake_call(c, 0x0040_6df0);
    }

    pub(crate) fn oc1122_fake_error(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(c, &OC1122_PREV_ERROR, ee::s15::f_004034a8__generated);
        }
        fakes::fake_call(c, 0x0040_34a8);
    }

    pub(crate) fn oc1122_fake_ext_user_data(c: &mut Cpu) {
        if !oc1122_gated(c) {
            return fakes::fall_back(
                c,
                &OC1122_PREV_EXT_USER_DATA,
                ee::s15::f_004076f8__generated,
            );
        }
        fakes::fake_call(c, 0x0040_76f8);
    }

    pub(crate) fn oc1122_install_fakes() {
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
    pub(crate) fn oc1122_mpeg_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
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

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC1123_MAGIC` (otherwise the bodies they replaced run).
    pub(crate) const OC1123_CTRL: u32 = DATA + 0x0079_0000;

    pub(crate) const OC1123_MAGIC: u32 = 0x1123_c0de;

    /// The DIntr result the fake returns (0 or not), and the stop flag the
    /// chunk-handler fake leaves in the transfer state.
    pub(crate) const OC1123_INTR: u32 = DATA + 0x0079_0010;

    pub(crate) const OC1123_STOP: u32 = DATA + 0x0079_0018;

    /// The decoder state and the picture record.
    pub(crate) const OC1123_DEC: u32 = DATA + 0x0079_1000;

    pub(crate) const OC1123_PIC: u32 = DATA + 0x0079_2000;

    /// ComputePts' outputs: PTS, DTS, flags.
    pub(crate) const OC1123_OUT: u32 = DATA + 0x0079_3000;

    /// The mpeg system state (its +0x858 field is the dispatch object).
    pub(crate) const OC1123_MPEG: u32 = DATA + 0x0079_4000;

    pub(crate) const OC1123_OBJ: u32 = DATA + 0x0079_5000;

    pub(crate) fn oc1123_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1123_CTRL) == OC1123_MAGIC
    }

    /// A callee fake that logs the call and returns (gate on), or runs the
    /// body it replaced (gate off).
    macro_rules! oc1123_fake {
        ($name:ident, $target:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if !oc1123_gated(c) {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc1123_fake!(
        oc1123_fake_sprintf,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    oc1123_fake!(
        oc1123_fake_error,
        0x0040_34a8,
        ee::s15::f_004034a8__generated
    );

    oc1123_fake!(
        oc1123_fake_eintr,
        0x0042_c0c0,
        ee::s15::f_0042c0c0__generated
    );

    oc1123_fake!(
        oc1123_fake_ipu_command,
        0x0040_6df0,
        ee::s15::f_00406df0__generated
    );

    oc1123_fake!(
        oc1123_fake_enable,
        0x0042_49b8,
        ee::s15::f_004249b8__generated
    );

    oc1123_fake!(
        oc1123_fake_disable,
        0x0042_4950,
        ee::s15::f_00424950__generated
    );

    oc1123_fake!(
        oc1123_fake_remove,
        0x0042_3ad0,
        ee::s15::f_00423ad0__generated
    );

    /// DIntr: returns the state the test put in OC1123_INTR (zero included).
    pub(crate) fn oc1123_fake_dintr(c: &mut Cpu) {
        if !oc1123_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1123_fake_dintr),
                ee::s15::f_0042c078__generated,
            );
        }
        let state = c.bus.read32(OC1123_INTR);
        fakes::fake_call(c, 0x0042_c078);
        c.g[V0][0] = state as u64;
    }

    /// AddDmacHandler: the chunk handler is never run; the transfer state
    /// (a3) is left as if every chunk was fed, with the stop flag from the test.
    pub(crate) fn oc1123_fake_add_handler(c: &mut Cpu) {
        if !oc1123_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1123_fake_add_handler),
                ee::s15::f_00423ac0__generated,
            );
        }
        let state = c.g[A3][0] as u32;
        let n = c.bus.read32(state + 0x10);
        let stop = c.bus.read32(OC1123_STOP);
        fakes::fake_call(c, 0x0042_3ac0);
        c.bus.write32(state, n);
        c.bus.write32(state + 4, stop);
    }

    /// DispatchCallback: the DMA the caller started has finished (CHCR's STR
    /// bit is cleared, the way the IPU would clear it), and the IPU is idle:
    /// the CSC command has no input in the test machine, so BUSY would stay
    /// set. A CTRL write with RST (bit 30) resets the IPU and clears BUSY.
    pub(crate) fn oc1123_fake_dispatch(c: &mut Cpu) {
        if !oc1123_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1123_fake_dispatch),
                ee::s15::f_00402c30__generated,
            );
        }
        fakes::fake_call(c, 0x0040_2c30);
        c.sw(0x1000_b000, 0);
        c.sw(0x1000_2010, 1u64 << 30);
    }

    pub(crate) fn oc1123_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1123_fake_sprintf,
            fakes::prev_cell(oc1123_fake_sprintf),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1123_fake_error,
            fakes::prev_cell(oc1123_fake_error),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C078),
            oc1123_fake_dintr,
            fakes::prev_cell(oc1123_fake_dintr),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C0C0),
            oc1123_fake_eintr,
            fakes::prev_cell(oc1123_fake_eintr),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406DF0),
            oc1123_fake_ipu_command,
            fakes::prev_cell(oc1123_fake_ipu_command),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1123_fake_dispatch,
            fakes::prev_cell(oc1123_fake_dispatch),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423AC0),
            oc1123_fake_add_handler,
            fakes::prev_cell(oc1123_fake_add_handler),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423AD0),
            oc1123_fake_remove,
            fakes::prev_cell(oc1123_fake_remove),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004249B8),
            oc1123_fake_enable,
            fakes::prev_cell(oc1123_fake_enable),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424950),
            oc1123_fake_disable,
            fakes::prev_cell(oc1123_fake_disable),
        );
    }

    pub(crate) fn oc1123_u64(r: &mut Rng) -> u64 {
        (r.next_u32() as u64) << 32 | r.next_u32() as u64
    }

    /// Small signed values (-8..=15) as 32-bit words.
    pub(crate) fn oc1123_small(r: &mut Rng) -> u32 {
        (r.next_u32() % 24).wrapping_sub(8)
    }

    #[test]
    fn oc1123_check_output_buffer_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_004081d8__generated,
            oc1123_check_output_buffer,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0001, 2);
                c.pc = 0x0040_81d8;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                // Rows case (seed % 4 != 0) and the flat case (rows == 0).
                c.bus.write32(OC1123_DEC + 0xdc, oc1123_small(&mut r));
                let rows = if seed % 4 == 0 {
                    0
                } else {
                    oc1123_small(&mut r)
                };
                c.bus.write32(OC1123_DEC + 0xe0, rows);
                c.bus.write32(OC1123_DEC + 0xe4, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0x4, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0x8, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0xc, oc1123_small(&mut r));
                c.bus.write32(OC1123_PIC + 0x10, oc1123_small(&mut r));
                c.g[A0][0] = sx(OC1123_DEC);
                c.g[A1][0] = sx(OC1123_PIC);
            },
        );
    }

    #[test]
    fn oc1123_compute_pts_matches() {
        check(
            ee::s15::f_00408530__generated,
            oc1123_compute_pts,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0002, 0);
                c.pc = 0x0040_8530;
                c.g[A0][0] = sx(OC1123_DEC);
                c.g[A1][0] = sx(OC1123_PIC);
                c.g[A2][0] = sx(OC1123_OUT);
                c.g[A3][0] = sx(OC1123_OUT + 0x8);
                c.g[T0][0] = sx(OC1123_OUT + 0x10);
                let d = OC1123_DEC;
                // No state (f70 == 0) on a quarter of the seeds.
                let f70 = if seed % 4 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(d + 0x70, f70);
                c.bus.write64(d + 0x78, oc1123_u64(&mut r));
                c.bus.write64(d + 0x88, oc1123_u64(&mut r));
                // The last PTS: negative on a fifth of the seeds.
                let last = r.next_u32();
                let last = if seed % 5 == 0 {
                    last | 0x8000_0000
                } else {
                    last & 0x7fff_ffff
                };
                c.bus.write32(d + 0x80, last);
                c.bus.write32(d + 0x90, r.next_u32());
                // The hand-off: fF8 == 2 on a seventh, fF0 negative on half.
                let f0 = oc1123_u64(&mut r);
                let f0 = if seed % 2 == 0 {
                    f0 | (1u64 << 63)
                } else {
                    f0 & !(1u64 << 63)
                };
                c.bus.write64(d + 0xf0, f0);
                c.bus
                    .write32(d + 0xf8, if seed % 7 == 0 { 2 } else { r.next_u32() % 4 });
                // The picture record: PTS negative on a third of the seeds.
                let p = OC1123_PIC;
                let pts = oc1123_u64(&mut r);
                let pts = if seed % 3 == 0 {
                    pts | (1u64 << 63)
                } else {
                    pts & !(1u64 << 63)
                };
                c.bus.write64(p + 0x18, pts);
                c.bus.write64(p + 0x20, oc1123_u64(&mut r));
                for off in [0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40] {
                    c.bus.write32(p + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn oc1123_ipu_from_dma_start_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_00408af8__generated,
            oc1123_ipu_from_dma_start,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0003, 2);
                c.pc = 0x0040_8af8;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                // Interrupts were off (0) or on, on alternate seeds.
                let intr = if seed % 2 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(OC1123_INTR, intr);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn oc1123_ipu_csc_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_00408b80__generated,
            oc1123_ipu_csc,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0004, 2);
                c.pc = 0x0040_8b80;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                let intr = if seed % 2 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(OC1123_INTR, intr);
                c.bus.write32(OC1123_MPEG + 0x858, OC1123_OBJ);
                c.g[A0][0] = sx(OC1123_MPEG);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32() % 64);
            },
        );
    }

    #[test]
    fn oc1123_ipu_csc_dma_matches() {
        oc1123_install_fakes();
        check(
            ee::s15::f_00408df0__generated,
            oc1123_ipu_csc_dma,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1123_0005, 2);
                c.pc = 0x0040_8df0;
                c.bus.write32(OC1123_CTRL, OC1123_MAGIC);
                let intr = if seed % 2 == 0 { 0 } else { r.next_u32() | 0x1 };
                c.bus.write32(OC1123_INTR, intr);
                // The chunk handler reports a stop on a fifth of the seeds.
                c.bus
                    .write32(OC1123_STOP, if seed % 5 == 0 { 1 } else { 0 });
                c.bus.write32(OC1123_MPEG + 0x858, OC1123_OBJ);
                c.g[A0][0] = sx(OC1123_MPEG);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32() % 0x2000);
            },
        );
    }

    /// The IPU bit position register reads 0 on the test platform, so the
    /// alignment flush is not taken here; the start-code loop is.
    #[test]
    fn libmpeg_next_start_code_matches() {
        check_lohi(
            ee::s15::f_00407438__generated,
            libmpeg_next_start_code,
            200,
            &|c, seed| {
                let _ = oc1137_setup(c, seed, 0x0040_7438, 8);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn libmpeg_clear_frame_flags_matches() {
        check(
            ssx3_ee::s15::f_00402bb0__generated,
            libmpeg_clear_frame_flags,
            200,
            &|c, seed| {
                let mut r = Rng(0x4c4d_5045 ^ seed as u64);
                scramble(c, &mut r);
                let sys = DATA + 0x1000;
                c.bus.write32(DATA + CLEAR_FRAME_SYS, sys);
                for (i, slot) in [0x1b8u32, 0x1c8, 0x1d8, 0x1bc, 0x1cc, 0x1dc]
                    .iter()
                    .enumerate()
                {
                    let record = DATA + 0x2000 + (i as u32) * 0x100;
                    let present = seed % 7 != 0 && (seed >> i) & 1 == 1;
                    c.bus.write32(sys + slot, if present { record } else { 0 });
                    c.bus.write32(record + CLEAR_FRAME_FLAGS, r.next_u32());
                }
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
