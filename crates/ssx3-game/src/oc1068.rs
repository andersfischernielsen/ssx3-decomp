//! Staging module oc1068 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1068`.

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
    (0x002e37d0, c_player_pulse_fx_dtor, "cPlayerPulseFX_dtor"),
    (
        0x002e44c8,
        c_transition_man_start_2,
        "cTransitionMan_Start_2",
    ),
    (0x002e83f0, c_board_trail_fx_dtor, "cBoardTrailFX_dtor"),
    (
        0x002eac60,
        rider_base_dtor_power_up_trail,
        "RiderBase_DtorPowerUpTrail",
    ),
    (0x002ec450, lens_fx_dtor, "LensFX_dtor"),
    (
        0x002ee390,
        world_painter_get_irradiance3,
        "WorldPainter_GetIrradiance3",
    ),
    (
        0x002ef1f0,
        world_painter_get_irradiance_name3,
        "WorldPainter_GetIrradianceName3",
    ),
    (0x002ef530, c_air_streamer_fx_dtor, "cAirStreamerFX_dtor"),
];

/// Direct callees (the generated alias goes through each callee's override slot).
const LENS_FX_BASE_DTOR: Callee = Callee {
    addr: 0x002e3060,
    f: ee::s09::f_002e3060,
};
const TRANSITION_MAN_START: Callee = Callee {
    addr: 0x002e4370,
    f: ee::s09::f_002e4370,
};
const IRRADIANCE_REF_GET_ENTRY: Callee = Callee {
    addr: 0x002ee010,
    f: ee::s09::f_002ee010,
};
const WORLD_PAINTER_GET_IRRADIANCE_NAME3: Callee = Callee {
    addr: 0x002ef1f0,
    f: ee::s09::f_002ef1f0,
};

/// The object finalisers of the four effect families. They share one body:
/// the same frame, the same two 0x40-byte table copies and the same three
/// tail calls. What differs is the constants (table offsets, the object's
/// count word, the loop heads and the return addresses).
#[derive(Clone, Copy)]
pub(crate) struct Family {
    /// `0x490000 - table_sub`: the first table, copied to the stack.
    pub(crate) table_sub: u32,
    /// `0x490000 - table_b_sub`: the second table, copied into the object.
    pub(crate) table_b_sub: u32,
    /// Subtracted from the object's word at +0 for the stack offset.
    pub(crate) count_sub: u32,
    /// Loop heads of the two copy loops (the event re-entry pcs).
    pub(crate) copy_head: u32,
    pub(crate) fill_head: u32,
    /// Return addresses of the `0x11b978` and `0x317e50` tail calls.
    pub(crate) ret_y: u32,
    pub(crate) ret_z: u32,
    /// The optional `0x317e98` call: (offset of its argument, return address).
    pub(crate) x: Option<(u32, u32)>,
}

/// `cPlayerPulseFX` (0x2e37d0).
pub(crate) const FAMILY_PULSE: Family = Family {
    table_sub: 0x7638,
    table_b_sub: 0x75f0,
    count_sub: 0x40,
    copy_head: 0x002e3834,
    fill_head: 0x002e3898,
    ret_y: 0x002e3904,
    ret_z: 0x002e3918,
    x: None,
};

/// `cBoardTrailFX` (0x2e83f0).
pub(crate) const FAMILY_BOARD_TRAIL: Family = Family {
    table_sub: 0x7448,
    table_b_sub: 0x7400,
    count_sub: 0xf0,
    copy_head: 0x002e8454,
    fill_head: 0x002e84b8,
    ret_y: 0x002e8538,
    ret_z: 0x002e854c,
    x: Some((0x10, 0x002e8520)),
};

/// `RiderBase` power-up trail (0x2eac60).
pub(crate) const FAMILY_POWER_UP_TRAIL: Family = Family {
    table_sub: 0x7ba8,
    table_b_sub: 0x7b60,
    count_sub: 0x110,
    copy_head: 0x002eacc4,
    fill_head: 0x002ead28,
    ret_y: 0x002ead94,
    ret_z: 0x002eada8,
    x: None,
};

/// `cAirStreamerFX` (0x2ef530).
pub(crate) const FAMILY_AIR_STREAMER: Family = Family {
    table_sub: 0x7730,
    table_b_sub: 0x76e8,
    count_sub: 0x20,
    copy_head: 0x002ef594,
    fill_head: 0x002ef5f8,
    ret_y: 0x002ef678,
    ret_z: 0x002ef68c,
    x: Some((0x4, 0x002ef660)),
};

/// The two 0x40-byte copies: the first table (at `0x490000 - table_sub`) to
/// the stack, then the second table into the object's words at `*this`, with
/// the stack offset stored in the object. Runs only when the flag word in s1
/// is zero. Leaves every scratch register as the generated code does.
fn copy_object_tables(c: &mut Cpu, fam: &Family) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A2][0];
    c.g[V1][0] = c.g[SP][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    loop {
        c.cycles += 11;
        c.g[A0][0] = c.ld((c.g[A1][0] as u32));
        c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
        c.sd((c.g[V1][0] as u32), c.g[A0][0]);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, fam.copy_head);
        }
    }
    c.cycles += 14;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(fam.table_sub));
    c.g[A2][0] = c.ld((c.g[A1][0] as u32));
    c.sd((c.g[V1][0] as u32), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(fam.table_b_sub));
    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
    c.g[A1][0] = c.g[T0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(fam.count_sub));
    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    loop {
        c.cycles += 11;
        c.g[V0][0] = c.ld((c.g[A2][0] as u32));
        c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
        c.sd((c.g[A1][0] as u32), c.g[V0][0]);
        c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
        let more = c.g[A2][0] != c.g[T1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, fam.fill_head);
        }
    }
    c.cycles += 10;
    c.g[V0][0] = c.ld((c.g[A2][0] as u32));
    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
    c.sd((c.g[A1][0] as u32), c.g[V0][0]);
    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(fam.table_b_sub));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
}

/// The shared body of the four object finalisers (`cPlayerPulseFX`, ...): sets
/// the object's vtable-like words, copies the tables when s1 is zero, then the
/// tail calls selected by the flag bits in s1, and the epilogue.
pub(crate) fn object_tables_body(c: &mut Cpu, fam: &Family) -> Flow {
    c.cycles += 21;
    let frame = Frame::open(c, 0x130);
    c.g[V0][0] = 0x00490000;
    frame.save(c, 16, 0x120);
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(fam.table_sub));
    frame.save(c, 17, 0x110);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = 0x00460000;
    c.g[A3][0] = c.g[V0][0];
    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
    c.g[V1][0] = 0x00490000;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
    c.g[T2][0] = c.g[V1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(fam.table_b_sub));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
    let flagged = c.g[S1][0] != 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
    if !flagged {
        copy_object_tables(c, fam);
    }

    // Optional call on the word at `this + off` (0x317e98).
    if let Some((off, ret_x)) = fam.x {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(off));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        if !none {
            c.cycles += 3;
            call_ptr(c, 0x00317e98, ret_x)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
        }
    }

    // Call `this->word0` with a1 = 0 (0x11b978) when s1 bit 1 is set.
    c.cycles += if fam.x.is_some() { 2 } else { 3 };
    let no_y = (c.g[S1][0] & 0x2) == 0;
    c.g[V0][0] = c.g[S1][0] & 0x1;
    if !no_y {
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011b978, fam.ret_y)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
    }

    // Call `this` (0x317e50) when s1 bit 0 is set.
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00317e50, fam.ret_z)?;
    }
    frame.restore(c, 16, 0x120);
    c.cycles += 4;
    frame.restore(c, 17, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

/// `cPlayerPulseFX` finaliser (0x2e37d0): the pulse object's tables and tail calls.
pub fn c_player_pulse_fx_dtor(c: &mut Cpu) {
    guest(c, |c| object_tables_body(c, &FAMILY_PULSE))
}

/// `cBoardTrailFX` finaliser (0x2e83f0).
pub fn c_board_trail_fx_dtor(c: &mut Cpu) {
    guest(c, |c| object_tables_body(c, &FAMILY_BOARD_TRAIL))
}

/// `RiderBase_DtorPowerUpTrail` (0x2eac60).
pub fn rider_base_dtor_power_up_trail(c: &mut Cpu) {
    guest(c, |c| object_tables_body(c, &FAMILY_POWER_UP_TRAIL))
}

/// `cAirStreamerFX` finaliser (0x2ef530).
pub fn c_air_streamer_fx_dtor(c: &mut Cpu) {
    guest(c, |c| object_tables_body(c, &FAMILY_AIR_STREAMER))
}

/// `LensFX_dtor` (0x2ec450): the vtable word of the object at +0x649c, then the
/// base destructor.
pub fn lens_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x00480000u32.wrapping_add(0x7f00));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x649c), c.g[V0][0]);
        LENS_FX_BASE_DTOR.call(c, 0x002ec46c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cTransitionMan_Start` with f14 = 0 and a3/t0/t1 = 0 (0x2e44c8).
pub fn c_transition_man_start_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.mtc1(0, 14);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        TRANSITION_MAN_START.call(c, 0x002e44e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldPainter_GetIrradiance3` (0x2ee390): the irradiance entry named by
/// `WorldPainter_GetIrradianceName3`, looked up in its reference table.
pub fn world_painter_get_irradiance3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        WORLD_PAINTER_GET_IRRADIANCE_NAME3.call(c, 0x002ee3a0)?;
        c.g[A0][0] = c.g[V0][0];
        IRRADIANCE_REF_GET_ENTRY.call(c, 0x002ee3a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldPainter_GetIrradianceName3` (0x2ef1f0): the irradiance name of the
/// current frame's painter, or `gp + 0x24c0` when the painter is not set up.
pub fn world_painter_get_irradiance_name3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xa74));
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if ready {
            c.cycles += 2;
            c.g[V0][0] = sx(gp.wrapping_add(0x24c0));
        } else {
            c.cycles += 14;
            c.g[V1][0] = 0xf0;
            c.g[V0][0] = 0x00500000;
            c.mult(0, 3, 4, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c90));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x140));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x144));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002ef230)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The fakes answer only while this word holds `GATE_MAGIC` (unique to
    /// this module, see the writer brief).
    const GATE: u32 = DATA + 0x9_0000;
    const GATE_MAGIC: u32 = 0x1068_c0de;

    /// A gated fake callee: logs the call and returns to `ra`, or runs the
    /// body that was in the slot before the hook when the gate is off.
    macro_rules! gated_callee {
        ($fake:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(GATE) != GATE_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    gated_callee!(
        oc1068_fake_11b978,
        OC1068_PREV_11B978,
        0x0011b978,
        ee::s00::f_0011b978__generated
    );
    gated_callee!(
        oc1068_fake_317e50,
        OC1068_PREV_317E50,
        0x00317e50,
        ee::s10::f_00317e50__generated
    );
    gated_callee!(
        oc1068_fake_317e98,
        OC1068_PREV_317E98,
        0x00317e98,
        ee::s10::f_00317e98__generated
    );
    gated_callee!(
        oc1068_fake_2e3060,
        OC1068_PREV_2E3060,
        0x002e3060,
        ee::s09::f_002e3060__generated
    );
    gated_callee!(
        oc1068_fake_2e4370,
        OC1068_PREV_2E4370,
        0x002e4370,
        ee::s09::f_002e4370__generated
    );
    gated_callee!(
        oc1068_fake_2ee010,
        OC1068_PREV_2EE010,
        0x002ee010,
        ee::s09::f_002ee010__generated
    );
    gated_callee!(
        oc1068_fake_2ef1f0,
        OC1068_PREV_2EF1F0,
        0x002ef1f0,
        ee::s09::f_002ef1f0__generated
    );

    /// Hooks the gated fakes into the callees' override slots (once per process).
    fn oc1068_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc1068_fake_11b978,
            &OC1068_PREV_11B978,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1068_fake_317e50,
            &OC1068_PREV_317E50,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1068_fake_317e98,
            &OC1068_PREV_317E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E3060),
            oc1068_fake_2e3060,
            &OC1068_PREV_2E3060,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4370),
            oc1068_fake_2e4370,
            &OC1068_PREV_2E4370,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EE010),
            oc1068_fake_2ee010,
            &OC1068_PREV_2EE010,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EF1F0),
            oc1068_fake_2ef1f0,
            &OC1068_PREV_2EF1F0,
        );
    }

    /// An object finalisation fixture: `this` at DATA+0x2000 holds the body
    /// pointer at +0 (the words the function writes go there), the optional
    /// argument word at +4 and +0x10, the flag word in s1 (all four bit
    /// combinations), and the two tables hold random words.
    fn object_fixture(c: &mut Cpu, seed: u32, fam: &Family) {
        machine(c, seed, 1068, 3);
        c.bus.write32(GATE, GATE_MAGIC);
        let this = DATA + 0x2000;
        let body = DATA + 0x2800;
        let arg = if seed % 3 == 0 {
            0
        } else {
            DATA + 0x3000 + (seed % 5) * 0x10
        };
        c.bus.write32(this, body);
        c.bus.write32(this + 4, arg);
        c.bus.write32(this + 0x10, arg);
        c.bus.write32(body, arg);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (seed % 4) as u64;
        let mut r = crate::difftest::Rng(0x1068_0000 ^ seed as u64);
        fill(c, &mut r, 0x00490000 - fam.table_sub, 0x20);
        fill(c, &mut r, 0x00490000 - fam.table_b_sub, 0x20);
    }

    #[test]
    fn c_player_pulse_fx_dtor_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002e37d0__generated,
            c_player_pulse_fx_dtor,
            300,
            &|c, seed| object_fixture(c, seed, &FAMILY_PULSE),
        );
    }

    #[test]
    fn c_board_trail_fx_dtor_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002e83f0__generated,
            c_board_trail_fx_dtor,
            300,
            &|c, seed| object_fixture(c, seed, &FAMILY_BOARD_TRAIL),
        );
    }

    #[test]
    fn rider_base_dtor_power_up_trail_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002eac60__generated,
            rider_base_dtor_power_up_trail,
            300,
            &|c, seed| object_fixture(c, seed, &FAMILY_POWER_UP_TRAIL),
        );
    }

    #[test]
    fn c_air_streamer_fx_dtor_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002ef530__generated,
            c_air_streamer_fx_dtor,
            300,
            &|c, seed| object_fixture(c, seed, &FAMILY_AIR_STREAMER),
        );
    }

    #[test]
    fn lens_fx_dtor_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002ec450__generated,
            lens_fx_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 1068, 2);
                c.bus.write32(GATE, GATE_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_transition_man_start_2_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002e44c8__generated,
            c_transition_man_start_2,
            200,
            &|c, seed| {
                machine(c, seed, 1068, 2);
                c.bus.write32(GATE, GATE_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn world_painter_get_irradiance3_matches() {
        oc1068_install_fakes();
        check(
            ee::s09::f_002ee390__generated,
            world_painter_get_irradiance3,
            200,
            &|c, seed| {
                machine(c, seed, 1068, 2);
                c.bus.write32(GATE, GATE_MAGIC);
            },
        );
    }

    /// The painter's table path: `gp + 0xa74` is the "ready" word; when it is
    /// zero the index `a0 * 0xf0 + 0x4fa370` leads through three pointers to
    /// the callee (a fake at `FAKE`, reached through the miss hook).
    #[test]
    fn world_painter_get_irradiance_name3_matches() {
        check(
            ee::s09::f_002ef1f0__generated,
            world_painter_get_irradiance_name3,
            300,
            &|c, seed| {
                machine(c, seed, 1068, 1);
                let k = seed % 4;
                c.g[A0][0] = k as u64;
                c.bus
                    .write32(GAME_GP + 0xa74, if seed % 2 == 0 { 0 } else { 1 });
                let base = 0x004fa370 + 0xf0 * k;
                let entry = DATA + 0x2000;
                c.bus.write32(base + 0x1c, entry);
                c.bus.write32(entry, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 4, DATA + 0x2200);
                c.bus.write16(DATA + 0x2200 + 0x140, (seed * 7) as u16);
                c.bus.write32(DATA + 0x2200 + 0x144, fakes::FAKE);
                c.g[A0][0] = sx(k);
            },
        );
    }
}
