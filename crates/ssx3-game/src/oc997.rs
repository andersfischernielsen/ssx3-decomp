//! Staging module oc997 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc997`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_2360,
        ov_screen_autosave_update_icon,
        "OVScreenAutosave_UpdateIcon",
    ),
    (
        0x0021_2680,
        ov_screen_debug_printf_stub,
        "OVScreen_DebugPrintfStub",
    ),
    (
        0x0021_c678,
        ov_state_overlay_ctor_2,
        "cOVStateOverlay_ctor_2",
    ),
    (0x0021_c6d8, ov_state_overlay_dtor, "cOVStateOverlay_dtor"),
];

/// The field at `off` from the pointer held in register `reg`, read as the
/// compiled code does (32-bit address arithmetic).
fn oc997_reg_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

// ---------------------------------------------------------------------------
// Profile save PDA and autosave overlays (`src/fe/ovstateprofile.cpp`).

/// `void OVScreenProfileSavePDA_Shutdown(OVScreenProfileSavePDA* self)`
/// (0x211b10): tears down the memory-card side of the screen and clears the
/// pending-info field at +0x1a4.
#[allow(dead_code)]
pub fn ov_screen_profile_save_pda_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_58b8, 0x0021_1b24)?;
        oc997_reg_at(c, S0, 0x1a4).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_Update(OVScreenAutosave* self)` (0x212208): the
/// autosave overlay's per-frame step. In the check phase (state 1) it starts
/// the save (file type 1); in the save phase it gives up once the device is
/// gone; then the memory-card screen updates. State at +0x1c0, flags at
/// +0x1a8, +0x1dc, +0x1e4, +0x1f0.
#[allow(dead_code)]
pub fn ov_screen_autosave_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        MC_MANAGER_GET.call(c, 0x0021_2228)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[S1][0] = oc997_reg_at(c, S0, 0x1c0).lw(c);
        c.g[V0][0] = 1;
        'tail: {
            'p: {
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V1][0] = oc997_reg_at(c, S0, 0x1c0).lw(c);
                    break 'p;
                }
                c.cycles += 3;
                c.g[V0][0] = oc997_reg_at(c, S0, 0x1a8).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = oc997_reg_at(c, S0, 0x1c0).lw(c);
                    break 'p;
                }
                c.cycles += 10;
                c.g[A0][0] = c.g[A1][0];
                c.g[A2][0] = 0;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_cae8, 0x0021_2258)?;
                c.g[V0][0] = 3;
                let s1 = c.g[S1][0];
                oc997_reg_at(c, S0, 0x1f0).sw(c, s1);
                let v0 = c.g[V0][0];
                oc997_reg_at(c, S0, 0x1c0).sw(c, v0);
                oc997_reg_at(c, S0, 0x1a8).sw(c, s1);
                oc997_reg_at(c, S0, 0x1dc).sw(c, s1);
                break 'tail;
            }
            c.cycles += 3;
            c.g[V0][0] = 3;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0xe0;
            if t {
                break 'tail;
            }
            'q: {
                'qa: {
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x428)).lw(c);
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x350)).lw(c);
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0];
                    if t {
                        break 'qa;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x344)).lw(c);
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 1;
                        break 'q;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x348)).lw(c);
                    let t = c.g[V1][0] != 0;
                    if t {
                        break 'tail;
                    }
                    break 'qa;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
            c.cycles += 5;
            c.g[V1][0] = 6;
            let v0 = c.g[V0][0];
            oc997_reg_at(c, S0, 0x1e4).sw(c, v0);
            let v1 = c.g[V1][0];
            oc997_reg_at(c, S0, 0x1c0).sw(c, v1);
            oc997_reg_at(c, S0, 0x1dc).sw(c, 0);
            oc997_reg_at(c, S0, 0x1a8).sw(c, 0);
            break 'tail;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_8de0, 0x0021_22cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_UpdateIcon(OVScreenAutosave* self)` (0x212360):
/// shows or hides the autosave icon and refreshes its label and animation
/// from the card state; `self` keeps the icon widget at +0x40.
pub fn ov_screen_autosave_update_icon(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        MC_MANAGER_GET.call(c, 0x0021_2384)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = oc997_reg_at(c, S1, 0x1a8).lw(c);
        c.g[V0][0] = oc997_reg_at(c, S1, 0x230).lw(c);
        'body: {
            if c.g[V1][0] == c.g[V0][0] {
                frame.restore(c, S0, 0x30);
                break 'body;
            }
            c.cycles += 10;
            let v1 = c.g[V1][0];
            oc997_reg_at(c, S1, 0x230).sw(c, v1);
            c.g[A0][0] = 0x004a_2770;
            call_ptr(c, 0x0031_7670, 0x0021_23a8)?;
            c.g[A0][0] = oc997_reg_at(c, S1, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_23b4)?;
            c.g[S2][0] = c.g[V0][0];
            if c.g[S2][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'body;
            }
            'b478: {
                c.cycles += 3;
                c.g[V0][0] = oc997_reg_at(c, S1, 0x1a8).lw(c);
                if c.g[V0][0] == 0 {
                    break 'b478;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_c898, 0x0021_23d4)?;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 6;
                if t {
                    break 'b478;
                }
                c.cycles += 3;
                c.g[V1][0] = oc997_reg_at(c, S1, 0x1c0).lw(c);
                let t = c.g[V1][0] == c.g[V0][0];
                if t {
                    break 'b478;
                }
                'b424: {
                    c.cycles += 10;
                    c.g[S0][0] = oc997_reg_at(c, S1, 0x10).lw(c);
                    c.g[A0][0] = 0x0047_1e48;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    call_ptr(c, 0x0031_7670, 0x0021_23fc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_7140, 0x0021_2408)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        break 'b424;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 9;
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0;
                    call_ptr(c, 0x0039_fcc8, 0x0021_2424)?;
                }
                c.cycles += 21;
                c.g[A0][0] = 0x0047_1e38;
                call_ptr(c, 0x0031_7670, 0x0021_2430)?;
                c.g[A0][0] = oc997_reg_at(c, S1, 0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0021_243c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[V0][0] = oc997_reg_at(c, S0, 0x8).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0021_2458)?;
                c.g[A0][0] = 0x0047_1e58;
                call_ptr(c, 0x0031_7670, 0x0021_2464)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0021_2470)?;
                frame.restore(c, S0, 0x30);
                break 'body;
            }
            c.cycles += 15;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039_fd38, 0x0021_2480)?;
            c.g[A0][0] = 0x0047_1e38;
            call_ptr(c, 0x0031_7670, 0x0021_248c)?;
            c.g[A0][0] = oc997_reg_at(c, S1, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_2498)?;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
            let t = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0021_24b0)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_Shutdown(OVScreenAutosave* self)` (0x212508): tears
/// down the memory-card screen, then leaves the screen by the parent it was
/// opened from: exit the game (1), the title (6), the lodge (7), or start a
/// race from the hub (0).
#[allow(dead_code)]
pub fn ov_screen_autosave_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        call_ptr(c, 0x001d_58b8, 0x0021_2528)?;
        c.g[V1][0] = oc997_reg_at(c, S0, 0x22c).lw(c);
        let t = c.g[V1][0] != c.g[S2][0];
        c.g[V0][0] = 6;
        'exit: {
            'first: {
                if t {
                    break 'first;
                }
                c.cycles += 6;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                call_ptr(c, 0x0023_0488, 0x0021_2544)?;
                frame.restore(c, S0, 0x30);
                break 'exit;
            }
            'l2c0: {
                'l2ac: {
                    'l580: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 7;
                        if t {
                            break 'l580;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = 0x0047_1e70;
                        c.g[A0][0] = 0x4c;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0021_256c)?;
                        c.g[A1][0] = oc997_reg_at(c, S0, 0x10).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0019_4738, 0x0021_2578)?;
                        c.g[A0][0] = oc997_reg_at(c, S0, 0x10).lw(c);
                        break 'l2ac;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x0047_0000;
                    if t {
                        break 'l2c0;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = 0x0047_1e80;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, 0x0021_259c)?;
                    c.g[A1][0] = oc997_reg_at(c, S0, 0x10).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001f_3700, 0x0021_25a8)?;
                    c.g[A0][0] = oc997_reg_at(c, S0, 0x10).lw(c);
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x0021_25b8)?;
                frame.restore(c, S0, 0x30);
                break 'exit;
            }
            c.cycles += 2;
            let t = c.g[V1][0] != 0;
            frame.restore(c, S0, 0x30);
            if t {
                break 'exit;
            }
            c.cycles += 40;
            call_ptr(c, 0x0014_dc80, 0x0021_25d0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0021_25dc)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dc80, 0x0021_25e4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_dd58, 0x0021_25f0)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            call_ptr(c, 0x0014_4df0, 0x0021_2600)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xc;
            call_ptr(c, 0x0014_51e8, 0x0021_260c)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_6d98, 0x0021_2618)?;
            c.g[V1][0] = 0x0053_0000;
            let s2 = c.g[S2][0];
            Addr((c.g[GP][0] as u32).wrapping_sub(0x1f38)).sw(c, s2);
            let v0 = c.g[V0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x5c08)).sw(c, v0);
            c.g[V0][0] = oc997_reg_at(c, S0, 0xc).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0021_2638)?;
            c.g[A1][0] = 0x0047_1e90;
            c.g[A0][0] = 8;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0021_2650)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0023_2720, 0x0021_2658)?;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0021_2664)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreen_DebugPrintfStub(...)` (0x212680): the variadic debug-print
/// stub. It spills the argument registers a1..a3, t0..t3 and f12, f14, f16,
/// f18 into its frame, returns its first argument in v0, and does nothing else.
pub fn ov_screen_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    let base = sp(c);
    c.sd(base.wrapping_add(0x48), c.g[A1][0]);
    c.sd(base.wrapping_add(0x50), c.g[A2][0]);
    c.sd(base.wrapping_add(0x58), c.g[A3][0]);
    c.sd(base.wrapping_add(0x60), c.g[T0][0]);
    c.sd(base.wrapping_add(0x68), c.g[T1][0]);
    c.sd(base.wrapping_add(0x70), c.g[T2][0]);
    c.sd(base.wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, base.wrapping_add(0x38));
    c.swc1(14, base.wrapping_add(0x3c));
    c.swc1(16, base.wrapping_add(0x40));
    c.swc1(18, base.wrapping_add(0x44));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Overlay state base (`src/fe/all_in_one_fe.cpp`).

/// `cOVStateOverlay* cOVStateOverlay_ctor(this, engine, overlayId)` (0x21c678,
/// the second constructor entry): builds the UI state base, then installs the
/// overlay vtable and clears the pause-menu and PDA flags.
pub fn ov_state_overlay_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_c694)?;
        let overlay_id = c.g[S1][0];
        oc997_reg_at(c, S0, 0x98).sw(c, overlay_id);
        c.g[V1][0] = 0x0047_4e08;
        oc997_reg_at(c, S0, 0x48).sw(c, 0);
        let vtable = c.g[V1][0];
        oc997_reg_at(c, S0, 0x8).sw(c, vtable);
        c.g[A0][0] = 0x004a_0000;
        oc997_reg_at(c, S0, 0x4c).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
        oc997_reg_at(c, S0, 0x54).sw(c, 0);
        frame.restore(c, S1, 0x10);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2088)).lhu(c);
        frame.restore_ra(c, 0);
        let chat = c.g[V1][0];
        oc997_reg_at(c, S0, 0x58).sh(c, chat);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVStateOverlay_dtor(this)` (0x21c6d8): resets the vtable to the
/// overlay's and runs the UI state base destructor.
pub fn ov_state_overlay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = 0x0047_4e08;
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0039_e390, 0x0021_c6f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, Rng, DATA, GAME_GP};

    /// Clears `len` bytes at `addr` (word stores).
    fn oc997_clear(c: &mut Cpu, addr: u32, len: u32) {
        for off in (0..len).step_by(4) {
            c.bus.write32(addr + off, 0);
        }
    }

    /// The MC-manager and UI-manager singletons at gp-0x47c and gp-0x10c8:
    /// zeroed dummies, so the lookups do not build the real managers.
    fn oc997_singletons(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x3e000);
        c.bus.write32(GAME_GP.wrapping_sub(0x10c8), DATA + 0x3d000);
        oc997_clear(c, DATA + 0x3d000, 0x80);
        oc997_clear(c, DATA + 0x3e000, 0x200);
    }

    #[test]
    fn ov_screen_autosave_update_icon_matches() {
        check(
            ssx3_ee::s05::f_00212360__generated,
            ov_screen_autosave_update_icon,
            48,
            &|c, seed| {
                fakes::machine(c, seed, 0x0021_2360, 4);
                elf_image(c);
                oc997_singletons(c);
                let this = DATA + 0x100;
                oc997_clear(c, this, 0x300);
                c.g[A0][0] = sx(this);
                // Equal stored and current icon state: the early exit.
                c.bus.write32(this + 0x1a8, seed % 7);
                c.bus.write32(
                    this + 0x230,
                    if seed % 4 == 0 { seed % 7 } else { seed % 5 },
                );
                c.bus
                    .write32(this + 0x1c0, [0, 6, 3, 9][(seed % 4) as usize]);
                c.bus.write32(this + 0x10, DATA + 0x3f800);
                c.bus.write32(this + 0x40, DATA + 0x3f000);
                oc997_clear(c, DATA + 0x3f000, 0x100);
                oc997_clear(c, DATA + 0x3f800, 0x100);
            },
        );
    }

    #[test]
    fn ov_screen_debug_printf_stub_matches() {
        check(
            ssx3_ee::s05::f_00212680__generated,
            ov_screen_debug_printf_stub,
            16,
            &|c, seed| {
                let mut r: Rng = fakes::machine(c, seed, 0x0021_2680, 0);
                for f in [12usize, 14, 16, 18] {
                    c.f[f] = r.next_u32();
                }
            },
        );
    }

    #[test]
    fn ov_state_overlay_ctor_2_matches() {
        check(
            ssx3_ee::s05::f_0021c678__generated,
            ov_state_overlay_ctor_2,
            16,
            &|c, seed| {
                fakes::machine(c, seed, 0x0021_c678, 2);
                elf_image(c);
                oc997_singletons(c);
                let obj = DATA + 0x100;
                oc997_clear(c, obj, 0x300);
                c.g[A0][0] = sx(obj);
                c.g[A2][0] = u64::from(seed);
            },
        );
    }

    #[test]
    fn ov_state_overlay_dtor_matches() {
        check(
            ssx3_ee::s05::f_0021c6d8__generated,
            ov_state_overlay_dtor,
            8,
            &|c, seed| {
                fakes::machine(c, seed, 0x0021_c6d8, 2);
                let obj = DATA + 0x100;
                oc997_clear(c, obj, 0x100);
                c.g[A0][0] = sx(obj);
            },
        );
    }
}
