//! Native WScriptMan_RunKillScript (0x0030a5c0), retained uninstalled.
//! No tests, cargo, game or replays: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    run_kill_script,
    "WScriptMan_RunKillScript",
)];

const ENTRY: u32 = 0x0030_a5c0;
const AFTER_LOOKUP: u32 = 0x0030_a5e4;
const AFTER_CALL: u32 = 0x0030_a5fc;
const END: u32 = 0x0030_a610;
const WORLD: u32 = 0x28c;
const CURRENT_INSTANCE: u32 = 0x290;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

/// `void WScriptMan_RunKillScript(WScriptMan* manager, Instance* instance)`.
/// Set manager+0x290 to the instance and resolve its native callback slot 4
/// through World_GetObjectScriptFunc0x3a6b78. A function RID other than -1 is passed
/// immediately to WScriptMan_CallFunc0x309c88 as a one-word stack record.
/// This wrapper queues no command and writes no rider/contact/process fields.
/// It clears manager+0x290 to zero afterwards, rather than restoring its old
/// value, and leaves the callee/lookup result in V0 without interpretation.
pub fn run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    Frame::open(c, 0x30);
                    c.g[V0][0] = c.g[A1][0];
                    frame.save(c, S0, 0x20);
                    frame.save_ra(c, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    c.sw(r(c, S0).wrapping_add(CURRENT_INSTANCE), c.g[V0][0]);
                    c.g[A2][0] = 4;
                    c.g[A0][0] = c.lw(r(c, S0).wrapping_add(WORLD));
                    call_ptr(c, 0x003a_6b78, AFTER_LOOKUP)?;
                    pc = AFTER_LOOKUP;
                }
                AFTER_LOOKUP => {
                    c.g[V1][0] = !c.g[V0][0];
                    let absent = c.g[V1][0] == 0;
                    c.sw(r(c, SP), c.g[V0][0]);
                    if absent { break }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0030_9c88, AFTER_CALL)?;
                    pc = AFTER_CALL;
                }
                AFTER_CALL => break,
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        c.sw(r(c, S0).wrapping_add(CURRENT_INSTANCE), 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}
