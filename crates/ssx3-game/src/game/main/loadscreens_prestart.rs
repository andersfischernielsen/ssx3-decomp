//! `src/main/loadscreens_prestart.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Flow;
use crate::call::Frame;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base;
    pub(crate) use crate::game::bx::bxstring2::tests::f;

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
}
