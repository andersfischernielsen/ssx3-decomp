//! `src/ui/focusman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_32c0, focus_man_add_object, "FocusMan_AddObject"),
    (0x0010_33b0, focus_man_add_focus_box, "FocusMan_AddFocusBox"),
];


// ---------------------------------------------------------------------------
// Focus manager.

/// `void FocusMan_AddObject(FocusMan *self, FocusObject *obj)` (0x1032c0):
/// tag `obj->flags` (+0x8) with 0x100; store the new flags — OR 0x300 when
/// `self->locked` (+0xd4) is zero, else the tagged value with 0x200 cleared;
/// append `obj` to the list at +0xdc (count at +0xd8) and bump the count.
pub fn focus_man_add_object(c: &mut Cpu) {
    const LOCKED: u32 = 0xd4;
    const FLAGS: u32 = 0x8;
    const COUNT: u32 = 0xd8;
    const OBJECTS: u32 = 0xdc;
    c.cycles += 6;
    let this = abi::arg(c, 0);
    let obj = Addr(abi::arg(c, 1));
    let flags = obj.off(FLAGS).lw(c);
    c.g[V1][0] = flags;
    let tagged = flags | 0x100;
    c.g[A2][0] = tagged;
    obj.off(FLAGS).sw(c, tagged);
    let locked = Addr(this).off(LOCKED).lw(c);
    c.g[V0][0] = locked;
    let stored = if locked == 0 {
        flags | 0x300
    } else {
        c.cycles += 3;
        tagged & !0x200u64
    };
    c.g[V0][0] = stored;
    c.cycles += 8;
    obj.off(FLAGS).sw(c, stored);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V1][0] = count;
    c.g[V0][0] = sx((count as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(c.g[V0][0] as u32).off(OBJECTS).sw(c, c.g[A1][0]);
    let ret = c.g[RA][0] as u32;
    Addr(this).off(COUNT).sw(c, c.g[V1][0]);
    c.pc = ret;
}


/// `void FocusMan_AddFocusBox(FocusMan *self, FocusBox *box)` (0x1033b0):
/// clear `self->active` (+0xd0) to -1, store `box` in slot `self->count`
/// (stride 0x20, +0x10), copy the box's quad to the next slot (+0x20) and to
/// the just-opened stack scratch, then bump the count.
pub fn focus_man_add_focus_box(c: &mut Cpu) {
    const COUNT: u32 = 0x0;
    const ACTIVE: u32 = 0xd0;
    const SLOT: u32 = 0x10;
    const QUAD: u32 = 0x20;
    c.cycles += 18;
    let this = abi::arg(c, 0);
    let box_ptr = abi::arg(c, 1);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V0][0] = count;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    Addr(this).off(ACTIVE).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = sx((count as u32) << 5);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(SLOT).sw(c, c.g[A1][0]);
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V1][0] = count;
    c.lq(6, box_ptr);
    c.g[V1][0] = sx((count as u32) << 5);
    c.g[V1][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
    c.sq(6, c.g[SP][0] as u32);
    c.sq(6, (c.g[V1][0] as u32).wrapping_add(QUAD));
    let count = Addr(this).off(COUNT).lw(c);
    c.g[V0][0] = count;
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    Addr(this).off(COUNT).sw(c, c.g[V0][0]);
    let ret = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ret;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;


    #[test]
    fn focus_man_add_object_matches() {
        check(ssx3_ee::s00::f_001032c0, focus_man_add_object, 300, &|c, seed| {
            let mut r = Rng(0x1032_c000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            let count = (seed % 5) as u32;
            c.bus.write32(DATA + 0xd4, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(DATA + 0xd8, count);
            c.bus.write32(DATA + 0x400 + 0x8, r.next_u32());
            c.bus.write32(DATA + count * 4 + 0xdc, r.next_u32());
        });
    }


    #[test]
    fn focus_man_add_focus_box_matches() {
        check(ssx3_ee::s00::f_001033b0, focus_man_add_focus_box, 200, &|c, seed| {
            let mut r = Rng(0x1033_b000 ^ seed as u64);
            scramble(c, &mut r);
            let count = (seed % 4) as u32;
            c.g[A0][0] = DATA as u64;
            // Odd seeds use a misaligned box to cover `lq` masking.
            c.g[A1][0] = (DATA + 0x600 + (seed % 2) * 8) as u64;
            c.bus.write32(DATA, count);
            for i in 0..4 {
                c.bus.write32(DATA + 0x600 + i * 4, r.next_u32());
            }
        });
    }
}
