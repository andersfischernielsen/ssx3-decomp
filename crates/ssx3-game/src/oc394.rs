//! Staging module oc394 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc394`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0024_64e8,
    c_game_load_state_setup_qp_widget,
    "cGameLoadState_SetupQPWidget",
)];

// ---------------------------------------------------------------------------
// cGameLoadState_SetupQPWidget (0x2464e8)
// ---------------------------------------------------------------------------

/// The widget's id (the `StrHash` of its name).
pub(crate) const QPW_WIDGET_ID: u32 = 0x38;
/// The widget's vtable pointer (slot 9 at +0x48: `SetVisible(bool)`-like).
pub(crate) const QPW_WIDGET_VT: u32 = 0x08;
/// `cBENewRaceInterface_newRace.cGameMode` (a signed byte).
#[cfg(test)]
pub(crate) const QPW_GAME_MODE: u32 = 0x0053_5bc8 + 0x48;
/// The load state's hint record (+0x48) and its texture handle (+0x10).
pub(crate) const QPW_STATE_HINT: u32 = 0x48;
/// The game-mode jump tables (.rodata).
pub(crate) const QPW_MODE_TABLE_1: u32 = 0x0047_db70;
pub(crate) const QPW_MODE_TABLE_2: u32 = 0x0047_db90;

const QPW_HASH: u32 = 0x0031_7670;
const QPW_GET_BE: u32 = 0x0014_dc80;
const QPW_GET_INTERFACE: u32 = 0x0014_dd58;
const QPW_SET_WIDE: u32 = 0x003a_0e90;
const QPW_SPRINTF: u32 = 0x002c_26d0;
const QPW_STRCPY: u32 = 0x002c_2508;
const QPW_GET_TRACK: u32 = 0x0014_4be0;
const QPW_GET_HUMAN_RIDER: u32 = 0x0014_6e98;
const QPW_START: u32 = 0x0024_64e8;
const QPW_END: u32 = 0x0024_6cf4;

/// `sp + off` as a register value.
fn qpw_sp(c: &Cpu, off: u32) -> u64 {
    sx(sp(c).wrapping_add(off))
}

/// `s0 = widget id; a0 = key; s0 == StrHash(key)?` (the `IsId` test the
/// function repeats for every widget name), returning to `ret`.
fn qpw_is_id(c: &mut Cpu, key: u32, ret: u32) -> Result<bool, Unwound> {
    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
    c.g[A0][0] = sx(key);
    call_ptr(c, QPW_HASH, ret)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// The locale string of the key in `a0`: `locale = world->locale` (from
/// `world`), `locale->vt[4](locale + delta, StrHash(key))` (0x20/0x24).
fn qpw_locale_get(c: &mut Cpu, world: u32, ret_hash: u32, ret_get: u32) -> Flow {
    c.g[S0][0] = c.lw(world.wrapping_add(0x8c));
    c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(4));
    c.g[V0][0] = c.lh(r32(c, S1).wrapping_add(0x20));
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    call_ptr(c, QPW_HASH, ret_hash)?;
    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x24));
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    let f = r32(c, V1);
    call_ptr(c, f, ret_get)
}

/// The epilogue from 0x246cd4 (`lq s1`; `s0` is already restored).
fn qpw_exit_cd4(c: &mut Cpu) {
    c.cycles += 1;
    let frame = Frame { size: 0x380 };
    frame.restore(c, S1, 0x360);
    qpw_exit_cd8(c);
}

/// The epilogue from 0x246cd8 (`s0`, `s1` restored).
fn qpw_exit_cd8(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame { size: 0x380 };
    frame.restore(c, S2, 0x350);
    frame.restore(c, S3, 0x340);
    frame.restore(c, S4, 0x330);
    frame.restore(c, S5, 0x320);
    frame.restore_ra(c, 0x310);
    frame.close(c);
}

/// `lq s0` and the epilogue (the common exit after the last call).
fn qpw_exit(c: &mut Cpu) {
    Frame { size: 0x380 }.restore(c, S0, 0x370);
    qpw_exit_cd4(c);
}

/// Show the widget (0x246a2c): `widget->vt[9](widget + delta, 1)`; `v0`
/// holds the vtable.
fn qpw_activate(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    let vt = r32(c, V0);
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh(vt.wrapping_add(0x48));
    c.g[V1][0] = c.lw(vt.wrapping_add(0x4c));
    let f = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
    call_ptr(c, f, 0x0024_6a40)?;
    qpw_exit(c);
    Ok(())
}

/// The QP (track peak) line (0x246694): `sprintf(buf, "%s %s", Loc(peak
/// name), track name)` for peaks 0..2, then `SetWideString(widget, buf)`.
fn qpw_peak_line(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[A0][0] = qpw_sp(c, 0xa0);
    c.g[A1][0] = 0;
    c.g[S4][0] = c.g[A0][0];
    c.g[A2][0] = 0xc8;
    call_ptr(c, 0x0041_6210, 0x0024_66a8)?;
    c.g[V1][0] = 0x004a_0000;
    c.g[A0][0] = c.g[S3][0];
    c.g[S2][0] = sx(0x004a_2bd8);
    call_ptr(c, 0x0014_4c98, 0x0024_66b8)?;
    let peak = c.g[V0][0];
    c.g[V1][0] = peak;
    c.g[V0][0] = ((peak as i64) < 2) as u64;
    // The format block's return addresses, or no line.
    let line: Option<[u32; 4]> = if peak == 1 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        c.g[A0][0] = sx(0x0047_ba68);
        c.cycles += 19;
        Some([0x0024_671c, 0x0024_672c, 0x0024_6738, 0x0024_674c])
    } else {
        c.cycles += 2;
        let below2 = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if below2 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
            if peak == 0 {
                c.cycles += 3 + 19;
                c.g[A0][0] = sx(0x0047_ba58);
                Some([0x0024_671c, 0x0024_672c, 0x0024_6738, 0x0024_674c])
            } else {
                c.cycles += 2;
                None
            }
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S5][0];
            if peak == 2 {
                c.cycles += 21;
                c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                c.g[A0][0] = sx(0x0047_ba78);
                Some([0x0024_6774, 0x0024_6784, 0x0024_6790, 0x0024_67a4])
            } else {
                c.cycles += 2;
                None
            }
        }
    };
    if let Some(rets) = line {
        let world = r32(c, V0);
        qpw_locale_get(c, world, rets[0], rets[1])?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0014_4bf0, rets[2])?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002c_27c0, rets[3])?;
    }
    c.g[A0][0] = c.g[S5][0];
    // 0x2467a8
    c.cycles += 4;
    c.g[A1][0] = c.g[S4][0];
    call_ptr(c, QPW_SET_WIDE, 0x0024_67b0)?;
    qpw_exit(c);
    Ok(())
}

/// One game-mode case of the first switch: (key, return address, last case).
fn qpw_mode_case_1(target: u32) -> Option<(u32, u32, bool)> {
    Some(match target {
        0x0024_67e8 => (0x0047_bb80, 0x0024_67f8, false),
        0x0024_6808 => (0x0047_bb98, 0x0024_6818, false),
        0x0024_6828 => (0x0047_bba8, 0x0024_6838, false),
        0x0024_6848 => (0x0047_bbb8, 0x0024_6858, false),
        0x0024_6868 => (0x0047_bb80, 0x0024_6878, false),
        0x0024_6888 => (0x0047_bbc8, 0x0024_6898, false),
        0x0024_68a8 => (0x0047_bbc8, 0x0024_68b8, true),
        _ => return None,
    })
}

/// One game-mode case of the second switch: `Some(None)` is the empty case.
fn qpw_mode_case_2(target: u32) -> Option<Option<(u32, u32, bool)>> {
    Some(Some(match target {
        0x0024_68ec => (0x004a_2be0, 0x0024_68fc, false),
        0x0024_690c => (0x004a_2be8, 0x0024_691c, false),
        0x0024_692c => (0x004a_2bf0, 0x0024_693c, false),
        0x0024_694c => (0x004a_2bf8, 0x0024_695c, false),
        0x0024_696c => (0x0047_bbd8, 0x0024_697c, false),
        0x0024_698c => (0x0047_bbe8, 0x0024_699c, true),
        0x0024_69a4 => return Some(None),
        _ => return None,
    }))
}

/// A `sprintf(buf, Loc("..."), table[track])` line (0x246a5c / 0x246ad4):
/// the format at `sp+fmt`, the value from the stack table at `sp+table`.
fn qpw_track_table_line(c: &mut Cpu, fmt: u32, from_s4: bool, rets: [u32; 4]) -> Flow {
    c.cycles += 25;
    c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.g[S2][0] = qpw_sp(c, fmt);
    c.g[A0][0] = sx(0x0047_bc30);
    let world = r32(c, V0);
    qpw_locale_get(c, world, rets[0], rets[1])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, QPW_STRCPY, rets[2])?;
    c.g[S0][0] = qpw_sp(c, 0xa0);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, QPW_GET_TRACK, rets[3])?;
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A1][0] = c.g[S2][0];
    let at = if from_s4 {
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        r32(c, V0)
    } else {
        c.g[V1][0] = sx(sp(c).wrapping_add(r32(c, V0)));
        r32(c, V1)
    };
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.lw(at);
    Ok(())
}

/// `void cGameLoadState_SetupQPWidget(cGameLoadState* self, cUIText* widget)`
/// (0x2464e8): fills in one widget of the race load screen by its name hash:
/// the track-peak line, the game-mode and track-location icons (shown via
/// vtable slot 9), the two per-track statistics lines (from two 17-entry
/// tables copied to the stack), the rider's short name, the rider's overall
/// rating line, or the hint texture handle.
pub fn c_game_load_state_setup_qp_widget(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x380);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S5, 0x320);
        c.g[A2][0] = sx(0x0047_bae0);
        frame.save(c, S2, 0x350);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S0, 0x370);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x360);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S3, 0x340);
        c.g[V0][0] = 0; // 0x47bae0 & 7: the aligned copy
        frame.save(c, S4, 0x330);
        frame.save_ra(c, 0x310);

        // tA = table 0x47bae0 (17 words) to sp+0.
        c.cycles += 4;
        c.g[V1][0] = sx(0x0047_bb20);
        c.g[V0][0] = 0x0048_0000;
        c.g[S4][0] = qpw_sp(c, 0x50);
        c.g[S0][0] = 0x0048_0000;
        loop {
            c.cycles += 11;
            let src = r32(c, A2);
            let dst = r32(c, A1);
            c.g[T2][0] = c.ld(src);
            c.g[A0][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[T2][0]);
            c.sd(dst.wrapping_add(8), c.g[A0][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[A2][0] = sx(src.wrapping_add(0x20));
            c.g[A1][0] = sx(dst.wrapping_add(0x20));
            if c.g[A2][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0024_6598);
            }
        }
        c.cycles += 1;
        c.g[A0][0] = sx(0x0047_bb28);
        c.cycles += 5;
        c.g[T1][0] = c.lw(r32(c, A2));
        c.g[V1][0] = c.g[S4][0];
        c.g[V0][0] = 0; // 0x47bb28 & 7
        c.sw(r32(c, A1), c.g[T1][0]);

        // tB = table 0x47bb28 (17 words) to sp+0x50.
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x40));
        loop {
            c.cycles += 11;
            let src = r32(c, A0);
            let dst = r32(c, V1);
            c.g[A3][0] = c.ld(src);
            c.g[T0][0] = c.ld(src.wrapping_add(8));
            c.g[T1][0] = c.ld(src.wrapping_add(0x10));
            c.g[T2][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A3][0]);
            c.sd(dst.wrapping_add(8), c.g[T0][0]);
            c.sd(dst.wrapping_add(0x10), c.g[T1][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T2][0]);
            c.g[A0][0] = sx(src.wrapping_add(0x20));
            c.g[V1][0] = sx(dst.wrapping_add(0x20));
            if c.g[A0][0] == c.g[V0][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0024_6638);
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw(r32(c, A0));
        c.sw(r32(c, V1), c.g[V0][0]);

        // iface0 = BELibrary_GetInstance()->GetInterface(0) (s3).
        call_ptr(c, QPW_GET_BE, 0x0024_6670)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, QPW_GET_INTERFACE, 0x0024_667c)?;
        c.g[A0][0] = sx(0x0047_bb70);
        c.g[S3][0] = c.g[V0][0];
        c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
        call_ptr(c, QPW_HASH, 0x0024_668c)?;
        let is_peak = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x0053_0000;
        if is_peak {
            return qpw_peak_line(c);
        }

        // First game-mode switch (0x2467b8).
        c.cycles += 6;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5bc8));
        c.g[A0][0] = c.lb(r32(c, V0).wrapping_add(0x48));
        let in_range = c.g[A0][0] < 7;
        c.g[V1][0] = in_range as u64;
        c.g[V0][0] = 0x0048_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(QPW_MODE_TABLE_1);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            let Some((key, ret_at, last)) = qpw_mode_case_1(target) else {
                c.jump_out(target, QPW_START, QPW_END);
                return Err(Unwound);
            };
            c.cycles += 6;
            let hit = qpw_is_id(c, key, ret_at)?;
            if last {
                if hit {
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5bc8));
            } else {
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5bc8));
                if hit {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x5bc8));
        }

        // Second game-mode switch (0x2468c4).
        c.cycles += 4;
        c.g[A0][0] = c.lb(r32(c, V0).wrapping_add(0x48));
        let in_range = c.g[A0][0] < 8;
        c.g[V1][0] = in_range as u64;
        c.g[V0][0] = 0x0048_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(QPW_MODE_TABLE_2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            let Some(case) = qpw_mode_case_2(target) else {
                c.jump_out(target, QPW_START, QPW_END);
                return Err(Unwound);
            };
            if let Some((key, ret_at, last)) = case {
                c.cycles += 6;
                if qpw_is_id(c, key, ret_at)? {
                    if !last {
                        c.cycles += 2;
                    }
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
            }
        }

        // Track-location icons (0x2469a4).
        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, QPW_GET_TRACK, 0x0024_69ac)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0014_4d18, 0x0024_69b8)?;
        let kind = c.g[V0][0];
        c.g[V1][0] = kind;
        c.g[V0][0] = ((kind as i64) < 2) as u64;
        // The location key to test, or None to go on with the line widgets
        // (`true`: s0 already loaded, from 0x246a4c).
        let probe: Result<u32, bool> = if kind == 1 {
            c.cycles += 4;
            c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
            Ok(0x0047_bbf8)
        } else {
            c.cycles += 2;
            let below2 = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if below2 {
                if kind == 0 {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
                    Ok(0x004a_2c00)
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x0048_0000;
                    Err(false)
                }
            } else {
                c.g[A0][0] = 0x0048_0000;
                if kind == 2 {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
                    Ok(0x0047_bc08)
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
                    Err(true)
                }
            }
        };
        let s0_loaded = match probe {
            Ok(key) => {
                c.g[A0][0] = sx(key);
                c.cycles += 4;
                call_ptr(c, QPW_HASH, 0x0024_6a20)?;
                let hit = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = 0x0048_0000;
                if hit {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_VT));
                    return qpw_activate(c);
                }
                false
            }
            Err(loaded) => loaded,
        };
        if !s0_loaded {
            c.cycles += 1;
            c.g[S0][0] = c.lw(r32(c, S5).wrapping_add(QPW_WIDGET_ID));
        }

        // The statistics line from tB (0x246a4c).
        c.cycles += 4;
        c.g[A0][0] = sx(0x0047_bc18);
        call_ptr(c, QPW_HASH, 0x0024_6a54)?;
        if c.g[S0][0] == c.g[V0][0] {
            qpw_track_table_line(
                c,
                0x170,
                true,
                [0x0024_6a80, 0x0024_6a90, 0x0024_6a9c, 0x0024_6aa8],
            )?;
            return qpw_sprintf_and_set(c);
        }
        c.g[A0][0] = 0x0048_0000;

        // The statistics line from tA (0x246ac0).
        c.cycles += 5;
        if qpw_is_id(c, 0x0047_bc48, 0x0024_6acc)? {
            qpw_track_table_line(
                c,
                0x240,
                false,
                [0x0024_6af8, 0x0024_6b08, 0x0024_6b14, 0x0024_6b20],
            )?;
            return qpw_sprintf_and_set(c);
        }
        c.g[A0][0] = 0x0048_0000;

        // The rider's short name (0x246b38).
        c.cycles += 5;
        let hit = qpw_is_id(c, 0x0047_bc60, 0x0024_6b44)?;
        c.g[A0][0] = 0x0048_0000;
        if hit {
            c.cycles += 22;
            call_ptr(c, QPW_GET_BE, 0x0024_6b54)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6b60)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, QPW_GET_BE, 0x0024_6b68)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 2;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6b74)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            call_ptr(c, QPW_GET_HUMAN_RIDER, 0x0024_6b84)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x0024_6b90)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0024_6b9c)?;
            qpw_exit(c);
            return Ok(());
        }

        // The rider's overall rating line (0x246ba4).
        c.cycles += 5;
        let hit = qpw_is_id(c, 0x0047_bc70, 0x0024_6bb0)?;
        c.g[A0][0] = 0x004a_0000;
        if hit {
            c.cycles += 50;
            call_ptr(c, QPW_GET_BE, 0x0024_6bc0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6bcc)?;
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, QPW_GET_BE, 0x0024_6bd4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 3;
            call_ptr(c, QPW_GET_INTERFACE, 0x0024_6be0)?;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
            c.g[S2][0] = c.g[V0][0];
            c.g[S3][0] = qpw_sp(c, 0x170);
            c.g[A0][0] = sx(0x0047_bc80);
            let world = r32(c, V1);
            qpw_locale_get(c, world, 0x0024_6c08, 0x0024_6c18)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, QPW_STRCPY, 0x0024_6c24)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, QPW_GET_HUMAN_RIDER, 0x0024_6c30)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_8950, 0x0024_6c3c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, QPW_GET_HUMAN_RIDER, 0x0024_6c4c)?;
            // rating = riders[idx].flags & 2 ? 1 : rating (0x5308b8, 12 bytes each).
            c.g[V1][0] = 0xc;
            c.g[A3][0] = 0x0053_0000;
            c.mult(0, V0, V0, V1);
            c.g[A3][0] = sx(0x0053_08b8);
            c.g[S0][0] = qpw_sp(c, 0xa0);
            c.g[A2][0] = 1;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[V1][0] = sx(r32(c, V1) >> 1);
            c.g[V1][0] &= 1;
            if c.g[V1][0] == 0 {
                c.g[A2][0] = c.g[S1][0];
            }
            return qpw_sprintf_and_set(c);
        }

        // The hint texture (0x246c9c).
        c.cycles += 5;
        let hit = qpw_is_id(c, 0x004a_2c08, 0x0024_6ca8)?;
        frame.restore(c, S0, 0x370);
        if !hit {
            qpw_exit_cd4(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(QPW_STATE_HINT));
        frame.restore(c, S1, 0x360);
        if c.g[A0][0] == 0 {
            qpw_exit_cd8(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
        if c.g[A0][0] == 0 {
            qpw_exit_cd8(c);
            return Ok(());
        }
        c.cycles += 3;
        let widget = r32(c, S5);
        c.sw(widget.wrapping_add(0x78), c.g[A0][0]);
        c.sw(widget.wrapping_add(0x7c), 0);
        qpw_exit(c);
        Ok(())
    })
}

/// 0x246c80: `UStr_Sprintf(a0, a1, a2)`, then `SetWideString(widget, s0)`.
fn qpw_sprintf_and_set(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    call_ptr(c, QPW_SPRINTF, 0x0024_6c88)?;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, QPW_SET_WIDE, 0x0024_6c94)?;
    qpw_exit(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{put_words, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate and their controlled results.
    const OC394_CTRL: u32 = DATA + 0xf_0000;
    const OC394_MAGIC: u32 = 0x0c39_4a11;
    /// `BENewRaceInt_GetTrackPeak`, `..GetTrackField60`, `..GetTrack`,
    /// `BENewPlayerInt_GetHumanRider` results.
    const OC394_PEAK: u32 = OC394_CTRL + 4;
    const OC394_KIND: u32 = OC394_CTRL + 8;
    const OC394_TRACK: u32 = OC394_CTRL + 0xc;
    const OC394_RIDER: u32 = OC394_CTRL + 0x10;

    /// The callees: logged by `fakes::fake`; `StrHash` returns its key's
    /// address (so a widget id equal to a key address matches that key) and
    /// the interface getters the results the fixture chose.
    fn oc394_fake_body(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0];
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        let forced = |c: &mut Cpu, at: u32| sx(c.bus.read32(at));
        match target {
            QPW_HASH => c.g[V0][0] = a0,
            0x0014_4c98 => c.g[V0][0] = forced(c, OC394_PEAK),
            0x0014_4d18 => c.g[V0][0] = forced(c, OC394_KIND),
            QPW_GET_TRACK => c.g[V0][0] = forced(c, OC394_TRACK),
            QPW_GET_HUMAN_RIDER => c.g[V0][0] = forced(c, OC394_RIDER),
            _ => {}
        }
    }

    macro_rules! oc394_gated {
        ($name:ident, $prev:ident, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC394_CTRL) != OC394_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                oc394_fake_body(c, target);
            }
        };
    }

    oc394_gated!(
        oc394_fake_14dc80,
        OC394_PREV_14DC80,
        ee::s01::f_0014dc80__generated
    );
    oc394_gated!(
        oc394_fake_14dd58,
        OC394_PREV_14DD58,
        ee::s01::f_0014dd58__generated
    );
    oc394_gated!(
        oc394_fake_317670,
        OC394_PREV_317670,
        ee::s10::f_00317670__generated
    );
    oc394_gated!(
        oc394_fake_416210,
        OC394_PREV_416210,
        ee::s15::f_00416210__generated
    );
    oc394_gated!(
        oc394_fake_144c98,
        OC394_PREV_144C98,
        ee::s01::f_00144c98__generated
    );
    oc394_gated!(
        oc394_fake_144bf0,
        OC394_PREV_144BF0,
        ee::s01::f_00144bf0__generated
    );
    oc394_gated!(
        oc394_fake_2c27c0,
        OC394_PREV_2C27C0,
        ee::s08::f_002c27c0__generated
    );
    oc394_gated!(
        oc394_fake_3a0e90,
        OC394_PREV_3A0E90,
        ee::s13::f_003a0e90__generated
    );
    oc394_gated!(
        oc394_fake_144be0,
        OC394_PREV_144BE0,
        ee::s01::f_00144be0__generated
    );
    oc394_gated!(
        oc394_fake_144d18,
        OC394_PREV_144D18,
        ee::s01::f_00144d18__generated
    );
    oc394_gated!(
        oc394_fake_2c2508,
        OC394_PREV_2C2508,
        ee::s08::f_002c2508__generated
    );
    oc394_gated!(
        oc394_fake_146e98,
        OC394_PREV_146E98,
        ee::s01::f_00146e98__generated
    );
    oc394_gated!(
        oc394_fake_14ee58,
        OC394_PREV_14EE58,
        ee::s01::f_0014ee58__generated
    );
    oc394_gated!(
        oc394_fake_148950,
        OC394_PREV_148950,
        ee::s01::f_00148950__generated
    );
    oc394_gated!(
        oc394_fake_2c26d0,
        OC394_PREV_2C26D0,
        ee::s08::f_002c26d0__generated
    );
    oc394_gated!(
        oc394_fake_3a0c70,
        OC394_PREV_3A0C70,
        ee::s13::f_003a0c70__generated
    );

    fn oc394_install() {
        use fakes::hook;
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc394_fake_14dc80,
            &OC394_PREV_14DC80,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc394_fake_14dd58,
            &OC394_PREV_14DD58,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc394_fake_317670,
            &OC394_PREV_317670,
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416210),
            oc394_fake_416210,
            &OC394_PREV_416210,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144C98),
            oc394_fake_144c98,
            &OC394_PREV_144C98,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BF0),
            oc394_fake_144bf0,
            &OC394_PREV_144BF0,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C27C0),
            oc394_fake_2c27c0,
            &OC394_PREV_2C27C0,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc394_fake_3a0e90,
            &OC394_PREV_3A0E90,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BE0),
            oc394_fake_144be0,
            &OC394_PREV_144BE0,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144D18),
            oc394_fake_144d18,
            &OC394_PREV_144D18,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            oc394_fake_2c2508,
            &OC394_PREV_2C2508,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc394_fake_146e98,
            &OC394_PREV_146E98,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EE58),
            oc394_fake_14ee58,
            &OC394_PREV_14EE58,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00148950),
            oc394_fake_148950,
            &OC394_PREV_148950,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc394_fake_2c26d0,
            &OC394_PREV_2C26D0,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C70),
            oc394_fake_3a0c70,
            &OC394_PREV_3A0C70,
        );
    }

    /// Widget names (key addresses; the fake `StrHash` returns the address)
    /// and the game mode / location kind that reaches each one's test.
    const OC394_KEYS: &[(u32, &[i32])] = &[
        (0x0047_bb70, &[]),     // peak line
        (0x0047_bb80, &[0, 4]), // mode icons, first switch
        (0x0047_bb98, &[1]),
        (0x0047_bba8, &[2]),
        (0x0047_bbb8, &[3]),
        (0x0047_bbc8, &[5, 6]),
        (0x004a_2be0, &[0]), // second switch
        (0x004a_2be8, &[1]),
        (0x004a_2bf0, &[2]),
        (0x004a_2bf8, &[3]),
        (0x0047_bbd8, &[5]),
        (0x0047_bbe8, &[6]),
        (0x004a_2c00, &[]), // location icons (kind 0, 1, 2)
        (0x0047_bbf8, &[]),
        (0x0047_bc08, &[]),
        (0x0047_bc18, &[]), // statistics lines
        (0x0047_bc48, &[]),
        (0x0047_bc60, &[]), // rider name
        (0x0047_bc70, &[]), // rating line
        (0x004a_2c08, &[]), // hint texture
        (0x0012_3400, &[]), // no widget of this function
    ];

    fn oc394_setup(c: &mut Cpu, seed: u32) {
        let mut r: Rng = machine(c, seed, 0x0024_64e8, 14);
        c.pc = 0x0024_64e8;
        c.bus.write32(OC394_CTRL, OC394_MAGIC);
        let pick = |r: &mut Rng, n: u32| r.next_u32() % n;

        // The jump tables and the two 17-word statistics tables.
        put_words(
            c,
            QPW_MODE_TABLE_1,
            &[
                0x2467e8, 0x246808, 0x246828, 0x246848, 0x246868, 0x246888, 0x2468a8,
            ],
        );
        put_words(
            c,
            QPW_MODE_TABLE_2,
            &[
                0x2468ec, 0x24690c, 0x24692c, 0x24694c, 0x2469a4, 0x24696c, 0x24698c, 0x2469a4,
            ],
        );
        for i in 0..18 {
            c.bus.write32(0x0047_bae0 + 4 * i, r.next_u32());
            c.bus.write32(0x0047_bb28 + 4 * i, r.next_u32());
        }
        // Rider records (12 bytes each; bit 1 of the first word).
        for i in 0..8 {
            c.bus.write32(0x0053_08b8 + 0xc * i, r.next_u32());
        }

        // The widget, its vtable (slot 9) and the load state.
        let widget = DATA + 0x1000;
        let (key, modes) = OC394_KEYS[(seed as usize) % OC394_KEYS.len()];
        c.bus.write32(widget + QPW_WIDGET_ID, key);
        let vt = DATA + 0x2000;
        c.bus.write32(widget + QPW_WIDGET_VT, vt);
        c.bus.write16(vt + 0x48, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(vt + 0x4c, fakes::FAKE + 0x90);
        let state = DATA + 0x3000;
        let hint = DATA + 0x3800;
        let hint_ptr = if pick(&mut r, 4) == 0 { 0 } else { hint };
        c.bus.write32(state + QPW_STATE_HINT, hint_ptr);
        let tex = if pick(&mut r, 3) == 0 {
            0
        } else {
            r.next_u32()
        };
        c.bus.write32(hint + 0x10, tex);

        // The world's locale object and its lookup slot (vt+0x20/0x24).
        let world = DATA + 0x4000;
        c.bus.write32(GAME_GP_WORLD, world);
        let locale = DATA + 0x5000;
        c.bus.write32(world + 0x8c, locale);
        let lvt = DATA + 0x6000;
        c.bus.write32(locale + 4, lvt);
        c.bus.write16(lvt + 0x20, (r.next_u32() & 0x7c) as u16);
        c.bus.write32(lvt + 0x24, fakes::FAKE + 0xa0);

        // The game mode: the one the key wants most of the time.
        let mode = if !modes.is_empty() && pick(&mut r, 4) != 0 {
            modes[pick(&mut r, modes.len() as u32) as usize]
        } else {
            pick(&mut r, 10) as i32 - 1
        };
        c.bus.write8(QPW_GAME_MODE, mode as u8);
        // Peak / location kind: -1..3, mostly matching the key's case.
        c.bus
            .write32(OC394_PEAK, (pick(&mut r, 5) as i32 - 1) as u32);
        let kind = match key {
            0x004a_2c00 => 0,
            0x0047_bbf8 => 1,
            0x0047_bc08 => 2,
            _ => pick(&mut r, 5) as i32 - 1,
        };
        let kind = if pick(&mut r, 5) == 0 {
            pick(&mut r, 5) as i32 - 1
        } else {
            kind
        };
        c.bus.write32(OC394_KIND, kind as u32);
        c.bus.write32(OC394_TRACK, pick(&mut r, 17));
        c.bus.write32(OC394_RIDER, pick(&mut r, 8));

        c.g[A0][0] = sx(state);
        c.g[A1][0] = sx(widget);
    }

    /// `gp - 0x848`: the world pointer.
    const GAME_GP_WORLD: u32 = crate::difftest::GAME_GP - 0x848;

    /// `cGameLoadState_SetupQPWidget`: every widget name, each game mode
    /// (-1..8, both jump tables), peak and location kinds -1..3, hint
    /// pointers null / empty / set, callee unwinds.
    #[test]
    fn c_game_load_state_setup_qp_widget_matches() {
        oc394_install();
        covered(
            ee::s06::f_002464e8__generated,
            c_game_load_state_setup_qp_widget,
            420,
            &oc394_setup,
        );
    }
}
