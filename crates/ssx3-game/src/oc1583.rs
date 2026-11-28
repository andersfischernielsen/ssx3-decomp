//! Staging module oc1583 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1583`.

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
    (0x003a_39b4, fpu_scale_store_24, "lbl_003A39B4"),
    (0x003a_39c4, fpu_scale_store_28, "lbl_003A39C4"),
    (0x003b_0680, rcmp_decoder_reset, "RCMP_DECODER_Reset"),
    (0x003b_06f8, rcmp_decoder_release_chunk, "RCMP_DECODER_ReleaseChunk"),
    (0x003b_07b8, rcmp_global_ctors, "RCMP_GlobalCtors"),
    (0x003b_07d8, rcmp_global_dtors, "RCMP_GlobalDtors"),
    (
        0x003b_16e0,
        mpeg2_group_of_pictures_header,
        "MPEG2_group_of_pictures_header",
    ),
    (0x003b_5a60, snd_remove_event_callback, "SND_RemoveEventCallback"),
];

/// `lbl_003A39B4` (0x3a39b4): `*(float*)(t0 + 0x24) = f1 * *(float*)(gp - 0x24e0)`,
/// then a tail jump into 0x3a39e0.
pub fn fpu_scale_store_24(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24e0));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x24));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

/// `lbl_003A39C4` (0x3a39c4): the same as [`fpu_scale_store_24`] with the
/// global at gp - 0x24dc and the store at t0 + 0x28.
pub fn fpu_scale_store_28(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24dc));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x28));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

const MPEG2_GET_BITS: Callee = Callee {
    addr: 0x003b_1300,
    f: ee::s13::f_003b1300,
};
const MPEG2_MARKER_BIT: Callee = Callee {
    addr: 0x003b_2360,
    f: ee::s13::f_003b2360,
};
const MPEG2_EXTENSION_AND_USER_DATA: Callee = Callee {
    addr: 0x003b_1988,
    f: ee::s13::f_003b1988,
};
const RCMP_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x003b_0770,
    f: ee::s13::f_003b0770,
};
const SND_LOCK: Callee = Callee {
    addr: 0x003b_58a0,
    f: ee::s13::f_003b58a0,
};
const SND_UNLOCK: Callee = Callee {
    addr: 0x003b_58d8,
    f: ee::s13::f_003b58d8,
};

/// `RCMP_DECODER_Reset` (0x3b0680): calls the method in slot `+0x34` of the
/// object at `self + 0x14` (its `this` is `obj + vt[+0x30]`, a 16-bit
/// offset), passing `a1` through.
pub fn rcmp_decoder_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x003b_06a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_DECODER_ReleaseChunk(self, blk)` (0x3b06f8): calls the function at
/// `self + 0xc` with `a0 = self`, `a1 = *(self + 4)`, `a2 = blk`.
pub fn rcmp_decoder_release_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        call_ptr(c, t, 0x003b_0714)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_GlobalCtors` (0x3b07b8): `RCMP_StaticInitDestroy(1, 0xffff)`.
pub fn rcmp_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        RCMP_STATIC_INIT_DESTROY.call(c, 0x003b_07cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_GlobalDtors` (0x3b07d8): `RCMP_StaticInitDestroy(0, 0xffff)`.
pub fn rcmp_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        RCMP_STATIC_INIT_DESTROY.call(c, 0x003b_07ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MPEG2_group_of_pictures_header` (0x3b16e0): reads the GOP fields
/// through `MPEG2_Get_Bits` and `MPEG2_marker_bit`, then
/// `MPEG2_extension_and_user_data`. A first-GOP check (the word at
/// 0x510000 - 0x5f78 against 0x509648) also bumps the counter pair at
/// 0x450000 - 0x3c80 / 0x3c88 before the reads.
pub fn mpeg2_group_of_pictures_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x510000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5f78));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x69b8));
        let t = c.g[A0][0] != c.g[V1][0];
        frame.save_ra(c, 0);
        if !t {
            c.cycles += 8;
            c.g[V1][0] = 0x450000;
            c.g[A1][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x3c84));
            c.g[A0][0] = 0x450000;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[A0][0] as u32).wrapping_sub(0x3c80), c.g[V1][0]);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x3c88), c.g[V0][0]);
        }
        c.cycles += 35;
        c.g[A0][0] = 0x1;
        MPEG2_GET_BITS.call(c, 0x003b_1724)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x5;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d8), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1734)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x6;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d4), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1744)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x490000;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69d0), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5468));
        MPEG2_MARKER_BIT.call(c, 0x003b_1758)?;
        c.g[A0][0] = 0x6;
        MPEG2_GET_BITS.call(c, 0x003b_1760)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x6;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69cc), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1770)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c8), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1780)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c4), c.g[V0][0]);
        MPEG2_GET_BITS.call(c, 0x003b_1790)?;
        c.g[V1][0] = 0x510000;
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x69c0), c.g[V0][0]);
        MPEG2_EXTENSION_AND_USER_DATA.call(c, 0x003b_179c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_RemoveEventCallback(cb)` (0x3b5a60): under `SND_Lock`, finds `cb` in the
/// callback array of the sound system (count byte at +0x179, entries from
/// +0x1c8, one word each), removes it by shifting the rest down and decrements
/// the count; then `SND_Unlock`. The scan loop makes the structured event
/// check at its back edges, as the generated code does.
pub fn snd_remove_event_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        SND_LOCK.call(c, 0x003b_5a74)?;
        'exit: {
            c.g[A1][0] = 0x510000;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5718));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x179));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'exit;
            }
            c.cycles += 4;
            c.g[A2][0] = c.g[V1][0];
            c.g[T0][0] = c.g[A1][0];
            c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1c8));
            c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
            'scan: loop {
                'next: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    if c.g[V1][0] != c.g[S0][0] {
                        c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x179));
                        break 'next;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x179));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.sb((c.g[A2][0] as u32).wrapping_add(0x179), c.g[V0][0]);
                    if t {
                        break 'exit;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x5718));
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c8));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    'shift: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x179));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003b_5ad0);
                            }
                            continue 'shift;
                        }
                        c.cycles += 2;
                        break 'exit;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = sx((c.g[A0][0] as u32) << 2);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_5a98);
                    }
                    continue 'scan;
                }
                break 'exit;
            }
        }
        c.cycles += 6;
        SND_UNLOCK.call(c, 0x003b_5b0c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words of the fakes below (one per fake, in each machine's own RAM).
    const OC1583_MAGIC: u32 = 0x1583_0001;
    const OC1583_STATIC_CTRL: u32 = DATA + 0x20_0000;
    const OC1583_BITS_CTRL: u32 = DATA + 0x20_1000;
    const OC1583_MARKER_CTRL: u32 = DATA + 0x20_2000;
    const OC1583_EXT_CTRL: u32 = DATA + 0x20_3000;
    const OC1583_LOCK_CTRL: u32 = DATA + 0x20_4000;
    const OC1583_UNLOCK_CTRL: u32 = DATA + 0x20_5000;

    /// The sound system's block (0x510000 - 0x5718): the count byte at +0x179,
    /// the callback words from +0x1c8.
    const OC1583_SND_BLOCK: u32 = 0x0050_a8e8;
    /// The GOP first-picture cell (0x510000 - 0x5f78).
    const OC1583_GOP_CELL: u32 = 0x0050_a088;

    static OC1583_PREV_STATIC: AtomicUsize = AtomicUsize::new(0);
    static OC1583_PREV_BITS: AtomicUsize = AtomicUsize::new(0);
    static OC1583_PREV_MARKER: AtomicUsize = AtomicUsize::new(0);
    static OC1583_PREV_EXT: AtomicUsize = AtomicUsize::new(0);
    static OC1583_PREV_LOCK: AtomicUsize = AtomicUsize::new(0);
    static OC1583_PREV_UNLOCK: AtomicUsize = AtomicUsize::new(0);

    fn oc1583_fake_static_init(c: &mut Cpu) {
        if c.bus.read32(OC1583_STATIC_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_STATIC, ee::s13::f_003b0770__generated);
        }
        fakes::fake_call(c, 0x003b_0770);
    }

    fn oc1583_fake_get_bits(c: &mut Cpu) {
        if c.bus.read32(OC1583_BITS_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_BITS, ee::s13::f_003b1300__generated);
        }
        fakes::fake_call(c, 0x003b_1300);
    }

    fn oc1583_fake_marker(c: &mut Cpu) {
        if c.bus.read32(OC1583_MARKER_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_MARKER, ee::s13::f_003b2360__generated);
        }
        fakes::fake_call(c, 0x003b_2360);
    }

    fn oc1583_fake_ext(c: &mut Cpu) {
        if c.bus.read32(OC1583_EXT_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_EXT, ee::s13::f_003b1988__generated);
        }
        fakes::fake_call(c, 0x003b_1988);
    }

    fn oc1583_fake_lock(c: &mut Cpu) {
        if c.bus.read32(OC1583_LOCK_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_LOCK, ee::s13::f_003b58a0__generated);
        }
        fakes::fake_call(c, 0x003b_58a0);
    }

    fn oc1583_fake_unlock(c: &mut Cpu) {
        if c.bus.read32(OC1583_UNLOCK_CTRL) != OC1583_MAGIC {
            return fakes::fall_back(c, &OC1583_PREV_UNLOCK, ee::s13::f_003b58d8__generated);
        }
        fakes::fake_call(c, 0x003b_58d8);
    }

    fn oc1583_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B0770),
            oc1583_fake_static_init,
            &OC1583_PREV_STATIC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1300),
            oc1583_fake_get_bits,
            &OC1583_PREV_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B2360),
            oc1583_fake_marker,
            &OC1583_PREV_MARKER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1988),
            oc1583_fake_ext,
            &OC1583_PREV_EXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc1583_fake_lock,
            &OC1583_PREV_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc1583_fake_unlock,
            &OC1583_PREV_UNLOCK,
        );
    }

    /// The stub: a float product stored through `t0`, then the tail into
    /// 0x3a39e0 (its index test fails at once with these registers).
    #[test]
    fn fpu_scale_store_24_matches() {
        check(
            ee::s13::f_003a39b4__generated,
            fpu_scale_store_24,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_39b4, 0);
                c.bus.write32(GAME_GP - 0x24e0, r.edge_f32());
                c.g[T0][0] = (DATA + 0x100) as u64;
                c.f[1] = if seed % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(100.0)
                };
                c.g[T2][0] = 5;
                c.g[A1][0] = 3;
            },
        );
    }

    #[test]
    fn fpu_scale_store_28_matches() {
        check(
            ee::s13::f_003a39c4__generated,
            fpu_scale_store_28,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_39c4, 0);
                c.bus.write32(GAME_GP - 0x24dc, r.edge_f32());
                c.g[T0][0] = (DATA + 0x100) as u64;
                c.f[1] = if seed % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(100.0)
                };
                c.g[T2][0] = 5;
                c.g[A1][0] = 3;
            },
        );
    }

    /// `RCMP_DECODER_Reset`: the object's method (slot +0x34, `this` offset
    /// from +0x30) is a logged fake; the unwind index covers the return path.
    #[test]
    fn rcmp_decoder_reset_matches() {
        check(
            ee::s13::f_003b0680__generated,
            rcmp_decoder_reset,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_0680, 1);
                let this = fakes::ptr(&mut r);
                let obj = fakes::ptr(&mut r);
                let vt = fakes::vtable(c, &mut r, &[0x30]);
                c.bus.write32(obj, vt);
                c.bus.write32(this + 0x14, obj);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `RCMP_DECODER_ReleaseChunk`: the function at `self + 0xc` is a logged
    /// fake (the miss hook); `a1` comes from `self + 4`.
    #[test]
    fn rcmp_decoder_release_chunk_matches() {
        check(
            ee::s13::f_003b06f8__generated,
            rcmp_decoder_release_chunk,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_06f8, 1);
                let this = fakes::ptr(&mut r);
                c.bus.write32(this + 0xc, fakes::FAKE);
                c.bus.write32(this + 4, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `RCMP_GlobalCtors` / `RCMP_GlobalDtors`: the static-init callee is a
    /// gated logged fake, so its two arguments are compared.
    #[test]
    fn rcmp_global_ctors_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b07b8__generated,
            rcmp_global_ctors,
            200,
            &|c, seed| {
                machine(c, seed, 0x003b_07b8, 1);
                c.bus.write32(OC1583_STATIC_CTRL, OC1583_MAGIC);
            },
        );
    }

    #[test]
    fn rcmp_global_dtors_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b07d8__generated,
            rcmp_global_dtors,
            200,
            &|c, seed| {
                machine(c, seed, 0x003b_07d8, 1);
                c.bus.write32(OC1583_STATIC_CTRL, OC1583_MAGIC);
            },
        );
    }

    /// `MPEG2_group_of_pictures_header`: both arms of the first-GOP check,
    /// and the three callees as logged fakes (one may unwind).
    #[test]
    fn mpeg2_group_of_pictures_header_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b16e0__generated,
            mpeg2_group_of_pictures_header,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_16e0, 9);
                c.bus.write32(OC1583_BITS_CTRL, OC1583_MAGIC);
                c.bus.write32(OC1583_MARKER_CTRL, OC1583_MAGIC);
                c.bus.write32(OC1583_EXT_CTRL, OC1583_MAGIC);
                let cell = if seed % 2 == 0 {
                    0x510000 - 0x69b8
                } else {
                    r.next_u32()
                };
                c.bus.write32(OC1583_GOP_CELL, cell);
            },
        );
    }

    /// `SND_RemoveEventCallback`: an empty list, an absent callback, and one
    /// present first, in the middle or last, with the lock calls logged.
    #[test]
    fn snd_remove_event_callback_matches() {
        oc1583_install();
        check(
            ee::s13::f_003b5a60__generated,
            snd_remove_event_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_5a60, 2);
                c.bus.write32(OC1583_LOCK_CTRL, OC1583_MAGIC);
                c.bus.write32(OC1583_UNLOCK_CTRL, OC1583_MAGIC);
                let count = seed % 6;
                c.bus.write8(OC1583_SND_BLOCK + 0x179, count as u8);
                for i in 0..count {
                    c.bus.write32(OC1583_SND_BLOCK + 0x1c8 + 4 * i, r.next_u32());
                }
                let cb = 0x1234_0000 | seed;
                if count > 0 && seed % 4 != 0 {
                    let k = (seed / 4) % count;
                    c.bus.write32(OC1583_SND_BLOCK + 0x1c8 + 4 * k, cb);
                }
                c.g[A0][0] = cb as u64;
            },
        );
    }
}
