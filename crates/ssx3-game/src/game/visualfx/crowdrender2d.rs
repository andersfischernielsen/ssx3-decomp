//! `src/visualfx/crowdrender2d.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_b9c8, c_crowd_anim2d_ctor, "cCrowdAnim2D_ctor"),
    (0x002d_bac0, c_crowd_anim2d_update, "cCrowdAnim2D_Update"),
];


// ---------------------------------------------------------------------------
// 2D crowd / fog particles.

/// `cCrowdAnim2D *cCrowdAnim2D_ctor(cCrowdAnim2D *this, cCrowdAnim2DData *data)`
/// (0x2db9c8): initialise the animation record, storing the data pointer at
/// +0x1c and resolving the shared tables.
pub fn c_crowd_anim2d_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x4;
    Addr(this).off(0x1c).sw(c, c.g[A1][0]);
    Addr(this).off(0x8).sw(c, 0x4);
    c.g[A1][0] = 0x44_0000;
    Addr(this).off(0x4).sw(c, 0);
    c.g[A3][0] = 0x3;
    Addr(this).off(0xc).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x5dd0)).lw(c);
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x1c)).lw(c);
    Addr(this).off(0x14).sw(c, c.g[A3][0]);
    Addr(this).off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// The 2D crowd's animation tables in `.data`: `D_00445D38` (0xd8-byte
/// animations: `+4` frame count, `+0x98 + 4 * frame` texture), the
/// transition table `D_00445E10` (12-byte entries: next animation, mode,
/// kind) and its per-type base index `D_00445E20`.
const ANIMS: u32 = 0x0044_5d38;
const TRANSITIONS: u32 = 0x0044_5e10;
const TRANSITION_BASE: u32 = 0x0044_5e20;

/// Where a `cCrowdAnim2D_Update` path continues (the shared tails of the
/// compiled code).
enum Next {
    /// `v0` = the animation entry: load its frame count, then step.
    Count,
    /// Step the frame (`a0..a3` set).
    Step,
    /// Store `v0` as the frame.
    StoreFrame,
    /// Reload the animation.
    LoadAnim,
    /// `v1` = the animation: pick the texture.
    Texture,
}

/// `void cCrowdAnim2D_Update(cCrowdAnim2D* self)` (0x2dbac0): once per tick
/// for each 2D crowd sprite. When `+0x14` ticks have passed since the last
/// step (`+0x18`, against the app manager's update count `+0x1c`), advance
/// the animation: follow the transition table for the sprite's type `+0`
/// and animation `+4` (kind 0: switch at frame 0, kind 1: at the last
/// frame, kind 2: at once), step the frame `+0xc` in direction `+8`
/// (`cCrowdAnim2D_StepFrame`), and store the frame's texture at `+0x10`.
///
/// Tick rate: the interval is in 60 Hz frames (3 from the ctor), so above
/// 60 it is compared as `ticks_from_frames60(interval)` ticks.
pub fn c_crowd_anim2d_update(c: &mut Cpu) {
    const STEP_FRAME: Callee = Callee { addr: 0x002d_ba08, f: ee::s09::f_002dba08 };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[V1][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        c.g[A0][0] = Addr(this).off(0x14).lw(c);
        if crate::tick::rate(c) != 60 && (c.g[A0][0] as i64) >= 0 {
            c.g[A0][0] = crate::tick::ticks_from_frames60(c, c.g[A0][0] as u32) as u64;
        }
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            // The transition for this type and animation.
            c.cycles += 18;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x0044_0000;
            Addr(this).off(0x18).sw(c, c.g[A1][0]);
            c.g[V0][0] = sx(TRANSITION_BASE);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = 0xc;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = Addr(this).off(4).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = 0x0044_0000;
            c.mult(0, A0, A2, A1);
            c.g[V1][0] = sx(TRANSITIONS);
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let tr = r32(c, T1);
            c.g[A0][0] = Addr(tr).lw(c);
            let mut next;
            if c.g[A0][0] == c.g[A2][0] {
                // No transition: keep stepping this animation.
                c.cycles += 10;
                c.g[V1][0] = 0xd8;
                c.g[A2][0] = 0x0044_0000;
                c.mult(0, V1, A0, V1);
                c.g[V0][0] = sx(ANIMS);
                c.g[S1][0] = c.g[A2][0];
                c.g[A1][0] = Addr(this).off(0xc).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = sx(this.wrapping_add(8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                next = Next::Count;
            } else {
                c.g[T2][0] = Addr(tr).off(8).lw(c);
                let kind = c.g[T2][0];
                c.cycles += 3;
                c.g[A2][0] = 1;
                c.g[V0][0] = ((kind as i64) < 2) as u64;
                if kind == 1 {
                    // Kind 1: run backwards to the last frame, then switch.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this).off(8).lw(c);
                    c.g[A3][0] = sx(this.wrapping_add(8));
                    if c.g[V0][0] == kind {
                        c.cycles += 1;
                        Addr(this).off(8).sw(c, 0);
                    }
                    c.cycles += 12;
                    c.g[V1][0] = Addr(this).off(4).lw(c);
                    c.g[T3][0] = 0xd8;
                    c.g[A0][0] = 0x0044_0000;
                    c.g[A1][0] = Addr(this).off(0xc).lw(c);
                    c.mult(0, V1, V1, T3);
                    c.g[V0][0] = sx(ANIMS);
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[A2][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    c.g[S1][0] = c.g[A0][0];
                    if c.g[A1][0] != c.g[V0][0] {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        next = Next::Step;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = Addr(tr).lw(c);
                        Addr(this).off(4).sw(c, c.g[V1][0]);
                        c.g[V0][0] = Addr(tr).off(4).lw(c);
                        Addr(this).off(8).sw(c, c.g[V0][0]);
                        if c.g[V0][0] != kind {
                            c.cycles += 2;
                            Addr(this).off(0xc).sw(c, 0);
                            next = Next::LoadAnim;
                        } else {
                            c.cycles += 5;
                            c.mult(0, V0, V1, T3);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            next = Next::StoreFrame;
                        }
                    }
                } else {
                    c.cycles += 2;
                    let below_two = c.g[V0][0] != 0;
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    c.g[S1][0] = 0x0044_0000;
                    if !below_two && kind == 2 {
                        // Kind 2: switch at once and step the new animation.
                        c.cycles += 12;
                        Addr(this).off(4).sw(c, c.g[A0][0]);
                        c.g[V1][0] = 0xd8;
                        c.mult(0, V1, A0, V1);
                        c.g[T0][0] = 0x0044_0000;
                        c.g[A2][0] = Addr(tr).off(4).lw(c);
                        c.g[V0][0] = sx(ANIMS);
                        c.g[A1][0] = Addr(this).off(0xc).lw(c);
                        c.g[A0][0] = c.g[S0][0];
                        Addr(this).off(8).sw(c, c.g[A2][0]);
                        c.g[A3][0] = sx(this.wrapping_add(8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S1][0] = c.g[T0][0];
                        next = Next::Count;
                    } else if below_two && kind == 0 {
                        // Kind 0: run forwards to frame 0, then switch.
                        c.cycles += 3;
                        c.g[V0][0] = Addr(this).off(8).lw(c);
                        c.g[A3][0] = sx(this.wrapping_add(8));
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            Addr(this).off(8).sw(c, c.g[A2][0]);
                        }
                        c.cycles += 3;
                        c.g[A1][0] = Addr(this).off(0xc).lw(c);
                        if c.g[A1][0] != 0 {
                            c.g[V1][0] = Addr(this).off(4).lw(c);
                            c.cycles += 8;
                            c.g[A0][0] = 0xd8;
                            c.g[A2][0] = 0x0044_0000;
                            c.mult(0, V1, V1, A0);
                            c.g[V0][0] = sx(ANIMS);
                            c.g[S1][0] = c.g[A2][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            next = Next::Count;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = Addr(tr).lw(c);
                            Addr(this).off(4).sw(c, c.g[V1][0]);
                            c.g[V0][0] = Addr(tr).off(4).lw(c);
                            Addr(this).off(8).sw(c, c.g[V0][0]);
                            if c.g[V0][0] != c.g[A2][0] {
                                c.cycles += 3;
                                Addr(this).off(0xc).sw(c, 0);
                                c.g[S1][0] = 0x0044_0000;
                                next = Next::LoadAnim;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = 0xd8;
                                c.g[V0][0] = 0x0044_0000;
                                c.mult(0, A0, V1, A0);
                                c.g[S1][0] = c.g[V0][0];
                                c.g[V1][0] = sx(ANIMS);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V0][0] = Addr(r32(c, V1)).off(4).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                                next = Next::StoreFrame;
                            }
                        }
                    } else {
                        // Any other kind: no change.
                        c.cycles += 2;
                        c.g[V1][0] = Addr(this).off(4).lw(c);
                        next = Next::Texture;
                    }
                }
            }

            // The shared tails.
            loop {
                match next {
                    Next::Count => {
                        c.cycles += 1;
                        c.g[A2][0] = Addr(r32(c, V0)).off(4).lw(c);
                        next = Next::Step;
                    }
                    Next::Step => {
                        c.cycles += 2;
                        STEP_FRAME.call(c, 0x002d_bccc)?;
                        next = Next::StoreFrame;
                    }
                    Next::StoreFrame => {
                        c.cycles += 1;
                        Addr(this).off(0xc).sw(c, c.g[V0][0]);
                        next = Next::LoadAnim;
                    }
                    Next::LoadAnim => {
                        c.cycles += 1;
                        c.g[V1][0] = Addr(this).off(4).lw(c);
                        next = Next::Texture;
                    }
                    Next::Texture => break,
                }
            }
            c.cycles += 10;
            c.g[A1][0] = 0xd8;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5d38));
            c.mult(0, V1, V1, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(r32(c, A0)).off(0x98).lw(c);
            Addr(this).off(0x10).sw(c, c.g[V1][0]);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;


    #[test]
    fn c_crowd_anim2d_ctor_matches() {
        check(ssx3_ee::s09::f_002db9c8, c_crowd_anim2d_ctor, 200, &|c, seed| {
            let mut r = Rng(0x1004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let table = DATA + 0x500;
            c.bus.write32(GAME_GP + 0x2a74, table);
            c.bus.write32(table + 0x1c, r.next_u32());
            c.bus.write32(0x44_5dd0, r.next_u32());
        });
    }


    /// A crowd sprite at `DATA` and the animation tables: animations 0, 8
    /// and 9 (1..6 frames, textures `0x7e00_0000 | anim << 8 | frame`), and
    /// the transitions of types 0 and 1 (base index `20 + 4 * type`), with
    /// random next animation, mode and kind (0..3) per seed.
    pub(crate) fn crowd_anim_fixture(c: &mut Cpu, r: &mut Rng, kind: Option<u32>) {
        const ANIM_IDS: [u32; 3] = [0, 8, 9];
        for &a in &ANIM_IDS {
            let e = ANIMS + 0xd8 * a;
            c.bus.write32(e + 4, 1 + r.next_u32() % 6);
            for f in 0..16 {
                c.bus.write32(e + 0x98 + 4 * f, 0x7e00_0000 | a << 8 | f);
            }
        }
        for t in 0..2u32 {
            let base = 20 + 4 * t;
            c.bus.write32(TRANSITION_BASE + 4 * t, base);
            for &a in &ANIM_IDS {
                let e = TRANSITIONS + 12 * (base + a);
                c.bus.write32(e, ANIM_IDS[(r.next_u32() % 3) as usize]);
                c.bus.write32(e + 4, r.next_u32() % 2);
                c.bus.write32(e + 8, kind.unwrap_or(r.next_u32() % 4));
            }
        }
        let this = DATA;
        let anim = ANIM_IDS[(r.next_u32() % 3) as usize];
        let count = c.bus.read32(ANIMS + 0xd8 * anim + 4);
        c.bus.write32(this, r.next_u32() % 2);
        c.bus.write32(this + 4, anim);
        c.bus.write32(this + 8, r.next_u32() % 3);
        let frame = match r.next_u32() % 3 {
            0 => 0,
            1 => count - 1,
            _ => r.next_u32() % count,
        };
        c.bus.write32(this + 0xc, frame);
        c.bus.write32(this + 0x10, r.next_u32());
        c.bus.write32(this + 0x14, 3);
        c.g[A0][0] = DATA as u64;
    }


    #[test]
    fn c_crowd_anim2d_update_matches() {
        check_lohi(ssx3_ee::s09::f_002dbac0__generated, c_crowd_anim2d_update, 300, &|c, seed| {
            let mut r = Rng(0x2dbac0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let app = DATA + 0x800;
            c.bus.write32(GAME_GP + 0x2a74, app);
            crowd_anim_fixture(c, &mut r, None);
            // Interval 3: due (3 or more ticks since the last step) or not.
            let now = r.next_u32() >> 4;
            c.bus.write32(app + 0x1c, now);
            let since = if seed % 5 == 0 { r.next_u32() % 3 } else { 3 + r.next_u32() % 4 };
            c.bus.write32(DATA + 0x18, now.wrapping_sub(since));
        });
    }


    /// The sprite steps once per 3 frames at 60 Hz at any rate: 20 steps in
    /// one second of ticks (60, 120 or 240 updates).
    #[test]
    fn c_crowd_anim2d_update_follows_the_rate() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = Rng(0x2dbac1);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            crate::tick::set_for_test(c, hz);
            let app = c.bus.read32(GAME_GP + 0x2a74);
            crowd_anim_fixture(c, &mut r, Some(3));
            c.bus.write32(DATA + 0x18, 0);
            let mut steps = 0;
            for now in 1..=hz {
                c.bus.write32(app + 0x1c, now);
                c.g[A0][0] = DATA as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let last = c.bus.read32(DATA + 0x18);
                c_crowd_anim2d_update(c);
                assert_eq!(c.pc, crate::difftest::RA);
                if c.bus.read32(DATA + 0x18) != last {
                    steps += 1;
                }
            }
            assert_eq!(steps, 20, "{hz} Hz");
        }
    }
}
