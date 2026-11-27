//! Staging module oc1412 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1412`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `VD_FloorLog2` (`0x00254350`).
    pub(crate) const VD_FLOOR_LOG2: Callee = Callee {
        addr: 0x0025_4350,
        f: ee::s06::VD_FloorLog2,
    };

    /// `VD_TextureSink_SetFrameSize` (`0x002543f0`).
    pub(crate) const VD_TEXTURE_SINK_SET_FRAME_SIZE: Callee = Callee {
        addr: 0x0025_43f0,
        f: ee::s06::VD_TextureSink_SetFrameSize,
    };

    /// `VD_TextureSink_SetFrameRate` (`0x00254400`).
    pub(crate) const VD_TEXTURE_SINK_SET_FRAME_RATE: Callee = Callee {
        addr: 0x0025_4400,
        f: ee::s06::VD_TextureSink_SetFrameRate,
    };

    /// `PS2GraphicsMan_SetTextureFilter` (`0x0037d090`), called through the
    /// function table.
    pub(crate) const PS2_GRAPHICS_MAN_SET_TEXTURE_FILTER: u32 = 0x0037_d090;

    /// `MPEG2_Initialize_Buffer` (`0x003b1228`).
    pub(crate) const MPEG2_INITIALIZE_BUFFER: Callee = Callee {
        addr: 0x003b_1228,
        f: ee::s13::MPEG2_Initialize_Buffer,
    };

    /// `MPEG2_Get_Hdr` (`0x003b13d8`).
    pub(crate) const MPEG2_GET_HDR: Callee = Callee {
        addr: 0x003b_13d8,
        f: ee::s13::MPEG2_Get_Hdr,
    };

    /// `PS2_SONY_CODEC_INTERNAL_InitFrame` (`0x003b07f8`).
    pub(crate) const SONY_CODEC_INIT_FRAME: Callee = Callee {
        addr: 0x003b_07f8,
        f: ee::s13::PS2_SONY_CODEC_INTERNAL_InitFrame,
    };

    /// `sceMpegInit` (`0x00402708`), through the function table.
    pub(crate) const SCE_MPEG_INIT: u32 = 0x0040_2708;
    /// `sceMpegCreate` (`0x004027b8`), through the function table.
    pub(crate) const SCE_MPEG_CREATE: u32 = 0x0040_27b8;
    /// `sceMpegAddCallback` (`0x00402c08`), through the function table.
    pub(crate) const SCE_MPEG_ADD_CALLBACK: u32 = 0x0040_2c08;

    /// `VisualEffectsMan_ctor` (`0x002eff98`).
    pub(crate) const VISUAL_EFFECTS_MAN_CTOR: Callee = Callee {
        addr: 0x002e_ff98,
        f: ee::s09::VisualEffectsMan_ctor,
    };

    /// `EA_MemAllocNamed` (`0x00317d70`), through the function table.
    pub(crate) const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `DetailSystem_ctor` (`0x003759e8`), through the function table.
    pub(crate) const DETAIL_SYSTEM_CTOR: u32 = 0x0037_59e8;
    /// `PathArrow_ctor` (`0x00372aa0`), through the function table.
    pub(crate) const PATH_ARROW_CTOR: u32 = 0x0037_2aa0;

    /// `DLHeap_Malloc` (`0x0031e818`).
    pub(crate) const DLHEAP_MALLOC: Callee = Callee {
        addr: 0x0031_e818,
        f: ee::s10::DLHeap_Malloc,
    };

    /// `DLHeap_Free` (`0x0031ed60`).
    pub(crate) const DLHEAP_FREE: Callee = Callee {
        addr: 0x0031_ed60,
        f: ee::s10::DLHeap_Free,
    };

    /// `DLHeap_Consolidate` (`0x0031eee8`).
    pub(crate) const DLHEAP_CONSOLIDATE: Callee = Callee {
        addr: 0x0031_eee8,
        f: ee::s10::DLHeap_Consolidate,
    };
}

/// `__getreent()` (0x415fb8, through the function table): the errno block.
pub(crate) const DLHEAP_GETREENT: u32 = 0x0041_5fb8;

/// `ENOMEM`, stored to errno when a request is too large.
pub(crate) const DLHEAP_ENOMEM: u64 = 0xc;

/// `memcpy` (0x3e6574) and `memset` (0x3e6448), called through the table.
pub(crate) const DLHEAP_MEMCPY: u32 = 0x003e_6574;

pub(crate) const DLHEAP_MEMSET: u32 = 0x003e_6448;

/// `__builtin_delete` (0x317e50), called through the table.
pub(crate) const VFX_MAN_DTOR_DELETE: u32 = 0x0031_7e50;

/// `PS2VisualEffectsMan`: vtable pointer offset, and the 17 owned objects
/// at `+0x00 .. +0x40`.
pub(crate) const VFX_MAN_VPTR: u32 = 0x44;

#[cfg(test)]
pub(crate) const VFX_MAN_OBJECTS: u32 = 17;

/// Return addresses of the 17 owned-object deletes (copied from the
/// generated `c.g[RA][0] = sx(..)` lines).
pub(crate) const VFX_MAN_DTOR_RETS: [u32; 17] = [
    0x002f_0864,
    0x002f_0884,
    0x002f_08a4,
    0x002f_08c4,
    0x002f_08e4,
    0x002f_0904,
    0x002f_0924,
    0x002f_0944,
    0x002f_0964,
    0x002f_0984,
    0x002f_09a4,
    0x002f_09c4,
    0x002f_09e4,
    0x002f_0a04,
    0x002f_0a24,
    0x002f_0a44,
    0x002f_0a64,
];

/// A gcc 2.9x virtual call on the graphics manager held in register `obj`:
/// the vtable `lw(obj + 0x10d8)` into `vt`, the `this` delta `lh(vt + slot)`
/// into `delta`, the method `lw(vt + slot + 4)` into `func`, then
/// `a0 = obj + delta` and the call returning to `ret` (the generated order).
fn vdts_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    delta: usize,
    func: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[vt][0] = c.lw((c.g[obj][0] as u32).wrapping_add(0x10d8));
    c.g[delta][0] = c.lh((c.g[vt][0] as u32).wrapping_add(slot));
    c.g[func][0] = c.lw((c.g[vt][0] as u32).wrapping_add(slot + 4));
    let target = c.g[func][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[delta][0] as u32));
    call_ptr(c, target, ret)
}

/// gcc's `fd = (float)(unsigned)v0` after a call: a non-negative word
/// converts directly (4 cycles); one with bit 31 set converts
/// `(v0 >> 1) | (v0 & 1)` and doubles it (6 cycles). Both arms leave
/// `v1 = v0 >> 1`.
fn vdts_word_to_float(c: &mut Cpu, fd: usize) {
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if negative {
        c.cycles += 6;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, fd);
        c.cvt_s_w(fd, fd);
        c.add_s(fd, fd, fd);
    } else {
        c.cycles += 4;
        c.mtc1(2, fd);
        c.cvt_s_w(fd, fd);
    }
}

/// `VD_TextureSink *VD_TextureSink_ctor(VD_TextureSink *sink, int id,
/// VD_ImageDesc *desc, PS2GraphicsMan *gm, int t0)` (0x254410): build a
/// video texture sink over the image of `desc`: the pixels follow the
/// header (or sit at its `+0x10` offset when flag `0x1000` is set), the
/// height is split into power-of-two strips (count at `+0x20`, heights from
/// `+0x24`), one texture cell per 16-pixel column and strip is created
/// through the graphics manager (`+0x178` slot) and given its filter; then
/// the frame is centred on the screen (`SetFrameSize`) at unit scale
/// (`SetFrameRate`) and `t0` lands at `+0x3c`. Returns `sink`.
///
/// Dispatcher-shaped: the strip loop (0x2544c8), the column loop
/// (0x254580) and the cell loop (0x2545a8) unwind at their back edges and
/// are re-entered at those heads.
pub fn vd_texture_sink_ctor(c: &mut Cpu) {
    use callee::*;
    let frame = Frame { size: 0x100 };
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0025_4410 => {
                    c.cycles += 27;
                    Frame::open(c, 0x100);
                    c.g[V0][0] = 0x480000;
                    frame.save(c, 19, 0xa0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x270));
                    frame.save(c, 16, 0xd0);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, 17, 0xc0);
                    frame.save(c, 18, 0xb0);
                    frame.save(c, 20, 0x90);
                    frame.save(c, 21, 0x80);
                    frame.save(c, 22, 0x70);
                    frame.save(c, 23, 0x60);
                    frame.save(c, 30, 0x50);
                    frame.save_ra(c, 0x40);
                    frame.save_fpr(c, 22, 0xf0);
                    frame.save_fpr(c, 21, 0xe8);
                    frame.save_fpr(c, 20, 0xe0);
                    c.sw(c.g[S3][0] as u32, c.g[A1][0]);
                    // The vtable (0x480270).
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw(frame.at(c, 0x20), c.g[T0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] & 0x1000;
                    if c.g[V0][0] == 0u64 {
                        // Pixels right after the 16-byte header (beql slot).
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    } else {
                        // 0x25447c: pixels at the header's own offset.
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    // 0x254484: width/16 columns, height into S0.
                    c.cycles += 17;
                    c.sw(frame.at(c, 0x24), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x6));
                    c.sw(frame.at(c, 0x28), c.g[A0][0]);
                    c.g[S1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), 0u64);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf));
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[S1][0];
                    }
                    c.g[S0][0] = c.lw(frame.at(c, 0x28));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x24));
                    c.g[S2][0] = 0x510000;
                    c.g[S4][0] = 0x480000;
                    pc = 0x0025_44f0;
                    continue;
                }
                0x0025_44c8 => {
                    // Store the strip `1 << log2(rest)` and take it off.
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    pc = 0x0025_44f0;
                    continue;
                }
                0x0025_44f0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    VD_FLOOR_LOG2.call(c, 0x0025_44f8)?;
                    c.g[V1][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[V0][0] as u32) & 31));
                    let t = c.g[S0][0] != c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0025_44c8) {
                            return Ok(());
                        }
                        pc = 0x0025_44c8;
                        continue;
                    }
                    // 0x254508: the last strip (a power of two), then the
                    // cell-pointer array (cells * 12 bytes).
                    c.cycles += 28;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = 0xc;
                    c.g[A2][0] = 0u64;
                    c.mult(0, 5, 5, 3);
                    c.lwc1(21, frame.at(c, 0x28));
                    c.cvt_s_w(21, 21);
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x258));
                    c.g[A3][0] = 0u64;
                    c.mtc1(17, 22);
                    c.cvt_s_w(22, 22);
                    c.g[FP][0] = 0u64;
                    c.mult(0, 0, 5, 2);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A1][0] = c.lo[0];
                    let t = c.g[V0][0] as u32;
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    call_ptr(c, t, 0x0025_4568)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0u64;
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        pc = 0x0025_46a8;
                        continue;
                    }
                    // 0x254578.
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    pc = 0x0025_4580;
                    continue;
                }
                0x0025_4580 => {
                    // One 16-pixel column (index in A0).
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[S7][0] = 0u64;
                    c.g[S5][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.sw(frame.at(c, 0x30), c.g[V1][0]);
                    if t {
                        pc = 0x0025_4694;
                        continue;
                    }
                    // 0x254594.
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 6);
                    c.g[S4][0] = c.g[S6][0];
                    c.g[A0][0] = c.lw(frame.at(c, 0x28));
                    c.mult(0, 4, 4, 2);
                    c.sw(frame.at(c, 0x2c), c.g[A0][0]);
                    pc = 0x0025_45a8;
                    continue;
                }
                0x0025_45a8 => {
                    // One cell: a 12-byte record, its texture, its filter.
                    c.cycles += 59;
                    c.g[A1][0] = 0x510000;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ea0));
                    c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[S2][0] = sx((c.g[FP][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x510000;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6bd0));
                    c.g[A2][0] = 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = 0xc;
                    c.g[A3][0] = 0u64;
                    let t = c.g[V0][0] as u32;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    call_ptr(c, t, 0x0025_45e4)?;
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V0][0] = 0x10;
                    c.g[A1][0] = sx((c.g[S7][0] as u32) << 6);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[T2][0] = 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V1][0] = c.lw(frame.at(c, 0x2c));
                    c.g[T3][0] = 0u64;
                    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.g[T1][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(frame.at(c, 0x24));
                    c.g[V1][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2ea8));
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x178));
                    c.sw(frame.at(c, 0x18), c.g[V0][0]);
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(frame.at(c, 0x8), 0u64);
                    c.sw(frame.at(c, 0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x17c));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] as u32;
                    c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, t, 0x0025_4658)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0u64;
                    call_ptr(c, PS2_GRAPHICS_MAN_SET_TEXTURE_FILTER, 0x0025_4678)?;
                    c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                    c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x0025_45a8) {
                            return Ok(());
                        }
                        pc = 0x0025_45a8;
                        continue;
                    }
                    pc = 0x0025_4694;
                    continue;
                }
                0x0025_4694 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw(frame.at(c, 0x30));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                        if back_edge_unwinds(c, 0x0025_4580) {
                            return Ok(());
                        }
                        pc = 0x0025_4580;
                        continue;
                    }
                    pc = 0x0025_46a8;
                    continue;
                }
                0x0025_46a8 => {
                    // Screen width (slot 0x50) and height (slot 0x58) as
                    // unsigned floats.
                    c.cycles += 9;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    vdts_vcall(c, A1, V1, A0, V0, 0x50, 0x0025_46c4)?;
                    vdts_word_to_float(c, 20);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.cycles += 7;
                    vdts_vcall(c, A1, V1, A0, V0, 0x58, 0x0025_4708)?;
                    vdts_word_to_float(c, 13);
                    c.sub_s(13, 13, 21);
                    // 0x254738: centre the frame, unit scale, keep `t0`.
                    c.cycles += 32;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 0);
                    c.sub_s(12, 20, 22);
                    c.g[A0][0] = 0x4a0000;
                    c.lwc1(14, (c.g[A0][0] as u32).wrapping_add(0x2eb0));
                    c.mul_s(13, 13, 0);
                    c.g[A0][0] = c.g[S3][0];
                    c.mul_s(12, 12, 0);
                    VD_TEXTURE_SINK_SET_FRAME_SIZE.call(c, 0x0025_475c)?;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S3][0];
                    c.mov_s(13, 12);
                    VD_TEXTURE_SINK_SET_FRAME_RATE.call(c, 0x0025_4770)?;
                    c.g[A1][0] = c.lw(frame.at(c, 0x20));
                    c.g[V0][0] = c.g[S3][0];
                    frame.restore(c, 16, 0xd0);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
                    frame.restore(c, 17, 0xc0);
                    frame.restore(c, 18, 0xb0);
                    frame.restore(c, 19, 0xa0);
                    frame.restore(c, 20, 0x90);
                    frame.restore(c, 21, 0x80);
                    frame.restore(c, 22, 0x70);
                    frame.restore(c, 23, 0x60);
                    frame.restore(c, 30, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.restore_fpr(c, 22, 0xf0);
                    frame.restore_fpr(c, 21, 0xe8);
                    frame.restore_fpr(c, 20, 0xe0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0025_4410, 0x0025_47b8);
                    return Ok(());
                }
            }
        }
    })
}

/// `void VD_TextureSink_DrawFrame(VD_TextureSink *sink)` (0x2548d0): wait
/// until the graphics manager is idle (slot `0x88`), set a full-screen
/// viewport (slot `0xd0`, the screen size from slots `0x50`/`0x58` as
/// unsigned floats) and the render state (slot `0x110` with `0x4ff1a0`),
/// put the sink's 5-bit layer `+0x3c` into the top render state, then draw
/// one textured quad per cell (slot `0x238`), column strip by column strip
/// from `(x, y, z) = (+0x40, +0x44, +0x48)`, cell width `+0x4c * 16`, row
/// heights `+0x24[i] * +0x50`.
pub fn vd_texture_sink_draw_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        frame.save(c, 16, 0x120);
        frame.save(c, 17, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x100);
        frame.save(c, 19, 0xf0);
        frame.save(c, 20, 0xe0);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 27, 0x168);
        frame.save_fpr(c, 26, 0x160);
        frame.save_fpr(c, 25, 0x158);
        frame.save_fpr(c, 24, 0x150);
        frame.save_fpr(c, 23, 0x148);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 21, 0x138);
        frame.save_fpr(c, 20, 0x130);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        // 0x254918: spin until the renderer is idle.
        loop {
            c.cycles += 7;
            vdts_vcall(c, A1, V1, A0, V0, 0x88, 0x0025_492c)?;
            let busy = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            if !busy {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0025_4918);
            }
        }
        // Screen width into f20, height into f15.
        c.cycles += 7;
        vdts_vcall(c, A1, V1, A0, V0, 0x50, 0x0025_4948)?;
        vdts_word_to_float(c, 20);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.cycles += 7;
        vdts_vcall(c, A1, V1, A0, V0, 0x58, 0x0025_498c)?;
        vdts_word_to_float(c, 15);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        // 0x2549bc: viewport (0, 0, 0.0, 0.0, w, h, 0.0, 1.0).
        c.cycles += 33;
        c.g[A1][0] = 0u64;
        c.mtc1(0, 12);
        c.g[A2][0] = 0u64;
        c.mov_s(14, 20);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 17);
        c.mov_s(13, 12);
        c.mov_s(16, 12);
        vdts_vcall(c, A3, V1, A0, V0, 0xd0, 0x0025_49f0)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x500000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
        vdts_vcall(c, A2, V0, A0, V1, 0x110, 0x0025_4a10)?;
        // The top render state's layer bits (`+0x8`, bits 5..9).
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.g[A2][0] = 0x3;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.g[V1][0] & 0x3e0;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        // 0x254a40: the compiler's empty 4-step count-down.
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let t = c.g[A2][0] != c.g[A3][0];
            c.g[A0][0] = 0x80;
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0025_4a40);
            }
        }
        // Four vertices on the stack: colour 0x80 x4, q = 1.0.
        c.cycles += 5;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V1][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
        loop {
            c.cycles += 8;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                break;
            }
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0025_4a70);
            }
        }
        // 0x254a90: uv corners, x0 = x + 0.5, cell width = cw * 16.
        c.cycles += 23;
        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x4c));
        c.g[S3][0] = 0u64;
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = 0u64;
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 0);
        c.mul_s(24, 3, 1);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.add_s(20, 2, 0);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.sw(frame.at(c, 0), 0u64);
        c.swc1(1, frame.at(c, 0x94));
        c.sw(frame.at(c, 0x4), 0u64);
        c.add_s(21, 20, 24);
        c.swc1(1, frame.at(c, 0x30));
        c.sw(frame.at(c, 0x34), 0u64);
        c.sw(frame.at(c, 0x60), 0u64);
        c.swc1(1, frame.at(c, 0x64));
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.swc1(1, frame.at(c, 0x90));
        if !no_rows {
            c.cycles += 1;
            // 0x254af0: one column strip per `+0x1c` count.
            loop {
                'cells: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.mtc1(0, 23);
                    c.g[S2][0] = 0u64;
                    c.add_s(25, 20, 24);
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.add_s(26, 21, 24);
                    if t {
                        break 'cells;
                    }
                    c.cycles += 7;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 27);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 22);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    // 0x254b28: one quad per `+0x20` count.
                    loop {
                        c.cycles += 49;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.lwc1(2, c.g[S1][0] as u32);
                        c.cvt_s_w(2, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A1][0] = 0x4;
                        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                        c.add_s(0, 0, 27);
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        c.g[A2][0] = c.g[SP][0];
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x50));
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.sh((c.g[A3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        c.add_s(0, 0, 23);
                        c.swc1(20, frame.at(c, 0xc0));
                        c.mul_s(2, 2, 1);
                        c.g[A3][0] = 0u64;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x48));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.swc1(0, frame.at(c, 0xc4));
                        c.add_s(3, 0, 2);
                        c.swc1(1, frame.at(c, 0xc8));
                        c.add_s(23, 23, 2);
                        c.swc1(22, frame.at(c, 0xcc));
                        c.lq(2, frame.at(c, 0xc0));
                        c.swc1(21, frame.at(c, 0xc0));
                        c.sq(2, frame.at(c, 0x20));
                        c.lq(3, frame.at(c, 0xc0));
                        c.swc1(20, frame.at(c, 0xc0));
                        c.swc1(3, frame.at(c, 0xc4));
                        c.sq(3, frame.at(c, 0x50));
                        c.lq(4, frame.at(c, 0xc0));
                        c.swc1(21, frame.at(c, 0xc0));
                        c.sq(4, frame.at(c, 0x80));
                        c.lq(2, frame.at(c, 0xc0));
                        c.sq(2, frame.at(c, 0xb0));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        vdts_vcall(c, A0, V0, V1, T0, 0x238, 0x0025_4bdc)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0u64 {
                            break 'cells;
                        }
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_4b28);
                        }
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[V1][0] = c.g[S4][0];
                c.mov_s(20, 25);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0u64;
                c.mov_s(21, 26);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0025_4af0);
                }
            }
        }
        c.cycles += 16;
        frame.restore(c, 16, 0x120);
        frame.restore(c, 17, 0x110);
        frame.restore(c, 18, 0x100);
        frame.restore(c, 19, 0xf0);
        frame.restore(c, 20, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 27, 0x168);
        frame.restore_fpr(c, 26, 0x160);
        frame.restore_fpr(c, 25, 0x158);
        frame.restore_fpr(c, 24, 0x150);
        frame.restore_fpr(c, 23, 0x148);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

/// Draw one grid strip of `n` stack vertices through the graphics
/// manager's slot `0x248` (`PS2GraphicsMan_v73`): `(this, n, verts, 0)`.
fn bezier_grid_draw_strip(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = 0u64;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `void PS2BezierMan_DebugDrawGrid(Vec3 *points, Vec2 *uvs, int n)`
/// (0x38c590): draw the `n x n` control grid as `n` row strips then `n`
/// column strips (colour 0x80, w = 1.0), each built on the stack (at most
/// 64 vertices) and drawn through the global graphics manager
/// (`gp + 0x2a90`). Nothing for `n <= 0`.
pub fn ps2_bezier_man_debug_draw_grid(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc80);
        c.g[V0][0] = 0x3f;
        frame.save(c, 17, 0xc60);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, 16, 0xc70);
        frame.save(c, 19, 0xc40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 20, 0xc30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 18, 0xc50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 21, 0xc20);
        frame.save_ra(c, 0xc10);
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[V1][0] = 0u64;
        if !t {
            c.cycles += 2;
            c.g[S5][0] = 0xc;
            // 0x38c5d8: row strips (row index in V1).
            loop {
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) <= 0;
                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if !t {
                    c.cycles += 10;
                    c.mult(0, 2, 3, 16);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[A3][0] = 0x80;
                    c.g[V1][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.mult(0, 5, 2, 21);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    // 0x38c608: one vertex per point of the row.
                    loop {
                        c.cycles += 23;
                        c.lwc1(1, c.g[A0][0] as u32);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                        c.lwc1(2, c.g[A1][0] as u32);
                        c.swc1(1, frame.at(c, 0xc00));
                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                        c.swc1(0, frame.at(c, 0xc04));
                        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.swc1(3, frame.at(c, 0xc0c));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.swc1(1, frame.at(c, 0xc08));
                        c.lq(2, frame.at(c, 0xc00));
                        c.swc1(2, c.g[V1][0] as u32);
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                        c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                        let more = c.g[A2][0] != 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0038_c608);
                        }
                    }
                }
                c.cycles += 12;
                bezier_grid_draw_strip(c, 0x0038_c684)?;
                c.g[V1][0] = c.g[S2][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_c5d8);
                }
            }
        }
        // 0x38c694: column strips (column index in V1).
        c.cycles += 2;
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[V1][0] = 0u64;
        if !t {
            c.cycles += 1;
            loop {
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) <= 0;
                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if !t {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[A2][0] = c.g[V1][0];
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[T1][0] = 0xc;
                    c.g[A3][0] = 0x80;
                    c.g[V1][0] = c.g[SP][0];
                    c.g[T0][0] = sx((c.g[S0][0] as u32) << 3);
                    c.g[A1][0] = c.g[S0][0];
                    // 0x38c6d0: one vertex per point of the column.
                    loop {
                        c.cycles += 25;
                        c.mult(0, 10, 6, 9);
                        c.lwc1(2, c.g[A0][0] as u32);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.swc1(0, frame.at(c, 0xc00));
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                        c.swc1(1, frame.at(c, 0xc04));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.swc1(3, frame.at(c, 0xc0c));
                        c.swc1(0, frame.at(c, 0xc08));
                        c.lq(2, frame.at(c, 0xc00));
                        c.swc1(2, c.g[V1][0] as u32);
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                        c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                        let more = c.g[A1][0] != 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0038_c6d0);
                        }
                    }
                }
                c.cycles += 12;
                bezier_grid_draw_strip(c, 0x0038_c754)?;
                c.g[V1][0] = c.g[S2][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if c.g[V0][0] == 0u64 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_c6a0);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0xc70);
        frame.restore(c, 17, 0xc60);
        frame.restore(c, 18, 0xc50);
        frame.restore(c, 19, 0xc40);
        frame.restore(c, 20, 0xc30);
        frame.restore(c, 21, 0xc20);
        frame.restore_ra(c, 0xc10);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2_SONY_CODEC_INTERNAL_Open(SonyCodec *ctx, Script *script,
/// Stream *stream)` (0x3b0c58): (re)open the MPEG decoder on `stream`:
/// free the old work and IPU buffers (video free hook `0x509434`), parse the
/// sequence header (`MPEG2_Initialize_Buffer` + `MPEG2_Get_Hdr` into the
/// decoder globals at `0x509648`; MPEG-1 streams get progressive defaults),
/// derive the macroblock-aligned frame size (interlaced MPEG-2 rounds the
/// height to 32), allocate and link `script->count` frames, the work area
/// (`w * h * 4.5 + 0x176c`) and the IPU buffer (0x32000, uncached alias at
/// `+0x7c`), then `sceMpegInit` / `sceMpegCreate` / `sceMpegAddCallback`
/// with the no-data callback. Returns 2.
pub fn ps2_sony_codec_internal_open(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 30, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 16, 0xa0);
        c.g[FP][0] = 0x510000;
        frame.save(c, 17, 0x90);
        frame.save(c, 20, 0x60);
        frame.save(c, 21, 0x50);
        frame.save(c, 22, 0x40);
        frame.save(c, 23, 0x30);
        frame.save_ra(c, 0x10);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x28), c.g[A1][0]);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), 0u64);
        let no_work = c.g[A0][0] == 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
        if !no_work {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] as u32;
            call_ptr(c, t, 0x003b_0cb4)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), 0u64);
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
        let no_ipu = c.g[A0][0] == 0u64;
        c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        if !no_ipu {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] as u32;
            call_ptr(c, t, 0x003b_0cd0)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x78), 0u64);
        }
        // Parse the sequence header into the decoder globals.
        c.cycles += 13;
        c.g[S1][0] = 0x1;
        c.g[S0][0] = 0x510000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[S1][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x69b8));
        c.g[V0][0] = 0x510000;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
        c.sw((c.g[V0][0] as u32).wrapping_sub(0x5f78), c.g[S0][0]);
        MPEG2_INITIALIZE_BUFFER.call(c, 0x003b_0cf4)?;
        MPEG2_GET_HDR.call(c, 0x003b_0cfc)?;
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x420));
        let mpeg2 = c.g[T1][0] != 0u64;
        c.g[V1][0] = 0x510000;
        if !mpeg2 {
            // MPEG-1: progressive 4:2:0 frame-picture defaults.
            c.cycles += 13;
            c.g[A3][0] = 0x510000;
            c.g[T0][0] = 0x510000;
            c.g[A2][0] = 0x510000;
            c.g[V1][0] = 0x3;
            c.g[A0][0] = 0x510000;
            c.g[A1][0] = 0x510000;
            c.g[V0][0] = 0x5;
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x6a7c), c.g[V1][0]);
            c.sw((c.g[A0][0] as u32).wrapping_sub(0x6a74), c.g[S1][0]);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x6ab8), c.g[V0][0]);
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x6adc), c.g[S1][0]);
            c.sw((c.g[T0][0] as u32).wrapping_sub(0x6a60), c.g[S1][0]);
            c.g[V1][0] = 0x510000;
        }
        // Macroblock columns (`(hsize + 15) / 16`) into 0x5094fc, rows into
        // V0 (stored at 0x509500 below).
        'mb_rows: {
            'rows_by_16: {
                'mpeg1: {
                    c.cycles += 14;
                    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6b0c));
                    c.g[A2][0] = 0x510000;
                    c.g[A1][0] = 0x510000;
                    c.g[T3][0] = c.g[A2][0];
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                    c.g[T2][0] = c.g[A1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6b00));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    let t = c.g[T1][0] == 0u64;
                    c.sw((c.g[A2][0] as u32).wrapping_sub(0x6b04), c.g[V0][0]);
                    if t {
                        break 'mpeg1;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 0x510000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6adc));
                    let progressive = c.g[V1][0] != 0u64;
                    c.g[A0][0] = 0x510000;
                    if progressive {
                        break 'rows_by_16;
                    }
                    // Interlaced: rows = (vsize + 31) / 32 * 2.
                    c.cycles += 9;
                    c.g[V1][0] = 0x510000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6b08));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    break 'mb_rows;
                }
                c.cycles += 1;
                c.g[A0][0] = 0x510000;
            }
            // rows = (vsize + 15) / 16.
            c.cycles += 7;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x6b08));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A1][0];
            }
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        }
        // 0x3b0dc8: pixel sizes (and halves) into the globals and `ctx`.
        c.cycles += 22;
        c.sw(c.g[T0][0] as u32, c.g[V0][0]);
        c.g[A0][0] = 0x510000;
        c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_sub(0x6b04));
        c.g[A2][0] = 0x510000;
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
        c.g[T0][0] = 0x510000;
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x6b00));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.sw((c.g[A0][0] as u32).wrapping_sub(0x6b2c), c.g[V0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x6b28), c.g[V1][0]);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
        c.sw((c.g[T0][0] as u32).wrapping_sub(0x6b24), c.g[V0][0]);
        c.g[A0][0] = 0x510000;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
        c.sw((c.g[A0][0] as u32).wrapping_sub(0x6b20), c.g[A1][0]);
        let no_frames = (c.g[V0][0] as i64) <= 0;
        c.sw(frame.at(c, 0), c.g[V1][0]);
        if !no_frames {
            c.cycles += 6;
            c.g[S4][0] = 0x510000;
            c.g[S3][0] = c.g[V0][0];
            c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6bd0));
            c.g[S6][0] = 0x490000;
            c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
            // 0x3b0e38: allocate, init and push one frame on `ctx + 0x20`.
            loop {
                c.cycles += 22;
                c.g[T0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x53e8));
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x6bd0));
                c.g[A1][0] = 0x14;
                c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x003b_0e5c)?;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S0][0];
                SONY_CODEC_INIT_FRAME.call(c, 0x003b_0e70)?;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0u64;
                }
                c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                let more = c.g[S3][0] != 0u64;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S5][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_0e38);
                }
            }
        }
        // 0x3b0e90: work area, IPU buffer, then the libmpeg decoder.
        c.cycles += 68;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = 0x490000;
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x53f0));
        c.mult(0, 9, 9, 2);
        c.g[A2][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        c.g[T0][0] = c.g[T0][0] | 0x400;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[T1][0] as u32) << 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.g[V1][0] = sx((c.g[A1][0] as u32) >> 31);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x176c));
        call_ptr(c, t, 0x003b_0ed8)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
        c.g[A1][0] = 0x30000;
        c.g[A0][0] = 0x490000;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5400));
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_sub(0x6bd0));
        c.g[A2][0] = 0u64;
        c.g[T0][0] = c.g[T0][0] | 0x400;
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[A1][0] | 0x2000;
        call_ptr(c, t, 0x003b_0f04)?;
        c.g[V1][0] = 0xfff0000;
        c.g[A0][0] = 0x30000000;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
        call_ptr(c, SCE_MPEG_INIT, 0x003b_0f24)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lw(frame.at(c, 0));
        c.mult(0, 3, 3, 2);
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2c));
        c.g[A2][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 31);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 1) as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x176c));
        call_ptr(c, SCE_MPEG_CREATE, 0x003b_0f54)?;
        c.g[A0][0] = c.lw(frame.at(c, 0));
        c.g[A2][0] = 0x3b0000;
        c.g[A3][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb10));
        c.g[A1][0] = 0x1;
        call_ptr(c, SCE_MPEG_ADD_CALLBACK, 0x003b_0f6c)?;
        frame.restore(c, 16, 0xa0);
        c.g[V0][0] = 0x2;
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The sub-managers `PS2VisualEffectsMan_ctor` allocates (4-byte objects
/// from `EA_MemAllocNamed`), before `DetailSystem`: (vtable written into the
/// previous allocation, name of this allocation, field of `self` that
/// receives the previous allocation, return address of this allocation).
const VFX_MAN_PARTS_BEFORE_DETAIL: [(u32, u32, u32, u32); 6] = [
    (0x0049_31a8, 0x0048_7bc8, 0x00, 0x002f_05a8),
    (0x0049_2fd0, 0x0048_7be0, 0x04, 0x002f_05d0),
    (0x0048_7eb8, 0x004a_3b88, 0x08, 0x002f_05f8),
    (0x0049_3178, 0x0048_7bf0, 0x0c, 0x002f_0620),
    (0x0048_7e88, 0x0048_7c00, 0x10, 0x002f_0648),
    (0x0048_7e58, 0x0048_7c10, 0x14, 0x002f_0670),
];

/// As [`VFX_MAN_PARTS_BEFORE_DETAIL`], between `DetailSystem` and
/// `PathArrow`.
const VFX_MAN_PARTS_AFTER_DETAIL: [(u32, u32, u32, u32); 9] = [
    (0x0049_2fa0, 0x004a_3b90, 0x1c, 0x002f_06bc),
    (0x0048_7e28, 0x0048_7c30, 0x20, 0x002f_06e4),
    (0x0048_7df8, 0x0048_7c40, 0x24, 0x002f_070c),
    (0x0048_7fa8, 0x004a_3b98, 0x28, 0x002f_0734),
    (0x0048_7dc8, 0x0048_7c50, 0x2c, 0x002f_075c),
    (0x0048_8330, 0x0048_7c68, 0x30, 0x002f_0784),
    (0x0048_8050, 0x0048_7c78, 0x34, 0x002f_07ac),
    (0x0048_82b0, 0x004a_3ba0, 0x38, 0x002f_07d4),
    (0x0049_30a0, 0x0048_7c88, 0x3c, 0x002f_07fc),
];

/// One step of the allocation chain: give the previous object (`v0`) its
/// vtable, store it at `self + field`, allocate the next 4-byte object
/// named `name`.
fn vfx_man_alloc_part(
    c: &mut Cpu,
    vtable: u32,
    name: u32,
    field: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[V1][0] = sx(vtable);
    c.g[A1][0] = sx(name);
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    c.g[A0][0] = 0x4;
    c.sw((c.g[S0][0] as u32).wrapping_add(field), c.g[V0][0]);
    c.g[A2][0] = 0u64;
    c.g[A3][0] = 0u64;
    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, ret)
}

/// `PS2VisualEffectsMan *PS2VisualEffectsMan_ctor(PS2VisualEffectsMan
/// *self)` (0x2f0548): the base `VisualEffectsMan_ctor`, the PS2 vtable
/// (`0x487d28` at `+0x44`), then sixteen sub-managers in `+0x00..+0x40`:
/// fifteen 4-byte vtable-only objects, `DetailSystem` at `+0x18` and
/// `PathArrow` at `+0x40` (both constructed in place). Returns `self`.
pub fn ps2_visual_effects_man_ctor(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 181;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        VISUAL_EFFECTS_MAN_CTOR.call(c, 0x002f_055c)?;
        c.g[V1][0] = sx(0x0048_7d28);
        c.g[A1][0] = sx(0x0048_7bb8);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
        c.g[A0][0] = 0x4;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, EA_MEM_ALLOC_NAMED, 0x002f_0580)?;
        for (vtable, name, field, ret) in VFX_MAN_PARTS_BEFORE_DETAIL {
            vfx_man_alloc_part(c, vtable, name, field, ret)?;
        }
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, DETAIL_SYSTEM_CTOR, 0x002f_0678)?;
        c.g[A1][0] = sx(0x0048_7c20);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.g[A0][0] = 0x4;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, EA_MEM_ALLOC_NAMED, 0x002f_0694)?;
        for (vtable, name, field, ret) in VFX_MAN_PARTS_AFTER_DETAIL {
            vfx_man_alloc_part(c, vtable, name, field, ret)?;
        }
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, PATH_ARROW_CTOR, 0x002f_0804)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `PS2VisualEffectsMan::~PS2VisualEffectsMan(int flags)` (0x2f0820):
/// installs the class vtable (`0x487d28`), deletes each of the 17 owned
/// objects at `this+0x00 .. +0x40` through its own vtable (the deleting
/// destructor in slot `+8`, `{delta, fn}` at `vt+8 / vt+0xc`, flag 3),
/// installs the base vtable (`0x487da0`) and, when `flags & 1`, frees
/// `this` with `__builtin_delete`. Frame 0x30 (ra 0, s1 0x10, s0 0x20).
pub fn ps2_visual_effects_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7d28));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(r32(c, S0).wrapping_add(VFX_MAN_VPTR), c.g[V0][0]);
        for (i, &ret) in VFX_MAN_DTOR_RETS.iter().enumerate() {
            let off = 4 * i as u32;
            c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(off));
            let owned = c.g[A2][0] != 0;
            if i == 0 {
                // Charged in the 11-cycle entry block; the delay slot keeps
                // the flags and `a1 = 3` is set only on the call path.
                c.g[S1][0] = c.g[A1][0];
                if owned {
                    c.cycles += 6;
                    c.g[A1][0] = 0x3;
                    vfx_man_dtor_delete_object(c, ret)?;
                }
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x3;
                if owned {
                    c.cycles += 5;
                    vfx_man_dtor_delete_object(c, ret)?;
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7da0));
        let keep = c.g[V1][0] == 0;
        c.sw(r32(c, S0).wrapping_add(VFX_MAN_VPTR), c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, VFX_MAN_DTOR_DELETE, 0x002f_0a80)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One owned-object delete of `PS2VisualEffectsMan_dtor`: the object in
/// `a2`, its vtable slot `+8` (`lh` delta, `lw` method), `this + delta`.
fn vfx_man_dtor_delete_object(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw(r32(c, A2));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x8));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    let method = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void *DLHeap_Realloc(DLHeap *heap, void *oldmem, unsigned int bytes)`
/// (0x31f018): dlmalloc `realloc`. A null `oldmem` is a plain malloc; a
/// too-large request or an mmapped chunk sets `ENOMEM` and returns null.
/// Otherwise (nb = the request as a chunk size) the chunk is kept when it
/// is big enough, grows into the top chunk or into a free next chunk
/// (unlinked), or is moved (malloc, copy, free; a malloc that returns the
/// next chunk is merged in place). A kept chunk gives back a tail of 16
/// bytes or more through `DLHeap_Free`. Frame 0x80 (ra 0, s6 0x10, s5
/// 0x20, s4 0x30, s3 0x40, s2 0x50, s1 0x60, s0 0x70).
///
/// Registers: s5 heap, s1 oldmem, s4 old chunk, s2 its size, s3 nb, a3
/// the state (`*heap`), s0 the next chunk, s6 the new chunk.
pub fn dlheap_realloc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        'restore: {
            'done: {
                'fail: {
                    'enomem: {
                        frame.save(c, S1, 0x60);
                        frame.save(c, S5, 0x20);
                        c.g[S1][0] = c.g[A1][0];
                        frame.save(c, S0, 0x70);
                        c.g[S5][0] = c.g[A0][0];
                        frame.save(c, S2, 0x50);
                        c.g[A1][0] = c.g[A2][0];
                        frame.save(c, S3, 0x40);
                        frame.save(c, S4, 0x30);
                        frame.save(c, S6, 0x10);
                        frame.save_ra(c, 0);
                        let have_old = c.g[S1][0] != 0;
                        c.g[A3][0] = c.lw(r32(c, S5));
                        if !have_old {
                            // realloc(0, n): malloc(heap, n).
                            c.cycles += 4;
                            DLHEAP_MALLOC.call(c, 0x0031_f058)?;
                            frame.restore(c, S0, 0x70);
                            break 'restore;
                        }
                        // 0x31f060: too large?
                        c.cycles += 5;
                        c.g[V0][0] = 0xffff_ffff_ffff_ffdf;
                        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                        let too_big = c.g[V0][0] != 0;
                        c.g[V1][0] = sx(r32(c, A1).wrapping_add(0x13));
                        if too_big {
                            break 'enomem;
                        }
                        // nb = request2size(bytes).
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
                        let tiny = c.g[V0][0] != 0;
                        c.g[S3][0] = 0x10;
                        if !tiny {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                            c.g[S3][0] = c.g[V1][0] & c.g[V0][0];
                        }
                        // 0x31f08c: the old chunk (s4) and its size (s2).
                        c.cycles += 7;
                        c.g[A2][0] = c.lw(r32(c, S1).wrapping_sub(0x4));
                        c.g[A0][0] = 0xffff_ffff_ffff_fffc;
                        c.g[S4][0] = sx(r32(c, S1).wrapping_sub(0x8));
                        c.g[V0][0] = c.g[A2][0] & 0x2;
                        let mmapped = c.g[V0][0] != 0;
                        c.g[S2][0] = c.g[A2][0] & c.g[A0][0];
                        if mmapped {
                            break 'enomem;
                        }
                        'split: {
                            // Big enough already?
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[S2][0] < c.g[S3][0]) as u64;
                            if c.g[V0][0] == 0 {
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S2][0];
                                break 'split;
                            }
                            c.g[V0][0] = c.lw(r32(c, A3).wrapping_add(0x30)); // av->top
                            'moved: {
                                'next_used: {
                                    // 0x31f0bc: the next chunk.
                                    c.cycles += 3;
                                    c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, S2)));
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x4));
                                    } else {
                                        // Next is top: grow into it if it leaves MINSIZE.
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x4));
                                        c.g[V1][0] = sx(r32(c, S3).wrapping_add(0x10));
                                        c.g[V0][0] &= c.g[A0][0];
                                        c.g[A1][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                                        c.g[V1][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
                                        let short = c.g[V1][0] != 0;
                                        c.g[A0][0] = c.g[S5][0];
                                        if short {
                                            break 'moved;
                                        }
                                        c.cycles += 10;
                                        c.g[V0][0] = c.g[A2][0] & 0x1;
                                        c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                                        c.g[V0][0] |= c.g[S3][0];
                                        c.g[V1][0] = sx(r32(c, A1).wrapping_sub(r32(c, S3)));
                                        c.sw(r32(c, S1).wrapping_sub(0x4), c.g[V0][0]);
                                        c.g[V1][0] |= 0x1;
                                        c.sw(r32(c, A3).wrapping_add(0x30), c.g[A0][0]);
                                        c.g[V0][0] = c.g[S1][0];
                                        c.sw(r32(c, A0).wrapping_add(0x4), c.g[V1][0]);
                                        break 'done;
                                    }
                                    // 0x31f10c: is the next chunk free (its next's PREV_INUSE)?
                                    c.cycles += 9;
                                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                                    c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
                                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                                    c.g[V1][0] ^= 0x1;
                                    c.g[V1][0] &= 0x1;
                                    let next_inuse = c.g[V1][0] == 0;
                                    c.g[V0][0] = c.g[A1][0] & c.g[A0][0];
                                    if next_inuse {
                                        break 'next_used;
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                                    c.g[V1][0] = (c.g[A1][0] < c.g[S3][0]) as u64;
                                    let short = c.g[V1][0] != 0;
                                    c.g[A0][0] = c.g[S5][0];
                                    if short {
                                        break 'moved;
                                    }
                                    // Absorb the free next chunk: unlink it.
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0xc));
                                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x8));
                                    c.sw(r32(c, V0).wrapping_add(0xc), c.g[V1][0]);
                                    c.sw(r32(c, V1).wrapping_add(0x8), c.g[V0][0]);
                                    break 'split;
                                }
                                // 0x31f154
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S5][0];
                            }
                            // 0x31f158: malloc(nb - 15), copy, free the old chunk.
                            c.cycles += 5;
                            c.g[A1][0] = sx(r32(c, S3).wrapping_sub(0xf));
                            DLHEAP_MALLOC.call(c, 0x0031_f160)?;
                            c.g[A3][0] = c.g[V0][0];
                            let failed = c.g[A3][0] == 0;
                            c.g[S6][0] = sx(r32(c, A3).wrapping_sub(0x8));
                            if failed {
                                break 'fail;
                            }
                            c.cycles += 5;
                            c.g[V1][0] = c.lw(r32(c, A3).wrapping_sub(0x4));
                            c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                            let adjacent = c.g[S6][0] == c.g[S0][0];
                            c.g[A1][0] = c.g[V1][0] & c.g[V0][0];
                            if adjacent {
                                // malloc returned the next chunk: merge in place.
                                c.cycles += 2;
                                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, S2)));
                                break 'split;
                            }
                            'free_old: {
                                'small_copy: {
                                    // 0x31f188: copy size - 4 bytes; under 10 words unrolled.
                                    c.cycles += 5;
                                    c.g[A2][0] = sx(r32(c, S2).wrapping_sub(0x4));
                                    c.g[A1][0] = sx(r32(c, A2) >> 2);
                                    c.g[V0][0] = (c.g[A1][0] < 0xa) as u64;
                                    if c.g[V0][0] != 0 {
                                        c.g[V1][0] = c.lw(r32(c, S1));
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[A3][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, DLHEAP_MEMCPY, 0x0031_f1a8)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        break 'free_old;
                                    }
                                    // 0x31f1b0: words 0-2.
                                    c.cycles += 7;
                                    c.g[A0][0] = (c.g[A1][0] < 0x5) as u64;
                                    c.sw(r32(c, A3), c.g[V1][0]);
                                    dlheap_realloc_copy_word(c, V0, 0x4);
                                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x8));
                                    let done = c.g[A0][0] != 0;
                                    c.sw(r32(c, A3).wrapping_add(0x8), c.g[V1][0]);
                                    if done {
                                        break 'small_copy;
                                    }
                                    // Words 3-4.
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0xc));
                                    c.g[A0][0] = (c.g[A1][0] < 0x7) as u64;
                                    c.sw(r32(c, A3).wrapping_add(0xc), c.g[V0][0]);
                                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x10));
                                    let done = c.g[A0][0] != 0;
                                    c.sw(r32(c, A3).wrapping_add(0x10), c.g[V1][0]);
                                    if done {
                                        break 'small_copy;
                                    }
                                    // Words 5-6.
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x14));
                                    c.g[A0][0] = (c.g[A1][0] < 0x9) as u64;
                                    c.sw(r32(c, A3).wrapping_add(0x14), c.g[V0][0]);
                                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x18));
                                    let done = c.g[A0][0] != 0;
                                    c.sw(r32(c, A3).wrapping_add(0x18), c.g[V1][0]);
                                    if done {
                                        break 'small_copy;
                                    }
                                    // Words 7-8.
                                    c.cycles += 4;
                                    dlheap_realloc_copy_word(c, V0, 0x1c);
                                    dlheap_realloc_copy_word(c, V1, 0x20);
                                }
                                // 0x31f20c
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S5][0];
                            }
                            // 0x31f210
                            c.cycles += 4;
                            c.g[A1][0] = c.g[S1][0];
                            DLHEAP_FREE.call(c, 0x0031_f218)?;
                            c.g[V0][0] = sx(r32(c, S6).wrapping_add(0x8));
                            break 'done;
                        }
                        // 0x31f220: keep the chunk (size a1); free a tail of MINSIZE or more.
                        'tail_done: {
                            c.cycles += 4;
                            c.g[A0][0] = sx(r32(c, A1).wrapping_sub(r32(c, S3)));
                            c.g[V0][0] = (c.g[A0][0] < 0x10) as u64;
                            let splits = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(0x4));
                            if !splits {
                                c.cycles += 8;
                                c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
                                c.g[V0][0] &= 0x1;
                                c.g[V0][0] |= c.g[A1][0];
                                c.sw(r32(c, S4).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x4));
                                c.g[V1][0] |= 0x1;
                                c.sw(r32(c, A0).wrapping_add(0x4), c.g[V1][0]);
                                break 'tail_done;
                            }
                            // 0x31f250
                            c.cycles += 13;
                            c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                            c.g[A3][0] = c.g[A0][0] | 0x1;
                            c.g[A2][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                            c.g[V0][0] &= 0x1;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V0][0] |= c.g[S3][0];
                            c.g[A1][0] = sx(r32(c, V1).wrapping_add(0x8));
                            c.sw(r32(c, S4).wrapping_add(0x4), c.g[V0][0]);
                            c.sw(r32(c, V1).wrapping_add(0x4), c.g[A3][0]);
                            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                            c.g[V0][0] |= 0x1;
                            c.sw(r32(c, A2).wrapping_add(0x4), c.g[V0][0]);
                            DLHEAP_FREE.call(c, 0x0031_f284)?;
                        }
                        // 0x31f284
                        c.cycles += 2;
                        c.g[V0][0] = sx(r32(c, S4).wrapping_add(0x8));
                        break 'done;
                    }
                    // 0x31f28c: errno = ENOMEM.
                    c.cycles += 4;
                    call_ptr(c, DLHEAP_GETREENT, 0x0031_f294)?;
                    c.g[V1][0] = DLHEAP_ENOMEM;
                    c.sw(r32(c, V0), c.g[V1][0]);
                }
                // 0x31f29c
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            // 0x31f2a0
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        // 0x31f2a4
        c.cycles += 9;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One word of `DLHeap_Realloc`'s unrolled small copy: `*(a3 + off) =
/// *(s1 + off)` through register `via`.
fn dlheap_realloc_copy_word(c: &mut Cpu, via: usize, off: u32) {
    c.g[via][0] = c.lw(r32(c, S1).wrapping_add(off));
    c.sw(r32(c, A3).wrapping_add(off), c.g[via][0]);
}

/// `void **DLHeap_IAlloc(DLHeap *heap, unsigned n, unsigned *sizes, int
/// opts, void **chunks)` (0x31f858): dlmalloc `ialloc`, the body of
/// independent_calloc / comalloc. Consolidates an uninitialised state, then
/// allocates one chunk holding `n` elements (`opts & 1`: all of
/// `sizes[0]`, else `sizes[i]`) plus, without a caller array, the pointer
/// array; with mmap disabled around the malloc (`av+0x354`). `opts & 2`
/// zeroes the contents. The chunk is carved into the elements, whose
/// pointers go to the array. `chunks` with `n == 0` returns `chunks`; no
/// array and `n == 0` is `malloc(heap, 0)`. Frame 0xb0 (opts at 0, ra
/// 0x10, fp 0x20, s7..s0 0x30..0xa0).
///
/// Resumable: its two loops deliver events at back edges and unwind with
/// `pc` at the head (0x31f968, 0x31fa10); a re-entry there runs the
/// generated body, which picks the loop up from the registers.
pub fn dlheap_ialloc(c: &mut Cpu) {
    if c.pc != 0x0031_f858 {
        return ee::s10::f_0031f858__generated(c);
    }
    use callee::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[T0][0]; // chunks
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A0][0]; // heap
        frame.save(c, FP, 0x20);
        c.g[S3][0] = c.g[A1][0]; // n
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x10);
        frame.save(c, S1, 0x90);
        c.g[S1][0] = c.lw(r32(c, S2)); // av
        c.sw(frame.at(c, 0), c.g[A3][0]); // opts
        c.g[V0][0] = c.lw(r32(c, S1)); // av->max_fast
        let ready = c.g[V0][0] != 0;
        c.g[FP][0] = c.g[A2][0]; // sizes
        if !ready {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            DLHEAP_CONSOLIDATE.call(c, 0x0031_f8ac)?;
        }
        'restore: {
            'done: {
                // 0x31f8ac
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    // 0x31f8b4: the caller's array (s5), no array chunk (s7 = 0).
                    c.cycles += 2;
                    let have = c.g[S3][0] != 0;
                    c.g[S5][0] = c.g[S0][0];
                    if !have {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S0][0];
                        break 'done;
                    }
                    c.cycles += 2;
                    c.g[S7][0] = 0;
                } else {
                    // 0x31f8cc: the array chunk size request2size(n * 4) (s7).
                    c.cycles += 2;
                    let have = c.g[S3][0] != 0;
                    c.g[V1][0] = sx(r32(c, S3) << 2);
                    if !have {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 0;
                        DLHEAP_MALLOC.call(c, 0x0031_f8e0)?;
                        frame.restore(c, S0, 0xa0);
                        break 'restore;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x13));
                    c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
                    let tiny = c.g[V0][0] != 0;
                    c.g[S5][0] = 0;
                    if tiny {
                        c.cycles += 1;
                        c.g[S7][0] = 0x10;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                        c.g[S7][0] = c.g[V1][0] & c.g[V0][0];
                    }
                }
                // 0x31f90c: the contents size (s0); s4 = the element size if fixed.
                c.cycles += 4;
                c.g[V1][0] = c.lw(frame.at(c, 0));
                c.g[V0][0] = c.g[V1][0] & 0x1;
                let same_size = c.g[V0][0] != 0;
                c.g[S4][0] = 0;
                if same_size {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, FP));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x13));
                    c.g[V1][0] = (c.g[A0][0] < 0x10) as u64;
                    let tiny = c.g[V1][0] != 0;
                    c.g[S4][0] = 0x10;
                    if !tiny {
                        c.cycles += 3;
                        c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                        c.g[S4][0] = c.g[A0][0] & c.g[V0][0];
                    }
                    // 0x31f93c: s0 = n * element size.
                    c.cycles += 3;
                    c.mult(0, S0, S3, S4);
                    c.g[S6][0] = c.lw(r32(c, S1).wrapping_add(0x354));
                } else {
                    // 0x31f948: s0 = sum of request2size(sizes[i]).
                    c.cycles += 4;
                    c.g[S0][0] = 0;
                    c.g[A2][0] = 0;
                    let none = c.g[S3][0] == 0;
                    c.g[S6][0] = c.lw(r32(c, S1).wrapping_add(0x354));
                    if !none {
                        c.cycles += 4;
                        c.g[A1][0] = 0xffff_ffff_ffff_fff0;
                        c.g[A0][0] = c.g[FP][0];
                        loop {
                            // 0x31f968
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(r32(c, A0));
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x13));
                            c.g[V1][0] = (c.g[V0][0] < 0x10) as u64;
                            if c.g[V1][0] != 0 {
                                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10));
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] &= c.g[A1][0];
                                c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                            }
                            c.cycles += 3;
                            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x1));
                            let more = c.g[A2][0] != c.g[S3][0];
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4));
                            if !more {
                                break;
                            }
                            if back_edge_unwinds(c, 0x0031_f968) {
                                return Ok(());
                            }
                        }
                    }
                }
                // 0x31f990: one chunk, mmap disabled around the malloc.
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, S0).wrapping_add(r32(c, S7)));
                c.sw(r32(c, S1).wrapping_add(0x354), 0);
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xf));
                c.g[A0][0] = c.g[S2][0];
                DLHEAP_MALLOC.call(c, 0x0031_f9a4)?;
                c.g[A1][0] = c.g[V0][0];
                c.sw(r32(c, S1).wrapping_add(0x354), c.g[S6][0]);
                let failed = c.g[A1][0] == 0;
                c.g[V0][0] = 0;
                if failed {
                    break 'done;
                }
                // 0x31f9b4: s1 = the chunk, s2 = its size.
                c.cycles += 8;
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_sub(0x4));
                c.g[V1][0] = 0xffff_ffff_ffff_fffc;
                c.g[A2][0] = c.lw(frame.at(c, 0));
                c.g[S2][0] = c.g[V0][0] & c.g[V1][0];
                c.g[A0][0] = c.g[A2][0] & 0x2;
                let clear = c.g[A0][0] != 0;
                c.g[S1][0] = sx(r32(c, A1).wrapping_sub(0x8));
                if clear {
                    c.cycles += 5;
                    c.g[A2][0] = sx(r32(c, S7).wrapping_add(0x4));
                    c.g[A0][0] = c.g[A1][0];
                    c.g[A2][0] = sx(r32(c, S2).wrapping_sub(r32(c, A2)));
                    c.g[A1][0] = 0;
                    call_ptr(c, DLHEAP_MEMSET, 0x0031_f9e8)?;
                }
                // 0x31f9e8: no caller array: the array is the chunk's tail.
                c.cycles += 2;
                let have_array = c.g[S5][0] != 0;
                c.g[A2][0] = 0;
                if !have_array {
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, S2).wrapping_sub(r32(c, S0)));
                    c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                    c.g[V0][0] |= 0x1;
                    c.g[S2][0] = c.g[S0][0];
                    c.sw(r32(c, V1).wrapping_add(0x4), c.g[V0][0]);
                    c.g[S5][0] = sx(r32(c, V1).wrapping_add(0x8));
                }
                // 0x31fa08
                c.cycles += 2;
                c.g[A1][0] = sx(r32(c, S3).wrapping_sub(0x1));
                loop {
                    // 0x31fa14: array[i] = the element's memory.
                    c.cycles += 5;
                    c.g[A0][0] = sx(r32(c, A2) << 2);
                    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    let last = c.g[A2][0] == c.g[A1][0];
                    c.sw(r32(c, V0), c.g[V1][0]);
                    if last {
                        break;
                    }
                    // 0x31fa28: the element's chunk size.
                    c.cycles += 2;
                    let fixed = c.g[S4][0] != 0;
                    c.g[V1][0] = c.g[S4][0];
                    if !fixed {
                        c.cycles += 6;
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, FP)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x13));
                        c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = 0x10;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                            c.g[V1][0] &= c.g[V0][0];
                        }
                    }
                    // 0x31fa54: carve it.
                    c.cycles += 5;
                    c.g[V0][0] = c.g[V1][0] | 0x1;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V1)));
                    c.sw(r32(c, S1).wrapping_add(0x4), c.g[V0][0]);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                    if back_edge_unwinds(c, 0x0031_fa10) {
                        return Ok(());
                    }
                    // 0x31fa10
                    c.cycles += 1;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x1));
                }
                // 0x31fa68: the last element takes the rest.
                c.cycles += 3;
                c.g[V0][0] = c.g[S2][0] | 0x1;
                c.sw(r32(c, S1).wrapping_add(0x4), c.g[V0][0]);
                c.g[V0][0] = c.g[S5][0];
            }
            // 0x31fa74
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
        }
        // 0x31fa78
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_MemalignOffset(DLHeap *heap, unsigned align, unsigned
/// bytes, unsigned offset)` (0x31f4e8): EA's `memalign` variant that
/// aligns `mem + offset` (not `mem`). An alignment of 16 or less is a
/// plain malloc of `bytes`; otherwise the alignment is raised to a power
/// of two (at least 0x20 when not one already), `bytes + offset` padded to
/// a chunk size (`ENOMEM` and null when too large), `nb + align + 16`
/// allocated, the misaligned lead given back (free chunk, or the mmapped
/// chunk's offset moved) and a tail of more than 16 bytes freed. Frame
/// 0x60 (ra 0, s4 0x10, s3 0x20, s2 0x30, s1 0x40, s0 0x50).
pub fn dlheap_memalign_offset(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0]; // align
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[A1][0] = c.g[A2][0]; // bytes
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A0][0]; // heap
        frame.save(c, S2, 0x30);
        c.g[S3][0] = c.g[A3][0]; // offset
        c.g[V0][0] = (c.g[S0][0] < 0x11) as u64;
        let small = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        'restore: {
            'done: {
                if small {
                    c.cycles += 4;
                    DLHEAP_MALLOC.call(c, 0x0031_f524)?;
                    frame.restore(c, S0, 0x50);
                    break 'restore;
                }
                // 0x31f52c: s0 = the alignment as a power of two.
                c.cycles += 8;
                c.g[V0][0] = 0xf;
                c.g[A0][0] = 0x10;
                c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A0][0];
                }
                c.g[V1][0] = sx(r32(c, S0).wrapping_sub(0x1));
                c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
                let pow2 = c.g[V1][0] == 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, S3))); // bytes + offset
                if !pow2 {
                    c.cycles += 4;
                    c.g[V1][0] = 0x20;
                    c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = sx(r32(c, V1) << 1);
                        loop {
                            // 0x31f560
                            c.cycles += 7;
                            c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                            if c.g[V0][0] == 0 {
                                c.cycles += 1;
                                c.g[S0][0] = c.g[V1][0];
                                break;
                            }
                            c.g[V1][0] = sx(r32(c, V1) << 1);
                            if event_due(c) {
                                deliver_event_here(c, 0x0031_f560);
                            }
                        }
                    }
                }
                // 0x31f580: too large?
                c.cycles += 5;
                c.g[V0][0] = 0xffff_ffff_ffff_ffdf;
                c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                let too_big = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(0x13));
                if too_big {
                    c.cycles += 6;
                    call_ptr(c, DLHEAP_GETREENT, 0x0031_f59c)?;
                    c.g[V1][0] = DLHEAP_ENOMEM;
                    c.sw(r32(c, V0), c.g[V1][0]);
                    c.g[V0][0] = 0;
                    break 'done;
                }
                // 0x31f5ac: s2 = request2size(bytes + offset).
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
                let tiny = c.g[V0][0] != 0;
                c.g[S2][0] = 0x10;
                if !tiny {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                    c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
                }
                // 0x31f5c4: malloc(nb + align + 16); t0 = mem, v1 = mem + offset.
                c.cycles += 7;
                c.g[A1][0] = sx(r32(c, S2).wrapping_add(r32(c, S0)));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
                DLHEAP_MALLOC.call(c, 0x0031_f5d4)?;
                c.g[T0][0] = c.g[V0][0];
                let got = c.g[T0][0] != 0;
                c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, S3)));
                if !got {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'done;
                }
                // 0x31f5e8: the divide-by-zero trap test (no trap: s0 >= 0x20).
                c.cycles += 2;
                // 0x31f5f0: misalignment of mem + offset.
                c.cycles += 4;
                c.divu(0, V1, S0);
                c.g[V0][0] = c.hi[0];
                let misaligned = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, T0).wrapping_sub(0x8));
                if misaligned {
                    // s0 = the aligned chunk (brk, or brk + align when the
                    // lead would be under MINSIZE), a3 = lead, a0 = new size.
                    c.cycles += 19;
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
                    c.g[A2][0] = 0xf;
                    c.g[V1][0] = sx(0u32.wrapping_sub(r32(c, S0)));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[A1][0] = c.lw(r32(c, T0).wrapping_sub(0x4));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S3)));
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x8));
                    c.g[V1][0] |= 0xfffc;
                    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S1)));
                    c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.g[A2][0] = (c.g[A2][0] < c.g[A0][0]) as u64;
                    c.g[V1][0] = c.g[A1][0] & c.g[V1][0];
                    if c.g[A2][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A1][0] &= 0x2;
                    c.g[A3][0] = sx(r32(c, S0).wrapping_sub(r32(c, S1)));
                    let mmapped = c.g[A1][0] != 0;
                    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
                    if mmapped {
                        // Move the mmapped chunk's offset forward.
                        c.cycles += 7;
                        c.g[V1][0] = c.lw(r32(c, T0).wrapping_sub(0x8));
                        c.g[V0][0] = c.g[A0][0] | 0x2;
                        c.sw(r32(c, S0).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
                        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x8));
                        c.sw(r32(c, S0), c.g[V1][0]);
                        break 'done;
                    }
                    // 0x31f668: give the lead back as a free chunk.
                    c.cycles += 14;
                    c.g[V0][0] = c.g[A0][0] | 0x1;
                    c.g[A2][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    c.sw(r32(c, S0).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A1][0] = c.g[T0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[S1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                    c.g[V0][0] |= 0x1;
                    c.sw(r32(c, A2).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw(r32(c, T0).wrapping_sub(0x4));
                    c.g[V1][0] &= 0x1;
                    c.g[V1][0] |= c.g[A3][0];
                    c.sw(r32(c, T0).wrapping_sub(0x4), c.g[V1][0]);
                    DLHEAP_FREE.call(c, 0x0031_f6a0)?;
                }
                // 0x31f6a0: s1 = the aligned chunk.
                c.cycles += 4;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                c.g[V0][0] = c.g[A0][0] & 0x2;
                let mmapped = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x8));
                if mmapped {
                    break 'done;
                }
                // Free a tail of more than MINSIZE.
                c.cycles += 7;
                c.g[V1][0] = sx(r32(c, S2).wrapping_add(0x10));
                c.g[V0][0] = c.g[A0][0] & 0xffff_ffff_ffff_fffc;
                c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                let split = c.g[V1][0] != 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S2)));
                if split {
                    c.cycles += 10;
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
                    c.g[V0][0] |= 0x1;
                    c.g[A0][0] = c.g[S4][0];
                    c.sw(r32(c, A1).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4));
                    c.g[V0][0] &= 0x1;
                    c.g[V0][0] |= c.g[S2][0];
                    c.sw(r32(c, S1).wrapping_add(0x4), c.g[V0][0]);
                    DLHEAP_FREE.call(c, 0x0031_f6f4)?;
                }
                // 0x31f6f4
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x8));
            }
            // 0x31f6f8
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        // 0x31f6fc
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `struct mallinfo DLHeap_Mallinfo(DLHeap *heap)` (0x31fc20; `a0` the
/// result struct, `a1` the heap): dlmalloc `mallinfo`. Consolidates an
/// uninitialised state (no top), counts the fast bins (blocks, bytes) and
/// the 95 regular bins (walked through `bk`, stopping on a self-linked
/// chunk), then writes the ten words `{arena, ordblks, smblks, hblks,
/// hblkhd, usmblks, fsmblks, uordblks, fordblks, keepcost}` through the
/// stack (unaligned `sdl/sdr` pairs). Returns the struct pointer. Frame
/// 0x60 (the struct at 0..0x28, ra 0x30, s1 0x40, s0 0x50).
pub fn dlheap_mallinfo(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x30);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.lw(r32(c, A1)); // av
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x30)); // av->top
        let have_top = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A0][0]; // the result
        if !have_top {
            c.cycles += 4;
            c.g[A0][0] = c.g[A1][0];
            c.g[A1][0] = c.g[S0][0];
            DLHEAP_CONSOLIDATE.call(c, 0x0031_fc4c)?;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x30));
        }
        // 0x31fc50: t0 avail, t2 nblocks, t4 nfastblocks, t5 fastavail.
        c.cycles += 10;
        c.g[V1][0] = 0xffff_ffff_ffff_fffc;
        c.g[T2][0] = 0x1;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x4));
        c.g[T4][0] = 0;
        c.g[T5][0] = 0;
        c.g[A2][0] = 0;
        c.g[T0][0] = c.g[A0][0] & c.g[V1][0];
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x4));
        loop {
            // 0x31fc78: fast bin a2 (a0 = &av->fastbins[a2]).
            c.cycles += 3;
            c.g[A1][0] = c.lw(r32(c, A0));
            if c.g[A1][0] == 0 {
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x1));
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0xffff_ffff_ffff_fffc;
                loop {
                    // 0x31fc90
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x4));
                    c.g[T4][0] = sx(r32(c, T4).wrapping_add(0x1));
                    c.g[A1][0] = c.lw(r32(c, A1).wrapping_add(0x8));
                    c.g[V0][0] &= c.g[V1][0];
                    let more = c.g[A1][0] != 0;
                    c.g[T5][0] = sx(r32(c, T5).wrapping_add(r32(c, V0)));
                    if !more {
                        c.cycles += 1;
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x1));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_fc90);
                    }
                }
            }
            // 0x31fcb0
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 0xb) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_fc78);
            }
        }
        // The regular bins 1..95 (t1 the size mask, a2 the index).
        c.cycles += 5;
        c.g[T1][0] = 0xffff_ffff_ffff_0000;
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, T5)));
        c.g[A2][0] = 0x1;
        c.g[T1][0] |= 0xfffc;
        c.g[V0][0] = sx(r32(c, A2) << 3);
        loop {
            'next_bin: {
                'bin_done: {
                    // 0x31fcd0: a3 = the bin header, a1 = its last chunk.
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x38));
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                    c.g[A3][0] = sx(r32(c, V0).wrapping_sub(0x8));
                    let empty = c.g[A1][0] == c.g[A3][0];
                    c.g[T3][0] = sx(r32(c, A2).wrapping_add(0x1));
                    if empty {
                        break 'bin_done;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0xc));
                    c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x1));
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x4));
                    c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V1][0] &= c.g[T1][0];
                    let looped = c.g[A1][0] == c.g[A0][0];
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                    if looped {
                        break 'bin_done;
                    }
                    // p == p->bk cannot hold here (p->bk->bk would be p,
                    // caught above).
                    c.cycles += 2;
                    if c.g[A1][0] != c.g[A2][0] {
                        c.g[A1][0] = c.g[A2][0];
                    } else {
                        c.cycles += 2;
                        if c.g[A1][0] != c.g[A3][0] {
                            c.g[A2][0] = c.g[T3][0];
                            break 'next_bin;
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.g[A2][0];
                    }
                    loop {
                        // 0x31fd20
                        c.cycles += 2;
                        let at_header = c.g[A1][0] == c.g[A3][0];
                        c.g[A2][0] = c.g[T3][0];
                        if at_header {
                            break 'next_bin;
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(0xc));
                        c.g[T2][0] = sx(r32(c, T2).wrapping_add(0x1));
                        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x4));
                        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0xc));
                        c.g[A2][0] = c.g[A0][0];
                        c.g[V0][0] &= c.g[T1][0];
                        let looped = c.g[A1][0] == c.g[V1][0];
                        c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
                        if looped {
                            break 'bin_done;
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != c.g[A0][0] {
                            c.g[A1][0] = c.g[A2][0];
                        } else {
                            c.cycles += 2;
                            let at_header = c.g[A1][0] == c.g[A3][0];
                            c.g[A1][0] = c.g[A2][0];
                            if !at_header {
                                break 'bin_done;
                            }
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_fd20);
                        }
                    }
                }
                // 0x31fd58
                c.cycles += 1;
                c.g[A2][0] = c.g[T3][0];
            }
            // 0x31fd5c
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 96i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, A2) << 3);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_fcd0);
            }
        }
        // The struct on the stack, then copied out.
        c.cycles += 47;
        c.sw(frame.at(c, 0x8), c.g[T4][0]);
        c.g[A2][0] = 0xffff_ffff_ffff_0000;
        c.sw(frame.at(c, 0x4), c.g[T2][0]);
        c.g[A2][0] |= 0xfffc;
        c.sw(frame.at(c, 0x20), c.g[T0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore_ra(c, 0x30);
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x368));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
        c.sw(frame.at(c, 0x1c), c.g[V1][0]);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x368));
        c.sw(frame.at(c, 0), c.g[A0][0]);
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x350));
        c.sw(frame.at(c, 0xc), c.g[V1][0]);
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x364));
        c.sw(frame.at(c, 0x18), c.g[T5][0]);
        c.sw(frame.at(c, 0x10), c.g[A1][0]);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x30));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x4));
        c.g[V1][0] &= c.g[A2][0];
        c.sw(frame.at(c, 0x24), c.g[V1][0]);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x374));
        frame.restore(c, S0, 0x50);
        c.sw(frame.at(c, 0x14), c.g[A0][0]);
        for (reg, off) in [(V1, 0x0), (A0, 0x8), (A1, 0x10), (A2, 0x18)] {
            c.ldl(reg, frame.at(c, off + 0x7));
            c.ldr(reg, frame.at(c, off));
        }
        for (reg, off) in [(V1, 0x0u32), (A0, 0x8), (A1, 0x10), (A2, 0x18)] {
            c.sdl(reg, r32(c, S1).wrapping_add(off + 0x7));
            c.sdr(reg, r32(c, S1).wrapping_add(off));
        }
        c.ldl(V1, frame.at(c, 0x27));
        c.ldr(V1, frame.at(c, 0x20));
        c.sdl(V1, r32(c, S1).wrapping_add(0x27));
        c.sdr(V1, r32(c, S1).wrapping_add(0x20));
        frame.restore(c, S1, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_4410, vd_texture_sink_ctor, "VD_TextureSink_ctor"),
    (
        0x0025_48d0,
        vd_texture_sink_draw_frame,
        "VD_TextureSink_DrawFrame",
    ),
    (
        0x0038_c590,
        ps2_bezier_man_debug_draw_grid,
        "PS2BezierMan_DebugDrawGrid",
    ),
    (
        0x003b_0c58,
        ps2_sony_codec_internal_open,
        "PS2_SONY_CODEC_INTERNAL_Open",
    ),
    (
        0x002f_0548,
        ps2_visual_effects_man_ctor,
        "PS2VisualEffectsMan_ctor",
    ),
    (
        0x002f_0820,
        ps2_visual_effects_man_dtor,
        "PS2VisualEffectsMan_dtor",
    ),
    (0x0031_f018, dlheap_realloc, "DLHeap_Realloc"),
    (0x0031_f858, dlheap_ialloc, "DLHeap_IAlloc"),
    (0x0031_f4e8, dlheap_memalign_offset, "DLHeap_MemalignOffset"),
    (0x0031_fc20, dlheap_mallinfo, "DLHeap_Mallinfo"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{
        check_paths, check_paths_lohi, elf_image, fresh, run_generated, Rng, DATA, GAME_GP,
    };
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ---------- VD_TextureSink (ctor, DrawFrame) ----------

    /// The `miss` hook at a fixed 4-cycle cost (the fake's hash-based cost
    /// would make path totals seed-varying).
    fn vdts_norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    pub(crate) const VDTS_SINK: u32 = DATA + 0x5_0000;
    pub(crate) const VDTS_DESC: u32 = DATA + 0x5_0200;
    pub(crate) const VDTS_HDR: u32 = DATA + 0x5_0300;
    pub(crate) const VDTS_GM: u32 = DATA + 0x5_1000;
    pub(crate) const VDTS_VT: u32 = DATA + 0x5_8000;
    pub(crate) const VDTS_MODES: u32 = DATA + 0x5_9000;
    pub(crate) const VDTS_TEXMAN: u32 = DATA + 0x5_a000;
    pub(crate) const VDTS_TEXREC: u32 = DATA + 0x5_b000;
    pub(crate) const VDTS_ITEMS: u32 = DATA + 0x5_c000;
    pub(crate) const VDTS_CELLS: u32 = DATA + 0x5_c100;
    pub(crate) const VDTS_SCRATCH: u32 = DATA + 0x5_d000;
    /// `this` delta of the real-method slots (0x50, 0x58, 0x88).
    pub(crate) const VDTS_GETTER_DELTA: u32 = 0x20;

    /// Gate of the `IsRenderIdle` fake (`DrawFrame`'s wait loop) and the
    /// number of "busy" answers it gives before "idle".
    pub(crate) const VDTS_DRAW_CTRL: u32 = DATA + 0x7_f000;
    pub(crate) const VDTS_DRAW_MAGIC: u32 = 0x1d1e_5eed;
    pub(crate) const VDTS_DRAW_BUSY: u32 = DATA + 0x7_f004;
    static VDTS_DRAW_PREV_IDLE: AtomicUsize = AtomicUsize::new(0);

    /// `PS2GraphicsMan_IsRenderIdle` (0x382730) with its gate on: logs the
    /// call, costs 1000 cycles (so the retry shows in the path total), and
    /// answers 0 while the busy count lasts, then 1.
    fn vdts_draw_fake_idle(c: &mut Cpu) {
        if c.bus.read32(VDTS_DRAW_CTRL) != VDTS_DRAW_MAGIC {
            return fakes::fall_back(c, &VDTS_DRAW_PREV_IDLE, ee::s12::f_00382730__generated);
        }
        let before = c.cycles;
        fakes::fake_call(c, 0x0038_2730);
        c.cycles = before + 1000;
        let busy = c.bus.read32(VDTS_DRAW_BUSY);
        if busy != 0 {
            c.bus.write32(VDTS_DRAW_BUSY, busy - 1);
        }
        c.g[V0][0] = (busy == 0) as u64;
    }

    pub(crate) fn vdts_draw_hook() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00382730),
            vdts_draw_fake_idle,
            &VDTS_DRAW_PREV_IDLE,
        );
    }

    /// The graphics manager both sink functions talk to (also at
    /// `gp + 0x2a90`): every vtable slot is `FAKE` except the real screen
    /// getters (`GetScreenWidth`/`Height`, mode record 1 holding `width` /
    /// `height`) and `IsRenderIdle`, all with a `this` delta; a texture
    /// manager whose 16 slots point at distinctive records; the render-state
    /// scratch at `+0xe84`.
    pub(crate) fn vdts_graphics_man(c: &mut Cpu, seed: u32, width: u32, height: u32) {
        for i in 0..0x40u32 {
            c.bus.write32(VDTS_GM + 4 * i, 0x3333_0000 + i);
        }
        c.bus.write32(VDTS_GM + 0x10d8, VDTS_VT);
        c.bus.write32(VDTS_GM + 0x18f4, VDTS_TEXMAN);
        c.bus.write32(VDTS_GM + 0xe84, VDTS_SCRATCH);
        let this = VDTS_GM + VDTS_GETTER_DELTA;
        c.bus.write32(this + 0x59c8, 1);
        c.bus.write32(this + 0x59cc, VDTS_MODES);
        c.bus.write32(VDTS_MODES + 0xa4 + 0x88, width);
        c.bus.write32(VDTS_MODES + 0xa4 + 0x8c, height);
        for i in 0..0xa0u32 {
            c.bus.write32(VDTS_VT + 4 * i, FAKE);
        }
        for (slot, f) in [
            (0x50u32, 0x0039_5330u32),
            (0x58, 0x0039_5350),
            (0x88, 0x0038_2730),
        ] {
            c.bus.write16(VDTS_VT + slot, VDTS_GETTER_DELTA as u16);
            c.bus.write32(VDTS_VT + slot + 4, f);
        }
        for i in 0..16u32 {
            c.bus
                .write32(VDTS_TEXMAN + 8 + 4 * i, VDTS_TEXREC + 0x80 * i);
            c.bus.write64(
                VDTS_TEXREC + 0x80 * i + 0x40,
                0x1234_5678_9abc_def0 ^ ((i as u64) << 8) ^ seed as u64,
            );
        }
        for i in 0..8u32 {
            c.bus.write32(VDTS_SCRATCH + 4 * i, 0x6666_0000 + seed + i);
        }
        c.bus.write32(GAME_GP + 0x2a90, VDTS_GM);
    }

    /// The screen size word for a seed bit: positive, or with bit 31 set
    /// (gcc's unsigned-to-float arm).
    fn vdts_screen_word(base: u32, negative: bool) -> u32 {
        if negative {
            0x8000_0000 | base
        } else {
            base
        }
    }

    /// `VD_TextureSink_ctor` fixture. Seed bits: 0 = header flag 0x1000
    /// (0x25447c arm), 1 = screen width bit 31 (0x2546dc), 2 = screen height
    /// bit 31 (0x254720); `seed / 8 % 9` picks (width, height) of the image:
    /// strips p = popcount(height), columns = width / 16 (<= 0 skips to
    /// 0x2546a8; the last shape is a negative width, the `movn` rounding).
    /// At most 3 cells, so every fake result is forced: call 0 is the cell
    /// array, odd calls cell records, even calls texture slots 3/5/7.
    pub(crate) fn vdts_ctor_fixture(c: &mut Cpu, seed: u32) {
        const SHAPES: [(i16, i16); 9] = [
            (8, 4),
            (8, 6),
            (8, 7),
            (16, 4),
            (20, 6),
            (31, 7),
            (32, 2),
            (48, 1),
            (-20, 5),
        ];
        let flag = seed % 2 == 1;
        let (w, h) = SHAPES[(seed / 8) as usize % SHAPES.len()];
        vdts_graphics_man(
            c,
            seed,
            vdts_screen_word(0x280 + seed, (seed / 2) % 2 == 1),
            vdts_screen_word(0x1c0 + seed, (seed / 4) % 2 == 1),
        );
        for i in 0..0x20u32 {
            c.bus.write32(VDTS_SINK + 4 * i, 0x2222_0000 + i);
        }
        c.bus.write32(VDTS_DESC, 0x4242_0000 + seed);
        c.bus.write32(VDTS_DESC + 4, VDTS_HDR);
        for i in 0..8u32 {
            c.bus.write32(VDTS_HDR + 4 * i, 0x5555_0000 + i);
        }
        c.bus.write16(VDTS_HDR + 4, w as u16);
        c.bus.write16(VDTS_HDR + 6, h as u16);
        c.bus
            .write32(VDTS_HDR + 0xc, if flag { 0x0000_1003 } else { 0x0000_0e03 });
        c.bus.write32(VDTS_HDR + 0x10, 0x40 + 0x10 * (seed % 5));
        // The video allocator hook (alloc through the fake) and its flags.
        c.bus.write32(0x0050_9430, FAKE);
        c.bus.write32(0x0050_9438, 0x0bad_0000 | seed);
        c.bus.write32(LOG_RESULTS, VDTS_ITEMS);
        for k in 0..3u32 {
            c.bus
                .write32(LOG_RESULTS + 4 * (1 + 2 * k), VDTS_CELLS + 0x20 * k);
            c.bus.write32(LOG_RESULTS + 4 * (2 + 2 * k), 3 + 2 * k);
        }
        c.bus.write32(LOG_RESULTS + 4 * 7, VDTS_CELLS + 0x60);
        for i in 0..0x20u32 {
            c.bus.write32(VDTS_ITEMS + 4 * i, 0x7a7a_0000 + i);
            c.bus.write32(VDTS_CELLS + 4 * i, 0x7b7b_0000 + i);
        }
        c.g[A0][0] = VDTS_SINK as u64;
        c.g[A1][0] = 0x1111_0000 + seed as u64;
        c.g[A2][0] = VDTS_DESC as u64;
        c.g[A3][0] = VDTS_GM as u64;
        c.g[T0][0] = 0x7777_0000 + seed as u64;
        c.pc = 0x0025_4410;
    }

    /// `VD_TextureSink_ctor`: every strip / column / flag / sign arm, with
    /// the real `VD_FloorLog2`, `SetTextureFilter` (+ `TexMan_SetTex1Filter`),
    /// screen getters, `SetFrameSize` / `SetFrameRate`; allocations and
    /// texture creation through the fake. Path totals (f = flag, n = negative
    /// getter results, p = strips, k = columns):
    /// `143 + 22p + 2f + 2n + [k > 0](2 + k(15 + 105p))`.
    #[test]
    fn vd_texture_sink_ctor_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_4410, 0);
            elf_image(c);
            c.hooks.miss = vdts_norm_miss;
            vdts_ctor_fixture(c, seed);
        };
        check_paths(
            ee::s06::f_00254410__generated,
            vd_texture_sink_ctor,
            72,
            // 165 p=1 k=0 (all non-negative, flag clear); 171 = f=1 n=2 (seed 7:
            // 0x25447c + 0x2546dc + 0x254720); 187 p=2; 209 p=3; 287 p=1 k=1;
            // 414 p=2 k=1; 541 p=3 k=1; 407 p=1 k=2; 527 p=1 k=3.
            &[165, 171, 187, 209, 287, 407, 414, 527, 541],
            &setup,
        );
        fakes::covered(
            ee::s06::f_00254410__generated,
            vd_texture_sink_ctor,
            72,
            &|c, seed| {
                machine(c, seed, 0x0025_4411, 8);
                elf_image(c);
                c.hooks.miss = vdts_norm_miss;
                vdts_ctor_fixture(c, seed);
            },
        );
    }

    driven!(vdts_ctor_gen, ee::s06::f_00254410__generated);

    driven!(vdts_ctor_hand, vd_texture_sink_ctor);

    /// The ctor's three back edges (0x2544c8 strips, 0x2545a8 cells,
    /// 0x254580 columns) under delivered and unwinding events: shapes with
    /// p >= 2 or more than one column only.
    #[test]
    fn vd_texture_sink_ctor_events() {
        check_events(vdts_ctor_gen, vdts_ctor_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x0025_4412, 0);
            elf_image(c);
            c.hooks.miss = vdts_norm_miss;
            // Shapes 4..7 of the fixture (seed / 8 % 9 in 4..=7).
            let shape_seed = (4 + seed % 4) * 8 + (seed / 4) % 8;
            vdts_ctor_fixture(c, shape_seed);
            schedule(c, &mut r, seed, 0x0025_4410)
        });
    }

    /// `VD_TextureSink_DrawFrame` fixture. Seed bits: 0 = one "busy" answer
    /// before idle (the 0x254918 retry), 1 = screen width bit 31 (0x254960
    /// arm), 2 = screen height bit 31 (0x2549a4 arm); `seed / 8 % 4` rows
    /// (`+0x1c`, 0 skips 0x254af0), `seed / 32 % 3` cells per row (`+0x20`,
    /// 0 skips 0x254b28).
    pub(crate) fn vdts_draw_fixture(c: &mut Cpu, seed: u32) {
        let rows = (seed / 8) % 4;
        let cols = (seed / 32) % 3;
        vdts_graphics_man(
            c,
            seed,
            vdts_screen_word(0x280 + seed, (seed / 2) % 2 == 1),
            vdts_screen_word(0x1c0 + seed, (seed / 4) % 2 == 1),
        );
        for i in 0..0x20u32 {
            c.bus.write32(VDTS_SINK + 4 * i, 0x2222_0000 + i);
        }
        c.bus.write32(VDTS_SINK + 8, VDTS_GM);
        c.bus.write32(VDTS_SINK + 0x14, VDTS_ITEMS);
        c.bus.write32(VDTS_SINK + 0x1c, rows);
        c.bus.write32(VDTS_SINK + 0x20, cols);
        for i in 0..3u32 {
            c.bus.write32(VDTS_SINK + 0x24 + 4 * i, 2 + i + seed % 3);
        }
        c.bus.write32(VDTS_SINK + 0x3c, 0x4444_0000 + seed * 7);
        c.bus.write32(VDTS_SINK + 0x40, 0x4120_0000 + (seed << 8));
        c.bus.write32(VDTS_SINK + 0x44, 0x41a0_0000 + (seed << 8));
        c.bus.write32(VDTS_SINK + 0x48, 0x3f00_0000 + (seed << 4));
        c.bus.write32(VDTS_SINK + 0x4c, 0x4080_0000 + (seed << 6));
        c.bus.write32(VDTS_SINK + 0x50, 0x3f80_0000 + (seed << 8));
        for k in 0..8u32 {
            c.bus.write32(VDTS_ITEMS + 4 * k, VDTS_CELLS + 0x10 * k);
            c.bus
                .write32(VDTS_CELLS + 0x10 * k, 0x0abc_0000 + k + (seed << 4));
        }
        c.bus.write32(VDTS_DRAW_CTRL, VDTS_DRAW_MAGIC);
        c.bus.write32(VDTS_DRAW_BUSY, seed % 2);
        c.g[A0][0] = VDTS_SINK as u64;
        c.pc = 0x0025_48d0;
    }

    /// `VD_TextureSink_DrawFrame`: idle retry, both unsigned-float arms,
    /// rows x cells up to 3 x 2 through the fake draw slots. Path totals
    /// (R = idle calls, n = negative getter results, r rows, k cells):
    /// `163 + 1007R + 36 + 2n + [r > 0](1 + r(13 + [k > 0](7 + 53k)))`.
    #[test]
    fn vd_texture_sink_draw_frame_matches() {
        vdts_draw_hook();
        check_paths(
            ee::s06::f_002548d0__generated,
            vd_texture_sink_draw_frame,
            96,
            // 1206 r=0 (R=1, n=0); 1210 n=2 (seed 6); 2213 R=2 (seed 1);
            // 1220 r=1 k=0; 1246 r=3 k=0; 1280 r=1 k=1; 1353 r=2 k=1;
            // 1459 r=2 k=2; 1585 r=3 k=2.
            &[1206, 1210, 2213, 1220, 1246, 1280, 1353, 1459, 1585],
            &|c, seed| {
                machine(c, seed, 0x0025_48d0, 0);
                c.hooks.miss = vdts_norm_miss;
                vdts_draw_fixture(c, seed);
            },
        );
        fakes::covered(
            ee::s06::f_002548d0__generated,
            vd_texture_sink_draw_frame,
            96,
            &|c, seed| {
                machine(c, seed, 0x0025_48d1, 6);
                c.hooks.miss = vdts_norm_miss;
                vdts_draw_fixture(c, seed);
            },
        );
    }

    // ---------- PS2BezierMan_DebugDrawGrid ----------

    /// The five draw loops under a delivered VBlank. All heads are
    /// structured: each side must carry on past the delivery, never
    /// stopping there (compared by `check` on the raw bodies; `drive`
    /// cannot resume past a nested unwind). The per-seed VBlank time
    /// lands inside a loop for every head: 1140 hits 0x254a70, 120 the
    /// 0x254918 busy retry (via `vdts_draw_hook`), 1300/2200/2400 the
    /// row/cell loops, the rest 0x254a40; a reverted head unwinds there
    /// and `check` fails.
    #[test]
    fn vd_texture_sink_draw_frame_events() {
        use crate::difftest::events::UNWIND;
        let (gen, hand) = (
            ee::s06::f_002548d0__generated as fn(&mut Cpu),
            vd_texture_sink_draw_frame as fn(&mut Cpu),
        );
        let mut hit = std::collections::BTreeSet::new();
        vdts_draw_hook();
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_48d0, 0);
            c.hooks.miss = vdts_norm_miss;
            vdts_draw_fixture(c, seed);
            unwind_at(
                c,
                0x0025_48d0,
                [1140, 120, 1300, 1150, 100, 2200, 140, 2400][seed as usize % 8] as u64,
            );
        };
        check(gen, hand, 96, &setup);
        for seed in 0..crate::difftest::seed_count(96) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            hit.insert(c.pc & !UNWIND);
        }
        let bad: Vec<u32> = [
            0x0025_4918,
            0x0025_4a40,
            0x0025_4a70,
            0x0025_4b28,
            0x0025_4af0,
        ]
        .into_iter()
        .filter(|h| hit.contains(h))
        .collect();
        assert!(
            bad.is_empty(),
            "unwound at structured heads {bad:#x?}: {hit:#x?}"
        );
    }

    fn bezier_grid_norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    pub(crate) const BEZIER_GRID_GM: u32 = DATA + 0x5_0000;
    pub(crate) const BEZIER_GRID_VT: u32 = DATA + 0x5_8000;
    pub(crate) const BEZIER_GRID_POINTS: u32 = DATA + 0x6_0000;
    pub(crate) const BEZIER_GRID_UVS: u32 = DATA + 0x6_4000;

    /// `n` = 0, 1, 2, 3, 4 or -2 by `seed % 6`; distinctive point and uv
    /// words; the graphics manager's draw slot (0x248: `this` delta 0x30,
    /// method FAKE).
    pub(crate) fn bezier_grid_fixture(c: &mut Cpu, seed: u32) {
        let n: u64 = [0, 1, 2, 3, 4, 0xffff_ffff_ffff_fffe][seed as usize % 6];
        for i in 0..0x40u32 {
            c.bus.write32(BEZIER_GRID_GM + 4 * i, 0x3333_0000 + i);
        }
        c.bus.write32(BEZIER_GRID_GM + 0x10d8, BEZIER_GRID_VT);
        for i in 0..0xa0u32 {
            c.bus.write32(BEZIER_GRID_VT + 4 * i, FAKE);
        }
        // The slot's `this` delta, distinct from the method word's low half.
        c.bus.write16(BEZIER_GRID_VT + 0x248, 0x30);
        c.bus.write32(GAME_GP + 0x2a90, BEZIER_GRID_GM);
        for i in 0..48u32 {
            c.bus
                .write32(BEZIER_GRID_POINTS + 4 * i, 0x3f80_0000 + (i << 12) + seed);
        }
        for i in 0..32u32 {
            c.bus
                .write32(BEZIER_GRID_UVS + 4 * i, 0x3e00_0000 + (i << 12) + seed);
        }
        c.g[A0][0] = BEZIER_GRID_POINTS as u64;
        c.g[A1][0] = BEZIER_GRID_UVS as u64;
        c.g[A2][0] = n;
        c.pc = 0x0038_c590;
    }

    /// `PS2BezierMan_DebugDrawGrid`: totals `27` for n <= 0, else
    /// `30 + 56n + 48n^2` (134, 334, 630, 1022 for n = 1..4).
    #[test]
    fn ps2_bezier_man_debug_draw_grid_matches() {
        check_paths(
            ee::s12::f_0038c590__generated,
            ps2_bezier_man_debug_draw_grid,
            60,
            &[27, 134, 334, 630, 1022],
            &|c, seed| {
                machine(c, seed, 0x0038_c590, 0);
                c.hooks.miss = bezier_grid_norm_miss;
                bezier_grid_fixture(c, seed);
            },
        );
        fakes::covered(
            ee::s12::f_0038c590__generated,
            ps2_bezier_man_debug_draw_grid,
            60,
            &|c, seed| {
                machine(c, seed, 0x0038_c591, 6);
                c.hooks.miss = bezier_grid_norm_miss;
                bezier_grid_fixture(c, seed);
            },
        );
    }

    // ---------- PS2_SONY_CODEC_INTERNAL_Open ----------

    fn sony_open_norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// Gate of the codec-open fakes below, and the switch that makes the
    /// `InitFrame` fake return null (the `movz` arm of the frame push).
    pub(crate) const SONY_OPEN_CTRL: u32 = DATA + 0x7_e000;
    pub(crate) const SONY_OPEN_MAGIC: u32 = 0x5c0d_ec01;
    pub(crate) const SONY_OPEN_NULL_FRAME: u32 = DATA + 0x7_e004;

    /// A gated fake: with the gate on, a logged fake call at a fixed 4-cycle
    /// cost; otherwise the slot's previous body.
    macro_rules! sony_open_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(SONY_OPEN_CTRL) != SONY_OPEN_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let before = c.cycles;
                fakes::fake_call(c, $addr);
                c.cycles = before + 4;
            }
        };
    }

    sony_open_fake!(
        sony_open_fake_init_buffer,
        SONY_OPEN_PREV_INIT_BUFFER,
        0x003b_1228,
        ee::s13::f_003b1228__generated
    );
    sony_open_fake!(
        sony_open_fake_get_hdr,
        SONY_OPEN_PREV_GET_HDR,
        0x003b_13d8,
        ee::s13::f_003b13d8__generated
    );
    sony_open_fake!(
        sony_open_fake_mpeg_init,
        SONY_OPEN_PREV_MPEG_INIT,
        0x0040_2708,
        ee::s15::f_00402708__generated
    );
    sony_open_fake!(
        sony_open_fake_mpeg_create,
        SONY_OPEN_PREV_MPEG_CREATE,
        0x0040_27b8,
        ee::s15::f_004027b8__generated
    );
    sony_open_fake!(
        sony_open_fake_add_callback,
        SONY_OPEN_PREV_ADD_CALLBACK,
        0x0040_2c08,
        ee::s15::f_00402c08__generated
    );

    static SONY_OPEN_PREV_INIT_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// `PS2_SONY_CODEC_INTERNAL_InitFrame` with the gate on: a logged fake
    /// (4 cycles) returning the fake's pointer, or null when the fixture
    /// asks for it.
    fn sony_open_fake_init_frame(c: &mut Cpu) {
        if c.bus.read32(SONY_OPEN_CTRL) != SONY_OPEN_MAGIC {
            return fakes::fall_back(
                c,
                &SONY_OPEN_PREV_INIT_FRAME,
                ee::s13::f_003b07f8__generated,
            );
        }
        let before = c.cycles;
        fakes::fake_call(c, 0x003b_07f8);
        c.cycles = before + 4;
        if c.bus.read32(SONY_OPEN_NULL_FRAME) != 0 {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn sony_open_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B1228),
            sony_open_fake_init_buffer,
            &SONY_OPEN_PREV_INIT_BUFFER,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B13D8),
            sony_open_fake_get_hdr,
            &SONY_OPEN_PREV_GET_HDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B07F8),
            sony_open_fake_init_frame,
            &SONY_OPEN_PREV_INIT_FRAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402708),
            sony_open_fake_mpeg_init,
            &SONY_OPEN_PREV_MPEG_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004027B8),
            sony_open_fake_mpeg_create,
            &SONY_OPEN_PREV_MPEG_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C08),
            sony_open_fake_add_callback,
            &SONY_OPEN_PREV_ADD_CALLBACK,
        );
    }

    pub(crate) const SONY_OPEN_CTX: u32 = DATA + 0x5_0000;
    pub(crate) const SONY_OPEN_CFG: u32 = DATA + 0x5_1000;
    pub(crate) const SONY_OPEN_DESC: u32 = DATA + 0x5_2000;

    /// Decoder globals the open writes (prefilled so every store shows).
    const SONY_OPEN_OUT_GLOBALS: [u32; 12] = [
        0x0050_94d4,
        0x0050_94d8,
        0x0050_94dc,
        0x0050_94e0,
        0x0050_94fc,
        0x0050_9500,
        0x0050_9524,
        0x0050_9548,
        0x0050_9584,
        0x0050_958c,
        0x0050_95a0,
        0x0050_a088,
    ];

    /// Codec-open fixture. `seed % 3`: 0 = MPEG-1 (`0x509a68` = 0, the
    /// defaults arm and rows/16 via 0x3b0da8), 1 = MPEG-2 progressive
    /// (`0x509524` != 0, rows/16 via 0x3b0dac), 2 = MPEG-2 interlaced
    /// (`0x509524` = 0, rows/32*2); `seed / 3 % 4` bit 0 = old work buffer
    /// (`+0x2c`), bit 1 = old IPU buffer (`+0x78`); `seed / 12 % 4` frames;
    /// `seed / 48` negative h/v sizes (the `movn` rounding); every fifth seed
    /// `InitFrame` returns null.
    pub(crate) fn sony_open_fixture(c: &mut Cpu, seed: u32) {
        let mode = seed % 3;
        let frees = (seed / 3) % 4;
        let frames = (seed / 12) % 4;
        let negative = (seed / 48) % 2 == 1;
        for i in 0..0x30u32 {
            c.bus.write32(SONY_OPEN_CTX + 4 * i, 0x5555_0000 + i);
        }
        c.bus.write32(
            SONY_OPEN_CTX + 0x2c,
            if frees & 1 != 0 { DATA + 0x5_3000 } else { 0 },
        );
        c.bus.write32(
            SONY_OPEN_CTX + 0x78,
            if frees & 2 != 0 { DATA + 0x5_3100 } else { 0 },
        );
        c.bus.write32(SONY_OPEN_CTX + 0x20, SONY_OPEN_CTX + 0x20);
        c.bus.write32(SONY_OPEN_CTX + 0x24, SONY_OPEN_CTX + 0x20);
        for i in 0..8u32 {
            c.bus.write32(SONY_OPEN_CFG + 4 * i, 0x6666_0000 + i);
        }
        c.bus.write32(SONY_OPEN_CFG + 0x10, frames);
        c.bus.write32(SONY_OPEN_DESC, 0x4d4d_0000 + seed);
        c.bus.write32(SONY_OPEN_DESC + 4, 0x5eed_0000 + seed);
        // The video alloc/free hooks (both the fake) and the alloc flags.
        c.bus.write32(0x0050_9430, FAKE);
        c.bus.write32(0x0050_9434, FAKE);
        c.bus.write32(0x0050_9438, 0x0bad_0000 | seed);
        for (i, a) in SONY_OPEN_OUT_GLOBALS.into_iter().enumerate() {
            c.bus.write32(a, 0x7a7a_0000 + i as u32);
        }
        c.bus
            .write32(0x0050_9a68, if mode == 0 { 0 } else { 1 + seed });
        c.bus
            .write32(0x0050_9524, if mode == 2 { 0 } else { 0x10 + seed });
        let (h, v) = if negative {
            (
                (-(0x21 + (seed % 7) as i32)) as u32,
                (-(0x13 + (seed % 5) as i32)) as u32,
            )
        } else {
            (0x140 + seed, 0xe0 + seed)
        };
        c.bus.write32(0x0050_94f4, h);
        c.bus.write32(0x0050_94f8, v);
        c.bus.write32(SONY_OPEN_CTRL, SONY_OPEN_MAGIC);
        c.bus.write32(SONY_OPEN_NULL_FRAME, (seed % 5 == 4) as u32);
        c.g[A0][0] = SONY_OPEN_CTX as u64;
        c.g[A1][0] = SONY_OPEN_CFG as u64;
        c.g[A2][0] = SONY_OPEN_DESC as u64;
        c.pc = 0x003b_0c58;
    }

    /// `PS2_SONY_CODEC_INTERNAL_Open`: all three header modes, both frees,
    /// 0..3 frames. Totals: 167 + {11 progressive, 13 interlaced, 21 MPEG-1}
    /// + 9 (work free) + 8 (IPU free) + [frames > 0](6 + 30 * frames).
    #[test]
    fn ps2_sony_codec_internal_open_matches() {
        sony_open_hook();
        check_paths(
            ee::s13::f_003b0c58__generated,
            ps2_sony_codec_internal_open,
            96,
            // 178 progressive; 180 interlaced; 187 progressive + work free;
            // 196 MPEG-1 + IPU free; 205 MPEG-1 + both frees; 214 progressive
            // + 1 frame; 274 progressive + 3 frames.
            &[178, 180, 187, 196, 205, 214, 274],
            &|c, seed| {
                machine(c, seed, 0x003b_0c58, 0);
                c.hooks.miss = sony_open_norm_miss;
                sony_open_fixture(c, seed);
            },
        );
        fakes::covered(
            ee::s13::f_003b0c58__generated,
            ps2_sony_codec_internal_open,
            96,
            &|c, seed| {
                machine(c, seed, 0x003b_0c59, 9);
                c.hooks.miss = sony_open_norm_miss;
                sony_open_fixture(c, seed);
            },
        );
    }

    // ---------- PS2VisualEffectsMan_ctor ----------

    /// Gate of the `EA_MemAllocNamed` fake below and its bump pointer.
    pub(crate) const VFX_CTOR_CTRL: u32 = DATA + 0x7_d000;
    pub(crate) const VFX_CTOR_MAGIC: u32 = 0x0fc7_a110;
    pub(crate) const VFX_CTOR_BUMP: u32 = DATA + 0x7_d004;
    static VFX_CTOR_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemAllocNamed` with the gate on: a logged fake call (4 cycles)
    /// returning distinct 16-byte objects from the bump pointer.
    fn vfx_ctor_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(VFX_CTOR_CTRL) != VFX_CTOR_MAGIC {
            return fakes::fall_back(c, &VFX_CTOR_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        let before = c.cycles;
        fakes::fake_call(c, 0x0031_7d70);
        c.cycles = before + 4;
        let p = c.bus.read32(VFX_CTOR_BUMP);
        c.bus.write32(VFX_CTOR_BUMP, p + 0x10);
        c.g[V0][0] = p as u64;
    }

    pub(crate) fn vfx_ctor_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            vfx_ctor_fake_alloc,
            &VFX_CTOR_PREV_ALLOC,
        );
    }

    pub(crate) const VFX_CTOR_OBJ: u32 = DATA + 0x5_0000;
    pub(crate) const VFX_CTOR_GM: u32 = DATA + 0x5_1000;
    pub(crate) const VFX_CTOR_PARTS: u32 = DATA + 0x5_2000;

    /// VFX-manager fixture: the object and the graphics manager the base
    /// ctor writes (`gp + 0x2a90`), prefilled; `PathArrow`'s first-use flag
    /// `gp + 0x114c` clear on even seeds (its table-fill loop), set on odd;
    /// its table at 0x504560 prefilled.
    pub(crate) fn vfx_ctor_fixture(c: &mut Cpu, seed: u32) {
        for i in 0..0x14u32 {
            c.bus.write32(VFX_CTOR_OBJ + 4 * i, 0x1515_0000 + i);
        }
        for i in 0..0xa0u32 {
            c.bus.write32(VFX_CTOR_GM + 4 * i, 0x3333_0000 + i);
        }
        c.bus.write32(GAME_GP + 0x2a90, VFX_CTOR_GM);
        for off in [0xa88u32, 0x134c, 0x14e4] {
            c.bus.write32(GAME_GP + off, 0x4747_0000 + off);
        }
        c.bus
            .write32(GAME_GP + 0x114c, if seed % 2 == 0 { 0 } else { 1 + seed });
        for i in 0..4u32 {
            c.bus
                .write32(GAME_GP + 0x1118 + 4 * i, 0x4111_0000 + (i << 16) + seed);
        }
        for i in 0..0x68u32 {
            c.bus.write32(0x0050_4560 + 4 * i, 0x5a5a_0000 + i);
        }
        for i in 0..0x120u32 {
            c.bus.write32(VFX_CTOR_PARTS + 4 * i, 0x6b6b_0000 + i);
        }
        c.bus.write32(VFX_CTOR_BUMP, VFX_CTOR_PARTS);
        c.bus.write32(VFX_CTOR_CTRL, VFX_CTOR_MAGIC);
        c.g[A0][0] = VFX_CTOR_OBJ as u64;
        c.pc = 0x002f_0548;
    }

    /// `PS2VisualEffectsMan_ctor`: real base ctor, `DetailSystem_ctor` and
    /// `PathArrow_ctor`, 17 fake allocations. Totals 181 + 66 + 17 * 4 + 5 +
    /// {7 flag set, 108 table fill}.
    #[test]
    fn ps2_visual_effects_man_ctor_matches() {
        vfx_ctor_hook();
        check_paths(
            ee::s09::f_002f0548__generated,
            ps2_visual_effects_man_ctor,
            60,
            &[327, 428],
            &|c, seed| {
                machine(c, seed, 0x002f_0548, 0);
                vfx_ctor_fixture(c, seed);
            },
        );
        fakes::covered(
            ee::s09::f_002f0548__generated,
            ps2_visual_effects_man_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x002f_0549, 17);
                vfx_ctor_fixture(c, seed);
            },
        );
    }

    // ----- PS2VisualEffectsMan_dtor -------------------------------------

    /// Gate of the `__builtin_delete` fake: the real delete runs unless a
    /// fixture plants this word.
    const VFX_MAN_DTOR_CTRL: u32 = DATA + 0x7f00;
    const VFX_MAN_DTOR_MAGIC: u32 = 0x2f08_20d1;
    static VFX_MAN_DTOR_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    /// `__builtin_delete` under the gate: logged like a fake call, a fixed
    /// 4-cycle cost (the fake's hash cost would make the totals vary).
    fn vfx_man_dtor_fake_delete(c: &mut Cpu) {
        if c.bus.read32(VFX_MAN_DTOR_CTRL) != VFX_MAN_DTOR_MAGIC {
            return fakes::fall_back(c, &VFX_MAN_DTOR_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        let before = c.cycles;
        fakes::fake_call(c, VFX_MAN_DTOR_DELETE);
        c.cycles = before + 4;
    }

    /// The `miss` hook (the owned objects' methods at `FAKE + ..`) with a
    /// fixed 4-cycle cost.
    fn vfx_man_dtor_norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// The manager at `DATA + 0x1000`; owned object `i` (when its bit is
    /// set) at `DATA + 0x2000 + 0x40 i` with its own vtable (delta, method
    /// `FAKE + 0x10 i`). Seeds 0..7: none / all owned, delete flag off / on
    /// (the four path classes); later seeds random masks, flags and unwinds.
    pub(crate) fn vfx_man_dtor_fixture(c: &mut Cpu, seed: u32) {
        let calls = if seed < 8 { 0 } else { VFX_MAN_OBJECTS + 1 };
        let mut r = machine(c, seed, 0x002f_0820, calls);
        c.hooks.miss = vfx_man_dtor_norm_miss;
        c.bus.write32(VFX_MAN_DTOR_CTRL, VFX_MAN_DTOR_MAGIC);
        const THIS: u32 = DATA + 0x1000;
        let (mask, delete) = if seed < 8 {
            (if seed % 4 < 2 { 0 } else { 0x1_ffff }, seed % 2 == 1)
        } else {
            (r.next_u32() & 0x1_ffff, r.next_u32() % 2 == 1)
        };
        for i in 0..VFX_MAN_OBJECTS {
            let member = if mask >> i & 1 != 0 {
                let obj = DATA + 0x2000 + 0x40 * i;
                let vt = DATA + 0x3000 + 0x20 * i;
                c.bus.write32(obj, vt);
                let delta = if i % 2 == 0 { 8 * i } else { 0x1_0000 - 8 * i };
                c.bus.write16(vt + 8, delta as u16);
                c.bus.write32(vt + 0xc, FAKE + 0x10 * i);
                obj
            } else {
                0
            };
            c.bus.write32(THIS + 4 * i, member);
        }
        c.bus.write32(THIS + VFX_MAN_VPTR, 0x1234_5678 ^ seed);
        c.g[A0][0] = THIS as u64;
        let flags = (r.next_u32() & 0xfe) as u64;
        c.g[A1][0] = flags | delete as u64;
        c.pc = 0x002f_0820;
    }

    /// `PS2VisualEffectsMan_dtor`: no / all / random owned objects deleted
    /// through their vtables, `this` freed or kept. Paths (4-cycle fakes):
    /// 69 none kept, 75 none + delete, 223 all kept, 229 all + delete.
    #[test]
    fn ps2_visual_effects_man_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            vfx_man_dtor_fake_delete,
            &VFX_MAN_DTOR_PREV_DELETE,
        );
        check_paths(
            ee::s09::f_002f0820__generated,
            ps2_visual_effects_man_dtor,
            64,
            &[69, 75, 223, 229],
            &|c, seed| vfx_man_dtor_fixture(c, seed),
        );
    }

    // ----- shared DLHeap arena fixture -----------------------------------

    /// Heap header and arena (`DLHeap_InitArena`, 256 KiB at `DATA+0x40000`).
    pub(crate) const DLHEAP_FIX_HEAP: u32 = DATA + 0x2000;
    pub(crate) const DLHEAP_FIX_ARENA: u32 = DATA + 0x4_0000;
    /// `__getreent()` loads the errno block from here; the fixtures point it
    /// at a prefilled word so `ENOMEM` lands in compared RAM.
    pub(crate) const DLHEAP_FIX_REENT_PTR: u32 = 0x0045_4944;
    pub(crate) const DLHEAP_FIX_REENT: u32 = DATA + 0x1_0000;

    /// Run a real allocator entry point with `args` and return `v0`
    /// (`ra`, `pc` restored by the caller's own setup).
    pub(crate) fn dlheap_fix_call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        let ra = c.g[RA][0];
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "fixture call {addr:#x} did not return");
        c.g[RA][0] = ra;
        c.g[V0][0]
    }

    pub(crate) fn dlheap_fix_malloc(c: &mut Cpu, n: u32) -> u32 {
        let heap = DLHEAP_FIX_HEAP as u64;
        dlheap_fix_call(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[heap, n as u64]) as u32
    }

    pub(crate) fn dlheap_fix_free(c: &mut Cpu, p: u32) {
        let heap = DLHEAP_FIX_HEAP as u64;
        dlheap_fix_call(c, 0x0031_ed60, ee::s10::DLHeap_Free, &[heap, p as u64]);
    }

    /// A fresh arena (state zeroed: Consolidate initialises it on first
    /// use) and the errno block; returns the heap header.
    pub(crate) fn dlheap_fix_arena(c: &mut Cpu) -> u32 {
        dlheap_fix_call(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[DLHEAP_FIX_HEAP as u64, DLHEAP_FIX_ARENA as u64, 0x4_0000],
        );
        c.bus.write32(DLHEAP_FIX_REENT_PTR, DLHEAP_FIX_REENT);
        c.bus.write32(DLHEAP_FIX_REENT, 0x5a5a_0001);
        DLHEAP_FIX_HEAP
    }

    /// Distinct payload words in a block (so copies are compared).
    pub(crate) fn dlheap_fix_fill(c: &mut Cpu, p: u32, bytes: u32, tag: u32) {
        for i in 0..bytes / 4 {
            c.bus.write32(p + 4 * i, tag.wrapping_mul(0x9e37_79b9) ^ i);
        }
    }

    // ----- DLHeap_Realloc --------------------------------------------------

    /// A fake mmapped chunk (outside the arena) for the `ENOMEM` arm.
    const DLHEAP_REALLOC_MMAPPED: u32 = DATA + 0x2_0000;
    const DLHEAP_REALLOC_CLASSES: u32 = 15;

    /// Six blocks out of a real arena: p0 (chunk 0x10), p1 (0x20, fast),
    /// pa (0x110), pb (0x210), pq (0x20), pc (0x90, next to top), each
    /// with distinct payload. Class `seed % 15` (seeds < 45) picks the
    /// arm; later seeds free random blocks and resize a random live one.
    pub(crate) fn dlheap_realloc_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0031_f018, 0);
        let heap = dlheap_fix_arena(c);
        let sizes = [8u32, 0x18, 0x100, 0x200, 0x18, 0x80];
        let mut ptrs = [0u32; 6];
        for (i, &n) in sizes.iter().enumerate() {
            ptrs[i] = dlheap_fix_malloc(c, n);
            dlheap_fix_fill(c, ptrs[i], n, seed * 8 + i as u32 + 1);
        }
        let [p0, p1, pa, pb, pq, pc] = ptrs;
        let (old, bytes): (u32, u64) = if seed < 3 * DLHEAP_REALLOC_CLASSES {
            match seed % DLHEAP_REALLOC_CLASSES {
                // realloc(0, n) is malloc (returns to 0x31f058).
                0 => (0, 0x100),
                // 0x31f28c: request above -0x21.
                1 => (pa, sx(0xffff_fff0)),
                // 0x31f28c: an mmapped chunk.
                2 => {
                    c.bus.write32(DLHEAP_REALLOC_MMAPPED, 0x40);
                    c.bus.write32(DLHEAP_REALLOC_MMAPPED + 4, 0x102);
                    (DLHEAP_REALLOC_MMAPPED + 8, 0x40)
                }
                // 0x31f220 (already big enough), remainder 0: no split.
                3 => (pa, 0xfd + (seed / 15) as u64),
                // 0x31f220, remainder 0xe0: 0x31f250 split + DLHeap_Free.
                4 => (pa, 0x20),
                // 0x31f0bc next == top, room: grow into top.
                5 => (pc, 0x280),
                // next == top, no room: 0x31f158 malloc fails (0x31f29c).
                6 => (pc, 0x10_0000),
                // next in use: 0x31f158 move, memcpy.
                7 => (pa, 0x300),
                // move, small copy of 3 words (exit at 0x31f1b0's a1 < 5).
                8 => (p0, 0x40),
                // move, small copy of 7 words (exit at a1 < 9).
                9 => (pq, 0x80),
                // 0x31f10c next free, enough: unlink then split.
                10 => {
                    dlheap_fix_free(c, pb);
                    (pa, 0x180)
                }
                // next free, exactly nb: unlink, no split.
                11 => {
                    dlheap_fix_free(c, pb);
                    (pa, 0x310)
                }
                // next free, too small: move (memcpy).
                12 => {
                    dlheap_fix_free(c, pb);
                    (pa, 0x1000)
                }
                // malloc returns the next chunk (p1 from its fast bin):
                // 0x31f188 merge in place, then split.
                13 => {
                    dlheap_fix_free(c, p1);
                    (p0, 0x18)
                }
                // A zero-extended 0xfffffff0: not "too large" in the 64-bit
                // compare, request2size wraps to the MINSIZE arm (0x31f08c).
                _ => (pa, 0xffff_fff0),
            }
        } else {
            let mut live = vec![p0, pa, pc];
            for p in [p1, pb, pq] {
                if r.next_u32() % 2 == 0 {
                    dlheap_fix_free(c, p);
                } else {
                    live.push(p);
                }
            }
            let old = if r.next_u32() % 8 == 0 {
                0
            } else {
                live[(r.next_u32() as usize) % live.len()]
            };
            (old, (r.next_u32() % 0x600) as u64)
        };
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = old as u64;
        c.g[A2][0] = bytes;
        c.g[A3][0] = 0xcccc_0000 + seed as u64;
        c.pc = 0x0031_f018;
    }

    /// `DLHeap_Realloc` over a real arena: every arm from malloc, ENOMEM
    /// (size, mmapped), keep (split / no split), grow into top or into a
    /// free next chunk, to move (memcpy, 3- and 7-word copies, malloc
    /// failure, merge with the returned next chunk).
    /// Paths: 37 class 1, 50 class 2, 61 class 3, 65 class 5 (derived);
    /// every class total is pinned, in class order.
    #[test]
    fn dlheap_realloc_matches() {
        check_paths(
            ee::s10::f_0031f018__generated,
            dlheap_realloc,
            75,
            &[
                86, 37, 50, 61, 118, 65, 210, 292, 174, 186, 137, 80, 465, 171, 115,
            ],
            &|c, seed| dlheap_realloc_fixture(c, seed),
        );
    }

    // ----- DLHeap_IAlloc ---------------------------------------------------

    const DLHEAP_IALLOC_SIZES: u32 = DATA + 0x6000;
    const DLHEAP_IALLOC_CHUNKS: u32 = DATA + 0x6800;
    const DLHEAP_IALLOC_CLASSES: u32 = 11;

    /// `ialloc(heap, n, sizes, opts, chunks)` on a real arena: fresh (the
    /// Consolidate arm) or with traffic; the caller's array prefilled.
    /// Class `seed % 11` (seeds < 33) picks the arm, later seeds random.
    pub(crate) fn dlheap_ialloc_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0031_f858, 0);
        let heap = dlheap_fix_arena(c);
        let class = if seed < 3 * DLHEAP_IALLOC_CLASSES {
            seed % DLHEAP_IALLOC_CLASSES
        } else {
            99
        };
        if class != 0 {
            let a = dlheap_fix_malloc(c, 0x40);
            let _ = dlheap_fix_malloc(c, 0x120);
            dlheap_fix_free(c, a);
        }
        for i in 0..8 {
            c.bus.write32(DLHEAP_IALLOC_SIZES + 4 * i, 0x10 + 0x18 * i);
            c.bus
                .write32(DLHEAP_IALLOC_CHUNKS + 4 * i, 0xeeee_0000 + i + seed);
        }
        let chunks = DLHEAP_IALLOC_CHUNKS as u64;
        let (n, opts, chunks): (u64, u64, u64) = match class {
            // Fresh state: Consolidate (0x31f8a4); sizes loop, malloc,
            // array chunk (0x31f9f0), per-element carve.
            0 => (3, 0, 0),
            // chunks with n == 0: return chunks (0x31f8bc).
            1 => (0, 0, chunks),
            // No array, n == 0: malloc(heap, 0) (0x31f8d4).
            2 => (0, 0, 0),
            // Caller array, all one size (0x31f91c, mult; carve with s4).
            3 => (4, 1, chunks),
            // Cleared contents (0x31f9d4 memset), sizes loop.
            4 => (5, 2, 0),
            // One size and cleared.
            5 => (4, 3, 0),
            // A wrapped size (request2size MINSIZE arms 0x31f968 / 0x31fa30).
            6 => {
                c.bus.write32(DLHEAP_IALLOC_SIZES + 4, 0xffff_fff0);
                (3, 0, 0)
            }
            // One wrapped size (0x31f91c MINSIZE arm).
            7 => {
                c.bus.write32(DLHEAP_IALLOC_SIZES, 0xffff_fff0);
                (3, 1, chunks)
            }
            // Contents beyond the arena: malloc fails (v0 = 0 at 0x31f990).
            8 => {
                for i in 0..4 {
                    c.bus.write32(DLHEAP_IALLOC_SIZES + 4 * i, 0x2_0000);
                }
                (4, 0, 0)
            }
            // Caller array, one size, cleared. (The `n * 4` wrap of
            // 0x31f908 needs n = 0x3ffffffc, and the generated carve loop
            // then walks n elements: not bounded in a test.)
            9 => (2, 3, chunks),
            // One element: the carve loop ends at once.
            10 => (1, 0, 0),
            _ => {
                for i in 0..8 {
                    let s = r.next_u32() % 0x300;
                    c.bus.write32(DLHEAP_IALLOC_SIZES + 4 * i, s);
                }
                let chunks = if r.next_u32() % 2 == 0 { chunks } else { 0 };
                (
                    1 + (r.next_u32() % 7) as u64,
                    (r.next_u32() % 4) as u64,
                    chunks,
                )
            }
        };
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = n;
        c.g[A2][0] = DLHEAP_IALLOC_SIZES as u64;
        c.g[A3][0] = opts;
        c.g[T0][0] = chunks;
        c.pc = 0x0031_f858;
    }

    /// `DLHeap_IAlloc`: every arm of the array / size / clear options over
    /// a real arena (LO/HI compared: the one-size product is a `mult`).
    /// Not reachable: 0x31f948's `n == 0` exit (n == 0 returns earlier), and
    /// the array-size wrap 0x31f908 (unbounded, see class 9).
    /// Paths: every class total, in class order.
    #[test]
    fn dlheap_ialloc_matches() {
        check_paths_lohi(
            ee::s10::f_0031f858__generated,
            dlheap_ialloc,
            66,
            &[1122, 37, 165, 240, 672, 351, 279, 230, 483, 303, 226],
            &|c, seed| dlheap_ialloc_fixture(c, seed),
        );
    }

    // ----- DLHeap_MemalignOffset ---------------------------------------------

    /// Manual Unwind-mode schedule past the prologue: the VBlank goes off
    /// `k` cycles after entry. `schedule` cannot do this: its k < 48
    /// fires inside the prologue calls (or before the draw loops),
    /// and `drive` cannot re-enter past a nested unwind.
    fn unwind_at(c: &mut Cpu, entry: u32, k: u64) {
        let saved = (c.g[A0], c.g[A1], c.g[RA]);
        ssx3_ps2::kernel::start_main_thread(c, entry, 0, 0);
        (c.g[A0], c.g[A1], c.g[RA]) = saved;
        c.pc = entry;
        c.cycles = 1000;
        c.next_event = 1000 + k;
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .sched
            .next_vblank = 1000 + k;
    }

    /// `IAlloc`'s two dispatcher heads under a delivered VBlank.
    /// Both heads are dispatcher-shaped in generated too, so each side
    /// must stop at the head with an unwind pending (compared by `check`
    /// on the raw bodies, which needs no re-entry; `drive` cannot resume
    /// past a nested unwind, so `check_events` cannot drive this port).
    /// One dense `k` grid per head (`k` rides on the seed, so every run
    /// sweeps the loop's span).
    #[test]
    fn dlheap_ialloc_events() {
        use crate::difftest::events::UNWIND;
        let (gen, hand) = (
            ee::s10::f_0031f858__generated as fn(&mut Cpu),
            dlheap_ialloc as fn(&mut Cpu),
        );
        for (head, base, step) in [(0x0031_f968, 750u64, 15u64), (0x0031_fa10, 900u64, 10u64)] {
            let mut hit = std::collections::BTreeSet::new();
            let setup = |c: &mut Cpu, seed: u32| {
                dlheap_ialloc_fixture(c, seed);
                unwind_at(c, 0x0031_f858, base + (seed % 33) as u64 * step);
            };
            check(gen, hand, 33, &setup);
            for seed in 0..crate::difftest::seed_count(33) {
                let mut c = crate::difftest::fresh();
                setup(&mut c, seed);
                crate::difftest::run_generated(&mut c, gen);
                hit.insert(c.pc & !UNWIND);
            }
            assert!(
                hit.contains(&head),
                "dispatcher head {head:#x} never unwound: {hit:#x?}"
            );
        }
    }

    /// Gate and chunk of the `DLHeap_Malloc` fake that hands back a planted
    /// chunk (mmapped, or exactly sized) the real arena never returns.
    const DLHEAP_MEMOFF_CTRL: u32 = DATA + 0x7e00;
    const DLHEAP_MEMOFF_MAGIC: u32 = 0x31f4_e8a1;
    const DLHEAP_MEMOFF_CHUNK: u32 = DATA + 0x2_0000;
    const DLHEAP_MEMOFF_CLASSES: u32 = 13;
    static DLHEAP_MEMOFF_PREV_MALLOC: AtomicUsize = AtomicUsize::new(0);

    fn dlheap_memoff_fake_malloc(c: &mut Cpu) {
        if c.bus.read32(DLHEAP_MEMOFF_CTRL) != DLHEAP_MEMOFF_MAGIC {
            return fakes::fall_back(
                c,
                &DLHEAP_MEMOFF_PREV_MALLOC,
                ee::s10::f_0031e818__generated,
            );
        }
        let before = c.cycles;
        fakes::fake_call(c, 0x0031_e818);
        c.cycles = before + 4;
    }

    /// The arena every MemalignOffset class starts from (deterministic per
    /// seed, so a probe machine can predict where the malloc lands).
    fn dlheap_memoff_layout(c: &mut Cpu, seed: u32) -> u32 {
        let heap = dlheap_fix_arena(c);
        let a = dlheap_fix_malloc(c, 0x40 + 0x10 * (seed % 5));
        let _ = dlheap_fix_malloc(c, 0x200);
        let _ = dlheap_fix_malloc(c, 0x30);
        if seed % 2 == 1 {
            dlheap_fix_free(c, a);
        }
        heap
    }

    /// request2size as the function computes it (no wrap for these sizes).
    fn dlheap_memoff_nb(total: u32) -> u32 {
        if total + 0x13 < 0x10 {
            0x10
        } else {
            (total + 0x13) & !0xf
        }
    }

    /// The offset that puts `mem + offset` at `rem` past an `align`
    /// boundary, with `mem` the chunk malloc returns for the request
    /// (found on a probe machine built the same way).
    fn dlheap_memoff_offset(seed: u32, align: u32, total: u32, rem: u32) -> u32 {
        let mut p = fresh();
        dlheap_memoff_layout(&mut p, seed);
        let request = dlheap_memoff_nb(total) + align + 0x10;
        let mem = dlheap_fix_malloc(&mut p, request);
        (rem + align - mem % align) % align
    }

    /// Classes (seeds < 39, `seed % 13`): a1 the alignment, `total` =
    /// bytes + offset, `rem` the wanted misalignment of mem + offset.
    pub(crate) fn dlheap_memoff_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0031_f4e8, 0);
        let heap = dlheap_memoff_layout(c, seed);
        let class = if seed < 3 * DLHEAP_MEMOFF_CLASSES {
            seed % DLHEAP_MEMOFF_CLASSES
        } else {
            99
        };
        // (align as passed, align as adjusted, total, rem) for real-malloc classes.
        let real = |align: u32, adj: u32, total: u32, rem: u32| -> (u32, u64, u32) {
            let off = dlheap_memoff_offset(seed, adj, total, rem);
            (align, (total - off) as u64, off)
        };
        let (align, bytes, offset): (u32, u64, u32) = match class {
            // align <= 16: plain malloc of bytes (returns to 0x31f524).
            0 => (8, 0x100, 0x10),
            // 0x31f52c: 0x18 -> 0x20; lead 0x10 (>= MINSIZE: brk).
            1 => real(0x18, 0x20, 0x200, 0x10),
            // 0x31f560 loop, no shift: 0x30 -> 0x40; lead 0x20.
            2 => real(0x30, 0x40, 0x200, 0x20),
            // 0x31f560 loop, shifts: 0x300 -> 0x400; lead 0x200.
            3 => real(0x300, 0x400, 0x600, 0x200),
            // Power of two, already aligned: tail split only.
            4 => real(0x80, 0x80, 0x300, 0),
            // Lead under MINSIZE (rem = align - 8): brk + align.
            5 => real(0x40, 0x40, 0x300, 0x38),
            // Power of two, misaligned.
            6 => real(0x100, 0x100, 0x400, 0x80),
            // Fake mmapped chunk, misaligned: move its offset (the +7 arm).
            7 => {
                c.bus.write32(DLHEAP_MEMOFF_CHUNK, 0x40);
                c.bus.write32(DLHEAP_MEMOFF_CHUNK + 4, 0x1000 | 2);
                (0x40, 0x100, 0)
            }
            // Fake mmapped chunk, aligned: returned as is (0x31f6a0).
            8 => {
                c.bus.write32(DLHEAP_MEMOFF_CHUNK, 0x40);
                c.bus.write32(DLHEAP_MEMOFF_CHUNK + 4, 0x1000 | 2);
                (0x40, 0x100, 0x38)
            }
            // Fake plain chunk of exactly nb + 16, aligned: no tail split.
            9 => {
                c.bus.write32(DLHEAP_MEMOFF_CHUNK, 0x40);
                c.bus.write32(DLHEAP_MEMOFF_CHUNK + 4, (0x140 + 0x10) | 1);
                (0x40, 0x100, 0x38)
            }
            // bytes + offset above -0x21: ENOMEM.
            10 => (0x40, sx(0xffff_ffe0), 0x10),
            // Beyond the arena: malloc fails.
            11 => (0x40, 0x10_0000, 0x10),
            // Power of two 0x20, misaligned by 0x10.
            12 => real(0x20, 0x20, 0x180, 0x10),
            _ => {
                let align = match r.next_u32() % 3 {
                    0 => 1 << (5 + r.next_u32() % 5),
                    1 => 17 + r.next_u32() % 0x200,
                    _ => r.next_u32() % 17,
                };
                (
                    align,
                    (r.next_u32() % 0x800) as u64,
                    8 * (r.next_u32() % 0x10),
                )
            }
        };
        if (7..=9).contains(&class) {
            c.bus.write32(LOG_RESULTS, DLHEAP_MEMOFF_CHUNK + 8);
            c.bus.write32(DLHEAP_MEMOFF_CTRL, DLHEAP_MEMOFF_MAGIC);
        }
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = align as u64;
        c.g[A2][0] = bytes;
        c.g[A3][0] = offset as u64;
        c.pc = 0x0031_f4e8;
    }

    /// `DLHeap_MemalignOffset`: alignment rounding (small, 0x20, loop with
    /// and without shifts), ENOMEM, malloc failure, aligned / misaligned
    /// (lead >= or < MINSIZE) over a real arena; mmapped and exact-size
    /// chunks through a gated `DLHeap_Malloc` fake. Not reachable: the
    /// request2size MINSIZE arm (bytes + offset wrapping is caught as too
    /// large first) and, with real chunks, the no-split tail (the request
    /// leaves at least align + 16 over).
    /// Paths (4-cycle fake): 83 class 7, 61 class 8, 69 class 9, 43 class
    /// 10 (derived); every class total, in class order.
    #[test]
    fn dlheap_memalign_offset_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031E818),
            dlheap_memoff_fake_malloc,
            &DLHEAP_MEMOFF_PREV_MALLOC,
        );
        // `check_paths_lohi`: the port does `divu`, so LO/HI are compared.
        check_paths_lohi(
            ee::s10::f_0031f4e8__generated,
            dlheap_memalign_offset,
            78,
            &[83, 361, 240, 431, 175, 465, 260, 61, 69, 43, 320, 227],
            &|c, seed| dlheap_memoff_fixture(c, seed),
        );
    }

    // ----- DLHeap_Mallinfo ---------------------------------------------------

    /// The `MemalignOffset` align loop under a delivered VBlank. Its
    /// head (0x31f560) is structured: each side must carry on past the
    /// delivery, never stopping there (compared by `check` on the raw
    /// bodies; `drive` cannot resume past a nested unwind). The VBlank
    /// goes off 20 cycles after entry: the loop runs at +33..+54 (only
    /// class 3 reaches it) with no earlier checks, so a reverted head
    /// unwinds there and `check` fails.
    #[test]
    fn dlheap_memalign_offset_events() {
        use crate::difftest::events::UNWIND;
        let (gen, hand) = (
            ee::s10::f_0031f4e8__generated as fn(&mut Cpu),
            dlheap_memalign_offset as fn(&mut Cpu),
        );
        let mut hit = std::collections::BTreeSet::new();
        let setup = |c: &mut Cpu, seed: u32| {
            dlheap_memoff_fixture(c, seed);
            unwind_at(c, 0x0031_f4e8, 20);
        };
        check(gen, hand, 39, &setup);
        for seed in 0..crate::difftest::seed_count(39) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            hit.insert(c.pc & !UNWIND);
        }
        assert!(
            !hit.contains(&0x0031_f560),
            "unwound at structured head 0x31f560: {hit:#x?}"
        );
    }

    const DLHEAP_MALLINFO_OUT: u32 = DATA + 0x6000;
    const DLHEAP_MALLINFO_CLASSES: u32 = 4;

    /// Arena states: 0 fresh (no top: Consolidate), 1 fast bins (three
    /// chunks in one bin, one in another) plus three chunks in the unsorted
    /// bin, 2 the same sorted into small bins by a malloc, 3 no free chunk;
    /// later seeds random traffic.
    fn dlheap_mallinfo_layout(c: &mut Cpu, r: &mut Rng, class: u32) {
        if class == 0 {
            return;
        }
        if class == 3 {
            dlheap_fix_malloc(c, 0x80);
            return;
        }
        if class > 3 {
            let mut ptrs = Vec::new();
            for _ in 0..2 + r.next_u32() % 8 {
                ptrs.push(dlheap_fix_malloc(c, 8 + r.next_u32() % 0x400));
            }
            for (i, &p) in ptrs.iter().enumerate() {
                if r.next_u32() % 2 == 0 && i + 1 < ptrs.len() {
                    dlheap_fix_free(c, p);
                }
            }
            return;
        }
        let mut fast = Vec::new();
        for n in [0x18u32, 0x18, 0x18, 0x38] {
            fast.push(dlheap_fix_malloc(c, n));
        }
        let mut big = Vec::new();
        for n in [0x100u32, 0x100, 0x200, 0x300] {
            big.push(dlheap_fix_malloc(c, n));
            dlheap_fix_malloc(c, 0x40); // guard: no merges
        }
        for &p in big.iter() {
            dlheap_fix_free(c, p);
        }
        if class == 2 {
            dlheap_fix_malloc(c, 0x60); // sorts the unsorted chunks into bins
        }
        for &p in fast.iter() {
            dlheap_fix_free(c, p);
        }
    }

    pub(crate) fn dlheap_mallinfo_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0031_fc20, 0);
        let heap = dlheap_fix_arena(c);
        let class = if seed < 3 * DLHEAP_MALLINFO_CLASSES {
            seed % DLHEAP_MALLINFO_CLASSES
        } else {
            4
        };
        dlheap_mallinfo_layout(c, &mut r, class);
        for i in 0..12 {
            c.bus
                .write32(DLHEAP_MALLINFO_OUT + 4 * i, 0xdddd_0000 + i + seed);
        }
        c.g[A0][0] = DLHEAP_MALLINFO_OUT as u64;
        c.g[A1][0] = heap as u64;
        c.pc = 0x0031_fc20;
    }

    /// `DLHeap_Mallinfo`: fresh (Consolidate), empty, fast-bin and
    /// unsorted / small-bin walks over a real arena; the prefilled struct
    /// and the stack copy compared. Not reachable: the self-linked-chunk
    /// exits (`p == p->bk` implies `p == p->bk->bk`, tested first).
    /// Paths: 1086 class 3 (derived: 11 empty fast bins, 95 empty bins);
    /// every class total, in class order.
    #[test]
    fn dlheap_mallinfo_matches() {
        check_paths(
            ee::s10::f_0031fc20__generated,
            dlheap_mallinfo,
            40,
            &[1786, 1169, 1154, 1086],
            &|c, seed| dlheap_mallinfo_fixture(c, seed),
        );
    }

    driven!(dlheap_mallinfo_gen, ee::s10::f_0031fc20__generated);

    driven!(dlheap_mallinfo_hand, dlheap_mallinfo);

    /// `DLHeap_Mallinfo` under delivered events (initialised arenas only,
    /// so no Consolidate): the fast-bin and bin loops' back edges.
    #[test]
    fn dlheap_mallinfo_events() {
        check_events(dlheap_mallinfo_gen, dlheap_mallinfo_hand, 60, &|c, seed| {
            dlheap_mallinfo_fixture(c, 1 + seed % 3 + 4 * (seed / 3));
            let mut r = Rng(seed as u64 ^ 0x31fc20);
            schedule(c, &mut r, seed, 0x0031_fc20)
        });
    }
}
