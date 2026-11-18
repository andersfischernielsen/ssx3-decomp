//! Staging module oc436 (wave wave222): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc436`.

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
///
/// `AdfDataSource_ctor` (0x311250) is written but not listed: its callee
/// `EAFile_LoadDecompressed` reads files and cannot be bounded in a fixture.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0027_4388,
        oc436_in_seq_channel_get_header_field4,
        "InSeq_ChannelGetHeaderField4",
    ),
    (
        0x0015_cc70,
        oc436_cam_controller_reset_for_rider,
        "CamController_ResetForRider",
    ),
    (
        0x0016_ce30,
        oc436_camera_trigger_man_reload,
        "CameraTriggerMan_Reload",
    ),
    (
        0x0030_8b58,
        oc436_wscript_man_create_local_table_thunk,
        "WScriptMan_CreateLocalTableThunk",
    ),
    (
        0x0011_9918,
        oc436_rider_metrix_set_grind_orientation,
        "RiderMetrix_SetGrindOrientation",
    ),
    (
        0x0027_9888,
        oc436_script_engine_unload_script_bank,
        "ScriptEngine_UnloadScriptBank",
    ),
    (0x0015_fff0, oc436_camera_wobble_init, "CameraWobble_Init"),
    (0x003a_7dd0, oc436_cworld_cache_ctor, "cWorldCache_ctor"),
    (0x0011_c230, oc436_rider_load, "Rider_Load"),
    (
        0x0022_d668,
        oc436_section_man_advance_event_switch_sync,
        "SectionMan_AdvanceEventSwitchSync",
    ),
    (
        0x0031_1220,
        oc436_cadf_data_source_ctor,
        "cAdfDataSource_ctor",
    ),
    (
        0x0028_3180,
        oc436_script_cam_binding_unbind,
        "ScriptCamBinding_Unbind",
    ),
    (
        0x0027_24c8,
        oc436_in_seq_heap_new_array,
        "InSeq_HeapNewArray",
    ),
    (
        0x0012_89f0,
        oc436_ai_reset_riders_default,
        "AI_ResetRidersDefault",
    ),
    (0x0034_15a8, oc436_boost_v01, "Boost_v01"),
    (
        0x0028_2bb0,
        oc436_script_engine_get_seq_process_script,
        "ScriptEngine_GetSeqProcessScript",
    ),
    (
        0x0016_d200,
        oc436_camera_trigger_stack_clear,
        "CameraTriggerStack_Clear",
    ),
    (
        0x0034_fe00,
        oc436_mod_obj_run_script_event_19b8,
        "ModObj_RunScriptEvent19b8",
    ),
    (
        0x0016_bf20,
        oc436_camera_trigger_man_clear_data,
        "CameraTriggerMan_ClearData",
    ),
];

/// `void* InSeq_ChannelGetHeaderField4(InSeqChannel* ch)` (0x274388): the
/// word at +4 of the header the channel points to (`ch->header` at +0x10).
pub fn oc436_in_seq_channel_get_header_field4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(OC436_CHANNEL_HEADER));
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
    c.pc = t;
}

/// `InSeqChannel.header`.
pub(crate) const OC436_CHANNEL_HEADER: u32 = 0x10;

/// `void* AdfDataSource_ctor(AdfDataSource* self, const char* path)`
/// (0x311250): load the (decompressed) file `path` with
/// `EAFile_LoadDecompressed(path, 0)`, store the buffer at `self+0`, and hand
/// it to `AdfDataSource_SetData(self, buffer)`; returns `self`.
///
/// No `OVERRIDES` entry and no test: the file loader cannot be bounded in a
/// fixture (it reaches `EAFile_CallWithPriority`). Transcribed from the
/// generated body.
pub fn oc436_adf_data_source_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_22f0, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x0031_1290, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AdfDataSource* cAdfDataSource_ctor(AdfDataSource* self, data)` (0x311220):
/// clear `self+0`, then `AdfDataSource_SetData(self, data)`; returns `self`.
pub fn oc436_cadf_data_source_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0);
        call_ptr(c, 0x0031_1290, 0x0031_1238)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `CamController.cameras`: the list head word at +8 (its first word is the
/// first list node).
pub(crate) const OC436_CAMCTRL_LIST: u32 = 0x8;
/// `CamController.camera` (the game camera at +0x10).
pub(crate) const OC436_CAMCTRL_CAMERA: u32 = 0x10;
/// List node: the object (+0) and the next node (+0x14).
pub(crate) const OC436_NODE_NEXT: u32 = 0x14;
/// Object vtable (+0x10) slot at +0x18 (`this` adjustment, halfword) and
/// +0x1c (method).
pub(crate) const OC436_OBJ_VTABLE: u32 = 0x10;
pub(crate) const OC436_VT_ADJUST: u32 = 0x18;
pub(crate) const OC436_VT_METHOD: u32 = 0x1c;

/// `void CamController_ResetForRider(CamController* self, int rider)`
/// (0x15cc70): call the virtual reset method (vtable +0x1c, with the
/// `this` adjustment at +0x18) of every object in the controller's list with
/// `rider`, then `GameCamera_ResetDefaultChase(self->camera)` and the world
/// helper at 0x2f41a8 with the same camera.
pub fn oc436_cam_controller_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CAMCTRL_LIST));
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        let empty = c.g[S0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 9;
                c.g[A1][0] = c.g[S2][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC436_OBJ_VTABLE));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(OC436_VT_ADJUST));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(OC436_VT_METHOD));
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0015_ccb8)?;
                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OC436_NODE_NEXT));
                if c.g[S0][0] != 0 {
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_cca0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CAMCTRL_CAMERA));
        call_ptr(c, 0x0015_e030, 0x0015_cccc)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CAMCTRL_CAMERA));
        call_ptr(c, 0x002f_41a8, 0x0015_ccd4)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan.triggers` (array of trigger pointers at +0) and
/// `.count` (+4); a trigger's mask at +0x20.
pub(crate) const OC436_CTM_TRIGGERS: u32 = 0x0;
pub(crate) const OC436_CTM_COUNT: u32 = 0x4;
pub(crate) const OC436_TRIGGER_MASK: u32 = 0x20;

/// `void CameraTriggerMan_Reload(CameraTriggerMan* self, data)` (0x16ce30):
/// `Unload(self, data)`, `Parse(self, data)`, clear the mask (+0x20) of every
/// trigger in the new array, then `UpdateTriggersMask2(self)`.
pub fn oc436_camera_trigger_man_reload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0016_c9b0, 0x0016_ce4c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0016_c228, 0x0016_ce58)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_COUNT));
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_TRIGGERS));
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw((c.g[V1][0] as u32).wrapping_add(OC436_TRIGGER_MASK), 0);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_COUNT));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(OC436_CTM_TRIGGERS));
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_ce68);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0016_d1b8, 0x0016_ce94)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_CreateLocalTableThunk(...)` (0x308b58): tail call of
/// `WScriptMan_CreateLocalTable` with the same arguments.
pub fn oc436_wscript_man_create_local_table_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0030_8b78), 0x0030_8b68)
}

/// `RiderMetrix.grindOrientation` (+0x20).
pub(crate) const OC436_RM_GRIND_ORIENTATION: u32 = 0x20;

/// `void RiderMetrix_SetGrindOrientation(RiderMetrix* m, int o)` (0x119918):
/// store `o` at +0x20, then `RiderMetrix_SetSpinAmount(m, ...)` (a1 is left
/// as it is).
pub fn oc436_rider_metrix_set_grind_orientation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.sw(
            (c.g[A0][0] as u32).wrapping_add(OC436_RM_GRIND_ORIENTATION),
            c.g[A1][0],
        );
        call_ptr(c, 0x0011_9898, 0x0011_9928)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_UnloadScriptBank(int id)` (0x279888): find the bank
/// slot of `id` (`FindBankSlotById(id, ?, 2)`); when it is found (>= 0),
/// `ScriptEngine_UnloadBank(id, slot)`.
pub fn oc436_script_engine_unload_script_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_b578, 0x0027_98a0)?;
        let not_found = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.g[S0][0];
        if !not_found {
            c.cycles += 2;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0027_b410, 0x0027_98b0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraWobble* CameraWobble_Init(CameraWobble* w)` (0x15fff0): zero the
/// six words at +0..+0x14, set +0x28 and +0x2c to the float at gp-0x6c2c and
/// +0x20 and +0x24 to the float at gp-0x6c28; returns `w`. Other entries go
/// to the generated `__entries` twin.
pub fn oc436_camera_wobble_init(c: &mut Cpu) {
    if c.pc != 0x0015_fff0 {
        return oc436_camera_wobble_init_resume(c);
    }
    c.cycles += 14;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6c2c));
    c.g[V0][0] = c.g[A0][0];
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6c28));
    let w = c.g[A0][0] as u32;
    c.swc1(0, w.wrapping_add(0x2c));
    c.swc1(1, w.wrapping_add(0x24));
    c.swc1(0, w.wrapping_add(0x28));
    for off in [0x0, 0x4, 0x8, 0xc, 0x10, 0x14] {
        c.sw(w.wrapping_add(off), 0);
    }
    let t = c.g[RA][0] as u32;
    c.swc1(1, w.wrapping_add(0x20));
    c.pc = t;
}

#[cfg(not(feature = "ee-unwind"))]
fn oc436_camera_wobble_init_resume(c: &mut Cpu) {
    ee::s01::f_0015fff0__entries(c)
}

// At install the orchestrator switches this to the generated twin; until
// then the unwind form (not built for staging) has no name for it.
#[cfg(feature = "ee-unwind")]
fn oc436_camera_wobble_init_resume(c: &mut Cpu) {
    ee::s01::f_0015fff0__generated(c)
}

/// `cWorldCache::cWorldCache(this)` (0x3a7dd0): construct the streamer view
/// at +0x10 (`cWorldCache_StreamerCtor`) and the stream manager at +0x300
/// (`cStreamMan_ctor`), clear +0, +4, +8, +0xc, +0x3e8, +0x3ec and store
/// `(int)(float(*(int*)(*(gp+0x2a74) + 0x10)) * *(float*)(gp-0x23c4))` at
/// +0x1bf0; returns `this`.
pub fn oc436_cworld_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x003a_8f80, 0x003a_7de8)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x300));
        call_ptr(c, 0x003a_6d50, 0x003a_7df0)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        for off in [0x0, 0x3ec, 0x3e8, 0xc, 0x4, 0x8] {
            c.sw(this.wrapping_add(off), 0);
        }
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x23c4));
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.cvt_s_w(0, 0);
        frame.restore_ra(c, 0);
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.swc1(1, this.wrapping_add(0x1bf0));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Rider.loadState` (+0x880): 7 when the rider is fully loaded.
pub(crate) const OC436_RIDER_LOAD_STATE: u32 = 0x880;
pub(crate) const OC436_RIDER_LOADED: u64 = 7;

/// `void Rider_Load(Rider* r, int immediate)` (0x11c230): with `immediate`
/// one `Rider_LoadStep`; otherwise step until the load state reaches 7,
/// yielding one tick (`THREAD_yieldticks(1)`) between steps.
pub fn oc436_rider_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let immediate = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if immediate {
            c.cycles += 4;
            call_ptr(c, 0x0011_c298, 0x0011_c250)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OC436_RIDER_LOAD_STATE));
            c.g[V0][0] = OC436_RIDER_LOADED;
            c.g[S1][0] = OC436_RIDER_LOADED;
            if c.g[V1][0] != c.g[V0][0] {
                loop {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0011_c298, 0x0011_c270)?;
                    c.g[A0][0] = 1;
                    call_ptr(c, 0x003e_52b0, 0x0011_c278)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OC436_RIDER_LOAD_STATE));
                    if c.g[V0][0] != c.g[S1][0] {
                        if event_due(c) {
                            deliver_event_here(c, 0x0011_c268);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SectionMan.eventSwitchState` (+0x1b4) and `.eventSwitchArg` (+0x1b8).
pub(crate) const OC436_SM_SWITCH_STATE: u32 = 0x1b4;
pub(crate) const OC436_SM_SWITCH_ARG: u32 = 0x1b8;
/// The original's jump table (states 0..4).
pub(crate) const OC436_SM_SWITCH_TABLE: u32 = 0x0047_b430;

/// `void SectionMan_AdvanceEventSwitchSync(SectionMan* s, int arg)`
/// (0x22d668): when `arg != 0` and the switch state (+0x1b4) is below 5, a
/// jump table steps the state machine: state 0 stores `arg` (+0x1b8) and goes
/// to 3, state 1 goes to 3 (and returns at once), state 2 goes to 4, states
/// 3 and 4 stay. Other entries go to the generated `__entries` twin.
pub fn oc436_section_man_advance_event_switch_sync(c: &mut Cpu) {
    if c.pc != 0x0022_d668 {
        return oc436_section_man_advance_resume(c);
    }
    c.cycles += 2;
    let skip = c.g[A1][0] == 0;
    c.g[A2][0] = c.g[A0][0];
    if !skip {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC436_SM_SWITCH_STATE));
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let out_of_range = c.g[V0][0] == 0;
        c.g[V0][0] = 0x48_0000;
        if !out_of_range {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4bd0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            let this = c.g[A2][0] as u32;
            match target {
                0x0022_d698 => {
                    c.cycles += 4;
                    c.g[V0][0] = 3;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_ARG), c.g[A1][0]);
                    let t = c.g[RA][0] as u32;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_STATE), c.g[V0][0]);
                    c.pc = t;
                    return;
                }
                0x0022_d6a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    let t = c.g[RA][0] as u32;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_STATE), c.g[V0][0]);
                    c.pc = t;
                    return;
                }
                0x0022_d6b4 => {
                    c.cycles += 2;
                    c.g[V0][0] = 4;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_STATE), c.g[V0][0]);
                }
                0x0022_d6bc => {}
                _ => {
                    c.jump_out(target, 0x0022_d668, 0x0022_d6c4);
                    return;
                }
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

#[cfg(not(feature = "ee-unwind"))]
fn oc436_section_man_advance_resume(c: &mut Cpu) {
    ee::s05::f_0022d668__entries(c)
}

// At install the orchestrator switches this to the generated twin.
#[cfg(feature = "ee-unwind")]
fn oc436_section_man_advance_resume(c: &mut Cpu) {
    ee::s05::f_0022d668__generated(c)
}

/// `ScriptCamBinding.state` (+0xc): 9 once unbound.
pub(crate) const OC436_BINDING_STATE: u32 = 0xc;

/// `void ScriptCamBinding_Unbind(ScriptCamBinding* b)` (0x283180):
/// `ScriptBinding_Unbind(b)`, then state (+0xc) = 9.
pub fn oc436_script_cam_binding_unbind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_3000, 0x0028_3194)?;
        c.g[V0][0] = 9;
        frame.restore_ra(c, 0);
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(OC436_BINDING_STATE),
            c.g[V0][0],
        );
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* InSeq_HeapNewArray(...)` (0x2724c8): tail call of `InSeq_HeapNew`.
pub fn oc436_in_seq_heap_new_array(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0027_2488), 0x0027_24d8)
}

/// `void AI_ResetRidersDefault(ai)` (0x1289f0): `AI_ResetRiders(ai, 0)`.
pub fn oc436_ai_reset_riders_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0012_97c8, 0x0012_8a00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Boost.vtable` (+0xc).
pub(crate) const OC436_BOOST_VTABLE: u32 = 0xc;

/// `void Boost_v01::~Boost_v01(Boost* b)` (0x3415a8): point the vtable
/// (+0xc) at the base class's table (0x4914e0), then `WorldObj_dtor(b)`.
pub fn oc436_boost_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14e0));
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(OC436_BOOST_VTABLE),
            c.g[V0][0],
        );
        call_ptr(c, 0x0034_fbf0, 0x0034_15c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Script* ScriptEngine_GetSeqProcessScript(?, SeqProcess* p)` (0x282bb0):
/// `p->owner(+0xc)->script(+0xc)`.
pub fn oc436_script_engine_get_seq_process_script(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.pc = t;
}

/// `void CameraTriggerStack_Clear(stack)` (0x16d200): zero +0 and +4.
pub fn oc436_camera_trigger_stack_clear(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A0][0] as u32).wrapping_add(4), 0);
    let t = c.g[RA][0] as u32;
    c.sw(c.g[A0][0] as u32, 0);
    c.pc = t;
}

/// `void ModObj_RunScriptEvent19b8(?, obj, a, b)` (0x34fe00): shift the
/// arguments down and call `WScriptMan_ObjTriggerHook(obj, a, b)`.
pub fn oc436_mod_obj_run_script_event_19b8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A0][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x002d_19b8, 0x0034_fe18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan* CameraTriggerMan_ClearData(CameraTriggerMan* m)`
/// (0x16bf20): zero the first word, then `CameraTriggerMan_ResetHeader(m)`;
/// v0 holds `m` before the call (the callee's result replaces it).
pub fn oc436_camera_trigger_man_clear_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(c.g[V0][0] as u32, 0);
        call_ptr(c, 0x0016_c390, 0x0016_bf34)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{check, put_words, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// The gate of this module's callee fakes (a word in the test machine's
    /// RAM): with it off, the slots run what they held before.
    const OC436_CTRL: u32 = DATA + 0xf_fe00;
    const OC436_MAGIC: u32 = 0x6f63_3436;
    /// What the `FindBankSlotById` fake returns (sign-extended).
    const OC436_BANK_RESULT: u32 = DATA + 0xf_fe10;

    /// A gated logging fake for the callee at `$addr` (`$gen`: its body).
    macro_rules! oc436_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC436_CTRL) == OC436_MAGIC {
                    let ra = c.g[RA][0] as u32;
                    return fake(c, $addr, ra);
                }
                fall_back(c, &$prev, $gen)
            }
        };
    }

    oc436_gated!(
        oc436_fake_unload,
        OC436_PREV_UNLOAD,
        0x0016_c9b0,
        ee::s02::f_0016c9b0__generated
    );
    oc436_gated!(
        oc436_fake_parse,
        OC436_PREV_PARSE,
        0x0016_c228,
        ee::s02::f_0016c228__generated
    );
    oc436_gated!(
        oc436_fake_update_triggers,
        OC436_PREV_UPDATE_TRIGGERS,
        0x0016_cf48,
        ee::s02::f_0016cf48__generated
    );
    oc436_gated!(
        oc436_fake_reset_chase,
        OC436_PREV_RESET_CHASE,
        0x0015_e030,
        ee::s01::f_0015e030__generated
    );
    oc436_gated!(
        oc436_fake_unload_bank,
        OC436_PREV_UNLOAD_BANK,
        0x0027_b410,
        ee::s07::f_0027b410__generated
    );
    oc436_gated!(
        oc436_fake_stream_man_ctor,
        OC436_PREV_STREAM_MAN_CTOR,
        0x003a_6d50,
        ee::s13::f_003a6d50__generated
    );
    oc436_gated!(
        oc436_fake_streamer_reset,
        OC436_PREV_STREAMER_RESET,
        0x003a_9188,
        ee::s13::f_003a9188__generated
    );
    oc436_gated!(
        oc436_fake_set_data,
        OC436_PREV_SET_DATA,
        0x0031_1290,
        ee::s10::f_00311290__generated
    );
    oc436_gated!(
        oc436_fake_binding_unbind,
        OC436_PREV_BINDING_UNBIND,
        0x0028_3000,
        ee::s07::f_00283000__generated
    );
    oc436_gated!(
        oc436_fake_heap_new,
        OC436_PREV_HEAP_NEW,
        0x0027_2488,
        ee::s07::f_00272488__generated
    );
    oc436_gated!(
        oc436_fake_reset_riders,
        OC436_PREV_RESET_RIDERS,
        0x0012_97c8,
        ee::s00::f_001297c8__generated
    );
    oc436_gated!(
        oc436_fake_world_obj_dtor,
        OC436_PREV_WORLD_OBJ_DTOR,
        0x0034_fbf0,
        ee::s11::f_0034fbf0__generated
    );
    oc436_gated!(
        oc436_fake_obj_trigger_hook,
        OC436_PREV_OBJ_TRIGGER_HOOK,
        0x002d_19b8,
        ee::s09::f_002d19b8__generated
    );
    oc436_gated!(
        oc436_fake_reset_header,
        OC436_PREV_RESET_HEADER,
        0x0016_c390,
        ee::s02::f_0016c390__generated
    );
    oc436_gated!(
        oc436_fake_create_local_table,
        OC436_PREV_CREATE_LOCAL_TABLE,
        0x0030_8b78,
        ee::s10::f_00308b78__generated
    );
    oc436_gated!(
        oc436_fake_set_spin_amount,
        OC436_PREV_SET_SPIN_AMOUNT,
        0x0011_9898,
        ee::s00::f_00119898__generated
    );

    static OC436_PREV_FIND_BANK: AtomicUsize = AtomicUsize::new(0);
    /// `FindBankSlotById`: a logged fake whose result the test forces
    /// (negative included).
    fn oc436_fake_find_bank(c: &mut Cpu) {
        if c.bus.read32(OC436_CTRL) == OC436_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, 0x0027_b578, ra);
            c.g[V0][0] = sx(c.bus.read32(OC436_BANK_RESULT));
            return;
        }
        fall_back(c, &OC436_PREV_FIND_BANK, ee::s07::f_0027b578__generated)
    }

    /// Test-private rider field: `Rider_LoadStep` calls left before the
    /// fake sets the load state to 7.
    const OC436_RIDER_STEPS: u32 = 0x884;
    static OC436_PREV_LOAD_STEP: AtomicUsize = AtomicUsize::new(0);
    /// `Rider_LoadStep`: a logged fake that finishes the load after the
    /// rider's step counter runs out.
    fn oc436_fake_load_step(c: &mut Cpu) {
        if c.bus.read32(OC436_CTRL) == OC436_MAGIC {
            let rider = c.g[A0][0] as u32;
            let steps = c.bus.read32(rider + OC436_RIDER_STEPS);
            if steps > 0 {
                c.bus.write32(rider + OC436_RIDER_STEPS, steps - 1);
                if steps == 1 {
                    c.bus.write32(rider + OC436_RIDER_LOAD_STATE, 7);
                }
            }
            let ra = c.g[RA][0] as u32;
            return fake(c, 0x0011_c298, ra);
        }
        fall_back(c, &OC436_PREV_LOAD_STEP, ee::s00::f_0011c298__generated)
    }

    #[allow(unused_unsafe)]
    fn oc436_install_fakes() {
        use std::ptr::addr_of_mut;
        unsafe {
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_0016C9B0),
                oc436_fake_unload,
                &OC436_PREV_UNLOAD,
            );
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_0016C228),
                oc436_fake_parse,
                &OC436_PREV_PARSE,
            );
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_0016CF48),
                oc436_fake_update_triggers,
                &OC436_PREV_UPDATE_TRIGGERS,
            );
            hook(
                addr_of_mut!(ee::s01::OVERRIDE_0015E030),
                oc436_fake_reset_chase,
                &OC436_PREV_RESET_CHASE,
            );
            hook(
                addr_of_mut!(ee::s07::OVERRIDE_0027B578),
                oc436_fake_find_bank,
                &OC436_PREV_FIND_BANK,
            );
            hook(
                addr_of_mut!(ee::s07::OVERRIDE_0027B410),
                oc436_fake_unload_bank,
                &OC436_PREV_UNLOAD_BANK,
            );
            hook(
                addr_of_mut!(ee::s13::OVERRIDE_003A6D50),
                oc436_fake_stream_man_ctor,
                &OC436_PREV_STREAM_MAN_CTOR,
            );
            hook(
                addr_of_mut!(ee::s13::OVERRIDE_003A9188),
                oc436_fake_streamer_reset,
                &OC436_PREV_STREAMER_RESET,
            );
            hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011C298),
                oc436_fake_load_step,
                &OC436_PREV_LOAD_STEP,
            );
            hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311290),
                oc436_fake_set_data,
                &OC436_PREV_SET_DATA,
            );
            hook(
                addr_of_mut!(ee::s07::OVERRIDE_00283000),
                oc436_fake_binding_unbind,
                &OC436_PREV_BINDING_UNBIND,
            );
            hook(
                addr_of_mut!(ee::s07::OVERRIDE_00272488),
                oc436_fake_heap_new,
                &OC436_PREV_HEAP_NEW,
            );
            hook(
                addr_of_mut!(ee::s00::OVERRIDE_001297C8),
                oc436_fake_reset_riders,
                &OC436_PREV_RESET_RIDERS,
            );
            hook(
                addr_of_mut!(ee::s11::OVERRIDE_0034FBF0),
                oc436_fake_world_obj_dtor,
                &OC436_PREV_WORLD_OBJ_DTOR,
            );
            hook(
                addr_of_mut!(ee::s09::OVERRIDE_002D19B8),
                oc436_fake_obj_trigger_hook,
                &OC436_PREV_OBJ_TRIGGER_HOOK,
            );
            hook(
                addr_of_mut!(ee::s02::OVERRIDE_0016C390),
                oc436_fake_reset_header,
                &OC436_PREV_RESET_HEADER,
            );
            hook(
                addr_of_mut!(ee::s10::OVERRIDE_00308B78),
                oc436_fake_create_local_table,
                &OC436_PREV_CREATE_LOCAL_TABLE,
            );
            hook(
                addr_of_mut!(ee::s00::OVERRIDE_00119898),
                oc436_fake_set_spin_amount,
                &OC436_PREV_SET_SPIN_AMOUNT,
            );
        }
    }

    /// A machine with scrambled registers, the fake hooks, a game-like gp,
    /// the callee fakes switched on and an unwind at a random call below
    /// `calls`.
    fn oc436_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc436_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC436_CTRL, OC436_MAGIC);
        r
    }

    /// A random word with the edge values zero and negative mixed in.
    fn oc436_word(r: &mut Rng) -> u32 {
        match r.next_u32() % 4 {
            0 => 0,
            1 => 0xffff_fff0,
            _ => r.next_u32(),
        }
    }

    /// `InSeq_ChannelGetHeaderField4`: a channel whose header (+0x10) points
    /// at a record with a random word at +4.
    #[test]
    fn oc436_in_seq_channel_get_header_field4_matches() {
        check(
            ee::s07::f_00274388__generated,
            oc436_in_seq_channel_get_header_field4,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x2743_88 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let ch = DATA + 0x100;
                let header = DATA + 0x200;
                c.g[A0][0] = sx(ch);
                c.bus.write32(ch + OC436_CHANNEL_HEADER, header);
                let w = oc436_word(&mut r);
                c.bus.write32(header + 4, w);
            },
        );
    }

    /// `ScriptEngine_GetSeqProcessScript`: `p->owner->script` through two
    /// pointers.
    #[test]
    fn oc436_script_engine_get_seq_process_script_matches() {
        check(
            ee::s07::f_00282bb0__generated,
            oc436_script_engine_get_seq_process_script,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x2825_b0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let (p, owner) = (DATA + 0x100, DATA + 0x200);
                c.g[A1][0] = sx(p);
                c.bus.write32(p + 0xc, owner);
                let w = oc436_word(&mut r);
                c.bus.write32(owner + 0xc, w);
            },
        );
    }

    /// `CameraTriggerStack_Clear`: both words are cleared whatever they held.
    #[test]
    fn oc436_camera_trigger_stack_clear_matches() {
        check(
            ee::s02::f_0016d200__generated,
            oc436_camera_trigger_stack_clear,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x16d2_00 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let stack = DATA + 0x100;
                c.g[A0][0] = sx(stack);
                let (a, b) = (r.next_u32(), r.next_u32());
                c.bus.write32(stack, a);
                c.bus.write32(stack + 4, b);
            },
        );
    }

    /// `CameraWobble_Init`: the two literals at gp-0x6c2c / gp-0x6c28 are
    /// copied into the object, the rest cleared.
    #[test]
    fn oc436_camera_wobble_init_matches() {
        check(
            ee::s01::f_0015fff0__generated,
            oc436_camera_wobble_init,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x15ff_f0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0015_fff0;
                c.g[GP][0] = GAME_GP as u64;
                let w = DATA + 0x100;
                c.g[A0][0] = sx(w);
                for k in 0..12 {
                    let v = r.next_u32();
                    c.bus.write32(w + 4 * k, v);
                }
                let (a, b) = (r.edge_f32(), r.edge_f32());
                c.bus.write32(GAME_GP - 0x6c2c, a);
                c.bus.write32(GAME_GP - 0x6c28, b);
            },
        );
    }

    /// `SectionMan_AdvanceEventSwitchSync`: `arg` zero or not, states 0..4
    /// (every jump-table case) and states out of range (5, 6, large,
    /// negative).
    #[test]
    fn oc436_section_man_advance_event_switch_sync_matches() {
        check(
            ee::s05::f_0022d668__generated,
            oc436_section_man_advance_event_switch_sync,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x22d6_68 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0022_d668;
                let table = [
                    0x0022_d698,
                    0x0022_d6a8,
                    0x0022_d6b4,
                    0x0022_d6bc,
                    0x0022_d6bc,
                ];
                put_words(c, OC436_SM_SWITCH_TABLE, &table);
                let sm = DATA + 0x100;
                c.g[A0][0] = sx(sm);
                let arg = if seed % 5 == 0 { 0 } else { r.next_u32() | 1 };
                c.g[A1][0] = sx(arg);
                let state = match seed % 9 {
                    0..=4 => seed % 9,
                    5 => 5,
                    6 => 6,
                    7 => 0x8000_0000 | seed,
                    _ => r.next_u32() % 3,
                };
                c.bus.write32(sm + OC436_SM_SWITCH_STATE, state);
                let old = r.next_u32();
                c.bus.write32(sm + OC436_SM_SWITCH_ARG, old);
            },
        );
    }

    /// The wrappers with one call each: the thunks, `AI_ResetRidersDefault`,
    /// `Boost_v01`, `ModObj_RunScriptEvent19b8`, `CameraTriggerMan_ClearData`,
    /// `cAdfDataSource_ctor`, `ScriptCamBinding_Unbind` and
    /// `RiderMetrix_SetGrindOrientation`: random registers, the object in
    /// a0 (or a1), the callee a gated logged fake that may unwind.
    #[test]
    fn oc436_one_call_wrappers_match() {
        type Fns = (GuestFn, GuestFn, u64);
        let all: [Fns; 9] = [
            (
                ee::s10::f_00308b58__generated,
                oc436_wscript_man_create_local_table_thunk,
                0x308b58,
            ),
            (ee::s07::f_002724c8__generated, oc436_in_seq_heap_new_array, 0x2724c8),
            (ee::s00::f_001289f0__generated, oc436_ai_reset_riders_default, 0x1289f0),
            (ee::s11::f_003415a8__generated, oc436_boost_v01, 0x3415a8),
            (
                ee::s11::f_0034fe00__generated,
                oc436_mod_obj_run_script_event_19b8,
                0x34fe00,
            ),
            (
                ee::s02::f_0016bf20__generated,
                oc436_camera_trigger_man_clear_data,
                0x16bf20,
            ),
            (ee::s10::f_00311220__generated, oc436_cadf_data_source_ctor, 0x311220),
            (
                ee::s07::f_00283180__generated,
                oc436_script_cam_binding_unbind,
                0x283180,
            ),
            (
                ee::s00::f_00119918__generated,
                oc436_rider_metrix_set_grind_orientation,
                0x119918,
            ),
        ];
        for (gen, hand, salt) in all {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = oc436_machine(c, seed, salt, 1);
                let obj = DATA + 0x1000 + (r.next_u32() & 0xff0);
                for reg in [A0, A1, A2, A3] {
                    c.g[reg][0] = sx(oc436_word(&mut r));
                }
                c.g[A0][0] = sx(obj);
                if salt == 0x34fe00 {
                    // `a1` is the object here.
                    c.g[A1][0] = sx(obj);
                }
                let old = r.next_u32();
                c.bus.write32(obj, old);
            });
        }
    }

    /// `ScriptEngine_UnloadScriptBank`: the bank slot found (>= 0), not found
    /// (-1 or any negative) and a fake that unwinds.
    #[test]
    fn oc436_script_engine_unload_script_bank_matches() {
        covered(
            ee::s07::f_00279888__generated,
            oc436_script_engine_unload_script_bank,
            SEEDS,
            &|c, seed| {
                let mut r = oc436_machine(c, seed, 0x279888, 2);
                let slot = match seed % 3 {
                    0 => 0xffff_ffff,
                    1 => 0x8000_0000 | r.next_u32(),
                    _ => r.next_u32() & 0x7fff_ffff,
                };
                c.bus.write32(OC436_BANK_RESULT, slot);
                c.g[A0][0] = sx(oc436_word(&mut r));
            },
        );
    }

    /// `CamController_ResetForRider`: a list of 0-3 nodes whose objects have
    /// a vtable method (a logged fake, with a `this` adjustment); the
    /// controller's camera is then reset (a fake) and the world helper at
    /// 0x2f41a8 runs for real on a world with no sections.
    #[test]
    fn oc436_cam_controller_reset_for_rider_matches() {
        covered(
            ee::s01::f_0015cc70__generated,
            oc436_cam_controller_reset_for_rider,
            SEEDS,
            &|c, seed| {
                let mut r = oc436_machine(c, seed, 0x15cc70, 6);
                // The world has no sections: the helper at 0x2f41a8 returns
                // at once.
                let world = DATA + 0x6000;
                c.bus.write32(GAME_GP - 0x848, world);
                let ctrl = DATA + 0x1000;
                let list = DATA + 0x1100;
                let cam = DATA + 0x1200;
                c.g[A0][0] = sx(ctrl);
                c.g[A1][0] = sx(r.next_u32());
                c.bus.write32(ctrl + OC436_CAMCTRL_LIST, list);
                c.bus.write32(ctrl + OC436_CAMCTRL_CAMERA, cam);
                let n = seed % 4;
                let mut next = 0;
                for i in (0..n).rev() {
                    let node = DATA + 0x1400 + 0x40 * i;
                    let obj = DATA + 0x1800 + 0x40 * i;
                    // The vtable: `this` adjustment (a random halfword) and
                    // a method that is a fake address.
                    let vt = DATA + 0x2000 + 0x40 * i;
                    let adjust = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + OC436_VT_ADJUST, adjust as u16);
                    c.bus.write32(vt + OC436_VT_METHOD, FAKE + 0x10 * i);
                    c.bus.write32(obj + OC436_OBJ_VTABLE, vt);
                    c.bus.write32(node, obj);
                    c.bus.write32(node + OC436_NODE_NEXT, next);
                    next = node;
                }
                c.bus.write32(list, next);
            },
        );
    }

    /// `CameraTriggerMan_Reload`: no triggers, negative count, 1-3 triggers
    /// (each has its mask word cleared); the callees are logged fakes
    /// (`UpdateTriggersMask2` runs for real on top of the `UpdateTriggers`
    /// fake).
    #[test]
    fn oc436_camera_trigger_man_reload_matches() {
        covered(
            ee::s02::f_0016ce30__generated,
            oc436_camera_trigger_man_reload,
            SEEDS,
            &|c, seed| {
                let mut r = oc436_machine(c, seed, 0x16ce30, 3);
                let man = DATA + 0x1000;
                let array = DATA + 0x1100;
                c.g[A0][0] = sx(man);
                c.g[A1][0] = sx(r.next_u32());
                let count = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    k => k - 1,
                };
                c.bus.write32(man + OC436_CTM_TRIGGERS, array);
                c.bus.write32(man + OC436_CTM_COUNT, count);
                for i in 0..4u32 {
                    let trigger = DATA + 0x1400 + 0x40 * i;
                    c.bus.write32(array + 4 * i, trigger);
                    let junk = r.next_u32() | 1;
                    c.bus.write32(trigger + OC436_TRIGGER_MASK, junk);
                }
            },
        );
    }

    /// `cWorldCache_ctor`: random object contents, the streamer reset and
    /// stream-manager constructors are fakes (the streamer constructor runs
    /// for real around its reset); the frame-count word at `*(gp+0x2a74) +
    /// 0x10` and the scale at gp-0x23c4 give the stored int.
    #[test]
    fn oc436_cworld_cache_ctor_matches() {
        covered(
            ee::s13::f_003a7dd0__generated,
            oc436_cworld_cache_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = oc436_machine(c, seed, 0x3a7dd0, 3);
                let this = DATA + 0x1000;
                for k in 0..8u32 {
                    let v = r.next_u32();
                    c.bus.write32(this + 4 * k, v);
                }
                c.g[A0][0] = sx(this);
                let cfg = DATA + 0x8000;
                c.bus.write32(GAME_GP + 0x2a74, cfg);
                let count = if seed % 4 == 0 {
                    0xffff_fff0
                } else {
                    r.next_u32() % 1000
                };
                c.bus.write32(cfg + 0x10, count);
                let scale = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                c.bus.write32(GAME_GP - 0x23c4, scale);
            },
        );
    }

    /// `Rider_Load`: immediate (one step), already loaded (state 7: no
    /// loop) and 1-3 steps through the yield loop (`Rider_LoadStep` is a fake
    /// that sets the state to 7 when the rider's step counter runs out;
    /// `THREAD_yieldticks` runs for real over the fake syscalls).
    #[test]
    fn oc436_rider_load_matches() {
        covered(ee::s00::f_0011c230__generated, oc436_rider_load, SEEDS, &|c, seed| {
            let mut r = oc436_machine(c, seed, 0x11c230, 6);
            let rider = DATA + 0x2000;
            c.g[A0][0] = sx(rider);
            let immediate = seed % 3 == 0;
            c.g[A1][0] = if immediate { sx(r.next_u32() | 1) } else { 0 };
            let state = if seed % 3 == 1 { 7 } else { r.next_u32() % 7 };
            c.bus.write32(rider + OC436_RIDER_LOAD_STATE, state);
            c.bus
                .write32(rider + OC436_RIDER_STEPS, 1 + r.next_u32() % 3);
        });
    }
}
