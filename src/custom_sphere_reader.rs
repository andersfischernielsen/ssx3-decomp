//! Native reader for serialized sphere-tree collision bodies.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_7da8,
    parse_sphere_header,
    "cPS2ChunkResolve_ParseHeader",
)];

const BODY_DATA: u32 = 0x98;
const INLINE_DATA: u32 = 0xc0;
const COPY_MEMORY: u32 = 0x003e_6574;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

fn copy_field(
    c: &mut Cpu,
    source_offset: u32,
    destination_offset: u32,
    size: u64,
    ret: u32,
) -> Flow {
    c.g[A0][0] = c.lw(r(c, S0).wrapping_add(BODY_DATA));
    c.g[A1][0] = sx(r(c, S1).wrapping_add(source_offset));
    c.g[A2][0] = size;
    c.g[A0][0] = sx(r(c, A0).wrapping_add(destination_offset));
    call_ptr(c, COPY_MEMORY, ret)
}

/// `int ParseHeader(SphereTree* body, const void* source, int value)`
/// (0x327da8): populate the body's inline collision data and retain pointers
/// into the serialized levels and occupancy mask. A null source consumes no
/// bytes. Otherwise the returned byte count includes the mask and padding.
pub fn parse_sphere_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        let null_source = c.g[S1][0] == 0;
        c.g[V0][0] = 0;
        if !null_source {
            c.cycles += 61;
            c.g[V0][0] = sx(r(c, S0).wrapping_add(INLINE_DATA));
            c.g[V1][0] = 1;
            c.sw(r(c, S0).wrapping_add(0xc0), c.g[A2][0]);
            c.g[A0][0] = sx(r(c, SP).wrapping_add(0x10));
            c.sw(r(c, S0).wrapping_add(0xc4), 0);
            c.g[A1][0] = c.g[S1][0];
            c.sw(r(c, S0).wrapping_add(BODY_DATA), c.g[V0][0]);
            c.g[A2][0] = 4;
            c.sw(r(c, S0).wrapping_add(0xd0), c.g[V1][0]);
            call_ptr(c, COPY_MEMORY, 0x0032_7df0)?;
            c.g[A0][0] = sx(r(c, SP).wrapping_add(0x14));
            c.g[A1][0] = sx(r(c, S1).wrapping_add(4));
            c.g[A2][0] = 4;
            call_ptr(c, COPY_MEMORY, 0x0032_7e00)?;

            for (source, destination, size, ret) in [
                (0x08, 0x08, 0x04, 0x0032_7e14),
                (0x0c, 0x0c, 0x04, 0x0032_7e28),
                (0x10, 0x14, 0x0c, 0x0032_7e3c),
                (0x1c, 0x2c, 0x0c, 0x0032_7e50),
                (0x28, 0x38, 0x24, 0x0032_7e64),
                (0x4c, 0x5c, 0x24, 0x0032_7e78),
            ] {
                copy_field(c, source, destination, size, ret)?;
            }

            c.g[V0][0] = c.lw(r(c, S0).wrapping_add(BODY_DATA));
            c.g[A1][0] = sx(r(c, S1).wrapping_add(0x70));
            c.g[A0][0] = 12;
            c.sw(r(c, V0).wrapping_add(0x20), c.g[A1][0]);
            c.g[V1][0] = c.lw(r(c, S0).wrapping_add(BODY_DATA));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xc));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
            c.mult(0, A2, V0, A0);
            c.g[A1][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
            c.sw(r(c, V1).wrapping_add(0x24), c.g[A1][0]);
            c.g[A2][0] = c.lw(r(c, S0).wrapping_add(BODY_DATA));
            c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x14));
            c.g[A0][0] = c.lw(r(c, SP).wrapping_add(0x10));
            c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
            c.g[V1][0] = c.lw(r(c, A2).wrapping_add(8));
            let raw_mask = c.g[V1][0] == 0;
            c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
            c.cycles += 2;
            if raw_mask {
                c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x24));
                c.sw(r(c, A2).wrapping_add(0x28), c.g[V0][0]);
            } else {
                c.sw(r(c, A2).wrapping_add(0x28), 0);
            }

            c.cycles += 13;
            c.g[V1][0] = c.lw(r(c, S0).wrapping_add(BODY_DATA));
            c.g[V0][0] = sx(r(c, A1).wrapping_sub(r(c, S1)));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.lwc1(0, r(c, V1).wrapping_add(0x14));
            c.swc1(0, r(c, SP));
            c.lwc1(1, r(c, V1).wrapping_add(0x18));
            c.swc1(1, r(c, SP).wrapping_add(4));
            c.lwc1(0, r(c, V1).wrapping_add(0x1c));
            c.swc1(2, r(c, SP).wrapping_add(0xc));
            c.swc1(0, r(c, SP).wrapping_add(8));
            c.lq(V1, r(c, SP));
            c.sq(V1, r(c, S0).wrapping_add(0x80));
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}
