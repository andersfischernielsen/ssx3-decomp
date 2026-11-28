//! Staging module oc1491 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1491`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_6a90,
    wscript_spawn_multi_particle,
    "WScriptFn_SpawnMultiParticle",
)];

/// Offsets of `defaults` (the guarded static record at `0x4fbf68`) that hold 0.0.
const MPS_ZERO_F: &[u32] = &[
    0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50,
    0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x88, 0x8c, 0x90, 0xa8,
    0xac, 0xb0, 0xb8, 0xbc, 0xc0, 0xcc,
];

/// Offsets of the static record that hold 1.0.
const MPS_ONE_F: &[u32] = &[0xc, 0x84, 0x94, 0x98, 0x9c, 0xa0, 0xa4, 0xb4];

/// `WScriptFn_SpawnMultiParticle(Nil* result, int n, Ent* params)` (0x306a90):
/// script function that spawns a multi-particle. On its first call it fills the
/// default particle record (static at 0x4fbf68, guard word at `gp+0x27c4`); it
/// then copies that record (228 bytes) onto the stack, applies the `n`
/// 16-byte parameter entries (`idx`, `val`, `type` at +0xc; an int value whose
/// declared type in the table at 0x446890 is float (2) is converted), calls
/// `MultiParticleMan_CreateSlot` and copies the 12-byte nil result to `a0`.
pub fn wscript_spawn_multi_particle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x27c4));
        frame.save(c, S1, 0x130);
        c.g[A3][0] = 0x500000;
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x110);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x140);
        c.g[S3][0] = c.g[A2][0];
        let initialised = c.g[V0][0] != 0u64;
        frame.save_ra(c, 0x100);
        if !initialised {
            c.cycles += 72;
            let rec = 0x500000u32.wrapping_sub(0x4098);
            c.g[AT][0] = 0x40800000;
            c.mtc1(1, 1);
            c.g[V0][0] = sx(rec);
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 2);
            c.g[V1][0] = 0x1;
            c.mtc1(0, 0);
            c.g[A1][0] = 0x10;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 3);
            c.g[A2][0] = 0x80;
            c.swc1(1, rec + 0x10);
            c.swc1(2, rec + 0x8);
            c.sw(rec, c.g[V1][0]);
            c.sw(rec + 0x4, 0u64);
            for &o in MPS_ONE_F {
                c.swc1(3, rec + o);
            }
            for &o in MPS_ZERO_F {
                c.swc1(0, rec + o);
            }
            c.g[AT][0] = 0x41a00000;
            c.mtc1(1, 1);
            c.g[A0][0] = c.lbu(rec + 0xe0);
            c.sw(rec + 0xc4, c.g[A1][0]);
            c.sw(rec + 0xdc, c.g[A2][0]);
            c.swc1(1, rec + 0xd4);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x27c4), c.g[V1][0]);
            c.sw(rec + 0xc8, 0u64);
            c.sw(rec + 0xd0, c.g[V1][0]);
            c.sw(rec + 0xd8, 0u64);
            c.sb(sp(c), c.g[A0][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0xe4;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4098));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e6574, 0x00306bf4)?;
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6890));
            c.g[A3][0] = 0x2;
            'mps_args: loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] != c.g[V0][0] {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        // int value declared as float: convert it
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        c.cycles += 3;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        let more = c.g[A1][0] != 0u64;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x00306c10);
                            }
                            continue 'mps_args;
                        }
                        break 'mps_args;
                    }
                }
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.cycles += 1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x00306c10);
                    }
                    continue 'mps_args;
                }
                break 'mps_args;
            }
        }
        c.cycles += 21;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xf38));
        c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xdc));
        call_ptr(c, 0x00357a78, 0x00306c6c)?;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S2][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S2][0] as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    const OC1491_GATE: u32 = DATA + 0x2c_0000;
    const OC1491_MAGIC: u32 = 0x1491_0001;

    fn oc1491_no_prev(_c: &mut Cpu) {
        unreachable!("oc1491 fake called before it was hooked");
    }

    /// Gated fake of `MultiParticleMan_CreateSlot` (0x357a78): logs and returns.
    fn oc1491_fake_create_slot(c: &mut Cpu) {
        if c.bus.read32(OC1491_GATE) != OC1491_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1491_fake_create_slot),
                oc1491_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0035_7a78, ra);
    }

    /// `WScriptFn_SpawnMultiParticle`: both init-guard states, zero and several
    /// entries, entries whose type matches, an int entry for a float field
    /// (converted) and a mismatch for a non-float field (copied).
    #[test]
    fn wscript_spawn_multi_particle_matches() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00357A78),
            oc1491_fake_create_slot,
            fakes::prev_cell(oc1491_fake_create_slot),
        );
        fakes::covered(
            ee::s10::f_00306a90__generated,
            wscript_spawn_multi_particle,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x306a90, 2);
                c.pc = 0x0030_6a90;
                c.bus.write32(OC1491_GATE, OC1491_MAGIC);
                c.bus
                    .write32(GAME_GP + 0x27c4, if seed % 2 == 0 { 0 } else { 1 });
                // declared types of the 57 fields: 1 = int, 2 = float, 3 = other
                for i in 0..57u32 {
                    c.bus.write32(0x0044_6890 + 4 * i, 1 + r.next_u32() % 3);
                }
                let n: i32 = match seed % 7 {
                    0 => 0,
                    1 => -1,
                    _ => 1 + (r.next_u32() % 8) as i32,
                };
                let params = DATA + 0x1000;
                for i in 0..8u32 {
                    let idx = r.next_u32() % 57;
                    let decl = c.bus.read32(0x0044_6890 + 4 * idx);
                    let ty = match r.next_u32() % 3 {
                        0 => decl,
                        1 => 1 + (decl % 3),
                        _ => 2,
                    };
                    c.bus.write32(params + 16 * i, idx);
                    c.bus.write32(params + 16 * i + 4, r.next_u32() % 1000);
                    c.bus.write32(params + 16 * i + 0xc, ty);
                }
                for o in 0..4u32 {
                    c.bus.write32(0x004c_9098 + 4 * o, r.next_u32());
                }
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(n as u32);
                c.g[A2][0] = sx(params);
            },
        );
    }
}
