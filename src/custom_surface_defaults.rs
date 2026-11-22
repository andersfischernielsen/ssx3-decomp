//! Native initialization of the 19 surface physics/FX parameter records.
//!
//! The type names are not encoded here. Each record has a 0x90-byte scalar
//! portion and a 0x20-byte curve copied from the game's initialized globals.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002c_2f18,
    surf_prop_cache_init_defaults,
    "SurfPropCache_InitDefaults",
)];

const SURFACE_BYTES: u32 = 0xb0;
const SURFACE_COUNT: u32 = 19;
const CURVE_OFFSET: u32 = 0x90;
const CURVE_BYTES: u64 = 0x20;
const COPY_MEMORY: u32 = 0x003e_6574;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

#[inline]
fn float_constant(c: &mut Cpu, register: usize, bits: u64) {
    c.g[AT][0] = bits;
    c.f[register] = bits as u32;
}

#[inline]
fn load_pool(c: &mut Cpu, fields: &[(usize, u32)]) {
    for &(register, offset) in fields {
        c.lwc1(register, r(c, GP).wrapping_sub(offset));
    }
}

#[inline]
fn write_float_fields(c: &mut Cpu, base: u32, fields: &[(u32, usize)]) {
    for &(offset, register) in fields {
        c.swc1(register, base.wrapping_add(offset));
    }
}

#[inline]
fn write_word_fields(c: &mut Cpu, base: u32, offsets: &[u32], value: u64) {
    for &offset in offsets {
        c.sw(base.wrapping_add(offset), value);
    }
}

#[inline]
fn write_flag_fields(c: &mut Cpu, base: u32, offsets: &[u32]) {
    let value = c.g[T6][0];
    write_word_fields(c, base, offsets, value);
}

/// `void SurfPropCache_InitDefaults(SurfProp* records)` (0x2c2f18).
///
/// Initialize 19 records, apply the per-surface scalar defaults, then copy
/// each surface's four-pair depth/response curve from the native table.
/// Field meanings beyond the curve are intentionally left as offsets.
/// The store order, scratch registers and FPU pool loads match the EE body.
/// No differential test was added or run, per the user's explicit instruction.
pub fn surf_prop_cache_init_defaults(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x90);
        float_constant(c, 0, 0x3f80_0000);
        frame.save(c, S0, 0x20);
        c.g[T7][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        c.sd(r(c, SP).wrapping_add(0x10), c.g[RA][0]);
        frame.save_fpr(c, 31, 0x88);
        c.g[T8][0] = sx(r(c, S0).wrapping_add(0x6c));
        for (register, offset) in [
            (30, 0x80),
            (29, 0x78),
            (28, 0x70),
            (27, 0x68),
            (26, 0x60),
            (25, 0x58),
            (24, 0x50),
            (23, 0x48),
            (22, 0x40),
            (21, 0x38),
            (20, 0x30),
        ] {
            frame.save_fpr(c, register, offset);
        }
        float_constant(c, 2, 0x41a0_0000);
        load_pool(c, &[(1, 0x4168), (9, 0x4164)]);
        float_constant(c, 8, 0x4220_0000);
        load_pool(c, &[(7, 0x4160)]);
        float_constant(c, 6, 0x42a0_0000);
        load_pool(c, &[(5, 0x415c)]);
        float_constant(c, 4, 0x42f0_0000);
        load_pool(c, &[(3, 0x4158)]);
        c.g[V0][0] = SURFACE_BYTES as u64;
        loop {
            c.cycles += 92;
            float_constant(c, 10, 0x4475_0000);
            c.mult(0, V0, T7, V0);
            load_pool(c, &[(31, 0x4154)]);
            float_constant(c, 30, 0x41d8_0000);
            c.g[T6][0] = 1;
            float_constant(c, 29, 0x4234_0000);
            c.g[T7][0] = sx(r(c, T7).wrapping_add(1));
            c.g[T5][0] = ((c.g[T7][0] as i64) < SURFACE_COUNT as i64) as u64;
            c.g[T4][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
            let entry = r(c, T4);
            c.swc1(10, entry);
            c.g[V1][0] = c.g[T4][0];
            c.sw(entry.wrapping_add(4), 0);
            c.g[A0][0] = c.g[V1][0];
            load_pool(c, &[(10, 0x4150)]);
            c.g[A1][0] = c.g[V1][0];
            c.swc1(0, entry.wrapping_add(8));
            c.g[A2][0] = c.g[V1][0];
            c.swc1(10, entry.wrapping_add(0xc));
            c.g[A3][0] = c.g[V1][0];
            float_constant(c, 10, 0x4120_0000);
            c.g[T0][0] = c.g[V1][0];
            c.swc1(2, entry.wrapping_add(0x10));
            c.g[T1][0] = c.g[V1][0];
            c.swc1(10, entry.wrapping_add(0x14));
            c.g[T2][0] = c.g[V1][0];
            float_constant(c, 10, 0x41f0_0000);
            c.g[T3][0] = c.g[V1][0];
            c.swc1(2, entry.wrapping_add(0x18));
            c.g[V0][0] = c.g[V1][0];
            c.swc1(10, entry.wrapping_add(0x1c));
            float_constant(c, 10, 0x4252_0000);
            c.swc1(10, entry.wrapping_add(0x20));
            load_pool(c, &[(10, 0x414c)]);
            write_float_fields(
                c,
                entry,
                &[
                    (0x24, 10),
                    (0x28, 1),
                    (0x2c, 31),
                    (0x30, 0),
                    (0x34, 30),
                    (0x38, 2),
                    (0x3c, 0),
                ],
            );
            write_word_fields(c, entry, &[0x40, 0x44, 0x48], 0);
            c.swc1(0, entry.wrapping_add(0x4c));
            c.sw(entry.wrapping_add(0x50), 0);
            c.swc1(29, entry.wrapping_add(0x54));
            write_word_fields(
                c,
                entry,
                &[0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74],
                0,
            );
            c.sw(entry.wrapping_add(0x78), c.g[T6][0]);
            write_float_fields(c, entry, &[(0x7c, 1), (0x80, 1)]);
            write_word_fields(c, entry, &[0x84, 0x88], 0);
            c.sw(r(c, SP), 0);
            c.swc1(9, r(c, SP).wrapping_add(4));
            c.sw(entry.wrapping_add(0x8c), 0);
            copy_curve_pair(c, entry, 0x90);
            for (first, second, offset) in [(8, 7, 0x98), (6, 5, 0xa0), (4, 3, 0xa8)] {
                c.swc1(first, r(c, SP));
                c.swc1(second, r(c, SP).wrapping_add(4));
                copy_curve_pair(c, entry, offset);
            }
            let more = c.g[T5][0] != 0;
            c.g[V0][0] = SURFACE_BYTES as u64;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_2fa0);
            }
        }

        apply_surface_defaults(c);
        c.g[A1][0] = c.lw(r(c, GP).wrapping_add(0x7b4));
        let base = r(c, S0);
        write_word_fields(c, base, &[0xc34, 0xc38], 0);
        c.swc1(6, base.wrapping_add(0xc30));
        c.sw(base.wrapping_add(0xc3c), c.g[T6][0]);
        call_ptr(c, COPY_MEMORY, 0x002c_3e40)?;
        // GP curve slots follow authoring order: surface 18 precedes 8..17.
        for (ordinal, surface) in [
            1, 2, 3, 4, 5, 6, 7, 18, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        ]
        .into_iter()
        .enumerate()
        {
            let ordinal = ordinal as u32 + 1;
            c.g[A1][0] = c.lw(r(c, GP).wrapping_add(0x7b4 + ordinal * 8));
            c.g[A0][0] = sx(r(c, S0).wrapping_add(surface * SURFACE_BYTES + CURVE_OFFSET));
            c.g[A2][0] = CURVE_BYTES;
            call_ptr(c, COPY_MEMORY, 0x002c_3e40 + ordinal * 0x10)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        for (register, offset) in [
            (31, 0x88),
            (30, 0x80),
            (29, 0x78),
            (28, 0x70),
            (27, 0x68),
            (26, 0x60),
            (25, 0x58),
            (24, 0x50),
            (23, 0x48),
            (22, 0x40),
            (21, 0x38),
            (20, 0x30),
        ] {
            frame.restore_fpr(c, register, offset);
        }
        frame.close(c);
        Ok(())
    })
}

/// Preserve the EE's unaligned 64-bit copies, including v0's result.
#[inline]
fn copy_curve_pair(c: &mut Cpu, entry: u32, offset: u32) {
    c.ldl(V0, r(c, SP).wrapping_add(7));
    c.ldr(V0, r(c, SP));
    c.sdl(V0, entry.wrapping_add(offset + 7));
    c.sdr(V0, entry.wrapping_add(offset));
}

/// Per-surface scalar overrides. Offsets address the whole 19-record cache;
/// keeping authoring order preserves aliases between table and pool memory.
fn apply_surface_defaults(c: &mut Cpu) {
    let base = r(c, S0);
    c.cycles += 932;
    load_pool(c, &[(0, 0x4148)]);
    c.g[A0][0] = sx(base.wrapping_add(CURVE_OFFSET));
    float_constant(c, 2, 0x3f800000);
    c.g[A2][0] = 0x20;
    write_float_fields(c, base, &[(0x8, 0)]);
    float_constant(c, 0, 0x3f000000);
    load_pool(c, &[(6, 0x4144)]);
    float_constant(c, 7, 0x41a00000);
    load_pool(
        c,
        &[
            (12, 0x4140),
            (9, 0x413c),
            (25, 0x4138),
            (26, 0x4134),
            (27, 0x4130),
            (28, 0x412c),
            (1, 0x4128),
            (3, 0x4124),
            (4, 0x4120),
            (5, 0x411c),
        ],
    );
    write_float_fields(
        c,
        base,
        &[
            (0x14, 0),
            (0xc4, 0),
            (0x28, 6),
            (0x30, 2),
            (0x3c, 2),
            (0xd0, 12),
            (0xd8, 6),
            (0xe0, 2),
            (0xc, 1),
            (0x20, 3),
            (0xb8, 4),
            (0xbc, 5),
            (0, 9),
            (0x10, 25),
            (0x18, 26),
            (0x1c, 27),
            (0x24, 28),
            (0x2c, 31),
            (0x34, 30),
            (0x38, 7),
            (0xb0, 9),
            (0xc0, 25),
            (0xc8, 26),
            (0xcc, 27),
            (0xd4, 28),
            (0xdc, 31),
            (0xe4, 30),
            (0xe8, 7),
        ],
    );
    write_word_fields(c, base, &[0x4, 0x40, 0x44, 0xb4], 0);
    load_pool(c, &[(0, 0x4118)]);
    write_float_fields(c, base, &[(0xec, 2), (0x160, 0)]);
    float_constant(c, 0, 0xffffffffbf800000);
    load_pool(
        c,
        &[
            (1, 0x4114),
            (3, 0x4110),
            (4, 0x410c),
            (5, 0x4108),
            (8, 0x4104),
            (10, 0x4100),
            (11, 0x40fc),
            (13, 0x40f8),
            (14, 0x40f4),
            (15, 0x40f0),
            (16, 0x40ec),
            (17, 0x40e8),
            (18, 0x40e4),
            (19, 0x40e0),
            (20, 0x40dc),
            (21, 0x40d8),
            (22, 0x40d4),
            (23, 0x40d0),
            (24, 0x40cc),
        ],
    );
    write_float_fields(
        c,
        base,
        &[
            (0x164, 0),
            (0x214, 0),
            (0x188, 6),
            (0x190, 2),
            (0x238, 6),
            (0x240, 2),
            (0x168, 1),
            (0x16c, 3),
            (0x170, 4),
            (0x174, 5),
            (0x178, 8),
            (0x17c, 10),
            (0x180, 11),
            (0x184, 13),
            (0x19c, 14),
            (0x1a0, 15),
            (0x210, 16),
            (0x218, 17),
            (0x21c, 18),
            (0x220, 19),
            (0x224, 20),
            (0x228, 21),
            (0x22c, 22),
            (0x230, 23),
            (0x234, 24),
            (0x18c, 31),
            (0x194, 30),
            (0x198, 7),
            (0x23c, 31),
        ],
    );
    write_word_fields(c, base, &[0xf0, 0xf4, 0x1a4], 0);
    load_pool(c, &[(0, 0x40c8)]);
    write_float_fields(c, base, &[(0x244, 30), (0x24c, 0)]);
    float_constant(c, 0, 0x3f000000);
    load_pool(
        c,
        &[
            (13, 0x40c4),
            (14, 0x40c0),
            (15, 0x40bc),
            (16, 0x40b8),
            (17, 0x40b4),
            (18, 0x40b0),
            (19, 0x40ac),
            (10, 0x40a8),
            (20, 0x40a4),
            (21, 0x40a0),
            (22, 0x409c),
            (1, 0x4098),
        ],
    );
    write_float_fields(
        c,
        base,
        &[
            (0x2d4, 0),
            (0x384, 0),
            (0x2c0, 14),
            (0x2c4, 2),
            (0x2c8, 15),
            (0x2cc, 16),
            (0x2d0, 17),
            (0x2d8, 18),
            (0x2dc, 19),
            (0x2e8, 6),
            (0x2f0, 2),
            (0x2fc, 2),
            (0x37c, 13),
            (0x390, 12),
            (0x398, 6),
            (0x250, 1),
            (0x248, 7),
            (0x2e0, 20),
            (0x2e4, 21),
            (0x2ec, 31),
            (0x2f4, 22),
            (0x2f8, 7),
            (0x370, 9),
            (0x378, 10),
            (0x380, 25),
            (0x388, 26),
            (0x38c, 27),
            (0x394, 28),
        ],
    );
    write_word_fields(c, base, &[0x254, 0x300, 0x304, 0x374], 0);
    load_pool(c, &[(0, 0x4094)]);
    write_float_fields(c, base, &[(0x39c, 31), (0x4d0, 0)]);
    float_constant(c, 0, 0x3f000000);
    load_pool(c, &[(1, 0x4090)]);
    write_float_fields(c, base, &[(0x434, 0), (0x4e4, 0)]);
    float_constant(c, 0, 0x41f00000);
    load_pool(c, &[(3, 0x408c), (4, 0x4088), (5, 0x4084), (8, 0x4080)]);
    write_float_fields(c, base, &[(0x4ec, 0)]);
    load_pool(c, &[(11, 0x407c)]);
    write_float_fields(
        c,
        base,
        &[
            (0x3a0, 2),
            (0x3ac, 2),
            (0x42c, 13),
            (0x440, 12),
            (0x448, 6),
            (0x450, 2),
            (0x45c, 2),
            (0x4d8, 1),
            (0x4dc, 3),
            (0x4e0, 4),
            (0x4e8, 5),
            (0x4f0, 8),
            (0x3a4, 30),
            (0x3a8, 7),
            (0x420, 9),
            (0x428, 10),
            (0x430, 25),
            (0x438, 26),
            (0x43c, 27),
            (0x444, 28),
            (0x44c, 31),
            (0x454, 30),
            (0x458, 7),
        ],
    );
    write_word_fields(c, base, &[0x3b0, 0x3b4, 0x424, 0x460, 0x464, 0x4d4], 0);
    float_constant(c, 0, 0x44750000);
    write_float_fields(c, base, &[(0x4f4, 11), (0xc60, 0)]);
    load_pool(c, &[(0, 0x4078)]);
    write_float_fields(c, base, &[(0x4f8, 6), (0xc6c, 0)]);
    float_constant(c, 0, 0x41200000);
    write_float_fields(c, base, &[(0x500, 2), (0xc74, 0)]);
    float_constant(c, 0, 0x41f00000);
    write_float_fields(c, base, &[(0x50c, 2), (0xc7c, 0)]);
    float_constant(c, 0, 0x42520000);
    write_float_fields(c, base, &[(0xc68, 2), (0xc80, 0)]);
    load_pool(c, &[(0, 0x4074)]);
    write_float_fields(c, base, &[(0xc88, 6), (0xc84, 0)]);
    float_constant(c, 0, 0x3f000000);
    write_float_fields(
        c,
        base,
        &[
            (0xc90, 2),
            (0xc9c, 2),
            (0x58c, 13),
            (0x4fc, 31),
            (0x504, 30),
            (0x508, 7),
            (0xc70, 7),
            (0xc78, 7),
            (0xc8c, 31),
            (0xc94, 30),
            (0xc98, 7),
            (0x580, 9),
            (0x588, 10),
            (0x590, 25),
            (0x594, 0),
            (0x598, 26),
        ],
    );
    write_word_fields(c, base, &[0x510, 0x514, 0xc64, 0xca0, 0xca4, 0x584], 0);
    write_float_fields(
        c,
        base,
        &[
            (0x59c, 27),
            (0x5a0, 12),
            (0x5a8, 6),
            (0x5b0, 2),
            (0x5bc, 2),
            (0x63c, 13),
            (0x650, 12),
            (0x658, 6),
            (0x660, 2),
            (0x66c, 2),
            (0x6ec, 13),
            (0x5a4, 28),
            (0x5ac, 31),
            (0x5b4, 30),
            (0x5b8, 7),
            (0x630, 9),
            (0x638, 10),
            (0x640, 25),
            (0x644, 0),
            (0x648, 26),
            (0x64c, 27),
            (0x654, 28),
            (0x65c, 31),
            (0x664, 30),
            (0x668, 7),
            (0x6e0, 9),
            (0x6e8, 10),
            (0x6f0, 25),
        ],
    );
    write_word_fields(c, base, &[0x5c0, 0x5c4, 0x634, 0x670, 0x674, 0x6e4], 0);
    write_float_fields(
        c,
        base,
        &[
            (0x6f4, 0),
            (0x7a4, 0),
            (0x790, 14),
            (0x798, 15),
            (0x79c, 16),
            (0x7a0, 17),
            (0x7a8, 18),
            (0x7ac, 19),
            (0x700, 12),
            (0x708, 6),
            (0x710, 2),
            (0x71c, 2),
            (0x794, 2),
            (0x7b8, 6),
            (0x7c0, 2),
            (0x7cc, 2),
            (0x7b0, 20),
            (0x7b4, 21),
            (0x7c4, 22),
            (0x840, 9),
            (0x848, 10),
            (0x6f8, 26),
            (0x6fc, 27),
            (0x704, 28),
            (0x70c, 31),
            (0x714, 30),
            (0x718, 7),
            (0x7bc, 31),
            (0x7c8, 7),
        ],
    );
    write_word_fields(c, base, &[0x720, 0x724, 0x7d0, 0x7d4, 0x844], 0);
    load_pool(c, &[(0, 0x4070)]);
    write_float_fields(c, base, &[(0x84c, 13), (0x8f0, 0)]);
    load_pool(c, &[(10, 0x406c)]);
    float_constant(c, 0, 0xffffffffbf800000);
    write_float_fields(c, base, &[(0x910, 10), (0x8f4, 0)]);
    float_constant(c, 10, 0x3f000000);
    load_pool(
        c,
        &[
            (9, 0x4068),
            (11, 0x4064),
            (1, 0x4060),
            (3, 0x405c),
            (4, 0x4058),
            (5, 0x4054),
            (8, 0x4050),
        ],
    );
    float_constant(c, 0, 0x44750000);
    write_float_fields(
        c,
        base,
        &[
            (0x854, 10),
            (0x860, 12),
            (0x90c, 9),
            (0x914, 11),
            (0x868, 6),
            (0x870, 2),
            (0x87c, 2),
            (0x918, 6),
            (0x920, 2),
            (0x92c, 2),
            (0x850, 25),
            (0x858, 26),
            (0x85c, 27),
            (0x864, 28),
            (0x8f8, 1),
            (0x8fc, 3),
            (0x900, 4),
            (0x904, 5),
            (0x908, 8),
            (0x86c, 31),
            (0x874, 30),
            (0x878, 7),
            (0x91c, 31),
            (0x924, 30),
            (0x928, 7),
            (0x9a0, 0),
        ],
    );
    write_word_fields(c, base, &[0x880, 0x884, 0x930, 0x934], 0);
    load_pool(c, &[(10, 0x404c)]);
    write_word_fields(c, base, &[0x9a4], 0);
    write_float_fields(c, base, &[(0x9ac, 10)]);
    float_constant(c, 0, 0x41200000);
    float_constant(c, 10, 0x41f00000);
    write_float_fields(c, base, &[(0x9b4, 0), (0x9bc, 10)]);
    float_constant(c, 0, 0x42520000);
    load_pool(c, &[(10, 0x4048)]);
    write_float_fields(c, base, &[(0x9c0, 0), (0x9c4, 10)]);
    float_constant(c, 0, 0x44750000);
    load_pool(c, &[(10, 0x4044)]);
    write_float_fields(c, base, &[(0xa50, 0), (0xa5c, 10)]);
    float_constant(c, 0, 0x41200000);
    float_constant(c, 10, 0x41f00000);
    write_float_fields(c, base, &[(0xa64, 0), (0xa6c, 10)]);
    float_constant(c, 0, 0x42520000);
    load_pool(c, &[(10, 0x4040)]);
    write_float_fields(
        c,
        base,
        &[
            (0xa70, 0),
            (0x9a8, 2),
            (0x9c8, 6),
            (0x9d0, 2),
            (0x9dc, 2),
            (0xa58, 2),
            (0xa78, 6),
            (0xa80, 2),
            (0xa8c, 2),
            (0x9b0, 7),
            (0x9b8, 7),
            (0x9cc, 31),
            (0x9d4, 30),
            (0x9d8, 7),
            (0xa60, 7),
            (0xa68, 7),
            (0xa74, 10),
            (0xa7c, 31),
            (0xa84, 30),
            (0xa88, 7),
        ],
    );
    write_word_fields(c, base, &[0x9e0, 0x9e4, 0xa54, 0xa90], 0);
    float_constant(c, 0, 0x44750000);
    write_word_fields(c, base, &[0xa94], 0);
    load_pool(c, &[(10, 0x403c)]);
    write_float_fields(c, base, &[(0xbb0, 0), (0xbbc, 10)]);
    float_constant(c, 0, 0x41200000);
    float_constant(c, 10, 0x41f00000);
    write_float_fields(c, base, &[(0xbc4, 0), (0xbcc, 10)]);
    float_constant(c, 0, 0x42520000);
    load_pool(c, &[(10, 0x4038)]);
    write_float_fields(c, base, &[(0xbd0, 0), (0xbd4, 10)]);
    float_constant(c, 0, 0x44750000);
    load_pool(c, &[(10, 0x4034)]);
    write_float_fields(c, base, &[(0xb00, 0), (0xb0c, 10)]);
    float_constant(c, 0, 0x41200000);
    float_constant(c, 10, 0x41f00000);
    write_float_fields(c, base, &[(0xb14, 0), (0xb1c, 10)]);
    float_constant(c, 0, 0x42520000);
    load_pool(c, &[(10, 0x4030)]);
    write_float_fields(
        c,
        base,
        &[
            (0xb20, 0),
            (0xb08, 2),
            (0xb28, 6),
            (0xb30, 2),
            (0xb3c, 2),
            (0xbb8, 2),
            (0xbd8, 6),
            (0xbe0, 2),
            (0xbe4, 30),
            (0xbe8, 7),
            (0xb10, 7),
            (0xb18, 7),
            (0xb24, 10),
            (0xb2c, 31),
            (0xb34, 30),
            (0xb38, 7),
            (0xbc0, 7),
            (0xbc8, 7),
            (0xbdc, 31),
        ],
    );
    write_word_fields(c, base, &[0xb04, 0xb40, 0xb44, 0xbb4], 0);
    float_constant(c, 0, 0x40000000);
    write_float_fields(c, base, &[(0xbec, 2), (0x5c, 0)]);
    load_pool(c, &[(1, 0x402c)]);
    float_constant(c, 0, 0x3f000000);
    write_float_fields(c, base, &[(0x68, 1), (0x4c, 0), (0x48, 2)]);
    write_flag_fields(c, base, &[0x58, 0x60]);
    write_float_fields(c, base, &[(0x54, 29)]);
    write_word_fields(c, base, &[0xbf0, 0xbf4, 0x50, 0x64], 0);
    load_pool(c, &[(0, 0x4028), (1, 0x4024), (3, 0x4020)]);
    float_constant(c, 4, 0x40400000);
    load_pool(c, &[(5, 0x401c)]);
    c.sw((c.g[T8][0] as u32), 0u64);
    load_pool(c, &[(7, 0x4018), (8, 0x4014)]);
    write_float_fields(
        c,
        base,
        &[(0x7c, 0), (0x80, 1), (0xfc, 3), (0x10c, 4), (0x124, 5)],
    );
    load_pool(c, &[(3, 0x4010)]);
    write_float_fields(c, base, &[(0x70, 2)]);
    write_flag_fields(c, base, &[0x78, 0x88]);
    write_float_fields(c, base, &[(0xf8, 2)]);
    write_flag_fields(c, base, &[0x100, 0x108, 0x110]);
    write_float_fields(c, base, &[(0x118, 7), (0x120, 6)]);
    write_flag_fields(c, base, &[0x128]);
    load_pool(c, &[(4, 0x400c)]);
    float_constant(c, 5, 0x40a00000);
    float_constant(c, 1, 0x40800000);
    write_float_fields(c, base, &[(0x74, 31), (0x104, 29)]);
    write_word_fields(c, base, &[0x84, 0x8c, 0x114, 0x11c], 0);
    load_pool(c, &[(0, 0x4008)]);
    write_float_fields(c, base, &[(0x12c, 8)]);
    float_constant(c, 10, 0x3f000000);
    write_float_fields(c, base, &[(0x1bc, 1)]);
    write_flag_fields(c, base, &[0x138]);
    write_float_fields(c, base, &[(0x1a8, 2), (0x1ac, 2)]);
    write_flag_fields(c, base, &[0x1b0, 0x1b8, 0x1c0, 0x1c4]);
    write_float_fields(c, base, &[(0x1c8, 7), (0x1d0, 2), (0x1d4, 3)]);
    write_flag_fields(c, base, &[0x1d8]);
    write_float_fields(c, base, &[(0x1dc, 6), (0x1e0, 6)]);
    write_flag_fields(c, base, &[0x1e4, 0x1e8]);
    write_float_fields(c, base, &[(0x258, 2), (0x25c, 4)]);
    write_flag_fields(c, base, &[0x260, 0x268]);
    write_float_fields(c, base, &[(0x26c, 5)]);
    write_flag_fields(c, base, &[0x270, 0x274]);
    write_float_fields(
        c,
        base,
        &[(0x278, 7), (0x280, 2), (0x130, 0), (0x1b4, 29), (0x264, 29)],
    );
    write_word_fields(c, base, &[0x134, 0x13c, 0x1cc, 0x1ec, 0x27c], 0);
    write_float_fields(c, base, &[(0x284, 3)]);
    write_flag_fields(c, base, &[0x288]);
    write_float_fields(c, base, &[(0x28c, 6), (0x290, 6)]);
    write_flag_fields(c, base, &[0x294, 0x298]);
    write_float_fields(c, base, &[(0x330, 10), (0x33c, 6), (0x340, 6)]);
    write_flag_fields(c, base, &[0x348]);
    write_float_fields(c, base, &[(0x3bc, 10), (0x314, 29), (0x3c4, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0x29c, 0x308, 0x30c, 0x310, 0x318, 0x31c, 0x320, 0x324, 0x328, 0x32c, 0x334, 0x338,
            0x344, 0x34c, 0x3b8, 0x3c0, 0x3c8, 0x3cc, 0x3d0, 0x3d4, 0x3d8,
        ],
        0,
    );
    float_constant(c, 0, 0x3f400000);
    write_word_fields(c, base, &[0x3dc], 0);
    write_float_fields(c, base, &[(0x3e0, 2), (0x3e4, 3)]);
    write_flag_fields(c, base, &[0x3e8]);
    write_float_fields(
        c,
        base,
        &[(0x3ec, 6), (0x3f0, 6), (0x46c, 3), (0x490, 2), (0x494, 3)],
    );
    write_flag_fields(c, base, &[0x498]);
    write_float_fields(
        c,
        base,
        &[
            (0x49c, 6),
            (0x4a0, 6),
            (0x51c, 2),
            (0x468, 0),
            (0x474, 29),
            (0x524, 29),
        ],
    );
    write_word_fields(
        c,
        base,
        &[
            0x3f4, 0x3f8, 0x3fc, 0x470, 0x478, 0x47c, 0x480, 0x484, 0x488, 0x48c, 0x4a4, 0x4a8,
            0x4ac, 0x518, 0x520, 0x528, 0x52c, 0x530, 0x534,
        ],
        0,
    );
    write_float_fields(c, base, &[(0x53c, 7), (0x540, 2), (0x544, 2)]);
    write_flag_fields(c, base, &[0x548]);
    write_float_fields(c, base, &[(0x54c, 6), (0x550, 6)]);
    write_flag_fields(c, base, &[0x55c]);
    write_float_fields(c, base, &[(0xcac, 2), (0xccc, 7), (0xcd0, 2), (0xcd4, 2)]);
    write_flag_fields(c, base, &[0xcd8]);
    write_float_fields(c, base, &[(0xcdc, 6), (0xce0, 6)]);
    write_flag_fields(c, base, &[0xcec]);
    load_pool(c, &[(0, 0x4004)]);
    write_float_fields(c, base, &[(0xcb4, 29), (0x5d4, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0x538, 0x554, 0x558, 0xca8, 0xcb0, 0xcb8, 0xcbc, 0xcc0, 0xcc4, 0xcc8, 0xce4, 0xce8,
            0x5c8, 0x5cc, 0x5d0, 0x5d8, 0x5dc,
        ],
        0,
    );
    write_float_fields(c, base, &[(0x5ec, 0), (0x5f0, 2), (0x5f4, 3)]);
    write_flag_fields(c, base, &[0x5f8]);
    write_float_fields(c, base, &[(0x5fc, 6), (0x600, 6)]);
    write_flag_fields(c, base, &[0x60c]);
    write_float_fields(c, base, &[(0x6a0, 2), (0x6a4, 3)]);
    write_flag_fields(c, base, &[0x6a8]);
    write_float_fields(c, base, &[(0x6ac, 6), (0x6b0, 6)]);
    write_flag_fields(c, base, &[0x6b8]);
    write_float_fields(c, base, &[(0x684, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0x5e0, 0x5e4, 0x5e8, 0x604, 0x608, 0x678, 0x67c, 0x680, 0x688, 0x68c, 0x690, 0x694,
            0x698, 0x69c, 0x6b4, 0x6bc, 0x728, 0x72c, 0x730,
        ],
        0,
    );
    write_float_fields(c, base, &[(0x734, 29), (0x800, 10), (0x750, 2), (0x754, 3)]);
    write_flag_fields(c, base, &[0x758]);
    write_float_fields(c, base, &[(0x75c, 6), (0x760, 6)]);
    write_flag_fields(c, base, &[0x76c]);
    write_float_fields(c, base, &[(0x80c, 6), (0x810, 6)]);
    write_flag_fields(c, base, &[0x818]);
    write_float_fields(c, base, &[(0x7e4, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0x738, 0x73c, 0x740, 0x744, 0x748, 0x74c, 0x764, 0x768, 0x7d8, 0x7dc, 0x7e0, 0x7e8,
            0x7ec, 0x7f0, 0x7f4, 0x7f8, 0x7fc, 0x804, 0x808, 0x814, 0x81c, 0x888, 0x88c,
        ],
        0,
    );
    write_float_fields(c, base, &[(0x93c, 4), (0x94c, 5), (0x8b0, 2), (0x8b4, 3)]);
    write_flag_fields(c, base, &[0x8b8]);
    write_float_fields(c, base, &[(0x8bc, 6), (0x8c0, 6), (0x938, 2)]);
    write_flag_fields(c, base, &[0x940, 0x948, 0x950, 0x954]);
    write_float_fields(c, base, &[(0x958, 7), (0x960, 2), (0x964, 3)]);
    write_flag_fields(c, base, &[0x968]);
    write_float_fields(c, base, &[(0x96c, 6), (0x970, 6)]);
    write_flag_fields(c, base, &[0x974, 0x978]);
    write_float_fields(c, base, &[(0x894, 29), (0x944, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0x890, 0x898, 0x89c, 0x8a0, 0x8a4, 0x8a8, 0x8ac, 0x8c4, 0x8c8, 0x8cc, 0x95c, 0x97c,
        ],
        0,
    );
    write_float_fields(
        c,
        base,
        &[(0xa10, 3), (0x9ec, 2), (0xa14, 2), (0xa1c, 6), (0xa20, 6)],
    );
    write_flag_fields(c, base, &[0xa28]);
    write_float_fields(c, base, &[(0xa9c, 2), (0xac0, 2), (0xac4, 2)]);
    write_flag_fields(c, base, &[0xac8]);
    write_float_fields(c, base, &[(0xacc, 6), (0xad0, 6), (0x9f4, 29), (0xaa4, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0x9e8, 0x9f0, 0x9f8, 0x9fc, 0xa00, 0xa04, 0xa08, 0xa0c, 0xa18, 0xa24, 0xa2c, 0xa98,
            0xaa0, 0xaa8, 0xaac, 0xab0, 0xab4, 0xab8, 0xabc, 0xad4,
        ],
        0,
    );
    write_float_fields(
        c,
        base,
        &[(0xb6c, 7), (0xc24, 2), (0xb4c, 2), (0xb70, 2), (0xb74, 2)],
    );
    write_flag_fields(c, base, &[0xb78]);
    write_float_fields(c, base, &[(0xb7c, 6), (0xb80, 6)]);
    write_flag_fields(c, base, &[0xb8c]);
    write_float_fields(c, base, &[(0xbfc, 2), (0xc20, 2)]);
    write_flag_fields(c, base, &[0xc28]);
    write_float_fields(c, base, &[(0xc04, 29), (0xb54, 29)]);
    write_word_fields(
        c,
        base,
        &[
            0xad8, 0xadc, 0xb48, 0xb50, 0xb58, 0xb5c, 0xb60, 0xb64, 0xb68, 0xb84, 0xb88, 0xbf8,
            0xc00, 0xc08, 0xc0c, 0xc10, 0xc14, 0xc18, 0xc1c,
        ],
        0,
    );
    write_float_fields(c, base, &[(0xc2c, 6)]);
}
