//! Stage 0b of `docs/investigation/phase3-render.md`: one rendered frame as
//! typed draws.
//!
//! [`build`] reads, from EE memory only (a [`Mem`]), what
//! `PS2GraphicsMan_RenderFrame` (0x363490) left behind after it emitted the
//! frame's VIF1 chain, and before `DeferMan_ResetFrame` clears it:
//!
//! - the DeferMan sorted bucket list (+0x67ca4/+0x67ca8) and each bucket's
//!   continuation chain (+0x18): one [`Draw`] per bucket, in emission order;
//! - each bucket's interned [`RenderState`] (+0, decoded), textures (+4),
//!   matrix index (+8) and packet range (+0xc first start = CALL target,
//!   +0x10 last end = where the RET was written);
//! - the MatrixRecords (+0x69cd0), scissor table (+0x6aed4), the per-view
//!   post-effect parameters, and the published chain heads (gm+0x5aa8..);
//! - the layer split (thresholds at 0x4a4058, replicated with the game's
//!   binary search) into the depth-fog, phase-boundary, bloom and colour
//!   filter markers.
//!
//! Each draw's payload comes from walking the published chain with the
//! stage-0a VIF interpreter ([`crate::vif`]): when the top-level chain
//! CALLs the next bucket's first packet, everything until the matching RET
//! (which must sit at the bucket's last end) is that draw's payload. What the
//! chain emits between buckets is split into the draw's `prelude` (state
//! change from the last `CNT 1 {FLUSHE}` on, plus the texture bind), and the
//! default-state / post-effect segments. One VIF interpreter runs over the
//! whole frame, so double-buffer parity and STCYCL/BASE/OFFSET are what the
//! VU sees.
//!
//! The builder also simulates `DeferMan_EmitStateChange` /
//! `DeferMan_EmitDefaultState` on the decoded states ([`Emitter`]) to predict
//! the GS registers (SCISSOR/ZBUF/TEST/CLAMP/ALPHA, both contexts) each draw
//! runs with, and records what the chain actually wrote ([`ChainObs`]) so the
//! two can be compared.

use std::collections::HashMap;
use std::ops::Range;

use crate::chain::Mem;
use crate::gif::{reg, GifReader};
use crate::vif::{Event, Vif1, VifRegs, VuCall};

/// Fixed guest addresses.
pub mod addr {
    /// `PS2GraphicsMan *` singleton (gp+0x2a90).
    pub const GM_PTR: u32 = 0x004a_5b80;
    /// Default RenderState `{0, 0xb00294, 0x80, 0, 0xffffffff}`.
    pub const DEFAULT_STATE: u32 = 0x0050_1420;
    /// Layer thresholds `{6, 9}` that split the sorted buckets.
    pub const LAYER_THRESHOLDS: u32 = 0x004a_4058;
}

/// PS2GraphicsMan offsets (`structs/render.md`).
pub mod gm {
    pub const DEFER: u32 = 0x18f0;
    pub const TEXMAN: u32 = 0x18f4;
    pub const SCREEN_OFFSET: u32 = 0x0000;
    pub const VIEW_MATRICES: u32 = 0x5930;
    pub const NUM_VIEWS: u32 = 0x59c0;
    pub const BLIT: u32 = 0x59e8;
    pub const BRIGHTNESS_RGBA: u32 = 0x59ec;
    pub const BRIGHTNESS: u32 = 0x59fc;
    pub const FB_WIDTH: u32 = 0x5a30;
    pub const FB_HEIGHT: u32 = 0x5a34;
    pub const ZPSM: u32 = 0x5a40;
    pub const ZBP: u32 = 0x5a80;
    pub const DRAW_FBP: u32 = 0x5a84;
    pub const KICK_SEMA: u32 = 0x5ac8;
    pub const QUEUED_VIF1: u32 = 0x5aa8;
    pub const QUEUED_VIF2: u32 = 0x5aac;
    pub const QUEUED_GIF1: u32 = 0x5ab0;
    pub const QUEUED_GIF2: u32 = 0x5ab4;
    /// Per view, 0x20 bytes (`PS2GraphicsMan_RenderDepthFog`).
    pub const DEPTH_FOG: u32 = 0x6c64;
    /// Per view, 0x18 bytes: scale rgb, offset rgb (`RenderColorFilter`).
    pub const COLOR_FILTER: u32 = 0x6ca4;
    /// Per view, 0x1c bytes (`RenderBloom`).
    pub const BLOOM: u32 = 0x6cd4;
}

/// DeferMan offsets.
pub mod dm {
    pub const NUM_SORTED: u32 = 0x67ca4;
    pub const SORT_LIST: u32 = 0x67ca8;
    pub const EMIT_MATERIAL: u32 = 0x69cbc;
    pub const CUR_MATRIX: u32 = 0x69cc0;
    pub const NUM_MATRICES: u32 = 0x69cc8;
    pub const MATRICES: u32 = 0x69cd0;
    pub const MAX_MATRICES: u32 = 12;
    pub const NUM_SCISSORS: u32 = 0x6aed0;
    pub const SCISSORS: u32 = 0x6aed4;
}

// ---------------------------------------------------------------------------
// Memory access.

fn norm(a: u32) -> u32 {
    if a >> 28 == 7 {
        a
    } else {
        a & 0x1fff_ffff
    }
}

/// A 32-bit word of EE memory.
pub fn word(m: &dyn Mem, a: u32) -> u32 {
    let a = norm(a);
    (m.qword(a & !15) >> ((a & 12) * 8)) as u32
}

fn half(m: &dyn Mem, a: u32) -> i16 {
    (word(m, a & !3) >> ((a & 2) * 8)) as u16 as i16
}

fn f32_at(m: &dyn Mem, a: u32) -> f32 {
    f32::from_bits(word(m, a))
}

fn vec4(m: &dyn Mem, a: u32) -> [f32; 4] {
    [f32_at(m, a), f32_at(m, a + 4), f32_at(m, a + 8), f32_at(m, a + 12)]
}

fn mat4(m: &dyn Mem, a: u32) -> [[f32; 4]; 4] {
    [vec4(m, a), vec4(m, a + 16), vec4(m, a + 32), vec4(m, a + 48)]
}

// ---------------------------------------------------------------------------
// RenderState.

/// VU1 material type (RenderState w0 bits 6-9): selects the microprogram.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Material {
    /// No VU setup: DIRECT/PATH2 draws.
    None,
    /// f4babac5, Bezier terrain, uniform LOD.
    Bezier1,
    /// 9790369f, Bezier terrain, stitched LOD edges.
    Bezier2,
    /// d159716b, the general program.
    General,
    /// f55ee634, particles.
    Particle,
    /// e72540c9, SnowBox.
    Snow,
    /// 9fa47842 with the slot-124 lighting packet.
    Skinned,
    /// 9fa47842 with the slot-101 shadow header.
    Shadow,
    /// 8..15: no program (8 sorts like particles).
    Other(u8),
}

impl Material {
    pub fn from_type(t: u32) -> Self {
        match t {
            0 => Self::None,
            1 => Self::Bezier1,
            2 => Self::Bezier2,
            3 => Self::General,
            4 => Self::Particle,
            5 => Self::Snow,
            6 => Self::Skinned,
            7 => Self::Shadow,
            n => Self::Other(n as u8),
        }
    }
    pub fn type_id(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Bezier1 => 1,
            Self::Bezier2 => 2,
            Self::General => 3,
            Self::Particle => 4,
            Self::Snow => 5,
            Self::Skinned => 6,
            Self::Shadow => 7,
            Self::Other(n) => n as u32,
        }
    }
    /// Sort rank in the bucket key (`DeferMan_ComputeSortKey`).
    pub fn rank(self) -> u32 {
        match self.type_id() {
            1 => 4,
            2 => 5,
            3 => 3,
            4 | 8 => 1,
            5 => 6,
            6 | 7 => 2,
            _ => 0,
        }
    }
}

/// Texture wrap mode (CLAMP_1/2 from w0 bits 2-3 / 4-5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Wrap {
    Repeat,
    /// Clamp T, repeat S.
    ClampT,
    /// Clamp S, repeat T.
    ClampS,
    Clamp,
}

impl Wrap {
    pub fn from_bits(b: u32) -> Self {
        [Self::Repeat, Self::ClampT, Self::ClampS, Self::Clamp][(b & 3) as usize]
    }
    /// The CLAMP register value (`GsPkt_AddClamp`): WMS bits 0-1, WMT 2-3.
    pub fn clamp_reg(self) -> u64 {
        match self {
            Self::Repeat => 0,
            Self::ClampT => 0b0100,
            Self::ClampS => 0b0001,
            Self::Clamp => 0b0101,
        }
    }
}

/// Alpha test (w1 bits 20-21). Always on (ATE = 1) with AFAIL = FB_ONLY: a
/// failing pixel still writes colour, not Z.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlphaTest {
    Always,
    /// Pass if A < AREF.
    Less,
    /// Pass if A > AREF.
    Greater,
}

impl AlphaTest {
    pub fn from_sel(s: u32) -> Self {
        match s {
            2 => Self::Less,
            3 => Self::Greater,
            _ => Self::Always,
        }
    }
    /// GS ATST.
    pub fn atst(self) -> u64 {
        match self {
            Self::Always => 1,
            Self::Less => 2,
            Self::Greater => 6,
        }
    }
}

/// Depth test (w1 bits 23-24); larger Z is nearer. Always on (ZTE = 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DepthTest {
    Greater,
    GEqual,
    Always,
}

impl DepthTest {
    pub fn from_sel(s: u32) -> Self {
        match s {
            0 => Self::Greater,
            1 => Self::GEqual,
            _ => Self::Always,
        }
    }
    /// GS ZTST.
    pub fn ztst(self) -> u64 {
        match self {
            Self::Greater => 3,
            Self::GEqual => 2,
            Self::Always => 1,
        }
    }
}

/// Sort mode (w1 bits 0-1): the 10-bit sort value in the bucket key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sort {
    /// Mode 0: 0x3ff (drawn first within the layer).
    First,
    /// Mode 1: 0x3fe.
    Second,
    /// Mode 2: the explicit value from w2 bits 10-28.
    Explicit,
    /// Mode 3: 0.
    Last,
}

/// GS ALPHA blend `(A - B) * C + D` (A/B/D: 0 Cs, 1 Cd, 2 zero; C: 0 As,
/// 1 Ad, 2 FIX), with C = 0x80 meaning 1.0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Blend {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub fix: u8,
}

impl Blend {
    pub fn from_reg(v: u64) -> Self {
        Self { a: (v & 3) as u8, b: ((v >> 2) & 3) as u8, c: ((v >> 4) & 3) as u8, d: ((v >> 6) & 3) as u8, fix: (v >> 32) as u8 }
    }
}

/// The ALPHA register value of blend preset `p` (`GsPkt_AddAlpha`, jump
/// table 0x491fb0) with the state's FIX. `None` for presets 15 and 18-31:
/// the game reserves the A+D pair but writes no value (stale memory).
pub fn blend_preset(p: u32, fix: u8) -> Option<u64> {
    let fix = (fix as u64) << 32;
    Some(match p {
        0 => 0xaa,
        1 => 0x2a,
        2 => 0x80 << 32 | 0x68,
        3 => 0x6a,
        4 => 0x89,
        5 => 0x44,
        6 => 0x46,
        7 => 0x48,
        8 => 0x81,
        9 => 0x42,
        10 | 21 => 0x49,
        11 => fix | 0x64,
        12 => fix | 0x68,
        13 | 14 => fix | 0x88,
        16 => 0x98,
        17 => 0x58,
        _ => return None,
    })
}

/// The TEST register value (`GsPkt_AddTest`).
pub fn test_reg(zsel: u32, asel: u32, aref: u32) -> u64 {
    1 | AlphaTest::from_sel(asel).atst() << 1 | (aref as u64) << 4 | 1 << 12 | 1 << 16 | DepthTest::from_sel(zsel).ztst() << 17
}

/// The ZBUF register value (`GsPkt_AddZbuf`).
pub fn zbuf_reg(zbp: u32, zpsm: u32, zmsk: bool) -> u64 {
    zbp as u64 | (zpsm.wrapping_sub(0x30) as u64) << 24 | (zmsk as u64) << 32
}

/// The SCISSOR register value (`GsPkt_AddScissor`) for a table entry
/// `{x, y, w, h}`.
pub fn scissor_reg(r: [i16; 4]) -> u64 {
    let sx = |v: u32| v as i32 as i64 as u64;
    let (x, y, w, h) = (r[0] as i32 as u32, r[1] as i32 as u32, r[2] as i32 as u32, r[3] as i32 as u32);
    let x1 = sx(x.wrapping_sub(1).wrapping_add(w));
    let y1 = sx(y.wrapping_sub(1).wrapping_add(h));
    sx(x) | x1 << 16 | sx(y) << 32 | y1 << 48
}

/// A DeferMan RenderState (0x14 bytes), decoded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderState {
    /// CLAMP_1 / CLAMP_2.
    pub clamp: [Wrap; 2],
    pub material: Material,
    /// ALPHA_1 / ALPHA_2 FIX.
    pub fix: [u8; 2],
    pub sort: Sort,
    /// ALPHA_1 / ALPHA_2 blend preset (0..31, see [`blend_preset`]).
    pub blend: [u8; 2],
    pub aref: u8,
    pub atst: AlphaTest,
    /// No Z write.
    pub zmask: bool,
    pub ztst: DepthTest,
    /// Index into [`Frame::scissors`].
    pub scissor: u8,
    pub layer: u8,
    /// Explicit sort value (w2 bits 10-28; used by [`Sort::Explicit`]).
    pub sort_value: u32,
    /// w3 bits 0-29: EE address of the type-6 lighting packet or type-7
    /// shadow header packet (CALLed by the material setup), else 0.
    pub material_param: u32,
    /// Texture ids from the interned state (the bucket's own ids are in
    /// [`Draw::tex`]; the state copy is from the first record interned).
    pub tex: [i16; 2],
    pub raw: [u32; 5],
}

impl RenderState {
    pub fn decode(w: [u32; 5]) -> Self {
        Self {
            clamp: [Wrap::from_bits(w[0] >> 2), Wrap::from_bits(w[0] >> 4)],
            material: Material::from_type((w[0] >> 6) & 0xf),
            fix: [(w[0] >> 12) as u8, (w[0] >> 20) as u8],
            sort: [Sort::First, Sort::Second, Sort::Explicit, Sort::Last][(w[1] & 3) as usize],
            blend: [((w[1] >> 2) & 0x1f) as u8, ((w[1] >> 7) & 0x1f) as u8],
            aref: (w[1] >> 12) as u8,
            atst: AlphaTest::from_sel((w[1] >> 20) & 3),
            zmask: (w[1] >> 22) & 1 != 0,
            ztst: DepthTest::from_sel((w[1] >> 23) & 3),
            scissor: (w[2] & 0x1f) as u8,
            layer: ((w[2] >> 5) & 0x1f) as u8,
            sort_value: (w[2] >> 10) & 0x7ffff,
            material_param: w[3] & 0x3fff_ffff,
            tex: [w[4] as i16, (w[4] >> 16) as i16],
            raw: w,
        }
    }

    pub fn read(m: &dyn Mem, a: u32) -> Self {
        Self::decode([word(m, a), word(m, a + 4), word(m, a + 8), word(m, a + 12), word(m, a + 16)])
    }

    /// The 10-bit sort value of the bucket key.
    pub fn sort_key_value(&self) -> u32 {
        match self.sort {
            Sort::First => 0x3ff,
            Sort::Second => 0x3fe,
            Sort::Explicit => self.sort_value,
            Sort::Last => 0,
        }
    }

    /// `DeferMan_ComputeSortKey` for this state with bucket textures `tex`
    /// (the value stored in the sort list, already inverted).
    pub fn sort_key(&self, tex: [i16; 2]) -> u32 {
        let mut t = 0u32;
        if tex[0] >= 0 {
            t = tex[0] as u32;
        }
        if tex[1] >= 0 {
            t |= (tex[1] as u32 & 0xf) << 6;
        }
        !((t & 0x3ff) << 3 | (31 - self.layer as u32) << 26 | (self.sort_key_value() & 0x3ff) << 16 | self.material.rank() << 13)
    }

    /// ALPHA register for context `ctx` (`None`: stale, see [`blend_preset`]).
    pub fn alpha_reg(&self, ctx: usize) -> Option<u64> {
        blend_preset(self.blend[ctx] as u32, self.fix[ctx])
    }
    pub fn blend(&self, ctx: usize) -> Option<Blend> {
        self.alpha_reg(ctx).map(Blend::from_reg)
    }
    /// TEST register (both contexts get the same value from a state change).
    pub fn test_reg(&self) -> u64 {
        test_reg((self.raw[1] >> 23) & 3, (self.raw[1] >> 20) & 3, self.aref as u32)
    }
    pub fn zbuf_reg(&self, zbp: u32, zpsm: u32) -> u64 {
        zbuf_reg(zbp, zpsm, self.zmask)
    }
    pub fn clamp_reg(&self, ctx: usize) -> u64 {
        self.clamp[ctx].clamp_reg()
    }
}

// ---------------------------------------------------------------------------
// GS draw-state registers.

/// The state registers a draw runs with: SCISSOR, ZBUF, TEST, CLAMP, ALPHA
/// for both contexts. `None` = not known (never written in this frame, or
/// written with stale memory).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GsRegs(pub [Option<u64>; 10]);

impl GsRegs {
    pub const REGS: [u8; 10] = [
        reg::SCISSOR_1,
        reg::SCISSOR_2,
        reg::ZBUF_1,
        reg::ZBUF_2,
        reg::TEST_1,
        reg::TEST_2,
        reg::CLAMP_1,
        reg::CLAMP_2,
        reg::ALPHA_1,
        reg::ALPHA_2,
    ];
    pub fn slot(r: u8) -> Option<usize> {
        Self::REGS.iter().position(|&x| x == r)
    }
    pub fn set(&mut self, r: u8, v: Option<u64>) {
        if let Some(i) = Self::slot(r) {
            self.0[i] = v;
        }
    }
    pub fn get(&self, r: u8) -> Option<u64> {
        Self::slot(r).and_then(|i| self.0[i])
    }
}

pub fn reg_name(r: u8) -> &'static str {
    match r {
        reg::SCISSOR_1 => "SCISSOR_1",
        reg::SCISSOR_2 => "SCISSOR_2",
        reg::ZBUF_1 => "ZBUF_1",
        reg::ZBUF_2 => "ZBUF_2",
        reg::TEST_1 => "TEST_1",
        reg::TEST_2 => "TEST_2",
        reg::CLAMP_1 => "CLAMP_1",
        reg::CLAMP_2 => "CLAMP_2",
        reg::ALPHA_1 => "ALPHA_1",
        reg::ALPHA_2 => "ALPHA_2",
        reg::TEX0_1 => "TEX0_1",
        reg::TEX0_2 => "TEX0_2",
        _ => "?",
    }
}

/// Which material setups `DeferMan_EmitMaterialSetup` emitted before a draw.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialSetup {
    /// Mode 0: microprogram + full header (the type changed).
    pub program: bool,
    /// Mode 1: matrix header only (same type, other matrix).
    pub matrix: bool,
    /// Mode 2: `CALL w3` (types 6/7; the parameter changed).
    pub param: bool,
}

/// The game's state-change emission, simulated on decoded states
/// (`DeferMan_EmitStateChange` 0x363c20, `DeferMan_EmitDefaultState`
/// 0x364360). It predicts the GS registers and material setups without
/// reading the chain.
#[derive(Clone, Debug)]
pub struct Emitter {
    /// DeferMan +0x69ca8: the last emitted state.
    pub state: [u32; 5],
    /// +0x69cc4.
    pub scissor: i32,
    /// +0x69cc0: matrix of the last header upload.
    pub matrix: i32,
    /// +0x69cbc.
    pub material_param: u32,
    pub regs: GsRegs,
    pub zbp: u32,
    pub zpsm: u32,
}

impl Emitter {
    /// `DeferMan_EmitDefaultState`: 8 A+D writes from `d`. Context 2's ZBUF
    /// and TEST take their selectors from other bits (ZMSK bit 23, ZTST bits
    /// 25-26, ATST bits 22-23) than a state change does: a game quirk.
    pub fn default_state(&mut self, d: [u32; 5]) {
        self.state = d;
        let (w0, w1) = (d[0], d[1]);
        let aref = (w1 >> 12) & 0xff;
        self.regs.set(reg::CLAMP_1, Some(Wrap::from_bits(w0 >> 2).clamp_reg()));
        self.regs.set(reg::ALPHA_1, blend_preset((w1 >> 2) & 0x1f, (w0 >> 12) as u8));
        self.regs.set(reg::ZBUF_1, Some(zbuf_reg(self.zbp, self.zpsm, (w1 >> 22) & 1 == 1)));
        self.regs.set(reg::TEST_1, Some(test_reg((w1 >> 23) & 3, (w1 >> 20) & 3, aref)));
        self.regs.set(reg::CLAMP_2, Some(Wrap::from_bits(w0 >> 4).clamp_reg()));
        self.regs.set(reg::ALPHA_2, blend_preset((w1 >> 7) & 0x1f, (w0 >> 20) as u8));
        self.regs.set(reg::ZBUF_2, Some(zbuf_reg(self.zbp, self.zpsm, (w1 >> 23) & 1 == 1)));
        self.regs.set(reg::TEST_2, Some(test_reg((w1 >> 25) & 3, (w1 >> 22) & 3, aref)));
    }

    /// `DeferMan_EmitStateChange` for state `s` (words) and matrix index.
    pub fn state_change(&mut self, s: [u32; 5], matrix: i32, scissors: &[[i16; 4]]) -> MaterialSetup {
        let mut setup = MaterialSetup::default();
        let ty = (s[0] >> 6) & 0xf;
        if ty != 0 {
            let w3 = s[3] & 0x3fff_ffff;
            if (self.state[0] >> 6) & 0xf == ty {
                if matrix != self.matrix {
                    self.material_param = w3;
                    self.matrix = matrix;
                    setup.matrix = true;
                }
            } else {
                self.material_param = w3;
                self.matrix = matrix;
                setup.program = true;
            }
            if self.material_param != w3 {
                self.matrix = matrix;
                self.material_param = w3;
                setup.param = true;
            }
        }
        let sc = (s[2] & 0x1f) as i32;
        if sc != self.scissor {
            self.scissor = sc;
            let v = scissors.get(sc as usize).map(|r| scissor_reg(*r));
            self.regs.set(reg::SCISSOR_1, v);
            self.regs.set(reg::SCISSOR_2, v);
        }
        let (e0, e1) = (self.state[0], self.state[1]);
        let (w0, w1) = (s[0], s[1]);
        if e1 & 0x40_0000 != w1 & 0x40_0000 {
            let z = zbuf_reg(self.zbp, self.zpsm, w1 & 0x40_0000 != 0);
            self.regs.set(reg::ZBUF_1, Some(z));
            self.regs.set(reg::ZBUF_2, Some(z));
        }
        if e1 & 0x1bf_f000 != w1 & 0x1bf_f000 {
            let t = test_reg((w1 >> 23) & 3, (w1 >> 20) & 3, (w1 >> 12) & 0xff);
            self.regs.set(reg::TEST_1, Some(t));
            self.regs.set(reg::TEST_2, Some(t));
        }
        if e0 & 0xc != w0 & 0xc {
            self.regs.set(reg::CLAMP_1, Some(Wrap::from_bits(w0 >> 2).clamp_reg()));
        }
        if e1 & 0x7c != w1 & 0x7c || (e0 >> 12) & 0xff != (w0 >> 12) & 0xff {
            self.regs.set(reg::ALPHA_1, blend_preset((w1 >> 2) & 0x1f, (w0 >> 12) as u8));
        }
        if e0 & 0x30 != w0 & 0x30 {
            self.regs.set(reg::CLAMP_2, Some(Wrap::from_bits(w0 >> 4).clamp_reg()));
        }
        if e1 & 0xf80 != w1 & 0xf80 || (e0 >> 20) & 0xff != (w0 >> 20) & 0xff {
            self.regs.set(reg::ALPHA_2, blend_preset((w1 >> 7) & 0x1f, (w0 >> 20) as u8));
        }
        self.state = s;
        setup
    }
}

// ---------------------------------------------------------------------------
// The typed frame.

/// A MatrixRecord (0x180, DeferMan +0x69cd0): gm+0x5780.. at the time the
/// view matrices were rebuilt.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatrixRecord {
    pub world: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    /// view × world: VU1 header qw0-3 of types 1, 2, 3, 6.
    pub world_view: [[f32; 4]; 4],
    /// Header qw4/5 of types 1, 2, 3, 6.
    pub guard_scale: [f32; 4],
    pub guard_offset: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
    pub world_clip: [[f32; 4]; 4],
    pub screen_scale: [f32; 4],
    pub screen_offset: [f32; 4],
}

impl MatrixRecord {
    pub fn read(m: &dyn Mem, a: u32) -> Self {
        Self {
            world: mat4(m, a),
            view: mat4(m, a + 0x40),
            world_view: mat4(m, a + 0x80),
            guard_scale: vec4(m, a + 0xc0),
            guard_offset: vec4(m, a + 0xd0),
            view_proj: mat4(m, a + 0xe0),
            world_clip: mat4(m, a + 0x120),
            screen_scale: vec4(m, a + 0x160),
            screen_offset: vec4(m, a + 0x170),
        }
    }
    /// VU1 header qw0-5 (worldView rows, guard scale, guard offset) as the
    /// raw bit patterns UNPACK V4-32 writes.
    pub fn header_qwords(&self) -> [u128; 6] {
        let q = |v: [f32; 4]| v.iter().enumerate().fold(0u128, |acc, (i, f)| acc | (f.to_bits() as u128) << (32 * i));
        let wv = self.world_view;
        [q(wv[0]), q(wv[1]), q(wv[2]), q(wv[3]), q(self.guard_scale), q(self.guard_offset)]
    }
}

/// What a draw's packets contain, in chain order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Payload {
    /// A VU1 microprogram start with its input.
    Vu(VuCall),
    /// A GIF stream through DIRECT/DIRECTHL (PATH2).
    Gif(Vec<u128>),
    /// An MPG upload (microcode), address and size in bytes.
    Microcode { addr: u32, bytes: u32 },
    /// MSKPATH3.
    MaskPath3(bool),
}

impl From<Event> for Payload {
    fn from(e: Event) -> Self {
        match e {
            Event::Call(c) => Self::Vu(c),
            Event::Direct(d) => Self::Gif(d.qwords),
            Event::Microcode { addr, bytes } => Self::Microcode { addr, bytes },
            Event::MaskPath3(b) => Self::MaskPath3(b),
        }
    }
}

/// One bucket (a sorted bucket or one of its continuation buckets).
#[derive(Clone, Debug)]
pub struct Draw {
    /// Guest address of the Bucket (0x20).
    pub bucket: u32,
    /// Index in the sorted list; continuation buckets share their head's.
    pub sorted: u32,
    /// Reached through a predecessor's continuation link (+0x18).
    pub continuation: bool,
    /// Guest address of the interned RenderState.
    pub state_addr: u32,
    pub state: RenderState,
    /// TexMan ids for context 1 / 2 (bucket +4).
    pub tex: [Option<u16>; 2],
    /// Index into [`Frame::matrices`] (bucket +8).
    pub matrix: i16,
    /// Matrix of the VU1 header the draw actually runs with: the game only
    /// re-uploads the header when the *state pointer* changes, so a bucket
    /// that shares its predecessor's state keeps the older matrix.
    pub header_matrix: i32,
    /// Material setups emitted before this draw.
    pub setup: MaterialSetup,
    /// A state change was emitted before this draw (state pointer changed).
    pub state_change: bool,
    /// Predicted GS state registers at the draw.
    pub gs: GsRegs,
    /// Packet range: first start (CALL target) .. last end (RET position).
    pub packets: Range<u32>,
    /// Chain contents emitted for this draw before its CALL: state change
    /// (A+D, material setup, lighting/shadow packet) and texture bind
    /// (TEX0/TEX1/MIPTBP, PATH3 unmask).
    pub prelude: Vec<Payload>,
    /// The bucket's packets: VU calls and GIF streams.
    pub payload: Vec<Payload>,
}

/// One step of the frame, in emission order.
#[derive(Clone, Debug)]
pub enum Pass {
    /// Consecutive draws.
    Draws(Vec<Draw>),
    /// `DeferMan_EmitDefaultState` (chain head, and after each post group).
    DefaultState(Vec<Payload>),
    /// Per view, after the layer < 6 buckets.
    DepthFog { view: u8, params: [u32; 8] },
    /// END of the phase-1 chain; the phase-2 callback (LensFX) runs here.
    PhaseBoundary,
    /// Per view, at the start of phase 2.
    Bloom { view: u8, params: [u32; 7] },
    ColorFilter { view: u8, scale: [f32; 3], offset: [f32; 3] },
    /// The chain contents of the post markers just before (GIF/VIF as
    /// emitted; the post passes themselves go through PATH3 in GifBuff).
    PostChain(Vec<Payload>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisplayState {
    pub fb_width: u32,
    pub fb_height: u32,
    pub draw_fbp: u32,
    pub zbp: u32,
    pub zpsm: u32,
    pub blit: bool,
    pub brightness_rgba: [i32; 4],
    pub brightness: f32,
    pub screen_offset: [f32; 2],
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    /// gm+0x5930 + 0x40·i: clip → GS 12.4 screen matrix.
    pub screen: [[f32; 4]; 4],
}

/// One frame as the renderer sees it.
#[derive(Clone, Debug)]
pub struct Frame {
    pub gm: u32,
    pub defer: u32,
    /// Published chain heads: VIF1 phase 1/2, GIF phase 1/2.
    pub vif_heads: [u32; 2],
    pub gif_heads: [u32; 2],
    pub views: Vec<View>,
    pub matrices: Vec<MatrixRecord>,
    pub scissors: Vec<[i16; 4]>,
    pub passes: Vec<Pass>,
    pub display: DisplayState,
}

impl Frame {
    pub fn draws(&self) -> impl Iterator<Item = &Draw> {
        self.passes.iter().flat_map(|p| match p {
            Pass::Draws(d) => d.as_slice(),
            _ => &[],
        })
    }
}

// ---------------------------------------------------------------------------
// Builder.

/// Who an event of the chain belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    /// Draw `n`'s packets.
    Payload(usize),
    /// Draw `n`'s prelude.
    Prelude(usize),
    /// A default-state segment.
    DefaultState,
    /// A post-effect segment.
    Post,
    /// Chain contents where the bucket structure predicts none.
    Unattributed,
}

/// What the chain wrote before a draw's CALL.
#[derive(Clone, Debug, Default)]
pub struct ChainObs {
    /// Address of the top-level CALL tag (0: the draw was not called).
    pub call_tag: u32,
    /// The RET that ended the payload walk sits at the bucket's last end.
    pub ret_at_end: bool,
    /// State registers as written by non-payload chain segments.
    pub emitted: GsRegs,
    /// The same including writes from earlier draws' payloads.
    pub actual: GsRegs,
    /// TEX0_1/2 (with TEX2 merged) as written by non-payload segments.
    pub tex0: [Option<u64>; 2],
    /// FLUSHE state-change marks in the region before the CALL.
    pub flushe_marks: u32,
}

/// [`build`]'s result: the frame plus what verification needs.
pub struct Built {
    pub frame: Frame,
    /// Every chain event in order, with its owner.
    pub events: Vec<(Owner, Event)>,
    /// Per draw (emission order), what the chain did before its CALL.
    pub obs: Vec<ChainObs>,
    /// Structural problems found while matching chain and buckets.
    pub issues: Vec<String>,
    /// VIF registers after the frame (seed for the next one).
    pub regs_end: VifRegs,
    /// The emitter after the frame (+0x69cbc/+0x69cc0 must match memory).
    pub emitter: Emitter,
}

/// Inputs carried from frame to frame.
#[derive(Clone, Debug, Default)]
pub struct Carry {
    pub regs: VifRegs,
    /// DeferMan +0x69cc0 / +0x69cbc at the start of the frame (not reset by
    /// the game).
    pub matrix: i32,
    pub material_param: u32,
}

struct Plan {
    /// Draw skeletons in emission order (payload/prelude empty).
    draws: Vec<Draw>,
    /// Draw index ranges of the three layer ranges.
    ranges: [Range<usize>; 3],
    thresholds: [u32; 2],
    num_sorted: u32,
}

fn tex_id(t: i16) -> Option<u16> {
    (t >= 0).then_some(t as u16)
}

/// Read the DeferMan bucket list and simulate the state emission.
fn plan(m: &dyn Mem, g: u32, defer: u32, carry: &Carry, scissors: &[[i16; 4]], issues: &mut Vec<String>) -> (Plan, Emitter) {
    let num_sorted = word(m, defer + dm::NUM_SORTED).min(2048);
    let thresholds = [word(m, addr::LAYER_THRESHOLDS), word(m, addr::LAYER_THRESHOLDS + 4)];
    let bucket_of = |i: u32| word(m, defer + dm::SORT_LIST + i * 8 + 4);
    let layer_of = |i: u32| {
        let b = bucket_of(i);
        (word(m, word(m, b) + 8) >> 5) & 0x1f
    };
    // The game's split search (RenderFrame): the first bucket of a range is
    // never tested, so a range always keeps at least one bucket.
    let mut split = [0u32; 2];
    for k in 0..2 {
        let mut lo = if k == 0 { 0 } else { split[k - 1] };
        let n = num_sorted;
        let mut hi = n;
        if n.saturating_sub(lo) > 1 {
            loop {
                let mid = (lo + hi) >> 1;
                if layer_of(mid) < thresholds[k] {
                    lo = mid;
                } else {
                    hi = mid;
                }
                if hi - lo <= 1 {
                    break;
                }
            }
        }
        split[k] = hi;
    }
    let default = [0u32, 4, 8, 12, 16].map(|o| word(m, addr::DEFAULT_STATE + o));
    let mut em = Emitter {
        state: default,
        scissor: word(m, defer + 0x69cc4) as i32,
        matrix: carry.matrix,
        material_param: carry.material_param,
        regs: GsRegs::default(),
        zbp: word(m, g + gm::ZBP),
        zpsm: word(m, g + gm::ZPSM),
    };
    em.default_state(default);
    em.scissor = -1;
    let mut draws = Vec::new();
    let mut ranges = [0..0, 0..0, 0..0];
    let mut last_state = 0u32;
    let bounds = [0, split[0], split[1], num_sorted];
    for k in 0..3 {
        let start = draws.len();
        for i in bounds[k]..bounds[k + 1].max(bounds[k]) {
            let mut b = bucket_of(i);
            let mut cont = false;
            let mut guard = 0;
            while b != 0 && guard < 4096 {
                guard += 1;
                let state_addr = word(m, b);
                let state = RenderState::read(m, state_addr);
                let texw = word(m, b + 4);
                let matrix = word(m, b + 8) as i32;
                let mut setup = MaterialSetup::default();
                let changed = state_addr != last_state;
                if changed {
                    setup = em.state_change(state.raw, matrix, scissors);
                    last_state = state_addr;
                }
                if !cont {
                    let key = word(m, defer + dm::SORT_LIST + i * 8);
                    let want = state.sort_key([texw as i16, (texw >> 16) as i16]);
                    if key != want {
                        issues.push(format!("sorted {i}: key {key:#x}, state predicts {want:#x}"));
                    }
                }
                draws.push(Draw {
                    bucket: b,
                    sorted: i,
                    continuation: cont,
                    state_addr,
                    state,
                    tex: [tex_id(texw as i16), tex_id((texw >> 16) as i16)],
                    matrix: matrix as i16,
                    header_matrix: em.matrix,
                    setup,
                    state_change: changed,
                    gs: em.regs,
                    packets: word(m, b + 0xc)..word(m, b + 0x10),
                    prelude: Vec::new(),
                    payload: Vec::new(),
                });
                cont = true;
                b = word(m, b + 0x18);
            }
        }
        ranges[k] = start..draws.len();
        if k < 2 && num_sorted > 0 && (thresholds[k] == 6 || thresholds[k] == 9) {
            em.scissor = -1;
            em.default_state(default);
            last_state = 0;
        }
    }
    (Plan { draws, ranges, thresholds, num_sorted }, em)
}

fn is_flushe_cnt(tag: u128, first: u128) -> bool {
    tag == 0x1000_0001 && first == 0x1000_0000u128 << 96
}

/// TEX2 into a tracked TEX0: PSM, CBP, CPSM, CSM, CSA, CLD.
fn merge_tex2(tex0: Option<u64>, tex2: u64) -> Option<u64> {
    const MASK: u64 = 0x3f << 20 | 0x3fff << 37 | 0xf << 51 | 1 << 55 | 0x1f << 56 | 7 << 61;
    Some((tex0.unwrap_or(0) & !MASK) | (tex2 & MASK))
}

struct Walker<'a> {
    m: &'a dyn Mem,
    vif: Vif1,
    gif: GifReader,
    events: Vec<(Owner, Event)>,
    /// Owner for events while inside a payload.
    in_payload: Option<usize>,
    emitted: GsRegs,
    actual: GsRegs,
    tex0: [Option<u64>; 2],
    flushe: Vec<usize>,
}

impl Walker<'_> {
    fn feed(&mut self, data: &[u128]) {
        if data.is_empty() {
            return;
        }
        self.vif.write(data);
        let owner = match self.in_payload {
            Some(d) => Owner::Payload(d),
            None => Owner::Unattributed,
        };
        for e in std::mem::take(&mut self.vif.events) {
            if let Event::Direct(d) = &e {
                let (emitted, actual, tex0) = (&mut self.emitted, &mut self.actual, &mut self.tex0);
                let payload = self.in_payload.is_some();
                self.gif.feed(&d.qwords, &mut |r, v| {
                    actual.set(r, Some(v));
                    if !payload {
                        emitted.set(r, Some(v));
                        match r {
                            reg::TEX0_1 => tex0[0] = Some(v),
                            reg::TEX0_2 => tex0[1] = Some(v),
                            reg::TEX2_1 => tex0[0] = merge_tex2(tex0[0], v),
                            reg::TEX2_2 => tex0[1] = merge_tex2(tex0[1], v),
                            _ => {}
                        }
                    }
                });
            }
            self.events.push((owner, e));
        }
    }
}

/// Region of the chain between draws: events `start..` up to the next CALL.
struct Region {
    phase: u8,
    /// The draw whose CALL ends the region; `None` at a phase end.
    before: Option<usize>,
    events: Range<usize>,
    /// Event indices of top-level `CNT 1 {FLUSHE}` tags in the region.
    flushe: Vec<usize>,
}

/// Build the frame. `g` = PS2GraphicsMan, called when EndFrame publishes the
/// chains (after `RenderFrame`, before `DeferMan_ResetFrame`).
pub fn build(m: &dyn Mem, g: u32, carry: &Carry) -> Built {
    let defer = word(m, g + gm::DEFER);
    let mut issues = Vec::new();
    let nm = word(m, defer + dm::NUM_MATRICES).min(dm::MAX_MATRICES);
    let matrices: Vec<_> = (0..nm).map(|i| MatrixRecord::read(m, defer + dm::MATRICES + i * 0x180)).collect();
    let ns = word(m, defer + dm::NUM_SCISSORS).min(32);
    let scissors: Vec<[i16; 4]> = (0..32.max(ns))
        .take(32)
        .map(|i| {
            let a = defer + dm::SCISSORS + i * 8;
            [half(m, a), half(m, a + 2), half(m, a + 4), half(m, a + 6)]
        })
        .collect();
    let (plan, emitter) = plan(m, g, defer, carry, &scissors, &mut issues);
    let num_views = word(m, g + gm::NUM_VIEWS).min(4);
    let vif_heads = [word(m, g + gm::QUEUED_VIF1), word(m, g + gm::QUEUED_VIF2)];
    let gif_heads = [word(m, g + gm::QUEUED_GIF1), word(m, g + gm::QUEUED_GIF2)];

    // Walk both phases.
    let starts: HashMap<u32, usize> = plan.draws.iter().enumerate().filter(|(_, d)| d.packets.start != 0).map(|(i, d)| (d.packets.start, i)).collect();
    let called: Vec<usize> = plan.draws.iter().enumerate().filter(|(_, d)| d.packets.start != 0).map(|(i, _)| i).collect();
    let mut next_called = 0usize;
    let mut w = Walker {
        m,
        vif: Vif1::new(carry.regs),
        gif: GifReader::default(),
        events: Vec::new(),
        in_payload: None,
        emitted: GsRegs::default(),
        actual: GsRegs::default(),
        tex0: [None; 2],
        flushe: Vec::new(),
    };
    let mut obs = vec![ChainObs::default(); plan.draws.len()];
    let mut payload_ranges: Vec<Option<Range<usize>>> = vec![None; plan.draws.len()];
    let mut regions: Vec<Region> = Vec::new();
    for (phase, &head) in vif_heads.iter().enumerate() {
        let mut region_start = w.events.len();
        w.flushe.clear();
        let mut tadr = head;
        let mut asp = 0u32;
        let mut asr = [0u32; 2];
        let mut payload_start = 0usize;
        let mut ended = false;
        for _ in 0..1_000_000 {
            let tag = w.m.qword(tadr);
            let qwc = (tag & 0xffff) as u32;
            let id = ((tag >> 28) & 7) as u32;
            let a = ((tag >> 32) & 0x7fff_fff0) as u32;
            let spr = |x: u32| if (tag >> 63) & 1 != 0 { x | 0x8000_0000 } else { x };
            let top = asp == 0;
            let (data, mut next, end) = match id {
                0 => (spr(a), tadr + 16, true),
                1 => (tadr + 16, tadr + 16 + qwc * 16, false),
                2 => (tadr + 16, a, false),
                3 | 4 => (spr(a), tadr + 16, false),
                5 => (tadr + 16, a, false),
                6 => (tadr + 16, tadr, asp == 0),
                _ => (tadr + 16, tadr, true),
            };
            let q: Vec<u128> = (0..qwc).map(|i| w.m.qword(data + i * 16)).collect();
            if top && w.in_payload.is_none() && qwc == 1 && is_flushe_cnt(tag, q[0]) {
                w.flushe.push(w.events.len());
            }
            // A top-level CALL into a bucket's first packet starts its payload.
            let bucket_call = if top && id == 5 { starts.get(&a).copied() } else { None };
            w.feed(&q);
            if let Some(d) = bucket_call {
                match called.get(next_called) {
                    Some(&want) if want == d => {}
                    want => issues.push(format!("CALL {a:#x} at {tadr:#x} is draw {d}, expected {want:?}")),
                }
                next_called = called.iter().position(|&x| x == d).map_or(next_called, |p| p + 1);
                regions.push(Region { phase: phase as u8, before: Some(d), events: region_start..w.events.len(), flushe: std::mem::take(&mut w.flushe) });
                obs[d] = ChainObs {
                    call_tag: tadr,
                    ret_at_end: false,
                    emitted: w.emitted,
                    actual: w.actual,
                    tex0: w.tex0,
                    flushe_marks: regions.last().unwrap().flushe.len() as u32,
                };
                w.in_payload = Some(d);
                payload_start = w.events.len();
            }
            // Other top-level CALLs (material setup: .vutext program chains,
            // w3 packets) stay in the current region.
            if id == 5 {
                if asp < 2 {
                    asr[asp as usize] = tadr + 16 + qwc * 16;
                    asp += 1;
                }
            } else if id == 6 && asp > 0 {
                asp -= 1;
                next = asr[asp as usize];
                if asp == 0 {
                    if let Some(d) = w.in_payload.take() {
                        // Flush VIF codes still pending in the payload.
                        obs[d].ret_at_end = tadr == plan.draws[d].packets.end;
                        payload_ranges[d] = Some(payload_start..w.events.len());
                        region_start = w.events.len();
                    }
                }
            }
            if id == 0 {
                next = tadr + 16;
            }
            tadr = next;
            if end {
                ended = true;
                break;
            }
        }
        if !ended {
            issues.push(format!("phase {} chain at {head:#x} did not end", phase + 1));
        }
        if let Some(d) = w.in_payload.take() {
            issues.push(format!("phase {} ended inside draw {d}'s payload", phase + 1));
            payload_ranges[d] = Some(payload_start..w.events.len());
        }
        regions.push(Region { phase: phase as u8, before: None, events: region_start..w.events.len(), flushe: std::mem::take(&mut w.flushe) });
    }
    for (i, d) in plan.draws.iter().enumerate() {
        if d.packets.start != 0 && obs[i].call_tag == 0 {
            issues.push(format!("draw {i} (bucket {:#x}, start {:#x}) never CALLed", d.bucket, d.packets.start));
        }
    }

    // Owners, prelude and pass assembly.
    let mut events = w.events;
    let set_owner = |events: &mut Vec<(Owner, Event)>, r: Range<usize>, o: Owner| {
        for e in &mut events[r] {
            e.0 = o;
        }
    };
    let mut draws = plan.draws;
    for (d, r) in payload_ranges.iter().enumerate() {
        if let Some(r) = r {
            draws[d].payload = events[r.clone()].iter().map(|(_, e)| e.clone().into()).collect();
        }
    }
    let to_payload = |events: &[(Owner, Event)], r: Range<usize>| -> Vec<Payload> { events[r].iter().map(|(_, e)| e.clone().into()).collect() };
    let last_r0 = plan.ranges[0].end.checked_sub(1).filter(|_| !plan.ranges[0].is_empty());
    let after_draw_of_region = {
        // The draw whose payload precedes each region (None for a phase head).
        let mut prev: Option<usize> = None;
        let mut v = Vec::new();
        for r in &regions {
            v.push(prev);
            prev = r.before;
            if r.before.is_none() {
                prev = None;
            }
        }
        v
    };
    let mut passes: Vec<Pass> = Vec::new();
    let mut cur: Vec<Draw> = Vec::new();
    let mut fog_done = false;
    let views = |base: u32, stride: u32, n: u32| -> Vec<u32> { (0..n).map(|v| g + base + v * stride).collect() };
    let mut draw_slots: Vec<Option<Draw>> = draws.into_iter().map(Some).collect();
    let mut emitted_draws = 0usize;
    let flush_draws = |cur: &mut Vec<Draw>, passes: &mut Vec<Pass>| {
        if !cur.is_empty() {
            passes.push(Pass::Draws(std::mem::take(cur)));
        }
    };
    let mut phase_seen = 0u8;
    for (ri, r) in regions.iter().enumerate() {
        let mut expect_post = false;
        let mut expect_default = ri == 0;
        if r.phase == 1 && phase_seen == 0 {
            phase_seen = 1;
            flush_draws(&mut cur, &mut passes);
            if !fog_done && plan.num_sorted > 0 {
                // Range 1 empty: the depth fog is in phase 1's tail, which
                // the previous region already handled.
                fog_done = true;
            }
            passes.push(Pass::PhaseBoundary);
            if plan.num_sorted > 0 && plan.thresholds[1] == 9 {
                for (v, a) in views(gm::BLOOM, 0x1c, num_views).into_iter().enumerate() {
                    passes.push(Pass::Bloom { view: v as u8, params: core::array::from_fn(|k| word(m, a + 4 * k as u32)) });
                }
                for (v, a) in views(gm::COLOR_FILTER, 0x18, num_views).into_iter().enumerate() {
                    let f = |k: u32| f32_at(m, a + 4 * k);
                    passes.push(Pass::ColorFilter { view: v as u8, scale: [f(0), f(1), f(2)], offset: [f(3), f(4), f(5)] });
                }
                expect_post = true;
                expect_default = true;
            }
        }
        if !fog_done && last_r0.is_some() && after_draw_of_region[ri] == last_r0 && plan.thresholds[0] == 6 {
            fog_done = true;
            flush_draws(&mut cur, &mut passes);
            for (v, a) in views(gm::DEPTH_FOG, 0x20, num_views).into_iter().enumerate() {
                passes.push(Pass::DepthFog { view: v as u8, params: core::array::from_fn(|k| word(m, a + 4 * k as u32)) });
            }
            expect_post = true;
            expect_default = true;
        }
        // Split: [post][default state][prelude of the next draw].
        let mut fl = r.flushe.clone();
        let prelude_start = match r.before {
            Some(d) if draw_slots[d].as_ref().is_some_and(|x| x.state_change) => fl.pop().unwrap_or_else(|| {
                issues.push(format!("region {ri}: draw {d} predicts a state change, chain has no FLUSHE block"));
                r.events.start
            }),
            Some(_) => r.events.start,
            None => r.events.end,
        };
        let default_start = if expect_default { fl.pop().unwrap_or(prelude_start) } else { prelude_start };
        if !fl.is_empty() {
            issues.push(format!("region {ri}: {} extra FLUSHE state blocks", fl.len()));
        }
        let post = r.events.start..default_start;
        let def = default_start..prelude_start;
        let pre = prelude_start..r.events.end;
        if !post.is_empty() {
            if expect_post {
                set_owner(&mut events, post.clone(), Owner::Post);
                passes.push(Pass::PostChain(to_payload(&events, post.clone())));
            } else {
                // Nothing predicted here: keep it visible as post chain but
                // mark it unattributed.
                flush_draws(&mut cur, &mut passes);
                passes.push(Pass::PostChain(to_payload(&events, post.clone())));
            }
        }
        if !def.is_empty() {
            set_owner(&mut events, def.clone(), Owner::DefaultState);
            flush_draws(&mut cur, &mut passes);
            passes.push(Pass::DefaultState(to_payload(&events, def.clone())));
        }
        if let Some(d) = r.before {
            set_owner(&mut events, pre.clone(), Owner::Prelude(d));
            // Uncalled draws (no packets) in emission order before this one.
            while emitted_draws < d {
                if let Some(x) = draw_slots[emitted_draws].take() {
                    cur.push(x);
                }
                emitted_draws += 1;
            }
            if let Some(mut x) = draw_slots[d].take() {
                x.prelude = to_payload(&events, pre.clone());
                cur.push(x);
            } else {
                issues.push(format!("region {ri}: draw {d} called twice"));
            }
            if let Some(p) = &payload_ranges[d] {
                set_owner(&mut events, p.clone(), Owner::Payload(d));
            }
            emitted_draws = emitted_draws.max(d + 1);
        }
    }
    for x in draw_slots.into_iter().flatten() {
        cur.push(x);
    }
    flush_draws(&mut cur, &mut passes);

    let display = DisplayState {
        fb_width: word(m, g + gm::FB_WIDTH),
        fb_height: word(m, g + gm::FB_HEIGHT),
        draw_fbp: word(m, g + gm::DRAW_FBP),
        zbp: word(m, g + gm::ZBP),
        zpsm: word(m, g + gm::ZPSM),
        blit: word(m, g + gm::BLIT) != 0,
        brightness_rgba: core::array::from_fn(|k| word(m, g + gm::BRIGHTNESS_RGBA + 4 * k as u32) as i32),
        brightness: f32_at(m, g + gm::BRIGHTNESS),
        screen_offset: [f32_at(m, g + gm::SCREEN_OFFSET), f32_at(m, g + gm::SCREEN_OFFSET + 4)],
    };
    let views = (0..num_views).map(|v| View { screen: mat4(m, g + gm::VIEW_MATRICES + v * 0x40) }).collect();
    Built {
        frame: Frame { gm: g, defer, vif_heads, gif_heads, views, matrices, scissors: scissors[..ns.max(1) as usize].to_vec(), passes, display },
        events,
        obs,
        issues,
        regs_end: w.vif.regs,
        emitter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_decodes() {
        let s = RenderState::decode([0, 0xb0_0294, 0x80, 0, 0xffff_ffff]);
        assert_eq!(s.material, Material::None);
        assert_eq!(s.blend, [5, 5]);
        assert_eq!(s.alpha_reg(0), Some(0x44));
        assert_eq!(s.blend(0), Some(Blend { a: 0, b: 1, c: 0, d: 1, fix: 0 }));
        assert_eq!(s.atst, AlphaTest::Greater);
        assert_eq!(s.aref, 0);
        assert!(!s.zmask);
        assert_eq!(s.ztst, DepthTest::GEqual);
        assert_eq!(s.layer, 4);
        assert_eq!(s.scissor, 0);
        assert_eq!(s.sort, Sort::First);
        assert_eq!(s.tex, [-1, -1]);
        // ATE, ATST GREATER (6), AFAIL FB_ONLY, ZTE, ZTST GEQUAL (2).
        assert_eq!(s.test_reg(), 1 | 6 << 1 | 1 << 12 | 1 << 16 | 2 << 17);
    }

    #[test]
    fn live_record_decodes() {
        // Record 0 of the frame-5000 dump: texture 0x119 in ctx 1.
        let s = RenderState::decode([0xcc, 0xc1_4184, 0, 0, 0xffff_0119]);
        assert_eq!(s.material, Material::General);
        assert_eq!(s.clamp, [Wrap::Clamp, Wrap::Repeat]);
        assert_eq!(s.blend[0], 1);
        assert_eq!(s.blend[1], 3);
        assert_eq!(s.aref, 0x14);
        assert_eq!(s.atst, AlphaTest::Always);
        assert!(s.zmask);
        assert_eq!(s.ztst, DepthTest::GEqual);
        assert_eq!(s.tex, [0x119, -1]);
    }

    #[test]
    fn registers_match_the_game_helpers() {
        assert_eq!(Wrap::from_bits(1).clamp_reg(), 4);
        assert_eq!(Wrap::from_bits(2).clamp_reg(), 1);
        assert_eq!(Wrap::from_bits(3).clamp_reg(), 5);
        assert_eq!(zbuf_reg(0xe0, 0x31, true), 0xe0 | 1 << 24 | 1 << 32);
        assert_eq!(scissor_reg([0, 0, 512, 448]), 511 << 16 | 447u64 << 48);
        assert_eq!(blend_preset(11, 0x40), Some(0x40 << 32 | 0x64));
        assert_eq!(blend_preset(2, 0x10), Some(0x80 << 32 | 0x68));
        assert_eq!(blend_preset(15, 0), None);
        assert_eq!(blend_preset(18, 0), None);
        assert_eq!(test_reg(2, 2, 0x80), 1 | 2 << 1 | 0x80 << 4 | 1 << 12 | 1 << 16 | 1 << 17);
        assert_eq!(test_reg(0, 0, 0), 1 | 1 << 1 | 1 << 12 | 1 << 16 | 3 << 17);
    }

    #[test]
    fn sort_key_orders_layer_then_sort_then_rank_then_texture() {
        let a = RenderState::decode([3 << 6, 0, 2 << 5, 0, 0]);
        let b = RenderState::decode([3 << 6, 0, 3 << 5, 0, 0]);
        // Ascending sort on the stored (inverted) key: lower layer first.
        assert!(a.sort_key([-1, -1]) < b.sort_key([-1, -1]));
        let p = RenderState::decode([4 << 6, 2, 7 << 5 | 5 << 10, 0, 0]);
        assert_eq!(p.sort_key_value(), 5);
        assert_eq!(!p.sort_key([3, 0x12]), (31 - 7) << 26 | 5 << 16 | 1 << 13 | (3 | 2 << 6) << 3);
    }

    #[test]
    fn emitter_emits_only_differences_and_quirky_default() {
        let mut e = Emitter { state: [0; 5], scissor: -1, matrix: -1, material_param: 0, regs: GsRegs::default(), zbp: 0xe0, zpsm: 0x31 };
        let d = [0, 0xb0_0294, 0x80, 0, 0xffff_ffff];
        e.default_state(d);
        // Context 2 takes ZMSK from bit 23 and ZTST/ATST from bits 25/22.
        assert_eq!(e.regs.get(reg::ZBUF_2), Some(zbuf_reg(0xe0, 0x31, true)));
        assert_eq!(e.regs.get(reg::TEST_2), Some(test_reg(0, 2, 0)));
        assert_eq!(e.regs.get(reg::TEST_1), Some(test_reg(1, 3, 0)));
        let sc = [[0i16, 0, 512, 448]];
        let mut s = d;
        s[0] |= 3 << 6;
        let setup = e.state_change(s, 2, &sc);
        assert_eq!(setup, MaterialSetup { program: true, matrix: false, param: false });
        assert_eq!(e.regs.get(reg::SCISSOR_1), Some(scissor_reg(sc[0])));
        // Unchanged TEST bits: TEST_2 keeps the default-state value.
        assert_eq!(e.regs.get(reg::TEST_2), Some(test_reg(0, 2, 0)));
        // Same type, other matrix: header only.
        assert_eq!(e.state_change(s, 3, &sc), MaterialSetup { program: false, matrix: true, param: false });
        // Type 6 with a new parameter: program, then param on change.
        let mut t = s;
        t[0] = 6 << 6;
        t[3] = 0x6241c0;
        assert_eq!(e.state_change(t, 3, &sc), MaterialSetup { program: true, matrix: false, param: false });
        t[3] = 0x6241e0;
        assert_eq!(e.state_change(t, 3, &sc), MaterialSetup { program: false, matrix: false, param: true });
    }

    #[test]
    fn tex2_merges_clut_fields() {
        let tex0 = 0x2000_0005_dd40_8000u64 | 0x1234;
        let tex2 = (0x13u64 << 20) | (0x2a0 << 37);
        let m = merge_tex2(Some(tex0), tex2).unwrap();
        assert_eq!(m & 0x3fff, 0x1234);
        assert_eq!((m >> 37) & 0x3fff, 0x2a0);
        assert_eq!((m >> 20) & 0x3f, 0x13);
    }
}
