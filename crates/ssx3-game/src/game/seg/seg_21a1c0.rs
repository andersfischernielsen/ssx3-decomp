//! `src/seg/seg_21a1c0.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0041_7828, sprintf, "sprintf")];

// ---------------------------------------------------------------------------
// sprintf (0x00417828)

/// `int vfprintf(FILE*, const char*, va_list)` (0x0041b658).
pub(crate) const VFPRINTF: Callee = Callee {
    addr: 0x0041_b658,
    f: ee::s15::f_0041b658,
};

/// `_impure_ptr` (the global reentrancy struct pointer) at 0x454944.
pub(crate) const IMPURE_PTR: u32 = 0x0045_4944;

/// The string FILE newlib builds on the stack: `_p` (+0), `_w` (+0x8),
/// `_flags` (+0xc), `_bf._base` (+0x10), `_bf._size` (+0x14), `_data`
/// (+0x54).
pub(crate) const FILE_P: u32 = 0x00;

pub(crate) const FILE_W: u32 = 0x08;

pub(crate) const FILE_FLAGS: u32 = 0x0c;

pub(crate) const FILE_BASE: u32 = 0x10;

pub(crate) const FILE_SIZE: u32 = 0x14;

pub(crate) const FILE_DATA: u32 = 0x54;

/// `__SWR | __SSTR`.
pub(crate) const STRING_FLAGS: u64 = 0x208;

/// The variadic register save area (a2..t3 as 64-bit, then f12/f14/f16/f18).
pub(crate) const VA_GPRS: u32 = 0xb0;

pub(crate) const VA_FPRS: u32 = 0xa0;

/// `int sprintf(char* buf, const char* fmt, ...)` (0x00417828): newlib's
/// string printf. Builds a write-only string FILE on the stack over `buf`
/// (unbounded size 0x7fffffff), spills the variadic registers into a
/// `va_list` area, formats with `vfprintf` and terminates the output with
/// a NUL. Returns the character count from `vfprintf` in v0.
pub fn sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0xe0);
        c.g[T5][0] = crate::guest::Addr(IMPURE_PTR).lw(c);
        c.g[T4][0] = c.g[A0][0];
        c.g[V1][0] = 0x7fff_ffff;
        let a = frame.at(c, VA_GPRS);
        c.sd(a, c.g[A2][0]);
        c.g[V0][0] = STRING_FLAGS;
        frame.save_ra(c, 0x60);
        c.g[A0][0] = c.g[SP][0];
        let file = sp(c);
        c.sw(file + FILE_SIZE, c.g[V1][0]);
        c.g[A2][0] = sx(file.wrapping_add(VA_GPRS));
        c.sw(file + FILE_W, c.g[V1][0]);
        for (i, r) in [A3, T0, T1, T2, T3].into_iter().enumerate() {
            let a = file.wrapping_add(VA_GPRS + 8 + 8 * i as u32);
            c.sd(a, c.g[r][0]);
        }
        for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
            c.swc1(f, file.wrapping_add(VA_FPRS + 4 * i as u32));
        }
        c.sh(file + FILE_FLAGS, c.g[V0][0]);
        c.sw(file + FILE_BASE, c.g[T4][0]);
        c.sw(file + FILE_DATA, c.g[T5][0]);
        c.sw(file + FILE_P, c.g[T4][0]);
        VFPRINTF.call(c, 0x0041_7894)?;
        // *fp._p = '\0'
        c.g[V1][0] = crate::guest::Addr(sp(c) + FILE_P).lw(c);
        frame.restore_ra(c, 0x60);
        c.sb(c.g[V1][0] as u32, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// sprintf formats through the real vfprintf on short formats: plain
    /// text, an empty format, `%d` and `%s` (integer path, `_vfiprintf_r`).
    pub(crate) fn sprintf_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x0041_7828, 0);
        elf_image(c);
        let buf = DATA;
        let fmt = DATA + 0x100;
        let s = DATA + 0x180;
        const FORMATS: [&[u8]; 5] = [b"abc", b"", b"n=%d!", b"<%s>", b"%d,%s"];
        let f = FORMATS[seed as usize % FORMATS.len()];
        for (i, b) in f.iter().enumerate() {
            c.bus.write8(fmt + i as u32, *b);
        }
        c.bus.write8(fmt + f.len() as u32, 0);
        for (i, b) in b"snow\0".iter().enumerate() {
            c.bus.write8(s + i as u32, *b);
        }
        for i in 0..0x40 {
            c.bus.write8(buf + i, 0xaa);
        }
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = sx(fmt);
        match seed % FORMATS.len() as u32 {
            3 => c.g[A2][0] = sx(s),
            4 => {
                c.g[A2][0] = sx(seed.wrapping_mul(7919));
                c.g[A3][0] = sx(s);
            }
            _ => c.g[A2][0] = sx(seed.wrapping_mul(2654435761) >> 8),
        }
        c.pc = 0x0041_7828;
    }

    #[test]
    fn sprintf_matches() {
        check(ee::s15::f_00417828__generated, sprintf, 100, &sprintf_setup);
    }
}
