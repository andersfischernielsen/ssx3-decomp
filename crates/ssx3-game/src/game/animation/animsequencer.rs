//! `src/animation/animsequencer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::Flow;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::abi::r32;
use crate::call::sp;
use ssx3_ee as ee;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_3d28, anim_sequence_set_track_weight, "AnimSequence_SetTrackWeight"),
    (0x0031_3d40, anim_sequence_set_track_loop, "AnimSequence_SetTrackLoop"),
    (0x0031_42e8, anim_seq_list_init, "AnimSeqList_Init"),
    (0x0031_4368, anim_seq_list_append, "AnimSeqList_Append"),
    (0x0031_4418, anim_seq_list_collect_tracks, "AnimSeqList_CollectTracks"),
    (0x0031_4978, mdf_file_ctor, "MdfFile_ctor"),
    (0x0031_4988, mdf_file_dtor, "MdfFile_dtor"),
    (0x0031_49d0, mdf_file_free, "MdfFile_Free"),
    (0x0031_4aa0, mdf_file_parse_header, "MdfFile_ParseHeader"),
    (0x0031_4c00, mdf_file_create_model_by_name, "MdfFile_CreateModelByName"),
];

// ---------------------------------------------------------------- AnimSequence

/// Direct-call targets of the animation functions (shard s10).
pub(crate) mod anim_callee {
    use super::Callee;
    use ssx3_ee::s10 as ee;
    pub(crate) use crate::game::animation::animmodel::anim_callee::FIRE_TRIGGERS_AT_TIME;
    pub(crate) use crate::game::animation::animmodel::anim_callee::FIRE_END_TRIGGERS;
    pub(crate) use crate::game::animation::animmodel::anim_callee::FIRE_TRIGGERS_IN_RANGE;
    pub(crate) use crate::game::animation::animmodel::anim_callee::GET_TRACK_BLEND_WEIGHT;
    pub(crate) use crate::game::animation::animmodel::anim_callee::ALLOC_CHAN_CACHE;
    pub(crate) use crate::game::animation::animmodel::anim_callee::DECODE_CHAN_CACHE;
    pub(crate) use crate::game::animation::animmodel::anim_callee::EVAL_CHAN_CURVES;
}

/// `AnimSequence` layout used here (tracks of 0x1c bytes from +0).
pub(crate) mod seq {

    pub const TRACK_SIZE: u32 = 0x1c;

    /// Track fields: anim id, time, rate, length, loop mode (+0x1c, the
    /// word after the track's active flag).
    pub const T_ANIM: u32 = 0x4;

    pub const T_TIME: u32 = 0x8;

    pub const T_RATE: u32 = 0xc;

    pub const T_LENGTH: u32 = 0x10;

    pub const T_ACTIVE: u32 = 0x18;

    pub const T_MODE: u32 = 0x1c;

    /// Sequence fields.
    pub const BLEND_A: u32 = 0x60;

    pub const BLEND_B: u32 = 0x70;

    pub const LAYER: u32 = 0x80;

    pub const FLAGS: u32 = 0x84;

    pub const USER: u32 = 0x88;

    pub const RATE: u32 = 0x90;

    pub const FIRED_MASK: u32 = 0xb8;

    pub const END_FLAG: u32 = 0xc0;

    pub const DIRTY: u32 = 0xc4;

    pub const NEXT: u32 = 0xc8;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
}


// ---------------------------------------------------------------- anim sequence

/// Size of one animation track (`AnimTrack`).
pub(crate) const TRACK_SIZE: u32 = 0x1c;

/// `AnimTrack+0x14`: the track's blend weight.
pub(crate) const TRACK_WEIGHT: u32 = 0x14;

/// `AnimTrack+0x1c`: the track's loop flag.
pub(crate) const TRACK_LOOP: u32 = 0x1c;


/// `void AnimSequence_SetTrackWeight(AnimSequence *seq, int track, float w)`
/// (0x313d28): set track `track`'s blend weight (`seq + track*0x1c + 0x14`)
/// to `w`. The index multiply also sets `LO`/`HI`.
pub fn anim_sequence_set_track_weight(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = TRACK_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 1), TRACK_SIZE);
    c.g[V1][0] = sx(offset);
    let seq = abi::arg(c, 0);
    let track = Addr(seq.wrapping_add(offset));
    c.g[A0][0] = sx(track.0);
    track.off(TRACK_WEIGHT).write_fpu(c, abi::farg(c, 0));
    abi::ret(c);
}


/// `void AnimSequence_SetTrackLoop(AnimSequence *seq, int track, uint loop)`
/// (0x313d40): store the loop flag at `seq + track*0x1c + 0x1c`. The index
/// multiply also sets `LO`/`HI`.
pub fn anim_sequence_set_track_loop(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = TRACK_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 1), TRACK_SIZE);
    c.g[V1][0] = sx(offset);
    let seq = abi::arg(c, 0);
    let track = Addr(seq.wrapping_add(offset));
    c.g[A0][0] = sx(track.0);
    track.off(TRACK_LOOP).sw(c, c.g[A2][0]);
    abi::ret(c);
}


/// `void AnimSeqList_Init(AnimSeqList *this)` (0x3142e8): clear the head and
/// count.
pub fn anim_seq_list_init(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, 0);
    abi::ret(c);
}


/// `MdfFile *MdfFile_ctor(MdfFile *this)` (0x314978): clear the header word.
pub fn mdf_file_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}


/// `void MdfFile_ParseHeader(MdfFile *this, const u8 *data)` (0x314aa0):
/// store the data pointer and resolve the geometry/vertex sections.
pub fn mdf_file_parse_header(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    let data = c.g[A1][0] as u32;
    Addr(this).off(0x4).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(data).off(0x6).lh(c);
    c.g[V0][0] = sx(data.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0x8).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(data).off(0x8).lw(c);
    c.g[A1][0] = sx(data.wrapping_add(c.g[V1][0] as u32));
    Addr(this).off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// AnimSeqList.

/// `AnimSeqList` layout.
pub(crate) const LIST_HEAD: u32 = 0x4;

pub(crate) const SEQ_NEXT: u32 = 0xc8;


/// `void AnimSeqList_Append(AnimSeqList *list, AnimSequence *seq)`
/// (0x314368): append `seq` to the singly-linked list at +0x4 (following
/// +0xc8), or set it as the head when the list is empty; clear the new node's
/// link and bump the count at +0. Back-edge event check at 0x314388.
pub fn anim_seq_list_append(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_4388;
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let seq = abi::arg(c, 1);
    let head = list.off(LIST_HEAD).lw(c);
    c.g[V0][0] = head;
    c.g[V1][0] = head;
    if head == 0 {
        c.cycles += 1;
        list.off(LIST_HEAD).sw(c, seq as u64);
    } else {
        c.cycles += 3;
        let first_link = Addr(head as u32).off(SEQ_NEXT).lw(c);
        c.g[V0][0] = first_link;
        if first_link == 0 {
            Addr(head as u32).off(SEQ_NEXT).sw(c, seq as u64);
        } else {
            c.cycles += 2;
            let mut node = first_link;
            c.g[V1][0] = node;
            loop {
                c.cycles += 7;
                let link = Addr(node as u32).off(SEQ_NEXT).lw(c);
                c.g[V0][0] = link;
                if link != 0 {
                    node = link;
                    c.g[V1][0] = link;
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                } else {
                    c.cycles += 2;
                    Addr(node as u32).off(SEQ_NEXT).sw(c, seq as u64);
                    break;
                }
            }
        }
    }
    c.cycles += 5;
    Addr(seq).off(SEQ_NEXT).sw(c, 0);
    let count = list.lw(c) as u32;
    c.g[V0][0] = sx(count.wrapping_add(1));
    list.sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `int AnimSeqList_CollectTracks(AnimSeqList* list, TrackSample* out)`
/// (0x314418): for every sequence in the list (`+4` first, linked by
/// `+0xc8`) and each of its three tracks that is active, append a 0x50-byte
/// sample: anim id, time, the sequence's layer word (+0x84 → +8), the track
/// blend weight (`AnimSequence_GetTrackBlendWeight`, +0xc), the sequence's
/// user doubleword (+0x88 → +0x10), its two blend quadwords (+0x60/+0x70 →
/// +0x20/+0x30) and +0x80 (→ +0x40). Returns the number of samples.
///
/// The loop state lives in s0..s7 as in the original (event checks at
/// 0x314468 per track and 0x314458 per sequence).
pub fn anim_seq_list_collect_tracks(c: &mut Cpu) {
    const TRACK_HEAD: u32 = 0x0031_4468;
    const SEQ_HEAD: u32 = 0x0031_4458;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x70);
        // s6 = count, s7 = out, s1 = sequence, s5 = out offset of the
        // sequence's first sample, s4 = track, s2/s3 = the track's time /
        // active field, s0 = next sample.
        c.g[S6][0] = 0;
        c.g[S7][0] = c.g[A1][0];
        let first = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[S1][0] = first;
        if first != 0 {
            c.cycles += 2;
            c.g[S5][0] = 0;
            c.g[S4][0] = 0;
            loop {
                c.cycles += 4;
                let s = c.g[S1][0] as u32;
                c.g[S2][0] = sx(s.wrapping_add(seq::T_TIME));
                c.g[S3][0] = sx(s.wrapping_add(seq::T_ACTIVE));
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                loop {
                    c.cycles += 3;
                    let s = c.g[S1][0] as u32;
                    let active = Addr(c.g[S3][0] as u32).lw(c);
                    c.g[V0][0] = active;
                    c.g[A0][0] = c.g[S1][0];
                    if active != 0 {
                        c.cycles += 20;
                        let time_at = Addr(c.g[S2][0] as u32);
                        let out = Addr(c.g[S0][0] as u32);
                        let anim = time_at.off(0u32.wrapping_sub(seq::T_TIME - seq::T_ANIM)).lw(c);
                        c.g[V0][0] = anim;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x50));
                        out.sw(c, anim);
                        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                        let time = time_at.read_fpu(c);
                        c.f[0] = time.0;
                        out.off(4).write_fpu(c, time);
                        c.lq(V0, s.wrapping_add(seq::BLEND_A));
                        c.sq(V0, out.0.wrapping_add(0x20));
                        c.lq(V1, s.wrapping_add(seq::BLEND_B));
                        c.sq(V1, out.0.wrapping_add(0x30));
                        let layer = Addr(s).off(seq::LAYER).lw(c);
                        c.g[V0][0] = layer;
                        out.off(0x40).sw(c, layer);
                        let flags = Addr(s).off(seq::FLAGS).lw(c);
                        c.g[V1][0] = flags;
                        out.off(8).sw(c, flags);
                        anim_callee::GET_TRACK_BLEND_WEIGHT.call(c, 0x0031_44b4)?;
                        let weight = FpuF32(c.f[0]);
                        out.off(0xc).write_fpu(c, weight);
                        let user = Addr(s).off(seq::USER).ld(c);
                        c.g[V0][0] = user;
                        out.off(0x10).sd(c, user);
                        c.g[S0][0] = sx(out.0.wrapping_add(0x50));
                    }
                    c.cycles += 5;
                    let track = (c.g[S4][0] as u32).wrapping_add(1);
                    c.g[S4][0] = sx(track);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(seq::TRACK_SIZE));
                    let more = (c.g[S4][0] as i64) < 3;
                    c.g[V0][0] = more as u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(seq::TRACK_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, TRACK_HEAD);
                    }
                }
                c.cycles += 3;
                let next = Addr(c.g[S1][0] as u32).off(seq::NEXT).lw(c);
                c.g[S1][0] = next;
                c.g[S4][0] = 0;
                if next == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SEQ_HEAD);
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S6][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Foo_dtor(void* self, int flags)` (0x30 frame): run the destructor
/// `base`, then `__builtin_delete(self)` when bit 0 of `flags` is set.
///
/// The original reloads `s0` (`lq 16, 0x20(sp)`) *before* the delete branch, so
/// the delete call already sees the caller's `s0` — and so does its own frame
/// save. Reproduce that order.
pub(crate) fn dtor(c: &mut Cpu, base: Callee, ret_base: u32, ret_delete: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    base.call(c, ret_base)?;
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] == 0;
    frame.restore(c, S0, 0x20);
    if !keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        DELETE.call(c, ret_delete)?;
        frame.restore(c, S0, 0x20);
    }
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    c.cycles += 4;
    frame.close(c);
    Ok(())
}


// ---------------------------------------------------------------------------
// Destructor thunks and vtable forwarders.

/// `void MdfFile_dtor(MdfFile* f, int flags)` (0x314988): free the file and,
/// when `flags & 1`, the object.
pub fn mdf_file_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(c, callee::MDF_FILE_FREE, 0x0031_49a4, 0x0031_49b8)
    })
}


/// `void MdfFile_Free(MdfFile* f)` (0x3149d0): release the buffer (`+0`) and
/// zero the four list fields.
pub fn mdf_file_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(r32(c, S0));
        if c.g[A0][0] == 0 {
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        } else {
            c.cycles += 3;
            callee::EA_MEM_FREE.call(c, 0x0031_49f4)?;
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        }
        c.cycles += 7;
        c.sw(r32(c, S0), 0);
        c.sw(r32(c, S0).wrapping_add(4), 0);
        c.sw(r32(c, S0).wrapping_add(8), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `MdfFile_CreateModelByName` (0x314c00): find the model whose name matches
/// `a1` (case-insensitive) in the file's 0x60-byte records and create it.
pub fn mdf_file_create_model_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(this, 4)).lw(c);
        c.g[V0][0] = Addr(p(r(c, V1), 4)).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 6;
                c.g[A1][0] = Addr(p(this, 8)).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0031_4c48);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                call_ptr(c, callee::STRCASECMP, 0x0031_4c48)?;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(p(this, 4)).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0031_4c5c);
                    c.g[A1][0] = c.g[S0][0];
                    callee::MDF_FILE_CREATE_MODEL.call(c, 0x0031_4c5c)?;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V0][0] = Addr(p(r(c, V1), 4)).lh(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(0x60));
                if !more {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4c38);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::ptr as ptr_2;
    pub(crate) use crate::game::visualfx::worldlightman::tests::f;
    pub(crate) use crate::game::visualfx::worldlightman::tests::pick;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_4 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;


    #[test]
    fn anim_sequence_set_track_weight_matches() {
        check(s10::f_00313d28__generated, anim_sequence_set_track_weight, SEEDS, &|c, seed| {
            let mut r = rng(seed, 6);
            scramble(c, &mut r);
            let seq = ptr_2(&mut r, 0);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = (r.next_u32() % 6) as u64;
            c.f[12] = r.edge_f32();
        });
    }


    #[test]
    fn anim_sequence_set_track_loop_matches() {
        check(s10::f_00313d40__generated, anim_sequence_set_track_loop, SEEDS, &|c, seed| {
            let mut r = rng(seed, 7);
            scramble(c, &mut r);
            let seq = ptr_2(&mut r, 0);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = (r.next_u32() % 6) as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(r.next_u32()) };
        });
    }


    #[test]
    fn anim_seq_list_init_matches() {
        check(ssx3_ee::s10::f_003142e8, anim_seq_list_init, 100, &|c, seed| {
            let mut r = Rng(0x200d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }


    #[test]
    fn mdf_file_ctor_matches() {
        check(ssx3_ee::s10::f_00314978, mdf_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x200e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }


    #[test]
    fn mdf_file_parse_header_matches() {
        check(ssx3_ee::s10::f_00314aa0, mdf_file_parse_header, 200, &|c, seed| {
            let mut r = Rng(0x200f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write16(DATA + 0x106, (r.next_u32() & 0xffff) as u16);
            c.bus.write32(DATA + 0x108, r.next_u32());
        });
    }


    #[test]
    fn anim_seq_list_append_matches() {
        check_events(
            ssx3_ee::s10::f_00314368,
            anim_seq_list_append,
            80,
            &|c, seed| {
                let mut r = Rng(0x3143_6800 ^ seed as u64);
                scramble(c, &mut r);
                let list = DATA;
                c.g[A0][0] = list as u64;
                let seq = DATA + 0x800;
                c.g[A1][0] = seq as u64;
                // Always three or more links so every loop reaches the
                // back-edge event check.
                let n = 3 + (r.next_u32() % 3);
                let mut node = DATA + 0x1000;
                c.bus.write32(list + LIST_HEAD, node);
                for i in 0..n {
                    c.bus.write32(node + SEQ_NEXT, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(list, r.next_u32());
                c.bus.write32(seq + SEQ_NEXT, r.next_u32());
                let mode = schedule(c, &mut r, seed, 0x0031_4368);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn anim_seq_list_append_short_matches() {
        check(ssx3_ee::s10::f_00314368, anim_seq_list_append, 80, &|c, seed| {
            let mut r = Rng(0x3143_68f0 ^ seed as u64);
            scramble(c, &mut r);
            let list = DATA;
            c.g[A0][0] = list as u64;
            let seq = DATA + 0x800;
            c.g[A1][0] = seq as u64;
            let n = r.next_u32() % 4;
            let mut node = DATA + 0x1000;
            c.bus.write32(list + LIST_HEAD, if n == 0 { 0 } else { node });
            for i in 0..n {
                c.bus.write32(node + SEQ_NEXT, if i + 1 < n { node + 0x10 } else { 0 });
                node += 0x10;
            }
            c.bus.write32(list, r.next_u32());
            c.bus.write32(seq + SEQ_NEXT, r.next_u32());
        });
    }


    // ------------------------------------------------------------ AnimSequence

    /// A sequence at DATA (random fields, no triggers so the trigger
    /// callees take their short paths).
    pub(crate) fn sequence(c: &mut Cpu, r: &mut Rng, s: u32) {
        fill(c, r, s, 0x34);
        c.bus.write32(s + 0xac, 0);
        for k in 0..3 {
            let t = s + k * 0x1c;
            c.bus.write32(t + 0x8, r.f32(4.0));
            c.bus.write32(t + 0x10, f(pick(r, &[0.5, 1.0, 2.0, 3.0])));
            c.bus.write32(t + 0x14, r.f32(1.0));
            c.bus.write32(t + 0x18, if r.next_u32() % 3 == 0 { 0 } else { 1 });
        }
        c.bus.write32(s + 0x94, r.f32(1.0));
        c.bus.write32(s + 0xa4, f(pick(r, &[0.0, 0.25, 0.5])));
        c.bus.write32(s + 0xa8, f(pick(r, &[0.0, 0.25, 0.5])));
    }


    #[test]
    fn anim_seq_list_collect_tracks_matches() {
        check(ee::s10::f_00314418__generated, anim_seq_list_collect_tracks, 200, &|c, seed| {
            let mut r = Rng(0xc011_ec75 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let list = DATA;
            let out = DATA + 0x8000;
            let mut link = list + 4;
            for k in 0..seed % 4 {
                let s = DATA + 0x1000 + k * 0x400;
                sequence(c, &mut r, s);
                c.bus.write32(link, s);
                link = s + 0xc8;
            }
            c.bus.write32(link, 0);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(out);
        });
    }


    #[test]
    fn mdf_file_dtor_matches() {
        check(ee::s10::f_00314988__generated, mdf_file_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x31_4988, 0);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj, if seed % 3 == 0 { DATA + 0x2000 } else { 0 });
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
            let _ = r.next_u32();
        });
    }


    #[test]
    fn mdf_file_free_matches() {
        check(ee::s10::f_003149d0__generated, mdf_file_free, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x31_49d0, 0);
            let obj = DATA + 0x1000;
            c.bus.write32(
                obj,
                match seed % 3 {
                    0 => 0,
                    1 => DATA + 0x2000,
                    _ => 4,
                },
            );
            c.bus.write32(obj + 0xc, r.next_u32());
            c.g[A0][0] = sx(obj);
        });
    }


    #[test]
    fn mdf_file_create_model_by_name_matches() {
        check(
            ee::s10::f_00314c00__generated,
            mdf_file_create_model_by_name,
            SEEDS_2,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let records = DATA + 0x1000;
                let names = DATA + 0x1100;
                let count = seed % 3;
                w(c, this + 4, records);
                wh(c, records + 4, count as u16);
                w(c, this + 8, names);
                let query = DATA + 0x9000;
                let words: [&[u8]; 3] = [b"alpha", b"beta", b"gamma"];
                put_str(c, query, words[(seed % 3) as usize]);
                for i in 0..count {
                    let s = names + 0x60 * i;
                    put_str(c, s, words[i as usize]);
                    if seed & 1 == 0 && i == 0 {
                        put_str(c, s, words[(seed % 3) as usize]);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = query as u64;
            }),
        );
    }
}
