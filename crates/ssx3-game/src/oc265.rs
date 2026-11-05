//! Staging wave 208 (oc265): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_3ff8, rider_anim_play_seq_then_loop, "RiderAnim_PlaySeqThenLoop"),
    (0x0031_42f8, anim_seq_list_push_new, "AnimSeqList_PushNew"),
    (0x0035_6a28, mod_obj_v38, "ModObj_v38"),
    (0x0035_fd98, uv_scroll_modifier_save, "UVScrollModifier_Save"),
    (0x003c_8560, snd_ee_voice_set_volume, "SND_EeVoiceSetVolume"),
    (0x0015_0198, bebagt_int_get_trick_anim, "BEBAGTInt_GetTrickAnim"),
    (
        0x0010_09e0,
        computer_actor_rider_cruise_approach_jump,
        "ComputerActorRider_CruiseApproachJump",
    ),
    (0x0038_f800, vfx_visual_effects_test_render, "VfxVisualEffectsTest_Render"),
    (0x0031_d7e0, c_csi_cubic_spline_add_point, "cCSICubicSpline_AddPoint"),
];

// ---------------------------------------------------------------------------
// RiderAnim_PlaySeqThenLoop (0x103ff8)

/// `gp+0xd8c`: the rider animation bank (animation ids at +0x1030).
const PSTL_ANIM_BANK_GP: u32 = 0xd8c;
/// Animation ids in the bank, indexed by key.
const PSTL_BANK_IDS: u32 = 0x1030;
/// `gp-0x7f64` / `gp-0x7f60` / `gp-0x7f5c`: 1/60 (`.lit4`), ticks to seconds
/// for the intro advance, the plain advance and the weight fade.
const PSTL_TICK_INTRO_GP: u32 = 0x7f64;
const PSTL_TICK_PLAIN_GP: u32 = 0x7f60;
const PSTL_TICK_FADE_GP: u32 = 0x7f5c;
/// AnimSequence: track 0's animation id, time and length.
const PSTL_SEQ_ANIM0: u32 = 0x4;
const PSTL_SEQ_TIME0: u32 = 0x8;
const PSTL_SEQ_LENGTH0: u32 = 0x10;
/// AnimSequence methods (cross-shard direct calls, through the table).
const PSTL_SET_TRACK_ANIM: u32 = 0x0031_3c50;
const PSTL_SET_TRACK_TIME: u32 = 0x0031_3cf0;
const PSTL_SET_TRACK_LOOP: u32 = 0x0031_3d40;
const PSTL_ADVANCE_TRACK: u32 = 0x0031_35b0;
const PSTL_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
const PSTL_LIST_REMOVE: u32 = 0x0031_45f8;

/// `int RiderAnim_PlaySeqThenLoop(RiderAnim* anim, AnimSeqList* list,
/// AnimSequence* seq, int intro_key, float dt, int loop_key)` (0x103ff8):
/// while track 0 still plays the intro animation (`bank[intro_key]`), play
/// it once (no loop) and advance it by `dt/60`; when it reaches its end,
/// switch the track to `bank[loop_key]`, looping, from time 0. Otherwise
/// just advance the track. Then run the weight fade; when it has finished,
/// remove the sequence from `list` and return 1, else 0. (`a3` intro_key,
/// `t0` loop_key, `f12` dt.)
pub fn rider_anim_play_seq_then_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(PSTL_ANIM_BANK_GP)).lw(c);
        frame.save(c, S0, 0x30);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12];
        let seq = r32(c, S0);
        c.g[V1][0] = Addr(seq.wrapping_add(PSTL_SEQ_ANIM0)).lw(c);
        c.g[V0][0] = Addr(r32(c, A3).wrapping_add(PSTL_BANK_IDS)).lw(c);
        let intro = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[T0][0];
        if intro {
            c.cycles += 15;
            // Play the intro once.
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, PSTL_SET_TRACK_LOOP, 0x0010_4048)?;
            let k = Addr(r32(c, GP).wrapping_sub(PSTL_TICK_INTRO_GP)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.f[12] = (FpuF32(c.f[20]) * k).0;
            call_ptr(c, PSTL_ADVANCE_TRACK, 0x0010_405c)?;
            let seq = r32(c, S0);
            let time = Addr(seq.wrapping_add(PSTL_SEQ_TIME0)).read_fpu(c);
            c.f[1] = time.0;
            let length = Addr(seq.wrapping_add(PSTL_SEQ_LENGTH0)).read_fpu(c);
            c.f[0] = length.0;
            let ended = length.le(time);
            abi::set_fcc(c, ended);
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(PSTL_ANIM_BANK_GP)).lw(c);
            if ended {
                // Switch to the looping animation from the start.
                c.cycles += 16;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A2][0] = Addr(r32(c, V0).wrapping_add(PSTL_BANK_IDS)).lw(c);
                call_ptr(c, PSTL_SET_TRACK_ANIM, 0x0010_408c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 1;
                call_ptr(c, PSTL_SET_TRACK_LOOP, 0x0010_409c)?;
                c.f[12] = 0;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, PSTL_SET_TRACK_TIME, 0x0010_40ac)?;
            }
        } else {
            c.cycles += 5;
            let k = Addr(r32(c, GP).wrapping_sub(PSTL_TICK_PLAIN_GP)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.f[12] = (FpuF32(c.f[20]) * k).0;
            call_ptr(c, PSTL_ADVANCE_TRACK, 0x0010_40c8)?;
        }
        c.cycles += 6;
        let k = Addr(r32(c, GP).wrapping_sub(PSTL_TICK_FADE_GP)).read_fpu(c);
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = (FpuF32(c.f[20]) * k).0;
        call_ptr(c, PSTL_UPDATE_WEIGHT_FADE, 0x0010_40d8)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, PSTL_LIST_REMOVE, 0x0010_40f0)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// AnimSeqList_PushNew (0x3142f8)

/// `EA_MemAllocNamed(size, name, flags, align)`.
const PUSH_NEW_ALLOC: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed };
/// `AnimSequence_ctor(seq, type)`.
const PUSH_NEW_SEQ_CTOR: Callee = Callee { addr: 0x0031_3508, f: ee::s10::AnimSequence_ctor };
/// Size of an AnimSequence, its allocation name (0x48a5c0) and flags.
const PUSH_NEW_SEQ_SIZE: u64 = 0xd0;
const PUSH_NEW_ALLOC_FLAGS: u64 = 0x2000_0000;
/// AnimSeqList: `count` at +0, `first` at +4; AnimSequence `next` at +0xc8.
const PUSH_NEW_LIST_COUNT: u32 = 0x0;
const PUSH_NEW_LIST_FIRST: u32 = 0x4;
const PUSH_NEW_SEQ_NEXT: u32 = 0xc8;

/// `AnimSequence* AnimSeqList_PushNew(AnimSeqList* list, int type)`
/// (0x3142f8): allocate and construct a new sequence of `type`, link it at
/// the head of the list and bump the count; returns the sequence.
pub fn anim_seq_list_push_new(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = PUSH_NEW_ALLOC_FLAGS;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = PUSH_NEW_SEQ_SIZE;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x5a40));
        PUSH_NEW_ALLOC.call(c, 0x0031_4328)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        PUSH_NEW_SEQ_CTOR.call(c, 0x0031_4334)?;
        let list = r32(c, S0);
        c.g[V1][0] = Addr(list.wrapping_add(PUSH_NEW_LIST_FIRST)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        frame.restore(c, S1, 0x10);
        let seq = r32(c, A0);
        let old_first = c.g[V1][0];
        Addr(seq.wrapping_add(PUSH_NEW_SEQ_NEXT)).sw(c, old_first);
        Addr(list.wrapping_add(PUSH_NEW_LIST_FIRST)).sw(c, seq as u64);
        c.g[V1][0] = Addr(list.wrapping_add(PUSH_NEW_LIST_COUNT)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        let count = c.g[V1][0];
        Addr(list.wrapping_add(PUSH_NEW_LIST_COUNT)).sw(c, count);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// ModObj_v38 (0x356a28)

/// ModObj: its move-node block (+0x1c), whose first word is the node.
const MODOBJ_V38_BLOCK: u32 = 0x1c;
/// The node's vtable slot called (delta at +0xa0, function at +0xa4).
const MODOBJ_V38_SLOT: u32 = 0xa0;

/// `int ModObj_v38(ModObj* self)` (0x356a28): forward to the move-node's
/// virtual slot 0xa0 (its `delta`-adjusted `this`); 0 when there is no
/// block or no node.
pub fn mod_obj_v38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V1][0] = Addr(this.wrapping_add(MODOBJ_V38_BLOCK)).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, V1)).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 7;
                let node = r32(c, A1);
                c.g[V1][0] = Addr(node).lw(c);
                let vt = r32(c, V1);
                c.g[A0][0] = Addr(vt.wrapping_add(MODOBJ_V38_SLOT)).lh(c);
                c.g[V0][0] = Addr(vt.wrapping_add(MODOBJ_V38_SLOT + 4)).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx(node.wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0035_6a5c)?;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// UVScrollModifier_Save (0x35fd98)

/// The stream's `write(data, size)` vtable slot (delta at +8, fn at +0xc).
const UV_SCROLL_SAVE_WRITE_SLOT: u32 = 0x8;
/// Bytes of UVScrollModifier state saved.
const UV_SCROLL_SAVE_SIZE: u64 = 0x50;

/// `void UVScrollModifier_Save(UVScrollModifier* self, Stream* out)`
/// (0x35fd98): write the modifier's first 0x50 bytes with `out->write(self,
/// 0x50)` (virtual slot 8).
pub fn uv_scroll_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = UV_SCROLL_SAVE_SIZE;
        let out = r32(c, A3);
        c.g[V1][0] = Addr(out).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt.wrapping_add(UV_SCROLL_SAVE_WRITE_SLOT)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(UV_SCROLL_SAVE_WRITE_SLOT + 4)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(out.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_fdc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetVolume (0x3c8560)

/// The sound driver state (0x515b40) and its voice table pointer (+0x1dc).
const SND_SET_VOL_STATE: u32 = 0x0051_5b40;
const SND_SET_VOL_VOICES: u32 = 0x1dc;
/// A voice entry: 0x60 bytes; dirty flag byte at +1, volumes (floats) at +0x1c.
const SND_SET_VOL_VOICE_SIZE: u32 = 0x60;
const SND_SET_VOL_DIRTY: u32 = 0x1;
const SND_SET_VOL_VOLUMES: u32 = 0x1c;

/// `void SND_EeVoiceSetVolume(int voice, int channel, float volume)`
/// (0x3c8560): store `volume` as the voice's channel volume and mark the
/// voice dirty.
pub fn snd_ee_voice_set_volume(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = SND_SET_VOL_VOICE_SIZE as u64;
    let voice_index = r32(c, A0);
    let off = abi::mult(c, voice_index, SND_SET_VOL_VOICE_SIZE);
    c.g[A0][0] = sx(off);
    c.g[A2][0] = sx(SND_SET_VOL_STATE);
    let voices = Addr(SND_SET_VOL_STATE + SND_SET_VOL_VOICES).lw(c);
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[A3][0] = 1;
    let slot = off.wrapping_add(voices as u32).wrapping_add(r32(c, A1));
    c.g[V0][0] = sx(slot);
    let volume = FpuF32(c.f[12]);
    Addr(slot.wrapping_add(SND_SET_VOL_VOLUMES)).write_fpu(c, volume);
    c.g[V1][0] = Addr(SND_SET_VOL_STATE + SND_SET_VOL_VOICES).lw(c);
    let voice = off.wrapping_add(r32(c, V1));
    c.g[A0][0] = sx(voice);
    Addr(voice.wrapping_add(SND_SET_VOL_DIRTY)).sb(c, 1);
    ret(c);
}

// ---------------------------------------------------------------------------
// BEBAGTInt_GetTrickAnim (0x150198)

/// `BE_GetRiderAltCharacter(rider)`: the rider's alternate character id.
const TRICK_ANIM_ALT_CHARACTER: Callee =
    Callee { addr: 0x0014_a0b0, f: ee::s01::BE_GetRiderAltCharacter };
/// `BEBAGTInt_GetUberTrickSlotValue(self, rider, trick, second)`: the slot
/// of the rider's animation for trick `trick` (-1 when the trick has none).
const TRICK_ANIM_SLOT_VALUE: Callee =
    Callee { addr: 0x0014_fea8, f: ee::s01::BEBAGTInt_GetUberTrickSlotValue };
/// The trick table (0x45aeb8): 0x14-byte entries, `anims` (8-byte entries,
/// the animation id a `u16` at +0) at +0x10.
const TRICK_ANIM_TABLE: u32 = 0x0045_aeb8;
const TRICK_ANIM_ENTRY_SIZE: u32 = 0x14;
const TRICK_ANIM_ANIMS: u32 = 0x10;
/// The animation returned when the trick has no slot.
const TRICK_ANIM_NONE: u64 = 0x1b6;

/// The alternate characters with their own animation for one trick:
/// 0x18 -> 0x9f, 0x14 -> 0xa0, 0x1c -> 0xa1 (trick 4), 0x19 -> 0xa2 (trick 9).
///
/// Sets the registers and cycles the original's compare chain leaves;
/// returns the special animation (already in v0) or `None` when the
/// generic lookup runs (a0 = self, a1 = rider set).
fn trick_anim_special(c: &mut Cpu, ch: u64) -> Option<u64> {
    c.g[V1][0] = ch;
    c.g[V0][0] = 0x18;
    // `trick == want ? special : generic` tail, with a0/a1 as the branch
    // leaves them (`a0_first`: a0 is set in this block, else a1).
    let pick = |c: &mut Cpu, want: u64, special: u64, a0_first: bool| -> Option<u64> {
        c.cycles += 3;
        c.g[V0][0] = want;
        if a0_first {
            c.g[A0][0] = c.g[S2][0];
        } else {
            c.g[A1][0] = c.g[S1][0];
        }
        if c.g[S0][0] != want {
            if a0_first {
                c.cycles += 1;
                c.g[A1][0] = c.g[S1][0];
            }
            None
        } else {
            c.cycles += 2;
            c.g[V0][0] = special;
            Some(special)
        }
    };
    if ch == 0 {
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        return None;
    }
    c.cycles += 2;
    c.g[V0][0] = ((ch as i64) < 25) as u64;
    if ch == 0x18 {
        return pick(c, 4, 0x9f, true);
    }
    c.cycles += 2;
    let below_25 = c.g[V0][0] != 0;
    c.g[V0][0] = 0x14;
    if below_25 {
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if ch == 0x14 {
            return pick(c, 4, 0xa0, false);
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1c;
        if ch == 0x19 {
            return pick(c, 9, 0xa2, true);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if ch == 0x1c {
            return pick(c, 4, 0xa1, false);
        }
    }
    c.cycles += 2;
    c.g[A1][0] = c.g[S1][0];
    None
}

/// `int BEBAGTInt_GetTrickAnim(BEBAGTInt* self, int rider, int trick, int
/// second)` (0x150198): the animation a rider plays for a trick. Some
/// alternate characters have a fixed animation for one trick (0x14, 0x18 and
/// 0x1c for trick 4, 0x19 for trick 9); otherwise look up the rider's slot
/// for the trick and return `trick_table[trick].anims[slot]`, or 0x1b6 when
/// the trick has no slot.
pub fn bebagt_int_get_trick_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = c.g[S1][0];
        TRICK_ANIM_ALT_CHARACTER.call(c, 0x0015_01c8)?;
        let ch = c.g[V0][0];
        if trick_anim_special(c, ch).is_none() {
            c.cycles += 6;
            c.g[A3][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            TRICK_ANIM_SLOT_VALUE.call(c, 0x0015_0274)?;
            let slot = c.g[V0][0];
            c.g[V1][0] = slot;
            c.g[A1][0] = TRICK_ANIM_ENTRY_SIZE as u64;
            if (slot as i64) < 0 {
                c.cycles += 1;
                c.g[V0][0] = TRICK_ANIM_NONE;
            } else {
                c.cycles += 9;
                let trick = r32(c, S0);
                let entry_off = abi::mult(c, trick, TRICK_ANIM_ENTRY_SIZE);
                c.g[A1][0] = sx(entry_off);
                let entry = TRICK_ANIM_TABLE.wrapping_add(entry_off);
                c.g[V0][0] = sx(entry);
                let anims = Addr(entry.wrapping_add(TRICK_ANIM_ANIMS)).lw(c);
                c.g[A0][0] = anims;
                let at = (slot as u32).wrapping_shl(3).wrapping_add(anims as u32);
                c.g[V1][0] = sx(at);
                c.g[V0][0] = Addr(at).lhu(c);
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
    });
}

// ---------------------------------------------------------------------------
// ComputerActorRider_CruiseApproachJump (0x1009e0)

/// `ComputerActorRider_FindJumpMarker(self, &speed, &below, &o3, &o4, &o5, &o6)`.
const CAJ_FIND_JUMP_MARKER: Callee =
    Callee { addr: 0x0010_b980, f: ee::s00::ComputerActorRider_FindJumpMarker };
/// `ComputerActorRider_CruiseFollowPath(self, pad)`.
const CAJ_CRUISE_FOLLOW_PATH: Callee =
    Callee { addr: 0x0010_0680, f: ee::s00::ComputerActorRider_CruiseFollowPath };
/// `ComputerActorRider_GetSteerTarget(&v, self)`.
const CAJ_GET_STEER_TARGET: Callee =
    Callee { addr: 0x0010_0610, f: ee::s00::ComputerActorRider_GetSteerTarget };
/// `float ComputerActorRider_ComputeSteer(self, &v)`.
const CAJ_COMPUTE_STEER: Callee =
    Callee { addr: 0x0010_0348, f: ee::s00::ComputerActorRider_ComputeSteer };
/// `ComputerActorRider_PlanAirTricks(self, o5, o6, o4, o3)`.
const CAJ_PLAN_AIR_TRICKS: Callee =
    Callee { addr: 0x0010_bfa8, f: ee::s00::ComputerActorRider_PlanAirTricks };
/// `ComputerActorRider_CrouchAndBoost(self, pad)`.
const CAJ_CROUCH_AND_BOOST: Callee =
    Callee { addr: 0x0010_c140, f: ee::s00::ComputerActorRider_CrouchAndBoost };
/// ComputerActorRider: rider (+0x18), cruise speed (+0xdf0), think state
/// (+0xf38, `u16`) and think handler (+0xf44, an 8-byte member pointer).
const CAJ_RIDER: u32 = 0x18;
const CAJ_SPEED: u32 = 0xdf0;
const CAJ_THINK_STATE: u32 = 0xf38;
const CAJ_THINK_FN: u32 = 0xf44;
/// The think-handler table (0x43cf20); entry 1 (cruise) at +8.
const CAJ_THINK_TABLE: u32 = 0x0043_cf20;
/// Rider: velocity (+0x1e0) and height above ground (+0x4c8).
const CAJ_RIDER_VELOCITY: u32 = 0x1e0;
const CAJ_RIDER_HEIGHT: u32 = 0x4c8;
/// `gp-0x7fc4`: 10000.0 (height limit); `gp-0x7fc0`: 138.8889 (speed band).
const CAJ_HEIGHT_LIMIT_GP: u32 = 0x7fc4;
const CAJ_SPEED_BAND_GP: u32 = 0x7fc0;

/// `void ComputerActorRider_CruiseApproachJump(ComputerActorRider* self,
/// Pad* pad)` (0x1009e0): approaching a jump marker while cruising. No
/// marker ahead: go back to the cruise think state and follow the path.
/// Otherwise steer to the target (stick x, 6 bits at bit 20 of pad word 0),
/// and if the marker is below, the rider is not too high and the steer is
/// small, plan the air tricks and press both trick buttons (bits 14-15);
/// else crouch and boost when slower than the marker speed by more than
/// 138.9, or set the pad's slow-down bits (0x1f) when faster by more.
pub fn computer_actor_rider_cruise_approach_jump(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        let base = sp(c);
        c.g[A2][0] = sx(base.wrapping_add(0x10));
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        let me = r32(c, S0);
        c.g[A1][0] = sx(me.wrapping_add(CAJ_SPEED));
        c.g[A3][0] = sx(base.wrapping_add(0x14));
        c.g[T0][0] = sx(base.wrapping_add(0x18));
        c.g[T1][0] = sx(base.wrapping_add(0x1c));
        c.g[T2][0] = sx(base.wrapping_add(0x20));
        CAJ_FIND_JUMP_MARKER.call(c, 0x0010_0a14)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[SP][0];
        if !found {
            // Back to cruising: think handler = table[1], state 1.
            c.cycles += 13;
            c.g[V1][0] = 1;
            c.g[V0][0] = sx(CAJ_THINK_TABLE);
            c.g[A0][0] = c.g[S0][0];
            c.ldl(A1, CAJ_THINK_TABLE + 0xf);
            c.ldr(A1, CAJ_THINK_TABLE + 0x8);
            let me = r32(c, S0);
            c.sdl(A1, me.wrapping_add(CAJ_THINK_FN + 7));
            c.sdr(A1, me.wrapping_add(CAJ_THINK_FN));
            c.g[A1][0] = c.g[S1][0];
            Addr(me.wrapping_add(CAJ_THINK_STATE)).sh(c, 1);
            CAJ_CRUISE_FOLLOW_PATH.call(c, 0x0010_0a48)?;
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 33;
            c.g[A1][0] = c.g[S0][0];
            CAJ_GET_STEER_TARGET.call(c, 0x0010_0a58)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            CAJ_COMPUTE_STEER.call(c, 0x0010_0a64)?;
            // Stick x = (int)(steer * 31), 6 bits at bit 20 of pad word 0.
            let steer = FpuF32(c.f[0]);
            c.f[2] = steer.0;
            let pad = r32(c, S1);
            let word = Addr(pad).lw(c);
            c.g[AT][0] = 0x41f8_0000;
            c.g[V0][0] = 0xffff_ffff_fc0f_ffff;
            c.g[A0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
            let scaled = steer * FpuF32(0x41f8_0000);
            c.f[0] = scaled.0;
            c.g[V1][0] = word & c.g[V0][0];
            c.f[1] = scaled.to_i32() as u32;
            c.g[V0][0] = sx(c.f[1]) & 0x3f;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
            c.g[V1][0] |= c.g[V0][0];
            let packed = c.g[V1][0];
            Addr(pad).sw(c, packed);
            // |rider velocity| (VU0 macro mode, as the original).
            let me = r32(c, S0);
            c.g[V1][0] = Addr(me.wrapping_add(CAJ_RIDER)).lw(c);
            let rider = r32(c, V1);
            c.lqc2(3, rider.wrapping_add(CAJ_RIDER_VELOCITY));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(V0, 22);
            let len = FpuF32(c.g[V0][0] as u32);
            c.f[3] = len.0;
            // Marker below, rider low enough and a small steer: air tricks.
            let mut tricks = false;
            if c.g[A0][0] != 0 {
                c.cycles += 6;
                let height = Addr(rider.wrapping_add(CAJ_RIDER_HEIGHT)).read_fpu(c);
                c.f[1] = height.0;
                let limit = Addr(r32(c, GP).wrapping_sub(CAJ_HEIGHT_LIMIT_GP)).read_fpu(c);
                c.f[0] = limit.0;
                if abi::c_lt_s(c, height, limit) {
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f80_0000;
                    c.f[1] = 0x3f80_0000;
                    let mag = steer.abs();
                    c.f[0] = mag.0;
                    let small = abi::c_lt_s(c, mag, FpuF32(0x3f80_0000));
                    c.g[A1][0] = Addr(sp(c).wrapping_add(0x1c)).lw(c);
                    if small {
                        tricks = true;
                    } else {
                        c.cycles += 1;
                    }
                }
            }
            if tricks {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                let base = sp(c);
                c.g[A2][0] = Addr(base.wrapping_add(0x20)).lw(c);
                c.g[A3][0] = Addr(base.wrapping_add(0x18)).lw(c);
                c.g[T0][0] = Addr(base.wrapping_add(0x14)).lw(c);
                CAJ_PLAN_AIR_TRICKS.call(c, 0x0010_0b1c)?;
                let pad = r32(c, S1);
                c.g[V0][0] = Addr(pad).lw(c) | 0xc000;
                let v = c.g[V0][0];
                Addr(pad).sw(c, v);
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
            } else {
                let me = r32(c, S0);
                let speed = Addr(me.wrapping_add(CAJ_SPEED)).read_fpu(c);
                c.f[1] = speed.0;
                c.cycles += 6;
                let band = Addr(r32(c, GP).wrapping_sub(CAJ_SPEED_BAND_GP)).read_fpu(c);
                c.f[2] = band.0;
                let low = speed - band;
                c.f[0] = low.0;
                let slow = abi::c_lt_s(c, len, low);
                c.g[A0][0] = c.g[S0][0];
                if slow {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    CAJ_CROUCH_AND_BOOST.call(c, 0x0010_0b50)?;
                    frame.restore(c, S0, 0x50);
                } else {
                    c.cycles += 5;
                    let high = speed + band;
                    c.f[0] = high.0;
                    let fast = abi::c_lt_s(c, high, len);
                    c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
                    if fast {
                        c.cycles += 4;
                        let pad = r32(c, S1);
                        c.g[V0][0] = (Addr(pad.wrapping_add(4)).lw(c) & c.g[V1][0]) | 0x1f;
                        let v = c.g[V0][0];
                        Addr(pad.wrapping_add(4)).sw(c, v);
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x50);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// VfxVisualEffectsTest_Render (0x38f800)

/// `cWorldPainterMan_DebugDraw()` (0x2bb6b8, through the table).
const VFX_TEST_RENDER_DEBUG_DRAW: u32 = 0x002b_b6b8;

/// `void VfxVisualEffectsTest_Render(void)` (0x38f800): the visual-effects
/// test's render hook: draw the world painter's debug view.
pub fn vfx_visual_effects_test_render(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(VFX_TEST_RENDER_DEBUG_DRAW),
        0x0038_f810,
    );
}

// ---------------------------------------------------------------------------
// cCSICubicSpline_AddPoint (0x31d7e0)

/// `cCSICubicSpline_Solve(spline)`: compute the second derivatives.
const SPLINE_ADD_SOLVE: Callee = Callee { addr: 0x0031_d8b0, f: ee::s10::cCSICubicSpline_Solve };
/// cCSICubicSpline: flags word at +0 (bit 2 solved, bits 3..10 points
/// added, bits 11..18 capacity), the keys at +8.
const SPLINE_ADD_FLAGS: u32 = 0x0;
const SPLINE_ADD_KEYS: u32 = 0x8;
const SPLINE_ADD_SOLVED: u64 = 0x4;
/// A key: 0x14 bytes, `{x, y, end_x, end_y, d2}`.
const SPLINE_ADD_KEY_SIZE: u32 = 0x14;

/// `void cCSICubicSpline_AddPoint(cCSICubicSpline* self, float x, float y)`
/// (0x31d7e0): append the key `(x, y)`; when the spline is already full,
/// store `(x, y)` as the last key's end tangent instead, solve the spline
/// and mark it solved.
pub fn c_csi_cubic_spline_add_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let me = r32(c, S0);
        let flags = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) as u32;
        c.g[V1][0] = sx(flags >> 11) & 0xff;
        c.g[V0][0] = sx(flags >> 3);
        c.g[A2][0] = c.g[V0][0] & 0xff;
        let added = r32(c, A2);
        let x = FpuF32(c.f[12]);
        let y = FpuF32(c.f[13]);
        if c.g[A2][0] == c.g[V1][0] {
            // Full: the last key's end point, then solve.
            c.cycles += 17;
            c.g[A1][0] = SPLINE_ADD_KEY_SIZE as u64;
            c.g[V0][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let end = abi::mult(c, added, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V0));
            c.g[V1][0] = sx(end);
            Addr(end.wrapping_sub(0xc)).write_fpu(c, x);
            let flags = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) as u32;
            c.g[V1][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let n = (flags >> 3) & 0xff;
            let end = abi::mult(c, n, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(end);
            Addr(end.wrapping_sub(0x8)).write_fpu(c, y);
            SPLINE_ADD_SOLVE.call(c, 0x0031_d840)?;
            let me = r32(c, S0);
            c.g[V0][0] = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) | SPLINE_ADD_SOLVED;
            let v = c.g[V0][0];
            Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).sw(c, v);
        } else {
            // Append key `added`.
            c.cycles += 20;
            c.g[A0][0] = SPLINE_ADD_KEY_SIZE as u64;
            c.g[V0][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let key = abi::mult(c, added, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V0));
            c.g[A1][0] = 0xffff_ffff_ffff_f807;
            c.g[V1][0] = sx(key);
            Addr(key).write_fpu(c, x);
            let flags = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c) as u32;
            c.g[V1][0] = Addr(me.wrapping_add(SPLINE_ADD_KEYS)).lw(c);
            let n = (flags >> 3) & 0xff;
            let key = abi::mult(c, n, SPLINE_ADD_KEY_SIZE).wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(key);
            Addr(key.wrapping_add(4)).write_fpu(c, y);
            // added += 1 (8 bits at bit 3).
            c.g[V1][0] = Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).lw(c);
            let next = (sx(r32(c, V1) >> 3) as u32).wrapping_add(1) & 0xff;
            c.g[V0][0] = sx(next << 3);
            c.g[V1][0] = (c.g[V1][0] & c.g[A1][0]) | c.g[V0][0];
            let v = c.g[V1][0];
            Addr(me.wrapping_add(SPLINE_ADD_FLAGS)).sw(c, v);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RiderAnim_PlaySeqThenLoop: track 0 plays the intro key (or not), the
    /// intro reaches its end (or not) after the real AdvanceTrack, the real
    /// weight fade finishes (or not) and the real list removal runs.
    #[test]
    fn rider_anim_play_seq_then_loop_matches() {
        use crate::difftest::fakes::{machine, ptr};
        use crate::difftest::{check, elf_image, GAME_GP};
        check(ee::s00::f_00103ff8__generated, rider_anim_play_seq_then_loop, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0010_3ff8, 0);
            elf_image(c);
            c.pc = 0x0010_3ff8;
            let bank = ptr(&mut r);
            c.bus.write32(GAME_GP + PSTL_ANIM_BANK_GP, bank);
            for k in 0..8 {
                c.bus.write32(bank + PSTL_BANK_IDS + 4 * k, 0x100 + k * 0x101);
            }
            let intro = r.next_u32() % 8;
            c.g[A3][0] = intro as u64;
            c.g[T0][0] = (r.next_u32() % 8) as u64;
            let seq = ptr(&mut r);
            let list = ptr(&mut r);
            c.g[A0][0] = sx(ptr(&mut r));
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(seq);
            for off in [0x8, 0xc, 0x10, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                let v = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.edge_f32(),
                    _ => r.f32(3.0),
                };
                c.bus.write32(seq + off, v);
            }
            let cur = if seed % 3 == 0 {
                0x100 + (r.next_u32() % 8) * 0x101
            } else {
                0x100 + intro * 0x101
            };
            c.bus.write32(seq + PSTL_SEQ_ANIM0, cur);
            c.bus.write32(seq + 0x1c, r.next_u32() & 1);
            c.bus.write32(seq + 0xa0, r.next_u32() & 1);
            let mut at = list + 4;
            for _ in 0..seed % 3 {
                let other = ptr(&mut r) + 0x8;
                c.bus.write32(at, other);
                at = other + 200;
            }
            c.f[12] = if seed % 4 == 0 { r.edge_f32() } else { r.f32(3.0) };
        });
    }

    /// AnimSeqList_PushNew: the real allocator (size-class pools ready) and
    /// the real AnimSequence_ctor; an empty or non-empty list.
    #[test]
    fn anim_seq_list_push_new_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image, DATA};
        use crate::game::bx::bxstring::tests::pool;
        check(ee::s10::f_003142f8__generated, anim_seq_list_push_new, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_42f8, 0);
            elf_image(c);
            pool(c);
            c.pc = 0x0031_42f8;
            let list = DATA + 0x100;
            c.bus.write32(list + PUSH_NEW_LIST_COUNT, seed % 5);
            let first = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(list + PUSH_NEW_LIST_FIRST, first);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = (r.next_u32() % 16) as u64;
        });
    }

    /// ModObj_v38: no block, a block without a node, or a node whose slot
    /// 0xa0 is a fake method (which may unwind).
    #[test]
    fn mod_obj_v38_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        covered(ee::s11::f_00356a28__generated, mod_obj_v38, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_6a28, 1);
            c.pc = 0x0035_6a28;
            let this = DATA + 0x100;
            let block = DATA + 0x200;
            let node = DATA + 0x300;
            c.g[A0][0] = sx(this);
            match seed % 4 {
                0 => c.bus.write32(this + MODOBJ_V38_BLOCK, 0),
                1 => {
                    c.bus.write32(this + MODOBJ_V38_BLOCK, block);
                    c.bus.write32(block, 0);
                }
                _ => {
                    c.bus.write32(this + MODOBJ_V38_BLOCK, block);
                    c.bus.write32(block, node);
                    let vt = vtable(c, &mut r, &[MODOBJ_V38_SLOT]);
                    c.bus.write32(node, vt);
                }
            }
        });
    }

    /// UVScrollModifier_Save: the stream's write method is a fake (which may
    /// unwind).
    #[test]
    fn uv_scroll_modifier_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        covered(ee::s12::f_0035fd98__generated, uv_scroll_modifier_save, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_fd98, 1);
            c.pc = 0x0035_fd98;
            let out = DATA + 0x300;
            let vt = vtable(c, &mut r, &[UV_SCROLL_SAVE_WRITE_SLOT]);
            c.bus.write32(out, vt);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(out);
        });
    }

    /// SND_EeVoiceSetVolume: a voice table in test memory, random voice,
    /// channel and volume.
    #[test]
    fn snd_ee_voice_set_volume_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        check_lohi(ee::s14::f_003c8560__generated, snd_ee_voice_set_volume, 200, &|c, seed| {
            let mut r = Rng(0x003c_8560 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x003c_8560;
            c.bus.write32(SND_SET_VOL_STATE + SND_SET_VOL_VOICES, DATA);
            c.g[A0][0] = (r.next_u32() % 48) as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.f[12] = r.edge_f32();
        });
    }

    /// BEBAGTInt_GetTrickAnim: the real character/profile getters over a
    /// test rider table (alternate character 0, each special one, and
    /// others), the real slot lookup over a test trick table (tricks with
    /// and without animations) and slot bytes.
    #[test]
    fn bebagt_int_get_trick_anim_matches() {
        use crate::difftest::{check_lohi, elf_image, fill, scramble, Rng, DATA, GAME_GP};
        const RIDER_PROFILES: u32 = 0x0053_05b0;
        const PROFILES: u32 = 0x0053_5b20;
        const SLOTS: u32 = 0x0053_0ec0;
        check_lohi(ee::s01::f_00150198__generated, bebagt_int_get_trick_anim, 300, &|c, seed| {
            let mut r = Rng(0x0015_0198 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0015_0198;
            let rider = r.next_u32() % 4;
            let profile = r.next_u32() % 3;
            c.bus.write32(RIDER_PROFILES + 4 * rider, profile);
            let entry = PROFILES + 0x1c * profile;
            fill(c, &mut r, entry, 7);
            c.bus.write32(entry + 0xc, if seed % 5 == 0 { u32::MAX } else { r.next_u32() });
            c.bus.write8(entry + 0x11, (r.next_u32() % 8) as u8);
            let chars = [0u8, 0x14, 0x18, 0x19, 0x1c, 5, 30, 0xf0];
            c.bus.write8(entry + 0x12, chars[(seed % 8) as usize]);
            for i in 0..0x4000u32 {
                c.bus.write8(SLOTS + i, (mix_small(seed, i) % 8) as u8);
            }
            for t in 0..12u32 {
                let e = TRICK_ANIM_TABLE + TRICK_ANIM_ENTRY_SIZE * t;
                c.bus.write16(e + 0xc, if t % 3 == 0 { 0 } else { 8 });
                let anims = DATA + 0x1000 + 0x100 * t;
                c.bus.write32(e + TRICK_ANIM_ANIMS, anims);
                fill(c, &mut r, anims, 0x40);
            }
            let trick = match seed % 4 {
                0 => 4,
                1 => 9,
                _ => r.next_u32() % 12,
            };
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = rider as u64;
            c.g[A2][0] = trick as u64;
            c.g[A3][0] = (r.next_u32() % 2) as u64;
        });
        fn mix_small(seed: u32, i: u32) -> u32 {
            (seed.wrapping_mul(0x9e37_79b9) ^ i.wrapping_mul(0x85eb_ca6b)).rotate_left(13)
        }
    }

    // ----------------------------------------- CruiseApproachJump fixture

    /// The test's ComputerActorRider, its rider and pad, the record of the
    /// fake callees' arguments and the fakes' answers.
    const CAJ_SELF: u32 = crate::difftest::DATA;
    const CAJ_RIDER_AT: u32 = crate::difftest::DATA + 0x2000;
    const CAJ_PAD: u32 = crate::difftest::DATA + 0x3000;
    const CAJ_REC: u32 = crate::difftest::DATA + 0x4000;
    const CAJ_CTRL: u32 = crate::difftest::DATA + 0x5000;

    /// Log a fake call's argument registers, `ra`, `sp` and cycles at
    /// `CAJ_REC + slot * 0x80` (the RAM comparison checks them).
    fn caj_record(c: &mut Cpu, slot: u32) {
        let at = CAJ_REC + slot * 0x80;
        for (i, r) in [A0, A1, A2, A3, T0, T1, T2, RA, SP, S0, S1].into_iter().enumerate() {
            c.bus.write64(at + 8 * i as u32, c.g[r][0]);
        }
        c.bus.write64(at + 0x60, c.cycles);
        c.bus.write32(at + 0x68, c.f[12]);
    }

    /// `FindJumpMarker`: found = ctrl+0; writes below (ctrl+4) and four
    /// marker words through a2, a3, t0, t1, t2.
    fn caj_fake_find_jump_marker(c: &mut Cpu) {
        caj_record(c, 0);
        let found = c.bus.read32(CAJ_CTRL);
        let below = c.bus.read32(CAJ_CTRL + 4);
        let a2 = c.g[A2][0] as u32;
        c.bus.write32(a2, below);
        for (i, r) in [A3, T0, T1, T2].into_iter().enumerate() {
            let p = c.g[r][0] as u32;
            c.bus.write32(p, 0x100 + i as u32 * 3 + found);
        }
        c.g[V0][0] = found as u64;
        c.cycles += 11;
        abi::ret(c);
    }

    /// `CruiseFollowPath`: only logged.
    fn caj_fake_cruise_follow_path(c: &mut Cpu) {
        caj_record(c, 1);
        c.cycles += 7;
        abi::ret(c);
    }

    /// `GetSteerTarget(&v, self)`: writes a vector from ctrl+0x10.
    fn caj_fake_get_steer_target(c: &mut Cpu) {
        caj_record(c, 2);
        let v = c.g[A0][0] as u32;
        for i in 0..4 {
            let w = c.bus.read32(CAJ_CTRL + 0x10 + 4 * i);
            c.bus.write32(v + 4 * i, w);
        }
        c.cycles += 5;
        abi::ret(c);
    }

    /// `ComputeSteer`: returns the steer at ctrl+8 in f0.
    fn caj_fake_compute_steer(c: &mut Cpu) {
        caj_record(c, 3);
        c.f[0] = c.bus.read32(CAJ_CTRL + 8);
        c.cycles += 9;
        abi::ret(c);
    }

    /// Installs the fakes in the callees' override slots; restores them on
    /// drop.
    struct CajSlots([fn(&mut Cpu); 4]);

    impl CajSlots {
        fn install() -> CajSlots {
            unsafe {
                let saved = CajSlots([
                    ssx3_ee::s00::OVERRIDE_0010B980,
                    ssx3_ee::s00::OVERRIDE_00100680,
                    ssx3_ee::s00::OVERRIDE_00100610,
                    ssx3_ee::s00::OVERRIDE_00100348,
                ]);
                ssx3_ee::s00::OVERRIDE_0010B980 = caj_fake_find_jump_marker;
                ssx3_ee::s00::OVERRIDE_00100680 = caj_fake_cruise_follow_path;
                ssx3_ee::s00::OVERRIDE_00100610 = caj_fake_get_steer_target;
                ssx3_ee::s00::OVERRIDE_00100348 = caj_fake_compute_steer;
                saved
            }
        }
    }

    impl Drop for CajSlots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s00::OVERRIDE_0010B980 = self.0[0];
                ssx3_ee::s00::OVERRIDE_00100680 = self.0[1];
                ssx3_ee::s00::OVERRIDE_00100610 = self.0[2];
                ssx3_ee::s00::OVERRIDE_00100348 = self.0[3];
            }
        }
    }

    /// ComputerActorRider_CruiseApproachJump: no marker (back to cruising),
    /// or a marker with the rider below/above it, high or low, a small or
    /// large steer (the real PlanAirTricks), and slower (the real
    /// CrouchAndBoost), within or faster than the speed band. The four
    /// callees with override slots are fakes.
    #[test]
    fn computer_actor_rider_cruise_approach_jump_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image};
        let _slots = CajSlots::install();
        check(
            ee::s00::f_001009e0__generated,
            computer_actor_rider_cruise_approach_jump,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_09e0, 0);
                elf_image(c);
                c.pc = 0x0010_09e0;
                c.g[A0][0] = sx(CAJ_SELF);
                c.g[A1][0] = sx(CAJ_PAD);
                c.bus.write32(CAJ_SELF + CAJ_RIDER, CAJ_RIDER_AT);
                let speed = 50.0 + (r.next_u32() % 250) as f32;
                c.bus.write32(CAJ_SELF + CAJ_SPEED, speed.to_bits());
                c.bus.write32(CAJ_SELF + 0xdfc, r.f32(1.0));
                c.bus.write32(CAJ_PAD, r.next_u32());
                c.bus.write32(CAJ_PAD + 4, r.next_u32());
                // Rider velocity: |v| around the cruise speed, +-300.
                let delta = f32::from_bits(r.f32(300.0));
                let target = (speed + delta).max(0.0);
                let (x, y) = (r.f32(1.0), r.f32(1.0));
                let (x, y) = (f32::from_bits(x), f32::from_bits(y));
                let n = (x * x + y * y + 1.0).sqrt();
                for (i, v) in [x / n, y / n, 1.0 / n, 0.0].into_iter().enumerate() {
                    c.bus.write32(CAJ_RIDER_AT + CAJ_RIDER_VELOCITY + 4 * i as u32, (v * target).to_bits());
                }
                let height = if seed % 3 == 0 { 20000.0f32 } else { 5.0 };
                c.bus.write32(CAJ_RIDER_AT + CAJ_RIDER_HEIGHT, height.to_bits());
                // The fakes' answers.
                c.bus.write32(CAJ_CTRL, (seed % 5 != 0) as u32);
                c.bus.write32(CAJ_CTRL + 4, (seed % 2) as u32);
                let steer = match seed % 7 {
                    0 => r.edge_f32(),
                    1 => 2.5f32.to_bits(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(CAJ_CTRL + 8, steer);
                for i in 0..4 {
                    let w = r.f32(100.0);
                    c.bus.write32(CAJ_CTRL + 0x10 + 4 * i, w);
                }
            },
        );
    }


    /// VfxVisualEffectsTest_Render: the real cWorldPainterMan_DebugDraw over
    /// a test world, with the debug-draw flag (gp+0x13c4) off (its short
    /// path) or on with no painter entry.
    #[test]
    fn vfx_visual_effects_test_render_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        check(ee::s12::f_0038f800__generated, vfx_visual_effects_test_render, 60, &|c, seed| {
            let _ = machine(c, seed, 0x0038_f800, 0);
            c.pc = 0x0038_f800;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let painter = DATA + 0x300;
            let current = DATA + 0x400;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, man);
            c.bus.write32(man + 0x10, painter);
            c.bus.write32(painter + 0x8, current);
            c.bus.write32(GAME_GP + 0x13c4, seed % 2);
            // Flag on: entry table at gp+0x770 empty (null entry), index 0.
            c.bus.write32(GAME_GP + 0x770, 0);
            c.bus.write32(current + 0x4, 0);
        });
    }

    /// cCSICubicSpline_AddPoint: room left (append) or full (end point and
    /// the real Solve over a small sorted spline); random flag bits around
    /// the counts.
    #[test]
    fn c_csi_cubic_spline_add_point_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        check_lohi(ee::s10::f_0031d7e0__generated, c_csi_cubic_spline_add_point, 300, &|c, seed| {
            let mut r = Rng(0x0031_d7e0 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x0031_d7e0;
            let spline = DATA + 0x100;
            let keys = DATA + 0x400;
            let cap = 2 + r.next_u32() % 6;
            let added = if seed % 3 == 0 { cap } else { r.next_u32() % cap };
            let other = r.next_u32() & !(0xffff << 3);
            c.bus.write32(spline + SPLINE_ADD_FLAGS, other | added << 3 | cap << 11);
            c.bus.write32(spline + SPLINE_ADD_KEYS, keys);
            let mut x = 0.0f32;
            for k in 0..cap {
                x += 0.25 + (r.next_u32() % 100) as f32 / 50.0;
                let at = keys + SPLINE_ADD_KEY_SIZE * k;
                c.bus.write32(at, x.to_bits());
                c.bus.write32(at + 4, r.f32(10.0));
                c.bus.write32(at + 8, r.f32(10.0));
                c.bus.write32(at + 0xc, r.f32(10.0));
                c.bus.write32(at + 0x10, r.f32(10.0));
            }
            c.g[A0][0] = sx(spline);
            c.f[12] = r.f32(10.0);
            c.f[13] = if seed % 5 == 0 { r.edge_f32() } else { r.f32(10.0) };
        });
    }
}
