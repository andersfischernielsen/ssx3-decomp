//! `src/main/game.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
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
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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
}
