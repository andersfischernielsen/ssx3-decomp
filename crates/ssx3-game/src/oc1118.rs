//! Staging module oc1118 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1118`.

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
    (
        0x0040_3500,
        libmpeg_add_bs_dma_chain,
        "libmpeg_AddBsDmaChain",
    ),
    (
        0x0040_3600,
        libmpeg_set_picture_size,
        "libmpeg_SetPictureSize",
    ),
    (
        0x0040_3bb0,
        libmpeg_sequence_extension,
        "libmpeg_SequenceExtension",
    ),
    (
        0x0040_3ce0,
        libmpeg_sequence_display_extension,
        "libmpeg_SequenceDisplayExtension",
    ),
    (
        0x0040_3dd8,
        libmpeg_restart_dma_callback,
        "libmpeg_RestartDmaCallback",
    ),
    (
        0x0040_5b48,
        libmpeg_set_ipu_mpeg1_bit,
        "libmpeg_SetIpuMpeg1Bit",
    ),
];

// Direct-call targets of this module's functions (guest address, readable alias
// in the generated shard).
const LIBMPEG_CALLEE_DISPATCH_CALLBACK: Callee = Callee {
    addr: 0x0040_2c30,
    f: ee::s15::libmpeg_DispatchCallback,
};
const LIBMPEG_CALLEE_ERROR: Callee = Callee {
    addr: 0x0040_34a8,
    f: ee::s15::libmpeg_Error,
};
const LIBMPEG_CALLEE_IPU_COMMAND: Callee = Callee {
    addr: 0x0040_6df0,
    f: ee::s15::libmpeg_IpuCommand,
};
const LIBMPEG_CALLEE_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};
const LIBMPEG_CALLEE_GET_BITS: Callee = Callee {
    addr: 0x0040_72e8,
    f: ee::s15::libmpeg_GetBits,
};
const LIBMPEG_CALLEE_SET_IPU_MPEG1_BIT: Callee = Callee {
    addr: 0x0040_5b48,
    f: ee::s15::libmpeg_SetIpuMpeg1Bit,
};
const LIBMPEG_CALLEE_IPU_RESTART_DMA: Callee = Callee {
    addr: 0x0040_94e8,
    f: ee::s15::sceIpuRestartDMA,
};
const LIBMPEG_CALLEE_FLUSH_CACHE: Callee = Callee {
    addr: 0x0042_4020,
    f: ee::s15::FlushCache,
};
const LIBMPEG_CALLEE_DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::DIntr,
};
const LIBMPEG_CALLEE_EINTR: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::EIntr,
};

/// `void libmpeg_AddBsDmaChain(base, addr, n)` (0x403500): build a DMA tag chain
/// at `base` (`(base + 0x88f)` rounded down to 16 bytes): one 16-byte tag per
/// chunk of at most 0xfff40 bytes, with id 3 on every chunk but the last, then
/// flush the cache, mask interrupts around the channel start (`0x105` to the
/// channel control), and restore the interrupt state.
pub fn libmpeg_add_bs_dma_chain(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let f = Frame::open(c, 0x20);
        let a = f.at(c, 0x00);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x88f));
        c.g[A0][0] = sx((c.g[A0][0] as u32) >> 4);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 4);
        f.save_ra(c, 0x10);
        c.g[T3][0] = c.g[A1][0];
        c.g[T0][0] = c.g[A2][0];
        c.g[T2][0] = c.g[S0][0];
        if (c.g[T0][0] as i64) > 0 {
            c.cycles += 6;
            c.g[T1][0] = 0x000f_ff40;
            c.g[T4][0] = 0x0fff_ffff;
            c.g[T6][0] = 3;
            c.g[T5][0] = u64::MAX;
            loop {
                c.cycles += 23;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                c.g[A3][0] = c.g[T1][0];
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[T0][0];
                }
                c.g[V1][0] = 0;
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xf));
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1e));
                c.g[A1][0] = ((c.g[T5][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[A2][0];
                }
                c.g[A0][0] = c.g[T3][0] & c.g[T4][0];
                if c.g[T0][0] != 0 {
                    c.g[V1][0] = c.g[T6][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                c.g[A0][0] <<= 32;
                c.g[V1][0] <<= 28;
                c.g[V0][0] <<= 32;
                c.g[A0][0] |= c.g[V1][0];
                c.g[V0][0] >>= 32;
                c.g[A0][0] |= c.g[V0][0];
                let a = c.g[T2][0] as u32;
                let v = c.g[A0][0];
                c.sd(a, v);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A3][0] as u32));
                let more = (c.g[T0][0] as i64) > 0;
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_3540);
                }
            }
        }
        c.cycles += 19;
        c.g[A0][0] = 0;
        LIBMPEG_CALLEE_FLUSH_CACHE.call(c, 0x0040_35a4)?;
        LIBMPEG_CALLEE_DINTR.call(c, 0x0040_35ac)?;
        c.g[V1][0] = 0x0fff_ffff;
        c.g[A1][0] = 0x1000_b430;
        c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
        c.g[A0][0] = 0x1000_0000;
        let a = c.g[A1][0] as u32;
        let v = c.g[V1][0];
        Addr(a).sw(c, v);
        c.g[A0][0] |= 0xb420;
        let a = c.g[A0][0] as u32;
        Addr(a).sw(c, 0);
        c.g[V1][0] = 0x1000_b400;
        c.g[A0][0] = 0x105;
        let a = c.g[V1][0] as u32;
        let v = c.g[A0][0];
        Addr(a).sw(c, v);
        f.restore_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let a = f.at(c, 0x00);
            c.g[S0][0] = Addr(a).ld(c);
            f.close(c);
        } else {
            c.cycles += 3;
            let a = f.at(c, 0x00);
            c.g[S0][0] = Addr(a).ld(c);
            c.g[SP][0] = sx(sp(c).wrapping_add(f.size));
            c.pc = 0x0042_c0c0;
            ee::s15::EIntr(c);
        }
        Ok(())
    })
}

/// `void libmpeg_SetPictureSize(pic, width, height)` (0x403600): store the
/// size and the size in 16-pixel macroblocks (`>> 4`) into the picture record
/// (+4 width, +8 height, +0xc/+0x10 macroblocks); returns 1.
pub fn libmpeg_set_picture_size(c: &mut Cpu) {
    c.cycles += 8;
    let pic = c.g[A0][0] as u32;
    let width = c.g[A1][0];
    let height = c.g[A2][0];
    c.g[V0][0] = sx(((width as u32) as i32 >> 4) as u32);
    c.g[V1][0] = sx(((height as u32) as i32 >> 4) as u32);
    let v = c.g[V0][0];
    Addr(pic.wrapping_add(0xc)).sw(c, v);
    let v = c.g[V1][0];
    Addr(pic.wrapping_add(0x10)).sw(c, v);
    c.g[V0][0] = 1;
    Addr(pic.wrapping_add(0x4)).sw(c, width);
    Addr(pic.wrapping_add(0x8)).sw(c, height);
    ret(c);
}

/// `void libmpeg_SetupFrameBuffers(y, uBuffer, vBuffer, ..., w, height)`
/// (0x4039f0): stores the Y, U and V output pointers of three frame-buffer
/// sets (the first two at the plane start, the third advanced by
/// `w * height / 512` in the reference). Its arguments are partly on the
/// stack (the 9th to 14th words); the register form follows the generated
/// body exactly, including the 32-bit multiplies (`mult`) and the scratch
/// registers they leave.
#[allow(dead_code)]
pub fn libmpeg_setup_frame_buffers(c: &mut Cpu) {
    c.cycles += 56;
    let f = Frame::open(c, 0x50);
    c.g[T6][0] = 0x0fff_0000;
    let a = f.at(c, 0x70);
    c.g[V1][0] = Addr(a).lw(c);
    c.g[T6][0] |= 0xffff;
    let a = f.at(c, 0x00);
    let v = c.g[S0][0];
    c.sd(a, v);
    c.mult(0, 3, 3, 2);
    c.g[S0][0] = u64::MAX;
    let a = f.at(c, 0x20);
    let v = c.g[S2][0];
    c.sd(a, v);
    c.g[S2][0] = 0x180;
    let a = f.at(c, 0x58);
    c.g[T4][0] = Addr(a).lw(c);
    let a = f.at(c, 0x10);
    let v = c.g[S1][0];
    c.sd(a, v);
    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ff));
    if c.g[S0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[S1][0] = c.g[T4][0] & c.g[T6][0];
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
    let a = f.at(c, 0x30);
    let v = c.g[S3][0];
    c.sd(a, v);
    c.mult(0, 3, 2, 18);
    let a = f.at(c, 0x60);
    c.g[T5][0] = Addr(a).lw(c);
    c.g[S3][0] = 0x2000_0000;
    let a = f.at(c, 0x40);
    let v = c.g[S4][0];
    c.sd(a, v);
    let a = f.at(c, 0x68);
    c.g[S4][0] = Addr(a).lw(c);
    c.g[S1][0] |= c.g[S3][0];
    c.g[T7][0] = c.g[T5][0] & c.g[T6][0];
    let a = c.g[A0][0] as u32;
    let v = c.g[S1][0];
    Addr(a).sw(c, v);
    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
    c.g[T7][0] |= c.g[S3][0];
    c.mult(0, 3, 2, 18);
    c.mult(0, 4, 2, 18);
    let a = c.g[A1][0] as u32;
    let v = c.g[T7][0];
    Addr(a).sw(c, v);
    c.g[T4][0] &= c.g[T6][0];
    c.g[T4][0] |= c.g[S3][0];
    let a = f.at(c, 0x20);
    c.g[S2][0] = Addr(a).ld(c);
    let a = f.at(c, 0x00);
    c.g[S0][0] = Addr(a).ld(c);
    c.g[T5][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[V1][0] = c.g[S4][0] & c.g[T6][0];
    c.g[T5][0] &= c.g[T6][0];
    c.g[V1][0] |= c.g[S3][0];
    c.g[V0][0] &= c.g[T6][0];
    let a = c.g[A2][0] as u32;
    let v = c.g[V1][0];
    Addr(a).sw(c, v);
    c.g[T5][0] |= c.g[S3][0];
    let a = c.g[A3][0] as u32;
    let v = c.g[S1][0];
    Addr(a).sw(c, v);
    c.g[V0][0] |= c.g[S3][0];
    let a = c.g[T0][0] as u32;
    let v = c.g[T7][0];
    Addr(a).sw(c, v);
    let a = c.g[T1][0] as u32;
    let v = c.g[V1][0];
    Addr(a).sw(c, v);
    let a = c.g[T2][0] as u32;
    let v = c.g[T4][0];
    Addr(a).sw(c, v);
    let a = f.at(c, 0x50);
    c.g[V1][0] = Addr(a).lw(c);
    let a = c.g[T3][0] as u32;
    let v = c.g[T5][0];
    Addr(a).sw(c, v);
    let a = f.at(c, 0x40);
    c.g[S4][0] = Addr(a).ld(c);
    let a = f.at(c, 0x30);
    c.g[S3][0] = Addr(a).ld(c);
    let a = f.at(c, 0x10);
    c.g[S1][0] = Addr(a).ld(c);
    let a = c.g[V1][0] as u32;
    let v = c.g[V0][0];
    Addr(a).sw(c, v);
    f.close(c);
}

/// `void libmpeg_LoadIqMatrix(dec, srcAddr, ipuCmd)` (0x403ad0): wait for the
/// dispatch callback and the IPU, start the IPU source DMA at `srcAddr` with
/// interrupts masked around the channel setup, run the IPU command `ipuCmd`,
/// wait again and dispatch the final callback (state 3).
#[allow(dead_code)]
pub fn libmpeg_load_iq_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let f = Frame::open(c, 0x60);
        c.g[V0][0] = 2;
        let a = f.at(c, 0x40);
        let v = c.g[S2][0];
        c.sd(a, v);
        let a = f.at(c, 0x30);
        let v = c.g[S1][0];
        c.sd(a, v);
        c.g[S2][0] = c.g[A1][0];
        let a = f.at(c, 0x20);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[S1][0] = c.g[A0][0];
        let a = f.at(c, 0x50);
        let v = c.g[RA][0];
        c.sd(a, v);
        c.g[S0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[SP][0];
        let a = (c.g[S1][0] as u32).wrapping_add(0x858);
        c.g[A0][0] = Addr(a).lw(c);
        let a = f.at(c, 0x00);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        LIBMPEG_CALLEE_DISPATCH_CALLBACK.call(c, 0x0040_3b04)?;
        c.g[A0][0] = c.g[S1][0];
        LIBMPEG_CALLEE_IPU_WAIT_BUSY.call(c, 0x0040_3b0c)?;
        c.g[V0][0] = 0x1000_2000;
        c.g[A0][0] = c.g[S1][0];
        Addr(0x1000_2000).sw(c, 0);
        LIBMPEG_CALLEE_IPU_WAIT_BUSY.call(c, 0x0040_3b20)?;
        LIBMPEG_CALLEE_DINTR.call(c, 0x0040_3b28)?;
        c.g[V1][0] = 0x0fff_ffff;
        c.g[A0][0] = 0x1000_b410;
        c.g[S0][0] &= c.g[V1][0];
        c.g[A1][0] = 0x1000_0000;
        let a = c.g[A0][0] as u32;
        let v = c.g[S0][0];
        Addr(a).sw(c, v);
        c.g[A1][0] |= 0xb420;
        c.g[A0][0] = 4;
        c.g[V1][0] = 0x1000_0000;
        let a = c.g[A1][0] as u32;
        let v = c.g[A0][0];
        Addr(a).sw(c, v);
        c.g[V1][0] |= 0xb400;
        c.g[A0][0] = 0x101;
        let a = c.g[V1][0] as u32;
        let v = c.g[A0][0];
        Addr(a).sw(c, v);
        if c.g[V0][0] == 0 {
            c.g[A1][0] = c.g[S2][0];
        } else {
            c.cycles += 3;
            LIBMPEG_CALLEE_EINTR.call(c, 0x0040_3b70)?;
            c.g[A1][0] = c.g[S2][0];
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S1][0];
        LIBMPEG_CALLEE_IPU_COMMAND.call(c, 0x0040_3b7c)?;
        c.g[A0][0] = c.g[S1][0];
        LIBMPEG_CALLEE_IPU_WAIT_BUSY.call(c, 0x0040_3b84)?;
        let a = (c.g[S1][0] as u32).wrapping_add(0x858);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[V0][0] = 3;
        let a = f.at(c, 0x00);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        c.g[A1][0] = c.g[SP][0];
        LIBMPEG_CALLEE_DISPATCH_CALLBACK.call(c, 0x0040_3b98)?;
        f.restore_ra(c, 0x50);
        let a = f.at(c, 0x40);
        c.g[S2][0] = Addr(a).ld(c);
        let a = f.at(c, 0x30);
        c.g[S1][0] = Addr(a).ld(c);
        let a = f.at(c, 0x20);
        c.g[S0][0] = Addr(a).ld(c);
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_SequenceExtension(dec)` (0x403bb0): clear the IPU MPEG-1 bit,
/// read the 28-bit sequence-extension word, check its profile field (bits
/// 17-18 must be 1, else `libmpeg_Error`), keep the fields in the decoder
/// (+0x140, +0x124..+0x138 accumulators), and read the 16-bit chroma/level
/// word whose code (bits 20-27 of the first word, 0x48/0x58/0x44) is checked
/// the same way.
pub fn libmpeg_sequence_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let f = Frame::open(c, 0x70);
        let a = f.at(c, 0x10);
        let v = c.g[S1][0];
        c.sd(a, v);
        let a = f.at(c, 0x00);
        let v = c.g[S0][0];
        c.sd(a, v);
        c.g[S1][0] = c.g[A0][0];
        let a = f.at(c, 0x50);
        let v = c.g[S5][0];
        c.sd(a, v);
        c.g[S0][0] = 1;
        let a = f.at(c, 0x40);
        let v = c.g[S4][0];
        c.sd(a, v);
        c.g[A0][0] = 0;
        let a = f.at(c, 0x30);
        let v = c.g[S3][0];
        c.sd(a, v);
        let a = f.at(c, 0x20);
        let v = c.g[S2][0];
        c.sd(a, v);
        f.save_ra(c, 0x60);
        let a = (c.g[S1][0] as u32).wrapping_add(0x848);
        let v = c.g[S0][0];
        Addr(a).sw(c, v);
        LIBMPEG_CALLEE_SET_IPU_MPEG1_BIT.call(c, 0x0040_3be4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1c;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3bf0)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[S2][0] as u32) >> 1);
        c.g[V0][0] = sx((c.g[S2][0] as u32) >> 17);
        c.g[S5][0] = c.g[V1][0] & 0xfff;
        c.g[V0][0] &= 3;
        c.g[A0][0] = sx((c.g[S2][0] as u32) >> 13);
        c.g[V1][0] = sx((c.g[S2][0] as u32) >> 15);
        c.g[S4][0] = c.g[A0][0] & 3;
        c.g[S3][0] = c.g[V1][0] & 3;
        let t = c.g[V0][0] == c.g[S0][0];
        let a = (c.g[S1][0] as u32).wrapping_add(0x140);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        if !t {
            c.cycles += 4;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7d10));
            LIBMPEG_CALLEE_ERROR.call(c, 0x0040_3c2c)?;
        }
        c.cycles += 11;
        c.g[V0][0] = sx((c.g[S2][0] as u32) >> 19);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] &= 1;
        c.g[A1][0] = 0x10;
        let a = (c.g[S1][0] as u32).wrapping_add(0x13c);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        c.g[S0][0] = sx((c.g[S2][0] as u32) >> 20);
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3c48)?;
        c.g[S2][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[V0][0] = 0x48;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x58;
        if !t {
            c.cycles += 2;
            let t = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = 0x44;
            if !t {
                c.cycles += 2;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[A1][0] = 0x490000;
                if !t {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7d38));
                    LIBMPEG_CALLEE_ERROR.call(c, 0x0040_3c74)?;
                }
            }
        }
        c.cycles += 27;
        let a = (c.g[S1][0] as u32).wrapping_add(0x124);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[T0][0] = sx((c.g[S5][0] as u32) << 18);
        let a = (c.g[S1][0] as u32).wrapping_add(0x128);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[T1][0] = sx((c.g[S2][0] as u32) << 10);
        let a = (c.g[S1][0] as u32).wrapping_add(0x134);
        c.g[A2][0] = Addr(a).lw(c);
        c.g[A3][0] = sx((c.g[S3][0] as u32) << 12);
        let a = (c.g[S1][0] as u32).wrapping_add(0x138);
        c.g[V0][0] = Addr(a).lw(c);
        c.g[A1][0] = sx((c.g[S4][0] as u32) << 12);
        c.g[A0][0] &= 0xfff;
        c.g[V1][0] &= 0xfff;
        c.g[A3][0] |= c.g[A0][0];
        c.g[A1][0] |= c.g[V1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        let a = (c.g[S1][0] as u32).wrapping_add(0x138);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        let a = (c.g[S1][0] as u32).wrapping_add(0x124);
        let v = c.g[A3][0];
        Addr(a).sw(c, v);
        let a = (c.g[S1][0] as u32).wrapping_add(0x128);
        let v = c.g[A1][0];
        Addr(a).sw(c, v);
        let a = (c.g[S1][0] as u32).wrapping_add(0x134);
        let v = c.g[A2][0];
        Addr(a).sw(c, v);
        f.restore_ra(c, 0x60);
        let a = f.at(c, 0x50);
        c.g[S5][0] = Addr(a).ld(c);
        let a = f.at(c, 0x40);
        c.g[S4][0] = Addr(a).ld(c);
        let a = f.at(c, 0x30);
        c.g[S3][0] = Addr(a).ld(c);
        let a = f.at(c, 0x20);
        c.g[S2][0] = Addr(a).ld(c);
        let a = f.at(c, 0x10);
        c.g[S1][0] = Addr(a).ld(c);
        let a = f.at(c, 0x00);
        c.g[S0][0] = Addr(a).ld(c);
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_SequenceDisplayExtension(dec)` (0x403ce0): read the display
/// extension: a 1-bit flag, then three 8-bit colour fields (+0x144) when set,
/// then the 14-bit frame width (+0x148), a 1-bit flag and a 14-bit height
/// (+0x14c).
pub fn libmpeg_sequence_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 3;
        let a = f.at(c, 0x00);
        let v = c.g[S0][0];
        c.sd(a, v);
        f.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d04)?;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            for ret_addr in [0x0040_3d14u32, 0x0040_3d20, 0x0040_3d2c] {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 8;
                LIBMPEG_CALLEE_GET_BITS.call(c, ret_addr)?;
            }
            let a = (c.g[S0][0] as u32).wrapping_add(0x144);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xe;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d3c)?;
        let a = (c.g[S0][0] as u32).wrapping_add(0x148);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xe;
        LIBMPEG_CALLEE_GET_BITS.call(c, 0x0040_3d58)?;
        let a = (c.g[S0][0] as u32).wrapping_add(0x14c);
        let v = c.g[V0][0];
        Addr(a).sw(c, v);
        f.restore_ra(c, 0x10);
        let a = f.at(c, 0x00);
        c.g[S0][0] = Addr(a).ld(c);
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_RestartDmaCallback(dec)` (0x403dd8): restart the IPU source
/// DMA of the decoder's stream block (`dec+0x40`, at +0x4c), return 1.
pub fn libmpeg_restart_dma_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let f = Frame::open(c, 0x10);
        f.save_ra(c, 0x00);
        let a = (c.g[A0][0] as u32).wrapping_add(0x40);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
        LIBMPEG_CALLEE_IPU_RESTART_DMA.call(c, 0x0040_3dec)?;
        f.restore_ra(c, 0x00);
        c.g[V0][0] = 1;
        f.close(c);
        Ok(())
    })
}

/// `void libmpeg_SetIpuMpeg1Bit(flag)` (0x405b48): write the argument's low
/// bits (`flag << 23`) into the IPU control word at `0x10002010`, keeping
/// every other bit except bit 23, which takes the flag.
pub fn libmpeg_set_ipu_mpeg1_bit(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A1][0] = 0x1000_0000;
    c.g[V1][0] = 0xffff_ffff_ff7f_0000;
    c.g[A1][0] |= 0x2010;
    c.g[V1][0] |= 0xffff;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 23);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A0][0];
    let a = c.g[A1][0] as u32;
    let v = c.g[V0][0];
    Addr(a).sw(c, v);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::SP as STACK_TOP;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the test-only `libmpeg_GetBits` fake: the fake answers only
    /// in a machine that wrote this magic here.
    const OC1118_GATE: u32 = DATA + 0x000f_1118;
    const OC1118_MAGIC: u32 = 0x1118_6e75;
    /// Number of fake calls so far in this machine.
    const OC1118_CALLS: u32 = DATA + 0x000f_1120;
    /// The queue of words the fake returns, one per call (16 words).
    const OC1118_WORDS: u32 = DATA + 0x000f_1180;
    /// Previous override-slot body, run when the gate is off.
    static OC1118_PREV_GET_BITS: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_GetBits(dec, nbits)` fake: returns the next queued word masked
    /// to `nbits`; with the gate off it runs the real body.
    fn oc1118_get_bits_fake(c: &mut Cpu) {
        if c.bus.read32(OC1118_GATE) != OC1118_MAGIC {
            return fakes::fall_back(c, &OC1118_PREV_GET_BITS, ee::s15::f_004072e8__generated);
        }
        let k = c.bus.read32(OC1118_CALLS);
        c.bus.write32(OC1118_CALLS, k.wrapping_add(1));
        let bits = c.g[A1][0] as u32;
        let word = c.bus.read32(OC1118_WORDS + 4 * (k & 15));
        let mask = if bits >= 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };
        c.g[V0][0] = sx(word & mask);
        ret(c);
    }

    fn oc1118_hook_get_bits() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1118_get_bits_fake,
            &OC1118_PREV_GET_BITS,
        );
    }

    /// Arm the GetBits fake in this machine with a queue of words.
    fn oc1118_get_bits_queue(c: &mut Cpu, words: &[u32]) {
        c.bus.write32(OC1118_GATE, OC1118_MAGIC);
        c.bus.write32(OC1118_CALLS, 0);
        for (i, w) in words.iter().enumerate() {
            c.bus.write32(OC1118_WORDS + 4 * i as u32, *w);
        }
    }

    /// The MPEG error path prints nothing: its format words are NULled (as in
    /// `game/lib/mpeg.rs`).
    fn oc1118_quiet_printf(c: &mut Cpu) {
        c.bus.write32(0x0049_7c68, 0);
        c.bus.write32(0x0049_7ce8, 0);
    }

    #[test]
    fn libmpeg_set_picture_size_matches() {
        check(
            ee::s15::f_00403600__generated,
            libmpeg_set_picture_size,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3600 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_set_ipu_mpeg1_bit_matches() {
        check(
            ee::s15::f_00405b48__generated,
            libmpeg_set_ipu_mpeg1_bit,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_5b48 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = r.next_u32() as u64;
                c.bus.write32(0x1000_2010, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_restart_dma_callback_matches() {
        check(
            ee::s15::f_00403dd8__generated,
            libmpeg_restart_dma_callback,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3dd8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x40, DATA + 0x200);
                // sceIpuRestartDMA's count word (+0x10 of the block at +0x4c) is 0:
                // its short path.
                c.bus.write32(DATA + 0x200 + 0x4c + 0x10, 0);
                c.bus.write32(DATA + 0x200 + 0x4c + 0x1c, r.next_u32());
                c.bus.write32(DATA + 0x200 + 0x4c, r.next_u32());
            },
        );
    }

    #[test]
    fn libmpeg_add_bs_dma_chain_matches() {
        check(
            ee::s15::f_00403500__generated,
            libmpeg_add_bs_dma_chain,
            200,
            &|c, seed| {
                // The machine's syscall hook answers FlushCache.
                let _ = fakes::machine(c, seed, 0x0040_3500, 0);
                let mut r = Rng(0x0040_3500 ^ (seed as u64) << 16);
                c.g[A0][0] = (DATA + 0x1000 * (seed % 8)) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                let n = match seed % 3 {
                    0 => 0,
                    1 => 0x0010_0000 + r.next_u32() % 0x1000,
                    _ => r.next_u32() % 0x0004_0000 + 1,
                };
                c.g[A2][0] = sx(n);
                c.cop0[12] = if seed % 2 == 0 { 0x0001_0000 } else { 0 };
            },
        );
    }

    #[test]
    fn libmpeg_sequence_extension_matches() {
        oc1118_hook_get_bits();
        check(
            ee::s15::f_00403bb0__generated,
            libmpeg_sequence_extension,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3bb0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1118_quiet_printf(c);
                // First word (28 bits): bits 17-18 == 1 take the no-error leg
                // (seed % 5 != 0); bits 20-27 select the case.
                let code = [0x48u32, 0x58, 0x44, 0x10][(seed as usize / 5) % 4];
                let mut w0 = r.next_u32();
                w0 &= !(3u32 << 17);
                if seed % 5 != 0 {
                    w0 |= 1u32 << 17;
                }
                w0 &= !(0xffu32 << 20);
                w0 |= code << 20;
                let w1 = r.next_u32();
                oc1118_get_bits_queue(c, &[w0, w1]);
            },
        );
    }

    #[test]
    fn libmpeg_sequence_display_extension_matches() {
        oc1118_hook_get_bits();
        check(
            ee::s15::f_00403ce0__generated,
            libmpeg_sequence_display_extension,
            200,
            &|c, seed| {
                let mut r = Rng(0x0040_3ce0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let words: [u32; 16] = std::array::from_fn(|_| r.next_u32());
                oc1118_get_bits_queue(c, &words);
            },
        );
    }
}
