//! `src/main/loadscreens_prestart.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::gp as gp_3;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_2;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::fe::festateloadscreen::render_screen;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::main::ssxappdtor::vtable_dtor;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp;
pub(crate) use crate::game::world::worldview::p_2 as p_3;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_2348,
        pre_start_screen_update,
        "PreStartScreen_Update",
    ),
    (
        0x0023_2488,
        pre_start_screen_render,
        "PreStartScreen_Render",
    ),
    (
        0x0023_2510,
        pre_start_screen_try_exit,
        "PreStartScreen_TryExit",
    ),
    (0x0023_2538, init_load_screen_dtor, "InitLoadScreen_dtor"),
    (
        0x0023_2588,
        init_load_screen_update,
        "InitLoadScreen_Update",
    ),
    (
        0x0023_2650,
        init_load_screen_render,
        "InitLoadScreen_Render",
    ),
    (
        0x0023_26d8,
        init_load_screen_try_exit,
        "InitLoadScreen_TryExit",
    ),
    (
        0x0023_2720,
        c_pre_game_load_screen_ctor,
        "cPreGameLoadScreen_ctor",
    ),
    (
        0x0023_2738,
        c_pre_fe_load_screen_ctor,
        "cPreFELoadScreen_ctor",
    ),
    (
        0x0023_2750,
        c_pre_game_load_screen_dtor,
        "cPreGameLoadScreen_dtor",
    ),
    (
        0x0023_2780,
        c_pre_fe_load_screen_dtor,
        "cPreFELoadScreen_dtor",
    ),
    (
        0x0023_27b0,
        cpre_game_load_screen_try_exit,
        "cPreGameLoadScreen_TryExit",
    ),
    (
        0x0023_27e8,
        cpre_fe_load_screen_try_exit,
        "cPreFELoadScreen_TryExit",
    ),
    (
        0x0023_2820,
        c_pre_game_load_screen_render,
        "cPreGameLoadScreen_Render",
    ),
    (
        0x0023_28a8,
        c_pre_fe_load_screen_render,
        "cPreFELoadScreen_Render",
    ),
    (
        0x0023_2930,
        c_pre_game_load_screen_update,
        "cPreGameLoadScreen_Update",
    ),
    (
        0x0023_2a08,
        c_pre_fe_load_screen_update,
        "cPreFELoadScreen_Update",
    ),
    (0x0023_2b28, start_screen_load, "StartScreen_Load"),
    (0x0023_2c98, start_screen_enter, "StartScreen_Enter"),
    (0x0023_2cf8, start_screen_try_exit, "StartScreen_TryExit"),
    (0x0023_2d18, start_screen_update, "StartScreen_Update"),
    (0x0023_2d50, start_screen_unload, "StartScreen_Unload"),
    (0x0023_2da8, start_screen_render, "StartScreen_Render"),
    (
        0x0023_3260,
        c_game_load_screen_enter,
        "cGameLoadScreen_Enter",
    ),
    (
        0x0023_32e8,
        c_game_load_screen_unload,
        "cGameLoadScreen_Unload",
    ),
    (
        0x0023_3390,
        c_game_load_screen_render,
        "cGameLoadScreen_Render",
    ),
    (
        0x0023_3408,
        c_game_load_screen_update,
        "cGameLoadScreen_Update",
    ),
    (0x0023_2ae0, start_screen_ctor, "StartScreen_ctor"),
    (0x0023_2e20, c_game_load_screen_load, "cGameLoadScreen_Load"),
    (
        0x0023_32c8,
        c_game_load_screen_try_exit,
        "cGameLoadScreen_TryExit",
    ),
    (
        0x0023_2568,
        init_load_screen_enter_load,
        "InitLoadScreen_Enter",
    ),
    (
        0x0023_3048,
        c_game_load_screen_load_pictures_3,
        "cGameLoadScreen_LoadPictures",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;

    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;

    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;

    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE;

    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;

    pub(crate) use crate::game::fe::fepopup::callee::C00231F80;

    pub(crate) use crate::game::main::ssxappdtor::callee::APP_MODULE_UPDATE;

    pub(crate) use crate::game::main::ssxappdtor::callee::UI_ENGINE_UPDATE;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
}

/// `cPreGameLoadScreen *cPreGameLoadScreen_ctor(cPreGameLoadScreen *self)`
/// (0x232720): vtable 0x47d8a8, zero `+4`, return `self`.
pub fn c_pre_game_load_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2758;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 6;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    abi::ret(c);
}

/// `cPreFELoadScreen *cPreFELoadScreen_ctor(cPreFELoadScreen *self)`
/// (0x232738): vtable 0x47d860, zero `+4`, return `self`.
pub fn c_pre_fe_load_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x27a0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 6;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    abi::ret(c);
}

/// `void InitLoadScreen_Enter(void)` (0x00232568): run the boot load for the
/// world (`gp - 0x848`).
pub fn init_load_screen_enter(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = Addr(gp(c, -0x848)).lw(c),
        callee::SSX_APP_BOOT_LOAD,
        0x0023_257c,
    )
}

// ---------------------------------------------------------------------------
// Load screens.

/// `void cGameLoadScreen_Update(self)` (0x233408): drive the game load
/// screen's UI engine (`self+0xc`) and then the app module's fade.
pub fn c_game_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(p(r32(c, S0), 0xc));
        call_ptr(c, callee::UI_ENGINE_UPDATE, 0x0023_3420)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::APP_MODULE_UPDATE, 0x0023_3428)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InitLoadScreen` destructor (vtable at `+0`).
pub fn init_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0023_255c);
}

/// `cPreGameLoadScreen` destructor (vtable at `+0`).
pub fn c_pre_game_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0023_2774);
}

/// `cPreFELoadScreen` destructor (vtable at `+0`).
pub fn c_pre_fe_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0023_27a4);
}

// ---------------------------------------------------------------------------
// Start screen, movie manager.

/// `void StartScreen_Load(StartScreen* self)` (0x232b28): tell the graphics
/// manager to load (`v44(0x100)`), build the screen's UI engine (`+0xc`),
/// load its package and hook it to the app's pad and text tables, then push
/// the first UI state by the screen's mode (`+0x18`): 0 the memory-card
/// check (`cFEStateMemCard`), 2 a plain `UIState`, anything else none.
pub fn start_screen_load(c: &mut Cpu) {
    // `operator new(size, name, flags, 0)` and the UIStateMgr behind the engine.
    const ALLOC: u32 = 0x0031_7d70;
    const UI_ENGINE_CTOR: u32 = 0x0039_7b08;
    const UI_ENGINE_LOAD_PACKAGE: u32 = 0x0039_7c40;
    const MEM_CARD_STATE_CTOR: u32 = 0x0018_6a08;
    const UI_STATE_CTOR: u32 = 0x0039_e2a0;
    const UI_STATE_MGR_SWITCH_STATE: u32 = 0x0039_f400;
    /// The screen's UI engine.
    const ENGINE: u32 = 0xc;
    /// Which first state to push.
    const MODE: u32 = 0x18;
    guest(c, |c| {
        c.cycles += 52;
        let frame = Frame::open(c, 0x30);
        let gfx = Addr(r32(c, GP).wrapping_sub(0x854)).lw(c);
        c.g[A2][0] = gfx;
        frame.save(c, S1, 0x10);
        c.g[A1][0] = 0x100;
        frame.save(c, S0, 0x20);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        // gfx->v44(0x100): the vtable lives at +0x10d8, slot `{delta, fn}` at 0x160.
        c.g[V0][0] = Addr(gfx as u32).off(0x10d8).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt).off(0x160).lh(c);
        c.g[V1][0] = Addr(vt).off(0x164).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = abi::addu(gfx, c.g[A0][0]);
        call_ptr(c, target, 0x0023_2b58)?;

        // The UI engine: new(0x70), ctor, load the start screen package.
        c.g[A1][0] = 0x0047_b698;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x70;
        call_ptr(c, ALLOC, 0x0023_2b70)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, UI_ENGINE_CTOR, 0x0023_2b78)?;
        c.g[A1][0] = 0x0047_b9e0;
        c.g[A0][0] = c.g[V0][0];
        let v = c.g[V0][0];
        Addr(self_).off(ENGINE).sw(c, v);
        call_ptr(c, UI_ENGINE_LOAD_PACKAGE, 0x0023_2b8c)?;

        // engine+0x10 = app+0x8c; then a zeroed 0x1c-byte node (vtable at +8)
        // at engine+0xc.
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 0x0047_b9f0;
        c.g[T0][0] = Addr(self_).off(ENGINE).lw(c);
        c.g[A0][0] = 0x1c;
        c.g[V0][0] = Addr(r32(c, V1)).off(0x8c).lw(c);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        let v = c.g[V0][0];
        Addr(r32(c, T0)).off(0x10).sw(c, v);
        call_ptr(c, ALLOC, 0x0023_2bb4)?;
        let node = Addr(r32(c, V0));
        c.g[A0][0] = Addr(self_).off(ENGINE).lw(c);
        c.g[V1][0] = 0x0046_d1d0;
        node.off(0x4).sb(c, 0);
        let v = c.g[V1][0];
        node.off(0x8).sw(c, v);
        node.off(0x18).sw(c, 0);
        node.off(0xc).sw(c, 0);
        node.off(0x10).sw(c, 0);
        node.off(0x14).sw(c, 0);
        let v = c.g[V0][0];
        Addr(r32(c, A0)).off(0xc).sw(c, v);
        // engine+8 = app+0x88.
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr(self_).off(ENGINE).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x88).lw(c);
        let v = c.g[V0][0];
        Addr(r32(c, A0)).off(0x8).sw(c, v);

        c.g[V1][0] = Addr(self_).off(MODE).lw(c);
        c.g[V0][0] = 2;
        if c.g[V1][0] == 0 {
            // The memory-card check state.
            c.cycles += 15;
            c.g[A1][0] = 0x0047_b978;
            c.g[A0][0] = 0x248;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, ALLOC, 0x0023_2c10)?;
            c.g[A1][0] = Addr(self_).off(ENGINE).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, MEM_CARD_STATE_CTOR, 0x0023_2c1c)?;
            c.g[A0][0] = Addr(self_).off(ENGINE).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
            call_ptr(c, UI_STATE_MGR_SWITCH_STATE, 0x0023_2c2c)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 2;
            let mode2 = c.g[V1][0] == c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if mode2 {
                // A plain UIState with the start screen's vtable.
                c.cycles += 18;
                c.g[A1][0] = 0x0047_ba00;
                c.g[A0][0] = 0x48;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, ALLOC, 0x0023_2c54)?;
                c.g[S0][0] = c.g[V0][0];
                let state = r32(c, S0);
                c.g[A1][0] = Addr(self_).off(ENGINE).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, UI_STATE_CTOR, 0x0023_2c64)?;
                c.g[A0][0] = Addr(self_).off(ENGINE).lw(c);
                c.g[V0][0] = 0x0047_ca18;
                c.g[A1][0] = c.g[S0][0];
                let v = c.g[V0][0];
                Addr(state).off(0x8).sw(c, v);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                call_ptr(c, UI_STATE_MGR_SWITCH_STATE, 0x0023_2c80)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StartScreen_Unload(StartScreen* s)` (0x232d50): free the world's
/// loaded screen data, release the screen texture and unload the world.
pub fn start_screen_unload(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = Addr(gp_2(c, -0x854)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p_2(c, A1, 0x10d8)).lw(c);
    c.g[A0][0] = Addr(p_2(c, V0, 0x98)).lh(c);
    c.g[V1][0] = Addr(p_2(c, V0, 0x9c)).lw(c);
    let target = abi::r32(c, V1);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if call_ptr(c, target, 0x0023_2d78).is_err() {
        return;
    }
    c.g[A0][0] = Addr(p_2(c, S0, 0xc)).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = 3;
        if call_ptr(c, 0x0039_7b70, 0x0023_2d8c).is_err() {
            return;
        }
    }
    c.sw(p_2(c, S0, 0xc), 0);
    c.cycles += 6;
    c.g[A0][0] = Addr(gp_2(c, -0x854)).lw(c);
    if call_ptr(c, 0x0036_a020, 0x0023_2d98).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Load screens.

/// The three load-screen `TryExit`s: mark the screen done (`done_off`),
/// then return whether the loading object behind the world has finished
/// (its progress float at +4 compares equal to 0.0).
pub(crate) fn load_screen_try_exit(c: &mut Cpu, done_off: u32) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 1;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.sw(p_3(self_, done_off), c.g[V0][0]);
    c.mtc1(0, 1); // f1 = 0.0 (the zero register)
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.c_eq_s(0, 1);
    let done = c.fcc();
    c.g[A0][0] = 1;
    if !done {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `int PreStartScreen_TryExit(Screen* s)` (0x00232510): true once the
/// screen's timer (`+4`) reached 0.75 s.
pub fn pre_start_screen_try_exit(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = abi::arg(c, 0);
    c.lwc1(1, p_3(self_, 0x4));
    c.g[AT][0] = 0x3f40_0000; // 0.75f
    c.mtc1(1, 0);
    c.c_lt_s(1, 0);
    let below = c.fcc();
    c.g[V0][0] = 0;
    if !below {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int InitLoadScreen_TryExit(InitLoadScreen* s)` (0x002326d8).
pub fn init_load_screen_try_exit(c: &mut Cpu) {
    load_screen_try_exit(c, 0x8);
}

/// `int cPreGameLoadScreen_TryExit(cPreGameLoadScreen* s)` (0x002327b0).
pub fn cpre_game_load_screen_try_exit(c: &mut Cpu) {
    load_screen_try_exit(c, 0x4);
}

/// `int cPreFELoadScreen_TryExit(cPreFELoadScreen* s)` (0x002327e8).
pub fn cpre_fe_load_screen_try_exit(c: &mut Cpu) {
    load_screen_try_exit(c, 0x4);
}

// ---------------------------------------------------------------------------
// Start screen and camera algorithms.

/// `void StartScreen_Enter(StartScreen* self)` (0x232c98): start the game
/// from the front end through the world's race manager.
pub fn start_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(gp_3(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_2cb0)?;
        Addr(at(c, S0, 0x14)).sw(c, 0);
        Addr(gp_3(c, 0x1234)).sw(c, 0);
        Addr(gp_3(c, 0x14e8)).sw(c, 0);
        Addr(gp_3(c, 0x12d8)).sw(c, 0);
        c.g[V0][0] = Addr(gp_3(c, -0x854)).lw(c);
        c.g[V1][0] = Addr(r32_2(c, V0)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32_2(c, V1)).off(0x78).lh(c);
        Addr(r32_2(c, SP)).sw(c, 0);
        Addr(r32_2(c, SP).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        Addr(r32_2(c, SP).wrapping_add(8)).sw(c, 0);
        c.g[V0][0] = Addr(r32_2(c, V1)).off(0x7c).lw(c);
        let target = r32_2(c, V0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, target, 0x0023_2ce8)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int StartScreen_TryExit(StartScreen* self)` (0x232cf8): forwards to
/// `AppModule_TryExit` (0x231cf0) and returns its result.
pub fn start_screen_try_exit(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x0023_1cf0),
        0x0023_2d08,
    );
}

/// `void StartScreen_Update(StartScreen* self)` (0x232d18): run the app
/// module's fade, drive the screen's UI engine (`+0xc`) and count the frame
/// (`+0x14`).
pub fn start_screen_update(c: &mut Cpu) {
    /// The screen's UI engine.
    const ENGINE: u32 = 0xc;
    /// Frames since the screen started.
    const FRAMES: u32 = 0x14;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::APP_MODULE_UPDATE, 0x0023_2d2c)?;
        let this = r32(c, S0);
        c.g[A0][0] = c.lw(p(this, ENGINE));
        call_ptr(c, callee::UI_ENGINE_UPDATE, 0x0023_2d34)?;
        let frames = (c.lw(p(this, FRAMES)) as u32).wrapping_add(1);
        c.g[V0][0] = sx(frames);
        c.sw(p(this, FRAMES), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cGameLoadScreen_Enter()` (0x233260): snapshot the texture table, start
/// the game module, clear the flag at `gp+0x1234`, push an empty vector to the
/// render manager and start the loading-screen audio loop.
pub fn c_game_load_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = lw(c, gp(c, -0x854));
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0023_3274)?;
        c.g[A0][0] = lw(c, gp(c, -0x848));
        call_ptr(c, 0x0022_81d8, 0x0023_327c)?;
        sw(c, gp(c, 0x1234), 0);
        let g = lw(c, gp(c, -0x854));
        c.g[V0][0] = g;
        c.g[V1][0] = lw(c, p(g as u32, 0x10d8));
        c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x78));
        sw(c, sp(c), 0);
        sw(c, p(sp(c), 4), 0);
        c.g[A0][0] = sx((g as u32).wrapping_add(c.g[A0][0] as u32));
        sw(c, p(sp(c), 8), 0);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x7c));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sp(c) as u64;
        call_ptr(c, target, 0x0023_32a8)?;
        call_ptr(c, 0x0028_b180, 0x0023_32b0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_f768, 0x0023_32b8)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `StartScreen_Render` (0x232da8).
pub fn start_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_2dd0, 0x0023_2de0, 0x0023_2df0, 0x0023_2e08])
    })
}

/// `cGameLoadScreen_Render` (0x233390).
pub fn c_game_load_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_33b8, 0x0023_33c8, 0x0023_33d8, 0x0023_33f0])
    })
}

/// `int <X>_Render(void)` (0x232488, 0x232650, 0x232820, 0x2328a8): when the
/// FE screen is active, push its view, draw the loading indicator and the
/// screen's HUD slot; returns 1 when a screen was drawn.
pub(crate) fn fe_load_screen_render(c: &mut Cpu, entry: u32) -> Flow {
    c.cycles += 10;
    open(c, 0x20);
    let sp0 = spv(c);
    let g = gpa(c, -0x854);
    c.g[A1][0] = lw_at(c, g as u64, 0);
    c.sd(at_2(sp0, 0x10), c.g[RA][0]);
    let manager = c.g[A1][0];
    c.g[V1][0] = lw_at(c, manager, 0x10d8);
    c.g[A0][0] = lh_at(c, c.g[V1][0], 0x88);
    c.g[V0][0] = lw_at(c, c.g[V1][0], 0x8c);
    let func = c.g[V0][0] as u32;
    c.g[A0][0] = sx(at_2(manager as u32, c.g[A0][0] as u32));
    call_ptr(c, func, entry + 0x20)?;
    let active = c.g[V0][0] != 0;
    let g = gpa(c, -0x854);
    c.g[V0][0] = lw_at(c, g as u64, 0);
    if active {
        c.cycles += 20;
        let mgr = c.g[V0][0];
        c.g[V1][0] = lw_at(c, mgr, 0x10d8);
        c.g[A0][0] = lh_at(c, c.g[V1][0], 0x78);
        c.sw(sp0, 0);
        c.sw(at_2(sp0, 4), 0);
        c.g[A0][0] = sx(at_2(mgr as u32, c.g[A0][0] as u32));
        c.sw(at_2(sp0, 8), 0);
        c.g[V0][0] = lw_at(c, c.g[V1][0], 0x7c);
        let func = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, func, entry + 0x4c)?;
        let g = gpa(c, -0x848);
        c.g[V1][0] = lw_at(c, g as u64, 0);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 0xa4);
        callee::LOAD_INDICATOR_RENDER.call(c, entry + 0x58)?;
        let g = gpa(c, -0x854);
        c.g[A1][0] = lw_at(c, g as u64, 0);
        let mgr = c.g[A1][0];
        c.g[V0][0] = lw_at(c, mgr, 0x10d8);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0xa0);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0xa4);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_2(mgr as u32, c.g[A0][0] as u32));
        call_ptr(c, func, entry + 0x70)?;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(at_2(sp0, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(sp0, 0x20));
    c.pc = ra;
    Ok(())
}

/// `int PreStartScreen_Render(void)` (0x232488).
pub fn pre_start_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_2488))
}

/// `int InitLoadScreen_Render(void)` (0x232650).
pub fn init_load_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_2650))
}

/// `int cPreGameLoadScreen_Render(void)` (0x232820).
pub fn c_pre_game_load_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_2820))
}

/// `int cPreFELoadScreen_Render(void)` (0x2328a8).
pub fn c_pre_fe_load_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_28a8))
}

/// `void cGameLoadScreen_Unload(self)` (0x2332e8): destroy the screen and
/// audio of the load screen, then release every remaining sub-object.
pub fn c_game_load_screen_unload(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 14;
        c.g[A1][0] = lw(c, gp_4(c, -0x854));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at_3(c.g[A1][0], 0x10d8));
        c.g[A0][0] = lh(c, at_3(c.g[V0][0], 0x98));
        c.g[V1][0] = lw(c, at_3(c.g[V0][0], 0x9c));
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023_3314);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_3314)?;
        c.g[A0][0] = lw(c, at_3(c.g[S0][0], 0xc));
        if c.g[A0][0] == 0 {
            c.g[S0][0] = sx(at_3(c.g[S0][0], 0x10));
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0023_3328);
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_7b70, 0x0023_3328)?;
            c.g[S0][0] = sx(at_3(c.g[S0][0], 0x10));
        }
        c.cycles += 1;
        c.g[S1][0] = 1;
        loop {
            {
                c.cycles += 3;
                c.g[A1][0] = lw(c, c.g[S0][0] as u32);
                let t = c.g[A1][0] == 0;
                c.g[A2][0] = lw(c, gp_4(c, -0x854));
                if !t {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, at_3(c.g[A2][0], 0x10d8));
                    c.g[A0][0] = lh(c, at_3(c.g[V0][0], 0x190));
                    c.g[V1][0] = lw(c, at_3(c.g[V0][0], 0x194));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023_3350);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_3350)?;
                }
            }
            c.cycles += 3;
            c.g[S1][0] = sx(at_3(c.g[S1][0], 0xffff_ffff));
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(at_3(c.g[S0][0], 4));
            if !t {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0023_3364);
                c.g[A0][0] = lw(c, gp_4(c, -0x854));
                callee::GRAPHICS_MAN_FREE_ALL_TEXTURES.call(c, 0x0023_3364)?;
                c.g[RA][0] = sx(0x0023_336c);
                callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0023_336c)?;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[RA][0] = sx(0x0023_337c);
                c.g[A0][0] = c.g[V0][0];
                callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP.call(c, 0x0023_337c)?;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0023_3330);
            }
        }
    })
}

/// `void InitLoadScreen_Update(InitLoadScreen *self)` (0x232588): advance the
/// loading indicator and fade the progress bar in or out.
pub fn init_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(world(c));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
        callee::LOAD_INDICATOR_ADVANCE.call(c, 0x0023_25a4)?;
        c.g[V0][0] = Addr(p_4(c.g[S0][0], 0x8)).lw(c);
        let fade_out = c.g[V0][0] == 0;
        c.g[V0][0] = sx(world(c));
        let fcc;
        if !fade_out {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x518c));
            c.g[V0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
            c.mtc1(0, 2);
            c.lwc1(0, p_4(c.g[V0][0], 0x4));
            c.sub_s(0, 0, 1);
            c.c_lt_s(0, 2);
            fcc = c.fcc();
            c.swc1(0, p_4(c.g[V0][0], 0x4));
        } else {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x5188));
            c.g[V0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.lwc1(0, p_4(c.g[V0][0], 0x4));
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            fcc = c.fcc();
            c.swc1(0, p_4(c.g[V0][0], 0x4));
        }
        c.cycles += 2;
        if fcc {
            c.swc1(2, p_4(c.g[V0][0], 0x4));
        }
        c.cycles += 4;
        c.g[V0][0] = sx(world(c));
        c.g[V1][0] = Addr(p_4(c.g[V0][0], 0x4)).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 15;
            c.g[A1][0] = 0x0047_b8d8;
            c.g[A0][0] = 0x1c;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0023_2620)?;
            c.g[V1][0] = Addr(p_4(c.g[S0][0], 0x4)).lw(c);
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.g[V0][0];
            if c.g[V1][0] == 0 {
                c.g[A1][0] = 0;
            }
            callee::START_SCREEN_CTOR.call(c, 0x0023_2634)?;
            c.g[A0][0] = sx(world(c));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0023_2640)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPreGameLoadScreen_Update(cPreGameLoadScreen *self)` (0x232930):
/// advance the loading indicator, fade it in or out, and once the app has no
/// next module, create the 0x18-byte game module (vtable 0x47d788) and queue it.
pub fn c_pre_game_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(world(c));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
        callee::LOAD_INDICATOR_ADVANCE.call(c, 0x0023_294c)?;
        c.g[V0][0] = Addr(p_4(c.g[S0][0], 0x4)).lw(c);
        let fade_in = c.g[V0][0] == 0;
        c.g[V0][0] = sx(world(c));
        let fcc;
        if !fade_in {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x5184));
            c.g[V0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
            c.mtc1(0, 2);
            c.lwc1(0, p_4(c.g[V0][0], 0x4));
            c.sub_s(0, 0, 1);
            c.c_lt_s(0, 2);
            fcc = c.fcc();
            c.swc1(0, p_4(c.g[V0][0], 0x4));
        } else {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x5180));
            c.g[V0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.lwc1(0, p_4(c.g[V0][0], 0x4));
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            fcc = c.fcc();
            c.swc1(0, p_4(c.g[V0][0], 0x4));
        }
        c.cycles += 2;
        if fcc {
            c.swc1(2, p_4(c.g[V0][0], 0x4));
        }
        c.cycles += 4;
        c.g[V0][0] = sx(world(c));
        c.g[V1][0] = Addr(p_4(c.g[V0][0], 0x4)).lw(c);
        // Restored in the branch's delay slot: the callees below see the
        // caller's s0.
        frame.restore(c, S0, 0x10);
        if c.g[V1][0] == 0 {
            c.cycles += 19;
            c.g[A1][0] = 0x0047_b8e8;
            c.g[A0][0] = 0x18;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0023_29c8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::APP_MODULE_CTOR.call(c, 0x0023_29d4)?;
            let module = Addr(r32(c, S0));
            c.g[A0][0] = sx(world(c));
            let vtable = 0x0047_d788;
            c.g[V1][0] = vtable;
            c.g[A1][0] = c.g[S0][0];
            module.sw(c, vtable);
            module.off(0xc).sw(c, 0);
            module.off(0x10).sw(c, 0);
            module.off(0x14).sw(c, 0);
            call_ptr(c, 0x0031_6ef8, 0x0023_29f8)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPreFELoadScreen_Update(cPreFELoadScreen *self)` (0x232a08): twin of
/// `cPreGameLoadScreen_Update` with its own fade steps; once the app has no
/// next module, create the 0x1c-byte front-end module (vtable 0x47d740) and queue it.
pub fn c_pre_fe_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(world(c));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
        callee::LOAD_INDICATOR_ADVANCE.call(c, 0x0023_2a24)?;
        c.g[V0][0] = Addr(p_4(c.g[S0][0], 0x4)).lw(c);
        let fade_in = c.g[V0][0] == 0;
        c.g[V0][0] = sx(world(c));
        let fcc;
        if !fade_in {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x517c));
            c.g[V0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
            c.mtc1(0, 2);
            c.lwc1(0, p_4(c.g[V0][0], 0x4));
            c.sub_s(0, 0, 1);
            c.c_lt_s(0, 2);
            fcc = c.fcc();
            c.swc1(0, p_4(c.g[V0][0], 0x4));
        } else {
            c.cycles += 8;
            c.lwc1(1, gp(c, -0x5178));
            c.g[V0][0] = Addr(p_4(c.g[V0][0], 0xa4)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.lwc1(0, p_4(c.g[V0][0], 0x4));
            c.add_s(0, 0, 1);
            c.c_lt_s(2, 0);
            fcc = c.fcc();
            c.swc1(0, p_4(c.g[V0][0], 0x4));
        }
        c.cycles += 2;
        if fcc {
            c.swc1(2, p_4(c.g[V0][0], 0x4));
        }
        c.cycles += 4;
        c.g[V0][0] = sx(world(c));
        c.g[V1][0] = Addr(p_4(c.g[V0][0], 0x4)).lw(c);
        // Restored in the branch's delay slot: the callees below see the
        // caller's s0.
        frame.restore(c, S0, 0x10);
        if c.g[V1][0] == 0 {
            c.cycles += 19;
            c.g[A1][0] = 0x004a_2ab8;
            c.g[A0][0] = 0x1c;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0023_2aa0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::APP_MODULE_CTOR.call(c, 0x0023_2aac)?;
            let module = Addr(r32(c, S0));
            c.g[A0][0] = sx(world(c));
            let vtable = 0x0047_d740;
            c.g[V1][0] = vtable;
            c.g[A1][0] = c.g[S0][0];
            module.sw(c, vtable);
            module.off(0xc).sw(c, 0);
            module.off(0x10).sw(c, 0);
            module.off(0x14).sw(c, 0);
            call_ptr(c, 0x0031_6ef8, 0x0023_2ad0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00232348(void* self)` (0x232348): per-frame update of the
/// pre-start screen: tick the loading indicator, step the fade at_4
/// `world+0xa4+4` towards 1, spawn the next module once the app has none, and
/// advance the screen timer (faster when the world is loading).
pub fn pre_start_screen_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_232398: {
        c.cycles += 19;
        c.g[V0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, at_4(c, SP, 0x20));
        c.g[RA][0] = sx(0x00232368);
        c.g[A0][0] = lw_2(c, at_4(c, V0, 0xa4));
        if callee::C00231F80.call(c, 0x00232368).is_err() {
            return;
        }
        c.g[V0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x848));
        c.lwc1(1, r32(c, GP).wrapping_sub(0x5198));
        c.g[V0][0] = lw_2(c, at_4(c, V0, 0xa4));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 2);
        c.lwc1(0, at_4(c, V0, 0x4));
        c.add_s(0, 0, 1);
        c.c_lt_s(2, 0);
        let t = !c.fcc();
        c.swc1(0, at_4(c, V0, 0x4));
        if t {
            break 'b_232398;
        } else {
            c.cycles += 1;
            c.swc1(2, at_4(c, V0, 0x4));
            break 'b_232398;
        }
    }
    'b_2323e8: {
        c.cycles += 4;
        c.g[V0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x848));
        c.g[V1][0] = lw_2(c, at_4(c, V0, 0x4));
        if c.g[V1][0] != 0u64 {
            c.lwc1(1, at_4(c, S0, 0x4));
            break 'b_2323e8;
        } else {
            c.cycles += 16;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4738));
            c.g[A2][0] = 0x100;
            c.g[RA][0] = sx(0x002323c0);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317d70, 0x002323c0).is_err() {
                return;
            }
            c.g[V1][0] = 0x480000;
            c.g[A0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x848));
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x2710));
            sw_2(c, at_4(c, V0, 0x4), 0u64);
            sw_2(c, r32(c, V0), c.g[V1][0]);
            c.g[A1][0] = c.g[V0][0];
            sw_2(c, at_4(c, V0, 0x8), 0u64);
            c.g[RA][0] = sx(0x002323e4);
            sw_2(c, at_4(c, S0, 0x8), c.g[V0][0]);
            if call_ptr(c, 0x00316ef8, 0x002323e4).is_err() {
                return;
            }
            c.lwc1(1, at_4(c, S0, 0x4));
            break 'b_2323e8;
        }
    }
    'b_232478: {
        c.cycles += 7;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x5194));
        c.g[A0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x850));
        c.add_s(1, 1, 0);
        c.swc1(1, at_4(c, S0, 0x4));
        c.g[V0][0] = lw_2(c, at_4(c, A0, 0x2ee8));
        if (c.g[V0][0] as i64) <= 0 {
            frame.restore(c, S0, 0x10);
            break 'b_232478;
        } else {
            c.cycles += 8;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x5190));
            c.g[A1][0] = 0u64;
            c.add_s(0, 1, 0);
            c.g[RA][0] = sx(0x00232418);
            c.swc1(0, at_4(c, S0, 0x4));
            if call_ptr(c, 0x00326ca0, 0x00232418).is_err() {
                return;
            }
            c.g[V0][0] = ((c.g[V0][0] as i64) < 11i64) as u64;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, 0x10);
                break 'b_232478;
            } else {
                c.cycles += 9;
                c.g[A0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x850));
                c.mtc1(0, 20);
                c.g[RA][0] = sx(0x00232434);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x00326cc8, 0x00232434).is_err() {
                    return;
                }
                c.lwc1(0, at_4(c, V0, 0x20));
                c.c_eq_s(0, 20);
                if c.fcc() {
                    frame.restore(c, S0, 0x10);
                    break 'b_232478;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = lw_2(c, r32(c, GP).wrapping_sub(0x850));
                    c.g[RA][0] = sx(0x00232454);
                    c.g[A1][0] = 0u64;
                    if call_ptr(c, 0x00326cc8, 0x00232454).is_err() {
                        return;
                    }
                    c.lwc1(0, at_4(c, V0, 0x28));
                    c.c_eq_s(0, 20);
                    if c.fcc() {
                        frame.restore(c, S0, 0x10);
                        break 'b_232478;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = lw_2(c, at_4(c, S0, 0x8));
                        c.g[V0][0] = 0x1;
                        sw_2(c, at_4(c, V1, 0x4), c.g[V0][0]);
                        frame.restore(c, S0, 0x10);
                        break 'b_232478;
                    }
                }
            }
        }
    }
    c.cycles += 4;
    c.g[RA][0] = ld(c, r32(c, SP));
    c.lwc1(20, at_4(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x30));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// StartScreen + memory card.

/// `StartScreen* StartScreen_Load(self, arg)` (0x232ae0): the start screen's
/// constructor: `AppModule_ctor`, then the module's id/vtable and a cleared
/// field.
pub fn start_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::APP_MODULE_CTOR.call(c, 0x0023_2afc)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        Addr(self_.wrapping_add(0x18)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x2830));
        c.g[V0][0] = c.g[S0][0];
        Addr(self_).sw(c, c.g[V1][0]);
        Addr(self_.wrapping_add(0xc)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AppModule_TryExitWhenLoaded`.
pub(crate) const OC428_APP_MODULE_TRY_EXIT_WHEN_LOADED: Callee = Callee {
    addr: 0x0023_1c70,
    f: ee::s06::AppModule_TryExitWhenLoaded,
};

/// `bool cGameLoadScreen_TryExit(cGameLoadScreen* self)` (0x2332c8):
/// `AppModule_TryExitWhenLoaded(self) != 0`.
pub fn c_game_load_screen_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC428_APP_MODULE_TRY_EXIT_WHEN_LOADED.call(c, 0x0023_32d8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void cGameLoadScreen_LoadPictures(cGameLoadScreen* self, int)` (0x233048):
/// loads the load screen's textures from a BigFile. Rider race (the byte at
/// `0x535c11` is 2): the two characters' pictures (`sprintf` of the character
/// ids of both players into the name, `FILE_loadpackz`, then a
/// texture load through the graphics manager's vtable `+0x170` into
/// `self + 0x10 + 4 * i`). Otherwise: the course picture of the location id
/// (0 when above 16) into `self + 0x10`. The BigFile is closed with flag 2.
///
/// NOT registered: `cBigFile` open and `FILE_loadpackz` are real, unslotted
/// callees that read the disc, so no bounded differential fixture exists.
pub fn c_game_load_screen_load_pictures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S2, 0xe0);
        frame.save(c, S0, 0x100);
        frame.save(c, S3, 0xd0);
        frame.save(c, S1, 0xf0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3078)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_3084)?; // cBE_getInterface(1): players
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_308c)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_3098)?; // cBE_getInterface(0): race
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 2;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
        let course = c.g[A0][0] != c.g[V1][0];
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        if !course {
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = 0x48_0000;
            c.g[S7][0] = c.g[A0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4288));
            c.g[A2][0] = 0x100;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_68b0, 0x0023_30cc)?; // cBigFile open
            c.g[S1][0] = 0;
            c.g[V0][0] = 0x44_0000;
            c.g[S5][0] = 0x48_0000;
            c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
            c.g[S4][0] = 0x48_0000;
            loop {
                c.cycles += 29;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = (0 < c.g[S1][0]) as u64;
                call_ptr(c, 0x0014_7488, 0x0023_30ec)?; // getPlayerCharID
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x5b4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4270));
                call_ptr(c, 0x0041_7828, 0x0023_3108)?; // sprintf
                c.g[A1][0] = 0x300_0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] |= 0x100;
                call_ptr(c, 0x003e_2190, 0x0023_3118)?; // FILE_loadpackz
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4260));
                c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A3][0] = 0;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[T0][0] = 1;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                c.g[T1][0] = 0xffff_ffff_ffff_ffff;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_314c)?; // loadTex
                let no_data = c.g[S0][0] == 0;
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                if !no_data {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_7e98, 0x0023_315c)?; // EA_MemFree
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_30e0);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_68e8, 0x0023_3178)?; // cBigFile close
            frame.restore(c, S0, 0x100);
        } else {
            c.cycles += 40;
            c.g[A1][0] = 0x48_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4250));
            c.g[A2][0] = 0x100;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_68b0, 0x0023_3194)?; // cBigFile open
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014_4be0, 0x0023_319c)?; // getLocationID
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[A2][0] = 0x44_0000;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2950));
            c.g[V1][0] = ((c.g[V0][0] as i64) < 17) as u64;
            c.g[A1][0] = 0x4a_0000;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = 0;
            }
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            call_ptr(c, 0x0041_7828, 0x0023_31d0)?; // sprintf
            c.g[A1][0] = 0x300_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] |= 0x100;
            call_ptr(c, 0x003e_2190, 0x0023_31e0)?; // FILE_loadpackz
            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[A2][0] = 0x48_0000;
            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4238));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
            c.g[T0][0] = 1;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
            c.g[T1][0] = 0xffff_ffff_ffff_ffff;
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_3218)?; // loadTex
            let no_data = c.g[S0][0] == 0;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            if !no_data {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x0023_3228)?; // EA_MemFree
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_68e8, 0x0023_3234)?; // cBigFile close
            frame.restore(c, S0, 0x100);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void cGameLoadScreen_Load(cGameLoadScreen* self)` (0x232e20): builds the
/// load screen UI ("data/ui/gl"). Asks the graphics manager (vtable `+0x160`,
/// argument 0x100) to prepare, allocates the 0x70-byte UI engine (`+0xc`) and
/// loads the file into it, hands it the world's fonts and locale (`+0x88`,
/// `+0x8c`), gives it a fresh 0x1c-byte input node, and pushes the first load
/// state: the lodge state (0x48 bytes) when the lodge flag (`gp - 0x1718`) is
/// set, or the race has no path and the mode is 5 or 6; the conquer state
/// (0x48 bytes, clears `gp - 0x1714`) when there is no path and the mode is
/// not 5/6; otherwise the 0x4c-byte picture state, which loads the pictures
/// (`cGameLoadScreen_LoadPictures`).
pub fn c_game_load_screen_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        frame.save(c, S2, 0x10);
        c.g[A1][0] = 0x100;
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_2e54)?; // graphics manager vtable call
        call_ptr(c, 0x0014_dc80, 0x0023_2e5c)?; // cBE_getBE
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_2e68)?; // cBE_getInterface(0)
        c.g[A1][0] = 0x48_0000;
        c.g[A2][0] = 0x100;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4968));
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0023_2e80)?; // cMemMan_alloc (UIEngine)
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7b08, 0x0023_2e88)?; // UIEngine ctor
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x42d8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x0039_7c40, 0x0023_2e9c)?; // UIEngine_loadFile("data/ui/gl")
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x48_0000;
        c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = 0x1c;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x88));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4610));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.sw((c.g[T1][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        call_ptr(c, 0x0031_7d70, 0x0023_2ed0)?; // cMemMan_alloc (input node)
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = 0x47_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e30));
        c.sb((c.g[V0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(8), c.g[V1][0]);
        let node = c.g[V0][0] as u32;
        for off in [0x18u32, 0xc, 0x10, 0x14] {
            c.sw(node.wrapping_add(off), 0);
        }
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
        let lodge_flag = c.g[V1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        // 0: lodge state, 1: conquer state, 2: picture state
        let state;
        if lodge_flag {
            state = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = 0x53_0000;
            c.g[A0][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            if c.g[V0][0] != 0 {
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                state = 3;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = 5;
                let mode5 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 6;
                if mode5 {
                    state = 0;
                } else {
                    c.cycles += 2;
                    let not6 = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                    state = if not6 { 3 } else { 0 };
                }
            }
        }
        let mut state = state;
        if state == 3 {
            // other: conquer state unless a path is set or the mode is 5/6
            c.cycles += 3;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            let path = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if !path {
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.g[V1][0] ^ 5;
                let mode5 = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] ^ 6;
                if !mode5 {
                    c.cycles += 1;
                    c.g[A0][0] = (0 < c.g[V0][0]) as u64;
                }
            }
            c.cycles += 2;
            c.g[A1][0] = 0x48_0000;
            state = if c.g[A0][0] != 0 { 1 } else { 2 };
        }
        match state {
            0 => {
                c.cycles += 16;
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = 0x48;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x42c8));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_2f48)?; // cMemMan_alloc (lodge state)
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_2f5c)?; // load item ctor
                c.g[V1][0] = 0x48_0000;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3788));
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
            }
            1 => {
                c.cycles += 15;
                c.g[A0][0] = 0x48;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x42b0));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_2fac)?; // cMemMan_alloc (conquer state)
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_2fc0)?; // load item ctor
                c.g[V1][0] = 0x48_0000;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1714), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3858));
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
            }
            _ => {
                c.cycles += 19;
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = 0x4c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4298));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_2fec)?; // cMemMan_alloc (picture state)
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_3000)?; // load item ctor
                c.g[V1][0] = 0x48_0000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x36b8));
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
                c.g[A1][0] = 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[S2][0]);
                call_ptr(c, 0x0023_3048, 0x0023_3020)?; // cGameLoadScreen_LoadPictures
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x0023_3030)?; // UIStateStack_push
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cInitLoadScreen::load(this)` (0x232568): `cSSXApp_initload(SSXApp)`
/// with `SSXApp = *(gp - 0x848)`. Not installed (see `OVERRIDES`): the callee
/// is a huge unported driver with no slot to fake, so the return path has no
/// bounded fixture.
#[allow(dead_code)]
pub fn init_load_screen_enter_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_82d0, 0x0023_257c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cGameLoadScreen_LoadPictures` (0x00233048): Loads the load-screen pictures from the big file: the two character pictures in a two-player race, else the course picture. Stores the textures at self+0x10. `(self)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): EAFile_LoadDecompressed/BigFile loads (pc 0x3de768) have no override slot and spin on test data.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn c_game_load_screen_load_pictures_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_233238: {
            'b_233180: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00233078)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00233084)?; // call_indirect
                c.g[S3][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023308c)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00233098)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x2;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let t = c.g[A0][0] != c.g[V1][0];
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_233180;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = 0x480000;
                    c.g[S7][0] = c.g[A0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4288));
                    c.g[A2][0] = 0x100;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003168b0, 0x002330cc)?; // call_indirect
                    c.g[S1][0] = 0u64;
                    c.g[V0][0] = 0x440000;
                    c.g[S5][0] = 0x480000;
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
                    c.g[S4][0] = 0x480000;
                    'l_2330e0: loop {
                        'b_23315c: {
                            c.cycles += 29;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = (0u64 < c.g[S1][0]) as u64;
                            call_ptr(c, 0x00147488, 0x002330ec)?; // call_indirect
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x5b4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4270));
                            call_ptr(c, 0x00417828, 0x00233108)?; // call_indirect
                            c.g[A1][0] = 0x3000000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[A1][0] | 0x100;
                            call_ptr(c, 0x003e2190, 0x00233118)?; // call_indirect
                            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4260));
                            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[T0][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0023314c)?; // call_indirect
                            let t = c.g[S0][0] == 0u64;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_23315c;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x0023315c)?; // call_indirect
                                break 'b_23315c;
                            }
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002330e0);
                            }
                            continue 'l_2330e0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S7][0];
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003168e8, 0x00233178)?; // call_indirect
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                            if t {
                                break 'b_233238;
                            } else {
                                break 'b_233180;
                            }
                        }
                    }
                }
            }
            'b_233228: {
                c.cycles += 40;
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4250));
                c.g[A2][0] = 0x100;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003168b0, 0x00233194)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144be0, 0x0023319c)?; // call_indirect
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[A2][0] = 0x440000;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2950));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                c.g[A1][0] = 0x4a0000;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                call_ptr(c, 0x00417828, 0x002331d0)?; // call_indirect
                c.g[A1][0] = 0x3000000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] | 0x100;
                call_ptr(c, 0x003e2190, 0x002331e0)?; // call_indirect
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = 0x480000;
                c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4238));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                c.g[T0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                c.g[T1][0] = 0xffffffffffffffff;
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00233218)?; // call_indirect
                let t = c.g[S0][0] == 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_233228;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x00233228)?; // call_indirect
                    break 'b_233228;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x003168e8, 0x00233234)?; // call_indirect
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            break 'b_233238;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    })
}

/// `void cGameLoadScreen_LoadPictures(cGameLoadScreen* self, int)` (0x233048):
/// loads the load screen's textures from a BigFile. Rider race (the byte at
/// `0x535c11` is 2): the two characters' pictures (`sprintf` of the character
/// ids of both players, `FILE_loadpackz`, then a texture load through the
/// graphics manager's vtable `+0x170` into `self + 0x10 + 4 * i`). Otherwise:
/// the course picture of the location id (0 when above 16) into `self + 0x10`.
/// The BigFile is closed with flag 2.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn c_game_load_screen_load_pictures_3(c: &mut Cpu) {
    guest(c, |c| {
        'b_233238: {
            'b_233180: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00233078)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00233084)?; // call_indirect
                c.g[S3][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023308c)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00233098)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x2;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let t = c.g[A0][0] != c.g[V1][0];
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_233180;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = 0x480000;
                    c.g[S7][0] = c.g[A0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4288));
                    c.g[A2][0] = 0x100;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003168b0, 0x002330cc)?; // call_indirect
                    c.g[S1][0] = 0u64;
                    c.g[V0][0] = 0x440000;
                    c.g[S5][0] = 0x480000;
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
                    c.g[S4][0] = 0x480000;
                    'l_2330e0: loop {
                        'b_23315c: {
                            c.cycles += 29;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = (0u64 < c.g[S1][0]) as u64;
                            call_ptr(c, 0x00147488, 0x002330ec)?; // call_indirect
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x5b4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4270));
                            call_ptr(c, 0x00417828, 0x00233108)?; // call_indirect
                            c.g[A1][0] = 0x3000000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[A1][0] | 0x100;
                            call_ptr(c, 0x003e2190, 0x00233118)?; // call_indirect
                            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4260));
                            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[T0][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0023314c)?; // call_indirect
                            let t = c.g[S0][0] == 0u64;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_23315c;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x0023315c)?; // call_indirect
                                break 'b_23315c;
                            }
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002330e0);
                            }
                            continue 'l_2330e0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S7][0];
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003168e8, 0x00233178)?; // call_indirect
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                            if t {
                                break 'b_233238;
                            } else {
                                break 'b_233180;
                            }
                        }
                    }
                }
            }
            'b_233228: {
                c.cycles += 40;
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4250));
                c.g[A2][0] = 0x100;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003168b0, 0x00233194)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144be0, 0x0023319c)?; // call_indirect
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[A2][0] = 0x440000;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2950));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                c.g[A1][0] = 0x4a0000;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                call_ptr(c, 0x00417828, 0x002331d0)?; // call_indirect
                c.g[A1][0] = 0x3000000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] | 0x100;
                call_ptr(c, 0x003e2190, 0x002331e0)?; // call_indirect
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = 0x480000;
                c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4238));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                c.g[T0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                c.g[T1][0] = 0xffffffffffffffff;
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00233218)?; // call_indirect
                let t = c.g[S0][0] == 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_233228;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x00233228)?; // call_indirect
                    break 'b_233228;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x003168e8, 0x00233234)?; // call_indirect
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            break 'b_233238;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    })
}

/// `void InitLoadScreen_Enter(void)` (0x00232568): start the boot load,
/// `cSSXApp_initload(*(gp - 0x848))` (the app object). The pre-existing
/// `init_load_screen_enter` / `_2` in `loadscreens_prestart.rs` are untested
/// duplicates of this: delete them at the reorg.
pub fn init_load_screen_enter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_82d0, 0x0023_257c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base;
    pub(crate) use crate::game::render::particle::tests::f;

    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_3;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::main::ssxappdtor::tests::dtor_case;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    pub(crate) use crate::game::world::worldview::tests::pick;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn c_pre_game_load_screen_ctor_matches() {
        check(
            ssx3_ee::s06::f_00232720,
            c_pre_game_load_screen_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..2 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_pre_fe_load_screen_ctor_matches() {
        check(
            ssx3_ee::s06::f_00232738,
            c_pre_fe_load_screen_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..2 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    // ---------------------------------------------------------- load screens

    #[test]
    fn c_game_load_screen_update_matches() {
        check(
            ee::s06::f_00233408__generated,
            c_game_load_screen_update,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x233408, 4);
                globals(c);
                clear(c, DATA, 0x1000);
                c.g[A0][0] = DATA as u64;
                // UIEngine -> (+0xc) view -> (+8) vtable -> +0x170 slot.
                let engine = DATA + 0x800;
                let view = DATA + 0x900;
                c.bus.write32(DATA + 0xc, engine);
                c.bus.write32(engine + 0xc, view);
                let vt = vtable(c, &mut r, &[0x170]);
                c.bus.write32(view + 8, vt);
                let _ = seed;
            },
        );
    }

    #[test]
    fn start_screen_update_matches() {
        check(
            ee::s06::f_00232d18__generated,
            start_screen_update,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232d18, 4);
                globals(c);
                clear(c, DATA, 0x1000);
                c.g[A0][0] = DATA as u64;
                // AppModule fade: +8 fading in/out, +4 the fade level.
                c.bus.write32(DATA + 8, seed & 1);
                let level = [0.0f32, 0.01, 0.5, 1.0][(seed as usize >> 1) & 3];
                c.bus.write32(DATA + 4, level.to_bits());
                // Frame counter, including the wrap.
                let frames = if seed % 7 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(DATA + 0x14, frames);
                // UIEngine -> (+0xc) view -> (+8) vtable -> +0x170 slot.
                let engine = DATA + 0x800;
                let view = DATA + 0x900;
                c.bus.write32(DATA + 0xc, engine);
                c.bus.write32(engine + 0xc, view);
                let vt = vtable(c, &mut r, &[0x170]);
                c.bus.write32(view + 8, vt);
            },
        );
    }

    #[test]
    fn init_load_screen_dtor_matches() {
        dtor_case(
            ee::s06::f_00232538__generated,
            init_load_screen_dtor,
            0x232538,
        );
    }

    #[test]
    fn c_pre_game_load_screen_dtor_matches() {
        dtor_case(
            ee::s06::f_00232750__generated,
            c_pre_game_load_screen_dtor,
            0x232750,
        );
    }

    #[test]
    fn c_pre_fe_load_screen_dtor_matches() {
        dtor_case(
            ee::s06::f_00232780__generated,
            c_pre_fe_load_screen_dtor,
            0x232780,
        );
    }

    #[test]
    fn start_screen_unload_matches() {
        check(
            ssx3_ee::s06::f_00232d50__generated,
            start_screen_unload,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232d50, 3);
                heap_init(c);
                let world = DATA + 0x100;
                let screen = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x854, world);
                // The graphics manager vtable: `v19` at 0x98 and
                // `FreeAllTextures` at 0x3c0.
                let vt = vtable(c, &mut r, &[0x98, 0x3c0]);
                c.bus.write32(world + 0x10d8, vt);
                let tex = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(screen + 0xc, tex);
                c.g[A0][0] = screen as u64;
            },
        );
    }

    #[test]
    fn start_screen_load_matches() {
        use crate::game::movie::movieplayer::tests::alloc_base as alloc_base_2;
        use crate::game::render::lightman::tests::eafile_pools;
        check(
            ssx3_ee::s06::f_00232b28__generated,
            start_screen_load,
            24,
            &|c, seed| {
                // One fake call (the graphics manager's v44) that may unwind.
                let mut r = alloc_base_2(c, seed, 0x232b28, 1);
                eafile_pools(c);
                // The BELibrary dummy and world of the cFEStateMemCard tests.
                be_library(c);
                world(c);
                let wld = DATA + 0x1000;
                c.bus.write32(wld + 0x88, r.next_u32());
                c.bus.write32(wld + 0x8c, r.next_u32());
                // The MCManager singleton already exists (no construction).
                c.bus.write32(GAME_GP - 0x47c, DATA + 0xc000);
                // The graphics manager: vtable at +0x10d8, slot 0x160 fake.
                let gfx = DATA + 0x9000;
                let vt = DATA + 0xb000;
                c.bus.write32(GAME_GP - 0x854, gfx);
                c.bus.write32(gfx + 0x10d8, vt);
                c.bus.write16(vt + 0x160, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + 0x164, FAKE);
                let screen = DATA + 0x8000;
                let mode = match seed % 3 {
                    0 => 0,
                    1 => 2,
                    _ => 3 + r.next_u32() % 8,
                };
                c.bus.write32(screen + 0x18, mode);
                c.g[A0][0] = screen as u64;
            },
        );
    }

    #[test]
    fn pre_start_screen_try_exit_matches() {
        check(
            ssx3_ee::s06::f_00232510__generated,
            pre_start_screen_try_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232510, 0);
                c.g[A0][0] = sx(DATA);
                word(
                    c,
                    DATA + 4,
                    pick(&mut r, &[0.0, -0.0, 0.1, 0.7499, 0.75, 1.0, -1.0]),
                );
            },
        );
    }

    #[test]
    fn load_screen_try_exits_match() {
        check(
            ssx3_ee::s06::f_002326d8__generated,
            init_load_screen_try_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2326d8, 0);
                c.g[A0][0] = sx(DATA);
                word(c, DATA + 8, 0);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_sub(0x848),
                    DATA + 0x300,
                );
                word(c, DATA + 0x300 + 0xa4, DATA + 0x400);
                word(c, DATA + 0x404, pick(&mut r, &[0.0, -0.0, 1.0, 1.0e-30]));
            },
        );
        check(
            ssx3_ee::s06::f_002327b0__generated,
            cpre_game_load_screen_try_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2327b0, 0);
                c.g[A0][0] = sx(DATA);
                word(c, DATA + 4, 0);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_sub(0x848),
                    DATA + 0x300,
                );
                word(c, DATA + 0x300 + 0xa4, DATA + 0x400);
                word(c, DATA + 0x404, pick(&mut r, &[0.0, -0.0, 1.0, 1.0e-30]));
            },
        );
        check(
            ssx3_ee::s06::f_002327e8__generated,
            cpre_fe_load_screen_try_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2327e8, 0);
                c.g[A0][0] = sx(DATA);
                word(c, DATA + 4, 0);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                word(
                    c,
                    crate::difftest::GAME_GP.wrapping_sub(0x848),
                    DATA + 0x300,
                );
                word(c, DATA + 0x300 + 0xa4, DATA + 0x400);
                word(c, DATA + 0x404, pick(&mut r, &[0.0, -0.0, 1.0, 1.0e-30]));
            },
        );
    }

    #[test]
    fn start_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_00232c98__generated,
            start_screen_enter,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x232c98, 0);
                w(c, GAME_GP - 0x854, 0);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn start_screen_try_exit_matches() {
        check(
            ssx3_ee::s06::f_00232cf8__generated,
            start_screen_try_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232cf8, 0);
                c.g[A0][0] = sx(DATA);
                word(c, DATA + 8, 0);
                c.g[GP][0] = GAME_GP as u64;
                word(
                    c,
                    GAME_GP.wrapping_sub(0x51b4),
                    pick(&mut r, &[0.0, 0.5, 1.0]),
                );
                word(
                    c,
                    DATA + 4,
                    pick(&mut r, &[0.0, -0.0, 0.4999, 0.5, 0.75, 1.0, -1.0]),
                );
            },
        );
    }

    // ------------------------------------------------------------ load screens

    #[test]
    fn c_game_load_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_00233260__generated,
            c_game_load_screen_enter,
            30,
            &|c, seed| {
                let mut r = setup(c, seed, 0x233260);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), DATA + 0x200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x300);
                let rm = DATA + 0x400;
                c.bus.write32(DATA + 0x200 + 0x10d8, rm);
                c.bus.write32(rm + 0x78, 0);
                c.bus.write32(rm + 0x7c, 0);
                c.bus.write32(DATA + 0x300 + 0x84, 0);
                c.bus.write32(DATA + 0x300 + 0x7c, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn start_screen_render_matches() {
        check(
            ee::s06::f_00232da8__generated,
            start_screen_render,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x232da8, 4);
                be_library(c);
                world(c);
                heap_init_2(c);
                let app = DATA + 0x2000;
                w_2(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w_2(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w_2(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    #[test]
    fn c_game_load_screen_render_matches() {
        check(
            ee::s06::f_00233390__generated,
            c_game_load_screen_render,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x233390, 4);
                be_library(c);
                world(c);
                heap_init_2(c);
                let app = DATA + 0x2000;
                w_2(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w_2(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w_2(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    // ------------------------------------------------------ load screen unload

    #[test]
    fn c_game_load_screen_unload_matches() {
        check(
            ee::s06::f_002332e8__generated,
            c_game_load_screen_unload,
            SEEDS_3,
            &|c, seed| {
                let _ = base(c, seed, 0x2332_e8, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w_3(c, obj + 0xc, if seed % 2 == 0 { 0 } else { obj + 0x2000 });
                for i in 0..3u32 {
                    w_3(
                        c,
                        obj + 0x10 + 4 * i,
                        if (seed % 4) as u32 == i {
                            obj + 0x3000 + 4 * i
                        } else {
                            0
                        },
                    );
                }
                for i in 0..3u32 {
                    w_3(c, obj + 0x3000 + 4 * i + 4, obj + 0x2100);
                }
                w_3(c, IFACE + 0x10d8, obj + 0x2200);
                slot_2(c, obj + 0x2200, 0x98, 0);
                slot_2(c, obj + 0x2200, 0x190, 1);
            },
        );
    }

    #[test]
    fn init_load_screen_update_matches() {
        check(
            ee::s06::f_00232588__generated,
            init_load_screen_update,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2325);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 8, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(W_2 + 0xa4, DATA + 0x3000);
                c.bus.write32(DATA + 0x3004, 0x3f00_0000 | (seed & 0x7f));
                // Skip creating the start screen except for a few seeds.
                c.bus.write32(W_2 + 4, if seed % 8 == 0 { 0 } else { 1 });
                c.bus.write32(gp_at(-0x518c), f(0.05));
                c.bus.write32(gp_at(-0x5188), f(0.05));
            },
        );
    }

    #[test]
    fn c_pre_game_load_screen_update_matches() {
        check(
            ee::s06::f_00232930__generated,
            c_pre_game_load_screen_update,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2329);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 4, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(W_2 + 0xa4, DATA + 0x3000);
                // Fade value: mid-range, or close enough to 0/1 to clamp.
                let fade = [0.5, 0.02, 0.98, -0.1][(seed as usize / 2) % 4];
                c.bus.write32(DATA + 0x3004, f(fade));
                // Create the game module only for a few seeds.
                c.bus.write32(W_2 + 4, if seed % 8 < 2 { 0 } else { 1 });
                c.bus.write32(gp_at(-0x5184), f(0.05));
                c.bus.write32(gp_at(-0x5180), f(0.05));
                c.bus.write32(gp_at(-0x51b8), f(0.25));
            },
        );
    }

    #[test]
    fn c_pre_fe_load_screen_update_matches() {
        check(
            ee::s06::f_00232a08__generated,
            c_pre_fe_load_screen_update,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x232a);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 4, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(W_2 + 0xa4, DATA + 0x3000);
                // Fade value: mid-range, or close enough to 0/1 to clamp.
                let fade = [0.5, 0.02, 0.98, -0.1][(seed as usize / 2) % 4];
                c.bus.write32(DATA + 0x3004, f(fade));
                // Create the front-end module only for a few seeds.
                c.bus.write32(W_2 + 4, if seed % 8 < 2 { 0 } else { 1 });
                c.bus.write32(gp_at(-0x517c), f(0.05));
                c.bus.write32(gp_at(-0x5178), f(0.05));
                c.bus.write32(gp_at(-0x51b8), f(0.25));
            },
        );
    }

    pub(crate) const PRE_START: u32 = DATA + 0x1_0000;

    pub(crate) const PRE_MODULE: u32 = DATA + 0x1_1000;

    #[test]
    fn pre_start_screen_update_matches() {
        check_all(
            ee::s06::f_00232348__generated,
            pre_start_screen_update,
            24,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x2323_48, 8);
                c.g[A0][0] = sx(PRE_START);
                c.bus.write32(PRE_START + 0x0, (seed % 4) as u32);
                c.bus.write32(PRE_START + 0x4, r.edge_f32());
                c.bus.write32(WORLD + 0xa4, PRE_MODULE);
                c.bus.write32(PRE_MODULE + 0x4, r.edge_f32());
                c.bus.write32(WORLD + 0x4, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn start_screen_ctor_matches() {
        check(
            ee::s06::f_00232ae0__generated,
            start_screen_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232a_e0, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(GAME_GP - 0x51b8, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;
    pub(crate) use crate::game::luno::lunovm::tests::oc428_w;

    /// The BELibrary dummy (`gp - 0x1f04`): without it a library lookup in a
    /// real callee loads from disc forever.
    pub(crate) fn oc428_belib(c: &mut Cpu) {
        oc428_w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    /// `cGameLoadScreen_TryExit` against the generated body: the progress
    /// global at 100% (the exit request runs) or not.
    #[test]
    fn c_game_load_screen_try_exit_matches() {
        crate::difftest::check(
            ee::s06::f_002332c8__generated,
            c_game_load_screen_try_exit,
            60,
            &|c, seed| {
                machine(c, seed, 0x2332c8, 1);
                oc428_belib(c);
                c.g[A0][0] = DATA as u64;
                oc428_w(c, GAME_GP + 0x2a74, DATA + 0x100);
                let progress: f32 = if seed % 2 == 0 { 100.0 } else { 1.0 };
                oc428_w(c, DATA + 0x100 + 0x38, progress.to_bits());
            },
        );
    }

    /// Fixture for `cGameLoadScreen_Load`: the graphics manager (`gp - 0x854`)
    /// with a vtable whose slot `+0x160` is a fake, the world (`gp - 0x848`),
    /// the lodge flag (`gp - 0x1718`) and the race setup (no path, mode 0..7).
    /// The picture state (which runs the real `LoadPictures`) is never chosen:
    /// that needs a path set without the lodge flag.
    pub(crate) fn oc430_load_screen_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let gp = c.g[GP][0] as u32;
        let (grman, vt, world) = (DATA + 0x1000, DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(gp - 0x854, grman);
        c.bus.write32(grman + 0x10d8, vt);
        c.bus.write16(vt + 0x160, (r.next_u32() as i16 & !3) as u16);
        c.bus.write32(vt + 0x164, fakes::FAKE);
        c.bus.write32(gp - 0x848, world);
        c.bus.write32(world + 0x88, r.next_u32());
        c.bus.write32(world + 0x8c, r.next_u32());
        c.bus
            .write32(gp - 0x1718, if seed % 3 == 0 { 1 } else { 0 });
        c.bus.write8(0x0053_5bc8 + 0x49, 0);
        c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 8) as u8);
        c.g[A0][0] = sx(DATA + 0x5000);
    }

    /// `cGameLoadScreen_Load`: the lodge state (flag, or mode 5/6) and the
    /// conquer state (other modes); the allocator, constructors, the file load,
    /// the UI push and the graphics manager's method are fakes.
    #[test]
    fn c_game_load_screen_load_matches() {
        oc430_install_fakes();
        covered(
            ee::s06::f_00232e20__generated,
            c_game_load_screen_load,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0023_2e20, 10);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0023_2e20;
                oc430_load_screen_fixture(c, &mut r, seed);
            },
        );
    }

    pub(crate) use crate::game::be::beintstat::tests::oc673_machine;
    pub(crate) use crate::game::be::beintstat::tests::oc673_result_at;

    pub(crate) use crate::game::ai::ai::tests::oc674_gp;
    pub(crate) use crate::game::ai::ai::tests::oc674_install;
    pub(crate) use crate::game::ai::ai::tests::oc674_machine;

    /// `cGameLoadScreen_LoadPictures` against the generated body: the
    /// two-player branch (two pictures, the loaded file present or missing)
    /// and the single course picture (location id above 16 or not), unwinds
    /// in each call. The graphics manager's vtable slots are fake methods.
    #[test]
    fn c_game_load_screen_load_pictures_matches() {
        fakes::covered(
            ee::s06::f_00233048__generated,
            c_game_load_screen_load_pictures_3,
            200,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x233048, 17, 0x0023_3048);
                let this = DATA + 0x9_0000;
                let gm = DATA + 0x9_1000;
                let g = DATA + 0x9_2000;
                let file = DATA + 0x9_3000;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP - 0x854, gm);
                c.bus.write32(gm + 0x10d8, g);
                c.bus.write16(g + 0x170, ((r.next_u32() % 0x40) * 4) as u16);
                c.bus.write32(g + 0x174, FAKE);
                c.bus
                    .write8(0x0053_5c11, [2, 2, 3][(r.next_u32() % 3) as usize]);
                let loaded = if r.next_u32() % 4 == 0 { 0 } else { file };
                oc673_result_at(c, 0x003e_2190, loaded);
                c.bus.write32(file + 0x14, (r.next_u32() % 0x100) & !3);
                oc673_result_at(
                    c,
                    0x0014_4be0,
                    [5, 0x12, 0xff, 0x10, 0][(r.next_u32() % 5) as usize],
                );
            },
        );
    }

    /// `InitLoadScreen_Enter` stops at its first callee here; both sides log
    /// the call and agree on everything up to the unwind.
    #[test]
    fn init_load_screen_enter_matches() {
        oc674_install();
        check(
            ee::s06::f_00232568__generated,
            init_load_screen_enter_load,
            40,
            &|c, seed| {
                let mut r = oc674_machine(c, seed, 0x0674_0001, 0);
                c.pc = 0x0023_2568;
                let at = oc674_gp(c, 0x848);
                c.bus.write32(at, r.next_u32() & 0x00ff_fff0);
            },
        );
    }
}
