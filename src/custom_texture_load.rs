//! Native texture ownership, image/CLUT copies and GS texture descriptors.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0036_7440, tex_man_load_texture, "TexMan_LoadTexture")];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

fn invalid_table_target(c: &mut Cpu) -> Flow {
    c.jump_out(r(c, A0), 0x0036_7440, 0x0036_7b60);
    Err(Unwound)
}

/// `int TexMan_LoadTexture(manager, pixels, name, width, height, mips,
/// format, clut_format, clut, residency, owned, bound_id)` (0x367440).
/// Owned textures copy all image levels and a full native CLUT before the
/// caller releases its SSH bank. Format 9 retains the retail temporary-
/// texture / PSMT8H selection; this port does not alter that layout contract.
pub fn tex_man_load_texture(c: &mut Cpu) {
    let label = r(c, A2);
    let capture_exclamation = std::env::var_os("SSX3_ANGER_DUMP").is_some()
        && c.bus.read32(label) == u32::from_le_bytes(*b"exlm");
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[T2][0];
        frame.save(c, S3, 0xa0);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x80);
        c.g[S4][0] = c.g[T3][0];
        frame.save(c, S6, 0x70);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, S7, 0x60);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, FP, 0x50);
        c.g[S7][0] = c.g[A1][0];
        c.g[S0][0] = c.lw(at(c, SP, 0xf8));
        c.g[FP][0] = c.g[A2][0];
        let bound_id = (c.g[S0][0] as i64) >= 0;
        frame.save_ra(c, 0x40);
        c.cycles += 4;
        if bound_id {
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0036_7260, 0x0036_74b0)?;
            c.sw(r(c, SP), c.g[S0][0]);
        } else {
            call_ptr(c, 0x0036_7150, 0x0036_749c)?;
            c.sw(r(c, SP), c.g[V0][0]);
        }

        c.cycles += 19;
        c.g[V1][0] = c.lw(r(c, SP));
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = 8;
        c.g[V0][0] = sx(r(c, V1) << 2);
        c.g[V0][0] = sx(r(c, S6).wrapping_add(r(c, V0)));
        c.g[S0][0] = c.lw(at(c, V0, 8));
        c.g[A0][0] = sx(at(c, S0, 4));
        call_ptr(c, 0x0041_6ce0, 0x0036_74d4)?;
        c.g[V0][0] = c.lw(at(c, SP, 0xf0));
        c.g[V1][0] = u64::MAX;
        c.sw(at(c, S0, 0x34), c.g[V1][0]);
        c.sw(r(c, S0), c.g[V0][0]);
        c.g[V0][0] = (c.g[S1][0] < 13) as u64;
        c.sw(at(c, S0, 0xc), c.g[S1][0]);
        c.sw(at(c, S0, 0x28), c.g[V1][0]);
        c.sw(at(c, S0, 0x30), c.g[V1][0]);
        c.sw(at(c, S0, 0x2c), c.g[V1][0]);
        let known_format = c.g[V0][0] != 0;
        c.sw(at(c, S0, 0x24), c.g[S5][0]);
        select_format(c, known_format)?;
        c.cycles += 2;
        let supported = (c.g[T3][0] as i64) > 0;
        c.sw(at(c, S0, 0x18), c.g[T3][0]);
        if supported {
            image_storage(c)?;
            palette_storage(c)?;
            texture_registers(c);
            residency(c)?;
            finish_linear_format(c)?;
            if capture_exclamation {
                let info = r(c, S0);
                let handle = r(c, V0);
                trace_loaded_exclamation(c, info, handle);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 12;
        for (register, offset) in [
            (S0, 0xd0),
            (S1, 0xc0),
            (S2, 0xb0),
            (S3, 0xa0),
            (S4, 0x90),
            (S5, 0x80),
            (S6, 0x70),
            (S7, 0x60),
            (FP, 0x50),
        ] {
            frame.restore(c, register, offset);
        }
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// Capture the native post-load layout during an explicitly requested
/// windowed asset investigation. No guest registers or memory are changed.
fn trace_loaded_exclamation(c: &mut Cpu, info: u32, handle: u32) {
    let Some(directory) = std::env::var_os("SSX3_ANGER_DUMP") else {
        return;
    };
    let directory = std::path::PathBuf::from(directory).join("loaded");
    let _ = std::fs::create_dir_all(&directory);
    let pixels = c.bus.read32(info + 0x1c);
    let palette = c.bus.read32(info + 0x20);
    let format = c.bus.read32(info + 0xc);
    let tex0 = c.bus.read32(info + 0x38) as u64 | ((c.bus.read32(info + 0x3c) as u64) << 32);
    eprintln!("[anger load] handle={handle:08x} info={info:08x} pixels={pixels:08x} palette={palette:08x} format={format:x} TEX0={tex0:016x}");
    for (name, address, size) in [
        ("info.bin", info, 0x58),
        ("pixels.bin", pixels, 128 * 128),
        ("palette.bin", palette, 1024),
    ] {
        if address == 0 {
            continue;
        }
        let bytes: Vec<u8> = (0..size).map(|i| c.bus.read8(address + i)).collect();
        let _ = std::fs::write(directory.join(name), bytes);
    }
}

fn select_format(c: &mut Cpu, known: bool) -> Flow {
    if known {
        c.cycles += 7;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = sx(r(c, S1) << 2);
        c.g[V0][0] = sx(at(c, V0, 0x2090));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(r(c, V1));
        match r(c, A0) {
            0x0036_751c => {
                c.cycles += 2;
                c.g[A1][0] = 1;
            }
            0x0036_7524 => {
                c.cycles += 2;
                c.g[A1][0] = 2;
            }
            0x0036_752c => {
                c.cycles += 2;
                c.g[A1][0] = 0x13;
            }
            0x0036_7534 => {
                c.cycles += 2;
                c.g[A1][0] = 0x14;
            }
            0x0036_753c => {
                c.cycles += 1;
                c.g[A1][0] = 0;
            }
            _ => return invalid_table_target(c),
        }
    } else {
        c.cycles += 1;
        c.g[A1][0] = 0;
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x3b) as u64;
    let known_psm = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0049_0000;
    if known_psm {
        c.cycles += 6;
        c.g[V1][0] = sx(r(c, A1) << 2);
        c.g[V0][0] = sx(at(c, V0, 0x20d0));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(r(c, V1));
        match r(c, A0) {
            0x0036_7564 => {
                c.cycles += 2;
                c.g[T3][0] = 32;
            }
            0x0036_756c => {
                c.cycles += 2;
                c.g[T3][0] = 16;
            }
            0x0036_7574 => {
                c.cycles += 2;
                c.g[T3][0] = 8;
            }
            0x0036_757c => {
                c.cycles += 2;
                c.g[T3][0] = 4;
            }
            0x0036_7584 => {
                c.cycles += 1;
                c.g[T3][0] = 0;
            }
            _ => return invalid_table_target(c),
        }
    } else {
        c.cycles += 1;
        c.g[T3][0] = 0;
    }
    Ok(())
}

fn image_pages(c: &mut Cpu) -> Flow {
    if c.g[A3][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = sx(r(c, T0) << 2);
        c.g[V0][0] = sx(at(c, V0, 0x21c0));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(r(c, V1));
        match r(c, A0) {
            0x0036_7640 => {
                c.cycles += 3;
                c.g[A1][0] = 128;
                c.g[V1][0] = 128;
            }
            0x0036_764c => {
                c.cycles += 3;
                c.g[A1][0] = 128;
                c.g[V1][0] = 64;
            }
            0x0036_7658 => {
                c.cycles += 3;
                c.g[A1][0] = 64;
                c.g[V1][0] = 64;
            }
            0x0036_7664 => {
                c.cycles += 2;
                c.g[A1][0] = 64;
                c.g[V1][0] = 32;
            }
            _ => return invalid_table_target(c),
        }
    } else {
        c.cycles += 2;
        c.g[A1][0] = 64;
        c.g[V1][0] = 32;
    }
    Ok(())
}

fn empty_pages(c: &mut Cpu) -> Flow {
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = sx(r(c, V1) << 2);
        c.g[V0][0] = sx(at(c, V0, 0x2240));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(r(c, V1));
        match r(c, A0) {
            0x0036_7734 => {
                c.cycles += 3;
                c.g[A2][0] = 128;
                c.g[A0][0] = 128;
            }
            0x0036_7740 => {
                c.cycles += 3;
                c.g[A2][0] = 128;
                c.g[A0][0] = 64;
            }
            0x0036_774c => {
                c.cycles += 3;
                c.g[A2][0] = 64;
                c.g[A0][0] = 64;
            }
            0x0036_7758 => {
                c.cycles += 2;
                c.g[A2][0] = 64;
                c.g[A0][0] = 32;
            }
            _ => return invalid_table_target(c),
        }
    } else {
        c.cycles += 2;
        c.g[A2][0] = 64;
        c.g[A0][0] = 32;
    }
    Ok(())
}

fn image_storage(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let has_pixels = c.g[S7][0] != 0;
    c.g[V0][0] = sx(r(c, S3).wrapping_sub(1));
    if has_pixels {
        c.cycles += 16;
        c.sd(at(c, SP, 0x18), c.g[A1][0]);
        c.g[V1][0] = sx(r(c, S2).wrapping_sub(1));
        c.g[A1][0] = sx(at(c, S0, 0x38));
        c.sw(at(c, SP, 0x20), c.g[V0][0]);
        c.sw(at(c, SP, 8), c.g[A1][0]);
        c.g[V0][0] = ((c.g[T3][0] as i64) < 9) as u64;
        c.sw(at(c, SP, 0x30), c.g[V1][0]);
        c.g[A1][0] = sx(at(c, S3, 0x3f));
        c.g[V1][0] = c.g[S5][0] << 32;
        c.g[S1][0] = 0;
        c.g[T5][0] = 0;
        c.g[T4][0] = 0;
        c.sw(at(c, SP, 4), c.g[A1][0]);
        c.sw(at(c, SP, 0xc), c.g[V0][0]);
        let has_levels = (c.g[S5][0] as i64) >= 0;
        c.sd(at(c, SP, 0x10), c.g[V1][0]);
        if has_levels {
            loop {
                c.cycles += 17;
                c.g[T1][0] = sx(r(c, S3) >> (r(c, T4) & 31));
                c.g[T2][0] = sx(r(c, S2) >> (r(c, T4) & 31));
                c.mult(0, V1, T1, T2);
                c.g[A0][0] = u64::MAX;
                c.g[A1][0] = 255;
                c.g[A2][0] = 256;
                c.g[T0][0] = sx(r(c, T3).wrapping_sub(4));
                c.g[A3][0] = (c.g[T0][0] < 29) as u64;
                c.mult(0, V1, V1, T3);
                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx(at(c, V1, 7));
                if c.g[A0][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx(((r(c, V0) as i32) >> 3) as u32);
                c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = c.g[A2][0];
                }
                c.g[S1][0] = sx(r(c, S1).wrapping_add(r(c, V0)));
                image_pages(c)?;
                c.cycles += 18;
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[T1][0];
                }
                if c.g[A0][0] == 0 {
                    c.g[V1][0] = c.g[T2][0];
                }
                c.mult(0, V1, A1, V1);
                c.g[A0][0] = u64::MAX;
                c.g[T4][0] = sx(at(c, T4, 1));
                c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[T4][0] as i64)) as u64;
                c.mult(0, V1, V1, T3);
                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx(at(c, V1, 7));
                if c.g[A0][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx(((r(c, V0) as i32) >> 3) as u32);
                c.g[V1][0] = sx(0u32.wrapping_sub(r(c, V0)));
                c.g[V1][0] &= 255;
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                let more = c.g[A1][0] == 0;
                c.g[T5][0] = sx(r(c, T5).wrapping_add(r(c, V0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_75e0);
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(r(c, S0));
        c.sw(at(c, S0, 0x14), c.g[T5][0]);
        let owned = c.g[V0][0] != 0;
        c.sw(at(c, S0, 0x10), c.g[S1][0]);
        if owned {
            c.cycles += 14;
            c.g[V0][0] = c.lw(at(c, S6, 0x1f58));
            c.g[A2][0] = 0x0500_0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[FP][0];
            c.g[A2][0] |= c.g[V0][0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0036_76e0)?;
            c.g[A1][0] = c.g[S7][0];
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.sw(at(c, S0, 0x1c), c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x0036_76f4)?;
            c.g[T3][0] = c.lw(at(c, SP, 0xe0));
            return Ok(());
        }
        c.cycles += 2;
        c.sw(at(c, S0, 0x1c), c.g[S7][0]);
    } else {
        c.cycles += 5;
        c.g[V1][0] = sx(r(c, T3).wrapping_sub(4));
        c.sw(at(c, S0, 0x1c), 0);
        c.g[V0][0] = (c.g[V1][0] < 29) as u64;
        c.sw(at(c, S0, 0x10), 0);
        empty_pages(c)?;
        c.cycles += 28;
        c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[V1][0] == 0 {
            c.g[A0][0] = c.g[S2][0];
        }
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[S3][0];
        }
        c.mult(0, A0, A2, A0);
        c.g[V0][0] = sx(at(c, S0, 0x38));
        c.sw(at(c, SP, 8), c.g[V0][0]);
        c.g[V0][0] = sx(at(c, S3, 0x3f));
        c.sd(at(c, SP, 0x18), c.g[A1][0]);
        c.sw(at(c, SP, 4), c.g[V0][0]);
        c.g[V1][0] = u64::MAX;
        c.mult(0, A0, A0, T3);
        c.g[A1][0] = ((c.g[T3][0] as i64) < 9) as u64;
        c.sw(at(c, SP, 0xc), c.g[A1][0]);
        c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
        c.g[A1][0] = sx(r(c, S3).wrapping_sub(1));
        c.sw(at(c, SP, 0x30), c.g[S2][0]);
        c.sw(at(c, SP, 0x20), c.g[A1][0]);
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx(at(c, A0, 7));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.g[A1][0] = c.g[S5][0] << 32;
        c.g[V0][0] = sx(((r(c, V0) as i32) >> 3) as u32);
        c.sd(at(c, SP, 0x10), c.g[A1][0]);
        c.g[V1][0] = sx(0u32.wrapping_sub(r(c, V0)));
        c.g[V1][0] &= 255;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.sw(at(c, S0, 0x14), c.g[V0][0]);
    }
    c.cycles += 1;
    c.g[T3][0] = c.lw(at(c, SP, 0xe0));
    Ok(())
}

fn palette_storage(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let has_clut = c.g[T3][0] != 0;
    c.g[V0][0] = 1;
    let mut copy_palette = false;
    if has_clut {
        c.cycles += 2;
        let rgba = c.g[S4][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
        if rgba {
            c.cycles += 1;
            c.g[S2][0] = 4;
        } else {
            c.cycles += 2;
            let below_16 = c.g[V0][0] != 0;
            c.g[S2][0] = 0;
            if !below_16 {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if c.g[S4][0] == c.g[V0][0] {
                    c.g[S2][0] = 2;
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        let valid_clut = c.g[S2][0] != 0;
        c.g[V1][0] = 1;
        copy_palette = valid_clut;
    }
    if copy_palette {
        c.cycles += 8;
        c.g[V0][0] = c.lw(at(c, S0, 0xc));
        c.g[S1][0] = 256;
        c.g[A1][0] = 16;
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(11));
        c.g[A0][0] = c.lw(r(c, S0));
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        let owned = c.g[A0][0] != 0;
        if c.g[V1][0] == 0 {
            c.g[S1][0] = c.g[A1][0];
        }
        if owned {
            c.cycles += 15;
            c.g[V0][0] = c.lw(at(c, S6, 0x1f58));
            c.g[A2][0] = 0x0500_0000;
            c.g[A1][0] = c.g[FP][0];
            c.g[A0][0] = 1024;
            c.g[A2][0] |= c.g[V0][0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0036_7848)?;
            c.mult(0, ZERO, S1, S2);
            c.g[A1][0] = c.lw(at(c, SP, 0xe0));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.lo[0];
            c.sw(at(c, S0, 0x20), c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x0036_7860)?;
            c.g[T0][0] = 0;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(at(c, SP, 0xe0));
        c.sw(at(c, S0, 0x20), c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.sw(at(c, S0, 0x20), 0);
    }
    c.cycles += 1;
    c.g[T0][0] = 0;
    Ok(())
}

fn texture_registers(c: &mut Cpu) {
    c.cycles += 4;
    c.g[T2][0] = 0;
    c.g[T1][0] = 0xffff_ffff_ffff_ff47;
    let has_mips = c.g[S5][0] != 0;
    c.g[A3][0] = 5;
    if !has_mips {
        c.cycles += 4;
        c.g[T0][0] = 1;
        c.g[T2][0] = c.g[S5][0];
        c.g[T1][0] = 0;
        c.g[A3][0] = 1;
    }
    c.cycles += 19;
    c.g[V1][0] = c.lw(at(c, SP, 0x20));
    c.g[V0][0] = (c.g[V1][0] << 32) >> 32;
    c.plzcw(V0, V0);
    c.g[V0][0] ^= 31;
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
    c.g[A1][0] = c.lw(at(c, SP, 0x30));
    c.g[A0][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64;
    c.g[V1][0] = (c.g[A1][0] << 32) >> 32;
    c.plzcw(V1, V1);
    c.g[V1][0] ^= 31;
    c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
    c.g[V1][0] = ((c.g[V1][0] << 32) as i64 >> 32) as u64;
    c.g[V0][0] = 1;
    let rgba = c.g[S4][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
    if rgba {
        c.cycles += 1;
        c.g[A2][0] = 0;
    } else {
        c.cycles += 2;
        let below_16 = c.g[V0][0] != 0;
        c.g[A2][0] = 0;
        if !below_16 {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let is_16 = c.g[S4][0] == c.g[V0][0];
            c.g[A2][0] = 2;
            if !is_16 {
                c.cycles += 1;
                c.g[A2][0] = 0;
            }
        }
    }
    c.cycles += 21;
    c.g[T3][0] = c.lw(at(c, SP, 4));
    c.g[A0][0] = sx(at(c, A0, 1));
    c.g[A0][0] <<= 26;
    c.g[V1][0] = sx(at(c, V1, 1));
    c.g[V0][0] = sx(r(c, T3) >> 6);
    c.g[V1][0] <<= 30;
    c.g[T3][0] = c.ld(at(c, SP, 0x18));
    c.g[V0][0] = (c.g[V0][0] << 32) >> 18;
    c.g[A1][0] = c.g[T3][0] << 20;
    c.g[V0][0] |= c.g[A1][0];
    c.g[V0][0] |= c.g[A0][0];
    c.g[V0][0] |= c.g[V1][0];
    c.g[A0][0] = 0x8000 << 19;
    c.g[V0][0] |= c.g[A0][0];
    c.g[V1][0] = c.g[A2][0] << 51;
    c.g[A2][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.lw(at(c, SP, 0xc));
    let indexed = c.g[V0][0] != 0;
    c.g[A1][0] = c.ld(at(c, SP, 0x10));
    if indexed {
        c.cycles += 3;
        c.g[V0][0] = 0x8000 << 46;
        c.g[A2][0] |= c.g[V0][0];
    }
    c.cycles += 15;
    c.g[A0][0] = c.g[A3][0] << 6;
    c.g[V0][0] = c.g[T2][0];
    c.g[T3][0] = c.lw(at(c, SP, 8));
    c.g[V1][0] = c.g[A1][0] >> 30;
    c.g[V0][0] <<= 19;
    c.g[A1][0] = 32;
    c.g[V1][0] |= c.g[T0][0];
    c.g[A0][0] |= c.g[A1][0];
    c.sd(r(c, T3), c.g[A2][0]);
    c.g[V1][0] |= c.g[A0][0];
    c.g[V1][0] |= c.g[V0][0];
    c.g[A0][0] = c.g[T1][0] << 32;
    c.g[V1][0] |= c.g[A0][0];
    let no_mips = c.g[S5][0] == 0;
    c.sd(at(c, S0, 0x40), c.g[V1][0]);
    if no_mips {
        return;
    }
    c.cycles += 26;
    c.g[V0][0] = sx(at(c, S3, 0x7f));
    c.g[V1][0] = sx(at(c, S3, 0x3ff));
    c.g[V0][0] = sx(r(c, V0) >> 7);
    c.g[V1][0] = sx(r(c, V1) >> 10);
    c.g[A2][0] = sx(at(c, S3, 0xff));
    c.g[A3][0] = sx(at(c, S3, 0x7ff));
    c.g[V0][0] <<= 32;
    c.g[A2][0] = sx(r(c, A2) >> 8);
    c.g[V1][0] <<= 32;
    c.g[A3][0] = sx(r(c, A3) >> 11);
    c.g[A1][0] = sx(at(c, S3, 0xfff));
    c.g[A0][0] = sx(at(c, S3, 0x1ff));
    c.g[V0][0] >>= 18;
    c.g[A2][0] <<= 34;
    c.g[A0][0] = sx(r(c, A0) >> 9);
    c.g[V1][0] >>= 18;
    c.g[A3][0] <<= 34;
    c.g[A1][0] = sx(r(c, A1) >> 12);
    c.g[V0][0] |= c.g[A2][0];
    c.g[V1][0] |= c.g[A3][0];
    c.g[A0][0] <<= 54;
    c.g[A1][0] <<= 54;
    c.g[V0][0] |= c.g[A0][0];
    c.g[V1][0] |= c.g[A1][0];
    c.sd(at(c, S0, 0x48), c.g[V0][0]);
    c.sd(at(c, S0, 0x50), c.g[V1][0]);
}

fn residency(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = c.lw(at(c, SP, 0xe8));
    c.g[V0][0] = 1;
    let pinned = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = c.lw(at(c, SP, 0xe8));
    if pinned {
        c.cycles += 18;
        c.g[A1][0] = c.lw(at(c, S0, 0x14));
        c.g[A0][0] = sx(at(c, S6, 0x1f60));
        c.g[A2][0] = c.lw(r(c, SP));
        c.g[A3][0] = u64::MAX;
        c.g[T0][0] = u64::MAX;
        c.g[T1][0] = 1;
        call_ptr(c, 0x0036_6ce0, 0x0036_7a30)?;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = c.lw(at(c, S6, 0x3f50));
        c.mult(0, A0, V0, A0);
        c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        c.g[V1][0] = c.lw(r(c, A1));
        c.g[V1][0] |= 2;
        c.sw(r(c, A1), c.g[V1][0]);
        c.g[V1][0] = sx(((r(c, V1) as i32) >> 8) as u32);
        c.sw(at(c, S0, 0x30), c.g[V0][0]);
        c.sw(at(c, S0, 0x28), c.g[V1][0]);
        c.cycles += 1;
        c.g[V0][0] = c.lw(r(c, S0));
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    if c.g[A1][0] != c.g[V0][0] {
        c.g[V0][0] = c.lw(r(c, S0));
        return Ok(());
    }
    c.cycles += 21;
    c.g[A1][0] = c.lw(at(c, S0, 0x14));
    c.g[A0][0] = sx(at(c, S6, 0x1f60));
    c.g[A2][0] = c.lw(r(c, SP));
    c.g[A3][0] = u64::MAX;
    c.g[A1][0] = sx(at(c, A1, 0x2000));
    c.g[T0][0] = u64::MAX;
    c.g[T1][0] = 1;
    call_ptr(c, 0x0036_6ce0, 0x0036_7a88)?;
    c.g[A0][0] = 0x1c;
    c.g[A1][0] = c.lw(at(c, S6, 0x3f50));
    c.mult(0, A0, V0, A0);
    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
    c.g[V1][0] = c.lw(r(c, A1));
    c.g[V1][0] |= 0x22;
    c.g[A2][0] = sx(((r(c, V1) as i32) >> 8) as u32);
    c.sw(r(c, A1), c.g[V1][0]);
    c.g[A0][0] = sx(0u32.wrapping_sub(r(c, A2)));
    c.sw(at(c, S0, 0x30), c.g[V0][0]);
    c.g[A0][0] &= 31;
    c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
    c.sw(at(c, S0, 0x28), c.g[A2][0]);
    c.cycles += 1;
    c.g[V0][0] = c.lw(r(c, S0));
    Ok(())
}

fn finish_linear_format(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let owned = c.g[V0][0] != 0;
    c.g[V0][0] = 9;
    if owned {
        c.cycles += 3;
        c.g[V1][0] = c.lw(at(c, S0, 0xc));
        let linear = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.lw(r(c, SP));
        if !linear {
            return Ok(());
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(at(c, S6, 0x6344));
        c.g[V1][0] = c.lw(at(c, S0, 0x14));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let needs_temp = c.g[V0][0] != 0;
        c.g[T3][0] = c.lw(at(c, SP, 0xe8));
        if needs_temp {
            c.cycles += 2;
            if c.g[T3][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0036_6e90, 0x0036_7afc)?;
                c.g[V0][0] = c.lw(r(c, SP));
                return Ok(());
            }
            c.g[V0][0] = c.ld(at(c, S0, 0x38));
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.ld(at(c, S0, 0x38));
        }
    } else {
        c.cycles += 3;
        c.g[V1][0] = c.lw(at(c, S0, 0xc));
        let linear = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.lw(r(c, SP));
        if !linear {
            return Ok(());
        }
        c.cycles += 1;
        c.g[V0][0] = c.ld(at(c, S0, 0x38));
    }
    c.cycles += 7;
    c.g[V1][0] = 0xffff_ffff_fc0f_0000;
    c.g[V1][0] |= 0xffff;
    c.g[A0][0] = 0x01b0_0000;
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A0][0];
    c.sd(at(c, S0, 0x38), c.g[V0][0]);
    c.g[V0][0] = c.lw(r(c, SP));
    Ok(())
}
