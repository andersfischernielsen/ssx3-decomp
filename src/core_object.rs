//! Hooks for the Tricky edits the fork made in place in the object/render
//! files (`game/object/{railmodifier,flagset,debouncenode,movenode}.rs`,
//! `game/render/graphicsman.rs`).
//!
//! | function | address | kind | what changed | uncertain |
//! |---|---|---|---|---|
//! | ModObj_v41 | 0x355858 | chain | contact timer seconds (f12) replaced by `custom_animation_debounce::contact_seconds` for the WScriptMan rearm call; f12 restored after | none |
//! | Counter_Update | 0x343820 | chain | `custom_counter_trace::completed` when the count (+0x1c) is 0 | the fork's re-entry restructure is not reproduced (see below) |
//! | Counter_HandleEvent | 0x343868 | chain | `custom_counter_trace::{before,event}` around the call (entry/exit only) | none |
//! | Fence_Update | 0x34ce48 | chain | `custom_fences::prepare(fence)` before the neighbour delays | called at entry instead of after the prologue: same memory effect |
//! | TexMan_InitTable | 0x3670e0 | chain, own body when `world_texture_slots() != 1500` | first free-list entry at `8 + slots * 4`, `1999 - slots` links | at 1500 slots (the default) PREV runs unchanged |
//! | RollerModifier_Update | 0x35e850 | replacement | `custom_roller_runtime::sleeping` guard, else `custom_roller_update::roller_modifier_update` | do NOT also install `custom_roller_update::OVERRIDES` (same address) |
//! | Emitter_Render | 0x3708c0 | replacement | `0x4250xxxx` kinds use `kind & 31` as the layer; `0x5458xxxx` sprite values are resident world texture ids | copied from upstream's current body (equal to the fork's except the edits) |
//! | ModObj_Render | 0x356298 | in `core_race::draw_model` | `A2 = custom_crowds::materials or custom_movies::materials` before the draw call | done at the entry of the draw call it makes (graphics manager slot 0x2f8, RA 0x3565d8, the object in S4) |
//!
//! Not converted, nothing to hook: `ModObj_AddRollerModifier` (0x355db8) and
//! `Counter_ctor` (0x343768) changed in the fork only to be re-enterable at
//! the return of each native call (the `match c.pc` / `jump_out` form); no
//! Tricky call was added. The same restructure of `Counter_Update` and
//! `Fence_Update` is not reproduced: the wrappers pass a mid-function `c.pc`
//! straight to PREV. The doc/test-only changes in `particle.rs`, `lensfx.rs`,
//! `worldpaintermanquery.rs` and `debouncenode.rs`
//! (`SplineModifier_EvalPathAtDistance`: doc comment and a test) have no code
//! to hook.

use ssx3_game::abi;
use ssx3_game::call::{call_ptr, guest, Frame};
use ssx3_game::events::{deliver_event_here, event_due};
use ssx3_game::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_e850, roller_modifier_update, "RollerModifier_Update"),
    (0x0037_08c0, emitter_render, "Emitter_Render"),
];

pub static CHAINS: &[crate::Chain] = &[
    crate::Chain { addr: 0x0035_5858, f: mod_obj_v41, prev: &PREV_MOD_OBJ_V41, name: "ModObj_v41" },
    crate::Chain { addr: 0x0034_3820, f: counter_update, prev: &PREV_COUNTER_UPDATE, name: "Counter_Update" },
    crate::Chain { addr: 0x0034_3868, f: counter_handle_event, prev: &PREV_COUNTER_EVENT, name: "Counter_HandleEvent" },
    crate::Chain { addr: 0x0034_ce48, f: fence_update, prev: &PREV_FENCE_UPDATE, name: "Fence_Update" },
    crate::Chain { addr: 0x0036_70e0, f: tex_man_init_table, prev: &PREV_TEX_MAN_INIT, name: "TexMan_InitTable" },
];

static PREV_MOD_OBJ_V41: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_COUNTER_UPDATE: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_COUNTER_EVENT: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_FENCE_UPDATE: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_TEX_MAN_INIT: ssx3_plugin::Prev = ssx3_plugin::Prev::new();

#[inline]
fn r32(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn p(a: u32, off: u32) -> u32 { a.wrapping_add(off) }
#[inline]
fn gp(c: &Cpu, off: i32) -> u32 { r32(c, GP).wrapping_add(off as u32) }
#[inline]
fn lw(c: &mut Cpu, a: u32) -> u64 { c.lw(a) }
#[inline]
fn lh(c: &mut Cpu, a: u32) -> u64 { c.lh(a) }
#[inline]
fn ldf(c: &mut Cpu, f: usize, a: u32) { c.lwc1(f, a); }
#[inline]
fn vslot(c: &mut Cpu, vt: u32, off: u32) -> (u64, u64) {
    (lh(c, p(vt, off)), lw(c, p(vt, off + 4)))
}

/// The 20-byte render-state copy (two unaligned 8-byte moves and a word).
fn copy_render_state(c: &mut Cpu, r0: usize, r1: usize, word: usize, src: u32, dst: u32) {
    c.ldl(r0, p(src, 7));
    c.ldr(r0, src);
    c.ldl(r1, p(src, 0xf));
    c.ldr(r1, p(src, 8));
    c.g[word][0] = lw(c, p(src, 0x10));
    c.sdl(r0, p(dst, 0x1b));
    c.sdr(r0, p(dst, 0x14));
    c.sdl(r1, p(dst, 0x23));
    c.sdr(r1, p(dst, 0x1c));
    let w = c.g[word][0];
    c.sw(p(dst, 0x24), w);
}

mod ctx {
    pub const VTABLE: u32 = 0x10d8;
    pub const TOP: u32 = 0xe84;
    pub const TEX_IDS: u32 = 0xf50;
}

// ---------------------------------------------------------------------------
// Chains.

/// `ModObj_v41` (0x355858): the contact timer of an animation debounce node.
fn mod_obj_v41(c: &mut Cpu) {
    let original = c.f[12];
    // Only the entry call may be rewritten (it checks ra against the caller).
    if c.pc == 0x0035_5858 {
        if let Some(seconds) = crate::custom_animation_debounce::contact_seconds(c) {
            c.f[12] = seconds;
        }
    }
    PREV_MOD_OBJ_V41.call(c);
    c.f[12] = original;
}

/// `Counter_Update` (0x343820): trace the completion.
fn counter_update(c: &mut Cpu) {
    if c.pc == 0x0034_3820 {
        let node = r32(c, A0);
        if c.bus.read32(node.wrapping_add(0x1c)) == 0 {
            crate::custom_counter_trace::completed(c, node);
        }
    }
    PREV_COUNTER_UPDATE.call(c);
}

/// `Counter_HandleEvent` (0x343868): trace the state before and after.
fn counter_handle_event(c: &mut Cpu) {
    if c.pc != 0x0034_3868 {
        PREV_COUNTER_EVENT.call(c);
        return;
    }
    let node = abi::arg(c, 0);
    let message = abi::arg64(c, 1);
    let observed = crate::custom_counter_trace::before(c, node);
    let level = c.f[12];
    PREV_COUNTER_EVENT.call(c);
    crate::custom_counter_trace::event(c, node, message, level, observed);
}

/// `Fence_Update` (0x34ce48): supply the source adjacency first.
fn fence_update(c: &mut Cpu) {
    if c.pc == 0x0034_ce48 {
        let fence = r32(c, A0);
        crate::custom_fences::prepare(c, fence);
    }
    PREV_FENCE_UPDATE.call(c);
}

/// `TexMan_InitTable` (0x3670e0): room for the world textures of an imported
/// course. At 1500 slots the generated/ported body is exact.
fn tex_man_init_table(c: &mut Cpu) {
    let slots = crate::custom_tracks::world_texture_slots();
    if c.pc != 0x0036_70e0 || slots == 1500 {
        PREV_TEX_MAN_INIT.call(c);
        return;
    }
    let free_links = 1999u32.saturating_sub(slots);
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x1f40;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, S0)).sw(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 8));
        Addr(p(r32(c, S0), 4)).sw(c, 0);
        call_ptr(c, 0x003e_6448, 0x0036_7108)?;
        c.g[V0][0] = sx(p(r32(c, S0), 8 + slots * 4));
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x1f48)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x4;
        loop {
            c.cycles += 9;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x1f48)).lw(c);
            c.g[V0][0] = sx(r32(c, A1) << 2);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            Addr(r32(c, V0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < free_links as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7118);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Replacements.

/// `RollerModifier_Update` (0x35e850): imported bodies keep their resting
/// pose after the source pause; everything else runs the native update.
fn roller_modifier_update(c: &mut Cpu) {
    if c.pc == 0x0035_e850 && crate::custom_roller_runtime::sleeping(c, r32(c, A0)) {
        abi::ret(c);
        return;
    }
    crate::custom_roller_update::roller_modifier_update(c);
}

/// `Emitter_Render` (0x3708c0): upstream's body with the two Tricky edits.
fn emitter_render(c: &mut Cpu) {
    c.cycles += 29;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S4, 0x60);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S5, 0x50);
    frame.save(c, S1, 0x90);
    let e = r32(c, A0);
    c.g[S5][0] = c.g[A0][0];
    frame.save_ra(c, 0x40);
    let context = lw(c, gp(c, 0x2a90)) as u32;
    c.g[S4][0] = sx(context);
    c.g[A0][0] = sx(frame.at(c, 0));
    c.g[S3][0] = sx(p(e, 0x160));
    frame.save(c, S0, 0xa0);
    c.g[S2][0] = sx(p(e, 0x170));
    guest(c, |c| {
        'body: {
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[S0][0] = sx(vt);
            let (d93, f93) = vslot(c, vt, 0x2e8);
            c.g[S1][0] = d93;
            c.g[A1][0] = lh(c, p(vt, 0x158));
            let f43 = lw(c, p(vt, 0x15c));
            c.g[V0][0] = f43;
            c.g[S1][0] = sx(context.wrapping_add(d93 as u32));
            let f43 = f43 as u32;
            c.g[A1][0] = sx(context.wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, f43, 0x0037_0910)?;
            c.g[V0][0] = f93;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            let f93 = f93 as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, f93, 0x0037_0928)?;
            c.g[V1][0] = sx(1);
            let t = c.g[V0][0] == c.g[V1][0];
            frame.restore(c, S0, 0xa0);
            if t {
                break 'body;
            }
            c.cycles += 3;
            let visible = lw(c, p(e, 0xc));
            c.g[V0][0] = visible;
            let t = visible == 0;
            c.g[V0][0] = sx(0x45_0000);
            if t {
                break 'body;
            }
            c.cycles += 82;
            let kind = lw(c, p(e, 0x8));
            c.g[V1][0] = kind;
            c.g[V0][0] = sx(0x45_0000u32.wrapping_sub(0x4be0));
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(top);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = sx(!0x3e0u32);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
            // Tricky edit: imported descriptors carry an explicit native GS preset
            // (source blend4 uses ALPHA 0x46, absent from the three-entry table).
            let layer = if kind as u32 & 0xffff_0000 == 0x4250_0000 {
                sx(kind as u32 & 31)
            } else { lw(c, p(r32(c, V1), 0)) };
            c.g[T1][0] = layer;
            copy_render_state(c, 2, 3, A3, top, top);
            c.g[T5][0] = 0x40_0000;
            c.g[A3][0] = 0xffff_ffff_fe7f_ffff;
            c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(top);
            c.g[T4][0] = sx(context);
            c.g[V0][0] = sx(p(top, 0x14));
            c.g[T3][0] = 0xffff_ffff_fff0_0fff;
            c.g[T0][0] = 0x1_4000;
            c.g[T1][0] = sx(r32(c, T1) << 2);
            c.g[V0][0] = lw(c, p(top, 0x1c));
            c.g[T6][0] = 0xffff_ffff_ffff_ff83;
            c.g[T1][0] = c.g[T1][0] & 0x7c;
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | 0xe0;
            c.g[A1][0] = sx(p(e, 0x10));
            let v0 = c.g[V0][0];
            Addr(p(top, 0x1c)).sw(c, v0);
            // top++.
            let new_top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(new_top);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x14));
            let pushed = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, pushed);
            // flagsA: (flagsA & ~0x400000) | 0x400000.
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            // texture id and draw.
            let _u = ldf(c, 0, p(e, 0x184));
            c.g[V0][0] = lw(c, p(e, 0x4));
            let sprite = c.g[V0][0] as u32;
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[T4][0] = sx(r32(c, T4).wrapping_add(r32(c, V0)));
            // Tricky edit: TX-tagged values name ordinary resident world textures.
            let tex = if sprite & 0xffff_0000 == 0x5458_0000 {
                sx((sprite & 0xffff).wrapping_add(r32(c, V1)))
            } else { lw(c, p(r32(c, T4), ctx::TEX_IDS)) };
            c.g[V1][0] = tex;
            c.sh(p(a, 0x10), tex);
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[V0][0] = sx(vt);
            c.g[A0][0] = lh(c, p(vt, 0x290));
            let f = lw(c, p(vt, 0x294));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[A0][0] = sx(context.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, f, 0x0037_0a78)?;
            let top = lw(c, p(context, ctx::TOP));
            c.g[V0][0] = top;
            c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
            let top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, top);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}
