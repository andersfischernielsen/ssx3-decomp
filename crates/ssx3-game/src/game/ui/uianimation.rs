//! `src/ui/uianimation.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::render::graphicsman::RegMem;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_7140, ui_anim_set_create_anim, "UIAnimSet_CreateAnim"),
    (0x0039_71c8, uianim_eval_channel, "UIAnim_EvalChannel"),
    (
        0x0039_7278,
        uianim_get_frame_count_safe,
        "UIAnim_GetFrameCountSafe",
    ),
    (0x0039_73a8, ui_anim_dtor, "UIAnim_dtor"),
    (0x0039_7440, uianim_is_done, "UIAnim_IsDone"),
    (0x0039_7458, uianim_get_frame_count, "UIAnim_GetFrameCount"),
    (0x0039_7468, uianim_set_play_mode, "UIAnim_SetPlayMode"),
    (0x0039_74b0, ui_anim_advance, "UIAnim_Advance"),
    (0x0039_75e0, ui_anim_apply, "UIAnim_Apply"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;

    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;

    pub const UI_ANIM_EVAL_CHANNEL: Callee = Callee {
        addr: 0x0039_71c8,
        f: ee::s13::f_003971c8,
    };

    pub const UI_ANIM_GET_FRAME_COUNT_SAFE: Callee = Callee {
        addr: 0x0039_7278,
        f: ee::s13::f_00397278,
    };
}

// ---------------------------------------------------------------------------
// UI animation

/// `int UIAnim_GetFrameCountSafe(UIAnim **anim)` (0x397278): frame count at
/// `*anim + 2`, or 0 when the animation is null.
pub fn uianim_get_frame_count_safe(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
    abi::ret(c);
}

/// `bool UIAnim_IsDone(UIAnim *anim)` (0x397440): true when the current frame
/// (signed 16-bit at +0xa) has reached the frame count at `*anim + 2`.
pub fn uianim_is_done(c: &mut Cpu) {
    c.cycles += 6;
    let anim = Addr(abi::arg(c, 0));
    c.g[A1][0] = anim.lw(c);
    c.g[V0][0] = anim.off(0xa).lh(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x2).lhu(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    abi::ret(c);
}

/// `int UIAnim_GetFrameCount(UIAnim *anim)` (0x397458): signed frame count at
/// `*anim + 2`.
pub fn uianim_get_frame_count(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2).lh(c);
    abi::ret(c);
}

/// `void UIAnim_SetPlayMode(UIAnim *anim, int mode, int loops)` (0x397468):
/// set the play mode byte and initialise the loop counter.
pub fn uianim_set_play_mode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A1][0] = c.g[A1][0] & 0x7;
    c.g[A2][0] = c.g[A2][0] & 0xffff;
    if c.g[A1][0] == 0 {
        c.cycles += 1;
        Addr(c.g[A3][0] as u32).off(0xc).sh(c, c.g[A2][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = 0x2;
    if c.g[A1][0] != c.g[V0][0] {
        Addr(c.g[A3][0] as u32).off(0xa).sh(c, 0);
        c.cycles += 2;
        Addr(c.g[A3][0] as u32).off(0xe).sb(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
    Addr(c.g[A3][0] as u32).off(0xe).sb(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    Addr(c.g[A3][0] as u32).off(0xa).sh(c, c.g[V1][0]);
    abi::ret(c);
}

/// `UIAnim* UIAnimSet_CreateAnim(UIAnimSet* self, u32 id)` (0x397140): find
/// the animation entry with `id`, allocate it and run the animation
/// constructor. 0 when there is no such entry.
pub fn ui_anim_set_create_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x20);
        let sp0 = spv(c);
        c.g[A3][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x10));
        c.g[A2][0] = 0;
        c.sd(sp0, c.g[RA][0]);
        c.g[V0][0] = lw_at(c, c.g[A3][0], 0);
        let set = c.g[V0][0];
        c.g[V1][0] = lw_at(c, set, 0);
        let empty = c.g[V1][0] == 0;
        c.g[S0][0] = sx(at(set as u32, 4));
        let mut made = false;
        if !empty {
            c.cycles += 1;
            c.g[T0][0] = 0x4a0000;
            loop {
                c.cycles += 3;
                let ent = c.g[S0][0];
                c.g[V0][0] = lw_at(c, ent, 8);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[A0][0] = lw_at(c, c.g[A3][0], 0);
                } else {
                    c.cycles += 10;
                    c.g[A1][0] = sx(at(c.g[T0][0] as u32, 0x4768));
                    c.g[A0][0] = 0x14;
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0039_7188)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    callee::UI_ANIM_CTOR.call(c, 0x0039_7194)?;
                    c.lq(16, at(sp0, 0x10));
                    made = true;
                    break;
                }
                c.cycles += 6;
                c.g[A2][0] = sx(at(c.g[A2][0] as u32, 1));
                c.g[V1][0] = lw_at(c, c.g[S0][0], 0xc);
                c.g[V0][0] = lw_at(c, c.g[A0][0], 0);
                c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7168);
                }
            }
        }
        if !made {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.lq(16, at(sp0, 0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `UIAnim_dtor` (0x3973a8): release the animation's element array (when
/// present) and optionally free the object itself (`a1 & 1`).
pub fn ui_anim_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4d08));
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        Addr(p(this, 0x10)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(this, 4)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            let start = r(c, V1);
            c.g[V0][0] = Addr(p(start, 0u32.wrapping_sub(0x10))).lw(c);
            c.g[A0][0] = sx(start);
            c.g[V0][0] = shl32(c.g[V0][0], 2);
            c.g[V0][0] = sx(start.wrapping_add(r(c, V0)));
            let empty = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 7;
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_73f0);
                        }
                    } else {
                        break;
                    }
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x0039_7414);
            c.g[A0][0] = sx(start.wrapping_sub(0x10));
            callee::EA_MEM_FREE.call(c, 0x0039_7414)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_7428);
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0039_7428)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIAnim_EvalChannel` (0x3971c8): evaluate one animation channel at frame
// `t`: find the key interval containing it and interpolate; 0 outside.
pub fn uianim_eval_channel(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = c.lw(c.g[A0][0] as u32);
    if c.g[A0][0] == 0 {
        c.cycles += 3;
        c.mtc1(0, 0);
        abi::ret(c);
        return;
    }
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xa));
    c.cycles += 10;
    c.g[T0][0] = 0x1;
    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x6));
    c.g[T2][0] = 0u64;
    c.mtc1(2, 3);
    c.cvt_s_w(3, 3);
    c.g[A3][0] = 0u64;
    c.mtc1(0, 1);
    c.g[V0][0] = (c.g[T0][0] < c.g[V1][0]) as u64;
    let single = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    if !single {
        c.cycles += 2;
        c.g[T1][0] = c.g[V1][0];
        loop {
            c.cycles += 8;
            c.g[A2][0] = c.lhu(c.g[A0][0] as u32);
            c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x2));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.mtc1(3, 1);
            c.cvt_s_w(1, 1);
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let past = c.g[V0][0] == 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            if past {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                c.mtc1(6, 2);
                c.cvt_s_w(2, 2);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.sub_s(1, 1, 3);
                c.div_s(0, 0, 2);
                c.mul_s(0, 0, 1);
                c.add_s(0, 0, 3);
                abi::ret(c);
                return;
            }
            c.cycles += 5;
            c.g[T2][0] = c.g[A3][0];
            c.mov_s(3, 1);
            c.g[V0][0] = (c.g[T0][0] < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_7238);
            }
        }
    }
    c.cycles += 2;
    c.mov_s(0, 1);
    abi::ret(c);
}

/// `void UIAnim_Advance(UIAnim* self, unsigned flags)` (0x3974b0): step the
/// animation by `flags & 7` (mode 1 forward, 2 back, 3 bounce using
/// `self->dir`), clamping to `data->count`; bit 8 wraps instead of clamping.
/// The frame is at `+0xa`, the direction at `+0xe`, the data at `+0`.
///
/// Animations are authored at 60 fps: above 60 Hz each animation steps once
/// per `rate / 60` ticks of its own (`tick::frame60_due`), so it keeps its
/// wall-clock speed. Every call steps at 60.
pub fn ui_anim_advance(c: &mut Cpu) {
    let anim = r32(c, A0);
    let frame = c.bus.read16(anim.wrapping_add(0xa)) as u32;
    if !crate::tick::frame60_due(c, anim, frame) {
        abi::ret(c);
        return;
    }
    ui_anim_step(c);
    let frame = c.bus.read16(anim.wrapping_add(0xa)) as u32;
    crate::tick::frame60_ran(c, anim, frame);
}

/// The step of [`ui_anim_advance`] (the original).
fn ui_anim_step(c: &mut Cpu) {
    'b_3974e0: {
        c.cycles += 3;
        c.g[A2][0] = c.ldw_2(A0, 0);
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[A1][0] & 0xff;
        if t {
            break 'b_3974e0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[A1][0] & 0x7;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x2;
            if t {
                break 'b_3974e0;
            } else {
                'b_39752c: {
                    'b_397520: {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                        if t {
                            c.cycles += 5;
                            c.g[V0][0] = c.ldhu_2(A0, 0xa);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.sth_2(A0, 0xa, c.g[V0][0]);
                            if t {
                                break 'b_3974e0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[A1][0] & 0x8;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_39752c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.ldhu_2(A2, 0x2);
                                    break 'b_397520;
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.ldb(A0, 0xe);
                                    'b_397578: {
                                        'b_397574: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = c.ldhu_2(A0, 0xa);
                                            if t {
                                                break 'b_397574;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_397578;
                                                } else {
                                                    break 'b_397574;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        break 'b_397578;
                                    }
                                    'b_3975b4: {
                                        'b_3975b0: {
                                            c.cycles += 4;
                                            c.sth_2(A0, 0xa, c.g[V0][0]);
                                            c.g[V0][0] = c.ldb(A0, 0xe);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A2][0] = c.ldbu(A0, 0xe);
                                            if t {
                                                break 'b_3975b0;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.ldw_2(A0, 0);
                                                c.g[V0][0] = c.ldh_2(A0, 0xa);
                                                c.g[V1][0] = c.ldhu_2(A1, 0x2);
                                                c.g[V0][0] = ((c.g[V0][0] as i64)
                                                    < (c.g[V1][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                                if t {
                                                    break 'b_3975b4;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = 0x1;
                                                    c.stb(A0, 0xe, c.g[V1][0]);
                                                    let t = true;
                                                    c.g[V0][0] = c.ldhu_2(A1, 0x2);
                                                    if t {
                                                        break 'b_397520;
                                                    } else {
                                                        break 'b_3975b0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                        break 'b_3975b4;
                                    }
                                    'b_3975d8: {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0x1;
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                        let t = c.g[V0][0] != c.g[V1][0];

                                        if t {
                                            break 'b_3975d8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.ldh_2(A0, 0xa);
                                            let t = (c.g[V0][0] as i64) >= 0;

                                            if t {
                                                break 'b_3975d8;
                                            } else {
                                                c.cycles += 2;
                                                c.sth_2(A0, 0xa, 0u64);
                                                c.stb(A0, 0xe, 0u64);
                                                break 'b_3975d8;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let ra = c.g[RA][0] as u32;
                                    c.pc = ra;
                                    return;
                                } else {
                                    c.cycles += 2;
                                    let ra = c.g[RA][0] as u32;
                                    c.pc = ra;
                                    return;
                                }
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.ldhu_2(A0, 0xa);
                                    c.cycles += 8;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sth_2(A0, 0xa, c.g[V0][0]);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                                    c.g[V1][0] = c.ldhu_2(A2, 0x2);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.g[A1][0] & 0x8;
                                    if t {
                                        break 'b_3974e0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_39752c;
                                        } else {
                                            c.cycles += 2;
                                            let ra = c.g[RA][0] as u32;
                                            c.sth_2(A0, 0xa, c.g[V0][0]);
                                            c.pc = ra;
                                            return;
                                        }
                                    }
                                } else {
                                    break 'b_3974e0;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let ra = c.g[RA][0] as u32;
                    c.sth_2(A0, 0xa, c.g[V0][0]);
                    c.pc = ra;
                    return;
                }
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.sth_2(A0, 0xa, 0u64);
                c.pc = ra;
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `void UIAnim_Apply(UIAnim* self, UIObject* target, u8 mode, u16 clock)`
/// (0x3975e0): when the animation's frame changed since it was last applied
/// (`+0x8`), evaluate each channel there (`UIAnim_EvalChannel`: linear
/// between integer keys) and pass it to the target's parameter setter
/// (vtable `+0x64`, `UIObject_SetAnimParam`). With mode 0 and a start
/// frame (`+0xc`), the frame is `clock - start`: the screen's timeline.
///
/// Above 60 Hz the animation's frame (or its timeline thread's) steps once
/// per `rate / 60` ticks (`tick::frame60_due`); on the ticks between steps
/// the channels are set to the value between this frame and the next one by
/// how far its clock is into the frame (`tick::frame60_phase`), so a
/// sliding, scaling or fading widget moves every tick. At or below 60 every
/// call runs the original.
pub fn ui_anim_apply(c: &mut Cpu) {
    let (anim, target, mode, clock, ret) = (
        r32(c, A0),
        r32(c, A1),
        r32(c, A2) & 0xff,
        r32(c, A3) & 0xffff,
        r32(c, RA),
    );
    ui_anim_apply_original(c);
    if c.pc != ret || crate::tick::rate(c) <= 60 {
        return;
    }
    // Leave the registers as the original does.
    let (g, f, hi, lo, fcr31) = (c.g, c.f, c.hi, c.lo, c.fcr31);
    // The blend's calls (the setter may check events) must not unwind:
    // nothing would resume this code. A thread switch or a finished frame
    // waits for the next check, as in `deliver_event_here`.
    let outer = std::mem::replace(&mut c.no_unwind, cfg!(feature = "ee-unwind"));
    let was = BLENDED.with(|b| b.borrow_mut().remove(&anim));
    let applied = c.bus.read16(anim.wrapping_add(0x8)) as u32;
    let flow = off_the_clock(c, |c| match blend_frames(c, anim, mode, clock) {
        Some((cur, next, w)) => {
            BLENDED.with(|b| {
                let mut b = b.borrow_mut();
                if b.len() > 4096 {
                    b.clear();
                }
                b.insert(anim, cur);
            });
            set_channels(c, anim, target, cur, next, w)
        }
        // Back to the frame's own values when the blend stops but the
        // frame stays (a held or stopped timeline). When the frame moved,
        // the original has just set the new frame's values.
        None if was == Some(applied) => {
            let data = c.bus.read32(anim);
            if data != 0 && applied < c.bus.read16(data.wrapping_add(2)) as u32 {
                set_channels(c, anim, target, applied, applied, 0.0)
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    });
    c.no_unwind = outer;
    if flow.is_err() {
        // (Only a call-depth unwind gets here.)
        return;
    }
    (c.g, c.f, c.hi, c.lo, c.fcr31) = (g, f, hi, lo, fcr31);
    c.pc = ret;
}

/// Run the blend off the guest's clock: no event may see a clock that is
/// then set back. Events are held for the whole blend (one that comes due
/// in it is delivered at the next guest check, at the cycle count it would
/// have had without the blend), and the clock is put back afterwards, so
/// the guest's timing is the original's.
fn off_the_clock<T>(c: &mut Cpu, blend: impl FnOnce(&mut Cpu) -> T) -> T {
    let (cycles, next_event) = (c.cycles, c.next_event);
    c.next_event = HELD;
    let r = blend(c);
    // A request made inside (an interrupt raised by a store sets
    // `next_event` to the clock) comes due at once.
    let asked = c.next_event != HELD;
    c.cycles = cycles;
    c.next_event = if asked {
        next_event.min(cycles)
    } else {
        next_event
    };
    r
}

/// `next_event` while [`off_the_clock`] holds events: never in the game; in
/// tests the cycle budget, so a blend that runs away still fails there.
#[cfg(not(test))]
const HELD: u64 = u64::MAX;
#[cfg(test)]
const HELD: u64 = crate::difftest::CYCLE_BUDGET;

/// `UIObject_SetAnimParam`'s one integer parameter (`cvt.w.s` into bits of
/// `+0x14`): never set to an in-between value.
const PARAM_INT: u32 = 12;

thread_local! {
    /// Animations whose channels were last set to in-between values, and the
    /// frame they were blended from.
    static BLENDED: std::cell::RefCell<std::collections::HashMap<u32, u32>> = Default::default();
}

/// The applied frame, the frame the animation takes next and how far it is
/// towards it (0 < w < 1), or `None` when nothing moves on this tick.
fn blend_frames(c: &mut Cpu, anim: u32, mode: u32, clock: u32) -> Option<(u32, u32, f32)> {
    let data = c.bus.read32(anim);
    if data == 0 {
        return None;
    }
    let count = c.bus.read16(data.wrapping_add(2)) as u32;
    let cur = c.bus.read16(anim.wrapping_add(0x8)) as u32;
    if cur >= count {
        return None;
    }
    let start = c.bus.read16(anim.wrapping_add(0xc)) as u32;
    let (next, phase) = if mode & 7 == 0 {
        // On the screen's timeline: its thread's next frame
        // (`UIScreen_RunThreadFrame`: `+0xc` steps by `+0x12` towards
        // `+0x10`, 0xffff for none, while it differs from `+0xe`).
        if start == 0xffff {
            return None;
        }
        let screen = crate::game::ui::uiscreen::updating_screen(c)?;
        let thread = crate::game::ui::uiscreen::timeline_thread(c, screen);
        if thread == 0 {
            return None;
        }
        let frame = c.bus.read16(thread.wrapping_add(0xc)) as u32;
        let end = c.bus.read16(thread.wrapping_add(0x10)) as u32;
        let step = c.bus.read8(thread.wrapping_add(0x12)) as i8 as i32;
        let running = c.bus.read16(thread.wrapping_add(0xe)) as u32 != frame;
        let moves = end == 0xffff || (step > 0 && frame < end) || (step < 0 && end < frame);
        if frame != clock || !running || !moves || step == 0 || frame.wrapping_sub(start) != cur {
            return None;
        }
        let next = (frame as i32 + step) as u32 & 0xffff;
        if next < start {
            return None;
        }
        (next - start, crate::tick::frame60_phase(c, thread, frame))
    } else {
        // Its own clock: the frame `UIAnim_Advance` steps to.
        let frame = c.bus.read16(anim.wrapping_add(0xa)) as u32;
        if frame != cur {
            return None;
        }
        (
            next_own_frame(c, anim, mode),
            crate::tick::frame60_phase(c, anim, frame),
        )
    };
    // One frame at a time, along the authored curve: never a wrap, a jump
    // or a multi-frame step.
    if phase == 0 || next >= count || next.abs_diff(cur) != 1 {
        return None;
    }
    Some((cur, next, phase as f32 / crate::tick::rate(c) as f32))
}

/// The frame [`ui_anim_step`] would leave the animation at (the animation
/// is put back; the clock by [`off_the_clock`]).
fn next_own_frame(c: &mut Cpu, anim: u32, mode: u32) -> u32 {
    let frame = c.bus.read16(anim.wrapping_add(0xa));
    let dir = c.bus.read8(anim.wrapping_add(0xe));
    c.g[A0][0] = anim as u64;
    c.g[A1][0] = mode as u64;
    ui_anim_step(c);
    let next = c.bus.read16(anim.wrapping_add(0xa)) as u32;
    c.bus.write16(anim.wrapping_add(0xa), frame);
    c.bus.write8(anim.wrapping_add(0xe), dir);
    next
}

/// Set each channel of the animation on the target to its value at frame
/// `a` blended towards frame `b` by `w` (through the target's setter, as
/// `UIAnim_Apply` does). Runs inside [`off_the_clock`].
fn set_channels(c: &mut Cpu, anim: u32, target: u32, a: u32, b: u32, w: f32) -> Flow {
    let data = c.bus.read32(anim);
    let channels = c.bus.read32(anim.wrapping_add(4));
    let n = c.bus.read32(data.wrapping_add(4));
    for i in 0..n {
        let slot = channels.wrapping_add(i * 4);
        let ch = c.bus.read32(slot);
        let param = if ch == 0 {
            0
        } else {
            c.bus.read16(ch.wrapping_add(2)) as u32
        };
        if param == PARAM_INT && w != 0.0 {
            continue;
        }
        let mut v = eval_channel(c, slot, a)?;
        if w != 0.0 {
            let to = eval_channel(c, slot, b)?;
            v = v + (to - v) * FpuF32::from_f32(w);
        }
        let vt = c.bus.read32(target.wrapping_add(8));
        let this = target.wrapping_add(c.bus.read16(vt.wrapping_add(0x60)) as i16 as u32);
        let setter = c.bus.read32(vt.wrapping_add(0x64));
        c.g[A0][0] = sx(this);
        c.g[A1][0] = param as u64;
        c.f[12] = v.0;
        call_ptr(c, setter, 0x0039_76d4)?;
    }
    Ok(())
}

/// [`uianim_eval_channel`] at `frame` through its slot (the channel `ch`
/// may be null: 0).
fn eval_channel(c: &mut Cpu, slot: u32, frame: u32) -> Result<FpuF32, Unwound> {
    c.g[A0][0] = slot as u64;
    c.g[A1][0] = frame as u64;
    callee::UI_ANIM_EVAL_CHANNEL.call(c, 0x0039_76d4)?;
    Ok(FpuF32(c.f[0]))
}

/// The original [`ui_anim_apply`]: the channel apply loop above.
fn ui_anim_apply_original(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        c.g[A3][0] = c.g[A3][0] & 0xffff;
        frame.save(c, S4, 0x30);
        c.g[A2][0] = c.g[A2][0] & 0x7;
        frame.save(c, S6, 0x10);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x80);
        c.g[S5][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0xa));
        let own_clock = c.g[A2][0] != 0;
        c.g[A1][0] = c.g[S5][0];
        let mut apply = true;
        // The middle tail runs on every path through the merge.
        let mut middle = true;
        if !own_clock {
            // On the screen's timeline: the frame is `clock - start`.
            c.cycles += 5;
            c.g[V1][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = 0xffff;
            c.g[A0][0] = c.g[V1][0] & 0xffff;
            let no_start = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
            if !no_start {
                c.cycles += 2;
                let before_start = c.g[V0][0] != 0;
                if before_start {
                    apply = false;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
                    c.g[S5][0] = c.g[V1][0] & 0xffff;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    if c.g[V0][0] == c.g[S5][0] {
                        apply = false;
                        middle = false;
                    } else {
                        c.cycles += 1;
                        c.sh((c.g[S4][0] as u32).wrapping_add(0xa), c.g[S5][0]);
                    }
                }
            }
        }
        if apply {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != c.g[S5][0] {
                c.cycles += 5;
                c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = (c.g[S5][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[S3][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        loop {
                            c.cycles += 22;
                            c.g[S2][0] = sx((c.g[S3][0] as u32) << 2);
                            c.g[A1][0] = c.g[S5][0];
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            callee::UI_ANIM_EVAL_CHANNEL.call(c, 0x0039_76a4)?;
                            c.g[S1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                            c.mov_s(20, 0);
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x60));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            callee::UI_ANIM_GET_FRAME_COUNT_SAFE.call(c, 0x0039_76c0)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            let setter = c.g[A2][0] as u32;
                            c.mov_s(12, 20);
                            call_ptr(c, setter, 0x0039_76d4)?;
                            c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                            if event_due(c) {
                                deliver_event_here(c, 0x0039_7690);
                            }
                        }
                    }
                    c.cycles += 1;
                }
                c.sh((c.g[S4][0] as u32).wrapping_add(0x8), c.g[S5][0]);
                c.cycles += 1;
            }
        }
        if middle {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_fpr(c, 20, 0x80);
        frame.restore_ra(c, 0);
        c.cycles += 9;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS;

    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    use ssx3_ee as ee;

    #[test]
    fn uianim_get_frame_count_safe_matches() {
        check(
            ssx3_ee::s13::f_00397278,
            uianim_get_frame_count_safe,
            200,
            &|c, seed| {
                let mut r = Rng(0x3972_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                if seed % 3 == 0 {
                    c.bus.write32(DATA, 0);
                } else {
                    c.bus.write32(DATA, DATA + 0x100);
                    c.bus.write16(DATA + 0x102, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn uianim_is_done_matches() {
        check(ssx3_ee::s13::f_00397440, uianim_is_done, 200, &|c, seed| {
            let mut r = Rng(0x3974_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(
                DATA + 0xa,
                if seed % 5 == 0 {
                    0x8000
                } else {
                    (r.next_u32() & 0x7fff) as u16
                },
            );
            c.bus.write16(DATA + 0x102, (r.next_u32() & 0x7fff) as u16);
        });
    }

    #[test]
    fn uianim_get_frame_count_matches() {
        check(
            ssx3_ee::s13::f_00397458,
            uianim_get_frame_count,
            200,
            &|c, seed| {
                let mut r = Rng(0x3974_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write16(
                    DATA + 0x102,
                    if seed % 5 == 0 {
                        0x8000
                    } else {
                        (r.next_u32() & 0x7fff) as u16
                    },
                );
            },
        );
    }

    #[test]
    fn uianim_set_play_mode_matches() {
        check(
            ssx3_ee::s13::f_00397468,
            uianim_set_play_mode,
            200,
            &|c, seed| {
                let mut r = Rng(0x3974_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((seed % 3) as u64) | ((r.next_u32() as u64) << 32);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write16(DATA + 0x102, r.next_u32() as u16);
            },
        );
    }

    /// The engine's size-class pool allocator (see `oc57`): short free lists
    /// behind `gp + 0x2a60..0x2a84`, enabled by the flag at `gp + 0xdd8`.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    #[test]
    fn ui_anim_set_create_anim_matches() {
        check(
            ee::s13::f_00397140__generated,
            ui_anim_set_create_anim,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x397140, 3);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 6) as u64;
                // An empty animation set.
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x1000, if seed % 3 == 0 { 0 } else { 4 });
                if seed % 3 != 0 {
                    // A single entry with the sought id and stride 0x14.
                    c.bus.write32(DATA + 0x1004 + 8, seed % 6);
                    c.bus.write32(DATA + 0x1004 + 0xc, 0x14);
                    c.bus.write32(DATA + 0x1004 + 4, 0);
                }
            },
        );
    }

    #[test]
    fn ui_anim_dtor_matches() {
        check(
            ee::s13::f_003973a8__generated,
            ui_anim_dtor,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x60) } else { DATA };
                let block = alloc(c, 0x100);
                let array = block + 0x10;
                w(c, array - 0x10, seed % 4);
                w(c, this + 4, array);
                w(c, this + 0x10, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    #[test]
    fn uianim_eval_channel_matches() {
        check(
            ee::s13::f_003971c8__generated,
            uianim_eval_channel,
            120,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3971);
                let anim = DATA + 0x100;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 40) as u64;
                c.bus.write32(DATA, if seed % 7 == 0 { 0 } else { anim });
                c.bus.write16(anim + 6, (seed % 5) as u16);
                c.bus.write16(anim + 0xa, (seed % 20) as u16);
                for i in 0..5u32 {
                    c.bus.write16(anim + 0xc + 4 * i, (1 + i) as u16);
                    c.bus
                        .write16(anim + 0xc + 4 * i + 2, (r.next_u32() & 0xff) as u16);
                }
            },
        );
    }

    #[test]
    fn ui_anim_advance_matches() {
        same(
            ssx3_ee::s13::f_003974b0__generated,
            ui_anim_advance,
            200,
            &|c, seed| {
                let mut r = Rng(0x3974b0 ^ seed as u64);
                scramble(c, &mut r);
                let anim = DATA + 0x100;
                let data = DATA + 0x200;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = ((seed % 0x10) | ((seed % 3) << 3)) as u64;
                c.bus.write32(anim, data);
                c.bus.write16(data + 2, 1 + (r.next_u32() % 8) as u16);
                let frame = r.next_u32() % 10;
                c.bus.write16(
                    anim + 0xa,
                    if (seed & 1) == 0 {
                        frame as u16
                    } else {
                        0xffff
                    },
                );
                c.bus.write8(anim + 0xe, (seed % 2) as u8);
            },
        );
    }

    /// At 240 Hz a looping animation steps one frame per four ticks (60 per
    /// second), at 60 one per tick.
    #[test]
    fn ui_anim_advance_rate() {
        for (hz, ticks) in [(60u32, 60u32), (240, 240)] {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            let app = c.bus.read32(crate::tick::APPMAN_PTR);
            let anim = DATA + 0x100;
            let data = DATA + 0x200;
            c.bus.write32(anim, data);
            c.bus.write16(data + 2, 1000);
            c.bus.write16(anim + 0xa, 0);
            c.bus.write8(anim + 0xe, 0);
            for t in 1..=ticks {
                c.bus.write32(app + crate::tick::APPMAN_VSYNC, t);
                c.g[A0][0] = anim as u64;
                // Mode 1: forward.
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0010_0000);
                ui_anim_advance(&mut c);
            }
            assert_eq!(c.bus.read16(anim + 0xa), 60, "{hz} Hz");
        }
    }

    /// A seeded `UIAnim_Apply` call (anim, channels, target); `looped`
    /// always reaches the channel loop, over three channels. Mode, start
    /// frame, clock and the last applied frame are drawn independently, so
    /// every skip path is reached (`ui_anim_apply_matches`).
    fn apply_state(c: &mut Cpu, r: &mut Rng, seed: u32, looped: bool) {
        // `UIObject_SetAnimParam`'s jump table.
        crate::difftest::elf_image(c);
        let (anim, data, chans) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
        let (target, vt) = (DATA + 0x500, DATA + 0x600);
        let mode = if looped || r.next_u32() % 2 != 0 {
            1 + r.next_u32() % 7
        } else {
            0
        };
        let n = if looped { 3 } else { r.next_u32() % 4 };
        let null = !looped && r.next_u32() % 11 == 0;
        c.bus.write32(anim, if null { 0 } else { data });
        c.bus.write32(anim + 4, chans);
        let frame = r.next_u32() % 12;
        let start = if r.next_u32() % 3 == 0 {
            0xffff
        } else {
            r.next_u32() % 8
        };
        let clock = r.next_u32() % 16;
        // Already applied: the own frame, the timeline's frame, or neither.
        let applied = match (looped, r.next_u32() % 3) {
            (true, _) => frame + 1,
            (false, 0) => frame,
            (false, 1) => clock.wrapping_sub(start),
            _ => r.next_u32(),
        };
        c.bus.write16(anim + 0x8, applied as u16);
        c.bus.write16(anim + 0xa, frame as u16);
        c.bus.write16(anim + 0xc, start as u16);
        let count = (r.next_u32() % 16) as u16;
        c.bus.write16(data + 2, if looped { 15 } else { count });
        c.bus.write32(data + 4, n);
        for i in 0..n {
            let slot = chans + 4 * i;
            let ch = DATA + 0x700 + 0x40 * i;
            c.bus
                .write32(slot, if seed % 7 == 0 && i == 0 { 0 } else { ch });
            c.bus.write16(ch + 2, ((seed + i) % 17) as u16);
            c.bus.write16(ch + 6, (r.next_u32() % 4) as u16);
            c.bus.write16(ch + 0xa, (r.next_u32() % 100) as u16);
            for k in 0..3u32 {
                c.bus
                    .write16(ch + 0xc + 4 * k, (1 + r.next_u32() % 4) as u16);
                c.bus.write16(ch + 0xe + 4 * k, r.next_u32() as u16);
            }
        }
        c.bus.write32(target + 8, vt);
        c.bus.write16(vt + 0x60, 0x40);
        c.bus.write32(vt + 0x64, 0x003a_0048);
        c.pc = 0x0039_75e0;
        c.g[A0][0] = anim as u64;
        c.g[A1][0] = target as u64;
        c.g[A2][0] = mode as u64;
        c.g[A3][0] = clock as u64;
    }

    #[test]
    fn ui_anim_apply_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        // Every path that ends before the channel loop, by its cost: own
        // clock with the frame already applied 30, past the frame count 36,
        // no channels 40; on the timeline with no start frame 35, 41 and 45,
        // before the start 34, frame unchanged 39, frame moved 44, 50 and 54.
        check_paths(
            ee::s13::f_003975e0__generated,
            ui_anim_apply,
            400,
            &[30, 34, 35, 36, 39, 40, 41, 44, 45, 50, 54],
            &|c, seed| {
                let mut r = Rng(0x3975_e000 ^ seed as u64);
                scramble(c, &mut r);
                apply_state(c, &mut r, seed, false);
            },
        );
        // The channel loop's event check, delivered and unwinding.
        check_events(
            ee::s13::f_003975e0__generated,
            ui_anim_apply,
            60,
            &|c, seed| {
                let mut r = Rng(0x3975_e0e0 ^ seed as u64);
                scramble(c, &mut r);
                let mode: Events = schedule(c, &mut r, seed, 0x0039_75e0);
                apply_state(c, &mut r, seed, true);
                mode
            },
        );
    }

    /// A 20-frame animation whose one channel (parameter 0, the target's
    /// `+0x44`) goes 0 to 100 over frames 0-10, on a target whose setter is
    /// `UIObject_SetAnimParam`. Returns (anim, target).
    fn apply_fixture(c: &mut Cpu, start: u16) -> (u32, u32) {
        let (anim, data, chans, chan) = (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x400);
        let (target, vt) = (DATA + 0x500, DATA + 0x600);
        c.bus.write32(anim, data);
        c.bus.write32(anim + 4, chans);
        c.bus.write16(anim + 0x8, 0xffff);
        c.bus
            .write16(anim + 0xa, if start == 0xffff { 0 } else { 0xffff });
        c.bus.write16(anim + 0xc, start);
        c.bus.write8(anim + 0xe, 0);
        c.bus.write16(data + 2, 20);
        c.bus.write32(data + 4, 1);
        c.bus.write32(chans, chan);
        c.bus.write16(chan + 2, 0);
        c.bus.write16(chan + 6, 2);
        c.bus.write16(chan + 0xa, 0);
        c.bus.write16(chan + 0xc, 10);
        c.bus.write16(chan + 0xe, 100);
        c.bus.write32(target + 8, vt);
        c.bus.write16(vt + 0x60, 0);
        c.bus.write32(vt + 0x64, 0x003a_0048);
        (anim, target)
    }

    fn apply(c: &mut Cpu, anim: u32, target: u32, mode: u32, clock: u32) -> f32 {
        c.pc = 0x0039_75e0;
        c.g[A0][0] = anim as u64;
        c.g[A1][0] = target as u64;
        c.g[A2][0] = mode as u64;
        c.g[A3][0] = clock as u64;
        c.g[RA][0] = sx(0x0010_0000);
        ui_anim_apply(c);
        assert_eq!(c.pc, 0x0010_0000);
        f32::from_bits(c.bus.read32(target + 0x44))
    }

    /// A widget animation with its own clock (`UIObject_UpdateAnim`: apply,
    /// then advance) moves every tick at any rate, at 60 frames per second:
    /// at 60 the original's 0, 10, 20, ...; at 120 also 15, 25, ... between.
    #[test]
    fn ui_anim_apply_moves_every_tick() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            // `UIObject_SetAnimParam`'s jump table.
            crate::difftest::elf_image(&mut c);
            crate::tick::set_for_test(&mut c, hz);
            let (anim, target) = apply_fixture(&mut c, 0xffff);
            let k = hz / 60;
            for t in 0..8 * k {
                let v = apply(&mut c, anim, target, 1, 0);
                // Frame 0 on the first tick, then frame 1 + the time since.
                let want = if t == 0 {
                    0.0
                } else {
                    10.0 * (1.0 + (t - 1) as f32 / k as f32)
                };
                assert!((v - want).abs() < 1e-3, "{hz} Hz tick {t}: {v} != {want}");
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0010_0000);
                ui_anim_advance(&mut c);
            }
        }
    }

    /// Above 60 each tick sets the channel once: the original on a step tick
    /// (the frame's values), the blend between steps; a step after a blended
    /// tick is not set again (`apply_fixture`'s one channel, a counting fake
    /// setter).
    #[test]
    fn ui_anim_apply_sets_each_channel_once_per_tick() {
        use crate::difftest::fakes::{fake_call, FAKE, LOG_COUNT, LOG_UNWIND_AT};
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, 120);
        let (anim, target) = apply_fixture(&mut c, 0xffff);
        c.bus.write32(DATA + 0x600 + 0x64, FAKE);
        c.hooks.miss = fake_call;
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        for t in 0..16u32 {
            c.bus.write32(LOG_COUNT, 0);
            apply(&mut c, anim, target, 1, 0);
            assert_eq!(c.bus.read32(LOG_COUNT), 1, "tick {t}: setter calls");
            c.g[A0][0] = anim as u64;
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x0010_0000);
            ui_anim_advance(&mut c);
        }
    }

    /// The blend is off the guest's clock: on a blended tick no event is
    /// delivered inside the apply (an event due then waits for the next
    /// guest check), and the apply costs the original's cycles alone, as at
    /// a tick with nothing to blend. The setter is a fake that delivers a
    /// due event, as the real ones check (`UIObject_SetAnimParam`'s loop).
    #[test]
    fn ui_anim_apply_blends_off_the_clock() {
        use crate::difftest::fakes::{fake_call, FAKE, LOG_UNWIND_AT};
        use crate::events::{deliver_event_here, event_due};
        thread_local! {
            /// Events delivered, and the clock the last one saw.
            static SEEN: std::cell::Cell<(u32, u64)> = const { std::cell::Cell::new((0, 0)) };
        }
        fn event(c: &mut Cpu) -> bool {
            c.next_event = u64::MAX;
            SEEN.with(|s| s.set((s.get().0 + 1, c.cycles)));
            false
        }
        fn setter_with_check(c: &mut Cpu, target: u32) {
            if event_due(c) {
                deliver_event_here(c, target);
            }
            fake_call(c, target);
        }
        let setup = |c: &mut Cpu| {
            crate::tick::set_for_test(c, 120);
            let (anim, target) = apply_fixture(c, 0xffff);
            c.bus.write32(DATA + 0x600 + 0x64, FAKE);
            c.hooks.miss = setter_with_check;
            c.hooks.event = event;
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            (anim, target)
        };
        // The original's cost at a tick with nothing to apply or blend: a
        // second apply of the same frame, with no clock on the animation.
        let mut q = crate::difftest::fresh();
        let (anim, target) = setup(&mut q);
        apply(&mut q, anim, target, 1, 0);
        let before = q.cycles;
        apply(&mut q, anim, target, 1, 0);
        let original = q.cycles - before;
        assert!(BLENDED.with(|b| !b.borrow().contains_key(&anim)));
        let mut c = crate::difftest::fresh();
        let (anim, target) = setup(&mut c);
        let mut blends = 0;
        for t in 0..16u32 {
            // An event comes due at the start of every tick.
            c.next_event = c.cycles;
            let (events, before) = (SEEN.with(|s| s.get().0), c.cycles);
            apply(&mut c, anim, target, 1, 0);
            if BLENDED.with(|b| b.borrow().contains_key(&anim)) {
                blends += 1;
                let (inside, _) = SEEN.with(|s| s.get());
                assert_eq!(inside, events, "tick {t}: event delivered in the blend");
                assert_eq!(
                    c.cycles - before,
                    original,
                    "tick {t}: blend cycles counted"
                );
                // The held event is delivered at the next guest check, at
                // the clock it would have had without the blend.
                assert!(event_due(&c), "tick {t}: held event lost");
                deliver_event_here(&mut c, 0x0010_0000);
                assert_eq!(SEEN.with(|s| s.get()), (events + 1, before + original));
            }
            c.g[A0][0] = anim as u64;
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x0010_0000);
            ui_anim_advance(&mut c);
        }
        // Frames 0 and 1 on ticks 0 and 1 (a new clock steps at once), then
        // a blend on every even tick.
        assert_eq!(blends, 7, "blended ticks");
    }

    /// A request made inside the blend (here an interrupt raised by a store
    /// in the setter: `next_event` set to the clock) is kept: right after
    /// the blend the event is due, at the clock the blend put back.
    #[test]
    fn ui_anim_apply_blend_keeps_event_requests() {
        use crate::difftest::fakes::{fake_call, FAKE, LOG_UNWIND_AT};
        fn no_event(_: &mut Cpu) -> bool {
            false
        }
        fn setter_requesting(c: &mut Cpu, target: u32) {
            c.next_event = c.cycles;
            fake_call(c, target);
        }
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, 120);
        let (anim, target) = apply_fixture(&mut c, 0xffff);
        c.bus.write32(DATA + 0x600 + 0x64, FAKE);
        c.hooks.miss = setter_requesting;
        c.hooks.event = no_event;
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        let mut blends = 0;
        for t in 0..16u32 {
            // Nothing comes due on its own in the tick.
            c.next_event = c.cycles + 1_000_000;
            apply(&mut c, anim, target, 1, 0);
            if BLENDED.with(|b| b.borrow().contains_key(&anim)) {
                blends += 1;
                assert_eq!(c.next_event, c.cycles, "tick {t}: request lost");
            }
            c.g[A0][0] = anim as u64;
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x0010_0000);
            ui_anim_advance(&mut c);
        }
        assert_eq!(blends, 7, "blended ticks");
    }

    /// At or below 60 Hz the new path never runs: a widget animation keeps
    /// the original's applied values, one frame per tick.
    #[test]
    fn ui_anim_apply_below_sixty_keeps_original() {
        for hz in [30u32, 45, 60] {
            let mut c = crate::difftest::fresh();
            crate::difftest::elf_image(&mut c);
            crate::tick::set_for_test(&mut c, hz);
            let (anim, target) = apply_fixture(&mut c, 0xffff);
            for t in 0..8u32 {
                let v = apply(&mut c, anim, target, 1, 0);
                assert!((v - 10.0 * t as f32).abs() < 1e-3, "{hz} Hz tick {t}: {v}");
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0010_0000);
                ui_anim_advance(&mut c);
            }
        }
    }

    /// A widget animation on the screen's timeline (mode 0, start frame 0)
    /// moves every tick too: the timeline thread steps once per 60 Hz frame
    /// (as `UIScreen_RunThreadFrame`), the values between follow its clock;
    /// a thread that stops leaves the widget at its last frame's value.
    #[test]
    fn ui_anim_apply_follows_the_timeline() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            // `UIObject_SetAnimParam`'s jump table.
            crate::difftest::elf_image(&mut c);
            crate::tick::set_for_test(&mut c, hz);
            let (anim, target) = apply_fixture(&mut c, 0);
            let (screen, thread, sentinel) = (DATA + 0x800, DATA + 0x900, DATA + 0xa00);
            c.bus.write32(screen + 0x1c, thread);
            c.bus.write32(thread, screen + 0x18);
            c.bus.write32(thread + 4, sentinel);
            c.bus.write32(sentinel + 4, sentinel);
            c.bus.write32(thread + 0x10, 1 << 25);
            c.bus.write16(thread + 0xc, 0);
            c.bus.write16(thread + 0xe, 0xffff);
            c.bus.write16(thread + 0x10, 6);
            c.bus.write8(thread + 0x12, 1);
            let k = hz / 60;
            for t in 0..10 * k {
                // The thread's frame: +0xe the frame it ran, +0xc the next.
                let f = c.bus.read16(thread + 0xc) as u32;
                if c.bus.read16(thread + 0xe) as u32 != f
                    && crate::tick::frame60_due(&mut c, thread, f)
                {
                    c.bus.write16(thread + 0xe, f as u16);
                    let next = if f < 6 { f + 1 } else { f };
                    c.bus.write16(thread + 0xc, next as u16);
                    crate::tick::frame60_ran(&mut c, thread, next);
                }
                let clock = c.bus.read16(thread + 0xc) as u32;
                let mut v = 0.0;
                crate::game::ui::uiscreen::with_updating_screen(&mut c, screen, |c| {
                    v = apply(c, anim, target, 0, clock);
                });
                // Frame 1 on the first tick, then the time since, up to 6.
                let want = 10.0 * (1.0 + t as f32 / k as f32).min(6.0);
                assert!((v - want).abs() < 1e-3, "{hz} Hz tick {t}: {v} != {want}");
            }
        }
    }

    /// A timeline thread stepping more than one frame at a time never
    /// blends: the widget holds each stepped frame's value, forward and in
    /// reverse, instead of sliding along the chord between authored frames.
    #[test]
    fn ui_anim_apply_timeline_guards_multi_step() {
        for (step, start_frame) in [(2i32, 0u32), (-2, 6u32)] {
            let mut c = crate::difftest::fresh();
            crate::difftest::elf_image(&mut c);
            crate::tick::set_for_test(&mut c, 120);
            let (anim, target) = apply_fixture(&mut c, 0);
            let (screen, thread, sentinel) = (DATA + 0x800, DATA + 0x900, DATA + 0xa00);
            c.bus.write32(screen + 0x1c, thread);
            c.bus.write32(thread, screen + 0x18);
            c.bus.write32(thread + 4, sentinel);
            c.bus.write32(sentinel + 4, sentinel);
            c.bus.write32(thread + 0x10, 1 << 25);
            c.bus.write16(thread + 0xc, start_frame as u16);
            c.bus.write16(thread + 0xe, 0xffff);
            c.bus.write16(thread + 0x10, 6);
            c.bus.write8(thread + 0x12, step as u8);
            for t in 0..8u32 {
                let f = c.bus.read16(thread + 0xc) as u32;
                if c.bus.read16(thread + 0xe) as u32 != f
                    && crate::tick::frame60_due(&mut c, thread, f)
                {
                    c.bus.write16(thread + 0xe, f as u16);
                    let next = (f as i32 + step).clamp(0, 6) as u16;
                    c.bus.write16(thread + 0xc, next);
                    crate::tick::frame60_ran(&mut c, thread, next as u32);
                }
                let clock = c.bus.read16(thread + 0xc) as u32;
                let mut v = 0.0;
                crate::game::ui::uiscreen::with_updating_screen(&mut c, screen, |c| {
                    v = apply(c, anim, target, 0, clock);
                });
                // The stepped frame's own value: never an in-between.
                let want = 10.0 * clock as f32;
                assert!(
                    (v - want).abs() < 1e-3,
                    "step {step} tick {t}: {v} != {want}"
                );
            }
        }
    }
}
