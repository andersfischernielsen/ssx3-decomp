//! Staging module oc780 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc780`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_8140, midi_banks_ctor, "MIDIBanks_ctor"),
    (
        0x0019_e3d0,
        fe_module_rider_preview_ctor,
        "FEModule_RiderPreviewCtor",
    ),
    (
        0x002e_d048,
        world_painter_create_view_queries,
        "WorldPainter_CreateViewQueries",
    ),
    (
        0x003d_6498,
        spch_register_event_table,
        "SPCH_RegisterEventTable",
    ),
    (0x0039_4d50, defer_man_ctor, "DeferMan_ctor"),
    (0x002e_2e18, lens_fx_base_ctor, "LensFXBase_ctor"),
];

/// `cMidiSys* MIDIBanks_ctor(cMidiSys* self, int embedded, int count, void* heap)` (0x2a8140).
///
/// Constructs the MIDI bank system. With `embedded != 0` it first builds the
/// sound manager reference (`SoundManagerRef_ctor(self + 0x14, self + 0x1c)`)
/// and constructs the bank manager through the `self[0]` object (`SndBankMan_ctor`,
/// 0x28b9b8). It then runs the bank system constructor (0x28fd00), points the
/// vtables of the bank manager and its monitor at the static tables (0x483948
/// block), and, when not embedded, copies those tables onto the stack with the
/// `this` adjustment. Finally it initialises MIDI (0x3b97d8), allocates the
/// 0x20-byte "MIDIBANKS" block with `operator_new_named` into `self + 0x10`,
/// registers `self` as the global MIDI system (`gp + 0x654`) and returns it.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn midi_banks_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a819c: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A3][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S2][0] = c.g[A2][0];
            c.g[S3][0] = c.g[A1][0];
            let t = c.g[S3][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            if t {
                break 'b_2a819c;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S1][0];
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x002a8188);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V1][0]);
                Callee {
                    addr: 0x002ade88,
                    f: ee::s08::SoundManagerRef_ctor,
                }
                .call(c, 0x002a8188)?;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x002a819c);
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0028b9b8, 0x002a819c)?;
                break 'b_2a819c;
            }
        }
        'b_2a8288: {
            c.cycles += 19;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002a81b0);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0028fd00, 0x002a81b0)?;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x39f0));
            c.g[A0][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[A3][0] = 0x480000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3a10));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2a8288;
            } else {
                c.cycles += 40;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x39f0));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3a10));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_2a8288;
            }
        }
        c.cycles += 20;
        c.g[V0][0] = 0x480000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a48));
        c.g[RA][0] = sx(0x002a8298);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x003b97d8, 0x002a8298)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3148));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a82b0);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a82b0)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0x654), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `FEModule_RiderPreview* FEModule_RiderPreviewCtor(FEModule_RiderPreview* self)` (0x19e3d0).
///
/// Clears the rider preview module: ids set to -1, the 256-entry texture
/// reference table (12-byte entries at `+0x1c`: flag 0, id -1), the pose and
/// board pose copied from the identity pose at 0x4ff230, counters, flags and
/// loading state cleared. Returns `self`.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn fe_module_rider_preview_ctor(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A0][0] = 0xffffffffffffffff;
    c.sw(c.g[A3][0] as u32, c.g[V0][0]);
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc70), 0u64);
    c.g[V0][0] = 0xff;
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc74), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x10), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x14), 0u64);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x18), 0u64);
    'l_19e408: loop {
        c.cycles += 7;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        let t = (c.g[V0][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0019e408);
            }
            continue 'l_19e408;
        } else {
            c.cycles += 29;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc1c), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc2c), c.g[V0][0]);
            c.g[A1][0] = 0x500000;
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc20), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xdd0));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc24), c.g[A2][0]);
            c.g[V0][0] = c.g[A3][0];
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc28), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xccc), 0u64);
            c.lq(3, c.g[A1][0] as u32);
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0xc30));
            c.lq(4, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sq(4, (c.g[A3][0] as u32).wrapping_add(0xc40));
            c.lq(3, c.g[A1][0] as u32);
            c.sq(3, (c.g[A3][0] as u32).wrapping_add(0xc50));
            c.lq(4, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcd4), c.g[A2][0]);
            c.sq(4, (c.g[A3][0] as u32).wrapping_add(0xc60));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc78), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcc4), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcc8), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcb4), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcb8), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcbc), 0u64);
            let t = c.g[RA][0] as u32;
            c.sw((c.g[A3][0] as u32).wrapping_add(0xcc0), 0u64);
            c.pc = t;
            return;
        }
    }
}

/// `int WorldPainter_CreateViewQueries(void)` (0x2ed048).
///
/// For each of the 8 lens rows (0xf0 bytes at 0x4fa370 + 0xf0 * i) creates the
/// nine world painter queries (camera, fog, light glow, screen tint, sky box,
/// sun, surface, lighting, weather) with `WorldPainterQuery_Create(pm, region)`
/// (0x2c0a58), where `pm` is the world's world painter manager, stores them in
/// `q[0..9]`, clears `f24` and fills the two vectors at `+0x28` and `+0x38`
/// with 1.0. Sets the render state manager's initialised flag (`gp + 0xa64`)
/// and returns 1.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn world_painter_create_view_queries(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = 0x500000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c90));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = sx((c.g[GP][0] as u32).wrapping_add(0x24b0));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S6][0] = sx((c.g[GP][0] as u32).wrapping_add(0x24a8));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S7][0] = sx((c.g[GP][0] as u32).wrapping_add(0x24a0));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[FP][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2498));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        'l_2ed0b0: loop {
            c.cycles += 56;
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2470));
            c.g[RA][0] = sx(0x002ed0bc);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c0a58, 0x002ed0bc)?;
            c.g[S1][0] = 0xf0;
            c.g[A0][0] = c.g[S2][0];
            c.mult(0, 17, 19, 17);
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2478));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S4][0] as u32));
            c.g[RA][0] = sx(0x002ed0dc);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed0dc)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2480));
            c.g[RA][0] = sx(0x002ed0ec);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed0ec)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2488));
            c.g[RA][0] = sx(0x002ed0fc);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed0fc)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2490));
            c.g[RA][0] = sx(0x002ed10c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed10c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[FP][0];
            c.g[RA][0] = sx(0x002ed11c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed11c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S7][0];
            c.g[RA][0] = sx(0x002ed12c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed12c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S6][0];
            c.g[RA][0] = sx(0x002ed13c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x002ed13c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x002ed14c);
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, 0x002c0a58, 0x002ed14c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = 0x500000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c58));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0xc));
            c.swc1(20, c.g[V1][0] as u32);
            c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
            c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.swc1(20, c.g[S1][0] as u32);
            c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x4));
            let t = c.g[V0][0] != 0u64;
            c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x8));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002ed0b0);
                }
                continue 'l_2ed0b0;
            } else {
                c.cycles += 15;
                c.g[V0][0] = 0x1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sw((c.g[GP][0] as u32).wrapping_add(0xa64), c.g[V0][0]);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `int SPCH_RegisterEventTable(unsigned short* bankData, int bankKey)` (0x3d6498).
///
/// Registers a speech bank in the 8-slot table at 0x44ffb8 (pairs of data
/// pointer and key). Only banks whose header halfword is 0xc03 count. Returns
/// 0 when the bank (same bank/sub id bytes at `+8`/`+9`, read by
/// `SPCH_GetBankIdBytes`) is already registered, else stores `bankData` and
/// `bankKey` in the first free slot and returns 1 (0 when the table is full).
/// After a fresh registration attempt it marks the event table bytes of every
/// slot with the same key as 0xff (the clear loop).
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn spch_register_event_table(c: &mut Cpu) {
    guest(c, |c| {
        'b_3d6600: {
            'b_3d65fc: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V0][0] = 0xc03;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = 0u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(31, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A0][0];
                c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S1][0] = c.g[A1][0];
                if t {
                    break 'b_3d65fc;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003d64d0);
                    c.g[A2][0] = c.g[SP][0] | 0x4;
                    Callee {
                        addr: 0x003da3a8,
                        f: ee::s14::SPCH_GetBankIdBytes,
                    }
                    .call(c, 0x003d64d0)?;
                    c.g[T0][0] = 0x450000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x48));
                    'l_3d64e8: loop {
                        'b_3d6510: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                            if c.g[V1][0] == 0u64 {
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                break 'b_3d6510;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x9));
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    break 'b_3d6510;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
                                    let t = c.g[V0][0] == c.g[A2][0];
                                    c.g[V0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_3d6600;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        break 'b_3d6510;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 8i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003d64e8);
                            }
                            continue 'l_3d64e8;
                        } else {
                            c.cycles += 3;
                            c.g[A1][0] = 0u64;
                            c.g[A3][0] = c.g[S1][0] << 32;
                            'l_3d6528: loop {
                                'b_3d6558: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < 8i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x48));
                                    if t {
                                        break 'b_3d6558;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                                        c.g[V1][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003d6528);
                                            }
                                            continue 'l_3d6528;
                                        } else {
                                            c.cycles += 4;
                                            c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                                            c.g[V0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                                            c.g[S2][0] = 0x1;
                                            break 'b_3d6558;
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[V0][0] = 0x450000;
                                c.g[T1][0] = 0xffffffffffff0000;
                                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x48));
                                c.g[T3][0] = c.g[A3][0] >> 32;
                                c.g[T5][0] = sx((c.g[T2][0] as u32).wrapping_add(0x4));
                                c.g[T4][0] = c.g[T2][0];
                                c.g[V1][0] = 0u64;
                                c.g[T1][0] = c.g[T1][0] | 0xfffc;
                                c.g[A2][0] = sx((c.g[V1][0] as u32) << 3);
                                'l_3d6580: loop {
                                    'b_3d65f0: {
                                        'b_3d65ec: {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[T2][0] as u32));
                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                            let t = c.g[A1][0] == 0u64;
                                            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_3d65ec;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[T5][0] as u32));
                                                c.g[V1][0] = c.lwu(c.g[V0][0] as u32);
                                                let t = c.g[V1][0] != c.g[T3][0];
                                                c.g[V1][0] = c.g[T0][0];
                                                if t {
                                                    break 'b_3d65f0;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[V0][0] = c.lhu(
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.g[A0][0] = 0u64;
                                                    c.g[V1][0] = c.lbu(
                                                        (c.g[A1][0] as u32).wrapping_add(0x12),
                                                    );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x3));
                                                    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A3][0] = sx((c.g[A1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    if t {
                                                        break 'b_3d65ec;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A1][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add(c.g[T4][0] as u32));
                                                        c.g[A2][0] = 0xff;
                                                        c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        'l_3d65d0: loop {
                                                            c.cycles += 7;
                                                            c.sb(c.g[V0][0] as u32, c.g[A2][0]);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                            c.g[V0][0] = c.lbu(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x12),
                                                            );
                                                            c.g[V0][0] = ((c.g[A0][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x003d65d0,
                                                                    );
                                                                }
                                                                continue 'l_3d65d0;
                                                            } else {
                                                                break 'b_3d65ec;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = c.g[T0][0];
                                        break 'b_3d65f0;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A2][0] = sx((c.g[V1][0] as u32) << 3);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003d6580);
                                        }
                                        continue 'l_3d6580;
                                    } else {
                                        break 'b_3d65fc;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
            break 'b_3d6600;
        }
        c.cycles += 6;
        c.lq(31, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `DeferMan* DeferMan_ctor(DeferMan* self)` (0x394d50).
///
/// Constructs the deferred-work manager (`new (self) sFontCache`): fills
/// three tables of `-1` halfword pairs (2599 records of 0x80 bytes from
/// `+0x80`, 1023 records of 0x18 bytes from `+0x51484`, one 0x10-byte record
/// at `+0x69ca8`), each record followed by `memset(rec, 0, 0x10)`, then
/// runs two empty counting loops (0x800 and 12 iterations). Returns `self`.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn defer_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.g[V0][0] = 0xa27;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x80));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = 0xffffffffffffffff;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_394d80: loop {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
            c.g[A1][0] = 0u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            'l_394d90: loop {
                c.cycles += 7;
                c.sh(c.g[V1][0] as u32, c.g[S2][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00394d90);
                    }
                    continue 'l_394d90;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00394db8);
                    c.g[A2][0] = 0x10;
                    call_ptr(c, 0x00416210, 0x00394db8)?;
                    c.g[V0][0] = c.g[S1][0];
                    let t = c.g[V0][0] != c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00394d80);
                        }
                        continue 'l_394d80;
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = 0x50000;
                        c.g[V1][0] = 0x3ff;
                        c.g[V0][0] = c.g[V0][0] | 0x1484;
                        c.g[S2][0] = 0xffffffffffffffff;
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S4][0] = 0xffffffffffffffff;
                        'l_394de0: loop {
                            c.cycles += 4;
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = 0u64;
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                            'l_394df0: loop {
                                c.cycles += 7;
                                c.sh(c.g[V1][0] as u32, c.g[S2][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00394df0);
                                    }
                                    continue 'l_394df0;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x00394e18);
                                    c.g[A2][0] = 0x10;
                                    call_ptr(c, 0x00416210, 0x00394e18)?;
                                    c.g[V1][0] = c.g[S1][0];
                                    let t = c.g[V1][0] != c.g[S4][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00394de0);
                                        }
                                        continue 'l_394de0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x7ff;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_394e30: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00394e30);
                                                }
                                                continue 'l_394e30;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] = 0x60000;
                                                c.g[V0][0] = c.g[V0][0] | 0x9ca8;
                                                c.g[A2][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x10));
                                                'l_394e60: loop {
                                                    c.cycles += 7;
                                                    c.sh(c.g[V1][0] as u32, c.g[A2][0]);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00394e60);
                                                        }
                                                        continue 'l_394e60;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00394e88);
                                                        c.g[A2][0] = 0x10;
                                                        call_ptr(c, 0x00416210, 0x00394e88)?;
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        c.g[V0][0] = 0xb;
                                                        'l_394e90: loop {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            let t = c.g[V0][0] != c.g[V1][0];
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00394e90,
                                                                    );
                                                                }
                                                                continue 'l_394e90;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[V0][0] = c.g[S3][0];
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                c.lq(
                                                                    18,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lq(
                                                                    19,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lq(
                                                                    20,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.g[RA][0] =
                                                                    c.ld(c.g[SP][0] as u32);
                                                                let t = c.g[RA][0] as u32;
                                                                c.g[SP][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x60));
                                                                c.pc = t;
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

/// `cLensFxMan* LensFXBase_ctor(cLensFxMan* self)` (0x2e2e18).
///
/// Constructs the lens effect manager: sets the vtable word at `+0x649c`,
/// runs `LensFX_FlareCtorStub` over the 2 x 256 halo records (0x30 bytes each,
/// from `+0x68`), `LensFX_InitFlareSet` over the sun halo records (0x210
/// bytes each, from `+0x6070`), sets `mbLocked` (`+0x6494`), clears the
/// wake pair at `+0x60`/`+0x64`, installs the draw callback in the graphics
/// manager (vtable call, `gp + 0x2a90`) and sets `cLensFxMan_mbDrawSun`
/// (`gp + 0xa28`). Returns `self`.
#[allow(unused_labels, unused_assignments, unused_parens, unreachable_code)]
pub fn lens_fx_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.g[V0][0] = 0x490000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7950));
        c.g[S7][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x60));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[FP][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6070));
        c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x64));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[S4][0] = 0xffffffffffffffff;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V1][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.sw((c.g[S7][0] as u32).wrapping_add(0x649c), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x68));
        'l_2e2e78: loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6000));
            c.g[V0][0] = 0x1;
            'l_2e2e88: loop {
                c.cycles += 4;
                c.g[S0][0] = c.g[A0][0];
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3000));
                c.g[S1][0] = 0xff;
                'l_2e2e98: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002e2ea4);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x002e2860,
                        f: ee::s09::LensFX_FlareCtorStub,
                    }
                    .call(c, 0x002e2ea4)?;
                    let t = c.g[S1][0] != c.g[S4][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002e2e98);
                        }
                        continue 'l_2e2e98;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[S3][0];
                        let t = c.g[V0][0] != c.g[S4][0];
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002e2e88);
                            }
                            continue 'l_2e2e88;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.g[S6][0];
                            let t = c.g[V1][0] != c.g[S4][0];
                            c.g[V0][0] = c.g[S5][0];
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002e2e78);
                                }
                                continue 'l_2e2e78;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[FP][0];
                                c.g[V0][0] = 0u64;
                                c.g[S4][0] = 0xffffffffffffffff;
                                'l_2e2ed0: loop {
                                    c.cycles += 4;
                                    c.g[S0][0] = c.g[A0][0];
                                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x420));
                                    c.g[S1][0] = 0x1;
                                    'l_2e2ee0: loop {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x002e2eec);
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                        Callee {
                                            addr: 0x002f4380,
                                            f: ee::s09::LensFX_InitFlareSet,
                                        }
                                        .call(c, 0x002e2eec)?;
                                        let t = c.g[S1][0] != c.g[S4][0];
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x210));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002e2ee0);
                                            }
                                            continue 'l_2e2ee0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.g[S3][0];
                                            let t = c.g[V0][0] != c.g[S4][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002e2ed0);
                                                }
                                                continue 'l_2e2ed0;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = 0x1;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.sw(
                                                    (c.g[S7][0] as u32).wrapping_add(0x6494),
                                                    c.g[V0][0],
                                                );
                                                c.g[A0][0] = 0u64;
                                                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                                'l_2e2f18: loop {
                                                    c.cycles += 7;
                                                    c.sw(c.g[V0][0] as u32, 0u64);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                                    c.sw(c.g[V1][0] as u32, 0u64);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    let t = (c.g[A0][0] as i64) >= 0;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x002e2f18);
                                                        }
                                                        continue 'l_2e2f18;
                                                    } else {
                                                        c.cycles += 24;
                                                        c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_add(0x2a90));
                                                        c.g[A1][0] = 0x2e0000;
                                                        c.g[A2][0] = c.g[S7][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3110));
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x3a8));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x3ac));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[RA][0] = sx(0x002e2f58);
                                                        c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x002e2f58)?;
                                                        c.g[V0][0] = c.g[S7][0];
                                                        c.g[A0][0] = 0x1;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0xa0),
                                                        );
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x90),
                                                        );
                                                        c.lq(
                                                            18,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.lq(
                                                            19,
                                                            (c.g[SP][0] as u32).wrapping_add(0x70),
                                                        );
                                                        c.lq(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.lq(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        );
                                                        c.lq(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.lq(
                                                            30,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.g[RA][0] = c
                                                            .ld((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.sw(
                                                            (c.g[GP][0] as u32).wrapping_add(0xa28),
                                                            c.g[A0][0],
                                                        );
                                                        let t = c.g[RA][0] as u32;
                                                        c.g[SP][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xb0));
                                                        c.pc = t;
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc780's callee fakes in a machine's RAM.
    pub(crate) const OC780_CTRL: u32 = DATA + 0x40_0000;
    pub(crate) const OC780_MAGIC: u32 = 0x0780_f4ce;

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc780_arm(c: &mut Cpu) {
        c.bus.write32(OC780_CTRL, OC780_MAGIC);
    }

    /// Gated fakes for the callees that have an override slot: they log the
    /// call like `fakes::fake_call` (and may unwind) when armed, and run the
    /// body they replaced otherwise.
    macro_rules! oc780_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC780_CTRL) != OC780_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc780_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc780_fakes!(
        (
            oc780_fk_2ade88,
            OC780_PREV_2ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc780_fk_2e2860,
            OC780_PREV_2E2860,
            ee::s09::OVERRIDE_002E2860,
            ee::s09::f_002e2860__generated,
            0x002e_2860
        ),
        (
            oc780_fk_2c0a58,
            OC780_PREV_2C0A58,
            ee::s08::OVERRIDE_002C0A58,
            ee::s08::f_002c0a58__generated,
            0x002c_0a58
        ),
        (
            oc780_fk_416210,
            OC780_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc780_fk_317e30,
            OC780_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
    );

    /// `MIDIBanks_ctor`: embedded (`a1 != 0`, builds the sound manager
    /// reference first) and plain, with 0 or 1 banks. The callees with an
    /// override slot (`SoundManagerRef_ctor`, `operator_new_named`) are
    /// faked and may unwind; the bank system constructors (0x28b9b8,
    /// 0x28fd00) and `SNDmidiinit` run for real on the fixture.
    #[test]
    fn midi_banks_ctor_matches() {
        oc780_install_fakes();
        fakes::covered(
            ee::s08::f_002a8140__generated,
            midi_banks_ctor,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x780a, 4);
                oc780_arm(c);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(r.next_u32() | 1)
                };
                c.g[A2][0] = (seed as u64 / 2) % 2;
                c.g[A3][0] = sx(DATA + 0x8000);
                // The plain path reaches the bank manager through `this[0]`.
                c.bus.write32(this, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x1d8, DATA + 0x3000);
            },
        );
    }

    /// `FEModule_RiderPreviewCtor`: clears a random-filled module (a leaf:
    /// the 256-entry texture table loop and the identity pose copies).
    #[test]
    fn fe_module_rider_preview_ctor_matches() {
        check(
            ee::s03::f_0019e3d0__generated,
            fe_module_rider_preview_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x780b, 0);
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 0x340);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `WorldPainter_CreateViewQueries`: nine `WorldPainterQuery_Create` calls
    /// for each of the 8 lens rows (faked, may unwind), reached through
    /// `SSXApp->Game->World->m_pWorldPainterMan` (`gp - 0x848` chain).
    #[test]
    fn world_painter_create_view_queries_matches() {
        oc780_install_fakes();
        fakes::covered(
            ee::s09::f_002ed048__generated,
            world_painter_create_view_queries,
            120,
            &|c, seed| {
                machine(c, seed, 0x780c, 30);
                oc780_arm(c);
                let gp = c.g[GP][0] as u32;
                c.bus.write32(gp - 0x848, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0x10, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 0x8, DATA + 0x1300);
            },
        );
    }

    /// `SPCH_RegisterEventTable`: header 0xc03 or not, bank already in the
    /// table, a free slot, a full table, and the clear loop over the slots
    /// with the same key. The only callee (`SPCH_GetBankIdBytes`) is a real
    /// leaf.
    #[test]
    fn spch_register_event_table_matches() {
        check(
            ee::s14::f_003d6498__generated,
            spch_register_event_table,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x780d, 0);
                let table = 0x0044_ffb8u32;
                let bank = DATA + 0x1000;
                c.bus
                    .write16(bank, if seed % 7 == 0 { 0x1234 } else { 0xc03 });
                c.bus.write8(bank + 8, (r.next_u32() % 2) as u8);
                c.bus.write8(bank + 9, (r.next_u32() % 2) as u8);
                let full = seed % 4 == 0;
                for i in 0..8u32 {
                    let slot = if full || r.next_u32() % 3 != 0 {
                        DATA + 0x2000 + 0x400 * i
                    } else {
                        0
                    };
                    c.bus.write32(table + 8 * i, slot);
                    c.bus
                        .write32(table + 8 * i + 4, [0x11, 0x22][(r.next_u32() % 2) as usize]);
                    if slot != 0 {
                        c.bus.write8(slot + 8, (r.next_u32() % 2) as u8);
                        c.bus.write8(slot + 9, (r.next_u32() % 2) as u8);
                        c.bus.write16(slot + 0x10, (r.next_u32() % 6) as u16);
                        c.bus.write8(slot + 0x12, (r.next_u32() % 5) as u8);
                    }
                }
                c.g[A0][0] = sx(bank);
                c.g[A1][0] = [0x11u64, 0x22][(r.next_u32() % 2) as usize];
            },
        );
    }

    /// `DeferMan_ctor`: 3600 `memset` calls (faked, may unwind) between the
    /// record loops.
    #[test]
    fn defer_man_ctor_matches() {
        oc780_install_fakes();
        fakes::covered(
            ee::s13::f_00394d50__generated,
            defer_man_ctor,
            40,
            &|c, seed| {
                machine(c, seed, 0x780e, 3600);
                oc780_arm(c);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    /// `LensFXBase_ctor`: 512 `LensFX_FlareCtorStub` calls (faked, may
    /// unwind), the real `LensFX_InitFlareSet` (which calls the faked stub
    /// once) and the draw-callback registration through the graphics
    /// manager's vtable (a call to an address with no code: the miss hook).
    #[test]
    fn lens_fx_base_ctor_matches() {
        oc780_install_fakes();
        fakes::covered(
            ee::s09::f_002e2e18__generated,
            lens_fx_base_ctor,
            40,
            &|c, seed| {
                machine(c, seed, 0x780f, 515);
                oc780_arm(c);
                let gp = c.g[GP][0] as u32;
                let gfx = DATA + 0x8_0000;
                let table = DATA + 0x8_0100;
                c.bus.write32(gp + 0x2a90, gfx);
                c.bus.write32(gfx + 0x10d8, table);
                c.bus.write32(table + 0x3ac, 0x4000 + 0x10 * (seed % 4));
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }
}
