//! Staging module oc750 (wave wave243): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc750`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_6f40,
        cworld_sound_get_shape_bounds,
        "cWorldSound_CalcAudioVolumeBox",
    ),
    (
        0x002d_f4d0,
        cdynamic_spray_fx_find_bones,
        "cDynamicSprayFX_FindBones",
    ),
    (
        0x002e_ccb8,
        cbolt_on_fx_interface_refresh_emitters,
        "cBoltOnFXInterface_RefreshEmitters",
    ),
];

/// `void cWorldSound_CalcAudioVolumeBox(cWorldSound* self, int* shape, Vec3* lo, Vec3* hi)`
/// (0x2b6f40): the axis-aligned box of a sound volume shape. Shape record: type at
/// +0, centre at +8, extents at +0x14/+0x18/+0x1c, axis at +0x20. Types 0, 2 and 3
/// (sphere-like): centre -/+ e0 on every axis. Type 1 (oriented box): an orthonormal
/// frame from the axis and z, box = centre -/+ |axis*e0| + |side*e1| + |up*e2| as signed
/// sums. Other types leave the outputs alone.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cworld_sound_get_shape_bounds(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002b6f40);
    'b_2b730c: {
        'b_2b72b0: {
            'b_2b72ac: {
                'b_2b6f84: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    if t {
                        break 'b_2b6f84;
                    } else {
                        'b_2b6f6c: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = 0x2;
                                break 'b_2b6f6c;
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == 0u64 {
                                    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
                                    break 'b_2b72b0;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_2b730c;
                                    } else {
                                        break 'b_2b6f6c;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 0x3;
                        if t {
                            break 'b_2b72ac;
                        } else {
                            c.cycles += 2;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
                                break 'b_2b72b0;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_2b730c;
                                } else {
                                    break 'b_2b6f84;
                                }
                            }
                        }
                    }
                }
                c.cycles += 202;
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x24));
                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x28));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 7);
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x44), 0u64);
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V1][0]);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.mul_s(6, 2, 1);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(5, (c.g[SP][0] as u32));
                c.mul_s(4, 3, 8);
                c.mul_s(1, 0, 1);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 8);
                c.mul_s(2, 2, 5);
                c.sub_s(4, 4, 6);
                c.sub_s(1, 1, 3);
                c.sub_s(2, 2, 0);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.mul_s(0, 4, 4);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.mul_s(2, 5, 5);
                c.mul_s(3, 1, 1);
                c.add_s(0, 0, 2);
                c.add_s(0, 0, 3);
                c.sqrt_s(0, 0);
                c.div_s(7, 7, 0);
                c.mul_s(1, 1, 7);
                c.mul_s(4, 4, 7);
                c.mul_s(5, 5, 7);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.mul_s(6, 2, 1);
                c.lwc1(0, (c.g[SP][0] as u32));
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.mul_s(4, 3, 7);
                c.mul_s(1, 0, 1);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 7);
                c.mul_s(2, 2, 5);
                c.sub_s(4, 4, 6);
                c.sub_s(1, 1, 3);
                c.sub_s(2, 2, 0);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[A2][0] as u32));
                c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.lwc1(10, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[A3][0] as u32));
                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x14));
                c.mul_s(1, 12, 0);
                c.lwc1(6, (c.g[SP][0] as u32));
                c.mul_s(2, 10, 0);
                c.lwc1(5, (c.g[A1][0] as u32).wrapping_add(0x1c));
                c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 11, 0);
                c.mul_s(8, 9, 3);
                c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(15, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.mul_s(4, 6, 3);
                c.lwc1(16, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.mul_s(3, 7, 3);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.add_s(8, 8, 0);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.mul_s(13, 16, 5);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.add_s(4, 4, 1);
                c.mul_s(0, 14, 5);
                c.mul_s(5, 15, 5);
                c.add_s(3, 3, 2);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.add_s(4, 4, 0);
                c.add_s(8, 8, 13);
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.add_s(3, 3, 5);
                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[A2][0] as u32));
                c.sub_s(2, 2, 8);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x84));
                c.sub_s(0, 0, 4);
                c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x88));
                c.sub_s(1, 1, 3);
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A2][0] as u32));
                c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x14));
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.mul_s(6, 6, 0);
                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x1c));
                c.mul_s(7, 7, 0);
                c.lwc1(4, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.mul_s(9, 9, 0);
                c.lwc1(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.mul_s(12, 12, 1);
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.mul_s(10, 10, 1);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.mul_s(11, 11, 1);
                c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.add_s(6, 6, 12);
                c.lwc1(5, (c.g[A3][0] as u32));
                c.add_s(7, 7, 10);
                c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.add_s(9, 9, 11);
                c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.mul_s(16, 16, 2);
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(14, 14, 2);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.mul_s(15, 15, 2);
                c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x98));
                c.add_s(0, 9, 16);
                c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.add_s(6, 6, 14);
                c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.add_s(7, 7, 15);
                c.swc1(15, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.add_s(3, 3, 0);
                c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xa8));
                c.add_s(5, 5, 6);
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.add_s(4, 4, 7);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(5, (c.g[A3][0] as u32));
                c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x4));
                let t = true;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                if t {
                    break 'b_2b730c;
                } else {
                    break 'b_2b72ac;
                }
            }
            c.cycles += 1;
            c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
            break 'b_2b72b0;
        }
        c.cycles += 23;
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sub_s(0, 0, 1);
        c.swc1(0, (c.g[A2][0] as u32));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x14));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0xc));
        c.sub_s(1, 1, 0);
        c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.sub_s(0, 0, 1);
        c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x14));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.add_s(1, 1, 0);
        c.swc1(1, (c.g[A3][0] as u32));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
        c.add_s(0, 0, 1);
        c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
        c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.add_s(0, 0, 1);
        c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
        break 'b_2b730c;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.pc = t;
    return;
}

/// Where `cDynamicSprayFX` keeps its bone-index array pointer.
pub(crate) const SPRAY_BONES: u32 = 0x7c;

/// The second argument of a `getBoneSkelIndex` call: a bone-name string literal
/// (`a2 = lit`) or the callee-saved register that already holds it.
#[derive(Clone, Copy)]
pub(crate) enum BoneA2 {
    Lit(u32),
    Reg(usize),
}

/// `getBoneSkelIndex(self->mRider->mAnimModel, 0, name)` calls of `FindBones`, in order:
/// return address, the callee-saved register the original loads the name into (if
/// any), and the name argument. 30 calls fill `mBoneIndicies[0..30]`.
pub(crate) const SPRAY_BONE_CALLS: [(u32, Option<(usize, u32)>, BoneA2); 30] = [
    (0x002df518, None, BoneA2::Lit(0x4879d8)),
    (0x002df53c, Some((S7, 0x4879e8)), BoneA2::Reg(S7)),
    (0x002df560, Some((S6, 0x4879f8)), BoneA2::Reg(S6)),
    (0x002df584, Some((S5, 0x487a08)), BoneA2::Reg(S5)),
    (0x002df5a8, Some((S4, 0x487a18)), BoneA2::Reg(S4)),
    (0x002df5cc, Some((S3, 0x487a28)), BoneA2::Reg(S3)),
    (0x002df5f0, Some((S2, 0x4a3b00)), BoneA2::Reg(S2)),
    (0x002df614, Some((S1, 0x487a38)), BoneA2::Reg(S1)),
    (0x002df638, Some((S0, 0x487a48)), BoneA2::Reg(S0)),
    (0x002df658, None, BoneA2::Lit(0x487a58)),
    (0x002df678, None, BoneA2::Lit(0x4a3b08)),
    (0x002df698, None, BoneA2::Lit(0x487a68)),
    (0x002df6b8, None, BoneA2::Lit(0x487a78)),
    (0x002df6d8, None, BoneA2::Lit(0x487a88)),
    (0x002df6f8, None, BoneA2::Lit(0x487a98)),
    (0x002df718, None, BoneA2::Lit(0x4879d8)),
    (0x002df734, None, BoneA2::Reg(S7)),
    (0x002df750, None, BoneA2::Reg(S6)),
    (0x002df76c, None, BoneA2::Reg(S5)),
    (0x002df788, None, BoneA2::Reg(S4)),
    (0x002df7a4, None, BoneA2::Reg(S3)),
    (0x002df7c0, None, BoneA2::Reg(S2)),
    (0x002df7dc, None, BoneA2::Reg(S1)),
    (0x002df7f8, None, BoneA2::Reg(S0)),
    (0x002df818, None, BoneA2::Lit(0x487aa8)),
    (0x002df838, None, BoneA2::Lit(0x487ab8)),
    (0x002df858, None, BoneA2::Lit(0x487ac8)),
    (0x002df878, None, BoneA2::Lit(0x487ad8)),
    (0x002df898, None, BoneA2::Lit(0x487ae8)),
    (0x002df8b8, None, BoneA2::Lit(0x487af8)),
];

/// `void cDynamicSprayFX::FindBones()` (0x2df4d0): looks the spray's 30 bone names
/// up on the rider's anim model (`getBoneSkelIndex`), stores the indices in
/// `self->mBoneIndicies` (pointer at +0x7c) and sets the ready flag (+0x80).
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cdynamic_spray_fx_find_bones(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002df4d0);
    guest(c, |c| {
        c.cycles += 254;
        let frame = Frame::open(c, 0xa0);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[A0][0];
        let this = c.g[FP][0] as u32;
        for (i, &(ret, set_s, a2)) in SPRAY_BONE_CALLS.iter().enumerate() {
            if i > 0 {
                // The previous call's result goes to `mBoneIndicies[i - 1]`.
                let out = Addr(this.wrapping_add(SPRAY_BONES)).lw(c);
                c.g[V1][0] = out;
                let slot = Addr((out as u32).wrapping_add(4 * (i as u32 - 1)));
                let bone = c.g[V0][0];
                slot.sw(c, bone);
            }
            if let Some((reg, val)) = set_s {
                c.g[reg][0] = sx(val);
            }
            c.g[A1][0] = 0;
            match a2 {
                BoneA2::Lit(v) => c.g[A2][0] = sx(v),
                BoneA2::Reg(r) => c.g[A2][0] = c.g[r][0],
            }
            // registers at the call: a0 = self->mRider->mAnimModel, a1 = 0
            c.g[V0][0] = Addr(this).lw(c);
            let model = Addr((c.g[V0][0] as u32).wrapping_add(0x780)).lw(c);
            c.g[A0][0] = model;
            call_ptr(c, 0x0031_0c48, ret)?;
        }
        let out = Addr(this.wrapping_add(SPRAY_BONES)).lw(c);
        c.g[V1][0] = out;
        c.g[A0][0] = 0x1d;
        let slot = Addr((out as u32).wrapping_add(4 * 29));
        let bone = c.g[V0][0];
        slot.sw(c, bone);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        // The 30-word countdown the compiler left behind.
        loop {
            c.cycles += 7;
            if (c.g[A0][0] as i64) >= 0 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_f8c8);
                }
                continue;
            }
            break;
        }
        c.cycles += 14;
        c.g[V0][0] = 1;
        Addr(this.wrapping_add(0x80)).sw(c, 1);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cBoltOnFXInterface::RefreshEmitters()` (0x2eccb8): rebuilds the bolt-on
/// effect's emitters from the current rider/board identity: clears a 15-flag scratch
/// array, queries the BE bolt interface for the identity (rider, or the UI state's
/// entry), walks the identity's bolt records creating/marking emitters through
/// `FindOrCreateEmitter`, then destroys the emitters in the +0x10 table that were not
/// marked, and clears self+0.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cbolt_on_fx_interface_refresh_emitters(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002eccb8);
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.g[V1][0] = 0xe;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x38));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S4][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        'l_2eccf0: loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002eccf0);
                }
                continue 'l_2eccf0;
            } else {
                'b_2ecdbc: {
                    'b_2ecd7c: {
                        c.cycles += 8;
                        call_ptr(c, 0x0014abe0, 0x002ecd14)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffffffffffffffff;
                        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        let t = c.g[A1][0] == 0u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                        if t {
                            break 'b_2ecd7c;
                        } else {
                            c.cycles += 16;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002ecd44)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0014ad50, 0x002ecd50)?;
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                            call_ptr(c, 0x00123168, 0x002ecd5c)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_2ecdbc;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                call_ptr(c, 0x00123128, 0x002ecd74)?;
                                let t = true;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                                if t {
                                    break 'b_2ecdbc;
                                } else {
                                    break 'b_2ecd7c;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_2ecdbc;
                    } else {
                        c.cycles += 13;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039fa88, 0x002ecd98)?;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32));
                        call_ptr(c, 0x0014ad28, 0x002ecdac)?;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                        break 'b_2ecdbc;
                    }
                }
                'b_2ecefc: {
                    c.cycles += 16;
                    call_ptr(c, 0x0014bdb8, 0x002ecdc4)?;
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                    call_ptr(c, 0x0014d988, 0x002ecddc)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    call_ptr(c, 0x0014d998, 0x002ecdec)?;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                    c.g[A0][0] = c.g[V0][0];
                    let t = (c.g[A2][0] as i64) <= 0;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_2ecefc;
                    } else {
                        c.cycles += 1;
                        'l_2ece00: loop {
                            'b_2ecee8: {
                                c.cycles += 5;
                                c.g[S2][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x11));
                                c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = c.g[S2][0] == c.g[V0][0];
                                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                                if t {
                                    break 'b_2ecee8;
                                } else {
                                    'b_2ece44: {
                                        'b_2ece40: {
                                            c.cycles += 8;
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                            c.g[V0][0] =
                                                c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x288));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            if t {
                                                break 'b_2ece40;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                                let t = true;
                                                c.g[V1][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                if t {
                                                    break 'b_2ece44;
                                                } else {
                                                    break 'b_2ece40;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = 0u64;
                                        break 'b_2ece44;
                                    }
                                    'b_2ece64: {
                                        c.cycles += 4;
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                        c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_2ece64;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] & 0x10;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                                            if t {
                                                break 'b_2ecee8;
                                            } else {
                                                break 'b_2ece64;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                                    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                                    c.g[S7][0] = ((c.g[S2][0] as i64) < 50i64) as u64;
                                    c.g[S6][0] = 0x1;
                                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12));
                                    c.g[S0][0] = c.lb((c.g[S1][0] as u32));
                                    'l_2ece80: loop {
                                        'b_2ecedc: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            if c.g[S0][0] == c.g[V0][0] {
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                                break 'b_2ecedc;
                                            } else {
                                                'b_2ecec8: {
                                                    'b_2ecec0: {
                                                        c.cycles += 2;
                                                        let t = c.g[S7][0] != 0u64;
                                                        c.g[A2][0] = c.g[S0][0];
                                                        if t {
                                                            break 'b_2ecec0;
                                                        } else {
                                                            c.cycles += 11;
                                                            c.g[A0][0] = c.g[S4][0];
                                                            c.g[A1][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_sub(0x32));
                                                            c.g[A2][0] = c.g[S0][0];
                                                            Callee { addr: 0x002ecbc0, f: ee::s09::cBoltOnFXInterface_FindOrCreateEmitter }.call(c, 0x002ecea4)?;
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[A2][0] = c.g[S0][0];
                                                            c.g[V1][0] = sx((c.g[SP][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = c.g[S4][0];
                                                            c.sw((c.g[V1][0] as u32), c.g[S6][0]);
                                                            let t = true;
                                                            c.g[A1][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_sub(0x31));
                                                            if t {
                                                                break 'b_2ecec8;
                                                            } else {
                                                                break 'b_2ecec0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A0][0] = c.g[S4][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    break 'b_2ecec8;
                                                }
                                                c.cycles += 5;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                                Callee { addr: 0x002ecbc0, f: ee::s09::cBoltOnFXInterface_FindOrCreateEmitter }.call(c, 0x002eced0)?;
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V1][0] = sx((c.g[SP][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.sw((c.g[V1][0] as u32), c.g[S6][0]);
                                                break 'b_2ecedc;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[S1][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[S0][0] = c.lb((c.g[S1][0] as u32));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002ece80);
                                            }
                                            continue 'l_2ece80;
                                        } else {
                                            break 'b_2ecee8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                            c.g[A1][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.g[S3][0];
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002ece00);
                                }
                                continue 'l_2ece00;
                            } else {
                                break 'b_2ecefc;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4c));
                c.g[S3][0] = 0xffffffffffffffff;
                c.g[S1][0] = c.g[SP][0];
                c.g[S2][0] = 0xe;
                'l_2ecf10: loop {
                    'b_2ecf38: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        if c.g[A1][0] == c.g[S3][0] {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            break 'b_2ecf38;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                            if c.g[V0][0] != 0u64 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_2ecf38;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                Callee {
                                    addr: 0x002f2218,
                                    f: ee::s09::AttachedEmitterFX_DestroyEmitter,
                                }
                                .call(c, 0x002ecf30)?;
                                c.sw((c.g[S0][0] as u32), c.g[S3][0]);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_2ecf38;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002ecf10);
                        }
                        continue 'l_2ecf10;
                    } else {
                        c.cycles += 13;
                        c.sw((c.g[S4][0] as u32), 0u64);
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Control block of this module's gated callee fakes (in the machine's own
    /// RAM, away from the pool and the fakes' call log at DATA+0x100000).
    const OC750_CTRL: u32 = DATA + 0x20_0000;
    const OC750_MAGIC: u32 = 0x0750_0750;

    /// Gated fakes for callees with override slots: with the magic word at
    /// `OC750_CTRL` they log their arguments (id, a0, a1, a2, ra, cycles at
    /// `OC750_CTRL + 0x400 + 0x20 * n`) and return the word at
    /// `OC750_CTRL + 0x20 + 4 * id` in v0; with the gate off they run what the
    /// slot held before.
    macro_rules! oc750_fakes {
        ($($fake:ident = $id:expr, $shard:ident :: $slot:ident / $gen:ident;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC750_CTRL) != OC750_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    oc750_fake_body(c, $id);
                }
            )*
            fn oc750_install() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    fn oc750_fake_body(c: &mut Cpu, id: u32) {
        let n = c.bus.read32(OC750_CTRL + 8);
        c.bus.write32(OC750_CTRL + 8, n + 1);
        let base = OC750_CTRL + 0x400 + 0x20 * n;
        let (a0, a1, a2) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
        c.bus.write32(base, id);
        c.bus.write32(base + 4, a0);
        c.bus.write32(base + 8, a1);
        c.bus.write32(base + 0xc, a2);
        c.bus.write32(base + 0x10, c.g[RA][0] as u32);
        c.bus.write32(base + 0x14, c.cycles as u32);
        let mut v = c.bus.read32(OC750_CTRL + 0x20 + 4 * id);
        match id {
            // FindOrCreateEmitter: an emitter slot 0..14.
            9 => v = v.wrapping_add(7 * n) % 15,
            // getBoneSkelIndex: depends on the name argument.
            11 => v = a2 ^ n,
            _ => {}
        }
        c.g[V0][0] = sx(v);
        ret(c);
    }

    oc750_fakes! {
        oc750_fake_bolt_this = 0, s01::OVERRIDE_0014ABE0 / f_0014abe0__generated;
        oc750_fake_bolt_ad50 = 1, s01::OVERRIDE_0014AD50 / f_0014ad50__generated;
        oc750_fake_cheat_identity = 2, s00::OVERRIDE_00123168 / f_00123168__generated;
        oc750_fake_identity = 3, s00::OVERRIDE_00123128 / f_00123128__generated;
        oc750_fake_ui_state = 4, s13::OVERRIDE_0039FA88 / f_0039fa88__generated;
        oc750_fake_bolt_ad28 = 5, s01::OVERRIDE_0014AD28 / f_0014ad28__generated;
        oc750_fake_bolt_bdb8 = 6, s01::OVERRIDE_0014BDB8 / f_0014bdb8__generated;
        oc750_fake_bolt_d988 = 7, s01::OVERRIDE_0014D988 / f_0014d988__generated;
        oc750_fake_bolt_d998 = 8, s01::OVERRIDE_0014D998 / f_0014d998__generated;
        oc750_fake_find_emitter = 9, s09::OVERRIDE_002ECBC0 / f_002ecbc0__generated;
        oc750_fake_destroy_emitter = 10, s09::OVERRIDE_002F2218 / f_002f2218__generated;
        oc750_fake_bone_index = 11, s10::OVERRIDE_00310C48 / f_00310c48__generated;
    }

    fn oc750_gate(c: &mut Cpu) {
        c.bus.write32(OC750_CTRL, OC750_MAGIC);
    }

    /// `cWorldSound_CalcAudioVolumeBox`: every shape type (0 to 3, plus unknown
    /// ones that leave the outputs alone) with random centre, extents and axis;
    /// the output vectors start with junk.
    #[test]
    fn cworld_sound_get_shape_bounds_matches() {
        check(
            ee::s08::f_002b6f40__generated,
            cworld_sound_get_shape_bounds,
            300,
            &|c, seed| {
                let mut r = Rng(0x002b_6f40 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                let shape = DATA + 0x100;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(shape);
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x230);
                let kind = [0u32, 1, 2, 3, 4, 0xffff_ffff, 1, 7][seed as usize % 8];
                c.bus.write32(shape, kind);
                for off in (4..0x30u32).step_by(4) {
                    c.bus.write32(shape + off, r.f32(20.0));
                }
                if seed % 5 == 4 {
                    // Axis along z: the side vector is a zero vector.
                    c.bus.write32(shape + 0x20, 0);
                    c.bus.write32(shape + 0x24, 0);
                }
                for off in (0..0x40u32).step_by(4) {
                    c.bus.write32(DATA + 0x200 + off, r.next_u32());
                }
            },
        );
    }

    /// `cDynamicSprayFX::FindBones`: 30 `getBoneSkelIndex` calls (a gated fake
    /// that logs its arguments and returns a hash of the name), results stored
    /// through the array pointer at +0x7c, and the +0x80 ready flag.
    #[test]
    fn cdynamic_spray_fx_find_bones_matches() {
        oc750_install();
        check(
            ee::s09::f_002df4d0__generated,
            cdynamic_spray_fx_find_bones,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2df4d0, 0);
                oc750_gate(c);
                let this = DATA + 0x100;
                let rider = DATA + 0x400;
                let out = DATA + 0x600;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, rider);
                c.bus.write32(this + SPRAY_BONES, out);
                c.bus.write32(this + 0x80, r.next_u32());
                c.bus
                    .write32(rider + 0x780, DATA + 0x800 + r.next_u32() % 0x100);
                for i in 0..32u32 {
                    c.bus.write32(out + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `cBoltOnFXInterface::RefreshEmitters`: the identity comes from the rider
    /// (a virtual call, which unwinds on some seeds) or the UI state; the bolt
    /// records of the identity (+0x38 stride; a lookup table of half-words and
    /// a flag array decide which emitters are marked) go through
    /// `FindOrCreateEmitter`; unmarked emitters of the +0x10 table are
    /// destroyed. All callees are gated fakes that log their arguments.
    #[test]
    fn cbolt_on_fx_interface_refresh_emitters_matches() {
        oc750_install();
        fakes::covered(
            ee::s09::f_002eccb8__generated,
            cbolt_on_fx_interface_refresh_emitters,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eccb8, 1);
                oc750_gate(c);
                let this = DATA + 0x100;
                let rider = DATA + 0x800;
                let ui = DATA + 0x900;
                let world = DATA + 0x6000;
                let obj = DATA + 0x2000; // the "identity object" (a2 of the lookups)
                let tbl = DATA + 0x3000;
                let recs = DATA + 0x4000;
                let vt = DATA + 0xb00;
                c.g[A0][0] = sx(this);
                let have_rider = seed % 4 != 0;
                c.bus.write32(this, r.next_u32());
                c.bus.write32(this + 4, if have_rider { rider } else { 0 });
                c.bus.write32(this + 8, if seed % 2 == 0 { ui } else { 0 });
                c.bus.write32(this + 0xc, DATA + 0xa00);
                for k in 0..15u32 {
                    let word = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(this + 0x10 + 12 * k, word);
                    c.bus.write32(this + 0x14 + 12 * k, r.next_u32());
                    c.bus.write32(this + 0x18 + 12 * k, r.next_u32());
                }
                c.bus.write32(ui, 0x1000 + r.next_u32() % 0x100);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x38, 0);
                c.bus.write32(vt + 0x3c, fakes::FAKE + 0x10);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, DATA + 0x6100);
                c.bus.write32(DATA + 0x6100 + 0xc, DATA + 0x6200);
                for off in (0..0x80u32).step_by(4) {
                    c.bus.write32(DATA + 0x5000 + off, r.next_u32());
                }
                // Identity object: +0x288 lookup table, +0x290 flag words.
                c.bus.write32(obj + 0x288, tbl);
                for i in 0..24u32 {
                    c.bus.write16(
                        tbl + 2 * i,
                        (r.next_u32() % 8) as u16 | if r.next_u32() % 4 == 0 { 0xffff } else { 0 },
                    );
                    c.bus.write32(obj + 0x290 + 4 * i, r.next_u32());
                }
                let count = r.next_u32() % 5;
                for k in 0..4u32 {
                    let rec = recs + 0x38 * k;
                    for off in (0..0x38u32).step_by(4) {
                        c.bus.write32(rec + off, r.next_u32());
                    }
                    let pick = |r: &mut Rng| [0xffu8, 5, 0x31, 0x40][(r.next_u32() % 4) as usize];
                    c.bus.write16(rec + 4, (r.next_u32() % 8) as u16);
                    c.bus.write8(rec + 0x11, pick(&mut r));
                    c.bus.write8(rec + 0x12, pick(&mut r));
                    c.bus.write8(rec + 0x13, pick(&mut r));
                }
                let results: [(u32, u32); 9] = [
                    (0, DATA + 0xc00),
                    (1, obj),
                    (2, r.next_u32() % 16),
                    (3, r.next_u32() % 16),
                    (4, DATA + 0x5000),
                    (5, obj),
                    (6, r.next_u32()),
                    (7, count),
                    (8, recs),
                ];
                for (id, v) in results {
                    c.bus.write32(OC750_CTRL + 0x20 + 4 * id, v);
                }
                c.bus.write32(OC750_CTRL + 0x20 + 4 * 9, r.next_u32() % 15);
                c.bus.write32(OC750_CTRL + 0x20 + 4 * 10, r.next_u32());
            },
        );
    }
}
