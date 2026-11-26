//! Staging module oc820 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc820`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0017_f500,
        debug_menu_module_draw_cursor_trail,
        "DebugMenuModule_DrawCursorTrail",
    ),
    (0x0017_f7b8, debug_menu_module_ctor, "DebugMenuModule_ctor"),
    (0x0017_f838, debug_menu_module_dtor, "DebugMenuModule_dtor"),
    (0x0017_f868, debug_menu_module_init, "DebugMenuModule_Init"),
    (
        0x0017_fab0,
        debug_menu_module_try_exit,
        "DebugMenuModule_TryExit",
    ),
    (0x0017_fe80, feui_input_is_start, "FEUIInput_IsStart"),
];

/// `void DebugMenuModule_DrawCursorTrail(DebugMenuModule* self, short layer)`
/// (0x17f500): sets the 2D renderer's state words (blend and z flags in the
/// word at `+4` of the render state `*(*(gp-0x854)+0xe84)`, `layer` into its
/// halfword at `+0x10`), then draws the 64-point cursor trail stored in the
/// menu (`+0x10 + 8*i` = (x, y), ring head at `+0x210`) as 63 shrinking,
/// fading filled rectangles (callee 0x38ec40, Render2D_DrawFilledRect) and
/// a final box around the newest point. Registers and cycles follow the
/// generated body, so the body is kept instruction by instruction.
pub fn debug_menu_module_draw_cursor_trail(c: &mut Cpu) {
    guest(c, debug_menu_module_draw_cursor_trail_body);
}

fn debug_menu_module_draw_cursor_trail_body(c: &mut Cpu) -> Flow {
    c.cycles += 72;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.g[V1][0] = 0xfffffffffe7f0000;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.g[T3][0] = 0x1000000;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[A2][0] = 0xffffffffffcf0000;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[T4][0] = 0x300000;
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[A3][0] = 0xfffffffffff00000;
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xf8));
    c.g[A3][0] = c.g[A3][0] | 0xfff;
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.g[S0][0] = c.g[A0][0];
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
    c.g[T5][0] = 0xffffffffffffff83;
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.g[T2][0] = 0xffffffffffbf0000;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
    c.g[T2][0] = c.g[T2][0] | 0xffff;
    c.g[AT][0] = 0x40c00000;
    c.mtc1(1, 0);
    c.g[S1][0] = 0x1;
    c.g[T1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 20);
    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[AT][0] = 0x3c800000;
    c.mtc1(1, 23);
    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5a38));
    c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5a34));
    c.sw((c.g[T1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[T3][0] = 0x400000;
    c.g[S6][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[V0][0] = c.g[V0][0] | 0xd000;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[V0][0] = c.g[V0][0] | 0x14;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(0, c.g[SP][0] as u32);
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
    c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
    'l_17f620: loop {
        c.cycles += 53;
        c.mtc1(17, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(4, c.g[SP][0] as u32);
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3f));
        c.g[V1][0] = ((c.g[S6][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A3][0];
        }
        c.mul_s(4, 4, 20);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.mul_s(6, 6, 20);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.mul_s(1, 1, 23);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 3);
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
        c.mul_s(8, 1, 1);
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.mul_s(7, 1, 21);
        c.lwc1(3, c.g[A3][0] as u32);
        c.mul_s(9, 1, 20);
        c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.mul_s(8, 8, 22);
        c.sub_s(5, 3, 4);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sub_s(0, 0, 6);
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5a30));
        c.add_s(3, 3, 4);
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.mul_s(1, 1, 9);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.add_s(7, 7, 2);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
        c.g[A2][0] = c.g[S3][0];
        c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        c.mtc1(0, 12);
        c.g[A3][0] = c.g[S4][0];
        c.add_s(0, 0, 6);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x5c));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
        call_ptr(c, 0x0038ec40, 0x0017f6e8)?;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 64i64) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
            if event_due(c) {
                deliver_event_here(c, 0x0017f620);
            }
            continue 'l_17f620;
        } else {
            c.cycles += 48;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
            c.g[A1][0] = c.g[S2][0];
            c.lwc1(5, c.g[SP][0] as u32);
            c.g[A2][0] = c.g[S5][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
            c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 4);
            c.g[V1][0] = c.g[V0][0];
            c.g[AT][0] = 0x3e800000;
            c.mtc1(1, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = c.g[S3][0];
            c.lwc1(0, c.g[V0][0] as u32);
            c.sub_s(1, 1, 6);
            c.lwc1(2, c.g[V1][0] as u32);
            c.sub_s(0, 0, 5);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
            c.add_s(2, 2, 5);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.mtc1(0, 12);
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.add_s(0, 0, 6);
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x3c));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0038ec40, 0x0017f77c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
            c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
            c.pc = t;
            return Ok(());
        }
    }
}

/// `DebugMenuModule* DebugMenuModule_ctor(DebugMenuModule* self)` (0x17f7b8):
/// runs `AppModule_ctor`, installs the vtable (0x46d900), clears the fields at
/// `+0x24..+0x30` and zero-fills the 0x200-byte cursor trail at `+0x34` with
/// `EA_MemSet` (after a 64-step countdown the compiler left in); returns self.
pub fn debug_menu_module_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0023_1cd0,
            f: ee::s06::AppModule_ctor,
        }
        .call(c, 0x0017_f7cc)?;
        c.g[V0][0] = 0x47_0000;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x24), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2700));
        c.sw(this.wrapping_add(0x28), 0);
        c.sw(this, c.g[V0][0]);
        c.g[V1][0] = sx(this.wrapping_add(0x24));
        c.sw(this.wrapping_add(0x2c), 0);
        c.g[A0][0] = u64::MAX;
        c.sw(this.wrapping_add(0x30), 0);
        c.g[V0][0] = 0x3f;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[A0][0];
            c.g[A1][0] = 0;
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0017_f7f8);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x210), 0);
        c.g[A2][0] = 0x200;
        Callee {
            addr: 0x003e_6448,
            f: ee::s14::EA_MemSet,
        }
        .call(c, 0x0017_f824)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void DebugMenuModule_dtor(DebugMenuModule* self, int flags)` (0x17f838):
/// resets the vtable to the base class one (0x46d970) and, when bit 0 of
/// `flags` is set, frees the object (`__builtin_delete`, 0x317e50).
pub fn debug_menu_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x47_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2690));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let keep = c.g[A1][0] == 0;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::__builtin_delete,
            }
            .call(c, 0x0017_f85c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DebugMenuModule_Init(DebugMenuModule* self)` (0x17f868): flushes
/// the renderer, creates the debug font and loads its two resource files
/// (`+0x18` font, `+0x1c` and `+0x20` texture handles, the loaded buffers are freed
/// again), clears `+0x10`/`+0xc` and builds the DebugMenu (0x8c48 bytes,
/// `+0xc`, `+0x14`) and the LobbyMenu (0x1c18 bytes, `+0x10`); `+0x14` ends up the
/// lobby menu when the world flag at `+0x74` is set, else the debug menu.
/// The body is kept instruction by instruction (registers and cycles).
pub fn debug_menu_module_init(c: &mut Cpu) {
    guest(c, debug_menu_module_init_body);
}

fn debug_menu_module_init_body(c: &mut Cpu) -> Flow {
    'b_17f964: {
        c.cycles += 45;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0017f898)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2b48));
        c.g[A0][0] = 0x84;
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0017f8b0)?;
        c.g[A1][0] = 0x460000;
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2b38));
        call_ptr(c, 0x003915e8, 0x0017f8c4)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a2c));
        c.g[V1][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.g[A0][0] = 0x460000;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2b20));
        c.swc1(0, c.g[SP][0] as u32);
        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(5, c.g[SP][0] as u32);
        c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x37));
        c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0x100;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.ldl(9, (c.g[V0][0] as u32).wrapping_add(0x37));
        c.ldr(9, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.sdl(9, (c.g[V0][0] as u32).wrapping_add(0x3f));
        c.sdr(9, (c.g[V0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        call_ptr(c, 0x003e1908, 0x0017f910)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0u64;
        c.g[A0][0] = 0x460000;
        if t {
            break 'b_17f964;
        } else {
            'b_17f958: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let t = c.g[A1][0] == 0u64;
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                if t {
                    break 'b_17f958;
                } else {
                    c.cycles += 11;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1340));
                    c.g[A3][0] = 0u64;
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                    c.g[T0][0] = 0x1;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017f954)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    break 'b_17f958;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00317e98, 0x0017f960)?;
            c.g[A0][0] = 0x460000;
            break 'b_17f964;
        }
    }
    'b_17f9b8: {
        c.cycles += 19;
        c.g[A1][0] = 0x100;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2b08));
        call_ptr(c, 0x003e18c8, 0x0017f970)?;
        c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x4a0000;
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1348));
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
        c.g[T0][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
        c.g[T1][0] = 0xffffffffffffffff;
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0017f9a8)?;
        let t = c.g[S0][0] == 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        if t {
            break 'b_17f9b8;
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00317e98, 0x0017f9b8)?;
            break 'b_17f9b8;
        }
    }
    c.cycles += 2;
    c.g[V1][0] = 0x1;
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
    'l_17f9c0: loop {
        c.cycles += 7;
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        let t = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0017f9c0);
            }
            continue 'l_17f9c0;
        } else {
            'b_17fa48: {
                'b_17fa44: {
                    c.cycles += 24;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2af0));
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = 0x8c48;
                    call_ptr(c, 0x00317d70, 0x0017f9f4)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0017b1b0, 0x0017f9fc)?;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x1c18;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2ae0));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x0017fa1c)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0017d298, 0x0017fa24)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x74));
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        break 'b_17fa44;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                        if t {
                            break 'b_17fa48;
                        } else {
                            break 'b_17fa44;
                        }
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                break 'b_17fa48;
            }
            c.cycles += 5;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = t;
            return Ok(());
        }
    }
}

/// `bool DebugMenuModule_TryExit(void)` (0x17fab0): `AppModule_TryExit() > 0`.
pub fn debug_menu_module_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0023_1cf0,
            f: ee::s06::AppModule_TryExit,
        }
        .call(c, 0x0017_fac0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    });
}

/// `bool FEUIInput_IsStart(FEUIInput* self)` (0x17fe80): button 0x70 of the
/// binding at `*(self + 0x18)` (`InputBinding_EvalBool`, 0x320c48).
pub fn feui_input_is_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x70;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, 0x0017_fe94)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, LOG_UNWIND_AT};
    use crate::difftest::{check, DATA, GAME_GP};
    use crate::game::camera::camera::tests::{alloc_base, W_2};
    use crate::game::scripter::ssxscriptengine::tests::gp_at;
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    pub(crate) const OC820_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC820_MAGIC: u32 = 0x4f43_8200;

    /// Forced-result mailboxes, one word per fake callee.
    pub(crate) const OC820_MAIL: u32 = DATA + 0x20_1000;

    pub(crate) fn oc820_mail(addr: u32) -> u32 {
        OC820_MAIL + (addr & 0xffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (zero-extended).
    pub(crate) fn oc820_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc820_mail(addr), v.wrapping_add(2));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc820_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc820_mail(addr), 1);
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc820_arm(c: &mut Cpu) {
        c.bus.write32(OC820_CTRL, OC820_MAGIC);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    pub(crate) fn oc820_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc820_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 {
                0
            } else {
                mail.wrapping_sub(2) as u64
            };
        }
    }

    macro_rules! oc820_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC820_CTRL) != OC820_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc820_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc820_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc820_fakes!(
        (
            oc820_fake_17b1b0,
            OC820_PREV_17B1B0,
            ee::s02::OVERRIDE_0017B1B0,
            ee::s02::f_0017b1b0__generated,
            0x0017_b1b0
        ),
        (
            oc820_fake_231cd0,
            OC820_PREV_231CD0,
            ee::s06::OVERRIDE_00231CD0,
            ee::s06::f_00231cd0__generated,
            0x0023_1cd0
        ),
        (
            oc820_fake_231cf0,
            OC820_PREV_231CF0,
            ee::s06::OVERRIDE_00231CF0,
            ee::s06::f_00231cf0__generated,
            0x0023_1cf0
        ),
        (
            oc820_fake_317e50,
            OC820_PREV_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x0031_7e50
        ),
        (
            oc820_fake_317e98,
            OC820_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc820_fake_320c48,
            OC820_PREV_320C48,
            ee::s10::OVERRIDE_00320C48,
            ee::s10::f_00320c48__generated,
            0x0032_0c48
        ),
        (
            oc820_fake_38ec40,
            OC820_PREV_38EC40,
            ee::s12::OVERRIDE_0038EC40,
            ee::s12::f_0038ec40__generated,
            0x0038_ec40
        ),
        (
            oc820_fake_3915e8,
            OC820_PREV_3915E8,
            ee::s12::OVERRIDE_003915E8,
            ee::s12::f_003915e8__generated,
            0x0039_15e8
        ),
        (
            oc820_fake_3e18c8,
            OC820_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc820_fake_3e1908,
            OC820_PREV_3E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
        (
            oc820_fake_3e6448,
            OC820_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    /// `DebugMenuModule_TryExit`: `AppModule_TryExit() > 0` (unsigned compare
    /// of the forced result: zero, small, and values with the top bit set) and
    /// an unwinding callee.
    #[test]
    fn debug_menu_module_try_exit_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017fab0__generated,
            debug_menu_module_try_exit,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x17_fab0, 1);
                oc820_arm(c);
                match seed % 5 {
                    0 => {}
                    1 => oc820_force_zero(c, 0x0023_1cf0),
                    2 => oc820_force(c, 0x0023_1cf0, 5),
                    3 => oc820_force(c, 0x0023_1cf0, 0x8000_0000),
                    _ => oc820_force(c, 0x0023_1cf0, 0xffff_fff0),
                }
            },
        );
    }

    /// `FEUIInput_IsStart`: the button query on `*(input + 0x18)`.
    #[test]
    fn feui_input_is_start_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017fe80__generated,
            feui_input_is_start,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x17_fe80, 1);
                oc820_arm(c);
                c.g[A0][0] = sx(DATA + 0x8_0000);
                c.bus.write32(DATA + 0x8_0000 + 0x18, DATA + 0x8_1000);
                if seed % 3 == 1 {
                    oc820_force_zero(c, 0x0032_0c48);
                } else if seed % 3 == 2 {
                    oc820_force(c, 0x0032_0c48, 1);
                }
            },
        );
    }

    /// `DebugMenuModule_dtor`: base vtable store; the delete only when bit 0
    /// of the flags is set (other bits random), and an unwinding delete.
    #[test]
    fn debug_menu_module_dtor_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f838__generated,
            debug_menu_module_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17_f838, 1);
                oc820_arm(c);
                c.g[A0][0] = sx(DATA + 0x8_0000);
                c.g[A1][0] = (r.next_u32() & !1 | (seed & 1)) as u64;
            },
        );
    }

    /// `DebugMenuModule_ctor`: base constructor, field clears, vtable, the
    /// 0x200-byte fill, and unwinds in either call.
    #[test]
    fn debug_menu_module_ctor_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f7b8__generated,
            debug_menu_module_ctor,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x17_f7b8, 2);
                oc820_arm(c);
                c.g[A0][0] = sx(DATA + 0x8_0000);
            },
        );
    }

    /// `DebugMenuModule_DrawCursorTrail`: the render-state words, the 63 trail
    /// rectangles (head positive and negative, so the ring index takes both
    /// sides of the sign correction) and the final box; unwinds at any of the
    /// 64 draw calls.
    #[test]
    fn debug_menu_module_draw_cursor_trail_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f500__generated,
            debug_menu_module_draw_cursor_trail,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17_f500, 65);
                oc820_arm(c);
                let menu = DATA + 0x8_0000;
                let rend = DATA + 0x8_1000;
                let state = DATA + 0x8_2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), rend);
                c.bus.write32(GAME_GP.wrapping_sub(0x844), DATA + 0x8_3000);
                c.bus.write32(rend + 0xe84, state);
                c.bus.write32(state + 4, r.next_u32());
                c.bus.write32(state + 0x10, r.next_u32());
                let wild = seed % 4 == 0;
                for off in [0x5a38u32, 0x5a34, 0x5a30] {
                    let v = if wild { r.edge_f32() } else { r.f32(4.0) };
                    c.bus.write32(GAME_GP.wrapping_sub(off), v);
                }
                for i in 0..64u32 {
                    for k in 0..2u32 {
                        let v = if wild { r.edge_f32() } else { r.f32(300.0) };
                        c.bus.write32(menu + 0x10 + 8 * i + 4 * k, v);
                    }
                }
                let head = match seed % 6 {
                    0 => 0u32.wrapping_sub(1 + r.next_u32() % 100),
                    1 => 63,
                    _ => r.next_u32() % 200,
                };
                c.bus.write32(menu + 0x210, head);
                c.g[A0][0] = sx(menu);
            },
        );
    }

    /// `DebugMenuModule_Init`: the renderer flush and the two file loads (each
    /// absent, present with a buffer that is freed, or present with a zero
    /// payload pointer), the font setup, the two menus, and the final choice
    /// of the active menu by the world flag. Real pool allocator and real
    /// `LobbyMenu_ctor` (fixture of `lobby_menu_ctor_matches`); the file, font,
    /// free and debug-menu callees are gated fakes.
    #[test]
    fn debug_menu_module_init_matches() {
        oc820_install_fakes();
        check(
            ee::s02::f_0017f868__generated,
            debug_menu_module_init,
            24,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x17_f868);
                // The lobby menu's own fixture.
                let obj = DATA + 0x10_1000;
                c.bus.write32(W_2 + 0x80, obj);
                c.bus.write32(obj + 0x18, seed);
                c.bus.write32(obj + 0x20, seed * 3 + 1);
                c.bus.write32(W_2 + 0xb0, seed ^ 0x55);
                c.bus.write32(gp_at(0xda0), DATA + 0x10_2000);
                oc820_arm(c);
                c.bus.write32(
                    LOG_UNWIND_AT,
                    if seed % 3 == 0 {
                        r.next_u32() % 8
                    } else {
                        u32::MAX
                    },
                );
                let menu = DATA + 0x8_0000;
                let rend = DATA + 0x8_1000;
                let vt = DATA + 0x8_2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), rend);
                c.bus.write32(GAME_GP + 0x2a90, rend);
                c.bus.write32(rend + 0x10d8, vt);
                for slot in [0x160u32, 0x170] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x1c) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + slot);
                }
                // The font and the debug menu live outside the allocator.
                oc820_force(c, 0x0039_15e8, DATA + 0x8_5000);
                oc820_force(c, 0x0017_b1b0, DATA + 0x8_6000);
                // The two file loads: absent, present, present with a
                // buffer whose payload pointer cancels to zero.
                let (p1, p2) = (DATA + 0x8_4000, DATA + 0x8_4400);
                match seed % 4 {
                    0 => {
                        oc820_force_zero(c, 0x003e_1908);
                        oc820_force(c, 0x003e_18c8, p2);
                        c.bus.write32(p2 + 0x14, 0x40);
                    }
                    1 => {
                        oc820_force(c, 0x003e_1908, p1);
                        c.bus.write32(p1 + 0x14, 0x40);
                        oc820_force_zero(c, 0x003e_18c8);
                    }
                    2 => {
                        oc820_force(c, 0x003e_1908, p1);
                        c.bus.write32(p1 + 0x14, 0u32.wrapping_sub(p1));
                        oc820_force(c, 0x003e_18c8, p2);
                        c.bus.write32(p2 + 0x14, 0u32.wrapping_sub(p2));
                    }
                    _ => {
                        oc820_force(c, 0x003e_1908, p1);
                        c.bus.write32(p1 + 0x14, 0x40);
                        oc820_force(c, 0x003e_18c8, p2);
                        c.bus.write32(p2 + 0x14, 0x40);
                    }
                }
                // The world flag picks the active menu.
                c.bus.write32(W_2 + 0x74, (seed / 4) % 2);
                c.g[A0][0] = sx(menu);
            },
        );
    }
}
