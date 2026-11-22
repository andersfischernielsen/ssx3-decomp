//! Native WScriptMan_TryRunKillScript (0x002d19e8), retained uninstalled.
//! No tests, builds, game runs or replays: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    try_run_kill_script,
    "WScriptMan_TryRunKillScript",
)];

const ENTRY: u32 = 0x002d_19e8;
const AFTER_HAS_SCRIPT: u32 = 0x002d_1a04;
const AFTER_RUN_SCRIPT: u32 = 0x002d_1a1c;
const END: u32 = 0x002d_1a30;
const MANAGER_GP: u32 = 0xce8;

/// `int WScriptMan_TryRunKillScript(Instance* instance)`.
/// Resolve the manager through gp+0xce8. HasKillScript (0x30a598) receives
/// A0=manager/A1=instance. A zero answer returns 0 without dispatch; otherwise
/// RunKillScript (0x30a5c0) receives the same pair and this wrapper returns 1,
/// independently of that second helper's result. The naming dossier maps
/// both helpers to instance event 4. This wrapper supplies no numeric channel
/// argument and does not determine whether RunKillScript queues its work.
pub fn try_run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 9;
                    Frame::open(c, 0x20);
                    frame.save(c, S0, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MANAGER_GP));
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_a598, AFTER_HAS_SCRIPT)?;
                    pc = AFTER_HAS_SCRIPT;
                }
                AFTER_HAS_SCRIPT => {
                    let present = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MANAGER_GP));
                    if !present {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        break;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0030_a5c0, AFTER_RUN_SCRIPT)?;
                    pc = AFTER_RUN_SCRIPT;
                }
                AFTER_RUN_SCRIPT => {
                    c.g[V0][0] = 1;
                    break;
                }
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
