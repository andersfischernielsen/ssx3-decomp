//! Work package D6: frontend UI, fonts & HUD
//! (`docs/investigation/phase2-candidates-2.md`; class notes in
//! `docs/investigation/naming.md`, "UI / front end").
//!
//! The UI engine runs a stack of `UIState`s (front-end menus, in-race
//! overlays, popups) through `UIStateMgr`: every state sits in one of the
//! manager's intrusive `UIList`s (node links at +0/+4, see
//! [`crate::state::ui_list_first`]) and moves through phases kept in bits
//! 8..13 of its flags word (+0x1c): Init, TransitionIn, OnActivate, active,
//! TransitionOut, OnDeactivate, Shutdown. The HUD draws layout elements
//! (0x24-byte records: short position and size, float scale, float4 colour)
//! with the font and sprite helpers, and the rider's trick metrics queue
//! `cHUDFX` pop-up messages.
//!
//! The replacements follow the contract in `phase2-overrides.md`: cycles
//! block by block, the registers the original leaves (FCC, v0/v1 and
//! temporaries), calls through [`crate::call`], and loop event checks as in
//! [`crate::events`] (structured loops deliver due events at the back edge;
//! `match pc` dispatchers are re-entered at their loop heads).

use crate::abi::{r32, ret, set_fcc, set_word};
use crate::call::{call_ptr, guest, Flow, Frame, VSlot};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001e_9290, hud_get_elem_scale, "HUD_GetElemScale"),
    (0x001e_91a8, hud_get_elem_color, "HUD_GetElemColor"),
    (0x001e_91f8, hud_get_elem_pos, "HUD_GetElemPos"),
    (0x001e_9220, hud_get_elem_size, "HUD_GetElemSize"),
    (0x0011_7008, c_hudfx_set_text, "cHUDFX_SetText"),
    (0x0011_79e0, rider_metrix_remove_hud_message, "RiderMetrix_RemoveHUDMessage"),
    (0x0039_f7b8, ui_state_mgr_set_parent_chain_visible, "UIStateMgr_SetParentChainVisible"),
    (0x0039_ebe0, ui_state_mgr_render, "UIStateMgr_Render"),
    (0x0039_e868, ui_state_update_screens, "UIState_UpdateScreens"),
    (0x0039_f100, ui_state_mgr_update, "UIStateMgr_Update"),
    (0x0039_eb60, ui_state_mgr_has_state_in_phase, "UIStateMgr_HasStateInPhase"),
    (0x0039_ea90, ui_state_mgr_restart_state_in_phase, "UIStateMgr_RestartStateInPhase"),
    (0x0039_f840, ui_state_mgr_close_all, "UIStateMgr_CloseAll"),
    (0x0039_f9d8, ui_state_mgr_find_state, "UIStateMgr_FindState"),
    (0x0039_fa88, ui_state_mgr_get_active_state, "UIStateMgr_GetActiveState"),
    (0x0039_8038, ui_engine_update, "UIEngine_Update"),
    (0x0039_7df8, ui_engine_render, "UIEngine_Render"),
    (0x0011_7048, c_hudfx_show, "cHUDFX_Show"),
    (0x0011_71a8, c_hudfx_show_number_delayed, "cHUDFX_ShowNumberDelayed"),
    (0x0011_7b88, rider_metrix_show_hud_number, "RiderMetrix_ShowHUDNumber"),
    (0x001f_10f8, hud_align_elem_pos, "HUD_AlignElemPos"),
    (0x0039_1cb0, font_draw_text, "Font_DrawText"),
    (0x0039_2908, font_draw_text_centered, "Font_DrawTextCentered"),
    (0x001f_1190, hud_draw_sprite, "HUD_DrawSprite"),
    (0x0039_1fb0, font_measure_text, "Font_MeasureText"),
];

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `lw` through the CPU load path (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

// ---------------------------------------------------------------------------
// HUD layout elements.

/// A HUD layout element (0x24 bytes, from the HUD layout files): screen
/// position and size in pixels as shorts, a float2 scale for the font or
/// sprite drawn there, and an RGBA colour as four floats.
#[derive(Clone, Copy)]
pub struct HudElem(pub Addr);

impl HudElem {
    /// `short x, y`.
    pub fn pos(self, i: u32) -> Addr {
        self.0.off(2 * i)
    }
    /// `short w, h` (negative: take the size from the texture).
    pub fn size(self, i: u32) -> Addr {
        self.0.off(4 + 2 * i)
    }
    /// `float sx, sy`.
    pub fn scale(self, i: u32) -> Addr {
        self.0.off(8 + 4 * i)
    }
    /// `float r, g, b, a`.
    pub fn color(self, i: u32) -> Addr {
        self.0.off(0x10 + 4 * i)
    }
}

/// `HUD_GetElemScale(elem, float out[2])` (0x1e9290): copies the element's
/// scale. 5 cycles; leaves the two floats in f0/f1.
pub fn hud_get_elem_scale(c: &mut Cpu) {
    c.cycles += 5;
    let (e, out) = (HudElem(Addr(r32(c, A0))), Addr(r32(c, A1)));
    for i in 0..2 {
        let v = e.scale(i).read_fpu(c);
        c.f[i as usize] = v.0;
        out.off(4 * i).write_fpu(c, v);
    }
    ret(c);
}

/// `HUD_GetElemColor(elem, float out[4])` (0x1e91a8): copies the element's
/// RGBA colour. 9 cycles; leaves b in f0 and a in f1.
pub fn hud_get_elem_color(c: &mut Cpu) {
    c.cycles += 9;
    let (e, out) = (HudElem(Addr(r32(c, A0))), Addr(r32(c, A1)));
    for i in 0..4 {
        let v = e.color(i).read_fpu(c);
        c.f[(i & 1) as usize] = v.0;
        out.off(4 * i).write_fpu(c, v);
    }
    ret(c);
}

/// `HUD_GetElemPos(elem, float out[2])` (0x1e91f8): the element's pixel
/// position as floats. 9 cycles; leaves y (as a sign-extended short) in v0
/// and as a float in f0.
pub fn hud_get_elem_pos(c: &mut Cpu) {
    c.cycles += 9;
    let (e, out) = (HudElem(Addr(r32(c, A0))), Addr(r32(c, A1)));
    for i in 0..2 {
        let v = e.pos(i).lh(c);
        c.g[V0][0] = v;
        let f = FpuF32::from_i32(v as i32);
        c.f[0] = f.0;
        out.off(4 * i).write_fpu(c, f);
    }
    ret(c);
}

/// `HUD_GetElemSize(elem, float out[2], const float* texSize)` (0x1e9220):
/// the element's size as floats; a negative dimension means "use the
/// texture's size" (`texSize[1]` / `texSize[2]`, when a texture is given;
/// otherwise the negative value stays). Leaves v0 = h, f1 = 0, f0 = h and
/// the FCC of the last `0 <= h` compare.
pub fn hud_get_elem_size(c: &mut Cpu) {
    let (e, out) = (HudElem(Addr(r32(c, A0))), Addr(r32(c, A1)));
    // Width: 8, then 2 (negative), then 2 (texture).
    c.cycles += 8;
    for i in 0..2u32 {
        if i == 1 {
            c.cycles += 7;
        }
        let v = e.size(i).lh(c);
        c.g[V0][0] = v;
        c.f[1] = 0;
        let f = FpuF32::from_i32(v as i32);
        c.f[0] = f.0;
        let non_negative = FpuF32::ZERO.le(f);
        set_fcc(c, non_negative);
        let slot = out.off(4 * i);
        if non_negative {
            slot.write_fpu(c, f);
        } else {
            c.cycles += 2;
            slot.write_fpu(c, FpuF32::ZERO);
            if c.g[A2][0] != 0 {
                c.cycles += 2;
                let t = Addr(at(c, A2, 4 + 4 * i)).read_fpu(c);
                c.f[0] = t.0;
                slot.write_fpu(c, t);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// Trick HUD messages (cHUDFX, RiderMetrix).

/// `cHUDFX_SetText(fx, const char* text)` (0x117008): copies the text into
/// the message's buffer at +0x18 (a null text gives an empty string).
/// Structured loop at 0x117020 (7 cycles per character); leaves the
/// pointers one past the copied text in a0/a1 and the last byte read in
/// v0 (signed) / v1 (unsigned).
pub fn c_hudfx_set_text(c: &mut Cpu) {
    const HEAD: u32 = 0x0011_7020;
    c.cycles += 2;
    set_word(c, A0, at(c, A0, 0x18));
    if c.g[A1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A1)).lb(c);
        c.g[V1][0] = Addr(r32(c, A1)).lbu(c);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 7;
                Addr(r32(c, A0)).sb(c, c.g[V1][0]);
                set_word(c, A1, at(c, A1, 1));
                set_word(c, A0, at(c, A0, 1));
                c.g[V0][0] = Addr(r32(c, A1)).lb(c);
                c.g[V1][0] = Addr(r32(c, A1)).lbu(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, HEAD);
                }
            }
        }
    }
    c.cycles += 2;
    Addr(r32(c, A0)).sb(c, 0);
    ret(c);
}

/// The rider's HUD message table (`RiderMetrix` +0x1b0, count +0x1b4):
/// 0x9c-byte message slots whose first word is the message id. Ids below
/// 34 have a fixed slot; later ones are searched from slot 35 on.
pub mod hud_messages {
    /// `RiderMetrix` field: the message table.
    pub const TABLE: u32 = 0x1b0;
    /// `RiderMetrix` field: number of slots in use.
    pub const COUNT: u32 = 0x1b4;
    /// Bytes per slot.
    pub const SLOT_SIZE: u32 = 0x9c;
    /// Ids below this have their own slot.
    pub const FIXED_IDS: i64 = 34;
    /// First searched slot.
    pub const FIRST_FREE: u32 = 35;
    /// The id that marks a slot as removed.
    pub const REMOVED: u64 = 0x34;
}

/// `RiderMetrix_RemoveHUDMessage(metrix, int id)` (0x1179e0): marks the
/// message slot holding `id` as removed: the fixed slot for ids below 34,
/// else every searched slot (35..count) whose id matches. Nothing happens
/// without a table. Structured loop at 0x117a28; the fixed case uses
/// `mult` (LO/HI change).
pub fn rider_metrix_remove_hud_message(c: &mut Cpu) {
    use hud_messages::*;
    const HEAD: u32 = 0x0011_7a28;
    c.cycles += 4;
    c.g[T0][0] = c.g[A1][0];
    c.g[A1][0] = lw(c, at(c, A0, TABLE));
    c.g[V0][0] = ((c.g[T0][0] as i64) < FIXED_IDS) as u64;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            // A fixed slot.
            c.cycles += 6;
            c.g[V0][0] = SLOT_SIZE as u64;
            c.g[V1][0] = REMOVED;
            c.mult(0, A0, T0, V0);
            set_word(c, V0, r32(c, A0).wrapping_add(r32(c, A1)));
            Addr(r32(c, V0)).sw(c, REMOVED);
            ret(c);
            return;
        }
        c.g[V0][0] = lw(c, at(c, A0, COUNT));
        c.cycles += 4;
        c.g[A2][0] = FIRST_FREE as u64;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A1][0] = REMOVED;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A3][0] = (FIRST_FREE * SLOT_SIZE) as u64;
            loop {
                c.cycles += 5;
                let table = lw(c, at(c, A0, TABLE));
                set_word(c, V0, r32(c, A3).wrapping_add(table as u32));
                c.g[V1][0] = lw(c, r32(c, V0));
                if c.g[V1][0] == c.g[T0][0] {
                    Addr(r32(c, V0)).sw(c, c.g[A1][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, at(c, A0, COUNT));
                set_word(c, A2, at(c, A2, 1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                set_word(c, A3, at(c, A3, SLOT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, HEAD);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// UIState / UIStateMgr.

/// `UIState` fields.
pub mod ui_state {
    /// Intrusive list node: `{prev +0, next +4}`.
    pub const NEXT: u32 = 0x4;
    /// The state's id (a `StrHash` of its name).
    pub const ID: u32 = 0xc;
    /// Flags: bit 3 visible, bit 5 active (popup on top), bits 8..13 phase.
    pub const FLAGS: u32 = 0x1c;
    /// The parent state (0 for a root).
    pub const PARENT: u32 = 0x20;
    /// Flag bit: drawn.
    pub const VISIBLE_BIT: u32 = 3;
    /// Phases below this one (TransitionOut and later) still take the
    /// visibility flag.
    pub const PHASE_LIMIT: u64 = 6;

    /// The phase field of a flags word.
    pub fn phase(flags: u64) -> u64 {
        ((flags as u32) >> 8) as u64 & 0x3f
    }
}

/// `UIStateMgr_SetParentChainVisible(mgr, state, bool visible, bool self)`
/// (0x39f7b8): sets the visible flag of every state from `state` up its
/// parent chain (each one that has a parent and has not reached phase 6),
/// and, when `self` is set, of the root the walk ends on too. Leaf;
/// structured loop at 0x39f7d8.
pub fn ui_state_mgr_set_parent_chain_visible(c: &mut Cpu) {
    use ui_state::*;
    const HEAD: u32 = 0x0039_f7d8;
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 3;
        let parent = lw(c, at(c, A1, PARENT));
        c.g[V0][0] = c.g[A2][0] & 1;
        if parent != 0 {
            c.cycles += 3;
            c.g[T1][0] = !(1u64 << VISIBLE_BIT);
            set_word(c, T0, (r32(c, V0)) << VISIBLE_BIT);
            c.g[A0][0] = lw(c, at(c, A1, FLAGS));
            loop {
                c.cycles += 5;
                let open = phase(c.g[A0][0]) < PHASE_LIMIT;
                c.g[V0][0] = c.g[A0][0] & c.g[T1][0];
                if open {
                    c.cycles += 2;
                    c.g[V0][0] |= c.g[T0][0];
                    Addr(at(c, A1, FLAGS)).sw(c, c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[A1][0] = lw(c, at(c, A1, NEXT));
                c.g[V0][0] = lw(c, at(c, A1, PARENT));
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = lw(c, at(c, A1, FLAGS));
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = lw(c, at(c, A1, FLAGS));
            let open = phase(c.g[A0][0]) < PHASE_LIMIT;
            c.g[V0][0] = open as u64;
            c.g[V1][0] = c.g[A2][0] & 1;
            if open {
                c.cycles += 5;
                c.g[V0][0] = c.g[A0][0] & !(1u64 << VISIBLE_BIT);
                set_word(c, V1, r32(c, V1) << VISIBLE_BIT);
                c.g[V0][0] |= c.g[V1][0];
                Addr(at(c, A1, FLAGS)).sw(c, c.g[V0][0]);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// Direct callees of the state manager.
mod callee {
    use crate::call::Callee;
    use ssx3_ee as ee;
    pub const UI_LIST_FIRST: Callee = Callee { addr: 0x0039_7840, f: ee::s13::UIList_First };
    pub const UI_LIST_NODE_IS_SENTINEL: Callee =
        Callee { addr: 0x0039_7718, f: ee::s13::UIListNode_IsSentinel };
    pub const UI_SCREEN_UPDATE: Callee = Callee { addr: 0x0039_d778, f: ee::s13::UIScreen_Update };
    pub const UI_STATE_UPDATE_SCREENS: Callee =
        Callee { addr: 0x0039_e868, f: ee::s13::UIState_UpdateScreens };
    pub const UI_STATE_MGR_PROCESS_STATES: Callee =
        Callee { addr: 0x0039_ecb0, f: ee::s13::UIStateMgr_ProcessStates };
    pub const UI_STATE_MGR_UPDATE: Callee = Callee { addr: 0x0039_f100, f: ee::s13::UIStateMgr_Update };
    pub const UI_STATE_MGR_RENDER: Callee = Callee { addr: 0x0039_ebe0, f: ee::s13::UIStateMgr_Render };
}
use callee::*;

/// `UIStateMgr` lists: background states at +0, the state stack at +0x1c.
pub mod ui_state_mgr {
    pub const BACKGROUND: u32 = 0x0;
    pub const STACK: u32 = 0x1c;
}

/// `UIState` virtual slots (vptr at +8; byte offsets into the vtable).
mod ui_state_vt {
    pub const VPTR: u32 = 0x8;
    /// Slot 12, `Update`.
    pub const UPDATE: u32 = 0x60;
    /// Slot 15, `Render`.
    pub const RENDER: u32 = 0x78;
}

/// `s0 = state->next` (the walk's step; `lw`).
#[inline]
fn step_next(c: &mut Cpu) {
    c.g[S0][0] = lw(c, at(c, S0, ui_state::NEXT));
}

/// A virtual call on the state in s0: `vptr` in v0, the method in v1, the
/// adjusted `this` in a0, as the originals leave them.
fn call_state_method(c: &mut Cpu, slot: u32, ret_to: u32) -> Flow {
    let vt = lw(c, at(c, S0, ui_state_vt::VPTR));
    c.g[V0][0] = vt;
    let m = VSlot::read(c, vt, slot);
    c.g[V1][0] = m.func;
    c.g[A0][0] = m.this(c.g[S0][0]);
    call_ptr(c, m.func as u32, ret_to)
}

/// `UIListNode_IsSentinel(s0)` returning to `ret_to`; true when the walk
/// has come round to the list's anchor.
#[inline]
fn at_sentinel(c: &mut Cpu, ret_to: u32) -> Result<bool, crate::call::Unwound> {
    c.g[A0][0] = c.g[S0][0];
    UI_LIST_NODE_IS_SENTINEL.call(c, ret_to)?;
    Ok(c.g[V0][0] != 0)
}

/// Return addresses and loop head of one `UIStateMgr_Render` list pass.
struct RenderPass {
    after_render: u32,
    after_sentinel: u32,
    head: u32,
}

/// One render pass over the list whose first state is in s0: calls
/// `Render` (slot 15) on every visible state. Structured loop.
fn render_visible(c: &mut Cpu, p: &RenderPass) -> Flow {
    c.g[V0][0] = lw(c, at(c, S0, ui_state::FLAGS));
    loop {
        c.cycles += 4;
        set_word(c, V0, ((r32(c, V0) as i32) >> ui_state::VISIBLE_BIT) as u32);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            call_state_method(c, ui_state_vt::RENDER, p.after_render)?;
        }
        step_next(c);
        c.cycles += 4;
        if at_sentinel(c, p.after_sentinel)? {
            return Ok(());
        }
        c.g[V0][0] = lw(c, at(c, S0, ui_state::FLAGS));
        if event_due(c) {
            deliver_event_here(c, p.head);
        }
    }
}

/// `UIStateMgr_Render(mgr)` (0x39ebe0): draws the visible background
/// states, then the visible states of the stack (bottom to top), through
/// their `Render` method. Frame 0x30 (s1 +0x10, s0 +0x20).
pub fn ui_state_mgr_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        UI_LIST_FIRST.call(c, 0x0039_ebf8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            render_visible(
                c,
                &RenderPass { after_render: 0x0039_ec2c, after_sentinel: 0x0039_ec38, head: 0x0039_ec08 },
            )?;
        }
        c.cycles += 5;
        set_word(c, A0, at(c, S1, ui_state_mgr::STACK));
        UI_LIST_FIRST.call(c, 0x0039_ec48)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            render_visible(
                c,
                &RenderPass { after_render: 0x0039_ec7c, after_sentinel: 0x0039_ec88, head: 0x0039_ec58 },
            )?;
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIState_UpdateScreens(state)` (0x39e868): runs `UIScreen_Update` on
/// every screen of the state (list at +0x24). Frame 0x20 (s0 +0x10);
/// structured loop at 0x39e888.
pub fn ui_state_update_screens(c: &mut Cpu) {
    const HEAD: u32 = 0x0039_e888;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        set_word(c, A0, at(c, A0, 0x24));
        UI_LIST_FIRST.call(c, 0x0039_e87c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                UI_SCREEN_UPDATE.call(c, 0x0039_e890)?;
                step_next(c);
                if at_sentinel(c, 0x0039_e89c)? {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIStateMgr_Update(mgr)` (0x39f100): advances the state machine
/// (`UIStateMgr_ProcessStates`, a second time when the first pass changed
/// something), then for every state of the stack calls `Update` (slot 12)
/// if it is the active one (flag bit 5) and updates its screens. Frame 0x20
/// (s0 +0x10); structured loop at 0x39f140.
pub fn ui_state_mgr_update(c: &mut Cpu) {
    const HEAD: u32 = 0x0039_f140;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_STATE_MGR_PROCESS_STATES.call(c, 0x0039_f114)?;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            UI_STATE_MGR_PROCESS_STATES.call(c, 0x0039_f124)?;
        }
        c.cycles += 5;
        set_word(c, A0, at(c, S0, ui_state_mgr::STACK));
        UI_LIST_FIRST.call(c, 0x0039_f12c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = lw(c, at(c, S0, ui_state::FLAGS));
            loop {
                c.cycles += 4;
                set_word(c, V0, ((r32(c, V0) as i32) >> 5) as u32);
                c.g[V0][0] &= 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    call_state_method(c, ui_state_vt::UPDATE, 0x0039_f164)?;
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                UI_STATE_UPDATE_SCREENS.call(c, 0x0039_f16c)?;
                step_next(c);
                if at_sentinel(c, 0x0039_f178)? {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = lw(c, at(c, S0, ui_state::FLAGS));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Whether the state whose flags are in v1 is in phase s1 and not
/// suspended (flag bit 6). Leaves the tested field in v0. 4 (+4) cycles.
#[inline]
fn in_phase(c: &mut Cpu) -> bool {
    c.cycles += 4;
    c.g[V0][0] = ui_state::phase(c.g[V1][0]);
    if c.g[V0][0] != c.g[S1][0] {
        return false;
    }
    c.cycles += 4;
    set_word(c, V0, ((r32(c, V1) as i32) >> 6) as u32);
    c.g[V0][0] &= 1;
    c.g[V0][0] == 0
}

/// `bool UIStateMgr_HasStateInPhase(UIList* list, int8 phase)` (0x39eb60):
/// whether some state of the list is in `phase` and not suspended (flag
/// bit 6). Frame 0x30 (s1 +0x10, s0 +0x20); structured loop at 0x39eb90.
pub fn ui_state_mgr_has_state_in_phase(c: &mut Cpu) {
    const HEAD: u32 = 0x0039_eb90;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        set_word(c, A1, r32(c, A1) << 24);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        set_word(c, S1, ((r32(c, A1) as i32) >> 24) as u32);
        UI_LIST_FIRST.call(c, 0x0039_eb7c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0;
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = lw(c, at(c, S0, ui_state::FLAGS));
            loop {
                if in_phase(c) {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break;
                }
                step_next(c);
                c.cycles += 4;
                if at_sentinel(c, 0x0039_ebc0)? {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                c.g[V1][0] = lw(c, at(c, S0, ui_state::FLAGS));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
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

/// `bool UIStateMgr_RestartStateInPhase(UIList* list, int8 phase)`
/// (0x39ea90): the first state of the list in `phase` (and not suspended)
/// is made visible and active and sent back to phase 2 (TransitionIn) with
/// flag bit 4 set; returns whether one was found. The bit masks live in
/// s2..s5. Frame 0x70; structured loop at 0x39ead8.
pub fn ui_state_mgr_restart_state_in_phase(c: &mut Cpu) {
    const HEAD: u32 = 0x0039_ead8;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        set_word(c, A1, r32(c, A1) << 24);
        for (reg, off) in [(S1, 0x50), (S0, 0x60), (S2, 0x40), (S3, 0x30), (S4, 0x20), (S5, 0x10)] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0);
        set_word(c, S1, ((r32(c, A1) as i32) >> 24) as u32);
        UI_LIST_FIRST.call(c, 0x0039_eabc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[S2][0] = !0x8;
        let mut found = false;
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[S3][0] = !0x20;
            c.g[S4][0] = !0x3f00;
            c.g[S5][0] = !0x10;
            c.g[V1][0] = lw(c, at(c, S0, ui_state::FLAGS));
            loop {
                if in_phase(c) {
                    c.cycles += 11;
                    let mut f = c.g[V1][0];
                    f = (f & c.g[S2][0]) | 0x8; // visible
                    f = (f & c.g[S3][0]) | 0x20; // active
                    f = (f & c.g[S4][0]) | 0x200; // phase 2
                    f = (f & c.g[S5][0]) | 0x10;
                    c.g[V1][0] = f;
                    c.g[V0][0] = 1;
                    Addr(at(c, S0, ui_state::FLAGS)).sw(c, f);
                    found = true;
                    break;
                }
                step_next(c);
                c.cycles += 4;
                if at_sentinel(c, 0x0039_eb2c)? {
                    break;
                }
                c.g[V1][0] = lw(c, at(c, S0, ui_state::FLAGS));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        for (reg, off) in [(S0, 0x60), (S1, 0x50), (S2, 0x40), (S3, 0x30), (S4, 0x20), (S5, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Stages of the `match pc` dispatchers below (block addresses of the
/// originals that the walk moves between).
#[derive(Clone, Copy, PartialEq, Debug)]
enum Walk {
    /// The node test at the loop head (a resume point).
    Test,
    /// `UIListNode_IsSentinel` on s0.
    Sentinel,
    /// The second list (FindState).
    Second,
    /// The node test of the second list (a resume point).
    Test2,
    /// `UIListNode_IsSentinel` on s0, second list.
    Sentinel2,
    /// Epilogue.
    Return,
}

/// `UIStateMgr_CloseAll(UIList* list)` (0x39f840): starts closing every
/// state of the list: phase 7 (TransitionOut), flag bits 7 and 5 set. The
/// masks live in s1..s3. A `match pc` dispatcher in the original: the loop
/// head 0x39f878 (v0 = the state's flags, s0 = the state) is a resume
/// point. Frame 0x50.
pub fn ui_state_mgr_close_all(c: &mut Cpu) {
    const ENTRY: u32 = 0x0039_f840;
    const HEAD: u32 = 0x0039_f878;
    const END: u32 = 0x0039_f8c8;
    let frame = Frame { size: 0x50 };
    let mut stage = match c.pc {
        ENTRY => {
            c.cycles += 11;
            Frame::open(c, 0x50);
            for (reg, off) in [(S0, 0x40), (S1, 0x30), (S2, 0x20), (S3, 0x10)] {
                frame.save(c, reg, off);
            }
            frame.save_ra(c, 0);
            if UI_LIST_FIRST.call(c, 0x0039_f860).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[S3][0] = !0x3f00;
            if c.g[S0][0] == 0 {
                Walk::Return
            } else {
                c.cycles += 3;
                c.g[S2][0] = !0x80;
                c.g[S1][0] = !0x20;
                Walk::Sentinel
            }
        }
        HEAD => Walk::Test,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    loop {
        stage = match stage {
            Walk::Test => {
                c.cycles += 9;
                c.g[V1][0] = lw(c, at(c, S0, ui_state::NEXT));
                let mut f = c.g[V0][0];
                f = (f & c.g[S3][0]) | 0x700; // phase 7
                f = (f & c.g[S2][0]) | 0x80;
                f = (f & c.g[S1][0]) | 0x20;
                c.g[V0][0] = f;
                Addr(at(c, S0, ui_state::FLAGS)).sw(c, f);
                c.g[S0][0] = c.g[V1][0];
                Walk::Sentinel
            }
            Walk::Sentinel => {
                c.cycles += 4;
                match at_sentinel(c, 0x0039_f8a4) {
                    Err(_) => return,
                    Ok(true) => Walk::Return,
                    Ok(false) => {
                        c.g[V0][0] = lw(c, at(c, S0, ui_state::FLAGS));
                        if back_edge_unwinds(c, HEAD) {
                            return;
                        }
                        Walk::Test
                    }
                }
            }
            _ => break,
        };
    }
    c.cycles += 7;
    for (reg, off) in [(S0, 0x40), (S1, 0x30), (S2, 0x20), (S3, 0x10)] {
        frame.restore(c, reg, off);
    }
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `UIState* UIStateMgr_FindState(mgr, int id)` (0x39f9d8): the state with
/// `id` (+0xc), searched among the background states, then the stack;
/// null if none. A `match pc` dispatcher: the id tests at 0x39fa10 (v0 =
/// id, first list) and 0x39fa48 (second list) are resume points; s0 =
/// state, s1 = id, s2 = mgr. Frame 0x40.
pub fn ui_state_mgr_find_state(c: &mut Cpu) {
    const ENTRY: u32 = 0x0039_f9d8;
    const HEAD: u32 = 0x0039_fa10;
    const HEAD2: u32 = 0x0039_fa48;
    const END: u32 = 0x0039_fa84;
    let frame = Frame { size: 0x40 };
    let mut stage = match c.pc {
        ENTRY => {
            c.cycles += 11;
            Frame::open(c, 0x40);
            frame.save(c, S2, 0x10);
            frame.save(c, S1, 0x20);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S0, 0x30);
            frame.save_ra(c, 0);
            c.g[S1][0] = c.g[A1][0];
            if UI_LIST_FIRST.call(c, 0x0039_f9f8).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                Walk::Second
            } else {
                c.cycles += 2;
                Walk::Sentinel
            }
        }
        HEAD => Walk::Test,
        HEAD2 => Walk::Test2,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    loop {
        stage = match stage {
            Walk::Test => {
                c.cycles += 2;
                let hit = c.g[V0][0] == c.g[S1][0];
                c.g[V0][0] = c.g[S0][0];
                if hit {
                    Walk::Return
                } else {
                    c.cycles += 1;
                    step_next(c);
                    Walk::Sentinel
                }
            }
            Walk::Sentinel => {
                c.cycles += 4;
                match at_sentinel(c, 0x0039_fa24) {
                    Err(_) => return,
                    Ok(true) => Walk::Second,
                    Ok(false) => {
                        c.g[V0][0] = lw(c, at(c, S0, ui_state::ID));
                        if back_edge_unwinds(c, HEAD) {
                            return;
                        }
                        Walk::Test
                    }
                }
            }
            Walk::Second => {
                c.cycles += 5;
                set_word(c, A0, at(c, S2, ui_state_mgr::STACK));
                if UI_LIST_FIRST.call(c, 0x0039_fa34).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 0;
                if c.g[S0][0] == 0 {
                    Walk::Return
                } else {
                    c.cycles += 2;
                    Walk::Sentinel2
                }
            }
            Walk::Test2 => {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S0, ui_state::ID));
                if c.g[V0][0] != c.g[S1][0] {
                    step_next(c);
                    Walk::Sentinel2
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    Walk::Return
                }
            }
            Walk::Sentinel2 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                if UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_fa64).is_err() {
                    return;
                }
                let end = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if end {
                    Walk::Return
                } else {
                    if back_edge_unwinds(c, HEAD2) {
                        return;
                    }
                    Walk::Test2
                }
            }
            Walk::Return => break,
        };
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `UIState* UIStateMgr_GetActiveState(UIList* list)` (0x39fa88): the
/// first state of the list with the active flag (bit 5), or null. A
/// `match pc` dispatcher: the flag test at 0x39fab0 (s0 = state) is a
/// resume point. Frame 0x20 (s0 +0x10).
pub fn ui_state_mgr_get_active_state(c: &mut Cpu) {
    const ENTRY: u32 = 0x0039_fa88;
    const HEAD: u32 = 0x0039_fab0;
    const END: u32 = 0x0039_fae8;
    let frame = Frame { size: 0x20 };
    let mut stage = match c.pc {
        ENTRY => {
            c.cycles += 8;
            Frame::open(c, 0x20);
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            if UI_LIST_FIRST.call(c, 0x0039_fa9c).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0;
            if c.g[S0][0] == 0 {
                Walk::Return
            } else {
                c.cycles += 2;
                Walk::Sentinel
            }
        }
        HEAD => Walk::Test,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    loop {
        stage = match stage {
            Walk::Test => {
                c.cycles += 5;
                let flags = lw(c, at(c, S0, ui_state::FLAGS));
                let active = (flags as u32 as i32 >> 5) & 1 != 0;
                c.g[V0][0] = c.g[S0][0];
                if active {
                    Walk::Return
                } else {
                    c.cycles += 1;
                    step_next(c);
                    Walk::Sentinel
                }
            }
            Walk::Sentinel => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                if UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_fad0).is_err() {
                    return;
                }
                let end = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if end {
                    Walk::Return
                } else {
                    if back_edge_unwinds(c, HEAD) {
                        return;
                    }
                    Walk::Test
                }
            }
            _ => break,
        };
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// UIEngine.

/// `UIEngine` fields (alloc 0x70, `GameModule` +0x48).
pub mod ui_engine {
    /// `OVUIInput*`.
    pub const INPUT: u32 = 0xc;
    /// The embedded `UIStateMgr`.
    pub const STATE_MGR: u32 = 0x18;
    /// Vtable offset of the input object's per-frame poll (slot 46).
    pub const INPUT_POLL: u32 = 0x170;
}

/// The renderer the UI draws with (`gp-0x854`) and its render-state stack.
pub mod ui_renderer {
    /// `gp - 0x854`: the renderer.
    pub const GP_OFF: u32 = 0x854;
    /// Renderer field: the current 0x14-byte render-state record.
    pub const STATE_TOP: u32 = 0xe84;
    /// Renderer field: its vptr.
    pub const VPTR: u32 = 0x10d8;
    /// Bytes per render-state record.
    pub const STATE_SIZE: u32 = 0x14;
    /// The UI's default render state (copied over the pushed record).
    pub const UI_STATE: u32 = 0x0050_1420;
    /// The UI's view transform, passed to `SetView`.
    pub const UI_VIEW: u32 = 0x004f_f1a0;
    /// Vtable offsets of the methods used (slot * 8).
    pub const FLUSH: u32 = 0xf8;
    pub const BEGIN_2D: u32 = 0xa8;
    pub const SET_ORTHO: u32 = 0xd0;
    pub const SET_VIEW: u32 = 0x110;
    pub const END_2D: u32 = 0xb0;
    pub const FLUSH_UI: u32 = 0x100;
}

/// `UIEngine_Update(engine)` (0x398038): polls the UI input (input object
/// slot 46), then runs the state manager. 16 cycles plus the callees.
pub fn ui_engine_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let input = lw(c, at(c, S0, ui_engine::INPUT));
        c.g[A1][0] = input;
        let vt = lw(c, (input as u32).wrapping_add(8));
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, ui_engine::INPUT_POLL);
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(input);
        call_ptr(c, m.func as u32, 0x0039_8060)?;
        set_word(c, A0, at(c, S0, ui_engine::STATE_MGR));
        UI_STATE_MGR_UPDATE.call(c, 0x0039_8068)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The renderer `gp-0x854` (`lw`).
#[inline]
fn ui_renderer(c: &mut Cpu) -> u64 {
    let a = r32(c, GP).wrapping_sub(ui_renderer::GP_OFF);
    lw(c, a)
}

/// A renderer method call: the renderer in `obj_reg`, its vptr in
/// `vt_reg`, the method in `fn_reg`, `this` in a0 (as the original
/// leaves them). The renderer is loaded first.
fn renderer_call(c: &mut Cpu, obj_reg: usize, vt_reg: usize, fn_reg: usize, slot: u32, ret_to: u32) -> Flow {
    let r = ui_renderer(c);
    c.g[obj_reg][0] = r;
    renderer_call_on(c, r, vt_reg, fn_reg, slot, ret_to)
}

/// [`renderer_call`] with the renderer already loaded.
fn renderer_call_on(c: &mut Cpu, r: u64, vt_reg: usize, fn_reg: usize, slot: u32, ret_to: u32) -> Flow {
    let vt = lw(c, (r as u32).wrapping_add(ui_renderer::VPTR));
    c.g[vt_reg][0] = vt;
    let m = VSlot::read(c, vt, slot);
    c.g[fn_reg][0] = m.func;
    c.g[A0][0] = m.this(r);
    call_ptr(c, m.func as u32, ret_to)
}

/// Copy a 0x14-byte render-state record (two unaligned doublewords and a
/// word, through v1/a0/`last`), all loads before the stores.
fn copy_render_state(c: &mut Cpu, from: u32, to: u32, last: usize) {
    let a = Addr(from).read_u64_unaligned(c);
    c.g[V1][0] = a;
    let b = Addr(from.wrapping_add(8)).read_u64_unaligned(c);
    c.g[A0][0] = b;
    c.g[last][0] = lw(c, from.wrapping_add(0x10));
    Addr(to).write_u64_unaligned(c, a);
    Addr(to.wrapping_add(8)).write_u64_unaligned(c, b);
    Addr(to.wrapping_add(0x10)).sw(c, c.g[last][0]);
}

/// `UIEngine_Render(engine)` (0x397df8): draws the UI in 2D. Pushes a copy
/// of the renderer's current render state and replaces it with the UI's
/// default state, flushes and begins 2D, sets a 640x480 orthographic
/// projection (depth -1..1) and the UI view, sets the UI render-state bits
/// (z test/write off, alpha blending), renders the states
/// (`UIStateMgr_Render`), ends 2D, flushes and pops the render state.
/// One 135-cycle block; frame 0x20 (s0 +0x10).
pub fn ui_engine_render(c: &mut Cpu) {
    use ui_renderer::*;
    guest(c, |c| {
        c.cycles += 135;
        let frame = Frame::open(c, 0x20);
        let r = ui_renderer(c);
        c.g[A1][0] = r;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // Push: duplicate the top record.
        let top = lw(c, (r as u32).wrapping_add(STATE_TOP));
        c.g[V0][0] = top;
        copy_render_state(c, top as u32, (top as u32).wrapping_add(STATE_SIZE), A3);
        let top = lw(c, (r as u32).wrapping_add(STATE_TOP));
        c.g[A0][0] = ui_renderer(c);
        set_word(c, V1, (top as u32).wrapping_add(STATE_SIZE));
        Addr((r as u32).wrapping_add(STATE_TOP)).sw(c, c.g[V1][0]);
        // ...and make it the UI's default state.
        let top = lw(c, r32(c, A0).wrapping_add(STATE_TOP));
        c.g[V0][0] = top;
        c.g[A3][0] = UI_STATE as u64;
        copy_render_state(c, UI_STATE, top as u32, A1);

        renderer_call(c, A1, V0, V1, FLUSH, 0x0039_7e94)?;
        renderer_call(c, A1, V0, V1, BEGIN_2D, 0x0039_7eac)?;

        // SetOrtho(0, 0, 640, 480, -1, 1, a1 = 0, a2 = 1).
        let r = ui_renderer(c);
        c.g[A3][0] = r;
        c.g[A2][0] = 1;
        c.f[12] = 0;
        c.g[A1][0] = 0;
        c.f[13] = 0;
        for (f, bits) in [(14, 0x4420_0000u32), (15, 0x43f0_0000), (16, 0xbf80_0000), (17, 0x3f80_0000)] {
            c.g[AT][0] = sx(bits);
            c.f[f] = bits;
        }
        renderer_call_on(c, r, V1, V0, SET_ORTHO, 0x0039_7ef4)?;

        let r = ui_renderer(c);
        c.g[A2][0] = r;
        c.g[A1][0] = UI_VIEW as u64;
        renderer_call_on(c, r, V0, V1, SET_VIEW, 0x0039_7f14)?;

        // UI render-state bits in the current record.
        let r = ui_renderer(c);
        c.g[T2][0] = r;
        c.g[V1][0] = !0x40_0000;
        c.g[A1][0] = 0x40_0000;
        c.g[A2][0] = !0x180_0000;
        c.g[T3][0] = 0x100_0000;
        c.g[T0][0] = !0x30_0000;
        c.g[T1][0] = !0xf_f000;
        c.g[A3][0] = 0x1_4000;
        c.g[T4][0] = !0x7c;
        let set_bits = |c: &mut Cpu, reg: usize, off: u32, clear: u64, set: u64| {
            let rec = lw(c, (r as u32).wrapping_add(STATE_TOP));
            c.g[reg][0] = rec;
            let v = (lw(c, (rec as u32).wrapping_add(off)) & clear) | set;
            c.g[V0][0] = v;
            Addr((rec as u32).wrapping_add(off)).sw(c, v);
        };
        set_bits(c, A0, 4, !0x40_0000, 0x40_0000);
        set_word(c, A0, at(c, S0, ui_engine::STATE_MGR));
        set_bits(c, A1, 4, !0x180_0000, 0x100_0000);
        set_bits(c, V1, 4, !0x30_0000, 0);
        set_bits(c, A1, 4, !0xf_f000, 0x1_4000);
        set_bits(c, A2, 4, !0x7c, 0x14);
        set_bits(c, V1, 0, u64::MAX, 0xc);
        UI_STATE_MGR_RENDER.call(c, 0x0039_7fc4)?;

        renderer_call(c, A1, V0, V1, END_2D, 0x0039_7fdc)?;
        renderer_call(c, A1, V0, V1, FLUSH_UI, 0x0039_7ff4)?;

        // Pop.
        let r = ui_renderer(c);
        c.g[A0][0] = r;
        frame.restore(c, S0, 0x10);
        let top = lw(c, (r as u32).wrapping_add(STATE_TOP));
        frame.restore_ra(c, 0);
        set_word(c, V0, (top as u32).wrapping_sub(STATE_SIZE));
        Addr((r as u32).wrapping_add(STATE_TOP)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Trick HUD message display (cHUDFX) and HUD element placement.

/// `cHUDFX` fields: a trick-HUD pop-up message.
pub mod hud_fx {
    /// Message type.
    pub const TYPE: u32 = 0x0;
    /// Display timer (-1: waiting for the delay to run out).
    pub const TIMER: u32 = 0x4;
    /// Delay before showing (stored negated).
    pub const DELAY: u32 = 0x8;
    /// Style word (colour / layout).
    pub const STYLE: u32 = 0xc;
    /// The number shown (`ShowNumber*`).
    pub const NUMBER: u32 = 0x14;
    /// The text buffer.
    pub const TEXT: u32 = 0x18;
}

const C_HUDFX_SET_TEXT: crate::call::Callee =
    crate::call::Callee { addr: 0x0011_7008, f: ssx3_ee::s00::cHUDFX_SetText };
const C_HUDFX_SHOW_NUMBER: crate::call::Callee =
    crate::call::Callee { addr: 0x0011_70a8, f: ssx3_ee::s00::cHUDFX_ShowNumber };
const RIDER_METRIX_FIND_HUD_SLOT: crate::call::Callee =
    crate::call::Callee { addr: 0x0011_7a58, f: ssx3_ee::s00::RiderMetrix_FindHUDSlot };
/// `sprintf` (libc, another shard: called through the table).
const SPRINTF: u32 = 0x0041_7828;
/// The shared number buffer and its `"%d"` format.
const NUMBER_TEXT: u32 = 0x004a_5e20;
const NUMBER_FORMAT: u32 = 0x004a_1070;

/// `sd`-style save of f20 (`swc1 f20, off(sp)`) and its reload.
fn save_f20(c: &mut Cpu, frame: Frame, off: u32) {
    let a = frame.at(c, off);
    Addr(a).write_u32(c, c.f[20]);
}
fn restore_f20(c: &mut Cpu, frame: Frame, off: u32) {
    let a = frame.at(c, off);
    c.f[20] = Addr(a).read_u32(c);
}

/// `cHUDFX_Show(fx, int type, const char* text, int style, float time)`
/// (0x117048): shows `text` at once for `time`. 24 cycles; frame 0x50
/// (s0-s2, f20 +0x40).
pub fn c_hudfx_show(c: &mut Cpu) {
    use hud_fx::*;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        save_f20(c, frame, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.f[20] = c.f[12];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        C_HUDFX_SET_TEXT.call(c, 0x0011_7078)?;
        let fx = r32(c, S0);
        Addr(fx + TYPE).sw(c, c.g[S1][0]);
        Addr(fx + TIMER).write_u32(c, c.f[20]);
        Addr(fx + STYLE).sw(c, c.g[S2][0]);
        Addr(fx + NUMBER).sw(c, 0);
        Addr(fx + DELAY).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        restore_f20(c, frame, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `cHUDFX_ShowNumberDelayed(fx, int type, int number, int style, float
/// delay)` (0x1171a8): formats `number` with `"%d"` into the shared buffer,
/// copies it as the text, and queues the message: timer -1, delay stored
/// negated. 39 cycles; frame 0x70 (s0-s4, f20 +0x60).
pub fn c_hudfx_show_number_delayed(c: &mut Cpu) {
    use hud_fx::*;
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x70);
        for (reg, off) in [(S0, 0x50), (S4, 0x10), (S1, 0x40), (S2, 0x30), (S3, 0x20)] {
            frame.save(c, reg, off);
        }
        save_f20(c, frame, 0x60);
        frame.save_ra(c, 0);
        c.g[S0][0] = NUMBER_TEXT as u64;
        c.g[S4][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.f[20] = c.f[12];
        c.g[A0][0] = NUMBER_TEXT as u64;
        c.g[A1][0] = NUMBER_FORMAT as u64;
        call_ptr(c, SPRINTF, 0x0011_71f4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        C_HUDFX_SET_TEXT.call(c, 0x0011_7200)?;
        c.f[20] = (-FpuF32(c.f[20])).0;
        c.g[AT][0] = sx(0xbf80_0000);
        c.f[0] = 0xbf80_0000;
        let fx = r32(c, S1);
        Addr(fx + NUMBER).sw(c, c.g[S2][0]);
        Addr(fx + TYPE).sw(c, c.g[S4][0]);
        Addr(fx + DELAY).write_u32(c, c.f[20]);
        Addr(fx + STYLE).sw(c, c.g[S3][0]);
        Addr(fx + TIMER).write_u32(c, c.f[0]);
        for (reg, off) in [(S0, 0x50), (S1, 0x40), (S2, 0x30), (S3, 0x20), (S4, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        restore_f20(c, frame, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `int RiderMetrix_ShowHUDNumber(metrix, int id, int number, int style,
/// float time)` (0x117b88): finds the message slot for `id`
/// (`RiderMetrix_FindHUDSlot`, same arguments) and shows `number` there
/// (`cHUDFX_ShowNumber`); returns the slot, or -1 without a message table.
/// Frame 0x70 (s0-s4, f20 +0x60); the slot offset uses `mult`.
pub fn rider_metrix_show_hud_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        for (reg, off) in [(S1, 0x40), (S2, 0x30), (S3, 0x20), (S4, 0x10)] {
            frame.save(c, reg, off);
        }
        save_f20(c, frame, 0x60);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A3][0];
        c.g[V0][0] = lw(c, at(c, S1, hud_messages::TABLE));
        c.f[20] = c.f[12];
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 14;
            RIDER_METRIX_FIND_HUD_SLOT.call(c, 0x0011_7bcc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = hud_messages::SLOT_SIZE as u64;
            c.mult(0, A0, S0, A0);
            c.g[V0][0] = lw(c, at(c, S1, hud_messages::TABLE));
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.f[12] = c.f[20];
            c.g[A3][0] = c.g[S4][0];
            set_word(c, A0, r32(c, V0).wrapping_add(r32(c, A0)));
            C_HUDFX_SHOW_NUMBER.call(c, 0x0011_7bf4)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        for (reg, off) in [(S0, 0x50), (S1, 0x40), (S2, 0x30), (S3, 0x20), (S4, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        restore_f20(c, frame, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `HUD_AlignElemPos(int align, float out[2], const float pos[2], const
/// float size[2], const float scale[2] (t0), hAlign (t1), vAlign (t2),
/// flags (t3))` (0x1f10f8): the element's screen position, x from
/// 0x21e750 (mirrored around 640 by the flags) and y from 0x21e7a8, each
/// from (pos, size, scale) of its axis. One 37-cycle block; frame 0x70
/// (s0-s5).
pub fn hud_align_elem_pos(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x70);
        for (reg, off) in [(S0, 0x60), (S1, 0x50), (S2, 0x40), (S3, 0x30), (S4, 0x20), (S5, 0x10)] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[T2][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.g[S4][0] = c.g[T0][0];
        c.g[S5][0] = c.g[A1][0];
        c.g[A1][0] = c.g[T1][0];
        c.g[A2][0] = c.g[T3][0];
        for axis in 0..2u32 {
            if axis == 1 {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
            }
            for (f, reg) in [(12, S2), (13, S3), (14, S4)] {
                c.f[f] = Addr(at(c, reg, 4 * axis)).read_u32(c);
            }
            let (func, ret_to) = if axis == 0 { (0x0021_e750, 0x001f_1148) } else { (0x0021_e7a8, 0x001f_1164) };
            call_ptr(c, func, ret_to)?;
            Addr(at(c, S5, 4 * axis)).write_u32(c, c.f[0]);
        }
        for (reg, off) in [(S0, 0x60), (S1, 0x50), (S2, 0x40), (S3, 0x30), (S4, 0x20), (S5, 0x10)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Fonts.

/// `Font` fields.
pub mod font {
    /// Glyph offset x, y (scaled by the font size).
    pub const OFFSET: u32 = 0x18;
    /// Spacing x, y (passed to the renderer).
    pub const SPACING: u32 = 0x20;
    /// Drop-shadow offset x, y (no shadow when both are 0).
    pub const SHADOW: u32 = 0x28;
    /// Size (scale) x, y.
    pub const SIZE: u32 = 0x38;
    /// Text colour.
    pub const COLOR: u32 = 0x40;
    /// Shadow colour.
    pub const SHADOW_COLOR: u32 = 0x50;
    /// Draw depth (copied into the render state).
    pub const DEPTH: u32 = 0x60;
}

/// The font renderer (`gp+0x2a90`), its render-state record field holding
/// the draw layer (bits 5..9 of +8), its draw-depth field (+0x10) and the
/// `DrawText` method (vtable +0x280).
mod font_renderer {
    pub const GP_OFF: u32 = 0x2a90;
    pub const LAYER_WORD: u32 = 0x8;
    pub const LAYER_MASK: u64 = 0x3e0;
    pub const DEPTH: u32 = 0x10;
    pub const DRAW_TEXT: u32 = 0x280;
}

/// The renderer's `DrawText(renderer, font, text, colour, x, y, sx, sy,
/// spacing x, y, z)`: method from `vt_reg`'s vtable into `fn_reg`, the
/// floats from the position record at `pos` (x, y, z) and the font.
fn font_draw_call(c: &mut Cpu, fn_reg: usize, pos: u32, font_base: u32, ret_to: u32) -> Flow {
    use font::*;
    let rend = c.g[S3][0];
    let vt = c.g[V0][0];
    let m = VSlot::read(c, vt, font_renderer::DRAW_TEXT);
    c.g[A0][0] = m.this(rend);
    c.g[fn_reg][0] = m.func;
    for (f, a) in [
        (14, font_base + SIZE),
        (15, font_base + SIZE + 4),
        (16, font_base + SPACING),
        (17, font_base + SPACING + 4),
        (18, pos + 8),
    ] {
        c.f[f] = Addr(a).read_u32(c);
    }
    call_ptr(c, m.func as u32, ret_to)
}

/// `Font_DrawText(font, const char* text, float x, float y)` (0x391cb0):
/// draws `text` at (x, y) + the font's glyph offset (scaled by its size),
/// in the font's colour and depth; with a drop shadow (at the shadow offset,
/// shadow colour, one layer below: layer bits 5..9 of the render state,
/// clamped at 0, restored afterwards) when the shadow offset is non-zero.
/// Frame 0x70 (s0-s4, ra +0x10; the position {x, y, 0, 0} at +0).
pub fn font_draw_text(c: &mut Cpu) {
    use font::*;
    use font_renderer::*;
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        for (reg, off) in [(S3, 0x30), (S2, 0x40), (S4, 0x20), (S0, 0x60), (S1, 0x50)] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        let f = r32(c, S2);
        let rend = lw(c, r32(c, GP).wrapping_add(GP_OFF));
        c.g[S3][0] = rend;
        c.g[V1][0] = lw(c, f + font::DEPTH);
        let state = lw(c, (rend as u32).wrapping_add(ui_renderer::STATE_TOP));
        c.g[V0][0] = state;
        Addr((state as u32).wrapping_add(font_renderer::DEPTH)).sh(c, c.g[V1][0]);
        // pos = {x + offset.x * size.x, y + offset.y * size.y, 0, 0}
        let pos = frame.at(c, 0);
        Addr(pos + 8).sw(c, 0);
        let size_x = Addr(f + SIZE).read_fpu(c);
        let off_x = Addr(f + OFFSET).read_fpu(c);
        let off_y = Addr(f + OFFSET + 4).read_fpu(c);
        let size_y = Addr(f + SIZE + 4).read_fpu(c);
        c.f[3] = size_y.0;
        let dx = off_x * size_x;
        c.f[0] = dx.0;
        let shadow_x = Addr(f + SHADOW).read_fpu(c);
        c.f[2] = shadow_x.0;
        let dy = off_y * size_y;
        c.f[1] = dy.0;
        let zero = Addr(pos + 8).read_fpu(c);
        c.f[4] = zero.0;
        let x = FpuF32(c.f[12]) + dx;
        c.f[12] = x.0;
        Addr(pos + 0xc).sw(c, 0);
        let mut shadow = !shadow_x.eq_s(zero);
        set_fcc(c, !shadow);
        let y = FpuF32(c.f[13]) + dy;
        c.f[13] = y.0;
        Addr(pos).write_fpu(c, x);
        Addr(pos + 4).write_fpu(c, y);
        if !shadow {
            c.cycles += 5;
            let shadow_y = Addr(f + SHADOW + 4).read_fpu(c);
            c.f[0] = shadow_y.0;
            shadow = !shadow_y.eq_s(zero);
            set_fcc(c, !shadow);
        }
        if shadow {
            c.cycles += 40;
            let state = lw(c, (rend as u32).wrapping_add(ui_renderer::STATE_TOP));
            c.g[T0][0] = state;
            c.g[S1][0] = !LAYER_MASK;
            c.g[A1][0] = c.g[S2][0];
            let word = lw(c, (state as u32).wrapping_add(LAYER_WORD));
            c.g[A2][0] = c.g[S4][0];
            set_word(c, A3, f + SHADOW_COLOR);
            // One layer below (clamped at 0).
            set_word(c, S0, ((word & LAYER_MASK) as u32) >> 5);
            c.g[V1][0] = word & c.g[S1][0];
            let below = r32(c, S0).wrapping_sub(1);
            set_word(c, V0, below);
            let above_floor = -1 < c.g[V0][0] as i64;
            c.g[A0][0] = above_floor as u64;
            if !above_floor {
                c.g[V0][0] = 0;
            }
            set_word(c, V0, r32(c, V0) << 5);
            c.g[V0][0] &= LAYER_MASK;
            c.g[V1][0] |= c.g[V0][0];
            Addr((state as u32).wrapping_add(LAYER_WORD)).sw(c, c.g[V1][0]);
            c.g[V0][0] = lw(c, (rend as u32).wrapping_add(ui_renderer::VPTR));
            let px = Addr(pos).read_fpu(c);
            let sx = Addr(f + SHADOW).read_fpu(c);
            let py = Addr(pos + 4).read_fpu(c);
            let sy = Addr(f + SHADOW + 4).read_fpu(c);
            c.f[0] = px.0;
            c.f[1] = py.0;
            c.f[12] = (px + sx).0;
            c.f[13] = (py + sy).0;
            font_draw_call(c, T0, pos, f, 0x0039_1dbc)?;
            // Restore the layer.
            let state = lw(c, (rend as u32).wrapping_add(ui_renderer::STATE_TOP));
            c.g[V1][0] = state;
            set_word(c, S0, r32(c, S0) << 5);
            let v = (lw(c, (state as u32).wrapping_add(LAYER_WORD)) & c.g[S1][0]) | c.g[S0][0];
            c.g[V0][0] = v;
            Addr((state as u32).wrapping_add(LAYER_WORD)).sw(c, v);
        }
        c.g[V0][0] = lw(c, (rend as u32).wrapping_add(ui_renderer::VPTR));
        c.cycles += 22;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S4][0];
        set_word(c, A3, f + COLOR);
        c.f[12] = Addr(pos).read_u32(c);
        c.f[13] = Addr(pos + 4).read_u32(c);
        font_draw_call(c, V1, pos, f, 0x0039_1e10)?;
        for (reg, off) in [(S0, 0x60), (S1, 0x50), (S2, 0x40), (S3, 0x30), (S4, 0x20)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

const FONT_MEASURE_TEXT: crate::call::Callee =
    crate::call::Callee { addr: 0x0039_1fb0, f: ssx3_ee::s12::Font_MeasureText };
const FONT_DRAW_TEXT: crate::call::Callee =
    crate::call::Callee { addr: 0x0039_1cb0, f: ssx3_ee::s12::Font_DrawText };

/// `Font_DrawTextCentered(font, const char* text, float x, float y)`
/// (0x392908): measures the text at the font's size and draws it with its
/// centre at x (`x - width * 0.5`). One 30-cycle block; frame 0x40 (s0,
/// s1, f20/f21 +0x30/+0x38).
pub fn font_draw_text_centered(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let (a21, a20) = (frame.at(c, 0x38), frame.at(c, 0x30));
        Addr(a21).write_u32(c, c.f[21]);
        Addr(a20).write_u32(c, c.f[20]);
        c.f[21] = c.f[12];
        c.f[20] = c.f[13];
        c.f[12] = Addr(at(c, S0, font::SIZE)).read_u32(c);
        c.f[13] = Addr(at(c, S0, font::SIZE + 4)).read_u32(c);
        FONT_MEASURE_TEXT.call(c, 0x0039_2944)?;
        c.g[AT][0] = 0x3f00_0000;
        c.f[1] = 0x3f00_0000;
        c.g[A0][0] = c.g[S0][0];
        c.f[13] = c.f[20];
        c.g[A1][0] = c.g[S1][0];
        let half = FpuF32(c.f[0]) * FpuF32(c.f[1]);
        c.f[0] = half.0;
        c.f[12] = (FpuF32(c.f[21]) - half).0;
        FONT_DRAW_TEXT.call(c, 0x0039_2964)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        let (a21, a20) = (frame.at(c, 0x38), frame.at(c, 0x30));
        c.f[21] = Addr(a21).read_u32(c);
        c.f[20] = Addr(a20).read_u32(c);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD sprites.

/// A HUD texture record: draw depth (+0, copied into the render state) and
/// its UV rectangle (v0 +0xc, u0 +0x10, u1 +0x14, v1 +0x18).
pub mod hud_texture {
    pub const DEPTH: u32 = 0x0;
    pub const UV_V0: u32 = 0xc;
    pub const UV_U0: u32 = 0x10;
    pub const UV_U1: u32 = 0x14;
    pub const UV_V1: u32 = 0x18;
}

/// `HUD_DrawSprite(a0, HudTexture* tex, const float pos[2], const float
/// size[2], const float scale[2] (t0), const float color[4] (t1))`
/// (0x1f1190): draws a textured quad (renderer method +0x238, 4 vertices)
/// covering `pos .. pos + size * scale` in the given colour (0..1 floats
/// scaled to 0..128 ints). Nothing without a texture.
///
/// The quad is built on the stack as four 0x30-byte vertex records at sp
/// +0x00/+0x30/+0x60/+0x90: UV at +0, 1.0 at +8, colour ints at +0x10..+0x1c
/// (G, B, A, R), position {x, y, 0, 1} at +0x20, assembled through a
/// scratch quadword at sp+0xc0 with `lq`/`sq` (so v0/v1 keep its upper
/// lanes). Two structured loops: an empty 4-step one at 0x1f11c0 (what is
/// left of the vertex constructors) and the colour fill at 0x1f1238.
/// Frame 0xe0 (ra +0xd0).
pub fn hud_draw_sprite(c: &mut Cpu) {
    use hud_texture::*;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xe0);
        c.g[T2][0] = c.g[A1][0]; // texture
        frame.save_ra(c, 0xd0);
        c.g[T3][0] = c.g[A2][0]; // pos
        c.g[T4][0] = c.g[A3][0]; // size
        if c.g[T2][0] != 0 {
            c.cycles += 6;
            let rend = ui_renderer(c);
            c.g[A0][0] = rend;
            c.g[A1][0] = 3;
            c.g[V1][0] = lw(c, r32(c, T2) + DEPTH);
            c.g[A2][0] = u64::MAX;
            let state = lw(c, (rend as u32).wrapping_add(ui_renderer::STATE_TOP));
            c.g[V0][0] = state;
            Addr((state as u32).wrapping_add(0x10)).sh(c, c.g[V1][0]);
            // The empty vertex-constructor loop.
            loop {
                c.cycles += 7;
                set_word(c, A1, r32(c, A1).wrapping_sub(1));
                c.g[V1][0] = c.g[SP][0];
                if c.g[A1][0] == c.g[A2][0] {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001f_11c0);
                }
            }
            // Colour as ints 0..128.
            c.cycles += 23;
            let k128 = FpuF32(0x4300_0000);
            c.g[AT][0] = 0x4300_0000;
            c.f[1] = k128.0;
            let col = r32(c, T1);
            let b = Addr(col + 8).read_fpu(c) * k128;
            set_word(c, A3, frame.at(c, 0xc0));
            let a = Addr(col + 0xc).read_fpu(c) * k128;
            let r = Addr(col).read_fpu(c) * k128;
            let g = Addr(col + 4).read_fpu(c) * k128;
            c.g[AT][0] = 0x3f80_0000;
            c.f[5] = 0x3f80_0000;
            // cvt.w.s / mfc1 into t1 (b), a2 (r), a1 (g), a0 (a).
            let int = |v: FpuF32| v.to_i32() as u32;
            for (reg, v) in [(T1, b), (A2, r), (A1, g), (A0, a)] {
                c.g[reg][0] = sx(int(v));
            }
            c.f[0] = int(a);
            c.f[1] = int(g);
            c.f[2] = r.0;
            c.f[3] = g.0;
            c.f[4] = a.0;
            Addr(r32(c, V1) + 0x1c).sw(c, c.g[A2][0]);
            loop {
                c.cycles += 8;
                let v = r32(c, V1);
                Addr(v + 0x10).sw(c, c.g[A1][0]);
                Addr(v + 0x14).sw(c, c.g[T1][0]);
                Addr(v + 0x18).sw(c, c.g[A0][0]);
                Addr(v + 8).write_u32(c, c.f[5]);
                set_word(c, V1, v.wrapping_add(0x30));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                Addr(r32(c, V1) + 0x1c).sw(c, c.g[A2][0]);
                if event_due(c) {
                    deliver_event_here(c, 0x001f_1238);
                }
            }
            // Corners and UVs.
            c.cycles += 52;
            let (scale, pos, size, tex) = (r32(c, T0), r32(c, T3), r32(c, T4), r32(c, T2));
            let sc_x = Addr(scale).read_fpu(c);
            c.g[A1][0] = 4;
            let w = Addr(size).read_fpu(c);
            set_word(c, A2, frame.at(c, 0));
            let y0 = Addr(pos + 4).read_fpu(c);
            c.g[A3][0] = 0;
            let x0 = Addr(pos).read_fpu(c);
            let sc_y = Addr(scale + 4).read_fpu(c);
            let h = Addr(size + 4).read_fpu(c);
            c.g[AT][0] = 0x3f80_0000;
            c.f[6] = 0x3f80_0000;
            let x1 = x0 + w * sc_x;
            let y1 = y0 + h * sc_y;
            let scratch = frame.at(c, 0xc0);
            let s = |c: &mut Cpu, off: u32, v: u32| Addr(scratch + off).write_u32(c, v);
            s(c, 0, x0.0);
            s(c, 8, 0);
            s(c, 4, y0.0);
            s(c, 0xc, 0x3f80_0000);
            let v1_uv = Addr(tex + UV_V1).read_u32(c);
            c.lq(V1, scratch); // (x0, y0)
            s(c, 0, x1.0);
            s(c, 8, 0);
            let u0 = Addr(tex + UV_U0).read_u32(c);
            let v0_uv = Addr(tex + UV_V0).read_u32(c);
            let u1 = Addr(tex + UV_U1).read_u32(c);
            c.lq(V0, scratch); // (x1, y0)
            s(c, 0, x0.0);
            s(c, 4, y1.0);
            s(c, 8, 0);
            let sp = frame.at(c, 0);
            c.sq(V1, sp + 0x20);
            c.lq(V1, scratch); // (x0, y1)
            s(c, 0, x1.0);
            c.sq(V0, sp + 0x50);
            for (off, v) in [
                (0x34, v0_uv),
                (0x60, u0),
                (0x90, u1),
                (0x94, v1_uv),
                (0x00, u0),
                (0x04, v0_uv),
                (0x30, u1),
                (0x64, v1_uv),
            ] {
                Addr(sp + off).write_u32(c, v);
            }
            s(c, 8, 0);
            c.sq(V1, sp + 0x80);
            c.lq(V0, scratch); // (x1, y1)
            let rend = ui_renderer(c);
            c.g[T0][0] = rend;
            c.sq(V0, sp + 0xb0);
            for (f, v) in [(0, v0_uv), (1, u1), (2, y1.0), (3, x1.0), (4, x0.0), (5, u0), (7, v1_uv)] {
                c.f[f] = v;
            }
            renderer_call_on(c, rend, V1, V0, 0x238, 0x001f_1328)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// Font glyph tables (`Font` +0..+0x10) and glyph records (12 bytes).
pub mod glyphs {
    /// Number of glyph records.
    pub const COUNT: u32 = 0x0;
    /// The glyph records: first the dense range `FIRST..=LAST` (indexed by
    /// code), then the extra glyphs sorted by code (binary searched).
    pub const TABLE: u32 = 0x4;
    /// The glyph used for codes without one.
    pub const DEFAULT: u32 = 0x8;
    pub const FIRST: u32 = 0xc;
    pub const LAST: u32 = 0x10;
    /// Bytes per glyph record.
    pub const SIZE: u32 = 12;
    /// Glyph record fields: code (u16), advance (u8), height (u8), advance
    /// offset, left, top (s8).
    pub const CODE: u32 = 0x0;
    pub const ADVANCE: u32 = 0x2;
    pub const HEIGHT: u32 = 0x3;
    pub const ADVANCE_OFFSET: u32 = 0x8;
    pub const LEFT: u32 = 0x9;
    pub const TOP: u32 = 0xa;
}

/// FPU register access for the measuring code (which keeps its state in
/// f0..f7 like the original).
#[inline(always)]
fn fr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}
#[inline(always)]
fn set_fr(c: &mut Cpu, r: usize, v: FpuF32) {
    c.f[r] = v.0;
}
/// `mtc1 reg; cvt.s.w`.
#[inline(always)]
fn cvt_reg(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32::from_i32(c.g[r][0] as u32 as i32)
}

/// `float Font_MeasureText(font, const char* text, float rect[4], bool
/// fixedAdvance (a3), float sx, float sy)` (0x391fb0): the bounding box
/// {x0, y0, x1, y1} of `text` drawn at the origin at size (sx, sy), into
/// `rect` (or a scratch rect on the stack), widened by the font's drop
/// shadow; returns x1 in f0.
///
/// Per character: the glyph is the dense-range record (`FIRST..=LAST`),
/// a binary search over the sorted extra glyphs for codes above `LAST`
/// (falling back to `DEFAULT`), none below `FIRST`. The pen advances by the
/// glyph's advance offset (or its advance width unless `fixedAdvance`).
/// Structured loops at 0x391ff0 (characters) and 0x392048 (the search);
/// the record offsets use `mult` (LO/HI change). Frame 0x10 (the scratch
/// rect).
pub fn font_measure_text(c: &mut Cpu) {
    use glyphs::*;
    const CHAR_HEAD: u32 = 0x0039_1ff0;
    const SEARCH_HEAD: u32 = 0x0039_2048;
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[T5][0] = c.g[A3][0];
    c.g[T3][0] = if c.g[A2][0] != 0 { c.g[A2][0] } else { c.g[SP][0] };
    let rect = r32(c, T3);
    for i in 0..4 {
        Addr(rect + 4 * i).sw(c, 0);
    }
    c.g[A2][0] = c.g[A1][0];
    c.f[6] = Addr(rect).read_u32(c); // pen x
    c.g[V0][0] = Addr(r32(c, A2)).lb(c);
    c.f[7] = c.f[6]; // pen y
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[T4][0] = SIZE as u64;
        c.g[V1][0] = Addr(r32(c, A2)).lbu(c);
        loop {
            // Find the glyph of the code in v1 (into t2; 0 for none).
            let font = r32(c, A0);
            c.cycles += 4;
            c.g[T2][0] = lw(c, font + FIRST);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
            set_word(c, A1, at(c, A2, 1));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[T2][0] = 0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = lw(c, font + LAST);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    // Dense range.
                    c.cycles += 7;
                    c.mult(0, V1, V1, T4);
                    c.mult(0, T2, T2, T4);
                    c.g[V0][0] = lw(c, font + TABLE);
                    set_word(c, A1, at(c, A2, 1));
                    set_word(c, V0, r32(c, V0).wrapping_add(r32(c, V1)));
                    set_word(c, T2, r32(c, V0).wrapping_sub(r32(c, T2)));
                } else {
                    // Binary search of the extra glyphs [last - first, count).
                    set_word(c, T2, r32(c, A1).wrapping_sub(r32(c, T2)));
                    c.cycles += 5;
                    c.g[T0][0] = lw(c, font + COUNT);
                    c.g[T6][0] = lw(c, font + TABLE);
                    set_word(c, A1, at(c, A2, 1));
                    c.g[T1][0] = SIZE as u64;
                    set_word(c, V0, r32(c, T2).wrapping_add(r32(c, T0)));
                    loop {
                        c.cycles += 6;
                        set_word(c, A3, ((r32(c, V0) as i32) >> 1) as u32);
                        c.mult(0, A2, A3, T1);
                        set_word(c, V0, r32(c, A2).wrapping_add(r32(c, T6)));
                        c.g[A2][0] = Addr(r32(c, V0) + CODE).lhu(c);
                        if c.g[V1][0] == c.g[A2][0] {
                            c.g[T2][0] = c.g[V0][0];
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            set_word(c, T2, at(c, A3, 1));
                        } else {
                            c.cycles += 1;
                            c.g[T0][0] = c.g[A3][0];
                        }
                        c.cycles += 3;
                        let more = (c.g[T2][0] as i64) < (c.g[T0][0] as i64);
                        c.g[V0][0] = more as u64;
                        set_word(c, V0, r32(c, T2).wrapping_add(r32(c, T0)));
                        if !more {
                            c.cycles += 1;
                            c.g[T2][0] = lw(c, font + DEFAULT);
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, SEARCH_HEAD);
                        }
                    }
                }
            }
            // Grow the rect by the glyph at the pen, advance the pen.
            c.cycles += 2;
            c.g[A2][0] = c.g[A1][0];
            if c.g[T2][0] != 0 {
                c.cycles += 31;
                let g = r32(c, T2);
                let (sx, sy) = (fr(c, 12), fr(c, 13));
                c.g[V0][0] = Addr(g + LEFT).lb(c);
                c.f[2] = Addr(rect).read_u32(c);
                let left = cvt_reg(c, V0);
                c.g[V1][0] = Addr(g + ADVANCE_OFFSET).lb(c);
                c.f[3] = Addr(rect + 4).read_u32(c);
                let adv_off = cvt_reg(c, V1);
                set_fr(c, 1, adv_off);
                c.f[4] = Addr(rect + 0xc).read_u32(c);
                set_fr(c, 5, adv_off * sx);
                let x0 = (fr(c, 6) + left * sx).min(fr(c, 2));
                Addr(rect).write_fpu(c, x0);
                c.g[V0][0] = Addr(g + TOP).lb(c);
                let y0 = (fr(c, 7) + cvt_reg(c, V0) * sy).min(fr(c, 3));
                Addr(rect + 4).write_fpu(c, y0);
                c.g[V0][0] = Addr(g + TOP).lb(c);
                c.g[V1][0] = Addr(g + HEIGHT).lbu(c);
                set_word(c, V0, r32(c, V0).wrapping_add(r32(c, V1)));
                let y1 = (fr(c, 7) + cvt_reg(c, V0) * sy).max(fr(c, 4));
                set_fr(c, 0, y1);
                Addr(rect + 0xc).write_fpu(c, y1);
                let right;
                if c.g[T5][0] == 0 {
                    c.cycles += 10;
                    c.g[V1][0] = Addr(g + ADVANCE).lbu(c);
                    set_fr(c, 2, fr(c, 6) + fr(c, 5));
                    c.g[V0][0] = Addr(g + LEFT).lb(c);
                    c.f[1] = Addr(rect + 8).read_u32(c);
                    set_word(c, V0, r32(c, V0).wrapping_add(r32(c, V1)));
                    right = fr(c, 6) + cvt_reg(c, V0) * sx;
                } else {
                    c.cycles += 3;
                    right = fr(c, 6) + fr(c, 5);
                    c.f[1] = Addr(rect + 8).read_u32(c);
                    set_fr(c, 2, right);
                }
                c.cycles += 4;
                let x1 = right.max(fr(c, 1));
                set_fr(c, 0, x1);
                Addr(rect + 8).write_fpu(c, x1);
                c.f[6] = c.f[2];
                c.g[A2][0] = c.g[A1][0];
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A2)).lb(c);
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, A2)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, CHAR_HEAD);
            }
        }
    }
    // Make x1/y1 extents, then widen by the drop shadow.
    c.cycles += 14;
    let font = r32(c, A0);
    let x0 = Addr(rect).read_fpu(c);
    let y0 = Addr(rect + 4).read_fpu(c);
    let w = Addr(rect + 8).read_fpu(c) + x0;
    let h = Addr(rect + 0xc).read_fpu(c) + y0;
    set_fr(c, 4, x0);
    set_fr(c, 1, y0);
    set_fr(c, 3, w);
    c.f[2] = 0;
    Addr(rect + 8).write_fpu(c, w);
    Addr(rect + 0xc).write_fpu(c, h);
    let shadow_x = Addr(font + 0x28).read_fpu(c);
    let x1 = if shadow_x.lt(FpuF32::ZERO) {
        set_fcc(c, true);
        c.cycles += 4;
        let x0 = x0 - shadow_x;
        set_fr(c, 1, x0);
        Addr(rect).write_fpu(c, x0);
        let shadow_x = Addr(font + 0x28).read_fpu(c);
        w - shadow_x
    } else {
        set_fcc(c, false);
        w + shadow_x
    };
    c.cycles += 7;
    Addr(rect + 8).write_fpu(c, x1);
    let shadow_y = Addr(font + 0x2c).read_fpu(c);
    set_fr(c, 1, shadow_y);
    c.f[0] = 0;
    if shadow_y.lt(FpuF32::ZERO) {
        set_fcc(c, true);
        c.cycles += 8;
        let y0 = Addr(rect + 4).read_fpu(c) - shadow_y;
        set_fr(c, 0, y0);
        let y1 = Addr(rect + 0xc).read_fpu(c);
        Addr(rect + 4).write_fpu(c, y0);
        let shadow_y = Addr(font + 0x2c).read_fpu(c);
        set_fr(c, 1, shadow_y);
        let y1 = y1 - shadow_y;
        set_fr(c, 2, y1);
        Addr(rect + 0xc).write_fpu(c, y1);
    } else {
        set_fcc(c, false);
        let y1 = Addr(rect + 0xc).read_fpu(c);
        c.cycles += 2;
        let y1 = y1 + shadow_y;
        set_fr(c, 0, y1);
        Addr(rect + 0xc).write_fpu(c, y1);
    }
    c.cycles += 3;
    c.f[0] = Addr(rect + 8).read_u32(c);
    let _ = frame;
    set_word(c, SP, r32(c, SP).wrapping_add(0x10));
    ret(c);
}

#[cfg(test)]
mod tests {
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check_paths, check_paths_lohi, fill, scramble, Rng, DATA};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 16 ^ salt)
    }

    /// A HUD element at DATA with edge-case shorts and floats, `out` at
    /// DATA+0x100, an optional texture-size record at DATA+0x200.
    fn hud_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        fill(c, &mut r, DATA, 0x100);
        for i in 0..2 {
            let v: i16 = match r.next_u32() % 4 {
                0 => -((r.next_u32() % 3) as i16),
                1 => r.next_u32() as i16,
                _ => (r.next_u32() % 700) as i16,
            };
            c.bus.write16(DATA + 4 + 2 * i, v as u16);
        }
        for i in 0..6 {
            let f = r.edge_f32();
            c.bus.write32(DATA + 8 + 4 * i, f);
        }
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x100) as u64;
        c.g[A2][0] = if r.next_u32() % 3 == 0 { 0 } else { (DATA + 0x200) as u64 };
    }

    #[test]
    fn hud_get_elem_scale() {
        check_paths(ssx3_ee::s04::f_001e9290__generated, super::hud_get_elem_scale, SEEDS, &[5], &|c, s| {
            hud_setup(c, s, 1)
        });
    }

    #[test]
    fn hud_get_elem_color() {
        check_paths(ssx3_ee::s04::f_001e91a8__generated, super::hud_get_elem_color, SEEDS, &[9], &|c, s| {
            hud_setup(c, s, 2)
        });
    }

    #[test]
    fn hud_get_elem_pos() {
        check_paths(ssx3_ee::s04::f_001e91f8__generated, super::hud_get_elem_pos, SEEDS, &[9], &|c, s| {
            hud_setup(c, s, 3)
        });
    }

    #[test]
    fn hud_get_elem_size() {
        // Width: 0 / +2 / +4; height likewise; base 8 + 7 + 2.
        check_paths(
            ssx3_ee::s04::f_001e9220__generated,
            super::hud_get_elem_size,
            SEEDS,
            &[17, 19, 21, 25],
            &|c, s| hud_setup(c, s, 4),
        );
    }

    #[test]
    fn c_hudfx_set_text() {
        check_events(ssx3_ee::s00::f_00117008__generated, super::c_hudfx_set_text, SEEDS, &|c, seed| {
            let mut r = rng(seed, 5);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x80);
            // A string of 0..40 non-zero bytes (some with the top bit set).
            let n = r.next_u32() % 40;
            for i in 0..n {
                c.bus.write8(DATA + 0x200 + i, (r.next_u32() % 255 + 1) as u8);
            }
            c.bus.write8(DATA + 0x200 + n, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if r.next_u32() % 8 == 0 { 0 } else { (DATA + 0x200) as u64 };
            schedule(c, &mut r, seed, 0x0011_7008)
        });
    }

    fn remove_hud_message_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = rng(seed, 6);
        scramble(c, &mut r);
        let table = DATA + 0x1000;
        let count = r.next_u32() % 45;
        c.bus.write32(DATA + 0x1b0, if r.next_u32() % 8 == 0 { 0 } else { table });
        c.bus.write32(DATA + 0x1b4, count);
        for i in 0..70 {
            c.bus.write32(table + 0x9c * i, r.next_u32() % 40);
            c.bus.write32(table + 0x9c * i + 4, r.next_u32());
        }
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = match r.next_u32() % 3 {
            0 => (r.next_u32() % 34) as u64,
            1 => 34 + (r.next_u32() % 6) as u64,
            _ => r.next_u32() as i32 as i64 as u64,
        };
        r
    }

    #[test]
    fn rider_metrix_remove_hud_message() {
        let (generated, hand) = (ssx3_ee::s00::f_001179e0__generated, super::rider_metrix_remove_hud_message);
        check_paths_lohi(generated, hand, SEEDS, &[6, 12, 24], &|c, s| {
            remove_hud_message_setup(c, s);
        });
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = remove_hud_message_setup(c, seed);
            // Mostly the searched path, which has the loop.
            if seed % 4 != 0 {
                c.g[A1][0] = 34 + (r.next_u32() % 6) as u64;
                c.bus.write32(DATA + 0x1b0, DATA + 0x1000);
                c.bus.write32(DATA + 0x1b4, 40 + r.next_u32() % 30);
            }
            schedule(c, &mut r, seed, 0x0011_79e0)
        });
    }

    /// A chain of states at DATA + 0x40*i: state i's "next" link (+4) is
    /// state i+1, parents (+0x20) set for the first `depth` states.
    fn chain_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = rng(seed, 7);
        scramble(c, &mut r);
        let depth = r.next_u32() % 12;
        for i in 0..14u32 {
            let s = DATA + 0x40 * i;
            c.bus.write32(s + 4, s + 0x40);
            c.bus.write32(s + 0x20, if i < depth { r.next_u32() | 1 } else { 0 });
            let phase = r.next_u32() % 9;
            c.bus.write32(s + 0x1c, (r.next_u32() & !0x3f00) | phase << 8);
        }
        c.g[A0][0] = r.next_u32() as u64;
        c.g[A1][0] = if r.next_u32() % 10 == 0 { 0 } else { DATA as u64 };
        c.g[A2][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
        c.g[A3][0] = (r.next_u32() % 2) as u64;
        r
    }

    #[test]
    fn ui_state_mgr_set_parent_chain_visible() {
        let (generated, hand) = (ssx3_ee::s13::f_0039f7b8__generated, super::ui_state_mgr_set_parent_chain_visible);
        // null; no parent (with / without self, phase open / closed); one
        // step of the chain (open / closed).
        check_paths(generated, hand, SEEDS, &[4, 9, 15, 20, 21, 23], &|c, s| {
            chain_setup(c, s);
        });
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = chain_setup(c, seed);
            schedule(c, &mut r, seed, 0x0039_f7b8)
        });
    }

    // ---------------------------------------------------------- UI lists

    use crate::difftest::events::{driven, Events};
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fresh, GAME_GP};
    use ssx3_ee_rt::funcs::GuestFn;

    /// The manager (background list at +0, stack at +0x1c).
    const MGR: u32 = DATA;
    /// A shared vtable: every slot 0..0x100 a distinct fake.
    const VT: u32 = DATA + 0xf000;

    /// Link `states` into the list at `list` (anchor link at +4), ending
    /// at the self-linked `sentinel`. Sometimes the list is a null link.
    fn ui_list(c: &mut Cpu, r: &mut Rng, list: u32, states: &[u32], sentinel: u32) {
        if states.is_empty() && r.next_u32() % 4 == 0 {
            c.bus.write32(list + 4, 0);
            return;
        }
        let mut prev = list;
        for &s in states {
            c.bus.write32(prev + 4, s);
            c.bus.write32(s, prev);
            prev = s;
        }
        c.bus.write32(prev + 4, sentinel);
        c.bus.write32(sentinel, prev);
        c.bus.write32(sentinel + 4, sentinel);
    }

    /// A random state at `s`: id from a small set, flags with a phase in
    /// 0..8 and random bits, vptr = [`VT`].
    fn ui_state(c: &mut Cpu, r: &mut Rng, s: u32) {
        c.bus.write32(s + 8, VT);
        c.bus.write32(s + 0xc, r.next_u32() % 6);
        let phase = r.next_u32() % 8;
        c.bus.write32(s + 0x1c, (r.next_u32() & !0x3f00) | phase << 8);
    }

    /// Both manager lists with 0..`max` states each (states 0x40 apart
    /// from DATA+0x100 / DATA+0x800), and the vtable.
    fn ui_lists(c: &mut Cpu, r: &mut Rng, max: u32) {
        for off in (0..0x100).step_by(8) {
            c.bus.write16(VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(VT + off + 4, FAKE + 2 * off);
        }
        for (list, base) in [(MGR, DATA + 0x100), (MGR + 0x1c, DATA + 0x800)] {
            let n = r.next_u32() % (max + 1);
            let states: Vec<u32> = (0..n).map(|i| base + 0x40 * i).collect();
            for &s in &states {
                ui_state(c, r, s);
            }
            ui_list(c, r, list, &states, base + 0x6c0);
        }
    }

    /// Require that `gen` returns each of `results` (v0) for some seed.
    fn require_v0(gen: GuestFn, seeds: u32, results: &[&dyn Fn(u64) -> bool], setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = vec![false; results.len()];
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            for (i, f) in results.iter().enumerate() {
                seen[i] |= f(c.g[V0][0]);
            }
        }
        assert!(seen.iter().all(|&s| s), "results seen: {seen:?}");
    }

    /// The common setup: scrambled machine with fakes, gp, lists.
    fn mgr_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32, max: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        ui_lists(c, &mut r, max);
        r
    }

    #[test]
    fn ui_state_mgr_render() {
        let (generated, hand) = (ssx3_ee::s13::f_0039ebe0__generated, super::ui_state_mgr_render);
        covered(generated, hand, SEEDS, &|c, seed| {
            mgr_setup(c, seed, 0x10, 6, 4);
            c.g[A0][0] = MGR as u64;
        });
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = mgr_setup(c, seed, 0x11, 0, 8);
            c.g[A0][0] = MGR as u64;
            schedule(c, &mut r, seed, 0x0039_ebe0)
        });
    }

    fn phase_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = mgr_setup(c, seed, salt, 0, 8);
        c.g[A0][0] = if seed % 2 == 0 { MGR } else { MGR + 0x1c } as u64;
        // Low byte: a phase (sometimes negative); garbage above.
        let p = if r.next_u32() % 8 == 0 { 0xfc | r.next_u32() % 4 } else { r.next_u32() % 9 };
        c.g[A1][0] = (r.next_u32() as u64) << 32 | (r.next_u32() & !0xff | p) as u64;
        r
    }

    #[test]
    fn ui_state_mgr_has_state_in_phase() {
        let (generated, hand) = (ssx3_ee::s13::f_0039eb60__generated, super::ui_state_mgr_has_state_in_phase);
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = phase_setup(c, seed, 0x20);
            schedule(c, &mut r, seed, 0x0039_eb60)
        });
        require_v0(generated, SEEDS, &[&|v| v == 0, &|v| v == 1], &|c, seed| {
            phase_setup(c, seed, 0x21);
        });
    }

    #[test]
    fn ui_state_mgr_restart_state_in_phase() {
        let (generated, hand) = (ssx3_ee::s13::f_0039ea90__generated, super::ui_state_mgr_restart_state_in_phase);
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = phase_setup(c, seed, 0x30);
            schedule(c, &mut r, seed, 0x0039_ea90)
        });
        require_v0(generated, SEEDS, &[&|v| v == 0, &|v| v == 1], &|c, seed| {
            phase_setup(c, seed, 0x31);
        });
    }

    driven!(gen_close_all, ssx3_ee::s13::f_0039f840__generated);
    driven!(hand_close_all, super::ui_state_mgr_close_all);

    #[test]
    fn ui_state_mgr_close_all() {
        check_events(gen_close_all, hand_close_all, SEEDS, &|c, seed| {
            let mut r = mgr_setup(c, seed, 0x40, 0, 10);
            c.g[A0][0] = if seed % 2 == 0 { MGR } else { MGR + 0x1c } as u64;
            schedule(c, &mut r, seed, 0x0039_f840)
        });
    }

    driven!(gen_find_state, ssx3_ee::s13::f_0039f9d8__generated);
    driven!(hand_find_state, super::ui_state_mgr_find_state);

    fn find_state_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = mgr_setup(c, seed, 0x50, 0, 10);
        c.g[A0][0] = MGR as u64;
        // An id in 0..7 (6 and 7 never match), sometimes with upper bits.
        let id = (r.next_u32() % 8) as u64;
        c.g[A1][0] = if r.next_u32() % 8 == 0 { id | 1 << 40 } else { id };
        c.g[GP][0] = GAME_GP as u64;
        schedule(c, &mut r, seed, 0x0039_f9d8)
    }

    #[test]
    fn ui_state_mgr_find_state() {
        check_events(gen_find_state, hand_find_state, SEEDS, &find_state_setup);
        let first = |v: u64| (DATA + 0x100..DATA + 0x800).contains(&(v as u32));
        let second = |v: u64| (DATA + 0x800..DATA + 0xf00).contains(&(v as u32));
        require_v0(gen_find_state, SEEDS, &[&|v| v == 0, &first, &second], &|c, seed| {
            find_state_setup(c, seed);
        });
    }

    driven!(gen_get_active_state, ssx3_ee::s13::f_0039fa88__generated);
    driven!(hand_get_active_state, super::ui_state_mgr_get_active_state);

    fn get_active_state_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = mgr_setup(c, seed, 0x60, 0, 10);
        c.g[A0][0] = if seed % 2 == 0 { MGR } else { MGR + 0x1c } as u64;
        // Mostly inactive states, so the walk goes on.
        for i in 0..10 {
            for base in [DATA + 0x100, DATA + 0x800] {
                let f = base + 0x40 * i + 0x1c;
                let v = c.bus.read32(f);
                let v = if r.next_u32() % 6 == 0 { v | 0x20 } else { v & !0x20 };
                c.bus.write32(f, v);
            }
        }
        schedule(c, &mut r, seed, 0x0039_fa88)
    }

    #[test]
    fn ui_state_mgr_get_active_state() {
        check_events(gen_get_active_state, hand_get_active_state, SEEDS, &get_active_state_setup);
        require_v0(gen_get_active_state, SEEDS, &[&|v| v == 0, &|v| v != 0], &|c, seed| {
            get_active_state_setup(c, seed);
        });
    }

    // ------------------------------------------------- update and engine

    /// Give every state of both lists a screen list (+0x24) of 0..2
    /// screens that `UIScreen_Update` passes over quickly (no threads, no
    /// child widgets, no animation: the real callees run in the test).
    fn benign_screens(c: &mut Cpu, r: &mut Rng) {
        for (j, base) in [DATA + 0x100, DATA + 0x800].into_iter().enumerate() {
            for i in 0..12u32 {
                let s = base + 0x40 * i;
                let pool = DATA + 0x2_0000 + 0x400 * (12 * j as u32 + i);
                let n = r.next_u32() % 3;
                let screens: Vec<u32> = (0..n).map(|k| pool + 0x100 * k).collect();
                ui_list(c, r, s + 0x24, &screens, pool + 0x300);
            }
        }
    }

    const PROCESS_STATES_JUMPS: [u32; 6] =
        [0x0039_ed4c, 0x0039_ee14, 0x0039_edc8, 0x0039_ee78, 0x0039_ee9c, 0x0039_eee4];

    fn update_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = mgr_setup(c, seed, salt, calls, 6);
        benign_screens(c, &mut r);
        // UIStateMgr_ProcessStates' phase jump table (.rodata).
        crate::difftest::put_words(c, 0x0049_3fc0, &PROCESS_STATES_JUMPS);
        r
    }

    #[test]
    fn ui_state_update_screens() {
        let (generated, hand) = (ssx3_ee::s13::f_0039e868__generated, super::ui_state_update_screens);
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = update_setup(c, seed, 0x70, 0);
            c.g[A0][0] = (DATA + 0x100 + 0x40 * (r.next_u32() % 6)) as u64;
            schedule(c, &mut r, seed, 0x0039_e868)
        });
    }

    #[test]
    fn ui_state_mgr_update() {
        let (generated, hand) = (ssx3_ee::s13::f_0039f100__generated, super::ui_state_mgr_update);
        covered(generated, hand, SEEDS, &|c, seed| {
            update_setup(c, seed, 0x80, 8);
            c.g[A0][0] = MGR as u64;
        });
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = update_setup(c, seed, 0x81, 0);
            c.g[A0][0] = MGR as u64;
            schedule(c, &mut r, seed, 0x0039_f100)
        });
        // Both outcomes of the first ProcessStates (v0 != 0 takes the
        // second pass).
        let mut outcomes = [0u32; 2];
        for seed in 0..SEEDS {
            let mut c = fresh();
            update_setup(&mut c, seed, 0x81, 0);
            c.g[A0][0] = MGR as u64;
            c.pc = 0x0039_ecb0;
            ssx3_ee::s13::f_0039ecb0(&mut c);
            outcomes[(c.g[V0][0] != 0) as usize] += 1;
        }
        assert!(outcomes[0] > 0 && outcomes[1] > 0, "ProcessStates results {outcomes:?}");
    }

    #[test]
    fn ui_engine_update() {
        let (generated, hand) = (ssx3_ee::s13::f_00398038__generated, super::ui_engine_update);
        covered(generated, hand, SEEDS, &|c, seed| {
            let mut r = update_setup(c, seed, 0x90, 6);
            // engine + 0x18 is the manager; the input object uses VT too.
            let engine = MGR - 0x18;
            c.bus.write32(engine + 0xc, DATA + 0x1_0000);
            c.bus.write32(DATA + 0x1_0008, VT + (r.next_u32() % 4) * 8 - 0x170 + 0x80);
            c.g[A0][0] = engine as u64;
        });
    }

    /// The renderer at DATA+0x1_8000 (vptr VT-relative), its render-state
    /// stack at a random word offset, the UI default state and view.
    fn renderer_setup(c: &mut Cpu, r: &mut Rng) {
        let rend = DATA + 0x1_8000;
        c.bus.write32(GAME_GP - 0x854, rend);
        // Methods 0xa8..0x114 land in VT+0x00..0x70.
        c.bus.write32(rend + 0x10d8, VT - 0xa8);
        let stack = DATA + 0x1_c000 + 4 * (r.next_u32() % 16);
        c.bus.write32(rend + 0xe84, stack);
        fill(c, r, stack & !0xf, 0x20);
        fill(c, r, 0x0050_1420, 8);
        fill(c, r, 0x004f_f1a0, 16);
    }

    #[test]
    fn ui_engine_render() {
        let (generated, hand) = (ssx3_ee::s13::f_00397df8__generated, super::ui_engine_render);
        covered(generated, hand, SEEDS, &|c, seed| {
            let mut r = mgr_setup(c, seed, 0xa0, 10, 4);
            renderer_setup(c, &mut r);
            c.g[A0][0] = (MGR - 0x18) as u64;
        });
    }

    // ------------------------------------------------ cHUDFX and placing

    /// A message at DATA+0x400 and a text of 0..30 bytes at DATA+0x200.
    fn hud_fx_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        fill(c, &mut r, DATA + 0x400, 0x40);
        let n = r.next_u32() % 30;
        for i in 0..n {
            c.bus.write8(DATA + 0x200 + i, (r.next_u32() % 255 + 1) as u8);
        }
        c.bus.write8(DATA + 0x200 + n, 0);
        c.g[A0][0] = (DATA + 0x400) as u64;
        c.g[A2][0] = if r.next_u32() % 8 == 0 { 0 } else { (DATA + 0x200) as u64 };
        c.f[12] = r.edge_f32();
        r
    }

    #[test]
    fn c_hudfx_show() {
        crate::difftest::check(ssx3_ee::s00::f_00117048__generated, super::c_hudfx_show, SEEDS, &|c, s| {
            hud_fx_setup(c, s, 0xb0);
        });
    }

    /// The game's initialised data (the ELF's load segment), for the real
    /// libc `sprintf` (format string, `_impure_ptr`, jump tables).
    /// (Same as the private helper in `rider_states`' tests.)
    fn elf_image(c: &mut Cpu) {
        static IMAGE: std::sync::OnceLock<(u32, Vec<u8>)> = std::sync::OnceLock::new();
        let (vaddr, bytes) = IMAGE.get_or_init(|| {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
            let d = std::fs::read(path).expect("orig/SLUS_207.72");
            let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
            let ph = u32_at(0x1c) as usize;
            let (off, vaddr, size) = (u32_at(ph + 4) as usize, u32_at(ph + 8), u32_at(ph + 16) as usize);
            (vaddr, d[off..off + size].to_vec())
        });
        let at = *vaddr as usize;
        c.bus.ram[at..at + bytes.len()].copy_from_slice(bytes);
    }

    /// The game image (`"%d"`, libc state) and a number with some edge
    /// values.
    fn number_setup(c: &mut Cpu, r: &mut Rng) {
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        let n = match r.next_u32() % 4 {
            0 => 0,
            1 => 0x8000_0000,
            2 => r.next_u32() % 1000,
            _ => r.next_u32(),
        };
        c.g[A2][0] = n as i32 as i64 as u64;
    }

    #[test]
    fn c_hudfx_show_number_delayed() {
        crate::difftest::check(ssx3_ee::s00::f_001171a8__generated, super::c_hudfx_show_number_delayed, SEEDS, &|c, s| {
            let mut r = hud_fx_setup(c, s, 0xb1);
            number_setup(c, &mut r);
        });
        // The real sprintf formats into the message text.
        let mut c = fresh();
        let mut r = hud_fx_setup(&mut c, 0, 0xb1);
        number_setup(&mut c, &mut r);
        c.g[A2][0] = (-42i64) as u64;
        crate::difftest::run_generated(&mut c, ssx3_ee::s00::f_001171a8__generated);
        let text: Vec<u8> = (0..4).map(|i| c.bus.read8(DATA + 0x418 + i)).collect();
        assert_eq!(text, b"-42\0");
    }

    #[test]
    fn rider_metrix_show_hud_number() {
        let (generated, hand) = (ssx3_ee::s00::f_00117b88__generated, super::rider_metrix_show_hud_number);
        let setup = |c: &mut Cpu, s: u32| {
            let mut r = remove_hud_message_setup(c, s);
            number_setup(c, &mut r);
            c.f[12] = r.edge_f32();
        };
        crate::difftest::check_lohi(generated, hand, SEEDS, &setup);
        require_v0(generated, SEEDS, &[&|v| v == u64::MAX, &|v| v != u64::MAX], &setup);
    }

    #[test]
    fn hud_align_elem_pos() {
        crate::difftest::check(ssx3_ee::s04::f_001f10f8__generated, super::hud_align_elem_pos, SEEDS, &|c, seed| {
            let mut r = rng(seed, 0xb3);
            scramble(c, &mut r);
            fill(c, &mut r, DATA + 0x300, 0x10);
            for i in 0..6 {
                c.bus.write32(DATA + 4 * i, r.edge_f32());
            }
            c.g[A1][0] = (DATA + 0x300) as u64;
            c.g[A2][0] = DATA as u64;
            c.g[A3][0] = (DATA + 8) as u64;
            c.g[T0][0] = (DATA + 16) as u64;
            for reg in [A0, T1, T2, T3] {
                c.g[reg][0] = (r.next_u32() % 8) as u64;
            }
        });
    }

    // ---------------------------------------------------------------- fonts

    const FONT: u32 = DATA + 0x400;

    /// The font renderer (gp+0x2a90) with its render-state record and a
    /// vtable whose `DrawText` (+0x280) is a fake; a font with edge-case
    /// floats (shadow offsets often zero, sometimes denormal), glyph tables
    /// in random data, and a text of 0..20 bytes at DATA+0x200.
    fn font_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        for off in (0..0x100).step_by(8) {
            c.bus.write16(VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(VT + off + 4, FAKE + 2 * off);
        }
        let rend = DATA + 0x1_8000;
        c.bus.write32(GAME_GP + 0x2a90, rend);
        c.bus.write32(rend + 0x10d8, VT - 0x280 + 8 * (r.next_u32() % 8));
        let state = DATA + 0x1_c000 + 4 * (r.next_u32() % 16);
        c.bus.write32(rend + 0xe84, state);
        fill(c, &mut r, state & !0xf, 0x20);
        fill(c, &mut r, FONT, 0x20);
        for i in 0..8 {
            c.bus.write32(FONT + 0x18 + 4 * i, r.edge_f32());
        }
        for i in 0..2 {
            let v = match r.next_u32() % 4 {
                0 | 1 => 0,
                2 => r.next_u32() & 0x807f_ffff,
                _ => r.edge_f32(),
            };
            c.bus.write32(FONT + 0x28 + 4 * i, v);
        }
        // Glyph tables for Font_MeasureText: count, sorted codes, glyphs.
        let codes = DATA + 0x3000;
        let n = 1 + r.next_u32() % 40;
        let mut code = 0;
        for i in 0..n {
            code += 1 + r.next_u32() % 6;
            c.bus.write16(codes + 2 * i, code as u16);
        }
        fill(c, &mut r, DATA + 0x4000, 0x400);
        c.bus.write32(FONT, n);
        c.bus.write32(FONT + 4, codes);
        c.bus.write32(FONT + 8, DATA + 0x4000 + 12 * (r.next_u32() % 8));
        c.bus.write32(FONT + 0xc, r.next_u32() % 64);
        c.bus.write32(FONT + 0x10, 64 + r.next_u32() % 128);
        let len = r.next_u32() % 20;
        for i in 0..len {
            c.bus.write8(DATA + 0x200 + i, (1 + r.next_u32() % 200) as u8);
        }
        c.bus.write8(DATA + 0x200 + len, 0);
        c.g[A0][0] = FONT as u64;
        c.g[A1][0] = (DATA + 0x200) as u64;
        c.f[12] = r.edge_f32();
        c.f[13] = r.edge_f32();
        r
    }

    #[test]
    fn font_draw_text() {
        let (generated, hand) = (ssx3_ee::s12::f_00391cb0__generated, super::font_draw_text);
        covered(generated, hand, SEEDS, &|c, seed| {
            font_setup(c, seed, 0xc0, 2);
        });
        // No shadow (29 + 5 + 22 + fake), shadow from x (29 + 40 + 22) or
        // from y only (29 + 5 + 40 + 22): the fakes' cost varies, so look
        // at the number of fake calls instead.
        let mut seen = [0u32; 3];
        for seed in 0..SEEDS {
            let mut c = fresh();
            font_setup(&mut c, seed, 0xc1, 0);
            let x = c.bus.read32(FONT + 0x28) & 0x7f80_0000 != 0;
            crate::difftest::run_generated(&mut c, generated);
            let calls = c.bus.read32(crate::difftest::fakes::LOG_COUNT);
            seen[if calls == 1 { 0 } else if x { 1 } else { 2 }] += 1;
        }
        assert!(seen.iter().all(|&n| n > 0), "paths {seen:?}");
    }

    #[test]
    fn font_draw_text_centered() {
        let (generated, hand) = (ssx3_ee::s12::f_00392908__generated, super::font_draw_text_centered);
        covered(generated, hand, SEEDS, &|c, seed| {
            font_setup(c, seed, 0xc2, 2);
        });
    }

    // -------------------------------------------------------------- sprites

    fn sprite_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        renderer_setup(c, &mut r);
        // DrawPrimitive at +0x238 lands in the fake vtable.
        c.bus.write32(DATA + 0x1_8000 + 0x10d8, VT - 0x238 + 8 * (r.next_u32() % 8));
        for off in (0..0x100).step_by(8) {
            c.bus.write16(VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(VT + off + 4, FAKE + 2 * off);
        }
        for i in 0..32 {
            c.bus.write32(DATA + 0x500 + 4 * i, r.edge_f32());
        }
        // Colours mostly in 0..1.
        for i in 0..4 {
            if r.next_u32() % 4 != 0 {
                let f = (r.next_u32() % 1000) as f32 / 999.0;
                c.bus.write32(DATA + 0x540 + 4 * i, f.to_bits());
            }
        }
        c.g[A1][0] = if r.next_u32() % 8 == 0 { 0 } else { (DATA + 0x500) as u64 };
        c.g[A2][0] = (DATA + 0x520) as u64;
        c.g[A3][0] = (DATA + 0x528) as u64;
        c.g[T0][0] = (DATA + 0x530) as u64;
        c.g[T1][0] = (DATA + 0x540) as u64;
        r
    }

    #[test]
    fn hud_draw_sprite() {
        let (generated, hand) = (ssx3_ee::s04::f_001f1190__generated, super::hud_draw_sprite);
        covered(generated, hand, SEEDS, &|c, seed| {
            sprite_setup(c, seed, 0xd0, 1);
        });
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = sprite_setup(c, seed, 0xd1, 0);
            schedule(c, &mut r, seed, 0x001f_1190)
        });
    }

    fn measure_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = font_setup(c, seed, salt, 0);
        // Text with codes below, inside and above the dense range.
        let len = r.next_u32() % 24;
        for i in 0..len {
            c.bus.write8(DATA + 0x200 + i, (1 + r.next_u32() % 255) as u8);
        }
        c.bus.write8(DATA + 0x200 + len, 0);
        // Dense range [first, last], extra glyphs sorted above last.
        let first = 32 + r.next_u32() % 32;
        let last = first + r.next_u32() % 64;
        let dense = last - first + 1;
        let extra = r.next_u32() % 20;
        let table = DATA + 0x4000;
        fill(c, &mut r, table, 3 * (dense + extra + 1));
        let mut code = last;
        for i in 0..extra {
            code += 1 + r.next_u32() % 8;
            c.bus.write16(table + 12 * (dense + i), code as u16);
        }
        c.bus.write32(FONT, dense + extra);
        c.bus.write32(FONT + 4, table);
        c.bus.write32(FONT + 8, if r.next_u32() % 3 == 0 { 0 } else { table + 12 * (r.next_u32() % dense) });
        c.bus.write32(FONT + 0xc, first);
        c.bus.write32(FONT + 0x10, last);
        c.g[A2][0] = if r.next_u32() % 2 == 0 { 0 } else { (DATA + 0x380) as u64 };
        fill(c, &mut r, DATA + 0x380, 4);
        c.g[A3][0] = (r.next_u32() % 2) as u64;
        r
    }

    #[test]
    fn font_measure_text() {
        let (generated, hand) = (ssx3_ee::s12::f_00391fb0__generated, super::font_measure_text);
        crate::difftest::check_lohi(generated, hand, SEEDS, &|c, seed| {
            measure_setup(c, seed, 0xe0);
        });
        check_events(generated, hand, SEEDS, &|c, seed| {
            let mut r = measure_setup(c, seed, 0xe1);
            schedule(c, &mut r, seed, 0x0039_1fb0)
        });
    }
}
