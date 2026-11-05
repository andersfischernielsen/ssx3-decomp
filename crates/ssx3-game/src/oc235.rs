//! Staging wave 204 (oc235): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

// ------------------------------------------------- SND_DspClip16_Process

/// `int SND_DspClip16_Process(DspClip16* o, int count, float* in, s16* out)`
/// (0x3ca6a8): the last stage of a sound DSP chain. If the object has an
/// upstream stage (`o->+0x8`), its process method (first word of the stage)
/// is called first as `(stage, count, out, in)` with `t0 = o->+0x1a`; a
/// result `<= 0` is returned as is. Then `count` floats from `in` are
/// clamped to [-32768, 32767] and packed as 16-bit samples into `out`, 16
/// samples (four VU0 vectors) per step. Returns `count`.
pub fn snd_dsp_clip16_process(c: &mut Cpu) {
    /// The upstream DSP stage (0: none).
    const UPSTREAM: u32 = 0x8;
    /// The byte passed to the upstream stage in t0.
    const CHANNEL: u32 = 0x1a;
    /// Return address of the upstream call.
    const RET_UPSTREAM: u32 = 0x003c_a6e8;
    /// The clamp-and-pack loop head.
    const LOOP: u32 = 0x003c_a714;

    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        let stage = Addr(c.g[A0][0] as u32).off(UPSTREAM).lw(c);
        c.g[V0][0] = stage;
        c.g[S2][0] = c.g[A2][0];

        let mut clip = true;
        if stage != 0 {
            // Registers at the call: (stage, count, out, in), t0 = channel.
            c.cycles += 8;
            c.g[T0][0] = Addr(c.g[A0][0] as u32).off(CHANNEL).lbu(c);
            c.g[A0][0] = stage;
            c.g[A2][0] = c.g[S1][0];
            let method = Addr(stage as u32).lw(c);
            c.g[V0][0] = method;
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, method as u32, RET_UPSTREAM)?;
            clip = (c.g[V0][0] as i64) > 0;
        }

        if clip {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0]; // floats left
            c.g[A1][0] = c.g[S2][0]; // input cursor
            c.g[A2][0] = c.g[S1][0]; // output cursor
            c.g[T0][0] = 0xffff_ffff_ffff_8000;
            c.g[T1][0] = 0x7fff;
            // vf1.x = -32768.0, vf2.x = 32767.0
            c.qmtc2(T0, 1);
            c.qmtc2(T1, 2);
            c.vu0().itof(0, 0b1111, 1, 1);
            c.vu0().itof(0, 0b1111, 2, 2);
            loop {
                c.cycles += 28;
                let input = c.g[A1][0] as u32;
                c.lqc2(3, input);
                c.lqc2(4, input.wrapping_add(0x10));
                c.lqc2(5, input.wrapping_add(0x20));
                c.lqc2(6, input.wrapping_add(0x30));
                for vf in 3..=6u8 {
                    c.vu0().max(0b1111, vf, vf, Src::Bc(1, X));
                }
                for vf in 3..=6u8 {
                    c.vu0().mini(0b1111, vf, vf, Src::Bc(2, X));
                }
                for vf in 3..=6u8 {
                    c.vu0().ftoi(0, 0b1111, vf, vf);
                }
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                c.g[A1][0] = sx(input.wrapping_add(0x40));
                c.qmfc2(A3, 3);
                c.qmfc2(T0, 4);
                c.qmfc2(T1, 5);
                c.qmfc2(T2, 6);
                c.ppach(A3, T0, A3);
                c.ppach(T1, T2, T1);
                let out = c.g[A2][0] as u32;
                c.sq(A3, out);
                c.sq(T1, out.wrapping_add(0x10));
                if (c.g[A0][0] as i64) <= 0 {
                    break;
                }
                c.g[A2][0] = sx(out.wrapping_add(0x20));
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------ PS2GraphicsMan_SetTexFunction

/// `void PS2GraphicsMan_SetTexFunction(PS2GraphicsMan* g, int tex, int fn)`
/// (0x37cbc8): forwards to `TexMan_SetTexFunction(g->texMan, tex, fn)`
/// (the texture manager at +0x18f4).
pub fn ps2_graphics_man_set_tex_function(c: &mut Cpu) {
    /// `TexMan* texMan`.
    const TEX_MAN: u32 = 0x18f4;
    const TEX_MAN_SET_TEX_FUNCTION: Callee = Callee {
        addr: 0x0036_91b0,
        f: ee::s12::TexMan_SetTexFunction,
    };
    crate::call::tail_wrapper(
        c,
        Some(TEX_MAN),
        crate::call::Target::Direct(TEX_MAN_SET_TEX_FUNCTION),
        0x0037_cbd8,
    );
}

// ------------------------------------------------------------ PATH_Update

/// `void PATH_Update(char mode)` (0x3d5128): the PATH music player's tick.
/// Runs only when PATH is not locked (byte 0x44f42d), is initialised (byte
/// 0x44f42c) and no player is current (0x44f430 == 0). It then stores the
/// mode byte, marks "busy" (current = -1), samples the time in ms, refreshes
/// the platform streams, and for each of the four players (0x517610, 0x928
/// bytes each) that is active (+0x34) makes it current (state pointer
/// 0x44f420, current = its handle at +0x30) and, with a handle, selects its
/// master track and updates its tracks. Finally it refreshes the platform
/// streams with reset and clears current and state.
pub fn path_update(c: &mut Cpu) {
    use crate::game::misc::path::{MODE, STATE, TIME};
    /// Non-zero: PATH is locked, skip the tick.
    const LOCKED: u32 = 0x0044_f42d;
    /// Zero: PATH is not initialised.
    const READY: u32 = 0x0044_f42c;
    /// The current player's handle (-1 while updating, 0 when idle).
    const CURRENT: u32 = 0x0044_f430;
    /// The four players.
    const PLAYERS: u32 = 0x0051_7610;
    const PLAYER_SIZE: u32 = 0x928;
    const PLAYER_COUNT: u64 = 4;
    /// Player fields: its handle and its "active" word.
    const HANDLE: u32 = 0x30;
    const ACTIVE: u32 = 0x34;
    const PATH_SELECT_MASTER_TRACK: Callee = Callee {
        addr: 0x003d_4b00,
        f: ee::s14::PATH_SelectMasterTrack,
    };
    const PATH_UPDATE_TRACKS: Callee = Callee {
        addr: 0x003d_4f10,
        f: ee::s14::PATH_UpdateTracks,
    };
    const PATH_PLAT_GET_TIME_MS: u32 = 0x002a_f370;
    const PATH_PLAT_REFRESH_STATUS: u32 = 0x002a_efa8;
    /// The player loop head.
    const LOOP: u32 = 0x003d_51d8;

    guest(c, |c| {
        c.cycles += 15;
        c.g[V1][0] = 0x45_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = Addr(LOCKED).lbu(c);
        c.g[A0][0] &= 0xff;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        let mut run = c.g[V0][0] == 0;
        if run {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            c.g[V1][0] = Addr(READY).lbu(c);
            run = c.g[V1][0] != 0;
            c.g[V1][0] = 0x45_0000;
        }
        if run {
            c.cycles += 3;
            c.g[V0][0] = Addr(CURRENT).lw(c);
            c.g[S1][0] = c.g[V1][0];
            run = c.g[V0][0] == 0;
        }
        if run {
            // Busy: current = -1; sample the time.
            c.cycles += 22;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = 0x45_0000;
            c.sb(MODE, c.g[A0][0]);
            c.g[S2][0] = 0xffff_ffff_ffff_0000;
            c.sw(CURRENT, c.g[V0][0]);
            call_ptr(c, PATH_PLAT_GET_TIME_MS, 0x003d_519c)?;
            c.g[S0][0] = 0;
            c.g[V1][0] = 0x45_0000;
            c.g[A0][0] = 0;
            c.sw(TIME, c.g[V0][0]);
            c.g[S4][0] = 0x45_0000;
            call_ptr(c, PATH_PLAT_REFRESH_STATUS, 0x003d_51b4)?;

            // s0 = player index, a0 = its offset (s0 * 0x928, via mult).
            c.g[FP][0] = PLAYER_SIZE as u64;
            c.g[V0][0] = 0x51_0000;
            c.g[S2][0] |= 0xffff;
            c.g[S3][0] = sx(PLAYERS);
            c.g[S7][0] = u64::MAX;
            c.g[S6][0] = sx(PLAYERS + ACTIVE);
            c.g[S5][0] = sx(PLAYERS + HANDLE);
            c.mult(0, A0, S0, FP);
            loop {
                c.cycles += 4;
                let off = c.g[A0][0] as u32;
                c.g[V0][0] = sx(off.wrapping_add(r32(c, S6)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let active = c.g[V1][0] != 0;
                c.g[V0][0] = Addr(CURRENT).lw(c);
                if active {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.sw(CURRENT, c.g[S2][0]);
                    }
                    c.cycles += 3;
                    let in_range = c.g[S0][0] < PLAYER_COUNT;
                    c.g[V0][0] = sx(off.wrapping_add(r32(c, S5)));
                    if in_range {
                        // Make it current: state = the player, current = handle.
                        c.cycles += 6;
                        c.g[V1][0] = sx(off.wrapping_add(r32(c, S3)));
                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                        c.sw(STATE, c.g[V1][0]);
                        c.g[V0][0] = c.g[A0][0];
                        c.sw(CURRENT, c.g[A0][0]);
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[S7][0] {
                            c.sw(CURRENT, 0);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(CURRENT).lw(c);
                    }
                    c.cycles += 2;
                    let handle = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    if handle {
                        c.cycles += 4;
                        PATH_SELECT_MASTER_TRACK.call(c, 0x003d_5230)?;
                        PATH_UPDATE_TRACKS.call(c, 0x003d_5238)?;
                        c.cycles += 1;
                        c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                }
                c.cycles += 4;
                c.g[S0][0] = c.g[V0][0] & 0xff;
                c.g[V1][0] = (c.g[S0][0] < PLAYER_COUNT) as u64;
                c.mult(0, A0, S0, FP);
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
            // Refresh with reset; idle again.
            c.cycles += 4;
            c.g[A0][0] = 1;
            call_ptr(c, PATH_PLAT_REFRESH_STATUS, 0x003d_5254)?;
            c.sw(CURRENT, 0);
            c.sw(STATE, 0);
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- PS2BezierMan_RenderPatchFX

/// `void PS2BezierMan_RenderPatchFX(PS2BezierMan* bm, int pass)` (0x38d968):
/// the glitter pass over the terrain patches. Unless FX are off (0x4a44e0),
/// pushes a copy of the graphics manager's render state (stack top at
/// +0xe84, 0x14-byte entries), selects the current FX texture
/// (`texIds[texSel]`, +0xf50 / +0x64) and sets the FX blend/test flags,
/// refreshes the view matrices if stale (+0x6b90), then for every type-8
/// patch (count +0x4a60, 0x10-byte entries at +0x4a64) and every mixed patch
/// with a positive kind (count +0x4d84, 0x14-byte entries at +0x4d88, kind
/// at +8) uploads the patch (0x1b0 bytes) to the scratchpad and draws its FX
/// (`PS2BezierMan_DrawPatchFX(bm, uploaded, entry, pass)`); finally pops the
/// render state.
pub fn ps2_bezier_man_render_patch_fx(c: &mut Cpu) {
    /// Non-zero: patch FX are switched off (gp+0x13f0).
    const FX_OFF: u32 = 0x13f0;
    /// The graphics manager (gp+0x2a90).
    const GFX_MAN: u32 = 0x2a90;
    /// Graphics manager fields.
    const STATE_TOP: u32 = 0xe84;
    const TEX_SEL: u32 = 0x64;
    const TEX_IDS: u32 = 0xf50;
    const VIEW_VALID: u32 = 0x6b90;
    /// Render-state entry size and fields.
    const STATE_SIZE: u32 = 0x14;
    /// Bezier manager fields.
    const PATCH8_COUNT: u32 = 0x4a60;
    const PATCH8: u32 = 0x4a64;
    const MIX_COUNT: u32 = 0x4d84;
    const MIX: u32 = 0x4d88;
    const MIX_KIND: u32 = 0x8;
    /// Bytes of a patch uploaded to the scratchpad.
    const PATCH_BYTES: u64 = 0x1b0;
    const UPDATE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0037_d968,
        f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
    };
    const SPR_STREAM_UPLOAD: Callee = Callee {
        addr: 0x0038_f708,
        f: ee::s12::SprStream_Upload,
    };
    const DRAW_PATCH_FX: Callee = Callee {
        addr: 0x0038_d690,
        f: ee::s12::PS2BezierMan_DrawPatchFX,
    };

    /// `base = gfx->top; base->[off] = (base->[off] & mask) | set` with the
    /// pointer in `base`, the word in v0.
    fn update_top(c: &mut Cpu, gfx: u32, base: usize, off: u32, mask: u64, set: u64) {
        c.g[base][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
        let field = (c.g[base][0] as u32).wrapping_add(off);
        c.g[V0][0] = Addr(field).lw(c);
        c.g[V0][0] = (c.g[V0][0] & mask) | set;
        c.sw(field, c.g[V0][0]);
    }

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(FX_OFF)).lw(c);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 68;
            let gfx = Addr(gp.wrapping_add(GFX_MAN)).lw(c);
            c.g[S2][0] = gfx;
            let gfx = gfx as u32;
            c.g[T3][0] = !0x7c;
            c.g[T5][0] = !0x3e0;
            c.g[T0][0] = 0xffff_ffff_fe7f_0000;
            // Push: copy the top entry (8 + 8 + 4 bytes) one entry up.
            let top = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = top;
            let top = top as u32;
            c.g[T0][0] |= 0xffff;
            c.g[T6][0] = 0x80_0000;
            c.g[T1][0] = 0xffff_ffff_ffcf_0000;
            c.ldl(V1, top.wrapping_add(0x7));
            c.ldr(V1, top);
            c.ldl(A0, top.wrapping_add(0xf));
            c.ldr(A0, top.wrapping_add(0x8));
            c.g[A1][0] = Addr(top.wrapping_add(0x10)).lw(c);
            c.sdl(V1, top.wrapping_add(0x1b));
            c.sdr(V1, top.wrapping_add(0x14));
            c.sdl(A0, top.wrapping_add(0x23));
            c.sdr(A0, top.wrapping_add(0x1c));
            c.sw(top.wrapping_add(0x24), c.g[A1][0]);
            c.g[T1][0] |= 0xffff;
            c.g[T7][0] = 0x30_0000;
            c.g[T2][0] = 0xffff_ffff_fff0_0000;
            c.g[A3][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[T2][0] |= 0xfff;
            c.g[V0][0] = Addr(gfx.wrapping_add(TEX_SEL)).lw(c);
            c.g[A2][0] = 0x5_0000;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(STATE_SIZE));
            c.g[A2][0] |= 0xc000;
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V1][0]);
            c.g[V0][0] = sx(gfx.wrapping_add((c.g[V0][0] as u32) << 2));
            c.g[A1][0] = 0xffff_ffff_ffbf_0000;
            // The FX texture (halfword at the new top + 0x10).
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(TEX_IDS)).lw(c);
            c.g[A1][0] |= 0xffff;
            c.g[T4][0] = 0x40_0000;
            c.sh((c.g[A3][0] as u32).wrapping_add(0x24), c.g[A0][0]);
            // The FX blend / test flags.
            let (t3, t5, t0, t6, t1, t7, t2, a2, a1, t4) = (
                c.g[T3][0], c.g[T5][0], c.g[T0][0], c.g[T6][0], c.g[T1][0], c.g[T7][0],
                c.g[T2][0], c.g[A2][0], c.g[A1][0], c.g[T4][0],
            );
            update_top(c, gfx, V1, 0x4, t3, 0x54);
            update_top(c, gfx, A0, 0x8, t5, 0x80);
            update_top(c, gfx, V1, 0x4, t0, t6);
            update_top(c, gfx, A0, 0x4, t1, t7);
            update_top(c, gfx, V1, 0x4, t2, a2);
            update_top(c, gfx, A0, 0x4, a1, t4);
            c.g[V1][0] = Addr(gfx.wrapping_add(VIEW_VALID)).lw(c);
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                UPDATE_VIEW_MATRICES.call(c, 0x0038_daac)?;
            }
            c.g[A0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);

            c.cycles += 25;
            c.g[V1][0] = !0x3c0;
            c.g[A1][0] = 0xffff_ffff_c000_0000;
            c.g[A3][0] = !0x3;
            let top = c.g[A0][0] as u32;
            c.g[V0][0] = Addr(top).lw(c);
            c.g[A2][0] = 0xffff_ffff_e000_03ff;
            c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | 0x100;
            c.sw(top, c.g[V0][0]);
            let (a1, a3, a2) = (c.g[A1][0], c.g[A3][0], c.g[A2][0]);
            update_top(c, gfx, V1, 0xc, a1, 0);
            update_top(c, gfx, A1, 0x4, a3, 0x2);
            update_top(c, gfx, A0, 0x8, a2, 0);

            // Type-8 patches.
            let bm = c.g[S3][0] as u32;
            c.g[V1][0] = Addr(bm.wrapping_add(PATCH8_COUNT)).lw(c);
            c.g[S1][0] = 0;
            if (c.g[V1][0] as i64) > 0 {
                c.cycles += 1;
                c.g[S0][0] = sx(bm.wrapping_add(PATCH8));
                loop {
                    c.cycles += 14;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A1][0] = PATCH_BYTES;
                    c.g[A2][0] = 0x2;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    SPR_STREAM_UPLOAD.call(c, 0x0038_db2c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A3][0] = c.g[S4][0];
                    DRAW_PATCH_FX.call(c, 0x0038_db40)?;
                    c.g[V0][0] = Addr(bm.wrapping_add(PATCH8_COUNT)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_db18);
                    }
                }
            }

            // Mixed patches with a positive kind.
            c.cycles += 3;
            c.g[V0][0] = Addr(bm.wrapping_add(MIX_COUNT)).lw(c);
            c.g[S1][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 1;
                c.g[S0][0] = sx(bm.wrapping_add(MIX));
                loop {
                    c.cycles += 3;
                    let entry = c.g[S0][0] as u32;
                    c.g[V0][0] = Addr(entry.wrapping_add(MIX_KIND)).lw(c);
                    c.g[A1][0] = PATCH_BYTES;
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 8;
                        c.g[A0][0] = Addr(entry).lw(c);
                        c.g[A2][0] = 0x2;
                        SPR_STREAM_UPLOAD.call(c, 0x0038_db78)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[A3][0] = c.g[S4][0];
                        DRAW_PATCH_FX.call(c, 0x0038_db8c)?;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(bm.wrapping_add(MIX_COUNT)).lw(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_db60);
                    }
                }
            }

            // Pop the render state.
            c.cycles += 3;
            c.g[V0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(STATE_SIZE));
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V0][0]);
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ Rider_TryEnterBoardPress

/// `bool Rider_TryEnterBoardPress(Rider* r, float dir)` (0x1161d0): start a
/// board press (nose press for `dir > 0`, tail press for `dir < 0`). Fails
/// (returns 0) for `dir == 0`, while the press sub-state (+0x328) is 3 or 4,
/// or, outside the main state 0, when the board's forward axis (+0x1e0) and
/// the velocity (+0x3a0) point apart (VU0 dot product < 0). Otherwise plays
/// animation 0x18 / 0x20 on the rider's anim controller (+0x784) with blend
/// -1, stores the press side (1 nose, 2 tail) at +0x330, enters sub-state
/// BOARD_PRESS and returns 1.
pub fn rider_try_enter_board_press(c: &mut Cpu) {
    /// Press sub-state (3 and 4 block a new press).
    const PRESS_STATE: u32 = 0x328;
    /// Press side: 1 nose, 2 tail.
    const PRESS_SIDE: u32 = 0x330;
    const FORWARD: u32 = 0x1e0;
    const VELOCITY: u32 = 0x3a0;
    const ANIM_CTL: u32 = 0x784;
    const ANIM_NOSE_PRESS: u64 = 0x18;
    const ANIM_TAIL_PRESS: u64 = 0x20;
    const BOARD_PRESS: u64 = 1;
    const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;
    const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    /// -1.0f: the anim blend argument.
    const MINUS_ONE: u32 = 0xbf80_0000;

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.f[21] = 0;
        let dir = FpuF32(c.f[12]);
        c.f[20] = dir.0;
        frame.save(c, S0, 0x10);
        let no_dir = dir.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_dir);
        frame.save_ra(c, 0);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        c.g[V0][0] = 0;
        let r = Addr(rider as u32);
        'out: {
            if no_dir {
                break 'out;
            }
            c.cycles += 5;
            let state = r.off(PRESS_STATE).lw(c);
            let blocked = (state as u32).wrapping_sub(3) < 2;
            c.g[V0][0] = 0;
            if blocked {
                break 'out;
            }
            c.cycles += 4;
            c.g[A0][0] = rider;
            RIDER_GET_MAIN_STATE.call(c, 0x0011_6218)?;
            if c.g[V0][0] == 0 {
                // forward . velocity (xyz) via VU0, in f0.
                c.cycles += 13;
                c.lqc2(3, r.off(FORWARD).0);
                c.lqc2(5, r.off(VELOCITY).0);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(V0, 4);
                c.f[0] = c.g[V0][0] as u32;
                let apart = abi::c_lt_s(c, FpuF32(c.f[0]), FpuF32(c.f[21]));
                c.g[V0][0] = 0;
                if apart {
                    break 'out;
                }
            }
            c.cycles += 5;
            c.f[0] = 0;
            let nose = abi::c_lt_s(c, FpuF32::ZERO, dir);
            c.g[A0][0] = r.off(ANIM_CTL).lw(c);
            let (anim, side, ret) = if nose {
                c.cycles += 7;
                (ANIM_NOSE_PRESS, 1, 0x0011_627c)
            } else {
                c.cycles += 6;
                (ANIM_TAIL_PRESS, 2, 0x0011_6298)
            };
            c.g[A1][0] = anim;
            c.g[AT][0] = sx(MINUS_ONE);
            c.f[12] = MINUS_ONE;
            c.g[A2][0] = 0;
            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, ret)?;
            c.g[V1][0] = side;
            c.cycles += 5;
            r.off(PRESS_SIDE).sw(c, side);
            c.g[A0][0] = rider;
            c.g[A1][0] = BOARD_PRESS;
            RIDER_SET_SUB_STATE.call(c, 0x0011_62ac)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------------- SNDMIDI_Service

/// `void SNDMIDI_Service(void)` (0x3b96b8): the MIDI sequencer tick. For each
/// of the 16 MIDI slots (0x50a120, 0x70 bytes each) that is playing (state
/// byte +0x28 == 3) and whose volume fade (+0x18, `SNDMIDI_UpdateFade`) did
/// not stop it: advance the tick counter (+0x0) and, while the scaled time
/// `ticks * tempo% (+0x24) / 100` has reached the next event's time (+0x4)
/// and the slot is still playing, process one event and count it.
pub fn sndmidi_service(c: &mut Cpu) {
    const SLOTS: u32 = 0x0050_a120;
    const SLOT_SIZE: u64 = 0x70;
    const SLOT_COUNT: i64 = 16;
    /// Slot fields.
    const TICKS: u32 = 0x0;
    const NEXT_EVENT: u32 = 0x4;
    const FADE: u32 = 0x18;
    const TEMPO: u32 = 0x24;
    const STATE: u32 = 0x28;
    const PLAYING: u64 = 3;
    const UPDATE_FADE: Callee = Callee {
        addr: 0x003b_8de8,
        f: ee::s13::SNDMIDI_UpdateFade,
    };
    const PROCESS_EVENT: Callee = Callee {
        addr: 0x003b_94b0,
        f: ee::s13::SNDMIDI_ProcessEvent,
    };

    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = 0; // slot index
        frame.save(c, S6, 0x30);
        c.g[S5][0] = 100; // percent divisor
        frame.save(c, S7, 0x20);
        c.g[S6][0] = PLAYING;
        frame.save(c, FP, 0x10);
        c.g[S7][0] = SLOT_SIZE;
        frame.save(c, S0, 0x90);
        c.g[FP][0] = sx(SLOTS);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0);
        c.mult(0, V0, S2, S7);
        loop {
            c.cycles += 4;
            let slot = (c.g[V0][0] as u32).wrapping_add(r32(c, FP));
            c.g[S0][0] = sx(slot);
            let e = Addr(slot);
            c.g[S3][0] = e.off(STATE).lb(c);
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
            // false: the path that reaches the slot loop's end directly.
            let mut via_tail = true;
            'slot: {
                if c.g[S3][0] != c.g[S6][0] {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = e.off(FADE).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    UPDATE_FADE.call(c, 0x003b_9724)?;
                    if c.g[V0][0] != 0 {
                        // Stopped by its fade.
                        via_tail = false;
                        break 'slot;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = e.off(TICKS).lw(c);
                // (divide-by-zero check of the divisor)
                c.cycles += 2;
                c.cycles += 10;
                c.g[A1][0] = e.off(TEMPO).lhu(c);
                c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[V1][0] = e.off(NEXT_EVENT).lw(c);
                c.mult(0, V0, V0, A1);
                c.divu(0, V0, S5);
                c.g[S1][0] = c.lo[0]; // scaled time
                c.g[V1][0] = (c.g[S1][0] < c.g[V1][0]) as u64;
                let early = c.g[V1][0] != 0;
                e.off(TICKS).sw(c, c.g[A0][0]);
                if early {
                    break 'slot;
                }
                c.cycles += 3;
                c.g[V0][0] = e.off(STATE).lb(c);
                if c.g[V0][0] != c.g[S3][0] {
                    via_tail = false;
                    break 'slot;
                }
                c.cycles += 1;
                c.g[S3][0] = PLAYING;
                loop {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    PROCESS_EVENT.call(c, 0x003b_9778)?;
                    c.g[V0][0] = e.off(NEXT_EVENT).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let done = c.g[V1][0] != 0;
                    e.off(NEXT_EVENT).sw(c, c.g[V0][0]);
                    if done {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = e.off(STATE).lb(c);
                    if c.g[V0][0] != c.g[S3][0] {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_9770);
                    }
                }
            }
            if via_tail {
                c.cycles += 1;
            }
            c.g[S2][0] = c.g[S4][0];
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < SLOT_COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.mult(0, V0, S2, S7);
            if event_due(c) {
                deliver_event_here(c, 0x003b_9700);
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- LensFX_DrawFlareQuad

/// `void LensFX_DrawFlareQuad(LensFX* fx, int idx)` (0x2e2868): draw one
/// lens-flare sprite. Skipped unless the flare is on (+0x2c). The colour's
/// direction is the particle's (+0x20) velocity (+0x10) normalised (as is
/// when zero); its alpha is the particle's per-sprite alpha `[idx]` (+0x40)
/// when the flare uses alpha (+0x2e), else 1.0, and the render state's
/// blend bits are set to match. With a positive alpha the texture (+0x1024
/// glow, +0x1028 ring) and size (180 / 100 / 350, else 0) follow the
/// particle's type bits (+0xc & 0x70), and the graphics manager's sprite
/// draw method (vtable +0x10d8, slot +0x278) is called with the position
/// (+0x1c, w 1), the size, uv (0,0)-(1,1), the colour, the rotation (+0x28)
/// and mode 2.
pub fn lens_fx_draw_flare_quad(c: &mut Cpu) {
    const ON: u32 = 0x2c;
    const USE_ALPHA: u32 = 0x2e;
    const PARTICLE: u32 = 0x20;
    const ROTATION: u32 = 0x28;
    /// Particle fields.
    const TYPE: u32 = 0xc;
    const VELOCITY: u32 = 0x10;
    const POSITION: u32 = 0x1c;
    const ALPHA: u32 = 0x40;
    /// Graphics manager fields.
    const GFX_MAN: u32 = 0x2a90;
    const STATE_TOP: u32 = 0xe84;
    const TEX_GLOW: u32 = 0x1024;
    const TEX_RING: u32 = 0x1028;
    const VTABLE: u32 = 0x10d8;
    const DRAW_SPRITE: u32 = 0x278;
    /// The render state's flags word.
    const FLAGS: u32 = 0x4;

    /// `top->flags = (top->flags & mask) | set` with the top in `base` and
    /// the word in v0 (the last store is left to the caller).
    fn flags(c: &mut Cpu, gfx: u32, base: usize, mask: u64, set: u64, store: bool) {
        c.g[base][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
        let at = (c.g[base][0] as u32).wrapping_add(FLAGS);
        c.g[V0][0] = (Addr(at).lw(c) & mask) | set;
        if store {
            c.sw(at, c.g[V0][0]);
        }
    }

    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x70);
        let fx = c.g[A0][0];
        c.g[T3][0] = fx;
        frame.save_ra(c, 0x60);
        let fx = Addr(fx as u32);
        c.g[V0][0] = fx.off(ON).lhu(c);
        frame.restore_ra(c, 0x60);
        if c.g[V0][0] != 0 {
            // Normalised velocity into sp+0..8.
            c.cycles += 15;
            let part = fx.off(PARTICLE).lw(c);
            c.g[V0][0] = part;
            let part = Addr(part as u32);
            c.f[3] = 0;
            let vx = part.off(VELOCITY).read_fpu(c);
            let vy = part.off(VELOCITY + 4).read_fpu(c);
            let vz = part.off(VELOCITY + 8).read_fpu(c);
            c.f[5] = vx.0;
            c.f[6] = vy.0;
            c.f[4] = vz.0;
            let sum = vx * vx + vy * vy;
            let zz = vz * vz;
            c.f[2] = zz.0;
            let sum = sum + zz;
            c.f[0] = sum.0;
            let len = sum.sqrt();
            c.f[1] = len.0;
            let zero = len.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, zero);
            let sp = crate::call::sp(c);
            if zero {
                c.swc1(5, sp);
                c.cycles += 4;
                let y = part.off(VELOCITY + 4).read_fpu(c);
                c.f[0] = y.0;
                c.swc1(0, sp.wrapping_add(0x4));
                let z = part.off(VELOCITY + 8).read_fpu(c);
                c.f[1] = z.0;
                c.swc1(1, sp.wrapping_add(0x8));
            } else {
                c.cycles += 10;
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                let inv = FpuF32::ONE.div(len);
                c.f[1] = (vz * inv).0;
                c.f[2] = (vx * inv).0;
                c.f[0] = (vy * inv).0;
                c.swc1(1, sp.wrapping_add(0x8));
                c.swc1(2, sp);
                c.swc1(0, sp.wrapping_add(0x4));
            }

            // The colour: direction at sp+0x14, alpha at sp+0x10.
            c.cycles += 9;
            c.lwc1(0, sp);
            c.lwc1(1, sp.wrapping_add(0x4));
            c.lwc1(2, sp.wrapping_add(0x8));
            c.g[V0][0] = fx.off(USE_ALPHA).lhu(c);
            c.swc1(0, sp.wrapping_add(0x14));
            c.swc1(1, sp.wrapping_add(0x18));
            c.swc1(2, sp.wrapping_add(0x1c));
            let use_alpha = c.g[V0][0] != 0;
            let gfx = Addr((c.g[GP][0] as u32).wrapping_add(GFX_MAN)).lw(c);
            c.g[T2][0] = gfx;
            let gfx = gfx as u32;
            if use_alpha {
                c.cycles += 28;
                c.g[V0][0] = fx.off(PARTICLE).lw(c);
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[A1][0] = 0xffff_ffff_fe7f_ffff;
                c.g[T1][0] = 0x100_0000;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.lwc1(0, r32(c, V0).wrapping_add(ALPHA));
                c.g[A2][0] = 0xffff_ffff_ffcf_ffff;
                c.g[T0][0] = 0xffff_ffff_fff0_0fff;
                c.swc1(0, sp.wrapping_add(0x10));
                c.g[A3][0] = 0x1_4000;
                let (a1, t1, a2, t0, a3) =
                    (c.g[A1][0], c.g[T1][0], c.g[A2][0], c.g[T0][0], c.g[A3][0]);
                flags(c, gfx, A0, a1, t1, true);
                flags(c, gfx, V1, a2, 0, true);
                flags(c, gfx, A0, t0, a3, false);
            } else {
                c.cycles += 25;
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                c.f[0] = FpuF32::ONE.0;
                c.g[V1][0] = 0xffff_ffff_fe7f_ffff;
                c.g[T0][0] = 0x80_0000;
                c.swc1(0, sp.wrapping_add(0x10));
                c.g[A0][0] = 0xffff_ffff_ffcf_ffff;
                c.g[A2][0] = 0xffff_ffff_fff0_0fff;
                c.g[A1][0] = 0x1_4000;
                let (v1, t0, a0, a2, a1) =
                    (c.g[V1][0], c.g[T0][0], c.g[A0][0], c.g[A2][0], c.g[A1][0]);
                flags(c, gfx, A3, v1, t0, true);
                flags(c, gfx, V1, a0, 0, true);
                flags(c, gfx, A0, a2, a1, false);
            }
            c.cycles += 7;
            c.sw(r32(c, A0).wrapping_add(FLAGS), c.g[V0][0]);
            c.lwc1(0, sp.wrapping_add(0x10));
            c.f[1] = 0;
            let alpha = FpuF32(c.f[0]);
            let invisible = alpha.le(FpuF32::ZERO);
            abi::set_fcc(c, invisible);
            frame.restore_ra(c, 0x60);
            if !invisible {
                // Texture and size by the particle type.
                c.cycles += 6;
                c.g[V0][0] = fx.off(PARTICLE).lw(c);
                c.g[A0][0] = 0x20;
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(TYPE)).lw(c) & 0x70;
                c.f[3] = c.f[1];
                let kind = c.g[V1][0];
                let tex = match kind {
                    0x20 => {
                        c.cycles += 5;
                        c.g[V1][0] = Addr(gfx.wrapping_add(TEX_RING)).lw(c);
                        Some(0x42c8_0000) // 100.0
                    }
                    _ => {
                        c.cycles += 3;
                        c.g[V0][0] = (kind < 0x21) as u64;
                        if kind < 0x21 {
                            c.g[V0][0] = 0x10;
                            c.cycles += 2;
                            if kind == 0x10 {
                                c.g[V1][0] = Addr(gfx.wrapping_add(TEX_GLOW)).lw(c);
                                c.cycles += 4;
                                Some(0x4334_0000) // 180.0
                            } else {
                                c.cycles += 2;
                                None
                            }
                        } else {
                            c.g[V0][0] = 0x10;
                            c.cycles += 3;
                            c.g[V0][0] = 0x40;
                            if kind == 0x40 {
                                c.g[V1][0] = Addr(gfx.wrapping_add(TEX_GLOW)).lw(c);
                                c.cycles += 3;
                                Some(0x43af_0000) // 350.0
                            } else {
                                c.cycles += 2;
                                None
                            }
                        }
                    }
                };
                if let Some(size) = tex {
                    c.g[AT][0] = size as u64;
                    c.f[3] = size;
                    c.g[V0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
                    c.cycles += 2;
                    c.sh(r32(c, V0).wrapping_add(0x10), c.g[V1][0]);
                }
                c.g[V0][0] = fx.off(PARTICLE).lw(c);

                // Draw: (pos, size, uv0, uv1, colour, rot, 2).
                c.cycles += 28;
                let part = r32(c, V0);
                c.g[A1][0] = sx(sp.wrapping_add(0x20));
                let vt = Addr(gfx.wrapping_add(VTABLE)).lw(c);
                c.g[V1][0] = vt;
                c.g[A2][0] = sx(sp.wrapping_add(0x30));
                c.lwc1(0, part.wrapping_add(POSITION));
                c.g[A3][0] = sx(sp.wrapping_add(0x40));
                c.g[A0][0] = Addr((vt as u32).wrapping_add(DRAW_SPRITE)).lh(c);
                c.g[T0][0] = sx(sp.wrapping_add(0x50));
                c.swc1(0, sp.wrapping_add(0x20));
                c.g[T1][0] = sx(sp.wrapping_add(0x10));
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                c.f[2] = FpuF32::ONE.0;
                c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
                c.lwc1(1, part.wrapping_add(POSITION + 4));
                c.f[12] = fx.off(ROTATION).read_fpu(c).0;
                c.swc1(1, sp.wrapping_add(0x24));
                c.lwc1(0, part.wrapping_add(POSITION + 8));
                c.swc1(3, sp.wrapping_add(0x34));
                c.swc1(0, sp.wrapping_add(0x28));
                c.swc1(2, sp.wrapping_add(0x54));
                c.swc1(2, sp.wrapping_add(0x2c));
                c.swc1(3, sp.wrapping_add(0x30));
                c.sw(sp.wrapping_add(0x40), 0);
                c.sw(sp.wrapping_add(0x44), 0);
                c.swc1(2, sp.wrapping_add(0x50));
                let method = Addr((vt as u32).wrapping_add(DRAW_SPRITE + 4)).lw(c);
                c.g[V0][0] = method;
                c.g[T2][0] = 2;
                call_ptr(c, method as u32, 0x002e_2af4)?;
                frame.restore_ra(c, 0x60);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------- VfxAvalanche_Render

/// `void VfxAvalanche_Render(void)` (0x2d9130): draw the avalanche effect.
/// Runs when the avalanche is on (gp+0x940) and visible (gp+0x9cc), or when
/// forced (gp+0x10fc). Pushes the graphics manager's render state (a copy
/// of the top, then the avalanche template at 0x501420, blend bits 7, no
/// texture), sets the current camera's (world +0x84 -> +0x84, index +0x14)
/// matrix, position and rotation through the manager's vtable (+0x10d8:
/// slots +0xc0, +0xc8, +0x110), renders the 16 avalanche slots (0x538938,
/// 0x1c bytes each) while the avalanche is on, and pops the state.
pub fn vfx_avalanche_render(c: &mut Cpu) {
    const ON: u32 = 0x940;
    const VISIBLE: u32 = 0x9cc;
    const FORCED: u32 = 0x10fc;
    /// gp-0x848: the world (app) pointer.
    const WORLD: u32 = 0x848;
    const GFX_MAN: u32 = 0x2a90;
    const STATE_TOP: u32 = 0xe84;
    const VTABLE: u32 = 0x10d8;
    const STATE_SIZE: u32 = 0x14;
    /// The avalanche's render-state template.
    const TEMPLATE: u32 = 0x0050_1420;
    const SLOTS: u32 = 0x0053_8938;
    const SLOT_SIZE: u32 = 0x1c;
    const SLOT_COUNT: u32 = 16;
    const RENDER_ACTIVE: Callee = Callee {
        addr: 0x002d_8ea8,
        f: ee::s09::Avalanche_RenderActive,
    };

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(ON)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let mut draw = false;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            draw = Addr(gp.wrapping_add(VISIBLE)).lw(c) != 0;
            c.g[V0][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
        }
        if !draw {
            c.cycles += 3;
            draw = Addr(gp.wrapping_add(FORCED)).lw(c) != 0;
            c.g[V0][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
        }
        if draw {
            c.cycles += 72;
            c.g[A3][0] = 0x50_0000;
            let gfx = Addr(gp.wrapping_add(GFX_MAN)).lw(c);
            c.g[S1][0] = gfx;
            let gfx = gfx as u32;
            c.g[T0][0] = !0x3e0;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
            c.g[T1][0] = u64::MAX;
            let top = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[A1][0] = top;
            let top = top as u32;
            c.g[A2][0] = 1;
            // The current camera: views[cur] (s0).
            c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x84)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(0x4)).lw(c);
            // Push a copy of the top.
            c.ldl(V0, top.wrapping_add(0x7));
            c.ldr(V0, top);
            c.ldl(V1, top.wrapping_add(0xf));
            c.ldr(V1, top.wrapping_add(0x8));
            c.g[A0][0] = Addr(top.wrapping_add(0x10)).lw(c);
            c.sdl(V0, top.wrapping_add(0x1b));
            c.sdr(V0, top.wrapping_add(0x14));
            c.sdl(V1, top.wrapping_add(0x23));
            c.sdr(V1, top.wrapping_add(0x1c));
            c.sw(top.wrapping_add(0x24), c.g[A0][0]);
            c.g[V1][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(STATE_SIZE));
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V0][0]);
            // ... and overwrite the new top with the template.
            c.g[T2][0] = sx(TEMPLATE);
            let old = r32(c, V1);
            c.ldl(V0, TEMPLATE + 0x7);
            c.ldr(V0, TEMPLATE);
            c.ldl(A0, TEMPLATE + 0xf);
            c.ldr(A0, TEMPLATE + 0x8);
            c.g[A1][0] = Addr(TEMPLATE + 0x10).lw(c);
            c.sdl(V0, old.wrapping_add(0x1b));
            c.sdr(V0, old.wrapping_add(0x14));
            c.sdl(A0, old.wrapping_add(0x23));
            c.sdr(A0, old.wrapping_add(0x1c));
            c.sw(old.wrapping_add(0x24), c.g[A1][0]);
            // Blend bits 7, texture -1.
            c.g[A0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            let flags = r32(c, A0).wrapping_add(0x8);
            c.g[V0][0] = (Addr(flags).lw(c) & c.g[T0][0]) | 0xe0;
            c.sw(flags, c.g[V0][0]);
            c.g[V1][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.sh(r32(c, V1).wrapping_add(0x10), c.g[T1][0]);

            // setMatrix(&cam->mats[cur], 1).
            c.g[V0][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
            c.g[A1][0] = Addr(gfx.wrapping_add(VTABLE)).lw(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
            c.g[A3][0] = Addr(r32(c, A1).wrapping_add(0xc4)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0xc0)).lh(c);
            c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
            c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
            c.g[A1][0] = sx(r32(c, A1) << 4);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x18));
            c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
            call_ptr(c, r32(c, A3), 0x002d_9240)?;

            // setPos(view[0..3]).
            let view = Addr(r32(c, S0));
            c.g[V0][0] = Addr(gfx.wrapping_add(VTABLE)).lw(c);
            c.f[12] = view.read_fpu(c).0;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xcc)).lw(c);
            c.f[13] = view.off(0x4).read_fpu(c).0;
            c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
            c.f[14] = view.off(0x8).read_fpu(c).0;
            call_ptr(c, r32(c, V1), 0x002d_9260)?;

            // setRot(view + 0x10 floats).
            c.g[V0][0] = Addr(gfx.wrapping_add(VTABLE)).lw(c);
            c.g[A1][0] = sx(view.0.wrapping_add(0x40));
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x110)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x114)).lw(c);
            c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
            call_ptr(c, r32(c, V1), 0x002d_9278)?;

            let on = Addr(gp.wrapping_add(ON)).lw(c) != 0;
            c.g[V0][0] = 0x54_0000;
            if on {
                c.cycles += 3;
                c.g[S0][0] = sx(SLOTS);
                c.g[S2][0] = sx(SLOTS + SLOT_SIZE * SLOT_COUNT);
                loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    RENDER_ACTIVE.call(c, 0x002d_9298)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(SLOT_SIZE));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_9290);
                    }
                }
            }
            // Pop.
            c.cycles += 3;
            c.g[V0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(STATE_SIZE));
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------ PS2BezierMan_BuildPatchPacket

/// `void PS2BezierMan_BuildPatchPacket(PS2BezierMan* bm, int type, int idx)`
/// (0x38c788): write the VIF packet that uploads patch cache slot `idx` of
/// lod `type` (0..2: an n x n grid with n = 2 * type + 4) to VU1: a header
/// (STCYCL / unpack setup constants), then three UNPACK transfers of the
/// slot's control points (+0x58/+0x5c/+0x60, 0xc0/0x1b0/0x300 bytes per
/// slot), normals (+0x64.., 0x80/0x120/0x200) and colours (+0x70..), and a
/// closing DMA ret. The packet lives at `(bm->packets[type] (+0x44c) +
/// idx * 0x100) | 0x30000000` (uncached-accelerated). Other types use null
/// sources. Leaf.
pub fn ps2_bezier_man_build_patch_packet(c: &mut Cpu) {
    /// Source tables per lod.
    const POINTS: u32 = 0x58;
    const NORMALS: u32 = 0x64;
    const COLOURS: u32 = 0x70;
    const PACKETS: u32 = 0x44c;

    c.cycles += 5;
    let bm = c.g[A0][0];
    c.g[A3][0] = bm;
    let bm = bm as u32;
    let ty = c.g[A1][0];
    c.g[V1][0] = sx((ty as u32) << 1);
    c.g[V0][0] = 1;
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(4)); // n
    let field = |t: u32| bm.wrapping_add(t);

    // Control points -> t2.
    let lod = if ty == 1 {
        Some((1u32, 0x1b0u64))
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((ty as i64) < 2) as u64;
        c.g[V0][0] = 2;
        c.cycles += 2;
        c.g[T2][0] = 0;
        if (ty as i64) < 2 {
            if ty == 0 {
                Some((0, 0xc0))
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                None
            }
        } else if ty == 2 {
            Some((2, 0x300))
        } else {
            c.cycles += 2;
            c.g[V0][0] = 1;
            None
        }
    };
    if let Some((t, stride)) = lod {
        c.cycles += if t == 2 { 2 } else { 3 };
        c.g[V0][0] = stride;
        c.g[V1][0] = Addr(field(POINTS + 4 * t)).lw(c);
        c.cycles += 3;
        c.mult(0, V0, A2, V0);
        c.g[T2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[V0][0] = 1;
    }

    // Normals -> t3; colours -> t8 (same shape, different tables).
    for (dst, table) in [(T3, NORMALS), (T8, COLOURS)] {
        c.cycles += 2;
        c.g[V0][0] = ((ty as i64) < 2) as u64;
        let found = if ty == 1 {
            c.cycles += 5;
            c.g[V0][0] = 0x120;
            c.g[V1][0] = Addr(field(table + 4)).lw(c);
            c.mult(0, V0, A2, V0);
            c.g[dst][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            true
        } else {
            c.cycles += 2;
            let low = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            c.cycles += 2;
            c.g[dst][0] = 0;
            let (hit, k, shift) = if low {
                (ty == 0, 0, 7)
            } else {
                (ty == 2, 2, 9)
            };
            if hit {
                c.cycles += if k == 0 { 4 } else { 3 };
                c.g[V1][0] = Addr(field(table + 4 * k)).lw(c);
                c.g[V0][0] = sx(r32(c, A2) << shift);
                c.g[dst][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            } else {
                c.cycles += 2;
            }
            hit
        };
        if found {
            c.cycles += 1;
        }
        if dst == T3 {
            c.g[V0][0] = 1;
        } else {
            c.mult(0, A0, A0, A0); // n * n
        }
    }

    // The packet.
    c.cycles += 82;
    c.g[V0][0] = sx(bm.wrapping_add(r32(c, A1) << 2));
    c.g[T1][0] = 0x3000_0000;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(PACKETS)).lw(c);
    c.g[A2][0] = sx(r32(c, A2) << 8);
    c.g[A1][0] = 0x7800_0000;
    c.g[A3][0] = 0x7400_0000;
    let nn = r32(c, A0);
    c.g[V0][0] = sx(nn << 1);
    c.g[T0][0] = sx(((nn as i32) >> 1) as u32);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(nn));
    c.g[T2][0] <<= 32;
    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
    c.g[A0][0] = sx(nn << 16);
    c.g[V0][0] |= c.g[T1][0];
    c.g[T0][0] |= c.g[T1][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    c.g[T2][0] |= c.g[V0][0];
    c.g[A1][0] |= 0x8000;
    c.g[A3][0] |= 0x8040;
    c.g[V1][0] |= c.g[T1][0];
    c.g[A1][0] |= c.g[A0][0];
    c.g[A3][0] |= c.g[A0][0];
    c.g[V0][0] = 0x1000_0003;
    c.g[T3][0] <<= 32;
    let p = r32(c, V1);
    c.sd(p, c.g[V0][0]);
    c.g[T3][0] |= c.g[T0][0];
    c.g[T6][0] = 0x1000_0001;
    c.g[T7][0] = 0x5050_5050_2000_0000;
    c.g[A1][0] <<= 32;
    c.g[A3][0] <<= 32;
    c.g[T1][0] = 0x3000_0000_0500_0000;
    c.g[T4][0] = 0x3f80_0000_3f80_0000;
    c.g[T5][0] = 0x4040_4040_2000_0000;
    c.g[A2][0] = 0x7400_8080;
    c.g[V0][0] = c.g[T8][0] << 32;
    c.g[A0][0] |= c.g[A2][0];
    c.g[T0][0] |= c.g[V0][0];
    c.g[A0][0] <<= 32;
    c.g[V0][0] = 0x6000_0000;
    // Header and the points / normals transfers.
    let words: [(u32, u64); 15] = [
        (0x08, 0),
        (0x10, c.g[T1][0]),
        (0x20, c.g[T4][0]),
        (0x28, c.g[T5][0]),
        (0x18, 0),
        (0x38, c.g[A1][0]),
        (0x40, c.g[T2][0]),
        (0x68, c.g[A3][0]),
        (0x70, c.g[T3][0]),
        (0x30, 0),
        (0x48, 0),
        (0x50, c.g[T6][0]),
        (0x58, 0),
        (0x60, c.g[T7][0]),
        (0x78, 0),
    ];
    for &(off, v) in &words {
        c.sd(p.wrapping_add(off), v);
    }
    // The colours transfer.
    let p = p.wrapping_add(0x80);
    c.g[V1][0] = sx(p);
    c.sd(p, c.g[T6][0]);
    c.sd(p.wrapping_add(0x8), 0);
    c.sd(p.wrapping_add(0x10), c.g[T7][0]);
    c.sd(p.wrapping_add(0x18), c.g[A0][0]);
    c.sd(p.wrapping_add(0x20), c.g[T0][0]);
    c.sd(p.wrapping_add(0x28), 0);
    // DMA ret.
    let p = p.wrapping_add(0x30);
    c.g[V1][0] = sx(p);
    c.sd(p, c.g[V0][0]);
    c.sd(p.wrapping_add(0x8), 0);
    ret(c);
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_a6a8, snd_dsp_clip16_process, "SND_DspClip16_Process"),
    (
        0x0037_cbc8,
        ps2_graphics_man_set_tex_function,
        "PS2GraphicsMan_SetTexFunction",
    ),
    (0x003d_5128, path_update, "PATH_Update"),
    (
        0x0038_d968,
        ps2_bezier_man_render_patch_fx,
        "PS2BezierMan_RenderPatchFX",
    ),
    (
        0x0011_61d0,
        rider_try_enter_board_press,
        "Rider_TryEnterBoardPress",
    ),
    (0x003b_96b8, sndmidi_service, "SNDMIDI_Service"),
    (0x002e_2868, lens_fx_draw_flare_quad, "LensFX_DrawFlareQuad"),
    (0x002d_9130, vfx_avalanche_render, "VfxAvalanche_Render"),
    (
        0x0038_c788,
        ps2_bezier_man_build_patch_packet,
        "PS2BezierMan_BuildPatchPacket",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, put_words, scramble, Rng, DATA};

    /// `SND_DspClip16_Process`: with and without an upstream stage (a fake
    /// that returns a positive count or unwinds, or a leaf returning 0), and 16..64 floats
    /// with values both inside and beyond the 16-bit range.
    #[test]
    fn snd_dsp_clip16_process_matches() {
        check(ee::s14::f_003ca6a8__generated, snd_dsp_clip16_process, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x3ca6a8, 1);
            c.pc = 0x003c_a6a8;
            let o = DATA;
            let stage = DATA + 0x100;
            let input = DATA + 0x1000;
            let out = DATA + 0x2000;
            c.bus.write32(o + 0x8, if seed % 3 == 0 { 0 } else { stage });
            c.bus.write8(o + 0x1a, r.next_u32() as u8);
            // The upstream's method: a fake (returns a positive pointer, or
            // unwinds) or a real `return 0` leaf (0x3ce198: nothing to clip).
            let method = if seed % 3 == 2 { 0x003c_e198 } else { fakes::FAKE };
            c.bus.write32(stage, method);
            let count = 16 * (1 + seed % 4);
            for i in 0..count {
                let v = match r.next_u32() % 4 {
                    0 => r.f32(100.0),
                    1 => r.f32(60000.0),
                    2 => r.edge_f32(),
                    _ => r.f32(1.0e9),
                };
                c.bus.write32(input + 4 * i, v);
            }
            fill(c, &mut r, out, 0x40);
            c.g[A0][0] = o as u64;
            c.g[A1][0] = count as u64;
            c.g[A2][0] = input as u64;
            c.g[A3][0] = out as u64;
        });
    }

    /// `PS2GraphicsMan_SetTexFunction` with the real `TexMan_SetTexFunction`:
    /// texture index -1 (ignored) or 0..7 (the texture's GS TEX0 word at
    /// +0x38 gets the function bits).
    #[test]
    fn ps2_graphics_man_set_tex_function_matches() {
        check(
            ee::s12::f_0037cbc8__generated,
            ps2_graphics_man_set_tex_function,
            60,
            &|c, seed| {
                let mut r = Rng(0x0037_cbc8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0037_cbc8;
                let g = DATA;
                let tex_man = DATA + 0x2000;
                c.bus.write32(g + 0x18f4, tex_man);
                for i in 0..8 {
                    let tex = DATA + 0x3000 + 0x40 * i;
                    c.bus.write32(tex_man + 8 + 4 * i, tex);
                    c.bus.write32(tex + 0x38, r.next_u32());
                    c.bus.write32(tex + 0x3c, r.next_u32());
                }
                c.g[A0][0] = g as u64;
                c.g[A1][0] = if seed % 4 == 0 {
                    u64::MAX
                } else {
                    (seed % 8) as u64
                };
                c.g[A2][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }

    /// `PATH_Update` with its real callees: locked / uninitialised / busy
    /// early outs, and the full tick over four players that are inactive,
    /// active without a handle, or active with a handle (an empty track
    /// list, so `PATH_SelectMasterTrack` and `PATH_UpdateTracks` take their
    /// short paths; no platform streams).
    #[test]
    fn path_update_matches() {
        check(ee::s14::f_003d5128__generated, path_update, 120, &|c, seed| {
            let mut r = Rng(0x003d_5128 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.pc = 0x003d_5128;
            let gate = seed % 6;
            c.bus.write8(0x0044_f42d, (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3));
            c.bus.write8(0x0044_f42c, (gate != 1) as u8);
            c.bus.write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
            c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
            c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
            for i in 0..4u32 {
                let p = 0x0051_7610 + 0x928 * i;
                let kind = r.next_u32() % 3;
                c.bus.write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                c.bus.write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
            }
            c.g[A0][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
            if seed % 2 == 0 {
                c.g[A0][0] = (c.g[A0][0] & !0xff) | b'B' as u64;
            }
        });
    }

    /// `PS2BezierMan_RenderPatchFX` with its real callees: FX off, stale view
    /// matrices (the real `UpdateViewMatrices` on random data), 0..3 type-8
    /// patches and 0..3 mixed patches of either kind sign. The patches'
    /// flag word (+0xc) has bit 23 clear, so `DrawPatchFX` returns at once
    /// after each scratchpad upload.
    #[test]
    fn ps2_bezier_man_render_patch_fx_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s12::f_0038d968__generated,
            ps2_bezier_man_render_patch_fx,
            120,
            &|c, seed| {
                let mut r = Rng(0x0038_d968 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0038_d968;
                let gfx = DATA;
                let top = DATA + 0x1000 + 4 * (seed % 4);
                let bm = DATA + 0x10000;
                let objs = DATA + 0x20000;
                let scratch = DATA + 0x30000;
                c.bus.write32(GAME_GP + 0x13f0, (seed % 7 == 0) as u32);
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(GAME_GP + 0x2a94, scratch);
                // View data for UpdateViewMatrices.
                for k in 0..0x40 {
                    c.bus.write32(gfx + 4 * k, r.f32(2.0));
                }
                // Render-state stack: top entry plus room for the push.
                c.bus.write32(gfx + 0xe84, top);
                fill(c, &mut r, top, 10);
                let sel = r.next_u32() % 4;
                c.bus.write32(gfx + 0x64, sel);
                fill(c, &mut r, gfx + 0xf50, 4);
                c.bus.write32(
                    gfx + 0x6b90,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                // Patches.
                let n8 = seed % 4;
                let nmix = (seed / 4) % 4;
                c.bus.write32(bm + 0x4a60, n8);
                c.bus.write32(bm + 0x4d84, nmix);
                for i in 0..4 {
                    let obj = objs + 0x200 * i;
                    c.bus.write32(obj + 0xc, r.next_u32() & !0x80_0000);
                    c.bus.write32(bm + 0x4a64 + 0x10 * i, obj);
                    c.bus.write32(bm + 0x4d88 + 0x14 * i, obj + 0x1000);
                    c.bus.write32(obj + 0x100c, r.next_u32() & !0x80_0000);
                    let kind = match r.next_u32() % 3 {
                        0 => 0,
                        1 => (r.next_u32() % 4) as i32 + 1,
                        _ => -(1 + (r.next_u32() % 4) as i32),
                    };
                    c.bus.write32(bm + 0x4d88 + 0x14 * i + 8, kind as u32);
                }
                c.g[A0][0] = bm as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `Rider_TryEnterBoardPress` with its real callees: dir 0 / positive /
    /// negative / edge floats, press states 3, 4 and others, main state 0
    /// (the VU0 alignment test, both signs) or not. `AnimController_PlayAnim`
    /// reaches its virtual call through a test vtable at a fake forced to
    /// return 0x207 (its early exit; sometimes it unwinds); `SetSubState`'s
    /// owner has a state id past the state tables.
    #[test]
    fn rider_try_enter_board_press_matches() {
        check(
            ee::s00::f_001161d0__generated,
            rider_try_enter_board_press,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1161d0, 1);
                c.pc = 0x0011_61d0;
                let rider = DATA;
                let owner = DATA + 0x1000;
                let ctl = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = rider as u64;
                c.f[12] = match seed % 5 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.f32(4.0) & !0x8000_0000,
                    3 => r.f32(4.0) | 0x8000_0000,
                    _ => r.edge_f32(),
                };
                let press = match (seed / 5) % 4 {
                    0 => 3,
                    1 => 4,
                    2 => r.next_u32(),
                    _ => 0,
                };
                c.bus.write32(rider + 0x328, press);
                c.bus.write32(rider + 0x77c, owner);
                let main = if (seed / 20) % 2 == 0 { 0 } else { 1 + r.next_u32() % 9 };
                c.bus.write32(owner + 0xde0, main);
                c.bus.write32(owner + 0xde4, 0xe + r.next_u32() % 8);
                for k in 0..4 {
                    c.bus.write32(rider + 0x1e0 + 4 * k, r.f32(3.0));
                    c.bus.write32(rider + 0x3a0 + 4 * k, r.f32(30.0));
                }
                c.bus.write32(rider + 0x784, ctl);
                c.bus.write32(ctl + 0x58, vt);
                c.bus.write16(vt + 0x20, 0x10);
                c.bus.write32(vt + 0x24, fakes::FAKE);
                c.bus.write32(fakes::LOG_RESULTS, 0x207);
            },
        );
    }

    /// `SNDMIDI_Service` with its real callees over 16 slots: playing or
    /// not; no fade, a fade that keeps playing (no voices of the slot, so
    /// no volume calls) or one that ends (stop with an invalid handle,
    /// returns at once); scaled time before, at or 1..3 events past the next
    /// event. Events are program changes behind a non-zero delta counter
    /// (+0x27), so `SNDMIDI_ProcessEvent` only counts down.
    #[test]
    fn sndmidi_service_matches() {
        check(ee::s13::f_003b96b8__generated, sndmidi_service, 100, &|c, seed| {
            let mut r = Rng(0x003b_96b8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.pc = 0x003b_96b8;
            // Voice table: no voice belongs to any slot.
            for v in 0..24 {
                c.bus.write8(0x0050_a820 + 8 * v + 4, 0xff);
            }
            let events = DATA;
            put_words(c, events, &[0x0005_c000]);
            for i in 0..16u32 {
                let e = 0x0050_a120 + 0x70 * i;
                let k = r.next_u32();
                let playing = k % 5 != 0;
                c.bus.write8(e + 0x28, if playing { 3 } else { (k % 3) as u8 });
                match (k >> 4) % 3 {
                    0 => c.bus.write32(e + 0x18, 0),
                    1 => {
                        // Fading up, still below the target.
                        c.bus.write32(e + 0x18, 5);
                        c.bus.write32(e + 0x14, 10);
                        c.bus.write32(e + 0x1c, 100);
                    }
                    _ => {
                        // Fading out to -1: stop.
                        c.bus.write32(e + 0x18, (-5i32) as u32);
                        c.bus.write32(e + 0x14, 2);
                        c.bus.write32(e + 0x1c, u32::MAX);
                        c.bus.write32(e + 0x10, u32::MAX);
                    }
                }
                let ticks = r.next_u32() % 1000;
                let tempo = r.next_u32() % 200;
                c.bus.write32(e, ticks);
                c.bus.write16(e + 0x24, tempo as u16);
                let scaled = ticks * tempo / 100;
                let next = match (k >> 8) % 5 {
                    0 => scaled + 1 + r.next_u32() % 10,
                    n => scaled.wrapping_sub(n - 1),
                };
                c.bus.write32(e + 0x4, next);
                c.bus.write32(e + 0x8, events);
                c.bus.write8(e + 0x27, 0xff);
            }
        });
    }

    /// `LensFX_DrawFlareQuad`: off / on, zero or random velocity, with or
    /// without per-sprite alpha (positive, zero, negative), type bits 0x10,
    /// 0x20, 0x40 and others; the sprite draw method is a fake (sometimes
    /// unwinding) that logs the registers and the stack arguments.
    #[test]
    fn lens_fx_draw_flare_quad_matches() {
        use crate::difftest::GAME_GP;
        check(ee::s09::f_002e2868__generated, lens_fx_draw_flare_quad, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x2e2868, 1);
            c.pc = 0x002e_2868;
            let fx = DATA;
            let part = DATA + 0x100;
            let gfx = DATA + 0x1000;
            let top = DATA + 0x3000;
            let vt = DATA + 0x4000;
            c.bus.write16(fx + 0x2c, (seed % 9 != 0) as u16 * (1 + r.next_u32() as u16 % 3));
            c.bus.write16(fx + 0x2e, (seed % 2) as u16);
            c.bus.write32(fx + 0x20, part);
            c.bus.write32(fx + 0x28, r.f32(6.3));
            for k in 0..3 {
                let v = if seed % 7 == 3 { 0 } else { r.f32(50.0) };
                c.bus.write32(part + 0x10 + 4 * k, v);
                c.bus.write32(part + 0x1c + 4 * k, r.f32(1000.0));
            }
            let idx = r.next_u32() % 4;
            c.g[A1][0] = idx as u64 | (r.next_u32() as u64) << 32;
            for k in 0..4 {
                let a = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.f32(1.0) | 0x8000_0000,
                    _ => r.f32(1.0) & !0x8000_0000,
                };
                c.bus.write32(part + 0x40 + 4 * k, a);
            }
            let kind = [0x10, 0x20, 0x40, 0x00, 0x30, 0x50, 0x60, 0x70][(seed / 2 % 8) as usize];
            c.bus.write32(part + 0xc, (r.next_u32() & !0x70) | kind);
            c.bus.write32(GAME_GP + 0x2a90, gfx);
            c.bus.write32(gfx + 0xe84, top);
            fill(c, &mut r, top, 8);
            c.bus.write32(gfx + 0x1024, r.next_u32());
            c.bus.write32(gfx + 0x1028, r.next_u32());
            c.bus.write32(gfx + 0x10d8, vt);
            c.bus.write16(vt + 0x278, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + 0x27c, fakes::FAKE);
            c.g[A0][0] = fx as u64;
        });
    }

    /// `VfxAvalanche_Render`: on/visible/forced combinations (both skip and
    /// draw paths); the manager's three vtable methods are fakes (each may
    /// unwind); the 16 avalanche slots are inactive, so the real
    /// `Avalanche_RenderActive` returns at once.
    #[test]
    fn vfx_avalanche_render_matches() {
        use crate::difftest::GAME_GP;
        check(ee::s09::f_002d9130__generated, vfx_avalanche_render, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x2d9130, 3);
            c.pc = 0x002d_9130;
            let world = DATA;
            let cams = DATA + 0x100;
            let cam = DATA + 0x200;
            let gfx = DATA + 0x1000;
            let top = DATA + 0x3000 + 4 * (seed % 4);
            let vt = DATA + 0x4000;
            let view = DATA + 0x5000;
            let bits = seed % 8;
            c.bus.write32(GAME_GP + 0x940, (bits & 1) * (1 + r.next_u32() % 5));
            c.bus.write32(GAME_GP + 0x9cc, (bits >> 1) & 1);
            c.bus.write32(GAME_GP + 0x10fc, (bits >> 2) & 1);
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, cams);
            c.bus.write32(cams + 0x84, cam);
            let cur = r.next_u32() % 3;
            c.bus.write32(cam + 0x14, cur);
            c.bus.write32(cam + 4 + 4 * cur, view);
            fill(c, &mut r, view, 0x20);
            c.bus.write32(GAME_GP + 0x2a90, gfx);
            c.bus.write32(gfx + 0xe84, top);
            fill(c, &mut r, top, 10);
            fill(c, &mut r, 0x0050_1420, 5);
            c.bus.write32(gfx + 0x10d8, vt);
            for (i, slot) in [0xc0u32, 0xc8, 0x110].into_iter().enumerate() {
                c.bus.write16(vt + slot, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + slot + 4, fakes::FAKE + 0x10 * i as u32);
            }
        });
    }

    /// `PS2BezierMan_BuildPatchPacket`: lod types 0, 1, 2 and out of range
    /// (negative, 3+), random slot indices and table bases.
    #[test]
    fn ps2_bezier_man_build_patch_packet_matches() {
        check(
            ee::s12::f_0038c788__generated,
            ps2_bezier_man_build_patch_packet,
            200,
            &|c, seed| {
                let mut r = Rng(0x0038_c788 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0038_c788;
                let bm = DATA;
                fill(c, &mut r, bm + 0x58, 9);
                for t in 0..3 {
                    c.bus.write32(bm + 0x44c + 4 * t, DATA + 0x10000 + 0x4000 * t);
                }
                let ty: i64 = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => -1 - (r.next_u32() % 4) as i64,
                    4 => 3 + (r.next_u32() % 4) as i64,
                    _ => (seed % 3) as i64,
                };
                c.g[A0][0] = bm as u64;
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = (r.next_u32() % 32) as u64;
                // Out-of-range types read their packet pointer from the
                // tables around +0x44c; keep it inside the test area.
                for t in -5i32..8 {
                    if !(0..3).contains(&t) {
                        let a = (bm as i32 + 0x44c + 4 * t) as u32;
                        c.bus.write32(a, DATA + 0x20000 + 0x800 * (t + 5) as u32);
                    }
                }
            },
        );
    }
}
