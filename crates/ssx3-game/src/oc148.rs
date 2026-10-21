//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Front-end/utility code: rider init, input-binding queries, MPEG-2 bit
//! reader fields, the rider-model bone lookup, the bolts free-db helper and
//! the options-menu page destructors.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_0738, rider_init_data, "Rider_InitData"),
    (
        0x0012_76f0,
        player_get_trick_button,
        "Player_GetTrickButton",
    ),
    (
        0x0018_0300,
        fe_ui_input_is_any_button,
        "FEUIInput_IsAnyButton",
    ),
    (
        0x0019_e680,
        fe_module_rider_preview_find_bones,
        "FEModule_RiderPreviewFindBones",
    ),
    (0x0014_d240, bolts_free_db, "Bolts_FreeDB"),(
        0x0024_d998,
        visual_effects_test_menu_dtor,
        "VisualEffectsTestMenu_dtor",
    ),
    (
        0x0024_f6c0,
        visual_effects_main_menu_dtor,
        "VisualEffectsMainMenu_dtor",
    ),(
        0x0024_e200,
        snowfall_toggles_menu_dtor,
        "SnowfallTogglesMenu_dtor",
    ),(
        0x0018_e178,
        c_fe_state_options_device_select_v37,
        "cFEStateOptionsDeviceSelect_v37",
    ),
    (
        0x0018_ef70,
        c_fe_state_profile_load_v37,
        "cFEStateProfileLoad_v37",
    ),(
        0x001f_7f08,
        ov_screen_top_times_on_activate,
        "OVScreenTopTimes_OnActivate",
    ),];

// ---------------------------------------------------------------------------
// Addresses.

/// `int InputBinding_EvalBool(InputBinding*, int value)` — evaluates the input
/// map bound to a button, non-zero when the button is held.
const INPUT_BINDING_EVAL_BOOL: u32 = 0x0032_0c48;
/// `int MPEG2_Get_Bits(int n)` — read `n` bits from the MPEG-2 bit reader.
const MPEG2_GET_BITS: u32 = 0x003b_1300;
/// `int RiderModel_FindBone(RiderModel*, int bucket, const char *name)`.
const RIDER_MODEL_FIND_BONE: u32 = 0x0031_0c48;
/// `void *EA_MemSet(void*, int, int)`.
const EA_MEMSET: u32 = 0x003e_6448;
/// `void EA_MemFree(void*)`.
const EA_MEM_FREE: u32 = 0x0031_7e98;
/// `void MenuSysItem_dtor(MenuSysItem*, int flags)` and the page destructor.
const MENU_SYS_ITEM_DTOR: u32 = 0x002c_a280;
const MENU_SYS_PAGE_DTOR: u32 = 0x002c_aa80;

/// The MPEG-2 decoder's bit reader state pointer (`0x510000 - 0x5f78`).
const MPEG2_BIT_STATE: u32 = 0x0050_a088;

/// Direct calls through the generated function (which dispatches into the
/// replacement when one is installed).
mod callee {
    use super::{ee, Callee};

    pub const C_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::f_00186518,
    };
    pub const SOUND_MANAGER_REF_DTOR: Callee = Callee {
        addr: 0x002a_dee8,
        f: ee::s08::f_002adee8,
    };
}

// ---------------------------------------------------------------------------
// Rider_InitData.

/// `void Rider_InitData(Rider *rider)` (0x140738): clear the rider's control /
/// trick state words at `+0x1f0..+0x2d0`, set the `-1` sentinel at `+0x430` and
/// fill the 64-word input history at `+0x5b8` with `-1`.
pub fn rider_init_data(c: &mut Cpu) {
    c.cycles += 66;
    let rider = abi::arg(c, 0);
    // The original writes the 57 state words in a shuffled order; memory is
    // the same either way.
    let mut off = 0x1f0u32;
    while off <= 0x2d0 {
        Addr(rider).off(off).sw(c, 0);
        off += 4;
    }
    Addr(rider).off(0x430).sw(c, u64::MAX);

    // 64 words of -1 at +0x5b8. The loop's back edge can deliver an event, so
    // the address/counter live in their original registers.
    c.g[V0][0] = u64::MAX;
    c.g[A1][0] = u64::MAX;
    c.g[A2][0] = 0x3f;
    c.g[A3][0] = u64::MAX;
    c.g[V1][0] = sx(rider.wrapping_add(0x5b8));
    loop {
        c.cycles += 7;
        Addr(r32(c, V1)).sw(c, c.g[A1][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        let more = c.g[A2][0] != c.g[A3][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_0840);
        }
    }
}

// ---------------------------------------------------------------------------
// Player / FE input queries.

/// `int Player_GetTrickButton(Player *p)` (0x1276f0): index of the first held
/// trick button (bindings 0x1f..0x2c map to 0..13), 14 for the last binding
/// (0x2d) or -1 when none is held.
pub fn player_get_trick_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);

        for i in 0..14u32 {
            if i > 0 {
                c.cycles += 5;
            }
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xdf0)).lw(c);
            c.g[A1][0] = (0x1f + i) as u64;
            call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0012_770c + 0x14 * i)?;
            let held = c.g[V0][0] != 0;
            c.g[V0][0] = i as u64;
            if held {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        // Binding 0x2d: report 14 when held, -1 otherwise.
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xdf0)).lw(c);
        c.g[A1][0] = 0x2d;
        call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0012_7824)?;
        c.g[A0][0] = 0xe;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.g[V1][0];

        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The bindings `FEUIInput_IsAnyButton` tests, in the original's order.
const ANY_BUTTON_CODES: [u64; 16] = [
    0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7d, 0x7c, 0x90, 0x92, 0x91, 0x71,
];

/// `int FEUIInput_IsAnyButton(FEUIInput *input)` (0x180300): non-zero when any
/// of the sixteen menu buttons is held.
pub fn fe_ui_input_is_any_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;

        let mut last_held = false;
        for (i, code) in ANY_BUTTON_CODES.iter().enumerate() {
            if i > 0 {
                c.cycles += 5;
            }
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            c.g[A1][0] = *code;
            call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0018_0324 + 0x14 * i as u32)?;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = 1;
                last_held = i + 1 == ANY_BUTTON_CODES.len();
                break;
            }
        }
        // A held button costs one extra cycle; the last one costs two (it
        // falls through the tail check as well).
        if last_held {
            c.cycles += 2;
        } else if c.g[S1][0] != 0 {
            c.cycles += 1;
        }
        c.g[V0][0] = c.g[S1][0];

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider preview bones.

/// The bone-name addresses `FEModule_RiderPreviewFindBones` looks up and the
/// second argument each is found with (`.rodata` strings).
const PREVIEW_BONE_NAMES: [(u32, u64); 14] = [
    (0x004a_1970, 0),
    (0x004a_1978, 0),
    (0x0046_0a00, 0),
    (0x0046_0a10, 0),
    (0x0046_0a20, 0),
    (0x0046_0a30, 0),
    (0x0046_0a40, 0),
    (0x0046_0a50, 0),
    (0x0046_0a60, 0),
    (0x0046_0a70, 0),
    (0x0046_0a80, 0),
    (0x0046_0a90, 0),
    (0x0046_0aa0, 1),
    (0x0046_0ab0, 1),
];

/// `void FEModule_RiderPreviewFindBones(FEModule *m)` (0x19e680): resolve the
/// preview rider's bones by name and cache the indices at `+0xc7c..+0xcb0`.
pub fn fe_module_rider_preview_find_bones(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 92;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;

        for (i, &(name, second)) in PREVIEW_BONE_NAMES.iter().enumerate() {
            c.g[A2][0] = name as u64;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A1][0] = second;
            call_ptr(c, RIDER_MODEL_FIND_BONE, 0x0019_e6a4 + 0x18 * i as u32)?;
            Addr(r32(c, S0).wrapping_add(0xc7c + 4 * i as u32)).sw(c, c.g[V0][0]);
        }

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolts free-db.

/// The eleven 0x78-byte sub-blocks cleared by [`bolts_free_db`].
const BOLT_SUB_BLOCKS: [u32; 11] = [
    0x02c, 0x0a4, 0x11c, 0x194, 0x20c, 0x284, 0x2fc, 0x374, 0x3ec, 0x464, 0x4dc,
];

/// `void Bolts_FreeDB(BoltsDB *db)` (0x14d240): free the database's two heap
/// blocks and clear the eleven per-mode sub-blocks.
pub fn bolts_free_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        let db = r32(c, S0);

        c.g[A0][0] = Addr(db).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x0014_d264)?;
        }
        Addr(db).sw(c, 0);

        c.cycles += 56;
        // The first block's clear also covers the small header words.
        for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28] {
            Addr(db).off(off).sw(c, 0);
        }
        c.g[A1][0] = 0;
        for (i, off) in BOLT_SUB_BLOCKS.iter().enumerate() {
            c.g[A2][0] = 0x78;
            c.g[A0][0] = sx(db.wrapping_add(*off));
            call_ptr(c, EA_MEMSET, 0x0014_d29c + 0x10 * i as u32)?;
        }

        c.g[A0][0] = Addr(db).off(0x18).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x0014_d350)?;
        }
        Addr(db).off(0x18).sw(c, 0);

        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MPEG-2 picture coding extension.

/// `void MPEG2_picture_coding_extension(void)` (0x3b20b8): read the 21 fields
/// of the picture coding extension into the decoder state. The bit widths and
/// return addresses are the original's call schedule.
const PCE_FIRST: [(u32, u32); 16] = [
    (4, 0x003b_20d4),
    (4, 0x003b_20e8),
    (4, 0x003b_20f4),
    (4, 0x003b_2100),
    (2, 0x003b_210c),
    (2, 0x003b_211c),
    (1, 0x003b_212c),
    (1, 0x003b_213c),
    (1, 0x003b_214c),
    (1, 0x003b_215c),
    (1, 0x003b_216c),
    (1, 0x003b_217c),
    (1, 0x003b_218c),
    (1, 0x003b_219c),
    (1, 0x003b_21ac),
    (1, 0x003b_21bc),
];

const PCE_LAST: [(u32, u32); 5] = [
    (1, 0x003b_21d0),
    (3, 0x003b_21e0),
    (1, 0x003b_21f0),
    (7, 0x003b_2200),
    (8, 0x003b_2210),
];

pub fn mpeg2_picture_coding_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x0051_0000;
        // The state block starts at 0x510000-0x6a90 and grows by four bytes
        // per field; two fields live in the bit reader's sub-object.
        c.g[S0][0] = 0x0051_0000;
        c.g[S0][0] = sx(0x0051_0000u32.wrapping_sub(0x6a90));
        let base = r32(c, S0);

        // Store field `i` where the original does: fields 10 and 12 live in
        // the bit reader's sub-object, the rest walk the state block.
        fn store(c: &mut Cpu, base: u32, i: usize, v: u64) {
            let dst = match i {
                9 => (Addr(MPEG2_BIT_STATE).lw(c) as u32).wrapping_add(0x428),
                11 => (Addr(MPEG2_BIT_STATE).lw(c) as u32).wrapping_add(0x42c),
                _ => base.wrapping_add(4 * (i - usize::from(i > 9) - usize::from(i > 11)) as u32),
            };
            Addr(dst).sw(c, v);
        }

        for (i, &(width, ret)) in PCE_FIRST.iter().enumerate() {
            match i {
                4..=9 | 11 | 13..=15 => c.g[V1][0] = 0x0051_0000,
                10 | 12 => c.g[V1][0] = Addr(MPEG2_BIT_STATE).lw(c),
                _ => {}
            }
            c.g[A0][0] = width as u64;
            call_ptr(c, MPEG2_GET_BITS, ret)?;
            let v = c.g[V0][0];
            store(c, base, i, v);
        }

        // The sixteenth field gates the rest of the extension.
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 20;
        for (i, &(width, ret)) in PCE_LAST.iter().enumerate() {
            c.g[V1][0] = 0x0051_0000;
            c.g[A0][0] = width as u64;
            call_ptr(c, MPEG2_GET_BITS, ret)?;
            Addr(base.wrapping_add(0x38 + 4 * i as u32)).sw(c, c.g[V0][0]);
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Screen-object helpers.
//
// The front-end states look widgets up by hashed name and then call a method
// on the found object through its vtable (`lh vt+0x48` delta, `lw vt+0x4c`
// function).

/// `int StrHash(const char*)`.
const STR_HASH: u32 = 0x0031_7670;
/// `UIObject *UIScreen_FindObject(UIScreen*, int hash)`.
const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;

/// `StrHash(name)`, then `UIScreen_FindObject(*(this+0x40), hash)`; the found
/// object is left in `v0`.
fn screen_object(c: &mut Cpu, this: u32, name: u32, ret_hash: u32, ret_find: u32) -> Flow {
    c.g[A0][0] = name as u64;
    call_ptr(c, STR_HASH, ret_hash)?;
    c.g[A0][0] = Addr(this).off(0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, UI_SCREEN_FIND_OBJECT, ret_find)
}

/// Call the method at vtable slot `+0x4c` of `*(obj+8)` with `a1 = arg`
/// (the function pointer is fetched into `a2`, as the original does).
fn call_object_a2(c: &mut Cpu, obj: u64, arg: u64, ret: u32) -> Flow {
    c.g[V1][0] = Addr(obj as u32).off(8).lw(c);
    c.g[A1][0] = arg;
    c.g[A0][0] = Addr(r32(c, V1)).off(0x48).lh(c);
    c.g[A2][0] = Addr(r32(c, V1)).off(0x4c).lw(c);
    let func = c.g[A2][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

/// The same, fetching the function pointer into `v1` and the object's model
/// pointer (`*(obj+8)`) into `v0`.
fn call_object_v1(c: &mut Cpu, obj: u64, arg: u64, ret: u32) -> Flow {
    c.g[V0][0] = Addr(obj as u32).off(8).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
    let func = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

// ---------------------------------------------------------------------------
// cFEStateOptionsDeviceSelect_v37 / cFEStateProfileLoad_v37.

/// The UIState's frame (0x40, `s0`/`s1`/`s2` at +0x30/+0x20/+0x10).
fn ui_state_frame(c: &mut Cpu) -> Frame {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    frame.save(c, S1, 0x20);
    frame.save_ra(c, 0);
    frame
}

fn ui_state_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateOptionsDeviceSelect_v37(FEState *s, int selected)` (0x18e178):
/// re-apply the state `selected` to the three device/controller widgets and
/// record it in `+0x234`.
pub fn c_fe_state_options_device_select_v37(c: &mut Cpu) {
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
            c.cycles += 15;
            screen_object(c, this, 0x004a_15e0, 0x0018_e1ac, 0x0018_e1b8)?;
            let obj = c.g[V0][0];
            call_object_a2(c, obj, arg, 0x0018_e1d0)?;

            c.g[V0][0] = Addr(this).off(0x22c).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 12;
                screen_object(c, this, 0x004a_15d8, 0x0018_e27c, 0x0018_e288)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e2a0)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            } else {
                c.cycles += 38;
                screen_object(c, this, 0x004a_15d0, 0x0018_e1e8, 0x0018_e1f4)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e20c)?;
                screen_object(c, this, 0x004a_1458, 0x0018_e218, 0x0018_e224)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e23c)?;
                screen_object(c, this, 0x004a_1460, 0x0018_e248, 0x0018_e254)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e26c)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            }
        }
        ui_state_epilogue(c, frame);
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

// ---------------------------------------------------------------------------
// cFEStateRiderDetail_OnActivate.

/// The widget names `cFEStateRiderDetail_OnActivate` tags with state 5..9.
const RIDER_DETAIL_WIDGETS: [(u32, u64); 5] = [
    (0x0045_d888, 5),
    (0x0045_d898, 6),
    (0x0045_d8a8, 7),
    (0x0045_d8b8, 8),
    (0x0045_d8c8, 9),
];

/// `void cFEStateRiderDetail_OnActivate(FEState *s, int arg)` (0x1833d0): put
/// five rider-detail widgets into their state, then run the base activate.
pub fn c_fe_state_rider_detail_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);

        let this = r32(c, S1);
        for (i, &(name, state)) in RIDER_DETAIL_WIDGETS.iter().enumerate() {
            if i > 0 {
                c.cycles += 9;
            }
            let ret = 0x0018_33f8 + 0x40 * i as u32;
            screen_object(c, this, name, ret, ret + 0xc)?;
            let obj = c.g[V0][0];
            c.g[A1][0] = 1;
            if obj == 0 {
                continue;
            }
            c.cycles += 7;
            call_object_v1(c, obj, 1, ret + 0x2c)?;
            c.g[V0][0] = state;
            Addr(obj as u32).off(0x18).sw(c, c.g[V0][0]);
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::C_FE_STATE_ON_ACTIVATE.call(c, 0x0018_3538)?;

        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// OVScreenTopTimes_OnActivate.

const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;
const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
const SPRINTF: u32 = 0x0041_7828;
const PLAY_OBJECT_ANIM: u32 = 0x0020_a6f0;
/// Format strings in `.rodata` and the global byte that selects the second
/// round of labels.
const LABEL_FORMATS: [u32; 4] = [0x004a_2440, 0x004a_2428, 0x004a_2430, 0x004a_2438];
const LABEL_MODE: u32 = 0x0053_5c11;

/// Four `sprintf(buf, fmt[i], count)` + `PlayObjectAnim(this, buf, anim)`
/// pairs, with the original's return addresses.
fn play_object_labels(c: &mut Cpu, this: u64, count: u64, rets: &[(u32, u32); 4]) -> Flow {
    for i in 0..4 {
        c.g[A1][0] = LABEL_FORMATS[i] as u64;
        c.g[A0][0] = sp(c) as u64;
        c.g[A2][0] = count;
        call_ptr(c, SPRINTF, rets[i].0)?;
        c.g[S0][0] = 0x0047_0000;
        c.g[A1][0] = sp(c) as u64;
        c.g[S0][0] = sx(0x0047_0000u32.wrapping_sub(0x1b18));
        c.g[A0][0] = this;
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, PLAY_OBJECT_ANIM, rets[i].1)?;
    }
    Ok(())
}

/// `void OVScreenTopTimes_OnActivate(OVScreenTopTimes *s)` (0x1f7f08): format
/// and play the screen's label animations for the current mode.
pub fn ov_screen_top_times_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x20);
        c.g[S2][0] = abi::arg64(c, 0);
        let this = c.g[S2][0];

        call_ptr(c, UI_STATE_ON_ACTIVATE, 0x001f_7f28)?;
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x001f_7f30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x001f_7f3c)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x001f_7f44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x001f_7f50)?;

        c.g[S1][0] = Addr(r32(c, S3)).off(0x18).lw(c);
        let negative = (c.g[S1][0] as i64) < 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        if !negative {
            c.cycles += 38;
            play_object_labels(
                c,
                this,
                c.g[S1][0],
                &[
                    (0x001f_7f70, 0x001f_7f88),
                    (0x001f_7f9c, 0x001f_7fac),
                    (0x001f_7fc0, 0x001f_7fd0),
                    (0x001f_7fe4, 0x001f_7ff4),
                ],
            )?;
        }

        // The second block only runs for label mode 2.
        c.cycles += 5;
        c.g[V0][0] = 0x0053_0000;
        c.g[A0][0] = 2;
        c.g[V1][0] = Addr(LABEL_MODE).lb(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S3)).off(0x1c).lw(c);
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            if !negative {
                c.cycles += 39;
                play_object_labels(
                    c,
                    this,
                    c.g[S1][0],
                    &[
                        (0x001f_8028, 0x001f_8040),
                        (0x001f_8054, 0x001f_8064),
                        (0x001f_8078, 0x001f_8088),
                        (0x001f_809c, 0x001f_80ac),
                    ],
                )?;
            }
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SndPlayParams_dtor.

const SOUND_MANAGER_REF_DTOR: u32 = 0x002a_dee8;
const BUILTIN_DELETE: u32 = 0x0031_7e50;

/// The pointer members [`snd_play_params_dtor`] releases, in the original's
/// order (the first is `+0x20`, each block then re-loads the next one).
const SND_PARAM_PTRS: [u32; 14] = [
    0x20, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
];

/// `void SndPlayParams_dtor(SndPlayParams *p, int flags)` (0x2a97c0): free the
/// parameter's pointer members, then (per `flags`) the sound-manager reference
/// and the object itself.
pub fn snd_play_params_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x0048_0000;
        frame.save(c, S0, 0x40);
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_add(0x3be0));
        frame.save(c, S1, 0x30);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = abi::arg64(c, 1);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let keep = c.g[S1][0] != 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A3][0]);
        let this = r32(c, S0);
        if !keep {
            // The non-deleting ctor-style vtable/offset fixup.
            c.cycles += 16;
            c.g[A2][0] = c.ld(0x0048_3be0);
            c.g[V0][0] = c.ld(0x0048_3be8);
            c.g[V1][0] = c.ld(0x0048_3bf0);
            c.g[A0][0] = c.ld(0x0048_3bf8);
            c.g[A1][0] = Addr(this).lw(c);
            let sp_now = sp(c);
            Addr(sp_now.wrapping_add(8)).sd(c, c.g[V0][0]);
            Addr(sp_now.wrapping_add(0x10)).sd(c, c.g[V1][0]);
            Addr(sp_now).sd(c, c.g[A2][0]);
            Addr(sp_now.wrapping_add(0x18)).sd(c, c.g[A0][0]);
            Addr(c.g[A1][0] as u32).off(4).sw(c, sp_now as u64);
            c.g[V1][0] = Addr(0x0048_3be8).lhu(c);
            c.g[V0][0] = Addr(this).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x88));
            c.g[V0][0] = sx(this.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            Addr(sp_now.wrapping_add(8)).sh(c, c.g[V1][0]);
        }

        // Free the pointer members: the first block also loads `+0x20`.
        c.cycles += 3;
        c.g[A0][0] = Addr(this).off(SND_PARAM_PTRS[0]).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x002a_9840)?;
        }
        for (i, &off) in SND_PARAM_PTRS.iter().enumerate().skip(1) {
            c.cycles += 2;
            c.g[A0][0] = Addr(this).off(off).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, EA_MEM_FREE, 0x002a_9840 + 0x14 * i as u32)?;
            }
        }
        c.g[V0][0] = c.g[S1][0] & 2;

        // The sound-manager reference (`flags & 2`), then the object itself
        // (`flags & 1`).
        c.cycles += 2;
        let no_ref = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_ref {
            c.cycles += 4;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[A1][0] = 0;
            callee::SOUND_MANAGER_REF_DTOR.call(c, 0x002a_995c)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }

        c.cycles += 2;
        let no_delete = c.g[V0][0] == 0;
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, BUILTIN_DELETE, 0x002a_9970)?;
        }

        c.cycles += 4;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Options-menu page destructors.
//
// Every one of these destroys a fixed set of `MenuSysItem`s (with the
// non-deleting flag value 2) and then the page itself.

/// Destroy the embedded item at `this + off` (`MenuSysItem_dtor(x, 2)`).
fn item_dtor(c: &mut Cpu, this: u32, off: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(this.wrapping_add(off));
    c.g[A1][0] = 2;
    call_ptr(c, MENU_SYS_ITEM_DTOR, ret)
}

/// The shared body: `offsets[i]` is destroyed at `first_ret + 0xc*i`, then
/// `MenuSysPage_dtor(this, a1)` at the next return address.
fn menu_page_dtor(c: &mut Cpu, offsets: &[u32], first_ret: u32) {
    guest(c, |c| {
        c.cycles += 71;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);

        let this = r32(c, S0);
        for (i, &off) in offsets.iter().enumerate() {
            item_dtor(c, this, off, first_ret + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(
            c,
            MENU_SYS_PAGE_DTOR,
            first_ret + 0xc * offsets.len() as u32,
        )?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffectsTestMenu_dtor` (0x24d998).
pub fn visual_effects_test_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x464, 0x434, 0x404, 0x3d4, 0x3a4, 0x374, 0x344, 0x314, 0x2e4, 0x2b4, 0x284, 0x254,
            0x224, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_d9bc,
    );
}

/// `VisualEffectsMainMenu_dtor` (0x24f6c0).
pub fn visual_effects_main_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x324, 0x308, 0x2ec, 0x2d0, 0x2b4, 0x298, 0x27c, 0x260, 0x244, 0x228, 0x20c, 0x1f0,
            0x1d4, 0x1b8, 0x19c, 0x180, 0x164, 0x14c, 0x130,
        ],
        0x0024_f6e4,
    );
}

/// `BoardTrailMenu_dtor` (0x24dfa0).
pub fn board_trail_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x59c, 0x56c, 0x53c, 0x508, 0x4d4, 0x4a0, 0x46c, 0x438, 0x404, 0x3d0, 0x39c, 0x368,
            0x334, 0x300, 0x2cc, 0x298, 0x264, 0x230, 0x1fc, 0x1c8, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dfc4,
    );
}

/// `LightGlowMenu_dtor` (0x24dab8).
pub fn light_glow_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x700, 0x6cc, 0x698, 0x664, 0x630, 0x600, 0x5cc, 0x598, 0x564, 0x530, 0x4fc, 0x4c8,
            0x494, 0x460, 0x42c, 0x3fc, 0x3c8, 0x394, 0x360, 0x32c, 0x2f8, 0x2c4, 0x290, 0x25c,
            0x228, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dadc,
    );
}

/// `AvalancheMenu_dtor` (0x250ae0).
pub fn avalanche_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x77c, 0x748, 0x714, 0x6e0, 0x6b0, 0x67c, 0x648, 0x614, 0x5e0, 0x5ac, 0x578, 0x544,
            0x514, 0x4e0, 0x4ac, 0x478, 0x444, 0x410, 0x3dc, 0x3ac, 0x37c, 0x34c, 0x31c, 0x2e8,
            0x2b8, 0x288, 0x258, 0x228, 0x1f8, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0025_0b04,
    );
}

/// `ScreenTintMenu_dtor` (0x24dd70).
pub fn screen_tint_menu_dtor(c: &mut Cpu) {
    menu_page_dtor(
        c,
        &[
            0x928, 0x8f4, 0x8c0, 0x88c, 0x858, 0x824, 0x7f0, 0x7c0, 0x78c, 0x758, 0x724, 0x6f0,
            0x6bc, 0x688, 0x654, 0x620, 0x5ec, 0x5b8, 0x584, 0x550, 0x520, 0x4f0, 0x4c0, 0x490,
            0x45c, 0x42c, 0x3fc, 0x3c8, 0x394, 0x360, 0x32c, 0x2f8, 0x2c4, 0x290, 0x25c, 0x228,
            0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
        ],
        0x0024_dd94,
    );
}

/// The `SnowfallTogglesMenu_dtor` steps: offset from `this`, and whether the
/// field is an embedded page rather than an item.
const SNOWFALL_STEPS: [(u32, bool); 26] = [
    (0xfec, false),
    (0xfd0, false),
    (0xfac, false),
    (0xf88, false),
    (0xf64, false),
    (0xf40, false),
    (0x9a8, true),
    (0x990, false),
    (0x974, false),
    (0x950, false),
    (0x92c, false),
    (0x908, false),
    (0x8e4, false),
    (0x34c, true),
    (0x334, false),
    (0x300, false),
    (0x2cc, false),
    (0x298, false),
    (0x264, false),
    (0x230, false),
    (0x1fc, false),
    (0x1c8, false),
    (0x194, false),
    (0x164, false),
    (0x14c, false),
    (0x130, false),
];

/// `SnowfallTogglesMenu_dtor` (0x24e200): 24 item destructors interleaved with
/// two embedded pages at `+0x9a8` and `+0x34c`, then the page itself.
pub fn snowfall_toggles_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S3][0] = abi::arg64(c, 1);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);

        let this = r32(c, S0);
        c.g[S1][0] = sx(this.wrapping_add(0x990));
        c.g[S2][0] = sx(this.wrapping_add(0x334));

        for (i, &(off, page)) in SNOWFALL_STEPS.iter().enumerate() {
            let ret = if i < 3 {
                0x0024_e22c + 0x10 * i as u32
            } else {
                0x0024_e258 + 0xc * (i as u32 - 3)
            };
            c.g[A0][0] = sx(this.wrapping_add(off));
            c.g[A1][0] = 2;
            let target = if page {
                MENU_SYS_PAGE_DTOR
            } else {
                MENU_SYS_ITEM_DTOR
            };
            call_ptr(c, target, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, MENU_SYS_PAGE_DTOR, 0x0024_e36c)?;

        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, ptr, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP, SP};
    use ssx3_ee as ee;

    /// The allocator's mutex (`MEM_MUTEX`) and the fake result that keeps the
    /// delete path short.
    const MEM_MUTEX: u32 = 0x0053_8b00;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A valid, empty input map at `map` (`InputVM` stops on its first op).
    fn input_map(c: &mut Cpu, map: u32) {
        const INPUT_VM_TABLE: u32 = 0x0048_e330;
        const INPUT_VM_STOP: u32 = 0x0032_5ef8;
        const STOP_OPCODE: u32 = 0x20;
        w(c, map + 0x4, map + 0x80);
        w(c, map + 0x14, map + 0x100);
        for i in 0..4u32 {
            w(c, map + 0x80 + 4 * i, i);
            w(c, map + 0x100 + 4 * i, STOP_OPCODE);
        }
        w(c, INPUT_VM_TABLE + 4 * STOP_OPCODE, INPUT_VM_STOP);
    }

    /// An `InputBinding` at `binding` whose map is `map`.
    fn binding(c: &mut Cpu, binding: u32, map: u32, value: u32) {
        w(c, binding, value);
        w(c, binding + 4, map);
        input_map(c, map);
    }

    fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        w(c, MEM_MUTEX + 4, 0x42);
        w(c, LOG_RESULTS, 0x42);
        w(c, MEM_MUTEX + 8, r.next_u32() % 4);
    }

    /// The behaviour-library singleton (`gp-0x1f04`) pointed at a zeroed
    /// dummy, so `BELibrary_GetInstance` neither constructs one nor reads the
    /// disc.
    fn belib(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x8000);
    }

    const RET: u32 = 0x0012_3450;

    /// `int StrHash(const char *s)` run during setup (to build hash-keyed
    /// screens).
    fn str_hash(c: &mut Cpu, s: u32) -> u64 {
        c.g[A0][0] = sx(s);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    /// A one-object screen at `screen`: `UIScreen_FindObject` returns the
    /// object for `hash`, whose `+8` vtable sends slot `+0x4c` to the fake
    /// callee.
    fn screen_fixture(c: &mut Cpu, screen: u32, hash: u64) {
        let array = screen + 0x100;
        let obj = screen + 0x200;
        let vt = screen + 0x300;
        w(c, screen + 0x3c, 1);
        w(c, screen + 0x38, array);
        w(c, array, obj);
        w(c, obj + 0x38, hash as u32);
        w(c, obj + 8, vt);
        c.bus.write16(vt + 0x48, 0);
        w(c, vt + 0x4c, FAKE);
    }

    /// The shared fixture of the two `_v37` state switches: `this` at DATA,
    /// the `+0x234`/`+0x22c` fields and an optional widget screen.
    fn state_switch_setup(c: &mut Cpu, seed: u32, salt: u64, field: u32, calls: u32) {
        let _ = machine(c, seed, salt, calls);
        belib(c);
        let screen = DATA + 0x40_000;
        w(
            c,
            DATA + 0x234,
            if seed % 2 == 0 {
                seed % 2
            } else {
                (seed + 1) % 2
            },
        );
        w(c, DATA + field, if seed % 3 == 0 { 0 } else { 1 });
        let found = seed % 4 >= 2;
        if found {
            let hash = str_hash(c, 0x004a_15e0);
            screen_fixture(c, screen, hash);
            w(c, DATA + 0x40, screen);
        }
        // (Registers last: building the fixture calls guest code.)
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (seed % 2) as u64;
    }

    #[test]
    fn c_fe_state_options_device_select_v37_matches() {
        check(
            ee::s02::f_0018e178__generated,
            c_fe_state_options_device_select_v37,
            40,
            &|c, seed| state_switch_setup(c, seed, 0x18_e178, 0x22c, 8),
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
    fn ov_screen_top_times_on_activate_matches() {
        check(
            ee::s04::f_001f7f08__generated,
            ov_screen_top_times_on_activate,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f_7f08, 12);
                belib(c);
                c.bus.write8(LABEL_MODE, if seed % 2 == 0 { 0 } else { 2 });
                // No screen: PlayObjectAnim takes its not-found path.
                w(c, DATA + 0x40, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn rider_init_data_matches() {
        check(ee::s01::f_00140738__generated, rider_init_data, 60, &|c, seed| {
            let _ = machine(c, seed, 0x14_0738, 0);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn player_get_trick_button_matches() {
        check(
            ee::s00::f_001276f0__generated,
            player_get_trick_button,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x12_76f0, 15);
                c.g[A0][0] = DATA as u64;
                let b = DATA + 0x2000;
                binding(c, b, DATA + 0x3000, 0);
                w(c, DATA + 0xdf0, b);
                // The stopped input VM leaves the tested value in f0.
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                w(c, SP - 0x1f8, value.to_bits());
            },
        );
    }

    #[test]
    fn fe_ui_input_is_any_button_matches() {
        check(
            ee::s02::f_00180300__generated,
            fe_ui_input_is_any_button,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x18_0300, 16);
                c.g[A0][0] = DATA as u64;
                let b = DATA + 0x2000;
                binding(c, b, DATA + 0x3000, 0);
                w(c, DATA + 0x18, b);
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                w(c, SP - 0x208, value.to_bits());
            },
        );
    }

    #[test]
    fn fe_module_rider_preview_find_bones_matches() {
        check(
            ee::s03::f_0019e680__generated,
            fe_module_rider_preview_find_bones,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x19_e680, 14);
                c.g[A0][0] = DATA as u64;
                // A model with no bones: every lookup takes the not-found path.
                w(c, DATA + 8, DATA + 0x2000);
            },
        );
    }

    #[test]
    fn bolts_free_db_matches() {
        check(ee::s01::f_0014d240__generated, bolts_free_db, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x14_d240, 11);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            // Cover both the free and the already-free path.
            w(c, DATA, if seed % 2 == 0 { 0 } else { ptr(&mut r) });
            w(c, DATA + 0x18, if seed % 3 == 0 { 0 } else { ptr(&mut r) });
        });
    }

        /// The deleting-destructor fixture: a random page pointer, both values of
    /// the delete flag and a short `operator delete` path.
    fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 2);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn visual_effects_test_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024d998__generated,
            visual_effects_test_menu_dtor,
            0x24_d998,
        );
    }

    #[test]
    fn visual_effects_main_menu_dtor_matches() {
        dtor_case(
            ee::s06::f_0024f6c0__generated,
            visual_effects_main_menu_dtor,
            0x24_f6c0,
        );
    }

        #[test]
    fn snowfall_toggles_menu_dtor_matches() {
        dtor_case(ee::s06::f_0024e200__generated, snowfall_toggles_menu_dtor, 0x24_e200);
    }

                }
