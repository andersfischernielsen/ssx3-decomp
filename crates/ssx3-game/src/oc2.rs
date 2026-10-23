//! Staging module: the options controller's widget action (0x18ca10).
//!
//! Ported for the tick-rate pass: its vibration timers at `+0x48`/`+0x4c` are
//! 60-tick counts (1 s at 60 Hz), converted to `tick::ticks_from_frames60` in
//! the next wave. The body is a transcription of the generated code (the
//! authority for registers, cycles and call order); the callees are typed and
//! named after the reference.

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::returned;
use crate::call::Callee;
use crate::call::Flow;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_ca10, fe_options_controller_on_widget_action, "cFEStateOptionsController_OnWidgetAction"),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) const CBE_GET_BE: Callee = Callee { addr: 0x0014_dc80, f: ee::s01::f_0014dc80 };
    pub(crate) const CBE_GET_INTERFACE: Callee =
        Callee { addr: 0x0014_dd58, f: ee::s01::f_0014dd58 };
    pub(crate) const MEM_ALLOC: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::f_00317d70 };
    pub(crate) const STR_HASH: Callee = Callee { addr: 0x0031_7670, f: ee::s10::f_00317670 };
    pub(crate) const UI_POPUP_CTOR: Callee = Callee { addr: 0x0039_e318, f: ee::s13::f_0039e318 };
    pub(crate) const UI_SCREEN_ADD: Callee = Callee { addr: 0x0039_f290, f: ee::s13::f_0039f290 };
    pub(crate) const UI_SCREEN_ADD_OBJECT: Callee =
        Callee { addr: 0x0039_f400, f: ee::s13::f_0039f400 };
    pub(crate) const WIDGET_CHECKED: Callee = Callee { addr: 0x0039_a738, f: ee::s13::f_0039a738 };
    pub(crate) const VIBRATION_SET: Callee = Callee { addr: 0x0014_7528, f: ee::s01::f_00147528 };
    pub(crate) const VIBRATION_APPLY: Callee = Callee { addr: 0x0014_74e8, f: ee::s01::f_001474e8 };
    pub(crate) const SET_VIBRATION: Callee = Callee { addr: 0x0018_d108, f: ee::s02::f_0018d108 };
    pub(crate) const SHOW_CONFIG_TEXT: Callee =
        Callee { addr: 0x0018_cf50, f: ee::s02::f_0018cf50 };
    pub(crate) const CONFIG_SET: Callee = Callee { addr: 0x0014_7658, f: ee::s01::f_00147658 };
}

/// `jalr` through a vtable entry (the target is data).
#[inline]
pub(crate) fn call_vtable(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    c.g[RA][0] = sx(ret);
    c.pc = target;
    call_indirect(c, target);
    returned(c, ret)
}

/// `void cFEStateOptionsController_OnWidgetAction(self, widget, event)`
/// (0x18ca10): the options screen's widget actions. Event 7 closes a popup,
/// event 6 opens the config popup, event 9 toggles vibration (ids
/// `D_004A1568`/`D_004A1570`, timers `+0x48`/`+0x4c`, flags `+0x50` bits 0/1)
/// and switches the config text (`D_004A15C8`/`D_004A1578`).
pub fn fe_options_controller_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18cccc: {
        'b_18ccc8: {
        'b_18ccc4: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if t {
        break 'b_18ccc4;
        } else {
        'b_18cb34: {
        'b_18cadc: {
        'b_18ca88: {
        c.cycles += 9;
        c.g[RA][0] = sx(0x0018ca40);
        call_vtable(c, 0x0014dc80, 0x0018ca40)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018ca4c);
        c.g[A1][0] = 0x1;
        call_vtable(c, 0x0014dd58, 0x0018ca4c)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = 0x7;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x8u64) as u64;
        if t {
        break 'b_18ca88;
        } else {
        'b_18ca74: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x6;
        if t {
        break 'b_18ca74;
        } else {
        c.cycles += 2;
        let t = c.g[S0][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18cadc;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_18cccc;
        } else {
        break 'b_18ca74;
        }
        }
        }
        }
        c.cycles += 3;
        c.g[V0][0] = 0x9;
        let t = c.g[S0][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18cb34;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_18cccc;
        } else {
        break 'b_18ca88;
        }
        }
        }
        }
        c.cycles += 21;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x4c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x0018caa0);
        c.g[A3][0] = 0u64;
        call_vtable(c, 0x00317d70, 0x0018caa0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0018cab4);
        c.g[A2][0] = c.g[S2][0];
        call_vtable(c, 0x0039e318, 0x0018cab4)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        c.g[RA][0] = sx(0x0018cad4);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
        call_vtable(c, 0x0039f290, 0x0018cad4)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18ccc8;
        } else {
        break 'b_18cadc;
        }
        }
        c.cycles += 16;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018caf0);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_vtable(c, t, 0x0018caf0)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0018cb14);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_vtable(c, t, 0x0018cb14)?;
        let t = c.g[V0][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18ccc8;
        } else {
        c.cycles += 6;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018cb2c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_vtable(c, 0x0039f400, 0x0018cb2c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18ccc8;
        } else {
        break 'b_18cb34;
        }
        }
        }
        'b_18ccc0: {
        'b_18ccbc: {
        'b_18cc34: {
        'b_18cc24: {
        'b_18cbb0: {
        c.cycles += 6;
        c.g[A0][0] = 0x4a0000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cb44);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1568));
        call_vtable(c, 0x00317670, 0x0018cb44)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if t {
        break 'b_18cbb0;
        } else {
        'b_18cba4: {
        c.cycles += 11;
        c.g[RA][0] = sx(0x0018cb54);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cb54)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0018cb64);
        c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
        call_vtable(c, 0x00147528, 0x0018cb64)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cb70);
        c.g[A1][0] = 0u64;
        call_vtable(c, 0x001474e8, 0x0018cb70)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S2][0];
        if t {
        break 'b_18cba4;
        } else {
        c.cycles += 11;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0018cb8c);
        c.mov_s(13, 12);
        callee::SET_VIBRATION.call(c, 0x0018cb8c)?;
        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = 0x3c;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] | 0x1;
        let t = true;
        c.sb((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        if t {
        break 'b_18ccbc;
        } else {
        break 'b_18cba4;
        }
        }
        }
        c.cycles += 3;
        c.mtc1(0, 12);
        let t = true;
        c.g[A1][0] = 0u64;
        if t {
        break 'b_18cc24;
        } else {
        break 'b_18cbb0;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cbbc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1570));
        call_vtable(c, 0x00317670, 0x0018cbbc)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if t {
        break 'b_18cc34;
        } else {
        'b_18cc1c: {
        c.cycles += 11;
        c.g[RA][0] = sx(0x0018cbcc);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cbcc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0018cbdc);
        c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
        call_vtable(c, 0x00147528, 0x0018cbdc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cbe8);
        c.g[A1][0] = 0x1;
        call_vtable(c, 0x001474e8, 0x0018cbe8)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S2][0];
        if t {
        break 'b_18cc1c;
        } else {
        c.cycles += 11;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0018cc04);
        c.mov_s(13, 12);
        callee::SET_VIBRATION.call(c, 0x0018cc04)?;
        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = 0x3c;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] | 0x2;
        let t = true;
        c.sb((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        if t {
        break 'b_18ccbc;
        } else {
        break 'b_18cc1c;
        }
        }
        }
        c.cycles += 2;
        c.mtc1(0, 12);
        c.g[A1][0] = 0x1;
        break 'b_18cc24;
        }
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x0018cc2c);
        c.mov_s(13, 12);
        callee::SET_VIBRATION.call(c, 0x0018cc2c)?;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_18ccc0;
        } else {
        break 'b_18cc34;
        }
        }
        'b_18cc7c: {
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cc40);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15c8));
        call_vtable(c, 0x00317670, 0x0018cc40)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if t {
        break 'b_18cc7c;
        } else {
        c.cycles += 13;
        c.g[RA][0] = sx(0x0018cc50);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cc50)?;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cc60);
        c.g[A2][0] = c.g[V0][0];
        call_vtable(c, 0x00147658, 0x0018cc60)?;
        c.g[RA][0] = sx(0x0018cc68);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cc68)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0018cc74);
        c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
        callee::SHOW_CONFIG_TEXT.call(c, 0x0018cc74)?;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_18ccc0;
        } else {
        break 'b_18cc7c;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cc88);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1578));
        call_vtable(c, 0x00317670, 0x0018cc88)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x1;
        if t {
        break 'b_18ccc0;
        } else {
        c.cycles += 11;
        c.g[RA][0] = sx(0x0018cc98);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cc98)?;
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cca8);
        c.g[A2][0] = c.g[V0][0];
        call_vtable(c, 0x00147658, 0x0018cca8)?;
        c.g[RA][0] = sx(0x0018ccb0);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018ccb0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0018ccbc);
        c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
        callee::SHOW_CONFIG_TEXT.call(c, 0x0018ccbc)?;
        break 'b_18ccbc;
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        break 'b_18ccc0;
        }
        c.cycles += 1;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c14), c.g[V0][0]);
        break 'b_18ccc4;
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        break 'b_18ccc8;
        }
        c.cycles += 1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_18cccc;
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, DATA};
    use crate::game::fe::feasyncfile::tests::base_3;

    /// `StrHash` of a `D_004A...` id string, by the generated hasher (the
    /// test machine holds the game's `.rodata`).
    fn hash_of(addr: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = 0x0012_3450;
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.pc = 0x0031_7670;
        (ee::s10::f_00317670)(&mut c);
        c.g[V0][0] as u32
    }

    const ID_VIBRATION_0: u32 = 0x004a_1568;
    const ID_VIBRATION_1: u32 = 0x004a_1570;
    const ID_CONFIG_V0: u32 = 0x004a_15c8;
    const ID_CONFIG_V1: u32 = 0x004a_1578;

    fn fixture(c: &mut Cpu, seed: u32) {
        let _r = base_3(c, seed, 0x18ca10, 1);
        c.pc = 0x0018_ca10;
        let self_ = DATA;
        let widget = DATA + 0x800;
        c.g[A0][0] = self_ as u64;
        c.g[A1][0] = if seed % 6 == 0 { 0 } else { widget as u64 };
        c.g[A2][0] = (seed % 12) as u64;
        c.bus.write32(self_ + 0x10, DATA + 0x1000);
        c.bus.write32(self_ + 0x48, if seed % 3 == 0 { 3 } else { 0 });
        c.bus.write32(self_ + 0x4c, if seed % 5 == 0 { 4 } else { 0 });
        c.bus.write8(self_ + 0x50, (seed % 4) as u8);
        let id = match seed % 4 {
            0 => ID_VIBRATION_0,
            1 => ID_VIBRATION_1,
            2 => ID_CONFIG_V0,
            _ => ID_CONFIG_V1,
        };
        let h = hash_of(id);
        c.bus
            .write32(widget + 0x38, if seed % 7 == 3 { 0 } else { h });
        c.bus.write32(widget + 0x18, 0);
    }

    #[test]
    fn on_widget_action_matches() {
        check(
            ee::s02::f_0018ca10__generated,
            fe_options_controller_on_widget_action,
            16,
            &|c, seed| {
                fixture(c, seed);
            },
        );
    }
}
