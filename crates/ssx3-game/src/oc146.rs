//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! The wave is mostly leaves and small front-end / UI callbacks: page draw
//! begin, trigger lookup, quaternion slerp, fence and bridge render, lobby
//! network reply handlers, snow-box construction, reward unlocks.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_b180, menu_sys_page_begin_draw, "MenuSysPage_BeginDraw"),
    (
        0x002b_63d0,
        world_trigger_manager_get_trigger_info,
        "WorldTriggerManager_GetTriggerInfo",
    ),(0x0034_d778, fence_render, "Fence_Render"),(
        0x002e_be20,
        transition_effect_ctor_random2,
        "TransitionEffect_ctorRandom2",
    ),
    (
        0x003a_13e0,
        cuit_ext_scroll_handle_input,
        "cUITextScroll_HandleInput",
    ),
    (
        0x002d_0d48,
        menu_sys_point2d_item_measure,
        "MenuSysPoint2DItem_Measure",
    ),
    (0x0034_7b80, flex_bridge_render, "FlexBridge_Render"),(
        0x001c_70f0,
        cfe_popup_confirm_resize_text_widgets,
        "cFEPopupConfirm_ResizeTextWidgets",
    ),
    (
        0x0025_c0c0,
        lobby_man_on_edit_account_reply,
        "LobbyMan_OnEditAccountReply",
    ),
    (
        0x0025_c2d8,
        lobby_man_on_lost_by_email_reply,
        "LobbyMan_OnLostByEmailReply",
    ),
    (
        0x0025_c4f0,
        lobby_man_on_lost_by_name_reply,
        "LobbyMan_OnLostByNameReply",
    ),
    (0x002e_4d88, snow_box_ctor, "SnowBox_ctor"),
    (
        0x0033_0540,
        coll_sphere_tree_overlaps_aabb,
        "CollSphereTree_OverlapsAABB",
    ),
];

// ---------------------------------------------------------------------------
// Helpers.

#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    crate::abi::r32(c, GP).wrapping_add(off as u32)
}

/// The menu-system singleton (`gp - 0x854`).
#[inline(always)]
fn menu_sys(c: &mut Cpu) -> u32 {
    c.lw(gp(c, -0x854)) as u32
}

// ---------------------------------------------------------------------------
// 0x002cb180 MenuSysPage_BeginDraw
// ---------------------------------------------------------------------------

/// `void MenuSysPage_BeginDraw(void* page)` (0x2cb180).
///
/// Pushes the twenty-byte draw state of the current menu object (the page
/// pointer at `menu_sys + 0xe84`) onto that object's save stack, then runs the
/// four renderer passes the interface singleton's dispatch table holds:
/// viewport, scissor, background and font. The `v0` result is the caller's
/// (the last pass's return value is left in the register).
pub fn menu_sys_page_begin_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 94;
        let frame = Frame::open(c, 0x10);
        let ms = menu_sys(c);
        c.g[A0][0] = sx(ms);
        frame.save_ra(c, 0);

        // Push the 20-byte state at obj[0..0x14) onto obj[0x14..0x28).
        let obj = c.lw(p(ms, 0xe84)) as u32;
        c.ldl(3, p(obj, 0x7));
        c.ldr(3, obj);
        c.ldl(5, p(obj, 0xf));
        c.ldr(5, p(obj, 8));
        c.g[A2][0] = c.lw(p(obj, 0x10));
        c.sdl(3, p(obj, 0x1b));
        c.sdr(3, p(obj, 0x14));
        c.sdl(5, p(obj, 0x23));
        c.sdr(5, p(obj, 0x1c));
        c.sw(p(obj, 0x24), c.g[A2][0]);
        // ... and advance the save pointer by one slot.
        c.g[V1][0] = c.lw(p(ms, 0xe84));
        c.g[A1][0] = sx(ms);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
        c.sw(p(ms, 0xe84), c.g[V1][0]);

        // Renderer pass at 0xa8/0xac.
        let renderer = c.lw(p(ms, 0x10d8)) as u32;
        c.g[V0][0] = sx(renderer);
        c.g[A0][0] = c.lh(p(renderer, 0xa8));
        c.g[V1][0] = c.lw(p(renderer, 0xac));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b1dc)?;

        // Renderer pass at 0xf8/0xfc.
        c.g[A1][0] = sx(ms);
        c.g[V0][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0xf8));
        c.g[V1][0] = c.lw(p(renderer, 0xfc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b1f4)?;

        // The draw-state bit fields: clear and set the pass flags of the
        // current menu object.
        c.g[T0][0] = sx(ms);
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = 0xffff_ffff_fe7f_0000;
        c.g[T3][0] = 0x0100_0000;
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[T1][0] = 0xffff_ffff_ffcf_0000;
        c.g[T4][0] = 0x0030_0000;
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
        c.g[T1][0] = c.g[T1][0] | 0xffff;
        c.g[T2][0] = 0xffff_ffff_fff0_0000;
        c.g[A3][0] = 0x0001_0000;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[T2][0] = c.g[T2][0] | 0xfff;
        c.g[V0][0] = c.g[V0][0] | 0x220;
        c.g[A3][0] = c.g[A3][0] | 0x4000;
        c.sw(p(c.g[A0][0] as u32, 8), c.g[V0][0]);
        c.g[T5][0] = 0xffff_ffff_ffff_ff83;
        c.mtc1(0, 12); // f12 = 0.0
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0x4420_0000; // 40.0
        c.mtc1(1, 14);
        c.mov_s(13, 12);
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
        c.mov_s(16, 12);
        c.g[AT][0] = 0x43f0_0000; // 480.0
        c.mtc1(1, 15);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 17);
        c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
        c.sw(p(c.g[A0][0] as u32, 4), c.g[V0][0]);
        c.g[V1][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
        c.sw(p(c.g[V1][0] as u32, 4), c.g[V0][0]);
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
        c.sw(p(c.g[A0][0] as u32, 4), c.g[V0][0]);
        c.g[A3][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[A3][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw(p(c.g[A3][0] as u32, 4), c.g[V0][0]);

        // Renderer pass at 0xd0/0xd4.
        c.g[V1][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0xd0));
        c.g[V0][0] = c.lw(p(renderer, 0xd4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b2cc)?;

        // Renderer pass at 0x110/0x114.
        c.g[A2][0] = sx(menu_sys(c));
        c.g[A1][0] = 0x0050_0000;
        c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        c.g[V0][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0x110));
        c.g[V1][0] = c.lw(p(renderer, 0x114));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
        call_ptr(c, t, 0x002c_b2ec)?;

        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002b63d0 WorldTriggerManager_GetTriggerInfo
// ---------------------------------------------------------------------------

/// `int WorldTriggerManager_GetTriggerInfo(WTM* self, int idx, int* type,
/// int* a, int* b, int c, int d)` (0x2b63d0).
///
/// Looks the trigger entry up by index: type 3 is a "disabled" entry, type 1
/// hands out the entry's two object fields and type 2 picks one of the
/// object's four random thresholds with `Audio_Rand`. Anything else is
/// delegated to `GetSpecialTriggerInfo` unless `d` says not to. Returns 1 when
/// the index was in range.
pub fn world_trigger_manager_get_trigger_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.sw(c.g[T0][0] as u32, 0);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x340));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A3][0];

        'ret: {
            if !in_range {
                c.cycles += 4;
                c.sw(c.g[A2][0] as u32, 0);
                c.g[V0][0] = 0;
                c.sw(c.g[S0][0] as u32, 0);
                break 'ret;
            }
            'found: {
                'not3: {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x33c));
                    c.g[S2][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[V0][0] = 3;
                    c.g[A0][0] = sx(c.g[S2][0] as u32 + c.g[V1][0] as u32);
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S3][0] = 1;
                    if t {
                        break 'not3;
                    } else {
                        c.cycles += 3;
                        c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                        c.sw(c.g[S0][0] as u32, 0);
                        break 'found;
                    }
                }
                'not1: {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[S3][0];
                    c.g[V0][0] = 2;
                    if t {
                        break 'not1;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
                        c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                        c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x33c));
                        c.g[V0][0] = sx(c.g[S2][0] as u32 + c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 4));
                        c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                        c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                        break 'found;
                    }
                }
                'type2: {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    if t {
                        break 'type2;
                    }
                    c.cycles += 12;
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                    c.g[RA][0] = sx(0x002b_647c);
                    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                    call_ptr(c, 0x0028_b180, 0x002b_647c)?;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x118));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[RA][0] = sx(0x002b_648c);
                    c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 0x1d8));
                    call_ptr(c, 0x002a_df60, 0x002b_648c)?;
                    c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x33c));
                    c.g[A0][0] = 0x7fff;

                    // The four threshold buckets, largest first.
                    let entry = (c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32);
                    c.cycles += 11;
                    c.g[V0][0] = c.g[V0][0] & 0x7fff;
                    c.g[V1][0] = sx(entry);
                    c.g[A1][0] = c.lw(p(c.g[V1][0] as u32, 4));
                    c.g[V1][0] = c.lw(p(c.g[A1][0] as u32, 0x34));
                    c.g[A2][0] = c.lw(p(c.g[A1][0] as u32, 0x24));
                    c.mult(0, 2, 2, 3);
                    c.divu(0, 2, 4);
                    c.g[V1][0] = c.lo[0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.lw(p(c.g[A1][0] as u32, 0x28));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x14));
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        break 'found;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(c.g[A0][0] as u32 + c.g[A2][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x2c));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x18));
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        break 'found;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(c.g[V0][0] as u32 + c.g[A0][0] as u32);
                    c.g[V0][0] = sx(c.g[V0][0] as u32 + c.g[A2][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x20));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x1c));
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        break 'found;
                    }
                    c.cycles += 2;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    break 'found;
                }
                // Fallback entries: delegate unless `d` is set.
                c.cycles += 2;
                let t = c.g[T2][0] != 0;
                c.g[V0][0] = c.g[S3][0];
                if t {
                    break 'ret;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b_652c);
                    c.g[A3][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_6550, 0x002b_652c)?;
                    c.g[S3][0] = c.g[V0][0];
                    break 'found;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }

        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031bcb0 Quat_Slerp
// ---------------------------------------------------------------------------

/// `void Quat_Slerp(Quat* dst, const Quat* a, const Quat* b, float t, ...)`
/// (0x31bcb0).
///
/// Normalised quaternion slerp: negates `b` when the dot product is negative,
/// then blends `a` and (possibly negated) `b` with `sin((1-t)θ)/sinθ` and
/// `sin(tθ)/sinθ`, falling back to a straight lerp when the two quaternions
/// are nearly parallel. `theta = acos(dot)` comes from `Math_Asin` on the dot
/// product with the quadrant constant at `gp-0x2ec4`.
pub fn quat_slerp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        c.lwc1(5, c.g[A2][0] as u32);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_fpr(c, 22, 0x60);
        frame.save_ra(c, 0x20);
        c.mov_s(22, 12);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        c.mtc1(0, 4); // f4 = 0.0
        c.lwc1(6, p(c.g[A2][0] as u32, 4));
        c.lwc1(0, c.g[S0][0] as u32);
        c.lwc1(2, p(c.g[S0][0] as u32, 4));
        c.mul_s(0, 0, 5);
        c.lwc1(7, p(c.g[A2][0] as u32, 8));
        c.mul_s(2, 2, 6);
        c.lwc1(3, p(c.g[S0][0] as u32, 8));
        c.lwc1(8, p(c.g[A2][0] as u32, 0xc));
        c.mul_s(3, 3, 7);
        c.lwc1(1, p(c.g[S0][0] as u32, 0xc));
        c.add_s(0, 0, 2);
        c.mul_s(1, 1, 8);
        c.add_s(0, 0, 3);
        c.add_s(12, 0, 1);
        c.c_lt_s(12, 4);
        let negate = !c.fcc();
        c.g[S1][0] = c.g[A0][0];
        'neg: {
            if !negate {
                break 'neg;
            }
            c.cycles += 10;
            c.neg_s(0, 5);
            c.neg_s(1, 6);
            c.neg_s(2, 7);
            c.neg_s(3, 8);
            c.swc1(0, c.g[SP][0] as u32);
            c.swc1(1, p(c.g[SP][0] as u32, 4));
            c.neg_s(12, 12);
            c.swc1(2, p(c.g[SP][0] as u32, 8));
            c.swc1(3, p(c.g[SP][0] as u32, 0xc));
            break 'neg;
        }
        c.cycles += 4;
        c.swc1(5, c.g[SP][0] as u32);
        c.swc1(6, p(c.g[SP][0] as u32, 4));
        c.swc1(7, p(c.g[SP][0] as u32, 8));
        c.swc1(8, p(c.g[SP][0] as u32, 0xc));

        // The blend weights: f20 for `a`, f12 for `b`.
        'weights: {
            'lerp: {
                c.cycles += 8;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 21); // f21 = 1.0
                c.lwc1(1, gp(c, -0x2ec8));
                c.sub_s(0, 21, 12);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    // Nearly parallel: straight lerp.
                    c.sub_s(20, 21, 22);
                    break 'lerp;
                }
                c.cycles += 17;
                c.g[RA][0] = sx(0x0031_bd84);
                call_ptr(c, 0x0031_c128, 0x0031_bd84)?; // Math_Asin(dot)
                c.lwc1(20, gp(c, -0x2ec4));
                c.sub_s(20, 20, 0); // theta = half_pi - asin(dot)
                c.g[RA][0] = sx(0x0031_bd94);
                c.mov_s(12, 20);
                call_ptr(c, 0x0031_bf60, 0x0031_bd94)?; // sin(theta)
                c.sub_s(12, 21, 22);
                c.mov_s(21, 0);
                c.g[RA][0] = sx(0x0031_bda4);
                c.mul_s(12, 12, 20);
                call_ptr(c, 0x0031_bf60, 0x0031_bda4)?; // sin((1-t)theta)
                c.mul_s(12, 22, 20);
                c.div_s(20, 0, 21);
                c.g[RA][0] = sx(0x0031_bdb4);
                call_ptr(c, 0x0031_bf60, 0x0031_bdb4)?; // sin(t*theta)
                c.div_s(12, 0, 21);
                c.lwc1(7, p(c.g[S0][0] as u32, 0xc));
                break 'weights;
            }
            c.cycles += 2;
            c.mov_s(12, 22);
            c.lwc1(7, p(c.g[S0][0] as u32, 0xc));
        }

        // dst = a * f20 + b * f12
        c.cycles += 34;
        c.g[V0][0] = c.g[S1][0];
        c.lwc1(5, c.g[S0][0] as u32);
        c.lwc1(6, p(c.g[S0][0] as u32, 4));
        c.mul_s(7, 20, 7);
        c.lwc1(4, p(c.g[S0][0] as u32, 8));
        c.mul_s(5, 20, 5);
        c.lwc1(3, c.g[SP][0] as u32);
        c.mul_s(6, 20, 6);
        c.lwc1(2, p(c.g[SP][0] as u32, 4));
        c.mul_s(4, 20, 4);
        c.lwc1(1, p(c.g[SP][0] as u32, 8));
        c.mul_s(3, 12, 3);
        c.lwc1(0, p(c.g[SP][0] as u32, 0xc));
        c.mul_s(2, 12, 2);
        c.mul_s(1, 12, 1);
        c.lq(16, p(c.g[SP][0] as u32, 0x40));
        c.mul_s(0, 12, 0);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x20));
        c.add_s(5, 5, 3);
        c.lwc1(22, p(c.g[SP][0] as u32, 0x60));
        c.add_s(6, 6, 2);
        c.lwc1(21, p(c.g[SP][0] as u32, 0x58));
        c.add_s(4, 4, 1);
        c.lwc1(20, p(c.g[SP][0] as u32, 0x50));
        c.add_s(7, 7, 0);
        c.swc1(5, p(c.g[SP][0] as u32, 0x10));
        c.swc1(6, p(c.g[SP][0] as u32, 0x14));
        c.swc1(4, p(c.g[SP][0] as u32, 0x18));
        c.swc1(7, p(c.g[SP][0] as u32, 0x1c));
        c.lq(3, p(c.g[SP][0] as u32, 0x10));
        c.sq(3, c.g[S1][0] as u32);
        c.lq(17, p(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0034d778 Fence_Render
// ---------------------------------------------------------------------------

/// `int Fence_Render(Fence* self)` (0x34d778).
///
/// Returns 1 when the fence's board object is not in the "hidden" state; 0
/// when the rider's board model is not the active type or the visual state
/// mismatches; otherwise builds the render object for the fence's locator and
/// returns the result of the draw call. `v1` carries the last visual-state word
/// on the early-out paths.
pub fn fence_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        frame.save(c, 21, 0x70);
        frame.save(c, 16, 0xc0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0xb0);
        c.g[A0][0] = 0xff;
        frame.save(c, 18, 0xa0);
        frame.save(c, 19, 0x90);
        frame.save(c, 20, 0x80);
        frame.save_ra(c, 0x60);

        'ret: {
            c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            let t = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 1;
            if t {
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(c.g[A2][0] as u32 + c.g[V0][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x24));
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lh(p(c.g[A1][0] as u32, 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx(c.g[V1][0] as u32 + c.g[A2][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3f0));
            c.g[V1][0] = c.g[V1][0] ^ 3;
            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        }

        'body: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0;
            c.g[S4][0] = c.lw(gp(c, 0x2a90));
            if t {
                break 'body;
            }
            c.cycles += 37;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[A0][0] = c.g[S2][0];
            c.lwc1(0, p(c.g[V0][0] as u32, 0x60));
            c.g[S0][0] = c.lh(p(c.g[S1][0] as u32, 0x2e8));
            c.swc1(0, c.g[SP][0] as u32);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[S0][0] = sx(c.g[S4][0] as u32 + c.g[S0][0] as u32);
            c.lwc1(0, p(c.g[V0][0] as u32, 0x64));
            c.swc1(0, p(c.g[SP][0] as u32, 4));
            c.lwc1(1, p(c.g[V0][0] as u32, 0x68));
            c.swc1(2, p(c.g[SP][0] as u32, 0xc));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.lwc1(0, p(c.g[V0][0] as u32, 0x6c));
            c.swc1(0, p(c.g[SP][0] as u32, 0x10));
            c.lwc1(1, p(c.g[V0][0] as u32, 0x70));
            c.swc1(1, p(c.g[SP][0] as u32, 0x14));
            c.lwc1(0, p(c.g[V0][0] as u32, 0x74));
            c.swc1(2, p(c.g[SP][0] as u32, 0x1c));
            c.swc1(0, p(c.g[SP][0] as u32, 0x18));
            c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x158));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x15c));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx(c.g[S4][0] as u32 + c.g[A1][0] as u32);
            call_ptr(c, t, 0x0034_d868)?;
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, t, 0x0034_d880)?;
            c.g[V1][0] = 1;
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = 0xffff_ffff_ffff_ff83;
            if t {
                break 'body;
            }
            'inner: {
              'no_anim: {
                c.cycles += 22;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.ldl(4, p(c.g[V0][0] as u32, 7));
                c.ldr(4, c.g[V0][0] as u32);
                c.ldl(6, p(c.g[V0][0] as u32, 0xf));
                c.ldr(6, p(c.g[V0][0] as u32, 8));
                c.g[A3][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                c.sdl(4, p(c.g[V0][0] as u32, 0x1b));
                c.sdr(4, p(c.g[V0][0] as u32, 0x14));
                c.sdl(6, p(c.g[V0][0] as u32, 0x23));
                c.sdr(6, p(c.g[V0][0] as u32, 0x1c));
                c.sw(p(c.g[V0][0] as u32, 0x24), c.g[A3][0]);
                c.g[A0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.g[V1][0] = sx(c.g[A0][0] as u32 + 0x14);
                c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x18));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw(p(c.g[A0][0] as u32, 0x18), c.g[V0][0]);
                c.g[A0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[T0][0] = c.lw(p(c.g[S5][0] as u32, 0x40));
                let t = c.g[T0][0] == 0;
                c.g[A1][0] = sx(c.g[A0][0] as u32 + 0x10);
                if t {
                    break 'no_anim;
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(p(c.g[T0][0] as u32, 4));
                c.g[A3][0] = 0;
                c.g[A2][0] = c.lw(p(c.g[A0][0] as u32, 0x94));
                c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x38));
                c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(c.g[T0][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_d900)?;
                let t = true;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                if t {
                    break 'inner;
                }
              }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
                c.g[A1][0] = c.g[A0][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x300));
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x304));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S4][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_d92c)?;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V0][0]);
        }

        c.cycles += 9;
        frame.restore(c, 16, 0xc0);
        frame.restore(c, 17, 0xb0);
        frame.restore(c, 18, 0xa0);
        frame.restore(c, 19, 0x90);
        frame.restore(c, 20, 0x80);
        frame.restore(c, 21, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x001b7650 cFEStateNetAccountMem_ShowPopup
// ---------------------------------------------------------------------------

/// `void cFEStateNetAccountMem_ShowPopup(void* self, void* state, int kind,
/// int text_id, int* a, int b, int c)` (0x1b7650).
///
/// Builds the account-memory popup: `NetFEState_GetPopup` supplies the popup
/// object, which is configured with one or two option strings and then shown
/// through `NetFEState_ShowPopup`.
pub fn cfe_state_net_account_mem_show_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, 23, 0x20);
        frame.save(c, 18, 0x70);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, 19, 0x60);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, 20, 0x50);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, 21, 0x40);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, 22, 0x30);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, 30, 0x10);
        c.g[S6][0] = c.g[T2][0];
        frame.save(c, 16, 0x90);
        frame.save(c, 17, 0x80);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001b_769c);
        c.g[FP][0] = c.g[A1][0];
        call_ptr(c, 0x001a_9c50, 0x001b_769c)?; // NetFEState_GetPopup
        c.g[S1][0] = c.g[V0][0];
        let t = c.g[T0][0] == 0;
        c.g[S0][0] = sx(c.g[S1][0] as u32 + 0xbc);
        'ret: {
            if t {
                break 'ret;
            }
            'two: {
                c.cycles += 2;
                let t = c.g[S3][0] == 0;
                c.g[V0][0] = 2;
                if t {
                    break 'two;
                }
                c.cycles += 11;
                c.g[A2][0] = c.g[S2][0];
                c.sw(p(c.g[S1][0] as u32, 0xbc), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b_76c4);
                c.g[A1][0] = 0;
                call_ptr(c, 0x001c_57c0, 0x001b_76c4)?;
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b_76d4);
                c.g[A1][0] = 1;
                call_ptr(c, 0x001c_57c0, 0x001b_76d4)?;
                let t = true;
                c.sw(p(c.g[S1][0] as u32, 0x168), c.g[S4][0]);
                if t {
                    break 'ret;
                }
                break 'two;
            }
            // One option only.
            c.cycles += 8;
            c.g[V0][0] = 1;
            c.g[A2][0] = c.g[S2][0];
            c.sw(p(c.g[S1][0] as u32, 0xbc), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001b_76f4);
            c.g[A1][0] = 0;
            call_ptr(c, 0x001c_57c0, 0x001b_76f4)?;
            let t = true;
            c.sw(p(c.g[S1][0] as u32, 0x168), 0);
            if t {
                break 'ret;
            }
            c.cycles += 3;
            c.sw(p(c.g[S1][0] as u32, 0xbc), 0);
            c.sw(p(c.g[S1][0] as u32, 0x164), 0);
            c.sw(p(c.g[S1][0] as u32, 0x168), 0);
            break 'ret;
        }

        'opt1: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0;
            c.g[V0][0] = 1;
            if t {
                break 'opt1;
            }
            c.cycles += 2;
            c.sw(p(c.g[S0][0] as u32, 0x10), c.g[S5][0]);
            c.sw(p(c.g[S0][0] as u32, 0xc), c.g[V0][0]);
            break 'opt1;
        }
        'opt2: {
            c.cycles += 2;
            let t = c.g[S6][0] == 0;
            c.g[V0][0] = 1;
            if t {
                break 'opt2;
            }
            c.cycles += 2;
            c.sw(p(c.g[S0][0] as u32, 8), c.g[S6][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            break 'opt2;
        }
        c.cycles += 19;
        c.sw(p(c.g[S0][0] as u32, 0x94), c.g[FP][0]);
        c.g[V0][0] = 1;
        c.sw(p(c.g[S0][0] as u32, 0xa8), 0);
        c.g[A0][0] = c.g[S7][0];
        c.sw(p(c.g[S0][0] as u32, 0x98), c.g[V0][0]);
        c.g[RA][0] = sx(0x001b_7744);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_9bc0, 0x001b_7744)?; // NetFEState_ShowPopup

        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002ebe20 TransitionEffect_ctorRandom2
// ---------------------------------------------------------------------------

/// `void* TransitionEffect_ctorRandom2(TransitionEffect* self, const Vec4* v)`
/// (0x2ebe20).
///
/// Constructs the base effect, overrides its vtable, copies the four-float
/// vector argument into `+4` and stores two random durations (0..1000 ms) at
/// `+0x14` and `+0x18`. `v0` is the new object.
pub fn transition_effect_ctor_random2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.ldl(2, p(c.g[A1][0] as u32, 7));
        c.ldr(2, c.g[A1][0] as u32);
        c.ldl(3, p(c.g[A1][0] as u32, 0xf));
        c.ldr(3, p(c.g[A1][0] as u32, 8));
        c.sdl(2, p(c.g[SP][0] as u32, 7));
        c.sdr(2, c.g[SP][0] as u32);
        c.sdl(3, p(c.g[SP][0] as u32, 0xf));
        c.sdr(3, p(c.g[SP][0] as u32, 8));
        c.g[RA][0] = sx(0x002e_be58);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002e_4d70, 0x002e_be58)?; // TransitionEffect_ctor
        c.g[V1][0] = 0x0049_0000;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x7f80));
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.ldl(2, p(c.g[SP][0] as u32, 7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(3, p(c.g[SP][0] as u32, 0xf));
        c.ldr(3, p(c.g[SP][0] as u32, 8));
        c.sdl(2, p(c.g[S0][0] as u32, 0xb));
        c.sdr(2, p(c.g[S0][0] as u32, 4));
        c.sdl(3, p(c.g[S0][0] as u32, 0x13));
        c.sdr(3, p(c.g[S0][0] as u32, 0xc));
        c.g[RA][0] = sx(0x002e_be8c);
        call_ptr(c, 0x0031_77f0, 0x002e_be8c)?; // BXrand
        c.g[V1][0] = 0x3e8;
        c.g[A0][0] = sx(c.g[S0][0] as u32 + 0x14);
        c.divu(0, 2, 3);

        // 1000 * rand() / 0x7fff, as a signed conversion.
        'ms: {
            'conv: {
                c.cycles += 3;
                c.g[V0][0] = c.hi[0];
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t {
                    break 'conv;
                }
                c.cycles += 4;
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1);
                let t = true;
                if t {
                    break 'ms;
                }
                break 'conv;
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            break 'ms;
        }
        'second: {
            c.cycles += 9;
            c.lwc1(0, gp(c, -0x3994));
            c.mul_s(0, 1, 0);
            c.g[RA][0] = sx(0x002e_bee0);
            c.swc1(0, c.g[A0][0] as u32);
            call_ptr(c, 0x0031_77f0, 0x002e_bee0)?; // BXrand
            c.g[V1][0] = 0x3e8;
            c.g[A0][0] = sx(c.g[S0][0] as u32 + 0x18);
            c.divu(0, 2, 3);
            'ms2: {
                'conv2: {
                    c.cycles += 3;
                    c.g[V0][0] = c.hi[0];
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    if t {
                        break 'conv2;
                    }
                    c.cycles += 4;
                    c.mtc1(2, 1);
                    c.cvt_s_w(1, 1);
                    let t = true;
                    if t {
                        break 'ms2;
                    }
                    break 'conv2;
                }
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1);
                c.add_s(1, 1, 1);
                break 'ms2;
            }
            c.cycles += 8;
            c.lwc1(0, gp(c, -0x3990));
            c.g[V0][0] = c.g[S0][0];
            c.lq(16, p(c.g[SP][0] as u32, 0x20));
            c.mul_s(0, 1, 0);
            c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
            c.swc1(0, c.g[A0][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.pc = t;
            Ok(())
        }
    })
}

// ---------------------------------------------------------------------------
// 0x003a13e0 cUITextScroll_HandleInput
// ---------------------------------------------------------------------------

/// `int cUITextScroll_HandleInput(cUITextScroll* self, void* input)` (0x3a13e0).
///
/// Keyboard handling of a scrolling text box. The row count comes from the
/// box height divided by the scaled font height plus three; then, depending on
/// the "cursor mode" flag, either the cursor moves within the list (and the
/// visible top follows it) or the visible top scrolls and the cursor stays
/// put. Returns 1 when a key was consumed.
pub fn cuit_ext_scroll_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003a_1400);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x003a_12d0, 0x003a_1400)?; // cUITextScroll_GetFont
        let self_ = c.g[S0][0] as u32;
        let input = c.g[S1][0] as u32;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore(c, 16, 0x30);
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let font = c.g[V0][0] as u32;

        'ret: {
            'ret0: {
                'ret1: {
                    'scroll: {
                        c.cycles += 18;
                        c.lwc1(2, p(font, 0x14));
                        c.cvt_s_w(2, 2);
                        c.lwc1(3, p(font, 0x34));
                        c.lwc1(0, p(self_, 0x54));
                        c.g[AT][0] = 0x4040_0000; // 3.0
                        c.mtc1(1, 4);
                        c.mul_s(2, 2, 3);
                        c.lwc1(1, p(self_, 0x64));
                        c.g[V1][0] = c.lw(p(self_, 0x74));
                        c.mul_s(0, 0, 2);
                        c.g[V1][0] = c.g[V1][0] & 1;
                        c.add_s(0, 0, 4);
                        c.div_s(1, 1, 0);
                        c.cvt_w_s(0, 1);
                        c.mfc1(2, 0);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        let cursor_mode = c.g[V1][0] == 0;
                        c.g[S2][0] = sx((((c.g[V0][0] as u32) as i32) >> 24) as u32);
                        let rows = c.g[S2][0] as i32;

                        if cursor_mode {
                            break 'scroll;
                        }
                        // --- cursor mode: up ---------------------------------
                        c.cycles += 8;
                        c.g[V1][0] = c.lw(p(input, 8));
                        c.g[A1][0] = c.lb(p(self_, 0x88));
                        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x003a_1468)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu(p(self_, 0xc4));
                            let empty = c.g[V0][0] == 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            if empty {
                                break 'ret1;
                            }
                            c.cycles += 2;
                            c.sh(p(self_, 0xc4), c.g[V0][0]);
                            break 'ret1;
                        }
                        // --- cursor mode: down -------------------------------
                        c.g[V1][0] = c.lw(p(input, 8));
                        c.cycles += 7;
                        c.g[A1][0] = c.lb(p(self_, 0x89));
                        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x003a_1498)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 6;
                            c.g[V0][0] = c.lhu(p(self_, 0xc8));
                            c.g[V1][0] = c.lhu(p(self_, 0xc4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let last = c.g[V0][0] == 0;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                            if last {
                                break 'ret1;
                            }
                            c.cycles += 2;
                            c.sh(p(self_, 0xc4), c.g[V0][0]);
                            break 'ret1;
                        }
                        // Neither key: keep the cursor visible.
                        c.g[V1][0] = c.lhu(p(self_, 0xc4));
                        c.cycles += 6;
                        c.g[V0][0] = c.lhu(p(self_, 0xc6));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 16) as u32);
                        let above = (c.g[V0][0] as i64) >= 0;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        if !above {
                            c.cycles += 2;
                            c.sh(p(self_, 0xc6), c.g[V1][0]);
                            break 'ret0;
                        }
                        c.cycles += 2;
                        let visible = c.g[V0][0] != 0;
                        c.g[V0][0] = 0;
                        if visible {
                            break 'ret;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        c.sh(p(self_, 0xc6), c.g[V0][0]);
                        let _ = rows;
                        break 'ret0;
                    }
                    // --- scroll mode: up ------------------------------------
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(p(input, 8));
                    c.g[A1][0] = c.lb(p(self_, 0x88));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                    let f = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x003a_1510)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu(p(self_, 0xc6));
                        let empty = c.g[V0][0] == 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if empty {
                            break 'ret1;
                        }
                        c.cycles += 2;
                        c.sh(p(self_, 0xc6), c.g[V0][0]);
                        break 'ret1;
                    }
                    // --- scroll mode: down ----------------------------------
                    c.g[V1][0] = c.lw(p(input, 8));
                    c.cycles += 7;
                    c.g[A1][0] = c.lb(p(self_, 0x89));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                    let f = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x003a_1540)?;
                    let down = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if down {
                        break 'ret;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lhu(p(self_, 0xc8));
                    c.g[V1][0] = c.lhu(p(self_, 0xc6));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let full = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    if full {
                        break 'ret1;
                    }
                    c.cycles += 1;
                    c.sh(p(self_, 0xc6), c.g[V0][0]);
                    break 'ret1;
                }
                // Return 1.
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'ret;
            }
            // Return 0.
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'ret;
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002d0d48 MenuSysPoint2DItem_Measure
// ---------------------------------------------------------------------------

/// `void* MenuSysPoint2DItem_Measure(void* item, void* page)` (0x2d0d48).
///
/// Measures a point item: when the item is the page's selected item the height
/// grows by the selection margin, then the item's text is scaled to the
/// available width and the text's box (16 bytes of floats) is written back to
/// the item. `v0` is the item.
pub fn menu_sys_point2d_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        c.g[RA][0] = sx(0x002d_0d7c);
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        call_ptr(c, 0x002c_bf30, 0x002d_0d7c)?; // MenuSysPage_GetSelectedItem
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[S0][0] != c.g[V0][0];
        if !t {
            c.cycles += 2;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x40));
            c.g[S2][0] = (0 < c.g[V0][0]) as u64;
        }

        c.cycles += 31;
        c.g[V0][0] = 0x50;
        c.g[S1][0] = 0x14;
        c.g[A1][0] = c.lw(p(c.g[S0][0] as u32, 0x14));
        c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0x124));
        if c.g[S2][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x58));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x14));
        c.cvt_s_w(1, 1);
        c.g[A0][0] = c.g[V0][0];
        c.lwc1(0, p(c.g[V0][0] as u32, 0x34));
        c.lwc1(12, p(c.g[A0][0] as u32, 0x38));
        c.lwc1(13, p(c.g[A0][0] as u32, 0x3c));
        c.g[RA][0] = sx(0x002d_0dcc);
        c.mul_s(21, 1, 0);
        call_ptr(c, 0x0039_1fb0, 0x002d_0dcc)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = 0x0048_0000;
        c.mov_s(20, 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x6748));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x124));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 0x58));
        c.lwc1(12, p(c.g[A0][0] as u32, 0x38));
        c.g[RA][0] = sx(0x002d_0df8);
        c.lwc1(13, p(c.g[A0][0] as u32, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x002d_0df8)?;
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.mov_s(20, 0);
        }

        'height: {
            'half: {
                c.cycles += 4;
                c.mtc1(0, 0);
                c.swc1(0, c.g[SP][0] as u32);
                let t = c.g[S2][0] == 0;
                c.swc1(0, p(c.g[SP][0] as u32, 4));
                if t {
                    break 'half;
                }
                'clamp: {
                    c.cycles += 9;
                    c.add_s(1, 21, 21);
                    c.g[AT][0] = 0x40c0_0000;
                    c.mtc1(1, 0);
                    c.mtc1(17, 2);
                    c.cvt_s_w(2, 2);
                    c.add_s(1, 1, 0);
                    c.c_lt_s(1, 2);
                    let t = !c.fcc();
                    if t {
                        break 'clamp;
                    }
                    c.cycles += 4;
                    c.g[AT][0] = 0x41c0_0000;
                    c.mtc1(1, 0);
                    c.add_s(1, 2, 0);
                    break 'clamp;
                }
                c.cycles += 3;
                c.g[AT][0] = 0x41c0_0000;
                c.mtc1(1, 0);
                c.add_s(1, 1, 0);
                c.cycles += 2;
                c.swc1(1, p(c.g[SP][0] as u32, 0xc));
                break 'height;
            }
            // Not selected: the plain text box.
            c.cycles += 11;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[AT][0] = 0x4140_0000;
            c.mtc1(1, 2);
            c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x124));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x58));
            c.lwc1(0, p(c.g[V1][0] as u32, 0x14));
            c.cvt_s_w(0, 0);
            c.lwc1(1, p(c.g[V1][0] as u32, 0x34));
            c.mul_s(0, 0, 1);
            c.add_s(0, 0, 2);
            c.swc1(0, p(c.g[SP][0] as u32, 0xc));
            break 'height;
        }

        c.cycles += 26;
        c.g[V0][0] = 0x64;
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(1, 1);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        c.g[V0][0] = c.g[S3][0];
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.add_s(0, 0, 20);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.lwc1(21, p(c.g[SP][0] as u32, 0x68));
        c.lwc1(20, p(c.g[SP][0] as u32, 0x60));
        c.add_s(0, 0, 1);
        c.swc1(0, p(c.g[SP][0] as u32, 8));
        c.ldl(3, p(c.g[SP][0] as u32, 7));
        c.ldr(3, c.g[SP][0] as u32);
        c.ldl(4, p(c.g[SP][0] as u32, 0xf));
        c.ldr(4, p(c.g[SP][0] as u32, 8));
        c.sdl(3, p(c.g[S3][0] as u32, 7));
        c.sdr(3, c.g[S3][0] as u32);
        c.sdl(4, p(c.g[S3][0] as u32, 0xf));
        c.sdr(4, p(c.g[S3][0] as u32, 8));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00347b80 FlexBridge_Render
// ---------------------------------------------------------------------------

/// `int FlexBridge_Render(FlexBridge* self)` (0x347b80).
///
/// Renders the flexing bridge segment: pushes the draw state, asks the
/// renderer's dispatch table for the segment transform, builds the two vertex
/// arrays and submits them, then restores the state. Returns 0.
pub fn flex_bridge_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        frame.save(c, 21, 0x50);
        frame.save(c, 16, 0xa0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0x90);
        c.g[A0][0] = 0xff;
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save_ra(c, 0x40);

        'ret: {
            c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            let t = c.g[V1][0] == c.g[A0][0];
            c.g[V0][0] = 1;
            if t {
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lbu(p(c.g[A1][0] as u32, 0x7d));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx(c.g[A2][0] as u32 + c.g[V0][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x24));
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0;
                break 'ret;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lh(p(c.g[A1][0] as u32, 0x7e));
            c.g[A0][0] = 0x18;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx(c.g[V1][0] as u32 + c.g[A2][0] as u32);
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x3f0));
            c.g[V1][0] = c.g[V1][0] ^ 3;
            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
        }

        'body: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0;
            c.g[S4][0] = c.lw(gp(c, 0x2a90));
            if t {
                break 'body;
            }
            c.cycles += 19;
            c.g[A0][0] = c.g[SP][0];
            c.g[S2][0] = sx(c.g[S5][0] as u32 + 0x90);
            c.g[S3][0] = sx(c.g[S5][0] as u32 + 0xa0);
            c.g[S0][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
            c.g[S1][0] = c.lh(p(c.g[S0][0] as u32, 0x2e8));
            c.g[A1][0] = c.lh(p(c.g[S0][0] as u32, 0x158));
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x15c));
            c.g[S1][0] = sx(c.g[S4][0] as u32 + c.g[S1][0] as u32);
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = sx(c.g[S4][0] as u32 + c.g[A1][0] as u32);
            call_ptr(c, t, 0x0034_7c28)?;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            let t = c.g[V0][0] as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, t, 0x0034_7c40)?;
            c.g[V1][0] = 1;
            let t = c.g[V0][0] == c.g[V1][0];
            c.lq(16, p(c.g[SP][0] as u32, 0xa0));
            if t {
                break 'body;
            }
            'inner: {
              'no_anim: {
                c.cycles += 28;
                c.g[V1][0] = c.lw(p(c.g[S5][0] as u32, 0xc));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0xc0));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0xc4));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S5][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7c60)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0xffff_ffff_ffff_ff83;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.ldl(4, p(c.g[V0][0] as u32, 7));
                c.ldr(4, c.g[V0][0] as u32);
                c.ldl(7, p(c.g[V0][0] as u32, 0xf));
                c.ldr(7, p(c.g[V0][0] as u32, 8));
                c.g[T0][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                c.sdl(4, p(c.g[V0][0] as u32, 0x1b));
                c.sdr(4, p(c.g[V0][0] as u32, 0x14));
                c.sdl(7, p(c.g[V0][0] as u32, 0x23));
                c.sdr(7, p(c.g[V0][0] as u32, 0x1c));
                c.sw(p(c.g[V0][0] as u32, 0x24), c.g[T0][0]);
                c.g[A0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                c.g[V1][0] = sx(c.g[A0][0] as u32 + 0x14);
                c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x18));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw(p(c.g[A0][0] as u32, 0x18), c.g[V0][0]);
                c.g[T1][0] = c.lw(p(c.g[S5][0] as u32, 0x78));
                let t = c.g[T1][0] == 0;
                c.g[A1][0] = c.g[A2][0];
                if t {
                    break 'no_anim;
                }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[T1][0] as u32, 4));
                c.g[T0][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x38));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x3c));
                c.g[A2][0] = c.lw(p(c.g[T0][0] as u32, 0x94));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[T1][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7cdc)?;
                let t = true;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                if t {
                    break 'inner;
                }
              }
                c.cycles += 10;
                c.g[V1][0] = c.lw(p(c.g[S4][0] as u32, 0x10d8));
                c.g[A2][0] = 0;
                c.g[A1][0] = c.lw(p(c.g[S5][0] as u32, 0x18));
                c.g[A3][0] = 0;
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x300));
                c.g[T0][0] = 0;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x304));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(c.g[S4][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0034_7d08)?;
                c.g[V0][0] = c.lw(p(c.g[S4][0] as u32, 0xe84));
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
            c.sw(p(c.g[S4][0] as u32, 0xe84), c.g[V0][0]);
            break 'body;
        }
        c.cycles += 1;
        c.lq(16, p(c.g[SP][0] as u32, 0xa0));

        c.cycles += 8;
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LobbyMan reply handlers
// ---------------------------------------------------------------------------

/// The shared body of the `LobbyMan_On*Reply` handlers (0x25c0c0 / 0x25c2d8 /
/// 0x25c4f0).
///
/// Marks the handler consumed (`self->0 = 1`), builds a reply message — either
/// the short `{tag, D_004812B0}` one when the reply carries no text, or the
/// `{0x102, D_00481290, text}` one — and posts it through the lobby's message
/// list at `self + 0xf0`. `tag` is the message id that distinguishes the
/// handlers.
fn lobby_send_reply(c: &mut Cpu, tag: u64, ret1: u32, ret2: u32, ret3: u32) -> crate::call::Flow {
    let frame = Frame::open(c, 0x60);
    c.cycles += 12;
    c.g[V0][0] = 1;
    frame.save(c, 17, 0x40);
    frame.save(c, 18, 0x30);
    c.g[S1][0] = c.g[A0][0]; // self
    frame.save(c, 16, 0x50);
    c.g[S2][0] = c.g[A1][0]; // reply
    frame.save_ra(c, 0x20);
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    c.g[V0][0] = c.lw(p(c.g[S2][0] as u32, 8));
    let has_text = c.g[V0][0] != 0;
    c.g[A1][0] = 0x0048_0000;

    'tail: {
        if !has_text {
            // Short message.
            c.cycles += 19;
            c.g[A2][0] = 0x2000_0000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A0][0] = 8;
            c.g[RA][0] = sx(ret1);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, ret1)?; // cMemMan_alloc
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 0xf0));
            c.g[V0][0] = 0x0048_0000;
            c.g[V1][0] = tag;
            c.g[V0][0] = sx(0x0048_12b0);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[A1][0] = 0x0048_0000;
            c.sw(p(c.g[SP][0] as u32, 0x10), c.g[A2][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            break 'tail;
        }
        // Message with its text pointer.
        c.cycles += 20;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(ret2);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, ret2)?; // cMemMan_alloc
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(p(c.g[S2][0] as u32, 8));
        c.g[V0][0] = 0x0048_0000;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 0xf0));
        c.g[V0][0] = sx(0x0048_1290);
        c.g[V1][0] = 0x102;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[A0][0]);
        c.g[A1][0] = 0x0048_0000;
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.sw(p(c.g[SP][0] as u32, 0x10), c.g[A2][0]);
        c.g[A3][0] = 0;
        c.g[A2][0] = 0x2000_0000;
    }

    // Link the message into the list at self+0xf0.
    'link: {
        c.cycles += 6;
        c.g[RA][0] = sx(ret3);
        call_ptr(c, 0x0031_7d70, ret3)?; // cMemMan_alloc
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p(c.g[A1][0] as u32, 8), c.g[S0][0]);
            break 'link;
        }
        break 'link;
    }
    c.cycles += 15;
    c.g[V0][0] = c.lw(p(c.g[SP][0] as u32, 0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 4));
    c.sw(p(c.g[A1][0] as u32, 4), c.g[V1][0]);
    c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw(p(c.g[SP][0] as u32, 0x10));
    c.sw(p(c.g[V0][0] as u32, 4), c.g[A1][0]);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_OnEditAccountReply(LobbyMan* self, Reply* r)` (0x25c0c0):
/// posts the "account updated" message.
pub fn lobby_man_on_edit_account_reply(c: &mut Cpu) {
    guest(c, |c| lobby_send_reply(c, 0xce, 0x0025_c104, 0x0025_c150, 0x0025_c194))
}

/// `void LobbyMan_OnLostByEmailReply(LobbyMan* self, Reply* r)` (0x25c2d8):
/// posts the "account not found by e-mail" message.
pub fn lobby_man_on_lost_by_email_reply(c: &mut Cpu) {
    guest(c, |c| lobby_send_reply(c, 0xcf, 0x0025_c31c, 0x0025_c368, 0x0025_c3ac))
}

/// `void LobbyMan_OnLostByNameReply(LobbyMan* self, Reply* r)` (0x25c4f0):
/// posts the "account not found by name" message.
pub fn lobby_man_on_lost_by_name_reply(c: &mut Cpu) {
    guest(c, |c| lobby_send_reply(c, 0xd2, 0x0025_c534, 0x0025_c580, 0x0025_c5c4))
}

// ---------------------------------------------------------------------------
// 0x0018eb40 FEScreenSelectProfile_OnWidgetAction
// ---------------------------------------------------------------------------

/// `void FEScreenSelectProfile_OnWidgetAction(void* self, void* menu,
/// int action)` (0x18eb40).
///
/// Handles a widget action on a profile-select menu row: the menu object is
/// recoloured (white for "activate", magenta for "cancel"), and for the two
/// item actions the widget's own action method is called and its result
/// pushed into the front-end item list at `*(self+0x10) + 0x18`.
pub fn fescreen_select_profile_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A1][0]; // menu
        frame.save_ra(c, 0x10);
        let no_menu = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0]; // self
        'skip: {
            if !no_menu {
            'actions: {
                'item: {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    let is_two = c.g[A2][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[A2][0] < 3) as u64;
                    if is_two {
                        // Case 2: white.
                        c.cycles += 12;
                        c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
                        c.g[A1][0] = c.g[SP][0];
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
                        c.swc1(0, p(c.g[SP][0] as u32, 0xc));
                        c.swc1(0, c.g[SP][0] as u32);
                        c.swc1(0, p(c.g[SP][0] as u32, 4));
                        c.swc1(0, p(c.g[SP][0] as u32, 8));
                        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0018_ec8c);
                        c.g[A0][0] = sx(c.g[S0][0] as u32 + c.g[A0][0] as u32);
                        call_ptr(c, t, 0x0018_ec8c)?;
                        break 'skip;
                    }
                    'low: {
                        c.cycles += 2;
                        let below3 = c.g[V0][0] == 0; // action >= 3
                        c.g[V0][0] = 1;
                        if below3 {
                            break 'low;
                        }
                        c.cycles += 2;
                        if c.g[A2][0] == c.g[V0][0] {
                            // Case 1: magenta.
                            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
                            break 'item;
                        }
                        c.cycles += 2;
                        c.lq(16, p(c.g[SP][0] as u32, 0x30));
                        c.cycles += 4;
                        frame.restore(c, 17, 0x20);
                        frame.restore_ra(c, 0x10);
                        frame.close(c);
                        return Ok(());
                    }
                    // Cases 5 and 6: run the menu item's action method.
                    'five_six: {
                        'five: {
                            'six: {
                                c.cycles += 3;
                                c.g[V0][0] = 5;
                                if c.g[A2][0] == c.g[V0][0] {
                                    c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
                                    break 'six;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 6;
                                if c.g[A2][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                                    break 'five;
                                }
                                c.cycles += 2;
                                c.lq(16, p(c.g[SP][0] as u32, 0x30));
                                c.cycles += 4;
                                frame.restore(c, 17, 0x20);
                                frame.restore_ra(c, 0x10);
                                frame.close(c);
                                return Ok(());
                            }
                            // Case 5: yellow highlight plus the action call.
                            c.cycles += 19;
                            c.g[A1][0] = c.g[SP][0];
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(1, 0);
                            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
                            c.swc1(0, p(c.g[SP][0] as u32, 4));
                            c.swc1(0, c.g[SP][0] as u32);
                            c.sw(p(c.g[SP][0] as u32, 8), 0);
                            c.sw(p(c.g[SP][0] as u32, 0xc), 0);
                            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0018_ebcc);
                            c.g[A0][0] = sx(c.g[S0][0] as u32 + c.g[A0][0] as u32);
                            call_ptr(c, t, 0x0018_ebcc)?;
                            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw(p(c.g[S0][0] as u32, 0x18));
                            c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V1][0] = c.lw(p(c.g[A3][0] as u32, 4));
                            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x20));
                            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x24));
                            break 'five_six;
                        }
                        // Case 6: just the action call.
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.lw(p(c.g[S0][0] as u32, 0x18));
                        c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw(p(c.g[A3][0] as u32, 4));
                        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x28));
                        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x2c));
                    }
                    // Shared tail: call the item action and link the result.
                    c.cycles += 4;
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0018_ec0c);
                    c.g[A0][0] = sx(c.g[A3][0] as u32 + c.g[A0][0] as u32);
                    call_ptr(c, t, 0x0018_ec0c)?;
                    let none = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if none {
                        break 'skip;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                    c.g[RA][0] = sx(0x0018_ec20);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0018_ec20)?;
                    c.lq(16, p(c.g[SP][0] as u32, 0x30));
                    c.cycles += 4;
                    frame.restore(c, 17, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                // Case 1: magenta.
                c.cycles += 13;
                c.g[A1][0] = c.g[SP][0];
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
                c.swc1(0, p(c.g[SP][0] as u32, 8));
                c.swc1(0, c.g[SP][0] as u32);
                c.sw(p(c.g[SP][0] as u32, 4), 0);
                c.sw(p(c.g[SP][0] as u32, 0xc), 0);
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0018_ec54);
                c.g[A0][0] = sx(c.g[S0][0] as u32 + c.g[A0][0] as u32);
                call_ptr(c, t, 0x0018_ec54)?;
                break 'skip;
            }
            }
        }
        c.cycles += 1;
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.cycles += 4;
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x001c70f0 cFEPopupConfirm_ResizeTextWidgets
// ---------------------------------------------------------------------------

/// One child-widget resize pass of `cFEPopupConfirm_ResizeTextWidgets`: asks
/// the widget for its two sizes, stores the non-zero ones into the stack size
/// vector and calls the widget's resize method.
fn resize_child(c: &mut Cpu, widget_slot: u32, first_charge: u64, resize_arg: u64, ret1: u32, ret2: u32) -> crate::call::Flow {
    let obj = c.g[S1][0] as u32;
    c.g[A2][0] = c.lw(p(obj, widget_slot));
    c.g[A1][0] = c.g[SP][0];
    if c.g[A2][0] == 0 {
        return Ok(());
    }
    c.cycles += first_charge;
    c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa0));
    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xa4));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
    call_ptr(c, t, ret1)?;
    let size = c.g[S0][0] as u32;
    c.lwc1(0, size);
    c.mtc1(0, 1);
    c.c_eq_s(0, 1);
    if !c.fcc() {
        c.swc1(0, c.g[SP][0] as u32);
    }
    c.cycles += 5;
    c.lwc1(0, p(size, 4));
    c.c_eq_s(0, 1);
    if !c.fcc() {
        c.swc1(0, p(c.g[SP][0] as u32, 4));
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw(p(obj, widget_slot));
    c.g[A1][0] = resize_arg;
    c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa8));
    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xac));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
    call_ptr(c, t, ret2)
}

/// `void cFEPopupConfirm_ResizeTextWidgets(cFEPopupConfirm* self,
/// const Vec2* size)` (0x1c70f0).
///
/// Re-lays out the popup's three text widgets to the given size, then asks the
/// popup to resize itself. The size vector is rebuilt on the stack, skipping
/// zero components.
pub fn cfe_popup_confirm_resize_text_widgets(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let obj = c.g[A0][0] as u32;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = sx(obj);
        c.g[A2][0] = c.lw(p(obj, 0x74));
        let size = c.g[A1][0] as u32;
        c.g[S0][0] = sx(size);
        if c.g[A2][0] != 0 {
            c.cycles += 12;
            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa0));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xa4));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
            call_ptr(c, t, 0x001c_7128)?;
            c.lwc1(0, size);
            c.mtc1(0, 1);
            c.c_eq_s(0, 1);
            if !c.fcc() {
                c.swc1(0, c.g[SP][0] as u32);
            }
            c.cycles += 5;
            c.lwc1(0, p(size, 4));
            c.c_eq_s(0, 1);
            if !c.fcc() {
                c.swc1(0, p(c.g[SP][0] as u32, 4));
            }
            c.cycles += 7;
            c.g[A2][0] = c.lw(p(obj, 0x74));
            c.g[A1][0] = sx(size);
            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0xa8));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xac));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
            call_ptr(c, t, 0x001c_7170)?;
        }

        c.cycles += 3;
        let size_vec = c.g[SP][0];
        resize_child(c, 0x78, 11, size_vec, 0x001c_7190, 0x001c_71d8)?;
        c.cycles += 3;
        resize_child(c, 0x7c, 11, c.g[S0][0], 0x001c_71f8, 0x001c_7240)?;

        c.cycles += 5;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0034d778 Fence_Render
// ---------------------------------------------------------------------------


// ---------------------------------------------------------------------------
// 0x002e4d88 SnowBox_ctor
// ---------------------------------------------------------------------------

/// `SnowBox* SnowBox_ctor(SnowBox* self, const Vec4* a, const Vec4* b)`
/// (0x2e4d88).
///
/// Initialises the snow-box: clears its flag block, copies the two vectors into
/// `+0xc..+0x1c`, seeds three random values, builds the box orientation from
/// the identity matrix at `0x4ff1a0` (scaled by `+0xc`) and stores the unit
/// quadword at `+0x20`. `v0` is the object.
pub fn snow_box_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = 1;
        frame.save_fpr(c, 20, 0xb0);
        frame.save(c, 16, 0xa0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x90);
        let obj = c.g[S0][0] as u32;
        c.sw(p(obj, 8), 0);
        c.sw(p(obj, 0x10), 0);
        c.sw(p(obj, 0x20), 0);
        c.sw(p(obj, 0x24), 0);
        c.sw(p(obj, 0x28), 0);
        c.swc1(20, p(obj, 0x2c));
        c.sw(p(obj, 0x30), 0);
        c.sw(p(obj, 0x34), 0);
        c.sw(p(obj, 0x38), 0);
        c.sw(p(obj, 0x3c), 0);
        c.sw(p(obj, 0x40), 0);
        c.sw(p(obj, 0x44), 0);
        c.sw(p(obj, 0x48), 0);
        c.sw(p(obj, 0x4c), 0);
        c.sw(p(obj, 0x90), 0);
        c.sw(p(obj, 0x94), 0);
        c.swc1(20, p(obj, 0xa4));
        c.swc1(20, p(obj, 0xa8));
        c.swc1(20, p(obj, 0xac));
        c.sw(obj, c.g[A1][0]);
        c.sw(p(obj, 4), c.g[A2][0]);
        c.swc1(12, p(obj, 0xc));
        c.swc1(13, p(obj, 0x14));
        c.swc1(14, p(obj, 0x18));
        c.swc1(15, p(obj, 0x1c));
        c.g[RA][0] = sx(0x002e_4e14);
        c.sw(p(obj, 0xb0), c.g[V0][0]);
        call_ptr(c, 0x0031_77f0, 0x002e_4e14)?; // BXrand
        c.g[RA][0] = sx(0x002e_4e1c);
        c.sw(p(obj, 0x98), c.g[V0][0]);
        call_ptr(c, 0x0031_77f0, 0x002e_4e1c)?; // BXrand
        c.g[RA][0] = sx(0x002e_4e24);
        c.sw(p(obj, 0x9c), c.g[V0][0]);
        call_ptr(c, 0x0031_77f0, 0x002e_4e24)?; // BXrand
        c.g[V1][0] = 0x0050_0000;
        c.sw(p(obj, 0xa0), c.g[V0][0]);
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60)); // 0x4ff1a0
        c.g[V0][0] = sx(p(obj, 0x50));

        // The base matrix, copied quadruple by quadruple.
        c.lqc2(1, c.g[V1][0] as u32);
        c.lqc2(2, p(c.g[V1][0] as u32, 0x10));
        c.lqc2(3, p(c.g[V1][0] as u32, 0x20));
        c.lqc2(4, p(c.g[V1][0] as u32, 0x30));
        c.sqc2(1, c.g[V0][0] as u32);
        c.sqc2(2, p(c.g[V0][0] as u32, 0x10));
        c.sqc2(3, p(c.g[V0][0] as u32, 0x20));
        c.sqc2(4, p(c.g[V0][0] as u32, 0x30));
        c.lwc1(0, p(obj, 0xc));
        c.g[V1][0] = sx(p(c.g[SP][0] as u32, 0x50));
        c.swc1(20, p(c.g[SP][0] as u32, 0x4c));
        c.swc1(0, p(c.g[SP][0] as u32, 0x48));
        c.swc1(0, p(c.g[SP][0] as u32, 0x40));
        c.swc1(0, p(c.g[SP][0] as u32, 0x44));
        c.lqc2(1, p(c.g[SP][0] as u32, 0x40));
        c.lqc2(4, c.g[V0][0] as u32);
        c.lqc2(5, p(c.g[V0][0] as u32, 0x10));
        c.lqc2(6, p(c.g[V0][0] as u32, 0x20));
        c.lqc2(7, p(c.g[V0][0] as u32, 0x30));
        c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
        c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
        c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
        c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
        c.sqc2(8, c.g[V1][0] as u32);
        c.sqc2(9, p(c.g[V1][0] as u32, 0x10));
        c.sqc2(10, p(c.g[V1][0] as u32, 0x20));
        c.sqc2(11, p(c.g[V1][0] as u32, 0x30));
        c.lqc2(1, c.g[V1][0] as u32);
        c.lqc2(2, p(c.g[V1][0] as u32, 0x10));
        c.lqc2(3, p(c.g[V1][0] as u32, 0x20));
        c.lqc2(4, p(c.g[V1][0] as u32, 0x30));
        c.sqc2(1, c.g[SP][0] as u32);
        c.sqc2(2, p(c.g[SP][0] as u32, 0x10));
        c.sqc2(3, p(c.g[SP][0] as u32, 0x20));
        c.sqc2(4, p(c.g[SP][0] as u32, 0x30));
        c.lqc2(1, c.g[SP][0] as u32);
        c.lqc2(2, p(c.g[SP][0] as u32, 0x10));
        c.lqc2(3, p(c.g[SP][0] as u32, 0x20));
        c.lqc2(4, p(c.g[SP][0] as u32, 0x30));
        c.sqc2(1, c.g[V0][0] as u32);
        c.sqc2(2, p(c.g[V0][0] as u32, 0x10));
        c.sqc2(3, p(c.g[V0][0] as u32, 0x20));
        c.sqc2(4, p(c.g[V0][0] as u32, 0x30));
        c.swc1(20, p(c.g[SP][0] as u32, 0xc));
        c.g[V0][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, 0);
        c.sw(p(c.g[SP][0] as u32, 4), 0);
        c.sw(p(c.g[SP][0] as u32, 8), 0);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x90));
        c.lq(3, c.g[SP][0] as u32);
        c.lwc1(20, p(c.g[SP][0] as u32, 0xb0));
        c.sq(3, p(obj, 0x20));
        c.lq(16, p(c.g[SP][0] as u32, 0xa0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00330540 CollSphereTree_OverlapsAABB
// ---------------------------------------------------------------------------

/// `int CollSphereTree_OverlapsAABB(CollSphereTree* self, const Vec3* lo,
/// const Vec3* hi)` (0x330540).
///
/// Builds the tree's world box (centre at `self->[0x60] + 0x80` grown by the
/// three scales the object reports) and tests the two corners against it, one
/// axis at a time; returns 1 when the box overlaps.
pub fn coll_sphere_tree_overlaps_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 16, 0x70);
        frame.save(c, 17, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x88);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x80);
        frame.save_ra(c, 0x40);
        let obj = c.g[S0][0] as u32;
        let lo = c.g[S1][0] as u32;
        let hi = c.g[S2][0] as u32;
        c.g[RA][0] = sx(0x0033_0570);
        c.g[A0][0] = c.lw(p(obj, 0x60));
        call_ptr(c, 0x0032_c590, 0x0033_0570)?;
        c.g[A0][0] = c.lw(p(obj, 0x60));
        c.g[RA][0] = sx(0x0033_057c);
        c.mov_s(21, 0);
        call_ptr(c, 0x0032_c590, 0x0033_057c)?;
        c.g[A0][0] = c.lw(p(obj, 0x60));
        c.g[RA][0] = sx(0x0033_0588);
        c.mov_s(20, 0);
        call_ptr(c, 0x0032_c590, 0x0033_0588)?;
        c.swc1(21, c.g[SP][0] as u32);
        c.swc1(20, p(c.g[SP][0] as u32, 4));
        c.swc1(0, p(c.g[SP][0] as u32, 8));
        c.sw(p(c.g[SP][0] as u32, 0xc), 0);

        // centre + extent
        c.g[V0][0] = c.lw(p(obj, 0x60));
        c.lqc2(3, p(c.g[V0][0] as u32, 0x80));
        c.lqc2(4, c.g[SP][0] as u32);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(c.g[SP][0] as u32, 0x20));
        c.lq(3, p(c.g[SP][0] as u32, 0x20));
        c.g[V0][0] = c.lw(p(obj, 0x60));
        c.sq(3, p(c.g[SP][0] as u32, 0x10));
        // centre - extent
        c.lqc2(3, p(c.g[V0][0] as u32, 0x80));
        c.lqc2(4, c.g[SP][0] as u32);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(c.g[SP][0] as u32, 0x30));

        'ret: {
            c.lwc1(1, lo);
            c.lwc1(2, p(c.g[SP][0] as u32, 0x10));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.c_lt_s(2, 1);
            let no_lo = !c.fcc();
            c.sq(2, p(c.g[SP][0] as u32, 0x20));
            if no_lo {
                // X low edge not separated.
            } else {
                c.cycles += 5;
                c.lwc1(0, p(c.g[SP][0] as u32, 0x20));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'x_hi: {
                c.cycles += 5;
                c.lwc1(1, hi);
                c.c_lt_s(1, 2);
                let no_hi = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x20));
                if no_hi {
                    break 'x_hi;
                }
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'y_lo: {
                c.cycles += 6;
                c.lwc1(1, p(lo, 4));
                c.lwc1(2, p(c.g[SP][0] as u32, 0x14));
                c.c_lt_s(2, 1);
                let no_lo = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x24));
                if no_lo {
                    break 'y_lo;
                }
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'y_hi: {
                c.cycles += 5;
                c.lwc1(1, p(hi, 4));
                c.c_lt_s(1, 2);
                let no_hi = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x24));
                if no_hi {
                    break 'y_hi;
                }
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'z_lo: {
                c.cycles += 6;
                c.lwc1(1, p(lo, 8));
                c.lwc1(2, p(c.g[SP][0] as u32, 0x18));
                c.c_lt_s(2, 1);
                let no_lo = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x28));
                if no_lo {
                    break 'z_lo;
                }
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0;
                    break 'ret;
                }
            }
            'z_hi: {
                c.cycles += 5;
                c.lwc1(1, p(hi, 8));
                c.c_lt_s(1, 2);
                let no_hi = !c.fcc();
                c.lwc1(0, p(c.g[SP][0] as u32, 0x28));
                if no_hi {
                    break 'z_hi;
                }
                c.cycles += 4;
                c.c_lt_s(1, 0);
                let fail = c.fcc();
                c.g[V0][0] = 0;
                if fail {
                    break 'ret;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }

        c.cycles += 8;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn wf(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    /// The engine heap, so `cMemMan_alloc` / `MEM_AllocInternal` have an arena.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x20_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    /// The size-class pool allocator's heads and free lists.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, size)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let start = DATA + 0x40_0000 + 0x4_0000 * i;
            for n in 0..8u32 {
                let node = start + 0x40 * (n + 1);
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, start + size, start + 0x40);
            w(c, GAME_GP + slot, start);
        }
    }

    /// The BELibrary dummy and the world -> game -> race manager chain.
    fn globals(c: &mut Cpu) {
        let be = DATA + 0x30_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, 0x0053_08a8, DATA + 0x30_1000);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        w(c, race + 0x28, world + 0x3000);
    }

    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        c.g[GP][0] = GAME_GP as u64;
        heap_init(c);
        let r = machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        r
    }

    // -- 0x002cb180 MenuSysPage_BeginDraw --------------------------------

    #[test]
    fn menu_sys_page_begin_draw_matches() {
        covered(ssx3_ee::s09::f_002cb180__generated, menu_sys_page_begin_draw, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x2cb180, 4);
            let ms = DATA + 0x1000;
            let page = DATA + 0x2000;
            let renderer = DATA + 0x3000;
            w(c, GAME_GP.wrapping_sub(0x854), ms);
            w(c, ms + 0xe84, page);
            w(c, ms + 0x10d8, renderer);
            for (i, off) in [0xa8u32, 0xd0, 0xf8, 0x110].into_iter().enumerate() {
                c.bus.write16(renderer + off, (r.next_u32() & 0x7ff0) as u16);
                w(c, renderer + off + 4, FAKE + 0x10 * i as u32);
            }
            for i in 0..0x40u32 {
                c.bus.write8(page + i, (seed as u8).wrapping_add(i as u8) | 1);
            }
            w(c, page + 0x4, r.next_u32());
            w(c, page + 8, r.next_u32());
        });
    }

    // -- 0x002b63d0 WorldTriggerManager_GetTriggerInfo ------------------

    #[test]
    fn world_trigger_manager_get_trigger_info_matches() {
        check(
            ssx3_ee::s08::f_002b63d0__generated,
            world_trigger_manager_get_trigger_info,
            120,
            &|c, seed| {
                let mut r = Rng(0x2b63d0 ^ (seed as u64) << 9);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let self_ = DATA + 0x1000;
                let entries = DATA + 0x2000;
                let count = 4u32;
                w(c, self_ + 0x340, count);
                w(c, self_ + 0x33c, entries);
                c.g[A0][0] = sx(self_);
                let idx = (seed % 6) as u64;
                let ty = [1u32, 2, 3, 0, 3, 2][(seed % 6) as usize];
                c.g[A1][0] = sx(idx as u32);
                let typ = DATA + 0x3000;
                let av = DATA + 0x3100;
                let bv = DATA + 0x3200;
                c.g[A2][0] = sx(typ);
                c.g[A3][0] = sx(av);
                c.g[T0][0] = sx(bv);
                c.g[T1][0] = sx(r.next_u32());
                c.g[T2][0] = 1; // skip the special-trigger delegate
                for i in 0..8u32 {
                    w(c, entries + 8 * i, ty);
                    let obj = DATA + 0x4000 + 0x100 * i;
                    w(c, entries + 8 * i + 4, obj);
                    w(c, obj + 0x10, r.next_u32());
                    w(c, obj + 0x14, r.next_u32());
                    w(c, obj + 0x18, r.next_u32());
                    w(c, obj + 0x1c, r.next_u32());
                    w(c, obj + 0x20, r.next_u32());
                    w(c, obj + 0x24, r.next_u32() & 0x7fff);
                    w(c, obj + 0x28, r.next_u32() & 0x7fff);
                    w(c, obj + 0x2c, r.next_u32() & 0x7fff);
                    w(c, obj + 0x34, r.next_u32() & 0x7fff);
                }
                // The audio singleton chain the random threshold needs.
                w(c, GAME_GP + 0x410, DATA + 0x5000);
                w(c, DATA + 0x5000 + 0x118, DATA + 0x5100);
                w(c, DATA + 0x5100, DATA + 0x5200);
                w(c, DATA + 0x5200 + 0x1d8, 0x1234);
            },
        );
    }

    // -- 0x0031bcb0 Quat_Slerp ------------------------------------------

        // -- 0x0034d778 Fence_Render ---------------------------------------

    #[test]
    fn fence_render_matches() {
        covered(ssx3_ee::s11::f_0034d778__generated, fence_render, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x34d778, 4);
            let world = DATA + 0x8_0000;
            let fence = DATA + 0x1000;
            let board = DATA + 0x2000;
            let manager = DATA + 0x3000;
            let obj = DATA + 0x4000;
            let anim = DATA + 0x5000;
            let draw = DATA + 0x6000;
            w(c, GAME_GP.wrapping_add(0x16c8), manager);
            w(c, GAME_GP.wrapping_add(0x2a90), world);
            w(c, fence + 0x18, board);
            w(c, fence + 0x40, anim);
            c.g[A0][0] = sx(fence);
            c.bus.write8(board + 0x7d, if seed % 3 == 0 { 0xff } else { 1 });
            c.bus.write16(board + 0x7e, (r.next_u32() & 3) as u16);
            let table = DATA + 0x7000;
            w(c, manager, table);
            for i in 0..4u32 {
                w(c, table + 8 * i + 0x24, 6);
                w(c, table + 0x18 * i + 0x3f0, 3);
            }
            w(c, anim + 4, obj);
            w(c, anim + 0x38, 0);
            w(c, anim + 0x3c, FAKE);
            w(c, anim + 0x94, 0);
            w(c, world + 0xe84, draw);
            w(c, world + 0x10d8, obj);
            w(c, obj + 0x2e8, 0);
            w(c, obj + 0x158, 0);
            w(c, obj + 0x15c, FAKE + 0x20);
            w(c, obj + 0x2ec, FAKE + 0x30);
            w(c, obj + 0x300, 0);
            w(c, obj + 0x304, FAKE + 0x40);
            for off in [0x60u32, 0x64, 0x68, 0x6c, 0x70, 0x74] {
                wf(c, board + off, r.f32(10.0));
            }
            for i in 0..0x30u32 {
                c.bus.write8(draw + i, i as u8 | 1);
            }
            w(c, draw + 0x18, r.next_u32());
        });
    }

    // -- 0x001b7650 cFEStateNetAccountMem_ShowPopup ---------------------

        // -- 0x002ebe20 TransitionEffect_ctorRandom2 ------------------------

    #[test]
    fn transition_effect_ctor_random2_matches() {
        check(
            ssx3_ee::s09::f_002ebe20__generated,
            transition_effect_ctor_random2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2ebe20 ^ (seed as u64) << 3);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let this = DATA + 0x1000;
                let v = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(v);
                for i in 0..4u32 {
                    wf(c, v + 4 * i, r.f32(1.0));
                }
                for i in 0..0x20u32 {
                    w(c, this + 4 * i, r.next_u32());
                }
            },
        );
    }

    // -- 0x003a13e0 cUITextScroll_HandleInput ---------------------------

    #[test]
    fn cuit_ext_scroll_handle_input_matches() {
        check(
            ssx3_ee::s13::f_003a13e0__generated,
            cuit_ext_scroll_handle_input,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a13e0, 3);
                let this = DATA + 0x1000;
                let input = DATA + 0x2000;
                let fman = DATA + 0x3000;
                let ftab = DATA + 0x3100;
                let fobj = DATA + 0x3200;
                let font = DATA + 0x3300;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(input);
                // font lookup
                w(c, this + 0x5c, fman);
                w(c, fman + 0xd0, ftab);
                w(c, ftab + 0x10, fobj);
                w(c, fobj + 8, font);
                w(c, this + 0x74, (seed % 2) as u32);
                w(c, font + 0xc, DATA + 0x3400);
                w(c, font + 0x18, DATA + 0x3400);
                // the box's own fields
                wf(c, this + 0x54, 2.0f32.to_bits());
                wf(c, this + 0x64, 100.0f32.to_bits());
                wf(c, font + 0x14, 16.0f32.to_bits());
                wf(c, font + 0x34, 1.0f32.to_bits());
                c.bus.write16(this + 0xc4, (seed % 7) as u16);
                c.bus.write16(this + 0xc6, (seed % 5) as u16);
                c.bus.write16(this + 0xc8, 9);
                c.bus.write8(this + 0x88, 1);
                c.bus.write8(this + 0x89, 2);
                // input object: vtable-ish words
                w(c, input + 8, DATA + 0x3500);
                w(c, DATA + 0x3500 + 0x10, 0x10);
                w(c, DATA + 0x3500 + 0x14, FAKE);
                w(c, DATA + 0x3500 + 0x20, 0);
                // one of the two key checks reports "pressed"
                w(c, LOG_RESULTS + 4 * ((seed % 2) as u32), 1);
                let _ = r;
            },
        );
    }

    // -- 0x002d0d48 MenuSysPoint2DItem_Measure --------------------------

    #[test]
    fn menu_sys_point2d_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002d0d48__generated,
            menu_sys_point2d_item_measure,
            100,
            &|c, seed| {
                let mut r = Rng(0x2d0d48 ^ (seed as u64) << 4);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let item = DATA + 0x1000;
                let page = DATA + 0x2000;
                let menu = DATA + 0x3000;
                let font = DATA + 0x4000;
                let style = DATA + 0x5000;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(page);
                w(c, item, menu);
                w(c, page, menu);
                w(c, item + 0x14, DATA + 0x6000);
                w(c, item + 0x40, if seed % 2 == 0 { 1 } else { 0 });
                w(c, menu + 0x124, font);
                w(c, font + 0x58, style);
                wf(c, style + 0x14, 20.0f32.to_bits());
                wf(c, style + 0x34, 1.0f32.to_bits());
                wf(c, style + 0x38, 3.0f32.to_bits());
                wf(c, style + 0x3c, 4.0f32.to_bits());
                wf(c, item + 0x54, r.f32(100.0));
                wf(c, item + 0x64, r.f32(100.0));
                wf(c, item + 0x74, r.f32(100.0));
                for i in 0..0x20u32 {
                    w(c, item + 0x80 + 4 * i, r.next_u32());
                }
                // an empty string for the text measurement
                c.bus.write8(DATA + 0x6000, 0);
            },
        );
    }

    // -- 0x00347b80 FlexBridge_Render ----------------------------------

    #[test]
    fn flex_bridge_render_matches() {
        covered(ssx3_ee::s11::f_00347b80__generated, flex_bridge_render, 60, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x347b80, 4);
            let world = DATA + 0x8_0000;
            let bridge = DATA + 0x1000;
            let board = DATA + 0x2000;
            let manager = DATA + 0x3000;
            let obj = DATA + 0x4000;
            let draw = DATA + 0x6000;
            w(c, GAME_GP.wrapping_add(0x16c8), manager);
            w(c, GAME_GP.wrapping_add(0x2a90), world);
            w(c, bridge + 0x18, board);
            w(c, bridge + 0xc, board);
            w(c, bridge + 0x78, 0);
            c.g[A0][0] = sx(bridge);
            c.bus.write8(board + 0x7d, if seed % 3 == 0 { 0xff } else { 1 });
            c.bus.write16(board + 0x7e, (r.next_u32() & 3) as u16);
            let table = DATA + 0x7000;
            w(c, manager, table);
            for i in 0..4u32 {
                w(c, table + 8 * i + 0x24, 6);
                w(c, table + 0x18 * i + 0x3f0, 3);
            }
            w(c, world + 0xe84, draw);
            w(c, world + 0x10d8, obj);
            w(c, obj + 0x2e8, 0);
            w(c, obj + 0x158, 0);
            w(c, obj + 0x15c, FAKE + 0x20);
            w(c, obj + 0x2ec, FAKE + 0x30);
            w(c, obj + 0x300, 0);
            w(c, obj + 0x304, FAKE + 0x40);
            w(c, board + 0xc0, 0);
            w(c, board + 0xc4, FAKE + 0x50);
            for i in 0..0x30u32 {
                c.bus.write8(draw + i, i as u8 | 1);
            }
            w(c, draw + 0x18, r.next_u32());
        });
    }

    // -- 0x0018eb40 FEScreenSelectProfile_OnWidgetAction ----------------

        // -- 0x001c70f0 cFEPopupConfirm_ResizeTextWidgets -------------------

    #[test]
    fn cfe_popup_confirm_resize_text_widgets_matches() {
        check(
            ssx3_ee::s03::f_001c70f0__generated,
            cfe_popup_confirm_resize_text_widgets,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1c70f0, 8);
                let this = DATA + 0x1000;
                let size = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(size);
                wf(c, size, r.f32(50.0));
                wf(c, size + 4, if seed % 3 == 0 { 0 } else { 12.0f32.to_bits() });
                for (i, slot) in [0x74u32, 0x78, 0x7c].into_iter().enumerate() {
                    let wid = if (seed >> i) & 1 == 0 { 0 } else { DATA + 0x3000 + 0x100 * i as u32 };
                    w(c, this + slot, wid);
                    if wid != 0 {
                        let vt = DATA + 0x5000 + 0x100 * i as u32;
                        w(c, wid + 8, vt);
                        c.bus.write16(vt + 0xa0, 0);
                        w(c, vt + 0xa4, FAKE + 0x10 * i as u32);
                        c.bus.write16(vt + 0xa8, 0);
                        w(c, vt + 0xac, FAKE + 0x40 + 0x10 * i as u32);
                    }
                }
            },
        );
    }

    // -- LobbyMan reply handlers ----------------------------------------

    fn lobby_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base(c, seed, salt, 3);
        let self_ = DATA + 0x1000;
        let reply = DATA + 0x2000;
        let head = DATA + 0x2100;
        c.g[A0][0] = sx(self_);
        c.g[A1][0] = sx(reply);
        w(c, self_ + 0xf0, head);
        w(c, head, head);
        w(c, head + 4, head);
        w(c, reply + 8, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        r
    }
    #[test]
    fn lobby_man_on_edit_account_reply_matches() {
        check(ssx3_ee::s06::f_0025c0c0__generated, lobby_man_on_edit_account_reply, 60, &|c, seed| {
            lobby_fixture(c, seed, 0x25c0c0);
        });
    }

    #[test]
    fn lobby_man_on_lost_by_email_reply_matches() {
        check(ssx3_ee::s06::f_0025c2d8__generated, lobby_man_on_lost_by_email_reply, 60, &|c, seed| {
            lobby_fixture(c, seed, 0x25c2d8);
        });
    }

    #[test]
    fn lobby_man_on_lost_by_name_reply_matches() {
        check(ssx3_ee::s06::f_0025c4f0__generated, lobby_man_on_lost_by_name_reply, 60, &|c, seed| {
            lobby_fixture(c, seed, 0x25c4f0);
        });
    }

    // -- 0x002e4d88 SnowBox_ctor ----------------------------------------

    #[test]
    fn snow_box_ctor_matches() {
        check(ssx3_ee::s09::f_002e4d88__generated, snow_box_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2e4d88 ^ (seed as u64) << 6);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[SP][0] = crate::difftest::SP as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            let this = DATA + 0x1000;
            let a = DATA + 0x2000;
            let b = DATA + 0x2100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(a);
            c.g[A2][0] = sx(b);
            for i in 0..4u32 {
                wf(c, a + 4 * i, r.f32(2.0));
                wf(c, b + 4 * i, r.f32(2.0));
            }
            for i in 0..0x40u32 {
                w(c, this + 4 * i, r.next_u32());
            }
            // the base matrix the constructor scales
            for i in 0..0x40u32 {
                wf(c, 0x004f_f1a0 + 4 * i, r.f32(1.0));
            }
        });
    }

    // -- 0x00330540 CollSphereTree_OverlapsAABB -------------------------

    #[test]
    fn coll_sphere_tree_overlaps_aabb_matches() {
        check(
            ssx3_ee::s11::f_00330540__generated,
            coll_sphere_tree_overlaps_aabb,
            150,
            &|c, seed| {
                let mut r = Rng(0x330540 ^ (seed as u64) << 8);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let this = DATA + 0x1000;
                let lo = DATA + 0x2000;
                let hi = DATA + 0x2100;
                let inner = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(lo);
                c.g[A2][0] = sx(hi);
                w(c, this + 0x60, inner);
                for i in 0..4u32 {
                    wf(c, inner + 0x80 + 4 * i, r.f32(10.0));
                }
                let mut g = |c: &mut Cpu, at: u32| {
                    let v = [r.f32(4.0), r.f32(4.0), r.f32(4.0)];
                    for i in 0..3 {
                        wf(c, at + 4 * i as u32, v[i]);
                    }
                };
                g(c, lo);
                g(c, hi);
            },
        );
    }
}
