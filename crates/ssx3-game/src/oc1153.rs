//! Staging module oc1153 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1153`.

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
    (0x003c_5bb8, snd_set_config43, "SND_SetConfig43"),
    (0x003c_5bf8, snd_set_config_c8, "SND_SetConfigC8"),
    (0x003c_5df0, snd_is_initialised, "SND_IsInitialised"),
    (0x003c_5e08, snd_shutdown_and_clear, "SND_ShutdownAndClear"),
    (0x003c_5e58, snd_get_init_info, "SND_GetInitInfo"),
    (0x003c_5ed0, snd_set_config64, "SND_SetConfig64"),
    (0x003c_5f18, snd_set_config_percent4b, "SND_SetConfigPercent4B"),
    (0x003c_5f70, snd_get_config_percent4b, "SND_GetConfigPercent4B"),
];

/// The EA SND library entry points these wrappers call (the generated
/// entries; each dispatches through its override slot).
mod cfg_calls {
    use super::{ee, Callee};

    /// `int SND_GetConfig(SndOptions* out)`: copies the options block out.
    pub const GET_CONFIG: Callee = Callee {
        addr: 0x003b_4dd8,
        f: ee::s13::SND_GetConfig,
    };
    /// `int SND_SetConfig(const SndOptions* cfg)`: writes the block back.
    pub const SET_CONFIG: Callee = Callee {
        addr: 0x003b_4fc0,
        f: ee::s13::SND_SetConfig,
    };
    /// `SND_Shutdown`: stops the driver when it is installed.
    pub const SHUTDOWN: Callee = Callee {
        addr: 0x003b_5320,
        f: ee::s13::SND_Shutdown,
    };
    /// `SND_IsInitialised`: whether the driver is installed.
    pub const IS_INITIALISED: Callee = Callee {
        addr: 0x003c_5df0,
        f: ee::s13::SND_IsInitialised,
    };
}

/// `int SND_SetConfig43(u8 value)` (0x3c5bb8): read the options block, store
/// `value` in its byte at +0x43 (the routing of the SPU effects to the IOP,
/// `routespufxtoiopcpu`), write the block back. Returns 0.
pub fn snd_set_config43(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        cfg_calls::GET_CONFIG.call(c, 0x003c_5bd0)?;
        let value = c.g[S0][0];
        Addr(frame.at(c, 0x43)).sb(c, value);
        c.g[A0][0] = c.g[SP][0];
        cfg_calls::SET_CONFIG.call(c, 0x003c_5bdc)?;
        frame.restore(c, S0, 0xf0);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SetConfigC8(u32 value)` (0x3c5bf8): as [`snd_set_config43`], with
/// the word `value` at the options block's +0xc8 (`abortmsg`). Returns 0.
pub fn snd_set_config_c8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        cfg_calls::GET_CONFIG.call(c, 0x003c_5c10)?;
        let value = c.g[S0][0];
        Addr(frame.at(c, 0xc8)).sw(c, value);
        c.g[A0][0] = c.g[SP][0];
        cfg_calls::SET_CONFIG.call(c, 0x003c_5c1c)?;
        frame.restore(c, S0, 0xf0);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_IsInitialised(void)` (0x3c5df0): 1 when the driver's installed
/// flag is exactly 1, else 0. Leaves `v1` at the SND block's high half.
pub fn snd_is_initialised(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x0051_0000;
    let installed = Addr((c.g[V1][0] as u32).wrapping_sub(0x55a4)).lb(c);
    c.g[V0][0] = ((installed ^ 1) < 1) as u64;
    ret(c);
}

/// `void SND_ShutdownAndClear(void)` (0x3c5e08): stop the driver, then zero
/// the heap pointer cell (`pheap`, 0x44e1c0).
pub fn snd_shutdown_and_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        cfg_calls::SHUTDOWN.call(c, 0x003c_5e18)?;
        c.g[V1][0] = 0x0045_0000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x1e40)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_GetInitInfo(int device, void** outHeap, u32* outSize)`
/// (0x3c5e58): both outputs are zeroed first. Device 0 while the driver is
/// installed returns the heap pointer and its size (0); device 0 while not
/// installed returns -4; any other device returns -5.
pub fn snd_get_init_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        let device_is_zero = c.g[A0][0] == 0;
        Addr(c.g[S1][0] as u32).sw(c, 0);
        if device_is_zero {
            c.cycles += 4;
            cfg_calls::IS_INITIALISED.call(c, 0x003c_5e84)?;
            let initialised = c.g[V0][0] != 0;
            c.g[V1][0] = 0x0045_0000;
            if initialised {
                c.cycles += 6;
                c.g[A0][0] = 0x0051_0000;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x1e40)).lw(c);
                let heap = c.g[V0][0];
                Addr(c.g[S0][0] as u32).sw(c, heap);
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x57b0)).lw(c);
                let size = c.g[V1][0];
                Addr(c.g[S1][0] as u32).sw(c, size);
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(0xffff_fffc);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(0xffff_fffb);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SetConfig64(u32 first, u32 second)` (0x3c5ed0): read the options
/// block, store `first` at its +0x64 and `second` at +0x68, write it back.
/// Returns 0.
pub fn snd_set_config64(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        frame.save(c, S1, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        cfg_calls::GET_CONFIG.call(c, 0x003c_5ef0)?;
        let first = c.g[S0][0];
        Addr(frame.at(c, 0x64)).sw(c, first);
        c.g[A0][0] = c.g[SP][0];
        let second = c.g[S1][0];
        Addr(frame.at(c, 0x68)).sw(c, second);
        cfg_calls::SET_CONFIG.call(c, 0x003c_5f00)?;
        frame.restore(c, S0, 0x100);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SetConfigPercent4B(int index, float percent)` (0x3c5f18): index 0
/// stores `percent` (as a whole number of hundredths) in the options byte at
/// +0x4b (the sound effects volume) and writes the block back, returning 0;
/// any other index returns -5.
pub fn snd_set_config_percent4b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x100);
        frame.save_fpr(c, 20, 0xf0);
        frame.save_ra(c, 0xe0);
        let index_zero = c.g[A0][0] == 0;
        let percent = abi::farg(c, 0);
        abi::set_fpr(c, 20, percent);
        if index_zero {
            c.cycles += 11;
            c.g[A0][0] = c.g[SP][0];
            cfg_calls::GET_CONFIG.call(c, 0x003c_5f3c)?;
            c.g[AT][0] = 0x42c8_0000;
            abi::set_fpr(c, 0, FpuF32(c.g[AT][0] as u32));
            c.g[A0][0] = c.g[SP][0];
            let scaled = FpuF32(c.f[20]) * FpuF32(c.f[0]);
            abi::set_fpr(c, 0, scaled);
            abi::set_fpr(c, 1, FpuF32(scaled.to_i32() as u32));
            c.g[V0][0] = abi::mfc1(FpuF32(c.f[1]));
            let byte = c.g[V0][0];
            Addr(frame.at(c, 0x4b)).sb(c, byte);
            cfg_calls::SET_CONFIG.call(c, 0x003c_5f5c)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(0xffff_fffb);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0xe0);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_GetConfigPercent4B(int index, float* outPercent)` (0x3c5f70):
/// zeroes `*outPercent`; index 0 reads the options byte at +0x4b and stores
/// it as a fraction (`byte * 0.01f`), returning 0; any other index returns -5.
pub fn snd_get_config_percent4b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        frame.save_ra(c, 0xe0);
        c.g[S0][0] = c.g[A1][0];
        let index_zero = c.g[A0][0] == 0;
        Addr(c.g[S0][0] as u32).sw(c, 0);
        if index_zero {
            c.cycles += 11;
            c.g[A0][0] = c.g[SP][0];
            cfg_calls::GET_CONFIG.call(c, 0x003c_5f98)?;
            c.g[V1][0] = Addr(frame.at(c, 0x4b)).lbu(c);
            c.g[V0][0] = 0;
            c.g[AT][0] = 0x3c23_d70a;
            abi::set_fpr(c, 1, FpuF32(c.g[AT][0] as u32));
            let byte = FpuF32::from_i32(c.g[V1][0] as u32 as i32);
            let fraction = byte * FpuF32(c.f[1]);
            abi::set_fpr(c, 0, fraction);
            let out = Addr(c.g[S0][0] as u32);
            out.write_fpu(c, fraction);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(0xffff_fffb);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA};

    /// The options block's "capabilities fetched" flag (`SND_GetConfig`).
    const GOT_CAPS: u32 = 0x0044_c45c;
    /// The driver's installed flag (`sndgs.installed`, read by the SND calls).
    const INSTALLED: u32 = 0x0050_aa5c;
    /// The options block at the start of the SND state.
    const OPTIONS: u32 = 0x0050_a8e8;
    /// The heap pointer cell and the heap size cell of the SND block.
    const PHEAP: u32 = 0x0044_e1c0;
    const HEAP_SIZE: u32 = 0x0051_57b0;

    /// A machine with the options already fetched (so `SND_GetConfig` does not
    /// load the defaults) and the driver not installed (so `SND_SetConfig` and
    /// `SND_Shutdown` stop at their first test).
    fn config_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GOT_CAPS, 1);
        c.bus.write8(INSTALLED, 0);
        r
    }

    #[test]
    fn snd_set_config43_matches() {
        check(
            ee::s13::f_003c5bb8__generated,
            snd_set_config43,
            200,
            &|c, seed| {
                let mut r = config_machine(c, seed, 0x43);
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn snd_set_config_c8_matches() {
        check(
            ee::s13::f_003c5bf8__generated,
            snd_set_config_c8,
            200,
            &|c, seed| {
                let mut r = config_machine(c, seed, 0xc8);
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn snd_is_initialised_matches() {
        check(
            ee::s13::f_003c5df0__generated,
            snd_is_initialised,
            200,
            &|c, seed| {
                machine(c, seed, 0x5df0, 0);
                // Installed flag: 0, 1 (the only true value), 2 and -1.
                c.bus
                    .write8(INSTALLED, [0u8, 1, 2, 0xff][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn snd_shutdown_and_clear_matches() {
        check(
            ee::s13::f_003c5e08__generated,
            snd_shutdown_and_clear,
            200,
            &|c, seed| {
                let mut r = config_machine(c, seed, 0x5e08);
                c.bus.write32(PHEAP, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_get_init_info_matches() {
        check(
            ee::s13::f_003c5e58__generated,
            snd_get_init_info,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5e58, 0);
                // Device 0 (installed or not) and devices 1 and up.
                c.g[A0][0] = if seed % 3 == 2 { 1 + seed as u64 } else { 0 };
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3010);
                c.bus.write32(DATA + 0x3000, r.next_u32());
                c.bus.write32(DATA + 0x3010, r.next_u32());
                c.bus.write8(INSTALLED, (seed % 2) as u8);
                c.bus.write32(PHEAP, r.next_u32());
                c.bus.write32(HEAP_SIZE, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_set_config64_matches() {
        check(
            ee::s13::f_003c5ed0__generated,
            snd_set_config64,
            200,
            &|c, seed| {
                let mut r = config_machine(c, seed, 0x5ed0);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn snd_set_config_percent4b_matches() {
        check(
            ee::s13::f_003c5f18__generated,
            snd_set_config_percent4b,
            200,
            &|c, seed| {
                let mut r = config_machine(c, seed, 0x5f18);
                // Index 0 (the store) and other indices (-5).
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { 1 + (seed % 5) as u64 };
                c.f[12] = if seed % 2 == 0 {
                    r.f32(400.0)
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn snd_get_config_percent4b_matches() {
        check(
            ee::s13::f_003c5f70__generated,
            snd_get_config_percent4b,
            200,
            &|c, seed| {
                let mut r = config_machine(c, seed, 0x5f70);
                // The options block is read back, so give its bytes values.
                fill(c, &mut r, OPTIONS, 0x35);
                c.bus.write8(INSTALLED, 0);
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { 1 + (seed % 5) as u64 };
                c.g[A1][0] = sx(DATA + 0x2000 + 4 * (seed % 4));
            },
        );
    }
}
