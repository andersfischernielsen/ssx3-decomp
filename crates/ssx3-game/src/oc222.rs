//! Staging wave 203 (oc222): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_7828, sprintf, "sprintf"),
    (0x0041_23e8, fpcmp_parts_d, "__fpcmp_parts_d"),
    (0x0021_e7a8, hud_align_y, "HUD_AlignY"),
    (
        0x0034_bca0,
        group_compute_verts,
        "cFlagManager_GroupComputeVerts",
    ),
    (0x0026_f4a8, replay_update, "Replay_Update"),
];

// ---------------------------------------------------------------------------
// sprintf (0x00417828)

/// `int vfprintf(FILE*, const char*, va_list)` (0x0041b658).
const VFPRINTF: Callee = Callee {
    addr: 0x0041_b658,
    f: ee::s15::f_0041b658,
};

/// `_impure_ptr` (the global reentrancy struct pointer) at 0x454944.
const IMPURE_PTR: u32 = 0x0045_4944;

/// The string FILE newlib builds on the stack: `_p` (+0), `_w` (+0x8),
/// `_flags` (+0xc), `_bf._base` (+0x10), `_bf._size` (+0x14), `_data`
/// (+0x54).
const FILE_P: u32 = 0x00;
const FILE_W: u32 = 0x08;
const FILE_FLAGS: u32 = 0x0c;
const FILE_BASE: u32 = 0x10;
const FILE_SIZE: u32 = 0x14;
const FILE_DATA: u32 = 0x54;
/// `__SWR | __SSTR`.
const STRING_FLAGS: u64 = 0x208;
/// The variadic register save area (a2..t3 as 64-bit, then f12/f14/f16/f18).
const VA_GPRS: u32 = 0xb0;
const VA_FPRS: u32 = 0xa0;

/// `int sprintf(char* buf, const char* fmt, ...)` (0x00417828): newlib's
/// string printf. Builds a write-only string FILE on the stack over `buf`
/// (unbounded size 0x7fffffff), spills the variadic registers into a
/// `va_list` area, formats with `vfprintf` and terminates the output with
/// a NUL. Returns the character count from `vfprintf` in v0.
pub fn sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0xe0);
        c.g[T5][0] = crate::guest::Addr(IMPURE_PTR).lw(c);
        c.g[T4][0] = c.g[A0][0];
        c.g[V1][0] = 0x7fff_ffff;
        let a = frame.at(c, VA_GPRS);
        c.sd(a, c.g[A2][0]);
        c.g[V0][0] = STRING_FLAGS;
        frame.save_ra(c, 0x60);
        c.g[A0][0] = c.g[SP][0];
        let file = sp(c);
        c.sw(file + FILE_SIZE, c.g[V1][0]);
        c.g[A2][0] = sx(file.wrapping_add(VA_GPRS));
        c.sw(file + FILE_W, c.g[V1][0]);
        for (i, r) in [A3, T0, T1, T2, T3].into_iter().enumerate() {
            let a = file.wrapping_add(VA_GPRS + 8 + 8 * i as u32);
            c.sd(a, c.g[r][0]);
        }
        for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
            c.swc1(f, file.wrapping_add(VA_FPRS + 4 * i as u32));
        }
        c.sh(file + FILE_FLAGS, c.g[V0][0]);
        c.sw(file + FILE_BASE, c.g[T4][0]);
        c.sw(file + FILE_DATA, c.g[T5][0]);
        c.sw(file + FILE_P, c.g[T4][0]);
        VFPRINTF.call(c, 0x0041_7894)?;
        // *fp._p = '\0'
        c.g[V1][0] = crate::guest::Addr(sp(c) + FILE_P).lw(c);
        frame.restore_ra(c, 0x60);
        c.sb(c.g[V1][0] as u32, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __fpcmp_parts_d (0x004123e8)

/// libgcc's unpacked double (`fp_number_type`): class, sign, exponent,
/// 64-bit fraction.
const FP_CLASS: u32 = 0x0;
const FP_SIGN: u32 = 0x4;
const FP_EXP: u32 = 0x8;
const FP_FRAC: u32 = 0x10;
/// Classes: 0/1 = NaNs (below 2), 2 = zero, 3 = normal, 4 = infinity.
const CLASS_ZERO: u64 = 2;
const CLASS_INFINITY: u64 = 4;

/// `int __fpcmp_parts_d(fp_number_type* a, fp_number_type* b)` (0x004123e8):
/// the soft-float double compare core. 1 if either is a NaN; infinities and
/// zeros by sign; else by sign, exponent, then fraction. Result -1/0/1 in
/// v0 (two infinities: `b.sign - a.sign`). Leaves the original's scratch in
/// a0/a2/a3/v1.
pub fn fpcmp_parts_d(c: &mut Cpu) {
    use crate::guest::Addr;
    let a = c.g[A0][0] as u32;
    let b = c.g[A1][0] as u32;
    /// The shared tail: `v0` (already -1) becomes `v1` (1) when the sign
    /// is clear (gcc's `movz`).
    fn by_sign(c: &mut Cpu, cycles: u64, sign: u64) {
        c.cycles += cycles;
        if sign == 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        ret(c);
    }

    c.cycles += 4;
    let class_a = Addr(a + FP_CLASS).lw(c);
    c.g[A2][0] = class_a;
    c.g[V0][0] = (class_a < 2) as u64;
    if class_a >= 2 {
        c.cycles += 4;
        let class_b = Addr(b + FP_CLASS).lw(c);
        c.g[V1][0] = class_b;
        c.g[V0][0] = class_a ^ CLASS_INFINITY;
        if class_b >= 2 {
            c.cycles += 2;
            c.g[V0][0] = class_b ^ CLASS_INFINITY;
            let sign_a;
            if class_a != CLASS_INFINITY {
                c.cycles += 2;
                if class_b == CLASS_INFINITY {
                    // Finite against infinity: the sign of b decides.
                    c.cycles += 5;
                    let sign_b = Addr(b + FP_SIGN).lw(c);
                    c.g[A0][0] = sign_b;
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 1;
                    if sign_b == 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    return ret(c);
                }
                c.g[V0][0] = class_a ^ CLASS_ZERO;
                c.cycles += 2;
                c.g[V0][0] = class_b ^ CLASS_ZERO;
                if class_a == CLASS_ZERO {
                    c.cycles += 2;
                    if class_b != CLASS_ZERO {
                        // Zero against a number: the sign of b decides.
                        let sign_b = Addr(b + FP_SIGN).lw(c);
                        c.g[A0][0] = sign_b;
                        c.cycles += 4;
                        c.g[V1][0] = 1;
                        c.g[V0][0] = u64::MAX;
                        if sign_b != 0 {
                            c.g[V0][0] = c.g[V1][0];
                        }
                        return ret(c);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    return ret(c);
                }
                c.cycles += 2;
                if class_b != CLASS_ZERO {
                    return compare_normals(c, a, b);
                }
                // A number against zero: the sign of a decides.
                sign_a = Addr(a + FP_SIGN).lw(c);
            } else {
                c.cycles += 2;
                if class_b == CLASS_INFINITY {
                    // Two infinities: b.sign - a.sign.
                    c.cycles += 4;
                    let sb = Addr(b + FP_SIGN).lw(c);
                    c.g[V1][0] = sb;
                    let sa = Addr(a + FP_SIGN).lw(c);
                    c.g[V0][0] = sx((sb as u32).wrapping_sub(sa as u32));
                    return ret(c);
                }
                // Infinity against a finite number: the sign of a decides.
                sign_a = Addr(a + FP_SIGN).lw(c);
            }
            c.g[A0][0] = sign_a;
            c.g[V1][0] = 1;
            c.g[V0][0] = u64::MAX;
            return by_sign(c, 4, sign_a);
        }
    }
    // A NaN: unordered.
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// Both operands normal (the `'b_4124a0` block of `__fpcmp_parts_d`).
fn fpcmp_normals_tail(c: &mut Cpu, cycles: u64, v0: u64, sign_a: u64) {
    c.cycles += cycles;
    c.g[V0][0] = v0;
    if sign_a == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    ret(c);
}

/// Two normal numbers: different signs -> by the sign of a; same sign ->
/// by exponent, then by fraction (`v1` = the result for a positive `a`,
/// negated by the sign).
fn compare_normals(c: &mut Cpu, a: u32, b: u32) {
    use crate::guest::Addr;
    c.cycles += 4;
    let sign_a = Addr(a + FP_SIGN).lw(c);
    c.g[A3][0] = sign_a;
    let sign_b = Addr(b + FP_SIGN).lw(c);
    c.g[V0][0] = sign_b;
    if sign_a != sign_b {
        c.cycles += 1;
        c.g[V1][0] = 1;
        return fpcmp_normals_tail(c, 3, u64::MAX, sign_a);
    }
    let exp_a = Addr(a + FP_EXP).lw(c);
    c.g[A2][0] = exp_a;
    c.cycles += 4;
    let exp_b = Addr(b + FP_EXP).lw(c);
    c.g[V1][0] = exp_b;
    c.g[V0][0] = ((exp_b as i64) < (exp_a as i64)) as u64;
    if (exp_b as i64) < (exp_a as i64) {
        // |a| > |b|.
        c.g[V1][0] = 1;
        return fpcmp_normals_tail(c, 3, u64::MAX, sign_a);
    }
    c.cycles += 3;
    c.g[V0][0] = ((exp_a as i64) < (exp_b as i64)) as u64;
    if (exp_a as i64) < (exp_b as i64) {
        // |a| < |b|.
        c.cycles += 1;
        c.g[V1][0] = u64::MAX;
        return fpcmp_normals_tail(c, 3, 1, sign_a);
    }
    let frac_a = Addr(a + FP_FRAC).ld(c);
    c.g[V1][0] = frac_a;
    c.cycles += 4;
    let frac_b = Addr(b + FP_FRAC).ld(c);
    c.g[A0][0] = frac_b;
    c.g[V0][0] = (frac_b < frac_a) as u64;
    if frac_b < frac_a {
        c.g[V1][0] = 1;
        return fpcmp_normals_tail(c, 3, u64::MAX, sign_a);
    }
    c.cycles += 3;
    c.g[V0][0] = (frac_a < frac_b) as u64;
    c.g[V1][0] = u64::MAX;
    if frac_a < frac_b {
        return fpcmp_normals_tail(c, 3, 1, sign_a);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// HUD_AlignY (0x0021e7a8)

/// `float HUD_AlignY(void* self, int align, float pos, float size, float
/// scale)` (0x0021e7a8): the top edge of a HUD element aligned at `pos`.
/// `align == 0` centres it (`pos - scale*size*0.5`), `align > 0` bottom
/// aligns it (`pos - scale*size`), `align < 0` keeps `pos` (top aligned).
/// Result in f0 (and f12 holds it too).
pub fn hud_align_y(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    let align = c.g[A1][0];
    let pos = FpuF32(c.f[12]);
    let size = FpuF32(c.f[13]);
    let scale = FpuF32(c.f[14]);
    c.cycles += 2;
    let mut top = pos;
    if align == 0 {
        c.cycles += 5;
        const HALF: u32 = 0x3f00_0000;
        c.g[AT][0] = HALF as u64;
        c.f[1] = HALF;
        let offset = (scale * size) * FpuF32(HALF);
        c.f[0] = offset.0;
        c.cycles += 1;
        top = pos - offset;
    } else {
        c.cycles += 2;
        if (align as i64) > 0 {
            c.cycles += 1;
            let offset = scale * size;
            c.f[0] = offset.0;
            c.cycles += 1;
            top = pos - offset;
        }
    }
    c.f[12] = top.0;
    c.cycles += 2;
    c.f[0] = top.0;
    ret(c);
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupComputeVerts (0x0034bca0)

/// `FlagManager* World_GetFlagManager()` (0x002d1cb0, called through the
/// function table as the original does).
const WORLD_GET_FLAG_MANAGER: u32 = 0x002d_1cb0;

/// `this + off` (or any register plus an offset), as the compiled address
/// arithmetic does it (32-bit wrap).
#[inline(always)]
fn ad(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// The sine/cosine slots of the 512-entry wave table at `base` for the
/// angle index in v0: `v0 = &table[i & 0x1ff]`, `v1 = &table[(i & 0x1ff) +
/// 0x80]` (a quarter turn later; the table has 0x80 extra entries).
#[inline(always)]
fn wave_slots(c: &mut Cpu, base: usize) {
    c.g[V0][0] &= 0x1ff;
    c.g[V1][0] = sx(ad(c, V0, 0x80));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[base][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[base][0] as u32));
}

/// Flag group fields (`cFlagGroup`, s0): style bytes, wave phases and
/// amplitudes, grid size, rest positions.
const GROUP_PINNED: u32 = 0x00; // byte; with +1 also a u16 "animated" test
const GROUP_MIRROR: u32 = 0x01; // byte: 1 = flip the columns
const GROUP_EDGE_ONLY: u32 = 0x02; // byte: freeze the first row
const GROUP_COLUMNS: u32 = 0x64;
const GROUP_ROWS: u32 = 0x68;
const GROUP_COLUMN_SPAN: u32 = 0x6c;
const GROUP_ROW_SPAN: u32 = 0x70;
const GROUP_REST_VERTS: u32 = 0x74;
const GROUP_WIND_BLEND: u32 = 0x4c;

/// `void cFlagManager_GroupComputeVerts(cFlagGroup* group, Vec3* verts)`
/// (0x0034bca0): animate a cloth flag. Four wave phases (+0x78, scaled by
/// a `.lit4` constant) and four amplitudes (+0x14, scaled by the wind
/// blend of the flag manager) drive a per-column offset table on the
/// stack; then every grid vertex is the rest vertex plus its column's
/// offset plus two travelling ripples weighted by the row. Pinned groups
/// fold the weight at the middle (`> 0.5` mirrors); the mirror style flips
/// the column order; the edge-only style copies the first row unchanged.
/// The register-level FPU ops and scratch registers follow the original
/// exactly (the wave table at 0x504fb8, `cvt.w.s` indices).
pub fn group_compute_verts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x120);
        'flags: {
            frame.save(c, S0, 0x110);
            frame.save(c, S2, 0xf0);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S3, 0xe0);
            c.g[S2][0] = 0;
            frame.save(c, S1, 0x100);
            frame.save_ra(c, 0xd0);
            c.g[V0][0] = c.lb(ad(c, S0, GROUP_PINNED));
            c.g[S3][0] = c.g[A1][0];
            if c.g[V0][0] == 0 {
                break 'flags;
            }
            c.cycles += 2;
            c.g[V0][0] = c.lb(ad(c, S0, GROUP_MIRROR));
            // s2: pinned and mirrored (the "fold at the middle" style).
            c.g[S2][0] = (c.g[V0][0] != 0) as u64;
        }
        c.cycles += 17;
        c.g[S1][0] = c.lhu(ad(c, S0, GROUP_PINNED)) as u64;
        call_ptr(c, WORLD_GET_FLAG_MANAGER, 0x0034_bcdc)?;

        // Wind blend: f12 = blend + manager.wind * (1 - blend).
        c.lwc1(2, ad(c, S0, GROUP_WIND_BLEND));
        c.g[S1][0] = (c.g[S1][0] < 1) as u64; // s1: not animated
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[A1][0] = sx(ad(c, SP, 0x10));
        c.lwc1(0, ad(c, V0, 0x10));
        c.g[A0][0] = sx(ad(c, S0, 0x14));
        c.sub_s(1, 1, 2);
        c.lwc1(3, ad(c, GP, 0u32.wrapping_sub(0x2afc)));
        c.g[V1][0] = c.g[SP][0];
        c.g[V0][0] = sx(ad(c, S0, 0x78));
        c.g[A2][0] = 3;
        c.mul_s(0, 0, 1);
        c.add_s(12, 2, 0);
        // Four phases (sp+0x00) and amplitudes (sp+0x10).
        loop {
            c.cycles += 12;
            c.lwc1(1, ad(c, V0, 0));
            c.g[A2][0] = sx(ad(c, A2, 0xffff_ffff));
            c.lwc1(0, ad(c, A0, 0));
            c.g[V0][0] = sx(ad(c, V0, 4));
            c.mul_s(1, 1, 3);
            c.g[A0][0] = sx(ad(c, A0, 4));
            c.mul_s(0, 0, 12);
            c.swc1(1, ad(c, V1, 0));
            c.swc1(0, ad(c, A1, 0));
            c.g[V1][0] = sx(ad(c, V1, 4));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx(ad(c, A1, 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_bd18);
            }
        }

        // Per-column offsets (sp+0x20, 12 bytes per column).
        c.cycles += 9;
        c.g[A0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
        c.g[V0][0] = 9;
        c.g[V1][0] = u64::MAX;
        c.mtc1(0, 13);
        c.g[A2][0] = 0;
        c.mov_s(16, 13);
        c.mov_s(15, 13);
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 14);
            loop {
                column_offsets(c);
                c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.add_s(13, 13, 14);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_bd78);
                }
            }
        }

        // The vertex grid.
        c.cycles += 5;
        c.g[V0][0] = c.lw(ad(c, S0, GROUP_ROWS));
        c.g[T2][0] = 0;
        c.mtc1(0, 18);
        c.g[V1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[T4][0] = 0xc;
            c.g[T1][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
                c.g[T3][0] = sx(ad(c, V1, 1));
                c.mtc1(0, 13);
                c.g[A3][0] = 0;
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 13;
                    c.mult(0, A2, T2, T4);
                    c.g[V0][0] = 0x50_0000;
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(AT, 11);
                    c.g[T0][0] = sx(ad(c, V0, 0x4fb8));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 12);
                    c.mov_s(14, 13);
                    c.lwc1(17, ad(c, GP, 0u32.wrapping_sub(0x2af0)));
                    c.g[A1][0] = sx(ad(c, SP, 0x20));
                    c.lwc1(16, ad(c, GP, 0u32.wrapping_sub(0x2aec)));
                    c.g[A0][0] = sx(ad(c, T1, c.g[S3][0] as u32));
                    c.lwc1(15, ad(c, GP, 0u32.wrapping_sub(0x2ae8)));
                    loop {
                        grid_vertex(c);
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
                        c.g[A3][0] = sx(ad(c, A3, 1));
                        c.g[A1][0] = sx(ad(c, A1, 0xc));
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.add_s(13, 13, 12);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_c030);
                        }
                    }
                }
                c.cycles += 7;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.g[V1][0] = c.g[T3][0];
                c.g[V0][0] = c.lw(ad(c, S0, GROUP_ROWS));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.add_s(18, 18, 0);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_bfe8);
                }
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// One column of the offset table (`a2` = column, f13 = column number as
/// a float, f14 = 1.0): its weight `f3`, the two travelling waves and the
/// static shape terms, into the 12-byte slot of the (possibly mirrored)
/// column index `a0`.
fn column_offsets(c: &mut Cpu) {
    'weighted: {
        c.cycles += 5;
        c.lwc1(0, ad(c, S0, GROUP_COLUMN_SPAN));
        c.g[A0][0] = c.g[A2][0];
        c.div_s(3, 13, 0);
        c.mov_s(4, 3);
        if c.g[S1][0] != 0 {
            // Not animated: full weight.
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 3);
            break 'weighted;
        }
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            // Folded at the middle.
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(0, 3);
            if c.fcc() {
                c.sub_s(3, 14, 3);
            } else {
                c.cycles += 2;
            }
            break 'weighted;
        }
        c.g[V1][0] = c.lb(ad(c, S0, GROUP_MIRROR));
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            // Mirrored: slot columns - a2 - 1.
            c.cycles += 3;
            c.g[V0][0] = c.lw(ad(c, S0, GROUP_COLUMNS));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
            c.g[A0][0] = sx(ad(c, V0, 0xffff_ffff));
        }
    }
    // First wave (x).
    c.cycles += 22;
    c.lwc1(2, ad(c, GP, 0u32.wrapping_sub(0x2af8)));
    c.g[V0][0] = 0x500000;
    c.lwc1(1, ad(c, S0, 0x24));
    c.g[A1][0] = sx(ad(c, V0, 0x4fb8));
    c.mul_s(5, 4, 2);
    c.lwc1(0, ad(c, SP, 0));
    c.lwc1(6, ad(c, GP, 0u32.wrapping_sub(0x2af4)));
    c.lwc1(4, ad(c, SP, 0x10));
    c.mul_s(1, 5, 1);
    c.add_s(0, 0, 1);
    c.mul_s(0, 0, 6);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    wave_slots(c, A1);
    c.lwc1(2, ad(c, V0, 0));
    c.lwc1(1, ad(c, V1, 0));
    if c.g[S2][0] == 0 {
        c.cycles += 3;
        c.mul_s(0, 4, 3);
        c.mul_s(0, 0, 1);
        c.add_s(16, 16, 0);
    }
    // Second wave (y); the slot gets (x, running sum, 0).
    c.cycles += 30;
    c.mul_s(0, 4, 3);
    c.g[V0][0] = 0xc;
    c.mult(0, 3, 4, 2);
    c.swc1(16, ad(c, SP, 0xa4));
    c.sw(ad(c, SP, 0xa8), 0u64);
    c.mul_s(0, 0, 2);
    c.lwc1(1, ad(c, S0, 0x28));
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(ad(c, SP, 0)));
    c.swc1(0, ad(c, SP, 0xa0));
    c.mul_s(1, 5, 1);
    c.ldl(7, ad(c, SP, 0xa7));
    c.ldr(7, ad(c, SP, 0xa0));
    c.g[T0][0] = c.lw(ad(c, SP, 0xa8));
    c.sdl(7, ad(c, A0, 0x27));
    c.sdr(7, ad(c, A0, 0x20));
    c.sw(ad(c, A0, 0x28), c.g[T0][0]);
    c.lwc1(0, ad(c, SP, 0x4));
    c.add_s(0, 0, 1);
    c.mul_s(0, 0, 6);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    wave_slots(c, A1);
    c.lwc1(2, ad(c, V0, 0));
    c.lwc1(1, ad(c, V1, 0));
    if c.g[S2][0] == 0 {
        c.cycles += 4;
        c.lwc1(0, ad(c, SP, 0x14));
        c.mul_s(0, 0, 3);
        c.mul_s(0, 0, 1);
        c.add_s(15, 15, 0);
    }
    // The static shape terms (+0x34..+0x48) blended by the wind.
    c.cycles += 64;
    c.lwc1(1, ad(c, SP, 0x14));
    c.sub_s(5, 14, 12);
    c.swc1(15, ad(c, SP, 0xa4));
    c.g[A2][0] = sx(ad(c, A2, 0x1));
    c.mul_s(1, 1, 3);
    c.sw(ad(c, SP, 0xa8), 0u64);
    c.lwc1(10, ad(c, S0, 0x40));
    c.mul_s(1, 1, 2);
    c.lwc1(9, ad(c, S0, 0x44));
    c.lwc1(7, ad(c, S0, 0x48));
    c.mul_s(10, 10, 3);
    c.lwc1(8, ad(c, S0, 0x34));
    c.mul_s(9, 9, 3);
    c.lwc1(6, ad(c, S0, 0x38));
    c.mul_s(7, 7, 3);
    c.swc1(1, ad(c, SP, 0xa0));
    c.mul_s(8, 8, 3);
    c.mul_s(6, 6, 3);
    c.lwc1(0, ad(c, A0, 0x20));
    c.mul_s(10, 10, 12);
    c.lwc1(4, ad(c, S0, 0x3c));
    c.mul_s(8, 8, 5);
    c.add_s(0, 0, 1);
    c.lwc1(2, ad(c, A0, 0x24));
    c.mul_s(4, 4, 3);
    c.lwc1(3, ad(c, A0, 0x28));
    c.mul_s(9, 9, 12);
    c.swc1(0, ad(c, A0, 0x20));
    c.mul_s(7, 7, 12);
    c.mul_s(4, 4, 5);
    c.lwc1(0, ad(c, SP, 0xa4));
    c.mul_s(6, 6, 5);
    c.add_s(11, 10, 8);
    c.add_s(2, 2, 0);
    c.add_s(1, 7, 4);
    c.add_s(5, 9, 6);
    c.swc1(2, ad(c, A0, 0x24));
    c.lwc1(0, ad(c, SP, 0xa8));
    c.add_s(3, 3, 0);
    c.swc1(3, ad(c, A0, 0x28));
    c.swc1(5, ad(c, SP, 0xc4));
    c.swc1(1, ad(c, SP, 0xc8));
    c.swc1(10, ad(c, SP, 0xa0));
    c.swc1(9, ad(c, SP, 0xa4));
    c.swc1(7, ad(c, SP, 0xa8));
    c.swc1(8, ad(c, SP, 0xb0));
    c.swc1(6, ad(c, SP, 0xb4));
    c.swc1(4, ad(c, SP, 0xb8));
    c.swc1(11, ad(c, SP, 0xc0));
    c.lwc1(1, ad(c, A0, 0x20));
    c.lwc1(2, ad(c, A0, 0x24));
    c.add_s(1, 1, 11);
    c.lwc1(3, ad(c, A0, 0x28));
    c.swc1(1, ad(c, A0, 0x20));
    c.lwc1(0, ad(c, SP, 0xc4));
    c.add_s(2, 2, 0);
    c.swc1(2, ad(c, A0, 0x24));
    c.lwc1(0, ad(c, SP, 0xc8));
    c.add_s(3, 3, 0);
    c.swc1(3, ad(c, A0, 0x28));
}

/// One grid vertex (`a0` = output, `a1` = its column's offset slot, `a2` =
/// rest-vertex offset; f13 = column, f18 = row as floats; f11 = 0.5,
/// f12 = 1.0, f14 = 0.0, f15..f17 = `.lit4` constants).
fn grid_vertex(c: &mut Cpu) {
    'style: {
        c.cycles += 6;
        c.lwc1(0, ad(c, S0, GROUP_COLUMN_SPAN));
        c.lwc1(1, ad(c, S0, GROUP_ROW_SPAN));
        c.div_s(8, 13, 0);
        c.div_s(9, 18, 1);
        c.mov_s(10, 8);
        if c.g[S2][0] != 0 {
            c.cycles += 4;
            c.c_lt_s(11, 8);
            if c.fcc() {
                c.sub_s(8, 12, 8);
                c.cycles += 1;
            } else {
                c.cycles += 2;
            }
            break 'style;
        }
        c.cycles += 4;
        c.g[V1][0] = c.lb(ad(c, S0, GROUP_MIRROR));
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 3;
            c.sub_s(8, 12, 8);
            c.mov_s(10, 8);
            c.cycles += 1;
            break 'style;
        }
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 8);
            c.cycles += 1;
        }
    }
    c.g[V0][0] = c.lb(ad(c, S0, GROUP_EDGE_ONLY));
    c.cycles += 2;
    let edge_only = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(ad(c, S0, GROUP_REST_VERTS));
    if edge_only {
        c.cycles += 3;
        c.c_eq_s(9, 14);
        if c.fcc() {
            // First row of an edge-only flag: the rest vertex as is.
            c.cycles += 8;
            c.g[V0][0] = sx(ad(c, A2, c.g[V0][0] as u32));
            c.ldl(V1, ad(c, V0, 0x7));
            c.ldr(V1, ad(c, V0, 0));
            c.g[T5][0] = c.lw(ad(c, V0, 0x8));
            c.sdl(V1, ad(c, A0, 0x7));
            c.sdr(V1, ad(c, A0, 0));
            c.sw(ad(c, A0, 0x8), c.g[T5][0]);
            return;
        }
    }
    // Rest vertex plus the column offset.
    c.cycles += 22;
    c.c_lt_s(14, 10);
    c.lwc1(5, ad(c, A1, 0));
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(ad(c, V0, 0)));
    c.lwc1(4, ad(c, A1, 0x4));
    c.lwc1(2, ad(c, V0, 0x8));
    c.lwc1(0, ad(c, V0, 0));
    c.lwc1(1, ad(c, V0, 0x4));
    c.lwc1(3, ad(c, A1, 0x8));
    c.add_s(0, 0, 5);
    c.add_s(1, 1, 4);
    c.add_s(2, 2, 3);
    c.swc1(0, ad(c, SP, 0xa0));
    c.swc1(1, ad(c, SP, 0xa4));
    c.swc1(2, ad(c, SP, 0xa8));
    c.ldl(2, ad(c, SP, 0xa7));
    c.ldr(2, ad(c, SP, 0xa0));
    c.g[V1][0] = c.lw(ad(c, SP, 0xa8));
    c.sdl(2, ad(c, A0, 0x7));
    c.sdr(2, ad(c, A0, 0));
    c.sw(ad(c, A0, 0x8), c.g[V1][0]);
    let ripple1 = c.fcc();
    c.lwc1(6, ad(c, SP, 0x18));
    if ripple1 {
        c.cycles += 4;
        c.c_lt_s(17, 6);
        let strong = c.fcc();
        c.lwc1(1, ad(c, SP, 0x8));
        if strong {
            // Ripple 1 (amplitude sp+0x18, phase sp+0x08, freq +0x2c).
            c.cycles += 37;
            c.add_s(0, 10, 9);
            c.lwc1(2, ad(c, S0, 0x2c));
            c.add_s(3, 8, 9);
            c.mul_s(2, 2, 16);
            c.lwc1(5, ad(c, A0, 0));
            c.mul_s(0, 0, 11);
            c.mul_s(3, 3, 11);
            c.mul_s(4, 6, 9);
            c.mul_s(0, 0, 2);
            c.mul_s(3, 6, 3);
            c.mul_s(2, 6, 8);
            c.add_s(1, 1, 0);
            c.mul_s(1, 1, 15);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            wave_slots(c, T0);
            c.lwc1(0, ad(c, V0, 0));
            c.lwc1(1, ad(c, V1, 0));
            c.mul_s(4, 4, 0);
            c.mul_s(3, 3, 1);
            c.mul_s(2, 2, 0);
            c.swc1(4, ad(c, SP, 0xa8));
            c.swc1(3, ad(c, SP, 0xa4));
            c.swc1(2, ad(c, SP, 0xa0));
            c.add_s(5, 5, 2);
            c.lwc1(0, ad(c, A0, 0x4));
            c.lwc1(1, ad(c, A0, 0x8));
            c.add_s(0, 0, 3);
            c.swc1(5, ad(c, A0, 0));
            c.add_s(1, 1, 4);
            c.swc1(0, ad(c, A0, 0x4));
            c.swc1(1, ad(c, A0, 0x8));
        }
        c.cycles += 2;
        c.c_lt_s(14, 10);
    }
    c.cycles += 2;
    let ripple2 = c.fcc();
    c.lwc1(7, ad(c, SP, 0x1c));
    if ripple2 {
        c.cycles += 4;
        c.c_lt_s(17, 7);
        let strong = c.fcc();
        c.lwc1(1, ad(c, SP, 0xc));
        if strong {
            // Ripple 2 (amplitude sp+0x1c, phase sp+0x0c, freq +0x30).
            c.cycles += 39;
            c.add_s(0, 10, 9);
            c.lwc1(2, ad(c, S0, 0x30));
            c.add_s(4, 8, 9);
            c.mul_s(2, 2, 16);
            c.lwc1(6, ad(c, A0, 0));
            c.mul_s(0, 0, 11);
            c.mul_s(4, 4, 11);
            c.sub_s(3, 12, 9);
            c.mul_s(0, 0, 2);
            c.mul_s(5, 7, 8);
            c.mul_s(3, 7, 3);
            c.add_s(1, 1, 0);
            c.mul_s(4, 7, 4);
            c.mul_s(1, 1, 15);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            wave_slots(c, T0);
            c.lwc1(0, ad(c, V0, 0));
            c.lwc1(1, ad(c, V1, 0));
            c.mul_s(5, 5, 0);
            c.mul_s(4, 4, 1);
            c.mul_s(3, 3, 0);
            c.swc1(5, ad(c, SP, 0xa0));
            c.add_s(6, 6, 5);
            c.swc1(4, ad(c, SP, 0xa4));
            c.swc1(3, ad(c, SP, 0xa8));
            c.swc1(6, ad(c, A0, 0));
            c.lwc1(1, ad(c, A0, 0x4));
            c.lwc1(2, ad(c, A0, 0x8));
            c.add_s(1, 1, 4);
            c.swc1(1, ad(c, A0, 0x4));
            c.lwc1(0, ad(c, SP, 0xa8));
            c.add_s(2, 2, 0);
            c.swc1(2, ad(c, A0, 0x8));
        }
    }
    c.cycles += 4;
    c.g[A0][0] = sx(ad(c, A0, 0xc));
    c.g[A2][0] = sx(ad(c, A2, 0xc));
    c.g[T1][0] = sx(ad(c, T1, 0xc));
    c.g[T2][0] = sx(ad(c, T2, 1));
}

// ---------------------------------------------------------------------------
// Replay_Update (0x0026f4a8)

mod replay_callee {
    use super::{ee, Callee};
    pub const SET_STATE: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::Replay_SetState,
    };
    pub const HANDLE_INPUT: Callee = Callee {
        addr: 0x0026_fb88,
        f: ee::s07::Replay_HandleInput,
    };
    pub const PROCESS_COMMAND: Callee = Callee {
        addr: 0x0026_fe50,
        f: ee::s07::Replay_ProcessCommand,
    };
    pub const END_INSTANT_REPLAY: Callee = Callee {
        addr: 0x0027_0670,
        f: ee::s07::Replay_EndInstantReplay,
    };
    pub const CANCEL_INSTANT_REPLAY: Callee = Callee {
        addr: 0x0027_0628,
        f: ee::s07::Replay_CancelInstantReplay,
    };
    pub const CLEAR_ALL_HIGHLIGHTS: Callee = Callee {
        addr: 0x0027_0e18,
        f: ee::s07::Replay_ClearAllHighlights,
    };
    pub const PRUNE_SNAPSHOTS: Callee = Callee {
        addr: 0x0027_0538,
        f: ee::s07::Replay_PruneSnapshots,
    };
    pub const RECORD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d760,
        f: ee::s07::Replay_RecordSnapshot,
    };
    pub const ADD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d740,
        f: ee::s07::Replay_AddSnapshot,
    };
    pub const BEGIN_PLAYBACK: Callee = Callee {
        addr: 0x0026_f8a0,
        f: ee::s07::Replay_BeginPlayback,
    };
    /// `bool InputBinding_EvalBool(InputBinding*, int)` (0x320c48, another
    /// shard: through the function table).
    pub const INPUT_EVAL_BOOL: u32 = 0x0032_0c48;
}

/// Replay manager fields.
const REPLAY_STATE: u32 = 0x000;
const REPLAY_F20: u32 = 0x020;
const REPLAY_PLAYER: u32 = 0x028;
const REPLAY_FRAME: u32 = 0x3c8;
const REPLAY_SECONDS: u32 = 0x3cc;
const REPLAY_BLOCK: u32 = 0x3d0;
const REPLAY_START: u32 = 0x3d4;
const REPLAY_KEY_RATE: u32 = 0x018;
const REPLAY_LAST: u32 = 0x484;
const REPLAY_F604: u32 = 0x604;
const REPLAY_F608: u32 = 0x608;
const REPLAY_F60C: u32 = 0x60c;
const REPLAY_PAUSED: u32 = 0x610;
const REPLAY_TIMER: u32 = 0x614;
const REPLAY_F618: u32 = 0x618;
const REPLAY_F61C: u32 = 0x61c;
const REPLAY_F620: u32 = 0x620;
const REPLAY_F624: u32 = 0x624;
/// The state switch's jump table (0x480000 + 0x1750), 14 entries.
const REPLAY_STATE_TABLE: u32 = 0x0048_1750;

/// Where a state handler of `Replay_Update` continues.
enum ReplayNext {
    /// `Replay_ProcessCommand(this)` then out (0x26f73c).
    Command,
    /// `ProcessCommand` with the 2-cycle block (0x26f7a0).
    CommandShort,
    /// The common "restore s0" exit (0x26f7a8).
    Out,
    /// s0 already restored (the `lq` in the branch delay slot).
    Done,
}

/// `void Replay_Update(Replay* this)` (0x0026f4a8): the replay manager's
/// per-frame step. Counts down the timer, clears the per-frame flag, drops
/// back to state 3 on request (+0x618 == 7), then runs the state: record
/// (0/12: snapshot every 60 frames, a key frame every `keyRate` snapshots,
/// count seconds and frames; or start the pending instant replay or
/// playback), instant replay (2), playback (3, with the rider's skip
/// check), the transitions 8/9/11 and the command processing of the rest.
/// The state switch reads the original's jump table (other targets leave
/// through `jump_out`). Entries other than the start go to the generated
/// `__entries` twin (override stub).
pub fn replay_update(c: &mut Cpu) {
    use replay_callee::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let field = |off: u32| this.wrapping_add(off);
        c.g[V0][0] = c.lw(field(REPLAY_STATE));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64 ^ 1;
        let next = if c.g[V0][0] == 0 {
            // States 14/15: nothing to do.
            frame.restore(c, S0, 0x20);
            ReplayNext::Done
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(field(REPLAY_TIMER));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if !expired {
                c.cycles += 1;
                c.sw(field(REPLAY_TIMER), c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(field(REPLAY_F618));
            c.g[V0][0] = 7;
            c.sw(field(REPLAY_F604), 0);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 3;
                SET_STATE.call(c, 0x0026_f4fc)?;
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(field(REPLAY_STATE));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] == 0 {
                c.sw(field(REPLAY_F20), 0);
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(field(REPLAY_STATE));
            c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = 0x48_0000;
            if out_of_range {
                ReplayNext::CommandShort
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx(REPLAY_STATE_TABLE);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                match target {
                    0x0026_f538 => {
                        // 11: on to 12.
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xc;
                        SET_STATE.call(c, 0x0026_f544)?;
                        frame.restore(c, S0, 0x20);
                        ReplayNext::Done
                    }
                    0x0026_f54c => replay_instant(c, this)?,
                    0x0026_f57c => replay_record(c, this, frame)?,
                    0x0026_f708 => replay_playback(c, this)?,
                    0x0026_f74c => {
                        // 9: start playback from the top, state 1.
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 1;
                        BEGIN_PLAYBACK.call(c, 0x0026_f760)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 1;
                        SET_STATE.call(c, 0x0026_f76c)?;
                        c.g[V0][0] = 1;
                        c.sw(field(REPLAY_F61C), c.g[V0][0]);
                        ReplayNext::Out
                    }
                    0x0026_f778 | 0x0026_f78c => {
                        if target == 0x0026_f778 {
                            // 8: rewind first.
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0;
                            c.g[A2][0] = 0;
                            c.g[A3][0] = 0;
                            BEGIN_PLAYBACK.call(c, 0x0026_f78c)?;
                        }
                        // 4..7: back to playback (3).
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 3;
                        SET_STATE.call(c, 0x0026_f798)?;
                        frame.restore(c, S0, 0x20);
                        ReplayNext::Done
                    }
                    0x0026_f7a0 => ReplayNext::CommandShort,
                    0x0026_f7a8 => ReplayNext::Out,
                    _ => {
                        c.jump_out(target, 0x0026_f4a8, 0x0026_f7b8);
                        return Ok(());
                    }
                }
            }
        };
        match next {
            ReplayNext::Command => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                PROCESS_COMMAND.call(c, 0x0026_f744)?;
                frame.restore(c, S0, 0x20);
            }
            ReplayNext::CommandShort => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                PROCESS_COMMAND.call(c, 0x0026_f7a8)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            ReplayNext::Out => {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            ReplayNext::Done => {}
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// State 2 (instant replay): past the last recorded frame -> handle input
/// and process commands; else end the instant replay and record (state 0).
fn replay_instant(c: &mut Cpu, this: u32) -> Result<ReplayNext, crate::call::Unwound> {
    use replay_callee::*;
    c.cycles += 6;
    c.g[V0][0] = c.lw(this + REPLAY_FRAME);
    c.g[V1][0] = c.lw(this + REPLAY_LAST);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = 0;
        HANDLE_INPUT.call(c, 0x0026_f56c)?;
        return Ok(ReplayNext::Command);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    END_INSTANT_REPLAY.call(c, 0x0026_f57c)?;
    let frame = Frame { size: 0x30 };
    replay_record(c, this, frame)
}

/// Push one snapshot: `AddSnapshot(this, RecordSnapshot(this, {key, 0,
/// 0}))` with the 4-byte argument at `sp`.
fn replay_snapshot(c: &mut Cpu, key: u64, ret_record: u32, ret_add: u32) -> crate::call::Flow {
    use replay_callee::*;
    let sp_ = sp(c);
    c.g[A1][0] = c.g[SP][0];
    c.sh(sp_, key);
    c.sb(sp_ + 2, 0);
    c.sb(sp_ + 3, 0);
    RECORD_SNAPSHOT.call(c, ret_record)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    ADD_SNAPSHOT.call(c, ret_add)
}

/// States 0/12 (recording).
fn replay_record(c: &mut Cpu, this: u32, frame: Frame) -> Result<ReplayNext, crate::call::Unwound> {
    use replay_callee::*;
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    SET_STATE.call(c, 0x0026_f588)?;
    c.g[V0][0] = c.lw(this + REPLAY_F60C);
    c.sw(this + REPLAY_BLOCK, 0);
    if c.g[V0][0] != 0 {
        // A finished recording: close it with a key snapshot, state 13.
        c.cycles += 3;
        c.g[V0][0] = c.lw(this + REPLAY_F608);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
            return Ok(ReplayNext::Done);
        }
        c.cycles += 21;
        c.g[A0][0] = c.g[S0][0];
        CANCEL_INSTANT_REPLAY.call(c, 0x0026_f5a8)?;
        c.g[A0][0] = c.g[S0][0];
        CLEAR_ALL_HIGHLIGHTS.call(c, 0x0026_f5b0)?;
        c.g[A0][0] = c.g[S0][0];
        PRUNE_SNAPSHOTS.call(c, 0x0026_f5b8)?;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        replay_snapshot(c, 1, 0x0026_f5d4, 0x0026_f5e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xd;
        SET_STATE.call(c, 0x0026_f5ec)?;
        frame.restore(c, S0, 0x20);
        return Ok(ReplayNext::Done);
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(this + REPLAY_F624);
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        // A pending instant replay: start it (state 2).
        c.cycles += 11;
        c.g[A1][0] = c.lw(this + REPLAY_START);
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        BEGIN_PLAYBACK.call(c, 0x0026_f610)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        SET_STATE.call(c, 0x0026_f61c)?;
        c.g[V0][0] = 1;
        c.sw(this + REPLAY_F624, 0);
        c.sw(this + REPLAY_F620, c.g[V0][0]);
        return Ok(ReplayNext::Out);
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(this + REPLAY_PAUSED);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = c.lw(this + REPLAY_FRAME);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.lw(this + REPLAY_FRAME);
        c.g[V0][0] = 0x3c;
        c.cycles += 4;
        c.div(0, A0, V0);
        c.g[V1][0] = c.hi[0];
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.lw(this + REPLAY_PAUSED);
        } else {
            // Every 60 frames: prune and push a snapshot (a key one on
            // frame 0 and every keyRate frames).
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            PRUNE_SNAPSHOTS.call(c, 0x0026_f660)?;
            c.g[V0][0] = c.lw(this + REPLAY_FRAME);
            let key = if c.g[V0][0] == 0 {
                c.g[V0][0] = 1;
                true
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw(this + REPLAY_KEY_RATE);
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[S0][0];
                    false
                } else {
                    c.cycles += 3;
                    c.div(0, V0, V1);
                    c.cycles += 3;
                    c.g[V0][0] = c.hi[0];
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[S0][0];
                        false
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                        true
                    }
                }
            };
            if key {
                c.cycles += 11;
                c.g[A0][0] = c.g[S0][0];
                let k = c.g[V0][0];
                replay_snapshot(c, k, 0x0026_f6ac, 0x0026_f6b8)?;
            } else {
                c.cycles += 9;
                replay_snapshot(c, 0, 0x0026_f6d4, 0x0026_f6e0)?;
            }
            c.g[V0][0] = c.lw(this + REPLAY_PAUSED);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(this + REPLAY_FRAME);
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(this + REPLAY_SECONDS);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(this + REPLAY_SECONDS, c.g[V0][0]);
            c.g[V0][0] = c.lw(this + REPLAY_FRAME);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(this + REPLAY_FRAME, c.g[V0][0]);
    Ok(ReplayNext::Out)
}

/// State 3 (playback): the followed player's input binding may ask for a
/// skip (0x6b) -> handle input; then process commands.
fn replay_playback(c: &mut Cpu, this: u32) -> Result<ReplayNext, crate::call::Unwound> {
    use replay_callee::*;
    c.cycles += 7;
    c.g[V0][0] = c.lw(this + REPLAY_PLAYER);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = 0x6b;
        call_ptr(c, INPUT_EVAL_BOOL, 0x0026_f72c)?;
        let skip = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if skip {
            c.cycles += 2;
            c.g[A1][0] = 0;
            HANDLE_INPUT.call(c, 0x0026_f73c)?;
        }
    }
    Ok(ReplayNext::Command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, DATA};

    /// sprintf formats through the real vfprintf on short formats: plain
    /// text, an empty format, `%d` and `%s` (integer path, `_vfiprintf_r`).
    fn sprintf_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x0041_7828, 0);
        elf_image(c);
        let buf = DATA;
        let fmt = DATA + 0x100;
        let s = DATA + 0x180;
        const FORMATS: [&[u8]; 5] = [b"abc", b"", b"n=%d!", b"<%s>", b"%d,%s"];
        let f = FORMATS[seed as usize % FORMATS.len()];
        for (i, b) in f.iter().enumerate() {
            c.bus.write8(fmt + i as u32, *b);
        }
        c.bus.write8(fmt + f.len() as u32, 0);
        for (i, b) in b"snow\0".iter().enumerate() {
            c.bus.write8(s + i as u32, *b);
        }
        for i in 0..0x40 {
            c.bus.write8(buf + i, 0xaa);
        }
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = sx(fmt);
        match seed % FORMATS.len() as u32 {
            3 => c.g[A2][0] = sx(s),
            4 => {
                c.g[A2][0] = sx(seed.wrapping_mul(7919));
                c.g[A3][0] = sx(s);
            }
            _ => c.g[A2][0] = sx(seed.wrapping_mul(2654435761) >> 8),
        }
        c.pc = 0x0041_7828;
    }

    #[test]
    fn sprintf_matches() {
        check(ee::s15::f_00417828__generated, sprintf, 100, &sprintf_setup);
    }

    /// Two unpacked doubles covering every class pair (NaN, zero, normal,
    /// infinity, and an out-of-range negative class), both signs, and
    /// equal/greater/smaller exponents and fractions.
    #[test]
    fn fpcmp_parts_d_matches() {
        use crate::difftest::{scramble, Rng};
        check(ee::s15::f_004123e8__generated, fpcmp_parts_d, 400, &|c, seed| {
            let mut r = Rng(0x4123_e8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            let (a, b) = (DATA, DATA + 0x40);
            let classes = [0u32, 1, 2, 3, 3, 3, 4, 0xffff_fff0];
            c.bus.write32(a + FP_CLASS, classes[seed as usize % 8]);
            c.bus
                .write32(b + FP_CLASS, classes[(seed as usize / 8) % 8]);
            let pick = |r: &mut Rng, n: u32| r.next_u32() % n;
            let sa = pick(&mut r, 2);
            let sb = if pick(&mut r, 2) == 0 {
                sa
            } else {
                pick(&mut r, 2)
            };
            c.bus.write32(a + FP_SIGN, sa);
            c.bus.write32(b + FP_SIGN, sb);
            let ea = pick(&mut r, 5).wrapping_sub(2);
            let eb = if pick(&mut r, 2) == 0 {
                ea
            } else {
                pick(&mut r, 5).wrapping_sub(2)
            };
            c.bus.write32(a + FP_EXP, ea);
            c.bus.write32(b + FP_EXP, eb);
            let fa = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & 0xf;
            let fb = if pick(&mut r, 3) == 0 {
                fa
            } else {
                fa ^ (pick(&mut r, 4) as u64)
            };
            c.bus.write32(a + FP_FRAC, fa as u32);
            c.bus.write32(
                a + FP_FRAC + 4,
                (fa >> 32) as u32 | 0x8000_0000 * pick(&mut r, 2),
            );
            c.bus.write32(b + FP_FRAC, fb as u32);
            c.bus.write32(b + FP_FRAC + 4, (fb >> 32) as u32);
            c.g[A0][0] = sx(a);
            c.g[A1][0] = sx(b);
            c.pc = 0x0041_23e8;
        });
    }

    /// Centred, bottom and top alignment (align 0, positive, negative with
    /// the upper half set) over edge-case floats.
    #[test]
    fn hud_align_y_matches() {
        use crate::difftest::{scramble, Rng};
        check(ee::s05::f_0021e7a8__generated, hud_align_y, 300, &|c, seed| {
            let mut r = Rng(0x21e7a8 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.g[A1][0] = match seed % 4 {
                0 => 0,
                1 => 1 + (r.next_u32() & 0xff) as u64,
                2 => sx(0xffff_fffe),
                _ => 0x1_0000_0000,
            };
            c.f[12] = if seed % 3 == 0 {
                r.edge_f32()
            } else {
                r.f32(500.0)
            };
            c.f[13] = if seed % 5 == 0 {
                r.edge_f32()
            } else {
                r.f32(100.0)
            };
            c.f[14] = if seed % 7 == 0 {
                r.edge_f32()
            } else {
                r.f32(4.0)
            };
            c.pc = 0x0021_e7a8;
        });
    }

    /// A flag group of 1-6 columns by 1-4 rows in every style (static,
    /// animated, mirrored, pinned/folded, edge-only) with amplitudes of
    /// both signs (the ripples are skipped below the `.lit4` threshold),
    /// real `.lit4` constants and a sine table at 0x504fb8; the flag
    /// manager is reached through the world at gp-0x848.
    #[test]
    fn group_compute_verts_matches() {
        use crate::difftest::{scramble, Rng, GAME_GP};
        check(ee::s11::f_0034bca0__generated, group_compute_verts, 300, &|c, seed| {
            let mut r = Rng(0x34bca0 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            // World -> +0x84 -> +0x70 = flag manager (wind at +0x10).
            let (world, sub, mgr) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3200);
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, sub);
            c.bus.write32(sub + 0x70, mgr);
            c.bus.write32(mgr + 0x10, r.f32(1.0));
            // The wave table: 0x200 + 0x80 entries of sin(2*pi*i/512).
            for i in 0..0x280u32 {
                let v = (i as f32 * std::f32::consts::TAU / 512.0).sin();
                c.bus.write32(0x0050_4fb8 + 4 * i, v.to_bits());
            }
            let g = DATA;
            let style: [(u8, u8); 6] = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
            let (pinned, mirror) = style[seed as usize % 6];
            c.bus.write8(g + GROUP_PINNED, pinned);
            c.bus.write8(g + GROUP_MIRROR, mirror);
            c.bus.write8(g + GROUP_EDGE_ONLY, ((seed / 6) % 2) as u8);
            for off in (0x14..0x4c).step_by(4) {
                c.bus.write32(g + off, r.f32(3.0));
            }
            c.bus.write32(g + GROUP_WIND_BLEND, r.f32(1.0));
            let cols = 1 + (seed / 12) % 6;
            let rows = if seed % 37 == 0 {
                0
            } else {
                1 + (seed / 3) % 4
            };
            let cols = if seed % 41 == 0 { 0 } else { cols };
            c.bus.write32(g + GROUP_COLUMNS, cols);
            c.bus.write32(g + GROUP_ROWS, rows);
            c.bus
                .write32(g + GROUP_COLUMN_SPAN, ((cols.max(2) - 1) as f32).to_bits());
            c.bus
                .write32(g + GROUP_ROW_SPAN, ((rows.max(2) - 1) as f32).to_bits());
            let rest = DATA + 0x2000;
            c.bus.write32(g + GROUP_REST_VERTS, rest);
            for i in 0..(6 * 4 * 3) {
                c.bus.write32(rest + 4 * i, r.f32(50.0));
            }
            for off in (0x78..0x88).step_by(4) {
                c.bus.write32(g + off, r.f32(6.0));
            }
            c.g[A0][0] = sx(g);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.pc = 0x0034_bca0;
        });
    }

    /// Replay_Update with its real callees on their short paths (the
    /// game's jump tables from the ELF; a world whose player slot is empty;
    /// `+0x618` outside 1..8 except 7, so `ProcessCommand` only runs in
    /// state 3 where it is short). Covers: states 14/15, the timer, the
    /// "+0x618 == 7" drop to state 3, playback (3), 4-7, 10, 11, 13, and
    /// recording (0/12, also via 2 -> EndInstantReplay) with the finished
    /// and paused / not-on-a-60-frame-boundary paths. Not reached (their
    /// callees snapshot or step the whole world): the snapshot pushes,
    /// BeginPlayback (8, 9, +0x624), HandleInput in state 2, states >= 16,
    /// and a non-empty player slot.
    #[test]
    fn replay_update_matches() {
        use crate::difftest::{fill, scramble, Rng, GAME_GP};
        check(ee::s07::f_0026f4a8__generated, replay_update, 300, &|c, seed| {
            let mut r = Rng(0x26f4a8 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            fill(c, &mut r, this, 0x640 / 4);
            // World: player slot 0 (+0xb0) empty; the chain HandleInput
            // reads (+0x84 -> +0x48 -> +0x14).
            let (world, sub, sub2) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, sub);
            c.bus.write32(world + 0xb0, 0);
            c.bus.write32(sub + 0x48, sub2);
            c.bus.write32(sub2 + 0x14, 0);
            c.bus.write32(this + REPLAY_PLAYER, 0);
            const STATES: [u32; 13] = [14, 15, 3, 3, 4, 5, 6, 7, 11, 10, 13, 0, 12];
            let i = seed as usize % 16;
            let (state, f618) = match i {
                0..=12 => (STATES[i], if i == 3 { 7 } else { 0 }),
                13 => (2, 0),
                14 => (0, 0),
                _ => (12, 0x1_0000),
            };
            c.bus.write32(this + REPLAY_STATE, state);
            c.bus.write32(this + REPLAY_F618, f618);
            c.bus
                .write32(this + REPLAY_TIMER, (seed / 16 % 3).wrapping_sub(1));
            // State 2: never past the last frame (EndInstantReplay path).
            c.bus.write32(this + REPLAY_LAST, 0x7fff_ffff);
            c.bus.write32(this + REPLAY_F620, 0);
            // Recording variants.
            let rec = seed / 48 % 4;
            c.bus.write32(this + REPLAY_F60C, (rec == 0) as u32);
            c.bus.write32(this + REPLAY_F608, 1);
            c.bus.write32(this + REPLAY_F624, 0);
            c.bus.write32(this + REPLAY_PAUSED, (rec == 1) as u32);
            let frame = 60 * (seed % 7) + 1 + seed % 59;
            c.bus.write32(this + REPLAY_FRAME, frame);
            c.g[A0][0] = sx(this);
            c.pc = 0x0026_f4a8;
        });
    }
}
