//! Native traversal of model nodes before relocating each part's DMA.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_bf50,
    relocate_model_dma_parts,
    "cPS2ChunkResolve_RelocateMeshDmaParts",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Walk `model->nodes` (+8), using the signed node count at +4. Each
/// sixteen-byte node's +4 field is an already-relocated geometry pointer;
/// nonzero geometry is passed to the part walker with the model base in A2.
/// The node's secondary geometry (+8) is not visited by this DMA pass.
pub fn relocate_model_dma_parts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r(c, S2).wrapping_add(4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = c.lw(r(c, S2).wrapping_add(8));
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = c.g[V0][0];
            loop {
                c.cycles += 3;
                c.g[A1][0] = c.lw(r(c, S1).wrapping_add(4));
                let no_geometry = c.g[A1][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if !no_geometry {
                    c.cycles += 2;
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x003a_bfc0, 0x003a_bf94)?;
                }
                c.cycles += 3;
                c.g[S0][0] = sx(r(c, S0).wrapping_sub(1));
                let more_nodes = c.g[S0][0] != 0;
                c.g[S1][0] = sx(r(c, S1).wrapping_add(0x10));
                if !more_nodes {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_bf80);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
