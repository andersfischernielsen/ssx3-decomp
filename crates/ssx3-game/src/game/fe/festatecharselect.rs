//! `src/fe/festatecharselect.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::sp;
use crate::call::Flow;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::call::Unwound;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::bx::bxstring2::gp_4 as gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_1148, cfestate_charselect_init, "cFEStateCharSelect_Init"),
    (0x0018_1238, c_fe_state_char_select_shutdown, "cFEStateCharSelect_Shutdown"),
    (0x0018_12a8, c_fe_state_char_select_transition_in, "cFEStateCharSelect_TransitionIn"),
    (0x0018_1420, cfe_state_char_select_update, "cFEStateCharSelect_Update"),
    (0x0018_2420, c_fe_state_cheat_char_select_init, "cFEStateCheatCharSelect_Init"),
    (0x0018_2808, cfe_state_cheat_char_select_on_widget_notify, "cFEStateCheatCharSelect_OnWidgetNotify"),
    (0x0018_2870, cfe_state_cheat_char_select_apply_outfit, "cFEStateCheatCharSelect_ApplyOutfit"),
    (0x0018_28c0, cfe_state_char_setup_ctor, "cFEStateCharSetup_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
}


// ---------------------------------------------------------------------------
// Frontend states.

/// `void cFEStateCharSelect_Update(cFEStateCharSelect* self)` (0x181420): the
/// rider-preview update, then the base state's update.
pub fn cfe_state_char_select_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW.call(c, 0x0018_1434)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_143c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Front-end states: activate / refresh / shutdown hooks.

/// `void cFEStateCheatCharSelect_ApplyOutfit(cFEStateCheatCharSelect* s)`
/// (0x182870): for the six outfit slots at +0x54, point each non-null item's
/// +0x78 at -1 and its +0x7c at the current outfit table entry
/// (`s+0xf4+4*index`). 8 + 6×(3 (+4) + 3) + 2 cycles.
pub fn cfe_state_cheat_char_select_apply_outfit(c: &mut Cpu) {
    c.cycles += 8;
    let self_ = r32(c, A0);
    let owner = c.lw(p(self_, 0x6c));
    c.g[V1][0] = owner;
    c.g[A1][0] = sx(p(self_, 0xf4));
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(self_, 0x54));
    let index = c.lbu(p(owner as u32, 0x98));
    c.g[V0][0] = index;
    c.g[A2][0] = 5;
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    loop {
        c.cycles += 3;
        let item = c.lw(r32(c, A0));
        c.g[V1][0] = item;
        let next = p(r32(c, A1), 0);
        if item == 0 {
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(next);
            c.sw(p(item as u32, 0x78), c.g[A3][0]);
            c.sw(p(item as u32, 0x7c), c.g[V0][0]);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
        }
        c.cycles += 3;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0018_2890);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}


/// `int cFEStateCharSelect_TransitionIn(cFEStateCharSelect* self, void* on)`
/// (0x1812a8): refresh the selected character's attribute bars on entry.
pub fn c_fe_state_char_select_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let inactive = r32(c, A1) == 0;
        c.g[S0][0] = c.g[A0][0];
        if !inactive {
            c.cycles += 13;
            c.g[RA][0] = sx(0x0018_12c4);
            call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0018_12c4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0018_12d0);
            c.g[A1][0] = 1;
            call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0018_12d0)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x44).lb(c);
            c.g[RA][0] = sx(0x0018_12dc);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT, 0x0018_12dc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0) << 24);
            c.g[A1][0] = Addr(r32(c, A0)).off(0x44).lb(c);
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 24) as u32);
            c.g[RA][0] = sx(0x0018_12f0);
            callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS.call(c, 0x0018_12f0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCheatCharSelect_Init(cFEStateCheatCharSelect* self)` (0x182420).
pub fn c_fe_state_cheat_char_select_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_d780, [0x0018_2444, 0x0018_2458, 0x0018_2470])
    })
}


/// `cFEStateCharSetup* cFEStateCharSetup_ctor(self, engine, signed char idx)`
/// (0x1828c0): `UIState` constructor, this class's vtable, the player index
/// byte and the pad mask for that player.
pub fn cfe_state_char_setup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0018_28e0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x0018_28e0)?;
        c.g[V0][0] = 0xc;
        c.g[V1][0] = 0x470000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x31a0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sb((c.g[S0][0] as u32).wrapping_add(0x44), c.g[S1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x0018_2908);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        call_ptr(c, 0x001a_1cd0, 0x0018_2908)?;
        c.sb((c.g[S0][0] as u32).wrapping_add(0x15), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state helpers.

/// `int cFEStateCheatCharSelect_OnWidgetNotify(self, item, int msg)`
/// (0x182808): route widget notifications to `ApplyOutfit`; return 0x100 for
/// the range messages 7..9, 0x101 otherwise, and for message 1 whether the
/// value is below the state's limit.
pub fn cfe_state_cheat_char_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        open(c, 0x10);
        c.g[V0][0] = 0x5;
        let msg = c.g[A2][0];
        let is5 = msg == 5;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if !is5 {
            c.cycles += 3;
            c.g[V0][0] = (msg < 6) as u64;
            if msg < 6 {
                c.cycles += 2;
                if msg == 1 {
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                }
            } else {
                // The second compare block's own +3 (the one above is the
                // first compare's), charged on every msg >= 6 path.
                c.cycles += 3;
                c.g[V0][0] = (msg < 0xa) as u64;
                if msg < 0xa {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                    if msg >= 7 {
                        c.cycles += 2;
                        c.g[V0][0] = 0x100;
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0x101;
                }
            }
        } else {
            c.cycles += 2;
            call_ptr(c, 0x0018_2870, 0x0018_2860)?;
            c.cycles += 1;
            c.g[V0][0] = 0x101;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x10);
        Ok(())
    })
}


/// `void cFEStateCharSelect_Shutdown(cFEStateCharSelect* self)` (0x181238):
/// release the BE interface, hide the rider preview when riders exist and
/// shut the UI state down.
pub fn c_fe_state_char_select_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_124c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0018_1258)?;
        c.g[V1][0] = Addr(gpr(c, -0x848)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x7c)).lw(c);
        let no_riders = c.g[A0][0] == 0;
        c.g[V0][0] = 0x530000;
        let mut hide = false;
        if !no_riders {
            c.cycles += 3;
            c.g[V1][0] = Addr(at(c, V0, 0x5c11)).lb(c);
            hide = c.g[V1][0] != 0;
        }
        if hide {
            c.cycles += 6;
            c.g[A1][0] = Addr(at(c, S0, 0x44)).lb(c);
            c.g[A0][0] = sx(at(c, A0, 0xb0));
            call_ptr(c, 0x001a_0548, 0x0018_1280)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0019_e538, 0x0018_128c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e4a0, 0x0018_1294)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateCharSelect_Init(self)` (0x181148): build the character-select
/// screen: create its menu from the state's menu hash, then lay out the
/// rider preview, the confirm prompts and the enter animation.
pub fn cfestate_charselect_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        frame.save(c, 16, 0x40);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a00));
        c.g[RA][0] = sx(0x0018_116c);
        c.g[S0][0] = lw_2(c, p(c.g[S1][0] as u32, 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_116c)?; // StrHash(name)
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0018_1180);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_1180)?; // the screen's menu
        let t = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x0018_1198);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_1198)?;
        }
        c.cycles += 40;
        c.g[V0][0] = lw_2(c, gp(c, -0x848));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[A2][0] = sx(p(sp(c), 0x10));
        c.g[S0][0] = lw_2(c, p(c.g[V0][0] as u32, 0x7c));
        c.g[AT][0] = 0x4348_0000;
        c.mtc1(1, 0);
        c.swc1(1, p(sp(c), 0x1c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.swc1(0, p(sp(c), 4));
        c.sw(sp(c), 0);
        c.sw(p(sp(c), 8), 0);
        c.swc1(1, p(sp(c), 0xc));
        c.sw(p(sp(c), 0x10), 0);
        c.sw(p(sp(c), 0x14), 0);
        c.g[RA][0] = sx(0x0018_11e0);
        c.sw(p(sp(c), 0x18), 0);
        call_ptr(c, 0x0015_e050, 0x0018_11e0)?;
        c.lwc1(0, gp(c, -0x5a28));
        c.g[V0][0] = lw_2(c, gp(c, -0x848));
        c.swc1(0, p(c.g[S0][0] as u32, 0x10));
        c.g[A0][0] = lw_2(c, p(c.g[V0][0] as u32, 0x7c));
        c.g[A1][0] = c.lb(p(c.g[S1][0] as u32, 0x44));
        c.g[RA][0] = sx(0x0018_11fc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a_0548, 0x0018_11fc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018_1208);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0019_e538, 0x0018_1208)?;
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(1, 0);
        c.g[V0][0] = 1;
        c.sw(p(c.g[S1][0] as u32, 0x80), c.g[V0][0]);
        c.swc1(0, p(c.g[S1][0] as u32, 0x5c));
        c.sw(p(c.g[S1][0] as u32, 0x60), 0);
        c.sw(p(c.g[S1][0] as u32, 0x58), 0);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee::s01;
    use ssx3_ee::s03;
    use ssx3_ee::s00;
    use ssx3_ee::s02;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::fakes::vtable;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::compare;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::clear as clear_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::w;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_3;


    // -----------------------------------------------------------------
    // Frontend states.

    #[test]
    fn cfe_state_char_select_update_matches() {
        check(s02::f_00181420__generated, cfe_state_char_select_update, 120, &|c, seed| {
            machine(c, seed, 0x181420, 2);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    // ---------------------------------------------------- front-end states

    #[test]
    fn cfe_state_cheat_char_select_apply_outfit_matches() {
        check(ee::s02::f_00182870__generated, cfe_state_cheat_char_select_apply_outfit, 80, &|c, seed| {
            let mut r = Rng(0x1828_7042 ^ seed as u64);
            scramble(c, &mut r);
            let s = DATA;
            let owner = DATA + 0x400;
            c.bus.write32(s + 0x6c, owner);
            c.bus.write8(owner + 0x98, (r.next_u32() % 4) as u8);
            for k in 0..6u32 {
                let item = if k % 3 == 0 || seed % 3 == 0 { 0 } else { DATA + 0x1000 + 0x100 * k };
                c.bus.write32(s + 0x54 + 4 * k, item);
            }
            fill(c, &mut r, s + 0xf4, 8);
            c.g[A0][0] = sx(s);
        });
    }


    #[test]
    fn c_fe_state_char_select_transition_in_matches() {
        check(ee::s02::f_001812a8__generated, c_fe_state_char_select_transition_in, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x1812a8, 3);
            let this = DATA + 0x100;
            clear_2(c, this, 0x100);
            c.bus.write8(this + 0x44, (seed % 12) as u8);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { seed as u64 + 1 };
        });
    }


    #[test]
    fn c_fe_state_cheat_char_select_init_matches() {
        check(
            ssx3_ee::s02::f_00182420__generated,
            c_fe_state_cheat_char_select_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x182420);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }


    #[test]
    fn cfe_state_cheat_char_select_on_widget_notify_matches() {
        check(ee::s02::f_00182808__generated, cfe_state_cheat_char_select_on_widget_notify, 60, &|c, seed| {
            let mut r = base(c, seed, 0x18_2808);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = (seed % 13) as u64;
            c.g[A3][0] = (r.next_u32() % 8) as u64;
            c.bus.write32(DATA + 0x78, (r.next_u32() % 8) as u32);
            // `ApplyOutfit` reads a class table through self+0x6c and walks six
            // entries at self+0x54; all zeroed.
            c.bus.write32(DATA + 0x6c, DATA + 0x300);
            c.bus.write8(DATA + 0x300 + 0x98, (seed % 4) as u8);
        });
    }


    #[test]
    fn cfe_state_char_setup_ctor_matches() {
        check(ee::s02::f_001828c0__generated, cfe_state_char_setup_ctor, 30, &|c, seed| {
            let mut r = base(c, seed, 0x18_28c0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
            let idx = seed % 4;
            c.g[A2][0] = idx as u64;
            c.bus.write32(DATA + 0x200, DATA + 0x300);
            c.bus.write8(DATA + 0x300 + idx + 8, (r.next_u32() & 0x77) as u8);
        });
    }


    #[test]
    fn c_fe_state_char_select_shutdown_matches() {
        check(
            ee::s02::f_00181238__generated,
            c_fe_state_char_select_shutdown,
            40,
            &|c, seed| {
                base_2(c, seed, 0x1812_38);
                let iface = DATA + 0x3000;
                let vt = DATA + 0x3200;
                c.bus.write32(0x5308_a8, iface);
                c.bus.write32(iface + 4, vt);
                put_vtable(c, vt, &[8]);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(W + 0x7c, 0);
            },
        );
    }


    #[test]
    fn cfestate_charselect_init_matches() {
        strict(ee::s02::f_00181148__generated, cfestate_charselect_init, SEEDS_2, &|c, seed| {
            let mut r = base_3(c, seed, 0x1148, 0);
            let this = DATA + 0x100;
            let world = DATA + 0x8_0000;
            w(c, world + 0x7c, DATA + 0x2000);
            w(c, this + 0x10, if seed % 3 == 0 { 0 } else { r.next_u32() & !7 });
            c.bus.write8(this + 0x44, (seed % 4) as u8);
            c.g[A0][0] = sx(this);
        });
    }
}
