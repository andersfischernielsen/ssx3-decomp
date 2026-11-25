//! Staging module oc649 (wave234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc649`.

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
    (
        0x0023_c2d8,
        gm_handler7_on_rider_finished,
        "GMHandler7_OnRiderFinished",
    ),
    (0x0023_c618, gm_handler_unused_reset, "GMHandlerUnused_Reset"),
    (
        0x0024_4a40,
        game_module_update_sections,
        "GameModule_UpdateSections",
    ),
];

/// `GMHandler7_OnRiderFinished(game, idx)` (0x23c2d8): rider `idx` (a1) has
/// finished the race. The rider's 0xac-byte results block (the rider's
/// `+0x790` pointer plus `0xfc`) is copied to the stack first (8-byte
/// copies, or ldl/ldr pairs when the block is misaligned), then:
/// - a DNF clears the rider's point entry, sets the next heat and flags the
///   rider as not finished;
/// - a total below the point target stores the points and the next heat;
/// - otherwise the rider is ranked 0, 1 or 2 by the two peak goals, and the
///   results are locked.
pub fn gm_handler7_on_rider_finished(c: &mut Cpu) {
    guest(c, rider_finished)
}

fn rider_finished(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x110);
    c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.g[A1][0] = sx(r32(c, A1) << 2);
    frame.save(c, S0, 0x100);
    frame.save(c, S1, 0xf0);
    frame.save(c, S2, 0xe0);
    frame.save(c, S3, 0xd0);
    frame.save(c, S4, 0xc0);
    frame.save_ra(c, 0xb0);
    let game = r32(c, A0);
    c.g[V1][0] = c.lw(game.wrapping_add(0x84));
    let ai = r32(c, V1);
    c.g[V0][0] = c.lw(ai.wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x790));
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(0xfc));
    c.g[V0][0] = c.g[V1][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[A2][0] = c.g[SP][0];
    c.cycles += 3;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x19c));
    c.g[S2][0] = c.g[A1][0];
    c.g[S4][0] = 0x53_0000;
    if aligned {
        rider_copy_aligned(c);
    } else {
        rider_copy_unaligned(c);
    }

    // The copy's last 12 bytes (8 by ldl/ldr, a word) end the stack block.
    c.cycles += 19;
    let src = r32(c, V1);
    c.ldl(2, src.wrapping_add(0x7));
    c.ldr(2, src);
    c.g[A0][0] = c.lw(src.wrapping_add(0x8));
    let dst = r32(c, A2);
    c.sdl(2, dst.wrapping_add(0x7));
    c.sdr(2, dst);
    let word = c.g[A0][0];
    c.sw(dst.wrapping_add(0x8), word);

    call_ptr(c, 0x0014_dc80, 0x0023_c3d4)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0023_c3e0)?; // cBE_getInterface_Fv(be, 0): the race
    c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x84));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x28)); // the rider
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x480)); // mDidNotFinish
    let dnf = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, S4).wrapping_add(0x6640));
    if dnf {
        rider_dnf(c);
    } else {
        rider_ranked(c)?;
    }

    c.cycles += 11;
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[V0][0] = 1;
    frame.restore(c, S0, 0x100);
    frame.restore(c, S1, 0xf0);
    frame.restore(c, S2, 0xe0);
    frame.restore(c, S3, 0xd0);
    frame.restore(c, S4, 0xc0);
    frame.restore_ra(c, 0xb0);
    let gm = r32(c, V1).wrapping_add(0x9c);
    let locked = c.g[V0][0];
    c.sw(gm, locked);
    frame.close(c);
    Ok(())
}

/// The aligned pass of the results copy: 32 bytes (four doublewords) a pass
/// from `V1` to `A2`, until `V1` reaches `V0`.
fn rider_copy_aligned(c: &mut Cpu) {
    loop {
        c.cycles += 11;
        let src = r32(c, V1);
        let dst = r32(c, A2);
        c.g[A0][0] = c.ld(src);
        c.g[A1][0] = c.ld(src.wrapping_add(0x8));
        c.g[A3][0] = c.ld(src.wrapping_add(0x10));
        c.g[T0][0] = c.ld(src.wrapping_add(0x18));
        let (a0, a1, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A3][0], c.g[T0][0]);
        c.sd(dst, a0);
        c.sd(dst.wrapping_add(0x8), a1);
        c.sd(dst.wrapping_add(0x10), a3);
        c.sd(dst.wrapping_add(0x18), t0);
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A2][0] = sx(dst.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_c38c);
        }
    }
}

/// The misaligned pass of the results copy: the same 32 bytes a pass, moved
/// with ldl/ldr and sdl/sdr pairs (registers a0, a1, a3, t0 carry the
/// halves the generated loop leaves behind).
fn rider_copy_unaligned(c: &mut Cpu) {
    loop {
        c.cycles += 19;
        let src = r32(c, V1);
        c.ldl(4, src.wrapping_add(0x7));
        c.ldr(4, src);
        c.ldl(5, src.wrapping_add(0xf));
        c.ldr(5, src.wrapping_add(0x8));
        c.ldl(7, src.wrapping_add(0x17));
        c.ldr(7, src.wrapping_add(0x10));
        c.ldl(8, src.wrapping_add(0x1f));
        c.ldr(8, src.wrapping_add(0x18));
        let dst = r32(c, A2);
        c.sdl(4, dst.wrapping_add(0x7));
        c.sdr(4, dst);
        c.sdl(5, dst.wrapping_add(0xf));
        c.sdr(5, dst.wrapping_add(0x8));
        c.sdl(7, dst.wrapping_add(0x17));
        c.sdr(7, dst.wrapping_add(0x10));
        c.sdl(8, dst.wrapping_add(0x1f));
        c.sdr(8, dst.wrapping_add(0x18));
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A2][0] = sx(dst.wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0023_c32c);
            }
            continue;
        }
        c.cycles += 2;
        return;
    }
}

/// The DNF branch of [`gm_handler7_on_rider_finished`] (no calls): the
/// rider's point entry is cleared, the next heat is set and the rider's
/// `+0x100` finish flag is zeroed.
fn rider_dnf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
    c.g[A1][0] = 1;
    let points = r32(c, V0);
    c.sw(points, 0);
    c.g[A0][0] = 0x53_0000;
    let manager = r32(c, V1);
    let one = c.g[A1][0];
    c.sw(manager.wrapping_add(0x70), one);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x66a8));
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x84));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
    let rider = r32(c, V1);
    c.sw(rider.wrapping_add(0x100), 0);
    let finished = r32(c, A0);
    let one = c.g[A1][0];
    c.sw(finished, one);
}

/// The not-DNF branch: below the point target the points are kept; at or
/// above it the rider goes on to the tier ranking.
fn rider_ranked(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x9c));
    c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x7c));
    let below = (c.g[V0][0] as i64) < (c.g[V1][0] as i64);
    c.g[V0][0] = below as u64;
    c.g[V1][0] = sx(r32(c, S4).wrapping_add(0x6640));
    if below {
        c.cycles += 11;
        let rider_flag = r32(c, A0).wrapping_add(0x100);
        c.sw(rider_flag, 0);
        c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, V1)));
        c.g[V0][0] = 0x53_0000;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x9c));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x66a8));
        c.g[A0][0] = 1;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        let points = r32(c, V1);
        let total = c.g[A1][0];
        c.sw(points, total);
        let heat = r32(c, A2).wrapping_add(0x70);
        let one = c.g[A0][0];
        c.sw(heat, one);
        let not_done = r32(c, V0);
        c.sw(not_done, 0);
        return Ok(());
    }
    rider_tiers(c)
}

/// The tier ranking: the two peak goals of the event times 100 are the tier
/// targets; the total at or above the second tier is rank 0.
fn rider_tiers(c: &mut Cpu) -> Flow {
    c.cycles += 29;
    c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x8));
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 2;
    c.g[S1][0] = 0x64;
    call_ptr(c, 0x0014_55d0, 0x0023_c498)?; // getPeakGoal(race, event, 2)
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.mult(0, 16, 2, 17); // s0 = tier 2 target
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 1;
    c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0x8));
    call_ptr(c, 0x0014_55d0, 0x0023_c4b0)?; // getPeakGoal(race, event, 1)
    c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.mult(0, 8, 2, 17); // t0 = tier 1 target
    c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A3][0] = 1;
    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x84));
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(0x6640));
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
    let rider_flag = r32(c, V1).wrapping_add(0x100);
    let one = c.g[A3][0];
    c.sw(rider_flag, one);
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x9c));
    let points = r32(c, A0);
    let total = c.g[V0][0];
    c.sw(points, total);
    let done = r32(c, A2).wrapping_add(0x84);
    c.sw(done, one);
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x9c));
    c.g[S0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
    let below_t2 = c.g[S0][0] != 0;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T0][0] as i64)) as u64;
    if below_t2 {
        rider_rank_low(c);
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = 0x53_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6730));
    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
    let rank = r32(c, V0);
    c.sw(rank, 0);
    Ok(())
}

/// Rank 1 or 2 (below the second tier): 2 when the total is also below the
/// first tier (`v0` from the compare), else 1 (`a3`).
fn rider_rank_low(c: &mut Cpu) {
    c.cycles += 2;
    let below_t1 = c.g[V0][0] != 0;
    c.g[V0][0] = 0x53_0000;
    if below_t1 {
        c.cycles += 4;
        c.g[V1][0] = 2;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6730));
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        let rank = r32(c, V0);
        let two = c.g[V1][0];
        c.sw(rank, two);
    } else {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6730));
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        let rank = r32(c, V0);
        let one = c.g[A3][0];
        c.sw(rank, one);
    }
}

/// `cGMDebugHandler_init(self)` (0x23c618, GMHandlerUnused_Reset): the debug
/// game mode's reset. One heat (the last), the AI count from the new race,
/// the ten seats filled from a shuffled list of the riders with the human's
/// character kept in the human's seat, the cleared flags, and the handler's
/// virtual `cleanRiderRanksPoints` at the end.
pub fn gm_handler_unused_reset(c: &mut Cpu) {
    guest(c, unused_reset)
}

fn unused_reset(c: &mut Cpu) -> Flow {
    c.cycles += 34;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S2, 0x40);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    frame.save_ra(c, 0x30);
    c.g[S2][0] = c.g[A0][0];
    call_ptr(c, 0x0014_dc80, 0x0023_c634)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0023_c640)?; // cBE_getInterface_Fv(be, 0)
    call_ptr(c, 0x0014_dc80, 0x0023_c648)?; // cBE_getBE() again
    c.g[A1][0] = 1;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x0023_c654)?; // cBE_getInterface_Fv(be, 1): the player
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A1][0] = 1;
    c.g[S0][0] = c.g[V0][0];
    c.g[A2][0] = 0x53_0000;
    let manager = r32(c, V1);
    let one = c.g[A1][0];
    c.sw(manager.wrapping_add(0x98), one);
    c.g[A0][0] = c.g[S0][0];
    c.sw(manager, one);
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x5c04));
    c.sw(manager.wrapping_add(0x14), 0);
    let ai = c.g[V0][0];
    c.sw(manager.wrapping_add(0x10), ai);
    call_ptr(c, 0x0014_6e98, 0x0023_c684)?; // getHumanRider(player, 0)
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0014_7398, 0x0023_c694)?; // getRiderCharID(player, human)
    c.g[S0][0] = c.g[V0][0];

    // The rider ids 9..0 into the stack array (arr[i] = i).
    c.g[V1][0] = 9;
    c.g[V0][0] = sx(sp(c).wrapping_add(0x24));
    loop {
        c.cycles += 7;
        let at = r32(c, V0);
        let id = c.g[V1][0];
        c.sw(at, id);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_c6a0);
        }
    }
    c.cycles += 7;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0xa;
    call_ptr(c, 0x0023_c770, 0x0023_c6c8)?; // GameRand_Shuffle(arr, 10)

    // Seat the riders: iCharacters[j] from the shuffled list, skipping the
    // human's character wherever it comes up.
    c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A1][0] = 0;
    c.g[V1][0] = c.g[SP][0];
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x18));
    loop {
        c.cycles += 2;
        if c.g[A1][0] == c.g[S1][0] {
            let seat = r32(c, A0);
            let ch = c.g[S0][0];
            c.sw(seat, ch);
        } else {
            c.cycles += 3;
            let src = r32(c, V1);
            c.g[V0][0] = c.lw(src);
            if c.g[V0][0] != c.g[S0][0] {
                let seat = r32(c, A0);
                let id = c.g[V0][0];
                c.sw(seat, id);
            } else {
                c.cycles += 3;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                c.g[V0][0] = c.lw(r32(c, V1));
                let seat = r32(c, A0);
                let id = c.g[V0][0];
                c.sw(seat, id);
            }
            c.cycles += 1;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        }
        c.cycles += 4;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < 10) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_c6d8);
        }
    }

    c.cycles += 25;
    c.g[V0][0] = c.lw(r32(c, S2));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x20));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x24));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0023_c720)?; // self->cleanRiderRanksPoints()
    c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A0][0] = 1;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore_ra(c, 0x30);
    let gm = r32(c, V0);
    let neg = c.g[V1][0];
    c.sw(gm.wrapping_add(0x6c), neg);
    let one = c.g[A0][0];
    c.sw(gm.wrapping_add(0x74), one);
    c.sw(gm.wrapping_add(0x94), 0);
    c.sw(gm.wrapping_add(0x70), one);
    for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90] {
        c.sw(gm.wrapping_add(off), 0);
    }
    frame.close(c);
    Ok(())
}

/// `cGame_updateStreamLoading(self)` (0x244a40): the per-frame stream
/// update. When the view and the section manager are both ready the world
/// and sections run and the stream is marked unpaused; otherwise the world
/// and the view update, the sections update if they are not ready, the stream
/// pauses and, on the first pause with the view still loading, the flush flag
/// is set. A set flush flag flushes the streamer's focus and the world cache.
/// Returns 1 when the stream is ready, 0 otherwise.
pub fn game_module_update_sections(c: &mut Cpu) {
    guest(c, update_sections)
}

fn update_sections(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x84));
    call_ptr(c, 0x0022_e920, 0x0024_4a58)?; // cGameViewMan_isStreamReady(view)
    let ready = if c.g[V0][0] == 0 {
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
        false
    } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x78));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1d0));
        let sections_ready = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
        sections_ready
    };
    let value = if ready {
        stream_ready(c)?;
        1
    } else {
        stream_not_ready(c)?;
        0
    };
    c.g[V0][0] = value;
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The stream and the sections are ready: run the world and the sections
/// with the stream unpaused (the tail that returns 1).
fn stream_ready(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A0][0] = 1;
    c.g[V1][0] = c.lw(r32(c, V0));
    let world = r32(c, V1).wrapping_add(0x3d4);
    let one = c.g[A0][0];
    c.sw(world, one);
    c.sw(r32(c, S0).wrapping_add(0x9c), 0);
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
    c.sw(r32(c, S0).wrapping_add(0xa0), 0);
    call_ptr(c, 0x003a_6928, 0x0024_4b2c)?; // cWorld_update(world)
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x78));
    call_ptr(c, 0x0022_d8d8, 0x0024_4b34)?; // cSectionMan_update(sections)
    Ok(())
}

/// The stream is not ready: the world and the view update, the sections
/// update when they are not ready (with the state update first), the pause
/// and the flush flag are set, and a flush runs when the flag was set. Returns
/// 0.
fn stream_not_ready(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[V1][0] = c.lw(r32(c, V0));
    let world = r32(c, V1).wrapping_add(0x3d4);
    c.sw(world, 0);
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
    call_ptr(c, 0x003a_6928, 0x0024_4a80)?; // cWorld_update(world)
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x84));
    call_ptr(c, 0x0022_e840, 0x0024_4a88)?; // cGameViewMan_updateAll(view)
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x78));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1d0));
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x9c));
    } else {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_0f40, 0x0024_4aa0)?; // cGame_updateState(self)
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x78));
        call_ptr(c, 0x0022_d8d8, 0x0024_4aa8)?; // cSectionMan_update(sections)
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x9c));
    }

    // The stream was paused already? Otherwise pause it and, if the view
    // still is not ready, set the flush flag.
    c.cycles += 2;
    let paused = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if !paused {
        c.cycles += 4;
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x84));
        call_ptr(c, 0x0022_e920, 0x0024_4abc)?; // cGameViewMan_isStreamReady(view)
        let view_ready = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !view_ready {
            c.cycles += 2;
            let flush = r32(c, S0).wrapping_add(0xa0);
            let one = c.g[V0][0];
            c.sw(flush, one);
            c.g[V0][0] = 1;
        }
    }

    c.cycles += 3;
    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0xa0));
    let no_flush = c.g[V1][0] == 0;
    let pause = r32(c, S0).wrapping_add(0x9c);
    let one = c.g[V0][0];
    c.sw(pause, one);
    if !no_flush {
        stream_flush(c)?;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    Ok(())
}

/// The flush: the graphics manager's virtual call, the view's stream focus,
/// the world cache's active pages, and the flush flag cleared.
fn stream_flush(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x98));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x9c));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0024_4af0)?; // GrMan->vt[0x9c]()
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x84));
    call_ptr(c, 0x0022_e8b8, 0x0024_4af8)?; // cGameViewMan_updateStreamFocus(view)
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
    c.g[A0][0] = c.lw(r32(c, V0));
    call_ptr(c, 0x003a_88a8, 0x0024_4b04)?; // cWorldCache_flushInActivePages(world)
    c.sw(r32(c, S0).wrapping_add(0xa0), 0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's own RAM: the fakes below log only when it
    /// is set, so other tests keep the generated callees.
    const CTRL: u32 = DATA + 0x20_f000;
    const MAGIC: u32 = 0x6f63_3634;
    /// Scratch area of the fixtures (the machine is fresh per run).
    const BASE: u32 = DATA + 0x20_0000;

    macro_rules! gated {
        ($fake:ident, $prev:ident, $target:literal, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(CTRL) != MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated!(
        oc649_fake_be,
        OC649_PREV_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    gated!(
        oc649_fake_iface,
        OC649_PREV_IFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    gated!(
        oc649_fake_peak,
        OC649_PREV_PEAK,
        0x0014_55d0,
        ee::s01::f_001455d0__generated
    );
    gated!(
        oc649_fake_human,
        OC649_PREV_HUMAN,
        0x0014_6e98,
        ee::s01::f_00146e98__generated
    );
    gated!(
        oc649_fake_char,
        OC649_PREV_CHAR,
        0x0014_7398,
        ee::s01::f_00147398__generated
    );
    gated!(
        oc649_fake_all_view,
        OC649_PREV_ALL_VIEW,
        0x0022_e840,
        ee::s05::f_0022e840__generated
    );
    gated!(
        oc649_fake_sm_update,
        OC649_PREV_SM_UPDATE,
        0x0022_d8d8,
        ee::s05::f_0022d8d8__generated
    );
    gated!(
        oc649_fake_focus,
        OC649_PREV_FOCUS,
        0x0022_e8b8,
        ee::s05::f_0022e8b8__generated
    );
    gated!(
        oc649_fake_update_state,
        OC649_PREV_UPDATE_STATE,
        0x0023_0f40,
        ee::s06::f_00230f40__generated
    );
    gated!(
        oc649_fake_world_update,
        OC649_PREV_WORLD_UPDATE,
        0x003a_6928,
        ee::s13::f_003a6928__generated
    );
    gated!(
        oc649_fake_flush,
        OC649_PREV_FLUSH,
        0x003a_88a8,
        ee::s13::f_003a88a8__generated
    );

    static OC649_PREV_READY: AtomicUsize = AtomicUsize::new(0);

    /// `cGameViewMan_isStreamReady`: the answer is the gate's word 4.
    fn oc649_fake_ready(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC649_PREV_READY, ee::s05::f_0022e920__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_e920, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(CTRL + 4) as u64;
        }
    }

    fn oc649_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc649_fake_be,
            &OC649_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc649_fake_iface,
            &OC649_PREV_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001455D0),
            oc649_fake_peak,
            &OC649_PREV_PEAK,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc649_fake_human,
            &OC649_PREV_HUMAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            oc649_fake_char,
            &OC649_PREV_CHAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E840),
            oc649_fake_all_view,
            &OC649_PREV_ALL_VIEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D8D8),
            oc649_fake_sm_update,
            &OC649_PREV_SM_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E8B8),
            oc649_fake_focus,
            &OC649_PREV_FOCUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E920),
            oc649_fake_ready,
            &OC649_PREV_READY,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230F40),
            oc649_fake_update_state,
            &OC649_PREV_UPDATE_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6928),
            oc649_fake_world_update,
            &OC649_PREV_WORLD_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A88A8),
            oc649_fake_flush,
            &OC649_PREV_FLUSH,
        );
    }

    /// A rider finishing: the rider `idx = seed % 4`, its DNF flag on every
    /// fifth seed, the point total below or above the target, the peak goals
    /// (fake calls 2 and 3) and both copy alignments (the stats block at
    /// `rider + 0x800`, shifted by `seed & 4`).
    fn oc649_rider_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0023_c2d8, 0);
        c.pc = 0x0023_c2d8;
        c.bus.write32(CTRL, MAGIC);
        let idx = seed % 4;
        c.g[A1][0] = idx as u64;
        let game = BASE;
        let ai = BASE + 0x1000;
        let riders = BASE + 0x2000;
        let gm = BASE + 0x1_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), game);
        c.bus.write32(GAME_GP.wrapping_sub(0x480), gm);
        c.bus.write32(game + 0x84, ai);
        c.bus.write32(ai + 0xc, riders);
        for k in 0..4u32 {
            let rider = BASE + 0x3000 + k * 0x1000;
            let stats = rider + 0x800 + (seed & 4);
            c.bus.write32(riders + 0x28 + 4 * k, rider);
            c.bus.write32(rider + 0x790, stats);
            c.bus
                .write32(rider + 0x480, (k == idx && seed % 5 == 0) as u32);
            fill(c, &mut r, stats, 0x1b0 / 4);
        }
        let rider = BASE + 0x3000 + idx * 0x1000;
        let stats = rider + 0x800 + (seed & 4);
        c.bus.write32(stats + 0x198, r.next_u32() % 700);
        c.bus.write32(gm + 0x8, seed % 3);
        c.bus.write32(gm + 0x7c, 200 + 100 * (seed % 3));
        c.bus.write32(fakes::LOG_RESULTS + 8, 1 + r.next_u32() % 6);
        c.bus.write32(fakes::LOG_RESULTS + 12, 1 + r.next_u32() % 6);
    }

    /// `GMHandlerUnused_Reset`: the human's seat and the character are chosen
    /// by the fixture (fake calls 4 and 5), the vtable's cleanup is a fake.
    fn oc649_seat_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0023_c618, 0);
        c.pc = 0x0023_c618;
        c.bus.write32(CTRL, MAGIC);
        let this = BASE + 0x7000;
        let vt = BASE + 0x7100;
        let gm = BASE + 0x1_0000;
        c.g[A0][0] = this as u64;
        c.bus.write32(this, vt);
        c.bus.write16(vt + 0x20, (0x10 * (seed % 2)) as u16);
        c.bus.write32(vt + 0x24, fakes::FAKE);
        c.bus.write32(GAME_GP.wrapping_sub(0x480), gm);
        c.bus.write32(0x0053_5c04, 3 + seed % 5);
        fill(c, &mut r, gm, 0x40);
        c.bus.write32(fakes::LOG_RESULTS + 16, 1 + seed % 9);
        c.bus.write32(fakes::LOG_RESULTS + 20, 1 + (seed / 9) % 9);
    }

    /// `GameModule_UpdateSections`: the flags of the stream (ready, sections
    /// ready, paused, flush pending) come from the seed; the view's answer is
    /// the gate's word 4.
    fn oc649_stream_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x0024_4a40, 0);
        c.pc = 0x0024_4a40;
        c.bus.write32(CTRL, MAGIC);
        c.bus.write32(CTRL + 4, seed & 1);
        let this = BASE + 0x9000;
        let view = BASE + 0x9800;
        let world = BASE + 0xa000;
        let world_obj = BASE + 0xa100;
        let sections = BASE + 0xb000;
        let grman = BASE + 0xc000;
        let vt = BASE + 0xc100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x84, view);
        c.bus.write32(this + 0x10, world);
        c.bus.write32(world, world_obj);
        c.bus.write32(this + 0x78, sections);
        c.bus.write32(sections + 0x1d0, (seed >> 1) & 1);
        c.bus.write32(this + 0x9c, (seed >> 2) & 1);
        c.bus.write32(this + 0xa0, (seed >> 3) & 1);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), grman);
        c.bus.write32(grman + 0x10d8, vt);
        c.bus.write16(vt + 0x98, 0x20);
        c.bus.write32(vt + 0x9c, fakes::FAKE);
    }

    #[test]
    fn gm_handler7_on_rider_finished_matches() {
        oc649_install();
        check(
            ee::s06::f_0023c2d8__generated,
            gm_handler7_on_rider_finished,
            300,
            &|c, seed| oc649_rider_setup(c, seed),
        );
    }

    #[test]
    fn gm_handler_unused_reset_matches() {
        oc649_install();
        check(
            ee::s06::f_0023c618__generated,
            gm_handler_unused_reset,
            300,
            &|c, seed| oc649_seat_setup(c, seed),
        );
    }

    #[test]
    fn game_module_update_sections_matches() {
        oc649_install();
        check(
            ee::s06::f_00244a40__generated,
            game_module_update_sections,
            300,
            &|c, seed| oc649_stream_setup(c, seed),
        );
    }
}
