//! Work package A4: render packets and small graphics state
//! (`docs/investigation/phase2-candidates.md`, layouts in
//! `docs/investigation/structs/render.md`).
//!
//! The renderer builds its GS register writes as A+D pairs (a 64-bit value
//! followed by a 64-bit register id) inside VIF DIRECT blocks. The
//! `GsPkt_Add*` helpers append one or two such pairs to a [`GsPacket`]
//! cursor; `TexMan_BuildUploadPacket` / `TexMan_BuildSpritePacket` build
//! complete DMA packets (DMA tag, VIF code, GIF tag, data) at a `u64*`
//! cursor; `DeferMan_AppendPacket` chains a packet into a draw bucket. The
//! rest are small pieces of frame state (frame-buffer ownership, the lens
//! flare source lists, the sky box, the scratchpad stream ring index, two
//! float timers).
//!
//! Every replacement reproduces the original's observable state exactly:
//! memory writes in the same order and widths (64-bit values are OR-ed
//! unmasked, as the game does, so out-of-range inputs bleed into
//! neighbouring fields the same way), the registers a caller or the
//! differential test can see (v0/v1, s0/s1 and sp, f0/f1, the FPU condition
//! bit, LO/HI after `mult`/`div`), and the EE cycle charge of each executed
//! block. None of these functions checks events or calls other functions.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_25c0, gs_pkt_add_clamp, "GsPkt_AddClamp"),
    (0x0036_23d8, gs_pkt_add_scissor, "GsPkt_AddScissor"),
    (0x0036_26d8, gs_pkt_add_test, "GsPkt_AddTest"),
    (0x0036_2660, gs_pkt_add_zbuf, "GsPkt_AddZbuf"),
    (
        0x0036_7db8,
        tex_man_build_upload_packet,
        "TexMan_BuildUploadPacket",
    ),
    (
        0x0036_7f18,
        tex_man_build_sprite_packet,
        "TexMan_BuildSpritePacket",
    ),
    (
        0x0036_2978,
        defer_man_append_packet,
        "DeferMan_AppendPacket",
    ),
    (
        0x0038_26e0,
        gm_acquire_frame_buffer,
        "PS2GraphicsMan_AcquireFrameBuffer",
    ),
    (
        0x0038_2730,
        gm_is_render_idle,
        "PS2GraphicsMan_IsRenderIdle",
    ),
    (0x002e_2fa8, lens_fx_add_source, "LensFX_AddSource"),
    (0x002e_2f98, lens_fx_clear_sources, "LensFX_ClearSources"),
    (0x0035_3d98, sky_box_clear, "SkyBox_Clear"),
    (0x0038_f6a8, spr_stream_next_index, "SprStream_NextIndex"),
    (0x0011_6fb8, hud_fx_update, "cHUDFX_Update"),
    (0x0023_1d18, app_module_update_fade, "AppModule_UpdateFade"),
    (0x003f_daa0, sce_gs_get_gparam, "sceGsGetGParam"),
];

// ---------------------------------------------------------------------------
// GS register ids and packet constants.

/// GS register ids (the "A" half of an A+D pair).
pub mod gs_reg {
    pub const CLAMP_1: u64 = 0x08;
    pub const CLAMP_2: u64 = 0x09;
    pub const XYOFFSET_1: u64 = 0x18;
    pub const SCISSOR_1: u64 = 0x40;
    pub const SCISSOR_2: u64 = 0x41;
    pub const TEST_1: u64 = 0x47;
    pub const TEST_2: u64 = 0x48;
    pub const FRAME_1: u64 = 0x4c;
    pub const ZBUF_1: u64 = 0x4e;
    pub const ZBUF_2: u64 = 0x4f;
}

/// The GS context a helper's selector picks: 1 means context 2, anything
/// else context 1 (compared as a full 64-bit register).
#[inline]
fn is_context_2(selector: u64) -> bool {
    selector == 1
}

/// `1 << n` as the game computes it (`sllv` of 1, 32-bit, sign-extended).
#[inline]
fn pow2(n: u64) -> u32 {
    1u32 << (n as u32 & 31)
}

/// `x >> s` arithmetic on 32 bits (`sra`), sign-extended back to 64.
#[inline]
fn sra32(x: u32, s: u32) -> u64 {
    sx(((x as i32) >> s) as u32)
}

/// `x << s` on 32 bits (`sll`), sign-extended back to 64.
#[inline]
fn sll32(x: u32, s: u32) -> u64 {
    sx(x << s)
}

/// TEX0 (TexInfo +0x38) texture width exponent TW (bits 26..29).
#[inline]
fn tex0_tw(tex0: u64) -> u64 {
    (tex0 >> 26) & 0xf
}

/// TEX0 texture height exponent TH (bits 30..33).
#[inline]
fn tex0_th(tex0: u64) -> u64 {
    (tex0 >> 30) & 0xf
}

/// TEX0 pixel storage format PSM (bits 20..25).
#[inline]
fn tex0_psm(tex0: u64) -> u64 {
    (tex0 >> 20) & 0x3f
}

// ---------------------------------------------------------------------------
// GsPacket: an A+D list being built.

/// A GS packet being built: `{ int count @0; u64 *cursor @4 }`. `count` is
/// the number of A+D pairs written so far. The helpers re-read both fields
/// from memory around every store, as the original does.
#[derive(Clone, Copy, Debug)]
pub struct GsPacket(pub Addr);

impl GsPacket {
    fn count(self) -> Addr {
        self.0
    }
    fn cursor_field(self) -> Addr {
        self.0.off(4)
    }
    /// The current write position (`lw`, sign-extended).
    fn cursor(self, c: &mut Cpu) -> u64 {
        self.cursor_field().lw(c)
    }
    /// Store a doubleword at `cursor + off` (re-reading the cursor).
    fn put(self, c: &mut Cpu, off: u32, v: u64) {
        let at = Addr(self.cursor(c) as u32).off(off);
        at.sd(c, v);
    }
    /// Advance the cursor by `pairs` A+D pairs and the count by `pairs`;
    /// leaves the new cursor in v1 and the new count in v0 (`pairs` = 1) or
    /// the other way round (`pairs` = 2, AddScissor), as the originals do.
    fn advance(self, c: &mut Cpu, pairs: u32) -> (u64, u64) {
        let cursor = sx((self.cursor(c) as u32).wrapping_add(pairs * 0x10));
        let count = sx((self.count().lw(c) as u32).wrapping_add(pairs));
        self.cursor_field().sw(c, cursor);
        self.count().sw(c, count);
        (cursor, count)
    }
    /// The tail every single-pair helper shares: the register id after the
    /// (already written) value, then cursor += 0x10, count += 1.
    fn finish_pair(self, c: &mut Cpu, reg_id: u64) {
        self.put(c, 8, reg_id);
        let (cursor, count) = self.advance(c, 1);
        c.g[V1][0] = cursor;
        c.g[V0][0] = count;
    }
}

/// `void GsPkt_AddClamp(GsPacket *p, int wrapMode, int ctx)` (0x3625c0):
/// appends CLAMP_1/CLAMP_2 = {WMS, WMT} for a wrap mode (0 repeat, 1 clamp
/// T, 2 clamp S, 3 clamp both; anything else repeat). Cycles: context
/// selection 3/6/4, mode 7..12, tail 9.
pub fn gs_pkt_add_clamp(c: &mut Cpu) {
    let p = GsPacket(Addr(abi::arg(c, 0)));
    let mode = abi::reg(c, A1) as i64;
    let ctx = abi::reg(c, A2);

    let reg_id = if ctx == 0 {
        c.cycles += 3;
        gs_reg::CLAMP_1
    } else if is_context_2(ctx) {
        c.cycles += 6;
        gs_reg::CLAMP_2
    } else {
        c.cycles += 4;
        gs_reg::CLAMP_1
    };

    // CLAMP: WMS in bits 0-1, WMT in bits 2-3 (1 = clamp).
    c.cycles += 3;
    let clamp = if mode == 2 {
        c.cycles += 4;
        0b0001
    } else if mode < 3 {
        c.cycles += 4;
        if mode == 1 {
            c.cycles += 3;
            0b0100
        } else {
            c.cycles += 3;
            0
        }
    } else {
        c.cycles += 5;
        if mode == 3 {
            c.cycles += 4;
            0b0101
        } else {
            c.cycles += 1;
            0
        }
    };
    p.put(c, 0, clamp);

    c.cycles += 9;
    p.finish_pair(c, reg_id);
    abi::ret(c);
}

/// `void GsPkt_AddScissor(GsPacket *p, int x, int w, int y, int h)`
/// (0x3623d8): appends SCISSOR_1 and SCISSOR_2 with the same rectangle
/// (SCAX0 = x, SCAX1 = x+w-1, SCAY0 = y, SCAY1 = y+h-1). 27 cycles.
pub fn gs_pkt_add_scissor(c: &mut Cpu) {
    c.cycles += 27;
    let p = GsPacket(Addr(abi::arg(c, 0)));
    let (x, w, y, h) = (
        abi::reg(c, A1),
        abi::arg(c, 2),
        abi::reg(c, A3),
        abi::arg(c, 4),
    );
    let x1 = sx((x as u32).wrapping_sub(1).wrapping_add(w));
    let y1 = sx((y as u32).wrapping_sub(1).wrapping_add(h));
    // 64-bit ORs of the whole registers: wide inputs bleed into neighbours.
    let scissor = x | (x1 << 16) | (y << 32) | (y1 << 48);

    p.put(c, 0x00, scissor);
    p.put(c, 0x08, gs_reg::SCISSOR_1);
    p.put(c, 0x10, scissor);
    p.put(c, 0x18, gs_reg::SCISSOR_2);
    let (cursor, count) = p.advance(c, 2);
    c.g[V0][0] = cursor;
    c.g[V1][0] = count;
    abi::ret(c);
}

/// `void GsPkt_AddTest(GsPacket *p, int zsel, int asel, u32 aref, int ctx)`
/// (0x3626d8): appends TEST_1/TEST_2 with the alpha test on (ATST: `asel`
/// 2 -> LESS, 3 -> GREATER, else ALWAYS; AREF; AFAIL = FB_ONLY) and the
/// depth test on (ZTST: `zsel` 1 -> GEQUAL, 0 -> GREATER, else ALWAYS).
/// Cycles: context 3/7/5, alpha 5/6/9/10, depth 5/7/8, tail 20.
pub fn gs_pkt_add_test(c: &mut Cpu) {
    let p = GsPacket(Addr(abi::arg(c, 0)));
    let (zsel, asel, aref, ctx) = (
        abi::reg(c, A1) as i64,
        abi::reg(c, A2) as i64,
        abi::reg(c, A3),
        abi::reg(c, T0),
    );

    let reg_id = if ctx == 0 {
        c.cycles += 3;
        gs_reg::TEST_1
    } else if is_context_2(ctx) {
        c.cycles += 7;
        gs_reg::TEST_2
    } else {
        c.cycles += 5;
        gs_reg::TEST_1
    };

    // ATST (bits 1-3).
    c.cycles += 3;
    let atst: u64 = if asel == 2 {
        c.cycles += 2;
        2
    } else if asel < 3 {
        c.cycles += 3;
        1
    } else if asel == 3 {
        c.cycles += 6;
        6
    } else {
        c.cycles += 7;
        1
    };

    // ZTST (bits 17-18).
    c.cycles += 2;
    let ztst: u64 = if zsel == 1 {
        c.cycles += 3;
        2
    } else if zsel >= 2 {
        c.cycles += 3;
        1
    } else if zsel == 0 {
        c.cycles += 5;
        3
    } else {
        c.cycles += 6;
        1
    };

    c.cycles += 20;
    const ATE: u64 = 1;
    const AFAIL_FB_ONLY: u64 = 1 << 12;
    const ZTE: u64 = 1 << 16;
    // AREF: the low 32 bits of the argument at bit 4, unmasked.
    let aref_field = (aref << 32) >> 28;
    let test = (atst << 1) | aref_field | ATE | (ztst << 17) | ZTE | AFAIL_FB_ONLY;
    p.put(c, 0, test);
    p.finish_pair(c, reg_id);
    abi::ret(c);
}

/// `void GsPkt_AddZbuf(GsPacket *p, u32 zbp, int psm, int zmsk, int ctx)`
/// (0x362660): appends ZBUF_1/ZBUF_2 = {ZBP, PSM = psm - 0x30, ZMSK = (zmsk
/// == 1)}. Cycles: context 3/7/5, 9 (+3 with ZMSK), tail 10.
pub fn gs_pkt_add_zbuf(c: &mut Cpu) {
    let p = GsPacket(Addr(abi::arg(c, 0)));
    let (zbp, psm, zmsk, ctx) = (
        abi::reg(c, A1),
        abi::arg(c, 2),
        abi::reg(c, A3),
        abi::reg(c, T0),
    );

    let reg_id = if ctx == 0 {
        c.cycles += 3;
        gs_reg::ZBUF_1
    } else if is_context_2(ctx) {
        c.cycles += 7;
        gs_reg::ZBUF_2
    } else {
        c.cycles += 5;
        gs_reg::ZBUF_1
    };

    c.cycles += 9;
    // ZBP: the whole low word; PSM: `psm - 0x30` (the Z formats are
    // 0x30..0x3a) at bit 24, as a 32-bit value (unmasked).
    let mut zbuf = ((zbp << 32) >> 32) | ((sx(psm.wrapping_sub(0x30)) << 32) >> 8);
    if zmsk == 1 {
        c.cycles += 3;
        zbuf |= 1 << 32;
    }
    c.cycles += 10;
    p.put(c, 0, zbuf);
    p.finish_pair(c, reg_id);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// TexMan packets.

/// The texture manager: `TexInfo *tex[2000]` at +8.
#[derive(Clone, Copy, Debug)]
pub struct TexMan(pub Addr);

impl TexMan {
    /// `tm->tex[slot]` (32-bit address arithmetic, no bounds check).
    fn tex(self, c: &mut Cpu, slot: u32) -> TexInfo {
        let entry = self.0 .0.wrapping_add(slot << 2);
        TexInfo(Addr(Addr(entry).off(8).lw(c) as u32))
    }
}

/// A texture descriptor (`TexInfo`, 0x58 bytes).
#[derive(Clone, Copy, Debug)]
pub struct TexInfo(pub Addr);

impl TexInfo {
    /// TBP: the VRAM block address (`int` at +0x28).
    fn tbp(self, c: &mut Cpu) -> u64 {
        self.0.off(0x28).lw(c)
    }
    /// The TEX0 register value (`u64` at +0x38).
    fn tex0(self, c: &mut Cpu) -> u64 {
        self.0.off(0x38).ld(c)
    }
}

/// `void TexMan_BuildUploadPacket(TexMan *tm, int slot, u64 **cursor)`
/// (0x367db8): builds the 6-qword packet that points the GS frame buffer
/// at a texture's VRAM so it can be drawn into: DMA CNT (5 qw), VIF FLUSH +
/// DIRECT 4, a GIF tag (A+D x3, EOP), then FRAME_1 (FBP = TBP / 32, FBW =
/// width / 64, PSM), SCISSOR_1 (the whole texture) and XYOFFSET_1 (the
/// texture centred on the 2048 primitive origin). Advances `*cursor` by
/// 0x60. 87 cycles.
pub fn tex_man_build_upload_packet(c: &mut Cpu) {
    c.cycles += 87;
    let tex = TexMan(Addr(abi::arg(c, 0))).tex(c, abi::arg(c, 1));
    let cursor = Addr(abi::arg(c, 2));
    let p = cursor.lw(c) as u32;
    let at = |o: u32| Addr(p.wrapping_add(o));

    at(0x00).sd(c, 0x1000_0005); // DMA CNT, qwc 5
    at(0x20).sd(c, 0x1000_0000_0000_8003); // GIF tag: NLOOP 3, EOP, A+D
    at(0x10).sd(c, 0x1100_0000_0000_0000); // VIF FLUSH
    at(0x18).sd(c, 0x5000_0004_0000_0000); // VIF DIRECT 4
    at(0x28).sd(c, 0xe); // GIF tag REGS = A+D
    at(0x08).sd(c, 0);

    let tex0 = tex.tex0(c);
    let tbp = tex.tbp(c);
    at(0x38).sd(c, gs_reg::FRAME_1);
    let fbw = sra32(pow2(tex0_tw(tex0)), 6);
    let frame = ((tbp as i64 >> 5) as u64) | (fbw << 16) | (tex0_psm(tex0) << 24);
    at(0x30).sd(c, frame);

    let tex0 = tex.tex0(c);
    at(0x48).sd(c, gs_reg::SCISSOR_1);
    let x1 = sx(pow2(tex0_tw(tex0)).wrapping_sub(1));
    let y1 = sx(pow2(tex0_th(tex0)).wrapping_sub(1));
    at(0x40).sd(c, (x1 << 16) | (y1 << 48));

    let tex0 = tex.tex0(c);
    at(0x58).sd(c, gs_reg::XYOFFSET_1);
    let (tw, th) = (tex0_tw(tex0), tex0_th(tex0));
    let half_w = sra32(pow2(tw), 1) as u32;
    let half_h = sra32(pow2(th), 1) as u32;
    let ofx = sll32(0x800u32.wrapping_sub(half_w), 4);
    let ofy = sll32(0x800u32.wrapping_sub(half_h), 4);
    let xyoffset = ofx | (ofy << 32);
    at(0x50).sd(c, xyoffset);

    cursor.sw(c, sx(p.wrapping_add(0x60)));
    // Leftovers: the last two results.
    c.g[V0][0] = xyoffset;
    c.g[V1][0] = sx(tw as u32);
    abi::ret(c);
}

/// `void TexMan_BuildSpritePacket(TexMan *tm, int slot, const float
/// argb[4], u64 **cursor)` (0x367f18): builds the 7-qword packet that fills
/// a texture-sized rectangle with a flat colour: DMA CNT (6 qw), VIF
/// DIRECT 5, a GIF REGLIST tag (PRIM, RGBAQ, XYZ2, XYZ2), the sprite, then
/// an A+D tag with SCISSOR_1 = (1, 1, w-2, h-2). Colour channels are the
/// floats times 128 (0x80 = full), Q = 1.0. Advances `*cursor` by 0x70.
/// 136 cycles.
///
/// The original saves s0/s1 with `sq` in a 0x20-byte frame (dead stack
/// writes that stay in RAM) and reloads them with `lq` at different
/// points; both are reproduced in place.
pub fn tex_man_build_sprite_packet(c: &mut Cpu) {
    c.cycles += 136;
    let sp = sx((abi::reg(c, SP) as u32).wrapping_sub(0x20));
    c.g[SP][0] = sp;
    let frame = Addr(sp as u32);
    c.sq(S1, frame.0);
    let entry = abi::arg(c, 0).wrapping_add(abi::arg(c, 1) << 2);
    c.sq(S0, frame.off(0x10).0);
    let rgba = Addr(abi::arg(c, 2));
    let cursor = Addr(abi::arg(c, 3));
    let p = cursor.lw(c) as u32;
    let tex = TexInfo(Addr(Addr(entry).off(8).lw(c) as u32));
    let at = |o: u32| Addr(p.wrapping_add(o));

    at(0x18).sd(c, 0x5000_0005_0000_0000); // VIF DIRECT 5
    at(0x00).sd(c, 0x1000_0006); // DMA CNT, qwc 6
    at(0x20).sd(c, 0x4400_0000_0000_0001); // GIF tag: NLOOP 1, REGLIST, NREG 4
    at(0x28).sd(c, 0x5510); // regs: PRIM, RGBAQ, XYZ2, XYZ2
    at(0x30).sd(c, 0x10e); // PRIM: sprite, Gouraud, FST
    at(0x08).sd(c, 0);
    at(0x10).sd(c, 0);

    // RGBAQ: the four channels scaled by 128 (FPU `mul.s`, `cvt.w.s`); the
    // array is {a, r, g, b}. Each converted word is sign-extended and OR-ed
    // unmasked.
    let scale = FpuF32(0x4300_0000); // 128.0
    let g = rgba.off(0x8).read_fpu(c) * scale;
    let b = rgba.off(0xc).read_fpu(c) * scale;
    let a = rgba.off(0x0).read_fpu(c) * scale;
    let r = rgba.off(0x4).read_fpu(c) * scale;
    let q = FpuF32::ONE;
    let channel = |x: FpuF32| sx(x.to_i32() as u32);
    c.lq(S1, frame.0);
    let rgbaq = channel(r)
        | (channel(g) << 8)
        | (channel(b) << 16)
        | (channel(a) << 24)
        | ((q.0 as u64) << 32);
    at(0x38).sd(c, rgbaq);

    // XYZ2 corners (12.4 fixed point, Z = 1), centred on 2048.
    const Z1: u64 = 1 << 32;
    let tex0 = tex.tex0(c);
    let half_w = sra32(pow2(tex0_tw(tex0)), 1) as u32;
    let half_h = sra32(pow2(tex0_th(tex0)), 1) as u32;
    let x0 = sll32(0x800u32.wrapping_sub(half_w), 4);
    let y0 = sll32(0x800u32.wrapping_sub(half_h), 4);
    at(0x40).sd(c, x0 | (y0 << 16) | Z1);

    let tex0 = tex.tex0(c);
    at(0x50).sd(c, 0x1000_0000_0000_8001); // GIF tag: NLOOP 1, EOP, A+D
    at(0x58).sd(c, 0xe);
    let half_w = sra32(pow2(tex0_tw(tex0)), 1) as u32;
    let half_h = sra32(pow2(tex0_th(tex0)), 1) as u32;
    let x1 = sll32(half_w.wrapping_add(0x801), 4);
    let y1 = sll32(half_h.wrapping_add(0x801), 4);
    at(0x48).sd(c, x1 | (y1 << 16) | Z1);

    let tex0 = tex.tex0(c);
    at(0x68).sd(c, gs_reg::SCISSOR_1);
    let (tw, th) = (tex0_tw(tex0), tex0_th(tex0));
    let scax1 = sx(pow2(tw).wrapping_sub(2));
    let scay1 = sx(pow2(th).wrapping_sub(2));
    const SCAX0_SCAY0_1: u64 = 0x1_0000_0001;
    let v0 = (scay1 << 48) | SCAX0_SCAY0_1;
    let scissor = (scax1 << 16) | v0;
    c.lq(S0, frame.off(0x10).0);
    at(0x60).sd(c, scissor);
    cursor.sw(c, sx(p.wrapping_add(0x70)));

    // Leftovers: FPU scratch and the last GPR results.
    c.f[0] = a.to_i32() as u32;
    c.f[1] = g.0;
    c.f[2] = r.0;
    c.f[3] = b.0;
    c.f[4] = a.0;
    c.f[5] = q.0;
    c.g[V0][0] = v0;
    c.g[V1][0] = sx(tw as u32);
    c.g[SP][0] = sx((sp as u32).wrapping_add(0x20));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// DeferMan.

/// A DeferMan draw bucket (0x20 bytes): +0xc first packet start (the CALL
/// target), +0x10 last packet end.
#[derive(Clone, Copy, Debug)]
pub struct Bucket(pub Addr);

impl Bucket {
    fn first_start(self) -> Addr {
        self.0.off(0xc)
    }
    fn last_end(self) -> Addr {
        self.0.off(0x10)
    }
}

/// `void DeferMan_AppendPacket(Bucket *b, u32 start, u32 end)` (0x362978):
/// adds a packet `[start, end)` to the bucket's DMA chain. The first packet
/// becomes the chain head; later ones are linked by writing a DMA NEXT tag
/// (`start` as the address) at the previous end, through the uncached
/// accelerated mapping (`| 0x30000000`). 4 + 2 (first) or 4 + 10 cycles.
pub fn defer_man_append_packet(c: &mut Cpu) {
    c.cycles += 4;
    let bucket = Bucket(Addr(abi::arg(c, 0)));
    let (start, end) = (abi::reg(c, A1), abi::reg(c, A2));
    let head = bucket.first_start().lw(c);
    if head == 0 {
        c.cycles += 2;
        bucket.last_end().sw(c, end);
        bucket.first_start().sw(c, start);
        c.g[V0][0] = head;
    } else {
        c.cycles += 10;
        const UNCACHED_ACCEL: u64 = 0x3000_0000;
        const DMA_NEXT: u64 = 0x2000_0000;
        let prev_end = bucket.last_end().lw(c) | UNCACHED_ACCEL;
        let tag = (start << 32) | DMA_NEXT;
        let at = Addr(prev_end as u32);
        if ssx3_render::direct::packetless() {
            // No tag: the link goes to the renderer with the frame's draws.
            ssx3_render::direct::record(ssx3_render::direct::Recorded {
                start: prev_end as u32 & 0x0fff_ffff,
                end: prev_end as u32 & 0x0fff_ffff,
                first_tag: 0,
                cmd: ssx3_render::direct::Cmd::Link(start as u32),
            });
        } else {
            at.sd(c, tag);
            at.off(8).sd(c, 0);
        }
        bucket.last_end().sw(c, end);
        c.g[V0][0] = tag;
        c.g[V1][0] = prev_end;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan frame state.

/// The graphics manager (`PS2GraphicsMan`, 0x75e0 bytes).
#[derive(Clone, Copy, Debug)]
pub struct GraphicsMan(pub Addr);

impl GraphicsMan {
    /// `int buildIndex`: the VifBuff/GifBuff half being built.
    fn build_index(self) -> Addr {
        self.0.off(0x5a10)
    }
    /// `int kickState`: 0 when the render pipeline is idle.
    fn kick_state(self) -> Addr {
        self.0.off(0x5a8c)
    }
    /// `int bufState[2]`: 0 free, 1 building, 2 queued, 3 in flight.
    fn buf_state(self, index: u32) -> Addr {
        Addr(self.0 .0.wrapping_add(0x5a90).wrapping_add(index << 2))
    }
}

/// `int PS2GraphicsMan_AcquireFrameBuffer(PS2GraphicsMan *gm)` (0x3826e0):
/// claims the half being built: free (0) becomes building (1); returns 1 if
/// it is now ours (free or already building), 0 if it is still queued or
/// displayed. 13 (free), 16 (building) or 13 (busy) cycles.
pub fn gm_acquire_frame_buffer(c: &mut Cpu) {
    c.cycles += 7;
    let gm = GraphicsMan(Addr(abi::arg(c, 0)));
    let index = gm.build_index().lw(c) as u32;
    let state = gm.buf_state(index);
    let s = state.lw(c);
    c.g[V1][0] = s;
    if s != 0 {
        c.cycles += 4;
        let s = state.lw(c);
        c.g[V1][0] = s;
        if s != 1 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return abi::ret(c);
        }
    }
    c.cycles += 3;
    if state.lw(c) == 0 {
        c.cycles += 1;
        state.sw(c, 1);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool PS2GraphicsMan_IsRenderIdle(PS2GraphicsMan *gm)` (0x382730):
/// whether `kickState` is 0. 3 cycles.
pub fn gm_is_render_idle(c: &mut Cpu) {
    c.cycles += 3;
    let gm = GraphicsMan(Addr(abi::arg(c, 0)));
    c.g[V0][0] = (gm.kick_state().lw(c) == 0) as u64;
    abi::ret(c);
}

/// `sceGsGParam *sceGsGetGParam(void)` (0x3fdaa0, SDK): the global GS
/// parameter block. 3 cycles.
pub fn sce_gs_get_gparam(c: &mut Cpu) {
    c.cycles += 3;
    const GS_GPARAM: u32 = 0x0045_1550;
    abi::set_ret(c, GS_GPARAM);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// LensFX, SkyBox, SprStream.

/// The lens flare manager (`LensFX`, 0x6820 bytes): per list an `int count`
/// at +0x60 and 0x3000-byte arrays of 0x30-byte source records at +0x88.
#[derive(Clone, Copy, Debug)]
pub struct LensFx(pub Addr);

impl LensFx {
    pub const LIST_STRIDE: u32 = 0x3000;
    pub const SOURCE_SIZE: u32 = 0x30;
    fn count(self, list: u32) -> Addr {
        Addr(self.0 .0.wrapping_add(0x60).wrapping_add(list << 2))
    }
}

/// `void LensFX_ClearSources(LensFX *l, int list)` (0x2e2f98): empties a
/// source list (the records are left as they are). 4 cycles.
pub fn lens_fx_clear_sources(c: &mut Cpu) {
    c.cycles += 4;
    let lens = LensFx(Addr(abi::arg(c, 0)));
    lens.count(abi::arg(c, 1)).sw(c, 0);
    abi::ret(c);
}

/// `void LensFX_AddSource(LensFX *l, u32 source, int list)` (0x2e2fa8):
/// appends a source record `{+0x88 source, +0x90 0, +0x94 short 0, +0x96
/// short 0}` to a list (no bounds check). 20 cycles; two `mult`s leave
/// LO/HI = count * 0x30.
pub fn lens_fx_add_source(c: &mut Cpu) {
    c.cycles += 20;
    let lens = LensFx(Addr(abi::arg(c, 0)));
    let (source, list) = (abi::reg(c, A1), abi::arg(c, 2));
    let count = lens.count(list);
    let n = count.lw(c) as u32;
    let list_off = abi::mult(c, list, LensFx::LIST_STRIDE);
    let rec_off = abi::mult(c, n, LensFx::SOURCE_SIZE);
    let rec = Addr(lens.0 .0.wrapping_add(rec_off.wrapping_add(list_off)));
    rec.off(0x88).sw(c, source);
    rec.off(0x96).sh(c, 0);
    rec.off(0x94).sh(c, 0);
    rec.off(0x90).sw(c, 0);
    let n = sx((count.lw(c) as u32).wrapping_add(1));
    count.sw(c, n);
    c.g[V0][0] = n;
    c.g[V1][0] = sx(rec.0);
    abi::ret(c);
}

/// `void SkyBox_Clear(SkyBox *s)` (0x353d98): no section (-1 at +0x10),
/// +0x14 = 0. 4 cycles.
pub fn sky_box_clear(c: &mut Cpu) {
    c.cycles += 4;
    let sky = Addr(abi::arg(c, 0));
    const NONE: u64 = u64::MAX;
    sky.off(0x14).sw(c, 0);
    sky.off(0x10).sw(c, NONE);
    c.g[V0][0] = NONE;
    abi::ret(c);
}

/// `void SprStream_NextIndex(SprStream *s)` (0x38f6a8): moves "next buffer
/// to end" (+0xc) round the ring of +4 scratchpad buffers (signed `div`
/// remainder; a zero count leaves `index + 1`). 9 cycles.
pub fn spr_stream_next_index(c: &mut Cpu) {
    c.cycles += 9;
    let s = Addr(abi::arg(c, 0));
    let next = sx((s.off(0xc).lw(c) as u32).wrapping_add(1));
    let buffers = s.off(0x4).lw(c);
    let (_, r) = abi::div(c, next as u32, buffers as u32);
    s.off(0xc).sw(c, sx(r as u32));
    c.g[V0][0] = next;
    c.g[V1][0] = buffers;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Float timers.

/// HUD effect timer step per frame (gp-0x7b18).
const HUD_FX_STEP: Addr = Addr(0x0049_b5d8);
/// Fade steps per frame: up (gp-0x51b0) and down (gp-0x51ac).
const FADE_UP_STEP: Addr = Addr(0x0049_df40);
const FADE_DOWN_STEP: Addr = Addr(0x0049_df44);

/// `void cHUDFX_Update(cHUDFX *f)` (0x116fb8): `{int state @0, float
/// target @4, float time @8}`. Unless finished (state 0x34) or untimed
/// (target -1.0), advances the time by one step and finishes once it
/// reaches the target. Cycles 6, 12, or 19/20 (+1 when it finishes).
pub fn hud_fx_update(c: &mut Cpu) {
    const FINISHED: u64 = 0x34;
    c.cycles += 4;
    let fx = Addr(abi::arg(c, 0));
    let state = fx.lw(c);
    c.g[V0][0] = state;
    c.g[V1][0] = FINISHED;
    if state != FINISHED {
        c.cycles += 6;
        let target = fx.off(4).read_fpu(c);
        let untimed = FpuF32::from_f32(-1.0);
        c.g[AT][0] = sx(untimed.0);
        c.f[2] = target.0;
        c.f[0] = untimed.0;
        let is_untimed = target.eq_s(untimed);
        abi::set_fcc(c, is_untimed);
        if !is_untimed {
            c.cycles += 7;
            let step = HUD_FX_STEP.read_fpu(c);
            let time = fx.off(8).read_fpu(c) + step;
            let done = target.le(time);
            abi::set_fcc(c, done);
            c.f[0] = time.0;
            c.f[1] = step.0;
            fx.off(8).write_fpu(c, time);
            if done {
                c.cycles += 1;
                fx.sw(c, FINISHED);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AppModule_UpdateFade(Fade *f)` (0x231d18): `{float level @4, int
/// direction @8}`. Fading in (direction != 0) adds the up step, unclamped;
/// fading out subtracts the down step and clamps at 0. Cycles 7, or 12/13.
pub fn app_module_update_fade(c: &mut Cpu) {
    c.cycles += 3;
    let fade = Addr(abi::arg(c, 0));
    let direction = fade.off(8).lw(c);
    c.g[V0][0] = direction;
    let level = fade.off(4).read_fpu(c);
    if direction == 0 {
        c.cycles += 7;
        let step = FADE_DOWN_STEP.read_fpu(c);
        let level = level - step;
        let below = level.lt(FpuF32::ZERO);
        abi::set_fcc(c, below);
        c.f[0] = level.0;
        c.f[1] = step.0;
        c.f[2] = FpuF32::ZERO.0;
        fade.off(4).write_fpu(c, level);
        if below {
            c.cycles += 1;
            fade.off(4).write_fpu(c, FpuF32::ZERO);
        }
        c.cycles += 2;
    } else {
        c.cycles += 4;
        let step = FADE_UP_STEP.read_fpu(c);
        let level = level + step;
        c.f[0] = level.0;
        c.f[1] = step.0;
        fade.off(4).write_fpu(c, level);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA};

    const SEEDS: u32 = 600;
    /// The game's gp (globals are addressed relative to it).
    const GP_VALUE: u64 = crate::difftest::GAME_GP as u64;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt)
    }

    /// Scramble, then the common pointers.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GP_VALUE;
        r
    }

    /// A selector value: small values (incl. negatives) most of the time,
    /// sometimes a wide 64-bit register.
    fn selector(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            1 => (r.next_u32() % 4) as i64 as u64 | 0x1_0000_0000,
            2 => -((r.next_u32() % 3) as i64 + 1) as u64,
            _ => (r.next_u32() % 6) as u64,
        }
    }

    /// GsPacket at DATA {count, cursor = DATA + 0x100 + 16k}.
    fn packet(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(DATA, r.next_u32());
        c.bus
            .write32(DATA + 4, DATA + 0x100 + (r.next_u32() % 16) * 0x10);
        fill(c, r, DATA + 0x100, 0x100);
        c.g[A0][0] = DATA as u64;
    }

    fn wide(r: &mut Rng) -> u64 {
        if r.next_u32() % 3 == 0 {
            (r.next_u32() as u64) << 32 | r.next_u32() as u64
        } else {
            (r.next_u32() % 0x800) as u64
        }
    }

    #[test]
    fn gs_pkt_add_clamp_matches() {
        check(
            ssx3_ee::s12::f_003625c0__generated,
            gs_pkt_add_clamp,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 1);
                packet(c, &mut r);
                c.g[A1][0] = selector(&mut r);
                c.g[A2][0] = selector(&mut r);
            },
        );
    }

    #[test]
    fn gs_pkt_add_scissor_matches() {
        check(
            ssx3_ee::s12::f_003623d8__generated,
            gs_pkt_add_scissor,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 2);
                packet(c, &mut r);
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
                c.g[T0][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn gs_pkt_add_test_matches() {
        check(
            ssx3_ee::s12::f_003626d8__generated,
            gs_pkt_add_test,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 3);
                packet(c, &mut r);
                c.g[A1][0] = selector(&mut r);
                c.g[A2][0] = selector(&mut r);
                c.g[A3][0] = wide(&mut r);
                c.g[T0][0] = selector(&mut r);
            },
        );
    }

    #[test]
    fn gs_pkt_add_zbuf_matches() {
        check(
            ssx3_ee::s12::f_00362660__generated,
            gs_pkt_add_zbuf,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 4);
                packet(c, &mut r);
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = if r.next_u32() % 2 == 0 {
                    0x30 + (r.next_u32() % 11) as u64
                } else {
                    wide(&mut r)
                };
                c.g[A3][0] = selector(&mut r);
                c.g[T0][0] = selector(&mut r);
            },
        );
    }

    /// TexMan at DATA with tex[slot] -> TexInfo at DATA + 0x2000 (random
    /// TEX0/TBP), cursor word at DATA + 0x1000 pointing to DATA + 0x3000.
    fn texman(c: &mut Cpu, r: &mut Rng) {
        fill(c, r, DATA, 0x10);
        let slot = r.next_u32() % 4;
        c.bus.write32(DATA + 8 + slot * 4, DATA + 0x2000);
        fill(c, r, DATA + 0x2000, 0x16);
        if r.next_u32() % 2 == 0 {
            c.bus.write32(DATA + 0x2028, r.next_u32() % 0x4000);
        }
        c.bus
            .write32(DATA + 0x1000, DATA + 0x3000 + (r.next_u32() % 4) * 0x10);
        fill(c, r, DATA + 0x3000, 0x40);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = slot as u64;
    }

    #[test]
    fn tex_man_build_upload_packet_matches() {
        check(
            ssx3_ee::s12::f_00367db8__generated,
            tex_man_build_upload_packet,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 5);
                texman(c, &mut r);
                c.g[A2][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn tex_man_build_sprite_packet_matches() {
        check(
            ssx3_ee::s12::f_00367f18__generated,
            tex_man_build_sprite_packet,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 6);
                texman(c, &mut r);
                for i in 0..4 {
                    let f = if r.next_u32() % 2 == 0 {
                        r.f32(2.0)
                    } else {
                        r.edge_f32()
                    };
                    c.bus.write32(DATA + 0x4000 + i * 4, f);
                }
                c.g[A2][0] = (DATA + 0x4000) as u64;
                c.g[A3][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn defer_man_append_packet_matches() {
        check(
            ssx3_ee::s12::f_00362978__generated,
            defer_man_append_packet,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 7);
                fill(c, &mut r, DATA, 8);
                if r.next_u32() % 2 == 0 {
                    c.bus.write32(DATA + 0xc, 0);
                }
                c.bus
                    .write32(DATA + 0x10, DATA + 0x200 + (r.next_u32() % 8) * 0x10);
                fill(c, &mut r, DATA + 0x200, 0x40);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = wide(&mut r);
            },
        );
    }

    fn graphics_man(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(DATA + 0x5a10, r.next_u32() % 2);
        c.bus.write32(
            DATA + 0x5a8c,
            if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() % 6
            },
        );
        for i in 0..2 {
            let s = if r.next_u32() % 5 == 0 {
                r.next_u32()
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(DATA + 0x5a90 + i * 4, s);
        }
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn gm_acquire_frame_buffer_matches() {
        check(
            ssx3_ee::s12::f_003826e0__generated,
            gm_acquire_frame_buffer,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 8);
                graphics_man(c, &mut r);
            },
        );
    }

    #[test]
    fn gm_is_render_idle_matches() {
        check(
            ssx3_ee::s12::f_00382730__generated,
            gm_is_render_idle,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 9);
                graphics_man(c, &mut r);
            },
        );
    }

    #[test]
    fn sce_gs_get_gparam_matches() {
        check(
            ssx3_ee::s15::f_003fdaa0__generated,
            sce_gs_get_gparam,
            200,
            &|c, seed| {
                base(c, seed, 10);
            },
        );
    }

    fn lens_fx(c: &mut Cpu, r: &mut Rng) {
        for l in 0..4 {
            c.bus.write32(DATA + 0x60 + l * 4, r.next_u32() % 16);
        }
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A2][0] = (r.next_u32() % 4) as u64;
    }

    #[test]
    fn lens_fx_add_source_matches() {
        check_lohi(
            ssx3_ee::s09::f_002e2fa8__generated,
            lens_fx_add_source,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 11);
                lens_fx(c, &mut r);
            },
        );
    }

    #[test]
    fn lens_fx_clear_sources_matches() {
        check(
            ssx3_ee::s09::f_002e2f98__generated,
            lens_fx_clear_sources,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 12);
                lens_fx(c, &mut r);
                c.g[A1][0] = (r.next_u32() % 64) as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }

    #[test]
    fn sky_box_clear_matches() {
        check(
            ssx3_ee::s11::f_00353d98__generated,
            sky_box_clear,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 13);
                fill(c, &mut r, DATA, 8);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn spr_stream_next_index_matches() {
        check_lohi(
            ssx3_ee::s12::f_0038f6a8__generated,
            spr_stream_next_index,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 14);
                let (n, i) = match r.next_u32() % 6 {
                    0 => (0, r.next_u32()),
                    1 => (u32::MAX, i32::MAX as u32),
                    2 => (r.next_u32(), r.next_u32()),
                    _ => (r.next_u32() % 4, r.next_u32() % 4),
                };
                c.bus.write32(DATA + 4, n);
                c.bus.write32(DATA + 0xc, i);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn hud_fx_update_matches() {
        check(
            ssx3_ee::s00::f_00116fb8__generated,
            hud_fx_update,
            1000,
            &|c, seed| {
                let mut r = base(c, seed, 15);
                let state = if r.next_u32() % 4 == 0 {
                    0x34
                } else {
                    r.next_u32() % 0x40
                };
                c.bus.write32(DATA, state);
                let target = match r.next_u32() % 4 {
                    0 => 0xbf80_0000,
                    _ => r.edge_f32(),
                };
                c.bus.write32(DATA + 4, target);
                let acc = if r.next_u32() % 3 == 0 {
                    target
                } else {
                    r.edge_f32()
                };
                c.bus.write32(DATA + 8, acc);
                let step = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    r.edge_f32()
                };
                c.bus.write32(0x0049_b5d8, step);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn app_module_update_fade_matches() {
        check(
            ssx3_ee::s06::f_00231d18__generated,
            app_module_update_fade,
            1000,
            &|c, seed| {
                let mut r = base(c, seed, 16);
                c.bus.write32(DATA + 4, r.edge_f32());
                c.bus.write32(
                    DATA + 8,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32()
                    },
                );
                c.bus.write32(0x0049_df40, r.edge_f32());
                c.bus.write32(0x0049_df44, r.edge_f32());
                c.g[A0][0] = DATA as u64;
            },
        );
    }
}
