//! `src/fe/ovstates.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::dirtysock::tagsunk::const_reg;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::fe_ctor_with_vtable;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::festatebonusmaterial::FE_UI_STATE_CTOR;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festatecharsetup::oc437_fe_state_dtor;
pub(crate) use crate::game::fe::festatetrophyroom::init_state;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_3;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
pub(crate) use crate::game::sound::soundsys::p as p_5;
pub(crate) use crate::game::wscript::wscriptman::p as p_2;
use crate::guest::Addr;
use crate::tick;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_4778, c_fe_state_title_init, "cFEStateTitle_Init"),
    (
        0x0019_47f8,
        cfe_state_title_transition_in,
        "cFEStateTitle_TransitionIn",
    ),
    (0x0019_48a8, cfestatetitle_update, "cFEStateTitle_Update"),
    (
        0x0019_4988,
        c_fe_state_title_on_widget_created,
        "cFEStateTitle_OnWidgetCreated",
    ),
    (
        0x0019_49e0,
        c_fe_state_title_on_widget_action,
        "cFEStateTitle_OnWidgetAction",
    ),
    (
        0x0019_4a48,
        cfe_state_title_on_widget_notify,
        "cFEStateTitle_OnWidgetNotify",
    ),
    (
        0x0019_4a60,
        fe_state_main_menu_ctor,
        "cFEStateMainMenu_ctor",
    ),
    (
        0x0019_4d18,
        cfe_state_main_menu_on_widget_created,
        "cFEStateMainMenu_OnWidgetCreated",
    ),
    (
        0x0019_4dd0,
        cfestatemainmenu_update,
        "cFEStateMainMenu_Update",
    ),
    (
        0x0019_52e8,
        fe_state_main_menu_on_widget_notify,
        "cFEStateMainMenu_OnWidgetNotify",
    ),
    (
        0x0019_5498,
        fe_state_bonus_material_ctor,
        "cFEStateBonusMaterial_ctor",
    ),
    (0x001d_e6c8, fe_state_title_dtor, "cFEStateTitle_dtor"),
    (
        0x001d_e728,
        fe_state_main_menu_dtor,
        "cFEStateMainMenu_dtor",
    ),
    (0x001d_e690, c_fe_state_title_ctor_2, "cFEStateTitle_ctor_2"),
    (
        0x001d_e6f0,
        c_fe_state_main_menu_ctor_2,
        "cFEStateMainMenu_ctor_2",
    ),
    (
        0x0019_4bf0,
        cfe_state_main_menu_on_render,
        "cFEStateMainMenu_OnRender",
    ),
    (
        0x0019_4aa8,
        cfe_state_main_menu_init,
        "cFEStateMainMenu_Init",
    ),
    (
        0x0019_4ea8,
        main_menu_on_widget_action,
        "cFEStateMainMenu_OnWidgetAction",
    ),
    (
        0x0019_5328,
        main_menu_on_child_event,
        "cFEStateMainMenu_OnChildEvent",
    ),
    (
        0x0019_4980,
        c_fe_state_title_on_child_event_00194980,
        "cFEStateTitle_OnChildEvent",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_CTOR_2;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::UI_STATE_CTOR;

    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateBonusMaterial* cFEStateBonusMaterial_ctor(cFEStateBonusMaterial*)`
/// (0x195498): the shared UI-state constructor, then vtable `0x46b918` at +0x8
/// and state id 5 at +0xC.
pub fn fe_state_bonus_material_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0019_54ac)?;
        init_state(c, this, 0x0046_b918, 5);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMainMenu* cFEStateMainMenu_ctor(self)` (0x194a60): UI state
/// constructor, then the main-menu vtable/state and two cleared fields.
pub fn fe_state_main_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR_2, 0x0019_4a74)?;
        let this = r32(c, S0);
        c.g[V0][0] = 2;
        const_reg(c, V1, 0x47_0000, -0x4618);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x48, 0);
        sw(c, this, 0x4c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateTitle_OnWidgetCreated(cFEStateTitle* self, UIObject* item)`
/// (0x194988): when the widget is the title screen's, clear its user data.
pub fn c_fe_state_title_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4a_17d0; // D_004A17D0
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let item = r32(c, A1);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(p_2(item, 0x38)).lw(c);
        call_ptr(c, table::STR_HASH, 0x0019_49ac)?;
        let other = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !other {
            c.cycles += 6;
            Addr(p_2(item, 0x18)).sw(c, 0);
            c.g[A0][0] = sx(item);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_OBJECT_SET_USER_DATA.call(c, 0x0019_49c8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateTitle_OnWidgetNotify(State*, void*, int key)` (0x00194a48):
/// the title accepts every key but reports 0x101 only for the start widget.
pub fn cfe_state_title_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x6;
    c.g[V1][0] = 0x101;
    c.g[V0][0] = if c.g[A2][0] == 0 { 0x101 } else { 0x100 };
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end widget notifications.

/// `int cFEStateMainMenu_OnWidgetNotify(cFEStateMainMenu* self, void* widget,
/// int id, void* data)` (0x1952E8): a `Back`/`Continue` widget reports whether
/// the selection it points at (`+0x48`) is null.
pub fn fe_state_main_menu_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    if c.g[A2][0] == c.g[V0][0] {
        c.g[A0][0] = c.lw(ea(c.g[A0][0], 0x48));
    } else {
        c.cycles += 3;
        c.g[V0][0] = 9;
        let is_nine = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = WIDGET_IGNORED;
        if is_nine {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        // (the original's `li t, 1; beqz` tail always returns here)
        c.cycles += 4;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V1][0] = WIDGET_IGNORED;
    c.g[V0][0] = 1;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `cFEStateTitle_OnWidgetAction(cFEStateTitle* self, Widget* w, int type)`
/// (0x1949e0): on a widget action of type 5, let the first UI object handle
/// the message and push the resulting state.
pub fn c_fe_state_title_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 5;
        frame.save_ra(c, 0);
        let action = c.g[A2][0] != c.g[V0][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        if !action {
            c.cycles += 11;
            c.g[V0][0] = lw(c, p_3(this as u32, 0x10));
            c.g[A1][0] = this;
            c.g[A2][0] = lw(c, p_3(c.g[V1][0] as u32, 0x18));
            c.g[A3][0] = lw(c, c.g[V0][0] as u32);
            c.g[V1][0] = lw(c, p_3(c.g[A3][0] as u32, 4));
            c.g[A0][0] = lh(c, p_3(c.g[V1][0] as u32, 0x20));
            c.g[V0][0] = lw(c, p_3(c.g[V1][0] as u32, 0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_4a20)?;
            let handled = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !handled {
                c.cycles += 3;
                c.g[A0][0] = lw(c, p_3(this as u32, 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x0019_4a34)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state changes.

/// `void cFEStateTitle_Init(cFEStateTitle* st)` (0x00194778): clear the
/// autosave counter, create the title screen from the screen engine named by
/// `st+0x10`, start it, then set the FE music screen.
pub fn c_fe_state_title_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = sx(0x004a_17c8); // "cFEStateTitle"
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0019_47a0)?; // StrHash
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_47b4)?; // UIEngine_CreateScreen
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_47cc)?; // UIScreen_StartThread
        }
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x0019_47d4)?; // SSXAudioSystem_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_f140, 0x0019_47e0)?; // SSXAudioSystem_SetFEMusicScreen
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// The pre-fade wait of the title screen: `D_004A17C0` (`gp-0x1930`) starts at
/// 300 (5 s at 60 Hz) in the game's data and is decremented once per update by
/// `cFEStateTitle_TransitionIn` only (Ghidra: the only references to
/// `0x4a17c0` are the load and the store in that function).
#[cfg(test)]
const TRANSIENT_WAIT_FRAMES: u32 = 300;

/// `bool cFEStateTitle_TransitionIn(self)` (0x1947f8): count the transition
/// in over a few frames, then fade in the title's two text widgets.
pub fn cfe_state_title_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p_4(r32(c, GP), 0xffff_e6d0)).lw(c);
        let mut fade = c.g[V0][0] == 0;
        if !fade {
            c.cycles += 3;
            // The countdown is a wall-clock wait (5 s at 60 Hz). The data
            // initializer is 300 frames; `tick::on_vblank` rewrites it to
            // 300 * rate / 60 ticks once, when it applies the rate (see
            // `tick::FRAME_COUNTS`), so the countdown is the original one.
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let still = c.g[V0][0] != 0;
            c.sw(p_4(r32(c, GP), 0xffff_e6d0), c.g[V0][0]);
            if still {
                // one more frame to wait
                c.cycles += 1;
                c.g[V0][0] = 0;
                frame.restore(c, S0, 0x10);
                c.cycles += 4;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            fade = true;
        }
        if fade {
            // create the screen and fade its title texts in
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
            call_ptr(c, 0x0031_7670, 0x0019_4828)?;
            c.g[A0][0] = Addr(p_4(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_4834)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = 0x1;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_4(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x38)).lh(c);
                c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x3c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_4854)?;
            }
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17d0));
            call_ptr(c, 0x0031_7670, 0x0019_4860)?;
            c.g[A0][0] = Addr(p_4(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_486c)?;
            c.g[A2][0] = c.g[V0][0];
            let missing = c.g[A2][0] == 0;
            c.g[A1][0] = 0x1;
            if !missing {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_4(c.g[A2][0] as u32, 8)).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 0x48)).lh(c);
                c.g[V1][0] = Addr(p_4(c.g[V0][0] as u32, 0x4c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_488c)?;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `a + off`.
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `addiu sp, sp, -n`.
#[inline(always)]
pub(crate) fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}

/// `addiu sp, sp, n; jr ra` (`ra` already reloaded by the caller).
#[inline(always)]
pub(crate) fn close(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(n));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// FE widget messages.

/// Hash of the string at `site` via the ELF hash, i.e. `GetHashValue32`.
pub(crate) fn hash_eq(c: &mut Cpu, item: u64, site: u32, ret: u32) -> Flow {
    c.g[S0][0] = Addr(item as u32).off(0x38).lw(c);
    c.g[A0][0] = sx(site);
    call_ptr(c, 0x0031_7670, ret)?;
    Ok(())
}

/// `void cFEStateMainMenu_OnWidgetCreated(cFEStateMainMenu* self,
/// FEItem* item)` (0x194d18): map the widget's hash id (+0x38) to a screen
/// action id (+0x18), comparing against five hash-keyed widgets.
pub fn cfe_state_main_menu_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[A0][0] = 0x460000;
        c.sq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let item = c.g[A1][0];
        c.g[S1][0] = item;
        c.sq(16, at(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = sx(0x0019_4d3c);
        c.g[S0][0] = Addr(item as u32).off(0x38).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_4d3c)?;
        let same = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = 0x460000;
        if same {
            c.cycles += 2;
            c.g[V0][0] = 0xb;
            c.cycles += 2;
            Addr(c.g[S1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
            c.lq(16, at(c.g[SP][0] as u32, 0x20));
            c.cycles += 4;
            c.lq(17, at(c.g[SP][0] as u32, 0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            close(c, 0x30);
            return Ok(());
        }
        for (site, id, ret) in [
            (0x0046_0060u32, 0x9u64, 0x0019_4d58u32),
            (0x0046_0040, 0x16, 0x0019_4d74),
            (0x0046_0070, 0x5, 0x0019_4d90),
            (0x004a_17d8, 0x31, 0x0019_4dac),
        ] {
            c.cycles += 5;
            hash_eq(c, item, site, ret)?;
            if c.g[S0][0] == c.g[V0][0] {
                if id == 0x31 {
                    c.cycles += 1;
                    c.g[V0][0] = 0x31;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = id;
                }
                c.cycles += 2;
                Addr(c.g[S1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
                c.lq(16, at(c.g[SP][0] as u32, 0x20));
                c.cycles += 4;
                c.lq(17, at(c.g[SP][0] as u32, 0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                close(c, 0x30);
                return Ok(());
            }
        }
        c.lq(16, at(c.g[SP][0] as u32, 0x20));
        c.cycles += 4;
        c.lq(17, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front end: title / main menu.

/// The title screen's idle timeout before the attract mode: 30 s, the
/// original's `self->timer > 0x708` in frames.
const TITLE_IDLE_FRAMES: u32 = 1800;

/// `void cFEStateTitle_Update(self)` (0x1948a8): tick the idle timer and drop
/// back to the attract mode after 30 seconds; any active list item resets it.
///
/// The idle timeout is a wall-clock 30 s (`self->timer > 0x708` in the C++,
/// i.e. 1800 frames at 60 Hz), not a fixed number of updates: the limit is
/// `ticks_from_frames60(1800)`, so the counter `timer + 1` continuing means
/// `timer + 1 <= 1800` (the original `timer + 1 < 1801`; the two forms agree
/// for every int32, and at 60 the limit is exactly 1800).
pub fn cfestatetitle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x0019_48bc)?;
        c.g[V0][0] = Addr(p_5(r32(c, S0), 0x48)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        let idle = tick::ticks_from_frames60(c, TITLE_IDLE_FRAMES) as i64;
        c.g[V1][0] = ((c.g[V0][0] as i64) <= idle) as u64;
        let ticking = c.g[V1][0] != 0;
        Addr(p_5(r32(c, S0), 0x48)).sw(c, c.g[V0][0]);
        if !ticking {
            c.cycles += 5;
            c.g[V0][0] = Addr(gp(c, -0x1724)).lw(c);
            Addr(p_5(r32(c, S0), 0x48)).sw(c, 0);
            c.g[V0][0] = c.g[V0][0] | 4;
            Addr(gp(c, -0x1724)).sw(c, c.g[V0][0]);
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A3][0] = 0;
        c.g[V1][0] = 0;
        c.g[T0][0] = 0x1c;
        c.g[T1][0] = sx(p_5(r32(c, V0), 0xb0));
        c.g[V0][0] = sx(r32(c, V1) << 2);
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[V0][0] = sx(p_5(r32(c, T1), r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, V1)).lw(c);
            c.g[A1][0] = Addr(r32(c, V1)).lw(c);
            if c.g[A1][0] == 0 {
                c.g[V1][0] = c.g[A2][0];
            } else {
                c.cycles += 1;
                c.g[A0][0] = 0;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.mult(0, 10, 4, 8);
                    let mut found = 0u64;
                    if out_of_range {
                        c.cycles += 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx(p_5(r32(c, T2), r32(c, V1)));
                        found = Addr(p_5(r32(c, V0), 0x10)).lw(c);
                    }
                    c.cycles += 2;
                    if found == 0 {
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_4920);
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A3][0] = 1;
                        break;
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    Addr(p_5(r32(c, S0), 0x48)).sw(c, 0);
                }
                c.cycles += 1;
                c.g[V1][0] = c.g[A2][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, V1) << 2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_4900);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMainMenu_Update(self)` (0x194dd0): select the two menu
/// columns and pick the first entry if both are valid.
pub fn cfestatemainmenu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0039_e510, 0x0019_4df0)?;
        c.g[V0][0] = Addr(p_5(r32(c, S1), 0x10)).lw(c);
        c.g[S0][0] = Addr(p_5(r32(c, V0), 0xc)).lw(c);
        Addr(p_5(r32(c, S1), 0x55)).sb(c, 0);
        c.g[V0][0] = Addr(p_5(r32(c, S0), 8)).lw(c);
        loop {
            c.cycles += 12;
            c.g[A1][0] = c.g[S2][0] & 0xff;
            c.g[A0][0] = Addr(p_5(r32(c, V0), 0x160)).lh(c);
            c.g[A2][0] = Addr(p_5(r32(c, V0), 0x164)).lw(c);
            let select = r32(c, A2);
            c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
            call_ptr(c, select, 0x0019_4e14)?;
            c.g[V1][0] = Addr(p_5(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p_5(r32(c, V1), 0x168)).lh(c);
            c.g[V0][0] = Addr(p_5(r32(c, V1), 0x16c)).lw(c);
            let valid = r32(c, V0);
            c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
            call_ptr(c, valid, 0x0019_4e28)?;
            let not_valid = c.g[V0][0] == 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            if !not_valid {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_5(r32(c, S1), 0x55)).lbu(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                Addr(p_5(r32(c, S1), 0x55)).sb(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p_5(r32(c, S0), 8)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_4e00);
            }
        }
        c.cycles += 9;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1398));
        call_ptr(c, 0x0031_7670, 0x0019_4e54)?;
        c.g[A0][0] = Addr(p_5(r32(c, S1), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4e60)?;
        c.g[A0][0] = c.g[V0][0];
        let no_menu = c.g[A0][0] == 0;
        c.g[V0][0] = 2;
        if no_menu {
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
            c.cycles += 1;
            c.lq(S1, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(p_5(r32(c, A0), 0x95)).lbu(c);
            let not_sel = c.g[V1][0] != c.g[V0][0];
            c.lq(S0, frame.at(c, 0x30));
            if not_sel {
                c.cycles += 1;
                c.lq(S1, frame.at(c, 0x20));
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(p_5(r32(c, S1), 0x55)).lb(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let few = c.g[V0][0] == 0;
                c.lq(S1, frame.at(c, 0x20));
                if !few {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0039_b960, 0x0019_4e90)?;
                    c.cycles += 1;
                    c.lq(S0, frame.at(c, 0x30));
                    c.cycles += 1;
                    c.lq(S1, frame.at(c, 0x20));
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMainMenu_dtor(this, flags)` (0x1de728).
pub fn fe_state_main_menu_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_e744)
}

/// `cFEStateTitle_dtor(this, flags)` (0x1de6c8).
pub fn fe_state_title_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_e6e4)
}

/// `cFEStateTitle` constructor (0x1de690): `UIState_ctor(this, 0)`, vtable at `+8`.
pub fn c_fe_state_title_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e6a8,
            0x4548,
        )
    });
}

/// `cFEStateMainMenu` constructor (0x1de6f0): `UIState_ctor(this, 0)`, vtable at `+8`.
pub fn c_fe_state_main_menu_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e708,
            0x4618,
        )
    });
}

/// `void cFEStateMainMenu_OnRender(cFEStateMainMenu* self)` (0x194bf0): looks
/// up the screen object (`UIScreen_FindObject(self->screen, StrHash(...))`,
/// +0x40), and when it exists and `self`'s mode bits (`lb +0x55 < 2`, then
/// the `& 0x3f == 5` test on the global at gp-0x704) say so, animates the
/// menu's two sub-objects through their vtables (the entries at +0xc8/+0xcc
/// and +0x40/+0x44 of the object's vtable), then the object's +0x58/+0x5c
/// slot, with the alpha-half show on the way out.
pub fn cfe_state_main_menu_on_render(c: &mut Cpu) {
    guest(c, cfe_state_main_menu_on_render_body);
}

pub(crate) fn cfe_state_main_menu_on_render_body(c: &mut Cpu) -> Flow {
    let frame = Frame::open(c, 0x40);
    'b_194cd0: {
        c.cycles += 14;
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x46_0000;
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x0031_7670, 0x0019_4c10)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4c1c)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
        if t {
            break 'b_194cd0;
        }
        'b_194c48: {
            c.cycles += 4;
            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x55));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x1d));
                break 'b_194c48;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0019_4498,
                f: ee::s02::f_00194498,
            }
            .call(c, 0x0019_4c40)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
            break 'b_194cd0;
        }
        c.cycles += 4;
        c.g[V1][0] = 0x5;
        c.g[V0][0] &= 0x3f;
        let t = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
        if t {
            break 'b_194cd0;
        }
        c.cycles += 30;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0019_4c70)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0019_4c88)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = c.g[SP][0];
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x23));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1c));
        c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x2b));
        c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x24));
        let sp_now = sp(c);
        c.sdl(2, sp_now.wrapping_add(0x7));
        c.sdr(2, sp_now);
        c.sdl(3, sp_now.wrapping_add(0xf));
        c.sdr(3, sp_now.wrapping_add(0x8));
        c.swc1(0, sp_now);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0019_4ccc)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
        break 'b_194cd0;
    }
    'b_194d04: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if t {
            break 'b_194d04;
        }
        c.cycles += 8;
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17d8));
        call_ptr(c, 0x0031_7670, 0x0019_4ce4)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4cf0)?;
        let t = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if t {
            break 'b_194d04;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0019_4498,
            f: ee::s02::f_00194498,
        }
        .call(c, 0x0019_4d00)?;
        frame.restore(c, S0, 0x30);
        break 'b_194d04;
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void cFEStateMainMenu::Init(cFEStateMainMenu* this)` (0x194aa8).
///
/// Front-end main menu initialisation: adds the "main menu" screen to the UI
/// engine (`this + 0x10`; 0x397cc0, screen stored at `this + 0x40`) and
/// plays its first frame (0x39c870), clears the `cBERules` words at 0x5308b8
/// (0x20 bytes) and the flags `+0x48`/`+0x54`, resets the network/game
/// systems (0x266cd8, 0x261008, 0x25b688, `cGameComm_destroy` 0x256c50,
/// `cCommSystem_destroy` 0x255898), submits back-end interface 7 of the world
/// (`gp - 0x848`), selects audio state 1 (0x28b180, 0x28f140), then, for the
/// controller ports 0 and 1, calls two virtual slots of the engine's input
/// object, counting in `this + 0x55` the ports whose second call succeeded.
pub fn cfe_state_main_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_194afc: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00194ad0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00194ae4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_194afc;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00194afc)?;
                break 'b_194afc;
            }
        }
        c.cycles += 37;
        c.g[A0][0] = 0x530000;
        c.g[A2][0] = 0x20;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x003e6448, 0x00194b10)?;
        c.g[S2][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
        call_ptr(c, 0x00266cd8, 0x00194b20)?;
        call_ptr(c, 0x00261008, 0x00194b28)?;
        call_ptr(c, 0x0025b688, 0x00194b30)?;
        call_ptr(c, 0x00256c50, 0x00194b38)?;
        call_ptr(c, 0x00255898, 0x00194b40)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x7;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014dd58, 0x00194b50)?;
        c.g[A0][0] = 0x530000;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4b30), 0u64);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00194b6c)?;
        call_ptr(c, 0x0028b180, 0x00194b74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0028f140, 0x00194b80)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sb((c.g[S1][0] as u32).wrapping_add(0x55), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        'l_194b90: loop {
            'b_194bd0: {
                c.cycles += 13;
                c.g[A1][0] = c.g[S2][0] & 0xff;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x164));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00194ba8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x168));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x16c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00194bbc)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                if t {
                    break 'b_194bd0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x55));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x55), c.g[V0][0]);
                    break 'b_194bd0;
                }
            }
            c.cycles += 2;
            if c.g[V1][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                if event_due(c) {
                    deliver_event_here(c, 0x00194b90);
                }
                continue 'l_194b90;
            } else {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void cFEStateMainMenu_OnWidgetAction(self, Widget* item, int event)`
/// (0x00194ea8): main menu events. 5 select: the item's `mMiscData` (jump
/// table at 0x4600b0, 5 to 49) sets the game path and the go-to-next flag
/// (49 also creates the online managers), then the next state is pushed; 6
/// back; 7 options; 8 the hidden cheat counter that reveals the three
/// extra menu items at 7.
pub fn main_menu_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00194ea8 {
        return main_menu_on_widget_action_resume(c);
    }
    guest(c, |c| 'l_194ea8: loop {
        'b_1952d4: {
            'b_1952d0: {
                'b_1952cc: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A1][0];
                    let t = c.g[S2][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1952cc;
                    } else {
                        'b_195208: {
                            'b_1951ac: {
                                'b_195188: {
                                    'b_19516c: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        if t {
                                            break 'b_19516c;
                                        } else {
                                            'b_194f08: {
                                                'b_194eec: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_194eec;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        if t {
                                                            break 'b_194f08;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_1952d4;
                                                            } else {
                                                                break 'b_194eec;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x7;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x8;
                                                if t {
                                                    break 'b_1951ac;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_195208;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_1952d4;
                                                        } else {
                                                            break 'b_194f08;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_194fd0: {
                                                c.cycles += 13;
                                                c.g[RA][0] = sx(0x00194f10);
                                                c.g[A0][0] = 0u64;
                                                call_ptr(c, 0x001a86e8, 0x00194f10)?;
                                                c.g[RA][0] = sx(0x00194f18);
                                                call_ptr(c, 0x0014dc80, 0x00194f18)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00194f24);
                                                c.g[A1][0] = 0u64;
                                                call_ptr(c, 0x0014dd58, 0x00194f24)?;
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                c.g[A0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x5));
                                                c.g[V1][0] = (c.g[A0][0] < 0x2du64) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[V0][0] = 0x460000;
                                                if t {
                                                    break 'b_194fd0;
                                                } else {
                                                    'b_194fcc: {
                                                        'b_194fc8: {
                                                            'b_194f9c: {
                                                                'b_194f88: {
                                                                    'b_194f68: {
                                                                        c.cycles += 6;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xb0,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x00194ea8 => {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x00194ea8);
                                                                                }
                                                                                continue 'l_194ea8;
                                                                            }
                                                                            0x00194f54 => {
                                                                                c.cycles += 5;
                                                                                c.g[A0][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x00194f60);
                                                                                c.g[A1][0] = 0u64;
                                                                                call_ptr(
                                                                                    c, 0x00145108,
                                                                                    0x00194f60,
                                                                                )?;
                                                                                let t = true;
                                                                                c.g[V0][0] = 0x1;
                                                                                if t {
                                                                                    break 'b_194fcc;
                                                                                } else {
                                                                                    break 'b_194f68;
                                                                                }
                                                                            }
                                                                            0x00194f68 => {
                                                                                break 'b_194f68;
                                                                            }
                                                                            0x00194f88 => {
                                                                                break 'b_194f88;
                                                                            }
                                                                            0x00194f9c => {
                                                                                break 'b_194f9c;
                                                                            }
                                                                            0x00194fc8 => {
                                                                                break 'b_194fc8;
                                                                            }
                                                                            0x00194fd0 => {
                                                                                break 'b_194fd0;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x00194ea8,
                                                                                    0x001952e4,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 8;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x00194f74);
                                                                    c.g[A1][0] = 0x1;
                                                                    call_ptr(
                                                                        c, 0x00145108, 0x00194f74,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lb((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x54));
                                                                    c.g[V1][0] = 0x1;
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x48),
                                                                        c.g[V1][0],
                                                                    );
                                                                    let t = true;
                                                                    c.g[V0][0] = ((c.g[V0][0]
                                                                        as i64)
                                                                        < 7i64)
                                                                        as u64;
                                                                    if t {
                                                                        break 'b_194fd0;
                                                                    } else {
                                                                        break 'b_194f88;
                                                                    }
                                                                }
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x00194f94);
                                                                c.g[A1][0] = 0x2;
                                                                call_ptr(
                                                                    c, 0x00145108, 0x00194f94,
                                                                )?;
                                                                let t = true;
                                                                c.g[V0][0] = 0x1;
                                                                if t {
                                                                    break 'b_194fcc;
                                                                } else {
                                                                    break 'b_194f9c;
                                                                }
                                                            }
                                                            c.cycles += 11;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x00194fa8);
                                                            c.g[A1][0] = 0x2;
                                                            call_ptr(c, 0x00145108, 0x00194fa8)?;
                                                            c.g[RA][0] = sx(0x00194fb0);
                                                            call_ptr(c, 0x00255860, 0x00194fb0)?;
                                                            c.g[RA][0] = sx(0x00194fb8);
                                                            call_ptr(c, 0x0025b650, 0x00194fb8)?;
                                                            c.g[RA][0] = sx(0x00194fc0);
                                                            call_ptr(c, 0x00260f80, 0x00194fc0)?;
                                                            c.g[RA][0] = sx(0x00194fc8);
                                                            call_ptr(c, 0x00266ca0, 0x00194fc8)?;
                                                            break 'b_194fc8;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x1;
                                                        break 'b_194fcc;
                                                    }
                                                    c.cycles += 1;
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0x48),
                                                        c.g[V0][0],
                                                    );
                                                    break 'b_194fd0;
                                                }
                                            }
                                            'b_195150: {
                                                c.cycles += 9;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00194fe4);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00194fe4)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                c.g[V0][0] = 0x16;
                                                if c.g[A0][0] != c.g[V0][0] {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                    break 'b_195150;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        break 'b_195150;
                                                    } else {
                                                        'b_1950a0: {
                                                            c.cycles += 35;
                                                            c.g[V0][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = 0x460000;
                                                            c.g[A3][0] = 0u64;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x80));
                                                            c.g[A0][0] = 0x4c;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00195020);
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x50),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x00317d70, 0x00195020)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x00195034);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            call_ptr(c, 0x0039e318, 0x00195034)?;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x50));
                                                            c.g[V1][0] = 0x470000;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V0][0] = 0x1;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x6ae8));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                << ((c.g[A1][0] as u32) & 31));
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V1][0],
                                                            );
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x15),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[RA][0] = sx(0x00195064);
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x48),
                                                                0u64,
                                                            );
                                                            call_ptr(c, 0x0039f290, 0x00195064)?;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x4c),
                                                                c.g[S2][0],
                                                            );
                                                            c.g[A0][0] = 0x4a0000;
                                                            c.g[RA][0] = sx(0x00195074);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x17e0));
                                                            call_ptr(c, 0x00317670, 0x00195074)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x00195080);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x00195080)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_1950a0;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001950a0);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x001950a0)?;
                                                                break 'b_1950a0;
                                                            }
                                                        }
                                                        'b_1950d8: {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[RA][0] = sx(0x001950ac);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xa0));
                                                            call_ptr(c, 0x00317670, 0x001950ac)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001950b8);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001950b8)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_1950d8;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001950d8);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x001950d8)?;
                                                                break 'b_1950d8;
                                                            }
                                                        }
                                                        'b_195110: {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = 0x4a0000;
                                                            c.g[RA][0] = sx(0x001950e4);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1458));
                                                            call_ptr(c, 0x00317670, 0x001950e4)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001950f0);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001950f0)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_195110;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x00195110);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x00195110)?;
                                                                break 'b_195110;
                                                            }
                                                        }
                                                        c.cycles += 9;
                                                        c.g[A0][0] = 0x4a0000;
                                                        c.g[RA][0] = sx(0x0019511c);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1460));
                                                        call_ptr(c, 0x00317670, 0x0019511c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00195128);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x00195128)?;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        let t = c.g[A2][0] == 0u64;
                                                        c.g[A1][0] = 0u64;
                                                        if t {
                                                            break 'b_1952cc;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x00195148);
                                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x00195148)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_1952d0;
                                                            } else {
                                                                break 'b_195150;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A2][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                            if t {
                                                break 'b_195188;
                                            } else {
                                                break 'b_19516c;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    break 'b_195188;
                                }
                                c.cycles += 4;
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00195190);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00195190)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_1952cc;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001951a4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001951a4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1952d0;
                                    } else {
                                        break 'b_1951ac;
                                    }
                                }
                            }
                            c.cycles += 23;
                            c.g[A1][0] = 0x460000;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                            c.g[A0][0] = 0x54;
                            c.g[RA][0] = sx(0x001951c4);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00317d70, 0x001951c4)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001951d4);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x001887a0,
                                f: ee::s02::cFEStateOptions_ctor,
                            }
                            .call(c, 0x001951d4)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = 0xffffffffffffc0ff;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A2][0] = 0u64;
                            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                            c.g[A3][0] = 0u64;
                            c.g[V1][0] = c.g[V1][0] | 0x100;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x00195200);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f320, 0x00195200)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1952d0;
                            } else {
                                break 'b_195208;
                            }
                        }
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x54));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
                        let t = c.g[V1][0] != 0u64;
                        c.sb((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                        if t {
                            break 'b_1952cc;
                        } else {
                            'b_19525c: {
                                c.cycles += 9;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x00195230);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                                call_ptr(c, 0x00317670, 0x00195230)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x0019523c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0019523c)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_19525c;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019525c);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0019525c)?;
                                    break 'b_19525c;
                                }
                            }
                            'b_195294: {
                                c.cycles += 9;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x00195268);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                call_ptr(c, 0x00317670, 0x00195268)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x00195274);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00195274)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_195294;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00195294);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00195294)?;
                                    break 'b_195294;
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = 0x4a0000;
                            c.g[RA][0] = sx(0x001952a0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17e8));
                            call_ptr(c, 0x00317670, 0x001952a0)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001952ac);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001952ac)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1952cc;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001952cc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001952cc)?;
                                break 'b_1952cc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1952d0;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1952d4;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `main_menu_on_widget_action` (converted from the `entries` body): any entry
/// other than 0x00194ea8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn main_menu_on_widget_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00194ea8 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A1][0];
                    let t = c.g[S2][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        pc = 0x001952cc;
                        continue;
                    }
                    pc = 0x00194ec8;
                    continue;
                }
                0x00194ec8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6;
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                    if t {
                        pc = 0x0019516c;
                        continue;
                    }
                    pc = 0x00194ed4;
                    continue;
                }
                0x00194ed4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00194eec;
                        continue;
                    }
                    pc = 0x00194edc;
                    continue;
                }
                0x00194edc => {
                    c.cycles += 2;
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00194f08;
                        continue;
                    }
                    pc = 0x00194ee4;
                    continue;
                }
                0x00194ee4 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x001952d4;
                        continue;
                    }
                    pc = 0x00194eec;
                    continue;
                }
                0x00194eec => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x7;
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[V0][0] = 0x8;
                    if t {
                        pc = 0x001951ac;
                        continue;
                    }
                    pc = 0x00194ef8;
                    continue;
                }
                0x00194ef8 => {
                    c.cycles += 2;
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00195208;
                        continue;
                    }
                    pc = 0x00194f00;
                    continue;
                }
                0x00194f00 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        pc = 0x001952d4;
                        continue;
                    }
                    pc = 0x00194f08;
                    continue;
                }
                0x00194f08 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x00194f10);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x001a86e8, 0x00194f10)?;
                    c.g[RA][0] = sx(0x00194f18);
                    call_ptr(c, 0x0014dc80, 0x00194f18)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00194f24);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00194f24)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5));
                    c.g[V1][0] = (c.g[A0][0] < 0x2du64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x00194fd0;
                        continue;
                    }
                    pc = 0x00194f3c;
                    continue;
                }
                0x00194f3c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00194f54 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00194f60);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00145108, 0x00194f60)?;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00194fcc;
                        continue;
                    }
                    pc = 0x00194f68;
                    continue;
                }
                0x00194f68 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00194f74);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x00145108, 0x00194f74)?;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x54));
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                    let t = true;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 7i64) as u64;
                    if t {
                        pc = 0x00194fd0;
                        continue;
                    }
                    pc = 0x00194f88;
                    continue;
                }
                0x00194f88 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00194f94);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00145108, 0x00194f94)?;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00194fcc;
                        continue;
                    }
                    pc = 0x00194f9c;
                    continue;
                }
                0x00194f9c => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00194fa8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00145108, 0x00194fa8)?;
                    c.g[RA][0] = sx(0x00194fb0);
                    call_ptr(c, 0x00255860, 0x00194fb0)?;
                    c.g[RA][0] = sx(0x00194fb8);
                    call_ptr(c, 0x0025b650, 0x00194fb8)?;
                    c.g[RA][0] = sx(0x00194fc0);
                    call_ptr(c, 0x00260f80, 0x00194fc0)?;
                    c.g[RA][0] = sx(0x00194fc8);
                    call_ptr(c, 0x00266ca0, 0x00194fc8)?;
                    pc = 0x00194fc8;
                    continue;
                }
                0x00194fc8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00194fcc;
                    continue;
                }
                0x00194fcc => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    pc = 0x00194fd0;
                    continue;
                }
                0x00194fd0 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00194fe4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00194fe4)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = 0x16;

                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        pc = 0x00195150;
                        continue;
                    }
                    pc = 0x00194ff4;
                    continue;
                }
                0x00194ff4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        pc = 0x00195150;
                        continue;
                    }
                    pc = 0x00195000;
                    continue;
                }
                0x00195000 => {
                    c.cycles += 35;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x460000;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x80));
                    c.g[A0][0] = 0x4c;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00195020);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                    call_ptr(c, 0x00317d70, 0x00195020)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00195034);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e318, 0x00195034)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = 0x470000;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ae8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A1][0] as u32) & 31));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x15), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00195064);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                    call_ptr(c, 0x0039f290, 0x00195064)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S2][0]);
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00195074);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17e0));
                    call_ptr(c, 0x00317670, 0x00195074)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00195080);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00195080)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001950a0;
                        continue;
                    }
                    pc = 0x0019508c;
                    continue;
                }
                0x0019508c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001950a0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001950a0)?;
                    pc = 0x001950a0;
                    continue;
                }
                0x001950a0 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001950ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
                    call_ptr(c, 0x00317670, 0x001950ac)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001950b8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001950b8)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001950d8;
                        continue;
                    }
                    pc = 0x001950c4;
                    continue;
                }
                0x001950c4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001950d8);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001950d8)?;
                    pc = 0x001950d8;
                    continue;
                }
                0x001950d8 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001950e4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1458));
                    call_ptr(c, 0x00317670, 0x001950e4)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001950f0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001950f0)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00195110;
                        continue;
                    }
                    pc = 0x001950fc;
                    continue;
                }
                0x001950fc => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00195110);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00195110)?;
                    pc = 0x00195110;
                    continue;
                }
                0x00195110 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x0019511c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1460));
                    call_ptr(c, 0x00317670, 0x0019511c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00195128);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00195128)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001952cc;
                        continue;
                    }
                    pc = 0x00195134;
                    continue;
                }
                0x00195134 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00195148);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00195148)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x001952d0;
                        continue;
                    }
                    pc = 0x00195150;
                    continue;
                }
                0x00195150 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    if t {
                        pc = 0x00195188;
                        continue;
                    }
                    pc = 0x0019516c;
                    continue;
                }
                0x0019516c => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    pc = 0x00195188;
                    continue;
                }
                0x00195188 => {
                    c.cycles += 4;
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00195190);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00195190)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x001952cc;
                        continue;
                    }
                    pc = 0x00195198;
                    continue;
                }
                0x00195198 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x001951a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f400, 0x001951a4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x001952d0;
                        continue;
                    }
                    pc = 0x001951ac;
                    continue;
                }
                0x001951ac => {
                    c.cycles += 23;
                    c.g[A1][0] = 0x460000;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                    c.g[A0][0] = 0x54;
                    c.g[RA][0] = sx(0x001951c4);
                    c.g[A2][0] = 0x100;
                    call_ptr(c, 0x00317d70, 0x001951c4)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001951d4);
                    c.g[A2][0] = 0x1;
                    Callee {
                        addr: 0x001887a0,
                        f: ee::s02::cFEStateOptions_ctor,
                    }
                    .call(c, 0x001951d4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0xffffffffffffc0ff;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                    c.g[A3][0] = 0u64;
                    c.g[V1][0] = c.g[V1][0] | 0x100;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00195200);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f320, 0x00195200)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x001952d0;
                        continue;
                    }
                    pc = 0x00195208;
                    continue;
                }
                0x00195208 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x54));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    if t {
                        pc = 0x001952cc;
                        continue;
                    }
                    pc = 0x00195224;
                    continue;
                }
                0x00195224 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00195230);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    call_ptr(c, 0x00317670, 0x00195230)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0019523c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0019523c)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0019525c;
                        continue;
                    }
                    pc = 0x00195248;
                    continue;
                }
                0x00195248 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019525c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0019525c)?;
                    pc = 0x0019525c;
                    continue;
                }
                0x0019525c => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00195268);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x00317670, 0x00195268)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00195274);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00195274)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00195294;
                        continue;
                    }
                    pc = 0x00195280;
                    continue;
                }
                0x00195280 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00195294);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00195294)?;
                    pc = 0x00195294;
                    continue;
                }
                0x00195294 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001952a0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17e8));
                    call_ptr(c, 0x00317670, 0x001952a0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001952ac);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001952ac)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001952cc;
                        continue;
                    }
                    pc = 0x001952b8;
                    continue;
                }
                0x001952b8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001952cc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001952cc)?;
                    pc = 0x001952cc;
                    continue;
                }
                0x001952cc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x001952d0;
                    continue;
                }
                0x001952d0 => {
                    c.cycles += 1;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x001952d4;
                    continue;
                }
                0x001952d4 => {
                    c.cycles += 4;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00194ea8, 0x001952e4);
                    return Ok(());
                }
            }
        }
    });
}

/// One "show widget" step of `cFEStateMainMenu_OnChildEvent`: look up the
/// string at `hi + lo` (`0x317670`), find the child widget of the state's
/// screen (`0x39d860`), and when found call its vtable slot at +0x4c with
/// `a1 = 1` (show).
pub(crate) fn oc851_show_widget(c: &mut Cpu, hi: u32, lo: u32, ras: [u32; 3]) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = hi as u64;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(lo));
    call_ptr(c, 0x0031_7670, ras[0])?;
    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ras[1])?;
    c.g[A2][0] = c.g[V0][0];
    let found = c.g[A2][0] != 0;
    c.g[A1][0] = 1;
    if found {
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
        let t = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, t, ras[2])?;
    }
    Ok(())
}

/// `void cFEStateMainMenu_onWidgetEvent(self, obj, int msg)` (0x195328):
/// message 0x15 does nothing; message 0x16 asks the widget (vtable +0xb8)
/// and, if it answers, copies the saved control (+0x50 into +0x14), sets
/// `mGoToNextState` (+0x48) and tells the multi-mode widget (+0x4c) to play
/// keyframe 2 and gets event 5; then it shows four widgets.
pub fn main_menu_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, 16, 0x10);
        c.g[V0][0] = 0x15;
        frame.save_ra(c, 0);
        let is_15 = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if is_15 {
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x16;
            if c.g[A2][0] != c.g[V0][0] {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 8;
                c.g[V1][0] = Addr(r32(c, A3).wrapping_add(8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0xb8)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xbc)).lw(c);
                let t = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0019_5368)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                if !none {
                    c.cycles += 13;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x50)).lb(c);
                    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x4c)).lw(c);
                    c.g[A1][0] = 2;
                    let saved = c.g[V1][0];
                    Addr(r32(c, S0).wrapping_add(0x14)).sb(c, saved);
                    let one = c.g[V0][0];
                    Addr(r32(c, S0).wrapping_add(0x48)).sw(c, one);
                    call_ptr(c, 0x0039_ffa0, 0x0019_5388)?;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                    c.g[A2][0] = 5;
                    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x4c)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x98)).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x9c)).lw(c);
                    let t = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    call_ptr(c, t, 0x0019_53a4)?;
                }
                oc851_show_widget(
                    c,
                    0x4a_0000,
                    0x17e0,
                    [0x0019_53b0, 0x0019_53bc, 0x0019_53dc],
                )?;
                oc851_show_widget(c, 0x46_0000, 0xa0, [0x0019_53e8, 0x0019_53f4, 0x0019_5414])?;
                oc851_show_widget(
                    c,
                    0x4a_0000,
                    0x1458,
                    [0x0019_5420, 0x0019_542c, 0x0019_544c],
                )?;
                oc851_show_widget(
                    c,
                    0x4a_0000,
                    0x1460,
                    [0x0019_5458, 0x0019_5464, 0x0019_5484],
                )?;
                c.cycles += 1;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateTitle_OnChildEvent` (0x0019_4980): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_title_on_child_event_00194980(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as RET;

    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;

    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    use ssx3_ee as ee;
    // gp - 0x428c

    // -------------------------------------------------------------- front end

    #[test]
    fn fe_state_bonus_material_ctor_matches() {
        check(
            ssx3_ee::s02::f_00195498__generated,
            fe_state_bonus_material_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1954_98, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn fe_state_main_menu_ctor_matches() {
        check(
            ssx3_ee::s02::f_00194a60__generated,
            fe_state_main_menu_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x194a60, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn c_fe_state_title_on_widget_created_matches() {
        check(
            ee::s02::f_00194988__generated,
            c_fe_state_title_on_widget_created,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1949_88, 0);
                alloc_ready(c);
                let item = DATA + 0x100;
                clear(c, item, 0x100);
                put_words(c, 0x004a_17d0, &[0x5449_544c, 0]);
                let hash = load(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_17d0]) as u32;
                c.bus
                    .write32(item + 0x38, if seed % 2 == 0 { hash } else { hash ^ 1 });
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(item);
            },
        );
    }

    #[test]
    fn cfe_state_title_on_widget_notify_matches() {
        check(
            ssx3_ee::s02::f_00194a48__generated,
            cfe_state_title_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x194a48, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    // ------------------------------------------------------- front end widgets

    #[test]
    fn fe_state_main_menu_on_widget_notify_matches() {
        check(
            ee::s02::f_001952e8__generated,
            fe_state_main_menu_on_widget_notify,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0019_52e8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 6,
                    1 => 9,
                    2 => 5,
                    _ => 12,
                };
                w(c, DATA + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x100 });
            },
        );
    }

    #[test]
    fn c_fe_state_title_on_widget_action_matches() {
        check(
            ssx3_ee::s02::f_001949e0__generated,
            c_fe_state_title_on_widget_action,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1949e0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    5
                } else {
                    1 + seed as u64 % 7
                };
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x118, r.next_u32());
            },
        );
    }

    #[test]
    fn c_fe_state_title_init_matches() {
        check(
            ee::s02::f_00194778__generated,
            c_fe_state_title_init,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1947, 3);
                w(
                    c,
                    DATA + 0x10,
                    if seed % 3 == 0 { 0 } else { DATA + 0x1000 },
                );
                fill(c, &mut r, DATA + 0x40, 4);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_title_transition_in_matches() {
        check(
            ee::s02::f_001947f8__generated,
            cfe_state_title_transition_in,
            6,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0019_47f8, 3);
                c.g[A0][0] = OBJ as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1930),
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        _ => 2 + (seed % 3),
                    },
                );
                w(c, OBJ + 0x40, if seed % 3 == 0 { 0 } else { OBJ2 });
                w(c, OBJ2 + 8, OBJ3);
                c.bus.write16(OBJ3 + 0x38, (seed as u16) & !3);
                w(c, OBJ3 + 0x3c, FAKE);
                c.bus.write16(OBJ3 + 0x48, (seed as u16) & !3);
                w(c, OBJ3 + 0x4c, FAKE + 0x10);
            },
        );
    }

    /// Scrambled registers and a game-like `gp` (no fake-call hooks: these
    /// tests only reach callees that are already ported).
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x47_0133 ^ ((seed as u64) << 9) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Call a guest setup function and return `v0`.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `GetHashValue32` of the string at `site`.
    pub(crate) fn str_hash(c: &mut Cpu, site: u32) -> u64 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[site as u64])
    }

    // ------------------------------------------------------------------- FE.

    #[test]
    fn cfe_state_main_menu_on_widget_created_matches() {
        check(
            ee::s02::f_00194d18__generated,
            cfe_state_main_menu_on_widget_created,
            200,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x19_4d18);
                let sites = [
                    0x0046_0050u32,
                    0x0046_0060,
                    0x0046_0040,
                    0x0046_0070,
                    0x004a_17d8,
                ];
                // Distinct short strings at every hash key.
                for (i, site) in sites.iter().enumerate() {
                    for k in 0..8u32 {
                        c.bus.write8(site + k, 0);
                    }
                    for k in 0..(i as u32 + 1) {
                        c.bus.write8(site + k, b'A' + i as u8 + k as u8);
                    }
                }
                let pick = seed % 6;
                let id = if pick < 5 {
                    str_hash(c, sites[pick as usize]) as u32
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x38, id);
            },
        );
    }

    #[test]
    fn cfestatetitle_update_matches() {
        check_lohi(
            ee::s02::f_001948a8__generated,
            cfestatetitle_update,
            12,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1948a8, 2);
                let this = DATA;
                clear_2(c, this, 0x200);
                c.g[A0][0] = this as u64;
                w(
                    c,
                    this + 0x48,
                    match seed % 3 {
                        0 => 0,
                        1 => 1800,
                        _ => 1801,
                    },
                );
                w(c, GAME_GP.wrapping_sub(0x1724), 0);
                // two empty lists under the world
                let l0 = DATA + 0x1000;
                let l1 = DATA + 0x2000;
                clear_2(c, l0, 0x40);
                clear_2(c, l1, 0x40);
                w(c, WORLD + 0xb0, l0);
                w(c, WORLD + 0xb4, l1);
                w(c, l0, 0);
                w(c, l1, 0);
            },
        );
    }

    #[test]
    fn cfestatemainmenu_update_matches() {
        check(
            ee::s02::f_00194dd0__generated,
            cfestatemainmenu_update,
            12,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x194dd0, 3);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, DATA + 0x200);
                w(c, DATA + 0x200 + 0xc, DATA + 0x300);
                let vt = vtable(c, &mut r, &[0x160, 0x168]);
                w(c, DATA + 0x300 + 8, vt);
                w(c, this + 0x55, (seed % 3) as u32);
                w(c, this + 0x40, 0);
            },
        );
    }

    // ------------------------------------------------------- title screen timers

    /// A callee outside the translated code: answer null and return at once.
    /// The rate tests below only count updates (the widgets they look up are
    /// absent anyway), so the callee's results do not matter.
    fn no_callee(c: &mut Cpu, _target: u32) {
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// `cFEStateTitle_Update` at rate `hz` from a zero idle counter: true when
    /// the attract mode is entered within `n` updates.
    fn title_attracts_after(hz: u32, n: u32) -> bool {
        let mut c = crate::difftest::fresh();
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        c.g[GP][0] = GAME_GP as u64;
        let this = DATA;
        clear_2(&mut c, this, 0x200);
        w(&mut c, this + 0x48, 0);
        let flag = GAME_GP.wrapping_sub(0x1724);
        w(&mut c, flag, 0);
        // the two (empty) world lists the idle check walks
        let l0 = DATA + 0x1000;
        let l1 = DATA + 0x2000;
        clear_2(&mut c, l0, 0x40);
        clear_2(&mut c, l1, 0x40);
        w(&mut c, WORLD + 0xb0, l0);
        w(&mut c, WORLD + 0xb4, l1);
        for i in 1..=n {
            c.g[A0][0] = this as u64;
            cfestatetitle_update(&mut c);
            assert_eq!(c.pc, RET, "update {i} did not return");
            if c.bus.read32(flag) & 4 != 0 {
                return true;
            }
        }
        false
    }

    /// `cFEStateTitle_TransitionIn` at rate `hz`, starting from the game's
    /// data initializer of `D_004A17C0` (300): true when the fade starts
    /// within `n` updates.
    fn title_fades_after(hz: u32, n: u32) -> bool {
        let mut c = crate::difftest::fresh();
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        c.g[GP][0] = GAME_GP as u64;
        let this = DATA;
        clear_2(&mut c, this, 0x100);
        // no screen at +0x40: both widget lookups return null and are skipped
        w(&mut c, this + 0x40, 0);
        // The data initializer as `tick::on_vblank` writes it for this rate.
        let init = crate::tick::ticks_from_frames60(&mut c, TRANSIENT_WAIT_FRAMES);
        w(&mut c, GAME_GP.wrapping_sub(0x1930), init);
        for i in 1..=n {
            c.g[A0][0] = this as u64;
            cfe_state_title_transition_in(&mut c);
            assert_eq!(c.pc, RET, "transition {i} did not return");
            if c.g[V0][0] == 1 {
                return true;
            }
        }
        false
    }

    /// The title's 30 s idle timeout is a wall clock duration, not a count of
    /// updates: 1801 updates at 60 Hz (the original), 3601 at 120 Hz, 7201 at
    /// 240 Hz.
    #[test]
    fn cfestatetitle_idle_timeout_follows_the_rate() {
        assert!(!title_attracts_after(60, 1800));
        assert!(title_attracts_after(60, 1801));
        assert!(!title_attracts_after(120, 3600));
        assert!(title_attracts_after(120, 3601));
        assert!(!title_attracts_after(240, 7200));
        assert!(title_attracts_after(240, 7201));
    }

    /// The title's pre-fade wait (`D_004A17C0` = 300 frames = 5 s) is a wall
    /// clock duration too: `tick::on_vblank` rewrites its data initializer to
    /// 300 ticks at 60 Hz, 600 at 120 Hz, 1200 at 240 Hz (see
    /// `tick::FRAME_COUNTS`), and the countdown consumes exactly that many
    /// updates.
    #[test]
    fn cfe_state_title_transition_wait_is_the_patched_tick_count() {
        assert!(!title_fades_after(60, 299));
        assert!(title_fades_after(60, 300));
        assert!(!title_fades_after(120, 599));
        assert!(title_fades_after(120, 600));
        assert!(!title_fades_after(240, 1199));
        assert!(title_fades_after(240, 1200));
    }

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc437_covered;

    pub(crate) use crate::game::fe::festatecharsetup::fe_state_char_setup_dtor;
    pub(crate) use crate::game::fe::festateload::fe_state_event_select_dtor;
    pub(crate) use crate::game::fe::festatetransition::fe_state_background_dtor;
    pub(crate) use crate::game::fe::festatetransition::fe_state_transition_dtor;

    /// The front-end state destructors differ in vtable constant and return address.
    #[test]
    fn fe_state_dtors_match() {
        let all: [(fn(&mut Cpu), fn(&mut Cpu), u32); 6] = [
            (
                ee::s04::f_001de728__generated,
                fe_state_main_menu_dtor,
                0x001d_e728,
            ),
            (
                ee::s04::f_001de6c8__generated,
                fe_state_title_dtor,
                0x001d_e6c8,
            ),
            (
                ee::s04::f_001de668__generated,
                fe_state_transition_dtor,
                0x001d_e668,
            ),
            (
                ee::s04::f_001de5c8__generated,
                fe_state_background_dtor,
                0x001d_e5c8,
            ),
            (
                ee::s04::f_001de528__generated,
                fe_state_event_select_dtor,
                0x001d_e528,
            ),
            (
                ee::s04::f_001dd8a0__generated,
                fe_state_char_setup_dtor,
                0x001d_d8a0,
            ),
        ];
        for (generated, hand, entry) in all {
            oc437_covered(generated, hand, entry, 1, &|_, _, _| {});
        }
    }

    pub(crate) use crate::game::fe::festatebonusmaterial::tests::ui_state_case;

    #[test]
    fn c_fe_state_title_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de690__generated,
            c_fe_state_title_ctor_2,
            0x001d_e690,
        );
    }

    #[test]
    fn c_fe_state_main_menu_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de6f0__generated,
            c_fe_state_main_menu_ctor_2,
            0x001d_e6f0,
        );
    }

    /// `cFEStateMainMenu_OnRender` with the screen lookup returning nothing
    /// (the zeroed object, `+0x3c == 0`), so the paths through the vtable
    /// calls are taken only when the lookup succeeds: this covers the
    /// lookup-miss paths and the mode-bit tests. The vtable slots point at
    /// the fake methods (the `miss` hook), so the call log is compared.
    #[test]
    fn cfe_state_main_menu_on_render_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const SELF: u32 = DATA + 0x8000;
        const OBJ: u32 = DATA + 0x8100;
        const VT: u32 = DATA + 0x8200;
        check(
            ee::s02::f_00194bf0__generated,
            cfe_state_main_menu_on_render,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_4bf0, 0);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x0019_4bf0;
                c.g[A0][0] = sx(SELF);
                for (i, off) in [0xc8u32, 0x40, 0x58].into_iter().enumerate() {
                    c.bus.write16(VT + off, 0);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
                }
                c.bus.write32(OBJ + 0x8, VT);
                c.bus.write32(OBJ + 0x3c, 0);
                c.bus.write32(SELF + 0x40, OBJ);
                c.bus.write8(SELF + 0x55, r.next_u32() as u8);
                c.bus.write8(SELF + 0x1d, r.next_u32() as u8);
                c.bus.write32(GAME_GP - 0x704, r.next_u32() & 0x3f);
            },
        );
    }

    use crate::difftest::fakes;
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc783's callee fakes in a machine's RAM.
    pub(crate) const OC783_CTRL: u32 = DATA + 0x30_0010;

    pub(crate) const OC783_MAGIC: u32 = 0x0783_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc783_mail(addr: u32) -> u32 {
        DATA + 0x30_2000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc783_arm(c: &mut Cpu) {
        c.bus.write32(OC783_CTRL, OC783_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc783_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc783_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc783_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc783_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc783_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc783_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc783_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC783_CTRL) != OC783_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc783_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc783_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc783_fakes!(
        (
            oc783_fk_3e18c8,
            OC783_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc783_fk_3b39b8,
            OC783_PREV_3B39B8,
            ee::s13::OVERRIDE_003B39B8,
            ee::s13::f_003b39b8__generated,
            0x003b_39b8
        ),
        (
            oc783_fk_317670,
            OC783_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc783_fk_397cc0,
            OC783_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc783_fk_39c870,
            OC783_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc783_fk_3e6448,
            OC783_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc783_fk_266cd8,
            OC783_PREV_266CD8,
            ee::s07::OVERRIDE_00266CD8,
            ee::s07::f_00266cd8__generated,
            0x0026_6cd8
        ),
        (
            oc783_fk_261008,
            OC783_PREV_261008,
            ee::s06::OVERRIDE_00261008,
            ee::s06::f_00261008__generated,
            0x0026_1008
        ),
        (
            oc783_fk_25b688,
            OC783_PREV_25B688,
            ee::s06::OVERRIDE_0025B688,
            ee::s06::f_0025b688__generated,
            0x0025_b688
        ),
        (
            oc783_fk_14dd58,
            OC783_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc783_fk_28b180,
            OC783_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc783_fk_2b0da0,
            OC783_PREV_2B0DA0,
            ee::s08::OVERRIDE_002B0DA0,
            ee::s08::f_002b0da0__generated,
            0x002b_0da0
        ),
        (
            oc783_fk_3dedc0,
            OC783_PREV_3DEDC0,
            ee::s14::OVERRIDE_003DEDC0,
            ee::s14::f_003dedc0__generated,
            0x003d_edc0
        ),
        (
            oc783_fk_317f38,
            OC783_PREV_317F38,
            ee::s10::OVERRIDE_00317F38,
            ee::s10::f_00317f38__generated,
            0x0031_7f38
        ),
        (
            oc783_fk_3ded50,
            OC783_PREV_3DED50,
            ee::s14::OVERRIDE_003DED50,
            ee::s14::f_003ded50__generated,
            0x003d_ed50
        ),
        (
            oc783_fk_287968,
            OC783_PREV_287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc783_fk_2b0e60,
            OC783_PREV_2B0E60,
            ee::s08::OVERRIDE_002B0E60,
            ee::s08::f_002b0e60__generated,
            0x002b_0e60
        ),
        (
            oc783_fk_14dc80,
            OC783_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc783_fk_2adf60,
            OC783_PREV_2ADF60,
            ee::s08::OVERRIDE_002ADF60,
            ee::s08::f_002adf60__generated,
            0x002a_df60
        ),
        (
            oc783_fk_2b9100,
            OC783_PREV_2B9100,
            ee::s08::OVERRIDE_002B9100,
            ee::s08::f_002b9100__generated,
            0x002b_9100
        ),
        (
            oc783_fk_252fa0,
            OC783_PREV_252FA0,
            ee::s06::OVERRIDE_00252FA0,
            ee::s06::f_00252fa0__generated,
            0x0025_2fa0
        ),
    );

    pub(crate) const OC783_MENU: u32 = DATA + 0x1_0000;

    pub(crate) const OC783_ENGINE: u32 = DATA + 0x2_0000;

    pub(crate) const OC783_INPUT: u32 = DATA + 0x3_0000;

    pub(crate) const OC783_INPUT_VT: u32 = DATA + 0x3_1000;

    pub(crate) const OC783_WORLD: u32 = DATA + 0x4_0000;

    pub(crate) const OC783_BE_IFACE: u32 = DATA + 0x4_1000;

    pub(crate) const OC783_BE_IFACE_VT: u32 = DATA + 0x4_2000;

    pub(crate) const OC783_AUDIO: u32 = DATA + 0x8_0000;

    /// `cFEStateMainMenu_Init`: the screen lookup succeeding or not, the
    /// second input call's result (0 or not) for both controller ports, and
    /// the unwinds of every call. The real `cGameComm_destroy` and
    /// `cCommSystem_destroy` wrappers run on empty globals and
    /// `func_0028F140` returns at once (audio state already selected).
    #[test]
    fn cfe_state_main_menu_init_matches() {
        oc783_install_fakes();
        fakes::covered(
            ee::s02::f_00194aa8__generated,
            cfe_state_main_menu_init,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x783c, 12);
                oc783_arm(c);
                c.g[A0][0] = sx(OC783_MENU);
                c.bus.write32(OC783_MENU + 0x10, OC783_ENGINE);
                // The engine's input object and its two virtual calls.
                c.bus.write32(OC783_ENGINE + 0xc, OC783_INPUT);
                c.bus.write32(OC783_INPUT + 8, OC783_INPUT_VT);
                c.bus
                    .write16(OC783_INPUT_VT + 0x160, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_INPUT_VT + 0x164, FAKE + 0x30);
                c.bus
                    .write16(OC783_INPUT_VT + 0x168, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_INPUT_VT + 0x16c, 0x0039_c870);
                // The world, its back-end interface 7 and the interface's
                // virtual submit.
                c.bus.write32(GAME_GP - 0x848, OC783_WORLD);
                c.bus.write32(OC783_WORLD + 0x78, r.next_u32());
                oc783_force(c, 0x0014_dd58, OC783_BE_IFACE);
                c.bus.write32(OC783_BE_IFACE + 0xc, OC783_BE_IFACE_VT);
                c.bus
                    .write16(OC783_BE_IFACE_VT + 8, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_BE_IFACE_VT + 0xc, FAKE + 0x20);
                // `func_0028F140` returns at once when the state is the same.
                oc783_force(c, 0x0028_b180, OC783_AUDIO);
                c.bus.write32(OC783_AUDIO + 0x6290, 1);
                // Screen found (non-zero) or not; the 0x39c870 result is also the
                // second input call's result.
                if seed % 3 == 0 {
                    oc783_force_zero(c, 0x0039_7cc0);
                }
                if (seed / 3) % 2 == 0 {
                    oc783_force_zero(c, 0x0039_c870);
                }
            },
        );
    }

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc800_machine;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc800_mask;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::OC800_CTRL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::OC800_MAGIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::OC800_SELF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::OC800_SEQ;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc800_engine;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc800_widget;
    pub(crate) use crate::game::fe::festatecharselect::tests::OC800_ENG;

    pub(crate) use crate::game::fe::festatecharselect::tests::OC800_WIDGET;

    use crate::game::main::loadscreens::tests::oc736_fake_call;

    /// The fake callee: the shared oc736 fake (log, result mailbox), then the
    /// per-callee mask, if any, forces a 0 result on its clear bits.
    pub(crate) fn oc800_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let slot = OC800_SEQ + ((addr >> 2) & 0x3ff) * 8;
        let mask = c.bus.read32(slot);
        let n = c.bus.read32(slot + 4);
        if mask != 0 {
            c.bus.write32(slot + 4, n + 1);
        }
        oc736_fake_call(c, addr);
        if mask != 0 && c.pc == ra && (mask >> (n % 32)) & 1 == 0 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc800_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC800_CTRL) != OC800_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc800_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc800_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc800_fakes!(
        (
            oc800_fake_147318,
            OC800_PREV_147318,
            ee::s01::OVERRIDE_00147318,
            ee::s01::f_00147318__generated,
            0x00147318
        ),
        (
            oc800_fake_1474a8,
            OC800_PREV_1474A8,
            ee::s01::OVERRIDE_001474A8,
            ee::s01::f_001474a8__generated,
            0x001474a8
        ),
        (
            oc800_fake_181bd0,
            OC800_PREV_181BD0,
            ee::s02::OVERRIDE_00181BD0,
            ee::s02::f_00181bd0__generated,
            0x00181bd0
        ),
        (
            oc800_fake_1887a0,
            OC800_PREV_1887A0,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x001887a0
        ),
        (
            oc800_fake_192240,
            OC800_PREV_192240,
            ee::s02::OVERRIDE_00192240,
            ee::s02::f_00192240__generated,
            0x00192240
        ),
        (
            oc800_fake_192488,
            OC800_PREV_192488,
            ee::s02::OVERRIDE_00192488,
            ee::s02::f_00192488__generated,
            0x00192488
        ),
        (
            oc800_fake_1a1cb8,
            OC800_PREV_1A1CB8,
            ee::s03::OVERRIDE_001A1CB8,
            ee::s03::f_001a1cb8__generated,
            0x001a1cb8
        ),
        (
            oc800_fake_1a86e8,
            OC800_PREV_1A86E8,
            ee::s03::OVERRIDE_001A86E8,
            ee::s03::f_001a86e8__generated,
            0x001a86e8
        ),
        (
            oc800_fake_1d53b0,
            OC800_PREV_1D53B0,
            ee::s04::OVERRIDE_001D53B0,
            ee::s04::f_001d53b0__generated,
            0x001d53b0
        ),
        (
            oc800_fake_25b650,
            OC800_PREV_25B650,
            ee::s06::OVERRIDE_0025B650,
            ee::s06::f_0025b650__generated,
            0x0025b650
        ),
        (
            oc800_fake_260f80,
            OC800_PREV_260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x00260f80
        ),
        (
            oc800_fake_266ca0,
            OC800_PREV_266CA0,
            ee::s07::OVERRIDE_00266CA0,
            ee::s07::f_00266ca0__generated,
            0x00266ca0
        ),
        (
            oc800_fake_317670,
            OC800_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc800_fake_317d70,
            OC800_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc800_fake_39a3f8,
            OC800_PREV_39A3F8,
            ee::s13::OVERRIDE_0039A3F8,
            ee::s13::f_0039a3f8__generated,
            0x0039a3f8
        ),
        (
            oc800_fake_39a568,
            OC800_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039a568
        ),
        (
            oc800_fake_39a670,
            OC800_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039a670
        ),
        (
            oc800_fake_39a768,
            OC800_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039a768
        ),
        (
            oc800_fake_39a7a8,
            OC800_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039a7a8
        ),
        (
            oc800_fake_39c778,
            OC800_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039c778
        ),
        (
            oc800_fake_39c870,
            OC800_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc800_fake_39d860,
            OC800_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc800_fake_39e318,
            OC800_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039e318
        ),
        (
            oc800_fake_39f320,
            OC800_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc800_fake_39f400,
            OC800_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039f400
        ),
        (
            oc800_fake_3a1000,
            OC800_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc800_fake_3e5698,
            OC800_PREV_3E5698,
            ee::s14::OVERRIDE_003E5698,
            ee::s14::f_003e5698__generated,
            0x003e5698
        ),
        (
            oc800_fake_3e56e0,
            OC800_PREV_3E56E0,
            ee::s14::OVERRIDE_003E56E0,
            ee::s14::f_003e56e0__generated,
            0x003e56e0
        ),
        (
            oc800_fake_3f3b18,
            OC800_PREV_3F3B18,
            ee::s14::OVERRIDE_003F3B18,
            ee::s14::f_003f3b18__generated,
            0x003f3b18
        ),
        (
            oc800_fake_41aa88,
            OC800_PREV_41AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041aa88
        ),
    );

    /// The jump table of the item switch of `cFEStateMainMenu_OnWidgetAction`
    /// (`mMiscData` 5 to 49): 5 -> 0x194fc8, 9 -> 0x194f68, 11 -> 0x194f54,
    /// 22 -> 0x194f88, 49 -> 0x194f9c, else 0x194fd0.
    pub(crate) fn oc800_main_menu_table(c: &mut Cpu) {
        for i in 0..45u32 {
            c.bus.write32(0x0046_00b0 + 4 * i, 0x0019_4fd0);
        }
        for (misc, t) in [
            (5u32, 0x0019_4fc8u32),
            (9, 0x0019_4f68),
            (11, 0x0019_4f54),
            (22, 0x0019_4f88),
            (49, 0x0019_4f9c),
        ] {
            c.bus.write32(0x0046_00b0 + 4 * (misc - 5), t);
        }
    }

    pub(crate) fn oc800_main_menu_setup(
        c: &mut Cpu,
        seed: u32,
        ev: u32,
        misc: u32,
        hack: u32,
        null_widget: bool,
    ) {
        let mut r = oc800_machine(c, seed, 0x0019_4ea8, 40);
        oc800_main_menu_table(c);
        oc800_engine(c, seed);
        c.bus.write32(OC800_SELF + 0x10, OC800_ENG);
        c.bus.write8(OC800_SELF + 0x14, (seed % 4) as u8);
        c.bus.write32(OC800_SELF + 0x40, OC800_ENG + 0x100);
        c.bus.write32(OC800_SELF + 0x48, 0);
        c.bus.write8(OC800_SELF + 0x54, hack as u8);
        oc800_widget(c, misc);
        let mask = match seed % 3 {
            0 => 0xffff_ffff,
            _ => r.next_u32() | 0x8000_0000,
        };
        oc800_mask(c, 0x0039_d860, mask);
        c.g[A0][0] = sx(OC800_SELF);
        c.g[A1][0] = if null_widget { 0 } else { sx(OC800_WIDGET) };
        c.g[A2][0] = sx(ev);
        c.pc = 0x0019_4ea8;
    }

    /// `cFEStateMainMenu_OnWidgetAction`: select with each item kind that has
    /// no online setup (game path 0 to 2, the plain go-on item, an item the
    /// switch ignores), back, options, the cheat counter below and at 7, an
    /// unknown event, and a null item.
    ///
    /// Not reached: the `mMiscData == 22 && !goToNextState` popup (the switch
    /// sets the flag on every path that leaves 22, so it is dead code).
    #[test]
    fn main_menu_on_widget_action_matches() {
        check(
            ee::s02::f_00194ea8__generated,
            main_menu_on_widget_action,
            300,
            &|c, seed| {
                let (ev, misc) = [
                    (5u32, 11u32),
                    (5, 9),
                    (5, 22),
                    (5, 5),
                    (5, 7),
                    (6, 0),
                    (7, 0),
                    (8, 0),
                    (0, 0),
                    (9, 0),
                ][(seed % 10) as usize];
                let hack = (seed / 10) % 9;
                let null_widget = (seed / 10) % 13 == 12;
                oc800_main_menu_setup(c, seed, ev, misc, hack, null_widget);
            },
        );
    }

    /// Item 49 of `cFEStateMainMenu_OnWidgetAction`: the online managers
    /// (`LobbyMan_Create`, `BuddyMan_Create`, `cVoipMan_Create` on fakes; the
    /// real `cCommSystem_Create` and its constructor, which have no override
    /// slot, on the fakes of their callees).
    #[test]
    fn main_menu_on_widget_action_online_matches() {
        check(
            ee::s02::f_00194ea8__generated,
            main_menu_on_widget_action,
            80,
            &|c, seed| {
                oc800_main_menu_setup(c, seed, 5, 49, 0, false);
            },
        );
    }

    pub(crate) use crate::game::fe::festatebigradio::tests::oc851_machine;

    pub(crate) use crate::game::fe::festatebigradio::tests::oc851_vtable;

    pub(crate) use crate::game::fe::festatebigradio::tests::OC851_OBJ2;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC851_SELF;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC851_VT_SELF;
    pub(crate) use crate::game::fe::festatebonusmaterial::tests::OC851_ZERO_LEAF;

    pub(crate) const OC851_VT_OBJ2: u32 = DATA + 0x8_3800;

    /// `cFEStateMainMenu_OnChildEvent`: messages 0x15 and 0x16 (with and
    /// without a widget answer), anything else; the four show-widget steps.
    #[test]
    fn main_menu_on_child_event_matches() {
        check(
            ee::s02::f_00195328__generated,
            main_menu_on_child_event,
            200,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x195328, 4);
                oc851_vtable(c, &mut r, OC851_VT_SELF, &[0x98]);
                c.bus.write32(OC851_SELF + 8, OC851_VT_SELF);
                c.bus.write32(OC851_SELF + 0x4c, DATA + 0x4_0000);
                c.bus.write8(OC851_SELF + 0x50, r.next_u32() as u8);
                // The widget object: slot 0xb8 is a fake, or a real leaf returning 0.
                oc851_vtable(c, &mut r, OC851_VT_OBJ2, &[0xb8]);
                if seed % 3 == 0 {
                    c.bus.write32(OC851_VT_OBJ2 + 0xbc, OC851_ZERO_LEAF);
                }
                c.bus.write32(OC851_OBJ2 + 8, OC851_VT_OBJ2);
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = sx(OC851_OBJ2);
                c.g[A2][0] = [0x15u64, 0x16, 0x16, 0x16, 0x17, 0][seed as usize % 6];
                c.pc = 0x0019_5328;
            },
        );
    }

    /// `cFEStateTitle_OnChildEvent` (0x0019_4980) against the generated body.
    #[test]
    fn c_fe_state_title_on_child_event_00194980_matches() {
        check(
            ee::s02::f_00194980__generated,
            c_fe_state_title_on_child_event_00194980,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x194980, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
