//! `src/fe/festateriderbiodetail.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_1228,
        cfe_state_rider_bio_fill_bio_info,
        "cFEStateRiderBio_fillBioInfo",
    ),
    (
        0x0019_12d0,
        c_fe_state_rider_bio_on_widget_action,
        "cFEStateRiderBio_OnWidgetAction",
    ),
    (
        0x0019_1360,
        cfe_state_rule_select_ctor,
        "cFEStateRuleSelect_ctor",
    ),
    (
        0x0019_08c0,
        cfe_state_rider_bio_init,
        "cFEStateRiderBio_Init",
    ),
    (
        0x0019_0a08,
        cfe_state_rider_bio_show_page,
        "cFEStateRiderBio_ShowPage",
    ),
    (
        0x0019_0cd8,
        cfe_state_rider_bio_show_rider_image,
        "cFEStateRiderBio_ShowRiderImage",
    ),
    (
        0x0019_0de0,
        cfe_state_rider_bio_fill_dna,
        "cFEStateRiderBio_FillDNA",
    ),
    (
        0x0019_0f48,
        cfe_state_rider_bio_fill_faves,
        "cFEStateRiderBio_FillFaves",
    ),
    (
        0x0019_10c0,
        cfe_state_rider_bio_fill_qna,
        "cFEStateRiderBio_FillQNA",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
}

/// `cFEStateRuleSelect* cFEStateRuleSelect_ctor(void* self)` (0x191360): the
/// rule-select state: `UIState_ctor`, vtable, the string-type byte from the
/// first UI layer, and the state id 0x14.
pub fn cfe_state_rule_select_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x0039_e2a0, 0x0019_1374).is_err() {
        return;
    }
    c.g[V1][0] = sx(0x0046_b090);
    c.sw(p(c, S0, 0x48), 0);
    c.sb(p(c, S0, 0x44), 0);
    c.sw(p(c, S0, 8), c.g[V1][0]);
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr(p(c, S0, 0x10)).lw(c);
    c.g[A0][0] = Addr(p(c, V0, 0)).lw(c);
    if call_ptr(c, 0x001a_1cd0, 0x0019_1398).is_err() {
        return;
    }
    c.sb(p(c, S0, 0x15), c.g[V0][0]);
    c.g[V0][0] = 0x14;
    frame.restore_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.close(c);
}

/// `void cFEStateRiderBio_OnWidgetAction(self, item, msg)` (0x1912d0): on a
/// widget action (6) hand the event to the screen's virtual handler and swap
/// the screen; on a page change (9) remember the page and rebuild the bio.
pub fn c_fe_state_rider_bio_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let item = abi::arg64(c, 1);
        let _msg = abi::arg64(c, 2);
        c.g[V1][0] = item;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_item = c.g[V1][0] == 0;
        c.g[S0][0] = sx(self_);
        if no_item {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is_six = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 9;
            if is_six {
                c.cycles += 11;
                c.g[V0][0] = c.lw(at(c, S0, 0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw(at(c, V1, 0x18));
                c.g[A3][0] = c.lw(r32(c, V0));
                c.g[V1][0] = c.lw(at(c, A3, 4));
                c.g[A0][0] = c.lh(at(c, V1, 0x28));
                c.g[V0][0] = c.lw(at(c, V1, 0x2c));
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                let target = r32(c, V0);
                call_ptr(c, target, 0x0019_1328)?;
                let nothing = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if nothing {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(at(c, S0, 0x10));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                    callee::UI_STATE_MGR_SWITCH_STATE.call(c, 0x0019_133c)?;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.cycles += 2;
                let is_nine = c.g[A2][0] == c.g[V0][0];
                if is_nine {
                    c.g[V0][0] = c.lbu(at(c, V1, 0x319));
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(c, S0, 0x4c), c.g[V0][0]);
                    call_ptr(c, 0x0019_0a08, 0x0019_1350)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider bio: format the character's display name into the bio's text object.

/// `void cFEStateRiderBio_fillBioInfo(cFEStateRiderBio* self)` (0x191228):
/// find the scrolling-text object on the screen, format the character's
/// display name into a stack buffer and set it as the text (by hash).
pub fn cfe_state_rider_bio_fill_bio_info(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 14;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        c.g[A0][0] = sx(0x0046_0000);
        frame.save_ra(c, 0x40);
        c.g[RA][0] = sx(0x0019_1248);
        c.g[A0][0] = sx(0x0045_e438);
        call_ptr(c, 0x0031_7670, 0x0019_1248)?;
        c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x40));
        c.g[RA][0] = sx(0x0019_1254);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_1254)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.cycles += 23;
            c.g[RA][0] = sx(0x0019_1268);
            call_ptr(c, 0x0014_dc80, 0x0019_1268)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019_1274);
            c.g[A1][0] = 2;
            call_ptr(c, 0x0014_dd58, 0x0019_1274)?;
            c.g[A1][0] = lw(c, at_2(c.g[S0][0], 0x48));
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = sx(0x0046_0000);
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x0019_128c);
            c.g[S0][0] = sx(0x0045_f7d0);
            call_ptr(c, 0x0014_eec8, 0x0019_128c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(frame.at(c, 0));
            c.g[RA][0] = sx(0x0019_129c);
            c.g[A2][0] = c.g[V0][0];
            call_ptr(c, 0x0041_7828, 0x0019_129c)?;
            c.g[RA][0] = sx(0x0019_12a4);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_18c0, 0x0019_12a4)?;
            c.g[RA][0] = sx(0x0019_12ac);
            c.g[A0][0] = sx(frame.at(c, 0));
            call_ptr(c, 0x0031_7670, 0x0019_12ac)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0019_12b8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1f18, 0x0019_12b8)?;
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `GetHashValue32(const char*)`.
pub(crate) const OC846_HASH: u32 = 0x0031_7670;

/// `cUIScreen_getObjectByHashName(screen, hash)`.
pub(crate) const OC846_GET_OBJECT: u32 = 0x0039_d860;

/// `cUIState_showObjSafe(state, hash)`.
pub(crate) const OC846_SHOW_OBJ: u32 = 0x0039_e948;

/// `cUIState_hideObjSafe(state, hash)`.
pub(crate) const OC846_HIDE_OBJ: u32 = 0x0039_e8b8;

/// `sprintf(buf, fmt, ...)`.
pub(crate) const OC846_SPRINTF: u32 = 0x0041_7828;

/// `cUIEngine_addScreenByHashName(engine, state, hash, 0)`.
pub(crate) const OC846_ADD_SCREEN: u32 = 0x0039_7cc0;

/// `cUIScreen_playFrame(screen, 0, 0)`.
pub(crate) const OC846_PLAY_FRAME: u32 = 0x0039_c870;

/// `cUIListBox_addEntryByStringID(list, hash, 0)`.
pub(crate) const OC846_ADD_ENTRY: u32 = 0x0039_a568;

/// `cBE_getBE()`.
pub(crate) const OC846_GET_BE: u32 = 0x0014_dc80;

/// `cBE_getRider(be, 1)`.
pub(crate) const OC846_GET_RIDER: u32 = 0x0014_dd58;

/// Rider page lookup (`a1` = the character byte at +0x44).
pub(crate) const OC846_RIDER_PAGE: u32 = 0x0014_7488;

/// `cUIText_setString(obj, hash)`.
pub(crate) const OC846_SET_TEXT: u32 = 0x003a_1000;

/// Table base 0x460000 / 0x4a0000 the string addresses are built from.
pub(crate) const OC846_LO: u32 = 0x0046_0000;

pub(crate) const OC846_HI: u32 = 0x004a_0000;

/// `cFEStateRiderBio` fields.
pub(crate) const OC846_BIO_SCREEN: u32 = 0x40;

pub(crate) const OC846_BIO_CHAR: u32 = 0x44;

pub(crate) const OC846_BIO_RIDER_PAGE: u32 = 0x48;

pub(crate) const OC846_BIO_PAGE: u32 = 0x4c;

/// `cFEStateRiderBio::Init` (0x1908c0): creates the rider-bio screen (stored at
/// +0x40; plays its first frame), fills its list box with the four page
/// names, looks up the rider (`cBE_getBE` -> rider 1) and its page (+0x48),
/// calls the rider's vtable method, then shows page 0 and the rider image.
pub fn cfe_state_rider_bio_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(OC846_LO - 0x1c68);
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        call_ptr(c, OC846_HASH, 0x0019_08e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, OC846_ADD_SCREEN, 0x0019_08f8)?;
        let this = c.g[S1][0] as u32;
        let screen = c.g[V0][0];
        Addr(this.wrapping_add(OC846_BIO_SCREEN)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, OC846_PLAY_FRAME, 0x0019_0910)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(OC846_LO - 0x1c58);
        call_ptr(c, OC846_HASH, 0x0019_091c)?;
        c.g[A0][0] = Addr(this.wrapping_add(OC846_BIO_SCREEN)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, OC846_GET_OBJECT, 0x0019_0928)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 28;
            // (string, hash-call return, add-entry return)
            const NAMES: [(u32, u32, u32); 4] = [
                (0x1c48, 0x0019_0940, 0x0019_0950),
                (0x1c38, 0x0019_095c, 0x0019_096c),
                (0x1c28, 0x0019_0978, 0x0019_0988),
                (0x1c18, 0x0019_0994, 0x0019_09a4),
            ];
            for (off, ret_hash, ret_add) in NAMES {
                c.g[A0][0] = sx(OC846_LO - off);
                call_ptr(c, OC846_HASH, ret_hash)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, OC846_ADD_ENTRY, ret_add)?;
            }
        }
        c.cycles += 25;
        call_ptr(c, OC846_GET_BE, 0x0019_09ac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, OC846_GET_RIDER, 0x0019_09b8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this.wrapping_add(OC846_BIO_CHAR)).lb(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC846_RIDER_PAGE, 0x0019_09c8)?;
        let rider_page = c.g[V0][0];
        Addr(this.wrapping_add(OC846_BIO_RIDER_PAGE)).sw(c, rider_page);
        let rider = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(rider.wrapping_add(0xc)).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(0x14)).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx(rider.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0019_09e0)?;
        Addr(this.wrapping_add(OC846_BIO_PAGE)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0019_0a08,
            f: ee::s02::f_00190a08,
        }
        .call(c, 0x0019_09ec)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0019_0cd8,
            f: ee::s02::f_00190cd8,
        }
        .call(c, 0x0019_09f4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The nine objects every rider-bio page shows or hides, in order:
/// (string address, hide instead of show). The strings sit in two tables
/// (`OC846_LO - x` and `OC846_HI + x`).
pub(crate) const OC846_PAGE_OBJECTS: [u32; 9] = [
    OC846_LO - 0x1c08,
    OC846_HI + 0x1610,
    OC846_LO - 0x1bf8,
    OC846_LO - 0x1be8,
    OC846_LO - 0x1bd8,
    OC846_HI + 0x1618,
    OC846_LO - 0x1bc8,
    OC846_LO - 0x1bb0,
    OC846_LO - 0x1ba0,
];

/// `showObjSafe`/`hideObjSafe(self, str)` with `self` in `s0`.
pub(crate) fn oc846_page_obj(c: &mut Cpu, str_addr: u32, hide: bool, ret: u32) -> Flow {
    c.g[A1][0] = sx(str_addr);
    c.g[A0][0] = c.g[S0][0];
    let f = if hide { OC846_HIDE_OBJ } else { OC846_SHOW_OBJ };
    call_ptr(c, f, ret)
}

/// One page body: nine show/hide calls (returns `first_ret`, `+0x10`, ...),
/// then the page's fill function. `hide[i]` is true for "hide".
pub(crate) fn oc846_page_body(
    c: &mut Cpu,
    hide: [bool; 9],
    first_ret: u32,
    fill: Callee,
    fill_ret: u32,
) -> Flow {
    for (i, &h) in hide.iter().enumerate() {
        oc846_page_obj(c, OC846_PAGE_OBJECTS[i], h, first_ret + 0x10 * i as u32)?;
    }
    c.g[A0][0] = c.g[S0][0];
    fill.call(c, fill_ret)
}

/// `cFEStateRiderBio::ShowPage` (0x190a08): switches the rider-bio screen to
/// the page in +0x4c (0 DNA, 1 faves, 2 Q and A, 3 bio): each page shows its
/// own objects, hides the others and calls its fill function. Other page
/// numbers do nothing.
pub fn cfe_state_rider_bio_show_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(OC846_BIO_PAGE)).lw(c);
        let page = c.g[V1][0];
        c.g[V0][0] = ((page as i64) < 2i64) as u64;
        // 0: DNA, 1: faves, 2: Q and A, 3: full bio, None: nothing.
        let which: Option<u8> = if page == 1 {
            Some(1)
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0x2;
                c.cycles += 2;
                let is2 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                if is2 {
                    Some(2)
                } else {
                    c.cycles += 2;
                    let is3 = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x0046_0000;
                    if is3 {
                        Some(3)
                    } else {
                        c.cycles += 2;
                        None
                    }
                }
            } else {
                c.cycles += 2;
                let is0 = c.g[V1][0] == 0;
                c.g[A1][0] = 0x0046_0000;
                if is0 {
                    Some(0)
                } else {
                    c.cycles += 2;
                    None
                }
            }
        };
        match which {
            Some(0) => {
                c.cycles += 39;
                oc846_page_body(
                    c,
                    [false, false, true, true, true, true, true, false, false],
                    0x0019_0a64,
                    Callee {
                        addr: 0x0019_0de0,
                        f: ee::s02::f_00190de0,
                    },
                    0x0019_0aec,
                )?;
            }
            Some(1) => {
                c.cycles += 40;
                oc846_page_body(
                    c,
                    [true, true, false, false, true, true, true, false, false],
                    0x0019_0b04,
                    Callee {
                        addr: 0x0019_0f48,
                        f: ee::s02::f_00190f48,
                    },
                    0x0019_0b8c,
                )?;
            }
            Some(2) => {
                c.cycles += 40;
                oc846_page_body(
                    c,
                    [true, true, true, true, false, false, true, true, true],
                    0x0019_0ba4,
                    Callee {
                        addr: 0x0019_10c0,
                        f: ee::s02::f_001910c0,
                    },
                    0x0019_0c2c,
                )?;
            }
            Some(_) => {
                c.cycles += 38;
                oc846_page_body(
                    c,
                    [true, true, true, true, true, true, false, true, true],
                    0x0019_0c40,
                    Callee {
                        addr: 0x0019_1228,
                        f: ee::s02::f_00191228,
                    },
                    0x0019_0cc8,
                )?;
            }
            None => {}
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRiderBio::ShowRiderImage` (0x190cd8): for each of the ten rider
/// pictures (names from the table at 0x45e470, `sprintf` of the picture name
/// into a stack buffer), shows the picture object and its frame object when
/// the picture is the rider's page (+0x48), else hides both.
pub fn cfe_state_rider_bio_show_rider_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S1, 0x90);
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[SP][0];
        frame.save(c, S3, 0x70);
        c.g[S2][0] = sx(sp(c).wrapping_add(0x30));
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save_ra(c, 0x50);
        c.g[S4][0] = 0x4a_0000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1b90));
        // Unaligned 40-byte copy of the picture table to the bottom of the frame.
        let table = c.g[A2][0] as u32;
        let dst = sp(c);
        for (off, reg) in [(0x0u32, 3usize), (0x8, 4), (0x10, 5), (0x18, 3), (0x20, 4)] {
            c.ldl(reg, table.wrapping_add(off + 7));
            c.ldr(reg, table.wrapping_add(off));
            c.sdl(reg, dst.wrapping_add(off + 7));
            c.sdr(reg, dst.wrapping_add(off));
        }
        loop {
            c.cycles += 7;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1670));
            call_ptr(c, OC846_SPRINTF, 0x0019_0d70)?;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(OC846_BIO_RIDER_PAGE)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 7;
            if c.g[S3][0] == c.g[V0][0] {
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, OC846_SHOW_OBJ, 0x0019_0d84)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, OC846_SHOW_OBJ, 0x0019_0d90)?;
            } else {
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, OC846_HIDE_OBJ, 0x0019_0da4)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, OC846_HIDE_OBJ, 0x0019_0db0)?;
            }
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_0d60);
                }
                continue;
            }
            break;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRiderBio::FillDNA` (0x190de0): copies the DNA table (0x45e998,
/// 10 pages of 8 name pointers) to the stack, then for each of the eight DNA
/// rows formats "name N" into the buffer after it, looks the row's text object
/// up in the screen (+0x40) and sets its string from the table row of the
/// rider's page (+0x48).
pub fn cfe_state_rider_bio_fill_dna(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x1c0);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S4, 0x170);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1668));
        frame.save(c, S0, 0x1b0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x1a0);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S2, 0x190);
        // The table address is 8-byte aligned (0x45e998): the generated body's
        // unaligned ldl/ldr copy loop is dead code.
        c.g[V0][0] = c.g[V1][0] & 0x7;
        frame.save(c, S3, 0x180);
        frame.save(c, S5, 0x160);
        frame.save_ra(c, 0x150);
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x140));
        c.g[A0][0] = sx(sp(c).wrapping_add(0x140));
        c.g[S5][0] = 0x4a_0000;
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            let dst = c.g[A1][0] as u32;
            c.g[A2][0] = Addr(src).ld(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x8)).ld(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T1][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let (a2, a3, t0, t1) = (c.g[A2][0], c.g[A3][0], c.g[T0][0], c.g[T1][0]);
            Addr(dst).sd(c, a2);
            Addr(dst.wrapping_add(0x8)).sd(c, a3);
            Addr(dst.wrapping_add(0x10)).sd(c, t0);
            Addr(dst.wrapping_add(0x18)).sd(c, t1);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = sx(dst.wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_0e84);
                }
                continue;
            }
            c.cycles += 1;
            c.g[S3][0] = c.g[A0][0];
            break;
        }
        c.cycles += 3;
        c.g[S2][0] = 0;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 13;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1678));
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, OC846_SPRINTF, 0x0019_0ed0)?;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, OC846_HASH, 0x0019_0ed8)?;
            c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(OC846_BIO_SCREEN)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, OC846_GET_OBJECT, 0x0019_0ee4)?;
            c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
            c.g[S1][0] = c.g[V0][0];
            let missing = c.g[S1][0] == 0;
            c.g[S2][0] = c.g[S0][0];
            if !missing {
                c.cycles += 9;
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(OC846_BIO_RIDER_PAGE)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                call_ptr(c, OC846_HASH, 0x0019_0f0c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, OC846_SET_TEXT, 0x0019_0f18)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_0ec0);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x1b0);
        frame.restore(c, S1, 0x1a0);
        frame.restore(c, S2, 0x190);
        frame.restore(c, S3, 0x180);
        frame.restore(c, S4, 0x170);
        frame.restore(c, S5, 0x160);
        frame.restore_ra(c, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRiderBio_FillFaves(self)` (0x00190f48): copies the 12-entry (10 rows of 12 pointers) name table at 0x45f2d0 to the stack, then for each of the 12 widgets `<n>` (sprintf of the format at 0x4a1680) found on the screen at `+0x40` sets its text from the row `+0x48` of the table.
pub fn cfe_state_rider_bio_fill_faves(c: &mut Cpu) {
    guest(c, |c| {
        'b_191020: {
            'b_190fe4: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x270));
                c.g[V0][0] = 0x460000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.g[S4][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.g[A1][0] = c.g[SP][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x200));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x1f0), c.g[RA][0]);
                if t {
                    break 'b_190fe4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.g[S6][0] = 0x4a0000;
                    'l_190f90: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[V1][0] as u32);
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00190f90);
                            }
                            continue 'l_190f90;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S3][0] = c.g[A0][0];
                            if t {
                                break 'b_191020;
                            } else {
                                break 'b_190fe4;
                            }
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.g[S6][0] = 0x4a0000;
            'l_190ff0: loop {
                c.cycles += 11;
                c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00190ff0);
                    }
                    continue 'l_190ff0;
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[A0][0];
                    break 'b_191020;
                }
            }
        }
        c.cycles += 4;
        c.g[S2][0] = 0u64;
        c.g[S5][0] = 0x30;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        'l_191030: loop {
            'b_191088: {
                c.cycles += 13;
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1680));
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00191040);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00417828, 0x00191040)?;
                c.g[RA][0] = sx(0x00191048);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00317670, 0x00191048)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x00191054);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00191054)?;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[S0][0];
                if t {
                    break 'b_191088;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                    c.mult(0, 2, 2, 21);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x0019107c);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x0019107c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00191088);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00191088)?;
                    break 'b_191088;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 12i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00191030);
                }
                continue 'l_191030;
            } else {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1f0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x270));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateRiderBio_FillQNA(self)` (0x001910c0): like FillFaves for the 4 question widgets (format at 0x4a1688, 4-entry rows of the table at 0x45f730).
pub fn cfe_state_rider_bio_fill_qna(c: &mut Cpu) {
    guest(c, |c| {
        'b_191194: {
            'b_191158: {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
                c.g[V0][0] = 0x460000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8d0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[S4][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A1][0] = c.g[SP][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                if t {
                    break 'b_191158;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S5][0] = 0x4a0000;
                    'l_191104: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[V1][0] as u32);
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00191104);
                            }
                            continue 'l_191104;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S3][0] = c.g[A0][0];
                            if t {
                                break 'b_191194;
                            } else {
                                break 'b_191158;
                            }
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S5][0] = 0x4a0000;
            'l_191164: loop {
                c.cycles += 11;
                c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00191164);
                    }
                    continue 'l_191164;
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = c.g[A0][0];
                    break 'b_191194;
                }
            }
        }
        c.cycles += 3;
        c.g[S2][0] = 0u64;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        'l_1911a0: loop {
            'b_1911f8: {
                c.cycles += 13;
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1688));
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001911b0);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00417828, 0x001911b0)?;
                c.g[RA][0] = sx(0x001911b8);
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00317670, 0x001911b8)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001911c4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001911c4)?;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[S0][0];
                if t {
                    break 'b_1911f8;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x001911ec);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x001911ec)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001911f8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001911f8)?;
                    break 'b_1911f8;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001911a0);
                }
                continue 'l_1911a0;
            } else {
                c.cycles += 9;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::{fake, fall_back, hook, FAKE, LOG_RESULTS};
    use crate::difftest::DATA;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;
    pub(crate) use crate::game::fe::festatecredits::tests::RET_ZERO;
    use ssx3_ee as ee;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn cfe_state_rule_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00191360__generated,
            cfe_state_rule_select_ctor,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x191360, 2);
                let this = DATA + 0x400;
                c.bus.write32(this + 0x10, DATA + 0x500);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_rider_bio_on_widget_action_matches() {
        check(
            ee::s02::f_001912d0__generated,
            c_fe_state_rider_bio_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1912, 0);
                env(c);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let engine = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vt = DATA + 0x1200;
                slot(c, vt, 0x28, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                w32(c, this + 0x10, engine);
                w32(c, engine, obj);
                w32(c, obj + 4, vt);
                w8(c, item + 0x319, seed % 4);
                w32(c, item + 0x18, seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
            },
        );
    }

    /// The gate for the widget-action-6 fake below: only machines holding the
    /// magic at `RIDER_BIO_CTRL` take the fake.
    pub(crate) const RIDER_BIO_CTRL: u32 = DATA + 0x0e_33a0;
    pub(crate) const RIDER_BIO_MAGIC: u32 = 0x0019_1206;

    static RIDER_BIO_PREV_SWITCH_STATE: AtomicUsize = AtomicUsize::new(0);

    /// `UIStateMgr_SwitchState` (0x39f400) logging which call the port made.
    /// (`UIStateMgr_PushPopup`, the wrong callee, has no override slot, so on
    /// the hand side its generated body runs instead of a fake.)
    fn rider_bio_fake_switch_state(c: &mut Cpu) {
        if c.bus.read32(RIDER_BIO_CTRL) != RIDER_BIO_MAGIC {
            return fall_back(
                c,
                &RIDER_BIO_PREV_SWITCH_STATE,
                ee::s13::f_0039f400__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0039_f400, ra);
    }

    /// Widget action 6 with a live rider-screen handler: the handler answers
    /// a nonzero screen pointer, so the port must hand the screen to
    /// `UIStateMgr_SwitchState`. The seeded slot of the existing test only
    /// ever answers zero, so this arm never runs there.
    #[test]
    fn c_fe_state_rider_bio_on_widget_action_six_matches() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            rider_bio_fake_switch_state,
            &RIDER_BIO_PREV_SWITCH_STATE,
        );
        check(
            ee::s02::f_001912d0__generated,
            c_fe_state_rider_bio_on_widget_action,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1912, 0);
                env(c);
                c.bus.write32(RIDER_BIO_CTRL, RIDER_BIO_MAGIC);
                // The handler answers a live screen pointer: forced fake
                // result in zeroed fixture space, so the wrong-callee path
                // the hand port takes is fixed and short.
                c.bus.write32(LOG_RESULTS, DATA + 0x2000);
                let this = DATA + 0x400;
                let item = DATA + 0x800;
                let engine = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vt = DATA + 0x1200;
                slot(c, vt, 0x28, FAKE);
                w32(c, this + 0x10, engine);
                w32(c, engine, obj);
                w32(c, obj + 4, vt);
                w32(c, this + 0x28, DATA + 0x3000);
                w8(c, item + 0x319, seed % 4);
                w32(c, item + 0x18, seed % 8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(item);
                c.g[A2][0] = 6;
            },
        );
    }

    #[test]
    fn cfe_state_rider_bio_fill_bio_info_matches() {
        check(
            ee::s02::f_00191228__generated,
            cfe_state_rider_bio_fill_bio_info,
            SEEDS_2,
            &|c, seed| {
                let _ = base(c, seed, 0x1912_28, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The screen and (for the found branch) the text object.
                let screen = obj + 0x800;
                w(c, obj + 0x40, if seed % 3 == 0 { 0 } else { screen });
                w(c, obj + 0x48, seed % 8);
                // UIScreen_FindObject: count at_2 +0x3c, array at_2 +0x38,
                // entries whose +0x38 is the hash.
                w(c, screen + 0x3c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, screen + 0x38, screen + 0x900);
                let entry = screen + 0xa00;
                w(c, screen + 0x900, entry);
                // The hash of the (zeroed) bio-title string.
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x0045_e438u64]);
                w(c, entry + 0x38, hash as u32);
                // The text-scroll object: no lines, no popup chain.
                w(c, entry + 0xbc, 0);
                wh(c, entry + 0xc8, 0);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::festateruleselect::tests::oc847_arm;

    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC846_BASE: u32 = DATA + 0x20_0000;

    pub(crate) const OC846_CTRL: u32 = OC846_BASE;

    pub(crate) const OC846_MAGIC: u32 = 0x4f43_8460;

    /// Mailbox: result of `addScreenByHashName` (0 = null screen).
    pub(crate) const OC846_MAIL_SCREEN: u32 = OC846_BASE + 0x10;

    /// Mailbox: nonzero makes `getObjectByHashName` return null.
    pub(crate) const OC846_MAIL_NULL_OBJ: u32 = OC846_BASE + 0x14;

    /// Mailbox: the rider page `0x147488` returns.
    pub(crate) const OC846_MAIL_PAGE: u32 = OC846_BASE + 0x18;

    /// A UI object: +8 its vtable, whose +0x48 / +0x4c is `{delta, method}`.
    pub(crate) const OC846_UIOBJ: u32 = OC846_BASE + 0x1000;

    pub(crate) const OC846_UIVT: u32 = OC846_BASE + 0x2000;

    /// The rider object (`cBE_getRider`): +0xc its vtable (+0x10 / +0x14).
    pub(crate) const OC846_RIDER: u32 = OC846_BASE + 0x3000;

    pub(crate) const OC846_RIDER_VT: u32 = OC846_BASE + 0x4000;

    pub(crate) const OC846_SCREEN: u32 = OC846_BASE + 0x6000;

    /// The `cFEStateRiderBio` / `cFEStateProfileLoad` under test.
    pub(crate) const OC846_STATE: u32 = OC846_BASE + 0x8000;

    pub(crate) const OC846_WORLD: u32 = OC846_BASE + 0x9000;

    /// The fake callee: logs the call like `fakes::fake_call`, then gives the
    /// callees that return objects the fixture's objects.
    pub(crate) fn oc846_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let null_obj = c.bus.read32(OC846_MAIL_NULL_OBJ) != 0;
        let screen = c.bus.read32(OC846_MAIL_SCREEN);
        let page = c.bus.read32(OC846_MAIL_PAGE);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            OC846_GET_OBJECT => c.g[V0][0] = if null_obj { 0 } else { OC846_UIOBJ as u64 },
            OC846_ADD_SCREEN => c.g[V0][0] = screen as u64,
            OC846_GET_RIDER => c.g[V0][0] = OC846_RIDER as u64,
            OC846_RIDER_PAGE => c.g[V0][0] = page as u64,
            _ => {}
        }
    }

    macro_rules! oc846_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC846_CTRL) != OC846_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc846_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc846_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc846_fakes!(
        (
            oc846_fake_317670,
            OC846_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc846_fake_39d860,
            OC846_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc846_fake_39e948,
            OC846_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc846_fake_39e8b8,
            OC846_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc846_fake_417828,
            OC846_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc846_fake_397cc0,
            OC846_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc846_fake_39c870,
            OC846_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc846_fake_39a568,
            OC846_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc846_fake_14dc80,
            OC846_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc846_fake_14dd58,
            OC846_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc846_fake_147488,
            OC846_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc846_fake_227f80,
            OC846_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc846_fake_3a1000,
            OC846_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc846_fake_191228,
            OC846_PREV_191228,
            ee::s02::OVERRIDE_00191228,
            ee::s02::f_00191228__generated,
            0x0019_1228
        ),
    );

    /// The shared fixture: the machine with the fakes armed, the UI object and
    /// rider vtables, the mailboxes and the state object (`this` in a0).
    /// `salt` is the function's address.
    pub(crate) fn oc846_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC846_CTRL, OC846_MAGIC);
        c.bus.write32(GAME_GP - 0x848, OC846_WORLD);
        // Object vtable: slot +0x48 {delta, method}.
        c.bus.write32(OC846_UIOBJ + 8, OC846_UIVT);
        c.bus
            .write16(OC846_UIVT + 0x48, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(OC846_UIVT + 0x4c, fakes::FAKE);
        // Rider: +0xc vtable, slot +0x10 {delta, method}.
        c.bus.write32(OC846_RIDER + 0xc, OC846_RIDER_VT);
        c.bus
            .write16(OC846_RIDER_VT + 0x10, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(OC846_RIDER_VT + 0x14, fakes::FAKE + 0x10);
        c.bus.write32(
            OC846_MAIL_SCREEN,
            if seed % 4 == 0 { 0 } else { OC846_SCREEN },
        );
        c.bus
            .write32(OC846_MAIL_NULL_OBJ, ((seed / 4) % 3 == 0) as u32);
        c.bus.write32(OC846_MAIL_PAGE, (seed / 3) % 10);
        // The state: the screen (+0x40), the character (+0x44), the rider
        // page (+0x48), the shown page (+0x4c).
        c.bus.write32(OC846_STATE + 0x10, r.next_u32());
        c.bus.write32(OC846_STATE + 0x40, OC846_SCREEN);
        c.bus.write8(OC846_STATE + 0x44, r.next_u32() as u8);
        c.bus.write32(OC846_STATE + 0x48, (seed / 7) % 10);
        c.bus.write32(OC846_STATE + 0x4c, 0);
        c.g[A0][0] = sx(OC846_STATE);
        r
    }

    /// The tables `FillDNA` and `ShowRiderImage` copy: random words.
    pub(crate) fn oc846_tables(c: &mut Cpu, r: &mut Rng) {
        fill(c, r, 0x0045_e998, 0x50);
        fill(c, r, 0x0045_e470, 0x10);
    }

    /// `cFEStateRiderBio::Init`: screen created or null (no first frame),
    /// list-box object found or null (no entries), the rider lookup and its
    /// vtable call, then the real `ShowPage` (page 0, the real `FillDNA`) and
    /// `ShowRiderImage`.
    #[test]
    fn cfe_state_rider_bio_init_matches() {
        oc846_install_fakes();
        check(
            ee::s02::f_001908c0__generated,
            cfe_state_rider_bio_init,
            240,
            &|c, seed| {
                let mut r = oc846_setup(c, seed, 0x1908c0, 60);
                c.pc = 0x0019_08c0;
                oc846_tables(c, &mut r);
            },
        );
    }

    /// `cFEStateRiderBio::ShowPage`: pages -1 to 5 (0 to 3 show their own
    /// objects and call their fill function; the others do nothing).
    #[test]
    fn cfe_state_rider_bio_show_page_matches() {
        oc846_install_fakes();
        check(
            ee::s02::f_00190a08__generated,
            cfe_state_rider_bio_show_page,
            280,
            &|c, seed| {
                let mut r = oc846_setup(c, seed, 0x190a08, 40);
                c.pc = 0x0019_0a08;
                oc846_tables(c, &mut r);
                let page = [-1i32, 0, 1, 2, 3, 4, 5][(seed % 7) as usize];
                c.bus.write32(OC846_STATE + 0x4c, page as u32);
            },
        );
    }

    /// `cFEStateRiderBio::ShowRiderImage`: the ten pictures, the one at the
    /// rider's page (0 to 11: none for 10 and 11) shown, the others hidden.
    #[test]
    fn cfe_state_rider_bio_show_rider_image_matches() {
        oc846_install_fakes();
        check(
            ee::s02::f_00190cd8__generated,
            cfe_state_rider_bio_show_rider_image,
            200,
            &|c, seed| {
                let mut r = oc846_setup(c, seed, 0x190cd8, 32);
                c.pc = 0x0019_0cd8;
                oc846_tables(c, &mut r);
                c.bus.write32(OC846_STATE + 0x48, seed % 12);
            },
        );
    }

    /// `cFEStateRiderBio::FillDNA`: the table copy, then eight rows, each row's
    /// text object found (or null: no `setString`) and set.
    #[test]
    fn cfe_state_rider_bio_fill_dna_matches() {
        oc846_install_fakes();
        check(
            ee::s02::f_00190de0__generated,
            cfe_state_rider_bio_fill_dna,
            200,
            &|c, seed| {
                let mut r = oc846_setup(c, seed, 0x190de0, 36);
                c.pc = 0x0019_0de0;
                oc846_tables(c, &mut r);
            },
        );
    }

    /// The rider-bio name tables: 10 rows copied to the stack, the screen at
    /// `+0x40`, the row index at `+0x48`.
    pub(crate) fn oc847_bio_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        table: u32,
        words: u32,
        calls: u32,
    ) {
        let mut r = machine(c, seed, salt, calls);
        oc847_arm(c, &mut r, seed);
        fill(c, &mut r, table, words);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x40, 0x1234_5678);
        c.bus.write32(this + 0x48, r.next_u32() % 10);
    }

    /// `FillFaves`: all 12 favourites widgets (found or missing by mask) of
    /// every table row, and the unwinds of its sprintf/hash/find/text calls.
    /// The unaligned-source copy loop (`ldl`/`ldr`) is unreachable: the table
    /// address is a constant with `addr & 7 == 0`.
    #[test]
    fn cfe_state_rider_bio_fill_faves_matches() {
        check(
            ee::s02::f_00190f48__generated,
            cfe_state_rider_bio_fill_faves,
            200,
            &|c, seed| oc847_bio_fixture(c, seed, 0x8471, 0x0045_f2d0, 0x78, 60),
        );
    }

    /// `FillQNA`: the 4 question widgets of every table row.
    #[test]
    fn cfe_state_rider_bio_fill_qna_matches() {
        check(
            ee::s02::f_001910c0__generated,
            cfe_state_rider_bio_fill_qna,
            200,
            &|c, seed| oc847_bio_fixture(c, seed, 0x8472, 0x0045_f730, 0x28, 20),
        );
    }
}
