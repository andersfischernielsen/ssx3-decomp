//! `src/animation/gameanimmap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
pub(crate) use crate::game::wscript::wscriptman::static_ctor_stub;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use Op::Imm;
use Op::Pool;
use Op::Store;
use Op::Zero;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_50b0, anim_data_empty_hook, "AnimData_EmptyHook"),
    (
        0x0031_5a20,
        anim_tables_static_init_destroy,
        "AnimTables_StaticInitDestroy",
    ),
    (
        0x0031_50f8,
        anim_math_static_init_destroy,
        "AnimMath_StaticInitDestroy",
    ),
    (0x0031_58e8, xform_set_rotation_z, "XForm_SetRotationZ"),
    (0x0031_5a00, animmath_global_ctors, "AnimMath_GlobalCtors"),
    (
        0x0031_6878,
        animtables_global_ctors,
        "AnimTables_GlobalCtors",
    ),
];

/// `void AnimData_EmptyHook(void)` (0x3150b0): does nothing.
pub fn anim_data_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// One store of the static-table initialiser, in the generated order.
/// `Pool(f, k)` loads `f` from the game's constant pool at `gp - k`; `Imm`
/// sets `f` to a literal bit pattern; `Store(f, a)` writes `f` to `a`;
/// `Zero(a)` writes a zero word to `a`.
#[derive(Clone, Copy)]
pub(crate) enum Op {
    Pool(usize, i32),
    Imm(usize, u32),
    Store(usize, u32),
    Zero(u32),
}

/// The body that runs when `prio == 0xFFFF` and `init != 0`: 590 operations
/// that fill the animation globals (`0x004fe...`, the `D_004FExxx` tables and
/// the `cfm_sav_*` curves) with constants, most loaded from the game's pool.
/// Folded from the generated straight-line code (`0x00315a20`, 877 cycles). The registers it leaves
/// are in `INIT_FINAL_REGS`.
pub(crate) const INIT_OPS: &[Op] = &[
    Pool(16, -0x310c),
    Pool(4, -0x3108),
    Store(16, 0x004fe95c),
    Pool(1, -0x3104),
    Store(4, 0x004fe97c),
    Store(1, 0x004fe99c),
    Pool(5, -0x3100),
    Zero(0x004fe920),
    Store(5, 0x004fe93c),
    Pool(2, -0x30fc),
    Pool(15, -0x30f8),
    Pool(17, -0x30f4),
    Zero(0x004fe940),
    Store(2, 0x004fe948),
    Store(15, 0x004fe950),
    Store(17, 0x004fe954),
    Pool(6, -0x30f0),
    Pool(7, -0x30ec),
    Pool(8, -0x30e8),
    Pool(9, -0x30e4),
    Zero(0x004fe960),
    Store(5, 0x004fe964),
    Store(6, 0x004fe968),
    Store(7, 0x004fe96c),
    Store(8, 0x004fe970),
    Store(9, 0x004fe974),
    Pool(10, -0x30e0),
    Pool(11, -0x30dc),
    Pool(12, -0x30d8),
    Pool(13, -0x30d4),
    Pool(14, -0x30d0),
    Zero(0x004fe980),
    Imm(0, 0x3f800000),
    Store(10, 0x004fe984),
    Store(11, 0x004fe988),
    Store(12, 0x004fe98c),
    Store(13, 0x004fe990),
    Store(14, 0x004fe994),
    Zero(0x004fe9a0),
    Imm(3, 0x42f00000),
    Store(5, 0x004fe924),
    Store(2, 0x004fe928),
    Store(2, 0x004fe92c),
    Store(15, 0x004fe930),
    Store(2, 0x004fe934),
    Store(0, 0x004fe938),
    Store(16, 0x004fe944),
    Store(17, 0x004fe94c),
    Store(0, 0x004fe958),
    Store(3, 0x004fe978),
    Store(3, 0x004fe998),
    Pool(20, -0x30cc),
    Imm(4, 0x41200000),
    Store(20, 0x004fe9a4),
    Pool(1, -0x30c8),
    Store(4, 0x004fe9bc),
    Pool(2, -0x30c4),
    Pool(5, -0x30c0),
    Store(1, 0x004fea1c),
    Store(2, 0x004fe9a8),
    Store(5, 0x004fe9b0),
    Pool(6, -0x30bc),
    Pool(7, -0x30b8),
    Pool(8, -0x30b4),
    Imm(9, 0x42880000),
    Pool(10, -0x30b0),
    Zero(0x004fe9c0),
    Store(6, 0x004fe9c4),
    Store(7, 0x004fe9c8),
    Store(8, 0x004fe9cc),
    Store(9, 0x004fe9d0),
    Store(10, 0x004fe9d4),
    Store(0, 0x004fe9dc),
    Pool(11, -0x30ac),
    Pool(12, -0x30a8),
    Zero(0x004fe9e0),
    Store(11, 0x004fe9e8),
    Store(12, 0x004fe9f0),
    Zero(0x004fe9fc),
    Zero(0x004fea00),
    Pool(13, -0x30a4),
    Pool(14, -0x30a0),
    Pool(15, -0x309c),
    Pool(16, -0x3098),
    Pool(17, -0x3094),
    Store(13, 0x004fea04),
    Store(14, 0x004fea08),
    Store(15, 0x004fea0c),
    Store(16, 0x004fea10),
    Store(17, 0x004fea14),
    Pool(18, -0x3090),
    Pool(19, -0x308c),
    Zero(0x004fea20),
    Store(18, 0x004fea24),
    Store(19, 0x004fea28),
    Store(4, 0x004fe9ac),
    Store(4, 0x004fe9b4),
    Store(3, 0x004fe9b8),
    Store(3, 0x004fe9d8),
    Store(3, 0x004fe9f8),
    Zero(0x004fe9e4),
    Zero(0x004fe9ec),
    Zero(0x004fe9f4),
    Store(3, 0x004fea18),
    Pool(20, -0x3088),
    Pool(8, -0x3084),
    Store(20, 0x004fea2c),
    Store(8, 0x004fea3c),
    Pool(9, -0x3080),
    Pool(10, -0x307c),
    Store(9, 0x004fea50),
    Store(10, 0x004fea60),
    Pool(11, -0x3078),
    Pool(6, -0x3074),
    Store(11, 0x004fea70),
    Pool(7, -0x3070),
    Store(6, 0x004fea7c),
    Store(7, 0x004fea80),
    Pool(4, -0x306c),
    Pool(5, -0x3068),
    Store(4, 0x004fea90),
    Store(5, 0x004feaa0),
    Pool(1, -0x3064),
    Pool(2, -0x3060),
    Pool(16, -0x305c),
    Pool(17, -0x3058),
    Store(2, 0x004feab0),
    Store(1, 0x004feaac),
    Store(16, 0x004fea30),
    Store(17, 0x004fea34),
    Imm(23, 0x3f000000),
    Store(0, 0x004fea40),
    Store(23, 0x004fea4c),
    Zero(0x004fea5c),
    Zero(0x004fea6c),
    Pool(18, -0x3054),
    Pool(19, -0x3050),
    Pool(12, -0x304c),
    Pool(13, -0x3048),
    Store(18, 0x004fea74),
    Store(19, 0x004fea78),
    Store(12, 0x004fea84),
    Store(13, 0x004fea88),
    Zero(0x004fea8c),
    Pool(14, -0x3044),
    Pool(15, -0x3040),
    Pool(20, -0x303c),
    Zero(0x004fea9c),
    Store(14, 0x004feaa4),
    Store(15, 0x004feaa8),
    Store(3, 0x004fea38),
    Pool(4, -0x3038),
    Pool(2, -0x3034),
    Store(0, 0x004fea44),
    Store(0, 0x004fea48),
    Pool(1, -0x3030),
    Pool(10, -0x302c),
    Store(0, 0x004fea54),
    Store(0, 0x004fea58),
    Pool(8, -0x3028),
    Pool(7, -0x3024),
    Store(0, 0x004fea64),
    Store(0, 0x004fea68),
    Pool(5, -0x3020),
    Pool(6, -0x301c),
    Store(0, 0x004fea94),
    Store(0, 0x004fea98),
    Pool(9, -0x3018),
    Pool(25, -0x3014),
    Store(20, 0x004feab4),
    Imm(24, 0x42200000),
    Store(4, 0x004feac0),
    Pool(21, -0x3010),
    Store(2, 0x004fead0),
    Pool(20, -0x300c),
    Store(1, 0x004feadc),
    Pool(1, -0x3008),
    Pool(2, -0x3004),
    Store(10, 0x004feb0c),
    Imm(4, 0x41c80000),
    Imm(19, 0x42a00000),
    Store(0, 0x004feabc),
    Pool(22, -0x3000),
    Imm(11, 0x42860000),
    Store(7, 0x004feac8),
    Store(8, 0x004feac4),
    Zero(0x004feacc),
    Pool(7, -0x2ffc),
    Pool(13, -0x2ff8),
    Store(6, 0x004fead8),
    Store(5, 0x004fead4),
    Pool(5, -0x2ff4),
    Store(0, 0x004feae0),
    Imm(6, 0x42a20000),
    Store(0, 0x004feaec),
    Imm(14, 0x42ca0000),
    Store(0, 0x004feaf0),
    Pool(12, -0x2ff0),
    Zero(0x004feafc),
    Pool(8, -0x2fec),
    Store(0, 0x004feb00),
    Pool(27, -0x2fe8),
    Store(9, 0x004feb08),
    Imm(9, 0x42c20000),
    Store(0, 0x004feb10),
    Imm(15, 0x42a60000),
    Store(0, 0x004feb1c),
    Pool(16, -0x2fe4),
    Store(0, 0x004feb20),
    Pool(26, -0x2fe0),
    Store(0, 0x004feb2c),
    Imm(17, 0x42480000),
    Zero(0x004feb30),
    Pool(18, -0x2fdc),
    Store(0, 0x004feab8),
    Store(0, 0x004feae4),
    Store(0, 0x004feae8),
    Zero(0x004feaf4),
    Zero(0x004feaf8),
    Store(10, 0x004feb04),
    Pool(10, -0x2fd8),
    Store(0, 0x004feb14),
    Store(0, 0x004feb18),
    Store(0, 0x004feb24),
    Store(0, 0x004feb28),
    Store(25, 0x004feb34),
    Store(24, 0x004feb38),
    Store(21, 0x004feb3c),
    Store(1, 0x004feb8c),
    Store(20, 0x004feb4c),
    Zero(0x004feb50),
    Store(20, 0x004feb6c),
    Zero(0x004feb70),
    Store(2, 0x004feb74),
    Store(4, 0x004feb78),
    Store(5, 0x004feb7c),
    Store(6, 0x004feb80),
    Store(7, 0x004feb84),
    Zero(0x004feb90),
    Store(20, 0x004febac),
    Zero(0x004febb0),
    Store(3, 0x004feb48),
    Store(19, 0x004feb40),
    Store(22, 0x004feb44),
    Store(3, 0x004feb68),
    Store(25, 0x004feb54),
    Store(24, 0x004feb58),
    Store(21, 0x004feb5c),
    Store(19, 0x004feb60),
    Store(22, 0x004feb64),
    Store(3, 0x004feb88),
    Store(3, 0x004feba8),
    Store(25, 0x004feb94),
    Store(24, 0x004feb98),
    Store(21, 0x004feb9c),
    Store(19, 0x004feba0),
    Store(22, 0x004feba4),
    Pool(2, -0x2fd4),
    Store(11, 0x004febb8),
    Store(13, 0x004febbc),
    Store(14, 0x004febc0),
    Zero(0x004febb4),
    Store(12, 0x004febc4),
    Pool(1, -0x2fd0),
    Store(2, 0x004febec),
    Pool(7, -0x2fcc),
    Store(1, 0x004fec0c),
    Store(7, 0x004febcc),
    Pool(4, -0x2fc8),
    Imm(5, 0x41900000),
    Imm(6, 0x42a40000),
    Zero(0x004febd0),
    Store(4, 0x004febd4),
    Store(5, 0x004febd8),
    Store(6, 0x004febe0),
    Zero(0x004febf0),
    Store(8, 0x004febfc),
    Store(9, 0x004fec00),
    Store(10, 0x004fec04),
    Zero(0x004fec10),
    Store(7, 0x004fec2c),
    Zero(0x004fec30),
    Store(3, 0x004febc8),
    Store(2, 0x004febdc),
    Store(2, 0x004febe4),
    Store(3, 0x004febe8),
    Store(11, 0x004febf8),
    Store(3, 0x004fec08),
    Store(21, 0x004febf4),
    Store(11, 0x004fec18),
    Store(13, 0x004fec1c),
    Store(14, 0x004fec20),
    Store(12, 0x004fec24),
    Store(3, 0x004fec28),
    Zero(0x004fec14),
    Store(11, 0x004fec38),
    Store(13, 0x004fec3c),
    Store(14, 0x004fec40),
    Store(12, 0x004fec44),
    Store(3, 0x004fec48),
    Zero(0x004fec34),
    Pool(2, -0x2fc4),
    Store(7, 0x004fec4c),
    Pool(1, -0x2fc0),
    Store(2, 0x004fec6c),
    Store(1, 0x004fecac),
    Store(27, 0x004feccc),
    Imm(4, 0x41f00000),
    Pool(5, -0x2fbc),
    Pool(6, -0x2fb8),
    Zero(0x004fec50),
    Store(4, 0x004fec58),
    Store(5, 0x004fec5c),
    Store(6, 0x004fec64),
    Zero(0x004fec70),
    Store(20, 0x004fec8c),
    Pool(8, -0x2fb4),
    Imm(9, 0x41e00000),
    Pool(10, -0x2fb0),
    Zero(0x004fec90),
    Store(8, 0x004fec94),
    Store(9, 0x004fec98),
    Store(10, 0x004fec9c),
    Store(15, 0x004feca0),
    Store(16, 0x004feca4),
    Zero(0x004fecb0),
    Store(17, 0x004fecb8),
    Store(26, 0x004fecbc),
    Store(18, 0x004fecc0),
    Zero(0x004fecd0),
    Store(3, 0x004fec68),
    Zero(0x004fec54),
    Store(19, 0x004fec60),
    Store(3, 0x004fec88),
    Store(25, 0x004fec74),
    Store(24, 0x004fec78),
    Store(21, 0x004fec7c),
    Store(19, 0x004fec80),
    Store(22, 0x004fec84),
    Store(3, 0x004feca8),
    Store(3, 0x004fecc8),
    Store(26, 0x004fecb4),
    Store(27, 0x004fecc4),
    Zero(0x004fecd4),
    Store(20, 0x004fed4c),
    Store(7, 0x004fecec),
    Zero(0x004fecf0),
    Store(7, 0x004fed0c),
    Zero(0x004fed10),
    Store(7, 0x004fed2c),
    Zero(0x004fed30),
    Store(25, 0x004fed34),
    Store(24, 0x004fed38),
    Store(21, 0x004fed3c),
    Store(19, 0x004fed40),
    Store(22, 0x004fed44),
    Zero(0x004fed50),
    Store(11, 0x004fed58),
    Store(11, 0x004fecd8),
    Store(13, 0x004fecdc),
    Store(14, 0x004fece0),
    Store(12, 0x004fece4),
    Store(3, 0x004fece8),
    Store(11, 0x004fecf8),
    Store(13, 0x004fecfc),
    Store(14, 0x004fed00),
    Store(12, 0x004fed04),
    Store(3, 0x004fed08),
    Zero(0x004fecf4),
    Store(11, 0x004fed18),
    Store(13, 0x004fed1c),
    Store(14, 0x004fed20),
    Store(12, 0x004fed24),
    Store(3, 0x004fed28),
    Zero(0x004fed14),
    Store(3, 0x004fed48),
    Zero(0x004fed54),
    Store(13, 0x004fed5c),
    Store(7, 0x004fed6c),
    Pool(1, -0x2fac),
    Store(23, 0x004fedcc),
    Store(1, 0x004feddc),
    Store(14, 0x004fed60),
    Store(12, 0x004fed64),
    Store(3, 0x004fed68),
    Store(0, 0x004fed70),
    Zero(0x004fed7c),
    Store(0, 0x004fed80),
    Zero(0x004fed8c),
    Store(0, 0x004fed90),
    Store(0, 0x004fed9c),
    Store(0, 0x004feda0),
    Store(23, 0x004fedac),
    Store(0, 0x004fedb0),
    Store(23, 0x004fedbc),
    Pool(2, -0x2fa8),
    Pool(4, -0x2fa4),
    Store(0, 0x004fedc0),
    Store(0, 0x004fedd0),
    Store(2, 0x004fedd4),
    Store(4, 0x004fedd8),
    Pool(3, -0x2fa0),
    Store(0, 0x004fede0),
    Zero(0x004fed74),
    Zero(0x004fed78),
    Zero(0x004fed84),
    Zero(0x004fed88),
    Store(0, 0x004fed94),
    Store(0, 0x004fed98),
    Store(23, 0x004feda4),
    Store(0, 0x004feda8),
    Store(23, 0x004fedb4),
    Store(0, 0x004fedb8),
    Store(23, 0x004fedc4),
    Store(0, 0x004fedc8),
    Store(0, 0x004fede4),
    Pool(2, -0x2f9c),
    Store(3, 0x004fedf0),
    Pool(1, -0x2f98),
    Store(2, 0x004fee3c),
    Pool(4, -0x2f94),
    Store(1, 0x004fee5c),
    Pool(5, -0x2f90),
    Pool(6, -0x2f8c),
    Store(4, 0x004fede8),
    Zero(0x004fedec),
    Store(5, 0x004fedf4),
    Store(6, 0x004fedf8),
    Zero(0x004fedfc),
    Store(0, 0x004fee00),
    Store(0, 0x004fee0c),
    Pool(7, -0x2f88),
    Store(0, 0x004fee10),
    Store(7, 0x004fee18),
    Zero(0x004fee1c),
    Pool(8, -0x2f84),
    Pool(9, -0x2f80),
    Store(0, 0x004fee20),
    Store(8, 0x004fee24),
    Store(9, 0x004fee28),
    Zero(0x004fee2c),
    Pool(10, -0x2f7c),
    Pool(11, -0x2f78),
    Store(0, 0x004fee30),
    Store(10, 0x004fee34),
    Store(11, 0x004fee38),
    Pool(12, -0x2f74),
    Store(0, 0x004fee40),
    Store(12, 0x004fee48),
    Store(0, 0x004fee4c),
    Pool(13, -0x2f70),
    Store(0, 0x004fee50),
    Store(13, 0x004fee54),
    Store(0, 0x004fee60),
    Store(0, 0x004fee04),
    Store(0, 0x004fee08),
    Pool(14, -0x2f6c),
    Store(0, 0x004fee14),
    Store(14, 0x004fee44),
    Store(0, 0x004fee58),
    Store(0, 0x004fee64),
    Store(0, 0x004fee68),
    Pool(2, -0x2f68),
    Store(0, 0x004fee6c),
    Pool(1, -0x2f64),
    Store(2, 0x004feed0),
    Store(1, 0x004feeec),
    Store(0, 0x004fee70),
    Store(0, 0x004fee7c),
    Pool(3, -0x2f60),
    Pool(4, -0x2f5c),
    Store(0, 0x004fee80),
    Store(3, 0x004fee84),
    Store(4, 0x004fee88),
    Zero(0x004fee8c),
    Store(0, 0x004fee90),
    Store(0, 0x004fee9c),
    Pool(5, -0x2f58),
    Store(0, 0x004feea0),
    Store(5, 0x004feea4),
    Zero(0x004feeac),
    Store(0, 0x004feeb0),
    Store(0, 0x004feebc),
    Store(0, 0x004feec0),
    Zero(0x004feecc),
    Zero(0x004feedc),
    Pool(6, -0x2f54),
    Pool(7, -0x2f50),
    Store(0, 0x004feee0),
    Store(6, 0x004feee4),
    Store(7, 0x004feee8),
    Store(0, 0x004feef0),
    Store(0, 0x004fee74),
    Store(0, 0x004fee78),
    Store(0, 0x004fee94),
    Store(0, 0x004fee98),
    Store(0, 0x004feea8),
    Store(0, 0x004feeb4),
    Store(0, 0x004feeb8),
    Store(0, 0x004feec4),
    Zero(0x004feec8),
    Pool(8, -0x2f4c),
    Zero(0x004feed4),
    Zero(0x004feed8),
    Pool(2, -0x2f48),
    Store(8, 0x004feef4),
    Pool(1, -0x2f44),
    Store(2, 0x004feefc),
    Store(1, 0x004fef1c),
    Store(14, 0x004fef3c),
    Pool(3, -0x2f40),
    Store(0, 0x004fef00),
    Store(3, 0x004fef08),
    Zero(0x004fef0c),
    Pool(4, -0x2f3c),
    Store(0, 0x004fef10),
    Store(4, 0x004fef14),
    Store(0, 0x004fef20),
    Store(0, 0x004fef2c),
    Store(0, 0x004fef30),
    Store(0, 0x004fef40),
    Zero(0x004fef4c),
    Store(0, 0x004fef50),
    Zero(0x004fef5c),
    Store(0, 0x004fef60),
    Zero(0x004fef6c),
    Pool(5, -0x2f38),
    Store(0, 0x004fef70),
    Store(5, 0x004fef74),
    Store(0, 0x004feef8),
    Store(0, 0x004fef04),
    Store(0, 0x004fef18),
    Store(0, 0x004fef24),
    Store(0, 0x004fef28),
    Store(14, 0x004fef34),
    Store(0, 0x004fef38),
    Store(0, 0x004fef44),
    Zero(0x004fef48),
    Store(0, 0x004fef54),
    Zero(0x004fef58),
    Store(0, 0x004fef64),
    Zero(0x004fef68),
    Pool(15, -0x2f34),
    Store(0, 0x004fef78),
    Pool(3, -0x2f30),
    Store(15, 0x004fef7c),
    Pool(2, -0x2f2c),
    Store(3, 0x004fef8c),
    Pool(1, -0x2f28),
    Store(2, 0x004fefcc),
    Store(1, 0x004fefdc),
    Pool(4, -0x2f24),
    Store(0, 0x004fef80),
    Store(4, 0x004fef88),
    Pool(5, -0x2f20),
    Store(0, 0x004fef90),
    Store(5, 0x004fef98),
    Zero(0x004fef9c),
    Pool(6, -0x2f1c),
    Pool(7, -0x2f18),
    Store(0, 0x004fefa0),
    Store(6, 0x004fefa4),
    Store(7, 0x004fefa8),
    Zero(0x004fefac),
    Pool(8, -0x2f14),
    Pool(9, -0x2f10),
    Store(0, 0x004fefb0),
    Store(8, 0x004fefb4),
    Store(9, 0x004fefb8),
    Zero(0x004fefbc),
    Pool(10, -0x2f0c),
    Pool(11, -0x2f08),
    Store(0, 0x004fefc0),
    Store(10, 0x004fefc4),
    Store(11, 0x004fefc8),
    Pool(12, -0x2f04),
    Pool(13, -0x2f00),
    Store(0, 0x004fefd0),
    Store(12, 0x004fefd4),
    Store(13, 0x004fefd8),
    Store(0, 0x004fefe0),
    Zero(0x004fefec),
    Store(0, 0x004feff0),
    Store(0, 0x004feff4),
    Zero(0x004feffc),
    Store(0, 0x004fef84),
    Store(0, 0x004fef94),
    Zero(0x004fefe4),
    Store(0, 0x004fefe8),
    Zero(0x004feff8),
];

/// Integer registers the body leaves behind (`reg, value`), so a caller sees
/// the same state the generated code leaves.
pub(crate) const INIT_FINAL_REGS: &[(usize, u64)] = &[
    (V0, 0x4fefa0),
    (T4, 0x4feff0),
    (V1, 0x4feb50),
    (T5, 0x4fef70),
    (A0, 0x500000),
    (T6, 0x4febb0),
    (A1, 0x4fefb0),
    (T3, 0x500000),
    (T7, 0x4fef80),
    (S3, 0x500000),
    (T1, 0x500000),
    (T0, 0x500000),
    (T8, 0x4fef90),
    (A3, 0x500000),
    (T9, 0x4feaa0),
    (T2, 0x4fefe0),
    (S1, 0x4fefd0),
    (S0, 0x4fefc0),
    (S5, 0x500000),
    (FP, 0x500000),
    (S4, 0x4fecf0),
    (A2, 0x4fedf0),
    (AT, 0x41e00000),
    (S2, 0x4fec70),
    (S7, 0x4fed10),
    (S6, 0x4fecd0),
];

/// Cycles of the table body (generated: `c.cycles += 877`).
pub(crate) const INIT_CYCLES: u64 = 877;

pub(crate) fn run_init_ops(c: &mut Cpu) {
    c.cycles += INIT_CYCLES;
    for op in INIT_OPS {
        match *op {
            Pool(f, k) => {
                let a = (c.g[GP][0] as u32).wrapping_add(k as u32);
                c.lwc1(f, a);
            }
            Imm(f, bits) => c.f[f] = bits,
            Store(f, a) => c.swc1(f, a),
            Zero(a) => c.sw(a, 0),
        }
    }
    for &(reg, v) in INIT_FINAL_REGS {
        c.g[reg][0] = v;
    }
}

/// `void AnimTables_StaticInitDestroy(int init, int prio)` (0x315a20): with
/// `prio == 0xFFFF` and `init` set, writes the static animation tables (the
/// balance-rate, cruise and turn curves) into their globals. The callee-saved
/// registers and f20-f27 are saved in the 0xd0-byte frame and restored on
/// every path.
pub fn anim_tables_static_init_destroy(c: &mut Cpu) {
    c.cycles += 20;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
    let sp = c.g[SP][0] as u32;
    c.g[V0][0] = 0xffff;
    c.sq(16, sp.wrapping_add(0x80));
    c.sq(17, sp.wrapping_add(0x70));
    c.sq(18, sp.wrapping_add(0x60));
    c.sq(19, sp.wrapping_add(0x50));
    c.sq(20, sp.wrapping_add(0x40));
    c.sq(21, sp.wrapping_add(0x30));
    c.sq(22, sp.wrapping_add(0x20));
    c.sq(23, sp.wrapping_add(0x10));
    c.sq(30, sp);
    c.swc1(27, sp.wrapping_add(0xc8));
    c.swc1(26, sp.wrapping_add(0xc0));
    c.swc1(25, sp.wrapping_add(0xb8));
    c.swc1(24, sp.wrapping_add(0xb0));
    c.swc1(23, sp.wrapping_add(0xa8));
    c.swc1(22, sp.wrapping_add(0xa0));
    c.swc1(21, sp.wrapping_add(0x98));
    c.swc1(20, sp.wrapping_add(0x90));

    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        let init = c.g[A0][0] != 0;
        c.g[T4][0] = 0x500000;
        if init {
            run_init_ops(c);
        }
    }

    c.cycles += 19;
    c.lq(16, sp.wrapping_add(0x80));
    c.lq(17, sp.wrapping_add(0x70));
    c.lq(18, sp.wrapping_add(0x60));
    c.lq(19, sp.wrapping_add(0x50));
    c.lq(20, sp.wrapping_add(0x40));
    c.lq(21, sp.wrapping_add(0x30));
    c.lq(22, sp.wrapping_add(0x20));
    c.lq(23, sp.wrapping_add(0x10));
    c.lq(30, sp);
    c.lwc1(27, sp.wrapping_add(0xc8));
    c.lwc1(26, sp.wrapping_add(0xc0));
    c.lwc1(25, sp.wrapping_add(0xb8));
    c.lwc1(24, sp.wrapping_add(0xb0));
    c.lwc1(23, sp.wrapping_add(0xa8));
    c.lwc1(22, sp.wrapping_add(0xa0));
    c.lwc1(21, sp.wrapping_add(0x98));
    c.lwc1(20, sp.wrapping_add(0x90));
    c.g[SP][0] = sx(sp.wrapping_add(0xd0));
    ret(c);
}

/// Where a word of the static table lands: an absolute guest address, or an
/// offset from `gp`.
#[derive(Clone, Copy)]
pub(crate) enum InitDst {
    Abs(u32),
    Gp(i32),
}

/// What the word holds: a literal word read from `gp + k` (the generated
/// `lwc1`/`swc1` pair), a constant word, or a constant byte.
#[derive(Clone, Copy)]
pub(crate) enum InitSrc {
    Lit(i32),
    Word(u32),
    Byte(u8),
}

/// Cycles the static-table fill charges (the generated body's one `+= 471`).
pub(crate) const ANIM_INIT_CYCLES: u64 = 471;

/// The table fill, in the generated store order (226 stores: 224 table words
/// at `0x4fe420..`, and the invalid-anim byte and word at `gp + 0x2860`).
pub(crate) const ANIM_INIT_STORES: &[(InitDst, InitSrc)] = &[
    (InitDst::Gp(10336), InitSrc::Byte(0xff)),
    (InitDst::Abs(0x4fe430), InitSrc::Lit(-12832)),
    (InitDst::Abs(0x4fe440), InitSrc::Lit(-12828)),
    (InitDst::Abs(0x4fe450), InitSrc::Lit(-12824)),
    (InitDst::Abs(0x4fe45c), InitSrc::Lit(-12820)),
    (InitDst::Abs(0x4fe460), InitSrc::Lit(-12816)),
    (InitDst::Abs(0x4fe470), InitSrc::Lit(-12812)),
    (InitDst::Abs(0x4fe480), InitSrc::Lit(-12808)),
    (InitDst::Abs(0x4fe48c), InitSrc::Lit(-12804)),
    (InitDst::Abs(0x4fe490), InitSrc::Lit(-12800)),
    (InitDst::Gp(10336), InitSrc::Word(0xffffffff)),
    (InitDst::Abs(0x4fe420), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe42c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe43c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe44c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe454), InitSrc::Lit(-12796)),
    (InitDst::Abs(0x4fe458), InitSrc::Lit(-12792)),
    (InitDst::Abs(0x4fe464), InitSrc::Lit(-12788)),
    (InitDst::Abs(0x4fe468), InitSrc::Lit(-12784)),
    (InitDst::Abs(0x4fe46c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe47c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe484), InitSrc::Lit(-12780)),
    (InitDst::Abs(0x4fe488), InitSrc::Lit(-12776)),
    (InitDst::Abs(0x4fe494), InitSrc::Lit(-12772)),
    (InitDst::Abs(0x4fe424), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe428), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe434), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe438), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe444), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe448), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe474), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe478), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe498), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4a0), InitSrc::Lit(-12768)),
    (InitDst::Abs(0x4fe4b0), InitSrc::Lit(-12764)),
    (InitDst::Abs(0x4fe4bc), InitSrc::Lit(-12760)),
    (InitDst::Abs(0x4fe4ec), InitSrc::Lit(-12756)),
    (InitDst::Abs(0x4fe49c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4a4), InitSrc::Lit(-12752)),
    (InitDst::Abs(0x4fe4a8), InitSrc::Lit(-12748)),
    (InitDst::Abs(0x4fe4ac), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4b4), InitSrc::Lit(-12744)),
    (InitDst::Abs(0x4fe4b8), InitSrc::Lit(-12740)),
    (InitDst::Abs(0x4fe4c0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4cc), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4d0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4dc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4e0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4e8), InitSrc::Lit(-12736)),
    (InitDst::Abs(0x4fe4f0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4fc), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe500), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe50c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe510), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe51c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4c4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4c8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4d4), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4d8), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4e4), InitSrc::Lit(-12756)),
    (InitDst::Abs(0x4fe4f4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4f8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe504), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe508), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe514), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe518), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe520), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe56c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe57c), InitSrc::Lit(-12732)),
    (InitDst::Abs(0x4fe590), InitSrc::Lit(-12728)),
    (InitDst::Abs(0x4fe52c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe530), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe53c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe540), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe54c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe550), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe55c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe560), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe570), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe574), InitSrc::Lit(-12724)),
    (InitDst::Abs(0x4fe578), InitSrc::Lit(-12720)),
    (InitDst::Abs(0x4fe580), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe588), InitSrc::Lit(-12716)),
    (InitDst::Abs(0x4fe58c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe594), InitSrc::Lit(-12712)),
    (InitDst::Abs(0x4fe598), InitSrc::Lit(-12708)),
    (InitDst::Abs(0x4fe59c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5a0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe524), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe528), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe534), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe538), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe544), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe548), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe554), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe558), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe564), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe568), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe584), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5a4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5a8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5dc), InitSrc::Lit(-12704)),
    (InitDst::Abs(0x4fe5fc), InitSrc::Lit(-12700)),
    (InitDst::Abs(0x4fe5ac), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5b0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5b8), InitSrc::Lit(-12692)),
    (InitDst::Abs(0x4fe5bc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5c0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5c4), InitSrc::Lit(-12688)),
    (InitDst::Abs(0x4fe5c8), InitSrc::Lit(-12684)),
    (InitDst::Abs(0x4fe5cc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5d0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5d4), InitSrc::Lit(-12680)),
    (InitDst::Abs(0x4fe5d8), InitSrc::Lit(-12696)),
    (InitDst::Abs(0x4fe5e0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5e8), InitSrc::Lit(-12676)),
    (InitDst::Abs(0x4fe5ec), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5f0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5f4), InitSrc::Lit(-12672)),
    (InitDst::Abs(0x4fe600), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe60c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe610), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe61c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe620), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe624), InitSrc::Lit(-12668)),
    (InitDst::Abs(0x4fe628), InitSrc::Lit(-12664)),
    (InitDst::Abs(0x4fe62c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5b4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5e4), InitSrc::Lit(-12660)),
    (InitDst::Abs(0x4fe5f8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe604), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe608), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe614), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe618), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe630), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe670), InitSrc::Lit(-12656)),
    (InitDst::Abs(0x4fe68c), InitSrc::Lit(-12652)),
    (InitDst::Abs(0x4fe69c), InitSrc::Lit(-12648)),
    (InitDst::Abs(0x4fe63c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe640), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe644), InitSrc::Lit(-12644)),
    (InitDst::Abs(0x4fe64c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe650), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe65c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe660), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe66c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe67c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe680), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe684), InitSrc::Lit(-12640)),
    (InitDst::Abs(0x4fe688), InitSrc::Lit(-12636)),
    (InitDst::Abs(0x4fe690), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe694), InitSrc::Lit(-12632)),
    (InitDst::Abs(0x4fe6a0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6a8), InitSrc::Lit(-12628)),
    (InitDst::Abs(0x4fe6ac), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6b0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6b4), InitSrc::Lit(-12624)),
    (InitDst::Abs(0x4fe634), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe638), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe648), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe654), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe658), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe664), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe668), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe674), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe678), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe698), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6a4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6b8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6bc), InitSrc::Lit(-12620)),
    (InitDst::Abs(0x4fe6dc), InitSrc::Lit(-12660)),
    (InitDst::Abs(0x4fe71c), InitSrc::Lit(-12616)),
    (InitDst::Abs(0x4fe72c), InitSrc::Lit(-12612)),
    (InitDst::Abs(0x4fe6c0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6cc), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6d0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6e0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6ec), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6f0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6fc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe700), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe70c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe710), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe714), InitSrc::Lit(-12608)),
    (InitDst::Abs(0x4fe720), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe728), InitSrc::Lit(-12604)),
    (InitDst::Abs(0x4fe730), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe738), InitSrc::Lit(-12600)),
    (InitDst::Abs(0x4fe73c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6c4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6c8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6d4), InitSrc::Lit(-12660)),
    (InitDst::Abs(0x4fe6d8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6e4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6e8), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6f4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6f8), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe704), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe708), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe718), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe724), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe734), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe740), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe76c), InitSrc::Lit(-12596)),
    (InitDst::Abs(0x4fe77c), InitSrc::Lit(-12592)),
    (InitDst::Abs(0x4fe744), InitSrc::Lit(-12588)),
    (InitDst::Abs(0x4fe748), InitSrc::Lit(-12584)),
    (InitDst::Abs(0x4fe74c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe750), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe754), InitSrc::Lit(-12580)),
    (InitDst::Abs(0x4fe758), InitSrc::Lit(-12576)),
    (InitDst::Abs(0x4fe75c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe760), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe764), InitSrc::Lit(-12572)),
    (InitDst::Abs(0x4fe768), InitSrc::Lit(-12568)),
    (InitDst::Abs(0x4fe770), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe774), InitSrc::Lit(-12564)),
    (InitDst::Abs(0x4fe778), InitSrc::Lit(-12560)),
    (InitDst::Abs(0x4fe780), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe78c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe790), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe794), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe79c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe784), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe788), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe798), InitSrc::Word(0x0)),
];

/// GPRs the fill leaves set (the generated body's final values; the
/// callee-saved `s0..s7` and `fp` are restored by the epilogue).
pub(crate) const ANIM_INIT_GPRS: &[(usize, u64)] = &[
    (V0, 0x500000),
    (V1, 0x500000),
    (A0, 0x500000),
    (A1, 0x4fe760),
    (A2, 0x4fe770),
    (A3, 0x500000),
    (T0, 0x4fe750),
    (T1, 0x4fe5f0),
    (T2, 0x500000),
    (T3, 0x500000),
    (T4, 0x4fe640),
    (T5, 0x500000),
    (T6, 0x4fe720),
    (T7, 0x4fe710),
    (T8, 0x500000),
    (T9, 0x500000),
    (AT, 0x3f000000),
];

/// FPRs the fill leaves set: the last value moved into each (`mtc1` of the
/// constants 1.0 / 0.5, or the literal word of the last `lwc1`).
pub(crate) const ANIM_INIT_FPRS: &[(usize, InitSrc)] = &[
    (0, InitSrc::Word(0x3f800000)),
    (1, InitSrc::Lit(-12592)),
    (2, InitSrc::Lit(-12596)),
    (3, InitSrc::Lit(-12588)),
    (4, InitSrc::Lit(-12584)),
    (5, InitSrc::Lit(-12580)),
    (6, InitSrc::Lit(-12576)),
    (7, InitSrc::Lit(-12572)),
    (8, InitSrc::Lit(-12568)),
    (9, InitSrc::Lit(-12564)),
    (10, InitSrc::Lit(-12560)),
    (11, InitSrc::Lit(-12664)),
    (12, InitSrc::Lit(-12660)),
    (13, InitSrc::Lit(-12784)),
    (14, InitSrc::Lit(-12780)),
    (15, InitSrc::Lit(-12776)),
    (16, InitSrc::Lit(-12796)),
    (17, InitSrc::Lit(-12792)),
];

/// `void AnimMath_StaticInitDestroy(int prio, int init)` (0x3150f8, the
/// animation map's static setup). With `prio == 0xFFFF` and `init != 0` it sets
/// the invalid-anim byte and word at `gp + 0x2860` to -1 and fills the static
/// colour and ramp table at `0x4fe420..0x4fe7a0` (224 words), from literal
/// words in gp-relative data. Otherwise it does nothing. The
/// registers and cycles of the generated body are reproduced, including the
/// scratch values the fill leaves (`v0` is `0xffff` when `prio` is not
/// `0xFFFF`, `-1` when `init` is 0). The callee-saved registers are saved to
/// the frame and not changed.
pub fn anim_math_static_init_destroy(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x003150f8);
    c.cycles += 12;
    let frame_sp = (c.g[SP][0] as u32).wrapping_sub(0x90);
    c.g[SP][0] = sx(frame_sp);
    c.g[V0][0] = 0xffff;
    c.sq(16, frame_sp.wrapping_add(0x80));
    c.sq(17, frame_sp.wrapping_add(0x70));
    c.sq(18, frame_sp.wrapping_add(0x60));
    c.sq(19, frame_sp.wrapping_add(0x50));
    c.sq(20, frame_sp.wrapping_add(0x40));
    c.sq(21, frame_sp.wrapping_add(0x30));
    c.sq(22, frame_sp.wrapping_add(0x20));
    c.sq(23, frame_sp.wrapping_add(0x10));
    c.sq(30, frame_sp);
    if c.g[A1][0] == 0xffff {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if c.g[A0][0] != 0 {
            anim_init_fill_table(c);
        }
    }
    // Epilogue: the generated `lq` reloads of s0..s7 and fp read back the
    // values saved above, which the body never changed, so they are skipped.
    c.cycles += 11;
    c.g[SP][0] = sx(frame_sp.wrapping_add(0x90));
    ret(c);
}

/// The table fill of `anim_math_static_init_destroy` (the `init != 0` path).
pub(crate) fn anim_init_fill_table(c: &mut Cpu) {
    c.cycles += ANIM_INIT_CYCLES;
    let gp = c.g[GP][0] as u32;
    for &(dst, src) in ANIM_INIT_STORES {
        let at = match dst {
            InitDst::Abs(a) => a,
            InitDst::Gp(k) => gp.wrapping_add(k as u32),
        };
        match src {
            InitSrc::Lit(k) => {
                let w = Addr(gp.wrapping_add(k as u32)).read_u32(c);
                Addr(at).sw(c, w as u64);
            }
            InitSrc::Word(w) => Addr(at).sw(c, w as u64),
            InitSrc::Byte(b) => Addr(at).sb(c, b as u64),
        }
    }
    for &(r, v) in ANIM_INIT_GPRS {
        c.g[r][0] = v;
    }
    for &(n, src) in ANIM_INIT_FPRS {
        c.f[n] = match src {
            InitSrc::Lit(k) => Addr(gp.wrapping_add(k as u32)).read_u32(c),
            InitSrc::Word(w) => w,
            InitSrc::Byte(b) => b as u32,
        };
    }
}

/// `void AnimMath_GlobalCtors(void)` (0x315a00).
pub fn animmath_global_ctors(c: &mut Cpu) {
    let init = Callee {
        addr: 0x0031_50f8,
        f: ee::s10::f_003150f8,
    };
    static_ctor_stub(c, init, 0x0031_5a14);
}

/// `void AnimTables_GlobalCtors(void)` (0x316878).
pub fn animtables_global_ctors(c: &mut Cpu) {
    let init = Callee {
        addr: 0x0031_5a20,
        f: ee::s10::f_00315a20,
    };
    static_ctor_stub(c, init, 0x0031_688c);
}

/// `f[n]` as an FPU value.
pub(crate) fn xform_f(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `f[n] = v`.
pub(crate) fn xform_set_f(c: &mut Cpu, n: usize, v: FpuF32) {
    c.f[n] = v.0;
}

/// `void cRiderAnimBase_setHeadingOffset(XForm* self, float angle)` (0x3158e8):
/// the offset's orientation is a rotation of `-angle` about the Z axis
/// (`sin`/`cos` of half the angle), and its position the origin.
pub fn xform_set_rotation_z(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x60);
        let neg = -xform_f(c, 12);
        xform_set_f(c, 0, neg);
        frame.save(c, 16, 0x50);
        c.g[V0][0] = 0x500000;
        frame.save(c, 17, 0x40);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        frame.save(c, 18, 0x30);
        c.g[S1][0] = 0x500000;
        frame.save_ra(c, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        let origin = c.g[V0][0] as u32;
        c.lq(3, origin);
        c.g[A1][0] = sx(frame.at(c, 0x14));
        let half = xform_f(c, 0) * xform_f(c, 12);
        xform_set_f(c, 12, half);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.g[RA][0] = sx(0x0031_5934);
        let position = (c.g[S2][0] as u32).wrapping_add(0x30);
        c.sq(3, position);
        Callee {
            addr: 0x0031_be50,
            f: ee::s10::f_0031be50,
        }
        .call(c, 0x0031_5934)?;
        let axis = c.g[S0][0] as u32;
        c.lwc1(1, axis.wrapping_add(8));
        c.lwc1(0, frame.at(c, 0x10));
        c.lwc1(2, (c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.lwc1(4, axis.wrapping_add(4));
        let z = xform_f(c, 0) * xform_f(c, 1);
        xform_set_f(c, 1, z);
        let x = xform_f(c, 0) * xform_f(c, 2);
        xform_set_f(c, 2, x);
        c.lwc1(3, frame.at(c, 0x14));
        let y = xform_f(c, 0) * xform_f(c, 4);
        xform_set_f(c, 0, y);
        frame.restore(c, 16, 0x50);
        c.swc1(3, frame.at(c, 0xc));
        c.swc1(1, frame.at(c, 0x8));
        c.swc1(2, frame.at(c, 0x0));
        c.swc1(0, frame.at(c, 0x4));
        frame.restore(c, 17, 0x40);
        c.lq(2, frame.at(c, 0x0));
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        let quat = (c.g[S2][0] as u32).wrapping_add(0x40);
        c.sq(2, quat);
        frame.restore(c, 18, 0x30);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// Leaf setters and getters: random registers, argument pointing at the
    /// data area, struct contents random.
    pub(crate) fn oc438_leaf(gen: GuestFn, hand: GuestFn, entry: u32) {
        check(gen, hand, 200, &|c, seed| {
            let mut r = Rng(0x1eaf ^ entry as u64 ^ (seed as u64) << 32);
            scramble(c, &mut r);
            c.pc = entry;
            let p = DATA + (r.next_u32() & 0xf_ff00 & !3);
            c.g[A0][0] = p as u64;
            for k in 0..0x40 {
                c.bus.write32(p + 4 * k, r.next_u32());
            }
            // Flag bytes and ids: zero, small, negative, any.
            let word = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.bus.write32(p + 8, word);
            c.bus.write32(p, word ^ 0xe5);
        });
    }

    #[test]
    fn anim_data_empty_hook_matches() {
        oc438_leaf(
            ee::s10::f_003150b0__generated,
            anim_data_empty_hook,
            0x0031_50b0,
        );
    }

    use crate::difftest::GAME_GP;

    /// Covers the three paths: `prio != 0xFFFF` (early exit), `init == 0`
    /// (after the register setup), and the full table write.
    #[test]
    fn anim_tables_static_init_destroy_matches() {
        check(
            ee::s10::f_00315a20__generated,
            anim_tables_static_init_destroy,
            200,
            &|c, seed| {
                let mut r = Rng(0x0031_5a20 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0031_5a20;
                c.g[A0][0] = if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A1][0] = if seed % 3 == 0 {
                    r.next_u32() as u64
                } else {
                    0xffff
                };
            },
        );
    }

    use crate::difftest::fill;

    /// Both paths of the prologue, the table fill and the `init == 0` return:
    /// seed % 4 == 0 gives `prio != 0xFFFF`, seed % 6 == 0 gives `init == 0`.
    #[test]
    fn anim_math_static_init_destroy_matches() {
        check(
            ssx3_ee::s10::f_003150f8__generated,
            anim_math_static_init_destroy,
            200,
            &|c, seed| {
                let mut r = Rng(0x3150_f8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003150f8;
                c.g[GP][0] = sx(GAME_GP);
                c.g[SP][0] = sx(crate::difftest::SP);
                c.g[RA][0] = sx(crate::difftest::RA);
                let prio = if seed % 4 == 0 { r.next_u32() } else { 0xffff };
                let init = if seed % 6 == 0 { 0 } else { r.next_u32() | 1 };
                c.g[A1][0] = prio as u64;
                c.g[A0][0] = init as u64;
                // The literal words the fill reads, gp-relative.
                let mut off = -0x3400i32;
                while off < 0x2900 {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.next_u32());
                    off += 4;
                }
            },
        );
    }

    #[test]
    fn animmath_global_ctors_matches() {
        check(
            ee::s10::f_00315a00__generated,
            animmath_global_ctors,
            10,
            &|c, seed| {
                let mut r = Rng(0x0031_5a00 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_5a00;
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn animtables_global_ctors_matches() {
        check(
            ee::s10::f_00316878__generated,
            animtables_global_ctors,
            10,
            &|c, seed| {
                let mut r = Rng(0x0031_6878 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_6878;
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn xform_set_rotation_z_matches() {
        check(
            ee::s10::f_003158e8__generated,
            xform_set_rotation_z,
            200,
            &|c, seed| {
                let mut r = Rng(0x0031_58e8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_58e8;
                // The origin (t4vect_Origin) and the Z axis (t4vect_ZAxis).
                fill(c, &mut r, 0x004f_f130, 4);
                c.bus.write32(0x004f_f160, 0);
                c.bus.write32(0x004f_f164, 0);
                c.bus.write32(0x004f_f168, 0x3f80_0000);
                c.bus.write32(0x004f_f16c, 0);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(6.0);
            },
        );
    }
}
