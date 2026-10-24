//! `src/fe/ovstates5.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_3990,
        event_select_load_map_image,
        "EventSelect_LoadMapImage",
    ),
    (
        0x0020_3cb8,
        event_select_set_map_tab_mode,
        "EventSelect_SetMapTabMode",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
}

/// `void EventSelect_SetMapTabMode(EventSelect* self, int mode)` (0x203cb8):
/// store the map-tab mode and label the tab from the new mode.
pub fn event_select_set_map_tab_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = 0x470000;
        let this = c.g[S0][0];
        c.sw(this as u32, c.g[A1][0]);
        c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xab8));
        call_ptr(c, callee::STR_HASH, 0x0020_3cdc)?;
        c.g[A0][0] = lw_at(c, this, 0x2e8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_3ce8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = lw_at(c, this, 0);
        let mode_zero = c.g[V0][0] == 0;
        c.g[A0][0] = 0x470000;
        if mode_zero {
            c.cycles += 6;
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xad8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d20)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d2c)?;
            c.lq(16, at(sp0, 0x20));
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xac8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d04)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d10)?;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void EventSelect_LoadMapImage(EventSelect *self)` (0x203990): request the
/// selected event's map texture and latch which variant was loaded.
pub fn event_select_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        'b_203a44: {
            'b_203a40: {
                'b_2039f0: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                    if t {
                        break 'b_2039f0;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d4));
                        c.g[S2][0] = 0x1;
                        if c.g[V0][0] == c.g[S2][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_203a44;
                        } else {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x002039d4);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a38c0, 0x002039d4)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x002039e4);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x001a36e8, 0x002039e4)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), c.g[S2][0]);
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), 0u64);
                            if t {
                                break 'b_203a40;
                            } else {
                                break 'b_2039f0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                if t {
                    break 'b_203a40;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    if t {
                        break 'b_203a40;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        if c.g[V1][0] == c.g[V0][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_203a44;
                        } else {
                            c.cycles += 12;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x00203a1c);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a38c0, 0x00203a1c)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x00203a30);
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x001a3768, 0x00203a30)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
                            break 'b_203a40;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_203a44;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    use ssx3_ee as ee;

    #[test]
    fn event_select_set_map_tab_mode_matches() {
        check(
            ee::s05::f_00203cb8__generated,
            event_select_set_map_tab_mode,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x203cb8, 4);
                let screen = DATA + 0x1000;
                let node = screen_with(c, &mut r, screen, 0);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus
                    .write32(DATA + 0x2e8, if seed % 4 == 0 { 0 } else { screen });
                let _ = node;
            },
        );
    }

    #[test]
    fn event_select_load_map_image_matches() {
        strict(
            ee::s05::f_00203990__generated,
            event_select_load_map_image,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2039);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(gp_at(-0xb54), [-1i32, 0, 2, 3][(seed % 4) as usize] as u32);
                c.bus.write32(gp_at(-0xb4c), (seed % 5) as u32);
                c.bus.write32(DATA + 0x2d4, (seed % 7) as u32);
                c.bus.write32(DATA + 0x2d8, seed as u32);
                c.bus.write32(DATA + 0x2dc, 0);
                c.bus.write32(W + 0x11c, DATA + 0x3000);
            },
        );
    }
}
