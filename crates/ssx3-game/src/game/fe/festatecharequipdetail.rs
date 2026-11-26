//! `src/fe/festatecharequipdetail.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
#[cfg(test)]
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
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
}
