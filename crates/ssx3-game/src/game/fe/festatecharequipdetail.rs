//! `src/fe/festatecharequipdetail.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::gp;
pub(crate) use crate::game::ai::ai::GAME_MODULE_GP;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_4;
pub(crate) use crate::game::dirtysock::tagsunk::gp_8 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sb;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::fe::festatecharequipdetail::bolt_file_index_manager_get_node_2 as bolt_file_index_manager_get_node;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2_m2;
pub(crate) use crate::game::fe::ovstatepause::lw as lw_m2;
pub(crate) use crate::game::movie::movieplayer::at as at_m2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_m2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::ui::uiengine::oc391_at;
pub(crate) use crate::game::ui::uiengine::BOLT_FREE_SLOT;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_9148,
        cfestate_charequip_dtor,
        "cFEStateCharEquip_dtor",
    ),
    (
        0x0019_9350,
        cfe_state_char_equip_shutdown,
        "cFEStateCharEquip_Shutdown",
    ),
    (
        0x0019_93a0,
        c_fe_state_char_equip_on_activate,
        "cFEStateCharEquip_OnActivate",
    ),
    (
        0x0019_9420,
        fe_char_equip_on_deactivate,
        "cFEStateCharEquip_OnDeactivate",
    ),
    (
        0x0019_9870,
        cfe_state_char_equip_on_child_event,
        "cFEStateCharEquip_OnChildEvent",
    ),
    (
        0x0019_9a38,
        cfe_state_char_equip_handle_input,
        "cFEStateCharEquip_HandleInput",
    ),
    (
        0x0019_a098,
        c_fe_state_char_equip_get_preview_texture,
        "cFEStateCharEquip_GetPreviewTexture",
    ),
    (
        0x0019_a4e8,
        cfe_state_char_equip_request_preview,
        "cFEStateCharEquip_RequestPreview",
    ),
    (
        0x0019_afd0,
        cfe_state_char_equip_category_has_flagged_item,
        "cFEStateCharEquip_CategoryHasFlaggedItem",
    ),
    (
        0x0019_b098,
        c_fe_state_char_equip_category_all_flagged,
        "cFEStateCharEquip_CategoryAllFlagged",
    ),
    (
        0x0019_bfe8,
        cfe_state_char_equip_update_preview_camera,
        "cFEStateCharEquip_UpdatePreviewCamera",
    ),
    (
        0x0019_c880,
        bolt_file_index_manager_alloc_node,
        "BoltFileIndexManager_AllocNode",
    ),
    (
        0x0019_c8d0,
        bolt_file_index_manager_get_prev_node,
        "BoltFileIndexManager_GetPrevNode",
    ),
    (
        0x0019_c920,
        bolt_file_index_manager_node_index,
        "BoltFileIndexManager_NodeIndex",
    ),
    (
        0x0019_ca70,
        bolt_file_index_manager_free_node,
        "BoltFileIndexManager_FreeNode",
    ),
    (
        0x0019_cb38,
        bolt_file_index_manager_ctor,
        "BoltFileIndexManager_ctor",
    ),
    (
        0x0019_cbe0,
        bolt_file_index_manager_get_model_slot,
        "BoltFileIndexManager_GetModelSlot",
    ),
    (
        0x0019_cc30,
        bolt_file_index_manager_set_model_slot,
        "BoltFileIndexManager_SetModelSlot",
    ),
    (
        0x0019_cce8,
        bolt_file_index_manager_get_texture_slot,
        "BoltFileIndexManager_GetTextureSlot",
    ),
    (
        0x0019_cd38,
        bolt_file_index_manager_set_texture_slot,
        "BoltFileIndexManager_SetTextureSlot",
    ),
    (0x0019_cdf0, bolt_file_cache_ctor, "BoltFile_CacheCtor"),
    (0x0019_ce20, bolt_file_cache_dtor, "BoltFile_CacheDtor"),
    (
        0x0019_d3b0,
        bolt_file_find_free_slot,
        "BoltFile_FindFreeSlot",
    ),
    (
        0x0019_d428,
        bolt_file_find_slot_by_name,
        "BoltFile_FindSlotByName",
    ),
    (0x0019_e2b0, bolt_file_get_data, "BoltFile_GetData"),
    (0x0019_e330, bolt_file_detach_data, "BoltFile_DetachData"),
    (
        0x0019_e498,
        fe_module_rider_preview_dtor,
        "FEModule_RiderPreviewDtor",
    ),
    (
        0x0019_a238,
        cfe_state_char_equip_show_equip_mode,
        "cFEStateCharEquip_ShowEquipMode",
    ),
    (
        0x0019_a308,
        cfe_state_char_equip_show_buy_mode,
        "cFEStateCharEquip_ShowBuyMode",
    ),
    (
        0x0019_a3d0,
        cfe_state_char_equip_toggle_buy_equip,
        "cFEStateCharEquip_ToggleBuyEquip",
    ),
    (
        0x0019_a498,
        c_fe_state_char_equip_release_preview,
        "cFEStateCharEquip_ReleasePreview",
    ),
    (
        0x0019_a9b8,
        cfe_state_char_equip_refresh_item_info,
        "cFEStateCharEquip_RefreshItemInfo",
    ),
    (
        0x0019_aca0,
        c_fe_state_char_equip_refresh_rider_bolts,
        "cFEStateCharEquip_RefreshRiderBolts",
    ),
    (
        0x0019_b458,
        cfe_state_char_equip_remove_list_item,
        "cFEStateCharEquip_RemoveListItem",
    ),
    (
        0x0019_b518,
        c_fe_state_char_equip_fill_equip_rows,
        "cFEStateCharEquip_FillEquipRows",
    ),
    (
        0x0019_b598,
        c_fe_state_char_equip_fill_buy_rows,
        "cFEStateCharEquip_FillBuyRows",
    ),
    (
        0x0019_bba0,
        c_fe_state_char_equip_update_money_text,
        "cFEStateCharEquip_UpdateMoneyText",
    ),
    (
        0x0019_bbe0,
        cfe_state_char_equip_set_category_title,
        "cFEStateCharEquip_SetCategoryTitle",
    ),
    (
        0x0019_bc90,
        cfe_state_char_equip_update_stat_bar,
        "cFEStateCharEquip_UpdateStatBar",
    ),
    (
        0x0019_c7e8,
        cfe_state_char_equip_open_buy_popup,
        "cFEStateCharEquip_OpenBuyPopup",
    ),
    (
        0x0019_c8f8,
        bolt_file_index_manager_get_next_node,
        "BoltFileIndexManager_GetNextNode",
    ),
    (
        0x0019_cb60,
        bolt_file_index_manager_reset,
        "BoltFileIndexManager_Reset",
    ),
    (0x0019_ce68, boltfile_cache_init, "BoltFile_CacheInit"),
    (
        0x0019_cf40,
        bolt_file_cache_shutdown,
        "BoltFile_CacheShutdown",
    ),
    (0x0019_e238, bolt_file_is_loaded, "BoltFile_IsLoaded"),
    (
        0x0019_e538,
        femodule_rider_preview_set_visible,
        "FEModule_RiderPreviewSetVisible",
    ),
    (
        0x0019_c938,
        bolt_file_index_manager_find_node,
        "BoltFileIndexManager_FindNode",
    ),
    (0x0019_d250, bolt_file_cache_update, "BoltFile_CacheUpdate"),
    (0x0019_d4b8, bolt_file_free_slot, "BoltFile_FreeSlot"),
    (0x0019_d000, bolt_file_unpack_slot, "BoltFile_UnpackSlot"),
    (
        0x0019_d140,
        bolt_file_start_next_queued,
        "BoltFile_StartNextQueued",
    ),
    (
        0x0019_d578,
        bolt_file_ensure_budget,
        "BoltFile_EnsureBudget",
    ),
    (0x0019_d8b8, bolt_file_load_slot, "BoltFile_LoadSlot"),
    (0x0019_da10, bolt_file_acquire, "BoltFile_Acquire"),
    (0x0019_dc20, bolt_file_release, "BoltFile_Release"),
    (
        0x0019_dd10,
        bolt_file_compare_model_slots_2,
        "BoltFile_CompareModelSlots",
    ),
    (
        0x0019_e3d0,
        fe_module_rider_preview_ctor,
        "FEModule_RiderPreviewCtor",
    ),
    (
        0x0019_e588,
        fe_module_rider_preview_request_char,
        "FEModule_RiderPreviewRequestChar",
    ),
    (
        0x0019_de18,
        bolt_file_compare_texture_slots,
        "BoltFile_CompareTextureSlots",
    ),
    (
        0x0019_df20,
        bolt_file_prioritize_models,
        "BoltFile_PrioritizeModels",
    ),
    (
        0x0019_e070,
        bolt_file_prioritize_textures,
        "BoltFile_PrioritizeTextures",
    ),
    (
        0x0019_8db8,
        cfestate_char_equip_ctor,
        "cFEStateCharEquip_ctor",
    ),
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
    (
        0x0019_a638,
        cfe_state_char_equip_update_preview,
        "cFEStateCharEquip_UpdatePreview",
    ),
    (
        0x0019_a758,
        cfe_state_char_equip_release_preview_bolt_textures,
        "cFEStateCharEquip_ReleasePreviewBoltTextures",
    ),
    (
        0x0019_a798,
        cfe_state_char_equip_update_help_text,
        "cFEStateCharEquip_UpdateHelpText",
    ),
    (
        0x0019_aa08,
        cfe_state_char_equip_refresh_equip_info,
        "cFEStateCharEquip_RefreshEquipInfo",
    ),
    (
        0x0019_ab78,
        cfe_state_char_equip_refresh_buy_info,
        "cFEStateCharEquip_RefreshBuyInfo",
    ),
    (
        0x0019_b180,
        cfe_state_char_equip_build_item_list,
        "cFEStateCharEquip_BuildItemList",
    ),
    (
        0x0019_b618,
        cfe_state_char_equip_refresh_item_list,
        "cFEStateCharEquip_RefreshItemList",
    ),
    (
        0x0019_b7e0,
        cfe_state_char_equip_set_equip_row,
        "cFEStateCharEquip_SetEquipRow",
    ),
    (
        0x0019_ba60,
        cfe_state_char_equip_set_buy_row,
        "cFEStateCharEquip_SetBuyRow",
    ),
    (
        0x0019_bd48,
        cfe_state_char_equip_update_stat_preview_bar,
        "cFEStateCharEquip_UpdateStatPreviewBar",
    ),
    (
        0x0019_be80,
        cfe_state_char_equip_buy_selected_item,
        "cFEStateCharEquip_BuySelectedItem",
    ),
    (
        0x0019_bee8,
        cfe_state_char_equip_equip_selected_item,
        "cFEStateCharEquip_EquipSelectedItem",
    ),
    (0x0019_d738, bolt_file_evict_bytes, "BoltFile_EvictBytes"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;

    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BOLT_FILE_INDEX_MANAGER_RESET;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::FEMODULE_GET_RIDER_PREVIEW;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_OBJECT_SET_VISIBLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;

    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;

    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;

    pub(crate) use crate::game::fe::fepopup::callee::C0019E2B0;

    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;

    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;

    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;

    pub const BOLT_FILE_INDEX_MANAGER_FIND_NODE: Callee = Callee {
        addr: 0x0019_c938,
        f: ee::s03::f_0019c938,
    };
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;

    pub const BOLT_FILE_RELEASE: Callee = Callee {
        addr: 0x0019_dc20,
        f: ee::s03::f_0019dc20,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {

    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
}

/// `void* BoltFile_GetData(BoltFile *file, int outer, int inner)` (0x19e2b0):
/// 0 unless the state word at `file + outer*0x12000 + inner*0x120` is 1, else
/// the data pointer at `+0x104`.
pub fn bolt_file_get_data(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000;
    c.mult(0, A2, A2, V1);
    c.mult(0, A1, A1, V0);
    c.g[A3][0] = c.g[A0][0];
    c.g[A0][0] = 0x1;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let not_loaded = c.g[V1][0] != c.g[A0][0];
    c.g[V1][0] = c.g[V0][0];
    if not_loaded {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x104).lw(c);
    }
    abi::ret(c);
}

/// `void BoltFile_DetachData(BoltFile *file, int outer, int inner)`
/// (0x19e330): clears the state word and the data pointer of a loaded record,
/// and decrements the global live-data counter at `file + 0xb4000`.
pub fn bolt_file_detach_data(c: &mut Cpu) {
    c.cycles += 20;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000;
    c.mult(0, A2, A2, V1);
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x104));
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
    c.g[V0][0] = 0xb0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4000).lw(c);
    c.g[A1][0] = Addr(c.g[A3][0] as u32).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x4000).sw(c, c.g[V1][0]);
    Addr(c.g[A3][0] as u32).sw(c, 0);
    abi::ret(c);
}

/// `Node* BoltFileIndexManager_GetPrevNode(Node *base, Node *node)` (0x19c8d0).
pub fn bolt_file_index_manager_get_prev_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node(c, 0xc);
}

/// `int BoltFileIndexManager_NodeIndex(Node *base, Node *node)` (0x19c920):
/// `(node - base) >> 4`, or -1 for a null node.
pub fn bolt_file_index_manager_node_index(c: &mut Cpu) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(abi::arg(c, 0)));
    if node != 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Other constructors that clear their object.

/// `void* BoltFileIndexManager_ctor(void* self)` (0x19cb38): reset the two
/// arrays and return `self`.
pub fn bolt_file_index_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BOLT_FILE_INDEX_MANAGER_RESET.call(c, 0x0019_cb4c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt file cache and rider previews.

/// `BoltFile_Cache* BoltFile_CacheCtor(BoltFile_Cache* self)` (0x19cdf0):
/// publish the cache singleton (`gp-0x1784`), initialise it and return it.
pub fn bolt_file_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let self_ = c.g[S0][0];
        Addr(glo(c, g::BOLT_FILE_CACHE)).sw(c, self_);
        callee::BOLT_FILE_CACHE_INIT.call(c, 0x0019_ce08)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `void cFEStateCharEquip_OnDeactivate(cFEStateCharEquip* self)`
/// (0x199420): hide the rider preview of the character selected in `+0x44`.
pub fn fe_char_equip_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = world;
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::GET_RIDER_PREVIEW.call(c, 0x0019_943c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_9448)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void BoltFile_CacheDtor(BoltFileCache* c, int flags)` (0x19ce20): clear the
/// global bolt-file cache slot, shut the cache down and, when `flags & 1`,
/// delete the object.
pub fn bolt_file_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.sw(gp(c, -0x1784), 0);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::BOLT_FILE_CACHE_SHUTDOWN.call(c, 0x0019_ce40)?;
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        // The original reloads s0 before the delete branch (see `dtor`).
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::DELETE.call(c, 0x0019_ce54)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `BoltFileIndexManager_GetModelSlot` /
/// `GetTextureSlot`: find the index node for `(slot << 16) | index`, test bit
/// `slot` of the node's availability mask at `mask_off`, and return the value
/// at `value_off` (or -1).
pub(crate) fn bolt_slot_lookup(c: &mut Cpu, shift: u32, mask_off: u32, value_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        BOLT_FILE_INDEX_MANAGER_FIND_NODE.call(c, ret)?;
        c.g[V1][0] = c.g[V0][0];
        let node = c.g[V1][0] as u32;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if node != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(node).off(mask_off).lh(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (shift & 31)) as u32);
            c.g[V0][0] &= 1;
            let available = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if available {
                c.cycles += 1;
                c.g[V0][0] = Addr(node).off(value_off).lh(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BoltFileIndexManager_GetModelSlot(this, slot, index)` (0x19cbe0).
pub fn bolt_file_index_manager_get_model_slot(c: &mut Cpu) {
    let shift = c.g[A1][0] as u32;
    bolt_slot_lookup(c, shift, 6, 0xa, 0x0019_cbf8)
}

/// `int BoltFileIndexManager_GetTextureSlot(this, slot, index)` (0x19cce8).
pub fn bolt_file_index_manager_get_texture_slot(c: &mut Cpu) {
    let shift = c.g[A1][0] as u32;
    bolt_slot_lookup(c, shift, 4, 8, 0x0019_cd00)
}

/// `a0 = a0 + off` (the `addiu` the generated code uses for arguments).
#[inline(always)]
pub(crate) fn add_arg(c: &mut Cpu, reg: usize, off: u32) {
    c.g[reg][0] = sx(r32(c, reg).wrapping_add(off));
}

/// `void cFEStateCharEquip_Shutdown(cFEStateCharEquipDetail* s)` (0x199350):
/// invalidate the rider preview's card (-1 at +0xc2c) when a module manager is
/// present, then run the base shutdown. 8 (+6) + 6 cycles plus the callees.
pub fn cfe_state_char_equip_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let module = c.lw(gp_2(c, GAME_MODULE_GP));
        c.g[V0][0] = module;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let manager = c.lw(p(module as u32, 0x7c));
        c.g[A0][0] = manager;
        if manager != 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(self_, 0x44)).lb(c);
            add_arg(c, A0, 0xb0);
            callee::FEMODULE_GET_RIDER_PREVIEW.call(c, 0x0019_937c)?;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.sw(p(r32(c, V0), 0xc2c), c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_OBJECT_SET_VISIBLE, 0x0019_9390)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoltFileIndexNode* BoltFileIndexManager_AllocNode(BoltFileIndexManager* m)`
/// (0x19c880): pop the free list at +0x1f40 and mark both the popped node's
/// and the new head's link words -1; return the popped node. 14 + 6 cycles
/// plus the callee.
pub fn bolt_file_index_manager_alloc_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        let node = c.lw(p(self_, 0x1f40));
        c.g[S1][0] = node;
        c.g[A1][0] = node;
        callee::BOLT_FILE_INDEX_GET_NEXT_NODE.call(c, 0x0019_c8a0)?;
        c.sw(p(self_, 0x1f40), c.g[V0][0]);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(node as u32, 0xe)).sh(c, c.g[V1][0]);
        c.g[V0][0] = c.lw(p(self_, 0x1f40));
        if c.g[V0][0] != 0 {
            Addr(p(r32(c, V0), 0xc)).sh(c, c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoltFile_FindFreeSlot` (0x19D3B0): find a free bolt slot in `bank`'s 256
/// entries, preferring an unused-but-valid slot with the smallest stamp;
/// returns -1 when none.
pub fn bolt_file_find_free_slot(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x1_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] |= 0x2000;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V1, A1, V0);
    c.g[A1][0] = 0;
    c.g[A0][0] = low(c, V1).wrapping_add(low(c, A0));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[A1][0] = low(c, A1).wrapping_add(1);
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_at(c, A0, 0x104);
            let free = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[A1][0];
            if free {
                c.cycles += 2;
                c.g[V0][0] = c.g[A1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let no_best = (c.g[A2][0] as i64) < 0;
            c.g[V1][0] = lw_at(c, A0, 0x11c);
            let mut take_best = no_best;
            if !no_best {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = low(c, A1).wrapping_add(1);
                    c.cycles += 3;
                    c.g[A0][0] = low(c, A0).wrapping_add(0x120);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d3d0);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0];
                    abi::ret(c);
                    return;
                }
                take_best = true;
            }
            if take_best {
                c.cycles += 3;
                c.g[A3][0] = c.g[V1][0];
                c.g[A2][0] = c.g[A1][0];
                c.g[A1][0] = low(c, A1).wrapping_add(1);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = low(c, A0).wrapping_add(0x120);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0019_d3d0);
        }
    }
}

/// `void cFEStateCharEquip_OnActivate(cFEStateCharEquip* s, void* arg)`
/// (0x001993a0): show buy mode when the equip flag is set (else equip mode),
/// then refresh the stat bars and the item list and activate the base state.
pub fn c_fe_state_char_equip_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let equip = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if equip {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0019_a238, 0x0019_93dc)?; // cFEStateCharEquip_ShowEquipMode
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0019_a308, 0x0019_93cc)?; // cFEStateCharEquip_ShowBuyMode
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019_bc90, 0x0019_93e4)?; // cFEStateCharEquip_UpdateStatBar
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0019_bd48, 0x0019_93f0)?; // cFEStateCharEquip_UpdateStatPreviewBar
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0019_b618, 0x0019_9400)?; // cFEStateCharEquip_RefreshItemList
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0018_6518, 0x0019_940c)?; // cFEState_OnActivate
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `int BoltFile_FindSlotByName(EquipTable* self, int bank, const char* name)`
/// (0x19d428): index of the named slot in the bank's 256-entry table, or -1.
pub fn bolt_file_find_slot_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        let _self_ = abi::arg(c, 0);
        let _bank = abi::arg64(c, 1);
        let name = abi::arg64(c, 2);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        frame.save(c, S0, 0x40);
        c.mult(0, 5, 5, 2);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = name;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(4));
        c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lb(at(c, S1, 4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            let mut found = false;
            if empty {
                c.cycles += 1;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, callee::STRCMP, 0x0019_d474)?;
                if c.g[V0][0] != 0 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    found = true;
                }
            }
            if found {
                break;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x120));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 256) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x120));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_d460);
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

/// `void FEModule_RiderPreviewDtor(this, flags)` (0x19e498): unload the rider
/// preview, then release every occupied preview slot (0x100 of them).
pub fn fe_module_rider_preview_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = sx(this.wrapping_add(0x1c));
        frame.save_ra(c, 0);
        c.g[S4][0] = u64::MAX;
        callee::FEMODULE_RIDER_PREVIEW_UNLOAD.call(c, 0x0019_e4c8)?;
        c.g[S1][0] = 0xff;
        'slots: loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let free = (c.g[A1][0] as i64) < 0;
            c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
            if !free {
                c.cycles += 5;
                c.g[V0][0] = lw_2(c, A2, 0x10d8);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
                c.g[V1][0] = lw_2(c, V0, 0x194);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_e4f0)?;
            }
            c.cycles += 5;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            Addr(c.g[S0][0] as u32).off(4).sb(c, 0);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_e4d0);
                }
                continue 'slots;
            }
            break 'slots;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x50);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0019_e518)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of [`bolt_file_index_manager_set_model_slot`] and
/// [`bolt_file_index_manager_set_texture_slot`]: set (`value >= 0`) or clear
/// a per-node slot bit, dropping the node when it has no more references.
pub(crate) fn bolt_set_slot(
    c: &mut Cpu,
    entry: u32,
    flags_off: u32,
    value_off: u32,
    ref_off: u32,
) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    let manager = c.g[A0][0];
    c.g[S2][0] = manager;
    let bit = c.g[A1][0];
    c.g[S0][0] = bit;
    c.g[S1][0] = c.g[A3][0];
    c.g[A3][0] = !(0u64 | c.g[S1][0]);
    frame.save_ra(c, 0);
    c.g[A3][0] = (0u64 < c.g[A3][0]) as u64;
    call_ptr(c, 0x0019_c938, entry + 0x2c)?;
    c.g[A1][0] = c.g[V0][0];
    let mut skip_one = c.g[A1][0] == 0;
    if !skip_one {
        c.cycles += 2;
        let store = (c.g[S1][0] as i64) >= 0;
        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, flags_off)).lhu(c);
        if !store {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, flags_off)).lh(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S0][0] as u32) & 31)) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let clear = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1;
            if !clear {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
                let other = c.g[V1][0] != 0;
                c.sh(p(c.g[A1][0] as u32, flags_off), c.g[V0][0]);
                if !other {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[A1][0] as u32, ref_off)).lh(c);
                    let referenced = (c.g[V0][0] as i64) >= 0;
                    if !referenced {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0019_ca70, entry + 0x24)?;
                        skip_one = true;
                    }
                }
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
            c.sh(p(c.g[A1][0] as u32, value_off), c.g[S1][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sh(p(c.g[A1][0] as u32, flags_off), c.g[V1][0]);
        }
    }
    if !skip_one {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x30);
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void BoltFileIndexManager_SetModelSlot(self, int bit, int a2, int value)`
/// (0x19cc30): set or clear a model-slot bit on the node for (bit, a2).
pub fn bolt_file_index_manager_set_model_slot(c: &mut Cpu) {
    guest(c, |c| bolt_set_slot(c, 0x0019_cc30, 6, 0xa, 8))
}

/// `void BoltFileIndexManager_SetTextureSlot(self, int bit, int a2, int value)`
/// (0x19cd38): the same for texture slots.
pub fn bolt_file_index_manager_set_texture_slot(c: &mut Cpu) {
    guest(c, |c| bolt_set_slot(c, 0x0019_cd38, 4, 8, 0xa))
}

/// `cFEStateCharEquip_OnChildEvent(this, widget, msg)` (0x199870): keep the
/// equip screen's dirty flag in step with the selected item and buy it.
pub fn cfe_state_char_equip_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x15;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_15 = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = abi::arg64(c, 0);

        if is_15 {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::r32(c, A1)).off(0xc).lw(c);
            c.g[V0][0] = 7;
            let selected = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = sx(0xffff_fff7_u32);
            if !selected {
                c.cycles += 4;
                c.g[V0][0] = Addr(abi::r32(c, S0)).off(0x1c).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                Addr(abi::r32(c, S0)).off(0x1c).sw(c, c.g[V0][0]);
            }
            // 'b_199924 tail
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = 0x16;
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 2;
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // msg 0x16
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        c.cycles += 4;
        c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] | 8;
            Addr(c.g[S0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.g[V1][0] = Addr(abi::r32(c, A1)).off(0xc).lw(c);

        c.cycles += 3;
        c.g[V0][0] = 7;
        if c.g[V1][0] != c.g[V0][0] {
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(abi::r32(c, A1)).off(0x6c).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM.call(c, 0x0019_98f8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT.call(c, 0x0019_9900)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x5c).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x95).lbu(c);
        callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM.call(c, 0x0019_9910)?;
        let removed = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !removed {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST.call(c, 0x0019_9924)?;
        }

        c.cycles += 1;
        c.lq(16, frame.at(c, 0x10));
        c.cycles += 3;
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
        c.g[A1][0] = Addr(p_2(c.g[V0][0], 0xbc)).lw(c);
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
                c.g[V0][0] = Addr(p_2(c.g[A0][0], 0x34)).lw(c) & 0x4;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p_2(c.g[A0][0], 0x4)).lh(c);
                    c.g[V1][0] = Addr(p_2(c.g[S0][0], 0x288)).lw(c);
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
                    c.g[V0][0] = Addr(p_2(c.g[V0][0], 0x2)).lhu(c) & 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p_2(c.g[A0][0], 0x34)).lw(c) & 0x20;
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
        'b_19cb20: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            let t = c.g[S0][0] == 0u64;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            if t {
                break 'b_19cb20;
            } else {
                'b_19caa0: {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = 0x190;
                    if c.g[V0][0] == 0u64 {
                        break 'b_19caa0;
                    } else {
                        break 'b_19caa0;
                    }
                }
                'b_19cadc: {
                    'b_19cacc: {
                        c.cycles += 5;
                        c.div(0, 4, 2);
                        c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[S2][0] = c.hi[0];
                        let t = (c.g[V1][0] as i64) >= 0;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_19cacc;
                        } else {
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x0019cabc);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0019c8f8, 0x0019cabc)?;
                            c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            let t = true;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x1900), c.g[V0][0]);
                            if t {
                                break 'b_19cadc;
                            } else {
                                break 'b_19cacc;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019cad4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0019c8d0, 0x0019cad4)?;
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xe));
                    c.sh((c.g[V0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
                    break 'b_19cadc;
                }
                'b_19caf8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_19caf8;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0019caf0);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0019c8f8, 0x0019caf0)?;
                        c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.sh((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        break 'b_19caf8;
                    }
                }
                c.cycles += 10;
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A0][0] = c.g[S1][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0xa), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[RA][0] = sx(0x0019cb18);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1f40));
                call_ptr(c, 0x0019c920, 0x0019cb18)?;
                c.sh((c.g[S0][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1f40), c.g[S0][0]);
                break 'b_19cb20;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateCharEquip_CategoryAllFlagged (0x000019b098).

/// `cFEStateCharEquip_CategoryAllFlagged`: report whether every item of a character-equipment category has the given flag.
pub fn c_fe_state_char_equip_category_all_flagged(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b168: {
            'b_19b164: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x870));
                c.g[T0][0] = 0xffffffffffffffff;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x850));
                c.g[T1][0] = 0x1;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x860));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
                c.g[A0][0] = c.g[A1][0];
                c.g[S0][0] = c.g[A2][0];
                c.g[A2][0] = c.g[A3][0];
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
                c.g[A3][0] = c.g[SP][0];
                call_ptr(c, 0x0014d7e8, 0x0019b0cc)?;
                c.g[A3][0] = c.g[V0][0];
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[A2][0] = 0u64;
                if t {
                    break 'b_19b164;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[SP][0];
                    'l_19b0e0: loop {
                        'b_19b158: {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V0][0] & 0x4;
                            if c.g[V0][0] == 0u64 {
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                break 'b_19b158;
                            } else {
                                'b_19b120: {
                                    'b_19b11c: {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_19b11c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            let t = true;
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if t {
                                                break 'b_19b120;
                                            } else {
                                                break 'b_19b11c;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                    break 'b_19b120;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.g[V0][0] & 0x2;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    break 'b_19b158;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xcc));
                                    if c.g[V1][0] != c.g[V0][0] {
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                        break 'b_19b158;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                                        c.g[V0][0] = c.g[V0][0] & 0x20;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            break 'b_19b158;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_19b168;
                                            } else {
                                                break 'b_19b158;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0019b0e0);
                            }
                            continue 'l_19b0e0;
                        } else {
                            break 'b_19b164;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_19b168;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x850));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x870));
        c.pc = t;
        Ok(())
    });
}

/// `cFEStateCharEquip_dtor(self, flags)` (0x199148): release the character
/// equipment screen's preview and its four bolt-file bindings, then run the
/// UI state destructor (and free the object when `flags & 1`).
pub fn cfestate_charequip_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let _this = abi::arg(c, 0);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ab8));
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A2][0] = lw(c, p(c.g[S0][0] as u32, 0xa64));
        let t = (c.g[A2][0] as i64) < 0;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 7;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
            c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x7c));
            c.g[RA][0] = sx(0x0019_918c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            c.pc = 0x0019_dc20;
            ee::s03::BoltFile_Release(c);
            if c.pc != 0x0019_918c {
                return Err(Unwound);
            }
            c.g[V0][0] = sx(0xffff_ffff);
            c.sw(p(c.g[S0][0] as u32, 0xa64), c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa68));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_91b4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_91b4 {
                return Err(Unwound);
            }
        }
        c.cycles += 7;
        c.g[S1][0] = sx(0xffff_ffff);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0019_91c4);
        c.sw(p(c.g[S0][0] as u32, 0xa68), c.g[S1][0]);
        c.pc = 0x0019_a498;
        ee::s03::cFEStateCharEquip_ReleasePreview(c);
        if c.pc != 0x0019_91c4 {
            return Err(Unwound);
        }
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa70));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_91e4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_91e4 {
                return Err(Unwound);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa74));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_9204);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_9204 {
                return Err(Unwound);
            }
        }
        c.cycles += 14;
        c.g[V0][0] = 0x47_0000;
        c.sw(p(c.g[S0][0] as u32, 0xa74), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.sw(p(c.g[S0][0] as u32, 0xa70), c.g[S1][0]);
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0019_9224);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e390, 0x0019_9224)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoltFile_CompareModelSlots(row, id)` (0x19dd10): compare two model
/// slots of a bolt row by name: an empty/disabled slot sorts before a live
/// one; two live slots are compared with `strcasecmp`.
pub fn bolt_file_compare_model_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        let row = abi::arg(c, 0);
        let id = abi::arg(c, 1);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x0001_2000;
        c.g[T0][0] = 0x120;
        c.g[A3][0] = 3;
        c.g[V1][0] = lw(c, row);
        c.g[A2][0] = c.lhu(id);
        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
        c.mult(0, A0, A0, A2);
        c.g[V1][0] &= 0xffff;
        c.mult(0, V1, V1, T0);
        c.g[V0][0] = lw(c, gp(c, -0x1784));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.mult(0, V1, A2, T0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw(c, c.g[A0][0] as u32);
        let mut check_other = false;
        let mut memcmp = false;
        let mut extra = 0u32; // 1: the `break b_19de08` path charges one more
        if c.g[V1][0] != c.g[A3][0] {
            c.g[A2][0] = lw(c, c.g[A1][0] as u32);
            check_other = true;
        } else {
            c.cycles += 4;
            c.g[A2][0] = lw(c, p(c.g[A0][0] as u32, 0x110));
            c.g[V0][0] = 8;
            if c.g[A2][0] != c.g[V0][0] {
                c.g[A2][0] = lw(c, c.g[A1][0] as u32);
                check_other = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x11c));
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = lw(c, c.g[A1][0] as u32);
                    check_other = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if t {
                        extra = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x110));
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        if t {
                            extra = 1;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x11c));
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            if t {
                                memcmp = true;
                            } else {
                                c.cycles += 2;
                            }
                        }
                    }
                }
            }
        }
        if check_other {
            // sb is not live: 0; otherwise the same tests as sa, then 1.
            c.cycles += 3;
            c.g[V0][0] = 3;
            let t = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if t {
                extra = 1;
            } else {
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0x110));
                c.g[V0][0] = 8;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if t {
                    extra = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x11c));
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        extra = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, c.g[A0][0] as u32);
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 1;
                        if t {
                            extra = 1;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x110));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[V0][0] = 1;
                            if t {
                                extra = 1;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x11c));
                                let t = c.g[V0][0] != 0;
                                c.g[V0][0] = 1;
                                if t {
                                    extra = 1;
                                } else {
                                    memcmp = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        if memcmp {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0041_aa88, 0x003e_de00)?; // strcasecmp
        } else {
            c.cycles += extra as u64;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_0019A098(void* self, const char* name)` (0x19a098): find the
/// equipment package entry called `name` for the character in `self` (equip
/// table at `world+0x7c + 0x1a70`, slots at `+0xbc`/`+0xa64`) and return its
/// model id. A missing package or a miss returns -1; a hit binds the entry's
/// data through slot 46 or 48 depending on whether an id was already stored
/// at `self+0xa68`.
pub fn c_fe_state_char_equip_get_preview_texture(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    'b_19a1b0: {
        'b_19a1ac: {
            c.cycles += 14;
            frame.save(c, S3, 0x60);
            frame.save(c, S5, 0x40);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S4, 0x50);
            frame.save(c, S6, 0x30);
            frame.save(c, S7, 0x20);
            frame.save_ra(c, 0x10);
            c.g[V0][0] = lw(c, at(c, S3, 0xa60));
            let t = c.g[V0][0] == 0u64;
            c.g[S5][0] = c.g[A1][0];
            if t {
                break 'b_19a1ac;
            } else {
                c.cycles += 9;
                c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                c.g[A1][0] = lw(c, at(c, S3, 0xbc));
                c.g[A0][0] = lw(c, at(c, V0, 0x7c));
                c.g[A2][0] = lw(c, at(c, S3, 0xa64));
                c.g[RA][0] = sx(0x0019a0e8);
                c.g[A0][0] = sx(at(c, A0, 0x1a70));
                if callee::C0019E2B0.call(c, 0x0019a0e8).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_19a1b0;
                } else {
                    c.cycles += 3;
                    c.g[S4][0] = lw(c, at(c, S1, 0x8));
                    let t = (c.g[S4][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_19a1b0;
                    } else {
                        c.cycles += 4;
                        c.g[S6][0] = 0xffffffffffffffff;
                        c.g[S7][0] = 0x4a0000;
                        c.g[S2][0] = sx(at(c, S1, 0x14));
                        'l_19a110: loop {
                            'b_19a1a0: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0019a120);
                                c.g[A2][0] = c.g[SP][0];
                                if call_ptr(c, 0x003b3d00, 0x0019a120).is_err() {
                                    return;
                                }
                                sb(c, at(c, SP, 0x4), 0u64);
                                c.g[A0][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x0019a130);
                                c.g[A1][0] = c.g[S5][0];
                                if call_ptr(c, 0x004165a8, 0x0019a130).is_err() {
                                    return;
                                }
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx(at(c, S0, 0x1));
                                if t {
                                    break 'b_19a1a0;
                                } else {
                                    'b_19a198: {
                                        'b_19a17c: {
                                            c.cycles += 4;
                                            c.g[V0][0] = lw(c, r32(c, S2));
                                            c.g[A1][0] = lw(c, at(c, S3, 0xa68));
                                            let t = c.g[A1][0] != c.g[S6][0];
                                            c.g[A2][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            if t {
                                                break 'b_19a17c;
                                            } else {
                                                c.cycles += 13;
                                                c.g[T2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                                c.g[A1][0] = c.g[A2][0];
                                                c.g[A2][0] = sx(at(c, S7, 0x1950));
                                                c.g[A3][0] = 0u64;
                                                c.g[V1][0] = lw(c, at(c, T2, 0x10d8));
                                                c.g[T0][0] = 0x1;
                                                c.g[T1][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = lh(c, at(c, V1, 0x170));
                                                c.g[V0][0] = lw(c, at(c, V1, 0x174));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x0019a174);
                                                c.g[A0][0] =
                                                    sx(r32(c, T2).wrapping_add(r32(c, A0)));
                                                if call_ptr(c, t, 0x0019a174).is_err() {
                                                    return;
                                                }
                                                let t = true;
                                                sw_2(c, at(c, S3, 0xa68), c.g[V0][0]);
                                                if t {
                                                    break 'b_19a198;
                                                } else {
                                                    break 'b_19a17c;
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[T0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                        c.g[A3][0] = 0x1;
                                        c.g[V0][0] = lw(c, at(c, T0, 0x10d8));
                                        c.g[A0][0] = lh(c, at(c, V0, 0x180));
                                        c.g[V1][0] = lw(c, at(c, V0, 0x184));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x0019a198);
                                        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                                        if call_ptr(c, t, 0x0019a198).is_err() {
                                            return;
                                        }
                                        break 'b_19a198;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = lw(c, at(c, S3, 0xa68));
                                    if t {
                                        break 'b_19a1b0;
                                    } else {
                                        break 'b_19a1a0;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S2][0] = sx(at(c, S2, 0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019a110);
                                }
                                continue 'l_19a110;
                            } else {
                                break 'b_19a1ac;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0xffffffffffffffff;
        break 'b_19a1b0;
    }
    c.cycles += 11;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xa0));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Rider/item preview: camera, stick input, item preview request (tick rate).

/// Fields of `cFEStateCharEquip` used by the preview.
mod equip_preview {
    /// Mode word: 1 = the equip menu has the input (no preview control).
    pub const MODE: u32 = 0xc8;
    /// Preview zoom (0..1) and pitch (-1..1), driven by the right stick.
    pub const ZOOM: u32 = 0xaa4;
    pub const PITCH: u32 = 0xaa8;
    /// Preview yaw (degrees, 0..360), driven by the left stick.
    pub const YAW: u32 = 0xaac;
    /// Turntable angle of the preview rider (degrees, +1 per 60 Hz tick).
    pub const SPIN: u32 = 0xab0;
    /// The camera's two vectors (position-like and target-like) and its
    /// state (1/2: snap to a view, 3/4: glide to a view, then 2/1).
    pub const CAM_A: u32 = 0xac0;
    pub const CAM_B: u32 = 0xad0;
    pub const CAM_STATE: u32 = 0xae0;
    /// Item preview: load delay (ticks), item list, category.
    pub const PREVIEW_DELAY: u32 = 0x95c;
}

const PCAM_GET_RIDER_PREVIEW: Callee = Callee {
    addr: 0x001a_0548,
    f: ee::s03::f_001a0548,
};
const PCAM_SET_TRANSFORM: Callee = Callee {
    addr: 0x0019_f3e8,
    f: ee::s03::f_0019f3e8,
};
const PCAM_SET_TRANSFORM2: Callee = Callee {
    addr: 0x0019_f548,
    f: ee::s03::f_0019f548,
};
const PREQ_RELEASE_PREVIEW: Callee = Callee {
    addr: 0x0019_a498,
    f: ee::s03::f_0019a498,
};
const PCAM_MATH_SIN_COS: u32 = 0x0031_be50;
const PCAM_SNAP_CHASE_VIEW: u32 = 0x0015_e050;

/// `base(reg) + off`.
#[inline(always)]
fn pv_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp - off`.
#[inline(always)]
fn pv_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `lui at, hi; mtc1 at, f`.
#[inline(always)]
fn pv_lif(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(1, f);
}

/// The quaternion product the preview builds its rotations with:
/// `sp+0x40 = sp+0x10 * sp+0x30` (VU0 macro ops of the original).
fn pv_quat_mul(c: &mut Cpu) {
    let a = sp(c);
    c.lqc2(4, a.wrapping_add(0x10));
    c.lqc2(5, a.wrapping_add(0x30));
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, a.wrapping_add(0x40));
}

/// `mfc1 v1, f1` of the glide factor: at another rate the factor for the
/// rate (`1 - (1 - a)^(60/rate)`), so the glide keeps its time constant.
/// `f1` itself stays (it is also the settle threshold).
fn pv_glide_factor_to_v1(c: &mut Cpu) {
    if crate::tick::rate(c) == 60 {
        c.mfc1(3, 1);
    } else {
        let a = FpuF32(c.f[1]);
        let a = crate::game::ai::rider::smoothing_60(c, a);
        c.g[V1][0] = sx(a.0);
    }
}

/// States 3/4: both camera vectors move by `factor` (`.lit4` at `gp-k`) of
/// their gap to the view in `sp+0x20`/`sp+0x30`; returns whether the gap of
/// the vector at `sp+cmp` (before the move) was below the factor (the
/// settle test, `c.lt.s`).
fn pv_glide(c: &mut Cpu, k: u32, cmp: u32) -> bool {
    let s = sp(c);
    let at = |o: u32| s.wrapping_add(o);
    c.lqc2(3, at(0x20));
    c.lqc2(4, pv_at(c, S0, equip_preview::CAM_A));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(0x50));
    c.lwc1(1, pv_gp(c, k));
    c.lq(2, at(0x50));
    c.sq(2, at(0x40));
    c.lqc2(3, at(0x30));
    c.lqc2(4, pv_at(c, S0, equip_preview::CAM_B));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(0x60));
    c.lq(2, at(0x60));
    c.sq(2, at(0x50));
    pv_glide_factor_to_v1(c);
    c.lqc2(4, at(0x40));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(0x70));
    c.lq(2, at(0x70));
    c.sq(2, at(0x60));
    c.lqc2(3, pv_at(c, S0, equip_preview::CAM_A));
    c.lqc2(4, at(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, pv_at(c, S0, equip_preview::CAM_A));
    pv_glide_factor_to_v1(c);
    c.lqc2(4, at(0x50));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(0x70));
    c.lq(2, at(0x70));
    c.sq(2, at(0x60));
    c.lqc2(3, pv_at(c, S0, equip_preview::CAM_B));
    c.lqc2(4, at(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, pv_at(c, S0, equip_preview::CAM_B));
    c.lqc2(3, at(cmp));
    c.lqc2(5, at(cmp));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.c_lt_s(0, 1);
    c.fcc()
}

/// `void cFEStateCharEquip_UpdatePreviewCamera(cFEStateCharEquip* self)`
/// (0x19bfe8), once per tick on the equip screen: when the rider preview is
/// loaded, move the preview camera by its state (1/2 snap to the zoomed or
/// the overview, 3/4 glide there and settle), turn the rider by the stick
/// yaw, spin its turntable (+1 degree per 60 Hz tick, wrapped at 360), set
/// both preview transforms; then snap the game camera to the default view.
///
/// Tick rate: the spin step is `1 * 60/rate` and the glide factor
/// `1 - 0.8^(60/rate)` (the 0.2 settle threshold stays); exact at 60.
pub fn cfe_state_char_equip_update_preview_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x170);
        c.g[V0][0] = c.lw(pv_gp(c, 0x848));
        frame.save(c, S0, 0x130);
        frame.save(c, S4, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x120);
        frame.save(c, S2, 0x110);
        frame.save(c, S3, 0x100);
        frame.save(c, S5, 0xe0);
        frame.save(c, S6, 0xd0);
        frame.save(c, S7, 0xc0);
        frame.save_ra(c, 0xb0);
        frame.save_fpr(c, 24, 0x160);
        frame.save_fpr(c, 23, 0x158);
        frame.save_fpr(c, 22, 0x150);
        frame.save_fpr(c, 21, 0x148);
        frame.save_fpr(c, 20, 0x140);
        c.g[A0][0] = c.lw(pv_at(c, V0, 0x7c));
        c.g[A1][0] = c.lb(pv_at(c, S0, 0x44));
        c.g[A0][0] = sx(pv_at(c, A0, 0xb0));
        PCAM_GET_RIDER_PREVIEW.call(c, 0x0019_c03c)?;
        // The preview is ready: state < 10, loaded (+0xcb8) and shown (+0xcb4).
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        c.g[V1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(pv_at(c, S4, 0xcb8));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(pv_at(c, S4, 0xcb4));
                c.g[V1][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[V1][0] != 0 {
            preview_camera_move(c);
            preview_transforms(c)?;
        }

        // Snap the game camera to its default chase view of the front end.
        c.cycles += 40;
        let s = sp(c);
        c.g[V0][0] = c.lw(pv_gp(c, 0x848));
        c.g[A1][0] = c.g[SP][0];
        pv_lif(c, 3, 0x3f80_0000);
        c.g[A2][0] = sx(s.wrapping_add(0x10));
        c.g[S0][0] = c.lw(pv_at(c, V0, 0x7c));
        pv_lif(c, 1, 0xc2dc_0000);
        pv_lif(c, 2, 0x43c5_0000);
        c.g[A0][0] = sx(pv_at(c, S0, 0x10));
        pv_lif(c, 0, 0xc20c_0000);
        c.swc1(1, s);
        c.swc1(0, s.wrapping_add(0x10));
        c.swc1(2, s.wrapping_add(4));
        c.swc1(3, s.wrapping_add(0x1c));
        c.sw(s.wrapping_add(8), 0);
        c.swc1(3, s.wrapping_add(0xc));
        c.sw(s.wrapping_add(0x14), 0);
        c.sw(s.wrapping_add(0x18), 0);
        call_ptr(c, PCAM_SNAP_CHASE_VIEW, 0x0019_c7a0)?;
        c.lwc1(0, pv_gp(c, 0x5974));
        frame.restore(c, S1, 0x120);
        c.swc1(0, pv_at(c, S0, 0x10));
        frame.restore(c, S0, 0x130);
        frame.restore(c, S2, 0x110);
        frame.restore(c, S3, 0x100);
        frame.restore(c, S4, 0xf0);
        frame.restore(c, S5, 0xe0);
        frame.restore(c, S6, 0xd0);
        frame.restore(c, S7, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 24, 0x160);
        frame.restore_fpr(c, 23, 0x158);
        frame.restore_fpr(c, 22, 0x150);
        frame.restore_fpr(c, 21, 0x148);
        frame.restore_fpr(c, 20, 0x140);
        frame.close(c);
        Ok(())
    })
}

/// The camera state step of [`cfe_state_char_equip_update_preview_camera`]
/// (0x19c09c..0x19c3a4): ends with `sp+0x10 = 0`.
fn preview_camera_move(c: &mut Cpu) {
    use equip_preview as f;
    let s = sp(c);
    let at = |o: u32| s.wrapping_add(o);
    c.cycles += 12;
    c.lwc1(2, pv_at(c, S0, f::ZOOM));
    c.lwc1(0, pv_at(c, S0, f::PITCH));
    pv_lif(c, 3, 0x4282_0000);
    c.mul_s(0, 0, 2);
    pv_lif(c, 1, 0x43a0_0000);
    c.g[V1][0] = c.lw(pv_at(c, S0, f::CAM_STATE));
    c.mul_s(2, 2, 1);
    c.lwc1(24, pv_at(c, S0, f::YAW));
    c.mul_s(4, 0, 3);

    // Whether the camera reached its view this tick (sp+0x10 cleared with 1
    // more cycle) or is still moving / idle (cleared directly).
    let settled_path: bool;
    if c.g[V1][0] == c.g[A1][0] {
        // State 1: snap to the zoomed view (zoom/pitch place it).
        c.cycles += 20;
        pv_lif(c, 0, 0xc2c8_0000);
        pv_lif(c, 1, 0xc2a0_0000);
        c.add_s(0, 2, 0);
        pv_lif(c, 3, 0x3f80_0000);
        c.add_s(1, 4, 1);
        pv_lif(c, 2, 0xc2d2_0000);
        c.swc1(3, at(0x2c));
        c.swc1(2, at(0x20));
        c.swc1(0, at(0x24));
        c.swc1(1, at(0x28));
        c.lwc1(0, pv_gp(c, 0x59a0));
        c.lwc1(1, pv_gp(c, 0x599c));
        pv_lif(c, 2, 0xc368_0000);
        c.lq(3, at(0x20));
        pv_snap(c);
        settled_path = true;
    } else {
        c.cycles += 3;
        c.g[A0][0] = 2;
        c.g[V0][0] = 3;
        if c.g[V1][0] == c.g[A0][0] {
            // State 2: snap to the overview.
            c.cycles += 18;
            pv_lif(c, 0, 0xc37a_0000);
            c.lwc1(1, pv_gp(c, 0x5998));
            pv_lif(c, 2, 0xc357_0000);
            pv_lif(c, 3, 0x3f80_0000);
            c.swc1(0, at(0x20));
            c.swc1(1, at(0x24));
            c.swc1(2, at(0x28));
            c.swc1(3, at(0x2c));
            pv_lif(c, 0, 0xc2c8_0000);
            pv_lif(c, 1, 0x4234_0000);
            pv_lif(c, 2, 0x4170_0000);
            c.lq(3, at(0x20));
            pv_snap(c);
            settled_path = true;
        } else {
            c.cycles += 2;
            let state3 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 4;
            if state3 {
                // State 3: glide to the overview, then state 2.
                c.cycles += 69;
                pv_lif(c, 6, 0x3f80_0000);
                pv_lif(c, 0, 0xc37a_0000);
                c.lwc1(1, pv_gp(c, 0x5994));
                pv_lif(c, 2, 0xc357_0000);
                pv_lif(c, 3, 0xc2c8_0000);
                pv_lif(c, 4, 0x4234_0000);
                pv_lif(c, 5, 0x4170_0000);
                c.swc1(0, at(0x20));
                c.swc1(1, at(0x24));
                c.swc1(2, at(0x28));
                c.swc1(3, at(0x30));
                c.swc1(4, at(0x34));
                c.swc1(5, at(0x38));
                c.swc1(6, at(0x3c));
                c.swc1(6, at(0x2c));
                if pv_glide(c, 0x5990, 0x50) {
                    c.sw(pv_at(c, S0, f::CAM_STATE), c.g[A0][0]);
                    settled_path = true;
                } else {
                    c.cycles += 2;
                    settled_path = false;
                }
            } else {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    settled_path = false;
                } else {
                    // State 4: glide to the zoomed view, then state 1.
                    c.cycles += 70;
                    pv_lif(c, 0, 0xc2c8_0000);
                    pv_lif(c, 1, 0xc2a0_0000);
                    c.add_s(0, 2, 0);
                    pv_lif(c, 6, 0x3f80_0000);
                    c.add_s(1, 4, 1);
                    pv_lif(c, 2, 0xc2d2_0000);
                    c.lwc1(3, pv_gp(c, 0x598c));
                    c.lwc1(4, pv_gp(c, 0x5988));
                    pv_lif(c, 5, 0xc368_0000);
                    c.swc1(0, at(0x24));
                    c.swc1(1, at(0x28));
                    c.swc1(2, at(0x20));
                    c.swc1(3, at(0x30));
                    c.swc1(4, at(0x34));
                    c.swc1(5, at(0x38));
                    c.swc1(6, at(0x3c));
                    c.swc1(6, at(0x2c));
                    if pv_glide(c, 0x5984, 0x40) {
                        c.sw(pv_at(c, S0, f::CAM_STATE), c.g[A1][0]);
                    }
                    settled_path = true;
                }
            }
        }
    }
    if settled_path {
        c.cycles += 1;
    }
    c.sw(at(0x10), 0);
}

/// States 1/2: both camera vectors take the view (`v1` = the first, the
/// second from f0..f2 over `sp+0x20`).
fn pv_snap(c: &mut Cpu) {
    let s = sp(c);
    c.cycles += 7;
    c.swc1(0, s.wrapping_add(0x20));
    c.swc1(1, s.wrapping_add(0x24));
    c.swc1(2, s.wrapping_add(0x28));
    c.sq(3, pv_at(c, S0, equip_preview::CAM_A));
    c.lq(2, s.wrapping_add(0x20));
    c.sq(2, pv_at(c, S0, equip_preview::CAM_B));
}

/// The two preview transforms (0x19c3a4..0x19c748): rotations built from
/// sin/cos of the angles (yaw from the stick, the turntable spin), the spin
/// step, both `SetTransform` calls.
fn preview_transforms(c: &mut Cpu) -> Flow {
    use equip_preview as f;
    let s = sp(c);
    let at = |o: u32| s.wrapping_add(o);
    c.cycles += 130;
    c.g[A0][0] = sx(at(0x80));
    pv_lif(c, 21, 0x3f80_0000);
    c.g[A1][0] = sx(at(0x84));
    c.lwc1(20, at(0x10));
    c.g[S7][0] = 0x50_0000;
    c.lq(2, pv_at(c, S0, f::CAM_A));
    c.g[S6][0] = sx(pv_at(c, S7, 0u32.wrapping_sub(0xec0)));
    c.swc1(21, at(0x1c));
    c.mov_s(12, 20);
    c.sq(2, s);
    c.g[S5][0] = 0x50_0000;
    c.sw(at(0x14), 0);
    c.g[S3][0] = sx(pv_at(c, S5, 0u32.wrapping_sub(0xeb0)));
    c.sw(at(0x18), 0);
    c.g[S2][0] = 0x50_0000;
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c3e8)?;
    c.lwc1(23, pv_gp(c, 0x5980));
    c.lwc1(1, pv_at(c, S6, 4));
    c.g[A0][0] = sx(at(0x88));
    c.lwc1(0, at(0x80));
    c.g[A1][0] = sx(at(0x8c));
    c.lwc1(2, pv_at(c, S7, 0u32.wrapping_sub(0xec0)));
    c.mov_s(12, 20);
    c.lwc1(4, pv_at(c, S6, 8));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0x84));
    c.mul_s(0, 0, 4);
    pv_lif(c, 22, 0x3f00_0000);
    c.swc1(3, at(0x3c));
    c.g[S1][0] = sx(pv_at(c, S2, 0u32.wrapping_sub(0xea0)));
    c.swc1(2, at(0x30));
    c.swc1(1, at(0x34));
    c.swc1(0, at(0x38));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c474)?;
    c.lwc1(0, pv_at(c, S3, 4));
    c.g[A0][0] = sx(at(0x90));
    c.lwc1(1, at(0x88));
    c.g[A1][0] = sx(at(0x94));
    c.lwc1(2, pv_at(c, S5, 0u32.wrapping_sub(0xeb0)));
    pv_lif(c, 12, 0x42a0_0000);
    c.mul_s(0, 1, 0);
    c.lwc1(4, pv_at(c, S3, 8));
    c.mul_s(2, 1, 2);
    c.add_s(12, 24, 12);
    c.lwc1(3, at(0x8c));
    c.mul_s(1, 1, 4);
    c.swc1(0, at(0x34));
    c.swc1(3, at(0x3c));
    c.mul_s(12, 12, 23);
    c.swc1(2, at(0x30));
    c.swc1(1, at(0x38));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.mul_s(12, 12, 22);
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c500)?;
    c.lwc1(0, pv_at(c, S1, 4));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(2, at(0x90));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(3, pv_at(c, S2, 0u32.wrapping_sub(0xea0)));
    c.lwc1(4, pv_at(c, S1, 8));
    c.mul_s(0, 2, 0);
    c.mul_s(3, 2, 3);
    c.lwc1(1, at(0x94));
    c.mul_s(2, 2, 4);
    c.swc1(1, at(0x3c));
    c.swc1(0, at(0x34));
    c.swc1(3, at(0x30));
    c.swc1(2, at(0x38));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    PCAM_SET_TRANSFORM.call(c, 0x0019_c578)?;

    // The turntable: +1 degree per 60 Hz tick (f21 = 1.0, also the w of
    // the identity rotation stored below), wrapped once at 360.
    c.lwc1(0, pv_at(c, S0, f::SPIN));
    pv_lif(c, 1, 0x43b4_0000);
    if crate::tick::rate(c) == 60 {
        c.add_s(0, 0, 21);
    } else {
        let step = crate::game::ai::rider::rate_step_60(c, FpuF32(c.f[21]));
        c.f[0] = (FpuF32(c.f[0]) + step).0;
    }
    c.lq(2, pv_at(c, S0, f::CAM_B));
    c.swc1(20, at(0x18));
    c.sq(2, s);
    c.c_le_s(1, 0);
    c.swc1(21, at(0x1c));
    c.swc1(20, at(0x10));
    c.swc1(20, at(0x14));
    let wrap = c.fcc();
    c.swc1(0, pv_at(c, S0, f::SPIN));
    if wrap {
        c.cycles += 2;
        c.sub_s(0, 0, 1);
        c.swc1(0, pv_at(c, S0, f::SPIN));
    }

    c.cycles += 101;
    c.lwc1(12, pv_gp(c, 0x597c));
    c.g[A0][0] = sx(at(0x98));
    c.g[A1][0] = sx(at(0x9c));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c5c4)?;
    c.lwc1(1, pv_at(c, S1, 8));
    c.g[A0][0] = sx(at(0xa0));
    c.lwc1(0, at(0x98));
    c.g[A1][0] = sx(at(0xa4));
    c.lwc1(2, pv_at(c, S2, 0u32.wrapping_sub(0xea0)));
    c.lwc1(4, pv_at(c, S1, 4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0x9c));
    c.mul_s(0, 0, 4);
    c.lwc1(12, pv_gp(c, 0x5978));
    c.swc1(3, at(0x3c));
    c.swc1(1, at(0x38));
    c.swc1(2, at(0x30));
    c.swc1(0, at(0x34));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c640)?;
    c.lwc1(1, pv_at(c, S3, 8));
    c.g[A0][0] = sx(at(0xa8));
    c.lwc1(0, at(0xa0));
    c.g[A1][0] = sx(at(0xac));
    c.lwc1(2, pv_at(c, S5, 0u32.wrapping_sub(0xeb0)));
    c.lwc1(4, pv_at(c, S3, 4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0xa4));
    c.mul_s(0, 0, 4);
    pv_lif(c, 5, 0x42b4_0000);
    c.swc1(3, at(0x3c));
    c.swc1(1, at(0x38));
    c.swc1(2, at(0x30));
    c.swc1(0, at(0x34));
    pv_quat_mul(c);
    c.lwc1(12, pv_at(c, S0, f::SPIN));
    c.lq(2, at(0x40));
    c.add_s(12, 12, 5);
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    c.mul_s(12, 12, 23);
    c.mul_s(12, 12, 22);
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c6d0)?;
    c.lwc1(1, pv_at(c, S6, 8));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(0, at(0xa8));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(2, pv_at(c, S7, 0u32.wrapping_sub(0xec0)));
    c.lwc1(4, pv_at(c, S6, 4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0xac));
    c.mul_s(0, 0, 4);
    c.swc1(3, at(0x3c));
    c.swc1(1, at(0x38));
    c.swc1(2, at(0x30));
    c.swc1(0, at(0x34));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    PCAM_SET_TRANSFORM2.call(c, 0x0019_c748)?;
    Ok(())
}

/// `int cFEStateCharEquip_HandleInput(cFEStateCharEquip* self, Pad* pad)`
/// (0x199a38): unless the equip menu has the input (`+0xc8 == 1`), in the
/// camera states 1/4 the left stick turns the preview (`yaw += x * 3`,
/// wrapped to 0..360) and the right stick zooms (`+0xaa4`, 0..1) and
/// pitches (`+0xaa8`, -1..1) it by `-0.1` per tick. Returns whether
/// anything moved.
///
/// `UIState_Update` calls it every tick while the pad has input held (the
/// state's pad mask `+0x15` is set in its ctor), so it is a per-tick step:
/// the 3.0 becomes `3.0 * 60/rate` (the -0.1 steps are `tick::PER_TICK`
/// `.lit4` entries already). Exact at 60.
pub fn cfe_state_char_equip_handle_input(c: &mut Cpu) {
    use equip_preview as f;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(pv_at(c, S1, f::MODE)) ^ 1;
        c.g[S2][0] = 0;
        'out: {
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
                break 'out;
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(pv_at(c, S1, f::CAM_STATE));
            c.g[V0][0] = 1;
            let one = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 4;
            if !one {
                c.cycles += 2;
                let four = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = c.g[S2][0];
                if !four {
                    break 'out;
                }
            }

            // Left stick: yaw.
            c.cycles += 7;
            c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            pv_pad_call(c, V1, V0, 0x118, 0x0019_9a90)?;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            } else {
                c.cycles += 16;
                c.g[V0][0] = c.lw(pv_at(c, S0, 8));
                pv_pad_call(c, V0, V1, 0x130, 0x0019_9aac)?;
                pv_lif(c, 1, 0x4040_0000);
                crate::game::ai::rider::per_tick_60(c, 1);
                c.lwc1(2, pv_at(c, S1, f::YAW));
                c.mul_s(0, 0, 1);
                pv_lif(c, 3, 0x43b4_0000);
                c.add_s(2, 2, 0);
                c.c_le_s(3, 2);
                let wrap = c.fcc();
                c.swc1(2, pv_at(c, S1, f::YAW));
                if wrap {
                    const HEAD: u32 = 0x0019_9ae0;
                    c.cycles += 2;
                    c.mov_s(0, 3);
                    c.sub_s(2, 2, 0);
                    loop {
                        c.cycles += 4;
                        c.c_le_s(0, 2);
                        if !c.fcc() {
                            break;
                        }
                        c.sub_s(2, 2, 0);
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                    c.cycles += 1;
                    c.swc1(2, pv_at(c, S1, f::YAW));
                }
                c.cycles += 6;
                c.lwc1(1, pv_at(c, S1, f::YAW));
                c.mtc1(0, 2);
                c.c_lt_s(1, 2);
                if c.fcc() {
                    const HEAD: u32 = 0x0019_9b18;
                    c.cycles += 3;
                    pv_lif(c, 3, 0x43b4_0000);
                    loop {
                        c.cycles += 5;
                        c.add_s(0, 1, 3);
                        c.c_lt_s(0, 2);
                        let again = c.fcc();
                        c.mov_s(1, 0);
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                    c.cycles += 2;
                    c.swc1(0, pv_at(c, S1, f::YAW));
                }
                c.g[S2][0] = 1;
                c.cycles += 1;
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            }

            // Right stick up/down: zoom.
            c.cycles += 6;
            pv_pad_call(c, V1, V0, 0x108, 0x0019_9b48)?;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            } else {
                c.cycles += 16;
                c.g[V0][0] = c.lw(pv_at(c, S0, 8));
                pv_pad_call(c, V0, V1, 0x120, 0x0019_9b64)?;
                c.lwc1(1, pv_gp(c, 0x59a8));
                c.lwc1(3, pv_at(c, S1, f::ZOOM));
                c.mul_s(0, 0, 1);
                pv_lif(c, 4, 0x3f80_0000);
                c.mtc1(0, 2);
                c.add_s(3, 3, 0);
                c.c_le_s(2, 3);
                let inside = c.fcc();
                c.mov_s(0, 2);
                if inside {
                    c.cycles += 1;
                    c.min_s(0, 3, 4);
                }
                c.cycles += 3;
                c.swc1(0, pv_at(c, S1, f::ZOOM));
                c.g[S2][0] = 1;
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            }

            // Right stick left/right: pitch.
            c.cycles += 6;
            pv_pad_call(c, V1, V0, 0x110, 0x0019_9bb0)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[S2][0];
            if none {
                break 'out;
            }
            c.cycles += 17;
            c.g[V0][0] = c.lw(pv_at(c, S0, 8));
            pv_pad_call(c, V0, V1, 0x128, 0x0019_9bcc)?;
            c.lwc1(1, pv_gp(c, 0x59a4));
            c.lwc1(3, pv_at(c, S1, f::PITCH));
            c.mul_s(0, 0, 1);
            pv_lif(c, 2, 0xbf80_0000);
            pv_lif(c, 4, 0x3f80_0000);
            c.add_s(3, 3, 0);
            c.c_le_s(2, 3);
            let inside = c.fcc();
            c.mov_s(1, 2);
            if inside {
                c.cycles += 1;
                c.min_s(1, 3, 4);
            }
            c.cycles += 2;
            c.swc1(1, pv_at(c, S1, f::PITCH));
            c.g[S2][0] = 1;
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
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

/// A pad method call `pad->slot()` (pad in `s0`): the vtable already in
/// `vt`, the method into `fnr` (the registers the original uses).
fn pv_pad_call(c: &mut Cpu, vt: usize, fnr: usize, slot: u32, ret: u32) -> Flow {
    let v = c.g[vt][0] as u32;
    c.g[A0][0] = c.lh(v.wrapping_add(slot));
    c.g[fnr][0] = c.lw(v.wrapping_add(slot + 4));
    let target = c.g[fnr][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void cFEStateCharEquip_RequestPreview(cFEStateCharEquip* self, int
/// delayed)` (0x19a4e8): drop the current item preview (and the screen's
/// preview texture), then, for a selected item with a model name, start its
/// load after a delay of 20 frames when `delayed` (1 tick otherwise), and
/// build the file name `"<bank>_<name>.<digits>"`-style into `+0x960`.
///
/// Tick rate: the 20 frames are `ticks_from_frames60(20)` ticks
/// (`cFEStateCharEquip_UpdatePreview` counts the delay down per tick).
pub fn cfe_state_char_equip_request_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(pv_at(c, S1, 0x5c));
        c.g[S0][0] = c.g[A1][0];
        'tail: {
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                break 'tail;
            }
            c.cycles += 5;
            PREQ_RELEASE_PREVIEW.call(c, 0x0019_a518)?;
            c.g[A1][0] = c.lw(pv_at(c, S1, 0xa74));
            c.g[A2][0] = c.lw(pv_gp(c, 0x854));
            if (c.g[A1][0] as i64) >= 0 {
                // Release the screen's preview texture (a method of gp-0x854).
                c.cycles += 5;
                c.g[V0][0] = c.lw(pv_at(c, A2, 0x10d8));
                c.g[A0][0] = c.lh(pv_at(c, V0, 0x190));
                c.g[V1][0] = c.lw(pv_at(c, V0, 0x194));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(pv_at(c, A2, c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_a538)?;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw(pv_at(c, S1, 0xa70));
            c.g[V1][0] = u64::MAX;
            c.sw(pv_at(c, S1, 0xa70), c.g[V1][0]);
            c.sw(pv_at(c, S1, 0xa74), c.g[V0][0]);
            c.g[V1][0] = c.lw(pv_at(c, S1, 0x5c));
            c.g[V1][0] = c.lbu(pv_at(c, V1, 0x95));
            // (`bltz` on the byte just loaded: never taken.)
            c.cycles += 4;
            c.g[V0][0] = c.lw(pv_at(c, S1, 0x958));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let past_end = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if past_end {
                c.cycles += 1;
                break 'tail;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(pv_at(c, S1, c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(pv_at(c, V0, 0x124));
            c.g[S2][0] = c.lw(pv_at(c, V1, 0x30));
            if c.g[S2][0] == 0 {
                break 'tail;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = crate::tick::ticks_from_frames60(c, 20) as u64;
            }
            c.cycles += 24;
            c.sw(pv_at(c, S1, equip_preview::PREVIEW_DELAY), c.g[V0][0]);
            call_ptr(c, 0x0014_dc80, 0x0019_a590)?; // BELibrary_GetInstance
            c.g[S0][0] = sx(pv_at(c, S1, 0x960));
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 9;
            call_ptr(c, 0x0014_dd58, 0x0019_a5a0)?; // BELibrary_GetInterface(9)
            c.g[A1][0] = c.lw(pv_at(c, S1, 0xbc));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_b700, 0x0019_a5ac)?; // BEBoltInt_DispatchCategory
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0x2e;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_acc0, 0x0019_a5bc)?; // strrchr(path, '.')
            c.g[A1][0] = c.g[S1][0];
            c.g[S3][0] = c.g[V0][0];
            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_6ce0, 0x0019_a5d8)?; // strncpy
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x4a_0000;
            c.g[S0][0] = sx(pv_at(c, S0, c.g[S1][0] as u32));
            c.sb(c.g[S0][0] as u32, 0);
            // Skip the name to its first digit.
            const HEAD: u32 = 0x0019_a5e8;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                if c.g[V0][0] != 0 {
                    break;
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
            c.cycles += 5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1958));
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[A0][0];
            call_ptr(c, 0x0041_7828, 0x0019_a618)?; // sprintf("%s_%s%s")
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

pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
}

// ---------------------------------------------------------------------------
// Bolt file stream (front-end file manager).

/// `bool BoltFile_IsLoaded(BoltFile *file, int outer, int inner)` (0x19e238):
/// reads the word at_m2 `file + outer*0x12000 + inner*0x120` and returns whether
/// it equals 1.
pub fn bolt_file_is_loaded(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000; // 0x12000: stride of the outer table
    c.mult(0, A2, A2, V1); // inner *= 0x120
    c.mult(0, A1, A1, V0); // outer *= 0x12000
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end module: rider previews.

/// `void FEModule_RiderPreviewSetVisible(FEModule *self, uint visible)`
/// (0x19e538): when `visible` is zero clears the +0xcc4/+0xcc8 flags; otherwise
/// sets +0xcc4 to 1 if any of the +0xcc0/+0xcb8/+0xcb4 words is zero or +0xcd4
/// is non-negative, else sets +0xcc8 to 1.
pub fn femodule_rider_preview_set_visible(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    if abi::arg64(c, 1) == 0 {
        this.off(0xcc4).sw(c, 0);
        c.cycles += 2;
        this.off(0xcc8).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let mut early;
    c.g[V0][0] = this.off(0xcc0).lw(c);
    early = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1;
    if !early {
        c.cycles += 3;
        c.g[V0][0] = this.off(0xcb8).lw(c);
        early = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !early {
            c.cycles += 3;
            c.g[V0][0] = this.off(0xcb4).lw(c);
            early = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1;
            if !early {
                c.cycles += 3;
                c.g[V0][0] = this.off(0xcd4).lw(c);
                early = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 0x1;
                if !early {
                    c.cycles += 2;
                    this.off(0xcc8).sw(c, c.g[V0][0]);
                    abi::ret(c);
                    return;
                }
            }
        }
    }
    c.cycles += 2;
    this.off(0xcc4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bolt file index manager.

/// Shared body of `BoltFileIndexManager_GetPrev/NextNode` (0x19c8d0/0x19c8f8):
/// follow the 16-bit node link at_m2 `node + link` and return `base + index*16`.
pub(crate) fn bolt_file_index_manager_get_node_2(c: &mut Cpu, link: u32) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    if node != 0 {
        c.cycles += 3;
        let index = Addr(node as u32).off(link).lh(c);
        c.g[V0][0] = index;
        let negative = (index as i64) < 0;
        let scaled = sx((index as u32) << 4);
        c.g[V0][0] = scaled;
        if !negative {
            c.cycles += 2;
            c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(scaled as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Node* BoltFileIndexManager_GetNextNode(Node *base, Node *node)` (0x19c8f8).
pub fn bolt_file_index_manager_get_next_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node_2(c, 0xe);
}

/// `void cFEStateCharEquip_UpdateMoneyText(State* s)` (0x19bba0): the same
/// with the text at_m2 `+0x64` and the value at_m2 `+0xc0`.
pub fn c_fe_state_char_equip_update_money_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw_m2(c, this, 0x64);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw_m2(c, this, 0xc0);
            callee::FE_FORMAT_MONEY.call(c, 0x0019_bbc4)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw_m2(c, this, 0x64);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING.call(c, 0x0019_bbd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_ReleasePreview(self)` (0x19a498): release the
/// previewed equipment's bolt file (if a slot is held) and clear the preview
/// state.
pub fn c_fe_state_char_equip_release_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(this).off(0xa6c).lw(c);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(this).off(0xbc).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            BOLT_FILE_RELEASE.call(c, 0x0019_a4c4)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        Addr(this).off(0x960).sb(c, 0);
        Addr(this).off(0x95c).sw(c, c.g[V0][0]);
        Addr(this).off(0xa6c).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshItemInfo(cFEStateCharEquipDetail* s)`
/// (0x19a9b8): refresh the buy panel when the detail flags word at_m2 +0xc8 is
/// 1, else the equip panel, then the help text. 8 (+4, +2) + 6 cycles plus
/// the callees.
pub fn cfe_state_char_equip_refresh_item_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(p(self_, 0xc8));
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            callee::CHAR_EQUIP_REFRESH_BUY_INFO.call(c, 0x0019_a9e0)?;
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::CHAR_EQUIP_REFRESH_EQUIP_INFO.call(c, 0x0019_a9f0)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_EQUIP_UPDATE_HELP_TEXT.call(c, 0x0019_a9f8)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, cameras, memory cards.

/// `cFEStateCharEquip_RefreshRiderBolts` (0x19ACA0): ask the char-equip state's
/// rider preview for the rider's currently equipped bolts.
pub fn c_fe_state_char_equip_refresh_rider_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp_m2(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0xb0);
        call_ptr(c, addr::RIDER_PREVIEW, 0x0019_acc8)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, addr::BE_GET_BE, 0x0019_acd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, addr::BE_GET_INTERFACE, 0x0019_acdc)?;
        c.g[A2][0] = lw_at(c, S0, 0xbc);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        call_ptr(c, addr::BE_GET_INTERFACE_9, 0x0019_acec)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[A3][0] = 1;
        call_ptr(c, addr::RIDER_PREVIEW_BOLTS, 0x0019_ad00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_FillEquipRows(self)` (0x0019b518): fill up to six
/// equip rows for the selected profile.
pub fn c_fe_state_char_equip_fill_equip_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at_m2(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at_m2(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at_m2(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW.call(c, 0x0019_b558)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at_m2(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b550);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_FillBuyRows(self)` (0x0019b598): fill up to six
/// buy rows for the selected profile.
pub fn c_fe_state_char_equip_fill_buy_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at_m2(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at_m2(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at_m2(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW.call(c, 0x0019_b5d8)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at_m2(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b5d0);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BoltFileIndexManager_Reset(BoltFileIndexManager* m)` (0x0019cb60):
/// clear the index and the tail table, point every one of the 400 entries at_m2
/// the list head and mark the table's end with -1.
pub fn bolt_file_index_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = sx(0xffff_ffff);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] = 0x1900;
        call_ptr(c, 0x003e_6448, 0x0019_cb80)?; // EA_MemSet
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1900));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x640;
        call_ptr(c, 0x003e_6448, 0x0019_cb90)?; // EA_MemSet
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f40), c.g[S0][0]);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.sh((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A1][0] = c.g[V1][0];
            c.sh((c.g[A0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 400) as u64;
            c.sh((c.g[A0][0] as u32).wrapping_add(4), 0);
            c.sh((c.g[A0][0] as u32).wrapping_add(6), 0);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            if !more {
                c.cycles += 6;
                c.g[V0][0] = sx(0xffff_ffff);
                c.g[RA][0] = c.ld(sp(c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x18fe), c.g[V0][0]);
                c.lq(S0, sp(c).wrapping_add(0x10));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
                abi::ret(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_cba0);
            }
        }
    })
}

/// `void cFEStateCharEquip_OpenBuyPopup(self, item)` (0x19c7e8): allocate a buy
/// popup, fill it from the item's price/id and post it to the screen.
pub fn cfe_state_char_equip_open_buy_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x460000;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 0x70;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x790));
        call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0019_c81c)?;
        c.g[A1][0] = lw_2_m2(c, S1, 0x10);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = Addr(c.g[S1][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BUY_POPUP_CTOR, 0x0019_c830)?;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0xe).lh(c);
        c.g[S2][0] = c.g[V0][0];
        let no_price = (c.g[A2][0] as i64) <= 0;
        c.g[A1][0] = lw_2_m2(c, S0, 0x14);
        if !no_price {
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            c.mult(0, 6, 6, 2);
        }
        c.cycles += 13;
        c.g[A3][0] = lw_2_m2(c, S1, 0xc0);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48));
        callee::BUY_POPUP_INIT_BUY_BOLT.call(c, 0x0019_c854)?;
        c.g[A0][0] = lw_2_m2(c, S1, 0x10);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, table_callee::UI_OBJECT_PUSH, 0x0019_c864)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_SetCategoryTitle(self, int id)` (0x19bbe0): write
/// the category's localised title into the screen's text widget; `id` picks a
/// table entry by hash, with a special case for the "default" category id.
pub fn cfe_state_char_equip_set_category_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p(this as u32, 0x58)).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if no_text {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_e0e0)).lw(c);
            let default_id = c.g[S1][0] != c.g[V0][0];
            c.g[A0][0] = 0x460000;
            if !default_id {
                // the "default" category's own name
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9b0));
                call_ptr(c, 0x0031_7670, 0x0019_bc18)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc24)?;
                frame.restore(c, S0, 0x30);
            } else {
                // look `id` up in the BE entry table and hash its name
                c.cycles += 10;
                call_ptr(c, 0x0014_bdb8, 0x0019_bc34)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xbc)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0014_d448, 0x0019_bc48)?;
                c.g[A0][0] = c.g[V0][0];
                let found = (c.g[A0][0] as i64) >= 0;
                c.g[V0][0] = 0;
                if found {
                    c.cycles += 4;
                    c.g[V0][0] = 0x38;
                    c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c);
                    c.mult(0, 2, 4, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
                call_ptr(c, 0x0031_7670, 0x0019_bc6c)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc78)?;
                c.cycles += 1;
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

/// `void cFEStateCharEquip_UpdateStatBar(self)` (0x19bc90): animate the
/// stat-bar fill (0..1 normalised by the global max) and hand the value to
/// the bar widget.
pub fn cfe_state_char_equip_update_stat_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[A2][0] = Addr(p(this as u32, 0x70)).lw(c);
        let no_bar = c.g[A2][0] == 0;
        c.g[V1][0] = Addr(p(r32(c, GP), 0xffff_e7e8)).lw(c);
        if !no_bar {
            c.cycles += 13;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xc4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(0, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.div_s(20, 1, 0);
            c.c_lt_s(20, 2);
            if c.fcc() {
                c.mov_s(20, 2);
            }
            c.cycles += 19;
            c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_bcfc)?;
            c.neg_s(1, 20);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x70)).lw(c);
            c.swc1(0, p(c.g[SP][0] as u32, 8));
            c.swc1(1, p(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 7));
            c.ldr(3, c.g[SP][0] as u32);
            c.g[A0][0] = Addr(p(c.g[SP][0] as u32, 8)).lw(c);
            c.sdl(3, p(c.g[V0][0] as u32, 0x57));
            c.sdr(3, p(c.g[V0][0] as u32, 0x50));
            c.sw(p(c.g[V0][0] as u32, 0x58), c.g[A0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore_ra(c, 0x10);
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE lists.

/// `int cFEStateCharEquip_RemoveListItem(cFEStateCharEquip* self, int idx)`
/// (0x19b458): drop entry `idx` from the array at_m2 +0x124 (`count` at_m2 +0x958),
/// zero the vacated tail slot and clamp the list view (`top` at_m2 list+0x98,
/// `sel` at_m2 list+0x95). Returns 1, or 0 when `idx` is out of range.
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

// ---------------------------------------------------------------------------
// `BoltFile_CacheShutdown` (0x0019cf40).
//
// `void BoltFile_CacheShutdown(BoltFile* self)`: free every cached slot of
// every bolt file and drop the cache.

pub fn bolt_file_cache_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0xb0000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = 0x1;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_19cf80: loop {
            c.cycles += 4;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = c.g[S0][0];
            'l_19cf90: loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0019cfa0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0019d4b8, 0x0019cfa0)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.g[S0][0];
                if t {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0019cf90);
                    }
                    continue 'l_19cf90;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[S1][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw(c.g[V1][0] as u32, c.g[S6][0]);
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0019cf80);
                        }
                        continue 'l_19cf80;
                    } else {
                        'b_19cfd8: {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                            let t = c.g[A0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            if t {
                                break 'b_19cfd8;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0019cfd4);
                                call_ptr(c, 0x00317e50, 0x0019cfd4)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                break 'b_19cfd8;
                            }
                        }
                        c.cycles += 10;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1788), 0u64);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `cFEStateCharEquip_ShowBuyMode(this)` (0x19a308): relabel the money text
/// and dismiss the three item panels before showing the rider preview.
pub fn cfe_state_char_equip_show_buy_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[V0][0] = Addr(this).off(0x48).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_text {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x810));
            callee::STR_HASH.call(c, 0x0019_a32c)?;
            c.g[A0][0] = Addr(this).off(0x48).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_TEXT_ID.call(c, 0x0019_a338)?;
        }

        // The item panel at_m2 +0x80 (close with 0).
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x80).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a358)?;
        }

        // The item panel at_m2 +0x6c.
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x6c).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = Addr(gp_m2(c, -0x848)).lw(c);
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a37c)?;
            c.g[V0][0] = Addr(gp_m2(c, -0x848)).lw(c);
        }

        // Hide the rider preview, then the panel at_m2 +0x60 (close with 1).
        c.cycles += 10;
        c.g[A1][0] = Addr(this).off(0x44).lb(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FE_MODULE_GET_RIDER_PREVIEW.call(c, 0x0019_a390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a39c)?;
        c.g[A2][0] = Addr(this).off(0x60).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 1;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a3bc)?;
        }

        c.cycles += 4;
        frame.restore(c, 16, 0x10);
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
        'b_19a450: {
            'b_19a44c: {
                'b_19a420: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_19a420;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x0019a3f8);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), 0u64);
                        call_ptr(c, 0x0019a238, 0x0019a3f8)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_19a44c;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019a40c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x810));
                            call_ptr(c, 0x00317670, 0x0019a40c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                            c.g[RA][0] = sx(0x0019a418);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0019a418)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
                            if t {
                                break 'b_19a450;
                            } else {
                                break 'b_19a420;
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0019a42c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                call_ptr(c, 0x0019a308, 0x0019a42c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0x460000;
                if t {
                    break 'b_19a44c;
                } else {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0019a440);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x820));
                    call_ptr(c, 0x00317670, 0x0019a440)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x0019a44c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x0019a44c)?;
                    break 'b_19a44c;
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
            break 'b_19a450;
        }
        c.cycles += 18;
        c.g[A2][0] = 0x50;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x120), 0u64);
        c.g[RA][0] = sx(0x0019a460);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        call_ptr(c, 0x003e6448, 0x0019a460)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
        c.g[RA][0] = sx(0x0019a46c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019bbe0, 0x0019a46c)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
        c.g[RA][0] = sx(0x0019a478);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019b180, 0x0019a478)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0019a488);
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0019b618, 0x0019a488)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
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
        c.g[V0][0] = Addr(p_2(c.g[S0][0], 0x48)).lw(c);
        let no_label = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_label {
            c.cycles += 5;
            c.g[A0][0] = 0x0046_0820;
            call_ptr(c, 0x0031_7670, 0x0019_a25c)?;
            c.g[A0][0] = Addr(p_2(c.g[S0][0], 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0019_a268)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x60)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a288)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x88)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2a8)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x80)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = sx(world(c));
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2cc)?;
            c.g[V0][0] = sx(world(c));
        }
        c.cycles += 13;
        c.g[A1][0] = Addr(p_2(c.g[S0][0], 0x44)).lb(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FEMODULE_GET_RIDER_PREVIEW_2.call(c, 0x0019_a2e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a2ec)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        Addr(p_2(c.g[S0][0], 0xae0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.g[RA][0];
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt cache.

/// `void BoltFile_CacheInit(void* table)` (0x19ce68): clear the ten 256-entry
/// bolt rows and their counts, then create (or reset) the global index
/// manager.
pub fn boltfile_cache_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V0][0] = 0xb0000;
        c.g[T2][0] = c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.g[A2][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T0][0] = sx(p(r32(c, T2), r32(c, V0)));
        c.g[A0][0] = 0;
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 1;
        c.g[A2][0] = c.g[A2][0] | 0x2000;
        loop {
            c.cycles += 6;
            c.mult(0, 2, 4, 6);
            c.g[A1][0] = sx(r32(c, A0) << 2);
            c.g[V1][0] = sx(p(r32(c, V0), r32(c, T2)));
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[A0][0] = 0xff;
            loop {
                c.cycles += 10;
                c.sw(r32(c, V1), 0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                Addr(p(r32(c, V1), 4)).sb(c, 0);
                Addr(p(r32(c, V1), 0x104)).sw(c, 0);
                Addr(p(r32(c, V1), 0x118)).sw(c, 0);
                Addr(p(r32(c, V1), 0x108)).sw(c, 0);
                Addr(p(r32(c, V1), 0x114)).sw(c, 0);
                Addr(p(r32(c, V1), 0x110)).sw(c, c.g[A3][0]);
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx(p(r32(c, V1), 0x120));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_ceb0);
                }
            }
            c.cycles += 5;
            c.g[V1][0] = sx(p(r32(c, T0), r32(c, A1)));
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            Addr(r32(c, V1)).sw(c, c.g[T1][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_ce98);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0xb0000;
        c.g[A0][0] = Addr(gp_m2(c, -0x1788)).lw(c);
        c.g[V0][0] = sx(p(r32(c, T2), r32(c, V0)));
        Addr(p(r32(c, V0), 0x4000)).sw(c, 0);
        let have = c.g[A0][0] != 0;
        Addr(p(r32(c, V0), 0x4004)).sw(c, 0);
        if have {
            c.cycles += 2;
            call_ptr(c, 0x0019_cb60, 0x0019_cf34)?;
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x1f44;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0019_cf1c)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_cb38, 0x0019_cf24)?;
            Addr(gp_m2(c, -0x1788)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(r32(c, SP));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `BoltFile_Release(self, slot, layer)` (0x19dc20): drop one reference on a
/// bolt-file slot's entry; at_m2 zero either clear the slot (when it is not in
/// the loading state) or unpack it and free the slot.
pub fn boltfile_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        let _this = abi::arg(c, 0);
        let _slot = abi::arg(c, 1);
        let _layer = abi::arg(c, 2);
        c.g[A3][0] = 0x1_0000;
        frame.save(c, 18, 0x30);
        c.g[A3][0] |= 0x2000;
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.mult(0, V0, S2, A3); // v0 = slot * 0x12000
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = 0x120;
        frame.save(c, 17, 0x40);
        c.mult(0, V1, S3, V1); // v1 = layer * 0x120
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x50);
        c.sd(p(sp(c), 0x10), c.g[RA][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let entry = c.g[S0][0] as u32;
        c.g[V0][0] = lw_4(c, p(entry, 0x114));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let t = (c.g[V0][0] as i64) > 0;
        c.sw(p(entry, 0x114), c.g[V0][0]);
        let mut skip_one = false;
        if !t {
            c.cycles += 4;
            c.g[V1][0] = lw_4(c, entry);
            c.g[V0][0] = 2;
            let mut go = true;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = lw_4(c, p(entry, 0x104));
            } else {
                c.cycles += 5;
                call_ptr(c, 0x003d_f980, 0x0019_dc88)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] != c.g[V1][0] {
                    skip_one = true;
                    go = false;
                } else {
                    c.cycles += 16;
                    c.g[A0][0] = lw_4(c, p(entry, 0x108));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0019_dca4);
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0019_dca4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcb4);
                    c.g[A2][0] = c.g[S0][0];
                    c.pc = 0x0019_d000;
                    ee::s03::BoltFile_UnpackSlot(c);
                    if c.pc != 0x0019_dcb4 {
                        return Err(Unwound);
                    }
                    c.sw(p(c.g[S0][0] as u32, 0x10c), 0);
                    c.g[V1][0] = 0xb_0000;
                    c.sw(c.g[S0][0] as u32, 0);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x4004));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.sw(p(c.g[V1][0] as u32, 0x4004), c.g[V0][0]);
                    c.g[V0][0] = lw_4(c, p(c.g[S0][0] as u32, 0x104));
                }
            }
            if go {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(c.g[S0][0] as u32, 0);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcec);
                    c.g[A2][0] = c.g[S3][0];
                    c.pc = 0x0019_d4b8;
                    ee::s03::BoltFile_FreeSlot(c);
                    if c.pc != 0x0019_dcec {
                        return Err(Unwound);
                    }
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BoltFileIndexManager_FindNode (0x19c938)

/// `BoltFileIndexManager`: 400 nodes of 16 bytes (key +0, prev index +0xc,
/// next index +0xe), 400 bucket heads at +0x1900, the free list at +0x1f40.
pub(crate) const BOLT_INDEX_BUCKETS: u32 = 0x1900;

pub(crate) const BOLT_INDEX_FREE: u32 = 0x1f40;

pub(crate) const BOLT_INDEX_ALLOC_NODE: Callee = Callee {
    addr: 0x0019_c880,
    f: ee::s03::BoltFileIndexManager_AllocNode,
};

pub(crate) const BOLT_INDEX_NODE_INDEX: Callee = Callee {
    addr: 0x0019_c920,
    f: ee::s03::BoltFileIndexManager_NodeIndex,
};

pub(crate) const BOLT_INDEX_GET_NEXT_NODE: Callee = Callee {
    addr: 0x0019_c8f8,
    f: ee::s03::BoltFileIndexManager_GetNextNode,
};

/// `Node* BoltFileIndexManager_FindNode(BoltFileIndexManager* m, int hi,
/// int lo, bool create)` (0x19c938): the node of key `hi << 16 | lo` in
/// bucket `key % 400`; if absent and `create` (and a free node exists), a
/// new node appended to the bucket's chain. The found or new node's key is
/// (re)written; returns it or null.
pub fn bolt_file_index_manager_find_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 16);
        frame.save(c, S4, 0x20);
        c.g[V0][0] = 400;
        c.g[S4][0] = c.g[A1][0] | c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.div(0, S4, V0);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A3][0];
        // The bucket's head.
        c.cycles += 6;
        let m = c.g[S1][0] as u32;
        let free = Addr(m + BOLT_INDEX_FREE).lw(c);
        c.g[V1][0] = free;
        c.g[S3][0] = c.hi[0];
        let bucket = m.wrapping_add((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx(bucket);
        c.g[S0][0] = Addr(bucket.wrapping_add(BOLT_INDEX_BUCKETS)).lw(c);
        let mut searching = true;
        if free == 0 {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.g[V0][0] = 0;
                searching = false;
            } else {
                c.cycles += 1;
            }
        }
        if searching {
            'walk: loop {
                // At a node (or past the chain's end).
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.cycles += 2;
                    if c.g[S5][0] != 0 {
                        bolt_index_append_node(c)?;
                    }
                    break 'walk;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                loop {
                    c.cycles += 2;
                    let found = c.g[V0][0] == c.g[S4][0];
                    c.g[S2][0] = c.g[S0][0];
                    if found {
                        break 'walk;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    BOLT_INDEX_GET_NEXT_NODE.call(c, 0x0019_ca28)?;
                    c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BOLT_INDEX_FREE)).lw(c);
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[V1][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_c9a0);
                        }
                        continue 'walk;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        break 'walk;
                    }
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_ca14);
                    }
                }
            }
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Take a free node and link it after s2 (the chain's last node), or as
/// the bucket's head (bucket s3) when the chain is empty; s0 = the node.
pub(crate) fn bolt_index_append_node(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 8;
    c.g[A0][0] = c.g[S1][0];
    BOLT_INDEX_ALLOC_NODE.call(c, 0x0019_c9b8)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    BOLT_INDEX_NODE_INDEX.call(c, 0x0019_c9c8)?;
    Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sh(c, c.g[V0][0]);
    if c.g[S2][0] != 0 {
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        BOLT_INDEX_NODE_INDEX.call(c, 0x0019_c9dc)?;
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sh(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        BOLT_INDEX_NODE_INDEX.call(c, 0x0019_c9ec)?;
        Addr((c.g[S2][0] as u32).wrapping_add(0xe)).sh(c, c.g[V0][0]);
    } else {
        c.cycles += 5;
        let bucket = (c.g[S1][0] as u32).wrapping_add((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx(bucket);
        c.g[V1][0] = u64::MAX;
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sh(c, u64::MAX);
        Addr(bucket.wrapping_add(BOLT_INDEX_BUCKETS)).sw(c, c.g[S0][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    Addr((c.g[S0][0] as u32).wrapping_add(0xe)).sh(c, u64::MAX);
    Ok(())
}

// ---------------------------------------------------------------------------
// BoltFile_CacheUpdate (0x19d250)

/// The bolt file cache: 10 tables of 256 slots (0x120 bytes each, a table
/// every 0x12000 bytes); `+0xb4004` the number of pending loads.
pub(crate) const BOLT_CACHE_TABLES: u32 = 10;

#[cfg(test)]
pub(crate) const BOLT_CACHE_TABLE_SIZE: u32 = 0x12000;

pub(crate) const BOLT_CACHE_SLOT_SIZE: u32 = 0x120;

#[cfg(test)]
pub(crate) const BOLT_CACHE_PENDING: u32 = 0xb_4004;

/// Slot fields: state (2 = loading, 3 = queued), the async file handle,
/// the pending flag and the keep timer.
pub(crate) const BOLT_SLOT_STATE: u32 = 0x0;

pub(crate) const BOLT_SLOT_HANDLE: u32 = 0x108;

pub(crate) const BOLT_SLOT_PENDING: u32 = 0x10c;

pub(crate) const BOLT_SLOT_TIMER: u32 = 0x114;

pub(crate) const BOLT_UNPACK_SLOT: Callee = Callee {
    addr: 0x0019_d000,
    f: ee::s03::BoltFile_UnpackSlot,
};

pub(crate) const BOLT_START_NEXT_QUEUED: Callee = Callee {
    addr: 0x0019_d140,
    f: ee::s03::BoltFile_StartNextQueued,
};

pub(crate) const BOLT_LOAD_SLOT: Callee = Callee {
    addr: 0x0019_d8b8,
    f: ee::s03::BoltFile_LoadSlot,
};

/// `void BoltFile_CacheUpdate(BoltFileCache* self)` (0x19d250): poll every
/// slot of every table: a pending load whose async read finished
/// (`AsyncFile_IsDone == 1`) is released and unpacked, its state becomes 1
/// (or 0 without a keep timer) and, when it was the last pending load, the
/// next queued one starts; with no load pending, a queued slot (state 3)
/// starts loading.
pub fn bolt_file_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0xb_0000;
        frame.save(c, S2, 0x80);
        frame.save(c, S7, 0x30);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x70);
        c.g[S7][0] = 0x1_0000;
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0; // table
        frame.save(c, FP, 0x20);
        c.g[S4][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        frame.save(c, S0, 0xa0);
        c.g[S7][0] |= 0x2000; // BOLT_CACHE_TABLE_SIZE
        frame.save(c, S1, 0x90);
        c.g[FP][0] = 3; // queued
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x10);
        c.mult(0, V0, S3, S7);
        loop {
            c.cycles += 4;
            c.g[S5][0] = 0; // slot
            c.g[S6][0] = sx(r32(c, S3).wrapping_add(1));
            c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
            loop {
                bolt_file_cache_update_slot(c)?;
                // Next slot.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S5][0] as i64) < 256) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(BOLT_CACHE_SLOT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_d2b0);
                }
            }
            // Next table.
            c.cycles += 4;
            c.g[S3][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[S3][0] as i64) < BOLT_CACHE_TABLES as i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.mult(0, V0, S3, S7);
            if event_due(c) {
                deliver_event_here(c, 0x0019_d2a0);
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One slot of [`bolt_file_cache_update`] (s0 = the slot, s5 = its index,
/// incremented here).
pub(crate) fn bolt_file_cache_update_slot(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_PENDING)).lw(c);
    c.g[V1][0] = 2;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_STATE)).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            // Loading: has the read finished?
            c.cycles += 6;
            c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_HANDLE)).lw(c);
            call_ptr(c, 0x003d_f980, 0x0019_d2d0)?; // AsyncFile_IsDone
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            if c.g[S1][0] != c.g[V0][0] {
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                return Ok(());
            }
            c.cycles += 11;
            c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_HANDLE)).lw(c);
            c.g[A2][0] = c.g[SP][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x003d_f9d8, 0x0019_d2f0)?; // AsyncFile release
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            BOLT_UNPACK_SLOT.call(c, 0x0019_d300)?;
            c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_TIMER)).lw(c);
            if c.g[V0][0] != 0 {
                Addr(r32(c, S0)).sw(c, c.g[S1][0]);
            } else {
                c.cycles += 1;
                Addr(r32(c, S0)).sw(c, 0);
            }
            c.cycles += 5;
            Addr(oc391_at(c, S0, BOLT_SLOT_PENDING)).sw(c, 0);
            c.g[V0][0] = Addr(oc391_at(c, S4, 0x4004)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let more = (c.g[V0][0] as i64) > 0;
            Addr(oc391_at(c, S4, 0x4004)).sw(c, c.g[V0][0]);
            if more {
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                BOLT_START_NEXT_QUEUED.call(c, 0x0019_d330)?;
            }
            c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
            return Ok(());
        }
        c.g[V0][0] = Addr(oc391_at(c, S4, 0x4004)).lw(c);
    } else {
        c.cycles += 1;
        c.g[V0][0] = Addr(oc391_at(c, S4, 0x4004)).lw(c);
    }
    // Nothing pending anywhere: start a queued slot.
    c.cycles += 2;
    if (c.g[V0][0] as i64) <= 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_STATE)).lw(c);
        if c.g[V0][0] == c.g[FP][0] {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            BOLT_LOAD_SLOT.call(c, 0x0019_d360)?;
            c.cycles += 1;
        }
    }
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
    Ok(())
}

// ---------------------------------------------------------------------------
// BoltFile_FreeSlot (0x19d4b8)

/// Slot fields freed by [`bolt_file_free_slot`]: the data block, the name's
/// first byte, the cache index (-1 = none) and the data size.
pub(crate) const BOLT_SLOT_DATA: u32 = 0x104;

pub(crate) const BOLT_SLOT_NAME: u32 = 0x4;

pub(crate) const BOLT_SLOT_INDEX: u32 = 0x110;

pub(crate) const BOLT_SLOT_SIZE: u32 = 0x118;

/// `BoltFileCache+0xb4000`: the bytes in use.
#[cfg(test)]
pub(crate) const BOLT_CACHE_BYTES: u32 = 0xb_4000;

/// `void BoltFile_FreeSlot(BoltFileCache* self, int table, int slot)`
/// (0x19d4b8): cancel a slot's pending read (state 2), take its size off
/// the cache's byte count, free its data and clear it.
pub fn bolt_file_free_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = BOLT_CACHE_SLOT_SIZE as u64;
        c.g[V1][0] = 0x1_0000;
        c.mult(0, A2, A2, V0); // slot * 0x120
        c.g[V1][0] |= 0x2000;
        c.mult(0, V0, A1, V1); // table * 0x12000
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 2;
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.g[S0][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
        c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_STATE)).lw(c);
        let loading = c.g[V0][0] == c.g[A0][0];
        c.g[V0][0] = 0xb_0000;
        if loading {
            c.cycles += 3;
            c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_PENDING)).lw(c);
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_HANDLE)).lw(c);
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x003d_f9d8, 0x0019_d510)?; // AsyncFile release
            }
            c.cycles += 1;
            c.g[V0][0] = 0xb_0000;
        }
        c.cycles += 8;
        c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_SIZE)).lw(c);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(oc391_at(c, V0, 0x4000)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)));
        Addr(oc391_at(c, V0, 0x4000)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_DATA)).lw(c);
        c.g[V0][0] = u64::MAX;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0019_d53c)?; // EA_MemFree
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 13;
        let slot = r32(c, S0);
        Addr(slot + BOLT_SLOT_DATA).sw(c, 0);
        Addr(slot + BOLT_SLOT_INDEX).sw(c, c.g[V0][0]);
        Addr(slot + BOLT_SLOT_SIZE).sw(c, 0);
        Addr(slot + BOLT_SLOT_STATE).sw(c, 0);
        Addr(slot + BOLT_SLOT_NAME).sb(c, 0);
        Addr(slot + BOLT_SLOT_HANDLE).sw(c, 0);
        Addr(slot + BOLT_SLOT_PENDING).sw(c, 0);
        Addr(slot + BOLT_SLOT_TIMER).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Result of the compare passes of `BoltFile_CompareModelSlots`: where the
/// function leaves to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoltCompareExit {
    /// Second pass over the records (their words at +0 and +0x110/+0x11c).
    Second,
    /// Early return (1 cycle, then the epilogue).
    Early,
    /// Late return (2 cycles, then the epilogue).
    Late,
    /// Compare the two names (`+4`) with `strcasecmp`, then return.
    Names,
}

/// First pass of `BoltFile_CompareModelSlots`: the record at `a0` (the
/// computed slot) and the one at `a1` must match in their words at +0 (3 for
/// `a0`), +0x110 (8) and +0x11c (0).
pub(crate) fn bcms_first_pass(c: &mut Cpu) -> BoltCompareExit {
    if c.g[V1][0] != c.g[A3][0] {
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        return BoltCompareExit::Second;
    }
    c.cycles += 4;
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x110)).lw(c);
    c.g[V0][0] = 8;
    if c.g[A2][0] != c.g[V0][0] {
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        return BoltCompareExit::Second;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x11c)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        return BoltCompareExit::Second;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    let t = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = u64::MAX;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x110)).lw(c);
    let t = c.g[V0][0] != c.g[A2][0];
    c.g[V0][0] = u64::MAX;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x11c)).lw(c);
    let t = c.g[V0][0] == 0;
    c.g[V0][0] = u64::MAX;
    if t {
        return BoltCompareExit::Names;
    }
    BoltCompareExit::Late
}

/// Second pass of `BoltFile_CompareModelSlots`: the same words again, with
/// `a2` (the word at +0 of `a1`) as the expected value of `a0`'s word at +0.
pub(crate) fn bcms_second_pass(c: &mut Cpu) -> BoltCompareExit {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let t = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 4;
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x110)).lw(c);
    c.g[V0][0] = 8;
    let t = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x11c)).lw(c);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    let t = c.g[V0][0] != c.g[A2][0];
    c.g[V0][0] = 1;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x110)).lw(c);
    let t = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = 1;
    if t {
        return BoltCompareExit::Early;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x11c)).lw(c);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if t {
        return BoltCompareExit::Early;
    }
    BoltCompareExit::Names
}

/// `BoltFile_CompareModelSlots` (0x19dd10): finds the slot of a model in a
/// bolt file table and compares it with the slot named by the second
/// argument. `a0` points at a word whose high half selects a row (times
/// `0x12000`) and whose low half selects a slot (times `0x120`, the slot
/// record size); `a1` points at a 16-bit slot index. The two records are
/// compared on their type words (3 and 8) and their name field at +0x11c
/// (0); equal records compare their names (`+4`) with `strcasecmp`. Returns
/// the `strcasecmp` result, or 0/1/-1 for the early outs.
pub fn bolt_file_compare_model_slots_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x12000;
        c.g[T0][0] = 0x120;
        c.g[A3][0] = 3;
        c.g[V1][0] = Addr(crate::abi::arg(c, 0)).lw(c);
        c.g[A2][0] = Addr(crate::abi::arg(c, 1)).lhu(c);
        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
        c.mult(0, A0, A0, V0);
        c.g[V1][0] &= 0xffff;
        c.mult(0, V1, V1, T0);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1784)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.mult(0, V1, A2, T0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);

        let exit = match bcms_first_pass(c) {
            BoltCompareExit::Second => bcms_second_pass(c),
            other => other,
        };
        match exit {
            BoltCompareExit::Early => {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
            BoltCompareExit::Late => {
                c.cycles += 2;
                frame.restore_ra(c, 0);
            }
            BoltCompareExit::Names => {
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                call_ptr(c, 0x0041_aa88, 0x0019_de00)?;
                frame.restore_ra(c, 0);
            }
            BoltCompareExit::Second => {}
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `BoltFile_UnpackSlot` (0x19d000).
pub(crate) const BOLT_UNPACK_SLOT_2: u32 = 0x0019_d000;

/// `void BoltFile_Release(BoltFile* f, int a1, int a2)` (0x19dc20): drops one
/// use of a 0x120-byte slot of the bolt file table. The slot is at
/// `f + a1 * 0x12000 + a2 * 0x120`; its use count is at +0x114. When the count
/// stays positive the release ends there. Otherwise a slot whose state word
/// (+0) is 2 is unpacked (or its state is cleared when the callee says so),
/// and any other slot is freed with `BoltFile_FreeSlot`.
pub fn bolt_file_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x10000;
        frame.save(c, S2, 0x30);
        c.g[A3][0] = c.g[A3][0] | 0x2000;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.mult(0, 2, 18, 7);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = 0x120;
        frame.save(c, S1, 0x40);
        c.mult(0, 3, 19, 3);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x10);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));

        let plus_one = bolt_file_release_slot(c)?;
        if plus_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        c.cycles += 6;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The slot part of `BoltFile_Release`, with `s0` the slot. Returns true when
/// the generated code takes its extra `+1` cycle block on the way out; false
/// after an early exit (the state already decided by a callee).
pub(crate) fn bolt_file_release_slot(c: &mut Cpu) -> Result<bool, Unwound> {
    let slot = c.g[S0][0] as u32;
    c.g[V0][0] = Addr(slot.wrapping_add(0x114)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let uses = c.g[V0][0];
    Addr(slot.wrapping_add(0x114)).sw(c, uses);
    if (uses as i64) > 0 {
        return Ok(true);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(slot).lw(c);
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = Addr(slot.wrapping_add(0x104)).lw(c);
    } else {
        c.cycles += 5;
        c.g[A0][0] = Addr(slot.wrapping_add(0x108)).lw(c);
        call_ptr(c, 0x003d_f980, 0x0019_dc88)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] != c.g[V1][0] {
            return Ok(false);
        }
        c.cycles += 16;
        c.g[A0][0] = Addr(slot.wrapping_add(0x108)).lw(c);
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003d_f9d8, 0x0019_dca4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        Callee {
            addr: BOLT_UNPACK_SLOT_2,
            f: ee::s03::f_0019d000,
        }
        .call(c, 0x0019_dcb4)?;
        Addr(slot.wrapping_add(0x10c)).sw(c, 0);
        c.g[V1][0] = 0x000b_0000;
        Addr(slot).sw(c, 0);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let table = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(table.wrapping_add(0x4004)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let count = c.g[V0][0];
        Addr(table.wrapping_add(0x4004)).sw(c, count);
        c.g[V0][0] = Addr(slot.wrapping_add(0x104)).lw(c);
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        Addr(slot).sw(c, 0);
        return Ok(true);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    Callee {
        addr: BOLT_FREE_SLOT,
        f: ee::s03::f_0019d4b8,
    }
    .call(c, 0x0019_dcec)?;
    Ok(false)
}

/// `BoltFile_EnsureBudget(mgr, bank, size)` (0x19d578): keeps the bolt file
/// cache under its 9999999-byte budget before `size` more bytes are loaded:
/// when `mem_used + size` is over, frees unreferenced idle slots of the other
/// banks first, then the oldest (smallest stamp) idle slots of `bank` until
/// the budget holds or none is left. Slots are 0x120 bytes, 256 per bank, 10
/// banks 0x12000 apart.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn bolt_file_ensure_budget(c: &mut Cpu) {
    guest(c, |c| {
        'b_19d704: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[V0][0] = 0xb0000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V1][0] = 0x980000;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S4][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S7][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = c.g[V1][0] | 0x967f;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4000));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S7][0] as u32));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V1][0] == 0u64;
            c.g[FP][0] = c.g[A1][0];
            if t {
                break 'b_19d704;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0u64;
                c.g[S6][0] = 0u64;
                'l_19d5d8: loop {
                    'b_19d64c: {
                        c.cycles += 2;
                        let t = c.g[S3][0] == c.g[FP][0];
                        c.g[V0][0] = 0xb0000;
                        if t {
                            break 'b_19d64c;
                        } else {
                            c.cycles += 6;
                            c.g[S1][0] = 0x980000;
                            c.g[S2][0] = 0u64;
                            c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[S1][0] = c.g[S1][0] | 0x967f;
                            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            'l_19d5f8: loop {
                                'b_19d640: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x104));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        break 'b_19d640;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                            break 'b_19d640;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                break 'b_19d640;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A1][0] = c.g[S3][0];
                                                c.g[A2][0] = c.g[S2][0];
                                                call_ptr(c, 0x0019d4b8, 0x0019d62c)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S5][0] as u32).wrapping_add(0x4000));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.g[V0][0] = ((c.g[S1][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_19d704;
                                                } else {
                                                    break 'b_19d640;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[S2][0] as i64) < 256i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019d5f8);
                                    }
                                    continue 'l_19d5f8;
                                } else {
                                    break 'b_19d64c;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V1][0] = 0x10000;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.g[V1][0] | 0x2000;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019d5d8);
                        }
                        continue 'l_19d5d8;
                    } else {
                        c.cycles += 7;
                        c.mult(0, 18, 30, 3);
                        c.g[V0][0] = 0xb0000;
                        c.g[S0][0] = 0x980000;
                        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S0][0] = c.g[S0][0] | 0x967f;
                        c.g[A2][0] = 0xffffffffffffffff;
                        'l_19d680: loop {
                            c.cycles += 4;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            'l_19d690: loop {
                                'b_19d6d4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x104));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        break 'b_19d6d4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_19d6d4;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x114));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                break 'b_19d6d4;
                                            } else {
                                                'b_19d6c8: {
                                                    c.cycles += 2;
                                                    let t = (c.g[A2][0] as i64) < 0;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x11c)
                                                        );
                                                    if t {
                                                        break 'b_19d6c8;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[V1][0] < c.g[A3][0]) as u64;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_19d6d4;
                                                        } else {
                                                            break 'b_19d6c8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A3][0] = c.g[V1][0];
                                                c.g[A2][0] = c.g[A1][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                break 'b_19d6d4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A1][0] as i64) < 256i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x120));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019d690);
                                    }
                                    continue 'l_19d690;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[A2][0] as i64) < 0;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_19d704;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.g[FP][0];
                                        call_ptr(c, 0x0019d4b8, 0x0019d6f0)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4000));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                                        c.g[V0][0] =
                                            ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0019d680);
                                            }
                                            continue 'l_19d680;
                                        } else {
                                            break 'b_19d704;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    });
}

/// `BoltFile_Acquire(mgr, bank, name, file_type, async, deferred)` (0x19da10):
/// finds the cached slot of `name` in `bank` (or takes a free one, freeing it
/// first and copying the name), stamps it (0 when deferred, else the bank's
/// running counter), bumps its use count and starts or schedules the load
/// (async loads wait as state 3 when load threads exist). Returns the slot
/// index.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn bolt_file_acquire(c: &mut Cpu) {
    guest(c, |c| {
        'b_19dbf4: {
            'b_19dbf0: {
                'b_19db28: {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S6][0] = c.g[A2][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S4][0] = c.g[T1][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S5][0] = c.g[T0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S7][0] = c.g[A3][0];
                    call_ptr(c, 0x0019d428, 0x0019da54)?;
                    c.g[S3][0] = c.g[V0][0];
                    let t = (c.g[S3][0] as i64) < 0;
                    c.g[V0][0] = 0x10000;
                    if t {
                        break 'b_19db28;
                    } else {
                        'b_19daa8: {
                            'b_19da84: {
                                c.cycles += 7;
                                c.g[V1][0] = 0x120;
                                c.g[V0][0] = c.g[V0][0] | 0x2000;
                                c.mult(0, 3, 19, 3);
                                c.mult(0, 4, 18, 2);
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                let t = c.g[S4][0] == 0u64;
                                c.g[S0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                if t {
                                    break 'b_19da84;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                                    if t {
                                        break 'b_19daa8;
                                    } else {
                                        break 'b_19da84;
                                    }
                                }
                            }
                            c.cycles += 9;
                            c.g[V0][0] = 0xb0000;
                            c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                            c.g[V0][0] = c.g[V0][0] | 0x4008;
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                            break 'b_19daa8;
                        }
                        'b_19db1c: {
                            'b_19db18: {
                                'b_19dac8: {
                                    c.cycles += 2;
                                    if c.g[S5][0] == 0u64 {
                                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                        break 'b_19dac8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                                            break 'b_19db1c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x1;
                                            let t = true;
                                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                            if t {
                                                break 'b_19db18;
                                            } else {
                                                break 'b_19dac8;
                                            }
                                        }
                                    }
                                }
                                'b_19db0c: {
                                    'b_19daec: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10c), 0u64);
                                        if t {
                                            break 'b_19daec;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x108));
                                            c.g[A1][0] = 0u64;
                                            c.g[A2][0] = c.g[SP][0];
                                            call_ptr(c, 0x003df9d8, 0x0019dae4)?;
                                            let t = true;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_19db0c;
                                            } else {
                                                break 'b_19daec;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19db0c;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A2][0] = c.g[S3][0];
                                        call_ptr(c, 0x0019d8b8, 0x0019db08)?;
                                        c.g[V0][0] = 0x1;
                                        break 'b_19db0c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V1][0] = 0xffffffffffffffff;
                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x108), c.g[V1][0]);
                                break 'b_19db18;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                            break 'b_19db1c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x114), c.g[V0][0]);
                        if t {
                            break 'b_19dbf0;
                        } else {
                            break 'b_19db28;
                        }
                    }
                }
                'b_19db94: {
                    'b_19db74: {
                        c.cycles += 17;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0019d3b0, 0x0019db34)?;
                        c.g[V1][0] = 0x10000;
                        c.g[S3][0] = c.g[V0][0];
                        c.g[V1][0] = c.g[V1][0] | 0x2000;
                        c.g[V0][0] = 0x120;
                        c.mult(0, 4, 18, 3);
                        c.mult(0, 2, 19, 2);
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0019d4b8, 0x0019db64)?;
                        let t = c.g[S4][0] == 0u64;
                        c.g[V0][0] = 0xb0000;
                        if t {
                            break 'b_19db74;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                            if t {
                                break 'b_19db94;
                            } else {
                                break 'b_19db74;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = c.g[V0][0] | 0x4008;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    break 'b_19db94;
                }
                'b_19dbec: {
                    'b_19dbe8: {
                        c.cycles += 9;
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x004166f4, 0x0019dba0)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x110), c.g[S7][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10c), c.g[S5][0]);
                        let t = c.g[S4][0] != 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x114), c.g[V1][0]);
                        if t {
                            break 'b_19dbe8;
                        } else {
                            'b_19dbd0: {
                                c.cycles += 2;
                                let t = c.g[S5][0] == 0u64;
                                c.g[V0][0] = 0xb0000;
                                if t {
                                    break 'b_19dbd0;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4004));
                                    let t = (c.g[V1][0] as i64) > 0;
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_19dbec;
                                    } else {
                                        break 'b_19dbd0;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x0019d8b8, 0x0019dbe0)?;
                            let t = true;
                            c.g[V0][0] = c.g[S3][0];
                            if t {
                                break 'b_19dbf4;
                            } else {
                                break 'b_19dbe8;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x3;
                    break 'b_19dbec;
                }
                c.cycles += 1;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                break 'b_19dbf0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            break 'b_19dbf4;
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    });
}

/// `void BoltFile_UnpackSlot(BoltTable* self, int bank, BoltSlot* slot)`
/// (0x19d000): replaces the compressed data of a loaded slot (`+0x104`) by
/// its unpacked copy: asks for the unpacked size, makes room
/// (`BoltFile_EnsureBudget`), allocates the buffer (evicting and retrying
/// while the allocator fails), unpacks, frees the old data and books the size
/// difference in the table's memory counter (`+0xb4000`).
pub fn bolt_file_unpack_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_19d118: {
            'b_19d114: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S4][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S5][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S2][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_19d114;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                    let t = c.g[A0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    if t {
                        break 'b_19d118;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x0019d048);
                        call_ptr(c, 0x003b4818, 0x0019d048)?;
                        c.g[S1][0] = c.g[V0][0];
                        let t = (c.g[S1][0] as i64) <= 0;
                        c.g[A0][0] = c.g[S5][0];
                        if t {
                            break 'b_19d114;
                        } else {
                            'b_19d0d4: {
                                c.cycles += 13;
                                c.g[A1][0] = c.g[S4][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x0019d064);
                                c.g[S0][0] = 0x460000;
                                Callee {
                                    addr: 0x0019d578,
                                    f: ee::s03::BoltFile_EnsureBudget,
                                }
                                .call(c, 0x0019d064)?;
                                c.g[S6][0] = c.g[S0][0];
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9d8));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = 0x60000000;
                                c.g[RA][0] = sx(0x0019d07c);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317e30, 0x0019d07c)?;
                                c.g[S0][0] = c.g[V0][0];
                                if c.g[S0][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                                    break 'b_19d0d4;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = 0x540000;
                                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7500));
                                    'l_19d090: loop {
                                        c.cycles += 16;
                                        c.g[RA][0] = sx(0x0019d098);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x003e5700, 0x0019d098)?;
                                        c.g[RA][0] = sx(0x0019d0a0);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x003e5760, 0x0019d0a0)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[RA][0] = sx(0x0019d0b0);
                                        c.g[A2][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x0019d738,
                                            f: ee::s03::BoltFile_EvictBytes,
                                        }
                                        .call(c, 0x0019d0b0)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x9d8));
                                        c.g[A2][0] = 0x60000000;
                                        c.g[RA][0] = sx(0x0019d0c4);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317e30, 0x0019d0c4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;

                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0019d090);
                                            }
                                            continue 'l_19d090;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                                            break 'b_19d0d4;
                                        }
                                    }
                                }
                            }
                            'b_19d0f4: {
                                c.cycles += 5;
                                c.g[RA][0] = sx(0x0019d0dc);
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, 0x003b47f8, 0x0019d0dc)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                                if c.g[A0][0] == 0u64 {
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[S0][0]);
                                    break 'b_19d0f4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0019d0f0);
                                    call_ptr(c, 0x00317e98, 0x0019d0f0)?;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[S0][0]);
                                    break 'b_19d0f4;
                                }
                            }
                            c.cycles += 8;
                            c.g[V0][0] = 0xb0000;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4000));
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4000), c.g[A0][0]);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[S1][0]);
                            break 'b_19d114;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_19d118;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `void BoltFile_StartNextQueued(BoltTable* self, int bank)` (0x19d140):
/// picks the queued slot (flags 3) with the oldest stamp (`+0x11c`) in
/// `bank`, else in the first other bank that has one, and loads it
/// (`BoltFile_LoadSlot`).
pub fn bolt_file_start_next_queued(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x10000;
        c.g[T6][0] = c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.mult(0, 3, 14, 2);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T2][0] = 0u64;
        c.g[A2][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0x3;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        'l_19d170: loop {
            'b_19d19c: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                if c.g[V0][0] != c.g[T1][0] {
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    break 'b_19d19c;
                } else {
                    'b_19d190: {
                        c.cycles += 2;
                        let t = c.g[T2][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x11c));
                        if t {
                            break 'b_19d190;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < c.g[T2][0]) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                break 'b_19d19c;
                            } else {
                                break 'b_19d190;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[T2][0] = c.g[V1][0];
                    c.g[A2][0] = c.g[T0][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    break 'b_19d19c;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < 256i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x120));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0019d170);
                }
                continue 'l_19d170;
            } else {
                'b_19d244: {
                    'b_19d238: {
                        'b_19d230: {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[A2][0] != c.g[V0][0];

                            if t {
                                break 'b_19d230;
                            } else {
                                c.cycles += 5;
                                c.g[T3][0] = 0x10000;
                                c.g[T1][0] = 0u64;
                                c.g[T5][0] = 0u64;
                                c.g[T3][0] = c.g[T3][0] | 0x2000;
                                'l_19d1c8: loop {
                                    'b_19d224: {
                                        c.cycles += 2;
                                        if c.g[T1][0] == c.g[T6][0] {
                                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                            break 'b_19d224;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A3][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[T5][0] as u32));
                                            c.g[T0][0] = 0u64;
                                            c.g[T4][0] = 0x3;
                                            'l_19d1e0: loop {
                                                'b_19d210: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                    if c.g[V0][0] != c.g[T4][0] {
                                                        c.g[T0][0] =
                                                            sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_19d210;
                                                    } else {
                                                        'b_19d200: {
                                                            c.cycles += 2;
                                                            let t = c.g[T2][0] == 0u64;
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x11c));
                                                            if t {
                                                                break 'b_19d200;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = (c.g[V1][0]
                                                                    < c.g[T2][0])
                                                                    as u64;
                                                                if c.g[V0][0] == 0u64 {
                                                                    c.g[T0][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    break 'b_19d210;
                                                                } else {
                                                                    break 'b_19d200;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[T2][0] = c.g[V1][0];
                                                        c.g[A1][0] = c.g[T1][0];
                                                        c.g[A2][0] = c.g[T0][0];
                                                        c.g[T0][0] =
                                                            sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_19d210;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[T0][0] as i64) < 256i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x120));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0019d1e0);
                                                    }
                                                    continue 'l_19d1e0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = (c.g[A2][0] as i64) >= 0;
                                                    c.g[T1][0] =
                                                        sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_19d238;
                                                    } else {
                                                        break 'b_19d224;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[T1][0] as i64) < 10i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[T5][0] =
                                        sx((c.g[T5][0] as u32).wrapping_add(c.g[T3][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0019d1c8);
                                        }
                                        continue 'l_19d1c8;
                                    } else {
                                        break 'b_19d230;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = (c.g[A2][0] as i64) < 0;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        if t {
                            break 'b_19d244;
                        } else {
                            break 'b_19d238;
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019d240);
                    Callee {
                        addr: 0x0019d8b8,
                        f: ee::s03::BoltFile_LoadSlot,
                    }
                    .call(c, 0x0019d240)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    break 'b_19d244;
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void BoltFile_LoadSlot(BoltTable* self, int bank, int idx)` (0x19d8b8):
/// asks the file's size, makes room, allocates the data buffer (evicting and
/// retrying while the allocator fails), books the size, and loads the file:
/// synchronously (flags 1) or as an asynchronous read (flags 2, counted in
/// `+0xb4004`).
pub fn bolt_file_load_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_19d984: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = 0x10000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.g[V0][0] | 0x2000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[A0][0];
            c.g[S4][0] = c.g[A1][0];
            c.g[V1][0] = 0x120;
            c.mult(0, 4, 20, 2);
            c.mult(0, 6, 6, 3);
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = 0x460000;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x0019d90c);
            c.g[S5][0] = c.g[S0][0];
            call_ptr(c, 0x00317f60, 0x0019d90c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0019d920);
            c.g[A2][0] = c.g[S1][0];
            Callee {
                addr: 0x0019d578,
                f: ee::s03::BoltFile_EnsureBudget,
            }
            .call(c, 0x0019d920)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9f0));
            c.g[A2][0] = 0x60000000;
            c.g[RA][0] = sx(0x0019d934);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0019d934)?;
            let t = c.g[V0][0] != 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[V0][0]);
            if t {
                break 'b_19d984;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x540000;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7500));
                'l_19d948: loop {
                    c.cycles += 15;
                    c.g[RA][0] = sx(0x0019d950);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e5700, 0x0019d950)?;
                    c.g[RA][0] = sx(0x0019d958);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e5760, 0x0019d958)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0019d968);
                    c.g[A2][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0019d738,
                        f: ee::s03::BoltFile_EvictBytes,
                    }
                    .call(c, 0x0019d968)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x9f0));
                    c.g[A2][0] = 0x60000000;
                    c.g[RA][0] = sx(0x0019d97c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0019d97c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019d948);
                        }
                        continue 'l_19d948;
                    } else {
                        break 'b_19d984;
                    }
                }
            }
        }
        'b_19d9ec: {
            'b_19d9d0: {
                c.cycles += 8;
                c.g[V0][0] = 0xb0000;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4000));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4000), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10c));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x2;
                if t {
                    break 'b_19d9d0;
                } else {
                    c.cycles += 11;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[S1][0]);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                    c.g[A2][0] = c.g[S1][0];
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[RA][0] = sx(0x0019d9bc);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x003df748, 0x0019d9bc)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4004));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4004), c.g[V0][0]);
                    if t {
                        break 'b_19d9ec;
                    } else {
                        break 'b_19d9d0;
                    }
                }
            }
            c.cycles += 7;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[S1][0]);
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
            c.g[RA][0] = sx(0x0019d9e4);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x003e1bb8, 0x0019d9e4)?;
            c.g[V1][0] = 0x1;
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
            break 'b_19d9ec;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `FEModule::RiderPreviewRequestChar(self, char_id, load_all)` (0x19e588):
/// records the requested character (`+0xc24`) and load-all flag (`+0xc28`).
/// If the preview already shows that character with a matching load-all
/// state (`+0x0`, `+0x10`) and its model is compiled (`+0xcb8`, `+0xcb4`),
/// the visibility fields are updated (`+0xcd4` = -1, `+0xcc8`/`+0xcc4`);
/// otherwise, when no render is pending (`+0xcd4` < 0), the request is queued:
/// the graphics manager's frame count + 4 becomes `+0xcd4`, the visible flag
/// (`+0xcc8`) is saved to `+0xcc4` and cleared.
pub fn fe_module_rider_preview_request_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0xc24), c.g[A1][0]);
        c.g[V0][0] = c.lw(this);
        let different = c.g[V0][0] != c.g[A1][0];
        c.sw(this.wrapping_add(0xc28), c.g[A2][0]);
        let load_all = c.g[A2][0] != 0;
        // Which tail runs: the "same character" tail (true; `v0` = the current
        // character id) or the pending-request tail (false; `v0` = +0xcd4).
        let same_char;
        if different {
            c.cycles += 1;
            c.g[V0][0] = c.lw(this.wrapping_add(0xcd4));
            same_char = false;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(this.wrapping_add(0x10));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                if !load_all {
                    c.g[V0][0] = c.lw(this);
                    same_char = true;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(this.wrapping_add(0xcd4));
                    same_char = false;
                }
            } else {
                // 2 + 2 + 2 (+ 1 when `load_all` is 0) cycles of the
                // generated three-way flag test.
                c.cycles += 2;
                if load_all {
                    c.g[V0][0] = c.lw(this);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(this);
                }
                same_char = true;
            }
        }
        if same_char {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let big_id = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !big_id {
                c.cycles += 3;
                c.g[V0][0] = c.lw(this.wrapping_add(0xcb8));
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(this.wrapping_add(0xcb4));
                    c.g[V1][0] = (0 < c.g[V0][0]) as u64;
                }
            }
            c.cycles += 2;
            let not_ready = c.g[V1][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !not_ready {
                c.cycles += 5;
                c.g[V1][0] = c.lw(this.wrapping_add(0xcc8));
                c.sw(this.wrapping_add(0xcd4), c.g[V0][0]);
                c.g[A0][0] = sx(this.wrapping_add(0xcc8));
                let shown = c.g[V1][0] != 0;
                c.g[A1][0] = 0;
                let mut set_visible = shown;
                if !shown {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(this.wrapping_add(0xcc4));
                    if c.g[V0][0] == 0 {
                        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    } else {
                        set_visible = true;
                    }
                }
                if set_visible {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                }
                c.cycles += 2;
                c.sw(this.wrapping_add(0xcc4), 0);
            }
        } else {
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 11;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_e65c)?;
            let this = c.g[S0][0] as u32;
            c.g[V1][0] = c.lw(this.wrapping_add(0xcc8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.sw(this.wrapping_add(0xcd4), c.g[V0][0]);
            c.sw(this.wrapping_add(0xcc4), c.g[V1][0]);
            c.sw(this.wrapping_add(0xcc8), 0);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEModule_RiderPreview* FEModule_RiderPreviewCtor(FEModule_RiderPreview* self)` (0x19e3d0).
///
/// Clears the rider preview module: ids set to -1, the 256-entry texture
/// reference table (12-byte entries at `+0x1c`: flag 0, id -1), the pose and
/// board pose copied from the identity pose at 0x4ff230, counters, flags and
/// loading state cleared. Returns `self`.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn fe_module_rider_preview_ctor(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A0][0] = 0xffffffffffffffff;
    c.sw(c.g[A3][0] as u32, c.g[V0][0]);
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc70), 0u64);
    c.g[V0][0] = 0xff;
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc74), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x10), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x14), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x18), 0u64);
    'l_19e408: loop {
        c.cycles += 7;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        let t = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0019e408);
            }
            continue 'l_19e408;
        } else {
            c.cycles += 29;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc1c), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc2c), c.g[V0][0]);
            c.g[A1][0] = 0x500000;
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc20), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xdd0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc24), c.g[A2][0]);
            c.g[V0][0] = c.g[A3][0];
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc28), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xccc), 0u64);
            c.lq(3, c.g[A1][0] as u32);
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0xc30));
            c.lq(4, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sq(4, (c.g[A3][0] as u32).wrapping_add(0xc40));
            c.lq(3, c.g[A1][0] as u32);
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0xc50));
            c.lq(4, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcd4), c.g[A2][0]);
            c.sq(4, (c.g[A3][0] as u32).wrapping_add(0xc60));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc78), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcc4), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcc8), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcb4), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcb8), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcbc), 0u64);
            let t = c.g[RA][0] as u32;
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcc0), 0u64);
            c.pc = t;
            return;
        }
    }
}

/// `int BoltFile_CompareTextureSlots(const int* first, const int* second)` (0x19de18): the
/// qsort comparator that puts the loaded (state 3), type-11 slots with no load stamp
/// first and orders them by name (`strcasecmp`); `-1`/`1` when only the first/second is
/// such a slot, `0` when neither is. Keys are `(bank << 16) | slot` into the slot table
/// at `gp - 0x1784` (banks of 0x12000 bytes, slots of 0x120).
pub fn bolt_file_compare_texture_slots(c: &mut Cpu) {
    if c.pc != 0x0019de18 {
        return ee::s03::f_0019de18__generated(c);
    }
    guest(c, |c| {
        'b_19df14: {
            'b_19df10: {
                'b_19defc: {
                    'b_19deb0: {
                        c.cycles += 20;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                        c.g[V0][0] = 0x10000;
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[V0][0] = c.g[V0][0] | 0x2000;
                        c.g[T0][0] = 0x120;
                        c.g[A3][0] = 0x3;
                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                        c.g[A2][0] = c.lhu(c.g[A1][0] as u32);
                        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
                        c.mult(0, 4, 4, 2);
                        c.g[V1][0] = c.g[V1][0] & 0xffff;
                        c.mult(0, 3, 3, 8);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1784));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.mult(0, 3, 6, 8);
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                        if c.g[V1][0] != c.g[A3][0] {
                            c.g[A2][0] = c.lw(c.g[A1][0] as u32);
                            break 'b_19deb0;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));
                            c.g[V0][0] = 0xb;
                            if c.g[A2][0] != c.g[V0][0] {
                                c.g[A2][0] = c.lw(c.g[A1][0] as u32);
                                break 'b_19deb0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));
                                if c.g[V0][0] != 0u64 {
                                    c.g[A2][0] = c.lw(c.g[A1][0] as u32);
                                    break 'b_19deb0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_19df10;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x110));
                                        let t = c.g[V0][0] != c.g[A2][0];
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_19df10;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x11c));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            if t {
                                                break 'b_19defc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                                if t {
                                                    break 'b_19df14;
                                                } else {
                                                    break 'b_19deb0;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_19df10;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x110));
                        c.g[V0][0] = 0xb;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_19df10;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x11c));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_19df10;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                let t = c.g[V0][0] != c.g[A2][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_19df10;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19df10;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_19df10;
                                        } else {
                                            break 'b_19defc;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[RA][0] = sx(0x0019df08);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x0041aa88, 0x0019df08)?;
                let t = true;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                if t {
                    break 'b_19df14;
                } else {
                    break 'b_19df10;
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_19df14;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void BoltFile_PrioritizeModels(BoltTable* self, int bank)` (0x19df20): sorts the bank's 256
/// slot keys with the model comparator, then walks them in order, stamping each unstamped
/// type-8 (model) slot with the bank's running load counter and starting its load when
/// nothing is pending and the slot is in state 3. Stops at the first slot that is stamped or
/// of another type.
pub fn bolt_file_prioritize_models(c: &mut Cpu) {
    if c.pc != 0x0019df20 {
        return ee::s03::f_0019df20__generated(c);
    }
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4a0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.g[V1][0] = c.g[SP][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.g[S5][0] = c.g[A1][0];
        c.g[S6][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
        c.g[A1][0] = sx((c.g[S5][0] as u32) << 16);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.g[A0][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x400), c.g[RA][0]);

        // Fill the 256 keys (bank << 16 | index).
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 256i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if !t {
                break;
            }
            if back_edge_unwinds(c, 0x0019df60) {
                return Ok(());
            }
        }

        c.cycles += 23;
        c.g[A3][0] = 0x1a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x100;
        c.g[A2][0] = 0x4;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x22f0));
        c.g[RA][0] = sx(0x0019df98);
        c.g[FP][0] = 0x120;
        call_ptr(c, 0x00418ef8, 0x0019df98)?;
        c.g[S3][0] = 0u64;
        c.g[V0][0] = 0x10000;
        c.g[A2][0] = c.lhu(c.g[SP][0] as u32);
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        c.g[V1][0] = 0xb0000;
        c.mult(0, 4, 21, 2);
        c.mult(0, 5, 6, 30);
        c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S7][0] = 0x8;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4008));
        c.g[S0][0] = c.g[SP][0];
        c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
        c.g[A0][0] = sx((c.g[S5][0] as u32) << 2);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));

        // Walk the sorted keys while the slots are unstamped and of this type.
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));

            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));

            if c.g[V0][0] != c.g[S7][0] {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                break;
            }
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            if back_edge_unwinds(c, 0x0019dfd8) {
                return Ok(());
            }
            // stamp the slot
            c.cycles += 6;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4004));

            if (c.g[V1][0] as i64) > 0 {
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.g[V1][0] = 0x3;

                if c.g[V0][0] != c.g[V1][0] {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0019e00c);
                    c.g[A1][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0019d8b8,
                        f: ee::s03::BoltFile_LoadSlot,
                    }
                    .call(c, 0x0019e00c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 256i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                break;
            }
            c.cycles += 3;
            c.g[A2][0] = c.lhu(c.g[S0][0] as u32);
            c.mult(0, 2, 6, 30);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        }

        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.pc = t;
        Ok(())
    });
}

/// `void BoltFile_PrioritizeTextures(BoltTable* self, int bank)` (0x19e070): the texture twin
/// of `BoltFile_PrioritizeModels` (type 11, sorted with `BoltFile_CompareTextureSlots`).
pub fn bolt_file_prioritize_textures(c: &mut Cpu) {
    if c.pc != 0x0019e070 {
        return ee::s03::f_0019e070__generated(c);
    }
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4a0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.g[V1][0] = c.g[SP][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.g[S5][0] = c.g[A1][0];
        c.g[S6][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
        c.g[A1][0] = sx((c.g[S5][0] as u32) << 16);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.g[A0][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x400), c.g[RA][0]);

        // Fill the 256 keys (bank << 16 | index).
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 256i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if !t {
                break;
            }
            if back_edge_unwinds(c, 0x0019e0b0) {
                return Ok(());
            }
        }

        c.cycles += 23;
        c.g[A3][0] = 0x1a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x100;
        c.g[A2][0] = 0x4;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x21e8));
        c.g[RA][0] = sx(0x0019e0e8);
        c.g[FP][0] = 0x120;
        call_ptr(c, 0x00418ef8, 0x0019e0e8)?;
        c.g[S3][0] = 0u64;
        c.g[V0][0] = 0x10000;
        c.g[A2][0] = c.lhu(c.g[SP][0] as u32);
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        c.g[V1][0] = 0xb0000;
        c.mult(0, 4, 21, 2);
        c.mult(0, 5, 6, 30);
        c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S7][0] = 0xb;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4008));
        c.g[S0][0] = c.g[SP][0];
        c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
        c.g[A0][0] = sx((c.g[S5][0] as u32) << 2);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A1][0] as u32));

        // Walk the sorted keys while the slots are unstamped and of this type.
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x11c));

            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x110));

            if c.g[V0][0] != c.g[S7][0] {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                break;
            }
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            if back_edge_unwinds(c, 0x0019e128) {
                return Ok(());
            }
            // stamp the slot
            c.cycles += 6;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4004));

            if (c.g[V1][0] as i64) > 0 {
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.g[V1][0] = 0x3;

                if c.g[V0][0] != c.g[V1][0] {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0019e15c);
                    c.g[A1][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0019d8b8,
                        f: ee::s03::BoltFile_LoadSlot,
                    }
                    .call(c, 0x0019e15c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 256i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x490));
                break;
            }
            c.cycles += 3;
            c.g[A2][0] = c.lhu(c.g[S0][0] as u32);
            c.mult(0, 2, 6, 30);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        }

        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x480));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x450));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.pc = t;
        Ok(())
    });
}

/// `cFEStateCharEquip::cFEStateCharEquip(this, a1, player, flag, t0)`
/// (0x198db8): construct the character-equipment screen state: the camera
/// constants, the item-list bookkeeping fields, the five sound/anim handles
/// of the UI, the previewed character (`FEModule_RequestPreviewChar`) and
/// the item list (`BuildItemList`, `BoltFile_Acquire`).
pub fn cfestate_char_equip_ctor(c: &mut Cpu) {
    if c.pc != 0x0019_8db8 {
        return ee::s03::f_00198db8__entries(c);
    }
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, 18, 0x30);
        c.g[S4][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x0019_8dec)?;
        // Camera constants (floats built through `at`).
        c.g[V0][0] = 0x470000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 6);
        c.g[AT][0] = 0xffff_ffff_c2d2_0000;
        c.mtc1(1, 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ab8));
        c.g[AT][0] = 0xffff_ffff_c2c8_0000;
        c.mtc1(1, 4);
        c.g[V1][0] = 0xe;
        c.g[AT][0] = 0xffff_ffff_c2a0_0000;
        c.mtc1(1, 3);
        c.g[A1][0] = c.g[S0][0];
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59b0));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59ac));
        c.g[AT][0] = 0xffff_ffff_c368_0000;
        c.mtc1(1, 2);
        let this = |off: u32, c: &Cpu| (c.g[S1][0] as u32).wrapping_add(off);
        let a = this(8, c);
        c.sw(a, c.g[V0][0]);
        let a = this(0xac0, c);
        c.swc1(5, a);
        let a = this(0xac4, c);
        c.swc1(4, a);
        let a = this(0xac8, c);
        c.swc1(3, a);
        let a = this(0xad0, c);
        c.swc1(0, a);
        let a = this(0xad4, c);
        c.swc1(1, a);
        let a = this(0xad8, c);
        c.swc1(2, a);
        let a = this(0xadc, c);
        c.swc1(6, a);
        let a = this(0xc, c);
        c.sw(a, c.g[V1][0]);
        let a = this(0xacc, c);
        c.swc1(6, a);
        let a = this(0x44, c);
        c.sb(a, c.g[S0][0]);
        let owner = Addr(this(0x10, c)).lw(c);
        c.g[V0][0] = owner;
        c.g[A0][0] = Addr(owner as u32).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x0019_8e64)?;
        let a = this(0x15, c);
        c.sb(a, c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0019_8e6c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0019_8e78)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_7488, 0x0019_8e88)?;
        let a = this(0xbc, c);
        c.sw(a, c.g[V0][0]);
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let a = this(0xc8, c);
            c.sw(a, c.g[V0][0]);
        } else {
            c.cycles += 1;
            let a = this(0xc8, c);
            c.sw(a, 0);
        }
        c.cycles += 38;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f20)).lw(c);
        c.g[A0][0] = sx(this(0xd0, c));
        for off in [0x958, 0xa60] {
            let a = this(off, c);
            c.sw(a, 0);
        }
        c.g[A2][0] = 0x50;
        let a = this(0x960, c);
        c.sb(a, 0);
        for off in [
            0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x78, 0x7c, 0xab0,
        ] {
            let a = this(off, c);
            c.sw(a, 0);
        }
        let a = this(0xa74, c);
        c.sw(a, c.g[V0][0]);
        let a = this(0xae0, c);
        c.sw(a, c.g[V1][0]);
        for off in [0x95c, 0xa64, 0xa68, 0xa6c, 0xa70] {
            let a = this(off, c);
            c.sw(a, c.g[V0][0]);
        }
        call_ptr(c, 0x003e_6448, 0x0019_8f18)?;
        c.g[A2][0] = 0xffff_ffff_fffe_0000;
        let a = this(0x120, c);
        c.sw(a, 0);
        c.g[A2][0] |= 0x7961;
        c.g[A0][0] = sx(this(0xa78, c));
        c.g[V1][0] = sx(this(0xa4, c));
        c.g[V0][0] = sx(this(0x8c, c));
        c.g[A1][0] = 5;
        // Six slots of three parallel arrays (+0x8c, +0xa4, +0xa78).
        loop {
            c.cycles += 8;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            Addr(c.g[V1][0] as u32).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            let fill = c.g[A2][0];
            Addr(c.g[A0][0] as u32).sw(c, fill);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_8f38);
                }
                continue;
            }
            break;
        }
        c.cycles += 69;
        for off in [0x80, 0x84, 0x88] {
            let a = this(off, c);
            c.sw(a, 0);
        }
        // Four handles looked up by name in the UI at `owner + 0x58`:
        // (name string offset in 0x460000.., field).
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a8));
        c.g[S0][0] = Addr(this(0x10, c)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_8f74)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_8380, 0x0019_8f80)?;
        for (delta, field, ret_name, ret_lookup) in [
            (0x7b8u32, 0xa90u32, 0x0019_8f94u32, 0x0019_8fa0u32),
            (0u32.wrapping_sub(0x25b8), 0xa94, 0x0019_8fb4, 0x0019_8fc0),
            (0u32.wrapping_sub(0x25c8), 0xa98, 0x0019_8fd4, 0x0019_8fe0),
        ] {
            c.g[A0][0] = 0x460000;
            c.g[S0][0] = Addr(this(0x10, c)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(delta));
            let a = this(field, c);
            c.sw(a, c.g[V0][0]);
            call_ptr(c, 0x0031_7670, ret_name)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_8380, ret_lookup)?;
        }
        let a = this(0xa9c, c);
        c.sw(a, c.g[V0][0]);
        c.g[A3][0] = 1;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        let a = this(0xaa0, c);
        c.sw(a, 0);
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a_0508, 0x0019_9004)?;
        call_ptr(c, 0x0014_dc80, 0x0019_900c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0019_9018)?;
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        call_ptr(c, 0x0015_0928, 0x0019_9028)?;
        let a = this(0xc0, c);
        c.sw(a, c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0019_9030)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, 0x0014_dd58, 0x0019_903c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_aea8, 0x0019_9050)?;
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        call_ptr(c, 0x0014_b478, 0x0019_9060)?;
        let a = this(0xc4, c);
        c.sw(a, c.g[V0][0]);
        let a = this(0xae4, c);
        c.sw(a, c.g[S4][0]);
        // Item-list mode `v0` (0xcc): -1 when both flags are zero, else
        // derived through a 5-entry jump table on the player's slot kind.
        'mode: {
            if c.g[S3][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if c.g[S4][0] == 0 {
                    break 'mode;
                }
            }
            c.cycles += 9;
            c.g[A0][0] = Addr(this(0x44, c)).lb(c);
            call_ptr(c, 0x0014_a130, 0x0019_907c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_6d98, 0x0019_9088)?;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11));
            c.g[V0][0] = (c.g[V1][0] < 5) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = 0x460000;
            if out_of_range {
                c.cycles += 1;
                c.g[V0][0] = 1;
                break 'mode;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let (extra, mode) = match target {
                0x0019_90b0 | 0x0019_90b8 => (2, 1),
                0x0019_90c0 | 0x0019_90c8 => (2, 2),
                0x0019_90d0 => (2, 3),
                _ => {
                    c.jump_out(target, 0x0019_8db8, 0x0019_9144);
                    return Ok(());
                }
            };
            c.cycles += extra;
            c.g[V0][0] = mode;
        }
        c.cycles += 26;
        let a = this(0xcc, c);
        c.sw(a, c.g[V0][0]);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f20)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_b180, 0x0019_90ec)?;
        c.g[A1][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_b700, 0x0019_90f8)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A3][0] = 9;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[T0][0] = 1;
        c.g[T1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019_da10, 0x0019_911c)?;
        let a = this(0xa64, c);
        c.sw(a, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

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
pub(crate) const GP_WORLD: u32 = 0x848;

/// `world->fe->+0xb0 + 0x1a70` is the bolt file manager; `+0xb0` the rider manager.
pub(crate) const FE_RIDER_MANAGER: u32 = 0xb0;

pub(crate) const FE_BOLT_FILES: u32 = 0x1a70;

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
pub(crate) fn cfe_poll_icons(c: &mut Cpu, this: u32, ret_is: u32, ret_refresh: u32) -> Flow {
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
pub(crate) enum CfeActionExit {
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
pub(crate) fn cfe_action_event(c: &mut Cpu) -> Result<CfeActionExit, crate::call::Unwound> {
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
pub(crate) fn cfe_action_select(
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
pub(crate) fn cfe_action_back(
    c: &mut Cpu,
    this: u32,
) -> Result<CfeActionExit, crate::call::Unwound> {
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

/// `void cFEStateCharEquip_UpdatePreview(cFEStateCharEquip* self)` (0x19a638): per-frame
/// load of the large item preview texture. In buy mode (`+0xc8 == 1`) it counts
/// down the delay at `+0x95c`; at zero it acquires the preview bolt file
/// (`+0xa6c` = file index). Once the file is loaded, its first entry becomes the
/// preview texture (`+0xa70`, through the graphics manager's AddTexture vtable
/// slot), the file is released and the item info is refreshed.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_update_preview(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a744: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19a744;
            } else {
                'b_19a740: {
                    'b_19a6a4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x95c));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_19a6a4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x95c), c.g[V0][0]);
                            if t {
                                break 'b_19a740;
                            } else {
                                c.cycles += 13;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x960));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                c.g[A3][0] = 0x9;
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                c.g[T0][0] = 0x1;
                                c.g[T1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                                Callee {
                                    addr: 0x0019da10,
                                    f: ee::s03::f_0019da10,
                                }
                                .call(c, 0x0019a694)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa6c), c.g[V0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x95c), c.g[V1][0]);
                                break 'b_19a740;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa6c));
                    let t = (c.g[A2][0] as i64) < 0;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_19a740;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0019e238,
                            f: ee::s03::f_0019e238,
                        }
                        .call(c, 0x0019a6c4)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_19a740;
                        } else {
                            'b_19a720: {
                                'b_19a718: {
                                    c.cycles += 5;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa6c));
                                    Callee {
                                        addr: 0x0019e2b0,
                                        f: ee::s03::f_0019e2b0,
                                    }
                                    .call(c, 0x0019a6d8)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    if t {
                                        break 'b_19a718;
                                    } else {
                                        c.cycles += 14;
                                        c.g[A2][0] = 0x4a0000;
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1960));
                                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                                        c.g[A3][0] = 0u64;
                                        c.g[A1][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[T0][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                                        c.g[T1][0] = 0xffffffffffffffff;
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[T2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x0019a710)?;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa70), c.g[V0][0]);
                                        break 'b_19a720;
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa70), c.g[V0][0]);
                                break 'b_19a720;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa6c));
                            Callee {
                                addr: 0x0019dc20,
                                f: ee::s03::f_0019dc20,
                            }
                            .call(c, 0x0019a730)?;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[A0][0] = c.g[S0][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa6c), c.g[V0][0]);
                            Callee {
                                addr: 0x0019a9b8,
                                f: ee::s03::f_0019a9b8,
                            }
                            .call(c, 0x0019a740)?;
                            break 'b_19a740;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19a744;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_ReleasePreviewBoltTextures(self)` (0x19a758): if the
/// front end exists, ask its rider preview for the user's rider (`self+0x44`)
/// and release that rider's bolt textures.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_release_preview_bolt_textures(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a790: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            let t = c.g[A0][0] == 0u64;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            if t {
                break 'b_19a790;
            } else {
                c.cycles += 7;
                c.g[A1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                Callee {
                    addr: 0x001a0548,
                    f: ee::s03::f_001a0548,
                }
                .call(c, 0x0019a780)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x0019fa78,
                    f: ee::s03::f_0019fa78,
                }
                .call(c, 0x0019a78c)?;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                break 'b_19a790;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_UpdateHelpText(self)` (0x19a798): hide the help text
/// widget (`+0x4c`), choose the help string for the selected item (no items,
/// category row, affordable or not in buy mode, outfit polygon budget in equip
/// mode), set it, and show the widget again.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_update_help_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a9a4: {
            'b_19a9a0: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_19a9a0;
                } else {
                    'b_19a85c: {
                        'b_19a7f4: {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0019a7d0)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                            if (c.g[S2][0] as i64) < 0 {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                break 'b_19a7f4;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                                if t {
                                    break 'b_19a85c;
                                } else {
                                    c.cycles += 1;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                    break 'b_19a7f4;
                                }
                            }
                        }
                        'b_19a840: {
                            'b_19a83c: {
                                c.cycles += 10;
                                c.g[A1][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0019a80c)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_19a83c;
                                } else {
                                    'b_19a834: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xae4));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A0][0] = 0x460000;
                                            break 'b_19a834;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x8d8));
                                            break 'b_19a840;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8f0));
                                    break 'b_19a840;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x908));
                            break 'b_19a840;
                        }
                        c.cycles += 7;
                        call_ptr(c, 0x00317670, 0x0019a848)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x0019a854)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_19a9a4;
                    }
                    'b_19a988: {
                        'b_19a970: {
                            'b_19a954: {
                                'b_19a87c: {
                                    c.cycles += 6;
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[V1][0] = c.g[V1][0] & 0x20;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_19a87c;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x920));
                                        break 'b_19a954;
                                    }
                                }
                                'b_19a8c0: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_19a8c0;
                                    } else {
                                        'b_19a89c: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                                            let t = (c.g[V1][0] as i64) <= 0;
                                            c.g[V0][0] = 0xa;
                                            if t {
                                                break 'b_19a89c;
                                            } else {
                                                c.cycles += 1;
                                                c.mult(0, 3, 3, 2);
                                                break 'b_19a89c;
                                            }
                                        }
                                        'b_19a8b8: {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A0][0] = 0x460000;
                                                break 'b_19a8b8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x940));
                                                break 'b_19a954;
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x958));
                                        break 'b_19a954;
                                    }
                                }
                                'b_19a914: {
                                    'b_19a910: {
                                        c.cycles += 17;
                                        call_ptr(c, 0x0014dc80, 0x0019a8c8)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x9;
                                        call_ptr(c, 0x0014dd58, 0x0019a8d4)?;
                                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x0014ad28, 0x0019a8e4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_19a910;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            c.g[A2][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            break 'b_19a914;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A2][0] = 0u64;
                                    break 'b_19a914;
                                }
                                c.cycles += 12;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
                                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x98));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1818));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa78));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                if c.g[A0][0] != 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_19a970;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] & 0x4;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x970));
                                    break 'b_19a954;
                                }
                            }
                            c.cycles += 7;
                            call_ptr(c, 0x00317670, 0x0019a95c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0019a968)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                            break 'b_19a988;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x988));
                        call_ptr(c, 0x00317670, 0x0019a978)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x0019a984)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                        break 'b_19a988;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0019a9a0)?;
                    break 'b_19a9a0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_19a9a4;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshEquipInfo(self)` (0x19aa08): equip mode panel
/// refresh: hide the polygon group, the small bolt icon and the cost widget,
/// show them for a selected non-category item, set the icon texture, and
/// update the polygon preview bar.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_refresh_equip_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ab5c: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_19ab5c;
            } else {
                'b_19aa50: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19aa50;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019aa50)?;
                        break 'b_19aa50;
                    }
                }
                'b_19aa70: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19aa70;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019aa70)?;
                        break 'b_19aa70;
                    }
                }
                'b_19aa90: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19aa90;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019aa90)?;
                        break 'b_19aa90;
                    }
                }
                'b_19ab50: {
                    'b_19ab4c: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                        c.g[S1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_19ab4c;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V1][0] & 0x4;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[V1][0] & 0x20;
                            if t {
                                break 'b_19ab4c;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.g[S0][0];
                                    break 'b_19ab50;
                                } else {
                                    c.cycles += 15;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0019aae8)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x98));
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa78));
                                    if t {
                                        break 'b_19ab4c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A0][0] = c.g[S0][0];
                                            break 'b_19ab50;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xa60));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_19ab4c;
                                            } else {
                                                c.cycles += 11;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x0019ab34)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x0019a098,
                                                    f: ee::s03::f_0019a098,
                                                }
                                                .call(c, 0x0019ab40)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x78),
                                                    c.g[V0][0],
                                                );
                                                c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                                                break 'b_19ab4c;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    break 'b_19ab50;
                }
                c.cycles += 3;
                c.g[A1][0] = c.g[S3][0];
                Callee {
                    addr: 0x0019bd48,
                    f: ee::s03::f_0019bd48,
                }
                .call(c, 0x0019ab58)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_19ab5c;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshBuyInfo(self)` (0x19ab78): buy mode panel refresh:
/// show the large preview texture for the selected item, and its price
/// (x10 when positive) in the cost text.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_refresh_buy_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ac90: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19ac90;
            } else {
                'b_19abbc: {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_19abbc;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                        if t {
                            break 'b_19abbc;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            break 'b_19abbc;
                        }
                    }
                }
                'b_19ac18: {
                    'b_19ac14: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_19ac14;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0019abdc)?;
                            if c.g[S1][0] == 0u64 {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                break 'b_19ac18;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa70));
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_19ac14;
                                } else {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0u64);
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0019ac14)?;
                                    break 'b_19ac14;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    break 'b_19ac18;
                }
                'b_19ac8c: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19ac8c;
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019ac34)?;
                        if c.g[S1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_19ac90;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V0][0] & 0x20;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_19ac90;
                            } else {
                                'b_19ac5c: {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_19ac5c;
                                    } else {
                                        c.cycles += 1;
                                        c.mult(0, 4, 4, 2);
                                        break 'b_19ac5c;
                                    }
                                }
                                c.cycles += 12;
                                Callee {
                                    addr: 0x00198af0,
                                    f: ee::s03::f_00198af0,
                                }
                                .call(c, 0x0019ac64)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a0c50, 0x0019ac70)?;
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                c.g[A1][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0019ac8c)?;
                                break 'b_19ac8c;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19ac90;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int cFEStateCharEquip_BuildItemList(self, category)` (0x19b180): fill the item
/// list (`+0x124`, count at `+0x958`) with the category's items that are listed in
/// this mode (buy: not yet bought and for the current slot; equip: owned),
/// then sort it. Returns the count.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_build_item_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b414: {
            'b_19b410: {
                'b_19b334: {
                    c.cycles += 43;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8e0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.g[S1][0] = c.g[A1][0];
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x890));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x860));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x880));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x870));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x850));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x124));
                    call_ptr(c, 0x0014dc80, 0x0019b1bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019b1c8)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014bdb8, 0x0019b1d0)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.g[S7][0] = c.g[V0][0];
                    call_ptr(c, 0x0014ad28, 0x0019b1e4)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.g[T0][0] = 0xffffffffffffffff;
                    c.g[T1][0] = 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014d7e8, 0x0019b204)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x958), 0u64);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x834;
                    call_ptr(c, 0x003e6448, 0x0019b21c)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc8));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = c.g[S4][0];
                    if t {
                        break 'b_19b334;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[S0][0] as i64) <= 0;
                        c.g[FP][0] = 0x1a0000;
                        if t {
                            break 'b_19b410;
                        } else {
                            c.cycles += 3;
                            c.g[S5][0] = c.g[S0][0];
                            c.g[S4][0] = c.g[SP][0];
                            'l_19b240: loop {
                                'b_19b324: {
                                    c.cycles += 4;
                                    c.g[S0][0] = c.lw((c.g[S4][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                        break 'b_19b324;
                                    } else {
                                        'b_19b274: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[V0][0] & 0x4;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                                break 'b_19b274;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[A0][0] & 0x20;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[S5][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                                    break 'b_19b324;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    break 'b_19b274;
                                                }
                                            }
                                        }
                                        'b_19b2a0: {
                                            'b_19b29c: {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x288));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[A3][0] =
                                                    c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_19b29c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                                    c.g[S1][0] = sx((c.g[S3][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    break 'b_19b2a0;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S1][0] = 0u64;
                                            break 'b_19b2a0;
                                        }
                                        'b_19b308: {
                                            'b_19b2d8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[A0][0] & 0x20;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
                                                if t {
                                                    break 'b_19b2d8;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32
                                                        >> 16)
                                                        as u32);
                                                    c.g[A1][0] = c.g[S7][0];
                                                    c.g[A2][0] = c.g[S3][0];
                                                    Callee {
                                                        addr: 0x0019b098,
                                                        f: ee::s03::f_0019b098,
                                                    }
                                                    .call(c, 0x0019b2c0)?;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x958));
                                                        break 'b_19b308;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x34));
                                                        c.g[V0][0] = c.g[V0][0] & 0x20;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[S5][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            break 'b_19b324;
                                                        } else {
                                                            break 'b_19b2d8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] & 0x2;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S5][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                                break 'b_19b324;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0xcc));
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                if c.g[V1][0] == c.g[V0][0] {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x958)
                                                        );
                                                    break 'b_19b308;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lb((c.g[S0][0] as u32).wrapping_add(0x2));
                                                    if c.g[V1][0] != c.g[V0][0] {
                                                        c.g[S5][0] =
                                                            sx((c.g[S5][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        break 'b_19b324;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x958));
                                                        break 'b_19b308;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x958), c.g[V1][0]);
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                        break 'b_19b324;
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S5][0] != 0u64;
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019b240);
                                    }
                                    continue 'l_19b240;
                                } else {
                                    c.cycles += 2;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                    break 'b_19b414;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) <= 0;
                c.g[FP][0] = 0x1a0000;
                if t {
                    break 'b_19b410;
                } else {
                    c.cycles += 3;
                    c.g[S4][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                    'l_19b348: loop {
                        'b_19b404: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            if c.g[V0][0] == 0u64 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_19b404;
                            } else {
                                'b_19b378: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[V0][0] = c.g[V0][0] & 0x4;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        break 'b_19b378;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A0][0] & 0x20;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                            break 'b_19b404;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                            break 'b_19b378;
                                        }
                                    }
                                }
                                'b_19b3a4: {
                                    'b_19b3a0: {
                                        c.cycles += 6;
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[A3][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_19b3a0;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            c.g[V1][0] = sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            break 'b_19b3a4;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = 0u64;
                                    break 'b_19b3a4;
                                }
                                'b_19b3e8: {
                                    'b_19b3c8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A0][0] & 0x20;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
                                            break 'b_19b3c8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] & 0x2;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                                break 'b_19b3e8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                                break 'b_19b404;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 16) as u32);
                                    c.g[A1][0] = c.g[S7][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    Callee {
                                        addr: 0x0019afd0,
                                        f: ee::s03::f_0019afd0,
                                    }
                                    .call(c, 0x0019b3dc)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_19b404;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                        break 'b_19b3e8;
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x958), c.g[V1][0]);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_19b404;
                            }
                        }
                        c.cycles += 3;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                        if c.g[S4][0] != 0u64 {
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                            if event_due(c) {
                                deliver_event_here(c, 0x0019b348);
                            }
                            continue 'l_19b348;
                        } else {
                            break 'b_19b410;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
            break 'b_19b414;
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x7260));
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x00418ef8, 0x0019b424)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8d0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x850));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8e0));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshItemList(cFEStateCharEquip *self, int reset, int select)` (0x19b618):
/// hides the six name/check widgets, optionally resets the scroll menu to the
/// item count, refills the rows (buy list or equip list by mode), shows the
/// "empty" row when there is nothing, and clamps and re-selects the visible row.
pub fn cfe_state_char_equip_refresh_item_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b7bc: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S4][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] == 0u64;
            c.g[S5][0] = c.g[A2][0];
            if t {
                break 'b_19b7bc;
            } else {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa4));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                c.g[S3][0] = 0x5;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                'l_19b660: loop {
                    c.cycles += 18;
                    c.g[A1][0] = 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019b680);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019b680)?;
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019b6a0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019b6a0)?;
                    if (c.g[S3][0] as i64) >= 0 {
                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x0019b660);
                        }
                        continue 'l_19b660;
                    } else {
                        'b_19b6d8: {
                            c.cycles += 2;
                            if c.g[S4][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                break 'b_19b6d8;
                            } else {
                                'b_19b6cc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    if t {
                                        break 'b_19b6cc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0019b6c4);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0039b760, 0x0019b6c4)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                        if t {
                                            break 'b_19b6d8;
                                        } else {
                                            break 'b_19b6cc;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0019b6d4);
                                c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x958));
                                call_ptr(c, 0x0039b760, 0x0019b6d4)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                break 'b_19b6d8;
                            }
                        }
                        'b_19b700: {
                            'b_19b6f4: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_19b6f4;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019b6ec);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x0019b598,
                                        f: ee::s03::cFEStateCharEquip_FillBuyRows,
                                    }
                                    .call(c, 0x0019b6ec)?;
                                    let t = true;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_19b700;
                                    } else {
                                        break 'b_19b6f4;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0019b6fc);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0019b518,
                                f: ee::s03::cFEStateCharEquip_FillEquipRows,
                            }
                            .call(c, 0x0019b6fc)?;
                            c.g[A1][0] = c.g[V0][0];
                            break 'b_19b700;
                        }
                        'b_19b788: {
                            'b_19b77c: {
                                c.cycles += 2;
                                if c.g[A1][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    break 'b_19b77c;
                                } else {
                                    c.cycles += 29;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019b724);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0019b724)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                                    c.g[A1][0] = 0x4c0000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6708));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019b744);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0019b744)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x0019b750);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9a0));
                                    call_ptr(c, 0x00317670, 0x0019b750)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                                    c.g[RA][0] = sx(0x0019b75c);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x0019b75c)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[V1][0] = 0x1;
                                    c.g[A2][0] = 0u64;
                                    c.g[A1][0] = 0x1;
                                    c.sb((c.g[A0][0] as u32).wrapping_add(0x96), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    let t = true;
                                    c.sb((c.g[V0][0] as u32).wrapping_add(0x98), 0u64);
                                    if t {
                                        break 'b_19b788;
                                    } else {
                                        break 'b_19b77c;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x98));
                            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                            break 'b_19b788;
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[A2][0] = c.g[A0][0];
                        }
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = 0u64;
                        }
                        let t = c.g[S5][0] == 0u64;
                        if c.g[S4][0] != 0 {
                            c.g[A2][0] = 0u64;
                        }
                        if t {
                            break 'b_19b7bc;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x98));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[RA][0] = sx(0x0019b7bc);
                            c.g[A1][0] = c.g[A1][0] & 0xff;
                            call_ptr(c, 0x0039b960, 0x0019b7bc)?;
                            break 'b_19b7bc;
                        }
                    }
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_SetEquipRow(cFEStateCharEquip *self, int row)` (0x19b7e0):
/// fills one row of the equip list: the item's name (or text id), and the
/// price-delta/colour state found by trial-toggling the bolt in a copy of the
/// character profile (which is restored afterwards).
pub fn cfe_state_char_equip_set_equip_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b820: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8f0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8e0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8d0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x8c0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x8b0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x8a0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x890));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x880));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x870));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x860));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x850), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_19b820;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x98));
                break 'b_19b820;
            }
        }
        'b_19b990: {
            'b_19b878: {
                c.cycles += 20;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa78));
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x840), c.g[A1][0]);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[FP][0] = c.g[A0][0];
                c.g[V1][0] = 0xfffffffffffe0000;
                c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = c.g[V1][0] | 0x7961;
                c.g[A2][0] = c.g[V0][0];
                c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa4));
                c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x20;
                let t = c.g[V0][0] == 0u64;
                c.g[S7][0] = 0u64;
                if t {
                    break 'b_19b878;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xaa0));
                    if t {
                        break 'b_19b990;
                    } else {
                        break 'b_19b878;
                    }
                }
            }
            'b_19b8d0: {
                'b_19b8cc: {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x0019b880);
                    call_ptr(c, 0x0014dc80, 0x0019b880)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0019b88c);
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019b88c)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    c.g[RA][0] = sx(0x0019b8a0);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0014ad28, 0x0019b8a0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        break 'b_19b8cc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        let t = true;
                        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        if t {
                            break 'b_19b8d0;
                        } else {
                            break 'b_19b8cc;
                        }
                    }
                }
                c.cycles += 1;
                c.g[S2][0] = 0u64;
                break 'b_19b8d0;
            }
            c.cycles += 6;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x290));
            c.g[V1][0] = c.g[SP][0];
            c.g[A0][0] = 0x20c;
            c.g[A3][0] = 0xffffffffffffffff;
            c.g[A2][0] = 0xffffffffffffffff;
            'l_19b8e8: loop {
                c.cycles += 7;
                c.sh(c.g[V1][0] as u32, c.g[A3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x2), 0u64);
                let t = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019b8e8);
                    }
                    continue 'l_19b8e8;
                } else {
                    'b_19b97c: {
                        c.cycles += 18;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0019b910);
                        c.g[A2][0] = 0x834;
                        call_ptr(c, 0x003e6574, 0x0019b910)?;
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x0019b920);
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        call_ptr(c, 0x0014b478, 0x0019b920)?;
                        c.g[T0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x2));
                        c.g[S5][0] = c.g[V0][0];
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[T0][0] = c.g[T0][0] & 0x4;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[A3][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x0019b944);
                        c.g[T0][0] = (c.g[T0][0] < 0x1u64) as u64;
                        call_ptr(c, 0x0014afb0, 0x0019b944)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_19b97c;
                        } else {
                            c.cycles += 11;
                            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                            c.g[RA][0] = sx(0x0019b958);
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            call_ptr(c, 0x0014b478, 0x0019b958)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x840));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
                            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = c.g[V0][0] & 0x4;
                            if c.g[V0][0] == 0u64 {
                                c.g[S7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa9c));
                                break 'b_19b97c;
                            } else {
                                c.cycles += 1;
                                c.g[S7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa98));
                                break 'b_19b97c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x0019b990);
                    c.g[A3][0] = c.g[SP][0];
                    call_ptr(c, 0x0014acb0, 0x0019b990)?;
                    break 'b_19b990;
                }
            }
        }
        'b_19b9e4: {
            c.cycles += 2;
            let t = c.g[S6][0] == 0u64;

            if t {
                break 'b_19b9e4;
            } else {
                'b_19b9d0: {
                    'b_19b9c4: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0u64 {
                            c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                            break 'b_19b9c4;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019b9b0);
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                            call_ptr(c, 0x00317670, 0x0019b9b0)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0019b9bc);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x003a1000, 0x0019b9bc)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_19b9d0;
                            } else {
                                break 'b_19b9c4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019b9cc);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x003a0c50, 0x0019b9cc)?;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    break 'b_19b9d0;
                }
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0019b9e4);
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019b9e4)?;
                break 'b_19b9e4;
            }
        }
        'b_19ba34: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8e0));
            if t {
                break 'b_19ba34;
            } else {
                'b_19ba30: {
                    'b_19ba18: {
                        c.cycles += 2;
                        let t = c.g[S7][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_19ba18;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0019ba08);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019ba08)?;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[S7][0]);
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            if t {
                                break 'b_19ba30;
                            } else {
                                break 'b_19ba18;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019ba30);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019ba30)?;
                    break 'b_19ba30;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8e0));
                break 'b_19ba34;
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8d0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x8c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x850));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8f0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_SetBuyRow(cFEStateCharEquip *self, int row)` (0x19ba60):
/// fills one row of the buy list: the item's name and a coloured dollar icon
/// (green when affordable, red otherwise, plus when already owned).
pub fn cfe_state_char_equip_set_buy_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ba88: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x5c));
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_19ba88;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x98));
                break 'b_19ba88;
            }
        }
        'b_19bae4: {
            'b_19bac0: {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x20;
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                if t {
                    break 'b_19bac0;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xaa0));
                    if t {
                        break 'b_19bae4;
                    } else {
                        break 'b_19bac0;
                    }
                }
            }
            'b_19bad0: {
                c.cycles += 3;
                c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xe));
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[V0][0] = 0xa;
                if t {
                    break 'b_19bad0;
                } else {
                    c.cycles += 1;
                    c.mult(0, 3, 3, 2);
                    break 'b_19bad0;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.g[S2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa90));
                break 'b_19bae4;
            } else {
                c.cycles += 1;
                c.g[S2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa94));
                break 'b_19bae4;
            }
        }
        'b_19bb38: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;

            if t {
                break 'b_19bb38;
            } else {
                'b_19bb24: {
                    'b_19bb18: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0u64 {
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                            break 'b_19bb18;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019bb04);
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                            call_ptr(c, 0x00317670, 0x0019bb04)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0019bb10);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a1000, 0x0019bb10)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_19bb24;
                            } else {
                                break 'b_19bb18;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019bb20);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003a0c50, 0x0019bb20)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_19bb24;
                }
                c.cycles += 5;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0019bb38);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019bb38)?;
                break 'b_19bb38;
            }
        }
        'b_19bb88: {
            c.cycles += 2;
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_19bb88;
            } else {
                'b_19bb84: {
                    'b_19bb6c: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_19bb6c;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0019bb5c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019bb5c)?;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[S2][0]);
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                            if t {
                                break 'b_19bb84;
                            } else {
                                break 'b_19bb6c;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019bb84);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019bb84)?;
                    break 'b_19bb84;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_19bb88;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_UpdateStatPreviewBar(cFEStateCharEquip *self, int delta)` (0x19bd48):
/// sets the points bar's colour (invalid when over the limit), visibility
/// and vertical scale from `(polys + delta - 1500) / (max - 1500)`, clamped to 0..1.
pub fn cfe_state_char_equip_update_stat_preview_bar(c: &mut Cpu) {
    guest(c, |c| {
        'b_19be68: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1818));
            if t {
                break 'b_19be68;
            } else {
                'b_19bdc4: {
                    'b_19bdb0: {
                        c.cycles += 16;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
                        c.g[A2][0] = c.g[V0][0];
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[A0][0] = c.g[V1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5dc));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5dc));
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.div_s(20, 1, 0);
                        c.c_lt_s(2, 20);
                        let t = !c.fcc();

                        if t {
                            break 'b_19bdb0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.mov_s(20, 2);
                            if t {
                                break 'b_19bdc4;
                            } else {
                                break 'b_19bdb0;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(20, 0);
                    if c.fcc() {
                        c.mov_s(20, 0);
                        break 'b_19bdc4;
                    } else {
                        break 'b_19bdc4;
                    }
                }
                'b_19be1c: {
                    'b_19bdf8: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x4c0000;
                        if t {
                            break 'b_19bdf8;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0019bdf0);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0019bdf0)?;
                            let t = true;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                            if t {
                                break 'b_19be1c;
                            } else {
                                break 'b_19bdf8;
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    c.g[A1][0] = 0x4c0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66f8));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019be18);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0019be18)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    break 'b_19be1c;
                }
                c.cycles += 19;
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0019be34);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019be34)?;
                c.neg_s(1, 20);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(0, c.g[SP][0] as u32);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                break 'b_19be68;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCharEquip_BuySelectedItem(cFEStateCharEquip *self)` (0x19be80):
/// asks the profile interface to buy the selected item and stores the new money.
pub fn cfe_state_char_equip_buy_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x0019beac);
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        call_ptr(c, 0x0014dc80, 0x0019beac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0019beb8);
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x0014dd58, 0x0019beb8)?;
        c.g[A3][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[T0][0] = 0x1;
        c.g[RA][0] = sx(0x0019bed0);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
        call_ptr(c, 0x0014b560, 0x0019bed0)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `int cFEStateCharEquip_EquipSelectedItem(cFEStateCharEquip *self)` (0x19bee8):
/// toggles the selected item's equipped flag in the profile; 1 if it changed.
pub fn cfe_state_char_equip_equip_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        'b_19bfc8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_19bfc8;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = c.g[V0][0] & 0x4;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_19bfc8;
                } else {
                    'b_19bf88: {
                        'b_19bf84: {
                            c.cycles += 18;
                            c.g[RA][0] = sx(0x0019bf38);
                            call_ptr(c, 0x0014dc80, 0x0019bf38)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0019bf44);
                            c.g[A1][0] = 0x9;
                            call_ptr(c, 0x0014dd58, 0x0019bf44)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            c.g[RA][0] = sx(0x0019bf58);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0014ad28, 0x0019bf58)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x288));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if t {
                                break 'b_19bf84;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                let t = true;
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                if t {
                                    break 'b_19bf88;
                                } else {
                                    break 'b_19bf84;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                        break 'b_19bf88;
                    }
                    'b_19bfb8: {
                        c.cycles += 10;
                        c.g[T0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A3][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[T0][0] = c.g[T0][0] & 0x4;
                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[RA][0] = sx(0x0019bfa8);
                        c.g[T0][0] = (c.g[T0][0] < 0x1u64) as u64;
                        call_ptr(c, 0x0014afb0, 0x0019bfa8)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            break 'b_19bfb8;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_19bfc8;
                            } else {
                                break 'b_19bfb8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019bfc4);
                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    call_ptr(c, 0x0014aea8, 0x0019bfc4)?;
                    c.g[V0][0] = 0x1;
                    break 'b_19bfc8;
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// Size of one `BoltFileCache` slot record.
#[cfg(test)]
pub(crate) const OC859_SLOT_SIZE: u32 = 0x120;

/// Slot offsets read by `BoltFile_EvictBytes`: data pointer, timer, size and
/// eviction priority (the state word is at +0).
pub(crate) const OC859_SLOT_DATA: u32 = 0x104;

pub(crate) const OC859_SLOT_TIMER: u32 = 0x114;

pub(crate) const OC859_SLOT_SIZE_OFF: u32 = 0x118;

pub(crate) const OC859_SLOT_PRIORITY: u32 = 0x11c;

/// `void BoltFile_EvictBytes(BoltFileCache* self, int skip_table, int bytes)`
/// (0x19d738): free idle slots until `bytes` bytes are released. Pass 1
/// frees every idle slot of each table before the last (table `skip_table` is
/// skipped); pass 2 repeatedly frees the idle slot with the lowest priority
/// (+0x11c) of the table at `skip_table`, until enough is freed.
pub fn bolt_file_evict_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0;
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0);
        'done: {
            'pass1: loop {
                'next_table: {
                    c.cycles += 2;
                    let same = c.g[S3][0] == c.g[S6][0];
                    c.g[V1][0] = 0x10000;
                    if !same {
                        c.cycles += 2;
                        c.g[S1][0] = 0;
                        c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, S5)));
                        'scan: loop {
                            'next_slot: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC859_SLOT_DATA));
                                if c.g[V0][0] == 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                    break 'next_slot;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC859_SLOT_TIMER));
                                if c.g[V0][0] != 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                    break 'next_slot;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(r32(c, S0));
                                if c.g[V0][0] != 0 {
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                    break 'next_slot;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(OC859_SLOT_SIZE_OFF));
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
                                call_ptr(c, 0x0019_d4b8, 0x0019_d7bc)?; // BoltFile_FreeSlot
                                let done = (c.g[S2][0] as i64) <= 0;
                                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                                if done {
                                    break 'done;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 256) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x120));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019_d780);
                                }
                                continue 'scan;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = 0x10000;
                            break 'next_table;
                        }
                    }
                }
                c.cycles += 5;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[V1][0] |= 0x2000;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 10) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_d770);
                    }
                    continue 'pass1;
                }
                break 'pass1;
            }
            // Pass 2: the idle slot with the lowest priority, repeatedly.
            c.cycles += 6;
            c.mult(0, S0, S6, V1);
            c.g[S5][0] = 0x120;
            c.g[S3][0] = sx(r32(c, S4).wrapping_add(0x118));
            c.g[S1][0] = c.g[S0][0];
            c.g[A2][0] = u64::MAX;
            'pass2: loop {
                c.cycles += 4;
                c.g[A3][0] = 0;
                c.g[A1][0] = 0;
                c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, S1)));
                'pick: loop {
                    'next_slot: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(OC859_SLOT_DATA));
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            break 'next_slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A0));
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            break 'next_slot;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(OC859_SLOT_TIMER));
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            break 'next_slot;
                        }
                        'take: {
                            c.cycles += 2;
                            let none_yet = (c.g[A2][0] as i64) < 0;
                            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(OC859_SLOT_PRIORITY));
                            if !none_yet {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                                    break 'next_slot;
                                }
                            }
                            break 'take;
                        }
                        c.cycles += 3;
                        c.g[A3][0] = c.g[V1][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x120));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d810);
                        }
                        continue 'pick;
                    }
                    c.cycles += 2;
                    let none = (c.g[A2][0] as i64) < 0;
                    c.mult(0, V0, A2, S5);
                    if none {
                        break 'done;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
                    call_ptr(c, 0x0019_d4b8, 0x0019_d884)?; // BoltFile_FreeSlot
                    let more = (c.g[S2][0] as i64) > 0;
                    c.g[A2][0] = u64::MAX;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d800);
                        }
                        continue 'pass2;
                    }
                    break 'done;
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::abi::ret;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;

    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::cstr;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_7;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    use ssx3_ee as ee;

    use ssx3_ee::s03;

    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn bolt_file_get_data_matches() {
        check_lohi(
            ssx3_ee::s03::f_0019e2b0,
            bolt_file_get_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0002 ^ seed as u64);
                scramble(c, &mut r);
                let outer = (seed % 2) as u32;
                let inner = ((seed / 2) % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = outer as u64;
                c.g[A2][0] = inner as u64;
                let base = DATA
                    .wrapping_add(outer.wrapping_mul(0x12000))
                    .wrapping_add(inner.wrapping_mul(0x120));
                c.bus
                    .write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
                c.bus.write32(base + 0x104, r.next_u32());
            },
        );
    }

    #[test]
    fn bolt_file_detach_data_matches() {
        check_lohi(
            ssx3_ee::s03::f_0019e330,
            bolt_file_detach_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0003 ^ seed as u64);
                scramble(c, &mut r);
                let outer = (seed % 2) as u32;
                let inner = ((seed / 2) % 2) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = outer as u64;
                c.g[A2][0] = inner as u64;
                let d = outer
                    .wrapping_mul(0x12000)
                    .wrapping_add(inner.wrapping_mul(0x120));
                let v1 = DATA.wrapping_add(0x104).wrapping_add(d);
                let a3 = DATA.wrapping_add(0x118).wrapping_add(d);
                let counter = DATA.wrapping_add(0xb4000);
                c.bus.write32(v1, r.next_u32());
                c.bus.write32(a3, r.next_u32());
                c.bus.write32(counter, r.next_u32());
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_get_prev_node_matches() {
        check(
            ssx3_ee::s03::f_0019c8d0,
            bolt_file_index_manager_get_prev_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x19c8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.g[A1][0] = node as u64;
                if node != 0 {
                    let val = if seed % 2 == 0 {
                        r.next_u32() & 0x7fff
                    } else {
                        0x8000 | (r.next_u32() & 0x7fff)
                    };
                    c.bus.write16(node + 0xc, val as u16);
                }
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_node_index_matches() {
        check(
            ssx3_ee::s03::f_0019c920,
            bolt_file_index_manager_node_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x19ca ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + (r.next_u32() % 0x100) * 16) as u64
                };
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_ctor_matches() {
        check(
            ssx3_ee::s03::f_0019cb38__generated,
            bolt_file_index_manager_ctor,
            40,
            &|c, seed| {
                let mut r = rng(seed, 0x19);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x2000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    // -----------------------------------------------------------------
    // Bolt file cache and rider previews.

    #[test]
    fn bolt_file_cache_ctor_matches() {
        check(
            s03::f_0019cdf0__generated,
            bolt_file_cache_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x19cdf0, 1);
                globals(c);
                clear(c, OBJ, 0x100);
                c.g[A0][0] = sx(OBJ);
                // An existing cache skips the allocation in `BoltFile_CacheInit`;
                // alternate so both paths run (the null path allocates from the
                // zeroed heap, which returns nothing).
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1788),
                    if seed % 3 == 0 { 0 } else { OBJ },
                );
            },
        );
    }

    /// A dummy world: `world -> +0x84 -> race manager -> +0x84 -> riders`
    /// (the rider count at the riders pointer is left 0, so rider loops stop).
    pub(crate) fn world(c: &mut Cpu) {
        let w = DATA + 0x4000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x5000);
        c.bus.write32(w + 0x84, DATA + 0x6000);
        c.bus.write32(DATA + 0x6000 + 0x84, DATA + 0x7000);
        // No cut-scene transition to force off.
        c.bus.write32(DATA + 0x6000 + 0x64, 0);
    }

    // ------------------------------------------------------------------
    // FE screens.

    #[test]
    fn fe_char_equip_on_deactivate_matches() {
        check(
            ssx3_ee::s03::f_00199420__generated,
            fe_char_equip_on_deactivate,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0x99420, 0);
                world(c);
                let self_ = DATA + 0x300;
                c.bus.write8(self_ + 0x44, (seed % 4) as u8);
                c.g[A0][0] = sx(self_);
            },
        );
    }

    #[test]
    fn bolt_file_cache_dtor_matches() {
        // The cache teardown frees 10 * 256 slots for real, so keep the seed
        // count small: the body is a straight loop, not data dependent.
        check(
            ee::s03::f_0019ce20__generated,
            bolt_file_cache_dtor,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19_ce20, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(obj, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4 == 0) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x19cbe0 / 0x19cce8 BoltFileIndexManager slot lookups.

    pub(crate) fn bolt_slot_setup(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        mask_off: u32,
        value_off: u32,
    ) {
        let mut r = machine(c, seed, salt, 1);
        let index = DATA + 0x100;
        let slot = seed % 8;
        c.g[A0][0] = sx(index);
        c.g[A1][0] = slot as u64;
        c.g[A2][0] = 0;
        c.bus.write32(index + 0x1f40, 0); // no "always create" flag
        let key = slot << 16; // (slot << 16) | 0
        let bucket = key % 400;
        let node = DATA + 0x300;
        c.bus.write32(index + 0x1900 + bucket * 4, node);
        c.bus.write32(node, key);
        let mask = if seed % 3 == 0 {
            0
        } else {
            1u32 << (slot & 31)
        };
        c.bus.write16(node + mask_off, mask as u16);
        c.bus.write16(node + value_off, r.next_u32() as u16);
    }

    #[test]
    fn bolt_file_index_manager_get_model_slot_matches() {
        check(
            ee::s03::f_0019cbe0__generated,
            bolt_file_index_manager_get_model_slot,
            24,
            &|c, seed| bolt_slot_setup(c, seed, 0x19cb_e0, 6, 0xa),
        );
    }

    #[test]
    fn bolt_file_index_manager_get_texture_slot_matches() {
        check(
            ee::s03::f_0019cce8__generated,
            bolt_file_index_manager_get_texture_slot,
            24,
            &|c, seed| bolt_slot_setup(c, seed, 0x19cc_e8, 4, 8),
        );
    }

    #[test]
    fn cfe_state_char_equip_shutdown_matches() {
        check(
            ee::s03::f_00199350__generated,
            cfe_state_char_equip_shutdown,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x1993);
                let module = DATA + 0x1000;
                let manager = DATA + 0x2000;
                c.bus
                    .write32(gp_at(-0x848), if seed % 3 == 0 { 0 } else { module });
                c.bus.write32(module + 0x7c, manager);
                let preview = DATA + 0x3000;
                c.bus.write32(manager + 0xb0, preview);
                c.bus.write8(DATA + 0x44, (seed % 4) as u8);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_alloc_node_matches() {
        check(
            ee::s03::f_0019c880__generated,
            bolt_file_index_manager_alloc_node,
            60,
            &|c, seed| {
                base(c, seed, 0x19c8);
                let m = DATA;
                let n = seed % 3;
                let head = if n == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(m + 0x1f40, head);
                if n != 0 {
                    // The next-node link at +0xe is either -1 (empty) or 0 (the
                    // manager's own node, so a new head exists).
                    c.bus
                        .write16(head + 0xe, if seed % 2 == 0 { 0xbeef } else { 0 });
                    c.bus.write16(head + 0xc, 0xdead);
                    fill(c, &mut Rng(0x19c8_0000 ^ seed as u64), head, 3);
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn bolt_file_find_free_slot_matches() {
        check(
            ee::s03::f_0019d3b0__generated,
            bolt_file_find_free_slot,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x19_d3b0, 0);
                c.g[A0][0] = DATA as u64;
                let bank = (seed % 2) as u32;
                c.g[A1][0] = bank as u64;
                let base = DATA + 0x1_2000 * bank;
                let free_at = if seed % 3 == 0 { 256 } else { 200 + seed % 40 };
                for i in 0..256u32 {
                    let s = base + 0x120 * i;
                    let empty = i >= free_at;
                    let valid = if empty { 0 } else { 1 };
                    let used = if empty {
                        0
                    } else if i % 4 == 0 {
                        0
                    } else {
                        1
                    };
                    w(c, s, used);
                    w(c, s + 0x104, valid);
                    w(c, s + 0x11c, (i * 7) % 64);
                }
                c.pc = 0x0019_d3b0;
            },
        );
    }

    #[test]
    fn c_fe_state_char_equip_on_activate_matches() {
        check(
            ee::s03::f_001993a0__generated,
            c_fe_state_char_equip_on_activate,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1993, 3);
                let s = DATA;
                w(c, s + 0xc8, (seed % 3) as u32);
                fill(c, &mut r, s + 0x20, 0x40);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn bolt_file_find_slot_by_name_matches() {
        check(
            ee::s03::f_0019d428__generated,
            bolt_file_find_slot_by_name,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x19d4, 0);
                env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                cstr(c, name, "BOLT_B");
                // bank 1 selects the row at `this + 0x12000`.
                let row = this + 0x12000;
                match seed % 3 {
                    0 => {}
                    1 => cstr(c, row + 4, "BOLT_A"),
                    _ => {
                        cstr(c, row + 4, "BOLT_A");
                        cstr(c, row + 0x120 + 4, "BOLT_B");
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 1;
                c.g[A2][0] = sx(name);
            },
        );
    }

    #[test]
    fn fe_module_rider_preview_dtor_matches() {
        check(
            ssx3_ee::s03::f_0019e498__generated,
            fe_module_rider_preview_dtor,
            8,
            &|c, seed| {
                let _r = base_3(c, seed, 0x19e498, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
                w_2(c, this + 8, 0);
                w_2(c, this + 0xc, 0);
                w_2(c, GAME_GP.wrapping_sub(0x854), DATA + 0x2000);
                w_2(c, DATA + 0x2000 + 0x10d8, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 0x190, 0);
                w_2(c, DATA + 0x2100 + 0x194, 0);
                for i in 0..256u32 {
                    let slot = this + 0x1c + 0xc * i;
                    let used = seed % 3 != 0 && i % 7 != 0;
                    w_2(c, slot, if used { (i % 16) as u32 } else { 0xffff_ffff });
                    c.bus.write8(slot + 4, 0xff);
                }
            },
        );
    }

    /// A node for the (bit, a2) key in the index manager's hash bucket.
    pub(crate) fn bolt_node(c: &mut Cpu, seed: u32, flags_off: u32, value_off: u32, ref_off: u32) {
        let _ = base_4(c, seed, 0x0019_cc30, 3);
        let bit = (seed % 32) as u32;
        let a2 = (seed % 4) as u32;
        let hash = (bit << 16) | a2;
        let bucket = hash % 0x190;
        let node = OBJ3;
        c.g[A0][0] = OBJ as u64;
        c.g[A1][0] = bit as u64;
        c.g[A2][0] = a2 as u64;
        c.g[A3][0] = if seed & 1 == 0 {
            0
        } else {
            0xffff_ffff_ffff_ffff
        };
        w(c, OBJ + 0x1900 + bucket * 4, node);
        w(c, OBJ + 0x1f40, 0);
        w(c, node, hash);
        c.bus
            .write16(node + flags_off, if seed % 4 == 0 { 0 } else { 0xffff });
        c.bus.write16(node + value_off, (seed % 7) as u16);
        c.bus
            .write16(node + ref_off, if seed % 3 == 0 { 0xffff } else { 1 });
    }

    #[test]
    fn bolt_file_index_manager_set_model_slot_matches() {
        check(
            ee::s03::f_0019cc30__generated,
            bolt_file_index_manager_set_model_slot,
            6,
            &|c, seed| bolt_node(c, seed, 6, 0xa, 8),
        );
    }

    #[test]
    fn bolt_file_index_manager_set_texture_slot_matches() {
        check(
            ee::s03::f_0019cd38__generated,
            bolt_file_index_manager_set_texture_slot,
            6,
            &|c, seed| bolt_node(c, seed, 4, 8, 0xa),
        );
    }

    #[test]
    fn cfe_state_char_equip_on_child_event_matches() {
        check(
            ee::s03::f_00199870__generated,
            cfe_state_char_equip_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1998, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let widget = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 7,
                };
                c.bus.write32(widget + 0xc, (seed % 9) as u32);
                c.bus
                    .write32(widget + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x1c, r.next_u32());
                c.bus.write32(this + 0x5c, DATA + 0x2400);
                c.bus.write32(DATA + 0x2400 + 0x95, seed % 4);
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
                let _r = base_5(c, seed, 0x19af);
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
    fn bolt_file_index_manager_free_node_matches() {
        strict(
            ee::s03::f_0019ca70__generated,
            bolt_file_index_manager_free_node,
            80,
            &|c, seed| {
                let _r = base_5(c, seed, 0x19ca);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    (DATA + 0x1000) as u64
                };
                let node = DATA + 0x1000;
                c.bus.write32(node, (seed % 401) as u32);
                c.bus.write16(node + 8, 0xffff);
                c.bus.write16(node + 0xa, 0xffff);
                c.bus.write16(node + 0xc, 0xffff);
                c.bus.write16(node + 0xe, 0xffff);
                c.bus.write32(DATA + 0x1f40, 0);
            },
        );
    }

    #[test]
    fn char_equip_category_all_flagged_matches() {
        diff(
            ee::s03::f_0019b098__generated,
            c_fe_state_char_equip_category_all_flagged,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x19_b098);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = 0;
                c.g[A3][0] = (seed % 4) as u64;
                w(c, DATA + 0xbc, DATA + 0x3000);
                w(c, DATA + 0x3000, 0);
                w(c, DATA + 0x3004, r.next_u32());
            },
        );
    }

    #[test]
    fn cfestate_charequip_dtor_matches() {
        strict_2(
            ee::s03::f_00199148__generated,
            cfestate_charequip_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x9148, 0);
                let this = DATA + 0x100;
                let world = DATA + 0x8_0000;
                w(c, world + 0x7c, DATA + 0x2000);
                // no bolt-file bindings: the release paths are skipped
                w(c, this + 0xa64, 0xffff_ffff);
                w(c, this + 0xa68, 0xffff_ffff);
                w(c, this + 0xa70, 0xffff_ffff);
                w(c, this + 0xa74, 0xffff_ffff);
                w(c, this + 0xbc, r.next_u32() & 3);
                w(c, GAME_GP.wrapping_sub(0x854), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) const EQUIP: u32 = DATA + 0x1_0000;

    pub(crate) const EQUIP_TABLE: u32 = DATA + 0x1_1000;

    pub(crate) const EQUIP_NAME: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_char_equip_get_preview_texture_matches() {
        check_all(
            ee::s03::f_0019a098__generated,
            c_fe_state_char_equip_get_preview_texture,
            24,
            &|c, seed| {
                let _ = base_7(c, seed, 0x19a0_98, 8);
                c.g[A0][0] = sx(EQUIP);
                c.g[A1][0] = sx(EQUIP_NAME);
                c.bus.write32(EQUIP + 0xa60, (seed % 2) as u32);
                c.bus.write32(EQUIP + 0xbc, seed % 4);
                c.bus.write32(EQUIP + 0xa64, (seed / 2) % 3);
                c.bus.write32(EQUIP + 0xa68, 0xffff_ffff);
                c.bus.write8(EQUIP_NAME, b'a' + (seed % 3) as u8);
                // The equip table the package lookup starts from.
                c.bus.write32(WORLD_3 + 0x7c, EQUIP_TABLE);
                if seed % 3 == 0 {
                    c.bus.write32(EQUIP_TABLE + 0x1a70, DATA + 0x1_3000);
                }
            },
        );
    }

    // ------------------------------------------- preview camera / input / request

    /// The equip state at `DATA`, its rider preview module through the world
    /// (`gp-0x848 -> +0x7c`, preview 0 at `module + 0xb0`), the camera
    /// constants at `gp-0x59a0..-0x5970` and the preview fields.
    fn preview_camera_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x19_bfe8, 0);
        let st = DATA;
        let world = DATA + 0x8_0000;
        let module = DATA + 0x2000;
        let preview = module + 0xb0;
        w(c, GAME_GP - 0x848, world);
        w(c, world + 0x7c, module);
        for k in (0x5970u32..=0x59a0).step_by(4) {
            w(c, GAME_GP - k, r.f32(300.0));
        }
        // The two 0.2 glide factors / settle thresholds.
        w(c, GAME_GP - 0x5990, 0x3e4c_cccd);
        w(c, GAME_GP - 0x5984, 0x3e4c_cccd);
        c.bus.write8(st + 0x44, 0);
        w(c, preview, if seed % 9 == 0 { 10 } else { seed % 4 });
        w(c, preview + 0xcb8, if seed % 11 == 1 { 0 } else { 1 });
        w(c, preview + 0xcb4, if seed % 13 == 2 { 0 } else { 1 });
        let state = seed % 6;
        w(c, st + equip_preview::CAM_STATE, state);
        w(c, st + equip_preview::ZOOM, r.f32(1.0));
        w(c, st + equip_preview::PITCH, r.f32(1.0));
        w(c, st + equip_preview::YAW, r.f32(360.0));
        // Near the wrap sometimes.
        let spin = if seed % 3 == 0 {
            359.5f32.to_bits()
        } else {
            r.f32(360.0)
        };
        w(c, st + equip_preview::SPIN, spin);
        for i in 0..4 {
            w(c, st + equip_preview::CAM_A + 4 * i, r.f32(300.0));
            w(c, st + equip_preview::CAM_B + 4 * i, r.f32(300.0));
        }
        if seed % 4 == 1 {
            // Already at the view: the settle branches.
            if state == 3 {
                put_words(
                    c,
                    st + equip_preview::CAM_B,
                    &[
                        (-100f32).to_bits(),
                        45f32.to_bits(),
                        15f32.to_bits(),
                        1f32.to_bits(),
                    ],
                );
            } else {
                w(c, st + equip_preview::ZOOM, 0);
                w(c, st + equip_preview::PITCH, 0);
                put_words(
                    c,
                    st + equip_preview::CAM_A,
                    &[
                        (-105f32).to_bits(),
                        (-100f32).to_bits(),
                        (-80f32).to_bits(),
                        1f32.to_bits(),
                    ],
                );
            }
        }
        c.g[A0][0] = sx(st);
    }

    #[test]
    fn cfe_state_char_equip_update_preview_camera_matches() {
        check(
            ee::s03::f_0019bfe8__generated,
            cfe_state_char_equip_update_preview_camera,
            96,
            &|c, seed| preview_camera_setup(c, seed),
        );
    }

    /// Run a replacement once from the current state (returns to the
    /// sentinel `ra`).
    fn run_hand(c: &mut Cpu, f: GuestFn) {
        c.g[RA][0] = SENTINEL as u64;
        f(c);
        assert_eq!(c.pc, SENTINEL, "returned");
    }

    fn fbits(c: &mut Cpu, a: u32) -> f32 {
        f32::from_bits(c.bus.read32(a))
    }

    #[test]
    fn cfe_state_char_equip_update_preview_camera_rate() {
        // State 3 far from its view: one tick at 60 = two ticks at 120 for
        // the glide (both vectors) and the turntable spin.
        let run = |hz: u32| -> Vec<f32> {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            preview_camera_setup(c, 3);
            crate::tick::set_for_test(c, hz);
            let st = DATA;
            w(c, st + equip_preview::CAM_STATE, 3);
            w(c, st + equip_preview::SPIN, 10f32.to_bits());
            put_words(c, st + equip_preview::CAM_A, &[0, 0, 0, 1f32.to_bits()]);
            put_words(c, st + equip_preview::CAM_B, &[0, 0, 0, 1f32.to_bits()]);
            for _ in 0..hz / 60 {
                c.g[A0][0] = sx(st);
                run_hand(c, cfe_state_char_equip_update_preview_camera);
            }
            assert_eq!(
                c.bus.read32(st + equip_preview::CAM_STATE),
                3,
                "still gliding"
            );
            let mut v = vec![fbits(c, st + equip_preview::SPIN)];
            for i in 0..3 {
                v.push(fbits(c, st + equip_preview::CAM_A + 4 * i));
                v.push(fbits(c, st + equip_preview::CAM_B + 4 * i));
            }
            v
        };
        let a = run(60);
        assert!((a[0] - 11.0).abs() < 1e-5, "spin {}", a[0]);
        for hz in [120, 240] {
            let b = run(hz);
            for (x, y) in a.iter().zip(&b) {
                assert!(
                    (x - y).abs() <= 1e-3 * (1.0 + x.abs()),
                    "{hz} Hz: {a:?} vs {b:?}"
                );
            }
        }
    }

    /// Pad fakes for `HandleInput`: the vtable slots in the order
    /// [`PAD_SLOTS`] point at `FAKE + 0x10 * i`; the "active" queries
    /// (`0x118`, `0x108`, `0x110`) return 0/1, the stick reads -1..1 in f0.
    const PAD_SLOTS: [u32; 6] = [0x118, 0x130, 0x108, 0x120, 0x110, 0x128];

    fn pad_fake(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match target.wrapping_sub(FAKE) {
            0x00 | 0x20 | 0x40 => c.g[V0][0] = (h >> 7) & 1,
            _ => c.f[0] = (((h >> 8) & 0xffff) as f32 / 32768.0 - 1.0).to_bits(),
        }
    }

    fn pv_pad(c: &mut Cpu, r: &mut Rng, at_: u32) {
        let vt = at_ + 0x200;
        w(c, at_ + 8, vt);
        for (i, off) in PAD_SLOTS.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3 & 0xff) as u32;
            c.bus.write16(vt + off, delta as u16);
            w(c, vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    #[test]
    fn cfe_state_char_equip_handle_input_matches() {
        check(
            ee::s03::f_00199a38__generated,
            cfe_state_char_equip_handle_input,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19_9a38, 6);
                c.hooks.miss = pad_fake;
                let st = DATA;
                let p = DATA + 0x4000;
                pv_pad(c, &mut r, p);
                w(
                    c,
                    st + equip_preview::MODE,
                    if seed % 7 == 0 { 1 } else { seed % 3 },
                );
                w(
                    c,
                    st + equip_preview::CAM_STATE,
                    [1, 4, 1, 4, 2, 0][(seed % 6) as usize],
                );
                w(c, st + equip_preview::YAW, r.f32(1000.0));
                w(c, st + equip_preview::ZOOM, r.f32(1.2));
                w(c, st + equip_preview::PITCH, r.f32(1.2));
                w(c, GAME_GP - 0x59a8, 0xbdcc_cccd);
                w(c, GAME_GP - 0x59a4, 0xbdcc_cccd);
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(p);
            },
        );
    }

    /// Only the left stick, held at 0.5.
    fn pad_yaw(c: &mut Cpu, target: u32) {
        match target.wrapping_sub(FAKE) {
            0x00 => c.g[V0][0] = 1,
            0x10 => c.f[0] = 0.5f32.to_bits(),
            _ => c.g[V0][0] = 0,
        }
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn cfe_state_char_equip_handle_input_rate() {
        // The yaw turns as far in one 60 Hz tick as in rate/60 ticks.
        let run = |hz: u32| -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 1, 0x19_9a38, 0);
            c.hooks.miss = pad_yaw;
            crate::tick::set_for_test(c, hz);
            let st = DATA;
            let p = DATA + 0x4000;
            pv_pad(c, &mut r, p);
            w(c, st + equip_preview::MODE, 0);
            w(c, st + equip_preview::CAM_STATE, 1);
            w(c, st + equip_preview::YAW, 10f32.to_bits());
            for _ in 0..hz / 60 {
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(p);
                run_hand(c, cfe_state_char_equip_handle_input);
                assert_eq!(c.g[V0][0], 1);
            }
            fbits(c, st + equip_preview::YAW)
        };
        assert_eq!(run(60), 11.5);
        for hz in [120, 240] {
            assert!((run(hz) - 11.5).abs() < 1e-4, "{hz} Hz");
        }
    }

    /// The equip state at `EQUIP` with an item list, the item's model name,
    /// the BELibrary (`GetInterface(9)` a fake), the screen dummy at
    /// `gp-0x854` (its texture release a fake) and the game image (the bolt
    /// category path table and libc).
    fn request_preview_setup(c: &mut Cpu, seed: u32, calls: u32) {
        crate::difftest::elf_image(c);
        let mut r = crate::game::fe::fepopup::tests::base_14(c, seed, 0x19_a4e8, calls);
        use crate::game::fe::fepopup::tests::AUX;
        let st = EQUIP;
        let list = DATA + 0x1_4000;
        let item = DATA + 0x1_5000;
        let name = DATA + 0x1_6000;
        // BELibrary_GetInterface: index 0 of the interface table.
        w(c, GAME_GP - 0x1efc, 0);
        let iface = DATA + 0x1_7000;
        let vt = DATA + 0x1_7100;
        w(c, 0x0053_08a8, iface);
        w(c, iface + 4, vt);
        c.bus.write16(vt + 8, 0);
        w(c, vt + 0xc, FAKE);
        // The screen's texture release.
        let auxv = AUX + 0x2000;
        w(c, AUX + 0x10d8, auxv);
        c.bus.write16(auxv + 0x190, 0);
        w(c, auxv + 0x194, FAKE + 0x10);
        w(c, st + 0x5c, if seed % 7 == 0 { 0 } else { list });
        w(c, st + 0xa6c, u32::MAX);
        w(c, st + 0xa70, r.next_u32());
        w(
            c,
            st + 0xa74,
            if seed % 2 == 0 { u32::MAX } else { seed % 5 },
        );
        let idx = seed % 4;
        c.bus.write8(list + 0x95, idx as u8);
        w(c, st + 0x958, 3);
        w(c, st + 0x124 + 4 * idx, item);
        w(c, item + 0x30, if seed % 6 == 1 { 0 } else { name });
        cstr(
            c,
            name,
            ["hat07", "abc", "", "x9y", "12"][(seed % 5) as usize],
        );
        w(c, st + 0xbc, seed % 10);
        c.g[A0][0] = sx(st);
        c.g[A1][0] = (seed % 3 != 0) as u64;
    }

    #[test]
    fn cfe_state_char_equip_request_preview_matches() {
        check(
            ee::s03::f_0019a4e8__generated,
            cfe_state_char_equip_request_preview,
            60,
            &|c, seed| request_preview_setup(c, seed, 2),
        );
    }

    #[test]
    fn cfe_state_char_equip_request_preview_rate() {
        // The 20-frame delay as ticks: 20 at 60, 40 at 120, 80 at 240.
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            request_preview_setup(c, 5, 0);
            crate::tick::set_for_test(c, hz);
            c.g[A1][0] = 1;
            run_hand(c, cfe_state_char_equip_request_preview);
            assert_eq!(
                c.bus.read32(EQUIP + equip_preview::PREVIEW_DELAY),
                20 * hz / 60,
                "{hz} Hz"
            );
        }
    }

    use crate::difftest::fakes::vtable;
    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_m2;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2_m2;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::bb;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2 as world_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict as strict_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_4_m2;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::ovstates::tests::base_3 as base_6_m2;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_8;

    #[test]
    fn bolt_file_is_loaded_matches() {
        check_lohi(
            ssx3_ee::s03::f_0019e238,
            bolt_file_is_loaded,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0001 ^ seed as u64);
                scramble(c, &mut r);
                let outer = (seed % 2) as u32;
                let inner = ((seed / 2) % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = outer as u64;
                c.g[A2][0] = inner as u64;
                let base = DATA
                    .wrapping_add(outer.wrapping_mul(0x12000))
                    .wrapping_add(inner.wrapping_mul(0x120));
                c.bus
                    .write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_visible_matches() {
        check(
            ssx3_ee::s03::f_0019e538,
            femodule_rider_preview_set_visible,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) | 1
                };
                for (k, off) in [0xcc0u32, 0xcb8, 0xcb4, 0xcd4].iter().enumerate() {
                    let v = if (seed >> k) & 1 == 0 {
                        0
                    } else if k == 3 && (seed >> 6) & 1 == 0 {
                        0x8000_0000
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(DATA + off, v);
                }
                c.bus.write32(DATA + 0xcc4, r.next_u32());
                c.bus.write32(DATA + 0xcc8, r.next_u32());
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_get_next_node_matches() {
        check(
            ssx3_ee::s03::f_0019c8f8,
            bolt_file_index_manager_get_next_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x19c9 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.g[A1][0] = node as u64;
                if node != 0 {
                    let val = if seed % 2 == 0 {
                        r.next_u32() & 0x7fff
                    } else {
                        0x8000 | (r.next_u32() & 0x7fff)
                    };
                    c.bus.write16(node + 0xe, val as u16);
                }
            },
        );
    }

    // ---------------------------------------------------------- money text

    pub(crate) fn money_text_test(
        generated: GuestFn,
        hand: GuestFn,
        text_off: u32,
        value_off: u32,
        salt: u64,
    ) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            // The money formatter asks the BE library; a zeroed singleton
            // keeps it from lazily loading the rewards file from disc.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + text_off, 0);
            } else {
                c.bus.write32(DATA + text_off, DATA + 0x3000);
            }
            c.bus.write32(
                DATA + value_off,
                if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 100_000
                },
            );
            // The formatted string is a static buffer in the game image; the
            // text object at_m2 DATA + 0x3000 is only a target of the setter.
            c.bus.write32(DATA + 0x3000, 0);
        });
    }

    #[test]
    fn c_fe_state_char_equip_update_money_text_matches() {
        money_text_test(
            ssx3_ee::s03::f_0019bba0__generated,
            c_fe_state_char_equip_update_money_text,
            0x64,
            0xc0,
            0x1b,
        );
    }

    // -----------------------------------------------------------------------
    // 0x19a498 cFEStateCharEquip_ReleasePreview.

    #[test]
    fn c_fe_state_char_equip_release_preview_matches() {
        check(
            ee::s03::f_0019a498__generated,
            c_fe_state_char_equip_release_preview,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x19a4_98, 1);
                let this = DATA + 0x100;
                let world_m2 = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world_m2);
                c.bus.write32(world_m2 + 0x7c, DATA + 0x2000);
                let idx = if seed % 2 == 0 { 0xffff_ffffu32 } else { 2 };
                c.bus.write32(this + 0xa6c, idx);
                c.bus.write32(this + 0xbc, 0); // slot 0: offsets collapse to +0x120
                                               // A live refcount so BoltFile_Release only decrements it.
                let bolt = DATA + 0x2000 + 0x1a70;
                let node = bolt + 2 * 0x120;
                c.bus.write32(node + 0x114, 5);
                c.bus.write32(node, 0);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_refresh_item_info_matches() {
        check(
            ee::s03::f_0019a9b8__generated,
            cfe_state_char_equip_refresh_item_info,
            24,
            &|c, seed| {
                let _r = base(c, seed, 0x19a9);
                c.bus.write32(DATA + 0xc8, (seed % 3) as u32);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // ------------------------------------------------ riders, cards, misc

    #[test]
    fn c_fe_state_char_equip_refresh_rider_bolts_matches() {
        check(
            ee::s03::f_0019aca0__generated,
            c_fe_state_char_equip_refresh_rider_bolts,
            12,
            &|c, seed| {
                machine(c, seed, 0x19_aca0, 5);
                belib(c);
                world_m2(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                bb(c, DATA + 0x2000 + 0x44, (seed % 3) as u8);
                w(c, DATA + 0x2000 + 0xbc, 7 + seed % 3);
                c.pc = 0x0019_aca0;
            },
        );
    }

    pub(crate) fn fill_rows_test(gen: GuestFn, hand: GuestFn, salt: u64, row_target: u32) {
        check(gen, hand, 8, &|c, seed| {
            let _ = alloc_base(c, seed, salt, 4);
            let self_ = DATA;
            let profile = DATA + 0x1000;
            w(c, self_ + 0x5c, profile);
            wb(c, profile + 0x98, (seed % 4) as u8);
            w(c, self_ + 0x958, (seed % 10) as u32);
            let _ = row_target;
            c.g[A0][0] = self_ as u64;
        });
    }

    #[test]
    fn c_fe_state_char_equip_fill_equip_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b518__generated,
            c_fe_state_char_equip_fill_equip_rows,
            0x19b5_18,
            0x0019_b7e0,
        );
    }

    #[test]
    fn c_fe_state_char_equip_fill_buy_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b598__generated,
            c_fe_state_char_equip_fill_buy_rows,
            0x19b5_98,
            0x0019_ba60,
        );
    }

    #[test]
    fn bolt_file_index_manager_reset_matches() {
        check(
            ee::s03::f_0019cb60__generated,
            bolt_file_index_manager_reset,
            60,
            &|c, seed| {
                let _ = base_3_m2(c, seed, 0x19cb, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_open_buy_popup_matches() {
        check_lohi(
            ee::s03::f_0019c7e8__generated,
            cfe_state_char_equip_open_buy_popup,
            30,
            &|c, seed| {
                let mut r = base_4_m2(c, seed, 0x19c7e8, 4);
                let obj = DATA + 0x1000;
                let item = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let list = DATA + 0x4000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                w(c, obj + 0x10, screen);
                w(c, obj + 0xc0, r.next_u32());
                w8(c, obj + 0x44, seed as u8);
                w(c, screen + 0x18, list);
                w16(
                    c,
                    item + 0xe,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (seed % 0x100) as u16
                    },
                );
                w(c, item + 0x14, r.next_u32());
            },
        );
    }

    // ------------------------------------------------ more front end / scripts

    #[test]
    fn cfe_state_char_equip_set_category_title_matches() {
        check(
            ee::s03::f_0019bbe0__generated,
            cfe_state_char_equip_set_category_title,
            9,
            &|c, seed| {
                let _ = base_5_m2(c, seed, 0x0019_bbe0, 3);
                w(c, GAME_GP.wrapping_sub(0x1f20), 0x1234);
                c.g[A0][0] = OBJ_m2 as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0x1234
                } else {
                    (0x1000 + seed * 7) as u64
                };
                w(c, OBJ_m2 + 0x58, if seed % 4 == 0 { 0 } else { OBJ2 });
                w(c, OBJ_m2 + 0xbc, (seed % 5) as u32);
                w(c, OBJ2 + 0x18, 0);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_update_stat_bar_matches() {
        check(
            ee::s03::f_0019bc90__generated,
            cfe_state_char_equip_update_stat_bar,
            8,
            &|c, seed| {
                let mut r = base_5_m2(c, seed, 0x0019_bc90, 2);
                c.g[A0][0] = OBJ_m2 as u64;
                c.g[GP][0] = GAME_GP as u64;
                w(c, GAME_GP.wrapping_sub(0x1818), 0x5dc + (seed % 200));
                w(c, OBJ_m2 + 0xc4, 0x5dc + (seed % 300));
                w(c, OBJ_m2 + 0x70, if seed % 4 == 0 { 0 } else { OBJ2 });
                let vt = vtable(c, &mut r, &[0x48]);
                w(c, OBJ2 + 8, vt);
                w(c, OBJ2 + 0x50, 0);
                w(c, OBJ2 + 0x54, 0);
                w(c, OBJ2 + 0x58, 0);
                c.f[20] = r.edge_f32();
            },
        );
    }

    // -------------------------------------------------------------- FE lists.

    #[test]
    fn cfe_state_char_equip_remove_list_item_matches() {
        check(
            ee::s03::f_0019b458__generated,
            cfe_state_char_equip_remove_list_item,
            200,
            &|c, seed| {
                let mut r = base_6_m2(c, seed, 0x19_b458);
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
            },
        );
    }

    #[test]
    fn bolt_file_cache_shutdown_matches() {
        strict_m2(
            ee::s03::f_0019cf40__generated,
            bolt_file_cache_shutdown,
            6,
            &|c, _seed| {
                let _r = machine(c, 0, 0x19cf, 8);
                c.bus.write32(GAME_GP - 0x1788, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_buy_mode_matches() {
        check(
            ee::s03::f_0019a308__generated,
            cfe_state_char_equip_show_buy_mode,
            SEEDS_m2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19a3, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0x80u32, 0x6c, 0x60].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let vt = vtable(c, &mut r, &[0x48]);
                        c.bus.write32(o + 8, vt);
                    }
                }
                c.bus.write32(this + 0x44, seed % 4);
                c.bus.write32(WORLD_2 + 0x7c, DATA + 0x2c00);
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
                let _r = base_7_m2(c, seed, 0x19a2);
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
    fn cfe_state_char_equip_toggle_buy_equip_matches() {
        strict_2_m2(
            ee::s03::f_0019a3d0__generated,
            cfe_state_char_equip_toggle_buy_equip,
            120,
            &|c, seed| {
                let _r = base_7_m2(c, seed, 0x19a3);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc8, (seed & 1) as u32);
                if seed % 4 == 0 {
                    c.bus.write32(DATA + 0x50, 0);
                } else {
                    c.bus.write32(DATA + 0x50, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                    c.bus.write16(DATA + 0x2100 + 0x48, 0);
                    c.bus.write32(DATA + 0x2100 + 0x4c, FAKE);
                }
                for off in [0x48u32, 0x60, 0x80, 0x88, 0x6c] {
                    c.bus.write32(DATA + off, 0);
                }
            },
        );
    }

    #[test]
    fn boltfile_cache_init_matches() {
        check_lohi(
            ee::s03::f_0019ce68__generated,
            boltfile_cache_init,
            8,
            &|c, seed| {
                let _ = base_8(c, seed, 0x19ce68, 2);
                // A large private region for the 10x256 entry table.
                let base_at = 0x0100_0000u32;
                c.g[A0][0] = base_at as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1788),
                    if seed % 2 == 0 { DATA + 0x2_0000 } else { 0 },
                );
                clear_m2(c, DATA + 0x2_0000, 0x3000);
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_find_node_matches() {
        check(
            ee::s03::f_0019c938__generated,
            bolt_file_index_manager_find_node,
            300,
            &|c, seed| {
                let mut r = Rng(0xc938 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0019_c938;
                let m = DATA;
                let node = |i: u32| m + 0x10 * i;
                let hi = r.next_u32() % 0x80;
                let lo = r.next_u32() % 0x10000;
                let key = hi << 16 | lo;
                let b = key % 400;
                for i in 0..400 {
                    c.bus.write32(BOLT_INDEX_BUCKETS + m + 4 * i, 0);
                }
                // The bucket's chain: nodes 10, 20, 30.. (one may hold the key).
                let len = seed % 4;
                let hit = r.next_u32() % 4;
                for k in 0..len {
                    let n = node(10 * (k + 1));
                    c.bus.write32(
                        n,
                        if k == hit {
                            key
                        } else {
                            key ^ (1 + r.next_u32() % 0xffff)
                        },
                    );
                    c.bus
                        .write16(n + 0xc, if k == 0 { 0xffff } else { (10 * k) as u16 });
                    c.bus.write16(
                        n + 0xe,
                        if k + 1 < len {
                            (10 * (k + 2)) as u16
                        } else {
                            0xffff
                        },
                    );
                }
                c.bus.write32(
                    m + BOLT_INDEX_BUCKETS + 4 * b,
                    if len == 0 { 0 } else { node(10) },
                );
                // The free list: nodes 300, 301 (or none).
                let free = (seed / 4) % 3;
                for k in 0..free {
                    let n = node(300 + k);
                    fill(c, &mut r, n, 3);
                    c.bus.write16(
                        n + 0xe,
                        if k + 1 < free {
                            (301 + k) as u16
                        } else {
                            0xffff
                        },
                    );
                }
                c.bus
                    .write32(m + BOLT_INDEX_FREE, if free == 0 { 0 } else { node(300) });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() as u64) << 32 | hi as u64;
                c.g[A2][0] = lo as u64;
                c.g[A3][0] = ((seed / 12) % 2) as u64;
            },
        );
    }

    /// Gate of [`bolt_cu_fake_is_done`] and its answer/log area.
    pub(crate) const BOLT_CU_CTRL: u32 = DATA + 0xc_0000;

    pub(crate) const BOLT_CU_MAGIC: u32 = 0xb017_d250;

    /// `AsyncFile_IsDone` (0x3df980) for the cache update test: logs the
    /// handle and `ra`, answers 0 or 2 from the handle's low bit (never 1:
    /// the finished-read path runs the real release and unpack chain).
    pub(crate) fn bolt_cu_fake_is_done(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(BOLT_CU_CTRL) != BOLT_CU_MAGIC {
            return fall_back(
                c,
                prev_cell(bolt_cu_fake_is_done),
                ee::s14::f_003df980__generated,
            );
        }
        let n = c.bus.read32(BOLT_CU_CTRL + 4);
        c.bus.write32(BOLT_CU_CTRL + 4, n + 1);
        let at = BOLT_CU_CTRL + 0x10 + 8 * (n & 0x3ff);
        c.bus.write32(at, c.g[A0][0] as u32);
        c.bus.write32(at + 4, c.g[RA][0] as u32);
        c.g[V0][0] = (c.g[A0][0] & 1) * 2;
        c.cycles += 9;
        ret(c);
    }

    /// BoltFile_CacheUpdate: all 10 tables; pending slots (loading or not)
    /// whose read is not done (a fake IsDone), loads pending or not, and
    /// queued slots only while loads are pending. Not reached: a finished
    /// read (the real release/unpack chain) and starting a queued slot (the
    /// real BoltFile_LoadSlot): both need the whole streaming setup.
    #[test]
    fn bolt_file_cache_update_matches() {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            bolt_cu_fake_is_done,
            prev_cell(bolt_cu_fake_is_done),
        );
        check(
            ee::s03::f_0019d250__generated,
            bolt_file_cache_update,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19d250, 0);
                c.pc = 0x0019_d250;
                let this = DATA;
                c.g[A0][0] = sx(this);
                c.bus.write32(BOLT_CU_CTRL, BOLT_CU_MAGIC);
                let loads = if seed % 2 == 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write32(this + BOLT_CACHE_PENDING, loads);
                for k in 0..64 {
                    let i = r.next_u32() % BOLT_CACHE_TABLES;
                    let j = r.next_u32() % 256;
                    let slot = this + i * BOLT_CACHE_TABLE_SIZE + j * BOLT_CACHE_SLOT_SIZE;
                    let mut state = r.next_u32() % 5;
                    if state == 3 && loads == 0 {
                        state = 4;
                    }
                    c.bus.write32(slot + BOLT_SLOT_STATE, state);
                    c.bus.write32(slot + BOLT_SLOT_PENDING, (k % 3 != 0) as u32);
                    c.bus.write32(slot + BOLT_SLOT_HANDLE, r.next_u32());
                }
            },
        );
    }

    /// Gate of [`bolt_fs_fake_mem_free`] and its log.
    pub(crate) const BOLT_FS_CTRL: u32 = DATA + 0xc_8000;

    pub(crate) const BOLT_FS_MAGIC: u32 = 0xb017_d4b8;

    /// `EA_MemFree` (0x317e98) for the free-slot test: logs the block and
    /// `ra`.
    pub(crate) fn bolt_fs_fake_mem_free(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(BOLT_FS_CTRL) != BOLT_FS_MAGIC {
            return fall_back(
                c,
                prev_cell(bolt_fs_fake_mem_free),
                ee::s10::f_00317e98__generated,
            );
        }
        c.bus.write32(BOLT_FS_CTRL + 4, c.g[A0][0] as u32);
        c.bus.write32(BOLT_FS_CTRL + 8, c.g[RA][0] as u32);
        c.g[V0][0] = 0x1234;
        c.cycles += 13;
        ret(c);
    }

    /// BoltFile_FreeSlot: loading with or without a pending read (the real
    /// release chain on an unknown handle: mutex syscalls are fakes), other
    /// states, with or without data (a fake EA_MemFree).
    #[test]
    fn bolt_file_free_slot_matches() {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            bolt_fs_fake_mem_free,
            prev_cell(bolt_fs_fake_mem_free),
        );
        check(
            ee::s03::f_0019d4b8__generated,
            bolt_file_free_slot,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19d4b8, 0);
                c.pc = 0x0019_d4b8;
                c.bus.write32(BOLT_FS_CTRL, BOLT_FS_MAGIC);
                let this = DATA;
                let table = r.next_u32() % BOLT_CACHE_TABLES;
                let index = r.next_u32() % 256;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(table);
                c.g[A2][0] = sx(index);
                let slot = this + table * BOLT_CACHE_TABLE_SIZE + index * BOLT_CACHE_SLOT_SIZE;
                for off in (0..BOLT_CACHE_SLOT_SIZE).step_by(4) {
                    c.bus.write32(slot + off, r.next_u32());
                }
                c.bus.write32(
                    slot + BOLT_SLOT_STATE,
                    if seed % 2 == 0 { 2 } else { seed % 5 },
                );
                c.bus
                    .write32(slot + BOLT_SLOT_PENDING, (seed % 3 != 0) as u32);
                c.bus.write32(
                    slot + BOLT_SLOT_DATA,
                    if seed % 4 == 0 { 0 } else { DATA + 0xd_0000 },
                );
                c.bus.write32(this + BOLT_CACHE_BYTES, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::LOG_RESULTS;

    pub(crate) use crate::game::ui::uiengine::tests::oc726_hook;
    pub(crate) use crate::game::ui::uiengine::tests::CTRL;
    pub(crate) use crate::game::ui::uiengine::tests::CTRL_MAGIC;
    pub(crate) use crate::game::ui::uiengine::tests::SEEDS_10 as SEEDS_3;

    use std::sync::atomic::AtomicUsize;

    #[test]
    fn bolt_file_compare_model_slots_matches() {
        check(
            ee::s03::f_0019dd10__generated,
            bolt_file_compare_model_slots_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x0019_dd10 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0019_dd10;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
                let table = DATA + 0x0020_0000;
                let k = seed % 3;
                let j = 3 + seed % 3;
                c.bus.write32(DATA, k);
                c.bus.write16(DATA + 0x40, j as u16);
                c.bus.write32(GAME_GP - 0x1784, table);
                fill(c, &mut r, table, 0x120 * 8 / 4);
                let a = table + k * 0x120;
                let b = table + j * 0x120;
                // Modes walk the first pass, the second pass and the names.
                let (a0, a110, a11c, b0, b110, b11c) = match seed % 8 {
                    0 => (0, 8, 0, 3, 8, 0),
                    1 => (3, 0, 0, 3, 8, 0),
                    2 => (3, 8, 1, 3, 8, 0),
                    3 => (3, 8, 0, 4, 8, 0),
                    4 => (3, 8, 0, 3, 0, 0),
                    5 => (3, 8, 0, 3, 8, 1),
                    6 => (3, 8, 0, 3, 8, 0),
                    _ => (9, 8, 0, 3, 8, 0),
                };
                c.bus.write32(a, a0);
                c.bus.write32(a + 0x110, a110);
                c.bus.write32(a + 0x11c, a11c);
                c.bus.write32(b, b0);
                c.bus.write32(b + 0x110, b110);
                c.bus.write32(b + 0x11c, b11c);
                // The names at +4 differ in the third letter.
                c.bus.write32(a + 4, u32::from_le_bytes(*b"ABC\0"));
                c.bus.write32(b + 4, u32::from_le_bytes(*b"ABD\0"));
            },
        );
    }

    #[test]
    fn bolt_file_release_matches() {
        oc726_hook();
        check(
            ee::s03::f_0019dc20__generated,
            bolt_file_release,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 728, 1);
                // The first callee of the state-2 branch returns 2 (not the
                // unpack branch's 1, which would run the real unpacker).
                c.bus.write32(LOG_RESULTS, 2);
                let slot = DATA + 0x1000;
                c.g[A0][0] = slot as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.bus.write32(slot + 0x114, seed % 4);
                c.bus
                    .write32(slot, if seed % 3 == 0 { 2 } else { r.next_u32() });
                c.bus
                    .write32(slot + 0x104, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(slot + 0x108, DATA + 0x2000);
                // The slot table's use count at the table base (slot + 0xb0000).
                c.bus.write32(slot + 0xb0000 + 0x4004, 5);
                c.pc = 0x0019_dc20;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    pub(crate) const OC737_FX_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC737_FX_MAGIC: u32 = 0x4f43_3737;

    pub(crate) const OC737_FX_MGR: u32 = DATA + 0x30_0000;

    pub(crate) const OC737_FX_NAME: u32 = DATA + 0x3c_0000;

    // Result queues of the fakes, inside the control block: `[next, len,
    // results...]`.
    pub(crate) const OC737_Q_RIDER: u32 = 0x100;

    pub(crate) const OC737_Q_FIND: u32 = 0x140;

    pub(crate) const OC737_Q_FREE: u32 = 0x180;

    pub(crate) const OC737_Q_TABLE: u32 = 0x1c0;

    pub(crate) const OC737_Q_TEXT: u32 = 0x200;

    macro_rules! oc737_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC737_FX_CTRL) != OC737_FX_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, queue $q:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC737_FX_CTRL) != OC737_FX_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                // The next queued result, if any (else the fake's own).
                let base = OC737_FX_CTRL + $q;
                let next = c.bus.read32(base);
                c.bus.write32(base, next + 1);
                if next < c.bus.read32(base + 4) {
                    let v = c.bus.read32(base + 8 + 4 * next);
                    c.g[V0][0] = sx(v);
                }
            }
        };
    }

    oc737_fake!(oc737_fake_rider, OC737_PREV_RIDER, ee::s03::f_001a0548__generated, 0x001a_0548, queue OC737_Q_RIDER);

    oc737_fake!(
        oc737_fake_set_pose,
        OC737_PREV_SET_POSE,
        ee::s03::f_0019f3e8__generated,
        0x0019_f3e8
    );

    oc737_fake!(
        oc737_fake_set_board,
        OC737_PREV_SET_BOARD,
        ee::s03::f_0019f548__generated,
        0x0019_f548
    );

    oc737_fake!(
        oc737_fake_set_visible,
        OC737_PREV_SET_VISIBLE,
        ee::s03::f_0019e538__generated,
        0x0019_e538
    );

    oc737_fake!(oc737_fake_find_name, OC737_PREV_FIND_NAME, ee::s03::f_0019d428__generated, 0x0019_d428, queue OC737_Q_FIND);

    oc737_fake!(oc737_fake_find_free, OC737_PREV_FIND_FREE, ee::s03::f_0019d3b0__generated, 0x0019_d3b0, queue OC737_Q_FREE);

    oc737_fake!(
        oc737_fake_quat,
        OC737_PREV_QUAT,
        ee::s10::f_0031be50__generated,
        0x0031_be50
    );

    oc737_fake!(
        oc737_fake_anim_reset,
        OC737_PREV_ANIM_RESET,
        ee::s10::f_00311a50__generated,
        0x0031_1a50
    );

    oc737_fake!(
        oc737_fake_anim_play,
        OC737_PREV_ANIM_PLAY,
        ee::s10::f_003128e8__generated,
        0x0031_28e8
    );

    oc737_fake!(
        oc737_fake_hash,
        OC737_PREV_HASH,
        ee::s10::f_00317670__generated,
        0x0031_7670
    );

    oc737_fake!(
        oc737_fake_alloc,
        OC737_PREV_ALLOC,
        ee::s10::f_00317f60__generated,
        0x0031_7f60
    );

    oc737_fake!(
        oc737_fake_open,
        OC737_PREV_OPEN,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );

    oc737_fake!(
        oc737_fake_be,
        OC737_PREV_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );

    oc737_fake!(
        oc737_fake_iface,
        OC737_PREV_IFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );

    oc737_fake!(oc737_fake_uber, OC737_PREV_UBER, ee::s01::f_0014ff90__generated, 0x0014_ff90, queue OC737_Q_TABLE);

    oc737_fake!(
        oc737_fake_money,
        OC737_PREV_MONEY,
        ee::s01::f_00150928__generated,
        0x0015_0928
    );

    oc737_fake!(
        oc737_fake_char_id,
        OC737_PREV_CHAR_ID,
        ee::s01::f_00147488__generated,
        0x0014_7488
    );

    oc737_fake!(
        oc737_fake_list_first,
        OC737_PREV_LIST_FIRST,
        ee::s13::f_00397840__generated,
        0x0039_7840
    );

    oc737_fake!(oc737_fake_by_hash, OC737_PREV_BY_HASH, ee::s13::f_0039d860__generated, 0x0039_d860, queue OC737_Q_TEXT);

    oc737_fake!(
        oc737_fake_set_text,
        OC737_PREV_SET_TEXT,
        ee::s13::f_003a0c70__generated,
        0x003a_0c70
    );

    oc737_fake!(
        oc737_fake_release,
        OC737_PREV_RELEASE,
        ee::s14::f_003df9d8__generated,
        0x003d_f9d8
    );

    oc737_fake!(
        oc737_fake_read,
        OC737_PREV_READ,
        ee::s14::f_003df748__generated,
        0x003d_f748
    );

    oc737_fake!(
        oc737_fake_wait,
        OC737_PREV_WAIT,
        ee::s14::f_003e1bb8__generated,
        0x003e_1bb8
    );

    oc737_fake!(
        oc737_fake_strcpy,
        OC737_PREV_STRCPY,
        ee::s15::f_004166f4__generated,
        0x0041_66f4
    );

    oc737_fake!(
        oc737_fake_text_draw,
        OC737_PREV_TEXT_DRAW,
        ee::s12::f_00391e30__generated,
        0x0039_1e30
    );

    oc737_fake!(
        oc737_fake_font_text,
        OC737_PREV_FONT_TEXT,
        ee::s12::f_00391cb0__generated,
        0x0039_1cb0
    );

    pub(crate) static OC737_PREV_FREE_SLOT: AtomicUsize = AtomicUsize::new(0);

    /// `BoltFile_FreeSlot` fake: logs the call like the others, then does
    /// what the real one's callers rely on: the slot loses its data pointer
    /// and the cache's byte count drops by the control block's `+4` word.
    pub(crate) fn oc737_fake_free_slot(c: &mut Cpu) {
        if c.bus.read32(OC737_FX_CTRL) != OC737_FX_MAGIC {
            return fakes::fall_back(c, &OC737_PREV_FREE_SLOT, ee::s03::f_0019d4b8__generated);
        }
        let mgr = c.g[A0][0] as u32;
        let bank = c.g[A1][0] as u32;
        let index = c.g[A2][0] as u32;
        let slot = mgr
            .wrapping_add(bank.wrapping_mul(0x12000))
            .wrapping_add(index.wrapping_mul(0x120));
        fakes::fake_call(c, 0x0019_d4b8);
        c.bus.write32(slot.wrapping_add(0x104), 0);
        let used = c.bus.read32(mgr.wrapping_add(0xb4000));
        let drop = c.bus.read32(OC737_FX_CTRL + 4);
        c.bus
            .write32(mgr.wrapping_add(0xb4000), used.wrapping_sub(drop));
    }

    macro_rules! oc737_hook {
        ($shard:ident, $slot:ident, $fake:ident, $prev:ident) => {
            fakes::hook(std::ptr::addr_of_mut!(ee::$shard::$slot), $fake, &$prev);
        };
    }

    pub(crate) fn oc737_install_fakes() {
        oc737_hook!(s03, OVERRIDE_001A0548, oc737_fake_rider, OC737_PREV_RIDER);
        oc737_hook!(
            s03,
            OVERRIDE_0019F3E8,
            oc737_fake_set_pose,
            OC737_PREV_SET_POSE
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019F548,
            oc737_fake_set_board,
            OC737_PREV_SET_BOARD
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019E538,
            oc737_fake_set_visible,
            OC737_PREV_SET_VISIBLE
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019D428,
            oc737_fake_find_name,
            OC737_PREV_FIND_NAME
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019D3B0,
            oc737_fake_find_free,
            OC737_PREV_FIND_FREE
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019D4B8,
            oc737_fake_free_slot,
            OC737_PREV_FREE_SLOT
        );
        oc737_hook!(s10, OVERRIDE_0031BE50, oc737_fake_quat, OC737_PREV_QUAT);
        oc737_hook!(
            s10,
            OVERRIDE_00311A50,
            oc737_fake_anim_reset,
            OC737_PREV_ANIM_RESET
        );
        oc737_hook!(
            s10,
            OVERRIDE_003128E8,
            oc737_fake_anim_play,
            OC737_PREV_ANIM_PLAY
        );
        oc737_hook!(s10, OVERRIDE_00317670, oc737_fake_hash, OC737_PREV_HASH);
        oc737_hook!(s10, OVERRIDE_00317F60, oc737_fake_alloc, OC737_PREV_ALLOC);
        oc737_hook!(s10, OVERRIDE_00317E30, oc737_fake_open, OC737_PREV_OPEN);
        oc737_hook!(s01, OVERRIDE_0014DC80, oc737_fake_be, OC737_PREV_BE);
        oc737_hook!(s01, OVERRIDE_0014DD58, oc737_fake_iface, OC737_PREV_IFACE);
        oc737_hook!(s01, OVERRIDE_0014FF90, oc737_fake_uber, OC737_PREV_UBER);
        oc737_hook!(s01, OVERRIDE_00150928, oc737_fake_money, OC737_PREV_MONEY);
        oc737_hook!(
            s01,
            OVERRIDE_00147488,
            oc737_fake_char_id,
            OC737_PREV_CHAR_ID
        );
        oc737_hook!(
            s13,
            OVERRIDE_00397840,
            oc737_fake_list_first,
            OC737_PREV_LIST_FIRST
        );
        oc737_hook!(
            s13,
            OVERRIDE_0039D860,
            oc737_fake_by_hash,
            OC737_PREV_BY_HASH
        );
        oc737_hook!(
            s13,
            OVERRIDE_003A0C70,
            oc737_fake_set_text,
            OC737_PREV_SET_TEXT
        );
        oc737_hook!(
            s14,
            OVERRIDE_003DF9D8,
            oc737_fake_release,
            OC737_PREV_RELEASE
        );
        oc737_hook!(s14, OVERRIDE_003DF748, oc737_fake_read, OC737_PREV_READ);
        oc737_hook!(s14, OVERRIDE_003E1BB8, oc737_fake_wait, OC737_PREV_WAIT);
        oc737_hook!(s15, OVERRIDE_004166F4, oc737_fake_strcpy, OC737_PREV_STRCPY);
        oc737_hook!(
            s12,
            OVERRIDE_00391E30,
            oc737_fake_text_draw,
            OC737_PREV_TEXT_DRAW
        );
        oc737_hook!(
            s12,
            OVERRIDE_00391CB0,
            oc737_fake_font_text,
            OC737_PREV_FONT_TEXT
        );
    }

    /// A machine with the fakes installed, gated on, and `calls` as the range
    /// of fake-call indices at which a call may unwind.
    pub(crate) fn oc737_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc737_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC737_FX_CTRL, OC737_FX_MAGIC);
        r
    }

    /// Queues the results the next calls of a queued fake return.
    pub(crate) fn oc737_queue(c: &mut Cpu, q: u32, results: &[u32]) {
        c.bus.write32(OC737_FX_CTRL + q, 0);
        c.bus.write32(OC737_FX_CTRL + q + 4, results.len() as u32);
        for (i, v) in results.iter().enumerate() {
            c.bus.write32(OC737_FX_CTRL + q + 8 + 4 * i as u32, *v);
        }
    }

    /// The bolt file manager of the cache tests: slot `index` of `bank`.
    pub(crate) fn oc737_slot(bank: u32, index: u32) -> u32 {
        OC737_FX_MGR + bank * 0x12000 + index * 0x120
    }

    /// `BoltFile_EnsureBudget`: bytes in use around the budget, a handful of
    /// slots in the banks (idle ones with data, and decoys that are
    /// referenced, busy or empty), and a free that lowers the byte count by
    /// nothing, a little or everything.
    #[test]
    fn bolt_file_ensure_budget_matches() {
        fakes::covered(
            ee::s03::f_0019d578__generated,
            bolt_file_ensure_budget,
            200,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0019_d578, 8);
                let bank = r.next_u32() % 10;
                let drop = [0u32, 700_000, 100_000_000][(r.next_u32() % 3) as usize];
                c.bus.write32(OC737_FX_CTRL + 4, drop);
                let used = match seed % 4 {
                    0 => 0,
                    1 => 9_999_000,
                    _ => 10_000_000 + r.next_u32() % 5000,
                };
                c.bus.write32(OC737_FX_MGR + 0xb4000, used);
                for k in 0..10u32 {
                    let b = if k < 4 { bank } else { r.next_u32() % 10 };
                    let slot = oc737_slot(b, r.next_u32() % 256);
                    c.bus.write32(slot + 0x104, 1 + r.next_u32() % 1000);
                    c.bus.write32(slot + 0x11c, r.next_u32() >> 4);
                    match r.next_u32() % 4 {
                        0 => c.bus.write32(slot, 1),
                        1 => c.bus.write32(slot + 0x114, 1),
                        2 => c.bus.write32(slot + 0x104, 0),
                        _ => {}
                    }
                }
                c.g[A0][0] = sx(OC737_FX_MGR);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = (r.next_u32() % 3000) as u64;
                c.pc = 0x0019_d578;
            },
        );
    }

    /// `BoltFile_Acquire`: a cached slot of that name in each state, or a new
    /// one; async and deferred requests; load threads or none.
    #[test]
    fn bolt_file_acquire_matches() {
        fakes::covered(
            ee::s03::f_0019da10__generated,
            bolt_file_acquire,
            300,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0019_da10, 12);
                let bank = r.next_u32() % 10;
                let found = r.next_u32() % 256;
                let free = r.next_u32() % 256;
                c.bus.write32(OC737_FX_CTRL + 4, 0);
                c.bus
                    .write32(OC737_FX_MGR + 0xb4000, r.next_u32() % 1_000_000);
                let threads = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write32(OC737_FX_MGR + 0xb4004, threads);
                c.bus
                    .write32(OC737_FX_MGR + 0xb4008 + 4 * bank, r.next_u32() % 100);
                for slot in [oc737_slot(bank, found), oc737_slot(bank, free)] {
                    c.bus.write32(slot, r.next_u32() % 4);
                    c.bus.write32(slot + 0x114, r.next_u32() % 4);
                    c.bus.write32(slot + 0x108, r.next_u32());
                    c.bus.write32(slot + 0x10c, r.next_u32() % 3);
                    c.bus.write32(slot + 0x11c, r.next_u32());
                }
                let hit = if (seed / 4) % 3 != 0 { found } else { u32::MAX };
                oc737_queue(c, OC737_Q_FIND, &[hit]);
                oc737_queue(c, OC737_Q_FREE, &[free]);
                c.g[A0][0] = sx(OC737_FX_MGR);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = sx(OC737_FX_NAME);
                c.g[A3][0] = (r.next_u32() % 8) as u64;
                c.g[T0][0] = (seed & 1) as u64;
                c.g[T1][0] = ((seed >> 1) & 1) as u64;
                c.pc = 0x0019_da10;
            },
        );
    }

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const FX_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const FX_MAGIC: u32 = 0x4f43_3734;

    /// Allocation fake: calls made (`+0x10`) and calls that fail first (`+0x14`).
    pub(crate) const FX_ALLOC_COUNT: u32 = FX_CTRL + 0x10;

    pub(crate) const FX_ALLOC_FAILS: u32 = FX_CTRL + 0x14;

    pub(crate) const FX_PACKED: u32 = DATA + 0x24_0000;

    /// The Bolt file table (`cFEBoltFileManager`): 10 banks of 256 slots,
    /// then the memory counters (`+0xb4000` bytes used, `+0xb4004` async loads).
    pub(crate) const FX_TABLE: u32 = DATA + 0x30_0000;

    /// The fake callee: logs the call like `fakes::fake_call`, then gives the
    /// results the ported code's branches need: `GetHashValue32` (an
    /// injective hash: static strings hash to their address, the key buffer
    /// on the stack to `0x70000000 + first byte`), a file size from
    /// `BXFILE_sizez`, and an allocator that fails its first calls.
    pub(crate) fn oc748_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0031_7670 => {
                c.g[V0][0] = if a0 >= 0x0100_0000 {
                    let b = c.bus.read8(a0);
                    0x7000_0000 + b as u64
                } else {
                    a0 as u64
                };
            }
            0x0031_7f60 => c.g[V0][0] = 0x1000 + (a0 & 0xff0) as u64,
            0x0031_7e30 => {
                let n = c.bus.read32(FX_ALLOC_COUNT);
                c.bus.write32(FX_ALLOC_COUNT, n + 1);
                if n < c.bus.read32(FX_ALLOC_FAILS) {
                    c.g[V0][0] = 0;
                }
            }
            _ => {}
        }
    }

    macro_rules! oc748_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc748_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc748_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc748_fakes!(
        (
            oc748_fake_317670,
            OC748_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc748_fake_14dc80,
            OC748_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc748_fake_14dd58,
            OC748_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc748_fake_14eec8,
            OC748_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc748_fake_3a1000,
            OC748_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc748_fake_3a0c70,
            OC748_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc748_fake_3b47f8,
            OC748_PREV_3B47F8,
            ee::s13::OVERRIDE_003B47F8,
            ee::s13::f_003b47f8__generated,
            0x003b_47f8
        ),
        (
            oc748_fake_317e30,
            OC748_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc748_fake_317e98,
            OC748_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc748_fake_317f60,
            OC748_PREV_317F60,
            ee::s10::OVERRIDE_00317F60,
            ee::s10::f_00317f60__generated,
            0x0031_7f60
        ),
        (
            oc748_fake_3e5700,
            OC748_PREV_3E5700,
            ee::s14::OVERRIDE_003E5700,
            ee::s14::f_003e5700__generated,
            0x003e_5700
        ),
        (
            oc748_fake_3e5760,
            OC748_PREV_3E5760,
            ee::s14::OVERRIDE_003E5760,
            ee::s14::f_003e5760__generated,
            0x003e_5760
        ),
        (
            oc748_fake_3df748,
            OC748_PREV_3DF748,
            ee::s14::OVERRIDE_003DF748,
            ee::s14::f_003df748__generated,
            0x003d_f748
        ),
        (
            oc748_fake_3e1bb8,
            OC748_PREV_3E1BB8,
            ee::s14::OVERRIDE_003E1BB8,
            ee::s14::f_003e1bb8__generated,
            0x003e_1bb8
        ),
    );

    /// Machine state of the four tests: scrambled registers, fake hooks and
    /// their gate, the allocator's failure count, and the junk below `sp`.
    pub(crate) fn oc748_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(FX_CTRL, FX_MAGIC);
        c.bus.write32(FX_ALLOC_COUNT, 0);
        c.bus.write32(FX_ALLOC_FAILS, (seed / 7) % 3);
        for i in 1..=0x300u32 {
            let w = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(crate::difftest::SP - 4 * i, w);
        }
        r
    }

    /// Address of slot `idx` of `bank` in the fixture table.
    pub(crate) fn oc748_slot(bank: u32, idx: u32) -> u32 {
        FX_TABLE + bank * 0x12000 + idx * 0x120
    }

    /// A queued slot: `flags`, a load-time stamp (`+0x11c`), a data pointer
    /// (never with flags 0, which would send the real `EvictBytes` into
    /// `FreeSlot`), the async flag, and sizes.
    pub(crate) fn oc748_put_slot(
        c: &mut Cpu,
        r: &mut Rng,
        bank: u32,
        idx: u32,
        flags: u32,
        stamp: u32,
    ) {
        let s = oc748_slot(bank, idx);
        c.bus.write32(s, flags);
        c.bus
            .write32(s + 0x104, 0x0090_0000 + (r.next_u32() & 0xfff0));
        c.bus.write32(s + 0x10c, r.next_u32() & 1);
        c.bus.write32(s + 0x114, r.next_u32() & 1);
        c.bus.write32(s + 0x118, r.next_u32() & 0xffff);
        c.bus.write32(s + 0x11c, stamp);
        c.bus.write32(s + 4, r.next_u32());
    }

    /// The compressed data of an unpack test: a header byte (known size
    /// formats and unknown ones), the `0xfb` marker (or not) and a size that
    /// is positive, zero or (32-bit formats) negative.
    pub(crate) fn oc748_packed(c: &mut Cpu, r: &mut Rng, seed: u32) {
        const HDRS: [u8; 8] = [0x10, 0x11, 0x90, 0x91, 0x30, 0xb0, 0x34, 0x77];
        let hdr = HDRS[(seed % 8) as usize];
        c.bus.write8(FX_PACKED, hdr);
        c.bus
            .write8(FX_PACKED + 1, if (seed / 8) % 6 == 0 { 0x12 } else { 0xfb });
        let size = (r.next_u32() % 100) + 1;
        let (b2, b3, b4, b5) = match (seed / 5) % 5 {
            0 => (0, 0, 0, 0),
            1 => (0x80, 0, 0, 0),
            _ => (0, 0, size as u8, (r.next_u32() % 3) as u8),
        };
        c.bus.write8(FX_PACKED + 2, b2);
        c.bus.write8(FX_PACKED + 3, b3);
        c.bus.write8(FX_PACKED + 4, b4);
        c.bus.write8(FX_PACKED + 5, b5);
    }

    /// `BoltFile_UnpackSlot`: no slot, no data, unknown or empty or negative
    /// sizes, and a real unpack with the allocator failing 0 to 2 times (the
    /// real `EnsureBudget` returns at once, the real `EvictBytes` finds no
    /// slot to free).
    #[test]
    fn bolt_file_unpack_slot_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s03::f_0019d000__generated,
            bolt_file_unpack_slot,
            400,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0019_d000, 8);
                let bank = seed % 10;
                let slot = oc748_slot(bank, (seed / 10) % 256);
                c.bus
                    .write32(slot + 0x104, if seed % 11 == 1 { 0 } else { FX_PACKED });
                c.bus.write32(slot + 0x118, r.next_u32() & 0xffff);
                // Flags non-zero: the real `EvictBytes` must not free this slot.
                c.bus.write32(slot, 1);
                c.bus.write32(FX_TABLE + 0xb4000, r.next_u32() & 0xffff);
                oc748_packed(c, &mut r, seed);
                c.g[A0][0] = sx(FX_TABLE);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = if seed % 13 == 0 { 0 } else { sx(slot) };
                c.pc = 0x0019_d000;
            },
        );
    }

    /// `BoltFile_StartNextQueued`: no queued slot, the oldest queued slot in
    /// the own bank (ties and the stamp-0 rule), and a queued slot found only
    /// in another bank; the real `LoadSlot` runs on the fakes.
    #[test]
    fn bolt_file_start_next_queued_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s03::f_0019d140__generated,
            bolt_file_start_next_queued,
            300,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0019_d140, 12);
                let bank = seed % 10;
                // Decoys that are not queued (flags 1 and 2).
                for k in 0..3u32 {
                    let b = r.next_u32() % 10;
                    let i = r.next_u32() % 256;
                    let stamp = r.next_u32() % 4;
                    oc748_put_slot(c, &mut r, b, i, 1 + (k & 1), stamp);
                }
                match (seed / 10) % 3 {
                    0 => {}
                    1 => {
                        for _ in 0..1 + r.next_u32() % 3 {
                            let i = r.next_u32() % 256;
                            let stamp = r.next_u32() % 3;
                            oc748_put_slot(c, &mut r, bank, i, 3, stamp);
                        }
                    }
                    _ => {
                        let b = (bank + 1 + r.next_u32() % 9) % 10;
                        for _ in 0..1 + r.next_u32() % 3 {
                            let i = r.next_u32() % 256;
                            let stamp = r.next_u32() % 3;
                            oc748_put_slot(c, &mut r, b, i, 3, stamp);
                        }
                        if seed % 2 == 0 {
                            let b2 = (b + 1 + r.next_u32() % 9) % 10;
                            if b2 != bank {
                                let i = r.next_u32() % 256;
                                oc748_put_slot(c, &mut r, b2, i, 3, 1);
                            }
                        }
                    }
                }
                c.bus.write32(FX_TABLE + 0xb4000, 0);
                c.g[A0][0] = sx(FX_TABLE);
                c.g[A1][0] = bank as u64;
                c.pc = 0x0019_d140;
            },
        );
    }

    /// `BoltFile_LoadSlot`: synchronous and asynchronous slots, the allocator
    /// failing 0 to 2 times first (the real `EvictBytes` finds nothing to
    /// free).
    #[test]
    fn bolt_file_load_slot_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s03::f_0019d8b8__generated,
            bolt_file_load_slot,
            300,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0019_d8b8, 8);
                let bank = seed % 10;
                let idx = r.next_u32() % 256;
                let slot = oc748_slot(bank, idx);
                c.bus.write32(slot + 0x10c, (seed / 10) % 2);
                c.bus.write32(slot + 4, r.next_u32());
                c.bus.write32(FX_TABLE + 0xb4000, r.next_u32() & 0xffff);
                c.bus.write32(FX_TABLE + 0xb4004, r.next_u32() & 0xff);
                c.g[A0][0] = sx(FX_TABLE);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = idx as u64;
                c.pc = 0x0019_d8b8;
            },
        );
    }

    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc770_machine;

    /// `FEModule_RiderPreviewRequestChar`: every flag combination, ready and
    /// not-ready models, queued requests through the fake frame-count call.
    #[test]
    fn fe_module_rider_preview_request_char_matches() {
        const THIS: u32 = DATA + 0x2000;
        const GRMAN: u32 = DATA + 0x6000;
        const VT: u32 = DATA + 0x8000;
        check(
            ee::s03::f_0019e588__generated,
            fe_module_rider_preview_request_char,
            600,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7703, 1);
                let gp = c.g[GP][0] as u32;
                c.pc = 0x0019_e588;
                let req = r.next_u32() % 12;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = req as u64;
                c.g[A2][0] = (r.next_u32() % 3 == 0) as u64;
                let cur = if r.next_u32() % 3 == 0 { req + 1 } else { req };
                c.bus.write32(THIS, cur);
                c.bus.write32(THIS + 0x10, r.next_u32() % 2);
                c.bus.write32(THIS + 0xcb8, r.next_u32() % 2);
                let pick = |r: &mut Rng, v: [u32; 3]| v[(r.next_u32() % 3) as usize];
                c.bus
                    .write32(THIS + 0xcb4, pick(&mut r, [0, 5, 0xffff_fffd]));
                c.bus.write32(THIS + 0xcc8, pick(&mut r, [0, 1, 7]));
                c.bus.write32(THIS + 0xcc4, pick(&mut r, [0, 1, 3]));
                c.bus
                    .write32(THIS + 0xcd4, pick(&mut r, [0xffff_ffff, 0, 5]));
                c.bus.write32(gp - 0x854, GRMAN);
                c.bus.write32(GRMAN + 0x10d8, VT);
                c.bus.write16(VT + 0x390, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(VT + 0x394, fakes::FAKE);
            },
        );
    }

    /// `FEModule_RiderPreviewCtor`: clears a random-filled module (a leaf:
    /// the 256-entry texture table loop and the identity pose copies).
    #[test]
    fn fe_module_rider_preview_ctor_matches() {
        check(
            ee::s03::f_0019e3d0__generated,
            fe_module_rider_preview_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x780b, 0);
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 0x340);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `BoltFile_CompareTextureSlots`: every combination of a good / state-bad /
    /// type-bad / stamp-bad first and second slot, with equal, smaller and larger names.
    #[test]
    fn bolt_file_compare_texture_slots_matches() {
        check(
            ee::s03::f_0019de18__generated,
            bolt_file_compare_texture_slots,
            96,
            &|c, seed| {
                let mut r = Rng(0x0019_de18 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0019_de18;
                c.g[GP][0] = sx(GAME_GP);
                let table = DATA + 0x0020_0000;
                c.bus.write32(GAME_GP - 0x1784, table);
                let k = seed % 3;
                let j = 3 + seed % 3;
                let row = (seed / 7) % 2;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
                c.bus.write32(DATA, row << 16 | k);
                c.bus.write32(DATA + 0x40, r.next_u32() & 0xffff_0000 | j);
                let a = table + row * 0x12000 + k * 0x120;
                let b = table + row * 0x12000 + j * 0x120;
                for s in [a, b] {
                    for off in (0..0x120).step_by(4) {
                        c.bus.write32(s + off, 0);
                    }
                }
                // 0: good, 1: bad state, 2: bad type, 3: stamp set.
                let put = |c: &mut Cpu, s: u32, variant: u32| {
                    c.bus.write32(s, if variant == 1 { 2 } else { 3 });
                    c.bus.write32(s + 0x110, if variant == 2 { 8 } else { 11 });
                    c.bus.write32(s + 0x11c, (variant == 3) as u32);
                };
                put(c, a, seed % 4);
                put(c, b, (seed / 4) % 4);
                let (na, nb) = match (seed / 16) % 3 {
                    0 => (*b"ABC\0", *b"abc\0"),
                    1 => (*b"ABC\0", *b"ABD\0"),
                    _ => (*b"ABE\0", *b"abd\0"),
                };
                c.bus.write32(a + 4, u32::from_le_bytes(na));
                c.bus.write32(b + 4, u32::from_le_bytes(nb));
            },
        );
    }

    /// Fixture of the two prioritize functions: the bank table at `FX_TABLE` (also
    /// the comparator's table at `gp - 0x1784`), a few slots of `ty` with no stamp
    /// (state 3 or not) among zero slots, the pending counter and the bank counter.
    pub(crate) fn oc801_prioritize_fixture(c: &mut Cpu, seed: u32, salt: u64, ty: u32, entry: u32) {
        let mut r = oc748_fixture(c, seed, salt, 8);
        let gp = c.g[GP][0] as u32;
        c.bus.write32(gp - 0x1784, FX_TABLE);
        let bank = seed % 10;
        let n = 1 + seed % 5;
        for _i in 0..n {
            let idx = r.next_u32() % 256;
            let state = if r.next_u32() % 3 == 0 { 2 } else { 3 };
            oc748_put_slot(c, &mut r, bank, idx, state, 0);
            let s = oc748_slot(bank, idx);
            // Mostly the wanted type; the odd one stops the walk.
            c.bus
                .write32(s + 0x110, if r.next_u32() % 5 == 0 { ty ^ 1 } else { ty });
        }
        // The loop-bound counters: async loads pending (> 0 skips the loads).
        c.bus
            .write32(FX_TABLE + 0xb4004, if seed % 3 == 0 { 1 } else { 0 });
        c.bus
            .write32(FX_TABLE + 0xb4008 + bank * 4, r.next_u32() & 0xff);
        c.g[A0][0] = sx(FX_TABLE);
        c.g[A1][0] = bank as u64;
        c.pc = entry;
    }

    /// `BoltFile_PrioritizeModels`: the real `qsort` and model comparator, slots of
    /// type 8 with and without a pending load, the real `BoltFile_LoadSlot` (its callees
    /// faked). The sort order decides how many slots the walk reaches.
    #[test]
    fn bolt_file_prioritize_models_matches() {
        oc748_install_fakes();
        check(
            ee::s03::f_0019df20__generated,
            bolt_file_prioritize_models,
            120,
            &|c, seed| oc801_prioritize_fixture(c, seed, 0x0019_df20, 8, 0x0019_df20),
        );
    }

    /// `BoltFile_PrioritizeTextures`: as the models, with type 11 and the texture
    /// comparator.
    #[test]
    fn bolt_file_prioritize_textures_matches() {
        oc748_install_fakes();
        check(
            ee::s03::f_0019e070__generated,
            bolt_file_prioritize_textures,
            120,
            &|c, seed| oc801_prioritize_fixture(c, seed, 0x0019_e070, 11, 0x0019_e070),
        );
    }

    use crate::difftest::fakes::LOG_UNWIND_AT;
    pub(crate) use crate::game::fe::messagecenter::tests::oc855_base;
    pub(crate) use crate::game::fe::messagecenter::tests::oc855_force;
    pub(crate) use crate::game::fe::messagecenter::tests::oc855_force_zero;

    pub(crate) use crate::game::fe::messagecenter::tests::oc855_w;

    use crate::game::main::loadscreens::tests::oc736_fake_call;
    use crate::game::main::loadscreens::tests::oc736_force;
    use crate::game::main::loadscreens::tests::oc736_force_zero;
    use crate::game::main::loadscreens::tests::oc736_install_fakes;
    use crate::game::main::loadscreens::tests::FX_CTRL as FX_CTRL_m2;
    use crate::game::main::loadscreens::tests::FX_MAGIC as FX_MAGIC_m2;
    use crate::game::main::loadscreens::tests::FX_OBJ;
    use crate::game::main::loadscreens::tests::FX_WORLD;
    use std::ptr::addr_of_mut;

    #[test]
    fn cfestate_char_equip_ctor_matches() {
        check(
            ssx3_ee::s03::f_00198db8__generated,
            cfestate_char_equip_ctor,
            96,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198db8, 0);
                c.pc = 0x0019_8db8;
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                let world = DATA + 0x4000;
                let flag3 = seed % 3 != 0;
                let flag4 = (seed / 3) % 2 == 1;
                oc855_w(c, this + 0x10, owner);
                oc855_w(c, owner, DATA + 0x2400);
                oc855_w(c, GAME_GP.wrapping_sub(0x848), world);
                oc855_w(c, world + 0x7c, DATA + 0x5000);
                oc855_w(c, GAME_GP.wrapping_sub(0x1f20), r.next_u32());
                oc855_w(c, GAME_GP.wrapping_sub(0x59b0), r.next_u32());
                oc855_w(c, GAME_GP.wrapping_sub(0x59ac), r.next_u32());
                // The 5-entry jump table of the item-list mode.
                let table = [
                    0x0019_90b0,
                    0x0019_90b8,
                    0x0019_90c0,
                    0x0019_90c8,
                    0x0019_90d0,
                ];
                for (i, t) in table.into_iter().enumerate() {
                    oc855_w(c, 0x0046_07d0 + 4 * i as u32, t);
                }
                // `146d98` result: the slot kind (0x11..0x15 select a table
                // entry, anything else the default).
                let mut kind = 0x0f + (seed / 2) % 9;
                if seed % 16 == 5 {
                    // A corrupt entry: the dispatcher leaves through `jump_out`.
                    let bad = seed / 16 % 5;
                    oc855_w(c, 0x0046_07d0 + 4 * bad, 0x0019_9000);
                    kind = 0x11 + bad;
                }
                oc855_force(c, 0x0014_6d98, kind);
                // BuildItemList (real) finds an empty list.
                oc855_force_zero(c, 0x0014_d7e8);
                let mode_called = flag3 || flag4;
                let first_in_build = 22 + if mode_called { 2 } else { 0 };
                let unwind = match seed % 4 {
                    // Unwind somewhere in the constructor.
                    0 => r.next_u32() % first_in_build,
                    // Skip BuildItemList's list loops when `this+0xc8` = 0.
                    _ if !flag3 => first_in_build,
                    _ => u32::MAX,
                };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32() & 0x7f);
                c.g[A3][0] = flag3 as u64;
                c.g[T0][0] = flag4 as u64;
            },
        );
    }

    // Fixture objects, in DATA+0x2a0000 and up: clear of the fake log
    // (DATA+0x100000), of loadscreens' fixtures (0x20..0x29) and of the
    // fakes' result mailboxes (DATA+0x300000 and up).
    pub(crate) const SELF: u32 = DATA + 0x2a_0000;

    pub(crate) const RIDER: u32 = DATA + 0x2b_0000;

    pub(crate) const WIDGET_OBJ: u32 = DATA + 0x2c_0000;

    pub(crate) const WIDGET_VT: u32 = DATA + 0x2c_1000;

    pub(crate) const WIDGET: u32 = DATA + 0x2c_2000;

    pub(crate) const ENGINE: u32 = DATA + 0x2d_0000;

    pub(crate) const CUSTOM: u32 = DATA + 0x2d_1000;

    pub(crate) const CUSTOM_VT: u32 = DATA + 0x2d_2000;

    pub(crate) const LIST: u32 = DATA + 0x2e_0000;

    pub(crate) const ITEMS: u32 = DATA + 0x2f_0000;

    /// A real leaf that returns 0 (a virtual method answering "nothing").
    pub(crate) const ZERO_LEAF: u32 = 0x0014_06c0;

    macro_rules! oc856_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL_m2) != FX_MAGIC_m2 {
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
    pub(crate) fn oc856_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc736_install_fakes();
        oc856_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(FX_CTRL_m2, FX_MAGIC_m2);
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
    pub(crate) fn oc856_widget_object(c: &mut Cpu, seed: u32) {
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

    // Fixture objects, all at DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC857_FX_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC857_FX_MAGIC: u32 = 0x4f43_3835;

    /// The equip state object (0x1000 bytes).
    pub(crate) const OC857_SELF: u32 = DATA + 0x21_0000;

    pub(crate) const OC857_WORLD: u32 = DATA + 0x22_0000;

    /// The front end object `world+0x7c` points at.
    pub(crate) const OC857_FE: u32 = DATA + 0x22_1000;

    /// Widgets: `OC857_WIDGET + 0x100 * n`, all sharing one vtable.
    pub(crate) const OC857_WIDGET: u32 = DATA + 0x23_0000;

    pub(crate) const OC857_VT: u32 = DATA + 0x24_0000;

    /// Item structs, 0x40 bytes each.
    pub(crate) const OC857_ITEMS: u32 = DATA + 0x25_0000;

    pub(crate) const OC857_DB: u32 = DATA + 0x26_0000;

    pub(crate) const OC857_DB_TABLE: u32 = DATA + 0x26_4000;

    pub(crate) const OC857_MENU: u32 = DATA + 0x27_0000;

    pub(crate) const OC857_GR: u32 = DATA + 0x28_0000;

    pub(crate) const OC857_GR_SCREEN: u32 = DATA + 0x28_4000;

    pub(crate) const OC857_FILE: u32 = DATA + 0x29_0000;

    /// Forced results of the fakes: one word per callee.
    pub(crate) const OC857_MAIL: u32 = DATA + 0x30_0000;

    pub(crate) fn oc857_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    pub(crate) fn oc857_mail(addr: u32) -> u32 {
        OC857_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc857_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc857_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc857_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc857_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (none: the fake's pointer) unless it unwound.
    pub(crate) fn oc857_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc857_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc857_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC857_FX_CTRL) != OC857_FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc857_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc857_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc857_fakes!(
        (
            oc857_fake_19da10,
            OC857_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc857_fake_19e238,
            OC857_PREV_19E238,
            ee::s03::OVERRIDE_0019E238,
            ee::s03::f_0019e238__generated,
            0x0019_e238
        ),
        (
            oc857_fake_19e2b0,
            OC857_PREV_19E2B0,
            ee::s03::OVERRIDE_0019E2B0,
            ee::s03::f_0019e2b0__generated,
            0x0019_e2b0
        ),
        (
            oc857_fake_19dc20,
            OC857_PREV_19DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc857_fake_19a9b8,
            OC857_PREV_19A9B8,
            ee::s03::OVERRIDE_0019A9B8,
            ee::s03::f_0019a9b8__generated,
            0x0019_a9b8
        ),
        (
            oc857_fake_1a0548,
            OC857_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
        (
            oc857_fake_19fa78,
            OC857_PREV_19FA78,
            ee::s03::OVERRIDE_0019FA78,
            ee::s03::f_0019fa78__generated,
            0x0019_fa78
        ),
        (
            oc857_fake_19a098,
            OC857_PREV_19A098,
            ee::s03::OVERRIDE_0019A098,
            ee::s03::f_0019a098__generated,
            0x0019_a098
        ),
        (
            oc857_fake_19b098,
            OC857_PREV_19B098,
            ee::s03::OVERRIDE_0019B098,
            ee::s03::f_0019b098__generated,
            0x0019_b098
        ),
        (
            oc857_fake_19afd0,
            OC857_PREV_19AFD0,
            ee::s03::OVERRIDE_0019AFD0,
            ee::s03::f_0019afd0__generated,
            0x0019_afd0
        ),
        (
            oc857_fake_317670,
            OC857_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc857_fake_3a1000,
            OC857_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc857_fake_3a0c50,
            OC857_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc857_fake_14dc80,
            OC857_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc857_fake_14dd58,
            OC857_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc857_fake_14ad28,
            OC857_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc857_fake_14bdb8,
            OC857_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc857_fake_14d7e8,
            OC857_PREV_14D7E8,
            ee::s01::OVERRIDE_0014D7E8,
            ee::s01::f_0014d7e8__generated,
            0x0014_d7e8
        ),
        (
            oc857_fake_3e6448,
            OC857_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc857_fake_418ef8,
            OC857_PREV_418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
    );

    /// Machine with the fakes on, the front end (`world+0x7c`), the widgets'
    /// shared vtable (slot 0x48 `setVisible`, 0x58 a second fake), the equip
    /// state at `OC857_SELF` in `a0`, and a screen manager at `gp-0x854` whose
    /// vtable slot 0x170 is a fake.
    pub(crate) fn oc857_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC857_FX_CTRL, OC857_FX_MAGIC);
        oc857_install_fakes();
        c.bus.write16(OC857_VT + 0x48, 0);
        oc857_w(c, OC857_VT + 0x4c, FAKE + 0x10);
        c.bus.write16(OC857_VT + 0x58, 0);
        oc857_w(c, OC857_VT + 0x5c, FAKE + 0x20);
        for n in 0..8 {
            oc857_w(c, OC857_WIDGET + 0x100 * n + 8, OC857_VT);
        }
        oc857_w(c, GAME_GP - 0x848, OC857_WORLD);
        oc857_w(c, OC857_WORLD + 0x7c, OC857_FE);
        oc857_w(c, GAME_GP - 0x854, OC857_GR);
        oc857_w(c, OC857_GR + 0x10d8, OC857_GR_SCREEN);
        c.bus.write16(OC857_GR_SCREEN + 0x170, 0);
        oc857_w(c, OC857_GR_SCREEN + 0x174, FAKE + 0x30);
        // The polygon limit (`gp-0x1818`).
        oc857_w(c, GAME_GP - 0x1818, 100);
        c.g[A0][0] = sx(OC857_SELF);
        r
    }

    pub(crate) fn oc857_widget(n: u32) -> u32 {
        OC857_WIDGET + 0x100 * n
    }

    /// A widget pointer field: the widget `n`, or null for some seeds.
    pub(crate) fn oc857_put_widget(c: &mut Cpu, off: u32, n: u32, null: bool) {
        oc857_w(c, OC857_SELF + off, if null { 0 } else { oc857_widget(n) });
    }

    /// The menu (`self+0x5c`, selected index `+0x95`, scroll offset `+0x98`)
    /// and `count` items (`self+0x958`, pointers at `+0x124`); each item
    /// gets `flags`, `+0x30` an icon name pointer (or none), cost `+0xe`.
    pub(crate) fn oc857_put_menu(c: &mut Cpu, r: &mut Rng, sel: u32, count: u32) {
        oc857_w(c, OC857_SELF + 0x5c, OC857_MENU);
        c.bus.write8(OC857_MENU + 0x95, sel as u8);
        let off = r.next_u32() % (sel + 1);
        c.bus.write8(OC857_MENU + 0x98, off as u8);
        oc857_w(c, OC857_SELF + 0x958, count);
        for i in 0..6 {
            let item = OC857_ITEMS + 0x40 * i;
            oc857_w(c, OC857_SELF + 0x124 + 4 * i, item);
            oc857_w(
                c,
                item + 0x34,
                [4, 0x24, 0x20, 0, 0x44, 0x2c][(r.next_u32() % 6) as usize],
            );
            oc857_w(
                c,
                item + 0x30,
                if r.next_u32() % 3 == 0 { 0 } else { OC857_FILE },
            );
            c.bus.write16(
                item + 0xe,
                [-5i16, 0, 3, 100][(r.next_u32() % 4) as usize] as u16,
            );
            c.bus.write16(item + 4, (r.next_u32() % 8) as u16);
        }
        for i in 0..8 {
            oc857_w(c, OC857_SELF + 0xa78 + 4 * i, r.next_u32() % 60);
        }
    }

    /// The profile DB (`14ad28` result): the item id table and its entries.
    pub(crate) fn oc857_put_db(c: &mut Cpu, r: &mut Rng) {
        oc857_force(c, 0x0014_ad28, OC857_DB);
        oc857_w(c, OC857_DB + 0x288, OC857_DB_TABLE);
        for id in 0..8u32 {
            let idx = [0xffffu16, 0, 1, 2][(r.next_u32() % 4) as usize];
            c.bus.write16(OC857_DB_TABLE + 2 * id, idx);
        }
        for e in 0..3u32 {
            c.bus
                .write16(OC857_DB + 0x290 + 4 * e + 2, r.next_u32() as u16);
        }
    }

    // ------------------------------------------------------- UpdatePreview

    /// `UpdatePreview`: delay counting, acquire, loaded check, data entry or
    /// none, and the non-buy-mode early exit.
    #[test]
    fn cfe_state_char_equip_update_preview_matches() {
        check(
            ee::s03::f_0019a638__generated,
            cfe_state_char_equip_update_preview,
            144,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_a638, 6);
                c.pc = 0x0019_a638;
                let s = OC857_SELF;
                let mode = if seed % 9 == 8 {
                    [0u32, 2, 5][(seed / 9 % 3) as usize]
                } else {
                    1
                };
                oc857_w(c, s + 0xc8, mode);
                oc857_w(c, s + 0x95c, [0, u32::MAX, 1, 2][(seed % 4) as usize]);
                let idx = if (seed / 4) % 2 == 0 {
                    u32::MAX
                } else {
                    (seed / 8) % 5
                };
                oc857_w(c, s + 0xa6c, idx);
                oc857_w(c, s + 0xa70, r.next_u32());
                oc857_w(c, s + 0xbc, r.next_u32() % 10);
                if (seed / 8) % 2 == 0 {
                    oc857_force_zero(c, 0x0019_e238);
                } else {
                    oc857_force(c, 0x0019_e238, 1);
                }
                if (seed / 16) % 2 == 0 {
                    oc857_force_zero(c, 0x0019_e2b0);
                } else {
                    oc857_force(c, 0x0019_e2b0, OC857_FILE);
                    oc857_w(c, OC857_FILE + 0x14, r.next_u32() % 0x100);
                }
                oc857_force(c, 0x0019_da10, r.next_u32() % 20);
            },
        );
    }

    // -------------------------------------------- ReleasePreviewBoltTextures

    #[test]
    fn cfe_state_char_equip_release_preview_bolt_textures_matches() {
        check(
            ee::s03::f_0019a758__generated,
            cfe_state_char_equip_release_preview_bolt_textures,
            24,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_a758, 3);
                c.pc = 0x0019_a758;
                if seed % 3 == 0 {
                    oc857_w(c, OC857_WORLD + 0x7c, 0);
                }
                c.bus.write8(OC857_SELF + 0x44, r.next_u32() as u8);
            },
        );
    }

    // ------------------------------------------------------- UpdateHelpText

    /// `UpdateHelpText`: no widget, no item (both modes), category row,
    /// buy mode money check, equip mode polygon budget check.
    #[test]
    fn cfe_state_char_equip_update_help_text_matches() {
        check(
            ee::s03::f_0019a798__generated,
            cfe_state_char_equip_update_help_text,
            240,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_a798, 7);
                c.pc = 0x0019_a798;
                let s = OC857_SELF;
                oc857_put_widget(c, 0x4c, 0, seed % 13 == 0);
                let sel = [0u32, 1, 2, 3, 4, 5][(r.next_u32() % 6) as usize];
                oc857_put_menu(c, &mut r, sel, 4);
                oc857_w(c, s + 0xc8, [1u32, 0, 2, 1][(seed / 3 % 4) as usize]);
                oc857_w(c, s + 0xae4, r.next_u32() & 1);
                oc857_w(c, s + 0xc0, r.next_u32() % 2000);
                oc857_w(c, s + 0xc4, r.next_u32() % 90);
                oc857_w(c, s + 0xbc, r.next_u32() % 10);
                c.bus.write8(s + 0x44, r.next_u32() as u8);
                oc857_put_db(c, &mut r);
            },
        );
    }

    // ----------------------------------------------------- RefreshEquipInfo

    /// `RefreshEquipInfo`: widgets present or not, an item that is a
    /// category row or not, icon textures loaded or not.
    #[test]
    fn cfe_state_char_equip_refresh_equip_info_matches() {
        check(
            ee::s03::f_0019aa08__generated,
            cfe_state_char_equip_refresh_equip_info,
            240,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_aa08, 9);
                c.pc = 0x0019_aa08;
                let s = OC857_SELF;
                let nulls = r.next_u32();
                oc857_put_widget(c, 0x6c, 0, nulls % 7 == 0);
                oc857_put_widget(c, 0x84, 1, nulls / 7 % 7 == 0);
                oc857_put_widget(c, 0x7c, 2, nulls / 49 % 7 == 0);
                oc857_w(c, s + 0x74, 0);
                let sel = [0u32, 1, 2, 3, 4, 5][(r.next_u32() % 6) as usize];
                oc857_put_menu(c, &mut r, sel, 4);
                if seed % 17 == 0 {
                    oc857_w(c, s + 0x5c, 0);
                }
                oc857_w(c, s + 0xa60, r.next_u32() & 1);
            },
        );
    }

    // ------------------------------------------------------- RefreshBuyInfo

    /// `RefreshBuyInfo`: large preview widget and texture, the cost text
    /// (real `getDollarString`, with the game image for libc).
    #[test]
    fn cfe_state_char_equip_refresh_buy_info_matches() {
        check(
            ee::s03::f_0019ab78__generated,
            cfe_state_char_equip_refresh_buy_info,
            120,
            &|c, seed| {
                crate::difftest::elf_image(c);
                let mut r = oc857_fixture(c, seed, 0x19_ab78, 8);
                c.pc = 0x0019_ab78;
                let s = OC857_SELF;
                let nulls = r.next_u32();
                oc857_put_widget(c, 0x88, 3, nulls % 5 == 0);
                oc857_put_widget(c, 0x68, 4, nulls / 5 % 5 == 0);
                let sel = [0u32, 1, 2, 3, 4, 5][(r.next_u32() % 6) as usize];
                oc857_put_menu(c, &mut r, sel, 4);
                if seed % 19 == 0 {
                    oc857_w(c, s + 0x5c, 0);
                }
                oc857_w(c, s + 0xa70, [u32::MAX, 3, 7][(r.next_u32() % 3) as usize]);
            },
        );
    }

    // ------------------------------------------------------- BuildItemList

    /// `BuildItemList`: both modes' filters (name, flags, slot filter,
    /// bought flag, category check) over a stack list the (faked)
    /// `GetItems` leaves, then the sort call.
    #[test]
    fn cfe_state_char_equip_build_item_list_matches() {
        check(
            ee::s03::f_0019b180__generated,
            cfe_state_char_equip_build_item_list,
            240,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_b180, 14);
                c.pc = 0x0019_b180;
                let s = OC857_SELF;
                c.g[A1][0] = r.next_u32() as u64 % 20;
                oc857_w(c, s + 0xc8, [1u32, 0, 2][(seed % 3) as usize]);
                oc857_w(c, s + 0xcc, [u32::MAX, 0, 1][(seed / 3 % 3) as usize]);
                oc857_w(c, s + 0xbc, r.next_u32() % 10);
                c.bus.write8(s + 0x44, r.next_u32() as u8);
                oc857_put_db(c, &mut r);
                let n = (seed / 9 % 8).min(6);
                oc857_force(c, 0x0014_d7e8, if seed % 29 == 0 { 0 } else { n });
                // The item list the callee leaves at the new sp.
                let list = crate::difftest::SP - 0x8e0;
                for i in 0..6u32 {
                    let item = OC857_ITEMS + 0x40 * i;
                    oc857_w(c, list + 4 * i, item);
                    oc857_w(c, item + 0x14, if r.next_u32() % 5 == 0 { 0 } else { 1 });
                    oc857_w(
                        c,
                        item + 0x34,
                        [0u32, 4, 0x20, 0x24, 0x2c][(r.next_u32() % 5) as usize],
                    );
                    c.bus.write16(item + 4, (r.next_u32() % 8) as u16);
                    c.bus.write8(item + 2, (r.next_u32() % 3) as u8);
                }
                if (seed / 72) % 2 == 0 {
                    oc857_force_zero(c, 0x0019_b098);
                    oc857_force_zero(c, 0x0019_afd0);
                } else {
                    oc857_force(c, 0x0019_b098, 1);
                    oc857_force(c, 0x0019_afd0, 1);
                }
            },
        );
    }

    /// Gate of this module's callee fakes (the machine's own RAM).
    pub(crate) const OC858_CTRL: u32 = DATA + 0x21_0000;

    pub(crate) const OC858_MAGIC: u32 = 0x0c85_8858;

    // Fixture layout (all inside DATA..DATA+0xfffff).
    pub(crate) const THIS: u32 = DATA + 0x1000;

    pub(crate) const ITEMS_2: u32 = DATA + 0x2000;

    pub(crate) const MENU: u32 = DATA + 0x3000;

    pub(crate) const WIDGETS: u32 = DATA + 0x5000;

    pub(crate) const VT: u32 = DATA + 0x8000;

    pub(crate) const PROF: u32 = DATA + 0x9000;

    pub(crate) const TBL: u32 = DATA + 0xa000;

    pub(crate) const NAMES: u32 = DATA + 0xb000;

    // cFEStateCharEquip fields the functions read.
    pub(crate) const F_SLOT: u32 = 0x44;

    pub(crate) const F_MENU: u32 = 0x5c;

    pub(crate) const F_BAR: u32 = 0x74;

    pub(crate) const F_NAMES: u32 = 0x8c;

    pub(crate) const F_ICONS: u32 = 0xa4;

    pub(crate) const F_TEAM: u32 = 0xbc;

    pub(crate) const F_MONEY: u32 = 0xc0;

    pub(crate) const F_POLYS: u32 = 0xc4;

    pub(crate) const F_MODE: u32 = 0xc8;

    pub(crate) const F_ITEMS: u32 = 0x124;

    pub(crate) const F_NUM_ITEMS: u32 = 0x958;

    /// Forced results of fake callees (flag word, value word), by index.
    pub(crate) const FORCE_PROFILE: u32 = 0;

    pub(crate) const FORCE_TOGGLE: u32 = 1;

    pub(crate) const FORCE_FILL_BUY: u32 = 2;

    pub(crate) const FORCE_FILL_EQUIP: u32 = 3;

    macro_rules! oc858_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, $idx:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC858_CTRL) != OC858_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if $idx != 99 && c.bus.read32(OC858_CTRL + 0x100 + 8 * $idx) != 0 {
                    c.g[V0][0] = sx(c.bus.read32(OC858_CTRL + 0x104 + 8 * $idx));
                }
            }
        };
    }

    oc858_fake!(
        oc858_fake_acb0,
        OC858_PREV_ACB0,
        ee::s01::f_0014acb0__generated,
        0x0014_acb0,
        99
    );

    oc858_fake!(
        oc858_fake_ad28,
        OC858_PREV_AD28,
        ee::s01::f_0014ad28__generated,
        0x0014_ad28,
        FORCE_PROFILE
    );

    oc858_fake!(
        oc858_fake_afb0,
        OC858_PREV_AFB0,
        ee::s01::f_0014afb0__generated,
        0x0014_afb0,
        FORCE_TOGGLE
    );

    oc858_fake!(
        oc858_fake_b478,
        OC858_PREV_B478,
        ee::s01::f_0014b478__generated,
        0x0014_b478,
        99
    );

    oc858_fake!(
        oc858_fake_b560,
        OC858_PREV_B560,
        ee::s01::f_0014b560__generated,
        0x0014_b560,
        99
    );

    oc858_fake!(
        oc858_fake_dc80,
        OC858_PREV_DC80,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        99
    );

    oc858_fake!(
        oc858_fake_dd58,
        OC858_PREV_DD58,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        99
    );

    oc858_fake!(
        oc858_fake_aea8,
        OC858_PREV_AEA8,
        ee::s01::f_0014aea8__generated,
        0x0014_aea8,
        99
    );

    oc858_fake!(
        oc858_fake_hash,
        OC858_PREV_HASH,
        ee::s10::f_00317670__generated,
        0x0031_7670,
        99
    );

    oc858_fake!(
        oc858_fake_scrollable,
        OC858_PREV_SCROLLABLE,
        ee::s13::f_0039b760__generated,
        0x0039_b760,
        99
    );

    oc858_fake!(
        oc858_fake_select,
        OC858_PREV_SELECT,
        ee::s13::f_0039b960__generated,
        0x0039_b960,
        99
    );

    oc858_fake!(
        oc858_fake_set_id,
        OC858_PREV_SET_ID,
        ee::s13::f_003a1000__generated,
        0x003a_1000,
        99
    );

    oc858_fake!(
        oc858_fake_set_ascii,
        OC858_PREV_SET_ASCII,
        ee::s13::f_003a0c50__generated,
        0x003a_0c50,
        99
    );

    oc858_fake!(
        oc858_fake_memcopy,
        OC858_PREV_MEMCOPY,
        ee::s14::f_003e6574__generated,
        0x003e_6574,
        99
    );

    oc858_fake!(
        oc858_fake_fill_equip,
        OC858_PREV_FILL_EQUIP,
        ee::s03::f_0019b518__generated,
        0x0019_b518,
        FORCE_FILL_EQUIP
    );

    oc858_fake!(
        oc858_fake_fill_buy,
        OC858_PREV_FILL_BUY,
        ee::s03::f_0019b598__generated,
        0x0019_b598,
        FORCE_FILL_BUY
    );

    pub(crate) fn oc858_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014ACB0),
            oc858_fake_acb0,
            &OC858_PREV_ACB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AD28),
            oc858_fake_ad28,
            &OC858_PREV_AD28,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AFB0),
            oc858_fake_afb0,
            &OC858_PREV_AFB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014B478),
            oc858_fake_b478,
            &OC858_PREV_B478,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014B560),
            oc858_fake_b560,
            &OC858_PREV_B560,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc858_fake_dc80,
            &OC858_PREV_DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc858_fake_dd58,
            &OC858_PREV_DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014AEA8),
            oc858_fake_aea8,
            &OC858_PREV_AEA8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc858_fake_hash,
            &OC858_PREV_HASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B760),
            oc858_fake_scrollable,
            &OC858_PREV_SCROLLABLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B960),
            oc858_fake_select,
            &OC858_PREV_SELECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc858_fake_set_id,
            &OC858_PREV_SET_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc858_fake_set_ascii,
            &OC858_PREV_SET_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc858_fake_memcopy,
            &OC858_PREV_MEMCOPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019B518),
            oc858_fake_fill_equip,
            &OC858_PREV_FILL_EQUIP,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019B598),
            oc858_fake_fill_buy,
            &OC858_PREV_FILL_BUY,
        );
    }

    /// A machine with the callee fakes installed and gated on, the widget
    /// vtable set up, and `calls` as the range of fake-call indices at which a
    /// call may unwind.
    pub(crate) fn oc858_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc858_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC858_CTRL, OC858_MAGIC);
        // Widget methods: +0x48 setVisible(delta, fn), +0x58 setColour.
        c.bus.write16(VT + 0x48, 0x10);
        c.bus.write32(VT + 0x4c, FAKE);
        c.bus.write16(VT + 0x58, 0x20);
        c.bus.write32(VT + 0x5c, FAKE + 0x10);
        c.g[GP][0] = sx(crate::difftest::GAME_GP);
        r
    }

    pub(crate) fn oc858_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC858_CTRL + 0x100 + 8 * idx, 1);
        c.bus.write32(OC858_CTRL + 0x104 + 8 * idx, v);
    }

    pub(crate) fn oc858_widget(c: &mut Cpu, k: u32) -> u32 {
        let w = WIDGETS + 0x100 * k;
        c.bus.write32(w + 8, VT);
        w
    }

    /// The state object with a menu (current row, scroll offset), items,
    /// and all twelve row widgets.
    pub(crate) fn oc858_state(c: &mut Cpu, r: &mut Rng, with_menu: bool) {
        c.bus
            .write32(THIS + F_MENU, if with_menu { MENU } else { 0 });
        c.bus.write8(MENU + 0x95, (r.next_u32() % 10) as u8);
        c.bus.write8(MENU + 0x96, 0);
        c.bus.write8(MENU + 0x98, (r.next_u32() % 4) as u8);
        c.bus.write8(THIS + F_SLOT, (r.next_u32() % 4) as u8);
        c.bus.write32(THIS + F_TEAM, r.next_u32() % 3);
        for k in 0..6 {
            let n = oc858_widget(c, k);
            c.bus.write32(THIS + F_NAMES + 4 * k, n);
            let i = oc858_widget(c, 6 + k);
            c.bus.write32(THIS + F_ICONS + 4 * k, i);
        }
        for k in 0..12u32 {
            c.bus.write32(THIS + F_ITEMS + 4 * k, ITEMS_2 + 0x40 * k);
        }
        // The shared widget (the bar) of UpdateStatPreviewBar.
        let bar = oc858_widget(c, 12);
        c.bus.write32(THIS + F_BAR, bar);
    }

    pub(crate) fn oc858_item(c: &mut Cpu, r: &mut Rng, k: u32, flags: u32) {
        let it = ITEMS_2 + 0x40 * k;
        c.bus.write32(it + 0x34, flags);
        c.bus.write16(it + 4, (r.next_u32() % 6) as u16);
        c.bus.write16(
            it + 0xe,
            [0u16, 0xfff6, 5, 40, 300][(r.next_u32() % 5) as usize],
        );
        c.bus.write32(it + 0x14, NAMES + 16 * k);
    }

    /// The profile of `FUNC_PROFILE`: a table of slot indices by item id
    /// (some negative: not in the profile) and the flag entries.
    pub(crate) fn oc858_profile(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(PROF + 0x288, TBL);
        for id in 0..6u32 {
            let v: i16 = if (seed / 11 + id) % 4 == 0 {
                -1
            } else {
                (r.next_u32() % 8) as i16
            };
            c.bus.write16(TBL + 2 * id, v as u16);
        }
        for k in 0..8u32 {
            c.bus.write32(PROF + 0x290 + 4 * k, r.next_u32());
        }
        oc858_force(c, FORCE_PROFILE, PROF);
    }

    /// `RefreshItemList`: with and without a menu, reset and select flags,
    /// both modes, an empty list or rows, and a clamped visible row.
    #[test]
    fn cfe_state_char_equip_refresh_item_list_matches() {
        check(
            ee::s03::f_0019b618__generated,
            cfe_state_char_equip_refresh_item_list,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_b618, 20);
                oc858_state(c, &mut r, seed % 13 != 0);
                c.bus
                    .write32(THIS + F_MODE, [0u32, 1, 0x77][((seed / 4) % 3) as usize]);
                c.bus.write32(
                    THIS + F_NUM_ITEMS,
                    if (seed / 8) % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 9
                    },
                );
                let n = [0u32, 1, 3, 9][((seed / 24) % 4) as usize];
                oc858_force(c, FORCE_FILL_BUY, n);
                oc858_force(c, FORCE_FILL_EQUIP, n);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = [0u64, 1, 7][(seed % 3) as usize];
                c.g[A2][0] = [0u64, 1, 5][((seed / 2) % 3) as usize];
                c.pc = 0x0019_b618;
            },
        );
    }

    /// `SetEquipRow`: owned items, items found or missing in the profile,
    /// toggles that succeed or fail, and rows with or without widgets.
    #[test]
    fn cfe_state_char_equip_set_equip_row_matches() {
        check(
            ee::s03::f_0019b7e0__generated,
            cfe_state_char_equip_set_equip_row,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_b7e0, 14);
                oc858_state(c, &mut r, seed % 17 != 0);
                let row = seed % 6;
                let top = c.bus.read8(MENU + 0x98) as u32;
                for k in 0..12 {
                    let owned = seed % 5 == 0 && k == top + row;
                    let bits = r.next_u32() & !0x20;
                    oc858_item(c, &mut r, k, bits | if owned { 0x20 } else { 0 });
                }
                oc858_profile(c, &mut r, seed);
                oc858_force(c, FORCE_TOGGLE, (seed / 3) % 2);
                if (seed / 7) % 5 == 0 {
                    c.bus.write32(THIS + F_NAMES + 4 * row, 0);
                }
                if (seed / 3) % 7 == 0 {
                    c.bus.write32(THIS + F_ICONS + 4 * row, 0);
                }
                for off in [0xa90u32, 0xa94, 0xa98, 0xa9c, 0xaa0] {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(THIS + off, v);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = row as u64;
                c.pc = 0x0019_b7e0;
            },
        );
    }

    /// `SetBuyRow`: owned items (plus icon), affordable and too-expensive
    /// ones, free ones, and rows without text or icon widget.
    #[test]
    fn cfe_state_char_equip_set_buy_row_matches() {
        check(
            ee::s03::f_0019ba60__generated,
            cfe_state_char_equip_set_buy_row,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_ba60, 8);
                oc858_state(c, &mut r, seed % 17 != 0);
                let row = seed % 6;
                let top = c.bus.read8(MENU + 0x98) as u32;
                for k in 0..12 {
                    let owned = seed % 5 == 0 && k == top + row;
                    let bits = r.next_u32() & !0x20;
                    oc858_item(c, &mut r, k, bits | if owned { 0x20 } else { 0 });
                }
                let money = [0u32, 5, 10, 50, 400, 0xffff_fff0][((seed / 5) % 6) as usize];
                c.bus.write32(THIS + F_MONEY, money);
                if (seed / 7) % 5 == 0 {
                    c.bus.write32(THIS + F_NAMES + 4 * row, 0);
                }
                if (seed / 3) % 7 == 0 {
                    c.bus.write32(THIS + F_ICONS + 4 * row, 0);
                }
                for off in [0xa90u32, 0xa94, 0xaa0] {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(THIS + off, v);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = row as u64;
                c.pc = 0x0019_ba60;
            },
        );
    }

    /// `UpdateStatPreviewBar`: no bar, bars under, inside and over the
    /// polygon limit (clamped ratio at both ends).
    #[test]
    fn cfe_state_char_equip_update_stat_preview_bar_matches() {
        check(
            ee::s03::f_0019bd48__generated,
            cfe_state_char_equip_update_stat_preview_bar,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_bd48, 4);
                oc858_state(c, &mut r, true);
                if seed % 9 == 0 {
                    c.bus.write32(THIS + F_BAR, 0);
                }
                let limit = [3000u32, 4000, 1500, 2000][(seed % 4) as usize];
                c.bus.write32(crate::difftest::GAME_GP - 0x1818, limit);
                c.bus.write32(
                    THIS + F_POLYS,
                    [0u32, 1000, 2500, 3900, 5000][((seed / 4) % 5) as usize],
                );
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] =
                    [0u64, 200, 0xffff_ffff_ffff_fe00, 1500, 3000][((seed / 20) % 5) as usize];
                c.pc = 0x0019_bd48;
            },
        );
    }

    /// `BuySelectedItem`: the money word after the buy call.
    #[test]
    fn cfe_state_char_equip_buy_selected_item_matches() {
        check(
            ee::s03::f_0019be80__generated,
            cfe_state_char_equip_buy_selected_item,
            200,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_be80, 4);
                oc858_state(c, &mut r, true);
                for k in 0..12 {
                    oc858_item(c, &mut r, k, 0);
                }
                c.bus.write32(THIS + F_MONEY, r.next_u32());
                c.g[A0][0] = sx(THIS);
                c.pc = 0x0019_be80;
            },
        );
    }

    /// `EquipSelectedItem`: no items, an item that cannot be toggled, a
    /// missing profile entry, and a toggle that fails or succeeds.
    #[test]
    fn cfe_state_char_equip_equip_selected_item_matches() {
        check(
            ee::s03::f_0019bee8__generated,
            cfe_state_char_equip_equip_selected_item,
            300,
            &|c, seed| {
                let mut r = oc858_machine(c, seed, 0x0019_bee8, 6);
                oc858_state(c, &mut r, true);
                for k in 0..12 {
                    let bits = r.next_u32() & !4;
                    let flags = if seed % 4 == 1 { 0 } else { 4 | bits };
                    oc858_item(c, &mut r, k, flags);
                }
                let n = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    _ => 1 + r.next_u32() % 5,
                };
                c.bus.write32(THIS + F_NUM_ITEMS, n);
                oc858_profile(c, &mut r, seed);
                oc858_force(c, FORCE_TOGGLE, (seed / 6) % 2);
                c.g[A0][0] = sx(THIS);
                c.pc = 0x0019_bee8;
            },
        );
    }

    /// `BoltFile_EvictBytes`: tables of idle, busy (state/timer set) and
    /// empty slots with random sizes and priorities; the byte target spans
    /// "satisfied by the first free" to "never satisfied", and the skipped
    /// table is anywhere in 0..=10. `BoltFile_FreeSlot` runs for real with a
    /// fake `EA_MemFree`.
    #[test]
    fn bolt_file_evict_bytes_matches() {
        use crate::game::fe::festatecharequipdetail::tests::{
            bolt_fs_fake_mem_free, BOLT_FS_CTRL, BOLT_FS_MAGIC,
        };
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            bolt_fs_fake_mem_free,
            fakes::prev_cell(bolt_fs_fake_mem_free),
        );
        check(
            ee::s03::f_0019d738__generated,
            bolt_file_evict_bytes,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19d738, 0);
                c.pc = 0x0019_d738;
                c.bus.write32(BOLT_FS_CTRL, BOLT_FS_MAGIC);
                let this = DATA;
                let busy_percent = [0u32, 20, 60][(seed % 3) as usize];
                for table in 0..10u32 {
                    for _ in 0..(seed % 5) * 6 + 4 {
                        let index = if r.next_u32() % 8 == 0 {
                            255
                        } else {
                            r.next_u32() % 256
                        };
                        let slot = this + table * 0x12000 + index * OC859_SLOT_SIZE;
                        c.bus.write32(slot + OC859_SLOT_DATA, DATA + 0xd_0000);
                        c.bus
                            .write32(slot + OC859_SLOT_SIZE_OFF, 1 + r.next_u32() % 300);
                        c.bus.write32(slot + OC859_SLOT_PRIORITY, r.next_u32() % 40);
                        if r.next_u32() % 100 < busy_percent {
                            match r.next_u32() % 3 {
                                0 => c.bus.write32(slot, 1),
                                1 => c.bus.write32(slot + OC859_SLOT_TIMER, 5),
                                _ => c.bus.write32(slot + OC859_SLOT_DATA, 0),
                            }
                        }
                    }
                }
                c.bus.write32(this + 0xb_4000, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 11);
                c.g[A2][0] = sx(match seed % 4 {
                    0 => 1 + r.next_u32() % 100,
                    1 => 1 + r.next_u32() % 1000,
                    2 => 1 + r.next_u32() % 5000,
                    _ => 100_000,
                });
            },
        );
    }
}
