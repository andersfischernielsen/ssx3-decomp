//! Staging module oc763 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc763`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001d_5f40,
    mem_card_ui_apply_loaded_data,
    "MemCardUI_ApplyLoadedData",
)];

/// The inlined `Cfg_1D5F40()` copy: the 0x288-byte option block at 0x535610
/// goes to `dst` (here the stack). The source is a constant 8-aligned address,
/// so the original's unaligned `ldl/ldr` loop is never taken; this is its
/// doubleword loop (20 rounds of 32 bytes, `v0` the end pointer) with the four
/// scratch registers `lr` in the original's allocation. The 8-byte tail is
/// `oc763_cfg_tail`.
fn oc763_cfg_copy(c: &mut Cpu, src: usize, dst: usize, lr: [usize; 4], head: u32) {
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        let s = c.g[src][0] as u32;
        let d = c.g[dst][0] as u32;
        for (i, &reg) in lr.iter().enumerate() {
            c.g[reg][0] = c.ld(s.wrapping_add(8 * i as u32));
        }
        for (i, &reg) in lr.iter().enumerate() {
            let v = c.g[reg][0];
            c.sd(d.wrapping_add(8 * i as u32), v);
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[V0][0];
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The last 8 bytes of the option block copy (`ldl/ldr` into `reg`, then
/// `sdl/sdr`).
fn oc763_cfg_tail(c: &mut Cpu, src: usize, dst: usize, reg: usize) {
    let s = c.g[src][0] as u32;
    c.ldl(reg, s.wrapping_add(7));
    c.ldr(reg, s);
    let d = c.g[dst][0] as u32;
    c.sdl(reg, d.wrapping_add(7));
    c.sdr(reg, d);
}

/// `cFEMemCard_readDone` (0x1d5f40, `MemCardUI_ApplyLoadedData`): the memory
/// card read finished. With the save interface (`getInterface(getBE(), 5)`)
/// it hands the read buffer to `loadSaveGame` (type 2, then pushes the loaded
/// sound options through the audio object: output mode, three volumes, DJ
/// speech and arcade SFX, each re-reading the option block at 0x535610 into a
/// stack copy), or to `loadSaveProfile` (type 0, with the player index);
/// type 1 loads nothing. Returns 0 when the load fails; otherwise, if the
/// buffer state (`+0x228`) is 1, frees the start buffer and clears the
/// read/start buffer and state, and returns 1.
pub fn mem_card_ui_apply_loaded_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, 16, 0x2a0);
        frame.save_ra(c, 0x290);
        call_ptr(c, 0x0014_dc80, 0x001d_5f54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 5;
        call_ptr(c, 0x0014_dd58, 0x001d_5f60)?;
        let inst_slot = (c.g[GP][0] as u32).wrapping_sub(0x10c8);
        c.g[A1][0] = c.lw(inst_slot);
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1bc));
        let ty = c.g[V1][0];
        c.g[V0][0] = ((ty as i64) < 2) as u64;
        'epi: {
            'tail: {
                if ty == 1 {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(inst_slot);
                    break 'tail;
                }
                c.cycles += 2;
                let ge2 = c.g[V0][0] == 0;
                c.g[V0][0] = 2;
                if !ge2 {
                    c.cycles += 2;
                    if ty != 0 {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw(inst_slot);
                        break 'tail;
                    }
                    // type 0: loadSaveProfile(iface, readBuffer, playerIdx)
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc));
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1d0));
                    call_ptr(c, 0x0015_2de0, 0x001d_6464)?;
                    let ok = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(inst_slot);
                    if ok {
                        break 'tail;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'epi;
                }
                c.cycles += 2;
                if ty != c.g[V0][0] {
                    c.g[A0][0] = c.lw(inst_slot);
                    break 'tail;
                }
                // type 2: loadSaveGame(iface, readBuffer)
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1d0));
                call_ptr(c, 0x0015_2bb0, 0x001d_5fa0)?;
                let failed = c.g[V0][0] == 0;
                c.g[V0][0] = 0;
                if failed {
                    break 'epi;
                }
                c.cycles += 4;
                call_ptr(c, 0x0028_b180, 0x001d_5fb0)?;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.lw(inst_slot);
                    break 'tail;
                }
                // The audio object exists: apply the loaded sound options.
                c.cycles += 11;
                call_ptr(c, 0x0014_dc80, 0x001d_5fc0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 4;
                call_ptr(c, 0x0014_dd58, 0x001d_5fcc)?;
                c.g[V1][0] = 0x53_0000;
                c.g[S0][0] = c.g[V1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[V1][0] & 7;
                c.g[A0][0] = c.g[SP][0];
                oc763_cfg_copy(c, V1, A0, [T1, A1, A2, A3], 0x001d_6040);
                // output mode: bits 12..13
                c.cycles += 15;
                oc763_cfg_tail(c, V1, A0, 8);
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 12);
                c.g[A0][0] &= 3;
                let apply = Callee {
                    addr: 0x001d_64c0,
                    f: ee::s04::MemCardUI_ApplySoundMode,
                };
                apply.call(c, 0x001d_608c)?;
                call_ptr(c, 0x0028_b180, 0x001d_6094)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [A2, A3, T0, T1], 0x001d_6104);
                // sfx volume: bits 0..3
                c.cycles += 14;
                oc763_cfg_tail(c, A1, V1, 2);
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] &= 0xf;
                call_ptr(c, 0x0028_7410, 0x001d_614c)?;
                call_ptr(c, 0x0028_b180, 0x001d_6154)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [T1, A2, A3, T0], 0x001d_61c4);
                // speech volume: bits 8..11
                c.cycles += 15;
                oc763_cfg_tail(c, A1, V1, 9);
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[A1][0] &= 0xf;
                call_ptr(c, 0x0028_7488, 0x001d_6210)?;
                call_ptr(c, 0x0028_b180, 0x001d_6218)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [T0, T1, A2, A3], 0x001d_6288);
                // music volume: high nibble of byte 0
                c.cycles += 14;
                oc763_cfg_tail(c, A1, V1, 8);
                c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                c.g[A1][0] >>= 4;
                call_ptr(c, 0x0028_7520, 0x001d_62d0)?;
                call_ptr(c, 0x0028_b180, 0x001d_62d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A1][0] & 7;
                c.g[V1][0] = c.g[SP][0];
                oc763_cfg_copy(c, A1, V1, [A3, T0, T1, A2], 0x001d_6348);
                // DJ speech flag: bit 17
                c.cycles += 15;
                oc763_cfg_tail(c, A1, V1, 7);
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                c.g[A1][0] &= 1;
                call_ptr(c, 0x0028_7558, 0x001d_6394)?;
                call_ptr(c, 0x0028_b180, 0x001d_639c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[V1][0] & 7;
                c.g[A1][0] = c.g[SP][0];
                oc763_cfg_copy(c, V1, A1, [A2, A3, T0, T1], 0x001d_640c);
                // arcade SFX flag: bit 16
                c.cycles += 9;
                oc763_cfg_tail(c, V1, A1, 2);
                c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(2));
                c.g[A1][0] &= 1;
                call_ptr(c, 0x0028_75d0, 0x001d_6454)?;
                c.g[A0][0] = c.lw(inst_slot);
            }
            // The load worked: release a finished read buffer.
            c.cycles += 4;
            c.g[V1][0] = 1;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x228));
            let busy = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if busy {
                break 'epi;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1d4));
            let none = c.g[A0][0] == 0;
            c.g[V0][0] = c.lw(inst_slot);
            if !none {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x001d_649c)?;
                c.g[V0][0] = c.lw(inst_slot);
            }
            c.cycles += 4;
            let inst = c.g[V0][0] as u32;
            c.sw(inst.wrapping_add(0x228), 0);
            c.sw(inst.wrapping_add(0x1d0), 0);
            c.sw(inst.wrapping_add(0x1d4), 0);
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects in DATA+0x200000 and up, clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return.
    const OC763_CTRL: u32 = DATA + 0x20_0000;
    const OC763_MAGIC: u32 = 0x4f43_3736 + 3;
    const OC763_INST: u32 = DATA + 0x21_0000;
    const OC763_AUDIO: u32 = DATA + 0x22_0000;
    const OC763_P1: u32 = DATA + 0x23_0000;
    const OC763_P2: u32 = DATA + 0x24_0000;
    const OC763_P3: u32 = DATA + 0x25_0000;
    const OC763_P4: u32 = DATA + 0x26_0000;
    const OC763_FORCE: u32 = DATA + 0x30_0000;
    const OC763_CFG: u32 = 0x0053_5610;

    fn oc763_mail(addr: u32) -> u32 {
        OC763_FORCE + (addr & 0xf_fffc)
    }

    /// The gated fake callee: logs the call, returns the mailbox result
    /// (1 = 0, else value + 3; empty = the fake's pointer).
    fn oc763_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc763_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc763_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC763_CTRL) != OC763_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc763_fake_call(c, $addr);
                }
            )*
            fn oc763_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc763_fakes!(
        (
            oc763_fake_14dc80,
            OC763_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc763_fake_14dd58,
            OC763_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc763_fake_152bb0,
            OC763_PREV_152BB0,
            ee::s01::OVERRIDE_00152BB0,
            ee::s01::f_00152bb0__generated,
            0x0015_2bb0
        ),
        (
            oc763_fake_152de0,
            OC763_PREV_152DE0,
            ee::s01::OVERRIDE_00152DE0,
            ee::s01::f_00152de0__generated,
            0x0015_2de0
        ),
        (
            oc763_fake_287410,
            OC763_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc763_fake_287488,
            OC763_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc763_fake_287520,
            OC763_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc763_fake_287558,
            OC763_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc763_fake_2875d0,
            OC763_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
        (
            oc763_fake_28b180,
            OC763_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc763_fake_317e98,
            OC763_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
    );

    /// `cFEMemCard_readDone` against the generated body: every memory card
    /// type (0, 1, 2, an unknown one), failing and succeeding loads, with and
    /// without the audio object (whose option copies run the real
    /// `MemCardUI_ApplySoundMode` down its short path), a finished read buffer
    /// (state 1, with and without a start buffer) or not. The callees are gated
    /// fakes, so the call log and argument registers are compared too, and some
    /// calls unwind.
    #[test]
    fn mem_card_ui_apply_loaded_data_matches() {
        oc763_install_fakes();
        covered(
            ee::s04::f_001d5f40__generated,
            mem_card_ui_apply_loaded_data,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d_5f40, 20);
                c.bus.write32(OC763_CTRL, OC763_MAGIC);
                let ty = [2u32, 2, 2, 0, 0, 1, 3, 0xffff_ffff, 2, 0, 2, 5][(seed % 12) as usize];
                fill(c, &mut r, OC763_INST, 0x90);
                c.bus.write32(GAME_GP.wrapping_sub(0x10c8), OC763_INST);
                c.bus.write32(OC763_INST + 0x1bc, ty);
                c.bus.write32(OC763_INST + 0x1d0, DATA + 0x2000);
                c.bus.write32(
                    OC763_INST + 0x1d4,
                    if (seed / 3) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x3000
                    },
                );
                c.bus.write32(
                    OC763_INST + 0x228,
                    [0u32, 1, 1, 2][((seed / 2) % 4) as usize],
                );
                // Forced results: the loads fail now and then; the audio
                // object is missing now and then.
                c.bus.write32(
                    oc763_mail(0x0015_2bb0),
                    if seed % 5 == 4 { 1 } else { 0x77 + 3 },
                );
                c.bus.write32(
                    oc763_mail(0x0015_2de0),
                    if seed % 7 == 6 { 1 } else { 0x55 + 3 },
                );
                c.bus.write32(
                    oc763_mail(0x0028_b180),
                    if seed % 6 == 5 { 1 } else { OC763_AUDIO + 3 },
                );
                // `MemCardUI_ApplySoundMode`: audio -> +0x118 -> object ->
                // vtable owner +0x1d8 -> entry (+4) whose method is a fake.
                c.bus.write32(OC763_AUDIO + 0x118, OC763_P1);
                c.bus.write32(OC763_P1, OC763_P2);
                c.bus.write32(OC763_P2 + 0x1d8, OC763_P3);
                c.bus.write32(OC763_P3 + 4, OC763_P4);
                c.bus.write16(OC763_P4 + 0x10, 0);
                c.bus.write32(OC763_P4 + 0x14, fakes::FAKE + 0x60);
                fill(c, &mut r, OC763_CFG, 0xa4);
                c.pc = 0x001d_5f40;
            },
        );
    }
}
