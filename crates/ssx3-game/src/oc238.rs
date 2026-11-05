//! Staging wave 205 (oc238): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ------------------------------------------------------------ HUD_DrawTimeElem

mod hud_time_elem {
    use super::{ee, Callee};

    /// `HudElemDef` (0x24 bytes): pos (2 x s16), size, scale (2 x f32) at
    /// +0x8, colour at +0x10, horizontal / vertical alignment (s8) at
    /// +0x20 / +0x21.
    pub const DEF_SIZE: u64 = 0x24;

    pub const DEF_ALIGN_H: u32 = 0x20;

    pub const DEF_ALIGN_V: u32 = 0x21;

    pub const GET_ELEM_POS: Callee = Callee {
        addr: 0x001e_91f8,
        f: ee::s04::f_001e91f8,
    };

    pub const GET_ELEM_SCALE: Callee = Callee {
        addr: 0x001e_9290,
        f: ee::s04::f_001e9290,
    };

    pub const GET_ELEM_COLOR: Callee = Callee {
        addr: 0x001e_91a8,
        f: ee::s04::f_001e91a8,
    };

    pub const DRAW_TIME: Callee = Callee {
        addr: 0x001f_1840,
        f: ee::s04::f_001f1840,
    };
}

/// `void HUD_DrawTimeElem(HUD* hud, int a, int b, int c, int frac (t0),
/// int flags (t1), HudElemDef* defs (t2), int idx (t3), int screen,
/// float* scaleMul, float* posOff, sColor* color)` (0x1f16c0): draws a race
/// time at HUD element `defs[idx]`: the element's position (shifted by
/// `posOff`), its scale (times `scaleMul`), the caller's colour or the
/// element's, and the element's alignment, all handed to `HUD_DrawTime`.
/// Frame 0x100 (s0-s7, fp; pos at +0x20, scale at +0x30, colour at +0x40).
pub fn hud_draw_time_elem(c: &mut Cpu) {
    use hud_time_elem::*;
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = DEF_SIZE;
        frame.save(c, S4, 0xb0);
        c.g[S4][0] = c.g[T3][0];
        frame.save(c, S3, 0xc0);
        c.mult(0, V1, S4, V0);
        c.g[S3][0] = c.g[T2][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S6, 0x90);
        let s = sp(c);
        c.sw(s.wrapping_add(0x50), c.g[A0][0]);
        c.g[S6][0] = c.g[A1][0];
        // The element: defs + idx * 0x24.
        let def = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
        c.g[S1][0] = def;
        frame.save(c, S0, 0xf0);
        frame.save(c, S2, 0xd0);
        c.g[A1][0] = sx(s.wrapping_add(0x20));
        frame.save(c, S5, 0xa0);
        c.g[A0][0] = def;
        frame.save(c, S7, 0x80);
        frame.save(c, FP, 0x70);
        c.g[S7][0] = c.g[A2][0];
        c.sw(s.wrapping_add(0x54), c.g[T0][0]);
        c.sw(s.wrapping_add(0x58), c.g[T1][0]);
        let scale_mul = c.lw(s.wrapping_add(0x108));
        c.g[S0][0] = scale_mul;
        let pos_off = c.lw(s.wrapping_add(0x110));
        c.g[S2][0] = pos_off;
        let color = c.lw(s.wrapping_add(0x118));
        c.g[S5][0] = color;
        frame.save_ra(c, 0x60);
        c.g[FP][0] = c.g[A3][0];
        GET_ELEM_POS.call(c, 0x001f_1730)?;
        c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
        c.g[A0][0] = def;
        GET_ELEM_SCALE.call(c, 0x001f_173c)?;

        // scale *= scaleMul.
        let s = sp(c);
        c.lwc1(1, s.wrapping_add(0x30));
        if scale_mul != 0 {
            c.cycles += 7;
            let m = scale_mul as u32;
            c.lwc1(3, m.wrapping_add(0x4));
            c.lwc1(2, m);
            c.lwc1(0, s.wrapping_add(0x34));
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, s.wrapping_add(0x30));
            c.swc1(0, s.wrapping_add(0x34));
        }

        // pos += posOff.
        c.cycles += 2;
        c.lwc1(1, s.wrapping_add(0x20));
        if pos_off != 0 {
            c.cycles += 7;
            let o = pos_off as u32;
            c.lwc1(3, o.wrapping_add(0x4));
            c.lwc1(2, o);
            c.lwc1(0, s.wrapping_add(0x24));
            c.add_s(1, 1, 2);
            c.add_s(0, 0, 3);
            c.swc1(1, s.wrapping_add(0x20));
            c.swc1(0, s.wrapping_add(0x24));
        }

        // The colour at sp+0x40: the element's or the caller's.
        c.cycles += 2;
        let col = sx(s.wrapping_add(0x40));
        c.g[S0][0] = col;
        if color == 0 {
            c.cycles += 5;
            c.g[A0][0] = def;
            c.g[A1][0] = col;
            GET_ELEM_COLOR.call(c, 0x001f_1798)?;
        } else {
            c.cycles += 9;
            crate::game::fe::ovstatehud::copy16_unaligned(
                c,
                A0,
                A1,
                color as u32,
                s.wrapping_add(0x40),
            );
        }
        c.g[V1][0] = DEF_SIZE;

        // HUD_DrawTime(hud, a, b, c, frac, flags, pos, scale, &col,
        // def.alignH, def.alignV, screen).
        c.cycles += 30;
        let s = sp(c);
        c.sw(s, col);
        c.mult(0, V0, S4, V1);
        c.g[A0][0] = c.lw(s.wrapping_add(0x50));
        c.g[T0][0] = c.lw(s.wrapping_add(0x54));
        c.g[A1][0] = c.g[S6][0];
        c.g[T1][0] = c.lw(s.wrapping_add(0x58));
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = c.g[FP][0];
        c.g[T3][0] = sx(s.wrapping_add(0x30));
        let d = (c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32);
        c.g[V1][0] = sx(d);
        c.g[T2][0] = sx(s.wrapping_add(0x20));
        let align_h = c.lb(d.wrapping_add(DEF_ALIGN_H));
        c.sw(s.wrapping_add(0x8), align_h);
        let screen = c.lw(s.wrapping_add(0x100));
        c.g[V0][0] = screen;
        let align_v = c.lb(d.wrapping_add(DEF_ALIGN_V));
        c.g[T4][0] = align_v;
        c.sw(s.wrapping_add(0x18), screen);
        c.sw(s.wrapping_add(0x10), align_v);
        DRAW_TIME.call(c, 0x001f_180c)?;

        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- BECharInt_GetOverallRating

/// The seven attribute getters `BECharInt_GetOverallRating` averages, in
/// call order, with their return addresses.
const OVERALL_RATING_ATTRIBS: [(Callee, u32); 7] = [
    // BECharInt_GetAttrib1
    (Callee { addr: 0x0014_84e0, f: ee::s01::f_001484e0 }, 0x0014_897c),
    // BECharInt_GetAttrib3
    (Callee { addr: 0x0014_8680, f: ee::s01::f_00148680 }, 0x0014_899c),
    // BECharInt_GetAttrib0
    (Callee { addr: 0x0014_8410, f: ee::s01::f_00148410 }, 0x0014_89c0),
    // BECharInt_GetAttrib4
    (Callee { addr: 0x0014_8750, f: ee::s01::f_00148750 }, 0x0014_89e4),
    // BECharInt_GetAttrib5
    (Callee { addr: 0x0014_8820, f: ee::s01::f_00148820 }, 0x0014_8a08),
    // BECharInt_GetAttrib6
    (Callee { addr: 0x0014_88f0, f: ee::s01::f_001488f0 }, 0x0014_8a2c),
    // BECharInt_GetAttrib2
    (Callee { addr: 0x0014_85b0, f: ee::s01::f_001485b0 }, 0x0014_8a50),
];

/// `int BECharInt_GetOverallRating(BECharInt* self, int rider)` (0x148950):
/// a rider's overall rating, the mean of its seven attribute ratings each
/// divided by 5 (integer divisions: `sum(attr / 5) / 7`), at least 1.
/// Frame 0x60 (s0-s4).
pub fn be_char_int_get_overall_rating(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        let this = c.g[A0][0];
        let rider = c.g[A1][0];
        c.g[S2][0] = this;
        c.g[S3][0] = rider;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 5;
        let (getter, ret) = OVERALL_RATING_ATTRIBS[0];
        getter.call(c, ret)?;
        c.g[S4][0] = 0;
        // attr / 5 (the divisor check `teq s0, s4` never traps).
        c.div(0, V0, S0);

        c.cycles += 10;
        c.g[A0][0] = this;
        c.g[S1][0] = c.lo[0];
        c.g[A1][0] = rider;
        let (getter, ret) = OVERALL_RATING_ATTRIBS[1];
        getter.call(c, ret)?;
        c.div(0, V0, S0);
        c.g[V1][0] = 5;
        c.g[A0][0] = this;
        c.g[A1][0] = rider;

        for &(getter, ret) in &OVERALL_RATING_ATTRIBS[2..] {
            c.cycles += 9;
            c.g[V0][0] = c.lo[0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            getter.call(c, ret)?;
            c.div(0, V0, S0);
            c.g[V1][0] = 5;
            c.g[A0][0] = this;
            c.g[A1][0] = rider;
        }
        // The last block loads 7 (the mean's divisor), 1 (the minimum).
        c.g[V1][0] = 7;
        c.g[A1][0] = 1;
        c.g[A0][0] = 5;

        c.cycles += 15;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.lo[0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.div(0, S1, V1);
        let mean = c.lo[0];
        c.g[S1][0] = mean;
        c.g[V0][0] = if 0 < mean as i64 { mean } else { 1 };
        c.g[S4][0] = (0 < mean as i64) as u64;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S4, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------- AIAnimEventMap_PickAnim

mod pick_anim {
    use super::{ee, Callee};

    /// The event-map entries (`D_00449960`, 12 bytes each): weight, flags,
    /// animation value.
    pub const ENTRIES_HI: u64 = 0x0045_0000;

    pub const ENTRIES: u32 = 0x0044_9960;

    pub const ENTRY_SIZE: u64 = 0xc;

    pub const ENTRY_FLAGS: u32 = 0x4;

    pub const RAND_NEXT_GAME: Callee = Callee {
        addr: 0x0031_7810,
        f: ee::s10::f_00317810,
    };
}

/// `int AIAnimEventMap_PickAnim(Ranges* t, int idx, uint mask)` (0x311710):
/// picks an animation for an AI event. Range `t[idx]` (s16 start, s16
/// count) selects event-map entries; a single entry is returned directly,
/// else a weighted random choice among the entries whose flags contain all
/// of `mask` (`AIrand() % total weight`, walking down the weights); the
/// range's last entry if none is reached. Frame 0x60 (s0-s4).
pub fn ai_anim_event_map_pick_anim(c: &mut Cpu) {
    use pick_anim::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x50);
        let range = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
        c.g[V0][0] = sx(range);
        frame.save(c, S1, 0x40);
        c.g[A0][0] = sx(range);
        frame.save(c, S2, 0x30);
        let mask = c.g[A2][0];
        c.g[S0][0] = mask;
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let count = c.lh(range.wrapping_add(0x2));
        c.g[V1][0] = count;
        c.g[V0][0] = 1;
        let start = c.lh(range);
        c.g[S2][0] = start;

        // The chosen entry's address ends up in v1.
        'pick: {
            if count == 1 {
                c.cycles += 6;
                c.g[V1][0] = ENTRY_SIZE;
                c.g[V0][0] = sx(ENTRIES);
                c.mult(0, V1, S2, V1);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(ENTRIES));
                break 'pick;
            }

            // Total weight of the matching entries.
            c.cycles += 5;
            let end = sx((start as u32).wrapping_add(count as u32));
            c.g[S3][0] = end;
            c.g[S1][0] = 0;
            c.g[V0][0] = ((start as i64) < (end as i64)) as u64;
            c.g[S4][0] = ENTRIES_HI;
            if (start as i64) < (end as i64) {
                c.cycles += 6;
                c.g[V0][0] = ENTRY_SIZE;
                c.g[V1][0] = sx(ENTRIES);
                c.mult(0, V0, S2, V0);
                c.g[A0][0] = sx((end as u32).wrapping_sub(start as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(ENTRIES));
                loop {
                    c.cycles += 4;
                    let e = c.g[V1][0] as u32;
                    c.g[V0][0] = c.lw(e.wrapping_add(0x4)) & mask;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    if c.g[V0][0] == mask {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw(e);
                        c.g[S1][0] =
                            sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    c.cycles += 2;
                    c.g[V1][0] = sx(e.wrapping_add(0xc));
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_1790);
                    }
                }
            }

            // r = AIrand() % total (unsigned; the zero check never traps).
            c.cycles += 6;
            RAND_NEXT_GAME.call(c, 0x0031_17b8)?;
            c.divu(0, V0, S1);
            c.g[A1][0] = start;

            c.cycles += 4;
            c.g[V0][0] = ((start as i64) < (end as i64)) as u64;
            c.g[V1][0] = c.hi[0];
            c.g[A2][0] = c.hi[0];
            if (start as i64) < (end as i64) {
                c.cycles += 8;
                c.g[V0][0] = ENTRY_SIZE;
                c.g[V1][0] = ENTRIES_HI;
                c.mult(0, V0, A1, V0);
                c.g[A3][0] = sx(ENTRIES);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(ENTRIES));
                loop {
                    c.cycles += 4;
                    let w = c.g[A0][0] as u32;
                    c.g[V0][0] = c.lw(w.wrapping_add(ENTRY_FLAGS)) & mask;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    if c.g[V0][0] == mask {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(w);
                        c.g[A2][0] =
                            sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        if (c.g[A2][0] as i64) <= 0 {
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(ENTRIES));
                            break 'pick;
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = sx(w.wrapping_add(0xc));
                    let more = (c.g[A1][0] as i64) < (end as i64);
                    c.g[V0][0] = more as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_17f8);
                    }
                }
            }

            // None reached: the range's last entry.
            c.cycles += 5;
            c.g[V0][0] = ENTRY_SIZE;
            c.g[V1][0] = sx((end as u32).wrapping_sub(1));
            c.mult(0, V1, V1, V0);
            c.g[A0][0] = sx(ENTRIES);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(ENTRIES));
        }

        c.cycles += 9;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------------------- MEM_PoolAlloc

mod pool_alloc {
    use super::{ee, Callee};

    /// `gp+0xdd8`: the size-class pools are set up.
    pub const GP_POOLS_READY: u32 = 0xdd8;

    const fn pool(addr: u32, f: super::GuestFn) -> Callee {
        Callee { addr, f }
    }

    /// The 8-byte class (`MEM_Pool8Alloc(pool)`, gp slot, return address).
    pub const POOL8: (u32, Callee, u32) =
        (0x2a70, pool(0x0032_02c0, ee::s10::f_003202c0), 0x0031_9e88);

    /// The larger classes in order: (upper size limit, gp slot of the
    /// pool, `MEM_PoolNAlloc(pool, align)`, return address).
    pub const CLASSES: [(u64, u32, Callee, u32); 6] = [
        (0x10, 0x2a60, pool(0x0032_02f8, ee::s10::f_003202f8), 0x0031_9ea4),
        (0x20, 0x2a6c, pool(0x0032_0330, ee::s10::f_00320330), 0x0031_9ec0),
        (0x40, 0x2a7c, pool(0x0032_0368, ee::s10::f_00320368), 0x0031_9edc),
        (0x80, 0x2a84, pool(0x0032_03b8, ee::s10::f_003203b8), 0x0031_9ef8),
        (0x100, 0x2a78, pool(0x0032_0408, ee::s10::f_00320408), 0x0031_9f14),
        (0x200, 0x2a68, pool(0x0032_0448, ee::s10::f_00320448), 0x0031_9f30),
    ];

    /// `MEM_AllocInternal(size, align, 0, 0)`.
    pub const ALLOC_INTERNAL: Callee = pool(0x0031_9a90, ee::s10::f_00319a90);
}

/// `void* MEM_PoolAlloc(uint size, uint align)` (0x319e48): allocates a
/// small block. `size = max(size, align)`; with the pools ready it is
/// rounded up to 4 and served by the smallest size-class pool (8 ... 512
/// bytes) that fits; anything larger, or every request before the pools
/// exist (as size 0x201), goes to `MEM_AllocInternal(size, max(align, 8),
/// 0, 0)`. Frame 0x10 (ra only).
pub fn mem_pool_alloc(c: &mut Cpu) {
    use pool_alloc::*;
    guest(c, |c| {
        c.cycles += 7;
        let align = c.g[A1][0];
        c.g[A2][0] = align;
        let frame = Frame::open(c, 0x10);
        let bigger = align < c.g[A0][0];
        c.g[V0][0] = bigger as u64;
        let gp = c.g[GP][0] as u32;
        let ready = c.lw(gp.wrapping_add(GP_POOLS_READY));
        c.g[V1][0] = ready;
        frame.save_ra(c, 0);
        if !bigger {
            c.g[A0][0] = align;
        }

        'internal: {
            if ready == 0 {
                c.cycles += 2;
                c.g[A0][0] = 0x201;
                c.g[V0][0] = 7;
                break 'internal;
            }
            // Round up to a multiple of 4.
            c.cycles += 7;
            let pad = (0u32.wrapping_sub((c.g[A0][0] & 3) as u32) & 3) as u64;
            c.g[V0][0] = pad;
            let size = sx((c.g[A0][0] as u32).wrapping_add(pad as u32));
            c.g[A0][0] = size;
            c.g[V1][0] = (size < 9) as u64;
            c.g[V0][0] = (size < 0x11) as u64;
            if size < 9 {
                c.cycles += 4;
                let (slot, alloc, ret) = POOL8;
                c.g[A0][0] = c.lw(gp.wrapping_add(slot));
                alloc.call(c, ret)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            for (i, &(limit, slot, alloc, ret)) in CLASSES.iter().enumerate() {
                c.cycles += 2;
                c.g[V0][0] = match CLASSES.get(i + 1) {
                    Some(&(next, ..)) => (size < next + 1) as u64,
                    None => 7,
                };
                if size < limit + 1 {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(gp.wrapping_add(slot));
                    c.g[A1][0] = align;
                    alloc.call(c, ret)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
            }
        }

        // MEM_AllocInternal(size, max(align, 8), 0, 0); v0 holds 7.
        c.cycles += 7;
        c.g[A1][0] = 8;
        let wider = c.g[V0][0] < align;
        c.g[V0][0] = wider as u64;
        c.g[A3][0] = 0;
        if wider {
            c.g[A1][0] = align;
        }
        c.g[A2][0] = 0;
        ALLOC_INTERNAL.call(c, 0x0031_9f58)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------------- SNDSTRM_IsPrebuffering

mod prebuffering {
    /// Entry, the "prebuffer done" resume point (an event check branches
    /// there), and the end of the function.
    pub const ENTRY: u32 = 0x003b_70f8;

    pub const RESET: u32 = 0x003b_7184;

    pub const END: u32 = 0x003b_71d4;

    /// `SNDSTRM`: the stream file handle at +0, the queued-request count at
    /// +0x130, the prebuffer state at +0x134.
    pub const STREAM_FILE: u32 = 0x0;

    pub const QUEUED: u32 = 0x130;

    pub const PREBUFFER: u32 = 0x134;

    /// Prebuffer state: data rate (bytes per second) at +0x10, the target
    /// in milliseconds at +0x20 (negative: wait forever, 0: done).
    pub const RATE: u32 = 0x10;

    pub const TARGET_MS: u32 = 0x20;

    /// `StreamFile_GetChannelBytes(file)`, `StreamFile_GetReadState(file)`
    /// (other shard, through the table); read state 2 = at end of file.
    pub const GET_CHANNEL_BYTES: u32 = 0x003e_14c8;

    pub const GET_READ_STATE: u32 = 0x003e_14f8;

    pub const READ_AT_END: u64 = 2;

    /// The buffered byte count is capped at 4,000,000 before `* 1000`.
    pub const BYTES_CAP: u64 = 0x003d_0900;
}

/// Where `SNDSTRM_IsPrebuffering` goes after its checks.
enum PrebufferNext {
    /// The epilogue (v0 holds the result).
    Done,
    /// Prebuffering finished: clear the target, return 0.
    Reset,
    /// An event at the branch to `RESET` asked the thread to unwind.
    Unwind,
}

/// The checks of `SNDSTRM_IsPrebuffering` from its entry (prologue
/// included) up to the epilogue or the reset.
fn sndstrm_prebuffer_checks(c: &mut Cpu) -> Result<PrebufferNext, crate::call::Unwound> {
    use prebuffering::*;
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    let strm = c.g[A0][0];
    c.g[S1][0] = strm;
    frame.save(c, S0, 0x20);
    let strm = strm as u32;
    let pre = c.lw(strm.wrapping_add(PREBUFFER));
    c.g[S0][0] = pre;
    c.g[V0][0] = 0;
    if pre == 0 {
        return Ok(PrebufferNext::Done);
    }
    let pre = pre as u32;
    c.cycles += 3;
    let rate = c.lw(pre.wrapping_add(RATE));
    c.g[V0][0] = 0;
    if rate == 0 {
        return Ok(PrebufferNext::Done);
    }
    c.cycles += 3;
    let target = c.lw(pre.wrapping_add(TARGET_MS));
    c.g[V0][0] = 1;
    if (target as i64) < 0 {
        return Ok(PrebufferNext::Done);
    }
    c.cycles += 3;
    let target = c.lw(pre.wrapping_add(TARGET_MS));
    c.g[V0][0] = 0;
    if target == 0 {
        return Ok(PrebufferNext::Done);
    }

    // Buffered milliseconds: min(bytes, 4e6) * 1000 / rate.
    c.cycles += 10;
    c.g[A0][0] = c.lw(strm.wrapping_add(STREAM_FILE));
    call_ptr(c, GET_CHANNEL_BYTES, 0x003b_7144)?;
    c.g[A2][0] = 1000;
    let rate = c.lw(pre.wrapping_add(RATE));
    c.g[A1][0] = rate;
    c.g[V1][0] = BYTES_CAP;
    let over = BYTES_CAP < c.g[V0][0];
    c.g[A0][0] = over as u64;
    c.cycles += 8;
    if !over {
        c.g[V1][0] = c.g[V0][0];
    }
    c.mult(0, V1, V1, A2);
    c.g[V0][0] = c.lw(pre.wrapping_add(TARGET_MS));
    // (The divisor check `teq a1, zero` never traps: rate is non-zero.)
    c.divu(0, V1, A1);
    let short = c.lo[0] < c.g[V0][0];
    c.g[V1][0] = short as u64;
    if !short {
        return Ok(PrebufferNext::Reset);
    }

    // Not enough yet: done anyway when the file has been read to its end.
    c.cycles += 5;
    c.g[A0][0] = c.lw(strm.wrapping_add(STREAM_FILE));
    call_ptr(c, GET_READ_STATE, 0x003b_7198)?;
    c.g[V1][0] = READ_AT_END;
    if c.g[V0][0] == READ_AT_END {
        return Ok(if back_edge_unwinds(c, RESET) {
            PrebufferNext::Unwind
        } else {
            PrebufferNext::Reset
        });
    }
    // Or when requests are queued (> 0) still prebuffering; else done when
    // the file is idle (read state 0).
    c.cycles += 3;
    let queued = c.lw(strm.wrapping_add(QUEUED));
    c.g[V0][0] = 1;
    if (queued as i64) > 0 {
        return Ok(PrebufferNext::Done);
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(strm.wrapping_add(STREAM_FILE));
    call_ptr(c, GET_READ_STATE, 0x003b_71b8)?;
    let idle = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if idle {
        return Ok(if back_edge_unwinds(c, RESET) {
            PrebufferNext::Unwind
        } else {
            PrebufferNext::Reset
        });
    }
    Ok(PrebufferNext::Done)
}

/// `int SNDSTRM_IsPrebuffering(SNDSTRM* s)` (0x3b70f8): whether a sound
/// stream is still prebuffering. No prebuffer state, a zero data rate or a
/// zero target: no (0); a negative target: yes (1). Else it compares the
/// buffered time (`min(bytes, 4e6) * 1000 / rate` ms) with the target:
/// reached, or the file read to its end, or (with no queued requests) the
/// file idle, clears the target and answers 0; otherwise 1. Resumable at
/// 0x3b7184 (the event check on the way to clearing the target). Frame 0x30
/// (s0, s1).
pub fn sndstrm_is_prebuffering(c: &mut Cpu) {
    use prebuffering::*;
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let next = match c.pc {
            ENTRY => sndstrm_prebuffer_checks(c)?,
            RESET => PrebufferNext::Reset,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        match next {
            PrebufferNext::Unwind => return Ok(()),
            PrebufferNext::Reset => {
                c.cycles += 3;
                let pre = c.g[S0][0] as u32;
                c.sw(pre.wrapping_add(TARGET_MS), 0);
                c.g[V0][0] = 0;
            }
            PrebufferNext::Done => {}
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------------------- __fixunssfsi

mod fixunssfsi {
    use super::{ee, Callee};

    pub const ENTRY: u32 = 0x0041_39f8;

    /// The "answer 0" block the negative-exponent branch (event-checked)
    /// goes to: a resume point.
    pub const ZERO: u32 = 0x0041_3a28;

    pub const END: u32 = 0x0041_3a90;

    /// `__unpack_f(const float* in, fp_number_type* out)`.
    pub const UNPACK_F: Callee = Callee {
        addr: 0x0041_3068,
        f: ee::s15::f_00413068,
    };

    /// `fp_number_type` at sp+0: class, sign, normal exponent, fraction
    /// (`FRACBITS` 30, the implicit one at bit 30).
    pub const CLASS: u32 = 0x0;

    pub const SIGN: u32 = 0x4;

    pub const EXP: u32 = 0x8;

    pub const FRAC: u32 = 0xc;

    /// Classes: 0 sNaN, 1 qNaN, 2 zero, 4 infinity (3 normal).
    pub const CLASS_ZERO: u64 = 2;

    pub const CLASS_INFINITY: u64 = 4;
}

/// `unsigned __fixunssfsi(float f)` (libgcc soft-float, 0x4139f8): float to
/// unsigned int. NaN, zero, negative and below-one values give 0, infinity
/// and values of 2^32 and above give 0xffffffff, else the fraction shifted
/// by the exponent. Resumable at 0x413a28 (the "answer 0" block, reached
/// through an event check). Frame 0x30 (the unpacked number at +0, `f` at
/// +0x10).
pub fn fixunssfsi(c: &mut Cpu) {
    use fixunssfsi::*;
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        'zero: {
            match c.pc {
                ENTRY => {}
                ZERO => break 'zero,
                pc => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
            c.cycles += 10;
            let frame = Frame::open(c, 0x30);
            frame.save_ra(c, 0x20);
            let s = sp(c);
            c.g[A0][0] = sx(s.wrapping_add(0x10));
            c.swc1(12, s.wrapping_add(0x10));
            c.g[A1][0] = sx(s);
            UNPACK_F.call(c, 0x0041_3a10)?;
            let s = sp(c);
            let class = c.lw(s.wrapping_add(CLASS));
            c.g[V1][0] = class;
            c.g[V0][0] = (class < 2) as u64;
            if class == CLASS_ZERO {
                break 'zero;
            }
            // NaN: 0.
            c.cycles += 2;
            let sign = c.lw(s.wrapping_add(SIGN));
            c.g[V0][0] = sign;
            if class < 2 {
                break 'zero;
            }
            // Negative: 0.
            c.cycles += 2;
            c.g[V0][0] = 0;
            if sign != 0 {
                fixunssfsi_exit(c, frame);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = class ^ CLASS_INFINITY;
            let exp = c.lw(s.wrapping_add(EXP));
            c.g[A0][0] = exp;
            let exp = exp as i64;
            let saturate = 'sat: {
                if class == CLASS_INFINITY {
                    break 'sat true;
                }
                // Below one: 0 (an event check on the way).
                c.cycles += 2;
                c.g[V0][0] = (exp < 32) as u64;
                if exp < 0 {
                    if back_edge_unwinds(c, ZERO) {
                        return Ok(());
                    }
                    break 'zero;
                }
                c.cycles += 2;
                exp >= 32
            };
            if saturate {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                fixunssfsi_exit(c, frame);
                return Ok(());
            }
            c.g[V0][0] = (exp < 31) as u64;
            c.cycles += 2;
            if exp < 31 {
                // fraction >> (30 - exp).
                c.g[V0][0] = 0x1e;
                c.cycles += 3;
                let frac = c.lw(s.wrapping_add(FRAC));
                c.g[V1][0] = frac;
                let shift = 30u32.wrapping_sub(exp as u32);
                c.g[V0][0] = sx(shift);
                c.g[V0][0] = sx((frac as u32) >> (shift & 31));
            } else {
                // exp == 31: fraction << 1.
                c.cycles += 4;
                let frac = c.lw(s.wrapping_add(FRAC));
                c.g[V1][0] = frac;
                let shift = (exp as u32).wrapping_sub(0x1e);
                c.g[V0][0] = sx((frac as u32) << (shift & 31));
            }
            fixunssfsi_exit(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        fixunssfsi_exit(c, frame);
        Ok(())
    })
}

/// The epilogue of `__fixunssfsi`.
fn fixunssfsi_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

// -------------------------------------------------- WScriptFn_GetObjectSlot32

mod get_object_slot32 {
    /// `gp+0x25fc`: the default arguments are initialised.
    pub const GP_DEFAULTS_READY: u32 = 0x25fc;

    /// The default arguments (12 bytes at 0x4fbc00 = 0x500000 - 0x4400):
    /// object id -1, time -1.0, a byte at +8.
    pub const DEFAULTS_HI: u64 = 0x0050_0000;

    pub const DEFAULTS: u32 = 0x004f_bc00;

    /// `gp+0xba8`: the declared argument types (2 = float).
    pub const GP_ARG_TYPES: u32 = 0xba8;

    pub const TYPE_FLOAT: u64 = 2;

    /// `gp+0x16c8`: the script object manager (`**mgr + 8`: the per-class
    /// tables, `+0x1c` of a class: the handle table); `gp+0xce8`: the
    /// world (`+0x290`: the player object, for id -1).
    pub const GP_OBJECT_MAN: u32 = 0x16c8;

    pub const GP_WORLD: u32 = 0xce8;

    pub const WORLD_PLAYER: u32 = 0x290;

    /// `gp-0x3368`: 1/30 (frames to seconds).
    pub const GP_FRAME_TIME: u32 = 0x3368;

    /// `EA_MemCopy(dst, src, n)` and `LunoVMRegister_AsInt(reg)` (through
    /// the table).
    pub const MEM_COPY: u32 = 0x003e_6574;

    pub const REGISTER_AS_INT: u32 = 0x0022_6610;
}

/// `LunoVMRegister* WScriptFn_GetObjectSlot32(LunoVMRegister* ret, int n,
/// Arg* args)` (0x3019c8): script function. The arguments (object id,
/// time) start from the defaults (-1, -1.0) and take each passed argument
/// `{idx, val, _, type}` (an int passed for a float slot is converted). The
/// object is the player for id -1, else looked up by class (low byte) and
/// handle (the rest). Answers, as an int register, the object's
/// controller's virtual `+0x100` (slot 32) called with `time / 30`, or 0
/// without a controller or object. Frame 0x70 (s0-s3; the arguments at
/// +0x10).
pub fn wscript_fn_get_object_slot32(c: &mut Cpu) {
    use get_object_slot32::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let ready = c.lw(gp.wrapping_add(GP_DEFAULTS_READY));
        c.g[V0][0] = ready;
        frame.save(c, S3, 0x30);
        frame.save(c, S1, 0x50);
        let args = c.g[A2][0];
        c.g[S3][0] = args;
        frame.save(c, S2, 0x40);
        let ret = c.g[A0][0];
        c.g[S1][0] = ret;
        frame.save(c, S0, 0x60);
        let n = c.g[A1][0];
        c.g[S2][0] = n;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = DEFAULTS_HI;
        if ready == 0 {
            // First use: defaults = { -1, -1.0f, (byte kept) }.
            c.cycles += 12;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let b = c.lbu(DEFAULTS.wrapping_add(0x8));
            c.g[A1][0] = b;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            c.sw(DEFAULTS, u64::MAX);
            c.sb(DEFAULTS.wrapping_add(0x8), b);
            c.swc1(0, DEFAULTS.wrapping_add(0x4));
            c.sw(gp.wrapping_add(GP_DEFAULTS_READY), 1);
            c.sb(sp(c), b);
        }

        // buf (sp+0x10) = defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S0][0] = sx(buf);
        c.g[A1][0] = sx(DEFAULTS);
        c.g[A2][0] = 0xc;
        c.sw(buf, u64::MAX);
        c.g[A0][0] = sx(buf);
        call_ptr(c, MEM_COPY, 0x0030_1a48)?;
        c.g[A2][0] = args;
        if (n as i64) > 0 {
            // Each passed argument into its slot.
            c.cycles += 4;
            c.g[A1][0] = n;
            c.g[T0][0] = sx(gp.wrapping_add(GP_ARG_TYPES));
            c.g[A3][0] = TYPE_FLOAT;
            loop {
                c.cycles += 8;
                let arg = c.g[A2][0] as u32;
                let idx = c.lw(arg);
                let ty = c.lw(arg.wrapping_add(0xc));
                c.g[V1][0] = ty;
                let off = (idx as u32) << 2;
                let slot = buf.wrapping_add(off);
                c.g[A0][0] = sx(slot);
                let declared = c.lw((c.g[T0][0] as u32).wrapping_add(off));
                c.g[V0][0] = declared;
                if ty != declared && declared == TYPE_FLOAT {
                    c.cycles += 6;
                    c.lwc1(0, arg.wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, slot);
                } else {
                    if ty != declared {
                        c.cycles += 2;
                    }
                    c.cycles += 1;
                    let val = c.lw(arg.wrapping_add(0x4));
                    c.g[V0][0] = val;
                    c.sw(slot, val);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx(arg.wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1a60);
                }
            }
        }

        // The object: the player for id -1, else class table[id & 0xff],
        // handle table[id >> 8] >> 8, times 4.
        c.cycles += 4;
        let buf = c.g[S0][0] as u32;
        let id = c.lw(buf);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let man = c.lw(gp.wrapping_add(GP_OBJECT_MAN));
        c.g[A1][0] = man;
        let obj = if id == u64::MAX {
            c.cycles += 3;
            let world = c.lw(gp.wrapping_add(GP_WORLD));
            c.g[V0][0] = world;
            c.lw((world as u32).wrapping_add(WORLD_PLAYER))
        } else {
            c.cycles += 8;
            let class_off = ((id & 0xff) as u32) << 2;
            c.g[V0][0] = sx(class_off);
            let m = c.lw(man as u32);
            c.g[V1][0] = m;
            let tables = c.lw((m as u32).wrapping_add(0x8));
            c.g[A0][0] = tables;
            let entry = class_off.wrapping_add(tables as u32);
            c.g[V0][0] = sx(entry);
            let class = c.lw(entry);
            c.g[V1][0] = class;
            c.g[V0][0] = sx((id as u32) >> 8);
            let mut obj = 0;
            if class != 0 {
                c.cycles += 7;
                let handles = c.lw((class as u32).wrapping_add(0x1c));
                c.g[A0][0] = handles;
                let h = ((id as u32) >> 8) << 2;
                c.g[V0][0] = sx(h.wrapping_add(handles as u32));
                let word = c.lw(h.wrapping_add(handles as u32));
                c.g[V1][0] = word;
                let index = (word as u32) >> 8;
                obj = sx(index << 2);
                if index == 0 {
                    c.g[V0][0] = obj;
                }
            }
            if obj == 0 {
                c.cycles += 1;
            }
            obj
        };
        c.g[V0][0] = obj;

        c.cycles += 2;
        if obj == 0 {
            // No object: 0.
            c.g[V0][0] = 1;
            c.cycles += 4;
            c.g[A0][0] = ret;
            c.sw((ret as u32).wrapping_add(0x8), 1);
            call_ptr(c, REGISTER_AS_INT, 0x0030_1b64)?;
            c.sw(c.g[V0][0] as u32, 0);
        } else {
            c.cycles += 3;
            let ctrl = c.lw((obj as u32).wrapping_add(0xc));
            c.g[A1][0] = ctrl;
            c.lwc1(0, buf.wrapping_add(0x4));
            let value = if ctrl != 0 {
                // ctrl->vslot32(time / 30).
                c.cycles += 9;
                let vt = c.lw((ctrl as u32).wrapping_add(0xc));
                c.g[V1][0] = vt;
                c.lwc1(12, gp.wrapping_sub(GP_FRAME_TIME));
                let slot = crate::call::VSlot::read(c, vt, 0x100);
                c.g[A0][0] = slot.delta;
                c.mul_s(12, 0, 12);
                c.g[V0][0] = slot.func;
                c.g[A0][0] = slot.this(ctrl);
                call_ptr(c, slot.func as u32, 0x0030_1b34)?;
                c.g[V0][0]
            } else {
                c.cycles += 1;
                0
            };
            c.g[S0][0] = value;
            c.cycles += 6;
            c.g[V0][0] = 1;
            c.g[A0][0] = ret;
            c.sw((ret as u32).wrapping_add(0x8), 1);
            call_ptr(c, REGISTER_AS_INT, 0x0030_1b50)?;
            c.sw(c.g[V0][0] as u32, value);
        }

        c.cycles += 8;
        c.g[V0][0] = ret;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------------- LensFX_DrawFlareSet

mod flare_set {
    use super::{ee, Callee};

    /// `gp-0x854`: the renderer (`+0x10d8` vtable: width at +0x50, height
    /// at +0x58, DrawSprite at +0x270; `+0xe84` the draw state, whose
    /// halfword at +0x10 takes the word at `+0x1004`).
    pub const GP_RENDERER: u32 = 0x854;

    pub const VTABLE: u32 = 0x10d8;

    pub const RENDER_STATE: u32 = 0xe84;

    pub const BLEND: u32 = 0x1004;

    /// `LensFX`: sun position at +0x1f0/+0x1f4, the flare count at +0x200,
    /// the intensity at +0x204; flares (0x1c each) from +0x30: type,
    /// distance along the sun ray (+0x34), size (+0x38), colour (+0x3c).
    pub const SUN_X: u32 = 0x1f0;

    pub const SUN_Y: u32 = 0x1f4;

    pub const COUNT: u32 = 0x200;

    pub const INTENSITY: u32 = 0x204;

    pub const FLARE_TYPE: u32 = 0x30;

    pub const FLARE_DIST: u32 = 0x34;

    pub const FLARE_SIZE: u32 = 0x38;

    pub const FLARE_COLOUR: u32 = 0x3c;

    pub const FLARE_STRIDE: u32 = 0x1c;

    /// `WorldPainter_GetSunFlareIntensity(6)`.
    pub const SUN_FLARE_INTENSITY: Callee = Callee {
        addr: 0x002e_e268,
        f: ee::s09::f_002ee268,
    };
}

/// `(float)(unsigned)v` the way gcc converts an unsigned int: directly when
/// it is non-negative, else halved (keeping the low bit) and doubled.
/// Leaves `f1` = the result, and v0/v1 as the compiled code does (`v1` is
/// set by the caller where the original sets it in a delay slot).
fn flare_u32_to_f1(c: &mut Cpu, extra: u64) {
    let v = c.g[V0][0];
    if (v as i64) < 0 {
        c.cycles += 2 + extra;
        let half = sx((v as u32) >> 1);
        c.g[V1][0] = half;
        c.g[V0][0] = (v & 1) | half;
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
    } else {
        c.cycles += 4;
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
    }
}

/// An 8-byte struct copy through GPR `reg` (`ldl/ldr` + `sdl/sdr`).
fn flare_copy8(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.ldl(reg, src.wrapping_add(0x7));
    c.ldr(reg, src);
    c.sdl(reg, dst.wrapping_add(0x7));
    c.sdr(reg, dst);
}

/// `void LensFX_DrawFlareSet(LensFX* fx)` (0x2f4690): draws the lens
/// flares. The screen centre is half the renderer's width / height; each
/// flare sits at `centre + (sun - centre) * dist`, with its colour's red
/// scaled by the intensity times `WorldPainter_GetSunFlareIntensity(6)`,
/// the quarter of the flare texture its type selects (0..3, else the left
/// half), and a size of `size * 300`, drawn through the renderer's
/// DrawSprite. Frame 0xf0 (s0-s3, f20/f21; centre at +0, sun at +0x10, ray
/// at +0x20, position at +0x30, uv0 at +0x40, colour at +0x50, uv1 at
/// +0x60, scratch / size at +0x70, the sprite position at +0x80).
pub fn lens_fx_draw_flare_set(c: &mut Cpu) {
    use flare_set::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xf0);
        let gp = c.g[GP][0] as u32;
        let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
        c.g[A1][0] = rend;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        let fx = c.g[A0][0];
        c.g[S0][0] = fx;
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 21, 0xe8);
        frame.save_fpr(c, 20, 0xe0);
        let fx = fx as u32;
        // Width.
        let vt = c.lw((rend as u32).wrapping_add(VTABLE));
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, 0x50);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(rend);
        call_ptr(c, slot.func as u32, 0x002f_46cc)?;
        let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
        c.g[A1][0] = rend;
        flare_u32_to_f1(c, 4);

        // centre.x = width * 0.5; height.
        c.cycles += 10;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        let vt = c.lw((rend as u32).wrapping_add(VTABLE));
        c.g[V1][0] = vt;
        c.mul_s(20, 1, 0);
        let slot = crate::call::VSlot::read(c, vt, 0x58);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(rend);
        call_ptr(c, slot.func as u32, 0x002f_471c)?;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        flare_u32_to_f1(c, 3);

        // centre, sun, ray = sun - centre; the renderer's blend word.
        c.cycles += 20;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 5);
        let s = sp(c);
        c.lwc1(0, fx.wrapping_add(SUN_X));
        c.mul_s(1, 1, 5);
        c.lwc1(3, fx.wrapping_add(SUN_Y));
        c.sub_s(4, 0, 20);
        c.swc1(20, s);
        c.swc1(0, s.wrapping_add(0x10));
        c.sub_s(2, 3, 1);
        c.swc1(1, s.wrapping_add(0x4));
        c.swc1(4, s.wrapping_add(0x20));
        c.swc1(3, s.wrapping_add(0x14));
        c.swc1(2, s.wrapping_add(0x24));
        let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
        c.g[V1][0] = rend;
        let state = c.lw((rend as u32).wrapping_add(RENDER_STATE));
        c.g[A0][0] = state;
        let blend = c.lw((rend as u32).wrapping_add(BLEND));
        c.g[V0][0] = blend;
        c.sh((state as u32).wrapping_add(0x10), blend);
        let count = c.lw(fx.wrapping_add(COUNT));
        c.g[V1][0] = count;
        c.g[S3][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 21);
            c.mov_s(20, 5);
            c.g[S2][0] = sx(fx.wrapping_add(FLARE_COLOUR));
            c.g[S1][0] = 0;
            c.g[V0][0] = sx(fx);
            loop {
                // pos = centre + ray * dist; colour copy.
                c.cycles += 32;
                let s = sp(c);
                let flare = c.g[V0][0] as u32;
                c.lwc1(3, s.wrapping_add(0x20));
                c.lwc1(4, flare.wrapping_add(FLARE_DIST));
                c.lwc1(1, s.wrapping_add(0x24));
                c.mul_s(3, 3, 4);
                c.lwc1(2, s);
                c.mul_s(1, 1, 4);
                c.lwc1(0, s.wrapping_add(0x4));
                c.add_s(2, 2, 3);
                c.swc1(3, s.wrapping_add(0x40));
                c.add_s(0, 0, 1);
                c.swc1(1, s.wrapping_add(0x44));
                c.swc1(2, s.wrapping_add(0x30));
                c.swc1(0, s.wrapping_add(0x34));
                let colour = c.g[S2][0] as u32;
                crate::game::fe::ovstatehud::copy16_unaligned(c, V0, V1, colour, s.wrapping_add(0x50));
                c.g[A0][0] = 6;
                SUN_FLARE_INTENSITY.call(c, 0x002f_480c)?;

                // colour.r *= intensity * sun flare intensity.
                let s = sp(c);
                c.lwc1(2, fx.wrapping_add(INTENSITY));
                let flare = fx.wrapping_add(c.g[S1][0] as u32);
                c.g[V0][0] = sx(flare);
                c.lwc1(1, s.wrapping_add(0x50));
                c.g[V1][0] = 1;
                c.mul_s(2, 2, 0);
                let ty = c.lw(flare.wrapping_add(FLARE_TYPE));
                c.g[A0][0] = ty;
                c.mul_s(1, 1, 2);
                c.swc1(1, s.wrapping_add(0x50));

                // uv0 (sp+0x40) and uv1 (sp+0x60) by type, staged at
                // sp+0x70; f20 = 0.5, f21 = 1.0.
                let ty = ty as i64;
                let scratch = s.wrapping_add(0x70);
                let (uv0, uv1) = (s.wrapping_add(0x40), s.wrapping_add(0x60));
                if ty == 1 {
                    c.cycles += 14;
                    c.swc1(20, scratch);
                    c.sw(scratch.wrapping_add(4), 0);
                    flare_copy8(c, A1, scratch, uv0);
                    c.swc1(21, scratch);
                    c.swc1(20, scratch.wrapping_add(4));
                    flare_copy8(c, A0, scratch, uv1);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (ty < 2) as u64;
                    c.g[V0][0] = 2;
                    if ty < 2 {
                        c.cycles += 2;
                        c.sw(scratch, 0);
                        if ty == 0 {
                            c.cycles += 13;
                            c.sw(scratch.wrapping_add(4), 0);
                            flare_copy8(c, V0, scratch, uv0);
                            c.swc1(20, scratch);
                            c.swc1(20, scratch.wrapping_add(4));
                            flare_copy8(c, A2, scratch, uv1);
                        } else {
                            // Negative: the default.
                            c.cycles += 2;
                            c.sw(scratch.wrapping_add(4), 0);
                            flare_default_uv(c, scratch, uv0, uv1);
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                        if ty == 2 {
                            c.cycles += 14;
                            c.sw(scratch, 0);
                            c.swc1(20, scratch.wrapping_add(4));
                            flare_copy8(c, V1, scratch, uv0);
                            c.swc1(20, scratch);
                            c.swc1(21, scratch.wrapping_add(4));
                            flare_copy8(c, V0, scratch, uv1);
                        } else if ty == 3 {
                            c.cycles += 2;
                            c.swc1(20, scratch);
                            c.cycles += 13;
                            c.swc1(20, scratch.wrapping_add(4));
                            flare_copy8(c, A2, scratch, uv0);
                            c.swc1(21, scratch);
                            c.swc1(21, scratch.wrapping_add(4));
                            flare_copy8(c, A1, scratch, uv1);
                        } else {
                            // Not 3 either (+2), the zero store (+2).
                            c.cycles += 4;
                            c.sw(scratch, 0);
                            c.cycles += 1;
                            c.sw(scratch.wrapping_add(4), 0);
                            flare_default_uv(c, scratch, uv0, uv1);
                        }
                    }
                }
                let flare = fx.wrapping_add(c.g[S1][0] as u32);
                c.g[V0][0] = sx(flare);

                // DrawSprite(R, (pos, 0, 1), (size, size), uv0, uv1, colour).
                c.cycles += 30;
                c.g[AT][0] = 0x4396_0000;
                c.mtc1(AT, 1);
                c.lwc1(0, flare.wrapping_add(FLARE_SIZE));
                c.g[A1][0] = sx(s.wrapping_add(0x80));
                let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
                c.g[V1][0] = rend;
                c.g[A2][0] = sx(scratch);
                c.mul_s(0, 0, 1);
                c.lwc1(2, s.wrapping_add(0x30));
                c.lwc1(1, s.wrapping_add(0x34));
                c.g[A3][0] = sx(uv0);
                c.g[T0][0] = sx(uv1);
                c.g[T1][0] = sx(s.wrapping_add(0x50));
                c.swc1(0, scratch.wrapping_add(4));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.swc1(0, scratch);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(FLARE_STRIDE));
                let vt = c.lw((rend as u32).wrapping_add(VTABLE));
                c.g[V0][0] = vt;
                let slot = crate::call::VSlot::read(c, vt, 0x270);
                c.g[A0][0] = slot.delta;
                c.swc1(2, s.wrapping_add(0x80));
                c.swc1(1, s.wrapping_add(0x84));
                c.g[A0][0] = slot.this(rend);
                c.sw(s.wrapping_add(0x88), 0);
                c.swc1(21, s.wrapping_add(0x8c));
                c.g[V1][0] = slot.func;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(FLARE_STRIDE));
                call_ptr(c, slot.func as u32, 0x002f_49d4)?;
                let count = c.lw(fx.wrapping_add(COUNT));
                let more = (c.g[S3][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[V0][0] = sx(fx.wrapping_add(c.g[S1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_47b0);
                }
            }
        }

        c.cycles += 9;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// The default flare texture quarter: uv0 (0, 0) (the zeroed scratch),
/// uv1 (0.5, 1).
fn flare_default_uv(c: &mut Cpu, scratch: u32, uv0: u32, uv1: u32) {
    c.cycles += 11;
    flare_copy8(c, A0, scratch, uv0);
    c.swc1(20, scratch);
    c.swc1(21, scratch.wrapping_add(4));
    flare_copy8(c, V1, scratch, uv1);
}

// ---------------------------------------------------------- AnimSequence_Save

/// The fixed part of an `AnimSequence` `AnimSequence_Save` writes, in
/// order: (offset, size). The key list head is at +0xac.
const ANIM_SEQUENCE_FIELDS: [(u32, u64); 16] = [
    (0x0, 0x4),
    (0x4, 0x54),
    (0x60, 0x20),
    (0x80, 0x4),
    (0x84, 0x4),
    (0x88, 0x8),
    (0x90, 0x4),
    (0x94, 0x4),
    (0x98, 0x4),
    (0x9c, 0x4),
    (0xa0, 0x4),
    (0xa4, 0x4),
    (0xa8, 0x4),
    (0xb0, 0x10),
    (0xc0, 0x4),
    (0xc4, 0x4),
];

/// `void AnimSequence_Save(AnimSequence* seq, cAsStream* s)` (0x313dd0):
/// writes a sequence to a stream through the stream's virtual
/// `Write(data, size)` (vtable slot +0x8): the sixteen fixed fields, the
/// number of keys in the list at +0xac (linked through +0xc), then each
/// key's 16 bytes. Frame 0x50 (s0-s2; the count at +0).
pub fn anim_sequence_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 122;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        let stream = c.g[A1][0];
        c.g[S1][0] = stream;
        frame.save_ra(c, 0x10);
        let seq = c.g[A0][0];
        c.g[S2][0] = seq;
        let seq = seq as u32;
        for (i, &(off, size)) in ANIM_SEQUENCE_FIELDS.iter().enumerate() {
            let vt = c.lw(stream as u32);
            c.g[V1][0] = vt;
            c.g[A1][0] = sx(seq.wrapping_add(off));
            c.g[A2][0] = size;
            let slot = crate::call::VSlot::read(c, vt, 0x8);
            c.g[A0][0] = slot.delta;
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(stream);
            call_ptr(c, slot.func as u32, 0x0031_3e08 + 0x1c * i as u32)?;
        }

        // The key count.
        let head = c.lw(seq.wrapping_add(0xac));
        c.g[S0][0] = head;
        let s = sp(c);
        c.sw(s, 0);
        if head != 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            loop {
                c.cycles += 7;
                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_3fc0);
                }
            }
            c.cycles += 1;
            c.sw(sp(c), c.g[V0][0]);
        }
        c.cycles += 10;
        let vt = c.lw(stream as u32);
        c.g[V0][0] = vt;
        c.g[A1][0] = sx(sp(c));
        c.g[A2][0] = 0x4;
        let slot = crate::call::VSlot::read(c, vt, 0x8);
        c.g[A0][0] = slot.delta;
        c.g[V1][0] = slot.func;
        c.g[A0][0] = slot.this(stream);
        call_ptr(c, slot.func as u32, 0x0031_3ffc)?;

        // The keys.
        let head = c.lw(seq.wrapping_add(0xac));
        c.g[S0][0] = head;
        if head != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(stream as u32);
            loop {
                c.cycles += 9;
                let key = c.g[S0][0];
                c.g[A1][0] = key;
                c.g[A2][0] = 0x10;
                let vt = c.g[V0][0];
                let slot = crate::call::VSlot::read(c, vt, 0x8);
                c.g[A0][0] = slot.delta;
                c.g[V1][0] = slot.func;
                c.g[A0][0] = slot.this(stream);
                call_ptr(c, slot.func as u32, 0x0031_4028)?;
                c.g[S0][0] = c.lw((key as u32).wrapping_add(0xc));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = c.lw(stream as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4010);
                }
            }
        }
        frame.restore(c, S0, 0x40);

        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001f_16c0, hud_draw_time_elem, "HUD_DrawTimeElem"),
    (0x0014_8950, be_char_int_get_overall_rating, "BECharInt_GetOverallRating"),
    (0x0031_1710, ai_anim_event_map_pick_anim, "AIAnimEventMap_PickAnim"),
    (0x0031_9e48, mem_pool_alloc, "MEM_PoolAlloc"),
    (0x003b_70f8, sndstrm_is_prebuffering, "SNDSTRM_IsPrebuffering"),
    (0x0041_39f8, fixunssfsi, "__fixunssfsi"),
    (0x0030_19c8, wscript_fn_get_object_slot32, "WScriptFn_GetObjectSlot32"),
    (0x002f_4690, lens_fx_draw_flare_set, "LensFX_DrawFlareSet"),
    (0x0031_3dd0, anim_sequence_save, "AnimSequence_Save"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};

    /// A uniform float in `lo..hi`.
    fn oc238_uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    fn oc238_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// HUD_DrawTimeElem: element definitions with random positions, scales,
    /// colours and alignments; with and without scale multiplier, position
    /// offset and caller colour; the real HUD_DrawTime behind it (the HUD
    /// fixture of its own test: font renderer whose DrawText is a fake that
    /// may unwind).
    #[test]
    fn hud_draw_time_elem_matches() {
        use crate::game::fe::ovstatehud::hud_time::*;
        const HUD: u32 = DATA + 0x1_0000;
        const FONT_AT: u32 = DATA + 0x1_1000;
        const VT: u32 = DATA + 0x1_2000;
        const ARGS: u32 = DATA + 0x1_3000;
        const DEFS: u32 = DATA + 0x1_4000;
        covered(
            ee::s04::f_001f16c0__generated,
            hud_draw_time_elem,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f_16c0, 6);
                c.pc = 0x001f_16c0;
                for off in (0..0x100).step_by(8) {
                    c.bus.write16(VT + off, ((r.next_u32() % 16) * 4) as u16);
                    c.bus.write32(VT + off + 4, FAKE + 2 * off);
                }
                let rend = DATA + 0x1_8000;
                c.bus.write32(GAME_GP + 0x2a90, rend);
                c.bus
                    .write32(rend + 0x10d8, VT - 0x280 + 8 * (r.next_u32() % 8));
                let state = DATA + 0x1_c000 + 4 * (r.next_u32() % 16);
                c.bus.write32(rend + 0xe84, state);
                fill(c, &mut r, state & !0xf, 0x20);
                // The font.
                fill(c, &mut r, FONT_AT, 0x20);
                for i in 0..8 {
                    c.bus.write32(
                        FONT_AT + 0x18 + 4 * i,
                        oc238_uf(&mut r, 0.5, 30.0).to_bits(),
                    );
                }
                for i in 0..2 {
                    let v = if r.next_u32() % 2 == 0 { 0 } else { r.f32(3.0) };
                    c.bus.write32(FONT_AT + 0x28 + 4 * i, v);
                }
                c.bus.write32(FONT_AT + FONT_LINES, 1 + r.next_u32() % 3);
                let codes = DATA + 0x3000;
                let n = 1 + r.next_u32() % 40;
                let mut code = 0;
                for i in 0..n {
                    code += 1 + r.next_u32() % 6;
                    c.bus.write16(codes + 2 * i, code as u16);
                }
                fill(c, &mut r, DATA + 0x4000, 0x400);
                c.bus.write32(FONT_AT, n);
                c.bus.write32(FONT_AT + 4, codes);
                c.bus
                    .write32(FONT_AT + 8, DATA + 0x4000 + 12 * (r.next_u32() % 8));
                c.bus.write32(FONT_AT + 0xc, r.next_u32() % 64);
                c.bus.write32(FONT_AT + 0x10, 64 + r.next_u32() % 128);
                // The HUD.
                fill(c, &mut r, HUD + 0x160, 0x8);
                for off in [SEP_W, DIGIT_W, SIGN_ADVANCE, ONE_W] {
                    c.bus.write32(HUD + off, oc238_uf(&mut r, 1.0, 24.0).to_bits());
                }
                c.bus.write32(HUD + FONT, FONT_AT);
                c.bus.write32(HUD + SEPARATOR, ARGS + 0x100);
                oc238_cstr(c, ARGS + 0x100, b":");
                oc238_cstr(c, STR_PLUS, b"+");
                oc238_cstr(c, STR_MINUS, b"-");
                oc238_cstr(c, FMT_TWO_DIGITS, b"%02d");
                oc238_cstr(c, FMT_FRACTION, b".%02d");
                // Element definitions: pos (s16), scale, colour, alignment.
                for i in 0..4 {
                    let d = DEFS + 0x24 * i;
                    fill(c, &mut r, d, 9);
                    c.bus.write16(d, (r.next_u32() % 600) as u16);
                    c.bus.write16(d + 2, (r.next_u32() % 400) as u16);
                    c.bus.write32(d + 8, oc238_uf(&mut r, 0.25, 2.0).to_bits());
                    c.bus.write32(d + 0xc, oc238_uf(&mut r, 0.25, 2.0).to_bits());
                    c.bus.write8(d + 0x20, (r.next_u32() % 4) as u8);
                    c.bus.write8(d + 0x21, (r.next_u32() % 4) as u8);
                }
                // Optional scale multiplier, position offset, colour.
                let opt = |c: &mut Cpu, r: &mut Rng, on: bool, at: u32| -> u32 {
                    if on {
                        for k in 0..4 {
                            c.bus.write32(at + 4 * k, oc238_uf(r, 0.5, 2.0).to_bits());
                        }
                        at
                    } else {
                        0
                    }
                };
                let scale_mul = opt(c, &mut r, seed % 2 == 0, ARGS + 0x40);
                let pos_off = opt(c, &mut r, seed % 3 == 0, ARGS + 0x50);
                let colour = if seed % 4 < 2 {
                    let at = ARGS + 0x60 + r.next_u32() % 8;
                    for k in 0..16 {
                        c.bus.write8(at + k, r.next_u32() as u8);
                    }
                    at
                } else {
                    0
                };
                let sp0 = crate::difftest::SP;
                c.bus.write32(sp0, r.next_u32() % 4);
                c.bus.write32(sp0 + 0x8, scale_mul);
                c.bus.write32(sp0 + 0x10, pos_off);
                c.bus.write32(sp0 + 0x18, colour);
                let group = |r: &mut Rng| -> u64 {
                    let v = match r.next_u32() % 5 {
                        0 => 11,
                        1 => 10 + r.next_u32() % 10,
                        2 => 1 + 10 * (r.next_u32() % 10),
                        3 => r.next_u32() % 100,
                        _ => r.next_u32() % 10,
                    };
                    v as u64
                };
                c.g[A0][0] = HUD as u64;
                c.g[A1][0] = group(&mut r);
                c.g[A2][0] = group(&mut r);
                c.g[A3][0] = group(&mut r);
                c.g[T0][0] = group(&mut r);
                c.g[T1][0] = (r.next_u32() % 8) as u64;
                c.g[T2][0] = DEFS as u64;
                c.g[T3][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    /// BECharInt_GetOverallRating: the real attribute getters on a rider
    /// table (0x5305b0: rider -> profile slot), profiles (0x535b20, stride
    /// 0x1c: character byte at +0x11, the -1 / odd-flag words at +0xc /
    /// +0x10) and a random signed attribute table (0x535538), so ratings
    /// come out positive, zero and negative (the minimum of 1).
    #[test]
    fn be_char_int_get_overall_rating_matches() {
        check(
            ee::s01::f_00148950__generated,
            be_char_int_get_overall_rating,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_8950, 0);
                c.pc = 0x0014_8950;
                for i in 0..8u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                for i in 0..8u32 {
                    let p = 0x0053_5b20 + 0x1c * i;
                    fill(c, &mut r, p, 7);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(p + 0xc, 0xffff_ffff);
                    }
                    c.bus.write8(p + 0x11, (r.next_u32() % 10) as u8);
                }
                // Mostly positive attributes; some seeds all negative.
                let neg = seed % 5 == 0;
                for i in 0..0x100u32 {
                    let v = r.next_u32() as u8;
                    let v = if neg { v | 0x80 } else if seed % 5 < 3 { v & 0x7f } else { v };
                    c.bus.write8(0x0053_5538 + i, v);
                }
                c.g[A0][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    /// AIAnimEventMap_PickAnim: ranges of count 1 (direct pick), 0 or
    /// negative (empty sums), and 2..8 entries with flags that match the
    /// mask or not and small weights (including zero and negative), so the
    /// weighted walk returns early, runs off the end, or divides by a zero
    /// total; the real random generator with a random state.
    #[test]
    fn ai_anim_event_map_pick_anim_matches() {
        check(
            ee::s10::f_00311710__generated,
            ai_anim_event_map_pick_anim,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_1710, 0);
                c.pc = 0x0031_1710;
                fill(c, &mut r, 0x004f_f030, 8);
                // Entries 0..64.
                let mask = [0u32, 1, 3, 4][(seed % 4) as usize];
                for i in 0..64u32 {
                    let e = 0x0044_9960 + 12 * i;
                    let w = match r.next_u32() % 6 {
                        0 => 0,
                        1 => (r.next_u32() % 5).wrapping_neg(),
                        _ => r.next_u32() % 20,
                    };
                    c.bus.write32(e, w);
                    let flags = if r.next_u32() % 3 == 0 { r.next_u32() & 7 } else { mask | (r.next_u32() & 8) };
                    c.bus.write32(e + 4, flags);
                    c.bus.write32(e + 8, r.next_u32());
                }
                let t = DATA + (r.next_u32() & 0xff0);
                let idx = r.next_u32() % 4;
                let start = r.next_u32() % 48;
                let count = match seed % 7 {
                    0 => 1,
                    1 => 0,
                    2 => 0xffff,
                    _ => 2 + r.next_u32() % 12,
                };
                c.bus.write16(t + 4 * idx, start as u16);
                c.bus.write16(t + 4 * idx + 2, count as u16);
                c.g[A0][0] = t as u64;
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = mask as u64;
            },
        );
    }

    /// Runs a generated function to completion as a setup step.
    fn oc238_run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, crate::difftest::RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// MEM_PoolAlloc: pools ready (64 free nodes per size class) or not;
    /// sizes in every class, at the class limits (rounding up by 4 crosses
    /// them), above 0x200 and huge; alignments 0..128 (bigger than the size
    /// or not); the main dlmalloc heap behind MEM_AllocInternal.
    #[test]
    fn mem_pool_alloc_matches() {
        check(ee::s10::f_00319e48__generated, mem_pool_alloc, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x31_9e48, 0);
            oc238_run(
                c,
                0x0031_ff60,
                ee::s10::f_0031ff60,
                &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
            );
            if seed % 4 != 0 {
                c.bus.write32(GAME_GP + 0xdd8, 1);
                // (gp slot, free-list head offset in the pool, block stride)
                const POOLS: [(u32, u32, u32); 7] = [
                    (0x2a70, 0x800, 0x40),
                    (0x2a60, 0x2000, 0x40),
                    (0x2a6c, 0x6000, 0x40),
                    (0x2a7c, 0x32000, 0x40),
                    (0x2a84, 0x25800, 0x80),
                    (0x2a78, 0x20000, 0x100),
                    (0x2a68, 0x20000, 0x200),
                ];
                let mut pool = DATA + 0xc_0000;
                let mut base = DATA + 0x8_0000;
                for (slot, head, stride) in POOLS {
                    for n in 0..64u32 {
                        let node = base + stride * n;
                        c.bus.write32(node, if n == 63 { 0 } else { node + stride });
                    }
                    c.bus.write32(pool + head, base);
                    c.bus.write32(GAME_GP + slot, pool);
                    pool += 0x100;
                    base += 0x8000;
                }
            } else {
                c.bus.write32(GAME_GP + 0xdd8, 0);
            }
            let limits = [8u32, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200];
            let size = match seed % 6 {
                0 => r.next_u32() % 0x220,
                1 => limits[(r.next_u32() % 7) as usize] - (r.next_u32() % 4),
                2 => limits[(r.next_u32() % 7) as usize] + 1 + (r.next_u32() % 3),
                3 => r.next_u32() % 0x40,
                4 => r.next_u32() % 0x2000,
                _ => r.next_u32() % 0x210,
            };
            c.g[A0][0] = size as u64;
            c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128, 0x100][(r.next_u32() % 8) as usize];
            c.pc = 0x0031_9e48;
        });
    }

    /// The SNDSTRM_IsPrebuffering fixture: a stream at DATA with its file
    /// handle (StreamFile magic, or a bad one so the real getters answer 0),
    /// channel bytes below and above the 4e6 cap, read states 0..3, queued
    /// counts negative / zero / positive, and prebuffer state absent or with
    /// zero / random rate and negative / zero / small / large targets. Some
    /// seeds enter at the reset resume point (inside the frame, s0 = the
    /// prebuffer state). `evented` biases toward the two event-checked
    /// branches (large target, read state 2 or 0).
    fn oc238_prebuffer_setup(c: &mut Cpu, seed: u32, evented: bool) -> u32 {
        let mut r = machine(c, seed, 0x3b_70f8, 0);
        let strm = DATA;
        let pre = DATA + 0x400;
        let file = DATA + 0x800;
        let state = DATA + 0xc00;
        c.bus.write32(strm, file);
        let queued = match r.next_u32() % 3 {
            0 => 0,
            1 => 1 + r.next_u32() % 4,
            _ => (1 + r.next_u32() % 4).wrapping_neg(),
        };
        c.bus.write32(strm + 0x130, queued);
        let has_pre = evented || seed % 11 != 0;
        c.bus.write32(strm + 0x134, if has_pre { pre } else { 0 });
        let rate = if !evented && seed % 13 == 0 { 0 } else { 1 + r.next_u32() % 200_000 };
        c.bus.write32(pre + 0x10, rate);
        let target = if evented {
            0x4000_0000
        } else {
            match r.next_u32() % 6 {
                0 => 0,
                1 => (1 + r.next_u32() % 100).wrapping_neg(),
                2 => 0x7fff_0000,
                _ => r.next_u32() % 5000,
            }
        };
        c.bus.write32(pre + 0x20, target);
        c.bus.write32(file, state);
        c.bus.write32(file + 8, r.next_u32() % 0x50_0000);
        let magic = if !evented && seed % 9 == 0 { 0x1234_5678 } else { 0x4d52_5453 };
        c.bus.write32(state, magic);
        let read = if evented { [0, 2][(r.next_u32() % 2) as usize] } else { r.next_u32() % 4 };
        c.bus.write32(state + 0x38, read);
        c.g[A0][0] = strm as u64;
        if seed % 10 == 7 {
            // Resumed at the reset: inside the frame, s0 = the state.
            c.g[SP][0] = (crate::difftest::SP - 0x30) as u64;
            fill(c, &mut r, crate::difftest::SP - 0x30, 12);
            c.g[S0][0] = pre as u64;
            // The saved ra (sp+0) returns to the test's sentinel.
            c.bus.write64(crate::difftest::SP - 0x30, crate::difftest::RA as u64);
            0x003b_7184
        } else {
            0x003b_70f8
        }
    }

    crate::difftest::events::driven!(oc238_gen_prebuffering, ee::s13::f_003b70f8__generated);

    crate::difftest::events::driven!(oc238_hand_prebuffering, sndstrm_is_prebuffering);

    /// SNDSTRM_IsPrebuffering: every branch (real StreamFile getters), the
    /// resume entry, and events delivered or unwinding at the two
    /// event-checked branches to the reset.
    #[test]
    fn sndstrm_is_prebuffering_matches() {
        check(ee::s13::f_003b70f8__generated, sndstrm_is_prebuffering, 300, &|c, seed| {
            let entry = oc238_prebuffer_setup(c, seed, false);
            c.pc = entry;
        });
        crate::difftest::events::check_events(
            oc238_gen_prebuffering,
            oc238_hand_prebuffering,
            120,
            &|c, seed| {
                let entry = oc238_prebuffer_setup(c, seed, true);
                let mut r = Rng(0x3b_70f8 ^ seed as u64);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, entry);
                c.next_event = c.cycles + 4;
                mode
            },
        );
    }

    /// The __fixunssfsi argument: edge cases, NaN / infinity bit patterns,
    /// negatives, values below one, around 2^30 / 2^31 / 2^32 and random
    /// magnitudes. `small`: mostly in (0, 1) (the event-checked branch).
    /// Some seeds enter at the "answer 0" resume point inside the frame.
    fn oc238_fixunssfsi_setup(c: &mut Cpu, seed: u32, small: bool) -> u32 {
        let mut r = machine(c, seed, 0x41_39f8, 0);
        let bits = if small && seed % 4 != 0 {
            oc238_uf(&mut r, 0.0, 0.999).to_bits()
        } else {
            match seed % 9 {
                0 => r.edge_f32(),
                1 => 0x7f80_0000 | (r.next_u32() & 0x807f_ffff),
                2 => oc238_uf(&mut r, -1e6, 0.0).to_bits(),
                3 => oc238_uf(&mut r, 0.0, 1.0).to_bits(),
                4 => (0x4e80_0000u32 - 2 + r.next_u32() % 5) as u32,
                5 => (0x4f00_0000u32 - 2 + r.next_u32() % 5) as u32,
                6 => (0x4f80_0000u32 - 2 + r.next_u32() % 5) as u32,
                7 => (0x3f80_0000 + (r.next_u32() % 32 << 23)) | (r.next_u32() & 0x7f_ffff),
                _ => r.next_u32(),
            }
        };
        c.f[12] = bits;
        if seed % 10 == 7 {
            c.g[SP][0] = (crate::difftest::SP - 0x30) as u64;
            fill(c, &mut r, crate::difftest::SP - 0x30, 12);
            // The saved ra (sp+0x20) returns to the test's sentinel.
            c.bus.write64(crate::difftest::SP - 0x10, crate::difftest::RA as u64);
            0x0041_3a28
        } else {
            0x0041_39f8
        }
    }

    crate::difftest::events::driven!(oc238_gen_fixunssfsi, ee::s15::f_004139f8__generated);

    crate::difftest::events::driven!(oc238_hand_fixunssfsi, fixunssfsi);

    /// __fixunssfsi: every class and exponent range with the real
    /// __unpack_f; the resume entry; events at the below-one branch.
    #[test]
    fn fixunssfsi_matches() {
        check(ee::s15::f_004139f8__generated, fixunssfsi, 400, &|c, seed| {
            let entry = oc238_fixunssfsi_setup(c, seed, false);
            c.pc = entry;
        });
        crate::difftest::events::check_events(
            oc238_gen_fixunssfsi,
            oc238_hand_fixunssfsi,
            120,
            &|c, seed| {
                let entry = oc238_fixunssfsi_setup(c, seed, true);
                let mut r = Rng(0x41_39f8 ^ seed as u64);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, entry);
                c.next_event = c.cycles + 4;
                mode
            },
        );
    }

    /// WScriptFn_GetObjectSlot32: defaults initialised or not; 0..3 passed
    /// arguments (none, or n <= 0) with matching / int-for-float / other
    /// types; ids -1 (the player, present or not) and class/handle ids with
    /// a missing class, a zero handle or an object; objects with or without
    /// a controller, whose slot-32 method is a fake (may unwind).
    #[test]
    fn wscript_fn_get_object_slot32_matches() {
        covered(
            ee::s10::f_003019c8__generated,
            wscript_fn_get_object_slot32,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_19c8, 1);
                c.pc = 0x0030_19c8;
                let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
                let ready = seed % 4 != 0;
                c.bus.write32(gp(0x25fc), ready as u32);
                // An object id: -1 (the player) or handle << 8 | class.
                let some_id = |r: &mut Rng| -> u32 {
                    if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (r.next_u32() % 16) << 8 | r.next_u32() % 8
                    }
                };
                fill(c, &mut r, 0x004f_bc00, 3);
                let default_id = some_id(&mut r);
                c.bus.write32(0x004f_bc00, default_id);
                // Declared types: id int, time float, a third slot.
                c.bus.write32(gp(0xba8), r.next_u32() % 2);
                c.bus.write32(gp(0xba8 + 4), if seed % 5 == 0 { 1 } else { 2 });
                c.bus.write32(gp(0xba8 + 8), r.next_u32() % 3);
                c.bus.write32(gp(-0x3368), (1.0f32 / 30.0).to_bits());
                // Objects.
                let mgr = DATA + 0x100;
                let m = DATA + 0x200;
                let tables = DATA + 0x300;
                let class = DATA + 0x400;
                let handles = DATA + 0x800;
                let world = DATA + 0xc00;
                let obj = DATA + 0x1000;
                let ctrl = DATA + 0x1100;
                let vt = DATA + 0x1200;
                c.bus.write32(gp(0x16c8), mgr);
                c.bus.write32(mgr, m);
                c.bus.write32(m + 8, tables);
                for k in 0..8 {
                    let present = r.next_u32() % 4 != 0;
                    c.bus.write32(tables + 4 * k, if present { class } else { 0 });
                }
                c.bus.write32(class + 0x1c, handles);
                for k in 0..16 {
                    let word = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xff
                    } else {
                        ((obj >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    c.bus.write32(handles + 4 * k, word);
                }
                c.bus.write32(gp(0xce8), world);
                c.bus
                    .write32(world + 0x290, if seed % 7 == 0 { 0 } else { obj });
                c.bus
                    .write32(obj + 0xc, if seed % 5 == 1 { 0 } else { ctrl });
                c.bus.write32(ctrl + 0xc, vt);
                c.bus.write16(vt + 0x100, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(vt + 0x104, FAKE);
                // Arguments {idx, val, _, type}.
                let args = DATA + 0x2000;
                let n = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + r.next_u32() % 3,
                };
                for k in 0..3 {
                    let a = args + 0x10 * k;
                    let idx = if k == 0 { r.next_u32() % 2 } else { r.next_u32() % 3 };
                    c.bus.write32(a, idx);
                    let val = if idx == 0 {
                        some_id(&mut r)
                    } else if r.next_u32() % 2 == 0 {
                        r.next_u32() % 1000
                    } else {
                        oc238_uf(&mut r, -100.0, 100.0).to_bits()
                    };
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, r.next_u32());
                    c.bus.write32(a + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = (DATA + 0x3000) as u64;
                c.g[A1][0] = sx(n);
                c.g[A2][0] = args as u64;
            },
        );
    }

    /// LensFX_DrawFlareSet: the renderer's width / height through a real
    /// word getter (0x2743c0: positive, or with the top bit set for the
    /// unsigned-conversion path), 0..5 flares (or a negative count) of every
    /// type (-1..5), the painter's sun flare intensity and DrawSprite as
    /// fakes (some seeds unwind).
    #[test]
    fn lens_fx_draw_flare_set_matches() {
        const GET_WORD: u32 = 0x0027_43c0;
        covered(
            ee::s09::f_002f4690__generated,
            lens_fx_draw_flare_set,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_4690, 6);
                c.pc = 0x002f_4690;
                let rend = DATA + 0x1_0000;
                let rvt = DATA + 0x1_2000;
                let state = DATA + 0x1_3000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), rend);
                c.bus.write32(rend + 0x10d8, rvt);
                c.bus.write32(rend + 0xe84, state);
                c.bus.write32(rend + 0x1004, r.next_u32());
                fill(c, &mut r, state, 8);
                let dim = |r: &mut Rng, top: bool| -> u32 {
                    if top { 0x8000_0000 | r.next_u32() } else { r.next_u32() % 1024 }
                };
                let w = dim(&mut r, seed % 4 == 1);
                let h = dim(&mut r, seed % 5 == 2);
                c.bus.write32(rend, w);
                c.bus.write32(rend + 4, h);
                c.bus.write16(rvt + 0x50, 0);
                c.bus.write32(rvt + 0x54, GET_WORD);
                c.bus.write16(rvt + 0x58, 4);
                c.bus.write32(rvt + 0x5c, GET_WORD);
                c.bus.write16(rvt + 0x270, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(rvt + 0x274, FAKE + 0x20);
                // The painter's sun flare entry (index 6).
                let p = DATA + 0x1_4000;
                let obj = DATA + 0x1_4100;
                let pvt = DATA + 0x1_4200;
                c.bus.write32(0x004f_a370 + 6 * 0xf0 + 0x14, p);
                c.bus.write32(p, obj);
                c.bus.write32(obj + 4, pvt);
                c.bus.write16(pvt + 0x110, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(pvt + 0x114, FAKE + 0x10);
                // The flares.
                let fx = DATA + 0x1_8000;
                c.bus.write32(fx + 0x1f0, oc238_uf(&mut r, 0.0, 640.0).to_bits());
                c.bus.write32(fx + 0x1f4, oc238_uf(&mut r, 0.0, 448.0).to_bits());
                let count = match seed % 7 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    _ => 1 + r.next_u32() % 5,
                };
                c.bus.write32(fx + 0x200, count);
                c.bus.write32(fx + 0x204, oc238_uf(&mut r, 0.0, 2.0).to_bits());
                for k in 0..5 {
                    let f = fx + 0x1c * k;
                    let ty = (r.next_u32() % 7).wrapping_sub(1);
                    c.bus.write32(f + 0x30, ty);
                    c.bus.write32(f + 0x34, oc238_uf(&mut r, -1.0, 2.0).to_bits());
                    c.bus.write32(f + 0x38, oc238_uf(&mut r, 0.0, 0.5).to_bits());
                    for j in 0..4 {
                        c.bus.write32(f + 0x3c + 4 * j, oc238_uf(&mut r, 0.0, 255.0).to_bits());
                    }
                }
                c.g[A0][0] = fx as u64;
            },
        );
    }

    /// AnimSequence_Save: a stream whose Write is a fake (some seeds
    /// unwind, at any of the up to 21 calls); key lists of 0..4 keys.
    #[test]
    fn anim_sequence_save_matches() {
        covered(ee::s10::f_00313dd0__generated, anim_sequence_save, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x31_3dd0, 21);
            c.pc = 0x0031_3dd0;
            let seq = DATA + 0x1000;
            let stream = DATA + 0x2000;
            let vt = DATA + 0x3000;
            fill(c, &mut r, seq, 0x34);
            c.bus.write32(stream, vt);
            c.bus.write16(vt + 0x8, ((r.next_u32() & 0x3f) * 4) as u16);
            c.bus.write32(vt + 0xc, FAKE);
            let n = seed % 5;
            let mut next = 0;
            for k in (0..n).rev() {
                let key = DATA + 0x4000 + 0x40 * k;
                fill(c, &mut r, key, 4);
                c.bus.write32(key + 0xc, next);
                next = key;
            }
            c.bus.write32(seq + 0xac, next);
            c.g[A0][0] = seq as u64;
            c.g[A1][0] = stream as u64;
        });
    }
}
