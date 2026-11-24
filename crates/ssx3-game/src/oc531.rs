//! Staging module oc531 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc531`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_0ae0, avalanche_menu_ctor, "AvalancheMenu_dtor"),
    (
        0x0026_b680,
        path_data_find_nearest_marker,
        "PathData_FindNearestMarker",
    ),
    (
        0x0026_baa8,
        path_data_debug_printf_stub,
        "PathData_DebugPrintfStub",
    ),
    (0x0026_c438, path_data_global_ctors, "PathData_GlobalCtors"),
    (0x0026_c588, playback_man_dtor, "PlaybackMan_dtor"),
    (
        0x0026_cbb0,
        playback_man_clear_world_state_saved,
        "PlaybackMan_ClearWorldStateSaved",
    ),
    (
        0x0026_cd98,
        playback_man_start_recording,
        "PlaybackMan_StartRecording",
    ),
    (
        0x0026_cdd0,
        playback_man_stop_recording,
        "PlaybackMan_StopRecording",
    ),
    (0x0026_cfe8, playback_man_find_file, "PlaybackMan_FindFile"),
    (
        0x0026_d020,
        playback_man_apply_file,
        "PlaybackMan_ApplyFile",
    ),
    (
        0x0026_d558,
        replay_cache_deserialize,
        "ReplayCache_Deserialize",
    ),
];

/// Offsets (from the menu object) of the 34 `cMenuItem` members, in the order
/// the constructor builds them.
pub(crate) const AVALANCHE_MENU_ITEM_OFFSETS: [u32; 34] = [
    0x77c, 0x748, 0x714, 0x6e0, 0x6b0, 0x67c, 0x648, 0x614, 0x5e0, 0x5ac, 0x578, 0x544, 0x514,
    0x4e0, 0x4ac, 0x478, 0x444, 0x410, 0x3dc, 0x3ac, 0x37c, 0x34c, 0x31c, 0x2e8, 0x2b8, 0x288,
    0x258, 0x228, 0x1f8, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
];

/// `cMenuItem::cMenuItem(this, 2)` (0x2ca280).
pub(crate) const MENU_ITEM_CTOR: u32 = 0x002c_a280;
/// `cMenu::cMenu(this, flags)` (0x2caa80).
pub(crate) const MENU_CTOR: u32 = 0x002c_aa80;

/// `cAvalancheMenu::cAvalancheMenu(this, flags)` (0x250ae0). The constructor
/// (the list calls it a destructor) builds the menu's 34 `cMenuItem`s (flag 2)
/// at their fixed offsets, then runs the base `cMenu` constructor on the whole
/// object. Item `i` returns to `0x250b04 + 12 * i`; the base constructor returns
/// to `0x250c9c`. A callee that unwinds returns at once, as the compiled code
/// does.
pub fn avalanche_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (i, &off) in AVALANCHE_MENU_ITEM_OFFSETS.iter().enumerate() {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
            c.g[A1][0] = 2;
            call_ptr(c, MENU_ITEM_CTOR, 0x0025_0b04 + 12 * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, MENU_CTOR, 0x0025_0c9c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Fetch FPU register `r` as the value the FPU holds.
#[inline(always)]
fn find_marker_fpr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `int PathData_FindNearestMarker(PathTable* list, Vec3* pos)` (0x26b680): the
/// id of the nearest entry of type 2 with a non-zero id that
/// `BEMissionInt_SessionTypeAtLeast` (`cBEMissionInterface_isSessionPoint`)
/// accepts in the current section, or -1. Entries are 0x28 bytes: the id at
/// +0, the type at +4, the position at +8. A strictly nearer entry replaces the
/// best one, which starts at the game constant `gp-0x4a3c`.
pub fn path_data_find_nearest_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_fpr(c, 20, 0x80);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = 0;
        c.g[S3][0] = c.g[A1][0];
        let best = Addr((c.g[GP][0] as u32).wrapping_sub(0x4a3c)).read_fpu(c);
        abi::set_fpr(c, 20, best);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        'search: {
            if (c.g[V0][0] as i64) <= 0 {
                break 'search;
            }
            c.cycles += 2;
            c.g[S5][0] = 2;
            c.g[S1][0] = 0;
            loop {
                'entry: {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                    if c.g[V1][0] != c.g[S5][0] {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        break 'entry;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        break 'entry;
                    }
                    c.cycles += 16;
                    call_ptr(c, 0x0014_dc80, 0x0026_b6f0)?;
                    c.g[A1][0] = 0xa;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dd58, 0x0026_b6fc)?;
                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x78)).lw(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A2][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x1bc)).lw(c);
                    call_ptr(c, 0x0015_4630, 0x0026_b720)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        break 'entry;
                    }
                    c.cycles += 25;
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                    let p3 = Addr(c.g[S3][0] as u32).read_fpu(c);
                    abi::set_fpr(c, 3, p3);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let p5 = Addr((c.g[S3][0] as u32).wrapping_add(4)).read_fpu(c);
                    abi::set_fpr(c, 5, p5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
                    let p4 = Addr((c.g[S3][0] as u32).wrapping_add(8)).read_fpu(c);
                    abi::set_fpr(c, 4, p4);
                    let pos = c.g[V0][0] as u32;
                    let e0 = Addr(pos).read_fpu(c);
                    abi::set_fpr(c, 0, e0);
                    let e1 = Addr(pos.wrapping_add(4)).read_fpu(c);
                    abi::set_fpr(c, 1, e1);
                    let e2 = Addr(pos.wrapping_add(8)).read_fpu(c);
                    abi::set_fpr(c, 2, e2);
                    // difference = entry position - *pos, then the squared length
                    let dx = find_marker_fpr(c, 0) - find_marker_fpr(c, 3);
                    let dy = find_marker_fpr(c, 1) - find_marker_fpr(c, 5);
                    let dz = find_marker_fpr(c, 2) - find_marker_fpr(c, 4);
                    abi::set_fpr(c, 0, dx);
                    abi::set_fpr(c, 1, dy);
                    abi::set_fpr(c, 2, dz);
                    abi::set_fpr(c, 3, dx * dx);
                    Addr(frame.at(c, 0)).write_fpu(c, dx);
                    abi::set_fpr(c, 4, dy * dy);
                    Addr(frame.at(c, 4)).write_fpu(c, dy);
                    abi::set_fpr(c, 0, dz * dz);
                    Addr(frame.at(c, 8)).write_fpu(c, dz);
                    let sum = find_marker_fpr(c, 3) + find_marker_fpr(c, 4);
                    let sum = sum + find_marker_fpr(c, 0);
                    abi::set_fpr(c, 3, sum);
                    let root = sum.sqrt();
                    abi::set_fpr(c, 0, root);
                    let best_so_far = find_marker_fpr(c, 20);
                    let nearer = abi::c_lt_s(c, root, best_so_far);
                    let entry = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = entry;
                    if nearer {
                        c.cycles += 2;
                        abi::set_fpr(c, 20, root);
                        c.g[S4][0] = Addr(entry as u32).lw(c);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                }
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
                if !more {
                    break 'search;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b6c8);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `PathData_DebugPrintfStub` (0x26baa8): a debug printf that does nothing
/// but spill its argument registers (a1..a3, t0..t3 as 64-bit words, f12..f18
/// as 32-bit words) to its frame, then return with `v0 = a0`.
pub fn path_data_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    for (i, reg) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
        let slot = frame.at(c, 0x48 + 8 * i as u32);
        let v = c.g[reg][0];
        c.sd(slot, v);
    }
    for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
        frame.save_fpr(c, f, 0x38 + 4 * i as u32);
    }
    frame.close(c);
}

/// `PlaybackMan::ClearWorldStateSaved(this)` (0x26cbb0): clears the word at
/// +0x10 (the saved-world-state flag).
pub fn playback_man_clear_world_state_saved(c: &mut Cpu) {
    c.cycles += 2;
    Addr((c.g[A0][0] as u32).wrapping_add(0x10)).sw(c, 0);
    abi::ret(c);
}

/// `PlaybackMan::StopRecording(this)` (0x26cdd0): restores the world state
/// (`PlaybackMan_RestoreWorldState`, 0x26cc18), then clears the word at +0x70.
pub fn playback_man_stop_recording(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_cc18, 0x0026_cde4)?;
        Addr((c.g[S0][0] as u32).wrapping_add(0x70)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan::StartRecording(this)` (0x26cd98): saves the world state
/// (`PlaybackMan_SaveWorldState`, 0x26cbb8), clears the word at `*(this+4)+8`
/// and sets the mode word at +0x70 to 2.
pub fn playback_man_start_recording(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_cbb8, 0x0026_cdac)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = 2;
        frame.restore_ra(c, 0);
        Addr((c.g[V0][0] as u32).wrapping_add(8)).sw(c, 0);
        let mode = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x70)).sw(c, mode);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan::FindFile(this, key)` (0x26cfe8): scans the file table (count
/// at +0x50, entries of 12 bytes at +0x58, key in the entry's first word) and
/// returns the index of the first entry whose key matches, or -1.
pub fn playback_man_find_file(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
    let empty = (c.g[A2][0] as i64) <= 0;
    c.g[V1][0] = 0;
    'scan: {
        if empty {
            break 'scan;
        }
        c.cycles += 1;
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            if c.g[V0][0] != c.g[A1][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if !more {
                    break 'scan;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_cff8);
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    abi::ret(c);
}

/// `PathData_GlobalCtors` (0x26c438): the static-constructor stub, which runs
/// `PathData_StaticInit(1, 0xffff)` (0x26bae8).
pub fn path_data_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x0026_bae8, 0x0026_c44c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan::IsFileLoaded(this, key)` (0x26c998): finds `key` with
/// `PlaybackMan_FindFile` (0x26cfe8). Not found: 0 (after the `+0x78` word
/// is not consulted). Found: 1 when the entry's word at +4 is set; otherwise 0
/// unless the table's word at +0x78 is negative.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn playback_man_is_file_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_cfe8, 0x0026_c9ac)?;
        c.g[V1][0] = c.g[V0][0];
        if (c.g[V1][0] as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x58)).lw(c);
            c.mult(0, 2, 3, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
            c.g[V0][0] = 1;
            if c.g[V1][0] != 0 {
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x78)).lw(c);
                if (c.g[V1][0] as i64) >= 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan::~PlaybackMan(this, flags)` (0x26c588): frees the buffer at
/// `*this` (0x3168e8, size class 3), the two handles at +0x6c and +0x68
/// (0x317e50), the data (`PlaybackMan_FreeData`), clears the pointer at
/// +0x14 and, when `flags` bit 0 is set, frees `this` (0x317e50).
pub fn playback_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0031_68e8, 0x0026_c5b0)?;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x6c)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0026_c5b8)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x68)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0026_c5c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0026_c898, 0x0026_c5c8)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1898));
        let free_this = c.g[V1][0] != 0;
        let cleared = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x14)).sw(c, cleared);
        if free_this {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0026_c5e4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan_ApplyFile(this, index)` (0x26d020): the table entry at
/// `index` (12-byte entries at `this+0x58`). An entry already loaded (its word
/// at +4 set) is handed to 0x2c85a0 and the call returns 1. Otherwise the
/// stream at `this+4` is primed (0x3e6448) and the entry's two words are
/// written; the call returns 0.
pub fn playback_man_apply_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xc;
        frame.save(c, S0, 0x10);
        c.mult(0, 5, 5, 3);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(4)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
            c.cycles += 4;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002c_85a0, 0x0026_d08c)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 14;
            c.g[S0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
            c.g[A2][0] = 0x10000;
            c.g[A1][0] = 0;
            c.g[A2][0] |= 0xc22c;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6448, 0x0026_d060)?;
            c.g[V0][0] = 0x10000;
            c.g[V1][0] = 0xffff_ffff_dbac_0000;
            c.g[V0][0] |= 0xc200;
            c.g[V1][0] |= 0x101;
            let word1 = c.g[V0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, word1);
            c.g[V0][0] = 0;
            let word0 = c.g[V1][0];
            Addr(c.g[S0][0] as u32).sw(c, word0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ReplayCache_Deserialize(cache, stream)` (0x26d558): reads two words from
/// the stream (0x3e6574, 8 bytes then 4), stores the stream position back,
/// copies the second word into `cache[0]`, then reads a record with 0x2c85a0
/// and returns 1 with the stream position advanced by the record's size.
pub fn replay_cache_deserialize(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        call_ptr(c, 0x003e_6574, 0x0026_d580)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        c.g[V1][0] = Addr(frame.at(c, 4)).lw(c);
        c.g[A2][0] = 4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        let position = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, position);
        let first = c.g[V1][0];
        Addr(c.g[S1][0] as u32).sw(c, first);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        call_ptr(c, 0x003e_6574, 0x0026_d5a4)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        let advanced = c.g[A0][0];
        Addr(c.g[S0][0] as u32).sw(c, advanced);
        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_d5bc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = 1;
        c.g[A0][0] = Addr(frame.at(c, 0x10)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let end = c.g[V1][0];
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        Addr(c.g[S0][0] as u32).sw(c, end);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the menu constructor's callee fakes (test machines only).
    pub(crate) const AVALANCHE_MENU_GATE: u32 = DATA + 0x200000;
    static OC531_PREV_MENU_ITEM: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_MENU: AtomicUsize = AtomicUsize::new(0);

    fn oc531_fake_menu_item(c: &mut Cpu) {
        if c.bus.read32(AVALANCHE_MENU_GATE) != 0x0531_0001 {
            return fakes::fall_back(c, &OC531_PREV_MENU_ITEM, ee::s09::f_002ca280__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, MENU_ITEM_CTOR, ra);
    }

    fn oc531_fake_menu(c: &mut Cpu) {
        if c.bus.read32(AVALANCHE_MENU_GATE) != 0x0531_0001 {
            return fakes::fall_back(c, &OC531_PREV_MENU, ee::s09::f_002caa80__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, MENU_CTOR, ra);
    }

    #[test]
    fn avalanche_menu_ctor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc531_fake_menu_item,
            &OC531_PREV_MENU_ITEM,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            oc531_fake_menu,
            &OC531_PREV_MENU,
        );
        check(ee::s06::f_00250ae0__generated, avalanche_menu_ctor, 200, &|c, seed| {
            let mut r = Rng(0x0025_0ae0 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(AVALANCHE_MENU_GATE, 0x0531_0001);
            c.g[A0][0] = sx(DATA + 0x1000 * (seed % 4));
            c.g[A1][0] = r.next_u32() as u64;
            c.pc = 0x0025_0ae0;
        });
    }

    /// Gate word for the nearest-marker search's callee fakes.
    pub(crate) const PATH_FIND_NEAREST_GATE: u32 = DATA + 0x200000;
    /// The `PathTable` the search walks (count at +0, entries at +4).
    pub(crate) const PATH_FIND_NEAREST_LIST: u32 = DATA + 0x10000;
    static OC531_PREV_GET_BE: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_GET_IF: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_SESSION: AtomicUsize = AtomicUsize::new(0);

    fn oc531_fake_get_be(c: &mut Cpu) {
        if c.bus.read32(PATH_FIND_NEAREST_GATE) != 0x0531_0002 {
            return fakes::fall_back(c, &OC531_PREV_GET_BE, ee::s01::f_0014dc80__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0014_dc80, ra);
    }

    fn oc531_fake_get_if(c: &mut Cpu) {
        if c.bus.read32(PATH_FIND_NEAREST_GATE) != 0x0531_0002 {
            return fakes::fall_back(c, &OC531_PREV_GET_IF, ee::s01::f_0014dd58__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0014_dd58, ra);
    }

    /// `isSessionPoint`: zero or non-zero from the entry id and a seed word.
    fn oc531_fake_session(c: &mut Cpu) {
        if c.bus.read32(PATH_FIND_NEAREST_GATE) != 0x0531_0002 {
            return fakes::fall_back(c, &OC531_PREV_SESSION, ee::s01::f_00154630__generated);
        }
        let id = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0015_4630, ra);
        let salt = c.bus.read32(PATH_FIND_NEAREST_GATE + 4);
        c.g[V0][0] = if (id ^ salt) % 3 == 0 { 0 } else { 1 };
    }

    #[test]
    fn path_data_find_nearest_marker_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc531_fake_get_be,
            &OC531_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc531_fake_get_if,
            &OC531_PREV_GET_IF,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00154630),
            oc531_fake_session,
            &OC531_PREV_SESSION,
        );
        check(
            ee::s07::f_0026b680__generated,
            path_data_find_nearest_marker,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0026_b680, 4);
                c.bus.write32(PATH_FIND_NEAREST_GATE, 0x0531_0002);
                c.bus.write32(PATH_FIND_NEAREST_GATE + 4, r.next_u32());
                // The world pointer chain the search reads: gp-0x848 -> +0x84
                // -> +0x78 -> +0x1bc.
                let world = DATA + 0x1000;
                let section = DATA + 0x2000;
                let mission = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, section);
                c.bus.write32(section + 0x78, mission);
                c.bus.write32(mission + 0x1bc, r.next_u32() % 9);
                let best = if seed % 4 == 0 {
                    f32::MAX.to_bits()
                } else {
                    r.f32(40.0)
                };
                c.bus.write32(GAME_GP - 0x4a3c, best);
                let list = PATH_FIND_NEAREST_LIST;
                let entries = list + 0x100;
                let count = seed % 5;
                c.bus.write32(list, count);
                c.bus.write32(list + 4, entries);
                for i in 0..count {
                    let e = entries + 0x28 * i;
                    let id = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 7
                    };
                    c.bus.write32(e, id);
                    c.bus
                        .write32(e + 4, if r.next_u32() % 4 == 0 { 1 } else { 2 });
                    for k in 0..3 {
                        c.bus.write32(e + 8 + 4 * k, r.f32(40.0));
                    }
                }
                let pos = DATA + 0x20000;
                for k in 0..3 {
                    c.bus.write32(pos + 4 * k, r.f32(40.0));
                }
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(pos);
                c.pc = 0x0026_b680;
            },
        );
    }

    #[test]
    fn path_data_debug_printf_stub_matches() {
        check(
            ee::s07::f_0026baa8__generated,
            path_data_debug_printf_stub,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_baa8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_baa8;
            },
        );
    }

    #[test]
    fn playback_man_clear_world_state_saved_matches() {
        check(
            ee::s07::f_0026cbb0__generated,
            playback_man_clear_world_state_saved,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_cbb0 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x1000 * (seed % 3);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, r.next_u32());
                c.pc = 0x0026_cbb0;
            },
        );
    }

    /// Gate word for the world-state fakes of the recording functions.
    pub(crate) const PLAYBACK_RECORDING_GATE: u32 = DATA + 0x200000;
    static OC531_PREV_RESTORE_WS: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_SAVE_WS: AtomicUsize = AtomicUsize::new(0);

    fn oc531_fake_restore_ws(c: &mut Cpu) {
        if c.bus.read32(PLAYBACK_RECORDING_GATE) != 0x0531_0003 {
            return fakes::fall_back(c, &OC531_PREV_RESTORE_WS, ee::s07::f_0026cc18__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0026_cc18, ra);
    }

    fn oc531_fake_save_ws(c: &mut Cpu) {
        if c.bus.read32(PLAYBACK_RECORDING_GATE) != 0x0531_0003 {
            return fakes::fall_back(c, &OC531_PREV_SAVE_WS, ee::s07::f_0026cbb8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0026_cbb8, ra);
    }

    /// The playback object: the `+4` pointer and the word at `+0x70` the
    /// recording functions write.
    fn oc531_playback_fixture(c: &mut Cpu, seed: u32, r: &mut Rng) {
        let this = DATA + 0x1000;
        c.bus.write32(this + 4, DATA + 0x2000);
        c.bus.write32(this + 0x70, r.next_u32());
        c.bus.write32(DATA + 0x2000 + 8, r.next_u32());
        c.bus.write32(PLAYBACK_RECORDING_GATE, 0x0531_0003);
        c.g[A0][0] = sx(this + 0x100 * (seed % 2));
    }

    #[test]
    fn playback_man_stop_recording_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026CC18),
            oc531_fake_restore_ws,
            &OC531_PREV_RESTORE_WS,
        );
        check(
            ee::s07::f_0026cdd0__generated,
            playback_man_stop_recording,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_cdd0 ^ seed as u64);
                scramble(c, &mut r);
                oc531_playback_fixture(c, seed, &mut r);
                c.pc = 0x0026_cdd0;
            },
        );
    }

    #[test]
    fn playback_man_start_recording_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026CBB8),
            oc531_fake_save_ws,
            &OC531_PREV_SAVE_WS,
        );
        check(
            ee::s07::f_0026cd98__generated,
            playback_man_start_recording,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_cd98 ^ seed as u64);
                scramble(c, &mut r);
                oc531_playback_fixture(c, seed, &mut r);
                c.pc = 0x0026_cd98;
            },
        );
    }

    #[test]
    fn playback_man_find_file_matches() {
        check(
            ee::s07::f_0026cfe8__generated,
            playback_man_find_file,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_cfe8 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x1000;
                let entries = DATA + 0x2000;
                let count = if seed % 7 == 6 { u32::MAX } else { seed % 6 };
                c.bus.write32(table + 0x50, count);
                c.bus.write32(table + 0x58, entries);
                for i in 0..6 {
                    c.bus.write32(entries + 0xc * i, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(table);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.pc = 0x0026_cfe8;
            },
        );
    }

    #[test]
    fn path_data_global_ctors_matches() {
        check(
            ee::s07::f_0026c438__generated,
            path_data_global_ctors,
            20,
            &|c, seed| {
                let mut r = Rng(0x0026_c438 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_c438;
            },
        );
    }

    /// Gate word for the playback destructor's callee fakes.
    pub(crate) const PLAYBACK_DTOR_GATE: u32 = DATA + 0x200000;
    static OC531_PREV_DTOR_FREE_BUF: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_DTOR_HANDLE: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_DTOR_DATA: AtomicUsize = AtomicUsize::new(0);

    fn oc531_fake_dtor_buf(c: &mut Cpu) {
        if c.bus.read32(PLAYBACK_DTOR_GATE) != 0x0531_0004 {
            return fakes::fall_back(c, &OC531_PREV_DTOR_FREE_BUF, ee::s10::f_003168e8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_68e8, ra);
    }

    fn oc531_fake_dtor_handle(c: &mut Cpu) {
        if c.bus.read32(PLAYBACK_DTOR_GATE) != 0x0531_0004 {
            return fakes::fall_back(c, &OC531_PREV_DTOR_HANDLE, ee::s10::f_00317e50__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e50, ra);
    }

    fn oc531_fake_dtor_data(c: &mut Cpu) {
        if c.bus.read32(PLAYBACK_DTOR_GATE) != 0x0531_0004 {
            return fakes::fall_back(c, &OC531_PREV_DTOR_DATA, ee::s07::f_0026c898__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0026_c898, ra);
    }

    #[test]
    fn playback_man_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003168E8),
            oc531_fake_dtor_buf,
            &OC531_PREV_DTOR_FREE_BUF,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc531_fake_dtor_handle,
            &OC531_PREV_DTOR_HANDLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026C898),
            oc531_fake_dtor_data,
            &OC531_PREV_DTOR_DATA,
        );
        check(ee::s07::f_0026c588__generated, playback_man_dtor, 200, &|c, seed| {
            let mut r = Rng(0x0026_c588 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(PLAYBACK_DTOR_GATE, 0x0531_0004);
            let this = DATA + 0x1000;
            c.bus
                .write32(this, if seed % 4 == 0 { 0 } else { DATA + 0x3000 });
            c.bus.write32(this + 0x6c, r.next_u32());
            c.bus.write32(this + 0x68, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.pc = 0x0026_c588;
        });
    }

    /// Gate word for the stream and record callee fakes of the file loaders.
    pub(crate) const OC531_CALLEE_GATE: u32 = DATA + 0x200000;
    static OC531_PREV_PRIME: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_READ: AtomicUsize = AtomicUsize::new(0);
    static OC531_PREV_RECORD: AtomicUsize = AtomicUsize::new(0);

    fn oc531_fake_stream_prime(c: &mut Cpu) {
        if c.bus.read32(OC531_CALLEE_GATE) != 0x0531_0005 {
            return fakes::fall_back(c, &OC531_PREV_PRIME, ee::s14::f_003e6448__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003e_6448, ra);
    }

    fn oc531_fake_stream_read(c: &mut Cpu) {
        if c.bus.read32(OC531_CALLEE_GATE) != 0x0531_0005 {
            return fakes::fall_back(c, &OC531_PREV_READ, ee::s14::f_003e6574__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003e_6574, ra);
    }

    fn oc531_fake_record(c: &mut Cpu) {
        if c.bus.read32(OC531_CALLEE_GATE) != 0x0531_0005 {
            return fakes::fall_back(c, &OC531_PREV_RECORD, ee::s09::f_002c85a0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002c_85a0, ra);
    }

    #[test]
    fn playback_man_apply_file_matches() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            oc531_fake_stream_prime,
            &OC531_PREV_PRIME,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002C85A0),
            oc531_fake_record,
            &OC531_PREV_RECORD,
        );
        check(
            ee::s07::f_0026d020__generated,
            playback_man_apply_file,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_d020 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC531_CALLEE_GATE, 0x0531_0005);
                let this = DATA + 0x1000;
                let table = DATA + 0x2000;
                c.bus.write32(this + 0x58, table);
                c.bus.write32(this + 4, DATA + 0x3000);
                for i in 0..3 {
                    let loaded = if (seed + i) % 2 == 0 { 0 } else { r.next_u32() };
                    c.bus.write32(table + 12 * i + 4, loaded);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3) as u64;
                c.pc = 0x0026_d020;
            },
        );
    }

    #[test]
    fn replay_cache_deserialize_matches() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc531_fake_stream_read,
            &OC531_PREV_READ,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002C85A0),
            oc531_fake_record,
            &OC531_PREV_RECORD,
        );
        check(
            ee::s07::f_0026d558__generated,
            replay_cache_deserialize,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_d558 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC531_CALLEE_GATE, 0x0531_0005);
                let cache = DATA + 0x5000;
                let pos = DATA + 0x6000;
                c.bus.write32(pos, DATA + 0x7000);
                c.bus.write32(cache, r.next_u32());
                c.bus.write32(cache + 0xc, r.next_u32());
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = sx(pos);
                c.pc = 0x0026_d558;
            },
        );
    }
}
