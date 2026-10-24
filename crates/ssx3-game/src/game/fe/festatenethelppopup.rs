//! `src/fe/festatenethelppopup.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_a510,
        c_fe_state_net_help_popup_set_button_help,
        "cFEStateNetHelpPopup_SetButtonHelp",
    ),
    (
        0x001d_a530,
        c_fe_state_net_help_popup_set_body_text,
        "cFEStateNetHelpPopup_SetBodyText",
    ),
];

// ---------------------------------------------------------------------------
// Net-help popup.

/// `void cFEStateNetHelpPopup_SetButtonHelp(cFEStateNetHelpPopup *this, int index, uint help, uint help2)`
/// (0x1da510): stores `help` and `help2` into the button record at
/// `this + index*4` (+0x68/+0x84).
pub fn c_fe_state_net_help_popup_set_button_help(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let help = c.g[A2][0];
    let help2 = c.g[A3][0];
    c.cycles += 6;
    c.g[A1][0] = sx((index as u32) << 2);
    c.g[V0][0] = sx((this as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V0][0] as u32).off(0x68).sw(c, help);
    Addr(c.g[A0][0] as u32).off(0x84).sw(c, help2);
    abi::ret(c);
}

/// `void cFEStateNetHelpPopup_SetBodyText(cFEStateNetHelpPopup *this, int index, uint text)`
/// (0x1da530): stores `text` at `this + index*4 + 0x58`.
pub fn c_fe_state_net_help_popup_set_body_text(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let text = c.g[A2][0];
    c.cycles += 4;
    c.g[A1][0] = sx((index as u32) << 2);
    c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x58).sw(c, text);
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn c_fe_state_net_help_popup_set_button_help_matches() {
        check(
            ssx3_ee::s04::f_001da510,
            c_fe_state_net_help_popup_set_button_help,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_set_body_text_matches() {
        check(
            ssx3_ee::s04::f_001da530,
            c_fe_state_net_help_popup_set_body_text,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }
}
