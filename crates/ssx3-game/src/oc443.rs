//! Staging module oc443 (wave wave223): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc443`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002c_2f18,
    surf_prop_cache_init_defaults,
    "SurfPropCache_InitDefaults",
)];

/// Where a stored word comes from: an immediate the code builds in a
/// register (`lui`/`mtc1`) or a literal-pool float loaded from `gp + offset`.
#[derive(Clone, Copy)]
pub(crate) enum Src {
    Lit(u32),
    Pool(i16),
}

/// Size of one surface-property row, and the offset of its curve.
pub(crate) const SPIC_ROW_SIZE: u32 = 0xb0;
pub(crate) const SPIC_CURVE_OFF: u32 = 0x90;
pub(crate) const SPIC_ROWS: u32 = 19;
/// `memcpy`-like helper copying each row's tuned curve (0x20 bytes).
const SPIC_COPY: u32 = 0x003e_6574;
/// Rows copy their curve in this order (row 18 comes after row 7).
const SPIC_COPY_ORDER: [u32; 19] = [
    0, 1, 2, 3, 4, 5, 6, 7, 18, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
];

/// The value of `s` (the pool is read from memory at `gp`).
fn spic_val(c: &mut Cpu, gp: u32, s: Src) -> u32 {
    match s {
        Src::Lit(v) => v,
        Src::Pool(o) => c.bus.read32(gp.wrapping_add(o as i32 as u32)),
    }
}

/// `sw`/`swc1` of `s` to `at`.
fn spic_put(c: &mut Cpu, gp: u32, at: u32, s: Src) {
    let v = spic_val(c, gp, s);
    c.sw(at, v as u64);
}

/// `void SurfPropCache_InitDefaults(SurfPropCache* self)` (0x2c2f18): fills
/// the 19 surface-property rows (0xb0 bytes: physics tuning floats at
/// 0x00..0x8c and a four-point curve at 0x90) with the defaults, writes the
/// tuned fields of rows 0-17, and copies each row's tuned curve in from the
/// static tables at `gp + 0x7b4 ...` (`EA_MemCopy`, 0x20 bytes).
///
/// A flat table port of the straight-line stores: the registers the compiled
/// code leaves (GPR temporaries, FPRs, LO/HI) are set from a simulation of it,
/// per loop iteration (an event can arrive at each back edge) and at the
/// first call.
pub fn surf_prop_cache_init_defaults(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002c_2f18);
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x90);
        let gp = c.g[GP][0] as u32;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        for k in 0..12 {
            frame.save_fpr(c, 31 - k, 0x88 - 8 * k as u32);
        }
        let mut i = 0u32;
        loop {
            c.cycles += 92;
            let s0 = c.g[S0][0] as u32;
            let row = s0.wrapping_add(i * SPIC_ROW_SIZE);
            c.g[V0][0] = SPIC_ROW_SIZE as u64;
            c.g[T7][0] = i as u64;
            c.mult(0, V0, T7, V0);
            for &(off, s) in SPIC_ROW_DEFAULTS {
                spic_put(c, gp, row.wrapping_add(off as u32), s);
            }
            // Each curve point goes through the frame's scratch doubleword
            // (`sp + 0` x, `sp + 4` y) and is stored as one unaligned 64-bit.
            let scratch = sp(c);
            for (k, &(x, y)) in SPIC_CURVE.iter().enumerate() {
                spic_put(c, gp, scratch, x);
                spic_put(c, gp, scratch.wrapping_add(4), y);
                let w = c.ld(scratch);
                Addr(row.wrapping_add(SPIC_CURVE_OFF + 8 * k as u32)).write_u64_unaligned(c, w);
            }
            spic_loop_regs(c, gp, s0, row, i + 1);
            if i + 1 < SPIC_ROWS {
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2fa0);
                }
                i += 1;
                continue;
            }
            break;
        }
        c.cycles += 932;
        let s0 = c.g[S0][0] as u32;
        for &(off, s) in SPIC_TAIL_FIELDS {
            spic_put(c, gp, s0.wrapping_add(off as u32), s);
        }
        // Registers at the first copy call.
        let last = s0.wrapping_add(18 * SPIC_ROW_SIZE);
        c.g[AT][0] = 0x3f40_0000;
        c.g[T7][0] = 19;
        c.g[T5][0] = 0;
        c.g[T6][0] = 1;
        c.g[T8][0] = sx(s0.wrapping_add(0x6c));
        c.g[V0][0] = SPIC_ROW_SIZE as u64;
        for r in [V1, A3, T0, T1, T2, T3, T4] {
            c.g[r][0] = sx(last);
        }
        c.lwc1(0, gp.wrapping_sub(0x4004));
        c.f[1] = 0x40800000;
        c.f[2] = 0x3f800000;
        c.lwc1(3, gp.wrapping_sub(0x4010));
        c.lwc1(4, gp.wrapping_sub(0x400c));
        c.f[5] = 0x40a00000;
        c.lwc1(6, gp.wrapping_sub(0x4144));
        c.lwc1(7, gp.wrapping_sub(0x4018));
        c.lwc1(8, gp.wrapping_sub(0x4014));
        c.lwc1(9, gp.wrapping_sub(0x4068));
        c.f[10] = 0x3f000000;
        c.lwc1(11, gp.wrapping_sub(0x4064));
        c.lwc1(12, gp.wrapping_sub(0x4140));
        c.lwc1(13, gp.wrapping_sub(0x40c4));
        c.lwc1(14, gp.wrapping_sub(0x40c0));
        c.lwc1(15, gp.wrapping_sub(0x40bc));
        c.lwc1(16, gp.wrapping_sub(0x40b8));
        c.lwc1(17, gp.wrapping_sub(0x40b4));
        c.lwc1(18, gp.wrapping_sub(0x40b0));
        c.lwc1(19, gp.wrapping_sub(0x40ac));
        c.lwc1(20, gp.wrapping_sub(0x40a4));
        c.lwc1(21, gp.wrapping_sub(0x40a0));
        c.lwc1(22, gp.wrapping_sub(0x409c));
        c.lwc1(23, gp.wrapping_sub(0x40d0));
        c.lwc1(24, gp.wrapping_sub(0x40cc));
        c.lwc1(25, gp.wrapping_sub(0x4138));
        c.lwc1(26, gp.wrapping_sub(0x4134));
        c.lwc1(27, gp.wrapping_sub(0x4130));
        c.lwc1(28, gp.wrapping_sub(0x412c));
        c.f[29] = 0x42340000;
        c.f[30] = 0x41d80000;
        c.lwc1(31, gp.wrapping_sub(0x4154));
        for (k, &row) in SPIC_COPY_ORDER.iter().enumerate() {
            let s0 = c.g[S0][0] as u32;
            c.g[A1][0] = c.lw(gp.wrapping_add(0x7b4 + 8 * k as u32));
            c.g[A0][0] = sx(s0.wrapping_add(SPIC_CURVE_OFF + SPIC_ROW_SIZE * row));
            c.g[A2][0] = 0x20;
            call_ptr(c, SPIC_COPY, 0x002c_3e40 + 0x10 * k as u32)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        for k in 0..12 {
            frame.restore_fpr(c, 31 - k, 0x88 - 8 * k as u32);
        }
        frame.close(c);
        Ok(())
    });
}

/// Default fields of every row (row-relative byte offset, value), `f00`..`f8C` (the curve at `0x90` follows).
const SPIC_ROW_DEFAULTS: &[(u16, Src)] = &[
    (0x0, Src::Lit(0x44750000)),
    (0x4, Src::Lit(0x0)),
    (0x8, Src::Lit(0x3f800000)),
    (0xc, Src::Pool(-0x4150)),
    (0x10, Src::Lit(0x41a00000)),
    (0x14, Src::Lit(0x41200000)),
    (0x18, Src::Lit(0x41a00000)),
    (0x1c, Src::Lit(0x41f00000)),
    (0x20, Src::Lit(0x42520000)),
    (0x24, Src::Pool(-0x414c)),
    (0x28, Src::Pool(-0x4168)),
    (0x2c, Src::Pool(-0x4154)),
    (0x30, Src::Lit(0x3f800000)),
    (0x34, Src::Lit(0x41d80000)),
    (0x38, Src::Lit(0x41a00000)),
    (0x3c, Src::Lit(0x3f800000)),
    (0x40, Src::Lit(0x0)),
    (0x44, Src::Lit(0x0)),
    (0x48, Src::Lit(0x0)),
    (0x4c, Src::Lit(0x3f800000)),
    (0x50, Src::Lit(0x0)),
    (0x54, Src::Lit(0x42340000)),
    (0x58, Src::Lit(0x0)),
    (0x5c, Src::Lit(0x0)),
    (0x60, Src::Lit(0x0)),
    (0x64, Src::Lit(0x0)),
    (0x68, Src::Lit(0x0)),
    (0x6c, Src::Lit(0x0)),
    (0x70, Src::Lit(0x0)),
    (0x74, Src::Lit(0x0)),
    (0x78, Src::Lit(0x1)),
    (0x7c, Src::Pool(-0x4168)),
    (0x80, Src::Pool(-0x4168)),
    (0x84, Src::Lit(0x0)),
    (0x88, Src::Lit(0x0)),
    (0x8c, Src::Lit(0x0)),
];
/// Per-row overrides written after the loop (offset from the cache base, value): the tuned values of rows 0-17.
const SPIC_TAIL_FIELDS: &[(u16, Src)] = &[
    (0x8, Src::Pool(-0x4148)),
    (0x14, Src::Lit(0x3f000000)),
    (0xc4, Src::Lit(0x3f000000)),
    (0x28, Src::Pool(-0x4144)),
    (0x30, Src::Lit(0x3f800000)),
    (0x3c, Src::Lit(0x3f800000)),
    (0xd0, Src::Pool(-0x4140)),
    (0xd8, Src::Pool(-0x4144)),
    (0xe0, Src::Lit(0x3f800000)),
    (0xc, Src::Pool(-0x4128)),
    (0x20, Src::Pool(-0x4124)),
    (0xb8, Src::Pool(-0x4120)),
    (0xbc, Src::Pool(-0x411c)),
    (0x0, Src::Pool(-0x413c)),
    (0x10, Src::Pool(-0x4138)),
    (0x18, Src::Pool(-0x4134)),
    (0x1c, Src::Pool(-0x4130)),
    (0x24, Src::Pool(-0x412c)),
    (0x2c, Src::Pool(-0x4154)),
    (0x34, Src::Lit(0x41d80000)),
    (0x38, Src::Lit(0x41a00000)),
    (0xb0, Src::Pool(-0x413c)),
    (0xc0, Src::Pool(-0x4138)),
    (0xc8, Src::Pool(-0x4134)),
    (0xcc, Src::Pool(-0x4130)),
    (0xd4, Src::Pool(-0x412c)),
    (0xdc, Src::Pool(-0x4154)),
    (0xe4, Src::Lit(0x41d80000)),
    (0xe8, Src::Lit(0x41a00000)),
    (0x4, Src::Lit(0x0)),
    (0x40, Src::Lit(0x0)),
    (0x44, Src::Lit(0x0)),
    (0xb4, Src::Lit(0x0)),
    (0xec, Src::Lit(0x3f800000)),
    (0x160, Src::Pool(-0x4118)),
    (0x164, Src::Lit(0xbf800000)),
    (0x214, Src::Lit(0xbf800000)),
    (0x188, Src::Pool(-0x4144)),
    (0x190, Src::Lit(0x3f800000)),
    (0x238, Src::Pool(-0x4144)),
    (0x240, Src::Lit(0x3f800000)),
    (0x168, Src::Pool(-0x4114)),
    (0x16c, Src::Pool(-0x4110)),
    (0x170, Src::Pool(-0x410c)),
    (0x174, Src::Pool(-0x4108)),
    (0x178, Src::Pool(-0x4104)),
    (0x17c, Src::Pool(-0x4100)),
    (0x180, Src::Pool(-0x40fc)),
    (0x184, Src::Pool(-0x40f8)),
    (0x19c, Src::Pool(-0x40f4)),
    (0x1a0, Src::Pool(-0x40f0)),
    (0x210, Src::Pool(-0x40ec)),
    (0x218, Src::Pool(-0x40e8)),
    (0x21c, Src::Pool(-0x40e4)),
    (0x220, Src::Pool(-0x40e0)),
    (0x224, Src::Pool(-0x40dc)),
    (0x228, Src::Pool(-0x40d8)),
    (0x22c, Src::Pool(-0x40d4)),
    (0x230, Src::Pool(-0x40d0)),
    (0x234, Src::Pool(-0x40cc)),
    (0x18c, Src::Pool(-0x4154)),
    (0x194, Src::Lit(0x41d80000)),
    (0x198, Src::Lit(0x41a00000)),
    (0x23c, Src::Pool(-0x4154)),
    (0xf0, Src::Lit(0x0)),
    (0xf4, Src::Lit(0x0)),
    (0x1a4, Src::Lit(0x0)),
    (0x244, Src::Lit(0x41d80000)),
    (0x24c, Src::Pool(-0x40c8)),
    (0x2d4, Src::Lit(0x3f000000)),
    (0x384, Src::Lit(0x3f000000)),
    (0x2c0, Src::Pool(-0x40c0)),
    (0x2c4, Src::Lit(0x3f800000)),
    (0x2c8, Src::Pool(-0x40bc)),
    (0x2cc, Src::Pool(-0x40b8)),
    (0x2d0, Src::Pool(-0x40b4)),
    (0x2d8, Src::Pool(-0x40b0)),
    (0x2dc, Src::Pool(-0x40ac)),
    (0x2e8, Src::Pool(-0x4144)),
    (0x2f0, Src::Lit(0x3f800000)),
    (0x2fc, Src::Lit(0x3f800000)),
    (0x37c, Src::Pool(-0x40c4)),
    (0x390, Src::Pool(-0x4140)),
    (0x398, Src::Pool(-0x4144)),
    (0x250, Src::Pool(-0x4098)),
    (0x248, Src::Lit(0x41a00000)),
    (0x2e0, Src::Pool(-0x40a4)),
    (0x2e4, Src::Pool(-0x40a0)),
    (0x2ec, Src::Pool(-0x4154)),
    (0x2f4, Src::Pool(-0x409c)),
    (0x2f8, Src::Lit(0x41a00000)),
    (0x370, Src::Pool(-0x413c)),
    (0x378, Src::Pool(-0x40a8)),
    (0x380, Src::Pool(-0x4138)),
    (0x388, Src::Pool(-0x4134)),
    (0x38c, Src::Pool(-0x4130)),
    (0x394, Src::Pool(-0x412c)),
    (0x254, Src::Lit(0x0)),
    (0x300, Src::Lit(0x0)),
    (0x304, Src::Lit(0x0)),
    (0x374, Src::Lit(0x0)),
    (0x39c, Src::Pool(-0x4154)),
    (0x4d0, Src::Pool(-0x4094)),
    (0x434, Src::Lit(0x3f000000)),
    (0x4e4, Src::Lit(0x3f000000)),
    (0x4ec, Src::Lit(0x41f00000)),
    (0x3a0, Src::Lit(0x3f800000)),
    (0x3ac, Src::Lit(0x3f800000)),
    (0x42c, Src::Pool(-0x40c4)),
    (0x440, Src::Pool(-0x4140)),
    (0x448, Src::Pool(-0x4144)),
    (0x450, Src::Lit(0x3f800000)),
    (0x45c, Src::Lit(0x3f800000)),
    (0x4d8, Src::Pool(-0x4090)),
    (0x4dc, Src::Pool(-0x408c)),
    (0x4e0, Src::Pool(-0x4088)),
    (0x4e8, Src::Pool(-0x4084)),
    (0x4f0, Src::Pool(-0x4080)),
    (0x3a4, Src::Lit(0x41d80000)),
    (0x3a8, Src::Lit(0x41a00000)),
    (0x420, Src::Pool(-0x413c)),
    (0x428, Src::Pool(-0x40a8)),
    (0x430, Src::Pool(-0x4138)),
    (0x438, Src::Pool(-0x4134)),
    (0x43c, Src::Pool(-0x4130)),
    (0x444, Src::Pool(-0x412c)),
    (0x44c, Src::Pool(-0x4154)),
    (0x454, Src::Lit(0x41d80000)),
    (0x458, Src::Lit(0x41a00000)),
    (0x3b0, Src::Lit(0x0)),
    (0x3b4, Src::Lit(0x0)),
    (0x424, Src::Lit(0x0)),
    (0x460, Src::Lit(0x0)),
    (0x464, Src::Lit(0x0)),
    (0x4d4, Src::Lit(0x0)),
    (0x4f4, Src::Pool(-0x407c)),
    (0xc60, Src::Lit(0x44750000)),
    (0x4f8, Src::Pool(-0x4144)),
    (0xc6c, Src::Pool(-0x4078)),
    (0x500, Src::Lit(0x3f800000)),
    (0xc74, Src::Lit(0x41200000)),
    (0x50c, Src::Lit(0x3f800000)),
    (0xc7c, Src::Lit(0x41f00000)),
    (0xc68, Src::Lit(0x3f800000)),
    (0xc80, Src::Lit(0x42520000)),
    (0xc88, Src::Pool(-0x4144)),
    (0xc84, Src::Pool(-0x4074)),
    (0xc90, Src::Lit(0x3f800000)),
    (0xc9c, Src::Lit(0x3f800000)),
    (0x58c, Src::Pool(-0x40c4)),
    (0x4fc, Src::Pool(-0x4154)),
    (0x504, Src::Lit(0x41d80000)),
    (0x508, Src::Lit(0x41a00000)),
    (0xc70, Src::Lit(0x41a00000)),
    (0xc78, Src::Lit(0x41a00000)),
    (0xc8c, Src::Pool(-0x4154)),
    (0xc94, Src::Lit(0x41d80000)),
    (0xc98, Src::Lit(0x41a00000)),
    (0x580, Src::Pool(-0x413c)),
    (0x588, Src::Pool(-0x40a8)),
    (0x590, Src::Pool(-0x4138)),
    (0x594, Src::Lit(0x3f000000)),
    (0x598, Src::Pool(-0x4134)),
    (0x510, Src::Lit(0x0)),
    (0x514, Src::Lit(0x0)),
    (0xc64, Src::Lit(0x0)),
    (0xca0, Src::Lit(0x0)),
    (0xca4, Src::Lit(0x0)),
    (0x584, Src::Lit(0x0)),
    (0x59c, Src::Pool(-0x4130)),
    (0x5a0, Src::Pool(-0x4140)),
    (0x5a8, Src::Pool(-0x4144)),
    (0x5b0, Src::Lit(0x3f800000)),
    (0x5bc, Src::Lit(0x3f800000)),
    (0x63c, Src::Pool(-0x40c4)),
    (0x650, Src::Pool(-0x4140)),
    (0x658, Src::Pool(-0x4144)),
    (0x660, Src::Lit(0x3f800000)),
    (0x66c, Src::Lit(0x3f800000)),
    (0x6ec, Src::Pool(-0x40c4)),
    (0x5a4, Src::Pool(-0x412c)),
    (0x5ac, Src::Pool(-0x4154)),
    (0x5b4, Src::Lit(0x41d80000)),
    (0x5b8, Src::Lit(0x41a00000)),
    (0x630, Src::Pool(-0x413c)),
    (0x638, Src::Pool(-0x40a8)),
    (0x640, Src::Pool(-0x4138)),
    (0x644, Src::Lit(0x3f000000)),
    (0x648, Src::Pool(-0x4134)),
    (0x64c, Src::Pool(-0x4130)),
    (0x654, Src::Pool(-0x412c)),
    (0x65c, Src::Pool(-0x4154)),
    (0x664, Src::Lit(0x41d80000)),
    (0x668, Src::Lit(0x41a00000)),
    (0x6e0, Src::Pool(-0x413c)),
    (0x6e8, Src::Pool(-0x40a8)),
    (0x6f0, Src::Pool(-0x4138)),
    (0x5c0, Src::Lit(0x0)),
    (0x5c4, Src::Lit(0x0)),
    (0x634, Src::Lit(0x0)),
    (0x670, Src::Lit(0x0)),
    (0x674, Src::Lit(0x0)),
    (0x6e4, Src::Lit(0x0)),
    (0x6f4, Src::Lit(0x3f000000)),
    (0x7a4, Src::Lit(0x3f000000)),
    (0x790, Src::Pool(-0x40c0)),
    (0x798, Src::Pool(-0x40bc)),
    (0x79c, Src::Pool(-0x40b8)),
    (0x7a0, Src::Pool(-0x40b4)),
    (0x7a8, Src::Pool(-0x40b0)),
    (0x7ac, Src::Pool(-0x40ac)),
    (0x700, Src::Pool(-0x4140)),
    (0x708, Src::Pool(-0x4144)),
    (0x710, Src::Lit(0x3f800000)),
    (0x71c, Src::Lit(0x3f800000)),
    (0x794, Src::Lit(0x3f800000)),
    (0x7b8, Src::Pool(-0x4144)),
    (0x7c0, Src::Lit(0x3f800000)),
    (0x7cc, Src::Lit(0x3f800000)),
    (0x7b0, Src::Pool(-0x40a4)),
    (0x7b4, Src::Pool(-0x40a0)),
    (0x7c4, Src::Pool(-0x409c)),
    (0x840, Src::Pool(-0x413c)),
    (0x848, Src::Pool(-0x40a8)),
    (0x6f8, Src::Pool(-0x4134)),
    (0x6fc, Src::Pool(-0x4130)),
    (0x704, Src::Pool(-0x412c)),
    (0x70c, Src::Pool(-0x4154)),
    (0x714, Src::Lit(0x41d80000)),
    (0x718, Src::Lit(0x41a00000)),
    (0x7bc, Src::Pool(-0x4154)),
    (0x7c8, Src::Lit(0x41a00000)),
    (0x720, Src::Lit(0x0)),
    (0x724, Src::Lit(0x0)),
    (0x7d0, Src::Lit(0x0)),
    (0x7d4, Src::Lit(0x0)),
    (0x844, Src::Lit(0x0)),
    (0x84c, Src::Pool(-0x40c4)),
    (0x8f0, Src::Pool(-0x4070)),
    (0x910, Src::Pool(-0x406c)),
    (0x8f4, Src::Lit(0xbf800000)),
    (0x854, Src::Lit(0x3f000000)),
    (0x860, Src::Pool(-0x4140)),
    (0x90c, Src::Pool(-0x4068)),
    (0x914, Src::Pool(-0x4064)),
    (0x868, Src::Pool(-0x4144)),
    (0x870, Src::Lit(0x3f800000)),
    (0x87c, Src::Lit(0x3f800000)),
    (0x918, Src::Pool(-0x4144)),
    (0x920, Src::Lit(0x3f800000)),
    (0x92c, Src::Lit(0x3f800000)),
    (0x850, Src::Pool(-0x4138)),
    (0x858, Src::Pool(-0x4134)),
    (0x85c, Src::Pool(-0x4130)),
    (0x864, Src::Pool(-0x412c)),
    (0x8f8, Src::Pool(-0x4060)),
    (0x8fc, Src::Pool(-0x405c)),
    (0x900, Src::Pool(-0x4058)),
    (0x904, Src::Pool(-0x4054)),
    (0x908, Src::Pool(-0x4050)),
    (0x86c, Src::Pool(-0x4154)),
    (0x874, Src::Lit(0x41d80000)),
    (0x878, Src::Lit(0x41a00000)),
    (0x91c, Src::Pool(-0x4154)),
    (0x924, Src::Lit(0x41d80000)),
    (0x928, Src::Lit(0x41a00000)),
    (0x9a0, Src::Lit(0x44750000)),
    (0x880, Src::Lit(0x0)),
    (0x884, Src::Lit(0x0)),
    (0x930, Src::Lit(0x0)),
    (0x934, Src::Lit(0x0)),
    (0x9a4, Src::Lit(0x0)),
    (0x9ac, Src::Pool(-0x404c)),
    (0x9b4, Src::Lit(0x41200000)),
    (0x9bc, Src::Lit(0x41f00000)),
    (0x9c0, Src::Lit(0x42520000)),
    (0x9c4, Src::Pool(-0x4048)),
    (0xa50, Src::Lit(0x44750000)),
    (0xa5c, Src::Pool(-0x4044)),
    (0xa64, Src::Lit(0x41200000)),
    (0xa6c, Src::Lit(0x41f00000)),
    (0xa70, Src::Lit(0x42520000)),
    (0x9a8, Src::Lit(0x3f800000)),
    (0x9c8, Src::Pool(-0x4144)),
    (0x9d0, Src::Lit(0x3f800000)),
    (0x9dc, Src::Lit(0x3f800000)),
    (0xa58, Src::Lit(0x3f800000)),
    (0xa78, Src::Pool(-0x4144)),
    (0xa80, Src::Lit(0x3f800000)),
    (0xa8c, Src::Lit(0x3f800000)),
    (0x9b0, Src::Lit(0x41a00000)),
    (0x9b8, Src::Lit(0x41a00000)),
    (0x9cc, Src::Pool(-0x4154)),
    (0x9d4, Src::Lit(0x41d80000)),
    (0x9d8, Src::Lit(0x41a00000)),
    (0xa60, Src::Lit(0x41a00000)),
    (0xa68, Src::Lit(0x41a00000)),
    (0xa74, Src::Pool(-0x4040)),
    (0xa7c, Src::Pool(-0x4154)),
    (0xa84, Src::Lit(0x41d80000)),
    (0xa88, Src::Lit(0x41a00000)),
    (0x9e0, Src::Lit(0x0)),
    (0x9e4, Src::Lit(0x0)),
    (0xa54, Src::Lit(0x0)),
    (0xa90, Src::Lit(0x0)),
    (0xa94, Src::Lit(0x0)),
    (0xbb0, Src::Lit(0x44750000)),
    (0xbbc, Src::Pool(-0x403c)),
    (0xbc4, Src::Lit(0x41200000)),
    (0xbcc, Src::Lit(0x41f00000)),
    (0xbd0, Src::Lit(0x42520000)),
    (0xbd4, Src::Pool(-0x4038)),
    (0xb00, Src::Lit(0x44750000)),
    (0xb0c, Src::Pool(-0x4034)),
    (0xb14, Src::Lit(0x41200000)),
    (0xb1c, Src::Lit(0x41f00000)),
    (0xb20, Src::Lit(0x42520000)),
    (0xb08, Src::Lit(0x3f800000)),
    (0xb28, Src::Pool(-0x4144)),
    (0xb30, Src::Lit(0x3f800000)),
    (0xb3c, Src::Lit(0x3f800000)),
    (0xbb8, Src::Lit(0x3f800000)),
    (0xbd8, Src::Pool(-0x4144)),
    (0xbe0, Src::Lit(0x3f800000)),
    (0xbe4, Src::Lit(0x41d80000)),
    (0xbe8, Src::Lit(0x41a00000)),
    (0xb10, Src::Lit(0x41a00000)),
    (0xb18, Src::Lit(0x41a00000)),
    (0xb24, Src::Pool(-0x4030)),
    (0xb2c, Src::Pool(-0x4154)),
    (0xb34, Src::Lit(0x41d80000)),
    (0xb38, Src::Lit(0x41a00000)),
    (0xbc0, Src::Lit(0x41a00000)),
    (0xbc8, Src::Lit(0x41a00000)),
    (0xbdc, Src::Pool(-0x4154)),
    (0xb04, Src::Lit(0x0)),
    (0xb40, Src::Lit(0x0)),
    (0xb44, Src::Lit(0x0)),
    (0xbb4, Src::Lit(0x0)),
    (0xbec, Src::Lit(0x3f800000)),
    (0x5c, Src::Lit(0x40000000)),
    (0x68, Src::Pool(-0x402c)),
    (0x4c, Src::Lit(0x3f000000)),
    (0x48, Src::Lit(0x3f800000)),
    (0x58, Src::Lit(0x1)),
    (0x60, Src::Lit(0x1)),
    (0x54, Src::Lit(0x42340000)),
    (0xbf0, Src::Lit(0x0)),
    (0xbf4, Src::Lit(0x0)),
    (0x50, Src::Lit(0x0)),
    (0x64, Src::Lit(0x0)),
    (0x6c, Src::Lit(0x0)),
    (0x7c, Src::Pool(-0x4028)),
    (0x80, Src::Pool(-0x4024)),
    (0xfc, Src::Pool(-0x4020)),
    (0x10c, Src::Lit(0x40400000)),
    (0x124, Src::Pool(-0x401c)),
    (0x70, Src::Lit(0x3f800000)),
    (0x78, Src::Lit(0x1)),
    (0x88, Src::Lit(0x1)),
    (0xf8, Src::Lit(0x3f800000)),
    (0x100, Src::Lit(0x1)),
    (0x108, Src::Lit(0x1)),
    (0x110, Src::Lit(0x1)),
    (0x118, Src::Pool(-0x4018)),
    (0x120, Src::Pool(-0x4144)),
    (0x128, Src::Lit(0x1)),
    (0x74, Src::Pool(-0x4154)),
    (0x104, Src::Lit(0x42340000)),
    (0x84, Src::Lit(0x0)),
    (0x8c, Src::Lit(0x0)),
    (0x114, Src::Lit(0x0)),
    (0x11c, Src::Lit(0x0)),
    (0x12c, Src::Pool(-0x4014)),
    (0x1bc, Src::Lit(0x40800000)),
    (0x138, Src::Lit(0x1)),
    (0x1a8, Src::Lit(0x3f800000)),
    (0x1ac, Src::Lit(0x3f800000)),
    (0x1b0, Src::Lit(0x1)),
    (0x1b8, Src::Lit(0x1)),
    (0x1c0, Src::Lit(0x1)),
    (0x1c4, Src::Lit(0x1)),
    (0x1c8, Src::Pool(-0x4018)),
    (0x1d0, Src::Lit(0x3f800000)),
    (0x1d4, Src::Pool(-0x4010)),
    (0x1d8, Src::Lit(0x1)),
    (0x1dc, Src::Pool(-0x4144)),
    (0x1e0, Src::Pool(-0x4144)),
    (0x1e4, Src::Lit(0x1)),
    (0x1e8, Src::Lit(0x1)),
    (0x258, Src::Lit(0x3f800000)),
    (0x25c, Src::Pool(-0x400c)),
    (0x260, Src::Lit(0x1)),
    (0x268, Src::Lit(0x1)),
    (0x26c, Src::Lit(0x40a00000)),
    (0x270, Src::Lit(0x1)),
    (0x274, Src::Lit(0x1)),
    (0x278, Src::Pool(-0x4018)),
    (0x280, Src::Lit(0x3f800000)),
    (0x130, Src::Pool(-0x4008)),
    (0x1b4, Src::Lit(0x42340000)),
    (0x264, Src::Lit(0x42340000)),
    (0x134, Src::Lit(0x0)),
    (0x13c, Src::Lit(0x0)),
    (0x1cc, Src::Lit(0x0)),
    (0x1ec, Src::Lit(0x0)),
    (0x27c, Src::Lit(0x0)),
    (0x284, Src::Pool(-0x4010)),
    (0x288, Src::Lit(0x1)),
    (0x28c, Src::Pool(-0x4144)),
    (0x290, Src::Pool(-0x4144)),
    (0x294, Src::Lit(0x1)),
    (0x298, Src::Lit(0x1)),
    (0x330, Src::Lit(0x3f000000)),
    (0x33c, Src::Pool(-0x4144)),
    (0x340, Src::Pool(-0x4144)),
    (0x348, Src::Lit(0x1)),
    (0x3bc, Src::Lit(0x3f000000)),
    (0x314, Src::Lit(0x42340000)),
    (0x3c4, Src::Lit(0x42340000)),
    (0x29c, Src::Lit(0x0)),
    (0x308, Src::Lit(0x0)),
    (0x30c, Src::Lit(0x0)),
    (0x310, Src::Lit(0x0)),
    (0x318, Src::Lit(0x0)),
    (0x31c, Src::Lit(0x0)),
    (0x320, Src::Lit(0x0)),
    (0x324, Src::Lit(0x0)),
    (0x328, Src::Lit(0x0)),
    (0x32c, Src::Lit(0x0)),
    (0x334, Src::Lit(0x0)),
    (0x338, Src::Lit(0x0)),
    (0x344, Src::Lit(0x0)),
    (0x34c, Src::Lit(0x0)),
    (0x3b8, Src::Lit(0x0)),
    (0x3c0, Src::Lit(0x0)),
    (0x3c8, Src::Lit(0x0)),
    (0x3cc, Src::Lit(0x0)),
    (0x3d0, Src::Lit(0x0)),
    (0x3d4, Src::Lit(0x0)),
    (0x3d8, Src::Lit(0x0)),
    (0x3dc, Src::Lit(0x0)),
    (0x3e0, Src::Lit(0x3f800000)),
    (0x3e4, Src::Pool(-0x4010)),
    (0x3e8, Src::Lit(0x1)),
    (0x3ec, Src::Pool(-0x4144)),
    (0x3f0, Src::Pool(-0x4144)),
    (0x46c, Src::Pool(-0x4010)),
    (0x490, Src::Lit(0x3f800000)),
    (0x494, Src::Pool(-0x4010)),
    (0x498, Src::Lit(0x1)),
    (0x49c, Src::Pool(-0x4144)),
    (0x4a0, Src::Pool(-0x4144)),
    (0x51c, Src::Lit(0x3f800000)),
    (0x468, Src::Lit(0x3f400000)),
    (0x474, Src::Lit(0x42340000)),
    (0x524, Src::Lit(0x42340000)),
    (0x3f4, Src::Lit(0x0)),
    (0x3f8, Src::Lit(0x0)),
    (0x3fc, Src::Lit(0x0)),
    (0x470, Src::Lit(0x0)),
    (0x478, Src::Lit(0x0)),
    (0x47c, Src::Lit(0x0)),
    (0x480, Src::Lit(0x0)),
    (0x484, Src::Lit(0x0)),
    (0x488, Src::Lit(0x0)),
    (0x48c, Src::Lit(0x0)),
    (0x4a4, Src::Lit(0x0)),
    (0x4a8, Src::Lit(0x0)),
    (0x4ac, Src::Lit(0x0)),
    (0x518, Src::Lit(0x0)),
    (0x520, Src::Lit(0x0)),
    (0x528, Src::Lit(0x0)),
    (0x52c, Src::Lit(0x0)),
    (0x530, Src::Lit(0x0)),
    (0x534, Src::Lit(0x0)),
    (0x53c, Src::Pool(-0x4018)),
    (0x540, Src::Lit(0x3f800000)),
    (0x544, Src::Lit(0x3f800000)),
    (0x548, Src::Lit(0x1)),
    (0x54c, Src::Pool(-0x4144)),
    (0x550, Src::Pool(-0x4144)),
    (0x55c, Src::Lit(0x1)),
    (0xcac, Src::Lit(0x3f800000)),
    (0xccc, Src::Pool(-0x4018)),
    (0xcd0, Src::Lit(0x3f800000)),
    (0xcd4, Src::Lit(0x3f800000)),
    (0xcd8, Src::Lit(0x1)),
    (0xcdc, Src::Pool(-0x4144)),
    (0xce0, Src::Pool(-0x4144)),
    (0xcec, Src::Lit(0x1)),
    (0xcb4, Src::Lit(0x42340000)),
    (0x5d4, Src::Lit(0x42340000)),
    (0x538, Src::Lit(0x0)),
    (0x554, Src::Lit(0x0)),
    (0x558, Src::Lit(0x0)),
    (0xca8, Src::Lit(0x0)),
    (0xcb0, Src::Lit(0x0)),
    (0xcb8, Src::Lit(0x0)),
    (0xcbc, Src::Lit(0x0)),
    (0xcc0, Src::Lit(0x0)),
    (0xcc4, Src::Lit(0x0)),
    (0xcc8, Src::Lit(0x0)),
    (0xce4, Src::Lit(0x0)),
    (0xce8, Src::Lit(0x0)),
    (0x5c8, Src::Lit(0x0)),
    (0x5cc, Src::Lit(0x0)),
    (0x5d0, Src::Lit(0x0)),
    (0x5d8, Src::Lit(0x0)),
    (0x5dc, Src::Lit(0x0)),
    (0x5ec, Src::Pool(-0x4004)),
    (0x5f0, Src::Lit(0x3f800000)),
    (0x5f4, Src::Pool(-0x4010)),
    (0x5f8, Src::Lit(0x1)),
    (0x5fc, Src::Pool(-0x4144)),
    (0x600, Src::Pool(-0x4144)),
    (0x60c, Src::Lit(0x1)),
    (0x6a0, Src::Lit(0x3f800000)),
    (0x6a4, Src::Pool(-0x4010)),
    (0x6a8, Src::Lit(0x1)),
    (0x6ac, Src::Pool(-0x4144)),
    (0x6b0, Src::Pool(-0x4144)),
    (0x6b8, Src::Lit(0x1)),
    (0x684, Src::Lit(0x42340000)),
    (0x5e0, Src::Lit(0x0)),
    (0x5e4, Src::Lit(0x0)),
    (0x5e8, Src::Lit(0x0)),
    (0x604, Src::Lit(0x0)),
    (0x608, Src::Lit(0x0)),
    (0x678, Src::Lit(0x0)),
    (0x67c, Src::Lit(0x0)),
    (0x680, Src::Lit(0x0)),
    (0x688, Src::Lit(0x0)),
    (0x68c, Src::Lit(0x0)),
    (0x690, Src::Lit(0x0)),
    (0x694, Src::Lit(0x0)),
    (0x698, Src::Lit(0x0)),
    (0x69c, Src::Lit(0x0)),
    (0x6b4, Src::Lit(0x0)),
    (0x6bc, Src::Lit(0x0)),
    (0x728, Src::Lit(0x0)),
    (0x72c, Src::Lit(0x0)),
    (0x730, Src::Lit(0x0)),
    (0x734, Src::Lit(0x42340000)),
    (0x800, Src::Lit(0x3f000000)),
    (0x750, Src::Lit(0x3f800000)),
    (0x754, Src::Pool(-0x4010)),
    (0x758, Src::Lit(0x1)),
    (0x75c, Src::Pool(-0x4144)),
    (0x760, Src::Pool(-0x4144)),
    (0x76c, Src::Lit(0x1)),
    (0x80c, Src::Pool(-0x4144)),
    (0x810, Src::Pool(-0x4144)),
    (0x818, Src::Lit(0x1)),
    (0x7e4, Src::Lit(0x42340000)),
    (0x738, Src::Lit(0x0)),
    (0x73c, Src::Lit(0x0)),
    (0x740, Src::Lit(0x0)),
    (0x744, Src::Lit(0x0)),
    (0x748, Src::Lit(0x0)),
    (0x74c, Src::Lit(0x0)),
    (0x764, Src::Lit(0x0)),
    (0x768, Src::Lit(0x0)),
    (0x7d8, Src::Lit(0x0)),
    (0x7dc, Src::Lit(0x0)),
    (0x7e0, Src::Lit(0x0)),
    (0x7e8, Src::Lit(0x0)),
    (0x7ec, Src::Lit(0x0)),
    (0x7f0, Src::Lit(0x0)),
    (0x7f4, Src::Lit(0x0)),
    (0x7f8, Src::Lit(0x0)),
    (0x7fc, Src::Lit(0x0)),
    (0x804, Src::Lit(0x0)),
    (0x808, Src::Lit(0x0)),
    (0x814, Src::Lit(0x0)),
    (0x81c, Src::Lit(0x0)),
    (0x888, Src::Lit(0x0)),
    (0x88c, Src::Lit(0x0)),
    (0x93c, Src::Pool(-0x400c)),
    (0x94c, Src::Lit(0x40a00000)),
    (0x8b0, Src::Lit(0x3f800000)),
    (0x8b4, Src::Pool(-0x4010)),
    (0x8b8, Src::Lit(0x1)),
    (0x8bc, Src::Pool(-0x4144)),
    (0x8c0, Src::Pool(-0x4144)),
    (0x938, Src::Lit(0x3f800000)),
    (0x940, Src::Lit(0x1)),
    (0x948, Src::Lit(0x1)),
    (0x950, Src::Lit(0x1)),
    (0x954, Src::Lit(0x1)),
    (0x958, Src::Pool(-0x4018)),
    (0x960, Src::Lit(0x3f800000)),
    (0x964, Src::Pool(-0x4010)),
    (0x968, Src::Lit(0x1)),
    (0x96c, Src::Pool(-0x4144)),
    (0x970, Src::Pool(-0x4144)),
    (0x974, Src::Lit(0x1)),
    (0x978, Src::Lit(0x1)),
    (0x894, Src::Lit(0x42340000)),
    (0x944, Src::Lit(0x42340000)),
    (0x890, Src::Lit(0x0)),
    (0x898, Src::Lit(0x0)),
    (0x89c, Src::Lit(0x0)),
    (0x8a0, Src::Lit(0x0)),
    (0x8a4, Src::Lit(0x0)),
    (0x8a8, Src::Lit(0x0)),
    (0x8ac, Src::Lit(0x0)),
    (0x8c4, Src::Lit(0x0)),
    (0x8c8, Src::Lit(0x0)),
    (0x8cc, Src::Lit(0x0)),
    (0x95c, Src::Lit(0x0)),
    (0x97c, Src::Lit(0x0)),
    (0xa10, Src::Pool(-0x4010)),
    (0x9ec, Src::Lit(0x3f800000)),
    (0xa14, Src::Lit(0x3f800000)),
    (0xa1c, Src::Pool(-0x4144)),
    (0xa20, Src::Pool(-0x4144)),
    (0xa28, Src::Lit(0x1)),
    (0xa9c, Src::Lit(0x3f800000)),
    (0xac0, Src::Lit(0x3f800000)),
    (0xac4, Src::Lit(0x3f800000)),
    (0xac8, Src::Lit(0x1)),
    (0xacc, Src::Pool(-0x4144)),
    (0xad0, Src::Pool(-0x4144)),
    (0x9f4, Src::Lit(0x42340000)),
    (0xaa4, Src::Lit(0x42340000)),
    (0x9e8, Src::Lit(0x0)),
    (0x9f0, Src::Lit(0x0)),
    (0x9f8, Src::Lit(0x0)),
    (0x9fc, Src::Lit(0x0)),
    (0xa00, Src::Lit(0x0)),
    (0xa04, Src::Lit(0x0)),
    (0xa08, Src::Lit(0x0)),
    (0xa0c, Src::Lit(0x0)),
    (0xa18, Src::Lit(0x0)),
    (0xa24, Src::Lit(0x0)),
    (0xa2c, Src::Lit(0x0)),
    (0xa98, Src::Lit(0x0)),
    (0xaa0, Src::Lit(0x0)),
    (0xaa8, Src::Lit(0x0)),
    (0xaac, Src::Lit(0x0)),
    (0xab0, Src::Lit(0x0)),
    (0xab4, Src::Lit(0x0)),
    (0xab8, Src::Lit(0x0)),
    (0xabc, Src::Lit(0x0)),
    (0xad4, Src::Lit(0x0)),
    (0xb6c, Src::Pool(-0x4018)),
    (0xc24, Src::Lit(0x3f800000)),
    (0xb4c, Src::Lit(0x3f800000)),
    (0xb70, Src::Lit(0x3f800000)),
    (0xb74, Src::Lit(0x3f800000)),
    (0xb78, Src::Lit(0x1)),
    (0xb7c, Src::Pool(-0x4144)),
    (0xb80, Src::Pool(-0x4144)),
    (0xb8c, Src::Lit(0x1)),
    (0xbfc, Src::Lit(0x3f800000)),
    (0xc20, Src::Lit(0x3f800000)),
    (0xc28, Src::Lit(0x1)),
    (0xc04, Src::Lit(0x42340000)),
    (0xb54, Src::Lit(0x42340000)),
    (0xad8, Src::Lit(0x0)),
    (0xadc, Src::Lit(0x0)),
    (0xb48, Src::Lit(0x0)),
    (0xb50, Src::Lit(0x0)),
    (0xb58, Src::Lit(0x0)),
    (0xb5c, Src::Lit(0x0)),
    (0xb60, Src::Lit(0x0)),
    (0xb64, Src::Lit(0x0)),
    (0xb68, Src::Lit(0x0)),
    (0xb84, Src::Lit(0x0)),
    (0xb88, Src::Lit(0x0)),
    (0xbf8, Src::Lit(0x0)),
    (0xc00, Src::Lit(0x0)),
    (0xc08, Src::Lit(0x0)),
    (0xc0c, Src::Lit(0x0)),
    (0xc10, Src::Lit(0x0)),
    (0xc14, Src::Lit(0x0)),
    (0xc18, Src::Lit(0x0)),
    (0xc1c, Src::Lit(0x0)),
    (0xc2c, Src::Pool(-0x4144)),
    (0xc34, Src::Lit(0x0)),
    (0xc38, Src::Lit(0x0)),
    (0xc30, Src::Pool(-0x4144)),
    (0xc3c, Src::Lit(0x1)),
];
/// The four curve points (x, y) every row starts with.
const SPIC_CURVE: [(Src, Src); 4] = [
    (Src::Lit(0x0), Src::Pool(-0x4164)),
    (Src::Lit(0x42200000), Src::Pool(-0x4160)),
    (Src::Lit(0x42a00000), Src::Pool(-0x415c)),
    (Src::Lit(0x42f00000), Src::Pool(-0x4158)),
];

/// Registers at the end of a loop iteration (`row` = the row just filled, `next` = the next index).
fn spic_loop_regs(c: &mut Cpu, gp: u32, s0: u32, row: u32, next: u32) {
    c.g[AT][0] = 0x4252_0000;
    c.g[T7][0] = next as u64;
    c.g[T5][0] = (next < 19) as u64;
    c.g[T6][0] = 1;
    c.g[T8][0] = sx(s0.wrapping_add(0x6c));
    c.g[V0][0] = 0xb0;
    for r in [V1, A0, A1, A2, A3, T0, T1, T2, T3, T4] {
        c.g[r][0] = sx(row);
    }
    c.f[0] = 0x3f800000;
    c.lwc1(1, gp.wrapping_sub(0x4168));
    c.f[2] = 0x41a00000;
    c.lwc1(3, gp.wrapping_sub(0x4158));
    c.f[4] = 0x42f00000;
    c.lwc1(5, gp.wrapping_sub(0x415c));
    c.f[6] = 0x42a00000;
    c.lwc1(7, gp.wrapping_sub(0x4160));
    c.f[8] = 0x42200000;
    c.lwc1(9, gp.wrapping_sub(0x4164));
    c.lwc1(10, gp.wrapping_sub(0x414c));
    c.f[29] = 0x42340000;
    c.f[30] = 0x41d80000;
    c.lwc1(31, gp.wrapping_sub(0x4154));
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::schedule;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP, RA as SENTINEL};

    /// A cache at a random 16-aligned address with random bytes around, a
    /// random literal pool at `gp - 0x4168 ..` (every pool offset the
    /// function reads is checked) and the 19 curve sources at `gp + 0x7b4`.
    fn spic_fixture(c: &mut Cpu, r: &mut Rng) {
        c.g[GP][0] = GAME_GP as u64;
        let gp = GAME_GP;
        fill(c, r, gp - 0x4170, 0x60);
        let s0 = DATA + 0x1000 + (r.next_u32() % 64) * 16;
        fill(c, r, s0, 0x360);
        for k in 0..SPIC_ROWS {
            let from = DATA + 0x20_0000 + k * 0x40;
            fill(c, r, from, 16);
            c.bus.write32(gp + 0x7b4 + 8 * k, from);
        }
        c.g[A0][0] = sx(s0);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002c_2f18;
    }

    /// `SurfPropCache_InitDefaults`: all rows, tuned fields, curve copies,
    /// the registers and cycles left behind (no events).
    #[test]
    fn surf_prop_cache_init_defaults_matches() {
        check(
            ee::s08::f_002c2f18__generated,
            surf_prop_cache_init_defaults,
            24,
            &|c, seed| {
                let mut r = Rng(0x2c2f_1800 ^ seed as u64);
                scramble(c, &mut r);
                spic_fixture(c, &mut r);
            },
        );
    }

    /// Events delivered at the row loop's back edge (0x2c2fa0), delivered
    /// and unwind-requesting.
    #[test]
    fn surf_prop_cache_init_defaults_events_match() {
        check(
            ee::s08::f_002c2f18__generated,
            surf_prop_cache_init_defaults,
            90,
            &|c, seed| {
                let mut r = Rng(0x2c2f_1801 ^ seed as u64);
                scramble(c, &mut r);
                spic_fixture(c, &mut r);
                schedule(c, &mut r, seed, 0x002c_2f18);
            },
        );
    }
}
