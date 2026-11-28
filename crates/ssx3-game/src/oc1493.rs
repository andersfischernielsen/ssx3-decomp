//! Staging module oc1493 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1493`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_c428, camera_trigger_man_read_trigger, "CameraTriggerMan_ReadTrigger"),
    (0x003e_34e8, eafile_platform_get_size, "EAFile_PlatformGetSize"),
    (0x0032_e398, cworldline_transform, "cWorldLine_Transform"),
    (0x003e_32f8, eafile_platform_close, "EAFile_PlatformClose"),
    (0x003a_0270, lbl_003a0270, "lbl_003A0270"),
    (0x003a_3948, lbl_003a3948, "lbl_003A3948"),
    (0x003a_3958, lbl_003a3958, "lbl_003A3958"),
    (0x002b_af08, wp_data_relocate, "WPData_Relocate"),
];

// ---------------------------------------------------------------------------
// Camera trigger file reading.

/// `int CameraTriggerMan_ReadTrigger(void* buffer, void* trigger, int version)`
/// (0x16c428): reads one camera trigger. Two `get_uint` reads fill its first
/// two words from `*trigger` and `*trigger + 4`; then `ReadTriggerVolume` reads
/// `*trigger + 8` and, when that status is zero, `ReadTriggerAction` reads the
/// actions at `*trigger + 0xc` and `*trigger + 0x10`. Returns the first
/// non-zero status, or the status of the last read (0). Frame 0x40 (s0-s2).
pub fn camera_trigger_man_read_trigger(c: &mut Cpu) {
    const GET_UINT: u32 = 0x002d_2558;
    const READ_VOLUME: Callee = Callee {
        addr: 0x0016_c4c8,
        f: ee::s02::f_0016c4c8,
    };
    const READ_ACTION: Callee = Callee {
        addr: 0x0016_c518,
        f: ee::s02::f_0016c518,
    };
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0]; // the buffer
        c.g[S1][0] = c.g[A1][0]; // the trigger
        c.g[S2][0] = c.g[A2][0]; // the version
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        call_ptr(c, GET_UINT, 0x0016_c450)?;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
        call_ptr(c, GET_UINT, 0x0016_c460)?;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x8));
        READ_VOLUME.call(c, 0x0016_c474)?;
        'done: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 7;
            c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xc));
            READ_ACTION.call(c, 0x0016_c490)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'done;
            }
            c.cycles += 6;
            c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
            READ_ACTION.call(c, 0x0016_c4ac)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// File device (EAFile platform layer).

/// `int EAFile_PlatformGetSize(u32 handle)` (0x3e34e8): the size of the file
/// behind `handle`. The device is the top byte of the handle: device 1 asks
/// the CD entry table (`EAFile_CdGetEntry(handle & 0xffffff, 0, 0, 0)`), any
/// other device seeks with `sceLseek(fd, 0, 2)` (SEEK_END). Returns the
/// callee's value in v0. Frame 0x10.
pub fn eafile_platform_get_size(c: &mut Cpu) {
    const CD_GET_ENTRY: Callee = Callee {
        addr: 0x003e_48d8,
        f: ee::s14::f_003e48d8,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x00ff_ffff;
        c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        let device_one = c.g[A1][0] == c.g[V1][0];
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        if device_one {
            c.cycles += 6;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            CD_GET_ENTRY.call(c, 0x003e_3518)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = 0;
            c.g[A2][0] = 2;
            call_ptr(c, 0x0042_8168, 0x003e_352c)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_PlatformClose(u32 handle)` (0x3e32f8): closes the file behind
/// `handle`. Device 2 closes its fd with `sceClose(handle & 0xffffff)`, and if
/// the slot at `0x450c40` still holds that fd it is set to -1. Other devices
/// do nothing. Returns 0. Frame 0x20 (s0).
pub fn eafile_platform_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x00ff_ffff;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
        c.g[V1][0] = 2;
        let device_two = c.g[A1][0] == c.g[V1][0];
        c.g[S0][0] = c.g[A0][0] & c.g[V0][0];
        'done: {
            if device_two {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0042_7fe8, 0x003e_3324)?;
                c.g[V1][0] = 0x0045_0000;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc40)).lw(c);
                let owned = c.g[V0][0] == c.g[S0][0];
                frame.restore(c, S0, 0x10);
                if !owned {
                    break 'done;
                }
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                let minus_one = c.g[V0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0xc40)).sw(c, minus_one);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 4;
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Vector transform.

/// `cWorldLine_Transform` (0x32e398): a leaf that transforms the two end
/// points of a line object with the VU0 matrix at `a1` (rows at `a1`, `+0x10`,
/// `+0x20`, `+0x30`). The points are read at `a0 + 0x80` and `a0 + 0x90`; the
/// first result is written to `a0 + 0x60`, the second to `a0 + 0x70`, and
/// word `a0 + 4` becomes 1. Both halves scale their result by the float
/// argument in f12 through the stack scratch at `sp + 0..0x1f`. Frame 0x40.
pub fn cworldline_transform(c: &mut Cpu) {
    c.cycles += 71;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    let frame = Frame::open(c, 0x40);
    let st = sp(c);
    c.swc1(12, st.wrapping_add(0x8));
    c.g[V1][0] = 1;
    c.swc1(0, st.wrapping_add(0xc));
    c.swc1(12, st);
    c.swc1(12, st.wrapping_add(0x4));
    // First end point (a0 + 0x80) to a0 + 0x60.
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, st.wrapping_add(0x30));
    c.lq(V0, st.wrapping_add(0x30));
    c.lwc1(0, st);
    c.sq(V0, st.wrapping_add(0x20));
    c.lwc1(5, st.wrapping_add(0x4));
    c.lwc1(1, st.wrapping_add(0x20));
    c.lwc1(3, st.wrapping_add(0x24));
    c.mul_s(1, 1, 0);
    c.lwc1(4, st.wrapping_add(0x28));
    c.mul_s(3, 3, 5);
    c.lwc1(2, st.wrapping_add(0x2c));
    c.lwc1(0, st.wrapping_add(0x8));
    c.lwc1(5, st.wrapping_add(0xc));
    c.mul_s(4, 4, 0);
    c.swc1(1, st.wrapping_add(0x10));
    c.mul_s(2, 2, 5);
    c.swc1(3, st.wrapping_add(0x14));
    c.swc1(4, st.wrapping_add(0x18));
    c.swc1(2, st.wrapping_add(0x1c));
    c.lq(V0, st.wrapping_add(0x10));
    c.sq(V0, (c.g[A0][0] as u32).wrapping_add(0x60));
    // Second end point (a0 + 0x90) to a0 + 0x70.
    c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x90));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, st.wrapping_add(0x30));
    c.lq(V0, st.wrapping_add(0x30));
    c.lwc1(0, st);
    c.sq(V0, st.wrapping_add(0x20));
    c.lwc1(4, st.wrapping_add(0x4));
    c.lwc1(1, st.wrapping_add(0x20));
    c.lwc1(2, st.wrapping_add(0x24));
    c.lwc1(3, st.wrapping_add(0x28));
    c.mul_s(1, 1, 0);
    c.lwc1(5, st.wrapping_add(0x8));
    c.mul_s(2, 2, 4);
    c.lwc1(0, st.wrapping_add(0x2c));
    c.lwc1(4, st.wrapping_add(0xc));
    c.mul_s(3, 3, 5);
    c.swc1(1, st.wrapping_add(0x10));
    c.mul_s(0, 0, 4);
    c.swc1(2, st.wrapping_add(0x14));
    c.swc1(3, st.wrapping_add(0x18));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.swc1(0, st.wrapping_add(0x1c));
    c.lq(V0, st.wrapping_add(0x10));
    c.sq(V0, (c.g[A0][0] as u32).wrapping_add(0x70));
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Tail fragments: stores that fall into or jump to another function.

/// `lbl_003A0270` (0x3a0270): stores f1 at `a3 + 0x64`, then falls into the
/// function at 0x3a0274.
pub fn lbl_003a0270(c: &mut Cpu) {
    c.cycles += 1;
    c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x64));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `lbl_003A3948` (0x3a3948): `f0 = f1 * gp[-0x24f0]`, stored at `t0 + 0x50`,
/// then a tail call to 0x3a39e0. (The generated else arm is dead: its flag is
/// the constant true.)
pub fn lbl_003a3948(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24f0));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x50));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

/// `lbl_003A3958` (0x3a3958): as `lbl_003A3948` with `gp[-0x24ec]` and the
/// store at `t0 + 0x54`.
pub fn lbl_003a3958(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24ec));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x54));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

// ---------------------------------------------------------------------------
// World data relocation.

/// `void WPData_Relocate(WPData* d)` (0x2baf08): turns the offsets in the
/// painter data into pointers. `d->+0` becomes `d + d->+0` and is passed to
/// `QuadTree_InitFreeList`; `d->+8` becomes `d + d->+8`, the base of an array
/// of `d->+4` records (8 bytes each, the word at `+4`): a record word of -1
/// becomes 0, any other becomes `d + word`. Frame 0x20 (s0).
pub fn wp_data_relocate(c: &mut Cpu) {
    const QUAD_TREE_INIT_FREE_LIST: Callee = Callee {
        addr: 0x002c_17c0,
        f: ee::s08::f_002c17c0,
    };
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        let relocated = c.g[V0][0];
        Addr(this).sw(c, relocated);
        QUAD_TREE_INIT_FREE_LIST.call(c, 0x002b_af2c)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr(this.wrapping_add(0x4)).lw(c);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        let empty = (c.g[V1][0] as i64) <= 0;
        let base = c.g[V0][0];
        Addr(this.wrapping_add(0x8)).sw(c, base);
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = u64::MAX;
            loop {
                let record = r32(c, A0);
                c.cycles += 3;
                c.g[V0][0] = Addr(record.wrapping_add(0x4)).lw(c);
                let unset = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
                if unset {
                    c.cycles += 1;
                    Addr(record.wrapping_add(0x4)).sw(c, 0);
                } else {
                    c.cycles += 2;
                    let value = c.g[V0][0];
                    Addr(record.wrapping_add(0x4)).sw(c, value);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(this.wrapping_add(0x4)).lw(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_af50);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Each fake answers only while its own gate word (in the test machine's
    // RAM) holds its magic; otherwise it runs the body it replaced.
    pub(crate) const OC1493_RT_GATE: u32 = DATA + 0x0017_0000;
    pub(crate) const OC1493_RT_MAGIC: u32 = 0x1493_0001;
    pub(crate) const OC1493_SIZE_GATE: u32 = DATA + 0x0017_0100;
    pub(crate) const OC1493_SIZE_MAGIC: u32 = 0x1493_0002;
    pub(crate) const OC1493_CLOSE_GATE: u32 = DATA + 0x0017_0200;
    pub(crate) const OC1493_CLOSE_MAGIC: u32 = 0x1493_0003;
    pub(crate) const OC1493_TAIL_0274_GATE: u32 = DATA + 0x0017_0300;
    pub(crate) const OC1493_TAIL_0274_MAGIC: u32 = 0x1493_0004;
    pub(crate) const OC1493_TAIL_39E0_GATE: u32 = DATA + 0x0017_0400;
    pub(crate) const OC1493_TAIL_39E0_MAGIC: u32 = 0x1493_0005;
    pub(crate) const OC1493_QUAD_GATE: u32 = DATA + 0x0017_0500;
    pub(crate) const OC1493_QUAD_MAGIC: u32 = 0x1493_0006;

    /// A fake with no result of its own (the callee's v0 is the fake's hash).
    macro_rules! oc1493_fake {
        ($name:ident, $prev:ident, $gen:path, $target:expr, $gate:expr, $magic:expr) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32($gate) != $magic {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    /// Logs the call as a fake and returns the word at `gate + slot` in v0
    /// (what the test wrote there), when the gate is on and the call returned.
    fn oc1493_answer(
        c: &mut Cpu,
        gate: u32,
        magic: u32,
        prev: &AtomicUsize,
        generated: fn(&mut Cpu),
        target: u32,
        slot: u32,
    ) {
        if c.bus.read32(gate) != magic {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(gate + slot) as u64;
        }
    }

    pub(crate) static OC1493_PREV_GET_UINT: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn oc1493_fake_get_uint(c: &mut Cpu) {
        if c.bus.read32(OC1493_RT_GATE) != OC1493_RT_MAGIC {
            return fakes::fall_back(c, &OC1493_PREV_GET_UINT, ee::s09::f_002d2558__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002d_2558, ra);
    }

    pub(crate) static OC1493_PREV_READ_VOLUME: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn oc1493_fake_read_volume(c: &mut Cpu) {
        oc1493_answer(
            c,
            OC1493_RT_GATE,
            OC1493_RT_MAGIC,
            &OC1493_PREV_READ_VOLUME,
            ee::s02::f_0016c4c8__generated,
            0x0016_c4c8,
            0x8,
        );
    }

    pub(crate) static OC1493_PREV_READ_ACTION: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn oc1493_fake_read_action(c: &mut Cpu) {
        // The first action read answers at +0xc, the second (returning to
        // 0x16c4ac) at +0x10.
        let slot = if c.g[RA][0] as u32 == 0x0016_c4ac {
            0x10
        } else {
            0xc
        };
        oc1493_answer(
            c,
            OC1493_RT_GATE,
            OC1493_RT_MAGIC,
            &OC1493_PREV_READ_ACTION,
            ee::s02::f_0016c518__generated,
            0x0016_c518,
            slot,
        );
    }

    pub(crate) static OC1493_PREV_CD_GET_ENTRY: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn oc1493_fake_cd_get_entry(c: &mut Cpu) {
        oc1493_answer(
            c,
            OC1493_SIZE_GATE,
            OC1493_SIZE_MAGIC,
            &OC1493_PREV_CD_GET_ENTRY,
            ee::s14::f_003e48d8__generated,
            0x003e_48d8,
            0x8,
        );
    }

    pub(crate) static OC1493_PREV_LSEEK: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn oc1493_fake_lseek(c: &mut Cpu) {
        oc1493_answer(
            c,
            OC1493_SIZE_GATE,
            OC1493_SIZE_MAGIC,
            &OC1493_PREV_LSEEK,
            ee::s15::f_00428168__generated,
            0x0042_8168,
            0x10,
        );
    }

    oc1493_fake!(
        oc1493_fake_sce_close,
        OC1493_PREV_SCE_CLOSE,
        ee::s15::f_00427fe8__generated,
        0x0042_7fe8,
        OC1493_CLOSE_GATE,
        OC1493_CLOSE_MAGIC
    );

    oc1493_fake!(
        oc1493_fake_tail_0274,
        OC1493_PREV_TAIL_0274,
        ee::s13::f_003a0274__generated,
        0x003a_0274,
        OC1493_TAIL_0274_GATE,
        OC1493_TAIL_0274_MAGIC
    );

    oc1493_fake!(
        oc1493_fake_tail_39e0,
        OC1493_PREV_TAIL_39E0,
        ee::s13::f_003a39e0__generated,
        0x003a_39e0,
        OC1493_TAIL_39E0_GATE,
        OC1493_TAIL_39E0_MAGIC
    );

    oc1493_fake!(
        oc1493_fake_quad_init,
        OC1493_PREV_QUAD_INIT,
        ee::s08::f_002c17c0__generated,
        0x002c_17c0,
        OC1493_QUAD_GATE,
        OC1493_QUAD_MAGIC
    );

    pub(crate) fn oc1493_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2558),
            oc1493_fake_get_uint,
            &OC1493_PREV_GET_UINT,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C4C8),
            oc1493_fake_read_volume,
            &OC1493_PREV_READ_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C518),
            oc1493_fake_read_action,
            &OC1493_PREV_READ_ACTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E48D8),
            oc1493_fake_cd_get_entry,
            &OC1493_PREV_CD_GET_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00428168),
            oc1493_fake_lseek,
            &OC1493_PREV_LSEEK,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00427FE8),
            oc1493_fake_sce_close,
            &OC1493_PREV_SCE_CLOSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0274),
            oc1493_fake_tail_0274,
            &OC1493_PREV_TAIL_0274,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A39E0),
            oc1493_fake_tail_39e0,
            &OC1493_PREV_TAIL_39E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C17C0),
            oc1493_fake_quad_init,
            &OC1493_PREV_QUAD_INIT,
        );
    }

    /// `CameraTriggerMan_ReadTrigger`: the two get_uint reads, the volume and
    /// the two action reads are fakes (answers from the gate words); the
    /// statuses cover every early exit.
    #[test]
    fn camera_trigger_man_read_trigger_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s02::f_0016c428__generated,
            camera_trigger_man_read_trigger,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x16c428, 3);
                c.bus.write32(OC1493_RT_GATE, OC1493_RT_MAGIC);
                let nonzero = |r: &mut Rng| r.next_u32() | 1;
                c.bus.write32(
                    OC1493_RT_GATE + 0x8,
                    if seed % 3 == 0 { 0 } else { nonzero(&mut r) },
                );
                c.bus.write32(
                    OC1493_RT_GATE + 0xc,
                    if seed % 4 == 0 { 0 } else { nonzero(&mut r) },
                );
                c.bus.write32(
                    OC1493_RT_GATE + 0x10,
                    if seed % 5 == 0 { 0 } else { nonzero(&mut r) },
                );
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = sx(r.next_u32());
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `EAFile_PlatformGetSize`: devices 1, 2 and others (the top byte of the
    /// handle), with the CD entry and lseek answered by the gated fakes.
    #[test]
    fn eafile_platform_get_size_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s14::f_003e34e8__generated,
            eafile_platform_get_size,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3e34e8, 1);
                c.bus.write32(OC1493_SIZE_GATE, OC1493_SIZE_MAGIC);
                c.bus.write32(OC1493_SIZE_GATE + 0x8, r.next_u32());
                c.bus.write32(OC1493_SIZE_GATE + 0x10, r.next_u32());
                let device = [1u32, 2, 3, 0, 0x81][seed as usize % 5];
                let word = (device << 24) | (r.next_u32() & 0x00ff_ffff);
                c.g[A0][0] = sx(word);
            },
        );
    }

    /// `EAFile_PlatformClose`: device 2 with the slot holding the fd, holding
    /// another value, and other devices (no sceClose).
    #[test]
    fn eafile_platform_close_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s14::f_003e32f8__generated,
            eafile_platform_close,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3e32f8, 1);
                let handle = r.next_u32() & 0x00ff_ffff;
                let device = if seed % 2 == 0 {
                    2
                } else {
                    [1u32, 3, 0, 0x81][(seed as usize / 2) % 4]
                };
                c.g[A0][0] = sx((device << 24) | handle);
                let slot = if seed % 3 == 0 {
                    r.next_u32()
                } else {
                    handle
                };
                c.bus.write32(0x0045_0c40, slot);
                c.bus.write32(OC1493_CLOSE_GATE, OC1493_CLOSE_MAGIC);
            },
        );
    }

    /// `lbl_003A0270`: the store and the tail call to 0x3a0274 (faked).
    #[test]
    fn lbl_003a0270_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s13::f_003a0270__generated,
            lbl_003a0270,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a0270, 1);
                c.bus.write32(OC1493_TAIL_0274_GATE, OC1493_TAIL_0274_MAGIC);
                c.g[A3][0] = (DATA + 0x1000) as u64;
                c.f[1] = r.edge_f32();
            },
        );
    }

    /// `lbl_003A3948`: `f0 = f1 * gp[-0x24f0]`, stored, tail call (faked).
    #[test]
    fn lbl_003a3948_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s13::f_003a3948__generated,
            lbl_003a3948,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a3948, 1);
                c.bus.write32(OC1493_TAIL_39E0_GATE, OC1493_TAIL_39E0_MAGIC);
                c.bus.write32(GAME_GP.wrapping_sub(0x24f0), r.edge_f32());
                c.g[T0][0] = (DATA + 0x1000) as u64;
                c.f[1] = r.edge_f32();
            },
        );
    }

    /// `lbl_003A3958`: as above with `gp[-0x24ec]` and the store at +0x54.
    #[test]
    fn lbl_003a3958_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s13::f_003a3958__generated,
            lbl_003a3958,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a3958, 1);
                c.bus.write32(OC1493_TAIL_39E0_GATE, OC1493_TAIL_39E0_MAGIC);
                c.bus.write32(GAME_GP.wrapping_sub(0x24ec), r.edge_f32());
                c.g[T0][0] = (DATA + 0x1000) as u64;
                c.f[1] = r.edge_f32();
            },
        );
    }

    /// `WPData_Relocate`: the head offset, the record count (negative, zero
    /// and up to three records) and records that are unset (-1) or not.
    #[test]
    fn wp_data_relocate_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s08::f_002baf08__generated,
            wp_data_relocate,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2baf08, 1);
                c.bus.write32(OC1493_QUAD_GATE, OC1493_QUAD_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, r.next_u32() % 0x800);
                let count = [0u32, 1, 2, 3, 0xffff_ffff][seed as usize % 5];
                c.bus.write32(this + 0x4, count);
                c.bus.write32(this + 0x8, 0x400);
                for i in 0..3u32 {
                    let word = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 0x800
                    };
                    c.bus.write32(this + 0x400 + 8 * i + 0x4, word);
                }
            },
        );
    }

    /// `cWorldLine_Transform`: a leaf with VU0 work; random matrix, points
    /// and f12 (edge cases included).
    #[test]
    fn cworldline_transform_matches() {
        check(
            ee::s11::f_0032e398__generated,
            cworldline_transform,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x32e398, 0);
                fill(c, &mut r, DATA + 0x1000, 0x80);
                fill(c, &mut r, DATA + 0x2000, 0x40);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }
}
