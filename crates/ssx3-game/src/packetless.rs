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
use ssx3_render::direct::{self, Cmd, Recorded, STRIP_BATCH};

use crate::call::{call_ptr, guest};

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
// Checks (`SSX3_DRAW_CHECK=1`).

pub(crate) fn check_on() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_DRAW_CHECK").is_some())
}

/// Per checked function: calls, mismatches.
static STATS: std::sync::Mutex<Vec<(&'static str, u64, u64)>> = std::sync::Mutex::new(Vec::new());

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
    let mut w = Watch { ranges: vec![(st, st + 0x14), (dm, dm + 4), (cursor, cursor + 4)], ignore: vec![cursor] };
    for k in 0..records {
        let r = rec + 0x80 * k;
        w.ranges.push((r, r + 0x20));
        w.ignore.extend([r + 0x14, r + 0x18]);
    }
    w
}

/// A draw function's override: its native form while packetless (checked
/// against the generated one with `SSX3_DRAW_CHECK=1`), else `packets`
/// (the generated body with its capture).
pub(crate) fn dispatch(c: &mut Cpu, name: &'static str, records: u32, native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    dispatch_with(c, name, records, &[], native, packets)
}

/// [`dispatch`] with more `gm` fields watched (offsets).
pub(crate) fn dispatch_with(c: &mut Cpu, name: &'static str, records: u32, gm_fields: &[u32], native: fn(&mut Cpu), packets: fn(&mut Cpu)) {
    if !direct::packetless() {
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
    let rec_packets: Vec<Recorded> = {
        let r = direct::split_off(m);
        for x in &r {
            direct::record(x.clone());
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
    if cmds(&rec_native) != cmds(&rec_packets) {
        bad.push(format!("draws: native {:?} / packets {:?}", short(&rec_native), short(&rec_packets)));
    }
    let mut stats = STATS.lock().unwrap();
    let i = match stats.iter().position(|s| s.0 == name) {
        Some(i) => i,
        None => {
            stats.push((name, 0, 0));
            stats.len() - 1
        }
    };
    let s = &mut stats[i];
    s.1 += 1;
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
    STATS.lock().unwrap().iter().map(|(n, k, bad)| format!("{n}: {k} calls, {bad} differ")).collect()
}

fn short(r: &[Recorded]) -> Vec<String> {
    r.iter().map(|x| format!("{:.120}", format!("{:?}", x.cmd))).collect()
}
