//! Staging module oc555 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc555`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_28f8, game_link_free, "GameLink_Free"),
    (0x002c_2918, game_link_mem_copy, "GameLink_MemCopy"),
    (0x002c_2940, game_link_realloc, "GameLink_Realloc"),
    (
        0x002c_29d0,
        game_link_create_thread,
        "GameLink_CreateThread",
    ),
    (
        0x002c_2aa8,
        game_link_destroy_thread,
        "GameLink_DestroyThread",
    ),
    (0x002c_2b08, game_link_yield, "GameLink_Yield"),
    (0x002c_2b28, game_link_create_mutex, "GameLink_CreateMutex"),
    (
        0x002c_2b68,
        game_link_destroy_mutex,
        "GameLink_DestroyMutex",
    ),
    (0x002c_2b98, game_link_lock_mutex, "GameLink_LockMutex"),
    (0x002c_2bb8, game_link_unlock_mutex, "GameLink_UnlockMutex"),
    (
        0x002c_2bd8,
        game_link_setup_platform_funcs,
        "GameLink_SetupPlatformFuncs",
    ),
    (0x0030_7598, wscript_process_ctor, "WScriptProcess_ctor"),
    (0x0030_7a60, wscript_process_v07, "WScriptProcess_v07"),
    (0x0030_7a90, wscript_process_v08, "WScriptProcess_v08"),
    (0x0030_7ac0, wscript_process_v09, "WScriptProcess_v09"),
    (0x0030_7af0, wscript_process_v10, "WScriptProcess_v10"),
    (0x0030_7b20, wscript_process_v11, "WScriptProcess_v11"),
    (0x0030_7b50, wscript_process_v12, "WScriptProcess_v12"),
    (0x0030_7b80, wscript_process_v13, "WScriptProcess_v13"),
];

// Direct-call targets (the generated function, through its override slot when
// it has one).
const GL_ALLOC: Callee = Callee {
    addr: 0x002c_28c0,
    f: ee::s08::f_002c28c0,
};
const GL_FREE: Callee = Callee {
    addr: 0x002c_28f8,
    f: ee::s08::f_002c28f8,
};
const WS_LOOKUP_FUNC: Callee = Callee {
    addr: 0x0030_9b70,
    f: ee::s10::f_00309b70,
};
const WS_NEW_NAMED: Callee = Callee {
    addr: 0x0031_7e30,
    f: ee::s10::f_00317e30,
};
const WS_TASK_CTOR: Callee = Callee {
    addr: 0x0030_8228,
    f: ee::s10::f_00308228,
};
const WS_TASK_INIT: Callee = Callee {
    addr: 0x0030_8278,
    f: ee::s10::f_00308278,
};
const WS_CALL_PROCESS: Callee = Callee {
    addr: 0x0030_ac98,
    f: ee::s10::f_0030ac98,
};

/// `void GameLink_Free(void* block)` (0x2c28f8): `MEM_Free(block) != 0`.
pub fn game_link_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_23a8, 0x002c_2908)?;
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* GameLink_MemCopy(void* dst, ...)` (0x2c2918): `MEM_copy(...)`, then
/// returns its first argument.
pub fn game_link_mem_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_6574, 0x002c_292c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* GameLink_Realloc(void* block, void* name, int size, int zeroFlag)`
/// (0x2c2940): no block allocates; a zero size frees and returns 0; otherwise
/// the block is resized under the memory-manager mutex.
pub fn game_link_realloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let block_null = c.g[S1][0] == 0;
        c.g[A2][0] = c.g[A3][0];
        if block_null {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            GL_ALLOC.call(c, 0x002c_2970)?;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0x0054_0000;
            if c.g[S2][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                GL_FREE.call(c, 0x002c_29b4)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 11;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7500));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003e_5700, 0x002c_298c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0031_9bd8, 0x002c_2998)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003e_5760, 0x002c_29a4)?;
                c.g[V0][0] = c.g[S1][0];
            }
            c.cycles += 1;
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

/// `void* GameLink_CreateThread(entry, arg, stackSize)` (0x2c29d0): a thread
/// block (`[0]` the stack, `+4` the THREAD_createparam block), returned when
/// both allocations and the thread creation succeed, else freed and 0.
pub fn game_link_create_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xc;
        c.g[A2][0] = 0;
        GL_ALLOC.call(c, 0x002c_2a04)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0048_0000;
        Addr(r32(c, S0)).sw(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6088));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        GL_ALLOC.call(c, 0x002c_2a20)?;
        let stack_null = c.g[V0][0] == 0;
        let stack = c.g[V0][0];
        Addr(r32(c, S0)).sw(c, stack);
        if stack_null {
            // break to the shared tail below
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        } else {
            c.cycles += 9;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x4));
            c.g[T1][0] = 0x63;
            call_ptr(c, 0x003e_50c8, 0x002c_2a44)?;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            } else {
                c.cycles += 3;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                GL_FREE.call(c, 0x002c_2a54)?;
                Addr(r32(c, S0)).sw(c, 0);
                c.cycles += 1;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            }
        }
        // Shared tail: the thread is valid when its stack pointer is set.
        c.cycles += 2;
        let valid = c.g[A0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if !valid {
            c.cycles += 2;
            let thread_null = c.g[S0][0] == 0;
            c.g[V0][0] = 0;
            if !thread_null {
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 2;
                    GL_FREE.call(c, 0x002c_2a7c)?;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                GL_FREE.call(c, 0x002c_2a84)?;
                c.g[V0][0] = 0;
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

/// `void GameLink_DestroyThread(void** self)` (0x2c2aa8): frees the stack
/// (`*self`) and then the thread block itself, clearing both pointers.
pub fn game_link_destroy_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            GL_FREE.call(c, 0x002c_2acc)?;
            Addr(r32(c, S0)).sw(c, 0);
        }
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                GL_FREE.call(c, 0x002c_2aec)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            GL_FREE.call(c, 0x002c_2af4)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameLink_Yield()` (0x2c2b08): `THREAD_yield()`.
pub fn game_link_yield(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003e_5398), 0x002c_2b18)
}

/// `void* GameLink_CreateMutex()` (0x2c2b28): a 0x10-byte block, initialised
/// as a mutex, returned.
pub fn game_link_create_mutex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[A0][0] = 0x004a_0000;
        c.g[A1][0] = 0x10;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x3898));
        GL_ALLOC.call(c, 0x002c_2b48)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5698, 0x002c_2b54)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameLink_DestroyMutex(void* mutex)` (0x2c2b68): destroys the mutex,
/// then frees its block.
pub fn game_link_destroy_mutex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_56e0, 0x002c_2b7c)?;
        c.g[A0][0] = c.g[S0][0];
        GL_FREE.call(c, 0x002c_2b84)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameLink_LockMutex(void* mutex)` (0x2c2b98): `MUTEX_lock(mutex)`.
pub fn game_link_lock_mutex(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003e_5700), 0x002c_2ba8)
}

/// `void GameLink_UnlockMutex(void* mutex)` (0x2c2bb8): `MUTEX_unlock(mutex)`.
pub fn game_link_unlock_mutex(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003e_5760), 0x002c_2bc8)
}

/// `void GameLink_SetupPlatformFuncs(GameLinkFuncs* t)` (0x2c2bd8): fills the
/// platform function table at `t` (allocator, thread, mutex and string
/// entries). Leaf: no calls.
pub fn game_link_setup_platform_funcs(c: &mut Cpu) {
    c.cycles += 85;
    let frame = Frame::open(c, 0x80);
    frame.save(c, 16, 0x70);
    frame.save(c, 17, 0x60);
    frame.save(c, 18, 0x50);
    frame.save(c, 19, 0x40);
    frame.save(c, 20, 0x30);
    frame.save(c, 21, 0x20);
    frame.save(c, 22, 0x10);
    frame.save(c, 23, 0x00);
    let t = r32(c, A0);
    Addr(t.wrapping_add(0xc)).sw(c, 0x0041_0000 + 0x65a8);
    Addr(t.wrapping_add(0x8)).sw(c, 0x0041_0000 + 0x66f4);
    Addr(t.wrapping_add(0x4)).sw(c, 0x0041_0000 + 0x6810);
    Addr(t.wrapping_add(0x20)).sw(c, 0x002c_0000 + 0x29d0);
    Addr(t.wrapping_add(0x24)).sw(c, 0x002c_0000 + 0x2aa8);
    Addr(t.wrapping_add(0x28)).sw(c, 0x002c_0000 + 0x2b08);
    Addr(t.wrapping_add(0x2c)).sw(c, 0x002c_0000 + 0x2b28);
    Addr(t.wrapping_add(0x30)).sw(c, 0x002c_0000 + 0x2b68);
    c.g[V1][0] = 0x002c_0000 + 0x28c0;
    c.g[A1][0] = 0x002c_0000 + 0x2940;
    c.g[A2][0] = 0x002c_0000 + 0x28f8;
    c.g[A3][0] = 0x0041_0000 + 0x6210;
    c.g[T0][0] = 0x0041_0000 + 0x610c;
    c.g[T1][0] = 0x0041_0000 + 0x5fc8;
    c.g[T2][0] = 0x0041_0000 + 0x605c;
    c.g[T3][0] = 0x002c_0000 + 0x28a0;
    c.g[T4][0] = 0x0042_0000 - 0x7860;
    c.g[T5][0] = 0x0042_0000 - 0x7840;
    c.g[T6][0] = 0x0042_0000 - 0x7818;
    c.g[T7][0] = 0x0042_0000 - 0x7108;
    c.g[T8][0] = 0x002c_0000 + 0x2b98;
    c.g[V0][0] = 0x002c_0000 + 0x2bb8;
    frame.restore(c, 16, 0x70);
    frame.restore(c, 17, 0x60);
    frame.restore(c, 18, 0x50);
    frame.restore(c, 19, 0x40);
    frame.restore(c, 20, 0x30);
    frame.restore(c, 21, 0x20);
    frame.restore(c, 22, 0x10);
    frame.restore(c, 23, 0x00);
    let (v0, v1, a1, a2, a3) = (c.g[V0][0], c.g[V1][0], c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    let (t0, t1, t2, t3) = (c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]);
    let (t4, t5, t6, t7, t8) = (c.g[T4][0], c.g[T5][0], c.g[T6][0], c.g[T7][0], c.g[T8][0]);
    Addr(t.wrapping_add(0x3c)).sw(c, v1);
    Addr(t.wrapping_add(0x38)).sw(c, v0);
    Addr(t.wrapping_add(0x40)).sw(c, a1);
    Addr(t.wrapping_add(0x44)).sw(c, a2);
    Addr(t.wrapping_add(0x48)).sw(c, a3);
    Addr(t.wrapping_add(0x50)).sw(c, t0);
    Addr(t.wrapping_add(0x54)).sw(c, t1);
    Addr(t.wrapping_add(0x4c)).sw(c, t2);
    Addr(t.wrapping_add(0x58)).sw(c, t3);
    Addr(t.wrapping_add(0x14)).sw(c, t4);
    Addr(t.wrapping_add(0x10)).sw(c, t5);
    Addr(t.wrapping_add(0x1c)).sw(c, t6);
    Addr(t.wrapping_add(0x18)).sw(c, t7);
    Addr(t.wrapping_add(0x34)).sw(c, t8);
    frame.close(c);
}

/// Shared body of `WScriptProcess_v07`..`v13`: when the script slot at `field`
/// of the process is set, `WScriptMan_CallProcessFunc(manager, self, slot)`.
fn wscript_process_call_if_set(c: &mut Cpu, field: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A1);
        c.g[A2][0] = Addr(this.wrapping_add(field)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this.wrapping_add(0x10)).lw(c);
            WS_CALL_PROCESS.call(c, ret_addr)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess_v07` (0x307a60): the event script at +0x34 (`EventScriptCode[0]`).
pub fn wscript_process_v07(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x34, 0x0030_7a80)
}

/// `WScriptProcess_v08` (0x307a90): the slot at +0x38.
pub fn wscript_process_v08(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x38, 0x0030_7ab0)
}

/// `WScriptProcess_v09` (0x307ac0): the slot at +0x3c.
pub fn wscript_process_v09(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x3c, 0x0030_7ae0)
}

/// `WScriptProcess_v10` (0x307af0): the slot at +0x40.
pub fn wscript_process_v10(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x40, 0x0030_7b10)
}

/// `WScriptProcess_v11` (0x307b20): the slot at +0x44.
pub fn wscript_process_v11(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x44, 0x0030_7b40)
}

/// `WScriptProcess_v12` (0x307b50): the slot at +0x48.
pub fn wscript_process_v12(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x48, 0x0030_7b70)
}

/// `WScriptProcess_v13` (0x307b80): the slot at +0x4c.
pub fn wscript_process_v13(c: &mut Cpu) {
    wscript_process_call_if_set(c, 0x4c, 0x0030_7ba0)
}

/// `cWScriptProcess::cWScriptProcess(self, manager, sectionId, def)`
/// (0x307598): the process fields, its script codes (three process scripts at
/// +0x28, ten event scripts at +0x34), the task array (`NumTasks` tasks of
/// 0x44 bytes, initialised from `def->unk_008`).
pub fn wscript_process_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x0049_0000 - 0x6490;
        frame.save(c, 18, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x70);
        frame.save(c, 17, 0x60);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 19, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 21, 0x20);
        c.g[V1][0] = sx(r32(c, S2).wrapping_add(0x28));
        frame.save(c, 20, 0x30);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, 22, 0x10);
        c.g[A0][0] = c.g[V1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        let vtable = c.g[V0][0];
        Addr(r32(c, S2).wrapping_add(0x5c)).sw(c, vtable);
        c.g[A2][0] = 0xc;
        c.g[S3][0] = c.g[V1][0];
        call_ptr(c, 0x003e_6448, 0x0030_75f0)?;
        let section = c.g[S0][0];
        Addr(r32(c, S2).wrapping_add(0x24)).sw(c, section);
        let man = c.g[S1][0];
        Addr(r32(c, S2).wrapping_add(0x10)).sw(c, man);
        c.g[S0][0] = sx(r32(c, S5).wrapping_add(0xc));
        c.g[S1][0] = 2;
        c.g[V0][0] = Addr(r32(c, S5)).lw(c);
        Addr(r32(c, S2).wrapping_add(0x1c)).sw(c, 0);
        let process_id = c.g[V0][0];
        Addr(r32(c, S2).wrapping_add(0x20)).sw(c, process_id);

        // ProcessScriptCode[i] = WScriptMan_LookupFunc(manager, ProcessScriptId[i]), i < 3.
        loop {
            c.cycles += 8;
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x10)).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            WS_LOOKUP_FUNC.call(c, 0x0030_7624)?;
            let code = c.g[V0][0];
            Addr(r32(c, S3)).sw(c, code);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_7610);
            }
        }

        // EventScriptCode[i] = WScriptMan_LookupFunc(manager, EventScriptId[i]), i < 10.
        c.cycles += 4;
        c.g[S6][0] = sx(r32(c, S2).wrapping_add(0x14));
        c.g[S1][0] = sx(r32(c, S2).wrapping_add(0x34));
        c.g[S0][0] = sx(r32(c, S5).wrapping_add(0x18));
        c.g[S3][0] = 9;
        loop {
            c.cycles += 8;
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x10)).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            WS_LOOKUP_FUNC.call(c, 0x0030_7654)?;
            let code = c.g[V0][0];
            Addr(r32(c, S1)).sw(c, code);
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_7640);
            }
        }

        // The task array: NumTasks entries of 0x44 bytes with a count header.
        c.cycles += 15;
        c.g[S0][0] = Addr(r32(c, S5).wrapping_add(4)).lw(c);
        c.g[A0][0] = 0x44;
        c.g[A1][0] = 0x0049_0000 - 0x67b0;
        c.g[A2][0] = 0x2000_0000;
        c.mult(0, 4, 16, 4);
        let num_tasks = c.g[S0][0];
        Addr(r32(c, S2).wrapping_add(4)).sw(c, num_tasks);
        c.g[A3][0] = 0;
        c.g[S3][0] = sx(r32(c, S0).wrapping_sub(1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        WS_NEW_NAMED.call(c, 0x0030_768c)?;
        c.g[S4][0] = sx(r32(c, V0).wrapping_add(0x10));
        let count = c.g[S0][0];
        Addr(r32(c, V0)).sw(c, count);
        let no_tasks = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[S4][0];
        if !no_tasks {
            c.cycles += 1;
            c.g[S0][0] = u64::MAX;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
                WS_TASK_CTOR.call(c, 0x0030_76ac)?;
                let more = c.g[S3][0] != c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_76a0);
                }
            }
        }

        // Initialise each task from def->unk_008 (0x38 bytes further per task).
        c.cycles += 5;
        let tasks = c.g[S4][0];
        Addr(r32(c, S6)).sw(c, tasks);
        c.g[S1][0] = 0;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
        let no_init = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = Addr(r32(c, S5).wrapping_add(8)).lw(c);
        if !no_init {
            c.cycles += 2;
            c.g[S3][0] = 0;
            loop {
                c.cycles += 11;
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x14)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x38));
                WS_TASK_INIT.call(c, 0x0030_76ec)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_76d0);
                }
            }
        }

        c.cycles += 15;
        c.g[V1][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
        c.g[V0][0] = c.g[S2][0];
        Addr(r32(c, S2)).sw(c, 0);
        let running = c.g[V1][0];
        Addr(r32(c, S2).wrapping_add(8)).sw(c, running);
        Addr(r32(c, S2).wrapping_add(0x18)).sw(c, 0);
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, Rng, DATA};

    /// Gate word in the machine's RAM: the callee fakes below run only when it
    /// is set (otherwise they run the callee they replaced).
    const OC555_CTRL: u32 = DATA + 0x20_0000;
    const OC555_MAGIC: u32 = 0x0c55_5a5a;
    /// The fake-call index whose result is forced to 0 (u32::MAX: none).
    const OC555_ZERO_AT: u32 = DATA + 0x20_0010;
    /// Fixtures: the process, its manager and the definition block.
    const OC555_SELF: u32 = DATA + 0x21_0000;
    const OC555_MAN: u32 = DATA + 0x22_0000;
    const OC555_DEF: u32 = DATA + 0x23_0000;
    const OC555_ED: u32 = DATA + 0x24_0000;

    /// A fake callee: the call is logged, its result is forced to 0 at the
    /// chosen index, and it returns to `ra` (or unwinds at the chosen index).
    fn oc555_call_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc == ra && c.bus.read32(OC555_ZERO_AT) == n {
            c.g[V0][0] = 0;
        }
    }

    /// One gated fake per callee slot: `name shard slot generated address`.
    macro_rules! oc555_fakes {
        ($($name:ident $shard:ident $slot:ident $gen:ident $addr:literal;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC555_CTRL) != OC555_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    oc555_call_fake(c, $addr)
                }
            )*
            fn oc555_install_fakes() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $name,
                        fakes::prev_cell($name),
                    );
                )*
            }
        };
    }

    oc555_fakes! {
        oc555_f_mem_free s06 OVERRIDE_002523A8 f_002523a8__generated 0x0025_23a8;
        oc555_f_mem_copy s14 OVERRIDE_003E6574 f_003e6574__generated 0x003e_6574;
        oc555_f_mem_set s14 OVERRIDE_003E6448 f_003e6448__generated 0x003e_6448;
        oc555_f_thread_create s14 OVERRIDE_003E50C8 f_003e50c8__generated 0x003e_50c8;
        oc555_f_thread_yield s14 OVERRIDE_003E5398 f_003e5398__generated 0x003e_5398;
        oc555_f_mutex_lock s14 OVERRIDE_003E5700 f_003e5700__generated 0x003e_5700;
        oc555_f_mutex_unlock s14 OVERRIDE_003E5760 f_003e5760__generated 0x003e_5760;
        oc555_f_resize s10 OVERRIDE_00319BD8 f_00319bd8__generated 0x0031_9bd8;
        oc555_f_new_named s10 OVERRIDE_00317E30 f_00317e30__generated 0x0031_7e30;
        oc555_f_ea_alloc s10 OVERRIDE_00317D70 f_00317d70__generated 0x0031_7d70;
        oc555_f_task_ctor s10 OVERRIDE_00308228 f_00308228__generated 0x0030_8228;
        oc555_f_task_init s10 OVERRIDE_00308278 f_00308278__generated 0x0030_8278;
        oc555_f_lookup_func s10 OVERRIDE_00309B70 f_00309b70__generated 0x0030_9b70;
        oc555_f_call_226600 s05 OVERRIDE_00226600 f_00226600__generated 0x0022_6600;
        oc555_f_call_2224a8 s05 OVERRIDE_002224A8 f_002224a8__generated 0x0022_24a8;
        oc555_f_call_2224b8 s05 OVERRIDE_002224B8 f_002224b8__generated 0x0022_24b8;
        oc555_f_call_224da0 s05 OVERRIDE_00224DA0 f_00224da0__generated 0x0022_4da0;
        oc555_f_call_224df0 s05 OVERRIDE_00224DF0 f_00224df0__generated 0x0022_4df0;
        oc555_f_call_225b90 s05 OVERRIDE_00225B90 f_00225b90__generated 0x0022_5b90;
    }

    /// Hooks the fakes in the callees' override slots (once per process).
    fn oc555_setup_fakes() {
        oc555_install_fakes();
    }

    /// Fake machine state for one seed: scrambled registers, the fake hooks,
    /// the gate on, and the forced-zero call index.
    fn oc555_gate(c: &mut Cpu, seed: u32, salt: u64, zero_at: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, 4);
        c.bus.write32(OC555_CTRL, OC555_MAGIC);
        c.bus.write32(OC555_ZERO_AT, zero_at);
        r
    }

    #[test]
    fn game_link_free_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c28f8__generated,
            game_link_free,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c28f8, u32::MAX);
                c.pc = 0x002c_28f8;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_mem_copy_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2918__generated,
            game_link_mem_copy,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2918, u32::MAX);
                c.pc = 0x002c_2918;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_realloc_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2940__generated,
            game_link_realloc,
            300,
            &|c, seed| {
                let zero_at = [u32::MAX, 0, 1, 2][(seed % 4) as usize];
                let mut r = oc555_gate(c, seed, 0x2c2940, zero_at);
                c.pc = 0x002c_2940;
                // block: null on a third of the seeds; size: zero on a quarter.
                let block = if seed % 3 == 0 { 0 } else { fakes::ptr(&mut r) };
                let size = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x1000
                };
                c.g[A0][0] = sx(block);
                c.g[A2][0] = sx(size);
            },
        );
    }

    #[test]
    fn game_link_create_thread_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c29d0__generated,
            game_link_create_thread,
            300,
            &|c, seed| {
                // 0: the thread block is null, 1: the stack is null,
                // 2: THREAD_createparam fails, otherwise nothing fails.
                let zero_at = [u32::MAX, 0, 1, 2][(seed % 4) as usize];
                let mut r = oc555_gate(c, seed, 0x2c29d0, zero_at);
                c.pc = 0x002c_29d0;
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(1 + r.next_u32() % 0x1000);
            },
        );
    }

    #[test]
    fn game_link_destroy_thread_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2aa8__generated,
            game_link_destroy_thread,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2aa8, u32::MAX);
                c.pc = 0x002c_2aa8;
                let this = if seed % 7 == 0 { 0 } else { OC555_SELF };
                // *self: the stack pointer, set or clear.
                let stack = if seed % 3 == 0 { 0 } else { fakes::ptr(&mut r) };
                if this != 0 {
                    c.bus.write32(this, stack);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn game_link_yield_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b08__generated,
            game_link_yield,
            100,
            &|c, seed| {
                let _r = oc555_gate(c, seed, 0x2c2b08, u32::MAX);
                c.pc = 0x002c_2b08;
            },
        );
    }

    #[test]
    fn game_link_create_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b28__generated,
            game_link_create_mutex,
            200,
            &|c, seed| {
                // 0: the mutex block is null (MUTEX_create runs on address 0x10).
                let zero_at = if seed % 4 == 0 { 0 } else { u32::MAX };
                let _r = oc555_gate(c, seed, 0x2c2b28, zero_at);
                c.pc = 0x002c_2b28;
            },
        );
    }

    #[test]
    fn game_link_destroy_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b68__generated,
            game_link_destroy_mutex,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2b68, u32::MAX);
                c.pc = 0x002c_2b68;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_lock_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b98__generated,
            game_link_lock_mutex,
            100,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2b98, u32::MAX);
                c.pc = 0x002c_2b98;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_unlock_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2bb8__generated,
            game_link_unlock_mutex,
            100,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2bb8, u32::MAX);
                c.pc = 0x002c_2bb8;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_setup_platform_funcs_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2bd8__generated,
            game_link_setup_platform_funcs,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2c2bd8, 0);
                c.pc = 0x002c_2bd8;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    /// The process fixture: self, manager and definition with random script
    /// ids, `NumTasks` 0..3 and a random task-init block.
    fn wscript_process_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fill(c, r, OC555_SELF, 0x20);
        fill(c, r, OC555_MAN, 0xc0);
        fill(c, r, OC555_DEF, 0x20);
        fill(c, r, OC555_ED, 0x40);
        c.bus.write32(OC555_DEF, r.next_u32());
        c.bus.write32(OC555_DEF + 4, seed % 4);
        c.bus.write32(OC555_DEF + 8, OC555_ED);
        c.bus.write32(OC555_SELF + 0x10, OC555_MAN);
    }

    #[test]
    fn wscript_process_ctor_matches() {
        oc555_setup_fakes();
        check(
            ee::s10::f_00307598__generated,
            wscript_process_ctor,
            300,
            &|c, seed| {
                // 14: the task array allocation fails (its header is then at 0).
                let zero_at = if seed % 5 == 1 { 14 } else { u32::MAX };
                let mut r = oc555_gate(c, seed, 0x307598, zero_at);
                c.pc = 0x0030_7598;
                wscript_process_fixture(c, &mut r, seed);
                c.g[A0][0] = sx(OC555_SELF);
                c.g[A1][0] = sx(OC555_MAN);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(OC555_DEF);
            },
        );
    }

    /// The v07..v13 fixture: a process whose slot at `field` is set on two
    /// seeds out of three; the manager's fields are random (the callee reads
    /// only what it writes first).
    fn wscript_check(generated: fn(&mut Cpu), hand: fn(&mut Cpu), salt: u64, field: u32) {
        oc555_setup_fakes();
        check(generated, hand, 200, &|c, seed| {
            let mut r = oc555_gate(c, seed, salt, u32::MAX);
            fill(c, &mut r, OC555_SELF, 0x20);
            fill(c, &mut r, OC555_MAN, 0xc0);
            c.bus.write32(OC555_SELF + 0x10, OC555_MAN);
            let slot = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(OC555_SELF + field, slot);
            c.g[A0][0] = sx(OC555_SELF);
        });
    }

    #[test]
    fn wscript_process_v07_matches() {
        wscript_check(
            ee::s10::f_00307a60__generated,
            wscript_process_v07,
            0x307a60,
            0x34,
        );
    }

    #[test]
    fn wscript_process_v08_matches() {
        wscript_check(
            ee::s10::f_00307a90__generated,
            wscript_process_v08,
            0x307a90,
            0x38,
        );
    }

    #[test]
    fn wscript_process_v09_matches() {
        wscript_check(
            ee::s10::f_00307ac0__generated,
            wscript_process_v09,
            0x307ac0,
            0x3c,
        );
    }

    #[test]
    fn wscript_process_v10_matches() {
        wscript_check(
            ee::s10::f_00307af0__generated,
            wscript_process_v10,
            0x307af0,
            0x40,
        );
    }

    #[test]
    fn wscript_process_v11_matches() {
        wscript_check(
            ee::s10::f_00307b20__generated,
            wscript_process_v11,
            0x307b20,
            0x44,
        );
    }

    #[test]
    fn wscript_process_v12_matches() {
        wscript_check(
            ee::s10::f_00307b50__generated,
            wscript_process_v12,
            0x307b50,
            0x48,
        );
    }

    #[test]
    fn wscript_process_v13_matches() {
        wscript_check(
            ee::s10::f_00307b80__generated,
            wscript_process_v13,
            0x307b80,
            0x4c,
        );
    }
}
