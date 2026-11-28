//! Staging module oc1581 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1581`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0037_2144, dma_label_tail_0372144, "lbl_00372144"),
    (0x0037_2154, dma_label_tail_0372154, "lbl_00372154"),
    (0x0037_2178, dma_label_tail_0372178, "lbl_00372178"),
    (0x0037_2188, dma_label_tail_0372188, "lbl_00372188"),
    (0x0038_f560, sprstream_end_current_block, "SprStream_EndCurrentBlock"),
    (0x0039_2edc, dispatch_tail_0392edc, "lbl_00392EDC"),
    (0x0039_2ee4, dispatch_tail_0392ee4, "lbl_00392EE4"),
    (0x0039_2f08, tail_0392f08, "lbl_00392F08"),
];

/// Shared entry step of the labels 0x372144 and 0x372178: `cycles += 4`,
/// `v0 = a2 << 4` (sign-extended), `a1 = a1 + 0x10 + v0`.
fn dma_label_entry(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
}

/// Label 0x372144 in the DMA chain code: the entry step, then a jump to the
/// common tail at 0x372188 (the generated test on `t` is always true, so the
/// branch to 0x372154 is dead).
pub fn dma_label_tail_0372144(c: &mut Cpu) {
    dma_label_entry(c);
    c.pc = 0x0037_2188;
    ee::s12::f_00372188(c);
}

/// Label 0x372154: packs `t0`'s low word under `(t2 + a3) << 32` and stores
/// the doubleword at `a1`, then `a1 += 0x10` and a jump to 0x372188.
pub fn dma_label_tail_0372154(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = (!0xffff_u64) >> 32;
    c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    let store = c.g[A1][0] as u32;
    let packed = c.g[V1][0];
    c.sd(store, packed);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
    c.pc = 0x0037_2188;
    ee::s12::f_00372188(c);
}

/// Label 0x372178: the entry step with `t1 = 1` (the flag the common tail
/// reads), then a jump to 0x372188.
pub fn dma_label_tail_0372178(c: &mut Cpu) {
    dma_label_entry(c);
    c.g[T1][0] = 0x1;
    c.pc = 0x0037_2188;
    ee::s12::f_00372188(c);
}

/// Common tail of the DMA labels (0x372188). With `t1 == 0` it loads the
/// doubleword at `a1` into `v0` and jumps to 0x372100 (through the function
/// table, a jump that keeps `ra`); otherwise `v0 = a1` and it returns.
pub fn dma_label_tail_0372188(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[T1][0] == 0 {
        c.g[V0][0] = Addr(c.g[A1][0] as u32).ld(c);
        c.pc = 0x0037_2100;
        call_indirect(c, 0x0037_2100);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

/// `SprStream_EndCurrentBlock(stream, flags)` (0x38f560): calls
/// `SprStream_EndBlock(stream, unk_2C[mDMABlock] << 4, flags)` and returns
/// what it returns. The block index lives at +0xc, the table of block
/// offsets at +0x2c (words), and `ra` is saved in the 0x10-byte frame.
pub fn sprstream_end_current_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
        Callee {
            addr: 0x0038_f598,
            f: ee::s12::f_0038f598,
        }
        .call(c, 0x0038_f588)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Label 0x392edc: `v0 = 0xc`, then a jump to the resume point 0x392ef0 of
/// the dispatcher at 0x392eec (a tail call, `ra` unchanged).
pub fn dispatch_tail_0392edc(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xc;
    c.pc = 0x0039_2ef0;
    call_indirect(c, 0x0039_2ef0);
}

/// Label 0x392ee4: `v0 = 0x6`, then the same resume jump to 0x392ef0.
pub fn dispatch_tail_0392ee4(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x6;
    c.pc = 0x0039_2ef0;
    call_indirect(c, 0x0039_2ef0);
}

/// Label 0x392f08: rounds `a3` up to a multiple of 16 (`(a3 + 0xf) >> 4`,
/// with the signed compare of the generated code choosing the source), then
/// a tail call to 0x392f5c.
pub fn tail_0392f08(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xf));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[A3][0];
    }
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
    c.pc = 0x0039_2f5c;
    ee::s13::f_00392f5c(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The return address the fixtures put in `ra`.
    const RET: u32 = crate::difftest::RA;

    // Gate words: each gated fake answers only when its own word holds its
    // magic in the machine's RAM; with the gate off it runs the body it
    // replaced.
    const OC1581_GATE_DMA: u32 = DATA + 0x0042_0000;
    const OC1581_MAGIC_DMA: u32 = 0x3158_1001;
    const OC1581_GATE_END_BLOCK: u32 = DATA + 0x0042_0008;
    const OC1581_MAGIC_END_BLOCK: u32 = 0x3158_1002;
    const OC1581_GATE_F392F5C: u32 = DATA + 0x0042_0010;
    const OC1581_MAGIC_F392F5C: u32 = 0x3158_1003;

    static OC1581_PREV_DMA_RUNNER: AtomicUsize = AtomicUsize::new(0);
    static OC1581_PREV_END_BLOCK: AtomicUsize = AtomicUsize::new(0);
    static OC1581_PREV_F392F5C: AtomicUsize = AtomicUsize::new(0);

    /// The callee of the DMA tail at 0x372100 when the table holds its owner
    /// (0x371f98, a body with no resume entries): a logged call.
    fn oc1581_fake_dma_runner(c: &mut Cpu) {
        if c.bus.read32(OC1581_GATE_DMA) != OC1581_MAGIC_DMA {
            return fakes::fall_back(c, &OC1581_PREV_DMA_RUNNER, ee::s12::f_00371f98__generated);
        }
        fakes::fake_call(c, 0x0037_2100);
    }

    /// `SprStream_EndBlock` (0x38f598) as a logged call.
    fn oc1581_fake_end_block(c: &mut Cpu) {
        if c.bus.read32(OC1581_GATE_END_BLOCK) != OC1581_MAGIC_END_BLOCK {
            return fakes::fall_back(c, &OC1581_PREV_END_BLOCK, ee::s12::f_0038f598__generated);
        }
        fakes::fake_call(c, 0x0038_f598);
    }

    /// The dispatcher's tail callee 0x392f5c as a logged call.
    fn oc1581_fake_f392f5c(c: &mut Cpu) {
        if c.bus.read32(OC1581_GATE_F392F5C) != OC1581_MAGIC_F392F5C {
            return fakes::fall_back(c, &OC1581_PREV_F392F5C, ee::s13::f_00392f5c__generated);
        }
        fakes::fake_call(c, 0x0039_2f5c);
    }

    fn hook_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00371F98),
            oc1581_fake_dma_runner,
            &OC1581_PREV_DMA_RUNNER,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F598),
            oc1581_fake_end_block,
            &OC1581_PREV_END_BLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00392F5C),
            oc1581_fake_f392f5c,
            &OC1581_PREV_F392F5C,
        );
    }

    /// The DMA label family: `a1` at a 16-aligned word of the data area,
    /// `a2` a small count, `t1` picks the branch of the common tail, `ra` a
    /// known return address.
    fn dma_label_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = fakes::machine(c, seed, salt, 1);
        c.bus.write32(OC1581_GATE_DMA, OC1581_MAGIC_DMA);
        c.g[RA][0] = sx(RET);
        c.g[A1][0] = sx(DATA + 0x1000);
        c.g[A2][0] = (r.next_u32() % 4) as u64;
        c.g[T0][0] = r.next_u32() as u64;
        c.g[T1][0] = (seed % 2) as u64;
        c.g[T2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
    }

    #[test]
    fn dma_label_tail_0372144_matches() {
        hook_fakes();
        check(
            ee::s12::f_00372144__generated,
            dma_label_tail_0372144,
            120,
            &|c, seed| dma_label_setup(c, seed, 0x0037_2144),
        );
    }

    #[test]
    fn dma_label_tail_0372154_matches() {
        hook_fakes();
        check(
            ee::s12::f_00372154__generated,
            dma_label_tail_0372154,
            120,
            &|c, seed| dma_label_setup(c, seed, 0x0037_2154),
        );
    }

    #[test]
    fn dma_label_tail_0372178_matches() {
        hook_fakes();
        check(
            ee::s12::f_00372178__generated,
            dma_label_tail_0372178,
            120,
            &|c, seed| dma_label_setup(c, seed, 0x0037_2178),
        );
    }

    #[test]
    fn dma_label_tail_0372188_matches() {
        hook_fakes();
        check(
            ee::s12::f_00372188__generated,
            dma_label_tail_0372188,
            120,
            &|c, seed| dma_label_setup(c, seed, 0x0037_2188),
        );
    }

    #[test]
    fn sprstream_end_current_block_matches() {
        hook_fakes();
        check(
            ee::s12::f_0038f560__generated,
            sprstream_end_current_block,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0038_f560, 1);
                c.bus.write32(OC1581_GATE_END_BLOCK, OC1581_MAGIC_END_BLOCK);
                c.g[RA][0] = sx(RET);
                let stream = DATA + 0x1000;
                let block = seed % 4;
                c.g[A0][0] = sx(stream);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(stream + 0xc, block);
                c.bus.write32(stream + 0x2c + 4 * block, r.next_u32() & 0xff);
                c.bus
                    .write32(stream + 0x2c + 4 * ((block + 1) % 4), r.next_u32());
            },
        );
    }

    /// Dispatcher tail fixture: `a3` random, the callee 0x392f5c a logged
    /// call, `ra` known.
    fn dispatch_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = fakes::machine(c, seed, salt, 1);
        c.bus.write32(OC1581_GATE_F392F5C, OC1581_MAGIC_F392F5C);
        c.g[RA][0] = sx(RET);
        c.g[A3][0] = r.next_u32() as u64;
    }

    #[test]
    fn dispatch_tail_0392edc_matches() {
        hook_fakes();
        check(
            ee::s13::f_00392edc__generated,
            dispatch_tail_0392edc,
            120,
            &|c, seed| dispatch_setup(c, seed, 0x0039_2edc),
        );
    }

    #[test]
    fn dispatch_tail_0392ee4_matches() {
        hook_fakes();
        check(
            ee::s13::f_00392ee4__generated,
            dispatch_tail_0392ee4,
            120,
            &|c, seed| dispatch_setup(c, seed, 0x0039_2ee4),
        );
    }

    #[test]
    fn tail_0392f08_matches() {
        hook_fakes();
        check(
            ee::s13::f_00392f08__generated,
            tail_0392f08,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0039_2f08, 1);
                c.bus.write32(OC1581_GATE_F392F5C, OC1581_MAGIC_F392F5C);
                c.g[RA][0] = sx(RET);
                c.g[A3][0] = sx(r.next_u32());
            },
        );
    }
}
