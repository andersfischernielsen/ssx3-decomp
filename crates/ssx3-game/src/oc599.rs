//! Staging module oc599 (wave l588.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc599`.

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
        0x0035_8b28,
        rail_man_set_rail_enabled,
        "RailMan_SetRailEnabled",
    ),
    (0x0035_8c80, anim_combo_ctor, "AnimCombo_ctor"),
];

/// `cRailManNode::ActiveNode`: the global rail manager pointer, at gp+0xf50.
pub(crate) const RAIL_ACTIVE_NODE_GP: u32 = 0xf50;
/// `cRailManNode::RailInfo[128]`: one 4-byte RailRID per slot, from +0x18.
pub(crate) const RAIL_INFO: u32 = 0x18;
/// Slots in `RailInfo`.
pub(crate) const RAIL_SLOTS: u64 = 128;

/// `void cRailManNode::addRail(tResourceID key, int add)` (0x358b28, the
/// reference's `cRailManNode_addRail`). The manager is created on first use
/// (`cMemMan_alloc` of 0x218 bytes, then its constructor). The slot whose
/// RailRID is `key`, or else the first free slot (RailRID -1), gets `key`
/// and is enabled when `add != 0`, disabled otherwise. Nothing happens when
/// no slot is found, or when the slot already holds the same state.
pub fn rail_man_set_rail_enabled(c: &mut Cpu) {
    guest(c, |c| {
        // S0 = add, S1 = key (callee-saved, kept in the registers).
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let active = Addr((c.g[GP][0] as u32).wrapping_add(RAIL_ACTIVE_NODE_GP)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        if active == 0 {
            // cRailManNode_ActiveNode = new(0x218) cRailManNode(1)
            c.cycles += 9;
            c.g[A1][0] = 0x004a_4048; // "RailMan" name
            c.g[A0][0] = 0x218;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0035_8b60)?; // cMemMan_alloc
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            Callee {
                addr: 0x0035_84f0,
                f: ee::s11::f_003584f0,
            }
            .call(c, 0x0035_8b6c)?; // cRailManNode_cRailManNode1
        }

        // Scan for the key (and the first free slot) in RailInfo.
        c.cycles += 9;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(RAIL_ACTIVE_NODE_GP)).lw(c);
        c.g[A3][0] = u64::MAX; // 0xffff_ffff_ffff_0000 | 0xffff: all ones
        c.g[A1][0] = u64::MAX; // idx = -1
        c.g[A2][0] = 0; // i
        c.g[T1][0] = u64::MAX; // -1
        c.g[T0][0] = c.g[S1][0]; // key
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(RAIL_INFO)); // &RailInfo[i]
        'scan: loop {
            c.cycles += 2;
            let rid = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = rid;
            if c.g[A1][0] == c.g[T1][0] {
                // no slot yet: a free slot (RailRID -1) is the candidate
                c.cycles += 3;
                if (rid ^ c.g[A3][0]) == 0 {
                    c.g[A1][0] = c.g[A2][0];
                }
            }
            c.cycles += 2;
            if rid == c.g[T0][0] {
                c.g[A1][0] = c.g[A2][0];
                break 'scan;
            }
            c.cycles += 4;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A2][0] as i64) < RAIL_SLOTS as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break 'scan;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8b90);
            }
        }

        'c1c: {
            'c18: {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                let no_slot = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                if no_slot {
                    break 'c18;
                }
                'bf8: {
                    'bec: {
                        c.cycles += 5;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(RAIL_INFO).lw(c);
                        if c.g[V1][0] != c.g[S1][0] {
                            break 'bec;
                        }
                        // the slot already holds the key
                        c.cycles += 2;
                        let add_zero = c.g[S0][0] == 0;
                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        if add_zero {
                            break 'c18;
                        }
                        c.cycles += 2;
                        break 'bf8;
                    }
                    // another key is in the slot
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        break 'c1c;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                }
                // store the key, then enable (add != 0) or disable
                c.cycles += 2;
                let add_zero = c.g[S0][0] == 0;
                let key = c.g[S1][0];
                Addr(c.g[V0][0] as u32).off(RAIL_INFO).sw(c, key);
                if !add_zero {
                    c.cycles += 4;
                    Callee {
                        addr: 0x0035_8998,
                        f: ee::s11::f_00358998,
                    }
                    .call(c, 0x0035_8c08)?; // cRailManNode_enableRail
                    frame.restore(c, S0, 0x20);
                    break 'c1c;
                }
                c.cycles += 2;
                Callee {
                    addr: 0x0035_8780,
                    f: ee::s11::f_00358780,
                }
                .call(c, 0x0035_8c18)?; // cRailManNode_disableRail
            }
            // leaving through 'c18 (not through the 'c1c breaks above)
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAnimComboNode::mVtbl` value (`cAnimComboNode_vtbl`).
pub(crate) const ANIM_COMBO_VTABLE: u32 = 0x0048_ec28;

/// `cAnimComboNode_cAnimComboNode1(self, bucket, pInstance, adata)` (0x358c80,
/// `AnimCombo_ctor`): the animation object base (type 3), then the combo's
/// vtable, its rate and life times (the frame times from `adata`, turned into
/// seconds at 30 Hz), the speed per second over the timer rate, the stop flags
/// and the transform array (`NumAnimMeshes` elements of 64 bytes).
pub fn anim_combo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        // S0 = pInstance, S1 = self, S2 = adata
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        frame.save(c, S3, 0x10);
        c.g[A2][0] = 3;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S2][0];
        Callee {
            addr: 0x0034_1aa0,
            f: ee::s11::f_00341aa0,
        }
        .call(c, 0x0035_8cb8)?; // AnimObject_ctor(self+0x18, bucket, 3, pInstance, adata)
        c.g[V0][0] = 0x490000;
        c.mtc1(0, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x13d8));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8)); // &self->FromTime2 (rate)
        let vtable = c.g[V0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x54)).sw(c, vtable); // mVtbl

        // rate = (adata->FromTime2 < 0 ? adata->ToTime : adata->FromTime2) / 30
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x28));
        c.c_lt_s(1, 0);
        let from_neg = c.fcc();
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x80)).lw(c); // pInstance->m_Model
        if from_neg {
            c.cycles += 1;
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 9;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2a7c));
        c.mul_s(1, 1, 0);
        c.swc1(1, (c.g[V1][0] as u32));

        // life = (adata->ToTime2 < 0 ? pModel->m_fAnimTime : adata->ToTime2 / 30)
        c.mtc1(0, 0);
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x2c));
        c.c_lt_s(1, 0);
        let life_neg = c.fcc();
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        if life_neg {
            c.cycles += 2;
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x14));
        } else {
            c.cycles += 2;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2a78));
            c.mul_s(0, 1, 0);
        }

        // AnimSpeed2, Stop, StopInPlace, and the transform array
        c.cycles += 25;
        c.swc1(0, (c.g[V0][0] as u32));
        c.g[A1][0] = 0x490000;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c); // cAppMan_TheAppMan
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15e0));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = 0x2000_0000;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2a74));
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x10)); // TimerHz (an int)
        c.cvt_s_w(2, 2);
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c); // NumAnimMeshes
        c.g[A3][0] = 0;
        c.mul_s(0, 0, 1);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84)); // &self->ComboXform
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
        c.div_s(0, 0, 2);
        c.swc1(0, (c.g[S1][0] as u32));
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x34)).lw(c); // adata->Stop
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.sb((c.g[S1][0] as u32).wrapping_add(0x16), c.g[V0][0]);
        c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 31);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x17), c.g[V1][0]);
        call_ptr(c, 0x0031_7e30, 0x0035_8d74)?; // operator_new_named(n << 6, ...)
        let none = c.g[S0][0] == 0;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        if !none {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }

        c.cycles += 12;
        let combo = c.g[S3][0] as u32;
        let xform = c.g[V0][0];
        Addr(combo).sw(c, xform); // self->ComboXform = new[n]
        c.g[V0][0] = c.g[S1][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x14), 0);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x15), 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The pools and arena `cMemMan_alloc` needs for the manager (the same
    /// setup as `rbc_test_allocator`).
    fn rail_test_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            for n in 0..16u32 {
                let node = nodes + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, nodes);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
    }

    #[test]
    fn rail_man_set_rail_enabled_matches() {
        check(
            ee::s11::f_00358b28__generated,
            rail_man_set_rail_enabled,
            300,
            &|c, seed| {
                let mut r = Rng(0x5a17_0b28 ^ seed as u64);
                let key = (r.next_u32() % 6) as u64;
                let add = (seed % 3) as u64;
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                let mgr = DATA + 0x1000;
                if seed % 4 == 0 {
                    // no manager yet: the allocator and constructor run
                    c.bus.write32(gp + RAIL_ACTIVE_NODE_GP, 0);
                    rail_test_allocator(c);
                } else {
                    c.bus.write32(gp + RAIL_ACTIVE_NODE_GP, mgr);
                    for i in 0..RAIL_SLOTS as u32 {
                        let v = match seed % 7 {
                            // no key and no free slot: the search runs to the end
                            0 => 100 + r.next_u32() % 50,
                            _ => match r.next_u32() % 4 {
                                0 => u32::MAX,
                                _ => r.next_u32() % 6,
                            },
                        };
                        c.bus.write32(mgr + RAIL_INFO + 4 * i, v);
                    }
                }
                c.g[A0][0] = key;
                c.g[A1][0] = add;
                c.pc = 0x0035_8b28;
            },
        );
    }

    fn put_f(c: &mut Cpu, addr: u32, v: f32) {
        c.bus.write32(addr, v.to_bits());
    }

    #[test]
    fn anim_combo_ctor_matches() {
        check(
            ee::s11::f_00358c80__generated,
            anim_combo_ctor,
            240,
            &|c, seed| {
                let mut r = Rng(0x3c80_0358 ^ seed as u64);
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                // the allocator for the transform array (operator new)
                rail_test_allocator(c);
                let this = DATA + 0x2000;
                let inst = DATA + 0x3000;
                let model = DATA + 0x4000;
                let adata = DATA + 0x3800;
                let timer = DATA + 0x5000;
                let frac = |r: &mut Rng| ((r.next_u32() % 2000) as f32 - 1000.0) * 0.01;
                // float constants of the game (1/30 and a scale), and the timer
                put_f(c, gp.wrapping_sub(0x2a7c), 1.0 / 30.0);
                put_f(c, gp.wrapping_sub(0x2a78), 1.0 / 30.0);
                put_f(c, gp.wrapping_sub(0x2a74), 0.5);
                c.bus.write32(gp + 0x2a74, timer); // cAppMan_TheAppMan
                c.bus.write32(timer + 0x10, 30 + seed % 30); // TimerHz
                c.bus.write32(inst + 0x80, model); // pInstance->m_Model
                put_f(c, model + 0x14, frac(&mut r)); // m_fAnimTime
                let from = frac(&mut r).abs() + 0.25;
                put_f(c, adata + 0x28, if seed % 2 == 0 { from } else { -from }); // FromTime2
                put_f(c, adata + 0x10, frac(&mut r)); // ToTime
                let to2 = frac(&mut r).abs() + 0.25;
                put_f(c, adata + 0x2c, if seed % 3 == 0 { -to2 } else { to2 }); // ToTime2
                put_f(c, adata + 0x30, frac(&mut r)); // Speed2
                let stop = [0u32, 1, u32::MAX, 2][((seed / 3) % 4) as usize];
                c.bus.write32(adata + 0x34, stop); // Stop
                c.bus.write32(this + 0x74, seed % 4); // NumAnimMeshes
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = inst as u64;
                c.g[A3][0] = adata as u64;
                c.pc = 0x0035_8c80;
            },
        );
    }
}
