//! Native course-card and split-screen rider-picture loading.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_3048,
    game_load_screen_load_texture,
    "cGameLoadScreen_loadTexture",
)];

const TEXTURE_HANDLE: u32 = 0x10;
const SSH_ENTRY_OFFSET: u32 = 0x14;
const GRAPHICS_VTABLE: u32 = 0x10d8;
const LOAD_SSH_SLOT: u32 = 0x170;

fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    (c.g[register][0] as u32).wrapping_add(offset)
}

/// `void cGameLoadScreen_loadTexture(cGameLoadScreen *self)` (0x233048).
/// Single-player loads the selected track's SSH from `courspic.big` into
/// `self+0x10`; split-screen loads each rider's portrait from `charpic.big`
/// into the consecutive handles at `self+0x10` and `self+0x14`.
pub fn game_load_screen_load_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S2, 0xe0);
        frame.save(c, S0, 0x100);
        frame.save(c, S3, 0xd0);
        frame.save(c, S1, 0xf0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3078)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_3084)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_308c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_3098)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 2;
        c.g[V0][0] = 0x0053_0000;
        c.g[A0][0] = c.lb(at(c, V0, 0x5c11));
        let split_screen = c.g[A0][0] == c.g[V1][0];
        c.g[S1][0] = sx(at(c, SP, 0x70));
        if split_screen {
            rider_pictures(c)?;
        } else {
            course_picture(c)?;
        }
        c.cycles += 10;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

fn rider_pictures(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    c.g[A0][0] = sx(at(c, SP, 0x70));
    c.g[A1][0] = 0x0048_0000;
    c.g[S7][0] = c.g[A0][0];
    c.g[A1][0] = sx(at(c, A1, 0xffff_bd78)); // data/ui/charpic.big
    c.g[A2][0] = 0x100;
    c.g[S2][0] = sx(at(c, S2, TEXTURE_HANDLE));
    call_ptr(c, 0x0031_68b0, 0x0023_30cc)?;
    c.g[S1][0] = 0;
    c.g[V0][0] = 0x0044_0000;
    c.g[S5][0] = 0x0048_0000;
    c.g[S6][0] = sx(at(c, V0, 0x2928)); // Rider-name table.
    c.g[S4][0] = 0x0048_0000;
    loop {
        c.cycles += 29;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = (0 < c.g[S1][0]) as u64;
        call_ptr(c, 0x0014_7488, 0x0023_30ec)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A3][0] = c.lw(at(c, GP, 0xffff_fa4c));
        c.g[V0][0] = sx(at(c, V0, c.g[S6][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.lw(at(c, V0, 0));
        c.g[A1][0] = sx(at(c, S5, 0xffff_bd90)); // |%s%s.ssh
        call_ptr(c, 0x0041_7828, 0x0023_3108)?;
        c.g[A1][0] = 0x0300_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] |= 0x100;
        call_ptr(c, 0x003e_2190, 0x0023_3118)?;
        c.g[T2][0] = c.lw(at(c, GP, 0xffff_f7ac));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(at(c, S0, SSH_ENTRY_OFFSET));
        c.g[A2][0] = sx(at(c, S4, 0xffff_bda0)); // CharaPics
        c.g[V1][0] = c.lw(at(c, T2, GRAPHICS_VTABLE));
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(at(c, S0, c.g[A1][0] as u32));
        c.g[T0][0] = 1;
        c.g[A0][0] = c.lh(at(c, V1, LOAD_SSH_SLOT));
        c.g[T1][0] = u64::MAX;
        c.g[V0][0] = c.lw(at(c, V1, LOAD_SSH_SLOT + 4));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at(c, T2, c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_314c)?;
        let release_data = c.g[S0][0] != 0;
        c.sw(at(c, S2, 0), c.g[V0][0]);
        if release_data {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e98, 0x0023_315c)?;
        }
        c.cycles += 4;
        c.g[S1][0] = sx(at(c, S1, 1));
        c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S2][0] = sx(at(c, S2, 4));
        if !more {
            break;
        }
        if events::event_due(c) {
            events::deliver_event_here(c, 0x0023_30e0);
        }
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = 2;
    call_ptr(c, 0x0031_68e8, 0x0023_3178)?;
    c.lq(S0, at(c, SP, 0x100));
    Ok(())
}

fn course_picture(c: &mut Cpu) -> Flow {
    c.cycles += 40;
    c.g[A1][0] = 0x0048_0000;
    c.g[A1][0] = sx(at(c, A1, 0xffff_bdb0)); // data/ui/courspic.big
    c.g[A2][0] = 0x100;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0031_68b0, 0x0023_3194)?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0014_4be0, 0x0023_319c)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
    c.g[A2][0] = 0x0044_0000;
    c.g[V0][0] = sx(((c.g[V0][0] as u32 as i32) >> 24) as u32);
    c.g[A2][0] = sx(at(c, A2, 0x2950));
    c.g[V1][0] = ((c.g[V0][0] as i64) < 17) as u64;
    c.g[A1][0] = 0x004a_0000;
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    c.g[A0][0] = c.g[SP][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A1][0] = sx(at(c, A1, 0x2c30)); // |%s.ssh
    c.g[V0][0] = sx(at(c, V0, c.g[A2][0] as u32));
    c.g[A2][0] = c.lw(at(c, V0, 0));
    call_ptr(c, 0x0041_7828, 0x0023_31d0)?;
    c.g[A1][0] = 0x0300_0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] |= 0x100;
    call_ptr(c, 0x003e_2190, 0x0023_31e0)?;
    c.g[T2][0] = c.lw(at(c, GP, 0xffff_f7ac));
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = c.lw(at(c, S0, SSH_ENTRY_OFFSET));
    c.g[A2][0] = 0x0048_0000;
    c.g[V1][0] = c.lw(at(c, T2, GRAPHICS_VTABLE));
    c.g[A2][0] = sx(at(c, A2, 0xffff_bdc8)); // coursePics
    c.g[A1][0] = sx(at(c, S0, c.g[A1][0] as u32));
    c.g[A3][0] = 0;
    c.g[A0][0] = c.lh(at(c, V1, LOAD_SSH_SLOT));
    c.g[T0][0] = 1;
    c.g[V0][0] = c.lw(at(c, V1, LOAD_SSH_SLOT + 4));
    c.g[T1][0] = u64::MAX;
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(at(c, T2, c.g[A0][0] as u32));
    call_ptr(c, target, 0x0023_3218)?;
    let release_data = c.g[S0][0] != 0;
    c.sw(at(c, S2, TEXTURE_HANDLE), c.g[V0][0]);
    if release_data {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x0023_3228)?;
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 2;
    call_ptr(c, 0x0031_68e8, 0x0023_3234)?;
    c.lq(S0, at(c, SP, 0x100));
    Ok(())
}
