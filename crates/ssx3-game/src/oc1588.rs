//! Staging module oc1588 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1588`.

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
    (0x003e_5d30, debug_printf, "Debug_Printf"),
    (0x003e_5d78, debug_printf_channel2, "Debug_PrintfChannel2"),
    (0x003e_5dc8, debug_enable_output, "Debug_EnableOutput"),
    (0x003e_5e38, debug_is_output_enabled, "Debug_IsOutputEnabled"),
    (0x003e_5e88, debug_set_output, "Debug_SetOutput"),
    (0x003e_5ef8, debug_enable_channel, "Debug_EnableChannel"),
    (0x003e_5f48, debug_is_channel_enabled, "Debug_IsChannelEnabled"),
    (0x003e_5f98, debug_set_channel_name, "Debug_SetChannelName"),
];

/// The debug globals (`lui 0x450000`): the output device and channel tables
/// and the initialised flag.
pub(crate) const DEBUG_GLOBALS: u32 = 0x0045_0000;
/// Word that is non-zero once `Debug_InitChannels` has run.
pub(crate) const DEBUG_INIT_FLAG: u32 = 0x0045_1008;
/// Device table: 12-byte entries `{ name, output func, enabled }`.
pub(crate) const DEBUG_DEVICE_TABLE: u32 = 0x0045_1250;
/// Channel table: 8-byte entries `{ name, flags }` (bit 0 = enabled).
pub(crate) const DEBUG_CHANNEL_TABLE: u32 = 0x0045_0e08;

/// `Debug_VPrintf(channel, fmt, va_list)` (0x3e5c78).
pub(crate) const DEBUG_VPRINTF: Callee = Callee {
    addr: 0x003e_5c78,
    f: ee::s14::f_003e5c78,
};
/// `Debug_InitChannels()` (0x3e5b60).
pub(crate) const DEBUG_INIT_CHANNELS: Callee = Callee {
    addr: 0x003e_5b60,
    f: ee::s14::f_003e5b60,
};
/// `Debug_SetChannelFlag(device, enable)` (0x3e5a78).
pub(crate) const DEBUG_SET_CHANNEL_FLAG: Callee = Callee {
    addr: 0x003e_5a78,
    f: ee::s14::f_003e5a78,
};
/// `Debug_SetChannelState(channel, enable)` (0x3e5aa8).
pub(crate) const DEBUG_SET_CHANNEL_STATE: Callee = Callee {
    addr: 0x003e_5aa8,
    f: ee::s14::f_003e5aa8,
};

/// `sd reg, off(sp)` for a register the function does not preserve.
pub(crate) fn debug_save_gpr(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    c.sd(a, c.g[reg][0]);
}

/// `void Debug_Printf(int channel, const char* fmt, ...)` (0x3e5d30): spills
/// the variadic argument registers and float varargs to the frame (the va
/// list starts at sp+0x50) and forwards to `Debug_VPrintf`.
pub fn debug_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        debug_save_gpr(c, frame, A2, 0x50);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(frame.at(c, 0x50));
        debug_save_gpr(c, frame, A3, 0x58);
        debug_save_gpr(c, frame, T0, 0x60);
        debug_save_gpr(c, frame, T1, 0x68);
        debug_save_gpr(c, frame, T2, 0x70);
        debug_save_gpr(c, frame, T3, 0x78);
        frame.save_fpr(c, 12, 0x40);
        frame.save_fpr(c, 14, 0x44);
        frame.save_fpr(c, 16, 0x48);
        frame.save_fpr(c, 18, 0x4c);
        DEBUG_VPRINTF.call(c, 0x003e_5d68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Debug_PrintfChannel2(const char* fmt, ...)` (0x3e5d78): as
/// `Debug_Printf` on channel 2, the format moved to a1 and the variadic
/// arguments spilled from a1 on (the va list starts at sp+0x58).
pub fn debug_printf_channel2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        debug_save_gpr(c, frame, A1, 0x58);
        debug_save_gpr(c, frame, A2, 0x60);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2;
        debug_save_gpr(c, frame, A3, 0x68);
        c.g[A2][0] = sx(frame.at(c, 0x58));
        debug_save_gpr(c, frame, T0, 0x70);
        debug_save_gpr(c, frame, T1, 0x78);
        debug_save_gpr(c, frame, T2, 0x80);
        debug_save_gpr(c, frame, T3, 0x88);
        frame.save_fpr(c, 12, 0x48);
        frame.save_fpr(c, 14, 0x4c);
        frame.save_fpr(c, 16, 0x50);
        frame.save_fpr(c, 18, 0x54);
        DEBUG_VPRINTF.call(c, 0x003e_5dbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Debug_EnableOutput(int device, int enable)` (0x3e5dc8): for a device
/// that has an output function, initialises the channel tables if needed and
/// sets the device's enabled flag. Otherwise does nothing.
pub fn debug_enable_output(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = DEBUG_GLOBALS as u64;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx(DEBUG_DEVICE_TABLE);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        c.g[A0][0] = 0xc;
        frame.save_ra(c, 0);
        let device = c.g[S0][0] as u32;
        c.g[A0][0] = sx(crate::abi::mult(c, device, 0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = DEBUG_GLOBALS as u64;
            c.g[V1][0] = Addr(DEBUG_INIT_FLAG).lw(c);
            let inited = c.g[V1][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !inited {
                c.cycles += 3;
                DEBUG_INIT_CHANNELS.call(c, 0x003e_5e14)?;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            DEBUG_SET_CHANNEL_FLAG.call(c, 0x003e_5e20)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool Debug_IsOutputEnabled(int device)` (0x3e5e38): the device's enabled
/// flag (bit 0 of its table entry), after initialising the tables if needed.
pub fn debug_is_output_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = DEBUG_GLOBALS as u64;
        frame.save(c, 16, 0x10);
        c.g[V0][0] = Addr(DEBUG_INIT_FLAG).lw(c);
        c.g[S0][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !inited {
            c.cycles += 2;
            DEBUG_INIT_CHANNELS.call(c, 0x003e_5e5c)?;
        }
        c.cycles += 11;
        c.g[V1][0] = 0xc;
        c.g[V0][0] = DEBUG_GLOBALS as u64;
        let device = c.g[S0][0] as u32;
        c.g[V1][0] = sx(crate::abi::mult(c, device, 0xc));
        c.g[V0][0] = sx(DEBUG_DEVICE_TABLE);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        frame.close(c);
        Ok(())
    })
}

/// `void Debug_SetOutput(int device, const char* name, void* output)`
/// (0x3e5e88): stores the device's name and output function, after
/// initialising the tables if needed.
pub fn debug_set_output(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = DEBUG_GLOBALS as u64;
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(DEBUG_INIT_FLAG).lw(c);
        c.g[S2][0] = c.g[A2][0];
        let inited = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if !inited {
            c.cycles += 2;
            DEBUG_INIT_CHANNELS.call(c, 0x003e_5ebc)?;
        }
        c.cycles += 14;
        c.g[V1][0] = 0xc;
        c.g[V0][0] = DEBUG_GLOBALS as u64;
        let device = c.g[S0][0] as u32;
        c.g[V1][0] = sx(crate::abi::mult(c, device, 0xc));
        c.g[V0][0] = sx(DEBUG_DEVICE_TABLE);
        frame.restore(c, 16, 0x30);
        frame.restore_ra(c, 0);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let entry = c.g[A0][0] as u32;
        let name = c.g[S1][0];
        Addr(entry).sw(c, name);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let output = c.g[S2][0];
        Addr((c.g[V0][0] as u32).wrapping_add(4)).sw(c, output);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Debug_EnableChannel(int channel, int enable)` (0x3e5ef8): after
/// initialising the tables if needed, sets the channel's state.
pub fn debug_enable_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = DEBUG_GLOBALS as u64;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(DEBUG_INIT_FLAG).lw(c);
        c.g[S1][0] = c.g[A0][0];
        let inited = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if !inited {
            c.cycles += 2;
            DEBUG_INIT_CHANNELS.call(c, 0x003e_5f24)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        DEBUG_SET_CHANNEL_STATE.call(c, 0x003e_5f30)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool Debug_IsChannelEnabled(int channel)` (0x3e5f48): bit 0 of the
/// channel's flags, after initialising the tables if needed.
pub fn debug_is_channel_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = DEBUG_GLOBALS as u64;
        frame.save(c, 16, 0x10);
        c.g[V0][0] = Addr(DEBUG_INIT_FLAG).lw(c);
        c.g[S0][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !inited {
            c.cycles += 2;
            DEBUG_INIT_CHANNELS.call(c, 0x003e_5f6c)?;
        }
        c.cycles += 10;
        c.g[V0][0] = sx(DEBUG_CHANNEL_TABLE);
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
        frame.restore(c, 16, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        frame.close(c);
        Ok(())
    })
}

/// `void Debug_SetChannelName(int channel, const char* name)` (0x3e5f98):
/// stores the channel's name, after initialising the tables if needed.
pub fn debug_set_channel_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = DEBUG_GLOBALS as u64;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(DEBUG_INIT_FLAG).lw(c);
        c.g[S1][0] = c.g[A1][0];
        let inited = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if !inited {
            c.cycles += 2;
            DEBUG_INIT_CHANNELS.call(c, 0x003e_5fc4)?;
        }
        c.cycles += 10;
        c.g[V0][0] = sx(DEBUG_CHANNEL_TABLE);
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
        frame.restore(c, 16, 0x20);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        let entry = c.g[V1][0] as u32;
        let name = c.g[S1][0];
        Addr(entry).sw(c, name);
        frame.restore(c, 17, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, DATA};
    use std::ptr::addr_of_mut;

    /// Gates of this module's callee fakes (one word per fake).
    pub(crate) const OC1588_GATE_VPRINTF: u32 = DATA + 0x0020_0000;
    pub(crate) const OC1588_GATE_INIT: u32 = DATA + 0x0020_0010;
    pub(crate) const OC1588_GATE_FLAG: u32 = DATA + 0x0020_0020;
    pub(crate) const OC1588_GATE_STATE: u32 = DATA + 0x0020_0030;
    pub(crate) const OC1588_MAGIC: u32 = 0x1588_0001;

    pub(crate) fn oc1588_fake_vprintf(c: &mut Cpu) {
        if c.bus.read32(OC1588_GATE_VPRINTF) != OC1588_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1588_fake_vprintf),
                ee::s14::f_003e5c78__generated,
            );
        }
        fakes::fake_call(c, 0x003e_5c78);
    }

    pub(crate) fn oc1588_fake_init_channels(c: &mut Cpu) {
        if c.bus.read32(OC1588_GATE_INIT) != OC1588_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1588_fake_init_channels),
                ee::s14::f_003e5b60__generated,
            );
        }
        fakes::fake_call(c, 0x003e_5b60);
    }

    pub(crate) fn oc1588_fake_set_channel_flag(c: &mut Cpu) {
        if c.bus.read32(OC1588_GATE_FLAG) != OC1588_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1588_fake_set_channel_flag),
                ee::s14::f_003e5a78__generated,
            );
        }
        fakes::fake_call(c, 0x003e_5a78);
    }

    pub(crate) fn oc1588_fake_set_channel_state(c: &mut Cpu) {
        if c.bus.read32(OC1588_GATE_STATE) != OC1588_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1588_fake_set_channel_state),
                ee::s14::f_003e5aa8__generated,
            );
        }
        fakes::fake_call(c, 0x003e_5aa8);
    }

    /// Installs the four gated fakes in their override slots (once per process).
    pub(crate) fn oc1588_hook_callees() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5C78),
            oc1588_fake_vprintf,
            fakes::prev_cell(oc1588_fake_vprintf),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5B60),
            oc1588_fake_init_channels,
            fakes::prev_cell(oc1588_fake_init_channels),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5A78),
            oc1588_fake_set_channel_flag,
            fakes::prev_cell(oc1588_fake_set_channel_flag),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5AA8),
            oc1588_fake_set_channel_state,
            fakes::prev_cell(oc1588_fake_set_channel_state),
        );
    }

    /// Opens all four gates in this machine.
    pub(crate) fn oc1588_open_gates(c: &mut Cpu) {
        for g in [
            OC1588_GATE_VPRINTF,
            OC1588_GATE_INIT,
            OC1588_GATE_FLAG,
            OC1588_GATE_STATE,
        ] {
            c.bus.write32(g, OC1588_MAGIC);
        }
    }

    /// Sets the initialised flag: zero for the `Debug_InitChannels` path.
    pub(crate) fn oc1588_init_flag(c: &mut Cpu, seed: u32, zero_every: u32) {
        c.bus.write32(
            DEBUG_INIT_FLAG,
            if seed % zero_every == 0 { 0 } else { 1 },
        );
    }

    #[test]
    fn debug_printf_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5d30__generated,
            debug_printf,
            200,
            &[0x003e_5d68],
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003e_5d30, 1);
                oc1588_open_gates(c);
            },
        );
    }

    #[test]
    fn debug_printf_channel2_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5d78__generated,
            debug_printf_channel2,
            200,
            &[0x003e_5dbc],
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003e_5d78, 1);
                oc1588_open_gates(c);
            },
        );
    }

    #[test]
    fn debug_enable_output_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5dc8__generated,
            debug_enable_output,
            300,
            &[0x003e_5e14, 0x003e_5e20],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003e_5dc8, 2);
                oc1588_open_gates(c);
                // device 0..3; zero output function (no call), set one
                // (with or without the init call)
                let device = r.next_u32() % 4;
                let b = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(DEBUG_DEVICE_TABLE + 12 * device + 4, b);
                oc1588_init_flag(c, seed, 3);
                c.g[A0][0] = sx(device);
            },
        );
    }

    #[test]
    fn debug_is_output_enabled_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5e38__generated,
            debug_is_output_enabled,
            300,
            &[0x003e_5e5c],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003e_5e38, 1);
                oc1588_open_gates(c);
                let device = r.next_u32() % 4;
                c.bus.write32(DEBUG_DEVICE_TABLE + 12 * device + 8, r.next_u32());
                oc1588_init_flag(c, seed, 2);
                c.g[A0][0] = sx(device);
            },
        );
    }

    #[test]
    fn debug_set_output_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5e88__generated,
            debug_set_output,
            300,
            &[0x003e_5ebc],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003e_5e88, 1);
                oc1588_open_gates(c);
                let device = r.next_u32() % 4;
                oc1588_init_flag(c, seed, 2);
                c.g[A0][0] = sx(device);
                c.g[A1][0] = sx(DATA + 0x2000 + 4 * r.next_u32() % 256);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn debug_enable_channel_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5ef8__generated,
            debug_enable_channel,
            300,
            &[0x003e_5f24, 0x003e_5f30],
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003e_5ef8, 2);
                oc1588_open_gates(c);
                oc1588_init_flag(c, seed, 3);
            },
        );
    }

    #[test]
    fn debug_is_channel_enabled_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5f48__generated,
            debug_is_channel_enabled,
            300,
            &[0x003e_5f6c],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003e_5f48, 1);
                oc1588_open_gates(c);
                let channel = r.next_u32() % 8;
                c.bus.write32(DEBUG_CHANNEL_TABLE + 8 * channel + 4, r.next_u32());
                oc1588_init_flag(c, seed, 2);
                c.g[A0][0] = sx(channel);
            },
        );
    }

    #[test]
    fn debug_set_channel_name_matches() {
        oc1588_hook_callees();
        fakes::check_reaching(
            ee::s14::f_003e5f98__generated,
            debug_set_channel_name,
            300,
            &[0x003e_5fc4],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003e_5f98, 1);
                oc1588_open_gates(c);
                let channel = r.next_u32() % 8;
                oc1588_init_flag(c, seed, 2);
                c.g[A0][0] = sx(channel);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }
}
