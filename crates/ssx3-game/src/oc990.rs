//! Staging module oc990 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc990`.

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
        0x001f_88e8,
        ov_screen_pause_freeride_shutdown,
        "OVScreenPauseFreeride_Shutdown",
    ),
    (
        0x001f_db88,
        ov_screen_rival_pre_on_widget_action,
        "OVScreenRivalPre_OnWidgetAction",
    ),
    (
        0x001f_b2b8,
        ov_screen_in_game_error_handle_select,
        "OVScreenInGameError_HandleSelect",
    ),
    (
        0x001f_a9e0,
        ov_screen_be_options_update,
        "OVScreenBEOptions_Update",
    ),
];

/// `void OVScreenPauseFreeride_Shutdown(OVScreenPauseFreeride* self)`
/// (0x1f88e8): clear the screen's +0xc4 word and run the overlay base shutdown
/// (`cOVStateOverlay_Shutdown`, 0x20a430).
pub fn ov_screen_pause_freeride_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), 0u64);
        call_ptr(c, 0x0020_a430, 0x001f_88fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenRivalPre_OnWidgetAction(OVScreenRivalPre* self, widget, event)`
/// (0x1fdb88): on the "next" event (5) with a widget, set the
/// overlay status to 3, mark the game state's overlay done, start the
/// no-start-gate audio event and pop the UI state stack once.
pub fn ov_screen_rival_pre_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_widget = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut run_tail = false;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            if c.g[A2][0] == c.g[V0][0] {
                c.cycles += 14;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = 0x3;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x200));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                call_ptr(c, 0x0028_b180, 0x001f_dbc8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0028_72a8, 0x001f_dbd0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_dbe0)?;
                run_tail = true;
            }
        } else {
            run_tail = true;
        }
        if run_tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSaveReplay_Shutdown(OVScreenSaveReplay* self)` (0x1fde60):
/// when the screen's +0x1c8 flag is set, clear it through `0x270ed8`; then
/// drop the replay-state table entry (`gp-0x69c`), clear the replay slot at
/// `gp-0x6a0`, and run the two base shutdowns (`0x1d58b8`, `0x26f8a0`).
#[allow(dead_code)]
pub fn ov_screen_save_replay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
        let not_set = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        if !not_set {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0027_0ed8, 0x001f_de90)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), 0u64);
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        }
        c.cycles += 26;
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
        c.g[A2][0] = 0x440000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A1][0]);
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
        call_ptr(c, 0x001d_58b8, 0x001f_ded4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0026_f8a0, 0x001f_def0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `cGame_exit(game, 0)` tail of `OVScreenInGameError_HandleSelect`
/// (0x1fb360 return): `a1 = 0`, `a0 = world->game`, call 0x230488.
fn in_game_error_exit_call(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[A1][0] = 0u64;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    call_ptr(c, 0x0023_0488, 0x001f_b360)
}

/// `void OVScreenInGameError_HandleSelect(OVScreenInGameError* self, item)`
/// (0x1fb2b8): hide the error screen's options (`+0x1c` keeps its low bits
/// and sets `0x780`), pop the UI state stack, then act on the item kind
/// (`item+0x18`): kind 4 calls 0x12b340 and pops the table entry at
/// `gp-0x69c`, other kinds exit the game (`cGame_exit`).
pub fn ov_screen_in_game_error_handle_select(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f190, 0x001f_b2e0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | 0x780;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        let kind_zero = c.g[V1][0] == 0u64;
        c.g[V0][0] = 0x4;
        if kind_zero {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            in_game_error_exit_call(c)?;
        } else {
            c.cycles += 2;
            let kind_not_four = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if kind_not_four {
                in_game_error_exit_call(c)?;
            } else {
                c.cycles += 18;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_b340, 0x001f_b314)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                c.g[A0][0] = 0x530000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                c.g[A1][0] = 0x440000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBEOptions_Update(OVScreenBEOptions* self)` (0x1fa9e0): run the
/// overlay update, then, while the current option table (`gp-0x204`) has
/// its two flags clear, play the screen's frame named by the hash of
/// `D_0046E050` (`cUIScreen_playFrame`) and pop the UI state stack.
pub fn ov_screen_be_options_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x001f_a9f4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
            let zero_68 = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            let flags_clear = if zero_68 {
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                c.g[V0][0] == 0u64
            };
            if flags_clear {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.cycles += 10;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                call_ptr(c, 0x0031_7670, 0x001f_aa30)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_aa3c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let frame_found = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !frame_found {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039_c870, 0x001f_aa58)?;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_aa68)?;
            }
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use crate::game::bx::bxstringctor::tests::{base_3, slot_2};
    use crate::game::fe::fememcardcallbacks::tests::hash_screen;

    #[test]
    fn ov_screen_pause_freeride_shutdown_matches() {
        check(
            ee::s04::f_001f88e8__generated,
            ov_screen_pause_freeride_shutdown,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1f88);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x50, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x10, screen - 0x18);
                c.bus.write32(DATA + 0xc4, r.next_u32());
                hash_screen(c, screen, node, 0x004a_20d0);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot_2(c, DATA + 0x3000, 0xc0, 1);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_rival_pre_on_widget_action_matches() {
        check(
            ee::s05::f_001fdb88__generated,
            ov_screen_rival_pre_on_widget_action,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1fdb);
                let world = c.bus.read32(GAME_GP.wrapping_sub(0x848));
                let game = c.bus.read32(world + 0x84);
                c.bus.write32(game + 0x200, DATA + 0x5000);
                c.bus.write32(DATA + 0x5008, 0);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { 1 + seed as u64 };
                c.g[A2][0] = if seed % 2 == 0 { 5 } else { (seed % 7) as u64 };
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    /// Gate word for the `UIList_DeleteAll` fake below (in the machine's RAM,
    /// in this test's own area).
    const OC990_LIST_GATE: u32 = DATA + 0x20_0000;
    const OC990_LIST_MAGIC: u32 = 0x990c_0001;
    static OC990_PREV_DELETE_ALL: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// `UIList_DeleteAll` with its list walk faked when the gate is set (the
    /// real walk runs away on empty test memory); otherwise the body it replaced.
    fn oc990_list_delete_all_fake(c: &mut Cpu) {
        if c.bus.read32(OC990_LIST_GATE) != OC990_LIST_MAGIC {
            return fakes::fall_back(c, &OC990_PREV_DELETE_ALL, ee::s13::f_00397948__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn ov_screen_in_game_error_handle_select_matches() {
        check(
            ee::s04::f_001fb2b8__generated,
            ov_screen_in_game_error_handle_select,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1fb2, 0);
                let world = DATA + 0x1000;
                let game = DATA + 0x1200;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x200, DATA + 0x5000);
                c.bus.write32(game + 0xc, DATA + 0x6000);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), 1 + seed % 3);
                let item = DATA + 0x800;
                c.bus
                    .write32(item + 0x18, [0, 1, 4, 2][(seed % 4) as usize]);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(item);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_be_options_update_matches() {
        check(
            ee::s04::f_001fa9e0__generated,
            ov_screen_be_options_update,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1fa9);
                let table = DATA + 0x4000;
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 5 == 0 { 0 } else { table },
                );
                c.bus
                    .write32(table + 0x68, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(table + 0x64, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x40, DATA + 0x5000);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                c.g[A0][0] = sx(DATA);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }
}
