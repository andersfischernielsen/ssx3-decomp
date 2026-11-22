//! Native RaceStateCountdown_Enter0x113d38, retained uninstalled.
//! The initial debounce investigation needed this three-instruction leaf.

use crate::abi;
use ssx3_ee_rt::cpu::reg::{A1, V0};
use ssx3_ee_rt::Cpu;

pub fn race_state_countdown_enter(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 180;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
    abi::ret(c);
}
