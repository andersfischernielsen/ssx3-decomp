//! Staging module oc1105 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1105`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_1a88, path_count_events, "PATH_CountEvents"),
    (0x003d_1f80, path_add_event_to_maps, "PATH_AddEventToMaps"),
    (0x003d_21c0, path_set_event_mark_bit, "PATH_SetEventMarkBit"),
    (
        0x003d_2218,
        path_clear_event_mark_bits,
        "PATH_ClearEventMarkBits",
    ),
    (0x003d_2fa0, path_nop_stub_wrapper, "PATH_NopStub_Wrapper"),
    (
        0x003d_2fc0,
        path_get_time_value_wrapper,
        "PATH_GetTimeValue_Wrapper",
    ),
];

/// `D_0044F420`: the current path map (`PathMap*`).
const CUR_MAP: u32 = 0x0044_f420;
/// `D_0044F42C`: byte set once the path system is initialised.
const INIT_FLAG: u32 = 0x0044_f42c;
/// `D_0044F430`: id of the selected map (0 unset, -1 none).
const CUR_ID: u32 = 0x0044_f430;
/// The path-map table `D_00517610` (four maps, 0x928 bytes each).
const MAPS: u32 = 0x0051_7610;

/// `PATH_SelectMap(idx)` (0x3d5450).
const SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::f_003d5450,
};
/// `PATH_RemoveEvent(ev, flags)` (0x3d1ba8).
const REMOVE_EVENT: Callee = Callee {
    addr: 0x003d_1ba8,
    f: ee::s14::f_003d1ba8,
};
/// `PATH_NopStub` (0x3d16c8).
const NOP_STUB: Callee = Callee {
    addr: 0x003d_16c8,
    f: ee::s14::f_003d16c8,
};
/// `PATH_GetTimeValue` (0x3d16d0).
const GET_TIME_VALUE: Callee = Callee {
    addr: 0x003d_16d0,
    f: ee::s14::f_003d16d0,
};

/// `void PATH_FlushEvents(int mask)` (0x3d1940): for each map that is loaded
/// and whose group bits match `mask`, clears the event slots of the current
/// player path (`PATH_RemoveEvent(slot, 2)`), then forgets the selection.
#[allow(dead_code)]
pub fn path_flush_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0x0045_0000;
        c.g[V0][0] = Addr(INIT_FLAG).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        let t = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'a5c: {
            'a58: {
                if t {
                    break 'a58;
                }
                c.cycles += 4;
                c.g[V1][0] = 0x0045_0000;
                c.g[V0][0] = Addr(CUR_ID).lw(c);
                let t = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x90);
                if t {
                    break 'a5c;
                }
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                SELECT_MAP.call(c, 0x003d_1994)?;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x0051_0000;
                if t {
                    break 'a58;
                }
                c.cycles += 9;
                c.g[A0][0] = 0;
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
                c.g[S3][0] = 0x0045_0000;
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4));
                c.g[S7][0] = 0x928;
                c.g[S6][0] = sx(0xf000_0000);
                c.g[S5][0] = 0x0f00_0000;
                c.mult(0, 5, 4, 23);
                'l19c0: loop {
                    'b1a40: {
                        'b1a3c: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                            let t = c.g[V1][0] == 0;
                            c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b1a3c;
                            }
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
                            c.g[V0][0] = c.g[V1][0] & c.g[S6][0];
                            c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[V1][0] & c.g[S5][0];
                            if t {
                                break 'b1a3c;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                            if c.g[V0][0] == 0 {
                                c.g[A0][0] = c.g[S1][0];
                                break 'b1a40;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[A0][0] & 0xff;
                            SELECT_MAP.call(c, 0x003d_19fc)?;
                            let t = c.g[V0][0] == 0;
                            c.g[A0][0] = c.g[S1][0];
                            if t {
                                break 'b1a40;
                            }
                            c.cycles += 3;
                            c.g[S0][0] = 0;
                            c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_sub(0xbe0)).lw(c);
                            'l1a10: loop {
                                'b1a2c: {
                                    c.cycles += 5;
                                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xdc)).lw(c);
                                    let t = c.g[A0][0] == 0;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b1a2c;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = 0x2;
                                    REMOVE_EVENT.call(c, 0x003d_1a2c)?;
                                }
                                c.cycles += 4;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
                                let t = c.g[V0][0] != 0;
                                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_sub(0xbe0)).lw(c);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d_1a10);
                                    }
                                    continue 'l1a10;
                                } else {
                                    break 'b1a3c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S1][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    if c.g[V0][0] != 0 {
                        c.mult(0, 5, 4, 23);
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_19c0);
                        }
                        continue 'l19c0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x0045_0000;
                        Addr((c.g[S3][0] as u32).wrapping_sub(0xbe0)).sw(c, 0);
                        Addr((c.g[V0][0] as u32).wrapping_sub(0xbd0)).sw(c, 0);
                        break 'a58;
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
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

/// `int PATH_CountEvents(int mask)` (0x3d1a88): -18 if the path system is not
/// ready or no map is selected; otherwise the number of live event slots of
/// the selected player path in the maps whose group bits match `mask`, after
/// clearing the selection. Unlike `PATH_FlushEvents` this does not remove
/// anything: it only counts the slots of the first matching loaded map.
pub fn path_count_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        'b1b80: {
            'b1ae8: {
                'b1ae0: {
                    c.g[V0][0] = 0x0045_0000;
                    c.g[V1][0] = Addr(INIT_FLAG).lbu(c);
                    frame.save(c, S2, 0x50);
                    frame.save(c, S3, 0x40);
                    c.g[S2][0] = 0;
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S0, 0x70);
                    frame.save(c, S1, 0x60);
                    frame.save(c, S4, 0x30);
                    frame.save(c, S5, 0x20);
                    frame.save(c, S6, 0x10);
                    let t = c.g[V1][0] == 0;
                    frame.save_ra(c, 0);
                    if t {
                        break 'b1ae0;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = 0x0045_0000;
                    c.g[V0][0] = Addr(CUR_ID).lw(c);
                    let t = c.g[V0][0] != 0;
                    c.g[S6][0] = c.g[V1][0];
                    if t {
                        break 'b1ae0;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = u64::MAX;
                    SELECT_MAP.call(c, 0x003d_1ad8)?;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x0051_0000;
                    if t {
                        break 'b1ae8;
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = u64::MAX - 17;
                break 'b1b80;
            }
            c.cycles += 8;
            c.g[V1][0] = 0x0f00_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
            c.g[S5][0] = 0x0045_0000;
            c.g[S4][0] = c.g[S5][0];
            c.g[S3][0] = c.g[S3][0] & c.g[V1][0];
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = 0;
            'l1b08: loop {
                'b1b78: {
                    'b1b68: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            break 'b1b68;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        c.g[V0][0] &= c.g[S3][0];
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            break 'b1b68;
                        }
                        'b1b64: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003d_1b2c);
                            c.g[A0][0] = c.g[S0][0] & 0xff;
                            SELECT_MAP.call(c, 0x003d_1b2c)?;
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_sub(0xbe0)).lw(c);
                            if t {
                                break 'b1b64;
                            }
                            c.cycles += 3;
                            c.g[A1][0] = 0xf;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xdc));
                            'l1b40: loop {
                                c.cycles += 7;
                                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[A1][0] as i64) >= 0;
                                if c.g[V1][0] != 0 {
                                    c.g[S2][0] = c.g[V0][0];
                                }
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d_1b40);
                                    }
                                    continue 'l1b40;
                                } else {
                                    c.cycles += 2;
                                    Addr((c.g[S6][0] as u32).wrapping_sub(0xbd0)).sw(c, 0);
                                    break 'b1b78;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x928));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_1b08);
                        }
                        continue 'l1b08;
                    } else {
                        c.cycles += 1;
                        Addr((c.g[S6][0] as u32).wrapping_sub(0xbd0)).sw(c, 0);
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.g[S2][0];
                Addr((c.g[S5][0] as u32).wrapping_sub(0xbe0)).sw(c, 0);
                break 'b1b80;
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void PATH_AddEventToMaps(int mask, PathEvent* ev)` (0x3d1f80): for each
/// loaded map whose group bits contain `mask`'s group, appends `ev` to that
/// map's slot list (up to 16) with the current time stamped into its first two
/// words. Returns 0, -8 if no map matched, or -13 when a map's list is full.
pub fn path_add_event_to_maps(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = 0x0051_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7610));
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save(c, S0, 0x80);
        c.g[V0][0] = 0xffff_ffff_ff00_0000;
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0] & c.g[V0][0];
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x91c));
        c.g[S5][0] = 0;
        c.g[S6][0] = u64::MAX - 7;
        c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add(0xdc));
        frame.save_ra(c, 0);
        c.g[S4][0] = 0;
        'l1fd0: loop {
            'b203c: {
                'b2028: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x8e8)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        break 'b2028;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_sub(0x8ec)).lw(c);
                    c.g[V0][0] &= c.g[S3][0];
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        break 'b2028;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = u64::MAX - 12;
                    if t {
                        break 'b203c;
                    }
                    c.cycles += 11;
                    c.g[S6][0] = 0;
                    call_ptr(c, 0x002a_f370, 0x003d_2004)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    let now = c.g[V0][0];
                    Addr(c.g[S2][0] as u32).sw(c, now);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    Addr((c.g[S2][0] as u32).wrapping_add(0x8)).sw(c, now);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    let entry = c.g[S2][0];
                    Addr(c.g[V1][0] as u32).sw(c, entry);
                    let count = c.g[S0][0];
                    Addr(c.g[S1][0] as u32).sw(c, count);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x928));
                c.g[V0][0] = ((c.g[S5][0] as i64) < 4i64) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x928));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_1fd0);
                    }
                    continue 'l1fd0;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S6][0];
                    break 'b203c;
                }
            }
            break;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void PATH_SetEventMarkBit(PathEvent* ev, int disable)` (0x3d21c0): sets
/// bit 29 of the current path's event definition word (`eventDef+0xc`) to
/// `disable & 1`. The definition is the map's table at `+0x34` plus the
/// halfword at `+0x40 + ev->index * 2`, scaled by 4.
pub fn path_set_event_mark_bit(c: &mut Cpu) {
    'b2210: {
        c.cycles += 13;
        c.g[V0][0] = 0x0045_0000;
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lbu(c);
        c.g[A2][0] = Addr(CUR_MAP).lw(c);
        c.g[A3][0] = c.g[A1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let t = c.g[A1][0] == 0;
        c.g[A0][0] = c.g[A3][0] & 0x1;
        if t {
            break 'b2210;
        }
        c.cycles += 7;
        let def = Addr(c.g[A1][0] as u32).off(0xc);
        c.g[V0][0] = def.lw(c);
        c.g[V1][0] = !0x2000_0000u64;
        c.g[V0][0] &= c.g[V1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 29);
        c.g[V0][0] |= c.g[A0][0];
        let v = c.g[V0][0];
        def.sw(c, v);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void PATH_ClearEventMarkBits(u32 mask)` (0x3d2218): clears bit 29 of
/// the event definition word of every slot in the current path's list.
pub fn path_clear_event_mark_bits(c: &mut Cpu) {
    'b2284: {
        c.cycles += 6;
        c.g[V0][0] = 0x0045_0000;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0xbe0)).lw(c);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xf)).lbu(c);
        let t = c.g[V0][0] == 0;
        c.g[A2][0] = 0;
        if t {
            break 'b2284;
        }
        c.cycles += 4;
        c.g[T0][0] = !0x2000_0000u64;
        c.g[A3][0] = c.g[A0][0];
        c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x40)).lw(c);
        'l2240: loop {
            'b2270: {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
                c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x34)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if c.g[A1][0] == 0 {
                    c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x34)).lw(c);
                    break 'b2270;
                }
                c.cycles += 4;
                let def = Addr(c.g[A1][0] as u32).off(0xc);
                c.g[V0][0] = def.lw(c);
                c.g[V0][0] &= c.g[T0][0];
                let v = c.g[V0][0];
                def.sw(c, v);
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x34)).lw(c);
            }
            c.cycles += 5;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xf)).lbu(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x40)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_2240);
                }
                continue 'l2240;
            } else {
                break 'b2284;
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `PATH_NopStub_Wrapper` (0x3d2fa0): `PATH_NopStub()`.
pub fn path_nop_stub_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(NOP_STUB), 0x003d_2fb0)
}

/// `PATH_GetTimeValue_Wrapper` (0x3d2fc0): `PATH_GetTimeValue()`.
pub fn path_get_time_value_wrapper(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GET_TIME_VALUE), 0x003d_2fd0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    const SEEDS: u32 = 200;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt)
    }

    /// Build a path player at `DATA + 0x10000` with random event pointers
    /// and four maps at MAPS. Map flags and loaded flags vary by seed.
    fn path_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        scramble(c, r);
        c.bus.write8(INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
        c.bus
            .write32(CUR_ID, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
        let player = DATA + 0x1_0000;
        c.bus.write32(CUR_MAP, player);
        for k in 0..16 {
            let ev = if seed % 2 == 0 {
                0
            } else {
                DATA + 0x2_0000 + k * 0x100
            };
            c.bus.write32(player + 0xdc + 4 * k, ev);
        }
        for i in 0..4 {
            let m = MAPS + i * 0x928;
            c.bus.write32(m + 0x30, r.next_u32());
            c.bus
                .write32(m + 0x34, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(m + 0x91c, r.next_u32() % 16);
        }
    }

    #[test]
    fn path_count_events_matches() {
        check(
            ee::s14::f_003d1a88__generated,
            path_count_events,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 2);
                path_setup(c, seed, &mut r);
                c.g[A0][0] = match seed % 4 {
                    0 => 0xf000_0000,
                    1 => 0x0f00_0000,
                    2 => 0,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn path_add_event_to_maps_matches() {
        check(
            ee::s14::f_003d1f80__generated,
            path_add_event_to_maps,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 3);
                scramble(c, &mut r);
                for i in 0..4 {
                    let m = MAPS + i * 0x928;
                    c.bus.write32(m + 0x34, if seed % 3 == 0 { 0 } else { 1 });
                    c.bus.write32(m + 0x30, r.next_u32() | 0xf000_0000);
                    c.bus.write32(m + 0x91c, r.next_u32() % 18);
                }
                c.g[A0][0] = match seed % 3 {
                    0 => 0xf000_0000,
                    1 => 0x0f00_0000,
                    _ => r.next_u32() as u64,
                };
                c.g[A1][0] = sx(DATA + 0x4_0000);
            },
        );
    }

    #[test]
    fn path_set_event_mark_bit_matches() {
        check(
            ee::s14::f_003d21c0__generated,
            path_set_event_mark_bit,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 4);
                scramble(c, &mut r);
                let player = if seed % 5 == 0 {
                    0x0001_0000
                } else {
                    DATA + 0x1_0000
                };
                c.bus.write32(CUR_MAP, player);
                c.bus.write32(player + 0x40, DATA + 0x2_0000);
                let base = if seed % 5 == 0 {
                    0x0001_0000
                } else {
                    DATA + 0x3_0000
                };
                c.bus.write32(player + 0x34, base);
                let idx = (r.next_u32() % 16) as u32;
                // Halfword for event `idx`: a negative value that lands on
                // a null definition for the base 0x10000.
                let hw = if seed % 5 == 0 {
                    0xc000u16
                } else {
                    (r.next_u32() & 0xff) as u16
                };
                c.bus.write16(DATA + 0x2_0000 + idx * 2, hw);
                c.bus.write8(DATA + 0x5_0000 + 0xc, idx as u8);
                let def = base
                    .wrapping_add(((hw as i16 as i32) * 4) as u32)
                    .wrapping_add(0xc);
                c.bus.write32(def, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x5_0000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn path_clear_event_mark_bits_matches() {
        check(
            ee::s14::f_003d2218__generated,
            path_clear_event_mark_bits,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 5);
                scramble(c, &mut r);
                let base = if seed % 5 == 0 {
                    0x0001_0000
                } else {
                    DATA + 0x3_0000
                };
                let player = DATA + 0x1_0000;
                c.bus.write32(CUR_MAP, player);
                c.bus.write32(player + 0x34, base);
                c.bus.write32(player + 0x40, DATA + 0x2_0000);
                c.bus.write8(base + 0xf, (seed % 17) as u8);
                for k in 0..16 {
                    // Slot 3 of the null-base seeds lands on address 0.
                    let hw = if seed % 5 == 0 && k == 3 {
                        0xc000u16
                    } else {
                        (k as u16) % 24
                    };
                    c.bus.write16(DATA + 0x2_0000 + 2 * k, hw);
                    c.bus.write32(base + 4 * k + 0xc, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn path_nop_stub_wrapper_matches() {
        check(
            ee::s14::f_003d2fa0__generated,
            path_nop_stub_wrapper,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 6);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn path_get_time_value_wrapper_matches() {
        check(
            ee::s14::f_003d2fc0__generated,
            path_get_time_value_wrapper,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 7);
                scramble(c, &mut r);
            },
        );
    }
}
