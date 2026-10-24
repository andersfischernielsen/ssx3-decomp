//! `src/visualfx/angerfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::ai::rider::per_tick_60;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::tick;
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
    (0x002d_4be0, c_anger_fx_reset, "cAngerFX_Reset"),
    (0x002d_4c08, anger_fx_update, "cAngerFX_Update"),
    (0x002d_5598, avalanche_pack_bytes3, "Avalanche_PackBytes3"),
    (
        0x002d_55c0,
        avalanche_pack_clamped255x3,
        "Avalanche_PackClamped255x3",
    ),
    (
        0x002d_5658,
        avalanche_pack_snorm3_alpha,
        "Avalanche_PackSnorm3Alpha",
    ),
    (
        0x002d_56b0,
        visual_effects_man_emitter_holder_dtor,
        "VisualEffectsMan_EmitterHolderDtor",
    ),
    (
        0x002d_5718,
        avalanche_compare_float,
        "Avalanche_CompareFloat",
    ),
    (0x002d_5750, avalanche_compare_u16, "Avalanche_CompareU16"),
    (0x0037_15b0, memitter_free_data, "mEmitter_FreeData"),
];

// ---------------------------------------------------------------------------
// Anger FX.

/// `cAngerFX` fields.
pub(crate) mod anger {

    /// The rider.
    pub const RIDER: u32 = 0x00;

    /// Per-rider slots, 0x2c bytes each.
    pub const SLOTS: u32 = 0x04;

    pub const SLOT_STRIDE: u32 = 0x2c;
}

/// One rider's anger slot.
pub(crate) mod anger_slot {

    /// Glow level (approaches 1 while angry, 0 otherwise).
    pub const LEVEL: u32 = 0x00;

    /// RGBA colour (written as a 16-byte quad).
    pub const COLOUR: u32 = 0x04;

    /// Two glow intensities looked up from the animation tables.
    pub const GLOW_A: u32 = 0x14;

    pub const GLOW_B: u32 = 0x18;

    /// Last computed anger level (0..4+).
    pub const ANGER: u32 = 0x1c;

    /// Animation counters (mod 41, 31 and 24).
    pub const T20: u32 = 0x20;

    pub const T24: u32 = 0x24;

    pub const T28: u32 = 0x28;
}

/// Rider fields used by the anger update.
pub(crate) mod anger_rider {

    pub const VTABLE: u32 = 0x6c0;

    /// Set while the rider is in a state that hides the anger FX.
    pub const SPECIAL: u32 = 0xac4;

    /// The "other rider" link.
    pub const OTHER: u32 = 0xa8;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
}

// ---------------------------------------------------------------------------
// Avalanche compression.

/// `void Avalanche_PackBytes3(uchar* dst, float a, float b, float c)`
/// (0x2d5598): truncate three floats to int and pack the low bytes.
pub fn avalanche_pack_bytes3(c: &mut Cpu) {
    c.cycles += 10;
    let x = FpuF32(c.f[12]).to_i32();
    c.f[0] = x as u32;
    c.g[A1][0] = x as i64 as u64;
    let y = FpuF32(c.f[13]).to_i32();
    c.f[0] = y as u32;
    c.g[V0][0] = y as i64 as u64;
    let z = FpuF32(c.f[14]).to_i32();
    c.f[0] = z as u32;
    c.g[V1][0] = z as i64 as u64;
    let dst = Addr(abi::arg(c, 0));
    dst.sb(c, c.g[A1][0]);
    dst.off(0x1).sb(c, c.g[V0][0]);
    dst.off(0x2).sb(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Avalanche packing / fade.

/// `void Avalanche_PackClamped255x3(u8 *out, float x, float y, float z)`
/// (0x2d55c0): scale the three inputs by the global at `gp+0x948` (255.0),
/// clamp each to `[0, 1]` and convert to bytes at `out+3..6`.
pub fn avalanche_pack_clamped255x3(c: &mut Cpu) {
    c.cycles += 12;
    let out = abi::arg(c, 0);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x948));
    c.g[A1][0] = c.g[A0][0];
    c.mtc1(ZERO, 1); // f1 = 0.0
    c.mul_s(12, 12, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // f3 = 1.0
    c.mul_s(14, 14, 0);
    c.mov_s(2, 1);
    c.c_le_s(1, 12);
    let neg_x = !c.fcc();
    c.mul_s(13, 13, 0);
    if !neg_x {
        c.cycles += 1;
        c.min_s(2, 12, 3);
    }
    c.cycles += 4;
    c.c_le_s(1, 13);
    let neg_y = !c.fcc();
    c.mov_s(4, 1);
    if !neg_y {
        c.cycles += 1;
        c.min_s(4, 13, 3);
    }
    c.cycles += 4;
    c.c_le_s(1, 14);
    if c.fcc() {
        c.min_s(1, 14, 3);
    }
    c.cycles += 15;
    c.g[AT][0] = 0x437f_0000; // 255.0
    c.mtc1(AT, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(2, 2, 0);
    c.mul_s(0, 4, 0);
    c.cvt_w_s(3, 1);
    c.mfc1(V0, 3);
    c.cvt_w_s(1, 2);
    c.mfc1(V1, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(A0, 1);
    Addr(out).off(0x5).sb(c, c.g[V0][0]);
    Addr(out).off(0x3).sb(c, c.g[V1][0]);
    Addr(out).off(0x4).sb(c, c.g[A0][0]);
    abi::ret(c);
}

/// `void Avalanche_PackSnorm3Alpha(u8 *out, float x, float y, float z,
/// float a)` (0x2d5658): scale the x/y/z by 127 and the alpha by 255, convert
/// to integers and store at `out+6..10`.
pub fn avalanche_pack_snorm3_alpha(c: &mut Cpu) {
    c.cycles += 21;
    let out = abi::arg(c, 0);
    c.g[AT][0] = 0x42fe_0000; // 127.0
    c.mtc1(AT, 0);
    c.g[AT][0] = 0x437f_0000; // 255.0
    c.mtc1(AT, 1);
    c.mul_s(14, 14, 0);
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.cvt_w_s(0, 14);
    c.mfc1(V0, 0);
    c.mul_s(15, 15, 1);
    c.cvt_w_s(0, 12);
    c.mfc1(V1, 0);
    c.cvt_w_s(0, 13);
    c.mfc1(A1, 0);
    Addr(out).off(0x8).sb(c, c.g[V0][0]);
    c.cvt_w_s(0, 15);
    c.mfc1(A2, 0);
    Addr(out).off(0x6).sb(c, c.g[V1][0]);
    Addr(out).off(0x7).sb(c, c.g[A1][0]);
    Addr(out).off(0x9).sb(c, c.g[A2][0]);
    abi::ret(c);
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// The four colour ramps (20 bytes each): level 4, 3, 2, 1.
pub(crate) const ANGER_COLOURS: [u32; 4] = [0x004d_5b30, 0x004d_5b40, 0x004d_5b50, 0x004d_5b60];

/// `ldl/ldr` two quadwords from `src` into `dst` (a 20-byte record copy).
#[inline(always)]
pub(crate) fn copy_record20(c: &mut Cpu, src: u32, dst: u32) {
    c.ldl(4, p(src, 7));
    c.ldr(4, src);
    c.ldl(7, p(src, 0xf));
    c.ldr(7, p(src, 8));
    c.sdl(4, p(dst, 7));
    c.sdr(4, dst);
    c.sdl(7, p(dst, 0xf));
    c.sdr(7, p(dst, 8));
}

/// `void cAngerFX_Update(AngerFX* self)` (0x2d4c08): for every rider, look up
/// how much the AI/rival aggression targets this rider and fade the anger glow
/// in (level 2+) or out, with the animation counters that drive the colour
/// pulse and the two glow tables.
pub fn anger_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S4, 0x50);
        frame.save(c, S0, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let self_at = r32(c, S4);
        let sp0 = sp(c);
        // riderV(self->rider, 8) != 0 hides the FX.
        c.g[A0][0] = c.lw(self_at);
        c.g[V1][0] = c.lw(p(r32(c, A0), anger_rider::VTABLE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(anger_rider::VTABLE));
        c.g[A1][0] = c.lh(p(r32(c, V1), 0x40));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x44));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, target, 0x002d_4c54)?;
        let hidden = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x90);
        'out: {
            if hidden {
                break 'out;
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw(gp(c, -0x848));
            c.g[A0][0] = c.lw(p(r32(c, V1), 0x84));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw(p(r32(c, A0), 0x84));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x10));
            let no_players = c.g[V1][0] == 0;
            c.g[S3][0] = 0;
            if no_players {
                break 'out;
            }
            c.cycles += 10;
            c.g[V0][0] = 0x0049_0000;
            c.g[V1][0] = 0x0049_0000;
            c.g[FP][0] = sx(r32(c, V0).wrapping_sub(0x6f10));
            c.g[S7][0] = sx(r32(c, V1).wrapping_sub(0x70b8));
            c.g[V0][0] = 0x0049_0000;
            c.g[V1][0] = 0x0049_0000;
            c.g[S6][0] = sx(r32(c, V0).wrapping_sub(0x6f90));
            c.g[S5][0] = sx(r32(c, V1).wrapping_sub(0x6eb0));
            c.g[A2][0] = c.lw(p(r32(c, A1), 0x84));
            'players: loop {
                'player: {
                    c.cycles += 9;
                    c.g[A0][0] = shl32(c.g[S3][0], 2);
                    c.g[V0][0] = c.lw(p(r32(c, A2), 0x84));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(p(r32(c, V0), 4));
                    c.g[A0][0] = c.lw(p(r32(c, V1), anger_rider::OTHER));
                    c.g[V0][0] = c.lw(p(r32(c, A0), 0x20));
                    c.g[S0][0] = c.lw(p(r32(c, V0), 4));
                    if c.g[S0][0] == 0 {
                        break 'player;
                    }
                    c.g[V0][0] = sx(anger::SLOT_STRIDE);
                    'slot: {
                        'zero: {
                            c.cycles += 6;
                            c.g[A1][0] = c.lw(self_at);
                            c.mult(0, 2, 19, 2); // slot offset = i * 0x2c
                            c.g[V1][0] = c.lw(p(self_at, anger::SLOTS));
                            c.g[A0][0] = c.lw(p(r32(c, A1), anger_rider::SPECIAL));
                            let special = c.g[A0][0] != 0;
                            c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                            if special {
                                break 'zero;
                            }
                            // n = **(world->+0x84 + 0x28); 0 < n < 10 hides it.
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(p(r32(c, A2), 0x28));
                            c.g[V0][0] = c.lw(r32(c, V0));
                            let none = c.g[V0][0] == 0;
                            c.g[V1][0] = 0;
                            if !none {
                                c.cycles += 1;
                                c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] == 0 {
                                break 'slot;
                            }
                        }
                        c.cycles += 2;
                        c.sw(r32(c, S2), 0);
                        c.cycles += 1;
                        frame.restore(c, S0, 0x90);
                        break 'players;
                    }
                    // Aggression level of this rider against `other`.
                    c.cycles += 26;
                    call_ptr(c, 0x0015_5a50, 0x002d_4d08)?;
                    c.g[A0][0] = c.lw(self_at);
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(p(r32(c, A0), anger_rider::VTABLE));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(anger_rider::VTABLE));
                    c.g[A1][0] = c.lh(p(r32(c, V1), 0x38));
                    c.g[V0][0] = c.lw(p(r32(c, V1), 0x3c));
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                    call_ptr(c, target, 0x002d_4d28)?;
                    c.g[V1][0] = c.lw(p(r32(c, S0), anger_rider::VTABLE));
                    c.g[A1][0] = sx(r32(c, S0).wrapping_add(anger_rider::VTABLE));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lh(p(r32(c, V1), 0x38));
                    c.g[V0][0] = c.lw(p(r32(c, V1), 0x3c));
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x002d_4d44)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0015_5b50, 0x002d_4d54)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw(p(r32(c, S2), anger_slot::ANGER));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 4) as u64;
                    let below4 = c.g[V0][0] != 0;
                    c.sw(p(r32(c, S2), anger_slot::ANGER), c.g[A1][0]);
                    'colour: {
                        'lev: {
                            if below4 {
                                break 'lev;
                            }
                            c.cycles += 12;
                            c.g[V0][0] = 0x004d_0000;
                            copy_record20(c, ANGER_COLOURS[0], p(r32(c, S2), anger_slot::COLOUR));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < 2) as u64;
                            break 'colour;
                        }
                        'three: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
                            let under3 = c.g[V0][0] != 0;
                            c.g[V1][0] = ((c.g[A1][0] as i64) < 2) as u64;
                            if under3 {
                                break 'three;
                            }
                            c.cycles += 12;
                            c.g[V0][0] = 0x004d_0000;
                            copy_record20(c, ANGER_COLOURS[1], p(r32(c, S2), anger_slot::COLOUR));
                            break 'colour;
                        }
                        'four: {
                            c.cycles += 2;
                            let under2 = c.g[V1][0] != 0;
                            c.g[V0][0] = 0x004d_0000;
                            if !under2 {
                                c.cycles += 12;
                                c.g[V0][0] = 0x004d_0000;
                                copy_record20(
                                    c,
                                    ANGER_COLOURS[2],
                                    p(r32(c, S2), anger_slot::COLOUR),
                                );
                                break 'colour;
                            }
                        }
                        c.cycles += 9;
                        c.g[T1][0] = sx(r32(c, V0).wrapping_add(0x5b60));
                        copy_record20(c, r32(c, T1), p(r32(c, S2), anger_slot::COLOUR));
                        break 'colour;
                    }
                    // approach(level, 1, 0.1) for angry, (level, 0, 0.0666)
                    // otherwise.
                    'approach: {
                        'target: {
                            'low: {
                                c.cycles += 2;
                                let not_angry = c.g[V1][0] != 0;
                                c.lwc1(2, r32(c, S2));
                                if not_angry {
                                    break 'low;
                                }
                                c.cycles += 6;
                                c.lwc1(0, gp(c, -0x3d38)); // 1.1
                                c.lwc1(3, gp(c, -0x3d34)); // 0.1
                                if tick::rate(c) != 60 {
                                    // The fade-in step per tick is 0.1 * 60/rate
                                    // and the snap band 1 +/- that step (0.9..1.1
                                    // at 60), so the glow takes the same time.
                                    per_tick_60(c, 3);
                                    c.f[0] = (FpuF32::from_f32(1.0) + FpuF32(c.f[3])).0;
                                }
                                c.c_lt_s(0, 2);
                                if c.fcc() {
                                    c.sub_s(1, 2, 3);
                                    break 'approach;
                                }
                                c.cycles += 5;
                                c.lwc1(0, gp(c, -0x3d30)); // 0.9
                                if tick::rate(c) != 60 {
                                    c.f[0] = (FpuF32::from_f32(1.0) - FpuF32(c.f[3])).0;
                                }
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 1);
                                break 'target;
                            }
                            'rise: {
                                c.cycles += 4;
                                c.lwc1(3, gp(c, -0x3d2c)); // 0.0666
                                                           // The fade-out step: 1/15 * 60/rate per tick.
                                per_tick_60(c, 3);
                                c.c_lt_s(3, 2);
                                let t = !c.fcc();
                                if !t {
                                    c.cycles += 2;
                                    c.sub_s(1, 2, 3);
                                    break 'approach;
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, gp(c, -0x3d28)); // -0.0666
                            if tick::rate(c) != 60 {
                                // -step (the snap band below 0).
                                c.f[0] = c.f[3] ^ 0x8000_0000;
                            }
                            c.mtc1(0, 1); // f1 = 0
                            break 'target;
                        }
                        c.cycles += 4;
                        c.c_lt_s(2, 0);
                        if c.fcc() {
                            c.add_s(1, 2, 3);
                        }
                        break 'approach;
                    }
                    // Fade in the glow, then advance the three counters. The
                    // pulse counters are frames of 60 Hz colour tables: above
                    // 60 Hz a running pulse advances once per 60 Hz frame of
                    // this slot's own clock (always at 60); the starts and
                    // restarts (level changes) still happen on the tick.
                    let slot_at = r32(c, S2);
                    let due = tick::frame60_due(c, slot_at, 0);
                    't20_block: {
                        't20: {
                            c.cycles += 7;
                            c.swc1(1, r32(c, S2));
                            c.lwc1(0, r32(c, S2));
                            c.lwc1(1, gp(c, -0x3d24)); // 0.8
                            c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                            c.mul_s(0, 0, 1);
                            let running = (c.g[V0][0] as i64) > 0;
                            c.swc1(0, p(r32(c, S2), anger_slot::COLOUR));
                            if !running {
                                c.cycles += 2;
                                if c.g[A2][0] == c.g[A1][0] {
                                    c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                                    break 't20_block;
                                }
                            } else if !due {
                                c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                                break 't20_block;
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                        c.g[V1][0] = 0x29;
                        c.div(0, 2, 3);
                        c.sw(p(r32(c, S2), anger_slot::T20), c.g[V0][0]);
                        c.cycles += 3;
                        c.g[A0][0] = c.hi[0];
                        c.sw(p(r32(c, S2), anger_slot::T20), c.g[A0][0]);
                        c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                        break 't20_block;
                    }
                    't24_block: {
                        't24: {
                            c.cycles += 2;
                            let running = (c.g[V1][0] as i64) > 0;
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                            if running {
                                if !due {
                                    c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                                    break 't24_block;
                                }
                                break 't24;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 4) as u64;
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                                break 't24_block;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                                break 't24_block;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                        }
                        c.cycles += 5;
                        c.g[V1][0] = 0x1f;
                        c.sw(p(r32(c, S2), anger_slot::T24), c.g[V0][0]);
                        c.div(0, 2, 3);
                        c.cycles += 3;
                        c.g[A0][0] = c.hi[0];
                        c.sw(p(r32(c, S2), anger_slot::T24), c.g[A0][0]);
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                        break 't24_block;
                    }
                    't28_block: {
                        't28: {
                            c.cycles += 2;
                            let running = (c.g[V0][0] as i64) > 0;
                            c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            if !running {
                                c.cycles += 2;
                                if c.g[V1][0] == 0 {
                                    c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                                    break 't28_block;
                                }
                            } else if !due && c.g[V1][0] == 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                                break 't28_block;
                            }
                        }
                        c.cycles += 2;
                        if c.g[V1][0] != 0 {
                            c.sw(p(r32(c, S2), anger_slot::T28), 0);
                        }
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                        c.g[V1][0] = 0x18;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                        c.div(0, 2, 3);
                        c.sw(p(r32(c, S2), anger_slot::T28), c.g[V0][0]);
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                        c.g[A0][0] = c.hi[0];
                        let running = (c.g[V0][0] as i64) > 0;
                        c.sw(p(r32(c, S2), anger_slot::T28), c.g[A0][0]);
                        if !running {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                            if (c.g[V0][0] as i64) <= 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                                break 't28_block;
                            }
                        }
                        c.cycles += 2;
                        c.sw(p(r32(c, S2), anger_slot::T28), 0);
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                        break 't28_block;
                    }
                    // glow = level * A[t20] * C[t24] * E[t28]  (and the B/D/F
                    // tables for the second channel).
                    c.cycles += 30;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x7160));
                    c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                    c.g[V0][0] = shl32(c.g[V0][0], 2);
                    c.lwc1(3, r32(c, S2));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S7)));
                    c.g[V1][0] = shl32(c.g[V1][0], 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.lwc1(0, r32(c, A0));
                    c.lwc1(2, r32(c, V0));
                    c.g[A3][0] = 0x0049_0000;
                    c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, S6)));
                    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x7010));
                    c.mul_s(2, 3, 2);
                    c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
                    c.mul_s(3, 3, 0);
                    c.lwc1(0, r32(c, V1));
                    c.g[V0][0] = shl32(c.g[V0][0], 2);
                    c.lwc1(1, r32(c, A1));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                    c.mul_s(2, 2, 0);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, FP)));
                    c.mul_s(3, 3, 1);
                    c.lwc1(0, r32(c, V0));
                    c.lwc1(1, r32(c, V1));
                    c.mul_s(2, 2, 0);
                    c.mul_s(3, 3, 1);
                    c.swc1(2, p(r32(c, S2), anger_slot::GLOW_A));
                    c.swc1(3, p(r32(c, S2), anger_slot::GLOW_B));
                    break 'player;
                }
                // Next player.
                c.cycles += 9;
                c.g[V0][0] = c.lw(gp(c, -0x848));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[A0][0] = c.lw(p(r32(c, V0), 0x84));
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x84));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x10));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x90);
                    break 'players;
                }
                c.g[A2][0] = c.lw(p(r32(c, A1), 0x84));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_4ca0);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = sp0;
        Ok(())
    });
}

/// `void cAngerFX_Reset(cAngerFX *this)` (0x2d4be0): zero the 0x58-byte state
/// object at `this->state` (`+0x4`).
pub fn c_anger_fx_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x58;
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        call_ptr(c, table_callee::EA_MEMSET, 0x002d_4bf8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void mEmitter_FreeData(mEmitter* self, int flags)` (0x3715b0): free the
/// owned point list at +0x204 and pass the flags to the base class. 8 (+3)
/// + 8 cycles plus the callees.
pub fn memitter_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let owned = c.lw(p(self_, 0x204));
        c.g[A0][0] = owned;
        c.g[S1][0] = c.g[A1][0];
        if owned != 0 {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0037_15d8)?;
            c.sw(p(self_, 0x204), 0);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::DYNAMIC_EMITTER_FREE_DATA.call(c, 0x0037_15e8)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Avalanche_CompareFloat(float* a, float* b)` (0x2D5718): -1/0/1.
pub fn avalanche_compare_float(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(1, c.g[A0][0] as u32);
    c.lwc1(0, c.g[A1][0] as u32);
    c.c_lt_s(1, 0);
    if c.fcc() {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.cycles += 4;
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int Avalanche_CompareU16(u16* a, u16* b)` (0x2D5750): -1/0/1.
pub fn avalanche_compare_u16(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V1][0] = c.lhu(c.g[A1][0] as u32);
    c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
    let not_less = c.g[V0][0] == 0;
    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
    if not_less {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
    }
}

/// `VisualEffectsMan_EmitterHolderDtor(void* self, int flags)` (0x2d56b0):
/// free the holder's data and destroy the emitter, then delete the holder
/// when the low flag bit is set.
pub fn visual_effects_man_emitter_holder_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save(c, S1, 0x20);
        let flags = c.g[A1][0];
        c.g[S0][0] = flags;
        c.g[S1][0] = sx((this as u32).wrapping_add(0xd0));
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0037_15b0, 0x002d_56dc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0037_14f8, 0x002d_56e8)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x002d_56fc)?;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::object::animnode::tests::fb;
    pub(crate) use crate::game::object::railmodifier::tests::h;
    pub(crate) use crate::game::render::bezierman::tests::b;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::gp_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_2 as SEEDS;

    #[test]
    fn avalanche_pack_bytes3_matches() {
        check(
            ssx3_ee::s09::f_002d5598,
            avalanche_pack_bytes3,
            300,
            &|c, seed| {
                let mut r = Rng(0x5598 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + (seed % 8)) as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
            },
        );
    }

    #[test]
    fn avalanche_pack_clamped255x3_matches() {
        check(
            ssx3_ee::s09::f_002d55c0,
            avalanche_pack_clamped255x3,
            250,
            &|c, seed| {
                let mut r = Rng(0x1001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP + 0x948, r.edge_f32());
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                if seed % 3 == 0 {
                    c.f[12] |= 0x8000_0000;
                }
                if seed % 5 == 0 {
                    c.f[13] |= 0x8000_0000;
                }
                if seed % 7 == 0 {
                    c.f[14] |= 0x8000_0000;
                }
            },
        );
    }

    #[test]
    fn avalanche_pack_snorm3_alpha_matches() {
        check(
            ssx3_ee::s09::f_002d5658,
            avalanche_pack_snorm3_alpha,
            200,
            &|c, seed| {
                let mut r = Rng(0x1002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..4 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }

    /// The rider virtual's three queries (riderv(.., 8) and twice (.., 7)),
    /// answered by address so both machines take the same path.
    pub(crate) fn anger_rider_v(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = match target {
            0x0000_5000 => 0, // riderV(rider, 8): not hidden
            0x0000_5010 => 1, // riderV(rider, 7): rider id
            0x0000_5020 => 2, // riderV(other, 7): rival id
            _ => 0,
        };
        c.g[V1][0] = 0;
        c.pc = ra;
    }

    #[test]
    fn anger_fx_update_matches() {
        check(
            ee::s09::f_002d4c08__generated,
            anger_fx_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d4c, 3);
                c.hooks.miss = anger_rider_v;
                let (fx, rider, other, vt, vt2, world, g, players, aggr) = (
                    DATA,
                    DATA + 0x2000,
                    DATA + 0x2400,
                    DATA + 0x3000,
                    DATA + 0x3100,
                    DATA + 0x4000,
                    DATA + 0x4200,
                    DATA + 0x4400,
                    DATA + 0x4800,
                );
                let count = 1 + r.next_u32() % 3;
                c.bus.write32(fx, rider);
                for i in 0..count {
                    let slot = fx + 4 + 0x2c * i;
                    c.bus.write32(slot + anger_slot::LEVEL, r.edge_f32());
                    c.bus.write32(slot + anger_slot::ANGER, r.next_u32() % 6);
                    c.bus.write32(slot + anger_slot::T20, r.next_u32() % 45);
                    c.bus.write32(slot + anger_slot::T24, r.next_u32() % 35);
                    c.bus.write32(slot + anger_slot::T28, r.next_u32() % 26);
                    for k in 0..4u32 {
                        c.bus
                            .write32(slot + anger_slot::COLOUR + 4 * k, r.next_u32());
                    }
                }
                // The rider's vtable: riderV(.., 8) and riderV(.., 7).
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, 0x0000_5000);
                c.bus.write16(vt + 0x38, 0);
                c.bus.write32(vt + 0x3c, 0x0000_5010);
                c.bus.write32(other + 0x6c0, vt2);
                c.bus.write16(vt2 + 0x38, 0);
                c.bus.write32(vt2 + 0x3c, 0x0000_5020);
                c.bus.write32(rider + 0x2ec, r.edge_f32());
                c.bus.write32(
                    rider + anger_rider::SPECIAL,
                    if seed % 11 == 0 { 1 } else { 0 },
                );
                // world -> g -> players -> p[i] -> +0xa8 -> +0x20 -> +4 = other
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, count);
                for i in 0..count {
                    let rec = DATA + 0x5000 + 0x40 * i;
                    let node = DATA + 0x6000 + 0x40 * i;
                    c.bus.write32(players + 4 + 4 * i, rec);
                    c.bus.write32(rec + 0xa8, node);
                    c.bus.write32(node + 0x20, DATA + 0x7000 + 0x40 * i);
                    c.bus.write32(DATA + 0x7000 + 0x40 * i + 4, other);
                }
                let nptr = DATA + 0x7800;
                c.bus.write32(g + 0x28, nptr);
                c.bus
                    .write32(nptr, if seed % 13 == 0 { 0 } else { r.next_u32() % 20 });
                // Pre-cached singletons: the aggression interface and the race.
                c.bus.write32(gp_at(-0x1ea4), aggr);
                c.bus.write32(gp_at(-0x1f3c), DATA + 0x4900);
                c.bus.write32(gp_at(-0x1f04), DATA + 0x4a00);
                c.g[A0][0] = sx(fx);
            },
        );
    }

    /// One anger slot after `ticks` updates at `hz`: (glow level, the three
    /// pulse counters). The slot lives at its own address per run (the
    /// per-slot 60 Hz clock is keyed by it).
    fn anger_after(hz: u32, ticks: u32, slots: u32) -> (f32, u32, u32, u32) {
        let mut c = crate::difftest::fresh();
        let _r = machine(&mut c, 1, 0x2d4c, 0);
        c.hooks.miss = anger_rider_v;
        crate::tick::set_for_test(&mut c, hz);
        let (fx, rider, other, vt, vt2, world, g, players, aggr) = (
            DATA,
            DATA + 0x2000,
            DATA + 0x2400,
            DATA + 0x3000,
            DATA + 0x3100,
            DATA + 0x4000,
            DATA + 0x4200,
            DATA + 0x4400,
            DATA + 0x4800,
        );
        // The ramp constants (1.1, 0.1, 0.9, 1/15, -1/15, 0.8).
        for (off, v) in [
            (-0x3d38, 1.1f32),
            (-0x3d34, 0.1),
            (-0x3d30, 0.9),
            (-0x3d2c, 1.0 / 15.0),
            (-0x3d28, -1.0 / 15.0),
            (-0x3d24, 0.8),
        ] {
            c.bus.write32(gp_at(off), v.to_bits());
        }
        c.bus.write32(fx, rider);
        c.bus.write32(fx + anger::SLOTS, slots);
        c.bus.write32(slots + anger_slot::LEVEL, 0.3f32.to_bits());
        // A level change on the first tick, all three pulses running.
        c.bus.write32(slots + anger_slot::ANGER, 99);
        c.bus.write32(slots + anger_slot::T20, 1);
        c.bus.write32(slots + anger_slot::T24, 1);
        c.bus.write32(slots + anger_slot::T28, 1);
        c.bus.write32(rider + 0x6c0, vt);
        c.bus.write16(vt + 0x40, 0);
        c.bus.write32(vt + 0x44, 0x0000_5000);
        c.bus.write16(vt + 0x38, 0);
        c.bus.write32(vt + 0x3c, 0x0000_5010);
        c.bus.write32(other + 0x6c0, vt2);
        c.bus.write16(vt2 + 0x38, 0);
        c.bus.write32(vt2 + 0x3c, 0x0000_5020);
        c.bus.write32(rider + anger_rider::SPECIAL, 0);
        c.bus.write32(gp_at(-0x848), world);
        c.bus.write32(world + 0x84, g);
        c.bus.write32(g + 0x84, players);
        c.bus.write32(players + 0x10, 1);
        let (rec, node) = (DATA + 0x5000, DATA + 0x6000);
        c.bus.write32(players + 4, rec);
        c.bus.write32(rec + 0xa8, node);
        c.bus.write32(node + 0x20, DATA + 0x7000);
        c.bus.write32(DATA + 0x7000 + 4, other);
        let nptr = DATA + 0x7800;
        c.bus.write32(g + 0x28, nptr);
        c.bus.write32(nptr, 0);
        c.bus.write32(gp_at(-0x1ea4), aggr);
        c.bus.write32(gp_at(-0x1f3c), DATA + 0x4900);
        c.bus.write32(gp_at(-0x1f04), DATA + 0x4a00);
        for _ in 0..ticks {
            c.g[A0][0] = sx(fx);
            anger_fx_update(&mut c);
        }
        (
            f32::from_bits(c.bus.read32(slots + anger_slot::LEVEL)),
            c.bus.read32(slots + anger_slot::T20),
            c.bus.read32(slots + anger_slot::T24),
            c.bus.read32(slots + anger_slot::T28),
        )
    }

    /// The glow ramp and the pulse counters take the same wall-clock time at
    /// 120 and 240 Hz as at 60.
    #[test]
    fn anger_fx_update_follows_the_rate() {
        for (i, frames) in [2u32, 5, 12].into_iter().enumerate() {
            let at60 = anger_after(60, frames, DATA + 0x8000 + 0x100 * i as u32);
            // At 60 the glow level moved and the pulses ran.
            assert_eq!(at60.1, (1 + frames) % 41, "{frames}: {at60:?}");
            for (k, hz) in [120u32, 240].into_iter().enumerate() {
                let slots = DATA + 0x9000 + 0x100 * (4 * i + k) as u32;
                let at = anger_after(hz, frames * hz / 60, slots);
                assert_eq!(
                    (at.1, at.2, at.3),
                    (at60.1, at60.2, at60.3),
                    "{hz} Hz, {frames}: {at:?} vs {at60:?}"
                );
                assert!(
                    (at.0 - at60.0).abs() < 1e-4,
                    "{hz} Hz, {frames}: level {} vs {}",
                    at.0,
                    at60.0
                );
            }
        }
    }

    #[test]
    fn c_anger_fx_reset_matches() {
        check(
            ee::s09::f_002d4be0__generated,
            c_anger_fx_reset,
            60,
            &|c, seed| {
                let _r = game(c, seed, 0x2d4be0, 1);
                c.g[A0][0] = SCRATCH as u64;
                c.bus
                    .write32(SCRATCH + 4, SCRATCH + 0xb00 + 0x80 * (seed % 4));
            },
        );
    }

    #[test]
    fn memitter_free_data_matches() {
        check(
            ee::s12::f_003715b0__generated,
            memitter_free_data,
            60,
            &|c, seed| {
                base(c, seed, 0x3715);
                let obj = DATA;
                let owned = if seed % 3 == 0 {
                    0
                } else {
                    0x40 + 8 * (seed % 16)
                };
                c.bus.write32(obj + 0x204, owned);
                c.bus.write32(obj + 0x1a0, 0);
                c.bus.write32(obj + 0x1a4, 0);
                c.bus.write32(obj + 0x170, 0xffff_ffff);
                c.bus.write32(obj + 0x178, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed / 2 % 2) as u64;
            },
        );
    }

    #[test]
    fn avalanche_compare_float_matches() {
        check(
            ee::s09::f_002d5718__generated,
            avalanche_compare_float,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_5718, 0);
                belib(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x108) as u64;
                let a = match seed % 3 {
                    0 => fb(1.5),
                    1 => fb(-2.25),
                    _ => r.edge_f32(),
                };
                let b = if seed % 4 == 0 { a } else { r.edge_f32() };
                w(c, DATA + 0x100, a);
                w(c, DATA + 0x108, b);
            },
        );
    }

    #[test]
    fn avalanche_compare_u16_matches() {
        check(
            ee::s09::f_002d5750__generated,
            avalanche_compare_u16,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_5750, 0);
                belib(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x102) as u64;
                let a = (r.next_u32() & 0xffff) as u16;
                let b = if seed % 4 == 0 {
                    a
                } else {
                    (r.next_u32() & 0xffff) as u16
                };
                h(c, DATA + 0x100, a);
                h(c, DATA + 0x102, b);
            },
        );
    }

    #[test]
    fn visual_effects_man_emitter_holder_dtor_matches() {
        check(
            ssx3_ee::s09::f_002d56b0__generated,
            visual_effects_man_emitter_holder_dtor,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2d56b0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                // The emitter holder's data pointer: null keeps the free path out.
                c.bus.write32(DATA + 0xd0, 0);
                let _ = r.next_u32();
            },
        );
    }
}
