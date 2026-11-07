//! `src/fe/fepopup2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::gp_18 as gp;
pub(crate) use crate::game::fe::fepopup::widget_v24;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::movie::movieplayer::lw_at;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_9db8,
        c_fe_state_net_help_popup_init,
        "cFEStateNetHelpPopup_Init",
    ),
    (
        0x001d_9e20,
        c_fe_state_net_help_popup_v24,
        "cFEStateNetHelpPopup_v24",
    ),
    (
        0x001d_9e68,
        cfe_state_net_help_popup_update,
        "cFEStateNetHelpPopup_Update",
    ),
    (
        0x001d_a478,
        c_fe_state_net_help_popup_on_widget_action,
        "cFEStateNetHelpPopup_OnWidgetAction",
    ),
    (
        0x001d_a4a8,
        c_fe_state_net_help_popup_clear_fields,
        "cFEStateNetHelpPopup_ClearFields",
    ),
];

/// Cross-shard calls the generator emitted as `call_indirect` (no alias).
pub(crate) mod table {

    pub(crate) use crate::game::movie::movieplayer::table::UI_LIST_SET_FIRST_FLAGS;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

/// `cFEStateNetHelpPopup_OnWidgetAction(this, a1, msg)` 0x1da478: on a
/// "confirm" (message 6) put the popup's first-list entry into the pressed
/// state.
pub fn c_fe_state_net_help_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 6;
        let confirm = c.g[A2][0] == 6;
        frame.save_ra(c, 0);
        if confirm {
            c.cycles += 4;
            let this = abi::arg(c, 0);
            let obj = lw_at(c, this as u64, 0x10);
            c.g[A0][0] = obj;
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((obj as u32).wrapping_add(0x18));
            call_ptr(c, table::UI_LIST_SET_FIRST_FLAGS, 0x001d_a498)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup_v24`: the net-help popup's virtual method 24
/// wrapper, with its "popup open" flag at `+0x50`.
pub fn c_fe_state_net_help_popup_v24(c: &mut Cpu) {
    guest(c, |c| widget_v24(c, 0x50, 0x001d_9e54))
}

// ---------------------------------------------------------------------------
// Front-end state initialisers: `StrHash` of a screen name, create the
// screen through the UI engine, store it and start its thread.

pub(crate) fn fe_state_screen_init(c: &mut Cpu, hash: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    let this = c.g[A0][0];
    c.g[S0][0] = this;
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[A0][0] = sx(hash);
    c.g[S1][0] = lw(c, p(this as u32, 0x10));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = this;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_7cc0, rets[1])?;
    let no_screen = c.g[V0][0] == 0;
    sw(c, p(this as u32, 0x40), c.g[V0][0]);
    if !no_screen {
        c.cycles += 4;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0039_c870, rets[2])?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateNetHelpPopup_Init(cFEStateNetHelpPopup* self)` (0x1d9db8).
pub fn c_fe_state_net_help_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_79a0, [0x001d_9ddc, 0x001d_9df0, 0x001d_9e08])
    })
}

/// `cFEStateNetHelpPopup_ClearFields(cFEStateNetHelpPopup* self)` (0x1da4a8):
/// clear the popup's state words, its three counters, and its seven
/// `{a = 0; b = -1}` pairs (0x68 bytes apart).
pub fn c_fe_state_net_help_popup_clear_fields(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0] as u32;
    sw(c, p(this, 0x48), 0);
    c.g[V1][0] = 2;
    sw(c, p(this, 0x54), 0);
    c.g[V0][0] = sx(p(this, 0x60));
    loop {
        c.cycles += 7;
        sw(c, c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_a4b8);
        }
    }
    c.cycles += 5;
    sw(c, p(this, 0x64), 0);
    c.g[V0][0] = sx(p(this, 0x68));
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(this, 0x84));
    c.g[V1][0] = 6;
    loop {
        c.cycles += 7;
        sw(c, c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        sw(c, c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_a4e8);
        }
    }
}

/// `cFEStateNetHelpPopup_Update(this)` (0x1d9e68): if neither help-popup slot
/// has its binding pressed, close the popup.
pub fn cfe_state_net_help_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x60);
        frame.save_fpr(c, 20, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, 19, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        c.mtc1(0, 20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);

        loop {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xb0).lw(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !absent {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A1][0] = c.g[S3][0];
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = 0;
                }
            }

            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 6;
                c.g[A1][0] = 0x99;
                callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x001d_9ed4)?;
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.g[S1][0] = 1;
                }
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            }

            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_9ea0);
            }
        }

        c.cycles += 2;
        let found = c.g[S1][0] != 0;
        c.lq(16, frame.at(c, 0x40));
        if !found {
            c.cycles += 5;
            c.g[A0][0] = Addr(this).off(0x10).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001d_9f0c)?;
            c.lq(16, frame.at(c, 0x40));
        }

        c.cycles += 7;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby menu pages.

/// `void LobbyMenu_InitDeletePersonaPage(LobbyMenu* self, void* info)`
/// (0x1dabd0): build the delete-persona page: a label item and a spacer, then
/// fill the four persona action items and add them all to the menu.
pub fn lobby_menu_init_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        lobby_menu_init_page(
            c,
            3,
            0x46d5e0,
            0x467a98,
            4,
            0x001d_abd0,
            0x001d_ac00,
            0x001d_ac28,
            0x001d_ac34,
            0x001d_ac44,
            0x001d_ac60,
            0x001d_ac74,
            0x001d_ac88,
        )
    })
}

/// `void LobbyMenu_InitSelectRoomPage(LobbyMenu* self, void* info)`
/// (0x1dae20): as 0x1dabd0 with twelve room items.
pub fn lobby_menu_init_select_room_page(c: &mut Cpu) {
    guest(c, |c| {
        lobby_menu_init_page(
            c,
            11,
            0x46d4d8,
            0x467ab8,
            12,
            0x001d_ae20,
            0x001d_ae50,
            0x001d_ae78,
            0x001d_ae84,
            0x001d_ae94,
            0x001d_aeb0,
            0x001d_aec4,
            0x001d_aed8,
        )
    })
}

/// The shared body of the two page initialisers: `items` action items at
/// `self+0x168` and `count` insert-item calls.
#[allow(clippy::too_many_arguments)]
pub(crate) fn lobby_menu_init_page(
    c: &mut Cpu,
    items: u32,
    vtable: u32,
    label: u32,
    count: u32,
    _entry: u32,
    ret_ctor: u32,
    ret_label: u32,
    ret_spacer: u32,
    ret_action: u32,
    ret_insert1: u32,
    ret_insert2: u32,
    ret_insertn: u32,
) -> Flow {
    c.cycles += 26;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S1, 0x40);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x50);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save(c, S4, 0x10);
    c.g[S2][0] = sx(p(r(c, S1), 0x168));
    frame.save_ra(c, 0);
    c.g[S3][0] = items as u64;
    call_ptr(c, 0x002c_aa58, ret_ctor)?;

    c.g[S4][0] = u64::MAX;
    c.g[V0][0] = vtable as u64;
    c.g[A1][0] = label as u64;
    c.sw(Addr(r(c, S1)).off(0x12c).0, c.g[V0][0]);
    c.g[A0][0] = sx(p(r(c, S1), 0x134));
    c.g[V0][0] = Addr(r(c, S0)).lw(c);
    c.sw(Addr(r(c, S1)).off(0x130).0, c.g[V0][0]);
    call_ptr(c, 0x002c_d2a0, ret_label)?;
    c.g[A0][0] = sx(p(r(c, S1), 0x150));
    c.g[A1][0] = 0xe;
    call_ptr(c, 0x002c_d350, ret_spacer)?;

    'items: {
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx(r(c, S3).wrapping_sub(0x1));
            call_ptr(c, 0x002c_cf08, ret_action)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = sx(p(r(c, S2), 0x24));
            if !more {
                break 'items;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_ac38);
            }
        }
    }

    c.cycles += 11;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = sx(p(r(c, S1), 0x134));
    c.g[A2][0] = u64::MAX;
    c.g[S0][0] = sx(p(r(c, S1), 0x168));
    call_ptr(c, 0x002c_bcc8, ret_insert1)?;

    c.g[S2][0] = count as u64;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = sx(p(r(c, S1), 0x150));
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_insert2)?;

    c.g[A1][0] = c.g[S0][0];
    loop {
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        c.g[S0][0] = sx(p(r(c, S0), 0x24));
        call_ptr(c, 0x002c_bcc8, ret_insertn)?;
        c.g[S2][0] = sx(r(c, S2).wrapping_sub(0x1));
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[A1][0] = c.g[S0][0];
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_ac78);
        }
    }

    c.cycles += 9;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD;
    pub(crate) use crate::game::fe::fepopup::tests::popup_v24_machine;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    #[test]
    fn c_fe_state_net_help_popup_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001da478__generated,
            c_fe_state_net_help_popup_on_widget_action,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xac, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = if seed % 3 == 0 { 6 } else { 7 };
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_v24_matches() {
        check(
            ee::s04::f_001d9e20__generated,
            c_fe_state_net_help_popup_v24,
            120,
            &|c, seed| {
                popup_v24_machine(c, seed, 0x1d9e_20, 0x50);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_init_matches() {
        check(
            ssx3_ee::s04::f_001d9db8__generated,
            c_fe_state_net_help_popup_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d9db8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_clear_fields_matches() {
        check(
            ssx3_ee::s04::f_001da4a8__generated,
            c_fe_state_net_help_popup_clear_fields,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1da4a8);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x60u32 / 4 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_state_net_help_popup_update_matches() {
        check(
            ee::s04::f_001d9e68__generated,
            cfe_state_net_help_popup_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d9e, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                for i in 0..2u32 {
                    if seed % (i + 2) != 0 {
                        let bind = DATA + 0x2000 + 0x40 * i;
                        c.bus.write32(WORLD + 0xb0 + 4 * i, bind);
                        c.bus.write32(bind, DATA + 0x2400 + 0x40 * i);
                        c.bus.write32(DATA + 0x2400 + 0x40 * i, (seed % 2) as u32);
                    }
                }
                let _ = &mut r;
            },
        );
    }
}
