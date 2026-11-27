//! Staging module oc1070 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1070`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_7ea8,
        c_speed_boost_streamer_fx_reset_thunk,
        "cSpeedBoostStreamerFX_ResetThunk",
    ),
    (
        0x002f_8060,
        transition_effect_fade_ctor,
        "TransitionEffect_FadeCtor",
    ),
    (
        0x002f_81b8,
        c_cut_scene_transition_man_dtor,
        "cCutSceneTransitionMan_dtor",
    ),
    (
        0x002f_8370,
        visual_effects_man_global_ctors,
        "VisualEffectsMan_GlobalCtors",
    ),
    (
        0x002f_8390,
        visual_effects_man_global_dtors,
        "VisualEffectsMan_GlobalDtors",
    ),
    (
        0x002f_9818,
        attached_emitter_type_data_global_ctors1,
        "AttachedEmitterTypeData_GlobalCtors1",
    ),
    (
        0x002f_ae18,
        attached_emitter_type_data_global_ctors2,
        "AttachedEmitterTypeData_GlobalCtors2",
    ),
    (
        0x0036_7230,
        tex_man_destroy_nb_tex_info_array,
        "TexMan_DestroyNBTexInfoArray",
    ),
];

// ---------------------------------------------------------------------------
// cSpeedBoostStreamerFX::ResetThunk (0x2f7ea8)

/// `void cSpeedBoostStreamerFX_ResetThunk(void)` (0x2f7ea8): forwards to
/// `cSpeedBoostStreamerFX_Reset()` (0x2e6640) in gcc's 7-cycle wrapper shape.
pub fn c_speed_boost_streamer_fx_reset_thunk(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x002e_6640,
            f: ee::s09::f_002e6640,
        }),
        0x002f_7eb8,
    );
}

// ---------------------------------------------------------------------------
// TransitionEffect_FadeCtor (0x2f8060): cFade_TObj::cFade_TObj(this, colour)

/// `cFade_TObj::cFade_TObj(this, colour)` (0x2f8060): runs
/// `TransitionEffect_ctor(this)` (0x2e4d70) first, with the two halves of the
/// 16-byte colour block at `colour` left in v0/v1 (the registers the original
/// holds at that call), then writes the fade vtable word (0x4880c0) at `this`
/// and copies the colour block to `this + 4 .. this + 0x14`. The colour
/// block is staged on the stack through unaligned doubleword halves
/// (`ldl`/`ldr`, `sdl`/`sdr`).
pub fn transition_effect_fade_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);

        // Copy colour[0..16] to the stack buffer at sp+0.
        let colour = c.g[A1][0] as u32;
        c.ldl(2, colour.wrapping_add(0x7));
        c.ldr(2, colour);
        c.ldl(3, colour.wrapping_add(0xf));
        c.ldr(3, colour.wrapping_add(0x8));
        let buf = frame.at(c, 0);
        c.sdl(2, buf.wrapping_add(0x7));
        c.sdr(2, buf);
        c.sdl(3, buf.wrapping_add(0xf));
        c.sdr(3, buf.wrapping_add(0x8));

        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002e_4d70,
            f: ee::s09::f_002e4d70,
        }
        .call(c, 0x002f_8098)?;

        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x7f40));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        let this = c.g[S0][0] as u32;
        c.sw(this, c.g[V1][0]);

        // Copy the stack buffer to this + 4 .. this + 0x14.
        c.ldl(3, buf.wrapping_add(0x7));
        c.ldr(3, buf);
        c.ldl(4, buf.wrapping_add(0xf));
        c.ldr(4, buf.wrapping_add(0x8));
        c.sdl(3, this.wrapping_add(0xb));
        c.sdr(3, this.wrapping_add(0x4));
        c.sdl(4, this.wrapping_add(0x13));
        c.sdr(4, this.wrapping_add(0xc));

        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cCutSceneTransitionMan dtor (0x2f81b8)

/// `cCutSceneTransitionMan::~cCutSceneTransitionMan()` (0x2f81b8): forwards
/// to `GameProcess_dtor` (0x3546c8) through the function table (gcc's 7-cycle
/// wrapper shape with `jalr`).
pub fn c_cut_scene_transition_man_dtor(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, Target::Table(0x0035_46c8), 0x002f_81c8);
}

// ---------------------------------------------------------------------------
// Static constructor and destructor wrappers (0x2f8370 .. 0x2fae18)

/// The 8-cycle static-init wrappers: `StaticInit(a0, 0xffff)` in a 0x10
/// frame, returning what the callee leaves.
fn static_init_forward(c: &mut Cpu, a0: u64, callee: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = a0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        callee.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffectsMan_GlobalCtors` (0x2f8370): `VisualEffectsMan_StaticInitDestroy(1, 0xffff)`.
pub fn visual_effects_man_global_ctors(c: &mut Cpu) {
    static_init_forward(
        c,
        1,
        Callee {
            addr: 0x002f_6b88,
            f: ee::s09::f_002f6b88,
        },
        0x002f_8384,
    );
}

/// `VisualEffectsMan_GlobalDtors` (0x2f8390): `VisualEffectsMan_StaticInitDestroy(0, 0xffff)`.
pub fn visual_effects_man_global_dtors(c: &mut Cpu) {
    static_init_forward(
        c,
        0,
        Callee {
            addr: 0x002f_6b88,
            f: ee::s09::f_002f6b88,
        },
        0x002f_83a4,
    );
}

/// `AttachedEmitterTypeData_GlobalCtors1` (0x2f9818): `AttachedEmitterTypeData_StaticInit1(1, 0xffff)`.
pub fn attached_emitter_type_data_global_ctors1(c: &mut Cpu) {
    static_init_forward(
        c,
        1,
        Callee {
            addr: 0x002f_9040,
            f: ee::s09::f_002f9040,
        },
        0x002f_982c,
    );
}

/// `AttachedEmitterTypeData_GlobalCtors2` (0x2fae18): `AttachedEmitterTypeData_StaticInit2(1, 0xffff)`.
pub fn attached_emitter_type_data_global_ctors2(c: &mut Cpu) {
    static_init_forward(
        c,
        1,
        Callee {
            addr: 0x002f_a640,
            f: ee::s10::f_002fa640,
        },
        0x002f_ae2c,
    );
}

// ---------------------------------------------------------------------------
// TexMan_DestroyNBTexInfoArray (0x367230)

/// `TexMan_DestroyNBTexInfoArray` (0x367230): a countdown over 0x1f3 slots
/// with no memory effect and no callees. The loop variable starts at 498
/// and ends at -2 (the last test sees -1); each back edge takes an event
/// check, and the exit adds 2 cycles.
pub fn tex_man_destroy_nb_tex_info_array(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(0x1f3u32.wrapping_sub(1));
    loop {
        c.cycles += 7;
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0036_7238);
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn c_speed_boost_streamer_fx_reset_thunk_matches() {
        check(
            ee::s09::f_002f7ea8__generated,
            c_speed_boost_streamer_fx_reset_thunk,
            150,
            &|c, seed| {
                let mut r = Rng(0x2f7e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(
                    0x870,
                    if seed % 2 == 0 {
                        r.next_u32()
                    } else {
                        r.next_u32() | 0x8000_0000
                    },
                );
            },
        );
    }

    #[test]
    fn transition_effect_fade_ctor_matches() {
        check(
            ee::s09::f_002f8060__generated,
            transition_effect_fade_ctor,
            150,
            &|c, seed| {
                let mut r = Rng(0x2f80 ^ seed as u64);
                scramble(c, &mut r);
                let src = DATA + 0x200;
                for k in 0..8 {
                    c.bus.write32(src + 4 * k, r.next_u32());
                }
                // The colour block may start unaligned: cover 0..6 byte offsets.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (src + (seed % 3) * 3) as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_dtor_matches() {
        check(
            ee::s09::f_002f81b8__generated,
            c_cut_scene_transition_man_dtor,
            60,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f81, 0);
                // `this` at DATA+0x100 with its process-list links; an odd
                // flag takes the delete path.
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let links = seed % 3 != 0;
                c.bus
                    .write32(DATA + 0x104, if links { DATA + 0x200 } else { 0 });
                c.bus
                    .write32(DATA + 0x108, if links { DATA + 0x280 } else { 0 });
            },
        );
    }

    #[test]
    fn visual_effects_man_global_ctors_matches() {
        check(
            ee::s09::f_002f8370__generated,
            visual_effects_man_global_ctors,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f83, 0);
            },
        );
    }

    #[test]
    fn visual_effects_man_global_dtors_matches() {
        check(
            ee::s09::f_002f8390__generated,
            visual_effects_man_global_dtors,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f84, 0);
            },
        );
    }

    #[test]
    fn attached_emitter_type_data_global_ctors1_matches() {
        check(
            ee::s09::f_002f9818__generated,
            attached_emitter_type_data_global_ctors1,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2f98, 0);
            },
        );
    }

    #[test]
    fn attached_emitter_type_data_global_ctors2_matches() {
        check(
            ee::s10::f_002fae18__generated,
            attached_emitter_type_data_global_ctors2,
            8,
            &|c, seed| {
                fakes::machine_rt(c, seed, 0x2fae, 0);
            },
        );
    }

    #[test]
    fn tex_man_destroy_nb_tex_info_array_matches() {
        check(
            ee::s12::f_00367230__generated,
            tex_man_destroy_nb_tex_info_array,
            20,
            &|c, seed| {
                let mut r = Rng(0x3672 ^ seed as u64);
                scramble(c, &mut r);
                // Events at a few points of the countdown (seed 0: at every back edge).
                c.next_event = (seed as u64 % 8) * 700;
            },
        );
    }
}
