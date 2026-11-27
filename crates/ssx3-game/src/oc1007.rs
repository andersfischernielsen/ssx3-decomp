//! Staging module oc1007 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1007`.

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
    0x0024_72c0,
    cfe_load_state_show_mp_player_stats,
    "cFELoadState_ShowMPPlayerStats",
)];

/// `GameModeMan_GetInstance` (0x237cf8), a direct call in the original.
const OC1007_GAME_MODE_MAN: Callee = Callee {
    addr: 0x0023_7cf8,
    f: ee::s06::GameModeMan_GetInstance,
};

/// One statistic line of the multiplayer result panel (the six identical-looking
/// blocks of `cFELoadState_ShowMPPlayerStats`): hash the widget name at
/// `sp+name`, find the text object on the screen (`this->+0x40`); when it exists
/// format `"%d"`-style text with `stats[field]` (the stats record is saved at
/// `sp+0x224`) into the buffer at `sp+0xa0` and set it as the object's ASCII text.
/// The six blocks differ only in the name slot, the record field and the scratch
/// register that holds the record pointer (`stat_reg`: A3, T0, T1, V0, V1, A3), which
/// the callees see. `ret` is the return address of the hash call; the other three
/// calls return to `ret + 0xc`, `+ 0x44` and `+ 0x50`.
fn mp_stat_line(c: &mut Cpu, name: u32, field: u32, stat_reg: usize, rets: [u32; 4]) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(name));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, rets[1])?;
    c.g[S3][0] = c.g[V0][0];
    let missing = c.g[S3][0] == 0u64;
    c.g[V0][0] = 0x4a0000;
    if missing {
        return Ok(());
    }
    c.cycles += 14;
    // The "%d" format (2 bytes + NUL) at 0x4a2c28, copied through the stack.
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c28));
    c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2c28));
    c.g[A2][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x2));
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sh((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[A1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.sb((c.g[SP][0] as u32).wrapping_add(0xc2), c.g[A2][0]);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.g[stat_reg][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x224));
    c.g[A2][0] = c.lw((c.g[stat_reg][0] as u32).wrapping_add(field));
    call_ptr(c, 0x0041_7828, rets[2])?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x003a_0c50, rets[3])?;
    Ok(())
}

/// `cFELoadState_ShowMPPlayerStats(this, player)` (0x2472c0): fills the
/// multiplayer result panel of one player on the load screen. Looks up the
/// widgets `<n>_Player_rank_numb`, `<n>_rank`, `<n>_time`, `<n>_knockdowns`,
/// `<n>_wipeouts`, `<n>_tricks`, `<n>_handplants`, `<n>_ubertricks`,
/// `<n>_ubergrinds` (`n` = 1, or 2 for the second player) by hash on the load
/// screen (`this+0x40`) and sets their text: ranking, stat level (or 8 in a
/// ranked network game), race time or score, and the last event's counters
/// from `BEScoreInterface::getPlayerLastCompletedEventStats`; finally the player's
/// picture widget gets its texture from the parent module (`this+0x48`).
/// Stack frame 0x2d0: widget names at `sp+0x00..0x9f`, text buffers at
/// `sp+0xa0` (ascii) and `sp+0x150` (unicode), the stats record pointer at `sp+0x224`.
pub fn cfe_load_state_show_mp_player_stats(c: &mut Cpu) {
    guest(c, |c| {
        'b_2474b8: {
            c.cycles += 122;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2d0));
            c.g[V0][0] = 0x480000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.g[V1][0] = 0x4a0000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x280));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x250));
            c.g[S4][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.g[S7][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[A1][0] = 0x7;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x290));
            c.g[S2][0] = 0x530000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x270));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x260));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x240));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x230), c.g[RA][0]);
            // Stack names, 0x10 apart: the widget-name templates copied from the data section
            // ("1_Player_rank_numb" at +0x00, "1_rank" +0x20, "1_time" +0x30, "1_knockdowns" +0x40,
            // "1_wipeouts" +0x50, "1_tricks" +0x60, "1_handplants" +0x70, "1_ubertricks" +0x80,
            // "1_ubergrinds" +0x90). Unaligned 8/4-byte copies and single bytes, as the compiler emitted them.
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x41a0));
            c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(6, (c.g[T1][0] as u32));
            c.ldl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
            c.ldr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x10));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(6, (c.g[SP][0] as u32));
            c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
            c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T0][0]);
            c.g[A2][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x11));
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x12));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A2][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A3][0]);
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c40));
            c.lwl(2, (c.g[T1][0] as u32).wrapping_add(0x3));
            c.lwr(2, (c.g[T1][0] as u32));
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x5));
            c.swl(2, (c.g[SP][0] as u32).wrapping_add(0x23));
            c.swr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x24), c.g[A3][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x25), c.g[T0][0]);
            c.g[V0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x6));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x26), c.g[V0][0]);
            c.g[A2][0] = 0x4a0000;
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2c48));
            c.lwl(3, (c.g[T1][0] as u32).wrapping_add(0x3));
            c.lwr(3, (c.g[T1][0] as u32));
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x5));
            c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x33));
            c.swr(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x34), c.g[A3][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x35), c.g[T0][0]);
            c.g[V1][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x6));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x36), c.g[V1][0]);
            c.g[V0][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4188));
            c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(6, (c.g[T1][0] as u32));
            c.lwl(7, (c.g[T1][0] as u32).wrapping_add(0xb));
            c.lwr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x47));
            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.swl(7, (c.g[SP][0] as u32).wrapping_add(0x4b));
            c.swr(7, (c.g[SP][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4178));
            c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[T1][0] as u32));
            c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x9));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x57));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sb((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
            c.sb((c.g[SP][0] as u32).wrapping_add(0x59), c.g[T0][0]);
            c.g[A2][0] = 0x4a0000;
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2c50));
            c.ldl(3, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[T1][0] as u32));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x67));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4168));
            c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(6, (c.g[T1][0] as u32));
            c.lwl(7, (c.g[T1][0] as u32).wrapping_add(0xb));
            c.lwr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x77));
            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swl(7, (c.g[SP][0] as u32).wrapping_add(0x7b));
            c.swr(7, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.g[V1][0] = 0x480000;
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4158));
            c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[T1][0] as u32));
            c.lwl(7, (c.g[T1][0] as u32).wrapping_add(0xb));
            c.lwr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x87));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swl(7, (c.g[SP][0] as u32).wrapping_add(0x8b));
            c.swr(7, (c.g[SP][0] as u32).wrapping_add(0x88));
            c.g[A2][0] = 0x480000;
            c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4148));
            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[T0][0] as u32));
            c.lwl(4, (c.g[T0][0] as u32).wrapping_add(0xb));
            c.lwr(4, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x97));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.swl(4, (c.g[SP][0] as u32).wrapping_add(0x9b));
            c.swr(4, (c.g[SP][0] as u32).wrapping_add(0x98));
            // cBE_getInterface(world->be (+0x78), 7); then, unless this is the network host (or not
            // a network game), flip the player index (side: `player = player < 1`).
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x220), c.g[S7][0]);
            call_ptr(c, 0x0014dd58, 0x00247490)?;
            c.g[V1][0] = c.g[S2][0];
            c.g[T1][0] = c.g[V1][0];
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4b30));
            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
                break 'b_2474b8;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                c.g[V0][0] = (c.g[S7][0] < 0x1u64) as u64;
                if c.g[V1][0] != 0 {
                    c.g[S7][0] = c.g[V0][0];
                }
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
                break 'b_2474b8;
            }
        }
        'b_2474e8: {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x32;
            if t {
                break 'b_2474e8;
            } else {
                c.cycles += 9;
                c.sb((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                break 'b_2474e8;
            }
        }
        'b_2475cc: {
            c.cycles += 39;
            c.g[S1][0] = (0u64 < c.g[S7][0]) as u64;
            call_ptr(c, 0x0014dc80, 0x002474f0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x002474fc)?;
            c.g[S6][0] = c.g[S1][0];
            c.g[S5][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x00247508)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00247514)?;
            call_ptr(c, 0x0014dc80, 0x0024751c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0014dd58, 0x00247528)?;
            c.g[FP][0] = c.g[V0][0];
            OC1007_GAME_MODE_MAN.call(c, 0x00247530)?;
            call_ptr(c, 0x0014dc80, 0x00247538)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x8;
            call_ptr(c, 0x0014dd58, 0x00247544)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x00146e98, 0x00247554)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00155270, 0x00247560)?;
            c.g[A0][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x224), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x0024756c)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00247578)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = 0x530000;
            if t {
                break 'b_2475cc;
            } else {
                c.cycles += 18;
                c.g[A1][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                c.g[V1][0] = sx((c.g[S7][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c28));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32));
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2c28));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sb((c.g[SP][0] as u32).wrapping_add(0xc2), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[A3][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x002475c0)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x002475cc)?;
                break 'b_2475cc;
            }
        }
        'b_2476a4: {
            c.cycles += 9;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            call_ptr(c, 0x00317670, 0x002475d4)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002475e0)?;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x228), c.g[A0][0]);
            if t {
                break 'b_2476a4;
            } else {
                'b_247640: {
                    'b_247628: {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x7;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                        call_ptr(c, 0x0014dd58, 0x00247600)?;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4b30));
                        if c.g[V1][0] == 0u64 {
                            c.g[A0][0] = c.g[S5][0];
                            break 'b_247628;
                        } else {
                            c.cycles += 5;
                            c.g[A1][0] = 0x530000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] == 0u64 {
                                c.g[A0][0] = c.g[S5][0];
                                break 'b_247628;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[FP][0] = 0x8;
                                if t {
                                    break 'b_247640;
                                } else {
                                    break 'b_247628;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x00146e98, 0x00247630)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00148950, 0x0024763c)?;
                    c.g[FP][0] = c.g[V0][0];
                    break 'b_247640;
                }
                c.cycles += 25;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = 0x480000;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4380));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x228), c.g[S2][0]);
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x00247668)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x00247678)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x002c2508, 0x00247684)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[FP][0];
                call_ptr(c, 0x002c26d0, 0x00247698)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x002476a4)?;
                break 'b_2476a4;
            }
        }
        'b_24780c: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x00317670, 0x002476ac)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002476b8)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = 0x530000;
            if t {
                break 'b_24780c;
            } else {
                'b_247804: {
                    'b_247790: {
                        'b_2476d8: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c10));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_2476d8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A1][0] = c.g[S6][0];
                                if t {
                                    break 'b_247790;
                                } else {
                                    break 'b_2476d8;
                                }
                            }
                        }
                        'b_247724: {
                            'b_247710: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[A1][0] = c.g[S6][0];
                                call_ptr(c, 0x00146e98, 0x002476e4)?;
                                c.g[V1][0] = 0x530000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                if t {
                                    break 'b_247710;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(2, 0);
                                    c.cvt_s_w(0, 0);
                                    let t = true;

                                    if t {
                                        break 'b_247724;
                                    } else {
                                        break 'b_247710;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.add_s(0, 0, 0);
                            break 'b_247724;
                        }
                        'b_247774: {
                            c.cycles += 13;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5008));
                            c.mul_s(12, 0, 12);
                            call_ptr(c, 0x004139f8, 0x00247730)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c58));
                            call_ptr(c, 0x00317670, 0x00247740)?;
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0024774c)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_247774;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x00247770)?;
                                c.g[A0][0] = c.g[S0][0];
                                break 'b_247774;
                            }
                        }
                        c.cycles += 7;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        call_ptr(c, 0x0020a1a8, 0x0024777c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x004166f4, 0x00247788)?;
                        let t = true;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            break 'b_247804;
                        } else {
                            break 'b_247790;
                        }
                    }
                    'b_2477ec: {
                        c.cycles += 16;
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00146e98, 0x00247798)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                        c.g[A0][0] = 0x4a0000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c60));
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x002477b8)?;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x002477c4)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                        if t {
                            break 'b_2477ec;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002477e8)?;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            break 'b_2477ec;
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00247800)?;
                    c.g[A1][0] = c.g[S0][0];
                    break 'b_247804;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x003a0c50, 0x0024780c)?;
                break 'b_24780c;
            }
        }
        mp_stat_line(
            c,
            0x40,
            0x2c,
            A3,
            [0x0024_7814, 0x0024_7820, 0x0024_7858, 0x0024_7864],
        )?;
        mp_stat_line(
            c,
            0x50,
            0x28,
            T0,
            [0x0024_786c, 0x0024_7878, 0x0024_78b0, 0x0024_78bc],
        )?;
        mp_stat_line(
            c,
            0x60,
            0x14,
            T1,
            [0x0024_78c4, 0x0024_78d0, 0x0024_7908, 0x0024_7914],
        )?;
        mp_stat_line(
            c,
            0x70,
            0xc,
            V0,
            [0x0024_791c, 0x0024_7928, 0x0024_7960, 0x0024_796c],
        )?;
        mp_stat_line(
            c,
            0x80,
            0x18,
            V1,
            [0x0024_7974, 0x0024_7980, 0x0024_79b8, 0x0024_79c4],
        )?;
        mp_stat_line(
            c,
            0x90,
            0x20,
            A3,
            [0x0024_79cc, 0x0024_79d8, 0x0024_7a10, 0x0024_7a1c],
        )?;
        'b_247a84: {
            'b_247a80: {
                c.cycles += 17;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
                c.g[V0][0] = 0x4a0000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c18));
                c.g[T1][0] = c.lb((c.g[A2][0] as u32));
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                c.sb((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[T1][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0xe1), c.g[A0][0]);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x228));
                c.sb((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V1][0]);
                call_ptr(c, 0x00317670, 0x00247a48)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00247a54)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                let t = c.g[A0][0] == 0u64;
                c.g[V1][0] = c.g[V0][0];
                if t {
                    break 'b_247a80;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                    if t {
                        break 'b_247a80;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                        if t {
                            break 'b_247a84;
                        } else {
                            c.cycles += 2;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                            break 'b_247a80;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            break 'b_247a84;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x290));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x280));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x270));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x260));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x250));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x240));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x230));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, covered, machine};
    use crate::difftest::{elf_image, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x400000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC1007_CTRL: u32 = DATA + 0x40_0000;
    pub(crate) const OC1007_MAGIC: u32 = 0x1007_5a11;
    /// Bit k of this word: the k-th widget lookup finds the object.
    pub(crate) const OC1007_FIND_MASK: u32 = OC1007_CTRL + 4;
    /// Number of widget lookups made so far.
    pub(crate) const OC1007_FIND_COUNT: u32 = OC1007_CTRL + 8;
    /// The human-rider index the rider getter returns.
    pub(crate) const OC1007_RIDER: u32 = OC1007_CTRL + 0xc;
    pub(crate) const OC1007_STATS: u32 = DATA + 0x41_0000;
    pub(crate) const OC1007_WORLD: u32 = DATA + 0x42_0000;
    pub(crate) const OC1007_SELF: u32 = DATA + 0x43_0000;
    pub(crate) const OC1007_SCREEN: u32 = DATA + 0x44_0000;
    pub(crate) const OC1007_OBJ: u32 = DATA + 0x45_0000;
    pub(crate) const OC1007_OBJ_VT: u32 = DATA + 0x46_0000;
    pub(crate) const OC1007_LOCALE: u32 = DATA + 0x47_0000;
    pub(crate) const OC1007_LOCALE_VT: u32 = DATA + 0x48_0000;
    pub(crate) const OC1007_PARENT: u32 = DATA + 0x49_0000;

    /// The callee: logs the call like `fakes::fake_call`, then applies the
    /// fixture's results: the widget finder answers per the lookup mask, the
    /// rider getter and the stats getter return the fixture's values.
    fn oc1007_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0039_d860 => {
                let n = c.bus.read32(OC1007_FIND_COUNT);
                c.bus.write32(OC1007_FIND_COUNT, n + 1);
                let mask = c.bus.read32(OC1007_FIND_MASK);
                c.g[V0][0] = if (mask >> (n % 32)) & 1 != 0 {
                    OC1007_OBJ as u64
                } else {
                    0
                };
            }
            0x0014_6e98 => c.g[V0][0] = c.bus.read32(OC1007_RIDER) as u64,
            0x0015_5270 => c.g[V0][0] = OC1007_STATS as u64,
            _ => {}
        }
    }

    macro_rules! oc1007_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1007_CTRL) != OC1007_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1007_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1007_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1007_fakes!(
        (
            oc1007_fake_0014dc80,
            OC1007_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc1007_fake_0014dd58,
            OC1007_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc1007_fake_00146e98,
            OC1007_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc1007_fake_00155270,
            OC1007_PREV_00155270,
            ee::s01::OVERRIDE_00155270,
            ee::s01::f_00155270__generated,
            0x00155270
        ),
        (
            oc1007_fake_00148950,
            OC1007_PREV_00148950,
            ee::s01::OVERRIDE_00148950,
            ee::s01::f_00148950__generated,
            0x00148950
        ),
        (
            oc1007_fake_002c26d0,
            OC1007_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc1007_fake_002c2508,
            OC1007_PREV_002C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c2508
        ),
        (
            oc1007_fake_00317670,
            OC1007_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc1007_fake_0039d860,
            OC1007_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc1007_fake_003a0c50,
            OC1007_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc1007_fake_003a0e90,
            OC1007_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc1007_fake_00417828,
            OC1007_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc1007_fake_004139f8,
            OC1007_PREV_004139F8,
            ee::s15::OVERRIDE_004139F8,
            ee::s15::f_004139f8__generated,
            0x004139f8
        ),
        (
            oc1007_fake_004166f4,
            OC1007_PREV_004166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x004166f4
        ),
        (
            oc1007_fake_0020a1a8,
            OC1007_PREV_0020A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020a1a8
        ),
    );

    /// A load-state object with its screen and parent module, the world with
    /// the locale lookup slot, the network and game-mode globals and the
    /// per-rider result tables, all chosen from `seed`.
    fn oc1007_setup(c: &mut Cpu, seed: u32) {
        elf_image(c);
        let mut r: Rng = machine(c, seed, 0x0024_72c0, 45);
        c.pc = 0x0024_72c0;
        c.bus.write32(OC1007_CTRL, OC1007_MAGIC);
        let pick = |r: &mut Rng, n: u32| r.next_u32() % n;

        // Which widgets exist: mostly most of them, sometimes all or none.
        let mask = match seed % 11 {
            0 => 0,
            1 | 2 => u32::MAX,
            _ => r.next_u32() | r.next_u32(),
        };
        c.bus.write32(OC1007_FIND_MASK, mask);
        c.bus.write32(OC1007_RIDER, pick(&mut r, 6));
        fill(c, &mut r, OC1007_STATS, 0x20);

        // The world (gp-0x848): +0x78 back-end root, +0x8c the locale object whose
        // vtable slot (+0x24, offset in +0x20) the level text comes from.
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC1007_WORLD);
        c.bus.write32(OC1007_WORLD + 0x78, DATA + 0x4a_0000);
        c.bus.write32(OC1007_WORLD + 0x8c, OC1007_LOCALE);
        c.bus.write32(OC1007_LOCALE + 4, OC1007_LOCALE_VT);
        c.bus
            .write16(OC1007_LOCALE_VT + 0x20, (r.next_u32() & 0x7c) as u16);
        c.bus.write32(OC1007_LOCALE_VT + 0x24, fakes::FAKE + 0x30);
        // GameModeMan_GetInstance finds its instance (gp-0x484) already built.
        c.bus.write32(GAME_GP.wrapping_sub(0x484), DATA + 0x5000);

        // Network options (0x534b30: network game, ranked game, host) and game mode.
        for i in 0..3u32 {
            c.bus.write32(0x0053_4b30 + 4 * i, pick(&mut r, 2));
        }
        let modes = [0u8, 5, 1, 2, 3, 4, 0xff];
        c.bus
            .write8(0x0053_5c10, modes[(seed as usize / 3) % modes.len()]);
        // Rankings and per-rider scores/times (half with the sign bit, for the
        // unsigned-to-float conversion).
        for i in 0..8u32 {
            c.bus.write32(0x0053_6730 + 4 * i, pick(&mut r, 8));
            let w = r.next_u32();
            c.bus
                .write32(0x0053_6640 + 4 * i, if i % 2 == 0 { w } else { w >> 1 });
        }

        // The found widget with its vtable (slot 0x48/0x4c: `setVisible`).
        c.bus.write32(OC1007_OBJ + 8, OC1007_OBJ_VT);
        c.bus
            .write16(OC1007_OBJ_VT + 0x48, (r.next_u32() & 0x3c) as u16);
        c.bus.write32(OC1007_OBJ_VT + 0x4c, fakes::FAKE + 0x40);

        // The load state: screen (+0x40) and parent module (+0x48, sometimes none)
        // with its per-player texture ids (+0x10..; half of them 0).
        c.bus.write32(OC1007_SELF + 0x40, OC1007_SCREEN);
        let parent = if seed % 7 == 3 { 0 } else { OC1007_PARENT };
        c.bus.write32(OC1007_SELF + 0x48, parent);
        for i in 0..0x20u32 {
            let w = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(OC1007_PARENT + 4 * i, w);
        }

        c.g[A0][0] = sx(OC1007_SELF);
        c.g[A1][0] = [0u64, 1, 2, 0xff, 0x7f, 0x100][(seed % 6) as usize];
    }

    /// `cFELoadState_ShowMPPlayerStats(this, player)`: sets the multiplayer
    /// result texts (rank, level, time or score, stat counters, picture texture)
    /// through the real control flow; the callees are fakes (some unwinding)
    /// whose lookups find some widgets and not others.
    #[test]
    fn cfe_load_state_show_mp_player_stats_matches() {
        oc1007_install();
        covered(
            ee::s06::f_002472c0__generated,
            cfe_load_state_show_mp_player_stats,
            300,
            &oc1007_setup,
        );
    }

    /// Every arm makes its calls for some seed: the rank number, the stat
    /// level (both rank sources), the unicode text, race time and score, the six
    /// stat counters and the picture lookup.
    #[test]
    fn cfe_load_state_show_mp_player_stats_reaches_every_arm() {
        oc1007_install();
        check_reaching(
            ee::s06::f_002472c0__generated,
            cfe_load_state_show_mp_player_stats,
            300,
            &[
                0x0024_7578,
                0x0024_75c0,
                0x0024_75cc,
                0x0024_7630,
                0x0024_763c,
                0x0024_7698,
                0x0024_76a4,
                0x0024_7730,
                0x0024_7770,
                0x0024_7788,
                0x0024_77e8,
                0x0024_7800,
                0x0024_780c,
                0x0024_7858,
                0x0024_78b0,
                0x0024_7908,
                0x0024_7960,
                0x0024_79b8,
                0x0024_7a10,
                0x0024_7a1c,
                0x0024_7a54,
            ],
            &oc1007_setup,
        );
    }
}
