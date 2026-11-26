//! `src/fe/festateprofile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::festatenethelppopup::fe_state_screen_init;
pub(crate) use crate::game::fe::festateoptions::call_object_a2;
pub(crate) use crate::game::fe::festateoptions::fe_options_dtor;
pub(crate) use crate::game::fe::festateoptions::fe_options_ui_ctor;
#[cfg(test)]
pub(crate) use crate::game::fe::festateoptions::fe_setup;
pub(crate) use crate::game::fe::festateoptions::screen_object;
pub(crate) use crate::game::fe::festateoptions::ui_state_epilogue;
pub(crate) use crate::game::fe::festateoptions::ui_state_frame;
pub(crate) use crate::game::fe::festateoptions::FE_MEMCARD_STATE_DTOR;
#[cfg(test)]
pub(crate) use crate::game::fe::festateoptions::FE_SEEDS;
pub(crate) use crate::game::fe::festateoptions::FE_UI_STATE_DTOR;
pub(crate) use crate::game::fe::festateriderbiodetail::OC846_BIO_SCREEN;
pub(crate) use crate::game::fe::festateriderbiodetail::OC846_GET_OBJECT;
pub(crate) use crate::game::fe::festateriderbiodetail::OC846_HASH;
pub(crate) use crate::game::fe::festateriderbiodetail::OC846_HI;
pub(crate) use crate::game::fe::festateriderbiodetail::OC846_LO;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_ead8,
        fe_screen_select_profile_init,
        "FEScreenSelectProfile_Init",
    ),
    (
        0x0018_eeb8,
        cfe_state_profile_load_on_widget_notify,
        "cFEStateProfileLoad_OnWidgetNotify",
    ),
    (
        0x0018_ef70,
        c_fe_state_profile_load_v37,
        "cFEStateProfileLoad_v37",
    ),
    (
        0x0018_f0b8,
        cfe_state_profile_load_v38,
        "cFEStateProfileLoad_v38",
    ),
    (
        0x0018_f278,
        c_fe_state_profile_load_v26,
        "cFEStateProfileLoad_v26",
    ),
    (
        0x0018_faf0,
        c_fe_state_profile_load_v33,
        "cFEStateProfileLoad_v33",
    ),
    (
        0x0019_0858,
        c_fe_state_rider_bio_ctor,
        "cFEStateRiderBio_ctor",
    ),
    (
        0x001d_e1b8,
        fe_screen_select_profile_ctor_2,
        "FEScreenSelectProfile_ctor_2",
    ),
    (
        0x001d_e1f0,
        fe_screen_select_profile_dtor,
        "FEScreenSelectProfile_dtor",
    ),
    (
        0x001d_e218,
        cfe_state_profile_load_dtor,
        "cFEStateProfileLoad_dtor",
    ),
    (
        0x0018_eb40,
        fe_select_profile_on_widget_action,
        "FEScreenSelectProfile_OnWidgetAction",
    ),
    (
        0x0018_eca0,
        cfe_state_profile_load_ctor,
        "cFEStateProfileLoad_ctor",
    ),
    (
        0x0018_edc8,
        cfe_state_profile_load_init,
        "cFEStateProfileLoad_Init",
    ),
    (
        0x0018_ee98,
        cfe_state_profile_load_render,
        "cFEStateProfileLoad_Render",
    ),
    (
        0x0018_f168,
        oc845_profile_load_v39,
        "cFEStateProfileLoad_v39",
    ),
    (
        0x0018_f2b8,
        fe_profile_load_refresh_profile_list,
        "cFEStateProfileLoad_RefreshProfileList",
    ),
    (
        0x0018_f9d8,
        oc845_profile_load_refresh_slots,
        "cFEStateProfileLoad_RefreshSlots",
    ),
    (
        0x0018_fc70,
        oc845_profile_load_shutdown,
        "cFEStateProfileLoad_Shutdown",
    ),
    (
        0x0018_fc98,
        oc845_profile_load_begin_card_op,
        "cFEStateProfileLoad_BeginCardOp",
    ),
    (
        0x0018_fd90,
        cfe_state_profile_load_on_widget_action,
        "cFEStateProfileLoad_OnWidgetAction",
    ),
    (
        0x0019_02c0,
        oc845_profile_load_setup_titles,
        "cFEStateProfileLoad_SetupTitles",
    ),
    (
        0x0019_0540,
        oc845_profile_load_show_new_profile_title,
        "cFEStateProfileLoad_ShowNewProfileTitle",
    ),
    (
        0x0019_0670,
        cfe_state_profile_load_show_load_title,
        "cFEStateProfileLoad_ShowLoadTitle",
    ),
    (
        0x0019_0768,
        cfe_state_profile_load_show_save_title,
        "cFEStateProfileLoad_ShowSaveTitle",
    ),
];

// ---------------------------------------------------------------------------
// Callees, under the names the generated bodies give them.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
}

/// `void cFEStateProfileLoad_v26(this)` (0x18f278): refresh the profile list
/// when the state at `+0x214` is 3, the slot list otherwise.
pub fn c_fe_state_profile_load_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x214)).lw(c);
        let not_three = c.g[V0][0] != c.g[V1][0];
        if not_three {
            c.cycles += 3;
            callee::FE_REFRESH_SLOTS.call(c, 0x0018_f2a8)?;
        } else {
            c.cycles += 4;
            callee::FE_REFRESH_PROFILE_LIST.call(c, 0x0018_f298)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn sb(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sb(c, v);
}

/// `FEScreenSelectProfile_Init(FEScreenSelectProfile* self)` (0x18ead8).
pub fn fe_screen_select_profile_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_e240, [0x0018_eafc, 0x0018_eb10, 0x0018_eb28])
    })
}

/// `cFEStateRiderBio_ctor(cFEStateRiderBio* self, int, int rider)`
/// (0x190858): construct the UI state, install the vtable, remember the rider
/// index and its pad mask.
pub fn c_fe_state_rider_bio_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x0019_0878)?;
        c.g[V0][0] = 0xf;
        c.g[V1][0] = 0x0047_0000;
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        c.g[V1][0] = 0x0046_b160;
        sw(c, p(this as u32, 8), c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        sb(c, p(this as u32, 0x44), c.g[S1][0]);
        c.g[V0][0] = lw(c, p(this as u32, 0x10));
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        call_ptr(c, 0x001a_1cd0, 0x0019_08a0)?;
        sb(c, p(this as u32, 0x15), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateProfileLoad_v38` (0x18f0b8): when the profile mode is not 2,
// toggle the two option widgets (`on` selects `!on`) and remember the state.
pub fn cfe_state_profile_load_v38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
        frame.save(c, S1, 0x20);
        c.g[V0][0] = 0x2;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let skip = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if !skip {
            c.cycles += 25;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.g[S2][0] ^ 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1458));
            call_ptr(c, 0x00317670, 0x0018_f0f0)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018_f0fc)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_f114)?;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1460));
            call_ptr(c, 0x00317670, 0x0018_f120)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018_f12c)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_f144)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0] ^ 0x1;
        c.lq(16, frame.at(c, 0x30));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x220), c.g[V0][0]);
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateProfileLoad_OnWidgetNotify(self, item, unsigned int msg)`
/// (0x18eeb8): map a profile-load widget notification to the state that
/// should handle it.
pub fn cfe_state_profile_load_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
        let msg = c.g[A2][0];
        c.g[S0][0] = msg;
        let result: u64;
        'outer: {
            'b58: {
                'b54: {
                    'b4c: {
                        'b38: {
                            call_ptr(c, 0x0022_7f80, 0x0018_eed8)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[V0][0] = 8;
                            let is8 = c.g[S0][0] == 8;
                            c.g[V0][0] = (c.g[S0][0] < 9) as u64;
                            if is8 {
                                break 'b38;
                            }
                            'b14: {
                                'b00: {
                                    c.cycles += 2;
                                    let big = c.g[V0][0] == 0;
                                    c.g[V0][0] = 6;
                                    if big {
                                        break 'b00;
                                    }
                                    c.cycles += 2;
                                    let is6 = c.g[S0][0] == 6;
                                    c.g[V0][0] = 0x101;
                                    if is6 {
                                        break 'b14;
                                    }
                                    c.cycles += 2;
                                    result = c.g[V0][0];
                                    frame.restore(c, S0, 0x20);
                                    break 'outer;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 9;
                                let is9 = c.g[S0][0] == 9;
                                c.g[V0][0] = 0x101;
                                if is9 {
                                    break 'b54;
                                }
                                c.cycles += 2;
                                result = c.g[V0][0];
                                frame.restore(c, S0, 0x20);
                                break 'outer;
                            }
                            c.cycles += 9;
                            c.g[A2][0] = Addr(p(c.g[V1][0] as u32, 0x434)).lw(c);
                            c.g[A1][0] = Addr(p(c.g[V1][0] as u32, 0x428)).lw(c);
                            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x1b0)).lh(c);
                            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1b4)).lw(c);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0018_ef30)?;
                            break 'b4c;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr(p(r32(c, GP), 0xffff_e4f8)).lw(c);
                        c.g[V0][0] = 2;
                        let two = c.g[V1][0] == 2;
                        c.g[V0][0] = 0x100;
                        if two {
                            break 'b58;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x220)).lw(c);
                        break 'b4c;
                    }
                    c.cycles += 2;
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x101;
                    if nonzero {
                        break 'b58;
                    }
                    break 'b54;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x100;
                break 'b58;
            }
            c.cycles += 1;
            result = c.g[V0][0];
            frame.restore(c, S0, 0x20);
        }
        c.g[V0][0] = result as u64;
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0018eb40 FEScreenSelectProfile_OnWidgetAction
// ---------------------------------------------------------------------------

/// `void FEScreenSelectProfile_OnWidgetAction(void* self, void* menu,
/// int action)` (0x18eb40).
///
/// Handles a widget action on a profile-select menu row: the menu object is
/// recoloured (white for "activate", magenta for "cancel"), and for the two
/// item actions the widget's own action method is called and its result
/// pushed into the front-end item list at `*(self+0x10) + 0x18`.
pub fn fescreen_select_profile_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A1][0]; // menu
        frame.save_ra(c, 0x10);
        let no_menu = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0]; // self
        'skip: {
            if !no_menu {
                {
                    'item: {
                        c.cycles += 3;
                        c.g[V0][0] = 2;
                        let is_two = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = (c.g[A2][0] < 3) as u64;
                        if is_two {
                            // Case 2: white.
                            c.cycles += 12;
                            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
                            c.g[A1][0] = c.g[SP][0];
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(1, 0);
                            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
                            c.swc1(0, p(c.g[SP][0] as u32, 0xc));
                            c.swc1(0, c.g[SP][0] as u32);
                            c.swc1(0, p(c.g[SP][0] as u32, 4));
                            c.swc1(0, p(c.g[SP][0] as u32, 8));
                            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0018_ec8c);
                            c.g[A0][0] = sx(c.g[S0][0] as u32 + c.g[A0][0] as u32);
                            call_ptr(c, t, 0x0018_ec8c)?;
                            break 'skip;
                        }
                        'low: {
                            c.cycles += 2;
                            let below3 = c.g[V0][0] == 0; // action >= 3
                            c.g[V0][0] = 1;
                            if below3 {
                                break 'low;
                            }
                            c.cycles += 2;
                            if c.g[A2][0] == c.g[V0][0] {
                                // Case 1: magenta.
                                c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
                                break 'item;
                            }
                            c.cycles += 2;
                            c.lq(16, p(c.g[SP][0] as u32, 0x30));
                            c.cycles += 4;
                            frame.restore(c, 17, 0x20);
                            frame.restore_ra(c, 0x10);
                            frame.close(c);
                            return Ok(());
                        }
                        // Cases 5 and 6: run the menu item's action method.
                        'five_six: {
                            'five: {
                                'six: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 5;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
                                        break 'six;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 6;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                                        break 'five;
                                    }
                                    c.cycles += 2;
                                    c.lq(16, p(c.g[SP][0] as u32, 0x30));
                                    c.cycles += 4;
                                    frame.restore(c, 17, 0x20);
                                    frame.restore_ra(c, 0x10);
                                    frame.close(c);
                                    return Ok(());
                                }
                                // Case 5: yellow highlight plus the action call.
                                c.cycles += 19;
                                c.g[A1][0] = c.g[SP][0];
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 0);
                                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
                                c.swc1(0, p(c.g[SP][0] as u32, 4));
                                c.swc1(0, c.g[SP][0] as u32);
                                c.sw(p(c.g[SP][0] as u32, 8), 0);
                                c.sw(p(c.g[SP][0] as u32, 0xc), 0);
                                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0018_ebcc);
                                c.g[A0][0] = sx(c.g[S0][0] as u32 + c.g[A0][0] as u32);
                                call_ptr(c, t, 0x0018_ebcc)?;
                                c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = c.lw(p(c.g[S0][0] as u32, 0x18));
                                c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                                c.g[V1][0] = c.lw(p(c.g[A3][0] as u32, 4));
                                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x20));
                                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x24));
                                break 'five_six;
                            }
                            // Case 6: just the action call.
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw(p(c.g[S0][0] as u32, 0x18));
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw(p(c.g[A3][0] as u32, 4));
                            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x28));
                            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x2c));
                        }
                        // Shared tail: call the item action and link the result.
                        c.cycles += 4;
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0018_ec0c);
                        c.g[A0][0] = sx(c.g[A3][0] as u32 + c.g[A0][0] as u32);
                        call_ptr(c, t, 0x0018_ec0c)?;
                        let none = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[V0][0];
                        if none {
                            break 'skip;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                        c.g[RA][0] = sx(0x0018_ec20);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f400, 0x0018_ec20)?;
                        c.lq(16, p(c.g[SP][0] as u32, 0x30));
                        c.cycles += 4;
                        frame.restore(c, 17, 0x20);
                        frame.restore_ra(c, 0x10);
                        frame.close(c);
                        return Ok(());
                    }
                    // Case 1: magenta.
                    c.cycles += 13;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
                    c.swc1(0, p(c.g[SP][0] as u32, 8));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.sw(p(c.g[SP][0] as u32, 4), 0);
                    c.sw(p(c.g[SP][0] as u32, 0xc), 0);
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0018_ec54);
                    c.g[A0][0] = sx(c.g[S0][0] as u32 + c.g[A0][0] as u32);
                    call_ptr(c, t, 0x0018_ec54)?;
                    break 'skip;
                }
            }
        }
        c.cycles += 1;
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.cycles += 4;
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_v37(FEState *s, int selected)` (0x18ef70): the
/// profile-load screen's variant, which skips the second widget while
/// `gp-0x1b08 == 2`.
pub fn c_fe_state_profile_load_v37(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = ui_state_frame(c);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x234).lw(c);
        let same = c.g[V0][0] == c.g[S2][0];
        c.g[S1][0] = c.g[S2][0] ^ 1;
        let this = r32(c, S0);
        let arg = c.g[S1][0];
        if !same {
            c.cycles += 16;
            screen_object(c, this, 0x004a_15e0, 0x0018_efa4, 0x0018_efb0)?;
            let obj = c.g[V0][0];
            call_object_a2(c, obj, arg, 0x0018_efc8)?;

            c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x1b08)).lw(c);
            c.g[V0][0] = 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 12;
                screen_object(c, this, 0x004a_15d8, 0x0018_f078, 0x0018_f084)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_f09c)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            } else {
                c.cycles += 38;
                screen_object(c, this, 0x004a_15d0, 0x0018_efe4, 0x0018_eff0)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_f008)?;
                screen_object(c, this, 0x004a_1458, 0x0018_f014, 0x0018_f020)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_f038)?;
                screen_object(c, this, 0x004a_1460, 0x0018_f044, 0x0018_f050)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_f068)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            }
        }
        ui_state_epilogue(c, frame);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_v33(cFEStateProfileLoad* this)` (0x18faf0): once
/// the profile load has finished (`+0x1ac` still 0), leave the screen by the
/// exit mode at `+0x27c`: 0 asks the UI engine's owner (`+0x10`) for the next
/// state and switches to it (closing popups); 1 sets up the race (event type
/// 4, event 0xc, the rider's record field) and moves the app to a
/// `cPreGameLoadScreen`; 2 switches to a new `cFEStateTitle`. Then `+0x1ac`
/// is set to 1.
pub fn c_fe_state_profile_load_v33(c: &mut Cpu) {
    const DONE: u32 = 0x1ac;
    const EXIT_MODE: u32 = 0x27c;
    const ENGINE: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        let this = abi::arg(c, 0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(DONE).lw(c);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            // `plus_one`: the paths through the shared `v0 = 1` block.
            let mut plus_one = false;
            c.cycles += 3;
            c.g[S3][0] = Addr(this).off(EXIT_MODE).lw(c);
            let mode = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            if mode == 0 {
                // Ask the owner for the next state.
                c.cycles += 10;
                c.g[V0][0] = Addr(this).off(ENGINE).lw(c);
                c.g[A2][0] = Addr(this).off(0x1bc).lw(c);
                c.g[A3][0] = Addr(r32(c, V0)).lw(c);
                c.g[V1][0] = Addr(r32(c, A3)).off(4).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).off(0x28).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x2c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0018_fb44)?;
                let next = c.g[V0][0];
                c.g[A1][0] = next;
                if next == 0 {
                    plus_one = true;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(this).off(ENGINE).lw(c);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f4c0, 0x0018_fb58)?;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 2;
                if mode == 1 {
                    // Into the race: set up the event and load the game.
                    c.cycles += 41;
                    call_ptr(c, 0x0014_dc80, 0x0018_fb70)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0018_fb7c)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dc80, 0x0018_fb84)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0014_dd58, 0x0018_fb90)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 4;
                    call_ptr(c, 0x0014_4df0, 0x0018_fba0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xc;
                    call_ptr(c, 0x0014_51e8, 0x0018_fbac)?;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0014_6d98, 0x0018_fbb8)?;
                    c.g[V1][0] = 0x0053_0000;
                    Addr(r32(c, GP).wrapping_sub(0x1f38)).sw(c, c.g[S3][0]);
                    Addr(0x0053_5c08).sw(c, c.g[V0][0]);
                    // Commit the new-race interface (its vtable slot at +8).
                    let race = r32(c, S0);
                    c.g[V0][0] = Addr(race).off(0xc).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0)).off(8).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(race.wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x0018_fbd8)?;
                    // `App_SetNextState(app, new cPreGameLoadScreen)`.
                    c.g[A0][0] = 8;
                    c.g[A1][0] = sx(0x0045_ce60);
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0018_fbf0)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0023_2720, 0x0018_fbf8)?;
                    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0031_6ef8, 0x0018_fc04)?;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    if mode == 2 {
                        // Back to the title screen.
                        c.cycles += 14;
                        c.g[A0][0] = 0x4c;
                        c.g[A1][0] = sx(0x0045_e290);
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        Addr(r32(c, GP).wrapping_sub(0x1718)).sw(c, 0);
                        call_ptr(c, 0x0031_7d70, 0x0018_fc30)?;
                        c.g[A1][0] = Addr(this).off(ENGINE).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0019_4738, 0x0018_fc3c)?;
                        c.g[A0][0] = Addr(this).off(ENGINE).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f400, 0x0018_fc4c)?;
                        plus_one = true;
                    }
                }
            }
            if plus_one {
                c.cycles += 1;
            }
            c.g[V0][0] = 1;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        Addr(this).off(DONE).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelectProfile::FEScreenSelectProfile` (0x1de1b8, `_ctor_2`).
pub fn fe_screen_select_profile_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_e1d0, 0x3ea8))
}

/// `FEScreenSelectProfile::~FEScreenSelectProfile` (0x1de1f0).
pub fn fe_screen_select_profile_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e20c, 0x2f30)
    })
}

/// `cFEStateProfileLoad::~cFEStateProfileLoad` (0x1de218): the vptr, then the
/// `MemCardStateBase` destructor.
pub fn cfe_state_profile_load_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_MEMCARD_STATE_DTOR, 0x001d_e234, 0x4030)
    })
}

/// `reg + off` as a guest address.
#[inline(always)]
pub(crate) fn off842(reg: u64, off: u32) -> u32 {
    (reg as u32).wrapping_add(off)
}

/// `UIStateMgr_SwitchState` (0x39f400).
pub(crate) const OC842_SWITCH_STATE: u32 = 0x0039_f400;

/// `void FEScreenSelectProfile_OnWidgetAction(self, menu, action)` (0x18eb40):
/// a widget action on a profile-select row. The widget (`menu`) is recoloured
/// through its vtable (`+0x58/+0x5c`): white for action 2, magenta for
/// action 1, yellow for action 5. Actions 5 and 6 then run the owner's
/// action method (`*(self+0x10)`'s engine vtable, `+0x20/+0x24` for 5 and
/// `+0x28/+0x2c` for 6, with `menu->+0x18`); a non-null result is pushed with
/// `UIStateMgr_SwitchState(owner + 0x18, result)`. Other actions do nothing.
pub fn fe_select_profile_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let no_menu = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        // `'out`: the exits that already restored s0 (skip the `+1` block).
        'out: {
            // `'skip`: the exits that run the `+1` block, which restores s0.
            'skip: {
                if no_menu {
                    break 'skip;
                }
                c.cycles += 3;
                c.g[V0][0] = 2;
                let is2 = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = (c.g[A2][0] < 3) as u64;
                if is2 {
                    // Action 2: white (1, 1, 1, 1).
                    c.cycles += 12;
                    c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x8));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x58));
                    c.swc1(0, off842(c.g[SP][0], 0xc));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.swc1(0, off842(c.g[SP][0], 4));
                    c.swc1(0, off842(c.g[SP][0], 8));
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x5c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0018_ec8c)?;
                    break 'skip;
                }
                c.cycles += 2;
                let ge3 = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                if !ge3 {
                    c.cycles += 2;
                    if c.g[A2][0] == c.g[V0][0] {
                        // Action 1: magenta (1, 0, 1, 0).
                        c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x8));
                        c.cycles += 13;
                        c.g[A1][0] = c.g[SP][0];
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x58));
                        c.swc1(0, off842(c.g[SP][0], 8));
                        c.swc1(0, c.g[SP][0] as u32);
                        c.sw(off842(c.g[SP][0], 4), 0);
                        c.sw(off842(c.g[SP][0], 0xc), 0);
                        c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x5c));
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0018_ec54)?;
                        frame.restore(c, S0, 0x30);
                        break 'out;
                    }
                    c.cycles += 2;
                    frame.restore(c, S0, 0x30);
                    break 'out;
                }
                c.cycles += 3;
                c.g[V0][0] = 5;
                if c.g[A2][0] == c.g[V0][0] {
                    // Action 5: yellow (1, 1, 0, 0), then the owner's method.
                    c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x8));
                    c.cycles += 19;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x58));
                    c.swc1(0, off842(c.g[SP][0], 4));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.sw(off842(c.g[SP][0], 8), 0);
                    c.sw(off842(c.g[SP][0], 0xc), 0);
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x5c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0018_ebcc)?;
                    c.g[A0][0] = c.lw(off842(c.g[S1][0], 0x10));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw(off842(c.g[S0][0], 0x18));
                    c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = c.lw(off842(c.g[A3][0], 4));
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x20));
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x24));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 6;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(off842(c.g[S1][0], 0x10));
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'out;
                    }
                    // Action 6: just the owner's method.
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw(off842(c.g[S0][0], 0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw(off842(c.g[A3][0], 4));
                    c.g[A0][0] = c.lh(off842(c.g[V1][0], 0x28));
                    c.g[V0][0] = c.lw(off842(c.g[V1][0], 0x2c));
                }
                // Shared tail: run the method, push a non-null result.
                c.cycles += 4;
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0018_ec0c)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    break 'skip;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(off842(c.g[S1][0], 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, OC842_SWITCH_STATE, 0x0018_ec20)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad::cFEStateProfileLoad(this, a1, mode, flags)`
/// (0x18eca0): the memory-card state constructor (a2 = 0), then the vtable and
/// `+0xc = 0x23`; `mode` goes to the global at `gp-0x1b08`; the fields at
/// `+0x1cc/+0x230/+0x234/+0x27c/+0x22c` are cleared and `flags` is stored at
/// `+0x15`. When bit 0 of `flags` is clear, `+0x22c` counts the leading clear
/// bits (up to 3). Then the pad slots (2 or 1) are searched, with
/// `BELibrary_GetInterface(...)` and `GetPadIndexBySlot`, for the one equal to
/// `+0x22c`; the match goes to `+0x1cc`, and its low byte is copied to `+0x44`.
/// Returns `this`.
pub fn cfe_state_profile_load_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A3][0] & 0xff;
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x001d_53b0, 0x0018_eccc)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = 0x23;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x4030));
        c.sw(off842(c.g[S1][0], 0xc), c.g[V0][0]);
        c.sw(off842(c.g[S1][0], 0x8), c.g[V1][0]);
        c.g[A0][0] = 0;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1b08), c.g[S0][0]);
        c.g[V0][0] = c.g[S2][0] & 1;
        c.sw(off842(c.g[S1][0], 0x1cc), 0);
        c.sw(off842(c.g[S1][0], 0x230), 0);
        c.sw(off842(c.g[S1][0], 0x234), 0);
        c.sw(off842(c.g[S1][0], 0x27c), 0);
        c.sb(off842(c.g[S1][0], 0x15), c.g[S2][0]);
        let bit0 = c.g[V0][0] != 0;
        c.sw(off842(c.g[S1][0], 0x22c), 0);
        if !bit0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(off842(c.g[S1][0], 0x22c));
            'count: loop {
                c.cycles += 6;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32) >> 1);
                c.g[V1][0] = ((c.g[A0][0] as i64) < 3) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let end = c.g[V1][0] == 0;
                c.sw(off842(c.g[S1][0], 0x22c), c.g[V0][0]);
                if end {
                    break 'count;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[S2][0] & 1;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(off842(c.g[S1][0], 0x22c));
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_ed10);
                    }
                    continue 'count;
                }
                break 'count;
            }
        }
        c.cycles += 17;
        c.g[S2][0] = 1;
        call_ptr(c, 0x0014_dc80, 0x0018_ed3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_ed48)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_ed54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0018_ed60)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[A0][0] = 2;
        c.g[V1][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
        c.g[V0][0] = c.lb(off842(c.g[V1][0], 0x49));
        c.g[V0][0] ^= 2;
        if c.g[V0][0] == 0 {
            c.g[S2][0] = c.g[A0][0];
        }
        'pads: loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let past = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S3][0];
            if !past {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7318, 0x0018_ed8c)?;
                c.g[V1][0] = c.lw(off842(c.g[S1][0], 0x22c));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_ed78);
                    }
                    continue 'pads;
                }
                c.cycles += 1;
                c.sw(off842(c.g[S1][0], 0x1cc), c.g[S0][0]);
            }
            break 'pads;
        }
        c.cycles += 10;
        c.g[V1][0] = c.lb(off842(c.g[S1][0], 0x1cc));
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        c.sb(off842(c.g[S1][0], 0x44), c.g[V1][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad_Init(this)` (0x18edc8): create the profile screen
/// (`UIEngine_CreateScreen(owner, this, StrHash(name), 0)`, `owner = *(this+0x10)`),
/// keep it at `+0x40` and start its thread, set up the titles, then show the
/// title for the mode in `gp-0x1b08`: 1 new profile, 2 load, 3 save.
pub fn cfe_state_profile_load_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x1db0));
        c.g[S0][0] = c.lw(off842(c.g[S1][0], 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_edec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_ee00)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(off842(c.g[S1][0], 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_ee18)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_02c0, 0x0018_ee20)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
        c.g[V0][0] = 2;
        let mode2 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        // 0: nothing shown (the `+1` block is skipped), 1: new, 2: load, 3: save.
        let mut show = 0;
        if mode2 {
            show = 2;
        } else {
            c.cycles += 2;
            let ge3 = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if ge3 {
                c.cycles += 3;
                c.g[V0][0] = 3;
                let mode3 = c.g[V1][0] == c.g[V0][0];
                frame.restore(c, S0, 0x20);
                if mode3 {
                    show = 3;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x10);
                }
            } else {
                c.cycles += 2;
                let mode1 = c.g[V1][0] == c.g[V0][0];
                frame.restore(c, S0, 0x20);
                if mode1 {
                    show = 1;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x10);
                }
            }
        }
        if show != 0 {
            match show {
                1 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0019_0540, 0x0018_ee64)?;
                }
                2 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0019_0670, 0x0018_ee74)?;
                }
                _ => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0019_0768, 0x0018_ee84)?;
                }
            }
            frame.restore(c, S0, 0x20);
            c.cycles += 1;
            frame.restore(c, S1, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad_Render(this)` (0x18ee98): `UIState_Render(this)`.
pub fn cfe_state_profile_load_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e6b8, 0x0018_eea8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad::displayOn`: redraws the profile-load screen of the
/// memory-card front end. With no card it shows the "no cards" message and
/// refreshes the slot list instead. Otherwise it writes the device name, then
/// for each of the six profile slots sets its title, the static/greyed state
/// and the date/time texts, and finally shows the left/right device arrows
/// when another usable memory card exists in that direction.
///
/// Frame: `s1` = self, `s3` = the memory-card manager, `s4` = slot index
/// (0..6), `s5` = slot number (1..7), `s2` = the widget-name buffer,
/// `s6` = self.mTitle.
pub fn fe_profile_load_refresh_profile_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_18f9ac: {
            'b_18f370: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x730));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x710));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x700));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x6e0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x6d0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x6c0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x6b0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x690), c.g[RA][0]);
                call_ptr(c, 0x00227f80, 0x0018f2f4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S3][0] = c.g[V0][0];
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018f30c)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_18f370;
                } else {
                    'b_18f34c: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0018f32c)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                            break 'b_18f34c;
                        } else {
                            c.cycles += 6;
                            call_ptr(c, 0x001d8240, 0x0018f33c)?;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0018f9d8, 0x0018f344)?;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
                            break 'b_18f9ac;
                        }
                    }
                    'b_18f36c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        if t {
                            break 'b_18f36c;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f36c)?;
                            break 'b_18f36c;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_18f370;
                }
            }
            'b_18f9a8: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018f380)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_18f9a8;
                } else {
                    c.cycles += 40;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                    c.g[A3][0] = 0x320;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00241710, 0x0018f398)?;
                    c.g[S5][0] = 0x1;
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x680), 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e18));
                    c.g[FP][0] = 0x4a0000;
                    call_ptr(c, 0x00317670, 0x0018f3b0)?;
                    c.g[S7][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x238));
                    c.g[S4][0] = 0u64;
                    call_ptr(c, 0x0039d860, 0x0018f3c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0d00, 0x0018f3d4)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x21c), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018f3f0)?;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e28));
                    call_ptr(c, 0x00317670, 0x0018f3fc)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0018f408)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018f424)?;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    'l_18f428: loop {
                        'b_18f894: {
                            'b_18f7b8: {
                                'b_18f7a0: {
                                    'b_18f608: {
                                        c.cycles += 30;
                                        c.g[V0][0] = 0x460000;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1da0));
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00417828, 0x0018f43c)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00317670, 0x0018f444)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f450)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[V1][0] = 0x4c0000;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66c8));
                                        c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                                        c.ldr(4, c.g[V0][0] as u32);
                                        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                                        c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x8f));
                                        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x88));
                                        c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x97));
                                        c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x90));
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f498)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        if t {
                                            break 'b_18f608;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018f4b8)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                break 'b_18f608;
                                            } else {
                                                'b_18f604: {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x434)
                                                        );
                                                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1b8)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1bc)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x0018f4d8)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] = c.g[S4][0];
                                                    if t {
                                                        break 'b_18f604;
                                                    } else {
                                                        'b_18f594: {
                                                            'b_18f564: {
                                                                'b_18f560: {
                                                                    c.cycles += 14;
                                                                    c.g[A3][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x434));
                                                                    c.g[A2][0] = c.g[S6][0];
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[A3][0] as u32);
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x70));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x74));
                                                                    let t = c.g[V1][0] as u32;
                                                                    c.g[A0][0] = sx((c.g[A3][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A0][0] as u32,
                                                                        ));
                                                                    call_ptr(c, t, 0x0018f4fc)?;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[A1][0] = c.g[S6][0];
                                                                    call_ptr(
                                                                        c, 0x003a0e90, 0x0018f508,
                                                                    )?;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x1b08));
                                                                    c.g[V0][0] = 0x2;
                                                                    let t =
                                                                        c.g[V1][0] == c.g[V0][0];
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x680));
                                                                    if t {
                                                                        break 'b_18f560;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V1][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            break 'b_18f564;
                                                                        } else {
                                                                            c.cycles += 16;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[A0][0] = 0x1;
                                                                            c.sw(
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x680,
                                                                                    ),
                                                                                c.g[A0][0],
                                                                            );
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[A0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x40,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x44,
                                                                                ));
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, t, 0x0018f540,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[A0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xc8,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xcc,
                                                                                ));
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, t, 0x0018f558,
                                                                            )?;
                                                                            c.g[A2][0] = c.g[S5][0];
                                                                            break 'b_18f594;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_18f564;
                                                            }
                                                            c.cycles += 12;
                                                            c.g[A1][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xcc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0018f578)?;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0018f590)?;
                                                            c.g[A2][0] = c.g[S5][0];
                                                            break 'b_18f594;
                                                        }
                                                        c.cycles += 28;
                                                        c.g[A1][0] = sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x15f0));
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00417828, 0x0018f5a0)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00317670, 0x0018f5a8)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x0018f5b4)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0018f5d0)?;
                                                        c.g[A1][0] = sx((c.g[S7][0] as u32)
                                                            .wrapping_add(0x15f8));
                                                        c.g[A2][0] = c.g[S5][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00417828, 0x0018f5e0)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00317670, 0x0018f5e8)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x0018f5f4)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        break 'b_18f7a0;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                break 'b_18f608;
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018f620)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A2][0] = c.g[S6][0];
                                    if t {
                                        break 'b_18f7b8;
                                    } else {
                                        'b_18f6dc: {
                                            'b_18f6a8: {
                                                c.cycles += 19;
                                                c.g[A3][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                c.g[A1][0] = c.g[S4][0];
                                                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0018f644)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S6][0];
                                                call_ptr(c, 0x003a0e90, 0x0018f650)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                c.g[A1][0] = c.g[S4][0];
                                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0018f66c)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_18f6a8;
                                                } else {
                                                    c.cycles += 13;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x0018f688)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x0018f6a0)?;
                                                    c.g[A2][0] = c.g[S5][0];
                                                    break 'b_18f6dc;
                                                }
                                            }
                                            c.cycles += 13;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018f6c0)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018f6d8)?;
                                            c.g[A2][0] = c.g[S5][0];
                                            break 'b_18f6dc;
                                        }
                                        c.cycles += 49;
                                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x15f0));
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00417828, 0x0018f6e8)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00317670, 0x0018f6f0)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f6fc)?;
                                        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[SP][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x78));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f720)?;
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a0e90, 0x0018f72c)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f744)?;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00417828, 0x0018f754)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00317670, 0x0018f75c)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f768)?;
                                        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[A2][0] = c.g[SP][0];
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f78c)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, 0x003a0e90, 0x0018f798)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        break 'b_18f7a0;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018f7b0)?;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                break 'b_18f894;
                            }
                            c.cycles += 55;
                            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                            c.g[A1][0] = c.g[S4][0];
                            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f7d4)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S6][0];
                            call_ptr(c, 0x003a0e90, 0x0018f7e0)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f7f8)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f810)?;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x15f0));
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00417828, 0x0018f820)?;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00317670, 0x0018f828)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0018f834)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f850)?;
                            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00417828, 0x0018f860)?;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00317670, 0x0018f868)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0018f874)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f890)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            break 'b_18f894;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S5][0] as i64) < 7i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018f428);
                            }
                            continue 'l_18f428;
                        } else {
                            'b_18f928: {
                                'b_18f8f8: {
                                    c.cycles += 8;
                                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A3][0] = 0x1;
                                    call_ptr(c, 0x001d97e0, 0x0018f8b4)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[A3][0] = 0x460000;
                                    if t {
                                        break 'b_18f8f8;
                                    } else {
                                        c.cycles += 14;
                                        c.g[A2][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1d90));
                                        call_ptr(c, 0x00317670, 0x0018f8cc)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f8d8)?;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[A2][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f8f0)?;
                                        c.g[A1][0] = c.g[S3][0];
                                        break 'b_18f928;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1d90));
                                call_ptr(c, 0x00317670, 0x0018f900)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0018f90c)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018f924)?;
                                c.g[A1][0] = c.g[S3][0];
                                break 'b_18f928;
                            }
                            'b_18f97c: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x001d97e0, 0x0018f938)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V1][0] = 0x460000;
                                if t {
                                    break 'b_18f97c;
                                } else {
                                    c.cycles += 14;
                                    c.g[V0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1d80));
                                    call_ptr(c, 0x00317670, 0x0018f950)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x0018f95c)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018f974)?;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
                                    break 'b_18f9ac;
                                }
                            }
                            c.cycles += 11;
                            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1d80));
                            call_ptr(c, 0x00317670, 0x0018f984)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0018f990)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f9a8)?;
                            break 'b_18f9a8;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
            break 'b_18f9ac;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x710));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x700));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x690));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x730));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateProfileLoad::OnWidgetAction(cFEStateProfileLoad* this, void* item, int event)`
/// (0x18fd90): the profile load/delete screen's widget handler. A jump table
/// (0x45e2f0, event 5..=14) picks the case:
///
/// - 5 (item selected): with a valid item and no operation running, remembers
///   the slot and either opens the 8-character name keyboard popup (load
///   screens 1 and 3) or confirms the load (screen 2);
/// - 6 (back): cancels the card operation and sets the state to 6;
/// - 7 (delete): starts the delete action on the selected slot;
/// - 13 (keyboard done): stores the typed name (or "Player N") on the player
///   interface and the memory card manager, then begins the card operation;
/// - the others do nothing.
///
/// The generated body is a labelled-block jump table with an `__entries` twin
/// for resumes; the guard keeps the entry form.
pub fn cfe_state_profile_load_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x0018_fd90 {
        return ee::s02::f_0018fd90__entries(c);
    }
    guest(c, |c| 'l_18fd90: loop {
        'b_1902a0: {
            'b_19029c: {
                'b_190298: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    if t {
                        break 'b_190298;
                    } else {
                        'b_1901cc: {
                            'b_190164: {
                                'b_18fecc: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1d10));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x0018fd90 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0018fd90);
                                            }
                                            continue 'l_18fd90;
                                        }
                                        0x0018fde0 => {
                                            'b_18feac: {
                                                'b_18fe3c: {
                                                    c.cycles += 14;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[S0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x74));
                                                    call_ptr(c, 0x00227f80, 0x0018fdf0)?;
                                                    c.g[S5][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0014dc80, 0x0018fdf8)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[A1][0] = 0x1;
                                                    call_ptr(c, 0x0014dd58, 0x0018fe04)?;
                                                    c.g[S4][0] = c.g[V0][0];
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x00416810, 0x0018fe10)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_18fe3c;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1cc));
                                                        c.g[A0][0] = c.g[S4][0];
                                                        c.g[A2][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00147138, 0x0018fe28)?;
                                                        c.g[A0][0] =
                                                            sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x276));
                                                        c.g[A1][0] = c.g[S0][0];
                                                        call_ptr(c, 0x002c2540, 0x0018fe34)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_18feac;
                                                        } else {
                                                            break 'b_18fe3c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 28;
                                                c.g[A0][0] = 0x460000;
                                                c.g[S2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1d50));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x0018fe5c)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x0018fe6c)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x002c2580, 0x0018fe78)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                call_ptr(c, 0x00417828, 0x0018fe8c)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A2][0] = c.g[SP][0];
                                                call_ptr(c, 0x00147138, 0x0018fe9c)?;
                                                c.g[A0][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x276));
                                                c.g[A1][0] = c.g[SP][0];
                                                call_ptr(c, 0x002c2540, 0x0018fea8)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                                break 'b_18feac;
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018febc)?;
                                            c.g[A0][0] = c.g[S3][0];
                                            Callee {
                                                addr: 0x0018_fc98,
                                                f: ee::s02::cFEStateProfileLoad_BeginCardOp,
                                            }
                                            .call(c, 0x0018fec4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            if t {
                                                break 'b_19029c;
                                            } else {
                                                break 'b_18fecc;
                                            }
                                        }
                                        0x0018fecc => {
                                            break 'b_18fecc;
                                        }
                                        0x00190164 => {
                                            break 'b_190164;
                                        }
                                        0x001901cc => {
                                            break 'b_1901cc;
                                        }
                                        0x00190298 => {
                                            break 'b_190298;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x0018fd90, 0x001902bc);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_19029c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        break 'b_1902a0;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        call_ptr(c, 0x00227f80, 0x0018fee8)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                                        c.g[V0][0] = 0x3;
                                        if c.g[A1][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            break 'b_19029c;
                                        } else {
                                            'b_190134: {
                                                c.cycles += 6;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                c.g[V1][0] = 0x2;
                                                c.g[A2][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1c4),
                                                    c.g[V0][0],
                                                );
                                                let t = c.g[A2][0] == c.g[V1][0];
                                                c.sw(
                                                    (c.g[A0][0] as u32).wrapping_add(0xf8),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_190134;
                                                } else {
                                                    'b_18ff38: {
                                                        'b_18ff30: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                ((c.g[A2][0] as i64) < 3i64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_18ff30;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[A2][0] == c.g[V0][0];
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x140),
                                                                );
                                                                if t {
                                                                    break 'b_18ff38;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.lq(
                                                                        17,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    if t {
                                                                        break 'b_1902a0;
                                                                    } else {
                                                                        break 'b_18ff30;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] != c.g[A1][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x140),
                                                        );
                                                        if t {
                                                            break 'b_19029c;
                                                        } else {
                                                            break 'b_18ff38;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[S2][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1c0)
                                                        );
                                                    c.g[V0][0] = 0x1;
                                                    let t = c.g[S2][0] != c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x140),
                                                    );
                                                    if t {
                                                        break 'b_19029c;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x224));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        if t {
                                                            break 'b_1902a0;
                                                        } else {
                                                            'b_1900a8: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x1a4));
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                if t {
                                                                    break 'b_1900a8;
                                                                } else {
                                                                    c.cycles += 82;
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.g[A0][0] = 0x444;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1d38));
                                                                    c.g[A2][0] = 0x100;
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x0018ff78,
                                                                    )?;
                                                                    c.g[T0][0] = c.lbu(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x15),
                                                                    );
                                                                    c.g[A2][0] = c.g[S3][0];
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[A3][0] = 0u64;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    call_ptr(
                                                                        c, 0x001cb030, 0x0018ff90,
                                                                    )?;
                                                                    c.g[V1][0] = 0x37;
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4),
                                                                        c.g[V0][0],
                                                                    );
                                                                    c.sw(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x140),
                                                                        c.g[V1][0],
                                                                    );
                                                                    c.g[A1][0] = 0x8;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    call_ptr(
                                                                        c, 0x001cd088, 0x0018ffa8,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A2][0] = 0x1;
                                                                    c.g[A3][0] = 0x2;
                                                                    c.g[A1][0] = 0x4b;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0018ffbc,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x1;
                                                                    c.sw(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x58),
                                                                        c.g[S2][0],
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    call_ptr(
                                                                        c, 0x001ce408, 0x0018ffd0,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0xe;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0018ffe4,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x27;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0018fff8,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x28;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0019000c,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x34;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190020,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x1a;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190034,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x19;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190048,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x35;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0019005c,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x40;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190070,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x41;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190084,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x42;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190098,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x1;
                                                                    call_ptr(
                                                                        c, 0x001cd0b0, 0x001900a4,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    break 'b_1900a8;
                                                                }
                                                            }
                                                            c.cycles += 35;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x1d28));
                                                            c.g[S0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8c));
                                                            c.g[S1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.lh((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            call_ptr(c, 0x00317670, 0x001900c4)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x24));
                                                            c.g[A0][0] = c.g[S0][0];
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, t, 0x001900d4)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001ccf70, 0x001900e0)?;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x54),
                                                                c.g[S2][0],
                                                            );
                                                            call_ptr(c, 0x0014dc80, 0x001900ec)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0x1;
                                                            call_ptr(c, 0x0014dd58, 0x001900f8)?;
                                                            c.g[A1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1cc));
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x00147170, 0x00190104)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb418, 0x00190110)?;
                                                            c.g[V1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[A1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xd0));
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x00190128)?;
                                                            c.sw(
                                                                (c.g[S3][0] as u32)
                                                                    .wrapping_add(0x1a0),
                                                                c.g[S2][0],
                                                            );
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S3][0] as u32)
                                                                    .wrapping_add(0x19c),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_190298;
                                                            } else {
                                                                break 'b_190134;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x224));
                                            let t = c.g[V0][0] == 0u64;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            if t {
                                                break 'b_19029c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x0023c8f0, 0x00190148)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0019015c)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                if t {
                                                    break 'b_19029c;
                                                } else {
                                                    break 'b_190164;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            call_ptr(c, 0x00227f80, 0x0019016c)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[V0][0] = 0x6;
                            if t {
                                break 'b_190298;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_19029c;
                                } else {
                                    'b_1901c4: {
                                        'b_1901ac: {
                                            'b_1901a8: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1e4),
                                                    c.g[A0][0],
                                                );
                                                if t {
                                                    break 'b_1901a8;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x748));
                                                        break 'b_1901ac;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_1901c4;
                                                        } else {
                                                            break 'b_1901a8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                            break 'b_1901ac;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001901c0)?;
                                        c.g[V0][0] = 0x6;
                                        break 'b_1901c4;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                    if t {
                                        break 'b_190298;
                                    } else {
                                        break 'b_1901cc;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                        c.g[S2][0] = 0x1;
                        let t = c.g[V0][0] == c.g[S2][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_190298;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
                            if t {
                                break 'b_190298;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_19029c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x220));
                                    if c.g[V0][0] == 0u64 {
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        break 'b_1902a0;
                                    } else {
                                        c.cycles += 12;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        call_ptr(c, 0x00227f80, 0x00190208)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00190228)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            break 'b_19029c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c4),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xf8),
                                                c.g[V0][0],
                                            );
                                            c.g[S1][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                            if c.g[S1][0] != c.g[S2][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                break 'b_19029c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1c4));
                                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00190264)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_190298;
                                                } else {
                                                    c.cycles += 11;
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x1a8),
                                                        c.g[S1][0],
                                                    );
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x1c0),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x0023d570, 0x00190280)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x748)
                                                        );
                                                    c.g[A1][0] = 0x18;
                                                    c.g[A0][0] =
                                                        c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x00190298)?;
                                                    break 'b_190298;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_19029c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
            break 'b_1902a0;
        }
        c.cycles += 7;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateProfileLoad_RefreshProfileList` (0x18f2b8), not yet replaced.
pub(crate) const OC845_REFRESH_PROFILE_LIST: Callee = Callee {
    addr: 0x0018_f2b8,
    f: ee::s02::f_0018f2b8,
};

/// A string in the `0x46xxxx` rodata block: `lui 0x46; addiu -off`.
#[inline(always)]
pub(crate) fn oc845_str46(off: u32) -> u32 {
    0x0046_0000u32.wrapping_sub(off)
}

/// A string in the `0x4axxxx` rodata block: `lui 0x4a; addiu off`.
#[inline(always)]
pub(crate) fn oc845_str4a(off: u32) -> u32 {
    0x004a_0000u32.wrapping_add(off)
}

/// Find a widget of the state's screen (`*(this + 0x40)`, `this` in register
/// `this`) by string name: hash the name (`0x317670`), look the hash up in the
/// screen (`0x39d860`, `cUIScreen_getObjectByHashName`). The widget is in `v0`.
pub(crate) fn oc845_find(
    c: &mut Cpu,
    name: u32,
    this: usize,
    ret_hash: u32,
    ret_find: u32,
) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A0][0] = c.lw((c.g[this][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)
}

/// Call the virtual method at `{delta: i16, fn}` `off` of the widget in `v0`,
/// loading through `v1`/`a2` (the compiled form that keeps the widget in `v0`).
pub(crate) fn oc845_vcall_v0(c: &mut Cpu, a1: u64, off: u32, ret_to: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
    c.g[A1][0] = a1;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(off));
    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(off + 4));
    let target = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// Call the virtual method at `off` of the widget held in register `obj`,
/// loading through `v0`/`v1` (the compiled form that keeps the widget in a
/// saved register).
pub(crate) fn oc845_vcall_via(c: &mut Cpu, obj: usize, a1: u64, off: u32, ret_to: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[obj][0] as u32).wrapping_add(8));
    c.g[A1][0] = a1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// `void cFEStateProfileLoad_v39(this, int, int on)` (0x18f168): select the
/// two option widgets (`on ^ 1` to one, set on the other) and remember the
/// selection at `this+0x224`; with the profile mode (`gp-0x1b08`) equal to 2
/// the first widget is skipped and the selection comes from `a1`.
pub fn oc845_profile_load_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        oc845_find(c, oc845_str4a(0x15e0), S1, 0x0018_f198, 0x0018_f1a4)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
        c.g[V0][0] = 0x2;
        let mode_is_2 = c.g[V1][0] == c.g[V0][0];
        c.g[S0][0] ^= 0x1;
        if !mode_is_2 {
            c.cycles += 21;
            oc845_find(c, oc845_str4a(0x15d0), S1, 0x0018_f1c4, 0x0018_f1d0)?;
            let a1 = c.g[S0][0];
            oc845_vcall_v0(c, a1, 0x48, 0x0018_f1e8)?;
        } else {
            c.cycles += 21;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.g[S3][0] ^ 0x1;
            oc845_find(c, oc845_str4a(0x15d8), S1, 0x0018_f21c, 0x0018_f228)?;
            let a1 = c.g[S0][0];
            oc845_vcall_v0(c, a1, 0x48, 0x0018_f240)?;
        }
        c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[S0][0]);
        let a1 = c.g[S0][0];
        let ret_to = if !mode_is_2 { 0x0018_f204 } else { 0x0018_f25c };
        oc845_vcall_via(c, S2, a1, 0x48, ret_to)?;
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_RefreshSlots(this)` (0x18f9d8): ask the
/// memory-card manager and the screen whether the card is usable; when it is
/// not (a check answers 0), relabel the six slot widgets ("slot N": text off,
/// image on); when both answer yes, refresh the profile list instead.
pub fn oc845_profile_load_refresh_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x40);
        call_ptr(c, 0x0022_7f80, 0x0018_fa00)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_fa18)?;
        let first_zero = c.g[V0][0] == 0;
        c.g[S1][0] = 0x1;
        // Both checks passed: refresh the profile list and leave.
        let mut refreshed = false;
        if !first_zero {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_fa38)?;
            let second_zero = c.g[V0][0] == 0;
            c.g[S1][0] = 0x1;
            if !second_zero {
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_fa58)?;
                c.g[A0][0] = c.g[S2][0];
                OC845_REFRESH_PROFILE_LIST.call(c, 0x0018_fa60)?;
                frame.restore(c, S0, 0x80);
                refreshed = true;
            }
        }
        if !refreshed {
            c.cycles += 2;
            c.g[S3][0] = 0x460000;
            c.g[A0][0] = c.g[SP][0];
            loop {
                c.cycles += 25;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1da0));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0041_7828, 0x0018_fa80)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x0018_fa88)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_fa94)?;
                c.g[S0][0] = c.g[V0][0];
                oc845_vcall_via(c, S0, 0, 0xc8, 0x0018_fab0)?;
                oc845_vcall_via(c, S0, 1, 0x40, 0x0018_fac8)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 7i64) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[SP][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_fa70);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
                break;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_Shutdown(this)` (0x18fc70, `onDestroyScreen`):
/// shut the memory-card state base down and clear the "card op pending" word
/// at `this+0x1a4`.
pub fn oc845_profile_load_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_58b8, 0x0018_fc84)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), 0u64);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_BeginCardOp(this)` (0x18fc98): when a card
/// operation is requested (`this+0x224`), start the manager's save
/// (`MCManager_BeginSave(mgr, 1, this+0x1cc)`), mark the state busy
/// (`+0x1a8`, `+0x1c0 = 3`, `+0x1dc`) and, the first time (`+0x210`, `+0x204`
/// both 0), open the status popup: a 0x640-byte buffer filled by
/// `MCManager_GetStatusText` and handed to the screen's vtable `+0x120`.
pub fn oc845_profile_load_begin_card_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x224));
        'epilogue: {
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'epilogue;
            }
            'opened: {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S2][0] = 0x1;
                call_ptr(c, 0x0022_7f80, 0x0018_fcc8)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1cc));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0023_cae8, 0x0018_fcd8)?;
                c.g[V0][0] = 0x3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x210));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S2][0]);
                let popup_open = c.g[V1][0] != 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                if popup_open {
                    break 'opened;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x204));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'epilogue;
                }
                c.cycles += 28;
                c.g[V0][0] = 0x16;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx(oc845_str46(0x1d60));
                c.g[A2][0] = 0x100;
                c.g[A0][0] = 0x640;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x19c), c.g[V0][0]);
                call_ptr(c, 0x0031_7e30, 0x0018_fd1c)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0022_7f80, 0x0018_fd28)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x320;
                call_ptr(c, 0x0024_1e18, 0x0018_fd38)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x120));
                c.g[T0][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[T1][0] = 0u64;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let target = c.g[V1][0] as u32;
                c.g[T2][0] = 0x1;
                call_ptr(c, target, 0x0018_fd64)?;
                let no_buffer = c.g[S1][0] == 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x210), c.g[S2][0]);
                if !no_buffer {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0031_7e98, 0x0018_fd74)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_SetupTitles(this)` (0x1902c0): the screen's
/// initial widget state: hide six title/slot widgets (`vtable+0x48(0)`), then
/// for slots 1..6 set text slot N from the format string and tag the widget
/// with `N-1` (`+0x18`), then show the two buttons (`vtable+0x38`).
pub fn oc845_profile_load_setup_titles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 102;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[A0][0] = 0x460000;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[A0][0] = sx(oc845_str46(0x1ce8));
        frame.save_ra(c, 0x10);
        c.g[S1][0] = 0x1;
        call_ptr(c, 0x0031_7670, 0x0019_02ec)?;
        c.g[S3][0] = 0x4a0000;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[S2][0] = 0x460000;
        call_ptr(c, 0x0039_d860, 0x0019_0300)?;
        c.g[A2][0] = c.g[V0][0];
        oc845_vcall_via(c, A2, 0, 0x48, 0x0019_031c)?;
        // The other five widgets: (name, hash return, find return, call return).
        let hides: [(u32, u32, u32, u32); 5] = [
            (oc845_str46(0x1cd0), 0x0019_0328, 0x0019_0334, 0x0019_0350),
            (oc845_str46(0x1cb8), 0x0019_035c, 0x0019_0368, 0x0019_0384),
            (oc845_str46(0x1ca8), 0x0019_0390, 0x0019_039c, 0x0019_03b8),
            (oc845_str4a(0x15d0), 0x0019_03c4, 0x0019_03d0, 0x0019_03ec),
            (oc845_str4a(0x15d8), 0x0019_03f8, 0x0019_0404, 0x0019_0420),
        ];
        for (name, ret_hash, ret_find, ret_call) in hides {
            oc845_find(c, name, S0, ret_hash, ret_find)?;
            c.g[A2][0] = c.g[V0][0];
            oc845_vcall_via(c, A2, 0, 0x48, ret_call)?;
        }
        oc845_find(c, oc845_str4a(0x1458), S0, 0x0019_042c, 0x0019_0438)?;
        c.g[A2][0] = c.g[V0][0];
        oc845_vcall_via(c, A2, 0, 0x48, 0x0019_0454)?;
        c.g[A0][0] = c.g[SP][0];
        loop {
            c.cycles += 26;
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x12c0));
            call_ptr(c, 0x0041_7828, 0x0019_0464)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0019_046c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_0478)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0019_0484)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1da0));
            call_ptr(c, 0x0041_7828, 0x0019_0494)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0019_049c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0019_04a8)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 7i64) as u64;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[SP][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0019_0458);
                }
                continue;
            }
            break;
        }
        c.cycles += 31;
        oc845_find(c, oc845_str4a(0x1600), S0, 0x0019_04cc, 0x0019_04d8)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0019_04f0)?;
        oc845_find(c, oc845_str4a(0x1608), S0, 0x0019_04fc, 0x0019_0508)?;
        oc845_vcall_v0(c, 1, 0x38, 0x0019_0520)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_ShowNewProfileTitle(this)` (0x190540): show the
/// "new profile" title state: enable the first widget, set two text widgets
/// to their text ids (`0x3a1000`), enable the next widget and the last two
/// buttons, then mark the state (`+0x220`, `+0x224` = 1).
pub fn oc845_profile_load_show_new_profile_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(oc845_str46(0x1ce8));
        call_ptr(c, 0x0031_7670, 0x0019_0560)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_056c)?;
        c.g[S0][0] = c.g[V0][0];
        oc845_vcall_via(c, S0, 1, 0x48, 0x0019_0588)?;
        c.g[A0][0] = sx(oc845_str46(0x1c98));
        call_ptr(c, 0x0031_7670, 0x0019_0594)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x0019_05a0)?;
        c.g[A0][0] = sx(oc845_str46(0x1cb8));
        call_ptr(c, 0x0031_7670, 0x0019_05ac)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_05b8)?;
        c.g[S0][0] = c.g[V0][0];
        oc845_vcall_via(c, S0, 1, 0x48, 0x0019_05d4)?;
        c.g[A0][0] = sx(oc845_str46(0x1c88));
        call_ptr(c, 0x0031_7670, 0x0019_05e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x0019_05ec)?;
        oc845_find(c, oc845_str4a(0x15d0), S1, 0x0019_05f8, 0x0019_0604)?;
        oc845_vcall_v0(c, 1, 0x48, 0x0019_061c)?;
        oc845_find(c, oc845_str4a(0x1458), S1, 0x0019_0628, 0x0019_0634)?;
        oc845_vcall_v0(c, 1, 0x48, 0x0019_064c)?;
        c.g[V0][0] = 0x1;
        frame.restore(c, S0, 0x20);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x220), c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSSXApp_getMCManager(app)`.
pub(crate) const OC846_GET_MC: u32 = 0x0022_7f80;

/// One "get the object named `str_addr`, then call its `setVisible`
/// (vtable slot at +0x48/+0x4c of the object's vtable at +8)" step of the
/// profile-load title screens. `self` is in `s0`; `ret` is the return address
/// of the hash call, the other two calls return to `ret + 0xc` and
/// `ret + 0x24`.
pub(crate) fn oc846_title_set_visible(c: &mut Cpu, str_addr: u32, visible: u64, ret: u32) -> Flow {
    c.g[A0][0] = sx(str_addr);
    call_ptr(c, OC846_HASH, ret)?;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(OC846_BIO_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC846_GET_OBJECT, ret + 0xc)?;
    let obj = c.g[V0][0] as u32;
    c.g[V1][0] = Addr(obj.wrapping_add(0x8)).lw(c);
    c.g[A1][0] = visible;
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x48)).lh(c);
    c.g[A2][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
    let target = c.g[A2][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret + 0x24)
}

/// `cFEStateProfileLoad::ShowLoadTitle` (0x190670): shows the "load profile"
/// title, help text and button text, hides the square-button glyph, then
/// clears the delete-button flag (+0x220), sets the main-button flag (+0x224)
/// and clears the memory-card manager's poll-for-full-card flag (+0x11c).
pub fn cfe_state_profile_load_show_load_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        oc846_title_set_visible(c, OC846_LO - 0x1cd0, 1, 0x0019_068c)?;
        oc846_title_set_visible(c, OC846_LO - 0x1ca8, 1, 0x0019_06bc)?;
        oc846_title_set_visible(c, OC846_HI + 0x15d8, 1, 0x0019_06ec)?;
        oc846_title_set_visible(c, OC846_HI + 0x1460, 0, 0x0019_071c)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V0][0] = 0x1;
        let s0 = c.g[S0][0] as u32;
        Addr(s0.wrapping_add(0x220)).sw(c, 0);
        c.g[RA][0] = sx(0x0019_0754);
        let v0 = c.g[V0][0];
        Addr(s0.wrapping_add(0x224)).sw(c, v0);
        call_ptr(c, OC846_GET_MC, 0x0019_0754)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        let mc = c.g[V0][0] as u32;
        Addr(mc.wrapping_add(0x11c)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateProfileLoad::ShowSaveTitle` (0x190768): shows the "save profile"
/// title, help text and the two button texts, then sets both the delete-button
/// (+0x220) and main-button (+0x224) flags.
pub fn cfe_state_profile_load_show_save_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        oc846_title_set_visible(c, OC846_LO - 0x1ce8, 1, 0x0019_0784)?;
        oc846_title_set_visible(c, OC846_LO - 0x1cb8, 1, 0x0019_07b4)?;
        oc846_title_set_visible(c, OC846_HI + 0x15d0, 1, 0x0019_07e4)?;
        oc846_title_set_visible(c, OC846_HI + 0x1458, 1, 0x0019_0814)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        let s0 = c.g[S0][0] as u32;
        Addr(s0.wrapping_add(0x224)).sw(c, 1);
        Addr(s0.wrapping_add(0x220)).sw(c, 1);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::festateriderbiodetail::tests::oc846_install_fakes;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base;

    pub(crate) use crate::game::render::particle::tests::gp_at;

    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    pub(crate) use crate::game::fe::fememcardcallbacks::tests::hash_screen;

    pub(crate) use crate::game::fe::festateoptions::tests::fe_state_setup;

    pub(crate) use crate::game::fe::festateoptions::tests::state_switch_setup;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    use ssx3_ee as ee;

    #[test]
    fn c_fe_state_profile_load_v26_matches() {
        check(
            ssx3_ee::s02::f_0018f278__generated,
            c_fe_state_profile_load_v26,
            24,
            &|c, seed| {
                fe_state_setup(c, seed, 0x18f2_78);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_init_matches() {
        check(
            ssx3_ee::s02::f_0018ead8__generated,
            fe_screen_select_profile_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x18ead8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_rider_bio_ctor_matches() {
        check(
            ssx3_ee::s02::f_00190858__generated,
            c_fe_state_rider_bio_ctor,
            60,
            &|c, seed| {
                let _r = setup(c, seed, 0x190858);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = (seed % 4) as u64;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_v38_matches() {
        check(
            ee::s02::f_0018f0b8__generated,
            cfe_state_profile_load_v38,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x18f0);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(gp_at(-0x1b08), (seed % 3) as u32);
                c.bus.write32(DATA + 0x40, screen);
                hash_screen(c, screen, node, 0x004a_1458);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0x48, 1);
                fill(c, &mut r, DATA + 0x220, 0x10);
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_on_widget_notify_matches() {
        check(
            ee::s02::f_0018eeb8__generated,
            cfe_state_profile_load_on_widget_notify,
            14,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0018_eeb8, 1);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 7 {
                    0 => 6,
                    1 => 8,
                    2 => 9,
                    3 => 10,
                    4 => 0,
                    5 => 5,
                    _ => (seed % 12) as u64,
                };
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1b08),
                    if seed % 3 == 0 { 2 } else { 0 },
                );
                w(c, OBJ + 0x220, (seed % 2) as u32);
                w(c, OBJ2 + 0x428, 0);
                w(c, OBJ2 + 0x434, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_profile_load_v37_matches() {
        check(
            ee::s02::f_0018ef70__generated,
            c_fe_state_profile_load_v37,
            40,
            &|c, seed| {
                state_switch_setup(c, seed, 0x18_ef70, 0x0, 10);
                // The `gp-0x1b08` compare that skips the second widget.
                w(c, GAME_GP.wrapping_sub(0x1b08), seed % 2);
            },
        );
    }

    #[test]
    fn c_fe_state_profile_load_v33_matches() {
        use crate::game::fe::feasyncfile::tests::alloc_base_4;
        // A method that returns 0 (the owner has no next state).
        const RET_ZERO: u32 = 0x001a_8828;
        check(
            ee::s02::f_0018faf0__generated,
            c_fe_state_profile_load_v33,
            60,
            &|c, seed| {
                // BELibrary at gp-0x1f04, the app at gp-0x848, pools and heap.
                let mut r = alloc_base_4(c, seed, 0x18_faf0, 4);
                // Away from the fake-call results (DATA..DATA+0x40000), the
                // pools/heap (DATA+0x40000..0x80000) and the eafile pools.
                let this = DATA + 0xa_0000;
                let owner = DATA + 0xa_1000;
                let engine = DATA + 0xa_2000;
                let vt = DATA + 0xa_3000;
                let next = DATA + 0xa_4000;
                let case = seed % 6;
                w(
                    c,
                    this + 0x1ac,
                    if case == 5 { r.next_u32() | 1 } else { 0 },
                );
                let mode = match case {
                    0 | 1 => 0,
                    2 => 1,
                    3 => 2,
                    _ => r.next_u32() | 4,
                };
                w(c, this + 0x27c, mode);
                w(c, this + 0x1bc, r.next_u32());
                w(c, this + 0x10, owner);
                // The owner: its engine object with the next-state method at
                // vtable +0x28/+0x2c, and its UIStateMgr (an empty list) at +0x18.
                w(c, owner, engine);
                w(c, engine + 4, vt);
                c.bus.write16(vt + 0x28, 0);
                w(c, vt + 0x2c, if case == 0 { RET_ZERO } else { FAKE });
                // The method is fake call 0: return a real (empty) state node.
                w(c, LOG_RESULTS, next);
                let mgr = owner + 0x18;
                w(c, mgr, mgr);
                w(c, mgr + 4, mgr);
                // (Registers last: the fixture runs guest code.) The
                // generated body has secondary entries and dispatches on `pc`.
                c.g[A0][0] = sx(this);
                c.pc = 0x0018_faf0;
            },
        );
    }

    use crate::difftest::fakes::covered;

    #[test]
    fn fe_screen_select_profile_ctor_2_matches() {
        covered(
            ee::s04::f_001de1b8__generated,
            fe_screen_select_profile_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e1b8, 15);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_dtor_matches() {
        covered(
            ee::s04::f_001de1f0__generated,
            fe_screen_select_profile_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e1f0, 16);
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_dtor_matches() {
        covered(
            ee::s04::f_001de218__generated,
            cfe_state_profile_load_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e218, 17);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::game::fe::festateoptions::fe_install_fakes;
    use crate::game::fe::festateoptions::FE_GATE;
    use crate::game::fe::festateoptions::FE_GATE_MAGIC;
    pub(crate) use crate::game::fe::festateriderbiodetail::tests::oc846_setup;

    pub(crate) use crate::game::fe::festateriderbiodetail::tests::OC846_MAIL_NULL_OBJ;

    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// This module's own gate word (the fakes below act only when it is set).
    pub(crate) const OC842_GATE: u32 = DATA + 0xf_0100;

    pub(crate) const OC842_MAGIC: u32 = 0x0842_beef;

    /// Value the `UIEngine_CreateScreen` fake returns in v0.
    pub(crate) const OC842_RET_CREATE: u32 = DATA + 0xf_0110;

    /// Value the `GetPadIndexBySlot` fake returns in v0.
    pub(crate) const OC842_RET_PAD: u32 = DATA + 0xf_0114;

    /// A real method that returns 0 (the owner has no next state).
    pub(crate) const OC842_RET_ZERO: u32 = 0x001a_8828;

    /// A gated fake of the callee at `$addr`: with the gate word set it logs the
    /// call (registers, cycles, target) and returns to `ra`; with the word at
    /// `$ret` set (nonzero address) v0 is then replaced by that word's value.
    /// With the gate off it runs the body the slot held before.
    macro_rules! oc842_fake {
        ($name:ident, $prev:ident, $addr:expr, $generated:path, $ret:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC842_GATE) != OC842_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                let slot: u32 = $ret;
                if slot != 0 {
                    let v = c.bus.read32(slot);
                    c.g[V0][0] = sx(v);
                }
            }
        };
    }

    oc842_fake!(
        oc842_fake_render,
        OC842_PREV_RENDER,
        0x0039_e6b8,
        ee::s13::f_0039e6b8__generated,
        0
    );

    oc842_fake!(
        oc842_fake_switch_close,
        OC842_PREV_SWITCH_CLOSE,
        0x0039_f4c0,
        ee::s13::f_0039f4c0__generated,
        0
    );

    oc842_fake!(
        oc842_fake_switch,
        OC842_PREV_SWITCH,
        0x0039_f400,
        ee::s13::f_0039f400__generated,
        0
    );

    oc842_fake!(
        oc842_fake_create_screen,
        OC842_PREV_CREATE,
        0x0039_7cc0,
        ee::s13::f_00397cc0__generated,
        OC842_RET_CREATE
    );

    oc842_fake!(
        oc842_fake_start_thread,
        OC842_PREV_START_THREAD,
        0x0039_c870,
        ee::s13::f_0039c870__generated,
        0
    );

    oc842_fake!(
        oc842_fake_get_be,
        OC842_PREV_GET_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated,
        0
    );

    oc842_fake!(
        oc842_fake_get_interface,
        OC842_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated,
        0
    );

    oc842_fake!(
        oc842_fake_pad_index,
        OC842_PREV_PAD_INDEX,
        0x0014_7318,
        ee::s01::f_00147318__generated,
        OC842_RET_PAD
    );

    pub(crate) fn oc842_install() {
        fe_install_fakes();
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E6B8),
            oc842_fake_render,
            &OC842_PREV_RENDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F4C0),
            oc842_fake_switch_close,
            &OC842_PREV_SWITCH_CLOSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc842_fake_switch,
            &OC842_PREV_SWITCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc842_fake_create_screen,
            &OC842_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc842_fake_start_thread,
            &OC842_PREV_START_THREAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc842_fake_get_be,
            &OC842_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc842_fake_get_interface,
            &OC842_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147318),
            oc842_fake_pad_index,
            &OC842_PREV_PAD_INDEX,
        );
    }

    /// Scrambled machine for one seed with both gates set and `pc` at `entry`.
    /// Each of the `calls` fake calls may unwind (one per seed at most).
    pub(crate) fn oc842_machine(c: &mut Cpu, seed: u32, entry: u32, salt: u64, calls: u32) -> Rng {
        oc842_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(FE_GATE, FE_GATE_MAGIC);
        c.bus.write32(OC842_GATE, OC842_MAGIC);
        c.bus.write32(GAME_GP + 0xda0, r.next_u32());
        c.pc = entry;
        r
    }

    pub(crate) fn w842(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A vtable slot pair at `vt + off`: halfword delta, method address.
    pub(crate) fn slot842(c: &mut Cpu, vt: u32, off: u32, delta: u16, method: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, method);
    }

    #[test]
    fn fe_select_profile_on_widget_action_matches() {
        check(
            ssx3_ee::s02::f_0018eb40__generated,
            fe_select_profile_on_widget_action,
            90,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_eb40, 0x18eb40, 3);
                let this = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let menu_vt = DATA + 0x3000;
                let owner = DATA + 0x4000;
                let engine = DATA + 0x5000;
                let engine_vt = DATA + 0x6000;
                let action = match seed % 9 {
                    0 => 2,
                    1 => 1,
                    2 => 5,
                    3 => 6,
                    4 => 0,
                    5 => 3,
                    6 => 4,
                    7 => 7,
                    _ => sx(0xffff_ffff),
                };
                w842(c, this + 0x10, owner);
                w842(c, owner, engine);
                w842(c, engine + 4, engine_vt);
                w842(c, menu + 8, menu_vt);
                w842(c, menu + 0x18, r.next_u32());
                // The widget's colour method (`+0x58/+0x5c`).
                slot842(
                    c,
                    menu_vt,
                    0x58,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    FAKE,
                );
                // The owner's action methods: actions 5 (`+0x20`) and 6 (`+0x28`);
                // one in three returns no state.
                let method = if seed % 3 == 0 {
                    OC842_RET_ZERO
                } else {
                    FAKE + 0x10
                };
                slot842(
                    c,
                    engine_vt,
                    0x20,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    method,
                );
                slot842(
                    c,
                    engine_vt,
                    0x28,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    method,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 11 == 10 { 0 } else { sx(menu) };
                c.g[A2][0] = action;
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018eca0__generated,
            cfe_state_profile_load_ctor,
            120,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_eca0, 0x18eca0, 7);
                // `lb` of the pad-mode byte (0x535c11): 2 selects two pad slots.
                c.bus.write8(0x0053_5c11, if seed % 2 == 0 { 2 } else { 0 });
                // The pad index the fake returns: equal to the `+0x22c` count
                // (0..3) for some seeds.
                w842(c, OC842_RET_PAD, seed / 2 % 5);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0x7_fff0));
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_init_matches() {
        use crate::game::fe::fememcardcallbacks::tests::hash_screen;
        check(
            ssx3_ee::s02::f_0018edc8__generated,
            cfe_state_profile_load_init,
            60,
            &|c, seed| {
                let _ = oc842_machine(c, seed, 0x0018_edc8, 0x18edc8, 14);
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let node = DATA + 0x4000;
                let vt = DATA + 0x5000;
                w842(c, this + 0x10, owner);
                w842(c, OC842_RET_CREATE, if seed % 7 == 0 { 0 } else { screen });
                // One node answers every hash lookup (the names are zeroed
                // game strings); its vtable answers every slot with a fake.
                hash_screen(c, screen, node, 0x004a_1458);
                w842(c, node + 8, vt);
                for off in (0..0x100).step_by(8) {
                    slot842(c, vt, off, 0, FAKE);
                }
                // The title mode.
                w842(
                    c,
                    GAME_GP.wrapping_sub(0x1b08),
                    [1, 2, 3, 0, 4, 2, 1, 3][seed as usize % 8],
                );
                // (Registers last: the fixture ran guest code.)
                c.g[A0][0] = sx(this);
                c.pc = 0x0018_edc8;
            },
        );
    }

    #[test]
    fn cfe_state_profile_load_render_matches() {
        check(
            ssx3_ee::s02::f_0018ee98__generated,
            cfe_state_profile_load_render,
            40,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_ee98, 0x18ee98, 1);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0x7_fff0));
            },
        );
    }

    // Fixture objects, all in DATA+0x240000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC843_CTRL: u32 = DATA + 0x24_0000;

    pub(crate) const OC843_MAGIC: u32 = 0x4f43_3834;

    pub(crate) const OC843_SELF: u32 = DATA + 0x24_1000;

    pub(crate) const OC843_SELF_VT: u32 = DATA + 0x24_2000;

    pub(crate) const OC843_UI: u32 = DATA + 0x24_3000;

    pub(crate) const OC843_MP: u32 = DATA + 0x24_4000;

    pub(crate) const OC843_MC: u32 = DATA + 0x24_5000;

    pub(crate) const OC843_MC_VT: u32 = DATA + 0x24_6000;

    pub(crate) const OC843_WIDGET: u32 = DATA + 0x24_7000;

    pub(crate) const OC843_WIDGET_VT: u32 = DATA + 0x24_8000;

    pub(crate) const OC843_APP: u32 = DATA + 0x24_9000;

    /// Per-callee result lists: 0x80 bytes per `target & 0xfff`.
    pub(crate) const OC843_MAIL: u32 = DATA + 0x28_0000;

    /// Self's vtable: `displayFileOK`, `triggerDisplayState`, `setMenuHighlight`.
    pub(crate) const OC843_SELF_SLOTS: [u32; 3] = [0x150, 0x118, 0x158];

    /// The memory card's vtable: `IsAvailable`, `IsFormatted`, `IsActionable`,
    /// `IsFileSlotDisabled`, `HasFileName`, `GetTitleString`, `GetFileDate`,
    /// `GetFileTime`.
    pub(crate) const OC843_MC_SLOTS: [u32; 8] =
        [0x1a8, 0x1b8, 0x1b0, 0x210, 0x218, 0x70, 0x78, 0x80];

    /// A text widget's vtable: `setStatic`, `setVisible`, `setUseStaticColour`.
    pub(crate) const OC843_WIDGET_SLOTS: [u32; 3] = [0x40, 0x48, 0xc8];

    /// The mailbox of the callee at `target`: `[count, next, v0 values...]`.
    pub(crate) fn oc843_mail(target: u32) -> u32 {
        OC843_MAIL + (target & 0xfff) * 0x80
    }

    /// Make the callee at `target` return `vals` in turn (cycling) in `v0`.
    pub(crate) fn oc843_force(c: &mut Cpu, target: u32, vals: &[u64]) {
        assert!(vals.len() <= 14);
        let m = oc843_mail(target);
        c.bus.write64(m, vals.len() as u64);
        c.bus.write64(m + 8, 0);
        for (i, v) in vals.iter().enumerate() {
            c.bus.write64(m + 16 + 8 * i as u32, *v);
        }
    }

    /// A fake callee: logs the call like `fakes::fake_call`, then returns the
    /// next forced result of its mailbox, unless the call unwound.
    pub(crate) fn oc843_fake_call(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        let m = oc843_mail(target);
        let n = c.bus.read64(m);
        if c.pc == ra && n != 0 {
            let i = c.bus.read64(m + 8);
            c.g[V0][0] = c.bus.read64(m + 16 + 8 * (i % n) as u32);
            c.bus.write64(m + 8, i + 1);
        }
    }

    /// The machine's miss hook (vtable slots pointing at `FAKE`).
    pub(crate) fn oc843_miss(c: &mut Cpu, target: u32) {
        oc843_fake_call(c, target);
    }

    macro_rules! oc843_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC843_CTRL) != OC843_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc843_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc843_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc843_fakes!(
        (
            oc843_fake_227f80,
            OC843_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc843_fake_1d8240,
            OC843_PREV_1D8240,
            ee::s04::OVERRIDE_001D8240,
            ee::s04::f_001d8240__generated,
            0x001d_8240
        ),
        (
            oc843_fake_241710,
            OC843_PREV_241710,
            ee::s06::OVERRIDE_00241710,
            ee::s06::f_00241710__generated,
            0x0024_1710
        ),
        (
            oc843_fake_317670,
            OC843_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc843_fake_39d860,
            OC843_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc843_fake_3a0d00,
            OC843_PREV_3A0D00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a_0d00
        ),
        (
            oc843_fake_3a0e90,
            OC843_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc843_fake_417828,
            OC843_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc843_fake_1d97e0,
            OC843_PREV_1D97E0,
            ee::s04::OVERRIDE_001D97E0,
            ee::s04::f_001d97e0__generated,
            0x001d_97e0
        ),
    );

    /// Vtable slot `off` of `vt`: a random `this` delta, the method a fake
    /// whose mailbox key is its own address.
    pub(crate) fn oc843_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32) {
        let delta = (r.next_u32() % 9) * 4;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + off + 4);
    }

    /// The fake address of the method in vtable slot `off`.
    pub(crate) fn oc843_method(off: u32) -> u32 {
        fakes::FAKE + off + 4
    }

    /// `n` random 0/1 results, 1 with the chance `(m - 1) / m`.
    pub(crate) fn oc843_bits(r: &mut Rng, n: usize, m: u32) -> Vec<u64> {
        (0..n).map(|_| (r.next_u32() % m != 0) as u64).collect()
    }

    /// The screen, the card manager and its card, all methods fakes with
    /// forced results; every seed a different mix of card states.
    pub(crate) fn oc843_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x18_f2b8, 48);
        c.hooks.miss = oc843_miss;
        oc843_install_fakes();
        c.bus.write32(OC843_CTRL, OC843_MAGIC);
        c.pc = 0x0018_f2b8;
        c.g[A0][0] = sx(OC843_SELF);

        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC843_APP);
        let action = match r.next_u32() % 3 {
            0 => 2,
            n => n - 1,
        };
        c.bus.write32(GAME_GP.wrapping_sub(0x1b08), action);
        for i in 0..4 {
            c.bus.write32(0x004c_66c8 + 4 * i, r.next_u32());
        }

        c.bus.write32(OC843_SELF + 8, OC843_SELF_VT);
        c.bus.write32(OC843_SELF + 0x40, OC843_UI);
        let status = if r.next_u32() % 2 == 0 {
            3
        } else {
            r.next_u32() % 5
        };
        c.bus.write32(OC843_SELF + 0x214, status);
        for off in OC843_SELF_SLOTS {
            oc843_slot(c, &mut r, OC843_SELF_VT, off);
        }
        c.bus.write32(OC843_MP + 0x428, r.next_u32() % 4);
        c.bus.write32(OC843_MP + 0x434, OC843_MC);
        c.bus.write32(OC843_MC, OC843_MC_VT);
        for off in OC843_MC_SLOTS {
            oc843_slot(c, &mut r, OC843_MC_VT, off);
        }
        c.bus.write32(OC843_WIDGET + 8, OC843_WIDGET_VT);
        for off in OC843_WIDGET_SLOTS {
            oc843_slot(c, &mut r, OC843_WIDGET_VT, off);
        }

        oc843_force(c, 0x0022_7f80, &[OC843_MP as u64]);
        oc843_force(c, 0x0039_d860, &[OC843_WIDGET as u64]);
        // displayFileOK: a few answers in turn (RefreshSlots reads it too).
        let n = 1 + (r.next_u32() % 3) as usize;
        let ok = oc843_bits(&mut r, n, 3);
        oc843_force(c, oc843_method(0x150), &ok);
        // IsAvailable: constant, so that RefreshSlots (called when there is
        // no card) cannot recurse back into the function.
        let avail = (r.next_u32() % 4 != 0) as u64;
        oc843_force(c, oc843_method(0x1a8), &[avail]);
        // checkNextAvailableDevice: -1 (none) or a device, left then right.
        let next: Vec<u64> = (0..2)
            .map(|_| if r.next_u32() % 2 == 0 { u64::MAX } else { 3 })
            .collect();
        oc843_force(c, 0x001d_97e0, &next);
        let has = oc843_bits(&mut r, 6, 2);
        oc843_force(c, oc843_method(0x218), &has);
        let disabled = oc843_bits(&mut r, 6, 2);
        oc843_force(c, oc843_method(0x210), &disabled);
        let formatted = oc843_bits(&mut r, 6, 4);
        oc843_force(c, oc843_method(0x1b8), &formatted);
        let actionable = oc843_bits(&mut r, 6, 3);
        oc843_force(c, oc843_method(0x1b0), &actionable);
    }

    /// `cFEStateProfileLoad::displayOn` (the profile list of the load screen):
    /// no card (message and slot refresh), screen state change, the six slots
    /// in all their card states, and the device arrows, with calls unwinding
    /// at any point.
    #[test]
    fn fe_profile_load_refresh_profile_list_matches() {
        check(
            ee::s02::f_0018f2b8__generated,
            fe_profile_load_refresh_profile_list,
            300,
            &oc843_setup,
        );
    }

    /// The fixture reaches the function's branches: the no-card message, the
    /// screen state change, titles, static colours, dates and times, both
    /// arrows, and unwinds.
    #[test]
    fn fe_profile_load_refresh_profile_list_fixture_paths() {
        let generated: GuestFn = ee::s02::f_0018f2b8__generated;
        let mut targets = BTreeSet::new();
        let mut counts = BTreeSet::new();
        let mut unwinds = 0;
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc843_setup(&mut c, seed);
            generated(&mut c);
            unwinds += (c.pc & 0xffff_0000 == 0xdead_0000) as u32;
            let n = c.bus.read32(fakes::LOG_COUNT);
            counts.insert(n);
            for i in 0..n {
                targets.insert(c.bus.read32(fakes::LOG_START + 0x220 * i));
            }
        }
        for t in [
            0x001d_8240,
            0x0024_1710,
            0x001d_97e0,
            0x003a_0e90,
            0x0041_7828,
            oc843_method(0x118),
            oc843_method(0x158),
            oc843_method(0x40),
            oc843_method(0x48),
            oc843_method(0xc8),
            oc843_method(0x70),
            oc843_method(0x78),
            oc843_method(0x80),
        ] {
            assert!(targets.contains(&t), "never called: {t:#x}");
        }
        assert!(counts.len() >= 15, "call counts {counts:?}");
        assert!(unwinds > 20, "unwinds {unwinds}");
    }

    // Layout of the fixture (all below DATA + 0x100000, the fakes' log).
    pub(crate) const OC844_THIS: u32 = DATA + 0x1_0000;

    pub(crate) const OC844_THIS_VT: u32 = DATA + 0x2_0000;

    pub(crate) const OC844_ITEM: u32 = DATA + 0x3_0000;

    pub(crate) const OC844_MP: u32 = DATA + 0x4_0000;

    pub(crate) const OC844_MP_VT: u32 = DATA + 0x5_0000;

    pub(crate) const OC844_MEM: u32 = DATA + 0x6_0000;

    pub(crate) const OC844_MEM_VT: u32 = DATA + 0x7_0000;

    pub(crate) const OC844_WORLD: u32 = DATA + 0x8_0000;

    pub(crate) const OC844_LIST: u32 = DATA + 0x8_1000;

    pub(crate) const OC844_ENTRY: u32 = DATA + 0x8_2000;

    pub(crate) const OC844_PLAYER: u32 = DATA + 0x9_0000;

    pub(crate) const OC844_PLAYER_VT: u32 = DATA + 0x9_1000;

    pub(crate) const OC844_KB: u32 = DATA + 0xa_0000;

    pub(crate) const OC844_KB_LIST: u32 = DATA + 0xa_1000;

    pub(crate) const OC844_SCREEN: u32 = DATA + 0xb_0000;

    pub(crate) const OC844_STATE: u32 = DATA + 0xb_1000;

    pub(crate) const OC844_ENGINE: u32 = DATA + 0xb_2000;

    /// Jump table of the event switch (event 5 at index 0).
    pub(crate) const OC844_TABLE: u32 = 0x0045_e2f0;

    /// A real leaf (`jr ra` returning 0) for vtable slots that must return 0.
    pub(crate) const OC844_ZERO_LEAF: u32 = 0x0014_06c0;

    /// `cFEStateProfileLoad_mScreenAction`.
    pub(crate) const OC844_SCREEN_ACTION: u32 = GAME_GP - 0x1b08;

    pub(crate) const OC844_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC844_MAGIC: u32 = 0x0844_c0de;

    pub(crate) fn oc844_mail(idx: u32) -> u32 {
        OC844_CTRL + 4 + 4 * idx
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    pub(crate) fn oc844_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc844_mail(idx));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc844_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC844_CTRL) != OC844_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc844_fake_call(c, $addr, $idx);
                }
            )*
            fn oc844_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc844_fakes!(
        (
            oc844_fake_mc_manager,
            OC844_PREV_MC_MANAGER,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80,
            0
        ),
        (
            oc844_fake_get_be,
            OC844_PREV_GET_BE,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            1
        ),
        (
            oc844_fake_get_interface,
            OC844_PREV_GET_INTERFACE,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            2
        ),
        (
            oc844_fake_strlen,
            OC844_PREV_STRLEN,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810,
            3
        ),
        (
            oc844_fake_set_player_name,
            OC844_PREV_SET_NAME,
            ee::s01::OVERRIDE_00147138,
            ee::s01::f_00147138__generated,
            0x0014_7138,
            4
        ),
        (
            oc844_fake_strcpy_a,
            OC844_PREV_STRCPY_A,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540,
            5
        ),
        (
            oc844_fake_strcpy_b,
            OC844_PREV_STRCPY_B,
            ee::s08::OVERRIDE_002C2580,
            ee::s08::f_002c2580__generated,
            0x002c_2580,
            6
        ),
        (
            oc844_fake_str_hash,
            OC844_PREV_STR_HASH,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            7
        ),
        (
            oc844_fake_sprintf,
            OC844_PREV_SPRINTF,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            8
        ),
        (
            oc844_fake_set_max_chars,
            OC844_PREV_MAX_CHARS,
            ee::s04::OVERRIDE_001CD088,
            ee::s04::f_001cd088__generated,
            0x001c_d088,
            9
        ),
        (
            oc844_fake_set_include_key,
            OC844_PREV_INCLUDE_KEY,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8,
            10
        ),
        (
            oc844_fake_set_disable_shift,
            OC844_PREV_DISABLE_SHIFT,
            ee::s04::OVERRIDE_001CE408,
            ee::s04::f_001ce408__generated,
            0x001c_e408,
            11
        ),
        (
            oc844_fake_set_static,
            OC844_PREV_SET_STATIC,
            ee::s04::OVERRIDE_001CCF70,
            ee::s04::f_001ccf70__generated,
            0x001c_cf70,
            12
        ),
        (
            oc844_fake_get_player_name,
            OC844_PREV_GET_NAME,
            ee::s01::OVERRIDE_00147170,
            ee::s01::f_00147170__generated,
            0x0014_7170,
            13
        ),
        (
            oc844_fake_set_input_text,
            OC844_PREV_INPUT_TEXT,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418,
            14
        ),
        (
            oc844_fake_set_min_chars,
            OC844_PREV_MIN_CHARS,
            ee::s04::OVERRIDE_001CD0B0,
            ee::s04::f_001cd0b0__generated,
            0x001c_d0b0,
            15
        ),
        (
            oc844_fake_read_set_file_type,
            OC844_PREV_READ_TYPE,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0,
            16
        ),
        (
            oc844_fake_set_action,
            OC844_PREV_SET_ACTION,
            ee::s06::OVERRIDE_0023D570,
            ee::s06::f_0023d570__generated,
            0x0023_d570,
            17
        ),
    );

    /// A vtable method slot (`lh off+0 -> this adjust`, `lw off+4 -> function`)
    /// at `vt + off`: the fake, or a real leaf that returns 0.
    pub(crate) fn oc844_slot(c: &mut Cpu, vt: u32, off: u32, adjust: u32, zero: bool) {
        c.bus.write16(vt + off, adjust as u16);
        c.bus
            .write32(vt + off + 4, if zero { OC844_ZERO_LEAF } else { FAKE });
    }

    /// `cFEStateProfileLoad_OnWidgetAction`: every event (5..=14, and ones
    /// outside the table), item present/absent, operation running or not,
    /// screen status, screen actions 0..=4, states 0..=6, the keyboard
    /// popup / confirm-load / back / delete paths, empty and non-empty typed
    /// names, callee results of 0, the unwinds of its calls and a jump-table
    /// target outside the function (the jump-out arm).
    ///
    /// Not reached: the first-use keyboard construction (`this->mpKeyboard ==
    /// 0` calls the real `cKeyboardPopup` constructor, which has no override
    /// slot to fake and runs away on fixture data), and the jump-table arm
    /// that restarts the function (it re-enters the prologue forever).
    #[test]
    fn cfe_state_profile_load_on_widget_action_matches() {
        oc844_install_fakes();
        const EVENTS: [u32; 14] = [5, 5, 5, 6, 7, 7, 7, 8, 9, 13, 13, 14, 15, 3];
        check(
            ee::s02::f_0018fd90__generated,
            cfe_state_profile_load_on_widget_action,
            2400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0844, 36);
                c.pc = 0x0018_fd90;
                let pick = |r: &mut Rng, list: &[u32]| list[(r.next_u32() as usize) % list.len()];
                c.bus.write32(OC844_CTRL, OC844_MAGIC);
                let event = EVENTS[(seed as usize) % EVENTS.len()];
                // Jump table: event 5 -> 0x18fecc, 6 -> 0x190164, 7 -> 0x1901cc,
                // 13 -> 0x18fde0, the rest -> the epilogue.
                let mut table = [0x0019_0298u32; 10];
                table[0] = 0x0018_fecc;
                table[1] = 0x0019_0164;
                table[2] = 0x0019_01cc;
                table[8] = 0x0018_fde0;
                if seed % 41 == 0 {
                    table[(event.wrapping_sub(5) % 10) as usize] = 0x0019_0000;
                }
                put_words(c, OC844_TABLE, &table);
                // Callee results.
                c.bus.write32(oc844_mail(0), OC844_MP + 3);
                c.bus.write32(oc844_mail(2), OC844_PLAYER + 3);
                c.bus
                    .write32(oc844_mail(3), if r.next_u32() % 3 == 0 { 1 } else { 8 });
                // The screen object.
                let this = OC844_THIS;
                c.bus.write32(this + 0x8, OC844_THIS_VT);
                c.bus.write32(this + 0x10, DATA + 0xc_0000);
                c.bus.write8(this + 0x15, r.next_u32() as u8);
                c.bus.write32(this + 0x40, OC844_SCREEN);
                c.bus.write32(OC844_SCREEN + 0xd0, OC844_STATE);
                c.bus.write32(OC844_STATE + 0x10, OC844_ENGINE);
                c.bus.write32(this + 0x1a4, OC844_KB);
                c.bus.write32(OC844_KB + 0x20, OC844_KB_LIST);
                c.bus.write32(OC844_KB + 0x1c, r.next_u32());
                c.bus.write32(OC844_KB_LIST + 0x1c, r.next_u32());
                let processing = pick(&mut r, &[0, 0, 0, 1, 2]);
                c.bus.write32(this + 0x1a8, processing);
                c.bus
                    .write32(this + 0x1c0, pick(&mut r, &[1, 1, 1, 0, 2, 3, 4, 5, 6]));
                c.bus.write32(this + 0x1cc, pick(&mut r, &[0, 1, 2, 3]));
                c.bus.write32(this + 0x214, pick(&mut r, &[3, 3, 3, 3, 2]));
                c.bus.write32(this + 0x220, pick(&mut r, &[1, 1, 0]));
                // 0x224 (main button) must be 0 for event 13: the real
                // `BeginCardOp` it calls has deeper paths otherwise.
                let main_button = if event == 13 {
                    0
                } else {
                    pick(&mut r, &[1, 1, 1, 0])
                };
                c.bus.write32(this + 0x224, main_button);
                c.bus.write32(this + 0x1c4, r.next_u32());
                c.bus.write32(this + 0x1e4, r.next_u32());
                c.bus.write32(this + 0x19c, r.next_u32());
                c.bus.write32(this + 0x1a0, r.next_u32());
                c.bus
                    .write32(OC844_SCREEN_ACTION, pick(&mut r, &[1, 1, 3, 3, 2, 2, 0, 4]));
                // The item (null for some seeds).
                c.bus.write32(OC844_ITEM + 0x18, r.next_u32());
                // Memory card manager and its objects.
                c.bus.write32(OC844_MP + 0x434, OC844_MEM);
                c.bus.write32(OC844_MP + 0x428, r.next_u32());
                c.bus.write32(OC844_MP + 0x748, OC844_MP_VT);
                c.bus.write32(OC844_MEM, OC844_MEM_VT);
                c.bus.write32(OC844_MP + 0xf8, r.next_u32());
                let z = |r: &mut Rng| r.next_u32() % 5 == 0;
                let adj = |r: &mut Rng| (r.next_u32() & 0x3c) + 4;
                let a = adj(&mut r);
                oc844_slot(c, OC844_THIS_VT, 0x160, a, false);
                let a = adj(&mut r);
                oc844_slot(c, OC844_MP_VT, 0x8, a, false);
                let (a, zero) = (adj(&mut r), z(&mut r));
                oc844_slot(c, OC844_MEM_VT, 0x1b0, a, zero);
                let (a, zero) = (adj(&mut r), z(&mut r));
                oc844_slot(c, OC844_MEM_VT, 0x218, a, zero);
                let a = adj(&mut r);
                oc844_slot(c, OC844_PLAYER_VT, 0x8, a, false);
                c.bus.write32(OC844_PLAYER + 0xc, OC844_PLAYER_VT);
                // World -> list -> entry (a vtable call for the "Player N" text).
                c.bus.write32(GAME_GP - 0x848, OC844_WORLD);
                c.bus.write32(OC844_WORLD + 0x8c, OC844_LIST);
                c.bus.write32(OC844_LIST + 4, OC844_ENTRY);
                c.bus.write16(OC844_ENTRY + 0x20, 0x10);
                c.bus.write32(OC844_ENTRY + 0x24, FAKE);
                // Arguments.
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 8 == 7 { 0 } else { sx(OC844_ITEM) };
                c.g[A2][0] = sx(event);
            },
        );
    }

    /// Gate word (the fakes only act when it holds the magic) and the
    /// "return 0" mailboxes (one word per fake address).
    pub(crate) const OC845_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC845_MAGIC: u32 = 0x4f43_8450;

    pub(crate) const OC845_ZERO: u32 = DATA + 0x20_1000;

    /// The state object, a widget, a shared vtable and a screen.
    pub(crate) const OC845_THIS: u32 = DATA + 0x8_0000;

    pub(crate) const OC845_NODE: u32 = DATA + 0x9_0000;

    pub(crate) const OC845_VT: u32 = DATA + 0xa_0000;

    pub(crate) const OC845_SCREEN: u32 = DATA + 0xb_0000;

    pub(crate) const OC845_MGR: u32 = DATA + 0xc_0000;

    pub(crate) const OC845_OBJ: u32 = DATA + 0xd_0000;

    pub(crate) const OC845_VT2: u32 = DATA + 0xe_0000;

    /// A real leaf that returns 0.
    pub(crate) const OC845_RET_ZERO: u32 = 0x001a_8828;

    /// The fake callee: logs the call like `fakes::fake_call`; a mailbox set to
    /// 1 makes it return 0.
    pub(crate) fn oc845_fake_call(c: &mut Cpu, addr: u32) {
        let zero = c.bus.read32(OC845_ZERO + (addr & 0xffc)) == 1;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if zero && c.pc == ra {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc845_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC845_CTRL) != OC845_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc845_fake_call(c, $addr);
                }
            )*
            fn oc845_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc845_fakes!(
        (
            oc845_fake_317670,
            OC845_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc845_fake_39d860,
            OC845_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc845_fake_227f80,
            OC845_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc845_fake_417828,
            OC845_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc845_fake_3a0c50,
            OC845_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc845_fake_3a1000,
            OC845_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc845_fake_23cae8,
            OC845_PREV_23CAE8,
            ee::s06::OVERRIDE_0023CAE8,
            ee::s06::f_0023cae8__generated,
            0x0023_cae8
        ),
        (
            oc845_fake_317e30,
            OC845_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc845_fake_241e18,
            OC845_PREV_241E18,
            ee::s06::OVERRIDE_00241E18,
            ee::s06::f_00241e18__generated,
            0x0024_1e18
        ),
        (
            oc845_fake_317e98,
            OC845_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc845_fake_1d58b8,
            OC845_PREV_1D58B8,
            ee::s04::OVERRIDE_001D58B8,
            ee::s04::f_001d58b8__generated,
            0x001d_58b8
        ),
    );

    /// A vtable whose every `{delta 0, fn}` slot (8-byte stride) calls a fake.
    pub(crate) fn oc845_vtable(c: &mut Cpu, vt: u32) {
        for off in (0..0x200u32).step_by(8) {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * ((off / 8) % 8));
        }
    }

    /// The machine with the gate set and the shared objects: the state at
    /// `OC845_THIS` (screen at `+0x40`, vtable at `+8`), and a widget whose
    /// vtable calls fakes; the first fake results are that widget.
    pub(crate) fn oc845_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc845_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC845_CTRL, OC845_MAGIC);
        oc845_vtable(c, OC845_VT);
        c.bus.write32(OC845_THIS + 8, OC845_VT);
        c.bus.write32(OC845_THIS + 0x40, OC845_SCREEN);
        c.bus.write32(OC845_NODE + 8, OC845_VT);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// The first eight fake calls return the widget.
    pub(crate) fn oc845_results_node(c: &mut Cpu) {
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, OC845_NODE);
        }
    }

    #[test]
    fn oc845_profile_load_v39_matches() {
        check(
            ee::s02::f_0018f168__generated,
            oc845_profile_load_v39,
            120,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_f168, 10);
                if seed % 3 != 0 {
                    oc845_results_node(c);
                }
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1b08),
                    if seed % 2 == 0 { 2 } else { r.next_u32() & 3 },
                );
                c.bus.write32(OC845_THIS + 0x224, r.next_u32());
                c.g[A0][0] = sx(OC845_THIS);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0018_f168;
            },
        );
    }

    #[test]
    fn oc845_profile_load_refresh_slots_matches() {
        check(
            ee::s02::f_0018f9d8__generated,
            oc845_profile_load_refresh_slots,
            160,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_f9d8, 45);
                // Call 0 (the manager) returns the manager object.
                c.bus.write32(LOG_RESULTS, OC845_MGR);
                c.bus.write32(OC845_MGR + 0x434, OC845_OBJ);
                c.bus.write32(OC845_OBJ, OC845_VT2);
                oc845_vtable(c, OC845_VT2);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0xf_0000);
                // 0: the screen check answers 0; 1: the card check answers 0;
                // 2, 3: both answer yes and the profile list is refreshed.
                let case = seed % 4;
                if case == 0 {
                    c.bus.write32(OC845_VT + 0x154, OC845_RET_ZERO);
                }
                if case == 1 {
                    c.bus.write32(OC845_VT2 + 0x1ac, OC845_RET_ZERO);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.g[S1][0] = r.next_u32() as u64;
                c.pc = 0x0018_f9d8;
            },
        );
    }

    #[test]
    fn oc845_profile_load_shutdown_matches() {
        check(
            ee::s02::f_0018fc70__generated,
            oc845_profile_load_shutdown,
            40,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_fc70, 1);
                c.bus.write32(OC845_THIS + 0x1a4, r.next_u32());
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0018_fc70;
            },
        );
    }

    #[test]
    fn oc845_profile_load_begin_card_op_matches() {
        check(
            ee::s02::f_0018fc98__generated,
            oc845_profile_load_begin_card_op,
            160,
            &|c, seed| {
                let mut r = oc845_machine(c, seed, 0x18_fc98, 7);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0xf_0000);
                // 0: no card op requested; 1: the popup is already open
                // (+0x210); 2: first time but +0x204 set; 3: no buffer
                // (allocator returns 0); 4..7: popup built and the buffer freed.
                let case = seed % 8;
                c.bus.write32(
                    OC845_THIS + 0x224,
                    if case == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    OC845_THIS + 0x210,
                    if case == 1 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(
                    OC845_THIS + 0x204,
                    if case == 2 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(OC845_THIS + 0x1cc, r.next_u32());
                if case == 3 {
                    c.bus.write32(OC845_ZERO + (0x0031_7e30 & 0xffc), 1);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0018_fc98;
            },
        );
    }

    #[test]
    fn oc845_profile_load_setup_titles_matches() {
        check(
            ee::s02::f_001902c0__generated,
            oc845_profile_load_setup_titles,
            100,
            &|c, seed| {
                let _r = oc845_machine(c, seed, 0x19_02c0, 70);
                if seed % 3 != 0 {
                    oc845_results_node(c);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0019_02c0;
            },
        );
    }

    #[test]
    fn oc845_profile_load_show_new_profile_title_matches() {
        check(
            ee::s02::f_00190540__generated,
            oc845_profile_load_show_new_profile_title,
            80,
            &|c, seed| {
                let _r = oc845_machine(c, seed, 0x19_0540, 22);
                if seed % 3 != 0 {
                    oc845_results_node(c);
                }
                c.g[A0][0] = sx(OC845_THIS);
                c.pc = 0x0019_0540;
            },
        );
    }

    /// `cFEStateProfileLoad::ShowLoadTitle`: four object lookups and
    /// `setVisible` calls through the object's vtable, the flags at +0x220 /
    /// +0x224 and the memory-card manager's flag.
    #[test]
    fn cfe_state_profile_load_show_load_title_matches() {
        oc846_install_fakes();
        check(
            ee::s02::f_00190670__generated,
            cfe_state_profile_load_show_load_title,
            120,
            &|c, seed| {
                let _ = oc846_setup(c, seed, 0x190670, 13);
                c.pc = 0x0019_0670;
                c.bus.write32(OC846_MAIL_NULL_OBJ, 0);
            },
        );
    }

    /// `cFEStateProfileLoad::ShowSaveTitle`: four lookups, both button flags.
    #[test]
    fn cfe_state_profile_load_show_save_title_matches() {
        oc846_install_fakes();
        check(
            ee::s02::f_00190768__generated,
            cfe_state_profile_load_show_save_title,
            120,
            &|c, seed| {
                let _ = oc846_setup(c, seed, 0x190768, 12);
                c.pc = 0x0019_0768;
                c.bus.write32(OC846_MAIL_NULL_OBJ, 0);
            },
        );
    }
}
