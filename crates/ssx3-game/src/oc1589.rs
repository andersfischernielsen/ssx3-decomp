//! Staging module oc1589 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1589`.

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
    (0x003e_5ff0, debug_output_direct, "Debug_OutputDirect"),
    (0x003e_6428, debug_console_output, "Debug_ConsoleOutput"),
    (0x003f_b490, crc32_init_state, "Crc32_Init"),
    (0x003f_b4c0, crc32_update_buffer, "Crc32_Update"),
    (0x003f_b558, crc32_final_out, "Crc32_Final"),
    (0x003f_b5b0, crc32_digest_size, "Crc32_GetDigestSize"),
];

/// `void Debug_OutputDirect(int index, int a1, int a2)` (0x3e5ff0, the
/// reference's `PRINT_todevice`): on the first call sets the debug channels up
/// (`Debug_InitChannels`, while the flag at `0x451008` is 0), then calls the
/// handler `fn` (the word at +4) of device entry `index` (12-byte entries at
/// `0x451250`) with `(a1, a2)`, if it has one.
pub fn debug_output_direct(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x450000;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1008).lw(c);
        c.g[S2][0] = c.g[A2][0];
        let initialised = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if !initialised {
            c.cycles += 2;
            Callee {
                addr: 0x003e_5b60,
                f: ee::s14::f_003e5b60,
            }
            .call(c, 0x003e_6024)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0xc;
        c.g[V1][0] = 0x450000;
        c.mult(0, A0, S0, V0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
        let has_handler = c.g[V1][0] != 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        if has_handler {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
            c.g[A0][0] = c.g[S1][0];
            let handler = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, handler, 0x003e_6054)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Debug_ConsoleOutput(int channel, char* text)` (0x3e6428, the
/// reference's `PRINT_debuggerfunc`): `printf(text)`, with the text in a1.
pub fn debug_console_output(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0041_75c8, 0x003e_6438)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Crc32_Init(void)` (0x3fb490; dossier name `func_003FB490`): starts a
/// CRC with the state at `0x451518` (`table` +0x0, `crc` +0xc) set to all ones.
/// Returns 6 (not initialised) when the table pointer is null, else 0 in v0.
pub fn crc32_init_state(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x450000;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1518).lw(c);
    let no_table = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1518));
    if no_table {
        c.cycles += 2;
        c.g[V0][0] = 6;
        ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0;
    let crc = c.g[V1][0];
    Addr(c.g[A0][0] as u32).off(0xc).sw(c, crc);
    ret(c);
}

/// `int Crc32_Update(const u8* buf, u32 n)` (0x3fb4c0): folds `n` bytes into
/// the running CRC of the state at `0x451518`: `crc = (crc >> 8) ^
/// table[(crc ^ buf[i]) & 0xff]`. Returns 6 without a table, 2 for a null
/// buffer, else 0. Back edges check for due events.
pub fn crc32_update_buffer(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x450000;
    c.g[T2][0] = c.g[A0][0];
    c.g[T4][0] = c.g[V0][0];
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1518).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1518));
    c.g[T1][0] = c.g[A1][0];
    let has_table = c.g[V1][0] != 0;
    c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
    if !has_table {
        c.cycles += 2;
        c.g[V0][0] = 6;
        ret(c);
        return;
    }
    c.cycles += 2;
    let null_buf = c.g[T2][0] == 0;
    c.g[V0][0] = 0x2;
    if null_buf {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A2][0] = 0;
    if c.g[T1][0] != 0 {
        c.cycles += 4;
        c.g[T0][0] = 0xff0000;
        c.g[T3][0] = c.g[V1][0];
        c.g[T0][0] = c.g[T0][0] | 0xffff;
        loop {
            c.cycles += 13;
            c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A0][0] = sx((c.g[A3][0] as u32) >> 8);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
            c.g[A0][0] = c.g[A0][0] & c.g[T0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.g[A3][0] ^ c.g[V0][0];
            c.g[A1][0] = (c.g[A2][0] < c.g[T1][0]) as u64;
            c.g[V0][0] = c.g[V0][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T3][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let more = c.g[A1][0] != 0;
            c.g[A3][0] = c.g[A0][0] ^ c.g[V1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_b508);
            }
        }
    }
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1518));
    c.g[V0][0] = 0;
    let crc = c.g[A3][0];
    Addr(c.g[V1][0] as u32).off(0xc).sw(c, crc);
    ret(c);
}

/// `int Crc32_Final(u32** out)` (0x3fb558): stores the finished CRC in the
/// global `D_004A5B1C` (`gp + 0x2a2c`, the inverted state) and writes
/// `*out = &state.f04` (state `f04` = 4, `f08` = the global). Returns 6
/// without a table, 2 for a null `out`, else 0.
pub fn crc32_final_out(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x450000;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1518).lw(c);
    let has_table = c.g[V0][0] != 0;
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1518));
    if !has_table {
        c.cycles += 2;
        c.g[V0][0] = 6;
        ret(c);
        return;
    }
    c.cycles += 2;
    let null_out = c.g[A2][0] == 0;
    c.g[A0][0] = 0x4;
    if null_out {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 10;
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2a2c));
    let global = c.g[V0][0];
    Addr(c.g[A1][0] as u32).off(0x8).sw(c, global);
    let four = c.g[A0][0];
    Addr(c.g[A1][0] as u32).off(0x4).sw(c, four);
    c.g[V1][0] = !c.g[V1][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
    let inverted = c.g[V1][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0x2a2c)).sw(c, inverted);
    let f04 = c.g[V0][0];
    Addr(c.g[A2][0] as u32).sw(c, f04);
    c.g[V0][0] = 0;
    ret(c);
}

/// `int Crc32_GetDigestSize(u32* out)` (0x3fb5b0): `*out = 4`. Returns 6
/// without a table, 2 for a null `out`, else 0.
pub fn crc32_digest_size(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x450000;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1518).lw(c);
    let no_table = c.g[V0][0] == 0;
    c.g[V0][0] = 0x6;
    if no_table {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let null_out = c.g[A0][0] == 0;
    c.g[V1][0] = 0x4;
    if null_out {
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0;
    let four = c.g[V1][0];
    Addr(c.g[A0][0] as u32).sw(c, four);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA};
    use std::ptr::addr_of_mut;

    /// Debug channel flag (0x451008) and the device entries (0x451250, 12 bytes each).
    const DEBUG_INIT_FLAG: u32 = 0x0045_1008;
    const DEBUG_DEVICES: u32 = 0x0045_1250;
    /// CRC state (0x451518) and the fixture's table, buffer and output.
    const CRC_STATE: u32 = 0x0045_1518;
    const CRC_TABLE: u32 = DATA + 0x3000;
    const CRC_BUF: u32 = DATA + 0x4000;
    const CRC_OUT: u32 = DATA + 0x5000;
    /// Handler address for device entries: not a function, so the miss hook
    /// (`fake_call`) takes the call.
    const DEVICE_HANDLER: u32 = DATA + 0x2000;

    /// Gate words of this module's fakes (own address and magic in the
    /// machine's RAM: other tests' machines run the real bodies).
    const OC1589_PRINTF_GATE: u32 = DATA + 0x2f_1580;
    const OC1589_PRINTF_MAGIC: u32 = 0x1589_0001;
    const OC1589_INIT_GATE: u32 = DATA + 0x2f_1584;
    const OC1589_INIT_MAGIC: u32 = 0x1589_0002;

    fn oc1589_no_prev(_c: &mut Cpu) {
        unreachable!("oc1589 fake called before it was hooked");
    }

    /// Gated fake of `printf` (0x4175c8): logs and returns.
    fn oc1589_fake_printf(c: &mut Cpu) {
        if c.bus.read32(OC1589_PRINTF_GATE) != OC1589_PRINTF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1589_fake_printf),
                oc1589_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0041_75c8, ra);
    }

    /// Gated fake of `Debug_InitChannels` (0x3e5b60): logs and returns.
    fn oc1589_fake_init_channels(c: &mut Cpu) {
        if c.bus.read32(OC1589_INIT_GATE) != OC1589_INIT_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1589_fake_init_channels),
                oc1589_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003e_5b60, ra);
    }

    /// A CRC state with a table (or none), a random running value, a buffer of
    /// random bytes and a table of random words.
    fn crc_fixture(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        c.bus
            .write32(CRC_STATE, if seed % 11 == 0 { 0 } else { CRC_TABLE });
        c.bus.write32(CRC_STATE + 0xc, r.next_u32());
        fill(c, r, CRC_TABLE, 256);
        fill(c, r, CRC_BUF, 16);
    }

    /// `Debug_OutputDirect`: both init states, devices with and without a
    /// handler, and a handler call that may unwind.
    #[test]
    fn debug_output_direct_matches() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5B60),
            oc1589_fake_init_channels,
            fakes::prev_cell(oc1589_fake_init_channels),
        );
        check(
            ee::s14::f_003e5ff0__generated,
            debug_output_direct,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e5ff0, 2);
                c.bus.write32(OC1589_INIT_GATE, OC1589_INIT_MAGIC);
                c.bus
                    .write32(DEBUG_INIT_FLAG, if seed % 2 == 0 { 0 } else { 1 });
                let index = seed % 4;
                let entry = DEBUG_DEVICES + 12 * index;
                c.bus.write32(
                    entry + 4,
                    if seed % 5 == 0 { 0 } else { DEVICE_HANDLER },
                );
                c.g[A0][0] = index as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    /// `Debug_ConsoleOutput`: the text goes to `printf`, which is faked.
    #[test]
    fn debug_console_output_matches() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004175C8),
            oc1589_fake_printf,
            fakes::prev_cell(oc1589_fake_printf),
        );
        check(
            ee::s14::f_003e6428__generated,
            debug_console_output,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6428, 2);
                c.bus.write32(OC1589_PRINTF_GATE, OC1589_PRINTF_MAGIC);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = if seed % 6 == 0 {
                    0
                } else {
                    sx(DATA + 0x100)
                };
            },
        );
    }

    /// `Crc32_Init`: with and without a table, the running value overwritten.
    #[test]
    fn crc32_init_state_matches() {
        check(
            ee::s15::f_003fb490__generated,
            crc32_init_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fb490, 0);
                c.bus
                    .write32(CRC_STATE, if seed % 3 == 0 { 0 } else { CRC_TABLE });
                c.bus.write32(CRC_STATE + 0xc, r.next_u32());
            },
        );
    }

    /// `Crc32_Update`: no table, null buffer, zero and several bytes.
    #[test]
    fn crc32_update_buffer_matches() {
        check(
            ee::s15::f_003fb4c0__generated,
            crc32_update_buffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fb4c0, 0);
                crc_fixture(c, &mut r, seed);
                c.g[A0][0] = if seed % 7 == 0 { 0 } else { sx(CRC_BUF) };
                c.g[A1][0] = (seed % 9) as u64;
            },
        );
    }

    /// `Crc32_Update` with event checks at its back edges.
    #[test]
    fn crc32_update_buffer_events() {
        check_events(
            ee::s15::f_003fb4c0__generated,
            crc32_update_buffer,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fb4c1, 0);
                crc_fixture(c, &mut r, seed);
                c.g[A0][0] = sx(CRC_BUF);
                c.g[A1][0] = 2 + (seed % 7) as u64;
                schedule(c, &mut r, seed, 0x003f_b4c0)
            },
        );
    }

    /// `Crc32_Final`: no table, null out, and the store into the global.
    #[test]
    fn crc32_final_out_matches() {
        check(
            ee::s15::f_003fb558__generated,
            crc32_final_out,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fb558, 0);
                c.bus
                    .write32(CRC_STATE, if seed % 4 == 0 { 0 } else { CRC_TABLE });
                c.bus.write32(CRC_STATE + 0xc, r.next_u32());
                c.g[A0][0] = if seed % 6 == 0 { 0 } else { sx(CRC_OUT) };
            },
        );
    }

    /// `Crc32_GetDigestSize`: no table, null out, and the stored size.
    #[test]
    fn crc32_digest_size_matches() {
        check(
            ee::s15::f_003fb5b0__generated,
            crc32_digest_size,
            200,
            &|c, seed| {
                machine(c, seed, 0x3fb5b0, 0);
                c.bus
                    .write32(CRC_STATE, if seed % 4 == 0 { 0 } else { CRC_TABLE });
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(CRC_OUT) };
            },
        );
    }
}
