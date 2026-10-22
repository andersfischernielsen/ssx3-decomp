//! `src/fe/festateriderbio.cpp`: hand-written replacements (moved from the
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
use crate::call;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::W;
use crate::call::sp;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::feasyncfile::lh_2 as lh;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::festateoptions::STR_HASH;
pub(crate) use crate::game::fe::festateoptions::UI_SCREEN_FIND_OBJECT;
pub(crate) use crate::game::fe::festateoptions::screen_object;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_3550, c_fe_state_rider_detail_on_render, "cFEStateRiderDetail_OnRender"),
    (0x0018_35c8, cfe_state_rider_detail_update, "cFEStateRiderDetail_Update"),
    (0x0018_3a98, c_fe_state_uber_trick_ctor, "cFEStateUberTrick_ctor"),
    (0x0018_3b08, cfestate_uber_trick_select_left_panel, "cFEStateUberTrick_SelectLeftPanel"),
    (0x0018_3c08, cfestate_uber_trick_select_right_panel, "cFEStateUberTrick_SelectRightPanel"),
    (0x0018_3d08, cfestateubertrick_set_button_icon, "cFEStateUberTrick_SetButtonIcon"),
];

/// Direct calls through the generated function (which dispatches into the
/// replacement when one is installed).
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
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;


    pub const C_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::f_00186518,
    };

    pub const SOUND_MANAGER_REF_DTOR: Callee = Callee {
        addr: 0x002a_dee8,
        f: ee::s08::f_002adee8,
    };
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

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
}


/// `void cFEStateRiderDetail_Update(cFEStateRiderDetail* self)` (0x1835c8):
/// same shape as the character-setup state.
pub fn cfe_state_rider_detail_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_35dc)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW.call(c, 0x0018_35e4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateRiderDetail_OnRender(cFEStateRiderDetail* self)`
/// (0x183550): resolve the "rider detail" panel and fade it in.
pub fn c_fe_state_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0x50)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x45_d898; // D_0045D898
            call_ptr(c, table::STR_HASH, 0x0018_3578)?;
            c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x0018_3584)?;
            let none = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x10);
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_3594)?;
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateUberTrick* cFEStateUberTrick_ctor(cFEStateUberTrick* s, int a1,
/// signed char idx)` (0x183a98).
pub fn c_fe_state_uber_trick_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save_ra(c, 0);
        let s1 = c.g[S1][0] as u32;
        c.g[S1][0] = sx(((s1 as i32) >> 24) as u32);
        callee::FE_STATE_CTOR.call(c, 0x0018_3ab8)?;
        c.g[V0][0] = 0x12;
        c.g[V1][0] = 0x0046_ccc0;
        Addr(p_2(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(p_2(r32(c, S0), 0x44)).sb(c, c.g[S1][0]);
        c.g[V0][0] = Addr(p_2(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call::call_ptr(c, 0x001a_1cd0, 0x0018_3ae0)?;
        Addr(p_2(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0x50)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0x48)).sw(c, 0);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}


/// `void cFEStateUberTrick_SetButtonIcon(self, Rect* rect)` (0x183d08): pass
/// the rect's size (scaled) to the icon widget, or hide it when there is no
/// rect.
pub fn cfestateubertrick_set_button_icon(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.swc1(20, frame.at(c, 0x40));
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let no_rect = c.g[S0][0] == 0;
        c.mov_s(20, 12);
        if !no_rect {
            c.cycles += 33;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = Addr(p(r32(c, V0), 0xa0)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xa4)).lw(c);
            let getpos = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, getpos, 0x0018_3d44)?;
            c.lwc1(4, p(r32(c, S0), 0xc));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(2, p(r32(c, S0), 0x14));
            c.lwc1(3, p(r32(c, S0), 0x10));
            c.lwc1(1, p(r32(c, S0), 0x18));
            c.g[AT][0] = 0x43800000;
            c.mtc1(1, 0);
            c.sub_s(2, 2, 3);
            c.sub_s(1, 1, 4);
            c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.mul_s(0, 20, 0);
            c.mul_s(1, 1, 0);
            c.mul_s(2, 2, 0);
            c.swc1(1, p(r32(c, SP), 4));
            c.swc1(2, r32(c, SP));
            c.g[A0][0] = Addr(p(r32(c, V1), 0xa8)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0xac)).lw(c);
            let setpos = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, setpos, 0x0018_3d90)?;
            c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
            let show = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, show, 0x0018_3da8)?;
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, show, 0x0018_3dc8)?;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(20, frame.at(c, 0x40));
        frame.close(c);
        Ok(())
    })
}


/// One `select panel` step: look up the item on the screen by string id and
/// set its enabled flag (or clear it when it is missing).
pub(crate) fn panel_select(c: &mut Cpu, this: u32, string_off: u32, flag: u64, ret: u32, charge: u32) -> crate::call::Flow {
    c.cycles += charge as u64;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(string_off));
    call_ptr(c, 0x0031_7670, ret.wrapping_sub(0x2c))?; // StrHash
    c.g[A0][0] = lw(c, p(this, 0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret.wrapping_sub(0x20))?; // UIScreen_FindObject
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = flag;
    if !missing {
        c.cycles += 5;
        c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 8));
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x48));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, ret)?;
    }
    Ok(())
}


/// `cFEStateUberTrick_SelectLeftPanel(self)` (0x183b08): mark the left-hand
/// UberTrick panel items enabled/disabled and refresh their state.
pub fn cfestate_uber_trick_select_left_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        panel_select(c, this, 0x2700, 0, 0x0018_3b50, 0)?;
        panel_select(c, this, 0x26f0, 1, 0x0018_3b88, 9)?;
        panel_select(c, this, 0x26e0, 1, 0x0018_3bc0, 9)?;
        panel_select(c, this, 0x26d0, 0, 0x0018_3bf8, 9)?;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateUberTrick_SelectRightPanel(self)` (0x183c08): the mirrored
/// version of [`cfestate_uber_trick_select_left_panel`].
pub fn cfestate_uber_trick_select_right_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        panel_select(c, this, 0x2700, 1, 0x0018_3c50, 0)?;
        panel_select(c, this, 0x26f0, 0, 0x0018_3c88, 9)?;
        panel_select(c, this, 0x26e0, 0, 0x0018_3cc0, 9)?;
        panel_select(c, this, 0x26d0, 1, 0x0018_3cf8, 9)?;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The same, fetching the function pointer into `v1` and the object's model
/// pointer (`*(obj+8)`) into `v0`.
pub(crate) fn call_object_v1(c: &mut Cpu, obj: u64, arg: u64, ret: u32) -> Flow {
    c.g[V0][0] = Addr(obj as u32).off(8).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
    let func = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}


// ---------------------------------------------------------------------------
// cFEStateRiderDetail_OnActivate.

/// The widget names `cFEStateRiderDetail_OnActivate` tags with state 5..9.
pub(crate) const RIDER_DETAIL_WIDGETS: [(u32, u64); 5] = [
    (0x0045_d888, 5),
    (0x0045_d898, 6),
    (0x0045_d8a8, 7),
    (0x0045_d8b8, 8),
    (0x0045_d8c8, 9),
];


/// `void cFEStateRiderDetail_OnActivate(FEState *s, int arg)` (0x1833d0): put
/// five rider-detail widgets into their state, then run the base activate.
pub fn c_fe_state_rider_detail_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);

        let this = r32(c, S1);
        for (i, &(name, state)) in RIDER_DETAIL_WIDGETS.iter().enumerate() {
            if i > 0 {
                c.cycles += 9;
            }
            let ret = 0x0018_33f8 + 0x40 * i as u32;
            screen_object(c, this, name, ret, ret + 0xc)?;
            let obj = c.g[V0][0];
            c.g[A1][0] = 1;
            if obj == 0 {
                continue;
            }
            c.cycles += 7;
            call_object_v1(c, obj, 1, ret + 0x2c)?;
            c.g[V0][0] = state;
            Addr(obj as u32).off(0x18).sw(c, c.g[V0][0]);
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::C_FE_STATE_ON_ACTIVATE.call(c, 0x0018_3538)?;

        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// SndPlayParams_dtor.

pub(crate) const SOUND_MANAGER_REF_DTOR: u32 = 0x002a_dee8;

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
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::put_vec;
    use crate::difftest::SP;
    use ssx3_ee as ee;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::compare;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::sound::soundsys::tests::BE;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    pub(crate) use crate::game::sound::soundsys::tests::w;
    pub(crate) use crate::game::sound::soundsys::tests::wf;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_2;


    #[test]
    fn cfe_state_rider_detail_update_matches() {
        check(s02::f_001835c8__generated, cfe_state_rider_detail_update, 120, &|c, seed| {
            machine(c, seed, 0x1835c8, 2);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    #[test]
    fn c_fe_state_rider_detail_on_render_matches() {
        check(
            ee::s02::f_00183550__generated,
            c_fe_state_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1835_50, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let screen = DATA + 0x800;
                clear_2(c, this, 0x100);
                clear_2(c, screen, 0x200);
                c.bus.write32(this + 0x40, screen);
                c.bus
                    .write32(this + 0x50, if seed % 2 == 0 { 0 } else { 1 });
                put_words(c, 0x0045_d898, &[0x5449_544c, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn c_fe_state_uber_trick_ctor_matches() {
        check(ssx3_ee::s02::f_00183a98__generated, c_fe_state_uber_trick_ctor, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x183a98, 0);
            let s = DATA + 0x1000;
            let engine = DATA + 0x2000;
            let names = DATA + 0x3000;
            c.g[A0][0] = s as u64;
            c.g[A1][0] = engine as u64;
            c.g[A2][0] = (seed % 8) as u64;
            w32(c, s + 0x10, engine);
            w32(c, engine, names);
            for i in 0..8u32 {
                w8(c, names + i + 8, r.next_u32() as u8);
            }
        });
    }


    #[test]
    fn cfestateubertrick_set_button_icon_matches() {
        check(
            ee::s02::f_00183d08__generated,
            cfestateubertrick_set_button_icon,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x183d08, 3);
                let this = DATA;
                clear_3(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let rect = DATA + 0x200;
                clear_3(c, rect, 0x100);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { rect as u64 };
                c.f[12] = if seed % 2 == 0 {
                    1.0f32.to_bits()
                } else {
                    0.5f32.to_bits()
                };
                wf(c, rect + 0xc, 1.0);
                wf(c, rect + 0x10, 2.0);
                wf(c, rect + 0x14, 3.0);
                wf(c, rect + 0x18, 4.0);
                let vt = vtable(c, &mut r, &[0xa0, 0xa8, 0x48]);
                w(c, this + 8, vt);
            },
        );
    }


        #[test]
    fn cfestate_uber_trick_select_left_panel_matches() {
        strict(
            ssx3_ee::s02::f_00183b08__generated,
            cfestate_uber_trick_select_left_panel,
            SEEDS,
            &|c, seed| {
                let _ = base_2(c, seed, 0x3b08);
                let this = DATA + 0x1000;
                // No screen: every item lookup misses and only the flags of
                // the (absent) items are touched.
                w(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn cfestate_uber_trick_select_right_panel_matches() {
        strict(
            ssx3_ee::s02::f_00183c08__generated,
            cfestate_uber_trick_select_right_panel,
            SEEDS,
            &|c, seed| {
                let _ = base_2(c, seed, 0x3c08);
                let this = DATA + 0x1000;
                w(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
