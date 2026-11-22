//! Native default selection for the Single Event peak widget.

use crate::call::{call_ptr, guest, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_09d0,
    event_select_on_widget_created,
    "EventSelect_OnWidgetCreated",
)];

const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
const BE_NEW_RACE_INT_GET_TRACK_INFO: u32 = 0x0014_4bc0;
const STR_HASH: u32 = 0x0031_7670;
const UI_MENU_SET_SELECTION: u32 = 0x0039_b960;
const PEAK_LIST_NAME: u32 = 0x0047_0600;
const OBJECT_NAME: u32 = 0x38;
const TRACK_PEAK: u32 = 0x54;

/// `void EventSelect_OnWidgetCreated(EventSelect* self, UIObject* widget)`
/// (0x2009d0): initialize only the SP_PeakList menu's current selection.
/// Front-end flows select item 2 (Peak 1); overlay flows select the item
/// `2 - currentTrack.peak`. This does not assign child widget IDs.
pub fn event_select_on_widget_created(c: &mut Cpu) {
    if crate::custom_tracks::assets_available() {
        let widget = c.g[A1][0] as u32;
        if widget != 0 {
            let name = c.bus.read32(widget + OBJECT_NAME);
            let hash = crate::game::hashvalue::str_hash;
            if name == hash(b"Peak4") {
                c.bus.write32(widget + 0x18, 3);
            } else if name == hash(b"BonusRace") {
                c.bus.write32(widget + 0x18, 0);
            } else if name == hash(b"BonusFreestyle") {
                c.bus.write32(widget + 0x18, 1);
            } else {
                for tab in 0..2 {
                    for (index, _) in crate::custom_tracks::menu_tracks(tab).enumerate() {
                        let kind = if tab == 0 { "Race" } else { "Freestyle" };
                        if name == hash(format!("Tricky{kind}{index}").as_bytes()) {
                            c.bus.write32(widget + 0x18, index as u32);
                        }
                    }
                }
            }
        }
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0020_09ec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0020_09f8)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(OBJECT_NAME)).lw(c);
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = sx(PEAK_LIST_NAME);
        call_ptr(c, STR_HASH, 0x0020_0a0c)?;
        let is_peak_list = c.g[S0][0] == c.g[V0][0];
        frame.restore(c, S0, 0x30);
        if is_peak_list {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xb5c)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let front_end = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if front_end {
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, UI_MENU_SET_SELECTION, 0x0020_0a2c)?;
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, BE_NEW_RACE_INT_GET_TRACK_INFO, 0x0020_0a3c)?;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(TRACK_PEAK)).lbu(c);
                c.g[A1][0] = 2;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A1][0] &= 0xff;
                call_ptr(c, UI_MENU_SET_SELECTION, 0x0020_0a54)?;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
