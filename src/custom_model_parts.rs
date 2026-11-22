//! Native traversal of the mesh groups within one model part.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_bf20,
        relocate_model_dma,
        "cPS2ChunkResolve_RelocateMeshDma",
    ),
    (
        0x003a_bfc0,
        relocate_model_part,
        "cPS2ChunkResolve_RelocateMeshDmaPart",
    ),
    (0x003a_dc48, accept_model_group, "cPS2ChunkResolve_v08"),
];

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Relocate the model's DMA base before walking its node table.
pub fn relocate_model_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x24));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.sw(r(c, V1).wrapping_add(0x24), c.g[V0][0]);
        call_ptr(c, 0x003a_bf50, 0x003a_bf3c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    });
}

/// The native group callback accepts the descriptor without changing it.
pub fn accept_model_group(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 1;
    crate::abi::ret(c);
}

/// Relocate each group from the part's count (+0x1c) and pointer table (+0x20).
pub fn relocate_model_part(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x1c));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = c.lw(r(c, S2).wrapping_add(0x20));
        if !empty {
            loop {
                c.cycles += 10;
                c.g[A1][0] = c.lw(r(c, S1));
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S3][0];
                call_ptr(c, 0x003a_c048, 0x003a_c00c)?;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x1c));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_bff8);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
