//! `src/fe/festateruleselect.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::Callee;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee_rt::vu0::Src;
use crate::events::back_edge_unwinds;
use callee::*;
use crate::call::sp;
use crate::call::Flow;
use crate::abi::r32;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::main::gamestate::p_2 as p;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_1e08, c_fe_state_rule_select_v21, "cFEStateRuleSelect_v21"),
    (0x0019_26f0, c_fe_state_bragging_rights_init_player_rows, "cFEStateBraggingRights_InitPlayerRows"),
    (0x0019_2740, c_fe_state_bragging_rights_set_rows_visible, "cFEStateBraggingRights_SetRowsVisible"),
    (0x0019_2948, cfe_state_bragging_rights_is_row_enabled, "cFEStateBraggingRights_IsRowEnabled"),
    (0x0019_2d00, cfestatebraggingrights_ctor, "cFEStateBraggingRights_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
}


// ---------------------------------------------------------------------------
// Small leaves.

/// `bool cFEStateBraggingRights_IsRowEnabled(this, int idx)` (0x00192948):
/// bit `idx` of the top byte of the word at `+0x2c`.
pub fn cfe_state_bragging_rights_is_row_enabled(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x0100_0000;
    let word = Addr((c.g[A0][0] as u32).wrapping_add(0x2c)).lw(c);
    c.g[V0][0] = word;
    let shift = (c.g[A1][0] as u32) & 31;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << shift);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
    let masked = word & c.g[V1][0];
    c.g[V0][0] = masked;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < masked) as u64;
    c.pc = ra;
}

/// `StrHash` (0x317670).
pub(crate) const STR_HASH: u32 = 0x0031_7670;


/// `int cFEStateRuleSelect_v21(this, msg)` (0x191e08): 0x101 when the message
/// id at `+0x38` is the hash of `D_004A13A0`, 0x100 otherwise.
pub fn c_fe_state_rule_select_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0x004a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x004a_13a0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
        call_ptr(c, STR_HASH, 0x0019_1e24)?;
        c.g[S0][0] = c.g[S0][0] ^ c.g[V0][0];
        c.g[V1][0] = 0x100;
        c.g[V0][0] = 0x101;
        frame.restore_ra(c, 0);
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateRulesSelect* cFEStateBraggingRights_InitPlayerRows(self)`
/// (0x1926f0): install the vtable, clear the eight player rows (0x8..0x24,
/// descending) and the selection state; return `self`.
pub fn c_fe_state_bragging_rights_init_player_rows(c: &mut Cpu) {
    c.cycles += 6;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = 0x0047_0000;
    c.g[V1][0] = 0x7;
    c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x4f88)); // &D_0046B078
    Addr(this).off(0x30).sw(c, c.g[V0][0]);
    c.g[V0][0] = sx(this.wrapping_add(0x24));
    loop {
        c.cycles += 7;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0019_2708);
        }
    }
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    Addr(this).sw(c, 0);
    Addr(this).off(0x2c).sw(c, 0);
    Addr(this).off(0x28).sw(c, 0);
    Addr(this).off(4).sw(c, 0);
    crate::abi::ret(c);
}


/// `void cFEStateBraggingRights_SetRowsVisible(cFEState* self, int a1)`
/// (0x192740): tell up to eight child rows they are visible or not.
pub fn c_fe_state_bragging_rights_set_rows_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(at(c, A0, 8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 7;
        Addr(at(c, A0, 0x28)).sw(c, c.g[S2][0]);
        c.g[A2][0] = Addr(at(c, S0, 0)).lw(c);
        loop {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = sx(at(c, S0, 4));
            let none = c.g[A2][0] == 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(at(c, A2, 8)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x40)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x44)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019_278c)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[A2][0] = Addr(at(c, S0, 0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_2768);
            }
        }
    })
}


/// `cFEStateBraggingRights_ctor(self, ...)` (0x192d00): construct the state,
/// set its vtable and clear the two player rows (`0x34` bytes each).
pub fn cfestatebraggingrights_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6d0));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        call_ptr(c, 0x001a_8500, 0x0019_2d2c)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5098));
        c.sw(p(r32(c, S2), 8), c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            call_ptr(
                c,
                callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS.addr,
                0x0019_2d4c,
            )?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_2d40);
            }
        }
        c.cycles += 11;
        c.g[V1][0] = 0x14;
        c.g[V0][0] = c.g[S2][0];
        c.sw(p(r32(c, S2), 0xc), c.g[V1][0]);
        c.sw(p(r32(c, S2), 0x738), 0);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::*;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;


    #[test]
    fn cfe_state_bragging_rights_is_row_enabled_matches() {
        check(ee::s02::f_00192948__generated, cfe_state_bragging_rights_is_row_enabled, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x19_2948, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 48) as u64;
            c.bus.write32(DATA + 0x2c, r.next_u32());
        });
    }


    #[test]
    fn c_fe_state_rule_select_v21_matches() {
        check(ssx3_ee::s02::f_00191e08__generated, c_fe_state_rule_select_v21, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x191e_08, 0);
            c.g[GP][0] = GAME_GP as u64;
            let msg = DATA + 0x800;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = msg as u64;
            c.bus.write32(msg + 0x38, r.next_u32());
        });
    }


    // -----------------------------------------------------------------------
    // 0x1926f0 cFEStateBraggingRights_InitPlayerRows.

    #[test]
    fn c_fe_state_bragging_rights_init_player_rows_matches() {
        check(ee::s02::f_001926f0__generated, c_fe_state_bragging_rights_init_player_rows, 8, &|c, seed| {
            let _r = machine(c, seed, 0x1926_f0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
        });
    }


    #[test]
    fn c_fe_state_bragging_rights_set_rows_visible_matches() {
        check(
            ee::s02::f_00192740__generated,
            c_fe_state_bragging_rights_set_rows_visible,
            60,
            &|c, seed| {
                machine(c, seed, 0x1927_40, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                for i in 0..8u32 {
                    let p = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x1000 + i * 0x100
                    };
                    c.bus.write32(DATA + 8 + i * 4, p);
                    if p != 0 {
                        put_vtable(c, p, &[0x40]);
                    }
                }
            },
        );
    }


        #[test]
    fn cfestatebraggingrights_ctor_matches() {
        check(
            ee::s02::f_00192d00__generated,
            cfestatebraggingrights_ctor,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x2d00, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }
}
