//! `src/main/game.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
#[cfg(test)]
use crate::difftest::fakes;
#[cfg(test)]
use crate::difftest::fill;
#[cfg(test)]
use crate::difftest::Rng;
#[cfg(test)]
use crate::difftest::DATA;
#[cfg(test)]
use crate::difftest::GAME_GP;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::delete_if_flag;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::misc::gp_lit;
pub(crate) use crate::game::misc::lit;
pub(crate) use crate::game::movie::movieplayer::add32 as add32_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_add;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw0;
pub(crate) use crate::game::world::world::oc435_callee;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;
#[cfg(test)]
use std::ptr::addr_of_mut;
#[cfg(test)]
use std::sync::atomic::AtomicUsize;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0022_c410,
        game_module_draw_bezier_patch,
        "GameModule_DrawBezierPatch",
    ),
    (0x0022_cce8, section_man_ctor, "SectionMan_ctor"),
    (0x0022_cd18, section_man_dtor, "SectionMan_dtor"),
    (
        0x0022_d390,
        section_man_is_event_ready,
        "SectionMan_IsEventReady",
    ),
    (
        0x0022_d640,
        section_man_start_event_switch,
        "SectionMan_StartEventSwitch",
    ),
    (
        0x0022_de58,
        section_man_request_sky,
        "SectionMan_RequestSky",
    ),
    (
        0x0022_df50,
        section_man_begin_event_switch,
        "SectionMan_BeginEventSwitch",
    ),
    (
        0x0022_dff0,
        section_man_finish_event_switch,
        "SectionMan_FinishEventSwitch",
    ),
    (
        0x0022_e078,
        section_man_is_section_loaded,
        "SectionMan_IsSectionLoaded",
    ),
    (
        0x0022_e098,
        section_man_get_event_world_section,
        "SectionMan_GetEventWorldSection",
    ),
    (
        0x0022_e0c8,
        section_man_get_world_section_index,
        "SectionMan_GetWorldSectionIndex",
    ),
    (
        0x0022_e360,
        section_man_dequeue_section,
        "SectionMan_DequeueSection",
    ),
    (
        0x0022_c1b0,
        game_module_begin_bezier_pass,
        "GameModule_BeginBezierPass",
    ),
    (
        0x0022_c708,
        game_module_render_fog_particles,
        "GameModule_RenderFogParticles",
    ),
    (
        0x0022_c790,
        game_module_render_lens_flares,
        "GameModule_RenderLensFlares",
    ),
    (0x0022_d8d8, section_man_update, "SectionMan_Update"),
    (
        0x0022_d598,
        section_man_request_unload,
        "SectionMan_RequestUnload",
    ),
    (
        0x0022_d278,
        section_man_is_event_loaded,
        "SectionMan_IsEventLoaded",
    ),
    (
        0x0022_d478,
        section_man_request_activate,
        "SectionMan_RequestActivate",
    ),
    (0x0022_d088, section_man_load_event, "SectionMan_LoadEvent"),
    (0x0022_cd40, section_man_init, "SectionMan_Init"),
    (0x0022_de98, section_man_apply_sky, "SectionMan_ApplySky"),
    (
        0x0022_d668,
        oc436_section_man_advance_event_switch_sync,
        "SectionMan_AdvanceEventSwitchSync",
    ),
    (0x0022_c830, game_module_render_ui, "GameModule_RenderUI"),
    (
        0x0022_dfd0,
        section_man_is_event_switch_overlay_done,
        "SectionMan_IsEventSwitchOverlayDone",
    ),
    (
        0x0022_e118,
        section_man_reload_event_and_restart,
        "SectionMan_ReloadEventAndRestart",
    ),
    (
        0x0022_cea8,
        oc681_section_man_request_event,
        "SectionMan_RequestEvent",
    ),
    (
        0x0022_d3d8,
        section_man_request_load,
        "SectionMan_RequestLoad",
    ),
    (
        0x0022_d6c8,
        section_man_on_event_trigger_2,
        "SectionMan_OnEventTrigger",
    ),
];

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
pub(crate) mod view {

    pub const STRIDE: u32 = 0x50;

    pub const EYE: u32 = 0x250;

    /// View box (min, max).
    pub const BOX: u32 = 0x260;

    /// Child-plane points for the two children.
    pub const LEFT_POINT: u32 = 0x260;

    pub const RIGHT_POINT: u32 = 0x270;

    pub const HULL_RANGE: u32 = 0x280;

    pub const HULL_DIST: u32 = 0x284;

    pub const NEAREST: u32 = 0x288;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::SECTION_MAN_INIT;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE_3;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SECTION_MAN_PUSH_FREE_NODE;

    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;

    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;

    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
}

/// `void SectionMan_StartEventSwitch(SectionMan *man, uint event)` (0x22d640):
/// if `event` is non-zero and no switch is already pending (`+0x1b4 == 0`),
/// record it at `+0x1b8` and set the pending word at `+0x1b4` to 1. `v0` is 1
/// on the non-zero path, untouched otherwise.
pub fn section_man_start_event_switch(c: &mut Cpu) {
    const PENDING: u32 = 0x1b4;
    const EVENT: u32 = 0x1b8;
    let man = Addr(abi::arg(c, 0));
    let event = c.g[A1][0];
    c.cycles += 2;
    if event != 0 {
        c.cycles += 3;
        let pending = man.off(PENDING).lw(c);
        c.g[V0][0] = 1;
        if pending == 0 {
            c.cycles += 2;
            man.off(EVENT).sw(c, event);
            man.off(PENDING).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool SectionMan_IsSectionLoaded(int index)` (0x22e078): true when the
/// load-state word of section `index` (record base 0x442168 + index*16, field
/// `+8`) is 1 or 2.
pub fn section_man_is_section_loaded(c: &mut Cpu) {
    const SECTIONS: u32 = 0x44_2168;
    const STATE: u32 = 0x8;
    c.cycles += 8;
    let idx = abi::arg(c, 1);
    c.g[A1][0] = sx(idx << 4);
    let base = SECTIONS.wrapping_add(idx << 4);
    c.g[V1][0] = sx(base);
    let state = Addr(base).off(STATE).lw(c);
    c.g[V0][0] = (sx((state as u32).wrapping_sub(1)) < 2) as u64;
    abi::ret(c);
}

/// `int SectionMan_GetWorldSectionIndex(int index)` (0x22e0c8): the world
/// section word of record `index` (record base 0x442168 + index*16, `+4`).
pub fn section_man_get_world_section_index(c: &mut Cpu) {
    const SECTIONS: u32 = 0x44_2168;
    const WORLD: u32 = 0x4;
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    let base = SECTIONS.wrapping_add(c.g[A1][0] as u32);
    c.g[V1][0] = sx(base);
    c.g[V0][0] = Addr(base).off(WORLD).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bezier patches.

/// `GameModule_DrawBezierPatch(Patch* patch, View* view, void* arg /*a2*/)`
/// (0x22c410): tessellation level per patch edge from the squared
/// distances between the patch centre (+0x10) and the four corner points
/// held in the view object (+0x170..0x19c): `k * (d_a + d_b) + bias` for
/// the edges (0,1), (2,3), (0,2), (1,3), clamped to [min, max] (patch
/// +0x20..0x2c) and mapped through a byte table (gp-0x6e0); then the
/// renderer's virtual 0xa8 draws the patch with the four levels and two
/// halfwords of the view (+0x1a0/+0x1a2).
pub fn game_module_draw_bezier_patch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        let frame = Frame::open(c, 0x50);
        c.g[T4][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0x48);
        c.g[T5][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[T0][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        let patch = r32(c, T5);
        let view = r32(c, T4);
        let lod_min = ldf(c, 21, p(patch, 0x28));
        // The patch centre (+0x10) against the view's four points.
        let cx = ldf(c, 8, p(patch, 0x10));
        let cy = ldf(c, 2, p(patch, 0x14));
        setf(c, 23, lod_min);
        let ax = ldf(c, 0, p(view, 0x170));
        let ay = ldf(c, 1, p(view, 0x174));
        let dx = cx - ax;
        let cz = ldf(c, 4, p(patch, 0x18));
        let dy = cy - ay;
        let az = ldf(c, 0, p(view, 0x178));
        let bx = ldf(c, 1, p(view, 0x17c));
        let by = ldf(c, 3, p(view, 0x180));
        let dz = cz - az;
        let d0x = dx * dx;
        let bz = ldf(c, 0, p(view, 0x184));
        let d0y = dy * dy;
        let ex = ldf(c, 9, p(view, 0x188));
        let dx = cx - bx;
        let ey = ldf(c, 10, p(view, 0x18c));
        let dy = cy - by;
        let ez = ldf(c, 11, p(view, 0x190));
        let d0z = dz * dz;
        let gx = ldf(c, 14, p(view, 0x194));
        let dz = cz - bz;
        let gy = ldf(c, 12, p(view, 0x198));
        let d1y = dy * dy;
        let gz = ldf(c, 15, p(view, 0x19c));
        let d1x = dx * dx;
        let k = ldf(c, 6, p(patch, 0x20));
        let dx = cx - ex;
        let bias = ldf(c, 7, p(patch, 0x24));
        let dy = cy - ey;
        let lod_max = ldf(c, 22, p(patch, 0x2c));
        let d1z = dz * dz;
        setf(c, 13, d1z);
        let dz = cz - ez;
        let d2x = dx * dx;
        let d2y = dy * dy;
        setf(c, 9, d2y);
        let dx = cx - gx;
        setf(c, 19, dx);
        let dy = cy - gy;
        setf(c, 18, dy);
        let d1 = d1x + d1y;
        let d2z = dz * dz;
        setf(c, 8, d2z);
        let d0 = d0x + d0y;
        setf(c, 16, d0y);
        setf(c, 17, d0z);
        let d3y = dy * dy;
        let dz = cz - gz;
        setf(c, 20, dz);
        let d3x = dx * dx;
        let d0 = d0 + d0z;
        let d1 = d1 + d1z;
        let d2 = d2x + d2y;
        let d3 = d3x + d3y;
        let d3z = dz * dz;
        setf(c, 4, d3z);
        let s01 = d0 + d1;
        let d2 = d2 + d2z;
        let d3 = d3 + d3z;
        let s01 = k * s01;
        let s02 = d0 + d2;
        setf(c, 5, s02);
        let s13 = d1 + d3;
        let s23 = d2 + d3;
        let e01 = bias + s01;
        let s13 = k * s13;
        setf(c, 3, s13);
        let s23 = k * s23;
        let s02k = k * s02;
        setf(c, 6, s02k);
        let in01 = le(c, lod_min, e01);
        let e13 = bias + s13;
        let e23 = bias + s23;
        let e02 = bias + s02k;
        setf(c, 10, e01);
        setf(c, 1, e13);
        setf(c, 0, e23);
        setf(c, 7, e02);
        setf(c, 2, d3);
        // Clamp each edge level to [min, max].
        if in01 {
            c.cycles += 1;
            setf(c, 23, e01.min(lod_max));
        }
        c.cycles += 4;
        let in23 = le(c, lod_min, e23);
        c.mov_s(10, 23);
        setf(c, 6, lod_min);
        if in23 {
            c.cycles += 1;
            setf(c, 6, e23.min(lod_max));
        }
        c.cycles += 4;
        let in02 = le(c, lod_min, e02);
        c.mov_s(0, 6);
        setf(c, 6, lod_min);
        if in02 {
            c.cycles += 1;
            setf(c, 6, e02.min(lod_max));
        }
        c.cycles += 4;
        let in13 = le(c, lod_min, e13);
        c.mov_s(7, 6);
        setf(c, 2, lod_min);
        if in13 {
            c.cycles += 1;
            setf(c, 2, e13.min(lod_max));
        }
        // Levels through the byte table; call the renderer.
        c.cycles += 41;
        c.cvt_w_s(1, 0);
        c.mfc1(A1, 1);
        c.g[V1][0] = sx(gp(c, -0x6e0));
        c.cvt_w_s(1, 10);
        c.mfc1(A0, 1);
        c.g[T3][0] = c.lh(p(view, 0x1a0));
        c.cvt_w_s(0, 7);
        c.mfc1(A2, 0);
        c.g[T2][0] = c.lh(p(view, 0x1a2));
        c.cvt_w_s(0, 2);
        c.mfc1(V0, 0);
        let table = c.g[V1][0];
        c.g[A1][0] = add32(c.g[A1][0], table);
        c.g[A0][0] = add32(c.g[A0][0], table);
        let sp = sp(c);
        let v = c.g[T3][0];
        c.sw(p(sp, 0x10), v);
        c.g[A3][0] = c.lbu(r32(c, A0));
        c.g[A2][0] = add32(c.g[A2][0], table);
        c.g[V0][0] = add32(c.g[V0][0], table);
        c.g[A0][0] = c.lbu(r32(c, A1));
        c.g[V1][0] = c.lbu(r32(c, V0));
        c.g[A1][0] = c.g[T4][0];
        c.g[T1][0] = c.lbu(r32(c, A2));
        let v = c.g[A3][0];
        c.sb(sp, v);
        c.g[A2][0] = c.g[SP][0];
        let v = c.g[A0][0];
        c.sb(p(sp, 1), v);
        c.g[A3][0] = sx(p(sp, 0x10));
        let v = c.g[V1][0];
        c.sb(p(sp, 3), v);
        let v = c.g[T2][0];
        c.sw(p(sp, 0x14), v);
        let v = c.g[T1][0];
        c.sb(p(sp, 2), v);
        c.g[T1][0] = c.lw(patch);
        c.g[V0][0] = c.lw(at(c, T1, 4));
        let vt = r32(c, V0);
        c.g[A0][0] = c.lh(p(vt, 0xa8));
        c.g[V1][0] = c.lw(p(vt, 0xac));
        c.g[A0][0] = add32(c.g[T1][0], c.g[A0][0]);
        let target = r32(c, V1);
        call_ptr(c, target, 0x0022_c604)?;
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_dtor(SectionMan* m, int flags)` (0x22cd18).
pub fn section_man_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0022_cd30))
}

// ---------------------------------------------------------------------------
// Game and section managers.

/// `SectionMan_ctor(p, game)` 0x22cce8: remember the game object at +0x1b0 and
/// initialise the section tables; returns `p`.
pub fn section_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        let game = c.g[A1][0];
        c.sw((this as u32).wrapping_add(0x1b0), game);
        callee::SECTION_MAN_INIT.call(c, 0x0022_cd00)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Section manager.

/// `int SectionMan_GetEventWorldSection(SectionMan* man, int event)`
/// (0x22e098): the world section id of an event, through the two global
/// tables at 0x442498 (event records, 0x28 bytes each: a section record
/// index at `+0x10`) and 0x442168 (section records, 16 bytes each, id at
/// `+4`). The `mult` the compiler left in clobbers `a1` with its own product
/// (`mult a1, v0` with `rd = a1`), so the table index is `event * 0x28`.
pub fn section_man_get_event_world_section(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x28;
    c.g[V1][0] = 0x440000;
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x2488));
    c.g[A0][0] = 0x440000;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2168));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = c.lw(p(r32(c, V1), 0x10));
    c.g[V0][0] = sx(r32(c, V0) << 4);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    c.g[V0][0] = c.lw(p(r32(c, A0), 4));
    abi::ret(c);
}

/// `void SectionMan_RequestSky(SectionMan* m, int v)` (0x22de58): switch the
/// requested sky when it changes, activate the section and mark it
/// "requested".
pub fn section_man_request_sky(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1ac);
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 5;
            Addr(p(this, 0x1ac)).sw(c, c.g[V1][0]);
            callee::SECTION_MAN_REQUEST_ACTIVATE.call(c, 0x0022_de80)?;
            c.g[V0][0] = 1;
            Addr(p(crate::abi::r32(c, S0), 0x1a8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Section manager, replay playback and the memory-card manager.

/// `bool SectionMan_IsEventReady(SectionMan* self)` (0x22d390): true when the
/// event is loaded and its ready flag (+0x1c0) is set.
pub fn section_man_is_event_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::SECTION_MAN_IS_EVENT_LOADED.call(c, 0x0022_d3ac)?;
        let not_loaded = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !not_loaded {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S0, 0x1c0));
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SectionMan_DequeueSection(self, node)` (0x22e360): unlink a section
/// node from its list or record it as the new head, push it free and count it.
pub fn section_man_dequeue_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A1)).off(4).lw(c);
            Addr(p(r32(c, S0), 0x19c)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(4).lw(c);
            Addr(p(r32(c, V1), 4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x0022_e398);
        c.g[A0][0] = c.g[S0][0];
        callee::SECTION_MAN_PUSH_FREE_NODE.call(c, 0x0022_e398)?;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(0x1a0).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        Addr(p(this, 0x1a0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_BeginEventSwitch(self)` (0x0022df50): request the section
/// state and overlay, then tell the interface to begin the switch.
pub fn section_man_begin_event_switch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0xb;
        c.g[A0][0] = Addr(at(c, S0, 0x1b0)).lw(c);
        callee::GAME_MODULE_REQUEST_STATE_2.call(c, 0x0022_df74)?;
        callee::CBE_GET_BE.call(c, 0x0022_df7c)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        callee::CBE_GET_INTERFACE.call(c, 0x0022_df88)?;
        c.g[A1][0] = Addr(at(c, S0, 0x1bc)).lw(c);
        c.g[V1][0] = sx(0x530000);
        c.g[A0][0] = 0x1;
        Addr(at(c, V1, 0x5c08)).sw(c, c.g[A1][0]);
        Addr(gp(c, -0x1f38)).sw(c, c.g[A0][0]);
        c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x8)).lh(c);
        c.g[A1][0] = Addr(at(c, V1, 0xc)).lw(c);
        c.g[A0][0] = add32_2(word(c, V0), word(c, A0));
        call_ptr(c, word(c, A1), 0x0022_dfb0)?;
        c.g[A0][0] = Addr(at(c, S0, 0x1b0)).lw(c);
        c.g[A1][0] = 0xa;
        callee::GAME_MODULE_REQUEST_OVERLAY_STATE.call(c, 0x0022_dfbc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_FinishEventSwitch(SectionMan* self)` (0x22dff0):
/// reselect peak lighting for the track, drop the pending event switch and
/// request the game state change.
pub fn section_man_finish_event_switch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0022_e004)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0022_e010)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_GET_TRACK_INFO.call(c, 0x0022_e018)?;
        c.g[A1][0] = lw_at(c, c.g[V0][0], 0);
        let this = c.g[S0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::SECTION_SELECT_PEAK_LIGHTING.call(c, 0x0022_e024)?;
        c.g[V0][0] = lw_at(c, this, 0x1cc);
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 9;
            c.sw(at_2(this as u32, 0x1cc), 0);
            let g = gpa(c, -0x848);
            c.g[V0][0] = lw_at(c, g as u64, 0);
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0x84);
            c.g[A2][0] = lw_at(c, c.g[V1][0], 0xc);
            c.g[V0][0] = lw_at(c, c.g[A2][0], 0xcc);
            c.g[A0][0] = lh_at(c, c.g[V0][0], 0x28);
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0x2c);
            let func = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at_2(c.g[A2][0] as u32, c.g[A0][0] as u32));
            call_ptr(c, func, 0x0022_e054)?;
        }
        c.cycles += 9;
        c.g[A0][0] = lw_at(c, this, 0x1b0);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, callee::GAME_MODULE_REQUEST_STATE_3, 0x0022_e068)?;
        c.lq(16, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

pub(crate) mod bezier_pass {

    /// gp+0x1158: the camera; gp-0x854: the renderer (`+0x10d8` its
    /// vtable, slot 0x118 the view matrix).
    pub const GP_CAMERA: u32 = 0x1158;

    pub const GP_RENDERER: u32 = 0x854;

    pub const RENDERER_VT: u32 = 0x10d8;

    pub const SLOT_VIEW_MATRIX: u32 = 0x118;

    /// Context: camera, eye position (view space), depth fit (k, c),
    /// near/far (1, 3).
    pub const CAMERA: u32 = 0x0;

    pub const EYE: u32 = 0x10;

    pub const DEPTH_K: u32 = 0x20;

    pub const DEPTH_C: u32 = 0x24;

    pub const NEAR: u32 = 0x28;

    pub const FAR: u32 = 0x2c;

    /// Camera (vtable at +4): near plane, far plane, begin pass.
    pub const SLOT_NEAR: u32 = 0x40;

    pub const SLOT_FAR: u32 = 0x48;

    pub const SLOT_BEGIN: u32 = 0x88;
}

// ---------------------------------------------------------------------------
// GameModule_BeginBezierPass (0x22c1b0, src/main/game.cpp).

/// `this(a1)->vtable(vt_reg)[slot]` called with `a0 = a1 + delta`, the
/// method address in `fn_reg` (the compiled register choice).
pub(crate) fn vcall_a1(
    c: &mut Cpu,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret_to: u32,
) -> crate::call::Flow {
    let this = r32(c, A1);
    c.g[A0][0] = c.lh(at(c, vt_reg, slot));
    c.g[fn_reg][0] = c.lw(at(c, vt_reg, slot + 4));
    let method = r32(c, fn_reg);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret_to)
}

/// `Ctx* GameModule_BeginBezierPass(Ctx* ctx)` (0x22c1b0): take the
/// camera, compute the eye position (-translation of the view matrix,
/// transformed by the view matrix, w = 1), set near/far = 1/3 and fit the
/// depth mapping `k = (near - far) / (2 f^2 - 2 n^2)`, `c = far - k 2 n^2`
/// from the camera's near (n) and far (f) planes, then begin the camera's
/// pass. Returns ctx. One 130-cycle block; frame 0x60 (s0, f20).
pub fn game_module_begin_bezier_pass(c: &mut Cpu) {
    use bezier_pass::*;
    guest(c, |c| {
        c.cycles += 130;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_CAMERA));
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x50);
        let ctx = r32(c, S0);
        c.g[A1][0] = c.lw(gp_lit(c, GP_RENDERER));
        c.sw(ctx.wrapping_add(CAMERA), c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, A1, RENDERER_VT));
        vcall_a1(c, V1, V0, SLOT_VIEW_MATRIX, 0x0022_c1e4)?;
        // -translation (VU0 scale by -1).
        c.lq(3, at(c, V0, 0x30));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        let s = sp(c);
        c.sq(3, s.wrapping_add(0x10));
        c.mfc1(A0, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x20));
        c.lq(2, s.wrapping_add(0x20));
        c.g[A1][0] = c.lw(gp_lit(c, GP_RENDERER));
        c.sq(2, ctx.wrapping_add(EYE));
        c.sq(2, s);
        c.g[V1][0] = c.lw(at(c, A1, RENDERER_VT));
        vcall_a1(c, V1, V0, SLOT_VIEW_MATRIX, 0x0022_c22c)?;
        // eye = M * eye (row-major dot products, the compiled order).
        let m = r32(c, V0);
        c.lwc1(1, ctx.wrapping_add(EYE));
        c.lwc1(5, m);
        c.lwc1(8, m.wrapping_add(0x4));
        c.lwc1(4, m.wrapping_add(0x10));
        c.mul_s(5, 1, 5);
        c.lwc1(7, m.wrapping_add(0x14));
        c.lwc1(3, m.wrapping_add(0x20));
        c.mul_s(4, 1, 4);
        c.lwc1(6, m.wrapping_add(0x34));
        c.lwc1(0, ctx.wrapping_add(EYE + 4));
        c.mul_s(3, 1, 3);
        c.lwc1(12, m.wrapping_add(0x24));
        c.lwc1(2, m.wrapping_add(0x30));
        c.mul_s(6, 0, 6);
        c.mul_s(8, 0, 8);
        c.lwc1(10, m.wrapping_add(0x8));
        c.mul_s(7, 0, 7);
        c.lwc1(11, m.wrapping_add(0x18));
        c.mul_s(0, 0, 12);
        c.lwc1(9, m.wrapping_add(0x38));
        c.mul_s(1, 1, 2);
        c.lwc1(15, m.wrapping_add(0x28));
        c.lwc1(2, ctx.wrapping_add(EYE + 8));
        c.add_s(5, 5, 8);
        c.add_s(4, 4, 7);
        c.lwc1(12, m.wrapping_add(0x3c));
        c.mul_s(9, 2, 9);
        c.lwc1(8, m.wrapping_add(0xc));
        c.mul_s(10, 2, 10);
        c.lwc1(7, m.wrapping_add(0x1c));
        c.mul_s(11, 2, 11);
        c.lwc1(14, m.wrapping_add(0x2c));
        c.add_s(3, 3, 0);
        lit(c, 13, 0x3f80_0000);
        c.lwc1(0, ctx.wrapping_add(EYE + 0xc));
        c.mul_s(2, 2, 15);
        c.add_s(1, 1, 6);
        c.swc1(13, ctx.wrapping_add(NEAR));
        c.mul_s(12, 0, 12);
        lit(c, 6, 0x4040_0000);
        c.mul_s(8, 0, 8);
        c.g[A1][0] = c.lw(ctx.wrapping_add(CAMERA));
        c.mul_s(7, 0, 7);
        c.swc1(6, ctx.wrapping_add(FAR));
        c.add_s(5, 5, 10);
        c.add_s(4, 4, 11);
        c.add_s(1, 1, 9);
        c.mul_s(0, 0, 14);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 12);
        c.add_s(5, 5, 8);
        c.add_s(4, 4, 7);
        c.add_s(3, 3, 0);
        let s = sp(c);
        c.swc1(1, s.wrapping_add(0xc));
        c.swc1(5, s);
        c.swc1(4, s.wrapping_add(0x4));
        c.swc1(3, s.wrapping_add(0x8));
        c.lq(2, s);
        c.sq(2, ctx.wrapping_add(EYE));
        c.swc1(13, ctx.wrapping_add(EYE + 0xc));
        // The camera's near and far planes.
        c.g[V0][0] = c.lw(at(c, A1, 4));
        vcall_a1(c, V0, V1, SLOT_NEAR, 0x0022_c338)?;
        c.g[A1][0] = c.lw(ctx.wrapping_add(CAMERA));
        c.mov_s(20, 0);
        c.g[V0][0] = c.lw(at(c, A1, 4));
        vcall_a1(c, V0, V1, SLOT_FAR, 0x0022_c354)?;
        c.mul_s(3, 20, 20);
        c.lwc1(2, ctx.wrapping_add(FAR));
        c.mul_s(0, 0, 0);
        c.lwc1(1, ctx.wrapping_add(NEAR));
        c.g[A1][0] = c.lw(ctx.wrapping_add(CAMERA));
        c.add_s(20, 3, 3);
        c.add_s(0, 0, 0);
        c.sub_s(1, 1, 2);
        c.sub_s(0, 0, 20);
        c.div_s(1, 1, 0);
        c.mul_s(0, 1, 20);
        c.swc1(1, ctx.wrapping_add(DEPTH_K));
        c.sub_s(2, 2, 0);
        c.swc1(2, ctx.wrapping_add(DEPTH_C));
        c.g[V0][0] = c.lw(at(c, A1, 4));
        vcall_a1(c, V0, V1, SLOT_BEGIN, 0x0022_c3a0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod render_lens_flares {

    /// `LensFX_ClearSources(fx, view)` (table call).
    pub const CLEAR_SOURCES: u32 = 0x002e_2f98;

    /// `LensFX_AddSources(fx, sources, count, kind, view)` (table call).
    pub const ADD_SOURCES: u32 = 0x002e_2ff8;

    /// `LensFX_Render(fx, view)` (table call).
    pub const RENDER: u32 = 0x002e_30d0;

    /// Game module: its lens FX.
    pub const LENS_FX: u32 = 0x1c;

    /// Lens FX: "rendering" flag (set around the render).
    pub const RENDERING: u32 = 0x6494;

    /// World: two flare source lists `{count, sources[]}` (kinds 0 and 2).
    pub const SOURCES0_COUNT: u32 = 0x7bc0;

    pub const SOURCES0: u32 = 0x7bc4;

    pub const SOURCES2_COUNT: u32 = 0x7fc4;

    pub const SOURCES2: u32 = 0x7fc8;
}

// ---------------------------------------------- GameModule_RenderFogParticles

/// `GameModule`: the fog particle manager (+0x18; vtable at +0x4, slot 2
/// `renderFog` at +0x10/+0x14).
pub(crate) const GM_FOG_MAN: u32 = 0x18;

pub(crate) const FOG_VTABLE: u32 = 0x4;

pub(crate) const FOG_RENDER_SLOT: u32 = 0x10;

/// World: the two fog volume lists (count, then the volumes): kind 0 at
/// +0x78b4/+0x78b8, kind 2 at +0x7abc/+0x7ac0.
pub(crate) const WORLD_FOG_LISTS: [(u32, u32, u64, u32); 2] = [
    (0x78b4, 0x78b8, 0, 0x0022_c748),
    (0x7abc, 0x7ac0, 2, 0x0022_c770),
];

/// `void GameModule_RenderFogParticles(GameModule* self, World* world)`
/// (0x22c708): hand each non-empty fog volume list of the world to the fog
/// particle manager (`fogMan->renderFog(volumes, count, kind)`, kinds 0 and
/// 2), then let it finish the frame (`cPS2FogParticleMan` 0x2dbf98).
/// Frame 0x30 (s0 = world, s1 = self).
pub fn game_module_render_fog_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        for (i, &(count_off, list_off, kind, ret)) in WORLD_FOG_LISTS.iter().enumerate() {
            if i > 0 {
                c.cycles += 3;
            }
            let world = r32(c, S0);
            let count = Addr(world.wrapping_add(count_off)).lw(c);
            c.g[A2][0] = count;
            c.g[A1][0] = sx(world.wrapping_add(list_off));
            if count != 0 {
                c.cycles += 7;
                let man = Addr(r32(c, S1).wrapping_add(GM_FOG_MAN)).lw(c);
                c.g[T0][0] = man;
                c.g[A3][0] = kind;
                let vt = Addr((man as u32).wrapping_add(FOG_VTABLE)).lw(c);
                c.g[V0][0] = vt;
                let delta = Addr((vt as u32).wrapping_add(FOG_RENDER_SLOT)).lh(c);
                let method = Addr((vt as u32).wrapping_add(FOG_RENDER_SLOT + 4)).lw(c);
                c.g[V1][0] = method;
                c.g[A0][0] = sx((man as u32).wrapping_add(delta as u32));
                call_ptr(c, method as u32, ret)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(GM_FOG_MAN)).lw(c);
        call_ptr(c, 0x002d_bf98, 0x0022_c778)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModule_RenderLensFlares(GameModule* this, World* world, int
/// view)` (0x22c790): rebuild the lens FX's flare sources for `view` from
/// the world's two source lists (kinds 0 and 2) and render the flares, with
/// the lens FX's "rendering" flag set meanwhile.
pub fn game_module_render_lens_flares(c: &mut Cpu) {
    use render_lens_flares::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0]; // world
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // this
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0]; // view
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        Addr(r32(c, V0).wrapping_add(RENDERING)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        call_ptr(c, CLEAR_SOURCES, 0x0022_c7c8)?;
        // The kind-0 sources.
        let world = r32(c, S0);
        c.g[A2][0] = Addr(world.wrapping_add(SOURCES0_COUNT)).lw(c);
        c.g[A1][0] = sx(world.wrapping_add(SOURCES0));
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
            c.g[A3][0] = 0;
            c.g[T0][0] = c.g[S2][0];
            call_ptr(c, ADD_SOURCES, 0x0022_c7e4)?;
        }
        // The kind-2 sources.
        c.cycles += 3;
        let world = r32(c, S0);
        c.g[A2][0] = Addr(world.wrapping_add(SOURCES2_COUNT)).lw(c);
        c.g[A1][0] = sx(world.wrapping_add(SOURCES2));
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
            c.g[A3][0] = 0x2;
            c.g[T0][0] = c.g[S2][0];
            call_ptr(c, ADD_SOURCES, 0x0022_c800)?;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, RENDER, 0x0022_c80c)?;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(RENDERING)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod section_man_update_layout {

    /// A sky swap is pending (`SectionMan_ApplySky` runs while set).
    pub const SKY_PENDING: u32 = 0x1a8;

    /// The owning game module (`+0x10` -> `*` -> the world view).
    pub const GAME: u32 = 0x1b0;

    /// The event-switch state machine (0 idle .. 4 waiting for the overlay).
    pub const SWITCH: u32 = 0x1b4;

    pub const SWITCH_AUX: u32 = 0x1b8;

    /// Streaming enabled (picks the stream rate when nothing changed).
    pub const STREAM_ON: u32 = 0x1c0;

    /// The stream-rate class (index into the rate table).
    pub const RATE: u32 = 0x1c4;

    /// Sections settled: 1 when nothing forces the slow stream rate.
    pub const READY: u32 = 0x1d0;

    /// The world view: active and visible flags.
    pub const WV_ACTIVE: u32 = 0x250;

    pub const WV_VISIBLE: u32 = 0x2a0;

    /// The world view's stream manager.
    pub const WV_STREAM_MAN: u32 = 0x300;

    /// The section event table (16 bytes per event: `+4` world section,
    /// `+8` state, `+0xc` sky index).
    pub const EVENTS: u32 = 0x0044_2168;

    /// Per-sky request priorities (floats).
    pub const SKY_PRIORITY: u32 = 0x0044_c208;

    /// Stream rates per rate class.
    pub const RATES: u32 = 0x0044_2150;

    /// Path prefixes ("data/worlds/_", "data/worlds/") and the "%s%s" format.
    pub const PREFIX_MULTI: u32 = 0x0047_b470;

    pub const PREFIX_SINGLE: u32 = 0x0047_b480;

    pub const PATH_FORMAT: u32 = 0x004a_2a18;

    /// The event-state `switch` jump table (states 0..=8) and its targets.
    pub const STATE_TABLE: u32 = 0x0047_b490;

    pub const STATE_CASES: [u32; 9] = [
        0x0022_db64,
        0x0022_db00,
        0x0022_db2c,
        0x0022_dae4,
        0x0022_daf0,
        0x0022_d9e8,
        0x0022_da2c,
        0x0022_d9b8,
        0x0022_da78,
    ];

    /// The switch-state jump table (states 0..=4) and its targets.
    pub const SWITCH_TABLE: u32 = 0x0047_b4c0;

    pub const SWITCH_CASES: [u32; 5] = [
        0x0022_ddd8,
        0x0022_dd80,
        0x0022_ddd8,
        0x0022_dd94,
        0x0022_ddac,
    ];
}

pub(crate) const SU_APPLY_SKY: Callee = Callee {
    addr: 0x0022_de98,
    f: ee::s05::SectionMan_ApplySky,
};

pub(crate) const SU_NEXT_QUEUED: Callee = Callee {
    addr: 0x0022_e348,
    f: ee::s05::SectionMan_NextQueued,
};

pub(crate) const SU_DEQUEUE: Callee = Callee {
    addr: 0x0022_e360,
    f: ee::s05::SectionMan_DequeueSection,
};

pub(crate) const SU_REQUEST_SKY: Callee = Callee {
    addr: 0x0022_de58,
    f: ee::s05::SectionMan_RequestSky,
};

pub(crate) const SU_BEGIN_SWITCH: Callee = Callee {
    addr: 0x0022_df50,
    f: ee::s05::SectionMan_BeginEventSwitch,
};

pub(crate) const SU_OVERLAY_DONE: Callee = Callee {
    addr: 0x0022_dfd0,
    f: ee::s05::SectionMan_IsEventSwitchOverlayDone,
};

pub(crate) const SU_FINISH_SWITCH: Callee = Callee {
    addr: 0x0022_dff0,
    f: ee::s05::SectionMan_FinishEventSwitch,
};

/// `cBE_getBE`, `BELibrary_GetInterface`, the world cache streamer queries
/// and the game module hooks (called through the function table).
pub(crate) const SU_GET_BE: u32 = 0x0014_dc80;

pub(crate) const SU_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const SU_SECTION_UNLOADED: u32 = 0x003a_9890;

pub(crate) const SU_RELEASE_SECTION: u32 = 0x003a_9958;

pub(crate) const SU_SECTION_RESIDENT: u32 = 0x003a_9858;

pub(crate) const SU_ACTIVATE_SECTION: u32 = 0x003a_99d8;

pub(crate) const SU_SECTION_COUNT: u32 = 0x003a_9800;

pub(crate) const SU_REQUEST_SECTION: u32 = 0x003a_98c8;

pub(crate) const SU_ON_SECTION_UNLOAD: u32 = 0x0023_0360;

pub(crate) const SU_INVALIDATE_FOCUS: u32 = 0x0023_0338;

pub(crate) const SU_SECTION_FIELD10: u32 = 0x0014_4d50;

pub(crate) const SU_SECTION_ENTRY: u32 = 0x0014_4d38;

pub(crate) const SU_SPRINTF: u32 = 0x0041_7828;

pub(crate) const SU_SET_RATE: u32 = 0x003a_7058;

/// The register dance before every world-cache call: `v0 = self->game`,
/// `a1 = s0` (the world section), `v1 = game->+0x10`, `a0 = *v1 + 0x10`.
pub(crate) fn section_man_update_cache_args(c: &mut Cpu) {
    use section_man_update_layout::GAME;
    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(GAME)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
}

/// `this` (s2, re-read after every call).
pub(crate) fn section_man_update_this(c: &Cpu) -> Addr {
    Addr(r32(c, S2))
}

/// `void cSectionMan::update(cSectionMan* this)` (0x22d8d8): the per-frame
/// world-section streaming state machine.
///
/// Applies a pending sky swap, then walks the queued section events: waits
/// for unloads (7) and releases (5, then `GameModule_OnSectionUnload`),
/// promotes loaded sections (6 -> 1, 8 -> 2 and activates them), collects
/// pending loads (3, 4), and retires finished events (0, 1, 2: requesting
/// their sky, dequeuing the node). When nothing is busy it requests the
/// pending loads from the streamer (with the event's sky priority and its
/// path built by `sprintf`). Then it advances the event-switch state
/// (`+0x1b4`), picks the stream-rate class (`+0x1c4`) and sets the stream
/// manager's rate from the rate table.
pub fn section_man_update(c: &mut Cpu) {
    use section_man_update_layout::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x1b0);
        frame.save(c, S2, 0x180);
        frame.save(c, S0, 0x1a0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x190);
        frame.save(c, S3, 0x170);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save(c, S6, 0x140);
        frame.save(c, S7, 0x130);
        frame.save(c, FP, 0x120);
        frame.save_ra(c, 0x110);
        c.g[V0][0] = section_man_update_this(c).off(SKY_PENDING).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            SU_APPLY_SKY.call(c, 0x0022_d91c)?;
        }
        c.g[A0][0] = c.g[S2][0];

        // The first queued event.
        c.cycles += 5;
        c.g[A1][0] = 0;
        SU_NEXT_QUEUED.call(c, 0x0022_d928)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[S6][0] = 0;
        if c.g[S4][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = section_man_update_this(c).off(SWITCH).lw(c);
            frame.restore(c, S0, 0x1a0);
            if c.g[V0][0] == 0 {
                // Nothing queued and no event switch: done.
                section_man_update_epilogue(c, frame);
                return Ok(());
            }
        }

        c.cycles += 10;
        c.g[S7][0] = 0; // busy
        call_ptr(c, SU_GET_BE, 0x0022_d948)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, SU_GET_INTERFACE, 0x0022_d954)?;
        c.g[FP][0] = 0; // changed
        let iface_slot = Addr(frame.at(c, 0x100));
        let force_off = Addr(frame.at(c, 0x104));
        let pending = Addr(frame.at(c, 0x108));
        let prefix_slot = Addr(frame.at(c, 0x10c));
        let iface = c.g[V0][0];
        iface_slot.sw(c, iface);
        force_off.sw(c, 0);
        pending.sw(c, 0);

        if c.g[S4][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = 0x0044_0000;
            loop {
                // Loop head 0x22d970: s4 = node, s6 = previous node.
                c.cycles += 11;
                c.g[S1][0] = Addr(r32(c, S4)).lw(c); // event index
                c.g[S3][0] = c.g[V0][0];
                c.g[S5][0] = 0; // remove
                c.g[V1][0] = sx(r32(c, S1) << 4);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x2168));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = c.g[A0][0];
                let event = r32(c, A0);
                c.g[A0][0] = Addr(event.wrapping_add(8)).lw(c);
                c.g[V1][0] = (c.g[A0][0] < 9) as u64;
                c.g[S0][0] = Addr(event.wrapping_add(4)).lw(c); // world section
                if c.g[A0][0] < 9 {
                    c.cycles += 7;
                    let state = c.g[A0][0] as u32;
                    c.g[V0][0] = sx(STATE_TABLE);
                    c.g[V1][0] = sx((state << 2).wrapping_add(STATE_TABLE));
                    c.g[A0][0] = sx(STATE_CASES[state as usize]);
                    match state {
                        7 => {
                            // Unloading: done once the streamer dropped it.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_SECTION_UNLOADED, 0x0022_d9d0)?;
                            let unloaded = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                            if unloaded {
                                c.cycles += 4;
                                c.g[V1][0] = sx(r32(c, S1) << 4);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                Addr(r32(c, V0).wrapping_add(8)).sw(c, 0);
                            } else {
                                c.cycles += 2;
                                c.g[S7][0] = 1;
                            }
                        }
                        5 => {
                            // Releasing: on success the state becomes 7 and
                            // the game module hears of the unload.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_RELEASE_SECTION, 0x0022_da00)?;
                            let released = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                            if released {
                                c.cycles += 7;
                                c.g[V1][0] = sx(r32(c, S1) << 4);
                                c.g[A0][0] = 7;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                Addr(r32(c, V0).wrapping_add(8)).sw(c, 7);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                                call_ptr(c, SU_ON_SECTION_UNLOAD, 0x0022_da24)?;
                            }
                            c.cycles += 2;
                            c.g[S7][0] = 1;
                        }
                        6 => {
                            // Loading: once resident the state becomes 1.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_SECTION_RESIDENT, 0x0022_da44)?;
                            let resident = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                            if resident {
                                c.cycles += 9;
                                c.g[V1][0] = sx(r32(c, S1) << 4);
                                c.g[A0][0] = 1;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                Addr(r32(c, V0).wrapping_add(8)).sw(c, 1);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                                call_ptr(c, SU_INVALIDATE_FOCUS, 0x0022_da68)?;
                            } else {
                                c.cycles += 2;
                                c.g[FP][0] = 1;
                            }
                        }
                        8 => {
                            // Loading for an event switch: once resident the
                            // state becomes 2 and the section is activated.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_SECTION_RESIDENT, 0x0022_da90)?;
                            let resident = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S1) << 4);
                            if resident {
                                c.cycles += 15;
                                c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                c.g[V1][0] = 2;
                                Addr(r32(c, A0).wrapping_add(8)).sw(c, 2);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                                call_ptr(c, SU_INVALIDATE_FOCUS, 0x0022_dab4)?;
                                c.g[V1][0] = section_man_update_this(c).off(GAME).lw(c);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lw(c);
                                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                                call_ptr(c, SU_ACTIVATE_SECTION, 0x0022_dacc)?;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 1;
                                c.g[FP][0] = 1;
                                force_off.sw(c, 1);
                            }
                        }
                        3 => {
                            // Waiting to be requested.
                            c.cycles += 3;
                            c.g[V1][0] = 1;
                            pending.sw(c, 1);
                        }
                        4 => {
                            // Waiting to be requested (event switch).
                            c.cycles += 4;
                            c.g[V0][0] = 1;
                            pending.sw(c, 1);
                            force_off.sw(c, 1);
                        }
                        1 => {
                            // Active: request its sky, retire the event.
                            c.cycles += 6;
                            c.g[A0][0] = iface_slot.lw(c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, SU_SECTION_FIELD10, 0x0022_db0c)?;
                            c.g[V1][0] = 4;
                            c.g[S5][0] = 1;
                            if c.g[V0][0] == 4 {
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S2][0];
                                SU_REQUEST_SKY.call(c, 0x0022_db24)?;
                                c.g[S5][0] = 1;
                            }
                        }
                        2 => {
                            // Active after a switch: request and apply its
                            // sky, retire the event.
                            c.cycles += 6;
                            c.g[A0][0] = iface_slot.lw(c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, SU_SECTION_FIELD10, 0x0022_db38)?;
                            c.g[V1][0] = 4;
                            if c.g[V0][0] == 4 {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S2][0];
                                SU_REQUEST_SKY.call(c, 0x0022_db50)?;
                            }
                            c.g[V0][0] = section_man_update_this(c).off(SKY_PENDING).lw(c);
                            c.cycles += 2;
                            c.g[S5][0] = 1;
                            if c.g[V0][0] != 0 {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S2][0];
                                SU_APPLY_SKY.call(c, 0x0022_db64)?;
                                c.cycles += 1;
                                c.g[S5][0] = 1;
                            }
                        }
                        _ => {
                            // 0: finished, retire the event.
                            c.cycles += 1;
                            c.g[S5][0] = 1;
                        }
                    }
                }

                // 0x22db68: dequeue a retired node (the walk continues from
                // the previous one).
                c.cycles += 2;
                c.g[A2][0] = c.g[S4][0];
                if c.g[S5][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[S4][0] = c.g[S6][0];
                    SU_DEQUEUE.call(c, 0x0022_db80)?;
                }
                c.cycles += 7;
                c.g[S6][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S6][0];
                SU_NEXT_QUEUED.call(c, 0x0022_db90)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[V0][0] = 0x0044_0000;
                if c.g[S4][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_d970);
                }
            }
        }

        // 0x22db9c: request the pending loads once nothing is busy.
        c.cycles += 2;
        c.g[V1][0] = pending.lw(c);
        let mut request = c.g[S7][0] == 0;
        if request {
            c.cycles += 2;
            request = c.g[V1][0] != 0;
        }
        if request {
            c.cycles += 12;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            SU_NEXT_QUEUED.call(c, 0x0022_dbb8)?;
            c.g[V1][0] = section_man_update_this(c).off(GAME).lw(c);
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            call_ptr(c, SU_SECTION_COUNT, 0x0022_dbd0)?;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(PREFIX_SINGLE);
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx(PREFIX_MULTI);
            }
            c.cycles += 2;
            let prefix = c.g[V0][0];
            prefix_slot.sw(c, prefix);
            if c.g[S4][0] != 0 {
                c.cycles += 7;
                c.g[V0][0] = 0x0044_0000;
                c.g[V1][0] = 0x0045_0000;
                c.g[S7][0] = sx(EVENTS);
                c.g[S6][0] = sx(SKY_PRIORITY);
                c.g[S5][0] = sx(EVENTS + 0xc);
                c.g[A1][0] = Addr(r32(c, S4)).lw(c);
                loop {
                    // Loop head 0x22dc10: s4 = node, a1 = its event index.
                    c.cycles += 19;
                    c.g[V1][0] = sx(EVENTS + 4);
                    c.g[A0][0] = iface_slot.lw(c);
                    c.g[S0][0] = sx(r32(c, A1) << 4);
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                    c.g[S3][0] = Addr(r32(c, V0)).lw(c); // world section
                    call_ptr(c, SU_SECTION_ENTRY, 0x0022_dc2c)?;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A2][0] = prefix_slot.lw(c);
                    c.g[V0][0] = 0x004a_0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx(PATH_FORMAT);
                    call_ptr(c, SU_SPRINTF, 0x0022_dc44)?;
                    c.g[V1][0] = sx(r32(c, S7).wrapping_add(8));
                    c.g[V0][0] = 3;
                    c.g[S1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = Addr(r32(c, S1)).lw(c); // state
                    c.g[V0][0] = 4;
                    let state = c.g[V1][0];
                    let mut next_a1_set = false;
                    if state == 3 || state == 4 {
                        if state == 4 {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S4][0];
                        }
                        let ret = if state == 3 { 0x0022_dc88 } else { 0x0022_dcd0 };
                        c.cycles += 13;
                        c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                        c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c); // sky index
                        c.g[A1][0] = c.g[S3][0];
                        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                        c.lwc1(12, r32(c, V0));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                        call_ptr(c, SU_REQUEST_SECTION, ret)?;
                        let requested = c.g[V0][0] != 0;
                        if state == 3 {
                            c.g[V0][0] = 6;
                            if requested {
                                c.cycles += 3;
                                c.g[FP][0] = 1;
                                Addr(r32(c, S1)).sw(c, 6);
                            }
                        } else {
                            c.g[V0][0] = 8;
                            if requested {
                                c.cycles += 4;
                                c.g[V1][0] = 1;
                                Addr(r32(c, S1)).sw(c, 8);
                                c.g[FP][0] = 1;
                                force_off.sw(c, 1);
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S4][0];
                        next_a1_set = true;
                    }
                    if !next_a1_set {
                        // Neither 3 nor 4 (0x22dc9c: the delay slot sets a1).
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S4][0];
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    SU_NEXT_QUEUED.call(c, 0x0022_dcf4)?;
                    c.g[S4][0] = c.g[V0][0];
                    if c.g[S4][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = Addr(r32(c, S4)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_dc10);
                    }
                }
            }
        }

        // 0x22dd00: the stream-rate class.
        c.cycles += 2;
        c.g[V0][0] = 2;
        if c.g[FP][0] != 0 {
            c.cycles += 2;
            section_man_update_this(c).off(RATE).sw(c, 2);
        } else {
            c.cycles += 3;
            c.g[V0][0] = section_man_update_this(c).off(STREAM_ON).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 3;
                section_man_update_this(c).off(RATE).sw(c, 3);
            } else {
                c.g[V0][0] = section_man_update_this(c).off(GAME).lw(c);
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x10)).lw(c);
                c.g[V1][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WV_ACTIVE)).lw(c);
                if c.g[V0][0] == 0 {
                    section_man_update_this(c).off(RATE).sw(c, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WV_VISIBLE)).lw(c);
                    let visible = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if visible {
                        c.cycles += 2;
                        section_man_update_this(c).off(RATE).sw(c, 1);
                    } else {
                        c.cycles += 1;
                        section_man_update_this(c).off(RATE).sw(c, 0);
                    }
                }
            }
        }

        // 0x22dd54: the event-switch state machine (s0 = hold).
        c.cycles += 4;
        c.g[V1][0] = section_man_update_this(c).off(SWITCH).lw(c);
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        c.g[S0][0] = 0;
        if c.g[V1][0] < 5 {
            c.cycles += 7;
            let sw = c.g[V1][0] as u32;
            c.g[V0][0] = sx(SWITCH_TABLE);
            c.g[V1][0] = sx((sw << 2).wrapping_add(SWITCH_TABLE));
            c.g[A0][0] = sx(SWITCH_CASES[sw as usize]);
            match sw {
                1 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    SU_BEGIN_SWITCH.call(c, 0x0022_dd88)?;
                    c.g[V0][0] = 2;
                    section_man_update_this(c).off(SWITCH).sw(c, 2);
                }
                3 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = 1;
                    SU_BEGIN_SWITCH.call(c, 0x0022_dda0)?;
                    c.g[V0][0] = 4;
                    section_man_update_this(c).off(SWITCH).sw(c, 4);
                }
                4 => {
                    c.cycles += 3;
                    c.g[V0][0] = force_off.lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = 1;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        SU_OVERLAY_DONE.call(c, 0x0022_ddc0)?;
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = 1;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            SU_FINISH_SWITCH.call(c, 0x0022_ddd0)?;
                            section_man_update_this(c).off(SWITCH).sw(c, 0);
                            section_man_update_this(c).off(SWITCH_AUX).sw(c, 0);
                        }
                    }
                }
                _ => {}
            }
        }

        // 0x22ddd8: force the slow rate while switching or loading.
        c.cycles += 3;
        c.g[V1][0] = force_off.lw(c);
        c.g[V0][0] = 4;
        let mut hold = c.g[V1][0] != 0;
        if !hold {
            c.cycles += 2;
            hold = c.g[S0][0] != 0;
        }
        if hold {
            c.cycles += 3;
            section_man_update_this(c).off(READY).sw(c, 0);
            section_man_update_this(c).off(RATE).sw(c, 4);
        } else {
            c.g[V0][0] = 1;
            c.cycles += 1;
            section_man_update_this(c).off(READY).sw(c, 1);
        }

        // 0x22ddfc: cStreamMan_SetRate(view + 0x300, rates[class]).
        c.cycles += 12;
        c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
        c.g[V1][0] = 0x0044_0000;
        c.g[V0][0] = section_man_update_this(c).off(RATE).lw(c);
        c.g[V1][0] = sx(RATES);
        c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(WV_STREAM_MAN));
        call_ptr(c, SU_SET_RATE, 0x0022_de28)?;
        frame.restore(c, S0, 0x1a0);
        section_man_update_epilogue(c, frame);
        Ok(())
    })
}

/// 0x22de2c: restore s1..s7, fp and ra (s0 is already restored), return.
pub(crate) fn section_man_update_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x190);
    frame.restore(c, S2, 0x180);
    frame.restore(c, S3, 0x170);
    frame.restore(c, S4, 0x160);
    frame.restore(c, S5, 0x150);
    frame.restore(c, S6, 0x140);
    frame.restore(c, S7, 0x130);
    frame.restore(c, FP, 0x120);
    frame.restore_ra(c, 0x110);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// SectionMan_RequestUnload (0x22d598)

/// `SectionMan_QueueSection(section)` (0x22e298).
pub(crate) const REQUEST_UNLOAD_QUEUE_SECTION: Callee = Callee {
    addr: 0x0022_e298,
    f: ee::s05::SectionMan_QueueSection,
};

/// The section map (`0x442168`, 16-byte entries: +4 slot, +8 state).
#[cfg(test)]
pub(crate) const REQUEST_UNLOAD_SECTION_MAP: u32 = 0x0044_2168;

/// `void cSectionMan::requestUnload(int section)` (0x22d598): when the
/// section's slot (`+4`) is not negative, state 3 (loaded) becomes 0
/// (unloaded) and states 1, 2 and 6 become 5 (unloading); any other state is
/// left alone. A section whose state changed is queued
/// (`SectionMan_QueueSection`). The state switch is a jump table at
/// `0x484c00`, one of three code labels per state.
pub fn section_man_request_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x44_0000;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[V0][0];
        c.g[A3][0] = oc417_add(c.g[V0][0], 0x2168);
        c.g[A2][0] = sx((c.g[A1][0] as u32) << 4);
        c.g[V0][0] = oc417_add(c.g[A3][0], c.g[A2][0]);
        c.g[V1][0] = oc417_lw(c, V0, 4);
        let not_loaded = (c.g[V1][0] as i64) < 0;
        c.g[T1][0] = c.g[A0][0];
        if not_loaded {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.g[V0][0];
            c.g[V1][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = (c.g[V1][0] < 9) as u64;
            let state_out_of_range = c.g[V0][0] == 0;
            c.g[A2][0] = 0;
            if !state_out_of_range {
                c.cycles += 7;
                c.g[V0][0] = 0x48_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c00));
                c.g[V1][0] = oc417_add(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = oc417_lw(c, V1, 0);
                let label = c.g[A0][0] as u32;
                match label {
                    // State 3: back to 0.
                    0x0022_d5f0 => {
                        c.cycles += 6;
                        c.g[V0][0] = oc417_add(c.g[T0][0], 0x2168);
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                        c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                        c.g[A2][0] = 1;
                        oc417_sw0(c, V0, 8);
                    }
                    // States 1, 2, 6: to 5.
                    0x0022_d608 => {
                        c.cycles += 6;
                        c.g[V0][0] = oc417_add(c.g[T0][0], 0x2168);
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                        c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                        c.g[A0][0] = 5;
                        oc417_sw(c, V0, 8, A0);
                        c.g[A2][0] = 1;
                    }
                    // Other states: unchanged.
                    0x0022_d620 => {}
                    _ => {
                        c.jump_out(label, 0x0022_d598, 0x0022_d63c);
                        return Ok(());
                    }
                }
            }
            c.cycles += 2;
            let unchanged = c.g[A2][0] == 0;
            frame.restore_ra(c, 0);
            if !unchanged {
                c.cycles += 2;
                c.g[A0][0] = c.g[T1][0];
                REQUEST_UNLOAD_QUEUE_SECTION.call(c, 0x0022_d630)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

pub(crate) const OC423_SECTION_IS_LOADED: Callee = Callee {
    addr: 0x0022_e078,
    f: ee::s05::SectionMan_IsSectionLoaded,
};

/// `int SectionMan_IsEventLoaded(SectionMan *sections, int event)` (0x22d278):
/// whether every section the event needs is loaded. Event `event` is the
/// 0x28-byte record at `0x442488 + event*0x28`: `count` (+4) section ids at
/// `+0x10` (each checked with `SectionMan_IsSectionLoaded`, stopping at the
/// first miss) and two optional extra section-map indexes at `+8` and `+0xc`
/// (negative: none; else the map entry at `0x442168 + index*0x10` is only
/// checked when its field `+4` is non-negative).
pub fn section_man_is_event_loaded(c: &mut Cpu) {
    /// The event records (`0x440000 + 0x2488`) and the section map
    /// (`0x440000 + 0x2168`).
    const EVENT_BASE: u32 = 0x2488;
    const SECTION_MAP: u32 = 0x2168;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x30);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S6][0] = 0x44_0000;
        // The event's own sections.
        loop {
            c.cycles += 8;
            c.g[V0][0] = 0x28;
            c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(EVENT_BASE));
            c.mult(0, S0, S4, V0);
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            if !more {
                break;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S5][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            OC423_SECTION_IS_LOADED.call(c, 0x0022_d2e4)?;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x0022_d2b0);
                }
                continue;
            }
            c.cycles += 1;
            c.g[S3][0] = 0;
            break;
        }
        // The first extra section (+8).
        'first: {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
            let none = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0x44_0000;
            if none {
                c.cycles += 1;
                c.g[V0][0] = 0x28;
                break 'first;
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SECTION_MAP));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            let unmapped = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x28;
            if unmapped {
                break 'first;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            OC423_SECTION_IS_LOADED.call(c, 0x0022_d320)?;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x28;
        }
        // The second extra section (+0xc); the result is s3.
        'second: {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(EVENT_BASE));
            c.mult(0, V0, S4, V0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let none = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0x44_0000;
            if none {
                c.cycles += 1;
                c.g[V0][0] = c.g[S3][0];
                break 'second;
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SECTION_MAP));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            let unmapped = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = c.g[S3][0];
            if unmapped {
                break 'second;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S5][0];
            OC423_SECTION_IS_LOADED.call(c, 0x0022_d360)?;
            if c.g[V0][0] == 0 {
                c.g[S3][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSectionMan_activateSection`'s state change (`section->state = state`).
pub(crate) fn section_request_set_state(c: &mut Cpu, state: u64) {
    c.cycles += 6;
    c.g[V0][0] = sx(r32(c, A2).wrapping_add(0x2168));
    c.g[V1][0] = sx(r32(c, S0) << 4);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = state;
    c.sw(r32(c, V0).wrapping_add(0x8), c.g[A0][0]);
}

/// `void SectionMan_RequestActivate(SectionMan* self, int i)` (0x22d478,
/// `cSectionMan::activateSection`): step the load state (`+8`) of section `i`
/// of the section map (0x442168, 16-byte entries; unless its world id at
/// `+4` is negative). A jump table (0x47b3d0, indexed by the
/// state, 0..8) picks: states 0, 3 and 7 become 4; 6 becomes 8; 1 and 5 become
/// 2 and the world view activates the section
/// (`cWorldCache_StreamerActivateSection(world + 0x10, section id)`); anything
/// else (2, 4, 8, out of range) stays. If it changed, queue it
/// (`SectionMan_QueueSection(self, i)`). The port accepts the resume entries
/// of the generated `__entries` twin through its fallback.
pub fn section_man_request_activate(c: &mut Cpu) {
    if c.pc != 0x0022_d478 {
        // The other entries (resume points after a callee unwound): the
        // structured form's `__entries` twin; the unwind form's body
        // dispatches on `c.pc` itself.
        return ee::s05::f_0022d478__entries(c);
    }
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[V0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x2168));
        c.g[A1][0] = sx(r32(c, S0) << 4);
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, A1)));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4));
        let unused = (c.g[V1][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        if !unused {
            c.cycles += 5;
            c.g[V1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0x8));
            c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            // `changed` (a1) stays 0 unless a state arm below sets it.
            if !out_of_range {
                c.cycles += 7;
                c.g[V0][0] = 0x48_0000;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4c30));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let case = r32(c, A0);
                let changed = match case {
                    0x0022_d4dc | 0x0022_d4f4 | 0x0022_d50c => {
                        section_request_set_state(c, 4);
                        true
                    }
                    0x0022_d524 => {
                        section_request_set_state(c, 8);
                        true
                    }
                    0x0022_d53c => {
                        c.cycles += 12;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(0x2168));
                        c.g[A1][0] = sx(r32(c, S0) << 4);
                        c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                        c.g[V0][0] = 0x2;
                        c.sw(r32(c, A0).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V1][0] = c.g[A0][0];
                        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x1b0));
                        c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0x4));
                        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
                        c.g[A0][0] = c.lw(r32(c, V0));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                        call_ptr(c, 0x003a_99d8, 0x0022_d56c)?;
                        true
                    }
                    0x0022_d570 => false,
                    _ => {
                        c.jump_out(case, 0x0022_d478, 0x0022_d594);
                        return Ok(());
                    }
                };
                if changed {
                    c.cycles += 1;
                    c.g[A1][0] = 0x1;
                }
            }
            c.cycles += 2;
            let unchanged = c.g[A1][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !unchanged {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0022_e298, 0x0022_d580)?;
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

/// Section event 2..6 of `SectionMan_OnEventTrigger`: the (game mode, game
/// event) pair the race setup is switched to, and the generated block's cycle
/// charge. Other events change nothing.
pub(crate) fn section_man_event_modes(event: u32) -> Option<(u32, u32, u64)> {
    match event {
        2 => Some((0, 0, 8)),
        3 => Some((1, 1, 8)),
        4 => Some((2, 3, 8)),
        5 => Some((3, 2, 8)),
        6 => Some((5, 4, 6)),
        _ => None,
    }
}

/// `void SectionMan_OnEventTrigger(SectionMan* self, int event)` (0x22d6c8):
/// when an event section is triggered in a race (no game path, game mode 4,
/// the location not in 14..16 and `event != 1`), resets the BE state; if the
/// game is in state 4 it resets the AI rider totals, clears the race manager's
/// `+0xc` word, and switches the race setup to the (mode, event) of the section
/// event (location 14..16: mode 4, event 12), runs the race manager's virtual
/// slot at `+0xc`, notifies the HUD with the game event and starts the section
/// transition of the UI at `+0x1b0`.
///
/// Resume entries: when registering, add the `c.pc != entry` fallback to the
/// generated twin (`__entries`, `__generated` in the unwind form); the generated
/// body has resume points, this port does not handle them.
///
/// NOT registered: the final steps run `0x238160` (the game mode switch:
/// its own jump tables and a virtual call), which has no bounded fixture.
pub fn section_man_on_event_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0022_d6ec)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0022_d6f8)?; // cBE_getInterface(0): race
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0x53_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
        let has_path = c.g[V1][0] != 0;
        c.g[A0][0] = 0;
        // skip: a game path is set, the mode is not 4, the section is the
        // finale (14..16) or the event is 1
        if has_path {
            c.cycles += 1;
            c.g[A0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V0][0] ^= 4;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 1;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    if c.g[S1][0] == c.g[V0][0] {
                        c.cycles += 1;
                        c.g[A0][0] = 1;
                    }
                }
            }
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0;
            call_ptr(c, 0x0014_de68, 0x0022_d758)?; // cBE_setState(0)
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = 4;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
            if c.g[V1][0] == c.g[A0][0] {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_b180, 0x0022_d778)?; // cAI_resetRidersStatTotals
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0);
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                let finale = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if finale {
                    c.cycles += 7;
                    c.g[A1][0] = 4;
                    call_ptr(c, 0x0014_4df0, 0x0022_d7ac)?; // setGameMode(4)
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xc;
                    call_ptr(c, 0x0014_51e8, 0x0022_d7b8)?; // setGameEvent(12)
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 7) as u64;
                    let in_table = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    let mut modes = None;
                    if in_table {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4bb0));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        modes = section_man_event_modes(c.g[S1][0] as u32);
                    }
                    match modes {
                        Some((mode, event, cyc)) => {
                            let (rets, last) = match mode {
                                0 => ((0x0022_d7f0, 0x0022_d7fc), false),
                                1 => ((0x0022_d810, 0x0022_d81c), false),
                                2 => ((0x0022_d830, 0x0022_d83c), false),
                                3 => ((0x0022_d850, 0x0022_d85c), false),
                                _ => ((0x0022_d870, 0x0022_d87c), true),
                            };
                            c.cycles += cyc;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = mode as u64;
                            call_ptr(c, 0x0014_4df0, rets.0)?; // setGameMode
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = event as u64;
                            call_ptr(c, 0x0014_51e8, rets.1)?; // setGameEvent
                            if last {
                                c.cycles += 1;
                            }
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        }
                        None => {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        }
                    }
                }
                c.cycles += 15;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0022_d890)?; // race object's virtual slot
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
                call_ptr(c, 0x0023_8160, 0x0022_d8a4)?; // set game mode
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                c.g[A1][0] = 1;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0023_1250, 0x0022_d8b8)?; // section transition
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_LoadEvent(SectionMan* self, int idx, int mode)`: unloads every section (0..49) that the location `idx` does not keep (sections 43..48 are never unloaded), marks the location activated, then activates its sections, the sky/lighting, the extra section and the location's mode (mode 7 means "the table's own mode").
/// (`SectionMan_LoadEvent`, 0x0022d088)
pub fn section_man_load_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_22d0e4: {
            'b_22d0e0: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0x7;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S3][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = c.g[A2][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_22d0e0;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2820));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t = true;
                    c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_22d0e4;
                    } else {
                        break 'b_22d0e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[FP][0] = c.g[A1][0];
            break 'b_22d0e4;
        }
        c.cycles += 15;
        c.g[V1][0] = 0x28;
        c.g[V0][0] = 0x440000;
        c.mult(0, 3, 18, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488));
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[S6][0] = 0x31;
        c.g[A1][0] = 0u64;
        c.g[S1][0] = 0x28;
        c.g[S7][0] = 0x4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = ((c.g[A0][0] as i64) < 0i64) as u64;
        if c.g[V1][0] == 0 {
            c.g[S6][0] = c.g[A0][0];
        }
        c.g[A3][0] = 0x1;
        'l_22d120: loop {
            c.cycles += 4;
            c.g[A2][0] = 0u64;
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c));
            c.g[T1][0] = c.g[A1][0] ^ 0x2b;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            'l_22d130: loop {
                'b_22d160: {
                    c.cycles += 6;
                    c.mult(0, 4, 18, 17);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        break 'b_22d160;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        if c.g[A1][0] != c.g[V1][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x0022d130);
                            }
                            continue 'l_22d130;
                        } else {
                            c.cycles += 1;
                            c.g[A3][0] = 0u64;
                            break 'b_22d160;
                        }
                    }
                }
                'b_22d180: {
                    c.cycles += 5;
                    c.g[V0][0] = (c.g[S7][0] < c.g[T0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[T1][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[A3][0] == 0u64 {
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22d180;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0022d598,
                            f: ee::s05::SectionMan_RequestUnload,
                        }
                        .call(c, 0x0022d17c)?;
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22d180;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 50i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = 0x1;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022d120);
                    }
                    continue 'l_22d120;
                } else {
                    'b_22d1f0: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x28;
                        c.g[V1][0] = 0x1;
                        c.mult(0, 5, 18, 2);
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
                        c.g[V0][0] = 0x440000;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1bc), c.g[S2][0]);
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488));
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S1][0] = 0u64;
                        if t {
                            break 'b_22d1f0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[S4][0] = c.g[V1][0];
                            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            'l_22d1d0: loop {
                                c.cycles += 8;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                Callee {
                                    addr: 0x0022d478,
                                    f: ee::s05::SectionMan_RequestActivate,
                                }
                                .call(c, 0x0022d1e0)?;
                                c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0022d1d0);
                                    }
                                    continue 'l_22d1d0;
                                } else {
                                    break 'b_22d1f0;
                                }
                            }
                        }
                    }
                    'b_22d210: {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0022e180,
                            f: ee::s05::SectionMan_SelectPeakLighting,
                        }
                        .call(c, 0x0022d1fc)?;
                        c.g[V0][0] = c.g[S6][0] ^ 0x31;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[S6][0];
                        if t {
                            break 'b_22d210;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S3][0];
                            Callee {
                                addr: 0x0022d478,
                                f: ee::s05::SectionMan_RequestActivate,
                            }
                            .call(c, 0x0022d210)?;
                            break 'b_22d210;
                        }
                    }
                    'b_22d23c: {
                        c.cycles += 8;
                        c.g[V0][0] = 0x28;
                        c.g[A0][0] = 0x440000;
                        c.mult(0, 2, 18, 2);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2488));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        let t = (c.g[A1][0] as i64) < 0;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_22d23c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S3][0];
                            Callee {
                                addr: 0x0022d478,
                                f: ee::s05::SectionMan_RequestActivate,
                            }
                            .call(c, 0x0022d238)?;
                            c.g[A0][0] = c.g[S3][0];
                            break 'b_22d23c;
                        }
                    }
                    c.cycles += 14;
                    c.g[A1][0] = c.g[FP][0];
                    Callee {
                        addr: 0x0022d668,
                        f: ee::s05::SectionMan_AdvanceEventSwitchSync,
                    }
                    .call(c, 0x0022d244)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
            }
        }
    })
}

/// `void SectionMan_Init(self)` (0x22cd40): resets the section manager.
/// Clears the 50-entry section map at 0x442168 (`{.., idx = -1, 0, ..}`, 16
/// bytes each, then entry 49's index = 0), then for each of the 50 section
/// files of the race interface matches the file name (`strcmp`) against every
/// streamer section name and stores the matching section index in the map.
/// Finally resets the mode/state fields (+0x1a4..+0x1d0), initialises the
/// node pool and sets `+0x1ac = -1`. A dispatcher: its loop back edges
/// deliver events and may unwind, so it is re-entered at `0x22cd90`,
/// `0x22cdc0` or `0x22cdc8`. Frame 0x80 (s0-s6).
pub fn section_man_init(c: &mut Cpu) {
    guest(c, section_man_init_stages)
}

pub(crate) fn section_man_init_stages(c: &mut Cpu) -> Flow {
    const ENTRY: u32 = 0x0022_cd40;
    const CLEAR_MAP: u32 = 0x0022_cd90;
    const START_SCAN: u32 = 0x0022_cdac;
    const NEXT_ENTRY: u32 = 0x0022_cdc0;
    const COMPARE: u32 = 0x0022_cdc8;
    const STORE_MATCH: u32 = 0x0022_ce00;
    const SECTION_LOOP: u32 = 0x0022_ce10;
    const NEXT_FILE: u32 = 0x0022_ce30;
    const FINISH: u32 = 0x0022_ce40;
    let frame = Frame { size: 0x80 };
    let mut pc = c.pc;
    loop {
        match pc {
            ENTRY => {
                c.cycles += 20;
                Frame::open(c, 0x80);
                frame.save(c, S2, 0x50);
                frame.save(c, S5, 0x20);
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                frame.save(c, S3, 0x40);
                frame.save(c, S4, 0x30);
                frame.save(c, S6, 0x10);
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014_dc80, 0x0022_cd6c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_dd58, 0x0022_cd78)?;
                c.g[S5][0] = c.g[V0][0];
                c.g[A1][0] = 0x44_0000;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x2168));
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x8));
                c.g[V1][0] = 0x31;
                pc = CLEAR_MAP;
            }
            CLEAR_MAP => {
                c.cycles += 7;
                c.sw(r32(c, V0).wrapping_sub(4), c.g[A0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                c.sw(r32(c, V0), 0);
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x10));
                if more {
                    if back_edge_unwinds(c, CLEAR_MAP) {
                        return Ok(());
                    }
                    pc = CLEAR_MAP;
                } else {
                    pc = START_SCAN;
                }
            }
            START_SCAN => {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x2168));
                c.g[S3][0] = 0;
                c.sw(r32(c, V0).wrapping_add(0x314), 0);
                c.g[S6][0] = sx(r32(c, V0).wrapping_add(4));
                c.g[S1][0] = 0;
                pc = NEXT_ENTRY;
            }
            NEXT_ENTRY => {
                c.cycles += 2;
                c.g[S4][0] = sx(r32(c, S3).wrapping_add(1));
                pc = SECTION_LOOP;
            }
            COMPARE => {
                c.cycles += 14;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0014_4d38, 0x0022_cdd0)?;
                let s2 = r32(c, S2);
                c.g[V1][0] = c.lw(s2.wrapping_add(0x1b0));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x10));
                c.g[A0][0] = c.lw(r32(c, V0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                call_ptr(c, 0x003a_9820, 0x0022_cdec)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0041_65a8, 0x0022_cdf8)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    pc = SECTION_LOOP;
                } else {
                    pc = STORE_MATCH;
                }
            }
            STORE_MATCH => {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, S3) << 4);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                c.sw(r32(c, V0), c.g[S1][0]);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                pc = SECTION_LOOP;
            }
            SECTION_LOOP => {
                c.cycles += 8;
                let s2 = r32(c, S2);
                c.g[V0][0] = c.lw(s2.wrapping_add(0x1b0));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x10));
                c.g[A0][0] = c.lw(r32(c, V1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                call_ptr(c, 0x003a_9800, 0x0022_ce24)?;
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S5][0];
                    if back_edge_unwinds(c, COMPARE) {
                        return Ok(());
                    }
                    pc = COMPARE;
                } else {
                    pc = NEXT_FILE;
                }
            }
            NEXT_FILE => {
                c.cycles += 4;
                c.g[S3][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 50) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = 0;
                if more {
                    if back_edge_unwinds(c, NEXT_ENTRY) {
                        return Ok(());
                    }
                    pc = NEXT_ENTRY;
                } else {
                    pc = FINISH;
                }
            }
            FINISH => {
                c.cycles += 26;
                c.g[V1][0] = 0x16;
                c.g[V0][0] = 1;
                let s2 = r32(c, S2);
                c.sw(s2.wrapping_add(0x1b8), 0);
                c.g[A0][0] = c.g[S2][0];
                c.sw(s2.wrapping_add(0x1b4), 0);
                c.sw(s2.wrapping_add(0x1c0), 0);
                c.sw(s2.wrapping_add(0x1c8), 0);
                c.sw(s2.wrapping_add(0x1cc), 0);
                c.sw(s2.wrapping_add(0x1bc), c.g[V1][0]);
                c.sw(s2.wrapping_add(0x1d0), c.g[V0][0]);
                SECTION_MAN_INIT_NODE_POOL.call(c, 0x0022_ce6c)?;
                c.g[V0][0] = u64::MAX;
                let s2 = r32(c, S2);
                c.sw(s2.wrapping_add(0x1c4), 0);
                c.sw(s2.wrapping_add(0x1ac), c.g[V0][0]);
                c.sw(s2.wrapping_add(0x1a4), 0);
                c.sw(s2.wrapping_add(0x1a8), 0);
                frame.restore(c, S0, 0x70);
                frame.restore(c, S1, 0x60);
                frame.restore(c, S2, 0x50);
                frame.restore(c, S3, 0x40);
                frame.restore(c, S4, 0x30);
                frame.restore(c, S5, 0x20);
                frame.restore(c, S6, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x0022_cd40, 0x0022_cea8);
                return Ok(());
            }
        }
    }
}

pub(crate) const SECTION_MAN_INIT_NODE_POOL: Callee = Callee {
    addr: 0x0022_e228,
    f: ee::s05::f_0022e228,
};

/// `SectionMan` fields of `SectionMan_ApplySky`.
pub(crate) const SECTION_MAN_SKY_PENDING: u32 = 0x1a4;

pub(crate) const SECTION_MAN_SKY_ACTIVE: u32 = 0x1a8;

pub(crate) const SECTION_MAN_SKY_CURRENT: u32 = 0x1ac;

pub(crate) const SECTION_MAN_SKY_BOX: u32 = 0x1b0;

/// `void SectionMan_ApplySky(SectionMan *self)` (0x22de98): when a sky change
/// is active (`+0x1a8`) and pending (`+0x1a4`), unloads every section
/// 0x2c..0x30 other than the current sky (`+0x1ac`), then points the sky box
/// (`+0x1b0 -> +0x14`) at the section the sky table (0x442168, 16-byte
/// entries, word 1) names for the current sky, clearing it first when that
/// differs from the box's present section (`+0x10`); finally clears pending
/// and active and sets the current sky to -1. Frame 0x30 (s0, s1).
pub fn section_man_apply_sky(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_ACTIVE)).lw(c);
        let inactive = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !inactive {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_PENDING)).lw(c);
            let none_pending = c.g[V0][0] == 0;
            c.g[S0][0] = 0x2c;
            if !none_pending {
                c.cycles += 1;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).lw(c);
                loop {
                    c.cycles += 2;
                    let is_current = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if !is_current {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S0][0];
                        oc435_callee!(0x0022_d598, ee::s05::SectionMan_RequestUnload)
                            .call(c, 0x0022_ded8)?;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 49) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_dec8);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 11;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).lw(c);
                c.g[V0][0] = 0x44_0000;
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_BOX)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x2168));
                c.g[V1][0] = sx(r32(c, V1) << 4);
                c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x14)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
                let same = c.g[V1][0] == c.g[S0][0];
                c.g[V0][0] = u64::MAX;
                if !same {
                    c.cycles += 7;
                    call_ptr(c, 0x0035_3d98, 0x0022_df1c)?;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_BOX)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
                    call_ptr(c, 0x0035_3cf0, 0x0022_df2c)?;
                    c.g[V0][0] = u64::MAX;
                }
                c.cycles += 3;
                Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_PENDING)).sw(c, 0);
                Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).sw(c, c.g[V0][0]);
                Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_ACTIVE)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SectionMan.eventSwitchState` (+0x1b4) and `.eventSwitchArg` (+0x1b8).
pub(crate) const OC436_SM_SWITCH_STATE: u32 = 0x1b4;

pub(crate) const OC436_SM_SWITCH_ARG: u32 = 0x1b8;

/// The original's jump table (states 0..4).
#[cfg(test)]
pub(crate) const OC436_SM_SWITCH_TABLE: u32 = 0x0047_b430;

/// `void SectionMan_AdvanceEventSwitchSync(SectionMan* s, int arg)`
/// (0x22d668): when `arg != 0` and the switch state (+0x1b4) is below 5, a
/// jump table steps the state machine: state 0 stores `arg` (+0x1b8) and goes
/// to 3, state 1 goes to 3 (and returns at once), state 2 goes to 4, states
/// 3 and 4 stay. Other entries go to the generated `__entries` twin.
pub fn oc436_section_man_advance_event_switch_sync(c: &mut Cpu) {
    if c.pc != 0x0022_d668 {
        return oc436_section_man_advance_resume_2(c);
    }
    c.cycles += 2;
    let skip = c.g[A1][0] == 0;
    c.g[A2][0] = c.g[A0][0];
    if !skip {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(OC436_SM_SWITCH_STATE));
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let out_of_range = c.g[V0][0] == 0;
        c.g[V0][0] = 0x48_0000;
        if !out_of_range {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4bd0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            let this = c.g[A2][0] as u32;
            match target {
                0x0022_d698 => {
                    c.cycles += 4;
                    c.g[V0][0] = 3;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_ARG), c.g[A1][0]);
                    let t = c.g[RA][0] as u32;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_STATE), c.g[V0][0]);
                    c.pc = t;
                    return;
                }
                0x0022_d6a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    let t = c.g[RA][0] as u32;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_STATE), c.g[V0][0]);
                    c.pc = t;
                    return;
                }
                0x0022_d6b4 => {
                    c.cycles += 2;
                    c.g[V0][0] = 4;
                    c.sw(this.wrapping_add(OC436_SM_SWITCH_STATE), c.g[V0][0]);
                }
                0x0022_d6bc => {}
                _ => {
                    c.jump_out(target, 0x0022_d668, 0x0022_d6c4);
                    return;
                }
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

pub(crate) fn oc436_section_man_advance_resume_2(c: &mut Cpu) {
    ee::s05::f_0022d668__entries(c)
}

// At install the orchestrator switches this to the generated twin.

/// `bool SectionMan::isPreLoadModeDone(void* self)` (0x22dfd0): the overlay
/// state of the section manager at `self+0x1b0` (through 0x2312d8).
pub fn section_man_is_event_switch_overlay_done(c: &mut Cpu) {
    tail_wrapper(c, Some(0x1b0), Target::Table(0x0023_12d8), 0x0022_dfe0);
}

/// `void SectionMan::gotoLocation(void* self, int location)` (0x22e118; the
/// list calls it ReloadEventAndRestart): calls `cBE_getBE` (0x14dc80) and
/// 0x14dd58, then unless the byte at 0x535c11 is set, loads the event
/// (`SectionMan_LoadEvent`, 0x22d088) and sets `self+0x1cc` to 1.
pub fn section_man_reload_event_and_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0022_e134)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0022_e140)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c11)).lb(c);
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 1;
            Callee {
                addr: 0x0022_d088,
                f: ssx3_ee::s05::f_0022d088,
            }
            .call(c, 0x0022_e160)?;
            c.g[V0][0] = 1;
            let one = c.g[V0][0];
            Addr((c.g[S1][0] as u32).wrapping_add(0x1cc)).sw(c, one);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `base + off` as a 32-bit guest address (`off` may be negative).
#[inline(always)]
pub(crate) fn render_ui_at(base: u64, off: i32) -> u32 {
    (base as u32).wrapping_add(off as u32)
}

/// `void GameModule_RenderUI(GameModule* self)` (0x22c830): the frame's UI pass.
///
/// For each view of the view manager (`self->ViewMan`: cameras at +4, count at
/// +0x10, current view at +0x14, views from +0x18) it makes the view current,
/// sets the viewport window, the perspective and the view matrix from the
/// camera, stores the layer in the state top, sets the render-state bits and
/// draws bucket 3. Then the 2D pass: an orthographic viewport, the debug font
/// scale and colour, the script engine, the UI engine, the load indicator and,
/// while active, the menu. It clears the screen, draws the frame with depth fog
/// off, runs the audio and world trigger updates and resets `mUpdateTick`.
///
/// Every call either returns to the address after it or the function returns at
/// once (host-thread unwind), so each call is followed by `?`.
pub fn game_module_render_ui(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0022_c830);
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 23, 0xd8);
        frame.save_fpr(c, 22, 0xd0);
        frame.save_fpr(c, 21, 0xc8);
        frame.save_fpr(c, 20, 0xc0);

        c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x10));
        let no_views = c.g[V1][0] == 0;
        c.g[S2][0] = 0;
        if !no_views {
            c.cycles += 10;
            c.g[S5][0] = 0xffff_ffff_fe7f_ffff;
            c.g[S4][0] = 0xffff_ffff_ffcf_ffff;
            c.g[S3][0] = 0xffff_ffff_fff0_0fff;
            c.g[FP][0] = 0x30_0000;
            c.g[S6][0] = 0x18;
            c.g[S7][0] = 0xffff_ffff_ffff_fc1f;
            c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
            loop {
                // One view: make it current (cur = i), then the render state.
                c.cycles += 81;
                c.g[A2][0] = 0x1;
                c.g[T1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.sw(render_ui_at(c.g[V0][0], 0x14), c.g[S2][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
                c.g[A3][0] = c.lw(render_ui_at(c.g[T1][0], 0x10d8));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x14));
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A0][0] = c.lh(render_ui_at(c.g[A3][0], 0xc0));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[T0][0] = c.lw(render_ui_at(c.g[A3][0], 0xc4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let t = c.g[T0][0] as u32;
                c.g[S0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                call_ptr(c, t, 0x0022_c8e8)?;

                // SetViewportPerspective(cam->View, cam->Near, cam->Far).
                c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.lwc1(12, c.g[S0][0] as u32);
                c.g[V0][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
                c.lwc1(13, render_ui_at(c.g[S0][0], 0x4));
                c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0xc8));
                c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0xcc));
                c.lwc1(14, render_ui_at(c.g[S0][0], 0x8));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0022_c90c)?;

                // SetView(&cam->CamMat).
                c.g[A2][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.g[A1][0] = sx(render_ui_at(c.g[S0][0], 0x40));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x10d8));
                c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0x110));
                c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x114));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0022_c928)?;

                // The layer into the state top, then the render-state bits.
                c.g[A2][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.g[T1][0] = 0x80_0000;
                c.g[A0][0] = sx(render_ui_at(c.g[GP][0], 0x2898));
                c.g[A1][0] = 0x3;
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x24));
                c.g[A3][0] = c.lw(render_ui_at(c.g[A2][0], 0xe84));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw(render_ui_at(c.g[A2][0], 0xf50));
                c.g[A2][0] = 0xffff_ffff_ffff_f07f;
                c.sh(render_ui_at(c.g[A3][0], 0x12), c.g[V1][0]);
                c.g[T0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
                c.g[V1][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x380;
                c.sw(render_ui_at(c.g[V1][0], 0x4), c.g[V0][0]);
                c.g[V1][0] = 0xffff_ffff_ffff_ffcf;
                c.g[A2][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x0));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x30;
                c.sw(render_ui_at(c.g[A2][0], 0x0), c.g[V0][0]);
                c.g[V1][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw(render_ui_at(c.g[V1][0], 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
                c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                c.sw(render_ui_at(c.g[A2][0], 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] | 0xd000;
                c.sw(render_ui_at(c.g[V1][0], 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(render_ui_at(c.g[T0][0], 0xe84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
                c.g[V0][0] = c.g[V0][0] | 0x80;
                c.sw(render_ui_at(c.g[A2][0], 0x8), c.g[V0][0]);
                // cBucketMan_renderBucket(gBucketMan, 3).
                call_ptr(c, 0x0035_50a0, 0x0022_c9d8)?;

                // while (++i < count): back edge with the event check.
                c.g[V1][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
                c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x10));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x84));
                if event_due(c) {
                    deliver_event_here(c, 0x0022_c8a8);
                }
            }
        }

        // The screen width query (vt slot +0x50 of the view state), into f20.
        c.cycles += 11;
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[V1][0] = c.lw(render_ui_at(c.g[A0][0], 0x10d8));
        c.g[V0][0] = c.lh(render_ui_at(c.g[V1][0], 0xb8));
        c.g[S2][0] = sx(render_ui_at(c.g[V1][0], 0xbc));
        c.g[A1][0] = c.lh(render_ui_at(c.g[V1][0], 0x50));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0022_ca10)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        if neg {
            // Unsigned word to float: halve, keep the low bit, convert, double.
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.add_s(20, 20, 20);
            c.g[V1][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
        } else {
            c.cycles += 4;
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.g[V1][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
        }

        // The screen height query (vt slot +0x58), into f15.
        c.cycles += 6;
        c.g[A0][0] = c.lh(render_ui_at(c.g[V1][0], 0x58));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x5c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_ca54)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        if neg {
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(V0, 15);
            c.cvt_s_w(15, 15);
            c.add_s(15, 15, 15);
        } else {
            c.cycles += 4;
            c.mtc1(V0, 15);
            c.cvt_s_w(15, 15);
        }

        // The 2D pass: the font scale and colour, the debug font, the overlays.
        c.cycles += 87;
        c.mtc1(0, 12);
        c.mov_s(14, 20);
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(20, 12);
        c.mov_s(13, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 23);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = 0x1;
        call_ptr(c, t, 0x0022_caa8)?;
        c.lwc1(21, render_ui_at(c.g[GP][0], -0x51dc));
        c.g[A3][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(AT, 14);
        c.mov_s(16, 20);
        c.g[V0][0] = c.lw(render_ui_at(c.g[A3][0], 0x10d8));
        c.mov_s(17, 23);
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(AT, 15);
        c.mov_s(12, 20);
        c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0xd0));
        c.mov_s(13, 20);
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0xd4));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 22);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_caf4)?;
        c.g[A2][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0xe60));
        c.g[V0][0] = c.lw(render_ui_at(c.g[A2][0], 0x10d8));
        c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0x110));
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x114));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_cb14)?;
        c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[A0][0] = 0xffff_ffff_ffff_fc1f;
        c.g[A2][0] = 0xffff_ffff_ffff_ff83;
        c.g[V1][0] = c.lw(render_ui_at(c.g[A1][0], 0xe84));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x300;
        c.sw(render_ui_at(c.g[V1][0], 0x8), c.g[V0][0]);
        c.g[A0][0] = c.lw(render_ui_at(c.g[A1][0], 0xe84));
        c.g[V0][0] = c.lw(render_ui_at(c.g[A0][0], 0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw(render_ui_at(c.g[A0][0], 0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
        c.lwc1(0, render_ui_at(c.g[V1][0], 0x30));
        c.lwc1(1, render_ui_at(c.g[V1][0], 0x34));
        c.mul_s(0, 0, 21);
        c.mul_s(1, 1, 21);
        c.swc1(0, render_ui_at(c.g[V1][0], 0x38));
        c.swc1(1, render_ui_at(c.g[V1][0], 0x3c));
        c.swc1(22, frame.at(c, 0x0));
        c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
        c.swc1(22, frame.at(c, 0x4));
        c.ldl(4, frame.at(c, 0x7));
        c.ldr(4, frame.at(c, 0x0));
        c.sdl(4, render_ui_at(c.g[V0][0], 0x2f));
        c.sdr(4, render_ui_at(c.g[V0][0], 0x28));
        c.swc1(23, frame.at(c, 0x10));
        c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
        c.swc1(21, frame.at(c, 0x14));
        c.swc1(21, frame.at(c, 0x18));
        c.swc1(20, frame.at(c, 0x1c));
        c.ldl(4, frame.at(c, 0x17));
        c.ldr(4, frame.at(c, 0x10));
        c.ldl(5, frame.at(c, 0x1f));
        c.ldr(5, frame.at(c, 0x18));
        c.sdl(4, render_ui_at(c.g[V0][0], 0x47));
        c.sdr(4, render_ui_at(c.g[V0][0], 0x40));
        c.sdl(5, render_ui_at(c.g[V0][0], 0x4f));
        c.sdr(5, render_ui_at(c.g[V0][0], 0x48));
        // cSSXScriptEngine_Render(ScriptEngine), cUIEngine_render2D(UI),
        // cBXLoadIndicator_Render(LoadIndicator), then the menu while active.
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], -0x84c));
        call_ptr(c, 0x0027_89c0, 0x0022_cbbc)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0x48));
        call_ptr(c, 0x0039_7df8, 0x0022_cbc4)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0x94));
        call_ptr(c, 0x001f_31e0, 0x0022_cbcc)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0xa4));
        call_ptr(c, 0x002c_c098, 0x0022_cbd4)?;
        let menu_active = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        if !menu_active {
            c.cycles += 23;
            c.g[A0][0] = c.lw(render_ui_at(c.g[S1][0], 0xa4));
            call_ptr(c, 0x002c_c350, 0x0022_cbe4)?;
            c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
            c.swc1(22, frame.at(c, 0x4));
            c.swc1(22, frame.at(c, 0x0));
            c.ldl(4, frame.at(c, 0x7));
            c.ldr(4, frame.at(c, 0x0));
            c.sdl(4, render_ui_at(c.g[V0][0], 0x2f));
            c.sdr(4, render_ui_at(c.g[V0][0], 0x28));
            c.swc1(23, frame.at(c, 0x0));
            c.g[V0][0] = c.lw(render_ui_at(c.g[S1][0], 0x24));
            c.swc1(21, frame.at(c, 0x8));
            c.swc1(21, frame.at(c, 0x4));
            c.swc1(20, frame.at(c, 0xc));
            c.ldl(4, frame.at(c, 0x7));
            c.ldr(4, frame.at(c, 0x0));
            c.ldl(5, frame.at(c, 0xf));
            c.ldr(5, frame.at(c, 0x8));
            c.sdl(4, render_ui_at(c.g[V0][0], 0x47));
            c.sdr(4, render_ui_at(c.g[V0][0], 0x40));
            c.sdl(5, render_ui_at(c.g[V0][0], 0x4f));
            c.sdr(5, render_ui_at(c.g[V0][0], 0x48));
            c.g[V0][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        }

        // The end of the frame: the clear colour, the draw with depth fog off,
        // then the audio and world trigger updates.
        c.cycles += 44;
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0x10d8));
        c.g[A0][0] = c.lh(render_ui_at(c.g[V1][0], 0x78));
        c.swc1(20, frame.at(c, 0x8));
        c.swc1(20, frame.at(c, 0x0));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.swc1(20, frame.at(c, 0x4));
        c.g[V0][0] = c.lw(render_ui_at(c.g[V1][0], 0x7c));
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, t, 0x0022_cc5c)?;
        c.g[S0][0] = c.lw(render_ui_at(c.g[GP][0], 0x1234));
        c.sw(render_ui_at(c.g[GP][0], 0x1234), 0u64);
        c.g[A1][0] = c.lw(render_ui_at(c.g[GP][0], -0x854));
        c.g[V0][0] = c.lw(render_ui_at(c.g[A1][0], 0x10d8));
        c.g[A0][0] = c.lh(render_ui_at(c.g[V0][0], 0xa0));
        c.g[V1][0] = c.lw(render_ui_at(c.g[V0][0], 0xa4));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0022_cc7c)?;
        c.sw(render_ui_at(c.g[GP][0], 0x1234), c.g[S0][0]);
        c.g[A1][0] = 0x1;
        c.lwc1(0, render_ui_at(c.g[GP][0], -0x51d8));
        c.lwc1(12, render_ui_at(c.g[S1][0], 0xac));
        c.cvt_s_w(12, 12);
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], 0x410));
        c.mul_s(12, 12, 0);
        // cSSXAudio_Update(gSSXAudio, 1, mUpdateTick / 60).
        call_ptr(c, 0x0028_5bf8, 0x0022_cc9c)?;
        c.g[A0][0] = c.lw(render_ui_at(c.g[GP][0], 0x21e4));
        // cWorldTriggerManager_Update(gWorldTriggerManager).
        call_ptr(c, 0x002b_5d78, 0x0022_cca4)?;
        c.sw(render_ui_at(c.g[S1][0], 0xac), 0u64);

        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 23, 0xd8);
        frame.restore_fpr(c, 22, 0xd0);
        frame.restore_fpr(c, 21, 0xc8);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    });
}

/// Magic word in the machine's RAM that turns the callee fakes on.
#[cfg(test)]
pub(crate) const OC515_GATE: u32 = DATA + 0x20_0000;

#[cfg(test)]
pub(crate) const OC515_GATE_MAGIC: u32 = 0x0515_0515;

/// Per-call result overrides (call index n: a flag word at +0, the 64-bit
/// value at +8): the fakes return the value in v0 when the flag is set.
#[cfg(test)]
pub(crate) const OC515_RES: u32 = DATA + 0x20_1000;

#[cfg(test)]
pub(crate) const OC515_RES_N: u32 = 48;

#[cfg(test)]
pub(crate) const OC515_SEEDS: u32 = 200;

#[cfg(test)]
pub(crate) static OC515_PREV_RENDER_BUCKET: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_UI_RENDER: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_LOAD_INDICATOR: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_MENU_ACTIVE: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_MENU_RENDER: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_AUDIO_UPDATE: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_WORLD_TRIGGER: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static OC515_PREV_MOVIE_PAUSE: AtomicUsize = AtomicUsize::new(0);

/// A fake call to `target` returning to `ra`, with the per-index result override.
#[cfg(test)]
pub(crate) fn oc515_fake_done(c: &mut Cpu, target: u32, ra: u32) {
    let n = c.bus.read32(fakes::LOG_COUNT);
    fakes::fake(c, target, ra);
    if n < OC515_RES_N && c.pc == ra {
        let slot = OC515_RES + 16 * n;
        if c.bus.read32(slot) != 0 {
            c.g[V0][0] = c.bus.read64(slot + 8);
        }
    }
}

/// A callee fake, gated on the magic word; with the gate off the body the slot
/// held before (`prev`, or the generated `generated`) runs.
#[cfg(test)]
pub(crate) fn oc515_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) {
    if c.bus.read32(OC515_GATE) != OC515_GATE_MAGIC {
        return fakes::fall_back(c, prev, generated);
    }
    let ra = c.g[RA][0] as u32;
    oc515_fake_done(c, target, ra);
}

/// The indirect-call hook of the fixture's machines: function pointers in the
/// view state point at fake addresses, which land here.
#[cfg(test)]
pub(crate) fn oc515_miss(c: &mut Cpu, target: u32) {
    let ra = c.g[RA][0] as u32;
    oc515_fake_done(c, target, ra);
}

#[cfg(test)]
pub(crate) fn oc515_fake_render_bucket(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0035_50a0,
        &OC515_PREV_RENDER_BUCKET,
        ee::s11::f_003550a0__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_ui_render(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0039_7df8,
        &OC515_PREV_UI_RENDER,
        ee::s13::f_00397df8__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_load_indicator(c: &mut Cpu) {
    oc515_gated(
        c,
        0x001f_31e0,
        &OC515_PREV_LOAD_INDICATOR,
        ee::s04::f_001f31e0__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_menu_active(c: &mut Cpu) {
    oc515_gated(
        c,
        0x002c_c098,
        &OC515_PREV_MENU_ACTIVE,
        ee::s09::f_002cc098__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_menu_render(c: &mut Cpu) {
    oc515_gated(
        c,
        0x002c_c350,
        &OC515_PREV_MENU_RENDER,
        ee::s09::f_002cc350__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_audio_update(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0028_5bf8,
        &OC515_PREV_AUDIO_UPDATE,
        ee::s07::f_00285bf8__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_world_trigger(c: &mut Cpu) {
    oc515_gated(
        c,
        0x002b_5d78,
        &OC515_PREV_WORLD_TRIGGER,
        ee::s08::f_002b5d78__generated,
    )
}

#[cfg(test)]
pub(crate) fn oc515_fake_movie_pause(c: &mut Cpu) {
    oc515_gated(
        c,
        0x0027_89c0,
        &OC515_PREV_MOVIE_PAUSE,
        ee::s07::f_002789c0__generated,
    )
}

/// Hooks the fakes into the override slots of the callees (once per process).
#[cfg(test)]
pub(crate) fn oc515_install_fakes() {
    fakes::hook(
        addr_of_mut!(ee::s11::OVERRIDE_003550A0),
        oc515_fake_render_bucket,
        &OC515_PREV_RENDER_BUCKET,
    );
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_00397DF8),
        oc515_fake_ui_render,
        &OC515_PREV_UI_RENDER,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001F31E0),
        oc515_fake_load_indicator,
        &OC515_PREV_LOAD_INDICATOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s09::OVERRIDE_002CC098),
        oc515_fake_menu_active,
        &OC515_PREV_MENU_ACTIVE,
    );
    fakes::hook(
        addr_of_mut!(ee::s09::OVERRIDE_002CC350),
        oc515_fake_menu_render,
        &OC515_PREV_MENU_RENDER,
    );
    fakes::hook(
        addr_of_mut!(ee::s07::OVERRIDE_00285BF8),
        oc515_fake_audio_update,
        &OC515_PREV_AUDIO_UPDATE,
    );
    fakes::hook(
        addr_of_mut!(ee::s08::OVERRIDE_002B5D78),
        oc515_fake_world_trigger,
        &OC515_PREV_WORLD_TRIGGER,
    );
    fakes::hook(
        addr_of_mut!(ee::s07::OVERRIDE_002789C0),
        oc515_fake_movie_pause,
        &OC515_PREV_MOVIE_PAUSE,
    );
}

/// The machine for one seed: scrambled registers and fake hooks, the gate word,
/// per-call result overrides, the game globals (GrMan at gp-0x854 with its view
/// state at +0x10d8 and state top at +0xe84), a view manager with 0..3 views,
/// and the self object with its pointers. `a0` is the self object.
#[cfg(test)]
pub(crate) fn oc515_setup(c: &mut Cpu, seed: u32) -> Rng {
    oc515_install_fakes();
    let mut r = fakes::machine(c, seed, 515, 12);
    c.hooks.miss = oc515_miss;
    c.pc = 0x0022_c830;
    c.bus.write32(OC515_GATE, OC515_GATE_MAGIC);
    for n in 0..OC515_RES_N {
        let slot = OC515_RES + 16 * n;
        c.bus.write32(slot, (r.next_u32() % 4 != 0) as u32);
        let v = match r.next_u32() % 3 {
            0 => (r.next_u32() | 0x8000_0000) as i32 as i64 as u64,
            1 => (r.next_u32() & 0x7fff_ffff) as u64,
            _ => 0,
        };
        c.bus.write64(slot + 8, v);
    }

    let me = DATA + 0x1000;
    let view = DATA + 0x4000;
    let gr = DATA + 0x1_0000;
    let st = DATA + 0x1_2000;
    let top = DATA + 0x1_8000;

    // Game globals.
    c.bus.write32(GAME_GP - 0x854, gr);
    c.bus.write32(GAME_GP - 0x84c, DATA + 0x3200);
    c.bus.write32(GAME_GP + 0x410, DATA + 0x3300);
    c.bus.write32(GAME_GP + 0x21e4, DATA + 0x3400);
    c.bus.write32(GAME_GP - 0x51d8, r.f32(4.0));
    c.bus.write32(GAME_GP - 0x51dc, r.f32(4.0));
    c.bus.write32(GAME_GP + 0x1234, r.next_u32());

    // GrMan: view state and state top, with a small index at +0x24.
    c.bus.write32(gr + 0x10d8, st);
    c.bus.write32(gr + 0xe84, top);
    c.bus.write32(gr + 0x24, r.next_u32() % 4);
    fill(c, &mut r, top, 0x40);
    // View state: a {delta, function} pair per virtual slot, delta at +0xb8.
    for (k, off) in [0x50u32, 0x58, 0x78, 0xa0, 0xc0, 0xc8, 0xd0, 0x110]
        .into_iter()
        .enumerate()
    {
        c.bus.write16(st + off, ((r.next_u32() as i16) & !3) as u16);
        c.bus.write32(st + off + 4, fakes::FAKE + 0x10 * k as u32);
    }
    c.bus
        .write16(st + 0xb8, ((r.next_u32() as i16) & !3) as u16);

    // View manager: count at +0x10, cameras at +4.
    c.bus.write32(view + 0x10, seed % 4);
    for i in 0..3u32 {
        let cam = DATA + 0x5000 + 0x100 * i;
        c.bus.write32(view + 4 + 4 * i, cam);
        c.bus.write32(cam, r.f32(100.0));
        c.bus.write32(cam + 4, r.f32(100.0));
        c.bus.write32(cam + 8, r.f32(100.0));
    }

    // The self object.
    c.bus.write32(me + 0x84, view);
    c.bus.write32(me + 0x24, DATA + 0x2000);
    c.bus.write32(me + 0x48, DATA + 0x3000);
    c.bus.write32(me + 0x94, DATA + 0x3100);
    c.bus.write32(me + 0xa4, DATA + 0x3180);
    c.bus.write32(me + 0xac, r.next_u32() % 100);
    fill(c, &mut r, DATA + 0x2000, 0x20);
    c.g[A0][0] = sx(me);
    r
}

/// cSectionMan's location tables: `cSectionMan_mLocationModeMap` (8 bytes
/// per location: the mode at +4) and `cSectionMan_mLocationMap_CEA8` (0x28
/// bytes per location).
#[allow(dead_code)]
pub(crate) const SM_LOCATION_MODE_MAP: u32 = 0x0044_2820;

#[allow(dead_code)]
pub(crate) const SM_LOCATION_MAP: u32 = 0x0044_2488;

/// `void SectionMan_RequestEvent(cSectionMan* self, int idx, int mode)` (0x22cea8):
/// switches to the event location `idx`: unloads every BE section it does not
/// use (43-48 stay), records the location, loads its sections, the sky section
/// (49 when the location has none) and the transport section, then starts the
/// event switch for `mode` (7 maps through the location-mode table).
pub fn oc681_section_man_request_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_22cf04: {
            'b_22cf00: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S3][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = c.g[A2][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_22cf00;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2820));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t = true;
                    c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_22cf04;
                    } else {
                        break 'b_22cf00;
                    }
                }
            }
            c.cycles += 1;
            c.g[FP][0] = c.g[A1][0];
            break 'b_22cf04;
        }
        c.cycles += 15;
        c.g[V1][0] = 0x28;
        c.g[V0][0] = 0x440000;
        c.mult(0, 3, 19, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488));
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[S6][0] = 0x31;
        c.g[A1][0] = 0u64;
        c.g[S1][0] = 0x28;
        c.g[S7][0] = 0x4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = ((c.g[A0][0] as i64) < 0i64) as u64;
        if c.g[V1][0] == 0 {
            c.g[S6][0] = c.g[A0][0];
        }
        c.g[A3][0] = 0x1;
        'l_22cf40: loop {
            c.cycles += 4;
            c.g[A2][0] = 0u64;
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c));
            c.g[T1][0] = c.g[A1][0] ^ 0x2b;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            'l_22cf50: loop {
                'b_22cf80: {
                    c.cycles += 6;
                    c.mult(0, 4, 19, 17);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        break 'b_22cf80;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        if c.g[A1][0] != c.g[V1][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x0022cf50);
                            }
                            continue 'l_22cf50;
                        } else {
                            c.cycles += 1;
                            c.g[A3][0] = 0u64;
                            break 'b_22cf80;
                        }
                    }
                }
                'b_22cfa0: {
                    c.cycles += 5;
                    c.g[V0][0] = (c.g[S7][0] < c.g[T0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[T1][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[A3][0] == 0u64 {
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22cfa0;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0022cf9c);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0022d598,
                            f: ee::s05::SectionMan_RequestUnload,
                        }
                        .call(c, 0x0022cf9c)?;
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22cfa0;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 50i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = 0x1;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022cf40);
                    }
                    continue 'l_22cf40;
                } else {
                    'b_22d008: {
                        c.cycles += 11;
                        c.g[V0][0] = 0x28;
                        c.g[V1][0] = 0x440000;
                        c.mult(0, 5, 19, 2);
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2488));
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x1bc), c.g[S3][0]);
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x1c0), 0u64);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S1][0] = 0u64;
                        if t {
                            break 'b_22d008;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[S4][0] = c.g[A0][0];
                            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            'l_22cfe8: loop {
                                c.cycles += 8;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[RA][0] = sx(0x0022cff8);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                Callee {
                                    addr: 0x0022d3d8,
                                    f: ee::s05::SectionMan_RequestLoad,
                                }
                                .call(c, 0x0022cff8)?;
                                c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0022cfe8);
                                    }
                                    continue 'l_22cfe8;
                                } else {
                                    break 'b_22d008;
                                }
                            }
                        }
                    }
                    'b_22d020: {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[S6][0] ^ 0x31;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[S6][0];
                        if t {
                            break 'b_22d020;
                        } else {
                            c.cycles += 3;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x1a4), 0u64);
                            c.g[RA][0] = sx(0x0022d020);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x0022d3d8,
                                f: ee::s05::SectionMan_RequestLoad,
                            }
                            .call(c, 0x0022d020)?;
                            break 'b_22d020;
                        }
                    }
                    'b_22d04c: {
                        c.cycles += 8;
                        c.g[V0][0] = 0x28;
                        c.g[A0][0] = 0x440000;
                        c.mult(0, 2, 19, 2);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2488));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        let t = (c.g[A1][0] as i64) < 0;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_22d04c;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0022d048);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x0022d3d8,
                                f: ee::s05::SectionMan_RequestLoad,
                            }
                            .call(c, 0x0022d048)?;
                            c.g[A0][0] = c.g[S2][0];
                            break 'b_22d04c;
                        }
                    }
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x0022d054);
                    c.g[A1][0] = c.g[FP][0];
                    Callee {
                        addr: 0x0022d640,
                        f: ee::s05::SectionMan_StartEventSwitch,
                    }
                    .call(c, 0x0022d054)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
            }
        }
    });
}

/// `SectionMan_RequestLoad` (0x0022d3d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn section_man_request_load(c: &mut Cpu) {
    if c.pc != 0x0022d3d8 {
        return ee::s05::f_0022d3d8__generated(c);
    }
    guest(c, |c| {
        'b_22d470: {
            'b_22d46c: {
                c.cycles += 10;
                c.g[V0][0] = 0x440000;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[T0][0] = c.g[V0][0];
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2168));
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[T1][0] = c.g[A0][0];
                if t {
                    break 'b_22d46c;
                } else {
                    'b_22d45c: {
                        c.cycles += 5;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_22d45c;
                        } else {
                            'b_22d454: {
                                'b_22d444: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x480000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c60));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x0022d430 => {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                sx((c.g[T0][0] as u32).wrapping_add(0x2168));
                                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            let t = true;
                                            c.g[A0][0] = 0x1;
                                            if t {
                                                break 'b_22d454;
                                            } else {
                                                break 'b_22d444;
                                            }
                                        }
                                        0x0022d444 => {
                                            break 'b_22d444;
                                        }
                                        0x0022d45c => {
                                            break 'b_22d45c;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x0022d3d8, 0x0022d478);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2168));
                                c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A0][0] = 0x3;
                                break 'b_22d454;
                            }
                            c.cycles += 2;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                            c.g[A2][0] = 0x1;
                            break 'b_22d45c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        break 'b_22d470;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0022d46c);
                        c.g[A0][0] = c.g[T1][0];
                        Callee {
                            addr: 0x0022e298,
                            f: ee::s05::SectionMan_QueueSection,
                        }
                        .call(c, 0x0022d46c)?;
                        break 'b_22d46c;
                    }
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            break 'b_22d470;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `SectionMan_OnEventTrigger` (0x0022d6c8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn section_man_on_event_trigger_2(c: &mut Cpu) {
    if c.pc != 0x0022d6c8 {
        return ee::s05::f_0022d6c8__generated(c);
    }
    guest(c, |c| {
        'b_22d748: {
            'b_22d744: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x0022d6ec);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x0022d6ec)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0022d6f8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0022d6f8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 0x530000;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_22d744;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.g[V0][0] ^ 0x4;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0x1;
                        break 'b_22d748;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = 0x1;
                            break 'b_22d748;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] != c.g[V0][0];

                            if t {
                                break 'b_22d748;
                            } else {
                                break 'b_22d744;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = 0x1;
            break 'b_22d748;
        }
        'b_22d8bc: {
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_22d8bc;
            } else {
                c.cycles += 8;
                c.g[RA][0] = sx(0x0022d758);
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x0014de68, 0x0022d758)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = 0x4;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
                if c.g[V1][0] != c.g[A0][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    break 'b_22d8bc;
                } else {
                    'b_22d880: {
                        'b_22d7c0: {
                            c.cycles += 13;
                            c.g[RA][0] = sx(0x0022d778);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            call_ptr(c, 0x0012b180, 0x0022d778)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_22d7c0;
                            } else {
                                c.cycles += 7;
                                c.g[RA][0] = sx(0x0022d7ac);
                                c.g[A1][0] = 0x4;
                                call_ptr(c, 0x00144df0, 0x0022d7ac)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022d7b8);
                                c.g[A1][0] = 0xc;
                                call_ptr(c, 0x001451e8, 0x0022d7b8)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                if t {
                                    break 'b_22d880;
                                } else {
                                    break 'b_22d7c0;
                                }
                            }
                        }
                        'b_22d87c: {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x480000;
                            if t {
                                break 'b_22d87c;
                            } else {
                                'b_22d864: {
                                    'b_22d844: {
                                        'b_22d824: {
                                            'b_22d804: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x4bb0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x0022d7e4 => {
                                                        c.cycles += 8;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0022d7f0);
                                                        c.g[A1][0] = 0u64;
                                                        call_ptr(c, 0x00144df0, 0x0022d7f0)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0022d7fc);
                                                        c.g[A1][0] = 0u64;
                                                        call_ptr(c, 0x001451e8, 0x0022d7fc)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_22d880;
                                                        } else {
                                                            break 'b_22d804;
                                                        }
                                                    }
                                                    0x0022d804 => {
                                                        break 'b_22d804;
                                                    }
                                                    0x0022d824 => {
                                                        break 'b_22d824;
                                                    }
                                                    0x0022d844 => {
                                                        break 'b_22d844;
                                                    }
                                                    0x0022d864 => {
                                                        break 'b_22d864;
                                                    }
                                                    0x0022d87c => {
                                                        break 'b_22d87c;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x0022d6c8, 0x0022d8d4);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0022d810);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x00144df0, 0x0022d810)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0022d81c);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x001451e8, 0x0022d81c)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                            if t {
                                                break 'b_22d880;
                                            } else {
                                                break 'b_22d824;
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0022d830);
                                        c.g[A1][0] = 0x2;
                                        call_ptr(c, 0x00144df0, 0x0022d830)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0022d83c);
                                        c.g[A1][0] = 0x3;
                                        call_ptr(c, 0x001451e8, 0x0022d83c)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_22d880;
                                        } else {
                                            break 'b_22d844;
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0022d850);
                                    c.g[A1][0] = 0x3;
                                    call_ptr(c, 0x00144df0, 0x0022d850)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0022d85c);
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, 0x001451e8, 0x0022d85c)?;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    if t {
                                        break 'b_22d880;
                                    } else {
                                        break 'b_22d864;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022d870);
                                c.g[A1][0] = 0x5;
                                call_ptr(c, 0x00144df0, 0x0022d870)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0022d87c);
                                c.g[A1][0] = 0x4;
                                call_ptr(c, 0x001451e8, 0x0022d87c)?;
                                break 'b_22d87c;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        break 'b_22d880;
                    }
                    c.cycles += 15;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0022d890);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0022d890)?;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5bc8));
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    c.g[RA][0] = sx(0x0022d8a4);
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc0));
                    call_ptr(c, 0x00238160, 0x0022d8a4)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b0));
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x1;
                    c.g[RA][0] = sx(0x0022d8b8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00231250, 0x0022d8b8)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    break 'b_22d8bc;
                }
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use crate::difftest::check;
    use crate::game::ai::control::handplantcontrol::tests::oc425_install;

    use crate::difftest::check_paths;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::world_2 as world;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_6 as be_library_3;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;

    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::w;

    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn section_man_start_event_switch_matches() {
        check_paths(
            ssx3_ee::s05::f_0022d640,
            section_man_start_event_switch,
            300,
            &[4, 7, 9],
            &|c, seed| {
                let mut r = Rng(0x0c11_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let event = if seed % 3 == 0 {
                    0
                } else {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                };
                c.g[A1][0] = event;
                c.bus
                    .write32(DATA + 0x1b4, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn section_man_is_section_loaded_matches() {
        check(
            ssx3_ee::s05::f_0022e078,
            section_man_is_section_loaded,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c11_0003 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 16) as u32;
                c.g[A1][0] = idx as u64;
                let base = 0x44_2168u32.wrapping_add(idx << 4);
                let state = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(base + 0x8, state);
            },
        );
    }

    #[test]
    fn section_man_get_world_section_index_matches() {
        check(
            ssx3_ee::s05::f_0022e0c8,
            section_man_get_world_section_index,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c11_0004 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 16) as u32;
                c.g[A1][0] = idx as u64;
                let base = 0x44_2168u32.wrapping_add(idx << 4);
                c.bus.write32(base + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn game_module_draw_bezier_patch_matches() {
        check(
            ee::s05::f_0022c410__generated,
            game_module_draw_bezier_patch,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22c4, 1);
                let (patch, view, obj, vt) = (DATA, DATA + 0x200, DATA + 0x400, DATA + 0x500);
                put_vec3(c, &mut r, patch + 0x10, -100.0, 100.0);
                for k in 0..12 {
                    c.bus
                        .write32(view + 0x170 + 4 * k, f(uf(&mut r, -100.0, 100.0)));
                }
                c.bus.write32(view + 0x1a0, r.next_u32());
                c.bus.write32(patch + 0x20, f(uf(&mut r, 0.0, 0.0005)));
                c.bus.write32(patch + 0x24, f(uf(&mut r, -2.0, 8.0)));
                let lo = uf(&mut r, 0.0, 2.0);
                c.bus.write32(patch + 0x28, f(lo));
                c.bus.write32(patch + 0x2c, f(lo + uf(&mut r, 0.0, 6.0)));
                fill(c, &mut r, gp_at(-0x6e0), 4);
                c.bus.write32(patch, obj);
                c.bus.write32(obj + 4, vt);
                c.bus.write16(vt + 0xa8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0xac, FAKE);
                c.g[A0][0] = sx(patch);
                c.g[A1][0] = sx(view);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn section_man_dtor_matches() {
        crate::difftest::check(
            ee::s05::f_0022cd18__generated,
            section_man_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22cd_18, 2);
                be_library(c);
                if seed % 2 == 1 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = 1;
                } else {
                    c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                    c.g[A1][0] = sx(r.next_u32() & !1);
                }
            },
        );
    }

    #[test]
    fn section_man_ctor_matches() {
        check(
            ssx3_ee::s05::f_0022cce8__generated,
            section_man_ctor,
            20,
            &|c, seed| {
                let _r = machine(c, seed, 0xb1, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3_0000 + (seed % 4) * 0x1000);
            },
        );
    }

    #[test]
    fn section_man_get_event_world_section_matches() {
        check(
            ee::s05::f_0022e098__generated,
            section_man_get_event_world_section,
            60,
            &|c, seed| {
                machine(c, seed, 0x22e098, 0);
                c.g[A0][0] = DATA as u64;
                // `event` is a byte offset into the event records; the function's
                // `mult` scales it by 0x28 to reach the record's section index.
                let event = (seed % 8) * 0x28;
                c.g[A1][0] = event as u64;
                let section = seed % 16;
                c.bus.write32(0x0044_2488 + 0x10 + event * 0x28, section);
                c.bus.write32(0x0044_216c + section * 16, 0x1000 + seed);
            },
        );
    }

    #[test]
    fn section_man_request_sky_matches() {
        check(
            ssx3_ee::s05::f_0022de58__generated,
            section_man_request_sky,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x32, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
                let v = (seed % 4) as u64;
                c.g[A1][0] = v;
                c.bus
                    .write32(DATA + 0x1ac, if seed % 3 == 0 { v as u32 } else { 0 });
            },
        );
    }

    #[test]
    fn section_man_is_event_ready_matches() {
        check(
            ssx3_ee::s05::f_0022d390__generated,
            section_man_is_event_ready,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x22d390, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x1c0, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn section_man_dequeue_section_matches() {
        check(
            ee::s05::f_0022e360__generated,
            section_man_dequeue_section,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22e360, 0);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let node = DATA + 0x2000;
                let next = DATA + 0x2100;
                clear(c, node, 0x40);
                clear(c, next, 0x40);
                c.bus.write32(next + 4, r.next_u32());
                c.bus.write32(this + 0x1a0, r.next_u32() % 100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(node) };
                c.g[A2][0] = sx(next);
                let _ = seed;
            },
        );
    }

    #[test]
    fn section_man_begin_event_switch_matches() {
        check(
            ee::s05::f_0022df50__generated,
            section_man_begin_event_switch,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x22df_50, 4);
                let self_ = DATA;
                let module = DATA + 0x1000;
                w(c, self_ + 0x1b0, module);
                w(c, self_ + 0x1bc, r.next_u32());
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn section_man_finish_event_switch_matches() {
        check(
            ee::s05::f_0022dff0__generated,
            section_man_finish_event_switch,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22dff0, 4);
                be_library_3(c, &mut r);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x1cc, (seed % 2) as u32);
                c.bus.write32(DATA + 0x1b0, 0);
                // A minimal world -> race manager -> object with a `+0xcc` slot.
                let w = DATA + 0x1000;
                world(c, w);
                c.bus.write32(w + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xc, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 0xcc, DATA + 0x1300);
                c.bus.write16(DATA + 0x1300 + 0x28, 0);
                c.bus.write32(DATA + 0x1300 + 0x2c, FAKE);
                // The track info's first word (a track id), and the lighting table
                // entry so the dispatcher takes a real arm.
                c.bus.write32(0x0043_d950, 0);
                c.bus.write32(0x0047_b4e0, 0x0022_e1ac);
            },
        );
    }

    // ------------------------------------------------ GameModule_BeginBezierPass

    /// GameModule_BeginBezierPass: the renderer's view-matrix method and
    /// the camera's near / far / begin methods are fakes (the matrix
    /// pointers forced to random matrices; may unwind).
    #[test]
    fn game_module_begin_bezier_pass_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use bezier_pass::*;
        const CTX: u32 = 0x00a0_0000;
        const REND: u32 = 0x00a0_1000;
        const CAM: u32 = 0x00a0_3000;
        const MAT: u32 = 0x00a0_4000;
        covered(
            ee::s05::f_0022c1b0__generated,
            game_module_begin_bezier_pass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_c1b0, 5);
                c.pc = 0x0022_c1b0;
                c.bus.write32(GAME_GP - GP_RENDERER, REND);
                let vt = vtable(c, &mut r, &[SLOT_VIEW_MATRIX]);
                c.bus.write32(REND + RENDERER_VT, vt);
                c.bus.write32(GAME_GP + GP_CAMERA, CAM);
                let cvt = vtable(c, &mut r, &[SLOT_NEAR, SLOT_FAR, SLOT_BEGIN]);
                c.bus.write32(CAM + 4, cvt);
                for k in 0..2u32 {
                    let m = MAT + 0x100 * k;
                    for i in 0..16 {
                        c.bus.write32(m + 4 * i, r.edge_f32());
                    }
                    c.bus.write32(LOG_RESULTS + 4 * k, m);
                }
                fill(c, &mut r, CTX, 0x10);
                c.g[A0][0] = CTX as u64;
            },
        );
    }

    /// GameModule_RenderFogParticles: both, one or neither fog list
    /// non-empty; the manager's renderFog slot is a fake (random this-delta,
    /// may unwind); the real frame finisher on its short path (no particles:
    /// first word 0).
    #[test]
    fn game_module_render_fog_particles_matches() {
        const MODULE: u32 = DATA + 0x1_0000;
        const WORLD: u32 = DATA + 0x2_0000;
        const MAN: u32 = DATA + 0x3_0000;
        covered(
            ee::s05::f_0022c708__generated,
            game_module_render_fog_particles,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_c708, 2);
                c.pc = 0x0022_c708;
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[FOG_RENDER_SLOT]);
                c.bus.write32(MAN, 0);
                c.bus.write32(MAN + FOG_VTABLE, vt);
                c.bus.write32(MODULE + GM_FOG_MAN, MAN);
                for (k, &(count_off, _, _, _)) in WORLD_FOG_LISTS.iter().enumerate() {
                    let n = if (seed >> k) % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 20
                    };
                    c.bus.write32(WORLD + count_off, n);
                }
                c.g[A0][0] = sx(MODULE);
                c.g[A1][0] = sx(WORLD);
            },
        );
    }

    // -- 0x0022c790 GameModule_RenderLensFlares --------------------------------

    /// `GameModule_RenderLensFlares` against the generated body, views 0 and
    /// 1: each world source list empty, negative, or with 1..3 sources (the
    /// real `LensFX_ClearSources` / `AddSources` rebuild the flares), then
    /// the real `LensFX_Render` with the `LensFX_PrepareView` fixture (camera
    /// interface slots and the projector's occlusion slot are fakes, the sun
    /// texture is -1 so the sun is hidden) and the `LensFX_DrawFlares` render
    /// state push/pop fakes (the added flares are off, nothing is drawn);
    /// unwinds in the first six calls.
    #[test]
    fn game_module_render_lens_flares_matches() {
        use super::render_lens_flares::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{fill, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::lens_prep::{
            CAMERA_GP, CAM_VT, VT_FOV, VT_VIEW_MATRIX, VT_VIEW_RECT,
        };
        use crate::game::visualfx::lensfx::tests::painter_sun;
        const THIS: u32 = DATA + 0x1000;
        const WORLD: u32 = DATA + 0x1_0000;
        const FX: u32 = DATA + 0x4_0000;
        const CAM: u32 = DATA + 0x5_0000;
        const CAM_VTABLE: u32 = DATA + 0x5_1000;
        const NO_SUN: u32 = DATA + 0x5_4000;
        const SRC: u32 = DATA + 0x5_5000;
        const MATRIX: u32 = DATA + 0x5_6000;
        covered(
            ee::s05::f_0022c790__generated,
            game_module_render_lens_flares,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_c790, 6);
                c.pc = 0x0022_c790;
                let view = seed % 2;
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(WORLD);
                c.g[A2][0] = view as u64;
                c.bus.write32(THIS + LENS_FX, FX);
                // The world's source lists.
                for (count_at, list) in [(SOURCES0_COUNT, SOURCES0), (SOURCES2_COUNT, SOURCES2)] {
                    let n = match r.next_u32() % 5 {
                        0 => 0,
                        1 => (r.next_u32() % 3 + 1).wrapping_neg(),
                        _ => 1 + r.next_u32() % 3,
                    };
                    c.bus.write32(WORLD + count_at, n);
                    for k in 0..3 {
                        c.bus.write32(WORLD + list + 4 * k, SRC + 0x40 * k);
                    }
                }
                fill(c, &mut r, SRC, 0x30);
                // The camera and its interface (as in the PrepareView test).
                c.bus.write32(GAME_GP + CAMERA_GP, CAM);
                c.bus.write32(CAM + CAM_VT, CAM_VTABLE);
                for (i, off) in [VT_VIEW_RECT, VT_VIEW_MATRIX, VT_FOV, 0x2d8]
                    .into_iter()
                    .enumerate()
                {
                    let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                    c.bus.write16(CAM_VTABLE + off, delta as u16);
                    c.bus.write32(CAM_VTABLE + off + 4, FAKE + 0x10 * i as u32);
                }
                fill(c, &mut r, MATRIX, 16);
                c.bus.write32(LOG_RESULTS + 4, MATRIX);
                for v in [6, 7] {
                    painter_sun(c, &mut r, v, DATA + 0x5_2000, DATA + 0x5_3000, &[0x108]);
                }
                c.bus.write32(NO_SUN, u32::MAX);
                c.bus.write32(LOG_RESULTS + 12, NO_SUN);
                fill(c, &mut r, FX, 0x18);
                // DrawFlares: the sun flag and the render-state methods.
                c.bus
                    .write32(GAME_GP + 0xa28, if seed % 3 == 0 { 0 } else { 1 });
                // (At a fixed address: `vtable` picks a random one that could
                // land on the lens FX.)
                const FX_VT: u32 = DATA + 0x5_7000;
                for (i, off) in [0x10u32, 0x18].into_iter().enumerate() {
                    c.bus
                        .write16(FX_VT + off, (r.next_u32() as i16 & !3) as u16);
                    c.bus
                        .write32(FX_VT + off + 4, FAKE + 0x40 + 0x10 * i as u32);
                }
                c.bus.write32(FX + 0x649c, FX_VT);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (other tests run the real bodies).
    pub(crate) const SECTION_UPDATE_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const SECTION_UPDATE_MAGIC: u32 = 0x5ec7_d8d8;

    macro_rules! section_update_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(SECTION_UPDATE_CTRL) != SECTION_UPDATE_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                fake_call(c, $addr);
            }
        };
    }

    section_update_gated!(
        section_update_fake_on_unload,
        SECTION_UPDATE_PREV_ON_UNLOAD,
        0x0023_0360,
        ee::s06::f_00230360__generated
    );

    section_update_gated!(
        section_update_fake_activate,
        SECTION_UPDATE_PREV_ACTIVATE,
        0x003a_99d8,
        ee::s13::f_003a99d8__generated
    );

    section_update_gated!(
        section_update_fake_request_sky,
        SECTION_UPDATE_PREV_REQUEST_SKY,
        0x0022_de58,
        ee::s05::f_0022de58__generated
    );

    section_update_gated!(
        section_update_fake_begin_switch,
        SECTION_UPDATE_PREV_BEGIN_SWITCH,
        0x0022_df50,
        ee::s05::f_0022df50__generated
    );

    section_update_gated!(
        section_update_fake_finish_switch,
        SECTION_UPDATE_PREV_FINISH_SWITCH,
        0x0022_dff0,
        ee::s05::f_0022dff0__generated
    );

    section_update_gated!(
        section_update_fake_sprintf,
        SECTION_UPDATE_PREV_SPRINTF,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    pub(crate) fn section_update_install_fakes() {
        use std::ptr::addr_of_mut;
        // SAFETY: override slots are plain fn pointers; `hook` serialises.
        {
            hook(
                addr_of_mut!(ee::s06::OVERRIDE_00230360),
                section_update_fake_on_unload,
                &SECTION_UPDATE_PREV_ON_UNLOAD,
            );
            hook(
                addr_of_mut!(ee::s13::OVERRIDE_003A99D8),
                section_update_fake_activate,
                &SECTION_UPDATE_PREV_ACTIVATE,
            );
            hook(
                addr_of_mut!(ee::s05::OVERRIDE_0022DE58),
                section_update_fake_request_sky,
                &SECTION_UPDATE_PREV_REQUEST_SKY,
            );
            hook(
                addr_of_mut!(ee::s05::OVERRIDE_0022DF50),
                section_update_fake_begin_switch,
                &SECTION_UPDATE_PREV_BEGIN_SWITCH,
            );
            hook(
                addr_of_mut!(ee::s05::OVERRIDE_0022DFF0),
                section_update_fake_finish_switch,
                &SECTION_UPDATE_PREV_FINISH_SWITCH,
            );
            hook(
                addr_of_mut!(ee::s15::OVERRIDE_00417828),
                section_update_fake_sprintf,
                &SECTION_UPDATE_PREV_SPRINTF,
            );
        }
    }

    /// SectionMan_Update on a queue of 0..6 section events in random states
    /// (0..=8 and out of range) over a real world cache (streamer states,
    /// counts and the `.rodata` jump tables from the ELF image), the real
    /// BE getters, list helpers, sky apply, overlay check and stream rate;
    /// fakes (that may unwind) for the module hooks, the activation, the
    /// sky request, the event-switch begin/finish and sprintf. Covers the
    /// empty-queue exit, busy/pending/changed/force-off combinations, every
    /// switch state and every rate class.
    #[test]
    fn section_man_update_matches() {
        use section_man_update_layout::*;
        section_update_install_fakes();
        covered(
            ee::s05::f_0022d8d8__generated,
            section_man_update,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0022_d8d8, 8);
                c.bus.write32(SECTION_UPDATE_CTRL, SECTION_UPDATE_MAGIC);
                c.pc = 0x0022_d8d8;
                let gp = GAME_GP;

                let this = DATA;
                let game = DATA + 0x2000;
                let holder = DATA + 0x2400;
                let view = DATA + 0x3000;
                let cache = view + 0x10;
                let streamer = DATA + 0x5000;
                c.g[A0][0] = sx(this);

                // BE: the library instance and interface 0 (a fake method).
                c.bus.write32(gp.wrapping_sub(0x1f04), DATA + 0x6000);
                c.bus.write32(gp.wrapping_sub(0x1efc), 0);
                let be_obj = DATA + 0x6100;
                let be_vt = DATA + 0x6200;
                c.bus.write32(0x0053_08a8, be_obj);
                c.bus.write32(be_obj + 4, be_vt);
                c.bus.write16(be_vt + 8, 0);
                c.bus.write32(be_vt + 0xc, crate::difftest::fakes::FAKE);
                // cStreamMan_SetRate's global.
                c.bus.write32(gp.wrapping_add(0x2a74), DATA + 0x6400);
                c.bus.write32(DATA + 0x6410, 1 + r.next_u32() % 0x8000);

                // The manager.
                let sky_pending = (r.next_u32() % 3 == 0) as u32;
                c.bus.write32(this + 0x1a4, 0);
                c.bus.write32(this + SKY_PENDING, sky_pending);
                c.bus.write32(this + 0x1ac, r.next_u32() % 50);
                c.bus.write32(this + GAME, game);
                let switch = match seed % 8 {
                    0..=4 => seed % 8,
                    5 => 4,
                    6 => 0,
                    _ => 5 + r.next_u32() % 3,
                };
                c.bus.write32(this + SWITCH, switch);
                c.bus.write32(this + SWITCH_AUX, r.next_u32());
                c.bus
                    .write32(this + STREAM_ON, (r.next_u32() % 3 != 0) as u32);
                c.bus.write32(this + RATE, r.next_u32());
                c.bus.write32(this + READY, r.next_u32());
                c.bus.write32(this + 0x198, DATA + 0x1800);

                // Game module, world view and its streamer.
                c.bus.write32(
                    game + 0xc,
                    if r.next_u32() & 1 == 0 {
                        0
                    } else {
                        DATA + 0x2100
                    },
                );
                c.bus.write32(
                    DATA + 0x2100 + 0xa4,
                    if r.next_u32() & 1 == 0 {
                        0
                    } else {
                        DATA + 0x2200
                    },
                );
                c.bus.write32(game + 0x10, holder);
                c.bus.write32(game + 0x204, 0);
                c.bus.write32(game + 0x208, r.next_u32() & 1);
                c.bus.write32(holder, view);
                c.bus.write32(view + WV_ACTIVE, r.next_u32() & 1);
                c.bus.write32(view + WV_VISIBLE, r.next_u32() & 1);
                let sections = 1 + r.next_u32() % 12;
                c.bus
                    .write32(cache + 4, if r.next_u32() % 8 == 0 { 0 } else { streamer });
                c.bus.write32(streamer + 8, sections);
                for s in 0..16 {
                    c.bus.write32(cache + 0x14 + 8 * s, r.next_u32() % 8);
                }

                // The queue: distinct events in random states.
                let n = match seed % 5 {
                    0 => 0,
                    _ => 1 + r.next_u32() % 6,
                };
                let mut used = Vec::new();
                let mut prev = 0u32;
                for i in 0..n {
                    let mut idx = r.next_u32() % 50;
                    while used.contains(&idx) {
                        idx = (idx + 1) % 50;
                    }
                    used.push(idx);
                    let node = DATA + 0x1000 + 8 * i;
                    c.bus.write32(node, idx);
                    c.bus.write32(node + 4, 0);
                    if prev == 0 {
                        c.bus.write32(this + 0x19c, node);
                    } else {
                        c.bus.write32(prev + 4, node);
                    }
                    prev = node;
                    let event = EVENTS + 16 * idx;
                    c.bus.write32(event + 4, r.next_u32() % (sections + 3));
                    let state = match r.next_u32() % 12 {
                        9 => 9 + r.next_u32() % 4,
                        10 => 3,
                        11 => 4,
                        s => s,
                    };
                    c.bus.write32(event + 8, state);
                    c.bus.write32(event + 0xc, r.next_u32() % 8);
                    let field10 = if r.next_u32() & 1 == 0 {
                        4
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(0x0043_e264 + 0x18 * idx, field10);
                }
                if n == 0 {
                    c.bus.write32(this + 0x19c, 0);
                }
                c.bus.write32(this + 0x1a0, n);
            },
        );
    }

    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc417_machine;

    // ----------------------------------------------------------------------
    // SectionMan_RequestUnload

    /// Every section state 0-9 (the jump table `0x484c00` is written with
    /// the three code labels, and, for one seed in 40, a wild address that
    /// leaves through the `jump_out` hook), negative and non-negative slots,
    /// the faked queue call.
    #[test]
    fn section_man_request_unload_matches() {
        check(
            ee::s05::f_0022d598__generated,
            section_man_request_unload,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0009, 1);
                c.pc = 0x0022_d598;
                let labels = [0x0022_d5f0u32, 0x0022_d608, 0x0022_d620];
                for state in 0..9u32 {
                    let label = match state {
                        3 => labels[0],
                        1 | 2 | 6 => labels[1],
                        _ => labels[2],
                    };
                    let label = if seed % 40 == 39 && state == 4 {
                        0x0022_d640
                    } else {
                        label
                    };
                    c.bus.write32(0x0048_4c00 + 4 * state, label);
                }
                let idx = seed % 4;
                let section = REQUEST_UNLOAD_SECTION_MAP + 0x10 * idx;
                let slot = if (seed / 4) % 5 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() % 3
                };
                c.bus.write32(section + 4, slot);
                c.bus.write32(section + 8, (seed / 4 + idx) % 10);
                c.g[A0][0] = (r.next_u32() % 16) as u64;
                c.g[A1][0] = idx as u64;
            },
        );
    }

    use crate::difftest::fakes;

    use std::ptr::addr_of_mut;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc423_fake;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc423_fake_call;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w_5 as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_CTRL;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_MAGIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_RET;

    pub(crate) use crate::game::ai::ai::tests::oc425_machine;

    oc423_fake!(
        oc423_fake_section_is_loaded,
        OC423_PREV_SECTION_IS_LOADED,
        0x0022_e078,
        ee::s05::f_0022e078__generated
    );

    /// Events with 0..3 own sections and optional extra sections (unmapped,
    /// mapped); the section-loaded check scripted per call (loaded or not).
    #[test]
    fn section_man_is_event_loaded_matches() {
        hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E078),
            oc423_fake_section_is_loaded,
            &OC423_PREV_SECTION_IS_LOADED,
        );
        fakes::covered(
            ee::s05::f_0022d278__generated,
            section_man_is_event_loaded,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_d278, 6);
                w_2(c, OC423_CTRL, OC423_MAGIC);
                for i in 0..8u32 {
                    // Mostly loaded, sometimes not.
                    w_2(
                        c,
                        OC423_RET + 4 * i,
                        if r.next_u32() % 4 == 0 { 0 } else { 1 },
                    );
                }
                let event = seed % 3;
                let record = 0x0044_2488 + 0x28 * event;
                w_2(c, record + 4, seed / 3 % 4);
                for i in 0..4u32 {
                    w_2(c, record + 0x10 + 4 * i, r.next_u32() % 8);
                }
                let extra = |k: u32| match k % 3 {
                    0 => 0xffff_ffffu32,
                    1 => 1,
                    _ => 2,
                };
                w_2(c, record + 8, extra(seed / 12));
                w_2(c, record + 0xc, extra(seed / 36));
                for i in 0..3u32 {
                    let entry = 0x0044_2168 + 0x10 * i;
                    w_2(
                        c,
                        entry + 4,
                        if (seed / 7 + i) % 2 == 0 {
                            0xffff_ffff
                        } else {
                            i
                        },
                    );
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = event as u64;
            },
        );
    }

    /// `SectionMan_RequestActivate`: every load state (0..9, and out of range
    /// values), sections with a negative world id, the real jump table (and a
    /// bogus entry for the jump-out), with the world cache's activation and
    /// the queueing faked (some unwinding).
    #[test]
    fn section_man_request_activate_matches() {
        use crate::difftest::put_words;
        oc425_install();
        covered(
            ee::s05::f_0022d478__generated,
            section_man_request_activate,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0022_d478, 2);
                c.pc = 0x0022_d478;
                let me = DATA + 0x1000;
                let i = r.next_u32() % 3;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(i);
                c.bus.write32(me + 0x1b0, DATA + 0x2000);
                c.bus.write32(DATA + 0x2010, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, r.next_u32());
                let section = 0x0044_2168 + 0x10 * i;
                c.bus.write32(
                    section + 4,
                    if seed % 5 == 4 {
                        u32::MAX
                    } else {
                        r.next_u32() % 40
                    },
                );
                let states = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, u32::MAX];
                let state = states[(seed % 12) as usize];
                c.bus.write32(section + 8, state);
                let table = [
                    0x22d4dc, 0x22d53c, 0x22d570, 0x22d50c, 0x22d570, 0x22d53c, 0x22d524, 0x22d4f4,
                    0x22d570,
                ];
                put_words(c, 0x0047_b3d0, &table);
                if seed % 17 == 16 && state < 9 {
                    // A table entry the function does not know: jump out.
                    c.bus.write32(0x0047_b3d0 + 4 * state, 0x0022_d5a0);
                }
            },
        );
    }

    pub(crate) use crate::game::be::belibrary::tests::oc431_install;
    pub(crate) use crate::game::be::belibrary::tests::oc431_machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    /// Per-location section lists (0x28 bytes per location: +4 count, +8 extra
    /// section, +0xc second extra section, +0x10 ids) and per-location modes.
    pub(crate) const OC431_LOCATIONS: u32 = 0x0044_2488;

    pub(crate) const OC431_LOCATION_MODES: u32 = 0x0044_2820;

    /// `SectionMan_LoadEvent`: a location keeps 0 to 4 sections (some of the
    /// protected 43..48), has or lacks the extra sections, and the mode is
    /// the caller's or (7) the location's own. Requests are fakes; the real
    /// `SectionMan_AdvanceEventSwitchSync` and `SectionMan_SelectPeakLighting`
    /// run (their jump tables are empty in the test RAM).
    #[test]
    fn section_man_load_event_matches() {
        oc431_install();
        covered(
            ee::s05::f_0022d088__generated,
            section_man_load_event,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x22d088, 60, 0x0022_d088);
                let this = OC431_BASE + 0x100;
                let idx = seed % 3;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = [7u32, 0, 3, 7][(r.next_u32() % 4) as usize] as u64;
                c.bus.write32(this + 0x1b4, r.next_u32() % 8);
                for i in 0..3u32 {
                    let loc = OC431_LOCATIONS + 0x28 * i;
                    c.bus.write32(loc + 4, r.next_u32() % 5);
                    let extra = [u32::MAX, 5, 49, 44][(r.next_u32() % 4) as usize];
                    c.bus.write32(loc + 8, extra);
                    let second = [u32::MAX, 9, 20][(r.next_u32() % 3) as usize];
                    c.bus.write32(loc + 0xc, second);
                    for k in 0..4u32 {
                        let id = [3u32, 43, 46, 12, 49, 0][(r.next_u32() % 6) as usize];
                        c.bus.write32(loc + 0x10 + 4 * k, id);
                    }
                    c.bus
                        .write32(OC431_LOCATION_MODES + 8 * i + 4, r.next_u32() % 6);
                }
            },
        );
    }

    pub(crate) use crate::game::ai::ai::ai_load_anim_data_sources;
    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;
    pub(crate) use crate::game::be::bedbreward::rewards_file_load_2 as rewards_file_load;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::default_3_chase_camera_algorithm_dtor;
    pub(crate) use crate::game::fe::debugfe::fe_ui_input_dtor;
    pub(crate) use crate::game::input::inputparse::input_parser_parse_all;
    pub(crate) use crate::game::main::gameload::game_module_enter_2 as game_module_enter;
    pub(crate) use crate::game::main::sectionman::section_man_select_peak_lighting;
    pub(crate) use crate::game::object::modifierblock::sky_box_set_section_2 as sky_box_set_section;
    pub(crate) use crate::game::world::world::tests::w_2 as w_3;
    pub(crate) use crate::game::world::world::tests::BASE_2 as BASE;
    pub(crate) use crate::game::world::world::world_load_2 as world_load;
    use ssx3_ee_rt::funcs::register_functions;

    oc432_fake!(
        fake_get_be,
        PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );

    oc432_fake!(
        fake_be_get_interface,
        PREV_BE_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );

    oc432_fake!(
        fake_streamer_get_section,
        PREV_STREAMER_GET_SECTION,
        ee::s13::f_003a9820__generated,
        0x003a_9820
    );

    oc432_fake!(
        fake_streamer_section_count,
        PREV_STREAMER_SECTION_COUNT,
        ee::s13::f_003a9800__generated,
        0x003a_9800,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 4) as u64;
        }
    );

    oc432_fake!(
        fake_get_section_entry,
        PREV_GET_SECTION_ENTRY,
        ee::s01::f_00144d38__generated,
        0x0014_4d38
    );

    oc432_fake!(
        fake_strcmp,
        PREV_STRCMP,
        ee::s15::f_004165a8__generated,
        0x0041_65a8,
        |c: &mut Cpu| {
            let n = c.bus.read32(fakes::LOG_COUNT);
            c.g[V0][0] = (n % 3 != 0) as u64;
        }
    );

    oc432_fake!(
        fake_section_init_node_pool,
        PREV_SECTION_INIT_NODE_POOL,
        ee::s05::f_0022e228__generated,
        0x0022_e228
    );

    /// `SectionMan_Init`: the BE getters, the streamer's section accessors,
    /// strcmp and the node pool init are logging fakes (the section count is
    /// 0..2 and strcmp matches on some calls, so the map update and both loop
    /// back edges run; fake unwinds exercise the call-unwind exits).
    #[test]
    fn section_man_init_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9820),
            fake_streamer_get_section,
            &PREV_STREAMER_GET_SECTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9800),
            fake_streamer_section_count,
            &PREV_STREAMER_SECTION_COUNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144D38),
            fake_get_section_entry,
            &PREV_GET_SECTION_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            fake_strcmp,
            &PREV_STRCMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E228),
            fake_section_init_node_pool,
            &PREV_SECTION_INIT_NODE_POOL,
        );
        fakes::covered(
            ee::s05::f_0022cd40__generated,
            section_man_init,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22cd40, 60);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus.write32(OC432_CTRL + 4, r.next_u32() % 3);
                c.pc = 0x0022_cd40;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x1b0, DATA + 0x400);
                c.bus.write32(DATA + 0x410, DATA + 0x500);
                c.bus.write32(DATA + 0x500, DATA + 0x600);
            },
        );
    }

    /// Stub controls of the parser test (`DATA + 0x200000` and up).
    pub(crate) const CTRL: u32 = DATA + 0x20_0000;

    /// Set in the child process of the tests whose callees are real guest
    /// code (`call_ptr` targets): there the function table is empty, so every
    /// such callee lands in the logging fake, and the direct callees' override
    /// slots hold logging fakes too. A table swap in the shared test process
    /// would race with the other tests' `fresh()`.
    pub(crate) const OC435_CHILD_ENV: &str = "SSX3_OC435_CHILD";

    /// Run test `name` of this module in a child process (the same test
    /// binary, `--exact`), where `body` runs against the stub table.
    pub(crate) fn in_child(name: &str, body: fn()) {
        if std::env::var_os(OC435_CHILD_ENV).is_some() {
            install_stubs();
            return body();
        }
        let path = format!(
            "{}::{name}",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([path.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC435_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// Per-seed machine of the child tests: scrambled registers, the fake
    /// hooks and an empty function table.
    pub(crate) fn child_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        register_functions(&[]);
        r
    }

    /// A logging stub for a direct callee (reached through its override slot).
    macro_rules! oc435_stub {
        ($fake:ident, $prev:ident, $addr:expr) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $fake(c: &mut Cpu) {
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra)
            }
        };
    }

    oc435_stub!(stub_request_unload, PREV_REQUEST_UNLOAD, 0x0022_d598);

    oc435_stub!(stub_sky_box_clear, PREV_SKY_BOX_CLEAR, 0x0035_3d98);

    oc435_stub!(stub_rewards_parse, PREV_REWARDS_PARSE, 0x0015_a818);

    oc435_stub!(stub_world_cache_reset, PREV_WORLD_CACHE_RESET, 0x003a_7f90);

    oc435_stub!(stub_world_cache_load, PREV_WORLD_CACHE_LOAD, 0x003a_7fc0);

    oc435_stub!(stub_section_count, PREV_SECTION_COUNT, 0x003a_9800);

    oc435_stub!(
        stub_script_cache_alloc,
        PREV_SCRIPT_CACHE_ALLOC,
        0x003a_c800
    );

    pub(crate) static PREV_NEXT_TOKEN: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_PARSE_STATEMENT: AtomicUsize = AtomicUsize::new(0);

    /// `InputParser_NextToken`: the token kind (`+0x98`) becomes the word at
    /// `CTRL`.
    pub(crate) fn stub_next_token(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        let p = c.g[A0][0] as u32;
        fakes::fake(c, 0x0032_1af0, ra);
        if c.pc == ra {
            let kind = c.bus.read32(CTRL);
            w_3(c, p.wrapping_add(0x98), kind);
        }
    }

    /// `InputParser_ParseStatement`: consumes one of the `CTRL + 4` rounds
    /// left; the last one ends the input (kind 0).
    pub(crate) fn stub_parse_statement(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        let p = c.g[A0][0] as u32;
        fakes::fake(c, 0x0032_30e0, ra);
        if c.pc == ra {
            let left = c.bus.read32(CTRL + 4);
            if left == 0 {
                w_3(c, p.wrapping_add(0x98), 0);
            } else {
                w_3(c, CTRL + 4, left - 1);
            }
        }
    }

    /// Child process only: put the logging stubs in the direct callees' slots.
    pub(crate) fn install_stubs() {
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D598),
            stub_request_unload,
            &PREV_REQUEST_UNLOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353D98),
            stub_sky_box_clear,
            &PREV_SKY_BOX_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A818),
            stub_rewards_parse,
            &PREV_REWARDS_PARSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7F90),
            stub_world_cache_reset,
            &PREV_WORLD_CACHE_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7FC0),
            stub_world_cache_load,
            &PREV_WORLD_CACHE_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9800),
            stub_section_count,
            &PREV_SECTION_COUNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AC800),
            stub_script_cache_alloc,
            &PREV_SCRIPT_CACHE_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321AF0),
            stub_next_token,
            &PREV_NEXT_TOKEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003230E0),
            stub_parse_statement,
            &PREV_PARSE_STATEMENT,
        );
    }

    // ---- tests whose callees are real guest code: run in a child process

    pub(crate) fn ai_load_anim_data_sources_checked() {
        covered(
            ee::s00::f_0012b280__generated,
            ai_load_anim_data_sources,
            100,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x12b280, 6);
                w_3(c, GAME_GP_TEST.wrapping_add(0xd08), BASE + 0x1000);
                w_3(c, GAME_GP_TEST.wrapping_add(0xd8c), r.next_u32());
            },
        );
    }

    /// `AI_LoadAnimDataSources`: the six guest calls (mount, data source
    /// allocation and construction, registration, post hook, unmount) with
    /// their arguments, the table store, unwinds at any call.
    #[test]
    fn ai_load_anim_data_sources_matches() {
        in_child(
            "ai_load_anim_data_sources_matches",
            ai_load_anim_data_sources_checked,
        );
    }

    pub(crate) const GAME_GP_TEST: u32 = crate::difftest::GAME_GP;

    pub(crate) fn section_man_apply_sky_checked() {
        covered(
            ee::s05::f_0022de98__generated,
            section_man_apply_sky,
            200,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x22de98, 8);
                let (this, sky_box, obj) = (BASE + 0x100, BASE + 0x200, BASE + 0x300);
                c.g[A0][0] = this as u64;
                w_3(c, this + SECTION_MAN_SKY_ACTIVE, (seed % 4 != 0) as u32);
                w_3(c, this + SECTION_MAN_SKY_PENDING, (seed % 5 != 0) as u32);
                let current = [0x2c_u32, 0x2e, 0x30, 5, 0xffff_ffff, 0x2d][(seed / 3) as usize % 6];
                w_3(c, this + SECTION_MAN_SKY_CURRENT, current);
                w_3(c, this + SECTION_MAN_SKY_BOX, sky_box);
                w_3(c, sky_box + 0x14, obj);
                let section = r.next_u32() & 0xff;
                w_3(c, obj + 0x10, section);
                // The sky table: 16-byte entries from 0x442168, word 1 the section.
                let entry = 0x44_2168u32.wrapping_add(current << 4);
                w_3(
                    c,
                    entry + 4,
                    if seed % 2 == 0 { section } else { section + 1 },
                );
            },
        );
    }

    /// `SectionMan_ApplySky`: inactive and nothing pending (both skipped),
    /// the unload loop with the current sky in or out of 0x2c..0x30, and the
    /// sky box cleared (section differs) or kept (same).
    #[test]
    fn section_man_apply_sky_matches() {
        in_child(
            "section_man_apply_sky_matches",
            section_man_apply_sky_checked,
        );
    }

    pub(crate) fn rewards_file_load_checked() {
        covered(
            ee::s01::f_0015ad58__generated,
            rewards_file_load,
            100,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x15ad58, 5);
                let gp = GAME_GP_TEST;
                let (out, world) = (BASE + 0x100, BASE + 0x200);
                c.g[A0][0] = out as u64;
                c.g[A1][0] = [0u64, 0, 1, 0x77][seed as usize % 4];
                w_3(c, gp.wrapping_sub(0x848), world);
                w_3(c, world + 0x7c, r.next_u32());
                w_3(c, gp.wrapping_sub(0x1e80), r.next_u32());
            },
        );
    }

    /// `RewardsFile_Load`: the synchronous read-and-parse path (`a1 == 0`) and
    /// the asynchronous acquire (`a1 != 0`), the result stored in `*out`.
    #[test]
    fn rewards_file_load_matches() {
        in_child("rewards_file_load_matches", rewards_file_load_checked);
    }

    pub(crate) fn sky_box_set_section_checked() {
        covered(
            ee::s11::f_00353cf0__generated,
            sky_box_set_section,
            200,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x353cf0, 2);
                let this = BASE + 0x100;
                let (p, a, b, c_arr, table) = (
                    BASE + 0x400,
                    BASE + 0x500,
                    BASE + 0x600,
                    BASE + 0x1000,
                    BASE + 0x700,
                );
                c.g[A0][0] = this as u64;
                let section = [0xffff_ffffu32, 0xffff_fffb, 0, 1, 2, 3, 0x102][seed as usize % 7];
                c.g[A1][0] = section as u64;
                // The first fake call (0x2d1bd8) returns `p`.
                w_3(c, LOG_RESULTS, p);
                w_3(c, p, a);
                w_3(c, a, b);
                w_3(c, b + 0x14, c_arr);
                w_3(c, a + 8, table);
                for i in 0..4u32 {
                    // Mesh counts: some zero or negative.
                    c.bus.write16(
                        c_arr + 0x58 * i + 0x26,
                        [3u16, 0, 0xfffe, 1][((seed + i) % 4) as usize],
                    );
                    let entry = BASE + 0x800 + 0x20 * i;
                    w_3(
                        c,
                        table + 4 * i,
                        if (seed + i) % 5 == 1 { 0 } else { entry },
                    );
                    let words = BASE + 0xa00 + 0x10 * i;
                    w_3(c, entry + 0x1c, words);
                    w_3(
                        c,
                        words,
                        if (seed + i) % 3 == 0 {
                            0xff
                        } else {
                            r.next_u32()
                        },
                    );
                }
            },
        );
    }

    /// `SkyBox_SetSection`: a negative section clears the box; otherwise the
    /// mesh table lookup (no meshes, no table entry, a zero or non-zero
    /// size word) decides what is stored at `+0x14`.
    #[test]
    fn sky_box_set_section_matches() {
        in_child("sky_box_set_section_matches", sky_box_set_section_checked);
    }

    pub(crate) fn section_man_select_peak_lighting_checked() {
        covered(
            ee::s05::f_0022e180__generated,
            section_man_select_peak_lighting,
            200,
            &|c, seed| {
                child_machine(c, seed, 0x22e180, 1);
                c.pc = 0x0022_e180;
                // The jr table (0x47b4e0): case -> arm (the real mapping), with
                // one bogus target at index 21 for half of the seeds.
                let arms: [u32; 22] = [
                    0x1fc, 0x1ac, 0x1c0, 0x1d4, 0x1e8, 0x1fc, 0x1d4, 0x1e8, 0x1fc, 0x1c0, 0x1e8,
                    0x1ac, 0x1c0, 0x1e8, 0x1fc, 0x1d4, 0x1e8, 0x1fc, 0x1ac, 0x1c0, 0x1d4, 0x1e8,
                ];
                for (i, a) in arms.iter().enumerate() {
                    w_3(c, 0x47_b4e0 + 4 * i as u32, 0x0022_e000 + a);
                }
                if (seed / 14) % 2 == 0 {
                    w_3(c, 0x47_b4e0 + 4 * 21, 0x0022_e100);
                }
                c.g[A1][0] = [
                    0u64,
                    1,
                    2,
                    3,
                    4,
                    5,
                    9,
                    11,
                    14,
                    18,
                    21,
                    0x16,
                    0x40,
                    0xffff_ffff,
                ][seed as usize % 14];
            },
        );
    }

    /// `SectionMan_SelectPeakLighting`: every table arm, the default, the
    /// out-of-range sky and a table target the original sends to `jump_out`.
    #[test]
    fn section_man_select_peak_lighting_matches() {
        in_child(
            "section_man_select_peak_lighting_matches",
            section_man_select_peak_lighting_checked,
        );
    }

    pub(crate) fn default_3_chase_camera_algorithm_dtor_checked() {
        covered(
            ee::s02::f_00176da8__generated,
            default_3_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                child_machine(c, seed, 0x176da8, 1);
                c.g[A0][0] = (BASE + 0x100) as u64;
            },
        );
    }

    /// `Default_3ChaseCameraAlgorithm_dtor`: vtable store, base destructor.
    #[test]
    fn default_3_chase_camera_algorithm_dtor_matches() {
        in_child(
            "default_3_chase_camera_algorithm_dtor_matches",
            default_3_chase_camera_algorithm_dtor_checked,
        );
    }

    pub(crate) fn game_module_enter_checked() {
        covered(
            ee::s06::f_00230050__generated,
            game_module_enter,
            100,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x230050, 9);
                let gp = GAME_GP_TEST;
                let (this, world, obj, vt) =
                    (BASE + 0x100, BASE + 0x200, BASE + 0x300, BASE + 0x400);
                c.g[A0][0] = this as u64;
                for off in [0x10u32, 0x94] {
                    w_3(c, this + off, r.next_u32());
                }
                w_3(c, gp.wrapping_sub(0x848), world);
                w_3(c, world + 0x8c, r.next_u32());
                for off in [0x84cu32, 0x854] {
                    w_3(c, gp.wrapping_sub(off), r.next_u32());
                }
                for off in [0x21e4u32, 0x410] {
                    w_3(c, gp.wrapping_add(off), r.next_u32());
                }
                w_3(c, gp.wrapping_add(0x1158), obj);
                w_3(c, obj + 4, vt);
                c.bus.write16(vt + 0x88, (r.next_u32() & 0xfffc) as u16);
                w_3(c, vt + 0x8c, FAKE);
                c.bus.write16(vt + 0x20, (r.next_u32() & 0xfffc) as u16);
                w_3(c, vt + 0x24, FAKE + 0x10);
            },
        );
    }

    /// `GameModule_Enter`: the nine guest calls (two of them virtual) in order
    /// with their arguments, the three flags set, unwinds at any call.
    #[test]
    fn game_module_enter_matches() {
        in_child("game_module_enter_matches", game_module_enter_checked);
    }

    pub(crate) fn world_load_checked() {
        covered(
            ee::s13::f_003a6ae0__generated,
            world_load,
            100,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x3a6ae0, 8);
                let world = BASE + 0x100;
                c.g[A0][0] = world as u64;
                for off in [0u32, 4, 8] {
                    w_3(c, world + off, r.next_u32());
                }
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// `World_Load`: the eight guest calls (reset, load, two section counts,
    /// the two allocations, the stack-record pair) with their arguments.
    #[test]
    fn world_load_matches() {
        in_child("world_load_matches", world_load_checked);
    }

    pub(crate) fn fe_ui_input_dtor_checked() {
        covered(
            ee::s04::f_001dd530__generated,
            fe_ui_input_dtor,
            100,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x1dd530, 1);
                c.g[A0][0] = (BASE + 0x100) as u64;
                c.g[A1][0] = (r.next_u32() & !1 | (seed & 1)) as u64;
            },
        );
    }

    /// `FEUIInput_dtor`: vtable store; delete only with the flag bit set.
    #[test]
    fn fe_ui_input_dtor_matches() {
        in_child("fe_ui_input_dtor_matches", fe_ui_input_dtor_checked);
    }

    pub(crate) fn input_parser_parse_all_checked() {
        covered(
            ee::s10::f_00323098__generated,
            input_parser_parse_all,
            200,
            &|c, seed| {
                child_machine(c, seed, 0x323098, 6);
                c.g[A0][0] = (BASE + 0x100) as u64;
                // First token: end of input (0) or a statement start; then 0..3
                // further statements before the input ends.
                w_3(c, CTRL, if seed % 4 == 0 { 0 } else { 1 + seed % 5 });
                w_3(c, CTRL + 4, (seed / 4) % 4);
            },
        );
    }

    /// `InputParser_ParseAll`: empty input, and 1..4 statements parsed.
    #[test]
    fn input_parser_parse_all_matches() {
        in_child(
            "input_parser_parse_all_matches",
            input_parser_parse_all_checked,
        );
    }

    use crate::difftest::put_words;
    pub(crate) use crate::game::camera::camera::tests::SEEDS_5 as SEEDS_3;

    /// `SectionMan_AdvanceEventSwitchSync`: `arg` zero or not, states 0..4
    /// (every jump-table case) and states out of range (5, 6, large,
    /// negative).
    #[test]
    fn oc436_section_man_advance_event_switch_sync_matches() {
        check(
            ee::s05::f_0022d668__generated,
            oc436_section_man_advance_event_switch_sync,
            SEEDS_3,
            &|c, seed| {
                let mut r = Rng(0x22d6_68 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0022_d668;
                let table = [
                    0x0022_d698,
                    0x0022_d6a8,
                    0x0022_d6b4,
                    0x0022_d6bc,
                    0x0022_d6bc,
                ];
                put_words(c, OC436_SM_SWITCH_TABLE, &table);
                let sm = DATA + 0x100;
                c.g[A0][0] = sx(sm);
                let arg = if seed % 5 == 0 { 0 } else { r.next_u32() | 1 };
                c.g[A1][0] = sx(arg);
                let state = match seed % 9 {
                    0..=4 => seed % 9,
                    5 => 5,
                    6 => 6,
                    7 => 0x8000_0000 | seed,
                    _ => r.next_u32() % 3,
                };
                c.bus.write32(sm + OC436_SM_SWITCH_STATE, state);
                let old = r.next_u32();
                c.bus.write32(sm + OC436_SM_SWITCH_ARG, old);
            },
        );
    }

    pub(crate) use crate::game::main::all_in_one_main::tests::object_machine;

    #[test]
    fn section_man_is_event_switch_overlay_done_matches() {
        check(
            ee::s05::f_0022dfd0__generated,
            section_man_is_event_switch_overlay_done,
            30,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0xdfd0);
                c.bus.write32(DATA + 0x1b0, r.next_u32());
            },
        );
    }

    #[test]
    fn section_man_reload_event_and_restart_matches() {
        check(
            ee::s05::f_0022e118__generated,
            section_man_reload_event_and_restart,
            60,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0xe118);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
                c.bus.write8(0x0053_0000 + 0x5c11, (seed % 2) as u8);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn game_module_render_ui_matches() {
        covered(
            ee::s05::f_0022c830__generated,
            game_module_render_ui,
            OC515_SEEDS,
            &|c, seed| {
                oc515_setup(c, seed);
            },
        );
    }

    #[test]
    fn section_man_request_load_matches() {
        check(
            ee::s05::f_0022d3d8__generated,
            section_man_request_load,
            200,
            &|c, seed| {
                machine(c, seed, 0xd3d8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0022d3d8;
            },
        );
    }

    pub(crate) use crate::game::bx::bxstringctor::tests::oc804_fixture;
    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_CTRL;
    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_MAGIC;
    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_SELF;
    pub(crate) use crate::game::bx::bxstringctor::tests::OC804_WORLD;
    pub(crate) use crate::game::fe::festateloadhint::tests::oc804_force;
    pub(crate) use crate::game::fe::festateloadhint::tests::oc804_mail;

    pub(crate) use crate::game::fe::festateloadhint::tests::OC804_Z;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::OC804_X;

    pub(crate) const OC804_Y: u32 = DATA + 0x27_0000;

    pub(crate) const OC804_IFACE: u32 = DATA + 0x29_0000;

    pub(crate) const OC804_IFVT: u32 = DATA + 0x2a_0000;

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    pub(crate) fn oc804_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc804_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc804_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC804_CTRL) != OC804_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc804_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc804_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc804_fakes!(
        (
            oc804_fake_003e22f0,
            OC804_PREV_003E22F0,
            ee::s14::OVERRIDE_003E22F0,
            ee::s14::f_003e22f0__generated,
            0x003e22f0
        ),
        (
            oc804_fake_0026adf0,
            OC804_PREV_0026ADF0,
            ee::s07::OVERRIDE_0026ADF0,
            ee::s07::f_0026adf0__generated,
            0x0026adf0
        ),
        (
            oc804_fake_0012b180,
            OC804_PREV_0012B180,
            ee::s00::OVERRIDE_0012B180,
            ee::s00::f_0012b180__generated,
            0x0012b180
        ),
        (
            oc804_fake_00144df0,
            OC804_PREV_00144DF0,
            ee::s01::OVERRIDE_00144DF0,
            ee::s01::f_00144df0__generated,
            0x00144df0
        ),
        (
            oc804_fake_001451e8,
            OC804_PREV_001451E8,
            ee::s01::OVERRIDE_001451E8,
            ee::s01::f_001451e8__generated,
            0x001451e8
        ),
        (
            oc804_fake_0014dc80,
            OC804_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc804_fake_0014dd58,
            OC804_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc804_fake_0014de68,
            OC804_PREV_0014DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014de68
        ),
        (
            oc804_fake_00231250,
            OC804_PREV_00231250,
            ee::s06::OVERRIDE_00231250,
            ee::s06::f_00231250__generated,
            0x00231250
        ),
        (
            oc804_fake_00238160,
            OC804_PREV_00238160,
            ee::s06::OVERRIDE_00238160,
            ee::s06::f_00238160__generated,
            0x00238160
        ),
        (
            oc804_fake_00146e98,
            OC804_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc804_fake_00148410,
            OC804_PREV_00148410,
            ee::s01::OVERRIDE_00148410,
            ee::s01::f_00148410__generated,
            0x00148410
        ),
        (
            oc804_fake_001484e0,
            OC804_PREV_001484E0,
            ee::s01::OVERRIDE_001484E0,
            ee::s01::f_001484e0__generated,
            0x001484e0
        ),
        (
            oc804_fake_001485b0,
            OC804_PREV_001485B0,
            ee::s01::OVERRIDE_001485B0,
            ee::s01::f_001485b0__generated,
            0x001485b0
        ),
        (
            oc804_fake_00148680,
            OC804_PREV_00148680,
            ee::s01::OVERRIDE_00148680,
            ee::s01::f_00148680__generated,
            0x00148680
        ),
        (
            oc804_fake_00148750,
            OC804_PREV_00148750,
            ee::s01::OVERRIDE_00148750,
            ee::s01::f_00148750__generated,
            0x00148750
        ),
        (
            oc804_fake_00148820,
            OC804_PREV_00148820,
            ee::s01::OVERRIDE_00148820,
            ee::s01::f_00148820__generated,
            0x00148820
        ),
        (
            oc804_fake_001488f0,
            OC804_PREV_001488F0,
            ee::s01::OVERRIDE_001488F0,
            ee::s01::f_001488f0__generated,
            0x001488f0
        ),
        (
            oc804_fake_00148950,
            OC804_PREV_00148950,
            ee::s01::OVERRIDE_00148950,
            ee::s01::f_00148950__generated,
            0x00148950
        ),
        (
            oc804_fake_002c26d0,
            OC804_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc804_fake_00317670,
            OC804_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc804_fake_0039c428,
            OC804_PREV_0039C428,
            ee::s13::OVERRIDE_0039C428,
            ee::s13::f_0039c428__generated,
            0x0039c428
        ),
        (
            oc804_fake_0039d860,
            OC804_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc804_fake_003a0c50,
            OC804_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc804_fake_003a0e90,
            OC804_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc804_fake_00417828,
            OC804_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc804_fake_001e3c00,
            OC804_PREV_001E3C00,
            ee::s04::OVERRIDE_001E3C00,
            ee::s04::f_001e3c00__generated,
            0x001e3c00
        ),
        (
            oc804_fake_001f7548,
            OC804_PREV_001F7548,
            ee::s04::OVERRIDE_001F7548,
            ee::s04::f_001f7548__generated,
            0x001f7548
        ),
        (
            oc804_fake_001f7738,
            OC804_PREV_001F7738,
            ee::s04::OVERRIDE_001F7738,
            ee::s04::f_001f7738__generated,
            0x001f7738
        ),
        (
            oc804_fake_00317d70,
            OC804_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc804_fake_0039e2a0,
            OC804_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039e2a0
        ),
        (
            oc804_fake_0039f320,
            OC804_PREV_0039F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc804_fake_001fdbf0,
            OC804_PREV_001FDBF0,
            ee::s05::OVERRIDE_001FDBF0,
            ee::s05::f_001fdbf0__generated,
            0x001fdbf0
        ),
        (
            oc804_fake_001fe968,
            OC804_PREV_001FE968,
            ee::s05::OVERRIDE_001FE968,
            ee::s05::f_001fe968__generated,
            0x001fe968
        ),
        (
            oc804_fake_002006b8,
            OC804_PREV_002006B8,
            ee::s05::OVERRIDE_002006B8,
            ee::s05::f_002006b8__generated,
            0x002006b8
        ),
        (
            oc804_fake_00212138,
            OC804_PREV_00212138,
            ee::s05::OVERRIDE_00212138,
            ee::s05::f_00212138__generated,
            0x00212138
        ),
    );

    /// `SectionMan_OnEventTrigger(this, mode)`: configures the race back end
    /// for a mode (the jump table of modes 0 to 6), then calls a virtual method
    /// of the interface and starts the game mode. The early exits come from the
    /// global flags (`0x535bc8`: bytes 0x48/0x49), `this+0x1bc` and mode 1.
    #[test]
    fn section_man_on_event_trigger_matches() {
        oc804_install_fakes();
        fakes::covered(
            ee::s05::f_0022d6c8__generated,
            section_man_on_event_trigger_2,
            220,
            &|c, seed| {
                elf_image(c);
                let mut r = oc804_fixture(c, seed, 0x0022_d6c8, 10);
                let counts = [0xdu32, 0xe, 0xf, 0x10, 0x11, 0, 0x1c];
                c.bus.write8(0x0053_5bc8 + 0x49, (seed % 7 == 0) as u8);
                c.bus
                    .write8(0x0053_5bc8 + 0x48, if seed % 5 == 0 { 3 } else { 4 });
                fill(c, &mut r, OC804_SELF, 0x80);
                c.bus
                    .write32(OC804_SELF + 0x1bc, counts[(r.next_u32() % 7) as usize]);
                c.bus.write32(OC804_SELF + 0x1b0, r.next_u32());
                // The world: world+0x84 = X; X+0xc = Y, X+0x78 = Z, X+0x214 = 4.
                fill(c, &mut r, OC804_X, 0x100);
                fill(c, &mut r, OC804_Y, 8);
                fill(c, &mut r, OC804_WORLD, 0x40);
                c.bus.write32(OC804_WORLD + 0x84, OC804_X);
                c.bus.write32(OC804_X + 0xc, OC804_Y);
                c.bus.write32(OC804_X + 0x78, OC804_Z);
                c.bus
                    .write32(OC804_X + 0x214, if seed % 4 == 3 { 5 } else { 4 });
                c.bus
                    .write32(OC804_Z + 0x1bc, counts[(r.next_u32() % 7) as usize]);
                // The interface the back-end calls go through: its method.
                c.bus.write32(OC804_IFACE + 0xc, OC804_IFVT);
                c.bus.write16(OC804_IFVT + 8, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC804_IFVT + 0xc, fakes::FAKE + 0x20);
                oc804_force(c, 0x0014_dd58, OC804_IFACE);
                c.g[A0][0] = sx(OC804_SELF);
                c.g[A1][0] = (seed % 9) as u64;
                c.pc = 0x0022_d6c8;
            },
        );
    }
}
