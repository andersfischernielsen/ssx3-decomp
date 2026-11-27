//! Staging module oc1016 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1016`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_2648, luno_vm_call_with_arg, "LunoVM_CallWithArg"),
    (
        0x0023_3ad0,
        game_module_request_overlay_state_global,
        "GameModule_RequestOverlayStateGlobal",
    ),
    (0x0023_3b28, game_state_load_state, "GameState_LoadState"),
    (
        0x0023_3bb0,
        game_state_quit_load_state,
        "GameStateQuit_LoadState",
    ),
    (
        0x0023_3bf0,
        game_state_idle_load_state,
        "GameStateIdle_LoadState",
    ),
    (
        0x0024_5950,
        c_fe_load_hint_state_init,
        "cFELoadHintState_Init",
    ),
    (
        0x0024_5f30,
        c_fe_state_progressive_verify_ps2_dtor,
        "cFEStateProgressiveVerifyPS2_dtor",
    ),
    (
        0x0024_6098,
        c_fe_state_progressive_verify_ps2_update,
        "cFEStateProgressiveVerifyPS2_Update",
    ),
];

/// `LunoVM_CallWithArg(out, state, table, value, retVar)` (0x222648): the
/// Luno VM's register table for one call. Allocates `n` registers of 12 bytes
/// (`new (str_LunoVMRegister, 0x20000000, 0) LV[n]`) with `n` read from the
/// chunk the state points at, stores the table into register 0, the value
/// into register 1 (a table is copied, a function pair is shared, anything
/// else is the raw word), and runs `LunoVM_Execute(out, state, retVar)`.
pub fn luno_vm_call_with_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = 0xc;
        frame.save(c, 17, 0x70);
        frame.save(c, 19, 0x50);
        frame.save(c, 20, 0x40);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, 21, 0x30);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 22, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, 23, 0x10);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, 16, 0x80);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, 18, 0x60);
        c.g[S7][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x6880));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        // n = *(chunk + *(chunk + 4) + 8), chunk = *state.
        c.g[V0][0] = c.lw(c.g[S6][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.mult(0, V1, S0, V1);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7e30, 0x0022_26b4)?;
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        let header = c.g[V0][0] as u32;
        c.sw(header, c.g[S0][0]);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        let t = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !t {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
            loop {
                c.cycles += 7;
                let entry = c.g[V0][0] as u32;
                c.sw(entry.wrapping_add(0x8), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.sw(entry, 0);
                let more = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_26d0);
                }
            }
        }
        // Register 0 (the table): its type word decides whether the
        // destructor runs first.
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[S2][0] = 0x4;
        let differs = c.g[V0][0] != c.g[S2][0];
        c.sw(c.g[S4][0] as u32, c.g[S1][0]);
        if !differs {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x0022_4df0, 0x0022_2708)?;
        }
        // Register 1 (the second entry): its table is assigned.
        c.cycles += 10;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0);
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[S1][0] as u32, 0);
        c.g[A1][0] = c.g[S5][0];
        call_ptr(c, 0x0022_5068, 0x0022_271c)?;
        c.g[S0][0] = c.lw(c.g[S4][0] as u32);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        let differs = c.g[V0][0] != c.g[S2][0];
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        if !differs {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x0022_4df0, 0x0022_273c)?;
        }
        // The value: a table is assigned, a function pair is shared, any
        // other type copies the raw words. Each arm ends at the Execute call;
        // only the `!=` arm (and the copy of the word) falls to the tail.
        c.cycles += 5;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
        let type_differs = c.g[V1][0] != c.g[S2][0];
        c.g[V0][0] = 0x5;
        if !type_differs {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0022_5068, 0x0022_275c)?;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            } else {
                c.cycles += 6;
                let s3 = c.g[S3][0] as u32;
                c.ldl(2, s3.wrapping_add(0x7));
                c.ldr(2, s3);
                let s0 = c.g[S0][0] as u32;
                c.sdl(2, s0.wrapping_add(0x13));
                c.sdr(2, s0.wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            }
        }
        // LunoVM_Execute(out, state, retVar).
        c.cycles += 16;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = c.g[S7][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_27d0, 0x0022_27a0)?;
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameModule_RequestOverlayStateGlobal()` (0x233ad0): request the overlay
/// state of the game module held in the global at `gp-0x488`.
pub fn game_module_request_overlay_state_global(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0023_1278, 0x0023_3ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameState_LoadState(self, obj)` (0x233b28): the load-state hook of a
/// game state. `obj` is the loader; its vtable (`*obj`) holds a half-word
/// offset at `+0x10` (added to `obj` to make the `this` of the call) and the
/// function at `+0x14`, called with `(this, self, 4)`.
pub fn game_state_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0x4;
        let obj = c.g[A3][0] as u32;
        c.g[V1][0] = c.lw(obj);
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_3b50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameStateQuit_LoadState(self, obj)` (0x233bb0): forwards to
/// `GameState_LoadState` with the same arguments.
pub fn game_state_quit_load_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0023_3b28,
            f: ee::s06::f_00233b28,
        }),
        0x0023_3bc0,
    )
}

/// `GameStateIdle_LoadState(self, obj)` (0x233bf0): forwards to
/// `GameState_LoadState` with the same arguments.
pub fn game_state_idle_load_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0023_3b28,
            f: ee::s06::f_00233b28,
        }),
        0x0023_3c00,
    )
}

/// `void cFELoadHintState_Init(cFELoadHintState* self)` (0x245950): add the
/// hint screen (`self+0x40`) to the engine (`self+0x10`), play it, and set
/// the title and body texts from the rotating hint counter (`gp-0x4b8`,
/// wrapping at 12). When the screen has a hint object (`self+0x48` with
/// its word at `+0x10`), copy that word into the object's `+0x78`.
pub fn c_fe_load_hint_state_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, 17, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x90);
        frame.save_ra(c, 0x80);
        frame.save(c, 16, 0xb0);
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_sub(0x4228));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0024_5978)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0024_598c)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0024_59a4)?;
        }
        // The hint counter (a signed byte at gp-0x4b8) and its wrap at 12.
        c.cycles += 5;
        let counter = (c.g[GP][0] as u32).wrapping_sub(0x4b8);
        c.g[V0][0] = c.lb(counter);
        c.g[V1][0] = 0xc;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        let t = c.g[S0][0] != c.g[V1][0];
        c.g[A0][0] = c.lbu(counter);
        if !t {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
            c.sb(counter, c.g[V1][0]);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        }
        // The two strings: sprintf(title, "%d", n) and sprintf(body, ..., n).
        c.cycles += 15;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4218));
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x0041_7828, 0x0024_59e0)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4200));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0041_7828, 0x0024_59f8)?;
        c.g[V1][0] = c.lb(counter);
        c.g[A1][0] = 0xf;
        // The counter advances modulo 15.
        c.cycles += 13;
        c.g[A0][0] = 0x0048_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_sub(0x41f0));
        c.div(0, V1, A1);
        c.g[V0][0] = c.hi[0];
        c.sb(counter, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0024_5a24)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_5a30)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.g[A0][0] = 0x0048_0000;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0024_5a44)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0024_5a50)?;
            c.g[A0][0] = 0x0048_0000;
        }
        // The body text.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41e0));
        call_ptr(c, 0x0031_7670, 0x0024_5a5c)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0024_5a68)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.g[A0][0] = 0x0048_0000;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7670, 0x0024_5a7c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0024_5a88)?;
            c.g[A0][0] = 0x0048_0000;
        }
        // The hint object of the screen (`self+0x48`): the three exits of the
        // generated labelled blocks, each falling through the same restores.
        'b_245ad4: {
            'b_245ad0: {
                'b_245acc: {
                    c.cycles += 7;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41d0));
                    call_ptr(c, 0x0031_7670, 0x0024_5a94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0024_5aa0)?;
                    let no_text = c.g[V0][0] == 0;
                    frame.restore(c, 16, 0xb0);
                    if no_text {
                        break 'b_245acc;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        let no_hint = c.g[A0][0] == 0;
                        frame.restore(c, 17, 0xa0);
                        if no_hint {
                            break 'b_245ad0;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                            let no_word = c.g[A0][0] == 0;
                            frame.restore(c, 18, 0x90);
                            if no_word {
                                break 'b_245ad4;
                            } else {
                                c.cycles += 3;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
                                frame.restore(c, 16, 0xb0);
                                break 'b_245acc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, 17, 0xa0);
                break 'b_245ad0;
            }
            c.cycles += 1;
            frame.restore(c, 18, 0x90);
            break 'b_245ad4;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProgressiveVerifyPS2_dtor(self)` (0x245f30): forwards to the
/// base destructor at 0x39e390.
pub fn c_fe_state_progressive_verify_ps2_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e390), 0x0024_5f40)
}

/// `cFEStateProgressiveVerifyPS2_Update(self)` (0x246098): the per-frame
/// update of the progressive-verify popup. Counts `self+0x4c` down; when it
/// reaches zero, once (the flag at `self+0x48`), sets the "display mode
/// changed" title and body text, the continue label, and disables the
/// "no" button and the graphics manager. Then `cUIState_onUpdate(self)`.
pub fn c_fe_state_progressive_verify_ps2_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
        let count_zero = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        if !count_zero {
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                let flag_set = c.g[V0][0] != 0;
                c.g[V0][0] = 0x1;
                if !flag_set {
                    progressive_verify_first_time(c)?;
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e510, 0x0024_61cc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The branch of the update that runs once, when the countdown ends and the
/// flag is clear: the title and body texts, the continue label, the "no"
/// button and the graphics manager's call.
fn progressive_verify_first_time(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 10;
    c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2ac0));
    let flag = (c.g[S1][0] as u32).wrapping_add(0x48);
    c.sb(flag, c.g[V0][0]);
    call_ptr(c, 0x0031_7670, 0x0024_60dc)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x0024_60e8)?;
    c.g[A0][0] = c.g[V0][0];
    if c.g[A0][0] == 0 {
        c.g[A0][0] = 0x004a_0000;
    } else {
        // The title text: its mode word decides which string the call uses.
        c.cycles += 3;
        let mode = (c.g[S1][0] as u32).wrapping_add(0x48);
        c.g[V0][0] = c.lb(mode);
        let mode_set = c.g[V0][0] != 0;
        c.g[A1][0] = 0x0048_0000;
        if !mode_set {
            c.cycles += 5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4708));
            call_ptr(c, 0x003a_0c50, 0x0024_610c)?;
            c.g[A0][0] = 0x004a_0000;
        } else {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x46d0));
            call_ptr(c, 0x003a_0c50, 0x0024_611c)?;
            c.g[A0][0] = 0x004a_0000;
        }
    }
    // The continue label.
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ac8));
    call_ptr(c, 0x0031_7670, 0x0024_6128)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x0024_6134)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[A0][0] = 0x004a_0000;
    } else {
        c.cycles += 7;
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4698));
        call_ptr(c, 0x0031_7670, 0x0024_614c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x0024_6158)?;
        c.g[A0][0] = 0x004a_0000;
    }
    // The "no" button: hide it through its vtable (slots 9 and 8).
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ad0));
    call_ptr(c, 0x0031_7670, 0x0024_6164)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x0024_6170)?;
    c.g[S0][0] = c.g[V0][0];
    let no_button = c.g[S0][0] == 0;
    c.g[A1][0] = 0;
    if !no_button {
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let f = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, f, 0x0024_6190)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let f = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, f, 0x0024_61a8)?;
    }
    // The graphics manager's vtable slot 8, cleared.
    c.cycles += 7;
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
    let f = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, 0x0024_61c4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate of this module's call fakes, in each test's own RAM: the
    /// magic at `OC1016_CTRL`, the zero mask at `+8` (bit n: call n returns 0)
    /// and the result every fake returns when non-zero, at `+16`.
    pub(crate) const OC1016_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1016_MAGIC: u32 = 0x1016_c0de;

    /// A fake callee: logs the call (`fakes::fake`), then applies the forced
    /// result and the zero mask of this test.
    fn oc1016_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        let forced = c.bus.read32(OC1016_CTRL + 16);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if forced != 0 {
            c.g[V0][0] = forced as u64;
        }
        let mask = c.bus.read64(OC1016_CTRL + 8);
        if n < 64 && (mask >> n) & 1 == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc1016_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1016_CTRL) != OC1016_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1016_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1016_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1016_fakes!(
        (
            oc1016_fake_00317e30,
            OC1016_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc1016_fake_00224df0,
            OC1016_PREV_00224DF0,
            ee::s05::OVERRIDE_00224DF0,
            ee::s05::f_00224df0__generated,
            0x0022_4df0
        ),
        (
            oc1016_fake_00225068,
            OC1016_PREV_00225068,
            ee::s05::OVERRIDE_00225068,
            ee::s05::f_00225068__generated,
            0x0022_5068
        ),
        (
            oc1016_fake_002227d0,
            OC1016_PREV_002227D0,
            ee::s05::OVERRIDE_002227D0,
            ee::s05::f_002227d0__generated,
            0x0022_27d0
        ),
        (
            oc1016_fake_00231278,
            OC1016_PREV_00231278,
            ee::s06::OVERRIDE_00231278,
            ee::s06::f_00231278__generated,
            0x0023_1278
        ),
        (
            oc1016_fake_00317670,
            OC1016_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc1016_fake_00397cc0,
            OC1016_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc1016_fake_0039c870,
            OC1016_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc1016_fake_0039d860,
            OC1016_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc1016_fake_0039e390,
            OC1016_PREV_0039E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039_e390
        ),
        (
            oc1016_fake_0039e510,
            OC1016_PREV_0039E510,
            ee::s13::OVERRIDE_0039E510,
            ee::s13::f_0039e510__generated,
            0x0039_e510
        ),
        (
            oc1016_fake_003a0c50,
            OC1016_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc1016_fake_003a1000,
            OC1016_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc1016_fake_00417828,
            OC1016_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// A test machine with the callee fakes armed: `forced` (0 for none) is
    /// the result of every fake, `mask` the calls that return 0.
    fn oc1016_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        forced: u32,
        mask: u64,
    ) -> Rng {
        oc1016_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1016_CTRL, OC1016_MAGIC);
        c.bus.write64(OC1016_CTRL + 8, mask);
        c.bus.write32(OC1016_CTRL + 16, forced);
        r
    }

    /// The setup of the three load-state functions: `self` in a0, the loader
    /// object pointer in a1 whose vtable has a half-word offset and the fake
    /// call target (outside the table: the machine's miss hook).
    fn load_state_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 8);
        let this = DATA + 0x1000;
        let slot = DATA + 0x2000;
        let obj = DATA + 0x3000;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(slot);
        c.bus.write32(slot, obj);
        c.bus.write16(obj + 0x10, (r.next_u32() & 0xffff) as u16);
        c.bus.write32(obj + 0x14, fakes::FAKE);
    }

    #[test]
    fn luno_vm_call_with_arg_matches() {
        check(
            ee::s05::f_00222648__generated,
            luno_vm_call_with_arg,
            300,
            &|c, seed| {
                let out = DATA + 0x5000;
                let state = DATA + 0x2000;
                let chunk = DATA + 0x3000;
                let table = DATA + 0x6000;
                let value = DATA + 0x7000;
                let ret_var = DATA + 0x8000;
                // The allocator returns the register block at DATA+0x4000
                // (every fake returns it; no zero mask).
                let block = DATA + 0x4000;
                oc1016_machine(c, seed, 0x222648, 24, block, 0);
                c.bus.write32(state, chunk);
                c.bus.write32(chunk + 4, 0x10);
                c.bus.write32(chunk + 0x18, seed % 4);
                // Register 0 (the block's first entry): its type word for
                // the destructor branch; register 1 (the second) for the
                // table assignment.
                c.bus
                    .write32(block + 0x10 + 8, if seed % 2 == 0 { 4 } else { seed % 7 });
                c.bus
                    .write32(block + 0x10 + 0x14, if seed % 3 == 0 { 0 } else { 0x10 });
                c.bus.write32(block + 0x10 + 0x14 + 0x8, 4);
                // The value's type decides the arm: a table (4), a function
                // (5) or a raw word.
                let ty = [4u32, 5, 0, 1, 2, 3][(seed % 6) as usize];
                c.bus.write32(value + 8, ty);
                c.bus.write32(value, 0x1234_5678 ^ seed);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(state);
                c.g[A2][0] = sx(table);
                c.g[A3][0] = sx(value);
                c.g[T0][0] = sx(ret_var);
            },
        );
    }

    #[test]
    fn game_module_request_overlay_state_global_matches() {
        check(
            ee::s06::f_00233ad0__generated,
            game_module_request_overlay_state_global,
            200,
            &|c, seed| {
                oc1016_machine(c, seed, 0x233ad0, 4, 0, (seed % 2) as u64);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x488),
                    DATA + 0x3000 + 0x100 * (seed % 3),
                );
            },
        );
    }

    #[test]
    fn game_state_load_state_matches() {
        check(
            ee::s06::f_00233b28__generated,
            game_state_load_state,
            300,
            &|c, seed| load_state_setup(c, seed, 0x233b28),
        );
    }

    #[test]
    fn game_state_quit_load_state_matches() {
        check(
            ee::s06::f_00233bb0__generated,
            game_state_quit_load_state,
            200,
            &|c, seed| load_state_setup(c, seed, 0x233bb0),
        );
    }

    #[test]
    fn game_state_idle_load_state_matches() {
        check(
            ee::s06::f_00233bf0__generated,
            game_state_idle_load_state,
            200,
            &|c, seed| load_state_setup(c, seed, 0x233bf0),
        );
    }

    #[test]
    fn c_fe_load_hint_state_init_matches() {
        check(
            ee::s06::f_00245950__generated,
            c_fe_load_hint_state_init,
            300,
            &|c, seed| {
                let obj = DATA + 0x7000;
                let mask = fakes::mix(seed as u64) & fakes::mix(seed as u64 ^ 0x55);
                oc1016_machine(c, seed, 0x245950, 40, obj, mask);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, DATA + 0x2000);
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
                c.bus.write32(
                    DATA + 0x3000 + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x6000 },
                );
                c.bus.write8(GAME_GP.wrapping_sub(0x4b8), (seed % 14) as u8);
            },
        );
    }

    #[test]
    fn c_fe_state_progressive_verify_ps2_dtor_matches() {
        check(
            ee::s06::f_00245f30__generated,
            c_fe_state_progressive_verify_ps2_dtor,
            200,
            &|c, seed| {
                oc1016_machine(c, seed, 0x245f30, 4, 0, (seed % 2) as u64);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_fe_state_progressive_verify_ps2_update_matches() {
        check(
            ee::s06::f_00246098__generated,
            c_fe_state_progressive_verify_ps2_update,
            300,
            &|c, seed| {
                let obj = DATA + 0x7000;
                let mask = fakes::mix(seed as u64) & fakes::mix(seed as u64 ^ 0x55);
                oc1016_machine(c, seed, 0x246098, 16, obj, mask);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x4c, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus.write8(this + 0x48, ((seed / 3) % 2) as u8);
                // Every fake returns `obj`, whose vtable calls the fake target.
                let vt = DATA + 0x7800;
                c.bus.write32(obj + 8, vt);
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, fakes::FAKE);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, fakes::FAKE);
                // The graphics manager (gp+0x2a90) and its slot 8.
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), DATA + 0x9000);
                c.bus.write32(DATA + 0x9000 + 0x10d8, DATA + 0x9800);
                c.bus.write16(DATA + 0x9800 + 0x40, 0);
                c.bus.write32(DATA + 0x9800 + 0x44, fakes::FAKE);
            },
        );
    }
}
