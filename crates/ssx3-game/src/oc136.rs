//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path functions — setup-manager messages, FE
//! screen/lifecycle helpers, manager bookkeeping — each with a differential
//! test against the generated body.
//!
//! The generated code is the authority for cycles, registers and memory.
//! Calls use [`crate::call`] (a [`Callee`] for a same-shard direct call,
//! [`call_ptr`] for `jalr`/cross-shard calls) so that overridden callees and
//! host-thread unwinds behave exactly as the compiled caller.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_9a18,
        csetup_man_client_recv_msg9,
        "cSetupMan_ClientRecvMsg9",
    ),
    (
        0x0024_fab8,
        detail_system_menu_dtor,
        "DetailSystemMenu_dtor",
    ),
    (
        0x001c_fff8,
        cfe_state_reward_gallery_register_textures,
        "cFEStateRewardGallery_RegisterTextures",
    ),
    (
        0x001d_0b00,
        cfe_state_reward_gallery_refresh_page,
        "cFEStateRewardGallery_RefreshPage",
    ),(
        0x0034_5570,
        spline_modifier_load_path,
        "SplineModifier_LoadPath",
    ),
    (
        0x001a_4670,
        cfe_state_net_messenger_get_buddy_status,
        "cFEStateNetMessenger_GetBuddyStatus",
    ),
    (
        0x001b_7820,
        cfe_state_net_account_mem_update_state06,
        "cFEStateNetAccountMem_UpdateState06",
    ),
    (
        0x001b_7938,
        cfe_state_net_account_mem_update_state0c,
        "cFEStateNetAccountMem_UpdateState0C",
    ),(
        0x002a_2568,
        ssx_audio_system_dj_say20cc_2,
        "SSXAudioSystem_DJSay20CC_2",
    ),
    (
        0x002a_e3a8,
        path_plat_remove_oldest_bank,
        "PATH_PlatRemoveOldestBank",
    ),(
        0x002c_c578,
        menu_sys_bool_item_handle_input,
        "MenuSysBoolItem_HandleInput",
    ),
    (
        0x0026_9538,
        csetup_man_host_recv_msg9,
        "cSetupMan_HostRecvMsg9",
    ),
    (0x0027_2df0, inseq_span_track_start, "InSeq_SpanTrackStart"),
    (
        0x0037_a540,
        ps2_graphics_man_create_grid_mesh,
        "PS2GraphicsMan_CreateGridMesh",
    ),
    (
        0x001c_bfc0,
        ckeyboard_popup_move_down_from_row4,
        "cKeyboardPopup_MoveDownFromRow4",
    ),
    (
        0x0030_b0e8,
        wscript_process_list_delete_by_section,
        "WScriptProcessList_DeleteBySection",
    ),(
        0x001c_f168,
        cfe_state_reward_gallery_on_activate,
        "cFEStateRewardGallery_OnActivate",
    ),
    (
        0x0019_a238,
        cfe_state_char_equip_show_equip_mode,
        "cFEStateCharEquip_ShowEquipMode",
    ),(
        0x0018_ddd0,
        cfe_state_options_device_select_on_widget_notify,
        "cFEStateOptionsDeviceSelect_OnWidgetNotify",
    ),
    (
        0x0019_afd0,
        cfe_state_char_equip_category_has_flagged_item,
        "cFEStateCharEquip_CategoryHasFlaggedItem",
    ),(0x0011_df18, rider_reset_motion, "Rider_ResetMotion"),(
        0x0023_2588,
        init_load_screen_update,
        "InitLoadScreen_Update",
    ),(
        0x0036_6548,
        vram_heap_free_unlocked,
        "VramHeap_FreeUnlocked",
    ),
    (
        0x0026_9948,
        csetup_man_client_recv_msg8,
        "cSetupMan_ClientRecvMsg8",
    ),(
        0x0032_45a8,
        input_parser_compile_identifier,
        "InputParser_CompileIdentifier",
    ),
    (0x0032_6360, input_preproc_write, "InputPreproc_Write"),
    (
        0x003a_9188,
        cworldcache_streamer_reset,
        "cWorldCache_StreamerReset",
    ),
    (
        0x0034_4730,
        crowd_man3d_group_set_mesh_frame,
        "CrowdMan3d_GroupSetMeshFrame",
    ),
    (0x0026_f980, replay_end_playback, "Replay_EndPlayback"),(
        0x0023_0360,
        game_module_on_section_unload,
        "GameModule_OnSectionUnload",
    ),(0x0023_3930, cfe_load_screen_unload, "cFELoadScreen_Unload"),(
        0x0039_b7b0,
        ui_menu_find_first_selectable,
        "UIMenu_FindFirstSelectable",
    ),(
        0x0020_eb50,
        cov_state_hud1p_load_meter_assets,
        "cOVStateHUD1P_LoadMeterAssets",
    ),];

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `a + off`.
#[inline(always)]
fn p(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

/// The world singleton at `gp-0x848`.
#[inline(always)]
fn world(c: &mut Cpu) -> u32 {
    Addr(gp(c, -0x848)).lw(c) as u32
}

// ---------------------------------------------------------------------------
// Setup-manager message helpers.

/// `void cSetupMan_ClientRecvMsg9(cSetupMan *self, u32 a1, u32 a2)`
/// (0x269a18): allocate a 0x14-byte message node (kind 0xFC, vtable
/// `0x481130`, the two arguments at +0xC/+0x10), then a 0xC-byte list node
/// and splice it after the head stored at `*(gp-0xC8)+0xF0`.
pub fn csetup_man_client_recv_msg9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A0][0] = 0x14;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x0048_0f18;
        frame.save(c, S2, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_9a4c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0048_1130;
        c.g[V0][0] = 0xfc;
        Addr(p(c.g[S2][0], 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x0048_04a8;
        Addr(p(c.g[S2][0], 0x4)).sw(c, c.g[V1][0]);
        Addr(p(c.g[S2][0], 0xc)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0xc;
        Addr(p(c.g[S2][0], 0x10)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0x2000_0000;
        Addr(p(c.g[S2][0], 0x8)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0xf0)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_9a94)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            Addr(p(c.g[A1][0], 0x8)).sw(c, c.g[S2][0]);
        }
        // Splice the list node after the head.
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x50);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        frame.restore(c, S2, 0x30);
        Addr(p(c.g[A1][0], 0x4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(p(c.g[V0][0], 0x4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end lifecycle.

/// `void DetailSystemMenu_dtor(DetailSystemMenu *self, int flags)`
/// (0x24fab8): destroy the twelve sub-objects at +0x334..+0x130 (each with
/// `flags = 2`) and then the menu itself.
pub fn detail_system_menu_dtor(c: &mut Cpu) {
    // (offset, return address) pairs, in call order.
    const DESTROYED: [(u32, u32); 12] = [
        (0x334, 0x0024_fadc),
        (0x300, 0x0024_fae8),
        (0x2cc, 0x0024_faf4),
        (0x298, 0x0024_fb00),
        (0x264, 0x0024_fb0c),
        (0x230, 0x0024_fb18),
        (0x1fc, 0x0024_fb24),
        (0x1c8, 0x0024_fb30),
        (0x194, 0x0024_fb3c),
        (0x164, 0x0024_fb48),
        (0x14c, 0x0024_fb54),
        (0x130, 0x0024_fb60),
    ];
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in DESTROYED {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_fb6c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_RegisterTextures(cFEStateRewardGallery *self,
/// const GalleryData *data)` (0x1cfff8): publish the data's entry count and,
/// for each entry, copy its name stub into `self->entries[i]` and resolve the
/// texture handle through `cGame`'s vtable slot 46.
pub fn cfe_state_reward_gallery_register_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        if c.g[S2][0] == 0 {
            c.cycles += 2;
            Addr(p(c.g[S4][0], 0x78)).sw(c, 0);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c.g[S2][0], 0x8)).lw(c);
            c.g[S1][0] = 0;
            Addr(p(c.g[S4][0], 0x78)).sw(c, c.g[V0][0]);
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 2;
                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7c));
                loop {
                    c.cycles += 24;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x003b_3d00, 0x001d_0054)?;
                    Addr(p(c.g[S0][0], 0x4)).sb(c, 0);
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 1;
                    c.g[T2][0] = Addr(gp(c, -0x854)).lw(c);
                    c.g[T1][0] = u64::MAX;
                    c.g[A1][0] = Addr(c.g[S3][0] as u32).lw(c);
                    c.g[V1][0] = Addr(p(c.g[T2][0], 0x10d8)).lw(c);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A0][0] = Addr(p(c.g[V1][0], 0x170)).lh(c);
                    c.g[V0][0] = Addr(p(c.g[V1][0], 0x174)).lw(c);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, 0x001d_008c)?;
                    Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = Addr(p(c.g[S4][0], 0x78)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_0040);
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_RefreshPage(cFEStateRewardGallery *self)`
/// (0x1d0b00): re-lay-out every page entry (`vtable[39]`), then refresh the
/// five text/scroll widgets (`vtable[46]`, `[45]`, `[44]`, `[41]`, `[42]`).
pub fn cfe_state_reward_gallery_refresh_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S1][0], 0x58)).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(p(c.g[S1][0], 0x8)).lw(c);
            loop {
                c.cycles += 10;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x138)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0], 0x13c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001d_0b40)?;
                c.g[V0][0] = Addr(p(c.g[S1][0], 0x58)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(c.g[S1][0], 0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001d_0b28);
                }
            }
        }
        c.cycles += 30;
        for (delta_off, fn_off, ret) in [
            (0x170u32, 0x174u32, 0x001d_0b64u32),
            (0x168, 0x16c, 0x001d_0b78),
            (0x160, 0x164, 0x001d_0b8c),
            (0x148, 0x14c, 0x001d_0ba0),
            (0x150, 0x154, 0x001d_0bb4),
        ] {
            c.g[V0][0] = Addr(p(c.g[S1][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], delta_off)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], fn_off)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Character-equip screen.

/// `void cFEStateCharEquip_ToggleBuyEquip(cFEStateCharEquip *self)` (0x19a3d0):
/// flip the `is-buy` flag, switch the detail panel's mode, retitle the
/// category, rebuild and refresh the item list.
pub fn cfe_state_char_equip_toggle_buy_equip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0], 0xc8)).lw(c);
        let buying = (c.g[V0][0] ^ 1) != 0;
        c.g[V0][0] = 1;
        if buying {
            // Switch to buy mode.
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            Addr(p(c.g[S0][0], 0xc8)).sw(c, c.g[V0][0]);
            callee::SHOW_BUY_MODE.call(c, 0x0019_a42c)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x50)).lw(c);
            let no_text = c.g[V0][0] == 0;
            c.g[A0][0] = 0x0046_0000;
            if !no_text {
                c.cycles += 5;
                c.g[A0][0] = 0x0046_0820;
                call_ptr(c, 0x0031_7670, 0x0019_a440)?;
                c.g[A0][0] = Addr(p(c.g[S0][0], 0x50)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_a44c)?;
            }
            // Buy mode always joins the shared tail through this cycle.
            c.cycles += 1;
        } else {
            // Switch to equip mode.
            c.cycles += 5;
            Addr(p(c.g[S0][0], 0xc8)).sw(c, 0);
            callee::SHOW_EQUIP_MODE.call(c, 0x0019_a3f8)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x50)).lw(c);
            let no_text = c.g[V0][0] == 0;
            c.g[A0][0] = 0x0046_0000;
            if !no_text {
                c.cycles += 7;
                c.g[A0][0] = 0x0046_0810;
                call_ptr(c, 0x0031_7670, 0x0019_a40c)?;
                c.g[A0][0] = Addr(p(c.g[S0][0], 0x50)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_a418)?;
            } else {
                c.cycles += 1;
            }
            c.g[A1][0] = Addr(gp(c, -0x1f20)).lw(c);
        }
        c.cycles += 18;
        c.g[A2][0] = 0x50;
        Addr(p(c.g[S0][0], 0x120)).sw(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        call_ptr(c, 0x003e_6448, 0x0019_a460)?;
        c.g[A1][0] = Addr(gp(c, -0x1f20)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        callee::SET_CATEGORY_TITLE.call(c, 0x0019_a46c)?;
        c.g[A1][0] = Addr(gp(c, -0x1f20)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        callee::BUILD_ITEM_LIST.call(c, 0x0019_a478)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 1;
        callee::REFRESH_ITEM_LIST.call(c, 0x0019_a488)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spline paths.

/// `void SplineModifier_LoadPath(SplineModifier *self, Stream *stream)`
/// (0x345570): let the stream load the modifier's path (`v02(self, 8)`),
/// resolve the first spline node, walk `count` nodes, and store the spline
/// length.
pub fn spline_modifier_load_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x8;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0], 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_55a0)?;
        call_ptr(c, 0x002d_1bd8, 0x0034_55a8)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        if c.g[V1][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = Addr(p(c.g[V1][0], 0x44)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(p(c.g[V0][0], 0x24)).lw(c);
        c.g[A0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
        let none = (c.g[A0][0] as i64) <= 0;
        Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x64)).lw(c);
                let more = c.g[A0][0] != 0;
                Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[V0][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_5600);
                }
            }
        }
        c.cycles += 7;
        callee::SPLINE_GET_PATH_LENGTH.call(c, 0x0034_5624)?;
        Addr(p(c.g[S0][0], 0xc)).write_fpu(c, crate::ps2f::FpuF32(c.f[0]));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network front-end state machines.

/// `int cFEStateNetMessenger_GetBuddyStatus(cFEStateNetMessenger *self,
/// int index)` (0x1a4670): 0 if the buddy is not connected, 2 if it is not
/// online at that index, else 1.
pub fn cfe_state_net_messenger_get_buddy_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let have = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A1][0];
        if !have {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_4690)?;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_1460, 0x001a_4698)?;
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_13c8, 0x001a_46a8)?;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
        }
        c.cycles += 2;
        let have2 = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        if !have2 {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001a_46bc)?;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_2400, 0x001a_46c8)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                call_ptr(c, 0x0026_0f80, 0x001a_46e4)?;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1460, 0x001a_46ec)?;
            let connected = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if !connected {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001a_46fc)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 2;
            let have2 = c.g[V0][0] != 0;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            if !have2 {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001a_4710)?;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0026_3338, 0x001a_471c)?;
            c.g[A0][0] = 0x2;
            c.g[V1][0] = 0x1;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two net-account substate updates: on substate 5,
/// drive the session through the `+0x6d4` sub-substate and set the next
/// state; otherwise do nothing. `final_state` is 9 (state 0x06) or 0x11
/// (state 0x0C).
fn net_account_mem_update(c: &mut Cpu, final_state: u64) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = Addr(gp(c, 0x2e0)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    let is4 = (c.g[V1][0] ^ 0x4) != 0;
    c.g[S0][0] = c.g[A0][0];
    if !is4 {
        c.cycles += 4;
        c.g[A1][0] = 0x7;
        callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_7848)?;
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[V1][0] ^ 0x5;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    // Substate 5: look at `self+0x6d4`.
    c.cycles += 4;
    c.g[V1][0] = Addr(p(c.g[S0][0], 0x6d4)).lw(c);
    c.g[V0][0] = 0x2;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 4;
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0026_5cb0, 0x001b_7874)?;
        let no = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no {
            c.cycles += 4;
            c.g[A1][0] = 0xd;
            callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_7884)?;
            c.cycles += 3;
            c.lq(16, frame.at(c, 0x10));
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(p(c.g[S0][0], 0x6d4)).lw(c);
        c.g[V0][0] = 0x1;
    } else {
        c.g[V0][0] = 0x1;
    }
    // `v1` is the sub-substate, `v0` = 1.
    c.cycles += 2;
    let not_one = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0x5;
    if !not_one {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x10;
        callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_78a8)?;
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 2;
    let is_five = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 0x6;
    let mut set_19 = is_five;
    if !is_five {
        c.cycles += 2;
        let not_six = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if not_six {
            // final state
            c.cycles += 3;
            c.g[A1][0] = final_state;
            callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_78dc)?;
            c.cycles += 3;
            c.lq(16, frame.at(c, 0x10));
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        set_19 = true;
    }
    if set_19 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x19;
        callee::NET_ACCOUNT_SET_STATE.call(c, 0x001b_78cc)?;
        c.cycles += 3;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    Ok(())
}

/// `void cFEStateNetAccountMem_UpdateState06(self)` (0x1b7820).
pub fn cfe_state_net_account_mem_update_state06(c: &mut Cpu) {
    guest(c, |c| net_account_mem_update(c, 9))
}

/// `void cFEStateNetAccountMem_UpdateState0C(self)` (0x1b7938).
pub fn cfe_state_net_account_mem_update_state0c(c: &mut Cpu) {
    guest(c, |c| net_account_mem_update(c, 0x11))
}

// ---------------------------------------------------------------------------
// Final-standings screen.

/// `int OVScreenFinalStandings_HandleInput(OVScreenFinalStandings *self,
/// Input *input)` (0x1e7aa8): route the input to the standings list when the
/// "final" object is selected, tell the BE interface about it, and run the
/// pending overlay handler. Always returns 0.
pub fn ov_screen_final_standings_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x0046_e510;
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_7670, 0x001e_7acc)?;
        c.g[A0][0] = Addr(p(c.g[S0][0], 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_7ad8)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(world(c));
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x95)).lbu(c);
            c.g[V0][0] = 0x2;
            let not_final = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = sx(world(c));
            if !not_final {
                c.cycles += 6;
                c.g[V1][0] = sx(world(c));
                c.g[A1][0] = Addr(p(c.g[S1][0], 0x4)).lbu(c);
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x84)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x28)).lw(c);
                call_ptr(c, 0x0026_f898, 0x001e_7b04)?;
                c.g[V0][0] = sx(world(c));
            }
        }
        c.cycles += 7;
        c.g[A1][0] = 0x7;
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x78)).lw(c);
        call_ptr(c, 0x0014_dd58, 0x001e_7b14)?;
        c.g[V1][0] = 0x0053_4b30;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let no_handler = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp(c, -0x204)).lw(c);
        // 0: exit through the outer block, 1: through the middle, 2: direct.
        let mut landing = 0u8;
        if !no_handler {
            c.cycles += 2;
            frame.restore(c, S0, 0x20);
            if c.g[V0][0] == 0 {
                landing = 1;
            } else {
                c.cycles += 3;
                let present = Addr(c.g[V0][0] as u32).lw(c) != 0;
                c.g[V0][0] = 0;
                if !present {
                    landing = 2;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(p(c.g[S1][0], 0x8)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V1][0], 0xd8)).lh(c);
                    c.g[V0][0] = Addr(p(c.g[V1][0], 0xdc)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001e_7b4c)?;
                    let zero = c.g[V0][0] == 0;
                    frame.restore(c, S0, 0x20);
                    if zero {
                        landing = 1;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x28;
                        call_ptr(c, 0x0020_ab50, 0x001e_7b5c)?;
                    }
                }
            }
        }
        if landing <= 1 {
            c.cycles += 1;
            if landing == 0 {
                frame.restore(c, S0, 0x20);
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Setup manager / audio / song setup.

/// `void cSetupMan_SetRemoteCharacter(cSetupMan *self, void *character,
/// const u8 *blob)` (0x264d80): publish the character pointer to the BE
/// interface, store it in the interface singleton, and copy the remote
/// character's four-byte id when the blob is non-null.
pub fn csetup_man_set_remote_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0014_dc80, 0x0026_4da0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0026_4dac)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_7448, 0x0026_4dc0)?;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0xc)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0026_4dd4)?;
        c.g[V0][0] = sx(world(c));
        c.g[A1][0] = 0x7;
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x78)).lw(c);
        call_ptr(c, 0x0014_dd58, 0x0026_4de4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0x0053_4b30;
        let no_blob = c.g[S2][0] == 0;
        Addr(p(c.g[V0][0], 0x32c)).sw(c, c.g[S1][0]);
        if !no_blob {
            c.cycles += 10;
            c.lwl(3, p(c.g[S2][0], 0x3));
            c.lwr(3, c.g[S2][0] as u32);
            c.g[A0][0] = Addr(p(c.g[S2][0], 0x4)).lb(c);
            c.g[A2][0] = Addr(p(c.g[S2][0], 0x5)).lb(c);
            c.g[A3][0] = Addr(p(c.g[S2][0], 0x6)).lb(c);
            c.swl(3, p(c.g[V0][0], 0x4ab));
            c.swr(3, p(c.g[V0][0], 0x4a8));
            Addr(p(c.g[V0][0], 0x4ac)).sb(c, c.g[A0][0]);
            Addr(p(c.g[V0][0], 0x4ad)).sb(c, c.g[A2][0]);
            Addr(p(c.g[V0][0], 0x4ae)).sb(c, c.g[A3][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(p(c.g[A1][0], 0xc)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0026_4e34)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DJSay20CC_2(self, u32 context)` (0x2a2568): when DJ
/// speech is on, install a 20CC speech event and, if accepted, queue the
/// actual line through the speech system.
pub fn ssx_audio_system_dj_say20cc_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_2588)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
        if off {
            // Speech off: fall through to the epilogue below.
        } else {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cc;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_25b0)?;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                c.cycles += 5;
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 25;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0029_f198, 0x002a_25c0)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0028_b1e8, 0x002a_25c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x002a_25d4)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4bc0, 0x002a_25dc)?;
            c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x002a_1bd8, 0x002a_25ec)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x1;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cc;
            call_ptr(c, 0x003d_8008, 0x002a_2600)?;
            c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = 0x3;
            let target = c.g[V1][0] as u32;
            c.g[T0][0] = 0x3;
            call_ptr(c, target, 0x002a_261c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSongConfiguration_ctor(cSongConfiguration *self)` (0x2b1a98): zero the
/// configuration, seed the limits (0x5A, 0x32, 0xFFFF, -1, 60.0), and copy
/// the ten 32-byte default name buffers from the game string table.
pub fn csong_configuration_ctor(c: &mut Cpu) {
    c.cycles += 42;
    c.g[V0][0] = 0x0048_3980;
    c.mtc1(1, 0);
    c.g[T6][0] = c.g[A0][0];
    c.g[V1][0] = 0x64;
    c.g[A2][0] = 0x8;
    c.g[T7][0] = 0x004a_0000;
    Addr(p(c.g[T6][0], 0x68)).sw(c, 0x2);
    Addr(p(c.g[T6][0], 0x70)).sw(c, 0x8);
    Addr(p(c.g[T6][0], 0xa8)).sw(c, c.g[V0][0]);
    c.g[A1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x44));
    Addr(p(c.g[T6][0], 0x6c)).sw(c, 0x4);
    Addr(p(c.g[T6][0], 0x74)).sw(c, 0x10);
    Addr(p(c.g[T6][0], 0x7c)).sw(c, 0x5a);
    Addr(p(c.g[T6][0], 0x84)).sw(c, 0x32);
    Addr(p(c.g[T6][0], 0x8c)).sw(c, c.g[V1][0]);
    Addr(p(c.g[T6][0], 0x90)).write_u32(c, 0x42f0_0000);
    Addr(p(c.g[T6][0], 0x9c)).sw(c, 0xffff);
    Addr(p(c.g[T6][0], 0xa0)).sw(c, u64::MAX);
    Addr(p(c.g[T6][0], 0xa4)).sw(c, 1);
    Addr(p(c.g[T6][0], 0x40)).sw(c, 0);
    Addr(p(c.g[T6][0], 0x64)).sw(c, 0x4);
    Addr(p(c.g[T6][0], 0x78)).sw(c, 0);
    Addr(p(c.g[T6][0], 0x80)).sw(c, 0x64);
    Addr(p(c.g[T6][0], 0x88)).sw(c, 0x64);
    Addr(p(c.g[T6][0], 0x94)).sw(c, 1);
    Addr(p(c.g[T6][0], 0x98)).sw(c, u64::MAX);
    c.g[V0][0] = Addr(p(c.g[T7][0], 0x35a8)).lbu(c);
    Addr(c.g[T6][0] as u32).sb(c, c.g[V0][0]);
    c.g[V1][0] = Addr(p(c.g[T7][0], 0x35a8)).lbu(c);
    Addr(p(c.g[T6][0], 0x20)).sb(c, c.g[V1][0]);
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c.g[T7][0], 0x35a8)).lbu(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        Addr(c.g[A1][0] as u32).sb(c, c.g[V0][0]);
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        if !more {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = c.g[T6][0];
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_1b40);
        }
    }
}

// ---------------------------------------------------------------------------
// Music banks and menu items.

/// `int PATH_PlatRemoveOldestBank(self)` (0x2ae3a8): scan the 24 bank slots
/// for the oldest (smallest positive age), remove that bank, and return 1
/// when one was found. A resumable loop: the back edge at 0x2ae3f0 is a legal
/// re-entry point.
pub fn path_plat_remove_oldest_bank(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        // The frame is opened at the entry arm and reused by the resume arms.
        let mut frame = Frame { size: 0x70 };
        loop {
            match pc {
                0x002a_e3a8 => {
                    c.cycles += 17;
                    frame = Frame::open(c, 0x70);
                    frame.save(c, S1, 0x50);
                    frame.save(c, S0, 0x60);
                    frame.save(c, S2, 0x40);
                    frame.save(c, S3, 0x30);
                    frame.save(c, S4, 0x20);
                    frame.save(c, S5, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = 0x7ff_ffff;
                    c.g[S2][0] = c.g[A0][0];
                    c.g[S3][0] = u64::MAX;
                    c.g[S0][0] = 0;
                    c.g[S5][0] = 0x004d_3ef8;
                    c.g[S4][0] = 0x28;
                    pc = 0x002a_e414;
                }
                0x002a_e3ec => {
                    c.cycles += 1;
                    pc = 0x002a_e3f0;
                }
                0x002a_e3f0 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0x24)).lw(c);
                    let negative = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if negative {
                        pc = 0x002a_e410;
                    } else {
                        pc = 0x002a_e400;
                    }
                }
                0x002a_e400 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        pc = 0x002a_e414;
                    } else {
                        pc = 0x002a_e408;
                    }
                }
                0x002a_e408 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.g[V1][0];
                    c.g[S3][0] = c.g[S0][0];
                    pc = 0x002a_e410;
                }
                0x002a_e410 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    pc = 0x002a_e414;
                }
                0x002a_e414 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 24) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if done {
                        pc = 0x002a_e430;
                    } else {
                        pc = 0x002a_e420;
                    }
                }
                0x002a_e420 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    callee::PATH_PLAT_FIND_BANK_SLOT.call(c, 0x002a_e428)?;
                    let found = (c.g[V0][0] as i64) >= 0;
                    abi::mult(c, c.g[A2][0] as u32, c.g[S4][0] as u32);
                    if found {
                        if c.cycles >= c.next_event {
                            c.pc = 0x002a_e3f0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x002a_e3f0;
                    } else {
                        pc = 0x002a_e430;
                    }
                }
                0x002a_e430 => {
                    c.cycles += 3;
                    c.g[V0][0] = u64::MAX;
                    let none = c.g[S3][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0];
                    if none {
                        pc = 0x002a_e44c;
                    } else {
                        pc = 0x002a_e43c;
                    }
                }
                0x002a_e43c => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S3][0];
                    callee::PATH_PLAT_REMOVE_BANK.call(c, 0x002a_e444)?;
                    c.lq(16, frame.at(c, 0x60));
                    pc = 0x002a_e454;
                }
                0x002a_e44c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.lq(16, frame.at(c, 0x60));
                    pc = 0x002a_e454;
                }
                0x002a_e454 => {
                    c.cycles += 8;
                    frame.restore(c, S1, 0x50);
                    frame.restore(c, S2, 0x40);
                    frame.restore(c, S3, 0x30);
                    frame.restore(c, S4, 0x20);
                    frame.restore(c, S5, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a_e3a8, 0x002a_e474);
                    return Ok(());
                }
            }
        }
    });
}
/// `int MenuSysBoolItem_HandleInput(MenuSysBoolItem *self)` (0x2cc578):
/// toggles the item's bool when the handled input is confirm (2/3) or the
/// item's own code is 6 (and the item is enabled), then notifies the owner.
pub fn menu_sys_bool_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_HANDLE_INPUT.call(c, 0x002c_c590)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED.call(c, 0x002c_c59c)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                c.lq(16, frame.at(c, 0x20));
            } else {
                let mut toggle = false;
                let mut value_ptr = 0u64;
                c.cycles += 3;
                if Addr(p(c.g[S0][0], 0x18)).lw(c) != 0 {
                    c.cycles += 3;
                    if Addr(p(c.g[S0][0], 0x1c)).lw(c) != 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5cc)?;
                        c.g[V1][0] = 0x2;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.g[V1][0] = Addr(p(c.g[S0][0], 0x20)).lw(c);
                            value_ptr = c.g[V1][0];
                            toggle = true;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5e0)?;
                            c.g[V1][0] = 0x3;
                            if c.g[V0][0] == c.g[V1][0] {
                                c.g[V1][0] = Addr(p(c.g[S0][0], 0x20)).lw(c);
                                value_ptr = c.g[V1][0];
                                toggle = true;
                            }
                        }
                    }
                }
                if !toggle {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5f4)?;
                    c.g[V1][0] = 0x6;
                    let other = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.g[S1][0];
                    if !other {
                        c.cycles += 1;
                        c.g[V1][0] = Addr(p(c.g[S0][0], 0x20)).lw(c);
                        value_ptr = c.g[V1][0];
                        toggle = true;
                    }
                }
                if toggle {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S0][0];
                    let value = Addr(value_ptr as u32).lw(c) ^ 1;
                    Addr(value_ptr as u32).sw(c, value);
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = Addr(p(c.g[A2][0], 0x12c)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0], 0x20)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_c62c)?;
                    c.g[V0][0] = c.g[S1][0];
                }
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sequencing and script processes.

/// `int InSeq_SpanTrackStart(SpanTrack *self, u16 *key, void *owner,
/// u32 *flags)` (0x272df0): start playing a span track, allocating and
/// activating its first key. Returns whether the track has a first key.
pub fn inseq_span_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[A1][0]);
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[A2][0]);
        Addr(p(c.g[S0][0], 0x10)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A3][0];
        let mut extra = true;
        if !empty {
            c.cycles += 14;
            c.g[V1][0] = Addr(p(c.g[A1][0], 0x2)).lhu(c);
            c.g[A1][0] = 0x1c;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
            abi::mult(c, c.g[V1][0] as u32, c.g[A1][0] as u32);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lw(c);
            c.g[A1][0] = c.lo[0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
            callee::ISEQ_HEAP_ALLOC.call(c, 0x0027_2e3c)?;
            c.g[V1][0] = Addr(p(c.g[S0][0], 0x10)).lw(c);
            Addr(p(c.g[S0][0], 0x20)).sw(c, c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            Addr(p(c.g[S0][0], 0x18)).sw(c, 0);
            let none = c.g[V0][0] == 0;
            Addr(p(c.g[S0][0], 0x14)).sw(c, c.g[V0][0]);
            if !none {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x8)).lhu(c);
                if c.g[V0][0] != 0 {
                    extra = false;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    loop {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] |= 0x4;
                        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A1][0] = Addr(p(c.g[S0][0], 0x14)).lw(c);
                        callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY.call(c, 0x0027_2e7c)?;
                        c.g[A0][0] = c.g[S0][0];
                        callee::ISEQ_SPAN_TRACK_NEXT_KEY.call(c, 0x0027_2e84)?;
                        c.g[V0][0] = Addr(p(c.g[S0][0], 0x14)).lw(c);
                        if c.g[V0][0] == 0 {
                            extra = false;
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(c.g[V0][0], 0x4)).lhu(c);
                        if c.g[V0][0] != 0 {
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_2e68);
                        }
                    }
                }
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(p(c.g[S0][0], 0x10)).lw(c);
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V0][0] > 0) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `cPS2GridMesh *PS2GraphicsMan_CreateGridMesh(PS2GraphicsMan *self,
/// int a1, int a2, u32 flags)` (0x37a540): allocate and construct a grid mesh;
/// when the `0x40000000` flag is set, initialise its vertex range too.
pub fn ps2_graphics_man_create_grid_mesh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x0049_28d8;
        c.g[A2][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1cc;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0037_a580)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        callee::PS2_GRID_MESH_CTOR.call(c, 0x0037_a598)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x4000_0000;
        c.g[S0][0] &= c.g[V0][0];
        let no_range = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !no_range {
            c.cycles += 2;
            let mut no_mesh = false;
            if c.g[S1][0] != 0 {
                c.g[V1][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                no_mesh = true;
            }
            if !no_mesh {
                c.cycles += 6;
                c.g[A0][0] = Addr(p(c.g[V1][0], 0x10)).lh(c);
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x14)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0037_a5cc)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.g[S1][0];
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
                    c.g[A1][0] = 0x3;
                    c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0037_a5ec)?;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 7;
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
// Keyboard popup (a resumable menu cursor mover).

/// `void cKeyboardPopup_MoveDownFromRow4(cKeyboardPopup *self, int step)`
/// (0x1cbfc0): move the cursor down, wrapping within the enabled keys of the
/// column. The 0x1cbffc back edge is a re-entry point.
pub fn ckeyboard_popup_move_down_from_row4(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        // Opened at the entry arm, reused by the 0x1cbffc resume arm.
        let mut frame = Frame { size: 0x40 };
        loop {
            match pc {
                0x001c_bfc0 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.g[V1][0] = Addr(p(c.g[S0][0], 0x140)).lw(c);
                    let not_row = c.g[V1][0] != 0x39;
                    c.g[S1][0] = c.g[A1][0];
                    if not_row {
                        pc = 0x001c_c004;
                    } else {
                        pc = 0x001c_bfe4;
                    }
                }
                0x001c_bfe4 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4b;
                    c.g[A2][0] = c.g[SP][0];
                    callee::KEYBOARD_IS_KEY_ENABLED.call(c, 0x001c_bff0)?;
                    let missing = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    if missing {
                        pc = 0x001c_c060;
                    } else {
                        pc = 0x001c_bff8;
                    }
                }
                0x001c_bff8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x4b;
                    pc = 0x001c_bffc;
                }
                0x001c_bffc => {
                    c.cycles += 2;
                    Addr(p(c.g[S0][0], 0x140)).sw(c, c.g[V0][0]);
                    pc = 0x001c_c074;
                }
                0x001c_c004 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3a));
                    c.g[V0][0] = (c.g[V0][0] < 0x8) as u64;
                    let not_grid = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if not_grid {
                        pc = 0x001c_c030;
                    } else {
                        pc = 0x001c_c014;
                    }
                }
                0x001c_c014 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4b;
                    c.g[A2][0] = c.g[SP][0];
                    callee::KEYBOARD_IS_KEY_ENABLED.call(c, 0x001c_c020)?;
                    let enabled = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x4b;
                    if enabled {
                        if c.cycles >= c.next_event {
                            c.pc = 0x001c_bffc;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x001c_bffc;
                    } else {
                        pc = 0x001c_c028;
                    }
                }
                0x001c_c028 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    pc = 0x001c_c060;
                }
                0x001c_c030 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x42;
                    let not_last = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0;
                    if not_last {
                        pc = 0x001c_c078;
                    } else {
                        pc = 0x001c_c03c;
                    }
                }
                0x001c_c03c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x4c;
                    c.g[A2][0] = c.g[SP][0];
                    callee::KEYBOARD_IS_KEY_ENABLED.call(c, 0x001c_c04c)?;
                    let missing = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x4c;
                    if missing {
                        pc = 0x001c_c05c;
                    } else {
                        pc = 0x001c_c054;
                    }
                }
                0x001c_c054 => {
                    c.cycles += 2;
                    Addr(p(c.g[S0][0], 0x140)).sw(c, c.g[V0][0]);
                    pc = 0x001c_c074;
                }
                0x001c_c05c => {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    pc = 0x001c_c060;
                }
                0x001c_c060 => {
                    c.cycles += 2;
                    let empty = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S1][0] < 1) as u64;
                    if empty {
                        pc = 0x001c_c06c;
                    } else {
                        pc = 0x001c_c068;
                    }
                }
                0x001c_c068 => {
                    c.cycles += 1;
                    Addr(p(c.g[S0][0], 0x144)).sw(c, c.g[V0][0]);
                    pc = 0x001c_c06c;
                }
                0x001c_c06c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    callee::KEYBOARD_FIND_ENABLED_FROM_TOP.call(c, 0x001c_c074)?;
                    pc = 0x001c_c074;
                }
                0x001c_c074 => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    pc = 0x001c_c078;
                }
                0x001c_c078 => {
                    c.cycles += 5;
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S1, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001c_bfc0, 0x001c_c08c);
                    return Ok(());
                }
            }
        }
    });
}
/// `void WScriptProcessList_DeleteBySection(WScriptProcessList *list,
/// int section)` (0x30b0e8): unlink and destroy every process whose section
/// matches, stopping at the first non-matching node. Re-entry points at
/// 0x30b118 and 0x30b168.
pub fn wscript_process_list_delete_by_section(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        // Opened at the entry arm, reused by the resume arms.
        let mut frame = Frame { size: 0x40 };
        loop {
            match pc {
                0x0030_b0e8 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[S2][0] = c.g[A1][0];
                    if empty {
                        pc = 0x0030_b19c;
                    } else {
                        pc = 0x0030_b10c;
                    }
                }
                0x0030_b10c => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[V1][0], 0x24)).lw(c);
                    let other = c.g[V0][0] != c.g[S2][0];
                    c.g[S0][0] = Addr(p(c.g[V1][0], 0x18)).lw(c);
                    if other {
                        pc = 0x0030_b150;
                    } else {
                        pc = 0x0030_b118;
                    }
                }
                0x0030_b118 => {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if none {
                        pc = 0x0030_b138;
                    } else {
                        pc = 0x0030_b124;
                    }
                }
                0x0030_b124 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(c.g[A2][0], 0x5c)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_b138)?;
                    pc = 0x0030_b138;
                }
                0x0030_b138 => {
                    c.cycles += 3;
                    Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
                    let empty = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S0][0];
                    if empty {
                        pc = 0x0030_b19c;
                    } else {
                        pc = 0x0030_b144;
                    }
                }
                0x0030_b144 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[V0][0], 0x24)).lw(c);
                    let matches = c.g[V0][0] == c.g[S2][0];
                    c.g[S0][0] = Addr(p(c.g[S0][0], 0x18)).lw(c);
                    if matches {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0030_b118;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0030_b118;
                    } else {
                        pc = 0x0030_b150;
                    }
                }
                0x0030_b150 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let empty = c.g[A0][0] == 0;
                    c.lq(16, frame.at(c, 0x30));
                    if empty {
                        pc = 0x0030_b1a0;
                    } else {
                        pc = 0x0030_b15c;
                    }
                }
                0x0030_b15c => {
                    c.cycles += 2;
                    c.g[S0][0] = Addr(p(c.g[A0][0], 0x18)).lw(c);
                    pc = 0x0030_b184;
                }
                0x0030_b164 => {
                    c.cycles += 1;
                    pc = 0x0030_b168;
                }
                0x0030_b168 => {
                    c.cycles += 2;
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if none {
                        pc = 0x0030_b184;
                    } else {
                        pc = 0x0030_b170;
                    }
                }
                0x0030_b170 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(c.g[A2][0], 0x5c)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_b184)?;
                    pc = 0x0030_b184;
                }
                0x0030_b184 => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.lq(16, frame.at(c, 0x30));
                        pc = 0x0030_b1a0;
                    } else {
                        pc = 0x0030_b18c;
                    }
                }
                0x0030_b18c => {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[V0][0] = Addr(p(c.g[S0][0], 0x24)).lw(c);
                    let matches = c.g[V0][0] == c.g[S2][0];
                    c.g[S0][0] = Addr(p(c.g[A2][0], 0x18)).lw(c);
                    if matches {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0030_b168;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0030_b168;
                    } else {
                        pc = 0x0030_b19c;
                    }
                }
                0x0030_b19c => {
                    c.cycles += 1;
                    c.lq(16, frame.at(c, 0x30));
                    pc = 0x0030_b1a0;
                }
                0x0030_b1a0 => {
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030_b0e8, 0x0030_b1b4);
                    return Ok(());
                }
            }
        }
    });
}
/// `void cSetupMan_HostRecvMsg9(cSetupMan *self, u32 a1, u32 a2)` (0x269538):
/// same 0x14-byte message layout as the client variant, with `+8 = 1` and
/// the node spliced after `*(gp-0xC8)+0xF0`.
pub fn csetup_man_host_recv_msg9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = 0x14;
        frame.save(c, S0, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x0048_0f18;
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0026_956c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xfc;
        c.g[V0][0] = 0x0048_1130;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = 0x1;
        Addr(p(c.g[S2][0], 0x8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = 0x0048_04a8;
        Addr(p(c.g[S2][0], 0x4)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S2][0], 0xc)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0xc;
        Addr(p(c.g[S2][0], 0x10)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[V1][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(p(c.g[V1][0], 0xf0)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_95b8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            Addr(p(c.g[A1][0], 0x8)).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x50);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        frame.restore(c, S2, 0x30);
        Addr(p(c.g[A1][0], 0x4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(p(c.g[V0][0], 0x4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `void KeyboardScreen_GetCursorObjectName(int key, char *out, char *dst)`
/// (0x180a90): copy an eight-byte prefix and append the character (or the
/// named function key) for `key` into `out`.
pub fn keyboard_screen_get_cursor_object_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = 0x004a_0000;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 10) as u64;
        c.g[T1][0] = 0x004a_1368;
        c.ldl(6, p(c.g[T1][0], 0x7));
        c.ldr(6, c.g[T1][0] as u32);
        c.sdl(6, p(c.g[A0][0], 0x7));
        c.sdr(6, c.g[A0][0] as u32);
        let digit = c.g[V1][0] != 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
        let mut append = digit;
        if !digit {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 36) as u64;
            let not_upper = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x37));
            append = !not_upper;
        }
        if append {
            c.cycles += 6;
            Addr(frame.at(c, 0x1)).sb(c, 0);
            Addr(frame.at(c, 0x0)).sb(c, c.g[V0][0]);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0041_62d0, 0x0018_0adc)?;
            frame.restore_ra(c, 0x10);
        } else {
            // Named function keys: labels b_180b04, b_180b20, b_180b3c.
            c.cycles += 3;
            if c.g[A1][0] == 0x24 {
                c.cycles += 5;
                c.g[A1][0] = 0x004a_1350;
                call_ptr(c, 0x0041_62d0, 0x0018_0afc)?;
                frame.restore_ra(c, 0x10);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            if c.g[A1][0] == 0x25 {
                c.cycles += 5;
                c.g[A1][0] = 0x0045_d560;
                call_ptr(c, 0x0041_62d0, 0x0018_0b18)?;
                frame.restore_ra(c, 0x10);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            if c.g[A1][0] == 0x26 {
                c.cycles += 5;
                c.g[A1][0] = 0x004a_1358;
                call_ptr(c, 0x0041_62d0, 0x0018_0b34)?;
                frame.restore_ra(c, 0x10);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            frame.restore_ra(c, 0x10);
            if c.g[A1][0] != 0x27 {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[A1][0] = 0x004a_1360;
            call_ptr(c, 0x0041_62d0, 0x0018_0b50)?;
            frame.restore_ra(c, 0x10);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_OnActivate(cFEStateRewardGallery *self)`
/// (0x1cf168): take the reward object's screen position into the state's
/// scroll anchor, then update the selected page.
pub fn cfe_state_reward_gallery_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(c.g[S0][0], 0x924)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V1][0] = Addr(p(c.g[S0][0], 0x48)).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0290, 0x001c_f18c)?;
            c.g[A0][0] = Addr(p(c.g[S0][0], 0x8f8)).lw(c);
            if c.g[A0][0] == 0 {
                c.g[V1][0] = Addr(p(c.g[S0][0], 0x48)).lw(c);
            } else {
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_0290, 0x001c_f1a0)?;
                c.lwc1(1, p(c.g[SP][0], 0x14));
                c.lwc1(3, p(c.g[SP][0], 0x4));
                c.lwc1(0, p(c.g[SP][0], 0x10));
                c.lwc1(2, c.g[SP][0] as u32);
                c.sub_s(1, 1, 3);
                c.sub_s(0, 0, 2);
                c.swc1(1, p(c.g[S0][0], 0x74));
                c.swc1(0, p(c.g[S0][0], 0x70));
                c.g[V1][0] = Addr(p(c.g[S0][0], 0x48)).lw(c);
            }
        }
        c.cycles += 4;
        c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
        c.g[V0][0] &= 1;
        let no_page = c.g[V0][0] == 0;
        Addr(p(c.g[S0][0], 0x64)).sw(c, 0);
        if !no_page {
            c.cycles += 4;
            c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 1) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                c.cycles += 3;
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x8fc)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x924)).lw(c);
                Addr(p(c.g[S0][0], 0x8fc)).sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
            }
            c.cycles += 10;
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x140)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x144)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_f20c)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x108)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x10c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_f224)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_ShowEquipMode(cFEStateCharEquip *self)` (0x19a238):
/// refresh the equip-mode labels, the preview panel, and the rider preview.
pub fn cfe_state_char_equip_show_equip_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x48)).lw(c);
        let no_label = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_label {
            c.cycles += 5;
            c.g[A0][0] = 0x0046_0820;
            call_ptr(c, 0x0031_7670, 0x0019_a25c)?;
            c.g[A0][0] = Addr(p(c.g[S0][0], 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0019_a268)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(c.g[S0][0], 0x60)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a288)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(c.g[S0][0], 0x88)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2a8)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(c.g[S0][0], 0x80)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = sx(world(c));
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2cc)?;
            c.g[V0][0] = sx(world(c));
        }
        c.cycles += 13;
        c.g[A1][0] = Addr(p(c.g[S0][0], 0x44)).lb(c);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FEMODULE_GET_RIDER_PREVIEW.call(c, 0x0019_a2e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a2ec)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        Addr(p(c.g[S0][0], 0xae0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.g[RA][0];
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `int InputSync_ApplyInput(InputSync *self)` (0x2379c8): when the sync is
/// ready, feed every pending input slot to its handler.
pub fn inputsync_apply_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[V1][0], 0x70)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 14) as u64;
        let early = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A0][0];
        let mut count = 0u64;
        if !early {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            callee::INPUTSYNC_IS_READY.call(c, 0x0023_79fc)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(c.g[S1][0], 0x10)).lw(c);
                count = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 7;
                frame.restore(c, S0, 0x40);
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 7;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // Walk the input slots.
        c.cycles += 2;
        c.g[S2][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 3;
            c.g[S3][0] = 0x64;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            loop {
                c.cycles += 6;
                c.g[A0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
                c.div(0, A0, V0);
                c.cycles += 15;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = Addr(p(c.g[S0][0], 0x10)).lw(c);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, S3);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x0032_1298, 0x0023_7a58)?;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(p(c.g[S0][0], 0x4)).sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(p(c.g[S1][0], 0x10)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_7a20);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOptionsDeviceSelect_OnWidgetNotify(self, void *widget,
/// int id)` (0x18ddd0): map a widget id to the menu's return code (0x100 for
/// "previous", 0x101 otherwise); for ids 6/8 it also notifies the selected
/// device entry.
pub fn cfe_state_options_device_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(world(c));
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0022_7f80, 0x0018_ddf4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x8;
        let is8 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 9) as u64;
        if is8 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S2][0], 0x22c)).lw(c);
            c.g[A2][0] = Addr(p(c.g[S1][0], 0x434)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A1][0] = Addr(p(c.g[S1][0], 0x428)).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(p(c.g[V1][0], 0x1b0)).lh(c);
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x1b4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_de54)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = 0x101;
                if nonzero {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A2][0] = Addr(p(c.g[S1][0], 0x434)).lw(c);
            } else {
                c.g[A2][0] = Addr(p(c.g[S1][0], 0x434)).lw(c);
            }
            // The shared device-entry notification (label b_18de60).
            c.cycles += 8;
            c.g[A1][0] = Addr(p(c.g[S1][0], 0x428)).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0], 0x1b0)).lh(c);
            c.g[V0][0] = Addr(p(c.g[V1][0], 0x1b4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_de78)?;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = 0x101;
            if nonzero {
                c.cycles += 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            }
            frame.restore(c, S0, 0x30);
        } else if c.g[S0][0] >= 9 {
            c.cycles += 2;
            c.cycles += 3;
            if c.g[S0][0] == 9 {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x101;
            }
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 2;
            if c.g[S0][0] == 6 {
                c.cycles += 2;
                c.cycles += 1;
                c.g[A2][0] = Addr(p(c.g[S1][0], 0x434)).lw(c);
                c.cycles += 8;
                c.g[A1][0] = Addr(p(c.g[S1][0], 0x428)).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(p(c.g[V1][0], 0x1b0)).lh(c);
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x1b4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_de78)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = 0x101;
                if nonzero {
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 2;
                c.cycles += 2;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Character-equip item flags and bolt index nodes.

/// `int cFEStateCharEquip_CategoryHasFlaggedItem(self, categoryList, db,
/// int unused)` (0x19afd0): collect the category's items and report whether
/// any enabled one is flagged in the database.
pub fn cfe_state_char_equip_category_has_flagged_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x860);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x850);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0x840);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[T0][0] = u64::MAX;
        c.g[A1][0] = Addr(p(c.g[V0][0], 0xbc)).lw(c);
        c.g[A3][0] = c.g[SP][0];
        c.g[T1][0] = 0x1;
        call_ptr(c, 0x0014_d7e8, 0x0019_b000)?;
        c.g[A3][0] = c.g[V0][0];
        let none = (c.g[A3][0] as i64) <= 0;
        c.g[A2][0] = 0;
        let mut found = false;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = c.g[SP][0];
            loop {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V0][0] = Addr(p(c.g[A0][0], 0x34)).lw(c) & 0x4;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p(c.g[A0][0], 0x4)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[S0][0], 0x288)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
                    let negative = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if negative {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p(c.g[V0][0], 0x2)).lhu(c) & 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p(c.g[A0][0], 0x34)).lw(c) & 0x20;
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            found = true;
                            break;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b010);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x850);
        frame.restore_ra(c, 0x840);
        frame.close(c);
        Ok(())
    })
}

/// `void BoltFileIndexManager_FreeNode(BoltFileIndexManager *self, Node *n)`
/// (0x19ca70): unlink the node from the bucket list, clear it and push it on
/// the free list.
pub fn bolt_file_index_manager_free_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        let none = c.g[S0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = 0x190;
            c.cycles += 5;
            c.div(0, A0, V0);
            c.g[V1][0] = Addr(p(c.g[S0][0], 0xc)).lh(c);
            c.g[S2][0] = c.hi[0];
            let has_prev = (c.g[V1][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !has_prev {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                callee::BOLT_GET_NEXT_NODE.call(c, 0x0019_cabc)?;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr(p(c.g[V1][0], 0x1900)).sw(c, c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::BOLT_GET_PREV_NODE.call(c, 0x0019_cad4)?;
            c.g[V1][0] = Addr(p(c.g[S0][0], 0xe)).lhu(c);
            Addr(p(c.g[V0][0], 0xe)).sh(c, c.g[V1][0]);
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0xe)).lh(c);
            let no_next = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S1][0];
            if !no_next {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                callee::BOLT_GET_NEXT_NODE.call(c, 0x0019_caf0)?;
                c.g[V1][0] = Addr(p(c.g[S0][0], 0xc)).lhu(c);
                Addr(p(c.g[V0][0], 0xc)).sh(c, c.g[V1][0]);
            }
            c.cycles += 10;
            c.g[V0][0] = u64::MAX;
            c.g[A0][0] = c.g[S1][0];
            Addr(p(c.g[S0][0], 0xc)).sh(c, c.g[V0][0]);
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            Addr(p(c.g[S0][0], 0xa)).sh(c, c.g[V0][0]);
            Addr(p(c.g[S0][0], 0x8)).sh(c, c.g[V0][0]);
            c.g[A1][0] = Addr(p(c.g[S1][0], 0x1f40)).lw(c);
            callee::BOLT_NODE_INDEX.call(c, 0x0019_cb18)?;
            Addr(p(c.g[S0][0], 0xe)).sh(c, c.g[V0][0]);
            Addr(p(c.g[S1][0], 0x1f40)).sw(c, c.g[S0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_ResetMotion(Rider *self, int notify)` (0x11df18): zero the
/// rider's motion, scale its velocity row by 1/833.33, reset the FX state and
/// (for a notify) push the new velocity into the rider's AI object.
pub fn rider_reset_motion(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x2;
        c.lq(2, p(c.g[S1][0], 0x1b0));
        c.g[A0][0] = Addr(p(c.g[S1][0], 0x784)).lw(c);
        c.sq(2, c.g[SP][0] as u32);
        call_ptr(c, 0x0031_1b20, 0x0011_df44)?;
        Addr(p(c.g[V0][0], 0x90)).sw(c, 0);
        c.lwc1(0, gp(c, -0x7988));
        c.mfc1(3, 0);
        c.lqc2(4, c.g[SP][0] as u32);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p(c.g[SP][0], 0x20));
        c.lq(2, p(c.g[SP][0], 0x20));
        c.g[A0][0] = Addr(p(c.g[S1][0], 0x77c)).lw(c);
        c.sq(2, p(c.g[S1][0], 0x1e0));
        Addr(p(c.g[S1][0], 0x1e8)).sw(c, 0);
        c.sq(2, p(c.g[SP][0], 0x10));
        callee::RIDER_BASE_RESET_FX.call(c, 0x0011_df78)?;
        let no_notify = c.g[S0][0] == 0;
        c.g[A0][0] = sx(world(c));
        if !no_notify {
            c.cycles += 17;
            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = Addr(p(c.g[S1][0], 0x6c0)).lw(c);
            c.g[V0][0] = Addr(p(c.g[A0][0], 0x84)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0], 0x38)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V0][0], 0x84)).lw(c);
            c.g[V0][0] = Addr(p(c.g[V1][0], 0x3c)).lw(c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(p(c.g[A1][0], 0x4)).lw(c);
            c.g[S0][0] = Addr(p(c.g[V1][0], 0xa0)).lw(c);
            c.g[S1][0] = Addr(p(c.g[S0][0], 0x14)).lw(c);
            c.g[V1][0] = Addr(p(c.g[S1][0], 0x18)).lh(c);
            let target = c.g[V0][0] as u32;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0011_dfb4)?;
            c.g[V1][0] = Addr(p(c.g[S1][0], 0x1c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x0011_dfc4)?;
        }
        c.cycles += 7;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_99f8, 0x0011_dfcc)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card / load screens.

/// `bool MemCardStateBase_IsCardReady(MemCardStateBase *self)` (0x1d9880):
/// the card is ready when the state is neither "none" (0) nor checking (6),
/// the menu entry is set, the dialog is idle and the port check passes.
pub fn memcard_state_base_is_card_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = sx(world(c));
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0022_7f80, 0x001d_98a4)?;
        c.g[V1][0] = Addr(p(c.g[S0][0], 0x1c0)).lw(c);
        let no_state = c.g[V1][0] == 0;
        c.g[S1][0] = c.g[V0][0];
        let mut ready = 0u64;
        if !no_state {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = c.g[S2][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S1][0], 0x340)).lw(c);
                let idle = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S2][0];
                if idle {
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0xe0)).lw(c);
                    let busy = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S2][0];
                    if busy {
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(p(c.g[S1][0], 0x434)).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(p(c.g[V1][0], 0x58)).lh(c);
                        c.g[V0][0] = Addr(p(c.g[V1][0], 0x5c)).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x001d_98ec)?;
                        let port = c.g[V0][0] != 0;
                        c.g[V0][0] = c.g[S2][0];
                        if port {
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = Addr(p(c.g[S1][0], 0x130)).lw(c);
                            c.g[V0][0] = c.g[V1][0] ^ 0x6;
                            let is6 = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[V1][0] ^ 0x1;
                            if is6 {
                                ready = 0;
                            } else {
                                c.cycles += 2;
                                let is1 = c.g[V0][0] == 0;
                                c.g[V0][0] = c.g[S2][0];
                                if is1 {
                                } else {
                                    c.cycles += 8;
                                    c.g[A1][0] = Addr(p(c.g[S1][0], 0x434)).lw(c);
                                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                    c.g[A0][0] = Addr(p(c.g[V1][0], 0x258)).lh(c);
                                    c.g[V0][0] = Addr(p(c.g[V1][0], 0x25c)).lw(c);
                                    let target = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, target, 0x001d_9924)?;
                                    c.g[V1][0] = 0x1;
                                    if c.g[V0][0] != 0 {
                                        c.g[S2][0] = c.g[V1][0];
                                    }
                                    ready = c.g[S2][0];
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = ready;
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InitLoadScreen_Update(InitLoadScreen *self)` (0x232588): advance the
/// loading indicator and fade the progress bar in or out.
pub fn init_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(world(c));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(c.g[V0][0], 0xa4)).lw(c);
        callee::LOAD_INDICATOR_ADVANCE.call(c, 0x0023_25a4)?;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
        let fade_out = c.g[V0][0] == 0;
        c.g[V0][0] = sx(world(c));
        let fcc;
        if !fade_out {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x518c));
            c.g[V0][0] = Addr(p(c.g[V0][0], 0xa4)).lw(c);
            c.mtc1(0, 2);
            c.lwc1(0, p(c.g[V0][0], 0x4));
            c.sub_s(0, 0, 1);
            c.c_lt_s(0, 2);
            fcc = c.fcc();
            c.swc1(0, p(c.g[V0][0], 0x4));
        } else {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x5188));
            c.g[V0][0] = Addr(p(c.g[V0][0], 0xa4)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.lwc1(0, p(c.g[V0][0], 0x4));
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            fcc = c.fcc();
            c.swc1(0, p(c.g[V0][0], 0x4));
        }
        c.cycles += 2;
        if fcc {
            c.swc1(2, p(c.g[V0][0], 0x4));
        }
        c.cycles += 4;
        c.g[V0][0] = sx(world(c));
        c.g[V1][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 15;
            c.g[A1][0] = 0x0047_b8d8;
            c.g[A0][0] = 0x1c;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0023_2620)?;
            c.g[V1][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.g[V0][0];
            if c.g[V1][0] == 0 {
                c.g[A1][0] = 0;
            }
            callee::START_SCREEN_CTOR.call(c, 0x0023_2634)?;
            c.g[A0][0] = sx(world(c));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0023_2640)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_LoadMapImage(EventSelect *self)` (0x203990): request the
/// selected event's map texture and latch which variant was loaded.
pub fn event_select_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = sx(world(c));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x20);
        c.g[V1][0] = Addr(gp(c, -0xb54)).lw(c);
        let active = c.g[V1][0] != 0;
        c.g[S1][0] = Addr(p(c.g[V0][0], 0x11c)).lw(c);
        if !active {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x2d4)).lw(c);
            c.g[S2][0] = 1;
            if c.g[V0][0] == c.g[S2][0] {
                frame.restore(c, S0, 0x30);
                c.cycles += 5;
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 10;
            c.g[A1][0] = Addr(p(c.g[S0][0], 0x2d8)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_38c0, 0x0020_39d4)?;
            c.g[A1][0] = Addr(p(c.g[S0][0], 0x2d8)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x001a_36e8, 0x0020_39e4)?;
            Addr(p(c.g[S0][0], 0x2d4)).sw(c, c.g[S2][0]);
            Addr(p(c.g[S0][0], 0x2dc)).sw(c, 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // Async request in flight.
        c.cycles += 2;
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        if negative {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(p(c.g[S0][0], 0x2d4)).lw(c);
        c.g[V0][0] = Addr(gp(c, -0xb4c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(2));
        if c.g[V1][0] == c.g[V0][0] {
            frame.restore(c, S0, 0x30);
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 12;
        c.g[A1][0] = Addr(p(c.g[S0][0], 0x2d8)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a_38c0, 0x0020_3a1c)?;
        c.g[A1][0] = Addr(gp(c, -0xb4c)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(p(c.g[S0][0], 0x2d8)).lw(c);
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x001a_3768, 0x0020_3a30)?;
        c.g[V0][0] = Addr(gp(c, -0xb4c)).lw(c);
        Addr(p(c.g[S0][0], 0x2dc)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(2));
        Addr(p(c.g[S0][0], 0x2d4)).sw(c, c.g[V0][0]);
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Heap, parser and engine teardown.

/// `void VramHeap_FreeUnlocked(VramHeap *self)` (0x366548): walk the block
/// index from `+0x23c0`, coalescing and freeing every unlocked block.
pub fn vram_heap_free_unlocked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(c.g[S1][0], 0x23c0)).lw(c);
        let mut index = c.g[A1][0];
        c.g[S2][0] = 0x1c;
        if (index as i64) < 0 {
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        c.mult(0, V1, A1, S2);
        loop {
            // Outer walk: locate the bucket containing `index`.
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[S1][0], 0x1ff0)).lw(c);
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = Addr(p(c.g[A3][0], 0x10)).lw(c);
            let negative = (c.g[S0][0] as i64) < 0;
            c.g[T0][0] = c.g[V0][0];
            if negative {
                c.cycles += 1;
                c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
            } else {
                c.cycles += 6;
                c.mult(0, V0, S0, S2);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                if c.g[V0][0] & 1 != 0 {
                    c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[A0][0] = 0x1c;
                    c.mult(0, V1, S0, A0);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[S0][0] = Addr(p(c.g[V0][0], 0x10)).lw(c);
                        let negative = (c.g[S0][0] as i64) < 0;
                        c.mult(0, V0, S0, A0);
                        if negative {
                            c.cycles += 1;
                            c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
                            break;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        let free = (c.g[V0][0] & 1) == 0;
                        c.mult(0, V1, S0, A0);
                        if free {
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_65a8);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
                        break;
                    }
                }
            }
            // Coalesce or free the block.
            c.cycles += 3;
            let head = (c.g[V1][0] & 1) == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
            if head {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[S0][0];
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0036_63d8, 0x0036_65f0)?;
                    c.g[A1][0] = c.g[S0][0];
                }
            }
            c.cycles += 2;
            let more = (c.g[A1][0] as i64) >= 0;
            c.mult(0, V1, A1, S2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_6570);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_ClientRecvMsg8(cSetupMan *self, u32 a1, u32 a2)`
/// (0x269948): queue message kind 0xFC with the extra word 1.
pub fn csetup_man_client_recv_msg8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = 0x14;
        frame.save(c, S0, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x0048_0f18;
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0026_997c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xfc;
        c.g[V0][0] = 0x0048_1130;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = 1;
        Addr(p(c.g[S2][0], 0x8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = 0x0048_04a8;
        Addr(p(c.g[S2][0], 0x4)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S2][0], 0xc)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0xc;
        Addr(p(c.g[S2][0], 0x10)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[V1][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(p(c.g[V1][0], 0xf0)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_99c8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            Addr(p(c.g[A1][0], 0x8)).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x50);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        frame.restore(c, S2, 0x30);
        Addr(p(c.g[A1][0], 0x4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(p(c.g[V0][0], 0x4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Close(VidEnginePlayer *self)` (0x2535f8): release the
/// player's texture/handle objects and clear its fields.
pub fn vid_engine_player_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x38)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x40)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x3c)).lw(c);
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = Addr(gp(c, 0x2a90)).lw(c);
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A1][0], 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x98)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0], 0x9c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0025_3634)?;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x40)).lw(c);
        }
        c.cycles += 2;
        let none = c.g[V0][0] == 0;
        let mut obj = 0u64;
        if none {
            obj = Addr(p(c.g[S0][0], 0x3c)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(p(c.g[S0][0], 0xc)).lw(c);
            if c.g[A1][0] == 0 {
                obj = Addr(p(c.g[S0][0], 0x3c)).lw(c);
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x38)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x64)).lw(c);
                call_ptr(c, 0x003b_0680, 0x0025_3658)?;
                obj = Addr(p(c.g[S0][0], 0x3c)).lw(c);
            }
        }
        c.cycles += 2;
        if obj != 0 {
            c.cycles += 9;
            c.g[V0][0] = Addr(p(obj, 0x4)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0025_3678)?;
            c.g[V0][0] = Addr(gp(c, -0x254)).lw(c);
            Addr(p(c.g[S0][0], 0x3c)).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            Addr(gp(c, -0x254)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(c.g[S0][0], 0x38)).lw(c);
        if c.g[A0][0] == 0 {
            Addr(p(c.g[S0][0], 0x40)).sw(c, 0);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x003a_e300, 0x0025_369c)?;
            Addr(p(c.g[S0][0], 0x38)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x40)).sw(c, 0);
        }
        c.cycles += 8;
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p(c.g[S0][0], 0x4)).sw(c, 0);
        Addr(p(c.g[S0][0], 0x8)).sw(c, 0);
        Addr(p(c.g[S0][0], 0xc)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InputParser_CompileIdentifier(InputParser *self, ...)` (0x3245a8):
/// look the identifier up and either emit its node or raise the parser error.
pub fn input_parser_compile_identifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[T0][0];
        call_ptr(c, 0x0032_1a40, 0x0032_45dc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A1][0] = 0x0048_e208;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, 0x0032_1500, 0x0032_45f8)?;
            c.g[V0][0] = u64::MAX;
        } else {
            c.g[A1][0] = Addr(p(c.g[V0][0], 0x40)).lw(c);
            c.cycles += 12;
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[T1][0] = c.g[S3][0];
            Addr(frame.at(c, 0x0)).sw(c, 0);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0032_4678, 0x0032_461c)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            let empty = (c.g[S0][0] as i64) <= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if empty {
                c.cycles += 5;
                c.g[A1][0] = 0x0048_e228;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1500, 0x0032_4650)?;
                c.g[V0][0] = u64::MAX;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x20;
                Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_Write(InputPreproc *self, int c)` (0x326360): append a
/// character to the growing buffer.
pub fn input_preproc_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
        let not_full = c.g[V1][0] != c.g[V0][0];
        c.g[S3][0] = sx((((c.g[A1][0] as u32) as i32) >> 24) as u32);
        if !not_full {
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x400));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = 0x0048_e3b8;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = 0;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 1) as u32);
            c.g[A3][0] = 0;
            c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[S1][0] = c.g[V0][0];
            }
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e30, 0x0032_63c8)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
            c.g[A2][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0041_605c, 0x0032_63dc)?;
            c.g[A0][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0032_63f0)?;
            }
            Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[S2][0]);
            c.cycles += 2;
            Addr(p(c.g[S0][0], 0x4)).sw(c, c.g[S1][0]);
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        }
        c.cycles += 12;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore(c, S2, 0x20);
        Addr(c.g[V0][0] as u32).sb(c, c.g[S3][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldCache_StreamerReset(cWorldCache *self)` (0x3a9188): clear the
/// streamer's view slots and reset both view descriptors.
pub fn cworldcache_streamer_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0050_0000;
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        Addr(c.g[S2][0] as u32).sw(c, 0);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        Addr(p(c.g[S2][0], 0x4)).sw(c, 0);
        Addr(p(c.g[S2][0], 0x8)).sw(c, 0);
        Addr(p(c.g[S2][0], 0xc)).sw(c, 0);
        Addr(p(c.g[S2][0], 0x10)).sw(c, 0);
        c.lq(3, c.g[V0][0] as u32);
        c.sq(3, p(c.g[S2][0], 0x220));
        c.lq(4, c.g[V0][0] as u32);
        c.sq(4, p(c.g[S2][0], 0x230));
        loop {
            c.cycles += 7;
            Addr((c.g[A1][0] as u32).wrapping_sub(0x4)).sw(c, 0);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            Addr(c.g[A1][0] as u32).sw(c, 0);
            c.g[V0][0] = (c.g[S1][0] < 0x40) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_91e0);
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x240));
        loop {
            c.cycles += 10;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A1][0] = c.g[S1][0];
            c.mtc1(0, 12);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xed0));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            call_ptr(c, 0x003a_9658, 0x003a_9224)?;
            c.g[V0][0] = (c.g[S1][0] < 0x2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_9208);
            }
        }
        c.cycles += 9;
        Addr(p(c.g[S2][0], 0x2e4)).sw(c, 0);
        Addr(p(c.g[S2][0], 0x2e0)).sw(c, 0);
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
// Crowd meshes and playback teardown.

/// `void CrowdMan3d_GroupSetMeshFrame(CrowdMan3d *self, int frame)`
/// (0x344730): copy every animated mesh's frame index into the group's
/// per-instance table.
pub fn crowd_man3d_group_set_mesh_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x002d_1bd8, 0x0034_474c)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A1][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let mut t0;
        if c.g[V1][0] != 0 {
            c.g[A0][0] = Addr(p(c.g[V1][0], 0x1c)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            t0 = c.g[V0][0];
        } else {
            c.cycles += 2;
            t0 = 0;
        }
        c.g[T0][0] = t0;
        c.cycles += 3;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x110)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x13c));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(c.g[T0][0], 0x94)).lw(c);
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x4)).lw(c);
                c.g[V0][0] = Addr(p(c.g[A0][0], 0x10)).lw(c);
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if !none {
                    c.cycles += 2;
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0x4)).lhu(c);
                    Addr(c.g[A3][0] as u32).sh(c, c.g[V1][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x110)).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_47b0);
                }
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

/// `void Replay_EndPlayback(Replay *self)` (0x26f980): cancel instant replay,
/// clear the replay state and restore the appropriate camera.
pub fn replay_end_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        call_ptr(c, 0x0027_0628, 0x0026_f9b0)?;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x61c)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x608)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x620)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x608)).lw(c);
            } else {
                c.cycles += 5;
                call_ptr(c, 0x0028_b180, 0x0026_f9d0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0028_f2c0, 0x0026_f9d8)?;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x608)).lw(c);
            }
        }
        c.cycles += 4;
        Addr(p(c.g[S0][0], 0x61c)).sw(c, 0);
        Addr(p(c.g[S0][0], 0x620)).sw(c, 0);
        let some = c.g[V0][0] != 0;
        Addr(p(c.g[S0][0], 0x624)).sw(c, 0);
        if some {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0026_f850, 0x0026_f9f4)?;
        c.g[V0][0] = Addr(p(c.g[S0][0], 0x620)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_ef80, 0x0026_fa08)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x60c)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_eea0, 0x0026_fa18)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x60c)).lw(c);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0026_f4a0, 0x0026_fa3c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0026_f4a0, 0x0026_fa2c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIEngine_dtor(UIEngine *self, int flags)` (0x397b70): destroy the
/// engine's child lists, texture set and state manager, then free the object.
pub fn ui_engine_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x4)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_7ba8)?;
        }
        // Remaining sequence.
        c.cycles += 3;
        c.g[A0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = Addr(p(c.g[S0][0], 0xc)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0039_7bbc)?;
            c.g[A2][0] = Addr(p(c.g[S0][0], 0xc)).lw(c);
        }
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_7bdc)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(c.g[S0][0], 0x14)).lw(c);
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x8)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_7bfc)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0039_8078, 0x0039_7c08)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0039_e9d8, 0x0039_7c14)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0039_7c28)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModule_OnSectionUnload(GameModule *self, int section)`
/// (0x230360): notify every section manager and release the module's crowd
/// and rail resources.
pub fn game_module_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_8700, 0x0023_0384)?;
        c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
        c.g[A0][0] = Addr(gp(c, -0x6f0)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0022_9408, 0x0023_0394)?;
        c.g[A0][0] = Addr(gp(c, 0xf00)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_3c08, 0x0023_03a0)?;
        c.g[A0][0] = Addr(gp(c, 0xf38)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_7b90, 0x0023_03ac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0023_03b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0035_51a8, 0x0023_03c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 8;
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0035_51a8, 0x0023_03d8)?;
        c.g[A0][0] = Addr(p(c.g[S2][0], 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(c.g[A0][0], 0xa4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 7;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0010_3308, 0x0023_03f8)?;
                c.g[V0][0] = Addr(p(c.g[S2][0], 0xc)).lw(c);
                c.g[A1][0] = 0x80;
                c.g[A0][0] = Addr(p(c.g[V0][0], 0xa4)).lw(c);
                call_ptr(c, 0x0010_3308, 0x0023_0408)?;
            }
        }
        c.g[A0][0] = Addr(p(c.g[S2][0], 0x30)).lw(c);
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_8fe0, 0x0023_0414)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateAudioOptions_TransitionIn(cFEStateAudioOptions *self, int a1)`
/// (0x1962a8): refresh the option labels on the way in.
pub fn cfe_state_audio_options_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let skip_all = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !skip_all {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x7c)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 31;
                for (field, ret) in [
                    (0x68u32, 0x1962e4u32),
                    (0x6c, 0x196300),
                    (0x50, 0x19631c),
                    (0x58, 0x196338),
                ] {
                    c.g[A2][0] = Addr(p(c.g[S0][0], field)).lw(c);
                    c.g[A1][0] = 0;
                    c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0], 0x48)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, ret)?;
                }
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x74)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 7;
                    c.g[A2][0] = Addr(p(c.g[S0][0], 0x64)).lw(c);
                    c.g[A1][0] = 0;
                    c.g[V0][0] = Addr(p(c.g[A2][0], 0x8)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0], 0x48)).lh(c);
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0019_6360)?;
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptMovie_Stop(ScriptMovie *self)` (0x283818): stop the movie
/// player and release its resources.
pub fn script_movie_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[A0][0] == 0 {
            Addr(p(c.g[S0][0], 0xc)).sw(c, 0);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0025_3418, 0x0028_383c)?;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
            let no_voice = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(gp(c, -0x69c)).lw(c);
            if !no_voice {
                c.cycles += 26;
                c.g[A0][0] = 0x0053_66e8;
                c.g[A1][0] = 0x0044_28f0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                Addr(gp(c, -0x69c)).sw(c, c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                Addr(gp(c, -0x6a0)).sw(c, c.g[V1][0]);
                call_ptr(c, 0x0028_b180, 0x0028_3880)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0029_ce70, 0x0028_3888)?;
                c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
                c.g[A1][0] = Addr(p(c.g[S0][0], 0x10)).lw(c);
                c.g[V0][0] = Addr(p(c.g[A2][0], 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x140)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0], 0x144)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0028_38a4)?;
                c.g[V0][0] = Addr(p(c.g[S0][0], 0x14)).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    Addr(p(c.g[S0][0], 0x14)).sw(c, 0);
                    c.g[V0][0] = sx(world(c));
                    c.g[V1][0] = Addr(p(c.g[V0][0], 0x84)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V1][0], 0x64)).lw(c);
                    call_ptr(c, 0x002e_a860, 0x0028_38c4)?;
                }
            }
            c.cycles += 1;
            Addr(p(c.g[S0][0], 0xc)).sw(c, 0);
        }
        c.cycles += 7;
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p(c.g[S0][0], 0x4)).sw(c, 0);
        Addr(p(c.g[S0][0], 0x8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ProcessList_Shutdown(ProcessList *self)` (0x354b38): delete every
/// priority bucket and free the pending game processes.
pub fn process_list_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0035_5118, 0x0035_4b68)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S0][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_4b60);
                }
            }
        }
        c.cycles += 3;
        c.g[A1][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
        let none = c.g[A1][0] == 0;
        c.g[V0][0] = 0x44;
        if !none {
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0xffff_fff0).lw(c);
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                loop {
                    c.cycles += 15;
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
                    call_ptr(c, 0x0035_46c8, 0x0035_4bac)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0035_46c8, 0x0035_4bb8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0035_46c8, 0x0035_4bc4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0035_46c8, 0x0035_4bd0)?;
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = Addr(p(c.g[S1][0], 0x4)).lw(c);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_4ba0);
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
        call_ptr(c, 0x0031_7e98, 0x0035_4be8)?;
        c.cycles += 7;
        Addr(c.g[S1][0] as u32).sw(c, 0);
        Addr(p(c.g[S1][0], 0x4)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Load screen, avalanche init, menu cursor, transition and standings helpers.

/// `void cFELoadScreen_Unload(cFELoadScreen *self)` (0x233930): destroy the
/// screen's UI objects, release the load-screen graphics and the meter.
pub fn cfe_load_screen_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = Addr(gp(c, -0x854)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[A1][0], 0x10d8)).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x98)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0], 0x9c)).lw(c);
        let first = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, first, 0x0023_395c)?;
        c.g[A1][0] = Addr(p(c.g[S0][0], 0x18)).lw(c);
        let none = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A2][0], 0x10d8)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x190)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0], 0x194)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_397c)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(c.g[S0][0], 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0039_7b70, 0x0023_3990)?;
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.cycles += 1;
        c.g[S1][0] = 1;
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let none = c.g[A1][0] == 0;
            c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A2][0], 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x190)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0], 0x194)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_39b8)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_3998);
            }
        }
        c.cycles += 13;
        c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_a020, 0x0023_39cc)?;
        call_ptr(c, 0x0028_b180, 0x0023_39d4)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_fa98, 0x0023_39e4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VfxAvalanche_Init(void)` (0x2d6410): clear the 2048 avalanche
/// particles and the 32 trail entries.
pub fn vfx_avalanche_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x004d_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x004d_5b70;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xff;
        loop {
            c.cycles += 15;
            c.g[V0][0] = c.g[A1][0] & 1;
            c.g[V1][0] = c.g[A2][0];
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0;
            }
            Addr(p(c.g[A0][0], 0x1c)).sw(c, c.g[A2][0]);
            Addr(p(c.g[A0][0], 0x18)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            Addr(p(c.g[A0][0], 0x10)).sw(c, c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2048) as u64;
            Addr(p(c.g[A0][0], 0x14)).sw(c, c.g[V1][0]);
            c.swc1(0, p(c.g[A0][0], 0x8));
            Addr(p(c.g[A0][0], 0xc)).sw(c, 0);
            Addr(c.g[A0][0] as u32).sw(c, 0);
            Addr(p(c.g[A0][0], 0x4)).sw(c, 0);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_6430);
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x004f_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = 0x004e_db70;
        c.g[A0][0] = 0xff;
        c.g[A1][0] = 0x004e_e170;
        Addr(p(c.g[V1][0], 0x1c)).sw(c, c.g[A0][0]);
        loop {
            c.cycles += 11;
            Addr(p(c.g[V1][0], 0x10)).sw(c, c.g[A0][0]);
            Addr(p(c.g[V1][0], 0x14)).sw(c, c.g[A0][0]);
            Addr(p(c.g[V1][0], 0x18)).sw(c, 0);
            c.swc1(0, p(c.g[V1][0], 0x8));
            Addr(p(c.g[V1][0], 0xc)).sw(c, 0);
            Addr(c.g[V1][0] as u32).sw(c, 0);
            Addr(p(c.g[V1][0], 0x4)).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            Addr(p(c.g[V1][0], 0x1c)).sw(c, c.g[A0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x002d_6488);
            }
        }
        c.cycles += 9;
        c.g[A0][0] = 0x004c_8450;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x400;
        Addr(gp(c, 0x9d0)).sw(c, 0);
        call_ptr(c, 0x0041_6210, 0x002d_64cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UIMenu_FindFirstSelectable(UIMenu *self, u32 skip)` (0x39b7b0): the
/// first enabled list entry after `skip`. A resumable loop (0x39b7e8).
pub fn ui_menu_find_first_selectable(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
                    let mut frame = Frame { size: 0x30 };
        loop {
            match pc {
                0x0039_b7b0 => {
                    c.cycles += 13;
                    frame = Frame::open(c, 0x30);
                    c.g[A1][0] &= 0xff;
                    frame.save(c, S0, 0x20);
                    c.g[V1][0] = c.g[A1][0] ^ 0xff;
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = 0;
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A0][0];
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0x96)).lbu(c);
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xff;
                    if done {
                        pc = 0x0039_b864;
                    } else {
                        pc = 0x0039_b7e4;
                    }
                }
                0x0039_b7e4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    pc = 0x0039_b7e8;
                }
                0x0039_b7e8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_7870, 0x0039_b7f0)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0x90)).lw(c);
                    c.g[V0][0] &= 0x4;
                    let no_flag = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    if no_flag {
                        pc = 0x0039_b838;
                    } else {
                        pc = 0x0039_b804;
                    }
                }
                0x0039_b804 => {
                    c.cycles += 11;
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0x5c)).lw(c);
                    c.g[A2][0] = 1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = Addr(p(c.g[V0][0], 0xd0)).lw(c);
                    c.g[V1][0] = Addr(p(c.g[T0][0], 0x8)).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V1][0], 0xa0)).lh(c);
                    c.g[V0][0] = Addr(p(c.g[V1][0], 0xa4)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_b828)?;
                    let no = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if no {
                        pc = 0x0039_b84c;
                    } else {
                        pc = 0x0039_b830;
                    }
                }
                0x0039_b830 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0039_b864;
                }
                0x0039_b838 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(c.g[V1][0], 0x14)).lw(c);
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 5) as u32);
                    c.g[V0][0] &= 1;
                    let no = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if no {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0039_b830;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0039_b830;
                    } else {
                        pc = 0x0039_b84c;
                    }
                }
                0x0039_b84c => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(c.g[S1][0], 0x96)).lbu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    if more {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0039_b7e8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0039_b7e8;
                    } else {
                        pc = 0x0039_b860;
                    }
                }
                0x0039_b860 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xff;
                    pc = 0x0039_b864;
                }
                0x0039_b864 => {
                    c.cycles += 5;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_b7b0, 0x0039_b878);
                    return Ok(());
                }
            }
        }
    });
}
/// `void OVScreenBCStart_TransitionIn(OVScreenBCStart *self, int a1)`
/// (0x1f7418): fade/slide the BC-start widgets in.
pub fn ov_screen_bcstart_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        let skip = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !skip {
            'b_1f748c: {
                c.cycles += 15;
                call_ptr(c, 0x0014_dc80, 0x001f_743c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xa;
                call_ptr(c, 0x0014_dd58, 0x001f_7448)?;
                c.g[A0][0] = 0x0046_e4f0;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0031_7670, 0x001f_7458)?;
                c.g[A0][0] = Addr(p(c.g[S1][0], 0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_7464)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.cycles += 7;
                    c.g[A1][0] = Addr(p(c.g[S1][0], 0x9c)).lw(c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0015_3c88, 0x001f_747c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_0e90, 0x001f_7488)?;
                }
                c.g[A0][0] = 0x004a_0000;
            }
            c.cycles += 8;
            c.g[A0][0] = 0x004a_23f0;
            call_ptr(c, 0x0031_7670, 0x001f_7494)?;
            c.g[A0][0] = Addr(p(c.g[S1][0], 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_74a0)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 6;
                c.g[A1][0] = Addr(p(c.g[S1][0], 0x9c)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0015_3d28, 0x001f_74b8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0d00, 0x001f_74c4)?;
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVStateHUD1P_LoadMeterAssets(self, a1, a2)` (0x20eb50): find the
/// meter textures/font and apply the rect.
pub fn cov_state_hud1p_load_meter_assets(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp(c, -0x988)).lw(c);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !ready {
            c.cycles += 7;
            c.g[V0][0] = sx(world(c));
            c.g[V1][0] = 0x004d_0000;
            c.g[S0][0] = 0x004c_8c58;
            c.g[S2][0] = 0xa;
            c.g[A0][0] = Addr(p(c.g[V0][0], 0x84)).lw(c);
            c.g[S1][0] = Addr(p(c.g[A0][0], 0x48)).lw(c);
            loop {
                c.cycles += 7;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0039_8380, 0x0020_ebb0)?;
                Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_eba0);
                }
            }
            c.cycles += 10;
            c.g[A0][0] = 0x004a_21f0;
            c.g[S0][0] = Addr(p(c.g[S1][0], 0x8)).lw(c);
            call_ptr(c, 0x0031_7670, 0x0020_ebcc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_83f0, 0x0020_ebd8)?;
            c.g[V1][0] = 1;
            Addr(gp(c, -0x98c)).sw(c, c.g[V0][0]);
            Addr(gp(c, -0x988)).sw(c, c.g[V1][0]);
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        call_ptr(c, 0x0020_ec18, 0x0020_ebf4)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BE_RecordCharProfileEventScore(self, event, rank, score)` (0x152460):
/// record a profile event score when it beats the stored one.
pub fn be_record_char_profile_event_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0014_ab68, 0x0015_248c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x1a;
        let invalid = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0;
        'b_152514: {
            'b_152510: {
                if !invalid {
                    c.cycles += 2;
                    if c.g[S1][0] != 0 {
                        'b_152508: {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xad0));
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = Addr(p(c.g[A0][0], 0x4)).lw(c);
                            let absent = c.g[V1][0] == 0;
                            c.g[V0][0] = 4;
                            if !absent {
                                'b_152500: {
                                    'b_1524fc: {
                                        'b_1524f8: {
                                            'b_1524f0: {
                                                c.cycles += 2;
                                                if c.g[S0][0] != c.g[V0][0] {
                                                    'b_1524e0: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            ((c.g[S0][0] as i64) < 5) as u64;
                                                        if c.g[V0][0] != 0 {
                                                            c.cycles += 2;
                                                            let zero_rank = c.g[S0][0] == 0;
                                                            c.g[V0][0] =
                                                                Addr(p(c.g[A0][0], 0x4)).lw(c);
                                                            if zero_rank {
                                                                break 'b_1524f0;
                                                            }
                                                            c.cycles += 2;
                                                            c.g[V0][0] =
                                                                (c.g[V0][0] < c.g[S1][0]) as u64;
                                                            break 'b_152500;
                                                        }
                                                        c.g[V0][0] =
                                                            ((c.g[S0][0] as i64) < 9) as u64;
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 6) as u64;
                                                    if t {
                                                        break 'b_1524f8;
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != 0;
                                                    c.g[V0][0] = Addr(p(c.g[A0][0], 0x4)).lw(c);
                                                    if t {
                                                        break 'b_1524fc;
                                                    }
                                                    break 'b_1524f0;
                                                }
                                                c.g[V0][0] = Addr(p(c.g[A0][0], 0x4)).lw(c);
                                                break 'b_1524f0;
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                                            break 'b_152500;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = Addr(p(c.g[A0][0], 0x4)).lw(c);
                                        break 'b_1524fc;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = (c.g[V0][0] < c.g[S1][0]) as u64;
                                    break 'b_152500;
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'b_152510;
                                }
                                break 'b_152508;
                            }
                            c.cycles += 2;
                            Addr(p(c.g[A0][0], 0x4)).sw(c, c.g[S1][0]);
                            c.g[V0][0] = 1;
                            break 'b_152510;
                        }
                    } else {
                        c.lq(16, frame.at(c, 0x30));
                        break 'b_152514;
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Same-shard direct callees used above.
mod callee {
    use super::{ee, Callee};

    pub const SHOW_EQUIP_MODE: Callee = Callee {
        addr: 0x0019_a238,
        f: ee::s03::cFEStateCharEquip_ShowEquipMode,
    };
    pub const SHOW_BUY_MODE: Callee = Callee {
        addr: 0x0019_a308,
        f: ee::s03::cFEStateCharEquip_ShowBuyMode,
    };
    pub const SET_CATEGORY_TITLE: Callee = Callee {
        addr: 0x0019_bb_e0,
        f: ee::s03::cFEStateCharEquip_SetCategoryTitle,
    };
    pub const BUILD_ITEM_LIST: Callee = Callee {
        addr: 0x0019_b180,
        f: ee::s03::cFEStateCharEquip_BuildItemList,
    };
    pub const REFRESH_ITEM_LIST: Callee = Callee {
        addr: 0x0019_b618,
        f: ee::s03::cFEStateCharEquip_RefreshItemList,
    };
    pub const SPLINE_GET_PATH_LENGTH: Callee = Callee {
        addr: 0x0034_54e8,
        f: ee::s11::SplineModifier_GetPathLength,
    };
    pub const NET_ACCOUNT_SET_STATE: Callee = Callee {
        addr: 0x001b_70d0,
        f: ee::s03::cFEStateNetAccountMem_SetState,
    };
    pub const PATH_PLAT_FIND_BANK_SLOT: Callee = Callee {
        addr: 0x002a_e230,
        f: ee::s08::PATH_PlatFindBankSlot,
    };
    pub const PATH_PLAT_REMOVE_BANK: Callee = Callee {
        addr: 0x002a_e328,
        f: ee::s08::PATH_PlatRemoveBank,
    };
    pub const MENU_SYS_ITEM_HANDLE_INPUT: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::MenuSysItem_HandleInput,
    };
    pub const MENU_SYS_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };
    pub const MENU_SYS_ITEM_GET_INPUT_CODE: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::MenuSysItem_GetInputCode,
    };
    pub const ISEQ_HEAP_ALLOC: Callee = Callee {
        addr: 0x0027_2308,
        f: ee::s07::InSeq_HeapAlloc,
    };
    pub const ISEQ_SPAN_TRACK_ACTIVATE_KEY: Callee = Callee {
        addr: 0x0027_30c0,
        f: ee::s07::InSeq_SpanTrackActivateKey,
    };
    pub const ISEQ_SPAN_TRACK_NEXT_KEY: Callee = Callee {
        addr: 0x0027_3140,
        f: ee::s07::InSeq_SpanTrackNextKey,
    };
    pub const PS2_GRID_MESH_CTOR: Callee = Callee {
        addr: 0x0038_5138,
        f: ee::s12::cPS2GridMesh_ctor,
    };
    pub const KEYBOARD_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };
    pub const KEYBOARD_FIND_ENABLED_FROM_TOP: Callee = Callee {
        addr: 0x001c_c090,
        f: ee::s04::cKeyboardPopup_FindEnabledInColumnFromTop,
    };
    pub const FEMODULE_GET_RIDER_PREVIEW: Callee = Callee {
        addr: 0x001a_0548,
        f: ee::s03::FEModule_GetRiderPreview,
    };
    pub const FEMODULE_RIDER_PREVIEW_SET_VISIBLE: Callee = Callee {
        addr: 0x0019_e538,
        f: ee::s03::FEModule_RiderPreviewSetVisible,
    };
    pub const INPUTSYNC_IS_READY: Callee = Callee {
        addr: 0x0023_7950,
        f: ee::s06::InputSync_IsReady,
    };
    pub const BOLT_GET_NEXT_NODE: Callee = Callee {
        addr: 0x0019_c8f8,
        f: ee::s03::BoltFileIndexManager_GetNextNode,
    };
    pub const BOLT_GET_PREV_NODE: Callee = Callee {
        addr: 0x0019_c8d0,
        f: ee::s03::BoltFileIndexManager_GetPrevNode,
    };
    pub const BOLT_NODE_INDEX: Callee = Callee {
        addr: 0x0019_c920,
        f: ee::s03::BoltFileIndexManager_NodeIndex,
    };
    pub const RIDER_BASE_RESET_FX: Callee = Callee {
        addr: 0x0011_1890,
        f: ee::s00::RiderBase_ResetFX,
    };
    pub const LOAD_INDICATOR_ADVANCE: Callee = Callee {
        addr: 0x0023_1f80,
        f: ee::s06::LoadIndicator_Advance,
    };
    pub const START_SCREEN_CTOR: Callee = Callee {
        addr: 0x0023_2ae0,
        f: ee::s06::StartScreen_ctor,
    };
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::Rng;
    use crate::difftest::{check, DATA, GAME_GP, RA as SENTINEL};
    use ssx3_ee as ee;

    const W: u32 = DATA + 0x5_0000;
    const RM: u32 = DATA + 0x5_1000;
    const BE: u32 = DATA + 0x5_2000;
    const MEM_HEAP: u32 = 0x004f_f250;

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// A dummy world (`gp-0x848`), its rider manager, and the BELibrary
    /// singleton (`gp-0x1f04`).
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(gp_at(-0x1f04), BE);
        c.bus.write32(gp_at(-0x848), W);
        c.bus.write32(W + 0x7c, RM);
        c.bus.write32(W + 0x84, RM);
        c.bus.write32(RM + 0x84, RM);
        c.bus.write32(RM + 0x10, 0);
        r
    }

    /// [`base`] plus the size-class pool allocator and a dlmalloc arena.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base(c, seed, salt);
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let block_base = nodes;
            for n in 0..16u32 {
                let node = block_base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, block_base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        r
    }

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn spline_modifier_load_path_matches() {
        check(
            ee::s11::f_00345570__generated,
            spline_modifier_load_path,
            100,
            &|c, seed| {
                let _r = base(c, seed, 0x3455);
                c.g[A0][0] = DATA as u64;
                // The stream's `v02(self, 8)`.
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 0x2000, DATA + 0x2100);
                c.bus.write16(DATA + 0x2110, 0);
                c.bus.write32(DATA + 0x2114, FAKE);
                // `World_GetInstance()` and the node table it returns.
                c.bus.write32(gp_at(0x16c8), DATA + 0x3000);
                c.bus.write32(DATA + 0x3008, DATA + 0x3100);
                let idx = seed % 4;
                c.bus.write32(DATA, idx | (seed % 3) << 8);
                c.bus.write32(DATA + 4, seed % 3);
                c.bus.write32(
                    DATA + 0x3100 + 4 * idx,
                    if seed % 2 == 0 { 0 } else { DATA + 0x3200 },
                );
                c.bus.write32(DATA + 0x3200 + 0x44, 0);
                c.bus.write32(DATA + 0x3200 + 0x24, DATA + 0x3300);
                c.bus.write32(DATA + 0x3300 + 0x68, DATA + 0x3400);
                c.bus.write32(DATA + 0x3300 + 0x64, DATA + 0x3300);
                c.bus.write32(DATA + 0x3400 + 0xc, f(1.5));
                c.bus.write32(DATA + 0x3400 + 0x84, f(2.5));
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_get_buddy_status_matches() {
        check(
            ee::s03::f_001a4670__generated,
            cfe_state_net_messenger_get_buddy_status,
            80,
            &|c, seed| {
                let _r = base(c, seed, 0x1a46);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(gp_at(0x238), DATA + 0x1000);
                c.bus.write32(gp_at(-0xc8), DATA + 0x1100);
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_update_state06_matches() {
        check(
            ee::s03::f_001b7820__generated,
            cfe_state_net_account_mem_update_state06,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x1b78);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(0x2e0), DATA + 0x1000);
                // Neither 4 nor 5: only the mode test runs.
                c.bus.write32(DATA + 0x1000, 6 + (seed % 3));
                c.bus.write32(DATA + 0x6d4, seed % 7);
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_update_state0c_matches() {
        check(
            ee::s03::f_001b7938__generated,
            cfe_state_net_account_mem_update_state0c,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x1b79);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(0x2e0), DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 6 + (seed % 3));
                c.bus.write32(DATA + 0x6d4, seed % 7);
            },
        );
    }

            #[test]
    fn ssx_audio_system_dj_say20cc_2_matches() {
        check(
            ee::s08::f_002a2568__generated,
            ssx_audio_system_dj_say20cc_2,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2a25);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                // Channel 2's volume: off for every other seed.
                c.bus
                    .write32(DATA + 0x62bc + 0x20, if seed % 2 == 0 { 0 } else { f(0.5) });
            },
        );
    }

    #[test]
    fn path_plat_remove_oldest_bank_matches() {
        check(
            ee::s08::f_002ae3a8__generated,
            path_plat_remove_oldest_bank,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2ae3);
                c.g[A0][0] = DATA as u64;
                // No slot matches: the scan leaves the "none" sentinel.
                c.bus.write32(0x004d_3ef8 + 8, DATA + 0x1000);
            },
        );
    }

        #[test]
    fn menu_sys_bool_item_handle_input_matches() {
        check(
            ee::s09::f_002cc578__generated,
            menu_sys_bool_item_handle_input,
            80,
            &|c, seed| {
                let _r = base(c, seed, 0x2cc5);
                c.g[A0][0] = DATA as u64;
                // Owner -> menu -> input code.
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x124, DATA + 0x1100);
                let code = [2u32, 3, 6, 5][(seed % 4) as usize];
                c.bus.write32(DATA + 0x1100 + 0x48, code);
                c.bus.write32(DATA + 0x1000 + 0x12c, DATA + 0x1300);
                c.bus.write16(DATA + 0x1300 + 0x20, 0);
                c.bus.write32(DATA + 0x1300 + 0x24, FAKE);
                // Item flags and the bool's address.
                c.bus.write32(DATA + 4, if seed % 8 == 0 { 8 } else { 0 });
                c.bus
                    .write32(DATA + 0x18, if seed % 2 == 0 { 1 } else { 0 });
                c.bus
                    .write32(DATA + 0x1c, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(DATA + 0x20, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200, seed & 1);
            },
        );
    }

    #[test]
    fn csetup_man_host_recv_msg9_matches() {
        check(
            ee::s07::f_00269538__generated,
            csetup_man_host_recv_msg9,
            80,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2695);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                c.g[A2][0] = (seed as u64) << 32 | 0x77;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn inseq_span_track_start_matches() {
        check(
            ee::s07::f_00272df0__generated,
            inseq_span_track_start,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x272d);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (DATA + 0x3000) as u64;
                // An empty key list: the short path (the full path allocates
                // and walks a heap that cannot be bounded on test data).
                c.bus.write16(DATA + 0x1000, 0);
                c.bus.write32(DATA + 0x3000, seed);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_create_grid_mesh_matches() {
        check(
            ee::s12::f_0037a540__generated,
            ps2_graphics_man_create_grid_mesh,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x37a5);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                // No 0x40000000 flag: the constructor alone runs.
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn ckeyboard_popup_move_down_from_row4_matches() {
        check(
            ee::s04::f_001cbfc0__generated,
            ckeyboard_popup_move_down_from_row4,
            80,
            &|c, seed| {
                let _r = base(c, seed, 0x1cbf);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let row = [0x39u32, 0x3a, 0x42, 0x10][(seed % 4) as usize];
                c.bus.write32(DATA + 0x140, row);
                c.bus.write32(DATA + 0x40, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn wscript_process_list_delete_by_section_matches() {
        check(
            ee::s10::f_0030b0e8__generated,
            wscript_process_list_delete_by_section,
            80,
            &|c, seed| {
                let _r = base(c, seed, 0x30b0);
                c.g[A0][0] = DATA as u64;
                let section = (seed % 4) as u64;
                c.g[A1][0] = section;
                let nodes = if seed % 3 == 0 {
                    0
                } else {
                    1 + (seed % 2) as u32
                };
                c.bus
                    .write32(DATA, if nodes == 0 { 0 } else { DATA + 0x1000 });
                for i in 0..nodes {
                    let node = DATA + 0x1000 + 0x100 * i;
                    c.bus.write32(node + 0x24, section as u32);
                    c.bus
                        .write32(node + 0x18, if i + 1 == nodes { 0 } else { node + 0x100 });
                    c.bus.write32(node + 0x5c, DATA + 0x5000);
                }
                c.bus.write16(DATA + 0x5000 + 0x8, 0);
                c.bus.write32(DATA + 0x5000 + 0xc, FAKE);
            },
        );
    }

        #[test]
    fn cfe_state_reward_gallery_on_activate_matches() {
        check(
            ee::s04::f_001cf168__generated,
            cfe_state_reward_gallery_on_activate,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x1cf1);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x48, (seed as u32) | 0x2);
                c.bus
                    .write32(DATA + 0x924, if seed % 4 == 0 { 0 } else { DATA + 0x3000 });
                c.bus
                    .write32(DATA + 0x8f8, if seed % 4 <= 1 { 0 } else { DATA + 0x3100 });
                c.bus
                    .write32(DATA + 0x8fc, if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
                c.bus.write32(DATA + 8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x140, 0);
                c.bus.write32(DATA + 0x2000 + 0x144, FAKE);
                c.bus.write16(DATA + 0x2000 + 0x108, 0);
                c.bus.write32(DATA + 0x2000 + 0x10c, FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_equip_mode_matches() {
        check(
            ee::s03::f_0019a238__generated,
            cfe_state_char_equip_show_equip_mode,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x19a2);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write8(DATA + 0x44, seed as u8);
                for (field, ret) in [(0x60u32, 0x19a288u32), (0x88, 0x19a2a8), (0x80, 0x19a2cc)] {
                    let obj = DATA + field;
                    c.bus
                        .write32(DATA + field, if seed % 3 == 0 { 0 } else { obj });
                    c.bus.write32(obj + 8, obj + 0x100);
                    c.bus.write16(obj + 0x100 + 0x48, 0);
                    c.bus.write32(obj + 0x100 + 0x4c, FAKE + (ret & 0xf));
                }
                c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 0x48, 0);
                c.bus.write32(DATA + 0x2100 + 0x4c, FAKE);
            },
        );
    }

        #[test]
    fn cfe_state_options_device_select_on_widget_notify_matches() {
        check(
            ee::s02::f_0018ddd0__generated,
            cfe_state_options_device_select_on_widget_notify,
            80,
            &|c, seed| {
                let _r = base(c, seed, 0x18dd);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.bus.write32(DATA + 0x1000, seed);
                c.g[A2][0] = [8u64, 9, 6, 0x101][(seed % 4) as usize];
                // A prepared manager so `MCManager_Get` does not construct one.
                c.bus.write32(gp_at(-0x47c), DATA + 0x2000);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_category_has_flagged_item_matches() {
        check(
            ee::s03::f_0019afd0__generated,
            cfe_state_char_equip_category_has_flagged_item,
            40,
            &|c, seed| {
                let _r = base(c, seed, 0x19af);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = 0;
                // Empty category: the collect call returns 0 immediately.
                c.bus.write32(DATA + 0xbc, 0);
                c.bus.write32(DATA + 0x1000 + 0xbc, 0);
            },
        );
    }

        #[test]
    fn rider_reset_motion_matches() {
        check(ee::s00::f_0011df18__generated, rider_reset_motion, 60, &|c, seed| {
            let _r = base(c, seed, 0x11df);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
            for i in 0..8 {
                c.bus.write32(DATA + 0x1b0 + 4 * i, seed + i as u32);
            }
            c.bus.write32(DATA + 0x6c0, 0);
            c.bus.write32(DATA + 0x784, 0);
            c.bus.write32(DATA + 0x77c, 0);
        });
    }

        #[test]
    fn init_load_screen_update_matches() {
        check(
            ee::s06::f_00232588__generated,
            init_load_screen_update,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2325);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 8, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(W + 0xa4, DATA + 0x3000);
                c.bus.write32(DATA + 0x3004, 0x3f00_0000 | (seed & 0x7f));
                // Skip creating the start screen except for a few seeds.
                c.bus.write32(W + 4, if seed % 8 == 0 { 0 } else { 1 });
                c.bus.write32(gp_at(-0x518c), f(0.05));
                c.bus.write32(gp_at(-0x5188), f(0.05));
            },
        );
    }

        #[test]
    fn vram_heap_free_unlocked_matches() {
        check(
            ee::s12::f_00366548__generated,
            vram_heap_free_unlocked,
            20,
            &|c, seed| {
                let _r = base(c, seed, 0x3665);
                c.g[A0][0] = DATA as u64;
                // Start index -1: nothing to free.
                c.bus.write32(DATA + 0x23c0, (-1i32) as u32);
                c.bus.write32(DATA + 0x1ff0, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg8_matches() {
        check(
            ee::s07::f_00269948__generated,
            csetup_man_client_recv_msg8,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2699);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                c.g[A2][0] = (seed as u64) << 32 | 0x99;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

        #[test]
    fn input_parser_compile_identifier_matches() {
        check(
            ee::s10::f_003245a8__generated,
            input_parser_compile_identifier,
            40,
            &|c, seed| {
                let _r = base(c, seed, 0x3245);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (seed % 8) as u64;
                c.g[T0][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn input_preproc_write_matches() {
        check(ee::s10::f_00326360__generated, input_preproc_write, 60, &|c, seed| {
            let _r = alloc_base(c, seed, 0x3263);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = seed as u64;
            let cap = if seed % 4 == 0 { 8 } else { 0x100 };
            let pos = if seed % 4 == 0 {
                cap
            } else {
                (seed % 8) as u32
            };
            c.bus.write32(DATA, pos);
            c.bus.write32(DATA + 4, cap);
            c.bus
                .write32(DATA + 8, if seed % 8 == 0 { 0 } else { DATA + 0x2000 });
            for i in 0..0x20 {
                c.bus.write8(DATA + 0x2000 + i, 0);
            }
        });
    }

    #[test]
    fn cworldcache_streamer_reset_matches() {
        check(
            ee::s13::f_003a9188__generated,
            cworldcache_streamer_reset,
            20,
            &|c, seed| {
                let _r = base(c, seed, 0x3a91);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn crowd_man3d_group_set_mesh_frame_matches() {
        check(
            ee::s11::f_00344730__generated,
            crowd_man3d_group_set_mesh_frame,
            40,
            &|c, seed| {
                let _r = base(c, seed, 0x3447);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                c.bus.write32(DATA + 4, 0);
                c.bus.write32(DATA + 0x110, 0);
                c.bus.write32(gp_at(0x16c8), DATA + 0x3000);
                c.bus.write32(DATA + 0x3008, DATA + 0x3100);
                c.bus.write32(DATA + 0x3100, 0);
            },
        );
    }

    #[test]
    fn replay_end_playback_matches() {
        check(ee::s07::f_0026f980__generated, replay_end_playback, 40, &|c, seed| {
            let _r = base(c, seed, 0x26f9);
            c.g[A0][0] = DATA as u64;
            // 0xE/0xF return immediately; other states run the teardown.
            c.bus.write32(DATA, if seed % 2 == 0 { 0xe } else { 0x3 });
            c.bus.write32(DATA + 0x608, (seed % 2) as u32);
            c.bus.write32(DATA + 0x60c, (seed % 3) as u32);
            c.bus.write32(DATA + 0x61c, 0);
            c.bus.write32(DATA + 0x620, 0);
        });
    }

        #[test]
    fn game_module_on_section_unload_matches() {
        check(
            ee::s06::f_00230360__generated,
            game_module_on_section_unload,
            30,
            &|c, seed| {
                let _r = base(c, seed, 0x2303);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 16) as u64;
                c.bus.write32(gp_at(-0x6f0), DATA + 0x1000);
                c.bus.write32(gp_at(0xf00), DATA + 0x1100);
                c.bus.write32(gp_at(0xf38), DATA + 0x1200);
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(DATA + 0x30, 0);
            },
        );
    }

                #[test]
    fn cfe_load_screen_unload_matches() {
        check(
            ee::s06::f_00233930__generated,
            cfe_load_screen_unload,
            40,
            &|c, seed| {
                let _r = base(c, seed, 0x2339);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18, (seed % 4) as u32);
                c.bus
                    .write32(DATA + 0xc, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x14, seed as u32);
            },
        );
    }

        #[test]
    fn ui_menu_find_first_selectable_matches() {
        check(
            ee::s13::f_0039b7b0__generated,
            ui_menu_find_first_selectable,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x39b7);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 6) as u64;
                c.bus.write8(DATA + 0x96, (seed % 5) as u8);
                c.bus.write32(DATA + 0x74, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x90, if seed % 3 == 0 { 4 } else { 0 });
                c.bus.write32(DATA + 0x5c, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 8, DATA + 0x2200);
                c.bus.write16(DATA + 0x2200 + 0xa0, 0);
                c.bus.write32(DATA + 0x2200 + 0xa4, FAKE);
                c.bus.write32(DATA + 0x1000 + 0x14, (seed % 2) as u32);
            },
        );
    }

        #[test]
    fn cov_state_hud1p_load_meter_assets_matches() {
        check(
            ee::s05::f_0020eb50__generated,
            cov_state_hud1p_load_meter_assets,
            20,
            &|c, seed| {
                let _r = base(c, seed, 0x20eb);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus.write32(gp_at(-0x988), 0);
                c.bus.write32(gp_at(-0x98c), 0);
                c.bus.write32(DATA + 0x3000 + 8, seed as u32);
            },
        );
    }

        #[test]
    fn csetup_man_recv_msg9_matches() {
        check(
            ee::s07::f_00269a18__generated,
            csetup_man_client_recv_msg9,
            100,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x269a);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                c.g[A2][0] = (seed as u64) << 32 | 0x55;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn detail_system_menu_dtor_matches() {
        check(
            ee::s06::f_0024fab8__generated,
            detail_system_menu_dtor,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x24fa);
                c.g[A0][0] = DATA as u64;
                // Bit 0 of the flags would free the (test) object.
                c.g[A1][0] = 2;
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_register_textures_matches() {
        check(
            ee::s04::f_001cfff8__generated,
            cfe_state_reward_gallery_register_textures,
            120,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cff);
                let count = seed % 3;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 11 == 0 {
                    0
                } else {
                    (DATA + 0x2000) as u64
                };
                c.bus.write32(DATA + 0x2008, count);
                for i in 0..3 {
                    c.bus.write32(DATA + 0x2014 + 8 * i, r.next_u32() & 0x7ff);
                }
                // `cGame` and its vtable slot 46.
                let game = DATA + 0x3000;
                let vt = DATA + 0x3100;
                c.bus.write32(gp_at(-0x854), game);
                c.bus.write32(game + 0x10d8, vt);
                c.bus.write16(vt + 0x170, 0);
                c.bus.write32(vt + 0x174, FAKE);
                for i in 0..3 {
                    // entry structs at self+0x7c, 0xc bytes each
                    c.bus.write32(DATA + 0x7c + 0xc * i + 4, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_refresh_page_matches() {
        check(
            ee::s04::f_001d0b00__generated,
            cfe_state_reward_gallery_refresh_page,
            120,
            &|c, seed| {
                let _r = base(c, seed, 0x1d0b);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x58, (seed % 3) as u32);
                let vt = DATA + 0x3000;
                c.bus.write32(DATA + 0x8, vt);
                for (i, (d, f)) in [
                    (0x138u32, 0x13cu32),
                    (0x170, 0x174),
                    (0x168, 0x16c),
                    (0x160, 0x164),
                    (0x148, 0x14c),
                    (0x150, 0x154),
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write16(vt + d, 0);
                    c.bus.write32(vt + f, FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    }
