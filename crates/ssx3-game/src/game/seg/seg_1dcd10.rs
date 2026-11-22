//! `src/seg/seg_1dcd10.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `FXLoader_ctor` (0x002dbd10): `cFXLoader::cFXLoader`: loads the particle (and unless `particles_only`, the effects) texture files, binds the 81 effect shapes to texture ids and fills the ten effect texture slots. `(self, particles_only)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): EAFile_LoadFileEx (pc 0x3de768) has no override slot and spins on test data.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn fx_loader_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2dbda4: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = 0x1;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S4][0] = 0x2;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            if c.g[A1][0] != 0 {
                c.g[S4][0] = c.g[V0][0];
            }
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[S4][0] == 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            if t {
                break 'b_2dbda4;
            } else {
                c.cycles += 3;
                c.g[S3][0] = 0x1;
                c.g[S2][0] = 0x480000;
                c.g[S1][0] = 0x480000;
                'l_2dbd60: loop {
                    'b_2dbd98: {
                        'b_2dbd94: {
                            'b_2dbd88: {
                                'b_2dbd78: {
                                    c.cycles += 2;
                                    if c.g[S0][0] == 0u64 {
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7918));
                                        break 'b_2dbd78;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == c.g[S3][0];
                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7938));
                                        if t {
                                            break 'b_2dbd88;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_2dbd98;
                                            } else {
                                                break 'b_2dbd78;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A1][0] = 0x100;
                                call_ptr(c, 0x003e18c8, 0x002dbd80)?; // call_indirect
                                let t = true;
                                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                                if t {
                                    break 'b_2dbd94;
                                } else {
                                    break 'b_2dbd88;
                                }
                            }
                            c.cycles += 3;
                            c.g[A1][0] = 0x100;
                            call_ptr(c, 0x003e18c8, 0x002dbd90)?; // call_indirect
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            break 'b_2dbd94;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        break 'b_2dbd98;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002dbd60);
                        }
                        continue 'l_2dbd60;
                    } else {
                        break 'b_2dbda4;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V0][0] = 0x490000;
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e50));
        c.g[S1][0] = 0u64;
        c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10b0));
        c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf50));
        c.g[S7][0] = 0xc;
        c.g[A1][0] = 0u64;
        'l_2dbdc8: loop {
            c.cycles += 4;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            'l_2dbdd8: loop {
                'b_2dbe04: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.mult(0, 2, 17, 23);
                    if t {
                        break 'b_2dbe04;
                    } else {
                        c.cycles += 8;
                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        call_ptr(c, 0x003b3900, 0x002dbdf8)?; // call_indirect
                        c.g[A1][0] = c.g[V0][0];
                        if c.g[A1][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x002dbdd8);
                            }
                            continue 'l_2dbdd8;
                        } else {
                            break 'b_2dbe04;
                        }
                    }
                }
                'b_2dbe44: {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_2dbe44;
                    } else {
                        c.cycles += 14;
                        c.mult(0, 6, 17, 23);
                        c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.g[V0][0] = 0x490000;
                        c.g[T0][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x10d8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e48));
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.g[A3][0] = c.lw(c.g[T2][0] as u32);
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002dbe44)?; // call_indirect
                        break 'b_2dbe44;
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[S1][0] = c.g[S3][0];
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V0][0] = ((c.g[S1][0] as i64) < 81i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002dbdc8);
                    }
                    continue 'l_2dbdc8;
                } else {
                    'b_2dbe88: {
                        c.cycles += 2;
                        let t = c.g[S4][0] == 0u64;
                        c.g[A0][0] = c.g[FP][0];
                        if t {
                            break 'b_2dbe88;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = c.g[S4][0];
                            c.g[S0][0] = c.g[SP][0];
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            'l_2dbe70: loop {
                                c.cycles += 5;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                call_ptr(c, 0x002523a8, 0x002dbe7c)?; // call_indirect
                                if c.g[S1][0] != 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002dbe70);
                                    }
                                    continue 'l_2dbe70;
                                } else {
                                    c.cycles += 1;
                                    c.g[A0][0] = c.g[FP][0];
                                    break 'b_2dbe88;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x11c));
                    c.g[A1][0] = 0x9;
                    'l_2dbe90: loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        let t = (c.g[A1][0] as i64) >= 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002dbe90);
                            }
                            continue 'l_2dbe90;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                            let t = c.g[RA][0] as u32;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                            c.pc = t;
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    })
}
