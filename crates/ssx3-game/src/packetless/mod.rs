//! Stage C of plan step N2 (`docs/plans/native-rendering.md`): the game's
//! draw functions without packets. Each native form keeps what the rest of
//! the game reads (the render state at `gm+0xe84`, the flags it clears, the
//! DeferMan record the frame's buckets are built from, the VifBuff cursor
//! advance) and records its draw for the renderer (`ssx3_render::direct`),
//! but builds no VIF/GIF packet: its record points at an empty packet (the
//! 0x10 bytes of room every draw leaves for the link tag `RenderFrame`
//! writes), which the renderer resolves through the capture.
//!
//! Used while a native renderer takes the captures
//! (`direct::packetless`); otherwise, and with `SSX3_PACKETS=1`, the
//! generated bodies run with their captures (`crate::capture`).
//! `SSX3_DRAW_CHECK=1` runs both from the same state and compares what the
//! game can see and the recorded draw (the generated run's results stay).
//!
//! Cycle charges are estimates, not the generated code's: runs are
//! compared by behaviour, not RAM identity.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{self, Cmd, Recorded, Step, STRIP_BATCH};

use crate::ps2f::FpuF32;

use crate::call::{call_ptr, guest};

mod bezier;
mod frame;
mod glyphs;
mod model;
mod rider;
mod ribbon;
mod sprites;
mod target;
mod vu0;

pub use bezier::end_pass as bezier_end_pass;
pub(crate) use frame::dispatch_render_frame;
pub use model::{draw_model, grid_mesh_draw};
pub use glyphs::{draw_glyphs, draw_glyphs_w};
pub use ribbon::{draw_board_wake, draw_shadow_volume};
pub use sprites::{draw_rotated_sprites, draw_sprite_3d};
pub use target::{fill_render_target, restore_render_target, set_render_target_texture};
pub use rider::{draw_skinned_model, draw_skinned_model_shadow, emit_lighting_packet, set_shadow_header};

/// PS2GraphicsMan fields (`structs/render.md`).
mod gmf {
    /// `RenderState **curState` (top of the render-state stack).
    pub const CUR_STATE: u32 = 0xe84;
    /// `DeferMan *`.
    pub const DEFER: u32 = 0x18f0;
    /// VifBuff write cursor.
    pub const VIF_CURSOR: u32 = 0x5a00;
    /// Non-zero while the view matrices are valid.
    pub const VIEW_VALID: u32 = 0x6b90;
    /// `BeginTriStrip`: the packet start, the vertex write pointer.
    pub const STRIP_START: u32 = 0x5a28;
    pub const STRIP_AT: u32 = 0x5a2c;
}

/// DeferMan fields.
mod dmf {
    /// Records written this frame.
    pub const COUNT: u32 = 0;
    /// Records, 0x80 each.
    pub const RECORDS: u32 = 0x80;
    pub const MATRIX: u32 = 0x69cc0;
    pub const SCISSOR: u32 = 0x69cc4;
    /// Records per frame (more are dropped silently).
    pub const MAX: i32 = 2600;
}

const UPDATE_VIEW_MATRICES: u32 = 0x0037_d968;

/// Native draw functions on while a native renderer takes the captures
/// (`SSX3_PACKETS=1` keeps the packets).
pub fn set_on(native_renderer: bool) {
    direct::set_packetless(native_renderer);
}

fn r32(c: &mut Cpu, a: u32) -> u32 {
    c.bus.read32(a)
}

fn w32(c: &mut Cpu, a: u32, v: u32) {
    c.bus.write32(a, v)
}

/// The record a draw function ends with (`count < 2600`): the current
/// render state with the current scissor index (written back into it), the
/// packet's start and end, the current matrix record, the continuation
/// flag.
fn add_record(c: &mut Cpu, gm: u32, start: u32, end: u32, continuation: bool) {
    let dm = r32(c, gm + gmf::DEFER);
    let count = r32(c, dm + dmf::COUNT) as i32;
    if count >= dmf::MAX {
        return;
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    let w2 = r32(c, st + 8) & !0x1f | r32(c, dm + dmf::SCISSOR) & 0x1f;
    w32(c, st + 8, w2);
    let matrix = r32(c, dm + dmf::MATRIX);
    let rec = dm + dmf::RECORDS + count as u32 * 0x80;
    w32(c, dm + dmf::COUNT, count as u32 + 1);
    for k in 0..5 {
        let v = r32(c, st + 4 * k);
        w32(c, rec + 4 * k, v);
    }
    w32(c, rec + 0x14, start);
    w32(c, rec + 0x18, end);
    c.bus.write16(rec + 0x1c, matrix as u16);
    c.bus.write16(rec + 0x1e, continuation as u16);
}

/// An empty packet at the VifBuff cursor (the room for the link tag): its
/// address; the cursor moves past it.
fn empty_packet(c: &mut Cpu, gm: u32) -> u32 {
    let at = r32(c, gm + gmf::VIF_CURSOR);
    w32(c, gm + gmf::VIF_CURSOR, at + 0x10);
    at
}

fn record(start: u32, cmd: Cmd) {
    direct::record(Recorded { start, end: start, first_tag: 0, cmd });
}

/// `PS2GraphicsMan_UpdateViewMatrices(gm)` when the view matrices are not
/// valid (it makes no event checks, so it cannot unwind); `ret`: the
/// generated caller's return address.
fn ensure_view_matrices(c: &mut Cpu, gm: u32, ret: u32) {
    if r32(c, gm + gmf::VIEW_VALID) != 0 {
        return;
    }
    let (ra, a0) = (c.g[RA], c.g[A0]);
    c.g[A0][0] = gm as u64;
    guest(c, |c| call_ptr(c, UPDATE_VIEW_MATRICES, ret));
    c.g[RA] = ra;
    c.g[A0] = a0;
}

fn ret(c: &mut Cpu, cycles: u64) {
    c.cycles += cycles;
    c.pc = c.g[RA][0] as u32;
}

/// The current state's material type (w0 bits 6-9) and its w3 (bits 0-29)
/// cleared, after a draw.
fn clear_type(c: &mut Cpu, gm: u32) {
    let st = r32(c, gm + gmf::CUR_STATE);
    let w0 = r32(c, st) & !0x3c0;
    w32(c, st, w0);
    let w3 = r32(c, st + 0xc) & 0xc000_0000;
    w32(c, st + 0xc, w3);
}

// ---------------------------------------------------------------------------
// Slot 71.

/// `PS2GraphicsMan_DrawTriStrip(gm, n, verts)` (slot 71, 0x379860): `n`
/// vertices of 3 qwords {STQ, RGBA, XYZW} drawn by the type 3 strip
/// program (entry 0x12b8; 0x12b0 once after `gp+0x1290` was set).
pub fn draw_tri_strip(c: &mut Cpu) {
    let (gm, n, verts) = (c.g[A0][0] as u32, c.g[A1][0] as i64, c.g[A2][0] as u32);
    let gp = c.g[GP][0] as u32;
    let entry = if r32(c, gp + 0x1290) == 0 { 0x12b8 } else { 0x12b0 };
    w32(c, gp + 0x1290, 0);
    let st = r32(c, gm + gmf::CUR_STATE);
    let w0 = r32(c, st) & !0x3c0 | 0xc0;
    w32(c, st, w0);
    let w3 = r32(c, st + 0xc) & 0xc000_0000;
    w32(c, st + 0xc, w3);
    ensure_view_matrices(c, gm, 0x0037_98fc);
    let st = r32(c, gm + gmf::CUR_STATE);
    let prim = if (c.bus.read16(st + 0x10) as i16) < 0 { 0x4c } else { 0x5c };
    let n = if n > 2 { n as u32 } else { 0 };
    let v: Vec<[u128; 3]> = (0..n).map(|i| std::array::from_fn(|k| c.bus.read128(verts + 48 * i + 16 * k as u32))).collect();
    let start = empty_packet(c, gm);
    record(start, if v.is_empty() { Cmd::Vu(Vec::new()) } else { Cmd::Strip { entry, prim, verts: v } });
    add_record(c, gm, start, start, false);
    clear_type(c, gm);
    ret(c, 60 + 20 * n as u64);
}

/// A strip begun with `BeginTriStrip`: entry, PRIM, vertex count, where
/// the caller writes the vertices.
static OPEN_STRIP: std::sync::Mutex<Option<(u32, u32, u32, u32)>> = std::sync::Mutex::new(None);

/// `PS2GraphicsMan_BeginTriStrip(gm, n)` (0x37a260): the caller writes `n`
/// vertices (as slot 71's) into a scratchpad block and calls
/// `EndTriStrip`; `gm+0x5a2c` points past them. The block is the one the
/// SprStream at `gp+0x1384` would hand out next (its state stays as it is).
pub fn begin_tri_strip(c: &mut Cpu) {
    let (gm, n) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    let gp = c.g[GP][0] as u32;
    let entry = if r32(c, gp + 0x1290) == 0 { 0x12b8 } else { 0x12b0 };
    w32(c, gp + 0x1290, 0);
    let st = r32(c, gm + gmf::CUR_STATE);
    let w0 = r32(c, st) & !0x3c0 | 0xc0;
    w32(c, st, w0);
    let w3 = r32(c, st + 0xc) & 0xc000_0000;
    w32(c, st + 0xc, w3);
    ensure_view_matrices(c, gm, 0x0037_a2e8);
    let st = r32(c, gm + gmf::CUR_STATE);
    let prim = if (c.bus.read16(st + 0x10) as i16) < 0 { 0x4c } else { 0x5c };
    let cursor = r32(c, gm + gmf::VIF_CURSOR);
    w32(c, gm + gmf::STRIP_START, cursor);
    let spr = r32(c, gp + 0x1384);
    let next = r32(c, spr + 8);
    let block = r32(c, spr + 0x14 + 4 * next);
    w32(c, gm + gmf::STRIP_AT, block.wrapping_add(0x50 + 0x30 * n));
    *OPEN_STRIP.lock().unwrap() = Some((entry, prim, n, block + 0x40));
    ret(c, 70);
}

/// `PS2GraphicsMan_EndTriStrip(gm)` (0x37a430).
pub fn end_tri_strip(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    let open = OPEN_STRIP.lock().unwrap().take();
    let start = empty_packet(c, gm);
    let cmd = match open {
        // (One batch: the strip program takes at most 72 vertices.)
        Some((entry, prim, n, at)) if (3..=STRIP_BATCH as u32).contains(&n) => {
            let verts = (0..n).map(|i| std::array::from_fn(|k| c.bus.read128(at + 48 * i + 16 * k as u32))).collect();
            Cmd::Strip { entry, prim, verts }
        }
        _ => Cmd::Vu(Vec::new()),
    };
    record(start, cmd);
    add_record(c, gm, start, start, false);
    clear_type(c, gm);
    w32(c, gm + gmf::STRIP_AT, 0);
    w32(c, gm + gmf::STRIP_START, 0);
    ret(c, 40);
}

// ---------------------------------------------------------------------------
// Particles (type 4/5): the VU1 programs generate the particles from a
// descriptor (and, for ring emitters, the particles' positions).

fn q(c: &mut Cpu, a: u32) -> u128 {
    c.bus.read128(a)
}

fn qv(c: &mut Cpu, a: u32) -> vu0::V {
    v4(q(c, a))
}

fn wv(c: &mut Cpu, a: u32, v: &vu0::V) {
    c.bus.write128(a, q4(*v))
}

fn qm(c: &mut Cpu, a: u32) -> vu0::M {
    std::array::from_fn(|i| qv(c, a + 16 * i as u32))
}

fn wm(c: &mut Cpu, a: u32, m: &vu0::M) {
    for (i, r) in m.iter().enumerate() {
        wv(c, a + 16 * i as u32, r);
    }
}

/// Call a guest function from a native draw function: `args` in a0..t3,
/// `ret` the return address the generated caller uses; its v0. A native
/// caller cannot be resumed after an unwind, so the callee runs with
/// `Cpu::no_unwind`.
fn guest_call(c: &mut Cpu, target: u32, args: &[u32], ret: u32) -> u32 {
    const R: [usize; 8] = [A0, A1, A2, A3, T0, T1, T2, T3];
    let ra = c.g[RA];
    for (i, &a) in args.iter().enumerate() {
        c.g[R[i]][0] = a as i32 as i64 as u64;
    }
    // (Host threads block in a thread switch and resume here; the unwind
    // form keeps the switch for the generated caller's next check.)
    let no_unwind = std::mem::replace(&mut c.no_unwind, true);
    guest(c, |c| call_ptr(c, target, ret));
    c.no_unwind = no_unwind;
    c.g[RA] = ra;
    c.g[V0][0] as u32
}

fn q4(v: [u32; 4]) -> u128 {
    v.iter().enumerate().fold(0u128, |q, (l, &x)| q | (x as u128) << (32 * l))
}

fn v4(q: u128) -> [u32; 4] {
    std::array::from_fn(|l| (q >> (32 * l)) as u32)
}

/// `n` qwords from `a`.
fn qwords(c: &mut Cpu, a: u32, n: u32) -> Vec<u128> {
    (0..n).map(|i| q(c, a + 16 * i)).collect()
}

/// Push the render state (`*curState` copied to the next slot, which
/// becomes current).
fn push_state(c: &mut Cpu, gm: u32) -> u32 {
    let st = r32(c, gm + gmf::CUR_STATE);
    for k in 0..5 {
        let v = r32(c, st + 4 * k);
        w32(c, st + 0x14 + 4 * k, v);
    }
    w32(c, gm + gmf::CUR_STATE, st + 0x14);
    st + 0x14
}

fn pop_state(c: &mut Cpu, gm: u32) {
    let st = r32(c, gm + gmf::CUR_STATE);
    w32(c, gm + gmf::CUR_STATE, st - 0x14);
}

/// Set state word `k` to `w & keep | set`.
fn state_word(c: &mut Cpu, st: u32, k: u32, keep: u32, set: u32) {
    let w = r32(c, st + 4 * k) & keep | set;
    w32(c, st + 4 * k, w);
}

/// One packet of VU1 input: the writes (TOPS-relative), then the program
/// start; its record.
fn vu_draw(c: &mut Cpu, gm: u32, writes: Vec<(u16, Vec<u128>)>, entry: u32) {
    let start = empty_packet(c, gm);
    let mut steps: Vec<Step> = writes.into_iter().map(|(addr, data)| Step::Write { addr, rel: true, data }).collect();
    steps.push(Step::Start(entry));
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, false);
}

/// Slot 82 `DrawEmitter(gm, desc)` (0x380518): one emitter, its 21-qword
/// descriptor to VU1 0, program 0; in a pushed state (type 4, sort mode
/// 2, layer 7). Off while `gp+0x1408` is set.
pub fn draw_emitter(c: &mut Cpu) {
    let (gm, desc) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    let gp = c.g[GP][0] as u32;
    if r32(c, gp + 0x1408) != 0 {
        return ret(c, 9);
    }
    let st = push_state(c, gm);
    state_word(c, st, 0, !0x3c0, 0x100);
    state_word(c, st, 3, 0xc000_0000, 0);
    state_word(c, st, 1, !3, 2);
    state_word(c, st, 2, 0xe000_03ff, 0);
    state_word(c, st, 2, !0x3e0, 0xe0);
    let d = qwords(c, desc, 21);
    ensure_view_matrices(c, gm, 0x0038_06d0);
    vu_draw(c, gm, vec![(0, d)], 0);
    pop_state(c, gm);
    ret(c, 120);
}

/// A ring emitter's batch input (slots 83, 84): the descriptor, then a
/// qword {count, total / ring size, as float, its time offset}, then per
/// ring (positions A, B) the batch's `count` entries from the ring (it
/// wraps) and the entry after them, `per + 1` qwords apart; for colours
/// (slot 84) the same of 32-bit colours (V4-8).
struct RingBatch {
    count: i32,
    first: i32,
    wrap: i32,
    i1: i32,
    end: i32,
}

/// The batches of a ring emitter of `per` particles per batch: batch
/// count, the per-batch ring indices (the generated code's arithmetic).
fn ring_batches(c: &mut Cpu, e: u32, per: i32, batches: i32) -> Vec<RingBatch> {
    let size = r32(c, e + 0x178) as i32;
    let mut done = 0i32;
    let mut out = Vec::new();
    for _ in 0..batches {
        let count = (size - done).min(per);
        let base = (r32(c, e + 0x17c) as i32).wrapping_add(1);
        let i1 = (done + base) % size;
        done += count;
        let wrap = (i1 + count - size).max(0);
        let first = count.min(size - i1);
        let end = if done < size { (done + base) % size } else { (done - 1 + base) % size };
        out.push(RingBatch { count, first, wrap, i1, end });
    }
    out
}

/// A ring's entries for a batch (`stride` bytes each): from `i1`, the
/// wrapped part from the start, the entry at `end`.
fn ring_part(c: &mut Cpu, ring: u32, b: &RingBatch) -> Vec<u128> {
    let mut v = qwords(c, ring + 16 * b.i1 as u32, b.first.max(0) as u32);
    v.extend(qwords(c, ring, b.wrap.max(0) as u32));
    v.push(q(c, ring + 16 * b.end as u32));
    v
}

/// The emitter's header qword for batch `k` of `per`.
fn ring_header(c: &mut Cpu, e: u32, count: i32, per_batch: i32, k: i32) -> u128 {
    let size = r32(c, e + 0x178) as i32;
    let bc = (r32(c, e + 0x20) as i32).wrapping_div(size);
    let f0 = FpuF32::from_i32(bc);
    let f1 = f0 * FpuF32::from_i32(k * per_batch) * FpuF32(r32(c, e + 0x2c));
    q4([count as u32, bc as u32, f0.0, f1.0])
}

/// Slot 83 `DrawDynamicEmitter(gm, e)` (0x3807a0): batches of 64 ring
/// particles, program 0x510. Off while `gp+0x140c` is set.
pub fn draw_dynamic_emitter(c: &mut Cpu) {
    let (gm, e) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    let gp = c.g[GP][0] as u32;
    if r32(c, gp + 0x140c) != 0 {
        return ret(c, 9);
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0x100);
    state_word(c, st, 3, 0xc000_0000, 0);
    ensure_view_matrices(c, gm, 0x0038_081c);
    let size = r32(c, e + 0x178) as i32;
    if size == 0 {
        return ret(c, 20);
    }
    let batches = (size >> 6) + 1;
    let desc = qwords(c, e + 0x20, 21);
    let (ra, rb) = (r32(c, e + 0x1a0), r32(c, e + 0x1a4));
    let mut n = 0;
    for (k, b) in ring_batches(c, e, 64, batches).into_iter().enumerate() {
        let mut a = desc.clone();
        a.push(ring_header(c, e, b.count, 64, k as i32));
        a.extend(ring_part(c, ra, &b));
        let bq = ring_part(c, rb, &b);
        vu_draw(c, gm, vec![(0, a), (22 + 65, bq)], 0x510);
        n += 1;
    }
    clear_type(c, gm);
    ret(c, 60 + 400 * n);
}

/// Slot 84 `DrawMEmitter(gm, e)` (0x380ce0): batches of 43 ring particles
/// with colours, program 0xa00 (0xf10 with `e+0x200`). Off while
/// `gp+0x140c` is set.
pub fn draw_m_emitter(c: &mut Cpu) {
    let (gm, e) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    let gp = c.g[GP][0] as u32;
    if r32(c, gp + 0x140c) != 0 {
        return ret(c, 9);
    }
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0x100);
    state_word(c, st, 3, 0xc000_0000, 0);
    ensure_view_matrices(c, gm, 0x0038_0d5c);
    let size = r32(c, e + 0x178) as i32;
    if size == 0 {
        return ret(c, 20);
    }
    let batches = size / 43 + 1;
    let entry = if r32(c, e + 0x200) == 0 { 0xa00 } else { 0xf10 };
    let desc = qwords(c, e + 0x20, 21);
    let (ra, rb, rc) = (r32(c, e + 0x1a0), r32(c, e + 0x1a4), r32(c, e + 0x204));
    let mut n = 0;
    for (k, b) in ring_batches(c, e, 43, batches).into_iter().enumerate() {
        let mut a = desc.clone();
        a.push(ring_header(c, e, b.count, 43, k as i32));
        a.extend(ring_part(c, ra, &b));
        let bq = ring_part(c, rb, &b);
        // Colours, one word each (V4-8: a byte per lane).
        let mut cols: Vec<u32> = (0..b.first.max(0) as u32).map(|i| r32(c, rc + 4 * (b.i1 as u32 + i))).collect();
        cols.extend((0..b.wrap.max(0) as u32).map(|i| r32(c, rc + 4 * i)));
        cols.push(r32(c, rc + 4 * b.end as u32));
        let cq = cols.iter().map(|&w| q4(std::array::from_fn(|l| (w >> (8 * l)) & 0xff))).collect();
        vu_draw(c, gm, vec![(0, a), (22 + 44, bq), (0x6e, cq)], entry);
        n += 1;
    }
    clear_type(c, gm);
    ret(c, 60 + 400 * n);
}

/// Slots 85 / 86 `DrawTrail(B)(gm, d)` (0x381310 / 0x3816f0): SnowBox
/// flakes (type 5): the view matrix times the box's (VU0, as the
/// generated code), {0}, {1, 1, 0, 0}, the descriptor's first 5 qwords;
/// program 0 (0x408 for B).
fn draw_trail_with(c: &mut Cpu, entry: u32, view_ret: u32) {
    let (gm, d) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0x140);
    state_word(c, st, 3, 0xc000_0000, 0);
    state_word(c, st, 1, !3, 2);
    state_word(c, st, 2, 0xe000_03ff, 0);
    ensure_view_matrices(c, gm, view_ret);
    use ssx3_vu::vu::fast;
    let rows: [[u32; 4]; 4] = std::array::from_fn(|i| v4(q(c, gm + 0x5860 + 16 * i as u32)));
    let mut data: Vec<u128> = (0..4u32)
        .map(|i| {
            let v = v4(q(c, d + 0x50 + 16 * i));
            let acc = fast::mul4::<0b1111, true, true>(&rows[0], &[v[0]; 4]).0;
            let acc = fast::madd4::<0b1111, true, true, true>(&acc, &rows[1], &[v[1]; 4]).0;
            let acc = fast::madd4::<0b1111, true, true, true>(&acc, &rows[2], &[v[2]; 4]).0;
            q4(fast::madd4::<0b1111, true, true, true>(&acc, &rows[3], &[v[3]; 4]).0)
        })
        .collect();
    data.push(0);
    data.push(q4([0x3f80_0000, 0x3f80_0000, 0, 0]));
    data.extend(qwords(c, d, 5));
    vu_draw(c, gm, vec![(0, data)], entry);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 1, !3, 0);
    state_word(c, st, 2, 0xe000_03ff, 0);
    clear_type(c, gm);
    ret(c, 150);
}

pub fn draw_trail(c: &mut Cpu) {
    draw_trail_with(c, 0, 0x0038_13a0)
}

pub fn draw_trail_b(c: &mut Cpu) {
    draw_trail_with(c, 0x408, 0x0038_1780)
}

/// `PS2BezierMan_DrawPatchFX(bm, patch, lod, &cursor)` (0x38d690): a
/// patch's glitter (patch flag 0x800000; type 4, program 0x1460): the
/// particle count from the patch's size, the patch's 16 qwords from +0x40,
/// BezierMan's 4 matrix qwords at +0x3e0 and 2 at +0x420 (the count
/// written into +0x43c first). The cursor is the caller's.
pub fn draw_patch_fx(c: &mut Cpu) {
    let (bm, patch, lod, cursor) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32);
    let gp = c.g[GP][0] as u32;
    let gm = r32(c, gp + 0x2a90);
    if r32(c, patch + 0xc) & 0x80_0000 == 0 {
        return ret(c, 8);
    }
    let f = |c: &mut Cpu, a: u32| FpuF32(r32(c, a));
    let dx = f(c, patch + 0x158) - f(c, patch + 0x164);
    let dy = f(c, patch + 0x15c) - f(c, patch + 0x168);
    let dz = f(c, patch + 0x160) - f(c, patch + 0x16c);
    let k = f(c, gp + 0x1258) * f(c, gp.wrapping_sub(0x26b4));
    let sum = dx * dx + dy * dy + dz * dz;
    let mut n = (sum * k * f(c, gm + 0xc4)).to_i32();
    if (r32(c, lod + 8) as i32) < 2 {
        n /= 2;
    }
    if n <= 0 {
        return ret(c, 30);
    }
    w32(c, bm + 0x43c, n as u32);
    let mut data = qwords(c, patch + 0x40, 16);
    data.extend(qwords(c, bm + 0x3e0, 4));
    data.extend(qwords(c, bm + 0x420, 2));
    let start = r32(c, cursor);
    w32(c, cursor, start + 0x10);
    record(start, Cmd::Vu(vec![Step::Write { addr: 0, rel: true, data }, Step::Start(0x1460)]));
    add_record(c, gm, start, start, false);
    ret(c, 120);
}

/// [`draw_patch_fx`]'s override (its graphics manager and cursor are not
/// its arguments).
pub(crate) fn dispatch_patch_fx(c: &mut Cpu, packets: fn(&mut Cpu)) {
    if !direct::packetless() || skipped("DrawPatchFX") {
        packets(c);
    } else if check_on() {
        let gm = r32(c, c.g[GP][0] as u32 + 0x2a90);
        let (bm, cursor) = (c.g[A0][0] as u32, c.g[A3][0] as u32);
        let mut w = record_watch(c, gm, 1);
        w.ranges.extend([(cursor, cursor + 4), (bm + 0x43c, bm + 0x440)]);
        w.ignore.push(cursor);
        checked(c, "DrawPatchFX", w, draw_patch_fx, packets);
    } else {
        draw_patch_fx(c);
    }
}

// ---------------------------------------------------------------------------
// Checks (`SSX3_DRAW_CHECK=1`).

fn keep_native() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("SSX3_DRAW_CHECK").is_ok_and(|v| v == "2"))
}

pub(crate) fn check_on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("SSX3_DRAW_CHECK").is_ok_and(|v| !v.is_empty() && v != "0"))
}

/// Per checked function: calls, mismatches.
/// Per checked function: calls, mismatches, packet qwords the native form
/// does not write (stale words past a batch's count).
static STATS: std::sync::Mutex<Vec<(&'static str, u64, u64, u64)>> = std::sync::Mutex::new(Vec::new());

/// What a check compares: guest ranges `[a, b)` (words) and, inside them,
/// words that may differ (packet addresses).
pub(crate) struct Watch {
    pub ranges: Vec<(u32, u32)>,
    pub ignore: Vec<u32>,
}

/// The ranges a draw that writes one record touches: the current state,
/// DeferMan's count, the next record (its packet start and end ignored).
pub(crate) fn record_watch(c: &mut Cpu, gm: u32, records: u32) -> Watch {
    let st = r32(c, gm + gmf::CUR_STATE);
    let dm = r32(c, gm + gmf::DEFER);
    let count = r32(c, dm + dmf::COUNT);
    let rec = dm + dmf::RECORDS + count * 0x80;
    let cursor = gm + gmf::VIF_CURSOR;
    let cur = gm + gmf::CUR_STATE;
    let mut w = Watch { ranges: vec![(st, st + 0x28), (dm, dm + 4), (cursor, cursor + 4), (cur, cur + 4)], ignore: vec![cursor] };
    for k in 0..records {
        let r = rec + 0x80 * k;
        w.ranges.push((r, r + 0x20));
        w.ignore.extend([r + 0x14, r + 0x18]);
    }
    w
}

/// `SSX3_PACKETLESS_SKIP=Name,...`: these keep their generated bodies
/// (bisecting).
fn skipped(name: &str) -> bool {
    static SKIP: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    SKIP.get_or_init(|| std::env::var("SSX3_PACKETLESS_SKIP").map(|v| v.split(',').map(str::to_string).collect()).unwrap_or_default()).iter().any(|s| s == name)
}

/// A draw function's override: its native form while packetless (checked
/// against the generated one with `SSX3_DRAW_CHECK=1`), else `packets`
/// (the generated body with its capture).
pub(crate) fn dispatch(c: &mut Cpu, name: &'static str, records: u32, native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    dispatch_with(c, name, records, &[], native, packets)
}

/// [`dispatch`] with more `gm` fields watched (offsets).
pub(crate) fn dispatch_with(c: &mut Cpu, name: &'static str, records: u32, gm_fields: &[u32], native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    if !direct::packetless() || skipped(name) {
        packets(c);
    } else if check_on() {
        let gm = c.g[A0][0] as u32;
        let mut w = record_watch(c, gm, records);
        w.ranges.extend(gm_fields.iter().map(|&o| (gm + o, gm + o + 4)));
        checked(c, name, w, native, packets);
    } else {
        native(c);
    }
}

/// A draw function's override checked against all of guest memory
/// (functions that write beyond their records: buckets, hash chains,
/// link tags): every `SSX3_DRAW_CHECK_EVERY`-th call (default 8) runs
/// both from a copy of memory and compares the pages either wrote.
pub(crate) fn dispatch_ram(c: &mut Cpu, name: &'static str, native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    if !direct::packetless() || skipped(name) {
        packets(c);
    } else if check_on() {
        static EVERY: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
        let every = *EVERY.get_or_init(|| std::env::var("SSX3_DRAW_CHECK_EVERY").ok().and_then(|v| v.parse().ok()).unwrap_or(8).max(1));
        static CALLS: std::sync::Mutex<Vec<(&'static str, u64)>> = std::sync::Mutex::new(Vec::new());
        let n = {
            let mut calls = CALLS.lock().unwrap();
            let i = calls.iter().position(|x| x.0 == name).unwrap_or_else(|| {
                calls.push((name, 0));
                calls.len() - 1
            });
            calls[i].1 += 1;
            calls[i].1
        };
        if n % every == 0 {
            checked_ram(c, name, native, packets);
        } else {
            packets(c);
        }
    } else {
        native(c);
    }
}

/// Is `a` a word that holds a packet address (or lies in render-private
/// memory), so native and generated runs differ there by design?
fn packet_word(c: &mut Cpu, gm: u32, a: u32) -> bool {
    // (The SprStream at gp+0x1384: its block ring, which native forms
    // leave alone.)
    let spr = r32(c, c.g[GP][0] as u32 + 0x1384);
    if (spr..spr + 0x40).contains(&a) {
        return true;
    }
    let vif = r32(c, gm + 0x5a0c);
    if (vif..vif + 2 * 0xccc80).contains(&a) || a == gm + gmf::VIF_CURSOR {
        return true;
    }
    let dm = r32(c, gm + gmf::DEFER);
    let recs = dm + dmf::RECORDS;
    if (recs..recs + 0x80 * dmf::MAX as u32).contains(&a) && matches!((a - recs) % 0x80, 0x14 | 0x18) {
        return true;
    }
    let buckets = dm + 0x57498;
    (buckets..buckets + 0x20 * 0x400).contains(&a) && matches!((a - buckets) % 0x20, 0xc | 0x10)
}

fn checked_ram(c: &mut Cpu, name: &'static str, native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    let gm = r32(c, c.g[GP][0] as u32 + 0x2a90);
    let sp = c.g[SP][0] as u32;
    let pages = c.bus.ram.len() / 4096;
    let mut pending = vec![0u64; pages.div_ceil(64)];
    ssx3_ee_rt::bus::take_written(&mut pending);
    let pre = c.bus.ram.clone();
    let pre_spr = c.bus.spr.clone();
    let (g, f, pc, cycles) = (c.g, c.f, c.pc, c.cycles);
    // (Interrupts wait until both ran: their handlers would write memory
    // in one run only.)
    let next = c.next_event;
    c.next_event = u64::MAX;
    let m = direct::mark();
    native(c);
    let mut wn = vec![0u64; pending.len()];
    ssx3_ee_rt::bus::take_written(&mut wn);
    let bits = |b: &[u64]| -> Vec<usize> { (0..pages).filter(|&p| b[p >> 6] >> (p & 63) & 1 != 0).collect() };
    let native_pages: Vec<(usize, Vec<u8>)> = bits(&wn).into_iter().map(|p| (p, c.bus.ram[p * 4096..(p + 1) * 4096].to_vec())).collect();
    let native_s = c.bus.spr[0x3800..0x4000].to_vec();
    let rec_native = direct::split_off(m);
    for (p, _) in &native_pages {
        c.bus.ram[p * 4096..(p + 1) * 4096].copy_from_slice(&pre[p * 4096..(p + 1) * 4096]);
    }
    c.bus.spr.copy_from_slice(&pre_spr);
    (c.g, c.f, c.pc, c.cycles) = (g, f, pc, cycles);
    c.next_event = u64::MAX;
    packets(c);
    c.next_event = if c.next_event == u64::MAX { next } else { c.next_event.min(next) };
    let mut wp = vec![0u64; pending.len()];
    ssx3_ee_rt::bus::take_written(&mut wp);
    let mut bad = Vec::new();
    let mut all: Vec<usize> = bits(&wn);
    all.extend(bits(&wp));
    all.sort_unstable();
    all.dedup();
    for p in all {
        let nat = native_pages.iter().find(|x| x.0 == p).map_or(&pre[p * 4096..(p + 1) * 4096], |x| &x.1[..]);
        for o in (0..4096).step_by(4) {
            let a = (p * 4096 + o) as u32;
            let x = u32::from_le_bytes(nat[o..o + 4].try_into().unwrap());
            let y = u32::from_le_bytes(c.bus.ram[p * 4096 + o..p * 4096 + o + 4].try_into().unwrap());
            // (Stack scratch below `sp` differs: the frames differ.)
            if x != y && !(sp.wrapping_sub(0x1000)..sp).contains(&a) && !packet_word(c, gm, a) {
                bad.push(format!("{a:#x}: native {x:#x}, packets {y:#x}"));
            }
        }
    }
    // (The GsPkt helper's builder fields, render-private: native forms do
    // not build with it.)
    let helper = r32(c, 0x004a_5b84);
    for o in (0..0x800).step_by(4) {
        if (helper..helper + 0x30).contains(&(0x7000_3800 + o as u32)) {
            continue;
        }
        let x = u32::from_le_bytes(native_s[o..o + 4].try_into().unwrap());
        let y = u32::from_le_bytes(c.bus.spr[0x3800 + o..0x3800 + o + 4].try_into().unwrap());
        if x != y {
            bad.push(format!("spr {:#x}: native {x:#x}, packets {y:#x}", 0x7000_3800 + o));
        }
    }
    for b in [&pending, &wn, &wp] {
        for p in bits(b) {
            ssx3_ee_rt::bus::mark_written(p * 4096);
        }
    }
    let rec_packets = direct::split_off(m);
    // (One draw per packet: a fast-path packet is also captured by
    // AppendPacket's wrapper, a record's ribbon calls by their emitters';
    // the widest capture at a start stands, nested ones go.)
    let filt = |r: &[Recorded]| -> Vec<Cmd> {
        let mut r: Vec<&Recorded> = r.iter().collect();
        r.sort_by_key(|x| (x.start, std::cmp::Reverse(x.end)));
        let mut out: Vec<&Recorded> = Vec::new();
        for x in r {
            if out.last().is_some_and(|l| x.start == l.start || x.start < l.end) {
                continue;
            }
            out.push(x);
        }
        out.into_iter().map(|x| x.cmd.clone()).filter(|c| !matches!(c, Cmd::Vu(v) if v.is_empty())).collect()
    };
    let (a, b) = (filt(&rec_native), filt(&rec_packets));
    let uncovered: u64 = a.iter().zip(&b).map(|(x, y)| uncovered(x, y)).sum();
    if a.len() != b.len() || a.iter().zip(&b).any(|(x, y)| !same_draw(x, y)) {
        bad.push(format!("draws: native {:?} / packets {:?}", short(&rec_native), short(&rec_packets)));
        if let Some((x, y)) = a.iter().zip(&b).find(|(x, y)| !same_draw(x, y)) {
            bad.push(format!("first: native {:.1500} / packets {:.1500}", format!("{x:x?}"), format!("{y:x?}")));
        }
    }
    let mut nat = rec_native.iter().filter(|x| !matches!(&x.cmd, Cmd::Vu(v) if v.is_empty()));
    for x in &rec_packets {
        let mut x = x.clone();
        if keep_native() {
            if let Some(n) = nat.next() {
                x.cmd = n.cmd.clone();
            }
        }
        direct::record(x);
    }
    report(name, bad, uncovered);
}

fn report(name: &'static str, bad: Vec<String>, uncovered: u64) {
    let mut stats = STATS.lock().unwrap();
    let i = match stats.iter().position(|s| s.0 == name) {
        Some(i) => i,
        None => {
            stats.push((name, 0, 0, 0));
            stats.len() - 1
        }
    };
    let s = &mut stats[i];
    s.1 += 1;
    s.3 += uncovered;
    if !bad.is_empty() {
        s.2 += 1;
        if s.2 <= 10 {
            let n = bad.len();
            let mut bad = bad;
            bad.truncate(12);
            eprintln!("[draw-check] {name} call {}: {n} differences: {}", s.1, bad.join("; "));
        }
    }
}

/// Run `native` and `packets` from the same registers and watched memory;
/// report differences in the watched words and the recorded draws. The
/// generated run's state and captures stay.
pub(crate) fn checked(c: &mut Cpu, name: &'static str, mut watch: Watch, native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    let gp = c.g[GP][0] as u32;
    watch.ranges.push((gp + 0x1290, gp + 0x1294));
    let words = |c: &mut Cpu, w: &Watch| -> Vec<(u32, u32)> {
        w.ranges.iter().flat_map(|&(a, b)| (a..b).step_by(4)).map(|a| (a, c.bus.read32(a))).collect()
    };
    let (g, f, pc, cycles) = (c.g, c.f, c.pc, c.cycles);
    let before = words(c, &watch);
    let m = direct::mark();
    native(c);
    let after_native = words(c, &watch);
    let rec_native = direct::split_off(m);
    for &(a, v) in &before {
        c.bus.write32(a, v);
    }
    (c.g, c.f, c.pc, c.cycles) = (g, f, pc, cycles);
    packets(c);
    let after = words(c, &watch);
    // (`SSX3_DRAW_CHECK=2`: the renderer gets the native commands, at the
    // packets' places: images show what the native forms draw while the
    // game runs exactly as generated.)
    let rec_packets: Vec<Recorded> = {
        let r = direct::split_off(m);
        let mut nat = rec_native.iter().filter(|x| !matches!(&x.cmd, Cmd::Vu(v) if v.is_empty()));
        for x in &r {
            let mut x = x.clone();
            if keep_native() {
                if let Some(n) = nat.next() {
                    x.cmd = n.cmd.clone();
                }
            }
            direct::record(x);
        }
        r
    };
    let mut bad = Vec::new();
    for (&(a, x), &(_, y)) in after_native.iter().zip(&after) {
        if x != y && !watch.ignore.contains(&a) {
            bad.push(format!("{a:#x}: native {x:#x}, packets {y:#x}"));
        }
    }
    // (A native draw of nothing records an empty command; the capture of
    // an empty packet records none.)
    let cmds = |r: &[Recorded]| -> Vec<Cmd> { r.iter().map(|x| x.cmd.clone()).filter(|c| !matches!(c, Cmd::Vu(v) if v.is_empty())).collect() };
    let (a, b) = (cmds(&rec_native), cmds(&rec_packets));
    let uncovered: u64 = a.iter().zip(&b).map(|(x, y)| uncovered(x, y)).sum();
    if a.len() != b.len() || a.iter().zip(&b).any(|(x, y)| !same_draw(x, y)) {
        bad.push(format!("draws: native {:?} / packets {:?}", short(&rec_native), short(&rec_packets)));
    }
    let mut stats = STATS.lock().unwrap();
    let i = match stats.iter().position(|s| s.0 == name) {
        Some(i) => i,
        None => {
            stats.push((name, 0, 0, 0));
            stats.len() - 1
        }
    };
    let s = &mut stats[i];
    s.1 += 1;
    s.3 += uncovered;
    if !bad.is_empty() {
        s.2 += 1;
        if s.2 <= 10 {
            eprintln!("[draw-check] {name} call {}: {}", s.1, bad.join("; "));
        }
    }
    if s.1.is_power_of_two() && s.1 >= 1024 {
        eprintln!("[draw-check] {name}: {} calls, {} differ", s.1, s.2);
    }
}

/// Check results per function (`SSX3_DRAW_CHECK=1`; empty otherwise).
pub fn check_report() -> Vec<String> {
    STATS.lock().unwrap().iter().map(|(n, k, bad, u)| format!("{n}: {k} calls, {bad} differ, {u} packet qwords not written natively")).collect()
}

/// Two commands draw the same: equal, or for VU input, between the same
/// other steps the native writes are a subset of the packet's with the
/// same values (a packet also uploads stale words the programs never
/// read: scratchpad and stack left-overs past a batch's count).
fn same_draw(native: &Cmd, packets: &Cmd) -> bool {
    let (Cmd::Vu(a), Cmd::Vu(b)) = (native, packets) else { return native == packets };
    type Seg = (std::collections::HashMap<(bool, u32), u128>, Option<Step>);
    let segs = |s: &[Step]| -> Vec<Seg> {
        let mut out = Vec::new();
        let mut m = std::collections::HashMap::new();
        for st in s {
            match st {
                Step::Write { addr, rel, data } => {
                    for (i, &v) in data.iter().enumerate() {
                        m.insert((*rel, *addr as u32 + i as u32), v);
                    }
                }
                other => out.push((std::mem::take(&mut m), Some(other.clone()))),
            }
        }
        out.push((m, None));
        out
    };
    let (sa, sb) = (segs(a), segs(b));
    sa.len() == sb.len() && sa.iter().zip(&sb).all(|((ma, ta), (mb, tb))| ta == tb && ma.iter().all(|(k, v)| mb.get(k) == Some(v)))
}

/// VU input qwords of `packets` that `native` does not write.
fn uncovered(native: &Cmd, packets: &Cmd) -> u64 {
    let (Cmd::Vu(a), Cmd::Vu(b)) = (native, packets) else { return 0 };
    let keys = |s: &[Step]| -> std::collections::HashSet<(usize, bool, u32)> {
        let mut seg = 0;
        let mut k = std::collections::HashSet::new();
        for st in s {
            match st {
                Step::Write { addr, rel, data } => k.extend((0..data.len() as u32).map(|i| (seg, *rel, *addr as u32 + i))),
                _ => seg += 1,
            }
        }
        k
    };
    let ka = keys(a);
    keys(b).difference(&ka).count() as u64
}

fn short(r: &[Recorded]) -> Vec<String> {
    r.iter()
        .map(|x| match &x.cmd {
            Cmd::Vu(steps) => {
                let s: Vec<String> = steps
                    .iter()
                    .map(|s| match s {
                        Step::Write { addr, data, .. } => format!("W{addr:#x}x{}", data.len()),
                        Step::Chain(a) => format!("C{a:#x}"),
                        Step::Start(e) => format!("S{e:#x}"),
                        s => format!("{s:?}"),
                    })
                    .collect();
                format!("{:#x}..{:#x} {}", x.start, x.end, s.join(" "))
            }
            c => format!("{:#x} {:.100}", x.start, format!("{c:?}")),
        })
        .collect()
}
