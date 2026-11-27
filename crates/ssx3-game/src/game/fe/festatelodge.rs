//! `src/fe/festatelodge.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopupmisc::oc969_sw_reg;
pub(crate) use crate::game::fe::festatebonusmaterial::widget_range_notify;
pub(crate) use crate::game::fe::ovstatehud::oc1002_vtable_dtor;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001f_3790, cfe_state_lodge_init, "cFEStateLodge_Init"),
    (
        0x001f_3cc8,
        fe_state_lodge_on_widget_notify,
        "cFEStateLodge_OnWidgetNotify",
    ),
    (
        0x001f_3ff8,
        c_fe_state_lodge_rider_detail_ctor,
        "cFEStateLodgeRiderDetail_ctor",
    ),
    (
        0x001f_4050,
        fe_state_lodge_rider_detail_init,
        "cFEStateLodgeRiderDetail_Init",
    ),
    (
        0x001f_4380,
        c_fe_state_lodge_rider_detail_on_render,
        "cFEStateLodgeRiderDetail_OnRender",
    ),
    (
        0x001f_43d8,
        fe_state_lodge_rider_detail_on_widget_notify,
        "cFEStateLodgeRiderDetail_OnWidgetNotify",
    ),
    (
        0x001f_3840,
        c_fe_state_lodge_on_widget_created,
        "cFEStateLodge_OnWidgetCreated",
    ),
    (
        0x001f_3a38,
        cfe_state_lodge_on_widget_action,
        "cFEStateLodge_OnWidgetAction",
    ),
    (
        0x001f_3cf0,
        cfe_state_lodge_on_child_event,
        "cFEStateLodge_OnChildEvent",
    ),
    (
        0x001f_4108,
        cfe_state_lodge_rider_detail_on_child_event,
        "cFEStateLodgeRiderDetail_OnChildEvent",
    ),
    (
        0x001f_4218,
        cfe_state_lodge_rider_detail_on_widget_created,
        "cFEStateLodgeRiderDetail_OnWidgetCreated",
    ),
    (
        0x001f_4400,
        cfe_state_lodge_rider_detail_on_widget_action,
        "cFEStateLodgeRiderDetail_OnWidgetAction",
    ),
    (
        0x001f_4728,
        cfe_state_buy_attrib_ctor,
        "cFEStateBuyAttrib_ctor",
    ),
    (0x0022_0d70, fe_state_lodge_dtor, "cFEStateLodge_dtor"),
    (
        0x0022_0dd0,
        fe_state_lodge_rider_detail_dtor,
        "cFEStateLodgeRiderDetail_dtor",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    pub(crate) use crate::game::dirtysock::tagsunk::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
}

/// `void* cFEStateLodgeRiderDetail_ctor(self)` (0x1f3ff8): run the UI state
/// ctor, install the class and copy the pad mask byte.
pub fn c_fe_state_lodge_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_400c);
        call_ptr(c, callee::UI_STATE_CTOR, 0x001f_400c)?;
        c.g[V0][0] = 0x28;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x39d8));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_4034);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_4034)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLodgeRiderDetail_OnRender(self)` (0x1f4380): when the detail
/// screen is not open, find its widget by name and reveal it.
pub fn c_fe_state_lodge_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x48).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x47_0000;
            c.g[RA][0] = sx(0x001f_43a8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1090));
            call_ptr(c, 0x0031_7670, 0x001f_43a8)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x40).lw(c);
            c.g[RA][0] = sx(0x001f_43b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_43b4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001f_43c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF, 0x001f_43c4)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateLodge_OnWidgetNotify(...)` (0x1F3CC8): ids 7..9 answer 0x100.
pub fn fe_state_lodge_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 7);
}

/// `int cFEStateLodgeRiderDetail_OnWidgetNotify(...)` (0x1F43D8): ids 8..9
/// answer 0x100.
pub fn fe_state_lodge_rider_detail_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 8);
}

/// `void cFEStateLodge_Init(self)` (0x1f3790): create the lodge screen, play
/// its first frame, select the lodge bank and start its music.
pub fn cfe_state_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2318));
        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_37b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_37c8)?;
        let missing = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_37e0)?;
        }
        c.cycles += 24;
        call_ptr(c, 0x0014_dc80, 0x001f_37e8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001f_37f4)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_74c8, 0x001f_3800)?;
        c.sw(p(c.g[S1][0] as u32, 0x48), c.g[V0][0]);
        c.g[V0][0] = 0x440000;
        c.sw(p(c.g[V0][0] as u32, 0x1b14), 0);
        call_ptr(c, 0x0028_b180, 0x001f_3810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_f140, 0x001f_381c)?;
        c.g[V0][0] = 0x12;
        c.sw(p(c.g[S1][0] as u32, 0x54), 0);
        c.sw(p(c.g[S1][0] as u32, 0x50), c.g[V0][0]);
        c.sw(p(c.g[S1][0] as u32, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: lodge rider detail.

/// `void cFEStateLodgeRiderDetail_Init(cFEStateLodgeRiderDetail* s)` (0x1f4050):
/// create the "rider detail" screen from its name hash, play its first frame,
/// then install the screen's rider name and the current peak's description.
pub fn fe_state_lodge_rider_detail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        'outer: {
            frame.save(c, S1, 0x10);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_ef20);
            c.g[RA][0] = sx(0x001f_4074);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_4074)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_4088);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_4088)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_40a0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_40a0)?;
            'mid: {
                c.cycles += 20;
                c.g[V0][0] = 1;
                c.g[RA][0] = sx(0x001f_40ac);
                Addr(p(r(c, S1), 0x48)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x0014_dc80, 0x001f_40ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40b8);
                c.g[A1][0] = 1;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40b8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[RA][0] = sx(0x001f_40c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7488, 0x001f_40c4)?;
                c.g[RA][0] = sx(0x001f_40cc);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dc80, 0x001f_40cc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40d8);
                c.g[A1][0] = 0xd;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40d8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40e8);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0015_77a0, 0x001f_40e8)?;
                if c.g[V0][0] == 0 {
                    Addr(p(r(c, S1), 0x48)).sw(c, 0);
                    break 'mid;
                }
                break 'mid;
            }
            break 'outer;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLodge_OnWidgetCreated(self, widget)` (0x1f3840): maps the created widget's name
/// hash (`widget+0x38`) to its slot index (`widget+0x18`: 0..5, 8, 6, 7 for the
/// ten known names); the eighth also shows/hides through its vtable, the
/// last one sets its text by the character level (`BENewPlayerInt_GetCharRecordField27C`).
///
/// Every register, cycle and call of the generated body is kept (labelled
/// blocks mirror its branches).
pub fn c_fe_state_lodge_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f3a24: {
            'b_1f3a20: {
                'b_1f3874: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[A0][0] = 0x470000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1240));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001f3864);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001f3864)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3874;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3874;
                        }
                    }
                }
                'b_1f3890: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f387c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1230));
                    call_ptr(c, 0x00317670, 0x001f387c)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3890;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3890;
                        }
                    }
                }
                'b_1f38ac: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3898);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1220));
                    call_ptr(c, 0x00317670, 0x001f3898)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f38ac;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f38ac;
                        }
                    }
                }
                'b_1f38c8: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f38b4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1210));
                    call_ptr(c, 0x00317670, 0x001f38b4)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f38c8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f38c8;
                        }
                    }
                }
                'b_1f38e4: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f38d0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1200));
                    call_ptr(c, 0x00317670, 0x001f38d0)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f38e4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x4;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f38e4;
                        }
                    }
                }
                'b_1f3900: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f38ec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11f0));
                    call_ptr(c, 0x00317670, 0x001f38ec)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3900;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3900;
                        }
                    }
                }
                'b_1f391c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3908);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11e0));
                    call_ptr(c, 0x00317670, 0x001f3908)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f391c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x8;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f391c;
                        }
                    }
                }
                'b_1f3938: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3924);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11d0));
                    call_ptr(c, 0x00317670, 0x001f3924)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3938;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x6;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_1f3a20;
                        } else {
                            break 'b_1f3938;
                        }
                    }
                }
                'b_1f3988: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f3940);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11c0));
                    call_ptr(c, 0x00317670, 0x001f3940)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f3988;
                    } else {
                        c.cycles += 16;
                        c.g[V0][0] = 0x7;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f3968);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f3968)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f3980);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f3980)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1f3a24;
                        } else {
                            break 'b_1f3988;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001f3990);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11b0));
                call_ptr(c, 0x00317670, 0x001f3990)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_1f3a24;
                } else {
                    'b_1f3a0c: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001f39a0);
                        call_ptr(c, 0x0014dc80, 0x001f39a0)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f39ac);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001f39ac)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f39b8);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00146d98, 0x001f39b8)?;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1f3a0c;
                        } else {
                            'b_1f39f0: {
                                'b_1f39dc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 19i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0x470000;
                                    if t {
                                        break 'b_1f39dc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11a0));
                                        if t {
                                            break 'b_1f39f0;
                                        } else {
                                            break 'b_1f39dc;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 21i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1f3a0c;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1188));
                                    break 'b_1f39f0;
                                }
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001f39f8);
                            call_ptr(c, 0x00317670, 0x001f39f8)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f3a04);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001f3a04)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1f3a24;
                            } else {
                                break 'b_1f3a0c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001f3a14);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1170));
                    call_ptr(c, 0x00317670, 0x001f3a14)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f3a20);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001f3a20)?;
                    break 'b_1f3a20;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1f3a24;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateLodge_OnWidgetAction(self, widget, event)`: a lodge menu button was
/// activated (event 5, ONNEXT). A locked entry does nothing. Entry 0 (save
/// progress) and entry 6 (quit) open a confirm popup unless a transition is
/// already running; entry 8 opens the profile-load state; any other entry plays
/// the screen's "TransitionOut" frame and pushes the state the custom
/// interface returns.
/// (0x001f3a38; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_lodge_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f3cac: {
            'b_1f3ca8: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[S0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_1f3ca8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    if c.g[A2][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1f3cac;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = 0x440000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1af8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        if c.g[A0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            break 'b_1f3cac;
                        } else {
                            'b_1f3be4: {
                                'b_1f3bd4: {
                                    'b_1f3b3c: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x001f3a90);
                                        call_ptr(c, 0x0014dc80, 0x001f3a90)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f3a9c);
                                        c.g[A1][0] = 0x4;
                                        call_ptr(c, 0x0014dd58, 0x001f3a9c)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        if c.g[V1][0] != 0u64 {
                                            c.g[S3][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                            break 'b_1f3b3c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                break 'b_1f3b3c;
                                            } else {
                                                c.cycles += 34;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1158));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x001f3ad4);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x001f3ad4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001f3ae4);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x001f3ae4)?;
                                                c.g[A1][0] = 0x470000;
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1140));
                                                c.g[A0][0] = 0xbfc;
                                                c.g[A2][0] = 0x100;
                                                c.g[RA][0] = sx(0x001f3b00);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001f3b00)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A3][0] = c.g[S0][0];
                                                c.sw(c.g[SP][0] as u32, 0u64);
                                                c.g[A2][0] = c.g[S2][0];
                                                c.g[T0][0] = 0u64;
                                                c.g[T1][0] = 0u64;
                                                c.g[T2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001f3b28);
                                                c.g[T3][0] = 0u64;
                                                Callee {
                                                    addr: 0x001d99c8,
                                                    f: ee::s04::cFEPopupConfirm_ctor,
                                                }
                                                .call(c, 0x001f3b28)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[V1][0] = 0x10;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x50),
                                                    c.g[V1][0],
                                                );
                                                let t = true;
                                                c.g[A1][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_1f3bd4;
                                                } else {
                                                    break 'b_1f3b3c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    let t = c.g[S3][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x8;
                                    if t {
                                        break 'b_1f3be4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_1f3be4;
                                        } else {
                                            c.cycles += 32;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1130));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001f3b74);
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            call_ptr(c, 0x00317670, 0x001f3b74)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001f3b84);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001f3b84)?;
                                            c.g[A1][0] = 0x470000;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x1140));
                                            c.g[A0][0] = 0xbfc;
                                            c.g[A2][0] = 0x100;
                                            c.g[RA][0] = sx(0x001f3ba0);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x001f3ba0)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A3][0] = c.g[S0][0];
                                            c.sw(c.g[SP][0] as u32, 0u64);
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[T0][0] = 0u64;
                                            c.g[T1][0] = 0u64;
                                            c.g[T2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001f3bc8);
                                            c.g[T3][0] = 0u64;
                                            Callee {
                                                addr: 0x001d99c8,
                                                f: ee::s04::cFEPopupConfirm_ctor,
                                            }
                                            .call(c, 0x001f3bc8)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x50),
                                                c.g[S3][0],
                                            );
                                            break 'b_1f3bd4;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001f3bdc);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f290, 0x001f3bdc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1f3cac;
                                } else {
                                    break 'b_1f3be4;
                                }
                            }
                            'b_1f3c4c: {
                                c.cycles += 2;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x470000;
                                    break 'b_1f3c4c;
                                } else {
                                    c.cycles += 24;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[T0][0] = 0x27;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A3][0] = 0u64;
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[A0][0] = 0x280;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1110));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001f3c14);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                                    call_ptr(c, 0x00317d70, 0x001f3c14)?;
                                    c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[A3][0] = 0x1;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A3][0] =
                                        sx((c.g[A3][0] as u32) << ((c.g[V1][0] as u32) & 31));
                                    c.g[A2][0] = 0x3;
                                    c.g[RA][0] = sx(0x001f3c34);
                                    c.g[A3][0] = c.g[A3][0] & 0xff;
                                    call_ptr(c, 0x0018eca0, 0x001f3c34)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001f3c44);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001f3c44)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_1f3cac;
                                    } else {
                                        break 'b_1f3c4c;
                                    }
                                }
                            }
                            c.cycles += 20;
                            c.g[RA][0] = sx(0x001f3c54);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                            call_ptr(c, 0x00317670, 0x001f3c54)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f3c60);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039c778, 0x001f3c60)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0] & 0xffff;
                            c.g[RA][0] = sx(0x001f3c70);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039c870, 0x001f3c70)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001f3c94);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001f3c94)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_1f3ca8;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001f3ca8);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x001f3ca8)?;
                                break 'b_1f3ca8;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1f3cac;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateLodge_OnChildEvent(self, msg, id)`: child-screen events of the lodge.
/// 0xf and 0x10 track whether a child popup is open (+0x54) and close the top
/// states; 0x10 then re-enters the screen (state 0x10 or 0x11); 0x16 opens the
/// profile-load or save-confirm popup chosen by the pending path (+0x50).
/// (0x001f3cf0; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_lodge_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f3fe8: {
            'b_1f3fe4: {
                'b_1f3fe0: {
                    'b_1f3e68: {
                        'b_1f3d80: {
                            c.cycles += 8;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S0][0] = 0x10;
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[S2][0] = c.g[A0][0];
                            let t = c.g[A2][0] == c.g[S0][0];
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                            if t {
                                break 'b_1f3d80;
                            } else {
                                'b_1f3d48: {
                                    'b_1f3d2c: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0xf;
                                        if t {
                                            break 'b_1f3d2c;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[V1][0] = 0x1;
                                            if t {
                                                break 'b_1f3d48;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f3fe4;
                                                } else {
                                                    break 'b_1f3d2c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x15;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x16;
                                    if t {
                                        break 'b_1f3fe0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[A2][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                            break 'b_1f3e68;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1f3fe4;
                                            } else {
                                                break 'b_1f3d48;
                                            }
                                        }
                                    }
                                }
                                'b_1f3d68: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                    c.g[V0][0] = 0x6;
                                    let t = c.g[A0][0] == c.g[V0][0];
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                                    if t {
                                        break 'b_1f3d68;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == c.g[S0][0];
                                        c.g[V0][0] = 0x11;
                                        if t {
                                            break 'b_1f3d68;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] != c.g[V0][0];
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1f3fe4;
                                            } else {
                                                break 'b_1f3d68;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.g[RA][0] = sx(0x001f3d78);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001f3d78)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f3fe4;
                                } else {
                                    break 'b_1f3d80;
                                }
                            }
                        }
                        'b_1f3df8: {
                            'b_1f3db0: {
                                c.cycles += 8;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x54), 0u64);
                                c.g[RA][0] = sx(0x001f3d94);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001f3d94)?;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                let t = c.g[V1][0] == c.g[S0][0];
                                c.g[V0][0] = 0x11;
                                if t {
                                    break 'b_1f3db0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_1f3df8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1f3fe8;
                                        } else {
                                            break 'b_1f3db0;
                                        }
                                    }
                                }
                            }
                            c.cycles += 18;
                            c.g[V0][0] = 0x1;
                            c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1240));
                            c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x98));
                            c.g[RA][0] = sx(0x001f3dd0);
                            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            call_ptr(c, 0x00317670, 0x001f3dd0)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f3ddc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f3ddc)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001f3df0);
                            c.g[A2][0] = 0x5;
                            call_ptr(c, t, 0x001f3df0)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1f3fe4;
                            } else {
                                break 'b_1f3df8;
                            }
                        }
                        c.cycles += 23;
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                        c.g[RA][0] = sx(0x001f3e0c);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001f3e0c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f3e18);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039c778, 0x001f3e18)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0] & 0xffff;
                        c.g[RA][0] = sx(0x001f3e28);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039c870, 0x001f3e28)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = 0x6;
                        c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f3e4c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f3e4c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[V0][0];
                        if t {
                            break 'b_1f3fe0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001f3e60);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f4c0, 0x001f3e60)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1f3fe4;
                            } else {
                                break 'b_1f3e68;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1f3fe4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            break 'b_1f3fe4;
                        } else {
                            'b_1f3f58: {
                                'b_1f3f28: {
                                    'b_1f3ef0: {
                                        'b_1f3eb4: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                            let t = c.g[V1][0] == c.g[S0][0];
                                            c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                                            if t {
                                                break 'b_1f3eb4;
                                            } else {
                                                'b_1f3ea0: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1f3ea0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        if t {
                                                            break 'b_1f3f58;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            if t {
                                                                break 'b_1f3fe4;
                                                            } else {
                                                                break 'b_1f3ea0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x11;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f3ef0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1f3fe8;
                                                    } else {
                                                        break 'b_1f3eb4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 15;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = 0x280;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1110));
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x001f3ecc);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001f3ecc)?;
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x15));
                                        c.g[RA][0] = sx(0x001f3ee0);
                                        c.g[A2][0] = 0x3;
                                        call_ptr(c, 0x0018eca0, 0x001f3ee0)?;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                        c.g[A2][0] = 0xffffffffffffc0ff;
                                        let t = true;
                                        c.g[V1][0] = 0x1;
                                        if t {
                                            break 'b_1f3f28;
                                        } else {
                                            break 'b_1f3ef0;
                                        }
                                    }
                                    c.cycles += 14;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A0][0] = 0x280;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1110));
                                    c.g[A2][0] = 0x100;
                                    c.g[RA][0] = sx(0x001f3f08);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001f3f08)?;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x15));
                                    c.g[RA][0] = sx(0x001f3f1c);
                                    c.g[A2][0] = 0x3;
                                    call_ptr(c, 0x0018eca0, 0x001f3f1c)?;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                    c.g[A2][0] = 0xffffffffffffc0ff;
                                    c.g[V1][0] = 0x2;
                                    break 'b_1f3f28;
                                }
                                c.cycles += 12;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[A0][0] & c.g[A2][0];
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), c.g[V1][0]);
                                c.g[A0][0] = c.g[A0][0] | 0x100;
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                                c.g[A3][0] = 0x1;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001f3f50);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f320, 0x001f3f50)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f3fe4;
                                } else {
                                    break 'b_1f3f58;
                                }
                            }
                            c.cycles += 34;
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10f8));
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x001f3f74);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00317670, 0x001f3f74)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001f3f84);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x001f3f84)?;
                            c.g[A1][0] = 0x470000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1140));
                            c.g[A0][0] = 0xbfc;
                            c.g[A2][0] = 0x100;
                            c.g[RA][0] = sx(0x001f3fa0);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001f3fa0)?;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0u64;
                            c.g[T2][0] = 0u64;
                            c.g[T3][0] = 0u64;
                            c.g[A3][0] = c.g[S0][0];
                            c.sw(c.g[SP][0] as u32, 0u64);
                            c.g[RA][0] = sx(0x001f3fc8);
                            c.g[A2][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001d99c8,
                                f: ee::s04::cFEPopupConfirm_ctor,
                            }
                            .call(c, 0x001f3fc8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = 0x11;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f3fe0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f290, 0x001f3fe0)?;
                            break 'b_1f3fe0;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_1f3fe4;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1f3fe8;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateLodgeRiderDetail_OnWidgetAction(self, widget, event)`: ONNEXT (5) on an
/// unlocked entry: 14 opens the name keyboard popup (8 characters, restricted
/// key set, current player name), 11 opens the cheat-character select when the
/// rider owns one, others play "TransitionOut" and push the next state. ONPREV
/// (6) pushes the previous state the custom interface returns.
/// (0x001f4400; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_lodge_rider_detail_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f471c: {
            'b_1f4718: {
                'b_1f4714: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    let t = c.g[S0][0] == 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        break 'b_1f4714;
                    } else {
                        'b_1f46dc: {
                            'b_1f4438: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                if t {
                                    break 'b_1f4438;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1f46dc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1f471c;
                                        } else {
                                            break 'b_1f4438;
                                        }
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = 0x440000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1af8));
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            let t = c.g[A0][0] == 0u64;
                            c.g[V0][0] = 0xe;
                            if t {
                                break 'b_1f4714;
                            } else {
                                'b_1f45e0: {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0xb;
                                    if t {
                                        break 'b_1f45e0;
                                    } else {
                                        c.cycles += 96;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = 0x444;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1030));
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x001f4478);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001f4478)?;
                                        c.g[T0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x15));
                                        c.g[A2][0] = c.g[S1][0];
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4490);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001cb030,
                                            f: ee::s04::cKeyboardPopup_ctor,
                                        }
                                        .call(c, 0x001f4490)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x8;
                                        c.g[V0][0] = 0x37;
                                        c.sb((c.g[S0][0] as u32).wrapping_add(0x44), 0u64);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f44ac);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001cd088,
                                            f: ee::s04::cKeyboardPopup_SetMaxLength,
                                        }
                                        .call(c, 0x001f44ac)?;
                                        c.g[A2][0] = 0x1;
                                        c.g[A3][0] = 0x2;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f44c0);
                                        c.g[A1][0] = 0x4b;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f44c0)?;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f44d4);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001ce408,
                                            f: ee::s04::cKeyboardPopup_SetDigitKeysDisabled,
                                        }
                                        .call(c, 0x001f44d4)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0xe;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f44e8);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f44e8)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x27;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f44fc);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f44fc)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x28;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4510);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4510)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x34;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4524);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4524)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x1a;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4538);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4538)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x19;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f454c);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f454c)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x35;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4560);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4560)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x40;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4574);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4574)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x41;
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001f4588);
                                        c.g[A3][0] = 0x2;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f4588)?;
                                        c.g[A2][0] = 0u64;
                                        c.g[A3][0] = 0x2;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f459c);
                                        c.g[A1][0] = 0x42;
                                        Callee {
                                            addr: 0x001ce3c8,
                                            f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                                        }
                                        .call(c, 0x001f459c)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f45ac);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f290, 0x001f45ac)?;
                                        c.g[RA][0] = sx(0x001f45b4);
                                        call_ptr(c, 0x0014dc80, 0x001f45b4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f45c0);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0014dd58, 0x001f45c0)?;
                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.g[RA][0] = sx(0x001f45cc);
                                        c.g[A0][0] = c.g[V0][0];
                                        call_ptr(c, 0x00147170, 0x001f45cc)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f45d8);
                                        c.g[A1][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001cb418,
                                            f: ee::s04::cKeyboardPopup_SetText,
                                        }
                                        .call(c, 0x001f45d8)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1f4718;
                                        } else {
                                            break 'b_1f45e0;
                                        }
                                    }
                                }
                                'b_1f4678: {
                                    c.cycles += 2;
                                    if c.g[A1][0] != c.g[V0][0] {
                                        c.g[A0][0] = 0x470000;
                                        break 'b_1f4678;
                                    } else {
                                        c.cycles += 19;
                                        c.g[RA][0] = sx(0x001f45f0);
                                        call_ptr(c, 0x0014dc80, 0x001f45f0)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f45fc);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0014dd58, 0x001f45fc)?;
                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.g[RA][0] = sx(0x001f4608);
                                        c.g[A0][0] = c.g[V0][0];
                                        call_ptr(c, 0x00147488, 0x001f4608)?;
                                        c.g[RA][0] = sx(0x001f4610);
                                        c.g[S0][0] = c.g[V0][0];
                                        call_ptr(c, 0x0014dc80, 0x001f4610)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f461c);
                                        c.g[A1][0] = 0xd;
                                        call_ptr(c, 0x0014dd58, 0x001f461c)?;
                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001f462c);
                                        c.g[A2][0] = c.g[S0][0];
                                        call_ptr(c, 0x001577a0, 0x001f462c)?;
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[A1][0] = 0x470000;
                                        if t {
                                            break 'b_1f4714;
                                        } else {
                                            c.cycles += 17;
                                            c.g[A0][0] = 0x1e4;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x1020));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001f4648);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x001f4648)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A3][0] =
                                                c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001f4660);
                                            c.g[A2][0] = c.g[S1][0];
                                            call_ptr(c, 0x00182220, 0x001f4660)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001f4670);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f290, 0x001f4670)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1f4718;
                                            } else {
                                                break 'b_1f4678;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[RA][0] = sx(0x001f4680);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x001f4680)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001f468c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x001f468c)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0] & 0xffff;
                                c.g[RA][0] = sx(0x001f469c);
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x0039c870, 0x001f469c)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f46c0);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f46c0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_1f4714;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001f46d4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001f46d4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1f4718;
                                    } else {
                                        break 'b_1f46dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001f4700);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f4700)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[V0][0];
                        if t {
                            break 'b_1f4714;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001f4714);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f400, 0x001f4714)?;
                            break 'b_1f4714;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1f4718;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1f471c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// One name test of `cFEStateLodgeRiderDetail_OnWidgetCreated`: the name hash
/// of `0x470000 - off` (a call to `0x317670`, returning to `ret_addr`) is
/// compared with the state's `+0x38` word. On a match `code` is stored at
/// `+0x18` (3 cycles) and true is returned.
pub(crate) fn oc969_widget_name_test(
    c: &mut Cpu,
    off: u32,
    ret_addr: u32,
    code: u64,
) -> Result<bool, crate::call::Unwound> {
    c.g[A0][0] = 0x47_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(off));
    call_ptr(c, 0x0031_7670, ret_addr)?;
    let matched = c.g[S0][0] == c.g[V0][0];
    c.g[A0][0] = 0x47_0000;
    if matched {
        c.cycles += 3;
        c.g[V0][0] = code;
        let store_at = (c.g[S1][0] as u32).wrapping_add(0x18);
        oc969_sw_reg(c, store_at, V0);
    }
    Ok(matched)
}

/// `cFEStateLodgeRiderDetail_OnChildEvent` (0x001f4108): the rider-detail
/// popup's child-event handler. On the popup-close event (a2 == 0x16) it
/// reads the popup's result (a name or a flag through the interface) and
/// forwards it to the owning state's virtual method at `+0x4c`.
pub fn cfe_state_lodge_rider_detail_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x00);
        c.g[S3][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_4130)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x001f_413c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = 0x16;
        let t = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x40);
        'e0: {
            if t {
                break 'e0;
            }
            'e1: {
                'e2: {
                    c.cycles += 6;
                    c.g[A0][0] = 0x47_0000;
                    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10c0));
                    call_ptr(c, 0x0031_7670, 0x001f_415c)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
                        break 'e2;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c);
                    let t = c.g[V0][0] == 0;
                    c.g[S1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x70)).lw(c);
                    if t {
                        break 'e1;
                    }
                    c.cycles += 17;
                    call_ptr(c, 0x0014_dc80, 0x001f_4178)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014_dd58, 0x001f_4184)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x44)).lb(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x0014_74a8, 0x001f_4198)?;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_41ac)?;
                    frame.restore(c, S0, 0x40);
                    break 'e0;
                }
                c.cycles += 2;
                let t = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x40);
                if t {
                    break 'e0;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x5c)).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                if t {
                    break 'e1;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041_6810, 0x001f_41d0)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x40);
                    break 'e0;
                }
                c.cycles += 9;
                c.g[A1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x44)).lb(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7138, 0x001f_41e8)?;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001f_41fc)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateLodgeRiderDetail_OnWidgetCreated` (0x001f4218): picks the
/// widget's code from its name hash. The state's name hash (`+0x38`) is
/// compared with the hashes of seven fixed names; the first match stores its
/// code at `+0x18`. With no match among the first seven, the eighth name
/// selects the rider's stat block and fills the stat fields from the
/// interface.
pub fn cfe_state_lodge_rider_detail_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0x00);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
        'e0: {
            'e1: {
                if oc969_widget_name_test(c, 0x0000_10b0, 0x001f_423c, 0x9)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_10a0, 0x001f_4258, 0xa)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1090, 0x001f_4274, 0xb)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1080, 0x001f_4290, 0xc)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1070, 0x001f_42ac, 0xd)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1060, 0x001f_42c8, 0xe)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1050, 0x001f_42e4, 0xf)? {
                    break 'e1;
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1040));
                call_ptr(c, 0x0031_7670, 0x001f_4300)?;
                if c.g[S0][0] != c.g[V0][0] {
                    frame.restore(c, S0, 0x20);
                    break 'e0;
                }
                c.cycles += 25;
                call_ptr(c, 0x0014_dc80, 0x001f_4310)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014_dd58, 0x001f_431c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0014_6e98, 0x001f_432c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7398, 0x001f_4338)?;
                c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                call_ptr(c, 0x0014_dc80, 0x001f_4344)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014_dd58, 0x001f_4350)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0014_eec8, 0x001f_4360)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_436c)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBuyAttrib_ctor` (0x001f4728): the buy-attribute state's
/// constructor. Sets the state's header and widget fields, then fills seven
/// per-stat experience pairs from the rider's stat profile (the stat value,
/// and that value divided by 5 as a signed byte).
pub fn cfe_state_buy_attrib_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x68));
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 6;
        frame.save_ra(c, 0x00);
        c.g[S5][0] = 5;
        call_ptr(c, 0x0039_e2a0, 0x001f_475c)?;
        c.g[V0][0] = 0x47_0000;
        c.g[V1][0] = 0x29;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3908));
        let this_a = (c.g[S2][0] as u32).wrapping_add(0xc);
        let v1_word = c.g[V1][0];
        Addr(this_a).sw(c, v1_word);
        let this_b = (c.g[S2][0] as u32).wrapping_add(0x8);
        let v0_word = c.g[V0][0];
        Addr(this_b).sw(c, v0_word);
        c.g[A1][0] = 0;
        Addr((c.g[S2][0] as u32).wrapping_add(0x44)).sb(c, 0);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x001f_4784)?;
        let flag_b = (c.g[S2][0] as u32).wrapping_add(0x15);
        let flag_v0 = c.g[V0][0];
        Addr(flag_b).sb(c, flag_v0);
        call_ptr(c, 0x0014_dc80, 0x001f_478c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x0014_dd58, 0x001f_4798)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x84));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x1c;
        call_ptr(c, 0x003e_6448, 0x001f_47ac)?;
        c.g[V0][0] = 0x48_0000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f50));
        loop {
            c.cycles += 14;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x0014_8158, 0x001f_47cc)?;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S3][0];
            let stat_at = (c.g[S1][0] as u32).wrapping_sub(0x1c);
            oc969_sw_reg(c, stat_at, V0);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_8158, 0x001f_47e0)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            let n = r32(c, V0);
            let d = r32(c, S5);
            let _ = crate::abi::div(c, n, d);
            c.cycles += 6;
            c.g[V0][0] = c.lo[0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            let level_at = c.g[S1][0] as u32;
            oc969_sw_reg(c, level_at, V0);
            let t = (c.g[S4][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !t {
                c.cycles += 10;
                c.g[V0][0] = c.g[S2][0];
                frame.restore(c, S0, 0x60);
                frame.restore(c, S1, 0x50);
                frame.restore(c, S2, 0x40);
                frame.restore(c, S3, 0x30);
                frame.restore(c, S4, 0x20);
                frame.restore(c, S5, 0x10);
                frame.restore_ra(c, 0x00);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x001f_47b8);
            }
        }
    })
}

/// `cFEStateLodge::~cFEStateLodge(this)` (0x220d70): installs the front-end
/// state vtable (0x46d0d0) and runs the `UIState` destructor.
pub fn fe_state_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0046_d0d0, 0x0022_0d8c))
}

/// `cFEStateLodgeRiderDetail::~cFEStateLodgeRiderDetail(this)` (0x220dd0): the
/// same shape as [`fe_state_lodge_dtor`] (vtable 0x46d0d0, return 0x220dec).
pub fn fe_state_lodge_rider_detail_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0046_d0d0, 0x0022_0dec))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::wb;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    use ssx3_ee as ee;

    #[test]
    fn c_fe_state_lodge_rider_detail_ctor_matches() {
        check(
            ee::s04::f_001f3ff8__generated,
            c_fe_state_lodge_rider_detail_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f3ff8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let module = DATA + 0x2000;
                clear(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_lodge_rider_detail_on_render_matches() {
        check(
            ee::s04::f_001f4380__generated,
            c_fe_state_lodge_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f4380, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let screen = DATA + 0x2000;
                clear(c, screen, 0x200);
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(this + 0x40, screen);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_lodge_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001f3cc8__generated,
            fe_state_lodge_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x001f_3cc8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    #[test]
    fn fe_state_lodge_rider_detail_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001f43d8__generated,
            fe_state_lodge_rider_detail_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x001f_43d8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    /// A screen-owning state: engine at +0x10, a screen created or not.
    pub(crate) fn screen_init(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = base(c, seed, salt, 2);
        c.g[A0][0] = OBJ as u64;
        w(c, OBJ + 0x10, OBJ2);
        w(c, OBJ + 0x40, 0);
        w(c, OBJ + 0x44, (seed % 4) as u32);
        w(c, OBJ2 + 0x10, OBJ3);
    }

    #[test]
    fn cfe_state_lodge_init_matches() {
        check(
            ee::s04::f_001f3790__generated,
            cfe_state_lodge_init,
            6,
            &|c, seed| screen_init(c, seed, 0x001f_3790),
        );
    }

    #[test]
    fn fe_state_lodge_rider_detail_init_matches() {
        check(
            ee::s04::f_001f4050__generated,
            fe_state_lodge_rider_detail_init,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x1f40, 5);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                wb(c, DATA + 0x44, (seed % 4) as u8);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::fepopupmisc::tests::oc962_lodge_fixture;

    pub(crate) use crate::game::fe::fepopupmisc::tests::oc969_setup;

    pub(crate) use crate::game::fe::fepopupmisc::tests::OC969_MBX;
    pub(crate) use crate::game::fe::fepopupmisc::tests::OC969_NULL;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `cFEStateLodge_OnWidgetCreated`: each widget name sets its slot index
    /// (the 8th also hides/shows through the vtable; the 10th picks a text by
    /// the character level), unknown names do nothing.
    #[test]
    fn c_fe_state_lodge_on_widget_created_matches() {
        check(
            ee::s04::f_001f3840__generated,
            c_fe_state_lodge_on_widget_created,
            176,
            &|c, seed| oc962_lodge_fixture(c, seed, 14),
        );
    }

    /// Gate word of this module's fakes: only machines holding the magic here
    /// take them (the fakes sit in shared override slots).
    pub(crate) const OC963_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC963_MAGIC: u32 = 0x0963_1f3a;

    /// Forced results (own mailbox): per callee `[flag, value]`.
    pub(crate) const OC963_FORCE: u32 = OC963_CTRL + 0x10;

    pub(crate) const OC963_F_OBJECT: u32 = 0;
    // UIScreen_FindObject (0x39d860)
    pub(crate) const OC963_F_OWNED: u32 = 1;
    // getNumOwnedCheatChars (0x1577a0)
    pub(crate) const OC963_F_COST: u32 = 2;
    // attribute level cost (0x150e50)
    pub(crate) const OC963_F_LOOKUP: u32 = 3;
    // UIScreen_LookupValue (0x39c778)

    pub(crate) const OC963_SELF: u32 = DATA + 0x20_1000;

    pub(crate) const OC963_ENG: u32 = DATA + 0x20_2000;

    pub(crate) const OC963_CUS: u32 = DATA + 0x20_3000;

    pub(crate) const OC963_CUSVT: u32 = DATA + 0x20_4000;

    pub(crate) const OC963_SCREEN: u32 = DATA + 0x20_5000;

    pub(crate) const OC963_WIDGET: u32 = DATA + 0x20_6000;

    pub(crate) const OC963_WVT: u32 = DATA + 0x20_7000;

    pub(crate) const OC963_MSG: u32 = DATA + 0x20_8000;

    pub(crate) const OC963_WORLD: u32 = DATA + 0x20_9000;

    pub(crate) const OC963_LOC: u32 = DATA + 0x20_a000;

    pub(crate) const OC963_LOCVT: u32 = DATA + 0x20_b000;

    pub(crate) const OC963_SELFVT: u32 = DATA + 0x20_c000;

    /// A real leaf that returns 0 (a vtable slot pointing here answers null).
    pub(crate) const OC963_RET_ZERO: u32 = 0x0028_b180;

    /// The lodge "unlocked" table.
    pub(crate) const OC963_UNLOCKED: u32 = 0x0044_1af8;

    pub(crate) fn w_2(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A vtable slot `{delta (halfword), method}` at `vt + off`.
    pub(crate) fn oc963_slot(c: &mut Cpu, vt: u32, off: u32, target: u32, delta: u16) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, target);
    }

    pub(crate) fn oc963_force(c: &mut Cpu, k: u32, v: u32) {
        c.bus.write32(OC963_FORCE + 8 * k, 1);
        c.bus.write32(OC963_FORCE + 8 * k + 4, v);
    }

    /// A method that answers null (a real leaf) or a nonzero pointer (a fake).
    pub(crate) fn oc963_method(r: &mut Rng) -> u32 {
        match r.next_u32() % 3 {
            0 => OC963_RET_ZERO,
            n => FAKE + 0x10 * n,
        }
    }

    /// A fake callee: logs the call like `fakes::fake_call`, then answers the
    /// forced result of the mailbox, if the test set one.
    pub(crate) fn oc963_hit(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let k = match addr {
            0x0039_d860 => OC963_F_OBJECT,
            0x0015_77a0 => OC963_F_OWNED,
            0x0015_0e50 => OC963_F_COST,
            0x0039_c778 => OC963_F_LOOKUP,
            _ => return,
        };
        if c.bus.read32(OC963_FORCE + 8 * k) != 0 {
            let v = c.bus.read32(OC963_FORCE + 8 * k + 4);
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc963_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC963_CTRL) != OC963_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc963_hit(c, $addr);
                }
            )*
            fn oc963_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc963_fakes!(
        (
            oc963_fake_0014dc80,
            OC963_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc963_fake_0014dd58,
            OC963_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc963_fake_00147170,
            OC963_PREV_00147170,
            ee::s01::OVERRIDE_00147170,
            ee::s01::f_00147170__generated,
            0x0014_7170
        ),
        (
            oc963_fake_00147488,
            OC963_PREV_00147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc963_fake_001577a0,
            OC963_PREV_001577A0,
            ee::s01::OVERRIDE_001577A0,
            ee::s01::f_001577a0__generated,
            0x0015_77a0
        ),
        (
            oc963_fake_00150e50,
            OC963_PREV_00150E50,
            ee::s01::OVERRIDE_00150E50,
            ee::s01::f_00150e50__generated,
            0x0015_0e50
        ),
        (
            oc963_fake_0018eca0,
            OC963_PREV_0018ECA0,
            ee::s02::OVERRIDE_0018ECA0,
            ee::s02::f_0018eca0__generated,
            0x0018_eca0
        ),
        (
            oc963_fake_00182220,
            OC963_PREV_00182220,
            ee::s02::OVERRIDE_00182220,
            ee::s02::f_00182220__generated,
            0x0018_2220
        ),
        (
            oc963_fake_00317670,
            OC963_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc963_fake_00317d70,
            OC963_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc963_fake_0039c778,
            OC963_PREV_0039C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc963_fake_0039c870,
            OC963_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc963_fake_0039f400,
            OC963_PREV_0039F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc963_fake_0039d860,
            OC963_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc963_fake_0039f190,
            OC963_PREV_0039F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc963_fake_0039f320,
            OC963_PREV_0039F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039_f320
        ),
        (
            oc963_fake_0039f4c0,
            OC963_PREV_0039F4C0,
            ee::s13::OVERRIDE_0039F4C0,
            ee::s13::f_0039f4c0__generated,
            0x0039_f4c0
        ),
        (
            oc963_fake_0039e508,
            OC963_PREV_0039E508,
            ee::s13::OVERRIDE_0039E508,
            ee::s13::f_0039e508__generated,
            0x0039_e508
        ),
        (
            oc963_fake_001d99c8,
            OC963_PREV_001D99C8,
            ee::s04::OVERRIDE_001D99C8,
            ee::s04::f_001d99c8__generated,
            0x001d_99c8
        ),
        (
            oc963_fake_001cb030,
            OC963_PREV_001CB030,
            ee::s04::OVERRIDE_001CB030,
            ee::s04::f_001cb030__generated,
            0x001c_b030
        ),
        (
            oc963_fake_001cb418,
            OC963_PREV_001CB418,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418
        ),
        (
            oc963_fake_001cd088,
            OC963_PREV_001CD088,
            ee::s04::OVERRIDE_001CD088,
            ee::s04::f_001cd088__generated,
            0x001c_d088
        ),
        (
            oc963_fake_001ce3c8,
            OC963_PREV_001CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc963_fake_001ce408,
            OC963_PREV_001CE408,
            ee::s04::OVERRIDE_001CE408,
            ee::s04::f_001ce408__generated,
            0x001c_e408
        ),
        (
            oc963_fake_001f50b0,
            OC963_PREV_001F50B0,
            ee::s04::OVERRIDE_001F50B0,
            ee::s04::f_001f50b0__generated,
            0x001f_50b0
        ),
        (
            oc963_fake_001f5240,
            OC963_PREV_001F5240,
            ee::s04::OVERRIDE_001F5240,
            ee::s04::f_001f5240__generated,
            0x001f_5240
        ),
    );

    /// The lodge-state fixture: the state at `OC963_SELF` with its engine, the
    /// custom interface (vtable slots 0x20 next / 0x28 previous state), the
    /// screen, the locale manager and the unlocked table (all entries open).
    pub(crate) fn oc963_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc963_install();
        let mut r = machine(c, seed, salt, 12);
        w_2(c, OC963_CTRL, OC963_MAGIC);
        w_2(c, GAME_GP.wrapping_sub(0x848), OC963_WORLD);
        w_2(c, OC963_WORLD + 0x8c, OC963_LOC);
        w_2(c, OC963_LOC + 4, OC963_LOCVT);
        oc963_slot(c, OC963_LOCVT, 0x20, FAKE + 0x40, 4 * (seed % 3) as u16);
        w_2(c, OC963_SELF + 0x10, OC963_ENG);
        w_2(c, OC963_SELF + 0x40, OC963_SCREEN);
        w_2(c, OC963_SELF + 8, OC963_SELFVT);
        w_2(c, OC963_ENG, OC963_CUS);
        w_2(c, OC963_CUS + 4, OC963_CUSVT);
        let next = oc963_method(&mut r);
        oc963_slot(c, OC963_CUSVT, 0x20, next, 8 * (seed % 2) as u16);
        let prev = oc963_method(&mut r);
        oc963_slot(c, OC963_CUSVT, 0x28, prev, 0);
        oc963_slot(c, OC963_SELFVT, 0x98, FAKE + 0x80, 4 * (seed % 5) as u16);
        w_2(c, OC963_WIDGET + 8, OC963_WVT);
        for i in 0..16 {
            w_2(c, OC963_UNLOCKED + 4 * i, 1);
        }
        c.bus.write8(OC963_SELF + 0x14, (r.next_u32() % 4) as u8);
        c.bus.write8(OC963_SELF + 0x15, (r.next_u32() % 16) as u8);
        c.g[A0][0] = sx(OC963_SELF);
        r
    }

    /// A random choice of a widget id: sometimes locked in the table.
    pub(crate) fn oc963_widget(c: &mut Cpu, r: &mut Rng, ids: &[u32]) -> u32 {
        let idx = ids[(r.next_u32() as usize) % ids.len()];
        w_2(c, OC963_WIDGET + 0x18, idx);
        if r.next_u32() % 6 == 0 {
            w_2(c, OC963_UNLOCKED + 4 * idx, 0);
        }
        c.g[A1][0] = if r.next_u32() % 12 == 0 {
            0
        } else {
            sx(OC963_WIDGET)
        };
        idx
    }

    /// Lodge widget action: the save-progress popup (entry 0), the quit popup
    /// (6), the profile load (8) and the generic transition, with locked
    /// entries, a null widget, other events, and a running transition.
    #[test]
    fn cfe_state_lodge_on_widget_action_matches() {
        check(
            ee::s04::f_001f3a38__generated,
            cfe_state_lodge_on_widget_action,
            240,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f3a38);
                oc963_widget(c, &mut r, &[0, 6, 8, 3]);
                w_2(c, OC963_SELF + 0x4c, r.next_u32() % 2);
                c.g[A2][0] = if r.next_u32() % 6 == 0 { 4 } else { 5 };
            },
        );
    }

    /// Lodge child events 0xf, 0x10, 0x15, 0x16 and others, for each pending
    /// path (6, 0x10, 0x11, other), with the popup-open flag set and clear.
    #[test]
    fn cfe_state_lodge_on_child_event_matches() {
        check(
            ee::s04::f_001f3cf0__generated,
            cfe_state_lodge_on_child_event,
            300,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f3cf0);
                let id =
                    [0xf, 0x10, 0x15, 0x16, 0x7, 0x10, 0x16, 0x16][(r.next_u32() % 8) as usize];
                let state = [6, 0x10, 0x11, 3][(r.next_u32() % 4) as usize];
                w_2(c, OC963_SELF + 0x50, state);
                w_2(c, OC963_SELF + 0x54, r.next_u32() % 2);
                w_2(c, OC963_MSG + 0xc, (r.next_u32() % 5 == 0) as u32);
                c.g[A1][0] = sx(OC963_MSG);
                c.g[A2][0] = id as u64;
            },
        );
    }

    /// Rider-detail widget action: ONNEXT on entries 14 (name keyboard), 11
    /// (cheat character select, owned or not) and others; ONPREV; locked
    /// entries and a null widget.
    #[test]
    fn cfe_state_lodge_rider_detail_on_widget_action_matches() {
        check(
            ee::s04::f_001f4400__generated,
            cfe_state_lodge_rider_detail_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f4400);
                oc963_widget(c, &mut r, &[14, 11, 3, 14, 11, 5]);
                let owned = [0, 1, 3][(r.next_u32() % 3) as usize];
                oc963_force(c, OC963_F_OWNED, owned);
                c.g[A2][0] = [5u64, 5, 5, 6, 7][(r.next_u32() % 5) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_lodge_rider_detail_on_child_event_matches() {
        check(
            ee::s04::f_001f4108__generated,
            cfe_state_lodge_rider_detail_on_child_event,
            300,
            &|c, seed| {
                oc969_setup(c);
                let mut r = fakes::machine(c, seed, 0x969_0006, 6);
                let rider = DATA + 0x6000;
                let state = DATA + 0x6100;
                let q = DATA + 0x6200;
                let iface = DATA + 0x6300;
                let vt = DATA + 0x6400;
                let inner = DATA + 0x6500;
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(state);
                c.g[A2][0] = if seed % 2 == 0 {
                    0x16
                } else {
                    r.next_u32() as u64 % 40
                };
                c.bus.write32(rider + 0x44, r.next_u32());
                c.bus
                    .write32(state + 0xc, if seed % 3 == 0 { 0 } else { q });
                c.bus
                    .write32(state + 0x5c, if seed % 5 == 0 { 0 } else { inner });
                c.bus.write32(state + 0x70, inner + 0x40);
                c.bus
                    .write32(state + 0x74, if seed % 4 == 0 { 0 } else { inner + 0x80 });
                c.bus.write32(OC969_MBX + 4, iface);
                c.bus
                    .write32(OC969_MBX + 12, if seed % 2 == 0 { q } else { 0 });
                c.bus
                    .write32(OC969_MBX + 20, if seed % 2 == 0 { OC969_NULL } else { 0 });
                c.bus.write32(iface + 0xc, vt);
                c.bus.write16(vt + 0x8, 0);
                c.bus.write32(vt + 0xc, fakes::FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_lodge_rider_detail_on_widget_created_matches() {
        check(
            ee::s04::f_001f4218__generated,
            cfe_state_lodge_rider_detail_on_widget_created,
            300,
            &|c, seed| {
                oc969_setup(c);
                fakes::machine(c, seed, 0x969_0007, 12);
                let state = DATA + 0x6600;
                let value = 0x1230_0000 + seed;
                // The step whose name hash matches (0..7; 8 = no match).
                let k = seed % 9;
                c.g[A0][0] = sx(DATA + 0x6700);
                c.g[A1][0] = sx(state);
                c.bus.write32(state + 0x38, value);
                if k < 8 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * k, value);
                }
            },
        );
    }

    #[test]
    fn cfe_state_buy_attrib_ctor_matches() {
        check(
            ee::s04::f_001f4728__generated,
            cfe_state_buy_attrib_ctor,
            200,
            &|c, seed| {
                oc969_setup(c);
                fakes::machine(c, seed, 0x969_0008, 12);
                let this = DATA + 0x6a00;
                let field = DATA + 0x6c00;
                let inner = DATA + 0x6d00;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, field);
                c.bus.write32(field, inner);
                c.bus.write32(inner, seed);
            },
        );
    }

    use crate::difftest::RA as DT_RA;
    pub(crate) use crate::game::fe::ovstatehud::tests::oc1002_arm;

    pub(crate) use crate::game::fe::ovstatehud::tests::oc1002_install_fakes;

    pub(crate) use crate::game::fe::ovstatehud::tests::OC1002_OBJ;

    /// `cFEStateLodge_dtor` (0x220d70): vtable store, `UIState_dtor` call.
    #[test]
    fn fe_state_lodge_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_00220d70__generated,
            fe_state_lodge_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0d70, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    /// `cFEStateLodgeRiderDetail_dtor` (0x220dd0): vtable store, `UIState_dtor` call.
    #[test]
    fn fe_state_lodge_rider_detail_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_00220dd0__generated,
            fe_state_lodge_rider_detail_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0dd0, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }
}
