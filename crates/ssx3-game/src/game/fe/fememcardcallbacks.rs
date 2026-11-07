//! `src/fe/fememcardcallbacks.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001c_5dd8,
    c_fe_popup_confirm_rebuild,
    "cFEPopupConfirm_Rebuild",
)];

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

pub(crate) mod callee {

    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
}

/// `void cFEPopupConfirm_Rebuild(self)` (0x001c5dd8): re-lay out the confirm
/// popup when it has a background, then scale and show it.
pub fn c_fe_popup_confirm_rebuild(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0x2bc)).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
            c.cycles += 3;
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 21;
        callee::FE_POPUP_CONFIRM_RESET_LAYOUT.call(c, 0x001c_5dfc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(word(c, S0).wrapping_add(0xbc));
        callee::FE_POPUP_CONFIRM_APPLY_CONFIG.call(c, 0x001c_5e08)?;
        c.g[A0][0] = c.g[S0][0];
        c.ldl(2, at(c, S0, 0x27f));
        c.ldr(2, at(c, S0, 0x278));
        c.g[V1][0] = Addr(at(c, S0, 0x280)).lw(c);
        c.sdl(2, at(c, SP, 0x7));
        c.sdr(2, at(c, SP, 0));
        Addr(at(c, SP, 0x8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[SP][0];
        callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE.call(c, 0x001c_5e2c)?;
        c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(at(c, V0, 0x30)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x34)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V1), 0x001c_5e44)?;
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::DATA;

    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;

    pub(crate) use crate::game::movie::movieplayer::tests::w;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;

    // ------------------------------------------------------------ front end II

    #[test]
    fn c_fe_popup_confirm_rebuild_matches() {
        check(
            ee::s03::f_001c5dd8__generated,
            c_fe_popup_confirm_rebuild,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1c5d_d8, 6);
                let self_ = DATA;
                let child = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x2bc, (seed % 2) as u32);
                w(c, self_ + 0x8, child);
                ret_slot(c, child, 0x30, RET_ZERO);
                let _ = vtable;
                w(c, self_ + 0x278, 0x1122_3344);
                w(c, self_ + 0x27c, 0x5566_7788);
                w(c, self_ + 0x280, 0x99aa_bbcc);
                w(c, self_ + 0xbc, 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }
}
