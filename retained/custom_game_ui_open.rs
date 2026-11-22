//! Native GameUI_OpenScreen20ab50, retained uninstalled.
//!
//! Direct dialog IDs0xc/0xd allocate the two finish dialogs, set their
//! finishov name and push them on the module's native UI stack. There is
//! no course/career/active-dialog suppression gate here. Loading and input
//! ownership are handled later by the selected UI state's virtual methods.
//! No automated test, build, replay or installation accompanies this port.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0020_ab50;
const END: u32 = 0x0020_bab8;
const ALLOC: u32 = 0x0031_7d70;
const BASE_CTOR: u32 = 0x0039_e2a0;
const HASH: u32 = 0x0031_7670;
const PUSH: u32 = 0x0039_f320;
const PUSH_RETURN: u32 = 0x0020_ba9c;
const ARRAY_LOOP: u32 = 0x0020_b7b0;
const FORM_ID: u32 = 0x004a_2088;
const FINISH_NAME: u32 = 0x0046_e340;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

#[derive(Clone, Copy)]
struct Dialog {
    bytes: u32,
    cycles: u64,
    allocated: u32,
    constructed: u32,
    vtable: u32,
}

// Native chain order1..40. Cycle charges precede allocation exactly as in
// each original block. Zero vtables select a complete derived constructor.
const DIALOGS: [Dialog; 40] = [
    d(0x114,22,0x20ab9c,0x20abac,0x473098),
    d(0x114,23,0x20ac00,0x20ac10,0x472fc0),
    d(0x114,23,0x20ac64,0x20ac74,0x472ee8),
    d(0x114,23,0x20acc8,0x20acd8,0x472e10),
    d(0x114,23,0x20ad2c,0x20ad3c,0x472d38),
    d(0x114,23,0x20ad90,0x20ada0,0x472c60),
    d(0x09c,22,0x20adf4,0x20ae04,0x472b88),
    d(0x09c,22,0x20ae54,0x20ae64,0x472850),
    d(0x09c,22,0x20aeb4,0x20aec4,0x472778),
    d(0x09c,22,0x20af14,0x20af24,0x474088),
    d(0x0a0,22,0x20af74,0x20af84,0x473fb0),
    d(0x0a4,28,0x20afd4,0x20afe4,0x473e00), //0xc Race finish
    d(0x0a4,28,0x20b04c,0x20b05c,0x473d28), //0xd Freestyle finish
    d(0x09c,22,0x20b0c4,0x20b0d4,0x474b80),
    d(0x0a4,22,0x20b124,0x20b134,0x4734d0),
    d(0x0a0,22,0x20b184,0x20b194,0x472108),
    d(0x0b8,22,0x20b1e4,0x20b1f4,0x474aa8),
    d(0x274,12,0x20b244,0x20b254,0),
    d(0x274,12,0x20b27c,0x20b28c,0),
    d(0x09c,22,0x20b2b4,0x20b2c4,0x4725c8),
    d(0x09c,22,0x20b314,0x20b324,0x4726a0),
    d(0x09c,22,0x20b374,0x20b384,0x473ed8),
    d(0x0a0,23,0x20b3d4,0x20b3e4,0x4749d0),
    d(0x0a0,24,0x20b438,0x20b448,0x4748f8),
    d(0x0a0,24,0x20b4a0,0x20b4b0,0x474820),
    d(0x0a0,24,0x20b508,0x20b518,0x474748),
    d(0x0a0,24,0x20b570,0x20b580,0x474670),
    d(0x0a0,24,0x20b5d8,0x20b5e8,0x474598),
    d(0x0a0,24,0x20b640,0x20b650,0x473688),
    d(0x0a4,24,0x20b6a8,0x20b6b8,0x4735b0),
    d(0x09c,22,0x20b710,0x20b720,0x473760),
    d(0x1d4,22,0x20b770,0x20b780,0x473320),
    d(0x390,24,0x20b7f0,0x20b800,0x4733f8),
    d(0x180,12,0x20b858,0x20b868,0),
    d(0x09c,22,0x20b890,0x20b8a0,0x473c50),
    d(0x27c,13,0x20b8f0,0x20b904,0),
    d(0x234,12,0x20b92c,0x20b93c,0),
    d(0x0a0,24,0x20b964,0x20b974,0x4744c0),
    d(0x0a4,25,0x20b9cc,0x20b9dc,0x472ab0),
    d(0x0e4,22,0x20ba38,0x20ba48,0x473248),
];

const fn d(bytes: u32, cycles: u64, allocated: u32, constructed: u32, vtable: u32) -> Dialog {
    Dialog { bytes, cycles, allocated, constructed, vtable }
}

#[derive(Clone, Copy)]
enum Phase { Entry, Select(usize), Allocated(usize), Constructed(usize), Extra(usize), Array, Push, Return }

fn resume(pc: u32) -> Option<Phase> {
    if pc == ENTRY { return Some(Phase::Entry) }
    if pc == ARRAY_LOOP { return Some(Phase::Array) }
    if pc == PUSH_RETURN { return Some(Phase::Return) }
    for (i, dialog) in DIALOGS.iter().enumerate() {
        if pc == dialog.allocated { return Some(Phase::Allocated(i)) }
        if pc == dialog.constructed { return Some(Phase::Constructed(i)) }
    }
    [(0x20b020,11),(0x20b098,12),(0x20b680,28),(0x20b6e8,29),(0x20b830,32)]
        .into_iter().find_map(|(at, i)| (pc == at).then_some(Phase::Extra(i)))
}

/// Initialize the three common UI-state fields in their native order.
fn clear_state(c: &mut Cpu) {
    c.sw(at(c, S0, 0x48), 0);
    c.sw(at(c, S0, 0x4c), 0);
    c.sw(at(c, S0, 0x54), 0);
}

/// The ordinary inlined constructors (without a third guest call).
fn initialize_simple(c: &mut Cpu, id: usize, dialog: Dialog) {
    c.g[V1][0] = 0x004a_0000;
    c.g[V0][0] = 0x0047_0000;
    c.g[A0][0] = c.lhu(FORM_ID);
    c.g[V0][0] = sx(dialog.vtable);
    if (24..=28).contains(&id) || id == 38 || id == 39 {
        c.g[V1][0] = if id <= 28 { (id - 23) as u64 } else if id == 38 { 6 } else { 1 };
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.sh(at(c, S0, 0x58), c.g[A0][0]);
        c.g[T0][0] = c.g[S0][0];
        c.sw(at(c, S0, if id == 39 { 0xa0 } else { 0x9c }), c.g[V1][0]);
        c.sw(at(c, S0, 0x98), c.g[S1][0]);
        clear_state(c);
        if id == 39 { c.sw(at(c, S0, 0x9c), 0); }
    } else if id == 40 {
        c.sw(at(c, S0, 0x98), c.g[S1][0]);
        c.g[T0][0] = c.g[S0][0];
        c.sh(at(c, S0, 0x58), c.g[A0][0]);
        c.sw(at(c, S0, 8), c.g[V0][0]);
        clear_state(c);
    } else {
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.g[T0][0] = c.g[S0][0];
        c.sh(at(c, S0, 0x58), c.g[A0][0]);
        c.sw(at(c, S0, 0x98), c.g[S1][0]);
        clear_state(c);
        if id <= 6 { c.sw(at(c, S0, 0xc4), 0); }
        if id == 23 { c.sw(at(c, S0, 0x9c), 0); }
    }
}

/// The finish dialogs and three dialogs with an explicit final initializer.
/// Return the original third call's target and return address.
fn initialize_with_call(c: &mut Cpu, id: usize, dialog: Dialog) -> (u32, u32) {
    c.g[V1][0] = 0x004a_0000;
    c.g[V0][0] = 0x0047_0000;
    c.g[A1][0] = c.lhu(FORM_ID);
    c.g[V0][0] = sx(dialog.vtable);
    if id == 12 || id == 13 {
        c.g[V1][0] = 1;
        c.g[A0][0] = 0x0047_0000;
        c.sh(at(c, S0, 0x58), c.g[A1][0]);
        c.g[A0][0] = sx(FINISH_NAME);
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.sw(at(c, S0, 0xa0), c.g[V1][0]);
        c.sw(at(c, S0, 0x98), c.g[S1][0]);
        clear_state(c);
        (HASH, if id == 12 { 0x0020_b020 } else { 0x0020_b098 })
    } else {
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.g[A0][0] = if id == 33 { sx(at(c, S0, 0x9c)) } else { c.g[S0][0] };
        c.sh(at(c, S0, 0x58), c.g[A1][0]);
        c.sw(at(c, S0, 0x98), c.g[S1][0]);
        clear_state(c);
        match id {
            29 => (0x001f_7548,0x0020_b680),
            30 => (0x001f_7738,0x0020_b6e8),
            33 => (0x0020_06b8,0x0020_b830),
            _ => unreachable!(),
        }
    }
}

/// `void GameUI_OpenScreen(int type)`.
/// Construct one of40 native UI dialog classes and push it as an active
/// special state. Native allocation, base initialization, default fields,
/// scratch registers, branch charges and call-return continuations remain
/// explicit. The retained port is uninstalled and equivalence was not run.
pub fn game_ui_open_screen(c: &mut Cpu) {
    guest(c, |c| {
        let Some(mut phase) = resume(c.pc) else {
            c.jump_out(c.pc, ENTRY, END);
            return Ok(());
        };
        let frame = Frame { size: 0x50 };
        loop {
            match phase {
                Phase::Entry => {
                    c.cycles += 13;
                    Frame::open(c, 0x50);
                    c.g[A1][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
                    frame.save(c, S1, 0x30);
                    c.g[T0][0] = 0;
                    frame.save(c, S0, 0x40);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x20);
                    c.g[V1][0] = 1;
                    frame.save(c, S3, 0x10);
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lw(at(c, A1, 0x84));
                    c.g[S2][0] = c.lw(at(c, V0, 0x48));
                    phase = Phase::Select(0);
                }
                Phase::Select(i) if i < 40 => {
                    let id = i + 1;
                    let selected;
                    if id == 1 {
                        selected = c.g[S1][0] == c.g[V1][0];
                    } else {
                        c.cycles += if id == 2 || id == 33 { 3 } else { 2 };
                        if id == 2 || id == 33 { c.g[V0][0] = id as u64; }
                        selected = c.g[S1][0] == c.g[V0][0];
                        if id == 32 { c.g[S3][0] = sx(at(c, S2, 0x18)); }
                        else if id != 40 { c.g[V0][0] = (id + 1) as u64; }
                    }
                    if !selected {
                        if id == 40 { c.g[V0][0] = c.lw(at(c, T0, 0x1c)); }
                        phase = Phase::Select(i+1);
                        continue;
                    }
                    let dialog = DIALOGS[i];
                    c.cycles += dialog.cycles;
                    c.g[A1][0] = 0x0047_0000;
                    c.g[A0][0] = dialog.bytes as u64;
                    c.g[A1][0] = sx(0x0047_1a30);
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 0;
                    call_ptr(c, ALLOC, dialog.allocated)?;
                    phase = Phase::Allocated(i);
                }
                Phase::Select(_) => phase = Phase::Push,
                Phase::Allocated(i) => {
                    let id = i+1;
                    let dialog = DIALOGS[i];
                    if dialog.vtable != 0 {
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, BASE_CTOR, dialog.constructed)?;
                    } else {
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = id as u64;
                        let ctor = match id {
                            18 => 0x001f_dbf0,
                            19 => 0x001f_e968,
                            34 => 0x001e_3c00,
                            36 => { c.g[A3][0] = 0x2a; 0x0021_1088 },
                            37 => 0x0021_2138,
                            _ => unreachable!(),
                        };
                        call_ptr(c, ctor, dialog.constructed)?;
                    }
                    phase = Phase::Constructed(i);
                }
                Phase::Constructed(i) => {
                    let id = i+1;
                    let dialog = DIALOGS[i];
                    if dialog.vtable == 0 {
                        c.g[T0][0] = c.g[V0][0];
                        c.g[V0][0] = (id+1) as u64;
                        phase = Phase::Select(i+1);
                    } else if matches!(id,12|13|29|30|33) {
                        let (target, ret) = initialize_with_call(c, id, dialog);
                        call_ptr(c, target, ret)?;
                        phase = Phase::Extra(i);
                    } else if id == 32 {
                        c.g[V0][0] = 0x004a_0000;
                        c.g[V1][0] = 0x0047_0000;
                        c.g[A0][0] = c.lhu(FORM_ID);
                        c.g[V1][0] = sx(dialog.vtable);
                        c.sw(at(c, S0, 0x98), c.g[S1][0]);
                        c.g[A1][0] = u64::MAX;
                        c.sh(at(c, S0, 0x58), c.g[A0][0]);
                        c.g[V0][0] = 7;
                        c.sw(at(c, S0, 8), c.g[V1][0]);
                        clear_state(c);
                        phase = Phase::Array;
                    } else {
                        initialize_simple(c, id, dialog);
                        if id == 40 { c.g[V0][0] = c.lw(at(c, T0, 0x1c)); }
                        else if id != 1 { c.g[V0][0] = (id+1) as u64; }
                        phase = Phase::Select(i+1);
                    }
                }
                Phase::Extra(i) => {
                    if i == 11 || i == 12 { c.sw(at(c, S0, 0xc), c.g[V0][0]); }
                    c.g[T0][0] = c.g[S0][0];
                    c.g[V0][0] = (i+2) as u64;
                    phase = Phase::Select(i+1);
                }
                Phase::Array => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    let more = c.g[V0][0] != c.g[A1][0];
                    c.g[T0][0] = c.g[S0][0];
                    if more {
                        if back_edge_unwinds(c, ARRAY_LOOP) { return Err(Unwound) }
                    } else { phase = Phase::Select(32); }
                }
                Phase::Push => {
                    c.cycles += 16;
                    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[T0][0];
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[A2][0] = 0;
                    c.g[V0][0] |= 0x100;
                    c.g[A3][0] = 0;
                    c.sw(at(c, T0, 0x1c), c.g[V0][0]);
                    call_ptr(c, PUSH, PUSH_RETURN)?;
                    phase = Phase::Return;
                }
                Phase::Return => {
                    frame.restore(c, S0, 0x40);
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    });
}
