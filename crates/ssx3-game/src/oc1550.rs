//! Staging module oc1550 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1550`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_b4d8, wp_file_begin_load, "WPFile_BeginLoad"),
    (0x002b_b5a8, wp_file_unload, "WPFile_Unload"),
    (0x002b_c4e0, wp_man_global_ctors, "cWorldPainterMan_GlobalCtors"),
    (0x002b_cd68, wp_igd_ambience_blend, "tWPIGD_Ambience_Blend"),
    (0x002b_ce08, wp_igd_speech_blend, "tWPIGD_Speech_Blend"),
    (0x002b_d968, wp_igd_danger_blend, "tWPIGD_Danger_Blend"),
    (
        0x002b_da98,
        wp_igd_music_trigger_is_equal,
        "tWPInGameData_MusicTrigger_isEqual",
    ),
    (0x002b_e078, wp_igd_danger_is_equal, "tWPInGameData_Danger_isEqual"),
];

/// `base + off` as the 32-bit address the generated code computes.
#[inline(always)]
fn wp_field(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `void cWorldPainterSection_load(cWorldPainterSection* self, const char*
/// fileName)` (0x2bb4d8, reference `cWorldPainterSection_load`): clears the
/// streamed flag at +2, builds `fileName + ".swp"` in a temporary string
/// (`cBXString` concatenation), starts an async load of it, stores the handle
/// at +4 and sets the state at +0 to 1 for a positive handle, 2 otherwise.
/// Both temporary strings are destroyed (flag 2) before the state is written.
pub fn wp_file_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        c.g[V0][0] = 0x4a_0000;
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = sx(frame.at(c, 0x10));
        Addr(wp_field(c.g[S1][0], 2)).sh(c, 0);
        c.g[A1][0] = sx(wp_field(c.g[V0][0], 0x3870));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_86d0, 0x002b_b508)?;
        c.g[A2][0] = 0x4a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(wp_field(c.g[A2][0], 0x3878));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0031_8630, 0x002b_b51c)?;
        c.g[A0][0] = Addr(sp(c)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x003d_f690, 0x002b_b528)?;
        let handle = c.g[V0][0];
        Addr(wp_field(c.g[S1][0], 4)).sw(c, handle);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x002b_b538)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x002b_b544)?;
        c.g[V0][0] = Addr(wp_field(c.g[S1][0], 4)).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = 1;
        if !positive {
            c.cycles += 1;
            c.g[V0][0] = 2;
        }
        c.cycles += 6;
        let state = c.g[V0][0];
        Addr(wp_field(c.g[S1][0], 0)).sh(c, state);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void WPFile_Unload(WPFile* self)` (0x2bb5a8): frees the data block at +8
/// when it is not borrowed (the flag at +2 is zero), clears the pointer, and
/// sets the state at +0 to 0.
pub fn wp_file_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(wp_field(c.g[S0][0], 8)).lw(c);
        if c.g[A0][0] == 0 {
            Addr(wp_field(c.g[S0][0], 0)).sh(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(wp_field(c.g[S0][0], 2)).lh(c);
            if c.g[V0][0] != 0 {
                Addr(wp_field(c.g[S0][0], 8)).sw(c, 0);
            } else {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x002b_b5d8)?;
                Addr(wp_field(c.g[S0][0], 8)).sw(c, 0);
            }
            c.cycles += 1;
            Addr(wp_field(c.g[S0][0], 0)).sh(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cWorldPainterMan_GlobalCtors(void)` (0x2bc4e0): the static
/// constructor of the world painter manager, `StaticInitDestroy(1, 0xffff)`.
pub fn wp_man_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x002b_bc38,
            f: ee::s08::f_002bbc38,
        }
        .call(c, 0x002b_c4f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The virtual call of a blend: the method at `vt + 0x218` (`lh` delta, `lw`
/// target) is called with `this + delta`, returning to `ret`.
fn wp_blend_call(c: &mut Cpu, this: u64, ret: u32) -> Flow {
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, 0x218);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(this);
    call_ptr(c, slot.func as u32, ret)
}

/// The bank blend shared by `tWPIGD_Ambience_Blend` and `tWPIGD_Speech_Blend`
/// (the two generated bodies are the same apart from their return addresses):
/// the blend factor is squared, the object's method is called (`first_ret`),
/// then the integer field at +8 becomes `b * rhs + (1 - b) * self` (`b` the
/// squared factor), the field at +0xc takes `rhs`'s value, and the method is
/// called again (`second_ret`).
fn wp_igd_bank_blend(c: &mut Cpu, first_ret: u32, second_ret: u32) -> Flow {
    c.cycles += 39;
    let frame = Frame::open(c, 0x40);
    frame.save(c, 16, 0x20);
    frame.save(c, 17, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save_fpr(c, 20, 0x30);
    c.mov_s(20, 12);
    c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
    c.mul_s(20, 20, 20);
    let this = c.g[S0][0];
    wp_blend_call(c, this, first_ret)?;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    let rhs_level = wp_field(c.g[S1][0], 4);
    c.lwc1(1, rhs_level);
    c.cvt_s_w(1, 1);
    let self_bank = wp_field(c.g[S0][0], 8);
    c.lwc1(2, self_bank);
    c.cvt_s_w(2, 2);
    c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
    c.sub_s(0, 0, 20);
    c.mul_s(20, 20, 1);
    c.mul_s(0, 0, 2);
    c.add_s(20, 20, 0);
    c.cvt_w_s(0, 20);
    let self_bank = wp_field(c.g[S0][0], 8);
    c.swc1(0, self_bank);
    c.g[V0][0] = Addr(wp_field(c.g[S1][0], 4)).lw(c);
    let stored = c.g[V0][0];
    Addr(wp_field(c.g[S0][0], 0xc)).sw(c, stored);
    let this = c.g[S0][0];
    wp_blend_call(c, this, second_ret)?;
    frame.restore(c, 16, 0x20);
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
    Ok(())
}

/// `void tWPIGD_Ambience_Blend(self, rhs, float blend)` (0x2bcd68).
pub fn wp_igd_ambience_blend(c: &mut Cpu) {
    guest(c, |c| wp_igd_bank_blend(c, 0x002b_cda0, 0x002b_cdec));
}

/// `void tWPIGD_Speech_Blend(self, rhs, float blend)` (0x2bce08): the same
/// blend as the ambience one, with its own call sites.
pub fn wp_igd_speech_blend(c: &mut Cpu) {
    guest(c, |c| wp_igd_bank_blend(c, 0x002b_ce40, 0x002b_ce8c));
}

/// `void tWPIGD_Danger_Blend(self, rhs, float blend)` (0x2bd968): squares the
/// blend factor, calls the method, then blends the level (integer, rounded
/// toward zero), the point rate (float) and the last-level field; the rate
/// blends `b * rhs + (1 - b) * self` with the squared factor.
pub fn wp_igd_danger_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
        c.mul_s(20, 20, 20);
        let this = c.g[S0][0];
        wp_blend_call(c, this, 0x002b_d9a0)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        let rhs_level = wp_field(c.g[S1][0], 4);
        c.lwc1(0, rhs_level);
        c.cvt_s_w(0, 0);
        let self_level = wp_field(c.g[S0][0], 8);
        c.lwc1(1, self_level);
        c.cvt_s_w(1, 1);
        let self_rate = wp_field(c.g[S0][0], 0x10);
        c.lwc1(3, self_rate);
        c.sub_s(2, 2, 20);
        c.g[V1][0] = Addr(wp_field(c.g[S0][0], 4)).lw(c);
        c.mul_s(0, 20, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(2, 2, 3);
        c.add_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        let self_level = wp_field(c.g[S0][0], 8);
        c.swc1(1, self_level);
        c.g[V0][0] = Addr(wp_field(c.g[S1][0], 4)).lw(c);
        let last_level = c.g[V0][0];
        Addr(wp_field(c.g[S0][0], 0xc)).sw(c, last_level);
        let rhs_rate = wp_field(c.g[S1][0], 8);
        c.lwc1(0, rhs_rate);
        c.mul_s(20, 20, 0);
        c.add_s(20, 20, 2);
        let self_rate = wp_field(c.g[S0][0], 0x10);
        c.swc1(20, self_rate);
        let rhs_rate = wp_field(c.g[S1][0], 8);
        c.lwc1(0, rhs_rate);
        let last_rate = wp_field(c.g[S0][0], 0x14);
        c.swc1(0, last_rate);
        let this = c.g[S0][0];
        wp_blend_call(c, this, 0x002b_da0c)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `bool tWPInGameData_MusicTrigger_isEqual(a, b)` (0x2bda98): false unless
/// `a+8 == b+4`; then true when `a+0x10 == b+8` (32-bit words, compared as
/// 64-bit register values).
pub fn wp_igd_music_trigger_is_equal(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(wp_field(c.g[A0][0], 8)).lw(c);
    c.g[V0][0] = Addr(wp_field(c.g[A1][0], 4)).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(wp_field(c.g[A0][0], 0x10)).lw(c);
        c.g[V1][0] = Addr(wp_field(c.g[A1][0], 8)).lw(c);
        let diff = c.g[V0][0] ^ c.g[V1][0];
        c.g[V0][0] = (diff < 1) as u64;
    }
    ret(c);
}

/// `bool tWPInGameData_Danger_isEqual(a, b)` (0x2be078): as the music trigger
/// one, but the last two words are floats compared with `c.eq.s`.
pub fn wp_igd_danger_is_equal(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(wp_field(c.g[A0][0], 8)).lw(c);
    c.g[V0][0] = Addr(wp_field(c.g[A1][0], 4)).lw(c);
    let differ = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if !differ {
        c.cycles += 5;
        let a_rate = wp_field(c.g[A0][0], 0x10);
        c.lwc1(1, a_rate);
        let b_rate = wp_field(c.g[A1][0], 8);
        c.lwc1(0, b_rate);
        c.c_eq_s(1, 0);
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = 1;
            ret(c);
            return;
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's gated fakes (in the machine's RAM).
    const OC1550_CTRL: u32 = DATA + 0x1_0000;
    const OC1550_MAGIC: u32 = 0x1550_c0de;

    static OC1550_PREV_CSTR_PLUS: AtomicUsize = AtomicUsize::new(0);
    static OC1550_PREV_STR_PLUS: AtomicUsize = AtomicUsize::new(0);
    static OC1550_PREV_ASYNC_LOAD: AtomicUsize = AtomicUsize::new(0);
    static OC1550_PREV_BX_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1550_PREV_MEM_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1550_PREV_STATIC_INIT: AtomicUsize = AtomicUsize::new(0);

    fn oc1550_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1550_CTRL) == OC1550_MAGIC
    }

    /// `cBXString_OpPlusCStrStr`: a logged fake while the gate is set.
    fn oc1550_fake_cstr_plus(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(
                c,
                &OC1550_PREV_CSTR_PLUS,
                ee::s10::f_003186d0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_86d0, ra);
    }

    /// `cBXString_OpPlusStrCStr`: a logged fake while the gate is set.
    fn oc1550_fake_str_plus(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_STR_PLUS, ee::s10::f_00318630__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_8630, ra);
    }

    /// `cBXString` destructor: a logged fake while the gate is set.
    fn oc1550_fake_bx_dtor(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_BX_DTOR, ee::s10::f_00318260__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_8260, ra);
    }

    /// `AsyncFile_Load`: a logged fake. Mailbox word +4 picks the handle: 1
    /// returns 0, 2 returns -1, anything else the fake's pointer (positive).
    fn oc1550_fake_async_load(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(
                c,
                &OC1550_PREV_ASYNC_LOAD,
                ee::s14::f_003df690__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003d_f690, ra);
        if c.pc == ra {
            match c.bus.read32(OC1550_CTRL + 4) {
                1 => c.g[V0][0] = 0,
                2 => c.g[V0][0] = u64::MAX,
                _ => {}
            }
        }
    }

    /// `EA_MemFree`: a logged fake while the gate is set.
    fn oc1550_fake_mem_free(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_MEM_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
    }

    /// `cWorldPainterMan_StaticInitDestroy`: a logged fake while the gate is set.
    fn oc1550_fake_static_init(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(
                c,
                &OC1550_PREV_STATIC_INIT,
                ee::s08::f_002bbc38__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002b_bc38, ra);
    }

    fn oc1550_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc1550_fake_cstr_plus,
            &OC1550_PREV_CSTR_PLUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc1550_fake_str_plus,
            &OC1550_PREV_STR_PLUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            oc1550_fake_async_load,
            &OC1550_PREV_ASYNC_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc1550_fake_bx_dtor,
            &OC1550_PREV_BX_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1550_fake_mem_free,
            &OC1550_PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BBC38),
            oc1550_fake_static_init,
            &OC1550_PREV_STATIC_INIT,
        );
    }

    /// `WPFile_BeginLoad` fixture: a file object at DATA+0x100, a name at
    /// DATA+0x200, the string and loader fakes gated on. The mailbox picks
    /// a positive, zero or negative handle.
    fn oc1550_load_setup(c: &mut Cpu, seed: u32) {
        let _r: Rng = fakes::machine(c, seed, 0x1550_01, 5);
        c.bus.write32(OC1550_CTRL, OC1550_MAGIC);
        const MODES: [u32; 4] = [0, 1, 0, 2];
        c.bus.write32(OC1550_CTRL + 4, MODES[(seed % 4) as usize]);
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = sx(DATA + 0x200);
    }

    #[test]
    fn wp_file_begin_load_matches() {
        oc1550_install();
        check(
            ee::s08::f_002bb4d8__generated,
            wp_file_begin_load,
            300,
            &|c, seed| oc1550_load_setup(c, seed),
        );
    }

    #[test]
    fn wp_file_unload_matches() {
        oc1550_install();
        check(
            ee::s08::f_002bb5a8__generated,
            wp_file_unload,
            200,
            &|c, seed| {
                let _r: Rng = fakes::machine(c, seed, 0x1550_02, 1);
                c.bus.write32(OC1550_CTRL, OC1550_MAGIC);
                let this = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 8, if seed % 4 == 0 { 0 } else { DATA + 0x500 });
                c.bus
                    .write16(this + 2, if seed % 3 == 0 { 0 } else { (seed as u16) | 1 });
            },
        );
    }

    #[test]
    fn wp_man_global_ctors_matches() {
        oc1550_install();
        check(
            ee::s08::f_002bc4e0__generated,
            wp_man_global_ctors,
            100,
            &|c, seed| {
                let _r: Rng = fakes::machine(c, seed, 0x1550_03, 1);
                c.bus.write32(OC1550_CTRL, OC1550_MAGIC);
            },
        );
    }

    /// A blend fixture: `self` at DATA+0x100 (method table at +4, bank at
    /// +8, level at +0xc, rate at +0x10), `rhs` at DATA+0x200 (level at +4,
    /// rate at +8), the blend factor in f12, and a method table at DATA+0x300
    /// whose slot +0x218 is a delta with a fake target (outside the table, so
    /// the miss hook logs the call).
    fn wp_blend_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1550_b1, 2);
        let this = DATA + 0x100;
        let rhs = DATA + 0x200;
        let vt = DATA + 0x300;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(rhs);
        c.f[12] = if seed % 5 == 0 {
            r.edge_f32()
        } else {
            r.f32(1.5)
        };
        c.bus.write32(this + 4, vt);
        c.bus.write32(this + 8, r.next_u32() % 2001);
        c.bus.write32(this + 0xc, r.next_u32() % 2001);
        c.bus.write32(this + 0x10, r.f32(10.0));
        c.bus.write32(rhs + 4, r.next_u32() % 2001);
        c.bus.write32(rhs + 8, r.f32(10.0));
        c.bus.write16(vt + 0x218, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + 0x21c, fakes::FAKE + 0x10 * (seed % 2));
    }

    #[test]
    fn wp_igd_ambience_blend_matches() {
        check(
            ee::s08::f_002bcd68__generated,
            wp_igd_ambience_blend,
            300,
            &|c, seed| wp_blend_setup(c, seed),
        );
    }

    #[test]
    fn wp_igd_speech_blend_matches() {
        check(
            ee::s08::f_002bce08__generated,
            wp_igd_speech_blend,
            300,
            &|c, seed| wp_blend_setup(c, seed),
        );
    }

    #[test]
    fn wp_igd_danger_blend_matches() {
        check(
            ee::s08::f_002bd968__generated,
            wp_igd_danger_blend,
            300,
            &|c, seed| wp_blend_setup(c, seed),
        );
    }

    #[test]
    fn wp_igd_music_trigger_is_equal_matches() {
        check(
            ee::s08::f_002bda98__generated,
            wp_igd_music_trigger_is_equal,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1550_07, 0);
                let a = DATA + 0x100;
                let b = DATA + 0x200;
                let w = r.next_u32();
                let x = r.next_u32();
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.bus.write32(a + 8, w);
                c.bus.write32(b + 4, if seed % 3 == 0 { w } else { r.next_u32() });
                c.bus.write32(a + 0x10, x);
                c.bus.write32(b + 8, if seed % 2 == 0 { x } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn wp_igd_danger_is_equal_matches() {
        check(
            ee::s08::f_002be078__generated,
            wp_igd_danger_is_equal,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1550_08, 0);
                let a = DATA + 0x100;
                let b = DATA + 0x200;
                let w = r.next_u32();
                let f = if seed % 4 == 0 { r.edge_f32() } else { r.f32(3.0) };
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.bus.write32(a + 8, w);
                c.bus.write32(b + 4, if seed % 3 == 0 { w } else { r.next_u32() });
                c.bus.write32(a + 0x10, f);
                c.bus.write32(b + 8, if seed % 2 == 0 { f } else { r.f32(3.0) });
            },
        );
    }
}
