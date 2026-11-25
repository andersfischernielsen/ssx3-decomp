//! Staging module oc653 (wave g33.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc653`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
];

/// `base + off` as a guest address (`addiu`; `off` may be negative).
fn oc653_off(base: u64, off: i32) -> u32 {
    (base as u32).wrapping_add(off as u32)
}

/// `void ShellRendererMenu_ctor(cShellRendererMenu *self)` (0x249f80): the
/// base menu, then the shell renderer's items: nine groups of `IntMenuItem`
/// constructors (`cIntMenuItem_cIntMenuItem`, two of them with float
/// arguments), each added to the menu with `cMenu_addItem` (slot `index`, or
/// -1 for the end). The item objects sit at `self + 0x130 .. 0x2ec`.
/// Frame 0xb0: saves ra and s0-s7, fp.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
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
        c.g[V0][0] = sx(oc653_off(c.g[S0][0], 0x130));
        c.g[S3][0] = sx(oc653_off(c.g[S0][0], 0x224));
        c.g[S2][0] = sx(oc653_off(c.g[S0][0], 0x284));
        c.g[S5][0] = sx(oc653_off(c.g[S0][0], 0x1c4));
        c.g[S6][0] = sx(oc653_off(c.g[S0][0], 0x254));
        c.g[S7][0] = sx(oc653_off(c.g[S0][0], 0x1f4));
        c.g[FP][0] = sx(oc653_off(c.g[S0][0], 0x164));
        let slot0 = frame.at(c, 0);
        c.sw(slot0, c.g[V0][0]);
        c.g[S4][0] = sx(oc653_off(c.g[S0][0], 0x2b8));
        // cMenu_cMenu(self): the base menu constructor (A0 is still self).
        call_ptr(c, 0x002c_aa58, 0x0024_9fd8)?;

        c.g[S1][0] = sx(oc653_off(c.g[S0][0], 0x2ec));
        c.g[V0][0] = sx(oc653_off(c.g[S0][0], 0x14c));
        c.g[A1][0] = 0x0048_0000;
        let slot4 = frame.at(c, 4);
        c.sw(slot4, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1a10));
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x588));
        let a12c = oc653_off(c.g[S0][0], 0x12c);
        c.sw(a12c, c.g[V0][0]);
        c.g[A0][0] = c.lw(slot0);
        call_ptr(c, 0x002c_d2a0, 0x0024_a000)?;
        c.g[A0][0] = c.lw(slot4);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_a00c)?;

        c.g[V0][0] = sx(oc653_off(c.g[S0][0], 0x194));
        c.g[T1][0] = 0x0048_0000;
        let slot8 = frame.at(c, 8);
        c.sw(slot8, c.g[V0][0]);
        c.g[A0][0] = c.g[FP][0];
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x2d30));
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15c8));
        c.g[A3][0] = 0;
        c.g[T0][0] = 1;
        call_ptr(c, 0x002c_d8f8, 0x0024_a034)?;

        c.g[A0][0] = c.lw(slot8);
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19f8));
        c.g[A1][0] = 1;
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15cc));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xc8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a054)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19e0));
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 2;
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15d0));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xc8;
        call_ptr(c, 0x002c_d8f8, 0x0024_a074)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b90));
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 3;
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15d4));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x2710;
        call_ptr(c, 0x002c_d8f8, 0x0024_a094)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1b80));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 4;
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15d8));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x2710;
        call_ptr(c, 0x002c_d8f8, 0x0024_a0b4)?;

        c.g[T0][0] = 0x10000;
        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x19c8));
        c.g[T0][0] |= 0x86a0;
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = 1;
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15dc));
        c.g[A1][0] = 5;
        call_ptr(c, 0x002c_d8f8, 0x0024_a0d8)?;

        // Float arguments: f12 = *(gp - 0x4fec); f13 from the AT constant.
        c.lwc1(12, oc653_off(c.g[GP][0], -0x4fec));
        c.g[A3][0] = 0x0048_0000;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 13);
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0x19b8));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15e0));
        c.g[A1][0] = 6;
        call_ptr(c, 0x002c_dce8, 0x0024_a0fc)?;

        c.mtc1(0, 12);
        c.g[A3][0] = 0x0048_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S4][0];
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_sub(0x19a8));
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15e4));
        c.g[A1][0] = 7;
        call_ptr(c, 0x002c_dce8, 0x0024_a120)?;

        c.g[T1][0] = sx(0x0048_0000u32.wrapping_sub(0x1990));
        c.g[A3][0] = 1;
        c.g[T0][0] = 0x40;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx(oc653_off(c.g[GP][0], 0x15e8));
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
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
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
        c.g[V0][0] = sx(oc653_off(c.g[S2][0], 0x28));
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
        c.g[A2][0] = sx(oc653_off(c.g[S2][0], 0x24));
        c.g[V1][0] = c.lw(f8);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(fc);
        c.g[A0][0] = c.g[S0][0];
        c.sw(oc653_off(c.g[S0][0], 0x10), c.g[A3][0]);
        c.sw(oc653_off(c.g[S0][0], 0x30), c.g[V1][0]);
        c.sw(oc653_off(c.g[S0][0], 0x34), c.g[V0][0]);
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
        let s1f = oc653_off(c.g[S0][0], 0x1f);
        let s1c = oc653_off(c.g[S0][0], 0x1c);
        c.swl(T1, s1f);
        c.swr(T1, s1c);
        c.sw(oc653_off(c.g[S0][0], 0x20), c.g[V1][0]);
        c.sw(oc653_off(c.g[S0][0], 0x28), c.g[A3][0]);
        c.sw(oc653_off(c.g[S0][0], 0x2c), c.g[T0][0]);
        c.sw(oc653_off(c.g[S0][0], 0x24), c.g[V0][0]);
        call_ptr(c, 0x0026_eab8, 0x0026_d708)?;

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0026_ed88, 0x0026_d714)?;

        frame.restore(c, 16, 0x60);
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldState_Restore(WorldState *self, ...)` (0x26dbd0): a tail
/// forwarder to `WorldState_Read` (`cReplay_restoreFrame`), 7 cycles.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn world_state_restore(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0026_dbf0), 0x0026_dbe0);
}

/// `void WorldState_Read(WorldState *self, Stream *s)` (0x26dbf0): restores
/// one replay frame from the stream `s` (a vtable read) into the world: the
/// buckets are merged and purged, the map reset, each subsystem's
/// `readFromReplayFrame` runs in turn, then the objects are notified and
/// the AI and game view updated. Frame 0x70: saves ra and s0-s2.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn world_state_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S0][0] = sx(oc653_off(c.g[GP][0], 0x2898));
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
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], 0x16c8));
        call_ptr(c, 0x003a_6800, 0x0026_dc3c)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0026_e9b0, 0x0026_dc44)?;

        // Stream read (vtable slot 0x10): the frame header into sp.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.lh(oc653_off(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc653_off(c.g[V1][0], 0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dc60)?;

        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0031_7908, 0x0026_dc68)?;

        // Stream read again (vtable slot 0x10) into sp+0x20.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx(oc653_off(c.g[SP][0], 0x20));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 4;
        c.g[A0][0] = c.lh(oc653_off(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc653_off(c.g[V1][0], 0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dc88)?;

        // The same slot again, through A3, with the word at sp+0x20 stored
        // to the global at gp+0x2a88.
        c.g[A3][0] = c.lw(c.g[S1][0] as u32);
        c.g[A2][0] = 4;
        c.g[V0][0] = c.lw(oc653_off(c.g[SP][0], 0x20));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lh(oc653_off(c.g[A3][0], 0x10));
        c.g[V1][0] = c.lw(oc653_off(c.g[A3][0], 0x14));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let t = c.g[V1][0] as u32;
        let gp2a88 = oc653_off(c.g[GP][0], 0x2a88);
        c.sw(gp2a88, c.g[V0][0]);
        call_ptr(c, t, 0x0026_dcac)?;

        c.g[V0][0] = c.lw(oc653_off(c.g[SP][0], 0x20));
        let gp2a8c = oc653_off(c.g[GP][0], 0x2a8c);
        c.sw(gp2a8c, c.g[V0][0]);
        call_ptr(c, 0x002c_03e8, 0x0026_dcb8)?;

        // cBE_readFromReplayFrame(cBE_getBE(), s).
        call_ptr(c, 0x0014_dc80, 0x0026_dcc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0014_dfa8, 0x0026_dccc)?;

        // Vtable call on world->view (+0x84), slot 0x48, with A0 = view + delta.
        c.g[V1][0] = c.lw(oc653_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(oc653_off(c.g[V1][0], 0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh(oc653_off(c.g[V0][0], 0x48));
        c.g[V1][0] = c.lw(oc653_off(c.g[V0][0], 0x4c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dcec)?;

        // Vtable call on view->+0xc, slot 0x40 (the field at +0xcc).
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(oc653_off(c.g[A0][0], 0x84));
        c.g[A2][0] = c.lw(oc653_off(c.g[V0][0], 0xc));
        c.g[V1][0] = c.lw(oc653_off(c.g[A2][0], 0xcc));
        c.g[A0][0] = c.lh(oc653_off(c.g[V1][0], 0x40));
        c.g[V0][0] = c.lw(oc653_off(c.g[V1][0], 0x44));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0026_dd10)?;

        // Object and stream subsystems: each reads from the replay frame.
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002d_9d68, 0x0026_dd18)?;
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], 0xce8));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_bd20, 0x0026_dd24)?;
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], -0x6f0));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0022_9e58, 0x0026_dd30)?;
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], 0xf00));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_4240, 0x0026_dd3c)?;
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], 0xf38));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_7d28, 0x0026_dd48)?;

        // WorldState_ReadObjects(self, 1, s); WorldState_ReadDeadObjects(self, s).
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0026_ddc0, 0x0026_dd58)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0026_e340, 0x0026_dd64)?;

        // Field of the view's object (+0x84 -> +0xc -> +0xa4): the 0x103578 call.
        c.g[A0][0] = c.lw(oc653_off(c.g[GP][0], -0x848));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw(oc653_off(c.g[A0][0], 0x84));
        c.g[V1][0] = c.lw(oc653_off(c.g[V0][0], 0xc));
        c.g[A0][0] = c.lw(oc653_off(c.g[V1][0], 0xa4));
        call_ptr(c, 0x0010_3578, 0x0026_dd7c)?;

        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0026_de58, 0x0026_dd88)?;

        c.g[V0][0] = c.lw(oc653_off(c.g[GP][0], -0x848));
        c.g[V1][0] = c.lw(oc653_off(c.g[V0][0], 0x84));
        c.g[A0][0] = c.lw(oc653_off(c.g[V1][0], 0xc));
        call_ptr(c, 0x0012_b788, 0x0026_dd98)?;

        c.g[V0][0] = c.lw(oc653_off(c.g[GP][0], -0x848));
        c.g[V1][0] = c.lw(oc653_off(c.g[V0][0], 0x84));
        c.g[A0][0] = c.lw(oc653_off(c.g[V1][0], 0x84));
        call_ptr(c, 0x0022_e840, 0x0026_dda8)?;

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, fall_back, hook, machine, prev_cell, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fixture region (inside RAM, away from the stack, the fake log at
    /// DATA+0x100000 and the pool).
    const BASE: u32 = DATA + 0x2_0000;
    /// Gate word: the gated fakes answer only when the test wrote MAGIC here.
    const CTRL: u32 = BASE;
    const MAGIC: u32 = 0x6536_0653;
    /// The object under construction (ShellRendererMenu_ctor's `self`).
    const OBJ: u32 = BASE + 0x1000;
    /// World (at gp-0x848), its view object, the view's vtable, and the
    /// sub-object (vtable at +0xcc) the replay reads from.
    const WORLD: u32 = BASE + 0x2000;
    const VIEW: u32 = BASE + 0x2100;
    const VT_VIEW: u32 = BASE + 0x2200;
    const SUB: u32 = BASE + 0x2300;
    const VT_SUB: u32 = BASE + 0x2400;
    /// The replay stream (a1 of WorldState_Read) and its vtable.
    const STREAM: u32 = BASE + 0x2500;
    const VT_STREAM: u32 = BASE + 0x2600;
    /// Snapshot pointer cell (`char **pp`) and its buffer for Replay_DeserializeSnapshot.
    const PP: u32 = BASE + 0x4000;
    const BUF: u32 = BASE + 0x4100;

    /// Fakes for the callees that have an override slot: each runs the fake
    /// (logs its registers, returns or unwinds) while the gate is set, and
    /// the body it replaced otherwise.
    macro_rules! oc653_fake {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CTRL) != MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                fake_call(c, $addr);
            }
        };
    }

    oc653_fake!(oc653_fake_002caa58, 0x002c_aa58, ee::s09::f_002caa58);
    oc653_fake!(oc653_fake_002cd8f8, 0x002c_d8f8, ee::s09::f_002cd8f8);
    oc653_fake!(oc653_fake_002cdce8, 0x002c_dce8, ee::s09::f_002cdce8);
    oc653_fake!(oc653_fake_002cbcc8, 0x002c_bcc8, ee::s09::f_002cbcc8);
    oc653_fake!(oc653_fake_0026d5e8, 0x0026_d5e8, ee::s07::f_0026d5e8);
    oc653_fake!(oc653_fake_0026ed88, 0x0026_ed88, ee::s07::f_0026ed88);
    oc653_fake!(oc653_fake_0026ed90, 0x0026_ed90, ee::s07::f_0026ed90);
    oc653_fake!(oc653_fake_0026e9b0, 0x0026_e9b0, ee::s07::f_0026e9b0);
    oc653_fake!(oc653_fake_0026ddc0, 0x0026_ddc0, ee::s07::f_0026ddc0);
    oc653_fake!(oc653_fake_0026e340, 0x0026_e340, ee::s07::f_0026e340);
    oc653_fake!(oc653_fake_0026de58, 0x0026_de58, ee::s07::f_0026de58);
    oc653_fake!(oc653_fake_003e6574, 0x003e_6574, ee::s14::f_003e6574);
    oc653_fake!(oc653_fake_003a6800, 0x003a_6800, ee::s13::f_003a6800);
    oc653_fake!(oc653_fake_00354c98, 0x0035_4c98, ee::s11::f_00354c98);
    oc653_fake!(oc653_fake_00355118, 0x0035_5118, ee::s11::f_00355118);
    oc653_fake!(oc653_fake_00344240, 0x0034_4240, ee::s11::f_00344240);
    oc653_fake!(oc653_fake_00357d28, 0x0035_7d28, ee::s11::f_00357d28);
    oc653_fake!(oc653_fake_0014dc80, 0x0014_dc80, ee::s01::f_0014dc80);
    oc653_fake!(oc653_fake_0014dfa8, 0x0014_dfa8, ee::s01::f_0014dfa8);
    oc653_fake!(oc653_fake_00103578, 0x0010_3578, ee::s00::f_00103578);
    oc653_fake!(oc653_fake_0012b788, 0x0012_b788, ee::s00::f_0012b788);
    oc653_fake!(oc653_fake_00229e58, 0x0022_9e58, ee::s05::f_00229e58);
    oc653_fake!(oc653_fake_0022e840, 0x0022_e840, ee::s05::f_0022e840);
    oc653_fake!(oc653_fake_00317d70, 0x0031_7d70, ee::s10::f_00317d70);
    oc653_fake!(oc653_fake_00316898, 0x0031_6898, ee::s10::f_00316898);
    oc653_fake!(oc653_fake_00316978, 0x0031_6978, ee::s10::f_00316978);
    oc653_fake!(oc653_fake_00244400, 0x0024_4400, ee::s06::f_00244400);
    oc653_fake!(oc653_fake_00244408, 0x0024_4408, ee::s06::f_00244408);
    oc653_fake!(oc653_fake_00417828, 0x0041_7828, ee::s15::f_00417828);

    /// Put the replaced-body fakes in their override slots (once per process).
    fn oc653_hook_all() {
        hook(addr_of_mut!(ee::s09::OVERRIDE_002CAA58), oc653_fake_002caa58, prev_cell(oc653_fake_002caa58));
        hook(addr_of_mut!(ee::s09::OVERRIDE_002CD8F8), oc653_fake_002cd8f8, prev_cell(oc653_fake_002cd8f8));
        hook(addr_of_mut!(ee::s09::OVERRIDE_002CDCE8), oc653_fake_002cdce8, prev_cell(oc653_fake_002cdce8));
        hook(addr_of_mut!(ee::s09::OVERRIDE_002CBCC8), oc653_fake_002cbcc8, prev_cell(oc653_fake_002cbcc8));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026D5E8), oc653_fake_0026d5e8, prev_cell(oc653_fake_0026d5e8));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026ED88), oc653_fake_0026ed88, prev_cell(oc653_fake_0026ed88));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026ED90), oc653_fake_0026ed90, prev_cell(oc653_fake_0026ed90));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026E9B0), oc653_fake_0026e9b0, prev_cell(oc653_fake_0026e9b0));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026DDC0), oc653_fake_0026ddc0, prev_cell(oc653_fake_0026ddc0));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026E340), oc653_fake_0026e340, prev_cell(oc653_fake_0026e340));
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026DE58), oc653_fake_0026de58, prev_cell(oc653_fake_0026de58));
        hook(addr_of_mut!(ee::s14::OVERRIDE_003E6574), oc653_fake_003e6574, prev_cell(oc653_fake_003e6574));
        hook(addr_of_mut!(ee::s13::OVERRIDE_003A6800), oc653_fake_003a6800, prev_cell(oc653_fake_003a6800));
        hook(addr_of_mut!(ee::s11::OVERRIDE_00354C98), oc653_fake_00354c98, prev_cell(oc653_fake_00354c98));
        hook(addr_of_mut!(ee::s11::OVERRIDE_00355118), oc653_fake_00355118, prev_cell(oc653_fake_00355118));
        hook(addr_of_mut!(ee::s11::OVERRIDE_00344240), oc653_fake_00344240, prev_cell(oc653_fake_00344240));
        hook(addr_of_mut!(ee::s11::OVERRIDE_00357D28), oc653_fake_00357d28, prev_cell(oc653_fake_00357d28));
        hook(addr_of_mut!(ee::s01::OVERRIDE_0014DC80), oc653_fake_0014dc80, prev_cell(oc653_fake_0014dc80));
        hook(addr_of_mut!(ee::s01::OVERRIDE_0014DFA8), oc653_fake_0014dfa8, prev_cell(oc653_fake_0014dfa8));
        hook(addr_of_mut!(ee::s00::OVERRIDE_00103578), oc653_fake_00103578, prev_cell(oc653_fake_00103578));
        hook(addr_of_mut!(ee::s00::OVERRIDE_0012B788), oc653_fake_0012b788, prev_cell(oc653_fake_0012b788));
        hook(addr_of_mut!(ee::s05::OVERRIDE_00229E58), oc653_fake_00229e58, prev_cell(oc653_fake_00229e58));
        hook(addr_of_mut!(ee::s05::OVERRIDE_0022E840), oc653_fake_0022e840, prev_cell(oc653_fake_0022e840));
        hook(addr_of_mut!(ee::s10::OVERRIDE_00317D70), oc653_fake_00317d70, prev_cell(oc653_fake_00317d70));
        hook(addr_of_mut!(ee::s10::OVERRIDE_00316898), oc653_fake_00316898, prev_cell(oc653_fake_00316898));
        hook(addr_of_mut!(ee::s10::OVERRIDE_00316978), oc653_fake_00316978, prev_cell(oc653_fake_00316978));
        hook(addr_of_mut!(ee::s06::OVERRIDE_00244400), oc653_fake_00244400, prev_cell(oc653_fake_00244400));
        hook(addr_of_mut!(ee::s06::OVERRIDE_00244408), oc653_fake_00244408, prev_cell(oc653_fake_00244408));
        hook(addr_of_mut!(ee::s15::OVERRIDE_00417828), oc653_fake_00417828, prev_cell(oc653_fake_00417828));
    }

    /// Write a `{int16 delta; fn}` pair at `vt + off` (the layout `VSlot` reads).
    fn oc653_slot(c: &mut Cpu, vt: u32, off: u32, delta: u16, func: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, func);
    }

    /// The world chain the WorldState_* bodies read through gp-0x848: the
    /// view's vtable (slot 0x48 and 0x40 targets on the sub-object), and the
    /// replay stream's vtable (slot 0x10). Every indirect target is a fake
    /// address (the machine's miss hook).
    fn oc653_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(WORLD + 0x84, VIEW);
        c.bus.write32(VIEW, VT_VIEW);
        c.bus.write32(VIEW + 0xc, SUB);
        c.bus.write32(SUB + 0xcc, VT_SUB);
        c.bus.write32(STREAM, VT_STREAM);
        oc653_slot(c, VT_VIEW, 0x48, 0x20, FAKE + 0x50);
        oc653_slot(c, VT_SUB, 0x40, 0x30, FAKE + 0x40);
        oc653_slot(c, VT_STREAM, 0x10, 0x10, FAKE + 0x10);
    }








}
