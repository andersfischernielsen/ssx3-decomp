//! Native replacement of an existing world object's modifier owner.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_ae38,
    attach_modifier,
    "WorldObjFactory_AttachModifier",
)];

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Query the owner's type through the adjusted virtual slot at +0x50.
fn query_type(c: &mut Cpu, kind: u64, back: u32) -> Flow {
    c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0xc));
    c.g[A1][0] = kind;
    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x50));
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x54));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
    call_ptr(c, target, back)
}

fn replacement(
    c: &mut Cpu,
    size: u32,
    name: u32,
    alloc_back: u32,
    ctor: u32,
    ctor_back: u32,
) -> Flow {
    c.cycles += 14;
    c.g[A1][0] = 0x0049_0000;
    c.g[A1][0] = sx(name);
    c.g[A0][0] = size as u64;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, alloc_back)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[T0][0] = c.g[S0][0];
    c.g[A1][0] = 1;
    call_ptr(c, ctor, ctor_back)?;
    c.g[V0][0] = u64::MAX;
    Ok(())
}

/// `int AttachModifier(WorldObj* owner, int kind, void* parameters)`
/// (0x2fae38). Keep compatible owners, refresh an exact type match, or
/// replace the owner with MeshAnim, Particle, Debounce or DeadFade.
/// Other requested types dispose of the old owner and return zero so the
/// caller can construct its replacement. This helper does not bind a collider.
pub fn attach_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 9;
        c.g[V1][0] = c.lw(r(c, S0).wrapping_add(0xc));
        c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x50));
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x54));
        let target = r(c, V0);
        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x002f_ae74)?;

        'finish: {
            let matched = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if matched {
                break 'finish;
            }
            c.cycles += 8;
            query_type(c, 6, 0x002f_ae94)?;
            let matched = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if matched {
                break 'finish;
            }
            c.cycles += 8;
            query_type(c, 0x16, 0x002f_aeb4)?;
            let matched = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if matched {
                break 'finish;
            }
            c.cycles += 8;
            let kind = c.g[S2][0];
            query_type(c, kind, 0x002f_aed4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0xc));
                c.g[A0][0] = c.lh(r(c, V0).wrapping_add(0x128));
                c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x12c));
                let target = r(c, V1);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x002f_aef0)?;
                c.g[V0][0] = 1;
                break 'finish;
            }

            c.g[V0][0] = 0xb;
            c.cycles += 2;
            let mesh_anim = c.g[S2][0] == c.g[V0][0];
            c.g[S1][0] = c.lw(r(c, S0).wrapping_add(0x18));
            if mesh_anim {
                c.cycles += 17;
                c.sw(r(c, S1).wrapping_add(0xc), 0);
                c.g[A1][0] = 0x0049_0000;
                c.g[A1][0] = sx(0x0048_9628);
                c.g[A0][0] = 0x6c0;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x002f_af8c)?;
                c.g[V1][0] = c.lw(r(c, GP).wrapping_add(0xce8));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.lw(r(c, V1).wrapping_add(0x294));
                c.g[T1][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0035_1b40, 0x002f_afac)?;
                c.g[V0][0] = u64::MAX;
                break 'finish;
            }

            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 12) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                if c.g[S2][0] == 0 {
                    c.sw(r(c, S1).wrapping_add(0xc), 0);
                    replacement(c, 0x34, 0x0048_9638, 0x002f_afcc, 0x0034_2c08, 0x002f_afe4)?;
                    break 'finish;
                }
                c.cycles += 2;
            } else {
                c.g[V0][0] = 0xd;
                c.cycles += 2;
                if c.g[S2][0] == c.g[V0][0] {
                    c.sw(r(c, S1).wrapping_add(0xc), 0);
                    replacement(c, 0x2c, 0x0048_9618, 0x002f_af50, 0x0035_78a8, 0x002f_af68)?;
                    break 'finish;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x16;
                if c.g[S2][0] == c.g[V0][0] {
                    c.sw(r(c, S1).wrapping_add(0xc), 0);
                    replacement(c, 0x3c, 0x0048_9648, 0x002f_b004, 0x0035_0750, 0x002f_b01c)?;
                    break 'finish;
                }
                c.cycles += 2;
            }

            c.cycles += 2;
            let no_owner = c.g[S0][0] == 0;
            c.g[A1][0] = 3;
            if !no_owner {
                c.cycles += 5;
                c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0xc));
                c.g[A0][0] = c.lh(r(c, V0).wrapping_add(8));
                c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0xc));
                let target = r(c, V1);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x002f_b040)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
