//! Staging module oc745 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc745`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_39f0,
        wscript_fn_crowd3d_control,
        "WScriptFn_Crowd3dControl",
    ),
    (
        0x0030_4320,
        wscript_fn_set_hud_height_target,
        "WScriptFn_SetHudHeightTarget",
    ),
];

/// The `(params, n)` loop of the WScript bodies: each argument is stored
/// into the block at `vp` by its index, as a float when the kind table says
/// float and the argument's kind differs, else as an int. `a2` holds the
/// parameter list and `count > 0` was checked; `entry` is the loop's entry
/// charge, `top` its back-edge head, `kinds` the kind table.
fn oc745_fill_args(c: &mut Cpu, vp: u64, count: u64, kinds: u32, entry: u64, top: u32) {
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(kinds);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The shared tail: copy the 12 bytes of `luno_gLunoNil` (0x4c9098, `v1` is
/// 0x4d0000 here) into the result `s3`, restore and return it.
fn oc745_return_nil(c: &mut Cpu, cycles: u64) {
    c.cycles += cycles;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
    c.sdr(4, c.g[S3][0] as u32);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    let s = sp(c);
    c.lq(16, s.wrapping_add(0x60));
    c.lq(17, s.wrapping_add(0x50));
    c.lq(18, s.wrapping_add(0x40));
    c.lq(19, s.wrapping_add(0x30));
    c.g[RA][0] = c.ld(s.wrapping_add(0x20));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    ret(c);
}

/// Opens the 0x70-byte frame both functions use (the register saves follow
/// in the bodies, interleaved as the generated code has them).
fn oc745_save_frame(c: &mut Cpu) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
}

/// `LunoVMRegister WScriptFn_Crowd3dControl(LunoVMRegister* out, int n,
/// Param* params)` (0x3039f0, `Crowd3dFunc`): a script call with a
/// (target, command) argument pair. Resolves the target (the manager's own
/// instance, or a world-cache instance record) and, for command 0 or 1,
/// adds or removes it in the crowd manager (`gp+0xf00`). Returns nil.
pub fn wscript_fn_crowd3d_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        oc745_save_frame(c);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26ec));
        let s = sp(c);
        c.sq(18, s.wrapping_add(0x40));
        c.sq(17, s.wrapping_add(0x50));
        c.g[S2][0] = c.g[A2][0];
        c.sq(19, s.wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, s.wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        let ra = c.g[RA][0];
        c.sd(s.wrapping_add(0x20), ra);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x500000;
        if !initialised {
            // Static init of the 12-byte argument block at 0x4fbc70.
            c.cycles += 11;
            c.g[V1][0] = sx(0x500000u32.wrapping_sub(0x4390));
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw(0x500000u32.wrapping_sub(0x4390), c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[V0][0] = 0x1;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x26ec), c.g[V0][0]);
            let s = sp(c);
            c.sb(s, c.g[A1][0]);
        }
        // Copy the block into the frame's 12-byte buffer, then the arg loop.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4390));
        c.g[A2][0] = 0xc;
        let s = sp(c);
        c.sw(s.wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3a6c)?;
        let no_args = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_args {
            let vp = c.g[S0][0];
            let count = c.g[S1][0];
            let kinds = (c.g[GP][0] as u32).wrapping_add(0xc48);
            oc745_fill_args(c, vp, count, kinds, 3, 0x0030_3a80);
        }
        // The target: buf[0] == -1 is the manager's instance, else a record.
        c.cycles += 4;
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let no_section = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if no_section {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !found {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 2;
        let no_obj = c.g[V0][0] == 0;
        c.g[V1][0] = 0x4d0000;
        if !no_obj {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let cmd_zero = c.g[A0][0] == 0;
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            if cmd_zero {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xf00));
                call_ptr(c, 0x0034_3c60, 0x0030_3b40)?;
                c.g[V1][0] = 0x4d0000;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let other = c.g[A0][0] != c.g[V0][0];
                c.g[V1][0] = 0x4d0000;
                if !other {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xf00));
                    call_ptr(c, 0x0034_3f38, 0x0030_3b5c)?;
                    c.g[V1][0] = 0x4d0000;
                }
            }
        }
        oc745_return_nil(c, 15);
        Ok(())
    });
}

/// `LunoVMRegister WScriptFn_SetHudHeightTarget(LunoVMRegister* out, int n,
/// Param* params)` (0x304320, `UpdateDisplayHeightFunc`): a script call with
/// one target argument. Resolves the target like `Crowd3dControl` and passes
/// the manager's first word to `WScriptMan_SetHudHeightTarget` (0x30b508)
/// with the target as the hud. Returns nil.
pub fn wscript_fn_set_hud_height_target(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        oc745_save_frame(c);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2708));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x50));
        c.sq(18, s.wrapping_add(0x40));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, s.wrapping_add(0x30));
        c.g[S2][0] = c.g[A2][0];
        c.sq(16, s.wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        let initialised = c.g[V0][0] != 0;
        let ra = c.g[RA][0];
        c.sd(s.wrapping_add(0x20), ra);
        if !initialised {
            // Static init of the 8-byte argument block at gp+0x2700.
            c.cycles += 7;
            c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2704));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 0x1;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2700), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2708), c.g[A0][0]);
            let s = sp(c);
            c.sb(s, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2700));
        let s = sp(c);
        c.sw(s.wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x0030_4388)?;
        let no_args = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_args {
            let vp = c.g[S0][0];
            let count = c.g[S1][0];
            let kinds = (c.g[GP][0] as u32).wrapping_add(0xc68);
            oc745_fill_args(c, vp, count, kinds, 4, 0x0030_43a0);
        }
        // The manager and the target.
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[A3][0] = c.g[V0][0];
        c.g[V1][0] = !c.g[A2][0];
        let by_id = c.g[V1][0] != 0;
        c.g[T0][0] = c.lw(c.g[V0][0] as u32);
        if !by_id {
            c.cycles += 2;
            c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x290));
        } else {
            c.cycles += 9;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            let has_section = c.g[A0][0] != 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if !has_section {
                c.cycles += 2;
                c.g[A2][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !found {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            }
        }
        c.cycles += 2;
        let no_obj = c.g[A2][0] == 0;
        c.g[A0][0] = c.g[A3][0];
        if !no_obj {
            c.cycles += 2;
            c.g[A1][0] = c.g[T0][0];
            Callee {
                addr: 0x0030_b508,
                f: ee::s10::f_0030b508,
            }
            .call(c, 0x0030_4468)?;
        }
        c.g[V1][0] = 0x4d0000;
        oc745_return_nil(c, 16);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// The result slot every fixture passes as `a0`.
    const OC745_OUT: u32 = DATA + 0x4000;
    /// The script manager (`gp+0xce8`), the world cache (`gp+0x16c8`) and its
    /// section and tag tables.
    const OC745_MGR: u32 = DATA + 0x1400;
    const OC745_WC: u32 = DATA + 0x1800;
    const OC745_WC_X: u32 = DATA + 0x1810;
    const OC745_SECT: u32 = DATA + 0x1820;
    const OC745_SEC0: u32 = DATA + 0x1830;
    const OC745_TAGS: u32 = DATA + 0x1860;
    /// The instance records the tags point at.
    const OC745_OBJ: u32 = DATA + 0x2400;
    /// The argument list (16-byte entries).
    const OC745_PARAMS: u32 = DATA + 0x2000;
    /// The fakes' gate word in the machine's RAM.
    const OC745_CTRL: u32 = DATA + 0x3_0000;
    const OC745_MAGIC: u32 = 0x0c74_5c0d;
    /// The crowd function's static block.
    const OC745_STATIC: u32 = 0x4f_bc70;

    macro_rules! oc745_fakes {
        ($($name:ident $slot:ident $gen:ident $addr:literal;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC745_CTRL) != OC745_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::s11::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra)
                }
            )*
            fn oc745_install_fakes() {
                $(
                    fakes::hook(addr_of_mut!(ee::s11::$slot), $name, fakes::prev_cell($name));
                )*
            }
        };
    }

    oc745_fakes! {
        oc745_fake_add_instance OVERRIDE_00343C60 f_00343c60__generated 0x0034_3c60;
        oc745_fake_remove_instance OVERRIDE_00343F38 f_00343f38__generated 0x0034_3f38;
    }

    /// The manager, the world-cache chain and the instance records. Ids are
    /// `section | tag_index << 8`; sections 0..3, tags 0..3.
    fn oc745_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(GAME_GP + 0xce8, OC745_MGR);
        fill(c, r, OC745_MGR, 0x300);
        let inst = if seed % 7 == 0 { 0 } else { OC745_OBJ };
        c.bus.write32(OC745_MGR + 0x290, inst);
        fill(c, r, OC745_OBJ, 0x100);
        c.bus.write32(GAME_GP + 0x16c8, OC745_WC);
        c.bus.write32(OC745_WC, OC745_WC_X);
        c.bus.write32(OC745_WC_X + 8, OC745_SECT);
        for sec in 0..3u32 {
            let present = (seed / 3 + sec) % 4 != 0;
            c.bus.write32(
                OC745_SECT + 4 * sec,
                if present { OC745_SEC0 + 0x10 * sec } else { 0 },
            );
            c.bus
                .write32(OC745_SEC0 + 0x10 * sec + 0x1c, OC745_TAGS + 0x10 * sec);
            for t in 0..3u32 {
                let k = (seed + sec + t) % 5;
                let tag = if k == 0 {
                    r.next_u32() & 0xff
                } else {
                    (((OC745_OBJ + 4 * k) / 4) << 8) | (r.next_u32() & 0xff)
                };
                c.bus.write32(OC745_TAGS + 0x10 * sec + 4 * t, tag);
            }
        }
        c.bus.write32(GAME_GP + 0xf00, DATA + 0x5000);
    }

    /// A target id: the manager's own (-1) or `section | tag << 8`.
    fn oc745_id(r: &mut Rng, seed: u32) -> u32 {
        if seed % 4 == 0 {
            u32::MAX
        } else {
            (r.next_u32() % 3) | ((r.next_u32() % 3) << 8)
        }
    }

    /// The kind table (index 2 is a float) and `count` arguments: index
    /// `j % 3` with an id, a command or a random int, and a random kind.
    fn oc745_args(c: &mut Cpu, r: &mut Rng, seed: u32, kinds: u32, count: u32) {
        c.g[A1][0] = count as u64;
        c.g[A2][0] = sx(OC745_PARAMS);
        c.bus.write32(GAME_GP + kinds, r.next_u32() % 2);
        c.bus.write32(GAME_GP + kinds + 4, r.next_u32() % 2);
        c.bus.write32(GAME_GP + kinds + 8, 2);
        for j in 0..count {
            let e = OC745_PARAMS + 16 * j;
            let idx = (j + seed / 5) % 3;
            let val = match idx {
                0 => oc745_id(r, seed / 2 + j),
                1 => r.next_u32() % 3,
                _ => r.next_u32(),
            };
            c.bus.write32(e, idx);
            c.bus.write32(e + 0x4, val);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
    }

    /// `WScriptFn_Crowd3dControl`: the static block, the target lookup and
    /// the add/remove command (calls to the crowd manager are fakes).
    #[test]
    fn wscript_fn_crowd3d_control_matches() {
        oc745_install_fakes();
        check(
            ee::s10::f_003039f0__generated,
            wscript_fn_crowd3d_control,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39f0, 2);
                c.pc = 0x0030_39f0;
                c.bus.write32(OC745_CTRL, OC745_MAGIC);
                c.g[A0][0] = sx(OC745_OUT);
                oc745_world(c, &mut r, seed);
                let initialised = seed % 2 == 1;
                c.bus.write32(GAME_GP + 0x26ec, initialised as u32);
                c.bus.write32(OC745_STATIC, oc745_id(&mut r, seed / 2));
                c.bus.write32(OC745_STATIC + 4, (seed / 3) % 3);
                c.bus.write32(OC745_STATIC + 8, r.next_u32());
                oc745_args(c, &mut r, seed, 0xc48, seed % 4);
            },
        );
    }

    /// `WScriptFn_SetHudHeightTarget`: the static block, the target lookup
    /// and the real `WScriptMan_SetHudHeightTarget` call.
    #[test]
    fn wscript_fn_set_hud_height_target_matches() {
        check(
            ee::s10::f_00304320__generated,
            wscript_fn_set_hud_height_target,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x4320, 0);
                c.pc = 0x0030_4320;
                c.g[A0][0] = sx(OC745_OUT);
                oc745_world(c, &mut r, seed);
                let initialised = seed % 2 == 1;
                c.bus.write32(GAME_GP + 0x2708, initialised as u32);
                c.bus.write32(GAME_GP + 0x2700, oc745_id(&mut r, seed / 2));
                c.bus.write32(GAME_GP + 0x2704, r.next_u32());
                oc745_args(c, &mut r, seed, 0xc68, seed % 4);
            },
        );
    }
}
