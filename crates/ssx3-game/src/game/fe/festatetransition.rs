//! `src/fe/festatetransition.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2 as sp;
pub(crate) use crate::game::main::gamemode::sw_r;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_4528,
        c_fe_state_background_init,
        "cFEStateBackground_Init",
    ),
    (
        0x0019_4588,
        c_fe_state_transition_init,
        "cFEStateTransition_Init",
    ),
    (
        0x0019_4600,
        cfe_state_transition_v16,
        "cFEStateTransition_v16",
    ),
    (
        0x0019_46a8,
        cfe_state_transition_start_fade,
        "cFEStateTransition_StartFade",
    ),
    (0x0019_4738, c_fe_state_title_ctor, "cFEStateTitle_ctor"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {

    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;

    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;

    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;

    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
}

/// `cFEStateTitle* ctor(this)` (0x194738): base `UIState_ctor`, vtable
/// `D_0046BAB8` at `+0x8`, zero the id at `+0xc` and the field at `+0x48`.
pub fn c_fe_state_title_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0019_474c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x0046_bab8);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x48), 0);
        c.sw((this as u32).wrapping_add(0x8), vt);
        c.sw((this as u32).wrapping_add(0xc), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBackground_Init(cFEStateBackground* self)` (0x194528):
/// create the background screen's widget tree.
pub fn c_fe_state_background_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = self_;
        c.g[A0][0] = 0x0045_ffd8;
        c.g[S1][0] = Addr(at(c, S0, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_454c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_4560)?;
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_4574)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTransition_onCreateScreen(self)` (0x194588): create the two
/// transition screens by name hash.
pub fn c_fe_state_transition_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = lw_r(c, S1, 0x10);
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x18));
        call_ptr(c, callee::STR_HASH, 0x0019_45ac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x0019_45c0)?;
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = lw_r(c, S1, 0x10);
        sw_r(c, S1, 0x48, c.g[V0][0]);
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x8));
        call_ptr(c, callee::STR_HASH, 0x0019_45d4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x0019_45e8)?;
        sw_r(c, S1, 0x4c, c.g[V0][0]);
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int cFEStateTransition_StartFade(void* self, int which)` (0x1946a8):
/// mark one of the two screens (`self + 0x48` for 0, `self + 0x4c` for 1) as
/// fading, play its next frame, and for 0 also kick the audio trigger 0xf.
pub fn cfe_state_transition_start_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        let which = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        if which != 0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let not_one = which != 1;
            frame.restore_ra(c, 0);
            let mut reloaded = false;
            if !not_one {
                c.cycles += 3;
                c.g[V1][0] = c.lw(p(self_, 0x4c));
                let empty = c.g[V1][0] == 0;
                c.g[A1][0] = 0;
                if empty {
                    frame.restore_ra(c, 0);
                    reloaded = true;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at_2(c, V1, 0x54));
                    c.g[A2][0] = 0;
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw(at_2(c, V1, 0x54), c.g[V0][0]);
                    c.g[A0][0] = c.lw(p(self_, 0x4c));
                    call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x0019_4724)?;
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                    reloaded = true;
                }
            }
            let _ = reloaded;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.lw(p(self_, 0x48));
            let empty = c.g[V1][0] == 0;
            c.g[A1][0] = 0;
            if empty {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 13;
                c.g[V0][0] = c.lw(at_2(c, V1, 0x54));
                c.g[A2][0] = 0;
                c.g[V0][0] = c.g[V0][0] | 0x40;
                c.sw(at_2(c, V1, 0x54), c.g[V0][0]);
                c.g[A0][0] = c.lw(p(self_, 0x48));
                call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x0019_46d8)?;
                call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0019_46e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xf;
                call_ptr(c, callee::AUDIO_SET_TRIGGER, 0x0019_46ec)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateTransition_v16(self, a1, a2)` (0x194600): if the transition
/// id matches the state's, clear or set the pending bits on the two objects.
pub fn cfe_state_transition_v16(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            'mid: {
                'inner: {
                    c.cycles += 18;
                    frame.save(c, S1, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x20);
                    frame.save(c, S3, 0x10);
                    c.g[A0][0] = sx(0x004a_0000);
                    c.g[A0][0] = sx(0x004a_1360);
                    frame.save(c, S0, 0x40);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0019_4630);
                    c.g[S3][0] = c.g[A2][0];
                    call_ptr(c, 0x0031_7670, 0x0019_4630)?;
                    c.g[A0][0] = sx(0x004a_0000);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0019_4640);
                    c.g[A0][0] = sx(0x004a_17b8);
                    call_ptr(c, 0x0031_7670, 0x0019_4640)?;
                    let t = c.g[S3][0] != c.g[S0][0];
                    c.g[A0][0] = sx(0xffff_ffbf);
                    if t {
                        break 'inner;
                    }
                    c.cycles += 8;
                    c.g[V1][0] = lw(c, at_3(c.g[S1][0], 0x1c));
                    c.g[V0][0] = sx(0xffff_fff7);
                    c.g[V1][0] &= c.g[V0][0];
                    Addr(at_3(c.g[S1][0], 0x1c)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = lw(c, at_3(c.g[S2][0], 0x54));
                    c.g[V0][0] &= c.g[A0][0];
                    Addr(at_3(c.g[S2][0], 0x54)).sw(c, c.g[V0][0]);
                    break 'mid;
                }
                c.cycles += 2;
                let t = c.g[S3][0] != c.g[V0][0];
                frame.restore(c, S0, 0x40);
                if t {
                    break 'end;
                }
                c.cycles += 6;
                c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x1c));
                c.g[V0][0] |= 8;
                Addr(at_3(c.g[S1][0], 0x1c)).sw(c, c.g[V0][0]);
                c.g[V1][0] = lw(c, at_3(c.g[S2][0], 0x54));
                c.g[V1][0] |= 0x40;
                Addr(at_3(c.g[S2][0], 0x54)).sw(c, c.g[V1][0]);
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
            break 'end;
        }
        c.cycles += 7;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) use crate::game::bx::bxstring::tests::heap_init;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_3;

    pub(crate) use crate::game::fe::festateoptions::tests::fe_state_setup;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::w as w_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn c_fe_state_title_ctor_matches() {
        check(
            ssx3_ee::s02::f_00194738__generated,
            c_fe_state_title_ctor,
            24,
            &|c, seed| {
                fe_state_setup(c, seed, 0x1947_38);
            },
        );
    }

    #[test]
    fn c_fe_state_background_init_matches() {
        check(
            ssx3_ee::s02::f_00194528__generated,
            c_fe_state_background_init,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x194528, 0);
                w(c, OBJ + 0x10, if seed % 2 == 0 { 0 } else { OBJ2 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_transition_init_matches() {
        check(
            ee::s02::f_00194588__generated,
            c_fe_state_transition_init,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0x194588, 2);
                be_library(c);
                heap_init(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w_2(c, obj + 0x10, DATA + 0x1000);
                w_2(c, DATA + 0x1000, 0);
            },
        );
    }

    #[test]
    fn cfe_state_transition_start_fade_matches() {
        check(
            ee::s02::f_001946a8__generated,
            cfe_state_transition_start_fade,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1946, 0);
                env(c);
                let this = DATA + 0x400;
                let which = (seed % 3) as u64;
                let screen_a = if seed % 2 == 0 { 0 } else { DATA + 0x800 };
                let screen_b = if seed % 4 < 2 { 0 } else { DATA + 0x900 };
                w32(c, this + 0x48, screen_a);
                w32(c, this + 0x4c, screen_b);
                for s in [DATA + 0x800, DATA + 0x900] {
                    w32(c, s + 0x54, 0x10);
                    w32(c, s + 0x4c, 0);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = which;
            },
        );
    }

    // --------------------------------------------------- state transition

    #[test]
    fn cfe_state_transition_v16_matches() {
        check(
            ee::s02::f_00194600__generated,
            cfe_state_transition_v16,
            SEEDS_3,
            &|c, seed| {
                let _ = base(c, seed, 0x1946_00, 4);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_1360u64]);
                c.g[A2][0] = if seed % 2 == 0 { hash } else { hash ^ 0x55 };
                w_3(c, obj + 0x1c, (seed as u32) & 0xff);
                w_3(c, obj + 0x1000 + 0x54, (seed.wrapping_mul(3)) & 0xff);
            },
        );
    }
}
