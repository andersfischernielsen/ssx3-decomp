//! Staging module oc683 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc683`.
//!
//! Hard leftovers: the bodies come from earlier dead ports (debugmenu.rs,
//! playbackman.rs, replay.rs, frameptr.rs: delete those copies at the reorg),
//! corrected against the generated code (the shell renderer constructor loaded
//! its first `t1` with `0x480000 - 0x2d30`; the generated value is
//! `0x4a0000 + 0x2d30`). The tests that reach callees without an override slot
//! run in a child process with an empty function table (see `oc683_in_child`).

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_9f80,
        shell_renderer_menu_ctor,
        "ShellRendererMenu_ctor",
    ),
    (
        0x0026_d678,
        replay_deserialize_snapshot,
        "Replay_DeserializeSnapshot",
    ),
    (
        0x0026_db88,
        replay_restore_snapshot,
        "Replay_RestoreSnapshot",
    ),
    (0x0026_dbd0, world_state_restore, "WorldState_Restore"),
    (0x0026_dbf0, world_state_read, "WorldState_Read"),
    (
        0x0026_eab8,
        replay_stream_decompress,
        "ReplayStream_Decompress",
    ),
    (
        0x0026_f850,
        replay_restore_resume_snapshot,
        "Replay_RestoreResumeSnapshot",
    ),
];

/// `base + off` as a guest address (`addiu`; `off` may be negative).
pub(crate) fn oc683_off(base: u64, off: i32) -> u32 {
    (base as u32).wrapping_add(off as u32)
}

/// The generated function of an intra-shard `jal` (the generated body calls it
/// directly; after the install the call goes through its override slot).
pub(crate) fn oc683_direct(addr: u32) -> Callee {
    let f: GuestFn = match addr {
        0x0026_cfe8 => ee::s07::f_0026cfe8,
        0x0026_db88 => ee::s07::f_0026db88,
        0x0026_dbd0 => ee::s07::f_0026dbd0,
        0x0026_dbf0 => ee::s07::f_0026dbf0,
        0x0026_ddc0 => ee::s07::f_0026ddc0,
        0x0026_de58 => ee::s07::f_0026de58,
        0x0026_e340 => ee::s07::f_0026e340,
        0x0026_e9b0 => ee::s07::f_0026e9b0,
        0x0026_eab8 => ee::s07::f_0026eab8,
        0x0026_ed88 => ee::s07::f_0026ed88,
        0x0026_ed90 => ee::s07::f_0026ed90,
        0x0026_ee68 => ee::s07::f_0026ee68,
        0x0026_ef30 => ee::s07::f_0026ef30,
        0x0026_f4a0 => ee::s07::f_0026f4a0,
        0x0026_f7f8 => ee::s07::f_0026f7f8,
        0x0027_0238 => ee::s07::f_00270238,
        0x0027_02f8 => ee::s07::f_002702f8,
        0x0027_03f0 => ee::s07::f_002703f0,
        0x0028_b180 => ee::s07::f_0028b180,
        0x0028_f200 => ee::s07::f_0028f200,
        _ => unreachable!(),
    };
    Callee { addr, f }
}

/// `void ShellRendererMenu_ctor(cShellRendererMenu *self)` (0x249f80): the
/// base menu, then the shell renderer's items: nine groups of `IntMenuItem`
/// constructors (`cIntMenuItem_cIntMenuItem`, two of them with float
/// arguments), each added to the menu with `cMenu_addItem` (slot `index`, or
/// -1 for the end). The item objects sit at `self + 0x130 .. 0x2ec`.
/// Frame 0xb0: saves ra and s0-s7, fp.
pub fn shell_renderer_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 169;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 16, 0xa0);
        frame.save(c, 17, 0x90);
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save(c, 21, 0x50);
        frame.save(c, 22, 0x40);
        frame.save(c, 23, 0x30);
        frame.save(c, 30, 0x20);
        frame.save_ra(c, 0x10);

        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx(oc683_off(c.g[S0][0], 0x130));
        c.g[S3][0] = sx(oc683_off(c.g[S0][0], 0x224));
        c.g[S2][0] = sx(oc683_off(c.g[S0][0], 0x284));
        c.g[S5][0] = sx(oc683_off(c.g[S0][0], 0x1c4));
        c.g[S6][0] = sx(oc683_off(c.g[S0][0], 0x254));
        c.g[S7][0] = sx(oc683_off(c.g[S0][0], 0x1f4));
        c.g[FP][0] = sx(oc683_off(c.g[S0][0], 0x164));
        let slot0 = frame.at(c, 0);
        c.sw(slot0, c.g[V0][0]);
        c.g[S4][0] = sx(oc683_off(c.g[S0][0], 0x2b8));
        // cMenu_cMenu(self): the base menu constructor (A0 is still self).
        call_ptr(c, 0x002c_aa58, 0x0024_9fd8)?;

        c.g[S1][0] = sx(oc683_off(c.g[S0][0], 0x2ec));
        c.g[V0][0] = sx(oc683_off(c.g[S0][0], 0x14c));
        c.g[A1][0] = 0x0048_0000;
        let slot4 = frame.at(c, 4);
        c.sw(slot4, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1a10));
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x588));
        let a12c = oc683_off(c.g[S0][0], 0x12c);
        c.sw(a12c, c.g[V0][0]);
        c.g[A0][0] = c.lw(slot0);
        call_ptr(c, 0x002c_d2a0, 0x0024_a000)?;
        c.g[A0][0] = c.lw(slot4);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_a00c)?;

        c.g[V0][0] = sx(oc683_off(c.g[S0][0], 0x194));
        c.g[T1][0] = 0x004a_0000;
        let slot8 = frame.at(c, 8);
        c.sw(slot8, c.g[V0][0]);
        c.g[A0][0] = c.g[FP][0];
        c.g[T1][0] = sx(0x004a_0000u32.wrapping_add(0x2d30));
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15c8));
        c.g[A3][0] = 0;
        c.g[T0][0] = 1;
        call_ptr(c, 0x002c_d8f8, 0x0024_a034)?;

        c.g[A0][0] = c.lw(slot8);
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19f8));
        c.g[A1][0] = 1;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15cc));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xc8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a054)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19e0));
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 2;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15d0));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xc8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a074)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b90));
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 3;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15d4));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x2710;
        call_ptr(c, 0x002c_d8f8, 0x0024_a094)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b80));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 4;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15d8));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x2710;
        call_ptr(c, 0x002c_d8f8, 0x0024_a0b4)?;

        c.g[T0][0] = 0x10000;
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19c8));
        c.g[T0][0] |= 0x86a0;
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = 1;
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15dc));
        c.g[A1][0] = 5;
        call_ptr(c, 0x002c_d8f8, 0x0024_a0d8)?;

        // Float arguments: f12 = *(gp - 0x4fec); f13 from the AT constant.
        c.lwc1(12, oc683_off(c.g[GP][0], -0x4fec));
        c.g[A3][0] = 0x0048_0000;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 13);
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0x19b8));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15e0));
        c.g[A1][0] = 6;
        call_ptr(c, 0x002c_dce8, 0x0024_a0fc)?;

        c.mtc1(0, 12);
        c.g[A3][0] = 0x0048_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S4][0];
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0x19a8));
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15e4));
        c.g[A1][0] = 7;
        call_ptr(c, 0x002c_dce8, 0x0024_a120)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1990));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x40;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx(oc683_off(c.g[GP][0], 0x15e8));
        c.g[A1][0] = 8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a140)?;

        // cMenu_addItem(self, item, -1) for the nine items, in order.
        c.g[A1][0] = c.lw(slot0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a150)?;
        c.g[A1][0] = c.lw(slot4);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a160)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a170)?;
        c.g[A1][0] = c.lw(slot8);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a180)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a190)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1a0)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1b0)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1c0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1d0)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1e0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x002c_bcc8, 0x0024_a1f0)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x90);
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_DeserializeSnapshot(Replay *self, char **pp)` (0x26d678):
/// copies the 0x28-byte snapshot header at `*pp` to the stack, stores the
/// frame fields into `self`, decompresses the payload and runs the empty
/// snapshot hook; returns 1. Frame 0x70: saves ra and s0-s2.
pub fn replay_deserialize_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x70);
        c.g[A2][0] = 0x28;
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 18, 0x40);

        // MEM_copy(hdr = sp, *pp, 0x28); then the header fields.
        c.g[A0][0] = c.g[SP][0];
        let s1 = c.g[S1][0] as u32;
        c.g[S2][0] = c.lw(s1);
        c.g[V0][0] = sx(oc683_off(c.g[S2][0], 0x28));
        c.g[A1][0] = c.g[S2][0];
        c.sw(s1, c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x0026_d6b0)?;

        let f4 = frame.at(c, 4);
        let f8 = frame.at(c, 8);
        let fc = frame.at(c, 0xc);
        let f14 = frame.at(c, 0x14);
        let f18 = frame.at(c, 0x18);
        let f1c = frame.at(c, 0x1c);
        let f20 = frame.at(c, 0x20);
        c.g[A3][0] = c.lw(f4);
        c.g[A2][0] = sx(oc683_off(c.g[S2][0], 0x24));
        c.g[V1][0] = c.lw(f8);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(fc);
        c.g[A0][0] = c.g[S0][0];
        c.sw(oc683_off(c.g[S0][0], 0x10), c.g[A3][0]);
        c.sw(oc683_off(c.g[S0][0], 0x30), c.g[V1][0]);
        c.sw(oc683_off(c.g[S0][0], 0x34), c.g[V0][0]);
        c.g[V0][0] = c.lw(f18);
        c.g[V1][0] = c.lw(f14);
        c.g[A3][0] = c.lw(f1c);
        c.g[T0][0] = c.lw(f20);
        // Unaligned word pair (lwl/lwr) at sp+0x10 .. sp+0x13, stored back
        // unaligned at self+0x1c .. 0x1f.
        let f13 = frame.at(c, 0x13);
        let f10 = frame.at(c, 0x10);
        c.lwl(T1, f13);
        c.lwr(T1, f10);
        let s1f = oc683_off(c.g[S0][0], 0x1f);
        let s1c = oc683_off(c.g[S0][0], 0x1c);
        c.swl(T1, s1f);
        c.swr(T1, s1c);
        c.sw(oc683_off(c.g[S0][0], 0x20), c.g[V1][0]);
        c.sw(oc683_off(c.g[S0][0], 0x28), c.g[A3][0]);
        c.sw(oc683_off(c.g[S0][0], 0x2c), c.g[T0][0]);
        c.sw(oc683_off(c.g[S0][0], 0x24), c.g[V0][0]);
        oc683_direct(0x0026_eab8).call(c, 0x0026_d708)?;

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        oc683_direct(0x0026_ed88).call(c, 0x0026_d714)?;

        frame.restore(c, 16, 0x60);
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_RestoreSnapshot(Replay* self, ReplayFrame* frame)`
/// (0x26db88): mark the frame used, then restore the world state from it.
pub fn replay_restore_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        oc683_direct(0x0026_ed90).call(c, 0x0026_dbac)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        oc683_direct(0x0026_dbd0).call(c, 0x0026_dbb8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WorldState_Restore(WorldState *self, ...)` (0x26dbd0): a tail
/// forwarder to `WorldState_Read` (`cReplay_restoreFrame`), 7 cycles.
pub fn world_state_restore(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(oc683_direct(0x0026_dbf0)),
        0x0026_dbe0,
    );
}

/// `void WorldState_Read(WorldState *self, Stream *s)` (0x26dbf0): restores
/// one replay frame from the stream `s` (a vtable read) into the world: the
/// buckets are merged and purged, the map reset, each subsystem's
/// `readFromReplayFrame` runs in turn, then the objects are notified and
/// the AI and game view updated. Frame 0x70: saves ra and s0-s2.
pub fn world_state_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S0][0] = sx(oc683_off(c.g[GP][0], 0x2898));
        frame.save(c, 18, 0x40);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);

        // Buckets: merge and purge the bucket manager, reset the world map.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0026_dc1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_5118, 0x0026_dc28)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_5118, 0x0026_dc34)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0x16c8));
        call_ptr(c, 0x003a_6800, 0x0026_dc3c)?;
        c.g[A0][0] = c.g[S1][0];
        oc683_direct(0x0026_e9b0).call(c, 0x0026_dc44)?;

        // Stream read (vtable slot 0x10): the frame header into sp.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.lh(oc683_off(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc683_off(c.g[V1][0], 0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dc60)?;

        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0031_7908, 0x0026_dc68)?;

        // Stream read again (vtable slot 0x10) into sp+0x20.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx(oc683_off(c.g[SP][0], 0x20));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 4;
        c.g[A0][0] = c.lh(oc683_off(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc683_off(c.g[V1][0], 0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dc88)?;

        // The same slot again, through A3, with the word at sp+0x20 stored
        // to the global at gp+0x2a88.
        c.g[A3][0] = c.lw(c.g[S1][0] as u32);
        c.g[A2][0] = 4;
        c.g[V0][0] = c.lw(oc683_off(c.g[SP][0], 0x20));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lh(oc683_off(c.g[A3][0], 0x10));
        c.g[V1][0] = c.lw(oc683_off(c.g[A3][0], 0x14));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let t = c.g[V1][0] as u32;
        let gp2a88 = oc683_off(c.g[GP][0], 0x2a88);
        c.sw(gp2a88, c.g[V0][0]);
        call_ptr(c, t, 0x0026_dcac)?;

        c.g[V0][0] = c.lw(oc683_off(c.g[SP][0], 0x20));
        let gp2a8c = oc683_off(c.g[GP][0], 0x2a8c);
        c.sw(gp2a8c, c.g[V0][0]);
        call_ptr(c, 0x002c_03e8, 0x0026_dcb8)?;

        // cBE_readFromReplayFrame(cBE_getBE(), s).
        call_ptr(c, 0x0014_dc80, 0x0026_dcc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0014_dfa8, 0x0026_dccc)?;

        // Vtable call on world->view (+0x84), slot 0x48, with A0 = view + delta.
        c.g[V1][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(oc683_off(c.g[V1][0], 0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh(oc683_off(c.g[V0][0], 0x48));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0x4c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dcec)?;

        // Vtable call on view->+0xc, slot 0x40 (the field at +0xcc).
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(oc683_off(c.g[A0][0], 0x84));
        c.g[A2][0] = c.lw(oc683_off(c.g[V0][0], 0xc));
        c.g[V1][0] = c.lw(oc683_off(c.g[A2][0], 0xcc));
        c.g[A0][0] = c.lh(oc683_off(c.g[V1][0], 0x40));
        c.g[V0][0] = c.lw(oc683_off(c.g[V1][0], 0x44));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dd10)?;

        // Object and stream subsystems: each reads from the replay frame.
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002d_9d68, 0x0026_dd18)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0xce8));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_bd20, 0x0026_dd24)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], -0x6f0));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0022_9e58, 0x0026_dd30)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0xf00));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_4240, 0x0026_dd3c)?;
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], 0xf38));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_7d28, 0x0026_dd48)?;

        // WorldState_ReadObjects(self, 1, s); WorldState_ReadDeadObjects(self, s).
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        oc683_direct(0x0026_ddc0).call(c, 0x0026_dd58)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        oc683_direct(0x0026_e340).call(c, 0x0026_dd64)?;

        // Field of the view's object (+0x84 -> +0xc -> +0xa4): the 0x103578 call.
        c.g[A0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(oc683_off(c.g[A0][0], 0x84));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0xc));
        c.g[A0][0] = c.lw(oc683_off(c.g[V1][0], 0xa4));
        call_ptr(c, 0x0010_3578, 0x0026_dd7c)?;

        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        oc683_direct(0x0026_de58).call(c, 0x0026_dd88)?;

        c.g[V0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0x84));
        c.g[A0][0] = c.lw(oc683_off(c.g[V1][0], 0xc));
        call_ptr(c, 0x0012_b788, 0x0026_dd98)?;

        c.g[V0][0] = c.lw(oc683_off(c.g[GP][0], -0x848));
        c.g[V1][0] = c.lw(oc683_off(c.g[V0][0], 0x84));
        c.g[A0][0] = c.lw(oc683_off(c.g[V1][0], 0x84));
        call_ptr(c, 0x0022_e840, 0x0026_dda8)?;

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cReplayFramePtr_loadBlock` (0x26eab8), listed as `ReplayStream_Decompress`:
/// reads a block tag from `src` and its size from `*pp`, advances `*pp` past
/// the 4-byte size, Huffman-decodes `size` bytes into the stream's block
/// (`self+4`), moves `*pp` past them, sets `self+8` (WriteData) to the end of
/// the decoded bytes and rewinds the stream. Returns 1.
pub fn replay_stream_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        // MEM_copy(&hdr.tag, src, 4)
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        let pp = c.g[S0][0] as u32;
        c.g[A1][0] = c.lw(pp);
        c.g[A0][0] = c.g[SP][0] | 0x4;
        c.g[A2][0] = 4;
        // MEM_copy(&hdr.size, *pp, 4)
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = c.lw(pp);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        let advanced = c.g[A0][0];
        c.sw(pp, advanced);
        // HUFF_decode(*pp, 0, self->Block)
        let self_ptr = c.g[S1][0] as u32;
        c.g[A2][0] = c.lw(self_ptr.wrapping_add(0x4));
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        let size_slot = sp(c).wrapping_add(0x4);
        c.g[V1][0] = c.lw(size_slot);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.lw(pp);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let moved = c.g[A1][0];
        c.sw(pp, moved);
        let self_ptr = c.g[S1][0] as u32;
        c.g[V1][0] = c.lw(self_ptr.wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let write_data = c.g[V1][0];
        c.sw(self_ptr.wrapping_add(0x8), write_data);
        oc683_direct(0x0026_e9b0).call(c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cReplay_popReplaySaveFrame` (0x26f850), listed as
/// `Replay_RestoreResumeSnapshot`: when the save frame (`self+0x3d0`) is set,
/// restores it, frees it and clears the slot.
pub fn replay_restore_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let save_frame = c.lw(this.wrapping_add(0x3d0));
        c.g[A1][0] = save_frame;
        if save_frame != 0 {
            c.cycles += 7;
            oc683_direct(0x0026_db88).call(c, 0x0026_f874)?;
            c.g[A1][0] = c.lw(this.wrapping_add(0x3d0));
            c.g[A0][0] = c.g[S0][0];
            oc683_direct(0x0027_0238).call(c, 0x0026_f880)?;
            c.sw(this.wrapping_add(0x3d0), 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::funcs::register_functions;
    use std::ptr::addr_of_mut;

    const OC683_CHILD_ENV: &str = "SSX3_OC683_CHILD";

    /// Logging stubs for the intra-shard callees the generated bodies call
    /// directly (through their override slots), and `oc683_install_stubs`
    /// putting them in the slots. Child process only: the stubs are not gated.
    macro_rules! oc683_stubs {
        ($($name:ident, $slot:ident, $addr:expr;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra)
                }
            )*
            fn oc683_install_stubs() {
                $(
                    fakes::hook(
                        addr_of_mut!(ee::s07::$slot),
                        $name,
                        fakes::prev_cell($name),
                    );
                )*
            }
        };
    }

    oc683_stubs! {
        oc683_stub_26d4d8, OVERRIDE_0026D4D8, 0x0026_d4d8;
        oc683_stub_26ddc0, OVERRIDE_0026DDC0, 0x0026_ddc0;
        oc683_stub_26de58, OVERRIDE_0026DE58, 0x0026_de58;
        oc683_stub_26e340, OVERRIDE_0026E340, 0x0026_e340;
        oc683_stub_26e448, OVERRIDE_0026E448, 0x0026_e448;
        oc683_stub_26e9b0, OVERRIDE_0026E9B0, 0x0026_e9b0;
        oc683_stub_26ed88, OVERRIDE_0026ED88, 0x0026_ed88;
        oc683_stub_26ed90, OVERRIDE_0026ED90, 0x0026_ed90;
        oc683_stub_26ee68, OVERRIDE_0026EE68, 0x0026_ee68;
        oc683_stub_26ef30, OVERRIDE_0026EF30, 0x0026_ef30;
        oc683_stub_26f4a0, OVERRIDE_0026F4A0, 0x0026_f4a0;
        oc683_stub_26f7f8, OVERRIDE_0026F7F8, 0x0026_f7f8;
        oc683_stub_270238, OVERRIDE_00270238, 0x0027_0238;
        oc683_stub_2702f8, OVERRIDE_002702F8, 0x0027_02f8;
        oc683_stub_28b180, OVERRIDE_0028B180, 0x0028_b180;
        oc683_stub_28f200, OVERRIDE_0028F200, 0x0028_f200;
    }

    /// Runs `body` in a child process whose function table is empty, so every
    /// cross-shard call lands in the fake hook, with the direct callees'
    /// slots stubbed (callees with no override slot cannot be faked in the
    /// test process). The parent re-runs the named test and checks it passed.
    fn oc683_in_child(test: &str, body: &dyn Fn()) {
        if std::env::var_os(OC683_CHILD_ENV).is_some() {
            oc683_install_stubs();
            return body();
        }
        let name = format!(
            "{}::{}",
            module_path!().split_once("::").map_or("", |p| p.1),
            test
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC683_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// Per-seed machine of the child tests: scrambled registers, the fake
    /// hooks (unwinding at a random call below `calls`) and an empty function
    /// table (set after `fresh` re-registered the generated one).
    fn oc683_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        register_functions(&[]);
        r
    }

    // The world the WorldState_* bodies read through `gp - 0x848`: the world
    // (view at +0x84), the view (vtable at +0, sub-object at +0xc), the
    // sub-object (vtable at +0xcc) and the replay stream (vtable at +0).
    const OC683_BASE: u32 = DATA + 0x20_0000;
    const OC683_WORLD: u32 = OC683_BASE + 0x100;
    const OC683_VIEW: u32 = OC683_BASE + 0x200;
    const OC683_VT_VIEW: u32 = OC683_BASE + 0x300;
    const OC683_SUB: u32 = OC683_BASE + 0x400;
    const OC683_VT_SUB: u32 = OC683_BASE + 0x500;
    const OC683_STREAM: u32 = OC683_BASE + 0x600;
    const OC683_VT_STREAM: u32 = OC683_BASE + 0x700;

    /// A `{ i16 delta; fn }` vtable entry at `vt + off`.
    fn oc683_slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, func: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, func);
    }

    /// The world chain; every virtual target is a fake address (the miss hook).
    fn oc683_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC683_WORLD);
        c.bus.write32(OC683_WORLD + 0x84, OC683_VIEW);
        c.bus.write32(OC683_VIEW, OC683_VT_VIEW);
        c.bus.write32(OC683_VIEW + 0xc, OC683_SUB);
        c.bus.write32(OC683_SUB + 0xcc, OC683_VT_SUB);
        c.bus.write32(OC683_STREAM, OC683_VT_STREAM);
        oc683_slot(c, OC683_VT_VIEW, 0x48, 0x20, fakes::FAKE + 0x50);
        oc683_slot(c, OC683_VT_SUB, 0x40, 0x30, fakes::FAKE + 0x40);
        oc683_slot(c, OC683_VT_STREAM, 0x10, 0x10, fakes::FAKE + 0x10);
    }

    /// `ShellRendererMenu_ctor(self)`: the base menu and nine items, built by
    /// the real menu constructors (as for the sibling `ScreenTintMenu_ctor`).
    #[test]
    fn shell_renderer_menu_ctor_matches() {
        check(
            ee::s06::f_00249f80__generated,
            shell_renderer_menu_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0024_9f80 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x0024_9f80;
            },
        );
    }

    // The playback manager's file table: count at +0x50, 12-byte entries at
    // the pointer at +0x58 (key, pack data, ...), the async handle at +0x78.
    const OC683_PM: u32 = DATA + 0x20_1000;
    const OC683_PM_ENTRIES: u32 = DATA + 0x20_2000;
    /// The result the `0x3df690` fake returns (the async handle).
    const OC683_PM_HANDLE: u32 = DATA + 0x20_3000;

    /// Three entries (keys 1 to 3, pack data set or not), a count of 0 to 3,
    /// a handle word that is negative or not, and a key of 0 to 4 (0 and 4
    /// never match).
    fn oc683_file_table(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(OC683_PM + 0x50, seed % 4);
        c.bus.write32(OC683_PM + 0x58, OC683_PM_ENTRIES);
        for i in 0..3u32 {
            c.bus.write32(OC683_PM_ENTRIES + 12 * i, i + 1);
            let pack = if r.next_u32() & 1 == 0 {
                0
            } else {
                r.next_u32() | 1
            };
            c.bus.write32(OC683_PM_ENTRIES + 12 * i + 4, pack);
        }
        let handle = if r.next_u32() & 1 == 0 {
            0xffff_fff0
        } else {
            r.next_u32() & 0x7fff_ffff
        };
        c.bus.write32(OC683_PM + 0x78, handle);
        c.g[A0][0] = sx(OC683_PM);
        c.g[A1][0] = (seed / 4 % 5) as u64;
    }

    /// Miss hook of the StartFileLoad test: a logged fake, except that the
    /// file API call (`0x3df690`) returns the handle word at
    /// `OC683_PM_HANDLE` sign-extended (the fakes cannot force a negative).
    fn oc683_sfl_miss(c: &mut Cpu, target: u32) {
        let handle = c.bus.read32(OC683_PM_HANDLE);
        fakes::fake_call(c, target);
        if target == 0x003d_f690 {
            c.g[V0][0] = sx(handle);
        }
    }

    /// `PlaybackMan_StartFileLoad(self, key)`: the async file open (`0x417828`
    /// builds the path, `0x3df690` starts the load), with its failure path.

    const OC683_REPLAY: u32 = DATA + 0x20_5000;
    const OC683_PP: u32 = DATA + 0x20_5100;
    const OC683_BUF: u32 = DATA + 0x20_5200;

    /// `Replay_DeserializeSnapshot(self, &p)`: copies the header to the stack,
    /// stores its fields into the replay, decompresses (the real
    /// `ReplayStream_Decompress`) and runs the snapshot hook.
    #[test]
    fn replay_deserialize_snapshot_matches() {
        oc683_in_child("replay_deserialize_snapshot_matches", &|| {
            check(
                ee::s07::f_0026d678__generated,
                replay_deserialize_snapshot,
                100,
                &|c, seed| {
                    let mut r = oc683_machine(c, seed, 0x0026_d678, 8);
                    for i in 0..16 {
                        c.bus.write32(OC683_REPLAY + 4 * i, r.next_u32());
                    }
                    c.bus.write32(OC683_PP, OC683_BUF);
                    for i in 0..16 {
                        c.bus.write32(OC683_BUF + 4 * i, r.next_u32());
                    }
                    c.g[A0][0] = sx(OC683_REPLAY);
                    c.g[A1][0] = sx(OC683_PP);
                    c.pc = 0x0026_d678;
                },
            );
        });
    }

    const OC683_STATE: u32 = DATA + 0x20_7000;

    /// `Replay_RestoreSnapshot(self, frame)`: the snapshot hook, then the real
    /// `WorldState_Restore` -> `WorldState_Read` (its callees are logged fakes).
    #[test]
    fn replay_restore_snapshot_matches() {
        oc683_in_child("replay_restore_snapshot_matches", &|| {
            check(
                ee::s07::f_0026db88__generated,
                replay_restore_snapshot,
                60,
                &|c, seed| {
                    let _ = oc683_machine(c, seed, 0x0026_db88, 40);
                    oc683_world(c);
                    c.g[A0][0] = sx(OC683_STATE);
                    c.g[A1][0] = sx(OC683_STREAM);
                    c.pc = 0x0026_db88;
                },
            );
        });
    }

    /// `WorldState_Restore(self, stream)`: forwards to `WorldState_Read`.
    #[test]
    fn world_state_restore_matches() {
        oc683_in_child("world_state_restore_matches", &|| {
            check(
                ee::s07::f_0026dbd0__generated,
                world_state_restore,
                60,
                &|c, seed| {
                    let _ = oc683_machine(c, seed, 0x0026_dbd0, 40);
                    oc683_world(c);
                    c.g[A0][0] = sx(OC683_STATE);
                    c.g[A1][0] = sx(OC683_STREAM);
                    c.pc = 0x0026_dbd0;
                },
            );
        });
    }

    /// `WorldState_Read(self, stream)`: one replay frame read back into the
    /// world, 30 callees, each a logged fake (unwinds at random calls).
    #[test]
    fn world_state_read_matches() {
        oc683_in_child("world_state_read_matches", &|| {
            check(
                ee::s07::f_0026dbf0__generated,
                world_state_read,
                120,
                &|c, seed| {
                    let _ = oc683_machine(c, seed, 0x0026_dbf0, 34);
                    oc683_world(c);
                    c.g[A0][0] = sx(OC683_STATE);
                    c.g[A1][0] = sx(OC683_STREAM);
                    c.pc = 0x0026_dbf0;
                },
            );
        });
    }

    // The stream of `ReplayStream_Decompress`: the block pointer at +4 and
    // the write pointer at +8, the `u8 **` cursor and the source tag.
    const OC683_RS_SELF: u32 = DATA + 0x20_8000;
    const OC683_RS_CURSOR: u32 = DATA + 0x20_8100;
    const OC683_RS_SRC: u32 = DATA + 0x20_8200;
    const OC683_RS_BLOCK: u32 = DATA + 0x20_9000;
    const OC683_RS_CODED: u32 = DATA + 0x20_9800;

    /// `ReplayStream_Decompress(self, &p, src)`: tag, size and Huffman body
    /// (the copies and the decoder are logged fakes), then the rewind.
    #[test]
    fn replay_stream_decompress_matches() {
        oc683_in_child("replay_stream_decompress_matches", &|| {
            check(
                ee::s07::f_0026eab8__generated,
                replay_stream_decompress,
                100,
                &|c, seed| {
                    let mut r = oc683_machine(c, seed, 0x0026_eab8, 4);
                    c.bus.write32(OC683_RS_SELF + 0x4, OC683_RS_BLOCK);
                    c.bus.write32(OC683_RS_SELF + 0x8, r.next_u32());
                    c.bus.write32(OC683_RS_CURSOR, OC683_RS_CODED);
                    c.bus.write32(OC683_RS_SRC, r.next_u32());
                    c.bus.write32(OC683_RS_CODED, 4 + seed % 8);
                    for i in 0..16 {
                        c.bus.write32(OC683_RS_CODED + 4 + 4 * i, r.next_u32());
                    }
                    c.g[A0][0] = sx(OC683_RS_SELF);
                    c.g[A1][0] = sx(OC683_RS_CURSOR);
                    c.g[A2][0] = sx(OC683_RS_SRC);
                    c.pc = 0x0026_eab8;
                },
            );
        });
    }

    /// `Replay_RestoreResumeSnapshot(self)`: with a resume snapshot at `+0x3d0`
    /// it is restored (the real `Replay_RestoreSnapshot` chain), freed and
    /// cleared; without one nothing is called.
    #[test]
    fn replay_restore_resume_snapshot_matches() {
        oc683_in_child("replay_restore_resume_snapshot_matches", &|| {
            check(
                ee::s07::f_0026f850__generated,
                replay_restore_resume_snapshot,
                60,
                &|c, seed| {
                    let _ = oc683_machine(c, seed, 0x0026_f850, 40);
                    oc683_world(c);
                    let snapshot = if seed % 3 == 0 { 0 } else { OC683_STREAM };
                    c.bus.write32(OC683_STATE + 0x3d0, snapshot);
                    c.g[A0][0] = sx(OC683_STATE);
                    c.pc = 0x0026_f850;
                },
            );
        });
    }

    const OC683_BP: u32 = DATA + 0x20_a000;
}
