//! Native relocation of a world script table and its instance/spline data.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003a_ccd8, worldscript_relocate, "tWScriptCache_Relocate")];

const CACHE_DATA: u32 = 8;
const INSTANCE_COUNT: u32 = 0x44;
const INSTANCE_PROPERTY_INDICES: u32 = 0x48;
const PROPERTY_COUNT: u32 = 0x4c;
const PROPERTY_POINTERS: u32 = 0x50;
const SPLINE_COUNT: u32 = 0x54;
const SPLINE_PROPERTIES: u32 = 0x58;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

fn finish(c: &mut Cpu, frame: Frame) {
    c.cycles += 7;
    c.sh(r(c, S0), c.g[V0][0]);
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `void tWScriptCache_Relocate(cache, world, section)` (0x3accd8).
/// Validate the format/checksum, relocate the serialized tables, bind each
/// instance to its property record and each property to its collision model,
/// then install spline attributes and relocate process/mission payloads.
pub fn worldscript_relocate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.lw(at(c, S0, CACHE_DATA));
        let absent = c.g[A3][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if !absent {
            c.cycles += 4;
            c.g[V1][0] = c.lhu(r(c, A3));
            c.g[V0][0] = 0x1000;
            let mut valid = false;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lh(at(c, S0, 2));
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r(c, S1));
                c.g[A1][0] = c.lw(at(c, A3, 0xc));
                c.g[V0][0] = c.lw(r(c, A0));
                c.g[V1][0] = c.lw(at(c, V0, 4));
                if c.g[A1][0] == c.g[V1][0] {
                    c.g[V1][0] = c.lw(at(c, S0, CACHE_DATA));
                    valid = true;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.lh(at(c, S0, 2));
                }
            }
            if !valid {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(at(c, S0, CACHE_DATA), 0);
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[A3][0];
                    call_ptr(c, 0x0031_7e98, 0x003a_cd3c)?;
                    c.sw(at(c, S0, CACHE_DATA), 0);
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw(at(c, S0, CACHE_DATA));
            }
            c.cycles += 2;
            let missing = c.g[V1][0] == 0;
            c.g[V0][0] = 3;
            if missing {
                finish(c, frame);
                return Ok(());
            }
            relocate_tables(c);
            bind_instances(c);
            resolve_colliders(c);
            bind_splines(c);
            relocate_processes_and_missions(c);
        }
        c.cycles += 1;
        c.g[V0][0] = 3;
        finish(c, frame);
        Ok(())
    })
}

fn relocate_field(c: &mut Cpu, base_register: usize, offset: u32) {
    c.g[base_register][0] = c.lw(at(c, S0, CACHE_DATA));
    c.g[V0][0] = c.lw(at(c, base_register, offset));
    c.g[V0][0] = sx(r(c, base_register).wrapping_add(r(c, V0)));
    c.sw(at(c, base_register, offset), c.g[V0][0]);
}

fn relocate_tables(c: &mut Cpu) {
    c.cycles += 44;
    c.g[V0][0] = c.lw(at(c, V1, 0x14));
    c.g[A1][0] = 0;
    c.g[V0][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    c.sw(at(c, V1, 0x14), c.g[V0][0]);
    for (register, offset) in [
        (V1, 0x1c),
        (A0, 0x24),
        (V1, 0x2c),
        (A0, 0x34),
        (V1, 0x3c),
        (A0, 0x40),
        (V1, 0x48),
        (A0, 0x50),
        (V1, 0x58),
    ] {
        relocate_field(c, register, offset);
    }
    c.g[A0][0] = c.lw(at(c, S0, CACHE_DATA));
    c.g[V0][0] = c.lw(at(c, A0, 0x38));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = c.lw(at(c, A0, 0x3c));
    if !empty {
        c.cycles += 1;
        loop {
            c.cycles += 9;
            c.g[V0][0] = c.lw(r(c, V1));
            c.g[A1][0] = sx(r(c, A1).wrapping_add(1));
            c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
            c.sw(r(c, V1), c.g[V0][0]);
            c.g[A0][0] = c.lw(at(c, S0, CACHE_DATA));
            c.g[V0][0] = c.lw(at(c, A0, 0x38));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(4));
            if !more {
                c.cycles += 1;
                c.g[A0][0] = c.lw(at(c, S0, CACHE_DATA));
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_ce00);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, A0, PROPERTY_COUNT));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if !empty {
        c.cycles += 1;
        c.g[A1][0] = c.lw(at(c, A0, PROPERTY_POINTERS));
        loop {
            c.cycles += 12;
            c.g[V1][0] = sx(r(c, A2) << 2);
            c.g[A0][0] = c.lw(at(c, A0, 0x40));
            c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
            c.g[V0][0] = c.lw(r(c, V1));
            c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
            c.sw(r(c, V1), c.g[A0][0]);
            c.g[A0][0] = c.lw(at(c, S0, CACHE_DATA));
            c.g[V0][0] = c.lw(at(c, A0, PROPERTY_COUNT));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A1][0] = c.lw(at(c, A0, PROPERTY_POINTERS));
            if event_due(c) {
                deliver_event_here(c, 0x003a_ce38);
            }
        }
    }
}

/// Resolve a handle after its section-table lookup has left V0 at the
/// section table and T0 holding the handle. The result is left in A2.
fn resolve_mesh_handle(c: &mut Cpu, table_offset: u32) {
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A2][0] = 0;
    } else {
        c.cycles += 8;
        c.g[V0][0] = c.lw(at(c, V0, table_offset));
        c.g[V1][0] = sx(r(c, T0) >> 8);
        c.g[V1][0] = sx(r(c, V1) << 2);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(r(c, V1));
        c.g[V0][0] = sx(r(c, A0) >> 8);
        let present = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r(c, V0) << 2);
        if !present {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[V0][0];
    }
}

fn bind_instances(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lw(at(c, S0, CACHE_DATA));
    c.g[T0][0] = 0xffff_ffff_ffff_0000;
    c.g[A3][0] = 0;
    c.g[T0][0] |= 0xff00;
    c.g[V1][0] = c.lw(at(c, V0, INSTANCE_COUNT));
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[T0][0] = c.g[S2][0] | c.g[T0][0];
    if empty {
        return;
    }
    c.cycles += 3;
    c.g[T1][0] = 0xffff_ffff_ffff_0000;
    c.g[A2][0] = c.lw(r(c, S1));
    loop {
        c.cycles += 8;
        c.g[A1][0] = c.g[T0][0] & 0xff;
        c.g[V0][0] = sx(r(c, A1) << 2);
        c.g[V1][0] = sx(r(c, A3) << 8);
        c.g[A0][0] = c.lw(at(c, A2, 8));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
        c.g[V0][0] = c.lw(r(c, V0));
        c.g[T0][0] = c.g[A1][0] | c.g[V1][0];
        resolve_mesh_handle(c, 0x1c);
        c.cycles += 22;
        c.g[V0][0] = c.lw(at(c, S0, CACHE_DATA));
        c.g[A0][0] = sx(r(c, A3) << 1);
        c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
        c.g[V1][0] = c.lw(at(c, V0, INSTANCE_PROPERTY_INDICES));
        c.g[A1][0] = c.lw(at(c, V0, PROPERTY_POINTERS));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] = sx(r(c, V0) << 2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
        c.g[V1][0] = c.lw(r(c, V0));
        c.sw(at(c, A2, 0x88), c.g[V1][0]);
        c.g[V0][0] = c.lw(at(c, V1, 4));
        c.g[V0][0] &= c.g[T1][0];
        c.g[V1][0] = sx((r(c, V0) as i32 >> 16) as u32);
        c.g[V0][0] |= c.g[V1][0];
        c.g[V0][0] |= 2;
        c.sw(at(c, A2, 8), c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, S0, CACHE_DATA));
        c.g[V0][0] = c.lw(at(c, V1, INSTANCE_COUNT));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A2][0] = c.lw(r(c, S1));
        if event_due(c) {
            deliver_event_here(c, 0x003a_ce90);
        }
    }
}

fn resolve_colliders(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw(at(c, S0, CACHE_DATA));
    c.g[V0][0] = c.lw(at(c, V1, PROPERTY_COUNT));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if empty {
        return;
    }
    c.cycles += 4;
    c.g[T2][0] = 1;
    c.g[T1][0] = 3;
    c.g[V1][0] = c.lw(at(c, V1, PROPERTY_POINTERS));
    loop {
        c.cycles += 6;
        c.g[V0][0] = sx(r(c, A3) << 2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A2][0] = c.lw(r(c, V0));
        c.g[V0][0] = c.lw(r(c, A2));
        let mut resolve = c.g[V0][0] == c.g[T2][0];
        if resolve {
            c.g[A1][0] = c.lw(at(c, A2, 0xc));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[T1][0] {
                c.sw(at(c, A2, 0xc), 0);
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.lw(at(c, A2, 0xc));
                resolve = true;
            }
        }
        if resolve {
            c.cycles += 8;
            c.g[V0][0] = c.lw(r(c, S1));
            c.g[V1][0] = c.g[A1][0] & 0xff;
            c.g[A0][0] = c.lw(at(c, V0, 8));
            c.g[V1][0] = sx(r(c, V1) << 2);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V1));
            let missing = c.g[V1][0] == 0;
            c.g[V0][0] = sx(r(c, A1) >> 8);
            let mut found = false;
            if !missing {
                c.cycles += 7;
                c.g[A0][0] = c.lw(at(c, V1, 0x64));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(r(c, V0));
                c.g[V0][0] = sx(r(c, V1) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V0) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.sw(at(c, A2, 0xc), c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V1][0] = c.lw(at(c, S0, CACHE_DATA));
        c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
        c.g[V0][0] = c.lw(at(c, V1, PROPERTY_COUNT));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V1][0] = c.lw(at(c, V1, PROPERTY_POINTERS));
        if event_due(c) {
            deliver_event_here(c, 0x003a_cf58);
        }
    }
}

fn bind_splines(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw(at(c, S0, CACHE_DATA));
    c.g[T1][0] = c.lw(at(c, V0, SPLINE_COUNT));
    let empty = (c.g[T1][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if empty {
        return;
    }
    c.cycles += 2;
    c.g[T2][0] = 0xffff_ffff_ffff_0000;
    loop {
        c.cycles += 9;
        c.g[A2][0] = c.lw(r(c, S1));
        c.g[A1][0] = c.g[T0][0] & 0xff;
        c.g[V0][0] = sx(r(c, A1) << 2);
        c.g[V1][0] = sx(r(c, A3) << 8);
        c.g[A0][0] = c.lw(at(c, A2, 8));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
        c.g[V0][0] = c.lw(r(c, V0));
        c.g[T0][0] = c.g[A1][0] | c.g[V1][0];
        resolve_mesh_handle(c, 0x44);
        c.cycles += 21;
        c.g[A0][0] = c.lw(at(c, S0, CACHE_DATA));
        c.g[V0][0] = sx(r(c, A3) << 2);
        c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
        c.g[V1][0] = c.lw(at(c, A0, SPLINE_PROPERTIES));
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.lwl(V1, at(c, V0, 3));
        c.lwr(V1, r(c, V0));
        c.swl(V1, at(c, SP, 3));
        c.swr(V1, r(c, SP));
        c.g[V1][0] = c.lhu(at(c, SP, 2));
        c.g[V1][0] = sx(r(c, V1) << 16);
        c.g[V1][0] = sx((r(c, V1) as i32 >> 16) as u32);
        c.sw(at(c, A2, 0x28), c.g[V1][0]);
        c.g[V0][0] = c.lh(r(c, SP));
        c.g[V0][0] = sx(r(c, V0) << 16);
        c.g[V1][0] = sx(r(c, V0) >> 16);
        c.g[V0][0] &= c.g[T2][0];
        c.g[V0][0] |= c.g[V1][0];
        let more = c.g[A1][0] != 0;
        c.sw(at(c, A2, 0x1c), c.g[V0][0]);
        if !more {
            c.cycles += 1;
            c.g[V0][0] = c.lw(at(c, S0, CACHE_DATA));
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_cff0);
        }
    }
}

fn relocate_processes_and_missions(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw(at(c, V0, 0x28));
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[A0][0] = c.lw(at(c, V0, 0x2c));
    if !empty {
        c.cycles += 2;
        c.g[A1][0] = c.g[V1][0];
        relocate_records(c, 0x40, 0x003a_d0b0);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(at(c, S0, CACHE_DATA));
    c.g[V1][0] = c.lw(at(c, V0, 0x20));
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[A0][0] = c.lw(at(c, V0, 0x24));
    if !empty {
        c.cycles += 1;
        c.g[A1][0] = c.g[V1][0];
        relocate_records(c, 0x54, 0x003a_d0e0);
    }
}

fn relocate_records(c: &mut Cpu, stride: u32, loop_pc: u32) {
    loop {
        c.cycles += 7;
        c.g[V1][0] = c.lw(at(c, A0, 8));
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
        c.g[V0][0] = c.lw(at(c, S0, CACHE_DATA));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.sw(at(c, A0, 8), c.g[V0][0]);
        let more = c.g[A1][0] != 0;
        c.g[A0][0] = sx(r(c, A0).wrapping_add(stride));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, loop_pc);
        }
    }
}
