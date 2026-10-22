//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for front-end popup/state handlers, lobby/net message
//! handlers, audio-system helpers, UI/world-script utilities and misc
//! engine leaves, each with a differential test against the generated body.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers may read `v0`/`v1` and the saved registers), follow the call
//! protocol of [`crate::call`] and the event rules of [`crate::events`].

use crate::abi;
use crate::call::{guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x001e_3c00, ov_screen_message_center_ctor, "OVScreenMessageCenter_ctor"),
    (0x0025_bf18, lobbyman_edit_account, "LobbyMan_EditAccount"),(0x002a_3708, ssx_audio_system_pa_say20c5, "SSXAudioSystem_PASay20C5"),(0x0012_f230, rider_reset_sub_enter, "RiderResetSub_Enter"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const MEMSET: Callee = Callee { addr: 0x0041_6210, f: ee::s15::f_00416210 };
    pub const STR_HASH: Callee = Callee { addr: 0x0031_7670, f: ee::s10::f_00317670 };
    pub const SPRINTF: Callee = Callee { addr: 0x0041_7828, f: ee::s15::f_00417828 };
    pub const FORMAT_TAGGED: Callee = Callee { addr: 0x002c_2580, f: ee::s08::f_002c2580 };
    pub const BE_RESET_PROFILE_CHARACTERS: Callee =
        Callee { addr: 0x0015_67b8, f: ee::s01::BE_ResetProfileCharacters };
    pub const BE_NEW_PLAYER_INT_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_5970, f: ee::s01::BENewPlayerInt_GetInstance };
    pub const BE_CHAR_INT_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_7f78, f: ee::s01::BECharInt_GetInstance };
    pub const BE_BAGT_INT_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_f900, f: ee::s01::BEBAGTInt_GetInstance };
    pub const C_MEM_MAN_ALLOC: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::f_00317d70 };
    pub const TAG_FIELD_FIND: Callee = Callee { addr: 0x003e_b6d8, f: ee::s14::f_003eb6d8 };
    pub const TAG_FIELD_GET_NUMBER: Callee = Callee { addr: 0x003e_cf30, f: ee::s14::f_003ecf30 };

    // Front-end back-end interface accessors.
    pub const CBE_GET_BE: Callee = Callee { addr: 0x0014_dc80, f: ee::s01::f_0014dc80 };
    pub const CBE_GET_INTERFACE: Callee = Callee { addr: 0x0014_dd58, f: ee::s01::f_0014dd58 };
    pub const GET_CHAR_ID: Callee = Callee { addr: 0x0014_7488, f: ee::s01::f_00147488 };
    pub const GET_ALT_CHAR: Callee = Callee { addr: 0x0014_74c8, f: ee::s01::f_001474c8 };

    // The GM handler and the memory-card manager.
    pub const GM_PICK_OPPONENTS: Callee = Callee { addr: 0x0023_b0a8, f: ee::s06::GMHandler8_PickOpponents };
    pub const MC_SET_MODE: Callee = Callee { addr: 0x0023_d570, f: ee::s06::MCManager_SetMode };
    pub const MC_GET_PRODUCT_CODE: Callee =
        Callee { addr: 0x0024_1aa0, f: ee::s06::MCManager_GetProductCode };
    pub const MC_GET_SAVE_TYPE_TAG: Callee =
        Callee { addr: 0x0024_1d40, f: ee::s06::MCManager_GetSaveTypeTag };
    pub const F_00152BA8: Callee = Callee { addr: 0x0015_2ba8, f: ee::s01::f_00152ba8 };
    pub const F_00152948: Callee = Callee { addr: 0x0015_2948, f: ee::s01::f_00152948 };
    pub const F_002C2540: Callee = Callee { addr: 0x002c_2540, f: ee::s08::f_002c2540 };
    pub const F_002C26D0: Callee = Callee { addr: 0x002c_26d0, f: ee::s08::f_002c26d0 };
    pub const MESSAGE_CENTER_IS_FLAG_SET: Callee =
        Callee { addr: 0x001e_3a30, f: ee::s04::MessageCenter_IsFlagSet };
    pub const MESSAGE_CENTER_SET_FLAG: Callee =
        Callee { addr: 0x001e_3a78, f: ee::s04::MessageCenter_SetFlag };
    pub const STR_INTERN: Callee = Callee { addr: 0x0039_8380, f: ee::s13::f_00398380 };
    pub const F_0039E2A0: Callee = Callee { addr: 0x0039_e2a0, f: ee::s13::f_0039e2a0 };

    // Lobby messages and the network manager.
    pub const SNPRINTF: Callee = Callee { addr: 0x003e_c180, f: ee::s14::f_003ec180 };
    pub const LOBBYMAN_SEND_REQUEST: Callee =
        Callee { addr: 0x0025_b608, f: ee::s06::LobbyMan_SendRequest };
    pub const NET_1: Callee = Callee { addr: 0x0026_0f80, f: ee::s06::f_00260f80 };
    pub const NET_2: Callee = Callee { addr: 0x0026_1460, f: ee::s06::f_00261460 };
    pub const NET_3: Callee = Callee { addr: 0x0026_13c8, f: ee::s06::f_002613c8 };
    pub const NET_4: Callee = Callee { addr: 0x0026_1408, f: ee::s06::f_00261408 };
    pub const NET_FLUSH: Callee = Callee { addr: 0x0025_b800, f: ee::s06::f_0025b800 };
    pub const NET_5: Callee = Callee { addr: 0x0025_5fa0, f: ee::s06::f_00255fa0 };

    // Audio speech helpers.
    pub const IS_PA_SPEECH_ON: Callee = Callee { addr: 0x0029_f128, f: ee::s08::f_0029f128 };
    pub const IS_REPLAY_PLAYING: Callee = Callee { addr: 0x0028_8ae0, f: ee::s07::f_00288ae0 };
    pub const RIDER_123168: Callee = Callee { addr: 0x0012_3168, f: ee::s00::f_00123168 };
    pub const IS_RACE_TYPE_4OR5: Callee = Callee { addr: 0x002a_4248, f: ee::s08::f_002a4248 };
    pub const BANK_MONITOR_ADD_TIMER: Callee =
        Callee { addr: 0x002a_dca0, f: ee::s08::f_002adca0 };
    pub const SPEECH_SET_EVENT_CONTEXT: Callee =
        Callee { addr: 0x002b_1458, f: ee::s08::f_002b1458 };
    pub const GET_LANGUAGE_MASK: Callee = Callee { addr: 0x0029_f198, f: ee::s08::f_0029f198 };
    pub const GET_RIDER_VOICE_MASK: Callee = Callee { addr: 0x002a_1d48, f: ee::s08::f_002a1d48 };
    pub const GET_FINISH_SPEECH_MASK: Callee =
        Callee { addr: 0x002a_1e68, f: ee::s08::f_002a1e68 };
    pub const SPEECH_QUEUE: Callee = Callee { addr: 0x003d_8008, f: ee::s14::f_003d8008 };

    // Network teardown.
    pub const NET_FIND: Callee = Callee { addr: 0x0025_5d38, f: ee::s06::f_00255d38 };
    pub const NET_REMOVE: Callee = Callee { addr: 0x0025_5cc8, f: ee::s06::f_00255cc8 };
    pub const VOIP_DEACTIVATE: Callee = Callee { addr: 0x0026_6e88, f: ee::s07::cVoipMan_Deactivate };
    pub const VOIP_STOP: Callee = Callee { addr: 0x0026_6df8, f: ee::s07::cVoipMan_Stop };
    pub const SETUPMAN_DESTROY: Callee = Callee { addr: 0x0026_4098, f: ee::s07::cSetupMan_Destroy };
    pub const NET_RESET: Callee = Callee { addr: 0x0025_f740, f: ee::s06::f_0025f740 };

    // Text layout helpers for the popup description.
    pub const TEXT_OBJ: Callee = Callee { addr: 0x003a_04f0, f: ee::s13::f_003a04f0 };
    pub const MEASURE_PLAIN: Callee = Callee { addr: 0x0039_21f0, f: ee::s12::f_003921f0 };
    pub const MEASURE_WRAP: Callee = Callee { addr: 0x0039_1fb0, f: ee::s12::f_00391fb0 };

    // Rider reset transition effect.
    pub const ANIM_GET_LAYER_SEQUENCE: Callee =
        Callee { addr: 0x0031_1b20, f: ee::s10::AnimController_GetLayerSequence };
    pub const IS_LOCAL_PLAYER: Callee =
        Callee { addr: 0x0012_f588, f: ee::s00::RiderResetSub_IsLocalPlayer };
    pub const GET_TRANSITION_MAN: Callee =
        Callee { addr: 0x0023_0698, f: ee::s06::GameModule_GetTransitionMan };
    pub const TRANSITION_STOP: Callee = Callee { addr: 0x002e_4578, f: ee::s09::cTransitionMan_Stop };
    pub const EFFECT_ALLOC: Callee = Callee { addr: 0x002e_4ce8, f: ee::s09::TransitionEffect_AllocData };
    pub const EFFECT_CTOR: Callee = Callee { addr: 0x002e_4d70, f: ee::s09::TransitionEffect_ctor };
    pub const TRANSITION_START: Callee = Callee { addr: 0x002e_4370, f: ee::s09::cTransitionMan_Start };
}

/// `D_004A2C70`: the current GM handler object (`lw(gp - 0x480)`).
const GM_HANDLER: u32 = 0x480;
/// `D_00534B30`: "player profile exists" flag.
const HAVE_PROFILE: u32 = 0x0053_4b30;
const D_00535C04: u32 = 0x0053_5c04;

/// `sRiderEntry` table.
const RIDER_ENTRIES: u32 = 0x0053_5b20;

/// Tag names looked up in the lobby reply's tag list.
const TAG_A: u32 = 0x004a_3198;
const TAG_IDX: u32 = 0x004a_31a8;
const TAG_B: u32 = 0x004a_31a0;

// ---------------------------------------------------------------------------
// Functions.

/// `void cFEPopupConfirm_CfgReset(cFEPopupConfirm *self)` (0x1c5630): clear
/// the four 8-byte entries at +4, the five {8-byte, 12-byte} pairs at +0x24
/// and +0x54, then reset the popup's counters, the vector at +0x9c and the
/// "no selection" index (-1).
pub fn cfepopupconfirm_cfg_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);

        // The four 8-byte entries at +4, +0xc, +0x14, +0x1c.
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        const CLEAR_RETS: [u32; 4] = [0x001c_5664, 0x001c_5678, 0x001c_5688, 0x001c_5698];
        for (i, off) in [4u32, 0xc, 0x14, 0x1c].into_iter().enumerate() {
            c.g[A0][0] = sx(this.wrapping_add(off));
            callee::MEMSET.call(c, CLEAR_RETS[i])?;
        }

        // Five iterations clearing an 8-byte entry (from +0x24, stride 8) and
        // a 12-byte entry (from +0x54, stride 0xc).
        c.g[S1][0] = sx(this.wrapping_add(0x24));
        c.g[S2][0] = sx(this.wrapping_add(0x54));
        c.g[S3][0] = 4;
        loop {
            c.cycles += 12;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 8;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            callee::MEMSET.call(c, 0x001c_56b0)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0xc;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            callee::MEMSET.call(c, 0x001c_56c8)?;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[A0][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_56a0);
            }
        }

        c.cycles += 32;
        c.g[A0][0] = sx(this.wrapping_add(0x4c));
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        callee::MEMSET.call(c, 0x001c_56e0)?;

        let one = 1;
        c.g[V1][0] = one;
        c.sw(this.wrapping_add(0x98), 0);
        c.sw(this.wrapping_add(0xbc), one);
        c.g[V0][0] = 0x0050_0000;
        c.sw(this.wrapping_add(0xa8), one);
        let minus_one = u64::MAX;
        c.g[A0][0] = minus_one;
        c.sw(this.wrapping_add(0x94), 0);
        c.sw(this.wrapping_add(0xb8), 0);
        c.sw(this.wrapping_add(0xb0), 0);
        c.sw(this, 0);
        c.sw(this.wrapping_add(0x90), 0);
        // *(sVec3*)(self + 0x9c) = D_004FF0D8 (a 12-byte copy, unaligned).
        let src = (c.g[V0][0] as u32).wrapping_sub(0xf28);
        c.g[A3][0] = sx(src);
        c.ldl(3, src.wrapping_add(7));
        c.ldr(3, src);
        c.g[A1][0] = c.lw(src.wrapping_add(8));
        c.sdl(3, this.wrapping_add(0xa3));
        c.sdr(3, this.wrapping_add(0x9c));
        c.sw(this.wrapping_add(0xa4), c.g[A1][0]);
        c.sw(this.wrapping_add(0xac), minus_one);
        c.sw(this.wrapping_add(0xb4), 0);
        c.sw(this.wrapping_add(0xc0), 0);

        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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
fn reset_interface(c: &mut Cpu, get: Callee, get_ret: u32, vt_ret: u32) -> crate::call::Flow {
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

/// `void LobbyMan_OnSnapshotReply(LobbyMan *self, Msg *msg)` (0x25e590): a
/// snapshot reply either carries the rider's score in three tag fields
/// (stored summed at `self + idx*4 + 0x104`) or a message to append to the
/// pending list at `self+0xf0`.
pub fn lobbyman_on_snapshot_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 10;
        frame.save(c, 16, 0x60);
        frame.save(c, 19, 0x30);
        let msg = abi::arg(c, 1);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save(c, 17, 0x50);
        frame.save_ra(c, 0x20);
        frame.save(c, 18, 0x40);
        c.g[S3][0] = abi::arg64(c, 0);
        c.g[S2][0] = c.lw(msg.wrapping_add(8));

        if c.g[S2][0] == 0 {
            c.cycles += 28;
            // a = TagFieldGetNumber(TagFieldFind(msg->tags, TAG_A), 0)
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(TAG_A);
            callee::TAG_FIELD_FIND.call(c, 0x0025_e5c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_e5d4)?;
            c.g[S1][0] = c.g[V0][0];
            // idx = TagFieldGetNumber(TagFieldFind(msg->tags, TAG_IDX), 0)
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = sx(TAG_IDX);
            callee::TAG_FIELD_FIND.call(c, 0x0025_e5e8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_e5f4)?;
            c.g[S0][0] = c.g[V0][0];
            // b = TagFieldGetNumber(TagFieldFind(msg->tags, TAG_B), 0)
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(TAG_B);
            callee::TAG_FIELD_FIND.call(c, 0x0025_e608)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_e614)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x104), c.g[S1][0]);
            frame.restore(c, 16, 0x60);
            frame.restore(c, 17, 0x50);
            frame.restore(c, 18, 0x40);
            frame.restore(c, 19, 0x30);
            c.cycles += 7;
            frame.restore_ra(c, 0x20);
            frame.close(c);
            return Ok(());
        }

        // Non-zero: build a 0xc-byte message node and prepend it to the list.
        c.cycles += 25;
        c.g[A1][0] = 0x0048_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_e640)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf0));
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = 0x102;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S2][0]);
        c.g[A1][0] = 0x0048_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_e67c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
        }

        // sMsgList_insert(head, node): splice the node in after the head.
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.cycles += 7;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `lw(gp - off)`.
#[inline(always)]
fn gpl(c: &mut Cpu, off: u32) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(off))
}

/// `void GMHandler8_Reset(void *self)` (0x23ac88): reset the GM handler's
/// screen fields, clear the 5*8-byte table at `self+4`, run its vtable's
/// slot 5 and copy the player's character ids and alt-character ids into the
/// handler; pick opponents when the mode word was 1.
pub fn gmhandler8_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;

        // player = cBE_getInterface(cBE_getBE(), 1)
        callee::CBE_GET_BE.call(c, 0x0023_aca8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        callee::CBE_GET_INTERFACE.call(c, 0x0023_acb4)?;
        c.g[S0][0] = c.g[V0][0];

        let gm = gpl(c, GM_HANDLER);
        let this = c.g[S1][0] as u32;
        c.g[V1][0] = 4;
        c.g[V0][0] = gm;
        c.g[A0][0] = 3;
        c.g[A2][0] = 2;
        c.g[A3][0] = u64::MAX;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = 7;
        c.g[V1][0] = gpl(c, 0x848);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A3][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[S2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A0][0]);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        callee::CBE_GET_INTERFACE.call(c, 0x0023_ad0c)?;

        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
        let profile_missing = c.g[V0][0] == 0;
        c.g[V0][0] = gpl(c, GM_HANDLER);
        if !profile_missing {
            c.cycles += 10;
            callee::CBE_GET_BE.call(c, 0x0023_ad24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::CBE_GET_INTERFACE.call(c, 0x0023_ad30)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = gpl(c, GM_HANDLER);
            c.g[V0][0] = c.lw(D_00535C04);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = gpl(c, GM_HANDLER);
        }

        c.cycles += 36;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = 9;
        c.g[A0][0] = sx(this.wrapping_add(4));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
        callee::MEMSET.call(c, 0x0023_ad68)?;

        // self's vtable slot 5: {int16 delta; fn}(self + delta)
        c.g[V1][0] = c.lw(this);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let f = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        crate::call::call_ptr(c, f, 0x0023_ad7c)?;

        let player = c.g[S0][0];
        for (alt, ret, slot) in [(0u64, 0x0023_ad88u32, 0x18u32), (1, 0x0023_ad9c, 0x1c)] {
            if alt == 0 {
                c.g[A0][0] = player;
                c.g[A1][0] = 0;
                callee::GET_CHAR_ID.call(c, ret)?;
            } else {
                c.g[A0][0] = player;
                c.g[A1][0] = 1;
                callee::GET_CHAR_ID.call(c, ret)?;
            }
            c.g[V1][0] = gpl(c, GM_HANDLER);
            c.sw((c.g[V1][0] as u32).wrapping_add(slot), c.g[V0][0]);
        }
        for (slot, ret) in [(0x40u32, 0x0023_adb0u32), (0x44, 0x0023_adc4)] {
            let alt = if slot == 0x40 { 0 } else { 1 };
            c.g[A0][0] = player;
            c.g[A1][0] = alt;
            callee::GET_ALT_CHAR.call(c, ret)?;
            c.g[V1][0] = gpl(c, GM_HANDLER);
            c.sw((c.g[V1][0] as u32).wrapping_add(slot), c.g[V0][0]);
        }
        c.g[V1][0] = gpl(c, GM_HANDLER);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        let pick = c.g[A0][0] != c.g[S2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        if !pick {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0023_addc);
            c.pc = 0x0023_b0a8;
            ee::s06::GMHandler8_PickOpponents(c);
            if c.pc != 0x0023_addc {
                return Err(crate::call::Unwound);
            }
        }

        c.cycles += 8;
        c.g[V0][0] = gpl(c, GM_HANDLER);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginSave(MCManager *self, int mode, int arg)` (0x23cae8):
/// start a save: record the mode, pick the "saving"/"loading" state word and
/// tell the movie object, then format the three 0x200-byte strings (title,
/// product code, save-type tag) into the request at `self+0x1b4`.
pub fn mcmanager_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x640);
        frame.save(c, 18, 0x610);
        let self_ = abi::arg64(c, 0);
        frame.save(c, 16, 0x630);
        frame.save(c, 17, 0x620);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save_ra(c, 0x600);
        let this = self_ as u32;
        c.sw(this.wrapping_add(0xb4), abi::arg64(c, 1));
        c.g[A1][0] = 1;
        callee::MC_SET_MODE.call(c, 0x0023_cb10)?;

        c.g[V1][0] = 1;
        c.g[V0][0] = c.lw(this.wrapping_add(0xb4));
        c.sw(this.wrapping_add(0xfc), c.g[S0][0]);
        let nonzero = c.g[V0][0] != 0;
        c.sw(this.wrapping_add(0xbc), c.g[V1][0]);
        // `buf` receives the format string; `desc`, `name` the other two.
        let fmt = (c.g[SP][0] as u32).wrapping_add(0x400);
        let desc = c.g[SP][0] as u32;
        let name = (c.g[SP][0] as u32).wrapping_add(0x200);
        if !nonzero {
            // mode 0: "saving", tell the movie object directly.
            c.cycles += 5;
            c.g[V0][0] = 4;
            c.g[A2][0] = c.lw(this.wrapping_add(0x434));
            c.sw(this.wrapping_add(0xec), c.g[V0][0]);
            c.g[A1][0] = 0x0008_0000;
            c.cycles += 7;
            movie_finish_step(c, 0x0023_cb78)?;
            c.g[S0][0] = sx(fmt);
        } else {
            c.cycles += 2;
            let mode_is_one = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = 6;
            if !mode_is_one {
                // mode 2+: "loading", ask the BAGT interface.
                c.cycles += 15;
                c.sw(this.wrapping_add(0xec), c.g[V1][0]);
                callee::CBE_GET_BE.call(c, 0x0023_cb88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_cb94)?;
                callee::F_00152948.call(c, 0x0023_cb9c)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                movie_finish_step(c, 0x0023_cbb8)?;
                c.g[S0][0] = sx(fmt);
            } else {
                // mode 1: "saving", ask the player interface.
                c.cycles += 9;
                c.sw(this.wrapping_add(0xec), c.g[V0][0]);
                callee::CBE_GET_BE.call(c, 0x0023_cb48)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_cb54)?;
                callee::F_00152BA8.call(c, 0x0023_cb5c)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                c.cycles += 7;
                movie_finish_step(c, 0x0023_cb78)?;
                c.g[S0][0] = sx(fmt);
            }
        }

        c.cycles += 24;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        callee::F_002C2540.call(c, 0x0023_cbcc)?;
        c.g[A0][0] = sx(desc);
        c.g[A1][0] = 0x200;
        callee::MC_GET_PRODUCT_CODE.call(c, 0x0023_cbd8)?;
        c.g[A1][0] = c.lw(this.wrapping_add(0xb4));
        c.g[A0][0] = self_;
        c.g[A2][0] = sx(name);
        c.g[A3][0] = 0x100;
        callee::MC_GET_SAVE_TYPE_TAG.call(c, 0x0023_cbf0)?;
        c.g[A0][0] = sx(this.wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = sx(name);
        c.g[A2][0] = sx(desc);
        callee::F_002C26D0.call(c, 0x0023_cc04)?;
        frame.restore(c, 16, 0x630);
        frame.restore(c, 17, 0x620);
        frame.restore(c, 18, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginLoad(MCManager *self, int mode)` (0x23c8f0): the load
/// twin of [`mcmanager_begin_save`] (`-1` in the request slot, no cached
/// value, and the "loading" state word in the mode-2 path).
pub fn mcmanager_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x640);
        frame.save(c, 18, 0x610);
        let self_ = abi::arg64(c, 0);
        frame.save(c, 16, 0x630);
        frame.save(c, 17, 0x620);
        frame.save_ra(c, 0x600);
        let this = self_ as u32;
        c.sw(this.wrapping_add(0xb4), abi::arg64(c, 1));
        c.g[A1][0] = 1;
        callee::MC_SET_MODE.call(c, 0x0023_c914)?;

        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = c.lw(this.wrapping_add(0xb4));
        c.sw(this.wrapping_add(0xfc), c.g[V0][0]);
        let nonzero = c.g[V1][0] != 0;
        c.sw(this.wrapping_add(0xbc), 0);
        let fmt = (c.g[SP][0] as u32).wrapping_add(0x400);
        let desc = c.g[SP][0] as u32;
        let name = (c.g[SP][0] as u32).wrapping_add(0x200);
        if !nonzero {
            // mode 0: "loading", tell the movie object directly.
            c.cycles += 5;
            c.g[V0][0] = 4;
            c.g[A2][0] = c.lw(this.wrapping_add(0x434));
            c.sw(this.wrapping_add(0xec), c.g[V0][0]);
            c.g[A1][0] = 0x0008_0000;
            c.cycles += 7;
            movie_finish_step(c, 0x0023_c984)?;
            c.g[S0][0] = sx(fmt);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                // mode 2+: "loading", ask the BAGT interface.
                c.cycles += 15;
                c.sw(this.wrapping_add(0xec), c.g[V0][0]);
                callee::CBE_GET_BE.call(c, 0x0023_c994)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_c9a0)?;
                callee::F_00152948.call(c, 0x0023_c9a8)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                movie_finish_step(c, 0x0023_c9c4)?;
                c.g[S0][0] = sx(fmt);
            } else {
                // mode 1: "loading", ask the player interface.
                c.cycles += 10;
                c.g[V0][0] = 6;
                c.sw(this.wrapping_add(0xec), c.g[V0][0]);
                callee::CBE_GET_BE.call(c, 0x0023_c954)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_c960)?;
                callee::F_00152BA8.call(c, 0x0023_c968)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                c.cycles += 7;
                movie_finish_step(c, 0x0023_c984)?;
                c.g[S0][0] = sx(fmt);
            }
        }

        c.cycles += 24;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        callee::F_002C2540.call(c, 0x0023_c9d8)?;
        c.g[A0][0] = sx(desc);
        c.g[A1][0] = 0x200;
        callee::MC_GET_PRODUCT_CODE.call(c, 0x0023_c9e4)?;
        c.g[A1][0] = c.lw(this.wrapping_add(0xb4));
        c.g[A0][0] = self_;
        c.g[A2][0] = sx(name);
        c.g[A3][0] = 0x100;
        callee::MC_GET_SAVE_TYPE_TAG.call(c, 0x0023_c9fc)?;
        c.g[A0][0] = sx(this.wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = sx(name);
        c.g[A2][0] = sx(desc);
        callee::F_002C26D0.call(c, 0x0023_ca10)?;
        frame.restore(c, 16, 0x630);
        frame.restore(c, 17, 0x620);
        frame.restore(c, 18, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenMessageCenter *OVScreenMessageCenter_ctor(void *self, void *owner,
/// int arg)` (0x1e3c00): build a message-center screen over the base screen
/// constructor: install both vtables, clear the eight 5-word rows at +0x9c
/// and the counters, look up three string hashes in the owner's engine
/// dictionary, then record the "message center shown" flag.
pub fn ov_screen_message_center_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        let this64 = abi::arg64(c, 0);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        let this = this64 as u32;
        c.g[S1][0] = this64;
        c.g[S0][0] = abi::arg64(c, 2);
        callee::F_0039E2A0.call(c, 0x001e_3c1c)?;

        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(8), c.g[V1][0]);
        c.g[A0][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V0][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4318));
        c.g[A1][0] = sx(this.wrapping_add(0x11c));
        c.g[A2][0] = 7;
        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(8), c.g[V0][0]);
        c.sh(this.wrapping_add(0x58), c.g[V1][0]);

        // Eight rows of five words, at +0x9c, +0xbc, +0xdc, +0xfc and +0x11c.
        loop {
            c.cycles += 8;
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x80), 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x60), 0);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x40), 0);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x20), 0);
            c.sw(c.g[A1][0] as u32, 0);
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001e_3c58);
            }
        }

        c.cycles += 36;
        c.sw(this.wrapping_add(0x13c), 0);
        c.g[A0][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x140), 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d8));
        c.sw(this.wrapping_add(0x144), 0);
        c.sw(this.wrapping_add(0x14c), 0);
        c.sw(this.wrapping_add(0x150), 0);
        c.sw(this.wrapping_add(0x154), 0);
        c.sw(this.wrapping_add(0x158), 0);
        c.sw(this.wrapping_add(0x15c), 0);
        c.sw(this.wrapping_add(0x160), 0);

        // The three keys: hash the name, intern it in the engine dictionary.
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        callee::STR_HASH.call(c, 0x001e_3cac)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::STR_INTERN.call(c, 0x001e_3cb8)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2098));
        c.sw(this.wrapping_add(0x164), c.g[V0][0]);
        callee::STR_HASH.call(c, 0x001e_3ccc)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::STR_INTERN.call(c, 0x001e_3cd8)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2088));
        c.sw(this.wrapping_add(0x168), c.g[V0][0]);
        callee::STR_HASH.call(c, 0x001e_3cec)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::STR_INTERN.call(c, 0x001e_3cf8)?;

        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1024));
        c.g[V1][0] = !(0 | c.g[V1][0]);
        let unset = c.g[V1][0] == 0;
        c.sw(this.wrapping_add(0x16c), c.g[V0][0]);
        if unset {
            // Already shown: 2 means "no".
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.sw(this.wrapping_add(0x18), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 4;
            c.g[A0][0] = 7;
            callee::MESSAGE_CENTER_IS_FLAG_SET.call(c, 0x001e_3d10)?;
            let first = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if first {
                c.cycles += 1;
                c.sw(this.wrapping_add(0x18), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sw(this.wrapping_add(0x18), 0);
            }
            c.cycles += 5;
            c.g[A0][0] = 7;
            c.g[A1][0] = 1;
            callee::MESSAGE_CENTER_SET_FLAG.call(c, 0x001e_3d30)?;
            c.g[V0][0] = c.g[S1][0];
        }

        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_EditAccount(LobbyMan *self, Msg *msg)` (0x25bf18): build the
/// account-edit screen's four message strings into a stack buffer and send
/// the "edit" request with the account id.
pub fn lobbyman_edit_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x270);
        c.g[A2][0] = 0x004a_0000;
        frame.save(c, 17, 0x250);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
        frame.save(c, 18, 0x240);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, 16, 0x260);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x230);
        let this = c.g[S2][0] as u32;
        let msg = c.g[S1][0] as u32;
        let buf = c.g[SP][0] as u32;

        c.g[A0][0] = sx(buf);
        c.g[A1][0] = 0x200;
        c.g[A3][0] = c.lw(this.wrapping_add(0x48));
        c.sb(buf, 0);
        callee::SNPRINTF.call(c, 0x0025_bf50)?;
        c.g[V1][0] = c.lw(msg.wrapping_add(0x44));
        let no_title = c.g[V1][0] == 0;
        c.g[A2][0] = 0x004a_0000;
        if !no_title {
            c.cycles += 11;
            c.g[A3][0] = c.lw(this.wrapping_add(0x4c));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bf70)?;
            c.g[A2][0] = 0x004a_0000;
            c.g[A3][0] = c.lw(msg.wrapping_add(4));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30a0));
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bf88)?;
        }

        // Second string: the two "selected" markers.
        c.cycles += 3;
        c.g[V0][0] = c.lw(msg.wrapping_add(0x48));
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = 0x59;
        if !none {
            c.cycles += 17;
            c.g[A0][0] = c.lw(msg.wrapping_add(0x38));
            c.g[V1][0] = c.lw(msg.wrapping_add(0x3c));
            c.g[A2][0] = 0x4e;
            if c.g[A0][0] != 0 {
                c.g[A2][0] = c.g[V0][0];
            }
            let line = c.g[SP][0] as u32 + 0x200;
            c.g[S0][0] = sx(line);
            c.g[A1][0] = 0x004a_0000;
            c.g[A3][0] = 0x4e;
            if c.g[V1][0] != 0 {
                c.g[A3][0] = c.g[V0][0];
            }
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3030));
            c.g[A0][0] = sx(line);
            callee::SPRINTF.call(c, 0x0025_bfc0)?;
            c.g[A2][0] = 0x004a_0000;
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3048));
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bfd8)?;
        }

        // Third string: the account id.
        c.cycles += 3;
        c.g[V0][0] = c.lw(msg.wrapping_add(0x4c));
        let no_id = c.g[V0][0] == 0;
        c.g[A0][0] = sx(buf);
        if !no_id {
            c.cycles += 5;
            c.g[A2][0] = 0x004a_0000;
            c.g[A3][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3050));
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bff8)?;
        }

        // Send the request and queue a 0xc-byte node on self+0xf0.
        c.cycles += 34;
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(1, 12); // f12 = 20.0
        c.g[T0][0] = 0x0027_0000;
        c.g[A2][0] = 0x6564_0000;
        c.g[A3][0] = c.g[SP][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7408));
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[A2][0] | 0x6974;
        callee::LOBBYMAN_SEND_REQUEST.call(c, 0x0025_c020)?;

        c.g[S0][0] = 0xcd;
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] = 8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_c03c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = c.lw(this.wrapping_add(0xf0));
        c.g[V0][0] = 0x0048_0000;
        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = 0x0048_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x220), c.g[V1][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_c070)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
        }

        // Splice the node in after the list head.
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
        frame.restore(c, 16, 0x260);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        frame.restore(c, 17, 0x250);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        frame.restore(c, 18, 0x240);
        c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
        frame.restore_ra(c, 0x230);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
        c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x210), c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_DisconnectAll(void *self, void *arg)` (0x2636f0): tear down the
/// network: make sure the net manager exists, stop any session, clear the
/// pending-message list and post the "disconnected" notification.
pub fn net_disconnect_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = abi::arg64(c, 1);
        frame.save(c, 16, 0x60);
        c.g[S3][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x50);
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !have {
            c.cycles += 2;
            callee::NET_1.call(c, 0x0026_3720)?;
        }

        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::NET_2.call(c, 0x0026_3728)?;
        let running = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if !running {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::NET_3.call(c, 0x0026_3738)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        }

        c.cycles += 2;
        let pending = c.g[V0][0] != 0;
        if !pending {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            callee::NET_FLUSH.call(c, 0x0026_374c)?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::NET_4.call(c, 0x0026_3754)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        callee::NET_FLUSH.call(c, 0x0026_375c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let idle = c.g[V0][0] == 0;
        frame.restore(c, 16, 0x60);
        if !idle {
            c.cycles += 4;
            callee::NET_5.call(c, 0x0026_3778)?;
            let no_arg = c.g[S2][0] == 0;
            c.g[A1][0] = 0x0048_0000;
            if !no_arg {
                c.cycles += 25;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xed0));
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                callee::C_MEM_MAN_ALLOC.call(c, 0x0026_3798)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x0048_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
                c.g[V0][0] = 0x116;
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[A1][0] = 0x0048_0000;
                c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S2][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                callee::C_MEM_MAN_ALLOC.call(c, 0x0026_37d4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
                c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
                c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
                c.sw(c.g[SP][0] as u32, c.g[A1][0]);
            }
            c.cycles += 2;
            c.sw(c.g[S3][0] as u32, 0);
            frame.restore(c, 16, 0x60);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_AbortGameSetup(int *self, int arg)` (0x263ba8): drop the pending
/// game-setup session, stop VOIP and the setup manager, reset the net lobby
/// and post the "abort" message (0x10c for arg 1, 0x112 for arg 0).
pub fn net_abort_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        let no_session = c.g[V0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 1);
        if !no_session {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
            callee::NET_FIND.call(c, 0x0026_3bd4)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if !missing {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                callee::NET_REMOVE.call(c, 0x0026_3be4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_DEACTIVATE.call(c, 0x0026_3bec)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_STOP.call(c, 0x0026_3bf4)?;
            }
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
        let no_setup = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        if !no_setup {
            c.cycles += 3;
            callee::SETUPMAN_DESTROY.call(c, 0x0026_3c08)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        }

        c.cycles += 5;
        c.g[A1][0] = 0;
        callee::NET_RESET.call(c, 0x0026_3c14)?;
        c.g[V0][0] = 1;
        let other = c.g[S1][0] != c.g[V0][0];
        c.sw(c.g[S0][0] as u32, 0);
        if other {
            // Neither 0 nor 1: no message.
            c.cycles += 2;
            let queued = c.g[S1][0] != 0;
            frame.restore(c, 16, 0x40);
            if !queued {
                c.cycles += 9;
                c.g[A1][0] = 0x0048_0000;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[A0][0] = 8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                callee::C_MEM_MAN_ALLOC.call(c, 0x0026_3c6c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x112;
                post_node(c, 0x0026_3ca4)?;
            }
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x0048_0000;
            c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::C_MEM_MAN_ALLOC.call(c, 0x0026_3c3c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = 0x10c;
            post_node(c, 0x0026_3ca4)?;
        }

        c.cycles += 4;
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Post the 8-byte node `s1` (already holding its tag in `v1` and the class
/// pointer) onto the lobby message list at `s0+0xf0`, then restore `s0`.
fn post_node(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 16;
    c.g[V0][0] = 0x0048_0000;
    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = 0x0048_0000;
    c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf0));
    c.g[A3][0] = 0;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    callee::C_MEM_MAN_ALLOC.call(c, ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0 {
        c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
    }
    c.cycles += 10;
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    Ok(())
}

/// `void SSXAudioSystem_PASay20C5(SSXAudioSystem *self, Rider *rider)`
/// (0x2a3708): when PA speech is on and the rider is in a race, either queue
/// the 1-second bank timer or queue the finish speech (language, voice and
/// finish masks) as event context 0x20c5.
pub fn ssx_audio_system_pa_say20c5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = abi::arg64(c, 1);

        let mut out = false;
        callee::IS_PA_SPEECH_ON.call(c, 0x002a_372c)?;
        out = c.g[V0][0] == 0;
        if !out {
            c.cycles += 4;
            callee::IS_REPLAY_PLAYING.call(c, 0x002a_373c)?;
            out = c.g[V0][0] != 0;
        }
        if !out {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            callee::RIDER_123168.call(c, 0x002a_374c)?;
            out = c.g[V0][0] != 0;
        }
        let mut bank_path = false;
        if !out {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::IS_RACE_TYPE_4OR5.call(c, 0x002a_375c)?;
            let not45 = c.g[V0][0] == 0;
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6c0));
            bank_path = !not45;
        }
        if bank_path {
            c.cycles += 22;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6c0));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[S1][0] = 0x004a_0000;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x36e8));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let f = c.g[V0][0] as u32;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            crate::call::call_ptr(c, f, 0x002a_378c)?;
            c.ldl(6, (c.g[S1][0] as u32).wrapping_add(7));
            c.ldr(6, (c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3e8;
            c.g[A3][0] = 0;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            callee::BANK_MONITOR_ADD_TIMER.call(c, 0x002a_37b4)?;
            frame.restore(c, 16, 0x40);
        } else if !out {
            c.cycles += 11;
            c.mtc1(0, 12); // fourth float argument = 0.0
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20c5;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xb;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_37e0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 23;
                callee::GET_LANGUAGE_MASK.call(c, 0x002a_37f0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                callee::GET_RIDER_VOICE_MASK.call(c, 0x002a_3800)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                callee::GET_FINISH_SPEECH_MASK.call(c, 0x002a_3810)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20c5;
                callee::SPEECH_QUEUE.call(c, 0x002a_3824)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S0][0];
                let f = c.g[V1][0] as u32;
                c.g[A1][0] = 3;
                crate::call::call_ptr(c, f, 0x002a_3840)?;
                frame.restore(c, 16, 0x40);
            }
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_MeasureDesc(TextObj *text, Vec3 *scale, int *info,
/// float *out)` (0x1c8320): scale the text object's layout size by the
/// caller's scale and measure the description: a plain or a wrapped single
/// line for info[0] 1/2, a looked-up string (or sprintf'd number) for 3, and
/// two zero words for 0.
pub fn cfepopupconfirm_measure_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, 17, 0x80);
        frame.save(c, 18, 0x70);
        let text64 = abi::arg64(c, 1);
        frame.save(c, 19, 0x60);
        c.g[S1][0] = text64;
        c.g[S2][0] = abi::arg64(c, 2);
        frame.save(c, 16, 0x90);
        let text = text64 as u32;
        c.g[A0][0] = text64;
        frame.save_ra(c, 0x50);
        // Copy the text object's layout scale (three words from +0x50).
        c.ldl(2, text.wrapping_add(0x57));
        c.ldr(2, text.wrapping_add(0x50));
        c.g[V1][0] = c.lw(text.wrapping_add(0x58));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[SP][0] as u32);
        c.sw((c.g[SP][0] as u32).wrapping_add(8), c.g[V1][0]);
        c.g[S3][0] = abi::arg64(c, 3);
        callee::TEXT_OBJ.call(c, 0x001c_8364)?;

        // g->size.x *= v.x, g->size.y *= v.y
        c.lwc1(2, c.g[SP][0] as u32);
        c.g[V1][0] = 1;
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(4));
        let g = c.g[V0][0] as u32;
        c.lwc1(0, g.wrapping_add(0x30));
        c.lwc1(1, g.wrapping_add(0x34));
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, g.wrapping_add(0x38));
        c.swc1(1, g.wrapping_add(0x3c));

        let info = c.g[S2][0] as u32;
        let out = c.g[S3][0];
        c.g[A0][0] = c.lw(info);
        let sel = c.g[A0][0];
        let mut v0 = ((c.g[A0][0] as i64) < 2) as u64;
        c.g[V0][0] = v0;
        if sel == c.g[V1][0] {
            // case 1
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            callee::TEXT_OBJ.call(c, 0x001c_83cc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw(info.wrapping_add(4));
            measure_line(c, callee::MEASURE_PLAIN, out, 0x001c_843c)?;
        } else {
            c.cycles += 2;
            if v0 == 0 {
                c.g[V0][0] = 2;
                v0 = 2;
            } else {
                c.cycles += 2;
                if sel == 0 {
                    frame.restore(c, 16, 0x90);
                    // case 0: two zero words.
                    c.cycles += 2;
                    c.sw((out as u32).wrapping_add(0xc), 0);
                    c.sw((out as u32).wrapping_add(8), 0);
                    c.cycles += 1;
                    frame.restore(c, 17, 0x80);
                    return epilogue(c, frame);
                }
                c.cycles += 2;
                frame.restore(c, 17, 0x80);
                return epilogue(c, frame);
            }
            c.cycles += 2;
            let is_two = sel == c.g[V0][0];
            c.g[V0][0] = 3;
            if is_two {
                // case 2
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                callee::TEXT_OBJ.call(c, 0x001c_83e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(info.wrapping_add(4));
                measure_line(c, callee::MEASURE_WRAP, out, 0x001c_847c)?;
            } else {
                c.cycles += 2;
                let is_three = sel == c.g[V0][0];
                frame.restore(c, 16, 0x90);
                if !is_three {
                    c.cycles += 2;
                    frame.restore(c, 17, 0x80);
                    return epilogue(c, frame);
                }
                // case 3: look the description up in the world's UI object.
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.lw(info.wrapping_add(4));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let f = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                crate::call::call_ptr(c, f, 0x001c_840c)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    // sprintf(buf, "%d", info[1]) then measure wrapped.
                    c.cycles += 9;
                    c.g[A2][0] = c.lw(info.wrapping_add(4));
                    c.g[A1][0] = 0x0046_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
                    callee::SPRINTF.call(c, 0x001c_8458)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::TEXT_OBJ.call(c, 0x001c_8460)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    measure_line(c, callee::MEASURE_WRAP, out, 0x001c_847c)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    callee::TEXT_OBJ.call(c, 0x001c_8420)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    measure_line(c, callee::MEASURE_PLAIN, out, 0x001c_843c)?;
                }
            }
        }

        frame.restore(c, 16, 0x90);
        c.cycles += 1;
        frame.restore(c, 17, 0x80);
        epilogue(c, frame)
    })
}

/// Measure one line into `out`: `f(g, str, out, 0, g->size.x, g->size.y)`.
fn measure_line(c: &mut Cpu, f: Callee, out: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 7;
    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
    c.g[A2][0] = out;
    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
    c.g[A3][0] = 0;
    f.call(c, ret)
}

/// The shared tail of [`cfepopupconfirm_measure_desc`].
fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 5;
    frame.restore(c, 18, 0x70);
    frame.restore(c, 19, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(())
}

/// `void RiderResetSub_Enter(RiderResetSub *self)` (0x12f230): clear the
/// reset flag and the animation's +0x90 word; for the local player, stop a
/// running transition and start a white fade from two 1.0-alpha effects.
pub fn rider_reset_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        c.g[A1][0] = 2;
        frame.save(c, 16, 0x70);
        frame.save(c, 20, 0x30);
        let this64 = abi::arg64(c, 0);
        c.g[S0][0] = this64;
        frame.save(c, 17, 0x60);
        let this = this64 as u32;
        c.g[S4][0] = sx(this.wrapping_add(4));
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0x80);
        c.sw(this.wrapping_add(4), 0);
        c.g[V0][0] = c.lw(this.wrapping_add(8));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        callee::ANIM_GET_LAYER_SEQUENCE.call(c, 0x0012_f26c)?;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);

        c.g[A0][0] = c.g[S0][0];
        callee::IS_LOCAL_PLAYER.call(c, 0x0012_f278)?;
        let not_local = c.g[V0][0] == 0;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !not_local {
            c.cycles += 8;
            c.g[V0][0] = c.lw(this.wrapping_add(8));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x870));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            callee::GET_TRANSITION_MAN.call(c, 0x0012_f290)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
            let idle = c.g[V0][0] == 0;
            if !idle {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                callee::TRANSITION_STOP.call(c, 0x0012_f2a8)?;
            }

            c.cycles += 51;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20); // f20 = 1.0
            c.g[A0][0] = 0x14;
            callee::EFFECT_ALLOC.call(c, 0x0012_f2b8)?;
            c.g[S2][0] = c.g[V0][0];
            // The white colour {1, 1, 1, 1}.
            for off in [0u32, 4, 8, 0xc] {
                c.swc1(20, frame.at(c, off));
            }
            c.g[A0][0] = c.g[S2][0];
            callee::EFFECT_CTOR.call(c, 0x0012_f2d4)?;
            c.g[S0][0] = 0x0049_0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7f40));
            c.sw(c.g[S2][0] as u32, c.g[S0][0]);
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[SP][0] as u32).wrapping_add(8));
            c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0xb));
            c.sdr(2, (c.g[S2][0] as u32).wrapping_add(4));
            c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0x13));
            c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0xc));

            c.g[A0][0] = 0x14;
            callee::EFFECT_ALLOC.call(c, 0x0012_f308)?;
            c.g[S1][0] = c.g[V0][0];
            for off in [0x1c, 0x10, 0x14, 0x18] {
                c.swc1(20, frame.at(c, off));
            }
            c.g[A0][0] = c.g[S1][0];
            callee::EFFECT_CTOR.call(c, 0x0012_f324)?;

            // The start call: 0.5 fade, 0.0 delay, 0.5 out.
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S3][0];
            c.sw(c.g[S1][0] as u32, c.g[S0][0]);
            c.g[A2][0] = c.g[S2][0];
            c.mtc1(0, 13);
            c.g[A3][0] = c.g[S1][0];
            c.g[T1][0] = c.g[S4][0];
            c.g[A1][0] = 1;
            c.mov_s(14, 12);
            c.ldl(10, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.ldr(10, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.ldl(11, (c.g[SP][0] as u32).wrapping_add(0x1f));
            c.ldr(11, (c.g[SP][0] as u32).wrapping_add(0x18));
            c.sdl(10, (c.g[S1][0] as u32).wrapping_add(0xb));
            c.sdr(10, (c.g[S1][0] as u32).wrapping_add(4));
            c.sdl(11, (c.g[S1][0] as u32).wrapping_add(0x13));
            c.sdr(11, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[T0][0] = 0;
            callee::TRANSITION_START.call(c, 0x0012_f374)?;
        }

        c.cycles += 9;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// The movie object's vtable slot 0x98/0x9c call shared by the save modes:/// `a2` (the movie object) and `a1` (the step argument) are already set.
fn movie_finish_step(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
    let f = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    crate::call::call_ptr(c, f, ret)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, fill, put_words, seed_count, Rng, DATA, GAME_GP};

    /// The zeroed BELibrary dummy.
    const BE: u32 = DATA + 0x3_0000;
    /// The dummy world (and its nested objects).
    const WORLD: u32 = DATA + 0x8_0000;

    /// Assert no differences at all (hard, caller-saved and LO/HI).
    fn diff(gen: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(gen, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (caller-saved): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc).
    fn belib(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BE);
    }

                            #[test]
    fn ov_screen_message_center_ctor_matches() {
        diff(ee::s04::f_001e3c00__generated, ov_screen_message_center_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x1e_3c00, 4);
            belib(c);
            let this = DATA + 0x1_0000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x2_0000);
            c.g[A2][0] = r.next_u32() as u64;
            c.bus.write32(this + 0x10, DATA + 0x2_1000);
            c.bus.write32(GAME_GP - 0x1024, if seed % 2 == 0 { u32::MAX } else { 0 });
            c.bus.write32(0x004a_2088, r.next_u32());
        });
    }

    #[test]
    fn lobbyman_edit_account_matches() {
        diff(ee::s06::f_0025bf18__generated, lobbyman_edit_account, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x25_bf18, 4);
            belib(c);
            let this = DATA + 0x1_0000;
            let msg = DATA + 0x2_0000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(msg);
            c.bus.write32(this + 0x48, r.next_u32());
            c.bus.write32(this + 0x4c, r.next_u32());
            let head = DATA + 0x4_0000;
            c.bus.write32(this + 0xf0, head);
            c.bus.write32(head, head);
            c.bus.write32(head + 4, head);
            c.bus.write32(msg + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(msg + 4, r.next_u32());
            c.bus.write32(msg + 0x48, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(msg + 0x38, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(msg + 0x3c, if seed % 7 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(msg + 0x4c, if seed % 4 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(msg + 0xc, r.next_u32() & 0xffff);
        });
    }

            #[test]
    fn ssx_audio_system_pa_say20c5_matches() {
        diff(ee::s08::f_002a3708__generated, ssx_audio_system_pa_say20c5, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_3708, 4);
            belib(c);
            let audio = DATA + 0x1_0000;
            let rider = DATA + 0x2_0000;
            c.g[A0][0] = sx(audio);
            c.g[A1][0] = sx(rider);
            // Channel 3's volume: 0.0 on even seeds (speech off), 1.0 otherwise.
            c.bus
                .write32(audio + 0x62bc + 3 * 16, if seed % 2 == 0 { 0 } else { 0x3f80_0000 });
            // The replay/race-type/rider predicates stay false.
            c.bus.write32(audio + 0x6294, 0);
            c.bus.write32(audio + 0x530, 0);
            c.bus.write32(rider + 0x6c0, DATA + 0x3_0000);
            c.bus.write32(DATA + 0x3_0000 + 0x38, 0);
            let _ = r.next_u32();
        });
    }

        #[test]
    fn rider_reset_sub_enter_matches() {
        diff(ee::s00::f_0012f230__generated, rider_reset_sub_enter, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x12_f230, 4);
            belib(c);
            let this = DATA + 0x1_0000;
            let anim = DATA + 0x2_0000;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 8, anim);
            c.bus.write32(anim + 0x784, DATA + 0x2_1000);
            // The layer sequence object's +0x90 is written.
            c.bus.write32(DATA + 0x2_1000, DATA + 0x2_2000);
            // IsLocalPlayer: present and id >= 0 on two seeds in three.
            c.bus.write32(anim + 0x87c, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(anim + 0x870, if seed % 6 < 3 { 0x100 } else { 0xffff_ffff });
            c.bus.write32(GAME_GP - 0x848, WORLD);
            let game = WORLD + 0x1000;
            c.bus.write32(WORLD + 0x84, game);
            let transition = DATA + 0x4_0000;
            c.bus.write32(game + 0xc, transition);
            c.bus.write32(transition + 0x44, if seed % 2 == 0 { 0 } else { 1 });
            let _ = r.next_u32();
        });
    }
}
