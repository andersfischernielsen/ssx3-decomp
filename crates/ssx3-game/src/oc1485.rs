//! Staging module oc1485 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1485`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_db10, camera_is_equal, "tWPInGameData_Camera_isEqual"),
    (0x002b_de08, surface_is_equal, "tWPInGameData_Surface_isEqual"),
    (0x002b_dbd0, light_glow_is_equal, "tWPInGameData_LightGlow_isEqual"),
    (0x0037_ba50, ps2_graphics_man_set_shadow_char_matrices, "cPS2GraphicsMan_SetShadowCharMatrices"),
    (0x0039_04a0, ps2_graphics_man_render_color_filter, "PS2GraphicsMan_RenderColorFilter"),
    (0x003e_33b0, eafile_host_read_service, "EAFile_HostReadService"),
    (0x0037_a260, ps2_graphics_man_begin_tri_strip, "PS2GraphicsMan_BeginTriStrip"),
    (0x0037_a430, ps2_graphics_man_end_tri_strip, "PS2GraphicsMan_EndTriStrip"),
];

/// `c.lwc1 f<fd>, 0(addr)`: loads a float into FPU register `fd` and returns it.
#[inline]
fn load_fpr(c: &mut Cpu, fd: usize, addr: u32) -> FpuF32 {
    let v = Addr(addr).read_fpu(c);
    abi::set_fpr(c, fd, v);
    v
}

/// `c.eq.s a, b`: the compare result, latched in the FCC bit.
#[inline]
fn eq_s_latched(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let eq = a.eq_s(b);
    abi::set_fcc(c, eq);
    eq
}

/// `bool tWPInGameData_Camera_isEqual(a0, a1)` (0x002bdb10): the float at +0x8
/// of `a0` against the float at +0x4 of `a1` (`c.eq.s`); v0 = 1 when equal.
pub fn camera_is_equal(c: &mut Cpu) {
    c.cycles += 6;
    let ax = r32(c, A0).wrapping_add(0x8);
    let x = load_fpr(c, 1, ax);
    let ay = r32(c, A1).wrapping_add(0x4);
    let y = load_fpr(c, 0, ay);
    let eq = eq_s_latched(c, x, y);
    c.g[V0][0] = 0;
    if eq {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool tWPInGameData_Surface_isEqual(a0, a1)` (0x002bde08): the same compare as
/// [`camera_is_equal`] (+0x8 of `a0` against +0x4 of `a1`), a separate body in
/// the generated code.
pub fn surface_is_equal(c: &mut Cpu) {
    c.cycles += 6;
    let ax = r32(c, A0).wrapping_add(0x8);
    let x = load_fpr(c, 1, ax);
    let ay = r32(c, A1).wrapping_add(0x4);
    let y = load_fpr(c, 0, ay);
    let eq = eq_s_latched(c, x, y);
    c.g[V0][0] = 0;
    if eq {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool tWPInGameData_LightGlow_isEqual(a0, a1)` (0x002bdbd0): seven float pairs
/// must all compare equal (`c.eq.s`): `a0` at +0x8, +0x10, +0x18, +0x20, +0x28,
/// +0x30, +0x38 against `a1` at +0x4, +0x8, +0xc, +0x10, +0x14, +0x18, +0x1c.
/// Stops at the first pair that differs (v0 = 0); v0 = 1 when all match.
pub fn light_glow_is_equal(c: &mut Cpu) {
    const PAIRS: [(u32, u32); 7] = [
        (0x08, 0x04),
        (0x10, 0x08),
        (0x18, 0x0c),
        (0x20, 0x10),
        (0x28, 0x14),
        (0x30, 0x18),
        (0x38, 0x1c),
    ];
    // The `cycles += N` the generated body charges before each pair's loads.
    const CYCLES_BEFORE: [u64; 7] = [6, 4, 5, 5, 5, 6, 5];
    let a0 = r32(c, A0);
    let a1 = r32(c, A1);
    let mut all_equal = true;
    for (k, &(ao, bo)) in PAIRS.iter().enumerate() {
        c.cycles += CYCLES_BEFORE[k];
        let x = load_fpr(c, 1, a0.wrapping_add(ao));
        let y = load_fpr(c, 0, a1.wrapping_add(bo));
        if !eq_s_latched(c, x, y) {
            all_equal = false;
            break;
        }
    }
    c.g[V0][0] = all_equal as u64;
    c.cycles += 2;
    abi::ret(c);
}

/// `cPS2GraphicsMan_SetShadowCharMatrices` (0x0037ba50): leaf that writes a GS
/// packet header (the two `sd` words) into the packet at `gs->0x5a00`, copies the
/// 4 VU0 quads of `a1` and the quads of `a2`/`a3`, and patches the shadow matrix
/// fields of the table at `gs->0xe84`. Leaves every register the body leaves
/// (including the 128-bit `v1` from the last `lq`).
pub fn ps2_graphics_man_set_shadow_char_matrices(c: &mut Cpu) {
    c.cycles += 47;
    let a_5a00 = r32(c, A0).wrapping_add(0x5a00);
    c.g[T1][0] = c.lw(a_5a00);
    c.g[T0][0] = 0x3000_0000;
    c.g[V0][0] = 0x6000_0000;
    c.g[V0][0] |= 0x7;
    c.g[V1][0] = 0x8000;
    c.g[V1][0] <<= 45;
    c.g[T0][0] = c.g[T1][0] | c.g[T0][0];
    c.g[T2][0] = 0xd80c;
    c.g[T2][0] <<= 47;
    let t0 = r32(c, T0);
    let v0 = c.g[V0][0];
    c.sd(t0, v0);
    let v1 = c.g[V1][0];
    c.sd(t0.wrapping_add(0x10), v1);
    c.g[V0][0] = sx(t0.wrapping_add(0x20));
    let t2 = c.g[T2][0];
    c.sd(t0.wrapping_add(0x18), t2);
    c.sd(t0.wrapping_add(0x8), 0);
    let a1 = r32(c, A1);
    c.lqc2(1, a1);
    c.lqc2(2, a1.wrapping_add(0x10));
    c.lqc2(3, a1.wrapping_add(0x20));
    c.lqc2(4, a1.wrapping_add(0x30));
    let d0 = r32(c, V0);
    c.sqc2(1, d0);
    c.sqc2(2, d0.wrapping_add(0x10));
    c.sqc2(3, d0.wrapping_add(0x20));
    c.sqc2(4, d0.wrapping_add(0x30));
    let a2 = r32(c, A2);
    c.lq(V1, a2);
    c.g[A1][0] = 0x0fff_0000;
    c.g[A1][0] |= 0xffff;
    c.g[V0][0] = 0x3fff_0000;
    let t0_60 = r32(c, T0).wrapping_add(0x60);
    c.sq(V1, t0_60);
    c.g[V0][0] |= 0xffff;
    let mask_v0 = c.g[V0][0];
    c.g[T1][0] &= mask_v0;
    c.g[T2][0] = 0xffff_ffff_ffff_fc3f;
    let a3 = r32(c, A3);
    c.lq(V1, a3);
    c.g[T3][0] = 0xffff_ffff_c000_0000;
    let t0_70 = r32(c, T0).wrapping_add(0x70);
    c.sq(V1, t0_70);
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x80));
    let mask_a1 = c.g[A1][0];
    c.g[T0][0] &= mask_a1;
    let e84 = r32(c, A0).wrapping_add(0xe84);
    c.g[A2][0] = c.lw(e84);
    let t0_store = c.g[T0][0];
    c.sw(a_5a00, t0_store);
    let a2p = r32(c, A2);
    c.g[V0][0] = c.lw(a2p);
    let mask_t2 = c.g[T2][0];
    c.g[V0][0] &= mask_t2;
    c.g[V0][0] |= 0x1c0;
    let v0_store = c.g[V0][0];
    c.sw(a2p, v0_store);
    let e84b = r32(c, A0).wrapping_add(0xe84);
    c.g[V1][0] = c.lw(e84b);
    let a_v1c = r32(c, V1).wrapping_add(0xc);
    c.g[V0][0] = c.lw(a_v1c);
    let mask_t3 = c.g[T3][0];
    c.g[V0][0] &= mask_t3;
    let t1v = c.g[T1][0];
    c.g[V0][0] |= t1v;
    let v0_final = c.g[V0][0];
    c.sw(a_v1c, v0_final);
    abi::ret(c);
}

/// The compare chain of `PS2GraphicsMan_RenderColorFilter` (the blocks
/// `'b_390580`..`'b_390588` of the generated body): it compares the entry of
/// the colour-filter table (`gs + 0x6ca4 + 0x18 * i`) against 1.0 three times
/// and 0.0 three times. Ends in one of the two exits the body has.
enum FilterExit {
    /// Some compare failed: the body goes on to the stream and path calls.
    Calls,
    /// The last compare matched: straight to the epilogue.
    Done,
}

fn filter_chain(c: &mut Cpu) -> FilterExit {
    c.cycles += 10;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.mult(0, V0, S1, V0);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6ca4));
    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
    let entry = r32(c, V0);
    let one = FpuF32(c.f[1]);
    let x0 = load_fpr(c, 0, entry);
    let eq0 = eq_s_latched(c, x0, one);
    let gp_1384 = r32(c, GP).wrapping_add(0x1384);
    c.g[S0][0] = c.lw(gp_1384);
    if !eq0 {
        return filter_fall_580(c);
    }
    c.cycles += 5;
    let x1 = load_fpr(c, 0, entry.wrapping_add(0x4));
    let eq1 = eq_s_latched(c, x1, one);
    c.g[A3][0] = 0;
    if !eq1 {
        return filter_fall_584(c);
    }
    c.cycles += 5;
    let x2 = load_fpr(c, 0, entry.wrapping_add(0x8));
    let eq2 = eq_s_latched(c, x2, one);
    if !eq2 {
        let s3 = r32(c, S3);
        c.g[A1][0] = c.lw(s3);
        return FilterExit::Calls;
    }
    c.cycles += 6;
    let x3 = load_fpr(c, 0, entry.wrapping_add(0xc));
    c.mtc1(0, 1);
    let zero = FpuF32(c.f[1]);
    let eq3 = eq_s_latched(c, x3, zero);
    if !eq3 {
        let s3 = r32(c, S3);
        c.g[A1][0] = c.lw(s3);
        return FilterExit::Calls;
    }
    c.cycles += 5;
    let x4 = load_fpr(c, 0, entry.wrapping_add(0x10));
    let eq4 = eq_s_latched(c, x4, zero);
    if !eq4 {
        let s3 = r32(c, S3);
        c.g[A1][0] = c.lw(s3);
        return FilterExit::Calls;
    }
    c.cycles += 5;
    let x5 = load_fpr(c, 0, entry.wrapping_add(0x14));
    let eq5 = eq_s_latched(c, x5, zero);
    let sp_now = r32(c, SP);
    c.lq(S0, sp_now.wrapping_add(0x60));
    if eq5 {
        return FilterExit::Done;
    }
    c.cycles += 1;
    c.g[S0][0] = c.lw(gp_1384);
    filter_fall_580(c)
}

/// The fall-through from a failed compare in `'b_390580`: `A3 = 0`, then the
/// `'b_390584` exit.
fn filter_fall_580(c: &mut Cpu) -> FilterExit {
    c.cycles += 1;
    c.g[A3][0] = 0;
    filter_fall_584(c)
}

/// The `'b_390584` exit: `A1 = s3`, then the `'b_390588` exit into the calls.
fn filter_fall_584(c: &mut Cpu) -> FilterExit {
    c.cycles += 1;
    let s3 = r32(c, S3);
    c.g[A1][0] = c.lw(s3);
    FilterExit::Calls
}

/// The stream and path calls of `PS2GraphicsMan_RenderColorFilter` (the
/// `'b_390588` tail, depth 4 of the generated body).
fn filter_calls(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A2][0] = u64::MAX;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0038_f460,
        f: ee::s12::SprStream_BeginBlock,
    }
    .call(c, 0x0039_0594)?;
    c.g[A2][0] = c.g[SP][0];
    let sp = r32(c, SP);
    let v0 = c.g[V0][0];
    c.sw(sp, v0);
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S2][0];
    Callee {
        addr: 0x0039_05e8,
        f: ee::s12::PS2GraphicsMan_EmitColorFilterPacket,
    }
    .call(c, 0x0039_05a8)?;
    let sp = r32(c, SP);
    c.g[A1][0] = c.lw(sp);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = 4;
    Callee {
        addr: 0x0038_f668,
        f: ee::s12::SprStream_EndBlockAligned,
    }
    .call(c, 0x0039_05b8)?;
    let s3 = r32(c, S3);
    let v0 = c.g[V0][0];
    c.sw(s3, v0);
    c.g[A1][0] = c.g[S4][0];
    let a_18f4 = r32(c, S2).wrapping_add(0x18f4);
    c.g[A0][0] = c.lw(a_18f4);
    Callee {
        addr: 0x0036_8138,
        f: ee::s12::TexMan_EmitPath3Window,
    }
    .call(c, 0x0039_05c8)?;
    Ok(())
}

/// `PS2GraphicsMan_RenderColorFilter(gs, i, pp, path)` (0x003904a0): loads the
/// colour filter entry `i` of the table at `gs + 0x6ca4` (24-byte entries) and
/// compares it with 1.0 and 0.0; if the filter matches, it calls the stream and
/// path emitters (`ResetColorFilter` first when the global at `gp + 0x14ec` is set).
pub fn ps2_graphics_man_render_color_filter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
        let gp_14e8 = r32(c, GP).wrapping_add(0x14e8);
        c.g[V0][0] = c.lw(gp_14e8);
        let sp = r32(c, SP);
        c.sq(S1, sp.wrapping_add(0x50));
        c.sq(S2, sp.wrapping_add(0x40));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S3, sp.wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sq(S4, sp.wrapping_add(0x20));
        c.g[S3][0] = c.g[A3][0];
        c.sq(S0, sp.wrapping_add(0x60));
        c.g[S4][0] = c.g[A2][0];
        let initial_zero = c.g[V0][0] == 0;
        let ra = c.g[RA][0];
        c.sd(sp.wrapping_add(0x10), ra);

        let mut direct_exit = false;
        if !initial_zero {
            c.cycles += 3;
            let gp_14ec = r32(c, GP).wrapping_add(0x14ec);
            c.g[V0][0] = c.lw(gp_14ec);
            let reset_zero = c.g[V0][0] == 0;
            c.g[V0][0] = 0x18;
            if !reset_zero {
                c.cycles += 3;
                Callee {
                    addr: 0x0039_0458,
                    f: ee::s12::PS2GraphicsMan_ResetColorFilter,
                }
                .call(c, 0x0039_04e8)?;
                c.g[V0][0] = 0x18;
            }
            match filter_chain(c) {
                FilterExit::Done => direct_exit = true,
                FilterExit::Calls => filter_calls(c)?,
            }
        }
        if !direct_exit {
            // The `'b_3905c8` exit: one cycle, then restore s0.
            c.cycles += 1;
            c.lq(S0, sp.wrapping_add(0x60));
        }
        // Epilogue.
        c.cycles += 7;
        c.lq(S1, sp.wrapping_add(0x50));
        c.lq(S2, sp.wrapping_add(0x40));
        c.lq(S3, sp.wrapping_add(0x30));
        c.lq(S4, sp.wrapping_add(0x20));
        c.g[RA][0] = c.ld(sp.wrapping_add(0x10));
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x70));
        abi::ret(c);
        Ok(())
    })
}

/// `EAFile_HostReadService` (0x003e33b0): when the request global at 0x519c20 is
/// set, compares the stream position with the request's (0x450000 table), seeks
/// with `sceLseek` unless they match, reads with `sceRead` and completes the
/// request through `EAFile_CompleteRequest`.
pub fn eafile_host_read_service(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x50));
        c.g[V0][0] = 0x0052_0000;
        let sp = r32(c, SP);
        c.sq(S2, sp.wrapping_add(0x20));
        c.g[A1][0] = sx(r32(c, V0).wrapping_sub(0x63e0));
        let a_req = r32(c, V0).wrapping_sub(0x63e0);
        c.g[V1][0] = c.lw(a_req);
        c.g[S2][0] = c.g[V0][0];
        c.sq(S0, sp.wrapping_add(0x40));
        c.sq(S1, sp.wrapping_add(0x30));
        c.sq(S3, sp.wrapping_add(0x10));
        let v1_zero = c.g[V1][0] == 0;
        let ra = c.g[RA][0];
        c.sd(sp, ra);
        if !v1_zero {
            c.cycles += 6;
            c.g[V0][0] = 0x0045_0000;
            let a_4 = r32(c, A1).wrapping_add(0x4);
            c.g[A0][0] = c.lw(a_4);
            let a_c40 = r32(c, V0).wrapping_add(0xc40);
            c.g[V1][0] = c.lw(a_c40);
            c.g[S3][0] = c.g[V0][0];
            let pos_differs = c.g[V1][0] != c.g[A0][0];
            c.g[S1][0] = 0x0045_0000;
            // Whether the seek runs (the `'b_3e3404` tail).
            let mut seek = pos_differs;
            if !pos_differs {
                c.cycles += 4;
                let a_c4c = r32(c, S1).wrapping_add(0xc4c);
                c.g[V1][0] = c.lw(a_c4c);
                let a_c = r32(c, A1).wrapping_add(0xc);
                c.g[V0][0] = c.lw(a_c);
                let same_len = c.g[V1][0] == c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0x63e0));
                seek = !same_len;
            }
            if seek {
                c.cycles += 5;
                let a_4b = r32(c, A1).wrapping_add(0x4);
                c.g[A0][0] = c.lw(a_4b);
                c.g[A2][0] = 0;
                let a_c = r32(c, A1).wrapping_add(0xc);
                c.g[A1][0] = c.lw(a_c);
                c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0x63e0));
                call_ptr(c, 0x0042_8168, 0x003e_3418)?;
            }
            // The `'b_3e3418` exit.
            c.cycles += 16;
            let s0 = r32(c, S0);
            c.g[A0][0] = c.lw(s0.wrapping_add(0x4));
            c.g[A1][0] = c.lw(s0.wrapping_add(0x8));
            c.g[A2][0] = c.lw(s0.wrapping_add(0x10));
            call_ptr(c, 0x0042_83a0, 0x003e_342c)?;
            let s0 = r32(c, S0);
            c.g[A0][0] = c.lw(s0.wrapping_add(0x10));
            c.g[A2][0] = c.lw(s0.wrapping_add(0x4));
            c.g[V1][0] = c.lw(s0.wrapping_add(0xc));
            c.g[V0][0] ^= c.g[A0][0];
            c.g[A1][0] = c.lw(s0.wrapping_add(0x10));
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            let a_s3 = r32(c, S3).wrapping_add(0xc40);
            let a2v = c.g[A2][0];
            c.sw(a_s3, a2v);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
            let a_req0 = r32(c, S2).wrapping_sub(0x63e0);
            c.sw(a_req0, 0);
            let a_c4c = r32(c, S1).wrapping_add(0xc4c);
            let v1v = c.g[V1][0];
            c.sw(a_c4c, v1v);
            Callee {
                addr: 0x003d_e420,
                f: ee::s14::EAFile_CompleteRequest,
            }
            .call(c, 0x003e_3458)?;
        }
        // Epilogue (the `'b_3e3458` exit).
        c.cycles += 8;
        c.lq(S0, sp.wrapping_add(0x40));
        c.g[V0][0] = 0;
        c.lq(S1, sp.wrapping_add(0x30));
        c.lq(S2, sp.wrapping_add(0x20));
        c.lq(S3, sp.wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}

/// `cPS2GraphicsMan_BeginTriStrip(gfx, n)` (0x0037a260): starts a GS tri-strip
/// packet: updates the view matrices when the frame flag at `gp + 0x1290` is
/// clear, writes the packet header and the strip count into `gs`, and calls
/// `SprStream_BeginBlock` to open the block the packet goes into.
pub fn ps2_graphics_man_begin_tri_strip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x60));
        let gp_1290 = r32(c, GP).wrapping_add(0x1290);
        c.g[V0][0] = c.lw(gp_1290);
        let sp = r32(c, SP);
        c.sq(S0, sp.wrapping_add(0x50));
        c.sq(S1, sp.wrapping_add(0x40));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp.wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S3, sp.wrapping_add(0x20));
        c.sq(S4, sp.wrapping_add(0x10));
        let flag_zero = c.g[V0][0] == 0;
        let ra = c.g[RA][0];
        c.sd(sp, ra);
        if flag_zero {
            c.cycles += 2;
            c.g[V0][0] = sx(0x12b8);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx(0x12b0);
        }

        // 'b_37a2ec.
        c.cycles += 16;
        c.g[S3][0] = sx(r32(c, V0) >> 3);
        let e84 = r32(c, S0).wrapping_add(0xe84);
        c.g[A1][0] = c.lw(e84);
        c.g[V1][0] = 0xffff_ffff_ffff_fc3f;
        let flag_addr = r32(c, GP).wrapping_add(0x1290);
        c.sw(flag_addr, 0);
        c.g[A2][0] = 0xffff_ffff_c000_0000;
        let a1 = r32(c, A1);
        c.g[V0][0] = c.lw(a1);
        let mask_v1 = c.g[V1][0];
        c.g[V0][0] &= mask_v1;
        c.g[V0][0] |= 0xc0;
        let v0_store = c.g[V0][0];
        c.sw(a1, v0_store);
        let e84b = r32(c, S0).wrapping_add(0xe84);
        c.g[A0][0] = c.lw(e84b);
        let a0c = r32(c, A0).wrapping_add(0xc);
        c.g[V0][0] = c.lw(a0c);
        let mask_a2 = c.g[A2][0];
        c.g[V0][0] &= mask_a2;
        let v0_store2 = c.g[V0][0];
        c.sw(a0c, v0_store2);
        let s0_6b90 = r32(c, S0).wrapping_add(0x6b90);
        c.g[V1][0] = c.lw(s0_6b90);
        if c.g[V1][0] != 0 {
            let e84c = r32(c, S0).wrapping_add(0xe84);
            c.g[V0][0] = c.lw(e84c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0037_d968,
                f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
            }
            .call(c, 0x0037_a2e8)?;
            let e84d = r32(c, S0).wrapping_add(0xe84);
            c.g[V0][0] = c.lw(e84d);
        }

        // 'b_37a308.
        c.cycles += 3;
        let p = r32(c, V0);
        c.g[V1][0] = c.lh(p.wrapping_add(0x10));
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[S2][0] = 0x5c;
        c.cycles += 2;
        if negative {
            c.g[S2][0] = 0x4c;
            c.g[S4][0] = 0x41f;
        } else {
            c.g[S4][0] = 0x412;
        }

        // The main part.
        c.cycles += 74;
        let a_5a00 = r32(c, S0).wrapping_add(0x5a00);
        c.g[V0][0] = c.lw(a_5a00);
        c.g[A2][0] = u64::MAX;
        let a_1384 = r32(c, GP).wrapping_add(0x1384);
        c.g[A0][0] = c.lw(a_1384);
        c.g[A3][0] = 0;
        c.g[A1][0] = c.g[V0][0];
        let a_5a28 = r32(c, S0).wrapping_add(0x5a28);
        let v0_5a28 = c.g[V0][0];
        c.sw(a_5a28, v0_5a28);
        Callee {
            addr: 0x0038_f460,
            f: ee::s12::SprStream_BeginBlock,
        }
        .call(c, 0x0037_a324)?;
        c.g[A0][0] = sx(r32(c, S1) << 1);
        c.g[A1][0] = 0x1000_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
        let a_5a2c = r32(c, S0).wrapping_add(0x5a2c);
        let v0_block = c.g[V0][0];
        c.sw(a_5a2c, v0_block);
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x4));
        c.g[T0][0] = 3;
        c.g[V1][0] |= c.g[A1][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2));
        let pk = r32(c, V0);
        let v1_hdr = c.g[V1][0];
        c.sd(pk, v1_hdr);
        c.g[A0][0] = sx(r32(c, A0) << 16);
        c.g[V0][0] = 0x6c00_0000;
        c.mult(0, 0, S1, T0);
        let a_5a2c_b = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V1][0] = c.lw(a_5a2c_b);
        c.g[V0][0] |= 0x8000;
        let a0_or = c.g[V0][0];
        c.g[A0][0] |= a0_or;
        c.g[A1][0] = c.g[S2][0] << 47;
        let a_v1_8 = r32(c, V1).wrapping_add(0x8);
        c.sd(a_v1_8, 0);
        c.g[V0][0] = 0x8000;
        c.g[V0][0] <<= 31;
        c.g[V1][0] = c.g[S1][0] | 0x8000;
        let v0_bit = c.g[V0][0];
        c.g[A1][0] |= v0_bit;
        c.g[A1][0] = c.g[V1][0] | c.g[A1][0];
        c.g[V0][0] = 0xc000;
        c.g[V0][0] <<= 46;
        let v0_bit2 = c.g[V0][0];
        c.g[A1][0] |= v0_bit2;
        let a_5a2c_c = r32(c, S0).wrapping_add(0x5a2c);
        c.g[A2][0] = c.lw(a_5a2c_c);
        c.g[V0][0] = c.lo[0];
        c.g[T0][0] = c.hi[0];
        c.g[V0][0] <<= 32;
        let a_a2_10 = r32(c, A2).wrapping_add(0x10);
        c.sd(a_a2_10, 0);
        c.g[V0][0] >>= 32;
        c.g[T0][0] <<= 32;
        let v0_lo = c.g[V0][0];
        c.g[T0][0] |= v0_lo;
        c.g[A0][0] <<= 32;
        let a_5a2c_d = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V0][0] = c.lw(a_5a2c_d);
        c.g[A2][0] = 0x30;
        c.g[A3][0] = 0x1400_0000;
        let a_v0_18 = r32(c, V0).wrapping_add(0x18);
        let a0_store = c.g[A0][0];
        c.sd(a_v0_18, a0_store);
        c.mult(0, A2, S1, A2);
        let s3_or = c.g[S3][0] | c.g[A3][0];
        c.g[A3][0] = s3_or;
        c.lq(S1, sp.wrapping_add(0x40));
        let a_5a2c_e = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V0][0] = c.lw(a_5a2c_e);
        c.g[A3][0] <<= 32;
        c.lq(S2, sp.wrapping_add(0x30));
        let a_v0_20 = r32(c, V0).wrapping_add(0x20);
        let a1_store = c.g[A1][0];
        c.sd(a_v0_20, a1_store);
        c.lq(S3, sp.wrapping_add(0x20));
        let a_5a2c_f = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V1][0] = c.lw(a_5a2c_f);
        c.g[RA][0] = c.ld(sp);
        let a_v1_28 = r32(c, V1).wrapping_add(0x28);
        let s4_store = c.g[S4][0];
        c.sd(a_v1_28, s4_store);
        c.lq(S4, sp.wrapping_add(0x10));
        let a_5a2c_g = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V0][0] = c.lw(a_5a2c_g);
        let a_v0_30 = r32(c, V0).wrapping_add(0x30);
        let t0_store = c.g[T0][0];
        c.sd(a_v0_30, t0_store);
        let a_5a2c_h = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V1][0] = c.lw(a_5a2c_h);
        let a_v1_38 = r32(c, V1).wrapping_add(0x38);
        c.sd(a_v1_38, 0);
        let a_5a2c_i = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V0][0] = c.lw(a_5a2c_i);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x40));
        c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        let a_5a2c_j = r32(c, S0).wrapping_add(0x5a2c);
        let a2_store = c.g[A2][0];
        c.sw(a_5a2c_j, a2_store);
        let a_a2 = r32(c, A2);
        c.sd(a_a2, 0);
        let a_5a2c_k = r32(c, S0).wrapping_add(0x5a2c);
        c.g[A0][0] = c.lw(a_5a2c_k);
        let a_a0_8 = r32(c, A0).wrapping_add(0x8);
        let a3_store = c.g[A3][0];
        c.sd(a_a0_8, a3_store);
        let a_5a2c_l = r32(c, S0).wrapping_add(0x5a2c);
        c.g[V1][0] = c.lw(a_5a2c_l);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        let a_5a2c_m = r32(c, S0).wrapping_add(0x5a2c);
        let v1_store = c.g[V1][0];
        c.sw(a_5a2c_m, v1_store);
        c.lq(S0, sp.wrapping_add(0x50));
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x60));
        abi::ret(c);
        Ok(())
    })
}

/// `cPS2GraphicsMan_EndTriStrip(gfx)` (0x0037a430): closes the tri-strip block
/// (`SprStream_EndBlockAligned`) and, when the strip count is below 2600, writes
/// the packet tail into the GS packet buffer; then retires the strip counter.
pub fn ps2_graphics_man_end_tri_strip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[A2][0] = 4;
        let sp = r32(c, SP);
        c.sq(S0, sp.wrapping_add(0x10));
        let ra = c.g[RA][0];
        c.sd(sp, ra);
        c.g[S0][0] = c.g[A0][0];
        let a_1384 = r32(c, GP).wrapping_add(0x1384);
        c.g[A0][0] = c.lw(a_1384);
        let a_5a2c = r32(c, S0).wrapping_add(0x5a2c);
        c.g[A1][0] = c.lw(a_5a2c);
        Callee {
            addr: 0x0038_f668,
            f: ee::s12::SprStream_EndBlockAligned,
        }
        .call(c, 0x0037_a450)?;
        c.g[T1][0] = c.g[V0][0];
        let a_18f0 = r32(c, S0).wrapping_add(0x18f0);
        c.g[A3][0] = c.lw(a_18f0);
        let s0_5a00 = r32(c, S0).wrapping_add(0x5a00);
        let t1_store = c.g[T1][0];
        c.sw(s0_5a00, t1_store);
        let a_5a28 = r32(c, S0).wrapping_add(0x5a28);
        c.g[T2][0] = c.lw(a_5a28);
        let a3_load = r32(c, A3);
        c.g[V0][0] = c.lw(a3_load);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2600) as u64;
        let below = c.g[V0][0] != 0;
        let a_e84 = r32(c, S0).wrapping_add(0xe84);
        c.g[T0][0] = c.lw(a_e84);
        if below {
            c.cycles += 33;
            c.g[A1][0] = 0x0006_0000;
            let a_t0_8 = r32(c, T0).wrapping_add(0x8);
            c.g[A0][0] = c.lw(a_t0_8);
            c.g[A1][0] |= 0x8000;
            c.g[V1][0] = 0xffff_ffff_ffff_ffe0;
            c.g[A1][0] = sx(r32(c, A3).wrapping_add(r32(c, A1)));
            let mask_e0 = c.g[V1][0];
            c.g[A0][0] &= mask_e0;
            let a_a1_1cc4 = r32(c, A1).wrapping_add(0x1cc4);
            c.g[V0][0] = c.lw(a_a1_1cc4);
            c.g[A2][0] = sx(r32(c, A3).wrapping_add(0x80));
            c.g[V1][0] = 0x3000_0000;
            c.g[V0][0] &= 0x1f;
            let v1_mask = c.g[V1][0];
            c.g[A2][0] |= v1_mask;
            let v0_mask = c.g[V0][0];
            c.g[A0][0] |= v0_mask;
            let a_t0_8b = r32(c, T0).wrapping_add(0x8);
            let a0_store = c.g[A0][0];
            c.sw(a_t0_8b, a0_store);
            let a3_w = r32(c, A3);
            c.g[V1][0] = c.lw(a3_w);
            let a_a1_1cc0 = r32(c, A1).wrapping_add(0x1cc0);
            c.g[A0][0] = c.lw(a_a1_1cc0);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
            let a3_store = r32(c, A3);
            let v1_count = c.g[V1][0];
            c.sw(a3_store, v1_count);
            // Unaligned doubleword moves of the packet words (ldl/ldr, sdl/sdr).
            let t0 = r32(c, T0);
            c.ldl(V1, t0.wrapping_add(0x7));
            c.ldr(V1, t0);
            c.ldl(A1, t0.wrapping_add(0xf));
            c.ldr(A1, t0.wrapping_add(0x8));
            let a_t0_10 = r32(c, T0).wrapping_add(0x10);
            c.g[A2][0] = c.lw(a_t0_10);
            let v0p = r32(c, V0);
            c.sdl(V1, v0p.wrapping_add(0x7));
            c.sdr(V1, v0p);
            c.sdl(A1, v0p.wrapping_add(0xf));
            c.sdr(A1, v0p.wrapping_add(0x8));
            let a2_store = c.g[A2][0];
            c.sw(v0p.wrapping_add(0x10), a2_store);
            let t2_store = c.g[T2][0];
            c.sw(v0p.wrapping_add(0x14), t2_store);
            let t1_store2 = c.g[T1][0];
            c.sw(v0p.wrapping_add(0x18), t1_store2);
            let a0_h = c.g[A0][0];
            c.sh(v0p.wrapping_add(0x1c), a0_h);
            c.sh(v0p.wrapping_add(0x1e), 0);
        }
        // Common tail.
        c.cycles += 19;
        let a_5a00 = r32(c, S0).wrapping_add(0x5a00);
        c.g[V1][0] = c.lw(a_5a00);
        c.g[A1][0] = 0xffff_ffff_ffff_fc3f;
        let a_e84b = r32(c, S0).wrapping_add(0xe84);
        c.g[A0][0] = c.lw(a_e84b);
        c.g[A2][0] = 0xffff_ffff_c000_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp);
        let s0_5a00b = r32(c, S0).wrapping_add(0x5a00);
        let v1_5a00 = c.g[V1][0];
        c.sw(s0_5a00b, v1_5a00);
        let a0_word = r32(c, A0);
        c.g[V0][0] = c.lw(a0_word);
        let mask_a1 = c.g[A1][0];
        c.g[V0][0] &= mask_a1;
        let v0_a0 = c.g[V0][0];
        c.sw(a0_word, v0_a0);
        let a_e84c = r32(c, S0).wrapping_add(0xe84);
        c.g[V1][0] = c.lw(a_e84c);
        let a_v1c = r32(c, V1).wrapping_add(0xc);
        c.g[V0][0] = c.lw(a_v1c);
        let mask_a2 = c.g[A2][0];
        c.g[V0][0] &= mask_a2;
        let v0_v1c = c.g[V0][0];
        c.sw(a_v1c, v0_v1c);
        let s0_5a2c = r32(c, S0).wrapping_add(0x5a2c);
        c.sw(s0_5a2c, 0);
        let s0_5a28 = r32(c, S0).wrapping_add(0x5a28);
        c.sw(s0_5a28, 0);
        c.lq(S0, sp.wrapping_add(0x10));
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, check_paths, fakes, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's gated callee fakes, in the machine's RAM.
    const OC1485_CTRL: u32 = DATA + 0x6_e000;
    const OC1485_MAGIC: u32 = 0x1485_c0de;

    /// Gated fake of a callee slot: logs the call like `fakes::fake` only when
    /// this test's gate word is set, else runs the body the slot held before.
    macro_rules! oc1485_fake {
        ($name:ident, $prev:ident, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1485_CTRL) != OC1485_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                let ret = c.g[RA][0] as u32;
                fakes::fake(c, target, ret);
            }
        };
    }

    oc1485_fake!(oc1485_f_view, OC1485_P_VIEW, ee::s12::f_0037d968__generated);
    oc1485_fake!(oc1485_f_begin_block, OC1485_P_BEGIN, ee::s12::f_0038f460__generated);
    oc1485_fake!(oc1485_f_end_block, OC1485_P_END, ee::s12::f_0038f668__generated);
    oc1485_fake!(oc1485_f_reset_filter, OC1485_P_RESET, ee::s12::f_00390458__generated);
    oc1485_fake!(oc1485_f_emit_filter, OC1485_P_EMIT, ee::s12::f_003905e8__generated);
    oc1485_fake!(oc1485_f_path3, OC1485_P_PATH3, ee::s12::f_00368138__generated);
    oc1485_fake!(oc1485_f_complete, OC1485_P_COMPLETE, ee::s14::f_003de420__generated);
    oc1485_fake!(oc1485_f_lseek, OC1485_P_LSEEK, ee::s15::f_00428168__generated);
    oc1485_fake!(oc1485_f_read, OC1485_P_READ, ee::s15::f_004283a0__generated);

    fn oc1485_install() {
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_0037D968), oc1485_f_view, &OC1485_P_VIEW);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_0038F460), oc1485_f_begin_block, &OC1485_P_BEGIN);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_0038F668), oc1485_f_end_block, &OC1485_P_END);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00390458), oc1485_f_reset_filter, &OC1485_P_RESET);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_003905E8), oc1485_f_emit_filter, &OC1485_P_EMIT);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00368138), oc1485_f_path3, &OC1485_P_PATH3);
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DE420), oc1485_f_complete, &OC1485_P_COMPLETE);
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_00428168), oc1485_f_lseek, &OC1485_P_LSEEK);
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_004283A0), oc1485_f_read, &OC1485_P_READ);
    }

    fn oc1485_gate(c: &mut Cpu) {
        c.bus.write32(OC1485_CTRL, OC1485_MAGIC);
    }

    /// Two floats: `x` at `a0 + 8` and `y` at `a1 + 4`; equal for a third of the
    /// seeds, one ulp apart for another third, unrelated for the rest.
    fn oc1485_pair_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1485_0b10 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        let x = r.f32(4.0);
        let y = match seed % 3 {
            0 => x,
            1 => x ^ 1,
            _ => r.f32(4.0),
        };
        c.bus.write32(DATA + 0x1008, x);
        c.bus.write32(DATA + 0x2004, y);
    }

    /// Seven float pairs; the first `seed % 8` pairs match, the next one differs
    /// (`seed % 8 == 7`: all match).
    fn oc1485_glow_setup(c: &mut Cpu, seed: u32) {
        const AO: [u32; 7] = [0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38];
        const BO: [u32; 7] = [0x04, 0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c];
        let mut r = Rng(0x1485_2bd0 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        let fail = seed % 8;
        for j in 0..7u32 {
            let x = r.f32(4.0);
            let y = if j < fail {
                x
            } else if j == fail {
                x ^ 1
            } else {
                r.f32(4.0)
            };
            c.bus.write32(DATA + 0x1000 + AO[j as usize], x);
            c.bus.write32(DATA + 0x2000 + BO[j as usize], y);
        }
    }

    /// The shadow-matrix fixture: a GS block with its packet pointer at +0x5a00
    /// and the table pointer at +0xe84; the VU0 quads and the packet quads.
    fn oc1485_shadow_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1485_ba50 ^ seed as u64);
        scramble(c, &mut r);
        let gs = DATA + 0x1_0000;
        c.g[A0][0] = sx(gs);
        c.g[A1][0] = sx(DATA + 0x1_2000);
        c.g[A2][0] = sx(DATA + 0x1_4000);
        c.g[A3][0] = sx(DATA + 0x1_6000);
        fill(c, &mut r, DATA + 0x1_2000, 16);
        fill(c, &mut r, DATA + 0x1_4000, 4);
        fill(c, &mut r, DATA + 0x1_6000, 4);
        c.bus.write32(gs + 0x5a00, DATA + 0x1_8000 + (seed % 4) * 0x100);
        c.bus.write32(gs + 0xe84, DATA + 0x1_a000);
        fill(c, &mut r, DATA + 0x1_a000, 8);
    }

    /// The colour filter fixture: the branch selectors (`gp+0x14e8`,
    /// `gp+0x14ec`), the table entry `i` (its six floats match 1.0, 1.0, 1.0,
    /// 0.0, 0.0, 0.0 up to the `fail` index) and the path pointers.
    fn oc1485_filter_setup(c: &mut Cpu, seed: u32, calls: u32) {
        oc1485_install();
        let mut r = fakes::machine(c, seed, 0x1485_3904, calls);
        oc1485_gate(c);
        let gp = GAME_GP;
        let gs = DATA + 0x1_0000;
        c.bus.write32(gp + 0x14e8, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(gp + 0x14ec, if seed % 3 == 0 { 0 } else { 1 });
        c.bus.write32(gp + 0x1384, DATA + 0x2_2000);
        let i = (seed / 3) % 3;
        c.g[A0][0] = sx(gs);
        c.g[A1][0] = i as u64;
        c.g[A2][0] = 7;
        c.g[A3][0] = sx(DATA + 0x1_8000);
        let fail = (seed / 12) % 7;
        let entry = gs + 0x6ca4 + 0x18 * i;
        for j in 0..6u32 {
            let good = if j < 3 { 1.0f32 } else { 0.0f32 };
            let bad = if j < 3 { 2.0f32 } else { 0.5f32 };
            let v = if j < fail {
                good.to_bits()
            } else if j == fail {
                bad.to_bits()
            } else {
                r.next_u32()
            };
            c.bus.write32(entry + 4 * j, v);
        }
        c.bus.write32(gs + 0x18f4, DATA + 0x2_0000);
        c.bus.write32(DATA + 0x1_8000, r.next_u32());
        fill(c, &mut r, DATA + 0x2_0000, 8);
    }

    /// The host-read fixture: the request global at 0x519c20 (non-zero for the
    /// main path), the request's position and length words, and the stream
    /// globals at 0x450c40 / 0x450c4c.
    fn oc1485_read_setup(c: &mut Cpu, seed: u32, calls: u32) {
        oc1485_install();
        let mut r = fakes::machine(c, seed, 0x1485_3e33, calls);
        oc1485_gate(c);
        c.bus.write32(0x0051_9c20, if seed % 5 == 0 { 0 } else { 1 });
        let g = r.next_u32();
        c.bus.write32(0x0045_0c40, g);
        c.bus.write32(0x0051_9c24, if seed % 2 == 0 { g } else { g.wrapping_add(1) });
        let x = r.next_u32();
        c.bus.write32(0x0045_0c4c, x);
        c.bus.write32(0x0051_9c2c, if seed % 3 == 0 { x } else { x.wrapping_add(1) });
        c.bus.write32(0x0051_9c28, r.next_u32());
        c.bus.write32(0x0051_9c30, r.next_u32());
    }

    /// The tri-strip fixture: the GS block with its frame flag (`gp+0x1290`), the
    /// packet pointers, the table pointer (+0xe84) and the strip count.
    fn oc1485_begin_setup(c: &mut Cpu, seed: u32, calls: u32) {
        oc1485_install();
        let mut r = fakes::machine(c, seed, 0x1485_a260, calls);
        oc1485_gate(c);
        let gs = DATA + 0x1_0000;
        let p = DATA + 0x2_0000;
        c.g[A0][0] = sx(gs);
        c.g[A1][0] = (seed % 5) as u64;
        c.bus.write32(GAME_GP + 0x1290, if seed % 2 == 0 { 0 } else { 0x55 });
        c.bus.write32(GAME_GP + 0x1384, DATA + 0x4_0000);
        c.bus.write32(gs + 0x5a00, DATA + 0x3_0000 + 0x40 * (seed % 4));
        c.bus.write32(gs + 0x6b90, if seed % 4 == 0 { 0 } else { 7 });
        c.bus.write32(gs + 0xe84, p);
        fill(c, &mut r, p, 16);
        c.bus.write32(p + 0x10, if seed % 3 == 0 { 0xffff_8000 } else { 0x0000_7fff });
    }

    /// The tri-strip end fixture: the strip count below or above 2600, the packet
    /// pointer at +0xe84, the unaligned-store source.
    fn oc1485_end_setup(c: &mut Cpu, seed: u32, calls: u32) {
        oc1485_install();
        let mut r = fakes::machine(c, seed, 0x1485_a430, calls);
        oc1485_gate(c);
        let gs = DATA + 0x1_0000;
        let a3 = DATA + 0x4_0000;
        let p = DATA + 0x2_0000;
        c.g[A0][0] = sx(gs);
        c.bus.write32(GAME_GP + 0x1384, DATA + 0x5_0000);
        c.bus.write32(gs + 0x5a2c, DATA + 0x5_0000);
        c.bus.write32(gs + 0x5a28, r.next_u32());
        c.bus.write32(gs + 0x18f0, a3);
        let w = if seed % 2 == 0 { seed * 7 % 2600 } else { 2600 + seed % 100 };
        c.bus.write32(a3, w);
        c.bus.write32(gs + 0xe84, p);
        fill(c, &mut r, p, 16);
        fill(c, &mut r, DATA + 0xa_8000 + 0x1cc0, 2);
    }

    #[test]
    fn camera_is_equal_matches() {
        check(ee::s08::f_002bdb10__generated, camera_is_equal, 200, &|c, seed| {
            oc1485_pair_setup(c, seed)
        });
    }

    #[test]
    fn surface_is_equal_matches() {
        check(ee::s08::f_002bde08__generated, surface_is_equal, 200, &|c, seed| {
            oc1485_pair_setup(c, seed)
        });
    }

    #[test]
    fn light_glow_is_equal_matches() {
        check_paths(
            ee::s08::f_002bdbd0__generated,
            light_glow_is_equal,
            200,
            &[8, 12, 17, 22, 27, 33, 38],
            &|c, seed| oc1485_glow_setup(c, seed),
        );
    }

    #[test]
    fn shadow_char_matrices_matches() {
        check(
            ee::s12::f_0037ba50__generated,
            ps2_graphics_man_set_shadow_char_matrices,
            200,
            &|c, seed| oc1485_shadow_setup(c, seed),
        );
    }

    #[test]
    fn render_color_filter_matches() {
        check_lohi(
            ee::s12::f_003904a0__generated,
            ps2_graphics_man_render_color_filter,
            300,
            &|c, seed| oc1485_filter_setup(c, seed, 0),
        );
    }

    #[test]
    fn render_color_filter_unwinds() {
        fakes::covered(
            ee::s12::f_003904a0__generated,
            ps2_graphics_man_render_color_filter,
            300,
            &|c, seed| oc1485_filter_setup(c, seed, 5),
        );
    }

    #[test]
    fn render_color_filter_reaches_every_call() {
        fakes::check_reaching(
            ee::s12::f_003904a0__generated,
            ps2_graphics_man_render_color_filter,
            300,
            &[0x0039_04e8, 0x0039_0594, 0x0039_05a8, 0x0039_05b8, 0x0039_05c8],
            &|c, seed| oc1485_filter_setup(c, seed, 0),
        );
    }

    #[test]
    fn host_read_service_matches() {
        check(
            ee::s14::f_003e33b0__generated,
            eafile_host_read_service,
            300,
            &|c, seed| oc1485_read_setup(c, seed, 0),
        );
    }

    #[test]
    fn host_read_service_unwinds() {
        fakes::covered(
            ee::s14::f_003e33b0__generated,
            eafile_host_read_service,
            300,
            &|c, seed| oc1485_read_setup(c, seed, 3),
        );
    }

    #[test]
    fn host_read_service_reaches_every_call() {
        fakes::check_reaching(
            ee::s14::f_003e33b0__generated,
            eafile_host_read_service,
            300,
            &[0x003e_3418, 0x003e_342c, 0x003e_3458],
            &|c, seed| oc1485_read_setup(c, seed, 0),
        );
    }

    #[test]
    fn begin_tri_strip_matches() {
        fakes::covered(
            ee::s12::f_0037a260__generated,
            ps2_graphics_man_begin_tri_strip,
            300,
            &|c, seed| oc1485_begin_setup(c, seed, 2),
        );
    }

    #[test]
    fn begin_tri_strip_reaches_every_call() {
        fakes::check_reaching(
            ee::s12::f_0037a260__generated,
            ps2_graphics_man_begin_tri_strip,
            300,
            &[0x0037_a2e8, 0x0037_a324],
            &|c, seed| oc1485_begin_setup(c, seed, 0),
        );
    }

    #[test]
    fn end_tri_strip_matches() {
        fakes::covered(
            ee::s12::f_0037a430__generated,
            ps2_graphics_man_end_tri_strip,
            300,
            &|c, seed| oc1485_end_setup(c, seed, 1),
        );
    }

    #[test]
    fn end_tri_strip_reaches_every_call() {
        fakes::check_reaching(
            ee::s12::f_0037a430__generated,
            ps2_graphics_man_end_tri_strip,
            300,
            &[0x0037_a450],
            &|c, seed| oc1485_end_setup(c, seed, 0),
        );
    }
}
