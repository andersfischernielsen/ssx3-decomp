//! Staging module oc882 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc882`.

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
        0x001b_3bf0,
        cfe_state_chat_room_update_player_quality,
        "cFEStateChatRoom_UpdatePlayerQuality",
    ),
    (
        0x001b_3d20,
        cfe_state_chat_room_highlight_player,
        "cFEStateChatRoom_HighlightPlayer",
    ),
];

/// `void cFEStateChatRoom_UpdatePlayerQuality(cFEStateChatRoom* self)`
/// (0x1b3bf0): walks the chat room's player slots (pointers at `self +
/// 0x6d0`, four bytes apart) until the slot counter `s3` reaches six. Each
/// slot's object has a vtable whose method at `+0x4c` is called with the
/// object's `+0x48` adjustment; slots without that method get their quality
/// icons refreshed (`NetFE_SetQualityIcons`) after the name is formatted
/// with `sprintf` from the string at 0x463a60.
pub fn cfe_state_chat_room_update_player_quality(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S4, 0x70);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6d0));
        frame.save(c, S3, 0x80);
        frame.save(c, S5, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S6, 0x50);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save_ra(c, 0x40);
        c.g[S6][0] = 0x0046_0000;
        call_ptr(c, 0x0025_f1b0, 0x001b_3c2c)?;
        c.g[S5][0] = c.g[V0][0];
        loop {
            update_player_quality_pass(c)?;
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_3c30);
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Calls the method at `vt + 0x4c` of the object `obj` (its vtable at
/// `obj + 8`), with `a0 = obj + lh(vt + 0x48)`, returning to `ret`.
fn update_player_quality_method(c: &mut Cpu, obj: u64, ret: u32) -> Flow {
    c.g[A2][0] = obj;
    c.g[V0][0] = Addr((obj as u32).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// One pass of the player loop (generated blocks `b_1b3cec`, `b_1b3cd0`,
/// `b_1b3ca8`): format the slot's name, then either call its method (the
/// slot counter `s3` advances) or refresh its quality icons.
fn update_player_quality_pass(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x704)).lw(c);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x3a60));
    c.g[A2][0] = c.g[S3][0];
    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
    call_ptr(c, 0x0041_7828, 0x001b_3c48)?;
    c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[S5][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        // Past the poll count: one method call on the slot, no counter step
        // before it (the slot object is `*s2`).
        c.g[A2][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.cycles += 7;
        c.g[A1][0] = 0;
        let obj = c.g[A2][0];
        update_player_quality_method(c, obj, 0x001b_3ce8)?;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        return Ok(());
    }
    // The poll count applies: `poll(view, s1)` then the slot's method.
    c.cycles += 13;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0025_f248, 0x001b_3c60)?;
    c.g[A2][0] = Addr(c.g[S2][0] as u32).lw(c);
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    let obj = c.g[A2][0];
    update_player_quality_method(c, obj, 0x001b_3c80)?;
    c.g[A1][0] = 0;
    if c.g[S0][0] != 0 {
        c.cycles += 8;
        c.g[A2][0] = Addr(c.g[S2][0] as u32).lw(c);
        let obj = c.g[A2][0];
        update_player_quality_method(c, obj, 0x001b_3ca0)?;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        return Ok(());
    }
    c.cycles += 10;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    call_ptr(c, 0x0025_f288, 0x001b_3cb8)?;
    c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x30)).lw(c);
    call_ptr(c, 0x001a_ad50, 0x001b_3cc8)?;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
    Ok(())
}

/// `void cFEStateChatRoom_HighlightPlayer(self, int slot, int a2)`
/// (0x1b3d20): formats the highlighted player's name and object, and moves
/// the highlight to the slot in `a1` (the chat room's player list, `self +
/// 0x40`). Returns through the epilogue of the generated body on every path.
pub fn cfe_state_chat_room_highlight_player(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xc0);
        highlight_player_body(c)?;
        c.cycles += 10;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `cFEStateChatRoom_HighlightPlayer` between the prologue and
/// the epilogue. `Ok` means the generated code reached its epilogue.
fn highlight_player_body(c: &mut Cpu) -> Flow {
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
    c.g[S5][0] = c.g[A2][0];
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    // Block `b_1b3dc0`: look the player up and decide whether to move on.
    c.cycles += 21;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[S6][0] = c.g[A1][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3a60));
    call_ptr(c, 0x0041_7828, 0x001b_3d74)?;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x001b_3d7c)?;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_3d88)?;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
    c.g[S4][0] = c.g[V0][0];
    call_ptr(c, 0x0025_f1b0, 0x001b_3d94)?;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x6ec)).lw(c);
    c.g[A3][0] = c.g[V0][0];
    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x704)).lw(c);
    c.g[A2][0] = 6;
    call_ptr(c, 0x0039_da20, 0x001b_3da8)?;
    if c.g[S4][0] == 0 {
        c.cycles += 2;
        let negative = (c.g[S3][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < 6) as u64;
        if !negative {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                return Ok(());
            }
        }
    }
    // Block `b_1b3dc0` fell through: the highlight loop over the six slots.
    c.cycles += 6;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 20);
    c.g[S0][0] = 0;
    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A0][0] = c.g[SP][0];
    loop {
        c.cycles += 22;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x3a60));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x0041_7828, 0x001b_3de8)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0031_7670, 0x001b_3df0)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001b_3dfc)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        Addr((c.g[SP][0] as u32).wrapping_add(0x24)).sw(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        Addr((c.g[SP][0] as u32).wrapping_add(0x28)).sw(c, 0);
        Addr((c.g[SP][0] as u32).wrapping_add(0x2c)).sw(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, target, 0x001b_3e24)?;
        c.g[V1][0] = ((c.g[S0][0] as i64) < 6) as u64;
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[SP][0];
            if event_due(c) {
                deliver_event_here(c, 0x001b_3dd8);
            }
            continue;
        }
        break;
    }
    c.cycles += 9;
    c.g[A0][0] = sx(0x004a_17e0);
    call_ptr(c, 0x0031_7670, 0x001b_3e3c)?;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_3e48)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    // Block `b_1b3e7c`: with a poll count (`s5`), go on to the name; without,
    // call the object's method and leave.
    c.cycles += 2;
    let poll = c.g[S5][0] != 0;
    c.g[A1][0] = 0x0046_0000;
    if !poll {
        c.cycles += 8;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_3e74)?;
        return Ok(());
    }
    c.cycles += 11;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3ea0));
    c.g[A2][0] = c.g[S3][0];
    call_ptr(c, 0x0041_7828, 0x001b_3e8c)?;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x001b_3e94)?;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_3ea0)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 40;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x003a_0290, 0x001b_3eb4)?;
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A1][0] = 0x1;
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(1, 3);
    c.add_s(1, 1, 2);
    c.add_s(0, 0, 3);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x28)).lw(c);
    c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x4b));
    c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x44));
    let v1 = c.g[V1][0];
    Addr((c.g[S0][0] as u32).wrapping_add(0x4c)).sw(c, v1);
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001b_3f0c)?;
    c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x18)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x001b_27b0, 0x001b_3f18)?;
    c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001b_3f48)?;
    // The last call returned to the common tail of the body.
    c.cycles += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, DATA, GAME_GP};

    /// A player object: vtable at `obj + 8`, method at `+0x4c` (a leaf that
    /// returns), adjustment `+0x48` zero.
    fn write_player(c: &mut Cpu, obj: u32, vt: u32) {
        c.bus.write32(obj + 8, vt);
        c.bus.write16(vt + 0x48, 0);
        c.bus.write16(vt + 0x58, 0);
        c.bus.write32(vt + 0x4c, 0x0014_06c0);
        c.bus.write32(vt + 0x5c, 0x0014_06c0);
    }

    #[test]
    fn cfe_state_chat_room_update_player_quality_matches() {
        check(
            ssx3_ee::s03::f_001b3bf0__generated,
            cfe_state_chat_room_update_player_quality,
            60,
            &|c, seed| {
                c.pc = 0x001b_3bf0;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x2000);
                c.bus.write32(DATA + 0x704, seed % 4);
                for k in 0..6 {
                    let obj = DATA + 0x3000 + 0x100 * k;
                    c.bus.write32(DATA + 0x6d0 + 4 * k, obj);
                    write_player(c, obj, DATA + 0x4000);
                }
                c.bus
                    .write32(DATA + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
            },
        );
    }

    #[test]
    fn cfe_state_chat_room_highlight_player_matches() {
        check(
            ssx3_ee::s03::f_001b3d20__generated,
            cfe_state_chat_room_highlight_player,
            60,
            &|c, seed| {
                c.pc = 0x001b_3d20;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 6) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x2000);
                c.bus.write32(DATA + 0x40, DATA + 0x1000);
                c.bus.write32(DATA + 0x6ec, DATA + 0x5000);
                c.bus.write32(DATA + 0x704, seed % 4);
                write_player(c, DATA + 0x1000, DATA + 0x4000);
                write_player(c, DATA + 0x5000, DATA + 0x4000);
            },
        );
    }
}
