//! `src/main/gamerender.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2 as sp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_e730, game_view_man_dtor, "GameViewMan_dtor"),
    (
        0x0022_e7c8,
        game_view_man_reset_cameras,
        "GameViewMan_ResetCameras",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
}

/// `void GameViewMan_ResetCameras(self)` (0x22e7c8): call the reset method of
/// each active view.
pub fn game_view_man_reset_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sq(16, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = lw_r(c, S2, 0x10);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x4));
            c.g[V1][0] = Addr(low(c, S0)).lw(c);
            loop {
                c.cycles += 12;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                c.g[A1][0] = lw_r(c, V1, 0xa0);
                c.g[V0][0] = lw_r(c, A1, 0x14);
                c.g[A0][0] = lh_r(c, V0, 0x20);
                c.g[V1][0] = lw_r(c, V0, 0x24);
                let target = low(c, V1);
                c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                call_ptr(c, target, 0x0022_e818)?;
                c.g[V0][0] = lw_r(c, S2, 0x10);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(low(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e7f8);
                }
            }
        }
        c.cycles += 6;
        c.lq(16, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `GameViewMan_dtor` (0x22e730): destroy every view in the manager's list
/// through its virtual destructor and optionally free the manager.
pub fn game_view_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        c.g[S3][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[S0][0] = sx(this.wrapping_add(4));
            loop {
                c.cycles += 3;
                let entry = Addr(r(c, S0)).lw(c);
                c.g[A2][0] = entry;
                c.g[A1][0] = 3;
                if entry != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(entry as u32, 0x90)).lw(c);
                    c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x0022_e780);
                    c.g[A0][0] = sx((entry as u32).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0022_e780)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S2)).lw(c);
                c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e760);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let free_self = c.g[V0][0] != 0;
        Addr(r(c, S2)).off(0x10).sw(c, 0);
        if free_self {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0022_e7a8);
            c.g[A0][0] = c.g[S2][0];
            callee::BUILTIN_DELETE.call(c, 0x0022_e7a8)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
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
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::h;
    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn game_view_man_reset_cameras_matches() {
        check(
            ee::s05::f_0022e7c8__generated,
            game_view_man_reset_cameras,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22e7c8, 1);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                let n = seed % 3;
                w(c, obj + 0x10, n);
                for i in 0..n {
                    let v = DATA + 0x1000 + 0x100 * i;
                    let o = DATA + 0x1400 + 0x100 * i;
                    w(c, obj + 4 + 4 * i, v);
                    w(c, v + 0xa0, o);
                    w(c, o + 0x14, DATA + 0x3000);
                    slot(c, DATA + 0x3000, 0x20, 0x30, FAKE + 0x40 * i);
                    w(c, v + 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_view_man_dtor_matches() {
        check(
            ee::s05::f_0022e730__generated,
            game_view_man_dtor,
            SEEDS_2,
            &alloc_base_setup(|c, seed, r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x60) } else { DATA };
                let count = seed % 3;
                w(c, this, count);
                for i in 0..count {
                    let entry = if (seed >> (i + 1)) & 1 == 0 {
                        0
                    } else {
                        DATA + 0x800 + 0x40 * i
                    };
                    if entry != 0 {
                        let vt = vtable(c, r, &[8]);
                        w(c, entry + 0x90, vt);
                    }
                    w(c, this + 4 + 4 * i, entry);
                }
                w(c, this + 0x10, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }
}
