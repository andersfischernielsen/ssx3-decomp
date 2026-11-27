//! Staging module oc1084 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1084`.

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
    (
        0x003a_1360,
        ui_text_scroll_refresh_text,
        "cUITextScroll_RefreshText",
    ),
    (
        0x003a_1c80,
        ui_text_scroll_set_ascii_string,
        "cUITextScroll_ResetString",
    ),
    (
        0x003a_1f90,
        ui_text_scroll_remove_line,
        "cUITextScroll_RemoveLine",
    ),
    (0x003a_3f08, ui_vector_return_self, "UIVector_ReturnSelf"),
    (0x003a_4900, ui_object_ctor_2, "UIObject_ctor_2"),
];

/// A field of a guest object: `base + off` as an address (the `addiu` the
/// generated code folds into each load and store).
#[inline(always)]
fn ui_text_field(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `void cUITextScroll_RefreshText(cUITextScroll* self)` (0x3a1360): reads
/// the text id at `self+0xc0`; when set, resolves the owner chain
/// (`self+0x5c` font list, `+0xd0` table, `+0x10` owner, `+0x10` text object)
/// and calls the text object's vtable slot at `+0x20` (`this + delta`) for
/// the wide string, replaces the string with it (`cUITextScroll_ReplaceWideString`
/// when the call returns non-null), then calls the vtable slot at `self+8`
/// `+0x28` to relayout.
pub fn ui_text_scroll_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = ui_text_field(c.g[S0][0], 0xc0).lw(c);
        if c.g[A1][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = ui_text_field(c.g[S0][0], 0x5c).lw(c);
            c.g[V1][0] = ui_text_field(c.g[V0][0], 0xd0).lw(c);
            c.g[A0][0] = ui_text_field(c.g[V1][0], 0x10).lw(c);
            c.g[A2][0] = ui_text_field(c.g[A0][0], 0x10).lw(c);
            if c.g[A2][0] == 0 {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 7;
                c.g[V1][0] = ui_text_field(c.g[A2][0], 4).lw(c);
                c.g[A0][0] = ui_text_field(c.g[V1][0], 0x20).lh(c);
                c.g[V0][0] = ui_text_field(c.g[V1][0], 0x24).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_13a8)?;
                let text_is_null = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !text_is_null {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    let replace = Callee {
                        addr: 0x003a_1cf0,
                        f: ee::s13::f_003a1cf0,
                    };
                    replace.call(c, 0x003a_13b8)?;
                }
                c.cycles += 7;
                c.g[V0][0] = ui_text_field(c.g[S0][0], 8).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = ui_text_field(c.g[V0][0], 0x28).lh(c);
                c.g[V1][0] = ui_text_field(c.g[V0][0], 0x2c).lw(c);
                let relayout = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, relayout, 0x003a_13d0)?;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_ResetString(cUITextScroll* self, char* str)`
/// (0x3a1c80): clears the lines, resets the line count, calls the vtable
/// slots at `self+8` (`+0x28`, then `+0xc0`), and sets the string.
pub fn ui_text_scroll_set_ascii_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let clear = Callee {
            addr: 0x003a_18c0,
            f: ee::s13::f_003a18c0,
        };
        clear.call(c, 0x003a_1c9c)?;
        c.g[V0][0] = ui_text_field(c.g[S0][0], 8).lw(c);
        c.g[A1][0] = 1;
        ui_text_field(c.g[S0][0], 0xc0).sw(c, 0);
        c.g[A0][0] = ui_text_field(c.g[V0][0], 0x28).lh(c);
        c.g[V1][0] = ui_text_field(c.g[V0][0], 0x2c).lw(c);
        let first = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, first, 0x003a_1cb8)?;
        c.g[V0][0] = ui_text_field(c.g[S0][0], 8).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = ui_text_field(c.g[V0][0], 0xc0).lh(c);
        c.g[V1][0] = ui_text_field(c.g[V0][0], 0xc4).lw(c);
        let second = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, second, 0x003a_1cd0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        let set_string = Callee {
            addr: 0x003a_19f8,
            f: ee::s13::f_003a19f8,
        };
        set_string.call(c, 0x003a_1cdc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_RemoveLine(cUITextScroll* self, int idx)` (0x3a1f90):
/// when the line array is set and `idx` is below the line count, frees the
/// array entry (`EA_MemFree`), moves the tail down one slot and shrinks the
/// count; the array is reallocated to the new size first when it is not
/// empty.
pub fn ui_text_scroll_remove_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        'exit: {
            c.g[V0][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
            c.g[S1][0] = c.g[A1][0];
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 4;
            c.g[A0][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let idx_below_count = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            if !idx_below_count {
                break 'exit;
            }
            'grow: {
                c.cycles += 8;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40a8));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x003a_1fdc)?;
                let idx_non_positive = (c.g[S1][0] as i64) <= 0;
                c.g[S2][0] = c.g[V0][0];
                if idx_non_positive {
                    break 'grow;
                }
                c.cycles += 4;
                c.g[A1][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
                call_ptr(c, 0x0041_605c, 0x003a_1ff4)?;
            }
            'tail: {
                c.cycles += 5;
                c.g[A2][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
                if t {
                    break 'tail;
                }
                c.cycles += 8;
                c.g[A1][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                call_ptr(c, 0x0041_605c, 0x003a_2028)?;
            }
            'free: {
                c.cycles += 3;
                c.g[A0][0] = ui_text_field(c.g[S0][0], 0xbc).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
                    break 'free;
                }
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x003a_203c)?;
                c.g[V0][0] = ui_text_field(c.g[S0][0], 0xc8).lhu(c);
            }
            c.cycles += 3;
            let new_array = c.g[S2][0];
            ui_text_field(c.g[S0][0], 0xbc).sw(c, new_array);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let new_count = c.g[V0][0];
            ui_text_field(c.g[S0][0], 0xc8).sh(c, new_count);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIVector* UIVector_ReturnSelf(UIVector* self, ...)` (0x3a3f08): spills
/// the argument registers and the f12..f18 arguments to the caller's frame
/// and returns `self` in v0.
pub fn ui_vector_return_self(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    for (reg, off) in [
        (A1, 0x48),
        (A2, 0x50),
        (A3, 0x58),
        (T0, 0x60),
        (T1, 0x68),
        (T2, 0x70),
        (T3, 0x78),
    ] {
        let slot = frame.at(c, off);
        let value = c.g[reg][0];
        c.sd(slot, value);
    }
    for (fpr, off) in [(12, 0x38), (14, 0x3c), (16, 0x40), (18, 0x44)] {
        let slot = frame.at(c, off);
        c.swc1(fpr, slot);
    }
    frame.close(c);
}

/// `void UIObject_ctor_2(UIObject* self)` (0x3a4900): stores the object's
/// vtable word and two self pointers, then runs the flags constructor.
/// Returns `self` in v0.
pub fn ui_object_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4bd8));
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        ui_text_field(this, 4).sw(c, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x14));
        ui_text_field(this, 0).sw(c, this);
        let vtable = c.g[V0][0];
        ui_text_field(this, 8).sw(c, vtable);
        let flags_ctor = Callee {
            addr: 0x003a_6548,
            f: ee::s13::f_003a6548,
        };
        flags_ctor.call(c, 0x003a_492c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate magic: a gated fake answers with the logged call only when its
    /// own gate word holds this value in the test machine's RAM.
    const MAGIC: u32 = 0x1084_c0de;

    /// One gate word per fake, in the test data area (above the call log).
    const fn gate(k: u32) -> u32 {
        DATA + 0x20_0000 + 4 * k
    }
    const GATE_REPLACE: u32 = gate(0);
    const GATE_CLEAR: u32 = gate(1);
    const GATE_SET_STRING: u32 = gate(2);
    const GATE_FLAGS_CTOR: u32 = gate(3);
    const GATE_ALLOC: u32 = gate(4);
    const GATE_FREE: u32 = gate(5);
    const GATE_MOVE: u32 = gate(6);

    fn gates_on(c: &mut Cpu) {
        for g in [
            GATE_REPLACE,
            GATE_CLEAR,
            GATE_SET_STRING,
            GATE_FLAGS_CTOR,
            GATE_ALLOC,
            GATE_FREE,
            GATE_MOVE,
        ] {
            c.bus.write32(g, MAGIC);
        }
    }

    /// A gated fake: with its gate on, it logs the call and returns (the
    /// generated side and the port see the same call); with it off, it runs
    /// what the slot held before.
    macro_rules! gated {
        ($fake:ident, $prev:ident, $gate:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated!(
        oc1084_fake_replace,
        OC1084_PREV_REPLACE,
        GATE_REPLACE,
        0x003a_1cf0,
        ee::s13::f_003a1cf0__generated
    );
    gated!(
        oc1084_fake_clear,
        OC1084_PREV_CLEAR,
        GATE_CLEAR,
        0x003a_18c0,
        ee::s13::f_003a18c0__generated
    );
    gated!(
        oc1084_fake_set_string,
        OC1084_PREV_SET_STRING,
        GATE_SET_STRING,
        0x003a_19f8,
        ee::s13::f_003a19f8__generated
    );
    gated!(
        oc1084_fake_flags_ctor,
        OC1084_PREV_FLAGS_CTOR,
        GATE_FLAGS_CTOR,
        0x003a_6548,
        ee::s13::f_003a6548__generated
    );
    gated!(
        oc1084_fake_alloc,
        OC1084_PREV_ALLOC,
        GATE_ALLOC,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    gated!(
        oc1084_fake_free,
        OC1084_PREV_FREE,
        GATE_FREE,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );
    gated!(
        oc1084_fake_move,
        OC1084_PREV_MOVE,
        GATE_MOVE,
        0x0041_605c,
        ee::s15::f_0041605c__generated
    );

    /// Puts the gated fakes in their override slots (once per process).
    fn install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1CF0),
            oc1084_fake_replace,
            &OC1084_PREV_REPLACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A18C0),
            oc1084_fake_clear,
            &OC1084_PREV_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A19F8),
            oc1084_fake_set_string,
            &OC1084_PREV_SET_STRING,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6548),
            oc1084_fake_flags_ctor,
            &OC1084_PREV_FLAGS_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1084_fake_alloc,
            &OC1084_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1084_fake_free,
            &OC1084_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0041605C),
            oc1084_fake_move,
            &OC1084_PREV_MOVE,
        );
    }

    /// A vtable slot `{delta, func}` at `vt + off`.
    fn vt_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, func: u32) {
        c.bus.write16(vt + off, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// The fixture of `cUITextScroll_RefreshText`: the text id, the owner
    /// chain down to the text object, and the two vtables the calls read.
    /// Every fifth seed has no text object (the early exit), every fourth no
    /// id, every third a slot that returns null (the replace is skipped).
    fn refresh_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x3a13_60, 3);
        gates_on(c);
        let obj = DATA;
        let font = DATA + 0x100;
        let table = DATA + 0x200;
        let owner = DATA + 0x300;
        let text_obj = DATA + 0x500;
        let vt = DATA + 0x600;
        let vt2 = DATA + 0x700;
        c.g[A0][0] = obj as u64;
        let id = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(obj + 0xc0, id);
        c.bus.write32(obj + 0x5c, font);
        c.bus.write32(obj + 8, vt2);
        c.bus.write32(font + 0xd0, table);
        c.bus.write32(table + 0x10, owner);
        c.bus
            .write32(owner + 0x10, if seed % 5 == 0 { 0 } else { text_obj });
        c.bus.write32(text_obj + 4, vt);
        let null_func = if seed % 3 == 0 {
            0x0014_06c0
        } else {
            fakes::FAKE
        };
        vt_slot(c, &mut r, vt, 0x20, null_func);
        vt_slot(c, &mut r, vt2, 0x28, fakes::FAKE + 0x10);
    }

    #[test]
    fn ui_text_scroll_refresh_text_matches() {
        install();
        check(
            ee::s13::f_003a1360__generated,
            ui_text_scroll_refresh_text,
            300,
            &refresh_setup,
        );
    }

    /// The fixture of `cUITextScroll_ResetString`: the vtable at `self+8`
    /// with its two slots (`+0x28` and `+0xc0`), and a string argument.
    fn reset_string_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x3a1c_80, 3);
        gates_on(c);
        let obj = DATA;
        let vt = DATA + 0x600;
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = if seed % 3 == 0 {
            0
        } else {
            (DATA + 0x800 + (r.next_u32() & 0x3f0)) as u64
        };
        c.bus.write32(obj + 8, vt);
        vt_slot(c, &mut r, vt, 0x28, fakes::FAKE);
        vt_slot(c, &mut r, vt, 0xc0, fakes::FAKE + 0x10);
    }

    #[test]
    fn ui_text_scroll_set_ascii_string_matches() {
        install();
        check(
            ee::s13::f_003a1c80__generated,
            ui_text_scroll_set_ascii_string,
            200,
            &reset_string_setup,
        );
    }

    /// The fixture of `cUITextScroll_RemoveLine`: a line array of `count`
    /// entries (empty on every fourth seed) and an index that is negative,
    /// zero, at the count or inside the array.
    fn remove_line_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x3a1f_90, 4);
        gates_on(c);
        let obj = DATA;
        let lines = DATA + 0x400;
        let count = (seed % 6) as u16;
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = match seed % 5 {
            0 => u64::MAX,
            1 => count as u64,
            2 => 0,
            _ => (r.next_u32() % 8) as u64,
        };
        c.bus
            .write32(obj + 0xbc, if seed % 4 == 0 { 0 } else { lines });
        c.bus.write16(obj + 0xc8, count);
        for k in 0..8 {
            c.bus.write32(lines + 4 * k, DATA + 0x800 + 0x10 * k);
        }
    }

    #[test]
    fn ui_text_scroll_remove_line_matches() {
        install();
        check(
            ee::s13::f_003a1f90__generated,
            ui_text_scroll_remove_line,
            300,
            &remove_line_setup,
        );
    }

    #[test]
    fn ui_vector_return_self_matches() {
        check(
            ee::s13::f_003a3f08__generated,
            ui_vector_return_self,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a3f_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn ui_object_ctor_2_matches() {
        install();
        check(
            ee::s13::f_003a4900__generated,
            ui_object_ctor_2,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x3a49_00, 2);
                gates_on(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }
}
