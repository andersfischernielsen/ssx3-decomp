//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path small functions (front-end screens, lobby and
//! message plumbing, UI/menu helpers, mesh/effect utilities), each with a
//! differential test against the generated body.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers may read `v0`/`v1`), follow the call protocol of
//! [`crate::call`] and the event rules of [`crate::events`].

use crate::abi;
use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_7d20, benewplayerint_raise_difficulty, "BENewPlayerInt_RaiseDifficulty"),
    (0x0014_7e18, benewplayerint_lower_difficulty, "BENewPlayerInt_LowerDifficulty"),(0x0028_21a0, script_engine_start_script, "ScriptEngine_StartScript"),
    (0x0032_da40, shell_sort_desc_key_val, "ShellSortDesc_KeyVal"),(0x0034_6d38, flex_bridge_init_segment_midpoints, "FlexBridge_InitSegmentMidpoints"),(0x0038_5aa0, cps2_grid_mesh_pack_color, "cPS2GridMesh_PackColor"),
    (0x0039_9820, uilistbox_load, "UIListBox_Load"),
    (0x003a_03f0, uitext_load, "UIText_Load"),
    (0x003a_7c88, section_table_ctor, "SectionTable_ctor"),
    // OVERRIDES_END
];

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// An `s32` expression of `v`.
#[inline(always)]
fn s32(v: u64) -> i32 {
    v as u32 as i32
}

// ---------------------------------------------------------------------------
// Functions.

/// `int func_001E4B80(int idx)` (0x1e4b80): the message-center group variant
/// byte for popup entry `idx`: 0xff when the message center is not up or the
/// entry is not of the "shown" kind, else `MessageCenter_GetIndexInType`
/// modulo a per-group divisor (entries of type 3 are halved first), masked to
/// a byte.
pub fn ovscreen_message_center_get_group_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let idx = c.g[A1][0];
        c.g[V0][0] = c.lw(gp(c, -0x102c)); // D_004A20C4: the message center
        frame.save(c, 16, 0x10);
        c.g[A2][0] = idx;
        frame.save_ra(c, 0);
        let up = c.g[V0][0] != 0;
        c.g[A3][0] = 0xff;
        if up {
            c.cycles += 9;
            c.g[A0][0] = 0x18;
            c.g[V0][0] = 0x4c0000;
            // a = &D_004C6C08[idx] (0x18-byte popup entries)
            c.mult(0, 4, 6, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
            c.g[A1][0] = 0x1;
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
            let other_kind = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = c.g[A3][0];
            if !other_kind {
                // b = &D_00441630[a->field_0x4] (0x14-byte group records)
                c.cycles += 13;
                c.g[RA][0] = sx(0x001e_4bd0);
                c.g[A0][0] = c.g[A2][0];
                c.pc = 0x001e_2dc0;
                ee::s04::MessageCenter_GetIndexInType(c);
                if c.pc != 0x001e_4bd0 {
                    return Err(Unwound);
                }
                c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 4));
                c.g[A1][0] = 0x14;
                c.g[A0][0] = 0x44_0000;
                c.g[A2][0] = c.g[V0][0];
                c.mult(0, 3, 3, 5);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
                c.g[V0][0] = 0x4;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                let case4 = c.g[A0][0] == c.g[V0][0];
                c.g[A1][0] = 0x1;
                if case4 {
                    c.cycles += 2;
                    c.g[A1][0] = 0xa;
                } else {
                    c.cycles += 3;
                    let ge5 = (s32(c.g[A0][0]) as i64) >= 5;
                    if ge5 {
                        c.cycles += 3;
                        if c.g[A0][0] == 0x5 {
                            c.cycles += 5;
                            c.g[A1][0] = 0xb;
                        } else if c.g[A0][0] == 0x8 {
                            c.cycles += 3;
                            c.g[A1][0] = 0x4;
                        } else {
                            c.cycles += 4;
                        }
                    } else if c.g[A0][0] == 0x3 {
                        // case 3: halve the index (rounding toward zero)
                        c.cycles += 2 + 5;
                        c.g[A1][0] = 0xa;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 31);
                        let v = (c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32);
                        c.g[A2][0] = sx(((v as i32) >> 1) as u32);
                    } else {
                        c.cycles += 2 + 2;
                    }
                }
                // the (signed) remainder, low byte
                c.div(0, A2, A1);
                c.cycles += 2;
                c.cycles += 3;
                c.g[V0][0] = c.hi[0];
                c.g[A3][0] = c.g[V0][0] & 0xff;
                c.g[V0][0] = c.g[A3][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xff;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00385AA0(void* self, u16* buf, int idx, const float* c)`
/// (0x385aa0): pack `c[1]`, `c[2]`, `c[3]`, `c[0]` (clamped to 0..1 and scaled
/// by 32) into the 16-bit PS2 colour word `buf[idx]`: `a << 15 | b << 10 |
/// g << 5 | r`, each channel saturated to 31 (alpha becomes 1 when set).
pub fn cps2_grid_mesh_pack_color(c: &mut Cpu) {
    c.cycles += 10;
    c.mtc1(0, 2); // f2 = 0.0
    c.g[T3][0] = c.g[A1][0]; // buf
    c.lwc1(1, p(c.g[A3][0] as u32, 4)); // f1 = c[1]
    c.g[T2][0] = c.g[A2][0]; // idx
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3); // f3 = 1.0
    c.c_le_s(2, 1); // 0 <= c[1]
    c.mov_s(0, 2); // f0 = 0.0
    if c.fcc() {
        c.cycles += 1;
        c.min_s(0, 1, 3); // f0 = min(c[1], 1.0)
    }

    c.cycles += 10;
    c.g[AT][0] = 0x4200_0000;
    c.mtc1(1, 5); // f5 = 32.0
    c.mov_s(4, 2); // f4 = 0.0
    c.lwc1(1, p(c.g[A3][0] as u32, 8)); // f1 = c[2]
    c.mul_s(0, 0, 5); // f0 *= 32
    c.c_le_s(2, 1);
    c.cvt_w_s(6, 0); // f6 = (int)f0
    c.mfc1(2, 6); // v0 = f6
    let t = !c.fcc();
    c.g[T1][0] = c.g[V0][0] & 0xff; // r
    if !t {
        c.cycles += 1;
        c.min_s(4, 1, 3); // f4 = min(c[2], 1.0)
    }

    c.cycles += 8;
    c.mul_s(0, 4, 5); // f0 = f4 * 32
    c.lwc1(1, p(c.g[A3][0] as u32, 0xc)); // f1 = c[3]
    c.mov_s(4, 2);
    c.c_le_s(2, 1);
    c.cvt_w_s(6, 0);
    c.mfc1(2, 6);
    let t = !c.fcc();
    c.g[T0][0] = c.g[V0][0] & 0xff; // g
    if !t {
        c.cycles += 1;
        c.min_s(4, 1, 3);
    }

    c.cycles += 8;
    c.mul_s(0, 4, 5);
    c.lwc1(1, c.g[A3][0] as u32); // f1 = c[0]
    c.mov_s(4, 2);
    c.c_le_s(2, 1);
    c.cvt_w_s(2, 0);
    c.mfc1(2, 2);
    let t = !c.fcc();
    c.g[A3][0] = c.g[V0][0] & 0xff; // b
    if !t {
        c.cycles += 1;
        c.min_s(4, 1, 3);
    }

    c.cycles += 23;
    c.mul_s(0, 4, 5); // f0 = f4 * 32
    c.g[A1][0] = 0x1f;
    c.g[V1][0] = (c.g[A3][0] < 0x20) as u64;
    c.g[V0][0] = 0x1;
    if c.g[V1][0] == 0 {
        c.g[A3][0] = c.g[A1][0];
    }
    c.g[A0][0] = (c.g[T0][0] < 0x20) as u64;
    c.cvt_w_s(1, 0);
    c.mfc1(3, 1); // t3 = (int)(clamp(c[0]) * 32)
    if c.g[A0][0] == 0 {
        c.g[T0][0] = c.g[A1][0];
    }
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 10);
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 5);
    c.g[A2][0] = (c.g[T1][0] < 0x20) as u64;
    c.g[A0][0] = sx((c.g[T2][0] as u32) << 1);
    c.g[V1][0] = c.g[V1][0] & 0xff;
    if c.g[A2][0] == 0 {
        c.g[T1][0] = c.g[A1][0];
    }
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 15);
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
    c.sh(c.g[A0][0] as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void func_00346D38(FlexBridge* self)` (0x346d38): place the midpoint of
/// every bridge segment (`pts[i]` to `pts[count + i]`) into `segs[i].pos`, then
/// recompute the bridge's start/end offsets from the first segment.
pub fn flex_bridge_init_segment_midpoints(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    let this = c.g[A1][0] as u32;
    c.g[T1][0] = c.lw(p(this, 0x50)); // count
    c.g[A2][0] = 0;
    let empty = s32(c.g[T1][0]) <= 0;
    c.g[A3][0] = c.lw(p(this, 0x74)); // segs
    if !empty {
        c.cycles += 4;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0); // f0 = 0.5
        c.g[T0][0] = sx((c.g[T1][0] as u32) << 4);
        'loop_segments: loop {
            c.cycles += 32;
            c.g[V0][0] = c.lw(p(this, 0x7c)); // pts
            c.g[A0][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, c.g[V0][0] as u32); // pts[count + i]
            c.lqc2(4, c.g[V1][0] as u32); // pts[i]
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.sq(2, p(c.g[SP][0] as u32, 0x20));
            c.mfc1(3, 0); // v1 = 0.5
            c.lqc2(4, p(c.g[SP][0] as u32, 0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.g[V1][0] = c.lw(p(this, 0x7c));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sq(2, p(c.g[SP][0] as u32, 0x10));
            c.lqc2(3, c.g[A0][0] as u32);
            c.lqc2(4, p(c.g[SP][0] as u32, 0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, p(c.g[SP][0] as u32, 0x30));
            c.lq(2, p(c.g[SP][0] as u32, 0x30));
            c.sq(2, p(c.g[SP][0] as u32, 0));
            c.sq(2, c.g[A3][0] as u32); // segs[i].pos
            c.g[V0][0] = c.lw(p(this, 0x50));
            c.g[V0][0] = ((s32(c.g[A2][0]) < s32(c.g[V0][0])) as u64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
            if !more {
                break 'loop_segments;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_6d60);
            }
        }
    }
    c.cycles += 21;
    c.g[V0][0] = c.lw(p(this, 0x7c));
    c.g[A0][0] = sx((c.g[T1][0] as u32) << 4);
    c.g[A3][0] = c.lw(p(this, 0x74));
    c.lqc2(3, c.g[V0][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(c.g[SP][0] as u32, 0x10));
    c.lq(3, p(c.g[SP][0] as u32, 0x10));
    c.g[V0][0] = c.lw(p(this, 0x7c));
    c.sq(3, p(this, 0x30)); // start
    c.sq(3, p(c.g[SP][0] as u32, 0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(c.g[SP][0] as u32, 0x10));
    c.lq(2, p(c.g[SP][0] as u32, 0x10));
    c.sq(2, p(this, 0x20)); // end
    c.sq(2, p(c.g[SP][0] as u32, 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}

/// `void func_00147D20(void* unused, int which)` (0x147d20): step the current
/// rider's difficulty up: `a` (a 16-bit counter) increments and wraps to 0 at
/// 2, in which case `b` (bounded to 2) follows; the `which` pair of the
/// character record at `D_004A6CA8 + profile*0x9b50 + character*0xf88` is
/// updated.
pub fn benewplayerint_raise_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_7d40);
        c.g[S1][0] = 0; // profile
        c.pc = 0x0014_a080;
        ee::s01::BE_GetRiderCharacter(c);
        if c.pc != 0x0014_7d40 {
            return Err(Unwound);
        }
        let second = c.g[S0][0] == 0;
        c.g[A2][0] = c.g[V0][0]; // character id
        if second {
            c.cycles += 8;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x286)); // b1
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x284)); // a1
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x282)); // b0
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x280)); // a0
        }
        // a++; at 2 wrap to 0 and step b (up to 2)
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V1][0] = ((c.g[A1][0] as i64) < 2) as u64;
        let a_wrapped = c.g[V1][0] == 0;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
        if a_wrapped {
            c.cycles += 2;
            let b_at_max = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if !b_at_max {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
            }
            c.cycles += 1;
            c.g[A1][0] = 0;
        }
        // write the pair back
        c.cycles += 2;
        let second_store = c.g[S0][0] == 0;
        c.g[V0][0] = 0xf88;
        if second_store {
            c.cycles += 8;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x286), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x284), c.g[A3][0]);
        } else {
            c.cycles += 9;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x282), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x280), c.g[A3][0]);
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00147E18(void* unused, int which)` (0x147e18): the mirror of
/// [`benewplayerint_raise_difficulty`]: `a` counts down and wraps to 0 below
/// -1, in which case `b` (bounded to 0) follows.
pub fn benewplayerint_lower_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_7e38);
        c.g[S1][0] = 0; // profile
        c.pc = 0x0014_a080;
        ee::s01::BE_GetRiderCharacter(c);
        if c.pc != 0x0014_7e38 {
            return Err(Unwound);
        }
        let second = c.g[S0][0] == 0;
        c.g[A2][0] = c.g[V0][0];
        if second {
            c.cycles += 8;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x286));
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x284));
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x282));
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x280));
        }
        // a--; below -1 wrap to 0 and step b (down to 0)
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V1][0] = ((c.g[A1][0] as i64) < -1) as u64;
        let a_under = c.g[V1][0] != 0;
        if a_under {
            c.cycles += 2;
            let b_at_zero = c.g[A3][0] == 0;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            if !b_at_zero {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
            }
            c.cycles += 1;
            c.g[A1][0] = 0;
        }
        // write the pair back
        c.cycles += 2;
        let second_store = c.g[S0][0] == 0;
        c.g[V0][0] = 0xf88;
        if second_store {
            c.cycles += 8;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x286), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x284), c.g[A3][0]);
        } else {
            c.cycles += 9;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x282), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x280), c.g[A3][0]);
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_0032DA40(int n, int* keys, int* vals)` (0x32da40): shell sort
/// (descending on `keys`, `vals` carried along) using the gap table at
/// `D_0044AF98`, starting with the first gap below 2.
pub fn shell_sort_desc_key_val(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x45_0000;
    c.g[T5][0] = c.g[A0][0]; // n
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5068)); // D_0044AF98
    c.g[T2][0] = c.g[A1][0]; // keys
    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
    let done = c.g[V0][0] == 0;
    c.g[V1][0] = 0x1;
    if !done {
        // k = 1; while (D[k] < n) k++
        c.cycles += 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
            let small = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if !small {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0032_da68);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = 0x45_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5068));
    c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(4));
    loop {
        // gap = D[--k]; the insertion pass only runs for gap < count
        c.cycles += 4;
        c.g[T0][0] = c.lw(c.g[T6][0] as u32);
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T5][0] as i64)) as u64;
        let no_pass = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[T0][0];
        if !no_pass {
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[T4][0] = c.lw(c.g[V1][0] as u32); // vals[i]
                c.g[T1][0] = c.lw(c.g[V0][0] as u32); // key = keys[i]
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    let ordered = c.g[V1][0] == 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    if ordered {
                        break;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]); // keys[j+gap] = keys[j]
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    let more = (c.g[A3][0] as i64) >= 0;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]); // vals[j+gap] = vals[j]
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_dac8);
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A1][0] = c.g[T3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[T5][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[T1][0]); // keys[j+gap] = key
                let next_i = c.g[A0][0] != 0;
                c.sw(c.g[V0][0] as u32, c.g[T4][0]); // vals[j+gap] = val
                if !next_i {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_daa8);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
        let next_gap = c.g[V0][0] == 0;
        c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_sub(4));
        if !next_gap {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0032_da98);
        }
    }
}

/// `void func_00399820(UIListBox* self, sUIData* data)` (0x399820): load the
/// list box's shared UI-object data, take its horizontal alignment and
/// visibility from `data`, and when the "wrap" flag is set compute the four
/// texel-size floats from the byte channels at `data+0x24`.
pub fn uilistbox_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let data = c.g[A1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = data;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0039_983c);
        c.g[S1][0] = c.g[A0][0];
        c.pc = 0x0039_fe00;
        ee::s13::UIObject_Load(c);
        if c.pc != 0x0039_983c {
            return Err(Unwound);
        }
        c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x20));
        c.g[A0][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 8)); // vtable
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x74), c.g[V0][0]);
        c.g[A0][0] = c.lh(p(c.g[A2][0] as u32, 0xb8));
        c.g[A1][0] = c.lhu(p(c.g[S0][0] as u32, 0x22));
        c.g[V1][0] = c.lw(p(c.g[A2][0] as u32, 0xbc));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039_9874);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        call_ptr(c, func, 0x0039_9874)?;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            c.g[V0][0] = c.lbu(p(c.g[S0][0] as u32, 0x27));
            c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x24));
            c.g[A0][0] = c.lbu(p(c.g[S0][0] as u32, 0x25));
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x26));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            c.lwc1(0, gp(c, -0x2544)); // texel scale
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            c.swc1(4, p(c.g[SP][0] as u32, 0xc));
            c.swc1(2, c.g[SP][0] as u32);
            c.swc1(3, p(c.g[SP][0] as u32, 4));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.ldl(2, p(c.g[SP][0] as u32, 7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 0xf));
            c.ldr(3, p(c.g[SP][0] as u32, 8));
            c.sdl(2, p(c.g[S1][0] as u32, 0x7f));
            c.sdr(2, p(c.g[S1][0] as u32, 0x78));
            c.sdl(3, p(c.g[S1][0] as u32, 0x87));
            c.sdr(3, p(c.g[S1][0] as u32, 0x80));
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x14));
        } else {
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x14));
        }
        c.cycles += 7;
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.g[V0][0] = c.g[V0][0] | 0x80;
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.sw(p(c.g[S1][0] as u32, 0x14), c.g[V0][0]);
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        frame.close(c);
        Ok(())
    })
}

/// `void func_003A03F0(UIText* self, sUIData* data)` (0x3a03f0): the text
/// element's load: the shared object load, alignment and visibility, the text
/// id from `data+0x24`, and with the wrap flag the four texel-size floats from
/// the byte channels at `data+0x28`.
pub fn uitext_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        let data = c.g[A1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = data;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003a_040c);
        c.g[S1][0] = c.g[A0][0];
        c.pc = 0x0039_fe00;
        ee::s13::UIObject_Load(c);
        if c.pc != 0x003a_040c {
            return Err(Unwound);
        }
        c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x20));
        c.g[A0][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 8));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x74), c.g[V0][0]);
        c.g[A0][0] = c.lh(p(c.g[A2][0] as u32, 0xb8));
        c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x22));
        c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0xbc));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003a_0444);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        call_ptr(c, func, 0x003a_0444)?;
        c.g[A1][0] = c.lw(p(c.g[S0][0] as u32, 0x24));
        c.g[RA][0] = sx(0x003a_0450);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x003a_1000;
        ee::s13::UIText_SetTextId(c);
        if c.pc != 0x003a_0450 {
            return Err(Unwound);
        }
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        } else {
            c.cycles += 30;
            c.g[V0][0] = c.lbu(p(c.g[S0][0] as u32, 0x2b));
            c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x28));
            c.g[A0][0] = c.lbu(p(c.g[S0][0] as u32, 0x29));
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x2a));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            c.lwc1(0, gp(c, -0x24f8)); // texel scale
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            c.swc1(4, p(c.g[SP][0] as u32, 0xc));
            c.swc1(2, c.g[SP][0] as u32);
            c.swc1(3, p(c.g[SP][0] as u32, 4));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.ldl(2, p(c.g[SP][0] as u32, 7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 0xf));
            c.ldr(3, p(c.g[SP][0] as u32, 8));
            c.sdl(2, p(c.g[S1][0] as u32, 0x7f));
            c.sdr(2, p(c.g[S1][0] as u32, 0x78));
            c.sdl(3, p(c.g[S1][0] as u32, 0x87));
            c.sdr(3, p(c.g[S1][0] as u32, 0x80));
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        }
        c.cycles += 4;
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `OVScreenTrophy_OnWidgetAction` (0x20e9a0) and
/// `OVScreenRewardsListPopup_OnWidgetAction` (0x200288): the same screen
/// widget-action handler compiled twice. `ret0` is the return address of its
/// first nested call; the others follow at fixed offsets.
fn ov_screen_widget_action(c: &mut Cpu, ret0: u32) -> Flow {    c.cycles += 7;
    let frame = Frame::open(c, 0x40);
    let widget = c.g[A1][0];
    frame.save(c, 16, 0x30);
    frame.save(c, 17, 0x20);
    c.g[S0][0] = widget;
    frame.save_ra(c, 0x10);
    let no_widget = c.g[S0][0] == 0;
    c.g[S1][0] = c.g[A0][0];
    if !no_widget {
        c.cycles += 3;
        c.g[V0][0] = 0x5;
        let action5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 0x6;
        if action5 {
            // a "clicked" action: tell the widget (slot 0x58) the new state
            c.cycles += 16;
            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 8));
            c.g[A1][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x58));
            c.swc1(0, p(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.sw(p(c.g[SP][0] as u32, 8), 0);
            c.sw(p(c.g[SP][0] as u32, 0xc), 0);
            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x5c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(ret0);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, ret0)?;
            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 0x18));
            c.g[V0][0] = 0x1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = 0;
            if !not_first {
                c.cycles += 10;
                c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x20));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x24));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x34);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x34)?;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
            c.g[RA][0] = sx(ret0 + 0x44);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f400, ret0 + 0x44)?;
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        } else {
            c.cycles += 2;
            if c.g[A2][0] == c.g[V0][0] {
                // the "second" action: hand the widget's state to the screen
                c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                c.cycles += 10;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(p(c.g[S0][0] as u32, 0x18));
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x28));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x2c));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x6c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x6c)?;
                let unchanged = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !unchanged {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x10));
                    c.g[RA][0] = sx(ret0 + 0x80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, ret0 + 0x80)?;
                }
            } else {
                c.cycles += 2;
                c.lq(16, p(c.g[SP][0] as u32, 0x30));
            }
        }
    }
    c.cycles += 1;
    c.lq(16, p(c.g[SP][0] as u32, 0x30));
    c.cycles += 4;
    c.lq(17, p(c.g[SP][0] as u32, 0x20));
    c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    Ok(())
}

/// `void func_0020E9A0(OVScreen*, Widget* w, int action)` (0x20e9a0): the
/// trophy screen's widget-action handler.
pub fn ovscreen_trophy_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_ea08))
}

/// `void func_00200288(OVScreen*, Widget* w, int action)` (0x200288): the
/// rewards-list popup's widget-action handler (the same code).
pub fn ovscreen_rewards_list_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_02f0))
}

/// `int func_003592D0(AnimDelta* s)` (0x3592d0): tick a particle/animation
/// delta: reset the two 1e10 guards, and when the object is not attached to a
/// live animation, count the delta down (clamped at 0), run the animation
/// object step once it reaches 0, and flag the parent; an attached object gets
/// its callback with the "0x30" payload. Returns 1 except when the callback
/// path ran.
pub fn anim_delta_v15(c: &mut Cpu) {
    /// What the tail of the function does.
    enum Tail {
        /// "+1; v0 = 1": report the object as alive.
        Alive,
        /// "+2; *(u16*)(s+0x42) |= 1; +1; v0 = 1".
        Flag,
        /// Return straight away (the callback path already set `v0`).
        Return,
    }
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.lwc1(0, gp(c, -0x2a6c)); // 1e10
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = c.g[S0][0] as u32;
        c.swc1(0, p(s, 0x28));
        c.g[V0][0] = c.lw(p(s, 8));
        let attached = c.g[V0][0] != 0;
        c.swc1(0, p(s, 0x24));
        let tail;
        if attached {
            tail = Tail::Alive;
        } else {
            c.cycles += 6;
            c.lwc1(1, p(s, 0x6c)); // v
            c.mtc1(0, 2); // f2 = 0.0
            c.c_lt_s(2, 1); // 0 < v
            let v_le_zero = !c.fcc();
            c.g[V0][0] = c.lw(p(s, 0xc)); // t
            if v_le_zero {
                // t > 0: count it down
                c.cycles += 2;
                let t_le_zero = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if !t_le_zero {
                    c.cycles += 1;
                    c.sw(p(s, 0xc), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(s, 8));
                let no_parent = c.g[V0][0] == 0;
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                if no_parent {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu(p(c.g[S0][0] as u32, 0x42));
                    tail = Tail::Flag;
                } else {
                    // the slot-0x110 callback: the object is attached elsewhere
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 0x3c));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x110));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x114));
                    let func = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0035_93a4);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, func, 0x0035_93a4)?;
                    c.g[V0][0] = 0;
                    tail = Tail::Return;
                }
            } else if (c.g[V0][0] as i64) < 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lhu(p(c.g[S0][0] as u32, 0x42));
                tail = Tail::Flag;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    if !c.fcc() {
                        // t == 0 and v > 0: bleed the delta by |s+0x10|
                        c.cycles += 7;
                        c.lwc1(0, p(c.g[S0][0] as u32, 0x10));
                        c.abs_s(0, 0);
                        c.sub_s(0, 1, 0);
                        c.c_lt_s(0, 2);
                        let negative = !c.fcc();
                        c.swc1(0, p(c.g[S0][0] as u32, 0x6c));
                        if !negative {
                            c.cycles += 1;
                            c.swc1(2, p(c.g[S0][0] as u32, 0x6c));
                        }
                    }
                }
                c.cycles += 8;
                c.g[RA][0] = sx(0x0035_934c);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x0034_1d48;
                ee::s11::AnimObject_v15(c);
                if c.pc != 0x0035_934c {
                    return Err(Unwound);
                }
                c.lwc1(1, p(c.g[S0][0] as u32, 0x6c));
                c.mtc1(0, 0); // f0 = 0.0
                c.c_lt_s(0, 1); // 0 < v
                let stopped = !c.fcc(); // v <= 0 after the step
                c.g[V0][0] = 0x1;
                if stopped {
                    c.cycles += 2;
                    c.sw(p(c.g[S0][0] as u32, 4), 0);
                    tail = Tail::Alive;
                } else {
                    c.cycles += 2;
                    c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
                    tail = Tail::Return;
                }
            }
        }
        match tail {
            Tail::Alive => {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            Tail::Flag => {
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.sh(p(c.g[S0][0] as u32, 0x42), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            Tail::Return => {}
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_002821A0(ScriptEngine* self, int key, void* arg)` (0x2821a0):
/// start the script bound to slot `key` of the engine's script table: allocate
/// a thread, and either queue it for a sequence (when the slot has data) or
/// hand it to the slot's start callback; a refused start tears the thread down
/// again and returns -1.
pub fn script_engine_start_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 20, 0x10);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        c.g[A1][0] = 0x1;
        c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x2b0));
        c.g[RA][0] = sx(0x0028_21dc);
        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.pc = 0x0028_2c38;
        ee::s07::ScriptEngine_AllocProcess(c);
        if c.pc != 0x0028_21dc {
            return Err(Unwound);
        }
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S3][0] as u32); // slot data
        let has_data = c.g[V0][0] != 0;
        c.g[V0][0] = 0x2;
        let mut refused = false;
        if has_data {
            c.cycles += 16;
            c.sw(p(c.g[S0][0] as u32, 8), c.g[S2][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x2b8));
            c.g[RA][0] = sx(0x0028_224c);
            c.g[A1][0] = c.lw(c.g[S3][0] as u32);
            c.pc = 0x0027_2938;
            ee::s07::InSeq_StartSequence(c);
            if c.pc != 0x0028_224c {
                return Err(Unwound);
            }
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x2b8));
            c.g[RA][0] = sx(0x0028_2258);
            c.g[A1][0] = c.g[V0][0];
            c.pc = 0x0027_2cc0;
            ee::s07::InSeq_GetInstance(c);
            if c.pc != 0x0028_2258 {
                return Err(Unwound);
            }
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0028_2268);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.pc = 0x0027_4c10;
            ee::s07::InSeq_InstanceSetUserData(c);
            if c.pc != 0x0028_2268 {
                return Err(Unwound);
            }
            c.g[V1][0] = c.lhu(p(c.g[S3][0] as u32, 4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.sh(p(c.g[S3][0] as u32, 4), c.g[V1][0]);
            c.sw(p(c.g[S0][0] as u32, 0xc), c.g[S4][0]);
        } else {
            c.cycles += 12;
            c.g[V0][0] = 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x14), c.g[S2][0]);
            c.sh(p(c.g[S0][0] as u32, 0x12), c.g[V0][0]);
            c.g[A1][0] = c.g[S2][0];
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x2a8));
            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x38));
            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x3c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0028_2214);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0028_2214)?;
            if c.g[V0][0] != 0 {
                c.sw(p(c.g[S0][0] as u32, 0xc), c.g[S4][0]);
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0x7;
                c.g[A0][0] = c.g[S1][0];
                c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
                c.g[RA][0] = sx(0x0028_2230);
                c.g[A1][0] = c.g[S0][0];
                c.pc = 0x0028_2c88;
                ee::s07::ScriptEngine_ClearProcess(c);
                if c.pc != 0x0028_2230 {
                    return Err(Unwound);
                }
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                refused = true;
            }
        }
        if !refused {
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x1c));
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWorldSphTree* func_003303F0(cWorldSphTree* self, SphereNode* owner, int flag)`
/// (0x3303f0): construct a collision sphere tree over `owner`: set its vtable
/// and counters, copy the owner's sphere, derive the plus/minus boxes from the
/// radius, and allocate and initialise the node storage.
pub fn coll_sphere_tree_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, 17, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1a10));
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.sw(p(c.g[S1][0] as u32, 0x50), c.g[V0][0]); // vtable
        c.sw(p(c.g[S1][0] as u32, 8), 0);
        let no_flag = c.g[A2][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 4), 0);
        if no_flag {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        } else {
            c.sw(c.g[S1][0] as u32, 0);
        }
        c.cycles += 47;
        c.sw(p(c.g[S1][0] as u32, 0x64), c.g[A1][0]);
        c.g[A0][0] = c.g[A1][0];
        c.sw(p(c.g[S1][0] as u32, 0x60), c.g[A1][0]);
        c.lq(2, p(c.g[A1][0] as u32, 0x80)); // the owner's sphere
        c.sq(2, c.g[SP][0] as u32);
        call_ptr(c, 0x0032_c590, 0x0033_0444)?; // SphereTree_GetRadius(owner)
        c.lwc1(3, c.g[SP][0] as u32); // cx
        c.g[A1][0] = 0x49_0000;
        c.lwc1(2, p(c.g[SP][0] as u32, 4)); // cy
        c.g[A2][0] = 0;
        c.lwc1(1, p(c.g[SP][0] as u32, 8)); // cz
        c.add_s(4, 3, 0); // cx + r
        c.add_s(5, 2, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 7); // f7 = 1.0
        c.add_s(6, 1, 0);
        c.lq(2, c.g[SP][0] as u32);
        c.swc1(4, p(c.g[SP][0] as u32, 0x10));
        c.sub_s(1, 1, 0);
        c.swc1(5, p(c.g[SP][0] as u32, 0x14));
        c.sub_s(3, 3, 0);
        c.swc1(6, p(c.g[SP][0] as u32, 0x18));
        c.sub_s(2, 2, 0);
        c.swc1(7, p(c.g[SP][0] as u32, 0x1c));
        c.g[A3][0] = 0;
        c.sq(2, p(c.g[S1][0] as u32, 0x40)); // the sphere itself
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ab0)); // D_0048E550
        c.g[A0][0] = 0x140;
        c.lq(3, p(c.g[SP][0] as u32, 0x10));
        c.swc1(3, p(c.g[SP][0] as u32, 0x10));
        c.swc1(2, p(c.g[SP][0] as u32, 0x14));
        c.swc1(1, p(c.g[SP][0] as u32, 0x18));
        c.swc1(7, p(c.g[SP][0] as u32, 0x1c));
        c.sq(3, p(c.g[S1][0] as u32, 0x20)); // plus box
        c.lq(2, p(c.g[SP][0] as u32, 0x10));
        c.sq(2, p(c.g[S1][0] as u32, 0x30)); // minus box
        call_ptr(c, 0x0031_7d70, 0x0033_04c0)?; // EA_MemAllocNamed(0x140, tag, 0, 0)
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0032_c508, 0x0033_04cc)?; // SphereTree_ctor(nodes)
        c.sw(p(c.g[S1][0] as u32, 0x68), c.g[S0][0]);
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, p(c.g[SP][0] as u32, 0x40));
        c.lq(17, p(c.g[SP][0] as u32, 0x30));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x20));
        frame.close(c);
        Ok(())
    })
}

/// `void* func_003A7C88(WorldCacheTable* self, short* sizes)` (0x3a7c88):
/// build the world-cache table: keep the 28 class sizes, total them, allocate
/// and clear the entry pool, then point the non-empty classes at their slices.
pub fn section_table_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0xe0;
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        c.sw(p(c.g[S1][0] as u32, 0xe4), c.g[A1][0]); // the size table
        c.g[A1][0] = 0;
        call_ptr(c, 0x0041_6210, 0x003a_7cb8)?; // memset(entries, 0, 0xe0)
        c.g[S0][0] = 0; // total
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[S2][0];
        loop {
            c.cycles += 10;
            c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0xe4));
            c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = ((c.g[A2][0] as i64) < 28) as u64;
            c.g[V1][0] = c.lh(c.g[V0][0] as u32);
            c.sw(c.g[A1][0] as u32, c.g[V1][0]); // counts[i]
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let more = c.g[A0][0] != 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_7cc8);
            }
        }
        c.cycles += 18;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[A1][0] = 0x49_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e18)); // D_00494E18
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x003a_7d0c)?; // operator new(total * 4)
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]); // self->pool
        call_ptr(c, 0x0041_6210, 0x003a_7d20)?; // clear the pool
        c.g[T0][0] = c.lw(c.g[S1][0] as u32); // pool
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        c.g[A2][0] = 0x1b;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32); // count
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
            if !empty {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]); // ptr = pool + offset
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
            if !more {
                c.cycles += 7;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, 16, 0x30);
                frame.restore(c, 17, 0x20);
                frame.restore(c, 18, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_7d38);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, put_vec, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    /// The `BELibrary` singleton (`gp - 0x1f04`) and the world (`gp - 0x848`).
    fn globals(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// Scrambled state, the fake call hooks, `gp` and the globals.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        globals(c);
        r
    }

        #[test]
    fn cps2_grid_mesh_pack_color_matches() {
        strict(ee::s12::f_00385aa0__generated, cps2_grid_mesh_pack_color, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x5aa0, 0);
            let colour = DATA + 0x1000;
            let buf = DATA + 0x2000;
            let idx = (r.next_u32() % 8) as u32;
            // clamped and out-of-range channels on both sides
            for k in 0..4 {
                let v = match (seed as usize + k) % 4 {
                    0 => 0u32,
                    1 => 0x3f80_0000,
                    2 => r.f32(2.0),
                    _ => (r.next_u32() & 0x807f_ffff) | 0x3f00_0000,
                };
                w(c, colour + 4 * k as u32, v);
            }
            c.g[A0][0] = sx(DATA + 0x3000);
            c.g[A1][0] = sx(buf);
            c.g[A2][0] = sx(idx);
            c.g[A3][0] = sx(colour);
        });
    }

    #[test]
    fn flex_bridge_init_segment_midpoints_matches() {
        strict(ee::s11::f_00346d38__generated, flex_bridge_init_segment_midpoints, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x6d38, 0);
            let this = DATA + 0x1000;
            let pts = DATA + 0x4000;
            let segs = DATA + 0x6000;
            let count = (seed % 4) as u32;
            w(c, this + 0x50, count);
            w(c, this + 0x7c, pts);
            w(c, this + 0x74, segs);
            for k in 0..(2 * count).max(1) {
                let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, pts + 0x10 * k, v);
            }
            c.g[A0][0] = sx(this);
        });
    }

    /// The rider character record `BENewPlayerInt_*Difficulty` reads:
    /// `BE_GetRiderCharacter(0)` resolves a byte of it through two tables.
    fn difficulty_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base(c, seed, salt, 0);
        let character = (seed % 4) as u32;
        w(c, 0x0053_05b0, character);
        c.bus.write8(0x0053_5b30 + character * 0x1c + 1, character as u8);
        let rec = 0x004a_6ca8 + character * 0xf88;
        let edge = [0u16, 1, 2, 3, 0xffff];
        let a = edge[((seed / 4) % 5) as usize];
        let b = edge[((seed / 20) % 5) as usize];
        c.bus.write16(rec + 0x280, a);
        c.bus.write16(rec + 0x284, a);
        c.bus.write16(rec + 0x282, b);
        c.bus.write16(rec + 0x286, b);
        r
    }

    #[test]
    fn benewplayerint_raise_difficulty_matches() {
        strict(ee::s01::f_00147d20__generated, benewplayerint_raise_difficulty, SEEDS, &|c, seed| {
            let _ = difficulty_fixture(c, seed, 0x7d20);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn benewplayerint_lower_difficulty_matches() {
        strict(ee::s01::f_00147e18__generated, benewplayerint_lower_difficulty, SEEDS, &|c, seed| {
            let _ = difficulty_fixture(c, seed, 0x7e18);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn shell_sort_desc_key_val_matches() {
        strict(ee::s11::f_0032da40__generated, shell_sort_desc_key_val, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xda40, 0);
            let keys = DATA + 0x1000;
            let vals = DATA + 0x2000;
            let n = (seed % 7) as u32;
            // the gap table the sort walks down
            for (i, g) in [1u32, 4, 13, 40, 121, 364].iter().enumerate() {
                w(c, 0x0044_af98 + 4 * i as u32, *g);
            }
            for i in 0..n {
                let k = if i > 0 && seed % 3 == 0 { i - 1 } else { r.next_u32() % 100 };
                w(c, keys + 4 * i, k);
                w(c, vals + 4 * i, r.next_u32());
            }
            c.g[A0][0] = n as u64;
            c.g[A1][0] = sx(keys);
            c.g[A2][0] = sx(vals);
        });
    }

    /// A UI element whose vtable slots are fake calls, plus its data block.
    fn ui_fixture(c: &mut Cpu, seed: u32, salt: u64, slots: &[u32]) -> (u32, u32) {
        let mut r = base(c, seed, salt, 0);
        let this = DATA + 0x1000;
        let data = DATA + 0x2000;
        let vt = crate::difftest::fakes::vtable(c, &mut r, slots);
        w(c, this + 8, vt);
        w(c, this + 0x14, r.next_u32());
        // bit 2 of +0x74 survives the load and selects the texel block
        w(c, this + 0x74, if seed % 2 == 0 { r.next_u32() & !4 } else { r.next_u32() | 4 });
        w(c, this + 0x78, r.next_u32());
        w(c, this + 0x80, r.next_u32());
        for k in 0..0x30u32 {
            c.bus.write8(data + 0x20 + k, r.next_u32() as u8);
        }
        w(c, data + 4, r.next_u32());
        w(c, data + 8, r.next_u32());
        w(c, data + 0xc, r.next_u32());
        w(c, data + 0x24, r.next_u32());
        w(c, GAME_GP.wrapping_sub(0x2544), r.f32(4.0));
        w(c, GAME_GP.wrapping_sub(0x24f8), r.f32(4.0));
        (this, data)
    }

    #[test]
    fn uilistbox_load_matches() {
        strict(ee::s13::f_00399820__generated, uilistbox_load, SEEDS, &|c, seed| {
            let (this, data) = ui_fixture(c, seed, 0x9820, &[0x48, 0x4c, 0xb8, 0xbc]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(data);
        });
    }

    #[test]
    fn uitext_load_matches() {
        strict(ee::s13::f_003a03f0__generated, uitext_load, SEEDS, &|c, seed| {
            let (this, data) = ui_fixture(c, seed, 0x03f0, &[0x28, 0x2c, 0x48, 0x4c, 0xb8, 0xbc]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(data);
        });
    }

    /// A vtable at `at` whose slots are fake calls (all with delta 8).
    fn fake_table(c: &mut Cpu, at: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, 8u16);
            w(c, at + off + 4, crate::difftest::fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// The screen + widget pair an `OVScreen*_OnWidgetAction` gets.
    fn action_fixture(c: &mut Cpu, seed: u32, salt: u64) -> (u32, u32) {
        let mut r = base(c, seed, salt, 0);
        let this = DATA + 0x1000;
        let widget = DATA + 0x2000;
        let obj = DATA + 0x3000;
        let inner = DATA + 0x4000;
        fake_table(c, DATA + 0x5000, &[0x58, 0x5c]);
        fake_table(c, DATA + 0x6000, &[0x20, 0x24, 0x28, 0x2c]);
        w(c, widget + 8, DATA + 0x5000);
        w(c, widget + 0x18, if seed % 3 == 0 { 1 } else { r.next_u32() });
        w(c, this + 0x10, obj);
        w(c, obj, inner);
        w(c, inner + 4, DATA + 0x6000);
        w(c, obj + 0x18, r.next_u32());
        (this, widget)
    }

    /// A particle/animation delta object for `anim_delta_v15`.
    fn anim_delta_fixture(c: &mut Cpu, seed: u32, salt: u64) -> u32 {
        let mut r = base(c, seed, salt, 0);
        let s = DATA + 0x1000;
        let kind = seed % 9;
        w(c, s + 8, match kind {
            0 => 1,
            4 => 2,
            _ => 0,
        });
        w(c, s + 4, 0);
        w(c, s + 0x1c, DATA + 0x3000);
        // the "attached elsewhere" callback slot
        fake_table(c, DATA + 0x7000, &[0x110, 0x114]);
        w(c, s + 0x3c, DATA + 0x7000);
        c.bus.write16(s + 0x42, (r.next_u32() & 0x3) as u16);
        let v = match kind {
            1 | 2 | 4 => 0.0f32,
            _ => 1.5f32,
        };
        w(c, s + 0x6c, v.to_bits());
        let abs = match kind {
            5 => 1.5f32,
            6 => 0.25f32,
            8 => 2.5f32,
            _ => 0.75f32,
        };
        w(c, s + 0x10, abs.to_bits());
        w(c, s + 4, 0);
        w(c, s + 0xc, match kind {
            2 => 3,
            3 => 0xffff_ffff,
            7 => 2,
            _ => 0,
        });
        s
    }

        #[test]
    fn script_engine_start_script_matches() {
        // Only the "slot has no sequence data" arm: the sequence arm needs a
        // live sequence manager behind the engine.
        strict(ee::s07::f_002821a0__generated, script_engine_start_script, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x21a0, 1);
            let engine = DATA + 0x1000;
            let slots = DATA + 0x4000;
            let callbacks = DATA + 0x5000;
            let key = (seed % 4) as u32;
            w(c, engine + 0x2b0, slots);
            w(c, engine + 0x2a8, callbacks);
            fake_table(c, callbacks, &[0x38]);
            w(c, slots + 8 * key, 0);
            c.bus.write16(slots + 8 * key + 4, r.next_u32() as u16);
            // the start callback's verdict: refuse on half the seeds
            w(
                c,
                crate::difftest::fakes::LOG_RESULTS,
                if seed % 2 == 0 { 0 } else { DATA + 0x7000 },
            );
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = key as u64;
            c.g[A2][0] = sx(DATA + 0x3000);
        });
    }

    /// The engine's size-class pool allocator behind `gp + 0xdd8`.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x400 * i as u32;
            for n in 0..8u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

        #[test]
    fn section_table_ctor_matches() {
        strict(ee::s13::f_003a7c88__generated, section_table_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x7c88, 0);
            pool(c);
            let this = DATA + 0x1000;
            let sizes = DATA + 0x2000;
            for i in 0..28u32 {
                // some empty classes, some counted ones
                let n = if (seed as u32 + i) % 5 == 0 { 0 } else { (r.next_u32() % 2) as u16 };
                c.bus.write16(sizes + 2 * i, n);
            }
            // at least one class must be non-empty: the allocation is size 0 otherwise
            c.bus.write16(sizes, 3);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(sizes);
        });
    }

        }
