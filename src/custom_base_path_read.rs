//! Native variable-length path fields, geometry and event decoding.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0026_a180, path_read, "Path_Read")];

const READ_INT: u32 = 0x0026_ba68;
const READ_VEC3: u32 = 0x0026_ba48;
const READ_SEGMENT: u32 = 0x0026_a338;
const READ_EVENT: u32 = 0x0026_a378;
const FIELD_HEAD: u32 = 0x0026_a1e0;
const SEGMENT_HEAD: u32 = 0x0026_a2a8;
const EVENT_HEAD: u32 = 0x0026_a2e8;
const EVENT_COUNT: u32 = 0;
const EVENTS: u32 = 4;
const SEGMENT_COUNT: u32 = 8;
const ORIGIN: u32 = 0xc;
const SEGMENTS: u32 = 0x18;
const BOUNDS_MIN: u32 = 0x1c;
const BOUNDS_MAX: u32 = 0x28;
const PATH_VTABLE: u32 = 0x34;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

#[inline]
fn advance_source(c: &mut Cpu, bytes: u32) {
    c.g[S0][0] = sx(at(c, S0, bytes));
}

/// `int Path_Read(Path* self, const void* source)` (0x26a180).
/// Reset the derived path through its virtual method, read the keyed fields,
/// then bind and decode its segment/event arrays in the supplied buffer.
/// Each field is a key and byte length followed by that many payload bytes;
/// the serialized TrackPath header `1, 0, 4, distance` therefore contains
/// one four-byte distance field, rather than a pass/lap count of four.
/// Return the consumed byte count and retain the three resumable loop heads.
pub fn path_read(c: &mut Cpu) {
    guest(c, |c| {
        match c.pc {
            0x0026_a180 => {
                c.cycles += 23;
                let frame = Frame::open(c, 0x70);
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x40);
                c.g[S1][0] = 0;
                frame.save(c, S3, 0x30);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S4, 0x20);
                frame.save_ra(c, 0x10);
                c.g[S4][0] = c.g[S0][0];
                c.g[V1][0] = c.lw(at(c, S2, PATH_VTABLE));
                c.g[A0][0] = c.lh(at(c, V1, 0x18));
                c.g[V0][0] = c.lw(at(c, V1, 0x1c));
                let target = r(c, V0);
                c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0026_a1c0)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, READ_INT, 0x0026_a1cc)?;
                advance_source(c, 4);
                c.g[V0][0] = c.lw(r(c, SP));
                let has_fields = (c.g[V0][0] as i64) > 0;
                c.g[S3][0] = sx(at(c, SP, 0xc));
                if has_fields {
                    c.cycles += 1;
                    read_fields(c)?;
                }
                read_geometry(c)?;
                read_events(c)?;
            }
            FIELD_HEAD => {
                read_fields(c)?;
                read_geometry(c)?;
                read_events(c)?;
            }
            SEGMENT_HEAD => {
                read_segments(c)?;
                read_events(c)?;
            }
            EVENT_HEAD => read_event_array(c)?,
            pc => {
                c.jump_out(pc, 0x0026_a180, 0x0026_a338);
                return Ok(());
            }
        }
        c.cycles += 9;
        c.g[V0][0] = sx(r(c, S0).wrapping_sub(r(c, S4)));
        let frame = Frame { size: 0x70 };
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

fn read_fields(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 22;
        c.g[A0][0] = sx(at(c, SP, 4));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, READ_INT, 0x0026_a1ec)?;
        advance_source(c, 4);
        c.g[A0][0] = sx(at(c, SP, 8));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, READ_INT, 0x0026_a1fc)?;
        advance_source(c, 4);
        c.g[V1][0] = c.lw(at(c, S2, PATH_VTABLE));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.lw(at(c, SP, 4));
        c.g[S1][0] = sx(at(c, S1, 1));
        c.g[A0][0] = c.lh(at(c, V1, 0x20));
        c.g[V0][0] = c.lw(at(c, V1, 0x24));
        c.g[A3][0] = c.lw(at(c, SP, 8));
        let target = r(c, V0);
        c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0026_a224)?;
        c.g[V1][0] = c.lw(at(c, SP, 8));
        c.g[V0][0] = c.lw(r(c, SP));
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        let payload_bytes = r(c, V1);
        advance_source(c, payload_bytes);
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, FIELD_HEAD) {
            return Err(Unwound);
        }
    }
}

fn read_geometry(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, READ_INT, 0x0026_a244)?;
    advance_source(c, 4);
    c.g[V0][0] = c.lw(at(c, SP, 0xc));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S3][0];
    advance_source(c, 4);
    c.sw(at(c, S2, SEGMENT_COUNT), c.g[V0][0]);
    c.g[S1][0] = 0;
    call_ptr(c, READ_INT, 0x0026_a264)?;
    c.g[V0][0] = c.lw(at(c, SP, 0xc));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx(at(c, S2, ORIGIN));
    advance_source(c, 0xc);
    c.sw(at(c, S2, EVENT_COUNT), c.g[V0][0]);
    call_ptr(c, READ_VEC3, 0x0026_a27c)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx(at(c, S2, BOUNDS_MIN));
    call_ptr(c, READ_VEC3, 0x0026_a288)?;
    advance_source(c, 0xc);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx(at(c, S2, BOUNDS_MAX));
    call_ptr(c, READ_VEC3, 0x0026_a298)?;
    advance_source(c, 0xc);
    c.g[V0][0] = c.lw(at(c, S2, SEGMENT_COUNT));
    let has_segments = (c.g[V0][0] as i64) > 0;
    c.sw(at(c, S2, SEGMENTS), c.g[S0][0]);
    if has_segments {
        read_segments(c)?;
    }
    Ok(())
}

fn read_segments(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 11;
        c.g[V0][0] = c.lw(at(c, S2, SEGMENTS));
        c.g[A2][0] = sx(r(c, S1) << 4);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
        c.g[S1][0] = sx(at(c, S1, 1));
        call_ptr(c, READ_SEGMENT, 0x0026_a2c4)?;
        c.g[V0][0] = c.lw(at(c, S2, SEGMENT_COUNT));
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        advance_source(c, 0x10);
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, SEGMENT_HEAD) {
            return Err(Unwound);
        }
    }
}

fn read_events(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = c.lw(at(c, S2, EVENT_COUNT));
    c.g[S1][0] = 0;
    let has_events = (c.g[V0][0] as i64) > 0;
    c.sw(at(c, S2, EVENTS), c.g[S0][0]);
    if has_events {
        c.cycles += 1;
        read_event_array(c)?;
    }
    Ok(())
}

fn read_event_array(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 11;
        c.g[V0][0] = c.lw(at(c, S2, EVENTS));
        c.g[A2][0] = sx(r(c, S1) << 4);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
        c.g[S1][0] = sx(at(c, S1, 1));
        call_ptr(c, READ_EVENT, 0x0026_a304)?;
        c.g[V0][0] = c.lw(at(c, S2, EVENT_COUNT));
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        advance_source(c, 0x10);
        if !more {
            return Ok(());
        }
        if back_edge_unwinds(c, EVENT_HEAD) {
            return Err(Unwound);
        }
    }
}
