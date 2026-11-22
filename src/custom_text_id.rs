//! Native UIText locale-ID assignment and virtual text invalidation.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003a_1000, ui_text_set_text_id, "UIText_SetTextId")];

const TEXT_ID: u32 = 0xb0;
const TEXT_REFRESH_SLOT: u32 = 0x28;

/// `void UIText_SetTextId(UIText* self, u32 id)` (0x3a1000).
/// Store the locale ID, then call the object's virtual text-refresh method
/// with zero. Locale lookup and cached-string management belong to that
/// method rather than this setter; even assigning the same ID calls it.
pub fn ui_text_set_text_id(c: &mut Cpu) {
    c.g[A1][0] = sx(crate::custom_tracks::menu_text_id(c.g[A1][0] as u32));
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let object = c.g[A0][0] as u32;
        c.sw(object.wrapping_add(TEXT_ID), c.g[A1][0]);
        c.g[V0][0] = c.lw(object.wrapping_add(8));
        c.g[A1][0] = 0;
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = c.lh(table.wrapping_add(TEXT_REFRESH_SLOT));
        c.g[A2][0] = c.lw(table.wrapping_add(TEXT_REFRESH_SLOT + 4));
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx(object.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x003a_1024)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
