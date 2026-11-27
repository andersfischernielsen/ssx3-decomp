//! `src/fe/all_in_one_fe.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::game::fe::fepopup::oc508_vtable_dtor;
pub(crate) use crate::game::fe::ovtemplatedialog::oc998_ov_state_dtor_body;
pub(crate) use crate::game::fe::ovtemplatepausemenu::ov_state_fill_fields;
pub(crate) use crate::game::fe::ovtemplatepausemenu::OV_STATE_UI_CTOR;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0021_ccc8, ui_icon_flags_ctor, "UIIconFlags_ctor"),
    (0x0021_c848, ov_screen_id0e_dtor, "OVScreenId0E_dtor"),
    (
        0x0021_c900,
        ov_screen_yn_dialog_restart_event_dtor,
        "OVScreenYNDialogRestartEvent_dtor",
    ),
    (
        0x0021_c9c0,
        ov_screen_yn_dialog_restart_chal_dtor,
        "OVScreenYNDialogRestartChal_dtor",
    ),
    (
        0x0021_c9e8,
        ov_screen_yn_dialog_quit_chal_dtor,
        "OVScreenYNDialogQuitChal_dtor",
    ),
    (
        0x0021_ca10,
        ov_screen_yn_dialog_quit_event_dtor,
        "OVScreenYNDialogQuitEvent_dtor",
    ),
    (
        0x0021_ca38,
        ov_screen_yn_dialog_give_up_dtor,
        "OVScreenYNDialogGiveUp_dtor",
    ),
    (
        0x0021_ca60,
        ov_screen_yn_dialog_save_before_quit_dtor,
        "OVScreenYNDialogSaveBeforeQuit_dtor",
    ),
    (
        0x0021_cb70,
        ov_state_static_init_record,
        "OVStateStatic_InitRecord",
    ),
    (
        0x0022_15f0,
        ov_screen_pause_mode1_ctor,
        "OVScreenPauseMode1_ctor",
    ),
    (
        0x0022_1658,
        ov_screen_pause_mode1_ctor_2,
        "OVScreenPauseMode1_ctor_2",
    ),
    (
        0x0022_16c8,
        ov_screen_pause_mode1_dtor,
        "OVScreenPauseMode1_dtor",
    ),
    (
        0x0022_16f0,
        ov_screen_pause_mode2_ctor,
        "OVScreenPauseMode2_ctor",
    ),
    (
        0x0022_1758,
        ov_screen_pause_mode2_ctor_2,
        "OVScreenPauseMode2_ctor_2",
    ),
    (
        0x0022_17c8,
        ov_screen_pause_mode2_dtor,
        "OVScreenPauseMode2_dtor",
    ),
    (
        0x0022_17f0,
        ov_screen_pause_mode3_ctor,
        "OVScreenPauseMode3_ctor",
    ),
    (
        0x0022_1858,
        ov_screen_pause_mode3_ctor_2,
        "OVScreenPauseMode3_ctor_2",
    ),
    (
        0x0022_18c8,
        ov_screen_pause_mode3_dtor,
        "OVScreenPauseMode3_dtor",
    ),
    (
        0x0022_18f0,
        ov_screen_pause_mode4_ctor,
        "OVScreenPauseMode4_ctor",
    ),
    (
        0x0022_1958,
        ov_screen_pause_mode4_ctor_2,
        "OVScreenPauseMode4_ctor_2",
    ),
    (
        0x0022_19c8,
        ov_screen_pause_mode4_dtor,
        "OVScreenPauseMode4_dtor",
    ),
    (
        0x0022_19f0,
        ov_screen_pause_mode5_ctor,
        "OVScreenPauseMode5_ctor",
    ),
    (
        0x0022_1a58,
        ov_screen_pause_mode5_ctor_2,
        "OVScreenPauseMode5_ctor_2",
    ),
    (
        0x0022_1ac8,
        ov_screen_pause_mode5_dtor,
        "OVScreenPauseMode5_dtor",
    ),
    (
        0x0022_1af0,
        ov_screen_pause_mode6_ctor,
        "OVScreenPauseMode6_ctor",
    ),
    (
        0x0022_1b58,
        ov_screen_pause_mode6_ctor_2,
        "OVScreenPauseMode6_ctor_2",
    ),
    (
        0x0022_1bc8,
        ov_screen_pause_mode6_dtor,
        "OVScreenPauseMode6_dtor",
    ),
];

/// `UIIconFlags::ctor` (0x21ccc8): keeps the high nibble of the flags byte
/// at `*a0` and clears the low one. Returns the flags pointer in v0.
pub fn ui_icon_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let flags = c.g[A0][0];
    let masked = Addr(flags as u32).lbu(c) & 0xf0;
    c.g[V1][0] = masked;
    c.g[V0][0] = flags;
    c.g[V1][0] = masked;
    Addr(flags as u32).sb(c, masked);
    ret(c);
}

/// Destructor of the id-0x0e overlay (0x21c848): the shared `cOVState`
/// destructor body.
pub fn ov_screen_id0e_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c864));
}

/// Vtable of the overlay state base class (`cOVState_vtbl`, 0x474e08).
pub(crate) const OC999_OV_STATE_VTBL: u32 = 0x0047_4e08;

/// Destructor of an overlay YN dialog (`cOVState` subclass): installs
/// `cOVState_vtbl` at +8 and runs the base `UIState` destructor (0x39e390).
/// The generated body, through the shared vtable-dtor helper.
pub(crate) fn oc999_yn_dtor(c: &mut Cpu, ret_addr: u32) -> crate::call::Flow {
    oc508_vtable_dtor(c, sx(OC999_OV_STATE_VTBL), 0x0039_e390, ret_addr)
}

/// `OVScreenYNDialogRestartEvent_dtor` (0x21c900).
pub fn ov_screen_yn_dialog_restart_event_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_c91c));
}

/// `OVScreenYNDialogRestartChal_dtor` (0x21c9c0).
pub fn ov_screen_yn_dialog_restart_chal_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_c9dc));
}

/// `OVScreenYNDialogQuitChal_dtor` (0x21c9e8).
pub fn ov_screen_yn_dialog_quit_chal_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca04));
}

/// `OVScreenYNDialogQuitEvent_dtor` (0x21ca10).
pub fn ov_screen_yn_dialog_quit_event_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca2c));
}

/// `OVScreenYNDialogGiveUp_dtor` (0x21ca38).
pub fn ov_screen_yn_dialog_give_up_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca54));
}

/// `OVScreenYNDialogSaveBeforeQuit_dtor` (0x21ca60).
pub fn ov_screen_yn_dialog_save_before_quit_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca7c));
}

/// `void OVStateStatic_InitRecord(OVStateStatic *rec)` (0x21cb70): a leaf that
/// clears the record: word 0 = 1 (v0 keeps the 1), words +4, +8, +0xc, +0x10,
/// +0x14 = 0.
pub fn ov_state_static_init_record(c: &mut Cpu) {
    c.cycles += 8;
    let rec = c.g[A0][0] as u32;
    c.g[V0][0] = 0x1;
    c.sw(rec.wrapping_add(0x14), 0);
    c.sw(rec, c.g[V0][0]);
    c.sw(rec.wrapping_add(0x4), 0);
    c.sw(rec.wrapping_add(0x8), 0);
    c.sw(rec.wrapping_add(0xc), 0);
    c.sw(rec.wrapping_add(0x10), 0);
    ret(c);
}

/// `OVScreenPauseMode1_ctor` (0x2215f0): the pause mode 1 overlay state
/// (competition pause vtable, pause thread cleared, overlay id 0).
pub fn ov_screen_pause_mode1_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1608)?;
        ov_state_fill_fields(c, 0, 0x0047_3098, true);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `UIState_ctor(this, flags)` (0x39e2a0), the base constructor of the pause
/// overlay states.
pub(crate) const OV_PAUSE_UI_STATE_CTOR: u32 = 0x0039_e2a0;

/// `UIState_dtor(this, flags)` (0x39e390), the base destructor.
pub(crate) const OV_PAUSE_UI_STATE_DTOR: u32 = 0x0039_e390;

/// The `cOVState` base vtable (`0x470000 + 0x4e08`): written at `+8` first by
/// each pause constructor, and by both destructors.
pub(crate) const OV_PAUSE_BASE_VTABLE: u32 = 0x0047_4e08;

/// The `cOVState_PAUSE_COMPETE` vtable (`0x470000 + 0x3098`), Mode1's final one.
pub(crate) const OV_PAUSE_COMPETE_VTABLE: u32 = 0x0047_3098;

/// The `cOVState_PAUSE_BIGCHAL` vtable (`0x470000 + 0x2fc0`), Mode2's final one.
pub(crate) const OV_PAUSE_BIGCHAL_VTABLE: u32 = 0x0047_2fc0;

/// The data page the constructors read from (`0x4a0000`): the chat-buffer
/// initial value `str_sp_b[0]` is the halfword at `+0x2088`.
pub(crate) const OV_PAUSE_DATA_PAGE: u32 = 0x004a_0000;

/// The fields both pause constructors of the `_ctor_2` shape set after
/// `UIState_ctor(this, engine)`: the overlay id (`s1`) at `+0x98`, zeroed
/// words at `+0x48`, `+0x4c`, `+0x54`, the base vtable at `+8` and then
/// `final_vtable` at `+8`, the chat halfword at `+0x58`, and zero at `+0xc4`.
/// `ret_ctor` is the return address of the base constructor call.
pub(crate) fn ov_pause_ctor_2(c: &mut Cpu, ret_ctor: u32, final_vtable: u32) -> Flow {
    c.cycles += 27;
    let f = Frame::open(c, 0x30);
    f.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A0][0];
    f.save(c, S1, 0x10);
    f.save_ra(c, 0);
    c.g[S1][0] = c.g[A2][0];
    call_ptr(c, OV_PAUSE_UI_STATE_CTOR, ret_ctor)?;

    let this = c.g[S0][0] as u32;
    c.g[V1][0] = 0x0047_0000;
    c.sw(this.wrapping_add(0x98), c.g[S1][0]);
    c.g[V1][0] = sx(OV_PAUSE_BASE_VTABLE);
    c.sw(this.wrapping_add(0x48), 0);
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.g[A1][0] = sx(OV_PAUSE_DATA_PAGE);
    c.sw(this.wrapping_add(0x4c), 0);
    c.g[V1][0] = 0x0047_0000;
    c.sw(this.wrapping_add(0x54), 0);
    c.g[V1][0] = sx(final_vtable);
    c.g[V0][0] = c.g[S0][0];
    f.restore(c, S1, 0x10);
    c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.sh(this.wrapping_add(0x58), c.g[A0][0]);
    c.sw(this.wrapping_add(0xc4), 0);
    f.restore(c, S0, 0x20);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenPauseMode1_ctor_2(this, engine, overlayId)` (0x221658): the
/// COMPETE pause overlay's constructor (the overlay id is stored at `+0x98`).
/// Registers and cycles follow the generated body.
pub fn ov_screen_pause_mode1_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ov_pause_ctor_2(c, 0x0022_1674, OV_PAUSE_COMPETE_VTABLE)
    });
}

/// `OVScreenPauseMode1_dtor(this, flags)` (0x2216c8): the base vtable at
/// `+8`, then `UIState_dtor(this, flags)`.
pub fn ov_screen_pause_mode1_dtor(c: &mut Cpu) {
    guest(c, |c| ov_pause_dtor(c, 0x0022_16e4));
}

/// The shared body of the two pause destructors: a 0x10 frame holding `ra`,
/// the base vtable at `this + 8`, then the base `UIState` destructor (the
/// argument registers pass through unchanged).
pub(crate) fn ov_pause_dtor(c: &mut Cpu, ret_dtor: u32) -> Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(OV_PAUSE_BASE_VTABLE);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, OV_PAUSE_UI_STATE_DTOR, ret_dtor)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenPauseMode2_ctor(this)` (0x2216f0): `UIState_ctor(this, 0)`, then
/// the BIGCHAL overlay's fields. Unlike the `_ctor_2` shape it passes flags 0
/// and stores zero (not the overlay id) at `+0x98`.
pub fn ov_screen_pause_mode2_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_PAUSE_UI_STATE_CTOR, 0x0022_1708)?;

        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), 0);
        c.g[V1][0] = sx(OV_PAUSE_BASE_VTABLE);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = sx(OV_PAUSE_DATA_PAGE);
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_BIGCHAL_VTABLE);
        c.g[V0][0] = c.g[S0][0];
        f.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xc4), 0);
        f.restore(c, S0, 0x10);
        f.close(c);
        Ok(())
    });
}

/// `OVScreenPauseMode2_ctor_2(this, engine, overlayId)` (0x221758): the
/// BIGCHAL pause overlay's constructor, the same shape as Mode1's `_ctor_2`
/// with the BIGCHAL vtable.
pub fn ov_screen_pause_mode2_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ov_pause_ctor_2(c, 0x0022_1774, OV_PAUSE_BIGCHAL_VTABLE)
    });
}

/// `OVScreenPauseMode2_dtor(this, flags)` (0x2217c8): the same body as
/// Mode1's destructor, returning from its base call to `0x2217e4`.
pub fn ov_screen_pause_mode2_dtor(c: &mut Cpu) {
    guest(c, |c| ov_pause_dtor(c, 0x0022_17e4));
}

/// The game's global block (`0x470000`): the pause overlay vtables sit at fixed
/// offsets from it.
pub(crate) const OVPAUSE_GLOBALS: u32 = 0x0047_0000;

/// The base `cOVState` vtable (`OVPAUSE_GLOBALS + 0x4e08`), stored at `+8` first.
pub(crate) const OVPAUSE_VTABLE_BASE: u32 = OVPAUSE_GLOBALS + 0x4e08;

/// The config block (`0x4a0000`): its halfword at `+0x2088` goes to `+0x58`.
pub(crate) const OVPAUSE_CONFIG_BLOCK: u32 = 0x004a_0000;

/// The `cUIState` constructor (0x39e2a0) and destructor (0x39e390), called
/// through the function table.
pub(crate) const OVPAUSE_UISTATE_CTOR: u32 = 0x0039_e2a0;

pub(crate) const OVPAUSE_UISTATE_DTOR: u32 = 0x0039_e390;

/// The destructor body of the pause overlays (`this` in `a0`): stores the base
/// `cOVState` vtable at `+8` of `this`, runs the `cUIState` destructor
/// returning to `ret_addr`, and returns to the caller.
pub(crate) fn ov_pause_dtor_body(c: &mut Cpu, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = u64::from(OVPAUSE_VTABLE_BASE);
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, vtable);
        call_ptr(c, OVPAUSE_UISTATE_DTOR, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The constructor body of the plain pause overlays (`this` in `a0`, no overlay
/// id): runs the `cUIState` constructor returning to `base_ret` with `a1` = 0,
/// zeroes the overlay id at `+0x98`, `+0x48`, `+0x4c`, `+0x54` and `+0xc4`,
/// stores the base vtable at `+8` and then the pause vtable
/// (`OVPAUSE_GLOBALS + final_offset`) at `+8`, and the config halfword at `+0x58`.
pub(crate) fn ov_pause_ctor_plain_body(c: &mut Cpu, base_ret: u32, final_offset: u32) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OVPAUSE_UISTATE_CTOR, base_ret)?;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x98).sw(c, 0);
        Addr(this).off(0x48).sw(c, 0);
        Addr(this).off(8).sw(c, u64::from(OVPAUSE_VTABLE_BASE));
        c.g[A1][0] = u64::from(OVPAUSE_CONFIG_BLOCK);
        Addr(this).off(0x4c).sw(c, 0);
        Addr(this).off(0x54).sw(c, 0);
        c.g[V1][0] = u64::from(OVPAUSE_GLOBALS + final_offset);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        let config = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = config;
        let final_vtable = c.g[V1][0];
        Addr(this).off(8).sw(c, final_vtable);
        Addr(this).off(0x58).sh(c, config);
        Addr(this).off(0xc4).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The constructor body of the pause overlays that take an overlay id (`this`
/// in `a0`, overlay id in `a2`): as the plain constructor, but the id is stored
/// at `+0x98` and kept in `s1` across the `cUIState` constructor.
pub(crate) fn ov_pause_ctor_overlay_body(c: &mut Cpu, base_ret: u32, final_offset: u32) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OVPAUSE_UISTATE_CTOR, base_ret)?;
        let this = c.g[S0][0] as u32;
        let overlay = c.g[S1][0];
        Addr(this).off(0x98).sw(c, overlay);
        Addr(this).off(0x48).sw(c, 0);
        Addr(this).off(8).sw(c, u64::from(OVPAUSE_VTABLE_BASE));
        c.g[A1][0] = u64::from(OVPAUSE_CONFIG_BLOCK);
        Addr(this).off(0x4c).sw(c, 0);
        Addr(this).off(0x54).sw(c, 0);
        c.g[V1][0] = u64::from(OVPAUSE_GLOBALS + final_offset);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let config = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = config;
        let final_vtable = c.g[V1][0];
        Addr(this).off(8).sw(c, final_vtable);
        Addr(this).off(0x58).sh(c, config);
        Addr(this).off(0xc4).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenPauseMode3::~OVScreenPauseMode3` (0x2218c8).
pub fn ov_screen_pause_mode3_dtor(c: &mut Cpu) {
    ov_pause_dtor_body(c, 0x0022_18e4);
}

/// `OVScreenPauseMode3::OVScreenPauseMode3` (0x2217f0): the plain constructor
/// with the `PAUSE_FREERIDE` vtable.
pub fn ov_screen_pause_mode3_ctor(c: &mut Cpu) {
    ov_pause_ctor_plain_body(c, 0x0022_1808, 0x2ee8);
}

/// `OVScreenPauseMode3` constructor taking the overlay id (0x221858).
pub fn ov_screen_pause_mode3_ctor_2(c: &mut Cpu) {
    ov_pause_ctor_overlay_body(c, 0x0022_1874, 0x2ee8);
}

/// `OVScreenPauseMode4::~OVScreenPauseMode4` (0x2219c8).
pub fn ov_screen_pause_mode4_dtor(c: &mut Cpu) {
    ov_pause_dtor_body(c, 0x0022_19e4);
}

/// `OVScreenPauseMode4::OVScreenPauseMode4` (0x2218f0): the plain constructor
/// with the `PAUSE_SPOKE` vtable.
pub fn ov_screen_pause_mode4_ctor(c: &mut Cpu) {
    ov_pause_ctor_plain_body(c, 0x0022_1908, 0x2e10);
}

/// `OVScreenPauseMode4` constructor taking the overlay id (0x221958).
pub fn ov_screen_pause_mode4_ctor_2(c: &mut Cpu) {
    ov_pause_ctor_overlay_body(c, 0x0022_1974, 0x2e10);
}

/// `OVScreenPauseMode5::OVScreenPauseMode5` (0x2219f0): the plain constructor
/// with the `PAUSE_QUICKPLAY` vtable.
pub fn ov_screen_pause_mode5_ctor(c: &mut Cpu) {
    ov_pause_ctor_plain_body(c, 0x0022_1a08, 0x2d38);
}

/// `OVScreenPauseMode5` constructor taking the overlay id (0x221a58).
pub fn ov_screen_pause_mode5_ctor_2(c: &mut Cpu) {
    ov_pause_ctor_overlay_body(c, 0x0022_1a74, 0x2d38);
}

// The `cOVState` screens below derive from the UI state base: its constructor
// at 0x39e2a0 and destructor at 0x39e390, both reached through the function
// table (`call_ptr`). Their vtables sit in the game image at 0x47xxxx (the
// `+8` word of each object). Offsets of the state fields: `+0x48` done
// transition, `+0x4c` allow pause menu, `+0x54` remove PDA, `+0x58` chat
// buffer (u16), `+0x98` overlay id, `+0x9c` ping start tick, `+0xa0` display
// aux info flag, `+0xc4` thread.

/// Base `cOVState` vtable (game image 0x474e08): the first vptr of every
/// pause, options and in-game error state built here.
pub(crate) const OV_STATE_VTBL_BASE: u32 = 0x0047_4e08;

/// The `cOVState_PAUSE_*_vtbl` each constructor switches to.
pub(crate) const OV_PAUSE_VTBL_ONLINE: u32 = 0x0047_2c60;

/// The high half loaded into the constructors' argument register before the
/// `str_sp_b[0]` read (`lhu` at +0x2088): the chat buffer's initial word.
pub(crate) const OV_STR_SP_B_HI: u32 = 0x004a_0000;

/// `void OVScreenPauseMode5_dtor(OVScreenPauseMode5 *self)` (0x221ac8): sets
/// the base vtable at `+8` and runs the UIState destructor.
pub fn ov_screen_pause_mode5_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OV_STATE_VTBL_BASE);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPauseMode6 *OVScreenPauseMode6_ctor(OVScreenPauseMode6 *self)`
/// (0x221af0): runs the UIState constructor (with a null second argument),
/// clears the state fields, sets the online-pause vtable and the chat buffer's
/// initial word. Returns `self` in `v0`.
pub fn ov_screen_pause_mode6_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1b08)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), 0);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xc4), 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPauseMode6 *OVScreenPauseMode6_ctor_2(OVScreenPauseMode6 *self,
/// int a1, int overlayId)` (0x221b58): as `OVScreenPauseMode6_ctor`, but the
/// UIState constructor gets the caller's `a1` and the overlay id is stored at
/// `+0x98`. Returns `self` in `v0`.
pub fn ov_screen_pause_mode6_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1b74)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xc4), 0);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPauseMode6_dtor(OVScreenPauseMode6 *self)` (0x221bc8): sets
/// the base vtable at `+8` and runs the UIState destructor.
pub fn ov_screen_pause_mode6_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OV_STATE_VTBL_BASE);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1be4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn ui_icon_flags_ctor_matches() {
        check(
            ee::s05::f_0021ccc8__generated,
            ui_icon_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0021_ccc8 ^ seed as u64);
                scramble(c, &mut r);
                let p = DATA + 0x100 + 0x10 * (seed % 16);
                c.g[A0][0] = sx(p);
                c.bus.write8(p, r.next_u32() as u8);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::fill;
    pub(crate) use crate::game::fe::fepopupmisc::tests::SEEDS_12 as SEEDS;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc998_hook;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc998_setup;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::ov_object_setup;

    use std::sync::atomic::AtomicUsize;

    #[test]
    fn ov_screen_id0e_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c848__generated,
            ov_screen_id0e_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c848, 2),
        );
    }

    /// Fixture for the overlay dialogs: a machine with the object at DATA+0x2000.
    pub(crate) fn oc999_yn_setup(c: &mut Cpu, seed: u32, entry: u32) {
        machine(c, seed, entry as u64, 2);
        c.g[A0][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = (seed % 5) as u64;
        c.pc = entry;
    }

    #[test]
    fn ov_screen_yn_dialog_restart_event_dtor_matches() {
        check(
            ee::s05::f_0021c900__generated,
            ov_screen_yn_dialog_restart_event_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c900),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_restart_chal_dtor_matches() {
        check(
            ee::s05::f_0021c9c0__generated,
            ov_screen_yn_dialog_restart_chal_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c9c0),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_chal_dtor_matches() {
        check(
            ee::s05::f_0021c9e8__generated,
            ov_screen_yn_dialog_quit_chal_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c9e8),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_event_dtor_matches() {
        check(
            ee::s05::f_0021ca10__generated,
            ov_screen_yn_dialog_quit_event_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_ca10),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_give_up_dtor_matches() {
        check(
            ee::s05::f_0021ca38__generated,
            ov_screen_yn_dialog_give_up_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_ca38),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_save_before_quit_dtor_matches() {
        check(
            ee::s05::f_0021ca60__generated,
            ov_screen_yn_dialog_save_before_quit_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_ca60),
        );
    }

    #[test]
    fn ov_state_static_init_record_matches() {
        check(
            ee::s05::f_0021cb70__generated,
            ov_state_static_init_record,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x21cb_70 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x100, 0x10);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 3) * 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode1_ctor_matches() {
        check(
            ee::s05::f_002215f0__generated,
            ov_screen_pause_mode1_ctor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    /// The overlay object the constructors and destructors work on.
    pub(crate) const OV_THIS: u32 = DATA + 0x2000;

    /// A machine with the runtime and allocator, `this` at [`OV_THIS`], and a
    /// random chat halfword in the data page (the constructors copy it).
    pub(crate) fn ov_pause_machine(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32) {
        scramble(c, r);
        machine_rt(c, seed, entry as u64, 0);
        c.bus
            .write16(OV_PAUSE_DATA_PAGE + 0x2088, r.next_u32() as u16);
        c.g[A0][0] = sx(OV_THIS);
        c.pc = entry;
    }

    #[test]
    fn ov_screen_pause_mode1_ctor_2_matches() {
        check(
            ee::s05::f_00221658__generated,
            ov_screen_pause_mode1_ctor_2,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_1658 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_1658);
                c.g[A1][0] = sx(seed % 2);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode2_ctor_2_matches() {
        check(
            ee::s05::f_00221758__generated,
            ov_screen_pause_mode2_ctor_2,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_1758 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_1758);
                c.g[A1][0] = sx(seed % 2);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode2_ctor_matches() {
        check(
            ee::s05::f_002216f0__generated,
            ov_screen_pause_mode2_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_16f0 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_16f0);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode1_dtor_matches() {
        check(
            ee::s05::f_002216c8__generated,
            ov_screen_pause_mode1_dtor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_16c8 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_16c8);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode2_dtor_matches() {
        check(
            ee::s05::f_002217c8__generated,
            ov_screen_pause_mode2_dtor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0022_17c8 ^ (seed as u64) << 8);
                ov_pause_machine(c, &mut r, seed, 0x0022_17c8);
                c.g[A1][0] = 0;
            },
        );
    }

    /// A zeroed overlay object at `DATA+0x1000` (the base destructor's lists
    /// take their short path), `a1` = 0 so no `operator_delete` runs.
    pub(crate) fn pause_dtor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        machine_rt(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = 0;
    }

    /// A zeroed overlay object, the engine pointer in `a1` (stored, not read),
    /// a random overlay id in `a2`, and the config halfword the constructor
    /// copies into `+0x58`.
    pub(crate) fn pause_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine_rt(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x3000);
        c.g[A2][0] = sx(r.next_u32());
        c.bus.write16(
            OVPAUSE_CONFIG_BLOCK + 0x2088,
            (r.next_u32() & 0xffff) as u16,
        );
    }

    #[test]
    fn ov_screen_pause_mode3_dtor_matches() {
        check(
            ee::s05::f_002218c8__generated,
            ov_screen_pause_mode3_dtor,
            200,
            &|c, seed| pause_dtor_fixture(c, seed, 0x0022_18c8),
        );
    }

    #[test]
    fn ov_screen_pause_mode3_ctor_matches() {
        check(
            ee::s05::f_002217f0__generated,
            ov_screen_pause_mode3_ctor,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_17f0),
        );
    }

    #[test]
    fn ov_screen_pause_mode3_ctor_2_matches() {
        check(
            ee::s05::f_00221858__generated,
            ov_screen_pause_mode3_ctor_2,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_1858),
        );
    }

    #[test]
    fn ov_screen_pause_mode4_dtor_matches() {
        check(
            ee::s05::f_002219c8__generated,
            ov_screen_pause_mode4_dtor,
            200,
            &|c, seed| pause_dtor_fixture(c, seed, 0x0022_19c8),
        );
    }

    #[test]
    fn ov_screen_pause_mode4_ctor_matches() {
        check(
            ee::s05::f_002218f0__generated,
            ov_screen_pause_mode4_ctor,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_18f0),
        );
    }

    #[test]
    fn ov_screen_pause_mode4_ctor_2_matches() {
        check(
            ee::s05::f_00221958__generated,
            ov_screen_pause_mode4_ctor_2,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_1958),
        );
    }

    #[test]
    fn ov_screen_pause_mode5_ctor_matches() {
        check(
            ee::s05::f_002219f0__generated,
            ov_screen_pause_mode5_ctor,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_19f0),
        );
    }

    #[test]
    fn ov_screen_pause_mode5_ctor_2_matches() {
        check(
            ee::s05::f_00221a58__generated,
            ov_screen_pause_mode5_ctor_2,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_1a58),
        );
    }

    /// Gate word of the oc1012 fakes (a fixture address of this machine only).
    pub(crate) const GATE: u32 = DATA + 0x0020_1200;

    pub(crate) const MAGIC: u32 = 0x0c10_1200;

    pub(crate) static OC1012_PREV_39E2A0: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1012_PREV_39E390: AtomicUsize = AtomicUsize::new(0);

    /// The gated fake for the UIState base: logs the call like `fake_call`
    /// while the gate word is set, else runs the previous body.
    pub(crate) fn oc1012_gated(
        c: &mut Cpu,
        target: u32,
        prev: &AtomicUsize,
        generated: fn(&mut Cpu),
    ) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    pub(crate) fn oc1012_fake_39e2a0(c: &mut Cpu) {
        oc1012_gated(
            c,
            0x0039_e2a0,
            &OC1012_PREV_39E2A0,
            ee::s13::f_0039e2a0__generated,
        )
    }

    pub(crate) fn oc1012_fake_39e390(c: &mut Cpu) {
        oc1012_gated(
            c,
            0x0039_e390,
            &OC1012_PREV_39E390,
            ee::s13::f_0039e390__generated,
        )
    }

    pub(crate) fn oc1012_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1012_fake_39e2a0,
            &OC1012_PREV_39E2A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1012_fake_39e390,
            &OC1012_PREV_39E390,
        );
    }

    /// Scramble the machine (the `calls` fake unwinds at a random call) and
    /// arm the UIState fakes for this machine.
    pub(crate) fn oc1012_object(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        machine(c, seed, salt, calls);
        c.bus.write32(GATE, MAGIC);
    }

    #[test]
    fn ov_screen_pause_mode5_dtor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221ac8__generated,
            ov_screen_pause_mode5_dtor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221ac8, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode6_ctor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221af0__generated,
            ov_screen_pause_mode6_ctor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221af0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode6_ctor_2_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221b58__generated,
            ov_screen_pause_mode6_ctor_2,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221b58, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                // `a1` is passed through to the UIState constructor.
                c.g[A1][0] = sx(seed.wrapping_mul(0x9e37_79b9));
                c.g[A2][0] = sx(0x30 + seed % 9);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode6_dtor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221bc8__generated,
            ov_screen_pause_mode6_dtor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221bc8, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }
}
