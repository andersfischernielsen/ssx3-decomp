//! Native race-line projection, split markers and crossed-event collection.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0011_2fb0,
    rider_update_dist_to_finish,
    "Rider_UpdateDistToFinish",
)];

const POSITION: u32 = 0x110;
const TRACK_PATH: u32 = 0xab4;
const TRACK_CURSOR: u32 = 0xac0;
const PATH_REMAINING: u32 = 0x38;
const PATH_VTABLE: u32 = 0x34;
const DISTANCE: u32 = 0x4d0;
const PREVIOUS_DISTANCE: u32 = 0x4d4;
const EVENTS: u32 = 0x4dc;
const EVENT_COUNT: u32 = 0x59c;
const SPLIT_DATA: u32 = 0x004d_33a0;
const SPLIT_LOOP: u32 = 0x0011_3058;
const PROJECT_PATH: u32 = 0x0026_a638;
const ON_SPLIT_MARKER: u32 = 0x0010_e558;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void Rider_UpdateDistToFinish(Rider* self, float* off_path,
/// float* arc_length, Vec4* nearest)` (0x112fb0).
/// Project the rider onto its selected TrackPath, then subtract that arc
/// length from the path's remaining distance at its origin. On forward
/// progress notify crossed global splits and collect up to twelve path
/// events; otherwise query the previous distance without advancing it.
/// This function has no lap counter: distance and events belong to the
/// selected path, and +0x4d4 advances only when the new distance decreases.
pub fn rider_update_dist_to_finish(c: &mut Cpu) {
    guest(c, |c| {
        match c.pc {
            0x0011_2fb0 => {
                c.cycles += 25;
                let frame = Frame::open(c, 0x60);
                c.g[T1][0] = 1;
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                c.g[S0][0] = c.g[A2][0];
                frame.save(c, S2, 0x20);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S3, 0x10);
                c.g[A2][0] = c.g[A3][0];
                frame.save_ra(c, 0);
                c.g[A3][0] = c.g[A1][0];
                frame.save_fpr(c, 20, 0x50);
                c.g[A1][0] = sx(at(c, S1, POSITION));
                c.g[A0][0] = c.lw(at(c, S1, TRACK_PATH));
                c.g[T0][0] = c.lw(at(c, S1, TRACK_CURSOR));
                call_ptr(c, PROJECT_PATH, 0x0011_2ff0)?;

                c.swc1(0, r(c, S0));
                c.g[A3][0] = c.lw(at(c, S1, TRACK_PATH));
                c.lwc1(12, at(c, S1, PREVIOUS_DISTANCE));
                c.lwc1(1, at(c, A3, PATH_REMAINING));
                c.sub_s(1, 1, 0);
                c.c_lt_s(1, 12);
                let forward = c.fcc();
                c.swc1(1, at(c, S1, DISTANCE));
                if forward {
                    c.cycles += 10;
                    c.g[V0][0] = 0x004d_0000;
                    c.mtc1(ZERO, 2);
                    c.g[V0][0] = sx(SPLIT_DATA);
                    c.g[S2][0] = 0;
                    c.lwc1(0, at(c, V0, 0x1c));
                    c.g[A1][0] = sx(at(c, V0, 0x18));
                    c.c_eq_s(0, 2);
                    let no_splits = c.fcc();
                    c.g[S3][0] = sx(at(c, S1, EVENTS));
                    if no_splits {
                        load_current_path(c);
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(0, 1);
                        if c.fcc() {
                            // The first split is already beyond this interval.
                            c.g[A3][0] = c.lw(at(c, S1, TRACK_PATH));
                        } else {
                            c.cycles += 3;
                            c.mov_s(20, 2);
                            c.g[S0][0] = c.g[A1][0];
                            c.lwc1(1, at(c, A1, 4));
                            scan_splits(c)?;
                            load_current_path(c);
                        }
                    }
                    collect_forward_events(c)?;
                } else {
                    collect_stationary_events(c)?;
                }
            }
            SPLIT_LOOP => {
                scan_splits(c)?;
                load_current_path(c);
                collect_forward_events(c)?;
            }
            pc => {
                c.jump_out(pc, 0x0011_2fb0, 0x0011_3128);
                return Ok(());
            }
        }

        c.cycles += 8;
        let frame = Frame { size: 0x60 };
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    });
}

fn load_current_path(c: &mut Cpu) {
    c.cycles += 1;
    c.g[A3][0] = c.lw(at(c, S1, TRACK_PATH));
}

fn scan_splits(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 5;
        c.lwc1(0, at(c, S1, PREVIOUS_DISTANCE));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, ON_SPLIT_MARKER, 0x0011_3074)?;
        }
        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
        let full = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S0, 8));
        if full {
            return Ok(());
        }

        c.cycles += 5;
        c.lwc1(1, at(c, S0, 4));
        c.c_eq_s(1, 20);
        let end = c.fcc();
        c.g[A1][0] = c.g[S0][0];
        if end {
            return Ok(());
        }
        c.cycles += 5;
        c.lwc1(0, at(c, S1, DISTANCE));
        c.c_lt_s(1, 0);
        if c.fcc() {
            return Ok(());
        }
        c.lwc1(1, at(c, A1, 4));
        if back_edge_unwinds(c, SPLIT_LOOP) {
            return Err(Unwound);
        }
    }
}

fn collect_forward_events(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[A1][0] = c.g[S3][0];
    c.lwc1(12, at(c, S1, PREVIOUS_DISTANCE));
    c.g[A2][0] = 12;
    c.g[V1][0] = c.lw(at(c, A3, PATH_VTABLE));
    c.lwc1(13, at(c, S1, DISTANCE));
    c.g[A0][0] = c.lh(at(c, V1, 8));
    c.g[V0][0] = c.lw(at(c, V1, 12));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
    call_ptr(c, target, 0x0011_30d4)?;
    c.lwc1(0, at(c, S1, DISTANCE));
    c.sw(at(c, S1, EVENT_COUNT), c.g[V0][0]);
    c.swc1(0, at(c, S1, PREVIOUS_DISTANCE));
    Ok(())
}

fn collect_stationary_events(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[V1][0] = c.lw(at(c, A3, PATH_VTABLE));
    c.mov_s(13, 12);
    c.g[A1][0] = sx(at(c, S1, EVENTS));
    c.g[A2][0] = 12;
    c.g[A0][0] = c.lh(at(c, V1, 8));
    c.g[V0][0] = c.lw(at(c, V1, 12));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
    call_ptr(c, target, 0x0011_3104)?;
    c.sw(at(c, S1, EVENT_COUNT), c.g[V0][0]);
    Ok(())
}
