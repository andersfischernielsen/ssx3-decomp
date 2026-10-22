//! `src/be/be.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::events::deliver_event_here;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use crate::call::sp;
use crate::abi::r32;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::camera::camera::gp;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_dd98, be_library_shutdown, "BELibrary_Shutdown"),
    (0x0014_de28, be_reset_player_profile, "BE_ResetPlayerProfile"),
    (0x0014_de68, be_library_set_phase, "BELibrary_SetPhase"),
    (0x0014_dfa8, be_library_load_state, "BELibrary_LoadState"),
    (0x0014_e048, be_library_get_save_data_size, "BELibrary_GetSaveDataSize"),
    (0x0014_e050, be_library_write_save_data, "BELibrary_WriteSaveData"),
    (0x0014_e0c0, be_library_read_save_data, "BELibrary_ReadSaveData"),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
}


// ---------------------------------------------------------------------------
// Collected-mission bit fields and the save round-trip.

pub(crate) const BELIBRARY_DESERIALIZE_SAVE: Callee = Callee { addr: 0x0014_a5e0, f: ssx3_ee::s01::f_0014a5e0 };


/// `bool BELibrary_ReadSaveData(BELibrary *self, const void *data)` (0x14e0c0):
/// `BELibrary_DeserializeSave(data)`.
pub fn be_library_read_save_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BELIBRARY_DESERIALIZE_SAVE.call(c, 0x0014_e0d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Completed-mission counts and save writing.

pub(crate) const BELIBRARY_SERIALIZE_SAVE: Callee = Callee { addr: 0x0014_a188, f: ssx3_ee::s01::f_0014a188 };

pub(crate) const BELIBRARY_GET_SAVE_SIZE: Callee = Callee { addr: 0x0014_e048, f: ssx3_ee::s01::f_0014e048 };


/// `void BELibrary_WriteSaveData(BELibrary *self, void *out)` (0x14e050):
/// serialise the state, clear the header, and append a trailer.
pub fn be_library_write_save_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        BELIBRARY_SERIALIZE_SAVE.call(c, 0x0014_e070)?;
        c.g[A0][0] = c.g[S1][0];
        BELIBRARY_GET_SAVE_SIZE.call(c, 0x0014_e078)?;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        c.g[A2][0] = 0xfbea;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_62d0, 0x0014_e088)?;
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        BELIBRARY_GET_SAVE_SIZE.call(c, 0x0014_e094)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        frame.restore(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0x10);
        c.lwl(V0, (c.g[SP][0] as u32).wrapping_add(3));
        c.lwr(V0, c.g[SP][0] as u32);
        c.swl(V0, (c.g[S0][0] as u32).wrapping_add(3));
        c.swr(V0, c.g[S0][0] as u32);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Save-image sizes and singletons.

/// `int BELibrary_GetSaveDataSize(BELibrary* be)` (0x14e048): the size in
/// bytes of one serialized profile save.
pub fn be_library_get_save_data_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xbb78;
    abi::ret(c);
}


/// `void BE_ResetPlayerProfile(int player)` (0x14de28): reset the BE library's
/// profile (when there is one) and the BAGT profile of player `player`.
pub fn be_reset_player_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x10);
        let library = gp(c, -0x1ef8);
        c.g[A0][0] = Addr(library).lw(c);
        c.g[S0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        let present = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        if present {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            callee::BE_LIBRARY_RESET_PROFILE.call(c, 0x0014_de4c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::BEBAGT_RESET_PROFILE.call(c, 0x0014_de54)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// A vtable dispatch: `lh(vt + delta); lw(vt + delta + 4); a0 = obj + delta`.
#[inline(always)]
pub(crate) fn vt_call(c: &mut Cpu, vt: u64, delta_off: u32, obj: u64, func_reg: usize, ret: u32) -> Flow {
    let base = (vt as u32).wrapping_add(delta_off);
    let delta = c.lh(base);
    let func = c.lw(base.wrapping_add(4));
    c.g[func_reg][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}


/// `void BELibrary_SetPhase(int phase)` (0x0014de68): leave the previous
/// phase's library (vtable `+0x18`/`+0x1c`) and enter the new one (vtable
/// `+0x20`/`+0x24`), recording the phase in `gp-0x1efc`.
pub fn be_library_set_phase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpn(c, 0x1efc));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        let same = c.g[V0][0] == c.g[S1][0];
        c.sd(sp(c), c.g[RA][0]);
        if !same {
            c.cycles += 19;
            c.g[S0][0] = sx(0x0053_08a8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
            let vt = c.g[V1][0];
            let obj = c.g[A2][0];
            vt_call(c, vt, 0x18, obj, A1, 0x0014_deac)?;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            c.sw(gpn(c, 0x1efc), c.g[S1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(4));
            let vt = c.g[V1][0];
            let obj = c.g[A1][0];
            vt_call(c, vt, 0x20, obj, V0, 0x0014_ded0)?;
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// BE library.

/// `void BELibrary_Shutdown()` (0x14dd98): destroy the BE library, shut the
/// three BE states down and free the singleton.
pub fn be_library_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x1ef8)).lw(c);
        frame.save(c, S0, 0x10);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::BELIBRARY_DTOR.call(c, 0x0014_ddb4)?;
        }
        c.cycles += 28;
        c.g[S0][0] = 0x0053_0000;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x8a8)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x8a8));
        c.g[V0][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x0014_ddd4)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x0014_ddec)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x0014_de04)?;
        c.g[A0][0] = Addr(gp(c, -0x1f04)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0014_de0c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, -0x1f04)).sw(c, 0);
        Addr(gp(c, -0x1f00)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// BE library.

/// `void BELibrary_LoadState(BELibrary* library, void* request)` (0x14dfa8):
/// run the request's start method, then load the 14 interfaces through
/// `BELibrary_GetInterface`, calling each interface's load callback with the
/// request.
pub fn be_library_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = sx(gp_2(c, -0x1efc));
        c.g[V0][0] = lw(c, c.g[S2][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x14));
        c.g[V1][0] = sx(c.g[V1][0] as u32);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0014_dfe4)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_dd58, 0x0014_dff0)?; // GetInterface
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            let missing = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            if !missing {
                c.cycles += 8;
                call_ptr(c, 0x0014_dd58, 0x0014_e008)?; // GetInterface
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x20)).lh(c);
                c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, 0x24));
                c.g[A2][0] = sx(c.g[A2][0] as u32);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[A2][0] as u32, 0x0014_e020)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 14) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_dfe8);
                }
                continue;
            }
            break;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fill;
    use crate::difftest::check_lohi;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use ssx3_ee as ee;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::vtable;
    use crate::difftest::elf_image;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;


    #[test]
    fn be_library_read_save_data_matches() {
        check(ssx3_ee::s01::f_0014e0c0, be_library_read_save_data, 30, &|c, seed| {
            let mut r = machine(c, seed, 0xe0c0, 0);
            c.g[A1][0] = (DATA + 0x400) as u64;
            fill(c, &mut r, DATA, 0x1000);
        });
    }


    #[test]
    fn be_library_write_save_data_matches() {
        check(ssx3_ee::s01::f_0014e050, be_library_write_save_data, 30, &|c, seed| {
            let mut r = machine(c, seed, 0xe050, 0);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            fill(c, &mut r, DATA, 0x4000);
        });
    }


    #[test]
    fn be_library_get_save_data_size_matches() {
        check(ee::s01::f_0014e048__generated, be_library_get_save_data_size, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_e048, seed);
            scramble(c, &mut r);
        });
    }


    #[test]
    fn be_reset_player_profile_matches() {
        check(ssx3_ee::s01::f_0014de28__generated, be_reset_player_profile, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x14de_28, 0);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = r.next_u32() as u64;
            // No BE library: the first call is skipped for every second seed.
            let library = if seed % 2 == 0 { 0 } else { DATA + 0x6000 };
            c.bus.write32(GAME_GP.wrapping_sub(0x1ef8), library);
        });
    }


    #[test]
    fn be_library_set_phase_matches() {
        check(ee::s01::f_0014de68__generated, be_library_set_phase, 80, &|c, seed| {
            let _ = base(c, seed, 0x14de, 2);
            let old = (seed % 4) as u32;
            let new = if seed % 2 == 0 { old } else { (seed / 4) % 4 };
            w(c, GAME_GP.wrapping_sub(0x1efc), old);
            c.g[A0][0] = new as u64;
        });
    }


    #[test]
    fn be_library_shutdown_matches() {
        check(ee::s01::f_0014dd98__generated, be_library_shutdown, 60, &|c, seed| {
            let _ = base_2(c, seed, 0x14_dd98);
            globals(c);
            // No BE library and no BE states: every call is a no-op or fake.
            w(c, GAME_GP.wrapping_sub(0x1ef8), 0);
            w(c, GAME_GP.wrapping_sub(0x1f04), 0);
            w(c, GAME_GP.wrapping_sub(0x1f00), 0);
            for i in 0..3u32 {
                w(c, 0x0053_08a8 + 4 * i, 0);
            }
        });
    }


    // -------------------------------------------------------- BE_2 library

    #[test]
    fn be_library_load_state_matches() {
        check(ssx3_ee::s01::f_0014dfa8__generated, be_library_load_state, 8, &|c, seed| {
            let _ = base_3(c, seed, 0x0014_dfa8);
            let lib = DATA + 0x100;
            let request = DATA + 0x2000;
            let vtable = DATA + 0x3000;
            clear(c, lib, 0x100);
            clear(c, request, 0x100);
            clear(c, vtable, 0x80);
            w(c, request, vtable);
            c.bus.write16(vtable + 0x10, 0);
            w(c, vtable + 0x14, crate::difftest::fakes::FAKE);
            w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            c.g[A0][0] = sx(lib);
            c.g[A1][0] = sx(request);
        });
    }
}
