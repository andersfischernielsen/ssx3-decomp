//! Staging wave 212 (oc310): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// RiderMetrix_CountTrickRepeats (0x1190f0)

/// Rider metrics: the 10-entry history of recent trick ids (8 bytes each).
pub(crate) const COUNT_TRICK_REPEATS_HIST: u32 = 0xa8;
/// Rider metrics: next history slot (0..9).
pub(crate) const COUNT_TRICK_REPEATS_HIST_IDX: u32 = 0xf8;
/// Rider metrics: the state words that suspend trick counting when set.
pub(crate) const COUNT_TRICK_REPEATS_BUSY: [u32; 5] = [0x08, 0x20, 0x70, 0x7c, 0x28];

/// `int RiderMetrix_CountTrickRepeats(RiderMetrix* self, sTrickId* id)`
/// (0x1190f0): how often the 8-byte trick id already appears in the
/// rider's 10-entry recent-trick history (0 for a few trick classes that
/// never count as repeats), then records it at the history cursor and
/// advances the cursor modulo 10. Returns 0 without recording while any of
/// the busy state words is set.
pub fn rider_metrix_count_trick_repeats(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[T3][0] = c.g[A1][0];
    let me = c.g[A3][0] as u32;
    let id = c.g[T3][0] as u32;
    let mut busy = 0u64;
    for (k, off) in COUNT_TRICK_REPEATS_BUSY.into_iter().enumerate() {
        if k > 0 {
            c.cycles += 3;
        }
        busy = Addr(me.wrapping_add(off)).lw(c);
        if busy != 0 {
            break;
        }
    }
    if busy != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }

    // Trick classes that are never counted as repeats.
    c.cycles += 7;
    let word1 = Addr(id.wrapping_add(4)).lw(c);
    c.g[V1][0] = 0x3f800;
    c.g[T2][0] = 0; // repeats found
    c.g[V0][0] = word1 & 0x3f800;
    c.g[A0][0] = 0x800;
    c.g[T5][0] = Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).lw(c);
    if word1 & 0x3f800 != 0x800 {
        c.cycles += 6;
        let word0 = Addr(id).lw(c);
        c.g[A0][0] = 0x400000;
        c.g[V0][0] = word0 & 0xfc0_0000;
        c.g[V1][0] = 0; // history index
        if word0 & 0xfc0_0000 != 0x40_0000 {
            c.cycles += 3;
            c.g[T4][0] = u64::MAX;
            c.g[V0][0] = 0; // index * 8
            count_trick_repeats_scan(c);
        }
    }

    // Record the id at the cursor; cursor = (cursor + 1) % 10.
    c.cycles += 14;
    let me = c.g[A3][0] as u32;
    let src = c.g[T3][0] as u32;
    let entry = ((c.g[T5][0] as u32) << 3).wrapping_add(me);
    c.g[A0][0] = sx(entry);
    c.g[A1][0] = 10;
    c.g[V0][0] = c.g[T2][0];
    let trick = Addr(src).read_u64_unaligned(c);
    c.g[V1][0] = trick;
    Addr(entry.wrapping_add(COUNT_TRICK_REPEATS_HIST)).write_u64_unaligned(c, trick);
    let cursor = (Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).lw(c) as u32).wrapping_add(1);
    c.g[V1][0] = sx(cursor);
    abi::div(c, cursor, 10);
    Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).sw(c, sx(cursor));
    c.cycles += 3;
    let wrapped = c.hi[0];
    c.g[A0][0] = wrapped;
    Addr(me.wrapping_add(COUNT_TRICK_REPEATS_HIST_IDX)).sw(c, wrapped);
    abi::ret(c);
}

/// The repeat scan of [`rider_metrix_count_trick_repeats`]: byte-compares
/// the id (t3) with each of the 10 history entries, counting matches in t2.
/// Works on the registers (v1 = index, v0 = index * 8, a3 = self) so the
/// back-edge event checks see the original's state.
fn count_trick_repeats_scan(c: &mut Cpu) {
    loop {
        c.cycles += 6;
        c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[A1][0] = c.g[T3][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(COUNT_TRICK_REPEATS_HIST));
        c.g[A2][0] = 7;
        c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        // 8-byte compare (a2 counts 7..-1).
        loop {
            c.cycles += 4;
            let a = c.lbu(c.g[A1][0] as u32);
            let b = c.lbu(c.g[A0][0] as u32);
            c.g[V1][0] = a;
            c.g[V0][0] = 0;
            if a != b {
                break;
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if c.g[A2][0] != c.g[T4][0] {
                if event_due(c) {
                    deliver_event_here(c, 0x0011_9190);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
            break;
        }
        c.cycles += 5;
        c.g[V1][0] = c.g[T1][0];
        if c.g[V0][0] != 0 {
            c.g[T2][0] = c.g[T0][0];
        }
        let more = (c.g[V1][0] as i64) < 10;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0011_9178);
        }
    }
}

// ---------------------------------------------------------------------------
// AnimObject_BaseCtor (0x34d9b0)

/// AnimObject: current and start time (+0, +4), window start / end
/// (+8, +0xc, "never" = 1e10), the active flag (+0x10), the embedded
/// ModObj (+0x14; its vtable at +0x20, its node at +0x2c), the node-slot
/// array (+0x44) and the mesh instances (+0x48).
pub(crate) const ANIM_OBJECT_BASE_CTOR_MOD: u32 = 0x14;
pub(crate) const ANIM_OBJECT_BASE_CTOR_SLOTS: u32 = 0x44;
pub(crate) const ANIM_OBJECT_BASE_CTOR_MESHES: u32 = 0x48;
/// The AnimObject ModObj vtable (`D_00490CC8`) and the slot array's
/// allocation name (`D_0048E758`).
pub(crate) const ANIM_OBJECT_BASE_CTOR_VTABLE: u32 = 0x0049_0cc8;
pub(crate) const ANIM_OBJECT_BASE_CTOR_SLOT_NAME: u32 = 0x0048_e758;
/// gp-relative 1e10 ("no window").
pub(crate) const ANIM_OBJECT_BASE_CTOR_NEVER_GP: u32 = 0x2aa8;

/// `AnimObject* AnimObject_BaseCtor(AnimObject* self, int list, int kind,
/// Node* node, int force)` (0x34d9b0): constructs the embedded ModObj on
/// `node`, sets the AnimObject vtable, allocates one 64-byte slot per node
/// definition, builds the mesh instances (`AnimObject_AllocNodeInfo`),
/// starts at time 0 with the duration of the first mesh, an open window
/// (1e10), active; when `force` or the node's flag bit 0 is set, the node
/// flags get bit 2 instead of bit 1. Finally evaluates the meshes
/// (`AnimObject_v52`). Returns `self`.
pub fn anim_object_base_ctor(c: &mut Cpu) {
    const MOD_OBJ_CTOR: Callee = Callee {
        addr: 0x0035_5280,
        f: ee::s11::ModObj_ctor,
    };
    const ALLOC_NODE_INFO: Callee = Callee {
        addr: 0x0034_e448,
        f: ee::s11::AnimObject_AllocNodeInfo,
    };
    const GET_DURATION: Callee = Callee {
        addr: 0x0035_1508,
        f: ee::s11::IntersectInfo_GetDuration,
    };
    const V52: Callee = Callee {
        addr: 0x0034_e348,
        f: ee::s11::AnimObject_v52,
    };
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(ANIM_OBJECT_BASE_CTOR_MOD));
        frame.save(c, S2, 0x20);
        c.g[A0][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0]; // force
        MOD_OBJ_CTOR.call(c, 0x0034_d9dc)?;

        // The node-slot array: new (name, 0x20000000, 0) Slot[definitions].
        let me = r32(c, S1);
        c.g[S2][0] = sx(me.wrapping_add(ANIM_OBJECT_BASE_CTOR_SLOTS));
        c.g[V1][0] = sx(ANIM_OBJECT_BASE_CTOR_VTABLE);
        c.g[A1][0] = sx(ANIM_OBJECT_BASE_CTOR_SLOT_NAME);
        Addr(me).off(0x20).sw(c, sx(ANIM_OBJECT_BASE_CTOR_VTABLE));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(r32(c, S0)).off(0x18).lw(c); // node
        c.g[V1][0] = Addr(r32(c, V0)).off(0x80).lw(c); // node definitions
        c.g[S0][0] = Addr(r32(c, V1)).off(4).lw(c); // definition count
        c.g[A0][0] = sx(r32(c, S0) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_da10)?; // operator_new_named
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        let slots = c.g[V0][0];
        Addr(r32(c, S2)).sw(c, slots);
        ALLOC_NODE_INFO.call(c, 0x0034_da2c)?;

        // Start time: the first mesh's duration, else 0.
        c.g[A0][0] = Addr(r32(c, S1)).off(ANIM_OBJECT_BASE_CTOR_MESHES).lw(c);
        c.f[1] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            GET_DURATION.call(c, 0x0034_da44)?;
            c.f[1] = c.f[0];
        }

        c.cycles += 9;
        let me = r32(c, S1);
        let gp = r32(c, GP);
        c.f[0] = Addr(gp.wrapping_sub(ANIM_OBJECT_BASE_CTOR_NEVER_GP)).read_fpu(c).0;
        c.g[V0][0] = 1;
        let (start, never) = (c.f[1], c.f[0]);
        Addr(me).off(4).sw(c, start as u64);
        c.g[A0][0] = sx(me.wrapping_add(ANIM_OBJECT_BASE_CTOR_MOD));
        Addr(me).off(0x10).sw(c, 1);
        Addr(me).off(0xc).sw(c, never as u64);
        Addr(me).sw(c, start as u64);
        Addr(me).off(8).sw(c, never as u64);
        let set = if c.g[S3][0] != 0 {
            true
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr(r32(c, A0)).off(0x18).lw(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c) & 1;
            c.g[V0][0] != 0
        };
        if set {
            // Node flags: clear bit 1, set bit 2.
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, A0)).off(0x18).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            let flags = (Addr(r32(c, A0)).off(8).lw(c) & !2) | 4;
            c.g[V0][0] = flags;
            Addr(r32(c, A0)).off(8).sw(c, flags);
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        V52.call(c, 0x0034_daa4)?;
        c.g[V0][0] = c.g[S1][0];
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
// StreamFile_FreeRequest (0x3dfe18)

/// Stream file: the request list's head (+0x5c), current (+0x60) and tail
/// (+0x64), and the free-request list (+0x68).
pub(crate) const STREAM_FILE_FREE_REQUEST_HEAD: u32 = 0x5c;
pub(crate) const STREAM_FILE_FREE_REQUEST_CURRENT: u32 = 0x60;
pub(crate) const STREAM_FILE_FREE_REQUEST_TAIL: u32 = 0x64;
pub(crate) const STREAM_FILE_FREE_REQUEST_FREE: u32 = 0x68;
/// Request: in-use word (+4), previous (+8) and next (+0xc) links.
pub(crate) const STREAM_FILE_FREE_REQUEST_BUSY: u32 = 0x4;
pub(crate) const STREAM_FILE_FREE_REQUEST_PREV: u32 = 0x8;
pub(crate) const STREAM_FILE_FREE_REQUEST_NEXT: u32 = 0xc;

/// `void StreamFile_FreeRequest(StreamFile* file, Request* req)`
/// (0x3dfe18): unlinks `req` from the file's doubly linked request list
/// (fixing head / tail, and moving the current request to its neighbour,
/// next first), marks it unused and pushes it on the free list.
pub fn stream_file_free_request(c: &mut Cpu) {
    const PREV: u32 = STREAM_FILE_FREE_REQUEST_PREV;
    const NEXT: u32 = STREAM_FILE_FREE_REQUEST_NEXT;
    let file = Addr(c.g[A0][0] as u32);
    let req_word = c.g[A1][0];
    let req = Addr(req_word as u32);

    // prev->next = next (or head = next).
    c.cycles += 3;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_HEAD).lw(c);
    if req_word != c.g[V0][0] {
        c.g[V1][0] = req.off(PREV).lw(c);
        c.cycles += 2;
        c.g[V0][0] = req.off(NEXT).lw(c);
        let next = c.g[V0][0];
        Addr(c.g[V1][0] as u32).off(NEXT).sw(c, next);
    } else {
        c.cycles += 3;
        c.g[V0][0] = req.off(NEXT).lw(c);
        let next = c.g[V0][0];
        file.off(STREAM_FILE_FREE_REQUEST_HEAD).sw(c, next);
    }

    // next->prev = prev (or tail = prev).
    c.cycles += 3;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_TAIL).lw(c);
    if req_word != c.g[V0][0] {
        c.g[V1][0] = req.off(NEXT).lw(c);
        c.cycles += 2;
        c.g[V0][0] = req.off(PREV).lw(c);
        let prev = c.g[V0][0];
        Addr(c.g[V1][0] as u32).off(PREV).sw(c, prev);
    } else {
        c.cycles += 3;
        c.g[V0][0] = req.off(PREV).lw(c);
        let prev = c.g[V0][0];
        file.off(STREAM_FILE_FREE_REQUEST_TAIL).sw(c, prev);
    }

    // The current request moves to the next one, else the previous one.
    c.cycles += 3;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_CURRENT).lw(c);
    if req_word == c.g[V0][0] {
        c.cycles += 3;
        c.g[V0][0] = req.off(NEXT).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = req.off(PREV).lw(c);
        }
        c.cycles += 2;
        let current = c.g[V0][0];
        file.off(STREAM_FILE_FREE_REQUEST_CURRENT).sw(c, current);
    }
    req.off(STREAM_FILE_FREE_REQUEST_BUSY).sw(c, 0);

    // Push on the free list.
    c.cycles += 4;
    c.g[V0][0] = file.off(STREAM_FILE_FREE_REQUEST_FREE).lw(c);
    let free = c.g[V0][0];
    req.off(NEXT).sw(c, free);
    file.off(STREAM_FILE_FREE_REQUEST_FREE).sw(c, req_word);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider_GetPlayerCharacterInfo (0x123168)

/// Rider: its AI / player index (+0x86c).
pub(crate) const RIDER_GET_PLAYER_CHARACTER_INFO_AI_INDEX: u32 = 0x86c;

/// `int Rider_GetPlayerCharacterInfo(Rider* self)` (0x123168): the
/// alternate-character byte of the rider's player slot, from the BE
/// new-player interface (`BENewPlayerInt_GetAltCharacter(
/// BELibrary_GetInterface(BELibrary_GetInstance(), 1), self->aiIndex)`).
pub fn rider_get_player_character_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0012_317c)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0012_3188)?; // BELibrary_GetInterface
        c.g[A1][0] = Addr(r32(c, S0))
            .off(RIDER_GET_PLAYER_CHARACTER_INFO_AI_INDEX)
            .lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7410, 0x0012_3194)?; // BENewPlayerInt_GetAltCharacter
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_RetireRequest (0x3b6a70)

/// The sound-stream table (`0x50a0a0`, one pointer per stream index).
pub(crate) const SNDSTRM_RETIRE_REQUEST_STREAMS: u32 = 0x0050_a0a0;
/// Stream: the active request list (+0x11c), the free list (+0x128) and the
/// request being played (+0x134).
pub(crate) const SNDSTRM_RETIRE_REQUEST_ACTIVE: u32 = 0x11c;
pub(crate) const SNDSTRM_RETIRE_REQUEST_FREE: u32 = 0x128;
pub(crate) const SNDSTRM_RETIRE_REQUEST_PLAYING: u32 = 0x134;

/// `void SNDSTRM_RetireRequest(int handle)` (0x3b6a70): finds the request
/// for `handle` on its stream (stream index = low byte), moves it from the
/// active list to the front of the free list, and clears the stream's
/// playing request if it was that one.
pub fn sndstrm_retire_request(c: &mut Cpu) {
    const FIND_REQUEST: Callee = Callee {
        addr: 0x003b_7a70,
        f: ee::s13::SNDSTRM_FindRequest,
    };
    const LIST_REMOVE: Callee = Callee {
        addr: 0x003c_6320,
        f: ee::s13::SND_ListRemove,
    };
    const LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x003c_6288,
        f: ee::s13::SND_ListPushFront,
    };
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        let slot = sx((c.g[A0][0] as u32 & 0xff) << 2);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(SNDSTRM_RETIRE_REQUEST_STREAMS);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx((slot as u32).wrapping_add(SNDSTRM_RETIRE_REQUEST_STREAMS));
        c.g[S1][0] = Addr(r32(c, V0)).lw(c); // stream
        FIND_REQUEST.call(c, 0x003b_6a9c)?;
        c.g[S0][0] = c.g[V0][0]; // request
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(SNDSTRM_RETIRE_REQUEST_ACTIVE));
        c.g[A1][0] = c.g[S0][0];
        LIST_REMOVE.call(c, 0x003b_6aac)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(SNDSTRM_RETIRE_REQUEST_FREE));
        c.g[A1][0] = c.g[S0][0];
        LIST_PUSH_FRONT.call(c, 0x003b_6ab8)?;
        let playing = Addr(r32(c, S1)).off(SNDSTRM_RETIRE_REQUEST_PLAYING);
        c.g[V0][0] = playing.lw(c);
        if c.g[V0][0] == c.g[S0][0] {
            playing.sw(c, 0);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_RollSkipTricks (0x10c1d0)

/// Computer actor: the chance (0..1) of doing tricks (+0xdfc).
pub(crate) const ROLL_SKIP_TRICKS_CHANCE: u32 = 0xdfc;

/// `bool ComputerActorRider_RollSkipTricks(ComputerActorRider* self)`
/// (0x10c1d0): rolls a game random number in 0..99 and returns whether
/// `chance * 100 <= roll`, i.e. whether the AI skips its tricks this time.
pub fn computer_actor_rider_roll_skip_tricks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7810, 0x0010_c1e4)?; // Rand_NextGame
        c.g[V1][0] = 100;
        let chance = Addr(r32(c, S0)).off(ROLL_SKIP_TRICKS_CHANCE).read_fpu(c);
        let rand = r32(c, V0);
        let (_, roll) = abi::divu(c, rand, 100);
        c.g[AT][0] = 0x42c8_0000;
        let hundred = FpuF32(0x42c8_0000);

        c.cycles += 3;
        c.g[V0][0] = sx(roll);
        let threshold = chance * hundred;
        let rolled = if (c.g[V0][0] as i64) < 0 {
            // Unsigned to float (never taken: the remainder is below 100).
            c.cycles += 6;
            let v = c.g[V0][0];
            c.g[V1][0] = sx((v as u32) >> 1);
            c.g[V0][0] = (v & 1) | c.g[V1][0];
            let half = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
            half + half
        } else {
            c.cycles += 4;
            FpuF32::from_i32(roll as i32)
        };
        abi::set_fpr(c, 1, threshold);
        abi::set_fpr(c, 0, rolled);

        c.cycles += 4;
        let skip = threshold.le(rolled);
        abi::set_fcc(c, skip);
        c.g[V0][0] = 1;
        if !skip {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_FadeVolume (0x2acb80)

/// Bank-monitor instance: the voice kind (+0: 1 sound, 2 MIDI, 3 stream
/// buffer), the sound / MIDI handle (+4), the stream buffer (+0xc), the
/// "stopping" flag (+0x54) and two "volume touched" flags (+0x58, +0x9c).
pub(crate) const FADE_VOLUME_HANDLE: u32 = 0x4;
pub(crate) const FADE_VOLUME_BUFFER: u32 = 0xc;
pub(crate) const FADE_VOLUME_STOPPING: u32 = 0x54;
pub(crate) const FADE_VOLUME_TOUCHED: u32 = 0x58;
pub(crate) const FADE_VOLUME_TOUCHED_2: u32 = 0x9c;

/// `void BankMonitorInstance_FadeVolume(Instance* self, int volume, int
/// touch, float seconds)` (0x2acb80): unless the instance is already
/// stopping, fades its voice to `volume` (clamped to 0..127) over
/// `seconds` (in centiseconds to the driver; 0 = set at once), under the
/// sound lock for sound and MIDI voices. Volume -1 means stop: the
/// instance is marked stopping and a stream buffer is stopped. `touch`
/// sets the two volume-touched flags first.
pub fn bank_monitor_instance_fade_volume(c: &mut Cpu) {
    const STOP: Callee = Callee {
        addr: 0x002a_a408,
        f: ee::s08::StreamBuffer_Stop,
    };
    const SET_BASE_VOLUME: Callee = Callee {
        addr: 0x002a_9f30,
        f: ee::s08::StreamBuffer_SetBaseVolume,
    };
    let frame = Frame { size: 0x40 };
    guest(c, |c| {
        c.cycles += 9;
        Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // volume
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12]; // seconds
        if c.g[A2][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let me = Addr(r32(c, S1));
            me.off(FADE_VOLUME_TOUCHED_2).sw(c, 1);
            me.off(FADE_VOLUME_TOUCHED).sw(c, 1);
        }

        c.cycles += 3;
        let me = Addr(r32(c, S1));
        c.g[V0][0] = me.off(FADE_VOLUME_STOPPING).lw(c);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            // Volume: -1 = stop, else clamped to 0..127.
            c.cycles += 3;
            c.g[A0][0] = u64::MAX;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
            if c.g[S0][0] == u64::MAX {
                c.cycles += 3;
                c.g[V0][0] = 1;
                me.off(FADE_VOLUME_STOPPING).sw(c, 1);
            } else {
                c.cycles += 4;
                c.g[V1][0] = 0x7f;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 0x7f;
                }
                c.g[A0][0] = (-1i64 < c.g[S0][0] as i64) as u64;
                if c.g[A0][0] == 0 {
                    c.g[S0][0] = 0;
                }
            }

            c.cycles += 4;
            let kind = me.lw(c);
            c.g[V1][0] = kind;
            c.g[V0][0] = ((kind as i64) < 3) as u64;
            if kind == 2 {
                // MIDI: SNDMIDI_SetVolume / SNDMIDI_FadeVolume.
                fade_volume_driver(c, 0x002a_cc7c, (0x003b_9e48, 0x002a_ccc0, 2), (0x003b_9f38, 0x002a_ccb0))?;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                let below_3 = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if !below_3 {
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    if kind == 3 {
                        // Stream buffer: stop, or set its base volume.
                        c.g[V0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_STOPPING).lw(c);
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_BUFFER).lw(c);
                            c.cycles += 3;
                            c.g[A1][0] = c.g[S0][0];
                            SET_BASE_VOLUME.call(c, 0x002a_ccf0)?;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_BUFFER).lw(c);
                            STOP.call(c, 0x002a_cce0)?;
                        }
                    } else {
                        c.cycles += 2;
                    }
                } else {
                    c.cycles += 2;
                    if kind == 1 {
                        // Sound: SND_SetVolume / SND_FadeVolume.
                        fade_volume_driver(c, 0x002a_cc28, (0x003b_85f0, 0x002a_cc6c, 4), (0x003b_8530, 0x002a_cc5c))?;
                    } else {
                        c.cycles += 2;
                    }
                }
                frame.restore(c, S0, 0x20);
            }
        }

        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The sound / MIDI arm of [`bank_monitor_instance_fade_volume`]: under
/// `SND_Lock`, `set(handle, volume)` when `seconds` (f20) is 0, else
/// `fade(handle, (int)(seconds * 100), volume)`; then `SND_Unlock`. `set`
/// is `(address, return address, cycles before the call)`.
fn fade_volume_driver(
    c: &mut Cpu,
    lock_ret: u32,
    set: (u32, u32, u64),
    fade: (u32, u32),
) -> crate::call::Flow {
    c.cycles += 7;
    call_ptr(c, 0x003b_58a0, lock_ret)?; // SND_Lock
    c.f[0] = 0;
    let seconds = FpuF32(c.f[20]);
    let at_once = seconds.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, at_once);
    if at_once {
        c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_HANDLE).lw(c);
        c.cycles += set.2;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, set.0, set.1)?;
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x42c8_0000;
        c.g[A2][0] = c.g[S0][0];
        let centis = seconds * FpuF32(0x42c8_0000);
        abi::set_fpr(c, 0, centis);
        c.f[1] = centis.to_i32() as u32;
        c.g[A1][0] = sx(c.f[1]);
        c.g[A0][0] = Addr(r32(c, S1)).off(FADE_VOLUME_HANDLE).lw(c);
        call_ptr(c, fade.0, fade.1)?;
    }
    c.cycles += 4;
    call_ptr(c, 0x003b_58d8, 0x002a_ccc8) // SND_Unlock
}

// ---------------------------------------------------------------------------
// WScriptFn_ParticleControl (0x302490)

/// gp+: the static-defaults guard; the argument type table; the handle
/// tables; the world.
pub(crate) const PARTICLE_CONTROL_INIT_GUARD: u32 = 0x2644;
pub(crate) const PARTICLE_CONTROL_ARG_TYPES: u32 = 0xbd8;
pub(crate) const PARTICLE_CONTROL_HANDLE_TABLES: u32 = 0x16c8;
pub(crate) const PARTICLE_CONTROL_WORLD: u32 = 0xce8;
/// The static default argument buffer (3 words: handle, mode, tag byte).
pub(crate) const PARTICLE_CONTROL_DEFAULTS: u32 = 0x004f_bc30;
/// The static "no value" result (0x4d0000 - 0x6f68).
pub(crate) const PARTICLE_CONTROL_RESULT_NONE: u32 = 0x004c_9098;

/// `WScriptResult WScriptFn_ParticleControl(int argc, WScriptArg* args)`
/// (0x302490, hidden result pointer in a0): the world-script command that
/// resets (mode 0) or prunes (mode 1) the particles of an object's
/// modifier block. The arguments (16 bytes each: slot index, value, -,
/// type) fill a 3-word buffer initialised from a static default (`{-1, -1,
/// tag}`, set up on first use); an int given for a float slot is
/// converted. `buf[0]` is the object handle (-1: the world's default
/// object); the object's model (`+0xc`) must answer its "has particles"
/// virtual (slot 0x80) before its modifier block (`+0x1c`) is touched.
/// Returns the static "no value" result.
pub fn wscript_fn_particle_control(c: &mut Cpu) {
    const DEFAULT_OBJECT: u32 = 0x290;
    const MODEL: u32 = 0xc;
    const MODIFIERS: u32 = 0x1c;
    const FLOAT_ARG: u64 = 2;
    const LOOP_HEAD: u32 = 0x0030_2520;
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp.wrapping_add(PARTICLE_CONTROL_INIT_GUARD)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0]; // args
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A1][0]; // argc
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0]; // result
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // Guarded static init of the default argument buffer.
            c.cycles += 11;
            c.g[V1][0] = sx(PARTICLE_CONTROL_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = Addr(PARTICLE_CONTROL_DEFAULTS + 8).lbu(c);
            c.g[A1][0] = tag;
            Addr(PARTICLE_CONTROL_DEFAULTS).sw(c, u64::MAX);
            c.g[A0][0] = u64::MAX;
            Addr(PARTICLE_CONTROL_DEFAULTS + 8).sb(c, tag);
            c.g[V0][0] = 1;
            Addr(PARTICLE_CONTROL_DEFAULTS + 4).sw(c, u64::MAX);
            Addr(gp.wrapping_add(PARTICLE_CONTROL_INIT_GUARD)).sw(c, 1);
            Addr(sp(c)).sb(c, tag);
        }

        // buf = defaults (buf[0] = -1 first).
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S1][0] = sx(buf);
        c.g[A1][0] = sx(PARTICLE_CONTROL_DEFAULTS);
        c.g[A2][0] = 0xc;
        Addr(buf).sw(c, u64::MAX);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6574, 0x0030_250c)?; // memcpy
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(gp.wrapping_add(PARTICLE_CONTROL_ARG_TYPES));
            c.g[A3][0] = FLOAT_ARG;
            loop {
                c.cycles += 8;
                let arg = Addr(c.g[A2][0] as u32);
                let slot = arg.lw(c);
                c.g[V1][0] = arg.off(0xc).lw(c);
                let off = (slot as u32) << 2;
                c.g[A0][0] = sx(off.wrapping_add(c.g[S1][0] as u32));
                let ty_addr = off.wrapping_add(c.g[T0][0] as u32);
                c.g[V0][0] = sx(ty_addr);
                c.g[V0][0] = Addr(ty_addr).lw(c);
                let dst = c.g[A0][0] as u32;
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    let v = FpuF32::from_i32(arg.off(4).lw(c) as i32);
                    abi::set_fpr(c, 0, v);
                    Addr(dst).write_fpu(c, v);
                } else {
                    let v = arg.off(4).lw(c);
                    c.g[V0][0] = v;
                    c.cycles += 1;
                    Addr(dst).sw(c, v);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }

        // Resolve the object handle.
        c.cycles += 4;
        let buf = Addr(c.g[S1][0] as u32);
        let id = buf.lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        c.g[A1][0] = Addr(gp.wrapping_add(PARTICLE_CONTROL_HANDLE_TABLES)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let world = Addr(gp.wrapping_add(PARTICLE_CONTROL_WORLD)).lw(c);
            c.g[V0][0] = world;
            c.g[S0][0] = Addr(world as u32).off(DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let tables = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = tables;
            let sets = Addr(tables as u32).off(8).lw(c);
            c.g[A0][0] = sets;
            let set_addr = ((id as u32 & 0xff) << 2).wrapping_add(sets as u32);
            c.g[V0][0] = sx(set_addr);
            let set = Addr(set_addr).lw(c);
            c.g[V1][0] = set;
            c.g[V0][0] = sx((id as u32) >> 8);
            let mut found = false;
            if set != 0 {
                c.cycles += 7;
                let entries = Addr(set as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry_addr = ((c.g[V0][0] as u32) << 2).wrapping_add(entries as u32);
                c.g[V0][0] = sx(entry_addr);
                let entry = Addr(entry_addr).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                c.g[V0][0] = sx(index << 2);
                found = index != 0;
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[V0][0];
        }

        // The object's model, its "has particles" virtual, the mode.
        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0)).off(MODEL).lw(c);
            c.g[S1][0] = Addr(r32(c, S1)).off(4).lw(c); // mode
            if c.g[A1][0] != 0 {
                c.cycles += 7;
                let model = r32(c, A1);
                let vt = Addr(model).off(0xc).lw(c);
                c.g[V1][0] = vt;
                let delta = Addr(vt as u32).off(0x80).lh(c);
                let target = Addr(vt as u32).off(0x84).lw(c);
                c.g[V0][0] = target;
                c.g[A0][0] = sx(model.wrapping_add(delta as u32));
                call_ptr(c, target as u32, 0x0030_25f0)?;
                let has_particles = c.g[V0][0] != 0;
                c.g[V1][0] = 0x004d_0000;
                if has_particles {
                    c.cycles += 2;
                    let mode = c.g[S1][0];
                    c.g[V1][0] = Addr(r32(c, S0)).off(MODEL).lw(c);
                    let call = if mode == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r32(c, V1)).off(MODIFIERS).lw(c);
                        Some((0x0035_3228, 0x0030_2628, 4)) // ModifierBlock_ResetParticles
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        if mode == 1 {
                            c.g[A0][0] = Addr(r32(c, V1)).off(MODIFIERS).lw(c);
                            c.cycles += 2;
                            Some((0x0035_3278, 0x0030_2640, 3)) // ModifierBlock_RemoveDeadParticles
                        } else {
                            c.cycles += 2;
                            None
                        }
                    };
                    c.g[V1][0] = 0x004d_0000;
                    if let Some((f, ret_to, cost)) = call {
                        if c.g[A0][0] != 0 {
                            c.cycles += cost;
                            call_ptr(c, f, ret_to)?;
                            c.g[V1][0] = 0x004d_0000;
                        }
                    }
                }
            }
        }

        // *result = the static "no value" result (12 bytes, unaligned copy).
        c.cycles += 15;
        let result = c.g[S3][0] as u32;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = c.g[A3][0] as u32;
        let head = Addr(src).read_u64_unaligned(c);
        c.g[A0][0] = head;
        let tail = Addr(src).off(8).lw(c);
        c.g[A1][0] = tail;
        Addr(result).write_u64_unaligned(c, head);
        Addr(result).off(8).sw(c, tail);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModObj_LinkParticleModifier (0x355658)

/// ModObj: its modifier block (+0x1c); the block's particle-modifier list
/// (+0x10).
pub(crate) const LINK_PARTICLE_MODIFIER_BLOCK: u32 = 0x1c;
pub(crate) const LINK_PARTICLE_MODIFIER_LIST: u32 = 0x10;

/// `void ModObj_LinkParticleModifier(ModObj* self, Modifier* item)`
/// (0x355658, `cMoveNode::addEffectModifier`): creates the modifier block
/// when absent, then pushes `item` onto its particle-modifier list.
pub fn mod_obj_link_particle_modifier(c: &mut Cpu) {
    const CREATE_MODIFIER_BLOCK: Callee = Callee {
        addr: 0x0035_5468,
        f: ee::s11::ModObj_CreateModifierBlock,
    };
    const LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x0034_5700,
        f: ee::s11::ModifierList_PushFront,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(LINK_PARTICLE_MODIFIER_BLOCK).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            CREATE_MODIFIER_BLOCK.call(c, 0x0035_5680)?;
        }
        c.cycles += 9;
        let block = Addr(r32(c, S0)).off(LINK_PARTICLE_MODIFIER_BLOCK).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx((block as u32).wrapping_add(LINK_PARTICLE_MODIFIER_LIST));
        LIST_PUSH_FRONT.call(c, 0x0035_5690)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_90f0, rider_metrix_count_trick_repeats, "RiderMetrix_CountTrickRepeats"),
    (0x0034_d9b0, anim_object_base_ctor, "AnimObject_BaseCtor"),
    (0x003d_fe18, stream_file_free_request, "StreamFile_FreeRequest"),
    (0x0012_3168, rider_get_player_character_info, "Rider_GetPlayerCharacterInfo"),
    (0x003b_6a70, sndstrm_retire_request, "SNDSTRM_RetireRequest"),
    (0x0010_c1d0, computer_actor_rider_roll_skip_tricks, "ComputerActorRider_RollSkipTricks"),
    (0x002a_cb80, bank_monitor_instance_fade_volume, "BankMonitorInstance_FadeVolume"),
    (0x0030_2490, wscript_fn_particle_control, "WScriptFn_ParticleControl"),
    (0x0035_5658, mod_obj_link_particle_modifier, "ModObj_LinkParticleModifier"),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// RiderMetrix_CountTrickRepeats: busy states, the two never-counted
    /// trick classes, and histories with exact copies, near misses (one byte
    /// off) and random entries; events delivered inside the byte-compare
    /// loops.
    #[test]
    fn rider_metrix_count_trick_repeats_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::{scramble, Rng, DATA};
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0011_90f0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let me = DATA;
            let id = DATA + 0x200 + 4 * (seed % 2);
            let mut w0 = r.next_u32();
            let mut w1 = r.next_u32();
            match r.next_u32() % 8 {
                0 => w0 = (w0 & !0x0fc0_0000) | 0x0040_0000,
                1 => w1 = (w1 & !0x0003_f800) | 0x800,
                _ => {
                    // Keep both classes countable.
                    if w0 & 0x0fc0_0000 == 0x0040_0000 {
                        w0 ^= 0x0100_0000;
                    }
                    if w1 & 0x0003_f800 == 0x800 {
                        w1 ^= 0x1000;
                    }
                }
            }
            c.bus.write32(id, w0);
            c.bus.write32(id + 4, w1);
            let trick = (w1 as u64) << 32 | w0 as u64;
            for off in COUNT_TRICK_REPEATS_BUSY {
                c.bus.write32(me + off, 0);
            }
            if r.next_u32() % 6 == 0 {
                let off = COUNT_TRICK_REPEATS_BUSY[(r.next_u32() % 5) as usize];
                c.bus.write32(me + off, r.next_u32() | 1);
            }
            for i in 0..10u32 {
                let e = me + COUNT_TRICK_REPEATS_HIST + 8 * i;
                let v = match r.next_u32() % 3 {
                    0 => trick,
                    1 => trick ^ (1u64 << (8 * (r.next_u32() % 8) + r.next_u32() % 8)),
                    _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                };
                c.bus.write64(e, v);
            }
            c.bus.write32(me + COUNT_TRICK_REPEATS_HIST_IDX, r.next_u32() % 10);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = sx(id);
            schedule(c, &mut r, seed, 0x0011_90f0)
        };
        check_events(
            ee::s00::f_001190f0__generated,
            rider_metrix_count_trick_repeats,
            150,
            &setup,
        );
    }

    /// AnimObject_BaseCtor: the ModObj_ctor fixture (process lists, world ->
    /// focus manager, node flags with or without bit 0 / 0x100) plus 0..4
    /// node definitions with or without mesh sources (so the mesh duration
    /// branch runs or not), durations from a clip or none, `force` on or
    /// off; every callee (operator_new_named on the test pools / heap,
    /// AllocNodeInfo, GetDuration, v52 -> IntersectInfo_EvaluateMatrix) runs
    /// for real on the ELF image.
    #[test]
    fn anim_object_base_ctor_matches() {
        use crate::difftest::{check, elf_image, fill, DATA, GAME_GP};
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const DEF: u32 = DATA + 0x400;
        const ENTRIES: u32 = DATA + 0x500;
        const SOURCES: u32 = DATA + 0x600;
        const CURVES: u32 = DATA + 0x800;
        const ACTION: u32 = DATA + 0x840;
        const CLIP: u32 = DATA + 0x880;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(
            ee::s11::f_0034d9b0__generated,
            anim_object_base_ctor,
            120,
            &|c, seed| {
                elf_image(c);
                let mut r = alloc_machine(c, seed, 0x34_d9b0);
                c.pc = 0x0034_d9b0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                for i in 0..0x40 {
                    w(c, SELF + 4 * i, 0);
                }
                fill(c, &mut r, NODE, 0x20);
                let flags = r.next_u32();
                w(
                    c,
                    NODE + 0x8,
                    if seed % 4 == 0 {
                        flags | 0x100
                    } else {
                        flags & !0x100
                    },
                );
                w(c, NODE + 0x80, DEF);
                let defs = seed % 5;
                w(c, DEF + 4, defs);
                w(c, DEF + 8, ENTRIES);
                for i in 0..defs {
                    let src = if seed % 7 == 1 || r.next_u32() % 3 == 0 {
                        0
                    } else {
                        SOURCES + 0x40 * i
                    };
                    w(c, ENTRIES + 0x10 * i + 8, src);
                    for k in 0..6 {
                        w(c, SOURCES + 0x40 * i + 4 * k, r.f32(100.0));
                    }
                    w(c, SOURCES + 0x40 * i + 0x18, 0); // no animated channel
                    w(c, SOURCES + 0x40 * i + 0x20, CURVES);
                }
                w(c, CURVES, if seed % 3 == 0 { 0 } else { ACTION });
                w(c, ACTION + 4, if seed % 5 == 2 { 0 } else { CLIP });
                w(c, CLIP + 0x10, r.f32(30.0));
                // The ModObj_ctor fixture: process lists, world -> focus.
                let list = seed % 4;
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                for i in 0..4 {
                    w(c, PROCS + 0x44 * i, r.next_u32());
                    w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                    w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
                }
                let kind = match (seed / 4) % 6 {
                    0 => 6,
                    1 => 0x10,
                    2 => 0x16,
                    _ => r.next_u32() % 0x20,
                };
                fill(c, &mut r, 0x0044_aff0, 0x20);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, WORLD + 0x100);
                w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
                w(c, WORLD + 0x200 + 0xa4, FOCUS);
                w(
                    c,
                    FOCUS + 0xd4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                w(c, FOCUS + 0xd8, r.next_u32() % 8);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(kind);
                c.g[A3][0] = sx(NODE);
                c.g[T0][0] = if seed % 3 == 1 { sx(r.next_u32() | 1) } else { 0 };
            },
        );
    }

    /// StreamFile_FreeRequest: the request at the head, the tail, the middle
    /// or alone in the list, current or not, with and without a next
    /// neighbour (current falls back to the previous one).
    #[test]
    fn stream_file_free_request_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const FILE: u32 = DATA;
        const REQS: u32 = DATA + 0x100;
        check(
            ee::s14::f_003dfe18__generated,
            stream_file_free_request,
            200,
            &|c, seed| {
                let mut r = Rng(0x003d_fe18 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_fe18;
                // A list of 1..4 requests (0x20 apart), doubly linked.
                let n = 1 + seed % 4;
                let at = |i: u32| REQS + 0x20 * i;
                for i in 0..n {
                    c.bus.write32(at(i) + 4, r.next_u32());
                    c.bus.write32(at(i) + 8, if i == 0 { 0 } else { at(i - 1) });
                    c.bus.write32(at(i) + 0xc, if i + 1 == n { 0 } else { at(i + 1) });
                }
                c.bus.write32(FILE + 0x5c, at(0));
                c.bus.write32(FILE + 0x64, at(n - 1));
                let pick = (seed / 4) % n;
                let current = match r.next_u32() % 3 {
                    0 => at(pick),
                    1 => at(r.next_u32() % n),
                    _ => 0,
                };
                c.bus.write32(FILE + 0x60, current);
                c.bus.write32(FILE + 0x68, if seed % 2 == 0 { 0 } else { DATA + 0x400 });
                c.g[A0][0] = sx(FILE);
                c.g[A1][0] = sx(at(pick));
            },
        );
    }

    /// Rider_GetPlayerCharacterInfo: a built BELibrary whose interface
    /// method is a fake (unwinding for some seeds); the real GetSlot /
    /// GetAltCharacter read the slot table and the player records.
    #[test]
    fn rider_get_player_character_info_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const RIDER: u32 = DATA + 0x1000;
        const LIB: u32 = DATA + 0x8000;
        const IFACE: u32 = DATA + 0x8100;
        const IFACE_VT: u32 = DATA + 0x8200;
        covered(
            ee::s00::f_00123168__generated,
            rider_get_player_character_info,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_3168, 1);
                c.pc = 0x0012_3168;
                c.bus.write32(GAME_GP - 0x1f04, LIB);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, IFACE);
                c.bus.write32(IFACE + 4, IFACE_VT);
                c.bus.write16(IFACE_VT + 8, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(IFACE_VT + 0xc, FAKE);
                // Player slots (0x5305b0) and records (0x534fe0, 0x1c each).
                for i in 0..8 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                fill(c, &mut r, 0x0053_4fe0, 0x1c * 8 / 4);
                c.bus.write32(RIDER + RIDER_GET_PLAYER_CHARACTER_INFO_AI_INDEX, seed % 8);
                c.g[A0][0] = sx(RIDER);
            },
        );
    }

    /// SNDSTRM_RetireRequest: four streams with a three-request active list
    /// and a one-entry free list; the handle names the first, middle or
    /// last request, which is or is not the playing one (the real
    /// FindRequest / ListRemove / ListPushFront run).
    #[test]
    fn sndstrm_retire_request_matches() {
        use crate::difftest::{check, scramble, Rng};
        const B: u32 = 0x00b0_0000;
        const STREAM_COUNT: u32 = 0x0050_a92e;
        check(
            ee::s13::f_003b6a70__generated,
            sndstrm_retire_request,
            200,
            &|c, seed| {
                let mut r = Rng(0x003b_6a70 ^ (seed as u64) << 5);
                scramble(c, &mut r);
                c.pc = 0x003b_6a70;
                c.bus.write8(STREAM_COUNT, 4);
                for i in 0..4u32 {
                    let stream = B + 0x1000 * i;
                    c.bus.write32(SNDSTRM_RETIRE_REQUEST_STREAMS + 4 * i, stream);
                    // Active list {head, tail, count}: three requests
                    // {next, prev, ?, handle}.
                    let req = |k: u32| stream + 0x200 + 0x40 * k;
                    for k in 0..3 {
                        c.bus.write32(req(k), if k == 2 { 0 } else { req(k + 1) });
                        c.bus.write32(req(k) + 4, if k == 0 { 0 } else { req(k - 1) });
                        c.bus.write32(req(k) + 8, r.next_u32());
                        c.bus.write32(req(k) + 0xc, i | (k + 1) << 8);
                    }
                    let active = stream + SNDSTRM_RETIRE_REQUEST_ACTIVE;
                    c.bus.write32(active, req(0));
                    c.bus.write32(active + 4, req(2));
                    c.bus.write32(active + 8, 3);
                    // Free list: one spare request, or empty.
                    let free = stream + SNDSTRM_RETIRE_REQUEST_FREE;
                    let spare = stream + 0x400;
                    let has_spare = r.next_u32() % 2 == 0;
                    c.bus.write32(spare, 0);
                    c.bus.write32(spare + 4, 0);
                    c.bus.write32(free, if has_spare { spare } else { 0 });
                    c.bus.write32(free + 4, if has_spare { spare } else { 0 });
                    c.bus.write32(free + 8, has_spare as u32);
                    let playing = match r.next_u32() % 3 {
                        0 => 0,
                        _ => req(r.next_u32() % 3),
                    };
                    c.bus.write32(stream + SNDSTRM_RETIRE_REQUEST_PLAYING, playing);
                }
                let stream = r.next_u32() % 4;
                let k = 1 + seed % 3;
                c.g[A0][0] = sx(stream | k << 8);
            },
        );
    }

    /// ComputerActorRider_RollSkipTricks: the real game random number
    /// generator from a random state, trick chances from edge-case floats,
    /// fractions of 1 and exact multiples of 1/100 (ties with the roll).
    #[test]
    fn computer_actor_rider_roll_skip_tricks_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const RAND_STATE: u32 = 0x004f_f030;
        check(
            ee::s00::f_0010c1d0__generated,
            computer_actor_rider_roll_skip_tricks,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_c1d0, 0);
                c.pc = 0x0010_c1d0;
                fill(c, &mut r, RAND_STATE, 0x10);
                let chance = match seed % 4 {
                    0 => r.edge_f32(),
                    1 => ((r.next_u32() % 101) as f32 / 100.0).to_bits(),
                    _ => (r.next_u32() as f32 / u32::MAX as f32).to_bits(),
                };
                c.bus.write32(DATA + ROLL_SKIP_TRICKS_CHANCE, chance);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// BankMonitorInstance_FadeVolume: kinds 0..4 (sound, MIDI, stream
    /// buffer, none), already stopping or not, volume -1 / out of range /
    /// in range, `touch` on or off, seconds 0, -0 or not; sound and MIDI
    /// handles that the real drivers reject (short paths), the lock hooks
    /// are fakes (unwinding for some seeds).
    #[test]
    fn bank_monitor_instance_fade_volume_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::DATA;
        const INST: u32 = DATA;
        const BUFFER: u32 = DATA + 0x200;
        covered(
            ee::s08::f_002acb80__generated,
            bank_monitor_instance_fade_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_cb80, 4);
                c.pc = 0x002a_cb80;
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write32(INST, seed % 5);
                c.bus.write32(INST + FADE_VOLUME_HANDLE, r.next_u32());
                c.bus.write32(INST + FADE_VOLUME_BUFFER, BUFFER);
                c.bus.write32(BUFFER + 8, r.next_u32() | 0x8000_0000);
                c.bus.write32(BUFFER + 0xc, r.next_u32());
                c.bus.write32(
                    INST + FADE_VOLUME_STOPPING,
                    if r.next_u32() % 4 == 0 { 1 } else { 0 },
                );
                c.bus.write32(INST + FADE_VOLUME_TOUCHED, r.next_u32());
                c.bus.write32(INST + FADE_VOLUME_TOUCHED_2, r.next_u32());
                let volume: u64 = match (seed / 5) % 6 {
                    0 => u64::MAX,
                    1 => 128 + (r.next_u32() % 1000) as u64,
                    2 => (-((1 + r.next_u32() % 1000) as i64)) as u64,
                    3 => 127,
                    _ => (r.next_u32() % 128) as u64,
                };
                c.g[A1][0] = volume;
                c.g[A2][0] = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() as u64 | 1 };
                c.f[12] = match r.next_u32() % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.edge_f32(),
                    _ => r.f32(5.0),
                };
                c.g[A0][0] = sx(INST);
            },
        );
    }

    /// WScriptFn_ParticleControl: the static defaults initialised or not;
    /// 0..4 extra arguments over the handle and mode arguments (or none at
    /// all), int arguments for a float slot converted; the handle -1 (the
    /// world's default object, present or not), an empty handle set, an
    /// empty entry, or a found object; no model, a model whose "has
    /// particles" virtual says no (a real leaf returning 0) or yes (a fake);
    /// modes 0 / 1 / other with and without a modifier block (the real
    /// reset / prune see an empty particle list).
    #[test]
    fn wscript_fn_particle_control_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        check(
            ee::s10::f_00302490__generated,
            wscript_fn_particle_control,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_2490, 0);
                let gp = GAME_GP;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults: fresh, or already set up (handle -1).
                w(c, gp + PARTICLE_CONTROL_INIT_GUARD, seed % 2);
                fill(c, &mut r, PARTICLE_CONTROL_DEFAULTS, 3);
                w(c, PARTICLE_CONTROL_DEFAULTS, u32::MAX);
                w(c, PARTICLE_CONTROL_DEFAULTS + 4, r.next_u32() % 3);
                let types = gp + PARTICLE_CONTROL_ARG_TYPES;
                w(c, types, 0);
                w(c, types + 4, 0);
                w(c, types + 8, r.next_u32() % 3);
                fill(c, &mut r, PARTICLE_CONTROL_RESULT_NONE, 3);
                // Handle tables: gp+0x16c8 -> P -> Q; Q+8 -> sets.
                let (p, q, sets) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
                let (set, entries, obj) = (DATA + 0x2800, DATA + 0x2900, DATA + 0x3000);
                let (world, model, vt, mods) =
                    (DATA + 0x3800, DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                w(c, gp + PARTICLE_CONTROL_HANDLE_TABLES, p);
                w(c, p, q);
                w(c, q + 8, sets);
                w(c, sets + 4 * 3, set);
                w(c, sets + 4 * 7, 0);
                w(c, set + 0x1c, entries);
                let found = (obj >> 2) << 8 | (r.next_u32() & 0xff);
                w(
                    c,
                    entries + 4 * 5,
                    if seed / 2 % 4 == 2 {
                        r.next_u32() & 0xff
                    } else {
                        found
                    },
                );
                w(c, gp + PARTICLE_CONTROL_WORLD, world);
                w(c, world + 0x290, if seed % 9 == 4 { 0 } else { obj });
                // The model and its "has particles" virtual (slot 0x80).
                w(c, obj + 0xc, if seed % 7 == 3 { 0 } else { model });
                w(c, model + 0xc, vt);
                c.bus
                    .write16(vt + 0x80, (r.next_u32() as i16 as i32 & !3) as u16);
                w(c, vt + 0x84, if seed % 5 == 1 { 0x0014_06c0 } else { FAKE });
                w(c, model + 0x1c, if seed % 6 == 5 { 0 } else { mods });
                w(c, mods + 0x10, 0); // no particles
                let id = match seed / 2 % 4 {
                    0 => u32::MAX,
                    1 => 5 << 8 | 7,
                    _ => 5 << 8 | 3,
                };
                let mode = match seed / 8 % 4 {
                    3 => r.next_u32() % 16,
                    m => m % 2,
                };
                // Arguments (16 bytes: slot, value, -, type).
                let args = DATA + 0x100;
                let extra = seed / 3 % 5;
                let mut n = 0;
                let mut put = |c: &mut Cpu, slot: u32, val: u32, ty: u32| {
                    let a = args + 0x10 * n;
                    c.bus.write32(a, slot);
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, 0x5555);
                    c.bus.write32(a + 0xc, ty);
                    n += 1;
                };
                put(c, 0, id, 0);
                put(c, 1, mode, 0);
                for _ in 0..extra {
                    let val = r.next_u32();
                    let ty = r.next_u32() % 3;
                    put(c, 2, val, ty);
                }
                let argc = match seed % 11 {
                    0 => 0,
                    5 => sx(0xffff_fffe),
                    _ => n as u64,
                };
                c.g[A0][0] = sx(DATA + 0x10);
                c.g[A1][0] = argc;
                c.g[A2][0] = sx(args);
                c.pc = 0x0030_2490;
            },
        );
    }

    /// ModObj_LinkParticleModifier: the modifier block present or absent
    /// (the real ModObj_CreateModifierBlock allocates it from the test
    /// pools / heap), the particle list empty or holding one modifier.
    #[test]
    fn mod_obj_link_particle_modifier_matches() {
        use crate::difftest::{check, DATA};
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const THIS: u32 = DATA + 0x100;
        const BLOCK: u32 = DATA + 0x800;
        const ITEM: u32 = DATA + 0x900;
        const OTHER: u32 = DATA + 0xa00;
        check(
            ee::s11::f_00355658__generated,
            mod_obj_link_particle_modifier,
            120,
            &|c, seed| {
                let _ = alloc_machine(c, seed, 0x35_5658);
                c.pc = 0x0035_5658;
                for a in [THIS, BLOCK, ITEM, OTHER] {
                    for i in 0..0x18 {
                        c.bus.write32(a + 4 * i, 0);
                    }
                }
                let has_block = seed % 2 == 1;
                c.bus.write32(
                    THIS + LINK_PARTICLE_MODIFIER_BLOCK,
                    if has_block { BLOCK } else { 0 },
                );
                if has_block && seed % 4 == 3 {
                    // One modifier already listed.
                    c.bus.write32(BLOCK + LINK_PARTICLE_MODIFIER_LIST, OTHER);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(ITEM);
            },
        );
    }
}
