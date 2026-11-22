//! Native SSH image and palette pointer resolution for texture creation.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0037_c8c0,
    load_texture_from_shape,
    "PS2GraphicsMan_LoadTextureFromShape",
)];

const RELATIVE_DATA: u64 = 0x1000;
const PRESWIZZLED: u64 = 0x2000;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Create a texture from its SSH image header. Flag 0x1000 selects the
/// relative data offset stored at +0x10; otherwise data follows at +0x10.
/// For indexed images flag 0x2000 chooses the pre-swizzled TexMan format.
/// Palette headers follow the image's size field and use the same pointer rule.
pub fn load_texture_from_shape(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[T5][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[T6][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T7][0] = c.g[T1][0];
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        c.g[V1][0] = c.lbu(r(c, S0));
        c.g[V0][0] = 2;
        let eight_bit = c.g[V1][0] == c.g[V0][0];
        c.g[T4][0] = 0;
        if eight_bit {
            c.cycles += 14;
            c.g[V0][0] = c.lw(r(c, S0));
            c.g[T2][0] = 9;
            c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0xc));
            c.g[A2][0] = 10;
            resolve_indexed_header(c);
        } else {
            resolve_other_format(c);
        }

        c.cycles += 2;
        let relative_pixels = c.g[V0][0] != 0;
        c.g[A1][0] = sx(r(c, S0).wrapping_add(0x10));
        if relative_pixels {
            c.cycles += 2;
            c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0x10));
            c.g[A1][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        }

        c.cycles += 19;
        c.g[V1][0] = c.lw(r(c, S2).wrapping_add(0x10d8));
        c.g[A2][0] = c.g[T5][0];
        c.g[A3][0] = c.lh(r(c, S0).wrapping_add(4));
        c.g[T1][0] = sx(r(c, T1) >> 28);
        c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x178));
        c.sw(r(c, SP), c.g[T4][0]);
        c.sw(r(c, SP).wrapping_add(8), c.g[T6][0]);
        c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
        c.sw(r(c, SP).wrapping_add(0x10), c.g[T0][0]);
        c.sw(r(c, SP).wrapping_add(0x18), c.g[T7][0]);
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x17c));
        let create = r(c, V0);
        c.g[RA][0] = sx(0x0037_ca18);
        c.g[T0][0] = c.lh(r(c, S0).wrapping_add(6));
        call_ptr(c, create, 0x0037_ca18)?;

        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xffff_ffff_f000_0000;
        c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0xc));
        c.g[V0][0] &= c.g[V1][0];
        let no_mips = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !no_mips {
            c.cycles += 3;
            c.g[V0][0] = c.lh(r(c, S0).wrapping_add(8));
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.g[S1][0];
            } else {
                set_mipmap_filter(c)?;
            }
        }

        crate::custom_movies::published(c, r(c, S0), r(c, S2), r(c, S1));

        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// The indexed paths have selected T2/A2 as linear/pre-swizzled formats,
/// A1 as image flags, and V0 as the packed image format/size word.
fn resolve_indexed_header(c: &mut Cpu) {
    c.g[V0][0] = sx(((r(c, V0) as i32) >> 8) as u32);
    c.g[T3][0] = 1;
    c.g[V1][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
    c.g[A0][0] = c.g[A1][0] & PRESWIZZLED;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = 0;
    }
    if c.g[A0][0] != 0 {
        c.g[T2][0] = c.g[A2][0];
    }
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xc));
    c.g[V0][0] &= RELATIVE_DATA;
    let relative_palette = c.g[V0][0] != 0;
    c.g[T1][0] = c.g[A1][0];
    if relative_palette {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x10));
        c.g[T4][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    } else {
        c.cycles += 2;
        c.g[T4][0] = sx(r(c, V1).wrapping_add(0x10));
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[T1][0] & RELATIVE_DATA;
}

fn resolve_other_format(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[V1][0] < 3) as u64;
    let above_indexed = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !above_indexed {
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw(r(c, S0));
            c.cycles += 13;
            c.g[T2][0] = 11;
            c.g[A1][0] = c.lw(r(c, S0).wrapping_add(0xc));
            c.g[A2][0] = 12;
            resolve_indexed_header(c);
            return;
        }
        c.cycles += 2;
        c.g[T1][0] = c.lw(r(c, S0).wrapping_add(0xc));
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        let sixteen_bit = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 5;
        if sixteen_bit {
            c.cycles += 3;
            c.g[T2][0] = 2;
            c.g[T1][0] = c.lw(r(c, S0).wrapping_add(0xc));
        } else {
            c.cycles += 2;
            let full_colour = c.g[V1][0] == c.g[V0][0];
            c.g[T1][0] = c.lw(r(c, S0).wrapping_add(0xc));
            if full_colour {
                c.cycles += 1;
                c.g[T2][0] = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0] & RELATIVE_DATA;
                return;
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[T1][0] & RELATIVE_DATA;
}

/// Preserve the original scalar arithmetic, double-precision helper calls
/// and clamp before selecting the texture's mip filter.
fn set_mipmap_filter(c: &mut Cpu) -> Flow {
    c.cycles += 29;
    c.f[12] = FpuF32::from_i32(c.g[V0][0] as u32 as i32).0;
    c.g[AT][0] = 0x3880_0000;
    c.f[0] = 0x3880_0000;
    c.g[AT][0] = 0x4370_0000;
    c.f[1] = 0x4370_0000;
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    c.f[12] = FpuF32(c.f[1]).div(FpuF32(c.f[12])).0;
    call_ptr(c, 0x0041_3af8, 0x0037_ca64)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0040_d610, 0x0037_ca6c)?;
    c.g[AT][0] = 0x0049_0000;
    c.g[A1][0] = c.ld(r(c, AT).wrapping_add(0x2db0));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_1fd8, 0x0037_ca7c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    call_ptr(c, 0x0041_1f70, 0x0037_ca88)?;
    c.g[A1][0] = 0x8060;
    c.g[A1][0] <<= 47;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_1fd8, 0x0037_ca98)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0041_2608, 0x0037_caa0)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < -2047) as u64;
    let below_minimum = c.g[V0][0] != 0;
    c.g[A0][0] = 0xffff_ffff_ffff_ff79;
    if below_minimum {
        c.cycles += 1;
        c.g[T1][0] = 0xffff_ffff_ffff_f801;
    } else {
        c.cycles += 4;
        c.g[T1][0] = c.g[A0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[T1][0] = c.g[V1][0];
        }
    }
    c.cycles += 7;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 5;
    c.g[A3][0] = 1;
    c.g[T0][0] = 0;
    call_ptr(c, 0x0037_d090, 0x0037_cadc)?;
    c.g[V0][0] = c.g[S1][0];
    Ok(())
}
