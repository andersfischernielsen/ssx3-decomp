//! `src/be/beintoption.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::guest;
use crate::call::Flow;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use crate::events::back_edge_unwinds;
use crate::call::sp;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintnetwork::OPTION_GP;
pub(crate) use crate::game::be::bedbreward::destroy_instance;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::be::beintnetwork::copy_block;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::fe::debugfe::r32_2 as r32;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_f2a8, be_option_int_commit, "BEOptionInt_Commit"),
    (0x0014_f360, be_option_int_reset, "BEOptionInt_Reset"),
    (0x0014_f418, be_option_int_ctor, "BEOptionInt_ctor"),
    (0x0014_f458, be_option_int_set_sound_defaults, "BEOptionInt_SetSoundDefaults"),
    (0x0014_f4f8, be_option_int_set_game_defaults, "BEOptionInt_SetGameDefaults"),
    (0x0014_f600, be_option_int_set_hud_defaults, "BEOptionInt_SetHUDDefaults"),
    (0x0014_f648, beoption_int_reset_singleton, "BEOptionInt_ResetSingleton"),
    (0x0014_f678, be_option_int_get_quick_message_key, "BEOptionInt_GetQuickMessageKey"),
    (0x0014_f6a8, be_option_int_get_quick_message, "BEOptionInt_GetQuickMessage"),
    (0x0014_f6e8, be_option_int_reset_quick_message, "BEOptionInt_ResetQuickMessage"),
    (0x0014_f758, be_option_int_reset_quick_messages, "BEOptionInt_ResetQuickMessages"),
    (0x0014_f7a8, be_option_int_get_player_handicap, "BEOptionInt_GetPlayerHandicap"),
    (0x0014_f7e8, be_option_int_get_computer_handicap, "BEOptionInt_GetComputerHandicap"),
    (0x0014_f810, be_option_int_is_any_handicap_set, "BEOptionInt_IsAnyHandicapSet"),
    (0x0014_f870, be_option_int_get_max_handicap, "BEOptionInt_GetMaxHandicap"),
    (0x0014_f898, be_option_int_swap_player_settings, "BEOptionInt_SwapPlayerSettings"),
    (0x0015_bd30, be_option_int_destroy_instance, "BEOptionInt_DestroyInstance"),
];

// ---------------------------------------------------------------------------
// Callees, under the names the generated bodies give them.

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


    pub const REPLAY_FIND_SNAPSHOT: Callee =
        Callee { addr: 0x0026_e448, f: ee::s07::Replay_FindSnapshot };

    pub const INSEQ_GET_INSTANCE: Callee =
        Callee { addr: 0x0027_2cc0, f: ee::s07::InSeq_GetInstance };

    pub const INSEQ_INSTANCE_STOP: Callee =
        Callee { addr: 0x0027_4918, f: ee::s07::InSeq_InstanceStop };

    pub const SCRIPT_ENGINE_CLEAR_PROCESS: Callee =
        Callee { addr: 0x0028_2c88, f: ee::s07::ScriptEngine_ClearProcess };

    pub const STREAM_ARRAY_STOP: Callee =
        Callee { addr: 0x002a_ae40, f: ee::s08::StreamArray_Stop };

    pub const C_WORLD_SOUND_RELOCATE_EMITTERS: Callee =
        Callee { addr: 0x002b_6b50, f: ee::s08::cWorldSound_RelocateEmitters };

    pub const USTR_FROM_ASCII_2: Callee =
        Callee { addr: 0x002c_2540, f: ee::s08::UStr_FromAscii_2 };

    pub const WORLD_PAINTER_GET_IRRADIANCE_NAME2: Callee =
        Callee { addr: 0x002e_f198, f: ee::s09::WorldPainter_GetIrradianceName2 };

    pub const IRRADIANCE_REF_GET_ENTRY: Callee =
        Callee { addr: 0x002e_e010, f: ee::s09::IrradianceRef_GetEntry };

    pub const BE_OPTION_INT_GET_MAX_HANDICAP: Callee =
        Callee { addr: 0x0014_f870, f: ee::s01::BEOptionInt_GetMaxHandicap };
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


/// `BEOptionInt_DestroyInstance()` (0x15bd30): as above for `gp - 0x1ee4`.
pub fn be_option_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, OPTION_GP, 0x0015_bd44);
}


// ---------------------------------------------------------------------------
// Options.

/// `BEOptionInt_SetHUDDefaults(self)` (0x14f600): set bits 0x3fff of the HUD
/// flags word at 0x53610, then notify the current profile (`self+0xc` vtable
/// slot +0xc).
pub fn be_option_int_set_hud_defaults(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        c.g[A0][0] = 0x0053_0000;
        c.g[V0][0] = 0x3fff_0000;
        c.g[V1][0] = Addr(0x0053_5610).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(OPTION_GP)).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        frame.save_ra(c, 0);
        Addr(0x0053_5610).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32 + 0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        c.g[RA][0] = 0x0014_f638;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0014_f638)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BEOptionInt_GetQuickMessageKey(self, index)` (0x14f678): format the index
/// into a small stack buffer and look the key up.
pub fn be_option_int_get_quick_message_key(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        c.g[A2][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x0046_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x5940));
        call_ptr(c, 0x0041_7828, 0x0014_f694)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = 0x0014_f69c;
        call_ptr(c, 0x0031_7670, 0x0014_f69c)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `BEOptionInt_GetQuickMessage(self, index)` (0x14f6a8): the quick-message
/// string for `index` at `0x535617 + index*0x40`, or 0 when empty.
pub fn be_option_int_get_quick_message(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5617));
        c.g[S0][0] = sx((c.g[A1][0] as u32) << 6);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x0014_f6cc)?;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Option defaults and quick messages.

/// `BEOptionInt_ctor(self)` (0x14f418): run the base initialiser, then set the
/// HUD, game and sound defaults.
pub fn be_option_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_f648, 0x0014_f42c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f600, 0x0014_f434)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f4f8, 0x0014_f43c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_f458, 0x0014_f444)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BEOptionInt_SetSoundDefaults(self)` (0x14f458): clear and set the sound
/// bits of the option flags word at 0x53610, then notify the profile.
pub fn be_option_int_set_sound_defaults(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 40;
        c.g[A2][0] = 0x0053_0000;
        c.g[V1][0] = 0xffff_ffff_ffff_fff0;
        c.g[V0][0] = Addr(0x0053_5610).lw(c);
        c.g[A0][0] = 0xffff_ffff_ffff_ff0f;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = 0xffff_ffff_ffff_f0ff;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] | 0xa;
        c.g[T1][0] = 0x0053_5610;
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[T2][0] = 0xffff_ffff_ffff_cfff;
        c.g[V0][0] = c.g[V0][0] | 0xa0;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
        c.g[A0][0] = Addr(0x0053_5614).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0xa00;
        c.g[A1][0] = c.g[A1][0] | 0x3fff;
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.g[A3][0] = 0x1_0000;
        c.g[V1][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[T0][0] = 0x2_0000;
        c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
        c.g[V1][0] = 0x4_0000;
        c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
        c.g[A0][0] = c.g[A0][0] | 1;
        Addr(0x0053_5614).sw(c, c.g[A0][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        Addr(0x0053_5610).sw(c, c.g[V0][0]);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(OPTION_GP)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32 + 8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0014_f4ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BEOptionInt_SetGameDefaults(self)` (0x14f4f8): set the game-mode bits of
/// the option flags word from the current mode, then notify the profile.
pub fn be_option_int_set_game_defaults(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x9c).lw(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32 + 0x24).lw(c);
        let eq1 = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if eq1 {
            c.cycles += 6;
            c.g[A0][0] = 0x0053_0000;
            c.g[V1][0] = 0xffff_ffff_ffcf_0000;
            c.g[V0][0] = Addr(0x0053_5610).lw(c);
            c.g[V1][0] = c.g[V1][0] | 0xffff;
            c.g[A1][0] = 0x20_0000;
            apply_game_mask(c);
        } else {
            c.cycles += 2;
            let lt2 = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0053_0000;
            if lt2 {
                c.cycles += 6;
                clear_game_mask(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let eq2 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x0053_0000;
                if eq2 {
                    c.cycles += 5;
                    c.g[A0][0] = 0x0053_0000;
                    c.g[V1][0] = 0xffff_ffff_ffcf_0000;
                    c.g[V0][0] = Addr(0x0053_5610).lw(c);
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A1][0] = 0x10_0000;
                    apply_game_mask(c);
                } else {
                    c.cycles += 2;
                    c.cycles += 6;
                    clear_game_mask(c);
                }
            }
        }
        c.cycles += 28;
        c.g[V1][0] = Addr(c.g[A2][0] as u32 + 0x5610).lw(c);
        c.g[V0][0] = 0xffff_ffff_fe3f_0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A0][0] = 0xffff_ffff_fff7_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[A1][0] = 0xffff_ffff_dfff_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        c.g[V0][0] = 0xffff_ffff_e1ff_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[A0][0] = 0x1200_0000;
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5610));
        Addr(0x0053_5610).sw(c, c.g[V1][0]);
        c.sb(c.g[V0][0] as u32 + 6, 0);
        c.sb(c.g[V0][0] as u32 + 5, 0);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(OPTION_GP)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32 + 8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0014_f5f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The shared "apply the mode bits" tail of `SetGameDefaults`.
pub(crate) fn apply_game_mask(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    Addr(0x0053_5610).sw(c, c.g[V0][0]);
}


/// The "clear the mode bits" tail of `SetGameDefaults` (v0 already holds the
/// flags word for the 0/>2 path, or the base for the inner path).
pub(crate) fn clear_game_mask(c: &mut Cpu) {
    c.g[V1][0] = 0xffff_ffff_ffcf_0000;
    c.g[A0][0] = Addr(0x0053_5610).lw(c);
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
    Addr(0x0053_5610).sw(c, c.g[A0][0]);
}


/// `BEOptionInt_ResetQuickMessage(self, index)` (0x14f6e8): format the index
/// into the controller's key buffer and copy it into the quick-message slot.
pub fn be_option_int_reset_quick_message(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[V0][0] as u32 + 0x8c).lw(c);
        c.g[S1][0] = Addr(c.g[S0][0] as u32 + 4).lw(c);
        c.g[V0][0] = Addr(c.g[S1][0] as u32 + 0x20).lh(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0014_f678, 0x0014_f718)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32 + 0x24).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0014_f728)?;
        c.g[A0][0] = 0x0053_0000;
        c.g[S2][0] = sx((c.g[S2][0] as u32) << 6);
        c.g[A0][0] = sx(0x0053_0000u32.wrapping_add(0x5617));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x003d_cb20, 0x0014_f740)?;
        frame.restore(c, S2, 0x10);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BEOptionInt_ResetQuickMessages(self)` (0x14f758): reset all 10 quick
/// messages.
pub fn be_option_int_reset_quick_messages(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_f6e8, 0x0014_f780)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_f778);
            }
        }
    })
}


// ---------------------------------------------------------------------------
// Option / reward backend accessors.

/// `int BEOptionInt_GetComputerHandicap(void)` (0x14f7e8): the maximum
/// handicap minus the configured computer handicap (`D_005308D4`).
pub fn be_option_int_get_computer_handicap(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_OPTION_INT_GET_MAX_HANDICAP.call(c, 0x0014_f7f8)?;
        c.g[V1][0] = 0x0053_0000;
        frame.restore_ra(c, 0);
        let configured = Addr(0x0053_08d4).lw(c);
        c.g[A0][0] = configured;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(configured as u32));
        frame.close(c);
        Ok(())
    })
}


/// `int BEOptionInt_GetMaxHandicap(void)` (0x14f870): the largest of the three
/// handicap limits in the option state at `0x5308b8`.
pub fn be_option_int_get_max_handicap(c: &mut Cpu) {
    c.cycles += 10;
    let base = 0x0053_08b8u32;
    c.g[V1][0] = sx(base);
    c.g[V0][0] = Addr(base.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = Addr(base.wrapping_add(0x8)).lw(c);
    c.g[A2][0] = Addr(base.wrapping_add(0x14)).lw(c);
    c.g[A0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[A1][0];
    }
    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let ret = c.g[RA][0] as u32;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.pc = ret;
}


/// `void BEOptionInt_ResetSingleton(void)` (0x14f648): reset the option
/// singleton's current page (`D_004A120C -> +0xc` object, slot at `+8`).
pub fn beoption_int_reset_singleton(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr(glo(c, g::BE_OPTION_SINGLETON)).lw(c);
        frame.save_ra(c, 0);
        let opt = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(at(opt, 0xc)).lw(c);
        let page = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(at(page, 8)).lh(c);
        c.g[V1][0] = Addr(at(page, 0xc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(opt.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0014_f668)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `int BEOptionInt_GetPlayerHandicap(int i)` (0x14f7a8): the maximum handicap
/// minus the player's stored handicap, both from the option table at
/// `D_005308B8`.
pub fn be_option_int_get_player_handicap(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V1][0] = 0xc;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x0053_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0053_08b8);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
        callee::BE_OPTION_GET_MAX_HANDICAP.call(c, 0x0014_f7d0)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void* BEOptionInt_Commit()` (0x14F2A8): copy the live option block at
/// 0x535610 over the committed one at 0x535898 (0x280 bytes).
pub fn be_option_int_commit(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x5898));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x5610));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x280));
    copy_block(c, 0x0014_f2c8, 5, 1);
}


/// `void* BEOptionInt_Reset()` (0x14F360): copy the committed option block
/// back over the live one.
pub fn be_option_int_reset(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = sx(ea(c.g[V0][0], 0x5610));
    c.g[V1][0] = sx(ea(c.g[V1][0], 0x5898));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    c.g[V0][0] = sx(ea(c.g[V1][0], 0x280));
    copy_block(c, 0x0014_f380, 5, 1);
}


/// `void BEOptionInt_IsAnyHandicapSet(...)` (0x14f810): true when any of the
/// twelve handicap entries of the option table is set.
pub fn be_option_int_is_any_handicap_set(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0xc;
    c.g[V1][0] = 0x53_0000;
    c.mult(0, 4, 4, 2);
    c.g[A1][0] = sx(0x0053_08b8);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    let mut any = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    if !any {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x4).lw(c);
        any = c.g[V1][0] != 0;
        if !any {
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, V0)).off(0x8).lw(c);
            any = c.g[V1][0] != 0;
            if !any {
                c.cycles += 3;
                c.g[V0][0] = Addr(at_2(c, A1, 0x18)).lw(c);
                any = c.g[V0][0] != 0;
                if !any {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at_2(c, A1, 0x1c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        abi::ret(c);
                        return;
                    }
                    any = true;
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}


/// `void BEOptionInt_SwapPlayerSettings()` (0x14f898): swap the two 12-byte
/// player option triples at `D_005308B8` through a stack temporary.
pub fn be_option_int_swap_player_settings(c: &mut Cpu) {
    c.cycles += 25;
    c.g[V0][0] = 0x0053_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = lw(c, (c.g[A3][0] as u32).wrapping_add(8));
    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[SP][0] as u32);
    sw(c, (c.g[SP][0] as u32).wrapping_add(8), c.g[A1][0]);
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x13));
    c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x14));
    c.sdl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[A3][0] as u32);
    sw(c, (c.g[A3][0] as u32).wrapping_add(8), c.g[A1][0]);
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(7));
    c.ldr(2, c.g[SP][0] as u32);
    c.g[A0][0] = lw(c, (c.g[SP][0] as u32).wrapping_add(8));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x13));
    c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0xc));
    sw(c, (c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check_lohi;
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;


    #[test]
    fn be_option_int_set_hud_defaults_matches() {
        covered(
            ssx3_ee::s01::f_0014f600,
            be_option_int_set_hud_defaults,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf600, 1);
                let opt = ptr(&mut r);
                c.bus.write32(GAME_GP - OPTION_GP, opt);
                let profile = ptr(&mut r);
                c.bus.write32(opt + 0xc, profile);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(profile + 0xc, vt);
                c.bus.write32(0x0053_5610, r.next_u32());
            },
        );
    }


    #[test]
    fn be_option_int_get_quick_message_matches() {
        check(
            ssx3_ee::s01::f_0014f6a8,
            be_option_int_get_quick_message,
            150,
            &|c, seed| {
                let mut r = Rng(0xf6a8_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let idx = seed % 4;
                c.g[A1][0] = idx as u64;
                let p = 0x0053_5617 + idx * 0x40;
                if seed % 3 == 0 {
                    c.bus.write8(p, 0);
                } else {
                    for i in 0..8 {
                        c.bus.write8(p + i, (r.next_u32() | 1) as u8);
                    }
                    c.bus.write8(p + 8, 0);
                }
            },
        );
    }


    #[test]
    fn be_option_int_get_quick_message_key_matches() {
        check(
            ssx3_ee::s01::f_0014f678,
            be_option_int_get_quick_message_key,
            100,
            &|c, seed| {
                let mut r = Rng(0xf678_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    (seed % 100) as u64
                } else {
                    r.next_u32() as u64
                };
                // A tiny format string at the address the original uses.
                for (i, b) in [b'%', b'd', 0, 0].into_iter().enumerate() {
                    c.bus.write8(0x0045_a6c0 + i as u32, b);
                }
            },
        );
    }


    #[test]
    fn be_option_int_ctor_matches() {
        covered(ssx3_ee::s01::f_0014f418, be_option_int_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xf418, 3);
            let world = ptr(&mut r);
            c.bus.write32(GAME_GP - 0x848, world);
            let sub = ptr(&mut r);
            c.bus.write32(world + 0x9c, sub);
            c.bus.write32(sub + 0x24, (seed % 4) as u32);
            let opt = ptr(&mut r);
            c.bus.write32(GAME_GP - OPTION_GP, opt);
            let profile = ptr(&mut r);
            c.bus.write32(opt + 0xc, profile);
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(profile + 0xc, vt);
            c.bus.write32(0x0053_5610, r.next_u32());
            c.bus.write32(0x0053_5614, r.next_u32());
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn be_option_defaults_match() {
        covered(
            ssx3_ee::s01::f_0014f458,
            be_option_int_set_sound_defaults,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf458, 1);
                let opt = ptr(&mut r);
                c.bus.write32(GAME_GP - OPTION_GP, opt);
                let profile = ptr(&mut r);
                c.bus.write32(opt + 0xc, profile);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(profile + 0xc, vt);
                c.bus.write32(0x0053_5610, r.next_u32());
                c.bus.write32(0x0053_5614, r.next_u32());
            },
        );
        covered(
            ssx3_ee::s01::f_0014f4f8,
            be_option_int_set_game_defaults,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf4f8, 1);
                let world = ptr(&mut r);
                c.bus.write32(GAME_GP - 0x848, world);
                let sub = ptr(&mut r);
                c.bus.write32(world + 0x9c, sub);
                c.bus.write32(sub + 0x24, (seed % 4) as u32);
                let opt = ptr(&mut r);
                c.bus.write32(GAME_GP - OPTION_GP, opt);
                let profile = ptr(&mut r);
                c.bus.write32(opt + 0xc, profile);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(profile + 0xc, vt);
                c.bus.write32(0x0053_5610, r.next_u32());
                c.bus.write32(0x0053_5614, r.next_u32());
            },
        );
    }


    #[test]
    fn be_option_quick_messages_match() {
        fn qm_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
            let world = ptr(r);
            c.bus.write32(GAME_GP - 0x848, world);
            let obj = ptr(r);
            c.bus.write32(world + 0x8c, obj);
            let ctrl = ptr(r);
            c.bus.write32(obj + 4, ctrl);
            c.bus.write16(ctrl + 0x20, (r.next_u32() % 4) as u16);
            let vt = vtable(c, r, &[0x24]);
            c.bus.write32(ctrl + 0x24, vt);
            // GetQuickMessageKey's format string.
            for (i, b) in [b'%', b'd', 0, 0].into_iter().enumerate() {
                c.bus.write8(0x0045_a6c0 + i as u32, b);
            }
            let _ = seed;
        }
        check(
            ssx3_ee::s01::f_0014f6e8,
            be_option_int_reset_quick_message,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf6e8, 1);
                qm_setup(c, &mut r, seed);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 10) as u64;
            },
        );
        check(
            ssx3_ee::s01::f_0014f758,
            be_option_int_reset_quick_messages,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf758, 1);
                qm_setup(c, &mut r, seed);
                c.g[A0][0] = DATA as u64;
            },
        );
    }


    // ------------------------------------------------------ option / reward

    #[test]
    fn be_option_int_get_max_handicap_matches() {
        check(ee::s01::f_0014f870__generated, be_option_int_get_max_handicap, 200, &|c, seed| {
            let mut r = Rng(0x14f8_7000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            // Three signed limits; keep the ordering varied so both `max`
            // steps take both outcomes.
            let vals = [
                (r.next_u32() % 7) as i32 - 3,
                (r.next_u32() % 7) as i32 - 3,
                (r.next_u32() % 7) as i32 - 3,
            ];
            for (i, off) in [0x1c, 0x8, 0x14].into_iter().enumerate() {
                c.bus.write32(0x0053_08b8 + off, vals[i] as u32);
            }
        });
    }


    #[test]
    fn be_option_int_get_computer_handicap_matches() {
        check(ee::s01::f_0014f7e8__generated, be_option_int_get_computer_handicap, 120, &|c, seed| {
            let mut r = Rng(0x14f7_e800 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            for (i, off) in [0x1c, 0x8, 0x14, 0x1c].into_iter().enumerate() {
                c.bus.write32(0x0053_08b8 + off, (r.next_u32() % 9) as u32 + i as u32);
            }
            c.bus.write32(0x0053_08d4, r.next_u32() % 9);
        });
    }

    /// A vtable the object points at.
    pub(crate) const VT: u32 = DATA + 0x8000;


    /// A `{ short delta; short; void (*fn)(); }` slot pointing at [`FAKE`].
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, FAKE);
    }


    #[test]
    fn beoption_int_reset_singleton_matches() {
        covered(s01::f_0014f648__generated, beoption_int_reset_singleton, 120, &|c, seed| {
            machine(c, seed, 0x14f648, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x40);
            clear(c, CHILD, 0x40);
            slot(c, VT, 0x8);
            // D_004A120C -> current page object (+0xc) -> vtable.
            c.bus.write32(GAME_GP.wrapping_sub(0x1ee4), OBJ);
            c.bus.write32(OBJ + 0xc, CHILD);
            c.bus.write32(CHILD, VT);
            let _ = seed;
        });
    }


    #[test]
    fn be_option_int_get_player_handicap_matches() {
        check(ssx3_ee::s01::f_0014f7a8__generated, be_option_int_get_player_handicap, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x14f7_a8, 0);
            let table = 0x0053_08b8u32;
            let i = r.next_u32() % 4;
            c.g[A0][0] = i as u64;
            for k in 0..4u32 {
                c.bus.write32(table + k * 0xc + 8, r.next_u32() % 20);
            }
            c.bus.write32(table + 0x1c, r.next_u32() % 20);
            c.bus.write32(table + 0x8, r.next_u32() % 20);
            c.bus.write32(table + 0x14, r.next_u32() % 20);
        });
    }


    #[test]
    fn be_option_int_commit_matches() {
        check(ee::s01::f_0014f2a8__generated, be_option_int_commit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0014_f2a8, 0);
            belib(c);
            for i in 0..0xa0u32 {
                w(c, 0x0053_5610 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn be_option_int_reset_matches() {
        check(ee::s01::f_0014f360__generated, be_option_int_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0014_f360, 0);
            belib(c);
            for i in 0..0xa0u32 {
                w(c, 0x0053_5898 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn be_option_int_is_any_handicap_set_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014f810__generated,
            be_option_int_is_any_handicap_set,
            100,
            &|c, seed| {
                fakes::machine(c, seed, 0x14f810, 0);
                let index = (seed % 4) as u32;
                for i in 0..4u32 {
                    let set = i <= index && (seed >> (i + 2)) & 1 != 0;
                    w_2(c, 0x0053_08b8 + 12 * i, set as u32);
                }
                w_2(c, 0x0053_08b8 + 0x18, (seed >> 1) & 1);
                w_2(c, 0x0053_08b8 + 0x1c, seed & 1);
                c.g[A0][0] = index as u64;
            },
        );
    }


    // ------------------------------------------------------------- option swap

    #[test]
    fn be_option_int_swap_player_settings_matches() {
        check(
            ssx3_ee::s01::f_0014f898__generated,
            be_option_int_swap_player_settings,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x14f898);
                for i in 0..8u32 {
                    c.bus.write32(0x0053_08b8 + 4 * i, r.next_u32());
                }
                let _ = seed;
            },
        );
    }
}
