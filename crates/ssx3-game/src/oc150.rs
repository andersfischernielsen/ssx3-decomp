//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Camera trigger bookkeeping: `CameraTriggerMan_UpdateForRider` refreshes the
//! camera-trigger octree for the rider and applies the triggers that became
//! active (push/apply) or inactive (pop/apply, then unlink).

use crate::abi::r32;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0016_d320,
    camera_trigger_man_update_for_rider,
    "CameraTriggerMan_UpdateForRider",
)];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const CLEAR_FLAGS: Callee = Callee {
        addr: 0x0016_c678,
        f: ee::s02::f_0016c678,
    };
    pub const FIND: Callee = Callee {
        addr: 0x0016_c6b0,
        f: ee::s02::f_0016c6b0,
    };
    pub const ADD: Callee = Callee {
        addr: 0x0016_c718,
        f: ee::s02::f_0016c718,
    };
    pub const FIND_BY_ID: Callee = Callee {
        addr: 0x0016_c6e0,
        f: ee::s02::f_0016c6e0,
    };
    pub const REMOVE: Callee = Callee {
        addr: 0x0016_c778,
        f: ee::s02::f_0016c778,
    };
    pub const POP_TRIGGER: Callee = Callee {
        addr: 0x0016_d260,
        f: ee::s02::f_0016d260,
    };
    pub const PUSH_TRIGGER: Callee = Callee {
        addr: 0x0016_d238,
        f: ee::s02::f_0016d238,
    };
    pub const APPLY_ACTION: Callee = Callee {
        addr: 0x0016_e1d8,
        f: ee::s02::f_0016e1d8,
    };
    pub const COLLECT_NODE: Callee = Callee {
        addr: 0x0017_6890,
        f: ee::s02::f_00176890,
    };
    pub const COLLECT_BOX: Callee = Callee {
        addr: 0x0017_4848,
        f: ee::s02::f_00174848,
    };
    pub const CLASSIFY_AABB: u32 = 0x0032_8360;
}

// ---------------------------------------------------------------------------
// Small helpers.

#[inline]
fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// Where a value comes from at an instruction of the original: a register,
/// a word on the frame (`lw(off(sp))`), or a constant.
#[derive(Clone, Copy)]
enum Src {
    Reg(usize),
    Stack(u32),
    Const(u64),
}

impl Src {
    fn get(self, c: &mut Cpu) -> u64 {
        match self {
            Src::Reg(r) => c.g[r][0],
            Src::Stack(off) => Addr((c.g[SP][0] as u32).wrapping_add(off)).lw(c),
            Src::Const(v) => v,
        }
    }
}

/// What a pass leaves behind when it does not process its list.
#[derive(Clone, Copy)]
enum End {
    /// `a0 = *(sp + 0x44)` (trigger count).
    Count,
    /// `v0 = *(s6 + off)` (next category's flag).
    Field(u32),
    /// `v1 = *(sp + 0x48)` (count * 4).
    Stack48,
}

fn apply_end(c: &mut Cpu, end: End) {
    match end {
        End::Count => c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c),
        End::Field(off) => c.g[V0][0] = Addr(r32(c, S6).wrapping_add(off)).lw(c),
        End::Stack48 => c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c),
    }
}

/// One category pass of `CameraTriggerMan_UpdateForRider`: test the
/// category's spatial key against the rider box, walk the trigger list
/// marking matching nodes (`*(node + 0xc) = marker`), then collect the
/// octree node's eight child trigger lists.
///
/// The original keeps its state in s0..s7/a0..a3/v0/v1; this helper does the
/// same (it is passed the descriptor and reads/writes those registers) so
/// the callees see exactly the registers the generated body leaves for
/// them, and so the event back edge can be delivered at `resume_pc`.
#[allow(clippy::too_many_arguments)]
fn category_pass(
    c: &mut Cpu,
    presence_gate: bool,
    desc: Src,
    s0: Option<u32>,
    s1: Option<u32>,
    gate_cycles: u64,
    gate_ret: u32,
    box_a0: Src,
    box_t1: Src,
    child_rets: &[u32; 8],
    marker_reg: usize,
    type_reg: usize,
    loop_cycles: u64,
    resume_pc: u32,
    end: End,
    box_ret: u32,
    vtable_rets: [u32; 4],
    setup: Option<(usize, u64)>,
    setup_cycles: u64,
) -> Flow {
    // The first category tests the list head itself rather than a flag.
    if presence_gate {
        let d = desc.get(c);
        c.g[S5][0] = Addr((d as u32).wrapping_add(0x10)).lw(c);
        if c.g[S5][0] == 0 {
            c.cycles += 1;
            apply_end(c, end);
            return Ok(());
        }
    }

    c.cycles += gate_cycles;
    if let Some(off) = s0 {
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(off));
    }
    if let Some(off) = s1 {
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(off));
    }
    let d = desc.get(c);
    if !presence_gate {
        c.g[S5][0] = Addr((d as u32).wrapping_add(0x10)).lw(c);
    }
    c.g[A0][0] = d;
    c.g[RA][0] = sx(gate_ret);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    call_ptr(c, callee::CLASSIFY_AABB, gate_ret)?;

    // 1 = outside, 2 = overlapping/partial, 0 = inside.
    c.g[V1][0] = c.g[V0][0];
    let nonzero = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if nonzero {
        c.cycles += 2;
        let differs = c.g[V1][0] != c.g[V0][0];
        apply_end(c, end);
        if !differs {
            c.cycles += 7;
            let a0 = box_a0.get(c);
            let t1 = box_t1.get(c);
            c.g[A0][0] = a0;
            c.g[T0][0] = c.g[S5][0];
            c.g[T1][0] = t1;
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[RA][0] = sx(box_ret);
            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            callee::COLLECT_BOX.call(c, box_ret)?;
            // `break` to the block after the box call: one cycle and the
            // category's end value (A0/V0/V1).
            c.cycles += 1;
            apply_end(c, end);
        }
        return Ok(());
    }

    // The list of trigger objects attached to this octree node.
    c.cycles += 3;
    c.g[S2][0] = Addr(r32(c, S5).wrapping_add(0x28)).lw(c);
    if c.g[S2][0] == 0 {
        c.g[A0][0] = Addr(r32(c, S5)).lw(c);
    } else {
        c.cycles += setup_cycles;
        if let Some((reg, val)) = setup {
            c.g[reg][0] = val;
        }
        if marker_reg == S6 {
            // The last category compares against 4 in s7 and marks with s6.
            c.g[S7][0] = 4;
        }
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
        loop {
            c.cycles += loop_cycles;
            c.g[type_reg][0] = 4;
            if c.g[V0][0] != c.g[type_reg][0] {
                c.g[S2][0] = Addr(r32(c, S2)).lw(c);
            } else {
                c.cycles += 18;
                c.g[S3][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(4)).lw(c);
                c.g[S0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
                c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x6c0)).lw(c);
                c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x28)).lh(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x38)).lh(c);
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x2c)).lw(c);
                c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
                let target = r32(c, V0);
                c.g[RA][0] = sx(vtable_rets[0]);
                c.g[S0][0] = add32(c.g[S0][0], c.g[A1][0]);
                call_ptr(c, target, vtable_rets[0])?;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x3c)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let target = r32(c, V1);
                c.g[RA][0] = sx(vtable_rets[1]);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, vtable_rets[1])?;
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = Addr(r32(c, S2)).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(r32(c, S4)).lw(c);
                    c.g[RA][0] = sx(vtable_rets[2]);
                    c.g[A1][0] = c.g[S3][0];
                    callee::FIND.call(c, vtable_rets[2])?;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = Addr(r32(c, S4)).lw(c);
                        c.cycles += 2;
                        c.g[RA][0] = sx(vtable_rets[3]);
                        c.g[A1][0] = c.g[S3][0];
                        callee::ADD.call(c, vtable_rets[3])?;
                    } else {
                        c.cycles += 2;
                        c.sw(r32(c, V0).wrapping_add(0xc), c.g[marker_reg][0]);
                    }
                    c.cycles += 1;
                    c.g[S2][0] = Addr(r32(c, S2)).lw(c);
                }
            }
            c.cycles += 2;
            if c.g[S2][0] != 0 {
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, resume_pc);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S5)).lw(c);
            break;
        }
    }

    // The node's eight child octree entries.
    for (i, ret) in child_rets.iter().enumerate() {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += if i == 7 { 4 } else { 3 };
            c.g[RA][0] = sx(*ret);
            c.g[A1][0] = c.g[S4][0];
            callee::COLLECT_NODE.call(c, *ret)?;
        }
        if i < 7 {
            c.g[A0][0] = Addr(r32(c, S5).wrapping_add(4 * (i as u32 + 1))).lw(c);
        }
    }
    apply_end(c, end);
    Ok(())
}

// ---------------------------------------------------------------------------
// CameraTriggerMan_UpdateForRider.

/// `void CameraTriggerMan_UpdateForRider(CameraTriggerMan*, Rider*)`
/// (0x16d320): clear the trigger flags for this rider's slot, classify the
/// rider's bounding box against each of the eight camera-trigger categories,
/// mark the triggers that contain it, collect the octree child lists, then
/// pop/apply the triggers the rider just left and push/apply the ones it
/// entered.
pub fn camera_trigger_man_update_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x100);
        frame.save(c, FP, 0x70);
        frame.save(c, S0, 0xf0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save_ra(c, 0x60);
        c.sw(frame.at(c, 0x40), c.g[A0][0]);

        c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x874)).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !skip {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x87c)).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }

        c.cycles += 2;
        let inactive = c.g[V1][0] == 0;
        c.g[S1][0] = sx(r32(c, FP).wrapping_add(0x6c0));
        if inactive {
            c.cycles += 12;
            frame.restore(c, S0, 0xf0);
            frame.restore(c, S1, 0xe0);
            frame.restore(c, S2, 0xd0);
            frame.restore(c, S3, 0xc0);
            frame.restore(c, S4, 0xb0);
            frame.restore(c, S5, 0xa0);
            frame.restore(c, S6, 0x90);
            frame.restore(c, S7, 0x80);
            frame.restore(c, FP, 0x70);
            frame.restore_ra(c, 0x60);
            frame.close(c);
            return Ok(());
        }

        // Clear this rider's flag words: this + 0x14 + count * 4.
        c.cycles += 33;
        c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x870)).lw(c);
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[S0][0] = sx((r32(c, V0)) << 2);
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x14));
        c.g[RA][0] = sx(0x0016_d38c);
        c.g[A0][0] = add32(c.g[V1][0], c.g[A0][0]);
        callee::CLEAR_FLAGS.call(c, 0x0016_d38c)?;

        // CameraTriggerMan from world->game->trigger manager.
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[S0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.g[A1][0] = Addr(r32(c, FP).wrapping_add(0x6c0)).lw(c);
        c.g[S6][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x14));
        c.sw(frame.at(c, 0), c.g[S0][0]);
        c.sw(frame.at(c, 4), c.g[FP][0]);

        // Two virtual calls on the rider's pose object (the two quadwords
        // they return are the rider's bounding box, copied to the frame).
        c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x2c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0016_d3c0);
        c.g[A0][0] = add32(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0016_d3c0)?;
        c.g[V1][0] = Addr(r32(c, FP).wrapping_add(0x6c0)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0016_d3d8);
        c.g[A0][0] = add32(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0016_d3d8)?;
        c.lq(3, r32(c, S0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(4, r32(c, V0));
        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S5][0] = Addr(r32(c, S6).wrapping_add(0x10)).lw(c);
        let no_head = c.g[S5][0] == 0;
        c.g[S4][0] = c.g[SP][0];

        // Category 0: the manager's first list (no flag; the head is the
        // test), its box collected with the manager as the second key.
        if !no_head {
            category_pass(
                c,
                true,
                Src::Reg(S6),
                Some(0x30),
                None,
                7,
                0x0016_d404,
                Src::Reg(S0),
                Src::Reg(S6),
                &[
                    0x0016_d4c0,
                    0x0016_d4d4,
                    0x0016_d4e8,
                    0x0016_d4fc,
                    0x0016_d510,
                    0x0016_d524,
                    0x0016_d538,
                    0x0016_d54c,
                ],
                S7,
                V1,
                3,
                0x0016_d428,
                End::Count,
                0x0016_d578,
                [0x0016_d464, 0x0016_d474, 0x0016_d488, 0x0016_d4a0],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        }

        // Cache the remaining category descriptors and flag fields.
        c.cycles += 16;
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x3c));
        c.g[V0][0] = Addr(r32(c, S6).wrapping_add(0x24)).lw(c);
        c.g[S0][0] = sx(r32(c, S6).wrapping_add(0x14));
        c.g[A0][0] = sx(r32(c, A0) << 2);
        c.sw(frame.at(c, 0x4c), c.g[V1][0]);
        c.sw(frame.at(c, 0x48), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x64));
        c.g[A0][0] = sx(r32(c, S6).wrapping_add(0x50));
        c.sw(frame.at(c, 0x54), c.g[V1][0]);
        c.sw(frame.at(c, 0x50), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x8c));
        c.g[A0][0] = sx(r32(c, S6).wrapping_add(0x78));
        c.g[S7][0] = sx(r32(c, S6).wrapping_add(0x28));
        c.sw(frame.at(c, 0x58), c.g[A0][0]);
        let skip = c.g[V0][0] == 0;
        c.sw(frame.at(c, 0x5c), c.g[V1][0]);
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(S0),
                None,
                Some(0x31),
                8,
                0x0016_d5d0,
                Src::Reg(S1),
                Src::Reg(S0),
                &[
                    0x0016_d68c,
                    0x0016_d6a0,
                    0x0016_d6b4,
                    0x0016_d6c8,
                    0x0016_d6dc,
                    0x0016_d6f0,
                    0x0016_d704,
                    0x0016_d718,
                ],
                V1,
                A0,
                3,
                0x0016_d5f0,
                End::Field(0x38),
                0x0016_d744,
                [0x0016_d62c, 0x0016_d63c, 0x0016_d650, 0x0016_d66c],
                None,
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x38));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x32));
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(S7),
                None,
                None,
                7,
                0x0016_d760,
                Src::Reg(S0),
                Src::Reg(S7),
                &[
                    0x0016_d81c,
                    0x0016_d830,
                    0x0016_d844,
                    0x0016_d858,
                    0x0016_d86c,
                    0x0016_d880,
                    0x0016_d894,
                    0x0016_d8a4,
                ],
                S7,
                A0,
                3,
                0x0016_d780,
                End::Field(0x4c),
                0x0016_d8d0,
                [0x0016_d7bc, 0x0016_d7cc, 0x0016_d7e0, 0x0016_d7f8],
                Some((S7, 1)),
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x4c));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4c)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(V0),
                Some(0x33),
                None,
                8,
                0x0016_d8f0,
                Src::Reg(S0),
                Src::Stack(0x4c),
                &[
                    0x0016_d9a8,
                    0x0016_d9bc,
                    0x0016_d9d0,
                    0x0016_d9e4,
                    0x0016_d9f8,
                    0x0016_da0c,
                    0x0016_da20,
                    0x0016_da34,
                ],
                S7,
                V1,
                3,
                0x0016_d910,
                End::Field(0x60),
                0x0016_da60,
                [0x0016_d94c, 0x0016_d95c, 0x0016_d970, 0x0016_d988],
                Some((S7, 1)),
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x60));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x50)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(A0),
                Some(0x34),
                None,
                7,
                0x0016_da7c,
                Src::Reg(S0),
                Src::Stack(0x50),
                &[
                    0x0016_db38,
                    0x0016_db4c,
                    0x0016_db60,
                    0x0016_db74,
                    0x0016_db88,
                    0x0016_db9c,
                    0x0016_dbb0,
                    0x0016_dbc4,
                ],
                S7,
                V1,
                3,
                0x0016_daa0,
                End::Field(0x74),
                0x0016_dbf0,
                [0x0016_dadc, 0x0016_daec, 0x0016_db00, 0x0016_db18],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x74));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x54)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(A0),
                Some(0x35),
                None,
                7,
                0x0016_dc0c,
                Src::Reg(S0),
                Src::Stack(0x54),
                &[
                    0x0016_dcc8,
                    0x0016_dcdc,
                    0x0016_dcf0,
                    0x0016_dd04,
                    0x0016_dd18,
                    0x0016_dd2c,
                    0x0016_dd40,
                    0x0016_dd54,
                ],
                S7,
                V1,
                3,
                0x0016_dc30,
                End::Field(0x88),
                0x0016_dd80,
                [0x0016_dc6c, 0x0016_dc7c, 0x0016_dc90, 0x0016_dca8],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x88));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x58)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(A0),
                Some(0x36),
                None,
                7,
                0x0016_dd9c,
                Src::Reg(S0),
                Src::Stack(0x58),
                &[
                    0x0016_de58,
                    0x0016_de6c,
                    0x0016_de80,
                    0x0016_de94,
                    0x0016_dea8,
                    0x0016_debc,
                    0x0016_ded0,
                    0x0016_dee4,
                ],
                S7,
                V1,
                3,
                0x0016_ddc0,
                End::Field(0x9c),
                0x0016_df10,
                [0x0016_ddfc, 0x0016_de0c, 0x0016_de20, 0x0016_de38],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x9c));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x5c)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src::Reg(A0),
                Some(0x37),
                None,
                7,
                0x0016_df2c,
                Src::Reg(S0),
                Src::Stack(0x5c),
                &[
                    0x0016_dfe8,
                    0x0016_dff8,
                    0x0016_e00c,
                    0x0016_e020,
                    0x0016_e034,
                    0x0016_e048,
                    0x0016_e05c,
                    0x0016_e070,
                ],
                S6,
                S7,
                2,
                0x0016_df50,
                End::Stack48,
                0x0016_e09c,
                [0x0016_df88, 0x0016_df98, 0x0016_dfac, 0x0016_dfc4],
                Some((S6, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Stack48);
        }

        // Triggers that just became inactive: pop from the stack box, apply
        // the action, or apply the matching trigger's action, then unlink.
        c.cycles += 5;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) << 5);
            c.g[S2][0] = sx(r32(c, A0).wrapping_add(0x14));
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x1c));
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            loop {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
                } else {
                    c.cycles += 9;
                    c.g[S1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c));
                    c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                    c.g[RA][0] = sx(0x0016_e0ec);
                    c.g[A0][0] = add32(c.g[V0][0], c.g[S3][0]);
                    callee::POP_TRIGGER.call(c, 0x0016_e0ec)?;
                    let popped = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x38)).lw(c);
                    if !popped {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[RA][0] = sx(0x0016_e100);
                        c.g[A1][0] = c.g[FP][0];
                        callee::APPLY_ACTION.call(c, 0x0016_e100)?;
                        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    } else {
                        c.cycles += 2;
                        let no_entry = c.g[V0][0] == 0;
                        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                        if !no_entry {
                            c.cycles += 5;
                            c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x3c)).lw(c);
                            c.g[RA][0] = sx(0x0016_e11c);
                            c.g[A0][0] = add32(c.g[V1][0], c.g[S2][0]);
                            callee::FIND_BY_ID.call(c, 0x0016_e11c)?;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                                c.g[RA][0] = sx(0x0016_e130);
                                c.g[A1][0] = c.g[FP][0];
                                callee::APPLY_ACTION.call(c, 0x0016_e130)?;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0016_e140);
                    c.g[A0][0] = add32(c.g[V0][0], c.g[S2][0]);
                    callee::REMOVE.call(c, 0x0016_e140)?;
                    c.g[S0][0] = c.g[V0][0];
                }
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_e0c8);
                    }
                    continue;
                }
                break;
            }
        }

        // Triggers that just became active: push the stack box and apply the
        // trigger's action.
        c.cycles += 6;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[S1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let empty = c.g[S1][0] == 0;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V1) << 5);
            c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x1c));
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            loop {
                c.cycles += 2;
                let no_entry = c.g[V0][0] == 0;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                if !no_entry {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[RA][0] = sx(0x0016_e188);
                    c.g[A0][0] = add32(c.g[V1][0], c.g[S0][0]);
                    callee::PUSH_TRIGGER.call(c, 0x0016_e188)?;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0016_e198);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                    callee::APPLY_ACTION.call(c, 0x0016_e198)?;
                }
                c.cycles += 3;
                c.g[S1][0] = Addr(r32(c, S1).wrapping_add(4)).lw(c);
                if c.g[S1][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_e170);
                    }
                    continue;
                }
                break;
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, LOG_RESULTS};
    use crate::difftest::{compare, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    /// Hard, caller-saved (`soft`) and LO/HI must all match.
    fn strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    fn w16(c: &mut Cpu, at: u32, v: u16) {
        c.bus.write16(at, v);
    }

    /// Rider at DATA+0x14000, its pose vtable object at +0x15c0, camera
    /// trigger manager at DATA+0x12000 (two spatial-key categories), the
    /// rider's trigger list at DATA+0x13014, and the two virtual pose calls
    /// forced to return the rider box at DATA+0x16000 / +0x17000.
    ///
    /// The box is min=max=(0, 2048, 0), so a category whose cell is
    /// [-2048, 2048]^3 classifies as 0 (inside; exercises the list walk) and
    /// a far-away cell as 1 (outside; exercises the skip/tail paths). The
    /// `v0 == 2` (box collection) path is not exercised: it would run the
    /// real octree box walker, whose fixture is much larger than this test's
    /// budget.
    fn setup(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x0016_d320, 0);
        let world = DATA + 0x1_0000;
        let game = world + 0x1000;
        let mgr = DATA + 0x12_000;
        let this = DATA + 0x13_000;
        let rider = DATA + 0x14_000;
        let vt = DATA + 0x15_000;
        let amin = DATA + 0x16_000;
        let amax = DATA + 0x17_000;

        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x11_000); // BELibrary dummy
        w(c, GAME_GP.wrapping_sub(0x848), world);
        w(c, world + 0x84, game);
        w(c, game + 0x20, mgr);

        // Rider slot count and the trigger list inside `this`.
        let count = seed % 3 % 2;
        w(c, this + 0x870, count);
        w(c, this + 0x14, 0);
        w(c, this + 0x18, 0);

        // Entry gate: 0x874 != 0 and 0x87c > 0 to do any work.
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(rider);
        w(c, rider + 0x874, if seed % 8 == 0 { 0 } else { 1 });
        w(
            c,
            rider + 0x87c,
            match seed % 8 {
                1 => 0,
                2 => 0xffff_ffff,
                _ => 1,
            },
        );

        // The rider's pose object; both virtual calls go to the fake hook.
        w(c, rider + 0x6c0, vt);
        w16(c, vt + 0x28, 0);
        w(c, vt + 0x2c, 0x5000);
        w(c, LOG_RESULTS, amin);
        w(c, LOG_RESULTS + 4, amax);
        for a in [amin, amax] {
            w(c, a, 0);
            w(c, a + 4, 0x4500_0000); // 2048.0
            w(c, a + 8, 0);
            w(c, a + 12, 0);
        }

        // Category 0 is the manager itself; its key is either inside or far
        // outside the rider box depending on the seed.
        let outside = !matches!(seed % 8, 3 | 4 | 5 | 6);
        let idx = if outside { -100i32 } else { -1i32 };
        w(c, mgr, 11);
        for off in [4u32, 8, 12] {
            w(c, mgr + off, idx as u32);
        }

        // The other seven categories (spatial key + list head at +0x10).
        for off in [0x14u32, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c] {
            w(c, mgr + off, 11);
            for k in [4u32, 8, 12] {
                w(c, mgr + off + k, (-100i32) as u32);
            }
            w(c, mgr + off + 0x10, 0);
        }

        // A small valid list node so category 0's head and the child slots
        // are real (all child pointers null, so no octree walk).
        let node = DATA + 0x18_000;
        let walk = DATA + 0x19_000;
        w(c, node + 0x28, 0);
        for off in [0u32, 4, 8, 12, 0x10, 0x14, 0x18, 0x1c] {
            w(c, node + off, 0);
        }

        match seed % 8 {
            3 => {
                w(c, mgr + 0x10, node);
            }
            4 => {
                // One non-matching list node: type != 4, end of list.
                w(c, walk, 0);
                w(c, walk + 8, 0);
                w(c, node + 0x28, walk);
                w(c, mgr + 0x10, node);
            }
            5 => {
                // One matching list node (type 4): two virtual calls; the
                // second is pointed at ActiveCameraTriggerList_Find with an
                // empty list, so it returns 0 and no list is modified.
                let trigger = DATA + 0x1a_000;
                let obj1 = DATA + 0x1b_000;
                let obj2 = DATA + 0x1c_000;
                w(c, walk, 0);
                w(c, walk + 8, 4);
                w(c, walk + 0xc, trigger);
                w(c, node + 0x28, walk);
                w(c, mgr + 0x10, node);
                w(c, trigger + 8, obj2);
                w(c, vt + 0x6c0, obj1);
                w16(c, obj1 + 0x28, 0);
                w(c, obj1 + 0x2c, 0x5000);
                w(c, obj2 + 0x24, obj2);
                w(c, obj2, 0);
                w16(c, obj2 + 0x38, 0);
                w(c, obj2 + 0x3c, 0x0016_c6b0);
            }
            6 => {}
            7 => {
                // Every category has a list head: all eight category tests
                // run (but their keys are outside the box, so none walks a
                // list or collects a box).
                let dummy = DATA + 0x1d_000;
                for off in [0x10u32, 0x24, 0x38, 0x4c, 0x60, 0x74, 0x88, 0x9c] {
                    w(c, mgr + off, dummy);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn camera_trigger_man_update_for_rider_matches() {
        strict(
            ee::s02::f_0016d320__generated,
            camera_trigger_man_update_for_rider,
            SEEDS,
            &setup,
        );
    }
}
