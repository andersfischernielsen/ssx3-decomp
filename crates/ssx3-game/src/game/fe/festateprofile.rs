//! `src/fe/festateprofile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;
use crate::call::sp;
use crate::call::Flow;
use ssx3_ee_rt::vu0::X;
use crate::abi::r32;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::festateoptions::STR_HASH;
pub(crate) use crate::game::fe::festateoptions::UI_SCREEN_FIND_OBJECT;
pub(crate) use crate::game::fe::festateoptions::screen_object;
pub(crate) use crate::game::fe::festateoptions::call_object_a2;
pub(crate) use crate::game::fe::festateoptions::ui_state_frame;
pub(crate) use crate::game::fe::festateoptions::ui_state_epilogue;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_ead8, fe_screen_select_profile_init, "FEScreenSelectProfile_Init"),
    (0x0018_eeb8, cfe_state_profile_load_on_widget_notify, "cFEStateProfileLoad_OnWidgetNotify"),
    (0x0018_ef70, c_fe_state_profile_load_v37, "cFEStateProfileLoad_v37"),
    (0x0018_f0b8, cfe_state_profile_load_v38, "cFEStateProfileLoad_v38"),
    (0x0018_f278, c_fe_state_profile_load_v26, "cFEStateProfileLoad_v26"),
    (0x0019_0858, c_fe_state_rider_bio_ctor, "cFEStateRiderBio_ctor"),
];

// ---------------------------------------------------------------------------
// Callees, under the names the generated bodies give them.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
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
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
        let mut result = 0;
        'outer: {
            'b58: {
                'b54: {
                    'b4c: {
                        'b38: {
                            call_ptr(c, 0x0022_7f80, 0x0018_eed8)?;
                            c.g[V1][0]=c.g[V0][0];
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
            'actions: {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::RA as SENTINEL;
    use ssx3_ee as ee;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP;
    pub(crate) use crate::game::fe::festateoptions::tests::fe_state_setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::run;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::fememcard2::tests::str_hash;
    pub(crate) use crate::game::fe::fememcard2::tests::hash_screen;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::festateoptions::tests::belib;
    pub(crate) use crate::game::fe::festateoptions::tests::RET;
    pub(crate) use crate::game::fe::festateoptions::tests::str_hash_2;
    pub(crate) use crate::game::fe::festateoptions::tests::screen_fixture;
    pub(crate) use crate::game::fe::festateoptions::tests::state_switch_setup;


    #[test]
    fn c_fe_state_profile_load_v26_matches() {
        check(ssx3_ee::s02::f_0018f278__generated, c_fe_state_profile_load_v26, 24, &|c, seed| {
            fe_state_setup(c, seed, 0x18f2_78);
        });
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
                w(c, GAME_GP.wrapping_sub(0x1b08), if seed % 3 == 0 { 2 } else { 0 });
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
}
