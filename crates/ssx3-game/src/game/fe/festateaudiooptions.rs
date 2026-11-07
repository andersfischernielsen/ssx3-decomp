//! `src/fe/festateaudiooptions.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::set_widget_enabled;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0019_7b88,
    fe_state_request_line_refresh_list,
    "cFEStateRequestLine_RefreshList",
)];

// ---------------------------------------------------------------------------
// Callees (the generated name of each address, so a later hand-written
// replacement of a callee is reached through its override slot too).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
}

/// `void cFEStateRequestLine_RefreshList(cFEStateRequestLine* self)`
/// (0x197B88): refresh the song names, icons, scroll arrows and the song
/// count of the request-line screen.
pub fn fe_state_request_line_refresh_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::REQUEST_LINE_UPDATE_SONG_NAMES.call(c, 0x0019_7b9c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SONG_ICONS.call(c, 0x0019_7ba4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS.call(c, 0x0019_7bac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SONG_COUNT.call(c, 0x0019_7bb4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_UpdateScrollArrows(self)` (0x197db8): the same
/// two-arrow update for the jukebox request list (widgets at +0x1ec/+0x1f0).
pub fn cfe_request_line_update_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let count = Addr(this as u32).off(0x150).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x1ec).lw(c);
        if count == 0 {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x0019_7dec)?;
            c.cycles += 5;
        } else {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x0019_7e0c)?;
            c.cycles += 5;
        }
        c.g[V0][0] = Addr(this as u32).off(0x150).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x1f0).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x14c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x0019_7e5c)?;
        } else {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x0019_7e3c)?;
        }
        frame.restore(c, S0, 0x10);
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

    pub(crate) use crate::game::fe::festaterewards2::tests::belib;

    #[test]
    fn fe_state_request_line_refresh_list_matches() {
        check(
            ssx3_ee::s02::f_00197b88__generated,
            fe_state_request_line_refresh_list,
            60,
            &|c, seed| {
                machine(c, seed, 0x197b_88, 4);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }
}
