//! `PS2GraphicsMan_RenderFrame` (0x363490) without packets. The generated
//! one sorts DeferMan's buckets and emits the frame's VIF chain (per bucket
//! the state change, material setup, texture binds and uploads, a CALL of
//! its packets and the RET after them) with the post passes at the layer
//! thresholds. The native frame build reads the sorted buckets and the
//! captures directly (`ssx3_render::frame::native`), so this keeps only
//! what it and the game use: the buckets (built and sorted), TexMan's
//! per-frame state, the post passes (their captures), DeferMan's emit state
//! and the published chain heads, each an `END` (the kick of a frame the
//! renderer drew completes without running it, `frame_sink::skip_chain`).
//! No texture is bound to VRAM (the renderer takes them from TexMan).

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;

use super::*;

const BUILD_BUCKETS: u32 = 0x0036_2de8;
const SORT_BUCKETS: u32 = 0x0036_4050;
const RESET_USAGE_TABLE: u32 = 0x0036_66f8;
const INVALIDATE_BOUND: u32 = 0x0039_6128;
const RENDER_DEPTH_FOG: u32 = 0x0036_ac00;
const RENDER_BLOOM: u32 = 0x0036_c790;
const RENDER_COLOR_FILTER: u32 = 0x0039_04a0;
/// The layer thresholds {6, 9}: after the buckets below 6 the depth fog,
/// at 9 the phase split, bloom and colour filter.
const LAYER_THRESHOLDS: u32 = 0x004a_4058;
const DEFAULT_STATE: u32 = 0x0050_1420;
/// DeferMan's emit state (the state the chain last set).
const EMIT_STATE: u32 = 0x69ca8;
const END: u64 = 0x7000_0000;

/// An `END` tag at the cursor `*at` (uncached, `alias` = 0x3000_0000 for
/// VifBuff, 0x2000_0000 for GifBuff); the cursor moves past it.
fn end_tag(c: &mut Cpu, at: u32, alias: u32) -> u32 {
    let p = r32(c, at);
    c.bus.write64(p | alias, END);
    c.bus.write64((p | alias) + 8, 0);
    w32(c, at, p + 0x10);
    p + 0x10
}

/// DeferMan's emit state back to the default, TexMan's bound textures
/// forgotten (after a post pass, as `RenderFrame` does).
fn reset_emit(c: &mut Cpu, dm: u32, tm: u32) {
    w32(c, dm + dmf::SCISSOR, 0xffff_ffff);
    for k in 0..5 {
        let v = r32(c, DEFAULT_STATE + 4 * k);
        w32(c, dm + EMIT_STATE + 4 * k, v);
    }
    w32(c, tm + 0x1f50, 0xffff_ffff);
    w32(c, tm + 0x1f4c, 0xffff_ffff);
    w32(c, tm + 0x1f54, 0xffff_ffff);
}

/// `RenderFrame(dm, &vifCursor, &phase2Vif, &gifCursor, &phase2Gif)`.
pub fn render_frame(c: &mut Cpu) {
    let (dm, vif, vif2, gif, gif2) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32, c.g[T0][0] as u32);
    let gp = c.g[GP][0] as u32;
    let gm = r32(c, gp + 0x2a90);
    guest_call(c, BUILD_BUCKETS, &[dm], 0x0036_34d4);
    w32(c, vif2, 0);
    w32(c, gif2, 0);
    for k in 0..5 {
        let v = r32(c, DEFAULT_STATE + 4 * k);
        w32(c, dm + EMIT_STATE + 4 * k, v);
    }
    guest_call(c, SORT_BUCKETS, &[dm], 0x0036_3548);
    guest_call(c, RESET_USAGE_TABLE, &[], 0x0036_3550);
    let tm = r32(c, gm + 0x18f4);
    guest_call(c, INVALIDATE_BOUND, &[tm], 0x0036_3580);
    let views = r32(c, gm + 0x59c0) as i32;
    if r32(c, dm + 0x67ca4) != 0 {
        w32(c, dm + dmf::SCISSOR, 0xffff_ffff);
        // (The buckets' layer ranges only order the passes; each threshold
        // runs once, after its range.)
        for k in 0..2 {
            match r32(c, LAYER_THRESHOLDS + 4 * k) {
                6 => {
                    for v in 0..views.max(0) as u32 {
                        guest_call(c, RENDER_DEPTH_FOG, &[gm, v, vif, gif], 0x0036_38e8);
                    }
                    reset_emit(c, dm, tm);
                }
                9 => {
                    end_tag(c, vif, 0x3000_0000);
                    let g = end_tag(c, gif, 0x2000_0000);
                    let v2 = r32(c, vif);
                    w32(c, vif2, v2);
                    w32(c, gif2, g);
                    for v in 0..views.max(0) as u32 {
                        guest_call(c, RENDER_BLOOM, &[gm, v, vif, gif], 0x0036_3a20);
                    }
                    for v in 0..views.max(0) as u32 {
                        guest_call(c, RENDER_COLOR_FILTER, &[gm, v, vif, gif], 0x0036_3a58);
                    }
                    reset_emit(c, dm, tm);
                }
                _ => {}
            }
        }
    }
    if r32(c, vif2) == 0 {
        let v = end_tag(c, vif, 0x3000_0000);
        w32(c, vif2, v);
    }
    if r32(c, gif2) == 0 {
        let g = end_tag(c, gif, 0x2000_0000);
        w32(c, gif2, g);
    }
    end_tag(c, gif, 0x2000_0000);
    end_tag(c, vif, 0x3000_0000);
    ret(c, 3000);
}

/// [`render_frame`] while packetless and `SSX3_NATIVE_RENDER_FRAME` is not
/// 0; else the generated body.
pub(crate) fn dispatch_render_frame(c: &mut Cpu, generated: fn(&mut Cpu)) {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let on = *ON.get_or_init(|| std::env::var("SSX3_NATIVE_RENDER_FRAME").map_or(true, |v| v != "0"));
    if on && direct::packetless() {
        render_frame(c)
    } else {
        generated(c)
    }
}
