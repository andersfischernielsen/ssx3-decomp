//! Camera-centered native sky-instance rendering.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0035_3b10, sky_box_render, "SkyBox_Render")];

const SKY_INSTANCE: u32 = 0x14;
const IDENTITY: u32 = 0x004f_f1a0;
const MATERIAL_TOP: u32 = 0xe84;
const GRAPHICS_VTABLE: u32 = 0x10d8;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void SkyBox::Render()` (0x353b10).
/// Render the instance already selected at `self+0x14`. Its draw matrix is
/// identity translated to the current camera eye, with uniform scale 1.0;
/// geometry size therefore comes from the model, not an extra sky scale.
/// The render-state push and draw flags 0x5420 are the retail sky path.
pub fn sky_box_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw(r(c, GP).wrapping_add(0x143c));
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        let disabled = c.g[V0][0] != 0;
        frame.save_ra(c, 0x50);

        'render: {
            if disabled {
                break 'render;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(r(c, S1).wrapping_add(SKY_INSTANCE));
            let missing_instance = c.g[V0][0] == 0;
            c.g[S0][0] = c.lw(r(c, GP).wrapping_add(0x2a90));
            if missing_instance {
                break 'render;
            }

            c.cycles += 74;
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx(IDENTITY);
            c.lqc2(1, r(c, V0));
            c.lqc2(2, r(c, V0).wrapping_add(0x10));
            c.lqc2(3, r(c, V0).wrapping_add(0x20));
            c.lqc2(4, r(c, V0).wrapping_add(0x30));
            c.sqc2(1, r(c, SP));
            c.sqc2(2, r(c, SP).wrapping_add(0x10));
            c.sqc2(3, r(c, SP).wrapping_add(0x20));
            c.sqc2(4, r(c, SP).wrapping_add(0x30));
            c.g[S3][0] = 0xffff_ffff_ffff_fc1f;
            call_ptr(c, 0x002d_1c58, 0x0035_3b70)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002d_1c20, 0x0035_3b78)?;

            c.lq(V1, r(c, V0));
            c.g[A1][0] = 0xffff_ffff_ffbf_0000;
            c.g[A1][0] |= 0xffff;
            c.g[T2][0] = 0x0040_0000;
            c.sq(V1, r(c, SP).wrapping_add(0x30));
            c.g[A2][0] = 0xffff_ffff_fe7f_0000;
            c.sq(V1, r(c, SP).wrapping_add(0x40));
            c.g[A2][0] |= 0xffff;
            c.g[T3][0] = 0x0100_0000;
            c.g[A3][0] = 0xffff_ffff_ffcf_0000;
            c.g[V0][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[A3][0] |= 0xffff;
            c.g[T0][0] = 0xffff_ffff_fff0_0000;
            c.g[T1][0] = 0x0001_0000;

            // The retail material stack has a 20-byte stride. Preserve the
            // original unaligned 64-bit transfers and their register state.
            c.ldl(V1, r(c, V0).wrapping_add(7));
            c.ldr(V1, r(c, V0));
            c.ldl(A0, r(c, V0).wrapping_add(0xf));
            c.ldr(A0, r(c, V0).wrapping_add(8));
            c.g[T4][0] = c.lw(r(c, V0).wrapping_add(0x10));
            c.sdl(V1, r(c, V0).wrapping_add(0x1b));
            c.sdr(V1, r(c, V0).wrapping_add(0x14));
            c.sdl(A0, r(c, V0).wrapping_add(0x23));
            c.sdr(A0, r(c, V0).wrapping_add(0x1c));
            c.sw(r(c, V0).wrapping_add(0x24), c.g[T4][0]);
            c.g[T0][0] |= 0xfff;
            c.g[T1][0] |= 0x4000;
            c.g[A0][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[V1][0] = sx(r(c, A0).wrapping_add(0x14));
            c.sw(r(c, S0).wrapping_add(MATERIAL_TOP), c.g[V1][0]);
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x1c));
            c.g[V1][0] = c.g[V0][0] & c.g[S3][0];
            c.sw(r(c, A0).wrapping_add(0x1c), c.g[V1][0]);
            c.g[V0][0] &= 0x3e0;
            c.g[S2][0] = sx(r(c, V0) >> 5);

            c.g[V1][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, V1));
            c.g[V0][0] |= 0xc;
            c.sw(r(c, V1), c.g[V0][0]);
            c.g[A0][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(4));
            c.g[V0][0] &= c.g[A1][0];
            c.g[V0][0] |= c.g[T2][0];
            c.sw(r(c, A0).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, A1).wrapping_add(4));
            c.g[V0][0] &= c.g[A2][0];
            c.g[V0][0] |= c.g[T3][0];
            c.sw(r(c, A1).wrapping_add(4), c.g[V0][0]);
            c.g[V1][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, V1).wrapping_add(4));
            c.g[V0][0] &= c.g[A3][0];
            c.sw(r(c, V1).wrapping_add(4), c.g[V0][0]);
            c.g[A0][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(4));
            c.g[V0][0] &= c.g[T0][0];
            c.g[V0][0] |= c.g[T1][0];
            c.sw(r(c, A0).wrapping_add(4), c.g[V0][0]);

            c.g[A0][0] = c.lw(r(c, S1).wrapping_add(SKY_INSTANCE));
            let missing_before_draw = c.g[A0][0] == 0;
            c.g[A3][0] = c.g[SP][0];
            if !missing_before_draw {
                c.cycles += 19;
                c.g[V1][0] = c.lw(r(c, A0).wrapping_add(8));
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] |= 0xfffd;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 12);
                c.g[V1][0] &= c.g[V0][0];
                c.g[V1][0] |= 4;
                c.g[T0][0] = 0;
                c.sw(r(c, A0).wrapping_add(8), c.g[V1][0]);
                c.g[T1][0] = 0;
                c.g[T2][0] = 0x5420;
                c.g[A1][0] = c.lw(r(c, S1).wrapping_add(SKY_INSTANCE));
                c.g[V0][0] = c.lw(r(c, S0).wrapping_add(GRAPHICS_VTABLE));
                c.g[T3][0] = c.lw(r(c, A1).wrapping_add(0x80));
                c.g[A0][0] = c.lh(r(c, V0).wrapping_add(0x2f8));
                c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x2fc));
                c.g[A2][0] = c.lw(r(c, T3).wrapping_add(0xc));
                let draw_model = r(c, V1);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                call_ptr(c, draw_model, 0x0035_3cb4)?;
            }

            c.cycles += 8;
            c.g[A0][0] = c.lw(r(c, S0).wrapping_add(MATERIAL_TOP));
            c.g[A1][0] = sx(r(c, S2) << 5);
            c.g[V1][0] = sx(r(c, A0).wrapping_sub(0x14));
            c.sw(r(c, S0).wrapping_add(MATERIAL_TOP), c.g[V1][0]);
            c.g[V0][0] = c.lw(r(c, A0).wrapping_sub(0xc));
            c.g[V0][0] &= c.g[S3][0];
            c.g[V0][0] |= c.g[A1][0];
            c.sw(r(c, A0).wrapping_sub(0xc), c.g[V0][0]);
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });
}
