//! Native ModifierBlock_FreeTexFlip (0x00353150), retained uninstalled.
//! No automated tests, cargo, game or replays: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    modifier_block_free_tex_flip,
    "ModifierBlock_FreeTexFlip",
)];

const ENTRY: u32 = 0x0035_3150;
const RETURN_FROM_FREE: u32 = 0x0035_3174;
const END: u32 = 0x0035_3188;
const TEX_FLIP: u32 = 8;

/// `void ModifierBlock_FreeTexFlip(ModifierBlock* block)`.
/// Free the optional texture-flip allocation at block+8 through native
/// EAfree (0x317e50), then clear that slot. Other modifier slots and the
/// owning world object remain untouched. A missing allocation is a no-op.
pub fn modifier_block_free_tex_flip(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        match c.pc {
            ENTRY => {
                c.cycles += 7;
                Frame::open(c, 0x20);
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(TEX_FLIP));
                if c.g[A0][0] != 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0031_7e50, RETURN_FROM_FREE)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(TEX_FLIP), 0);
                }
            }
            RETURN_FROM_FREE => {
                c.sw((c.g[S0][0] as u32).wrapping_add(TEX_FLIP), 0);
            }
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
