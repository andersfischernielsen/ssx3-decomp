//! `src/main/debugmenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::guest;
use crate::abi::r32;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::a;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_d710, render_toggles_menu_dtor, "RenderTogglesMenu_dtor"),
    (0x0024_d998, visual_effects_test_menu_dtor, "VisualEffectsTestMenu_dtor"),
    (0x0024_dc68, depth_fog_menu_dtor, "DepthFogMenu_dtor"),
    (0x0024_e200, snowfall_toggles_menu_dtor, "SnowfallTogglesMenu_dtor"),
    (0x0024_f6c0, visual_effects_main_menu_dtor, "VisualEffectsMainMenu_dtor"),
    (0x0024_f860, plants_menu_dtor, "PlantsMenu_dtor"),
    (0x0024_fa00, height_fog_menu_dtor, "HeightFogMenu_dtor"),
    (0x0024_fab8, detail_system_menu_dtor, "DetailSystemMenu_dtor"),
    (0x0024_fbc0, shell_renderer_menu_dtor, "ShellRendererMenu_dtor"),
    (0x0024_fd00, shadow_menu_dtor, "ShadowMenu_dtor"),
    (0x0024_fe00, snow_surface_menu_dtor, "SnowSurfaceMenu_dtor"),
    (0x0024_ff28, flags_menu_dtor, "FlagsMenu_dtor"),
    (0x0025_0090, tube_menu_dtor, "TubeMenu_dtor"),
    (0x0025_09d8, aivisual_effects_main_menu_ctor, "AIVisualEffectsMainMenu_ctor"),
    (0x0025_0a60, aivisual_effects_main_menu_on_activate, "AIVisualEffectsMainMenu_OnActivate"),
    (0x0025_0a80, aivisual_effects_main_menu_on_deactivate, "AIVisualEffectsMainMenu_OnDeactivate"),
    (0x0025_15f0, avalanche_menu_on_activate, "AvalancheMenu_OnActivate"),
    (0x0025_1610, avalanche_menu_on_deactivate, "AvalancheMenu_OnDeactivate"),
    (0x0025_1630, aivisual_effects_main_menu_dtor, "AIVisualEffectsMainMenu_dtor"),
    (0x0025_1690, aivisual_effects_menus_global_ctors, "AIVisualEffectsMenus_GlobalCtors"),
    (0x0025_16b0, aivisual_effects_menus_global_dtors, "AIVisualEffectsMenus_GlobalDtors"),
    (0x0025_16d0, mem_init_block_header, "MEM_InitBlockHeader"),
];

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
}


// ---------------------------------------------------------------------------
// Memory heap.

/// `void MEM_InitBlockHeader(MEMBlock *block, uint16 magic, uint32 prev,
/// uint32 next, uint32 size)` (0x2516d0): fill the block header at `block`.
pub fn mem_init_block_header(c: &mut Cpu) {
    c.cycles += 8;
    let block = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x424d;
    block.off(0xc).sw(c, c.g[T1][0]);
    block.sh(c, c.g[V0][0]);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
    block.off(0x2).sh(c, c.g[T0][0]);
    block.off(0x8).sw(c, c.g[T2][0]);
    block.off(0x4).sw(c, c.g[A2][0]);
    abi::ret(c);
}

pub(crate) const AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY: Callee =
    Callee { addr: 0x0025_0cb0, f: ee::s06::f_00250cb0 };


/// `AIVisualEffectsMainMenu *AIVisualEffectsMainMenu_ctor(AIVisualEffectsMainMenu *this)`
/// (0x2509d8): construct the page, a label item, a spacer item and a submenu
/// item and insert the submenu into the page.
pub fn aivisual_effects_main_menu_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_aa58, 0x0025_09f0)?; // MenuSysPage_ctor(this)
        let s0 = c.g[S0][0];
        c.g[S1][0] = sx((s0 as u32).wrapping_add(0x164));
        c.g[V1][0] = 0x48_0000;
        c.g[A1][0] = 0x48_0000;
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x18));
        c.g[A0][0] = sx((s0 as u32).wrapping_add(0x130));
        c.sw((s0 as u32).wrapping_add(0x12c), c.g[V1][0]);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x50));
        call_ptr(c, 0x002c_d2a0, 0x0025_0a10)?; // MenuSysLabelItem_ctor
        let s0 = c.g[S0][0];
        c.g[A0][0] = sx((s0 as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0025_0a1c)?; // MenuSysSpacerItem_ctor
        c.g[A1][0] = 0x4d_0000;
        c.g[A2][0] = 0x48_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x4d_0000u32.wrapping_add(0x1c88));
        c.g[A2][0] = sx(0x48_0000u32.wrapping_sub(0x30));
        call_ptr(c, 0x002c_ccc8, 0x0025_0a34)?; // MenuSysSubmenuItem_ctor
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x0025_0a44)?; // MenuSysPage_InsertItem
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AIVisualEffectsMainMenu_OnActivate(AIVisualEffectsMainMenu *this)`
/// (0x250a60): `MenuSysPage_OnActivate(this)`.
pub fn aivisual_effects_main_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002c_bf60), 0x0025_0a70)
}


/// `void AIVisualEffectsMainMenu_OnDeactivate(AIVisualEffectsMainMenu *this)`
/// (0x250a80): `MenuSysPage_OnDeactivate(this)`.
pub fn aivisual_effects_main_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002c_bfd0), 0x0025_0a90)
}


/// `void AIVisualEffectsMainMenu_dtor(AIVisualEffectsMainMenu *this, uint flags)`
/// (0x251630): destruct the submenu, spacer, label and page, then the base.
pub fn aivisual_effects_main_menu_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_1654)?; // MenuSysItem_dtor
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_1660)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_166c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0025_1678)?; // MenuSysPage_dtor
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AIVisualEffectsMenus_GlobalCtors(void)` (0x251690): construct the
/// static menus (`AIVisualEffectsMenus_StaticInitDestroy(1, 0xffff)`).
pub fn aivisual_effects_menus_global_ctors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY.call(c, 0x0025_16a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AIVisualEffectsMenus_GlobalDtors(void)` (0x2516b0): destroy the
/// static menus (`AIVisualEffectsMenus_StaticInitDestroy(0, 0xffff)`).
pub fn aivisual_effects_menus_global_dtors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY.call(c, 0x0025_16c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Menu hooks and thunks.

/// `void AvalancheMenu_OnActivate(AvalancheMenu *self)` (0x2515f0):
/// `MenuSysPage_OnActivate(self)`.
pub fn avalanche_menu_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bf60, 0x0025_1600)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AvalancheMenu_OnDeactivate(AvalancheMenu *self)` (0x251610):
/// `MenuSysPage_OnDeactivate(self)`.
pub fn avalanche_menu_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bfd0, 0x0025_1620)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `a + off`.
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


// ---------------------------------------------------------------------------
// Destructors.

/// `void FlagsMenu_dtor(FlagsMenu* self, int flags)` (0x24ff28): destroy the
/// four sub-menus (each as a `MenuSysItem`) and then the page base (with
/// `flags`). `flags & 1` asks the page base to free `self`.
pub fn flags_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        // The four sub-menus, each destroyed as its base `MenuSysItem`.
        for (off, ret) in [
            (0x194u32, 0x0024_ff4cu32),
            (0x164, 0x0024_ff58),
            (0x14c, 0x0024_ff64),
            (0x130, 0x0024_ff70),
        ] {
            c.g[A0][0] = sx(at(this, off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_ff7c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}


/// `HeightFogMenu_dtor` (0x24FA00): destroy the height fog menu's five
/// sub-menus and then the menu base.
pub fn height_fog_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x1c8u32, 0x0024_fa24u32),
            (0x194, 0x0024_fa30),
            (0x164, 0x0024_fa3c),
            (0x14c, 0x0024_fa48),
            (0x130, 0x0024_fa54),
        ] {
            c.g[A0][0] = low(c, S0).wrapping_add(off as u64);
            c.g[A1][0] = 2;
            call_ptr(c, addr::MENU_DTOR_2, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, addr::MENU_DTOR, 0x0024_fa60)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `TubeMenu_dtor` (0x250090): destroy the menu's eight child objects and
/// then the base menu.
pub fn tube_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x25c, 0x0025_00b4),
            (0x22c, 0x0025_00c0),
            (0x1fc, 0x0025_00cc),
            (0x1c8, 0x0025_00d8),
            (0x194, 0x0025_00e4),
            (0x164, 0x0025_00f0),
            (0x14c, 0x0025_00fc),
            (0x130, 0x0025_0108),
        ] {
            c.g[A0][0] = sx(this.wrapping_add(off));
            c.g[RA][0] = sx(ret);
            c.g[A1][0] = 2;
            call_ptr(c, callee::TUBE_MENU_ITEM_DTOR, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0025_0114);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::TUBE_MENU_BASE_DTOR, 0x0025_0114)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void SnowSurfaceMenu_dtor(self, flags)` (0x24fe00): destroy the menu's
/// nine sub-objects and the base object.
pub fn snow_surface_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret) in [
            (0x294u32, 0x0024_fe24u32),
            (0x260, 0x0024_fe30),
            (0x22c, 0x0024_fe3c),
            (0x1f8, 0x0024_fe48),
            (0x1c4, 0x0024_fe54),
            (0x194, 0x0024_fe60),
            (0x164, 0x0024_fe6c),
            (0x14c, 0x0024_fe78),
            (0x130, 0x0024_fe84),
        ] {
            c.g[A0][0] = sx(p(r32(c, S0), off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024fe90)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
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


// ---------------------------------------------------------------------------
// PlantsMenu_dtor (0x000024f860).

/// `PlantsMenu_dtor`: destroy the plants menu: free its child objects and the base screen.
pub fn plants_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 56;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f884)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f890)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f89c)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8a8)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8b4)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8c0)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8cc)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8d8)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8e4)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8f0)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8fc)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f908)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f914)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f920)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x002caa80, 0x0024f92c)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}


/// `DepthFogMenu_dtor(self, flags)` (0x24dc68): destroy the depth-fog menu's
/// items and pages, then the menu base.
pub fn depth_fog_menu_dtor(c: &mut Cpu) {
    const ITEM_DTOR: u32 = 0x002c_a280;
    const PAGE_DTOR: u32 = 0x002c_aa80;
    const MENU_DTOR: u32 = 0x002c_aa80;

    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this.wrapping_add(0x2c4));
        const ITEMS: [u32; 16] = [
            0x904, 0x8e0, 0x8bc, 0x898, 0x874, 0x2dc, 0x2c4, 0x290, 0x25c, 0x22c, 0x1f8, 0x1c4,
            0x194, 0x164, 0x14c, 0x130,
        ];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(*off));
            c.g[A1][0] = 2;
            call_ptr(c, if *off == 0x2dc { PAGE_DTOR } else { ITEM_DTOR }, 0x0024_dc94 + 0xc * i as u32)?;
        }
        // The first entry is a page (MenuSysPage_dtor); the base dtor follows.
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, MENU_DTOR, 0x0024_dd54)?;
        c.lq(16, frame.at(c, 0x30));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `PowerUpFXTogglesMenu_dtor(self, flags)` (0x24e0f8): destroy the
/// power-up FX toggles menu's items, then the menu base.
pub fn power_up_fx_toggles_menu_dtor(c: &mut Cpu) {
    const ITEM_DTOR: u32 = 0x002c_a280;
    const MENU_DTOR: u32 = 0x002c_aa80;

    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        const ITEMS: [u32; 16] = [
            0x418, 0x3e8, 0x3b8, 0x388, 0x358, 0x328, 0x2f8, 0x2c8, 0x294, 0x260, 0x22c, 0x1f8,
            0x1c4, 0x194, 0x164, 0x14c,
        ];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(*off));
            c.g[A1][0] = 2;
            call_ptr(c, ITEM_DTOR, 0x0024_e11c + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, MENU_DTOR, 0x0024_e1e8)?;
        c.lq(16, frame.at(c, 0x20));
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MPEG2_Get_Bits(int n)` — read `n` bits from the MPEG-2 bit reader.
pub(crate) const MPEG2_GET_BITS: u32 = 0x003b_1300;

/// `void MenuSysItem_dtor(MenuSysItem*, int flags)` and the page destructor.
pub(crate) const MENU_SYS_ITEM_DTOR: u32 = 0x002c_a280;

pub(crate) const MENU_SYS_PAGE_DTOR: u32 = 0x002c_aa80;


/// The MPEG-2 decoder's bit reader state pointer (`0x510000 - 0x5f78`).
pub(crate) const MPEG2_BIT_STATE: u32 = 0x0050_a088;


// ---------------------------------------------------------------------------
// MPEG-2 picture coding extension.

/// `void MPEG2_picture_coding_extension(void)` (0x3b20b8): read the 21 fields
/// of the picture coding extension into the decoder state. The bit widths and
/// return addresses are the original's call schedule.
pub(crate) const PCE_FIRST: [(u32, u32); 16] = [
    (4, 0x003b_20d4),
    (4, 0x003b_20e8),
    (4, 0x003b_20f4),
    (4, 0x003b_2100),
    (2, 0x003b_210c),
    (2, 0x003b_211c),
    (1, 0x003b_212c),
    (1, 0x003b_213c),
    (1, 0x003b_214c),
    (1, 0x003b_215c),
    (1, 0x003b_216c),
    (1, 0x003b_217c),
    (1, 0x003b_218c),
    (1, 0x003b_219c),
    (1, 0x003b_21ac),
    (1, 0x003b_21bc),
];


pub(crate) const PCE_LAST: [(u32, u32); 5] = [
    (1, 0x003b_21d0),
    (3, 0x003b_21e0),
    (1, 0x003b_21f0),
    (7, 0x003b_2200),
    (8, 0x003b_2210),
];


pub fn mpeg2_picture_coding_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x0051_0000;
        // The state block starts at 0x510000-0x6a90 and grows by four bytes
        // per field; two fields live in the bit reader's sub-object.
        c.g[S0][0] = 0x0051_0000;
        c.g[S0][0] = sx(0x0051_0000u32.wrapping_sub(0x6a90));
        let base = r32(c, S0);

        // Store field `i` where the original does: fields 10 and 12 live in
        // the bit reader's sub-object, the rest walk the state block.
        fn store(c: &mut Cpu, base: u32, i: usize, v: u64) {
            let dst = match i {
                9 => (Addr(MPEG2_BIT_STATE).lw(c) as u32).wrapping_add(0x428),
                11 => (Addr(MPEG2_BIT_STATE).lw(c) as u32).wrapping_add(0x42c),
                _ => base.wrapping_add(4 * (i - usize::from(i > 9) - usize::from(i > 11)) as u32),
            };
            Addr(dst).sw(c, v);
        }

        for (i, &(width, ret)) in PCE_FIRST.iter().enumerate() {
            match i {
                4..=9 | 11 | 13..=15 => c.g[V1][0] = 0x0051_0000,
                10 | 12 => c.g[V1][0] = Addr(MPEG2_BIT_STATE).lw(c),
                _ => {}
            }
            c.g[A0][0] = width as u64;
            call_ptr(c, MPEG2_GET_BITS, ret)?;
            let v = c.g[V0][0];
            store(c, base, i, v);
        }

        // The sixteenth field gates the rest of the extension.
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 20;
        for (i, &(width, ret)) in PCE_LAST.iter().enumerate() {
            c.g[V1][0] = 0x0051_0000;
            c.g[A0][0] = width as u64;
            call_ptr(c, MPEG2_GET_BITS, ret)?;
            Addr(base.wrapping_add(0x38 + 4 * i as u32)).sw(c, c.g[V0][0]);
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Options-menu page destructors.
//
// Every one of these destroys a fixed set of `MenuSysItem`s (with the
// non-deleting flag value 2) and then the page itself.

/// Destroy the embedded item at `this + off` (`MenuSysItem_dtor(x, 2)`).
pub(crate) fn item_dtor(c: &mut Cpu, this: u32, off: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(this.wrapping_add(off));
    c.g[A1][0] = 2;
    call_ptr(c, MENU_SYS_ITEM_DTOR, ret)
}


/// The shared body: `offsets[i]` is destroyed at `first_ret + 0xc*i`, then
/// `MenuSysPage_dtor(this, a1)` at the next return address.
pub(crate) fn menu_page_dtor(c: &mut Cpu, offsets: &[u32], first_ret: u32) {
    guest(c, |c| {
        c.cycles += 71;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);

        let this = r32(c, S0);
        for (i, &off) in offsets.iter().enumerate() {
            item_dtor(c, this, off, first_ret + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(
            c,
            MENU_SYS_PAGE_DTOR,
            first_ret + 0xc * offsets.len() as u32,
        )?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `VisualEffectsTestMenu_dtor` (0x24d998).
pub fn visual_effects_test_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x464, 0x434, 0x404, 0x3d4, 0x3a4, 0x374, 0x344, 0x314, 0x2e4, 0x2b4, 0x284, 0x254,
            0x224, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_d9bc,
    );
}


/// `VisualEffectsMainMenu_dtor` (0x24f6c0).
pub fn visual_effects_main_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x324, 0x308, 0x2ec, 0x2d0, 0x2b4, 0x298, 0x27c, 0x260, 0x244, 0x228, 0x20c, 0x1f0,
            0x1d4, 0x1b8, 0x19c, 0x180, 0x164, 0x14c, 0x130,
        ],
        0x0024_f6e4,
    );
}


/// `BoardTrailMenu_dtor` (0x24dfa0).
pub fn board_trail_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x59c, 0x56c, 0x53c, 0x508, 0x4d4, 0x4a0, 0x46c, 0x438, 0x404, 0x3d0, 0x39c, 0x368,
            0x334, 0x300, 0x2cc, 0x298, 0x264, 0x230, 0x1fc, 0x1c8, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dfc4,
    );
}


/// `LightGlowMenu_dtor` (0x24dab8).
pub fn light_glow_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x700, 0x6cc, 0x698, 0x664, 0x630, 0x600, 0x5cc, 0x598, 0x564, 0x530, 0x4fc, 0x4c8,
            0x494, 0x460, 0x42c, 0x3fc, 0x3c8, 0x394, 0x360, 0x32c, 0x2f8, 0x2c4, 0x290, 0x25c,
            0x228, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dadc,
    );
}


/// `AvalancheMenu_dtor` (0x250ae0).
pub fn avalanche_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x77c, 0x748, 0x714, 0x6e0, 0x6b0, 0x67c, 0x648, 0x614, 0x5e0, 0x5ac, 0x578, 0x544,
            0x514, 0x4e0, 0x4ac, 0x478, 0x444, 0x410, 0x3dc, 0x3ac, 0x37c, 0x34c, 0x31c, 0x2e8,
            0x2b8, 0x288, 0x258, 0x228, 0x1f8, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0025_0b04,
    );
}


/// `ScreenTintMenu_dtor` (0x24dd70).
pub fn screen_tint_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x928, 0x8f4, 0x8c0, 0x88c, 0x858, 0x824, 0x7f0, 0x7c0, 0x78c, 0x758, 0x724, 0x6f0,
            0x6bc, 0x688, 0x654, 0x620, 0x5ec, 0x5b8, 0x584, 0x550, 0x520, 0x4f0, 0x4c0, 0x490,
            0x45c, 0x42c, 0x3fc, 0x3c8, 0x394, 0x360, 0x32c, 0x2f8, 0x2c4, 0x290, 0x25c, 0x228,
            0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dd94,
    );
}


/// The `SnowfallTogglesMenu_dtor` steps: offset from `this`, and whether the
/// field is an embedded page rather than an item.
pub(crate) const SNOWFALL_STEPS: [(u32, bool); 26] = [
    (0xfec, false),
    (0xfd0, false),
    (0xfac, false),
    (0xf88, false),
    (0xf64, false),
    (0xf40, false),
    (0x9a8, true),
    (0x990, false),
    (0x974, false),
    (0x950, false),
    (0x92c, false),
    (0x908, false),
    (0x8e4, false),
    (0x34c, true),
    (0x334, false),
    (0x300, false),
    (0x2cc, false),
    (0x298, false),
    (0x264, false),
    (0x230, false),
    (0x1fc, false),
    (0x1c8, false),
    (0x194, false),
    (0x164, false),
    (0x14c, false),
    (0x130, false),
];


/// `SnowfallTogglesMenu_dtor` (0x24e200): 24 item destructors interleaved with
/// two embedded pages at `+0x9a8` and `+0x34c`, then the page itself.
pub fn snowfall_toggles_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S3][0] = abi::arg64(c, 1);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);

        let this = r32(c, S0);
        c.g[S1][0] = sx(this.wrapping_add(0x990));
        c.g[S2][0] = sx(this.wrapping_add(0x334));

        for (i, &(off, page)) in SNOWFALL_STEPS.iter().enumerate() {
            let ret = if i < 3 {
                0x0024_e22c + 0x10 * i as u32
            } else {
                0x0024_e258 + 0xc * (i as u32 - 3)
            };
            c.g[A0][0] = sx(this.wrapping_add(off));
            c.g[A1][0] = 2;
            let target = if page {
                MENU_SYS_PAGE_DTOR
            } else {
                MENU_SYS_ITEM_DTOR
            };
            call_ptr(c, target, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, MENU_SYS_PAGE_DTOR, 0x0024_e36c)?;

        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `RenderTogglesMenu_dtor` (0x0024d710): destroy the debug render-toggles
/// menu: run the destructor (flags 2) of its 49 `MenuSysItem` sub-objects
/// -- laid out from `+0xa18` down to `+0x130`, 0x30 bytes apart with a few
/// 0x34 gaps -- and then of the menu object itself.
pub fn render_toggles_menu_dtor(c: &mut Cpu) {
    /// The 49 item sub-objects, in destruction order.
    const ITEMS: [u32; 49] = [
        0xa18, 0x9e8, 0x9b8, 0x988, 0x958, 0x924, 0x8f0, 0x8bc, 0x888, 0x858, 0x828, 0x7f8, 0x7c4,
        0x794, 0x764, 0x734, 0x704, 0x6d4, 0x6a4, 0x674, 0x644, 0x614, 0x5e4, 0x5b4, 0x584, 0x554,
        0x524, 0x4f4, 0x4c4, 0x494, 0x464, 0x434, 0x404, 0x3d4, 0x3a4, 0x374, 0x344, 0x314, 0x2e4,
        0x2b4, 0x284, 0x254, 0x224, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
    ];
    guest(c, |c| {
        c.cycles += 161;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let menu = c.g[S0][0];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx((menu as u32).wrapping_add(*off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, 0x0024_d734 + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_d980)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check_lohi;
    use crate::difftest::fill;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::SP;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_5;
    pub(crate) use crate::game::world::worldcache::tests::check_all;


    #[test]
    fn mem_init_block_header_matches() {
        check(ssx3_ee::s06::f_002516d0, mem_init_block_header, 100, &|c, seed| {
            let mut r = Rng(0x2516_d000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[T2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for off in [0u32, 2, 4, 8, 0xc] {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }


    #[test]
    fn aivisual_effects_main_menu_ctor_matches() {
        check(ee::s06::f_002509d8, aivisual_effects_main_menu_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x09d8, 0);
            c.g[A0][0] = DATA as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x200 / 4);
        });
    }


    #[test]
    fn aivisual_effects_main_menu_dtor_matches() {
        check(ee::s06::f_00251630, aivisual_effects_main_menu_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x1630, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x200 / 4);
        });
    }


    #[test]
    fn aivisual_effects_main_menu_activate_match() {
        for (gen, hand, salt) in [
            (ee::s06::f_00250a60 as fn(&mut Cpu), aivisual_effects_main_menu_on_activate as fn(&mut Cpu), 0x0a60u64),
            (ee::s06::f_00250a80, aivisual_effects_main_menu_on_deactivate, 0x0a80),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = DATA as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x100 / 4);
            });
        }
    }


    #[test]
    fn aivisual_effects_menus_globals_match() {
        for (gen, hand, salt) in [
            (ee::s06::f_00251690 as fn(&mut Cpu), aivisual_effects_menus_global_ctors as fn(&mut Cpu), 0x1690u64),
            (ee::s06::f_002516b0, aivisual_effects_menus_global_dtors, 0x16b0),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }


    #[test]
    fn avalanche_menu_hooks_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s06::f_002515f0 as fn(&mut Cpu), avalanche_menu_on_activate as fn(&mut Cpu), 0x515),
            (ssx3_ee::s06::f_00251610, avalanche_menu_on_deactivate, 0x610),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                c.g[A0][0] = DATA as u64;
                let _ = &mut r;
            });
        }
    }


    // -----------------------------------------------------------------------

    #[test]
    fn flags_menu_dtor_matches() {
        check(ee::s06::f_0024ff28__generated, flags_menu_dtor, 40, &|c, seed| {
            base(c, seed, 0x24_ff28);
            c.g[A0][0] = sx(DATA);
            // Even flags: the base destructor must not free the test object
            // (the sub-objects are test data, not heap blocks).
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            for off in [0x130u32, 0x14c, 0x164, 0x194] {
                c.bus.write32(DATA + off + 0x10, 0x1111);
                c.bus.write32(DATA + off + 0x12c, 0x2222);
            }
        });
    }


    #[test]
    fn height_fog_menu_dtor_matches() {
        check(ee::s06::f_0024fa00__generated, height_fog_menu_dtor, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x24_fa00, 4);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.pc = 0x0024_fa00;
        });
    }


        #[test]
    fn tube_menu_dtor_matches() {
        check(
            ee::s06::f_00250090__generated,
            tube_menu_dtor,
            SEEDS_3,
            &alloc_base_setup(|c, seed, _r| {
                let this = alloc(c, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
            }),
        );
    }


    #[test]
    fn snow_surface_menu_dtor_matches() {
        check(
            ee::s06::f_0024fe00__generated,
            snow_surface_menu_dtor,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x24fe00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x130, 0);
                w(c, OBJ + 0x164, 0);
            },
        );
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


    #[test]
    fn detail_system_menu_dtor_matches() {
        check(
            ee::s06::f_0024fab8__generated,
            detail_system_menu_dtor,
            60,
            &|c, seed| {
                let _r = base_4(c, seed, 0x24fa);
                c.g[A0][0] = DATA as u64;
                // Bit 0 of the flags would free the (test) object.
                c.g[A1][0] = 2;
            },
        );
    }


    #[test]
    fn plants_menu_dtor_matches() {
        diff(ee::s06::f_0024f860__generated, plants_menu_dtor, 30, &|c, seed| {
            let _r = fe_base(c, seed, 0x24_f860);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            for off in (0x130..0x3a0).step_by(4) {
                w(c, DATA + off, 0);
            }
        });
    }


    #[test]
    fn depth_fog_menu_dtor_matches() {
        strict(ssx3_ee::s06::f_0024dc68__generated, depth_fog_menu_dtor, SEEDS_4, &|c, seed| {
            let _ = base_5(c, seed, 0xdc68);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(if seed % 2 == 0 { 0 } else { 2 });
        });
    }


    /// The allocator's mutex (`MEM_MUTEX`) and the fake result that keeps the
    /// delete path short.
    pub(crate) const MEM_MUTEX: u32 = 0x0053_8b00;


    pub(crate) fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        w(c, MEM_MUTEX + 4, 0x42);
        w(c, LOG_RESULTS, 0x42);
        w(c, MEM_MUTEX + 8, r.next_u32() % 4);
    }


        /// The deleting-destructor fixture: a random page pointer, both values of
    /// the delete flag and a short `operator delete` path.
    pub(crate) fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 2);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn visual_effects_test_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024d998__generated,
            visual_effects_test_menu_dtor,
            0x24_d998,
        );
    }


    #[test]
    fn visual_effects_main_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024f6c0__generated,
            visual_effects_main_menu_dtor,
            0x24_f6c0,
        );
    }


        #[test]
    fn snowfall_toggles_menu_dtor_matches() {
        dtor_case(ee::s06::f_0024e200__generated, snowfall_toggles_menu_dtor, 0x24_e200);
    }


    #[test]
    fn render_toggles_menu_dtor_matches() {
        check_all(
            ee::s06::f_0024d710__generated,
            render_toggles_menu_dtor,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24d710, 0);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
                // Destructor flags with bit 0 clear, so the items do not take the
                // `operator delete` branch.
                c.g[A1][0] = ((r.next_u32() as u64) << 1) | 0x2;
            },
        );
    }
}
