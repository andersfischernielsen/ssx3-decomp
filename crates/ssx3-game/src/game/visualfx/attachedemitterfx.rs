//! `src/visualfx/attachedemitterfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::ps2f::FpuF32;
use crate::abi::r32;
use crate::call::Flow;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::visualfx::boardwakefx::at;
pub(crate) use crate::game::visualfx::boardwakefx::addiu;
pub(crate) use crate::game::visualfx::boardwakefx::back_edge;
pub(crate) use crate::game::visualfx::boardwakefx::vmul_bc;
pub(crate) use crate::game::visualfx::boardwakefx::vadd;
pub(crate) use crate::game::visualfx::boardwakefx::copy_quad;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_1810, attached_emitter_fx_free_type_data, "AttachedEmitterFX_FreeTypeData"),
    (0x002f_1850, attached_emitter_fx_ctor, "AttachedEmitterFX_ctor"),
    (0x002f_1890, attached_emitter_fx_ctor_2, "AttachedEmitterFX_ctor_2"),
    (0x002f_18d8, attached_emitter_fx_init, "AttachedEmitterFX_Init"),
    (0x002f_1940, attached_emitter_fx_dtor, "AttachedEmitterFX_dtor"),
    (0x002f_19f0, attached_emitter_fx_set_model, "AttachedEmitterFX_SetModel"),
    (0x002f_1a08, attached_emitter_fx_update, "AttachedEmitterFX_Update"),
    (0x002f_2218, attached_emitter_fx_destroy_emitter, "AttachedEmitterFX_DestroyEmitter"),
];

// ---------------------------------------------------------------------------
// AttachedEmitterFX_Update.

/// `AttachedEmitterFX` fields.
pub(crate) mod attached {

    /// Set once the bone indices below have been looked up.
    pub const BONES_FOUND: u32 = 0;

    /// Bone index per attachment point (0x19 of them, from +4).
    pub const BONES: u32 = 4;

    /// The ten emitters (0 = unused), from +0x6c.
    pub const EMITTERS: u32 = 0x6c;

    pub const EMITTERS_END: u32 = 0x28;

    /// The rider model (`+0x30` bone matrices, `+0x140` scale).
    pub const MODEL: u32 = 0x94;

    /// The rider (`+0x6c0` interface, `+0x370` velocity).
    pub const RIDER: u32 = 0x98;

    /// Set when the "random sparkle" attachment (0x19) spawned.
    pub const SPARKLED: u32 = 0xa0;
}

/// `DynamicEmitter` fields the update reads: attachment point, "always
/// on", noise phase, descriptor.
pub(crate) mod emitter {

    pub const POINT: u32 = 0x200;

    pub const ALWAYS: u32 = 0x204;

    pub const PHASE: u32 = 0x208;

    pub const DESC: u32 = 0x20c;
}

pub(crate) mod attached_callee {
    use super::ee;
    use super::Callee;

    pub const FIND_BONES: Callee = Callee { addr: 0x002f_22c8, f: ee::s09::AttachedEmitterFX_FindBones };

    pub const NOISE: Callee = Callee { addr: 0x002d_1928, f: ee::s09::Noise_Fractal1D };
}


/// `void AttachedEmitterFX_SetModel(AttachedEmitterFX *this, void *model)`
/// (0x2f19f0): store the model at +0x94 and clear the active flag.
pub fn attached_emitter_fx_set_model(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x94).sw(c, c.g[A1][0]);
    Addr(this).sw(c, 0);
    abi::ret(c);
}


pub(crate) const ATTACHED_EMITTER_FX_FREE_TYPE_DATA: Callee =
    Callee { addr: 0x002f_1810, f: ssx3_ee::s09::f_002f1810 };


pub(crate) const ATTACHED_EMITTER_FX_LOAD_TYPE_DATA: Callee =
    Callee { addr: 0x002f_17d0, f: ssx3_ee::s09::f_002f17d0 };

pub(crate) const ATTACHED_EMITTER_FX_INIT: Callee =
    Callee { addr: 0x002f_18d8, f: ssx3_ee::s09::f_002f18d8 };

pub(crate) const ATTACHED_EMITTER_FX_RESET: Callee =
    Callee { addr: 0x002f_1a00, f: ssx3_ee::s09::f_002f1a00 };

pub(crate) const RETURN_TRIVIAL: Callee = Callee { addr: 0x002f_8ff8, f: ssx3_ee::s09::f_002f8ff8 };


// ---------------------------------------------------------------------------
// Attached emitter FX.

/// `void AttachedEmitterFX_DestroyEmitter(AttachedEmitterFX *this, int index)`
/// (0x2f2218): clear `this->emitters[index]` (+0x6c + index*4) and, if set,
/// call its virtual release (slot +0x20 of `*(emitter + 0x1f8)`).
pub fn attached_emitter_fx_destroy_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        c.g[A1][0] = sx(index.wrapping_shl(2));
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x6c));
        frame.save_ra(c, 0);
        let slot = this.wrapping_add(0x6c).wrapping_add(index.wrapping_shl(2));
        c.g[S0][0] = sx(slot);
        let obj = Addr(slot).lw(c);
        c.g[A2][0] = obj;
        let empty = obj == 0;
        c.g[A1][0] = 3;
        if !empty {
            c.cycles += 5;
            let vt = Addr(obj as u32).off(0x1f8).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0x20);
            c.g[A0][0] = m.this(obj);
            c.g[V1][0] = m.func;
            call_ptr(c, m.func as u32, 0x002f_2250)?;
        }
        c.cycles += 5;
        Addr(slot).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// AttachedEmitterFX type data / constructors.

/// `void AttachedEmitterFX_FreeTypeData(void)` (0x2f1810): run the type
/// data's two shutdown hooks, and if the global type data (`gp+0xac0`) is
/// set, free it and clear the global.
pub fn attached_emitter_fx_free_type_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002f_1820);
        RETURN_TRIVIAL.call(c, 0x002f_1820)?;
        c.g[RA][0] = sx(0x002f_1828);
        call_ptr(c, 0x002f_a5f8, 0x002f_1828)?;
        let gp = c.g[GP][0] as u32;
        let data = Addr(gp.wrapping_add(0xac0)).lw(c);
        c.g[A0][0] = data;
        if data != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002f_183c)?;
        }
        c.cycles += 3;
        Addr(gp.wrapping_add(0xac0)).sw(c, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `AttachedEmitterFX *AttachedEmitterFX_ctor(AttachedEmitterFX *this,
/// void *type)` (0x2f1850): set the vtable, store `type` at +0x98, and run
/// `AttachedEmitterFX_Init`; returns `this`.
pub fn attached_emitter_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let ty = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        let vtable = 0x0049_0000u32.wrapping_sub(0x7e30);
        c.g[V0][0] = sx(vtable);
        frame.save_ra(c, 0);
        c.g[S0][0] = this as u64;
        Addr(this).off(0x94).sw(c, 0);
        Addr(this).off(0x9c).sw(c, 0);
        Addr(this).off(0xa4).sw(c, vtable as u64);
        c.g[RA][0] = sx(0x002f_187c);
        Addr(this).off(0x98).sw(c, ty);
        ATTACHED_EMITTER_FX_INIT.call(c, 0x002f_187c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `AttachedEmitterFX *AttachedEmitterFX_ctor_2(AttachedEmitterFX *this,
/// int index, void *type)` (0x2f1890): as the ctor but store `index` at +0x9c
/// (leaving +0x98 clear) and run `AttachedEmitterFX_Reset` afterwards.
pub fn attached_emitter_fx_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let index = abi::arg64(c, 1);
        let ty = abi::arg64(c, 2);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        let vtable = 0x0049_0000u32.wrapping_sub(0x7e30);
        c.g[V0][0] = sx(vtable);
        frame.save_ra(c, 0);
        c.g[S0][0] = this as u64;
        Addr(this).off(0x94).sw(c, 0);
        Addr(this).off(0x98).sw(c, 0);
        Addr(this).off(0x9c).sw(c, index);
        c.g[RA][0] = sx(0x002f_18bc);
        Addr(this).off(0xa4).sw(c, vtable as u64);
        ATTACHED_EMITTER_FX_INIT.call(c, 0x002f_18bc)?;
        c.g[RA][0] = sx(0x002f_18c4);
        c.g[A0][0] = c.g[S0][0];
        ATTACHED_EMITTER_FX_RESET.call(c, 0x002f_18c4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void AttachedEmitterFX_Init(AttachedEmitterFX *this)` (0x2f18d8): if the
/// global type data (`gp+0xac4`) is absent, load it; then bump the type
/// data's reference count and clear the ten words at +0x90..0x68 with a
/// back-edge event check at 0x2f1910.
pub fn attached_emitter_fx_init(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x002f_1910;
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let gp = c.g[GP][0] as u32;
        let loaded = Addr(gp.wrapping_add(0xac4)).lw(c);
        c.g[V0][0] = loaded;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = this as u64;
        if loaded == 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002f_18f8);
            ATTACHED_EMITTER_FX_LOAD_TYPE_DATA.call(c, 0x002f_18f8)?;
        }
        c.cycles += 6;
        let count = Addr(gp.wrapping_add(0xac4)).lw(c);
        c.g[V0][0] = count;
        let mut p = this.wrapping_add(0x90);
        c.g[A0][0] = sx(p);
        c.g[V1][0] = 9;
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        Addr(gp.wrapping_add(0xac4)).sw(c, c.g[V0][0]);
        let mut k = 9i64;
        loop {
            c.cycles += 7;
            Addr(p).sw(c, 0);
            k -= 1;
            c.g[V1][0] = sx(k as u32);
            let more = k >= 0;
            p = p.wrapping_sub(4);
            c.g[A0][0] = sx(p);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// AttachedEmitterFX destructor.

/// `void AttachedEmitterFX_dtor(AttachedEmitterFX *this, int flags)`
/// (0x2f1940): drop the type data reference (freeing it at 0), release and
/// clear the ten emitter slots at +0x6c, then free `this` when `flags & 1`.
/// Back-edge event check at 0x2f1990.
pub fn attached_emitter_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x002f_1990;
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        let gp = c.g[GP][0] as u32;
        let refcount = Addr(gp.wrapping_add(0xac4)).lw(c);
        c.g[V1][0] = refcount;
        frame.save(c, S2, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x40);
        let this = abi::arg(c, 0);
        c.g[S2][0] = this as u64;
        frame.save(c, S1, 0x30);
        let vtable = 0x0049_0000u32.wrapping_sub(0x7e30);
        c.g[V0][0] = sx(vtable);
        frame.save(c, S3, 0x10);
        let count = (refcount as u32).wrapping_sub(1);
        c.g[V1][0] = sx(count);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 1);
        Addr(this).off(0xa4).sw(c, vtable as u64);
        let still_used = count != 0;
        Addr(gp.wrapping_add(0xac4)).sw(c, c.g[V1][0]);
        if !still_used {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002f_1984);
            ATTACHED_EMITTER_FX_FREE_TYPE_DATA.call(c, 0x002f_1984)?;
        }
        c.cycles += 3;
        c.g[S3][0] = c.g[S0][0] & 1;
        c.g[S1][0] = 9;
        let mut p = this.wrapping_add(0x6c);
        c.g[S0][0] = sx(p);
        let mut k = 9i64;
        loop {
            c.cycles += 3;
            let obj = Addr(p).lw(c);
            c.g[A2][0] = obj;
            c.g[A1][0] = 3;
            if obj != 0 {
                c.cycles += 5;
                let vt = Addr(obj as u32).off(0x1f8).lw(c);
                c.g[V0][0] = vt;
                let m = VSlot::read(c, vt, 0x20);
                c.g[A0][0] = m.this(obj);
                c.g[V1][0] = m.func;
                call_ptr(c, m.func as u32, 0x002f_19b0)?;
            }
            c.cycles += 4;
            Addr(p).sw(c, 0);
            k -= 1;
            c.g[S1][0] = sx(k as u32);
            let more = k >= 0;
            p = p.wrapping_add(4);
            c.g[S0][0] = sx(p);
            if more {
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        let free = c.g[S3][0] == 0;
        frame.restore(c, S0, 0x40);
        if !free {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002f_19d0);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7e50, 0x002f_19d0)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The global random seed (`seed = (seed * 0x18fcd + 0xe9507c)`, the float
/// `1.0 | seed & 0x7fffff` in [1, 2)).
pub(crate) const RAND_SEED: u32 = 0xa0c;

/// The zero vector with w = 1 (`0x500000 - 0xee0`).
pub(crate) const ORIGIN: u32 = 0x004f_f120;

pub(crate) const DYNAMIC_EMITTER_SPAWN: u32 = 0x0037_10d0;


/// `|*src|` on VU0 as the original issues it (`vf4 = vf3 * vf3`, the sum
/// through ACC, `vsqrt`); Q goes through `gpr` to f`f`.
pub(crate) fn vu0_length(c: &mut Cpu, src: u32, gpr: usize, f: usize) {
    c.lqc2(3, src);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
    c.mtc1(gpr, f);
}


/// The emitter's bone matrix (`model->bones[this->bones[em->point]]`, the
/// emitter in v1, the bone table base in t0) copied to s3 through vf1-vf4,
/// and the origin row (address through `origin_reg`) to sp+0x70.
pub(crate) fn attached_bone(c: &mut Cpu, sp: u32, origin_reg: usize) {
    let model = c.lw(at(c.g[S2][0], attached::MODEL));
    c.g[A0][0] = model;
    c.g[V0][0] = c.lw(at(c.g[V1][0], emitter::POINT));
    c.g[A1][0] = c.lw(at(model, 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    let (m, s3) = (r32(c, V1), r32(c, S3));
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, m.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, s3.wrapping_add(0x10 * i));
    }
    c.g[origin_reg][0] = sx(ORIGIN);
    copy_quad(c, ORIGIN, sp.wrapping_add(0x70));
}


/// `vf12 = M * *p` with M's rows at s3 (vf4-vf7), stored to `out`.
pub(crate) fn attached_transform(c: &mut Cpu, p: u32, out: u32) {
    let s3 = r32(c, S3);
    c.lqc2(8, p);
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, s3.wrapping_add(0x10 * i));
    }
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, out);
}


/// One step of the global random seed (in v0) into v0 as the original
/// does it through registers: `rd = v0 * k`, `v0 = (rd + add) & mask |
/// one` (k = 0x18fcd, add = 0xe9507c, mask = 0x7fffff, one = 1.0f).
pub(crate) fn rand_step(c: &mut Cpu, rd: usize, k: usize, add: usize, mask: usize, one: usize) {
    c.mult(0, rd, V0, k);
    c.g[V0][0] = sx(r32(c, rd).wrapping_add(r32(c, add)));
    c.g[V0][0] &= c.g[mask][0];
    c.g[V0][0] |= c.g[one][0];
}


/// `void AttachedEmitterFX_Update(AttachedEmitterFX *fx)` (0x2f1a08): look
/// up the attachment bones once, then for each of the ten emitters decide
/// whether it spawns this frame (always-on emitters; others only within
/// the descriptor's speed window `+0xfc..+0x100` of the rider's speed;
/// then a random chance `+0x104` and a noise threshold `+0x108` with the
/// phase advancing by `+0x10c`), place the spawn point at the bone plus
/// the descriptor's offset (attachment 0x19 sparkles at a random offset,
/// 0x17/0x18 shift by ±75 in x), scaled by the model's scale and
/// transformed by the bone, and spawn: with a velocity (the rider's
/// direction and velocity weighted by `+0xd8..+0xe0` plus the bone-space
/// direction `+0xf0`) when it fires, else an idle spawn (no velocity).
///
/// Frame 0x180: s0-s7, fp at +0x140..+0xc0, ra +0xb0, f20-f25 at
/// +0x150..+0x178.
pub fn attached_emitter_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let sp = r32(c, SP).wrapping_sub(0x180);
        let gp = r32(c, GP);
        c.cycles += 21;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S2, 0x120);
        frame.save(c, S0, 0x140);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x130);
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        frame.save(c, S5, 0xf0);
        frame.save(c, S6, 0xe0);
        frame.save(c, S7, 0xd0);
        frame.save(c, FP, 0xc0);
        frame.save_ra(c, 0xb0);
        for (f, off) in [(25, 0x178), (24, 0x170), (23, 0x168), (22, 0x160), (21, 0x158), (20, 0x150)] {
            frame.save_fpr(c, f, off);
        }
        let found = c.lw(at(c.g[S2][0], attached::BONES_FOUND));
        c.g[V0][0] = 0x50_0000;
        if found == 0 {
            c.cycles += 3;
            attached_callee::FIND_BONES.call(c, 0x002f_1a64)?;
            c.g[V0][0] = 0x50_0000;
        }
        // The rider's direction (sp+0) and velocity (sp+0x10, sp+0x20).
        c.cycles += 12;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        c.g[S0][0] = sx(ORIGIN);
        let rider = c.lw(at(c.g[S2][0], attached::RIDER));
        c.g[A1][0] = rider;
        c.lq(V0, ORIGIN);
        c.sw(at(c.g[S2][0], attached::SPARKLED), 0);
        c.sq(V0, sp);
        c.sw(sp.wrapping_add(0x10), 0);
        c.sw(sp.wrapping_add(0x14), 0);
        c.swc1(20, sp.wrapping_add(0x18));
        c.swc1(20, sp.wrapping_add(0x1c));
        if rider != 0 {
            c.cycles += 12;
            let vt = c.lw(at(rider, 0x6c0));
            c.g[V1][0] = vt;
            c.g[A1][0] = sx(at(rider, 0x6c0));
            c.g[A0][0] = c.lh(at(vt, 0x10));
            let f = c.lw(at(vt, 0x14));
            c.g[V0][0] = f;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, f as u32, 0x002f_1ab0)?;
            c.lq(V1, r32(c, V0));
            c.g[A0][0] = c.lw(at(c.g[S2][0], attached::RIDER));
            c.sq(V1, sp);
            let vel = at(c.g[A0][0], 0x370);
            c.lq(V0, vel);
            c.sq(V0, sp.wrapping_add(0x10));
            c.sq(V0, sp.wrapping_add(0x20));
        }
        // Speed = |velocity|, the random constants, the emitter walk.
        c.cycles += 26;
        vu0_length(c, sp, V0, 21);
        c.g[FP][0] = sx(at(c.g[S2][0], attached::EMITTERS));
        c.g[S6][0] = 0x1_8fcd;
        c.g[S5][0] = 0xe9_507c;
        c.g[S4][0] = 0x7f_ffff;
        c.g[AT][0] = 0xffff_ffff_c2aa_0000;
        c.mtc1(AT, 25);
        c.g[AT][0] = 0xffff_ffff_c160_0000;
        c.mtc1(AT, 24);
        c.mov_s(22, 21);
        c.lwc1(23, gp.wrapping_sub(0x392c));
        c.g[S3][0] = sx(sp.wrapping_add(0x40));
        c.g[S1][0] = c.g[FP][0];
        c.g[V1][0] = c.lw(r32(c, S1));
        loop {
            attached_emitter(c, sp, gp)?;
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, FP).wrapping_add(attached::EMITTERS_END));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = c.lw(r32(c, S1));
            back_edge(c, 0x002f_1b30);
        }
        c.cycles += 18;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        for (f, off) in [(25, 0x178), (24, 0x170), (23, 0x168), (22, 0x160), (21, 0x158), (20, 0x150)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}


/// One emitter (loop body at 0x2f1b30; the emitter in v1, its slot at s1,
/// advanced by 4).
pub(crate) fn attached_emitter(c: &mut Cpu, sp: u32, gp: u32) -> Flow {
    let s = |off: u32| sp.wrapping_add(off);
    c.cycles += 2;
    let em = c.g[V1][0];
    c.g[S7][0] = 0;
    if em == 0 {
        c.cycles += 1;
        addiu(c, S1, S1, 4);
        return Ok(());
    }
    // Does it fire? Always-on, or within the speed window.
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(em, emitter::ALWAYS));
    let always = c.g[V0][0] != 0;
    let desc = c.lw(at(em, emitter::DESC));
    c.g[S0][0] = desc;
    let mut chance_loaded = false;
    if !always {
        c.cycles += 7;
        c.lwc1(0, at(desc, 0xfc));
        c.lwc1(1, gp.wrapping_sub(0x3928));
        c.mul_s(0, 0, 1);
        c.c_le_s(0, 21);
        if !c.fcc() {
            c.lwc1(1, at(desc, 0x104));
            chance_loaded = true;
        } else {
            c.cycles += 6;
            c.lwc1(0, at(desc, 0x100));
            c.mul_s(0, 0, 1);
            c.c_le_s(21, 0);
            if c.fcc() {
                c.g[S7][0] = 1;
            }
        }
    }
    if !chance_loaded {
        c.cycles += 1;
        c.lwc1(1, at(desc, 0x104));
    }
    // The random chance.
    c.cycles += 4;
    c.c_lt_s(1, 23);
    let roll = c.fcc();
    c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
    if roll {
        c.cycles += 12;
        c.g[A0][0] = 0x3f80_0000;
        rand_step(c, V1, S6, S5, S4, A0);
        let seed = c.g[V0][0];
        c.sw(gp.wrapping_add(RAND_SEED), seed);
        c.lwc1(0, gp.wrapping_add(RAND_SEED));
        c.sub_s(0, 0, 20);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.g[S7][0] = 0;
        }
    }
    // The noise threshold.
    c.cycles += 5;
    c.lwc1(0, at(desc, 0x108));
    c.c_lt_s(0, 23);
    if !c.fcc() {
        c.g[A2][0] = c.lw(r32(c, S1));
    } else {
        c.cycles += 15;
        c.lwc1(1, at(desc, 0x10c));
        c.lwc1(0, gp.wrapping_sub(0x3924));
        c.g[V0][0] = c.lw(r32(c, S1));
        c.mul_s(1, 1, 0);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(AT, 2);
        let phase = at(c.g[V0][0], emitter::PHASE);
        c.lwc1(0, phase);
        c.add_s(0, 0, 1);
        c.swc1(0, phase);
        c.g[V0][0] = c.lw(r32(c, S1));
        let phase = at(c.g[V0][0], emitter::PHASE);
        c.lwc1(0, phase);
        c.c_le_s(2, 0);
        let wrap = c.fcc();
        c.g[A0][0] = 3;
        if wrap {
            c.cycles += 3;
            c.sub_s(0, 0, 2);
            c.swc1(0, phase);
            c.g[V0][0] = c.lw(r32(c, S1));
        }
        c.cycles += 11;
        c.lwc1(12, at(c.g[V0][0], emitter::PHASE));
        attached_callee::NOISE.call(c, 0x002f_1c20)?;
        c.add_s(0, 0, 20);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 1);
        c.lwc1(2, at(desc, 0x108));
        c.mul_s(0, 0, 1);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.g[S7][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.lw(r32(c, S1));
    }
    // The spawn point: the bone's position (sp+0x20) and the offset
    // (sp+0x30).
    c.cycles += 25;
    addiu(c, A1, S2, attached::BONES);
    let model = c.lw(at(c.g[S2][0], attached::MODEL));
    c.g[V1][0] = model;
    c.g[T0][0] = c.g[A1][0];
    c.g[V0][0] = c.lw(at(c.g[A2][0], emitter::POINT));
    c.g[A3][0] = 0x19;
    c.g[A0][0] = c.lw(at(model, 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 6);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, A1));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let bone = r32(c, A0);
    copy_quad(c, bone.wrapping_add(0x30), s(0x20));
    c.lwc1(4, at(desc, 0xe4));
    c.lwc1(5, at(desc, 0xec));
    c.lwc1(0, at(desc, 0xe8));
    c.swc1(4, s(0x30));
    c.swc1(0, s(0x34));
    c.swc1(5, s(0x38));
    c.swc1(6, s(0x3c));
    c.g[V1][0] = c.lw(at(c.g[A2][0], emitter::POINT));
    c.g[V0][0] = 0x17;
    if c.g[V1][0] == c.g[A3][0] {
        // Attachment 0x19: a random offset (x += 170 * r - 85, z += 28 * r
        // - 14), and the sparkle flag.
        c.cycles += 32;
        c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
        c.g[V1][0] = 1;
        c.sw(at(c.g[S2][0], attached::SPARKLED), 1);
        c.g[A0][0] = 0x3f80_0000;
        c.mult(0, V1, V0, S6);
        c.g[AT][0] = 0x432a_0000;
        c.mtc1(AT, 2);
        c.g[AT][0] = 0x41e0_0000;
        c.mtc1(AT, 3);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
        c.g[V0][0] &= c.g[S4][0];
        c.g[V0][0] |= c.g[A0][0];
        c.mult(0, A0, V0, S6);
        let seed = c.g[V0][0];
        c.sw(gp.wrapping_add(RAND_SEED), seed);
        c.g[V0][0] = 0x3f80_0000;
        c.lwc1(1, gp.wrapping_add(RAND_SEED));
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
        c.sub_s(1, 1, 6);
        c.g[V1][0] &= c.g[S4][0];
        c.g[V1][0] |= c.g[V0][0];
        let seed = c.g[V1][0];
        c.sw(gp.wrapping_add(RAND_SEED), seed);
        c.mul_s(1, 1, 2);
        c.lwc1(0, gp.wrapping_add(RAND_SEED));
        c.add_s(1, 1, 25);
        c.sub_s(0, 0, 6);
        c.add_s(1, 4, 1);
        c.mul_s(0, 0, 3);
        c.swc1(1, s(0x30));
        c.add_s(0, 0, 24);
        c.add_s(0, 5, 0);
        c.swc1(0, s(0x38));
        c.cycles += 1;
        c.g[V0][0] = c.lw(at(c.g[S2][0], attached::MODEL));
    } else {
        // Attachments 0x17 / 0x18: x + 75 / x - 75.
        c.cycles += 2;
        let left = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x18;
        let mut shifted = true;
        if left {
            c.cycles += 4;
            c.g[AT][0] = 0x4296_0000;
            c.mtc1(AT, 0);
            c.add_s(0, 4, 0);
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(at(c.g[S2][0], attached::MODEL));
                shifted = false;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x4296_0000;
                c.mtc1(AT, 0);
                c.sub_s(0, 4, 0);
            }
        }
        if shifted {
            c.cycles += 1;
            c.swc1(0, s(0x30));
            c.cycles += 1;
            c.g[V0][0] = c.lw(at(c.g[S2][0], attached::MODEL));
        }
    }
    // Scale the offset by the model's scale; unless it is zero, transform
    // it by the bone and add it to the position.
    c.cycles += 15;
    let model = c.g[V0][0];
    c.lwc1(1, s(0x30));
    c.lwc1(0, at(model, 0x140));
    c.lwc1(2, s(0x34));
    c.mul_s(1, 1, 0);
    c.lwc1(3, s(0x38));
    c.mtc1(ZERO, 4);
    c.swc1(1, s(0x30));
    c.c_eq_s(1, 4);
    c.lwc1(0, at(model, 0x144));
    c.mul_s(2, 2, 0);
    c.swc1(2, s(0x34));
    c.lwc1(0, at(model, 0x148));
    c.mul_s(3, 3, 0);
    let x_zero = c.fcc();
    c.swc1(3, s(0x38));
    let mut transform = true;
    let mut reload = true;
    if x_zero {
        c.cycles += 4;
        c.c_eq_s(2, 4);
        if !c.fcc() {
            c.g[V1][0] = c.lw(r32(c, S1));
            reload = false;
        } else {
            c.cycles += 3;
            c.c_eq_s(3, 4);
            transform = !c.fcc();
        }
    }
    if transform {
        if reload {
            c.cycles += 1;
            c.g[V1][0] = c.lw(r32(c, S1));
        }
        c.cycles += 37;
        attached_bone(c, sp, V1);
        attached_transform(c, s(0x30), s(0x90));
        copy_quad(c, s(0x90), s(0x30));
        c.sq(V0, s(0x80));
        vadd(c, s(0x20), s(0x30), s(0x20), false);
    }
    c.cycles += 2;
    if c.g[S7][0] == 0 {
        // Idle spawn: no velocity.
        c.g[A0][0] = c.lw(r32(c, S1));
        c.cycles += 5;
        c.g[A1][0] = sx(s(0x20));
        c.lwc1(12, gp.wrapping_sub(0x391c));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, DYNAMIC_EMITTER_SPAWN, 0x002f_202c)?;
        c.cycles += 1;
        addiu(c, S1, S1, 4);
        return Ok(());
    }
    // Fire: velocity = dir * d8 + vel * dc + vel * e0 * speed (sp+0x30),
    // plus the descriptor's direction in bone space.
    c.cycles += 49;
    c.lwc1(0, at(desc, 0xd8));
    vmul_bc(c, V0, 0, s(0), s(0x60));
    c.mtc1(ZERO, 1);
    copy_quad(c, s(0x60), s(0x50));
    c.lwc1(0, at(desc, 0xdc));
    vmul_bc(c, V1, 0, s(0x10), s(0x70));
    copy_quad(c, s(0x70), s(0x60));
    vadd(c, s(0x50), s(0x60), s(0x70), false);
    copy_quad(c, s(0x70), s(0x40));
    c.lwc1(0, at(desc, 0xe0));
    vmul_bc(c, V1, 0, s(0x10), s(0x90));
    copy_quad(c, s(0x90), s(0x80));
    vmul_bc(c, V1, 22, s(0x80), s(0x90));
    copy_quad(c, s(0x90), s(0x70));
    vadd(c, s(0x40), s(0x70), s(0x90), false);
    copy_quad(c, s(0x90), s(0x30));
    // Unless the direction (+0xf0) is zero, add it in bone space.
    c.lwc1(0, at(desc, 0xf0));
    c.c_eq_s(0, 1);
    let mut turn = true;
    let mut reload = false;
    if c.fcc() {
        c.cycles += 5;
        c.lwc1(0, at(desc, 0xf4));
        c.c_eq_s(0, 1);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, at(desc, 0xf8));
            c.c_eq_s(0, 1);
            if c.fcc() {
                c.g[A0][0] = c.lw(r32(c, S1));
                turn = false;
            } else {
                reload = true;
            }
        }
    }
    if turn {
        if reload {
            c.cycles += 1;
        }
        c.g[V1][0] = c.lw(r32(c, S1));
        c.cycles += 45;
        attached_bone(c, sp, A0);
        c.lwc1(2, at(desc, 0xf8));
        c.lwc1(0, at(desc, 0xf0));
        c.lwc1(1, at(desc, 0xf4));
        c.swc1(0, s(0x80));
        c.swc1(1, s(0x84));
        c.swc1(2, s(0x88));
        c.swc1(20, s(0x8c));
        attached_transform(c, s(0x80), s(0xa0));
        copy_quad(c, s(0xa0), s(0x80));
        c.sq(V0, s(0x90));
        vadd(c, s(0x30), s(0x80), s(0x30), false);
        c.g[A0][0] = c.lw(r32(c, S1));
    }
    c.cycles += 7;
    c.g[A1][0] = sx(s(0x20));
    c.lwc1(12, gp.wrapping_sub(0x3920));
    c.g[A2][0] = sx(s(0x30));
    c.g[A3][0] = 1;
    call_ptr(c, DYNAMIC_EMITTER_SPAWN, 0x002f_2010)?;
    addiu(c, S1, S1, 4);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::slot;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::delta;


    #[test]
    fn attached_emitter_fx_set_model_matches() {
        check(ssx3_ee::s09::f_002f19f0, attached_emitter_fx_set_model, 100, &|c, seed| {
            let mut r = Rng(0x1010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }


    #[test]
    fn attached_emitter_fx_destroy_emitter_matches() {
        covered(
            ssx3_ee::s09::f_002f2218,
            attached_emitter_fx_destroy_emitter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 21, 1);
                let this = ptr(&mut r);
                c.g[A0][0] = this as u64;
                let idx = r.next_u32() % 8;
                c.g[A1][0] = idx as u64;
                let obj = if r.next_u32() % 4 == 0 { 0 } else { ptr(&mut r) };
                c.bus.write32(this + 0x6c + idx * 4, obj);
                if obj != 0 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(obj + 0x1f8, vt);
                }
            },
        );
    }


    #[test]
    fn attached_emitter_fx_free_type_data_matches() {
        check(ssx3_ee::s09::f_002f1810, attached_emitter_fx_free_type_data, 40, &|c, seed| {
            let mut r = Rng(0x2f18_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            let data = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
            c.bus.write32(crate::difftest::GAME_GP + 0xac0, data);
        });
    }


    /// Shared setup for the AttachedEmitterFX constructors: a global type
    /// data pointer already present, so `Init` skips loading it.
    #[test]
    fn attached_emitter_fx_ctor_matches() {
        check_events(ssx3_ee::s09::f_002f1850, attached_emitter_fx_ctor, 40, &|c, seed| {
            let mut r = Rng(0x2f18_5000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.bus.write32(crate::difftest::GAME_GP + 0xac4, 1);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            let mode = schedule(c, &mut r, seed, 0x002f_1850);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }


    #[test]
    fn attached_emitter_fx_ctor_2_matches() {
        check_events(ssx3_ee::s09::f_002f1890, attached_emitter_fx_ctor_2, 40, &|c, seed| {
            let mut r = Rng(0x2f18_9000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.bus.write32(crate::difftest::GAME_GP + 0xac4, 1);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            let mode = schedule(c, &mut r, seed, 0x002f_1890);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }


    #[test]
    fn attached_emitter_fx_init_matches() {
        check_events(ssx3_ee::s09::f_002f18d8, attached_emitter_fx_init, 40, &|c, seed| {
            let mut r = Rng(0x2f18_d800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.bus.write32(crate::difftest::GAME_GP + 0xac4, 1);
            c.bus.write32(crate::difftest::GAME_GP + 0xac0, DATA + 0x400);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            let mode = schedule(c, &mut r, seed, 0x002f_18d8);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }


    #[test]
    fn attached_emitter_fx_dtor_matches() {
        covered(
            ssx3_ee::s09::f_002f1940,
            attached_emitter_fx_dtor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 23, 10);
                let this = ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() & !1) as u64; // do not free
                c.bus.write32(crate::difftest::GAME_GP + 0xac4, 2); // keep type data
                for i in 0..10 {
                    let obj = if r.next_u32() % 4 == 0 { 0 } else { ptr(&mut r) };
                    c.bus.write32(this + 0x6c + 4 * i, obj);
                    if obj != 0 {
                        let vt = vtable(c, &mut r, &[0x20]);
                        c.bus.write32(obj + 0x1f8, vt);
                    }
                }
            },
        );
    }


    #[test]
    fn attached_emitter_fx_update_matches() {
        covered(ssx3_ee::s09::f_002f1a08__generated, attached_emitter_fx_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2f1a_08, 1);
            let (fx, model, rider, rider_vt) = (DATA, DATA + 0x8000, DATA + 0xa000, DATA + 0xa800);
            let (bones, lods, lod) = (DATA + 0x9100, DATA + 0x8100, DATA + 0x8200);
            c.g[A0][0] = sx(fx);
            // Bones looked up already, or not (then FindBones finds none:
            // every RiderModel_FindBone answers -1).
            c.bus.write32(fx + attached::BONES_FOUND, (seed % 5 != 0) as u32);
            for i in 0..0x1a {
                c.bus.write32(fx + attached::BONES + 4 * i, r.next_u32() % 4);
            }
            c.bus.write32(fx + attached::MODEL, model);
            c.bus.write32(model + 0x30, bones);
            fill(c, &mut r, bones - 0x100, 0x100);
            c.bus.write32(model + 0x1c, lods);
            c.bus.write32(model + 0xc, lod);
            c.bus.write32(lod + 0x44, 0);
            for (i, off) in [0x140, 0x144, 0x148].into_iter().enumerate() {
                let zero = (seed >> i) % 3 == 0;
                c.bus.write32(model + off, if zero { 0 } else { r.f32(2.0) });
            }
            c.bus.write32(fx + attached::RIDER, if seed % 4 == 3 { 0 } else { rider });
            c.bus.write32(rider + 0x6c0, rider_vt);
            slot(c, rider_vt, 0x10, delta(&mut r), FAKE);
            fill(c, &mut r, rider + 0x370, 4);
            // The emitters: attachment points 0x17-0x19 or random, always-on
            // or not, idle spawns (DynamicEmitter_Spawn returns at once).
            for k in 0..10u32 {
                let (em, desc) = (DATA + 0x1000 + 0x300 * k, DATA + 0x5000 + 0x200 * k);
                let used = r.next_u32() % 4 != 0;
                c.bus.write32(fx + attached::EMITTERS + 4 * k, if used { em } else { 0 });
                let point = match r.next_u32() % 5 {
                    0 => 0x17,
                    1 => 0x18,
                    2 => 0x19,
                    _ => r.next_u32() % 0x19,
                };
                c.bus.write32(em + emitter::POINT, point);
                c.bus.write32(em + emitter::ALWAYS, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(em + emitter::PHASE, ((r.next_u32() % 64) as f32 / 8.0).to_bits());
                c.bus.write32(em + emitter::DESC, desc);
                c.bus.write32(em + 0x174, 0);
                for off in (0xd8..0x110).step_by(4) {
                    c.bus.write32(desc + off, r.f32(4.0));
                }
                for off in [0xf0, 0xf4, 0xf8] {
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(desc + off, 0);
                    }
                }
            }
            c.bus.write32(GAME_GP + RAND_SEED, r.next_u32());
            for off in [0x392c, 0x3928, 0x3924, 0x3920, 0x391c] {
                c.bus.write32(GAME_GP - off, r.f32(2.0));
            }
            fill(c, &mut r, ORIGIN, 4);
            // Noise_Sample1D's table (0xa0 floats; negative indices too).
            for i in 0..0x140u32 {
                c.bus.write32(0x0044_5ab0 - 0x280 + 4 * i, r.f32(1.0));
            }
        });
    }
}
