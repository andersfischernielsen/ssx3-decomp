//! Staging module oc319 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc319`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_9e20, crowd_man2d_save_state, "CrowdMan2d_SaveState"),
    (0x002f_fb50, world_obj_factory_hide, "WorldObjFactory_Hide"),
    (
        0x0011_75f8,
        rider_metrix_reset_combo,
        "RiderMetrix_ResetCombo",
    ),
    (0x003c_83a0, snd_ee_voice_activate, "SND_EeVoiceActivate"),
    (
        0x002f_c7d0,
        world_obj_factory_debounce,
        "WorldObjFactory_Debounce",
    ),
    (0x0035_6cc8, mod_obj_v22, "ModObj_v22"),
    (
        0x002f_ded0,
        wscript_fn_add_spline_modifier,
        "WScriptFn_AddSplineModifier",
    ),
];

/// Bytes of the 2D crowd array saved after the object's first word.
pub(crate) const CROWD_MAN2D_SAVE_SIZE: u64 = 0x200;

/// `void CrowdMan2d_SaveState(void* self, cPbStream* s)` (0x229e20): writes the
/// crowd's 0x200-byte array at `self+8` through the stream's virtual write
/// method (vtable entry +8: a 16-bit `this` delta and the code pointer at
/// +0xc).
pub fn crowd_man2d_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = CROWD_MAN2D_SAVE_SIZE;
        // s->v01(self + 8, 0x200): vtable entry at +8.
        let stream = r32(c, A3);
        c.g[V1][0] = Addr(stream).lw(c);
        let slot = r32(c, V1);
        c.g[A0][0] = Addr(slot).off(0x8).lh(c);
        c.g[V0][0] = Addr(slot).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0022_9e48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_Hide (0x2ffb50)

/// gp-relative globals of the factory (`gp+0x2560` = `D_004A5650`, the
/// "static seed ready" flag).
pub(crate) const WOF_INIT_FLAG: u32 = 0x2560;

/// The 64-bit static seed word at `D_004A5648`, and its trailing byte at
/// `D_004A564C` (`gp+0x255c`).
pub(crate) const WOF_SEED_WORD: u32 = 0x2558;

pub(crate) const WOF_SEED_BYTE: u32 = 0x255c;

/// `D_004A3C10`: the per-index type of the entries (2 = float value).
pub(crate) const WOF_KINDS: u32 = 0xb20;

/// `D_004A47B8`: the world's object reference table.
pub(crate) const WOF_OBJECTS: u32 = 0x16c8;

/// `D_004A3DD8`: the object whose `+0x290` is returned for id `-1`.
pub(crate) const WOF_SPECIAL: u32 = 0xce8;

/// The tag passed to the allocator: `0x4a0000 + 0x3c18` = `D_004A3C18`.
pub(crate) const WOF_ALLOC_TAG_BASE: u32 = 0x4a_0000;

pub(crate) const WOF_ALLOC_TAG: u32 = 0x3c18;

/// `s4` holds `0x4d0000`, the epilogue's copy source minus 0x6f68.
pub(crate) const WOF_RESULT_BASE: u32 = 0x4d_0000;

pub(crate) const WOF_RESULT_OFF: u32 = 0x6f68;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const WOF_MEM_COPY: u32 = 0x003e_6574;

/// `EA_MemAllocNamed` (0x317d70).
pub(crate) const WOF_MEM_ALLOC: u32 = 0x0031_7d70;

/// `Hide_ctor` (0x350e90), called on the new 0x1c-byte block.
pub(crate) const WOF_HIDE_CTOR: u32 = 0x0035_0e90;

/// `V3 WorldObjFactory_Hide(int n, Ent* e)` (0x2ffb50): builds the 12-byte
/// world-object description `{id, value, value}` from `n` entries
/// (`{idx, val, pad, type}`), then hides the referenced object (kills it if it
/// is visible, else allocates a `Hide` node for it). `a0` is the hidden
/// return-buffer pointer; the result is copied there and returned in `v0`.
pub fn world_obj_factory_hide(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(r32(c, GP)).off(WOF_INIT_FLAG).lw(c);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !ready {
            // Guarded static init: seed word = -1, flag = 1, and the flag
            // byte goes through the stack (the copy-ctor artifact).
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SEED_BYTE).lbu(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] |= 0xffff;
            c.g[A0][0] = 0x1;
            c.sw(r32(c, GP).wrapping_add(WOF_SEED_WORD), c.g[V1][0]);
            c.sw(r32(c, GP).wrapping_add(WOF_INIT_FLAG), c.g[A0][0]);
            c.sb(r32(c, SP), c.g[V0][0]);
        }
        // buf64[0] = -1, then the 8-byte seed word is copied over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x8;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = sx(r32(c, GP).wrapping_add(WOF_SEED_WORD));
        c.g[S4][0] = sx(WOF_RESULT_BASE);
        call_ptr(c, WOF_MEM_COPY, 0x002f_fbc0)?;
        // `a2 = s2`: the entry pointer, in the branch delay slot.
        c.g[A2][0] = c.g[S2][0];
        // For each entry: buf[idx] = val, as a float when the kind table says
        // 2 and the entry's own type says otherwise.
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(r32(c, GP).wrapping_add(WOF_KINDS));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_fbd8);
                }
            }
        }
        // Resolve the id: id -1 means the object the world points at, else
        // `table[id & 0xff] -> [id >> 8]`, a reference (byte offset) or 0.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[S1][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            c.cycles += 1;
            c.g[S1][0] = c.g[A1][0];
        }
        // Hide it: nothing to do without an object; if it is still visible,
        // kill it; otherwise build a Hide node for it.
        c.cycles += 2;
        let mut hide = false;
        if c.g[S1][0] != 0 {
            hide = true;
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, S1)).off(0xc).lw(c);
            c.g[A1][0] = sx(WOF_ALLOC_TAG_BASE);
            if c.g[S0][0] != 0 {
                // p->v10(16)?
                c.cycles += 8;
                c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                c.g[A1][0] = 0x10;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x50).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x54).lw(c);
                let query = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, query, 0x002f_fcb8)?;
                let hidden = c.g[V0][0] != 0;
                c.g[A1][0] = 0x6;
                if hidden {
                    hide = false;
                } else {
                    // p->v10(6)?
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1)).off(0x50).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(0x54).lw(c);
                    let query = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    call_ptr(c, query, 0x002f_fcd4)?;
                    let hidden = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x3;
                    if hidden {
                        hide = false;
                    } else {
                        // Still visible: p->v01(3) kills it.
                        c.cycles += 6;
                        c.g[V0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                        c.g[A0][0] = Addr(r32(c, V0)).off(0x8).lh(c);
                        c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
                        let kill = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        call_ptr(c, kill, 0x002f_fcf0)?;
                        c.g[A1][0] = sx(WOF_ALLOC_TAG_BASE);
                    }
                }
            }
        }
        if hide {
            c.cycles += 8;
            c.g[A0][0] = 0x1c;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(WOF_ALLOC_TAG));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, WOF_MEM_ALLOC, 0x002f_fd08)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, WOF_HIDE_CTOR, 0x002f_fd14)?;
        }
        // Return the 12-byte result description by value.
        c.cycles += 16;
        let src = r32(c, S4).wrapping_sub(WOF_RESULT_OFF);
        c.g[A1][0] = sx(src);
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S3);
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S3][0];
        for (reg, off) in [(S0, 0x70), (S1, 0x60), (S2, 0x50), (S3, 0x40), (S4, 0x30)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_RemoveHUDMessage` (0x1179e0).
pub(crate) const RIDER_METRIX_REMOVE_HUD: u32 = 0x0011_79e0;

/// `void cRiderMetrix::resetComboChain(cRiderMetrix* this)` (0x1175f8):
/// removes the combo HUD message (id 3), then clears the combo chain count
/// (+0x9c), the combo points (+0xa0) and the combo timer (+0xa4 = -1.0).
pub fn rider_metrix_reset_combo(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, RIDER_METRIX_REMOVE_HUD, 0x0011_7610)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        let this = r32(c, S0);
        Addr(this).off(0x9c).sw(c, 0);
        c.swc1(0, this.wrapping_add(0xa4));
        Addr(this).off(0xa0).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_00515B40`: the voice table: `+5` = active count (byte), `+0x1dc` = the
/// 0x60-byte voice entries.
#[cfg(test)]
pub(crate) const SND_EE_VOICE_TABLE: u32 = 0x0051_5b40;

pub(crate) const SND_EE_VOICE_COUNT: u32 = 0x5;

pub(crate) const SND_EE_VOICE_ENTRIES: u32 = 0x1dc;

/// `void SND_EeVoiceActivate(int voice)` (0x3c83a0): resets voice `voice`:
/// clears its flags at +0 and +1 and its word at +0x3c, slides the two pitch
/// floats at +0x34 down one, copies the envelope floats at +4 from +0x1c
/// (as many as the active voice count), and marks the voice `2` (active).
pub fn snd_ee_voice_activate(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V1][0] = 0x60;
    c.g[V0][0] = 0x0051_0000;
    c.mult(0, A0, A0, V1); // a0 = voice * 0x60
    c.g[A2][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b40));
    c.g[A1][0] = 0;
    c.g[V1][0] = Addr(r32(c, V0)).off(SND_EE_VOICE_ENTRIES).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, A0)));
    let voice = r32(c, V1);
    c.sw(voice.wrapping_add(0x3c), 0);
    c.g[V0][0] = sx(voice.wrapping_add(0x34));
    c.sb(voice.wrapping_add(0x1), 0);
    // Two pitch floats: a[0] = a[1].
    loop {
        c.cycles += 7;
        c.lwc1(0, r32(c, V0).wrapping_add(0x4));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.swc1(0, r32(c, V0));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_83d0);
        }
    }
    // The envelope floats: b[0] = b[6] for every active voice.
    c.cycles += 4;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5b40));
    c.g[V0][0] = Addr(r32(c, A0)).off(SND_EE_VOICE_COUNT).lbu(c);
    let any = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    if any {
        c.cycles += 3;
        c.g[A2][0] = c.g[A0][0];
        c.g[A0][0] = sx(voice.wrapping_add(0x4));
        loop {
            c.cycles += 7;
            c.lwc1(0, r32(c, A0).wrapping_add(0x18));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.swc1(0, r32(c, A0));
            c.g[V0][0] = Addr(r32(c, A2)).off(SND_EE_VOICE_COUNT).lbu(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_8408);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 0x2;
    c.sb(voice, c.g[V0][0]);
    ret(c);
}

/// `WorldObjFactory_Debounce` globals: the static seed word at `0x4fb528`
/// (`a3 = 0x500000`, minus 0x4ad8), its ready flag at `gp+0x24fc`, and the
/// per-index kind table at `0x445fb0`.
pub(crate) const WOD_INIT_FLAG: u32 = 0x24fc;

pub(crate) const WOD_STATIC_BASE: u32 = 0x50_0000;

pub(crate) const WOD_STATIC_OFF: u32 = 0x4ad8;

pub(crate) const WOD_KINDS_BASE: u32 = 0x44_0000;

pub(crate) const WOD_KINDS_OFF: u32 = 0x5fb0;

/// The allocator tag: `0x490000 - 0x6918` = `D_004896E8`.
pub(crate) const WOD_ALLOC_TAG: u32 = 0x490000;

pub(crate) const WOD_ALLOC_TAG_OFF: u32 = 0x6918;

/// `WorldObjFactory_AttachModifier` (0x2fae38).
pub(crate) const WOD_CHECK_ACTIVE: u32 = 0x002f_ae38;

/// `Debounce_ctor_2` (0x342c08).
pub(crate) const WOD_DEBOUNCE_CTOR: u32 = 0x0034_2c08;

/// `V3 WorldObjFactory_Debounce(int n, Ent* e)` (0x2fc7d0): like
/// `WorldObjFactory_Hide`, but the node it builds is a `Debounce` (0x34
/// bytes) that ignores objects an active modifier already covers.
pub fn world_obj_factory_debounce(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP)).off(WOD_INIT_FLAG).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = sx(WOD_STATIC_BASE);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x30);
        if !ready {
            // Guarded static init of the 20-byte seed: -1, 1.0f, 1, -1 and
            // its trailing flag byte through the stack.
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WOD_STATIC_OFF));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = Addr(r32(c, V1)).off(0x10).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = !0u64;
            c.sw(r32(c, A3).wrapping_sub(WOD_STATIC_OFF), c.g[V0][0]);
            c.g[A1][0] = 0x1;
            c.sb(r32(c, V1).wrapping_add(0x10), c.g[A2][0]);
            c.swc1(0, r32(c, V1).wrapping_add(0x4));
            c.sw(r32(c, V1).wrapping_add(0xc), c.g[A0][0]);
            c.sw(r32(c, GP).wrapping_add(WOD_INIT_FLAG), c.g[A1][0]);
            c.sw(r32(c, V1).wrapping_add(0x8), c.g[A1][0]);
            c.sb(r32(c, SP), c.g[A2][0]);
        }
        // buf64[5] = -1, then the 20-byte static over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x14;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WOD_STATIC_OFF));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = sx(WOF_RESULT_BASE);
        call_ptr(c, WOF_MEM_COPY, 0x002f_c864)?;
        // `a2 = s3`: the entry pointer (the third argument), in the branch
        // delay slot.
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = sx(WOD_KINDS_BASE);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(WOD_KINDS_OFF));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c880);
                }
            }
        }
        // Resolve the id, exactly as Hide does.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[S0][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }
        // Skip objects that are already covered by an active modifier, else
        // build the Debounce node.
        c.cycles += 2;
        let mut insert = false;
        if c.g[S0][0] != 0 {
            insert = true;
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
            c.g[A1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, WOD_CHECK_ACTIVE, 0x002f_c950)?;
                if c.g[V0][0] != 0 {
                    insert = false;
                }
            }
        }
        if insert {
            c.cycles += 12;
            c.g[A1][0] = sx(WOD_ALLOC_TAG);
            c.g[A0][0] = 0x34;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(WOD_ALLOC_TAG_OFF));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, WOF_MEM_ALLOC, 0x002f_c970)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = 0;
            call_ptr(c, WOD_DEBOUNCE_CTOR, 0x002f_c988)?;
        }
        // Return the 12-byte result description by value.
        c.cycles += 16;
        let src = r32(c, S4).wrapping_sub(WOF_RESULT_OFF);
        c.g[A1][0] = sx(src);
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S2);
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        for (reg, off) in [(S0, 0x80), (S1, 0x70), (S2, 0x60), (S3, 0x50), (S4, 0x40)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `MultiSplineModifier_Release` (0x35aae0).
pub(crate) const MODOBJ_RELEASE: u32 = 0x0035_aae0;

/// `int cMoveNode::decFocusCount(tInstanceRecord* pInst)` (0x356cc8): if the
/// move node's block at +0x1c holds a node whose virtual query (slot 26)
/// returns 2, release that node and return its result; otherwise 0.
pub fn mod_obj_v22(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).off(0x1c).lw(c);
        let has_block = c.g[V1][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !has_block {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, V1)).lw(c);
            let has_node = c.g[A1][0] != 0;
            c.g[V0][0] = 0;
            let mut restored = false;
            if has_node {
                // node->vt[26].fn(node + delta) == 2?
                c.cycles += 8;
                c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).off(0xd0).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0xd4).lw(c);
                let query = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, query, 0x0035_6d08)?;
                c.g[V1][0] = 0x2;
                let not_two = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = 0;
                if !not_two {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, S0)).off(0x1c).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                    call_ptr(c, MODOBJ_RELEASE, 0x0035_6d24)?;
                    frame.restore(c, S0, 0x20);
                    restored = true;
                }
            }
            if !restored {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptFn_AddSplineModifier` globals: the static spline seed at 0x4fb778
/// (`0x500000 - 0x4888`), its ready flag at `gp+0x2524`, the kind table at
/// 0x4461d8, and the 12-word stack buffer at `sp+0x10`.
pub(crate) const WSA_INIT_FLAG: u32 = 0x2524;

pub(crate) const WSA_STATIC_BASE: u32 = 0x50_0000;

pub(crate) const WSA_STATIC_OFF: u32 = 0x4888;

pub(crate) const WSA_KINDS_BASE: u32 = 0x44_0000;

pub(crate) const WSA_KINDS_OFF: u32 = 0x61d8;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const WSA_MEM_COPY: u32 = 0x003e_6574;

/// `cMoveNode_addSpline1` (0x355ad0).
pub(crate) const WSA_ADD_SPLINE: u32 = 0x0035_5ad0;

pub(crate) const WSA_RESULT_BASE: u32 = 0x4d_0000;

pub(crate) const WSA_RESULT_OFF: u32 = 0x6f68;

/// `V3 WScriptFn_AddSplineModifier(int n, Ent* e)` (0x2fded0): the script
/// function behind "add spline": builds a 48-byte spline description from `n`
/// entries and, if its second word names a node, adds the spline to the move
/// node the first word resolves to.
pub fn wscript_fn_add_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP)).off(WSA_INIT_FLAG).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = sx(WSA_STATIC_BASE);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !ready {
            // Guarded static init: -1, -1, 1, 3, 20.0, 0, 0, 0, 1, 0, 0 and
            // the trailing tag byte through the stack.
            c.cycles += 23;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(WSA_STATIC_OFF));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.mtc1(0, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A2][0] = Addr(r32(c, V0)).off(0x2c).lbu(c);
            c.g[A1][0] = 0x3;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = 0x1;
            c.sw(r32(c, A3).wrapping_sub(WSA_STATIC_OFF), c.g[V1][0]);
            c.sb(r32(c, V0).wrapping_add(0x2c), c.g[A2][0]);
            c.sw(r32(c, V0).wrapping_add(0x4), c.g[V1][0]);
            c.sw(r32(c, V0).wrapping_add(0xc), c.g[A1][0]);
            c.swc1(1, r32(c, V0).wrapping_add(0x10));
            c.swc1(0, r32(c, V0).wrapping_add(0x28));
            c.sw(r32(c, GP).wrapping_add(WSA_INIT_FLAG), c.g[A0][0]);
            c.sw(r32(c, V0).wrapping_add(0x8), c.g[A0][0]);
            c.swc1(0, r32(c, V0).wrapping_add(0x14));
            c.swc1(0, r32(c, V0).wrapping_add(0x18));
            c.swc1(0, r32(c, V0).wrapping_add(0x1c));
            c.sw(r32(c, V0).wrapping_add(0x20), c.g[A0][0]);
            c.swc1(0, r32(c, V0).wrapping_add(0x24));
            c.sb(r32(c, SP), c.g[A2][0]);
        }
        // buf64[12]: -1, -1, then the 48-byte static spline over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x30;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WSA_STATIC_OFF));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, WSA_MEM_COPY, 0x002f_df80)?;
        // `a2 = s2`: the entry pointer, in the branch delay slot.
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(WSA_KINDS_BASE);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(WSA_KINDS_OFF));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_df98);
                }
            }
        }
        // Resolve the node id from buf[0].
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            let missing = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if !missing {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // If buf[1] names something, add the spline to the node it hangs on.
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, S1)).off(0x4).lw(c);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = sx(WSA_RESULT_BASE);
        } else {
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
            let no_node = c.g[S0][0] == 0;
            c.g[V1][0] = sx(WSA_RESULT_BASE);
            if !no_node {
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).off(0x80).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
                let spline = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, spline, 0x002f_e070)?;
                let unsupported = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !unsupported {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, WSA_ADD_SPLINE, 0x002f_e080)?;
                }
                c.cycles += 1;
                c.g[V1][0] = sx(WSA_RESULT_BASE);
            }
        }
        // Return the 12-byte result description by value.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(WSA_RESULT_OFF));
        let src = r32(c, A3);
        c.ldl(4, src.wrapping_add(0x7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S3);
        c.sdl(4, dst.wrapping_add(0x7));
        c.sdr(4, dst);
        c.sw(dst.wrapping_add(0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `WorldObjFactory_Hide`: `n` entries at DATA+0x8000 (the first one, when
    /// present, carries the id `0x123`), the world's object reference table at
    /// DATA+0x5000, the `-1` object at DATA+0x5400, and in some seeds an
    /// object to hide at DATA+0x6000 whose vtable slot 0x50 is a fake (hidden
    /// object) or the real zero leaf (killed, then a `Hide` node allocated).
    pub(crate) fn world_obj_factory_hide_setup(c: &mut Cpu, seed: u32) {
        /// `func_001406C0`, a real leaf returning 0.
        const ZERO_LEAF: u32 = 0x0014_06c0;
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;

        let mut r = machine(c, seed, 0x002f_fb50, 6);
        // Make sure the fake query/kill calls unwind in some seeds.
        if seed % 2 == 0 {
            c.bus.write32(LOG_UNWIND_AT, 0);
        }
        pool_allocator(c);
        heap_init(c);
        be_library(c);
        world(c);

        // The guarded static seed, and sometimes the uninitialized path.
        c.bus
            .write32(GAME_GP + WOF_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
        // The seed word is -1, as the guarded init leaves it.
        c.bus.write32(GAME_GP + WOF_SEED_WORD, 0xffff_ffff);
        c.bus
            .write32(GAME_GP + WOF_SEED_BYTE, 0x1122_3300 | (seed & 0xff));
        for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
            c.bus.write32(GAME_GP + WOF_KINDS + 4 * i as u32, kind);
        }

        // Variant: what the entry list holds (0-2 leave the id at -1).
        let variant = seed % 6;
        let obj_kind = (seed / 6) % 5;
        let entries: &[(u32, u32, u32)] = match variant {
            0 | 1 => &[],
            2 => &[(1, 0x42, 2), (2, 0x10, 3)],
            _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
        };
        let n: u32 = match variant {
            0 | 1 => 0,
            2 => 2,
            _ => 4,
        };
        let ents = DATA + 0x8000;
        for (i, &(idx, val, typ)) in entries.iter().enumerate() {
            let p = ents + 0x10 * i as u32;
            c.bus.write32(p, idx);
            c.bus.write32(p + 4, val);
            c.bus.write32(p + 0xc, typ);
        }

        // What the id resolves to: 0 (missing entry), 0 (null reference),
        // an object with no instance, a hidden object, or a visible one.
        let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
        if variant >= 3 {
            let world_obj = DATA + 0x5000;
            let table = DATA + 0x5100;
            let table_slot = table + 4 * 0x23; // id & 0xff
            c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
            c.bus.write32(world_obj + 8, table);
            if obj_kind == 0 {
                c.bus.write32(table_slot, 0);
            } else {
                let s = DATA + 0x5200;
                c.bus.write32(table_slot, s);
                c.bus.write32(s + 0x1c, DATA + 0x5300);
                // The reference: obj/4, read back shifted left by 8.
                let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                c.bus.write32(DATA + 0x5304, word); // [id >> 8] = [1]
            }
        } else {
            let d = DATA + 0x5400;
            c.bus.write32(GAME_GP + WOF_SPECIAL, d);
            c.bus.write32(d + 0x290, obj);
        }
        if obj != 0 {
            let p = DATA + 0x5900;
            match obj_kind {
                2 => c.bus.write32(obj + 0xc, 0),
                3 => {
                    let vt = vtable(c, &mut r, &[0x50]);
                    c.bus.write32(p + 0xc, vt);
                    c.bus.write32(obj + 0xc, p);
                }
                _ => {
                    let vt = DATA + 0x5800;
                    c.bus.write16(vt + 0x50, 0);
                    c.bus.write32(vt + 0x54, ZERO_LEAF);
                    c.bus.write32(p + 0xc, vt);
                    c.bus.write32(obj + 0xc, p);
                }
            }
        }

        c.g[A0][0] = sx(DATA + 0x700); // the 12-byte result buffer
        c.g[A1][0] = sx(n);
        c.g[A2][0] = sx(ents);
        c.pc = 0x002f_fb50;
    }

    #[test]
    fn world_obj_factory_hide_matches() {
        use crate::difftest::fakes::covered;
        covered(
            ee::s10::f_002ffb50__generated,
            world_obj_factory_hide,
            240,
            &|c, seed| world_obj_factory_hide_setup(c, seed),
        );
    }

    #[test]
    fn rider_metrix_reset_combo_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ee::s00::f_001175f8__generated,
            rider_metrix_reset_combo,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0011_75f8, 0);
                let this = DATA + 0x200;
                fill(c, &mut r, this + 0x90, 0x10);
                // The HUD table: absent in half the seeds, else an offset
                // into valid RAM so the fixed-slot write lands on RAM.
                let table = if seed % 2 == 0 { 0 } else { 0x4000 };
                c.bus.write32(this + 0x1b0, table);
                c.bus.write32(this + 0x1b4, (seed % 0x28) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.pc = 0x0011_75f8;
            },
        );
    }

    /// `SND_EeVoiceActivate`: the voice table global pointing at 0x60-byte
    /// entries in DATA, with the active voice count 0..3 and a voice index
    /// 0..3.
    #[test]
    fn snd_ee_voice_activate_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        check(
            ee::s14::f_003c83a0__generated,
            snd_ee_voice_activate,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_83a0, 0);
                let entries = DATA + 0x4000;
                let voice = seed % 4;
                fill(c, &mut r, entries, 0x60 * 4 / 4);
                c.bus
                    .write32(SND_EE_VOICE_TABLE + SND_EE_VOICE_ENTRIES, entries);
                c.bus
                    .write8(SND_EE_VOICE_TABLE + SND_EE_VOICE_COUNT, (seed % 4) as u8);
                c.g[A0][0] = sx(voice);
                c.pc = 0x003c_83a0;
            },
        );
    }

    /// `WorldObjFactory_Debounce`: the same kind of entry list as Hide (five
    /// slots), the static seed at 0x4fb528, and an object whose +0xc node is
    /// absent (the node is allocated) or present with a fake query method
    /// (nonzero: the object counts as covered, no allocation).
    #[test]
    fn world_obj_factory_debounce_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;
        check(
            ee::s10::f_002fc7d0__generated,
            world_obj_factory_debounce,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_c7d0, 6);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
                pool_allocator(c);
                heap_init(c);
                be_library(c);
                world(c);

                c.bus
                    .write32(GAME_GP + WOD_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
                // 0x4fb528: the static's post-init contents — id -1, 1.0f, 1,
                // -1 and the flag byte.
                let seed_word = WOF_RESULT_BASE - WOD_STATIC_OFF;
                c.bus.write32(seed_word, 0xffff_ffff);
                c.bus.write32(seed_word + 4, 0x3f80_0000);
                c.bus.write32(seed_word + 8, 1);
                c.bus.write32(seed_word + 12, 0xffff_ffff);
                c.bus.write32(seed_word + 16, 0x1122_3300 | (seed & 0xff));
                for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
                    c.bus
                        .write32(WOD_KINDS_BASE + WOD_KINDS_OFF + 4 * i as u32, kind);
                }

                let variant = seed % 6;
                let obj_kind = (seed / 6) % 4;
                let entries: &[(u32, u32, u32)] = match variant {
                    0 | 1 => &[],
                    2 => &[(1, 0x42, 2), (2, 0x10, 3)],
                    _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
                };
                let n: u32 = match variant {
                    0 | 1 => 0,
                    2 => 2,
                    _ => 4,
                };
                let ents = DATA + 0x9000;
                for (i, &(idx, val, typ)) in entries.iter().enumerate() {
                    let p = ents + 0x10 * i as u32;
                    c.bus.write32(p, idx);
                    c.bus.write32(p + 4, val);
                    c.bus.write32(p + 0xc, typ);
                }

                let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
                if variant >= 3 {
                    let world_obj = DATA + 0x5000;
                    let table = DATA + 0x5100;
                    let table_slot = table + 4 * 0x23;
                    c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
                    c.bus.write32(world_obj + 8, table);
                    if obj_kind == 0 {
                        c.bus.write32(table_slot, 0);
                    } else {
                        let s = DATA + 0x5200;
                        c.bus.write32(table_slot, s);
                        c.bus.write32(s + 0x1c, DATA + 0x5300);
                        let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                        c.bus.write32(DATA + 0x5304, word);
                    }
                } else {
                    let d = DATA + 0x5400;
                    c.bus.write32(GAME_GP + WOF_SPECIAL, d);
                    c.bus.write32(d + 0x290, obj);
                }
                if obj != 0 {
                    // No node yet, or a node whose virtual query is a fake
                    // (nonzero: the object counts as covered, no allocation).
                    let p = DATA + 0x5900;
                    if obj_kind == 2 {
                        c.bus.write32(obj + 0xc, 0);
                    } else {
                        let vt = vtable(c, &mut r, &[0x50]);
                        c.bus.write32(p + 0xc, vt);
                        c.bus.write32(obj + 0xc, p);
                    }
                }

                c.g[A0][0] = sx(DATA + 0x700);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ents);
                c.pc = 0x002f_c7d0;
            },
        );
    }

    /// `ModObj_v22` (`cMoveNode::decFocusCount`): no block, a block with no
    /// node, and a node whose virtual query is a fake forced to 2 (release)
    /// or left to return a pointer (not 2).
    #[test]
    fn mod_obj_v22_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::DATA;
        covered(
            ee::s11::f_00356cc8__generated,
            mod_obj_v22,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_6cc8, 1);
                let this = DATA;
                let block = DATA + 0x200;
                let node = DATA + 0x300;
                let vt = DATA + 0x400;
                c.bus.write32(this + 0x1c, 0);
                match seed % 4 {
                    0 => {}
                    1 => c.bus.write32(block, 0),
                    n => {
                        c.bus.write32(block, node);
                        c.bus.write32(node, vt);
                        c.bus.write16(vt + 0xd0, ((r.next_u32() & 0x3c) as u16) | 4);
                        c.bus.write32(vt + 0xd4, FAKE);
                        c.bus.write32(node + 0x34, r.next_u32());
                        if n == 2 {
                            c.bus.write32(LOG_RESULTS, 2);
                        }
                    }
                }
                if seed % 4 >= 1 {
                    c.bus.write32(this + 0x1c, block);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x600);
                c.pc = 0x0035_6cc8;
            },
        );
    }

    /// `WScriptFn_AddSplineModifier`: the 48-byte static seed at 0x4fb778,
    /// twelve entry slots, the id and the target-note word controlled by the
    /// entries, and a move node whose virtual "supports splines" slot 0x80 is
    /// a fake (adds the spline) or the real zero leaf (does not).
    #[test]
    fn wscript_fn_add_spline_modifier_matches() {
        /// `func_001406C0`, a real leaf returning 0.
        const ZERO_LEAF: u32 = 0x0014_06c0;
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;
        check(
            ee::s10::f_002fded0__generated,
            wscript_fn_add_spline_modifier,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_ded0, 8);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
                pool_allocator(c);
                heap_init(c);
                be_library(c);
                world(c);

                c.bus
                    .write32(GAME_GP + WSA_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
                // 0x4fb778: the static's post-init contents — -1, -1, 1, 3,
                // 20.0f, 0, 0, 0, 1, 0, 0 and the tag byte at +0x2c.
                let seed_word = WSA_STATIC_BASE - WSA_STATIC_OFF;
                for (off, word) in [
                    (0u32, 0xffff_ffffu32),
                    (4, 0xffff_ffff),
                    (8, 1),
                    (12, 3),
                    (16, 0x41a0_0000),
                    (20, 0),
                    (24, 0),
                    (28, 0),
                    (32, 1),
                    (36, 0),
                    (40, 0),
                ] {
                    c.bus.write32(seed_word + off, word);
                }
                c.bus.write32(seed_word + 44, 0x1122_3300 | (seed & 0xff));
                for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
                    c.bus
                        .write32(WSA_KINDS_BASE + WSA_KINDS_OFF + 4 * i as u32, kind);
                }

                let variant = seed % 6;
                let obj_kind = (seed / 6) % 5;
                let entries: &[(u32, u32, u32)] = match variant {
                    0 | 1 => &[],
                    2 => &[(2, 0x10, 3), (3, 0x7, 0)],
                    4 => &[(0, 0x123, 2), (2, 0x10, 3), (3, 0x7, 0)],
                    _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
                };
                let n: u32 = match variant {
                    0 | 1 => 0,
                    2 => 2,
                    4 => 3,
                    _ => 4,
                };
                let ents = DATA + 0x8000;
                for (i, &(idx, val, typ)) in entries.iter().enumerate() {
                    let p = ents + 0x10 * i as u32;
                    c.bus.write32(p, idx);
                    c.bus.write32(p + 4, val);
                    c.bus.write32(p + 0xc, typ);
                }

                let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
                if variant >= 3 {
                    let world_obj = DATA + 0x5000;
                    let table = DATA + 0x5100;
                    let table_slot = table + 4 * 0x23;
                    c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
                    c.bus.write32(world_obj + 8, table);
                    if obj_kind == 0 {
                        c.bus.write32(table_slot, 0);
                    } else {
                        let s = DATA + 0x5200;
                        c.bus.write32(table_slot, s);
                        c.bus.write32(s + 0x1c, DATA + 0x5300);
                        let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                        c.bus.write32(DATA + 0x5304, word);
                    }
                } else {
                    let d = DATA + 0x5400;
                    c.bus.write32(GAME_GP + WOF_SPECIAL, d);
                    c.bus.write32(d + 0x290, obj);
                }
                if obj != 0 {
                    let p = DATA + 0x5900;
                    match obj_kind {
                        2 => c.bus.write32(obj + 0xc, 0),
                        3 => {
                            let vt = vtable(c, &mut r, &[0x80]);
                            c.bus.write32(p + 0xc, vt);
                            c.bus.write32(obj + 0xc, p);
                        }
                        _ => {
                            let vt = DATA + 0x5800;
                            c.bus.write16(vt + 0x80, 0);
                            c.bus.write32(vt + 0x84, ZERO_LEAF);
                            c.bus.write32(p + 0xc, vt);
                            c.bus.write32(obj + 0xc, p);
                        }
                    }
                }

                c.g[A0][0] = sx(DATA + 0x700);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ents);
                c.pc = 0x002f_ded0;
            },
        );
    }

    /// `self` at DATA+0x800, the stream at DATA+0x10 whose vtable slot 8 is a
    /// fake write (random `this` delta; some calls unwind).
    #[test]
    fn crowd_man2d_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        covered(
            ee::s05::f_00229e20__generated,
            crowd_man2d_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_9e20, 1);
                let stream = DATA + 0x10;
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0022_9e20;
            },
        );
    }
}
