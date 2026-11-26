//! Staging module oc715 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc715`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_e8e0,
        cov_state_overlay_render,
        "cOVStateOverlay_Render",
    ),
    (0x0039_eca8, uistatemgr_render_stub, "UIStateMgr_RenderStub"),
    (0x003a_4e80, uiicon_dtor, "UIIcon_dtor"),
    (
        0x002a_7b08,
        ini_parser_next_section,
        "IniParser_NextSection",
    ),
    (0x002d_cf28, cboardwakefx_reset, "cBoardWakeFX_Reset"),
];

// ---------------------------------------------------------------------------
// Callees.

const UI_STATE_RENDER: u32 = 0x0039_e6b8;

const UI_OBJECT_DTOR: Callee = Callee {
    addr: 0x0039_fc48,
    f: ee::s13::f_0039fc48,
};

const INI_SKIP_COMMENT: Callee = Callee {
    addr: 0x002a_79c8,
    f: ee::s08::f_002a79c8,
};

const INI_GET_CHAR: Callee = Callee {
    addr: 0x002a_78e0,
    f: ee::s08::f_002a78e0,
};

const INI_EXPECT: Callee = Callee {
    addr: 0x002a_7ef0,
    f: ee::s08::f_002a7ef0,
};

const BWFX_RESET_HEAD: Callee = Callee {
    addr: 0x002d_e368,
    f: ee::s09::f_002de368,
};

/// The `gp` register as a guest address.
fn gp_of(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

// ---------------------------------------------------------------------------
// cOVStateOverlay_Render

/// `void cOVStateOverlay_Render(void)` (0x20e8e0): `UIState_Render()`, a
/// forwarding wrapper (7 cycles, 0x10 frame, tail call through the table).
pub fn cov_state_overlay_render(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(UI_STATE_RENDER), 0x0020_e8f0)
}

// ---------------------------------------------------------------------------
// UIStateMgr_RenderStub

/// `void UIStateMgr_RenderStub(void)` (0x39eca8): an empty render hook.
pub fn uistatemgr_render_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// UIIcon_dtor

/// `void UIIcon_dtor(UIIcon* self)` (0x3a4e80): set the icon's vtable word
/// (+0x8, 0x494aa8) and run `UIObject_dtor(self)`.
pub fn uiicon_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa8));
        let vtable = c.g[V0][0];
        let obj = c.g[V1][0] as u32;
        c.sw(obj.wrapping_add(0x8), vtable);
        UI_OBJECT_DTOR.call(c, 0x003a_4e9c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IniParser_NextSection

/// Offset of `mPtr` (the text cursor) in the parser.
const INI_MPTR: u32 = 0x8;
/// Offset of `mTempStr` (the section name buffer) in the parser.
const INI_TEMP: u32 = 0xc;
/// `'['` and `']'`, as the loaded byte values.
const SECTION_OPEN: u64 = 0x5b;
const SECTION_CLOSE: u64 = 0x5d;

fn s0_addr(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

/// `char* IniParser_NextSection(IniParser* p)` (0x2a7b08): skip comments
/// until a `[name]` line, copy the name into `p->mTempStr` (+0xc) and return
/// that buffer after `IniParser_Expect(p, "]")`. Returns 0 at the end of the
/// text (or when the closing token does not match).
pub fn ini_parser_next_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        frame.save(c, 20, 0x20);
        frame.save(c, 21, 0x10);
        frame.save_ra(c, 0);
        let v0 = ini_find_section(c)?;
        c.g[V0][0] = v0;
        c.cycles += 9;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The scan loop of [`ini_parser_next_section`]: returns the value left in `v0`.
fn ini_find_section(c: &mut Cpu) -> Result<u64, Unwound> {
    c.g[V0][0] = Addr(s0_addr(c).wrapping_add(INI_MPTR)).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
    c.g[S1][0] = SECTION_OPEN;
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[S2][0] = SECTION_CLOSE;
        c.g[S4][0] = sx(s0_addr(c).wrapping_add(INI_TEMP));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            INI_SKIP_COMMENT.call(c, 0x002a_7b50)?;
            c.g[A0][0] = c.g[S0][0];
            INI_GET_CHAR.call(c, 0x002a_7b58)?;
            if c.g[V0][0] == c.g[S1][0] {
                return ini_scan_name(c);
            }
            c.g[V0][0] = Addr(s0_addr(c).wrapping_add(INI_MPTR)).lw(c);
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_7b48);
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    Ok(0)
}

/// Copy the name after a `[` into `mTempStr` and check the closing token.
/// Every path returns the value of `v0`.
fn ini_scan_name(c: &mut Cpu) -> Result<u64, Unwound> {
    c.cycles += 5;
    c.g[V1][0] = Addr(s0_addr(c).wrapping_add(INI_MPTR)).lw(c);
    c.g[S1][0] = 0;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    let empty = c.g[V0][0] == c.g[S2][0];
    c.g[S5][0] = 0x4a_0000;
    if !empty {
        c.cycles += 3;
        c.g[S2][0] = sx(s0_addr(c).wrapping_add(INI_TEMP));
        c.g[S3][0] = SECTION_CLOSE;
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            INI_GET_CHAR.call(c, 0x002a_7b88)?;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32));
            let ch = c.g[V0][0];
            Addr(c.g[V1][0] as u32).sb(c, ch);
            c.g[V0][0] = Addr(s0_addr(c).wrapping_add(INI_MPTR)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
            let more = c.g[V1][0] != c.g[S3][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_7b80);
            }
        }
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3720));
    INI_EXPECT.call(c, 0x002a_7bac)?;
    let expect_failed = c.g[V0][0] != 0;
    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
    if expect_failed {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return Ok(0);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[S4][0];
    Addr(c.g[V1][0] as u32).sb(c, 0);
    Ok(c.g[V0][0])
}

// ---------------------------------------------------------------------------
// cBoardWakeFX_Reset

/// `void cBoardWakeFX_reset(sWakeDCF28* self)` (0x2dcf28): the wake has 5
/// columns when its board has a view index (else 4), a white colour, and
/// per-column tables (heights and values) laid out again when the column
/// count changed; the sub-params are cleared through `cWakeParams_Clear`.
pub fn cboardwakefx_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0x40);
        let this = s0_addr(c);
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x870)).lw(c);
        c.g[S1][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        if (c.g[V1][0] as i64) < 0 {
            c.cycles += 3;
            c.f[0] = Addr(gp_of(c).wrapping_sub(0x3c44)).read_fpu(c).0;
            c.g[V0][0] = 4;
            c.g[V1][0] = 0x16;
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x3fa0_0000;
            c.f[0] = c.g[AT][0] as u32;
            c.g[V0][0] = 5;
            c.g[V1][0] = 0x20;
        }

        // The 'b_2dd0a4 block: store the count, the two factors, the white
        // colour and the zeroed fields, then clear the sub-params.
        c.cycles += 27;
        c.sw(this.wrapping_add(0xb4), c.g[V0][0]);
        c.sw(this.wrapping_add(0xb8), c.g[V1][0]);
        let factor = FpuF32(c.f[0]);
        Addr(this.wrapping_add(0xbc)).write_fpu(c, factor);
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = c.g[AT][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(0x60));
        let sp0 = sp(c);
        let one = FpuF32(c.f[20]);
        for k in 0..4 {
            Addr(sp0.wrapping_add(4 * k)).write_fpu(c, one);
        }
        c.ldl(2, sp0.wrapping_add(7));
        c.ldr(2, sp0);
        c.ldl(3, sp0.wrapping_add(0xf));
        c.ldr(3, sp0.wrapping_add(8));
        c.sdl(2, this.wrapping_add(0xa7));
        c.sdr(2, this.wrapping_add(0xa0));
        c.sdl(3, this.wrapping_add(0xaf));
        c.sdr(3, this.wrapping_add(0xa8));
        c.sw(this.wrapping_add(0x38), 0);
        c.sw(this.wrapping_add(0x4), 0);
        c.sw(this.wrapping_add(0x8), 0);
        c.sw(this.wrapping_add(0x3c), 0);
        c.sw(this.wrapping_add(0x40), 0);
        BWFX_RESET_HEAD.call(c, 0x002d_cfd0)?;

        // The callee's return may have moved `s0` only if it misbehaved; the
        // generated code reads it from the register, so do the same.
        let this = s0_addr(c);
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        if c.g[S1][0] != c.g[V0][0] {
            c.cycles += 7;
            c.f[6] = Addr(gp_of(c).wrapping_sub(0x3c40)).read_fpu(c).0;
            c.f[3] = c.f[20];
            c.f[5] = Addr(gp_of(c).wrapping_sub(0x3c3c)).read_fpu(c).0;
            c.g[A1][0] = 0;
            c.f[4] = Addr(gp_of(c).wrapping_sub(0x3c38)).read_fpu(c).0;
            c.g[T2][0] = sx(this.wrapping_add(0x10));
            c.g[T1][0] = sx(this.wrapping_add(0x24));
            loop {
                // One column i: heightFactor = 0.99 - i * (0.98 / (count - 1)).
                c.cycles += 16;
                c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
                c.f[1] = c.g[A1][0] as u32;
                c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
                c.g[A3][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.f[0] = c.g[V0][0] as u32;
                c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                c.f[2] = c.f[1];
                c.g[A0][0] = 0;
                c.g[A2][0] = 0x1f;
                c.f[0] = FpuF32(c.f[6]).div(FpuF32(c.f[0])).0;
                c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
                c.f[0] = (FpuF32(c.f[5]) - FpuF32(c.f[1])).0;
                // The 32 entries of the column: a = 0, b = heightFactor,
                // c = 1.0, d = 0; the entries are 0x30 bytes apart.
                let row_slot = c.g[A1][0] as u32;
                loop {
                    c.cycles += 15;
                    let j_off = c.g[A0][0] as u32;
                    let elem = j_off.wrapping_add(Addr(row_slot).lw(c) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    c.g[V1][0] = sx(elem);
                    c.sw(elem, 0);
                    let w0 = FpuF32(c.f[0]);
                    let w3 = FpuF32(c.f[3]);
                    c.g[V0][0] = sx(j_off.wrapping_add(Addr(row_slot).lw(c) as u32));
                    Addr((c.g[V0][0] as u32).wrapping_add(4)).write_fpu(c, w0);
                    c.g[V1][0] = sx(j_off.wrapping_add(Addr(row_slot).lw(c) as u32));
                    Addr((c.g[V1][0] as u32).wrapping_add(8)).write_fpu(c, w3);
                    c.g[V0][0] = sx(j_off.wrapping_add(Addr(row_slot).lw(c) as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
                    let more = (c.g[A2][0] as i64) >= 0;
                    c.g[A0][0] = sx(j_off.wrapping_add(0x30));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_d038);
                    }
                }
                // The column's value: vals[i] = i / (count - 1) * -25.
                c.cycles += 11;
                c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[A1][0] = c.g[T0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[V1][0] = ((c.g[A1][0] as i64) < 5) as u64;
                c.f[0] = c.g[V0][0] as u32;
                c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
                c.f[0] = FpuF32(c.f[2]).div(FpuF32(c.f[0])).0;
                c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
                let more = c.g[V1][0] != 0;
                let vals_slot = c.g[A0][0] as u32;
                let value = FpuF32(c.f[0]);
                Addr(vals_slot).write_fpu(c, value);
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_cff8);
                }
            }
        }
        frame.restore(c, 16, 0x30);

        // Epilogue.
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, fall_back, hook};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate for the fakes below: they run only when the machine holds this
    /// word, so other tests reaching the same slots run the real bodies.
    const OC715_GATE: u32 = DATA + 0x200000;
    const OC715_MAGIC: u32 = 0x715c_0de5;
    static OC715_PREV_RENDER: AtomicUsize = AtomicUsize::new(0);
    static OC715_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);

    fn oc715_fake_ui_state_render(c: &mut Cpu) {
        if c.bus.read32(OC715_GATE) != OC715_MAGIC {
            return fall_back(c, &OC715_PREV_RENDER, ee::s13::f_0039e6b8__generated);
        }
        fake_call(c, UI_STATE_RENDER);
    }

    fn oc715_fake_object_dtor(c: &mut Cpu) {
        if c.bus.read32(OC715_GATE) != OC715_MAGIC {
            return fall_back(c, &OC715_PREV_DTOR, ee::s13::f_0039fc48__generated);
        }
        fake_call(c, 0x0039_fc48);
    }

    /// The token `IniParser_Expect` compares with (a global string).
    const OC715_INI_TOKEN: u32 = 0x004a_3720;
    /// Text pieces for the INI fixture (comments end at a newline).
    const OC715_INI_PIECES: &[&[u8]] = &[b"a ", b"[", b"[]", b"[ab]", b";c\n", b"\n", b"]"];

    #[test]
    fn cov_state_overlay_render_matches() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E6B8),
            oc715_fake_ui_state_render,
            &OC715_PREV_RENDER,
        );
        check(
            ee::s05::f_0020e8e0__generated,
            cov_state_overlay_render,
            50,
            &|c, seed| {
                let mut r = Rng(0x0020_e8e0 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC715_GATE, OC715_MAGIC);
            },
        );
    }

    #[test]
    fn uistatemgr_render_stub_matches() {
        check(
            ee::s13::f_0039eca8__generated,
            uistatemgr_render_stub,
            50,
            &|c, seed| {
                let mut r = Rng(0x0039_eca8 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn uiicon_dtor_matches() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039FC48),
            oc715_fake_object_dtor,
            &OC715_PREV_DTOR,
        );
        check(
            ee::s13::f_003a4e80__generated,
            uiicon_dtor,
            100,
            &|c, seed| {
                let mut r = Rng(0x003a_4e80 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC715_GATE, OC715_MAGIC);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
            },
        );
    }

    #[test]
    fn ini_parser_next_section_matches() {
        check(
            ee::s08::f_002a7b08__generated,
            ini_parser_next_section,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_7b08 ^ seed as u64);
                scramble(c, &mut r);
                let p = DATA + 0x100;
                let text = DATA + 0x400;
                c.g[A0][0] = sx(p);
                c.bus.write32(p + INI_MPTR, text);
                c.bus
                    .write8(OC715_INI_TOKEN, if seed % 3 == 0 { b'x' } else { b']' });
                c.bus.write8(OC715_INI_TOKEN + 1, 0);
                let mut at = text;
                for _ in 0..(r.next_u32() % 12) {
                    let piece =
                        OC715_INI_PIECES[(r.next_u32() % OC715_INI_PIECES.len() as u32) as usize];
                    for b in piece {
                        c.bus.write8(at, *b);
                        at += 1;
                    }
                }
                // A closing bracket and the terminator end every text.
                c.bus.write8(at, b']');
                c.bus.write8(at + 1, 0);
            },
        );
    }

    #[test]
    fn cboardwakefx_reset_matches() {
        check(
            ee::s09::f_002dcf28__generated,
            cboardwakefx_reset,
            200,
            &|c, seed| {
                let mut r = Rng(0x002d_cf28 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                for (off, f) in [
                    (0x3c44u32, 0.8f32),
                    (0x3c40, 0.98),
                    (0x3c3c, 0.99),
                    (0x3c38, -25.0),
                ] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), f.to_bits());
                }
                let this = DATA + 0x100;
                let obj = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, obj);
                let view = if seed % 2 == 0 { 0xffff_ffff } else { seed % 7 };
                c.bus.write32(obj + 0x870, view);
                // The old count is 4 or 5, so both the same-count and the
                // rebuild paths run.
                c.bus.write32(this + 0xb4, 4 + (seed / 2) % 2);
                for i in 0..5 {
                    c.bus
                        .write32(this + 0x10 + 4 * i, DATA + 0x2000 + i * 0x800);
                }
            },
        );
    }
}
