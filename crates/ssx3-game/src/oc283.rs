//! Staging wave 210 (oc283): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0013_3128, rider_spin_sub_enter, "RiderSpinSub_Enter"),
    (0x002a_cf60, bank_monitor_find_free_instance, "BankMonitor_FindFreeInstance"),
    (0x003d_54c8, path_update_tick20, "PATH_UpdateTick20"),
    (0x0013_f848, rider_ground_state_save_state, "RiderGroundState_SaveState"),
    (0x0013_2840, rider_railslide_sub_save_state, "RiderRailslideSub_SaveState"),
    (0x0030_bc80, wscript_man_save_process_list, "WScriptMan_SaveProcessList"),
    (0x0013_4cb0, rider_spin_sub_exit, "RiderSpinSub_Exit"),
    (0x0010_cf68, computer_actor_rider_pick_uber_trick, "ComputerActorRider_PickUberTrick"),
];

// ---------------------------------------------------------------------------
// RiderSpinSub_Enter (0x133128)

/// RiderSpinSub: state (1 spinning, 0 not), next state (-1 none) at +0x8,
/// the exit state at +0xc.
const SPIN_ENTER_STATE: u32 = 0x0;
const SPIN_ENTER_NEXT: u32 = 0x8;
const SPIN_ENTER_EXIT: u32 = 0xc;
/// Spin and flip rates (radians per tick), the spin factor.
const SPIN_ENTER_SPIN_RATE: u32 = 0x38;
const SPIN_ENTER_FLIP_RATE: u32 = 0x3c;
const SPIN_ENTER_FACTOR: u32 = 0x40;
/// The rider.
const SPIN_ENTER_RIDER: u32 = 0x58;
/// Rider: stick x and y at the jump, the spin and flip rates the
/// animation reads, the animation controller, player and character.
const SPIN_ENTER_STICK_X: u32 = 0x2a4;
const SPIN_ENTER_STICK_Y: u32 = 0x2b0;
const SPIN_ENTER_RIDER_SPIN_RATE: u32 = 0x2dc;
const SPIN_ENTER_RIDER_FLIP_RATE: u32 = 0x2e0;
const SPIN_ENTER_ANIM_CTRL: u32 = 0x784;
const SPIN_ENTER_PLAYER: u32 = 0x86c;
const SPIN_ENTER_CHARACTER: u32 = 0xb34;
/// Animation sequence: playback speed.
const SPIN_ENTER_SEQ_SPEED: u32 = 0x90;

const SPIN_ENTER_GET_SPIN_FACTOR: Callee = Callee {
    addr: 0x0013_5180,
    f: ee::s01::RiderSpinSub_GetSpinFactor,
};
const SPIN_ENTER_SET_SPIN_DIRECTION: Callee = Callee {
    addr: 0x0013_5b30,
    f: ee::s01::RiderSpinSub_SetSpinDirection,
};

/// `void RiderSpinSub::Enter(RiderSpinSub* this, int)` (0x133128): start
/// the air-spin sub-state. Clears the spin bookkeeping (+0x4..+0x54, rates
/// at the minimum 4.4997), and if the stick held at the jump (rider
/// +0x2a4/+0x2b0) is pushed at all: spin rate = max(7 * the larger stick
/// axis, min) when x is pushed, flip rate = that * 2/3 when y is pushed
/// (both mirrored into the rider), the spin factor and direction, and the
/// spin animation's speed = (1 + 0.5 * the character's spin attribute) *
/// max(flip, spin rate) / 7; state 1. Otherwise state 0, exit state 3.
pub fn rider_spin_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 39;
        let frame = Frame::open(c, 0x30);
        c.lwc1(5, gp.wrapping_sub(0x7544)); // minimum rate
        frame.save(c, S0, 0x10);
        c.g[V0][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, S0);
        Addr(this.wrapping_add(SPIN_ENTER_NEXT)).sw(c, u64::MAX);
        c.g[V1][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        for off in [0x4, 0x18, 0x1c, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34] {
            Addr(this.wrapping_add(off)).sw(c, 0);
        }
        c.swc1(5, this.wrapping_add(SPIN_ENTER_SPIN_RATE));
        c.swc1(5, this.wrapping_add(SPIN_ENTER_FLIP_RATE));
        Addr(this.wrapping_add(SPIN_ENTER_FACTOR)).sw(c, 0);
        Addr(this.wrapping_add(0x44)).sw(c, 0);
        let rider = r32(c, V1);
        Addr(rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE)).sw(c, 0);
        c.lwc1(0, gp.wrapping_sub(0x7540)); // 1e30
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        Addr(r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE)).sw(c, 0);
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        c.swc1(0, this.wrapping_add(0x50));
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        c.lwc1(4, this.wrapping_add(0x48)); // f4 = 0
        let rider = r32(c, V0);
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_X));
        c.c_eq_s(0, 4);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_Y));
            c.c_eq_s(0, 4);
            if c.fcc() {
                // The stick was centred: no spin.
                c.g[V0][0] = 3;
                c.cycles += 2;
                Addr(this.wrapping_add(SPIN_ENTER_STATE)).sw(c, 0);
                Addr(this.wrapping_add(SPIN_ENTER_EXIT)).sw(c, 3);
                return spin_enter_epilogue(c, frame);
            }
            c.cycles += 1;
        }
        Addr(this.wrapping_add(SPIN_ENTER_EXIT)).sw(c, 0);

        // f2 = the larger of |x| and |y|.
        c.cycles += 8;
        c.lwc1(3, rider.wrapping_add(SPIN_ENTER_STICK_X));
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_Y));
        c.abs_s(1, 3);
        c.abs_s(0, 0);
        c.c_lt_s(0, 1);
        let x_larger = c.fcc();
        c.mov_s(2, 0);
        if x_larger {
            c.cycles += 1;
            c.mov_s(2, 1);
        }

        // Spin rate from x.
        c.cycles += 4;
        c.c_eq_s(3, 4);
        if !c.fcc() {
            c.cycles += 6;
            c.lwc1(0, gp.wrapping_sub(0x753c)); // 7
            c.mul_s(0, 2, 0);
            c.max_s(0, 0, 5);
            c.swc1(0, this.wrapping_add(SPIN_ENTER_SPIN_RATE));
            c.swc1(0, rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE));
        }
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V0);

        // Flip rate from y.
        c.cycles += 4;
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_Y));
        c.c_eq_s(0, 4);
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_sub(0x7538)); // 7
            c.lwc1(1, gp.wrapping_sub(0x7534)); // 2/3
            c.mul_s(0, 2, 0);
            c.max_s(0, 0, 5);
            c.mul_s(0, 0, 1);
            c.swc1(0, this.wrapping_add(SPIN_ENTER_FLIP_RATE));
            c.swc1(0, rider.wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE));
        }

        c.cycles += 37;
        c.g[A0][0] = c.g[S0][0];
        SPIN_ENTER_GET_SPIN_FACTOR.call(c, 0x0013_325c)?;
        let this = r32(c, S0);
        c.swc1(0, this.wrapping_add(SPIN_ENTER_FACTOR));
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V0);
        c.lwc1(13, rider.wrapping_add(SPIN_ENTER_STICK_X));
        c.lwc1(12, rider.wrapping_add(SPIN_ENTER_STICK_Y));
        SPIN_ENTER_SET_SPIN_DIRECTION.call(c, 0x0013_3274)?;
        crate::game::ai::ai::BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_327c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        crate::game::ai::ai::BE_LIBRARY_GET_INTERFACE.call(c, 0x0013_3288)?;
        let this = r32(c, S0);
        c.g[V1][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V1);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(rider.wrapping_add(SPIN_ENTER_PLAYER)).lw(c);
        c.g[A2][0] = Addr(rider.wrapping_add(SPIN_ENTER_CHARACTER)).lw(c);
        crate::game::ai::ai::BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x0013_329c)?;

        // Animation speed = (1 + attrib * 0.5) * max(flip, spin) / 7.
        let this = r32(c, S0);
        c.lwc1(1, gp.wrapping_sub(0x7530));
        c.g[A1][0] = 2;
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V0);
        c.mul_s(20, 0, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE));
        c.lwc1(1, rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE));
        c.add_s(20, 20, 2);
        c.lwc1(3, gp.wrapping_sub(0x752c));
        c.max_s(0, 0, 1);
        c.g[A0][0] = Addr(rider.wrapping_add(SPIN_ENTER_ANIM_CTRL)).lw(c);
        c.mul_s(20, 20, 0);
        c.mul_s(20, 20, 3);
        call_ptr(c, 0x0031_1b20, 0x0013_32d8)?; // AnimController_GetLayerSequence(ctrl, 2)
        c.g[V1][0] = 1;
        let seq = r32(c, V0);
        c.swc1(20, seq.wrapping_add(SPIN_ENTER_SEQ_SPEED));
        let this = r32(c, S0);
        Addr(this.wrapping_add(SPIN_ENTER_STATE)).sw(c, 1);
        spin_enter_epilogue(c, frame);
        Ok(())
    })
}

/// RiderSpinSub_Enter's epilogue (5 cycles).
fn spin_enter_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 5;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// BankMonitor_FindFreeInstance (0x2acf60)

/// BankMonitor: the instance array (64 entries of 0xc0 bytes).
const BANK_MONITOR_INSTANCES: u32 = 0x8ec;
const BANK_MONITOR_INSTANCE_SIZE: u32 = 0xc0;
const BANK_MONITOR_INSTANCE_COUNT: i64 = 64;

const BANK_MONITOR_POLL_FREE: Callee = Callee {
    addr: 0x002a_bb80,
    f: ee::s08::BankMonitorInstance_PollFree,
};

/// `int BankMonitor::FindFreeInstance(BankMonitor* this, int n)`
/// (0x2acf60): the index of the n-th free sound-bank instance (polling each
/// with `BankMonitorInstance_PollFree`, which also recycles finished ones),
/// or -1. The loop state lives in s0 (n left), s1 (index), s2 (byte
/// offset), s3 (this), as in the original.
pub fn bank_monitor_find_free_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        loop {
            c.cycles += 5;
            let this = r32(c, S3);
            let base = Addr(this.wrapping_add(BANK_MONITOR_INSTANCES)).lw(c) as u32;
            c.g[A0][0] = sx(base.wrapping_add(r32(c, S2)));
            BANK_MONITOR_POLL_FREE.call(c, 0x002a_cf94)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                if c.g[S0][0] == 0 {
                    // Found the n-th free one.
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    break;
                }
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < BANK_MONITOR_INSTANCE_COUNT) as u64;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(BANK_MONITOR_INSTANCE_SIZE));
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_cf88);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_UpdateTick20 (0x3d54c8)

/// `void PATH_UpdateTick20(void)` (0x3d54c8): the music-path timer tick,
/// `PATH_Update(0x20)`. Frame 0x10 (ra).
pub fn path_update_tick20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x20;
        Callee {
            addr: 0x003d_5128,
            f: ee::s14::PATH_Update,
        }
        .call(c, 0x003d_54d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider sub-state SaveState: write the raw state block to a replay stream.

/// The stream's `Write(this, data, size)` virtual (vtable slot +0x8).
const SAVE_STATE_STREAM_WRITE: u32 = 0x8;

/// `stream->Write(self, size)` in a 0x10 frame, returning to `ret`: the
/// shape of the riders' plain `SaveState(self, stream)` methods (13
/// cycles).
fn rider_save_state_raw(c: &mut Cpu, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0]; // self
        c.g[A2][0] = size;
        let stream = c.g[A3][0];
        c.g[V1][0] = Addr(stream as u32).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, SAVE_STATE_STREAM_WRITE);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(stream);
        call_ptr(c, slot.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderGroundState::SaveState(RiderGroundState* this, Stream* s)`
/// (0x13f848): writes the state's 0x18-byte block to the replay stream.
pub fn rider_ground_state_save_state(c: &mut Cpu) {
    rider_save_state_raw(c, 0x18, 0x0013_f870);
}

/// `void RiderRailslideSub::SaveState(RiderRailslideSub* this, Stream* s)`
/// (0x132840): writes the sub-state's 8-byte block to the replay stream.
pub fn rider_railslide_sub_save_state(c: &mut Cpu) {
    rider_save_state_raw(c, 0x8, 0x0013_2868);
}

// ---------------------------------------------------------------------------
// WScriptMan_SaveProcessList (0x30bc80)

/// WScriptProcess: next in its list, script id.
const WSCRIPT_SAVE_LIST_NEXT: u32 = 0x18;
const WSCRIPT_SAVE_LIST_ID: u32 = 0x20;

const WSCRIPT_SAVE_LIST_COUNT: Callee = Callee {
    addr: 0x0030_b228,
    f: ee::s10::WScriptProcessList_Count,
};
const WSCRIPT_SAVE_PROCESS: Callee = Callee {
    addr: 0x0030_8038,
    f: ee::s10::WScriptProcess_SaveState,
};

/// `stream->Write(stream, a1, a2)` (s1 = the stream) with `v1` = the stream's vtable
/// (already loaded by the caller), returning to `ret`.
fn wscript_save_list_write_word(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    let stream = c.g[S1][0];
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, SAVE_STATE_STREAM_WRITE);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(stream);
    call_ptr(c, slot.func as u32, ret)
}

/// `void WScriptMan::SaveProcessList(WScriptMan*, WScriptProcessList* list,
/// Stream* s)` (0x30bc80): writes the number of processes, then for each
/// process its script id and its state (`WScriptProcess_SaveState`). The
/// list walk keeps the node in s0 and the stream in s1, as the original.
pub fn wscript_man_save_process_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0]; // list
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A2][0]; // stream
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        WSCRIPT_SAVE_LIST_COUNT.call(c, 0x0030_bca0)?;

        // The count.
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        let count = c.g[V0][0];
        Addr(sp(c)).sw(c, count);
        c.g[A2][0] = 4;
        wscript_save_list_write_word(c, 0x0030_bcc0)?;

        // Each process: its id, then its state.
        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            loop {
                c.cycles += 14;
                c.g[A1][0] = sx(sp(c).wrapping_add(4));
                c.g[A3][0] = Addr(r32(c, S0).wrapping_add(WSCRIPT_SAVE_LIST_ID)).lw(c);
                c.g[A2][0] = 4;
                let id = c.g[A3][0];
                Addr(sp(c).wrapping_add(4)).sw(c, id);
                wscript_save_list_write_word(c, 0x0030_bcf0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                WSCRIPT_SAVE_PROCESS.call(c, 0x0030_bcfc)?;
                c.g[S0][0] = Addr(r32(c, S0).wrapping_add(WSCRIPT_SAVE_LIST_NEXT)).lw(c);
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_bcd0);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_Exit (0x134cb0)

/// RiderSpinSub: spin start and current angle (+0x14 / +0x1c compared).
const SPIN_EXIT_START: u32 = 0x14;
const SPIN_EXIT_ANGLE: u32 = 0x1c;
/// Rider: the spin input block cleared on exit, the two input timers
/// (+0x290/+0x29c, reset to 1/60), the pose matrix.
const SPIN_EXIT_RIDER_POSE: u32 = 0x110;
const SPIN_EXIT_TIMER_A: u32 = 0x290;
const SPIN_EXIT_TIMER_B: u32 = 0x29c;

const SPIN_EXIT_APPLY_POSE_ROTATION: Callee = Callee {
    addr: 0x0013_4dd0,
    f: ee::s01::RiderSpinSub_ApplyPoseRotation,
};

/// `this->rider` (`lw (s0 + 0x58)`).
fn spin_exit_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S0).wrapping_add(SPIN_ENTER_RIDER)).lw(c)
}

/// `void RiderSpinSub::Exit(RiderSpinSub* this, int)` (0x134cb0): leave the
/// air spin. Fades out animation layer 1 (0.33 s), clears the rider's stick
/// snapshot (+0x2a4..+0x2b8), resets its input timers to 1/60, applies the
/// final pose rotation and rebuilds the world matrix; then scales the
/// rider's spin rate (+0x2dc) by 0.75 and snaps it to the quarter turn
/// above (pi/2, 3pi/2 or 5pi/2; 0 if not positive), negated when the spin
/// went backwards (angle +0x1c past start +0x14); flip rate +0x2e0 = 0.
pub fn rider_spin_sub_exit(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 42;
        let frame = Frame::open(c, 0x20);
        c.lwc1(12, gp.wrapping_sub(0x73a0)); // 0.33
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = spin_exit_rider(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(SPIN_ENTER_ANIM_CTRL)).lw(c);
        call_ptr(c, 0x0031_1e88, 0x0013_4cd4)?; // AnimController_FadeOutLayer(ctrl, 1, 0.33)

        c.g[V1][0] = spin_exit_rider(c);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(0, gp.wrapping_sub(0x739c)); // 1/60
        let rider = r32(c, V1);
        for off in [0x2a8, 0x2ac, 0x2a4] {
            Addr(rider.wrapping_add(off)).sw(c, 0);
        }
        c.g[V0][0] = spin_exit_rider(c);
        let rider = r32(c, V0);
        for off in [0x2b4, 0x2b8, 0x2b0] {
            Addr(rider.wrapping_add(off)).sw(c, 0);
        }
        c.g[V1][0] = spin_exit_rider(c);
        let rider = r32(c, V1);
        c.swc1(0, rider.wrapping_add(SPIN_EXIT_TIMER_A));
        Addr(rider.wrapping_add(SPIN_EXIT_TIMER_A + 4)).sw(c, 0);
        c.g[V0][0] = spin_exit_rider(c);
        let rider = r32(c, V0);
        c.swc1(0, rider.wrapping_add(SPIN_EXIT_TIMER_B));
        Addr(rider.wrapping_add(SPIN_EXIT_TIMER_B + 4)).sw(c, 0);
        let rider = spin_exit_rider(c) as u32;
        c.g[A1][0] = sx(rider.wrapping_add(SPIN_EXIT_RIDER_POSE));
        SPIN_EXIT_APPLY_POSE_ROTATION.call(c, 0x0013_4d20)?;
        c.g[A0][0] = spin_exit_rider(c);
        call_ptr(c, 0x0011_e098, 0x0013_4d28)?; // Rider_BuildWorldMatrix

        // Spin rate *= 0.75, snapped up to the next quarter turn.
        let gp = r32(c, GP);
        c.g[V0][0] = spin_exit_rider(c);
        let rider = r32(c, V0);
        let rate = rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE);
        c.lwc1(1, gp.wrapping_sub(0x7398)); // 0.75
        c.lwc1(0, rate);
        c.lwc1(2, gp.wrapping_sub(0x7394)); // 3pi/2
        c.mul_s(0, 0, 1);
        c.swc1(0, rate);
        c.g[V0][0] = spin_exit_rider(c);
        let rate = r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE);
        c.lwc1(1, rate);
        c.c_lt_s(2, 1);
        c.lwc1(0, gp.wrapping_sub(0x7390)); // 5pi/2
        if c.fcc() {
            c.swc1(0, rate);
        } else {
            c.cycles += 4;
            c.lwc1(0, gp.wrapping_sub(0x738c)); // pi
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 2;
                c.swc1(2, rate);
            } else {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.cycles += 2;
                    c.lwc1(0, gp.wrapping_sub(0x7388)); // pi/2
                }
                c.swc1(0, rate);
            }
        }

        // Flip rate cleared; a backwards spin turns the other way.
        c.cycles += 8;
        c.g[V0][0] = spin_exit_rider(c);
        Addr(r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE)).sw(c, 0);
        let this = r32(c, S0);
        c.lwc1(1, this.wrapping_add(SPIN_EXIT_ANGLE));
        c.lwc1(0, this.wrapping_add(SPIN_EXIT_START));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 5;
            c.g[V0][0] = spin_exit_rider(c);
            let rate = r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE);
            c.lwc1(0, rate);
            c.neg_s(0, 0);
            c.swc1(0, rate);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_PickUberTrick (0x10cf68)

/// ComputerActorRider: the rider, the uber-trick plan (picked flag, the
/// gap before the trick's end pose, the time asked for, the spare time),
/// and per-trick use counts (15 ints).
const UBER_PICK_RIDER: u32 = 0x18;
const UBER_PICK_PICKED: u32 = 0xe24;
const UBER_PICK_PLANNED: u32 = 0xe28;
const UBER_PICK_TAIL: u32 = 0xe2c;
const UBER_PICK_TIME: u32 = 0xe30;
const UBER_PICK_SPARE: u32 = 0xe34;
const UBER_PICK_USES: u32 = 0xeb8;
/// Rider: air time left (float), trick level (>= 5: the second animation
/// set), the trick interface object (vtable slot +0x38: rider index), the
/// animation controller.
const UBER_PICK_AIR_TIME: u32 = 0x2f0;
const UBER_PICK_LEVEL: u32 = 0x2f4;
const UBER_PICK_TRICK_IFACE: u32 = 0x6c0;
const UBER_PICK_IFACE_INDEX_SLOT: u32 = 0x38;
const UBER_PICK_UBER_COUNT: i64 = 15;
/// `BEBAGTInt_GetTrickAnim` result for a trick the rider has no slot for.
const UBER_PICK_NO_ANIM: u64 = 0x1b6;

const UBER_PICK_SKILL_FACTOR: Callee = Callee {
    addr: 0x0012_0038,
    f: ee::s00::Rider_GetTrickSkillFactor,
};

/// `rider->trickIface->GetIndex()` (vtable slot +0x38) with a0 = the rider
/// (as the original leaves it), returning to `ret`.
fn uber_pick_rider_index(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    let iface = r32(c, A0).wrapping_add(UBER_PICK_TRICK_IFACE);
    c.g[V1][0] = Addr(iface).lw(c);
    c.g[A0][0] = sx(iface);
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, UBER_PICK_IFACE_INDEX_SLOT);
    c.g[A1][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(c.g[A0][0]);
    call_ptr(c, slot.func as u32, ret)
}

/// `AnimController_GetCurrentAnimTriggerTime(rider->anim, a1, part)` into
/// f0 (`rider_reg` holds the rider; a1 is set by the caller).
fn uber_pick_trigger_time(c: &mut Cpu, rider_reg: usize, part: u64, ret: u32) -> crate::call::Flow {
    c.g[A2][0] = part;
    c.g[A0][0] = Addr(r32(c, rider_reg).wrapping_add(SPIN_ENTER_ANIM_CTRL)).lw(c);
    call_ptr(c, 0x0031_2820, ret)
}

/// `int ComputerActorRider::PickUberTrick(ComputerActorRider* this, float
/// time)` (0x10cf68): pick the uber trick a CPU rider can finish in `time`
/// seconds of air. -1 if the rider has no air time left (+0x2f0 <= 0).
/// Otherwise, over the 15 uber tricks the rider has an animation for, take
/// the least-used one whose start + end animation times (scaled by the
/// rider's trick skill) fit in `time`; record its tail and spare time,
/// count its use and return its index (-1 if none fits). The loop state
/// lives in s1 (animation), s2 (trick), s4 (best), s5 (best use count),
/// f22 (skill), f23 (time), as in the original.
pub fn computer_actor_rider_pick_uber_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        c.mtc1(0, 0);
        frame.save(c, S0, 0x60);
        frame.save_fpr(c, 23, 0x88);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x80);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        let this = r32(c, S0);
        Addr(this.wrapping_add(UBER_PICK_PLANNED)).sw(c, 0);
        c.g[V0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
        c.lwc1(1, r32(c, V0).wrapping_add(UBER_PICK_AIR_TIME));
        c.c_le_s(1, 0);
        let grounded = c.fcc();
        c.mov_s(23, 12);
        if grounded {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            return uber_pick_epilogue(c, frame);
        }

        c.cycles += 13;
        c.swc1(23, this.wrapping_add(UBER_PICK_TIME));
        c.g[S4][0] = u64::MAX;
        c.g[S5][0] = 10000;
        c.g[S2][0] = 0;
        call_ptr(c, 0x0014_dc80, 0x0010_cfd8)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 6;
        call_ptr(c, 0x0014_dd58, 0x0010_cfe4)?; // BELibrary_GetInterface(be, 6): BAGT
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        UBER_PICK_SKILL_FACTOR.call(c, 0x0010_cff0)?;
        c.mov_s(22, 0);
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
        loop {
            c.cycles += 18;
            uber_pick_rider_index(c, 0x0010_d010)?;
            let this = r32(c, S0);
            c.g[V1][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            let level = Addr(r32(c, V1).wrapping_add(UBER_PICK_LEVEL)).lw(c);
            c.g[A3][0] = (((level as i64) < 5) as u64) ^ 1;
            call_ptr(c, 0x0015_0198, 0x0010_d030)?; // BEBAGTInt_GetTrickAnim
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = UBER_PICK_NO_ANIM;
            if c.g[S1][0] != UBER_PICK_NO_ANIM {
                c.cycles += 34;
                let this = r32(c, S0);
                c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                uber_pick_rider_index(c, 0x0010_d05c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0015_00d8, 0x0010_d06c)?; // BEBAGTInt_GetTrickField0
                let this = r32(c, S0);
                c.g[V1][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                uber_pick_trigger_time(c, V1, 1, 0x0010_d080)?;
                let this = r32(c, S0);
                c.g[V0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                c.div_s(20, 0, 22); // start of the lead-in
                c.g[A1][0] = c.g[S1][0];
                uber_pick_trigger_time(c, V0, 1, 0x0010_d098)?;
                let this = r32(c, S0);
                c.g[V0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
                c.div_s(21, 0, 22); // start of the trick
                c.g[A1][0] = c.g[S1][0];
                uber_pick_trigger_time(c, V0, 2, 0x0010_d0b0)?;
                c.div_s(0, 0, 22); // end of the trick
                c.add_s(1, 0, 20);
                c.c_lt_s(1, 23);
                let fits = c.fcc();
                c.g[A0][0] = sx(r32(c, S2) << 2);
                if fits {
                    c.cycles += 6;
                    let this = r32(c, S0);
                    let uses = this.wrapping_add(UBER_PICK_USES).wrapping_add(r32(c, A0));
                    c.g[V1][0] = sx(uses);
                    let n = Addr(uses).lw(c);
                    let fewer = (n as i64) < (c.g[S5][0] as i64);
                    c.g[V0][0] = 1; // (the branch delay slot's li)
                    if fewer {
                        c.cycles += 7;
                        c.sub_s(0, 0, 21);
                        c.sub_s(1, 23, 1);
                        Addr(this.wrapping_add(UBER_PICK_PICKED)).sw(c, 1);
                        c.g[S4][0] = c.g[S2][0];
                        c.swc1(0, this.wrapping_add(UBER_PICK_TAIL));
                        c.swc1(1, this.wrapping_add(UBER_PICK_SPARE));
                        c.g[S5][0] = Addr(uses).lw(c);
                    }
                }
                c.cycles += 1;
            }
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < UBER_PICK_UBER_COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let this = r32(c, S0);
            c.g[A0][0] = Addr(this.wrapping_add(UBER_PICK_RIDER)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0010_cff8);
            }
        }

        // Count the pick.
        c.cycles += 3;
        let best = c.g[S4][0];
        c.g[V0][0] = ((best as i64) < UBER_PICK_UBER_COUNT) as u64;
        if best != u64::MAX {
            c.cycles += 2;
            let in_range = c.g[V0][0] != 0;
            let this = r32(c, S0);
            c.g[V0][0] = sx(this.wrapping_add(UBER_PICK_USES));
            if in_range {
                c.cycles += 5;
                c.g[A0][0] = sx((best as u32) << 2);
                let uses = r32(c, V0).wrapping_add(r32(c, A0));
                c.g[V0][0] = sx(uses);
                c.g[V1][0] = sx((Addr(uses).lw(c) as u32).wrapping_add(1));
                let n = c.g[V1][0];
                Addr(uses).sw(c, n);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S4][0];
        uber_pick_epilogue(c, frame)
    })
}

/// PickUberTrick's epilogue (13 cycles).
fn uber_pick_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 13;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 23, 0x88);
    frame.restore_fpr(c, 22, 0x80);
    frame.restore_fpr(c, 21, 0x78);
    frame.restore_fpr(c, 20, 0x70);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP};

    /// `RiderSpinSub_Enter`: the stick x and y held at the jump each zero,
    /// -0, an edge value or in [-1.5, 1.5], so the centred early exit, x
    /// only, y only and both (either axis larger) occur. The real
    /// GetSpinFactor (with Math_Atan), SetSpinDirection,
    /// BECharInt_GetAttrib4Factor (player 0, characters 1..5) and
    /// AnimController_GetLayerSequence run on the game image; the BE
    /// interface getter is a fake (sometimes unwinding).
    #[test]
    fn rider_spin_sub_enter_matches() {
        const THIS: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const CTRL: u32 = DATA + 0x3000;
        const LAYERS: u32 = DATA + 0x3100;
        const SEQ: u32 = DATA + 0x3200;
        const BE_LIBRARY: u32 = DATA + 0x5000;
        const BE_OBJ: u32 = DATA + 0x5100;
        const BE_VT: u32 = DATA + 0x5200;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        covered(
            ee::s01::f_00133128__generated,
            rider_spin_sub_enter,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_3128, 1);
                elf_image(c);
                c.pc = 0x0013_3128;
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_INTERFACES, BE_OBJ);
                c.bus.write32(BE_OBJ + 4, BE_VT);
                c.bus.write32(BE_VT + 0xc, FAKE);
                c.bus.write8(0x0053_08e2, 10);
                c.bus.write8(0x0053_08e4, 10);
                fill(c, &mut r, THIS, 0x60 / 4);
                c.bus.write32(THIS + SPIN_ENTER_RIDER, RIDER);
                let axis = |r: &mut Rng, k: u32| match k % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.edge_f32(),
                    _ => r.f32(1.5),
                };
                let x = axis(&mut r, seed);
                let y = axis(&mut r, seed / 4);
                c.bus.write32(RIDER + SPIN_ENTER_STICK_X, x);
                c.bus.write32(RIDER + SPIN_ENTER_STICK_Y, y);
                c.bus.write32(RIDER + SPIN_ENTER_RIDER_SPIN_RATE, r.next_u32());
                c.bus.write32(RIDER + SPIN_ENTER_RIDER_FLIP_RATE, r.next_u32());
                c.bus.write32(RIDER + SPIN_ENTER_ANIM_CTRL, CTRL);
                c.bus.write32(RIDER + SPIN_ENTER_PLAYER, 0);
                c.bus.write32(RIDER + SPIN_ENTER_CHARACTER, 1 + r.next_u32() % 5);
                // Layer 2 of the controller: a one-entry sequence list.
                c.bus.write32(CTRL + 0x50, LAYERS);
                c.bus.write32(LAYERS + 2 * 8 + 4, SEQ);
                c.bus.write32(SEQ + SPIN_ENTER_SEQ_SPEED, r.next_u32());
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// `BankMonitor_FindFreeInstance`: 64 instances, each free (word 0
    /// is 0) or busy (word 0 set, nothing playing at +0x9c), asking for
    /// the 0th..5th free one (0 never matches: -1 after all 64), so the
    /// found and not-found exits occur. PollFree runs for real on its
    /// short paths.
    #[test]
    fn bank_monitor_find_free_instance_matches() {
        const THIS: u32 = DATA;
        const INSTANCES: u32 = DATA + 0x1000;
        check(
            ee::s08::f_002acf60__generated,
            bank_monitor_find_free_instance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_cf60, 0);
                c.pc = 0x002a_cf60;
                c.bus.write32(THIS + BANK_MONITOR_INSTANCES, INSTANCES);
                let busy_odds = 1 + seed % 8;
                for i in 0..64 {
                    let inst = INSTANCES + i * BANK_MONITOR_INSTANCE_SIZE;
                    let busy = r.next_u32() % 8 < busy_odds;
                    c.bus.write32(inst, if busy { r.next_u32() | 1 } else { 0 });
                    c.bus.write32(inst + 0x9c, 0);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(seed % 6);
            },
        );
    }

    /// `PATH_UpdateTick20` against the generated body, with the real
    /// `PATH_Update` (fixture as in `path_update_tick42_matches`): locked /
    /// uninitialised / busy early outs, and the full tick over four players
    /// that are inactive, active without a handle, or active with a handle.
    #[test]
    fn path_update_tick20_matches() {
        use crate::difftest::scramble;
        check(ee::s14::f_003d54c8__generated, path_update_tick20, 120, &|c, seed| {
            let mut r = Rng(0x003d_54c8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003d_54c8;
            let gate = seed % 6;
            c.bus.write8(
                0x0044_f42d,
                (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3),
            );
            c.bus.write8(0x0044_f42c, (gate != 1) as u8);
            c.bus
                .write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
            c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
            c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
            for i in 0..4u32 {
                let p = 0x0051_7610 + 0x928 * i;
                let kind = r.next_u32() % 3;
                c.bus
                    .write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                c.bus
                    .write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
            }
        });
    }

    /// A replay stream whose Write virtual is a fake (sometimes unwinding),
    /// `self` anywhere in the data area.
    fn rider_save_state_setup(c: &mut Cpu, seed: u32, entry: u32) {
        use crate::difftest::fakes::{ptr, vtable};
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let vt = vtable(c, &mut r, &[SAVE_STATE_STREAM_WRITE]);
        let stream = DATA + 0x20_0000 + (r.next_u32() & 0xfff0);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(ptr(&mut r));
        c.g[A1][0] = sx(stream);
    }

    /// `RiderGroundState_SaveState`: one stream write of 0x18 bytes.
    #[test]
    fn rider_ground_state_save_state_matches() {
        covered(
            ee::s01::f_0013f848__generated,
            rider_ground_state_save_state,
            200,
            &|c, seed| rider_save_state_setup(c, seed, 0x0013_f848),
        );
    }

    /// `RiderRailslideSub_SaveState`: one stream write of 8 bytes.
    #[test]
    fn rider_railslide_sub_save_state_matches() {
        covered(
            ee::s01::f_00132840__generated,
            rider_railslide_sub_save_state,
            200,
            &|c, seed| rider_save_state_setup(c, seed, 0x0013_2840),
        );
    }

    /// `WScriptMan_SaveProcessList`: lists of 0..4 processes; the stream's
    /// Write virtual is a fake (sometimes unwinding); the real
    /// `WScriptProcessList_Count` and `WScriptProcess_SaveState` run (no
    /// tasks, no attached object; the real script-cache lookup on a small
    /// table).
    #[test]
    fn wscript_man_save_process_list_matches() {
        use crate::difftest::fakes::vtable;
        const LIST: u32 = DATA;
        const NODES: u32 = DATA + 0x1000;
        const CACHE_HOLDER: u32 = DATA + 0x3000;
        const CACHE: u32 = DATA + 0x3100;
        const TABLE: u32 = DATA + 0x3200;
        const STREAM: u32 = DATA + 0x20_0000;
        covered(
            ee::s10::f_0030bc80__generated,
            wscript_man_save_process_list,
            300,
            &|c, seed| {
                let n = seed % 5;
                let mut r = machine(c, seed, 0x0030_bc80, 1 + 3 * n);
                c.pc = 0x0030_bc80;
                let vt = vtable(c, &mut r, &[SAVE_STATE_STREAM_WRITE]);
                c.bus.write32(STREAM, vt);
                c.bus.write32(GAME_GP + 0x16c8, CACHE_HOLDER);
                c.bus.write32(CACHE_HOLDER + 4, CACHE);
                c.bus.write32(CACHE + 4, TABLE);
                fill(c, &mut r, TABLE, 16 * 3);
                c.bus.write32(LIST, if n == 0 { 0 } else { NODES });
                for i in 0..n {
                    let node = NODES + i * 0x100;
                    fill(c, &mut r, node, 0x30 / 4);
                    c.bus.write32(node + 0x4, 0); // no tasks
                    c.bus.write32(node + 0x1c, 0); // no attached object
                    c.bus.write32(node + 0x24, r.next_u32() % 16);
                    let next = if i + 1 < n { node + 0x100 } else { 0 };
                    c.bus.write32(node + WSCRIPT_SAVE_LIST_NEXT, next);
                }
                c.g[A0][0] = sx(DATA + 0x8000);
                c.g[A1][0] = sx(LIST);
                c.g[A2][0] = sx(STREAM);
            },
        );
    }

    /// `RiderSpinSub_Exit`: spin rates in [-10, 10] (after the 0.75 scale
    /// above 3pi/2, above pi, positive, or not), start/current angles in
    /// either order. The real AnimController_FadeOutLayer (empty layer),
    /// RiderSpinSub_ApplyPoseRotation (with GetSpinFactor, Math_Atan and
    /// the sin/cos helper) and Rider_BuildWorldMatrix run on random floats.
    #[test]
    fn rider_spin_sub_exit_matches() {
        const THIS: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const CTRL: u32 = DATA + 0x3000;
        const LAYERS: u32 = DATA + 0x3100;
        const BOARD: u32 = DATA + 0x5000;
        const BONES: u32 = DATA + 0x6000;
        check(ee::s01::f_00134cb0__generated, rider_spin_sub_exit, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0013_4cb0, 0);
            elf_image(c);
            c.pc = 0x0013_4cb0;
            for i in 0..0x60 / 4 {
                c.bus.write32(THIS + 4 * i, r.f32(8.0));
            }
            c.bus.write32(THIS + SPIN_ENTER_RIDER, RIDER);
            for i in 0..0xc00 / 4 {
                c.bus.write32(RIDER + 4 * i, r.f32(4.0));
            }
            c.bus.write32(RIDER + SPIN_ENTER_RIDER_SPIN_RATE, r.f32(10.0));
            c.bus.write32(RIDER + SPIN_ENTER_ANIM_CTRL, CTRL);
            c.bus.write32(RIDER + 0x780, BOARD);
            c.bus.write32(RIDER + 0x89c, r.next_u32() % 8);
            for i in 0..0x200 / 4 {
                c.bus.write32(BOARD + 4 * i, r.f32(2.0));
                c.bus.write32(BONES + 4 * i, r.f32(2.0));
            }
            c.bus.write32(BOARD + 0x24, BONES);
            // Layer 1 of the controller: an empty sequence list.
            c.bus.write32(CTRL + 0x50, LAYERS);
            c.bus.write32(LAYERS + 8 + 4, 0);
            c.g[A0][0] = sx(THIS);
        });
    }

    /// `ComputerActorRider_PickUberTrick`: no air time (early -1) or some;
    /// a time budget in [0, 20] s against random animation trigger times
    /// (some parts missing: -1.0) and random use counts, so tricks fit or
    /// not, are less used or not, and some riders have no animation for a
    /// trick (0x1b6). The real BELibrary lookups (interface getter a fake,
    /// sometimes unwinding), Rider_GetTrickSkillFactor, BEBAGTInt_GetTrickAnim
    /// / GetTrickField0 (fixture as in their own tests) and
    /// AnimController_GetCurrentAnimTriggerTime run; the rider-index and
    /// animation-index virtuals are the real 0-returning leaf 0x1406c0.
    #[test]
    fn computer_actor_rider_pick_uber_trick_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        const THIS: u32 = DATA;
        const RIDER: u32 = DATA + 0x2000;
        const IFACE_VT: u32 = DATA + 0x3000;
        const CTRL: u32 = DATA + 0x3100;
        const CTRL_VT: u32 = DATA + 0x3200;
        const TRIGGERS: u32 = DATA + 0x4000;
        const BANKS: u32 = DATA + 0x6000;
        const BANK: u32 = DATA + 0x6100;
        const ENTRIES: u32 = DATA + 0x6200;
        const TIMES: u32 = DATA + 0x6400;
        const BAGT: u32 = DATA + 0x7000;
        const ANIMS: u32 = DATA + 0x8000;
        const ZERO_LEAF: u32 = 0x0014_06c0;
        const RIDER_PROFILES: u32 = 0x0053_05b0;
        const PROFILES: u32 = 0x0053_5b20;
        const SLOTS: u32 = 0x0053_0ec0;
        const TRICK_TABLE: u32 = 0x0045_aeb8;
        covered(
            ee::s00::f_0010cf68__generated,
            computer_actor_rider_pick_uber_trick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cf68, 2);
                elf_image(c);
                c.pc = 0x0010_cf68;
                // The BE singletons; the library's interface getter is a
                // fake (the BAGT interface comes back as BAGT).
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x6_0000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                for g in [0x1f3c, 0x1f30, 0x1f2c, 0x1ea4] {
                    c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
                }
                let lib = DATA + 0x5_0000;
                c.bus.write32(0x0053_08a8, lib);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(lib + 4, vt);
                c.bus.write32(LOG_RESULTS, BAGT);
                // Player profiles and records (skill factor, trick slots).
                for p in 0..8 {
                    c.bus.write32(RIDER_PROFILES + 4 * p, r.next_u32() % 8);
                }
                for s in 0..8 {
                    let rec = PROFILES + s * 0x1c;
                    fill(c, &mut r, rec, 7);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(rec + 0xc, u32::MAX);
                    }
                    c.bus.write8(rec + 0x11, (r.next_u32() % 10) as u8);
                    let chars = [0u8, 0x14, 0x18, 0x19, 0x1c, 5, 30, 0xf0];
                    c.bus.write8(rec + 0x12, chars[(r.next_u32() % 8) as usize]);
                }
                for i in 0..0x4000u32 / 4 {
                    let w = r.next_u32() & 0x0707_0707;
                    c.bus.write32(SLOTS + 4 * i, w);
                }
                for t in 0..15u32 {
                    let e = TRICK_TABLE + 0x14 * t;
                    c.bus.write16(e, r.next_u32() as u16);
                    c.bus.write16(e + 0xc, if r.next_u32() % 3 == 0 { 0 } else { 8 });
                    let anims = ANIMS + 0x100 * t;
                    c.bus.write32(e + 0x10, anims);
                    fill(c, &mut r, anims, 0x40);
                }
                // The rider.
                c.bus.write32(THIS + UBER_PICK_RIDER, RIDER);
                let air = match seed % 7 {
                    0 => 0,
                    1 => r.f32(1.0) | 0x8000_0000,
                    _ => (1.0f32 + (r.next_u32() % 100) as f32 / 10.0).to_bits(),
                };
                c.bus.write32(RIDER + UBER_PICK_AIR_TIME, air);
                c.bus.write32(RIDER + UBER_PICK_LEVEL, r.next_u32() % 8);
                c.bus.write32(RIDER + UBER_PICK_TRICK_IFACE, IFACE_VT);
                c.bus.write32(IFACE_VT + UBER_PICK_IFACE_INDEX_SLOT + 4, ZERO_LEAF);
                c.bus.write32(RIDER + SPIN_ENTER_ANIM_CTRL, CTRL);
                c.bus.write32(RIDER + SPIN_ENTER_PLAYER, r.next_u32() % 8);
                let stats = match r.next_u32() % 3 {
                    0 => 0,
                    1 => r.next_u32() % 40,
                    _ => r.next_u32(),
                };
                c.bus.write32(RIDER + SPIN_ENTER_CHARACTER, stats);
                for i in 0..15 {
                    c.bus.write32(THIS + UBER_PICK_USES + 4 * i, r.next_u32() % 20);
                }
                // The animation controller: index 0 -> bank 0, entry e.
                c.bus.write32(CTRL + 0x58, CTRL_VT);
                c.bus.write32(CTRL_VT + 0x24, ZERO_LEAF);
                c.bus.write32(GAME_GP + 0xd8c, TRIGGERS);
                let entry = seed % 3;
                c.bus.write32(TRIGGERS + 0x1030, entry << 8);
                c.bus.write32(GAME_GP + 0xd08, BANKS);
                c.bus.write32(BANKS, BANK);
                c.bus.write32(BANK + 0x8, ENTRIES);
                c.bus.write32(BANK + 0x10, TIMES);
                for e in 0..3 {
                    c.bus.write16(ENTRIES + 0x14 * e + 0x10, (r.next_u32() % 64) as u16);
                    c.bus.write16(ENTRIES + 0x14 * e + 0x12, (r.next_u32() % 4) as u16);
                }
                for i in 0..0x100 {
                    c.bus.write32(TIMES + 4 * i, r.next_u32() % 600);
                }
                c.f[12] = r.f32(20.0) & 0x7fff_ffff;
                c.g[A0][0] = sx(THIS);
            },
        );
    }
}
