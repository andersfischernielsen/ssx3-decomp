//! Staging module oc1040 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1040`.

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
    0x002c_0b70,
    world_painter_query_static_init_destroy,
    "WorldPainterQuery_StaticInitDestroy",
)];

/// One float component of a static colour: a constant, or a word of the gp
/// constant pool (`gp - n`, the `.lit4` values the compiler hoisted).
#[derive(Clone, Copy)]
pub(crate) enum Oc1040Comp {
    Bits(u32),
    Pool(i32),
}
use Oc1040Comp::{Bits, Pool};

/// Address of the first `sColor` (16 bytes each, 56 of them).
pub(crate) const OC1040_COLOR_BASE: u32 = 0x004d_4638;
pub(crate) const OC1040_COLOR_COUNT: usize = 56;

/// The 56 static world-painter colours, 4 floats each, in address order.
pub(crate) static OC1040_COLORS: [[Oc1040Comp; 4]; OC1040_COLOR_COUNT] = [
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4638
    [
        Pool(-0x427c),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4648
    [
        Pool(-0x4278),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4658
    [Pool(-0x4274), Pool(-0x4254), Pool(-0x4250), Pool(-0x4270)], // 0x4d4668
    [
        Pool(-0x426c),
        Pool(-0x424c),
        Pool(-0x4248),
        Bits(0x00000000),
    ], // 0x4d4678
    [
        Pool(-0x4268),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4688
    [Pool(-0x4264), Pool(-0x4244), Pool(-0x4240), Pool(-0x4260)], // 0x4d4698
    [
        Pool(-0x425c),
        Pool(-0x423c),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d46a8
    [
        Pool(-0x4258),
        Pool(-0x4238),
        Pool(-0x4228),
        Bits(0x00000000),
    ], // 0x4d46b8
    [Pool(-0x4234), Pool(-0x4224), Pool(-0x4220), Pool(-0x4230)], // 0x4d46c8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d46d8
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d46e8
    [
        Bits(0x3f800000),
        Pool(-0x422c),
        Pool(-0x421c),
        Pool(-0x422c),
    ], // 0x4d46f8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4708
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4718
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4728
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4738
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4748
    [
        Bits(0x3f800000),
        Bits(0x3f000000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4758
    [
        Bits(0x3f800000),
        Bits(0x3f000000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4768
    [
        Bits(0x3f800000),
        Bits(0x3f000000),
        Bits(0x3f800000),
        Bits(0x3f000000),
    ], // 0x4d4778
    [
        Bits(0x3f800000),
        Pool(-0x4210),
        Pool(-0x420c),
        Pool(-0x4218),
    ], // 0x4d4788
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x4208),
        Bits(0x00000000),
    ], // 0x4d4798
    [
        Pool(-0x4214),
        Pool(-0x4204),
        Pool(-0x4200),
        Bits(0x00000000),
    ], // 0x4d47a8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d47b8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x41e8),
        Bits(0x00000000),
    ], // 0x4d47c8
    [
        Bits(0x3f800000),
        Pool(-0x41e4),
        Pool(-0x41e0),
        Bits(0x00000000),
    ], // 0x4d47d8
    [
        Bits(0x3f800000),
        Pool(-0x41dc),
        Pool(-0x41d8),
        Pool(-0x41fc),
    ], // 0x4d47e8
    [
        Bits(0x3f800000),
        Pool(-0x41d0),
        Pool(-0x41d4),
        Bits(0x3f800000),
    ], // 0x4d47f8
    [
        Bits(0x3f800000),
        Pool(-0x41f4),
        Bits(0x3f800000),
        Pool(-0x41f8),
    ], // 0x4d4808
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4818
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4828
    [
        Bits(0x3f800000),
        Pool(-0x41f0),
        Pool(-0x41ec),
        Bits(0x00000000),
    ], // 0x4d4838
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4848
    [
        Bits(0x3f800000),
        Pool(-0x41bc),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4858
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d4868
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4878
    [
        Pool(-0x41cc),
        Bits(0x00000000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4888
    [
        Bits(0x3f800000),
        Pool(-0x41b8),
        Pool(-0x41b4),
        Pool(-0x41c8),
    ], // 0x4d4898
    [
        Bits(0x3f800000),
        Pool(-0x41b0),
        Bits(0x3f800000),
        Pool(-0x41c4),
    ], // 0x4d48a8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x41ac),
        Bits(0x00000000),
    ], // 0x4d48b8
    [
        Bits(0x3f800000),
        Pool(-0x41a8),
        Bits(0x3f800000),
        Pool(-0x41c0),
    ], // 0x4d48c8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x3f800000),
    ], // 0x4d48d8
    [
        Bits(0x3f800000),
        Pool(-0x41d0),
        Bits(0x3f800000),
        Pool(-0x41d0),
    ], // 0x4d48e8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d48f8
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4908
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d4918
    [
        Bits(0x3f800000),
        Pool(-0x419c),
        Bits(0x3f800000),
        Pool(-0x41a4),
    ], // 0x4d4928
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x4198),
        Pool(-0x41a0),
    ], // 0x4d4938
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Pool(-0x4194),
        Bits(0x00000000),
    ], // 0x4d4948
    [
        Bits(0x3f800000),
        Pool(-0x4190),
        Pool(-0x418c),
        Bits(0x00000000),
    ], // 0x4d4958
    [
        Bits(0x3f800000),
        Pool(-0x4180),
        Pool(-0x417c),
        Bits(0x00000000),
    ], // 0x4d4968
    [
        Bits(0x3f800000),
        Pool(-0x4178),
        Pool(-0x4174),
        Pool(-0x4188),
    ], // 0x4d4978
    [
        Bits(0x3f800000),
        Pool(-0x4170),
        Pool(-0x416c),
        Pool(-0x4184),
    ], // 0x4d4988
    [
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x3f800000),
        Bits(0x00000000),
    ], // 0x4d4998
    [
        Bits(0x3f800000),
        Bits(0x3f800000),
        Bits(0x00000000),
        Bits(0x00000000),
    ], // 0x4d49a8
];

/// The gp-relative stores that follow (`is_byte`, gp offset, value): the
/// region-type table (the id as a byte, then replicated in a word), in the
/// order of the generated body.
pub(crate) static OC1040_GP_STORES: &[(bool, u32, u32)] = &[
    (true, 0x2300, 0x3),
    (true, 0x22e8, 0x0),
    (true, 0x22f0, 0x1),
    (true, 0x22f8, 0x2),
    (false, 0x22e8, 0x0),
    (false, 0x22f0, 0x101),
    (false, 0x22f8, 0x202),
    (false, 0x22e4, 0x0),
    (false, 0x22e0, 0x0),
    (false, 0x2300, 0x303),
    (true, 0x2308, 0x4),
    (true, 0x2310, 0x5),
    (true, 0x2340, 0xb),
    (true, 0x2350, 0xd),
    (true, 0x2318, 0x6),
    (true, 0x2320, 0x7),
    (true, 0x2328, 0x8),
    (true, 0x2330, 0x9),
    (true, 0x2338, 0xa),
    (true, 0x2348, 0xc),
    (false, 0x2308, 0x404),
    (false, 0x2310, 0x505),
    (false, 0x2318, 0x606),
    (false, 0x2320, 0x707),
    (false, 0x2328, 0x808),
    (false, 0x2330, 0x909),
    (false, 0x2338, 0xa0a),
    (false, 0x2340, 0xb0b),
    (false, 0x2348, 0xc0c),
    (false, 0x2350, 0xd0d),
];

/// FPU registers the generated body leaves behind (last `lwc1`/`mtc1`).
pub(crate) static OC1040_FINAL_F: &[(usize, Oc1040Comp)] = &[
    (0, Bits(0x3f800000)),
    (1, Pool(-0x4184)),
    (2, Pool(-0x4188)),
    (3, Pool(-0x4180)),
    (4, Pool(-0x417c)),
    (5, Pool(-0x4178)),
    (6, Pool(-0x4174)),
    (7, Pool(-0x4170)),
    (8, Pool(-0x416c)),
    (9, Pool(-0x41ac)),
    (10, Pool(-0x41a8)),
    (11, Pool(-0x41ec)),
    (12, Pool(-0x41d0)),
    (13, Pool(-0x424c)),
    (14, Pool(-0x4248)),
    (15, Pool(-0x4244)),
    (16, Pool(-0x4240)),
    (17, Pool(-0x4254)),
    (18, Pool(-0x4250)),
    (19, Pool(-0x4238)),
];

/// `void* WorldPainterQuery_StaticInitDestroy(int initialize, int priority)`
/// (0x2c0b70): the compiler's static-init/destroy function of the world
/// painter query unit. Only for `priority == 0xffff` and `initialize != 0` it
/// constructs the 56 static `sColor`s (0x4d4638..0x4d49b8), clears the null
/// name field and builds the 14 `tWorldPainterRegionType`s (ids 0..13, byte
/// and replicated-word forms at gp+0x22e0..0x2350). Every other call does
/// nothing. The pool constants are loaded at run time, as the original does,
/// and the scratch registers it leaves behind are reproduced.
pub fn world_painter_query_static_init_destroy(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x90));
    c.g[V0][0] = 0xffff;
    // sq s0..s7 at 0x80..0x10(sp)
    for i in 0..8usize {
        let a = sp(c).wrapping_add(0x80 - 0x10 * i as u32);
        c.sq(16 + i, a);
    }
    let priority_other = c.g[A1][0] != c.g[V0][0];
    let a = sp(c);
    c.sq(30, a);
    if !priority_other {
        c.cycles += 2;
        let initialize = c.g[A0][0] != 0;
        c.g[T5][0] = 0x4d0000;
        if initialize {
            oc1040_construct_statics(c);
        }
    }
    c.cycles += 11;
    for i in 0..8usize {
        let a = sp(c).wrapping_add(0x80 - 0x10 * i as u32);
        c.lq(16 + i, a);
    }
    let a = sp(c);
    c.lq(30, a);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
    c.pc = ra;
}

/// The initialising branch of [`world_painter_query_static_init_destroy`].
fn oc1040_construct_statics(c: &mut Cpu) {
    c.cycles += 544;
    let gp = c.g[GP][0] as u32;
    let comp = |c: &mut Cpu, k: Oc1040Comp| match k {
        Bits(b) => b,
        Pool(off) => c.bus.read32(gp.wrapping_add(off as u32)),
    };
    for (i, color) in OC1040_COLORS.iter().enumerate() {
        for (j, k) in color.iter().enumerate() {
            let v = comp(c, *k);
            Addr(OC1040_COLOR_BASE + 16 * i as u32 + 4 * j as u32).sw(c, v as u64);
        }
    }
    for &(is_byte, off, v) in OC1040_GP_STORES {
        let a = Addr(gp.wrapping_add(off));
        if is_byte {
            a.sb(c, v as u64);
        } else {
            a.sw(c, v as u64);
        }
    }
    for &(n, k) in OC1040_FINAL_F {
        c.f[n] = comp(c, k);
    }
    // scratch registers left by the generated body (s0..s7/fp are restored)
    c.g[AT][0] = 0x3f000000;
    c.g[T0][0] = 0x404;
    c.g[T1][0] = 0x707;
    c.g[T2][0] = 0x505;
    c.g[T3][0] = 0x606;
    c.g[T4][0] = 0x4d48b8;
    c.g[T5][0] = 0x6;
    c.g[T6][0] = 0x4d4948;
    c.g[T7][0] = 0x4d4938;
    c.g[T8][0] = 0x4d0000;
    c.g[T9][0] = 0x4d0000;
    c.g[A0][0] = 0xb0b;
    c.g[A1][0] = 0xa0a;
    c.g[A2][0] = 0x909;
    c.g[A3][0] = 0x808;
    c.g[V0][0] = 0xd0d;
    c.g[V1][0] = 0xc0c;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_paths, scramble, Rng, GAME_GP};

    /// Pool offsets (relative to gp) read by the colours and final FPRs.
    const OC1040_POOL: &[i32] = &[
        -0x416c, -0x4170, -0x4174, -0x4178, -0x417c, -0x4180, -0x4184, -0x4188, -0x418c, -0x4190,
        -0x4194, -0x4198, -0x419c, -0x41a0, -0x41a4, -0x41a8, -0x41ac, -0x41b0, -0x41b4, -0x41b8,
        -0x41bc, -0x41c0, -0x41c4, -0x41c8, -0x41cc, -0x41d0, -0x41d4, -0x41d8, -0x41dc, -0x41e0,
        -0x41e4, -0x41e8, -0x41ec, -0x41f0, -0x41f4, -0x41f8, -0x41fc, -0x4200, -0x4204, -0x4208,
        -0x420c, -0x4210, -0x4214, -0x4218, -0x421c, -0x4220, -0x4224, -0x4228, -0x422c, -0x4230,
        -0x4234, -0x4238, -0x423c, -0x4240, -0x4244, -0x4248, -0x424c, -0x4250, -0x4254, -0x4258,
        -0x425c, -0x4260, -0x4264, -0x4268, -0x426c, -0x4270, -0x4274, -0x4278, -0x427c,
    ];

    /// Three paths: wrong priority (23 cycles), `initialize == 0` (25) and
    /// the full init (569), with random bit patterns in the constant pool.
    #[test]
    fn world_painter_query_static_init_destroy_matches() {
        check_paths(
            ee::s08::f_002c0b70__generated,
            world_painter_query_static_init_destroy,
            90,
            &[23, 25, 569],
            &|c, seed| {
                let mut r = Rng(0x6c0b_7000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                for off in OC1040_POOL {
                    let v = r.next_u32();
                    c.bus.write32(GAME_GP.wrapping_add(*off as u32), v);
                }
                let (a0, a1) = match seed % 3 {
                    0 => (1 + (r.next_u32() as u64 & 0xff), r.next_u32() as u64),
                    1 => (0, 0xffff),
                    _ => (1 + (r.next_u32() as u64 & 0xff), 0xffff),
                };
                c.g[A0][0] = a0;
                c.g[A1][0] = a1;
                c.g[RA][0] = crate::difftest::RA as u64;
            },
        );
    }
}
