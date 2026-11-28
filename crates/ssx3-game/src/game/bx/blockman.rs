//! `src/bx/blockman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0031_6a48, block_man_init, "BlockMan_Init")];

/// `BlockMan_Init` (0x316a48): sets up a block manager from a list of
/// `(size word, size)` entries (`a2`, `a1` entries): aligns each size, sums
/// them into the manager's total at `+0x4`, allocates the storage, and fills
/// the per-block headers.
pub fn block_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S3, 0x40);
        frame.save(c, S5, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S6, 0x10);
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S1, 0x60);
        c.g[A1][0] = sx((c.g[S5][0] as u32) << 4);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        let a3 = c.g[A3][0];
        Addr(c.g[S3][0] as u32).off(0x10).sw(c, a3);
        call_ptr(c, 0x0031_6a20, 0x0031_6a84)?;
        c.g[S4][0] = c.g[V0][0];
        let t = (c.g[S5][0] as i64) <= 0;
        let s4 = c.g[S4][0];
        Addr(c.g[S3][0] as u32).off(0x4).sw(c, s4);
        'init: {
            if t {
                break 'init;
            }
            c.cycles += 2;
            c.g[S0][0] = c.g[S6][0];
            c.g[S1][0] = c.g[S5][0];
            loop {
                c.cycles += 11;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                call_ptr(c, 0x0031_6a20, 0x0031_6aa8)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[S3][0] as u32).off(0x4).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                c.mult(0, 3, 3, 2);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let t = c.g[S1][0] != 0;
                let total = c.g[A0][0];
                Addr(c.g[S3][0] as u32).off(0x4).sw(c, total);
                if !t {
                    break 'init;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_6a98);
                }
            }
        }
        'bb4: {
            c.cycles += 12;
            c.g[A0][0] = Addr(c.g[S3][0] as u32).off(0x4).lw(c);
            c.g[A1][0] = 0x490000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x27c0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[S2][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0031_6ae0)?;
            let s5 = c.g[S5][0];
            Addr(c.g[S3][0] as u32).off(0xc).sw(c, s5);
            c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            let v0 = c.g[V0][0];
            Addr(c.g[S3][0] as u32).sw(c, v0);
            let t = (c.g[S5][0] as i64) <= 0;
            Addr(c.g[S3][0] as u32).off(0x8).sw(c, v0);
            if t {
                break 'bb4;
            }
            c.cycles += 1;
            loop {
                'slot: {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
                    call_ptr(c, 0x0031_6a20, 0x0031_6b0c)?;
                    c.g[V1][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 4);
                    c.g[T0][0] = c.g[V0][0];
                    c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t0 = c.g[T0][0];
                    Addr(c.g[V1][0] as u32).off(0x8).sw(c, t0);
                    c.mult(0, 9, 2, 8);
                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let s4 = c.g[S4][0];
                    Addr(c.g[V0][0] as u32).off(0xc).sw(c, s4);
                    c.g[V1][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    Addr(c.g[V1][0] as u32).sw(c, s4);
                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x8).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t1 = c.g[T1][0];
                    Addr(c.g[A0][0] as u32).off(0x4).sw(c, t1);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A2][0] = 0;
                    if t {
                        break 'slot;
                    }
                    c.cycles += 4;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[T0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    loop {
                        c.cycles += 7;
                        let fill = c.g[V1][0];
                        Addr(c.g[A0][0] as u32).sw(c, fill);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if !t {
                            break 'slot;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_6b70);
                        }
                    }
                }
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[S2][0] = c.g[T2][0];
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.mult(0, 2, 8, 3);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[T1][0] as u32));
                let t = c.g[A0][0] != 0;
                Addr(c.g[V1][0] as u32).sw(c, 0);
                if !t {
                    break 'bb4;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_6af8);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use ssx3_ee as ee;

    use crate::difftest::fakes::machine_rt;

    use crate::difftest::DATA;
    pub(crate) use crate::game::wscript::wscriptman::tests::OC1577_SEEDS;

    pub(crate) const OC1577_BM: u32 = DATA + 0x5000;

    pub(crate) const OC1577_LIST: u32 = DATA + 0x5100;

    #[test]
    fn block_man_init_matches() {
        check(
            ee::s10::f_00316a48__generated,
            block_man_init,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                let mut r = machine_rt(c, seed, 0x1577_0005, 0);
                c.g[A0][0] = sx(OC1577_BM);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(OC1577_LIST);
                c.g[A3][0] = sx(r.next_u32());
                for i in 0..4u32 {
                    c.bus.write32(OC1577_LIST + 8 * i, r.next_u32() % 16);
                    c.bus
                        .write32(OC1577_LIST + 8 * i + 4, 1 + r.next_u32() % 200);
                }
            },
        );
    }
}
