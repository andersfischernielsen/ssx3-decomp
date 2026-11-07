//! `src/bx/bxstringctor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_ba48, path_data_read_vec3, "PathData_ReadVec3"),
    (0x0026_ba68, path_data_read_int, "PathData_ReadInt"),
    (0x0026_ba88, path_data_read_float, "PathData_ReadFloat"),
];

// ---------------------------------------------------------------------------
// Callees, under the names the generated bodies give them.

pub(crate) mod callee {

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
}

/// `void PathData_ReadVec3(Vec3 *dst, const Vec3 *src)` (0x0026ba48): copy
/// three floats unless the two are the same object.
pub fn path_data_read_vec3(c: &mut Cpu) {
    path_data_copy(c, 0xc, 0x0026_ba5c)
}

/// `void PathData_ReadInt(int *dst, const int *src)` (0x0026ba68): copy the
/// word unless the two are the same object.
pub fn path_data_read_int(c: &mut Cpu) {
    path_data_copy(c, 4, 0x0026_ba7c)
}

/// `void PathData_ReadFloat(float *dst, const float *src)` (0x0026ba88): copy
/// the float unless the two are the same object.
pub fn path_data_read_float(c: &mut Cpu) {
    path_data_copy(c, 4, 0x0026_ba9c)
}

/// Shared body of the three [`PathData_Read*`] copies.
pub(crate) fn path_data_copy(c: &mut Cpu, size: u32, return_to: u32) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let same = c.g[A0][0] == c.g[A1][0];
        frame.save_ra(c, 0);
        if !same {
            c.cycles += 2;
            c.g[A2][0] = size as u64;
            call_ptr(c, callee::EA_MEM_COPY, return_to)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) fn path_copy_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = if seed % 2 == 0 {
            DATA as u64
        } else {
            (DATA + 0x800) as u64
        };
        c.g[A2][0] = r.next_u32() as u64;
        for i in 0..4u32 {
            c.bus.write32(DATA + 0x800 + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn path_data_read_vec3_matches() {
        check(
            ee::s07::f_0026ba48__generated,
            path_data_read_vec3,
            200,
            &|c, seed| path_copy_setup(c, seed, 0x26_ba48),
        );
    }

    #[test]
    fn path_data_read_int_matches() {
        check(
            ee::s07::f_0026ba68__generated,
            path_data_read_int,
            200,
            &|c, seed| path_copy_setup(c, seed, 0x26_ba68),
        );
    }

    #[test]
    fn path_data_read_float_matches() {
        check(
            ee::s07::f_0026ba88__generated,
            path_data_read_float,
            200,
            &|c, seed| path_copy_setup(c, seed, 0x26_ba88),
        );
    }
}
