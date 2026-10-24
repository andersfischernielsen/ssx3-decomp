//! Slot 95 `DrawModel` (0x37e238) without packets: the model's matrices,
//! lighting and render state as the generated code computes them
//! (`docs/investigation/n2-capture/models.md`), the mesh chain by address.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

use super::vu0::{self, M, V};
use super::*;
use crate::ps2f::FpuF32;

/// `K`: normal → UV bias for the environment map (rows (0.5, 0, 0, 0),
/// (0, −0.5, 0, 0), (0, 0, 0, 0), (0.5, 0.5, 1, 1)).
const ENV_BIAS: u32 = 0x0050_4760;
/// An identity in RAM: a single node with it needs no node matrices.
const IDENTITY: u32 = 0x004f_f1a0;

const LIGHT_INSTANCE: u32 = 0x002f_5148;
const FIND_BUCKET: u32 = 0x0039_5000;
const COMPUTE_SORT_KEY: u32 = 0x0036_4240;
const APPEND_PACKET: u32 = 0x0036_2978;

fn f(c: &mut Cpu, a: u32) -> FpuF32 {
    FpuF32(r32(c, a))
}

/// `ClipTestBox` (graphics-manager slot 93): the box `min..max` (w = 1)
/// against `matrix`: 0 inside, 1 outside, 2 straddling. The box goes on
/// the guest stack.
fn clip_test_box(
    c: &mut Cpu,
    gm: u32,
    min: u32,
    max: u32,
    matrix: Option<M>,
    matrix_at: u32,
    ret: u32,
) -> u32 {
    let sp = c.g[SP][0] as u32;
    let frame = sp - 0x80;
    c.g[SP][0] = frame as i32 as i64 as u64;
    let corner = |c: &mut Cpu, a: u32| -> V { [r32(c, a), r32(c, a + 4), r32(c, a + 8), vu0::ONE] };
    let (lo, hi) = (corner(c, min), corner(c, max));
    wv(c, frame, &lo);
    wv(c, frame + 0x10, &hi);
    let m = match matrix {
        Some(m) => {
            wm(c, frame + 0x20, &m);
            frame + 0x20
        }
        None => matrix_at,
    };
    let (this, f) = vmethod(c, gm, 0x2ec);
    let r = guest_call(c, f, &[this, frame, frame + 0x10, m], ret);
    c.g[SP][0] = sp as i32 as i64 as u64;
    r
}

/// A graphics-manager virtual method: (`this` adjusted, function).
pub(super) fn vmethod(c: &mut Cpu, gm: u32, slot: u32) -> (u32, u32) {
    let vt = r32(c, gm + 0x10d8);
    let f = r32(c, vt + slot);
    let adj = c.bus.read16(vt + slot - 4) as i16 as i32;
    (gm.wrapping_add(adj as u32), f)
}

/// The depth sort value of the model's origin (`S[3]`): its clip w scaled
/// by `gm+0x5920` (w lane), clamped to 0..`.x`, as an integer.
fn sort_value(c: &mut Cpu, gm: u32, s: u32, ret: u32) -> u32 {
    sort_value_at(c, gm, s + 0x30, ret)
}

/// [`sort_value`] of the point at `at`.
pub(super) fn sort_value_at(c: &mut Cpu, gm: u32, at: u32, ret: u32) -> u32 {
    let origin = qv(c, at);
    ensure_view_matrices(c, gm, ret);
    let k = qv(c, gm + 0x5920);
    let wc = qm(c, gm + 0x58a0);
    let t = vu0::xform(&wc, &origin);
    let w = vu0::mul(&t, &vu0::bc(&k, 3))[3];
    let w = vu0::max(w, 0);
    let w = vu0::min(w, k[0]);
    vu0::ftoi0(w) & 0xffff
}

/// The sort fields of the current state for a depth-sorted draw (`w1`
/// sort mode 2, `w2` sort value) and the ZTST / ATST / AREF fields.
fn sorted_state(c: &mut Cpu, gm: u32, s: u32, blend: (u32, u32), ret: u32) {
    let st = r32(c, gm + gmf::CUR_STATE);
    let mut w1 = r32(c, st + 4);
    w1 = w1 & 0xffff_ff83 | blend.0;
    w1 = w1 & 0xffff_f07f | blend.1;
    w32(c, st + 4, w1);
    let z = sort_value(c, gm, s, ret);
    let st = r32(c, gm + gmf::CUR_STATE);
    let mut w1 = r32(c, st + 4);
    w1 = w1 & !3 | 2;
    let w2 = r32(c, st + 8) & 0xe000_03ff | (z << 10 & 0x1fff_fc00);
    w32(c, st + 8, w2);
    w1 = w1 & 0xfe7f_ffff | 0x80_0000;
    w1 = w1 & 0xffcf_ffff | 0x30_0000;
    w1 = w1 & 0xfff0_0fff | 0x1_4000;
    w32(c, st + 4, w1);
}

/// The base-state blend modes (not depth sorted): `w1` fields.
fn base_state(c: &mut Cpu, gm: u32, blend: (u32, u32), sort: u32, ztst: u32, aref: u32) {
    let st = r32(c, gm + gmf::CUR_STATE);
    let mut w1 = r32(c, st + 4);
    w1 = w1 & 0xffff_ff83 | blend.0;
    w1 = w1 & 0xffff_f07f | blend.1;
    w1 = w1 & !3 | sort;
    let w2 = r32(c, st + 8) & 0xe000_03ff;
    w32(c, st + 8, w2);
    w1 = w1 & 0xfe7f_ffff | 0x80_0000;
    w1 = w1 & 0xffcf_ffff | ztst;
    w1 = w1 & 0xfff0_0fff | aref;
    w32(c, st + 4, w1);
}

/// `PS2GraphicsMan_DrawModel(f12 alpha, gm, inst, texTable, world,
/// nodeMatrices, matrix2, flags)` (slot 95, 0x37e238).
pub fn draw_model(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    let alpha = FpuF32(c.f[12]);
    if r32(c, gp + 0x1400) != 0 || alpha.0 & 0x7fff_ffff == 0 {
        return ret(c, 10);
    }
    let (gm, inst, textab, world) = (
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] as u32,
        c.g[A3][0] as u32,
    );
    let (node_arg, mat2_arg, mut flags) = (c.g[T0][0] as u32, c.g[T1][0] as u32, c.g[T2][0] as u32);
    let mut groups_drawn = 0u64;
    ensure_view_matrices(c, gm, 0x0037_e2bc);
    if flags & 0x400 == 0 {
        let mut r = clip_test_box(
            c,
            gm,
            inst + 0x60,
            inst + 0x6c,
            None,
            gm + 0x58a0,
            0x0037_e330,
        );
        if r == 1 {
            return ret(c, 60);
        }
        if r == 2 {
            r = clip_test_box(
                c,
                gm,
                inst + 0x60,
                inst + 0x6c,
                None,
                gm + 0x5800,
                0x0037_e3a0,
            );
        }
        if r != 0 {
            flags |= 0x20;
        }
    }
    let mut light = 0;
    if r32(c, inst + 8) & 0x4000 != 0 {
        flags |= 1;
        light = guest_call(c, LIGHT_INSTANCE, &[inst], 0x0037_e3d8);
    }
    let lit = flags & 1 != 0;
    // The scratch: S[0..3] header matrix, S[4] scale, S[8..] node matrices,
    // S[0x68] lighting basis, S[0x6c] world, S[0x70] node × world, S[0x74]
    // the matrix-stack top, S[0x78] / S[0x7c] the environment matrices.
    let s = r32(c, gp + 0x2a94);
    let row = |i: u32| s + 0x10 * i;
    let desc = r32(c, inst + 0x80);
    let chain_base = if lit {
        r32(c, desc + 0x24)
    } else {
        r32(c, inst + 0x98)
    };
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0xc0);
    state_word(c, st, 3, 0xc000_0000, 0);
    let sc = f(c, inst + 0x84);
    let scale: V = [
        (f(c, desc + 0x18) * sc).0,
        (f(c, desc + 0x1c) * sc).0,
        (f(c, desc + 0x20) * sc).0,
        vu0::ONE,
    ];
    wv(c, row(4), &scale);
    let w = qm(c, world);
    wm(c, row(0x6c), &w);
    if c.bus.read16(inst + 0x92) & 1 != 0 {
        let (this, func) = vmethod(c, gm, 0x2f4);
        guest_call(c, func, &[this, row(0x6c)], 0x0037_e4f4);
    }
    let s6c = qm(c, row(0x6c));
    wm(c, row(0x70), &s6c);
    let mut node_mats = node_arg;
    if node_mats == 0 {
        let nodes = r32(c, desc + 8);
        let h = vu0::scale_rows(&s6c, &scale);
        wm(c, row(0), &h);
        let count = r32(c, desc + 4) as i32;
        if count < 2 && r32(c, nodes + 0xc) == IDENTITY {
            if lit {
                wm(c, row(0x68), &s6c);
            }
        } else {
            for i in 0..count.max(0) as u32 {
                let node = nodes + 0x10 * i;
                let parent = r32(c, node) as i32;
                let local = r32(c, node + 0xc);
                let mut l = qm(c, local);
                l[3] = [
                    (FpuF32(l[3][0]) * sc).0,
                    (FpuF32(l[3][1]) * sc).0,
                    (FpuF32(l[3][2]) * sc).0,
                    (FpuF32(l[3][3]) * FpuF32(vu0::ONE)).0,
                ];
                let m = if parent == -1 {
                    l
                } else {
                    vu0::mul_m(&qm(c, row(8 + 4 * parent as u32)), &l)
                };
                wm(c, row(8 + 4 * i), &m);
            }
            node_mats = row(8);
        }
    }
    let (this, func) = vmethod(c, gm, 0x11c);
    let top = guest_call(c, func, &[this], 0x0037_e840);
    let mut s74 = qm(c, top);
    wm(c, row(0x74), &s74);
    let env = r32(c, desc + 0x10) & 2 != 0;
    let bias = qm(c, ENV_BIAS);
    let mut extra = 0;
    // The environment-map matrices for the node matrix `s70`.
    let env_matrices = |c: &mut Cpu, s74: &M, s70: &M| {
        let mut s78 = vu0::mul_m(s74, s70);
        s78[3] = [0, 0, 0, vu0::ONE];
        wm(c, row(0x78), &s78);
        let s7c = vu0::mul_m(&bias, &s78);
        wm(c, row(0x7c), &s7c);
    };
    if env {
        // A rotation facing the model's centre (in the matrix-stack top's
        // space) from the eye.
        let corner =
            |c: &mut Cpu, a: u32| -> V { [r32(c, a), r32(c, a + 4), r32(c, a + 8), vu0::ONE] };
        let (lo, hi) = (corner(c, inst + 0x60), corner(c, inst + 0x6c));
        let centre = vu0::mul(&vu0::add(&lo, &hi), &[0x3f00_0000; 4]);
        let p = vu0::xform(&s74, &centre);
        let d = vu0::sub(&[0, 0, 0, vu0::ONE], &p);
        let n1 = vu0::normalize4(&[0, d[1], d[2], 0]);
        let n2 = vu0::normalize4(&[d[0], 0, d[2], 0]);
        let (y1, z1, x2, z2) = (FpuF32(n1[1]), FpuF32(n1[2]), FpuF32(n2[0]), FpuF32(n2[2]));
        let neg = |v: FpuF32| v.0 ^ 0x8000_0000;
        let r: M = [
            [z2.0, 0, neg(x2), 0],
            [(y1 * x2).0, z1.0, (y1 * z2).0, 0],
            [(z1 * x2).0, neg(y1), (z1 * z2).0, 0],
            [0, 0, 0, vu0::ONE],
        ];
        s74 = vu0::mul_m(&r, &s74);
        wm(c, row(0x74), &s74);
        let s70 = qm(c, row(0x70));
        env_matrices(c, &s74, &s70);
        extra = 0x100;
    }
    let sorted_model = r32(c, desc + 0x10) & 8 != 0;
    if sorted_model {
        let st = push_state(c, gm);
        state_word(c, st, 2, 0xffff_fc1f, 0xe0);
        state_word(c, st, 1, 0xffbf_ffff, 0x40_0000);
    }
    let count = r32(c, desc + 4) as i32;
    let nodes = r32(c, desc + 8);
    let fast_allowed = (flags as i32 >> 11) & 1 != 0;
    for i in 0..count.max(0) as u32 {
        let ml = r32(c, nodes + 0x10 * i + 4);
        if ml == 0 || r32(c, ml + 0x1c) == 0 {
            continue;
        }
        if node_mats != 0 {
            let n = qm(c, node_mats + 0x40 * i);
            let s70 = vu0::mul_m(&s6c, &n);
            wm(c, row(0x70), &s70);
            if env {
                env_matrices(c, &s74, &s70);
            }
            if lit {
                wm(c, row(0x68), &s70);
            }
            let h = vu0::scale_rows(&s70, &scale);
            wm(c, row(0), &h);
        }
        if count > 1 && flags & 0x1020 == 0x1020 {
            let wc = qm(c, gm + 0x58a0);
            let s70 = qm(c, row(0x70));
            let m = vu0::mul_m(&wc, &s70);
            if clip_test_box(c, gm, ml, ml + 0xc, Some(m), 0, 0x0037_f1ac) == 1 {
                continue;
            }
        }
        let groups = r32(c, ml + 0x20);
        let ngroups = r32(c, ml + 0x1c) as i32;
        for j in 0..ngroups.max(0) as u32 {
            groups_drawn += 1;
            let g = r32(c, groups + 4 * j);
            let mat = c.bus.read16(g) as i16 as i32;
            let e = r32(c, textab.wrapping_add(4).wrapping_add((4 * mat) as u32));
            let mut m = r32(c, e + 0xc);
            if c.bus.read16(g + 2) & 8 != 0 {
                m |= 0x4_0000;
            }
            let st = r32(c, gm + gmf::CUR_STATE);
            let (t0, t1) = (c.bus.read16(e), c.bus.read16(e + 2));
            c.bus.write16(st + 0x10, t0);
            c.bus.write16(st + 0x12, t1);
            if r32(c, gp + 0x1404) != 0 {
                m &= 0xff9f_ffff;
            }
            let mut fast = fast_allowed;
            let clamp = match m & 0x18_0000 {
                0x10_0000 => 4,
                0x8_0000 => 8,
                0x18_0000 => 0xc,
                _ => 0,
            };
            if clamp != 0 {
                fast = false;
            }
            state_word(c, st, 0, 0xffff_fff3, clamp);
            let mut base = 0;
            let (mut mat2, mut fl) = (mat2_arg, flags);
            let mut env_pass = || {
                mat2 = row(0x7c);
                fl = flags | extra;
            };
            if sorted_model {
                sorted_state(c, gm, s, (0x1c, 0x180), 0x0037_f3b8);
            } else {
                let dm = r32(c, gm + gmf::DEFER);
                match m & 0x66_0000 {
                    0x20_0000 => {
                        if fast {
                            base = r32(c, dm + 0x5748c);
                        } else {
                            base_state(c, gm, (4, 0x100), 0, 0x30_0000, 0x1_4000);
                        }
                        env_pass();
                    }
                    0x2_0000 => {
                        if fast {
                            base = r32(c, dm + 0x57490);
                        } else {
                            base_state(c, gm, (0x14, 0x180), 1, 0x30_0000, 0x5_c000);
                        }
                    }
                    0 => {
                        if fast {
                            base = r32(c, dm + 0x57488);
                        } else {
                            base_state(c, gm, (4, 0x180), 0, 0, 0x1_4000);
                        }
                    }
                    0x60_0000 => {
                        if fast {
                            base = r32(c, dm + 0x57488);
                        } else {
                            base_state(c, gm, (4, 0x880), 0, 0, 0x1_4000);
                        }
                        env_pass();
                    }
                    0x4_0000 | 0x6_0000 => sorted_state(c, gm, s, (0x14, 0x180), 0x0037_f9d0),
                    0x22_0000 | 0x26_0000 => {
                        sorted_state(c, gm, s, (0x14, 0x100), 0x0037_fb0c);
                        env_pass();
                    }
                    0x62_0000 | 0x66_0000 => {
                        sorted_state(c, gm, s, (0x14, 0x880), 0x0037_fc44);
                        env_pass();
                    }
                    _ => {
                        sorted_state(c, gm, s, (0x14, 0x180), 0x0037_f708);
                        env_pass();
                    }
                }
            }
            // The VU1 input (EmitModelHeader / Matrix2 / Lighting) and the
            // mesh chain.
            let mut header = qwords(c, s, 4);
            header.push(q4([0, 0, 0, fl]));
            let mut steps = vec![
                Step::Mask(0),
                Step::Cycle(0x0101),
                Step::Write {
                    addr: 0,
                    rel: true,
                    data: header,
                },
            ];
            if mat2 != 0 {
                steps.push(Step::Write {
                    addr: 5,
                    rel: true,
                    data: qwords(c, mat2, 4),
                });
            }
            if fl & 1 != 0 {
                steps.push(Step::Write {
                    addr: 9,
                    rel: true,
                    data: qwords(c, row(0x68), 3),
                });
                steps.push(Step::Write {
                    addr: 0xf,
                    rel: true,
                    data: qwords(c, light, 10),
                });
            }
            steps.push(Step::Chain(chain_base.wrapping_add(r32(c, g + 4))));
            let start = empty_packet(c, gm);
            record(start, Cmd::Vu(steps));
            if alpha.0 == vu0::ONE {
                if base == 0 {
                    add_record(c, gm, start, start, false);
                } else {
                    fast_bucket(c, gm, base, start);
                }
            } else {
                // Two records: a depth pre-pass, then the FIX blend at alpha.
                push_state(c, gm);
                let z = sort_value(c, gm, s, 0x0038_0030);
                let st = r32(c, gm + gmf::CUR_STATE);
                state_word(c, st, 1, !3, 2);
                state_word(c, st, 2, 0xe000_03ff, z << 10 & 0x1fff_fc00);
                state_word(c, st, 1, 0xffff_ff83, 0xc);
                state_word(c, st, 1, 0xffff_f07f, 0x180);
                add_record(c, gm, start, start, false);
                let st = r32(c, gm + gmf::CUR_STATE);
                for k in 0..5 {
                    let v = r32(c, st - 0x14 + 4 * k);
                    w32(c, st + 4 * k, v);
                }
                state_word(c, st, 1, 0xfe7f_ffff, 0x80_0000);
                state_word(c, st, 1, 0xffcf_ffff, 0);
                state_word(c, st, 1, 0xfff0_0fff, 0x1_4000);
                let fix =
                    ((alpha * FpuF32::from_f32(128.0)).to_i32() as u32 & 0xff) << 12 & 0xf_f000;
                state_word(c, st, 0, 0xfff0_0fff, fix);
                state_word(c, st, 1, 0xffff_ff83, 0x2c);
                add_record(c, gm, start, start, true);
                pop_state(c, gm);
            }
        }
    }
    if sorted_model {
        pop_state(c, gm);
    }
    ret(c, 300 + 250 * groups_drawn);
}

/// The opaque base-state path: no record; the packet goes straight into
/// the bucket of (base state, textures, matrix), created if needed.
fn fast_bucket(c: &mut Cpu, gm: u32, base: u32, start: u32) {
    let dm = r32(c, gm + gmf::DEFER);
    let st = r32(c, gm + gmf::CUR_STATE);
    let (t0, t1) = (c.bus.read16(st + 0x10), c.bus.read16(st + 0x12));
    let tex = t0 as u32 | (t1 as u32) << 16;
    let w2 = r32(c, base + 8) & !0x1f | r32(c, dm + dmf::SCISSOR) & 0x1f;
    w32(c, base + 8, w2);
    let matrix = r32(c, dm + dmf::MATRIX);
    let k = base ^ (t0 as i16 as i32 as u32 | ((t1 as i16 as i32 + 2) as u32) << 16);
    let mix =
        |sh: u32| -> u32 { ((((k as i32 >> sh) as u32) << 5).wrapping_add(k) as i32 >> sh) as u32 };
    let hash = ((k << 5).wrapping_add(k) ^ mix(8) ^ mix(16) ^ mix(24)) & 0xff;
    let mut bucket = guest_call(c, FIND_BUCKET, &[dm, base, tex, hash, matrix], 0x0037_fe84);
    if bucket == 0 {
        let n = r32(c, dm + 0x57494);
        w32(c, dm + 0x57494, n + 1);
        bucket = dm + 0x57498 + 0x20 * n;
        w32(c, bucket + 8, matrix);
        w32(c, bucket, base);
        w32(c, bucket + 4, tex);
        w32(c, bucket + 0xc, 0);
        w32(c, bucket + 0x14, 0);
        w32(c, bucket + 0x18, 0);
        let key = guest_call(c, COMPUTE_SORT_KEY, &[base, tex], 0x0037_fee0);
        let slot = dm + 0x678a0 + 4 * hash;
        let head = r32(c, slot);
        w32(c, bucket + 0x1c, head);
        w32(c, slot, bucket);
        let k = r32(c, dm + 0x67ca4);
        w32(c, dm + 0x67ca8 + 8 * k, key);
        w32(c, dm + 0x67cac + 8 * k, bucket);
        w32(c, dm + 0x67ca4, k + 1);
    }
    guest_call(c, APPEND_PACKET, &[bucket, start, start], 0x0037_ff38);
}

/// `cPS2GridMesh_Draw(this, matrix, texId, matrix2)` (0x385c10): the grid
/// (fence, flag, flexible bridge) through the model program: the header
/// matrix (rows scaled by `this+0x194`), matrix 2 when given, then the
/// active copy of its double-buffered chain.
pub fn grid_mesh_draw(c: &mut Cpu) {
    let (this, matrix, tex, mat2) = (
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] as u16,
        c.g[A3][0] as u32,
    );
    if r32(c, this + 0x1a4) != 0 {
        w32(c, this + 0x1a4, 0);
        let b = r32(c, this + 0x1a0).wrapping_add(1) & 1;
        w32(c, this + 0x1a0, b);
    }
    let gm = r32(c, this + 0x1c4);
    ensure_view_matrices(c, gm, 0x0038_5c70);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0xc0);
    state_word(c, st, 3, 0xc000_0000, 0);
    c.bus.write16(st + 0x10, tex);
    let s = r32(c, this + 0x194);
    let m = vu0::scale_rows(&qm(c, matrix), &[s, s, s, vu0::ONE]);
    let flags = if mat2 == 0 { 0x20 } else { 0x22 };
    let mut header: Vec<u128> = m.iter().map(|r| q4(*r)).collect();
    header.push(q4([0, 0, 0, flags]));
    let mut steps = vec![
        Step::Mask(0),
        Step::Cycle(0x0101),
        Step::Write {
            addr: 0,
            rel: true,
            data: header,
        },
    ];
    if mat2 != 0 {
        steps.push(Step::Write {
            addr: 5,
            rel: true,
            data: qwords(c, mat2, 4),
        });
    }
    let active = r32(c, this + 0x1a0);
    let chain = r32(c, this + 0x1ac + 4 * active);
    steps.push(Step::Chain(chain));
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, false);
    ret(c, 150);
}
