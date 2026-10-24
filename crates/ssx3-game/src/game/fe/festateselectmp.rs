//! `src/fe/festateselectmp.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::festaterewards2::init_state;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovstates::at;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_3090,
        fe_screen_sel_multi_init,
        "FEScreenSelMulti_Init",
    ),
    (
        0x001a_30f8,
        fe_screen_sel_multi_on_widget_created,
        "FEScreenSelMulti_OnWidgetCreated",
    ),
    (
        0x001a_3160,
        fescreen_sel_multi_on_widget_action,
        "FEScreenSelMulti_OnWidgetAction",
    ),
    (
        0x001a_3218,
        fe_state_mp_circuit_ctor,
        "cFEStateMPCircuit_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (the generated name of each address, so a later hand-written
// replacement of a callee is reached through its override slot too).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
}

/// `cFEStateMPCircuit* cFEStateMPCircuit_ctor(cFEStateMPCircuit*)` (0x1A3218):
/// the UI-state constructor, vtable `0x469178` and state id 0x17.
pub fn fe_state_mp_circuit_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x001a_322c)?;
        init_state(c, this, 0x0046_9178, 0x17);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelMulti_Init(this)` 0x1a3090: add the multi-rider selection
/// screen and play its first frame.
pub fn fe_screen_sel_multi_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(0x0046_13b8);
        c.g[S1][0] = lw_at(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x001a_30b4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001a_30c8)?;
        let screen = c.g[V0][0];
        Addr(p(this, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001a_30e0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelMulti_OnWidgetCreated(self, item)` 0x1a30f8: map the widget's
/// id to the player slot (0 or 1) or leave it untouched.
pub fn fe_screen_sel_multi_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        let item = abi::arg(c, 1);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx(0x004a_14f8);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(item);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw_at(c, item, 0x38);
        call_ptr(c, 0x0031_7670, 0x001a_311c)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        let mut matched_second = false;
        if !first {
            c.cycles += 2;
            Addr(p(item, 0x18)).sw(c, 0);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw_at(c, item, 0x38);
            c.g[A0][0] = sx(0x004a_1500);
            call_ptr(c, 0x0031_7670, 0x001a_3138)?;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 1;
                Addr(p(item, 0x18)).sw(c, c.g[V0][0]);
                matched_second = true;
            }
        }
        if !first || matched_second {
            c.cycles += 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared frame epilogue: `ld ra, off(sp)`, close the frame and return.
#[inline(always)]
pub(crate) fn epilogue(c: &mut Cpu, off: u32, size: u32) {
    c.g[RA][0] = c.ld(at(c.g[SP][0] as u32, off));
    close(c, size);
}

// ---------------------------------------------------------------------------
// FE widget messages.

/// `void FEScreenSelMulti_OnWidgetAction(FEScreenSelMulti* self, FEItem* item,
/// int msg)` (0x1a3160): on message 5/6 forward the item's action id (+0x18)
/// to the screen object's virtual handler (slot +0x20 or +0x28) and, when it
/// returns a state, push that state onto the state stack at `self->0x10+0x18`.
pub fn fescreen_sel_multi_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        c.sq(16, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let item = c.g[V1][0];
        let none = item == 0;
        c.g[S0][0] = c.g[A0][0];
        if none {
            c.cycles += 1;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 5;
        let is5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 6;
        if !is5 {
            c.cycles += 2;
            let is6 = c.g[A2][0] == c.g[V0][0];
            if !is6 {
                c.cycles += 2;
                c.lq(16, at(c.g[SP][0] as u32, 0x10));
                c.cycles += 3;
                epilogue(c, 0, 0x20);
                return Ok(());
            }
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.cycles += 10;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = Addr(item as u32).off(0x18).lw(c);
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001a_31f4)?;
            let null = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !null {
                c.cycles += 3;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x001a_3208)?;
            }
            c.cycles += 1;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = Addr(item as u32).off(0x18).lw(c);
        c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001a_31b8)?;
        let null = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[V0][0];
        if null {
            c.cycles += 1;
            c.lq(16, at(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x001a_31cc)?;
        c.lq(16, at(c.g[SP][0] as u32, 0x10));
        c.cycles += 3;
        epilogue(c, 0, 0x20);
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
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::init_screen_test;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn fe_state_mp_circuit_ctor_matches() {
        check(
            ssx3_ee::s03::f_001a3218__generated,
            fe_state_mp_circuit_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1a32_18, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn fe_screen_sel_multi_init_matches() {
        init_screen_test(
            ee::s03::f_001a3090__generated,
            fe_screen_sel_multi_init,
            0x1a3090,
        );
    }

    #[test]
    fn fe_screen_sel_multi_on_widget_created_matches() {
        check(
            ee::s03::f_001a30f8__generated,
            fe_screen_sel_multi_on_widget_created,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1a30f8, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                // "a" hashes to 0x61, "b" to 0x62.
                c.bus.write8(0x004a_14f8, b'a');
                c.bus.write8(0x004a_14f9, 0);
                c.bus.write8(0x004a_1500, b'b');
                c.bus.write8(0x004a_1501, 0);
                w_2(
                    c,
                    OBJ2 + 0x38,
                    match seed % 3 {
                        0 => 0x61,
                        1 => 0x62,
                        _ => 0x99,
                    },
                );
                w_2(c, OBJ2 + 0x18, 0x55);
            },
        );
    }

    // ------------------------------------------------------- FE screen select.

    #[test]
    fn fescreen_sel_multi_on_widget_action_matches() {
        check(
            ee::s03::f_001a3160__generated,
            fescreen_sel_multi_on_widget_action,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a_3160, 2);
                let this = DATA;
                let item = DATA + 0x1000;
                let obj = DATA + 0x2000;
                let vt = DATA + 0x2400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 8 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = sx((4 + seed % 3) as u32);
                c.bus.write32(this + 0x10, obj);
                c.bus.write32(item + 0x18, r.next_u32());
                c.bus.write32(obj, vt);
                c.bus.write16(vt + 0x20, 0);
                c.bus.write32(vt + 0x24, FAKE);
                c.bus.write16(vt + 0x28, 0);
                c.bus.write32(vt + 0x2c, FAKE + 0x10);
                c.bus.write32(obj + 0x18, 0);
                c.bus.write32(obj + 0x1c, 0);
            },
        );
    }
}
