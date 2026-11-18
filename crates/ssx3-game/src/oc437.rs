//! Staging module oc437 (wave wave222): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc437`.

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
    (
        0x0024_2d10,
        generic_stream_pause_create,
        "GenericStreamPause_Create",
    ),
    (
        0x0028_2db0,
        ctransition_man_stop_transition,
        "cTransitionMan_StopTransition",
    ),
    (
        0x0028_2c18,
        script_engine_get_seq_instance,
        "ScriptEngine_GetSeqInstance",
    ),
    (
        0x0027_2510,
        inseq_heap_delete_array,
        "InSeq_HeapDeleteArray",
    ),
    (
        0x0015_cd60,
        cam_controller_clear_algorithms,
        "CamController_ClearAlgorithms",
    ),
    (
        0x0027_7800,
        script_list_try_advance,
        "ScriptList_TryAdvance",
    ),
    (
        0x003a_8f80,
        cworld_cache_streamer_ctor,
        "cWorldCache_StreamerCtor",
    ),
    (
        0x0027_4c10,
        inseq_instance_set_user_data,
        "InSeq_InstanceSetUserData",
    ),
    (0x001d_cc78, feui_custom_dtor, "FEUICustom_dtor"),
    (
        0x0017_6398,
        target_script_camera_algo_dtor,
        "TargetScriptCameraAlgo_dtor",
    ),
    (
        0x0017_6368,
        manual_script_camera_algo_dtor,
        "ManualScriptCameraAlgo_dtor",
    ),
    (0x0036_1f60, intc_table_clear_slots, "IntcTable_ClearSlots"),
    (0x0036_6fe0, tex_man_set_alloc_flags, "TexMan_SetAllocFlags"),
    (
        0x0026_f498,
        replay_empty_reset_hook,
        "Replay_EmptyResetHook",
    ),
    (
        0x001d_e728,
        fe_state_main_menu_dtor,
        "cFEStateMainMenu_dtor",
    ),
    (0x001d_e6c8, fe_state_title_dtor, "cFEStateTitle_dtor"),
    (
        0x001d_e668,
        fe_state_transition_dtor,
        "cFEStateTransition_dtor",
    ),
    (
        0x001d_e5c8,
        fe_state_background_dtor,
        "cFEStateBackground_dtor",
    ),
    (
        0x001d_e528,
        fe_state_event_select_dtor,
        "cFEStateEventSelect_dtor",
    ),
    (
        0x001d_d8a0,
        fe_state_char_setup_dtor,
        "cFEStateCharSetup_dtor",
    ),
];

/// `operator_new`-style allocator (`EA_MemAllocNamed`, 0x317d70).
const OC437_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
/// The delete helper called by scalar-deleting destructors (0x317e50).
const OC437_OPERATOR_DELETE: u32 = 0x0031_7e50;
/// The destructor-chain helper the front-end state dtors call (0x39e390).
const OC437_BASE_DTOR: u32 = 0x0039_e390;

/// `cStreamPause* GenericStreamPause_Create(void)` (0x242d10): allocates a
/// 4-byte object (`EA_MemAllocNamed(4, name, 0x20000000, 0)`) and stores its
/// vtable pointer in it. Returns the allocation in `v0`.
pub fn generic_stream_pause_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 4;
        c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x3ae0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OC437_MEM_ALLOC_NAMED, 0x0024_2d30)?;
        c.g[V1][0] = 0x48_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x2f18));
        frame.restore_ra(c, 0);
        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cTransitionMan_StopTransition(self)` (0x282db0): calls
/// `cTransitionMan_StartTransition(self, a1, a2, self->field_0x28)` with the
/// caller's `a1`/`a2` untouched.
pub fn ctransition_man_stop_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x28)).lw(c);
        Callee {
            addr: 0x0028_2dd0,
            f: ee::s07::cTransitionMan_StartTransition,
        }
        .call(c, 0x0028_2dc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_GetSeqInstance(engine)` (0x282c18):
/// `InSeq_GetInstance(engine->seq(+0x2b8))`.
pub fn script_engine_get_seq_instance(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(0x2b8),
        Target::Direct(Callee {
            addr: 0x0027_2cc0,
            f: ee::s07::InSeq_GetInstance,
        }),
        0x0028_2c28,
    )
}

/// `InSeq_HeapDeleteArray(p)` (0x272510): `InSeq_HeapDelete(p)`.
pub fn inseq_heap_delete_array(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0027_24e8,
            f: ee::s07::InSeq_HeapDelete,
        }),
        0x0027_2520,
    )
}

/// `CamController_ClearAlgorithms(ctl)` (0x15cd60):
/// `AlgorithmList_Clear(ctl->algorithms(+8))`.
pub fn cam_controller_clear_algorithms(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(8),
        Target::Direct(Callee {
            addr: 0x0015_cb08,
            f: ee::s01::AlgorithmList_Clear,
        }),
        0x0015_cd70,
    )
}

/// `ScriptList.pending` (non-zero: an entry is already being processed).
const OC437_SCRIPT_LIST_PENDING: u32 = 0xa8;

/// `ScriptList_TryAdvance(list)` (0x277800): `ScriptList_CheckNextReady(list)`;
/// if `list->pending` (+0xa8) is zero, `ScriptList_NextEntry(list)`.
pub fn script_list_try_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0027_76e0,
            f: ee::s07::ScriptList_CheckNextReady,
        }
        .call(c, 0x0027_7814)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(OC437_SCRIPT_LIST_PENDING)).lw(c);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0027_7838,
                f: ee::s07::ScriptList_NextEntry,
            }
            .call(c, 0x0027_7828)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWorldCache* cWorldCache_StreamerCtor(self)` (0x3a8f80): resets the
/// streamer (`cWorldCache_StreamerReset(self)`) and returns `self`.
pub fn cworld_cache_streamer_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x003a_9188,
            f: ee::s13::cWorldCache_StreamerReset,
        }
        .call(c, 0x003a_8fa0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* InSeq_InstanceSetUserData(inst, data)` (0x274c10): swaps
/// `inst->userData` (+0xc) for `data`, returning the old value.
pub fn inseq_instance_set_user_data(c: &mut Cpu) {
    c.cycles += 3;
    let inst = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(inst.wrapping_add(0xc)).lw(c);
    c.sw(inst.wrapping_add(0xc), c.g[A1][0]);
    ret(c);
}

/// A scalar-deleting destructor: sets the vtable pointer
/// (`vtable_hi - vtable_lo`) at `this + vtable_field`, and when `flags & 1`
/// calls the delete helper (0x317e50) on `this`.
fn oc437_dtor_flag_delete(
    c: &mut Cpu,
    vtable_hi: u32,
    vtable_lo: u32,
    vtable_field: u32,
    ret_to: u32,
) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = vtable_hi as u64;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(vtable_hi.wrapping_sub(vtable_lo));
        c.g[A1][0] &= 1;
        let skip = c.g[A1][0] == 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(vtable_field), c.g[V0][0]);
        if !skip {
            c.cycles += 2;
            call_ptr(c, OC437_OPERATOR_DELETE, ret_to)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEUICustom_dtor(this, flags)` (0x1dcc78): vtable at +4, delete if `flags & 1`.
pub fn feui_custom_dtor(c: &mut Cpu) {
    oc437_dtor_flag_delete(c, 0x47_0000, 0x22d8, 0x4, 0x001d_cc9c)
}

/// `TargetScriptCameraAlgo_dtor(this, flags)` (0x176398): vtable at +0x10, delete if `flags & 1`.
pub fn target_script_camera_algo_dtor(c: &mut Cpu) {
    oc437_dtor_flag_delete(c, 0x46_0000, 0x4220, 0x10, 0x0017_63bc)
}

/// `ManualScriptCameraAlgo_dtor(this, flags)` (0x176368): vtable at +0x10, delete if `flags & 1`.
pub fn manual_script_camera_algo_dtor(c: &mut Cpu) {
    oc437_dtor_flag_delete(c, 0x46_0000, 0x4220, 0x10, 0x0017_638c)
}

/// `IntcTable_ClearSlots(table)` (0x361f60): zeroes three 8-byte slots at
/// `table + 0x80 .. 0x98` (two words each).
pub fn intc_table_clear_slots(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = 2;
    loop {
        c.cycles += 7;
        c.sw((c.g[A0][0] as u32).wrapping_sub(4), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.sw(c.g[A0][0] as u32, 0);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0036_1f68);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

/// `TexMan_SetAllocFlags(texman, flags)` (0x366fe0): `texman->allocFlags(+0x1f58) = flags`.
pub fn tex_man_set_alloc_flags(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1f58), c.g[A1][0]);
    ret(c);
}

/// `Replay_EmptyResetHook(void)` (0x26f498): empty.
pub fn replay_empty_reset_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// A front-end state destructor: `this->vtable(+8) = 0x470000 - vtable_lo`,
/// then the base destructor helper (0x39e390) with the caller's `a0`/`a1`.
fn oc437_fe_state_dtor(c: &mut Cpu, vtable_lo: u32, ret_to: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x47_0000u32.wrapping_sub(vtable_lo));
        c.sw((c.g[V1][0] as u32).wrapping_add(8), c.g[V0][0]);
        call_ptr(c, OC437_BASE_DTOR, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMainMenu_dtor(this, flags)` (0x1de728).
pub fn fe_state_main_menu_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_e744)
}
/// `cFEStateTitle_dtor(this, flags)` (0x1de6c8).
pub fn fe_state_title_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_e6e4)
}
/// `cFEStateTransition_dtor(this, flags)` (0x1de668).
pub fn fe_state_transition_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x4478, 0x001d_e684)
}
/// `cFEStateBackground_dtor(this, flags)` (0x1de5c8).
pub fn fe_state_background_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x43a8, 0x001d_e5e4)
}
/// `cFEStateEventSelect_dtor(this, flags)` (0x1de528).
pub fn fe_state_event_select_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_e544)
}
/// `cFEStateCharSetup_dtor(this, flags)` (0x1dd8a0).
pub fn fe_state_char_setup_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_d8bc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, covered, machine, ptr};
    use crate::difftest::{check, DATA};

    /// Gate for this module's callee fakes (outside the pool and the log).
    const OC437_GATE: u32 = DATA + 0x20_0000;
    const OC437_MAGIC: u32 = 0x0c43_7001;

    fn oc437_noop(_c: &mut Cpu) {}

    /// A gated fake for the callee behind override slot `$slot`: with the gate
    /// set it logs the call (`fakes::fake_call`, which may unwind); otherwise it
    /// runs what the slot held.
    macro_rules! oc437_fake {
        ($name:ident) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC437_GATE) != OC437_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), oc437_noop);
                }
                let target = c.pc;
                fakes::fake_call(c, target)
            }
        };
    }
    oc437_fake!(oc437_fake_a);
    oc437_fake!(oc437_fake_b);
    oc437_fake!(oc437_fake_c);
    oc437_fake!(oc437_fake_d);
    oc437_fake!(oc437_fake_e);
    oc437_fake!(oc437_fake_f);
    oc437_fake!(oc437_fake_g);
    oc437_fake!(oc437_fake_h);
    oc437_fake!(oc437_fake_i);
    oc437_fake!(oc437_fake_j);

    fn oc437_install_fakes() {
        use std::ptr::addr_of_mut;
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 10] = [
            (addr_of_mut!(ee::s10::OVERRIDE_00317D70), oc437_fake_a),
            (addr_of_mut!(ee::s07::OVERRIDE_00282DD0), oc437_fake_b),
            (addr_of_mut!(ee::s07::OVERRIDE_00272CC0), oc437_fake_c),
            (addr_of_mut!(ee::s07::OVERRIDE_002724E8), oc437_fake_d),
            (addr_of_mut!(ee::s01::OVERRIDE_0015CB08), oc437_fake_e),
            (addr_of_mut!(ee::s07::OVERRIDE_002776E0), oc437_fake_f),
            (addr_of_mut!(ee::s07::OVERRIDE_00277838), oc437_fake_g),
            (addr_of_mut!(ee::s13::OVERRIDE_003A9188), oc437_fake_h),
            (addr_of_mut!(ee::s13::OVERRIDE_0039E390), oc437_fake_i),
            (addr_of_mut!(ee::s10::OVERRIDE_00317E50), oc437_fake_j),
        ];
        for (slot, fake) in slots {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// Fixture start: fakes hooked and gated on, `c.pc` at the entry.
    fn oc437_setup(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> crate::difftest::Rng {
        oc437_install_fakes();
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC437_GATE, OC437_MAGIC);
        r
    }

    /// Runs `check`/`covered` for a call-making function.
    fn oc437_covered(
        generated: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        entry: u32,
        calls: u32,
        init: &dyn Fn(&mut Cpu, &mut crate::difftest::Rng, u32),
    ) {
        covered(generated, hand, 300, &|c, seed| {
            let mut r = oc437_setup(c, seed, entry, calls);
            let obj = ptr(&mut r);
            c.g[A0][0] = obj as u64;
            crate::difftest::fill(c, &mut r, obj, 0x300 / 4);
            init(c, &mut r, seed);
        });
    }

    /// Like [`oc437_covered`] but only `check`: for callees whose gated fake is
    /// not reached through the override slot; the real callee runs on the
    /// fixture, which keeps it short (zeroed list heads, negative counts).
    fn oc437_checked(
        generated: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        entry: u32,
        init: &dyn Fn(&mut Cpu, &mut crate::difftest::Rng, u32),
    ) {
        check(generated, hand, 300, &|c, seed| {
            let mut r = oc437_setup(c, seed, entry, 0);
            let obj = ptr(&mut r);
            c.g[A0][0] = obj as u64;
            crate::difftest::fill(c, &mut r, obj, 0x300 / 4);
            init(c, &mut r, seed);
        });
    }

    #[test]
    fn generic_stream_pause_create_matches() {
        oc437_covered(
            ee::s06::f_00242d10__generated,
            generic_stream_pause_create,
            0x0024_2d10,
            1,
            &|_, _, _| {},
        );
    }

    #[test]
    fn ctransition_man_stop_transition_matches() {
        oc437_covered(
            ee::s07::f_00282db0__generated,
            ctransition_man_stop_transition,
            0x0028_2db0,
            1,
            &|_, _, _| {},
        );
    }

    #[test]
    fn script_engine_get_seq_instance_matches() {
        oc437_checked(
            ee::s07::f_00282c18__generated,
            script_engine_get_seq_instance,
            0x0028_2c18,
            &|_, _, _| {},
        );
    }

    #[test]
    fn inseq_heap_delete_array_matches() {
        oc437_checked(
            ee::s07::f_00272510__generated,
            inseq_heap_delete_array,
            0x0027_2510,
            &|_, _, _| {},
        );
    }

    #[test]
    fn cam_controller_clear_algorithms_matches() {
        oc437_checked(
            ee::s01::f_0015cd60__generated,
            cam_controller_clear_algorithms,
            0x0015_cd60,
            &|_, _, _| {},
        );
    }

    /// `pending` non-zero (no `NextEntry`). The real `CheckNextReady` takes its
    /// early exit with the current index (+0xbc) negative. The `NextEntry`
    /// branch is not tested: its real callee chain (`PopEntries`, `Stop`, ...)
    /// runs away on fixture data and the gated fake is not reached.
    #[test]
    fn script_list_try_advance_matches() {
        oc437_checked(
            ee::s07::f_00277800__generated,
            script_list_try_advance,
            0x0027_7800,
            &|c, r, _seed| {
                let obj = c.g[A0][0] as u32;
                c.bus
                    .write32(obj + OC437_SCRIPT_LIST_PENDING, r.next_u32() | 1);
                c.bus
                    .write32(obj + 0xbc, 0xffff_fff0 | (r.next_u32() & 0xf));
            },
        );
    }

    #[test]
    fn cworld_cache_streamer_ctor_matches() {
        oc437_checked(
            ee::s13::f_003a8f80__generated,
            cworld_cache_streamer_ctor,
            0x003a_8f80,
            &|_, _, _| {},
        );
    }

    #[test]
    fn inseq_instance_set_user_data_matches() {
        check(
            ee::s07::f_00274c10__generated,
            inseq_instance_set_user_data,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c10, 0);
                c.pc = 0x0027_4c10;
                let obj = ptr(&mut r);
                c.g[A0][0] = obj as u64;
                c.bus.write32(obj + 0xc, r.next_u32());
            },
        );
    }

    /// `flags & 1` clear (no delete) and set (delete call, may unwind).
    fn oc437_dtor_flags(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        let hi = (r.next_u32() as u64) << 32;
        let low = (seed % 2) as u64;
        c.g[A1][0] = hi | low | (r.next_u32() & 0xe) as u64;
    }

    #[test]
    fn feui_custom_dtor_matches() {
        oc437_covered(
            ee::s04::f_001dcc78__generated,
            feui_custom_dtor,
            0x001d_cc78,
            1,
            &oc437_dtor_flags,
        );
    }

    #[test]
    fn target_script_camera_algo_dtor_matches() {
        oc437_covered(
            ee::s02::f_00176398__generated,
            target_script_camera_algo_dtor,
            0x0017_6398,
            1,
            &oc437_dtor_flags,
        );
    }

    #[test]
    fn manual_script_camera_algo_dtor_matches() {
        oc437_covered(
            ee::s02::f_00176368__generated,
            manual_script_camera_algo_dtor,
            0x0017_6368,
            1,
            &oc437_dtor_flags,
        );
    }

    /// The three-iteration clearing loop, with and without events at the back edges.
    #[test]
    fn intc_table_clear_slots_matches() {
        check_events(
            ee::s12::f_00361f60__generated,
            intc_table_clear_slots,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361f60, 0);
                let table = ptr(&mut r);
                c.g[A0][0] = table as u64;
                crate::difftest::fill(c, &mut r, table, 0x40);
                let mode = schedule(c, &mut r, seed, 0x0036_1f60);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn tex_man_set_alloc_flags_matches() {
        check(
            ee::s12::f_00366fe0__generated,
            tex_man_set_alloc_flags,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x366fe0, 0);
                c.pc = 0x0036_6fe0;
                c.g[A0][0] = ptr(&mut r) as u64;
            },
        );
    }

    #[test]
    fn replay_empty_reset_hook_matches() {
        check(
            ee::s07::f_0026f498__generated,
            replay_empty_reset_hook,
            20,
            &|c, seed| {
                machine(c, seed, 0x26f498, 0);
                c.pc = 0x0026_f498;
            },
        );
    }

    /// The front-end state destructors differ in vtable constant and return address.
    #[test]
    fn fe_state_dtors_match() {
        let all: [(fn(&mut Cpu), fn(&mut Cpu), u32); 6] = [
            (ee::s04::f_001de728__generated, fe_state_main_menu_dtor, 0x001d_e728),
            (ee::s04::f_001de6c8__generated, fe_state_title_dtor, 0x001d_e6c8),
            (ee::s04::f_001de668__generated, fe_state_transition_dtor, 0x001d_e668),
            (ee::s04::f_001de5c8__generated, fe_state_background_dtor, 0x001d_e5c8),
            (ee::s04::f_001de528__generated, fe_state_event_select_dtor, 0x001d_e528),
            (ee::s04::f_001dd8a0__generated, fe_state_char_setup_dtor, 0x001d_d8a0),
        ];
        for (generated, hand, entry) in all {
            oc437_covered(generated, hand, entry, 1, &|_, _, _| {});
        }
    }
}
