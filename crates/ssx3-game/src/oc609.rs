//! Staging module oc609 (wave l599b.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc609`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0035_a118,
    multispline_modifier_ctor_2,
    "MultiSplineModifier_ctor_2",
)];

/// `cMultiSplineModifier` vtable (`0x490000 - 0xe98`).
pub(crate) const MULTI_SPLINE_VTBL: u32 = 0x0048_f168;

/// `cMultiSplineModifier::cMultiSplineModifier(cBxReplayFrame *)` (0x35a118,
/// `MultiSplineModifier_ctor_2`): builds a multi-spline from a replay frame.
/// Sets the vtable and the defaults (spline id -1, no segment), reads the
/// model count and spline header through the stream's vtable (slots 0x10,
/// 0x18), loads the spline path, allocates the node arrays, then takes one
/// instance id per model from the stream, allocates its instance, and clones,
/// inserts and flags the root transform dirty. Returns `this`.
pub fn multispline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0xe98));
        frame.save(c, S2, 0x40);
        c.g[S0][0] = u64::MAX; // 0xffff_ffff_ffff_0000 | 0xffff
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[A2][0] = 0x34;
        frame.save_ra(c, 0x10);
        c.g[S4][0] = c.g[A1][0];

        // Defaults: vtable, spline id -1, no segment, zero length.
        let this = c.g[S2][0] as u32;
        let vtbl = c.g[V0][0];
        Addr(this).sw(c, vtbl);
        c.g[A1][0] = sx(this.wrapping_add(4));
        let id = c.g[S0][0];
        Addr(this.wrapping_add(0x48)).sw(c, id);
        c.g[S1][0] = sx(this.wrapping_add(0x48));
        Addr(this.wrapping_add(0x50)).sw(c, 0);
        c.g[S3][0] = 0;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        Addr(this.wrapping_add(0x54)).sw(c, 0);

        // stream->read(&NumModels, 52): slot 0x10 of the stream's vtable.
        let stream = c.g[S4][0] as u32;
        c.g[V0][0] = Addr(stream).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(0x14)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_a184)?;

        // cSpline_readFromReplayFrame(&this->Spline, stream).
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        Callee {
            addr: 0x0034_5570,
            f: ee::s11::f_00345570,
        }
        .call(c, 0x0035_a190)?;

        // stream->readReplayInstance(): slot 0x18 of the stream's vtable.
        let stream = c.g[S4][0] as u32;
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x18)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_a1a4)?;
        let instance = c.g[V0][0];
        let this = c.g[S2][0] as u32;
        Addr(this.wrapping_add(0x40)).sw(c, instance);

        // cMultiSplineModifier_allocNodes(this).
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x0035_a328,
            f: ee::s11::f_0035a328,
        }
        .call(c, 0x0035_a1b0)?;

        // for (i = 0; i < NumModels; i++) NodeInst[i] = allocInstance(man, id).
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(4).lw(c);
        let no_models = (c.g[V0][0] as i64) <= 0;
        let id = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, id);
        if !no_models {
            c.cycles += 1;
            c.g[V1][0] = Addr(c.g[S4][0] as u32).lw(c);
            loop {
                c.cycles += 20;
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 4;
                let v1 = c.g[V1][0] as u32;
                c.g[A0][0] = Addr(v1.wrapping_add(0x10)).lh(c);
                c.g[V0][0] = Addr(v1.wrapping_add(0x14)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_a1d8)?;
                // cObjectInterface_getInstanceMan().
                call_ptr(c, 0x002d_1ce0, 0x0035_a1e0)?;
                c.g[A1][0] = Addr(sp(c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x0035_11d0,
                    f: ee::s11::f_003511d0,
                }
                .call(c, 0x0035_a1ec)?;
                let this = c.g[S2][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x44)).lw(c);
                let s3 = c.g[S3][0] as u32;
                c.g[V1][0] = sx(s3 << 2);
                c.g[S3][0] = sx(s3.wrapping_add(1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let slot = c.g[V1][0] as u32;
                let inst = c.g[V0][0];
                Addr(slot).sw(c, inst);
                c.g[V0][0] = Addr(this.wrapping_add(4)).lw(c);
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(c.g[S4][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a1c0);
                }
            }
        }

        // cMultiSplineModifier_CloneInstances(this); setupOverlapSystem(this).
        c.cycles += 15;
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x0035_a458,
            f: ee::s11::f_0035a458,
        }
        .call(c, 0x0035_a218)?;
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x0035_a5f8,
            f: ee::s11::f_0035a5f8,
        }
        .call(c, 0x0035_a220)?;

        // RootTransformDirty = 1; return this.
        c.g[V1][0] = 1;
        c.g[V0][0] = c.g[S2][0];
        let this = c.g[S2][0] as u32;
        Addr(this.wrapping_add(0x30)).sw(c, 1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, fall_back, hook, machine, vtable};
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gated fakes answer only while this word is set in the machine's RAM.
    const OC609_GATE: u32 = DATA + 0x20_0000;
    const OC609_MAGIC: u32 = 0x0609_0609;
    const OC609_THIS: u32 = DATA + 0x3000;
    const OC609_STREAM: u32 = DATA + 0x1000;
    const OC609_NODES: u32 = DATA + 0x3100;

    static OC609_PREV_LOAD_PATH: AtomicUsize = AtomicUsize::new(0);
    static OC609_PREV_ALLOC_ARRAYS: AtomicUsize = AtomicUsize::new(0);
    static OC609_PREV_CLONE: AtomicUsize = AtomicUsize::new(0);
    static OC609_PREV_INSERT: AtomicUsize = AtomicUsize::new(0);
    static OC609_PREV_ALLOC_INSTANCE: AtomicUsize = AtomicUsize::new(0);

    /// Gate-on: log the call as a fake; gate-off: the body the slot held.
    fn oc609_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: fn(&mut Cpu)) {
        if c.bus.read32(OC609_GATE) != OC609_MAGIC {
            return fall_back(c, prev, generated);
        }
        fake_call(c, target);
    }

    fn oc609_fake_load_path(c: &mut Cpu) {
        oc609_gated(
            c,
            0x0034_5570,
            &OC609_PREV_LOAD_PATH,
            ssx3_ee::s11::f_00345570__generated,
        )
    }

    fn oc609_fake_alloc_arrays(c: &mut Cpu) {
        oc609_gated(
            c,
            0x0035_a328,
            &OC609_PREV_ALLOC_ARRAYS,
            ssx3_ee::s11::f_0035a328__generated,
        )
    }

    fn oc609_fake_clone(c: &mut Cpu) {
        oc609_gated(
            c,
            0x0035_a458,
            &OC609_PREV_CLONE,
            ssx3_ee::s11::f_0035a458__generated,
        )
    }

    fn oc609_fake_insert(c: &mut Cpu) {
        oc609_gated(
            c,
            0x0035_a5f8,
            &OC609_PREV_INSERT,
            ssx3_ee::s11::f_0035a5f8__generated,
        )
    }

    fn oc609_fake_alloc_instance(c: &mut Cpu) {
        oc609_gated(
            c,
            0x0035_11d0,
            &OC609_PREV_ALLOC_INSTANCE,
            ssx3_ee::s11::f_003511d0__generated,
        )
    }

    fn oc609_hook_fakes() {
        hook(
            addr_of_mut!(ssx3_ee::s11::OVERRIDE_00345570),
            oc609_fake_load_path,
            &OC609_PREV_LOAD_PATH,
        );
        hook(
            addr_of_mut!(ssx3_ee::s11::OVERRIDE_0035A328),
            oc609_fake_alloc_arrays,
            &OC609_PREV_ALLOC_ARRAYS,
        );
        hook(
            addr_of_mut!(ssx3_ee::s11::OVERRIDE_0035A458),
            oc609_fake_clone,
            &OC609_PREV_CLONE,
        );
        hook(
            addr_of_mut!(ssx3_ee::s11::OVERRIDE_0035A5F8),
            oc609_fake_insert,
            &OC609_PREV_INSERT,
        );
        hook(
            addr_of_mut!(ssx3_ee::s11::OVERRIDE_003511D0),
            oc609_fake_alloc_instance,
            &OC609_PREV_ALLOC_INSTANCE,
        );
    }

    /// Stream with a vtable whose slots 0x10 and 0x18 are fakes; the frame
    /// with a random model count (0 skips the instance loop).
    fn oc609_setup(c: &mut Cpu, seed: u32) {
        oc609_hook_fakes();
        let mut r = machine(c, seed, 0x0035_a118, 8);
        c.bus.write32(OC609_GATE, OC609_MAGIC);
        let vt = vtable(c, &mut r, &[0x10, 0x18]);
        c.bus.write32(OC609_STREAM, vt);
        c.g[A0][0] = sx(OC609_THIS);
        c.g[A1][0] = sx(OC609_STREAM);
        c.bus.write32(OC609_THIS + 4, seed % 4);
        c.bus.write32(OC609_THIS + 0x44, OC609_NODES);
        c.pc = 0x0035_a118;
    }

    #[test]
    fn multispline_modifier_ctor_2_matches() {
        covered(
            ssx3_ee::s11::f_0035a118__generated,
            multispline_modifier_ctor_2,
            200,
            &|c, seed| oc609_setup(c, seed),
        );
    }
}
