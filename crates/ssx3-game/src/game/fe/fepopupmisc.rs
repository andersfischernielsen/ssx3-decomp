//! `src/fe/fepopupmisc.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use ssx3_ee_rt::cpu::reg::*;
#[cfg(test)]
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001e_39f8,
    message_center_get_inbox,
    "MessageCenter_GetInbox",
)];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
}

/// `void MessageCenter_GetInbox(Inbox* self)` (0x1e39f8): the inbox of the
/// local player's profile (`BELibrary` interface 1, slot 0).
pub fn message_center_get_inbox(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001e_3a08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001e_3a14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_NEW_PLAYER_GET_INBOX, 0x001e_3a20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_2 as be_library;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;

    #[test]
    fn message_center_get_inbox_matches() {
        check(
            ssx3_ee::s04::f_001e39f8__generated,
            message_center_get_inbox,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0xe39f8, 2);
                be_library(c);
                // Empty BELibrary interface table: `GetInterface` resolves to
                // address 0, so the virtual call lands in the miss hook.
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
                c.bus.write32(0x0053_08a8, 0);
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
    }
}
