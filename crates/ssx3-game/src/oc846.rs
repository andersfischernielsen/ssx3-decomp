//! Staging module oc846 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc846`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
];

/// `GetHashValue32(const char*)`.
const OC846_HASH: u32 = 0x0031_7670;
/// `cUIScreen_getObjectByHashName(screen, hash)`.
const OC846_GET_OBJECT: u32 = 0x0039_d860;
/// `cUIState_showObjSafe(state, hash)`.
const OC846_SHOW_OBJ: u32 = 0x0039_e948;
/// `cUIState_hideObjSafe(state, hash)`.
const OC846_HIDE_OBJ: u32 = 0x0039_e8b8;
/// `sprintf(buf, fmt, ...)`.
const OC846_SPRINTF: u32 = 0x0041_7828;
/// `cUIEngine_addScreenByHashName(engine, state, hash, 0)`.
const OC846_ADD_SCREEN: u32 = 0x0039_7cc0;
/// `cUIScreen_playFrame(screen, 0, 0)`.
const OC846_PLAY_FRAME: u32 = 0x0039_c870;
/// `cUIListBox_addEntryByStringID(list, hash, 0)`.
const OC846_ADD_ENTRY: u32 = 0x0039_a568;
/// `cBE_getBE()`.
const OC846_GET_BE: u32 = 0x0014_dc80;
/// `cBE_getRider(be, 1)`.
const OC846_GET_RIDER: u32 = 0x0014_dd58;
/// Rider page lookup (`a1` = the character byte at +0x44).
const OC846_RIDER_PAGE: u32 = 0x0014_7488;
/// `cSSXApp_getMCManager(app)`.
const OC846_GET_MC: u32 = 0x0022_7f80;
/// `cUIText_setString(obj, hash)`.
const OC846_SET_TEXT: u32 = 0x003a_1000;

/// Table base 0x460000 / 0x4a0000 the string addresses are built from.
const OC846_LO: u32 = 0x0046_0000;
const OC846_HI: u32 = 0x004a_0000;

/// `cFEStateRiderBio` fields.
const OC846_BIO_SCREEN: u32 = 0x40;
const OC846_BIO_CHAR: u32 = 0x44;
const OC846_BIO_RIDER_PAGE: u32 = 0x48;
const OC846_BIO_PAGE: u32 = 0x4c;

/// One "get the object named `str_addr`, then call its `setVisible`
/// (vtable slot at +0x48/+0x4c of the object's vtable at +8)" step of the
/// profile-load title screens. `self` is in `s0`; `ret` is the return address
/// of the hash call, the other two calls return to `ret + 0xc` and
/// `ret + 0x24`.
fn oc846_title_set_visible(c: &mut Cpu, str_addr: u32, visible: u64, ret: u32) -> Flow {
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
const OC846_PAGE_OBJECTS: [u32; 9] = [
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
fn oc846_page_obj(c: &mut Cpu, str_addr: u32, hide: bool, ret: u32) -> Flow {
    c.g[A1][0] = sx(str_addr);
    c.g[A0][0] = c.g[S0][0];
    let f = if hide { OC846_HIDE_OBJ } else { OC846_SHOW_OBJ };
    call_ptr(c, f, ret)
}

/// One page body: nine show/hide calls (returns `first_ret`, `+0x10`, ...),
/// then the page's fill function. `hide[i]` is true for "hide".
fn oc846_page_body(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
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
            fn oc846_install_fakes() {
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
    fn oc846_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
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

    /// The tables `FillDNA` and `ShowRiderImage` copy: random words.
    fn oc846_tables(c: &mut Cpu, r: &mut Rng) {
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
}
