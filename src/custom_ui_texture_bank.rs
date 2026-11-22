//! Native loading of UI texture banks and their per-image mappings.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0039_8138,
    ui_texture_bank_set_data,
    "cUITextureBank_setData",
)];

const ALLOC: u32 = 0x0031_7e30;
const MEMSET: u32 = 0x0041_6210;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Load the named SSH bank, resolve its images through the UI renderer, and
/// build one 32-byte native mapping for each serialized 24-byte image record.
/// A2 points to the serialized count followed by those image records.
pub fn ui_texture_bank_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S4, 0x90);
        frame.save(c, S1, 0xc0);
        c.g[S4][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S0, 0xd0);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S3, 0xa0);
        c.g[A1][0] = sx(r(c, V0).wrapping_add(0x3d90));
        frame.save(c, S2, 0xb0);
        frame.save(c, S5, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = 0;
        call_ptr(c, 0x0041_7828, 0x0039_8178)?;
        c.g[A1][0] = 0x0300_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] |= 0x100;
        call_ptr(c, 0x003e_22f0, 0x0039_8188)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = 0x0049_0000;
        c.g[V0][0] = c.lw(r(c, S3).wrapping_add(8));
        c.g[A2][0] = 0;
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3da0));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx(r(c, V0) << 2);
        c.sw(r(c, S1).wrapping_add(8), c.g[V0][0]);
        call_ptr(c, ALLOC, 0x0039_81ac)?;
        c.g[A2][0] = c.lw(r(c, S1).wrapping_add(8));
        c.g[A0][0] = c.g[V0][0];
        c.sw(r(c, S1).wrapping_add(4), c.g[V0][0]);
        c.g[A1][0] = 0xff;
        c.g[A2][0] = sx(r(c, A2) << 2);
        call_ptr(c, MEMSET, 0x0039_81c4)?;
        c.g[V1][0] = c.lw(r(c, S1).wrapping_add(8));
        let no_bank_images = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = sx(r(c, S3).wrapping_add(0x14));
        if !no_bank_images {
            resolve_bank_images(c)?;
        }

        c.cycles += 16;
        c.g[A0][0] = c.g[S3][0];
        c.g[S2][0] = sx(r(c, S4).wrapping_add(4));
        call_ptr(c, 0x0025_23a8, 0x0039_823c)?;
        c.g[S3][0] = sx(r(c, S1).wrapping_add(0xc));
        c.sw(r(c, S1), c.g[S4][0]);
        c.g[A1][0] = 0x0049_0000;
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3db0));
        c.g[A2][0] = 0;
        c.g[S0][0] = c.lw(r(c, S4));
        c.g[A3][0] = 0;
        c.sw(r(c, S1).wrapping_add(0x10), c.g[S0][0]);
        c.g[A0][0] = sx(r(c, S0) << 5);
        call_ptr(c, ALLOC, 0x0039_8264)?;
        c.g[A0][0] = sx(r(c, S0).wrapping_sub(1));
        let no_mappings = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !no_mappings {
            initialize_mappings(c);
        }

        c.cycles += 9;
        c.sw(r(c, S3), c.g[V0][0]);
        c.g[A1][0] = 0xff;
        c.g[A2][0] = c.lw(r(c, S1).wrapping_add(0x10));
        c.g[A0][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.g[A2][0] = sx(r(c, A2) << 2);
        call_ptr(c, MEMSET, 0x0039_82c0)?;
        c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x10));
        let no_mappings = (c.g[V1][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !no_mappings {
            fill_mappings(c);
        }

        c.cycles += 9;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    });
}

/// SSH directory entries contain image offsets relative to the bank header.
fn resolve_bank_images(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[S5][0] = 0x004a_0000;
    loop {
        c.cycles += 4;
        c.g[V0][0] = c.lw(r(c, S2));
        c.g[A1][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
        let absent = c.g[A1][0] == 0;
        c.g[T2][0] = c.lw(r(c, GP).wrapping_sub(0x854));
        if !absent {
            c.cycles += 13;
            c.g[A2][0] = sx(r(c, S5).wrapping_add(0x4778));
            c.g[A3][0] = 0;
            c.g[T0][0] = 1;
            c.g[V1][0] = c.lw(r(c, T2).wrapping_add(0x10d8));
            c.g[T1][0] = u64::MAX;
            c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x170));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x174));
            let target = r(c, V0);
            if target == 0 || target & 3 != 0 || target >= 0x0200_0000 {
                eprintln!(
                    "[ui texture] invalid renderer callback: target={target:08x} graphics={:08x} vtable={:08x} bank={:08x} image={} texture_set={:08x}",
                    r(c, T2), r(c, V1), r(c, S3), r(c, S0), r(c, S1)
                );
            }
            c.g[A0][0] = sx(r(c, T2).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0039_820c)?;
            c.g[A0][0] = c.lw(r(c, S1).wrapping_add(4));
            c.g[V1][0] = sx(r(c, S0) << 2);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.sw(r(c, V1), c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(8));
        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S2][0] = sx(r(c, S2).wrapping_add(8));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_81d8);
        }
    }
}

/// Initialize the allocated native image records before applying file data.
fn initialize_mappings(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[A1][0] = u64::MAX;
    loop {
        c.cycles += 10;
        c.sw(r(c, V1), c.g[A1][0]);
        c.g[A0][0] = sx(r(c, A0).wrapping_sub(1));
        c.swc1(0, r(c, V1).wrapping_add(4));
        c.swc1(0, r(c, V1).wrapping_add(8));
        c.sw(r(c, V1).wrapping_add(0xc), 0);
        c.sw(r(c, V1).wrapping_add(0x10), 0);
        c.swc1(0, r(c, V1).wrapping_add(0x14));
        c.swc1(0, r(c, V1).wrapping_add(0x18));
        let more = c.g[A0][0] != c.g[A1][0];
        c.g[V1][0] = sx(r(c, V1).wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_8280);
        }
    }
}

fn fill_mappings(c: &mut Cpu) {
    c.cycles += 1;
    loop {
        c.cycles += 34;
        c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.g[A1][0] = sx(r(c, A3) << 5);
        c.lwc1(0, r(c, S2).wrapping_add(0xc));
        c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
        c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
        c.swc1(0, r(c, V1).wrapping_add(0xc));
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.lwc1(0, r(c, S2).wrapping_add(8));
        c.g[V0][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
        c.swc1(0, r(c, V0).wrapping_add(0x10));
        c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.lwc1(0, r(c, S2).wrapping_add(0x14));
        c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
        c.swc1(0, r(c, V1).wrapping_add(0x18));
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.lwc1(0, r(c, S2).wrapping_add(0x10));
        c.g[V0][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
        c.swc1(0, r(c, V0).wrapping_add(0x14));
        c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.g[A0][0] = c.lw(r(c, S2));
        c.g[V1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
        c.sw(r(c, V1).wrapping_add(0x1c), c.g[A0][0]);
        c.g[V0][0] = c.lbu(r(c, S2).wrapping_add(5));
        c.g[A2][0] = c.lw(r(c, S1).wrapping_add(4));
        c.g[V0][0] = sx(r(c, V0) << 2);
        c.g[A0][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
        c.g[V1][0] = c.lw(r(c, V0));
        c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        c.sw(r(c, A1), c.g[V1][0]);
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x10));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S2][0] = sx(r(c, S2).wrapping_add(0x18));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_82d0);
        }
    }
}
