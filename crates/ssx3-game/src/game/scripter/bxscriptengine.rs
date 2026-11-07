//! `src/scripter/bxscriptengine.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_3;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at as at_4;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamestate::lw_at_3;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_2cc0, in_seq_get_instance, "InSeq_GetInstance"),
    (
        0x0027_d050,
        script_engine_remove_all_script_objs,
        "ScriptEngine_RemoveAllScriptObjs",
    ),
    (
        0x0028_2150,
        script_engine_register_script,
        "ScriptEngine_RegisterScript",
    ),
    (
        0x0028_2178,
        script_engine_unregister_script,
        "ScriptEngine_UnregisterScript",
    ),
    (
        0x0028_21a0,
        script_engine_start_script,
        "ScriptEngine_StartScript",
    ),
    (
        0x0028_22a0,
        script_engine_run_process,
        "ScriptEngine_RunProcess",
    ),
    (
        0x0028_2338,
        script_engine_stop_process_seq,
        "ScriptEngine_StopProcessSeq",
    ),
    (
        0x0028_2390,
        script_engine_stop_all_process_seqs,
        "ScriptEngine_StopAllProcessSeqs",
    ),
    (
        0x0028_23f0,
        script_engine_kill_all_processes,
        "ScriptEngine_KillAllProcesses",
    ),
    (
        0x0028_2450,
        script_engine_tick_process,
        "ScriptEngine_TickProcess",
    ),
    (
        0x0028_26d8,
        script_engine_pause_process,
        "ScriptEngine_PauseProcess",
    ),
    (
        0x0028_2720,
        script_engine_resume_process,
        "ScriptEngine_ResumeProcess",
    ),
    (
        0x0028_2798,
        script_engine_get_process_state,
        "ScriptEngine_GetProcessState",
    ),
    (
        0x0028_2838,
        script_engine_is_process_finished,
        "ScriptEngine_IsProcessFinished",
    ),
    (
        0x0028_28b8,
        script_engine_reset_process_table,
        "ScriptEngine_ResetProcessTable",
    ),
    (
        0x0028_2908,
        script_engine_launch_pending_process,
        "ScriptEngine_LaunchPendingProcess",
    ),
    (
        0x0028_29d0,
        script_engine_seq_listener_on_start,
        "ScriptEngine_SeqListenerOnStart",
    ),
    (
        0x0028_2a18,
        script_engine_seq_listener_on_stop,
        "ScriptEngine_SeqListenerOnStop",
    ),
    (
        0x0028_2b40,
        script_engine_seq_listener_on_create,
        "ScriptEngine_SeqListenerOnCreate",
    ),
    (
        0x0028_2b88,
        script_engine_seq_listener_on_destroy,
        "ScriptEngine_SeqListenerOnDestroy",
    ),
    (
        0x0028_2bf0,
        script_engine_get_process_in_seq,
        "ScriptEngine_GetProcessInSeq",
    ),
    (
        0x0028_2c38,
        script_engine_alloc_process,
        "ScriptEngine_AllocProcess",
    ),
    (
        0x0028_2c88,
        script_engine_clear_process,
        "ScriptEngine_ClearProcess",
    ),
    (
        0x0028_2cb0,
        script_engine_get_process,
        "ScriptEngine_GetProcess",
    ),
    (
        0x0028_2cd0,
        c_transition_man_script_obj_ctor,
        "cTransitionMan_ScriptObjCtor",
    ),
    (
        0x0028_2d30,
        c_transition_man_script_advance_timer,
        "cTransitionMan_ScriptAdvanceTimer",
    ),
    (
        0x0028_2ef0,
        script_obj_cam_has_binding,
        "ScriptObjCam_HasBinding",
    ),
    (
        0x0028_2f00,
        c_transition_man_script_pause,
        "cTransitionMan_ScriptPause",
    ),
    (
        0x0028_2f30,
        c_transition_man_script_resume,
        "cTransitionMan_ScriptResume",
    ),
    (
        0x0028_2f90,
        script_binding_clear_listener,
        "ScriptBinding_ClearListener",
    ),
    (0x0028_2fb8, script_binding_bind, "ScriptBinding_Bind"),
    (0x0028_3000, script_binding_unbind, "ScriptBinding_Unbind"),
    (
        0x0028_3038,
        script_binding_pause_seq,
        "ScriptBinding_PauseSeq",
    ),
    (
        0x0028_3060,
        script_binding_resume_seq,
        "ScriptBinding_ResumeSeq",
    ),
    (
        0x0028_3088,
        script_cam_binding_ctor,
        "ScriptCamBinding_ctor",
    ),
    (
        0x0028_30c8,
        script_cam_binding_bind,
        "ScriptCamBinding_Bind",
    ),
    (
        0x0028_31b0,
        script_cam_binding_accept_command,
        "ScriptCamBinding_AcceptCommand",
    ),
    (0x0028_3200, script_obj_ctor, "ScriptObj_ctor"),
    (0x0028_3228, script_obj_dtor, "ScriptObj_dtor"),
    (0x0028_3298, script_obj_register, "ScriptObj_Register"),
    (0x0028_3320, scriptobj_pause, "ScriptObj_Pause"),
    (0x0028_33a8, scriptobj_resume, "ScriptObj_Resume"),
    (0x0028_3430, script_obj_activate, "ScriptObj_Activate"),
    (0x0028_3440, script_obj_deactivate, "ScriptObj_Deactivate"),
    (
        0x0028_20b0,
        script_engine_update_processes,
        "ScriptEngine_UpdateProcesses",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;

    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_6;

    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;

    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;

    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;

    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;

    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;

    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE as IN_SEQ_INSTANCE_PAUSE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2;

    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_4;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_KILL_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ;

    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;

    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_5;

    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS as SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS_3;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;

    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {}

// ---------------------------------------------------------------------------
// Instrument-sequence instances.

/// `void* InSeq_GetInstance(Channel* ch, int index)` (0x272cc0): the instance
/// pointer `index * 0x58` into the table at `ch + 0x14`, returned as the record
/// plus 8 (null when the record's +0xc field is zero).
pub fn in_seq_get_instance(c: &mut Cpu) {
    const TABLE: u32 = 0x14;
    const RECORD: u32 = 0x58;
    const RECORD_FLAG: u32 = 0xc;
    c.cycles += 8;
    c.g[V1][0] = RECORD as u64;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let scaled = abi::mult(c, abi::arg(c, 1), RECORD);
    c.g[A1][0] = sx(scaled);
    let record = sx((table as u32).wrapping_add(scaled));
    c.g[V0][0] = record;
    let flag = Addr(record as u32).off(RECORD_FLAG).lw(c);
    c.g[V1][0] = flag;
    c.g[V0][0] = sx((record as u32).wrapping_add(0x8));
    if flag == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine: registration table.

/// `bool ScriptEngine_RegisterScript(ScriptEngine *engine, ScriptObj *obj,
/// int index)` (0x282150): register `obj` in the table at `engine + 0x2b0`
/// (8-byte entries); false when the slot is already occupied.
pub fn script_engine_register_script(c: &mut Cpu) {
    c.cycles += 6;
    const TABLE: u32 = 0x2b0;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V1][0] = table;
    let index = abi::arg(c, 2);
    c.g[A2][0] = sx(index << 3);
    let slot = (table as u32).wrapping_add(index << 3);
    c.g[A2][0] = sx(slot);
    let existing = Addr(slot).lw(c);
    c.g[V0][0] = existing;
    c.g[V0][0] = 0x1;
    if existing == 0 {
        c.cycles += 2;
        Addr(slot).sw(c, abi::arg64(c, 1));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `ScriptObj *ScriptEngine_UnregisterScript(ScriptEngine *engine, int index)`
/// (0x282178): clear the table slot at `engine + 0x2b0 + index*8`, returning the
/// old pointer (0 when the slot was empty).
pub fn script_engine_unregister_script(c: &mut Cpu) {
    c.cycles += 6;
    const TABLE: u32 = 0x2b0;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let index = abi::arg(c, 1);
    c.g[A1][0] = sx(index << 3);
    let slot = (table as u32).wrapping_add(index << 3);
    c.g[A1][0] = sx(slot);
    let old = Addr(slot).lw(c);
    c.g[V0][0] = old;
    if old == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        Addr(slot).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine: process records.

/// `void ScriptEngine_ClearProcess(ScriptEngine *engine, ScriptProcess *proc)`
/// (0x282c88): reset a process record's fields (the engine argument is unused).
pub fn script_engine_clear_process(c: &mut Cpu) {
    c.cycles += 10;
    let proc = Addr(abi::arg(c, 1));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    proc.off(0xc).sw(c, 0);
    proc.off(0x14).sw(c, c.g[V0][0]);
    proc.sw(c, 0);
    proc.off(0x4).sw(c, 0);
    proc.off(0x20).sw(c, 0);
    proc.off(0x10).sh(c, 0);
    proc.off(0x12).sh(c, 0);
    proc.off(0x18).sw(c, 0);
    abi::ret(c);
}

/// `ScriptProcess *ScriptEngine_GetProcess(ScriptEngine *engine, int index)`
/// (0x282cb0): the process at `engine + 0x2c0 + index*0x24`, or null when its
/// +0x20 field is zero.
pub fn script_engine_get_process(c: &mut Cpu) {
    c.cycles += 7;
    const PROCESSES: u32 = 0x2c0;
    const PROCESS_SIZE: u32 = 0x24;
    const PRESENT: u32 = 0x20;
    let engine = abi::arg(c, 0);
    let offset = abi::mult(c, abi::arg(c, 1), PROCESS_SIZE).wrapping_add(PROCESSES);
    c.g[A1][0] = sx(offset);
    let proc = engine.wrapping_add(offset);
    c.g[V0][0] = sx(proc);
    c.g[V1][0] = Addr(proc).off(PRESENT).lw(c);
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script objects.

/// `ScriptObj *ScriptObj_ctor(ScriptObj *obj, int handle)` (0x283200):
/// initialise the object: `obj[0] = handle`, `obj[+0x4] = -1`, `obj[+0x8] = 0`,
/// `obj[+0xc] = vtable`.
pub fn script_obj_ctor(c: &mut Cpu) {
    c.cycles += 9;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x0048_0000;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x2558));
    obj.sw(c, abi::arg64(c, 1));
    obj.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = abi::arg64(c, 0);
    obj.off(0x4).sw(c, c.g[V1][0]);
    obj.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `void ScriptObj_Activate(ScriptObj *obj)` (0x283430): increment the
/// reference count at +0x8.
pub fn script_obj_activate(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let count = obj.off(0x8).lw(c);
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    obj.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void ScriptObj_Deactivate(ScriptObj *obj)` (0x283440): decrement the
/// reference count at +0x8.
pub fn script_obj_deactivate(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let count = obj.off(0x8).lw(c);
    c.g[V0][0] = sx((count as u32).wrapping_sub(1));
    obj.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `bool ScriptObjCam_HasBinding(ScriptObjCam* cam, int id)` (0x282ef0): true
/// when the binding id at +0x14 equals `id`.
pub fn script_obj_cam_has_binding(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x14)) ^ id;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `void ScriptEngine_SeqListenerOnDestroy(ScriptEngine *self, SeqListener *l)`
/// (0x282b88): take the listener's process (`l->+0xc`), mark it destroyed
/// (`state = 7` at `+0x4`) and clear it.
pub fn script_engine_seq_listener_on_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 7; // process state: destroyed
        frame.save_ra(c, 0);
        let ctx = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A1][0] = ctx;
        Addr((ctx as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        callee::SCRIPT_ENGINE_CLEAR_PROCESS.call(c, 0x0028_2ba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int* ScriptEngine_GetProcessInSeq(eng, seq)` (0x282bf0): the process
/// record of `seq`, or 0 when the slot is empty.
pub fn script_engine_get_process_in_seq(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_2c00)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script bindings.

/// `void ScriptBinding_ClearListener(ScriptBinding* self)` (0x282f90):
/// detach the listener from the binding's channel.
pub fn script_binding_clear_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0;
            callee::IN_SEQ_CHANNEL_SET_LISTENER.call(c, 0x0028_2fac)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptBinding_PauseSeq(ScriptBinding* self)` (0x283038): pause the
/// sequence instance behind the binding's channel.
pub fn script_binding_pause_seq(c: &mut Cpu) {
    instance_pause_resume(c, callee::IN_SEQ_INSTANCE_PAUSE, 0x0028_3054)
}

/// `void ScriptBinding_ResumeSeq(ScriptBinding* self)` (0x283060): resume it.
pub fn script_binding_resume_seq(c: &mut Cpu) {
    instance_pause_resume(c, callee::IN_SEQ_INSTANCE_RESUME, 0x0028_307c)
}

/// `if (self->channel) InstancePause/Resume(self->channel->instance)`.
pub(crate) fn instance_pause_resume(c: &mut Cpu, target: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
            target.call(c, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// The `ScriptObjCam`/`cTransitionMan` script hooks: a virtual method on the
/// script binding at `*(this + 0x10)`, whose vtable is at `*(binding + 0xc)`.
pub(crate) fn script_binding_vcall(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let binding = lw_at(c, this as u64, 0x10);
        c.g[A1][0] = binding;
        let vt = lw_at(c, binding, 0xc);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, slot);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((binding as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cTransitionMan_ScriptPause(this)` 0x282f00: slot +0x10 of the script
/// binding's vtable.
pub fn c_transition_man_script_pause(c: &mut Cpu) {
    script_binding_vcall(c, 0x10, 0x0028_2f20)
}

/// `cTransitionMan_ScriptResume(this)` 0x282f30: slot +0x18 of the script
/// binding's vtable.
pub fn c_transition_man_script_resume(c: &mut Cpu) {
    script_binding_vcall(c, 0x18, 0x0028_2f50)
}

/// `void ScriptBinding_Unbind(ScriptBinding* binding)` (0x283000):
/// deactivate the binding in the script engine and mark it unbound.
pub fn script_binding_unbind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_3014)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_DEACTIVATE_BINDING.call(c, 0x0028_3020)?;
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptCamBinding* ScriptCamBinding_ctor(ScriptCamBinding* self)`
/// (0x283088): base constructor, then the camera-binding vtable and the
/// camera index 9 / field 0.
pub fn script_cam_binding_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR.call(c, 0x0028_309c)?;
        c.g[V0][0] = 9;
        c.g[V1][0] = 0x0048_0000;
        c.sw(p(r32(c, S0), 0xc), c.g[V0][0]);
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x23e0));
        c.sw(r32(c, S0), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(p(r32(c, S0), 0x10), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObj_Register(ScriptObj* self)` (0x283298): register with the
/// script engine (virtual slot 2 at `engine->vtable+0x10`) and store the id
/// it returns at `+4`.
pub fn script_obj_register(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE_2.call(c, 0x0028_32ac)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x2a8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(r32(c, V1)).off(0x10).lh(c);
        c.g[A2][0] = Addr(r32(c, V1)).off(0x14).lw(c);
        let func = r32(c, A2);
        c.g[RA][0] = sx(0x0028_32c4);
        c.g[A0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x0028_32c4 {
            return Err(crate::call::Unwound);
        }
        c.sw(p(r32(c, S0), 4), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_TickProcess(self, index)` (0x282450): tick the sequence
/// process in slot `index` of `self` when it is running (state 3 or 4);
/// returns 1 when it was ticked.
pub fn script_engine_tick_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_GET_PROCESS_2.call(c, 0x0028_2460)?;
        c.g[A0][0] = c.g[V0][0];
        let missing = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if missing {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let process = abi::r32(c, A0);
        c.cycles += 5;
        c.g[V0][0] = Addr(process.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if idle {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(process).lw(c);
        callee::IN_SEQ_INSTANCE_UPDATE.call(c, 0x0028_2488)?;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_PauseProcess(self, index)` (0x2826d8): pause the
/// sequence process in slot `index` (`self + index*0x24 + 0x2c0`) and mark it
/// stopped (`+0x4 = 5`) when the process has pending work (`process+0x2c`).
pub fn script_engine_pause_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x24;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A1, A1, V0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c0));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let slot = abi::r32(c, S0);
        c.g[A0][0] = Addr(slot).lw(c);
        callee::IN_SEQ_INSTANCE_PAUSE_2.call(c, 0x0028_26fc)?;
        c.g[V0][0] = Addr(slot).lw(c);
        let process = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(process.wrapping_add(0x2c)).lw(c);
        let pending = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = 5;
        if !pending {
            c.cycles += 1;
            Addr(slot.wrapping_add(4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_SeqListenerOnStart(self, instance)` (0x2829d0): the
/// sequence's start callback: call the listener slot (`self+0x2a8`, entry 10)
/// with the instance's ids and mark the instance started (`+0x4 = 4`).
pub fn script_engine_seq_listener_on_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        let instance = c.g[V0][0] as u32;
        c.g[S0][0] = Addr(instance.wrapping_add(0xc)).lw(c);
        let state = abi::r32(c, S0);
        c.g[V1][0] = Addr(self_.wrapping_add(0x2a8)).lw(c);
        c.g[A2][0] = Addr(state.wrapping_add(8)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[A3][0] = Addr(vtable.wrapping_add(0x50)).lh(c);
        c.g[T0][0] = Addr(vtable.wrapping_add(0x54)).lw(c);
        let target = c.g[T0][0] as u32;
        c.g[A0][0] = sx(self_.wrapping_add(abi::r32(c, A3)));
        c.g[A3][0] = Addr(state.wrapping_add(0xc)).lw(c);
        call_ptr(c, target, 0x0028_2a00)?;
        c.g[V0][0] = 4;
        Addr(state.wrapping_add(4)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_SeqListenerOnCreate(self, obj)` (0x282b40): allocate a
/// script thread for `obj`, mark it "stop requested" (`+0x4 = 2`, handle
/// `+0x8 = -1`), store `obj` at `+0` and hand the thread to the instance.
pub fn script_engine_seq_listener_on_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_ENGINE_ALLOC_PROCESS.call(c, 0x0028_2b58)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 2;
        let thread = abi::r32(c, V0);
        Addr(thread.wrapping_add(8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Addr(thread.wrapping_add(4)).sw(c, c.g[V1][0]);
        Addr(thread).sw(c, c.g[S0][0]);
        callee::IN_SEQ_INSTANCE_SET_USER_DATA.call(c, 0x0028_2b78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptBinding_Bind(self, binding)` (0x282fb8): remember `binding`
/// at `self+8` and activate it with the instance's current key.
pub fn script_binding_bind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, S0);
        let binding = abi::r32(c, S1);
        Addr(self_.wrapping_add(8)).sw(c, c.g[S1][0]);
        callee::SCRIPT_ENGINE_GET_INSTANCE_3.call(c, 0x0028_2fd8)?;
        c.g[V1][0] = Addr(binding.wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        let state = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(state).lw(c);
        callee::SCRIPT_ENGINE_ACTIVATE_BINDING.call(c, 0x0028_2fec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A28A4`: the script-engine singleton.
pub(crate) const SCRIPT_ENGINE_GP: i32 = -0x84c;

/// `void ScriptEngine_ResetProcessTable(ScriptEngine* e)` (0x2828b8): clear
/// the process count at +0x2b8/+0x2bc and the sixteen 0x24-byte process
/// entries at +0x2c0: id = i, +0x2d4 = -1, everything else 0.
/// 6 + 16×12 + 2 cycles.
pub fn script_engine_reset_process_table(c: &mut Cpu) {
    c.cycles += 6;
    let base = r32(c, A0);
    c.sw(p(base, 0x2b8), 0);
    c.g[V1][0] = sx(base);
    c.sw(p(base, 0x2bc), 0);
    c.g[A1][0] = 0;
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    let mut slot = base;
    loop {
        c.cycles += 12;
        c.sw(p(slot, 0x2dc), c.g[A1][0]);
        c.sw(p(slot, 0x2e0), 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.sw(p(slot, 0x2c0), 0);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 16i64) as u64;
        c.sw(p(slot, 0x2c4), 0);
        Addr(p(slot, 0x2d0)).sh(c, 0);
        Addr(p(slot, 0x2d2)).sh(c, 0);
        c.sw(p(slot, 0x2d8), 0);
        c.sw(p(slot, 0x2d4), c.g[A0][0]);
        let more = c.g[V0][0] != 0;
        slot += 0x24;
        c.g[V1][0] = sx(slot);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0028_28d0);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}

/// `ScriptThread* ScriptEngine_AllocProcess(ScriptEngine* e, int type)`
/// (0x282c38): find the first of the sixteen process entries at +0x2c0 whose
/// type word (+0x2e0) is 0, set it to `type` (a type-1 process gets handle -1)
/// and return the entry; 0 when the table is full.
/// 6 + 3 (+4) + 2 / 2 (+1) + 2 cycles.
pub fn script_engine_alloc_process(c: &mut Cpu) {
    c.cycles += 6;
    let base = r32(c, A0);
    let mut type_slot = p(base, 0x2e0);
    c.g[V1][0] = sx(type_slot);
    let ty = c.g[A1][0];
    c.g[A2][0] = 0;
    c.g[T0][0] = 1;
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(base, 0x2c0));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(type_slot);
        if c.g[V0][0] != 0 {
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x24));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 16i64) as u64;
            let more = c.g[V0][0] != 0;
            type_slot += 0x24;
            c.g[V1][0] = sx(type_slot);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0028_2c50);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let plain = ty != c.g[T0][0];
        c.sw(type_slot, c.g[A1][0]);
        if !plain {
            c.cycles += 1;
            c.sw(type_slot.wrapping_sub(0x18), c.g[A3][0]);
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
}

/// `bool ScriptCamBinding_AcceptCommand(ScriptCamBinding* b, void* cmd, int n)`
/// (0x2831b0): true when `n < 4` and the instance script found for
/// `cmd->+8` has the same owner word (+0xc) as the binding's `+0x10 -> 0xf8`.
/// 6 (+9) + 1 or +4 cycles plus the callee.
pub fn script_cam_binding_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = ((c.g[A2][0] as i64) < 4i64) as u64;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let too_many = c.g[A2][0] == 0;
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if too_many {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
            c.g[A1][0] = c.lw(p(r32(c, A1), 8));
            callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT.call(c, 0x0028_31d4)?;
            c.g[V1][0] = c.lw(p(self_, 0x10));
            c.g[V0][0] = c.lw(p(r32(c, V0), 0xc));
            c.g[A0][0] = c.lw(p(r32(c, V1), 0xf8));
            c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_StopProcessSeq(ScriptEngine* e, int a1)` (0x282338):
/// stop the sequence process bound to the process returned by
/// `ScriptEngine_GetProcess`.
pub fn script_engine_stop_process_seq(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if callee::SCRIPT_ENGINE_GET_PROCESS_3
        .call(c, 0x0028_234c)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = c.g[V0][0];
    c.g[A1][0] = Addr(p_2(c, V1, 0)).lw(c);
    if c.g[A1][0] == 0 {
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(p_2(c, A1, 0x20)).lw(c);
    let skip_tail = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !skip_tail {
        c.cycles += 3;
        c.g[V1][0] = Addr(p_2(c, V1, 0x20)).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_2(c, S0, 0x2b8)).lw(c);
        c.g[A1][0] = Addr(p_2(c, A1, 0)).lw(c);
        if callee::IN_SEQ_STOP.call(c, 0x0028_2380).is_err() {
            return;
        }
    }
    // Both exits of the inner block charge this one cycle.
    c.cycles += 1;
    let _ = skip_tail;
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ScriptEngine_StopAllProcessSeqs(ScriptEngine* self)` (0x282390):
/// stop the sequence of each of the 16 active slots at `+0x2C0` (0x24 bytes
/// apart).
pub fn script_engine_stop_all_process_seqs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(ea(c.g[S1][0], 0x2c0));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xf;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(ea(c.g[S0][0], 0x1c));
                c.g[A0][0] = c.g[S1][0];
                callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ.call(c, 0x0028_23c8)?;
            }
            c.g[S2][0] = sx(ea(c.g[S2][0], 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(ea(c.g[S0][0], 0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_23b0);
            }
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

/// `void ScriptEngine_KillAllProcesses(ScriptEngine* self)` (0x2823F0): kill
/// each of the 16 slots at `+0x2C0` whose state word at `+0x20` is set.
pub fn script_engine_kill_all_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(ea(c.g[S1][0], 0x2c0));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xf;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x20));
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(ea(c.g[S0][0], 0x1c));
                c.g[A0][0] = c.g[S1][0];
                callee::SCRIPT_ENGINE_KILL_PROCESS.call(c, 0x0028_2428)?;
            }
            c.g[S2][0] = sx(ea(c.g[S2][0], 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(ea(c.g[S0][0], 0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_2410);
            }
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

/// `int ScriptEngine_GetProcessState(ScriptEngine* self, int id)` (0x282798):
/// a pending process is launched (and its new state returned); a live one
/// just reports its state word; no process is 0.
pub fn script_engine_get_process_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_4.call(c, 0x0028_27b0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = c.lw(ea(c.g[S0][0], 4));
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(ea(c.g[S0][0], 4));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS.call(c, 0x0028_27dc)?;
                c.g[V0][0] = c.lw(ea(c.g[S0][0], 4));
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Transitions, memory and pads.

/// `void cTransitionMan_ScriptObjCtor(cTransitionMan_ScriptObj* self)`
/// (0x282cd0): construct the script-object base and clear the transition
/// fields.
pub fn c_transition_man_script_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0028_3200, 0x0028_2ce8)?;
        c.g[V1][0] = 0x482_060;
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = sx(0x0048_2060);
        c.g[A0][0] = 0x9;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x14)).sw(c, c.g[A1][0]);
        Addr(at(c, S0, 0x18)).sw(c, c.g[A0][0]);
        Addr(at(c, S0, 0x10)).sw(c, 0);
        Addr(at(c, S0, 0x2c)).sw(c, 0);
        Addr(at(c, S0, 0x1c)).sw(c, 0);
        Addr(at(c, S0, 0x20)).sw(c, 0);
        Addr(at(c, S0, 0x24)).sw(c, 0);
        Addr(at(c, S0, 0x28)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine listeners.

/// `ScriptEngine_SeqListenerOnStop(ScriptEngine* self, ScriptSeqMsg* msg)`
/// (0x282a18): tell the engine's sequence callback (vtable slot 11) that the
/// sequence stopped, mark the sequence object's state 2 and kill the process
/// when it is still running.
pub fn script_engine_seq_listener_on_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        c.g[V0][0] = lw(c, p(this as u32, 0x2a8));
        c.g[S0][0] = lw(c, p(msg as u32, 0xc));
        let seq = c.g[S0][0] as u32;
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x58));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x5c));
        c.g[A2][0] = lw(c, p(seq, 8));
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V1][0] as u32;
        c.g[A3][0] = lw(c, p(seq, 0xc));
        call_ptr(c, target, 0x0028_2a4c)?;
        c.g[V0][0] = 2;
        c.g[V1][0] = lh(c, p(seq, 0x10));
        let done = c.g[V1][0] == 0;
        sw(c, p(seq, 4), c.g[V0][0]);
        if !done {
            c.cycles += 3;
            c.g[A1][0] = lw(c, p(seq, 0x1c));
            c.g[A0][0] = this;
            call_ptr(c, 0x0028_2540, 0x0028_2a68)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObj_dtor(ScriptObj* self, int flags)` (0x283228): release the
/// script object (unregister it when its registration word is 0 or -1) and
/// optionally free it.
pub fn script_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4825_58;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(at_2(c, S0, 4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
        let t = c.g[V1][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 4;
            call_ptr(c, 0x0028_32d8, 0x0028_3264)?;
            c.g[V0][0] = u64::MAX;
            Addr(at_2(c, S0, 4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0028_3280)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_RemoveAllScriptObjs(ScriptEngine* e)` (0x27d050):
/// remove the first 20 script objects, calling virtual slot 3 on each.
pub fn script_engine_remove_all_script_objs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 18, 0x10);
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !empty {
                c.cycles += 5;
                c.g[V1][0] = Addr(p_3(r32(c, S2), 0x2a8)).lw(c);
                c.g[A0][0] = Addr(p_3(c.g[V1][0] as u32, 0x18)).lh(c);
                c.g[V0][0] = Addr(p_3(c.g[V1][0] as u32, 0x1c)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                call::call_ptr(c, target, 0x0027_d09c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 20) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_d078);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ScriptEngine_ResumeProcess(ScriptEngine* e, int idx)` (0x282720):
/// resume the in-sequence instance in slot `idx` and, when it was stopped but
/// has re-entered a runnable state, move its slot state to 3 or 4.
pub fn script_engine_resume_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x24;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A1, A1, V0);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c0));
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        call::call_ptr(c, 0x0027_4a08, 0x0028_2744)?;
        c.g[V1][0] = Addr(p_3(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = 5;
        if c.g[V1][0] != c.g[V0][0] {
            lqsp(c, 16, 0x10);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = Addr(p_3(c.g[V1][0] as u32, 0x2c)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                lqsp(c, 16, 0x10);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_3(c.g[V1][0] as u32, 0x20)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 4;
                if !t {
                    c.cycles += 1;
                    c.g[V0][0] = 3;
                }
                c.cycles += 2;
                Addr(p_3(r32(c, S0), 4)).sw(c, c.g[V0][0]);
                lqsp(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `cTransitionMan_ScriptAdvanceTimer` (0x282D30): advance a script transition's
/// timer by the frame delta and stop the transition when the hold time has
/// elapsed.
pub fn c_transition_man_script_advance_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw_at_2(c, A0, 0x14);
        let no_timer = (c.g[A1][0] as i64) < 0;
        c.g[RA][0] = Addr(frame.at(c, 0)).ld(c);
        if !no_timer {
            c.cycles += 15;
            c.lwc1(1, r32(c, A0).wrapping_add(0x20));
            c.lwc1(0, r32(c, A0).wrapping_add(0x1c));
            c.lwc1(2, gp(c, -0x4780));
            c.add_s(0, 0, 1);
            c.lwc1(4, r32(c, A0).wrapping_add(0x24));
            c.mul_s(2, 12, 2);
            c.lwc1(1, r32(c, A0).wrapping_add(0x2c));
            c.lwc1(3, gp(c, -0x477c));
            c.add_s(0, 0, 4);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 3);
            c.c_le_s(0, 1);
            // The stop arm is the one where `(timer <= target)` holds: the
            // generated code breaks out when the comparison is false.
            let stop = c.fcc();
            c.swc1(1, r32(c, A0).wrapping_add(0x2c));
            if stop {
                c.cycles += 7;
                c.mtc1(ZERO, 12);
                c.g[A2][0] = 9;
                c.mov_s(13, 12);
                c.mov_s(14, 12);
                c.mov_s(15, 12);
                callee::STOP_TRANSITION.call(c, 0x0028_2d98)?;
                c.g[RA][0] = Addr(frame.at(c, 0)).ld(c);
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_IsProcessFinished(self, a1)` (0x00282838): after a
/// process reaches its end state, launch it and report completion.
pub fn script_engine_is_process_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_5.call(c, 0x0028_2850)?;
        c.g[S0][0] = c.g[V0][0];
        let absent = c.g[S0][0] == 0;
        c.g[V0][0] = 1;
        if !absent {
            c.cycles += 3;
            c.g[V1][0] = Addr(at_3(c, S0, 0x4)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = Addr(at_3(c, S0, 0x4)).lw(c);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2.call(c, 0x0028_2874)?;
                c.g[V0][0] = Addr(at_3(c, S0, 0x4)).lw(c);
            }
            c.cycles += 4;
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(4));
            c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                c.g[V1][0] = Addr(at_3(c, S0, 0)).lw(c);
                c.g[V0][0] = Addr(at_3(c, V1, 0x20)).lw(c);
                c.g[V0][0] ^= 4;
                c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObj_Pause(self)` (0x283320): pause every running sequence of
/// the object's binding.
pub fn scriptobj_pause(c: &mut Cpu) {
    script_obj_pause_resume(
        c,
        ScriptLoop {
            get_count_ret: 0x0028_3350,
            each: callee::SCRIPT_BINDING_PAUSE_SEQ,
            each_ret: 0x0028_3380,
            seq_ret: 0x0028_3378,
            head: 0x0028_3360,
        },
    );
}

/// `void ScriptObj_Resume(self)` (0x2833a8): resume every paused sequence.
pub fn scriptobj_resume(c: &mut Cpu) {
    script_obj_pause_resume(
        c,
        ScriptLoop {
            get_count_ret: 0x0028_33d8,
            each: callee::SCRIPT_BINDING_RESUME_SEQ,
            each_ret: 0x0028_3408,
            seq_ret: 0x0028_3400,
            head: 0x0028_33e8,
        },
    );
}

/// The addresses that differ between the pause and resume variants.
pub(crate) struct ScriptLoop {
    get_count_ret: u32,
    each: Callee,
    each_ret: u32,
    seq_ret: u32,
    head: u32,
}

pub(crate) fn script_obj_pause_resume(c: &mut Cpu, l: ScriptLoop) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_at_3(c, p(r32(c, S1), 0xc));
        let func = vslot(c, c.g[V1][0], 0x30, V0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, l.get_count_ret)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[S2][0] as i64) <= 0 {
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw_at_3(c, p(r32(c, S1), 0xc));
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let func = vslot(c, c.g[V1][0], 0x38, V0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, l.seq_ret)?;
                c.g[A0][0] = c.g[V0][0];
                l.each.call(c, l.each_ret)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = lw_at_3(c, p(r32(c, S1), 0xc));
                if event_due(c) {
                    deliver_event_here(c, l.head);
                }
            }
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_DestroyScriptDatabase(ScriptEngine* self)` (0x282020):
/// stop and kill everything, drop the script object list, the process table
/// and the sequence bank, and reset the process table.
pub fn script_engine_destroy_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(at_4(c, S0, 0x2b0));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            call_ptr(
                c,
                callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2,
                0x0028_2044,
            )?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES, 0x0028_204c)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(
                c,
                callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2,
                0x0028_2054,
            )?;
            c.g[A0][0] = c.lw(at_4(c, S0, 0x2b0));
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.lw(at_4(c, S0, 0x2b8));
            } else {
                c.cycles += 3;
                call_ptr(c, callee::OPERATOR_DELETE, 0x0028_2068)?;
                c.g[A0][0] = c.lw(at_4(c, S0, 0x2b8));
            }
            c.cycles += 3;
            c.sw(at_4(c, S0, 0x2b0), 0);
            let no_seq = c.g[A0][0] == 0;
            c.sw(at_4(c, S0, 0x2b4), 0);
            if !no_seq {
                c.cycles += 2;
                c.g[A1][0] = 3;
                call_ptr(c, callee::IN_SEQ_DTOR, 0x0028_2080)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(at_4(c, S0, 0x2bc));
            let no_bank = c.g[A0][0] == 0;
            c.sw(at_4(c, S0, 0x2b8), 0);
            if !no_bank {
                c.cycles += 2;
                call_ptr(c, callee::OPERATOR_DELETE, 0x0028_2094)?;
            }
            c.cycles += 4;
            c.sw(at_4(c, S0, 0x2bc), 0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE, 0x0028_20a0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_RunProcess(self, arg, value, unused)` (0x2822a0): run a
/// script process: store the 16-bit value and, if the process is armed, play
/// its sequence.
pub fn script_engine_run_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_6.call(c, 0x0028_22c8)?;
        c.g[S0][0] = c.g[V0][0];
        let mut run = false;
        if c.g[S0][0] != 0 {
            c.g[V1][0] = lw_2(c, S0, 0x4);
            run = true;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        if run {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let wrong_state = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if !wrong_state {
                c.cycles += 4;
                c.g[V1][0] = lw_2(c, S0, 0x20);
                c.g[V0][0] = 1;
                let not_armed = c.g[V1][0] != c.g[V0][0];
                Addr(c.g[S0][0] as u32).off(0x10).sh(c, c.g[S1][0]);
                if !not_armed {
                    c.cycles += 7;
                    c.g[V0][0] = 3;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[V0][0]);
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A0][0] = lw_2(c, S2, 0x2b8);
                    c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                    callee::IN_SEQ_PLAY.call(c, 0x0028_2314)?;
                }
                c.cycles += 1;
                c.g[V0][0] = lw_2(c, S0, 0x4);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script camera binding.

/// `void ScriptCamBinding_Bind(ScriptCamBinding* b, void* channel)` (0x2830c8):
/// bind the script camera to the channel and copy the camera flags appropriate
/// to its parameter type (`+0x21` for manual/target/subject, `+0x1c` for an
/// algorithmic camera).
pub fn script_cam_binding_bind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let channel = r(c, A1);
        c.g[S0][0] = sx(channel);
        let this = r(c, A0);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this);
        callee::SCRIPT_BINDING_BIND.call(c, 0x0028_30e4)?;
        c.g[RA][0] = sx(0x0028_30ec);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0027_43c8, 0x0028_30ec)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let manual = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        'outer: {
            'store: {
                'alg: {
                    'subj: {
                        'target: {
                            if manual {
                                break 'target;
                            }
                            'zero: {
                                'cmp: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0 {
                                        c.g[V0][0] = 2;
                                        break 'cmp;
                                    }
                                    c.cycles += 2;
                                    let is_zero = c.g[V1][0] == 0;
                                    if is_zero {
                                        break 'zero;
                                    }
                                    c.cycles += 2;
                                    frame.restore(c, 16, 0x20);
                                    break 'outer;
                                }
                                c.cycles += 2;
                                let is_two = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 3;
                                if is_two {
                                    break 'subj;
                                }
                                c.cycles += 2;
                                let is_three = c.g[V1][0] == c.g[V0][0];
                                if is_three {
                                    break 'alg;
                                }
                                c.cycles += 2;
                                frame.restore(c, 16, 0x20);
                                break 'outer;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0028_3134);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0027_c098, 0x0028_3134)?;
                            c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                            break 'store;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0028_3144);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_c2a8, 0x0028_3144)?;
                        c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                        break 'store;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0028_3154);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_c4b8, 0x0028_3154)?;
                    c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                    break 'store;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0028_3164);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_c6a0, 0x0028_3164)?;
                c.g[V1][0] = Addr(p(r(c, V0), 0x1c)).lb(c);
                break 'store;
            }
            c.cycles += 2;
            Addr(p(r(c, S1), 0xc)).sw(c, c.g[V1][0]);
            frame.restore(c, 16, 0x20);
            break 'outer;
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_LaunchPendingProcess(this, script)` (0x282908): start the
/// script's pending sequence, bind the instance as the thread's owner and
/// bump the slot's use count.
pub fn script_engine_launch_pending_process(c: &mut Cpu) {
    'b_2829b0: {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2b0));
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        let t = c.g[A0][0] == 0u64;
        c.g[V1][0] = 0x2;
        if t {
            break 'b_2829b0;
        } else {
            'b_2829a8: {
                c.cycles += 21;
                c.g[V0][0] = 0xffffffffffffffff;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x12), 0u64);
                c.mtc1(0, 20);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2b8));
                c.g[RA][0] = sx(0x00282968);
                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                c.pc = 0x00272938;
                ee::s07::InSeq_StartSequence(c);
                if c.pc != 0x00282968 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2b8));
                c.g[RA][0] = sx(0x00282974);
                c.g[A1][0] = c.g[V0][0];
                c.pc = 0x00272cc0;
                ee::s07::InSeq_GetInstance(c);
                if c.pc != 0x00282974 {
                    return;
                }
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00282984);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.pc = 0x00274c10;
                ee::s07::InSeq_InstanceSetUserData(c);
                if c.pc != 0x00282984 {
                    return;
                }
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.c_lt_s(20, 12);
                if !c.fcc() {
                    c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x4));
                    break 'b_2829a8;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002829a0);
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.pc = 0x00274808;
                    ee::s07::InSeq_InstanceSetSpeed(c);
                    if c.pc != 0x002829a0 {
                        return;
                    }
                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x4));
                    break 'b_2829a8;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            break 'b_2829b0;
        }
    }
    c.cycles += 7;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}

/// `ScriptEngine_CreateScriptDatabase(self, count, a2, a3)` (0x281f30):
/// create the script database: allocate the header, construct the
/// instruction sequence, allocate one 8-byte record per script and clear
/// them.
pub fn scriptengine_create_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        let _count = abi::arg(c, 1);
        let _a2 = abi::arg(c, 2);
        let _size = abi::arg(c, 3);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f48));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0028_1f70);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_1f70)?; // operator new(size, ...)
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34f0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0028_1f8c);
        c.sw(p(this, 0x2bc), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0028_1f8c)?; // operator new(0x20, ...)
        c.g[A1][0] = lw_3(c, p(this, 0x2bc));
        c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2ac));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0028_1fa4);
        c.g[A3][0] = c.g[S2][0];
        c.pc = 0x0027_2788;
        ee::s07::InSeq_ctor(c);
        if c.pc != 0x0028_1fa4 {
            return Err(Unwound);
        }
        c.sw(p(this, 0x2b4), c.g[S1][0]);
        c.g[A1][0] = 0x48_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f58));
        c.sw(p(this, 0x2b8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 3);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0028_1fc4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_1fc4)?; // operator new(count * 8, ...)
        c.g[V1][0] = lw_3(c, p(this, 0x2b4));
        c.g[A1][0] = 0;
        let t = (c.g[V1][0] as i64) <= 0;
        c.sw(p(this, 0x2b0), c.g[V0][0]);
        if !t {
            c.cycles += 1;
            c.g[V1][0] = lw_3(c, p(this, 0x2b0));
            loop {
                c.cycles += 11;
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V1][0] as u32, 0);
                c.g[V0][0] = lw_3(c, p(this, 0x2b0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh(p(c.g[A0][0] as u32, 4), 0);
                c.g[V0][0] = lw_3(c, p(this, 0x2b4));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw_3(c, p(this, 0x2b0));
                if event_due(c) {
                    deliver_event_here(c, 0x0028_1fd8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_002821A0(ScriptEngine* self, int key, void* arg)` (0x2821a0):
/// start the script bound to slot `key` of the engine's script table: allocate
/// a thread, and either queue it for a sequence (when the slot has data) or
/// hand it to the slot's start callback; a refused start tears the thread down
/// again and returns -1.
pub fn script_engine_start_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 20, 0x10);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        c.g[A1][0] = 0x1;
        c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x2b0));
        c.g[RA][0] = sx(0x0028_21dc);
        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.pc = 0x0028_2c38;
        ee::s07::ScriptEngine_AllocProcess(c);
        if c.pc != 0x0028_21dc {
            return Err(Unwound);
        }
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S3][0] as u32); // slot data
        let has_data = c.g[V0][0] != 0;
        c.g[V0][0] = 0x2;
        let mut refused = false;
        if has_data {
            c.cycles += 16;
            c.sw(p(c.g[S0][0] as u32, 8), c.g[S2][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x2b8));
            c.g[RA][0] = sx(0x0028_224c);
            c.g[A1][0] = c.lw(c.g[S3][0] as u32);
            c.pc = 0x0027_2938;
            ee::s07::InSeq_StartSequence(c);
            if c.pc != 0x0028_224c {
                return Err(Unwound);
            }
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x2b8));
            c.g[RA][0] = sx(0x0028_2258);
            c.g[A1][0] = c.g[V0][0];
            c.pc = 0x0027_2cc0;
            ee::s07::InSeq_GetInstance(c);
            if c.pc != 0x0028_2258 {
                return Err(Unwound);
            }
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0028_2268);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.pc = 0x0027_4c10;
            ee::s07::InSeq_InstanceSetUserData(c);
            if c.pc != 0x0028_2268 {
                return Err(Unwound);
            }
            c.g[V1][0] = c.lhu(p(c.g[S3][0] as u32, 4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.sh(p(c.g[S3][0] as u32, 4), c.g[V1][0]);
            c.sw(p(c.g[S0][0] as u32, 0xc), c.g[S4][0]);
        } else {
            c.cycles += 12;
            c.g[V0][0] = 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x14), c.g[S2][0]);
            c.sh(p(c.g[S0][0] as u32, 0x12), c.g[V0][0]);
            c.g[A1][0] = c.g[S2][0];
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x2a8));
            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x38));
            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x3c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0028_2214);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0028_2214)?;
            if c.g[V0][0] != 0 {
                c.sw(p(c.g[S0][0] as u32, 0xc), c.g[S4][0]);
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0x7;
                c.g[A0][0] = c.g[S1][0];
                c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
                c.g[RA][0] = sx(0x0028_2230);
                c.g[A1][0] = c.g[S0][0];
                c.pc = 0x0028_2c88;
                ee::s07::ScriptEngine_ClearProcess(c);
                if c.pc != 0x0028_2230 {
                    return Err(Unwound);
                }
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                refused = true;
            }
        }
        if !refused {
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x1c));
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- ScriptEngine_UpdateProcesses

/// `ScriptEngine` layout: the sequence player (+0x2b8) and sixteen 0x24-byte
/// process slots from +0x2c0 (type at +0x4, state at +0x20; 1 = pending).
pub(crate) const SE_SEQ_PLAYER: u32 = 0x2b8;

pub(crate) const SE_SLOTS: u32 = 0x2c0;

pub(crate) const SE_SLOT_SIZE: u64 = 0x24;

pub(crate) const SE_SLOT_TYPE: u32 = 0x4;

pub(crate) const SE_SLOT_STATE: u32 = 0x20;

/// The slot-type switch's jump table (six entries, `.rodata`).
pub(crate) const SE_TYPE_TABLE: u32 = 0x0048_1f70;

/// `InSeq_UpdateAll` (0x2728f8) and `ScriptEngine_LaunchPendingProcess`
/// (0x282908).
pub(crate) const SE_IN_SEQ_UPDATE_ALL: Callee = Callee {
    addr: 0x0027_28f8,
    f: ee::s07::f_002728f8,
};

pub(crate) const SE_LAUNCH_PENDING: Callee = Callee {
    addr: 0x0028_2908,
    f: ee::s07::f_00282908,
};

/// `void ScriptEngine_UpdateProcesses(ScriptEngine* self)` (0x2820b0), every
/// frame: update all running sequences, then launch each pending process
/// slot (state 1) of type 1 (`LaunchPendingProcess(self, slot)`; the other
/// types are no-ops). The type switch goes through the original's jump
/// table: its targets are the launch (0x282120), the skip (0x282128) and,
/// as the generated code allows, a restart at the top; anything else leaves
/// through `jump_out`. Frame 0x30 (s0 = slot index, s1 = self). Other
/// entries (resume points) go to the generated `__entries` twin.
pub fn script_engine_update_processes(c: &mut Cpu) {
    guest(c, |c| {
        'restart: loop {
            c.cycles += 10;
            let frame = Frame::open(c, 0x30);
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S1][0] = c.g[A0][0];
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(SE_SEQ_PLAYER)).lw(c);
            SE_IN_SEQ_UPDATE_ALL.call(c, 0x0028_20d0)?;
            c.g[V0][0] = SE_SLOT_SIZE;
            loop {
                c.cycles += 6;
                c.mult(0, V0, S0, V0);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(SE_SLOTS));
                let slot = r32(c, S1).wrapping_add(r32(c, V0));
                c.g[A1][0] = sx(slot);
                let state = Addr(slot.wrapping_add(SE_SLOT_STATE)).lw(c);
                c.g[V1][0] = state;
                c.g[V0][0] = 1;
                // Whether the slot ends at the shared `s0++` (1 cycle).
                let mut next = true;
                if state != 0 {
                    c.cycles += 2;
                    if state != 1 {
                        next = false;
                    } else {
                        c.cycles += 4;
                        let ty = Addr(slot.wrapping_add(SE_SLOT_TYPE)).lw(c);
                        c.g[V1][0] = ty;
                        let known = ty < 6;
                        c.g[V0][0] = known as u64;
                        c.g[V0][0] = 0x0048_0000;
                        if known {
                            c.cycles += 6;
                            c.g[V1][0] = sx((ty as u32) << 2);
                            c.g[V0][0] = sx(SE_TYPE_TABLE);
                            let entry = SE_TYPE_TABLE.wrapping_add(r32(c, V1));
                            c.g[V1][0] = sx(entry);
                            let target = Addr(entry).lw(c);
                            c.g[A0][0] = target;
                            match target as u32 {
                                0x0028_20b0 => {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0028_20b0);
                                    }
                                    continue 'restart;
                                }
                                0x0028_2120 => {
                                    c.cycles += 2;
                                    c.g[A0][0] = c.g[S1][0];
                                    SE_LAUNCH_PENDING.call(c, 0x0028_2128)?;
                                }
                                0x0028_2128 => {}
                                t => {
                                    c.jump_out(t, 0x0028_20b0, 0x0028_214c);
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                if next {
                    c.cycles += 1;
                }
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.cycles += 3;
                let more = (c.g[S0][0] as i64) < 16;
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V0][0] = SE_SLOT_SIZE;
                if event_due(c) {
                    deliver_event_here(c, 0x0028_20d8);
                }
            }
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::slot as slot_2;

    pub(crate) use crate::game::ai::rider::tests::fb;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_7;

    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_9;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::bx::bxstring2::tests::game;

    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_6;

    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals as game_globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_8;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_5;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::input::inputparse::tests::EMPTY as EMPTY_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_7;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::object::effectlink::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_6;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn in_seq_get_instance_matches() {
        check_lohi(
            ssx3_ee::s07::f_00272cc0,
            in_seq_get_instance,
            200,
            &|c, seed| {
                let mut r = Rng(0x72cc0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = seed % 4;
                c.g[A1][0] = index as u64 | ((r.next_u32() as u64) << 32);
                let table = DATA + 0x1000;
                c.bus.write32(DATA + 0x14, table);
                let record = table + index * 0x58;
                c.bus.write32(
                    record + 0xc,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn script_engine_register_script_matches() {
        check(
            ssx3_ee::s07::f_00282150,
            script_engine_register_script,
            200,
            &|c, seed| {
                let mut r = Rng(0x2821_50 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let index = (seed % 4) as u64;
                c.g[A2][0] = index;
                let table = DATA + 0x800;
                c.bus.write32(DATA + 0x2b0, table);
                let slot = table + index as u32 * 8;
                c.bus
                    .write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn script_engine_unregister_script_matches() {
        check(
            ssx3_ee::s07::f_00282178,
            script_engine_unregister_script,
            200,
            &|c, seed| {
                let mut r = Rng(0x2821_78 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (seed % 4) as u64;
                c.g[A1][0] = index;
                let table = DATA + 0x800;
                c.bus.write32(DATA + 0x2b0, table);
                let slot = table + index as u32 * 8;
                c.bus
                    .write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn script_engine_clear_process_matches() {
        check(
            ssx3_ee::s07::f_00282c88,
            script_engine_clear_process,
            100,
            &|c, seed| {
                let mut r = Rng(0x282c_88 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..0x24 {
                    c.bus.write8(DATA + 0x100 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn script_engine_get_process_matches() {
        check_lohi(
            ssx3_ee::s07::f_00282cb0,
            script_engine_get_process,
            200,
            &|c, seed| {
                let mut r = Rng(0x282c_b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (seed % 4) as u64;
                c.g[A1][0] = index;
                let proc = DATA + 0x2c0 + index as u32 * 0x24;
                c.bus
                    .write32(proc + 0x20, if seed % 3 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn script_obj_ctor_matches() {
        check(
            ssx3_ee::s07::f_00283200,
            script_obj_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2832_00 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn script_obj_activate_matches() {
        check(
            ssx3_ee::s07::f_00283430,
            script_obj_activate,
            100,
            &|c, seed| {
                let mut r = Rng(0x2834_30 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let value = match seed % 4 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x8, value);
            },
        );
    }

    #[test]
    fn script_obj_deactivate_matches() {
        check(
            ssx3_ee::s07::f_00283440,
            script_obj_deactivate,
            100,
            &|c, seed| {
                let mut r = Rng(0x2834_40 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let value = match seed % 4 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x8, value);
            },
        );
    }

    #[test]
    fn script_obj_cam_has_binding_matches() {
        check(
            ee::s07::f_00282ef0__generated,
            script_obj_cam_has_binding,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0028_2ef0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let id = r.next_u32();
                c.bus.write32(DATA + 0x14, id);
                c.g[A1][0] = match seed % 3 {
                    0 => mixed(&mut r, seed, id),
                    1 => id as u64,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_destroy_matches() {
        check(
            ee::s07::f_00282b88__generated,
            script_engine_seq_listener_on_destroy,
            100,
            &|c, seed| {
                let mut r = game(c, seed, 0x282b88, 1);
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (SCRATCH + 0x2000) as u64;
                let ctx = ptr(&mut r);
                c.bus.write32(SCRATCH + 0x200c, ctx);
            },
        );
    }

    #[test]
    fn script_engine_get_process_in_seq_matches() {
        check(
            ssx3_ee::s07::f_00282bf0__generated,
            script_engine_get_process_in_seq,
            200,
            &|c, seed| {
                let mut r = rng_2(seed, 0x35);
                scramble(c, &mut r);
                let engine = DATA + 0x4000;
                let index = seed % 6;
                let proc = engine + 0x2c0 + index * 0x24;
                c.bus.write32(proc + 0x20, (seed % 2) as u32);
                c.bus.write32(proc, r.next_u32());
                c.g[A0][0] = sx(engine);
                c.g[A1][0] = index as u64;
            },
        );
    }

    #[test]
    fn script_binding_clear_listener_matches() {
        check(
            ssx3_ee::s07::f_00282f90__generated,
            script_binding_clear_listener,
            150,
            &|c, seed| {
                let mut r = rng_2(seed, 0x36);
                scramble(c, &mut r);
                let binding = DATA + 0x4000;
                let channel = DATA + 0x5000;
                c.bus
                    .write32(binding + 8, if seed % 2 == 0 { 0 } else { channel });
                // No listener: the channel just records the null listener.
                c.bus.write32(channel + 0x2c, 0);
                c.bus.write32(channel + 0x1c, DATA + 0x6000);
                c.bus.write8(DATA + 0x6003, r.next_u32() as u8);
                c.g[A0][0] = sx(binding);
            },
        );
    }

    pub(crate) fn binding_pause_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let binding = DATA + 0x4000;
        let channel = DATA + 0x5000;
        let inst = DATA + 0x6000;
        c.bus
            .write32(binding + 8, if seed % 2 == 0 { 0 } else { channel });
        c.bus.write32(channel + 8, inst);
        c.bus.write32(inst + 0x20, (seed % 3 != 0) as u32);
        c.bus.write32(
            inst + 0x2c,
            match seed % 4 {
                0 => 0,
                1 => 1,
                _ => r.next_u32(),
            },
        );
        c.g[A0][0] = sx(binding);
    }

    #[test]
    fn script_binding_pause_seq_matches() {
        check(
            ssx3_ee::s07::f_00283038__generated,
            script_binding_pause_seq,
            200,
            &|c, seed| binding_pause_setup(c, seed, 0x37),
        );
    }

    #[test]
    fn script_binding_resume_seq_matches() {
        check(
            ssx3_ee::s07::f_00283060__generated,
            script_binding_resume_seq,
            200,
            &|c, seed| binding_pause_setup(c, seed, 0x38),
        );
    }

    /// The four script-binding hooks (ScriptObjCam / cTransitionMan).
    pub(crate) fn script_binding_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
        });
    }

    #[test]
    fn c_transition_man_script_pause_matches() {
        script_binding_test(
            ssx3_ee::s07::f_00282f00__generated,
            c_transition_man_script_pause,
            0xcf,
        );
    }

    #[test]
    fn c_transition_man_script_resume_matches() {
        script_binding_test(
            ssx3_ee::s07::f_00282f30__generated,
            c_transition_man_script_resume,
            0xd0,
        );
    }

    #[test]
    fn script_binding_unbind_matches() {
        check(
            ssx3_ee::s07::f_00283000__generated,
            script_binding_unbind,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283000, 1);
                let binding = DATA + 0x100;
                c.bus.write32(GAME_GP + 0x3ec, DATA + 0x2000);
                fill(c, &mut r, binding, 0x20);
                c.bus.write32(binding + 4, r.next_u32() % 8);
                c.g[A0][0] = sx(binding);
            },
        );
    }

    #[test]
    fn script_cam_binding_ctor_matches() {
        check(
            ee::s07::f_00283088__generated,
            script_cam_binding_ctor,
            SEEDS_3,
            &|c, _seed| {
                let _ = machine(c, 0, 0x283088, 0);
                c.g[A0][0] = (DATA + 0x200) as u64;
            },
        );
    }

    #[test]
    fn script_obj_register_matches() {
        check(
            ee::s07::f_00283298__generated,
            script_obj_register,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283298, 0);
                let engine = DATA + 0x400;
                let vt = vtable(c, &mut r, &[0x10]);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(GAME_GP + 0x3ec, engine);
                c.bus.write32(engine + 0x2a8, vt);
            },
        );
    }

    #[test]
    fn script_engine_tick_process_matches() {
        check(
            ee::s07::f_00282450__generated,
            script_engine_tick_process,
            32,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2824_50, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 0;
                let slot = this + 0x2c0;
                c.bus
                    .write32(slot + 0x20, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    slot + 4,
                    match seed % 4 {
                        0 => 3,
                        1 => 4,
                        2 => 2,
                        _ => r.next_u32() & 0xff,
                    },
                );
                c.bus.write32(slot, DATA + 0x400);
            },
        );
    }

    #[test]
    fn script_engine_pause_process_matches() {
        check(
            ee::s07::f_002826d8__generated,
            script_engine_pause_process,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2826_d8, 1);
                let this = DATA + 0x100;
                let index = seed % 3;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(index);
                let slot = this + 0x2c0 + 0x24 * index;
                c.bus.write32(slot, DATA + 0x400);
                c.bus.write32(slot + 4, r.next_u32());
                c.bus
                    .write32(DATA + 0x400 + 0x2c, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_start_matches() {
        check(
            ee::s07::f_002829d0__generated,
            script_engine_seq_listener_on_start,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2829_d0, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                let state = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 0xc, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(state + 0xc, r.next_u32());
                c.bus.write32(this + 0x2a8, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 0x50, FAKE + 0x20);
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_create_matches() {
        check(
            ee::s07::f_00282b40__generated,
            script_engine_seq_listener_on_create,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x282b_40, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x200);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn script_binding_bind_matches() {
        check(
            ee::s07::f_00282fb8__generated,
            script_binding_bind,
            40,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x282f_b8, 1);
                let this = DATA + 0x100;
                let binding = DATA + 0x200;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(binding);
                c.bus.write32(binding + 8, DATA + 0x300);
                c.bus.write32(DATA + 0x300, r.next_u32());
            },
        );
    }

    #[test]
    fn script_engine_reset_process_table_matches() {
        check(
            ee::s07::f_002828b8__generated,
            script_engine_reset_process_table,
            80,
            &|c, seed| {
                let mut r = Rng(0x2828_b811 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 0x300);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn script_engine_alloc_process_matches() {
        check(
            ee::s07::f_00282c38__generated,
            script_engine_alloc_process,
            80,
            &|c, seed| {
                let mut r = Rng(0x282c_3811 ^ seed as u64);
                scramble(c, &mut r);
                let e = DATA;
                fill(c, &mut r, e + 0x2c0, 16 * 0x24 / 4);
                // Leave a few slots occupied so the search walks the table; make
                // slot 0 occupied for some seeds so the walk exhausts.
                let used = seed % 6;
                for i in 0..used {
                    c.bus.write32(e + 0x2e0 + 0x24 * i, 1 + i);
                }
                c.bus.write32(e + 0x300, 0);
                c.g[A0][0] = sx(e);
                c.g[A1][0] = match seed % 3 {
                    0 => 1u64,
                    1 => 0,
                    _ => (seed % 5) as u64,
                };
            },
        );
    }

    #[test]
    fn script_cam_binding_accept_command_matches() {
        check(
            ee::s07::f_002831b0__generated,
            script_cam_binding_accept_command,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2831);
                let b = DATA;
                let cmd = DATA + 0x400;
                c.bus.write32(b + 0x10, DATA + 0x800);
                c.bus.write32(DATA + 0x800 + 0xf8, r.next_u32());
                c.bus.write32(cmd + 8, r.next_u32());
                c.bus.write32(gp_at(-0x84c), DATA + 0x1000);
                fill(c, &mut r, DATA + 0x1000, 0x40);
                c.g[A0][0] = sx(b);
                c.g[A1][0] = sx(cmd);
                c.g[A2][0] = (seed % 6) as u64;
            },
        );
    }

    #[test]
    fn script_engine_stop_process_seq_matches() {
        check(
            ssx3_ee::s07::f_00282338__generated,
            script_engine_stop_process_seq,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x282338, 2);
                // `ScriptEngine_GetProcess` returns 0 for a zeroed engine, so the
                // process pointer is null and the sequence stop is skipped.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn script_engine_stop_all_process_seqs_matches() {
        check(
            ee::s07::f_00282390__generated,
            script_engine_stop_all_process_seqs,
            64,
            &|c, seed| {
                machine(c, seed, 0x0028_2390, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                for i in 0..16u32 {
                    let slot = DATA + 0x2c0 + 0x24 * i;
                    for k in 0..9u32 {
                        w(c, slot + 4 * k, 0);
                    }
                }
                if seed % 2 == 1 {
                    // One active sequence; its process id 0 resolves to the empty
                    // slot 0, so `StopProcessSeq` returns at once.
                    w(c, DATA + 0x2c0 + 0x24 * (seed % 16) as u32, 1);
                }
            },
        );
    }

    #[test]
    fn script_engine_kill_all_processes_matches() {
        check(
            ee::s07::f_002823f0__generated,
            script_engine_kill_all_processes,
            64,
            &|c, seed| {
                machine(c, seed, 0x0028_23f0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                for i in 0..16u32 {
                    let slot = DATA + 0x2c0 + 0x24 * i;
                    for k in 0..9u32 {
                        w(c, slot + 4 * k, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn script_engine_get_process_state_matches() {
        check(
            ee::s07::f_00282798__generated,
            script_engine_get_process_state,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0028_2798, 2);
                belib(c);
                let id = seed % 6;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = id as u64;
                let slot = DATA + 0x2c0 + 0x24 * id;
                if seed % 2 == 0 {
                    w(c, slot + 0x20, 0); // no such process
                } else {
                    w(c, slot + 0x20, 1);
                    w(c, slot + 4, 0); // already running: reports its state word
                }
            },
        );
    }

    #[test]
    fn c_transition_man_script_obj_ctor_matches() {
        check(
            ssx3_ee::s07::f_00282cd0__generated,
            c_transition_man_script_obj_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x282cd0, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_stop_matches() {
        check(
            ssx3_ee::s07::f_00282a18__generated,
            script_engine_seq_listener_on_stop,
            90,
            &|c, seed| {
                let mut r = setup(c, seed, 0x282a18);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let vt = vtable(c, &mut r, &[0x58]);
                c.bus.write32(DATA + 0x2a8, vt);
                c.bus.write32(DATA + 0x10c, DATA + 0x200);
                c.bus.write32(DATA + 0x208, r.next_u32());
                c.bus.write32(DATA + 0x20c, r.next_u32());
                c.bus
                    .write16(DATA + 0x210, if seed % 2 == 0 { 0 } else { 3 });
                c.bus.write32(DATA + 0x21c, r.next_u32());
            },
        );
    }

    #[test]
    fn script_obj_dtor_matches() {
        check(
            ee::s07::f_00283228__generated,
            script_obj_dtor,
            80,
            &|c, seed| {
                alloc_base(c, seed, 0x2832_28);
                let se = DATA + 0x2000;
                let vt = DATA + 0x2200;
                c.bus.write32(GAME_GP + 0x3ec, se);
                c.bus.write32(se + 0x2a8, vt);
                put_vtable(c, vt, &[0x18]);
                let obj = alloc(c, 0x30);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [0u64, 2, 1][seed as usize % 3];
                c.bus
                    .write32(obj + 4, [0u32, u32::MAX, 1, 5][seed as usize % 4]);
            },
        );
    }

    #[test]
    fn script_engine_remove_all_script_objs_matches() {
        check(
            ssx3_ee::s07::f_0027d050__generated,
            script_engine_remove_all_script_objs,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27d050, 0);
                let e = DATA + 0x1000;
                c.g[A0][0] = e as u64;
                let vt = DATA + 0x5000;
                w32(c, e + 0x2a8, vt);
                slot_2(c, vt, 0x18, 0, EMPTY_2);
                for i in 0..20u32 {
                    w32(
                        c,
                        e + 4 * i,
                        if (seed + i) % 4 == 0 { 0 } else { r.next_u32() },
                    );
                }
            },
        );
    }

    #[test]
    fn script_engine_resume_process_matches() {
        check(
            ssx3_ee::s07::f_00282720__generated,
            script_engine_resume_process,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x282720, 0);
                let e = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                c.g[A0][0] = e as u64;
                c.g[A1][0] = idx as u64;
                let slotp = e + idx * 0x24 + 0x2c0;
                let inst = DATA + 0x3000 + idx * 0x100;
                w32(c, slotp, inst);
                w32(
                    c,
                    slotp + 4,
                    if seed % 3 == 0 { 5 } else { (seed % 7) as u32 },
                );
                w32(
                    c,
                    inst + 0x20,
                    if seed % 2 == 0 { 0 } else { 3 + (seed % 4) },
                );
                w32(c, inst + 0x2c, if seed % 5 == 0 { 1 } else { 0 });
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn c_transition_man_script_advance_timer_matches() {
        check(
            ee::s07::f_00282d30__generated,
            c_transition_man_script_advance_timer,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_2d30, 2);
                c.f[12] = r.edge_f32();
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let (a, b_, d) = if seed % 2 == 0 {
                    (fb(1.0), fb(2.0), fb(4.0))
                } else {
                    (fb(100.0), fb(0.5), fb(0.25))
                };
                w(
                    c,
                    DATA + 0x2000 + 0x14,
                    if seed % 3 == 0 { 0xffff_ffff } else { 3 },
                );
                w(c, DATA + 0x2000 + 0x1c, a);
                w(c, DATA + 0x2000 + 0x20, fb(0.5));
                w(c, DATA + 0x2000 + 0x24, fb(0.25));
                w(c, DATA + 0x2000 + 0x2c, b_);
                let _ = d;
                c.pc = 0x0028_2d30;
            },
        );
    }

    // ----------------------------------------------------------------- scripts

    #[test]
    fn script_engine_is_process_finished_matches() {
        check(
            ee::s07::f_00282838__generated,
            script_engine_is_process_finished,
            16,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2828_38, 4);
                let self_ = DATA;
                let idx = (seed % 6) as u32;
                let proc = self_ + 0x2c0 + idx * 0x24;
                w(c, proc + 0x20, if seed % 5 == 0 { 0 } else { 1 });
                w(c, proc + 0x4, (seed % 8) as u32);
                w(c, proc, DATA + 0x1000);
                w(c, DATA + 0x1000 + 0x20, (seed % 5) as u32);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn scriptobj_pause_resume_matches() {
        for (gen, hand) in [
            (
                ee::s07::f_00283320__generated as fn(&mut Cpu),
                scriptobj_pause as fn(&mut Cpu),
            ),
            (ee::s07::f_002833a8__generated, scriptobj_resume),
        ] {
            check(gen, hand, SEEDS_7, &|c, seed| {
                base_4(c, seed, 0x83320, 0);
                let obj = DATA + 0x100;
                let binding = DATA + 0x200;
                c.g[A0][0] = sx(obj);
                c.bus.write32(obj + 0xc, binding);
                slot_3(c, binding, 0x30, 0, FAKE);
                slot_3(c, binding, 0x38, 0, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS, 1 + seed % 3);
            });
        }
    }

    // ------------------------------------------------------------------
    // Script / player / crowd.

    #[test]
    fn script_engine_run_process_matches() {
        check(
            ee::s07::f_002822a0__generated,
            script_engine_run_process,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x2822a0, 3);
                let eng = DATA + 0x1000;
                let id = (seed % 4) as u32;
                let proc = eng + 0x2c0 + 0x24 * id;
                c.g[A0][0] = eng as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                w(c, proc + 0x20, (seed / 4 % 2) as u32);
                w(c, proc + 0x4, (seed / 8 % 4) as u32);
                w(c, proc + 0x10, r.next_u32());
                w(c, proc, DATA + 0x2000);
                w(c, DATA + 0x2000, r.next_u32());
                w(c, eng + 0x2b8, DATA + 0x3000);
            },
        );
    }

    // ---------------------------------------------------------- script cam

    #[test]
    fn script_cam_binding_bind_matches() {
        check(
            ee::s07::f_002830c8__generated,
            script_cam_binding_bind,
            30,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2830, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                for i in 0..0x40u32 {
                    w(c, DATA + 0x800 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn script_engine_launch_pending_process_matches() {
        check_all(
            ee::s07::f_00282908__generated,
            script_engine_launch_pending_process,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2829, 3);
                game_globals_2(c);
                let this = DATA;
                let script = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = script as u64;
                let slots = DATA + 0x2400;
                c.bus.write32(this + 0x2b0, slots);
                c.bus.write32(this + 0x2b8, DATA + 0x2800);
                c.bus.write32(script + 0x14, seed % 4);
                if seed % 3 != 0 {
                    c.bus.write32(slots + 8 * (seed % 4), DATA + 0x2c00);
                }
                c.bus.write32(script + 0x18, r.f32(1.0));
            },
        );
    }

    /// A vtable at `at` whose slots are fake calls (all with delta 8).
    pub(crate) fn fake_table(c: &mut Cpu, at: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, 8u16);
            w(
                c,
                at + off + 4,
                crate::difftest::fakes::FAKE + 0x10 * i as u32,
            );
        }
    }

    #[test]
    fn script_engine_start_script_matches() {
        // Only the "slot has no sequence data" arm: the sequence arm needs a
        // live sequence manager behind the engine.
        strict(
            ee::s07::f_002821a0__generated,
            script_engine_start_script,
            SEEDS_9,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x21a0, 1);
                let engine = DATA + 0x1000;
                let slots = DATA + 0x4000;
                let callbacks = DATA + 0x5000;
                let key = (seed % 4) as u32;
                w(c, engine + 0x2b0, slots);
                w(c, engine + 0x2a8, callbacks);
                fake_table(c, callbacks, &[0x38]);
                w(c, slots + 8 * key, 0);
                c.bus.write16(slots + 8 * key + 4, r.next_u32() as u16);
                // the start callback's verdict: refuse on half the seeds
                w(
                    c,
                    crate::difftest::fakes::LOG_RESULTS,
                    if seed % 2 == 0 { 0 } else { DATA + 0x7000 },
                );
                c.g[A0][0] = sx(engine);
                c.g[A1][0] = key as u64;
                c.g[A2][0] = sx(DATA + 0x3000);
            },
        );
    }

    use crate::difftest::put_words;

    /// ScriptEngine_UpdateProcesses: the real InSeq_UpdateAll with no
    /// running sequences; sixteen slots that are free, running (state 2+),
    /// or pending with types 0..7 (the jump table copied from the ELF); the
    /// real LaunchPendingProcess for type 1 on its short path (no script
    /// registered at the slot's index).
    #[test]
    fn script_engine_update_processes_matches() {
        const ENGINE: u32 = DATA;
        const SEQ: u32 = DATA + 0x1000;
        const SCRIPTS: u32 = DATA + 0x2000;
        check(
            ee::s07::f_002820b0__generated,
            script_engine_update_processes,
            300,
            &|c, seed| {
                let mut r = Rng(0x28_20b0 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                c.pc = 0x0028_20b0;
                put_words(
                    c,
                    SE_TYPE_TABLE,
                    &[
                        0x28_2128, 0x28_2120, 0x28_2128, 0x28_2128, 0x28_2128, 0x28_2128,
                    ],
                );
                c.bus.write32(ENGINE + SE_SEQ_PLAYER, SEQ);
                c.bus.write32(SEQ + 0x1c, 0);
                c.bus.write32(ENGINE + 0x2b0, SCRIPTS);
                for i in 0..16 {
                    let slot = ENGINE + SE_SLOTS + 0x24 * i;
                    fill(c, &mut r, slot, 9);
                    let state = match r.next_u32() % 4 {
                        0 => 0,
                        1 => 2 + r.next_u32() % 3,
                        _ => 1,
                    };
                    c.bus.write32(slot + SE_SLOT_STATE, state);
                    let ty = if r.next_u32() % 2 == 0 {
                        1
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(slot + SE_SLOT_TYPE, ty);
                    c.bus.write32(slot + 0x14, r.next_u32() % 32);
                }
                for i in 0..32 {
                    c.bus.write32(SCRIPTS + 8 * i, 0);
                }
                c.g[A0][0] = sx(ENGINE);
            },
        );
    }
}
