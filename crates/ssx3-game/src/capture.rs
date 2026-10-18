//! Draw capture (plan step N2, direct rendering, D1;
//! `docs/plans/native-rendering.md`): wrappers around the game's draw
//! functions. Each runs the generated body unchanged (guest memory, cycles
//! and event points stay exactly as without it) and records the draw as a
//! typed command for the renderer (`ssx3_render::direct`), keyed by the
//! packet the body wrote. Slots only: installed at boot unless
//! `SSX3_DIRECT=0`, not part of `OVERRIDES`; they record nothing while no
//! renderer takes the captures.

use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{self, Cmd, PostKind, Recorded, Step, STRIP_BATCH};

const DRAW_TRI_STRIP: u32 = 0x0037_9860;
const EMIT_BOARD_WAKE_STRIPS: u32 = 0x0038_85e0;
const EMIT_SHADOW_STRIPS: u32 = 0x0038_7ec0;
const DRAW_EMITTER: u32 = 0x0038_0518;
const DRAW_DYNAMIC_EMITTER: u32 = 0x0038_07a0;
const DRAW_M_EMITTER: u32 = 0x0038_0ce0;
const DRAW_TRAIL: u32 = 0x0038_1310;
const DRAW_TRAIL_B: u32 = 0x0038_16f0;
const DRAW_PATCH_FX: u32 = 0x0038_d690;
const DRAW_MODEL: u32 = 0x0037_e238;
const DRAW_SKINNED_MODEL: u32 = 0x0037_a610;
const BEZIER_END_PASS: u32 = 0x0038_b370;
const DEFER_MAN_APPEND_PACKET: u32 = 0x0036_2978;
const DRAW_SKINNED_MODEL_SHADOW: u32 = 0x0037_b548;
const GRID_MESH_DRAW: u32 = 0x0038_5c10;
const BEGIN_TRI_STRIP: u32 = 0x0037_a260;
const END_TRI_STRIP: u32 = 0x0037_a430;

/// Install the wrappers; false when switched off.
pub fn install() -> bool {
    if std::env::var("SSX3_DIRECT").is_ok_and(|v| v == "0") {
        return false;
    }
    for (a, f, name) in [
        (DRAW_TRI_STRIP, (|c: &mut Cpu| crate::packetless::dispatch(c, "DrawTriStrip", 1, crate::packetless::draw_tri_strip, draw_tri_strip)) as fn(&mut Cpu), "PS2GraphicsMan_DrawTriStrip"),
        (EMIT_BOARD_WAKE_STRIPS, emit_board_wake_strips, "PS2GraphicsMan_EmitBoardWakeStrips"),
        (EMIT_SHADOW_STRIPS, emit_shadow_strips, "PS2GraphicsMan_EmitShadowStrips"),
        (BEGIN_TRI_STRIP, |c: &mut Cpu| crate::packetless::dispatch_with(c, "BeginTriStrip", 0, &[0x5a28, 0x5a2c], crate::packetless::begin_tri_strip, begin_tri_strip), "PS2GraphicsMan_BeginTriStrip"),
        (END_TRI_STRIP, |c: &mut Cpu| crate::packetless::dispatch_with(c, "EndTriStrip", 1, &[0x5a28, 0x5a2c], crate::packetless::end_tri_strip, end_tri_strip), "PS2GraphicsMan_EndTriStrip"),
        (DRAW_EMITTER, |c: &mut Cpu| crate::packetless::dispatch(c, "DrawEmitter", 8, crate::packetless::draw_emitter, draw_emitter), "PS2GraphicsMan_DrawEmitter"),
        (DRAW_DYNAMIC_EMITTER, |c: &mut Cpu| crate::packetless::dispatch(c, "DrawDynamicEmitter", 8, crate::packetless::draw_dynamic_emitter, draw_dynamic_emitter), "PS2GraphicsMan_DrawDynamicEmitter"),
        (DRAW_M_EMITTER, |c: &mut Cpu| crate::packetless::dispatch(c, "DrawMEmitter", 8, crate::packetless::draw_m_emitter, draw_m_emitter), "PS2GraphicsMan_DrawMEmitter"),
        (DRAW_TRAIL, |c: &mut Cpu| crate::packetless::dispatch(c, "DrawTrail", 8, crate::packetless::draw_trail, draw_trail), "PS2GraphicsMan_DrawTrail"),
        (DRAW_TRAIL_B, |c: &mut Cpu| crate::packetless::dispatch(c, "DrawTrailB", 8, crate::packetless::draw_trail_b, draw_trail_b), "PS2GraphicsMan_DrawTrailB"),
        (DRAW_PATCH_FX, |c: &mut Cpu| crate::packetless::dispatch_patch_fx(c, draw_patch_fx), "PS2BezierMan_DrawPatchFX"),
        (DRAW_MODEL, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "DrawModel", crate::packetless::draw_model, draw_model), "PS2GraphicsMan_DrawModel"),
        (DRAW_SKINNED_MODEL, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "DrawSkinnedModel", crate::packetless::draw_skinned_model, draw_skinned_model), "PS2GraphicsMan_DrawSkinnedModel"),
        (BEZIER_END_PASS, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "BezierEndPass", crate::packetless::bezier_end_pass, bezier_end_pass), "PS2BezierMan_v18"),
        (0x0036_ac00, render_depth_fog, "PS2GraphicsMan_RenderDepthFog"),
        (0x0036_c790, render_bloom, "PS2GraphicsMan_RenderBloom"),
        (0x0039_04a0, render_color_filter, "PS2GraphicsMan_RenderColorFilter"),
        (0x0039_6b40, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "EmitLightingPacket", crate::packetless::emit_lighting_packet, emit_lighting_packet), "PS2GraphicsMan_EmitLightingPacket"),
        (0x0037_ba50, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "SetShadowHeader", crate::packetless::set_shadow_header, set_shadow_header), "PS2GraphicsMan_SetShadowHeader"),
        (DEFER_MAN_APPEND_PACKET, append_packet, "DeferMan_AppendPacket"),
        (DRAW_SKINNED_MODEL_SHADOW, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "DrawSkinnedModelShadow", crate::packetless::draw_skinned_model_shadow, draw_skinned_model_shadow), "PS2GraphicsMan_DrawSkinnedModelShadow"),
        (0x0037_d348, capture_setrendertargettexture, "SetRenderTargetTexture"),
        (0x0037_d450, capture_restorerendertarget, "RestoreRenderTarget"),
        (0x0037_d800, capture_fillrendertarget, "FillRenderTarget"),
        (0x0037_cc98, capture_drawtransition, "DrawTransition"),
        (0x0037_9bd0, capture_drawindexedtriangles, "DrawIndexedTriangles"),
        (0x0038_1ad0, capture_drawlinestrip, "DrawLineStrip"),
        (0x0038_1f10, capture_drawlineshape, "DrawLineShape"),
        (0x0037_7cf0, capture_drawsprite3d, "DrawSprite3D"),
        (0x0037_81a0, capture_drawrotatedsprites, "DrawRotatedSprites"),
        (0x0037_8808, capture_drawglyphs, "DrawGlyphs"),
        (0x0037_9028, capture_drawglyphsw, "DrawGlyphsW"),
        (0x0038_6e78, capture_drawshadowvolume, "DrawShadowVolume"),
        (0x0038_83b8, capture_drawboardwake, "DrawBoardWake"),
        (0x002d_c7b0, capture_fogparticleman_drawparticles, "FogParticleMan_DrawParticles"),
        (GRID_MESH_DRAW, |c: &mut Cpu| crate::packetless::dispatch_ram(c, "GridMeshDraw", crate::packetless::grid_mesh_draw, grid_mesh_draw), "cPS2GridMesh_Draw"),
    ] {
        assert!(ssx3_ee::set_override(a, f), "no override slot for {name}: list it in overrides.txt");
    }
    true
}

fn qword(c: &mut Cpu, a: u32) -> u128 {
    if a & 15 == 0 {
        return c.bus.read128(a);
    }
    (0..4).fold(0u128, |q, l| q | (c.bus.read32(a + 4 * l) as u128) << (32 * l))
}

/// `PS2GraphicsMan_DrawTriStrip(gm, n, verts)` (slot 71, 0x379860): n
/// vertices of 3 qwords {STQ, RGBA, XYZW}; the packet goes to `gm+0x5a00`.
fn draw_tri_strip(c: &mut Cpu) {
    let ret = c.g[RA][0] as u32;
    let (gm, n, verts) = (c.g[A0][0] as u32, c.g[A1][0] as i64, c.g[A2][0] as u32);
    let pre = direct::active().then(|| {
        let entry = if c.bus.read32(c.g[GP][0] as u32 + 0x1290) == 0 { 0x12b8 } else { 0x12b0 };
        let state = c.bus.read32(gm + 0xe84);
        let prim = if (c.bus.read32(state + 0x10) as u16 as i16) < 0 { 0x4c } else { 0x5c };
        let start = c.bus.read32(gm + 0x5a00);
        let n = n.max(0) as u32;
        let v: Vec<[u128; 3]> = (0..if n > 2 { n } else { 0 }).map(|i| std::array::from_fn(|k| qword(c, verts + 48 * i + 16 * k as u32))).collect();
        (entry, prim, start, v)
    });
    ssx3_ee::s12::f_00379860__generated(c);
    // (Not when the body unwound: the packet takes the decoding path.)
    let Some((entry, prim, start, verts)) = pre.filter(|p| c.pc == ret && !p.3.is_empty()) else { return };
    // The packet's first DMA tag: CNT over the first batch (tag, count,
    // 3 qwords per vertex, the MSCAL qword), VIF NOPs; built in the
    // scratchpad, so not read back from RAM here.
    let k = verts.len().min(STRIP_BATCH) as u128;
    let first_tag = (3 * k + 4) | 0x1000_0000;
    let end = c.bus.read32(gm + 0x5a00).wrapping_sub(0x10);
    direct::record(Recorded { start, end, first_tag, cmd: Cmd::Strip { entry, prim, verts } });
}

/// A ribbon emitter's packet as VU1 input, per program start: (offset from
/// TOP, qword). Built from the emitter's arguments and the vertex rings
/// exactly as its UNPACKs place them (literal ports below).
struct RibbonInput {
    calls: Vec<Vec<(u16, u128)>>,
    cur: Vec<(u16, u128)>,
}

impl RibbonInput {
    fn new() -> Self {
        Self { calls: Vec::new(), cur: Vec::new() }
    }
    /// `n` vertices (3 qwords each) from guest memory at `src`, unpacked at
    /// offset `at`; `None` for a count the packet could not have.
    fn verts(&mut self, c: &mut Cpu, at: u32, src: u32, n: i32) -> Option<()> {
        if !(0..=0x48).contains(&n) {
            return None;
        }
        for i in 0..3 * n as u32 {
            let q = qword(c, src.wrapping_add(16 * i));
            self.cur.push(((at + i) as u16, q));
        }
        Some(())
    }
    /// The strip tag and the counts / alpha ramp (offsets 0, 1).
    fn head(&mut self, n: i32, tag_lo: u64, regs: u64, alpha: FpuF32, step: FpuF32) {
        let n = n as u128;
        self.cur.push((0, (2 * n) | tag_lo as u128 | (regs as u128) << 64));
        self.cur.push((1, 3 * n | n << 32 | (step.0 as u128) << 64 | (alpha.0 as u128) << 96));
    }
    fn start(&mut self) {
        self.calls.push(std::mem::take(&mut self.cur));
    }
}

/// The emitters' entry: 0x3a08, or 0x3a00 once after the flag at
/// `gp + flag` was set (the body clears it).
fn ribbon_entry(c: &mut Cpu, flag: u32) -> u32 {
    if c.bus.read32(c.g[GP][0] as u32 + flag) == 0 { 0x3a08 } else { 0x3a00 }
}

/// Run a ribbon emitter's body and record its segment of the packet.
fn capture_ribbon(c: &mut Cpu, body: fn(&mut Cpu), input: Option<(u32, RibbonInput)>) {
    let ret = c.g[RA][0] as u32;
    let gm = c.g[A0][0] as u32;
    let start = c.bus.read32(gm + 0x5a00);
    body(c);
    let Some((entry, input)) = input.filter(|(_, i)| c.pc == ret && !i.calls.is_empty()) else { return };
    let end = c.bus.read32(gm + 0x5a00);
    direct::record(Recorded { start, end, first_tag: 0x1000_0003, cmd: Cmd::Ribbon { entry, calls: input.calls } });
}

/// `PS2GraphicsMan_EmitBoardWakeStrips(f12 alpha, f13 step, gm, rings,
/// first, ring size, count, ring A, ring B)` (0x3885e0).
fn emit_board_wake_strips(c: &mut Cpu) {
    let input = direct::active().then(|| board_wake_input(c)).flatten();
    capture_ribbon(c, ssx3_ee::s12::f_003885e0__generated, input);
}

fn board_wake_input(c: &mut Cpu) -> Option<(u32, RibbonInput)> {
    let entry = ribbon_entry(c, 0x12c0);
    let (mut alpha, step) = (FpuF32(c.f[12]), FpuF32(c.f[13]));
    let rings = c.g[A1][0] as u32;
    let (mut p5, p6, mut p7) = (c.g[A2][0] as i32, c.g[A3][0] as i32, c.g[T0][0] as i32);
    let ring_a = c.bus.read32(rings.wrapping_add(4 * c.g[T1][0] as u32));
    let ring_b = c.bus.read32(rings.wrapping_add(4 * c.g[T2][0] as u32));
    let mut r = RibbonInput::new();
    let mut first = true;
    while p7 != 0 {
        if p7 < 0 || p6 <= 0 || r.calls.len() > 64 {
            return None;
        }
        let n = p7.min(0x24);
        let mut l12 = n.min(p6 - p5);
        let i13 = (n - l12).max(0);
        let (mut i10, mut i15) = (1, p5);
        if first {
            l12 -= 1;
            i15 = p5 + 1;
            i10 = p5;
        }
        r.head(n, 0x302e_4000_0000_8000, 0x412, alpha, step);
        let mut at = 2u32;
        for ring in [ring_a, ring_b] {
            if i10 >= 0 {
                r.verts(c, at, ring.wrapping_add(i10 as u32 * 0x30), 1)?;
                at += 3;
            }
            if l12 != 0 {
                r.verts(c, at, ring.wrapping_add(i15 as u32 * 0x30), l12)?;
                at += 3 * l12 as u32;
            }
            if i13 != 0 {
                r.verts(c, at, ring, i13)?;
                at += 3 * i13 as u32;
            }
        }
        r.start();
        p7 -= n;
        alpha = alpha + step * FpuF32::from_i32(n);
        p5 = i15;
        if p7 > 0 {
            p7 += 1;
            p5 = (i15 + n - 1) % p6;
            first = false;
        }
    }
    Some((entry, r))
}

/// `PS2GraphicsMan_EmitShadowStrips(f12 alpha, f13 step, gm, rings, first,
/// count, ring A, ring B, textured)` (0x387ec0): rings of 0x36 vertices.
fn emit_shadow_strips(c: &mut Cpu) {
    let input = direct::active().then(|| shadow_input(c)).flatten();
    capture_ribbon(c, ssx3_ee::s12::f_00387ec0__generated, input);
}

fn shadow_input(c: &mut Cpu) -> Option<(u32, RibbonInput)> {
    let entry = ribbon_entry(c, 0x12b0);
    let (mut alpha, step) = (FpuF32(c.f[12]), FpuF32(c.f[13]));
    let rings = c.g[A1][0] as u32;
    let (mut p5, mut p6) = (c.g[A2][0] as i32, c.g[A3][0] as i32);
    let ring_a = c.bus.read32(rings.wrapping_add(4 * c.g[T0][0] as u32));
    let ring_b = c.bus.read32(rings.wrapping_add(4 * c.g[T1][0] as u32));
    let (prim, regs) = if c.g[T2][0] == 0 { (4u64, 0x41f) } else { (0x5c, 0x412) };
    let tag_lo = 0x8000 | prim << 47 | 1 << 46 | 3 << 60;
    // The closing pair's default (a game global, -1/-1 when unused).
    let default_pair = c.bus.read32(0x004a_43a8) as u64 | (c.bus.read32(0x004a_43ac) as u64) << 32;
    let mut r = RibbonInput::new();
    while p6 != 0 {
        if p6 < 0 || r.calls.len() > 64 {
            return None;
        }
        let n = p6.min(0x24);
        let mut l14 = n.min(0x36 - p5);
        let mut i12 = (n - l14).max(0);
        let mut pair = default_pair;
        if p6 <= n {
            pair = (((p5 + n - 1) % 0x36) as u32 as u64) << 32 | ((p5 + n - 2) % 0x36) as u32 as u64;
            if i12 < 2 {
                l14 = l14 - 2 + i12;
                i12 = 0;
            } else {
                i12 -= 2;
            }
        }
        r.head(n, tag_lo, regs, alpha, step);
        let mut at = 2u32;
        for ring in [ring_a, ring_b] {
            if l14 != 0 {
                r.verts(c, at, ring.wrapping_add(p5 as u32 * 0x30), l14)?;
                at += 3 * l14 as u32;
            }
            if i12 != 0 {
                r.verts(c, at, ring, i12)?;
                at += 3 * i12 as u32;
            }
            if pair as i32 >= 0 {
                r.verts(c, at, ring.wrapping_add((pair as u32).wrapping_mul(0x30)), 1)?;
                r.verts(c, at + 3, ring.wrapping_add(((pair >> 32) as u32).wrapping_mul(0x30)), 1)?;
                at += 6;
            }
        }
        r.start();
        p6 -= n;
        alpha = alpha + step * FpuF32::from_i32(n);
        if p6 > 0 {
            p6 += 1;
            p5 = (p5 + n - 1) % 0x36;
        }
    }
    Some((entry, r))
}

/// A strip begun with `BeginTriStrip`: entry, PRIM, packet start, vertex
/// count, where the caller writes the vertices (scratchpad).
static OPEN_STRIP: std::sync::Mutex<Option<(u32, u32, u32, u32, u32)>> = std::sync::Mutex::new(None);

/// `PS2GraphicsMan_BeginTriStrip(gm, n)` (0x37a260): one strip of `n`
/// vertices in a scratchpad block; the caller writes the vertices at
/// block + 0x40 (`gm+0x5a2c` points past them), `EndTriStrip` sends it.
fn begin_tri_strip(c: &mut Cpu) {
    let ret = c.g[RA][0] as u32;
    let (gm, n) = (c.g[A0][0] as u32, c.g[A1][0] as i64);
    let pre = direct::active().then(|| {
        let entry = if c.bus.read32(c.g[GP][0] as u32 + 0x1290) == 0 { 0x12b8 } else { 0x12b0 };
        let state = c.bus.read32(gm + 0xe84);
        let prim = if (c.bus.read32(state + 0x10) as u16 as i16) < 0 { 0x4c } else { 0x5c };
        (entry, prim, c.bus.read32(gm + 0x5a00))
    });
    ssx3_ee::s12::f_0037a260__generated(c);
    let mut open = OPEN_STRIP.lock().unwrap();
    *open = None;
    // (One batch: the strip program takes at most 72 vertices.)
    if let Some((entry, prim, start)) = pre.filter(|_| c.pc == ret && (3..=STRIP_BATCH as i64).contains(&n)) {
        let n = n as u32;
        let verts = c.bus.read32(gm + 0x5a2c).wrapping_sub(0x10 + 0x30 * n);
        *open = Some((entry, prim, start, n, verts));
    }
}

/// `PS2GraphicsMan_EndTriStrip(gm)` (0x37a430).
fn end_tri_strip(c: &mut Cpu) {
    let ret = c.g[RA][0] as u32;
    let gm = c.g[A0][0] as u32;
    let open = OPEN_STRIP.lock().unwrap().take();
    let pre = open.map(|(entry, prim, start, n, at)| {
        let verts: Vec<[u128; 3]> = (0..n).map(|i| std::array::from_fn(|k| qword(c, at + 48 * i + 16 * k as u32))).collect();
        (entry, prim, start, verts)
    });
    ssx3_ee::s12::f_0037a430__generated(c);
    let Some((entry, prim, start, verts)) = pre.filter(|_| c.pc == ret) else { return };
    let first_tag = (3 * verts.len() as u128 + 4) | 0x1000_0000;
    let end = c.bus.read32(gm + 0x5a00).wrapping_sub(0x10);
    direct::record(Recorded { start, end, first_tag, cmd: Cmd::Strip { entry, prim, verts } });
}

// ---------------------------------------------------------------------------
// Packets read back (stage A): functions whose packets are one CNT block of
// plain VIFcodes, built in the scratchpad and in RAM when the body returns
// (the fromSPR DMA completes at once here). Each is read back as `Step`s;
// stage C computes them from the function's inputs instead.

/// Where a packet ends.
#[derive(Clone, Copy, PartialEq)]
enum Shape {
    /// After its first CNT block.
    OneCnt,
    /// After the CALL into its mesh chain (models).
    UntilCall,
}

/// The VU1 effects of the packet at `start` and its end, or `None` for a
/// packet this reader does not take (other tags, masked UNPACKs, rows,
/// MSCNT; it then stays on the decoding path). The VIFcodes run across its
/// transfers (CNT data inline, REF data where it points), as the VIF sees
/// them. UNPACKs assume CL = WL, as the draw's state change leaves them
/// for these types (the capture check compares).
fn read_packet(c: &mut Cpu, start: u32, shape: Shape) -> Option<(u32, Vec<Step>)> {
    let mut steps = Vec::new();
    let mut words: Vec<u32> = Vec::new();
    let mut t = start;
    for _ in 0..256 {
        let tag = qword(c, t);
        let qwc = (tag & 0xffff) as u32;
        let addr = ((tag >> 32) & 0x7fff_fff0) as u32;
        let (data, next) = match (tag >> 28) & 7 {
            1 => (t + 16, t + 16 + 16 * qwc),
            3 if (tag >> 63) & 1 == 0 => (addr, t + 16),
            5 if shape == Shape::UntilCall && qwc == 0 => {
                parse_codes(&words, &mut steps)?;
                steps.push(Step::Chain(addr));
                return Some((t + 16, steps));
            }
            _ => return None,
        };
        for i in 0..qwc {
            let q = qword(c, data + 16 * i);
            words.extend((0..4).map(|l| (q >> (32 * l)) as u32));
        }
        t = next;
        if shape == Shape::OneCnt {
            parse_codes(&words, &mut steps)?;
            return Some((t, steps));
        }
    }
    None
}

/// The VU1 effects of the packet `start..end` (a DeferMan record's range):
/// CNT and REF transfers, CALLs into chains (their own data first) as
/// `Step::Chain`; `None` for anything else.
fn read_range(c: &mut Cpu, start: u32, end: u32) -> Option<Vec<Step>> {
    let mut steps = Vec::new();
    let mut words: Vec<u32> = Vec::new();
    let mut t = start;
    for _ in 0..4096 {
        if t == end {
            parse_codes(&words, &mut steps)?;
            return Some(steps);
        }
        let tag = qword(c, t);
        let qwc = (tag & 0xffff) as u32;
        let addr = ((tag >> 32) & 0x7fff_fff0) as u32;
        let (data, next, call) = match (tag >> 28) & 7 {
            1 => (t + 16, t + 16 + 16 * qwc, false),
            3 if (tag >> 63) & 1 == 0 => (addr, t + 16, false),
            // (A REF into the scratchpad: the shine pass of a patch without
            // normals reads whatever is there; taken as it is now.)
            3 => (0x7000_0000 | (addr & 0x3ff0), t + 16, false),
            5 => (t + 16, t + 16 + 16 * qwc, true),
            _ => return None,
        };
        for i in 0..qwc {
            let q = qword(c, data + 16 * i);
            words.extend((0..4).map(|l| (q >> (32 * l)) as u32));
        }
        if call {
            parse_codes(&words, &mut steps)?;
            words.clear();
            steps.push(Step::Chain(addr));
        }
        t = next;
    }
    None
}

/// Run a draw function's body and record the packets of the DeferMan
/// records it wrote (one command per distinct packet), read back. Nothing
/// when the body unwound; a packet `read_range` does not take is left out.
fn capture_records(c: &mut Cpu, body: fn(&mut Cpu), gm: u32) {
    let ret = c.g[RA][0] as u32;
    let on = direct::active();
    let dm = c.bus.read32(gm + 0x18f0);
    let n0 = c.bus.read32(dm);
    body(c);
    if !on || c.pc != ret {
        return;
    }
    let n1 = c.bus.read32(dm).min(0xa28);
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for i in n0..n1 {
        let rec = dm + 0x80 + 0x80 * i;
        let (start, end) = (c.bus.read32(rec + 0x14), c.bus.read32(rec + 0x18));
        // (An empty packet: the chain CALLs a bare link; nothing to draw.)
        if !seen.insert(start) || start == end {
            continue;
        }
        // (A packet the reader does not take stays on the decoding path.)
        let Some(steps) = read_range(c, start, end) else {
            if debug() {
                let tags: Vec<String> = (0..6).map(|k| format!("{:x}", qword(c, start + 16 * k) as u64)).collect();
                eprintln!("[capture] packet {start:#x}..{end:#x} not read: {}", tags.join(" "));
            }
            continue;
        };
        out.push(Recorded { start, end, first_tag: qword(c, start), cmd: Cmd::Vu(steps) });
    }
    for r in out {
        direct::record(r);
    }
}

/// VIFcodes as steps; `None` for a code this reader does not take or one
/// left waiting for data.
fn parse_codes(words: &[u32], steps: &mut Vec<Step>) -> Option<()> {
    let mut p = 0;
    while p < words.len() {
        let cmd = words[p];
        p += 1;
        let (op, imm) = ((cmd >> 24) & 0x7f, cmd & 0xffff);
        match op {
            0x00 | 0x10 | 0x11 | 0x13 => {}
            0x01 => steps.push(Step::Cycle(imm)),
            0x04 => steps.push(Step::Itop(imm & 0x3ff)),
            0x05 => steps.push(Step::Mode(imm & 3)),
            0x06 => steps.push(Step::MaskPath3((imm >> 15) & 1 != 0)),
            0x14 => steps.push(Step::Start(imm * 8)),
            0x20 => {
                steps.push(Step::Mask(*words.get(p)?));
                p += 1;
            }
            0x50 | 0x51 => {
                let n = if imm == 0 { 65536 } else { imm as usize };
                let d = words.get(p..p + 4 * n)?;
                p += 4 * n;
                steps.push(Step::Gif(d.chunks_exact(4).map(|w| (0..4).fold(0u128, |q, l| q | (w[l] as u128) << (32 * l))).collect()));
            }
            _ if op & 0x60 == 0x60 && op & 0x10 == 0 => {
                let (vn, vl, usn) = ((cmd >> 26) & 3, (cmd >> 24) & 3, (cmd >> 14) & 1 != 0);
                let num = match (cmd >> 16) & 0xff {
                    0 => 256,
                    n => n,
                } as usize;
                // (V4-5 is the only vl = 3 format.)
                if vl == 3 && vn != 3 {
                    return None;
                }
                let (elems, bits) = if vl == 3 { (1, 16) } else { (vn as usize + 1, 32 >> vl) };
                let n_words = (num * elems * bits).div_ceil(32);
                let d = words.get(p..p + n_words)?;
                p += n_words;
                let elem = |k: usize| -> u32 {
                    let bit = k * bits;
                    let v = d[bit / 32] >> (bit % 32);
                    match bits {
                        32 => v,
                        16 if usn || vl == 3 => v & 0xffff,
                        16 => v as u16 as i16 as i32 as u32,
                        _ if usn => v & 0xff,
                        _ => v as u8 as i8 as i32 as u32,
                    }
                };
                // What an unmasked UNPACK stores (vif.rs `unpack`).
                let vector = |i: usize| -> [u32; 4] {
                    let e = |j: usize| elem(i * elems + j);
                    match (vl, elems) {
                        (3, _) => {
                            let c = e(0);
                            [(c & 0x1f) << 3, ((c >> 5) & 0x1f) << 3, ((c >> 10) & 0x1f) << 3, ((c >> 15) & 1) << 7]
                        }
                        (_, 1) => [e(0); 4],
                        (_, 2) => [e(0), e(1), e(0), e(1)],
                        (_, 3) => [e(0), e(1), e(2), 0],
                        _ => [e(0), e(1), e(2), e(3)],
                    }
                };
                let data = (0..num).map(|i| vector(i).iter().enumerate().fold(0u128, |q, (l, &x)| q | (x as u128) << (32 * l))).collect();
                steps.push(Step::Write { addr: (cmd & 0x3ff) as u16, rel: (cmd >> 15) & 1 != 0, data });
            }
            // (Anything else: the rest as VIFcodes for the VIF to run,
            // padded with NOPs to whole quadwords.)
            _ => {
                let mut rest = words[p - 1..].to_vec();
                rest.resize(rest.len().next_multiple_of(4), 0);
                steps.push(Step::Raw(rest.chunks_exact(4).map(|w| (0..4).fold(0u128, |q, l| q | (w[l] as u128) << (32 * l))).collect()));
                return Some(());
            }
        }
    }
    Some(())
}

/// Run a body that writes packets at the cursor `*cursor`, one record each
/// (the next one 0x10 after the end), and record them read back. Nothing
/// when `kill` (a gp-relative debug flag) is set, the body unwound or a
/// packet is not one `read_packet` takes.
fn capture_packets(c: &mut Cpu, body: fn(&mut Cpu), cursor: u32, kill: Option<u32>, shape: Shape) {
    let ret = c.g[RA][0] as u32;
    let on = direct::active() && kill.is_none_or(|k| c.bus.read32(c.g[GP][0] as u32 + k) == 0);
    let start = c.bus.read32(cursor);
    body(c);
    if !on || c.pc != ret {
        return;
    }
    let end = c.bus.read32(cursor);
    let mut at = start;
    let mut out = Vec::new();
    while at + 0x10 < end {
        let Some((e, steps)) = read_packet(c, at, shape) else { return };
        out.push(Recorded { start: at, end: e, first_tag: qword(c, at), cmd: Cmd::Vu(steps) });
        at = e + 0x10;
    }
    if at != end {
        return;
    }
    for r in out {
        direct::record(r);
    }
}

/// Slot 82 `DrawEmitter(gm, desc)` (0x380518): a type-4 emitter.
fn draw_emitter(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_00380518__generated, gm + 0x5a00, Some(0x1408), Shape::OneCnt);
}

/// Slot 83 `DrawDynamicEmitter(gm, e)` (0x3807a0): ring emitters, batches of
/// 64.
fn draw_dynamic_emitter(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_003807a0__generated, gm + 0x5a00, Some(0x140c), Shape::OneCnt);
}

/// Slot 84 `DrawMEmitter(gm, e)` (0x380ce0): ring emitters with colours,
/// batches of 43.
fn draw_m_emitter(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_00380ce0__generated, gm + 0x5a00, Some(0x140c), Shape::OneCnt);
}

/// Slot 85 `DrawTrail(gm, d)` (0x381310): SnowBox flakes (type 5).
fn draw_trail(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_00381310__generated, gm + 0x5a00, None, Shape::OneCnt);
}

/// Slot 86 `DrawTrailB(gm, d)` (0x3816f0): the depth-faded variant.
fn draw_trail_b(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_003816f0__generated, gm + 0x5a00, None, Shape::OneCnt);
}

/// `PS2BezierMan_DrawPatchFX(bm, patch, entry, cursor)` (0x38d690): patch
/// glitter (type 4, entry 0x1460).
fn draw_patch_fx(c: &mut Cpu) {
    let cursor = c.g[A3][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_0038d690__generated, cursor, None, Shape::OneCnt);
}

/// Slot 95 `DrawModel(f12 alpha, gm, inst, ...)` (0x37e238): per strip
/// group a header packet and a CALL into the model's mesh chain.
fn draw_model(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_packets(c, ssx3_ee::s12::f_0037e238__generated, gm + 0x5a00, None, Shape::UntilCall);
}

/// `cPS2GridMesh_Draw(this, matrix, texId, mat2)` (0x385c10): one header
/// packet and a CALL into the grid's (double-buffered) chain.
fn grid_mesh_draw(c: &mut Cpu) {
    let gm = c.bus.read32(c.g[A0][0] as u32 + 0x1c4);
    capture_packets(c, ssx3_ee::s12::f_00385c10__generated, gm + 0x5a00, None, Shape::UntilCall);
}

/// Slot 99 `DrawSkinnedModel(gm, model, base, outline)` (0x37a610): per
/// mesh group one record: bone batches, then CALLs of the mesh (and morph)
/// chains.
fn draw_skinned_model(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_records(c, ssx3_ee::s12::f_0037a610__generated, gm);
}

/// Slot 100 `DrawSkinnedModelShadow` (0x37b548): one record per call.
fn draw_skinned_model_shadow(c: &mut Cpu) {
    let gm = c.g[A0][0] as u32;
    capture_records(c, ssx3_ee::s12::f_0037b548__generated, gm);
}

/// `SSX3_CAPTURE_DEBUG=1`: packets that are not captured are reported.
fn debug() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_CAPTURE_DEBUG").is_some())
}

/// The PS2GraphicsMan (`gp+0x2a90`).
fn gm_of(c: &mut Cpu) -> u32 {
    c.bus.read32(c.g[GP][0] as u32 + 0x2a90)
}

/// Draw functions whose packets are captured through the records they
/// write (`capture_records`; specs in `docs/investigation/n2-capture/`).
macro_rules! record_captures {
    ($($name:ident = $shard:ident :: $f:ident; $doc:literal)*) => {
        $(
            #[doc = $doc]
            fn $name(c: &mut Cpu) {
                let gm = gm_of(c);
                capture_records(c, ssx3_ee::$shard::$f, gm);
            }
        )*
    };
}

record_captures! {
    capture_setrendertargettexture = s12::f_0037d348__generated; "slot 53: render into a texture"
    capture_restorerendertarget = s12::f_0037d450__generated; "slot 54: back to the frame buffer"
    capture_fillrendertarget = s12::f_0037d800__generated; "slot 55: fill the target, inset the scissor"
    capture_drawtransition = s12::f_0037cc98__generated; "slot 61: screen copy into the transition texture"
    capture_drawindexedtriangles = s12::f_00379bd0__generated; "slot 72"
    capture_drawlinestrip = s12::f_00381ad0__generated; "slot 73"
    capture_drawlineshape = s12::f_00381f10__generated; "slot 75 (VU1 entry 0x3bf0)"
    capture_drawsprite3d = s12::f_00377cf0__generated; "slot 78"
    capture_drawrotatedsprites = s12::f_003781a0__generated; "slot 79"
    capture_drawglyphs = s12::f_00378808__generated; "slot 80"
    capture_drawglyphsw = s12::f_00379028__generated; "slot 81"
    capture_drawshadowvolume = s12::f_00386e78__generated; "slot 89: DIRECT segments and shadow ribbons, one record"
    capture_drawboardwake = s12::f_003883b8__generated; "slot 90: wake ribbons, one record"
    capture_fogparticleman_drawparticles = s09::f_002dc7b0__generated; "cPS2FogParticleMan_DrawParticles: GIF sprites"
}

/// `PS2BezierMan_v18(bm)` (0x38b370): the terrain patches drawn this frame
/// (immediate patches, their shine / projector passes, the glitter), by
/// their records; the deferred groups go through `DeferMan_AppendPacket`.
fn bezier_end_pass(c: &mut Cpu) {
    let gm = gm_of(c);
    capture_records(c, ssx3_ee::s12::f_0038b370__generated, gm);
}

/// `DeferMan_AppendPacket(bucket, start, end)` (0x362978): a packet added
/// to a bucket without a record (the opaque model fast path, the deferred
/// terrain groups), read back.
fn append_packet(c: &mut Cpu) {
    let ret = c.g[RA][0] as u32;
    let (start, end) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
    // (The hand-written replacement unless every function runs generated.)
    static GENERATED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    #[cfg(feature = "render")]
    let native = !*GENERATED.get_or_init(|| std::env::var_os("SSX3_NO_OVERRIDES").is_some());
    #[cfg(not(feature = "render"))]
    let native = { let _ = &GENERATED; false };
    if native {
        #[cfg(feature = "render")]
        crate::render::defer_man_append_packet(c);
    } else {
        ssx3_ee::s12::f_00362978__generated(c);
    }
    if !direct::active() || c.pc != ret || start == end {
        return;
    }
    if let Some(steps) = read_range(c, start, end) {
        direct::record(Recorded { start, end, first_tag: qword(c, start), cmd: Cmd::Vu(steps) });
    }
}

// ---------------------------------------------------------------------------
// What RenderFrame emits besides the buckets' packets (stage B: the frame
// built without walking the chain).

/// A post pass for one view (`(gm, view, &vifCursor, &gifCursor)`): the
/// GIF data it queued (CNT / REF transfers from the GIF cursor) and the
/// PATH3 windows it put into the VIF chain.
fn capture_post(c: &mut Cpu, body: fn(&mut Cpu), kind: PostKind) {
    let ret = c.g[RA][0] as u32;
    let (vif_at, gif_at) = (c.g[A2][0] as u32, c.g[A3][0] as u32);
    let (vif0, gif0) = (c.bus.read32(vif_at), c.bus.read32(gif_at));
    body(c);
    if !direct::active() || c.pc != ret {
        return;
    }
    let (vif1, gif1) = (c.bus.read32(vif_at), c.bus.read32(gif_at));
    let mut gif = Vec::new();
    let mut t = gif0;
    while t != gif1 {
        let tag = qword(c, t);
        let qwc = (tag & 0xffff) as u32;
        let (data, next) = match (tag >> 28) & 7 {
            1 => (t + 16, t + 16 + 16 * qwc),
            3 => (((tag >> 32) & 0x7fff_fff0) as u32, t + 16),
            _ => return,
        };
        gif.extend((0..qwc).map(|i| qword(c, data + 16 * i)));
        t = next;
        if gif.len() > 1 << 20 {
            return;
        }
    }
    // (Each window is one REF to the static window code.)
    let mut windows = 0;
    let mut t = vif0;
    while t != vif1 {
        let tag = qword(c, t);
        let qwc = (tag & 0xffff) as u32;
        t = match (tag >> 28) & 7 {
            1 => t + 16 + 16 * qwc,
            3 => {
                windows += 1;
                t + 16
            }
            _ => return,
        };
    }
    direct::record(Recorded { start: vif0, end: vif1, first_tag: qword(c, vif0), cmd: Cmd::Post { kind, gif, windows } });
}

fn render_depth_fog(c: &mut Cpu) {
    capture_post(c, ssx3_ee::s12::f_0036ac00__generated, PostKind::DepthFog);
}

fn render_bloom(c: &mut Cpu) {
    capture_post(c, ssx3_ee::s12::f_0036c790__generated, PostKind::Bloom);
}

fn render_color_filter(c: &mut Cpu) {
    capture_post(c, ssx3_ee::s12::f_003904a0__generated, PostKind::ColorFilter);
}

/// A packet that is one RET transfer with its data (the w3 packets the
/// state change CALLs), at the cursor `gm+0x5a00` before the body.
fn capture_ret_packet(c: &mut Cpu, body: fn(&mut Cpu)) {
    let ret = c.g[RA][0] as u32;
    let gm = c.g[A0][0] as u32;
    let start = c.bus.read32(gm + 0x5a00);
    body(c);
    if !direct::active() || c.pc != ret {
        return;
    }
    let tag = qword(c, start);
    if (tag >> 28) & 7 != 6 {
        return;
    }
    let qwc = (tag & 0xffff) as u32;
    let mut words = Vec::new();
    for i in 0..qwc {
        let q = qword(c, start + 16 + 16 * i);
        words.extend((0..4).map(|l| (q >> (32 * l)) as u32));
    }
    let mut steps = Vec::new();
    if parse_codes(&words, &mut steps).is_some() {
        direct::record(Recorded { start, end: start + 16 * (qwc + 1), first_tag: tag, cmd: Cmd::Vu(steps) });
    }
}

/// Slot 124 `EmitLightingPacket(gm)` (0x396b40): the type-6 lighting
/// packet (10 lighting qwords at VU1 7, MSCAL 0x1150), CALLed by the state
/// change through w3.
fn emit_lighting_packet(c: &mut Cpu) {
    capture_ret_packet(c, ssx3_ee::s13::f_00396b40__generated);
}

/// Slot 101 `SetShadowHeader(gm, matrix, scale, offset)` (0x37ba50): the
/// type-7 header packet (VU1 qw0-5), CALLed through w3.
fn set_shadow_header(c: &mut Cpu) {
    capture_ret_packet(c, ssx3_ee::s12::f_0037ba50__generated);
}
