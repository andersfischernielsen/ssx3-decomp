//! Staging module oc1414 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1414`.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `Serialize_ReadU32` (`0x002d2510`).
    pub(crate) const SERIALIZE_READ_U32: Callee = Callee {
        addr: 0x002d_2510,
        f: ee::s09::Serialize_ReadU32,
    };

    /// `Serialize_WriteU32_2` (`0x002d2210`).
    pub(crate) const SERIALIZE_WRITE_U32_2: Callee = Callee {
        addr: 0x002d_2210,
        f: ee::s09::Serialize_WriteU32_2,
    };

    /// `Serialize_WriteU8` (`0x002d23c8`).
    pub(crate) const SERIALIZE_WRITE_U8: Callee = Callee {
        addr: 0x002d_23c8,
        f: ee::s09::Serialize_WriteU8,
    };

    /// `MD5_Init` (`0x0031d368`).
    pub(crate) const MD5_INIT: Callee = Callee {
        addr: 0x0031_d368,
        f: ee::s10::MD5_Init,
    };

    /// `MD5_Update` (`0x0031d3a8`).
    pub(crate) const MD5_UPDATE: Callee = Callee {
        addr: 0x0031_d3a8,
        f: ee::s10::MD5_Update,
    };

    /// `MD5_Final` (`0x0031d4d0`).
    pub(crate) const MD5_FINAL: Callee = Callee {
        addr: 0x0031_d4d0,
        f: ee::s10::MD5_Final,
    };
}

/// `int Serialize_ReadQuat(void *stream, float *q)` (0x2d26f8): read 4
/// words into `q`, returning their sum.
pub fn serialize_read_quat(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x50);
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx(frame.at(c, 0x10));
        SERIALIZE_READ_U32.call(c, 0x002d_271c)?;
        c.g[A1][0] = sx(frame.at(c, 0x14));
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        SERIALIZE_READ_U32.call(c, 0x002d_272c)?;
        c.g[A1][0] = sx(frame.at(c, 0x18));
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        SERIALIZE_READ_U32.call(c, 0x002d_273c)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(frame.at(c, 0x1c));
        SERIALIZE_READ_U32.call(c, 0x002d_274c)?;
        c.lwc1(3, frame.at(c, 0x10));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lwc1(0, frame.at(c, 0x14));
        c.lwc1(1, frame.at(c, 0x18));
        c.lwc1(2, frame.at(c, 0x1c));
        c.swc1(3, frame.at(c, 0xc));
        c.swc1(0, frame.at(c, 0));
        c.swc1(1, frame.at(c, 0x4));
        c.swc1(2, frame.at(c, 0x8));
        frame.restore(c, 16, 0x50);
        c.lq(3, frame.at(c, 0));
        frame.restore(c, 17, 0x40);
        c.sq(3, c.g[S2][0] as u32);
        frame.restore(c, 18, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int Serialize_WriteDataBlock(void *stream, void *block)` (0x2d2478):
/// write the block's word then its bytes, returning the total written.
pub fn serialize_write_data_block(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_2478 => {
                    c.cycles += 17;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    c.g[S0][0] = 0u64;
                    frame.save(c, 18, 0x20);
                    frame.save(c, 19, 0x10);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[S3][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    SERIALIZE_WRITE_U32_2.call(c, 0x002d_24a8)?;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S1][0];
                    if t {
                        pc = 0x002d_24f0;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    pc = 0x002d_24c0;
                    continue;
                }
                0x002d_24c0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A1][0] = c.lb(c.g[V0][0] as u32);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    SERIALIZE_WRITE_U8.call(c, 0x002d_24d4)?;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_24c0);
                        }
                        pc = 0x002d_24c0;
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x002d_24f0;
                    continue;
                }
                0x002d_24f0 => {
                    c.cycles += 7;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_2478, 0x002d_24f0);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_WriteString(void *stream, char *s)` (0x2d23e8): write
/// the string bytes and its terminator, returning the length plus one.
pub fn serialize_write_string(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_23e8 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x60);
                    frame.save(c, 19, 0x20);
                    frame.save(c, 16, 0x50);
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, 17, 0x40);
                    c.g[S0][0] = c.g[S3][0];
                    frame.save(c, 18, 0x30);
                    c.g[S1][0] = 0u64;
                    frame.save(c, 20, 0x10);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S4][0] = 0u64;
                    pc = 0x002d_2434;
                    continue;
                }
                0x002d_241c => {
                    c.cycles += 1;
                    pc = 0x002d_2420;
                    continue;
                }
                0x002d_2420 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    SERIALIZE_WRITE_U8.call(c, 0x002d_2430)?;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    pc = 0x002d_2434;
                    continue;
                }
                0x002d_2434 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_6810, 0x002d_243c)?;
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = c.lb(c.g[S0][0] as u32);
                        if c.cycles >= c.next_event {
                            c.pc = 0x002d_2420;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x002d_2420;
                        continue;
                    }
                    pc = 0x002d_2448;
                    continue;
                }
                0x002d_2448 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0u64;
                    SERIALIZE_WRITE_U8.call(c, 0x002d_2454)?;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                    let frame = Frame { size: 0x60 };
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore(c, 20, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_23e8, 0x002d_2478);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_ReadVec3(void *stream, float *v)` (0x2d25a0): read 3
/// words into `v`, returning their sum.
pub fn serialize_read_vec3(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_25a0 => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x70);
                    frame.save(c, 16, 0x60);
                    frame.save(c, 17, 0x50);
                    c.g[S0][0] = c.g[SP][0];
                    frame.save(c, 18, 0x40);
                    c.g[S1][0] = 0x2;
                    frame.save(c, 19, 0x30);
                    c.g[S2][0] = 0u64;
                    frame.save(c, 20, 0x20);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.g[S4][0] = c.g[A1][0];
                    pc = 0x002d_25d0;
                    continue;
                }
                0x002d_25d0 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    SERIALIZE_READ_U32.call(c, 0x002d_25dc)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_25d0);
                        }
                        pc = 0x002d_25d0;
                        continue;
                    }
                    c.cycles += 15;
                    let frame = Frame { size: 0x70 };
                    c.ldl(3, frame.at(c, 0x7));
                    c.ldr(3, frame.at(c, 0));
                    c.g[A0][0] = c.lw(frame.at(c, 0x8));
                    c.sdl(3, c.g[S4][0] as u32 + 0x7);
                    c.sdr(3, c.g[S4][0] as u32);
                    c.g[V0][0] = c.g[S2][0];
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    let frame = Frame { size: 0x70 };
                    frame.restore(c, 16, 0x60);
                    frame.restore(c, 17, 0x50);
                    frame.restore(c, 18, 0x40);
                    frame.restore(c, 19, 0x30);
                    frame.restore(c, 20, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_25a0, 0x002d_25fc);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_ReadVec4(void *stream, float *v)` (0x2d2628): read 4
/// words into `v`, returning their sum.
pub fn serialize_read_vec4(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_2628 => {
                    c.cycles += 10;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x20);
                    c.g[S1][0] = 0x3;
                    frame.save(c, 19, 0x10);
                    c.g[S2][0] = 0u64;
                    frame.save_ra(c, 0);
                    c.g[S3][0] = c.g[A0][0];
                    pc = 0x002d_2650;
                    continue;
                }
                0x002d_2650 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    SERIALIZE_READ_U32.call(c, 0x002d_265c)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_2650);
                        }
                        pc = 0x002d_2650;
                        continue;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.g[S2][0];
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_2628, 0x002d_2678);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_ReadMatrix(void *stream, float *m)` (0x2d2690): read 16
/// words into `m`, returning their sum.
pub fn serialize_read_matrix(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_2690 => {
                    c.cycles += 10;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x20);
                    c.g[S1][0] = 0xf;
                    frame.save(c, 19, 0x10);
                    c.g[S2][0] = 0u64;
                    frame.save_ra(c, 0);
                    c.g[S3][0] = c.g[A0][0];
                    pc = 0x002d_26b8;
                    continue;
                }
                0x002d_26b8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    SERIALIZE_READ_U32.call(c, 0x002d_26c4)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_26b8);
                        }
                        pc = 0x002d_26b8;
                        continue;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.g[S2][0];
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_2690, 0x002d_26e0);
                    return Ok(());
                }
            }
        }
    })
}

/// `uint64_t MD5_HashString(const char *s, void *out, ...)` (0x317710):
/// hash `s` with MD5, writing the 128-bit digest to `out` and the first 16
/// bytes over `s`, returning `s`.
pub fn md5_hash_string(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 18, 0x80);
        frame.save(c, 16, 0xa0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x90);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        MD5_INIT.call(c, 0x0031_7738)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x0031_7740)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S0][0];
        MD5_UPDATE.call(c, 0x0031_7750)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(frame.at(c, 0x60));
        MD5_FINAL.call(c, 0x0031_775c)?;
        c.g[V1][0] = c.ld(frame.at(c, 0x68));
        c.g[V0][0] = c.g[S2][0];
        c.g[A0][0] = c.ld(frame.at(c, 0x60));
        c.sd((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sd(c.g[S1][0] as u32, c.g[A0][0]);
        c.sd(c.g[S2][0] as u32, c.g[A0][0]);
        frame.restore(c, 16, 0xa0);
        c.g[V1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x8));
        frame.restore(c, 17, 0x90);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, 18, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_InitState(void *heap, ...)` (0x31e260): thread the heap
/// header's 94 free nodes and publish the heap constants.
pub fn dlheap_init_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0031_e260 => {
                    c.cycles += 4;
                    c.g[T0][0] = c.g[A0][0];
                    c.g[V1][0] = 0x5e;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x38));
                    pc = 0x0031_e270;
                    continue;
                }
                0x0031_e270 => {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_e270);
                        }
                        pc = 0x0031_e270;
                        continue;
                    }
                    c.cycles += 18;
                    c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                    c.g[A1][0] = 0x40000;
                    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x360));
                    c.g[A2][0] = 0x8000;
                    c.g[V1][0] = c.g[V1][0] & 0x3;
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = c.g[A0][0] | 0x1;
                    c.g[V1][0] = c.g[V1][0] | 0x50;
                    c.g[V0][0] = 0x1000;
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x34c), c.g[A1][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x35c), c.g[V0][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x344), c.g[A2][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x360), c.g[A0][0]);
                    c.sw(c.g[T0][0] as u32, c.g[V1][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x348), 0u64);
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x354), 0u64);
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_e260, 0x0031_e2d8);
                    return Ok(());
                }
            }
        }
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_26f8, serialize_read_quat, "Serialize_ReadQuat"),
    (
        0x002d_2478,
        serialize_write_data_block,
        "Serialize_WriteDataBlock",
    ),
    (0x002d_23e8, serialize_write_string, "Serialize_WriteString"),
    (0x002d_25a0, serialize_read_vec3, "Serialize_ReadVec3"),
    (0x0031_7710, md5_hash_string, "MD5_HashString"),
    (0x0031_e260, dlheap_init_state, "DLHeap_InitState"),
    (0x002d_2628, serialize_read_vec4, "Serialize_ReadVec4"),
    (0x002d_2690, serialize_read_matrix, "Serialize_ReadMatrix"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::machine;
    use crate::difftest::{require_paths, Rng, DATA};
    /// A stream over a prefilled byte buffer.
    fn stream_bytes(c: &mut Cpu, r: &mut Rng, words: u32) -> (u32, u32) {
        const STREAM: u32 = DATA + 0x100;
        const BUF: u32 = DATA + 0x200;
        c.bus.write32(STREAM, BUF);
        for i in 0..words {
            c.bus.write32(
                BUF + 4 * i,
                0x1111_0000 + i * 0x0101_0101 + r.next_u32() % 7,
            );
        }
        (STREAM, BUF)
    }

    /// The three read loops share this fixture: a stream of 16 words plus
    /// an out area.
    fn read_fixture(c: &mut Cpu, seed: u32, addr: u32) -> (u32, u32) {
        let mut r = machine(c, seed, addr as u64, 0);
        let (stream, _buf) = stream_bytes(c, &mut r, 16);
        const OUT: u32 = DATA + 0x400;
        for i in 0..0x10u32 {
            c.bus.write32(OUT + 4 * i, 0x2222_0000 + i);
        }
        (stream, OUT)
    }

    /// `Serialize_ReadVec3`: every word lands in the out area on both
    /// sides, with the loop's event check covered by `check_events` below.
    #[test]
    fn serialize_read_vec3_matches() {
        check(
            ee::s09::f_002d25a0__generated,
            serialize_read_vec3,
            60,
            &|c, seed| {
                let (stream, out) = read_fixture(c, seed, 0x002d_25a0);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = out as u64;
                c.pc = 0x002d_25a0;
            },
        );
        require_paths(ee::s09::f_002d25a0__generated, 60, &[99], &|c, seed| {
            let (stream, out) = read_fixture(c, seed, 0x002d_25a0);
            c.g[A0][0] = stream as u64;
            c.g[A1][0] = out as u64;
            c.pc = 0x002d_25a0;
        });
    }

    /// `Serialize_ReadVec4`: 4 words into the out area.
    #[test]
    fn serialize_read_vec4_matches() {
        check(
            ee::s09::f_002d2628__generated,
            serialize_read_vec4,
            60,
            &|c, seed| {
                let (stream, out) = read_fixture(c, seed, 0x002d_2628);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = out as u64;
                c.pc = 0x002d_2628;
            },
        );
        require_paths(ee::s09::f_002d2628__generated, 60, &[114], &|c, seed| {
            let (stream, out) = read_fixture(c, seed, 0x002d_2628);
            c.g[A0][0] = stream as u64;
            c.g[A1][0] = out as u64;
            c.pc = 0x002d_2628;
        });
    }

    /// `Serialize_ReadMatrix`: 16 words into the out area.
    #[test]
    fn serialize_read_matrix_matches() {
        check(
            ee::s09::f_002d2690__generated,
            serialize_read_matrix,
            60,
            &|c, seed| {
                let (stream, out) = read_fixture(c, seed, 0x002d_2690);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = out as u64;
                c.pc = 0x002d_2690;
            },
        );
        require_paths(ee::s09::f_002d2690__generated, 60, &[402], &|c, seed| {
            let (stream, out) = read_fixture(c, seed, 0x002d_2690);
            c.g[A0][0] = stream as u64;
            c.g[A1][0] = out as u64;
            c.pc = 0x002d_2690;
        });
    }

    driven!(read3_gen, ee::s09::f_002d25a0__generated);

    driven!(read3_hand, serialize_read_vec3);
    driven!(read4_gen, ee::s09::f_002d2628__generated);
    driven!(read4_hand, serialize_read_vec4);
    driven!(readmat_gen, ee::s09::f_002d2690__generated);
    driven!(readmat_hand, serialize_read_matrix);

    /// The read loops under delivered events. Each head (0x2d25d0, 0x2d2650,
    /// 0x2d26b8) is structured: the port must carry on, never resume there.
    #[test]
    fn serialize_read_vecs_events() {
        for (gen, hand, addr, head) in [
            (
                read3_gen as fn(&mut Cpu),
                read3_hand as fn(&mut Cpu),
                0x002d_25a0,
                0x002d_25d0,
            ),
            (
                read4_gen as fn(&mut Cpu),
                read4_hand as fn(&mut Cpu),
                0x002d_2628,
                0x002d_2650,
            ),
            (
                readmat_gen as fn(&mut Cpu),
                readmat_hand as fn(&mut Cpu),
                0x002d_2690,
                0x002d_26b8,
            ),
        ] {
            let resumes = check_events(gen, hand, 60, &|c, seed| {
                let (stream, out) = read_fixture(c, seed, addr);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = out as u64;
                schedule(c, &mut Rng(seed as u64), seed, addr)
            });
            assert!(
                resumes.get(&head).is_none(),
                "unwound at structured head {head:#x}: {resumes:x?}"
            );
        }
    }

    /// `Serialize_ReadQuat`: the four words land in the out area, V0 is
    /// their sum.
    #[test]
    fn serialize_read_quat_matches() {
        check(
            ee::s09::f_002d26f8__generated,
            serialize_read_quat,
            100,
            &|c, seed| {
                let (stream, out) = read_fixture(c, seed, 0x002d_26f8);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = out as u64;
                c.pc = 0x002d_26f8;
            },
        );
        require_paths(ee::s09::f_002d26f8__generated, 100, &[106], &|c, seed| {
            let (stream, out) = read_fixture(c, seed, 0x002d_26f8);
            c.g[A0][0] = stream as u64;
            c.g[A1][0] = out as u64;
            c.pc = 0x002d_26f8;
        });
    }

    /// A datablock (`{words pointer, count}` over a byte array) with a
    /// steered count (0 skips the loop).
    fn datablock_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) -> (u32, u32) {
        const STREAM: u32 = DATA + 0x100;
        const BUF: u32 = DATA + 0x200;
        const BLOCK: u32 = DATA + 0x400;
        const WORDS: u32 = DATA + 0x500;
        const BYTES: u32 = DATA + 0x600;
        c.bus.write32(STREAM, BUF);
        for i in 0..0x20u32 {
            c.bus.write32(BUF + 4 * i, 0x3333_0000 + i);
        }
        let n = match seed % 4 {
            0 => 0,
            1 => 1 + r.next_u32() % 7,
            2 => 8 + r.next_u32() % 8,
            _ => 1 + r.next_u32() % 31,
        };
        c.bus.write32(BLOCK, WORDS);
        c.bus.write32(WORDS, n);
        c.bus.write32(WORDS + 4, BYTES);
        for i in 0..n {
            c.bus.write8(BYTES + i, (0x41 + i % 26) as u8);
        }
        (STREAM, BLOCK)
    }

    /// `Serialize_WriteDataBlock`: the count word then each byte lands in
    /// the stream buffer on both sides.
    #[test]
    fn serialize_write_data_block_matches() {
        check(
            ee::s09::f_002d2478__generated,
            serialize_write_data_block,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_2478, 0);
                let (stream, block) = datablock_fixture(c, &mut r, seed);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = block as u64;
                c.pc = 0x002d_2478;
            },
        );
        require_paths(
            ee::s09::f_002d2478__generated,
            100,
            &[
                37, 57, 75, 93, 111, 129, 147, 165, 183, 201, 219, 237, 255, 273, 291, 309, 345,
                363, 381, 453, 471, 543, 561, 579,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_2478, 0);
                let (stream, block) = datablock_fixture(c, &mut r, seed);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = block as u64;
                c.pc = 0x002d_2478;
            },
        );
    }

    driven!(wdb_gen, ee::s09::f_002d2478__generated);
    driven!(wdb_hand, serialize_write_data_block);

    /// The data-block loop under delivered events. The head (0x2d24c0) is
    /// structured: the port must carry on, never resume there.
    #[test]
    fn serialize_write_data_block_events() {
        let resumes = check_events(wdb_gen, wdb_hand, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x002d_2478, 0);
            let (stream, block) = datablock_fixture(c, &mut r, seed);
            c.g[A0][0] = stream as u64;
            c.g[A1][0] = block as u64;
            schedule(c, &mut r, seed, 0x002d_2478)
        });
        assert!(
            resumes.get(&0x002d_24c0).is_none(),
            "unwound at structured head 0x2d24c0: {resumes:x?}"
        );
    }

    /// `Serialize_WriteString`: the bytes and terminator land in the stream
    /// buffer; the length (plus one) is returned.
    #[test]
    fn serialize_write_string_matches() {
        check(
            ee::s09::f_002d23e8__generated,
            serialize_write_string,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_23e8, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const STR: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x40u32 {
                    c.bus.write32(BUF + 4 * i, 0x4444_0000 + i);
                }
                let len = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 7,
                    3 => 31,
                    4 => 32,
                    _ => (r.next_u32() % 40) as usize,
                };
                for i in 0..len {
                    c.bus.write8(STR + i as u32, 0x61 + (i % 26) as u8);
                }
                c.bus.write8(STR + len as u32, 0);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = STR as u64;
                c.pc = 0x002d_23e8;
            },
        );
        require_paths(
            ee::s09::f_002d23e8__generated,
            100,
            &[
                77, 148, 233, 868, 1388, 1597, 1628, 1853, 2092, 2308, 2594, 2612, 2852, 2910,
                3942, 4700, 5197, 5588,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_23e8, 0);
                const STREAM: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x200;
                const STR: u32 = DATA + 0x400;
                c.bus.write32(STREAM, BUF);
                for i in 0..0x40u32 {
                    c.bus.write32(BUF + 4 * i, 0x4444_0000 + i);
                }
                let len = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 7,
                    3 => 31,
                    4 => 32,
                    _ => (r.next_u32() % 40) as usize,
                };
                for i in 0..len {
                    c.bus.write8(STR + i as u32, 0x61 + (i % 26) as u8);
                }
                c.bus.write8(STR + len as u32, 0);
                c.g[A0][0] = STREAM as u64;
                c.g[A1][0] = STR as u64;
                c.pc = 0x002d_23e8;
            },
        );
    }

    driven!(wstr_gen, ee::s09::f_002d23e8__generated);
    driven!(wstr_hand, serialize_write_string);

    /// The string loop under delivered events.
    #[test]
    fn serialize_write_string_events() {
        check_events(wstr_gen, wstr_hand, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x002d_23e8, 0);
            const STREAM: u32 = DATA + 0x100;
            const BUF: u32 = DATA + 0x200;
            const STR: u32 = DATA + 0x400;
            c.bus.write32(STREAM, BUF);
            for i in 0..0x40u32 {
                c.bus.write32(BUF + 4 * i, 0x4444_0000 + i);
            }
            let len = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 7,
                3 => 31,
                4 => 32,
                _ => (r.next_u32() % 40) as usize,
            };
            for i in 0..len {
                c.bus.write8(STR + i as u32, 0x61 + (i % 26) as u8);
            }
            c.bus.write8(STR + len as u32, 0);
            c.g[A0][0] = STREAM as u64;
            c.g[A1][0] = STR as u64;
            schedule(c, &mut r, seed, 0x002d_23e8)
        });
    }

    /// `MD5_HashString`: the digest overwrites the string's first 16 bytes
    /// and the out area on both sides.
    #[test]
    fn md5_hash_string_matches() {
        check(
            ee::s10::f_00317710__generated,
            md5_hash_string,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7710, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let out = DATA + 0x100;
                let s = DATA + 0x200;
                for i in 0..4 {
                    c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
                }
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                for i in 0..0x10 {
                    c.bus.write8(s + len as u32 + 1 + i, 0x80 + i as u8);
                }
                c.g[A0][0] = s as u64;
                c.g[A1][0] = out as u64;
                c.g[A2][0] = s as u64;
                c.pc = 0x0031_7710;
            },
        );
        require_paths(ee::s10::f_00317710__generated, 100, &[2004], &|c, seed| {
            let mut r = machine(c, seed, 0x0031_7710, 0);
            let len = match seed % 8 {
                0 => 0,
                1 => 1,
                2 => 55,
                3 => 56,
                4 => 57,
                5 => 63,
                6 => 64,
                _ => (r.next_u32() % 65) as usize,
            };
            let out = DATA + 0x100;
            let s = DATA + 0x200;
            for i in 0..4 {
                c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
            }
            for i in 0..len {
                c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
            }
            c.bus.write8(s + len as u32, 0);
            for i in 0..0x10 {
                c.bus.write8(s + len as u32 + 1 + i, 0x80 + i as u8);
            }
            c.g[A0][0] = s as u64;
            c.g[A1][0] = out as u64;
            c.g[A2][0] = s as u64;
            c.pc = 0x0031_7710;
        });
    }

    /// `DLHeap_InitState`: 94 nodes get threaded through the heap header,
    /// and the constants land identically on both sides.
    #[test]
    fn dlheap_init_state_matches() {
        check(
            ee::s10::f_0031e260__generated,
            dlheap_init_state,
            60,
            &|c, seed| {
                machine(c, seed, 0x0031_e260, 0);
                const HEAP: u32 = DATA + 0x100;
                for i in 0..0x100u32 {
                    c.bus.write32(HEAP + 4 * i, 0x5555_0000 + i + seed);
                }
                c.g[A0][0] = HEAP as u64;
                c.pc = 0x0031_e260;
            },
        );
        require_paths(ee::s10::f_0031e260__generated, 60, &[687], &|c, seed| {
            machine(c, seed, 0x0031_e260, 0);
            const HEAP: u32 = DATA + 0x100;
            for i in 0..0x100u32 {
                c.bus.write32(HEAP + 4 * i, 0x5555_0000 + i + seed);
            }
            c.g[A0][0] = HEAP as u64;
            c.pc = 0x0031_e260;
        });
    }

    driven!(init_gen, ee::s10::f_0031e260__generated);

    driven!(init_hand, dlheap_init_state);

    /// The init loop under delivered events. The head (0x31e270) is
    /// structured: the port must carry on, never resume there.
    #[test]
    fn dlheap_init_state_events() {
        let resumes = check_events(init_gen, init_hand, 60, &|c, seed| {
            machine(c, seed, 0x0031_e260, 0);
            const HEAP: u32 = DATA + 0x100;
            for i in 0..0x100u32 {
                c.bus.write32(HEAP + 4 * i, 0x5555_0000 + i + seed);
            }
            c.g[A0][0] = HEAP as u64;
            schedule(c, &mut Rng(seed as u64), seed, 0x0031_e260)
        });
        assert!(
            resumes.get(&0x0031_e270).is_none(),
            "unwound at structured head 0x31e270: {resumes:x?}"
        );
    }
}
