//! `src/bx/bigfile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_6898, big_file_ctor, "BigFile_ctor"),
    (0x0031_6a00, block_man_ctor, "BlockMan_ctor"),
    (0x0031_6a20, block_man_align_size, "BlockMan_AlignSize"),
];

/// `BigFile *BigFile_ctor(BigFile *this)` (0x316898): mark empty with size -1.
pub fn big_file_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `BlockMan *BlockMan_ctor(BlockMan *this)` (0x316a00): clear the free lists
/// and set the block size to 16.
pub fn block_man_ctor(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x10;
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).off(0x8).sw(c, 0);
    Addr(this).off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `int BlockMan_AlignSize(BlockMan *this, int size)` (0x316a20): round `size`
/// up to the allocator's block granularity at +0x10.
pub fn block_man_align_size(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x10).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
    c.div(0, A1, V0);
    c.g[A1][0] = c.lo[0];
    c.mult(0, V0, V0, A1);
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn big_file_ctor_matches() {
        check(ssx3_ee::s10::f_00316898, big_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn block_man_ctor_matches() {
        check(ssx3_ee::s10::f_00316a00, block_man_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn block_man_align_size_matches() {
        check_lohi(
            ssx3_ee::s10::f_00316a20,
            block_man_align_size,
            300,
            &|c, seed| {
                let mut r = Rng(0x2013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(
                    DATA + 0x10,
                    match seed % 6 {
                        0 => 0,
                        1 => 8,
                        2 => 16,
                        3 => 0x1000,
                        4 => 0x8000_0000,
                        _ => r.next_u32() & 0xffff,
                    },
                );
            },
        );
    }
}
