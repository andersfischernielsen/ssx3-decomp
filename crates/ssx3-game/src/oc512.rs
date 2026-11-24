//! Staging module oc512 (wave wave229): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc512`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0022_fa98, game_module_unload, "GameModule_Unload")];

/// The `GP`-relative address `gp + off`.
#[inline(always)]
fn gp_at(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add_signed(off)
}

/// The `off(s0)` address of the game object, read from the callee-saved
/// register each time (a callee may not change it, but the port does not
/// keep a copy across calls).
#[inline(always)]
fn s0_at(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// The virtual deleting call of `VDelete` on the object in `a2`: its vtable
/// pointer sits at `obj + vt_at`, the `this` adjustment at vtable+8 and the
/// deleting entry at vtable+0xc (`a1` is set by the caller).
fn vdelete(c: &mut Cpu, vt_at: u32, ret: u32) -> Flow {
    let obj = c.g[A2][0] as u32;
    c.g[V0][0] = c.lw(obj.wrapping_add(vt_at));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `void GameModule_Unload(GameModule* self)` (0x22fa98; the reference names
/// it `cGame_purge`): tear the race down. Shuts the back end's network stats
/// down (copying the two riders' stats into the network record, then the
/// interface's virtual `+0xc` entry), purges the managers and renderers,
/// deletes the owned objects (`if (self->X) X->dtor(3)` for each field, with
/// the virtual deletes for the vtable-owned ones), clears `SSXApp->Game` and
/// the global purge flags.
pub fn game_module_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0x4d_0000;
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(gp_at(c, 0x2898));
        c.g[S3][0] = 0x4c_0000;
        // cBE_setState(0)
        call_ptr(c, 0x0014_de68, 0x0022_facc)?;
        c.g[S4][0] = 0x50_0000;
        c.g[V0][0] = c.lw(gp_at(c, -0x848));
        c.g[A1][0] = 7;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        // cBE_getInterface(BE, 7): the interface's vtable at +0xc
        call_ptr(c, 0x0014_dd58, 0x0022_fae0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[T1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(0x0053_4b30);
        let t = c.g[V0][0] == 0;
        c.g[A0][0] = sx(0x0053_4b30);
        'network: {
            if t {
                break 'network;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp_at(c, -0x848));
            if t {
                break 'network;
            }
            // The copies of both riders' stats (networkOpt+0x330 is MyStats,
            // +0x3dc OpponentStats; 0xac bytes each, the first 0xa0 in a loop).
            'copies: {
                'my_stats: {
                    c.cycles += 16;
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x330));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = (0u64 < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = (c.g[V1][0] < 1u64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x790));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xfc));
                    c.g[V0][0] = c.g[A1][0] | c.g[A3][0];
                    c.g[V0][0] &= 7;
                    let t = c.g[V0][0] == 0;
                    c.g[T0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        break 'my_stats;
                    }
                    // Unaligned: 32 bytes per step with ldl/ldr and sdl/sdr.
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19c));
                    c.g[V0][0] = 0x53_0000;
                    'l_my_u: loop {
                        c.cycles += 19;
                        let a1 = c.g[A1][0] as u32;
                        let a3 = c.g[A3][0] as u32;
                        c.ldl(4, a1.wrapping_add(0x7));
                        c.ldr(4, a1);
                        c.ldl(6, a1.wrapping_add(0xf));
                        c.ldr(6, a1.wrapping_add(0x8));
                        c.ldl(10, a1.wrapping_add(0x17));
                        c.ldr(10, a1.wrapping_add(0x10));
                        c.ldl(11, a1.wrapping_add(0x1f));
                        c.ldr(11, a1.wrapping_add(0x18));
                        c.sdl(4, a3.wrapping_add(0x7));
                        c.sdr(4, a3);
                        c.sdl(6, a3.wrapping_add(0xf));
                        c.sdr(6, a3.wrapping_add(0x8));
                        c.sdl(10, a3.wrapping_add(0x17));
                        c.sdr(10, a3.wrapping_add(0x10));
                        c.sdl(11, a3.wrapping_add(0x1f));
                        c.sdr(11, a3.wrapping_add(0x18));
                        c.g[A1][0] = sx(a1.wrapping_add(0x20));
                        let t = c.g[A1][0] != c.g[V1][0];
                        c.g[A3][0] = sx(a3.wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_fb48);
                            }
                            continue 'l_my_u;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(gp_at(c, -0x848));
                        break 'copies;
                    }
                }
                // Aligned: 32 bytes per step with ld/sd.
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = 0x53_0000;
                'l_my_a: loop {
                    c.cycles += 11;
                    let a1 = c.g[A1][0] as u32;
                    let a3 = c.g[A3][0] as u32;
                    c.g[A0][0] = c.ld(a1);
                    c.g[A2][0] = c.ld(a1.wrapping_add(0x8));
                    c.g[T2][0] = c.ld(a1.wrapping_add(0x10));
                    c.g[T3][0] = c.ld(a1.wrapping_add(0x18));
                    c.sd(a3, c.g[A0][0]);
                    c.sd(a3.wrapping_add(0x8), c.g[A2][0]);
                    c.sd(a3.wrapping_add(0x10), c.g[T2][0]);
                    c.sd(a3.wrapping_add(0x18), c.g[T3][0]);
                    c.g[A1][0] = sx(a1.wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V1][0];
                    c.g[A3][0] = sx(a3.wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_fba4);
                        }
                        continue 'l_my_a;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(gp_at(c, -0x848));
                    break 'copies;
                }
            }
            // The opponent's stats: the 12-byte header of the first record,
            // then 0xa0 bytes in a loop, each from the other rider.
            'opp_stats: {
                c.cycles += 18;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4f0c));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A1][0] as u32);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A3][0] as u32);
                let v = c.g[V1][0];
                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), v);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x790));
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xfc));
                c.g[V0][0] = c.g[A0][0] | c.g[A2][0];
                c.g[V0][0] &= 7;
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19c));
                if t {
                    // Aligned: the loop below, straight from here.
                    'l_opp_a: loop {
                        c.cycles += 11;
                        let a0 = c.g[A0][0] as u32;
                        let a2 = c.g[A2][0] as u32;
                        c.g[T2][0] = c.ld(a0);
                        c.g[T3][0] = c.ld(a0.wrapping_add(0x8));
                        c.g[V0][0] = c.ld(a0.wrapping_add(0x10));
                        c.g[A1][0] = c.ld(a0.wrapping_add(0x18));
                        c.sd(a2, c.g[T2][0]);
                        c.sd(a2.wrapping_add(0x8), c.g[T3][0]);
                        c.sd(a2.wrapping_add(0x10), c.g[V0][0]);
                        c.sd(a2.wrapping_add(0x18), c.g[A1][0]);
                        c.g[A0][0] = sx(a0.wrapping_add(0x20));
                        let t = c.g[A0][0] != c.g[V1][0];
                        c.g[A2][0] = sx(a2.wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_fc70);
                            }
                            continue 'l_opp_a;
                        }
                        break 'opp_stats;
                    }
                }
                // Unaligned: 32 bytes per step with ldl/ldr and sdl/sdr.
                'l_opp_u: loop {
                    c.cycles += 19;
                    let a0 = c.g[A0][0] as u32;
                    let a2 = c.g[A2][0] as u32;
                    c.ldl(2, a0.wrapping_add(0x7));
                    c.ldr(2, a0);
                    c.ldl(5, a0.wrapping_add(0xf));
                    c.ldr(5, a0.wrapping_add(0x8));
                    c.ldl(7, a0.wrapping_add(0x17));
                    c.ldr(7, a0.wrapping_add(0x10));
                    c.ldl(8, a0.wrapping_add(0x1f));
                    c.ldr(8, a0.wrapping_add(0x18));
                    c.sdl(2, a2.wrapping_add(0x7));
                    c.sdr(2, a2);
                    c.sdl(5, a2.wrapping_add(0xf));
                    c.sdr(5, a2.wrapping_add(0x8));
                    c.sdl(7, a2.wrapping_add(0x17));
                    c.sdr(7, a2.wrapping_add(0x10));
                    c.sdl(8, a2.wrapping_add(0x1f));
                    c.sdr(8, a2.wrapping_add(0x18));
                    c.g[A0][0] = sx(a0.wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.g[A2][0] = sx(a2.wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_fc1c);
                        }
                        continue 'l_opp_u;
                    }
                    c.cycles += 2;
                    break;
                }
            }
            // The last 12 bytes of the opponent's record, then the
            // interface's virtual entry (`vtEntry->fn(vthis(iface, *vtEntry))`).
            c.cycles += 11;
            let a0 = c.g[A0][0] as u32;
            let a2 = c.g[A2][0] as u32;
            c.ldl(7, a0.wrapping_add(0x7));
            c.ldr(7, a0);
            c.g[T0][0] = c.lw(a0.wrapping_add(0x8));
            c.sdl(7, a2.wrapping_add(0x7));
            c.sdr(7, a2);
            c.sw(a2.wrapping_add(0x8), c.g[T0][0]);
            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0022_fcc8)?;
        }

        // cCameraTriggerMan_cleanupOnExit(&gCamTriggerMan)
        c.cycles += 16;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5830));
        call_ptr(c, 0x0016_c9b0, 0x0022_fcd0)?;
        // GrMan->FlushRenderer()
        c.g[A1][0] = c.lw(gp_at(c, -0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_fce8)?;
        // cFEAsyncManager_FlushFEAsyncTextures(SSXApp->mFEAsyncManager)
        c.g[V0][0] = c.lw(gp_at(c, -0x848));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
        call_ptr(c, 0x001a_3c50, 0x0022_fcf4)?;
        // cSSXScriptEngine_ForceEndScriptsAll(ScriptEngine)
        c.g[A0][0] = c.lw(gp_at(c, -0x84c));
        call_ptr(c, 0x0027_8a98, 0x0022_fcfc)?;
        // if (self->UI) cUIEngine(self->UI, 3)
        c.g[A0][0] = c.lw(s0_at(c, 0x48));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_7b70, 0x0022_fd10)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x7c));
        // if (self->InputSync) cInputSync(self->InputSync, 3)
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            Callee {
                addr: 0x0023_70a0,
                f: ee::s06::InputSync_dtor,
            }
            .call(c, 0x0022_fd24)?;
        }
        // cCutSceneTransitionMan_ForceToRegularMode, SurfPropCache purge and
        // delete, then the SkyBox virtual delete.
        c.cycles += 9;
        c.g[A0][0] = c.lw(s0_at(c, 0x64));
        call_ptr(c, 0x002e_a820, 0x0022_fd2c)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x44));
        call_ptr(c, 0x002c_3fa0, 0x0022_fd34)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x44));
        call_ptr(c, 0x0031_7e50, 0x0022_fd3c)?;
        c.g[A2][0] = c.lw(s0_at(c, 0x14));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0xc, 0x0022_fd5c)?;
        }
        // FogMan (vtable at +4).
        c.cycles += 3;
        c.g[A2][0] = c.lw(s0_at(c, 0x18));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x4, 0x0022_fd7c)?;
        }
        // LensFxMan (vtable at +0x649c).
        c.cycles += 3;
        c.g[A2][0] = c.lw(s0_at(c, 0x1c));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x649c, 0x0022_fd9c)?;
        }
        // cPathSys_purgePaths(gPaths), cBXNash purge, cMenuSys delete.
        c.cycles += 7;
        c.g[RA][0] = sx(0x0022_fda4);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x33a0));
        call_ptr(c, 0x0026_af00, 0x0022_fda4)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x20));
        call_ptr(c, 0x0033_4850, 0x0022_fdac)?;
        c.g[A0][0] = c.lw(s0_at(c, 0xa4));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002c_c048, 0x0022_fdc0)?;
        }
        c.g[A0][0] = c.g[S1][0];
        // cBucketMan_mergeBucket(bm, 1), cBucketMan_purge(bm).
        c.cycles += 7;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0022_fdcc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_4b38, 0x0022_fdd4)?;
        // WScriptMan (vtable at +0x494).
        c.g[A2][0] = c.lw(s0_at(c, 0x30));
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x494, 0x0022_fdf4)?;
        }
        // CrowdMan2d, CrowdMan3d, MultiParticleMan, PlaybackMan, InstanceMan.
        c.cycles += 3;
        c.g[A0][0] = c.lw(s0_at(c, 0x38));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_9278, 0x0022_fe08)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x3c));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0034_3b28, 0x0022_fe1c)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x40));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0035_7a00, 0x0022_fe30)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x34));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0026_c588, 0x0022_fe44)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x88));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0035_1120, 0x0022_fe58)?;
        }
        // cCameraTriggerMan_purge, cRenderStateMan_purge, ViewMan delete.
        c.cycles += 7;
        c.g[RA][0] = sx(0x0022_fe60);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5830));
        call_ptr(c, 0x0016_cbe0, 0x0022_fe60)?;
        call_ptr(c, 0x002e_cfa0, 0x0022_fe68)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x84));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_e730, 0x0022_fe7c)?;
        }
        c.g[A2][0] = c.lw(s0_at(c, 0xc));
        // AI (vtable at +0xcc).
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0xcc, 0x0022_fe9c)?;
        }
        // Replay.
        c.cycles += 3;
        c.g[A0][0] = c.lw(s0_at(c, 0x28));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0026_f088, 0x0022_feb0)?;
        }
        c.g[A2][0] = c.lw(s0_at(c, 0x98));
        // StreamPause (vtable at +0).
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !t {
            c.cycles += 5;
            vdelete(c, 0x0, 0x0022_fed0)?;
        }
        // SectionMan, FXLoader, WorldLightMan, World, OcclusionMan, mpTrackBigFile.
        c.cycles += 3;
        c.g[A0][0] = c.lw(s0_at(c, 0x78));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_cd18, 0x0022_fee4)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x2c));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002d_bee0, 0x0022_fef8)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x90));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002f_5230, 0x0022_ff0c)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x10));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_6740, 0x0022_ff20)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x8c));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0022_ff34);
            c.g[A1][0] = 3;
            Callee {
                addr: 0x0024_2518,
                f: ee::s06::OcclusionMan_dtor,
            }
            .call(c, 0x0022_ff34)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0xa8));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0031_68e8, 0x0022_ff48)?;
        }
        // LoadIndicator (plain delete), DebugFont, then the BXNash array.
        c.cycles += 5;
        c.g[A0][0] = c.lw(s0_at(c, 0x94));
        call_ptr(c, 0x0031_7e50, 0x0022_ff50)?;
        c.g[A0][0] = c.lw(s0_at(c, 0x24));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_16c0, 0x0022_ff64)?;
        }
        c.g[A0][0] = c.lw(s0_at(c, 0x20));
        // nash->~sObj20() over its 0x14-byte entries, then delete.
        c.cycles += 2;
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
        if !t {
            c.cycles += 2;
            let t = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            if !t {
                loop {
                    c.cycles += 7;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                    if !t {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_ff78);
                    }
                }
            }
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0022_ff9c)?;
        }
        // GrMan->PurgeCharModels(), GrMan->RemoveTexture(self->mBoxTextureID),
        // cGraphicsMan_PurgeTextures, cIrradianceDataBase_Purge.
        c.cycles += 22;
        c.g[A1][0] = c.lw(gp_at(c, -0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x388));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_ffb4)?;
        c.g[A2][0] = c.lw(gp_at(c, -0x854));
        c.g[A1][0] = c.lw(s0_at(c, 0x220));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x190));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x194));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_ffd0)?;
        c.g[A0][0] = c.lw(gp_at(c, -0x854));
        call_ptr(c, 0x0036_a020, 0x0022_ffd8)?;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_adb0, 0x0022_ffe0)?;
        // SSXApp->Game = 0; WorldSound delete.
        c.g[V0][0] = c.lw(gp_at(c, -0x848));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        c.g[A0][0] = c.lw(s0_at(c, 0x4c));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002b_6ae0, 0x0022_fffc)?;
        }
        // The world-level resets and the global purges.
        c.cycles += 21;
        c.g[A0][0] = c.lw(gp_at(c, 0x21e4));
        call_ptr(c, 0x002b_68d0, 0x0023_0004)?;
        c.g[A0][0] = c.lw(gp_at(c, 0x410));
        call_ptr(c, 0x0028_6a80, 0x0023_000c)?;
        c.g[A0][0] = c.lw(gp_at(c, 0x410));
        call_ptr(c, 0x0028_6c00, 0x0023_0014)?;
        c.g[A0][0] = c.lw(gp_at(c, -0x84c));
        call_ptr(c, 0x0027_8590, 0x0023_001c)?;
        call_ptr(c, 0x0036_a290, 0x0023_0024)?;
        c.g[V0][0] = c.lw(gp_at(c, 0x2a74));
        c.sw(gp_at(c, -0x69c), 0);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};

    /// A vtable at `vt` whose slots `offs` hold a zero `this` adjustment and
    /// a fake method (the miss hook logs the call).
    fn put_vtable(c: &mut Cpu, vt: u32, offs: &[u32]) {
        for (i, &off) in offs.iter().enumerate() {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// Owned objects with virtual deleters: (self field, object, the offset
    /// of its vtable pointer, the vtable).
    const OWNED: [(u32, u32, u32, u32); 6] = [
        (0x0c, DATA + 0x2_0000, 0xcc, DATA + 0x3_4000),
        (0x14, DATA + 0x2_1000, 0xc, DATA + 0x3_5000),
        (0x18, DATA + 0x2_2000, 0x4, DATA + 0x3_6000),
        (0x1c, DATA + 0x2_3000, 0x649c, DATA + 0x3_7000),
        (0x30, DATA + 0x3_0000, 0x494, DATA + 0x3_8000),
        (0x98, DATA + 0x3_1000, 0x0, DATA + 0x3_9000),
    ];

    #[test]
    fn game_module_unload_matches() {
        check(ssx3_ee::s06::f_0022fa98__generated, game_module_unload, 200, &|c, seed| {
            let _r = fakes::machine(c, seed, 0x0022_fa98, 0);
            c.pc = 0x0022_fa98;
            c.g[RA][0] = crate::difftest::RA as u64;
            let this = DATA + 0x8000;
            c.g[A0][0] = this as u64;
            // SSXApp, its BE and the back end's interface table.
            let ssx = DATA + 0x3000;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), ssx);
            c.bus.write32(ssx + 0x78, DATA + 0x1000);
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
            c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
            let be_vt = DATA + 0x1200;
            put_vtable(c, be_vt, &[8]);
            c.bus.write32(0x0053_08a8, DATA + 0x1100);
            c.bus.write32(DATA + 0x1104, be_vt);
            // GrMan and its function block (flush, purge, remove-texture).
            let gr = DATA + 0x5000;
            let st = DATA + 0x6000;
            put_vtable(c, st, &[0x98, 0x190, 0x388]);
            c.bus.write32(gr + 0x10d8, st);
            c.bus.write32(GAME_GP.wrapping_sub(0x854), gr);
            // The singletons the purges reach (empty).
            c.bus.write32(GAME_GP.wrapping_sub(0x84c), DATA + 0x7000);
            c.bus.write32(GAME_GP.wrapping_add(0x410), DATA + 0x7800);
            c.bus.write32(GAME_GP.wrapping_add(0x21e4), DATA + 0x7c00);
            c.bus.write32(GAME_GP.wrapping_add(0x2a74), DATA + 0x7a00);
            // The owned objects with virtual deleters; everything else empty.
            for (field, obj, vt_at, vt) in OWNED {
                put_vtable(c, vt, &[8]);
                c.bus.write32(obj + vt_at, vt);
                c.bus.write32(this + field, obj);
            }
        });
    }
}
