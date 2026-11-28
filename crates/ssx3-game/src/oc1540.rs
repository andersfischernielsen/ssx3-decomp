//! Staging module oc1540 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1540`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001c_ed90, cfe_state_trophy_room_query_event_reward, "cFEStateTrophyRoom_QueryEventReward"),
    (0x001d_b718, std_list_clear, "StdList_Clear"),
    (0x001d_b778, std_vector_insert_aux, "StdVector_InsertAux"),
    (0x001d_b918, std_vector_insert_aux_2, "StdVector_InsertAux_2"),
    (0x001d_d6a8, cfe_state_ctor_2, "cFEState_ctor_2"),
    (0x001d_d6e0, cfe_state_ctor_3, "cFEState_ctor_3"),
    (0x001d_d718, cfe_state_ctor_4, "cFEState_ctor_4"),
    (0x001d_e240, cfe_state_profile_load_ctor_2, "cFEStateProfileLoad_ctor_2"),
];

/// The object's `this` (`s1`) as an address.
#[inline(always)]
fn this_of(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// `StdList_Clear(list)` (0x1db718): deletes every node after the sentinel
/// head (`list + 4`), each through the node deleter (0x317e50) with its
/// `next` read first, then points the sentinel at itself (`next` and
/// `prev`). A back-edge event check at the loop head 0x1db740.
pub fn std_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] != c.g[V0][0] {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, 0x0031_7e50, 0x001d_b748)?;
                c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_b740);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
                frame.restore(c, S0, 0x20);
                break;
            }
        } else {
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 7;
        let head = c.g[V0][0];
        Addr(head as u32).sw(c, head);
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
        frame.restore(c, S1, 0x10);
        let v1 = c.g[V1][0];
        Addr((v1 as u32).wrapping_add(4)).sw(c, v1);
        frame.close(c);
        Ok(())
    });
}

/// Return addresses of the calls in `StdVector_InsertAux` (the generated
/// body's `c.g[RA][0] = sx(0x...)` lines), per function copy.
#[derive(Clone, Copy)]
struct InsertRets {
    /// `memmove` of the tail, fast path (0x41610c).
    fast_move: u32,
    /// `EA_MemAllocNamed` for a one-element block (0x317d70).
    alloc_one: u32,
    /// `operator_new_named` for a grown block (0x317e30).
    alloc_many: u32,
    /// `memmove` of the head into the new block (0x41610c).
    copy_head: u32,
    /// `memmove` of the tail behind the new element (0x41610c).
    copy_tail: u32,
    /// Free of the old one-element block (0x317e50).
    free_one: u32,
    /// `EA_MemFree` of the old block (0x317e98).
    free_many: u32,
}

/// `StdVector_InsertAux(vec, pos, x)` (0x1db778): `_M_insert_aux` for a
/// vector of pointers (`mpStart`, `mpFinish`, `mpEndOfStorage` at +4, +8,
/// +0xc). With room behind the finish it shifts the tail up by one; a full
/// vector grows to twice its size (one element when empty) and moves both
/// halves into the new block.
pub fn std_vector_insert_aux(c: &mut Cpu) {
    guest(c, |c| {
        let rets = InsertRets {
            fast_move: 0x001d_b7ec,
            alloc_one: 0x001d_b834,
            alloc_many: 0x001d_b854,
            copy_head: 0x001d_b878,
            copy_tail: 0x001d_b8ac,
            free_one: 0x001d_b8d4,
            free_many: 0x001d_b8e4,
        };
        ptr_vector_insert_aux(c, &rets)
    });
}

/// `StdVector_InsertAux` for the second vector type (0x1db918): the same
/// body at its own addresses.
pub fn std_vector_insert_aux_2(c: &mut Cpu) {
    guest(c, |c| {
        let rets = InsertRets {
            fast_move: 0x001d_b98c,
            alloc_one: 0x001d_b9d4,
            alloc_many: 0x001d_b9f4,
            copy_head: 0x001d_ba18,
            copy_tail: 0x001d_ba4c,
            free_one: 0x001d_ba74,
            free_many: 0x001d_ba84,
        };
        ptr_vector_insert_aux(c, &rets)
    });
}

/// The frame and the full/room split of `_M_insert_aux`.
fn ptr_vector_insert_aux(c: &mut Cpu, rets: &InsertRets) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S1, 0x50);
    frame.save(c, S3, 0x30);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S4, 0x20);
    c.g[S3][0] = c.g[A1][0];
    frame.save(c, S0, 0x60);
    frame.save(c, S2, 0x40);
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
    c.g[V0][0] = Addr(this_of(c).wrapping_add(0xc)).lw(c);
    let full = c.g[V1][0] == c.g[V0][0];
    c.g[S4][0] = c.g[A2][0];
    if full {
        ptr_vector_grow(c, rets)?;
    } else {
        ptr_vector_room(c, rets)?;
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The room path: store the last pointer one slot up, then move the
/// `[pos, finish - 1)` run up by one and write `x` at `pos`.
fn ptr_vector_room(c: &mut Cpu, rets: &InsertRets) -> Flow {
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.g[A0][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(4)).lw(c);
        let last = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sw(c, last);
        c.g[A0][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
    }
    c.cycles += 12;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
    let new_finish = c.g[V0][0];
    Addr(this_of(c).wrapping_add(8)).sw(c, new_finish);
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
    call_ptr(c, 0x0041_610c, rets.fast_move)?;
    let x = c.g[S0][0];
    Addr(c.g[S3][0] as u32).sw(c, x);
    Ok(())
}

/// The full path: allocate the new block (twice the element count, or one
/// element when empty), copy the head, the new element and the tail into
/// it, free the old block and install the three new pointers.
fn ptr_vector_grow(c: &mut Cpu, rets: &InsertRets) -> Flow {
    'grow: {
        'single: {
            c.cycles += 9;
            c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
            c.g[S0][0] = 1;
            c.g[A0][0] = 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            let not_one = c.g[S0][0] != c.g[A0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            if not_one {
                break 'single;
            }
            c.cycles += 9;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3218));
            c.g[A0][0] = 4;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[S5][0] = 4;
            call_ptr(c, 0x0031_7d70, rets.alloc_one)?;
            c.g[S2][0] = c.g[V0][0];
            break 'grow;
        }
        c.cycles += 8;
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3218));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, rets.alloc_many)?;
        c.g[S5][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
    }

    // The head [start, pos) into the new block.
    c.cycles += 9;
    c.g[A1][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    call_ptr(c, 0x0041_610c, rets.copy_head)?;
    c.g[V1][0] = c.g[S0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
        let x = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, x);
    }

    // The tail [pos, finish) behind the new element, then the old block.
    'tail: {
        {
            c.cycles += 14;
            c.g[A2][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, 0x0041_610c, rets.copy_tail)?;
            c.g[A0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
            c.g[V0][0] = Addr(this_of(c).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            let no_old_block = c.g[A0][0] == 0;
            c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            if !no_old_block {
                'release: {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    if c.g[V1][0] != c.g[V0][0] {
                        break 'release;
                    }
                    c.cycles += 4;
                    call_ptr(c, 0x0031_7e50, rets.free_one)?;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    break 'tail;
                }
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, rets.free_many)?;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
    }
    c.cycles += 3;
    let finish = c.g[S3][0];
    Addr(this_of(c).wrapping_add(8)).sw(c, finish);
    let end = c.g[V0][0];
    Addr(this_of(c).wrapping_add(0xc)).sw(c, end);
    let start = c.g[S2][0];
    Addr(this_of(c).wrapping_add(4)).sw(c, start);
    Ok(())
}

/// The `cFEState` base constructor tail shared by the ctors: the vtable at
/// `this + 8`, `v0 = this`, then the frame's epilogue.
fn cfe_state_ctor_tail(c: &mut Cpu, frame: Frame, vtable: u64) {
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    let this = c.g[S0][0] as u32;
    Addr(this.wrapping_add(8)).sw(c, vtable);
    frame.restore(c, S0, 0x10);
    frame.close(c);
}

/// `cFEState::cFEState` variant 2 (0x1dd6a8): the base constructor with
/// `a1 = 0` (0x39e2a0), then the cFEState vtable at `this + 8`.
pub fn cfe_state_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x001d_d6c0)?;
        cfe_state_ctor_tail(c, frame, 0x0046_d0d0);
        Ok(())
    });
}

/// `cFEState::cFEState` variant 3 (0x1dd6e0): the base constructor (0x39e2a0)
/// then the cFEState vtable at `this + 8`.
pub fn cfe_state_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x001d_d6f4)?;
        cfe_state_ctor_tail(c, frame, 0x0046_d0d0);
        Ok(())
    });
}

/// `cFEState::cFEState` variant 4 (0x1dd718): the base constructor (0x39e318)
/// then the cFEState vtable at `this + 8`.
pub fn cfe_state_ctor_4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001d_d72c)?;
        cfe_state_ctor_tail(c, frame, 0x0046_d0d0);
        Ok(())
    });
}

/// `cFEStateProfileLoad::cFEStateProfileLoad` variant 2 (0x1de240): the
/// `cFEMemCard` base constructor (0x1d53b0) with `a1 = a2 = 0`, then the
/// profile-load vtable at `this + 8`.
pub fn cfe_state_profile_load_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x001d_53b0,
            f: ee::s04::f_001d53b0,
        }
        .call(c, 0x001d_e25c)?;
        cfe_state_ctor_tail(c, frame, 0x0046_bfd0);
        Ok(())
    });
}

/// `getChallengeMedal` (0x1ced90, `cFEStateTrophyRoom_QueryEventReward`):
/// asks the reward interface (the BE object's slot 13) for one trophy-room
/// medal. Arguments: a0 profile, a1 character, a2 peak, a3 trophy type,
/// t0 challenge. Trophy type 2 takes the collectibles medal (challenge 0)
/// or the big-challenges medal; 3 the earnings medal; any other type reads
/// a pair of bytes from the medal table (0x45aad8: peak x type x challenge,
/// 2 bytes each) and asks for the track medal.
pub fn cfe_state_trophy_room_query_event_reward(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);

        c.g[S4][0] = c.g[T0][0];
        call_ptr(c, 0x0014_dc80, 0x001c_edc4)?;
        c.g[A1][0] = 0xd;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001c_edd0)?;
        c.g[A1][0] = c.g[V0][0];

        trophy_medal_paths(c)?;

        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The medal call of `getChallengeMedal`, by trophy type.
fn trophy_medal_paths(c: &mut Cpu) -> Flow {
    c.g[V0][0] = 2;
    let type_two = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = 3;
    if type_two {
        c.cycles += 2;
        let big_challenge = c.g[S4][0] != 0;
        c.g[A0][0] = c.g[A1][0];
        if big_challenge {
            c.cycles += 6;
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, 0x0015_8a50, 0x001c_ee10)?;
        } else {
            c.cycles += 6;
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, 0x0015_89b0, 0x001c_edf8)?;
        }
        return Ok(());
    }

    c.cycles += 2;
    let earnings = c.g[S0][0] == c.g[V0][0];
    c.g[A3][0] = 0xa;
    if earnings {
        c.cycles += 6;
        c.g[A0][0] = c.g[A1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S1][0];
        call_ptr(c, 0x0015_8af0, 0x001c_ee7c)?;
        return Ok(());
    }

    c.cycles += 18;
    c.g[T1][0] = 0x28;
    c.mult(0, A3, S0, A3);
    c.mult(0, T1, S1, T1);
    c.g[V0][0] = sx((c.g[S4][0] as u32) << 1);
    c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x5528));
    c.g[A0][0] = c.g[A1][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let entry = c.g[T0][0] as u32;
    c.g[A3][0] = Addr(entry).lb(c);
    let kind_at = (c.g[V1][0] as u32).wrapping_add(1);
    c.g[T0][0] = Addr(kind_at).lb(c);
    call_ptr(c, 0x0015_8910, 0x001c_ee60)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fakes, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's fakes, in the test machine's RAM.
    const GATE: u32 = DATA + 0x0020_1540;
    const GATE_ON: u32 = 0x1540_c0de;
    /// The V0 the fake callees return (read from `RESULT_CELL`).
    const RESULT_CELL: u32 = DATA + 0x0020_1550;
    /// A block the fake allocator hands out (inside the fixture's data area).
    const NEW_BLOCK: u32 = DATA + 0x0008_0000;

    fn gate_on(c: &mut Cpu) {
        c.bus.write32(GATE, GATE_ON);
        c.bus.write32(RESULT_CELL, NEW_BLOCK);
    }

    /// A gated fake of one callee: with the gate on it returns at once with
    /// V0 = `RESULT_CELL` (allocators, getters, and void callees alike);
    /// with the gate off it runs the body the slot held before.
    macro_rules! oc1540_fake {
        ($fake:ident, $prev:ident, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(GATE) != GATE_ON {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                c.g[V0][0] = c.bus.read32(RESULT_CELL) as u64;
                ret(c);
            }
        };
    }

    oc1540_fake!(oc1540_fake_alloc_one, OC1540_PREV_ALLOC_ONE, ee::s10::f_00317d70__generated);
    oc1540_fake!(oc1540_fake_alloc_many, OC1540_PREV_ALLOC_MANY, ee::s10::f_00317e30__generated);
    oc1540_fake!(oc1540_fake_free_one, OC1540_PREV_FREE_ONE, ee::s10::f_00317e50__generated);
    oc1540_fake!(oc1540_fake_free_many, OC1540_PREV_FREE_MANY, ee::s10::f_00317e98__generated);
    oc1540_fake!(oc1540_fake_base_ctor, OC1540_PREV_BASE_CTOR, ee::s13::f_0039e2a0__generated);
    oc1540_fake!(oc1540_fake_base_ctor_4, OC1540_PREV_BASE_CTOR_4, ee::s13::f_0039e318__generated);
    oc1540_fake!(oc1540_fake_memcard, OC1540_PREV_MEMCARD, ee::s04::f_001d53b0__generated);
    oc1540_fake!(oc1540_fake_be, OC1540_PREV_BE, ee::s01::f_0014dc80__generated);
    oc1540_fake!(oc1540_fake_iface, OC1540_PREV_IFACE, ee::s01::f_0014dd58__generated);
    oc1540_fake!(oc1540_fake_collectibles, OC1540_PREV_COLLECTIBLES, ee::s01::f_001589b0__generated);
    oc1540_fake!(oc1540_fake_big, OC1540_PREV_BIG, ee::s01::f_00158a50__generated);
    oc1540_fake!(oc1540_fake_earnings, OC1540_PREV_EARNINGS, ee::s01::f_00158af0__generated);
    oc1540_fake!(oc1540_fake_track, OC1540_PREV_TRACK, ee::s01::f_00158910__generated);

    /// Hooks the fakes above into their slots (once per process).
    fn install_fakes() {
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317D70), oc1540_fake_alloc_one, &OC1540_PREV_ALLOC_ONE);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E30), oc1540_fake_alloc_many, &OC1540_PREV_ALLOC_MANY);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E50), oc1540_fake_free_one, &OC1540_PREV_FREE_ONE);
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E98), oc1540_fake_free_many, &OC1540_PREV_FREE_MANY);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_0039E2A0), oc1540_fake_base_ctor, &OC1540_PREV_BASE_CTOR);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_0039E318), oc1540_fake_base_ctor_4, &OC1540_PREV_BASE_CTOR_4);
        fakes::hook(addr_of_mut!(ee::s04::OVERRIDE_001D53B0), oc1540_fake_memcard, &OC1540_PREV_MEMCARD);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_0014DC80), oc1540_fake_be, &OC1540_PREV_BE);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_0014DD58), oc1540_fake_iface, &OC1540_PREV_IFACE);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_001589B0), oc1540_fake_collectibles, &OC1540_PREV_COLLECTIBLES);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_00158A50), oc1540_fake_big, &OC1540_PREV_BIG);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_00158AF0), oc1540_fake_earnings, &OC1540_PREV_EARNINGS);
        fakes::hook(addr_of_mut!(ee::s01::OVERRIDE_00158910), oc1540_fake_track, &OC1540_PREV_TRACK);
    }

    /// A vector of pointers for `StdVector_InsertAux`, in the four shapes the
    /// function branches on: room behind the finish (seed % 4 == 0), a full
    /// vector of one element, a full vector of three, an empty vector.
    fn vector_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1db778, 0);
        gate_on(c);
        let vec = DATA + 0x1000;
        let data = DATA + 0x2000;
        let fast = seed % 4 == 0;
        let (start, finish, end, slots) = match seed % 4 {
            0 => {
                let k = 1 + r.next_u32() % 3;
                (data, data + 4 * k, data + 4 * (k + 2), k)
            }
            1 => (data, data + 4, data + 4, 1),
            2 => (data, data + 12, data + 12, 3),
            _ => (0, 0, 0, 0),
        };
        // Room: the insert point is an element (up to the last one); full:
        // any point up to the finish.
        let span = if fast { slots } else { slots + 1 };
        let pos = start + 4 * ((seed / 4) % span);
        c.bus.write32(vec + 4, start);
        c.bus.write32(vec + 8, finish);
        c.bus.write32(vec + 0xc, end);
        for i in 0..slots {
            c.bus.write32(data + 4 * i, r.next_u32());
        }
        let x = DATA + 0x3000;
        c.bus.write32(x, r.next_u32());
        c.g[A0][0] = sx(vec);
        c.g[A1][0] = sx(pos);
        c.g[A2][0] = sx(x);
    }

    #[test]
    fn std_list_clear_matches() {
        install_fakes();
        check(
            ee::s04::f_001db718__generated,
            std_list_clear,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1db718, 0);
                gate_on(c);
                // The list object at +4 holds the sentinel; 0..3 nodes follow it.
                let list = DATA + 0x1000;
                let head = DATA + 0x2000;
                c.bus.write32(list + 4, head);
                let mut prev = head;
                for i in 0..seed % 4 {
                    let node = DATA + 0x2100 + 0x40 * i;
                    c.bus.write32(prev, node);
                    prev = node;
                }
                c.bus.write32(prev, head);
                c.g[A0][0] = sx(list);
            },
        );
    }

    #[test]
    fn std_vector_insert_aux_matches() {
        install_fakes();
        check(
            ee::s04::f_001db778__generated,
            std_vector_insert_aux,
            200,
            &|c, seed| vector_setup(c, seed),
        );
    }

    #[test]
    fn std_vector_insert_aux_2_matches() {
        install_fakes();
        check(
            ee::s04::f_001db918__generated,
            std_vector_insert_aux_2,
            200,
            &|c, seed| vector_setup(c, seed),
        );
    }

    #[test]
    fn cfe_state_ctor_2_matches() {
        install_fakes();
        check(
            ee::s04::f_001dd6a8__generated,
            cfe_state_ctor_2,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1dd6a8, 0);
                gate_on(c);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x40 * (seed % 3));
            },
        );
    }

    #[test]
    fn cfe_state_ctor_3_matches() {
        install_fakes();
        check(
            ee::s04::f_001dd6e0__generated,
            cfe_state_ctor_3,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1dd6e0, 0);
                gate_on(c);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x40 * (seed % 3));
            },
        );
    }

    #[test]
    fn cfe_state_ctor_4_matches() {
        install_fakes();
        check(
            ee::s04::f_001dd718__generated,
            cfe_state_ctor_4,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1dd718, 0);
                gate_on(c);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x40 * (seed % 3));
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_ctor_2_matches() {
        install_fakes();
        check(
            ee::s04::f_001de240__generated,
            cfe_state_profile_load_ctor_2,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1de240, 0);
                gate_on(c);
                c.g[A0][0] = sx(DATA + 0x1000 + 0x40 * (seed % 3));
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_query_event_reward_matches() {
        install_fakes();
        check(
            ee::s04::f_001ced90__generated,
            cfe_state_trophy_room_query_event_reward,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1ced90, 0);
                gate_on(c);
                // Trophy type (0..4) x challenge (0..4) x peak (0..3) cover
                // the collectibles, big-challenge, earnings and track arms.
                c.g[A0][0] = sx(seed % 3);
                c.g[A1][0] = sx(seed % 7);
                c.g[A2][0] = sx((seed / 25) % 4);
                c.g[A3][0] = sx(seed % 5);
                c.g[T0][0] = sx((seed / 5) % 5);
            },
        );
    }
}
