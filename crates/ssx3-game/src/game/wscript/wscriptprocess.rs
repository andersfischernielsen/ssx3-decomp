//! `src/wscript/wscriptprocess.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::ovstates::at as at_2;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0030_7738, w_script_process_dtor, "WScriptProcess_dtor"),
    (0x0030_77f8, wscript_process_update, "WScriptProcess_Update"),
    (
        0x0030_79d8,
        wscript_process_get_current_task_id,
        "WScriptProcess_GetCurrentTaskId",
    ),
    (
        0x0030_7a10,
        wscript_process_forward_event_to_task,
        "WScriptProcess_ForwardEventToTask",
    ),
    (0x0030_7d30, w_script_process_v06, "WScriptProcess_v06"),
    (0x0030_7dc8, w_script_process_v02, "WScriptProcess_v02"),
    (
        0x0030_7e10,
        wscript_process_next_task,
        "WScriptProcess_NextTask",
    ),
    (
        0x0030_7ec0,
        w_script_process_jump_to_task,
        "WScriptProcess_JumpToTask",
    ),
    (
        0x0030_7f58,
        wscript_process_goto_task,
        "WScriptProcess_GotoTask",
    ),
    (
        0x0030_8038,
        w_script_process_save_state,
        "WScriptProcess_SaveState",
    ),
    (0x0030_8228, wscript_task_ctor, "WScriptTask_ctor"),
    (0x0030_8278, wscript_task_init, "WScriptTask_Init"),
    (0x0030_8328, wscript_task_update, "WScriptTask_Update"),
    (0x0030_86b0, wscript_task_start, "WScriptTask_Start"),
    (0x0030_8720, wscript_task_stop, "WScriptTask_Stop"),
    (
        0x0030_8760,
        wscript_task_save_state,
        "WScriptTask_SaveState",
    ),
    (
        0x0030_8798,
        wscript_task_load_state,
        "WScriptTask_LoadState",
    ),
    (0x0030_88d8, wscript_man_dtor, "WScriptMan_dtor"),
    (0x0030_8988, wscript_man_reset, "WScriptMan_Reset"),
    (
        0x0030_8a48,
        w_script_man_free_local_tables,
        "WScriptMan_FreeLocalTables",
    ),
    (
        0x0030_8aa0,
        wscript_man_create_local_tables,
        "WScriptMan_CreateLocalTables",
    ),
    (
        0x0030_8be0,
        w_script_man_spawn_processes,
        "WScriptMan_SpawnProcesses",
    ),
    (
        0x0030_8f38,
        w_script_man_delete_all_processes,
        "WScriptMan_DeleteAllProcesses",
    ),
    (
        0x0030_8f70,
        wscript_man_unload_section,
        "WScriptMan_UnloadSection",
    ),
    (
        0x0030_8fe0,
        wscript_man_unload_section_and_free,
        "WScriptMan_UnloadSectionAndFree",
    ),
    (0x0030_9030, w_script_man_restart, "WScriptMan_Restart"),
    (
        0x0030_9b70,
        wscript_man_lookup_func,
        "WScriptMan_LookupFunc",
    ),
    (
        0x0030_9118,
        wscript_man_process_pending_request,
        "WScriptMan_ProcessPendingRequest",
    ),
    (0x0030_9270, wscript_man_update, "WScriptMan_Update"),
];

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_LUNO_TABLE_DTOR;

    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;

    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;

    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;

    // Script engine (shard 10).
    pub const WSCRIPT_PROCESS_TERMINATE: Callee = Callee {
        addr: 0x0030_7d78,
        f: ee::s10::f_00307d78,
    };

    pub const WSCRIPT_MAN_CALL_PROCESS_FUNC: Callee = Callee {
        addr: 0x0030_ac98,
        f: ee::s10::f_0030ac98,
    };

    pub const WSCRIPT_MAN_IS_PROCESS_ACTIVE: Callee = Callee {
        addr: 0x0030_b898,
        f: ee::s10::f_0030b898,
    };
}

// ---------------------------------------------------------------------------
// WScript processes.

/// `int WScriptProcess_GetCurrentTaskId(WScriptProcess *this)` (0x3079d8):
/// the task id of the current instruction, or -1 when exhausted.
pub fn wscript_process_get_current_task_id(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x8).lw(c);
    c.g[V0][0] = Addr(this).off(0x4).lw(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V1][0] = 0x44;
    if in_range {
        c.cycles += 5;
        c.g[V0][0] = Addr(this).off(0x14).lw(c);
        c.mult(0, V1, A1, V1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script and rider helpers.

/// `void WScriptMan_LookupFunc(WScriptMan* man, int id)` (0x309b70): resolve
/// a script function id through the man's cache, or return 0.
pub fn wscript_man_lookup_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let id = c.g[A1][0];
        c.g[V0][0] = !(0u64 | id);
        let wanted = c.g[V0][0] != 0;
        c.sw(frame.at(c, 0), c.g[A1][0]);
        if wanted {
            c.cycles += 3;
            c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x28c).lw(c);
            c.g[A1][0] = sx(frame.at(c, 0));
            call_ptr(c, callee::WORLD_GET_SECTION_SCRIPT_FUNC, 0x0030_9b98)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptTask_SaveState(WScriptTask* task, sWScriptStream* stream)`
/// (0x308760).
pub fn wscript_task_save_state(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0030_8788, 8, 4, None))
}

/// `void WScriptTask_LoadState(WScriptTask* task, sWScriptStream* stream)`
/// (0x308798).
pub fn wscript_task_load_state(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0030_87c0, 0x10, 4, None))
}

// ---------------------------------------------------------------------------
// WScriptMan_DeleteAllProcesses.

/// `void WScriptMan_DeleteAllProcesses(this)` (0x308f38): empty the three
/// script process lists at `+0x2b8`, `+0x2bc` and `+0x2b4`.
pub fn w_script_man_delete_all_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        for (off, ret) in [
            (0x2b8u32, 0x0030_8f50u32),
            (0x2bc, 0x0030_8f58),
            (0x2b4, 0x0030_8f60),
        ] {
            c.g[A0][0] = sx((this as u32).wrapping_add(off));
            callee::WSCRIPT_DELETE_ALL.call(c, ret)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScript tasks.

/// `void WScriptTask_Start(WScriptTask* self)` (0x3086b0): set the task state
/// to 1 and tell the script manager (through `self+0x3c`) to start the task.
pub fn wscript_task_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 1;
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).lw(c);
        let already = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 3;
            c.g[A2][0] = Addr(self_at).off(8).lw(c);
            let no_script = c.g[A2][0] == 0;
            c.sw(self_at, c.g[V1][0]);
            if !no_script {
                c.cycles += 5;
                c.g[V0][0] = Addr(self_at).off(0x3c).lw(c);
                c.g[A1][0] = c.g[A0][0];
                c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lw(c);
                callee::WSCRIPT_MAN_CALL_TASK_FUNC.call(c, 0x0030_86e4)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void WScriptTask_Stop(WScriptTask* self)` (0x308720): set the task state
/// to 3 and tell the script manager to stop the task at `+0x10`.
pub fn wscript_task_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).lw(c);
        let already = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 3;
            c.g[A2][0] = Addr(self_at).off(0x10).lw(c);
            let no_script = c.g[A2][0] == 0;
            c.sw(self_at, c.g[V1][0]);
            if !no_script {
                c.cycles += 5;
                c.g[V0][0] = Addr(self_at).off(0x3c).lw(c);
                c.g[A1][0] = c.g[A0][0];
                c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lw(c);
                callee::WSCRIPT_MAN_CALL_TASK_FUNC.call(c, 0x0030_8754)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script processes.

/// `void WScriptProcess_FinishProcess(self)` (0x307d30): mark the process
/// finished (`*self = 3`), hand its script to the script manager if it has
/// one (`+0x30`), then terminate it.
pub fn w_script_process_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        Addr(this).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(this).off(0x30).lw(c);
        if c.g[A2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this).off(0x10).lw(c);
            c.g[A1][0] = c.g[S0][0];
            WSCRIPT_MAN_CALL_PROCESS_FUNC.call(c, 0x0030_7d60)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        WSCRIPT_PROCESS_TERMINATE.call(c, 0x0030_7d68)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcess_Deactivate(self)` (0x307dc8): if the process is still
/// in the manager's active list, call its `deactivate` slot
/// (vtable 0x30/0x34).
pub fn w_script_process_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(this).off(0x10).lw(c);
        WSCRIPT_MAN_IS_PROCESS_ACTIVE.call(c, 0x0030_7de4)?;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(this).off(0x5c).lw(c);
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(vt).off(0x30).lh(c);
            c.g[V1][0] = Addr(vt).off(0x34).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0030_7e00);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_7e00)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptTask* WScriptTask_ctor(WScriptTask* self)` (0x308228): install the
/// task vtable, clear the 12-byte name and reset the counters.
pub fn wscript_task_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0048_9ae0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(this.wrapping_add(8));
        Addr(this).off(0x40).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xc;
        call_ptr(c, 0x003e_6448, 0x0030_8254)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x3c).sw(c, 0);
        Addr(this).off(4).sw(c, 0);
        Addr(this).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_UnloadSectionAndFree(WScriptMan* self, int section)`
/// (0x308fe0): unload the section and release its cache entry.
pub fn wscript_man_unload_section_and_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::WSCRIPT_MAN_UNLOAD_SECTION.call(c, 0x0030_8ffc)?;
        c.g[A0][0] = Addr(r32(c, S1)).off(0x28c).lw(c);
        c.g[V0][0] = 0xc;
        c.mult(0, S0, S0, V0);
        c.g[V1][0] = Addr(r32(c, A0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(4).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
        call_ptr(c, 0x003a_cc90, 0x0030_9018)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void WScriptProcess_ForwardEventToTask(WScriptProcList* self, int event)`
/// (0x307a10): forward an event to the task object of the process list's
/// current entry (entries are 0x44-byte records, their vtable slot at +0x40).
pub fn wscript_process_forward_event_to_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(p(this, 8)).lw(c);
        c.g[V0][0] = Addr(p(this, 4)).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 10;
            c.g[A0][0] = 0x44;
            c.g[V1][0] = Addr(p(this, 0x14)).lw(c);
            c.mult(0, A0, A2, A0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = Addr(r32(c, A0)).off(0x40).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x18).lh(c);
            c.g[A2][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
            let target = r32(c, A2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            call_ptr(c, target, 0x0030_7a54)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = r32(c, RA);
        Ok(())
    })
}

/// `void WScriptMan_FreeLocalTables(self)` (0x308a48): destroy and clear the
/// 50 local object tables.
pub fn w_script_man_free_local_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x3cc));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x31;
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw(r32(c, S0));
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[A1][0] = 3;
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0030_8a7c);
                call_ptr(c, callee::C_LUNO_TABLE_DTOR, 0x0030_8a7c)?;
                Addr(r32(c, S0)).sw(c, 0);
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_8a60);
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

/// `void WScriptMan_UnloadSection(WScriptMan* m, int section)` (0x308f70):
/// drop every process bound to the section from its three lists and free the
/// section's object slot.
pub fn wscript_man_unload_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
        call::call_ptr(c, 0x0030_b0e8, 0x0030_8f90)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b8));
        c.g[A1][0] = c.g[S0][0];
        call::call_ptr(c, 0x0030_b0e8, 0x0030_8f9c)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2bc));
        c.g[A1][0] = c.g[S0][0];
        call::call_ptr(c, 0x0030_b0e8, 0x0030_8fa8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3cc));
        c.g[S1][0] = abi::addu(c.g[S1][0], c.g[S0][0]);
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        lqsp(c, 16, 0x20);
        if !empty {
            c.cycles += 4;
            c.g[A1][0] = 3;
            call::call_ptr(c, 0x0022_4df0, 0x0030_8fc8)?;
            Addr(c.g[S1][0] as u32).sw(c, 0);
            lqsp(c, 16, 0x20);
        }
        c.cycles += 4;
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void WScriptTask_Update(self)` (0x308328): a two-state task step — state
/// 1 arms state 2 and runs the vtable action, state 2 just runs it.
pub fn wscript_task_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A1][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[V0][0] = 0x1;
        c.g[V1][0] = Addr(low(c, A1)).lw(c);
        let one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if !one {
            let below = c.g[V0][0] != 0;
            c.cycles += 2;
            if below {
                c.g[RA][0] = Addr(sp_2(c)).ld(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = lw_r(c, A1, 0x40);
                    c.cycles += 5;
                    c.g[A0][0] = lh_r(c, V0, 0x28);
                    c.g[V1][0] = lw_r(c, V0, 0x2c);
                    let target = low(c, V1);
                    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0030_8394)?;
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                }
            }
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = lw_r(c, A1, 0x40);
            sw_r(c, A1, 0, c.g[V0][0]);
            c.g[A0][0] = lh_r(c, V1, 0x28);
            c.g[V0][0] = lw_r(c, V1, 0x2c);
            let target = low(c, V0);
            c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0030_837c)?;
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
        }
        c.cycles += 2;
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void WScriptMan_SpawnProcesses(self, section)` (0x00308be0): spawn a
/// script process for every entry in the section.
pub fn w_script_man_spawn_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(c, S3, 0x28c)).lw(c);
        callee::WSCRIPT_CACHE_GET_PROCESS_COUNT.call(c, 0x0030_8c08)?;
        c.g[A0][0] = Addr(at(c, S3, 0x28c)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        callee::WSCRIPT_CACHE_GET_PROCESSES.call(c, 0x0030_8c18)?;
        c.g[S1][0] = c.g[V0][0];
        if (c.g[S0][0] as i64) > 0 {
            loop {
                c.cycles += 8;
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A3][0] = 0;
                callee::WSCRIPT_MAN_CREATE_PROCESS.call(c, 0x0030_8c34)?;
                c.g[S0][0] = sx(word(c, S0).wrapping_sub(1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(0x40));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_8c20);
                }
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

/// `WScriptProcess_JumpToTask` (0x307ec0): find the task with the given id in
/// the process's 0x44-byte task table, mark it jumped-to and call its start
/// handler.
pub fn w_script_process_jump_to_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = r(c, A0);
        c.g[A2][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr(p(this, 4)).lw(c);
        let none = (c.g[A3][0] as i64) <= 0;
        c.g[T0][0] = 0;
        let mut called = false;
        if none {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[T2][0] = 0x44;
            c.g[T3][0] = 3;
            c.g[T1][0] = 0;
            loop {
                c.cycles += 5;
                c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
                c.g[V0][0] = sx(r(c, T1).wrapping_add(r(c, A0)));
                c.g[V1][0] = Addr(p(r(c, V0), 4)).lw(c);
                if c.g[V1][0] != c.g[A1][0] {
                    c.g[A3][0] = Addr(p(this, 4)).lw(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(p(this, 8)).lw(c);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let in_range = c.g[V0][0] == 0;
                    c.mult(0, 5, 3, 10);
                    if !in_range {
                        c.cycles += 3;
                        c.g[V0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                        Addr(r(c, V0)).sw(c, c.g[T3][0]);
                        c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
                    }
                    c.cycles += 9;
                    Addr(p(this, 8)).sw(c, c.g[T0][0]);
                    c.g[A0][0] = sx(r(c, T1).wrapping_add(r(c, A0)));
                    c.g[V0][0] = Addr(p(r(c, A0), 0x40)).lw(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0x20)).lh(c);
                    c.g[A1][0] = Addr(p(r(c, V0), 0x24)).lw(c);
                    let target = r(c, A1);
                    c.g[RA][0] = sx(0x0030_7f34);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
                    call_ptr(c, target, 0x0030_7f34)?;
                    frame.restore_ra(c, 0);
                    called = true;
                    break;
                }
                c.cycles += 4;
                c.g[T0][0] = sx(r(c, T0).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T1][0] = sx(r(c, T1).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_7ee8);
                }
            }
            if !called {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_dtor(self, int flags)` (0x3088d8): free the script
/// manager's local tables and process lists, then optionally the object.
pub fn wscript_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x490000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6400));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(this as u32, 0x494), c.g[V0][0]);
        call_ptr(c, 0x0030_8a48, 0x0030_8900)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c0));
        let no_list = c.g[V1][0] == 0;
        c.sw(p(r32(c, GP), 0xce8), 0);
        let mut skip_one = false;
        if !no_list {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3cc));
            let single = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
            if single {
                skip_one = true;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10c));
                loop {
                    c.cycles += 7;
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10c));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_8920);
                    }
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
        c.cycles += 11;
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0030_b068, 0x0030_8948)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0030_b068, 0x0030_8954)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0030_b068, 0x0030_8960)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0030_8974)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcess_NextTask(self)` (0x307e10): advance the script
/// process to its next instruction, calling the current and next elements'
/// handlers; a finished process (state 3) is left alone.
pub fn wscript_process_next_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V1][0] = 0x3;
        c.g[V0][0] = Addr(this as u32).lw(c);
        let finished = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = 0x44;
        let mut skip_one = false;
        if finished {
        } else {
            c.cycles += 15;
            c.g[A0][0] = Addr(p(this as u32, 8)).lw(c);
            c.g[V1][0] = Addr(p(this as u32, 0x14)).lw(c);
            c.mult(0, 4, 4, 17);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x40)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x34)).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_7e58)?;
            c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 4)).lw(c);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let at_end = c.g[V0][0] == 0;
            c.mult(0, 3, 5, 17);
            if at_end {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_7eac)?;
            } else {
                c.cycles += 10;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x14)).lw(c);
                c.sw(p(c.g[S0][0] as u32, 8), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x40)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x20)).lh(c);
                c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x24)).lw(c);
                let target = c.g[A1][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, target, 0x0030_7e90)?;
                skip_one = true;
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptTask_Init(self, owner, def)` (0x308278): look the task's
/// script functions up (three in the first table, ten in the second) and
/// store the process-relative pointers.
pub fn wscript_task_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S1][0] = sx((this as u32).wrapping_add(8));
        c.g[S3][0] = 0x2;
        c.g[S4][0] = c.g[A2][0];
        c.sw(p(this as u32, 0x3c), c.g[A1][0]);
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
        c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
        c.sw(p(this as u32, 4), c.g[V0][0]);
        loop {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0x3c)).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0030_8018, 0x0030_82cc)?;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_82b8);
            }
        }
        c.cycles += 4;
        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[S3][0] = 0x9;
        loop {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0x3c)).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0030_8018, 0x0030_82fc)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_82e8);
            }
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

/// `void WScriptMan_CreateLocalTables(self)` (0x308aa0): recreate the script
/// manager's local tables from the current process list.
pub fn wscript_man_create_local_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[S0][0] = 0;
        call_ptr(c, 0x0030_8a48, 0x0030_8ac8)?;
        c.g[V0][0] = Addr(p(this as u32, 0x28c)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x003a_9800, 0x0030_8ad8)?;
        c.g[S2][0] = c.g[V0][0];
        let none = (c.g[S2][0] as i64) <= 0;
        c.g[S4][0] = 0x1;
        if !none {
            c.cycles += 1;
            c.g[S3][0] = 0;
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x28c)).lw(c);
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 4)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let skip = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !skip {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 8)).lw(c);
                    c.g[A1][0] = c.g[S4][0];
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    }
                }
                c.cycles += 2;
                let have = c.g[A1][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if have {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_8b78, 0x0030_8b24)?;
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_8ae8);
                }
            }
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

// ---------------------------------------------------------------------------
// Script process.

/// `void WScriptProcess_GotoTask(WScriptProcess* self, int id)` (0x307f58):
/// find the task whose id (+4) matches, retarget the process state word (+8)
/// and restart it (virtual `Start` at_2 `task+0x40 + 0x20/0x24`); when the task
/// is already current, ask it to resume instead.
pub fn wscript_process_goto_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x40);
        c.sq(16, at_2(c.g[SP][0] as u32, 0x30));
        c.sq(17, at_2(c.g[SP][0] as u32, 0x20));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sq(18, at_2(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = Addr(this as u32).off(4).lw(c);
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[A3][0] = 0x44;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
                if c.g[V1][0] != c.g[A1][0] {
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let resume = c.g[V0][0] != 0;
                    c.mult(0, 2, 3, 7);
                    if resume {
                        c.cycles += 6;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x40).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x30).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x34).lw(c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0030_7fc4)?;
                    }
                    c.cycles += 10;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
                    Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x40).lw(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
                    let target = c.g[A1][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, target, 0x0030_7fe4)?;
                    c.lq(16, at_2(c.g[SP][0] as u32, 0x30));
                    c.cycles += 5;
                    c.lq(17, at_2(c.g[SP][0] as u32, 0x20));
                    c.lq(18, at_2(c.g[SP][0] as u32, 0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    close(c, 0x40);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_7f88);
                }
            }
        }
        c.cycles += 1;
        c.lq(16, at_2(c.g[SP][0] as u32, 0x30));
        c.cycles += 5;
        c.lq(17, at_2(c.g[SP][0] as u32, 0x20));
        c.lq(18, at_2(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x40);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script manager.

/// `void WScriptMan_Reset(WScriptMan* self)` (0x308988): clear the whole
/// script-manager state (-1 / 0xFFFFFFFF sentinels included), the command
/// queue, the local tables and the 0x180-byte block at_2 +0x44.
pub fn wscript_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sq(16, at_2(c.g[SP][0] as u32, 0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V1][0] = u64::MAX;
        Addr(this as u32).off(0x290).sw(c, 0);
        Addr(this as u32).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(this as u32).off(0x294).sw(c, 0);
        c.g[A2][0] = 8;
        Addr(this as u32).off(0x298).sw(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x38));
        Addr(this as u32).off(0x2a4).sw(c, 0);
        Addr(this as u32).off(0x2a8).sw(c, 0);
        Addr(this as u32).off(0x29c).sw(c, 0);
        Addr(this as u32).off(0x2ac).sw(c, 0);
        for off in [0x4u32, 0x8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x28, 0x30, 0x34] {
            Addr(this as u32).off(off).sw(c, 0);
        }
        Addr(this as u32).off(0x24).sw(c, c.g[V1][0]);
        Addr(this as u32).off(0x2c).sw(c, c.g[V0][0]);
        Addr(this as u32).off(0x2a0).sw(c, c.g[V0][0]);
        Addr(this as u32).off(0x2b0).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0030_8a10);
        Addr(this as u32).off(0xc).sw(c, c.g[V1][0]);
        call_ptr(c, 0x003e_6448, 0x0030_8a10)?;
        c.g[RA][0] = sx(0x0030_8a18);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1c4));
        call_ptr(c, 0x0030_c760, 0x0030_8a18)?;
        Addr(this as u32).off(0x40).sw(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x44));
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0030_8a2c);
        c.g[A2][0] = 0x180;
        call_ptr(c, 0x003e_6448, 0x0030_8a2c)?;
        c.g[RA][0] = sx(0x0030_8a34);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0030_8aa0, 0x0030_8a34)?;
        c.lq(16, at_2(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `WScriptProcess_dtor` (0x00307738).
//
// `void WScriptProcess::~WScriptProcess(WScriptProcess* self, int flags)`:
// destroy the element array, release the script object and optionally delete
// the process.

pub fn w_script_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_3077b8: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = 0x490000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6490));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            let t = c.g[A2][0] == 0u64;
            c.g[S2][0] = c.g[A1][0];
            if t {
                break 'b_3077b8;
            } else {
                'b_3077b0: {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = 0x44;
                    c.mult(0, 4, 3, 2);
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    if c.g[A2][0] == c.g[S0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        break 'b_3077b0;
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                        'l_307788: loop {
                            c.cycles += 9;
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x003077a0);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x003077a0)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            if c.g[V0][0] != c.g[S0][0] {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                                if crate::events::event_due(c) {
                                    crate::events::deliver_event_here(c, 0x00307788);
                                }
                                continue 'l_307788;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                break 'b_3077b0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x003077b8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x00317e98, 0x003077b8)?;
                break 'b_3077b8;
            }
        }
        'b_3077d0: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if t {
                break 'b_3077d0;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003077cc);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x00224df0, 0x003077cc)?;
                c.g[V0][0] = c.g[S2][0] & 0x1;
                break 'b_3077d0;
            }
        }
        'b_3077e4: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_3077e4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003077e0);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00317e50, 0x003077e0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_3077e4;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `WScriptProcess_v04` (0x00307c40).
//
// `void WScriptProcess::v04(WScriptProcess* self)`: (re)initialise the
// process's element storage and run its entry function.

pub fn w_script_process_v04(c: &mut Cpu) {
    guest(c, |c| {
        'b_307c68: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            if c.g[A0][0] == 0u64 {
                c.g[A1][0] = 0x490000;
                break 'b_307c68;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00307c64);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x00224df0, 0x00307c64)?;
                c.g[A1][0] = 0x490000;
                break 'b_307c68;
            }
        }
        'b_307cac: {
            c.cycles += 14;
            c.g[A2][0] = 0x20000000;
            c.g[A0][0] = 0x4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6778));
            c.g[RA][0] = sx(0x00307c7c);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x00307c7c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00307c88);
            c.g[A1][0] = 0x8;
            call_ptr(c, 0x00224da0, 0x00307c88)?;
            c.g[V1][0] = 0x1;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            let t = c.g[A2][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
            if t {
                break 'b_307cac;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x00307cac);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0030ac98, 0x00307cac)?;
                break 'b_307cac;
            }
        }
        'b_307cf4: {
            c.cycles += 8;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x00307cb8);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x00309a60, 0x00307cb8)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_307cf4;
            } else {
                c.cycles += 10;
                c.g[A0][0] = 0x44;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.mult(0, 4, 5, 4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                let t = c.g[A1][0] as u32;
                c.g[RA][0] = sx(0x00307cf0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, t, 0x00307cf0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_307cf4;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `WScriptProcess_Update(this)` (0x3077f8): the scripted process's update —
/// start it on the first tick, advance its current element and, while the
/// element did not change the state, run the process's own step hook.
pub fn wscript_process_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);

        let state = Addr(this).lw(c);
        c.g[V1][0] = state;
        let is_one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if !is_one {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x20);
                c.cycles += 4;
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                frame.restore(c, 16, 0x20);
                c.cycles += 4;
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        } else {
            // state == 1: move to state 2 and fall through.
            c.cycles += 4;
            c.g[V0][0] = 2;
            Addr(this).sw(c, c.g[V0][0]);
        }

        // state == 2
        c.g[S1][0] = Addr(this).lw(c);
        c.g[V0][0] = 2;
        c.cycles += 2;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, 16, 0x20);
            c.cycles += 4;
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        let _ = state;
        c.cycles += 5;
        c.g[A1][0] = Addr(this).off(8).lw(c);
        c.g[V0][0] = Addr(this).off(4).lw(c);
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this).lw(c);
        } else {
            c.cycles += 10;
            c.g[A0][0] = 0x44;
            c.g[V1][0] = Addr(this).off(0x14).lw(c);
            c.mult(0, 4, 5, 4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x40).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
            c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_7888)?;
            c.g[V0][0] = Addr(this).lw(c);
        }

        c.cycles += 2;
        if c.g[V0][0] != c.g[S1][0] {
            frame.restore(c, 16, 0x20);
            c.cycles += 4;
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 6;
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x2c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_78a8)?;

        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptProcess_SaveState (0x0000308038).

/// `WScriptProcess_SaveState`: walk the process list and save each running script's state.
pub fn w_script_process_save_state(c: &mut Cpu) {
    guest(c, |c| {
        'b_3080c8: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[A2][0] = 0x10;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[RA][0] = sx(0x00308074);
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x00308074 {
                return Err(crate::call::Unwound);
            }
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x003aca38, 0x00308084)?;
            c.g[A3][0] = c.lw(c.g[S3][0] as u32);
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0x4;
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[RA][0] = sx(0x003080b0);
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x003080b0 {
                return Err(crate::call::Unwound);
            }
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[A2][0] = c.g[S0][0];
            if t {
                break 'b_3080c8;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x002257e0, 0x003080c8)?;
                break 'b_3080c8;
            }
        }
        'b_3080fc: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_3080fc;
            } else {
                c.cycles += 1;
                c.g[S1][0] = 0u64;
                'l_3080d8: loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    call_ptr(c, 0x00308760, 0x003080ec)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003080d8);
                        }
                        continue 'l_3080d8;
                    } else {
                        break 'b_3080fc;
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_Restart (0x0000309030).

/// `WScriptMan_Restart`: restart the world script manager: reset the script slots and load the startup scripts.
pub fn w_script_man_restart(c: &mut Cpu) {
    guest(c, |c| {
        'b_3090f4: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = 0u64;
            call_ptr(c, 0x0030b1b8, 0x0030905c)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
            call_ptr(c, 0x0030b1b8, 0x00309064)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
            call_ptr(c, 0x0030b1b8, 0x0030906c)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00308988, 0x00309074)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28c));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x003a9800, 0x00309084)?;
            c.g[S2][0] = c.g[V0][0];
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S4][0] = 0x1;
            if t {
                break 'b_3090f4;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0u64;
                'l_309098: loop {
                    'b_3090c4: {
                        c.cycles += 8;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28c));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.lh(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_3090c4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = c.g[S4][0];
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = 0u64;
                            }
                            break 'b_3090c4;
                        }
                    }
                    'b_3090e8: {
                        'b_3090dc: {
                            c.cycles += 2;
                            let t = c.g[A1][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_3090dc;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x00308be0, 0x003090d4)?;
                                let t = true;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_3090e8;
                                } else {
                                    break 'b_3090dc;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x00308f70, 0x003090e4)?;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_3090e8;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00309098);
                        }
                        continue 'l_309098;
                    } else {
                        break 'b_3090f4;
                    }
                }
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

pub(crate) mod wscript_pending {
    use super::*;

    /// The pending request kind (1..6; 0: none).
    pub const REQUEST: u32 = 0x2ac;

    /// The request's mission id (-1: none).
    pub const MISSION: u32 = 0x2b0;

    /// The id passed to the audio system's mission end.
    pub const AUDIO_ID: u32 = 0x2a0;

    /// The request jump table (`0x4898f0`, six entries).
    pub const JUMP_TABLE: u32 = 0x0048_98f0;

    pub const RUN_SETUP_SCRIPTS: Callee = Callee {
        addr: 0x0030_8db8,
        f: ee::s10::WScriptMan_RunMissionSetupScripts,
    };

    pub const MISSION_COMMAND: Callee = Callee {
        addr: 0x0030_a868,
        f: ee::s10::WScriptMan_MissionCommand,
    };

    /// `SSXAudioSystem* SSXAudioSystem_GetInstance()`.
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    /// `SSXAudioSystem_OnMissionEnd(audio, id)`.
    pub const AUDIO_ON_MISSION_END: u32 = 0x0029_d6e0;

    /// `SSXAudioSystem` FE confirmation sound (`audio, 1`).
    pub const AUDIO_FE_SOUND: u32 = 0x0029_dbb0;

    /// Replay hook on the world's replay object (`world->game->0x28`).
    pub const REPLAY_ON_MISSION: u32 = 0x0026_f7b8;

    /// Playback manager hook (`world->game->0x34`).
    pub const PLAYBACK_ON_MISSION: u32 = 0x0026_cc48;
}

/// `void WScriptMan_ProcessPendingRequest(WScriptMan* self)` (0x309118):
/// carry out the mission request queued in `self+0x2ac` (1: start — run the
/// setup scripts, command 1, end the audio mission, poke the replay; 2:
/// command 3; 3 and 4: command 0x12 / 5, rerun the setup scripts, play the
/// FE sound; 5: poke the playback manager, command 1 (forced), rerun the
/// setup scripts; 6: command 0x12 and rerun the setup scripts), then clear
/// the request (kind 0, mission -1). The kind dispatches through the
/// compiled jump table at 0x4898f0.
pub fn wscript_man_process_pending_request(c: &mut Cpu) {
    use wscript_pending::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = c.lw(this + REQUEST);
        c.g[A0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V1][0] = (c.g[A0][0] < 6) as u64;
        c.g[V0][0] = 0x0049_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(JUMP_TABLE));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            // Mission command `cmd` on the queued mission (a3 = forced).
            let command = |c: &mut Cpu, a2: u64, ret: u32| {
                c.g[A1][0] = c.lw(this + MISSION);
                c.g[A0][0] = sx(this);
                c.g[A2][0] = a2;
                c.g[A3][0] = 0;
                MISSION_COMMAND.call(c, ret)
            };
            let setup_scripts = |c: &mut Cpu, ret: u32| {
                c.g[A0][0] = sx(this);
                RUN_SETUP_SCRIPTS.call(c, ret)
            };
            match target {
                // 1: start the mission.
                0x0030_9154 => {
                    c.cycles += 18;
                    setup_scripts(c, 0x0030_915c)?;
                    command(c, 1, 0x0030_9170)?;
                    call_ptr(c, AUDIO_GET_INSTANCE, 0x0030_9178)?;
                    c.g[A1][0] = c.lw(this + AUDIO_ID);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, AUDIO_ON_MISSION_END, 0x0030_9184)?;
                    let gp = r32(c, GP);
                    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x28));
                    call_ptr(c, REPLAY_ON_MISSION, 0x0030_9194)?;
                }
                // 2: command 3.
                0x0030_919c => {
                    c.cycles += 7;
                    command(c, 3, 0x0030_91b0)?;
                }
                // 3 / 4: command 0x12 / 5, setup scripts, FE sound.
                0x0030_91b8 | 0x0030_91c4 => {
                    if target == 0x0030_91b8 {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw(this + MISSION);
                        c.g[A2][0] = 0x12;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.lw(this + MISSION);
                        c.g[A2][0] = 5;
                    }
                    c.cycles += 12;
                    c.g[A3][0] = 0;
                    c.g[A0][0] = sx(this);
                    MISSION_COMMAND.call(c, 0x0030_91d8)?;
                    setup_scripts(c, 0x0030_91e0)?;
                    call_ptr(c, AUDIO_GET_INSTANCE, 0x0030_91e8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, AUDIO_FE_SOUND, 0x0030_91f4)?;
                }
                // 5: playback hook, forced command 1, setup scripts.
                0x0030_91fc => {
                    c.cycles += 13;
                    let gp = r32(c, GP);
                    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x34));
                    call_ptr(c, PLAYBACK_ON_MISSION, 0x0030_920c)?;
                    c.g[A1][0] = c.lw(this + MISSION);
                    c.g[A0][0] = sx(this);
                    c.g[A2][0] = 1;
                    c.g[A3][0] = 1;
                    MISSION_COMMAND.call(c, 0x0030_9220)?;
                    setup_scripts(c, 0x0030_9228)?;
                }
                // 6: command 0x12, setup scripts.
                0x0030_9230 => {
                    c.cycles += 7;
                    command(c, 0x12, 0x0030_9244)?;
                    setup_scripts(c, 0x0030_924c)?;
                }
                _ => {
                    c.jump_out(target, 0x0030_9118, 0x0030_926c);
                    return Err(crate::call::Unwound);
                }
            }
        }
        // Clear the request.
        c.cycles += 8;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sw(this + REQUEST, 0);
        c.g[V0][0] |= 0xffff;
        frame.restore_ra(c, 0);
        c.sw(this + MISSION, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_Update (0x309270)

/// Script manager fields: the five per-frame counters cleared on entry, the
/// pending-command list (`count` then 12-byte commands, copied whole: 0x184
/// bytes), the world reference, the current lookup result and process, the
/// process lists.
pub(crate) const WSU_COUNTERS: [u32; 5] = [0x4, 0x10, 0x1c, 0x28, 0x34];

pub(crate) const WSU_COMMANDS: u32 = 0x40;

pub(crate) const WSU_COMMANDS_END: u32 = 0x1c0;

pub(crate) const WSU_WORLD_REF: u32 = 0x28c;

pub(crate) const WSU_LOOKUP: u32 = 0x290;

pub(crate) const WSU_CURRENT: u32 = 0x2a4;

pub(crate) const WSU_RUNNING: u32 = 0x2b4;

pub(crate) const WSU_FIRST_RUNNING: u32 = 0x2b8;

pub(crate) const WSU_STARTING: u32 = 0x2bc;

/// A process's next link.
pub(crate) const WSU_PROC_NEXT: u32 = 0x18;

/// Command flags: dispatch to the process, forward to its task.
pub(crate) const WSU_DISPATCH: u64 = 0x20000;

pub(crate) const WSU_FORWARD: u64 = 0x10000;

pub(crate) const WSU_PROCESS_PENDING: Callee = Callee {
    addr: 0x0030_9118,
    f: ee::s10::WScriptMan_ProcessPendingRequest,
};

pub(crate) const WSU_POP_FRONT: Callee = Callee {
    addr: 0x0030_b208,
    f: ee::s10::WScriptProcessList_PopFront,
};

pub(crate) const WSU_PUSH_FRONT: Callee = Callee {
    addr: 0x0030_b260,
    f: ee::s10::WScriptProcessList_PushFront,
};

pub(crate) const WSU_START_PROCESS: Callee = Callee {
    addr: 0x0030_9750,
    f: ee::s10::WScriptMan_StartProcess,
};

pub(crate) const WSU_DISPATCH_EVENT: Callee = Callee {
    addr: 0x0030_78c0,
    f: ee::s10::WScriptProcess_DispatchEvent,
};

pub(crate) const WSU_FORWARD_EVENT: Callee = Callee {
    addr: 0x0030_7a10,
    f: ee::s10::WScriptProcess_ForwardEventToTask,
};

pub(crate) const WSU_PROCESS_UPDATE: Callee = Callee {
    addr: 0x0030_77f8,
    f: ee::s10::WScriptProcess_Update,
};

/// `void WScriptMan_Update(WScriptMan* self)` (0x309270): the script
/// manager's frame. Handle the pending request, clear the five counters,
/// move every starting process to the running list and start it, then take
/// the frame's command list (copied to the stack, the original cleared):
/// for each command, look up its world object when the command's type is 0
/// (`self+0x290`), and hand it to every running process (dispatch and/or
/// forward to its task, by the command's flags, with `self+0x2a4` = the
/// process). Finally update every running process.
///
/// Dispatcher-shaped: an event at a loop back edge can unwind, and the
/// function is re-entered at that loop head (0x3092b8, 0x3092f4, 0x30934c,
/// 0x309398, 0x309418, 0x309480) with its state in the registers.
pub fn wscript_man_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x220 };
        let s2 = |c: &Cpu| r32(c, S2);
        let mut pc = c.pc;
        loop {
            match pc {
                0x0030_9270 => {
                    c.cycles += 18;
                    let _ = Frame::open(c, 0x220);
                    frame.save(c, S2, 0x1f0);
                    frame.save(c, S0, 0x210);
                    frame.save(c, S1, 0x200);
                    frame.save(c, S3, 0x1e0);
                    frame.save(c, S4, 0x1d0);
                    frame.save(c, S5, 0x1c0);
                    frame.save(c, S6, 0x1b0);
                    frame.save(c, S7, 0x1a0);
                    frame.save_ra(c, 0x190);
                    c.g[S2][0] = c.g[A0][0];
                    WSU_PROCESS_PENDING.call(c, 0x0030_92a0)?;
                    for off in WSU_COUNTERS {
                        Addr(s2(c)).off(off).sw(c, 0);
                    }
                    pc = 0x0030_92cc;
                }
                // Start the next process taken off the starting list.
                0x0030_92b8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    WSU_PUSH_FRONT.call(c, 0x0030_92c0)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    WSU_START_PROCESS.call(c, 0x0030_92cc)?;
                    pc = 0x0030_92cc;
                }
                0x0030_92cc => {
                    c.cycles += 5;
                    c.g[A0][0] = sx(s2(c).wrapping_add(WSU_STARTING));
                    WSU_POP_FRONT.call(c, 0x0030_92d4)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] != 0 {
                        c.g[A0][0] = sx(s2(c).wrapping_add(WSU_RUNNING));
                        if back_edge_unwinds(c, 0x0030_92b8) {
                            return Ok(());
                        }
                        pc = 0x0030_92b8;
                        continue;
                    }
                    // Copy the command list to the stack (unaligned or
                    // doubleword copies, 0x20 bytes a step).
                    c.cycles += 4;
                    c.g[V1][0] = sx(s2(c).wrapping_add(WSU_COMMANDS));
                    c.g[V0][0] = c.g[V1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx(s2(c).wrapping_add(WSU_COMMANDS_END));
                    pc = if aligned { 0x0030_934c } else { 0x0030_92f4 };
                }
                0x0030_92f4 => {
                    c.cycles += 19;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let o = 8 * i as u32;
                        c.ldl(reg, src.wrapping_add(o + 7));
                        c.ldr(reg, src.wrapping_add(o));
                    }
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        let o = 8 * i as u32;
                        c.sdl(reg, dst.wrapping_add(o + 7));
                        c.sdr(reg, dst.wrapping_add(o));
                    }
                    c.g[V1][0] = sx(src.wrapping_add(0x20));
                    c.g[A0][0] = sx(dst.wrapping_add(0x20));
                    if c.g[V1][0] != c.g[V0][0] {
                        if back_edge_unwinds(c, 0x0030_92f4) {
                            return Ok(());
                        }
                        continue;
                    }
                    c.cycles += 2;
                    pc = 0x0030_9378;
                }
                0x0030_934c => {
                    c.cycles += 11;
                    let (src, dst) = (r32(c, V1), r32(c, A0));
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.g[reg][0] = c.ld(src.wrapping_add(8 * i as u32));
                    }
                    for (i, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                        c.sd(dst.wrapping_add(8 * i as u32), c.g[reg][0]);
                    }
                    c.g[V1][0] = sx(src.wrapping_add(0x20));
                    c.g[A0][0] = sx(dst.wrapping_add(0x20));
                    if c.g[V1][0] != c.g[V0][0] {
                        if back_edge_unwinds(c, 0x0030_934c) {
                            return Ok(());
                        }
                        continue;
                    }
                    pc = 0x0030_9378;
                }
                0x0030_9378 => {
                    // The last word, clear the list, and walk the commands.
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    let v = c.g[V0][0];
                    Addr(r32(c, A0)).sw(c, v);
                    Addr(s2(c)).off(WSU_COMMANDS).sw(c, 0);
                    c.g[V0][0] = Addr(sp(c)).lw(c);
                    c.g[A2][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x0030_9470;
                        continue;
                    }
                    c.cycles += 2;
                    c.g[S7][0] = 0xc;
                    pc = 0x0030_9398;
                }
                // Command a2 (s3 = &commands[a2]).
                0x0030_9398 => {
                    c.cycles += 6;
                    c.mult(0, V0, A2, S7);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
                    c.g[S3][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, S3)).lhu(c);
                    if c.g[V1][0] == 0 {
                        wsu_lookup(c);
                    }
                    c.g[S0][0] = Addr(s2(c)).off(WSU_FIRST_RUNNING).lw(c);
                    c.cycles += 2;
                    c.g[S4][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if c.g[S0][0] == 0 {
                        pc = 0x0030_9458;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[S6][0] = WSU_DISPATCH;
                    c.g[S5][0] = WSU_FORWARD;
                    c.g[S1][0] = c.g[S0][0];
                    pc = 0x0030_9418;
                }
                // Hand the command to process s1 (s0 = the next one).
                0x0030_9418 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                    c.g[S0][0] = Addr(r32(c, S0)).off(WSU_PROC_NEXT).lw(c);
                    c.g[V0][0] &= c.g[S6][0];
                    let v = c.g[S1][0];
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, v);
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S3][0];
                        WSU_DISPATCH_EVENT.call(c, 0x0030_9438)?;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                    c.g[V0][0] &= c.g[S5][0];
                    c.g[A0][0] = c.g[S1][0];
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S3][0];
                        WSU_FORWARD_EVENT.call(c, 0x0030_9450)?;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[S1][0] = c.g[S0][0];
                        if back_edge_unwinds(c, 0x0030_9418) {
                            return Ok(());
                        }
                        continue;
                    }
                    pc = 0x0030_9458;
                }
                0x0030_9458 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(sp(c)).lw(c);
                    c.g[A2][0] = c.g[S4][0];
                    Addr(s2(c)).off(WSU_LOOKUP).sw(c, 0);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, 0);
                    if c.g[V0][0] != 0 {
                        if back_edge_unwinds(c, 0x0030_9398) {
                            return Ok(());
                        }
                        pc = 0x0030_9398;
                        continue;
                    }
                    pc = 0x0030_9470;
                }
                0x0030_9470 => {
                    c.cycles += 3;
                    c.g[S0][0] = Addr(s2(c)).off(WSU_FIRST_RUNNING).lw(c);
                    if c.g[S0][0] == 0 {
                        Addr(s2(c)).off(WSU_CURRENT).sw(c, 0);
                        pc = 0x0030_949c;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0030_9480;
                }
                // Update process v0 (s0 = the next one).
                0x0030_9480 => {
                    c.cycles += 6;
                    c.g[S0][0] = Addr(r32(c, S0)).off(WSU_PROC_NEXT).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    let v = c.g[V0][0];
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, v);
                    WSU_PROCESS_UPDATE.call(c, 0x0030_9490)?;
                    c.g[V0][0] = c.g[S0][0];
                    if c.g[S0][0] != 0 {
                        if back_edge_unwinds(c, 0x0030_9480) {
                            return Ok(());
                        }
                        continue;
                    }
                    c.cycles += 1;
                    Addr(s2(c)).off(WSU_CURRENT).sw(c, 0);
                    pc = 0x0030_949c;
                }
                0x0030_949c => {
                    c.cycles += 11;
                    frame.restore(c, S0, 0x210);
                    frame.restore(c, S1, 0x200);
                    frame.restore(c, S2, 0x1f0);
                    frame.restore(c, S3, 0x1e0);
                    frame.restore(c, S4, 0x1d0);
                    frame.restore(c, S5, 0x1c0);
                    frame.restore(c, S6, 0x1b0);
                    frame.restore(c, S7, 0x1a0);
                    frame.restore_ra(c, 0x190);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030_9270, 0x0030_94c8);
                    return Ok(());
                }
            }
        }
    })
}

/// The world object a type-0 command names (`id & 0xff` selects a table
/// entry, `id >> 8` a word in it; `self+0x290` = that word's top 24 bits
/// times 4, or 0), as `WScriptMan_Update` inlines it. s3 = the command.
pub(crate) fn wsu_lookup(c: &mut Cpu) {
    c.cycles += 10;
    let s2 = r32(c, S2);
    c.g[V1][0] = Addr(s2).off(WSU_WORLD_REF).lw(c);
    c.g[A1][0] = Addr(r32(c, S3)).off(4).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    c.g[V0][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = Addr(r32(c, A0)).off(8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
    let mut found = false;
    if c.g[V1][0] != 0 {
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    let v = c.g[V0][0];
    Addr(s2).off(WSU_LOOKUP).sw(c, v);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::pool8;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_4 as pool_allocator;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::ovstates::tests::base_3 as base_5;
    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;

    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::wh;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;

    pub(crate) use crate::game::object::multiparticle::tests::be_library_2 as be_library;
    pub(crate) use crate::game::render::lightman::tests::eafile_pools;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::ui::uiengine::tests::POOL0;

    pub(crate) use crate::game::worldpainter::quadtree::tests::TBL;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    // ----- s10 ------------------------------------------------------------

    #[test]
    fn wscript_process_get_current_task_id_matches() {
        check_lohi(
            ssx3_ee::s10::f_003079d8,
            wscript_process_get_current_task_id,
            250,
            &|c, seed| {
                let mut r = Rng(0x2001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let (v, a) = if seed % 2 == 0 {
                    (0x100u32, 0x40u32)
                } else {
                    (0x40u32, 0x100u32)
                };
                c.bus.write32(DATA + 4, v);
                c.bus.write32(DATA + 8, a);
                c.bus.write32(DATA + 0x14, 0);
                c.bus.write32(0x1104, r.next_u32());
            },
        );
    }

    // ------------------------------------------------- script and rider

    #[test]
    fn wscript_man_lookup_func_matches() {
        check(
            ee::s10::f_00309b70__generated,
            wscript_man_lookup_func,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x309b_70, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(r.next_u32()) };
                c.bus.write32(DATA + 0x100 + 0x28c, DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 8);
                // The lookup's table base: zero keeps the (fake-free) lookup in
                // low memory instead of following a random pointer.
                c.bus.write32(DATA + 0x404, 0);
            },
        );
    }

    #[test]
    fn wscript_task_save_state_matches() {
        check(
            ee::s10::f_00308760__generated,
            wscript_task_save_state,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x3087_60, 8);
            },
        );
    }

    #[test]
    fn wscript_task_load_state_matches() {
        check(
            ee::s10::f_00308798__generated,
            wscript_task_load_state,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x3087_98, 0x10);
            },
        );
    }

    // -----------------------------------------------------------------------
    // WScriptMan_DeleteAllProcesses.

    #[test]
    fn w_script_man_delete_all_processes_matches() {
        check(
            ssx3_ee::s10::f_00308f38__generated,
            w_script_man_delete_all_processes,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x308f_38, 0);
                c.g[A0][0] = DATA as u64;
                // Zeroed process lists: PopFront returns null at once.
                c.bus.write32(DATA + 0x2b8, 0);
                c.bus.write32(DATA + 0x2bc, 0);
                c.bus.write32(DATA + 0x2b4, 0);
                c.bus.write32(DATA + 0x10, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------
    // WScript tasks.

    #[test]
    fn wscript_task_start_matches() {
        check(
            ee::s10::f_003086b0__generated,
            wscript_task_start,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3086b0, 0);
                let self_at = DATA + 0x200;
                let mgr = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus
                    .write32(self_at, if seed % 3 == 0 { 1 } else { (seed % 4) as u32 });
                c.bus
                    .write32(self_at + 8, if seed % 2 == 0 { 0 } else { 1 + seed % 5 });
                c.bus.write32(self_at + 0x3c, mgr);
                c.bus.write32(mgr + 0x10, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn wscript_task_stop_matches() {
        check(
            ee::s10::f_00308720__generated,
            wscript_task_stop,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x308720, 0);
                let self_at = DATA + 0x200;
                let mgr = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus
                    .write32(self_at, if seed % 3 == 0 { 3 } else { (seed % 4) as u32 });
                c.bus
                    .write32(self_at + 0x10, if seed % 2 == 0 { 0 } else { 1 + seed % 5 });
                c.bus.write32(self_at + 0x3c, mgr);
                c.bus.write32(mgr + 0x10, (seed % 3) as u32);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x307d30 WScriptProcess_v06.

    #[test]
    fn w_script_process_v06_matches() {
        check(
            ee::s10::f_00307d30__generated,
            w_script_process_v06,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x307d_30, 2);
                pool_allocator(c);
                be_library(c);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let proc = DATA + 0x400;
                let script = DATA + 0x500;
                c.bus.write32(this + 0x10, proc);
                c.bus.write32(this + 0x1c, 0);
                c.bus.write32(proc + 0x2a4, DATA + 0x600);
                c.bus.write32(proc + 0x290, 0);
                // The string helper reads a word from the caller's frame.
                c.bus.write32(crate::difftest::SP - 0xc0, 0);
                c.bus
                    .write32(this + 0x30, if seed % 2 == 0 { 0 } else { script });
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x307dc8 WScriptProcess_v02.

    #[test]
    fn w_script_process_v02_matches() {
        check(
            ee::s10::f_00307dc8__generated,
            w_script_process_v02,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x307d_c8, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x10, 0); // not in any list
                } else {
                    let proc = DATA + 0x400;
                    let node = DATA + 0x600;
                    let id = r.next_u32() & 0xffff;
                    c.bus.write32(this + 0x10, proc);
                    c.bus.write32(proc + 0x2b8, node);
                    c.bus.write32(node + 0x20, id);
                    c.bus.write32(this + 0x20, id);
                    let vt = vtable(c, &mut r, &[0x30]);
                    c.bus.write32(this + 0x5c, vt);
                }
            },
        );
    }

    #[test]
    fn wscript_task_ctor_matches() {
        check(
            ee::s10::f_00308228__generated,
            wscript_task_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x3082_28);
                let this = DATA + 0x100;
                for i in 0..0x20u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn wscript_man_unload_section_and_free_matches() {
        check(
            ee::s10::f_00308fe0__generated,
            wscript_man_unload_section_and_free,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x308f_e0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let cache = DATA + 0x600;
                clear(c, cache, 0x40);
                c.bus.write32(this + 0x28c, cache);
                c.bus.write32(cache + 4, cache + 0x10);
                c.bus.write32(cache + 0x14, DATA + 0x700);
                clear(c, DATA + 0x700, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn wscript_process_forward_event_to_task_matches() {
        check(
            ee::s10::f_00307a10__generated,
            wscript_process_forward_event_to_task,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x307a_10, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let base = DATA + 0x800;
                clear(c, this, 0x20);
                clear(c, base, 0x400);
                let count = (seed % 3) as u32;
                let cur = (seed % 4) as u32;
                c.bus.write32(this + 4, count);
                c.bus.write32(this + 8, cur);
                c.bus.write32(this + 0x14, base);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                if cur < count {
                    let entry = base + cur * 0x44;
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(entry + 0x40, vt);
                }
            },
        );
    }

    #[test]
    fn w_script_man_free_local_tables_matches() {
        check(
            ee::s10::f_00308a48__generated,
            w_script_man_free_local_tables,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x308a48, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x1000);
                for i in 0..50u32 {
                    if (seed + i) % 4 != 0 {
                        let table = heap_block(c, 0x40);
                        clear(c, table, 0x10);
                        c.bus.write32(this + 0x3cc + 4 * i, table);
                    }
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn wscript_man_unload_section_matches() {
        check(
            ssx3_ee::s10::f_00308f70__generated,
            wscript_man_unload_section,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x308f70, 0);
                let m = DATA + 0x1000;
                c.g[A0][0] = m as u64;
                c.g[A1][0] = (seed % 4) as u64;
                w32(c, m + 0x2b4, 0);
                w32(c, m + 0x2b8, 0);
                w32(c, m + 0x2bc, 0);
                for i in 0..4u32 {
                    w32(c, m + 0x3cc + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn wscript_task_update_matches() {
        check(
            ee::s10::f_00308328__generated,
            wscript_task_update,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x308328, 1);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(c, obj, (seed % 4) as u32);
                let vt = DATA + 0x3000;
                w(c, obj + 0x40, vt);
                slot(c, vt, 0x28, 0x40, FAKE);
            },
        );
    }

    #[test]
    fn w_script_man_spawn_processes_matches() {
        check(
            ee::s10::f_00308be0__generated,
            w_script_man_spawn_processes,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x308b_e0, 8);
                eafile_pools(c);
                let self_ = DATA;
                // The wrappers do `lw(lw(man+4)+4) + section` before reading
                // the cache; section 0 keeps the cache pointer aligned.
                let man = DATA + 0x1000;
                let mid = DATA + 0x1100;
                let cache = DATA + 0x1200;
                let table = DATA + 0x1300;
                w(c, self_ + 0x28c, man);
                w(c, man + 4, mid);
                w(c, mid + 4, cache);
                wh(c, cache, 3);
                w(c, cache + 8, table);
                let n = (seed % 4) as u32;
                w(c, table + 0x28, n);
                w(c, table + 0x2c, DATA + 0x3000);
                for i in 0..n {
                    w(c, DATA + 0x3000 + 0x40 * i, 0);
                }
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn w_script_process_jump_to_task_matches() {
        check(
            ee::s10::f_00307ec0__generated,
            w_script_process_jump_to_task,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let table = DATA + 0x1000;
                let count = seed % 3;
                w(c, this + 4, count);
                w(c, this + 8, (seed & 2) as u32);
                w(c, this + 0x14, table);
                let target = (seed & 1) as u64;
                let hit = if count > 0 && seed & 4 != 0 {
                    (seed % count) as u32
                } else {
                    u32::MAX
                };
                for i in 0..count {
                    let task = table + 0x44 * i;
                    w(c, task + 4, if i == hit { 0x1234 } else { 0x5678 });
                    let obj = DATA + 0x1200 + 0x40 * i;
                    w(c, task + 0x40, obj);
                    wh(c, obj + 0x20, 0);
                    w(c, obj + 0x24, RET_ZERO);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = target;
            }),
        );
    }

    #[test]
    fn wscript_man_dtor_matches() {
        check(
            ee::s10::f_003088d8__generated,
            wscript_man_dtor,
            6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0030_88d8, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
                w(c, OBJ + 0x28c, OBJ2);
                w(c, OBJ + 0x2bc, 0);
                w(c, OBJ + 0x2b8, 0);
                w(c, OBJ + 0x2b4, 0);
                w(c, OBJ2 + 4, OBJ3);
            },
        );
    }

    #[test]
    fn wscript_process_next_task_matches() {
        check(
            ee::s10::f_00307e10__generated,
            wscript_process_next_task,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0030_7e10, 3);
                c.g[A0][0] = OBJ as u64;
                w(c, OBJ, if seed % 4 == 0 { 3 } else { (seed % 3) as u32 });
                w(c, OBJ + 8, (seed % 4) as u32);
                w(c, OBJ + 4, 4);
                w(c, OBJ + 0x14, OBJ2);
                w(c, OBJ2 + 0x40, OBJ3);
                c.bus.write16(OBJ3 + 0x30, (seed as u16) & !3);
                w(c, OBJ3 + 0x34, FAKE);
                c.bus.write16(OBJ3 + 0x20, (seed as u16) & !3);
                w(c, OBJ3 + 0x24, FAKE + 0x10);
                w(c, OBJ + 0x5c, OBJ3);
                c.bus.write16(OBJ3 + 0x10, (seed as u16) & !3);
                w(c, OBJ3 + 0x14, FAKE + 0x20);
            },
        );
    }

    #[test]
    fn wscript_task_init_matches() {
        check(
            ee::s10::f_00308278__generated,
            wscript_task_init,
            6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0030_8278, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ3 as u64;
                c.g[A2][0] = OBJ2 as u64;
                w(c, OBJ2, (seed % 8) as u32);
                for i in 0..4 {
                    w(c, OBJ2 + 4 + 4 * i, 0x100 + i);
                }
                for i in 0..10 {
                    w(c, OBJ2 + 0x10 + 4 * i, 0x200 + i);
                }
                w(c, OBJ + 0x28c, OBJ3);
            },
        );
    }

    // ------------------------------------------------------------- scripting

    #[test]
    fn wscript_man_create_local_tables_matches() {
        check(
            ee::s10::f_00308aa0__generated,
            wscript_man_create_local_tables,
            6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0030_8aa0, 2);
                c.g[A0][0] = OBJ as u64;
                let count = 1 + (seed % 3) as u32;
                w(c, OBJ + 0x28c, OBJ2);
                w(c, OBJ2, OBJ3);
                w(c, OBJ3 + 0x10, OBJ4);
                w(c, OBJ4 + 4, count);
                w(c, OBJ2 + 4, OBJ4);
                w(c, OBJ4 + 4, TBL);
                for i in 0..count {
                    let e = TBL + i * 0xc;
                    c.bus.write16(e, if seed % 2 == 0 { 2 } else { 0 });
                    w(c, e + 8, if seed % 3 == 0 { 0 } else { 1 });
                }
            },
        );
    }

    // --------------------------------------------------------------- Script.

    #[test]
    fn wscript_process_goto_task_matches() {
        check(
            ee::s10::f_00307f58__generated,
            wscript_process_goto_task,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_7f58, 2);
                let this = DATA;
                c.g[A0][0] = sx(this);
                // Id 3 never matches, so some seeds walk the whole list.
                c.g[A1][0] = sx(if seed % 4 == 0 { 3 } else { seed % 3 });
                let n = (seed % 4) as u32;
                c.bus.write32(this + 4, n);
                c.bus.write32(this + 8, r.next_u32() % (n + 1));
                let arr = DATA + 0x1000;
                c.bus.write32(this + 0x14, arr);
                for i in 0..4u32 {
                    let t = arr + i * 0x44;
                    c.bus.write32(t + 4, (seed as u32 + i) % 3);
                    let obj = DATA + 0x2000 + i * 0x100;
                    c.bus.write32(t + 0x40, obj);
                    c.bus.write16(obj + 0x20, 0);
                    c.bus.write32(obj + 0x24, FAKE + 0x10 * i);
                    c.bus.write16(obj + 0x30, 0);
                    c.bus.write32(obj + 0x34, FAKE + 0x40 + 0x10 * i);
                }
            },
        );
    }

    #[test]
    fn wscript_man_reset_matches() {
        check(
            ee::s10::f_00308988__generated,
            wscript_man_reset,
            50,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x30_8988);
                c.g[A0][0] = sx(DATA);
                // Only the state fields: the local-table pointers at_2 +0x3cc must
                // stay zero so `WScriptMan_FreeLocalTables` walks an empty table.
                for i in 0..0x80u32 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn w_script_process_dtor_matches() {
        strict(
            ee::s10::f_00307738__generated,
            w_script_process_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3077, 10);
                pool8(c, POOL0);
                let this = POOL0 + 0x400;
                let arr = POOL0 + 0x100;
                c.bus.write32(arr - 0x10, 1);
                c.bus.write32(arr, r.next_u32());
                c.bus
                    .write32(this + 0x14, if seed % 3 == 0 { 0 } else { arr });
                c.bus
                    .write32(this + 0x1c, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn wscript_process_update_matches() {
        check(
            ee::s10::f_003077f8__generated,
            wscript_process_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3077, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, (seed % 4) as u32);
                c.bus.write32(this + 4, (seed % 3) as u32);
                c.bus.write32(this + 8, (seed % 4) as u32);
                c.bus.write32(this + 0x14, seed % 2);
                c.bus.write32(this + 0x40, DATA + 0x2000);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(this + 0x5c, vt);
            },
        );
    }

    #[test]
    fn w_script_process_save_state_matches() {
        diff(
            ee::s10::f_00308038__generated,
            w_script_process_save_state,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x30_8038);
                let obj = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
                w(c, obj, 0); // no vtable: the two virtual calls are fakes
                w(c, DATA + 4, (seed % 3) as u32);
                w(c, DATA + 0x14, DATA + 0x3000);
                w(c, DATA + 0x1c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, DATA + 0x24, r.next_u32());
                w(c, GAME_GP.wrapping_add(0x16c8), DATA + 0x4000);
                w(c, DATA + 0x4004, r.next_u32());
                w(c, DATA + 0x3000, r.next_u32());
            },
        );
    }

    #[test]
    fn w_script_man_restart_matches() {
        diff(
            ee::s10::f_00309030__generated,
            w_script_man_restart,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x30_9030);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x28c, DATA + 0x2000);
                w(c, DATA + 0x2000, DATA + 0x2100);
                w(c, DATA + 0x2004, DATA + 0x2200);
                w(c, DATA + 0x2104, DATA + 0x2300);
                w(c, DATA + 0x2110, 0);
                w(c, DATA + 0x2204, DATA + 0x2300);
                w(c, DATA + 0x2300, r.next_u32());
                w(c, DATA + 0x2308, r.next_u32());
            },
        );
    }

    // ------------------------------------------- WScriptMan_ProcessPendingRequest

    /// `WScriptMan_ProcessPendingRequest`: every request kind 0..7 (0 and 7
    /// out of the jump table). The real callees take their short paths: no
    /// world sections (setup scripts), mission commands that stop at the
    /// game-state check (`0x535c10 != 4`, behind a fake back-end interface
    /// call that may unwind), an audio system whose mission end and FE
    /// sound find nothing playing, an idle replay and a busy playback.
    #[test]
    fn wscript_man_process_pending_request_matches() {
        use super::wscript_pending::*;
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, elf_image, DATA, GAME_GP};
        const BE_LIBRARY: u32 = DATA + 0x8_6000;
        const BE_IFACE: u32 = DATA + 0x8_7000;
        const BE_VTABLE: u32 = DATA + 0x8_7100;
        check(
            ee::s10::f_00309118__generated,
            wscript_man_process_pending_request,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_9118, 2);
                elf_image(c);
                c.pc = 0x0030_9118;
                let (this, view, sections) = (DATA, DATA + 0x400, DATA + 0x500);
                let (audio, world, game, riders, rider) = (
                    DATA + 0x1_0000,
                    DATA + 0x2000,
                    DATA + 0x2100,
                    DATA + 0x2200,
                    DATA + 0x2300,
                );
                let (replay, playback) = (DATA + 0x3000, DATA + 0x3100);
                c.bus.write32(this + REQUEST, seed % 8);
                c.bus.write32(this + MISSION, r.next_u32());
                c.bus.write32(this + AUDIO_ID, r.next_u32());
                // Setup scripts: a world view without sections.
                c.bus.write32(this + 0x28c, view);
                c.bus.write32(view, sections);
                c.bus.write32(sections + 0x14, 0);
                // Mission command: the back end's interface (a fake call),
                // then not in game state 4.
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VTABLE);
                c.bus.write16(BE_VTABLE + 8, 0);
                c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
                c.bus.write8(0x0053_5c10, (r.next_u32() % 4) as u8);
                // The audio system: nothing playing.
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(audio + 0x5fd4, 0);
                c.bus.write32(audio + 0x5fdc, 0xffff_ffff);
                c.bus.write32(audio + 0x118 + 0x408, 0);
                c.bus.write32(audio + 0x608c, 2);
                // The world: rider without wipeout loops, replay and playback.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, riders);
                c.bus.write32(riders + 0x28, rider);
                c.bus.write32(rider + 0x774, 0xffff_ffff);
                c.bus.write32(rider + 0x778, 0xffff_ffff);
                c.bus.write32(game + 0x28, replay);
                c.bus.write32(replay, 0);
                c.bus.write32(game + 0x34, playback);
                c.bus.write32(playback + 0x70, 1);
                c.g[A0][0] = sx(this);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_CTRL;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_MAGIC;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_POP_NEXT;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::OC360_POP_QUEUE;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::oc360_fake;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::tests::oc360_gated;

    oc360_gated!(
        oc360_fake_wsm_pending,
        OC360_PREV_WSM_PENDING,
        0x0030_9118,
        ee::s10::f_00309118__generated
    );

    oc360_gated!(
        oc360_fake_wsm_pop,
        OC360_PREV_WSM_POP,
        0x0030_b208,
        ee::s10::f_0030b208__generated
    );

    oc360_gated!(
        oc360_fake_wsm_push,
        OC360_PREV_WSM_PUSH,
        0x0030_b260,
        ee::s10::f_0030b260__generated
    );

    oc360_gated!(
        oc360_fake_wsm_start,
        OC360_PREV_WSM_START,
        0x0030_9750,
        ee::s10::f_00309750__generated
    );

    oc360_gated!(
        oc360_fake_wsp_forward,
        OC360_PREV_WSP_FORWARD,
        0x0030_7a10,
        ee::s10::f_00307a10__generated
    );

    oc360_gated!(
        oc360_fake_wsp_update,
        OC360_PREV_WSP_UPDATE,
        0x0030_77f8,
        ee::s10::f_003077f8__generated
    );

    pub(crate) fn oc360_install_wsm_fakes() {
        use std::ptr::addr_of_mut;
        {
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00309118),
                oc360_fake_wsm_pending,
                &OC360_PREV_WSM_PENDING,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B208),
                oc360_fake_wsm_pop,
                &OC360_PREV_WSM_POP,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0030B260),
                oc360_fake_wsm_push,
                &OC360_PREV_WSM_PUSH,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00309750),
                oc360_fake_wsm_start,
                &OC360_PREV_WSM_START,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00307A10),
                oc360_fake_wsp_forward,
                &OC360_PREV_WSP_FORWARD,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_003077F8),
                oc360_fake_wsp_update,
                &OC360_PREV_WSP_UPDATE,
            );
        }
    }

    // ------------------------------------------- WScriptMan_Update

    /// The manager (8-aligned or not: the two copy loops), 0-3 starting
    /// processes, 0-3 running ones, up to 5 commands (type 0 with a world
    /// lookup that finds nothing, an empty word or a word; other types with
    /// the real `WScriptProcess_DispatchEvent` on fake methods; dispatch /
    /// forward flags). The other callees are logged fakes.
    pub(crate) fn wsu_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        elf_image(c);
        let mut r = machine(c, seed, 0x0030_9270, calls);
        c.pc = 0x0030_9270;
        let base = DATA + 0x7_1000;
        let man = base + 4 * (seed % 2);
        let procs = base + 0x1000;
        let p10 = base + 0x2000;
        let wref = base + 0x3000;
        let table = base + 0x3100;
        let entries = base + 0x3200;
        c.bus.write32(OC360_CTRL, OC360_MAGIC);
        c.bus.write32(OC360_POP_NEXT, 0);
        let starting = r.next_u32() % 4;
        for i in 0..8 {
            let v = if i < starting { r.next_u32() | 0x10 } else { 0 };
            c.bus.write32(OC360_POP_QUEUE + 4 * i, v);
        }
        c.g[A0][0] = sx(man);
        fill(c, &mut r, man, 0xc0);
        // Commands.
        let count = match r.next_u32() % 6 {
            0 => 0,
            1 => 0u32.wrapping_sub(1 + r.next_u32() % 3),
            n => n + (r.next_u32() % 2),
        };
        c.bus.write32(man + WSU_COMMANDS, count);
        for i in 0..32 {
            let cmd = man + WSU_COMMANDS + 4 + 12 * i;
            let ty = match r.next_u32() % 3 {
                0 => 0,
                _ => r.next_u32() % 12,
            };
            let flags = (r.next_u32() % 4) << 16;
            c.bus.write32(cmd, flags | ty);
            c.bus
                .write32(cmd + 4, (r.next_u32() % 4) << 8 | r.next_u32() % 4);
            c.bus.write32(cmd + 8, r.next_u32());
        }
        // World lookup tables.
        c.bus.write32(man + WSU_WORLD_REF, wref);
        c.bus.write32(wref, wref + 0x10);
        c.bus.write32(wref + 0x10 + 8, table);
        for i in 0..4 {
            let e = entries + 0x40 * i;
            c.bus
                .write32(table + 4 * i, if r.next_u32() % 3 == 0 { 0 } else { e });
            c.bus.write32(e + 0x1c, e + 0x20);
            for j in 0..4 {
                let w = if r.next_u32() % 3 == 0 {
                    r.next_u32() & 0xff
                } else {
                    r.next_u32()
                };
                c.bus.write32(e + 0x20 + 4 * j, w);
            }
        }
        // Running processes.
        let running = r.next_u32() % 4;
        c.bus.write32(
            man + WSU_FIRST_RUNNING,
            if running == 0 { 0 } else { procs },
        );
        for i in 0..running {
            let p = procs + 0x100 * i;
            let next = if i + 1 < running { p + 0x100 } else { 0 };
            c.bus.write32(p + WSU_PROC_NEXT, next);
            c.bus.write32(p + 0x10, p10);
            let vt = fakes::vtable(
                c,
                &mut r,
                &[0x38, 0x40, 0x48, 0x50, 0x58, 0x60, 0x68, 0x70, 0x78, 0x80],
            );
            c.bus.write32(p + 0x5c, vt);
        }
        r
    }

    #[test]
    fn wscript_man_update_matches() {
        use crate::difftest::events::{check_events, schedule};
        oc360_install_wsm_fakes();
        covered(
            ee::s10::f_00309270__generated,
            wscript_man_update,
            300,
            &|c, seed| {
                wsu_setup(c, seed, 12);
            },
        );
        check_events(
            ee::s10::f_00309270__generated,
            wscript_man_update,
            300,
            &|c, seed| {
                let mut r = wsu_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x0030_9270)
            },
        );
    }
}
