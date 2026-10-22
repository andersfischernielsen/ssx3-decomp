//! `src/be/beintstat.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;
use crate::call::Callee;
use crate::events::deliver_event;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::sound::soundsys::p;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
pub(crate) use crate::game::fe::festaterewards::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_9778, bechar_int_is_trick_score_above_rating, "BECharInt_IsTrickScoreAboveRating"),
    (0x0014_99a8, belibrary_dtor, "BELibrary_dtor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::*;
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2 as IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2 as GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3 as MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4 as STR_HASH;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2 as BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2 as MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2 as BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT as SPEECH_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK as GET_LANGUAGE_MASK_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;
}


// ---------------------------------------------------------------------------
// BE library.

/// `BELibrary::~BELibrary(this, int flags)` (0x1499a8): free every singleton
/// the library owns and optionally the library itself.
pub fn belibrary_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x1f3c)).lw(c);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7e50, 0x0014_99c8)?;
        c.g[A0][0] = Addr(gp(c, -0x1f30)).lw(c);
        Addr(gp(c, -0x1f3c)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99d4)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee8)).lw(c);
        Addr(gp(c, -0x1f30)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99e0)?;
        c.g[A0][0] = Addr(gp(c, -0x1f2c)).lw(c);
        Addr(gp(c, -0x1ee8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99ec)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee4)).lw(c);
        Addr(gp(c, -0x1f2c)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99f8)?;
        Addr(gp(c, -0x1ee4)).sw(c, 0);
        call_ptr(c, 0x0015_3050, 0x0014_9a00)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0014_9a08)?;
        c.g[A0][0] = Addr(gp(c, -0x1eec)).lw(c);
        Addr(gp(c, -0x1ee0)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a14)?;
        c.g[A0][0] = Addr(gp(c, -0x1ea8)).lw(c);
        Addr(gp(c, -0x1eec)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a20)?;
        c.g[A0][0] = Addr(gp(c, -0x1f28)).lw(c);
        Addr(gp(c, -0x1ea8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a2c)?;
        c.g[A0][0] = Addr(gp(c, -0x1eb8)).lw(c);
        Addr(gp(c, -0x1f28)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a38)?;
        c.g[A0][0] = Addr(gp(c, -0x1edc)).lw(c);
        Addr(gp(c, -0x1eb8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a44)?;
        c.g[A0][0] = Addr(gp(c, -0x1ea4)).lw(c);
        Addr(gp(c, -0x1edc)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a50)?;
        c.g[A0][0] = Addr(gp(c, -0x1e8c)).lw(c);
        Addr(gp(c, -0x1ea4)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a5c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        Addr(gp(c, -0x1e8c)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0014_9a70)?;
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
// BE character queries.

/// `int BECharInt_IsTrickScoreAboveRating(char* self, void* rider, int value)`
/// (0x149778): true when `value` is at least the rating threshold of the
/// rider's overall rating level (clamped to the level table).
pub fn bechar_int_is_trick_score_above_rating(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD.call(c, 0x0014_97a0)?;

        let shared = r(c, V0) != 0;
        c.g[V0][0] = 0;
        if shared {
            return bechar_rating_tail(c, frame);
        }

        c.cycles += 30;
        c.g[V0][0] = 0x460000;
        c.g[A0][0] = c.g[S1][0];
        c.g[T0][0] = sx(r(c, V0).wrapping_sub(0x5d50));
        // Copy the 0x2c-byte rating table to the stack.
        c.ldl(3, p(r(c, T0), 0x7));
        c.ldr(3, r(c, T0));
        c.ldl(6, p(r(c, T0), 0xf));
        c.ldr(6, p(r(c, T0), 0x8));
        c.ldl(7, p(r(c, T0), 0x17));
        c.ldr(7, p(r(c, T0), 0x10));
        c.sdl(3, p(r(c, SP), 0x7));
        c.sdr(3, r(c, SP));
        c.sdl(6, p(r(c, SP), 0xf));
        c.sdr(6, p(r(c, SP), 0x8));
        c.sdl(7, p(r(c, SP), 0x17));
        c.sdr(7, p(r(c, SP), 0x10));
        c.ldl(3, p(r(c, T0), 0x1f));
        c.ldr(3, p(r(c, T0), 0x18));
        c.ldl(6, p(r(c, T0), 0x27));
        c.ldr(6, p(r(c, T0), 0x20));
        c.g[A3][0] = Addr(p(r(c, T0), 0x28)).lw(c);
        c.sdl(3, p(r(c, SP), 0x1f));
        c.sdr(3, p(r(c, SP), 0x18));
        c.sdl(6, p(r(c, SP), 0x27));
        c.sdr(6, p(r(c, SP), 0x20));
        c.sw(p(r(c, SP), 0x28), c.g[A3][0]);
        c.g[A1][0] = c.g[S0][0];
        callee::BECHAR_GET_OVERALL_RATING.call(c, 0x0014_9814)?;

        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x1));
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = 0xa;
        let level;
        if negative {
            level = 0;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            level = 1;
        }
        if level == 0 {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }

        c.cycles += 5;
        c.g[V0][0] = sx(r(c, V1) << 2);
        c.g[V1][0] = sx(p(r(c, SP), r(c, V0)));
        c.g[V0][0] = Addr(r(c, V1)).lw(c);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        bechar_rating_tail(c, frame)
    })
}


/// The `lq s0-s2; ld ra; addiu sp; jr ra` tail.
pub(crate) fn bechar_rating_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}


/// `void BELibrary_ResetPlayerProfile(BELibrary *self, int idx)` (0x149a88):
/// clear and initialise the rider-table entry for `idx`, format its label,
/// clear the profile-string slot, then reset the player, character and BAGT
/// interfaces' per-rider caches.
pub fn belibrary_reset_player_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 19, 0x50);
        frame.save(c, 17, 0x70);
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        frame.save_ra(c, 0x40);

        // idx = (signed char)arg1, entry = D_00535B20 + idx * 0x1c.
        let idx = (abi::arg(c, 1) as u32 as i8) as i32 as u32;
        c.g[V0][0] = 0x0053_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
        c.g[S3][0] = sx(idx);
        c.g[S1][0] = 0x1c;
        c.mult(0, 17, 19, 17);
        c.g[A2][0] = 0x1c;
        c.g[A1][0] = 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let entry = c.g[S1][0] as u32;
        c.g[A0][0] = c.g[S1][0];
        callee::MEMSET.call(c, 0x0014_9acc)?;

        // entry->flags |= 2; entry->+0x11 = 4.
        c.g[V1][0] = c.lw(entry.wrapping_add(0x10));
        c.g[V0][0] = 4;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = 0x0046_0000;
        c.g[V1][0] |= 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d98));
        c.sw(entry.wrapping_add(0x10), c.g[V1][0]);
        c.sb(entry.wrapping_add(0x11), c.g[V0][0]);

        // o = *(world + 0x8c); vt = *(o + 4); label = o + vt->delta.
        let o = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c)) as u32;
        let vt = c.lw(o.wrapping_add(4)) as u32;
        c.g[S2][0] = sx(vt);
        c.g[V0][0] = c.lh(vt.wrapping_add(0x20));
        c.g[S0][0] = sx(o);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = sx(vt);
        callee::STR_HASH.call(c, 0x0014_9b00)?;

        // label = FormatTagged((char*)sp, vt->slot(o)); then
        // sprintf(entry, label, idx + 1).
        c.g[V1][0] = c.lw(vt.wrapping_add(0x24));
        c.g[A0][0] = sx(o);
        let f = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        crate::call::call_ptr(c, f, 0x0014_9b10)?;
        let buf = c.g[SP][0] as u32;
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = c.g[V0][0];
        callee::FORMAT_TAGGED.call(c, 0x0014_9b1c)?;

        c.g[A1][0] = sx(buf);
        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.g[A0][0] = c.g[S1][0];
        callee::SPRINTF.call(c, 0x0014_9b2c)?;

        // Clear the profile-string table's slot for this rider.
        c.g[V1][0] = 0x9b50;
        c.g[A0][0] = 0x004a_0000;
        c.mult(0, 19, 19, 3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = sx(0x0014_9b48);
        c.pc = 0x0015_67b8;
        ee::s01::BE_ResetProfileCharacters(c);
        if c.pc != 0x0014_9b48 {
            return Err(crate::call::Unwound);
        }

        reset_interface(c, callee::BE_NEW_PLAYER_INT_GET_INSTANCE, 0x0014_9b50, 0x0014_9b64)?;
        reset_interface(c, callee::BE_CHAR_INT_GET_INSTANCE, 0x0014_9b6c, 0x0014_9b80)?;
        reset_interface(c, callee::BE_BAGT_INT_GET_INSTANCE, 0x0014_9b88, 0x0014_9b9c)?;

        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


/// The tail shared by the three interfaces: get the singleton, then call its
/// vtable slot `{int16 delta; fn}`(+0x10/+0x14) with `this + delta`.
pub(crate) fn reset_interface(c: &mut Cpu, get: Callee, get_ret: u32, vt_ret: u32) -> crate::call::Flow {
    get.call(c, get_ret)?;
    let obj = c.g[V0][0];
    c.g[V1][0] = c.lw((obj as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let f = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    crate::call::call_ptr(c, f, vt_ret)?;
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_vec;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP;
    use ssx3_ee as ee;
    use crate::difftest::fakes;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::sound::soundsys::tests::BE;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    pub(crate) use crate::game::sound::soundsys::tests::w;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_2;
    pub(crate) use crate::game::fe::festaterewards::tests::W;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_2;


        #[test]
    fn belibrary_dtor_matches() {
        check(ee::s01::f_001499a8__generated, belibrary_dtor, 8, &|c, seed| {
            let _ = base(c, seed, 0x1499a8, 2);
            let this = DATA;
            clear(c, this, 0x100);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            for off in [
                -0x1f3ci32, -0x1f30, -0x1ee8, -0x1f2c, -0x1ee4, -0x1ee0, -0x1eec, -0x1ea8, -0x1f28,
                -0x1eb8, -0x1edc, -0x1ea4, -0x1e8c,
            ] {
                w(
                    c,
                    GAME_GP.wrapping_sub(off as u32),
                    if seed % 2 == 0 { 0 } else { 0 },
                );
            }
        });
    }


    /// Call a setup helper (a generated function) with up to six arguments.
    pub(crate) fn call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }


    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base_2 = nodes;
            for n in 0..16u32 {
                let node = base_2 + block * n;
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base_2);
            w(c, GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }


    /// The dlmalloc arena the named allocators fall back to.
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;


    pub(crate) fn allocator(c: &mut Cpu) {
        pool_allocator(c);
        call(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }


    // ------------------------------------------------------------ BE_2 char

    #[test]
    fn bechar_int_is_trick_score_above_rating_matches() {
        strict(
            ee::s01::f_00149778__generated,
            bechar_int_is_trick_score_above_rating,
            60,
            &|c, seed| {
                let _ = base_2(c, seed, 0x1497, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed * 37) % 2000) as u64;
                // rider object for the BE_2 queries
                w(c, DATA + 0x800, (seed % 4) as u32);
                w(c, DATA + 0x804, ((seed / 4) % 5) as u32);
                w(c, DATA + 0x808, 1);
                w(c, DATA + 0x80c, 0);
                w(c, DATA + 0x810, (seed % 7) as u32);
            },
        );
    }
}
