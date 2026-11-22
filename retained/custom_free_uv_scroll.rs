//! Native ModifierBlock_FreeUVScroll (0x00353118), retained uninstalled.
//!
//! No automated tests, builds, game runs or replays were added or run by
//! this writer, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    modifier_block_free_uv_scroll,
    "ModifierBlock_FreeUVScroll",
)];

const ENTRY: u32 = 0x0035_3118;
const RETURN_FROM_FREE: u32 = 0x0035_313c;
const END: u32 = 0x0035_3150;
const UV_SCROLL: u32 = 4;

/// `void ModifierBlock_FreeUVScroll(ModifierBlock* block)`.
/// Free the optional UV-scroll allocation at block+4 through the native
/// allocator, then clear only that slot. An empty slot is left untouched.
/// Preserve the native return continuation if allocator work suspends.
pub fn modifier_block_free_uv_scroll(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        match c.pc {
            ENTRY => {
                c.cycles += 7;
                Frame::open(c, 0x20);
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(UV_SCROLL));
                if c.g[A0][0] != 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0031_7e50, RETURN_FROM_FREE)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(UV_SCROLL), 0);
                }
            }
            RETURN_FROM_FREE => {
                c.sw((c.g[S0][0] as u32).wrapping_add(UV_SCROLL), 0);
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
