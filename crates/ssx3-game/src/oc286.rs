//! Staging wave 210 (oc286): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_4850, sync_process_ctor, "SyncProcess_ctor"),
    (0x0010_a898, computer_actor_rider_save_state, "ComputerActorRider_SaveState"),
    (0x003b_b0c8, snd_next_handle_serial, "SND_NextHandleSerial"),
    (0x0035_46c8, game_process_dtor, "GameProcess_dtor"),
    (0x0013_9970, rider_handplant_state_save_state, "RiderHandplantState_SaveState"),
    (0x0012_f5b0, rider_reset_sub_save_state, "RiderResetSub_SaveState"),
    (0x0010_c140, computer_actor_rider_crouch_and_boost, "ComputerActorRider_CrouchAndBoost"),
    (0x0031_3ad8, anim_sequence_add_event_trigger, "AnimSequence_AddEventTrigger"),
    (0x0035_4808, typed_process_dtor, "TypedProcess_dtor"),
];

/// `TypedProcess_ctor` (0x354720).
const TYPED_PROCESS_CTOR: Callee = Callee { addr: 0x0035_4720, f: ee::s11::TypedProcess_ctor };
/// The `SyncProcess` vtable (0x491e00).
const SYNC_PROCESS_VTABLE: u32 = 0x0049_1e00;
/// Process field: vtable pointer.
const PROCESS_VTABLE: u32 = 0xc;
/// `SyncProcess` field: its serial id.
const SYNC_PROCESS_ID: u32 = 0x14;
/// The type of sync processes that take their ids from the second counter.
const SYNC_TYPE_SECONDARY: u64 = 6;
/// Next-id counters (gp-relative): the common one and the one for type 6.
const SYNC_NEXT_ID: u32 = 0x2a88;
const SYNC_NEXT_ID_TYPE6: u32 = 0x2a8c;

/// `SyncProcess* SyncProcess_ctor(SyncProcess* self, int priority, int type)`
/// (0x354850): constructs the typed process (registers it in the process
/// list), installs the `SyncProcess` vtable and gives it the next serial id,
/// type 6 from its own counter, all others from the common one. Returns
/// `self`.
pub fn sync_process_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        TYPED_PROCESS_CTOR.call(c, 0x0035_486c)?;
        let this = r32(c, S1);
        c.g[V1][0] = sx(SYNC_PROCESS_VTABLE);
        c.g[V0][0] = SYNC_TYPE_SECONDARY;
        let counter = if c.g[S0][0] == SYNC_TYPE_SECONDARY {
            c.cycles += 5;
            SYNC_NEXT_ID_TYPE6
        } else {
            c.cycles += 4;
            SYNC_NEXT_ID
        };
        Addr(this).off(PROCESS_VTABLE).sw(c, c.g[V1][0]);
        let at = Addr(r32(c, GP).wrapping_add(counter));
        let id = at.lw(c);
        c.g[V1][0] = id;
        c.g[V0][0] = sx((id as u32).wrapping_add(1));
        at.sw(c, c.g[V0][0]);
        c.cycles += 7;
        Addr(this).off(SYNC_PROCESS_ID).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderBase_SaveState` (0x111ac0).
const RIDER_BASE_SAVE_STATE: Callee = Callee { addr: 0x0011_1ac0, f: ee::s00::RiderBase_SaveState };
/// `ComputerActorRider` field: the AI-specific state block saved after the
/// rider base (0x150 bytes).
const CAR_AI_STATE: u32 = 0xdf0;
const CAR_AI_STATE_SIZE: u64 = 0x150;

/// `void ComputerActorRider_SaveState(ComputerActorRider* self, Stream* s)`
/// (0x10a898): saves the rider base state, then writes the AI rider's own
/// 0x150-byte block at +0xdf0 through the stream's write method.
pub fn computer_actor_rider_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        RIDER_BASE_SAVE_STATE.call(c, 0x0010_a8b4)?;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(CAR_AI_STATE));
        c.g[A2][0] = CAR_AI_STATE_SIZE;
        // stream->Write(self + 0xdf0, 0x150): vtable slot +0x8 `{delta, fn}`.
        let stream = r32(c, S0);
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0010_a8d0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The last handle serial handed out (a byte, 0x44c588 = 0x450000 - 0x3a78).
const SND_SERIAL_HI: u32 = 0x0045_0000;
const SND_SERIAL_LO: u32 = 0x3a78;
/// The sound voice table (0x50a8e8): voice count (i16) at +0x17e, voice
/// array at +0x1f0, 0x8c bytes per voice; a voice's in-use flag (i8) at
/// +0x69 and its handle serial (u8) at +0x26.
const SND_VOICES: u32 = 0x0051_0000 - 0x5718;
const SND_VOICE_COUNT: u32 = 0x17e;
const SND_VOICE_ARRAY: u32 = 0x1f0;
const SND_VOICE_SIZE: u32 = 0x8c;
const SND_VOICE_IN_USE: u32 = 0x69;
const SND_VOICE_SERIAL: u32 = 0x26;

/// `u8 SND_NextHandleSerial(void)` (0x3bb0c8): advances the 8-bit handle
/// serial (skipping 0) until no voice in use carries it, and returns it.
/// Both loops' back edges check for events (the inner loop at 0x3bb110, the
/// retry at 0x3bb0cc); the loop state lives in registers throughout.
pub fn snd_next_handle_serial(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = sx(SND_SERIAL_HI);
    'retry: loop {
        // Bump the serial; 0 is skipped.
        c.cycles += 6;
        let serial = Addr(r32(c, V0).wrapping_sub(SND_SERIAL_LO));
        c.g[V1][0] = serial.lbu(c);
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[V1][0] = c.g[A0][0] & 0xff;
        serial.sb(c, c.g[A0][0]);
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            serial.sb(c, c.g[V0][0]);
        }
        // Scan the voices for one in use with this serial.
        c.cycles += 5;
        c.g[V0][0] = sx(SND_VOICES);
        c.g[A1][0] = Addr(SND_VOICES).off(SND_VOICE_COUNT).lh(c);
        let none = (c.g[A1][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[A2][0] = c.g[A1][0];
            c.g[V1][0] = Addr(SND_VOICES).off(SND_VOICE_ARRAY).lw(c);
            c.g[A1][0] = Addr(r32(c, A3).wrapping_sub(SND_SERIAL_LO)).lbu(c);
            loop {
                c.cycles += 3;
                let voice = Addr(r32(c, V1));
                c.g[V0][0] = voice.off(SND_VOICE_IN_USE).lb(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = voice.off(SND_VOICE_SERIAL).lbu(c);
                    let taken = c.g[V0][0] == c.g[A1][0];
                    c.g[V0][0] = sx(SND_SERIAL_HI);
                    if taken {
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_b0cc);
                        }
                        continue 'retry;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(SND_VOICE_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_b110);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A3).wrapping_sub(SND_SERIAL_LO)).lbu(c);
        ret(c);
        return;
    }
}

/// `ProcessList_Remove` (0x354e48).
const PROCESS_LIST_REMOVE: Callee = Callee { addr: 0x0035_4e48, f: ee::s11::ProcessList_Remove };
/// `operator delete` (`__builtin_delete`, 0x317e50; called through the table).
const OC286_BUILTIN_DELETE: u32 = 0x0031_7e50;
/// The `GameProcess` vtable (0x491f00).
const GAME_PROCESS_VTABLE: u32 = 0x0049_1f00;
/// The global process list (gp-relative).
const PROCESS_LIST: u32 = 0x2898;

/// `void GameProcess_dtor(GameProcess* self, int flags)` (0x3546c8):
/// reinstalls the `GameProcess` vtable, unlinks the process from the global
/// process list and, with bit 0 of `flags` (deleting destructor), frees it.
pub fn game_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(GAME_PROCESS_VTABLE);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, S1)).off(PROCESS_VTABLE).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(PROCESS_LIST));
        c.g[A1][0] = c.g[S1][0];
        PROCESS_LIST_REMOVE.call(c, 0x0035_46f8)?;
        c.g[S0][0] &= 1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC286_BUILTIN_DELETE, 0x0035_470c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Saved sizes of the handplant state and the reset sub-state.
const HANDPLANT_STATE_SIZE: u64 = 0xa0;
const RESET_SUB_SIZE: u64 = 0x8;

/// `void RiderHandplantState_SaveState(RiderHandplantState* self, Stream* s)` (0x139970): writes
/// the handplant state's 0xa0 bytes to the save stream.
pub fn rider_handplant_state_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = HANDPLANT_STATE_SIZE;
        // s->Write(self, 0xa0): vtable slot +0x8 `{delta, fn}`.
        let stream = r32(c, A3);
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0013_9998)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderResetSub_SaveState(RiderResetSub* self, Stream* s)` (0x12f5b0): writes the
/// reset sub-state's 8 bytes to the save stream.
pub fn rider_reset_sub_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = RESET_SUB_SIZE;
        // s->Write(self, 8): vtable slot +0x8 `{delta, fn}`.
        let stream = r32(c, A3);
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0012_f5d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ComputerActorRider` field: its rider; the rider's velocity (vec4) at +0x1e0.
const CAR_RIDER: u32 = 0x18;
const RIDER_VELOCITY: u32 = 0x1e0;
/// `.lit4` (gp-relative): the speed below which the AI crouches fully
/// (833.33) and the crouch scale above it (0.97).
const CROUCH_FULL_BELOW: u32 = 0x7ddc;
const CROUCH_FAST_SCALE: u32 = 0x7dd8;
/// The AI pad word: the low 26 bits are kept, the crouch amount (0..31)
/// goes into bits 26..31, and bits 16/17 (crouch and boost) are set.
const PAD_KEEP_MASK: u64 = 0x03ff_ffff;
const PAD_CROUCH: u64 = 0x1_0000;
const PAD_BOOST: u64 = 0x2_0000;

/// `void ComputerActorRider_CrouchAndBoost(ComputerActorRider* self, Pad* pad)`
/// (0x10c140): presses crouch and boost on the AI's pad, with a crouch
/// amount of 31 (scaled by 0.97 when the rider's speed is at least 833.33).
/// The speed is the VU0 length of the rider's velocity (Q, as the original).
pub fn computer_actor_rider_crouch_and_boost(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V0][0] = Addr(r32(c, A0)).off(CAR_RIDER).lw(c);
    c.g[A2][0] = c.g[A1][0];
    let gp = r32(c, GP);
    let full_below = Addr(gp.wrapping_sub(CROUCH_FULL_BELOW)).read_fpu(c);
    crate::abi::set_fpr(c, 1, full_below);
    c.lqc2(3, r32(c, V0).wrapping_add(RIDER_VELOCITY));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    let speed = FpuF32(c.g[V1][0] as u32);
    crate::abi::set_fpr(c, 0, speed);
    let mut scale = Addr(gp.wrapping_sub(CROUCH_FAST_SCALE)).read_fpu(c);
    crate::abi::set_fpr(c, 2, scale);
    if crate::abi::c_lt_s(c, speed, full_below) {
        c.cycles += 2;
        c.g[AT][0] = 0x3f80_0000;
        scale = FpuF32::ONE;
        crate::abi::set_fpr(c, 2, scale);
    }
    c.cycles += 17;
    c.g[AT][0] = 0x41f8_0000;
    let amount = scale * FpuF32(0x41f8_0000); // * 31.0
    crate::abi::set_fpr(c, 0, amount);
    let pad = Addr(r32(c, A2));
    c.g[V1][0] = pad.lw(c) & PAD_KEEP_MASK;
    c.g[A1][0] = PAD_CROUCH;
    c.g[A0][0] = PAD_BOOST;
    let amount_i = FpuF32(amount.to_i32() as u32);
    crate::abi::set_fpr(c, 1, amount_i);
    c.g[V0][0] = sx((amount_i.0) << 26);
    c.g[V1][0] |= c.g[V0][0] | PAD_CROUCH | PAD_BOOST;
    pad.sw(c, c.g[V1][0]);
    ret(c);
}

/// `EA_MemAllocNamed` (0x317d70).
const OC286_MEM_ALLOC_NAMED: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed };
/// The allocation name of event triggers (0x48a5b0) and the flags passed.
const EVENT_TRIGGER_ALLOC_NAME: u32 = 0x0049_0000 - 0x5a50;
const EVENT_TRIGGER_ALLOC_FLAGS: u64 = 0x2000_0000;
/// `AnimSequence` field: head of its event-trigger list.
const ANIM_SEQ_TRIGGERS: u32 = 0xac;
/// Event-trigger node (0x10 bytes): key, active flag, a cleared word, next.
const TRIGGER_KEY: u32 = 0x0;
const TRIGGER_ACTIVE: u32 = 0x4;
const TRIGGER_CLEARED: u32 = 0x8;
const TRIGGER_NEXT: u32 = 0xc;
const TRIGGER_SIZE: u64 = 0x10;

/// `void AnimSequence_AddEventTrigger(AnimSequence* self, int key)`
/// (0x313ad8): allocates a 16-byte trigger node `{key, active = 1, 0,
/// next}` and pushes it on the front of the sequence's trigger list.
pub fn anim_sequence_add_event_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = EVENT_TRIGGER_ALLOC_FLAGS;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = TRIGGER_SIZE;
        c.g[A1][0] = sx(EVENT_TRIGGER_ALLOC_NAME);
        OC286_MEM_ALLOC_NAMED.call(c, 0x0031_3b08)?;
        let seq = Addr(r32(c, S0));
        let node = Addr(r32(c, V0));
        c.g[A0][0] = seq.off(ANIM_SEQ_TRIGGERS).lw(c);
        c.g[V1][0] = 1;
        seq.off(ANIM_SEQ_TRIGGERS).sw(c, c.g[V0][0]);
        node.off(TRIGGER_KEY).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        node.off(TRIGGER_ACTIVE).sw(c, c.g[V1][0]);
        node.off(TRIGGER_NEXT).sw(c, c.g[A0][0]);
        node.off(TRIGGER_CLEARED).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameProcess_dtor` (0x3546c8).
const GAME_PROCESS_DTOR: Callee = Callee { addr: 0x0035_46c8, f: ee::s11::GameProcess_dtor };
/// The `TypedProcess` vtable (0x491e80).
const TYPED_PROCESS_VTABLE: u32 = 0x0049_1e80;
/// `TypedProcess` field: its type (i16).
const TYPED_PROCESS_TYPE: u32 = 0x10;
/// Live processes per type (i32 each, 0x44aff0 = 0x450000 - 0x5010).
const PROCESS_TYPE_COUNTS_HI: u32 = 0x0045_0000;
const PROCESS_TYPE_COUNTS: u32 = PROCESS_TYPE_COUNTS_HI - 0x5010;

/// `void TypedProcess_dtor(TypedProcess* self, int flags)` (0x354808):
/// reinstalls the `TypedProcess` vtable, decrements the live count of the
/// process's type, then runs the `GameProcess` destructor (with `flags`).
pub fn typed_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(TYPED_PROCESS_VTABLE);
        frame.save_ra(c, 0);
        let this = Addr(r32(c, A0));
        this.off(PROCESS_VTABLE).sw(c, c.g[V0][0]);
        c.g[A2][0] = this.off(TYPED_PROCESS_TYPE).lh(c);
        c.g[V1][0] = sx(PROCESS_TYPE_COUNTS);
        c.g[A2][0] = sx((r32(c, A2) << 2).wrapping_add(PROCESS_TYPE_COUNTS));
        let count = Addr(r32(c, A2));
        c.g[V0][0] = sx((count.lw(c) as u32).wrapping_sub(1));
        count.sw(c, c.g[V0][0]);
        GAME_PROCESS_DTOR.call(c, 0x0035_4840)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// The process list at `gp + 0x2898`: its per-priority node array (two
    /// sub-lists per priority, chosen by bit 0 of the entry's first word) with
    /// valid tail pointers, and the per-type process counts at 0x44aff0.
    fn oc286_process_list(c: &mut Cpu, r: &mut Rng) {
        let nodes = DATA + 0x1000;
        c.bus.write32(GAME_GP + 0x2898 + 4, nodes);
        for i in 0..16u32 {
            let e = nodes + i * 0x44;
            c.bus.write32(e, r.next_u32());
            c.bus.write32(e + 0x8, DATA + 0x3000 + i * 0x10);
            c.bus.write32(e + 0x28, DATA + 0x3800 + i * 0x10);
            c.bus.write32(0x0044_aff0 + i * 4, r.next_u32() % 50);
        }
    }

    /// Both id counters, for type 6 and others, after a real typed-process
    /// construction (process-list insertion in both sub-lists).
    #[test]
    fn sync_process_ctor_matches() {
        check(ee::s11::f_00354850__generated, sync_process_ctor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x35_4850, 0);
            oc286_process_list(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (r.next_u32() % 16) as u64;
            c.g[A2][0] = if seed % 2 == 0 { 6 } else { (r.next_u32() % 16) as u64 };
            c.bus.write32(GAME_GP + SYNC_NEXT_ID, r.next_u32());
            c.bus.write32(GAME_GP + SYNC_NEXT_ID_TYPE6, r.next_u32());
            c.pc = 0x0035_4850;
        });
    }

    /// A `RiderBase` at DATA+0x200000 that `RiderBase_SaveState` can save
    /// (copied from `rider_base_save_state_matches`: zeroed part objects,
    /// targets present or not, path indices found or not) and a stream at
    /// DATA+0x10 whose write method (slot +0x8) is a fake. Returns
    /// `(rider_base, stream)`.
    fn oc286_rider_base_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> (u32, u32) {
        use crate::game::ai::computer::{
            RB_PATH_DATA, RB_RIDER, RB_RIDER_AIR_PREDICTOR, RB_RIDER_AI_PATH, RB_RIDER_ANIM,
            RB_RIDER_BLOCK0, RB_RIDER_BLOCK1, RB_RIDER_METRIX, RB_RIDER_MODEL, RB_RIDER_TARGET0,
            RB_RIDER_TARGET1, RB_RIDER_TRACK_PATH, RB_TARGET_ID,
        };
        use crate::difftest::fakes::vtable;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let stream = DATA + 0x10;
        let vt = vtable(c, r, &[0x8]);
        w(c, stream, vt);
        let this = DATA + 0x20_0000;
        crate::difftest::fill(c, r, this, 0xf40 / 4);
        let rider = DATA + 0x21_0000;
        w(c, this + RB_RIDER, rider);
        for (k, f) in [RB_RIDER_ANIM, RB_RIDER_MODEL, RB_RIDER_AIR_PREDICTOR, RB_RIDER_METRIX]
            .into_iter()
            .enumerate()
        {
            w(c, rider + f, DATA + 0x22_0000 + 0x1000 * k as u32);
        }
        w(c, DATA + 0x22_0000 + 0x50, DATA + 0x22_8000);
        for (k, f) in [RB_RIDER_TARGET0, RB_RIDER_TARGET1].into_iter().enumerate() {
            let cell = DATA + 0x23_0000 + 0x400 * k as u32;
            let obj = cell + 0x100;
            w(c, rider + f, cell);
            let present = (seed >> k) & 1 != 0;
            w(c, cell, if present { obj } else { 0 });
            let id = r.next_u32();
            w(c, obj + RB_TARGET_ID, id);
        }
        let b0 = r.next_u32();
        w(c, rider + RB_RIDER_BLOCK0, b0);
        let b1 = r.next_u32();
        w(c, rider + RB_RIDER_BLOCK1, b1);
        let n = seed % 5;
        let tracks = DATA + 0x24_0000;
        let ais = DATA + 0x25_0000;
        w(c, RB_PATH_DATA + 0x10, n);
        w(c, RB_PATH_DATA + 0x14, tracks);
        w(c, RB_PATH_DATA + 0x8, n);
        w(c, RB_PATH_DATA + 0xc, ais);
        for (f, base, stride) in [(RB_RIDER_TRACK_PATH, tracks, 0x3c), (RB_RIDER_AI_PATH, ais, 0x40)] {
            let v = match r.next_u32() % 3 {
                0 => 0,
                1 => base + stride * (r.next_u32() % 5),
                _ => r.next_u32(),
            };
            w(c, rider + f, v);
        }
        (this, stream)
    }

    /// The real rider-base save, then the AI block write through the
    /// stream's fake write method (unwinding at any of the calls).
    #[test]
    fn computer_actor_rider_save_state_matches() {
        check(ee::s00::f_0010a898__generated, computer_actor_rider_save_state, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x10_a898, 41);
            let (this, stream) = oc286_rider_base_fixture(c, &mut r, seed);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
            c.pc = 0x0010_a898;
        });
    }

    /// The voice table with `count` voices (some negative/zero counts), each
    /// in use or not, with serials just above the current one so that the
    /// search retries (including the wrap through 0).
    fn oc286_snd_voices(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let serial = if seed % 5 == 0 { 0xff } else { r.next_u32() as u8 };
        c.bus.write8(SND_SERIAL_HI - SND_SERIAL_LO, serial);
        let count: i16 = match seed % 7 {
            0 => -((r.next_u32() % 3) as i16),
            _ => (r.next_u32() % 10) as i16,
        };
        c.bus.write16(SND_VOICES + SND_VOICE_COUNT, count as u16);
        let voices = DATA + 0x4000;
        c.bus.write32(SND_VOICES + SND_VOICE_ARRAY, voices);
        for i in 0..10u32 {
            let v = voices + i * SND_VOICE_SIZE;
            let used = if r.next_u32() % 3 == 0 { 0 } else { r.next_u32() as u8 | 1 };
            c.bus.write8(v + SND_VOICE_IN_USE, used);
            let ser = serial.wrapping_add(1 + (r.next_u32() % 4) as u8);
            c.bus.write8(v + SND_VOICE_SERIAL, ser);
        }
    }

    /// Serial bump (with the wrap that skips 0) and the retry search over the
    /// voices in use.
    #[test]
    fn snd_next_handle_serial_matches() {
        check(ee::s13::f_003bb0c8__generated, snd_next_handle_serial, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_b0c8, 0);
            oc286_snd_voices(c, &mut r, seed);
            c.pc = 0x003b_b0c8;
        });
    }

    /// The same with events due at the back edges (delivered or unwinding).
    #[test]
    fn snd_next_handle_serial_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(ee::s13::f_003bb0c8__generated, snd_next_handle_serial, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_b0c9, 0);
            oc286_snd_voices(c, &mut r, seed | 1);
            // Enough voices for several back edges.
            let count = 3 + r.next_u32() % 7;
            c.bus.write16(SND_VOICES + SND_VOICE_COUNT, count as u16);
            let mode = schedule(c, &mut r, seed, 0x003b_b0c8);
            c.next_event = c.cycles + 10;
            mode
        });
    }

    /// A process in a real heap block (so `operator delete` can free it),
    /// linked into a list (prev/next at +4/+8, either may be null), with its
    /// type (+0x10) and the per-type counts. Returns the process.
    fn oc286_heap_process(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        use crate::game::render::particle::tests::heap_block;
        use crate::game::worldpainter::quadtree::tests::{heap_init, pool_allocator};
        pool_allocator(c);
        heap_init(c);
        let this = heap_block(c, 0x80);
        let prev = if seed % 3 == 0 { 0 } else { DATA + 0x5000 };
        let next = if seed % 5 == 0 { 0 } else { DATA + 0x5100 };
        c.bus.write32(this + 4, prev);
        c.bus.write32(this + 8, next);
        c.bus.write16(this + 0x10, (r.next_u32() % 16) as u16);
        for i in 0..16u32 {
            c.bus.write32(0x0044_aff0 + i * 4, r.next_u32() % 50);
        }
        this
    }

    /// Unlink from the process list (prev/next present or not) and the
    /// delete flag clear or set (a real heap free).
    #[test]
    fn game_process_dtor_matches() {
        check(ee::s11::f_003546c8__generated, game_process_dtor, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x35_46c8, 0);
            let this = oc286_heap_process(c, &mut r, seed);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx((seed & 3) as u32);
            c.pc = 0x0035_46c8;
        });
    }

    /// A state at DATA+0x800 saved to a stream at DATA+0x10 whose write
    /// method (slot +0x8, random `this` delta) is a fake that may unwind.
    fn oc286_save_block_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes::vtable;
        let mut r = machine(c, seed, salt, 1);
        let stream = DATA + 0x10;
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(DATA + 0x800);
        c.g[A1][0] = sx(stream);
        c.pc = entry;
    }

    #[test]
    fn rider_handplant_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_00139970__generated,
            rider_handplant_state_save_state,
            200,
            &|c, seed| oc286_save_block_setup(c, seed, 0x13_9970, 0x0013_9970),
        );
    }

    #[test]
    fn rider_reset_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012f5b0__generated,
            rider_reset_sub_save_state,
            200,
            &|c, seed| oc286_save_block_setup(c, seed, 0x12_f5b0, 0x0012_f5b0),
        );
    }

    /// Rider velocities whose length falls below or above 833.33 (and edge
    /// values), the `.lit4` constants, a random pad word.
    #[test]
    fn computer_actor_rider_crouch_and_boost_matches() {
        check(
            ee::s00::f_0010c140__generated,
            computer_actor_rider_crouch_and_boost,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c140, 0);
                let this = DATA;
                let rider = DATA + 0x400;
                let pad = DATA + 0x800;
                c.bus.write32(this + CAR_RIDER, rider);
                let mut comp = || if seed % 7 == 0 { r.edge_f32() } else { r.f32(700.0) };
                let v = [comp(), comp(), comp(), comp()];
                crate::difftest::put_vec(c, rider + RIDER_VELOCITY, v);
                c.bus.write32(GAME_GP - CROUCH_FULL_BELOW, 833.333_43f32.to_bits());
                c.bus.write32(GAME_GP - CROUCH_FAST_SCALE, 0.97f32.to_bits());
                let word = r.next_u32();
                c.bus.write32(pad, word);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pad);
                c.pc = 0x0010_c140;
            },
        );
    }

    /// A real allocation from the pools/heap, pushed on an empty or
    /// non-empty trigger list.
    #[test]
    fn anim_sequence_add_event_trigger_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        check(ee::s10::f_00313ad8__generated, anim_sequence_add_event_trigger, 100, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x31_3ad8);
            let seq = DATA + 0x100;
            let head = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(seq + ANIM_SEQ_TRIGGERS, head);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = sx(r.next_u32());
            c.pc = 0x0031_3ad8;
        });
    }

    /// Type-count decrement, then the real `GameProcess` destructor: unlink
    /// (prev/next present or not) and the delete flag clear or set.
    #[test]
    fn typed_process_dtor_matches() {
        check(ee::s11::f_00354808__generated, typed_process_dtor, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x35_4808, 0);
            let this = oc286_heap_process(c, &mut r, seed);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx((seed & 3) as u32);
            c.pc = 0x0035_4808;
        });
    }
}
