//! `src/main/game2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::gp;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
use crate::game::visualfx::boardwakefx as bw;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_1cf0, app_module_try_exit, "AppModule_TryExit"),
    (
        0x0023_0f40,
        game_module_process_state_change,
        "GameModule_ProcessStateChange",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
}

/// `bool AppModule_TryExit(AppModule *self)` (0x231cf0): mark the flag at `+8`
/// and report whether the fade value at `+4` has reached the threshold at
/// `gp-0x51b4` (the FPU condition is left set either way).
pub fn app_module_try_exit(c: &mut Cpu) {
    const FADE: u32 = 0x4;
    const FLAG: u32 = 0x8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    let threshold = Addr((c.g[GP][0] as u32).wrapping_sub(0x51b4)).read_fpu(c); // f1
    c.f[1] = threshold.0;
    c.g[V0][0] = 1;
    let current = this.off(FADE).read_fpu(c); // f0
    c.f[0] = current.0;
    let done = threshold.le(current);
    abi::set_fcc(c, done);
    this.off(FLAG).sw(c, c.g[V0][0]);
    if !done {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `GameModule_CanRestartFromPause` (0x231ab8): true when the game module is
/// not loading and its state is not one of the pause-menu transition states.
pub fn game_module_can_restart_from_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(this, 0x210)).lw(c);
        c.g[V0][0] = 5;
        let zero = c.g[V1][0] == 0;
        let mut skip_check = false;
        if zero {
            c.cycles += 1;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            skip_check = true;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0xe;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
                frame.restore(c, S1, 0x10);
                frame.restore(c, S0, 0x20);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        }
        let _ = skip_check;
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023_1b0c);
        c.g[S0][0] = 0;
        call_ptr(c, callee::GAME_STATE_IS_LOADING, 0x0023_1b0c)?;
        let loaded = c.g[V0][0] == 0;
        c.g[V0][0] = 5;
        if loaded {
            c.g[S0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(this, 0x214)).lw(c);
            let five = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if !five {
                c.cycles += 2;
                let fourteen = c.g[A0][0] == c.g[V0][0];
                c.g[V1][0] = 1;
                if !fourteen {
                    c.cycles += 2;
                    let one = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = c.g[A0][0] ^ 0xa;
                    if !one {
                        c.cycles += 2;
                        c.g[S0][0] = c.g[V1][0];
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = 0;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod game_module {

    /// The loader (`GameViewMan_UpdateSplitScreen` argument).
    pub const LOADER: u32 = 0x84;

    /// The previous state object.
    pub const PREV: u32 = 0x1fc;

    /// The current state object (null for none).
    pub const CUR: u32 = 0x200;

    /// The pending (entered, still updating) state object.
    pub const PENDING: u32 = 0x204;

    /// Id (1..16) of the state to enter as pending; 0 for none.
    pub const PENDING_ID: u32 = 0x208;

    /// Id of the last entered pending state.
    pub const ENTERED_ID: u32 = 0x20c;

    /// Id (1..16) of the state to switch to; 0 for none.
    pub const NEXT_ID: u32 = 0x210;

    pub const CUR_ID: u32 = 0x214;

    pub const PREV_ID: u32 = 0x218;

    /// Argument for the next state's start.
    pub const NEXT_ARG: u32 = 0x21c;

    /// Non-zero holds the module while the global run flag is clear.
    pub const HOLD: u32 = 0x224;

    /// The state objects for ids 1..16 (each with its vtable at `+0xc`).
    pub const STATES: [u32; 16] = [
        0xb0, 0xd0, 0xe0, 0xf0, 0x100, 0x118, 0x128, 0x138, 0x148, 0x158, 0x178, 0x18c, 0x19c,
        0x1b8, 0x1dc, 0x1ec,
    ];

    /// State vtable slots.
    pub const VT_ENTER: u32 = 0x10;

    pub const VT_UPDATE: u32 = 0x18;

    pub const VT_START: u32 = 0x28;

    pub const VT_TICK: u32 = 0x30;

    pub const VT_EXIT: u32 = 0x38;

    /// The two `switch` jump tables (pending id, next id).
    pub const PENDING_TABLE: u32 = 0x0047_b7b0;

    pub const NEXT_TABLE: u32 = 0x0047_b7f0;

    /// The case labels of the two tables.
    pub const PENDING_CASE0: u32 = 0x0023_0ffc;

    pub const NEXT_CASE0: u32 = 0x0023_1124;

    pub const START: u32 = 0x0023_0f40;

    pub const END: u32 = 0x0023_124c;
}

/// `state->vt[slot](state)` (state vtable at `+0xc`), returning to `ret`.
pub(crate) fn game_state_call(c: &mut Cpu, obj: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let state = r32(c, obj);
    c.g[V0][0] = c.lw(bw::p(state, 0xc));
    c.g[A0][0] = c.lh(bw::p(r32(c, V0), slot));
    c.g[V1][0] = c.lw(bw::p(r32(c, V0), slot + 4));
    let method = r32(c, V1);
    c.g[A0][0] = sx(state.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void GameModule_ProcessStateChange(GameModule* m)` (0x230f40): the
/// game's state machine step. Unless held (the global run flag at
/// 0x534b30 clear with `hold` set, or flag 0x20 at gp-0x6a0): tick the
/// current state (not while state 3 waits for no switch); enter a pending
/// state (`enter`) and keep updating it; then, on a requested switch, exit
/// the current state, remember it as previous, select the next (ids 1..16),
/// drop the pending state if it is the one now current, refresh the split
/// screen and start the new state with the request's argument.
pub fn game_module_process_state_change(c: &mut Cpu) {
    use game_module::*;
    if c.pc != START {
        let pc = c.pc;
        c.jump_out(pc, START, END);
        return;
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(bw::gp(c, -0x848));
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 7;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S0);
        c.g[A0][0] = c.lw(bw::p(r32(c, V0), 0x78));
        call_ptr(c, 0x0014_dd58, 0x0023_0f68)?; // BELibrary_GetInterface(be, 7)
        'out: {
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = c.lw(0x0053_4b30);
            let running = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(bw::gp(c, -0x6a0));
            if !running {
                c.cycles += 3;
                c.g[V0][0] = c.lw(bw::p(m, HOLD));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'out;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(bw::gp(c, -0x6a0));
            }
            c.cycles += 3;
            c.g[V0][0] &= 0x20;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Tick the current state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, CUR));
            c.g[V0][0] = 3;
            'ticked: {
                if c.g[A1][0] != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(bw::p(m, CUR_ID));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(bw::p(m, NEXT_ID));
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
                            break 'ticked;
                        }
                        c.cycles += 1;
                    }
                    c.g[V0][0] = c.lw(bw::p(r32(c, A1), 0xc));
                    c.cycles += 4;
                    c.g[A0][0] = c.lh(bw::p(r32(c, V0), VT_TICK));
                    c.g[V1][0] = c.lw(bw::p(r32(c, V0), VT_TICK + 4));
                    let method = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0023_0fcc)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
            }
            // Enter the pending state.
            c.cycles += 2;
            let none = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V0).wrapping_sub(1));
            if !none {
                c.cycles += 3;
                let in_range = c.g[V1][0] < 0x10;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = 0x0048_0000;
                if in_range {
                    c.cycles += 6;
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = sx(PENDING_TABLE);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(PENDING_TABLE));
                    c.g[A0][0] = c.lw(r32(c, V1));
                    let target = r32(c, A0);
                    let k = target.wrapping_sub(PENDING_CASE0) / 8;
                    if target.wrapping_sub(PENDING_CASE0) % 8 != 0 || k >= 16 {
                        c.jump_out(target, START, END);
                        return Ok(());
                    }
                    c.cycles += if k == 15 { 1 } else { 2 };
                    c.g[V0][0] = sx(m.wrapping_add(STATES[k as usize]));
                    c.cycles += 1;
                    c.sw(bw::p(m, PENDING), c.g[V0][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
                c.sw(bw::p(m, PENDING_ID), 0);
                c.sw(bw::p(m, ENTERED_ID), c.g[V0][0]);
                c.g[A1][0] = c.lw(bw::p(m, PENDING));
                game_state_call(c, A1, VT_ENTER, 0x0023_10a0)?;
            }
            // Update the pending state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, PENDING));
            if c.g[A1][0] != 0 {
                c.cycles += 6;
                game_state_call(c, A1, VT_UPDATE, 0x0023_10c0)?;
            }
            c.g[S1][0] = c.lw(bw::p(m, NEXT_ID));
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Switch: exit the current state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, CUR));
            c.g[S2][0] = c.lw(bw::p(m, NEXT_ARG));
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                game_state_call(c, A1, VT_EXIT, 0x0023_10ec)?;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw(bw::p(m, CUR_ID));
            c.g[A1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[V1][0] = c.lw(bw::p(m, CUR));
            let in_range = c.g[A1][0] < 0x10;
            c.g[A0][0] = in_range as u64;
            c.sw(bw::p(m, PREV_ID), c.g[V0][0]);
            c.sw(bw::p(m, PREV), c.g[V1][0]);
            if in_range {
                c.cycles += 7;
                c.g[V0][0] = sx(NEXT_TABLE);
                c.g[V1][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(NEXT_TABLE));
                c.g[A0][0] = c.lw(r32(c, V1));
                let target = r32(c, A0);
                let k = target.wrapping_sub(NEXT_CASE0) / 0xc;
                if target.wrapping_sub(NEXT_CASE0) % 0xc != 0 || k >= 16 {
                    c.jump_out(target, START, END);
                    return Ok(());
                }
                c.cycles += if k == 15 { 2 } else { 3 };
                c.g[V0][0] = sx(m.wrapping_add(STATES[k as usize]));
                c.g[V1][0] = k as u64 + 1;
                c.cycles += 2;
                c.sw(bw::p(m, CUR), c.g[V0][0]);
                c.sw(bw::p(m, CUR_ID), c.g[V1][0]);
            }
            c.cycles += 5;
            c.g[V1][0] = c.lw(bw::p(m, ENTERED_ID));
            c.g[V0][0] = c.lw(bw::p(m, CUR_ID));
            c.sw(bw::p(m, NEXT_ID), 0);
            let differ = c.g[V1][0] != c.g[V0][0];
            c.sw(bw::p(m, NEXT_ARG), 0);
            if !differ {
                // The pending state is the new current one.
                c.cycles += 2;
                c.sw(bw::p(m, ENTERED_ID), 0);
                c.sw(bw::p(m, PENDING), 0);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(bw::p(m, CUR));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Start the new state.
            c.cycles += 10;
            c.g[A0][0] = c.lw(bw::p(m, LOADER));
            call_ptr(c, 0x0022_e550, 0x0023_1218)?; // GameViewMan_UpdateSplitScreen
            c.g[A2][0] = c.lw(bw::p(m, CUR));
            c.g[A1][0] = c.g[S2][0];
            game_state_call(c, A2, VT_START, 0x0023_1234)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::check_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn app_module_try_exit_matches() {
        check_paths(
            ssx3_ee::s06::f_00231cf0,
            app_module_try_exit,
            300,
            &[9, 10],
            &|c, seed| {
                let mut r = Rng(0x0c11_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                let threshold = match seed % 4 {
                    0 => 0x3f80_0000,
                    1 => 0xc000_0000,
                    2 => 0,
                    _ => r.edge_f32(),
                };
                let current = if seed % 2 == 0 {
                    0x3f00_0000
                } else {
                    0x4000_0000
                };
                c.bus.write32(GAME_GP.wrapping_sub(0x51b4), threshold);
                c.bus.write32(DATA + 0x4, current);
            },
        );
    }

    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::fill;
    use crate::game::visualfx::boardwakefx::tests::gp_at;

    /// `GameModule_ProcessStateChange` over the hold conditions, the tick
    /// (state 3 waiting on a switch), pending ids 0..17 and next ids 0..17
    /// (17: outside the `switch`), with every state method a fake (some
    /// seeds unwind at one) and `GameViewMan_UpdateSplitScreen` on its short
    /// path.
    #[test]
    fn game_module_process_state_change_matches() {
        use game_module::*;
        check(
            ee::s06::f_00230f40__generated,
            game_module_process_state_change,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x230f, 6);
                let (m, world, g, be, be_vt, svt, loader) = (
                    DATA,
                    DATA + 0x6000,
                    DATA + 0x6200,
                    DATA + 0x6400,
                    DATA + 0x6500,
                    DATA + 0x7000,
                    DATA + 0x6800,
                );
                fill(c, &mut r, m, 0x90);
                // BELibrary_GetInterface(world->be, 7): a fake interface method.
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x78, r.next_u32());
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, be);
                c.bus.write32(be + 4, be_vt);
                c.bus.write16(be_vt + 8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(be_vt + 0xc, FAKE + 0x40);
                // GameViewMan_UpdateSplitScreen: the view count is unchanged.
                c.bus.write32(world + 0x84, g);
                let views = r.next_u32();
                c.bus.write32(g + 0x214, views);
                c.bus.write32(m + LOADER, loader);
                c.bus.write32(loader + 0xc, views);
                // The state objects share a vtable of fakes.
                for off in STATES {
                    c.bus.write32(m + off + 0xc, svt);
                }
                for (i, slot) in [VT_ENTER, VT_UPDATE, VT_START, VT_TICK, VT_EXIT]
                    .into_iter()
                    .enumerate()
                {
                    c.bus.write16(svt + slot, (r.next_u32() & 0x3c) as u16);
                    c.bus.write32(svt + slot + 4, FAKE + 0x50 + 0x10 * i as u32);
                }
                for k in 0..16u32 {
                    c.bus.write32(PENDING_TABLE + 4 * k, PENDING_CASE0 + 8 * k);
                    c.bus.write32(NEXT_TABLE + 4 * k, NEXT_CASE0 + 0xc * k);
                }
                // Hold: the run flag, the module's hold, and gp-0x6a0 bit 0x20.
                c.bus.write32(0x0053_4b30, (seed % 7 != 0) as u32);
                c.bus
                    .write32(m + HOLD, if seed % 2 == 0 { 0 } else { r.next_u32() });
                let flags = r.next_u32() & !0x20;
                c.bus.write32(
                    gp_at(-0x6a0),
                    if seed % 11 == 3 { flags | 0x20 } else { flags },
                );
                let state = |r: &mut Rng| {
                    if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        m + STATES[(r.next_u32() % 16) as usize]
                    }
                };
                let cur = state(&mut r);
                c.bus.write32(m + CUR, cur);
                let pending = state(&mut r);
                c.bus.write32(m + PENDING, pending);
                let id = |r: &mut Rng| match r.next_u32() % 5 {
                    0 => 0,
                    1 => 17 + r.next_u32() % 3,
                    _ => 1 + r.next_u32() % 16,
                };
                c.bus
                    .write32(m + CUR_ID, if seed % 3 == 0 { 3 } else { id(&mut r) });
                c.bus.write32(m + NEXT_ID, id(&mut r));
                c.bus.write32(m + PENDING_ID, id(&mut r));
                c.bus.write32(
                    m + ENTERED_ID,
                    if seed % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 16
                    },
                );
                c.bus.write32(m + NEXT_ARG, r.next_u32());
                c.g[A0][0] = sx(m);
                c.pc = START;
            },
        );
    }
}
