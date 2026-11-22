//! Native course residency readiness, including its sky and shared section.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0022_d278, is_event_loaded, "SectionMan_IsEventLoaded")];

const IS_SECTION_LOADED: u32 = 0x0022_e078;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `bool SectionMan_IsEventLoaded(SectionMan* man, int course)` (0x22d278).
/// Every location listed by the course must be loaded. The optional sky and
/// shared sections are checked only when they have a nonnegative world map.
pub fn is_event_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x30);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S6][0] = 0x0044_0000;

        // Scan the authored list of at most six ordinary world locations.
        loop {
            c.cycles += 8;
            c.g[V0][0] = 0x28;
            c.g[S2][0] = sx(r(c, S6).wrapping_add(0x2488));
            c.mult(0, S0, S4, V0);
            c.g[V1][0] = sx(r(c, S2).wrapping_add(r(c, S0)));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(4));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let finished = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r(c, S1) << 2);
            if finished {
                break;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S5][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
            c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
            c.g[A1][0] = c.lw(r(c, V0).wrapping_add(0x10));
            call_ptr(c, IS_SECTION_LOADED, 0x0022_d2e4)?;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[S3][0] = 0;
                break;
            }
            c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x0022_d2b0);
            }
        }

        // The row's sky slot at +8 may be absent from the current SDB.
        c.cycles += 4;
        c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, S0)));
        c.g[A1][0] = c.lw(r(c, V0).wrapping_add(8));
        let no_sky = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = 0x0044_0000;
        let mut sky_map_missing = false;
        if !no_sky {
            c.cycles += 6;
            c.g[A0][0] = sx(r(c, A1) << 4);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x2168));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(4));
            sky_map_missing = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x28;
            if !sky_map_missing {
                c.cycles += 3;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, IS_SECTION_LOADED, 0x0022_d320)?;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
            }
        }
        if !sky_map_missing {
            c.cycles += 1;
            c.g[V0][0] = 0x28;
        }

        // The shared (TRANSP) slot at +0xc has the same optional semantics.
        c.cycles += 6;
        c.g[V1][0] = sx(r(c, S6).wrapping_add(0x2488));
        c.mult(0, V0, S4, V0);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A1][0] = c.lw(r(c, V1).wrapping_add(0xc));
        let no_shared = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = 0x0044_0000;
        let mut shared_map_missing = false;
        if !no_shared {
            c.cycles += 6;
            c.g[A0][0] = sx(r(c, A1) << 4);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x2168));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(4));
            shared_map_missing = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = c.g[S3][0];
            if !shared_map_missing {
                c.cycles += 3;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, IS_SECTION_LOADED, 0x0022_d360)?;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
            }
        }
        if !shared_map_missing {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
