//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for a mixed bag of small helpers — sort keys, BE interface
//! lookups, script/flag predicates, audio singletons, a VU0 triangle test,
//! two destructors, a scratchpad DMA wrapper, a particle update, a modifier
//! render hook — each with a differential test against the generated body.
//!
//! Calls follow [`crate::call`]: the same registers at each call, the same
//! return address, the unwind check after it, and the same cycle charge per
//! block. `BERace_GetInterface` and `DeferMan_ComputeSortKey` are jump-table
//! dispatchers: the tables they read live in the game image (`.rodata`), so
//! the ports read them from guest memory and jump like the original, and
//! `BERace_GetInterface` also accepts the resume entries its `__entries`
//! twin accepts (one per table target, plus the empty-result and epilogue
//! blocks), because a thread switch inside a getter unwinds back into it.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_46a0, flags64_test, "Flags64_Test"),
    (0x0015_30e0, be_race_get_interface, "BERace_GetInterface"),
    (0x0028_b180, ssx_audio_system_get_instance, "SSXAudioSystem_GetInstance"),
    (0x0028_b1c0, ssx_audio_system_get_app, "SSXAudioSystem_GetApp"),
    (0x002e_dfd0, irradiance_ref_resolve, "IrradianceRef_Resolve"),
    (0x0030_a270, wscript_man_has_obj_script1, "WScriptMan_HasObjScript1"),
    (0x0032_9970, rider_sphere_tree_dtor, "RiderSphereTree_dtor"),
    (0x0032_b6a8, vu0_triangle_intersect, "VU0_TriangleIntersect"),
    (0x0032_f708, coll_hull_dtor, "CollHull_dtor"),
    (0x0034_5bc8, particle_modifier_render, "ParticleModifier_Render"),
    (0x0035_7950, particle_update, "Particle_Update"),
    (0x0036_0800, sync_process_id_less_equal, "SyncProcess_IdLessEqual"),
    (0x0036_4240, defer_man_compute_sort_key, "DeferMan_ComputeSortKey"),
    (0x0038_f738, spr_stream_download, "SprStream_Download"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    /// `__builtin_delete` (operator delete): frees through the heap manager.
    pub const DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::f_00317e50 };
    /// `Dma_FromScratchpad` (0x371dd8): programs a scratchpad → RAM transfer.
    pub const DMA_FROM_SCRATCHPAD: Callee = Callee { addr: 0x0037_1dd8, f: ee::s12::f_00371dd8 };
    /// `ModObj_Update` (0x356198): the base object update.
    pub const MOD_OBJ_UPDATE: Callee = Callee { addr: 0x0035_6198, f: ee::s11::f_00356198 };
}

/// The BE interface getters `BERace_GetInterface` dispatches to, in table
/// order: (the case block, its return address, the getter).
const BE_INTERFACE_GETTERS: [(u32, u32, Callee); 14] = [
    (0x0015_310c, 0x0015_3114, Callee { addr: 0x0014_48d8, f: ee::s01::f_001448d8 }),
    (0x0015_311c, 0x0015_3124, Callee { addr: 0x0014_5970, f: ee::s01::f_00145970 }),
    (0x0015_312c, 0x0015_3134, Callee { addr: 0x0014_edd8, f: ee::s01::f_0014edd8 }),
    (0x0015_313c, 0x0015_3144, Callee { addr: 0x0014_7f78, f: ee::s01::f_00147f78 }),
    (0x0015_314c, 0x0015_3154, Callee { addr: 0x0014_f250, f: ee::s01::f_0014f250 }),
    (0x0015_31dc, 0x0015_31e4, Callee { addr: 0x0015_2638, f: ee::s01::f_00152638 }),
    (0x0015_315c, 0x0015_3164, Callee { addr: 0x0014_f900, f: ee::s01::f_0014f900 }),
    (0x0015_316c, 0x0015_3174, Callee { addr: 0x0014_e0e0, f: ee::s01::f_0014e0e0 }),
    (0x0015_317c, 0x0015_3184, Callee { addr: 0x0015_4948, f: ee::s01::f_00154948 }),
    (0x0015_318c, 0x0015_3194, Callee { addr: 0x0014_abe0, f: ee::s01::f_0014abe0 }),
    (0x0015_319c, 0x0015_31a4, Callee { addr: 0x0015_32a0, f: ee::s01::f_001532a0 }),
    (0x0015_31ac, 0x0015_31b4, Callee { addr: 0x0015_08d8, f: ee::s01::f_001508d8 }),
    (0x0015_31bc, 0x0015_31c4, Callee { addr: 0x0015_5a50, f: ee::s01::f_00155a50 }),
    (0x0015_31cc, 0x0015_31d4, Callee { addr: 0x0015_69c0, f: ee::s01::f_001569c0 }),
];

/// `BERace_GetInterface`'s kind table (`.rodata` at 0x45a810; entry `kind`).
const BE_INTERFACE_TABLE: u32 = 0x0045_a810;
/// `DeferMan_ComputeSortKey`'s class table (`.rodata` at 0x492010).
const DEFER_CLASS_TABLE: u32 = 0x0049_2010;

/// `SSXAudioSystem` globals: the app singleton (`gp - 0x848`, the same word as
/// the world pointer) and the audio instance (`gp + 0x410`).
const AUDIO_APP_GP: i32 = -0x848;
const AUDIO_INSTANCE_GP: i32 = 0x410;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `lw` at `addr` (sign-extended into the 64-bit register).
#[inline(always)]
fn lw(c: &mut Cpu, addr: u32) -> u64 {
    c.lw(addr)
}

/// `ra = *(sp)` (the frame's saved return address).
#[inline(always)]
fn reload_ra(c: &mut Cpu) {
    let a = sp(c);
    c.g[RA][0] = c.ld(a);
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Flags64_Test.

/// `bool Flags64_Test(Flags64* f, int bit)` (0x1446a0): whether bit `bit` of
/// the first 64-bit word is set (`bit` masked to 0..63).
pub fn flags64_test(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.ld(abi::arg(c, 0));
    c.g[V1][0] = 0x1;
    c.g[V1][0] = c.g[V1][0] << (c.g[A1][0] & 63);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BERace_GetInterface.

/// The stages `BERace_GetInterface` can be entered (or resumed) at.
#[derive(Clone, Copy)]
enum Iface {
    /// 0x1530e0: the frame, the range check.
    Entry,
    /// 0x1530f0: read the kind's table entry into `a0`.
    Dispatch,
    /// One getter's call block (index into [`BE_INTERFACE_GETTERS`]).
    Getter(usize),
    /// 0x1531ec: unknown kind, return null.
    Empty,
    /// The table's first entry points back at the entry block.
    Restart,
    /// 0x1531f4: `jr ra` with `sp += 0x10`.
    Return,
}

/// `void* BERace_GetInterface(int kind)` (0x1530e0): the BE subsystem's
/// interface object for `kind` (0 race, 1 player, 2 char DB, 3 char, 4 option,
/// 5 save, 6 bag, 7 network, 8 score, 9 bolt, 10 mission, 11 econ, 12
/// aggression, 13 reward), or null for anything else. The table of case blocks
/// is at 0x45a810; each block calls that kind's singleton getter.
pub fn be_race_get_interface(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        let mut stage = match c.pc as u32 {
            0x0015_30e0 => Iface::Entry,
            0x0015_30f0 => Iface::Dispatch,
            0x0015_31ec => Iface::Empty,
            0x0015_31f4 => Iface::Return,
            pc => match BE_INTERFACE_GETTERS.iter().position(|g| g.0 == pc) {
                Some(k) => Iface::Getter(k),
                None => {
                    c.jump_out(pc, 0x0015_30e0, 0x0015_31fc);
                    return Ok(());
                }
            },
        };
        loop {
            match stage {
                Iface::Entry => {
                    c.cycles += 4;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
                    c.g[V0][0] = (c.g[A1][0] < 0xeu64) as u64;
                    let in_range = c.g[V0][0] != 0;
                    frame.save_ra(c, 0);
                    stage = if in_range { Iface::Dispatch } else { Iface::Empty };
                }
                Iface::Dispatch => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x46_0000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    // (0x460000 - 0x57f0 is the kind table at 0x45a810.)
                    c.g[V0][0] = sx(BE_INTERFACE_TABLE);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    // `v1` already holds table + kind * 4.
                    let slot = c.g[V1][0] as u32;
                    c.g[A0][0] = Addr(slot).lw(c);
                    let target = c.g[A0][0] as u32;
                    stage = match target {
                        0x0015_30e0 => Iface::Restart,
                        pc => match BE_INTERFACE_GETTERS.iter().position(|g| g.0 == pc) {
                            Some(k) => Iface::Getter(k),
                            None => {
                                c.jump_out(target, 0x0015_30e0, 0x0015_31fc);
                                return Ok(());
                            }
                        },
                    };
                }
                Iface::Getter(k) => {
                    c.cycles += 4;
                    let (_, ret, getter) = BE_INTERFACE_GETTERS[k];
                    getter.call(c, ret)?;
                    reload_ra(c);
                    stage = Iface::Return;
                }
                Iface::Empty => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    reload_ra(c);
                    stage = Iface::Return;
                }
                Iface::Restart => {
                    // Unreachable with the shipped table: a target equal to the
                    // entry loops the whole function (with its event check).
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_30e0);
                    }
                    stage = Iface::Entry;
                }
                Iface::Return => {
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem singletons.

/// `void* SSXAudioSystem_GetInstance()` (0x28b180): the audio system object
/// stored in the game globals (`gp + 0x410`).
pub fn ssx_audio_system_get_instance(c: &mut Cpu) {
    c.cycles += 2;
    let a = gp(c, AUDIO_INSTANCE_GP);
    c.g[V0][0] = c.lw(a);
    abi::ret(c);
}

/// `void* SSXAudioSystem_GetApp()` (0x28b1c0): the app object (`gp - 0x848`,
/// the same global as the world pointer).
pub fn ssx_audio_system_get_app(c: &mut Cpu) {
    c.cycles += 2;
    let a = gp(c, AUDIO_APP_GP);
    c.g[V0][0] = c.lw(a);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WScriptMan_HasObjScript1.

/// `bool WScriptMan_HasObjScript1(WScriptMan* m, WScriptObj* obj)`
/// (0x30a270): whether the object has a slot 1 script — the script manager's
/// object table (`m + 0x28c`) lookup returns anything but -1.
pub fn wscript_man_has_obj_script1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28c));
        call_ptr(c, 0x003a_6b78, 0x0030_a284)?;
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VU0_TriangleIntersect.

/// `bool VU0_TriangleIntersect(const Vec4* a, const Vec4* b, const Vec4* c,
/// const Vec4* d)` (0x32b6a8): loads the four points into VF10..VF13 and runs
/// the microprogram at VU address 0x6e0 (`vcallmsr` through VI27); VI2 comes
/// back non-zero when the triangle intersects.
pub fn vu0_triangle_intersect(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6e0));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
    c.lqc2(10, c.g[A0][0] as u32);
    c.lqc2(11, c.g[A1][0] as u32);
    c.lqc2(12, c.g[A2][0] as u32);
    c.lqc2(13, c.g[A3][0] as u32);
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    c.cfc2(V0, 2);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.pc = t;
}

// ---------------------------------------------------------------------------
// IrradianceRef_Resolve.

/// `void IrradianceRef_Resolve(IrradianceRef* r)` (0x2edfd0): store the
/// manager's handle for `r` (`func_0038AC50(&D_005047F8, r)`) in `r->handle`
/// (+0) and the type tag 0x123400 in `r->tag` (+4).
pub fn irradiance_ref_resolve(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_ac50, 0x002e_dff0)?;
        c.g[V1][0] = 0x12_0000;
        let handle = c.g[V0][0];
        let this = r32(c, S0);
        Addr(this).sw(c, handle);
        c.g[V1][0] = c.g[V1][0] | 0x3400;
        frame.restore_ra(c, 0);
        let tag = c.g[V1][0];
        Addr(this.wrapping_add(0x4)).sw(c, tag);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSphereTree_dtor / CollHull_dtor.

/// `void RiderSphereTree_dtor(RiderSphereTree* t, int flags)` (0x329970): free
/// the object when `flags & 1` (its members are plain data).
pub fn rider_sphere_tree_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let free = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if free {
            c.cycles += 2;
            callee::DELETE.call(c, 0x0032_9988)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CollHull_dtor(CollHull* h, int flags)` (0x32f708): restore the base
/// vtable (`+0x50`), destroy the sphere tree at `+0x70`, then free the hull
/// when `flags & 1`.
pub fn coll_hull_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a70));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        let hull = r32(c, S1);
        Addr(hull.wrapping_add(0x50)).sw(c, vtable);
        c.g[A0][0] = sx(hull.wrapping_add(0x70));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0032_9970, 0x0032_f738)?;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0032_f74c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ParticleModifier_Render.

/// `void ParticleModifier_Render(ParticleModifier* m)` (0x345bc8): render the
/// modifier's effect (`m + 0x50`) only while its render handle (`+0x1e4`) is
/// still unset.
pub fn particle_modifier_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1e4));
        let has_handle = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !has_handle {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
            call_ptr(c, 0x0037_08c0, 0x0034_5be4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Particle_Update.

/// `void Particle_Update(Particle* p)` (0x357950): the base object update;
/// afterwards, unless the modifier block is mid-transform (`+0x1c` set with a
/// non-zero `+0x10`), tell the object's vtable slot 1 (`+8`) that it is done
/// (`a1 = 3`).
pub fn particle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MOD_OBJ_UPDATE.call(c, 0x0035_7964)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x1c));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 2;
        let null_self = c.g[S0][0] == 0;
        c.g[A1][0] = 0x3;
        if !null_self {
            c.cycles += 5;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_7998)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SyncProcess_IdLessEqual.

/// `bool SyncProcess_IdLessEqual(SyncProcess* self, SyncProcessObj* obj)`
/// (0x360800): `obj->GetId()` (slot at `obj->vt + 0x40`) `>= self->id`
/// (`self + 0x14`), i.e. the negation of `id < self->id`.
pub fn sync_process_id_less_equal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0036_0824)?;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DeferMan_ComputeSortKey.

/// `u32 DeferMan_ComputeSortKey(DeferEntry* e, u32 key)` (0x364240): the
/// packet's sort key, the bitwise complement of a packed
/// `{31 - layer(e+8), class(e+4), category(e+0), key}`. `class` is
/// `e[1] & 3` (0 → 0x3ff, 1 → 0x3fe, 2 → `(e[2] & 0x1ffffc00) >> 10`, 3 → 0);
/// `category` comes from the nine-entry table at 0x492010 indexed by
/// `(e[0] & 0x3c0) >> 6`; the key contributes `(short)key` when it is
/// non-negative and `(key >> 16) & 0xf` shifted to bits 6..9 when the high
/// half is non-negative.
pub fn defer_man_compute_sort_key(c: &mut Cpu) {
    c.cycles += 10;
    let entry = abi::arg(c, 0);
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = 0x1f;
    c.g[T0][0] = lw(c, entry.wrapping_add(0x8));
    c.g[A0][0] = 0x1;
    c.g[V1][0] = lw(c, entry.wrapping_add(0x4));
    c.g[V0][0] = c.g[T0][0] & 0x3e0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
    c.g[V1][0] = c.g[V1][0] & 0x3;
    let sel_one = c.g[V1][0] == c.g[A0][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    if sel_one {
        // b = 0x3fe
        c.cycles += 2;
        c.g[T0][0] = 0x3fe;
    } else {
        c.cycles += 3;
        if c.g[V1][0] == 0 {
            // b = 0x3ff
            c.cycles += 2;
            c.g[T0][0] = 0;
            c.cycles += 2;
            c.g[T0][0] = 0x3ff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x1fff_0000;
            if c.g[V1][0] == 2 {
                // b = (e[2] & 0x1ffffc00) >> 10
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] | 0xfc00;
                c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32) >> 10);
            } else {
                // b = 0
                c.cycles += 2;
                c.g[T0][0] = 0;
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = lw(c, entry);
    c.cycles += 5;
    c.g[V0][0] = c.g[V0][0] & 0x3c0;
    c.g[A0][0] = sx((c.g[V0][0] as u32) >> 6);
    c.g[V1][0] = (c.g[A0][0] < 0x9u64) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x49_0000;
    if in_table {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        // (0x490000 + 0x2010 is the class table at 0x492010.)
        c.g[V0][0] = sx(DEFER_CLASS_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let slot = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(slot).lw(c);
        let target = c.g[A0][0] as u32;
        let (cost, class) = match target {
            0x0036_42e0 => (2, 6),
            0x0036_42e8 => (2, 5),
            0x0036_42f0 => (2, 4),
            0x0036_42f8 => (2, 3),
            0x0036_4300 => (2, 2),
            0x0036_4308 => (2, 1),
            0x0036_4310 => (1, 0),
            _ => {
                c.jump_out(target, 0x0036_4240, 0x0036_4360);
                return;
            }
        };
        c.cycles += cost;
        c.g[A0][0] = class;
    } else {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }

    // The key: low half when non-negative, then the high nibble at bits 6..9.
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
    c.g[A3][0] = 0;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
    c.g[V1][0] = ((c.g[V0][0] as i64) < 0) as u64;
    let hi_negative = (c.g[A1][0] as i64) < 0;
    if c.g[V1][0] == 0 {
        c.g[A3][0] = c.g[V0][0];
    }
    if !hi_negative {
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0] & 0xf;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
    }
    c.cycles += 9;
    c.g[V0][0] = sx((c.g[T0][0] as u32) << 16);
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 26);
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 13);
    c.g[A3][0] = c.g[A3][0] & 0x3ff;
    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
    c.g[V0][0] = !(c.g[V0][0] | c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SprStream_Download.

/// `void SprStream_Download(u32 madr, int size, int flags)` (0x38f738): send
/// `size >> 4` qwords from the scratchpad to `madr` through the DMA manager's
/// state object (`gp + 0x2a94`).
pub fn spr_stream_download(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = c.g[A0][0];
        c.g[A3][0] = c.g[A2][0];
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 4) as u32);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[V0][0];
        callee::DMA_FROM_SCRATCHPAD.call(c, 0x0038_f758)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn flags64_test_matches() {
        check(ssx3_ee::s01::f_001446a0__generated, flags64_test, 200, &|c, seed| {
            let mut r = Rng(0x51ed_2701 ^ seed as u64);
            scramble(c, &mut r);
            let at = DATA + (seed % 8) * 0x10;
            c.g[A0][0] = sx(at);
            c.bus.write64(at, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
            // 0..63, plus out-of-range shifts (masked to 0..63).
            c.g[A1][0] = if seed % 5 == 0 { 64 + (seed % 9) as u64 } else { (r.next_u32() % 64) as u64 };
        });
    }

    #[test]
    fn ssx_audio_system_get_instance_matches() {
        check(ssx3_ee::s07::f_0028b180__generated, ssx_audio_system_get_instance, 100, &|c, seed| {
            let mut r = Rng(0x2b18_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x410, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn ssx_audio_system_get_app_matches() {
        check(ssx3_ee::s07::f_0028b1c0__generated, ssx_audio_system_get_app, 100, &|c, seed| {
            let mut r = Rng(0x2b1c_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn rider_sphere_tree_dtor_matches() {
        check(ssx3_ee::s10::f_00329970__generated, rider_sphere_tree_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x3299_70, 0);
            // Freeing a live object would walk the (uninitialised) heap manager,
            // so the delete branch is driven with a null object and the keep
            // branch with a real one: both reach the same code.
            if seed % 2 == 1 {
                c.g[A0][0] = 0;
                c.g[A1][0] = 1;
            } else {
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = sx(r.next_u32() & !1);
            }
        });
    }

    #[test]
    fn coll_hull_dtor_matches() {
        check(ssx3_ee::s11::f_0032f708__generated, coll_hull_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x32f7_08, 0);
            let flags = (r.next_u32() & !1) | (seed % 2) as u32;
            c.g[A1][0] = sx(flags);
            c.g[A0][0] = if flags & 1 != 0 {
                0 // delete(null): a live hull would go to the heap manager
            } else {
                sx(DATA + (r.next_u32() & 0x1ff0))
            };
        });
    }

    #[test]
    fn irradiance_ref_resolve_matches() {
        check(ssx3_ee::s09::f_002edfd0__generated, irradiance_ref_resolve, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x2edf_d0, 0);
            // The manager's entry count at D_005047F8+4 is <= 0, so the lookup
            // callee returns 0 without walking its table.
            c.bus.write32(0x0050_47f8 + 4, 0);
            c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
        });
    }

    #[test]
    fn wscript_man_has_obj_script1_matches() {
        check(ssx3_ee::s10::f_0030a270__generated, wscript_man_has_obj_script1, 200, &|c, seed| {
            machine(c, seed, 0x30a2_70, 0);
            let manager = DATA;
            let table = DATA + 0x400;
            let obj = DATA + 0x800;
            c.g[A0][0] = sx(manager);
            c.g[A1][0] = sx(obj);
            c.bus.write32(manager + 0x28c, table);
            c.bus.write8(obj + 0x78, 0);
            c.bus.write32(table + 4, DATA + 0x600);
            c.bus.write32(DATA + 0x604, DATA + 0xc00);
            if seed % 2 == 0 {
                // The object has no script cache entry: the lookup is -1.
                c.bus.write32(obj + 0x88, 0);
            } else {
                // A resident entry: the slot 1 script id comes back.
                c.bus.write32(obj + 0x88, DATA + 0x1000);
                c.bus.write32(DATA + 0x1008, 0x800);
                c.bus.write32(DATA + 0xc08, DATA + 0x1200);
                c.bus.write32(DATA + 0x121c, DATA + 0x1400);
                c.bus.write32(DATA + 0x140c, 0x1234_5678);
            }
        });
    }

    #[test]
    fn vu0_triangle_intersect_matches() {
        check(ssx3_ee::s10::f_0032b6a8__generated, vu0_triangle_intersect, 100, &|c, seed| {
            let mut r = Rng(0x32b6_a801 ^ seed as u64);
            scramble(c, &mut r);
            for k in 0..4 {
                let at = DATA + 0x100 + 0x10 * k as u32;
                c.g[A0 + k][0] = sx(at);
                put_vec(c, at, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            }
        });
    }

    #[test]
    fn particle_modifier_render_matches() {
        check(ssx3_ee::s11::f_00345bc8__generated, particle_modifier_render, 150, &|c, seed| {
            machine(c, seed, 0x345b_c8, 3);
            // The effect submitter (0x3708c0) walks the render context's
            // vtables; a zeroed context makes both calls miss, and the second
            // one returns 1 so it stops after its "already submitted" check.
            c.bus.write32(GAME_GP + 0x2a90, DATA + 0x2000);
            c.bus.write32(LOG_RESULTS, 0);
            c.bus.write32(LOG_RESULTS + 4, 1);
            let m = DATA;
            c.g[A0][0] = sx(m);
            c.bus.write32(m + 0x1e4, if seed % 2 == 0 { 1 } else { 0 });
        });
    }

    #[test]
    fn particle_update_matches() {
        check(ssx3_ee::s11::f_00357950__generated, particle_update, 150, &|c, seed| {
            machine(c, seed, 0x3579_50, 3);
            // Every virtual call is a fake: the first two results are forced
            // to 0, which takes the base update's short paths.
            c.bus.write32(LOG_RESULTS, 0);
            c.bus.write32(LOG_RESULTS + 4, 0);
            let obj = DATA;
            c.g[A0][0] = sx(obj);
            c.bus.write32(obj + 0xc, DATA + 0x200);
            for off in [0x08u32, 0x78, 0xa0, 0xb8] {
                c.bus.write16(DATA + 0x200 + off, 0);
                c.bus.write32(DATA + 0x200 + off + 4, FAKE);
            }
            c.bus.write32(obj + 0x20, 0);
            match seed % 3 {
                0 => c.bus.write32(obj + 0x1c, 0),
                1 => {
                    c.bus.write32(obj + 0x1c, DATA + 0x400);
                    c.bus.write32(DATA + 0x410, 1);
                }
                _ => {
                    c.bus.write32(obj + 0x1c, DATA + 0x400);
                    c.bus.write32(DATA + 0x410, 0);
                }
            }
        });
    }

    #[test]
    fn sync_process_id_less_equal_matches() {
        check(ssx3_ee::s12::f_00360800__generated, sync_process_id_less_equal, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3608_00, 1);
            let obj = DATA;
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(obj);
            c.bus.write32(obj.wrapping_add(0x14), r.next_u32());
            // The object's vtable: slot 8 (`+0x40`) with no `this` adjustment.
            c.bus.write32(obj + 0xc, DATA + 0x200);
            c.bus.write16(DATA + 0x200 + 0x40, 0);
            c.bus.write32(DATA + 0x200 + 0x44, FAKE);
        });
    }

    #[test]
    fn defer_man_compute_sort_key_matches() {
        check(ssx3_ee::s12::f_00364240__generated, defer_man_compute_sort_key, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3642_40, 0);
            c.pc = 0x0036_4240;
            // The category switch is a jump table in .rodata.
            let mut table = [
                0x0036_4310, 0x0036_42f0, 0x0036_42e8, 0x0036_42f8, 0x0036_4308, 0x0036_42e0,
                0x0036_4300, 0x0036_4300, 0x0036_4308,
            ];
            // sel: 0..3 (one per seed class); category: in range, past the table
            // (the default class), and one seed class where the table holds a
            // value that is not a case block.
            let category = if seed % 16 == 0 {
                table[0] = 0xdead_beef;
                0
            } else if seed % 8 == 0 {
                9 + seed % 7
            } else {
                seed % 9
            };
            put_words(c, DEFER_CLASS_TABLE, &table);
            let sel = seed % 4;
            let e = DATA;
            c.g[A0][0] = sx(e);
            c.bus.write32(e, (category << 6) | (r.next_u32() & !0x3c0));
            c.bus.write32(e + 4, sel | (r.next_u32() & !3));
            c.bus.write32(e + 8, r.next_u32());
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn be_race_get_interface_matches() {
        check(ssx3_ee::s01::f_001530e0__generated, be_race_get_interface, 300, &|c, seed| {
            machine(c, seed, 0x1530_e0, 1);
            c.pc = 0x0015_30e0;
            // The kind table and the 14 singletons: with the instance globals
            // set, each getter returns its dummy at once.
            let mut table = [
                0x0015_310c, 0x0015_311c, 0x0015_312c, 0x0015_313c, 0x0015_314c, 0x0015_31dc,
                0x0015_315c, 0x0015_316c, 0x0015_317c, 0x0015_318c, 0x0015_319c, 0x0015_31ac,
                0x0015_31bc, 0x0015_31cc,
            ];
            let kind = if seed % 15 == 0 {
                14 + (seed / 15) % 3 // out of range: null, no table read
            } else {
                seed % 15
            };
            if seed % 32 == 0 {
                // A table slot that is not a case block: the original jumps out
                // of itself (the fake hook takes the call).
                table[kind as usize % 14] = 0xdead_beef;
            }
            put_words(c, BE_INTERFACE_TABLE, &table);
            let globals = [
                0x1f3c, 0x1f30, 0x1ee8, 0x1f2c, 0x1ee4, 0x1ed4, 0x1ee0, 0x1eec, 0x1ea8, 0x1f28,
                0x1eb8, 0x1edc, 0x1ea4, 0x1e8c,
            ];
            for (i, off) in globals.iter().enumerate() {
                c.bus.write32(GAME_GP.wrapping_sub(*off), DATA + 0x1000 + 0x100 * i as u32);
            }
            c.g[A1][0] = sx(kind);
        });
    }
}
