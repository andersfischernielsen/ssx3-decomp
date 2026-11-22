//! Native UIMenu selection by child index, including scroll notifications.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0039_b960, ui_menu_set_selection, "UIMenu_SetSelection")];

const CHILDREN: u32 = 0x74;
const SELECTED_INDEX: u32 = 0x95;
const VISIBLE_COUNT: u32 = 0x97;
const FIRST_VISIBLE: u32 = 0x98;
const PENDING_INDEX: u32 = 0xa4;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Dispatch the notification vslot already prepared in t0/a1/a2/a3.
/// Its delta and target must also remain visible in a0 and v0.
fn notify(c: &mut Cpu, back: u32) -> Flow {
    c.g[V1][0] = c.lw(r(c, T0).wrapping_add(8));
    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0xa0));
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xa4));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, T0).wrapping_add(r(c, A0)));
    call_ptr(c, target, back)
}

/// `void UIMenu_SetSelection(UIMenu* menu, unsigned char index)`
/// (0x39b960). The argument is an index in the menu's child list, not
/// UIObject.id. An unready menu remembers it at +0xa4. A ready menu updates
/// +0x95, scrolls the visible range when needed, and selects the child at
/// `index - firstVisible` through UIList_GetAt and UIMenu_SetCurrentItem.
pub fn ui_menu_set_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x14));
        c.g[V0][0] = sx((r(c, V0) as i32 >> 4) as u32);
        c.g[V0][0] &= 1;
        let menu_ready = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0] & 0xff;
        let mut ready = false;
        if menu_ready {
            c.cycles += 7;
            c.g[A3][0] = c.lw(r(c, S0).wrapping_add(0x5c));
            c.g[V1][0] = c.lw(r(c, A3).wrapping_add(0xd0));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x1c));
            c.g[V0][0] = sx((r(c, V0) as i32 >> 5) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.lbu(r(c, S0).wrapping_add(FIRST_VISIBLE));
                ready = true;
            }
        }
        if !ready {
            c.cycles += 2;
            c.sb(r(c, S0).wrapping_add(PENDING_INDEX), c.g[S1][0]);
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.lbu(r(c, S0).wrapping_add(VISIBLE_COUNT));
            c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let in_bottom = c.g[V0][0] != 0;
            c.sb(r(c, S0).wrapping_add(SELECTED_INDEX), c.g[S1][0]);
            if !in_bottom {
                c.cycles += 24;
                c.g[V0][0] = sx(r(c, S1).wrapping_sub(r(c, V1)));
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                c.g[A2][0] = 3;
                c.sb(r(c, S0).wrapping_add(FIRST_VISIBLE), c.g[V0][0]);
                c.g[T0][0] = c.lw(r(c, A3).wrapping_add(0xd0));
                c.g[A3][0] = c.g[V0][0] & 0xff;
                notify(c, 0x0039_b9f0)?;
                c.g[A0][0] = c.lw(r(c, S0).wrapping_add(0x5c));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.lbu(r(c, S0).wrapping_add(FIRST_VISIBLE));
                c.g[A2][0] = 4;
                c.g[T0][0] = c.lw(r(c, A0).wrapping_add(0xd0));
                notify(c, 0x0039_ba18)?;
                c.g[A1][0] = c.lbu(r(c, S0).wrapping_add(FIRST_VISIBLE));
            } else {
                c.cycles += 3;
                c.g[V0][0] = (c.g[S1][0] < c.g[A0][0]) as u64;
                let above_top = c.g[V0][0] != 0;
                c.g[A2][0] = 2;
                if above_top {
                    c.cycles += 19;
                    c.sb(r(c, S0).wrapping_add(FIRST_VISIBLE), c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw(r(c, A3).wrapping_add(0xd0));
                    c.g[A3][0] = c.g[S1][0] & 0xff;
                    notify(c, 0x0039_ba50)?;
                    c.g[A0][0] = c.lw(r(c, S0).wrapping_add(0x5c));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = c.lbu(r(c, S0).wrapping_add(FIRST_VISIBLE));
                    c.g[A2][0] = 4;
                    c.g[T0][0] = c.lw(r(c, A0).wrapping_add(0xd0));
                    notify(c, 0x0039_ba78)?;
                }
                c.cycles += 1;
                c.g[A1][0] = c.lbu(r(c, S0).wrapping_add(FIRST_VISIBLE));
            }
            c.cycles += 7;
            c.g[A0][0] = sx(r(c, S0).wrapping_add(CHILDREN));
            c.g[A1][0] = sx(r(c, S1).wrapping_sub(r(c, A1)));
            call_ptr(c, 0x0039_7870, 0x0039_ba88)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_ad38, 0x0039_ba98)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
