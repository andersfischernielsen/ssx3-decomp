//! `src/fe/ovstates5.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_3990,
        event_select_load_map_image,
        "EventSelect_LoadMapImage",
    ),
    (
        0x0020_3cb8,
        event_select_set_map_tab_mode,
        "EventSelect_SetMapTabMode",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
}

/// `void EventSelect_SetMapTabMode(EventSelect* self, int mode)` (0x203cb8):
/// store the map-tab mode and label the tab from the new mode.
pub fn event_select_set_map_tab_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = 0x470000;
        let this = c.g[S0][0];
        c.sw(this as u32, c.g[A1][0]);
        c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xab8));
        call_ptr(c, callee::STR_HASH, 0x0020_3cdc)?;
        c.g[A0][0] = lw_at(c, this, 0x2e8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_3ce8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = lw_at(c, this, 0);
        let mode_zero = c.g[V0][0] == 0;
        c.g[A0][0] = 0x470000;
        if mode_zero {
            c.cycles += 6;
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xad8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d20)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d2c)?;
            c.lq(16, at(sp0, 0x20));
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xac8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d04)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d10)?;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void EventSelect_LoadMapImage(EventSelect *self)` (0x203990): request the
/// selected event's map texture and latch which variant was loaded.
pub fn event_select_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        'b_203a44: {
            'b_203a40: {
                'b_2039f0: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                    if t {
                        break 'b_2039f0;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d4));
                        c.g[S2][0] = 0x1;
                        if c.g[V0][0] == c.g[S2][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_203a44;
                        } else {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x002039d4);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a38c0, 0x002039d4)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x002039e4);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x001a36e8, 0x002039e4)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), c.g[S2][0]);
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), 0u64);
                            if t {
                                break 'b_203a40;
                            } else {
                                break 'b_2039f0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                if t {
                    break 'b_203a40;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    if t {
                        break 'b_203a40;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        if c.g[V1][0] == c.g[V0][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_203a44;
                        } else {
                            c.cycles += 12;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x00203a1c);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a38c0, 0x00203a1c)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x00203a30);
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x001a3768, 0x00203a30)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
                            break 'b_203a40;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_203a44;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::DATA;

    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;

    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    use ssx3_ee as ee;

    #[test]
    fn event_select_set_map_tab_mode_matches() {
        check(
            ee::s05::f_00203cb8__generated,
            event_select_set_map_tab_mode,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x203cb8, 4);
                let screen = DATA + 0x1000;
                let node = screen_with(c, &mut r, screen, 0);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus
                    .write32(DATA + 0x2e8, if seed % 4 == 0 { 0 } else { screen });
                let _ = node;
            },
        );
    }

    #[test]
    fn event_select_load_map_image_matches() {
        strict(
            ee::s05::f_00203990__generated,
            event_select_load_map_image,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2039);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(gp_at(-0xb54), [-1i32, 0, 2, 3][(seed % 4) as usize] as u32);
                c.bus.write32(gp_at(-0xb4c), (seed % 5) as u32);
                c.bus.write32(DATA + 0x2d4, (seed % 7) as u32);
                c.bus.write32(DATA + 0x2d8, seed as u32);
                c.bus.write32(DATA + 0x2dc, 0);
                c.bus.write32(W + 0x11c, DATA + 0x3000);
            },
        );
    }
}
