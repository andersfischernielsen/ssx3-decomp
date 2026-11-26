//! `src/fe/festatecredits.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tagsunk::const_reg;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::fe_ctor_with_vtable;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festatebonusmaterial::FE_UI_STATE_CTOR;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::sound::bankmonitor::leave;
use crate::guest::Addr;
use crate::tick;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_63b8,
        cfe_options_credits_update,
        "cFEOptionsCredits_Update",
    ),
    (
        0x0018_63e8,
        c_fe_options_credits_handle_input,
        "cFEOptionsCredits_HandleInput",
    ),
    (
        0x0018_6478,
        fe_options_credits_on_widget_notify,
        "cFEOptionsCredits_OnWidgetNotify",
    ),
    (
        0x0018_64b0,
        cfe_options_credits_on_widget_action,
        "cFEOptionsCredits_OnWidgetAction",
    ),
    (
        0x0018_65a8,
        cfe_state_on_deactivate,
        "cFEState_OnDeactivate",
    ),
    (
        0x0018_6610,
        fe_state_event_select_ctor,
        "cFEStateEventSelect_ctor",
    ),
    (
        0x001d_e4b0,
        c_fe_options_credits_ctor_2,
        "cFEOptionsCredits_ctor_2",
    ),
    (
        0x0018_5a98,
        cfe_options_credits_init,
        "cFEOptionsCredits_Init",
    ),
    (
        0x0018_5ba0,
        cfe_options_credits_on_activate,
        "cFEOptionsCredits_OnActivate",
    ),
    (
        0x0018_5f40,
        cfe_options_credits_render,
        "cFEOptionsCredits_Render",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;

    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// `void cFEOptionsCredits_Update(cFEOptionsCredits* self)` (0x1863b8):
/// advance the credits scroll (`+0x60 += +0x64`), then the base update.
///
/// The scroll is authored as a per-60 Hz-frame step: `+0x64` is a speed in
/// pixels per authored frame (it ramps between -8 and 8), so the position
/// advances once per 60 Hz boundary (`tick::ui_frame_due`) rather than once
/// per tick. Scaling the *step* with `* 60 / rate` is not usable here: a
/// 1 px/frame speed would truncate to 0 and the credits would stall while the
/// speed ramps up.
pub fn cfe_options_credits_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(self_, 0x60)).lw(c);
        c.g[A1][0] = Addr(at(self_, 0x64)).lw(c);
        // One authored frame per 60 Hz boundary; at 60 Hz that is every
        // update, so the original add runs unchanged.
        let step = if tick::ui_frame_due(c) {
            c.g[A1][0] as u32
        } else {
            0
        };
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(step));
        let y = c.g[V1][0];
        Addr(at(self_, 0x60)).sw(c, y);
        call_ptr(c, callee::UI_STATE_V13, 0x0018_63d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateEventSelect* cFEStateEventSelect_ctor(self)` (0x186610): UI state
/// constructor, then install the vtable/state and construct the embedded
/// event-select sub-object at +0x48.
pub fn fe_state_event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR, 0x0018_6624)?;
        let this = r32(c, S0);
        const_reg(c, V1, 0x47_0000, -0x42d8);
        c.g[V0][0] = 0x13;
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x48));
        call_ptr(c, 0x0020_06b8, 0x0018_6640)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `int cFEOptionsCredits_OnWidgetNotify(...)` (0x186478): `Ok`/`Next`/`Prev`
/// (6, 8, 9) answer 0x100, everything else 0x101.
pub fn fe_options_credits_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if !is_six {
        c.cycles += 2;
        let below_six = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
        if !below_six {
            c.cycles += 2;
            let at_least_ten = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 8) as u64;
            if at_least_ten {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let below_eight = c.g[V0][0] != 0;
            if below_eight {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            // 8 <= id < 10
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = WIDGET_IGNORED;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = WIDGET_ACTIVATED;
    abi::ret(c);
}

/// `void cFEOptionsCredits_OnWidgetAction(self, item, int msg)` (0x1864b0):
/// for message 6, ask the engine object's slot 5 for the credits screen
/// (hash name 0x24) and push it if found.
pub fn cfe_options_credits_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let no_item = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if no_item {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            if c.g[A2][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x24;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0018_64f4);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_64f4)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0018_6508);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0018_6508)?;
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `void cFEState_OnDeactivate(cFEState* self)` (0x1865a8): find the current
/// screen's credits object and tell the FE module it was left.
pub fn cfe_state_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0018_65c4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
        call_ptr(c, 0x0039_7840, 0x0018_65c4)?;
        c.g[S0][0] = c.g[V0][0];
        let no_screen = c.g[S0][0] == 0;
        c.g[A0][0] = 0x4a0000;
        if no_screen {
            // The empty-screen path runs the shared tail (its +1 and lq16).
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        } else {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0018_65d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
            call_ptr(c, 0x0031_7670, 0x0018_65d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0018_65e4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_65e4)?;
            let not_found = c.g[V0][0] == 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if !not_found {
                c.cycles += 4;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx(gp(c, 0x1d90));
                c.g[RA][0] = sx(0x0018_65fc);
                c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x95));
                call_ptr(c, 0x001a_0708, 0x0018_65fc)?;
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `int cFEOptionsCredits_HandleInput(self, Pad* pad)` (0x1863e8): map the
/// pad's left/right presses to a credits scroll step (or a page turn).
pub fn c_fe_options_credits_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let pad = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = pad;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(self_);
        c.g[V1][0] = c.lw(at_2(c, S0, 8));
        c.g[A0][0] = c.lh(at_2(c, V1, 0x88));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x8c));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0018_6414)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = c.lw(at_2(c, S1, 0x64));
            c.g[V0][0] = ((c.g[V1][0] as i64) < -7) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
                c.cycles += 1;
                c.sw(at_2(c, S1, 0x64), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        } else {
            c.g[V1][0] = c.lw(at_2(c, S0, 8));
            c.cycles += 6;
            c.g[A0][0] = c.lh(at_2(c, V1, 0x90));
            c.g[V0][0] = c.lw(at_2(c, V1, 0x94));
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0018_6444)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw(at_2(c, S1, 0x64));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                let too_far = c.g[V0][0] == 0;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                if too_far {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 1;
                    c.sw(at_2(c, S1, 0x64), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEOptionsCredits` constructor (0x1de4b0): `UIState_ctor(this, 0)`, then
/// the vtable at `+8`.
pub fn c_fe_options_credits_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001d_e4c8,
            0x4208,
        )
    });
}

/// The address `off(reg)`.
#[inline(always)]
pub(crate) fn oc830_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `lw reg, -off(gp)`.
#[inline(always)]
pub(crate) fn oc830_gp_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// `addiu`-style `sx(reg - k)`.
#[inline(always)]
pub(crate) fn oc830_sx_sub(c: &Cpu, r: usize, k: u32) -> u64 {
    sx((c.g[r][0] as u32).wrapping_sub(k))
}

/// `cFEOptionsCredits::Init` (0x185a98): create the credits screen from the
/// UI engine, start its thread, and look up the text-scroll widgets.
pub fn cfe_options_credits_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2480);
        let a = oc830_ea(c, S1, 0x10);
        c.g[S0][0] = c.lw(a);
        call_ptr(c, 0x0031_7670, 0x0018_5abc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_5ad0)?;
        let a = oc830_ea(c, S1, 0x40);
        c.sw(a, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_5ae8)?;
        }
        // Widget 1: set bit 3 of its flags word.
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2470);
        call_ptr(c, 0x0031_7670, 0x0018_5af4)?;
        let a = oc830_ea(c, S1, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5b00)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x46_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            let a = oc830_ea(c, V1, 0x90);
            c.g[V0][0] = c.lw(a);
            c.g[V0][0] |= 0x8;
            c.sw(a, c.g[V0][0]);
        }
        // Widgets 2 and 3: remember them at +0x58 and call their method at
        // vtable +0x48 (the same slot both times, with a1 = 0).
        c.cycles += 8;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2460);
        call_ptr(c, 0x0031_7670, 0x0018_5b20)?;
        let a = oc830_ea(c, S1, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5b2c)?;
        c.g[A2][0] = c.g[V0][0];
        let a = oc830_ea(c, S1, 0x58);
        c.sw(a, c.g[A2][0]);
        if c.g[A2][0] != 0 {
            oc830_init_vcall(c, 6, 0x0018_5b50)?;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2450);
        call_ptr(c, 0x0031_7670, 0x0018_5b5c)?;
        let a = oc830_ea(c, S1, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5b68)?;
        c.g[A2][0] = c.g[V0][0];
        let a = oc830_ea(c, S1, 0x58);
        c.sw(a, c.g[A2][0]);
        if c.g[A2][0] != 0 {
            oc830_init_vcall(c, 6, 0x0018_5b8c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Init's `widget->vtable[0x48](widget + delta, 0)` call (a2 = widget).
pub(crate) fn oc830_init_vcall(c: &mut Cpu, cycles: u64, ret: u32) -> Flow {
    c.cycles += cycles;
    let a = oc830_ea(c, A2, 0x8);
    c.g[V0][0] = c.lw(a);
    c.g[A1][0] = 0;
    let a = oc830_ea(c, V0, 0x48);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, V0, 0x4c);
    c.g[V1][0] = c.lw(a);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// One loop of OnActivate's text-id setup: `sprintf(buf, fmt, i)`, hash the
/// buffer and `SetTextId(scroll, hash)` for `i` up to `limit`. `a2` and `s1`
/// hold `i` on entry; `head` is the generated loop head (return addresses
/// are `head + 0x10/0x18/0x24`).
pub(crate) fn oc830_credits_ids(c: &mut Cpu, head: u32, buf: usize, limit: i64) -> Flow {
    loop {
        c.cycles += 12;
        c.g[A1][0] = oc830_sx_sub(c, S4, 0x2430);
        c.g[A0][0] = c.g[buf][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        call_ptr(c, 0x0041_7828, head + 0x10)?;
        c.g[A0][0] = c.g[buf][0];
        call_ptr(c, 0x0031_7670, head + 0x18)?;
        let a = oc830_ea(c, S2, 0x5c);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1f18, head + 0x24)?;
        c.g[V0][0] = ((c.g[S1][0] as i64) < limit) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[S1][0];
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `SetTextId(scroll, StrHash(<string literal at 0x460000 - off>))`.
pub(crate) fn oc830_credits_set_text(c: &mut Cpu, off: u32, ra_hash: u32, ra_set: u32) -> Flow {
    c.g[A0][0] = 0x46_0000;
    c.g[A0][0] = oc830_sx_sub(c, A0, off);
    call_ptr(c, 0x0031_7670, ra_hash)?;
    let a = oc830_ea(c, S2, 0x5c);
    c.g[A0][0] = c.lw(a);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003a_1f18, ra_set)
}

/// `cFEOptionsCredits::OnActivate` (0x185ba0): fill the credits text-scroll
/// widget with the credit lines' text ids, then run `cFEState::OnActivate`.
pub fn cfe_options_credits_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 18, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 22, 0x40);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, 16, 0xa0);
        frame.save(c, 17, 0x90);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, 19, 0x70);
        frame.save(c, 20, 0x60);
        frame.save(c, 21, 0x50);
        frame.save_ra(c, 0x30);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2440);
        call_ptr(c, 0x0031_7670, 0x0018_5bd8)?;
        let a = oc830_ea(c, S2, 0x40);
        c.g[A0][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_5be4)?;
        c.g[A2][0] = c.g[V0][0];
        let a = oc830_ea(c, S2, 0x5c);
        c.sw(a, c.g[A2][0]);
        if c.g[A2][0] != 0 {
            c.cycles += 28;
            let a = oc830_ea(c, A2, 0x8);
            c.g[V1][0] = c.lw(a);
            c.g[A1][0] = c.g[SP][0];
            c.g[S1][0] = 0;
            c.g[S4][0] = 0x46_0000;
            let a = oc830_ea(c, V1, 0xa0);
            c.g[A0][0] = c.lh(a);
            let a = oc830_ea(c, V1, 0xa4);
            c.g[V0][0] = c.lw(a);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_5c10)?;
            c.g[AT][0] = 0x4416_0000;
            c.mtc1(1, 0);
            c.g[A1][0] = c.g[SP][0];
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A2][0] = c.lw(a);
            let sp0 = c.g[SP][0] as u32;
            c.swc1(0, sp0);
            let a = oc830_ea(c, A2, 0x8);
            c.g[V0][0] = c.lw(a);
            let a = oc830_ea(c, V0, 0xa8);
            c.g[A0][0] = c.lh(a);
            let a = oc830_ea(c, V0, 0xac);
            c.g[V1][0] = c.lw(a);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_5c38)?;
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A2][0] = c.lw(a);
            c.g[A1][0] = 0;
            oc830_on_activate_vcall(c, 0x0018_5c54)?;
            c.g[S3][0] = sx(oc830_ea(c, SP, 0x10));
            c.g[S0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5c60, S0, 7)?;
            c.cycles += 16;
            c.g[S1][0] = 0x7;
            c.g[S0][0] = c.g[S3][0];
            oc830_credits_set_text(c, 0x2420, 0x0018_5ca4, 0x0018_5cb0)?;
            oc830_credits_set_text(c, 0x2410, 0x0018_5cbc, 0x0018_5cc8)?;
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5cd0, S0, 47)?;
            c.cycles += 4;
            c.g[S1][0] = 0x31;
            c.g[S0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5d10, S0, 75)?;
            c.cycles += 10;
            c.g[S1][0] = 0x4b;
            c.g[S0][0] = c.g[S3][0];
            oc830_credits_set_text(c, 0x2400, 0x0018_5d54, 0x0018_5d60)?;
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5d68, S0, 81)?;
            c.cycles += 16;
            c.g[S1][0] = 0x5a;
            c.g[S0][0] = c.g[S3][0];
            oc830_credits_set_text(c, 0x23f0, 0x0018_5dac, 0x0018_5db8)?;
            oc830_credits_set_text(c, 0x23e0, 0x0018_5dc4, 0x0018_5dd0)?;
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5dd8, S0, 146)?;
            // The tail: four single lines (ids 0x96..0x99), then the last run.
            c.cycles += 52;
            c.g[S0][0] = oc830_sx_sub(c, S4, 0x2430);
            c.g[S5][0] = c.g[S3][0];
            c.g[A0][0] = 0x46_0000;
            c.g[A0][0] = oc830_sx_sub(c, A0, 0x23d0);
            call_ptr(c, 0x0031_7670, 0x0018_5e1c)?;
            c.g[S1][0] = 0x9a;
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A0][0] = c.lw(a);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1f18, 0x0018_5e2c)?;
            oc830_credits_set_text(c, 0x23c0, 0x0018_5e38, 0x0018_5e44)?;
            for (id, r_fmt, r_hash, r_set) in [
                (0x96u64, 0x0018_5e54u32, 0x0018_5e5cu32, 0x0018_5e68u32),
                (0x97, 0x0018_5e78, 0x0018_5e80, 0x0018_5e8c),
                (0x98, 0x0018_5e9c, 0x0018_5ea4, 0x0018_5eb0),
                (0x99, 0x0018_5ec0, 0x0018_5ec8, 0x0018_5ed4),
            ] {
                c.g[A2][0] = id;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0041_7828, r_fmt)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7670, r_hash)?;
                let a = oc830_ea(c, S2, 0x5c);
                c.g[A0][0] = c.lw(a);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1f18, r_set)?;
            }
            c.g[A2][0] = c.g[S1][0];
            oc830_credits_ids(c, 0x0018_5ed8, S5, 168)?;
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S6][0];
        Callee {
            addr: 0x0018_6518,
            f: ee::s02::f_00186518,
        }
        .call(c, 0x0018_5f14)?;
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// OnActivate's `widget->vtable[0x48](widget + delta, 0)` (a2 = widget, a1 = 0).
pub(crate) fn oc830_on_activate_vcall(c: &mut Cpu, ret: u32) -> Flow {
    let a = oc830_ea(c, A2, 0x8);
    c.g[V0][0] = c.lw(a);
    let a = oc830_ea(c, V0, 0x48);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, V0, 0x4c);
    c.g[V1][0] = c.lw(a);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// Render's `widget->vtable[off](widget + delta, a1)` through `a2`: loads the
/// slot with v0 = vtable, a0 = delta, v1 = fn.
#[inline(always)]
pub(crate) fn oc830_render_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    fnr: usize,
    off: u32,
    ret: u32,
) -> Flow {
    let a = oc830_ea(c, obj, 0x8);
    c.g[vt][0] = c.lw(a);
    let a = oc830_ea(c, vt, off);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, vt, off + 4);
    c.g[fnr][0] = c.lw(a);
    let target = c.g[fnr][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, obj, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `cFEOptionsCredits::Render` (0x185f40): lay out the visible credit lines
/// of the text-scroll widget (`+0x5c`) with the font's glyph metrics (`'~'`
/// moves the marker widget at `+0x58`, `'^'` flips the colour block), draw
/// each run, then wrap the scroll position and run `UIState::Render`.
pub fn cfe_options_credits_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x140);
        frame.save(c, 18, 0xf0);
        frame.save(c, 16, 0x110);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x100);
        frame.save(c, 19, 0xe0);
        frame.save(c, 20, 0xd0);
        frame.save(c, 21, 0xc0);
        frame.save(c, 22, 0xb0);
        frame.save(c, 23, 0xa0);
        frame.save(c, 30, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 23, 0x138);
        frame.save_fpr(c, 22, 0x130);
        frame.save_fpr(c, 21, 0x128);
        frame.save_fpr(c, 20, 0x120);
        let a = oc830_ea(c, S2, 0x5c);
        c.g[A2][0] = c.lw(a);
        'draw: {
            if c.g[A2][0] == 0 {
                break 'draw;
            }
            // Copy the widget's two 12-byte rectangles to sp+0 and sp+0x10.
            c.cycles += 23;
            let a = oc830_ea(c, A2, 0x4b);
            c.ldl(2, a);
            let a = oc830_ea(c, A2, 0x44);
            c.ldr(2, a);
            let a = oc830_ea(c, A2, 0x4c);
            c.g[V1][0] = c.lw(a);
            let sp0 = c.g[SP][0] as u32;
            c.sdl(2, sp0.wrapping_add(0x7));
            c.sdr(2, sp0);
            c.sw(sp0.wrapping_add(0x8), c.g[V1][0]);
            let a = oc830_ea(c, A2, 0x57);
            c.ldl(7, a);
            let a = oc830_ea(c, A2, 0x50);
            c.ldr(7, a);
            let a = oc830_ea(c, A2, 0x58);
            c.g[T0][0] = c.lw(a);
            c.sdl(7, sp0.wrapping_add(0x17));
            c.sdr(7, sp0.wrapping_add(0x10));
            c.sw(sp0.wrapping_add(0x18), c.g[T0][0]);
            c.g[A1][0] = sx(sp0.wrapping_add(0x20));
            oc830_render_vcall(c, A2, V0, V1, 0xa0, 0x0018_5fd4)?;
            let a = oc830_ea(c, S2, 0x5c);
            c.g[A0][0] = c.lw(a);
            call_ptr(c, 0x003a_12d0, 0x0018_5fdc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
            if c.g[S1][0] == 0 {
                break 'draw;
            }
            // Scale the font metrics into a line count (s3) and the scroll
            // position into a first line (fp) and row offset (s4).
            c.cycles += 36;
            let a = oc830_ea(c, S1, 0x14);
            c.lwc1(2, a);
            c.cvt_s_w(2, 2);
            let a = oc830_ea(c, S1, 0x34);
            c.lwc1(0, a);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            let a = oc830_ea(c, S2, 0x5c);
            c.g[V0][0] = c.lw(a);
            c.g[S7][0] = 0;
            c.mul_s(2, 2, 0);
            c.swc1(1, sp0.wrapping_add(0x3c));
            c.swc1(1, sp0.wrapping_add(0x30));
            c.swc1(1, sp0.wrapping_add(0x34));
            c.swc1(1, sp0.wrapping_add(0x38));
            c.lwc1(0, sp0.wrapping_add(0x14));
            c.lwc1(1, sp0.wrapping_add(0x24));
            let a = oc830_ea(c, V0, 0x23);
            c.ldl(4, a);
            let a = oc830_ea(c, V0, 0x1c);
            c.ldr(4, a);
            let a = oc830_ea(c, V0, 0x2b);
            c.ldl(6, a);
            let a = oc830_ea(c, V0, 0x24);
            c.ldr(6, a);
            c.sdl(4, sp0.wrapping_add(0x47));
            c.sdr(4, sp0.wrapping_add(0x40));
            c.sdl(6, sp0.wrapping_add(0x4f));
            c.sdr(6, sp0.wrapping_add(0x48));
            c.mul_s(0, 0, 2);
            c.g[V1][0] = oc830_gp_lw(c, 0x854);
            let a = oc830_ea(c, V1, 0xe84);
            c.g[A0][0] = c.lw(a);
            c.cvt_w_s(2, 0);
            c.mfc1(2, 2);
            let a = oc830_ea(c, A0, 0x8);
            c.g[V1][0] = c.lw(a);
            c.g[V0][0] = sx(oc830_ea(c, V0, 0x3));
            c.g[V1][0] &= c.g[A1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] |= 0x2a0;
            c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.sw(a, c.g[V1][0]);

            c.cycles += 13;
            c.mtc1(19, 0);
            c.cvt_s_w(0, 0);
            let a = oc830_ea(c, S2, 0x60);
            c.g[A0][0] = c.lw(a);
            c.div(0, 4, 19);
            c.div_s(1, 1, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] = c.hi[0];
            c.g[FP][0] = c.lo[0];
            c.g[S4][0] = c.g[V1][0];
            c.g[S6][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            if (c.g[S4][0] as i64) < 0 {
                c.cycles += 3;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
                c.cycles += 3;
                c.div(0, 2, 19);
                c.g[V1][0] = c.hi[0];
                c.g[S4][0] = c.g[V1][0];
            }

            // Draw the lines.
            c.cycles += 2;
            let no_lines = (c.g[S6][0] as i64) <= 0;
            c.g[S0][0] = 0;
            if !no_lines {
                c.cycles += 5;
                c.g[AT][0] = 0x4420_0000;
                c.mtc1(1, 23);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 22);
                let a = (c.g[GP][0] as u32).wrapping_sub(0x59ec);
                c.lwc1(21, a);
                loop {
                    'tail: {
                        'next: {
                            c.cycles += 9;
                            let a = oc830_ea(c, S2, 0x5c);
                            c.g[A0][0] = c.lw(a);
                            c.g[A1][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[FP][0] as u32));
                            c.mtc1(19, 20);
                            c.cvt_s_w(20, 20);
                            c.g[S5][0] = sx(oc830_ea(c, S0, 0x1));
                            call_ptr(c, 0x003a_2068, 0x0018_60f8)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = 0x20;
                            if c.g[S0][0] == 0 {
                                break 'next;
                            }
                            // Skip leading spaces.
                            c.cycles += 5;
                            let a = oc830_ea(c, S0, 0);
                            c.g[V0][0] = c.lhu(a);
                            c.mtc1(20, 1);
                            c.cvt_s_w(1, 1);
                            if c.g[V0][0] != c.g[V1][0] {
                                let a = oc830_ea(c, S2, 0x60);
                                c.g[V0][0] = c.lw(a);
                            } else {
                                c.cycles += 2;
                                c.g[S0][0] = sx(oc830_ea(c, S0, 0x2));
                                loop {
                                    c.cycles += 7;
                                    let a = oc830_ea(c, S0, 0);
                                    c.g[V0][0] = c.lhu(a);
                                    if c.g[V0][0] == c.g[V1][0] {
                                        c.g[S0][0] = sx(oc830_ea(c, S0, 0x2));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0018_6120);
                                        }
                                        continue;
                                    }
                                    c.cycles += 1;
                                    let a = oc830_ea(c, S2, 0x60);
                                    c.g[V0][0] = c.lw(a);
                                    break;
                                }
                            }
                            // Copy rect 0 to sp+0x50 and nudge its y by the
                            // sign of the scroll position.
                            c.cycles += 8;
                            c.ldl(4, sp0.wrapping_add(0x7));
                            c.ldr(4, sp0);
                            c.g[A1][0] = c.lw(sp0.wrapping_add(0x8));
                            c.sdl(4, sp0.wrapping_add(0x57));
                            c.sdr(4, sp0.wrapping_add(0x50));
                            c.sw(sp0.wrapping_add(0x58), c.g[A1][0]);
                            let le = (c.g[V0][0] as i64) <= 0;
                            c.lwc1(0, sp0.wrapping_add(0x54));
                            if le {
                                c.cycles += 1;
                                c.add_s(0, 0, 1);
                            } else {
                                c.cycles += 2;
                                c.sub_s(0, 0, 1);
                            }
                            // A '~' marker moves the marker widget at +0x58.
                            c.cycles += 5;
                            c.swc1(0, sp0.wrapping_add(0x54));
                            let a = oc830_ea(c, S0, 0);
                            c.g[V1][0] = c.lhu(a);
                            c.g[V0][0] = 0x7e;
                            let not_marker = c.g[V1][0] != c.g[V0][0];
                            c.lwc1(1, sp0.wrapping_add(0x10));
                            if !not_marker {
                                c.cycles += 3;
                                let a = oc830_ea(c, S2, 0x58);
                                c.g[A2][0] = c.lw(a);
                                c.g[A1][0] = sx(sp0.wrapping_add(0x60));
                                if c.g[A2][0] != 0 {
                                    c.cycles += 26;
                                    let a = oc830_ea(c, A2, 0x8);
                                    c.g[V1][0] = c.lw(a);
                                    c.g[S7][0] = 0x1;
                                    let a = oc830_ea(c, V1, 0xa0);
                                    c.g[A0][0] = c.lh(a);
                                    let a = oc830_ea(c, V1, 0xa4);
                                    c.g[V0][0] = c.lw(a);
                                    let target = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, target, 0x0018_61a4)?;
                                    c.lwc1(0, sp0.wrapping_add(0x60));
                                    c.g[A1][0] = 0x1;
                                    let a = oc830_ea(c, S2, 0x58);
                                    c.g[V0][0] = c.lw(a);
                                    c.sub_s(0, 23, 0);
                                    c.mul_s(0, 0, 22);
                                    c.swc1(0, sp0.wrapping_add(0x50));
                                    c.ldl(8, sp0.wrapping_add(0x57));
                                    c.ldr(8, sp0.wrapping_add(0x50));
                                    c.g[V1][0] = c.lw(sp0.wrapping_add(0x58));
                                    let a = oc830_ea(c, V0, 0x4b);
                                    c.sdl(8, a);
                                    let a = oc830_ea(c, V0, 0x44);
                                    c.sdr(8, a);
                                    let a = oc830_ea(c, V0, 0x4c);
                                    c.sw(a, c.g[V1][0]);
                                    let a = oc830_ea(c, S2, 0x58);
                                    c.g[A2][0] = c.lw(a);
                                    oc830_render_vcall(c, A2, V0, V1, 0x48, 0x0018_61ec)?;
                                    c.lwc1(0, sp0.wrapping_add(0x4));
                                    break 'tail;
                                }
                            }
                            // Position the run (a '^' skips the colour copy).
                            c.cycles += 7;
                            c.g[V1][0] = 0x5e;
                            c.lwc1(0, sp0.wrapping_add(0x14));
                            c.swc1(1, sp0.wrapping_add(0x60));
                            c.swc1(0, sp0.wrapping_add(0x64));
                            let a = oc830_ea(c, S0, 0);
                            c.g[V0][0] = c.lhu(a);
                            if c.g[V0][0] == c.g[V1][0] {
                                c.cycles += 10;
                                c.ldl(7, sp0.wrapping_add(0x37));
                                c.ldr(7, sp0.wrapping_add(0x30));
                                c.ldl(8, sp0.wrapping_add(0x3f));
                                c.ldr(8, sp0.wrapping_add(0x38));
                                let a = oc830_ea(c, S1, 0x47);
                                c.sdl(7, a);
                                let a = oc830_ea(c, S1, 0x40);
                                c.sdr(7, a);
                                let a = oc830_ea(c, S1, 0x4f);
                                c.sdl(8, a);
                                let a = oc830_ea(c, S1, 0x48);
                                c.sdr(8, a);
                                c.g[S0][0] = sx(oc830_ea(c, S0, 0x2));
                            } else {
                                c.cycles += 14;
                                c.ldl(4, sp0.wrapping_add(0x47));
                                c.ldr(4, sp0.wrapping_add(0x40));
                                c.ldl(5, sp0.wrapping_add(0x4f));
                                c.ldr(5, sp0.wrapping_add(0x48));
                                let a = oc830_ea(c, S1, 0x47);
                                c.sdl(4, a);
                                let a = oc830_ea(c, S1, 0x40);
                                c.sdr(4, a);
                                let a = oc830_ea(c, S1, 0x4f);
                                c.sdl(5, a);
                                let a = oc830_ea(c, S1, 0x48);
                                c.sdr(5, a);
                                c.lwc1(0, sp0.wrapping_add(0x60));
                                c.lwc1(1, sp0.wrapping_add(0x64));
                                c.sub_s(0, 0, 21);
                                c.sub_s(1, 1, 21);
                                c.swc1(0, sp0.wrapping_add(0x60));
                                c.swc1(1, sp0.wrapping_add(0x64));
                            }
                            // Measure and draw the run.
                            c.cycles += 27;
                            c.lwc1(12, sp0.wrapping_add(0x60));
                            c.g[A0][0] = c.g[S1][0];
                            c.lwc1(13, sp0.wrapping_add(0x64));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = sx(sp0.wrapping_add(0x70));
                            c.g[A3][0] = 0;
                            call_ptr(c, 0x0039_21f0, 0x0018_628c)?;
                            c.lwc1(12, sp0.wrapping_add(0x78));
                            c.g[A0][0] = c.g[S1][0];
                            c.lwc1(1, sp0.wrapping_add(0x60));
                            c.g[A1][0] = c.g[S0][0];
                            c.sub_s(12, 23, 12);
                            c.lwc1(13, sp0.wrapping_add(0x54));
                            c.mul_s(12, 12, 22);
                            c.swc1(12, sp0.wrapping_add(0x50));
                            let a = oc830_ea(c, S1, 0x30);
                            c.lwc1(0, a);
                            let a = oc830_ea(c, S1, 0x34);
                            c.lwc1(2, a);
                            c.mul_s(0, 0, 1);
                            let a = oc830_ea(c, S1, 0x38);
                            c.swc1(0, a);
                            c.lwc1(1, sp0.wrapping_add(0x64));
                            c.mul_s(2, 2, 1);
                            let a = oc830_ea(c, S1, 0x3c);
                            c.swc1(2, a);
                            call_ptr(c, 0x0039_1e30, 0x0018_62cc)?;
                            let a = oc830_ea(c, S1, 0x37);
                            c.ldl(8, a);
                            let a = oc830_ea(c, S1, 0x30);
                            c.ldr(8, a);
                            let a = oc830_ea(c, S1, 0x3f);
                            c.sdl(8, a);
                            let a = oc830_ea(c, S1, 0x38);
                            c.sdr(8, a);
                        }
                        c.cycles += 1;
                        c.lwc1(0, sp0.wrapping_add(0x4));
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.g[S5][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    c.add_s(0, 0, 20);
                    let more = c.g[V0][0] != 0;
                    c.swc1(0, sp0.wrapping_add(0x4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_60e0);
                    }
                }
            }

            // Reset the marker widget when no '~' was seen.
            c.cycles += 2;
            if c.g[S7][0] != 0 {
                let a = oc830_ea(c, S2, 0x5c);
                c.g[V1][0] = c.lw(a);
            } else {
                c.cycles += 3;
                let a = oc830_ea(c, S2, 0x58);
                c.g[A2][0] = c.lw(a);
                c.g[A1][0] = 0;
                if c.g[A2][0] != 0 {
                    c.cycles += 5;
                    oc830_render_vcall(c, A2, V0, V1, 0x48, 0x0018_631c)?;
                }
                c.cycles += 1;
                let a = oc830_ea(c, S2, 0x5c);
                c.g[V1][0] = c.lw(a);
            }
            // Wrap the scroll position into the text height.
            c.cycles += 8;
            c.mult(0, 5, 22, 19);
            let a = oc830_ea(c, S2, 0x60);
            c.g[A0][0] = c.lw(a);
            let a = oc830_ea(c, V1, 0xc8);
            c.g[V0][0] = c.lhu(a);
            c.mult(0, 2, 2, 19);
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let in_range = c.g[A0][0] == 0;
            c.g[V1][0] = sx(oc830_ea(c, V0, 0).wrapping_add(c.g[A1][0] as u32));
            if !in_range {
                c.cycles += 3;
                let a = oc830_ea(c, S2, 0x64);
                c.g[V0][0] = c.lw(a);
                if (c.g[V0][0] as i64) < 0 {
                    let a = oc830_ea(c, S2, 0x60);
                    c.sw(a, c.g[V1][0]);
                }
            }
            c.cycles += 4;
            let a = oc830_ea(c, S2, 0x60);
            c.g[V0][0] = c.lw(a);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                let a = oc830_ea(c, S2, 0x64);
                c.g[V0][0] = c.lw(a);
                let stopped = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                if !stopped {
                    c.cycles += 1;
                    let a = oc830_ea(c, S2, 0x60);
                    c.sw(a, c.g[V0][0]);
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e6b8, 0x0018_6374)?;
        frame.restore(c, 16, 0x110);
        frame.restore(c, 17, 0x100);
        frame.restore(c, 18, 0xf0);
        frame.restore(c, 19, 0xe0);
        frame.restore(c, 20, 0xd0);
        frame.restore(c, 21, 0xc0);
        frame.restore(c, 22, 0xb0);
        frame.restore(c, 23, 0xa0);
        frame.restore(c, 30, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 23, 0x138);
        frame.restore_fpr(c, 22, 0x130);
        frame.restore_fpr(c, 21, 0x128);
        frame.restore_fpr(c, 20, 0x120);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beprepoststate::tests::belib;

    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_2;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::fereal::tests::fake_slot;
    pub(crate) use crate::game::fe::fereal::tests::str_hash;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    use ssx3_ee as ee;

    use ssx3_ee::s02;

    #[test]
    fn cfe_options_credits_update_matches() {
        check(
            s02::f_001863b8__generated,
            cfe_options_credits_update,
            120,
            &|c, seed| {
                machine(c, seed, 0x1863b8, 1);
                globals(c);
                clear(c, OBJ, 0x100);
                c.g[A0][0] = sx(OBJ);
                c.bus.write32(OBJ + 0x60, 0x1000 + seed as u32);
                c.bus.write32(OBJ + 0x64, seed as u32 & 0x3f);
            },
        );
    }

    // --------------------------------------------------- rate: credits scroll

    /// A callee outside the translated code: answer null and return at once.
    /// The rate test below only looks at the scroll position.
    fn no_callee(c: &mut Cpu, _target: u32) {
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// The credits position after `updates` updates at rate `hz` with speed
    /// `speed`, the VSync counter running from 1 (so a faster tick sees its
    /// 60 Hz boundaries).
    fn credits_position(hz: u32, updates: u32, speed: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let _ = machine(&mut c, 0, 0x1863b8, 0);
        globals(&mut c);
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        clear(&mut c, OBJ, 0x100);
        c.bus.write32(OBJ + 0x64, speed);
        for i in 0..updates {
            c.bus.write32(app + 0x18, 1 + i);
            c.g[A0][0] = sx(OBJ);
            cfe_options_credits_update(&mut c);
            assert_eq!(c.pc, crate::difftest::RA, "the update did not return");
        }
        c.bus.read32(OBJ + 0x60)
    }

    /// The credits scroll advances one authored frame per 60 Hz boundary: four
    /// such frames of speed 3 move 12 px whether that is 4 updates at 60 Hz, 8
    /// at 120 or 16 at 240, and a 1 px/frame speed is not truncated away.
    #[test]
    fn cfe_options_credits_scroll_follows_the_rate() {
        assert_eq!(credits_position(60, 4, 3), 12, "60 Hz, speed 3");
        assert_eq!(credits_position(120, 8, 3), 12, "120 Hz, speed 3");
        assert_eq!(credits_position(240, 16, 3), 12, "240 Hz, speed 3");
        assert_eq!(credits_position(60, 4, 1), 4, "60 Hz, speed 1");
        assert_eq!(credits_position(120, 8, 1), 4, "120 Hz, speed 1");
        assert_eq!(credits_position(240, 16, 1), 4, "240 Hz, speed 1");
    }

    #[test]
    fn fe_state_event_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00186610__generated,
            fe_state_event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x186610, 0);
                be_library(c);
                // Below 2 the event-select constructor skips the BE interface.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 5 },
                );
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn fe_options_credits_on_widget_notify_matches() {
        check(
            ee::s02::f_00186478__generated,
            fe_options_credits_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0018_6478, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    #[test]
    fn cfe_options_credits_on_widget_action_matches() {
        check(
            ee::s02::f_001864b0__generated,
            cfe_options_credits_on_widget_action,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18_64b0, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
                c.g[A2][0] = if seed % 2 == 0 { 6 } else { (seed % 9) as u64 };
                // self+0x10 -> engine; engine+0 -> object; object+4 -> vtable.
                c.bus.write32(DATA + 0x10, DATA + 0x200);
                c.bus.write32(DATA + 0x200, DATA + 0x210);
                fake_slot(c, &mut r, DATA + 0x210, 0x4, 0x28);
            },
        );
    }

    #[test]
    fn cfe_state_on_deactivate_matches() {
        check(
            ee::s02::f_001865a8__generated,
            cfe_state_on_deactivate,
            40,
            &|c, seed| {
                base(c, seed, 0x18_65a8);
                let hash = str_hash(c, 0x004a_1398);
                c.g[A0][0] = sx(DATA);
                // The list at self+0x24: head at +0x28, sentinel check on +4.
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x28, 0);
                } else {
                    let screen = DATA + 0x300;
                    c.bus.write32(DATA + 0x28, screen);
                    c.bus.write32(screen + 0x4, DATA + 0x310);
                    if seed % 3 == 1 {
                        // FindObject walks a null array: returns null.
                        c.bus.write32(screen + 0x3c, 0);
                    } else {
                        let count = DATA + 0x600;
                        let array = DATA + 0x610;
                        let obj = DATA + 0x700;
                        c.bus.write32(screen + 0x38, count);
                        c.bus.write32(count, 4);
                        c.bus.write32(screen + 0x3c, array);
                        c.bus.write32(array, obj);
                        c.bus.write32(obj + 0x38, hash as u32);
                        c.bus.write8(obj + 0x95, 0x2a);
                    }
                }
                c.bus.write8(DATA + 0xc, 0x11);
            },
        );
    }

    /// `SSXAudioSystem_GetInstance`: returns `*(gp+0x410)`, 0 in tests.
    pub(crate) const RET_ZERO: u32 = 0x0028_b180;

    #[test]
    fn c_fe_options_credits_handle_input_matches() {
        check(
            ee::s02::f_001863e8__generated,
            c_fe_options_credits_handle_input,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1863, 0);
                env(c);
                let this = DATA + 0x400;
                let pad = DATA + 0x800;
                let vt = DATA + 0x1000;
                slot(c, vt, 0x88, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                slot(c, vt, 0x90, if seed % 3 == 0 { RET_ZERO } else { EMPTY });
                w32(c, pad + 8, vt);
                w32(
                    c,
                    this + 0x64,
                    [0i32, 1, -7, -8, 7, 8, 9][(seed % 7) as usize] as u32,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pad);
            },
        );
    }

    pub(crate) use crate::game::fe::festatebonusmaterial::tests::ui_state_case;

    #[test]
    fn c_fe_options_credits_ctor_2_matches() {
        ui_state_case(
            ee::s04::f_001de4b0__generated,
            c_fe_options_credits_ctor_2,
            0x001d_e4b0,
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x200000 and up, clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return.
    pub(crate) const OC830_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC830_MAGIC: u32 = 0x4f43_3830;

    /// Per-callee forced result words, one per callee address.
    pub(crate) const OC830_MAIL: u32 = DATA + 0x30_0000;

    /// Fake-call counters, `0x100000` above the mailbox words.
    pub(crate) const OC830_STRIDE: u32 = 0xffff_fff0;

    /// The all-fake vtable: entry `{delta 0, fn FAKE}` at every multiple of 8.
    pub(crate) const OC830_V: u32 = DATA + 0x21_0000;

    pub(crate) const OC830_OBJ: u32 = DATA + 0x21_1000;

    pub(crate) const OC830_W: u32 = DATA + 0x21_2000;

    pub(crate) const OC830_STR: u32 = DATA + 0x22_6000;

    pub(crate) fn oc830_mail(addr: u32) -> u32 {
        OC830_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc830_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc830_mail(addr), v.wrapping_add(3));
    }

    pub(crate) fn oc830_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc830_mail(addr), 1);
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns its forced
    /// result (a pointer by default; `OC830_STRIDE`: string `n % 8`).
    pub(crate) fn oc830_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc830_mail(addr));
        let n = c.bus.read32(oc830_mail(addr) + 0x10_0000);
        c.bus.write32(oc830_mail(addr) + 0x10_0000, n + 1);
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                OC830_STRIDE => (OC830_STR + 0x40 * (n % 8)) as u64,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc830_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC830_CTRL) != OC830_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc830_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc830_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc830_fakes!(
        (
            oc830_fake_39e390,
            OC830_PREV_39E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039e390
        ),
        (
            oc830_fake_397cc0,
            OC830_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc830_fake_39c870,
            OC830_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc830_fake_39d860,
            OC830_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc830_fake_3a12d0,
            OC830_PREV_3A12D0,
            ee::s13::OVERRIDE_003A12D0,
            ee::s13::f_003a12d0__generated,
            0x003a12d0
        ),
        (
            oc830_fake_3a2068,
            OC830_PREV_3A2068,
            ee::s13::OVERRIDE_003A2068,
            ee::s13::f_003a2068__generated,
            0x003a2068
        ),
        (
            oc830_fake_39e6b8,
            OC830_PREV_39E6B8,
            ee::s13::OVERRIDE_0039E6B8,
            ee::s13::f_0039e6b8__generated,
            0x0039e6b8
        ),
        (
            oc830_fake_3a1f18,
            OC830_PREV_3A1F18,
            ee::s13::OVERRIDE_003A1F18,
            ee::s13::f_003a1f18__generated,
            0x003a1f18
        ),
        (
            oc830_fake_317670,
            OC830_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc830_fake_3921f0,
            OC830_PREV_3921F0,
            ee::s12::OVERRIDE_003921F0,
            ee::s12::f_003921f0__generated,
            0x003921f0
        ),
        (
            oc830_fake_391e30,
            OC830_PREV_391E30,
            ee::s12::OVERRIDE_00391E30,
            ee::s12::f_00391e30__generated,
            0x00391e30
        ),
        (
            oc830_fake_417828,
            OC830_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc830_fake_186518,
            OC830_PREV_186518,
            ee::s02::OVERRIDE_00186518,
            ee::s02::f_00186518__generated,
            0x00186518
        ),
        (
            oc830_fake_227f80,
            OC830_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x00227f80
        ),
        (
            oc830_fake_241dc8,
            OC830_PREV_241DC8,
            ee::s06::OVERRIDE_00241DC8,
            ee::s06::f_00241dc8__generated,
            0x00241dc8
        ),
        (
            oc830_fake_14dc80,
            OC830_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc830_fake_14dd58,
            OC830_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc830_fake_152ba8,
            OC830_PREV_152BA8,
            ee::s01::OVERRIDE_00152BA8,
            ee::s01::f_00152ba8__generated,
            0x00152ba8
        ),
        (
            oc830_fake_2c26d0,
            OC830_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
    );

    /// Switch this machine's callee fakes on and build the shared vtable.
    pub(crate) fn oc830_arm(c: &mut Cpu) {
        c.bus.write32(OC830_CTRL, OC830_MAGIC);
        for off in (0..0x140).step_by(8) {
            c.bus.write16(OC830_V + off, 0);
            c.bus.write32(OC830_V + off + 4, fakes::FAKE);
        }
    }

    pub(crate) fn oc830_gp_set(c: &mut Cpu, off: u32, v: u32) {
        c.bus.write32(GAME_GP - off, v);
    }

    #[test]
    fn cfe_options_credits_init_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00185a98__generated,
            cfe_options_credits_init,
            48,
            &|c, seed| {
                machine(c, seed, 0x18_5a98, 8);
                oc830_arm(c);
                c.pc = 0x0018_5a98;
                c.g[A0][0] = sx(OC830_OBJ);
                c.bus.write32(OC830_OBJ + 0x10, DATA + 0x1234);
                if seed % 4 >= 2 {
                    oc830_force_zero(c, 0x0039_7cc0);
                }
                // FindObject: nothing, or a widget with the shared vtable.
                if seed % 2 == 1 {
                    oc830_force(c, 0x0039_d860, OC830_W);
                    c.bus.write32(OC830_W + 0x8, OC830_V);
                    c.bus.write32(OC830_W + 0x90, 0x10);
                } else {
                    oc830_force_zero(c, 0x0039_d860);
                }
            },
        );
    }

    #[test]
    fn cfe_options_credits_on_activate_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00185ba0__generated,
            cfe_options_credits_on_activate,
            40,
            &|c, seed| {
                machine(c, seed, 0x18_5ba0, 200);
                oc830_arm(c);
                c.pc = 0x0018_5ba0;
                c.g[A0][0] = sx(OC830_OBJ);
                c.g[A1][0] = sx(DATA + 0x77);
                if seed % 4 == 0 {
                    oc830_force_zero(c, 0x0039_d860);
                } else {
                    oc830_force(c, 0x0039_d860, OC830_W);
                    c.bus.write32(OC830_W + 0x8, OC830_V);
                }
            },
        );
    }

    #[test]
    fn cfe_options_credits_render_matches() {
        oc830_install_fakes();
        const T: u32 = DATA + 0x22_1000;
        const W2: u32 = DATA + 0x22_2000;
        const F: u32 = DATA + 0x22_3000;
        const G: u32 = DATA + 0x22_4000;
        const GO: u32 = DATA + 0x22_5000;
        const POOL: [u16; 6] = [0x20, 0x41, 0x42, 0x7e, 0x5e, 0x20];
        check_lohi(
            ee::s02::f_00185f40__generated,
            cfe_options_credits_render,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18_5f40, 60);
                oc830_arm(c);
                c.pc = 0x0018_5f40;
                let s = OC830_OBJ;
                c.g[A0][0] = sx(s);
                let sp0 = (c.g[SP][0] as u32).wrapping_sub(0x140);
                let no_text = seed % 11 == 0;
                c.bus.write32(s + 0x5c, if no_text { 0 } else { T });
                c.bus.write32(s + 0x58, if seed % 3 == 0 { 0 } else { W2 });
                c.bus
                    .write32(s + 0x60, (r.next_u32() % 200).wrapping_sub(100));
                c.bus
                    .write32(s + 0x64, [0xffff_fffd, 0, 5][seed as usize % 3]);
                // The text-scroll widget and its rectangles.
                c.bus.write32(T + 0x8, OC830_V);
                for off in [0x1c, 0x24, 0x44, 0x4c, 0x50, 0x58] {
                    c.bus.write32(T + off, r.next_u32());
                }
                c.bus.write32(
                    T + 0x54,
                    [0x3f80_0000, 0x4000_0000, 0x3f00_0000][seed as usize % 3],
                );
                c.bus.write16(T + 0xc8, (r.next_u32() % 50) as u16);
                c.bus.write32(W2 + 0x8, OC830_V);
                // The font: 1..4 lines of 1.0 height.
                c.bus.write32(F + 0x14, 1 + seed % 4);
                c.bus.write32(F + 0x34, 0x3f80_0000);
                for off in [0x30, 0x38, 0x3c, 0x40, 0x44, 0x48] {
                    c.bus.write32(F + off, r.next_u32());
                }
                if seed % 13 == 0 {
                    oc830_force_zero(c, 0x003a_12d0);
                } else {
                    oc830_force(c, 0x003a_12d0, F);
                }
                c.bus.write32(oc830_mail(0x003a_2068), OC830_STRIDE);
                // Globals: the glyph-flag word through gp-0x854, a float at gp-0x59ec.
                oc830_gp_set(c, 0x854, G);
                c.bus.write32(G + 0xe84, GO);
                c.bus.write32(GO + 0x8, 0x1234_5678);
                oc830_gp_set(c, 0x59ec, 0x3f00_0000);
                // Total height (float at sp+0x24): 6, 12, 30, -5 or 0.
                let h =
                    [0x40c0_0000u32, 0x4140_0000, 0x41f0_0000, 0xc0a0_0000, 0][seed as usize % 5];
                c.bus.write32(sp0 + 0x24, h);
                // Eight lines of text.
                for k in 0..8u32 {
                    for i in 0..6u32 {
                        let ch = POOL[(r.next_u32() % 6) as usize];
                        c.bus.write16(OC830_STR + 0x40 * k + 2 * i, ch);
                    }
                    c.bus.write16(OC830_STR + 0x40 * k + 12, 0);
                }
            },
        );
    }
}
