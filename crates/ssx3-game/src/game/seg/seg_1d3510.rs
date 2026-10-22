//! `src/seg/seg_1d3510.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_21d0, serialize_write_u32, "Serialize_WriteU32"),
    (0x002d_2210, serialize_write_u32_2, "Serialize_WriteU32_2"),
    (0x002d_23c8, serialize_write_u8, "Serialize_WriteU8"),
    (0x002d_2510, serialize_read_u32, "Serialize_ReadU32"),
    (0x002d_2558, serialize_read_u32_2, "Serialize_ReadU32_2"),
    (0x002d_2790, serialize_read_u8, "Serialize_ReadU8"),
];


// ---------------------------------------------------------------------------
// Serialization streams.

/// `void Serialize_WriteU32(void** stream, ...)` (0x2d21d0): write the u32 in
/// `f12` big-endian at `*stream` and advance the stream word by 4.
pub fn serialize_write_u32(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = abi::mfc1(FpuF32(c.f[12]));
    let stream = Addr(abi::arg(c, 0));
    c.g[A1][0] = stream.lw(c);
    c.g[A2][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 24);
    c.g[V1][0] = sx((c.g[A2][0] as u32) >> 8);
    c.g[A3][0] = sx((c.g[A2][0] as u32) >> 16);
    let buf = c.g[A1][0] as u32;
    Addr(buf).off(0x3).sb(c, c.g[V0][0]);
    Addr(buf).off(0x1).sb(c, c.g[V1][0]);
    c.g[V0][0] = 0x4;
    Addr(buf).off(0x2).sb(c, c.g[A3][0]);
    Addr(buf).sb(c, c.g[A2][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void Serialize_WriteU32_2(void** stream, uint value)` (0x2d2210): write
/// `value` big-endian and advance the stream word by 4.
pub fn serialize_write_u32_2(c: &mut Cpu) {
    c.cycles += 13;
    let stream = Addr(abi::arg(c, 0));
    c.g[A2][0] = stream.lw(c);
    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 24);
    c.g[V1][0] = sx((c.g[A1][0] as u32) >> 8);
    c.g[A3][0] = sx((c.g[A1][0] as u32) >> 16);
    let buf = c.g[A2][0] as u32;
    Addr(buf).off(0x3).sb(c, c.g[V0][0]);
    Addr(buf).off(0x1).sb(c, c.g[V1][0]);
    c.g[V0][0] = 0x4;
    Addr(buf).off(0x2).sb(c, c.g[A3][0]);
    Addr(buf).sb(c, c.g[A1][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void Serialize_WriteU8(void** stream, uchar value)` (0x2d23c8): write the
/// byte and advance the stream word by 1.
pub fn serialize_write_u8(c: &mut Cpu) {
    c.cycles += 7;
    let stream = Addr(abi::arg(c, 0));
    c.g[A2][0] = stream.lw(c);
    c.g[V0][0] = 0x1;
    Addr(c.g[A2][0] as u32).sb(c, c.g[A1][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x1));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void Serialize_ReadU32(void** stream, uint* out)` (0x2d2510): read a
/// big-endian u32 into `*out` and advance the stream word by 4.
pub fn serialize_read_u32(c: &mut Cpu) {
    c.cycles += 17;
    let stream = Addr(abi::arg(c, 0));
    c.g[A2][0] = stream.lw(c);
    c.g[V0][0] = 0x4;
    let buf = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(buf).off(0x3).lbu(c);
    c.g[A3][0] = Addr(buf).off(0x2).lbu(c);
    c.g[T0][0] = Addr(buf).off(0x1).lbu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
    c.g[T1][0] = Addr(buf).lbu(c);
    c.g[V1][0] |= c.g[A3][0];
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 8);
    c.g[V1][0] |= c.g[T0][0];
    c.g[A3][0] = c.g[V1][0] | c.g[T1][0];
    Addr(abi::arg(c, 1)).sw(c, c.g[A3][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void Serialize_ReadU32_2(void** stream, uint* out)` (0x2d2558): as
/// `Serialize_ReadU32` with a different scratch-register allocation.
pub fn serialize_read_u32_2(c: &mut Cpu) {
    c.cycles += 17;
    let stream = Addr(abi::arg(c, 0));
    c.g[V1][0] = stream.lw(c);
    c.g[V0][0] = 0x4;
    let buf = c.g[V1][0] as u32;
    c.g[A2][0] = Addr(buf).off(0x3).lbu(c);
    c.g[A3][0] = Addr(buf).off(0x2).lbu(c);
    c.g[T0][0] = Addr(buf).off(0x1).lbu(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
    c.g[T1][0] = Addr(buf).lbu(c);
    c.g[A2][0] |= c.g[A3][0];
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 8);
    c.g[A2][0] |= c.g[T0][0];
    c.g[A2][0] |= c.g[T1][0];
    Addr(abi::arg(c, 1)).sw(c, c.g[A2][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void Serialize_ReadU8(void** stream, uchar* out)` (0x2d2790): read a byte
/// into `*out` and advance the stream word by 1.
pub fn serialize_read_u8(c: &mut Cpu) {
    c.cycles += 8;
    let stream = Addr(abi::arg(c, 0));
    c.g[V1][0] = stream.lw(c);
    c.g[V0][0] = 0x1;
    c.g[A2][0] = Addr(c.g[V1][0] as u32).lbu(c);
    Addr(abi::arg(c, 1)).sb(c, c.g[A2][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x1));
    stream.sw(c, c.g[V1][0]);
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
    fn serialize_write_u32_matches() {
        check(ssx3_ee::s09::f_002d21d0, serialize_write_u32, 200, &|c, seed| {
            let mut r = Rng(0x21d0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.f[12] = r.next_u32();
        });
    }


    #[test]
    fn serialize_write_u32_2_matches() {
        check(ssx3_ee::s09::f_002d2210, serialize_write_u32_2, 200, &|c, seed| {
            let mut r = Rng(0x2210 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA, DATA + 0x100);
        });
    }


    #[test]
    fn serialize_write_u8_matches() {
        check(ssx3_ee::s09::f_002d23c8, serialize_write_u8, 200, &|c, seed| {
            let mut r = Rng(0x23c8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA, DATA + 0x100);
        });
    }


    #[test]
    fn serialize_read_u32_matches() {
        check(ssx3_ee::s09::f_002d2510, serialize_read_u32, 200, &|c, seed| {
            let mut r = Rng(0x2510 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA, DATA + 0x100);
            for k in 0..4u32 {
                c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
            }
        });
    }


    #[test]
    fn serialize_read_u32_2_matches() {
        check(ssx3_ee::s09::f_002d2558, serialize_read_u32_2, 200, &|c, seed| {
            let mut r = Rng(0x2558 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA, DATA + 0x100);
            for k in 0..4u32 {
                c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
            }
        });
    }


    #[test]
    fn serialize_read_u8_matches() {
        check(ssx3_ee::s09::f_002d2790, serialize_read_u8, 200, &|c, seed| {
            let mut r = Rng(0x2790 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write8(DATA + 0x100, r.next_u32() as u8);
        });
    }
}
