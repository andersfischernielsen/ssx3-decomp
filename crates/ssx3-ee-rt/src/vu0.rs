//! VU0 as the EE sees it in macro mode (COP2).
//!
//! Recompiled EE code lowers every COP2 macro instruction at build time to
//! one call on [`Vu0`], e.g. `c.vu0().madd(0b1110, 1, 2, Src::Bc(3, W))`.
//! Each call is a complete macro op: the implementation (`ssx3-ps2`) brackets
//! the VU's per-op semantics with the macro-mode pipeline begin/end, so the
//! VU's register file, pipeline model and arithmetic all stay in the hardware
//! crate and can change without recompiling generated code.
//!
//! The operand types ([`Src`], [`Efu`], the field indices) and the op list
//! ([`for_each_vu0_op!`](crate::for_each_vu0_op)) are shared with the
//! hardware crate's VU, which re-exports them.

/// Field indices for broadcast operands.
pub const X: u8 = 0;
pub const Y: u8 = 1;
pub const Z: u8 = 2;
pub const W: u8 = 3;

/// Second operand of an FMAC op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    /// ft, lane by lane.
    Vec(u8),
    /// ft.field broadcast (field 0..3 = x..w).
    Bc(u8, u8),
    /// The Q register.
    Q,
    /// The I register.
    I,
}

/// EFU (elementary function unit) operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efu {
    Esadd,
    Ersadd,
    Eleng,
    Erleng,
    Eatanxy,
    Eatanxz,
    Esum,
    Esqrt,
    Ersqrt,
    Ercpr,
    Esin,
    Eatan,
    Eexp,
}

impl Efu {
    pub fn name(self) -> &'static str {
        match self {
            Efu::Esadd => "esadd",
            Efu::Ersadd => "ersadd",
            Efu::Eleng => "eleng",
            Efu::Erleng => "erleng",
            Efu::Eatanxy => "eatanxy",
            Efu::Eatanxz => "eatanxz",
            Efu::Esum => "esum",
            Efu::Esqrt => "esqrt",
            Efu::Ersqrt => "ersqrt",
            Efu::Ercpr => "ercpr",
            Efu::Esin => "esin",
            Efu::Eatan => "eatan",
            Efu::Eexp => "eexp",
        }
    }
}

/// Invoke `$m! { upper|lower name(arg: Type, ...); ... }` with every VU op
/// that a COP2 macro instruction can lower to. `upper` ops end with the
/// upper-op commit, `lower` ops with the lower-op end (see `ssx3-ps2`'s
/// `Vu::macro_end_upper` / `macro_end_lower`). Names and signatures match the
/// VU's own per-op methods, which the generator's source text (`vu.add(...)`)
/// relies on.
#[macro_export]
macro_rules! for_each_vu0_op {
    ($m:ident) => {
        $m! {
            upper add(dest: u8, fd: u8, fs: u8, src: Src);
            upper adda(dest: u8, fs: u8, src: Src);
            upper sub(dest: u8, fd: u8, fs: u8, src: Src);
            upper suba(dest: u8, fs: u8, src: Src);
            upper mul(dest: u8, fd: u8, fs: u8, src: Src);
            upper mula(dest: u8, fs: u8, src: Src);
            upper madd(dest: u8, fd: u8, fs: u8, src: Src);
            upper madda(dest: u8, fs: u8, src: Src);
            upper msub(dest: u8, fd: u8, fs: u8, src: Src);
            upper msuba(dest: u8, fs: u8, src: Src);
            upper max(dest: u8, fd: u8, fs: u8, src: Src);
            upper mini(dest: u8, fd: u8, fs: u8, src: Src);
            upper opmula(fs: u8, ft: u8);
            upper opmsub(fd: u8, fs: u8, ft: u8);
            upper itof(frac: u8, dest: u8, ft: u8, fs: u8);
            upper ftoi(frac: u8, dest: u8, ft: u8, fs: u8);
            upper abs(dest: u8, ft: u8, fs: u8);
            upper clipw(fs: u8, ft: u8);
            lower lq(dest: u8, ft: u8, is: u8, imm: i16);
            lower sq(dest: u8, fs: u8, it: u8, imm: i16);
            lower ilw(dest: u8, it: u8, is: u8, imm: i16);
            lower isw(dest: u8, it: u8, is: u8, imm: i16);
            lower ilwr(dest: u8, it: u8, is: u8);
            lower iswr(dest: u8, it: u8, is: u8);
            lower lqi(dest: u8, ft: u8, is: u8);
            lower lqd(dest: u8, ft: u8, is: u8);
            lower sqi(dest: u8, fs: u8, it: u8);
            lower sqd(dest: u8, fs: u8, it: u8);
            lower iaddiu(it: u8, is: u8, imm: u16);
            lower isubiu(it: u8, is: u8, imm: u16);
            lower iadd(id: u8, is: u8, it: u8);
            lower isub(id: u8, is: u8, it: u8);
            lower iaddi(it: u8, is: u8, imm: i8);
            lower iand(id: u8, is: u8, it: u8);
            lower ior(id: u8, is: u8, it: u8);
            lower fceq(imm: u32);
            lower fcand(imm: u32);
            lower fcor(imm: u32);
            lower fcset(imm: u32);
            lower fcget(it: u8);
            lower fseq(it: u8, imm: u16);
            lower fsand(it: u8, imm: u16);
            lower fsor(it: u8, imm: u16);
            lower fsset(imm: u16);
            lower fmeq(it: u8, is: u8);
            lower fmand(it: u8, is: u8);
            lower fmor(it: u8, is: u8);
            lower vmove(dest: u8, ft: u8, fs: u8);
            lower mr32(dest: u8, ft: u8, fs: u8);
            lower mtir(it: u8, fs: u8, fsf: u8);
            lower mfir(dest: u8, ft: u8, is: u8);
            lower div(fs: u8, fsf: u8, ft: u8, ftf: u8);
            lower sqrt(ft: u8, ftf: u8);
            lower rsqrt(fs: u8, fsf: u8, ft: u8, ftf: u8);
            lower waitq();
            lower rnext(dest: u8, ft: u8);
            lower rget(dest: u8, ft: u8);
            lower rinit(fs: u8, fsf: u8);
            lower rxor(fs: u8, fsf: u8);
            lower mfp(dest: u8, ft: u8);
            lower xtop(it: u8);
            lower xitop(it: u8);
            lower efu_vec(op: Efu, fs: u8);
            lower efu_scalar(op: Efu, fs: u8, fsf: u8);
            lower waitp();
        }
    };
}

macro_rules! declare_vu0 {
    ($($kind:ident $name:ident($($a:ident: $t:ty),*);)*) => {
        /// VU0 in macro mode, implemented by the hardware model. Every op
        /// method executes one whole COP2 macro instruction.
        pub trait Vu0 {
            $(fn $name(&mut self, $($a: $t),*);)*
            /// Upper VNOP: no operation, but the macro-mode pipeline still
            /// advances like any other upper op.
            fn vnop(&mut self);
            /// Decode and execute a macro instruction at run time (only for
            /// encodings the generator could not lower).
            fn macro_op(&mut self, word: u32);
            /// VF register `r` as raw float bits (x, y, z, w).
            fn vf(&self, r: usize) -> [u32; 4];
            fn set_vf(&mut self, r: usize, v: [u32; 4]);
            /// The FMAC accumulator ACC (x, y, z, w). Not visible to EE code
            /// directly, but VU0 context savers copy it to memory, so
            /// hand-written replacements reproduce it.
            fn acc(&self) -> [u32; 4] {
                [0; 4]
            }
            fn set_acc(&mut self, _v: [u32; 4]) {}
            /// cfc2 / ctc2 control registers.
            fn read_ctrl(&mut self, reg: u32) -> u32;
            fn write_ctrl(&mut self, reg: u32, val: u32);
            /// `vcallms`: run the VU0 microprogram at byte address `addr` to
            /// completion.
            fn vcallms(&mut self, addr: u32);
        }
    };
}

for_each_vu0_op!(declare_vu0);
