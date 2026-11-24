//! Staging module oc541 (wave 231, list s2): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc541`.
//!
//! Four of the five functions on the list already have hand-written bodies in
//! `game/replay/replaycache.rs` (`replay_recycle_oldest_snapshot`,
//! `replay_step_frame`, `replay_on_rider_event`, `replay_find_weakest_highlight`)
//! that are not installed in `overrides.txt`. They get differential tests here
//! (against the existing bodies, no second `OVERRIDES` entry), so the orchestrator
//! can install them. Only `Replay_SaveWriteSnapshot` (0x271380) is ported here.

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
    0x0027_1380,
    replay_save_write_snapshot,
    "Replay_SaveWriteSnapshot",
)];

/// Field address `base + off`, `base` being a 64-bit register value.
#[inline]
fn replay_field_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// The epilogue of `Replay_SaveWriteSnapshot` (frame 0x30, `ra` at +0x10).
fn write_snapshot_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `void cReplay_writeFrames(cReplay* self)` (0x271380): write the next piece
/// of the save stream. A pending encoded block is Huffman-encoded and emitted;
/// otherwise the next frame of the write queue is emitted as a header (with its
/// block queued); with no frame left the write state advances.
pub fn replay_save_write_snapshot(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x660));
        c.g[A3][0] = c.g[SP][0];
        if c.g[A0][0] != 0 {
            // A block is pending: encode it into WriteTmp + 4, store the size
            // word, emit the block and clear WriteBlockData.
            c.cycles += 13;
            c.g[A2][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
            c.g[A1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x664));
            c.g[A2][0] = sx(replay_field_ea(c.g[A2][0], 4));
            c.g[RA][0] = sx(0x0027_13b0);
            c.sw(c.g[SP][0] as u32, 0);
            call_ptr(c, 0x002c_a100, 0x0027_13b0)?;
            c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
            c.g[A2][0] = sx(replay_field_ea(c.g[V0][0], 4));
            c.g[A0][0] = c.g[S0][0];
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[RA][0] = sx(0x0027_13c8);
            c.g[A1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
            Callee {
                addr: 0x0027_1058,
                f: ee::s07::f_00271058,
            }
            .call(c, 0x0027_13c8)?;
            c.sw(replay_field_ea(c.g[S0][0], 0x660), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }

        // No block pending: the write queue (WriteFrame).
        c.cycles += 3;
        c.g[A0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[A1][0] = 1;
        if c.g[A0][0] == 0 {
            // Queue empty: advance the write state and clear the source.
            c.cycles += 5;
            c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x638));
            c.sw(replay_field_ea(c.g[S0][0], 0x644), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(replay_field_ea(c.g[S0][0], 0x648), 0);
            c.sw(replay_field_ea(c.g[S0][0], 0x638), c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }

        // A frame: count it, queue its block, and emit its header.
        c.cycles += 50;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x668));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[RA][0] = sx(0x0027_13ec);
        c.sw(replay_field_ea(c.g[S0][0], 0x668), c.g[V0][0]);
        Callee {
            addr: 0x0026_ed90,
            f: ee::s07::f_0026ed90,
        }
        .call(c, 0x0027_13ec)?;
        c.g[A3][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
        c.g[V0][0] = 0x1111_1113;
        c.g[T1][0] = 0x1111_1111;
        c.sw(c.g[A3][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A3][0];
        c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[A2][0] = 0x28;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x30));
        c.sw(replay_field_ea(c.g[A3][0], 0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x34));
        c.sw(replay_field_ea(c.g[A3][0], 0xc), c.g[V0][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[T0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x10));
        c.sw(replay_field_ea(c.g[A3][0], 0x24), c.g[T1][0]);
        c.sw(replay_field_ea(c.g[A3][0], 0x4), c.g[T0][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x20));
        c.sw(replay_field_ea(c.g[A3][0], 0x14), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x24));
        c.sw(replay_field_ea(c.g[A3][0], 0x18), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x28));
        c.sw(replay_field_ea(c.g[A3][0], 0x1c), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x2c));
        c.sw(replay_field_ea(c.g[A3][0], 0x20), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x28));
        c.sw(replay_field_ea(c.g[A3][0], 0x1c), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x2c));
        Callee {
            addr: 0x0027_1058,
            f: ee::s07::f_00271058,
        }
        .call(c, 0x0027_1484)?;
        // The frame's block becomes the pending one; the queue moves on.
        c.g[A0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V0][0] = c.lw(replay_field_ea(c.g[A0][0], 0x4));
        c.sw(replay_field_ea(c.g[S0][0], 0x660), c.g[V0][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[A0][0], 0x10));
        c.sw(replay_field_ea(c.g[S0][0], 0x664), c.g[V1][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[A0][0], 0x14));
        let no_next = c.g[V1][0] == 0;
        c.sw(replay_field_ea(c.g[S0][0], 0x65c), c.g[V1][0]);
        if no_next {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x3d0));
        if c.g[V1][0] != c.g[V0][0] {
            // The next frame is not the save frame: leave without the +1.
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x14));
        c.sw(replay_field_ea(c.g[S0][0], 0x65c), c.g[V0][0]);
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        write_snapshot_return(c, frame);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::replay::replaycache::tests::replay_find_rider_fixture;

    /// `Replay_RecycleOldestSnapshot` (existing hand-written body in
    /// `replaycache.rs`): a short free list with used and locked nodes; the
    /// generated dispatcher is entered at its head.

    /// `Replay_StepFrame` (existing hand-written body in `replaycache.rs`)
    /// with `reset == 0` and the tick before the end of the record: the frame
    /// cursor (skipping the save frame) and the tick advance.
    #[test]
    fn oc541_step_frame_matches() {
        check(
            ee::s07::f_002703f0,
            crate::game::replay::replaycache::replay_step_frame,
            200,
            &|c, seed| {
                let mut r = Rng(0x2703_f000 ^ seed as u64);
                scramble(c, &mut r);
                let save = DATA + 0x2000;
                let frame = match seed % 3 {
                    0 => 0,
                    1 => save,
                    _ => DATA + 0x3000,
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                let tick = r.next_u32() % 1000;
                c.bus.write32(DATA + 0x3d0, save);
                c.bus.write32(DATA + 0x488, frame);
                c.bus.write32(save + 0x18, DATA + 0x4000);
                c.bus.write32(DATA + 0x484, tick);
                c.bus.write32(DATA + 0x3cc, tick + 1 + seed % 2);
            },
        );
    }

    /// `Replay_OnRiderEvent` (existing hand-written body in `replaycache.rs`):
    /// each busy flag leaves at once; otherwise `Replay_FindRiderIndex` runs
    /// (the rider fixture, with the world at gp-0x848) and the event's mode
    /// decides the callee. The self object lives away from the world.

    /// `Replay_FindWeakestHighlight` (existing hand-written body in
    /// `replaycache.rs`): a leaf scan over 0-4 nodes, bit `i` in 0..3, the
    /// positive and negative branches, busy and clear nodes, a best of 0 or a
    /// node.

    /// `Replay_SaveWriteSnapshot` with no pending block and an empty write
    /// queue: the write state advances and the source is cleared (the queue
    /// and block paths need the encoder and the emit, covered by their own tests).
    #[test]
    fn oc541_save_write_snapshot_matches() {
        check(
            ee::s07::f_00271380__generated,
            replay_save_write_snapshot,
            200,
            &|c, seed| {
                let mut r = Rng(0x2713_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x660, 0);
                c.bus.write32(DATA + 0x65c, 0);
                c.bus.write32(DATA + 0x638, r.next_u32());
                c.bus.write32(DATA + 0x644, r.next_u32());
                c.bus.write32(DATA + 0x648, r.next_u32());
            },
        );
    }
}
