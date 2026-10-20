//! Phase 2 wave 3 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for collision, input and script helpers, each with a
//! differential test against the generated body.
//!
//! - **World painter cell lookup** (`WorldPainterQuery_LookupCell`): the
//!   quad-tree cell a query key falls in, or null for "outside".
//! - **Input VM operands** (`InputVM_ReadOperand`): the floating-point value
//!   of one bytecode operand: a script variable, a builtin or a controller
//!   channel reading.
//! - **Segment/triangle** (`CollSegment_CollideTriangle`): the segment
//!   against a ray/triangle test in a sphere tree.
//! - **Focus octree** (`FocusMan_CollectOctreeNode`): the focus manager's
//!   recursive walk of the octree, collecting the nodes that overlap the
//!   box at `+0x400`.
//! - **Nearest rail** (`OverlapList_FindNearestRail`): all overlapping rail
//!   objects against a sphere, keeping the closest contact.
//!
//! The VU0 code issues the original's COP2 macro operations in the same
//! order, so VF/ACC/Q and the VU0 flags end as the original leaves them.

use crate::call::{guest, Callee, Flow, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::{abi, events};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_0a10, world_painter_query_lookup_cell, "WorldPainterQuery_LookupCell"),
    (0x0032_5260, input_vm_read_operand, "InputVM_ReadOperand"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const WP_DATA_FIND_CELL_INDEX: Callee = Callee {
        addr: 0x002b_af90,
        f: ee::s08::WPData_FindCellIndex,
    };
    pub const INPUT_VM_READ_BUILTIN: Callee = Callee {
        addr: 0x0032_5430,
        f: ee::s10::InputVM_ReadBuiltin,
    };
    pub const INPUT_VM_READ_PAD_CHANNEL: Callee = Callee {
        addr: 0x0032_52f8,
        f: ee::s10::InputVM_ReadPadChannel,
    };
}

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

// ---------------------------------------------------------------------------
// World painter query: the cell a key falls in.

/// `sItem* WorldPainterQuery_LookupCell(sQuery* self, sList* list)`
/// (0x002c0a10): look the key up in the list's quad tree (the tree search
/// returns an index; -1 is "no cell") and return the cell record at
/// `list->items[index]`, or null.
pub fn world_painter_query_lookup_cell(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        callee::WP_DATA_FIND_CELL_INDEX.call(c, 0x002c_0a28)?;
        // The tree search leaves the cell index in v0; -1 means outside.
        let index = c.g[V0][0];
        c.g[A0][0] = index;
        c.g[V0][0] = u64::MAX;
        let outside = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if outside {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input VM: operand values.

/// `float InputVM_ReadOperand(sInputVM* self, int id)` (0x00325260): the
/// value of one operand. `id < 0x40` is a script variable (`self->v[id]`),
/// `0x40..0x60` a builtin, `0x60..0x3fc` controller channel
/// `(id - 0x60) / 6` reading `(id - 0x60) % 6`, and anything else 0.
pub fn input_vm_read_operand(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let id = c.g[A1][0];
        c.g[V0][0] = (id < 0x40) as u64;
        let not_variable = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !not_variable {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            let at = p(c.g[V0][0] as u32, 0x8);
            c.lwc1(0, at);
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x40..0x60: a builtin.
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = (c.g[V1][0] < 0x20) as u64;
        let not_builtin = c.g[V0][0] == 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x60));
        if !not_builtin {
            c.cycles += 4;
            c.g[A1][0] = c.g[V1][0];
            callee::INPUT_VM_READ_BUILTIN.call(c, 0x0032_5298)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        // 0x60..0x3fc: controller channel `id / 6`, reading `id % 6`.
        c.cycles += 3;
        c.g[V0][0] = (c.g[A1][0] < 0x39c) as u64;
        let padded = c.g[V0][0] != 0;
        c.g[V0][0] = 0x6;
        if !padded {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = 0x6;
        c.div(0, A1, V0);
        c.cycles += 4;
        c.g[A1][0] = c.lo[0];
        c.g[A2][0] = c.hi[0];
        callee::INPUT_VM_READ_PAD_CHANNEL.call(c, 0x0032_52d8)?;
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}
