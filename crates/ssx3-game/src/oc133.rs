//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path FE/UI/AI/graphics helpers, each with a
//! differential test against the generated body.
//!
//! Most of these bodies are thin wrappers over already-ported callees; the
//! tests build the objects (and, where a virtual call is involved, the fakes
//! from [`crate::difftest::fakes`]) so the callees take their short paths.

use crate::abi;
use crate::call::{call_ptr, guest, Flow};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0019_4d18, cfe_state_main_menu_on_widget_created, "cFEStateMainMenu_OnWidgetCreated"),
    (0x0019_b458, cfe_state_char_equip_remove_list_item, "cFEStateCharEquip_RemoveListItem"),
    (0x001a_3160, fescreen_sel_multi_on_widget_action, "FEScreenSelMulti_OnWidgetAction"),(0x001e_32c8, message_center_remove_messages_of_group, "MessageCenter_RemoveMessagesOfGroup"),
    (0x0022_92e0, crowd_man2d_reset, "CrowdMan2d_Reset"),
    (0x0023_7b28, input_sync_pack_sample, "InputSync_PackSample"),
    (0x0023_f438, mc_manager_state0f_save, "MCManager_State0F_Save"),
    (0x0024_fbc0, shell_renderer_menu_dtor, "ShellRendererMenu_dtor"),
    (0x0024_fd00, shadow_menu_dtor, "ShadowMenu_dtor"),
    (0x002e_4578, ctransition_man_stop, "cTransitionMan_Stop"),
    (0x002e_aa28, ccutscene_transition_man_save_viewport, "cCutSceneTransitionMan_SaveViewport"),
    (0x0030_7f58, wscript_process_goto_task, "WScriptProcess_GotoTask"),
    (0x0030_8988, wscript_man_reset, "WScriptMan_Reset"),
    (0x0031_4f30, game_anim_map_register_bank, "GameAnimMap_RegisterBank"),
    (0x0032_fb48, coll_sphere_ctor, "CollSphere_ctor"),
    (0x0039_e9d8, ui_state_mgr_dtor, "UIStateMgr_dtor"),
    (0x0039_fe00, ui_object_load, "UIObject_Load"),
];

/// `a + off`.
#[inline(always)]
fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `addiu sp, sp, -n`.
#[inline(always)]
fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}

/// `addiu sp, sp, n; jr ra` (`ra` already reloaded by the caller).
#[inline(always)]
fn close(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(n));
    abi::ret(c);
}

/// The shared frame epilogue: `ld ra, off(sp)`, close the frame and return.
#[inline(always)]
fn epilogue(c: &mut Cpu, off: u32, size: u32) {
    c.g[RA][0] = c.ld(at(c.g[SP][0] as u32, off));
    close(c, size);
}

// ---------------------------------------------------------------------------
// FE lists.

/// `int cFEStateCharEquip_RemoveListItem(cFEStateCharEquip* self, int idx)`
/// (0x19b458): drop entry `idx` from the array at +0x124 (`count` at +0x958),
/// zero the vacated tail slot and clamp the list view (`top` at list+0x98,
/// `sel` at list+0x95). Returns 1, or 0 when `idx` is out of range.
pub fn cfe_state_char_equip_remove_list_item(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0];
    c.g[T3][0] = this;
    let count = Addr(this as u32).off(0x958).lw(c);
    c.g[V1][0] = count;
    let in_range = (c.g[A1][0] as i64) < (count as i64);
    c.g[V0][0] = in_range as u64;
    c.g[V0][0] = sx((count as u32).wrapping_sub(1));
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let shift = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
    c.g[V0][0] = shift as u64;
    let items = sx((this as u32).wrapping_add(0x124));
    c.g[A2][0] = items;
    if shift {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(items as u32));
        loop {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
            let n = Addr(this as u32).off(0x958).lw(c);
            c.g[V0][0] = n;
            c.g[V0][0] = sx((n as u32).wrapping_sub(1));
            let more = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_b488);
            }
        }
    }
    c.cycles += 27;
    c.g[A0][0] = Addr(this as u32).off(0x958).lw(c);
    c.g[T1][0] = u64::MAX;
    c.g[V0][0] = 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    Addr(this as u32).off(0x958).sw(c, c.g[A0][0]);
    c.g[V1][0] = sx((items as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[A3][0] = Addr(this as u32).off(0x5c).lw(c);
    c.g[A2][0] = Addr(this as u32).off(0x958).lw(c);
    c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x98).lbu(c);
    c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_sub(6));
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(0x95).lbu(c);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(6));
    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A0][0] = c.g[T2][0];
    }
    c.g[A2][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if c.g[A2][0] == 0 {
        c.g[A1][0] = c.g[T0][0];
    }
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[T1][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    Addr(c.g[A3][0] as u32).off(0x98).sb(c, c.g[A0][0]);
    if c.g[T1][0] == 0 {
        c.g[A1][0] = 0;
    }
    c.g[V1][0] = Addr(this as u32).off(0x5c).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(c.g[V1][0] as u32).off(0x95).sb(c, c.g[A1][0]);
    c.pc = ra;
}

/// `int BERewardInt_FindRequirementTier(int ofs, int a, int b)` (0x15a3b8):
/// scan the `[3][4][5]` requirement table at 0x45aad8 (two bytes per entry,
/// 0xa apart per middle index) for the first entry whose `a` matches — and
/// whose `b` matches, or whose `a` is one of 6..10 — returning the middle
/// index, else -1.
pub fn be_reward_int_find_requirement_tier(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x460000;
    c.g[T1][0] = 0;
    c.g[T8][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5528));
    c.g[T7][0] = 0xa;
    c.g[T6][0] = 6;
    c.g[T5][0] = 7;
    c.g[T4][0] = 8;
    c.g[T3][0] = 9;
    c.g[T2][0] = 0xb;
    c.g[V0][0] = 0x28;
    loop {
        c.cycles += 2;
        c.g[T0][0] = 0;
        c.mult(0, 4, 9, 2);
        loop {
            c.cycles += 2;
            c.g[A3][0] = 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T8][0] as u32));
            loop {
                c.cycles += 3;
                let mut hit = Addr(c.g[V1][0] as u32).lb(c) == c.g[A1][0];
                if hit {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
                    hit = c.g[V0][0] == c.g[A2][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T6][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T5][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T4][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T3][0];
                }
                if !hit {
                    c.cycles += 2;
                    hit = c.g[A1][0] == c.g[T7][0];
                }
                if !hit {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    hit = c.g[A1][0] == c.g[T2][0];
                }
                if hit {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[T0][0];
                    c.pc = ra;
                    return;
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 5i64) as u64;
                let again = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_a3f0);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 4;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T0][0] as i64) < 4i64) as u64;
            let again = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa));
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_a3e8);
                }
                break;
            }
            c.cycles += 4;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T1][0] as i64) < 3i64) as u64;
            let again = c.g[V0][0] != 0;
            c.g[V0][0] = 0x28;
            if !again {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = u64::MAX;
                c.pc = ra;
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_a3e0);
            }
        }
    }
}

/// `void* Bolts_FindTable2EntriesByFlag(Bolts* self, int index, int kind,
/// int want, int* out)` (0x14db40): walk the `table2[index]` entries (`kind`
/// at +2, nonzero flag at +1) and return the first entry whose kind and flag
/// match, counting every match in `*out`.
pub fn bolts_find_table2_entries_by_flag(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    Addr(c.g[T0][0] as u32).sw(c, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20c));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T2][0] = Addr(c.g[V1][0] as u32).lw(c);
    let none = (c.g[T2][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
    if none {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = ra;
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[T3][0] = 0;
    if empty {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = c.g[T3][0];
        c.pc = ra;
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = c.g[V1][0];
    c.mult(0, 10, 9, 2);
    loop {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[T3][0];
                c.pc = ra;
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            if c.g[V0][0] != c.g[A3][0] {
                c.cycles += 2;
                if c.g[T3][0] != 0 {
                    let ra = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[T3][0];
                    c.pc = ra;
                    return;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                if c.g[T3][0] == 0 {
                    c.g[T3][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(c.g[T0][0] as u32).sw(c, c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            }
        }
        c.cycles += 6;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        c.g[V1][0] = Addr(c.g[T4][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xc));
        if !more {
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = c.g[T3][0];
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_db88);
        }
    }
}

// ---------------------------------------------------------------------------
// Input.

/// `int InputSync_PackSample(SampleSrc* src, unsigned char* dst, u8* mask)`
/// (0x237b28): copy the first sample byte to `dst[0]`, zero `dst[1..4]`, then
/// pack the `src->count` floats at `src+4` as `(int)(v * 255)` — skipping
/// zeros — into `dst+4`, recording presence bits in the `dst+1` mask.
/// Returns the packed byte count + 4.
pub fn input_sync_pack_sample(c: &mut Cpu) {
    c.cycles += 6;
    let src = c.g[A1][0];
    c.g[T3][0] = src;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    c.g[V0][0] = Addr(src as u32).lbu(c);
    c.g[A3][0] = 2;
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(3));
    Addr(c.g[A2][0] as u32).sb(c, c.g[V0][0]);
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sb(c, 0);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_7b40);
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
    c.g[T2][0] = 0;
    let no_samples = (c.g[V0][0] as i64) <= 0;
    c.g[T0][0] = 0;
    if !no_samples {
        c.cycles += 7;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(1, 1);
        c.g[T4][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
        c.g[T5][0] = 1;
        c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_add(4));
        loop {
            c.cycles += 7;
            c.lwc1(0, c.g[T1][0] as u32);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(2, 0);
            c.mfc1(2, 2);
            c.g[A3][0] = c.g[V0][0] & 0xff;
            let zero = c.g[A3][0] == 0;
            c.g[A0][0] = sx(((c.g[T2][0] as u32) as i32 >> 3) as u32);
            if !zero {
                c.cycles += 9;
                c.g[V1][0] = c.g[T2][0] & 7;
                c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[T5][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                Addr(c.g[A1][0] as u32).sb(c, c.g[A3][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_7b88);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(4));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Collision objects.

/// `CollSphere* CollSphere_ctor(CollSphere* self, const Vec3* pos, int flags,
/// float radius)` (0x32fb48): set the vtable and `state`, copy the centre and
/// radius, and build the `pos ± r` AABBs (`w = 1`).
pub fn coll_sphere_ctor(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x490000;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x19b0));
    open(c, 0x10);
    Addr(c.g[A3][0] as u32).off(0x50).sw(c, c.g[V0][0]);
    Addr(c.g[A3][0] as u32).off(8).sw(c, 0);
    let no_flag = c.g[A2][0] == 0;
    Addr(c.g[A3][0] as u32).off(4).sw(c, 0);
    if no_flag {
        c.cycles += 2;
        c.g[V0][0] = 1;
        Addr(c.g[A3][0] as u32).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(c.g[A3][0] as u32).sw(c, 0);
    }
    c.cycles += 35;
    c.lq(3, c.g[A1][0] as u32);
    c.g[V0][0] = c.g[A3][0];
    c.swc1(12, at(c.g[A3][0] as u32, 0x70));
    c.sq(3, at(c.g[A3][0] as u32, 0x60));
    c.sq(3, at(c.g[A3][0] as u32, 0x80));
    c.swc1(12, at(c.g[A3][0] as u32, 0x90));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lwc1(1, at(c.g[A1][0] as u32, 4));
    c.lwc1(2, at(c.g[A1][0] as u32, 8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.add_s(1, 1, 12);
    c.add_s(2, 2, 12);
    c.swc1(3, at(c.g[SP][0] as u32, 0xc));
    c.add_s(0, 0, 12);
    c.swc1(1, at(c.g[SP][0] as u32, 4));
    c.swc1(2, at(c.g[SP][0] as u32, 8));
    c.swc1(0, c.g[SP][0] as u32);
    c.lq(3, c.g[SP][0] as u32);
    c.sq(3, at(c.g[A3][0] as u32, 0x20));
    c.lwc1(1, at(c.g[A1][0] as u32, 4));
    c.lwc1(2, at(c.g[A1][0] as u32, 8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.sub_s(1, 1, 12);
    c.sub_s(2, 2, 12);
    c.sub_s(0, 0, 12);
    c.swc1(1, at(c.g[SP][0] as u32, 4));
    c.swc1(2, at(c.g[SP][0] as u32, 8));
    c.swc1(0, c.g[SP][0] as u32);
    c.lq(3, c.g[SP][0] as u32);
    c.sq(3, at(c.g[A3][0] as u32, 0x30));
    c.lq(4, c.g[A1][0] as u32);
    c.sq(4, at(c.g[A3][0] as u32, 0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// UI objects.

/// `void* UIObject_Load(UIObject* self, const UIObjectDef* def)` (0x39fe00):
/// take the id (+0x38), the data pointer from `def+4`, merge the packed
/// bit 0..4 / 6 flag of `self+0x14` with the `def+8` bits (5 bits into bit 8,
/// bit 6 into bit 0, bit 7 into bit 7), then forward bits 9..10 of `def+8` to
/// the object's `onCreate` slot.
pub fn ui_object_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        open(c, 0x30);
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        let def = c.g[A1][0];
        c.g[S0][0] = this;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = def;
        c.g[A0][0] = 0xffff_ffff_ffff_e0ff;
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        c.g[V0][0] = Addr(def as u32).off(4).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x14).lw(c);
        Addr(this as u32).off(0x38).sw(c, c.g[V0][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[A2][0] = Addr(this as u32).off(8).lw(c);
        c.g[V0][0] = Addr(def as u32).off(8).lbu(c);
        c.g[V0][0] = c.g[V0][0] & 0x1f;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this as u32).off(0x14).sw(c, c.g[V1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = Addr(def as u32).off(8).lw(c);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this as u32).off(0x14).sw(c, c.g[V1][0]);
        c.g[A1][0] = Addr(def as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x48).lh(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x4c).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 9) as u32);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A1][0] = c.g[A1][0] & 1;
        call_ptr(c, target, 0x0039_fe7c)?;
        c.g[V1][0] = Addr(def as u32).off(8).lbu(c);
        c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
        c.g[V0][0] = Addr(this as u32).off(0x14).lw(c);
        c.g[S1][0] = sx((def as u32).wrapping_add(0xc));
        c.g[V1][0] = c.g[V1][0] >> 7;
        Addr(this as u32).off(0x3c).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        Addr(this as u32).off(0x14).sw(c, c.g[V0][0]);
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `void MCManager_State0F_Save(MCManager* self)` (0x23f438): ask the
/// movie/stream object at +0x434 (three virtual predicates) whether a save is
/// due and forward the resulting state id (0x37 / 0x14 / 0x10) to the player
/// slot of `self->0x748`.
pub fn mc_manager_state0f_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x20);
        c.sq(16, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A1][0] = Addr(this as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f460)?;
        if c.g[V0][0] != 0 {
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A1][0] = Addr(this as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f480)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this as u32).off(0x748).lw(c);
            c.cycles += 6;
            c.g[A1][0] = 0x10;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_f4e4)?;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A1][0] = Addr(this as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x250).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x254).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f4a0)?;
        let no_save = c.g[V0][0] == 0;
        c.g[A1][0] = 0x37;
        if no_save {
            c.g[V0][0] = Addr(this as u32).off(0x748).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(this as u32).off(0x748).lw(c);
            c.g[A1][0] = 0x14;
        }
        c.cycles += 6;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f4c8)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x10));
        c.cycles += 3;
        epilogue(c, 0, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Visual FX.

/// `void cTransitionMan_Stop(cTransitionMan* self)` (0x2e4578): if a
/// transition is running, release both effect objects (virtual `Stop(3)`),
/// reset the state words and re-fetch the frame counter from the world's vfx
/// interface.
pub fn ctransition_man_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(this as u32).off(0x44).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 4;
        c.g[A2][0] = Addr(this as u32).off(0x4c).lw(c);
        Addr(this as u32).off(0x44).sw(c, 0);
        let none = c.g[A2][0] == 0;
        Addr(this as u32).off(0x40).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002e_45bc)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(this as u32).off(0x50).lw(c);
        let none = c.g[A2][0] == 0;
        Addr(this as u32).off(0x4c).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002e_45e0)?;
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(this as u32).off(0x6c).lw(c);
        Addr(this as u32).off(0x50).sw(c, 0);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        let skip = c.g[V0][0] == 0;
        Addr(this as u32).off(0x70).sw(c, 0);
        if !skip {
            c.cycles += 11;
            c.g[A1][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
            c.g[V0][0] = 1;
            Addr(this as u32).off(0x7c).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x390).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x394).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002e_4614)?;
            Addr(this as u32).off(0x80).sw(c, c.g[V0][0]);
            Addr(this as u32).off(0x64).sw(c, 0);
            Addr(this as u32).off(0x60).sw(c, 0);
        }
        c.cycles += 2;
        Addr(this as u32).off(0x78).sw(c, 0);
        c.lq(16, at(c.g[SP][0] as u32, 0x10));
        c.cycles += 3;
        epilogue(c, 0, 0x20);
        Ok(())
    })
}

/// `void cCutSceneTransitionMan_SaveViewport(cCutSceneTransitionMan* self)`
/// (0x2eaa28): snapshot the current viewport (16-byte quad + word at +0x10)
/// and the fade constants into +0x20.., choosing the "snap" variant when the
/// saved mode is 2.
pub fn ccutscene_transition_man_save_viewport(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        open(c, 0x20);
        c.g[A1][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
        c.sq(16, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x150).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x154).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_aa50)?;
        c.ldl(4, at(c.g[V0][0] as u32, 7));
        c.ldr(4, c.g[V0][0] as u32);
        c.ldl(5, at(c.g[V0][0] as u32, 0xf));
        c.ldr(5, at(c.g[V0][0] as u32, 8));
        c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.sdl(4, at(c.g[S0][0] as u32, 0x27));
        c.sdr(4, at(c.g[S0][0] as u32, 0x20));
        c.sdl(5, at(c.g[S0][0] as u32, 0x2f));
        c.sdr(5, at(c.g[S0][0] as u32, 0x28));
        Addr(c.g[S0][0] as u32).off(0x30).sw(c, c.g[A2][0]);
        c.g[V1][0] = 2;
        c.g[AT][0] = 0x3f40_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x20).lw(c);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x39a8));
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = 1;
            c.cycles += 7;
            c.g[AT][0] = 0x3e00_0000;
            c.mtc1(1, 0);
            c.swc1(2, at(c.g[S0][0] as u32, 0x44));
            Addr(c.g[S0][0] as u32).off(0x34).sw(c, c.g[V0][0]);
            c.swc1(0, at(c.g[S0][0] as u32, 0x38));
            c.swc1(1, at(c.g[S0][0] as u32, 0x40));
            c.swc1(1, at(c.g[S0][0] as u32, 0x3c));
        } else {
            c.cycles += 8;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.swc1(2, at(c.g[S0][0] as u32, 0x44));
            Addr(c.g[S0][0] as u32).off(0x34).sw(c, c.g[V0][0]);
            c.swc1(0, at(c.g[S0][0] as u32, 0x3c));
            c.swc1(1, at(c.g[S0][0] as u32, 0x40));
            Addr(c.g[S0][0] as u32).off(0x38).sw(c, 0);
        }
        c.cycles += 4;
        c.lq(16, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu destructors.

/// `void ShellRendererMenu_dtor(ShellRendererMenu* self, int flags)`
/// (0x24fbc0): destroy the eleven sub-menus as `MenuSysItem`s, then the page
/// base (which frees `self` when `flags & 1`).
pub fn shell_renderer_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        open(c, 0x30);
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        for (off, ret) in [
            (0x2ecu32, 0x0024_fbe4u32),
            (0x2b8, 0x0024_fbf0),
            (0x284, 0x0024_fbfc),
            (0x254, 0x0024_fc08),
            (0x224, 0x0024_fc14),
            (0x1f4, 0x0024_fc20),
            (0x1c4, 0x0024_fc2c),
            (0x194, 0x0024_fc38),
            (0x164, 0x0024_fc44),
            (0x14c, 0x0024_fc50),
            (0x130, 0x0024_fc5c),
        ] {
            c.g[A0][0] = sx(at(this as u32, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_fc68)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `void ShadowMenu_dtor(ShadowMenu* self, int flags)` (0x24fd00): destroy the
/// eleven sub-menus as `MenuSysItem`s, then the page base.
pub fn shadow_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        open(c, 0x30);
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        for (off, ret) in [
            (0x2fcu32, 0x0024_fd24u32),
            (0x2c8, 0x0024_fd30),
            (0x294, 0x0024_fd3c),
            (0x260, 0x0024_fd48),
            (0x22c, 0x0024_fd54),
            (0x1f8, 0x0024_fd60),
            (0x1c4, 0x0024_fd6c),
            (0x194, 0x0024_fd78),
            (0x164, 0x0024_fd84),
            (0x14c, 0x0024_fd90),
            (0x130, 0x0024_fd9c),
        ] {
            c.g[A0][0] = sx(at(this as u32, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_fda8)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd.

/// `void CrowdMan2d_Reset(CrowdMan2d* self)` (0x2292e0): clear the 0x2000 /
/// 0x500 byte blocks at +0x210 / +0x2220, set the 128 ids to -1 with a random
/// 300..599 ms delay each, and clear the counts.
pub fn crowd_man2d_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        open(c, 0x70);
        c.g[A1][0] = 0;
        c.sq(20, at(c.g[SP][0] as u32, 0x20));
        c.g[A2][0] = 0x2000;
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        c.sq(16, at(c.g[SP][0] as u32, 0x60));
        c.sq(17, at(c.g[SP][0] as u32, 0x50));
        c.g[A0][0] = sx((this as u32).wrapping_add(0x210));
        c.sq(18, at(c.g[SP][0] as u32, 0x40));
        c.g[S1][0] = sx((this as u32).wrapping_add(0x240));
        c.sq(19, at(c.g[SP][0] as u32, 0x30));
        c.g[S0][0] = sx((this as u32).wrapping_add(8));
        c.sq(21, at(c.g[SP][0] as u32, 0x10));
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        Addr(this as u32).off(0x2210).sw(c, 0);
        c.g[S5][0] = 0x12c;
        c.g[RA][0] = sx(0x0022_9330);
        c.g[S2][0] = 0x7f;
        call_ptr(c, 0x003e_6448, 0x0022_9330)?;
        loop {
            c.cycles += 7;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S3][0]);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[RA][0] = sx(0x0022_9340);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0031_77f0, 0x0022_9340)?;
            c.divu(0, 2, 21);
            c.cycles += 5;
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x12c));
            Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            if !more {
                c.cycles += 14;
                Addr(this as u32).off(0x2214).sw(c, 0);
                c.g[A0][0] = sx((this as u32).wrapping_add(0x2220));
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x500;
                call_ptr(c, 0x003e_6448, 0x0022_9374)?;
                c.lq(16, at(c.g[SP][0] as u32, 0x60));
                c.lq(17, at(c.g[SP][0] as u32, 0x50));
                c.lq(18, at(c.g[SP][0] as u32, 0x40));
                c.lq(19, at(c.g[SP][0] as u32, 0x30));
                c.lq(20, at(c.g[SP][0] as u32, 0x20));
                c.lq(21, at(c.g[SP][0] as u32, 0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x70);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9330);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// FE widget messages.

/// Hash of the string at `site` via the ELF hash, i.e. `GetHashValue32`.
fn hash_eq(c: &mut Cpu, item: u64, site: u32, ret: u32) -> Flow {
    c.g[S0][0] = Addr(item as u32).off(0x38).lw(c);
    c.g[A0][0] = sx(site);
    call_ptr(c, 0x0031_7670, ret)?;
    Ok(())
}

/// `void cFEStateMainMenu_OnWidgetCreated(cFEStateMainMenu* self,
/// FEItem* item)` (0x194d18): map the widget's hash id (+0x38) to a screen
/// action id (+0x18), comparing against five hash-keyed widgets.
pub fn cfe_state_main_menu_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[A0][0] = 0x460000;
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let item = c.g[A1][0];
        c.g[S1][0] = item;
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = sx(0x0019_4d3c);
        c.g[S0][0] = Addr(item as u32).off(0x38).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_4d3c)?;
        let same = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = 0x460000;
        if same {
            c.cycles += 2;
            c.g[V0][0] = 0xb;
            c.cycles += 2;
            Addr(c.g[S1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
            c.lq(16, at(c.g[SP][0] as u32, 0x20));
            c.cycles += 4;
            c.lq(17, at(c.g[SP][0] as u32, 0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            close(c, 0x30);
            return Ok(());
        }
        for (site, id, ret) in [
            (0x0046_0060u32, 0x9u64, 0x0019_4d58u32),
            (0x0046_0040, 0x16, 0x0019_4d74),
            (0x0046_0070, 0x5, 0x0019_4d90),
            (0x004a_17d8, 0x31, 0x0019_4dac),
        ] {
            c.cycles += 5;
            hash_eq(c, item, site, ret)?;
            if c.g[S0][0] == c.g[V0][0] {
                if id == 0x31 {
                    c.cycles += 1;
                    c.g[V0][0] = 0x31;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = id;
                }
                c.cycles += 2;
                Addr(c.g[S1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
                c.lq(16, at(c.g[SP][0] as u32, 0x20));
                c.cycles += 4;
                c.lq(17, at(c.g[SP][0] as u32, 0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x30);
                return Ok(());
            }
        }
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.cycles += 4;
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Animation.

/// `void GameAnimMap_RegisterBank(GameAnimMap* self, int kind, void* bank)`
/// (0x314f30): for each of `bank->count` handles (word at `bank+8`, stride
/// 0x14) register `handle | kind << 8` at index `i`.
pub fn game_anim_map_register_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x80);
        c.sq(17, at(c.g[SP][0] as u32, 0x60));
        c.sq(18, at(c.g[SP][0] as u32, 0x50));
        c.g[S1][0] = 0;
        c.sq(21, at(c.g[SP][0] as u32, 0x20));
        c.g[S2][0] = c.g[A2][0];
        c.sq(16, at(c.g[SP][0] as u32, 0x70));
        c.sq(19, at(c.g[SP][0] as u32, 0x40));
        c.sq(20, at(c.g[SP][0] as u32, 0x30));
        c.sq(22, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(4).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = c.g[A0][0];
        if !empty {
            c.cycles += 3;
            c.g[S4][0] = c.g[A1][0] & 0xff;
            c.g[S6][0] = 0xffff_ffff_ffff_ff00;
            c.g[S3][0] = 0;
            loop {
                c.cycles += 18;
                c.g[S0][0] = c.g[S0][0] & c.g[S6][0];
                c.g[V1][0] = Addr(c.g[S2][0] as u32).off(8).lw(c);
                c.g[S0][0] = c.g[S0][0] | c.g[S4][0];
                c.g[A0][0] = sx((c.g[S1][0] as u32) << 8);
                c.g[V0][0] = c.g[S0][0] & 0xff;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0031_4fac);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0031_4ef0, 0x0031_4fac)?;
                c.g[V1][0] = Addr(c.g[S2][0] as u32).off(4).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4f78);
                }
            }
        }
        c.cycles += 10;
        c.lq(16, at(c.g[SP][0] as u32, 0x70));
        c.lq(17, at(c.g[SP][0] as u32, 0x60));
        c.lq(18, at(c.g[SP][0] as u32, 0x50));
        c.lq(19, at(c.g[SP][0] as u32, 0x40));
        c.lq(20, at(c.g[SP][0] as u32, 0x30));
        c.lq(21, at(c.g[SP][0] as u32, 0x20));
        c.lq(22, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x80);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script process.

/// `void WScriptProcess_GotoTask(WScriptProcess* self, int id)` (0x307f58):
/// find the task whose id (+4) matches, retarget the process state word (+8)
/// and restart it (virtual `Start` at `task+0x40 + 0x20/0x24`); when the task
/// is already current, ask it to resume instead.
pub fn wscript_process_goto_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x40);
        c.sq(16, at(c.g[SP][0] as u32, 0x30));
        c.sq(17, at(c.g[SP][0] as u32, 0x20));
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sq(18, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = Addr(this as u32).off(4).lw(c);
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[A3][0] = 0x44;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
                if c.g[V1][0] != c.g[A1][0] {
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let resume = c.g[V0][0] != 0;
                    c.mult(0, 2, 3, 7);
                    if resume {
                        c.cycles += 6;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x40).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x30).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x34).lw(c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0030_7fc4)?;
                    }
                    c.cycles += 10;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
                    Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x40).lw(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
                    let target = c.g[A1][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, target, 0x0030_7fe4)?;
                    c.lq(16, at(c.g[SP][0] as u32, 0x30));
                    c.cycles += 5;
                    c.lq(17, at(c.g[SP][0] as u32, 0x20));
                    c.lq(18, at(c.g[SP][0] as u32, 0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    close(c, 0x40);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_7f88);
                }
            }
        }
        c.cycles += 1;
        c.lq(16, at(c.g[SP][0] as u32, 0x30));
        c.cycles += 5;
        c.lq(17, at(c.g[SP][0] as u32, 0x20));
        c.lq(18, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x40);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE widget messages.

/// `void FEScreenSelMulti_OnWidgetAction(FEScreenSelMulti* self, FEItem* item,
/// int msg)` (0x1a3160): on message 5/6 forward the item's action id (+0x18)
/// to the screen object's virtual handler (slot +0x20 or +0x28) and, when it
/// returns a state, push that state onto the state stack at `self->0x10+0x18`.
pub fn fescreen_sel_multi_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        c.sq(16, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let item = c.g[V1][0];
        let none = item == 0;
        c.g[S0][0] = c.g[A0][0];
        if none {
            c.cycles += 1;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 5;
        let is5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 6;
        if !is5 {
            c.cycles += 2;
            let is6 = c.g[A2][0] == c.g[V0][0];
            if !is6 {
                c.cycles += 2;
                c.lq(16, at(c.g[SP][0] as u32, 0x10));
                c.cycles += 3;
                epilogue(c, 0, 0x20);
                return Ok(());
            }
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.cycles += 10;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = Addr(item as u32).off(0x18).lw(c);
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001a_31f4)?;
            let null = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !null {
                c.cycles += 3;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x001a_3208)?;
            }
            c.cycles += 1;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = Addr(item as u32).off(0x18).lw(c);
        c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001a_31b8)?;
        let null = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[V0][0];
        if null {
            c.cycles += 1;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x001a_31cc)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x10));
        c.cycles += 3;
        epilogue(c, 0, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state constructors.

/// `cFEStateChatRoom* cFEStateChatRoom_ctor(cFEStateChatRoom* self, int a1)`
/// (0x1b1f88): run the net FE-state constructor, retag the vtable, clear the
/// two item pairs and the four colour floats (1,1,1,1), then copy the global
/// player name into +0x72c.
pub fn cfe_state_chat_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x30);
        c.g[A2][0] = 0;
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.sd(at(c.g[SP][0] as u32, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x001b_1fa0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x001a_8500, 0x001b_1fa0)?;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = 0x3d;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x51a8));
        Addr(this as u32).off(0xc).sw(c, c.g[V0][0]);
        Addr(this as u32).off(8).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((this as u32).wrapping_add(0x704));
        Addr(this as u32).off(0x6f8).sw(c, 0);
        c.g[V1][0] = 1;
        loop {
            c.cycles += 7;
            Addr((c.g[V0][0] as u32).wrapping_sub(8)).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(c.g[V0][0] as u32).sw(c, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1fc0);
            }
        }
        c.cycles += 26;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x70c));
        Addr(this as u32).off(0x6ec).sw(c, 0);
        c.swc1(0, at(c.g[SP][0] as u32, 0xc));
        c.swc1(0, c.g[SP][0] as u32);
        c.swc1(0, at(c.g[SP][0] as u32, 4));
        c.swc1(0, at(c.g[SP][0] as u32, 8));
        Addr(this as u32).off(0x6f0).sw(c, 0);
        Addr(this as u32).off(0x6f4).sw(c, 0);
        c.ldl(2, at(c.g[SP][0] as u32, 7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(3, at(c.g[SP][0] as u32, 0xf));
        c.ldr(3, at(c.g[SP][0] as u32, 8));
        c.sdl(2, at(this as u32, 0x733));
        c.sdr(2, at(this as u32, 0x72c));
        c.sdl(3, at(this as u32, 0x73b));
        c.sdr(3, at(this as u32, 0x734));
        c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[RA][0] = sx(0x001b_2030);
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x80).lw(c);
        call_ptr(c, 0x0041_66f4, 0x001b_2030)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(at(c.g[SP][0] as u32, 0x10));
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script manager.

/// `void WScriptMan_Reset(WScriptMan* self)` (0x308988): clear the whole
/// script-manager state (-1 / 0xFFFFFFFF sentinels included), the command
/// queue, the local tables and the 0x180-byte block at +0x44.
pub fn wscript_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sq(16, at(c.g[SP][0] as u32, 0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V1][0] = u64::MAX;
        Addr(this as u32).off(0x290).sw(c, 0);
        Addr(this as u32).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(this as u32).off(0x294).sw(c, 0);
        c.g[A2][0] = 8;
        Addr(this as u32).off(0x298).sw(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x38));
        Addr(this as u32).off(0x2a4).sw(c, 0);
        Addr(this as u32).off(0x2a8).sw(c, 0);
        Addr(this as u32).off(0x29c).sw(c, 0);
        Addr(this as u32).off(0x2ac).sw(c, 0);
        for off in [
            0x4u32, 0x8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x28, 0x30, 0x34,
        ] {
            Addr(this as u32).off(off).sw(c, 0);
        }
        Addr(this as u32).off(0x24).sw(c, c.g[V1][0]);
        Addr(this as u32).off(0x2c).sw(c, c.g[V0][0]);
        Addr(this as u32).off(0x2a0).sw(c, c.g[V0][0]);
        Addr(this as u32).off(0x2b0).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0030_8a10);
        Addr(this as u32).off(0xc).sw(c, c.g[V1][0]);
        call_ptr(c, 0x003e_6448, 0x0030_8a10)?;
        c.g[RA][0] = sx(0x0030_8a18);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1c4));
        call_ptr(c, 0x0030_c760, 0x0030_8a18)?;
        Addr(this as u32).off(0x40).sw(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x44));
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0030_8a2c);
        c.g[A2][0] = 0x180;
        call_ptr(c, 0x003e_6448, 0x0030_8a2c)?;
        c.g[RA][0] = sx(0x0030_8a34);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0030_8aa0, 0x0030_8a34)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE message centre.

/// `void MessageCenter_RemoveMessagesOfGroup(int group)` (0x1e32c8): walk the
/// inbox message list and drop every message whose group (in the global table
/// at 0x4c6c08, stride 0x18, field +4) matches, compacting as it goes.
pub fn message_center_remove_messages_of_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        open(c, 0x80);
        c.sq(20, at(c.g[SP][0] as u32, 0x30));
        c.sq(17, at(c.g[SP][0] as u32, 0x60));
        let group = c.g[A0][0];
        c.g[S4][0] = group;
        c.sq(18, at(c.g[SP][0] as u32, 0x50));
        c.g[S1][0] = 0;
        c.sq(19, at(c.g[SP][0] as u32, 0x40));
        c.sq(16, at(c.g[SP][0] as u32, 0x70));
        c.sq(21, at(c.g[SP][0] as u32, 0x20));
        c.sq(22, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001e_32fc);
        c.g[S3][0] = 0;
        call_ptr(c, 0x001e_39f8, 0x001e_32fc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xcc).lbu(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = 0x4c0000;
        if !empty {
            c.cycles += 5;
            c.g[S6][0] = 0x18;
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[S0][0] = c.g[S2][0];
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 5;
                c.mult(0, 3, 2, 22);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
                if c.g[S4][0] != c.g[V1][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e_333c);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x001e_31b8, 0x001e_333c)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xcc).lbu(c);
                }
                c.cycles += 2;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xcc).lbu(c);
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001e_3320);
                }
            }
        }
        c.cycles += 10;
        c.lq(16, at(c.g[SP][0] as u32, 0x70));
        c.lq(17, at(c.g[SP][0] as u32, 0x60));
        c.lq(18, at(c.g[SP][0] as u32, 0x50));
        c.lq(19, at(c.g[SP][0] as u32, 0x40));
        c.lq(20, at(c.g[SP][0] as u32, 0x30));
        c.lq(21, at(c.g[SP][0] as u32, 0x20));
        c.lq(22, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x80);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI state destructor.

/// `void UIStateMgr_dtor(UIStateMgr* self, int flags)` (0x39e9d8): destroy the
/// state stack, the three lists and the four list nodes, retagging each
/// vtable, then delete `self` when `flags & 1`.
pub fn ui_state_mgr_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        open(c, 0x60);
        c.sq(20, at(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        c.sq(18, at(c.g[SP][0] as u32, 0x30));
        c.sq(16, at(c.g[SP][0] as u32, 0x50));
        c.g[S2][0] = c.g[A1][0];
        c.sq(17, at(c.g[SP][0] as u32, 0x40));
        c.sq(19, at(c.g[SP][0] as u32, 0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039_ea04);
        c.g[S3][0] = sx((this as u32).wrapping_add(0x1c));
        call_ptr(c, 0x0039_f8c8, 0x0039_ea04)?;
        c.g[S1][0] = 0x490000;
        c.g[A0][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c98));
        c.g[RA][0] = sx(0x0039_ea18);
        Addr(this as u32).off(0x34).sw(c, c.g[S1][0]);
        call_ptr(c, 0x0039_7948, 0x0039_ea18)?;
        c.g[S0][0] = 0x490000;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x28));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4cc0));
        c.g[RA][0] = sx(0x0039_ea2c);
        Addr(this as u32).off(0x30).sw(c, c.g[S0][0]);
        call_ptr(c, 0x003a_4868, 0x0039_ea2c)?;
        Addr(this as u32).off(0x24).sw(c, c.g[S0][0]);
        c.g[RA][0] = sx(0x0039_ea38);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x003a_4868, 0x0039_ea38)?;
        Addr(this as u32).off(0x18).sw(c, c.g[S1][0]);
        c.g[RA][0] = sx(0x0039_ea44);
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0039_7948, 0x0039_ea44)?;
        Addr(this as u32).off(0x14).sw(c, c.g[S0][0]);
        c.g[RA][0] = sx(0x0039_ea50);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xc));
        call_ptr(c, 0x003a_4868, 0x0039_ea50)?;
        Addr(this as u32).off(8).sw(c, c.g[S0][0]);
        c.g[RA][0] = sx(0x0039_ea5c);
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x003a_4868, 0x0039_ea5c)?;
        c.g[S2][0] = c.g[S2][0] & 1;
        let keep = c.g[S2][0] == 0;
        c.lq(16, at(c.g[SP][0] as u32, 0x50));
        if !keep {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_ea70);
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0031_7e50, 0x0039_ea70)?;
            c.lq(16, at(c.g[SP][0] as u32, 0x50));
        }
        c.cycles += 7;
        c.lq(17, at(c.g[SP][0] as u32, 0x40));
        c.lq(18, at(c.g[SP][0] as u32, 0x30));
        c.lq(19, at(c.g[SP][0] as u32, 0x20));
        c.lq(20, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x60);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP, RA as RET};
    use ssx3_ee as ee;

    /// Scrambled registers and a game-like `gp` (no fake-call hooks: these
    /// tests only reach callees that are already ported).
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x47_0133 ^ ((seed as u64) << 9) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Call a guest setup function and return `v0`.
    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `GetHashValue32` of the string at `site`.
    fn str_hash(c: &mut Cpu, site: u32) -> u64 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[site as u64])
    }

    /// The lazily built BE singleton (`gp-0x1f04`) pointed at a zeroed dummy,
    /// and the current interface index (`gp-0x1efc`) at zero.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }

    /// The engine's size-class pool allocator (see `oc57`/`oc102`): short free
    /// lists behind `gp + 0x2a60..0x2a84`, enabled by the "pools ready" flag at
    /// `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    // -------------------------------------------------------------- FE lists.

    #[test]
    fn cfe_state_char_equip_remove_list_item_matches() {
        check(ee::s03::f_0019b458__generated, cfe_state_char_equip_remove_list_item, 200, &|c, seed| {
            let mut r = base(c, seed, 0x19_b458);
            c.g[A0][0] = sx(DATA);
            let count = (seed % 6) as u32;
            let idx = r.next_u32() % (count + 2);
            c.g[A1][0] = sx(idx);
            c.bus.write32(DATA + 0x958, count);
            for i in 0..8 {
                c.bus.write32(DATA + 0x124 + i * 4, r.next_u32());
            }
            let list = DATA + 0x1000;
            c.bus.write32(DATA + 0x5c, list);
            c.bus.write8(list + 0x95, (r.next_u32() % 9) as u8);
            c.bus.write8(list + 0x98, (r.next_u32() % 9) as u8);
        });
    }

            // ---------------------------------------------------------------- Input.

    #[test]
    fn input_sync_pack_sample_matches() {
        check(ee::s06::f_00237b28__generated, input_sync_pack_sample, 200, &|c, seed| {
            let mut r = base(c, seed, 0x23_7b28);
            let src = DATA + 0x1000;
            let dst = DATA + 0x2000;
            let n = (seed % 6) as u32;
            c.g[A1][0] = sx(src);
            c.g[A2][0] = sx(dst);
            c.g[A0][0] = sx(dst + 1);
            c.bus.write32(src, n);
            for i in 0..6u32 {
                let zero = seed % 3 == 0 && i < 2;
                let v = if zero { 0 } else { r.edge_f32() };
                c.bus.write32(src + 4 + i * 4, v);
            }
            for i in 0..8u32 {
                c.bus.write32(dst + i * 4, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------------ Collision.

    #[test]
    fn coll_sphere_ctor_matches() {
        check(ee::s11::f_0032fb48__generated, coll_sphere_ctor, 200, &|c, seed| {
            let mut r = base(c, seed, 0x32_fb48);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx((seed % 3) as u32);
            c.f[12] = r.edge_f32();
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x1000 + i * 4, r.edge_f32());
            }
        });
    }

    // ------------------------------------------------------------------ UI.

    #[test]
    fn ui_object_load_matches() {
        check(ee::s13::f_0039fe00__generated, ui_object_load, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x39_fe00, 1);
            let this = DATA;
            let def = DATA + 0x2000;
            let vt = DATA + 0x1000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(def);
            c.bus.write32(this + 8, vt);
            c.bus.write16(vt + 0x48, 0);
            c.bus.write32(vt + 0x4c, FAKE);
            c.bus.write32(def + 4, r.next_u32());
            c.bus.write32(def + 8, r.next_u32());
            c.bus.write32(this + 0x14, r.next_u32());
        });
    }

    #[test]
    fn mc_manager_state0f_save_matches() {
        check(ee::s06::f_0023f438__generated, mc_manager_state0f_save, 200, &|c, seed| {
            machine(c, seed, 0x23_f438, 4);
            let this = DATA;
            let obj = DATA + 0x1000;
            let vt = DATA + 0x2000;
            let vt2 = DATA + 0x2400;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0x434, obj);
            c.bus.write32(obj, vt);
            // Which of the three predicates report "save needed".
            let zero: u32 = 0x0028_b180; // returns gp+0x410 (0)
            let mode = seed % 4;
            let slot = |m: u32| if m != 0 { FAKE as u64 } else { zero as u64 };
            c.bus.write16(vt + 0x58, 0);
            c.bus.write32(vt + 0x5c, slot(mode) as u32);
            c.bus.write16(vt + 0x248, 0);
            c.bus.write32(vt + 0x24c, slot(mode / 2) as u32);
            c.bus.write16(vt + 0x250, 0);
            c.bus.write32(vt + 0x254, slot(mode / 3) as u32);
            c.bus.write32(this + 0x748, vt2);
            c.bus.write16(vt2 + 8, 0);
            c.bus.write32(vt2 + 0xc, FAKE + 0x40);
        });
    }

    // ------------------------------------------------------------- VisualFX.

    #[test]
    fn ctransition_man_stop_matches() {
        check(ee::s09::f_002e4578__generated, ctransition_man_stop, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e_4578, 3);
            let this = DATA;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0x44, if seed % 2 == 0 { 0 } else { 1 });
            let obj1 = DATA + 0x1000;
            let obj2 = DATA + 0x1200;
            c.bus.write32(this + 0x4c, if seed % 3 == 0 { 0 } else { obj1 });
            c.bus.write32(this + 0x50, if seed % 5 == 0 { 0 } else { obj2 });
            for (i, o) in [obj1, obj2].into_iter().enumerate() {
                let vt = DATA + 0x2000 + 0x100 * i as u32;
                c.bus.write32(o, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE + 0x10 * i as u32);
            }
            c.bus.write32(this + 0x6c, if seed % 7 == 0 { u32::MAX } else { r.next_u32() });
            let world = DATA + 0x3000;
            let vt3 = DATA + 0x3400;
            c.bus.write32(GAME_GP + 0x2a90, world);
            c.bus.write32(world + 0x10d8, vt3);
            c.bus.write16(vt3 + 0x390, 0);
            c.bus.write32(vt3 + 0x394, FAKE + 0x40);
        });
    }

    #[test]
    fn ccutscene_transition_man_save_viewport_matches() {
        check(ee::s09::f_002eaa28__generated, ccutscene_transition_man_save_viewport, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e_aa28, 1);
            let this = DATA;
            c.g[A0][0] = sx(this);
            let world = DATA + 0x3000;
            let vt = DATA + 0x3400;
            c.bus.write32(GAME_GP + 0x2a90, world);
            c.bus.write32(world + 0x10d8, vt);
            c.bus.write16(vt + 0x150, 0);
            c.bus.write32(vt + 0x154, FAKE);
            for i in 0..8u32 {
                c.bus.write32(this + 0x20 + i * 4, r.next_u32());
            }
            c.bus.write32(this + 0x20, if seed % 3 == 0 { 2 } else { r.next_u32() % 3 });
            c.bus.write32(GAME_GP.wrapping_sub(0x39a8), r.edge_f32());
        });
    }

    // -------------------------------------------------------------- Menu dtor.

    #[test]
    fn shell_renderer_menu_dtor_matches() {
        check(ee::s06::f_0024fbc0__generated, shell_renderer_menu_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x24_fbc0, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            for off in [0x130u32, 0x14c, 0x164, 0x194, 0x1c4, 0x1f4, 0x224, 0x254, 0x284, 0x2b8, 0x2ec] {
                c.bus.write32(DATA + off + 0x10, r.next_u32());
            }
        });
    }

    #[test]
    fn shadow_menu_dtor_matches() {
        check(ee::s06::f_0024fd00__generated, shadow_menu_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x24_fd00, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            for off in [0x130u32, 0x14c, 0x164, 0x194, 0x1c4, 0x1f8, 0x22c, 0x260, 0x294, 0x2c8, 0x2fc] {
                c.bus.write32(DATA + off + 0x10, r.next_u32());
            }
        });
    }

    // ---------------------------------------------------------------- Crowd.

    #[test]
    fn crowd_man2d_reset_matches() {
        check(ee::s05::f_002292e0__generated, crowd_man2d_reset, 5, &|c, seed| {
            let mut r = base(c, seed, 0x22_92e0);
            c.g[A0][0] = sx(DATA);
            for i in 0..0x2a0u32 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------------------- FE.

    #[test]
    fn cfe_state_main_menu_on_widget_created_matches() {
        check(ee::s02::f_00194d18__generated, cfe_state_main_menu_on_widget_created, 200, &|c, seed| {
            let mut r = base(c, seed, 0x19_4d18);
            let sites = [0x0046_0050u32, 0x0046_0060, 0x0046_0040, 0x0046_0070, 0x004a_17d8];
            // Distinct short strings at every hash key.
            for (i, site) in sites.iter().enumerate() {
                for k in 0..8u32 {
                    c.bus.write8(site + k, 0);
                }
                for k in 0..(i as u32 + 1) {
                    c.bus.write8(site + k, b'A' + i as u8 + k as u8);
                }
            }
            let pick = seed % 6;
            let id = if pick < 5 {
                str_hash(c, sites[pick as usize]) as u32
            } else {
                r.next_u32()
            };
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x38, id);
        });
    }

    // ------------------------------------------------------------ Animation.

    #[test]
    fn game_anim_map_register_bank_matches() {
        check(ee::s10::f_00314f30__generated, game_anim_map_register_bank, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x31_4f30, 0);
            let bank = DATA + 0x1000;
            c.g[A0][0] = sx(DATA + 0x2000);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(bank);
            let n = if seed % 6 == 0 { 0 } else { r.next_u32() % 4 };
            c.bus.write32(bank + 4, DATA + 0x1200);
            c.bus.write16(DATA + 0x1202, n as u16);
            c.bus.write32(bank + 8, DATA + 0x1400);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x1400 + i * 0x14, r.next_u32());
            }
        });
    }

    // --------------------------------------------------------------- Script.

    #[test]
    fn wscript_process_goto_task_matches() {
        check(ee::s10::f_00307f58__generated, wscript_process_goto_task, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x30_7f58, 2);
            let this = DATA;
            c.g[A0][0] = sx(this);
            // Id 3 never matches, so some seeds walk the whole list.
            c.g[A1][0] = sx(if seed % 4 == 0 { 3 } else { seed % 3 });
            let n = (seed % 4) as u32;
            c.bus.write32(this + 4, n);
            c.bus.write32(this + 8, r.next_u32() % (n + 1));
            let arr = DATA + 0x1000;
            c.bus.write32(this + 0x14, arr);
            for i in 0..4u32 {
                let t = arr + i * 0x44;
                c.bus.write32(t + 4, (seed as u32 + i) % 3);
                let obj = DATA + 0x2000 + i * 0x100;
                c.bus.write32(t + 0x40, obj);
                c.bus.write16(obj + 0x20, 0);
                c.bus.write32(obj + 0x24, FAKE + 0x10 * i);
                c.bus.write16(obj + 0x30, 0);
                c.bus.write32(obj + 0x34, FAKE + 0x40 + 0x10 * i);
            }
        });
    }

    #[test]
    fn wscript_man_reset_matches() {
        check(ee::s10::f_00308988__generated, wscript_man_reset, 50, &|c, seed| {
            let mut r = base(c, seed, 0x30_8988);
            c.g[A0][0] = sx(DATA);
            // Only the state fields: the local-table pointers at +0x3cc must
            // stay zero so `WScriptMan_FreeLocalTables` walks an empty table.
            for i in 0..0x80u32 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------- FE screen select.

    #[test]
    fn fescreen_sel_multi_on_widget_action_matches() {
        check(ee::s03::f_001a3160__generated, fescreen_sel_multi_on_widget_action, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x1a_3160, 2);
            let this = DATA;
            let item = DATA + 0x1000;
            let obj = DATA + 0x2000;
            let vt = DATA + 0x2400;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 8 == 0 { 0 } else { sx(item) };
            c.g[A2][0] = sx((4 + seed % 3) as u32);
            c.bus.write32(this + 0x10, obj);
            c.bus.write32(item + 0x18, r.next_u32());
            c.bus.write32(obj, vt);
            c.bus.write16(vt + 0x20, 0);
            c.bus.write32(vt + 0x24, FAKE);
            c.bus.write16(vt + 0x28, 0);
            c.bus.write32(vt + 0x2c, FAKE + 0x10);
            c.bus.write32(obj + 0x18, 0);
            c.bus.write32(obj + 0x1c, 0);
        });
    }

    // -------------------------------------------------- FE state constructor.

        // ----------------------------------------------------------- Message centre.

    #[test]
    fn message_center_remove_messages_of_group_matches() {
        check(ee::s04::f_001e32c8__generated, message_center_remove_messages_of_group, 100, &|c, seed| {
            machine(c, seed, 0x1e_32c8, 4);
            be_library(c);
            c.g[A0][0] = sx((seed % 8) as u32);
        });
    }

    // ------------------------------------------------------------- UI state.

    #[test]
    fn ui_state_mgr_dtor_matches() {
        check(ee::s13::f_0039e9d8__generated, ui_state_mgr_dtor, 100, &|c, seed| {
            machine(c, seed, 0x39_e9d8, 0);
            c.g[A0][0] = sx(DATA);
            // Even flags: the page base must not free the test object. The
            // lists stay empty so every list destructor takes its short path.
            c.g[A1][0] = [0u64, 2][seed as usize % 2];
        });
    }
}
