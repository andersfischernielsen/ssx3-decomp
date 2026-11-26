//! Staging module oc744 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc744`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0030_2df0, w_script_fn_next_task, "WScriptFn_NextTask")];

/// `Result* WScriptFn_NextTask(Result* ret, int n, Arg* args)` (0x302df0):
/// the world-script command that advances to the next task. Binds the call's
/// arguments into a 2-word record (defaults `{-1, ..}` set once on first
/// use, a typed int given for a float slot is converted), then calls
/// `WScriptMan_ProcessCommand(world manager, rec[0], 2)` and returns the
/// static "no value" result through `ret`.
///
/// Resume entries (`pc` inside the body) go to the generated `__entries`
/// twin; the orchestrator keeps that fallback at install.
pub fn w_script_fn_next_task(c: &mut Cpu) {
    if c.pc != 0x0030_2df0 {
        return ee::s10::f_00302df0__entries(c);
    }
    guest(c, |c| {
        'b_302e38: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26b0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A2][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_302e38;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x26ac));
                c.g[V1][0] = 0xffffffffffff0000;
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[A0][0] = 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x26a8), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x26b0), c.g[A0][0]);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                break 'b_302e38;
            }
        }
        'b_302ea8: {
            c.cycles += 7;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = 0x8;
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x26a8));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x00302e4c)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_302ea8;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xc18));
                c.g[A3][0] = 0x2;
                'l_302e60: loop {
                    'b_302e9c: {
                        'b_302e98: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_302e98;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_302e98;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_302e9c;
                                    } else {
                                        break 'b_302e98;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_302e9c;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00302e60);
                        }
                        continue 'l_302e60;
                    } else {
                        break 'b_302ea8;
                    }
                }
            }
        }
        c.cycles += 20;
        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x2;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x0030a728, 0x00302eb8)?;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const OC744_PARAMS: u32 = DATA + 0x25_0000;
    const OC744_SRET: u32 = DATA + 0x26_0000;
    /// The word the gated fake looks for (other tests' machines lack it).
    const OC744_GATE: u32 = DATA + 0x3f_0000;
    const OC744_MAGIC: u32 = 0x0744_0744;

    static OC744_PREV_PROCESS: AtomicUsize = AtomicUsize::new(0);

    /// `WScriptMan_ProcessCommand` (0x30a728), gated: logs the call and returns.
    fn oc744_fake_process(c: &mut Cpu) {
        if c.bus.read32(OC744_GATE) != OC744_MAGIC {
            return fakes::fall_back(c, &OC744_PREV_PROCESS, ee::s10::f_0030a728__generated);
        }
        fakes::fake_call(c, 0x0030_a728);
    }

    /// `WScriptFn_NextTask`: first call (record defaults set) or later ones,
    /// 0..3 and -1 arguments whose type matches the table (integer store),
    /// differs from it (integer store) or is the float slot type (converted).
    #[test]
    fn w_script_fn_next_task_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A728),
            oc744_fake_process,
            &OC744_PREV_PROCESS,
        );
        check(
            ee::s10::f_00302df0__generated,
            w_script_fn_next_task,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_2df0, 2);
                let gp = GAME_GP;
                c.bus.write32(gp + 0x26b0, r.next_u32() & 1);
                c.bus.write32(gp + 0x26a8, r.next_u32());
                c.bus.write8(gp + 0x26ac, r.next_u32() as u8);
                for i in 0..4u32 {
                    c.bus.write32(gp + 0xc18 + 4 * i, r.next_u32() % 4);
                }
                for i in 0..4u32 {
                    let p = OC744_PARAMS + 16 * i;
                    c.bus.write32(p, r.next_u32() % 4);
                    c.bus.write32(p + 4, r.next_u32() & 0xffff);
                    c.bus.write32(p + 0xc, r.next_u32() % 4);
                }
                c.bus.write32(gp + 0xce8, DATA + 0x27_0000);
                c.g[A0][0] = sx(OC744_SRET);
                c.g[A1][0] = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
                c.g[A2][0] = sx(OC744_PARAMS);
                c.bus.write32(OC744_GATE, OC744_MAGIC);
                c.pc = 0x0030_2df0;
            },
        );
    }
}
