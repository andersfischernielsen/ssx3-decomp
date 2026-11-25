//! Staging module oc640 (wave g20.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc640`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0022_4f68,
    luno_table_clear,
    "cLunoTable_Clear",
)];

/// The word at `off(base)` for a base register value (32-bit address math).
#[inline(always)]
fn oc640_field(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `cFEStateUberTrick::updateTrickMenuEntry(self)` (0x184520): refreshes the
/// uber-trick menu's cost text (the trick's cost from the table the category
/// lookup returns, as a dollar string) and its money text (the user's cash
/// for the character id). The trick and category are the two menus' rider
/// slot bytes.
///
/// Not in `OVERRIDES`: its callees `getDollarString` (0x198af0) and
/// `cUIText_setAsciiString` (0x3a0c50) have no override slot, so the
/// differential test cannot fake them, and running them on fixture data is
/// unbounded.
pub fn fe_uber_trick_update_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = oc640_field(c.g[S1][0], 0x50).lw(c);
        c.g[V0][0] = oc640_field(c.g[S1][0], 0x4c).lw(c);
        c.g[A1][0] = oc640_field(c.g[V1][0], 0xa0).lw(c);
        c.g[A0][0] = oc640_field(c.g[V0][0], 0xa0).lw(c);
        c.g[S4][0] = oc640_field(c.g[A1][0], 0x18).lb(c);
        c.g[S0][0] = oc640_field(c.g[A0][0], 0x18).lb(c);
        call_ptr(c, 0x0014_dc80, 0x0018_455c)?; // cBE_getBE()
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x6;
        call_ptr(c, 0x0014_dd58, 0x0018_4568)?; // cBE_getInterface_Fv(be, 6)
        c.g[A1][0] = oc640_field(c.g[S1][0], 0x44).lb(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[SP][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_ff90, 0x0018_457c)?; // the cost table for the category
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7840, 0x0018_4588)?; // cList_first(&mScreens)
        c.g[A0][0] = 0x0046_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2648));
        call_ptr(c, 0x0031_7670, 0x0018_4598)?; // GetHashValue32(str_cost_num)
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0039_d860, 0x0018_45a4)?; // cUIScreen_getObjectByHashName
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A0][0] = oc640_field(c.g[V0][0], 0x6).lhu(c);
            call_ptr(c, 0x0019_8af0, 0x0018_45bc)?; // getDollarString(cost)
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0018_45c8)?; // cUIText_setAsciiString
        }
        c.cycles += 9;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2638));
        call_ptr(c, 0x0031_7670, 0x0018_45d4)?; // GetHashValue32(str_have_num)
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_45e0)?; // cUIScreen_getObjectByHashName
        c.g[S2][0] = c.g[V0][0];
        if c.g[S2][0] != 0 {
            c.cycles += 24;
            call_ptr(c, 0x0014_dc80, 0x0018_45f4)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_dd58, 0x0018_4600)?; // cBE_getInterface_Fv(be, 1)
            c.g[A1][0] = oc640_field(c.g[S1][0], 0x44).lb(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_7488, 0x0018_460c)?; // getPlayerCharID(iface, user)
            // The character id is a signed byte.
            c.g[S0][0] = sx((((c.g[V0][0] as u32) << 24) as i32 >> 24) as u32);
            call_ptr(c, 0x0014_dc80, 0x0018_4618)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xb;
            call_ptr(c, 0x0014_dd58, 0x0018_4624)?; // cBE_getInterface_Fv(be, 11)
            c.g[A1][0] = oc640_field(c.g[S1][0], 0x44).lb(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0015_0928, 0x0018_4634)?; // cBEEconInterface_getMoney
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_8af0, 0x0018_463c)?; // getDollarString(money)
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0018_4648)?; // cUIText_setAsciiString
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cLunoTable::emptyTable(ref)` (0x224f68): destroys the live entries of the
/// table in reverse order (each through its vtable's slot 1, with the entry
/// adjusted by the vtable's offset half-word), frees their block with the
/// count header in front of it, then allocates `Modulus + 1` nodes (a count
/// header and the array), zeroes them, gives each the table's vtable and stores
/// the array as the table's `Entries`. A `Modulus` of -1 keeps the old array.
pub fn luno_table_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = oc640_field(c.g[V0][0], 0x10).lw(c);
        c.g[S2][0] = 0x0048_0000;
        if c.g[V1][0] != 0 {
            // Destroy the live entries, last first.
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if c.g[V1][0] != c.g[S0][0] {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                loop {
                    c.cycles += 10;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = oc640_field(c.g[S0][0], 0x1c).lw(c);
                    c.g[A0][0] = oc640_field(c.g[V0][0], 0x8).lh(c);
                    c.g[V1][0] = oc640_field(c.g[V0][0], 0xc).lw(c);
                    let dtor = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, dtor, 0x0022_4fc0)?;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V1][0] = oc640_field(c.g[V0][0], 0x10).lw(c);
                    let more = c.g[V1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                    if !more {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4fa8);
                    }
                }
            } else {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            }
            c.cycles += 4;
            c.g[A0][0] = oc640_field(c.g[V0][0], 0x10).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0022_4fe0)?; // cMemMan_free(block)
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        }
        // Allocate and initialise the new node array.
        c.cycles += 14;
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6790));
        c.g[A2][0] = 0xffff_ffff_a000_0000;
        c.g[A3][0] = 0;
        c.g[S0][0] = oc640_field(c.g[V0][0], 0xc).lw(c);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 5);
        c.g[A0][0] = c.g[A0][0] | 0x10;
        call_ptr(c, 0x0031_7e30, 0x0022_5008)?; // operator new[] (count + 1 nodes)
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        let count = c.g[S1][0];
        Addr(c.g[V0][0] as u32).sw(c, count);
        c.g[V0][0] = u64::MAX;
        let keep_old = c.g[S0][0] == c.g[V0][0];
        c.g[V1][0] = c.g[A0][0];
        if !keep_old {
            c.cycles += 3;
            c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x6770)); // the node vtable
            c.g[A1][0] = u64::MAX;
            loop {
                c.cycles += 9;
                let vtable = c.g[V0][0];
                oc640_field(c.g[V1][0], 0x1c).sw(c, vtable);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                oc640_field(c.g[V1][0], 0x8).sw(c, 0);
                oc640_field(c.g[V1][0], 0x0).sw(c, 0);
                oc640_field(c.g[V1][0], 0x14).sw(c, 0);
                oc640_field(c.g[V1][0], 0xc).sw(c, 0);
                oc640_field(c.g[V1][0], 0x18).sw(c, 0);
                let more = c.g[S0][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_5028);
                }
            }
        }
        c.cycles += 7;
        let entries_slot = c.g[S2][0] as u32;
        let entries = c.g[A0][0];
        Addr(entries_slot).sw(c, entries);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the fakes below: the magic word, then the flags, then the
    /// call counter.
    pub(crate) const OC640_CTRL: u32 = DATA + 0xf_f000;
    pub(crate) const OC640_MAGIC: u32 = 0x6f63_3634;
    /// The table's `ref`, the table, its entries (with the count in front)
    /// and their vtable.
    pub(crate) const OC640_REF: u32 = DATA + 0x1000;
    pub(crate) const OC640_TABLE: u32 = DATA + 0x2000;
    pub(crate) const OC640_ENTRIES: u32 = DATA + 0x3010;
    pub(crate) const OC640_VT: u32 = DATA + 0x4000;
    /// Where the gated allocator returns the new array (DATA+0x200000 and up
    /// is free of the fixtures).
    pub(crate) const OC640_ALLOC: u32 = DATA + 0x20_0000;

    pub(crate) static OC640_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC640_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// The gated `operator new[]` (0x317e30): logs the call and returns the
    /// fixture array when the gate is set.
    pub(crate) fn oc640_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC640_CTRL) != OC640_MAGIC {
            return fakes::fall_back(c, &OC640_PREV_ALLOC, ee::s10::f_00317e30__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e30, ra);
        if c.pc == ra {
            c.g[V0][0] = OC640_ALLOC as u64;
        }
    }

    /// The gated `cMemMan_free` (0x317e98): logs the call.
    pub(crate) fn oc640_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC640_CTRL) != OC640_MAGIC {
            return fakes::fall_back(c, &OC640_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
    }

    pub(crate) fn oc640_install_luno_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc640_fake_alloc,
            &OC640_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc640_fake_free,
            &OC640_PREV_FREE,
        );
    }

    /// `cLunoTable::emptyTable` against the generated body: a `Modulus` of -1
    /// (the old array is kept), a null `Entries` (nothing to destroy), zero
    /// to a few live entries (the vtable calls are fakes at a miss address),
    /// and the allocator and free fakes; the allocation is the fixture array.
    #[test]
    fn luno_table_clear_matches() {
        oc640_install_luno_fakes();
        fakes::covered(
            ee::s05::f_00224f68__generated,
            luno_table_clear,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x224f68, 6);
                c.bus.write32(OC640_CTRL, OC640_MAGIC);
                c.bus.write32(OC640_CTRL + 4, 0);
                let live = if seed % 5 == 0 { 0 } else { r.next_u32() % 4 };
                let modulus = if seed % 7 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 4
                };
                c.bus.write32(OC640_REF, OC640_TABLE);
                c.bus.write32(OC640_TABLE + 0xc, modulus);
                c.bus.write32(OC640_TABLE + 0x10, OC640_ENTRIES);
                if live == 0 && seed % 3 == 0 {
                    c.bus.write32(OC640_TABLE + 0x10, 0);
                }
                c.bus.write32(OC640_ENTRIES - 0x10, live);
                c.bus.write16(OC640_VT + 8, 0);
                c.bus.write32(OC640_VT + 0xc, fakes::FAKE + 0x20);
                for i in 0..live {
                    c.bus.write32(OC640_ENTRIES + 0x20 * i + 0x1c, OC640_VT);
                }
                c.g[A0][0] = sx(OC640_REF);
                c.pc = 0x0022_4f68;
            },
        );
    }
}
