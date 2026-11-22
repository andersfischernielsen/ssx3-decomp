//! Native recovery-only nearby-AI-path wrapper0x26af98, retained uninstalled.
//! Reference only, with no override registration or automated test.

use crate::call::{guest, Callee, Frame};
use ssx3_ee_rt::cpu::reg::T0;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0026_af98;
const RETURN: u32 = 0x0026_afa8;
const END: u32 = 0x0026_afb4;
const FIND_NEAR: Callee = Callee {
    addr: 0x0026_afb8,
    f: ssx3_ee::s07::PathData_FindNearAIPaths,
};

/// `int PathData_FindNearRecoverableAIPaths(data, point, out, max_out)`.
/// Forward A0..A3 unchanged, explicitly setting the fifth argument T0 to1.
/// The underlying selector therefore consults AIPath+0x3c (respawnable).
pub fn find_near_recoverable_ai_paths(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        match c.pc {
            ENTRY => {
                c.cycles += 7;
                Frame::open(c, frame.size);
                frame.save_ra(c, 0);
                c.g[T0][0] = 1;
                FIND_NEAR.call(c, RETURN)?;
            }
            RETURN => {}
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
