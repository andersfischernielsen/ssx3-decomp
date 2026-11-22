//! Native PS2GraphicsMan_SetTextureFilter (0x37d090), retained uninstalled.
//!
//! The generated EE body defines the register, stack and cycle contract.
//! No differential test is added or run, as requested by the user.

use crate::call::{self, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const TEX_MAN: u32 = 0x18f4;
const SET_TEX1_FILTER: u32 = 0x0036_9130;
const AFTER_SET_TEX1_FILTER: u32 = 0x0037_d0a0;

/// Forward `(self, index, mmin, mmag, l, k)` to TexMan_SetTex1Filter,
/// replacing `self` with its TexMan pointer at +0x18f4.
///
/// Arguments A1/A2/A3/T0/T1 pass through unchanged at the call. The callee
/// selects `*(TexMan + 8 + index*4)` and updates its 64-bit word at +0x40:
/// MMIN = mmin & 7 (bits 6..8), MMAG = mmag & 1 (bit 5),
/// L = l & 3 (bits 19..20), K = k & 0xfff (bits 32..43).
/// The wrapper does not synthesize a return value; callee results survive.
pub fn ps2_graphics_man_set_texture_filter(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);

        // Preserve the generated ordering: RA is set before the field load.
        c.g[RA][0] = sx(AFTER_SET_TEX1_FILTER);
        let graphics_man = Addr(c.g[A0][0] as u32);
        c.g[A0][0] = graphics_man.off(TEX_MAN).lw(c);
        c.pc = SET_TEX1_FILTER;
        ssx3_ee::s12::f_00369130(c);
        call::returned(c, AFTER_SET_TEX1_FILTER)?;

        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
