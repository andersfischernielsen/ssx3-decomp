//! Staging module oc1142 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1142`.

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
    (0x002f_2c30, c_splash_manager_render_drop, "cSplashManager_RenderDrop"),
    (0x002f_2d60, c_splash_manager_init_field4, "cSplashManager_InitField4"),
    (0x002f_3418, c_splash_manager_render_crystal, "cSplashManager_RenderCrystal"),
    (0x002f_3640, c_splash_manager_spawn_crystals, "cSplashManager_SpawnCrystals"),
    (0x002f_37a8, c_splash_manager_add_drop, "cSplashManager_AddDrop"),
    (0x003b_5320, snd_shutdown, "SND_Shutdown"),
    (0x003b_5c40, snd_mem_check_free, "SND_MemCheckFree"),
    (0x003b_5f60, snd_get_output_latency, "SND_GetOutputLatency"),
];

/// Direct callees of the splash manager ports (generated aliases).
const INIT_DROP: Callee = Callee {
    addr: 0x002f_2598,
    f: ee::s09::f_002f2598,
};
const INIT_CRYSTAL: Callee = Callee {
    addr: 0x002f_2d70,
    f: ee::s09::f_002f2d70,
};

/// Direct callees of the sound shutdown and its helpers (all in `s13`).
const SND_IS_INITIALIZED: Callee = Callee {
    addr: 0x003b_5440,
    f: ee::s13::f_003b5440,
};
const SND_SET_FX: Callee = Callee {
    addr: 0x003b_82b8,
    f: ee::s13::f_003b82b8,
};
const SND_FREE_PAN_TABLES: Callee = Callee {
    addr: 0x003b_c530,
    f: ee::s13::f_003bc530,
};
const SND_STOP_ALL: Callee = Callee {
    addr: 0x003b_8a38,
    f: ee::s13::f_003b8a38,
};
const SND_IOP_SHUTDOWN: Callee = Callee {
    addr: 0x003c_2268,
    f: ee::s13::f_003c2268,
};
const SND_LOCK: Callee = Callee {
    addr: 0x003b_58a0,
    f: ee::s13::f_003b58a0,
};
const SND_MEM_FREE: Callee = Callee {
    addr: 0x003b_5e98,
    f: ee::s13::f_003b5e98,
};
const SND_UNLOCK: Callee = Callee {
    addr: 0x003b_58d8,
    f: ee::s13::f_003b58d8,
};
const SND_MUTEX_DELETE: Callee = Callee {
    addr: 0x003c_3358,
    f: ee::s13::f_003c3358,
};
const SND_MEM_CHECK_FREE: Callee = Callee {
    addr: 0x003b_5c40,
    f: ee::s13::f_003b5c40,
};
const SND_MEM_GET_FREE_PERCENT: Callee = Callee {
    addr: 0x003c_6240,
    f: ee::s13::f_003c6240,
};
const SND_LATENCY_IMPL_ZERO: Callee = Callee {
    addr: 0x003c_51e0,
    f: ee::s13::f_003c51e0,
};

/// `gp + off`.
fn splash_gp_at(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_add(off)
}

/// `gp - off` (the globals below gp).
fn splash_gp_below(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

/// `sp + off`.
fn splash_sp_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `s1 + off` for the sound state block (`S1` holds `0x510000 - 0x5718`).
fn snd_s1_at(c: &Cpu, off: u32) -> u32 {
    r32(c, S1).wrapping_add(off)
}

/// `jalr v0` returning to `ret`: a hook slot of the sound state block, the
/// target is the current `v0`.
fn call_slot(c: &mut Cpu, ret: u32) -> Flow {
    let target = r32(c, V0);
    call_ptr(c, target, ret)
}

/// `void cSplashManager_RenderDrop(tDrop* obj, const t2Vector* off)`
/// (0x2f2c30): draws one drop as a sprite at its position plus the view
/// offset. Its colour fades with the life ratio `age / death` (alpha
/// `1 - ratio^2`) and its rotation is lerped over the drop's life. The
/// first calls set the sprite statics (texture origin 0, scale 1.0).
pub fn c_splash_manager_render_drop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x2408));
        c.sd(splash_sp_at(c, 0x30), c.g[RA][0]);
        c.g[T3][0] = c.g[A0][0];
        c.g[T4][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = 1;
            c.sw(splash_gp_at(c, 0x2400), 0);
            c.sw(splash_gp_at(c, 0x2408), c.g[V0][0]);
            c.sw(splash_gp_at(c, 0x2404), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x2418));
        if c.g[V0][0] != 0 {
            c.lwc1(0, r32(c, T3).wrapping_add(0x20));
        } else {
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.g[V0][0] = 1;
            c.sw(splash_gp_at(c, 0x2418), c.g[V0][0]);
            c.swc1(0, splash_gp_at(c, 0x2414));
            c.swc1(0, splash_gp_at(c, 0x2410));
            c.lwc1(0, r32(c, T3).wrapping_add(0x20));
        }
        c.cycles += 55;
        c.g[A2][0] = sx(r32(c, T3).wrapping_add(0xc));
        c.lwc1(1, r32(c, T3).wrapping_add(0x1c));
        c.g[A1][0] = sx(splash_sp_at(c, 0x10));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 5);
        c.g[A3][0] = sx(splash_gp_at(c, 0x2400));
        c.div_s(6, 1, 0);
        c.ldl(2, r32(c, T3).wrapping_add(0x2b));
        c.ldr(2, r32(c, T3).wrapping_add(0x24));
        c.ldl(3, r32(c, T3).wrapping_add(0x33));
        c.ldr(3, r32(c, T3).wrapping_add(0x2c));
        c.sdl(2, splash_sp_at(c, 7));
        c.sdr(2, splash_sp_at(c, 0));
        c.sdl(3, splash_sp_at(c, 0xf));
        c.sdr(3, splash_sp_at(c, 8));
        c.g[A0][0] = c.lw(splash_gp_below(c, 0x854));
        c.g[T0][0] = sx(splash_gp_at(c, 0x2410));
        c.lwc1(2, splash_sp_at(c, 0));
        c.g[T1][0] = c.g[SP][0];
        c.lwc1(3, r32(c, T3).wrapping_add(0x38));
        c.g[T2][0] = 1;
        c.lwc1(12, r32(c, T3).wrapping_add(0x34));
        c.lwc1(4, splash_gp_below(c, 0x3914));
        c.mul_s(0, 6, 6);
        c.mov_s(1, 6);
        c.mul_s(1, 1, 3);
        c.sub_s(6, 5, 0);
        c.add_s(12, 12, 1);
        c.mul_s(2, 2, 6);
        c.swc1(2, splash_sp_at(c, 0));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0xe84));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x1058));
        c.sh(r32(c, V1).wrapping_add(0x10), c.g[V0][0]);
        c.lwc1(3, r32(c, T4).wrapping_add(4));
        c.lwc1(2, r32(c, T3).wrapping_add(4));
        c.lwc1(0, r32(c, T4));
        c.lwc1(1, r32(c, T3).wrapping_add(8));
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x2a90));
        c.add_s(2, 2, 0);
        c.add_s(1, 1, 3);
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x278));
        c.swc1(2, splash_sp_at(c, 0x10));
        c.swc1(1, splash_sp_at(c, 0x14));
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.swc1(5, splash_sp_at(c, 0x1c));
        c.swc1(2, splash_sp_at(c, 0x20));
        c.swc1(1, splash_sp_at(c, 0x24));
        c.sw(splash_sp_at(c, 0x18), 0);
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x27c));
        let target = r32(c, V0);
        c.mul_s(12, 12, 4);
        call_ptr(c, target, 0x002f_2d50)?;
        c.g[RA][0] = c.ld(splash_sp_at(c, 0x30));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `cSplashManager_InitField4` (0x2f2d60): clears the crystal's active
/// flag at +4 and returns the crystal.
pub fn c_splash_manager_init_field4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    let ra = r32(c, RA);
    c.sw(r32(c, A0).wrapping_add(0x4), 0);
    c.pc = ra;
}

/// `void cSplashManager_RenderCrystal(tCrystal* obj, const t2Vector* off)`
/// (0x2f3418): the crystal counterpart of [`c_splash_manager_render_drop`]:
/// the same fade and rotation, with its own fields (age at +0x20, death at
/// +0x24, colour at +0x28, rotation at +0x38) and statics. It draws through
/// the render vtable at `+0x27c` of the draw state.
pub fn c_splash_manager_render_crystal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x2428));
        c.sd(splash_sp_at(c, 0x30), c.g[RA][0]);
        c.g[T3][0] = c.g[A0][0];
        c.g[T4][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = 1;
            c.sw(splash_gp_at(c, 0x2420), 0);
            c.sw(splash_gp_at(c, 0x2428), c.g[V0][0]);
            c.sw(splash_gp_at(c, 0x2424), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x2438));
        if c.g[V0][0] != 0 {
            c.lwc1(1, r32(c, T3).wrapping_add(0x20));
        } else {
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.g[V0][0] = 1;
            c.sw(splash_gp_at(c, 0x2438), c.g[V0][0]);
            c.swc1(0, splash_gp_at(c, 0x2434));
            c.swc1(0, splash_gp_at(c, 0x2430));
            c.lwc1(1, r32(c, T3).wrapping_add(0x20));
        }
        c.cycles += 57;
        c.g[A2][0] = sx(r32(c, T3).wrapping_add(0x10));
        c.lwc1(0, r32(c, T3).wrapping_add(0x24));
        c.g[A1][0] = sx(splash_sp_at(c, 0x10));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 6);
        c.g[A3][0] = sx(splash_gp_at(c, 0x2420));
        c.div_s(0, 1, 0);
        c.ldl(2, r32(c, T3).wrapping_add(0x2f));
        c.ldr(2, r32(c, T3).wrapping_add(0x28));
        c.ldl(3, r32(c, T3).wrapping_add(0x37));
        c.ldr(3, r32(c, T3).wrapping_add(0x30));
        c.sdl(2, splash_sp_at(c, 7));
        c.sdr(2, splash_sp_at(c, 0));
        c.sdl(3, splash_sp_at(c, 0xf));
        c.sdr(3, splash_sp_at(c, 8));
        c.g[A0][0] = c.lw(splash_gp_below(c, 0x854));
        c.g[T0][0] = sx(splash_gp_at(c, 0x2430));
        c.lwc1(1, splash_sp_at(c, 0));
        c.g[T1][0] = c.g[SP][0];
        c.lwc1(7, splash_gp_below(c, 0x3908));
        c.g[T2][0] = 1;
        c.mul_s(0, 0, 0);
        c.sub_s(0, 6, 0);
        c.mul_s(1, 1, 0);
        c.swc1(1, splash_sp_at(c, 0));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0xe84));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x105c));
        c.sh(r32(c, V1).wrapping_add(0x10), c.g[V0][0]);
        c.lwc1(0, r32(c, T3).wrapping_add(0x24));
        c.lwc1(1, r32(c, T3).wrapping_add(0x20));
        c.lwc1(4, r32(c, T3).wrapping_add(0x3c));
        c.div_s(1, 1, 0);
        c.lwc1(5, r32(c, T4).wrapping_add(4));
        c.lwc1(0, r32(c, T3).wrapping_add(8));
        c.lwc1(3, r32(c, T4));
        c.lwc1(2, r32(c, T3).wrapping_add(0xc));
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x2a90));
        c.add_s(0, 0, 3);
        c.lwc1(12, r32(c, T3).wrapping_add(0x38));
        c.add_s(2, 2, 5);
        c.mul_s(1, 1, 4);
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x278));
        c.add_s(12, 12, 1);
        c.swc1(0, splash_sp_at(c, 0x10));
        c.swc1(2, splash_sp_at(c, 0x14));
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.swc1(6, splash_sp_at(c, 0x1c));
        c.swc1(0, splash_sp_at(c, 0x20));
        c.swc1(2, splash_sp_at(c, 0x24));
        c.sw(splash_sp_at(c, 0x18), 0);
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x27c));
        let target = r32(c, V0);
        c.mul_s(12, 12, 7);
        call_ptr(c, target, 0x002f_3540)?;
        c.g[RA][0] = c.ld(splash_sp_at(c, 0x30));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void cSplashManager_SpawnCrystals(cSplashManager* this, const
/// t2Vector* p, int n)` (0x2f3640): up to `n` crystals around `p`, each
/// jittered by `randRange` in x and y and initialised by
/// `cSplashManager_InitCrystal`, while the count is under the limit at
/// `gp+0x1168`. The loop has a back-edge event check.
pub fn c_splash_manager_spawn_crystals(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0xd0));
        c.sq(16, splash_sp_at(c, 0xb0));
        c.sq(17, splash_sp_at(c, 0xa0));
        c.g[S0][0] = 0;
        c.sq(21, splash_sp_at(c, 0x60));
        c.g[S1][0] = c.g[A0][0];
        c.sq(22, splash_sp_at(c, 0x50));
        c.g[S5][0] = c.g[A1][0];
        c.sq(18, splash_sp_at(c, 0x90));
        c.g[S6][0] = c.g[A2][0];
        c.sq(19, splash_sp_at(c, 0x80));
        c.sq(20, splash_sp_at(c, 0x70));
        c.sq(23, splash_sp_at(c, 0x40));
        c.sq(30, splash_sp_at(c, 0x30));
        c.sd(splash_sp_at(c, 0x20), c.g[RA][0]);
        let none = (c.g[S6][0] as i64) <= 0;
        c.swc1(20, splash_sp_at(c, 0xc0));
        if !none {
            c.cycles += 11;
            c.g[S4][0] = 0x1_0000;
            c.g[S3][0] = 0xe9_0000;
            c.g[S2][0] = 0x7f_0000;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 20);
            c.g[S4][0] |= 0x8fcd;
            c.g[S3][0] |= 0x507c;
            c.g[S2][0] |= 0xffff;
            c.g[S7][0] = 0x3f80_0000;
            c.g[FP][0] = 0x4c;
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
            loop {
                'spawn: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(splash_gp_at(c, 0x1168));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V1][0] = c.lw(splash_gp_at(c, 0xa0c));
                    if t {
                        break 'spawn;
                    }
                    c.cycles += 41;
                    c.mult(0, 4, 4, FP);
                    c.lwc1(6, r32(c, S5));
                    c.g[A1][0] = sx(splash_sp_at(c, 0x10));
                    c.mult(0, 2, 3, 20);
                    c.lwc1(7, r32(c, S5).wrapping_add(4));
                    c.lwc1(2, splash_gp_at(c, 0x11e0));
                    c.g[A2][0] = c.g[S0][0];
                    c.lwc1(3, splash_gp_at(c, 0x11e4));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x724));
                    c.neg_s(4, 2);
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                    c.neg_s(5, 3);
                    c.g[V1][0] &= c.g[S2][0];
                    c.g[V1][0] |= c.g[S7][0];
                    c.sub_s(2, 2, 4);
                    c.mult(0, 7, 3, 20);
                    c.sw(splash_gp_at(c, 0xa0c), c.g[V1][0]);
                    c.sub_s(3, 3, 5);
                    c.lwc1(1, splash_gp_at(c, 0xa0c));
                    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, S3)));
                    c.sub_s(1, 1, 20);
                    c.g[V0][0] &= c.g[S2][0];
                    c.g[V0][0] |= c.g[S7][0];
                    c.sw(splash_gp_at(c, 0xa0c), c.g[V0][0]);
                    c.mul_s(2, 2, 1);
                    c.lwc1(0, splash_gp_at(c, 0xa0c));
                    c.add_s(4, 4, 2);
                    c.sub_s(0, 0, 20);
                    c.add_s(6, 6, 4);
                    c.swc1(4, splash_sp_at(c, 0));
                    c.mul_s(3, 3, 0);
                    c.swc1(6, splash_sp_at(c, 0x10));
                    c.add_s(5, 5, 3);
                    c.add_s(7, 7, 5);
                    c.swc1(5, splash_sp_at(c, 4));
                    c.swc1(7, splash_sp_at(c, 0x14));
                    INIT_CRYSTAL.call(c, 0x002f_3758)?;
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.sw(r32(c, S1).wrapping_add(0x18), c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_36b0);
                    }
                } else {
                    break;
                }
            }
        }
        c.cycles += 13;
        c.lq(16, splash_sp_at(c, 0xb0));
        c.lq(17, splash_sp_at(c, 0xa0));
        c.lq(18, splash_sp_at(c, 0x90));
        c.lq(19, splash_sp_at(c, 0x80));
        c.lq(20, splash_sp_at(c, 0x70));
        c.lq(21, splash_sp_at(c, 0x60));
        c.lq(22, splash_sp_at(c, 0x50));
        c.lq(23, splash_sp_at(c, 0x40));
        c.lq(30, splash_sp_at(c, 0x30));
        c.g[RA][0] = c.ld(splash_sp_at(c, 0x20));
        c.lwc1(20, splash_sp_at(c, 0xc0));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0xd0));
        c.pc = ra;
        Ok(())
    })
}

/// `void cSplashManager_AddDrop(cSplashManager* this, const tDrop* tmpl)`
/// (0x2f37a8): when the drop count is under its limit (`gp+0x1164`), copies
/// the template's 8 bytes at +0 to the stack, initialises the next drop slot
/// with `cSplashManager_InitDrop` and bumps the count at +0x14.
pub fn c_splash_manager_add_drop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(splash_gp_at(c, 0x1164));
        c.sq(16, splash_sp_at(c, 0x20));
        c.sd(splash_sp_at(c, 0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let full = c.g[V0][0] == 0;
        c.g[A3][0] = c.g[A1][0];
        if !full {
            c.cycles += 13;
            c.g[A0][0] = 0x3c;
            c.g[A1][0] = c.g[SP][0];
            c.mult(0, 4, 3, 4);
            c.ldl(2, r32(c, A3).wrapping_add(7));
            c.ldr(2, r32(c, A3));
            c.sdl(2, splash_sp_at(c, 7));
            c.sdr(2, splash_sp_at(c, 0));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1c));
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            INIT_DROP.call(c, 0x002f_37f4)?;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x14));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(r32(c, S0).wrapping_add(0x14), c.g[V0][0]);
        }
        c.cycles += 4;
        c.lq(16, splash_sp_at(c, 0x20));
        c.g[RA][0] = c.ld(splash_sp_at(c, 0x10));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void SND_Shutdown(void)` (0x3b5320): when the sound system is up, stops
/// it: calls the shutdown hooks in the state block (`s1 = 0x50a8e8`), frees
/// the pan tables, stops all voices, shuts the IOP side down, frees the two
/// buffers and deletes the mutex. Returns -14 when not initialised.
pub fn snd_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
        c.sq(16, splash_sp_at(c, 0x20));
        c.sq(17, splash_sp_at(c, 0x10));
        c.sd(splash_sp_at(c, 0), c.g[RA][0]);
        SND_IS_INITIALIZED.call(c, 0x003b_5338)?;
        let initialized = c.g[V0][0] != 0;
        c.g[A0][0] = 0;
        if initialized {
            snd_shutdown_teardown(c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_fff2;
        }
        c.cycles += 5;
        c.lq(16, splash_sp_at(c, 0x20));
        c.lq(17, splash_sp_at(c, 0x10));
        c.g[RA][0] = c.ld(splash_sp_at(c, 0));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

/// The initialised branch of [`snd_shutdown`]: the hook slots of the state
/// block, then the IOP and buffer teardown.
fn snd_shutdown_teardown(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    c.g[A3][0] = u64::MAX;
    c.g[T0][0] = u64::MAX;
    SND_SET_FX.call(c, 0x003b_535c)?;
    c.g[V1][0] = 0x51_0000;
    c.g[S1][0] = sx(r32(c, V1).wrapping_sub(0x5718));
    c.g[V0][0] = c.lw(snd_s1_at(c, 0x1d8));
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1dc));
    } else {
        c.cycles += 3;
        call_slot(c, 0x003b_5378)?;
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1dc));
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1e0));
    } else {
        c.cycles += 3;
        call_slot(c, 0x003b_538c)?;
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1e0));
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1ec));
    } else {
        c.cycles += 3;
        call_slot(c, 0x003b_53a0)?;
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1ec));
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1e8));
    } else {
        c.cycles += 3;
        call_slot(c, 0x003b_53b4)?;
        c.g[V0][0] = c.lw(snd_s1_at(c, 0x1e8));
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        call_slot(c, 0x003b_53c8)?;
    }
    c.cycles += 7;
    SND_FREE_PAN_TABLES.call(c, 0x003b_53d0)?;
    SND_STOP_ALL.call(c, 0x003b_53d8)?;
    c.g[V0][0] = c.lw(snd_s1_at(c, 0x1e4));
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = u64::MAX;
        call_slot(c, 0x003b_53ec)?;
    }
    c.cycles += 16;
    SND_IOP_SHUTDOWN.call(c, 0x003b_53f4)?;
    SND_LOCK.call(c, 0x003b_53fc)?;
    c.g[A0][0] = c.lw(snd_s1_at(c, 0x1f0));
    SND_MEM_FREE.call(c, 0x003b_5404)?;
    c.g[A0][0] = c.lw(snd_s1_at(c, 0x1f4));
    SND_MEM_FREE.call(c, 0x003b_540c)?;
    SND_UNLOCK.call(c, 0x003b_5414)?;
    SND_MEM_CHECK_FREE.call(c, 0x003b_541c)?;
    c.g[S0][0] = c.g[V0][0];
    c.sb(snd_s1_at(c, 0x174), 0);
    SND_MUTEX_DELETE.call(c, 0x003b_5428)?;
    c.g[V0][0] = c.g[S0][0];
    Ok(())
}

/// `int SND_MemCheckFree(void)` (0x3b5c40): the free percentage of the
/// sound memory, from `SND_MemGetFreePercent`.
pub fn snd_mem_check_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.sd(splash_sp_at(c, 0), c.g[RA][0]);
        SND_MEM_GET_FREE_PERCENT.call(c, 0x003b_5c50)?;
        c.g[RA][0] = c.ld(splash_sp_at(c, 0));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `int SND_GetOutputLatency(void)` (0x3b5f60): 0 when the sound system is
/// not initialised, else the output latency of the (zero-latency) backend.
pub fn snd_get_output_latency(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = 0x51_0000;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = c.lb(r32(c, V0).wrapping_sub(0x55a4));
        let initialized = c.g[V1][0] != 0;
        c.sd(splash_sp_at(c, 0), c.g[RA][0]);
        if initialized {
            c.cycles += 4;
            SND_LATENCY_IMPL_ZERO.call(c, 0x003b_5f7c)?;
            c.g[RA][0] = c.ld(splash_sp_at(c, 0));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[RA][0] = c.ld(splash_sp_at(c, 0));
        }
        c.cycles += 2;
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's gated fakes (a unique address in the
    /// machine's RAM; each test writes the magic into its own machine).
    pub(crate) const OC1142_CTRL: u32 = DATA + 0x20_1140;
    pub(crate) const OC1142_MAGIC: u32 = 0x4f43_1142;

    fn f32_bits(x: f32) -> u32 {
        x.to_bits()
    }

    /// Gated fakes of the direct callees: with the gate on they log the
    /// call like `fakes::fake_call`, else they run the generated body.
    macro_rules! oc1142_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1142_CTRL) != OC1142_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1142_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1142_fakes! {
        (oc1142_fake_init_drop, OC1142_PREV_INIT_DROP, ee::s09::OVERRIDE_002F2598, ee::s09::f_002f2598__generated, 0x002f_2598),
        (oc1142_fake_init_crystal, OC1142_PREV_INIT_CRYSTAL, ee::s09::OVERRIDE_002F2D70, ee::s09::f_002f2d70__generated, 0x002f_2d70),
        (oc1142_fake_set_fx, OC1142_PREV_SET_FX, ee::s13::OVERRIDE_003B82B8, ee::s13::f_003b82b8__generated, 0x003b_82b8),
        (oc1142_fake_free_pan, OC1142_PREV_FREE_PAN, ee::s13::OVERRIDE_003BC530, ee::s13::f_003bc530__generated, 0x003b_c530),
        (oc1142_fake_stop_all, OC1142_PREV_STOP_ALL, ee::s13::OVERRIDE_003B8A38, ee::s13::f_003b8a38__generated, 0x003b_8a38),
        (oc1142_fake_iop_shutdown, OC1142_PREV_IOP_SHUTDOWN, ee::s13::OVERRIDE_003C2268, ee::s13::f_003c2268__generated, 0x003c_2268),
        (oc1142_fake_lock, OC1142_PREV_LOCK, ee::s13::OVERRIDE_003B58A0, ee::s13::f_003b58a0__generated, 0x003b_58a0),
        (oc1142_fake_mem_free, OC1142_PREV_MEM_FREE, ee::s13::OVERRIDE_003B5E98, ee::s13::f_003b5e98__generated, 0x003b_5e98),
        (oc1142_fake_unlock, OC1142_PREV_UNLOCK, ee::s13::OVERRIDE_003B58D8, ee::s13::f_003b58d8__generated, 0x003b_58d8),
        (oc1142_fake_mutex_delete, OC1142_PREV_MUTEX_DELETE, ee::s13::OVERRIDE_003C3358, ee::s13::f_003c3358__generated, 0x003c_3358),
        (oc1142_fake_free_percent, OC1142_PREV_FREE_PERCENT, ee::s13::OVERRIDE_003C6240, ee::s13::f_003c6240__generated, 0x003c_6240),
        (oc1142_fake_latency_zero, OC1142_PREV_LATENCY_ZERO, ee::s13::OVERRIDE_003C51E0, ee::s13::f_003c51e0__generated, 0x003c_51e0),
    }

    /// Common machine setup for the gated fakes of this module.
    fn gated_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1142_CTRL, OC1142_MAGIC);
        r
    }

    #[test]
    fn c_splash_manager_init_field4_matches() {
        check(
            ee::s09::f_002f2d60__generated,
            c_splash_manager_init_field4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d60, 0);
                let crystal = DATA + 0x1000 + 0x40 * (seed % 5);
                c.g[A0][0] = sx(crystal);
                c.bus.write32(crystal + 4, r.next_u32());
            },
        );
    }

    /// The draw state a render function reaches through `gp`: the GrMan
    /// (`gp-0x854`) with its current draw state (`+0xe84`), the draw
    /// state pointer at `gp+0x2a90` and the fake draw vtable (`+0x27c`).
    fn render_setup(c: &mut Cpu, r: &mut Rng, seed: u32, obj: u32, off: u32, flags: [u32; 2]) {
        let (grman, cur, draw, state) = (DATA + 0x2000, DATA + 0x3000, DATA + 0x4000, DATA + 0x5000);
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = sx(off);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), grman);
        c.bus.write32(grman + 0xe84, cur);
        c.bus.write32(grman + 0x1058, r.next_u32());
        c.bus.write32(grman + 0x105c, r.next_u32());
        c.bus.write32(GAME_GP + 0x2a90, draw);
        c.bus.write32(draw + 0x10d8, state);
        c.bus.write16(state + 0x278, r.next_u32() as u16);
        c.bus.write32(state + 0x27c, FAKE + 0x10 * (seed % 3));
        c.bus.write32(GAME_GP.wrapping_sub(0x3914), f32_bits(0.017_453_3));
        c.bus.write32(GAME_GP.wrapping_sub(0x3908), f32_bits(0.5 + (seed % 4) as f32));
        c.bus.write32(GAME_GP + 0x2410, 0);
        c.bus.write32(GAME_GP + 0x2400, 0);
        c.bus.write32(GAME_GP + 0x11e0, r.f32(4.0));
        c.bus.write32(GAME_GP + 0x11e4, f32_bits(1.5));
        c.bus.write32(GAME_GP + 0x2430, 0);
        c.bus.write32(GAME_GP + 0x2420, 0);
        let _ = flags;
    }

    /// A drop (or crystal) object at `obj` with `age`, `death`, a colour
    /// and a position; the view offset at `off`.
    fn write_sprite_fields(c: &mut Cpu, r: &mut Rng, seed: u32, obj: u32, off: u32, layout: [u32; 4]) {
        let [age, death, colour, rot] = layout;
        c.bus.write32(obj + age, f32_bits((seed % 5) as f32 * 0.5));
        c.bus.write32(obj + death, f32_bits(1.0 + (seed % 7) as f32));
        for k in 0..4 {
            c.bus.write32(obj + colour + 4 * k, r.f32(8.0));
        }
        c.bus.write32(obj + 4, r.f32(50.0));
        c.bus.write32(obj + 8, r.f32(50.0));
        c.bus.write32(obj + 0xc, r.f32(4.0));
        c.bus.write32(obj + rot, r.f32(2.0));
        c.bus.write32(obj + rot + 4, r.f32(2.0));
        c.bus.write32(off, r.f32(20.0));
        c.bus.write32(off + 4, r.f32(20.0));
    }

    fn render_drop_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c30, 1);
        c.bus.write32(OC1142_CTRL, OC1142_MAGIC);
        let (obj, off) = (DATA + 0x1000, DATA + 0x1100);
        write_sprite_fields(c, &mut r, seed, obj, off, [0x1c, 0x20, 0x24, 0x34]);
        render_setup(c, &mut r, seed, obj, off, [0, 0]);
        c.bus.write32(GAME_GP + 0x2408, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + 0x2418, if seed % 3 == 0 { 0 } else { 1 });
    }

    fn render_crystal_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3418, 1);
        c.bus.write32(OC1142_CTRL, OC1142_MAGIC);
        let (obj, off) = (DATA + 0x1000, DATA + 0x1100);
        write_sprite_fields(c, &mut r, seed, obj, off, [0x20, 0x24, 0x28, 0x34]);
        c.bus.write32(obj + 0x38, r.f32(2.0));
        c.bus.write32(obj + 0x3c, r.f32(2.0));
        render_setup(c, &mut r, seed, obj, off, [0, 0]);
        c.bus.write32(GAME_GP + 0x2428, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + 0x2438, if seed % 3 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + 0x2430, 0);
    }

    #[test]
    fn c_splash_manager_render_drop_matches() {
        covered(
            ee::s09::f_002f2c30__generated,
            c_splash_manager_render_drop,
            300,
            &|c, seed| render_drop_setup(c, seed),
        );
    }

    #[test]
    fn c_splash_manager_render_crystal_matches() {
        covered(
            ee::s09::f_002f3418__generated,
            c_splash_manager_render_crystal,
            300,
            &|c, seed| render_crystal_setup(c, seed),
        );
    }

    #[test]
    fn c_splash_manager_spawn_crystals_matches() {
        oc1142_install_fakes();
        covered(
            ee::s09::f_002f3640__generated,
            c_splash_manager_spawn_crystals,
            300,
            &|c, seed| {
                let mut r = gated_machine(c, seed, 0x3640, 2);
                let (obj, p) = (DATA + 0x1000, DATA + 0x1100);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(p);
                c.g[A2][0] = (seed % 4) as u64;
                c.bus.write32(obj + 0x18, seed % 3);
                c.bus.write32(GAME_GP + 0x1168, 2 + seed % 5);
                c.bus.write32(GAME_GP + 0xa0c, r.next_u32() & 0x7fff_ffff);
                c.bus.write32(GAME_GP + 0x11e0, r.f32(3.0));
                c.bus.write32(GAME_GP + 0x11e4, r.f32(3.0));
                c.bus.write32(p, r.f32(100.0));
                c.bus.write32(p + 4, r.f32(100.0));
            },
        );
    }

    #[test]
    fn c_splash_manager_add_drop_matches() {
        oc1142_install_fakes();
        covered(
            ee::s09::f_002f37a8__generated,
            c_splash_manager_add_drop,
            200,
            &|c, seed| {
                let mut r = gated_machine(c, seed, 0x37a8, 1);
                let (obj, tmpl) = (DATA + 0x1000, DATA + 0x1100);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(tmpl);
                c.bus.write32(GAME_GP + 0x1164, 2 + seed % 4);
                c.bus.write32(obj + 0x14, seed % 3);
                c.bus.write32(tmpl, r.next_u32());
                c.bus.write32(tmpl + 4, r.next_u32());
            },
        );
    }

    /// The sound state block (`s1`) with its hook slots: `sound_on` sets the
    /// init byte, and each slot is zero or a fake target.
    fn snd_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = gated_machine(c, seed, 0x5320, 3);
        c.bus.write8(0x51_0000 - 0x55a4, (seed % 4 != 0) as u8);
        let s1 = 0x51_0000u32 - 0x5718;
        for (i, off) in [0x1d8u32, 0x1dc, 0x1e0, 0x1ec, 0x1e8, 0x1e4].into_iter().enumerate() {
            let v = if (seed >> i) % 3 == 0 { 0 } else { FAKE + 0x10 * i as u32 };
            c.bus.write32(s1 + off, v);
        }
        c.bus.write32(s1 + 0x1f0, DATA + 0x1000 + (r.next_u32() & 0xff0));
        c.bus.write32(s1 + 0x1f4, DATA + 0x1200);
    }

    #[test]
    fn snd_shutdown_matches() {
        oc1142_install_fakes();
        covered(
            ee::s13::f_003b5320__generated,
            snd_shutdown,
            300,
            &|c, seed| snd_shutdown_setup(c, seed),
        );
    }

    #[test]
    fn snd_mem_check_free_matches() {
        oc1142_install_fakes();
        covered(
            ee::s13::f_003b5c40__generated,
            snd_mem_check_free,
            200,
            &|c, seed| {
                let _ = gated_machine(c, seed, 0x5c40, 1);
            },
        );
    }

    #[test]
    fn snd_get_output_latency_matches() {
        oc1142_install_fakes();
        covered(
            ee::s13::f_003b5f60__generated,
            snd_get_output_latency,
            200,
            &|c, seed| {
                let _ = gated_machine(c, seed, 0x5f60, 1);
                c.bus.write8(0x51_0000 - 0x55a4, (seed % 2) as u8);
            },
        );
    }
}
