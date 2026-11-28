//! Staging module oc1552 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1552`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_f700, light_glow_save_state, "tWPIGD_LightGlow_SaveState"),
    (0x002b_f8b8, screen_tint_save_state, "tWPIGD_ScreenTint_SaveState"),
    (0x002b_fd30, lighting_save_state, "tWPIGD_Lighting_SaveState"),
    (0x002c_0308, danger_save_state, "tWPIGD_Danger_SaveState"),
    (0x002c_2d30, istr_alloc, "ISTR_Alloc"),
    (0x002c_2d68, istr_free, "ISTR_Free"),
    (0x002c_2d88, istr_mem_copy, "ISTR_MemCopy"),
    (0x002c_2db0, istr_realloc, "ISTR_Realloc"),
];

/// The string the ISTR allocations are tagged with (`str_ISTR_data`).
const ISTR_DATA: u32 = 0x0048_6098;

/// One `frame->write(self + off, size)` of a replay save: the call through the
/// frame's write slot (the `this` adjustment at +8 of its vtable, the function
/// at +0xc), returning to `ret_to`. Offset 0 passes the object pointer itself.
fn replay_write(c: &mut Cpu, off: u32, size: u64, ret_to: u32) -> Flow {
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    let vtbl = c.g[V1][0] as u32;
    c.g[A1][0] = if off == 0 {
        c.g[S1][0]
    } else {
        sx((c.g[S1][0] as u32).wrapping_add(off))
    };
    c.g[A2][0] = size;
    c.g[A0][0] = Addr(vtbl.wrapping_add(0x8)).lh(c);
    c.g[V0][0] = Addr(vtbl.wrapping_add(0xc)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// The save-state body shared by the four `tWPInGameData_*::writeToReplayFrame`
/// functions: `entry_cycles` charged at entry, then one [`replay_write`] per
/// row `(field offset, size, return address)`. The rows alternate a field with
/// the object's first word (offset 0), which the C++ writes after each field.
fn write_replay_frame(c: &mut Cpu, entry_cycles: u64, writes: &[(u32, u64, u32)]) -> Flow {
    c.cycles += entry_cycles;
    let frame = Frame::open(c, 0x30);
    c.g[A2][0] = 0x4;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    for &(off, size, ret_to) in writes {
        replay_write(c, off, size, ret_to)?;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `tWPInGameData_LightGlow::writeToReplayFrame` (0x2bf700): the seven light-glow
/// words (cutoff, cutoff scale, copy/source/frame intensity, blend 2 and 3).
pub fn light_glow_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 109, LIGHT_GLOW_WRITES))
}

const LIGHT_GLOW_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x4, 0x002b_f734),   // m_LightGlowCutoff
    (0, 0x4, 0x002b_f750),
    (0x10, 0x4, 0x002b_f76c),  // m_LightGlowCutoffScale
    (0, 0x4, 0x002b_f788),
    (0x18, 0x4, 0x002b_f7a4),  // m_LightGlowCopyIntensity
    (0, 0x4, 0x002b_f7c0),
    (0x20, 0x4, 0x002b_f7dc),  // m_LightGlowSourceIntensity
    (0, 0x4, 0x002b_f7f8),
    (0x28, 0x4, 0x002b_f814),  // m_LightGlowFrameIntensity
    (0, 0x4, 0x002b_f830),
    (0x30, 0x4, 0x002b_f84c),  // m_LightGlowBlend2
    (0, 0x4, 0x002b_f868),
    (0x38, 0x4, 0x002b_f884),  // m_LightGlowBlend3
    (0, 0x4, 0x002b_f8a0),
];

/// `tWPInGameData_ScreenTint::writeToReplayFrame` (0x2bf8b8): the scale and offset
/// of the red, green and blue screen tint.
pub fn screen_tint_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 95, SCREEN_TINT_WRITES))
}

const SCREEN_TINT_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x4, 0x002b_f8ec),   // m_ScreenTintScaleR
    (0, 0x4, 0x002b_f908),
    (0x10, 0x4, 0x002b_f924),  // m_ScreenTintScaleG
    (0, 0x4, 0x002b_f940),
    (0x18, 0x4, 0x002b_f95c),  // m_ScreenTintScaleB
    (0, 0x4, 0x002b_f978),
    (0x20, 0x4, 0x002b_f994),  // m_ScreenTintOffsetR
    (0, 0x4, 0x002b_f9b0),
    (0x28, 0x4, 0x002b_f9cc),  // m_ScreenTintOffsetG
    (0, 0x4, 0x002b_f9e8),
    (0x30, 0x4, 0x002b_fa04),  // m_ScreenTintOffsetB
    (0, 0x4, 0x002b_fa20),
];

/// `tWPInGameData_Lighting::writeToReplayFrame` (0x2bfd30): the four irradiance
/// texture names (8 bytes each) and the ambient and rim light offsets.
pub fn lighting_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 95, LIGHTING_WRITES))
}

const LIGHTING_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x8, 0x002b_fd64),   // m_LightIrradianceName
    (0, 0x4, 0x002b_fd80),
    (0x18, 0x8, 0x002b_fd9c),  // m_ShadowIrradianceName
    (0, 0x4, 0x002b_fdb8),
    (0x28, 0x8, 0x002b_fdd4),  // m_TunnelIrradianceName
    (0, 0x4, 0x002b_fdf0),
    (0x38, 0x8, 0x002b_fe0c),  // m_ObjectIrradianceName
    (0, 0x4, 0x002b_fe28),
    (0x48, 0x4, 0x002b_fe44),  // m_AmbientLightIntensityOffset
    (0, 0x4, 0x002b_fe60),
    (0x50, 0x4, 0x002b_fe7c),  // m_RimLightIntensityOffset
    (0, 0x4, 0x002b_fe98),
];

/// `tWPInGameData_Danger::writeToReplayFrame` (0x2c0308): the level, the travel
/// distance (offset 0) and the point rate.
pub fn danger_save_state(c: &mut Cpu) {
    guest(c, |c| write_replay_frame(c, 39, DANGER_WRITES))
}

const DANGER_WRITES: &[(u32, u64, u32)] = &[
    (0x8, 0x4, 0x002c_033c),   // m_Level
    (0, 0x4, 0x002c_0358),     // m_wpTravelDistance (the C++ writes it twice)
    (0x10, 0x4, 0x002c_0374),  // m_PointRate
    (0, 0x4, 0x002c_0390),
];

/// `ISTR_Alloc(name, size, zeroFlag)` (0x2c2d30): `new (str_ISTR_data,
/// zeroFlag ? 0x100 : 0, 0) char[size]`. The name is not used. Returns the
/// allocator's pointer.
pub fn istr_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = 0x100;
        let frame = Frame::open(c, 0x10);
        if c.g[A2][0] == 0 {
            c.g[V0][0] = 0u64;
        }
        c.g[A1][0] = sx(ISTR_DATA);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x0031_7e30, 0x002c_2d58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ISTR_Free(block)` (0x2c2d68): frees through `MEM_Free` and returns whether
/// that gave a non-zero result (`0 < v0`).
pub fn istr_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_23a8, 0x002c_2d78)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `ISTR_MemCopy(dst, src, size)` (0x2c2d88): `memcpy`, returns `dst`.
pub fn istr_mem_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_6574, 0x002c_2d9c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ISTR_Realloc(block, name, size, zeroFlag)` (0x2c2db0): no block allocates
/// (`ISTR_Alloc`); size 0 frees the block (`ISTR_Free`); otherwise the block is
/// resized under the `cMemMan` mutex.
pub fn istr_realloc(c: &mut Cpu) {
    guest(c, realloc_body)
}

fn realloc_body(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    c.g[A2][0] = c.g[A3][0];
    if c.g[S1][0] == 0 {
        // No block yet: allocate `size` bytes.
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x002c_2d30,
            f: ee::s08::f_002c2d30,
        }
        .call(c, 0x002c_2de0)?;
        frame.restore(c, S0, 0x30);
    } else {
        c.cycles += 2;
        let size_is_zero = c.g[S2][0] == 0;
        c.g[S0][0] = 0x540000;
        if size_is_zero {
            // Size 0: free the block.
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x002c_2d68,
                f: ee::s08::f_002c2d68,
            }
            .call(c, 0x002c_2e24)?;
            c.g[V0][0] = 0u64;
        } else {
            // Resize under the mutex (cMemMan_m_mutex at 0x538b00).
            c.cycles += 11;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7500));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_5700, 0x002c_2dfc)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x0031_9bd8, 0x002c_2e08)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_5760, 0x002c_2e14)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, machine_rt, vtable};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// The replay frame: its first word is the vtable of the write slot.
    const FRAME: u32 = DATA + 0x20_0000;
    /// The object being saved, the ISTR name and block.
    const OBJ: u32 = DATA + 0x20_1000;
    /// The memcpy source.
    const SRC: u32 = DATA + 0x20_2000;
    /// Gate word: the gated fakes answer only while it holds `MAGIC`.
    const GATE: u32 = DATA + 0x20_8000;
    /// Mailbox: word 0 is the MEM_Free result, word 1 the resize result.
    const MAILBOX: u32 = GATE + 4;
    const MAGIC: u32 = 0x1552_0a5e;
    const SALT: u64 = 0x1552_0000;

    static PREV_MEM_FREE: AtomicUsize = AtomicUsize::new(0);
    static PREV_RESIZE: AtomicUsize = AtomicUsize::new(0);
    static PREV_LOCK: AtomicUsize = AtomicUsize::new(0);
    static PREV_UNLOCK: AtomicUsize = AtomicUsize::new(0);

    fn gated(c: &mut Cpu) -> bool {
        c.bus.read32(GATE) == MAGIC
    }

    /// MEM_Free while gated: returns the mailbox word (zero included).
    fn oc1552_fake_mem_free(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_MEM_FREE, ee::s06::f_002523a8__generated);
        }
        c.g[V0][0] = c.bus.read32(MAILBOX) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// The cMemMan resize while gated: returns the second mailbox word.
    fn oc1552_fake_resize(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_RESIZE, ee::s10::f_00319bd8__generated);
        }
        c.g[V0][0] = c.bus.read32(MAILBOX + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// The mutex lock and unlock while gated: they return at once.
    fn oc1552_fake_lock(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_LOCK, ee::s14::f_003e5700__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn oc1552_fake_unlock(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_UNLOCK, ee::s14::f_003e5760__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Puts the gated fakes in their override slots (once per process).
    fn oc1552_hook_all() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1552_fake_mem_free,
            &PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00319BD8),
            oc1552_fake_resize,
            &PREV_RESIZE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            oc1552_fake_lock,
            &PREV_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            oc1552_fake_unlock,
            &PREV_UNLOCK,
        );
    }

    /// Opens the gate and sets the mailbox; some seeds get a zero result.
    fn oc1552_gate_on(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.bus.write32(GATE, MAGIC);
        let free = if seed % 3 == 0 { 0 } else { r.next_u32() };
        let resize = if seed % 5 == 0 { 0 } else { r.next_u32() };
        c.bus.write32(MAILBOX, free);
        c.bus.write32(MAILBOX + 4, resize);
    }

    /// A save-state fixture: `a0` is the object, `a1` the replay frame whose
    /// first word is a vtable with one write slot (a logged fake call).
    fn replay_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = machine(c, seed, SALT, calls);
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(FRAME, vt);
        c.g[A0][0] = sx(OBJ);
        c.g[A1][0] = sx(FRAME);
    }

    #[test]
    fn light_glow_save_state_matches() {
        covered(
            ee::s08::f_002bf700__generated,
            light_glow_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 14),
        );
    }

    #[test]
    fn screen_tint_save_state_matches() {
        covered(
            ee::s08::f_002bf8b8__generated,
            screen_tint_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 12),
        );
    }

    #[test]
    fn lighting_save_state_matches() {
        covered(
            ee::s08::f_002bfd30__generated,
            lighting_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 12),
        );
    }

    #[test]
    fn danger_save_state_matches() {
        covered(
            ee::s08::f_002c0308__generated,
            danger_save_state,
            200,
            &|c, seed| replay_fixture(c, seed, 4),
        );
    }

    #[test]
    fn istr_alloc_matches() {
        // The allocator (0x317e30) is real: the runtime fixture gives it a heap.
        check(ee::s08::f_002c2d30__generated, istr_alloc, 200, &|c, seed| {
            let mut r = machine_rt(c, seed, SALT, 0);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(1 + (r.next_u32() & 0xff));
            c.g[A2][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn istr_free_matches() {
        oc1552_hook_all();
        check(ee::s08::f_002c2d68__generated, istr_free, 200, &|c, seed| {
            let mut r = machine(c, seed, SALT, 0);
            oc1552_gate_on(c, seed, &mut r);
            c.g[A0][0] = sx(OBJ);
        });
    }

    #[test]
    fn istr_mem_copy_matches() {
        check(
            ee::s08::f_002c2d88__generated,
            istr_mem_copy,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, SALT, 0);
                fill(c, &mut r, SRC, 0x20);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(SRC);
                c.g[A2][0] = (r.next_u32() % 0x40) as u64;
            },
        );
    }

    #[test]
    fn istr_realloc_matches() {
        oc1552_hook_all();
        // Seeds cover the three paths: no block (allocate), size 0 (free) and
        // a resize under the mutex.
        check(
            ee::s08::f_002c2db0__generated,
            istr_realloc,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, SALT, 0);
                oc1552_gate_on(c, seed, &mut r);
                c.g[A0][0] = sx(if seed % 3 == 0 { 0 } else { OBJ + 0x40 });
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = if seed % 4 == 0 {
                    0
                } else {
                    1 + (r.next_u32() & 0xff) as u64
                };
                c.g[A3][0] = (r.next_u32() & 1) as u64;
            },
        );
    }
}
