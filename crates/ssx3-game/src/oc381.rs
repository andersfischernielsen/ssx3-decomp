//! Staging module oc381 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc381`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001a_2208, fe_module_render, "FEModule_Render")];

// ------------------------------------------------------------ FEModule_Render

/// `gp`-relative `cPS2GraphicsMan* GrMan`.
pub(crate) const FE_MODULE_RENDER_GRMAN_GP: u32 = 0x854;
/// `gp`-relative `D_004A19CC`: draw a black backdrop instead of the module.
pub(crate) const FE_MODULE_RENDER_BACKDROP_GP: u32 = 0x1724;
/// `gp`-relative literal 0.02f (the audio update step).
pub(crate) const FE_MODULE_RENDER_AUDIO_STEP_GP: u32 = 0x5970;
/// GrMan: top of the graphics-state stack (`sFeGs*`, 0x14 bytes each).
pub(crate) const FE_MODULE_RENDER_GS_TOP: u32 = 0xe84;
/// GrMan: vtable (`{short delta; short; fn}` entries, 8 bytes).
pub(crate) const FE_MODULE_RENDER_VTABLE: u32 = 0x10d8;
/// FEModule: fade-out counter (`self + 0xb5adc`).
pub(crate) const FE_MODULE_RENDER_FADE: u32 = 0x5adc;
/// FEModule: the playing video (`self + 0xb5ad4`).
pub(crate) const FE_MODULE_RENDER_VIDEO: u32 = 0x5ad4;
/// The module's default graphics state `D_00501420`.
pub(crate) const FE_MODULE_RENDER_DEFAULT_GS: u32 = 0x0050_1420;

/// `rt = GrMan` (`lw rt, -0x854(gp)`).
fn fe_render_grman(c: &mut Cpu, rt: usize) {
    let gp = r32(c, GP);
    c.g[rt][0] = Addr(gp.wrapping_sub(FE_MODULE_RENDER_GRMAN_GP)).lw(c);
}

/// `vt = GrMan->vtable`, with GrMan in `this`.
fn fe_render_vt(c: &mut Cpu, this: usize, vt: usize) {
    let t = r32(c, this);
    c.g[vt][0] = Addr(t.wrapping_add(FE_MODULE_RENDER_VTABLE)).lw(c);
}

/// A GrMan virtual call through the vtable entry at `off`: `delta`/`func`
/// get the entry's halves, `a0 = this + delta`, `jalr func` back to `ret`.
fn fe_render_vcall(
    c: &mut Cpu,
    this: usize,
    vt: usize,
    delta: usize,
    func: usize,
    off: u32,
    ret: u32,
) -> Flow {
    let obj = c.g[this][0];
    let entry = r32(c, vt).wrapping_add(off);
    c.g[delta][0] = Addr(entry).lh(c);
    c.g[func][0] = Addr(entry.wrapping_add(4)).lw(c);
    let target = r32(c, func);
    c.g[A0][0] = crate::abi::addu(obj, c.g[delta][0]);
    call_ptr(c, target, ret)
}

/// Unsigned int to float as gcc emits it into `fd`: `cvt.s.w` of the value,
/// or of `(v >> 1) | (v & 1)` doubled when the sign bit is set.
fn fe_render_u2f(c: &mut Cpu, fd: usize) {
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] &= 0x1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, fd);
        c.cvt_s_w(fd, fd);
        c.add_s(fd, fd, fd);
    } else {
        c.cycles += 4;
        c.mtc1(V0, fd);
        c.cvt_s_w(fd, fd);
    }
}

/// Copy the 0x14-byte graphics state at `src` to `dst` with the
/// unaligned `ldl/ldr` + `sdl/sdr` pairs, through `ra`/`rb`/`rw`.
fn fe_render_copy_gs(c: &mut Cpu, src: u32, dst: u32, ra: usize, rb: usize, rw: usize) {
    c.ldl(ra, src.wrapping_add(0x7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(0x8));
    c.g[rw][0] = Addr(src.wrapping_add(0x10)).lw(c);
    c.sdl(ra, dst.wrapping_add(0x7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(0x8));
    Addr(dst.wrapping_add(0x10)).sw(c, c.g[rw][0]);
}

/// `int FEModule_Render(FEModule* self)` (0x1a2208): the front end's frame
/// render. Nothing (0) while the graphics manager is not ready (vslot 17).
/// Counts the fade-out timer down; draws the fade (`AppModule_RenderFade`)
/// or the playing video and ends the frame (vslot 20) if either is active.
/// With `D_004A19CC` set it draws a black full-screen quad with an
/// orthographic view instead. Otherwise: camera (`self+0x10..0x18`) and view
/// matrix (`self+0x50`), the module's default graphics state pushed, the UI
/// state manager, the rider previews, the UI engine, the end of the frame and
/// the audio update (0.02 s). Returns 1.
pub fn fe_module_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x110);
        fe_render_grman(c, A1);
        frame.save(c, S1, 0xe0);
        frame.save(c, S0, 0xf0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        let this = r32(c, S1);
        // if (!GrMan->vslot17()) return 0;
        fe_render_vt(c, A1, V1);
        fe_render_vcall(c, A1, V1, A0, V0, 0x88, 0x001a_223c)?;
        let ready = c.g[V0][0] != 0;
        c.g[V0][0] = 0xb0000;
        if !ready {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return fe_render_epilogue(c, frame);
        }

        // Fade-out counter.
        c.cycles += 4;
        c.g[S0][0] = sx(this.wrapping_add(0xb0000));
        let hi = r32(c, S0);
        let fade = Addr(hi.wrapping_add(FE_MODULE_RENDER_FADE)).lw(c);
        c.g[V0][0] = sx((fade as u32).wrapping_sub(1));
        if fade != 0 {
            c.cycles += 1;
            Addr(hi.wrapping_add(FE_MODULE_RENDER_FADE)).sw(c, c.g[V0][0]);
        }

        // AppModule_RenderFade(self): end the frame.
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0023_1d60, 0x001a_2268)?;
        let fading = c.g[V0][0] != 0;
        fe_render_grman(c, A1);
        if fading {
            c.cycles += 7;
            fe_render_vt(c, A1, V0);
            fe_render_vcall(c, A1, V0, A0, V1, 0xa0, 0x001a_2284)?;
            c.g[V0][0] = 0x1;
            return fe_render_epilogue(c, frame);
        }

        // A playing video: VidEnginePlayer_Render(video, 0), end the frame.
        c.cycles += 3;
        let hi = r32(c, S0);
        c.g[A0][0] = Addr(hi.wrapping_add(FE_MODULE_RENDER_VIDEO)).lw(c);
        let video = c.g[A0][0] != 0;
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_sub(FE_MODULE_RENDER_BACKDROP_GP)).lw(c);
        if video {
            c.cycles += 10;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0025_3890, 0x001a_22a0)?;
            fe_render_grman(c, A1);
            fe_render_vt(c, A1, V0);
            fe_render_vcall(c, A1, V0, A0, V1, 0xa0, 0x001a_22b8)?;
            c.g[V0][0] = 0x1;
            return fe_render_epilogue(c, frame);
        }

        c.cycles += 2;
        let backdrop = c.g[V0][0] != 0;
        fe_render_grman(c, A1);
        if backdrop {
            fe_render_backdrop(c)?;
            c.g[V0][0] = 0x1;
            return fe_render_epilogue(c, frame);
        }

        // The module: begin the frame (vslots 21, 31), camera, view.
        c.cycles += 132;
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xa8, 0x001a_2584)?;
        fe_render_grman(c, A1);
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xf8, 0x001a_259c)?;
        fe_render_grman(c, A1);
        let this = r32(c, S1);
        c.lwc1(12, this.wrapping_add(0x10));
        c.lwc1(13, this.wrapping_add(0x14));
        c.lwc1(14, this.wrapping_add(0x18));
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xc8, 0x001a_25c0)?;
        fe_render_grman(c, A2);
        let this = r32(c, S1);
        c.g[A1][0] = sx(this.wrapping_add(0x50));
        fe_render_vt(c, A2, V0);
        fe_render_vcall(c, A2, V0, A0, V1, 0x110, 0x001a_25dc)?;

        // Push a copy of the top graphics state, then load the module's
        // default state and set its fields.
        fe_render_grman(c, A0);
        let grman = r32(c, A0);
        c.g[T2][0] = 0x500000;
        c.g[A2][0] = 0xffff_ffff_ffbf_0000 | 0xffff;
        c.g[T0][0] = 0xffff_ffff_fe7f_0000 | 0xffff;
        c.g[T3][0] = 0x800000;
        let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
        c.g[V0][0] = top;
        fe_render_copy_gs(c, top as u32, (top as u32).wrapping_add(0x14), T7, V1, A1);
        c.g[T4][0] = 0x300000;
        c.g[A3][0] = 0xffff_ffff_ffcf_0000 | 0xffff;
        c.g[T1][0] = 0xffff_ffff_fff0_0000 | 0xfff;
        let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
        fe_render_grman(c, V1);
        c.g[V0][0] = sx((top as u32).wrapping_add(0x14));
        c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
        Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).sw(c, c.g[V0][0]);
        c.g[T6][0] = 0xffff_ffff_ffff_ff83;
        let grman_v1 = r32(c, V1);
        c.g[V0][0] = Addr(grman_v1.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
        c.g[A1][0] = sx(FE_MODULE_RENDER_DEFAULT_GS);
        let dst = r32(c, V0);
        fe_render_copy_gs(c, FE_MODULE_RENDER_DEFAULT_GS, dst, T7, V1, A0);
        fe_render_grman(c, A1);
        // g22 = 0; g23 = 1; g20 = 3; g12 = 0xd; alpha = 0x15; g2 = 5.
        let masks = [
            (V1, 0x4, A2, None),
            (A0, 0x4, T0, Some(T3)),
            (V1, 0x4, A3, Some(T4)),
            (A0, 0x4, T1, None),
            (V1, 0x8, T5, None),
            (A0, 0x4, T6, None),
        ];
        let ors = [0, 0, 0, 0xd000, 0x2a0, 0x14];
        for (&(reg, off, mask, or_reg), &or_imm) in masks.iter().zip(ors.iter()) {
            let gm = r32(c, A1);
            c.g[reg][0] = Addr(gm.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
            let gs = r32(c, reg);
            c.g[V0][0] = Addr(gs.wrapping_add(off)).lw(c);
            c.g[V0][0] &= c.g[mask][0];
            if let Some(o) = or_reg {
                c.g[V0][0] |= c.g[o][0];
            }
            c.g[V0][0] |= or_imm;
            Addr(gs.wrapping_add(off)).sw(c, c.g[V0][0]);
        }

        // UIEngine_RenderStateMgr(self->ui); the rider previews.
        let this = r32(c, S1);
        c.g[A0][0] = Addr(this.wrapping_add(0xc)).lw(c);
        call_ptr(c, 0x0039_8018, 0x001a_2700)?;
        let this = r32(c, S1);
        c.g[A0][0] = sx(this.wrapping_add(0xb0));
        Callee {
            addr: 0x001a_0598,
            f: ee::s03::FEModule_RenderRiderPreviews,
        }
        .call(c, 0x001a_2708)?;

        // Pop the state, end the frame (vslots 32, 22).
        fe_render_pop(c);
        fe_render_vcall(c, A0, A1, V1, V0, 0x100, 0x001a_272c)?;
        fe_render_grman(c, A1);
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xb0, 0x001a_2744)?;

        // UIEngine_Render(self->ui), vslot 20, the audio update.
        let this = r32(c, S1);
        c.g[A0][0] = Addr(this.wrapping_add(0xc)).lw(c);
        call_ptr(c, 0x0039_7df8, 0x001a_274c)?;
        fe_render_grman(c, A1);
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xa0, 0x001a_2764)?;
        call_ptr(c, 0x0028_b180, 0x001a_276c)?;
        let gp = r32(c, GP);
        c.lwc1(12, gp.wrapping_sub(FE_MODULE_RENDER_AUDIO_STEP_GP));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_5bf8, 0x001a_277c)?;
        c.g[V0][0] = 0x1;
        fe_render_epilogue(c, frame)
    })
}

/// `GrMan->top--` with GrMan in `a0` and its vtable in `a1`.
fn fe_render_pop(c: &mut Cpu) {
    fe_render_grman(c, A0);
    let grman = r32(c, A0);
    let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    fe_render_vt(c, A0, A1);
    c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
    Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).sw(c, c.g[V0][0]);
}

/// The `D_004A19CC` path of [`fe_module_render`] (GrMan in `a1`): screen
/// size (vslots 10, 11), begin the frame, push the state, an orthographic
/// view of the screen with the identity matrix, alpha mode 9, a black
/// 4-vertex strip over the screen (vslot 71), end the frame, pop, vslot 20.
fn fe_render_backdrop(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    fe_render_vt(c, A1, V1);
    fe_render_vcall(c, A1, V1, A0, V0, 0x50, 0x001a_22dc)?;
    fe_render_grman(c, A1);
    fe_render_u2f(c, 21);
    fe_render_vt(c, A1, V1);
    c.cycles += 6;
    fe_render_vcall(c, A1, V1, A0, V0, 0x58, 0x001a_2320)?;
    fe_render_grman(c, A1);
    fe_render_u2f(c, 20);
    fe_render_vt(c, A1, V0);
    c.cycles += 57;
    fe_render_vcall(c, A1, V0, A0, V1, 0xa8, 0x001a_2364)?;
    fe_render_grman(c, A1);
    fe_render_vt(c, A1, V0);
    fe_render_vcall(c, A1, V0, A0, V1, 0xf8, 0x001a_237c)?;

    // Push: top[1] = top[0]; top++. Orthographic(0, 0, 0, 0, w, h, 0, 1).
    fe_render_grman(c, A0);
    let grman = r32(c, A0);
    c.g[A1][0] = 0;
    c.mtc1(0, 12);
    c.g[A2][0] = 0;
    let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    c.g[V0][0] = top;
    c.mov_s(15, 20);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 17);
    c.mov_s(14, 21);
    fe_render_copy_gs(c, top as u32, (top as u32).wrapping_add(0x14), V1, A3, T0);
    c.mov_s(16, 12);
    c.mov_s(13, 12);
    let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    fe_render_grman(c, A3);
    c.g[V0][0] = sx((top as u32).wrapping_add(0x14));
    Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).sw(c, c.g[V0][0]);
    fe_render_vt(c, A3, V1);
    fe_render_vcall(c, A3, V1, A0, V0, 0xd0, 0x001a_23f4)?;
    // SetViewMatrix(t4mat_Identity).
    fe_render_grman(c, A2);
    c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
    fe_render_vt(c, A2, V0);
    fe_render_vcall(c, A2, V0, A0, V1, 0x110, 0x001a_2414)?;

    // top->alpha = 9.
    fe_render_grman(c, A0);
    c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = 0x3;
    let grman = r32(c, A0);
    c.g[V1][0] = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    let gs = r32(c, V1);
    c.g[V0][0] = Addr(gs.wrapping_add(0x8)).lw(c);
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= 0x120;
    Addr(gs.wrapping_add(0x8)).sw(c, c.g[V0][0]);
    // The compiler's empty countdown loop (a2 = 3 .. -1).
    loop {
        c.cycles += 7;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        let more = c.g[A2][0] != c.g[A3][0];
        c.g[A1][0] = 0x80;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001a_2438);
        }
    }

    // Four vertices: rgba (0, 0, 0, 0x80), q = 1.
    c.cycles += 5;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[V1][0] = c.g[SP][0];
    let s = sp(c);
    c.g[A0][0] = sx(s.wrapping_add(0xc0));
    Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A1][0]);
    loop {
        c.cycles += 8;
        let v = r32(c, V1);
        Addr(v.wrapping_add(0x10)).sw(c, 0);
        Addr(v.wrapping_add(0x14)).sw(c, 0);
        Addr(v.wrapping_add(0x18)).sw(c, 0);
        c.swc1(0, v.wrapping_add(0x8));
        c.g[V1][0] = sx(v.wrapping_add(0x30));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A1][0]);
        if event_due(c) {
            deliver_event_here(c, 0x001a_2468);
        }
    }

    // Positions (0,0), (0,w), (h,0)... built in the sp+0xc0 scratch vector.
    c.cycles += 58;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    let s = sp(c);
    let p = s.wrapping_add(0xc0);
    Addr(p).sw(c, 0);
    c.g[A1][0] = 0x4;
    Addr(p.wrapping_add(4)).sw(c, 0);
    c.g[A2][0] = c.g[SP][0];
    Addr(p.wrapping_add(8)).sw(c, 0);
    c.g[A3][0] = 0;
    c.swc1(0, p.wrapping_add(0xc));
    fe_render_grman(c, T0);
    c.lq(V0, p);
    Addr(p).sw(c, 0);
    c.swc1(21, p.wrapping_add(4));
    Addr(p.wrapping_add(8)).sw(c, 0);
    c.sq(V0, s.wrapping_add(0x20));
    c.lq(V1, p);
    Addr(p).sw(c, 0);
    Addr(p.wrapping_add(4)).sw(c, 0);
    c.swc1(20, p.wrapping_add(8));
    c.sq(V1, s.wrapping_add(0x50));
    c.lq(A0, p);
    c.swc1(21, p.wrapping_add(4));
    Addr(p).sw(c, 0);
    c.sq(A0, s.wrapping_add(0x80));
    c.lq(V0, p);
    c.sq(V0, s.wrapping_add(0xb0));
    // top->tex = -1.
    let gm = r32(c, T0);
    c.g[V1][0] = Addr(gm.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    Addr(r32(c, V1).wrapping_add(0x10)).sh(c, c.g[T1][0]);
    // DrawStrip(4, v, 0).
    fe_render_grman(c, T0);
    fe_render_vt(c, T0, V0);
    fe_render_vcall(c, T0, V0, A0, V1, 0x238, 0x001a_2514)?;
    fe_render_grman(c, A1);
    fe_render_vt(c, A1, V0);
    fe_render_vcall(c, A1, V0, A0, V1, 0xb0, 0x001a_252c)?;
    fe_render_grman(c, A1);
    fe_render_vt(c, A1, V0);
    fe_render_vcall(c, A1, V0, A0, V1, 0x100, 0x001a_2544)?;
    fe_render_pop(c);
    fe_render_vcall(c, A0, A1, V1, V0, 0xa0, 0x001a_2568)
}

/// Restore s0/s1/ra/f20/f21 and return.
fn fe_render_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    frame.restore(c, S0, 0xf0);
    frame.restore(c, S1, 0xe0);
    frame.restore_ra(c, 0xd0);
    frame.restore_fpr(c, 21, 0x108);
    frame.restore_fpr(c, 20, 0x100);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's fakes (word `OC381_MAGIC`).
    const OC381_CTRL: u32 = DATA + 0x7_8000;
    const OC381_MAGIC: u32 = 0x0381_f00d;
    /// `OC381_CTRL + 4` = 1: the `AppModule_RenderFade` fake returns non-zero.
    const OC381_FADE_ON: u32 = OC381_CTRL + 4;
    /// `OC381_CTRL + 8` = 1: GrMan vslot 17 (ready) returns 0.
    const OC381_NOT_READY: u32 = OC381_CTRL + 8;
    const OC381_GRMAN: u32 = DATA + 0x1_0000;
    const OC381_VT: u32 = DATA + 0x2_0000;
    const OC381_TOP: u32 = DATA + 0x3_0000;
    const OC381_SELF: u32 = DATA + 0x4_0000;
    const OC381_VIDEO: u32 = DATA + 0x6_0000;
    /// The GrMan vtable entries FEModule_Render calls; entry i jumps to
    /// `FAKE + 0x10 * i`.
    const OC381_SLOTS: [u32; 12] = [
        0x50, 0x58, 0x88, 0xa0, 0xa8, 0xb0, 0xc8, 0xd0, 0xf8, 0x100, 0x110, 0x238,
    ];
    /// The fake behind vslot 17 (entry 0x88).
    const OC381_FAKE_READY: u32 = FAKE + 0x20;

    macro_rules! oc381_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC381_CTRL) != OC381_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if $addr == 0x0023_1d60u32 && c.bus.read32(OC381_FADE_ON) != 1 {
                    c.g[V0][0] = 0;
                }
            }
        };
    }

    oc381_gated!(
        oc381_fake_render_fade,
        OC381_PREV_RENDER_FADE,
        0x0023_1d60u32,
        ee::s06::f_00231d60__generated
    );
    oc381_gated!(
        oc381_fake_ui_state_mgr,
        OC381_PREV_UI_STATE_MGR,
        0x0039_8018u32,
        ee::s13::f_00398018__generated
    );
    oc381_gated!(
        oc381_fake_rider_previews,
        OC381_PREV_RIDER_PREVIEWS,
        0x001a_0598u32,
        ee::s03::f_001a0598__generated
    );
    oc381_gated!(
        oc381_fake_ui_render,
        OC381_PREV_UI_RENDER,
        0x0039_7df8u32,
        ee::s13::f_00397df8__generated
    );
    oc381_gated!(
        oc381_fake_audio_update,
        OC381_PREV_AUDIO_UPDATE,
        0x0028_5bf8u32,
        ee::s07::f_00285bf8__generated
    );
    oc381_gated!(
        oc381_fake_audio_instance,
        OC381_PREV_AUDIO_INSTANCE,
        0x0028_b180u32,
        ee::s07::f_0028b180__generated
    );

    fn oc381_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00231D60),
            oc381_fake_render_fade,
            &OC381_PREV_RENDER_FADE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00398018),
            oc381_fake_ui_state_mgr,
            &OC381_PREV_UI_STATE_MGR,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A0598),
            oc381_fake_rider_previews,
            &OC381_PREV_RIDER_PREVIEWS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397DF8),
            oc381_fake_ui_render,
            &OC381_PREV_UI_RENDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285BF8),
            oc381_fake_audio_update,
            &OC381_PREV_AUDIO_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc381_fake_audio_instance,
            &OC381_PREV_AUDIO_INSTANCE,
        );
    }

    /// GrMan's methods: logged fakes returning sign-extended results (the
    /// screen size can be "negative" for the unsigned-to-float path), and
    /// vslot 17 returns 0 on request.
    fn oc381_miss(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        c.g[V0][0] = if target == OC381_FAKE_READY && c.bus.read32(OC381_NOT_READY) == 1 {
            0
        } else {
            sx(c.g[V0][0] as u32)
        };
    }

    /// Paths by `seed % 8`: 0 not ready; 1 fading; 2 a video playing
    /// (`VidEnginePlayer_Render` runs for real on an idle player); 3 the
    /// black backdrop with screen sizes >= 2^31; 4 the backdrop with small
    /// sizes; 5-7 the module render. The fade counter is 0 or not.
    #[test]
    fn fe_module_render_matches() {
        oc381_install_fakes();
        covered(
            ee::s03::f_001a2208__generated,
            fe_module_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_2208, 14);
                c.hooks.miss = oc381_miss;
                c.pc = 0x001a_2208;
                c.bus.write32(OC381_CTRL, OC381_MAGIC);
                let path = seed % 8;
                c.bus.write32(OC381_NOT_READY, (path == 0) as u32);
                c.bus.write32(OC381_FADE_ON, (path == 1) as u32);

                c.bus
                    .write32(GAME_GP - FE_MODULE_RENDER_GRMAN_GP, OC381_GRMAN);
                c.bus
                    .write32(OC381_GRMAN + FE_MODULE_RENDER_VTABLE, OC381_VT);
                for (i, &off) in OC381_SLOTS.iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(OC381_VT + off, delta as u16);
                    c.bus.write32(OC381_VT + off + 4, FAKE + 0x10 * i as u32);
                }
                let top = OC381_TOP + 4 * (seed / 8 % 4);
                c.bus.write32(OC381_GRMAN + FE_MODULE_RENDER_GS_TOP, top);
                fill(c, &mut r, top, 16);
                fill(c, &mut r, FE_MODULE_RENDER_DEFAULT_GS, 5);

                let this = OC381_SELF;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x18);
                let fade = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(this + 0xb_0000 + FE_MODULE_RENDER_FADE, fade);
                let video = if path == 2 { OC381_VIDEO } else { 0 };
                c.bus
                    .write32(this + 0xb_0000 + FE_MODULE_RENDER_VIDEO, video);
                c.bus.write32(OC381_VIDEO + 0x38, 0);
                let backdrop = if path == 3 || path == 4 {
                    r.next_u32() | 1
                } else {
                    0
                };
                c.bus
                    .write32(GAME_GP - FE_MODULE_RENDER_BACKDROP_GP, backdrop);
                // Screen width/height: fake calls 2 and 3 on the backdrop path.
                if path == 3 {
                    c.bus.write32(LOG_RESULTS + 8, r.next_u32() | 0x8000_0000);
                    c.bus.write32(LOG_RESULTS + 12, r.next_u32() | 0x8000_0000);
                } else if path == 4 {
                    c.bus.write32(LOG_RESULTS + 8, 1 + r.next_u32() % 1024);
                    c.bus.write32(LOG_RESULTS + 12, 1 + r.next_u32() % 1024);
                }
            },
        );
    }
}
