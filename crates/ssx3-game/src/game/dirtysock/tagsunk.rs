//! `src/dirtysock/tagsunk.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Frame;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_9288,
        tagfieldsetstringescaped,
        "TagFieldSetStringEscaped",
    ),
    (
        0x0025_9310,
        tag_field_get_string_unescaped,
        "TagFieldGetStringUnescaped",
    ),
    (0x0026_1058, buddy_man_clear_login, "BuddyMan_ClearLogin"),
    (0x0026_6d00, c_voip_man_ctor, "cVoipMan_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;

    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;

    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY;

    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
}

// ---------------------------------------------------------------------------
// Buddy list.

/// `void BuddyMan_ClearLogin(BuddyMan *self)` (0x261058): clear the login
/// state (a byte at +0, the handle at +0x40 to -1, a byte at +0x44 and the
/// flags byte at +0x64).
pub fn buddy_man_clear_login(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    this.off(0x64).sb(c, 0);
    this.off(0x40).sw(c, c.g[V0][0]);
    this.sb(c, 0);
    this.off(0x44).sb(c, 0);
    abi::ret(c);
}

/// `cVoipMan *cVoipMan_ctor(cVoipMan *this)` (0x266d00): zero the two words of
/// a fresh voip manager and return it unchanged in v0.
pub fn c_voip_man_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `int TagFieldGetStringUnescaped(const char* tag, char* dst, int size)`
/// (0x00259310): parse a tag field into a stack buffer and copy it, dropping
/// a leading backtick.
pub fn tag_field_get_string_unescaped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.g[V0][0] = sx(word(c, A2) << 3);
        c.sq(30, at(c, SP, 0x30));
        c.g[V0][0] = sx(word(c, V0) >> 3);
        c.sq(16, at(c, SP, 0x40));
        c.g[V0][0] = sx(word(c, V0).wrapping_add(0xf));
        c.sd(at(c, SP, 0x20), c.g[RA][0]);
        c.g[V0][0] = sx(word(c, V0) >> 4);
        c.g[FP][0] = c.g[SP][0];
        c.g[V0][0] = sx(word(c, V0) << 4);
        c.g[S0][0] = c.g[A1][0];
        c.g[SP][0] = sx(sp(c).wrapping_sub(word(c, V0)));
        c.g[RA][0] = sx(0x0025_934c);
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_GET_STRING.call(c, 0x0025_934c)?;
        c.g[V1][0] = Addr(sp(c)).lb(c);
        c.g[V0][0] = 0x60;
        let plain = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if plain {
            c.cycles += 2;
            c.g[A1][0] = c.g[SP][0];
            callee::STRCPY.call(c, 0x0025_9374)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = sx(word(c, SP) | 1);
            callee::STRCPY.call(c, 0x0025_9364)?;
        }
        c.cycles += 6;
        c.g[SP][0] = c.g[FP][0];
        frame_restore(c, 16, 0x40);
        frame_restore(c, 30, 0x30);
        c.g[RA][0] = c.ld(at(c, SP, 0x20));
        let ra = word(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `lq reg, off(sp)` (a bare restore, no frame object).
#[inline(always)]
pub(crate) fn frame_restore(c: &mut Cpu, reg: usize, off: u32) {
    let a = at(c, SP, off);
    c.lq(reg, a);
}

/// `void TagFieldSetStringEscaped(self, tag, value, text)` (0x259288): when
/// `text` starts with a single quote, copy the rest through a stack buffer;
/// set the tag field.
pub fn tagfieldsetstringescaped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x140);
        c.g[V0][0] = 0x27;
        frame.save(c, 16, 0x130);
        frame.save(c, 17, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.sd(p(sp(c), 0x100), c.g[RA][0]);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lb(c);
        let quoted = c.g[V1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A2][0];
        if !quoted {
            c.cycles += 12;
            c.g[V0][0] = 0x60;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = c.g[SP][0] | 1;
            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
            call_ptr(c, callee::STR_COPY, 0x0025_92c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_92dc)?;
            frame.restore(c, 16, 0x130);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_92f4)?;
            frame.restore(c, 16, 0x130);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x120);
        frame.restore(c, 18, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::be::beintplayer::tests::r_u32;
    pub(crate) use crate::game::dirtysock::tags::tests::put_str;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;

    #[test]
    fn buddy_man_clear_login_matches() {
        check(
            ssx3_ee::s06::f_00261058,
            buddy_man_clear_login,
            100,
            &|c, seed| {
                let mut r = Rng(0x2610_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for off in [0u32, 0x40, 0x44, 0x64] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_voip_man_ctor_matches() {
        check(ssx3_ee::s07::f_00266d00, c_voip_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn tag_field_get_string_unescaped_matches() {
        check(
            ee::s06::f_00259310__generated,
            tag_field_get_string_unescaped,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x2593_10, 2);
                let tag = DATA + 0x1000;
                let dst = DATA + 0x2000;
                let text: &[u8] = match seed % 4 {
                    0 => b"`abcdef",
                    1 => b"abcdef",
                    2 => b"",
                    _ => b"`",
                };
                put_str(c, tag, text);
                c.g[A0][0] = tag as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = (4 + seed % 13) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }

    #[test]
    fn tagfieldsetstringescaped_matches() {
        check(
            ee::s06::f_00259288__generated,
            tagfieldsetstringescaped,
            SEEDS,
            &|c, seed| {
                base_2(c, seed, 0x59288, 0);
                let s = DATA + 0x100;
                let text = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r_u32(seed, 3) as u64;
                c.g[A3][0] = sx(text);
                c.bus.write8(text, if seed % 2 == 0 { 0x27 } else { b'x' });
                for i in 1..16u32 {
                    c.bus
                        .write8(text + i, b'a' + (seed.wrapping_add(i) % 26) as u8);
                }
                c.bus.write8(text + 16, 0);
                c.bus.write32(0x004a_50d0, 0);
            },
        );
    }
}
