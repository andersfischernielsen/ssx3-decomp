//! Staging module oc856 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc856`.
//!
//! Skipped (resumable `match pc` dispatchers, left to `tools/port_dispatcher.py`):
//! `cFEStateCharEquip_OnWidgetCreated` (0x00199458) and
//! `cFEStateCharEquip_OnWidgetNotify` (0x00199f20, also a jump table).

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
    (0x0019_9240, cfe_char_equip_init, "cFEStateCharEquip_Init"),
    (
        0x0019_9938,
        cfe_char_equip_update,
        "cFEStateCharEquip_Update",
    ),
    (
        0x0019_9c28,
        cfe_char_equip_on_widget_action,
        "cFEStateCharEquip_OnWidgetAction",
    ),
    (
        0x0019_a1e0,
        cfe_char_equip_poll_preview_loaded,
        "cFEStateCharEquip_PollPreviewLoaded",
    ),
];

// Field offsets of the char-equip state (the `this` of these functions).
pub(crate) const CFE_ENGINE: u32 = 0x10;
pub(crate) const CFE_SCREENS: u32 = 0x24;
pub(crate) const CFE_USER_NUM: u32 = 0x44;
pub(crate) const CFE_LIST: u32 = 0x5c;
pub(crate) const CFE_ITEM_COUNT: u32 = 0x958;
pub(crate) const CFE_PREVIEW_DELAY: u32 = 0x95c;
pub(crate) const CFE_ICONS_LOADED: u32 = 0xa60;
pub(crate) const CFE_PREVIEW_FILE: u32 = 0xa6c;
pub(crate) const CFE_ZOOM: u32 = 0xaa4;
pub(crate) const CFE_PAN: u32 = 0xaa8;
pub(crate) const CFE_ROTATION: u32 = 0xaac;

/// `gp - 0x848`: the world (`SSXApp`-like root); `+0x7c` is the front end.
const GP_WORLD: u32 = 0x848;

/// `world->fe->+0xb0 + 0x1a70` is the bolt file manager; `+0xb0` the rider manager.
const FE_RIDER_MANAGER: u32 = 0xb0;
const FE_BOLT_FILES: u32 = 0x1a70;

/// `cFEStateCharEquip::onCreateScreen(self)` (0x00199240): creates the
/// "equip" screen and starts its frame 0, puts the human rider on draw layer
/// 17 (no board animation), restarts the rider animation 434 when the model
/// is loaded, resets the zoom/rotation/pan and the camera, and tells the
/// audio system the front-end music screen is 2.
pub fn cfe_char_equip_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7e8));
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(CFE_ENGINE)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_9264)?; // StrHash("equip screen")
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_9278)?; // UIEngine_CreateScreen
        let no_screen = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !no_screen {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_928c)?; // UIScreen_StartThread
        }

        // The human rider: draw layer 17, no board animation.
        c.cycles += 13;
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(GP_WORLD)).lw(c);
        c.g[V0][0] = world;
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(CFE_USER_NUM)).lb(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(FE_RIDER_MANAGER));
        call_ptr(c, 0x001a_0548, 0x0019_92a0)?; // FEModule_GetRiderPreview
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = 0x11;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = 0;
        let layer = c.g[V1][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0xc2c)).sw(c, layer);
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        let not_ready = c.g[V0][0] == 0;
        Addr((c.g[A0][0] as u32).wrapping_add(0xccc)).sw(c, 0);
        if !not_ready {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xcb8)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xcb4)).lw(c);
                c.g[A1][0] = (0 < c.g[V0][0]) as u64; // unsigned compare (`sltu`), as generated
            }
        }
        c.cycles += 2;
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[S0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_1a50, 0x0019_92f0)?; // AnimController_Reset
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1b2;
                c.g[A2][0] = 0;
                call_ptr(c, 0x0031_28e8, 0x0019_9308)?; // AnimController_PlayAnim(434, 0, -1.0)
            }
        }

        // zoom = rotation = 0, pan = -1.0, camera, front-end music screen 2.
        c.cycles += 17;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(CFE_ZOOM)).sw(c, 0);
        Addr((c.g[S1][0] as u32).wrapping_add(CFE_ROTATION)).sw(c, 0);
        let pan = c.g[S1][0] as u32;
        c.swc1(0, pan.wrapping_add(CFE_PAN));
        call_ptr(c, 0x0019_bfe8, 0x0019_9324)?; // UpdatePreviewCamera
        call_ptr(c, 0x0028_b180, 0x0019_932c)?; // SSXAudioSystem_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0028_f140, 0x0019_9338)?; // SetFEMusicScreen
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The "icons loaded" poll shared by `Update` and `PollPreviewLoaded`: with
/// `a0` = the bolt file manager of the front end (`+0x1a70`), asks whether the
/// bolt file of the rider (`self+0xbc`, `self+0xa64`) is loaded; if so, sets
/// the loaded flag and refreshes the item info. Leaves `v0` as the generated
/// code does (1 when not loaded, else what the refresh returned).
fn cfe_poll_icons(c: &mut Cpu, this: u32, ret_is: u32, ret_refresh: u32) -> Flow {
    c.g[A1][0] = Addr(this.wrapping_add(0xbc)).lw(c);
    let fe = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
    c.g[A0][0] = fe;
    c.g[A2][0] = Addr(this.wrapping_add(0xa64)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(FE_BOLT_FILES));
    call_ptr(c, 0x0019_e238, ret_is)?; // BoltFile_IsLoaded
    let not_loaded = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !not_loaded {
        c.cycles += 3;
        c.g[A0][0] = this as u64;
        let one = c.g[V0][0];
        Addr(this.wrapping_add(CFE_ICONS_LOADED)).sw(c, one);
        call_ptr(c, 0x0019_a9b8, ret_refresh)?; // cFEStateCharEquip_RefreshItemInfo
    }
    Ok(())
}

/// `cFEStateCharEquip::pollPreviewLoaded(self)` (0x0019a1e0): if the icons
/// are not marked loaded yet and the bolt file is, mark them and refresh the
/// item info.
pub fn cfe_char_equip_poll_preview_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(CFE_ICONS_LOADED)).lw(c);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 8;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(GP_WORLD)).lw(c);
            let this = c.g[S0][0] as u32;
            cfe_poll_icons(c, this, 0x0019_a214, 0x0019_a228)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharEquip::update(self)` (0x00199938): runs the base state's
/// update, polls the icon file, then finds the "loading text" object of the
/// first screen and shows it or hides it through its `setVisible` method
/// (the rule depends on the equip mode, the preview file and the delay),
/// updates the preview and the camera.
pub fn cfe_char_equip_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x0019_9950)?; // UIState_Update
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(CFE_ICONS_LOADED)).lw(c);
        let loaded = c.g[V0][0] != 0;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(GP_WORLD)).lw(c);
        if !loaded {
            c.cycles += 7;
            let this = c.g[S1][0] as u32;
            cfe_poll_icons(c, this, 0x0019_9970, 0x0019_9984)?;
        }

        c.cycles += 12;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(CFE_SCREENS));
        call_ptr(c, 0x0039_7840, 0x0019_998c)?; // UIList_First
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8a0));
        call_ptr(c, 0x0031_7670, 0x0019_999c)?; // StrHash("loading_text")
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_99a8)?; // UIScreen_FindObject
        c.g[S0][0] = c.g[V0][0];
        let missing = c.g[S0][0] == 0;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(GP_WORLD)).lw(c);
        if !missing {
            c.cycles += 9;
            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(CFE_USER_NUM)).lb(c);
            let fe = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
            c.g[A0][0] = fe;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(FE_RIDER_MANAGER));
            call_ptr(c, 0x001a_0548, 0x0019_99c4)?; // FEModule_GetRiderPreview
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc8)).lw(c);
            c.g[V0][0] ^= 1;
            let not_mode1 = c.g[V0][0] != 0;
            c.g[A1][0] = 0;
            // `visible` -> a1, `v0` ends as the object's vtable.
            if not_mode1 {
                c.cycles += 2;
                c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xcc8)).lw(c);
                c.g[A1][0] ^= 1;
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(CFE_PREVIEW_FILE)).lw(c);
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[A1][0] = 1;
                    c.cycles += 1;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(CFE_PREVIEW_DELAY)).lw(c);
                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = 1;
                        c.cycles += 1;
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
                    }
                }
            }
            // obj->vtable->setVisible(obj, visible)
            c.cycles += 4;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_9a14)?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_a638, 0x0019_9a1c)?; // UpdatePreview
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_bfe8, 0x0019_9a24)?; // UpdatePreviewCamera
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const CFE_MODE: u32 = 0xc8;
pub(crate) const CFE_MONEY: u32 = 0xc0;
pub(crate) const CFE_SPENT: u32 = 0xc4;
pub(crate) const CFE_TEAM: u32 = 0xbc;
pub(crate) const CFE_DEPTH: u32 = 0x120;
pub(crate) const CFE_ITEMS: u32 = 0x124;
pub(crate) const CFE_PRICES: u32 = 0xa78;
pub(crate) const CFE_STACK: u32 = 0xd0;
pub(crate) const CFE_STATE: u32 = 0xae0;
/// Item flags: 0x20 opens a sub-category, 4 means usable (buy or equip).
pub(crate) const CFE_ITEM_FLAGS: u32 = 0x34;

/// What `OnWidgetAction` does after its event switch: the generated body
/// charges one more cycle on the `Early` exits (jumps to the common tail)
/// than on the `Direct` ones.
#[derive(Clone, Copy, PartialEq)]
enum CfeActionExit {
    /// `break 'b_199f04`: +1 cycle, then the restore of `s0`.
    Early,
    /// `lq s0` and out.
    Direct,
}

/// `cFEStateCharEquip::onWidgetAction(self, widget, event)` (0x00199c28):
/// event 5 (select) opens a sub-category (push on the category stack, new
/// title and item list), buys (mode 1) or equips the selected item; event 6
/// (back) pops the category stack, or at the top leaves the screen; event 1
/// requests the large preview and refreshes the item info.
pub fn cfe_char_equip_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        let no_widget = c.g[S2][0] == 0;
        frame.save_ra(c, 0);
        let exit = if no_widget {
            CfeActionExit::Early
        } else {
            cfe_action_event(c)?
        };
        if exit == CfeActionExit::Early {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The event switch of `OnWidgetAction` (`a2` = event; `s0` = self, `s2` =
/// widget).
fn cfe_action_event(c: &mut Cpu) -> Result<CfeActionExit, crate::call::Unwound> {
    use CfeActionExit::*;
    let this = c.g[S0][0] as u32;
    let widget = c.g[S2][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_select = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if is_select {
        return cfe_action_select(c, this, widget);
    }
    c.cycles += 2;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if high {
        c.cycles += 3;
        c.g[V0][0] = 6;
        if c.g[A2][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(this.wrapping_add(CFE_DEPTH)).lw(c);
            return cfe_action_back(c, this);
        }
        c.cycles += 2;
        return Ok(Direct);
    }
    c.cycles += 2;
    if c.g[A2][0] != c.g[V0][0] {
        c.cycles += 2;
        return Ok(Direct);
    }
    // Event 1: request the large preview unless in mode 1, refresh item info.
    c.g[V0][0] = Addr(this.wrapping_add(CFE_MODE)).lw(c);
    c.cycles += 3;
    c.g[V0][0] ^= 1;
    let not_mode1 = c.g[V0][0] != 0;
    c.g[A0][0] = this as u64;
    if !not_mode1 {
        c.cycles += 2;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0019_a4e8, 0x0019_9efc)?; // RequestPreview
    }
    c.cycles += 2;
    c.g[A0][0] = this as u64;
    call_ptr(c, 0x0019_a9b8, 0x0019_9f04)?; // RefreshItemInfo
    Ok(Early)
}

/// Event 5: the selected item of the list.
fn cfe_action_select(
    c: &mut Cpu,
    this: u32,
    widget: u32,
) -> Result<CfeActionExit, crate::call::Unwound> {
    use CfeActionExit::*;
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(CFE_LIST)).lw(c);
    if c.g[V0][0] == 0 {
        return Ok(Direct);
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x98)).lbu(c);
    c.g[V1][0] = Addr(widget.wrapping_add(0x18)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(CFE_ITEM_COUNT)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
    if out_of_range {
        return Ok(Early);
    }
    c.cycles += 6;
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(CFE_ITEMS)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(CFE_ITEM_FLAGS)).lw(c);
    c.g[V0][0] = c.g[V1][0] & 0x20;
    let not_category = c.g[V0][0] == 0;
    c.g[S1][0] = Addr((c.g[A1][0] as u32).wrapping_add(4)).lh(c);
    if !not_category {
        // Open the sub-category: push its id on the category stack.
        c.cycles += 20;
        c.g[V0][0] = Addr(this.wrapping_add(CFE_DEPTH)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let word = c.g[V0][0];
        Addr(this.wrapping_add(CFE_DEPTH)).sw(c, word);
        call_ptr(c, 0x0019_bbe0, 0x0019_9cdc)?; // SetCategoryTitle
        c.g[V0][0] = Addr(this.wrapping_add(CFE_DEPTH)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        let word = c.g[S1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(CFE_STACK)).sw(c, word);
        call_ptr(c, 0x0019_b180, 0x0019_9cf8)?; // BuildItemList
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 1;
        call_ptr(c, 0x0019_b618, 0x0019_9d08)?; // RefreshItemList
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f10)).lw(c);
        if c.g[S1][0] != c.g[V0][0] {
            return Ok(Direct);
        }
        c.cycles += 3;
        c.g[V0][0] = 3;
        let word = c.g[V0][0];
        Addr(this.wrapping_add(CFE_STATE)).sw(c, word);
        return Ok(Early);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[V1][0] & 0x4;
    if c.g[V0][0] == 0 {
        return Ok(Direct);
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(this.wrapping_add(CFE_MODE)).lw(c);
    c.g[V0][0] ^= 1;
    if c.g[V0][0] == 0 {
        // Mode 1 (buy): cost x10 (when positive) must fit the money.
        c.cycles += 3;
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xe)).lh(c);
        let free = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = 0xa;
        if !free {
            c.cycles += 1;
            c.mult(0, V1, V1, V0);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(this.wrapping_add(CFE_MONEY)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            return Ok(Direct);
        }
        c.cycles += 4;
        c.g[A0][0] = this as u64;
        call_ptr(c, 0x0019_c7e8, 0x0019_9d64)?; // OpenBuyPopup
        return Ok(Direct);
    }
    // Equip: the price of the slot must be within the budget.
    c.cycles += 16;
    c.g[S3][0] = sx(this.wrapping_add(CFE_PRICES));
    call_ptr(c, 0x0014_dc80, 0x0019_9d74)?; // cBE_getBE
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 9;
    call_ptr(c, 0x0014_dd58, 0x0019_9d80)?; // BELibrary_GetInterface
    c.g[V1][0] = Addr(widget.wrapping_add(0x18)).lw(c);
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = Addr(this.wrapping_add(CFE_SPENT)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1818)).lw(c);
    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[A0][0] != 0 {
        return Ok(Direct);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0019_bee8, 0x0019_9db4)?; // EquipSelectedItem
    if c.g[V0][0] == 0 {
        return Ok(Direct);
    }
    c.cycles += 21;
    c.g[A1][0] = Addr(this.wrapping_add(CFE_USER_NUM)).lb(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = Addr(this.wrapping_add(CFE_TEAM)).lw(c);
    call_ptr(c, 0x0014_b478, 0x0019_9dcc)?; // SumEquippedBoltValue
    let word = c.g[V0][0];
    Addr(this.wrapping_add(CFE_SPENT)).sw(c, word);
    c.g[A1][0] = 0;
    c.g[A2][0] = 1;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0019_b618, 0x0019_9de0)?; // RefreshItemList
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0019_bc90, 0x0019_9de8)?; // UpdateStatBar
    c.g[V0][0] = Addr(widget.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
    call_ptr(c, 0x0019_bd48, 0x0019_9e00)?; // UpdateStatPreviewBar
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0019_aca0, 0x0019_9e08)?; // RefreshRiderBolts
    Ok(Direct)
}

/// Event 6 (back): `v0` = the category depth.
fn cfe_action_back(c: &mut Cpu, this: u32) -> Result<CfeActionExit, crate::call::Unwound> {
    use CfeActionExit::*;
    c.cycles += 2;
    let at_top = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if !at_top {
        // Pop one category level.
        c.cycles += 6;
        c.g[S1][0] = sx(this.wrapping_add(CFE_STACK));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f10)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V1][0] != c.g[A0][0] {
            c.g[V0][0] = Addr(this.wrapping_add(CFE_DEPTH)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            let word = c.g[V0][0];
            Addr(this.wrapping_add(CFE_STATE)).sw(c, word);
            c.g[V0][0] = Addr(this.wrapping_add(CFE_DEPTH)).lw(c);
        }
        c.cycles += 19;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x0019_bbe0, 0x0019_9e54)?; // SetCategoryTitle
        c.g[V0][0] = Addr(this.wrapping_add(CFE_DEPTH)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x0019_b180, 0x0019_9e70)?; // BuildItemList
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 1;
        call_ptr(c, 0x0019_b618, 0x0019_9e80)?; // RefreshItemList
        return Ok(Direct);
    }
    // At the top: deselect the rider and push the next state.
    c.cycles += 19;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(GP_WORLD)).lw(c);
    c.g[A1][0] = Addr(this.wrapping_add(CFE_USER_NUM)).lb(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(FE_RIDER_MANAGER));
    call_ptr(c, 0x001a_0548, 0x0019_9e9c)?; // FEModule_GetRiderPreview
    c.g[V1][0] = 1;
    Addr((c.g[V0][0] as u32).wrapping_add(0xc20)).sw(c, 0);
    let word = c.g[V1][0];
    Addr((c.g[V0][0] as u32).wrapping_add(0xc1c)).sw(c, word);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = 0;
    c.g[V0][0] = Addr(this.wrapping_add(CFE_ENGINE)).lw(c);
    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(4)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0019_9ecc)?;
    c.g[A1][0] = c.g[V0][0];
    if c.g[V0][0] == 0 {
        return Ok(Early);
    }
    c.cycles += 5;
    c.g[A0][0] = Addr(this.wrapping_add(CFE_ENGINE)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x0019_9ee0)?; // UIStateMgr_SwitchState
    Ok(Direct)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, fill, Rng, DATA, GAME_GP};
    use crate::game::main::loadscreens::tests::{
        oc736_fake_call, oc736_force, oc736_force_zero, oc736_install_fakes, FX_CTRL, FX_MAGIC,
        FX_OBJ, FX_WORLD,
    };
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, in DATA+0x2a0000 and up: clear of the fake log
    // (DATA+0x100000), of loadscreens' fixtures (0x20..0x29) and of the
    // fakes' result mailboxes (DATA+0x300000 and up).
    const SELF: u32 = DATA + 0x2a_0000;
    const RIDER: u32 = DATA + 0x2b_0000;
    const WIDGET_OBJ: u32 = DATA + 0x2c_0000;
    const WIDGET_VT: u32 = DATA + 0x2c_1000;
    const WIDGET: u32 = DATA + 0x2c_2000;
    const ENGINE: u32 = DATA + 0x2d_0000;
    const CUSTOM: u32 = DATA + 0x2d_1000;
    const CUSTOM_VT: u32 = DATA + 0x2d_2000;
    const LIST: u32 = DATA + 0x2e_0000;
    const ITEMS: u32 = DATA + 0x2f_0000;
    /// A real leaf that returns 0 (a virtual method answering "nothing").
    const ZERO_LEAF: u32 = 0x0014_06c0;

    macro_rules! oc856_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            fn oc856_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc856_fakes!(
        (
            oc856_fake_317670,
            OC856_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc856_fake_397cc0,
            OC856_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc856_fake_39c870,
            OC856_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc856_fake_397840,
            OC856_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc856_fake_39d860,
            OC856_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc856_fake_39e510,
            OC856_PREV_39E510,
            ee::s13::OVERRIDE_0039E510,
            ee::s13::f_0039e510__generated,
            0x0039_e510
        ),
        (
            oc856_fake_39f400,
            OC856_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc856_fake_19bfe8,
            OC856_PREV_19BFE8,
            ee::s03::OVERRIDE_0019BFE8,
            ee::s03::f_0019bfe8__generated,
            0x0019_bfe8
        ),
        (
            oc856_fake_19e238,
            OC856_PREV_19E238,
            ee::s03::OVERRIDE_0019E238,
            ee::s03::f_0019e238__generated,
            0x0019_e238
        ),
        (
            oc856_fake_19a9b8,
            OC856_PREV_19A9B8,
            ee::s03::OVERRIDE_0019A9B8,
            ee::s03::f_0019a9b8__generated,
            0x0019_a9b8
        ),
        (
            oc856_fake_19bbe0,
            OC856_PREV_19BBE0,
            ee::s03::OVERRIDE_0019BBE0,
            ee::s03::f_0019bbe0__generated,
            0x0019_bbe0
        ),
        (
            oc856_fake_19c7e8,
            OC856_PREV_19C7E8,
            ee::s03::OVERRIDE_0019C7E8,
            ee::s03::f_0019c7e8__generated,
            0x0019_c7e8
        ),
        (
            oc856_fake_19bc90,
            OC856_PREV_19BC90,
            ee::s03::OVERRIDE_0019BC90,
            ee::s03::f_0019bc90__generated,
            0x0019_bc90
        ),
        (
            oc856_fake_19aca0,
            OC856_PREV_19ACA0,
            ee::s03::OVERRIDE_0019ACA0,
            ee::s03::f_0019aca0__generated,
            0x0019_aca0
        ),
        (
            oc856_fake_19a4e8,
            OC856_PREV_19A4E8,
            ee::s03::OVERRIDE_0019A4E8,
            ee::s03::f_0019a4e8__generated,
            0x0019_a4e8
        ),
        (
            oc856_fake_28f140,
            OC856_PREV_28F140,
            ee::s07::OVERRIDE_0028F140,
            ee::s07::f_0028f140__generated,
            0x0028_f140
        ),
        (
            oc856_fake_14b478,
            OC856_PREV_14B478,
            ee::s01::OVERRIDE_0014B478,
            ee::s01::f_0014b478__generated,
            0x0014_b478
        ),
    );

    /// Scrambled machine with the fakes' gate on, the world at `gp-0x848`
    /// (front end at `world+0x7c`), and random stack junk below `sp`.
    fn oc856_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc736_install_fakes();
        oc856_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(FX_CTRL, FX_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), FX_WORLD);
        c.bus.write32(FX_WORLD + 0x7c, FX_OBJ);
        fill(c, &mut r, SELF, 0x2c0);
        r
    }

    /// `onCreateScreen`: screen created or not, rider model ready or not,
    /// animation present or not, unwinds at any call.
    #[test]
    fn cfe_char_equip_init_matches() {
        check(
            ee::s03::f_00199240__generated,
            cfe_char_equip_init,
            200,
            &|c, seed| {
                let mut r = oc856_fixture(c, seed, 0x8560, 9);
                fill(c, &mut r, RIDER, 0x340);
                let b = r.next_u32();
                c.pc = 0x0019_9240;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(RIDER, if b & 1 != 0 { 3 } else { 20 });
                c.bus.write32(RIDER + 0xcb8, (b >> 1) & 1);
                c.bus
                    .write32(RIDER + 0xcb4, [0xffff_fffe, 0, 5][((b >> 2) % 3) as usize]);
                c.bus.write32(
                    RIDER + 0xc,
                    if (b >> 4) & 1 != 0 { RIDER + 0x800 } else { 0 },
                );
                oc736_force(c, 0x001a_0548, RIDER);
                if (b >> 5) & 1 != 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                }
            },
        );
    }

    /// `pollPreviewLoaded`: icons already loaded / file loaded / not loaded.
    #[test]
    fn cfe_char_equip_poll_preview_loaded_matches() {
        check(
            ee::s03::f_0019a1e0__generated,
            cfe_char_equip_poll_preview_loaded,
            120,
            &|c, seed| {
                let mut r = oc856_fixture(c, seed, 0x8561, 3);
                let b = r.next_u32();
                c.pc = 0x0019_a1e0;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + CFE_ICONS_LOADED, b & 1);
                if (b >> 1) & 1 != 0 {
                    oc736_force_zero(c, 0x0019_e238);
                } else {
                    oc736_force(c, 0x0019_e238, 1);
                }
            },
        );
    }

    /// The loading-text object (`vtable +0x48/+0x4c` = `setVisible`) of the
    /// first screen, found or not.
    fn oc856_widget_object(c: &mut Cpu, seed: u32) {
        c.bus.write32(WIDGET_OBJ + 8, WIDGET_VT);
        c.bus.write16(WIDGET_VT + 0x48, 0x10 * (seed % 3) as u16);
        c.bus.write32(WIDGET_VT + 0x4c, fakes::FAKE + 0x40);
    }

    /// `update`: icon poll, the loading text visibility rules (mode 1 with
    /// and without preview file/delay, other modes), the preview update
    /// (`UpdatePreview` runs for real, on states that exit early).
    #[test]
    fn cfe_char_equip_update_matches() {
        check(
            ee::s03::f_00199938__generated,
            cfe_char_equip_update,
            300,
            &|c, seed| {
                let mut r = oc856_fixture(c, seed, 0x8562, 12);
                fill(c, &mut r, RIDER, 0x340);
                oc856_widget_object(c, seed);
                let b = r.next_u32();
                c.pc = 0x0019_9938;
                c.g[A0][0] = sx(SELF);
                let mode = (b % 3) as u32;
                let delay = [5u32, 0xffff_fffe, 0][((b >> 2) % 3) as usize];
                let mut file = if (b >> 4) & 1 != 0 { 7 } else { 0xffff_ffff };
                if mode == 1 && delay != 5 {
                    // UpdatePreview must stop at once.
                    file = 0xffff_ffff;
                }
                c.bus.write32(SELF + CFE_MODE, mode);
                c.bus.write32(SELF + CFE_PREVIEW_DELAY, delay);
                c.bus.write32(SELF + CFE_PREVIEW_FILE, file);
                c.bus.write32(SELF + CFE_ICONS_LOADED, (b >> 5) & 1);
                oc736_force(c, 0x001a_0548, RIDER);
                if (b >> 6) & 1 != 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, WIDGET_OBJ);
                }
                if (b >> 7) & 1 != 0 {
                    oc736_force_zero(c, 0x0019_e238);
                } else {
                    oc736_force(c, 0x0019_e238, 1);
                }
            },
        );
    }

    /// `onWidgetAction` fixture: the paths that do not enter the not yet
    /// ported `BuildItemList`, `RefreshItemList`, `EquipSelectedItem` and
    /// `UpdateStatPreviewBar` (they have no override slot to fake): select on
    /// a buyable item (mode 1), select with the budget exceeded, select out
    /// of range or without list, back at the top of the category stack
    /// (virtual call answering a state or nothing), event 1 in both modes,
    /// other events, and no widget.
    #[test]
    fn cfe_char_equip_on_widget_action_matches() {
        check_lohi(
            ee::s03::f_00199c28__generated,
            cfe_char_equip_on_widget_action,
            400,
            &|c, seed| {
                let mut r = oc856_fixture(c, seed, 0x8563, 8);
                fill(c, &mut r, RIDER, 0x340);
                let b = r.next_u32();
                let b2 = r.next_u32();
                c.pc = 0x0019_9c28;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = if b % 11 == 0 { 0 } else { sx(WIDGET) };
                let ev = [5u32, 5, 5, 6, 6, 1, 1, 0, 2, 4, 7, 9][((b >> 4) % 12) as usize];
                c.g[A2][0] = sx(ev);
                let mode1 = (b >> 8) & 1 != 0;
                c.bus
                    .write32(SELF + CFE_MODE, if mode1 { 1 } else { (b >> 9) & 3 | 2 });
                c.bus.write32(
                    SELF + CFE_MONEY,
                    [0xffff_ffff, 0, 50, 1000, 100_000][((b >> 11) % 5) as usize],
                );
                c.bus.write32(SELF + CFE_SPENT, (b2 & 0xf) + 1);
                c.bus.write32(
                    SELF + CFE_DEPTH,
                    if (b >> 14) & 1 != 0 { 0 } else { 0xffff_ffff },
                );
                c.bus
                    .write32(SELF + CFE_LIST, if (b >> 15) & 3 == 0 { 0 } else { LIST });
                c.bus.write32(SELF + CFE_ITEM_COUNT, (b2 >> 4) & 3);
                c.bus.write32(SELF + CFE_ENGINE, ENGINE);
                // The widget: item index (+0x18) and the list's scroll top.
                let w = (b2 >> 8) % 3;
                c.bus.write32(WIDGET + 0x18, w);
                c.bus.write8(LIST + 0x98, ((b2 >> 12) & 1) as u8);
                for i in 0..4u32 {
                    let item = ITEMS + 0x100 * i;
                    c.bus.write32(SELF + CFE_ITEMS + 4 * i, item);
                    // No category (0x20) items: they enter BuildItemList.
                    let flags = [4u32, 0, 4, 8][((b2 >> (14 + 2 * i)) & 3) as usize];
                    c.bus.write32(item + CFE_ITEM_FLAGS, flags);
                    c.bus.write16(item + 4, 0x100 + i as u16);
                    c.bus.write16(
                        item + 0xe,
                        [0xfffbu16, 0, 7, 100][((b >> (20 + i)) % 4) as usize],
                    );
                    c.bus.write32(SELF + CFE_PRICES + 4 * i, (b2 >> 24) & 7);
                }
                // Budget below every price + spent: equip stops before the
                // unported `EquipSelectedItem`.
                c.bus.write32(GAME_GP.wrapping_sub(0x1818), 0x8000_0001);
                // The category id the stack compares with (never reached).
                c.bus.write32(GAME_GP.wrapping_sub(0x1f10), 0x77);
                // Event 6 at the top: engine -> custom interface -> method
                // (a state to push, or a real leaf answering 0).
                c.bus.write32(ENGINE, CUSTOM);
                c.bus.write32(CUSTOM + 4, CUSTOM_VT);
                c.bus.write16(CUSTOM_VT + 0x28, 0x10 * (seed % 3) as u16);
                c.bus.write32(
                    CUSTOM_VT + 0x2c,
                    if (b >> 24) & 1 != 0 {
                        ZERO_LEAF
                    } else {
                        fakes::FAKE + 0x50
                    },
                );
                oc736_force(c, 0x001a_0548, RIDER);
            },
        );
    }
}
