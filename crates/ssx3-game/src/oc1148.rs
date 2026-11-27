//! Staging module oc1148 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1148`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `SND_GetConfig(SndOptions* out)` (0x3b4dd8), the settings read.
pub(crate) const OC1148_GET_CONFIG: Callee = Callee {
    addr: 0x003b_4dd8,
    f: ee::s13::SND_GetConfig,
};

/// `SND_SetConfig(const SndOptions* cfg)` (0x3b4fc0), the settings write.
pub(crate) const OC1148_SET_CONFIG: Callee = Callee {
    addr: 0x003b_4fc0,
    f: ee::s13::SND_SetConfig,
};

/// The one-field setters share one body: `SndOptions cfg; SND_GetConfig(&cfg);`
/// store `a0` at `cfg + field` (`store`), `SND_SetConfig(&cfg); return 0`.
/// Frame 0x100: `ra` at +0xe0, `s0` (the value) at +0xf0. The two returns are
/// the generated call sites.
fn oc1148_set_config_field(
    c: &mut Cpu,
    field: u32,
    store: fn(&mut Cpu, u32, u64),
    get_ret: u32,
    set_ret: u32,
) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        OC1148_GET_CONFIG.call(c, get_ret)?;
        let at = frame.at(c, field);
        let value = c.g[S0][0];
        store(c, at, value);
        c.g[A0][0] = c.g[SP][0];
        OC1148_SET_CONFIG.call(c, set_ret)?;
        frame.restore(c, S0, 0xf0);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_SetConfig46(u8 v)` (0x3c5368): read the settings, set the byte at
/// +0x46 to `a0`, write them back. Returns 0.
pub fn snd_set_config46(c: &mut Cpu) {
    oc1148_set_config_field(
        c,
        0x46,
        |c, a, v| Addr(a).sb(c, v),
        0x003c_5380,
        0x003c_538c,
    )
}

/// `void SND_SetConfig3C(u8 v)` (0x3c53a0): as `SND_SetConfig46`, byte at +0x3c.
pub fn snd_set_config3c(c: &mut Cpu) {
    oc1148_set_config_field(
        c,
        0x3c,
        |c, a, v| Addr(a).sb(c, v),
        0x003c_53b8,
        0x003c_53c4,
    )
}

/// `void SND_SetConfig24(u32 v)` (0x3c53d8): as `SND_SetConfig46`, word at +0x24.
pub fn snd_set_config24(c: &mut Cpu) {
    oc1148_set_config_field(
        c,
        0x24,
        |c, a, v| Addr(a).sw(c, v),
        0x003c_53f0,
        0x003c_53fc,
    )
}

/// `void SND_SetServiceRate(float hz)` (0x3c5410): the service period in ms is
/// `1000 / hz` for a rate that is not negative (`0 <= hz`, NaN excluded), else
/// the rate itself. The value goes to the settings block (+0x34) and is written
/// back with `SND_SetConfig`. `f12` is the argument; `f20` is saved and restored.
pub fn snd_set_service_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x100);
        c.mtc1(0, 0);
        frame.save_fpr(c, 20, 0xf0);
        let rate = FpuF32(c.f[12]);
        abi::set_fpr(c, 20, rate);
        let not_negative = FpuF32(c.f[0]).le(FpuF32(c.f[20]));
        abi::set_fcc(c, not_negative);
        frame.save_ra(c, 0xe0);
        if not_negative {
            c.cycles += 3;
            c.g[AT][0] = 0x447a_0000;
            c.mtc1(1, 0);
            let period = FpuF32(c.f[0]).div(FpuF32(c.f[20]));
            abi::set_fpr(c, 20, period);
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[SP][0];
        OC1148_GET_CONFIG.call(c, 0x003c_5444)?;
        frame.save_fpr(c, 20, 0x34);
        c.g[A0][0] = c.g[SP][0];
        OC1148_SET_CONFIG.call(c, 0x003c_5450)?;
        frame.restore_ra(c, 0xe0);
        c.g[V0][0] = 0;
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `float SND_GetServiceRate(float* out)` (0x3c5468): read the settings, store
/// the stored rate (+0x34) to `*out`, and when it is not negative also store
/// `1000 / rate` (the period). Returns 0.
pub fn snd_get_service_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        OC1148_GET_CONFIG.call(c, 0x003c_5480)?;
        frame.restore_fpr(c, 1, 0x34);
        c.mtc1(0, 0);
        let out = c.g[S0][0] as u32;
        let not_negative = FpuF32(c.f[0]).le(FpuF32(c.f[1]));
        abi::set_fcc(c, not_negative);
        c.swc1(1, out);
        if not_negative {
            c.cycles += 4;
            c.g[AT][0] = 0x447a_0000;
            c.mtc1(1, 0);
            let period = FpuF32(c.f[0]).div(FpuF32(c.f[1]));
            abi::set_fpr(c, 0, period);
            c.swc1(0, out);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0xf0);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SetOutputMode(int mode)` (0x3c5600): the mode must be supported
/// (`SND_IsOutputModeSupported`, which reports through the frame), then it is
/// stored at 0x44e1c4. Returns 0, or -5 when the mode is not supported.
pub fn snd_set_output_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x003c_54c0,
            f: ee::s13::SND_IsOutputModeSupported,
        }
        .call(c, 0x003c_5618)?;
        let sp_now = sp(c);
        let supported = Addr(sp_now).lw(c);
        let unsupported = supported == 0;
        c.g[V1][0] = 0x450000;
        if unsupported {
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_fffb);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0;
            let mode = c.g[S0][0];
            let v1 = c.g[V1][0] as u32;
            Addr(v1.wrapping_sub(0x1e3c)).sw(c, mode);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_RoundSampleRate(int mode, int rate, int* out)` (0x3c5660): with the
/// settings read first, `mode` 0 stores the rate rounded to 24000, 36000 or
/// 48000 (below 24001, up to 36000, above) to `*out`; `mode` 1 stores the same
/// rounded value on another path; any other mode returns -5. The block layout
/// is the generated one (labelled blocks, one exit per jump), since the paths
/// differ in cycles and in the scratch registers they leave.
pub fn snd_round_sample_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S2, 0xf0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0xe0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        OC1148_GET_CONFIG.call(c, 0x003c_5688)?;
        'a: {
            'b: {
                'c: {
                    'd: {
                        'e: {
                            'f: {
                                'g: {
                                    'h: {
                                        let s2_zero = c.g[S2][0] == 0;
                                        c.g[V0][0] = 1;
                                        if s2_zero {
                                            break 'h;
                                        }
                                        c.cycles += 2;
                                        let s2_one = c.g[S2][0] == c.g[V0][0];
                                        c.g[V0][0] = sx(0xffff_fffb);
                                        if s2_one {
                                            break 'g;
                                        }
                                        c.cycles += 2;
                                        frame.restore(c, S0, 0x110);
                                        break 'a;
                                    }
                                    // Mode 0: round the rate.
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 24001) as u64;
                                    let below = c.g[V0][0] != 0;
                                    c.g[V0][0] = 0x5dc0;
                                    if below {
                                        break 'f;
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = 0x8ca0;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                                    let up_to_36000 = c.g[V0][0] == 0;
                                    c.g[V0][0] = 0xbb80;
                                    if up_to_36000 {
                                        break 'd;
                                    }
                                    c.cycles += 2;
                                    let s1 = c.g[S1][0] as u32;
                                    let v = c.g[V0][0];
                                    Addr(s1).sw(c, v);
                                    break 'b;
                                }
                                // Mode 1: the same rounding, another path.
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[S0][0] as i64) < 24001) as u64;
                                let at_least_24001 = c.g[V0][0] == 0;
                                c.g[V0][0] = 0x5dc0;
                                if at_least_24001 {
                                    break 'e;
                                }
                                break 'f;
                            }
                            // Below 24001 (either mode): 24000.
                            c.cycles += 2;
                            let s1 = c.g[S1][0] as u32;
                            let v = c.g[V0][0];
                            Addr(s1).sw(c, v);
                            break 'b;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = 0x8ca0;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        let above_36000 = c.g[V0][0] != 0;
                        c.g[V0][0] = 0xbb80;
                        if above_36000 {
                            break 'c;
                        }
                        break 'd;
                    }
                    // Up to 36000: store 36000 (V1).
                    c.cycles += 2;
                    let s1 = c.g[S1][0] as u32;
                    let v1 = c.g[V1][0];
                    Addr(s1).sw(c, v1);
                    break 'b;
                }
                // Above 36000: store 48000 (V0), then return 0.
                c.cycles += 1;
                let s1 = c.g[S1][0] as u32;
                let v0 = c.g[V0][0];
                Addr(s1).sw(c, v0);
                break 'b;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x110);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_SetSampleRate(int mode, int rate)` (0x3c5710): the rate is capped
/// at 0xffff (when it is above it), stored as a halfword in the settings (+0x2a
/// for mode 0, +0x2c for mode 1), written back with `SND_SetConfig`. Returns 0.
/// Other modes return -5 and write nothing back.
pub fn snd_set_sample_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S1, 0xf0);
        frame.save(c, S0, 0x100);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        OC1148_GET_CONFIG.call(c, 0x003c_5730)?;
        c.g[V1][0] = 0xffff;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = c.g[V1][0];
        }
        c.g[A0][0] = c.g[S0][0] & 0xff;
        let sp_now = sp(c);
        let low_byte = c.g[A0][0];
        Addr(sp_now.wrapping_add(0x2a)).sh(c, low_byte);
        'a: {
            'b: {
                'c: {
                    'd: {
                        if c.g[S1][0] == 0 {
                            break 'd;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        let mode_one = c.g[S1][0] == c.g[V0][0];
                        c.g[V0][0] = sx(0xffff_fffb);
                        if mode_one {
                            break 'c;
                        }
                        c.cycles += 2;
                        frame.restore(c, S0, 0x100);
                        break 'a;
                    }
                    // Mode 0: the capped rate, halfword at +0x2a.
                    c.cycles += 2;
                    let sp_now = sp(c);
                    let rate = c.g[S0][0];
                    Addr(sp_now.wrapping_add(0x2a)).sh(c, rate);
                    break 'b;
                }
                // Mode 1: the capped rate, halfword at +0x2c.
                c.cycles += 1;
                let sp_now = sp(c);
                let rate = c.g[S0][0];
                Addr(sp_now.wrapping_add(0x2c)).sh(c, rate);
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            OC1148_SET_CONFIG.call(c, 0x003c_5770)?;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x100);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, fill, Rng, DATA};

    /// The SND state the settings functions read and write (`snd.rs`), the
    /// capabilities-fetched flag, and the lock hooks the library calls through.
    const OC1148_SND_STATE: u32 = 0x0050_a8e8;
    const OC1148_GOT_CAPS: u32 = 0x0044_c45c;
    const OC1148_LOCK_HOOK: u32 = 0x0044_c46c;
    const OC1148_UNLOCK_HOOK: u32 = 0x0044_c470;

    /// A machine with the settings state filled, the capabilities fetched (so
    /// `SND_GetConfig` copies the state instead of loading defaults), the lock
    /// hooks on the fake callees, and the installed flag, the output route and
    /// the voices pointer varied by the seed (as in `game/lib/snd.rs`'s tests).
    /// `image` loads the game image first (the output-mode table is read).
    fn oc1148_snd_machine(c: &mut Cpu, seed: u32, entry: u32, image: bool) -> Rng {
        let mut r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        if image {
            elf_image(c);
        }
        c.bus.write32(OC1148_LOCK_HOOK, FAKE);
        c.bus.write32(OC1148_UNLOCK_HOOK, FAKE + 0x10);
        c.bus.write32(OC1148_GOT_CAPS, 1);
        fill(c, &mut r, OC1148_SND_STATE, 0x35);
        c.bus.write8(OC1148_SND_STATE + 0x174, ((seed / 3) % 4 == 0) as u8);
        c.bus.write16(OC1148_SND_STATE + 0x17e, (seed / 12 % 3) as u16);
        c.bus.write32(OC1148_SND_STATE + 0x1f0, DATA + 0x3000);
        r
    }

    #[test]
    fn snd_set_config46_matches() {
        check(
            ee::s13::f_003c5368__generated,
            snd_set_config46,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_5368, false);
                c.g[A0][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_set_config3c_matches() {
        check(
            ee::s13::f_003c53a0__generated,
            snd_set_config3c,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_53a0, false);
                c.g[A0][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn snd_set_config24_matches() {
        check(
            ee::s13::f_003c53d8__generated,
            snd_set_config24,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_53d8, false);
                c.g[A0][0] = sx(r.next_u32());
            },
        );
    }

    /// `SND_SetServiceRate`: negative, zero, NaN-free edge values and ordinary
    /// rates (the divide path and the one that skips it).
    #[test]
    fn snd_set_service_rate_matches() {
        check(
            ee::s13::f_003c5410__generated,
            snd_set_service_rate,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_5410, false);
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1000.0)
                };
            },
        );
    }

    /// `SND_GetServiceRate`: the stored rate negative or not (the divide path),
    /// from the settings block the copy brings in.
    #[test]
    fn snd_get_service_rate_matches() {
        check(
            ee::s13::f_003c5468__generated,
            snd_get_service_rate,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_5468, false);
                c.g[A0][0] = sx(DATA + 0x4000);
                let rate = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1000.0)
                };
                c.bus.write32(OC1148_SND_STATE + 0x34, rate);
            },
        );
    }

    /// `SND_SetOutputMode`: modes the table supports, the rest (the -5 return).
    #[test]
    fn snd_set_output_mode_matches() {
        check(
            ee::s13::f_003c5600__generated,
            snd_set_output_mode,
            200,
            &|c, seed| {
                oc1148_snd_machine(c, seed, 0x003c_5600, true);
                c.g[A0][0] = sx(seed % 11);
            },
        );
    }

    /// `SND_RoundSampleRate`: the three modes, rates on both sides of 24000,
    /// 24001, 36000 and 48000, and negative rates.
    #[test]
    fn snd_round_sample_rate_matches() {
        check(
            ee::s13::f_003c5660__generated,
            snd_round_sample_rate,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_5660, false);
                c.g[A0][0] = sx([0u32, 1, 2, 0, 1][(seed % 5) as usize]);
                let rate = match seed % 7 {
                    0 => 23_990 + r.next_u32() % 20,
                    1 => 35_990 + r.next_u32() % 20,
                    2 => 47_990 + r.next_u32() % 20,
                    6 => r.next_u32(),
                    _ => r.next_u32() % 60_000,
                };
                c.g[A1][0] = sx(rate);
                c.g[A2][0] = sx(DATA + 0x4000);
            },
        );
    }

    /// `SND_SetSampleRate`: modes 0 and 1 (both halfword stores), another mode
    /// (-5), and rates above and below the 0xffff cap.
    #[test]
    fn snd_set_sample_rate_matches() {
        check(
            ee::s13::f_003c5710__generated,
            snd_set_sample_rate,
            200,
            &|c, seed| {
                let mut r = oc1148_snd_machine(c, seed, 0x003c_5710, false);
                c.g[A0][0] = sx([0u32, 1, 0, 1, 2][(seed % 5) as usize]);
                let rate = if seed % 4 == 0 {
                    r.next_u32()
                } else {
                    r.next_u32() % 0x2_0000
                };
                c.g[A1][0] = sx(rate);
            },
        );
    }
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_5368, snd_set_config46, "SND_SetConfig46"),
    (0x003c_53a0, snd_set_config3c, "SND_SetConfig3C"),
    (0x003c_53d8, snd_set_config24, "SND_SetConfig24"),
    (0x003c_5410, snd_set_service_rate, "SND_SetServiceRate"),
    (0x003c_5468, snd_get_service_rate, "SND_GetServiceRate"),
    (0x003c_5600, snd_set_output_mode, "SND_SetOutputMode"),
    (0x003c_5660, snd_round_sample_rate, "SND_RoundSampleRate"),
    (0x003c_5710, snd_set_sample_rate, "SND_SetSampleRate"),
];
