//! `src/fe/festateriderbio2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
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
        0x0019_1228,
        cfe_state_rider_bio_fill_bio_info,
        "cFEStateRiderBio_fillBioInfo",
    ),
    (
        0x0019_12d0,
        c_fe_state_rider_bio_on_widget_action,
        "cFEStateRiderBio_OnWidgetAction",
    ),
    (
        0x0019_1360,
        cfe_state_rule_select_ctor,
        "cFEStateRuleSelect_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
}

/// `cFEStateRuleSelect* cFEStateRuleSelect_ctor(void* self)` (0x191360): the
/// rule-select state: `UIState_ctor`, vtable, the string-type byte from the
/// first UI layer, and the state id 0x14.
pub fn cfe_state_rule_select_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x0039_e2a0, 0x0019_1374).is_err() {
        return;
    }
    c.g[V1][0] = sx(0x0046_b090);
    c.sw(p(c, S0, 0x48), 0);
    c.sb(p(c, S0, 0x44), 0);
    c.sw(p(c, S0, 8), c.g[V1][0]);
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr(p(c, S0, 0x10)).lw(c);
    c.g[A0][0] = Addr(p(c, V0, 0)).lw(c);
    if call_ptr(c, 0x001a_1cd0, 0x0019_1398).is_err() {
        return;
    }
    c.sb(p(c, S0, 0x15), c.g[V0][0]);
    c.g[V0][0] = 0x14;
    frame.restore_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.close(c);
}

/// `void cFEStateRiderBio_OnWidgetAction(self, item, msg)` (0x1912d0): on a
/// widget action (6) hand the event to the screen's virtual handler and swap
/// the screen; on a page change (9) remember the page and rebuild the bio.
pub fn c_fe_state_rider_bio_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let item = abi::arg64(c, 1);
        let _msg = abi::arg64(c, 2);
        c.g[V1][0] = item;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_item = c.g[V1][0] == 0;
        c.g[S0][0] = sx(self_);
        if no_item {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is_six = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 9;
            if is_six {
                c.cycles += 11;
                c.g[V0][0] = c.lw(at(c, S0, 0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw(at(c, V1, 0x18));
                c.g[A3][0] = c.lw(r32(c, V0));
                c.g[V1][0] = c.lw(at(c, A3, 4));
                c.g[A0][0] = c.lh(at(c, V1, 0x28));
                c.g[V0][0] = c.lw(at(c, V1, 0x2c));
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                let target = r32(c, V0);
                call_ptr(c, target, 0x0019_1328)?;
                let nothing = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if nothing {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(at(c, S0, 0x10));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                    call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x0019_133c)?;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.cycles += 2;
                let is_nine = c.g[A2][0] == c.g[V0][0];
                if is_nine {
                    c.g[V0][0] = c.lbu(at(c, V1, 0x319));
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(c, S0, 0x4c), c.g[V0][0]);
                    call_ptr(c, 0x0019_0a08, 0x0019_1350)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider bio: format the character's display name into the bio's text object.

/// `void cFEStateRiderBio_fillBioInfo(cFEStateRiderBio* self)` (0x191228):
/// find the scrolling-text object on the screen, format the character's
/// display name into a stack buffer and set it as the text (by hash).
pub fn cfe_state_rider_bio_fill_bio_info(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 14;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        c.g[A0][0] = sx(0x0046_0000);
        frame.save_ra(c, 0x40);
        c.g[RA][0] = sx(0x0019_1248);
        c.g[A0][0] = sx(0x0045_e438);
        call_ptr(c, 0x0031_7670, 0x0019_1248)?;
        c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x40));
        c.g[RA][0] = sx(0x0019_1254);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_1254)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.cycles += 23;
            c.g[RA][0] = sx(0x0019_1268);
            call_ptr(c, 0x0014_dc80, 0x0019_1268)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019_1274);
            c.g[A1][0] = 2;
            call_ptr(c, 0x0014_dd58, 0x0019_1274)?;
            c.g[A1][0] = lw(c, at_2(c.g[S0][0], 0x48));
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = sx(0x0046_0000);
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x0019_128c);
            c.g[S0][0] = sx(0x0045_f7d0);
            call_ptr(c, 0x0014_eec8, 0x0019_128c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(frame.at(c, 0));
            c.g[RA][0] = sx(0x0019_129c);
            c.g[A2][0] = c.g[V0][0];
            call_ptr(c, 0x0041_7828, 0x0019_129c)?;
            c.g[RA][0] = sx(0x0019_12a4);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_18c0, 0x0019_12a4)?;
            c.g[RA][0] = sx(0x0019_12ac);
            c.g[A0][0] = sx(frame.at(c, 0));
            call_ptr(c, 0x0031_7670, 0x0019_12ac)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0019_12b8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1f18, 0x0019_12b8)?;
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::dirtysock::tags2::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::festatecredits::tests::RET_ZERO;
    use ssx3_ee as ee;

    #[test]
    fn cfe_state_rule_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00191360__generated,
            cfe_state_rule_select_ctor,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x191360, 2);
                let this = DATA + 0x400;
                c.bus.write32(this + 0x10, DATA + 0x500);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_rider_bio_on_widget_action_matches() {
        check(
            ee::s02::f_001912d0__generated,
            c_fe_state_rider_bio_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1912, 0);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let engine = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vt = DATA + 0x1200;
                slot(c, vt, 0x28, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                w32(c, this + 0x10, engine);
                w32(c, engine, obj);
                w32(c, obj + 4, vt);
                w8(c, item + 0x319, seed % 4);
                w32(c, item + 0x18, seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_rider_bio_fill_bio_info_matches() {
        check(
            ee::s02::f_00191228__generated,
            cfe_state_rider_bio_fill_bio_info,
            SEEDS_2,
            &|c, seed| {
                let _ = base(c, seed, 0x1912_28, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The screen and (for the found branch) the text object.
                let screen = obj + 0x800;
                w(c, obj + 0x40, if seed % 3 == 0 { 0 } else { screen });
                w(c, obj + 0x48, seed % 8);
                // UIScreen_FindObject: count at_2 +0x3c, array at_2 +0x38,
                // entries whose +0x38 is the hash.
                w(c, screen + 0x3c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, screen + 0x38, screen + 0x900);
                let entry = screen + 0xa00;
                w(c, screen + 0x900, entry);
                // The hash of the (zeroed) bio-title string.
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x0045_e438u64]);
                w(c, entry + 0x38, hash as u32);
                // The text-scroll object: no lines, no popup chain.
                w(c, entry + 0xbc, 0);
                wh(c, entry + 0xc8, 0);
            },
        );
    }
}
