//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! # `cOVStateHUD1P_Render` (0x001ec3f8, 19712 bytes)
//!
//! The 1P HUD's 2D render pass: `cOVStateHUD1P::onRender2D(self)` walks the
//! HUD state (`self`, passed in `a0`, kept in `s6`) and draws every enabled
//! element of the race HUD -- the race/thermometer meter
//! (`cOVStateHUD1P_DrawRaceTime`, `cOVStateHUD1P_DrawProgressMeter`), the
//! rider name and trick name lines, the timer and the money/collected
//! counters (formatted with `sprintf`/`strcpy`/`FE_FormatMoney`), the mission
//! status and the panels, icons, sprite quads and text runs that make up the
//! speedometer, pip meter and rider indicator.  Element geometry comes from
//! the HUD's `cOVStateHUDElem` records (0x20 bytes: texture, range, x/y/z/w)
//! through `HUD_GetElemPos`/`HUD_GetElemSize`/`HUD_GetElemScale`/
//! `HUD_GetElemColor`, aligned by `HUD_AlignElemPos` and submitted by
//! `HUD_DrawRect`/`HUD_DrawSprite`/`HUD_DrawTextElem`/`HUD_DrawTextLerp`/
//! `HUD_DrawWrappedText(W)`/`HUD_DrawIconText`/`HUD_DrawTextElemW`.  The
//! per-frame state drives which blocks run: the flag word at `self+0x3cc`
//! (bits 0x10000/0x20000/0x40000, tested at 0x1ec710/0x1ec810/0x1ec84c), the
//! counters at `self+0x3c0..0x3d8`, the timer/fade floats at
//! `self+0x180..0x198`, and the text objects reached through `self+0x42c`
//! (the glyph/font object every text element is drawn with).
//!
//! The reference decompilation has only assembly for this function
//! (`src/fe/ovstatehud.cpp:451`), so this replacement is a block-for-block
//! translation of the generated body: the same instructions in the same
//! order, the same cycle charges, the same registers (including the scratch
//! values left in `v0`/`v1`), the same FPU/VU0 macro ops (including the three
//! COP2 vector ops used by the rotated element), and the same load/store
//! widths.  Direct calls go through [`Callee`] (the shard's generated
//! function, which dispatches through its override slot); the vtable/function
//! pointer calls (`jalr`) leave the target in the register the original uses
//! and go through [`call_ptr`].  The generated `match pc` dispatcher is kept:
//! the original is a flattened state machine that the trampoline can re-enter
//! at any of its block labels, and unknown pcs fall through `jump_out` exactly
//! as the generated wildcard arm does.  The thirteen back edges that the
//! generated body guards with `if cycles >= next_event { pc = head; if
//! event() { return; } }` keep that shape through [`back_edge_unwinds`]: this
//! is a dispatcher, so an unwind must return at once (the following
//! `pc = head` would drop the pending-unwind bit) and the function is
//! re-entered at `head`, which the `match` accepts.

use crate::call::{call_ptr, Callee};
use crate::events::back_edge_unwinds;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001e_c3f8,
    c_ov_state_hud1p_render,
    "cOVStateHUD1P_Render",
)];

/// The functions the original calls (by address, as the generated body does).
/// Every one is a direct `jal`; the `jalr` targets are read from the caller's
/// vtables and go through [`call_ptr`].
mod callee {
    use super::{ee, Callee};

    /// `Trick_BuildName` (0x00116950).
    pub const TRICK_BUILD_NAME: Callee = Callee {
        addr: 0x0011_6950,
        f: ee::s00::f_00116950,
    };
    /// `BELibrary_GetInstance` (0x0014dc80).
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    /// `BELibrary_GetInterface` (0x0014dd58).
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    /// `BE_GetRiderDisplayName` (0x0014ee58).
    pub const BE_GET_RIDER_DISPLAY_NAME: Callee = Callee {
        addr: 0x0014_ee58,
        f: ee::s01::f_0014ee58,
    };
    /// `FE_FormatMoney` (0x00198af0).
    pub const FE_FORMAT_MONEY: Callee = Callee {
        addr: 0x0019_8af0,
        f: ee::s03::f_00198af0,
    };
    /// `HUD_GetElemColor` (0x001e91a8).
    pub const HUD_GET_ELEM_COLOR: Callee = Callee {
        addr: 0x001e_91a8,
        f: ee::s04::f_001e91a8,
    };
    /// `HUD_GetElemColorRotated` (0x001e91d0).
    pub const HUD_GET_ELEM_COLOR_ROTATED: Callee = Callee {
        addr: 0x001e_91d0,
        f: ee::s04::f_001e91d0,
    };
    /// `HUD_GetElemPos` (0x001e91f8).
    pub const HUD_GET_ELEM_POS: Callee = Callee {
        addr: 0x001e_91f8,
        f: ee::s04::f_001e91f8,
    };
    /// `HUD_GetElemSize` (0x001e9220).
    pub const HUD_GET_ELEM_SIZE: Callee = Callee {
        addr: 0x001e_9220,
        f: ee::s04::f_001e9220,
    };
    /// `HUD_GetElemScale` (0x001e9290).
    pub const HUD_GET_ELEM_SCALE: Callee = Callee {
        addr: 0x001e_9290,
        f: ee::s04::f_001e9290,
    };
    /// `HUD_DrawIconText` (0x001e95a0).
    pub const HUD_DRAW_ICON_TEXT: Callee = Callee {
        addr: 0x001e_95a0,
        f: ee::s04::f_001e95a0,
    };
    /// `HUD_AlignElemPos` (0x001f10f8).
    pub const HUD_ALIGN_ELEM_POS: Callee = Callee {
        addr: 0x001f_10f8,
        f: ee::s04::f_001f10f8,
    };
    /// `HUD_DrawSprite` (0x001f1190).
    pub const HUD_DRAW_SPRITE: Callee = Callee {
        addr: 0x001f_1190,
        f: ee::s04::f_001f1190,
    };
    /// `HUD_DrawRect` (0x001f1338).
    pub const HUD_DRAW_RECT: Callee = Callee {
        addr: 0x001f_1338,
        f: ee::s04::f_001f1338,
    };
    /// `HUD_DrawWobbleText` (0x001f14b0).
    pub const HUD_DRAW_WOBBLE_TEXT: Callee = Callee {
        addr: 0x001f_14b0,
        f: ee::s04::f_001f14b0,
    };
    /// `HUD_DrawTimeElem` (0x001f16c0).
    pub const HUD_DRAW_TIME_ELEM: Callee = Callee {
        addr: 0x001f_16c0,
        f: ee::s04::f_001f16c0,
    };
    /// `HUD_DrawTime` (0x001f1840).
    pub const HUD_DRAW_TIME: Callee = Callee {
        addr: 0x001f_1840,
        f: ee::s04::f_001f1840,
    };
    /// `HUD_DrawTextElem` (0x001f1b30).
    pub const HUD_DRAW_TEXT_ELEM: Callee = Callee {
        addr: 0x001f_1b30,
        f: ee::s04::f_001f1b30,
    };
    /// `HUD_DrawTextLerp` (0x001f1e28).
    pub const HUD_DRAW_TEXT_LERP: Callee = Callee {
        addr: 0x001f_1e28,
        f: ee::s04::f_001f1e28,
    };
    /// `HUD_DrawTextElemW` (0x001f22e8).
    pub const HUD_DRAW_TEXT_ELEM_W: Callee = Callee {
        addr: 0x001f_22e8,
        f: ee::s04::f_001f22e8,
    };
    /// `HUD_DrawWrappedText` (0x001f2aa0).
    pub const HUD_DRAW_WRAPPED_TEXT: Callee = Callee {
        addr: 0x001f_2aa0,
        f: ee::s04::f_001f2aa0,
    };
    /// `HUD_DrawWrappedTextW` (0x001f2db0).
    pub const HUD_DRAW_WRAPPED_TEXT_W: Callee = Callee {
        addr: 0x001f_2db0,
        f: ee::s04::f_001f2db0,
    };
    /// `cOVStateHUD1P_DrawProgressMeter` (0x0020eda0).
    pub const C_OV_STATE_HUD1_P_DRAW_PROGRESS_METER: Callee = Callee {
        addr: 0x0020_eda0,
        f: ee::s05::f_0020eda0,
    };
    /// `cOVStateHUD1P_DrawProgressMeterMP` (0x0020fb40).
    pub const C_OV_STATE_HUD1_P_DRAW_PROGRESS_METER_MP: Callee = Callee {
        addr: 0x0020_fb40,
        f: ee::s05::f_0020fb40,
    };
    /// `HUD_DrawMeterBar` (0x0021d1a0).
    pub const HUD_DRAW_METER_BAR: Callee = Callee {
        addr: 0x0021_d1a0,
        f: ee::s05::f_0021d1a0,
    };
    /// `HUD_DrawControlHint` (0x0021d9a0).
    pub const HUD_DRAW_CONTROL_HINT: Callee = Callee {
        addr: 0x0021_d9a0,
        f: ee::s05::f_0021d9a0,
    };
    /// `HUD_DrawRiderIndicator` (0x0021e1b0).
    pub const HUD_DRAW_RIDER_INDICATOR: Callee = Callee {
        addr: 0x0021_e1b0,
        f: ee::s05::f_0021e1b0,
    };
    /// `HUD_AlignX` (0x0021e750).
    pub const HUD_ALIGN_X: Callee = Callee {
        addr: 0x0021_e750,
        f: ee::s05::f_0021e750,
    };
    /// `HUD_AlignY` (0x0021e7a8).
    pub const HUD_ALIGN_Y: Callee = Callee {
        addr: 0x0021_e7a8,
        f: ee::s05::f_0021e7a8,
    };
    /// `HUD_DrawStretchedPanel` (0x0021e7e0).
    pub const HUD_DRAW_STRETCHED_PANEL: Callee = Callee {
        addr: 0x0021_e7e0,
        f: ee::s05::f_0021e7e0,
    };
    /// `HUD_DrawPositionPanel` (0x0021ed48).
    pub const HUD_DRAW_POSITION_PANEL: Callee = Callee {
        addr: 0x0021_ed48,
        f: ee::s05::f_0021ed48,
    };
    /// `HUD_DrawPipMeter` (0x0021f338).
    pub const HUD_DRAW_PIP_METER: Callee = Callee {
        addr: 0x0021_f338,
        f: ee::s05::f_0021f338,
    };
    /// `cOVStateHUD1P_DrawRaceTime` (0x0021f660).
    pub const C_OV_STATE_HUD1_P_DRAW_RACE_TIME: Callee = Callee {
        addr: 0x0021_f660,
        f: ee::s05::f_0021f660,
    };
    /// `cOVStateHUD1P_DrawCollectedCount` (0x0021f9b0).
    pub const C_OV_STATE_HUD1_P_DRAW_COLLECTED_COUNT: Callee = Callee {
        addr: 0x0021_f9b0,
        f: ee::s05::f_0021f9b0,
    };
    /// `cOVStateHUD1P_DrawMissionStatus` (0x0021fd38).
    pub const C_OV_STATE_HUD1_P_DRAW_MISSION_STATUS: Callee = Callee {
        addr: 0x0021_fd38,
        f: ee::s05::f_0021fd38,
    };
    /// `HUD_DrawSpeedometer` (0x002200c0).
    pub const HUD_DRAW_SPEEDOMETER: Callee = Callee {
        addr: 0x0022_00c0,
        f: ee::s05::f_002200c0,
    };
    /// `HUD_DrawFadingText` (0x002204a0).
    pub const HUD_DRAW_FADING_TEXT: Callee = Callee {
        addr: 0x0022_04a0,
        f: ee::s05::f_002204a0,
    };
    /// `HUD_DrawOptionIconText` (0x00220ad0).
    pub const HUD_DRAW_OPTION_ICON_TEXT: Callee = Callee {
        addr: 0x0022_0ad0,
        f: ee::s05::f_00220ad0,
    };
    /// `WScriptMan_GetHudTargetHeight` (0x00309ba8).
    pub const W_SCRIPT_MAN_GET_HUD_TARGET_HEIGHT: Callee = Callee {
        addr: 0x0030_9ba8,
        f: ee::s10::f_00309ba8,
    };
    /// `StrHash` (0x00317670).
    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };
    /// `Font_DrawText` (0x00391cb0).
    pub const FONT_DRAW_TEXT: Callee = Callee {
        addr: 0x0039_1cb0,
        f: ee::s12::f_00391cb0,
    };
    /// `Font_MeasureText` (0x00391fb0).
    pub const FONT_MEASURE_TEXT: Callee = Callee {
        addr: 0x0039_1fb0,
        f: ee::s12::f_00391fb0,
    };
    /// `Font_FindLineBreak` (0x00392430).
    pub const FONT_FIND_LINE_BREAK: Callee = Callee {
        addr: 0x0039_2430,
        f: ee::s12::f_00392430,
    };
    /// `Font_DrawTextCentered` (0x00392908).
    pub const FONT_DRAW_TEXT_CENTERED: Callee = Callee {
        addr: 0x0039_2908,
        f: ee::s12::f_00392908,
    };
    /// `strcpy` (0x004166f4).
    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::f_004166f4,
    };
    /// `sprintf` (0x00417828).
    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };
}

/// `void cOVStateHUD1P_Render(cOVStateHUD1P* self)` (0x001ec3f8): the 1P HUD's
/// 2D render pass, block for block (see the module docs).  Every `pc` in the
/// `match` is a block label of the original, so the function can be entered at
/// any of them.
pub fn c_ov_state_hud1p_render(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x001ec3f8 => {
                c.cycles += 32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x470));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x1450)).lw(c);
                c.g[S6][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                Addr((c.g[SP][0] as u32).wrapping_add(0x380)).sd(c, c.g[RA][0]);
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x468));
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x460));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x458));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x450));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x448));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x440));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x438));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x428));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x420));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                Addr((c.g[SP][0] as u32).wrapping_add(0x44)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x48)).sw(c, 0u64);
                let t = c.g[V0][0] != 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x4c)).sw(c, 0u64);
                if t { pc = 0x001f10a0; continue; }
                pc = 0x001ec478; continue;
            }
            0x001ec478 => {
                c.cycles += 94;
                if callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x001ec480).is_err() { return; }
                c.g[A1][0] = 0x4;
                c.g[A0][0] = c.g[V0][0];
                if callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x001ec48c).is_err() { return; }
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[V0][0] as u32));
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x23));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
                Addr((c.g[V0][0] as u32).wrapping_add(0x24)).sw(c, c.g[A2][0]);
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xac)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001ec4e0).is_err() { return; }
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xf8)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xfc)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001ec4f8).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A0][0] = 0x500000;
                c.g[A2][0] = 0xfffffffffe7f0000;
                c.g[T3][0] = 0x1000000;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A2][0] = c.g[A2][0] | 0xffff;
                c.g[T0][0] = 0xffffffffffcf0000;
                c.g[T2][0] = 0xfffffffffff00000;
                c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1420));
                c.ldl(9, (c.g[T6][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T6][0] as u32));
                c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[T5][0] = Addr((c.g[T6][0] as u32).wrapping_add(0x10)).lw(c);
                c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(9, (c.g[V1][0] as u32));
                c.sdl(12, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(12, (c.g[V1][0] as u32).wrapping_add(0x8));
                Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[T5][0]);
                c.g[T0][0] = c.g[T0][0] | 0xffff;
                c.g[T2][0] = c.g[T2][0] | 0xfff;
                c.g[T1][0] = 0x10000;
                c.g[A3][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T1][0] = c.g[T1][0] | 0x4000;
                c.g[T4][0] = 0xffffffffffffff83;
                c.g[A1][0] = 0x500000;
                c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                c.g[A2][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                Addr((c.g[A2][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x110)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x114)).lw(c);
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001ec5c4).is_err() { return; }
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xb8)).lh(c);
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbc));
                c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                if call_ptr(c, t, 0x001ec5e8).is_err() { return; }
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001ec600; continue; }
                pc = 0x001ec5f0; continue;
            }
            0x001ec5f0 => {
                c.cycles += 4;
                c.mtc1(2, 20);
                c.cvt_s_w(20, 20);
                let t = true; 
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
                if t { pc = 0x001ec61c; continue; }
                pc = 0x001ec600; continue;
            }
            0x001ec600 => {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.mtc1(2, 20);
                c.cvt_s_w(20, 20);
                c.add_s(20, 20, 20);
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
                pc = 0x001ec61c; continue;
            }
            0x001ec61c => {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001ec62c).is_err() { return; }
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t { pc = 0x001ec644; continue; }
                pc = 0x001ec634; continue;
            }
            0x001ec634 => {
                c.cycles += 4;
                c.mtc1(2, 15);
                c.cvt_s_w(15, 15);
                let t = true; 
                
                if t { pc = 0x001ec658; continue; }
                pc = 0x001ec644; continue;
            }
            0x001ec644 => {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.mtc1(2, 15);
                c.cvt_s_w(15, 15);
                c.add_s(15, 15, 15);
                pc = 0x001ec658; continue;
            }
            0x001ec658 => {
                c.cycles += 31;
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = Addr((c.g[S1][0] as u32)).lw(c);
                c.mov_s(14, 20);
                c.mov_s(13, 12);
                let t = c.g[V0][0] as u32;
                c.g[A1][0] = 0x1;
                if call_ptr(c, t, 0x001ec674).is_err() { return; }
                c.g[A3][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A1][0] = 0x1;
                c.mtc1(0, 12);
                c.g[A2][0] = 0u64;
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[AT][0] = 0x44200000;
                c.mtc1(1, 14);
                c.mov_s(13, 12);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xd0)).lh(c);
                c.mov_s(16, 12);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xd4)).lw(c);
                c.g[AT][0] = 0x43f00000;
                c.mtc1(1, 15);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 17);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001ec6b8).is_err() { return; }
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lw(c);
                let t = c.g[V1][0] != c.g[A1][0]; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x320)).sw(c, c.g[V1][0]);
                if t { pc = 0x001ec6f4; continue; }
                pc = 0x001ec6d4; continue;
            }
            0x001ec6d4 => {
                c.cycles += 8;
                c.g[V0][0] = 0x470000;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x53c));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x68b0));
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x54c));
                Addr((c.g[SP][0] as u32).wrapping_add(0x328)).sw(c, c.g[V1][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x324)).sw(c, c.g[V0][0]);
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x32c)).sw(c, c.g[A0][0]);
                if t { pc = 0x001ec710; continue; }
                pc = 0x001ec6f4; continue;
            }
            0x001ec6f4 => {
                c.cycles += 7;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x544));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x73f0));
                c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x554));
                Addr((c.g[SP][0] as u32).wrapping_add(0x328)).sw(c, c.g[A1][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x324)).sw(c, c.g[V0][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x32c)).sw(c, c.g[A2][0]);
                pc = 0x001ec710; continue;
            }
            0x001ec710 => {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3cc)).lw(c);
                c.g[V1][0] = 0x20000;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[T1][0] = 0u64;
                if t { pc = 0x001ec810; continue; }
                pc = 0x001ec724; continue;
            }
            0x001ec724 => {
                c.cycles += 59;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = 0x3e;
                c.g[T0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x944));
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if callee::HUD_DRAW_WRAPPED_TEXT_W.call(c, 0x001ec740).is_err() { return; }
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[A0][0] = c.g[S6][0];
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x5c));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 3);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 5);
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.add_s(0, 0, 3);
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.add_s(2, 2, 3);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.sub_s(1, 1, 5);
                c.sub_s(4, 4, 5);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x5c));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7838));
                c.ldl(9, (c.g[T5][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T5][0] as u32));
                c.ldl(11, (c.g[T5][0] as u32).wrapping_add(0xf));
                c.ldr(11, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x5c));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V0][0] = Addr((c.g[T6][0] as u32).wrapping_add(0x8da)).lb(c);
                c.g[V1][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] & 0x3e0;
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                Addr((c.g[T1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_RECT.call(c, 0x001ec810).is_err() { return; }
                pc = 0x001ec810; continue;
            }
            0x001ec810 => {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3cc)).lw(c);
                c.g[V1][0] = 0x40000;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = c.g[S6][0];
                if t { pc = 0x001ec840; continue; }
                pc = 0x001ec824; continue;
            }
            0x001ec824 => {
                c.cycles += 7;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = 0x3f;
                c.g[T0][0] = sx((c.g[S6][0] as u32).wrapping_add(0xa48));
                c.g[T1][0] = 0u64;
                c.g[T2][0] = 0u64;
                if callee::HUD_DRAW_WRAPPED_TEXT_W.call(c, 0x001ec840).is_err() { return; }
                pc = 0x001ec840; continue;
            }
            0x001ec840 => {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3d8)).lw(c);
                
                if c.g[V0][0] != 0u64 { c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0xc94)); pc = 0x001ec890; continue; }
                pc = 0x001ec84c; continue;
            }
            0x001ec84c => {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3cc)).lw(c);
                c.g[V1][0] = 0x10000;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                
                if c.g[V0][0] == 0u64 { c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0xc94)); pc = 0x001ec890; continue; }
                pc = 0x001ec860; continue;
            }
            0x001ec860 => {
                c.cycles += 6;
                c.lwc1(1, (c.g[S6][0] as u32).wrapping_add(0x190));
                c.mtc1(0, 0);
                c.c_lt_s(0, 1);
                let t = !c.fcc(); 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x320)).lw(c);
                if t { pc = 0x001ec88c; continue; }
                pc = 0x001ec878; continue;
            }
            0x001ec878 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[V0][0] ^ 0x1;
                c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
                if callee::HUD_DRAW_FADING_TEXT.call(c, 0x001ec88c).is_err() { return; }
                pc = 0x001ec88c; continue;
            }
            0x001ec88c => {
                c.cycles += 1;
                c.lwc1(12, (c.g[S6][0] as u32).wrapping_add(0xc94));
                pc = 0x001ec890; continue;
            }
            0x001ec890 => {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_le_s(0, 12);
                let t = !c.fcc(); 
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001ec8ac; continue; }
                pc = 0x001ec8a4; continue;
            }
            0x001ec8a4 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S6][0];
                if callee::HUD_DRAW_OPTION_ICON_TEXT.call(c, 0x001ec8ac).is_err() { return; }
                pc = 0x001ec8ac; continue;
            }
            0x001ec8ac => {
                c.cycles += 5;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A0][0] & 0x7;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001ec918; continue; }
                pc = 0x001ec8c0; continue;
            }
            0x001ec8c0 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                pc = 0x001ec8c4; continue;
            }
            0x001ec8c4 => {
                c.cycles += 19;
                c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[A0][0] as u32));
                c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[V1][0] as u32));
                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                let t = c.g[A0][0] != c.g[V0][0]; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t { if back_edge_unwinds(c, 0x001ec8c4) { return; } pc = 0x001ec8c4; continue; }
                pc = 0x001ec910; continue;
            }
            0x001ec910 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x001ec948; continue; }
                pc = 0x001ec918; continue;
            }
            0x001ec918 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                pc = 0x001ec91c; continue;
            }
            0x001ec91c => {
                c.cycles += 11;
                c.g[T1][0] = Addr((c.g[A0][0] as u32)).ld(c);
                c.g[T2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).ld(c);
                c.g[T3][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).ld(c);
                c.g[T4][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).ld(c);
                Addr((c.g[V1][0] as u32)).sd(c, c.g[T1][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sd(c, c.g[T2][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sd(c, c.g[T3][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sd(c, c.g[T4][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                let t = c.g[A0][0] != c.g[V0][0]; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t { if back_edge_unwinds(c, 0x001ec91c) { return; } pc = 0x001ec91c; continue; }
                pc = 0x001ec948; continue;
            }
            0x001ec948 => {
                c.cycles += 13;
                c.ldl(13, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.ldr(13, (c.g[A0][0] as u32));
                c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(13, (c.g[V1][0] as u32));
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x90)).lw(c);
                c.g[A0][0] = 0xffffffffc0000000;
                c.g[V1][0] = 0xffffffff80000000;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3d4)).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V1][0] = c.g[V0][0] ^ c.g[V1][0];
                c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
                let t = c.g[A1][0] != 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x330)).sw(c, c.g[V1][0]);
                if t { pc = 0x001ec9ac; continue; }
                pc = 0x001ec97c; continue;
            }
            0x001ec97c => {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3d8)).lw(c);
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001ec9b0; continue; }
                pc = 0x001ec988; continue;
            }
            0x001ec988 => {
                c.cycles += 6;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
                c.g[A0][0] = Addr((c.g[A0][0] as u32)).lw(c);
                let t = c.g[A0][0] == 0u64; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x001ec9a4; continue; }
                pc = 0x001ec9a0; continue;
            }
            0x001ec9a0 => {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                pc = 0x001ec9a4; continue;
            }
            0x001ec9a4 => {
                c.cycles += 2;
                
                if c.g[A1][0] == 0u64 { c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c); pc = 0x001ec9f0; continue; }
                pc = 0x001ec9ac; continue;
            }
            0x001ec9ac => {
                c.cycles += 1;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                pc = 0x001ec9b0; continue;
            }
            0x001ec9b0 => {
                c.cycles += 16;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xb0)).lh(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xb4)).lw(c);
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if call_ptr(c, t, 0x001ec9d0).is_err() { return; }
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x100)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x104)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001ec9e8).is_err() { return; }
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                if t { pc = 0x001f10a4; continue; }
                pc = 0x001ec9f0; continue;
            }
            0x001ec9f0 => {
                c.cycles += 48;
                c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x48));
                Addr((c.g[SP][0] as u32).wrapping_add(0x334)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x338)).sw(c, c.g[A2][0]);
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x4f));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x5f));
                c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x58));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.ldl(11, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(11, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x2f));
                c.sdr(11, (c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x4f));
                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x5f));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x58));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x48)).lw(c);
                
                if (c.g[V0][0] as i64) < 0 { c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3c4)).lw(c); pc = 0x001f07f0; continue; }
                pc = 0x001ecab0; continue;
            }
            0x001ecab0 => {
                c.cycles += 2;
                Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).sw(c, 0u64);
                pc = 0x001ecab8; continue;
            }
            0x001ecab8 => {
                c.cycles += 19;
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[V1][0] = Addr((c.g[T3][0] as u32)).lw(c);
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x370)).sw(c, c.g[T4][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x320)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x340)).sw(c, c.g[V0][0]);
                c.g[T5][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x790)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x344)).sw(c, c.g[T5][0]);
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x870)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T6][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = 0x100000;
                if t { pc = 0x001f07b0; continue; }
                pc = 0x001ecb04; continue;
            }
            0x001ecb04 => {
                c.cycles += 9;
                c.g[V1][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x80)).lw(c);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3cc)).lw(c);
                c.g[A0][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = !(0u64 | c.g[V1][0]);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[A1][0] = c.g[V0][0] & c.g[A1][0];
                let t = c.g[A1][0] == 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x348)).sw(c, c.g[V0][0]);
                if t { pc = 0x001ecb78; continue; }
                pc = 0x001ecb28; continue;
            }
            0x001ecb28 => {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x88)).lw(c);
                c.g[V0][0] = 0x2;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x001ecb58; continue; }
                pc = 0x001ecb38; continue;
            }
            0x001ecb38 => {
                c.cycles += 8;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A2][0] = c.g[T3][0];
                c.g[T0][0] = 0x1;
                if callee::C_OV_STATE_HUD1_P_DRAW_RACE_TIME.call(c, 0x001ecb50).is_err() { return; }
                let t = true; 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001ecb7c; continue; }
                pc = 0x001ecb58; continue;
            }
            0x001ecb58 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001ecb7c; continue; }
                pc = 0x001ecb60; continue;
            }
            0x001ecb60 => {
                c.cycles += 6;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[T0][0] = 0u64;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                if callee::C_OV_STATE_HUD1_P_DRAW_RACE_TIME.call(c, 0x001ecb78).is_err() { return; }
                pc = 0x001ecb78; continue;
            }
            0x001ecb78 => {
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                pc = 0x001ecb7c; continue;
            }
            0x001ecb7c => {
                c.cycles += 14;
                c.g[A0][0] = 0u64;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 26);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1b0)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x34c)).sw(c, 0u64);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xea0));
                c.mtc1(0, 28);
                if c.g[V1][0] != 0 { c.g[A0][0] = c.g[V0][0]; }
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 25);
                c.g[V0][0] = Addr((c.g[A0][0] as u32)).lw(c);
                c.g[V0][0] = c.g[V0][0] ^ 0x34;
                let t = c.g[V0][0] == 0u64; 
                c.mov_s(27, 26);
                if t { pc = 0x001ecc08; continue; }
                pc = 0x001ecbb4; continue;
            }
            0x001ecbb4 => {
                c.cycles += 21;
                c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5704));
                c.div_s(27, 1, 3);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5700));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x56fc));
                c.mul_s(0, 27, 0);
                c.mul_s(1, 27, 27);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 25, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(25, 0, 25);
                pc = 0x001ecc08; continue;
            }
            0x001ecc08 => {
                c.cycles += 8;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x330)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 29);
                Addr((c.g[SP][0] as u32).wrapping_add(0x350)).sw(c, 0u64);
                let t = c.g[V1][0] != 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x370)).sw(c, c.g[V0][0]);
                if t { pc = 0x001f07b0; continue; }
                pc = 0x001ecc28; continue;
            }
            0x001ecc28 => {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1b4)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x354)).sw(c, 0u64);
                if t { pc = 0x001efad4; continue; }
                pc = 0x001ecc3c; continue;
            }
            0x001ecc3c => {
                c.cycles += 1;
                pc = 0x001ecc40; continue;
            }
            0x001ecc40 => {
                c.cycles += 4;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1b0)).lw(c);
                let t = c.g[V1][0] == 0u64; 
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                if t { pc = 0x001ecc60; continue; }
                pc = 0x001ecc50; continue;
            }
            0x001ecc50 => {
                c.cycles += 4;
                c.g[V0][0] = 0x9c;
                c.mult(0, 4, 4, 2);
                let t = true; 
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if t { pc = 0x001ecc64; continue; }
                pc = 0x001ecc60; continue;
            }
            0x001ecc60 => {
                c.cycles += 1;
                c.g[S4][0] = 0u64;
                pc = 0x001ecc64; continue;
            }
            0x001ecc64 => {
                c.cycles += 14;
                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                Addr((c.g[SP][0] as u32).wrapping_add(0x358)).sw(c, c.g[A1][0]);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[A3][0] = Addr((c.g[S4][0] as u32).wrapping_add(0xc)).lw(c);
                c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[S5][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = Addr((c.g[S4][0] as u32)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).sw(c, c.g[A3][0]);
                c.div_s(22, 1, 0);
                c.g[V0][0] = (c.g[V1][0] < 0x34u64) as u64;
                c.g[T0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
                let t = c.g[V0][0] == 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x360)).sw(c, c.g[T0][0]);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ecc9c; continue;
            }
            0x001ecc9c => {
                c.cycles += 7;
                c.g[V0][0] = 0x470000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1390));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = Addr((c.g[V1][0] as u32)).lw(c);
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            0x001eccb8 => {
                c.cycles += 6;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x200000;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T2][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001eccd0; continue;
            }
            0x001eccd0 => {
                c.cycles += 7;
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.mtc1(0, 0);
                c.lwc1(1, (c.g[T3][0] as u32).wrapping_add(0x64));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ecd18; continue; }
                pc = 0x001eccec; continue;
            }
            0x001eccec => {
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b58));
                c.ldl(12, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(12, (c.g[V1][0] as u32));
                c.ldl(13, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(13, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ecd30; continue; }
                pc = 0x001ecd18; continue;
            }
            0x001ecd18 => {
                c.cycles += 6;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[S0][0];
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                c.g[A1][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ecd30).is_err() { return; }
                pc = 0x001ecd30; continue;
            }
            0x001ecd30 => {
                c.cycles += 12;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.mov_s(12, 22);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 14);
                c.g[T1][0] = c.g[S2][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x328)).lw(c);
                c.g[A2][0] = 0x7;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                let t = true; 
                c.g[A3][0] = 0x4;
                if t { pc = 0x001ecfe4; continue; }
                pc = 0x001ecd60; continue;
            }
            0x001ecd60 => {
                c.cycles += 6;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x200000;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T6][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ecd78; continue;
            }
            0x001ecd78 => {
                c.cycles += 7;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.mtc1(0, 0);
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x64));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ecdc0; continue; }
                pc = 0x001ecd94; continue;
            }
            0x001ecd94 => {
                c.cycles += 11;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b58));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A2][0] as u32));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                if t { pc = 0x001ecfbc; continue; }
                pc = 0x001ecdc0; continue;
            }
            0x001ecdc0 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56f8));
                c.c_le_s(22, 0);
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ece00; continue; }
                pc = 0x001ecdd4; continue;
            }
            0x001ecdd4 => {
                c.cycles += 11;
                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b38));
                c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T2][0] as u32));
                c.ldl(8, (c.g[T2][0] as u32).wrapping_add(0xf));
                c.ldr(8, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                if t { pc = 0x001ecfbc; continue; }
                pc = 0x001ece00; continue;
            }
            0x001ece00 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56f4));
                c.c_lt_s(22, 0);
                let t = !c.fcc(); 
                c.g[V1][0] = 0x4d0000;
                if t { pc = 0x001ece74; continue; }
                pc = 0x001ece14; continue;
            }
            0x001ece14 => {
                c.cycles += 24;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56f0));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x56ec));
                c.g[V0][0] = 0x4d0000;
                c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7b38));
                c.ldl(11, (c.g[T6][0] as u32).wrapping_add(0x7));
                c.ldr(11, (c.g[T6][0] as u32));
                c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x56e8));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b18));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A2][0] as u32));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x78));
                let t = true; 
                c.sub_s(0, 22, 0);
                if t { pc = 0x001ecf24; continue; }
                pc = 0x001ece74; continue;
            }
            0x001ece74 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56e4));
                c.c_le_s(22, 0);
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001eceb4; continue; }
                pc = 0x001ece88; continue;
            }
            0x001ece88 => {
                c.cycles += 11;
                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b18));
                c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T2][0] as u32));
                c.ldl(8, (c.g[T2][0] as u32).wrapping_add(0xf));
                c.ldr(8, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                if t { pc = 0x001ecfbc; continue; }
                pc = 0x001eceb4; continue;
            }
            0x001eceb4 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56e0));
                c.c_lt_s(22, 0);
                let t = !c.fcc(); 
                c.g[V1][0] = 0x4d0000;
                if t { pc = 0x001ecf90; continue; }
                pc = 0x001ecec8; continue;
            }
            0x001ecec8 => {
                c.cycles += 23;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56dc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x56d8));
                c.g[V0][0] = 0x4d0000;
                c.sub_s(0, 22, 0);
                c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7b18));
                c.ldl(11, (c.g[T6][0] as u32).wrapping_add(0x7));
                c.ldr(11, (c.g[T6][0] as u32));
                c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x56d4));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7af8));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A2][0] as u32));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x78));
                pc = 0x001ecf24; continue;
            }
            0x001ecf24 => {
                c.cycles += 27;
                c.add_s(0, 0, 1);
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.mul_s(0, 0, 3);
                c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x6c));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.sub_s(2, 2, 5);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x7c));
                c.sub_s(3, 3, 6);
                c.sub_s(4, 4, 7);
                c.sub_s(1, 1, 8);
                c.mul_s(2, 2, 0);
                c.mul_s(3, 3, 0);
                c.mul_s(1, 1, 0);
                c.mul_s(4, 4, 0);
                c.add_s(5, 5, 2);
                c.add_s(6, 6, 3);
                c.add_s(7, 7, 4);
                c.add_s(8, 8, 1);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x001ecfb8; continue; }
                pc = 0x001ecf90; continue;
            }
            0x001ecf90 => {
                c.cycles += 10;
                c.g[V0][0] = 0x4d0000;
                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7af8));
                c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T2][0] as u32));
                c.ldl(8, (c.g[T2][0] as u32).wrapping_add(0xf));
                c.ldr(8, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x58));
                pc = 0x001ecfb8; continue;
            }
            0x001ecfb8 => {
                c.cycles += 1;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                pc = 0x001ecfbc; continue;
            }
            0x001ecfbc => {
                c.cycles += 10;
                c.mov_s(12, 22);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 14);
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = 0x6;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x328)).lw(c);
                c.g[A3][0] = 0x2;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                pc = 0x001ecfe4; continue;
            }
            0x001ecfe4 => {
                c.cycles += 6;
                c.mov_s(13, 25);
                c.g[T3][0] = 0u64;
                c.g[S5][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                if callee::HUD_DRAW_METER_BAR.call(c, 0x001ecff4).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ecffc; continue;
            }
            0x001ecffc => {
                c.cycles += 5;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = c.g[T5][0] & 0x402;
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ed010; continue;
            }
            0x001ed010 => {
                c.cycles += 6;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[A0][0] = c.g[T5][0] & 0x400;
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x7c)).lw(c);
                let t = c.g[A0][0] == 0u64; 
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if t { pc = 0x001ed030; continue; }
                pc = 0x001ed028; continue;
            }
            0x001ed028 => {
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
                pc = 0x001ed030; continue;
            }
            0x001ed030 => {
                c.cycles += 10;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                if callee::SPRINTF.call(c, 0x001ed044).is_err() { return; }
                c.mtc1(0, 0);
                c.c_lt_s(27, 0);
                let t = !c.fcc(); 
                c.g[S2][0] = c.g[S0][0];
                if t { pc = 0x001ed098; continue; }
                pc = 0x001ed058; continue;
            }
            0x001ed058 => {
                c.cycles += 16;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[S2][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0x11;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ed090).is_err() { return; }
                let t = true; 
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                if t { pc = 0x001ed108; continue; }
                pc = 0x001ed098; continue;
            }
            0x001ed098 => {
                c.cycles += 28;
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = 0x7f;
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[T4][0] = 0x4d0000;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T0][0] = c.g[S2][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V1][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0x11;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x7ad8));
                c.ldl(13, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(13, (c.g[V1][0] as u32));
                c.ldl(14, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x87));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x8f));
                c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x88));
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ed104).is_err() { return; }
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                pc = 0x001ed108; continue;
            }
            0x001ed108 => {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = c.g[A1][0] & 0x10;
                c.g[FP][0] = c.g[A1][0] & 0x20;
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001ed4ac; continue; }
                pc = 0x001ed11c; continue;
            }
            0x001ed11c => {
                c.cycles += 57;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14c0));
                c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T2][0] as u32));
                c.g[T0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x8)).lw(c);
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x70));
                Addr((c.g[SP][0] as u32).wrapping_add(0x78)).sw(c, c.g[T0][0]);
                c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x33c));
                c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x3ec));
                c.g[A0][0] = c.g[S0][0];
                c.g[S3][0] = 0xfffffffffffffc1f;
                if callee::HUD_GET_ELEM_POS.call(c, 0x001ed158).is_err() { return; }
                c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x3dc));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001ed168).is_err() { return; }
                c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x438));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.cvt_s_w(0, 0);
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x3f0));
                c.mul_s(0, 0, 1);
                c.mul_s(21, 0, 2);
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ed198).is_err() { return; }
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x56d0));
                c.ldl(11, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.ldr(11, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0xaf));
                c.ldr(12, (c.g[SP][0] as u32).wrapping_add(0xa8));
                c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x47));
                c.sdr(11, (c.g[V1][0] as u32).wrapping_add(0x40));
                c.sdl(12, (c.g[V1][0] as u32).wrapping_add(0x4f));
                c.sdr(12, (c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.mul_s(0, 0, 3);
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.add_s(22, 1, 0);
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                pc = 0x001ed200; continue;
            }
            0x001ed200 => {
                c.cycles += 21;
                c.mov_s(13, 20);
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x35e)).lb(c);
                c.add_s(20, 20, 21);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.lwc1(12, (c.g[S0][0] as u32));
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                c.add_s(12, 22, 12);
                if callee::FONT_DRAW_TEXT.call(c, 0x001ed248).is_err() { return; }
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { if back_edge_unwinds(c, 0x001ed200) { return; } pc = 0x001ed200; continue; }
                pc = 0x001ed254; continue;
            }
            0x001ed254 => {
                c.cycles += 11;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 2);
                c.g[S1][0] = c.g[S7][0];
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = 0u64;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.g[S3][0] = 0xfffffffffffffc1f;
                c.mul_s(1, 1, 2);
                c.mul_s(0, 0, 2);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb4));
                pc = 0x001ed280; continue;
            }
            0x001ed280 => {
                c.cycles += 6;
                c.g[V0][0] = 0x18;
                c.mult(0, 3, 18, 2);
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                c.g[V1][0] = Addr((c.g[S0][0] as u32)).lw(c);
                let t = (c.g[V1][0] as i64) >= 0; 
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001ed37c; continue; }
                pc = 0x001ed298; continue;
            }
            0x001ed298 => {
                c.cycles += 57;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x31));
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x35e)).lb(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0xc0)).sb(c, c.g[A2][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0xc1)).sb(c, 0u64);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.mul_s(1, 1, 0);
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.mul_s(2, 2, 0);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                if callee::FONT_DRAW_TEXT.call(c, 0x001ed300).is_err() { return; }
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                if callee::FONT_MEASURE_TEXT.call(c, 0x001ed31c).is_err() { return; }
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.mul_s(1, 1, 3);
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x56cc));
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x38));
                c.add_s(12, 12, 3);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.add_s(12, 12, 0);
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x70)).lw(c);
                c.add_s(13, 13, 4);
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                if callee::FONT_DRAW_TEXT.call(c, 0x001ed374).is_err() { return; }
                let t = true; 
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                if t { pc = 0x001ed498; continue; }
                pc = 0x001ed37c; continue;
            }
            0x001ed37c => {
                c.cycles += 71;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x35e)).lb(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.mul_s(1, 1, 0);
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x38));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.mul_s(2, 2, 0);
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                if callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x001ed3c8).is_err() { return; }
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                if callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x001ed3d4).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S0][0] as u32)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                if callee::BE_GET_RIDER_DISPLAY_NAME.call(c, 0x001ed3e0).is_err() { return; }
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                if callee::STRCPY.call(c, 0x001ed3ec).is_err() { return; }
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x84));
                Addr((c.g[SP][0] as u32).wrapping_add(0xc3)).sb(c, 0u64);
                if callee::FONT_DRAW_TEXT.call(c, 0x001ed404).is_err() { return; }
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(3, (c.g[S6][0] as u32).wrapping_add(0x424));
                c.g[A0][0] = c.g[S6][0];
                c.sub_s(2, 22, 1);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                c.add_s(2, 2, 3);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x114));
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x79d8));
                c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T4][0] as u32));
                c.ldl(10, (c.g[T4][0] as u32).wrapping_add(0xf));
                c.ldr(10, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x127));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x12f));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x128));
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V0][0] = Addr((c.g[T5][0] as u32).wrapping_add(0x35e)).lb(c);
                c.g[V1][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] & 0x3e0;
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                Addr((c.g[T1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_RECT.call(c, 0x001ed494).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                pc = 0x001ed498; continue;
            }
            0x001ed498 => {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < 3i64) as u64;
                c.add_s(0, 0, 21);
                let t = c.g[V0][0] != 0u64; 
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                if t { if back_edge_unwinds(c, 0x001ed280) { return; } pc = 0x001ed280; continue; }
                pc = 0x001ed4ac; continue;
            }
            0x001ed4ac => {
                c.cycles += 2;
                let t = c.g[FP][0] == 0u64; 
                c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                if t { pc = 0x001ed5c8; continue; }
                pc = 0x001ed4b4; continue;
            }
            0x001ed4b4 => {
                c.cycles += 15;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc0)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x14b0));
                c.g[S1][0] = c.g[V0][0];
                c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
                if callee::SPRINTF.call(c, 0x001ed4d4).is_err() { return; }
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.mtc1(0, 0);
                c.lwc1(1, (c.g[T6][0] as u32).wrapping_add(0x78));
                c.c_lt_s(0, 1);
                let t = !c.fcc(); 
                c.g[T2][0] = 0u64;
                if t { pc = 0x001ed55c; continue; }
                pc = 0x001ed4f0; continue;
            }
            0x001ed4f0 => {
                c.cycles += 27;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56c8));
                c.g[V1][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x56c4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.mul_s(0, 1, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.mul_s(1, 1, 1);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x56c0));
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.mul_s(0, 0, 3);
                c.sub_s(1, 2, 1);
                c.cvt_w_s(3, 0);
                c.mfc1(2, 3);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 4);
                c.mul_s(0, 0, 1);
                c.add_s(0, 0, 2);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xc7));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xb7));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                pc = 0x001ed55c; continue;
            }
            0x001ed55c => {
                c.cycles += 27;
                c.g[V0][0] = 0x4d0000;
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x78f8));
                c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[T1][0] as u32));
                c.ldl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0xc7));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xcf));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[T0][0] = c.g[S1][0];
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x74)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[A3][0]);
                c.g[T3][0] = 0u64;
                if c.g[V0][0] == 0 { c.g[V1][0] = 0u64; }
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[A3][0]);
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A3][0] = 0x17;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ed5c8).is_err() { return; }
                pc = 0x001ed5c8; continue;
            }
            0x001ed5c8 => {
                c.cycles += 6;
                c.g[V0][0] = 0x8000000;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.g[V1][0] = c.g[T4][0] & c.g[V0][0];
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ed5e0; continue;
            }
            0x001ed5e0 => {
                c.cycles += 7;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x7c)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x48)).lw(c); pc = 0x001ed620; continue; }
                pc = 0x001ed5fc; continue;
            }
            0x001ed5fc => {
                c.cycles += 9;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[V0][0] = 0x1;
                c.g[A1][0] = 0u64;
                c.g[V1][0] = Addr((c.g[T5][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                let t = true; 
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
                if t { pc = 0x001ed624; continue; }
                pc = 0x001ed620; continue;
            }
            0x001ed620 => {
                c.cycles += 1;
                c.g[A1][0] = 0u64;
                pc = 0x001ed624; continue;
            }
            0x001ed624 => {
                c.cycles += 2;
                
                if c.g[V0][0] != 0u64 { c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c); pc = 0x001ed62c; continue; }
                pc = 0x001ed62c; continue;
            }
            0x001ed62c => {
                c.cycles += 19;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                c.g[A2][0] = 0x4d0000;
                c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x790)).lw(c);
                c.g[V0][0] = Addr((c.g[T6][0] as u32).wrapping_add(0x790)).lw(c);
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x198)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x198)).lw(c);
                c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7738));
                c.ldl(2, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[T0][0] as u32));
                c.ldl(4, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = ((c.g[S2][0] as i64) < -5000i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ed6a4; continue; }
                pc = 0x001ed678; continue;
            }
            0x001ed678 => {
                c.cycles += 11;
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7778));
                c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T4][0] as u32));
                c.ldl(10, (c.g[T4][0] as u32).wrapping_add(0xf));
                c.ldr(10, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x7f));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x78));
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                if t { pc = 0x001ed6d8; continue; }
                pc = 0x001ed6a4; continue;
            }
            0x001ed6a4 => {
                c.cycles += 2;
                
                if (c.g[S2][0] as i64) > 0 { c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c); pc = 0x001ed6d8; continue; }
                pc = 0x001ed6ac; continue;
            }
            0x001ed6ac => {
                c.cycles += 11;
                c.g[V0][0] = 0x4d0000;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7758));
                c.ldl(13, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.ldr(13, (c.g[A0][0] as u32));
                c.ldl(14, (c.g[A0][0] as u32).wrapping_add(0xf));
                c.ldr(14, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x7f));
                c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                pc = 0x001ed6d8; continue;
            }
            0x001ed6d8 => {
                c.cycles += 18;
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = 0x7f;
                c.g[T0][0] = 0x470000;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x14a0));
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0x4a;
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V1][0]);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.g[S1][0] = c.g[V1][0];
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ed718).is_err() { return; }
                let t = (c.g[S2][0] as i64) < 0; 
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x001ed740; continue; }
                pc = 0x001ed720; continue;
            }
            0x001ed720 => {
                c.cycles += 8;
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2298));
                c.g[A2][0] = c.g[S2][0];
                c.g[A0][0] = c.g[V0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[V0][0]);
                if callee::SPRINTF.call(c, 0x001ed738).is_err() { return; }
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                if t { pc = 0x001ed760; continue; }
                pc = 0x001ed740; continue;
            }
            0x001ed740 => {
                c.cycles += 8;
                c.g[A2][0] = 0x4a0000;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2138));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                if callee::SPRINTF.call(c, 0x001ed758).is_err() { return; }
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[S0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                pc = 0x001ed760; continue;
            }
            0x001ed760 => {
                c.cycles += 12;
                c.g[V0][0] = 0x7f;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x368)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = 0x4b;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S1][0]);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                if t { pc = 0x001ef59c; continue; }
                pc = 0x001ed790; continue;
            }
            0x001ed790 => {
                c.cycles += 6;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x200000;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ed7a8; continue;
            }
            0x001ed7a8 => {
                c.cycles += 3;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                
                if c.g[T1][0] == 0u64 { c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c); pc = 0x001efac4; continue; }
                pc = 0x001ed7b4; continue;
            }
            0x001ed7b4 => {
                c.cycles += 10;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x50)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x54)).sw(c, 0u64);
                c.mtc1(0, 20);
                c.ldl(10, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(10, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(10, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(10, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = (c.g[T1][0] as i64) <= 0; 
                c.g[A0][0] = 0u64;
                if t { pc = 0x001ed8d0; continue; }
                pc = 0x001ed7dc; continue;
            }
            0x001ed7dc => {
                c.cycles += 9;
                c.mov_s(0, 20);
                c.ldl(14, (c.g[SP][0] as u32).wrapping_add(0x37));
                c.ldr(14, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.c_lt_s(22, 0);
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x56bc));
                let t = c.fcc(); 
                c.mov_s(23, 20);
                if t { pc = 0x001ed8d0; continue; }
                pc = 0x001ed800; continue;
            }
            0x001ed800 => {
                c.cycles += 4;
                c.lwc1(24, (c.g[GP][0] as u32).wrapping_sub(0x56b8));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 21);
                pc = 0x001ed810; continue;
            }
            0x001ed810 => {
                c.cycles += 4;
                c.c_lt_s(22, 20);
                let t = !c.fcc(); 
                c.g[V0][0] = 0u64;
                if t { pc = 0x001ed854; continue; }
                pc = 0x001ed820; continue;
            }
            0x001ed820 => {
                c.cycles += 4;
                c.c_le_s(0, 22);
                let t = !c.fcc(); 
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ed854; continue; }
                pc = 0x001ed830; continue;
            }
            0x001ed830 => {
                c.cycles += 9;
                c.sub_s(0, 20, 22);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[V0][0] = 0x2;
                c.mul_s(0, 0, 24);
                c.add_s(0, 0, 21);
                c.mul_s(1, 1, 0);
                c.mul_s(2, 2, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                pc = 0x001ed854; continue;
            }
            0x001ed854 => {
                c.cycles += 23;
                c.g[T0][0] = c.g[A0][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).sw(c, c.g[A0][0]);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = 0x2;
                if callee::HUD_DRAW_POSITION_PANEL.call(c, 0x001ed894).is_err() { return; }
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).lw(c);
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let t = c.g[V1][0] == 0u64; 
                c.mov_s(0, 20);
                if t { pc = 0x001ed8d0; continue; }
                pc = 0x001ed8b0; continue;
            }
            0x001ed8b0 => {
                c.cycles += 8;
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x37));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.c_lt_s(22, 0);
                let t = !c.fcc(); 
                c.add_s(20, 0, 23);
                if t { if back_edge_unwinds(c, 0x001ed810) { return; } pc = 0x001ed810; continue; }
                pc = 0x001ed8d0; continue;
            }
            0x001ed8d0 => {
                c.cycles += 11;
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.ldl(10, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(10, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(10, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(10, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ed8fc; continue;
            }
            0x001ed8fc => {
                c.cycles += 5;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[V1][0] & 0x4000;
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ed910; continue;
            }
            0x001ed910 => {
                c.cycles += 10;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A3][0] = 0x1d;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T2][0] = 0u64;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                if callee::HUD_DRAW_WRAPPED_TEXT.call(c, 0x001ed930).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ed938; continue;
            }
            0x001ed938 => {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x1;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x320)).lw(c);
                let t = c.g[A1][0] != c.g[V0][0]; 
                c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ed94c; continue;
            }
            0x001ed94c => {
                c.cycles += 5;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x7f;
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ed960; continue;
            }
            0x001ed960 => {
                c.cycles += 12;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = 0x46;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                if t { pc = 0x001ef59c; continue; }
                pc = 0x001ed990; continue;
            }
            0x001ed990 => {
                c.cycles += 5;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = c.g[A3][0] & 0x4;
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ed9a4; continue;
            }
            0x001ed9a4 => {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3c4)).lw(c);
                c.g[V0][0] = 0x50;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ed9b4; continue;
            }
            0x001ed9b4 => {
                c.cycles += 8;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.c_lt_s(22, 0);
                let t = !c.fcc(); 
                c.g[V1][0] = 0x500000;
                if t { pc = 0x001eda1c; continue; }
                pc = 0x001ed9d4; continue;
            }
            0x001ed9d4 => {
                c.cycles += 18;
                c.add_s(1, 22, 22);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56b4));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x56b0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x56ac));
                c.mul_s(0, 1, 0);
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(20, 0, 20);
                pc = 0x001eda1c; continue;
            }
            0x001eda1c => {
                c.cycles += 16;
                c.g[V0][0] = 0x4d0000;
                c.g[A0][0] = 0x41;
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7818));
                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A3][0] as u32));
                c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = 0x2a;
                c.g[V0][0] = Addr((c.g[S4][0] as u32)).lw(c);
                c.g[S5][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4b0)).lw(c);
                let t = c.g[V0][0] != c.g[V1][0]; 
                c.g[S4][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                if t { pc = 0x001edab8; continue; }
                pc = 0x001eda5c; continue;
            }
            0x001eda5c => {
                c.cycles += 13;
                c.g[V0][0] = 0x4d0000;
                c.g[S5][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4ac)).lw(c);
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7738));
                c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
                c.ldr(8, (c.g[T3][0] as u32));
                c.ldl(9, (c.g[T3][0] as u32).wrapping_add(0xf));
                c.ldr(9, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = (c.g[S4][0] as i64) >= 0; 
                c.g[A0][0] = 0x40;
                if t { pc = 0x001edab8; continue; }
                pc = 0x001eda90; continue;
            }
            0x001eda90 => {
                c.cycles += 10;
                c.g[V0][0] = 0x4d0000;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7778));
                c.ldl(12, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(12, (c.g[V1][0] as u32));
                c.ldl(13, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(13, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x58));
                pc = 0x001edab8; continue;
            }
            0x001edab8 => {
                c.cycles += 62;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = 0x24;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.lo[0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.madd(0, 16, 4, 16);
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001edad8).is_err() { return; }
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001edaec).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001edaf8).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.mul_s(0, 0, 20);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.mul_s(1, 1, 20);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.mul_s(2, 2, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.mul_s(3, 3, 1);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.g[A3][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S6][0];
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = c.g[S2][0];
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[S7][0] = c.g[S2][0];
                c.g[T2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x21)).lb(c);
                c.g[T1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lb(c);
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001edb54).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001edb64).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T0][0] = 0xfffffffffffffc1f;
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x22)).lb(c);
                c.g[T1][0] = c.g[S1][0];
                c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A1][0] = c.g[S5][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[A3][0] = c.g[S3][0];
                c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                Addr((c.g[T2][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001edba8).is_err() { return; }
                let t = (c.g[S4][0] as i64) >= 0; 
                c.g[T1][0] = 0x4;
                if t { pc = 0x001edbb8; continue; }
                pc = 0x001edbb0; continue;
            }
            0x001edbb0 => {
                c.cycles += 2;
                c.g[S4][0] = sx(0u32.wrapping_sub((c.g[S4][0] as u32)));
                c.g[T1][0] = 0x2;
                pc = 0x001edbb8; continue;
            }
            0x001edbb8 => {
                c.cycles += 5;
                c.g[V0][0] = 0xe10;
                c.g[A2][0] = 0x3c;
                c.div(0, 20, 2);
                
                if c.g[V0][0] == 0u64 {  pc = 0x001edbcc; continue; }
                pc = 0x001edbcc; continue;
            }
            0x001edbcc => {
                c.cycles += 40;
                c.g[V1][0] = 0xe10;
                c.g[V0][0] = 0x3c;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x56a8));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = 0xffffffffffffffff;
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T2][0] = c.g[S7][0];
                c.mul_s(1, 1, 0);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S6][0];
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.g[T0][0] = 0u64;
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.add_s(3, 3, 1);
                c.mul_s(0, 0, 6);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.add_s(4, 4, 2);
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.mul_s(5, 5, 6);
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V1][0]);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[A1][0] = c.lo[0];
                c.g[T4][0] = c.hi[0];
                c.div(0, 12, 6);
                c.g[A2][0] = c.lo[0];
                c.g[A3][0] = c.hi[0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, 0u64);
                if callee::HUD_DRAW_TIME.call(c, 0x001edc64).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001edc6c; continue;
            }
            0x001edc6c => {
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = c.g[V1][0] & 0x2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001edc80; continue;
            }
            0x001edc80 => {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3c4)).lw(c);
                c.g[V0][0] = 0x50;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001edc90; continue;
            }
            0x001edc90 => {
                c.cycles += 8;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 21);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.c_lt_s(22, 21);
                let t = !c.fcc(); 
                c.g[V1][0] = 0x500000;
                if t { pc = 0x001edcf8; continue; }
                pc = 0x001edcb0; continue;
            }
            0x001edcb0 => {
                c.cycles += 18;
                c.add_s(1, 22, 22);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x56a4));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x56a0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x569c));
                c.mul_s(0, 1, 0);
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(20, 0, 20);
                pc = 0x001edcf8; continue;
            }
            0x001edcf8 => {
                c.cycles += 87;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x900));
                c.g[A0][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001edd10).is_err() { return; }
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4ac)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001edd24).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001edd30).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.mul_s(0, 0, 20);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.mul_s(1, 1, 20);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.mul_s(2, 2, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.mul_s(3, 3, 1);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A3][0] = c.g[S3][0];
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A0][0] = c.g[S6][0];
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[A1][0] = c.g[S2][0];
                c.g[T2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x921)).lb(c);
                c.g[T1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x920)).lb(c);
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001edd8c).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001edd98).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A3][0] = c.g[S3][0];
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x922)).lb(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[A0][0] = c.g[S6][0];
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4ac)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001edde0).is_err() { return; }
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(1, 1, 21);
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.add_s(4, 4, 1);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5698));
                c.add_s(5, 5, 3);
                c.mul_s(2, 2, 0);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(6, 6, 0);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7738));
                c.ldl(5, (c.g[T1][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[T1][0] as u32));
                c.ldl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xc7));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xcf));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0xc8));
                let t = (c.g[A2][0] as i64) < 0; 
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x001ede94; continue; }
                pc = 0x001ede54; continue;
            }
            0x001ede54 => {
                c.cycles += 16;
                c.g[V0][0] = 0x4d0000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7778));
                c.ldl(10, (c.g[T5][0] as u32).wrapping_add(0x7));
                c.ldr(10, (c.g[T5][0] as u32));
                c.ldl(11, (c.g[T5][0] as u32).wrapping_add(0xf));
                c.ldr(11, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0xc7));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0xcf));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2298));
                if callee::SPRINTF.call(c, 0x001ede8c).is_err() { return; }
                let t = true; 
                c.g[A1][0] = c.g[S0][0];
                if t { pc = 0x001edea8; continue; }
                pc = 0x001ede94; continue;
            }
            0x001ede94 => {
                c.cycles += 5;
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                c.g[A0][0] = c.g[S0][0];
                if callee::SPRINTF.call(c, 0x001edea4).is_err() { return; }
                c.g[A1][0] = c.g[S0][0];
                pc = 0x001edea8; continue;
            }
            0x001edea8 => {
                c.cycles += 34;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.mul_s(0, 0, 1);
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.g[S5][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xc7));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xcf));
                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x47));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x40));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4f));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x54));
                if callee::FONT_DRAW_TEXT_CENTERED.call(c, 0x001edf08).is_err() { return; }
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.mul_s(2, 2, 1);
                let t = true; 
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001edf30; continue;
            }
            0x001edf30 => {
                c.cycles += 6;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x100000;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001edf48; continue;
            }
            0x001edf48 => {
                c.cycles += 5;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[V0][0] = 0x6;
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x88)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001edf5c; continue;
            }
            0x001edf5c => {
                c.cycles += 49;
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.g[S2][0];
                c.g[S0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1b0));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001edf74).is_err() { return; }
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x498)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S4][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001edf88).is_err() { return; }
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001edf98).is_err() { return; }
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[T1][0] = Addr((c.g[T4][0] as u32).wrapping_add(0x1d0)).lb(c);
                c.g[A3][0] = c.g[S4][0];
                c.g[T2][0] = Addr((c.g[T4][0] as u32).wrapping_add(0x1d1)).lb(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001edfc4).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001edfd0).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A3][0] = c.g[S4][0];
                c.g[V1][0] = Addr((c.g[T5][0] as u32).wrapping_add(0x1d2)).lb(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x498)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001ee018).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee020; continue;
            }
            0x001ee020 => {
                c.cycles += 6;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x100000;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee038; continue;
            }
            0x001ee038 => {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[V0][0] = 0x4;
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x88)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee04c; continue;
            }
            0x001ee04c => {
                c.cycles += 49;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xaf8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001ee064).is_err() { return; }
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4f8)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S4][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001ee078).is_err() { return; }
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001ee088).is_err() { return; }
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A3][0] = c.g[S4][0];
                c.g[T1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xb18)).lb(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[T2][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xb19)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001ee0b4).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ee0c0).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xb1a)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A3][0] = c.g[S4][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4f8)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001ee108).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee110; continue;
            }
            0x001ee110 => {
                c.cycles += 6;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x100000;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee128; continue;
            }
            0x001ee128 => {
                c.cycles += 5;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[V0][0] = 0x5;
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x88)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee13c; continue;
            }
            0x001ee13c => {
                c.cycles += 49;
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.g[S2][0];
                c.g[S0][0] = sx((c.g[T3][0] as u32).wrapping_add(0xab0));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001ee154).is_err() { return; }
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4f4)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S4][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001ee168).is_err() { return; }
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001ee178).is_err() { return; }
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[T1][0] = Addr((c.g[T4][0] as u32).wrapping_add(0xad0)).lb(c);
                c.g[A3][0] = c.g[S4][0];
                c.g[T2][0] = Addr((c.g[T4][0] as u32).wrapping_add(0xad1)).lb(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001ee1a4).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ee1b0).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A3][0] = c.g[S4][0];
                c.g[V1][0] = Addr((c.g[T5][0] as u32).wrapping_add(0xad2)).lb(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4f4)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001ee1f8).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee200; continue;
            }
            0x001ee200 => {
                c.cycles += 6;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee218; continue;
            }
            0x001ee218 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x200;
                let t = c.g[V0][0] != 0u64; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee224; continue;
            }
            0x001ee224 => {
                c.cycles += 18;
                c.g[V0][0] = Addr((c.g[S4][0] as u32)).lw(c);
                c.g[A3][0] = 0x15;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5694));
                c.g[V1][0] = 0x16;
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = c.g[V0][0] ^ 0x24;
                c.c_le_s(0, 22);
                c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x50)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x54)).sw(c, 0u64);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x2f));
                c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x28));
                let t = !c.fcc(); 
                if c.g[A1][0] != 0 { c.g[A3][0] = c.g[V1][0]; }
                if t { pc = 0x001ee2b8; continue; }
                pc = 0x001ee26c; continue;
            }
            0x001ee26c => {
                c.cycles += 19;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T0][0] = 0x11;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.mov_s(12, 22);
                Addr((c.g[SP][0] as u32).wrapping_add(0x20)).sw(c, c.g[V1][0]);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_TEXT_LERP.call(c, 0x001ee2ac).is_err() { return; }
                c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                let t = true; 
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t { pc = 0x001ee3b0; continue; }
                pc = 0x001ee2b8; continue;
            }
            0x001ee2b8 => {
                c.cycles += 32;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5690));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x568c));
                c.g[V1][0] = 0x500000;
                c.mul_s(1, 22, 1);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5688));
                c.g[T6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ad8));
                c.ldl(11, (c.g[T6][0] as u32).wrapping_add(0x7));
                c.ldr(11, (c.g[T6][0] as u32));
                c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5684));
                c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.mul_s(0, 1, 0);
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                let t = c.g[A1][0] == 0u64; 
                c.add_s(20, 0, 20);
                if t { pc = 0x001ee360; continue; }
                pc = 0x001ee338; continue;
            }
            0x001ee338 => {
                c.cycles += 10;
                c.g[V0][0] = 0x4d0000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ab8));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A2][0] as u32));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x68));
                pc = 0x001ee360; continue;
            }
            0x001ee360 => {
                c.cycles += 20;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = 0x7f;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = 0x11;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.mov_s(12, 22);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.g[T3][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x20)).sw(c, c.g[V1][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[S0][0];
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x74));
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[S7][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_TEXT_LERP.call(c, 0x001ee3b0).is_err() { return; }
                pc = 0x001ee3b0; continue;
            }
            0x001ee3b0 => {
                c.cycles += 7;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.c_lt_s(12, 22);
                let t = !c.fcc(); 
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t { pc = 0x001ee3d8; continue; }
                pc = 0x001ee3cc; continue;
            }
            0x001ee3cc => {
                c.cycles += 3;
                c.mtc1(0, 12);
                let t = true; 
                
                if t { pc = 0x001ee3ec; continue; }
                pc = 0x001ee3d8; continue;
            }
            0x001ee3d8 => {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(0, 22);
                
                if c.fcc() { c.sub_s(12, 12, 22); pc = 0x001ee3ec; continue; }
                pc = 0x001ee3ec; continue;
            }
            0x001ee3ec => {
                c.cycles += 32;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.g[T0][0] = c.g[S1][0];
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[T1][0] = c.g[S7][0];
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x5c));
                c.g[A0][0] = c.g[S6][0];
                c.mul_s(4, 4, 0);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(2, 2, 0);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.add_s(3, 3, 4);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.add_s(1, 1, 2);
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[S5][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                if callee::HUD_DRAW_PIP_METER.call(c, 0x001ee440).is_err() { return; }
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.ldl(11, (c.g[SP][0] as u32).wrapping_add(0x87));
                c.ldr(11, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sdl(11, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(11, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee46c; continue;
            }
            0x001ee46c => {
                c.cycles += 6;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee484; continue;
            }
            0x001ee484 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0x200;
                let t = c.g[V0][0] != 0u64; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ee490; continue;
            }
            0x001ee490 => {
                c.cycles += 20;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[S2][0] = c.g[V0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x50)).sw(c, 0u64);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48));
                Addr((c.g[SP][0] as u32).wrapping_add(0x54)).sw(c, 0u64);
                c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5680));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x2f));
                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[A1][0] = c.g[V0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ee4c8).is_err() { return; }
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.c_lt_s(22, 21);
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ee554; continue; }
                pc = 0x001ee4e0; continue;
            }
            0x001ee4e0 => {
                c.cycles += 29;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x567c));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5678));
                c.g[V1][0] = 0x500000;
                c.mul_s(1, 22, 1);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5674));
                c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7898));
                c.ldl(10, (c.g[T5][0] as u32).wrapping_add(0x7));
                c.ldr(10, (c.g[T5][0] as u32));
                c.ldl(11, (c.g[T5][0] as u32).wrapping_add(0xf));
                c.ldr(11, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5670));
                c.mul_s(0, 1, 0);
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(20, 0, 20);
                pc = 0x001ee554; continue;
            }
            0x001ee554 => {
                c.cycles += 32;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A1][0] = 0x4a0000;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x22a0));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = c.g[S0][0];
                c.g[S3][0] = 0x7f;
                if callee::SPRINTF.call(c, 0x001ee578).is_err() { return; }
                c.g[S1][0] = c.g[S0][0];
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A3][0] = 0x2;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[S2][0]);
                c.g[T0][0] = 0x11;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                c.mov_s(12, 22);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S3][0]);
                c.g[T1][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x20)).sw(c, c.g[S3][0]);
                c.g[T3][0] = c.g[V0][0];
                c.g[S7][0] = c.g[V0][0];
                if callee::HUD_DRAW_TEXT_LERP.call(c, 0x001ee5c4).is_err() { return; }
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x566c));
                c.c_lt_s(22, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x001ee660; continue; }
                pc = 0x001ee5d4; continue;
            }
            0x001ee5d4 => {
                c.cycles += 4;
                c.c_lt_s(21, 22);
                let t = !c.fcc(); 
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ee604; continue; }
                pc = 0x001ee5e4; continue;
            }
            0x001ee5e4 => {
                c.cycles += 8;
                c.sub_s(2, 22, 21);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5668));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.mul_s(2, 2, 3);
                c.sub_s(0, 0, 2);
                c.mul_s(1, 1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                pc = 0x001ee604; continue;
            }
            0x001ee604 => {
                c.cycles += 23;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1490));
                c.neg_s(0, 0);
                c.g[A0][0] = c.g[S1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0xa0)).sw(c, 0u64);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa4));
                if callee::SPRINTF.call(c, 0x001ee628).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[A2][0]);
                c.g[T0][0] = c.g[S1][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T2][0] = c.g[S7][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S2][0]);
                c.g[A3][0] = 0x2;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S3][0]);
                c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S3][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ee660).is_err() { return; }
                pc = 0x001ee660; continue;
            }
            0x001ee660 => {
                c.cycles += 12;
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = true; 
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee690; continue;
            }
            0x001ee690 => {
                c.cycles += 6;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T5][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee6a8; continue;
            }
            0x001ee6a8 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[T5][0] & 0x200;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee6b4; continue;
            }
            0x001ee6b4 => {
                c.cycles += 19;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x50)).sw(c, 0u64);
                c.g[S2][0] = c.g[V0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x54)).sw(c, 0u64);
                c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5664));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x2f));
                c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_add(0x48));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ee6e8).is_err() { return; }
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.c_lt_s(22, 21);
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ee774; continue; }
                pc = 0x001ee700; continue;
            }
            0x001ee700 => {
                c.cycles += 29;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5660));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x565c));
                c.g[V1][0] = 0x500000;
                c.mul_s(1, 22, 1);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5658));
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7898));
                c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T4][0] as u32));
                c.ldl(10, (c.g[T4][0] as u32).wrapping_add(0xf));
                c.ldr(10, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5654));
                c.mul_s(0, 1, 0);
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(20, 0, 20);
                pc = 0x001ee774; continue;
            }
            0x001ee774 => {
                c.cycles += 27;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.g[S1][0] = 0x7f;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                if callee::FE_FORMAT_MONEY.call(c, 0x001ee788).is_err() { return; }
                c.g[T1][0] = c.g[V0][0];
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A3][0] = 0x2;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[T0][0] = 0x11;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[S2][0]);
                c.mov_s(12, 22);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.g[T3][0] = c.g[V1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S1][0]);
                c.g[S7][0] = c.g[V1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x20)).sw(c, c.g[S1][0]);
                if callee::HUD_DRAW_TEXT_LERP.call(c, 0x001ee7d0).is_err() { return; }
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5650));
                c.c_lt_s(22, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x001ee870; continue; }
                pc = 0x001ee7e0; continue;
            }
            0x001ee7e0 => {
                c.cycles += 4;
                c.c_lt_s(21, 22);
                let t = !c.fcc(); 
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ee810; continue; }
                pc = 0x001ee7f0; continue;
            }
            0x001ee7f0 => {
                c.cycles += 8;
                c.sub_s(2, 22, 21);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x564c));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.mul_s(2, 2, 3);
                c.sub_s(0, 0, 2);
                c.mul_s(1, 1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                pc = 0x001ee810; continue;
            }
            0x001ee810 => {
                c.cycles += 24;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.g[A1][0] = 0x470000;
                c.neg_s(0, 0);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1490));
                c.g[A0][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0xa0)).sw(c, 0u64);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa4));
                if callee::SPRINTF.call(c, 0x001ee838).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[S0][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T2][0] = c.g[S7][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S2][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S1][0]);
                c.g[A3][0] = 0x2;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
                c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S1][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ee870).is_err() { return; }
                pc = 0x001ee870; continue;
            }
            0x001ee870 => {
                c.cycles += 12;
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = true; 
                c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee8a0; continue;
            }
            0x001ee8a0 => {
                c.cycles += 6;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T2][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ee8b8; continue;
            }
            0x001ee8b8 => {
                c.cycles += 21;
                c.g[V0][0] = 0x4d0000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.g[T6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x78d8));
                c.ldl(11, (c.g[T6][0] as u32).wrapping_add(0x7));
                c.ldr(11, (c.g[T6][0] as u32));
                c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x58));
                Addr((c.g[SP][0] as u32).wrapping_add(0x364)).sw(c, 0u64);
                c.mov_s(20, 2);
                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.div_s(1, 0, 1);
                c.c_lt_s(1, 20);
                let t = !c.fcc(); 
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ee958; continue; }
                pc = 0x001ee90c; continue;
            }
            0x001ee90c => {
                c.cycles += 19;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5648));
                c.g[V1][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5644));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.mul_s(0, 1, 0);
                Addr((c.g[SP][0] as u32).wrapping_add(0x364)).sw(c, c.g[S2][0]);
                c.mul_s(1, 1, 1);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5640));
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(2, 0, 20);
                pc = 0x001ee958; continue;
            }
            0x001ee958 => {
                c.cycles += 11;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0xe4c));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0xe48));
                c.mul_s(0, 2, 0);
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.mul_s(1, 2, 1);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                let t = c.g[V0][0] != 0u64; 
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                if t { pc = 0x001eeba0; continue; }
                pc = 0x001ee984; continue;
            }
            0x001ee984 => {
                c.cycles += 58;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = c.g[V1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x22b0));
                if callee::SPRINTF.call(c, 0x001ee998).is_err() { return; }
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T2][0] = 0x1;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x364)).lw(c);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T4][0] = c.g[A2][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0xe38));
                c.g[S3][0] = c.g[A0][0] | 0x2;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0xe3c));
                c.g[S0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1d4));
                Addr((c.g[SP][0] as u32)).sw(c, c.g[T0][0]);
                c.g[T3][0] = c.g[S2][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[T2][0]);
                c.g[A3][0] = 0xd;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.g[T1][0] = c.g[S3][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[S1][0]);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, 0u64);
                c.g[A0][0] = c.g[S6][0];
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ee9fc).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8c));
                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[S1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[A1][0] = c.g[S1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x36c)).sw(c, c.g[S2][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.g[S2][0] = 0xffffffffffffffff;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                if callee::HUD_GET_ELEM_POS.call(c, 0x001eea30).is_err() { return; }
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[S7][0] = c.g[V0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001eea44).is_err() { return; }
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0xe34));
                c.g[A2][0] = c.g[S1][0];
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x88));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sub_s(0, 0, 1);
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0xb4)).sw(c, 0u64);
                c.add_s(2, 2, 0);
                let t = c.g[T5][0] == 0u64; 
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                if t { pc = 0x001eea80; continue; }
                pc = 0x001eea6c; continue;
            }
            0x001eea6c => {
                c.cycles += 5;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[AT][0] = 0x44200000;
                c.mtc1(1, 0);
                c.sub_s(0, 0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                pc = 0x001eea80; continue;
            }
            0x001eea80 => {
                c.cycles += 46;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[A1][0] = c.g[S4][0];
                c.add_s(0, 0, 2);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = c.g[FP][0];
                c.g[T1][0] = 0x1;
                c.g[T2][0] = 0u64;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[T3][0] = 0u64;
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001eeab4).is_err() { return; }
                c.g[S0][0] = 0xfffffffffffffc1f;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S4][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A3][0] = c.g[S5][0];
                c.g[V1][0] = Addr((c.g[T6][0] as u32).wrapping_add(0x1f6)).lb(c);
                c.g[T0][0] = c.g[FP][0];
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[T1][0] = c.g[S7][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4b8)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001eeafc).is_err() { return; }
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = 0x1;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1f6)).lb(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x360)).lw(c);
                let t = c.g[A1][0] == c.g[A2][0]; 
                c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
                if t { pc = 0x001eeb74; continue; }
                pc = 0x001eeb38; continue;
            }
            0x001eeb38 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x360)).lw(c);
                if t { pc = 0x001eeb50; continue; }
                pc = 0x001eeb40; continue;
            }
            0x001eeb40 => {
                c.cycles += 2;
                let t = c.g[A1][0] == 0u64; 
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                if t { pc = 0x001eeb7c; continue; }
                pc = 0x001eeb48; continue;
            }
            0x001eeb48 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = 0x4a0000;
                if t { pc = 0x001eebc4; continue; }
                pc = 0x001eeb50; continue;
            }
            0x001eeb50 => {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                let t = c.g[A3][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x3;
                if t { pc = 0x001eeb6c; continue; }
                pc = 0x001eeb5c; continue;
            }
            0x001eeb5c => {
                c.cycles += 2;
                let t = c.g[A3][0] != c.g[V0][0]; 
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                if t { pc = 0x001eebc0; continue; }
                pc = 0x001eeb64; continue;
            }
            0x001eeb64 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4bc)).lw(c);
                if t { pc = 0x001eeb80; continue; }
                pc = 0x001eeb6c; continue;
            }
            0x001eeb6c => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4c0)).lw(c);
                if t { pc = 0x001eeb80; continue; }
                pc = 0x001eeb74; continue;
            }
            0x001eeb74 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4c4)).lw(c);
                if t { pc = 0x001eeb80; continue; }
                pc = 0x001eeb7c; continue;
            }
            0x001eeb7c => {
                c.cycles += 1;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4c8)).lw(c);
                pc = 0x001eeb80; continue;
            }
            0x001eeb80 => {
                c.cycles += 8;
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = c.g[FP][0];
                c.g[T1][0] = c.g[S7][0];
                c.g[A0][0] = c.g[S6][0];
                if callee::HUD_DRAW_SPRITE.call(c, 0x001eeb98).is_err() { return; }
                let t = true; 
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                if t { pc = 0x001eebc0; continue; }
                pc = 0x001eeba0; continue;
            }
            0x001eeba0 => {
                c.cycles += 8;
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[S2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[T0][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x36c)).sw(c, c.g[T1][0]);
                c.g[S3][0] = c.g[T2][0] | 0x2;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                pc = 0x001eebc0; continue;
            }
            0x001eebc0 => {
                c.cycles += 1;
                c.g[A1][0] = 0x4a0000;
                pc = 0x001eebc4; continue;
            }
            0x001eebc4 => {
                c.cycles += 49;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x22c0));
                if callee::SPRINTF.call(c, 0x001eebd0).is_err() { return; }
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[S0][0] = sx((c.g[S3][0] as u32) << 24);
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x364)).lw(c);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x368)).lw(c);
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T6][0] = 0xffffffffffffffff;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S5][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[T4][0]);
                c.g[A3][0] = 0xd;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[T5][0]);
                c.g[T1][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S2][0]);
                c.g[T2][0] = c.g[S1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[T6][0]);
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001eec24).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = 0xffffffffffffffff;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T0][0] = 0x4a0000;
                c.neg_s(0, 0);
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x364)).lw(c);
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x368)).lw(c);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x22c8));
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[A0][0]);
                c.g[T1][0] = c.g[S0][0];
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x36c)).lw(c);
                c.g[T2][0] = c.g[S1][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0xd;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S2][0]);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb4));
                Addr((c.g[SP][0] as u32).wrapping_add(0xb0)).sw(c, 0u64);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001eec78).is_err() { return; }
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.g[V0][0] = 0x1;
                let t = c.g[A1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001eec88; continue;
            }
            0x001eec88 => {
                c.cycles += 34;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x563c));
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8c));
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.mul_s(1, 1, 2);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1d4));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.g[A0][0] = c.g[S0][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
                if callee::HUD_GET_ELEM_POS.call(c, 0x001eecc8).is_err() { return; }
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[S7][0] = c.g[V0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001eecdc).is_err() { return; }
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x88));
                c.g[A2][0] = c.g[S1][0];
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 1);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0xe20));
                c.mul_s(2, 2, 1);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0xb4)).sw(c, 0u64);
                c.sub_s(0, 0, 2);
                c.add_s(3, 3, 0);
                let t = c.g[A3][0] == 0u64; 
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                if t { pc = 0x001eed24; continue; }
                pc = 0x001eed10; continue;
            }
            0x001eed10 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[AT][0] = 0x44200000;
                c.mtc1(1, 0);
                c.sub_s(0, 0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                pc = 0x001eed24; continue;
            }
            0x001eed24 => {
                c.cycles += 46;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[A1][0] = c.g[S2][0];
                c.add_s(0, 0, 3);
                c.g[T0][0] = c.g[S4][0];
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = c.g[S3][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[T1][0] = 0x1;
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001eed58).is_err() { return; }
                c.g[S0][0] = 0xfffffffffffffc1f;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T1][0] = c.g[S7][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x1f6)).lb(c);
                c.g[A3][0] = c.g[S3][0];
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[T0][0] = c.g[S4][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4b8)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001eeda0).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x1f6)).lb(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x360)).lw(c);
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                let t = c.g[T2][0] == c.g[T3][0]; 
                c.g[V0][0] = ((c.g[T2][0] as i64) < 2i64) as u64;
                if t { pc = 0x001eee18; continue; }
                pc = 0x001eeddc; continue;
            }
            0x001eeddc => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x360)).lw(c);
                if t { pc = 0x001eedf4; continue; }
                pc = 0x001eede4; continue;
            }
            0x001eede4 => {
                c.cycles += 2;
                let t = c.g[T2][0] == 0u64; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001eee20; continue; }
                pc = 0x001eedec; continue;
            }
            0x001eedec => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x1b4)).lw(c);
                if t { pc = 0x001efac8; continue; }
                pc = 0x001eedf4; continue;
            }
            0x001eedf4 => {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                let t = c.g[T4][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x3;
                if t { pc = 0x001eee10; continue; }
                pc = 0x001eee00; continue;
            }
            0x001eee00 => {
                c.cycles += 2;
                let t = c.g[T4][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001eee08; continue;
            }
            0x001eee08 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4bc)).lw(c);
                if t { pc = 0x001eee24; continue; }
                pc = 0x001eee10; continue;
            }
            0x001eee10 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4c0)).lw(c);
                if t { pc = 0x001eee24; continue; }
                pc = 0x001eee18; continue;
            }
            0x001eee18 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4c4)).lw(c);
                if t { pc = 0x001eee24; continue; }
                pc = 0x001eee20; continue;
            }
            0x001eee20 => {
                c.cycles += 1;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4c8)).lw(c);
                pc = 0x001eee24; continue;
            }
            0x001eee24 => {
                c.cycles += 8;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.g[S4][0];
                c.g[T1][0] = c.g[S7][0];
                c.g[A0][0] = c.g[S6][0];
                if callee::HUD_DRAW_SPRITE.call(c, 0x001eee3c).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001eee44; continue;
            }
            0x001eee44 => {
                c.cycles += 6;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T6][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001eee5c; continue;
            }
            0x001eee5c => {
                c.cycles += 20;
                c.g[V0][0] = 0x4d0000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x78b8));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[A2][0] as u32));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[S5][0] = 0u64;
                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.div_s(1, 0, 1);
                c.c_lt_s(1, 20);
                let t = !c.fcc(); 
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001eeef8; continue; }
                pc = 0x001eeeac; continue;
            }
            0x001eeeac => {
                c.cycles += 19;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5638));
                c.g[V1][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5634));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.mul_s(0, 1, 0);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5630));
                c.mul_s(1, 1, 1);
                c.g[S5][0] = c.g[S2][0];
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(20, 0, 20);
                pc = 0x001eeef8; continue;
            }
            0x001eeef8 => {
                c.cycles += 62;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A1][0] = 0x4a0000;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x21c));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x22c0));
                if callee::SPRINTF.call(c, 0x001eef20).is_err() { return; }
                c.g[V1][0] = 0x7f;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T0][0] = c.g[S0][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[A3][0] = 0xe;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S5][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001eef58).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S4][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4f0)).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001eef6c).is_err() { return; }
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A2][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001eef80).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[A0][0] = c.g[S3][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x36c)).sw(c, c.g[S0][0]);
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001eef90).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[A2][0] = c.g[S1][0];
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
                c.g[A0][0] = c.g[S6][0];
                c.mul_s(0, 0, 20);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.mul_s(1, 1, 20);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.mul_s(3, 3, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.mul_s(2, 2, 1);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A3][0] = c.g[S0][0];
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.g[T1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x23c)).lb(c);
                c.g[T2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x23d)).lb(c);
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001eefe8).is_err() { return; }
                let t = c.g[S5][0] == 0u64; 
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                if t { pc = 0x001ef018; continue; }
                pc = 0x001eeff0; continue;
            }
            0x001eeff0 => {
                c.cycles += 10;
                c.ldl(4, (c.g[S5][0] as u32).wrapping_add(0x7));
                c.ldr(4, (c.g[S5][0] as u32));
                c.ldl(5, (c.g[S5][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xd7));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xdf));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0xd8));
                let t = true; 
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001ef030; continue; }
                pc = 0x001ef018; continue;
            }
            0x001ef018 => {
                c.cycles += 6;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001ef028).is_err() { return; }
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                pc = 0x001ef030; continue;
            }
            0x001ef030 => {
                c.cycles += 21;
                c.g[T0][0] = 0xfffffffffffffc1f;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = c.g[S4][0];
                c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V1][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x23e)).lb(c);
                c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[T1][0] = c.g[A2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x36c)).lw(c);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S5][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                Addr((c.g[T2][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001ef07c).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef084; continue;
            }
            0x001ef084 => {
                c.cycles += 6;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T5][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef09c; continue;
            }
            0x001ef09c => {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[S4][0] as u32)).lw(c);
                c.g[V0][0] = 0x27;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x13;
                if t { pc = 0x001ef0b4; continue; }
                pc = 0x001ef0ac; continue;
            }
            0x001ef0ac => {
                c.cycles += 2;
                let t = true; 
                c.g[A0][0] = 0xd;
                if t { pc = 0x001ef0c0; continue; }
                pc = 0x001ef0b4; continue;
            }
            0x001ef0b4 => {
                c.cycles += 3;
                c.g[V1][0] = c.g[V1][0] ^ 0x28;
                c.g[V0][0] = 0xe;
                if c.g[V1][0] == 0 { c.g[A0][0] = c.g[V0][0]; }
                pc = 0x001ef0c0; continue;
            }
            0x001ef0c0 => {
                c.cycles += 173;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S2][0] = 0x24;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.lo[0] = c.g[V1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x50)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x54)).sw(c, 0u64);
                c.madd(0, 18, 4, 18);
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.g[V1][0];
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2d0));
                c.g[A1][0] = c.g[S0][0];
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.mul_s(21, 22, 22);
                c.g[A0][0] = c.g[S2][0];
                c.g[S5][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                if callee::HUD_GET_ELEM_POS.call(c, 0x001ef110).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001ef120).is_err() { return; }
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A2][0] = 0u64;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A3][0] = 0u64;
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                if callee::FONT_MEASURE_TEXT.call(c, 0x001ef13c).is_err() { return; }
                c.mov_s(20, 0);
                Addr((c.g[SP][0] as u32).wrapping_add(0x94)).sw(c, 0u64);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[T1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x20)).lb(c);
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[T2][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x21)).lb(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S6][0];
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001ef16c).is_err() { return; }
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = c.g[S1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x94)).sw(c, 0u64);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[T1][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x2f0)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T2][0] = Addr((c.g[T4][0] as u32).wrapping_add(0x2f1)).lb(c);
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001ef1a0).is_err() { return; }
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S2][0];
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.sub_s(0, 0, 4);
                c.sub_s(1, 1, 3);
                c.mul_s(2, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.mul_s(21, 1, 21);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
                c.add_s(4, 2, 4);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.add_s(3, 21, 3);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x94));
                if callee::HUD_GET_ELEM_COLOR_ROTATED.call(c, 0x001ef1ec).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A0][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_COLOR_ROTATED.call(c, 0x001ef1f8).is_err() { return; }
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[A0][0] = c.g[S2][0];
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.mfc1(3, 22);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xcc));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xc4));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd8));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xdc));
                c.ldl(13, (c.g[SP][0] as u32).wrapping_add(0xd7));
                c.ldr(13, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.ldl(14, (c.g[SP][0] as u32).wrapping_add(0xdf));
                c.ldr(14, (c.g[SP][0] as u32).wrapping_add(0xd8));
                c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x47));
                c.sdr(13, (c.g[V1][0] as u32).wrapping_add(0x40));
                c.sdl(14, (c.g[V1][0] as u32).wrapping_add(0x4f));
                c.sdr(14, (c.g[V1][0] as u32).wrapping_add(0x48));
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001ef298).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001ef2a4).is_err() { return; }
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[A3][0] = 0xfffffffffffffc1f;
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.mov_s(12, 22);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = c.g[S6][0];
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
                c.sub_s(0, 0, 4);
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.sub_s(1, 1, 5);
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.mul_s(2, 0, 22);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.mul_s(3, 1, 22);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x114));
                c.add_s(4, 2, 4);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.add_s(5, 3, 5);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 4);
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x22)).lb(c);
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A2][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x94));
                if callee::HUD_DRAW_WOBBLE_TEXT.call(c, 0x001ef348).is_err() { return; }
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x104));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x107));
                c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef374; continue;
            }
            0x001ef374 => {
                c.cycles += 6;
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T4][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef38c; continue;
            }
            0x001ef38c => {
                c.cycles += 11;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.mtc1(0, 0);
                c.lwc1(1, (c.g[T5][0] as u32).wrapping_add(0x70));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ef408; continue; }
                pc = 0x001ef3b8; continue;
            }
            0x001ef3b8 => {
                c.cycles += 20;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x562c));
                c.g[V1][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5628));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.mul_s(0, 1, 0);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5624));
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 20, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(0, 0, 20);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                pc = 0x001ef408; continue;
            }
            0x001ef408 => {
                c.cycles += 24;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = 0x7f;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = 0x12;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ef440).is_err() { return; }
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.g[T1][0] = c.g[S2][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.mov_s(12, 20);
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T0][0] = 0u64;
                if callee::HUD_DRAW_PIP_METER.call(c, 0x001ef460).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef468; continue;
            }
            0x001ef468 => {
                c.cycles += 6;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef480; continue;
            }
            0x001ef480 => {
                c.cycles += 13;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = 0x4a0000;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x358)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x22d8));
                c.g[A0][0] = c.g[V0][0];
                c.g[S2][0] = c.g[V0][0];
                if callee::SPRINTF.call(c, 0x001ef49c).is_err() { return; }
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                c.c_lt_s(22, 12);
                let t = !c.fcc(); 
                c.g[A0][0] = 0x4d0000;
                if t { pc = 0x001ef56c; continue; }
                pc = 0x001ef4b4; continue;
            }
            0x001ef4b4 => {
                c.cycles += 46;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5620));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x561c));
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.mul_s(0, 22, 0);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x79f8));
                c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[T1][0] as u32));
                c.ldl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[V1][0] = 0x7f;
                Addr((c.g[SP][0] as u32)).sw(c, c.g[V0][0]);
                c.g[T2][0] = 0x500000;
                c.mul_s(0, 0, 1);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x4fb8));
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.mul_s(2, 22, 22);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T0][0] = c.g[S2][0];
                c.cvt_w_s(1, 0);
                c.mfc1(2, 1);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5618));
                c.sub_s(2, 12, 2);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = 0x1c;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                c.g[T3][0] = 0u64;
                c.lwc1(0, (c.g[V0][0] as u32));
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 2);
                c.add_s(0, 0, 12);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ef564).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef56c; continue;
            }
            0x001ef56c => {
                c.cycles += 12;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[S2][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0x1c;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T3][0] = 0u64;
                pc = 0x001ef59c; continue;
            }
            0x001ef59c => {
                c.cycles += 4;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ef5a4).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef5ac; continue;
            }
            0x001ef5ac => {
                c.cycles += 6;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x22e0));
                c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x350)).sw(c, c.g[V0][0]);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef5c4; continue;
            }
            0x001ef5c4 => {
                c.cycles += 6;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1480));
                c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x350)).sw(c, c.g[V0][0]);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef5dc; continue;
            }
            0x001ef5dc => {
                c.cycles += 6;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1470));
                c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x350)).sw(c, c.g[V0][0]);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef5f4; continue;
            }
            0x001ef5f4 => {
                c.cycles += 6;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1460));
                c.g[S5][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x350)).sw(c, c.g[V0][0]);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef60c; continue;
            }
            0x001ef60c => {
                c.cycles += 6;
                c.g[V0][0] = 0x470000;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1450));
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                let t = true; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x350)).sw(c, c.g[V0][0]);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef624; continue;
            }
            0x001ef624 => {
                c.cycles += 7;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = 0x100000;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.mov_s(29, 22);
                c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef640; continue;
            }
            0x001ef640 => {
                c.cycles += 5;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[V0][0] = 0x3;
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x88)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef654; continue;
            }
            0x001ef654 => {
                c.cycles += 20;
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                c.g[S0][0] = 0x470000;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1440));
                if callee::FE_FORMAT_MONEY.call(c, 0x001ef668).is_err() { return; }
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                if callee::SPRINTF.call(c, 0x001ef678).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[S1][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[A3][0] = 0x1;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                let t = true; 
                c.g[T2][0] = 0u64;
                if t { pc = 0x001ef90c; continue; }
                pc = 0x001ef6a4; continue;
            }
            0x001ef6a4 => {
                c.cycles += 6;
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x200000;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T4][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef6bc; continue;
            }
            0x001ef6bc => {
                c.cycles += 7;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.mtc1(0, 1);
                c.lwc1(0, (c.g[T5][0] as u32).wrapping_add(0x68));
                c.c_lt_s(0, 1);
                let t = c.fcc(); 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef6d8; continue;
            }
            0x001ef6d8 => {
                c.cycles += 27;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).lw(c);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[V0][0] = sx((c.g[T6][0] as u32) << 24);
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x35c)).lw(c);
                c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x2f));
                c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x28));
                c.ldl(10, (c.g[SP][0] as u32).wrapping_add(0x37));
                c.ldr(10, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S1][0] = sx((c.g[T6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 2);
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(0, (c.g[T6][0] as u32).wrapping_add(0x68));
                c.sub_s(0, 0, 2);
                c.add_s(0, 0, 0);
                c.mul_s(1, 1, 0);
                let t = c.g[V0][0] == 0u64; 
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001ef7a0; continue; }
                pc = 0x001ef744; continue;
            }
            0x001ef744 => {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[T0][0] as u32) << 24);
                c.g[V0][0] = 0x1000000;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S2][0] = 0x1000000;
                pc = 0x001ef758; continue;
            }
            0x001ef758 => {
                c.cycles += 18;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = 0x1;
                if callee::HUD_DRAW_POSITION_PANEL.call(c, 0x001ef78c).is_err() { return; }
                c.g[V0][0] = c.g[S0][0];
                c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                if t { if back_edge_unwinds(c, 0x001ef758) { return; } pc = 0x001ef758; continue; }
                pc = 0x001ef7a0; continue;
            }
            0x001ef7a0 => {
                c.cycles += 7;
                Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).sw(c, c.g[S1][0]);
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.ldl(14, (c.g[SP][0] as u32).wrapping_add(0x67));
                pc = 0x001ef7bc; continue;
            }
            0x001ef7bc => {
                c.cycles += 5;
                c.ldr(14, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(14, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(14, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef7d0; continue;
            }
            0x001ef7d0 => {
                c.cycles += 23;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[V0][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = 0x470000;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A3][0] = 0x35;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1430));
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ef80c).is_err() { return; }
                c.mov_s(26, 22);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[A3][0] = 0x1;
                Addr((c.g[SP][0] as u32).wrapping_add(0x34c)).sw(c, c.g[A3][0]);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 28);
                let t = true; 
                c.g[S5][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef82c; continue;
            }
            0x001ef82c => {
                c.cycles += 6;
                c.mtc1(0, 0);
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.c_lt_s(26, 0);
                let t = !c.fcc(); 
                c.g[S5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef844; continue;
            }
            0x001ef844 => {
                c.cycles += 3;
                c.mov_s(28, 22);
                let t = true; 
                c.mov_s(26, 0);
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef850; continue;
            }
            0x001ef850 => {
                c.cycles += 5;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[T2][0] & 0x100;
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef864; continue;
            }
            0x001ef864 => {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(27, 0);
                
                if !c.fcc() { c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x50)); pc = 0x001ef8ac; continue; }
                pc = 0x001ef878; continue;
            }
            0x001ef878 => {
                c.cycles += 13;
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                if callee::FE_FORMAT_MONEY.call(c, 0x001ef880).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[V0][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[A3][0] = 0x38;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                let t = true; 
                c.g[T2][0] = 0u64;
                if t { pc = 0x001ef90c; continue; }
                pc = 0x001ef8ac; continue;
            }
            0x001ef8ac => {
                c.cycles += 24;
                c.g[V0][0] = 0x4d0000;
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ad8));
                c.ldl(11, (c.g[T6][0] as u32).wrapping_add(0x7));
                c.ldr(11, (c.g[T6][0] as u32));
                c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0x67));
                c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x6f));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                if callee::FE_FORMAT_MONEY.call(c, 0x001ef8e4).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[V0][0];
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S0][0]);
                c.g[A3][0] = 0x38;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                pc = 0x001ef90c; continue;
            }
            0x001ef90c => {
                c.cycles += 6;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001ef91c).is_err() { return; }
                let t = true; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef924; continue;
            }
            0x001ef924 => {
                c.cycles += 6;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                c.g[V0][0] = 0x4000000;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001efac0; continue; }
                pc = 0x001ef93c; continue;
            }
            0x001ef93c => {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0x200;
                let t = c.g[V0][0] == 0u64; 
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                if t { pc = 0x001efac4; continue; }
                pc = 0x001ef948; continue;
            }
            0x001ef948 => {
                c.cycles += 11;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5614));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.c_le_s(0, 22);
                Addr((c.g[SP][0] as u32).wrapping_add(0x50)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x54)).sw(c, 0u64);
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x28));
                let t = !c.fcc(); 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001ef9c4; continue; }
                pc = 0x001ef974; continue;
            }
            0x001ef974 => {
                c.cycles += 20;
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                if callee::FE_FORMAT_MONEY.call(c, 0x001ef97c).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.mov_s(12, 22);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T1][0] = c.g[V0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x20)).sw(c, c.g[V1][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A3][0] = 0x12;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T0][0] = 0x38;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, 0u64);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_TEXT_LERP.call(c, 0x001ef9bc).is_err() { return; }
                let t = true; 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                if t { pc = 0x001efa98; continue; }
                pc = 0x001ef9c4; continue;
            }
            0x001ef9c4 => {
                c.cycles += 53;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5610));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x560c));
                c.g[V1][0] = 0x500000;
                c.mul_s(1, 22, 1);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5608));
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ad8));
                c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T4][0] as u32));
                c.ldl(10, (c.g[T4][0] as u32).wrapping_add(0xf));
                c.ldr(10, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(0, 1, 0);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5604));
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 3);
                c.sub_s(1, 2, 1);
                c.cvt_w_s(3, 0);
                c.mfc1(2, 3);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 4);
                c.mul_s(0, 0, 1);
                c.add_s(0, 0, 2);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x14)).lw(c);
                if callee::FE_FORMAT_MONEY.call(c, 0x001efa54).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.mov_s(12, 22);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T1][0] = c.g[V0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[S0][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x20)).sw(c, c.g[V1][0]);
                c.g[A3][0] = 0x12;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[T0][0] = 0x38;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, 0u64);
                c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_TEXT_LERP.call(c, 0x001efa94).is_err() { return; }
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x354)).lw(c);
                pc = 0x001efa98; continue;
            }
            0x001efa98 => {
                c.cycles += 10;
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x28));
                pc = 0x001efac0; continue;
            }
            0x001efac0 => {
                c.cycles += 1;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x344)).lw(c);
                pc = 0x001efac4; continue;
            }
            0x001efac4 => {
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x1b4)).lw(c);
                pc = 0x001efac8; continue;
            }
            0x001efac8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x354)).sw(c, c.g[S5][0]);
                if t { if back_edge_unwinds(c, 0x001ecc40) { return; } pc = 0x001ecc40; continue; }
                pc = 0x001efad4; continue;
            }
            0x001efad4 => {
                c.cycles += 3;
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x330)).lw(c);
                let t = c.g[T0][0] != 0u64; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x370)).lw(c);
                if t { pc = 0x001f07b4; continue; }
                pc = 0x001efae0; continue;
            }
            0x001efae0 => {
                c.cycles += 3;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x350)).lw(c);
                let t = c.g[T1][0] == 0u64; 
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x320)).lw(c);
                if t { pc = 0x001efb3c; continue; }
                pc = 0x001efaec; continue;
            }
            0x001efaec => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[T2][0] != c.g[V0][0]; 
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                if t { pc = 0x001efb3c; continue; }
                pc = 0x001efaf8; continue;
            }
            0x001efaf8 => {
                c.cycles += 4;
                c.g[V0][0] = 0x4000000;
                c.g[V0][0] = c.g[T3][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[T0][0] = c.g[T1][0];
                if t { pc = 0x001efb3c; continue; }
                pc = 0x001efb08; continue;
            }
            0x001efb08 => {
                c.cycles += 13;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[V0][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[T2][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001efb3c).is_err() { return; }
                pc = 0x001efb3c; continue;
            }
            0x001efb3c => {
                c.cycles += 5;
                c.mtc1(0, 5);
                c.c_le_s(5, 26);
                let t = !c.fcc(); 
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                if t { pc = 0x001efc40; continue; }
                pc = 0x001efb50; continue;
            }
            0x001efb50 => {
                c.cycles += 7;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5600));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 4);
                c.c_lt_s(26, 0);
                let t = !c.fcc(); 
                c.g[V1][0] = 0x500000;
                if t { pc = 0x001efbb8; continue; }
                pc = 0x001efb6c; continue;
            }
            0x001efb6c => {
                c.cycles += 19;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x55fc));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x55f8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fb8));
                c.mul_s(1, 26, 1);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x55f4));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x55f0));
                c.mul_s(0, 1, 0);
                c.mul_s(1, 1, 1);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 4, 1);
                c.cvt_w_s(2, 0);
                c.mfc1(2, 2);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.mul_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.add_s(4, 0, 4);
                pc = 0x001efbb8; continue;
            }
            0x001efbb8 => {
                c.cycles += 6;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x55ec));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 13);
                c.c_lt_s(0, 26);
                let t = !c.fcc(); 
                
                if t { pc = 0x001efbd8; continue; }
                pc = 0x001efbd0; continue;
            }
            0x001efbd0 => {
                c.cycles += 2;
                let t = true; 
                c.mov_s(13, 5);
                if t { pc = 0x001efbfc; continue; }
                pc = 0x001efbd8; continue;
            }
            0x001efbd8 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x55e8));
                c.c_lt_s(0, 26);
                let t = !c.fcc(); 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x34c)).lw(c);
                if t { pc = 0x001efc00; continue; }
                pc = 0x001efbec; continue;
            }
            0x001efbec => {
                c.cycles += 4;
                c.sub_s(0, 26, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x55e4));
                c.mul_s(0, 0, 1);
                c.sub_s(13, 13, 0);
                pc = 0x001efbfc; continue;
            }
            0x001efbfc => {
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x34c)).lw(c);
                pc = 0x001efc00; continue;
            }
            0x001efc00 => {
                c.cycles += 4;
                c.g[T0][0] = 0u64;
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x54));
                let t = c.g[V0][0] != 0u64; 
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001efc20; continue; }
                pc = 0x001efc10; continue;
            }
            0x001efc10 => {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = 0x1000000;
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[T0][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x001efc20; continue;
            }
            0x001efc20 => {
                c.cycles += 8;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.mov_s(12, 28);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x32c)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if callee::HUD_DRAW_CONTROL_HINT.call(c, 0x001efc3c).is_err() { return; }
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                pc = 0x001efc40; continue;
            }
            0x001efc40 => {
                c.cycles += 6;
                c.g[V0][0] = 0x200000;
                c.g[S4][0] = c.g[A0][0] & 0x1;
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                c.g[S5][0] = c.g[A0][0] & 0x8000;
                let t = c.g[V0][0] == 0u64; 
                c.g[FP][0] = c.g[A0][0] & 0x80;
                if t { pc = 0x001f0240; continue; }
                pc = 0x001efc58; continue;
            }
            0x001efc58 => {
                c.cycles += 9;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[S5][0] = 0x1;
                c.mtc1(0, 1);
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x64));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.c_le_s(1, 0);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xd6)).lb(c);
                let t = c.fcc(); 
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                if t { pc = 0x001efc80; continue; }
                pc = 0x001efc7c; continue;
            }
            0x001efc7c => {
                c.cycles += 1;
                c.g[S5][0] = 0u64;
                pc = 0x001efc80; continue;
            }
            0x001efc80 => {
                c.cycles += 14;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.g[S0][0];
                c.g[S7][0] = c.g[S0][0];
                c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6c));
                c.g[A0][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001efc9c).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x538)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001efcb0).is_err() { return; }
                let t = c.g[S5][0] == 0u64; 
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                if t { pc = 0x001efd28; continue; }
                pc = 0x001efcb8; continue;
            }
            0x001efcb8 => {
                c.cycles += 15;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0x5b));
                c.ldr(9, (c.g[T0][0] as u32).wrapping_add(0x54));
                c.ldl(10, (c.g[T0][0] as u32).wrapping_add(0x63));
                c.ldr(10, (c.g[T0][0] as u32).wrapping_add(0x5c));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x87));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x8f));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x88));
                c.lwc1(2, (c.g[T0][0] as u32).wrapping_add(0x64));
                c.c_lt_s(2, 0);
                let t = !c.fcc(); 
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                if t { pc = 0x001efd08; continue; }
                pc = 0x001efcf4; continue;
            }
            0x001efcf4 => {
                c.cycles += 5;
                c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[T5][0]);
                c.mul_s(0, 0, 2);
                let t = true; 
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                if t { pc = 0x001efd3c; continue; }
                pc = 0x001efd08; continue;
            }
            0x001efd08 => {
                c.cycles += 8;
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 1);
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.sub_s(1, 1, 2);
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[T6][0]);
                c.mul_s(0, 0, 1);
                let t = true; 
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                if t { pc = 0x001efd3c; continue; }
                pc = 0x001efd28; continue;
            }
            0x001efd28 => {
                c.cycles += 5;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001efd38).is_err() { return; }
                Addr((c.g[SP][0] as u32).wrapping_add(0x368)).sw(c, c.g[S0][0]);
                pc = 0x001efd3c; continue;
            }
            0x001efd3c => {
                c.cycles += 51;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[T1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lb(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8d)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S1][0];
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001efd68).is_err() { return; }
                c.g[S4][0] = 0xfffffffffffffc1f;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8e)).lb(c);
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x368)).lw(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[S2][0] = c.g[S0][0];
                c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x538)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001efdb4).is_err() { return; }
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x55e0));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sub_s(0, 0, 3);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x55dc));
                c.sub_s(2, 2, 3);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.add_s(4, 4, 5);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.add_s(1, 1, 5);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.mtc1(0, 3);
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x64));
                c.c_le_s(3, 0);
                let t = !c.fcc(); 
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001efe94; continue; }
                pc = 0x001efe08; continue;
            }
            0x001efe08 => {
                c.cycles += 35;
                c.g[V1][0] = sx((c.g[S3][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 13);
                c.g[T0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x55d8));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x55d4));
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
                c.mov_s(14, 13);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[T0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x97));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.ldr(12, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x5b));
                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x54));
                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x63));
                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x5c));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0xb7));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0xbf));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0xb8));
                c.lwc1(12, (c.g[V1][0] as u32).wrapping_add(0x64));
                if callee::HUD_DRAW_STRETCHED_PANEL.call(c, 0x001efe94).is_err() { return; }
                pc = 0x001efe94; continue;
            }
            0x001efe94 => {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                if t { pc = 0x001efec4; continue; }
                pc = 0x001efe9c; continue;
            }
            0x001efe9c => {
                c.cycles += 10;
                c.ldl(13, (c.g[T4][0] as u32).wrapping_add(0x5b));
                c.ldr(13, (c.g[T4][0] as u32).wrapping_add(0x54));
                c.ldl(14, (c.g[T4][0] as u32).wrapping_add(0x63));
                c.ldr(14, (c.g[T4][0] as u32).wrapping_add(0x5c));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                
                if t { pc = 0x001efed4; continue; }
                pc = 0x001efec4; continue;
            }
            0x001efec4 => {
                c.cycles += 4;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001efed4).is_err() { return; }
                pc = 0x001efed4; continue;
            }
            0x001efed4 => {
                c.cycles += 20;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0x4;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x328)).lw(c);
                c.g[A3][0] = 0u64;
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T1][0] = c.g[S2][0];
                c.mov_s(13, 25);
                c.mov_s(14, 12);
                c.g[T3][0] = c.g[S7][0];
                if callee::HUD_DRAW_METER_BAR.call(c, 0x001eff08).is_err() { return; }
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.mtc1(0, 0);
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x64));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t { pc = 0x001effd0; continue; }
                pc = 0x001eff24; continue;
            }
            0x001eff24 => {
                c.cycles += 43;
                c.g[T1][0] = 0xfffffffffffffc1f;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.g[T0][0] = sx((c.g[S3][0] as u32) << 5);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[T0][0] = c.g[T0][0] & 0x3e0;
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x6c));
                c.g[A0][0] = c.g[S6][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x55d0));
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x55cc));
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x55c8));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x55c4));
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x97));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0x87));
                c.ldr(12, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x5b));
                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x54));
                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x63));
                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x5c));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0xb7));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0xbf));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0xb8));
                c.lwc1(12, (c.g[V1][0] as u32).wrapping_add(0x64));
                if callee::HUD_DRAW_STRETCHED_PANEL.call(c, 0x001effd0).is_err() { return; }
                pc = 0x001effd0; continue;
            }
            0x001effd0 => {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                if t { pc = 0x001f0000; continue; }
                pc = 0x001effd8; continue;
            }
            0x001effd8 => {
                c.cycles += 10;
                c.ldl(13, (c.g[T4][0] as u32).wrapping_add(0x5b));
                c.ldr(13, (c.g[T4][0] as u32).wrapping_add(0x54));
                c.ldl(14, (c.g[T4][0] as u32).wrapping_add(0x63));
                c.ldr(14, (c.g[T4][0] as u32).wrapping_add(0x5c));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = true; 
                c.g[A3][0] = 0x3;
                if t { pc = 0x001f0014; continue; }
                pc = 0x001f0000; continue;
            }
            0x001f0000 => {
                c.cycles += 5;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb4));
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f0010).is_err() { return; }
                c.g[A3][0] = 0x1;
                pc = 0x001f0014; continue;
            }
            0x001f0014 => {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0x5;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.mov_s(13, 25);
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x328)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[T1][0] = c.g[S2][0];
                c.mov_s(14, 12);
                c.g[T3][0] = c.g[S7][0];
                if callee::HUD_DRAW_METER_BAR.call(c, 0x001f0044).is_err() { return; }
                c.mtc1(0, 20);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x64));
                c.c_le_s(20, 0);
                let t = !c.fcc(); 
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t { pc = 0x001f010c; continue; }
                pc = 0x001f0060; continue;
            }
            0x001f0060 => {
                c.cycles += 43;
                c.g[T1][0] = 0xfffffffffffffc1f;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.g[T0][0] = sx((c.g[S3][0] as u32) << 5);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[T0][0] = c.g[T0][0] & 0x3e0;
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x6c));
                c.g[A0][0] = c.g[S6][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x55c0));
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x55bc));
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x55b8));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x55b4));
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x77));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x97));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0x87));
                c.ldr(12, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0xa7));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x5b));
                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x54));
                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x63));
                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x5c));
                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0xb7));
                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0xbf));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0xb8));
                c.lwc1(12, (c.g[V1][0] as u32).wrapping_add(0x64));
                if callee::HUD_DRAW_STRETCHED_PANEL.call(c, 0x001f010c).is_err() { return; }
                pc = 0x001f010c; continue;
            }
            0x001f010c => {
                c.cycles += 7;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).lw(c);
                c.g[S4][0] = c.g[T5][0] & 0x1;
                c.g[S5][0] = c.g[T5][0] & 0x8000;
                c.g[V0][0] = ((c.g[T4][0] as i64) < 9i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[FP][0] = c.g[T5][0] & 0x80;
                if t { pc = 0x001f01d4; continue; }
                pc = 0x001f0128; continue;
            }
            0x001f0128 => {
                c.cycles += 12;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V0][0] = sx((c.g[T4][0] as u32) << 24);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = ((c.g[T0][0] as i64) < 9i64) as u64;
                c.ldl(14, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(14, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(14, (c.g[V1][0] as u32).wrapping_add(0x2f));
                c.sdr(14, (c.g[V1][0] as u32).wrapping_add(0x28));
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = sx((c.g[T0][0] as u32) << 24);
                if t { pc = 0x001f01b0; continue; }
                pc = 0x001f0158; continue;
            }
            0x001f0158 => {
                c.cycles += 4;
                c.g[V0][0] = 0x1000000;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S3][0] = 0x1000000;
                pc = 0x001f0168; continue;
            }
            0x001f0168 => {
                c.cycles += 18;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T1][0] = c.g[S2][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0x37));
                c.ldr(12, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_POSITION_PANEL.call(c, 0x001f019c).is_err() { return; }
                c.g[V0][0] = c.g[S0][0];
                c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[T0][0] as i64) < 9i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                if t { if back_edge_unwinds(c, 0x001f0168) { return; } pc = 0x001f0168; continue; }
                pc = 0x001f01b0; continue;
            }
            0x001f01b0 => {
                c.cycles += 9;
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 0);
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x2f));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x28));
                pc = 0x001f01d4; continue;
            }
            0x001f01d4 => {
                c.cycles += 5;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V1][0] = 0x3000000;
                c.g[V0][0] = c.g[A3][0] & c.g[V1][0];
                let t = c.g[V0][0] != c.g[V1][0]; 
                c.g[T0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f0240; continue; }
                pc = 0x001f01e8; continue;
            }
            0x001f01e8 => {
                c.cycles += 22;
                c.g[A1][0] = c.g[S2][0];
                c.g[S0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xa44));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001f01f8).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S7][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001f0204).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f0210).is_err() { return; }
                c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.ldr(9, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x87));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x560));
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x368)).lw(c);
                c.g[T0][0] = Addr((c.g[T5][0] as u32).wrapping_add(0xa64)).lb(c);
                c.g[T1][0] = Addr((c.g[T5][0] as u32).wrapping_add(0xa65)).lb(c);
                if callee::HUD_DRAW_ICON_TEXT.call(c, 0x001f0240).is_err() { return; }
                pc = 0x001f0240; continue;
            }
            0x001f0240 => {
                c.cycles += 2;
                let t = c.g[S4][0] == 0u64; 
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                if t { pc = 0x001f0264; continue; }
                pc = 0x001f0248; continue;
            }
            0x001f0248 => {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[T6][0] as u32).wrapping_add(0x44)).lw(c);
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f0264; continue; }
                pc = 0x001f0254; continue;
            }
            0x001f0254 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A2][0] = c.g[T6][0];
                if callee::HUD_DRAW_RIDER_INDICATOR.call(c, 0x001f0264).is_err() { return; }
                pc = 0x001f0264; continue;
            }
            0x001f0264 => {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64; 
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f027c; continue; }
                pc = 0x001f026c; continue;
            }
            0x001f026c => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                if callee::HUD_DRAW_SPEEDOMETER.call(c, 0x001f027c).is_err() { return; }
                pc = 0x001f027c; continue;
            }
            0x001f027c => {
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001f036c; continue; }
                pc = 0x001f0290; continue;
            }
            0x001f0290 => {
                c.cycles += 33;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb1c));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001f02a4).is_err() { return; }
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x470)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001f02b8).is_err() { return; }
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001f02c8).is_err() { return; }
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[T1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xb3c)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xb3d)).lb(c);
                c.g[A3][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S2][0];
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001f02f4).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f0300).is_err() { return; }
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x320)).lw(c);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x324)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001f0328; continue; }
                pc = 0x001f0314; continue;
            }
            0x001f0314 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x55b0));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                pc = 0x001f0328; continue;
            }
            0x001f0328 => {
                c.cycles += 17;
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S4][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[T0][0] = c.g[S2][0];
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xb3e)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A3][0] = c.g[S3][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x470)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001f036c).is_err() { return; }
                pc = 0x001f036c; continue;
            }
            0x001f036c => {
                c.cycles += 9;
                if callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x001f0374).is_err() { return; }
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x7;
                if callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x001f0380).is_err() { return; }
                c.g[T0][0] = 0x530000;
                c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x4b30)).lw(c);
                let t = c.g[V1][0] == 0u64; 
                c.g[A0][0] = 0x1;
                if t { pc = 0x001f03a8; continue; }
                pc = 0x001f0390; continue;
            }
            0x001f0390 => {
                c.cycles += 4;
                c.g[T1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                c.g[V0][0] = Addr((c.g[T1][0] as u32).wrapping_add(0x87c)).lw(c);
                let t = c.g[V0][0] == 0u64; 
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                if t { pc = 0x001f03a8; continue; }
                pc = 0x001f03a0; continue;
            }
            0x001f03a0 => {
                c.cycles += 2;
                c.g[A0][0] = sx(((c.g[T2][0] as u32) as i32 >> 6) as u32);
                c.g[A0][0] = c.g[A0][0] & 0x1;
                pc = 0x001f03a8; continue;
            }
            0x001f03a8 => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64; 
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x320)).lw(c);
                if t { pc = 0x001f03e8; continue; }
                pc = 0x001f03b0; continue;
            }
            0x001f03b0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T3][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                if t { pc = 0x001f03d0; continue; }
                pc = 0x001f03bc; continue;
            }
            0x001f03bc => {
                c.cycles += 5;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x8));
                if callee::C_OV_STATE_HUD1_P_DRAW_PROGRESS_METER.call(c, 0x001f03c8).is_err() { return; }
                let t = true; 
                
                if t { pc = 0x001f03e8; continue; }
                pc = 0x001f03d0; continue;
            }
            0x001f03d0 => {
                c.cycles += 6;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x338)).lw(c);
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                c.g[A0][0] = sx((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = (0u64 < c.g[T6][0]) as u64;
                if callee::C_OV_STATE_HUD1_P_DRAW_PROGRESS_METER_MP.call(c, 0x001f03e8).is_err() { return; }
                pc = 0x001f03e8; continue;
            }
            0x001f03e8 => {
                c.cycles += 2;
                let t = c.g[FP][0] == 0u64; 
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x340)).lw(c);
                if t { pc = 0x001f0404; continue; }
                pc = 0x001f03f0; continue;
            }
            0x001f03f0 => {
                c.cycles += 5;
                c.mov_s(12, 29);
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if callee::C_OV_STATE_HUD1_P_DRAW_COLLECTED_COUNT.call(c, 0x001f0404).is_err() { return; }
                pc = 0x001f0404; continue;
            }
            0x001f0404 => {
                c.cycles += 6;
                c.g[V0][0] = 0x400000;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[V0][0] | 0x3c00;
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x400000;
                if t { pc = 0x001f07b0; continue; }
                pc = 0x001f041c; continue;
            }
            0x001f041c => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f0430; continue; }
                pc = 0x001f0428; continue;
            }
            0x001f0428 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S6][0];
                if callee::C_OV_STATE_HUD1_P_DRAW_MISSION_STATUS.call(c, 0x001f0430).is_err() { return; }
                pc = 0x001f0430; continue;
            }
            0x001f0430 => {
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[S1][0] = 0x30;
                c.g[V0][0] = c.g[A0][0] & 0x400;
                let t = c.g[V0][0] == 0u64; 
                c.g[S2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
                if t { pc = 0x001f049c; continue; }
                pc = 0x001f0444; continue;
            }
            0x001f0444 => {
                c.cycles += 3;
                c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                let t = (c.g[A2][0] as i64) < 0; 
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001f049c; continue; }
                pc = 0x001f0450; continue;
            }
            0x001f0450 => {
                c.cycles += 19;
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1420));
                c.g[A0][0] = c.g[S0][0];
                if callee::SPRINTF.call(c, 0x001f0460).is_err() { return; }
                c.g[S1][0] = 0x31;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A3][0] = 0x30;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T1][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001f049c).is_err() { return; }
                pc = 0x001f049c; continue;
            }
            0x001f049c => {
                c.cycles += 4;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[A1][0] & 0x800;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001f0500; continue; }
                pc = 0x001f04ac; continue;
            }
            0x001f04ac => {
                c.cycles += 21;
                c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x14)).lw(c);
                c.g[A3][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x18)).lw(c);
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1410));
                c.g[A0][0] = c.g[S0][0];
                if callee::SPRINTF.call(c, 0x001f04c4).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A3][0] = c.g[S1][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[T0][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T1][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001f0500).is_err() { return; }
                pc = 0x001f0500; continue;
            }
            0x001f0500 => {
                c.cycles += 4;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                c.g[V0][0] = c.g[A2][0] & 0x1000;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                if t { pc = 0x001f05a4; continue; }
                pc = 0x001f0510; continue;
            }
            0x001f0510 => {
                c.cycles += 37;
                c.g[A0][0] = c.g[S2][0];
                if callee::W_SCRIPT_MAN_GET_HUD_TARGET_HEIGHT.call(c, 0x001f0518).is_err() { return; }
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x55ac));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[AT][0] = 0x42c80000;
                c.mtc1(1, 3);
                c.g[A1][0] = 0x470000;
                c.mul_s(0, 0, 1);
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1400));
                c.cvt_w_s(1, 0);
                c.mfc1(6, 1);
                c.mtc1(6, 1);
                c.cvt_s_w(1, 1);
                c.sub_s(0, 0, 1);
                c.mul_s(0, 0, 3);
                c.add_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.mfc1(7, 1);
                c.g[A0][0] = c.g[S0][0];
                if callee::SPRINTF.call(c, 0x001f0568).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[V1][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A3][0] = c.g[S1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[T0][0] = c.g[S0][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T1][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001f05a0).is_err() { return; }
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x348)).lw(c);
                pc = 0x001f05a4; continue;
            }
            0x001f05a4 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x2000;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x38));
                if t { pc = 0x001f07b0; continue; }
                pc = 0x001f05b0; continue;
            }
            0x001f05b0 => {
                c.cycles += 32;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = 0x80;
                if callee::TRICK_BUILD_NAME.call(c, 0x001f05c0).is_err() { return; }
                c.g[S2][0] = 0x1000000;
                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[A2][0] = 0u64;
                c.g[S7][0] = 0x1000000;
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x72c));
                c.g[A0][0] = c.g[S0][0];
                c.g[S5][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001f05e4).is_err() { return; }
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001f05f0).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001f05fc).is_err() { return; }
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.g[S1][0];
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                pc = 0x001f0630; continue;
            }
            0x001f0630 => {
                c.cycles += 12;
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[A3][0] = 0x32;
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x32));
                if callee::FONT_FIND_LINE_BREAK.call(c, 0x001f064c).is_err() { return; }
                c.g[V1][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[S3][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                let t = c.g[A1][0] != 0u64; 
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                if t { if back_edge_unwinds(c, 0x001f0630) { return; } pc = 0x001f0630; continue; }
                pc = 0x001f0660; continue;
            }
            0x001f0660 => {
                c.cycles += 41;
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S5][0];
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f066c).is_err() { return; }
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A2][0] = 0xfffffffffffffc1f;
                c.mtc1(19, 13);
                c.cvt_s_w(13, 13);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 14);
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x207));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x20f));
                c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x208));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x47));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x40));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4f));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[A0][0] = c.g[S6][0];
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x74e)).lb(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
                c.cvt_s_w(0, 0);
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x34));
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0xe4));
                c.g[A1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x74d)).lb(c);
                c.mul_s(0, 0, 1);
                c.mul_s(21, 0, 2);
                c.mul_s(13, 21, 13);
                if callee::HUD_ALIGN_Y.call(c, 0x001f06fc).is_err() { return; }
                let t = (c.g[S3][0] as i64) <= 0; 
                c.mov_s(20, 0);
                if t { pc = 0x001f078c; continue; }
                pc = 0x001f0704; continue;
            }
            0x001f0704 => {
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(0xd0));
                c.g[S2][0] = 0x1000000;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S4][0] = 0x1000000;
                pc = 0x001f0718; continue;
            }
            0x001f0718 => {
                c.cycles += 29;
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A3][0] = 0u64;
                c.g[A2][0] = 0u64;
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x32));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                if callee::FONT_MEASURE_TEXT.call(c, 0x001f0738).is_err() { return; }
                c.g[T3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.mov_s(13, 0);
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x74c)).lb(c);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 14);
                c.g[A2][0] = 0u64;
                if callee::HUD_ALIGN_X.call(c, 0x001f075c).is_err() { return; }
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.mov_s(13, 20);
                c.g[A1][0] = c.g[S0][0];
                c.mov_s(12, 0);
                if callee::FONT_DRAW_TEXT.call(c, 0x001f0770).is_err() { return; }
                c.add_s(20, 20, 21);
                c.g[V0][0] = c.g[S2][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                if t { if back_edge_unwinds(c, 0x001f0718) { return; } pc = 0x001f0718; continue; }
                pc = 0x001f078c; continue;
            }
            0x001f078c => {
                c.cycles += 9;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.mul_s(0, 0, 1);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.mul_s(2, 2, 1);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3c));
                pc = 0x001f07b0; continue;
            }
            0x001f07b0 => {
                c.cycles += 1;
                c.g[T4][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x370)).lw(c);
                pc = 0x001f07b4; continue;
            }
            0x001f07b4 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T4][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let t = c.g[V1][0] == 0u64; 
                Addr((c.g[SP][0] as u32).wrapping_add(0x334)).sw(c, c.g[V0][0]);
                if t { pc = 0x001f07ec; continue; }
                pc = 0x001f07c8; continue;
            }
            0x001f07c8 => {
                c.cycles += 9;
                c.g[T5][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x334)).lw(c);
                c.g[V0][0] = 0x8c;
                c.mult(0, 2, 13, 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                Addr((c.g[SP][0] as u32).wrapping_add(0x338)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[V0][0] as u32)).lw(c);
                
                if (c.g[V1][0] as i64) >= 0 { Addr((c.g[SP][0] as u32).wrapping_add(0x33c)).sw(c, 0u64); if back_edge_unwinds(c, 0x001ecab8) { return; } pc = 0x001ecab8; continue; }
                pc = 0x001f07ec; continue;
            }
            0x001f07ec => {
                c.cycles += 1;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3c4)).lw(c);
                pc = 0x001f07f0; continue;
            }
            0x001f07f0 => {
                c.cycles += 3;
                c.g[V0][0] = 0x50;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x8;
                if t { pc = 0x001f0a38; continue; }
                pc = 0x001f07fc; continue;
            }
            0x001f07fc => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x3a;
                if t { pc = 0x001f0814; continue; }
                pc = 0x001f0804; continue;
            }
            0x001f0804 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t { pc = 0x001f0928; continue; }
                pc = 0x001f080c; continue;
            }
            0x001f080c => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x001f0bc8; continue; }
                pc = 0x001f0814; continue;
            }
            0x001f0814 => {
                c.cycles += 69;
                c.g[T6][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S5][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x434)).lw(c);
                c.g[A1][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[T6][0] as u32).wrapping_add(0x120));
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x55a8));
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001f0834).is_err() { return; }
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001f0848).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001f0858).is_err() { return; }
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x3c0));
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x55a4));
                c.g[A2][0] = c.g[S3][0];
                c.mul_s(0, 0, 20);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[A3][0] = c.g[S2][0];
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T0][0] = c.g[S1][0];
                c.add_s(0, 0, 3);
                c.g[T3][0] = 0u64;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = c.g[S4][0];
                c.mul_s(1, 1, 0);
                c.mul_s(2, 2, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[T1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x140)).lb(c);
                c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x141)).lb(c);
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001f08ac).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f08b8).is_err() { return; }
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x3c0));
                c.g[T3][0] = 0xfffffffffffffc1f;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x55a0));
                c.g[A1][0] = c.g[S5][0];
                c.sub_s(0, 0, 20);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.mul_s(0, 0, 2);
                c.g[A0][0] = c.g[S6][0];
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.sub_s(1, 1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[T2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x142)).lb(c);
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = c.g[V1][0] & c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] & 0x3e0;
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                Addr((c.g[T2][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001f0920).is_err() { return; }
                let t = true; 
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t { pc = 0x001f0bc8; continue; }
                pc = 0x001f0928; continue;
            }
            0x001f0928 => {
                c.cycles += 8;
                c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x3c0));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x559c));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.c_lt_s(3, 2);
                
                if !c.fcc() { c.mul_s(0, 2, 3); pc = 0x001f095c; continue; }
                pc = 0x001f0948; continue;
            }
            0x001f0948 => {
                c.cycles += 5;
                c.sub_s(0, 2, 3);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5598));
                c.mul_s(0, 0, 1);
                c.sub_s(20, 20, 0);
                c.mul_s(0, 2, 3);
                pc = 0x001f095c; continue;
            }
            0x001f095c => {
                c.cycles += 32;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5594));
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S0][0] = 0x470000;
                c.g[S2][0] = 0x7f;
                c.add_s(0, 0, 1);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x828));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x13e8));
                c.g[A1][0] = c.g[S1][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f098c).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T0][0] = c.g[S0][0];
                c.mul_s(0, 0, 20);
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S1][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[A3][0] = 0x3a;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S2][0]);
                c.g[T1][0] = 0u64;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T2][0] = c.g[S3][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S2][0]);
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001f09d0).is_err() { return; }
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3c8)).lw(c);
                let t = (c.g[V0][0] as i64) <= 0; 
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f0bc4; continue; }
                pc = 0x001f09dc; continue;
            }
            0x001f09dc => {
                c.cycles += 23;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x84c));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f09e8).is_err() { return; }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3c8)).lw(c);
                c.mul_s(0, 0, 20);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                if callee::FE_FORMAT_MONEY.call(c, 0x001f09fc).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x42c)).lw(c);
                c.g[T0][0] = c.g[V0][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[T2][0] = c.g[S3][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S1][0]);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S2][0]);
                c.g[A3][0] = 0x3b;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T1][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S2][0]);
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001f0a30).is_err() { return; }
                let t = true; 
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t { pc = 0x001f0bc8; continue; }
                pc = 0x001f0a38; continue;
            }
            0x001f0a38 => {
                c.cycles += 6;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
                let t = (c.g[A0][0] as i64) <= 0; 
                c.g[V0][0] = 0x3c;
                if t { pc = 0x001f0bc4; continue; }
                pc = 0x001f0a50; continue;
            }
            0x001f0a50 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5590));
                c.div(0, 4, 2);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x558c));
                
                if c.g[V0][0] == 0u64 {  pc = 0x001f0a64; continue; }
                pc = 0x001f0a64; continue;
            }
            0x001f0a64 => {
                c.cycles += 17;
                c.g[V1][0] = 0x3c;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 21);
                c.g[A0][0] = c.hi[0];
                c.g[A1][0] = c.lo[0];
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.mul_s(20, 0, 1);
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                c.c_lt_s(20, 2);
                let t = !c.fcc(); 
                c.g[S5][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x434)).lw(c);
                if t { pc = 0x001f0ab8; continue; }
                pc = 0x001f0aa8; continue;
            }
            0x001f0aa8 => {
                c.cycles += 4;
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 0);
                let t = true; 
                c.mul_s(21, 20, 0);
                if t { pc = 0x001f0adc; continue; }
                pc = 0x001f0ab8; continue;
            }
            0x001f0ab8 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5588));
                c.c_lt_s(0, 20);
                let t = !c.fcc(); 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f0ae0; continue; }
                pc = 0x001f0acc; continue;
            }
            0x001f0acc => {
                c.cycles += 4;
                c.sub_s(0, 20, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5584));
                c.mul_s(0, 0, 1);
                c.sub_s(21, 21, 0);
                pc = 0x001f0adc; continue;
            }
            0x001f0adc => {
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                pc = 0x001f0ae0; continue;
            }
            0x001f0ae0 => {
                c.cycles += 57;
                c.g[S0][0] = 0x24;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.lo[0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                c.madd(0, 16, 4, 16);
                c.g[A0][0] = c.g[S0][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001f0afc).is_err() { return; }
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001f0b10).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001f0b20).is_err() { return; }
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5580));
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x557c));
                c.g[A2][0] = c.g[S3][0];
                c.mul_s(0, 20, 0);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.g[A3][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[T3][0] = 0u64;
                c.add_s(0, 0, 3);
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = c.g[S4][0];
                c.mul_s(1, 1, 0);
                c.mul_s(2, 2, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[T1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lb(c);
                c.g[T2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x21)).lb(c);
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001f0b70).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f0b7c).is_err() { return; }
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[T3][0] = 0xfffffffffffffc1f;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[A1][0] = c.g[S5][0];
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x22)).lb(c);
                c.g[A2][0] = c.g[S4][0];
                c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A3][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T0][0] = c.g[S1][0];
                c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[A0][0] = c.g[S6][0];
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[T2][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001f0bc4).is_err() { return; }
                pc = 0x001f0bc4; continue;
            }
            0x001f0bc4 => {
                c.cycles += 1;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                pc = 0x001f0bc8; continue;
            }
            0x001f0bc8 => {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64; 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x330)).lw(c);
                if t { pc = 0x001f0ea8; continue; }
                pc = 0x001f0bd0; continue;
            }
            0x001f0bd0 => {
                c.cycles += 3;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x3fc)).lw(c);
                let t = (c.g[A0][0] as i64) <= 0; 
                c.g[V0][0] = 0x3c;
                if t { pc = 0x001f0d70; continue; }
                pc = 0x001f0bdc; continue;
            }
            0x001f0bdc => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.div(0, 4, 2);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5578));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5574));
                
                if c.g[V0][0] == 0u64 {  pc = 0x001f0bf4; continue; }
                pc = 0x001f0bf4; continue;
            }
            0x001f0bf4 => {
                c.cycles += 11;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 21);
                c.g[A0][0] = c.hi[0];
                c.div(0, 3, 2);
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(20, 0, 2);
                c.c_lt_s(20, 1);
                c.g[V1][0] = c.lo[0];
                let t = !c.fcc(); 
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001f0c60; continue; }
                pc = 0x001f0c20; continue;
            }
            0x001f0c20 => {
                c.cycles += 4;
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 0);
                let t = true; 
                c.mul_s(21, 20, 0);
                if t { pc = 0x001f0c84; continue; }
                pc = 0x001f0c30; continue;
            }
            0x001f0c30 => {
                c.cycles += 12;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x13a8));
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if callee::STR_HASH.call(c, 0x001f0c48).is_err() { return; }
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                if call_ptr(c, t, 0x001f0c58).is_err() { return; }
                let t = true; 
                c.g[T0][0] = c.g[V0][0];
                if t { pc = 0x001f0e3c; continue; }
                pc = 0x001f0c60; continue;
            }
            0x001f0c60 => {
                c.cycles += 5;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5570));
                c.c_lt_s(0, 20);
                let t = !c.fcc(); 
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                if t { pc = 0x001f0c88; continue; }
                pc = 0x001f0c74; continue;
            }
            0x001f0c74 => {
                c.cycles += 4;
                c.sub_s(0, 20, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x556c));
                c.mul_s(0, 0, 1);
                c.sub_s(21, 21, 0);
                pc = 0x001f0c84; continue;
            }
            0x001f0c84 => {
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                pc = 0x001f0c88; continue;
            }
            0x001f0c88 => {
                c.cycles += 58;
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.g[S4][0];
                c.g[S2][0] = 0x7f;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x990));
                if callee::HUD_GET_ELEM_COLOR.call(c, 0x001f0c9c).is_err() { return; }
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5568));
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = 0x4a0000;
                c.mul_s(0, 20, 0);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5564));
                c.mul_s(1, 1, 21);
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                c.g[A0][0] = c.g[S3][0];
                c.add_s(0, 0, 2);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                if callee::SPRINTF.call(c, 0x001f0cd8).is_err() { return; }
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x13d8));
                c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8c)).lw(c);
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if callee::STR_HASH.call(c, 0x001f0cf8).is_err() { return; }
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                if call_ptr(c, t, 0x001f0d08).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[T0][0] = c.g[V0][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = 0x43;
                c.g[T1][0] = 0u64;
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S2][0]);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S2][0]);
                if callee::HUD_DRAW_TEXT_ELEM_W.call(c, 0x001f0d3c).is_err() { return; }
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[T0][0] = c.g[S3][0];
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32)).sw(c, c.g[S4][0]);
                c.g[A3][0] = 0x44;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[S2][0]);
                c.g[T1][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[S2][0]);
                c.g[T3][0] = 0u64;
                if callee::HUD_DRAW_TEXT_ELEM.call(c, 0x001f0d70).is_err() { return; }
                pc = 0x001f0d70; continue;
            }
            0x001f0d70 => {
                c.cycles += 6;
                c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
                let t = (c.g[V0][0] as i64) <= 0; 
                c.g[A1][0] = 0x3c;
                if t { pc = 0x001f0ea4; continue; }
                pc = 0x001f0d88; continue;
            }
            0x001f0d88 => {
                c.cycles += 6;
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5560));
                c.g[V0][0] = 0x3c;
                
                if c.g[A1][0] == 0u64 {  pc = 0x001f0da0; continue; }
                pc = 0x001f0da0; continue;
            }
            0x001f0da0 => {
                c.cycles += 24;
                c.g[A0][0] = 0x470000;
                c.mul_s(0, 0, 1);
                c.g[S0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8c)).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x13c0));
                c.cvt_w_s(1, 0);
                c.mfc1(2, 1);
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
                c.div(0, 2, 5);
                c.g[S2][0] = c.lo[0];
                c.g[S3][0] = c.hi[0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if callee::STR_HASH.call(c, 0x001f0dd4).is_err() { return; }
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, t, 0x001f0de4).is_err() { return; }
                c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[T0][0] = c.g[V0][0];
                c.mtc1(0, 1);
                c.g[A1][0] = 0u64;
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                pc = 0x001f0e00; continue;
            }
            0x001f0e00 => {
                c.cycles += 6;
                c.g[V0][0] = Addr((c.g[V1][0] as u32)).lw(c);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x470));
                c.c_le_s(1, 0);
                let t = c.fcc(); 
                c.g[A0][0] = 0x1;
                if t { pc = 0x001f0e1c; continue; }
                pc = 0x001f0e18; continue;
            }
            0x001f0e18 => {
                c.cycles += 1;
                c.g[A0][0] = 0u64;
                pc = 0x001f0e1c; continue;
            }
            0x001f0e1c => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64; 
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if t { pc = 0x001f0e30; continue; }
                pc = 0x001f0e24; continue;
            }
            0x001f0e24 => {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x87c)).lw(c);
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8c)).lw(c); if back_edge_unwinds(c, 0x001f0c30) { return; } pc = 0x001f0c30; continue; }
                pc = 0x001f0e30; continue;
            }
            0x001f0e30 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x001f0e00) { return; } pc = 0x001f0e00; continue; }
                pc = 0x001f0e3c; continue;
            }
            0x001f0e3c => {
                c.cycles += 26;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x428)).lw(c);
                c.g[V0][0] = 0x7f;
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A3][0] = 0x47;
                c.g[T1][0] = 0u64;
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_TEXT_ELEM_W.call(c, 0x001f0e70).is_err() { return; }
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.g[A0][0] = c.g[S6][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                c.g[A1][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, 0u64);
                c.g[T0][0] = 0u64;
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, 0u64);
                c.g[T1][0] = 0u64;
                c.g[T3][0] = 0x48;
                if callee::HUD_DRAW_TIME_ELEM.call(c, 0x001f0ea4).is_err() { return; }
                pc = 0x001f0ea4; continue;
            }
            0x001f0ea4 => {
                c.cycles += 1;
                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x330)).lw(c);
                pc = 0x001f0ea8; continue;
            }
            0x001f0ea8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                if t { pc = 0x001f1068; continue; }
                pc = 0x001f0eb0; continue;
            }
            0x001f0eb0 => {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3cc)).lw(c);
                c.g[V0][0] = c.g[V1][0] & 0x4;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                if t { pc = 0x001f0f3c; continue; }
                pc = 0x001f0ec0; continue;
            }
            0x001f0ec0 => {
                c.cycles += 21;
                c.lwc1(1, (c.g[S6][0] as u32).wrapping_add(0x3d0));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.g[T1][0] = c.g[V0][0] & 0x1;
                c.g[V1][0] = 0x4d0000;
                Addr((c.g[SP][0] as u32)).sw(c, 0u64);
                c.c_le_s(0, 1);
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                Addr((c.g[SP][0] as u32).wrapping_add(0x10)).sw(c, 0u64);
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7978));
                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[T0][0] as u32));
                c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x58));
                let t = !c.fcc(); 
                c.g[V0][0] = 0u64;
                if t { pc = 0x001f0f18; continue; }
                pc = 0x001f0f14; continue;
            }
            0x001f0f14 => {
                c.cycles += 1;
                c.g[V0][0] = c.g[A0][0];
                pc = 0x001f0f18; continue;
            }
            0x001f0f18 => {
                c.cycles += 9;
                c.g[A1][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x17c)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x180)).lw(c);
                c.g[T3][0] = 0x1a;
                c.g[A3][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x184)).lw(c);
                c.g[T0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x188)).lw(c);
                c.g[T2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                Addr((c.g[SP][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
                if callee::HUD_DRAW_TIME_ELEM.call(c, 0x001f0f3c).is_err() { return; }
                pc = 0x001f0f3c; continue;
            }
            0x001f0f3c => {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x3cc)).lw(c);
                c.g[V1][0] = 0x80000;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x4d0000;
                if t { pc = 0x001f1064; continue; }
                pc = 0x001f0f50; continue;
            }
            0x001f0f50 => {
                c.cycles += 16;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x164));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 1);
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7878));
                c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[T4][0] as u32));
                c.ldl(10, (c.g[T4][0] as u32).wrapping_add(0xf));
                c.ldr(10, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.c_lt_s(1, 0);
                let t = !c.fcc(); 
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                if t { pc = 0x001f0fb8; continue; }
                pc = 0x001f0f90; continue;
            }
            0x001f0f90 => {
                c.cycles += 10;
                c.g[V0][0] = 0x4d0000;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7858));
                c.ldl(13, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.ldr(13, (c.g[A0][0] as u32));
                c.ldl(14, (c.g[A0][0] as u32).wrapping_add(0xf));
                c.ldr(14, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x57));
                c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x5f));
                c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x58));
                pc = 0x001f0fb8; continue;
            }
            0x001f0fb8 => {
                c.cycles += 43;
                c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x894));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                if callee::HUD_GET_ELEM_POS.call(c, 0x001f0fd0).is_err() { return; }
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x4a4)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                if callee::HUD_GET_ELEM_SIZE.call(c, 0x001f0fe4).is_err() { return; }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                if callee::HUD_GET_ELEM_SCALE.call(c, 0x001f0ff0).is_err() { return; }
                c.g[A2][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8b4)).lb(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[T2][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8b5)).lb(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = c.g[S2][0];
                c.g[T3][0] = 0u64;
                if callee::HUD_ALIGN_ELEM_POS.call(c, 0x001f101c).is_err() { return; }
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[T2][0] = 0xfffffffffffffc1f;
                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x324)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8b6)).lb(c);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A3][0] = c.g[S1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[T1][0] = c.g[S3][0];
                c.g[V1][0] = c.g[V1][0] & 0x3e0;
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4a4)).lw(c);
                if callee::HUD_DRAW_SPRITE.call(c, 0x001f1064).is_err() { return; }
                pc = 0x001f1064; continue;
            }
            0x001f1064 => {
                c.cycles += 1;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                pc = 0x001f1068; continue;
            }
            0x001f1068 => {
                c.cycles += 14;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).lw(c);
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                Addr((c.g[A0][0] as u32).wrapping_add(0xe84)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xb0)).lh(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xb4)).lw(c);
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if call_ptr(c, t, 0x001f1088).is_err() { return; }
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10d8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x100)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x104)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                if call_ptr(c, t, 0x001f10a0).is_err() { return; }
                pc = 0x001f10a0; continue;
            }
            0x001f10a0 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                pc = 0x001f10a4; continue;
            }
            0x001f10a4 => {
                c.cycles += 21;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                c.g[RA][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x380)).ld(c);
                c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x468));
                c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x460));
                c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x458));
                c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x450));
                c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x448));
                c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x440));
                c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x438));
                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x428));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x420));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
                c.pc = t; return;
            }
            _ => { c.jump_out(pc, 0x001ec3f8, 0x001f10f8); return; }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, fresh, seed_count, DATA, GAME_GP};
    use ssx3_ee as ee;

    const ENTRY: u32 = 0x001e_c3f8;
    /// Dummy singletons the renderer dereferences (zeroed RAM).
    const BE: u32 = DATA + 0x4_0000; // BELibrary instance (gp-0x1f04)
    const WC: u32 = DATA + 0x4_1000; // renderer/world-cache singleton (gp-0x854)
    const W: u32 = DATA + 0x4_2000; // world (gp-0x848)
    const RM: u32 = DATA + 0x4_2100; // its race manager (world+0x84)
    const RACE: u32 = DATA + 0x4_2200; // the current race (race manager+0xc)
    const TXT: u32 = DATA + 0x4_3000; // the text/glyph objects the HUD draws through
    const SEEDS: u32 = 120;

    /// The `self+0x3c4` element kinds the renderer switches on (0x1f07f0).
    const ELEM_KINDS: [u32; 4] = [0x8, 0x50, 0x3a, 0x9];

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// The 1P HUD state at `DATA` plus the globals the renderer dereferences:
    /// the BELibrary singleton (`gp-0x1f04`), the renderer (`gp-0x854`) and the
    /// world (`gp-0x848`) point at zeroed dummies, so all the real callees take
    /// their short paths and every element pointer resolves into RAM.  The
    /// flags, counters and timer values drive the branches; `machine` sets the
    /// fake-call hooks (an indirect target of 0 lands in the `miss` hook), a
    /// game-like `gp`, and an unwind at one of the fake calls for some seeds.
    fn setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, 0x001e_c3f8, 32);
        c.bus.write32(gp_at(-0x1f04), BE);
        c.bus.write32(gp_at(-0x854), WC);
        c.bus.write32(gp_at(-0x848), W);
        // HUD "visible" global: non-zero returns through the epilogue at once.
        c.bus.write32(gp_at(0x1450), if seed % 8 == 0 { 1 } else { 0 });
        // world -> race manager -> current race.  Every list the callees walk
        // is empty or one entry: the rider/entry counts are 0 or 1, and the
        // element records the renderer dereferences are all zero (a NULL record
        // pointer is what 0x1ecc40 turns into `s4 = 0`, i.e. a 0-length record
        // list at 0x1ed854/0x1ef6d8).
        c.bus.write32(W + 0x84, RM);
        c.bus.write32(RM + 0x84, RM);
        c.bus.write32(RM + 0xc, RACE);
        c.bus.write32(RM + 0x10, seed % 2);
        c.bus.write32(RACE + 0x78, 0);
        c.bus.write32(RACE + 0x7c, seed % 2);
        c.bus.write32(RACE + 0x1c, seed % 2);
        c.bus.write32(RACE + 0x18, 0);
        // Element / text objects (`self+0x42c` is the glyph object every text
        // element is drawn with).
        for (i, off) in [
            0x428u32, 0x42c, 0x4b8, 0x4bc, 0x4c0, 0x4c4, 0x4c8, 0x4f4, 0x4f8, 0x538,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(DATA + off, TXT + 0x100 * i as u32);
        }
        // Per-frame state: the flag word and the element list.  Only the
        // fading-text bit (0x10000) is left set sometimes.  The wrapped-text
        // elements (flags 0x20000/0x40000, and the per-element 0x4000 bit that
        // 0x1ed8fc tests) are kept off: their `HUD_DrawWrappedText(W)` wrapper
        // loops until the layout helper (0x392680) reports the end of the
        // string, and for a zeroed text buffer that helper hands back the
        // buffer pointer instead of 0, so the wrapper spins forever.  The
        // element array at `self+0x48` stays zero (a 2-entry list of empty
        // records, bounded by the byte index at 0x1f07b4), and the counters
        // that scale the bars are zero.
        c.bus.write32(DATA + 0x3cc, ((seed & 1) as u32) << 16);
        c.bus.write32(DATA + 0x3c0, 0);
        c.bus.write32(DATA + 0x3c4, ELEM_KINDS[(seed >> 1) as usize % ELEM_KINDS.len()]);
        c.bus.write32(DATA + 0x3c8, 0);
        c.bus.write32(DATA + 0x3d8, (seed >> 2) & 1);
        // Timer / fade values, read as floats (0x180/0x184/0x188 are integer
        // time components used by HUD_DrawTime, so they stay 0).
        for off in [0x190u32, 0x194, 0x198, 0xc94] {
            c.bus.write32(DATA + off, r.edge_f32());
        }
        // The name/mission wide strings are empty (0-terminated, no line to
        // wrap).
        c.bus.write16(DATA + 0x944, 0);
        c.bus.write16(DATA + 0xa48, 0);
        c.g[A0][0] = DATA as u64;
        c.g[S6][0] = DATA as u64;
        c.pc = entry;
    }

    /// A strict differential run: every GPR (caller-saved scratch included),
    /// every FPR and LO/HI must match.
    fn strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    #[test]
    fn c_ov_state_hud1p_render_matches() {
        strict(
            ee::s04::f_001ec3f8__generated,
            c_ov_state_hud1p_render,
            SEEDS,
            &|c, seed| setup(c, seed, ENTRY),
        );
    }

    /// The same comparison entered at each loop head, with the frame already
    /// open and the callee-saved registers holding loop state, as the trampoline
    /// re-enters the function after an event.
    ///
    /// NOTE: this test is intentionally absent.  A synthetic fixture cannot
    /// supply the loop state a real resume would carry: 0x1ec8c4/0x1ec91c are
    /// the bodies of `while (a0 != v0) { copy 0x20 bytes; }`, 0x1ef758 and
    /// 0x1f0718 step a byte counter by a register (`s2`/`s3`) and 0x1f0630 by
    /// `s7`, so arbitrary registers make those loops run to the cycle budget.
    /// The whole `match` is still there, so the port accepts every one of the
    /// 13 heads; only the fixture for entering mid-loop is unbounded.

    /// The fixture must actually walk the element list (otherwise the
    /// comparison above proves nothing).
    #[test]
    fn c_ov_state_hud1p_render_covers_paths() {
        let mut costs = std::collections::BTreeSet::new();
        for seed in 0..32 {
            let mut c = fresh();
            setup(&mut c, seed, ENTRY);
            ee::s04::f_001ec3f8__generated(&mut c);
            costs.insert(c.cycles);
        }
        assert!(
            costs.len() >= 15,
            "fixture reaches only {} distinct paths: {costs:?}",
            costs.len()
        );
    }
}
