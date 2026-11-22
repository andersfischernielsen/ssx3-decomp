//! Native streaming-view request and camera readiness calculation.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0015_ec98,
    game_camera_update_stream_view,
    "GameCamera_UpdateStreamView",
)];

const APP: u32 = 0x848;
const STREAM_SYSTEM: u32 = 0x16c8;
const FAR_DISTANCE: u32 = 8;
const POSITION: u32 = 0x20;
const VIEW_INDEX: u32 = 0xb0;
const VIEW_READY: u32 = 0xb4;
const STREAM_VIEW_STRIDE: u32 = 0x50;
const STREAM_VIEW_ACTIVE: u32 = 0x250;
const REQUEST_VIEW: u32 = 0x003a_9658;
const QUERY_VIEW: u32 = 0x003a_96e0;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void GameCamera_UpdateStreamView(GameCamera* camera)` (0x15ec98).
/// Request a streamed sphere around the camera with radius `1.5*r`. In a
/// single-view game `r` is the camera's far distance; otherwise it is clamped
/// to [0, 15000]. The readiness distance is `clamp((2/3)*r, 15000, 30000)`.
/// Clear readiness, query the available streamed range, then mark ready when
/// either that distance or `r` is covered. Keep the native FPU operations,
/// scratch registers and the three epilogue paths exactly as generated.
pub fn game_camera_update_stream_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        c.g[A1][0] = c.lw(r(c, GP).wrapping_sub(APP));
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 1;
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        c.g[V0][0] = c.lw(at(c, A1, 0x84));
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x10));
        if c.g[A0][0] == c.g[A2][0] {
            c.cycles += 2;
            c.lwc1(20, at(c, S1, FAR_DISTANCE));
        } else {
            c.lwc1(1, at(c, S1, FAR_DISTANCE));
            c.cycles += 6;
            c.mtc1(ZERO, 0);
            c.lwc1(2, r(c, GP).wrapping_sub(0x6c68));
            c.c_le_s(0, 1);
            let nonnegative = c.fcc();
            c.mov_s(3, 0);
            if nonnegative {
                c.cycles += 1;
                c.min_s(3, 1, 2);
            }
            c.cycles += 1;
            c.mov_s(20, 3);
        }

        c.cycles += 11;
        c.lwc1(0, r(c, GP).wrapping_sub(0x6c64));
        c.lwc1(2, r(c, GP).wrapping_sub(0x6c60));
        c.mul_s(0, 20, 0);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 1);
        c.mov_s(21, 2);
        c.lwc1(3, r(c, GP).wrapping_sub(0x6c5c));
        c.c_le_s(2, 0);
        let above_minimum = c.fcc();
        c.mul_s(12, 20, 1);
        if above_minimum {
            c.cycles += 1;
            c.min_s(21, 0, 3);
        }

        c.cycles += 22;
        c.g[V1][0] = c.lw(at(c, S1, VIEW_INDEX));
        c.g[V0][0] = STREAM_VIEW_STRIDE as u64;
        c.g[S0][0] = c.lw(r(c, GP).wrapping_add(STREAM_SYSTEM));
        c.g[S2][0] = 1;
        c.mult(0, V1, V1, V0);
        c.g[A2][0] = sx(at(c, S1, POSITION));
        c.g[V0][0] = c.lw(r(c, S0));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.sw(at(c, V0, STREAM_VIEW_ACTIVE), c.g[S2][0]);
        c.g[A0][0] = c.lw(r(c, S0));
        c.g[A1][0] = c.lw(at(c, S1, VIEW_INDEX));
        c.g[A0][0] = sx(at(c, A0, 0x10));
        call_ptr(c, REQUEST_VIEW, 0x0015_ed60)?;

        c.sw(at(c, S1, VIEW_READY), 0);
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.lw(at(c, S1, VIEW_INDEX));
        c.g[A3][0] = sx(at(c, SP, 0x10));
        c.g[A0][0] = c.lw(r(c, S0));
        c.g[A0][0] = sx(at(c, A0, 0x10));
        call_ptr(c, QUERY_VIEW, 0x0015_ed7c)?;
        let available = c.g[V0][0] != 0;
        c.lwc1(0, at(c, SP, 0x10));
        let mut s0_restored_on_failure = false;
        if available {
            c.cycles += 4;
            c.c_le_s(21, 0);
            if c.fcc() {
                c.sw(at(c, S1, VIEW_READY), c.g[S2][0]);
            } else {
                c.cycles += 4;
                c.c_le_s(20, 0);
                let radius_covered = c.fcc();
                frame.restore(c, S0, 0x50);
                if radius_covered {
                    c.cycles += 1;
                    c.sw(at(c, S1, VIEW_READY), c.g[S2][0]);
                } else {
                    s0_restored_on_failure = true;
                }
            }
        }
        if !s0_restored_on_failure {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    });
}
