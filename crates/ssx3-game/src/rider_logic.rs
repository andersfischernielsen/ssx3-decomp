//! Work package E4: rider logic & metrics (`docs/investigation/phase2-candidates-3.md`,
//! E4; field names from `docs/investigation/structs/rider.md`).
//!
//! The per-frame rider logic around the state machine: the `RiderBase`
//! state dispatchers (Update / PostUpdate of the main state, HandleInput of
//! the sub state), the frame prologue (`Rider_BeginFrame`), target speed and
//! boost, slew targets, race and AI-path progress, the finish check, the
//! light set and model render, the animation sequence updater, and the
//! `RiderMetrix` helpers (HUD messages, trick reset).
//!
//! Every replacement follows the call protocol of [`crate::call`] (the same
//! stack frame, the registers at each call, the unwind check, cycles block
//! by block) and reproduces the incidental values the original leaves in
//! registers (marked "leftover"), so that a later spill or a caller sees the
//! same state. Loops make the same event checks at their back edges with
//! every register as the original has it there ([`crate::events`]).

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::game::ai::rider::{rate_step_60, smoothing_60};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_6ac8,
        rider_base_record_cheat_code_input,
        "RiderBase_RecordCheatCodeInput",
    ),
    (
        0x0011_7400,
        rider_metrix_is_hud_message_active,
        "RiderMetrix_IsHUDMessageActive",
    ),
    (
        0x0011_7838,
        rider_metrix_reset_trick,
        "RiderMetrix_ResetTrick",
    ),
];

// ---------------------------------------------------------------------------
// Field offsets.

/// `Rider` fields.
mod rider {
    /// `+0x120 rot` (unit quaternion).
    pub const ROT: u32 = 0x120;
    /// `+0x1c0` the world matrix's up row (z).
    pub const WORLD_UP: u32 = 0x1c0;
    /// `+0x1e0 vel` (units/s).
    pub const VEL: u32 = 0x1e0;
    /// `+0x3a0 groundForward`.
    pub const GROUND_FORWARD: u32 = 0x3a0;
    /// `+0x2e8 powerUpBoostTimer` (s of free boost).
    pub const POWER_UP_BOOST_TIMER: u32 = 0x2e8;
    /// `+0x2f0 uberTimer` (s).
    pub const UBER_TIMER: u32 = 0x2f0;
    /// `+0x86c index` (slot 0..5), `+0xb34 statsIndex`.
    pub const INDEX: u32 = 0x86c;
    pub const STATS_INDEX: u32 = 0xb34;
    /// `+0xb28 boostSourceMask`, `+0xb2c uberFlags` (bit 0: uber allowed).
    pub const BOOST_SOURCE_MASK: u32 = 0xb28;
    pub const UBER_FLAGS: u32 = 0xb2c;
    /// `+0x2f4 uberLevel`, `+0x2f8 boostMeter` (0..1), `+0x2fc
    /// boostStrength` (0 = not boosting), `+0x304 boostMode`.
    pub const UBER_LEVEL: u32 = 0x2f4;
    pub const BOOST_METER: u32 = 0x2f8;
    pub const BOOST_STRENGTH: u32 = 0x2fc;
    pub const BOOST_MODE: u32 = 0x304;
    /// `+0x77c owner` (`RiderBase*`).
    pub const OWNER: u32 = 0x77c;
    /// `+0x438 surfaceType` (SurfPropCache index).
    pub const SURFACE_TYPE: u32 = 0x438;
}

/// `RiderBase` fields.
mod base {
    /// `+0xde0 mainState`: 0 Ground, 1 Air, 2 Wipeout, 3 Static,
    /// 4 Railslide, 5 Handplant.
    pub const MAIN_STATE: u32 = 0xde0;
    /// `+0xde4 subState`: 0 Cruise .. 0xc UberGrind, 0xd Void.
    pub const SUB_STATE: u32 = 0xde4;
    /// `+0xd20` the cheat-code detector (`cCheatCodeDetector`).
    pub const CHEAT_DETECTOR: u32 = 0xd20;
}

/// `RiderMetrix` fields.
mod metrix {
    /// `+0x14 trickBoost`, `+0x18 multiplier`.
    pub const TRICK_BOOST: u32 = 0x14;
    pub const MULTIPLIER: u32 = 0x18;
    /// `+0x60 trickIds[3]`.
    pub const TRICK_IDS: u32 = 0x60;
    /// `+0x1b0` the active HUD messages (0x9c-byte records, first word =
    /// message id), `+0x1b4` their count, `+0x1b8` a bit set of the active
    /// ids (rebuilt when `+0x1c0` is set).
    pub const HUD_MESSAGES: u32 = 0x1b0;
    pub const HUD_MESSAGE_COUNT: u32 = 0x1b4;
    pub const HUD_ACTIVE_BITS: u32 = 0x1b8;
    pub const HUD_BITS_DIRTY: u32 = 0x1c0;
    /// Size of one HUD message record.
    pub const HUD_MESSAGE_SIZE: u32 = 0x9c;
    /// The id of a message that never counts as active.
    pub const HUD_MESSAGE_IGNORED: u64 = 0x34;
}

/// `lw base+off` (sign-extended, through the CPU load path).
#[inline(always)]
fn lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `sw v, base+off`.
#[inline(always)]
fn sw(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr((base as u32).wrapping_add(off)).sw(c, v)
}

/// `addiu rd, base, off` on a pointer (sign-extended 32-bit result).
#[inline(always)]
fn ptr_add(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

// ---------------------------------------------------------------------------
// Callees.

/// Cross-shard (or table) calls the generated code makes with `call_indirect`.
mod table {
    pub const EA_MEM_SET: u32 = 0x003e_6448;
    pub const STRCMP: u32 = 0x0041_65a8;
    pub const ANIM_SEQUENCE_SET_TRACK_TIME: u32 = 0x0031_3cf0;
    pub const ANIM_SEQUENCE_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
    pub const ANIM_SEQUENCE_ADVANCE_TRACK: u32 = 0x0031_35b0;
    pub const ANIM_SEQ_LIST_REMOVE: u32 = 0x0031_45f8;
    pub const CHEAT_CODE_DETECTOR_POST_EVENT: u32 = 0x002f_6ac8;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    /// `FUN_0029ab08(audio, rider, meter, amount)`: the meter-fill sound.
    pub const SSX_AUDIO_SYSTEM_ON_BOOST_ADDED: u32 = 0x0029_ab08;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_CHAR_INT_GET_ATTRIB2_FACTOR: u32 = 0x0014_9690;
    pub const SSX_AUDIO_SYSTEM_START_BOOST_SOUND: u32 = 0x0029_8d90;
    pub const SSX_AUDIO_SYSTEM_STOP_BOOST_SOUND: u32 = 0x0029_92d8;
    /// `FUN_00299368`: the sound of a boost press with an empty meter.
    pub const SSX_AUDIO_SYSTEM_NO_BOOST_SOUND: u32 = 0x0029_9368;
}

/// `RiderAnim` (the rider's animation controller) fields.
mod anim {
    /// `+0x60` the rider.
    pub const RIDER: u32 = 0x60;
}

/// `AnimSequence` fields.
mod seq_field {
    /// `+0x10` playback scale.
    pub const TIME_SCALE: u32 = 0x10;
}

/// Direct calls (same shard).
mod callee {
    use super::{ee, Callee};

    pub const RIDER_BUILD_WORLD_MATRIX: Callee = Callee {
        addr: 0x0011_e098,
        f: ee::s00::Rider_BuildWorldMatrix,
    };
    /// `FUN_0010e028` (proposed `Rider_SetHudMessage(rider, id)`).
    pub const RIDER_SET_HUD_MESSAGE: Callee = Callee {
        addr: 0x0010_e028,
        f: ee::s00::f_0010e028,
    };
    pub const RIDER_TOGGLE_STANCE: Callee = Callee {
        addr: 0x0011_5168,
        f: ee::s00::Rider_ToggleStance,
    };
}

// ---------------------------------------------------------------------------
// State dispatchers.

/// A state handler: called directly (same shard) or through the table.
#[derive(Clone, Copy)]
enum Handler {
    Direct(Callee),
    Table(u32),
}

/// One case of a state switch: the jump-table label, the state object's
/// offset in the `RiderBase` (the handler's `this`) and the handler. `None`
/// for a label that just returns.
type Case = (u32, Option<(u32, Handler)>);

/// The shape gcc emits for `switch (this->state) { case i: obj_i.Handle(); }`
/// in a 0x10 frame: `this` moved to `this_reg` (other argument registers
/// pass through), the state loaded and range-checked (`+7`), the jump-table
/// entry loaded (`+6`), the handler called (`+4`, the last one `+2`), and
/// the epilogue (`+2`, after `+1` when falling out of the last case or the
/// switch). An entry that isn't a case label leaves the function
/// (`jump_out`), as the generated code does.
fn state_switch(
    c: &mut Cpu,
    this_reg: usize,
    field: u32,
    jump_table: u32,
    cases: &[Case],
    range: (u32, u32),
) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[this_reg][0] = this;
        frame.save_ra(c, 0);
        let state = lw(c, this, field);
        c.g[V1][0] = state;
        c.g[V0][0] = sx(jump_table & 0xffff_0000);
        let mut tail = 1;
        if state < cases.len() as u64 {
            c.cycles += 6;
            c.g[V0][0] = sx(jump_table);
            let slot = sx((state as u32) << 2).wrapping_add(jump_table as u64);
            c.g[V1][0] = sx(slot as u32);
            let target = Addr(slot as u32).lw(c);
            c.g[A0][0] = target;
            let Some(&(_, case)) = cases.iter().find(|(label, _)| *label == target as u32) else {
                c.jump_out(target as u32, range.0, range.1);
                return Ok(());
            };
            if let Some((obj, handler)) = case {
                let last = cases.iter().rposition(|(_, h)| h.is_some())
                    == cases.iter().position(|(l, _)| *l == target as u32);
                c.cycles += if last { 2 } else { 4 };
                // The call returns to the instruction after its delay slot.
                let ret = target as u32 + 8;
                c.g[A0][0] = ptr_add(this, obj);
                match handler {
                    Handler::Direct(f) => f.call(c, ret)?,
                    Handler::Table(addr) => call_ptr(c, addr, ret)?,
                }
                if !last {
                    tail = 0;
                }
            }
        }
        c.cycles += tail;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `RiderBase_UpdateMainState(base)` (0x111408): the main state's per-frame
/// Update (`Rider<State>State_Update`); physics lives there.
pub fn rider_base_update_main_state(c: &mut Cpu) {
    use Handler::Table;
    const CASES: &[Case] = &[
        (0x0011_143c, Some((0x000, Table(0x0013_d818)))), // Ground
        (0x0011_144c, Some((0x020, Table(0x0013_9a20)))), // Air
        (0x0011_145c, Some((0x030, Table(0x0013_6e98)))), // Wipeout
        (0x0011_146c, Some((0x0a0, Table(0x0013_6958)))), // Static
        (0x0011_147c, Some((0x0b0, Table(0x0013_af28)))), // Railslide
        (0x0011_148c, Some((0x110, Table(0x0013_91a8)))), // Handplant
    ];
    state_switch(
        c,
        A1,
        base::MAIN_STATE,
        0x0045_6b50,
        CASES,
        (0x0011_1408, 0x0011_14a0),
    )
}

/// `RiderBase_PostUpdateMainState(base)` (0x1114a0): the main state's
/// PostUpdate (`Rider<State>State_PostUpdate`).
pub fn rider_base_post_update_main_state(c: &mut Cpu) {
    use Handler::Table;
    const CASES: &[Case] = &[
        (0x0011_14d4, Some((0x000, Table(0x0013_f178)))), // Ground
        (0x0011_14e4, Some((0x020, Table(0x0013_9c88)))), // Air
        (0x0011_14f4, Some((0x030, Table(0x0013_6ee0)))), // Wipeout
        (0x0011_1504, Some((0x0a0, Table(0x0013_6978)))), // Static
        (0x0011_1514, Some((0x0b0, Table(0x0013_bfa8)))), // Railslide
        (0x0011_1524, Some((0x110, Table(0x0013_9528)))), // Handplant
    ];
    state_switch(
        c,
        A1,
        base::MAIN_STATE,
        0x0045_6b70,
        CASES,
        (0x0011_14a0, 0x0011_1538),
    )
}

/// `RiderBase_HandleSubStateInput(base, input)` (0x111728): the sub state's
/// HandleInput with the control-input record (a1 passes through).
pub fn rider_base_handle_sub_state_input(c: &mut Cpu) {
    use ee::s00 as s;
    use Handler::{Direct, Table};
    const fn d(addr: u32, f: GuestFn) -> Handler {
        Direct(Callee { addr, f })
    }
    const CASES: &[Case] = &[
        (
            0x0011_175c,
            Some((0x1c0, d(0x0013_1620, s::RiderCruiseSub_HandleInput))),
        ),
        (
            0x0011_176c,
            Some((0x1d0, d(0x0012_fc80, s::RiderBoardPressSub_HandleInput))),
        ),
        (
            0x0011_177c,
            Some((0x1f0, d(0x0012_e9b8, s::RiderPrewindSub_HandleInput))),
        ),
        (
            0x0011_178c,
            Some((0x200, d(0x0012_e778, s::RiderBumpSub_HandleInput))),
        ),
        (
            0x0011_179c,
            Some((0x210, d(0x0012_f730, s::RiderNaturalAirSub_HandleInput))),
        ),
        (0x0011_17ac, Some((0x230, Table(0x0013_3308)))), // Spin
        (
            0x0011_17bc,
            Some((0x290, d(0x0012_bf68, s::RiderGateSub_HandleInput))),
        ),
        (
            0x0011_17cc,
            Some((0x2b0, d(0x0013_1d30, s::RiderRailslideSub_HandleInput))),
        ),
        (
            0x0011_17dc,
            Some((0x2c0, d(0x0012_cb68, s::RiderWipeoutSub_HandleInput))),
        ),
        (
            0x0011_17ec,
            Some((0x350, d(0x0012_f398, s::RiderResetSub_HandleInput))),
        ),
        (
            0x0011_17fc,
            Some((0x360, d(0x0012_c678, s::RiderFinishLineSub_HandleInput))),
        ),
        (0x0011_180c, Some((0x370, Table(0x0013_2a30)))), // Handplant
        (0x0011_181c, Some((0x390, Table(0x0013_6508)))), // UberGrind
        (0x0011_1824, None),                              // Void
    ];
    state_switch(
        c,
        A2,
        base::SUB_STATE,
        0x0045_6c10,
        CASES,
        (0x0011_1728, 0x0011_1830),
    )
}

// ---------------------------------------------------------------------------
// Small leaves.

/// `cCheatCodeDetector_PostEvent(detector, event)` (0x2f6ac8, called
/// `RiderBase_RecordCheatCodeInput`): record an input event in the
/// detector's event byte (+7): 0 = boost press (bit 0x10), 1 = jump press
/// (0x20), 2 = 0x40; other events are ignored.
pub fn rider_base_record_cheat_code_input(c: &mut Cpu) {
    let detector = c.g[A0][0];
    let event = c.g[A1][0];
    let events = Addr((detector as u32).wrapping_add(7));
    c.cycles += 3;
    let bit = if event == 1 {
        c.cycles += 4;
        Some(0x20)
    } else {
        c.cycles += 2;
        // Leftover: `slti v0, a1, 2`, then the compare constant.
        c.g[V0][0] = ((event as i64) < 2) as u64;
        c.cycles += 2;
        if (event as i64) < 2 {
            if event == 0 {
                c.cycles += 3;
                Some(0x10)
            } else {
                c.cycles += 2;
                None
            }
        } else {
            c.g[V0][0] = 2;
            if event == 2 {
                c.cycles += 3;
                Some(0x40)
            } else {
                c.cycles += 2;
                None
            }
        }
    };
    if let Some(bit) = bit {
        let v = events.lbu(c) | bit;
        c.g[V0][0] = v;
        events.sb(c, v);
    }
    crate::abi::ret(c);
}

/// `RiderMetrix_IsHUDMessageActive(metrix, id)` (0x117400): whether HUD
/// message `id` is showing. Without a message list: false. When the list
/// changed (+0x1c0), first OR the ids of all listed messages (except 0x34)
/// into the active-bit set and clear the flag; then test bit `id`.
pub fn rider_metrix_is_hud_message_active(c: &mut Cpu) {
    let m = c.g[A0][0];
    c.g[A2][0] = m;
    c.cycles += 4;
    let list = lw(c, m, metrix::HUD_MESSAGES);
    c.g[V0][0] = list;
    if list == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
        return;
    }
    let dirty = lw(c, m, metrix::HUD_BITS_DIRTY);
    c.g[V0][0] = dirty;
    c.cycles += 2;
    let bits = ptr_add(m, metrix::HUD_ACTIVE_BITS);
    c.g[T2][0] = bits;
    if dirty != 0 {
        c.cycles += 3;
        let count = lw(c, m, metrix::HUD_MESSAGE_COUNT);
        c.g[V0][0] = count;
        c.g[A3][0] = 0;
        if count as i64 > 0 {
            c.cycles += 5;
            c.g[T5][0] = metrix::HUD_MESSAGE_IGNORED;
            c.g[T4][0] = sx(!31);
            c.g[T1][0] = bits;
            c.g[T3][0] = 1;
            c.g[T0][0] = 0;
            let (mut i, mut rec) = (0u64, 0u64);
            loop {
                // Registers: a3 = i, t0 = i * 0x9c.
                c.cycles += 5;
                let (m, bits) = (c.g[A2][0], c.g[T1][0]);
                let list = lw(c, m, metrix::HUD_MESSAGES);
                let at = sx((rec as u32).wrapping_add(list as u32));
                let msg = Addr(at as u32).lw(c);
                c.g[V1][0] = msg;
                c.g[V0][0] = msg & sx(!31);
                if msg != metrix::HUD_MESSAGE_IGNORED {
                    c.cycles += 6;
                    let mask = sx(1u32 << (msg as u32 & 31));
                    c.g[A0][0] = mask;
                    let word =
                        sx((bits as u32).wrapping_add(((msg as u32 & !31) as i32 >> 3) as u32));
                    c.g[V0][0] = word;
                    let v = Addr(word as u32).lw(c) | mask;
                    c.g[V1][0] = v;
                    Addr(word as u32).sw(c, v);
                }
                c.cycles += 5;
                let count = lw(c, m, metrix::HUD_MESSAGE_COUNT);
                i = sx((i as u32).wrapping_add(1));
                c.g[A3][0] = i;
                let more = (i as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                rec = sx((rec as u32).wrapping_add(metrix::HUD_MESSAGE_SIZE));
                c.g[T0][0] = rec;
                if !more {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0011_7440);
                    // The generated loop reads its state from the registers.
                    i = c.g[A3][0];
                    rec = c.g[T0][0];
                }
            }
        }
        c.cycles += 1;
        sw(c, c.g[A2][0], metrix::HUD_BITS_DIRTY, 0);
    }
    let bits = c.g[T2][0];
    let id = c.g[A1][0];
    c.cycles += 10;
    c.g[V1][0] = id & sx(!31);
    let mask = sx(1u32 << (id as u32 & 31));
    c.g[A0][0] = mask;
    let word = sx((bits as u32).wrapping_add(((id as u32 & !31) as i32 >> 3) as u32));
    c.g[V1][0] = word;
    let v = Addr(word as u32).lw(c) & mask;
    c.g[V0][0] = (v != 0) as u64;
    crate::abi::ret(c);
}

/// `RiderMetrix_ResetTrick(metrix)` (0x117838): clear the trick in
/// progress: counters and flags 0, `multiplier` 1.0, the timers that mean
/// "unused" -1.0, and the three trick ids (`EA_MemSet`). 50 cycles.
pub fn rider_metrix_reset_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        const MINUS_ONE: u32 = 0xbf80_0000;
        const ONE: u32 = 0x3f80_0000;
        c.f[0] = MINUS_ONE;
        c.f[1] = ONE;
        c.g[AT][0] = ONE as u64;
        let frame = Frame::open(c, 0x10);
        let m = c.g[A0][0];
        c.g[V0][0] = m;
        frame.save_ra(c, 0);
        let f = |c: &mut Cpu, off: u32, v: u32| Addr((m as u32).wrapping_add(off)).write_u32(c, v);
        f(c, 0x78, MINUS_ONE);
        f(c, metrix::MULTIPLIER, ONE);
        f(c, metrix::TRICK_BOOST, 0);
        // 0x00..0x10: landing record; 0x1c..0x2c: rail style / press type
        // and their timers; 0x30..0x40: air time, spin, flip, hold rate,
        // grab time; 0x44..0x5c: grab sums and the class counters.
        for off in [0x1c, 0x00, 0x04, 0x08, 0x0c, 0x10, 0x20] {
            f(c, off, 0);
        }
        f(c, 0x24, MINUS_ONE);
        f(c, 0x28, 0);
        f(c, 0x2c, MINUS_ONE);
        f(c, 0x30, MINUS_ONE);
        for off in [0x34, 0x38, 0x3c] {
            f(c, off, 0);
        }
        f(c, 0x40, MINUS_ONE);
        for off in [0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x70] {
            f(c, off, 0);
        }
        f(c, 0x6c, MINUS_ONE);
        for off in [0x74, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x94, 0x98] {
            f(c, off, 0);
        }
        c.g[A0][0] = ptr_add(m, metrix::TRICK_IDS);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xc;
        call_ptr(c, table::EA_MEM_SET, 0x0011_78f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderModel_FindBone(model, set, name)` (0x310c48): the index of the
/// bone called `name` in bone set `set` (the set's first bone index plus
/// the position of the name in its 0x50-byte name records), -1 if absent.
pub fn rider_model_find_bone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        let model = c.g[A0][0];
        let set = c.g[A1][0];
        c.g[A1][0] = sx((set as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0x58;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        let name = c.g[A2][0];
        c.g[S3][0] = name;
        frame.save_ra(c, 0);
        let set_offsets = lw(c, model, 0x1c);
        let sets = lw(c, model, 0xc);
        c.g[A3][0] = sets;
        let slot = sx((c.g[A1][0] as u32).wrapping_add(set_offsets as u32));
        c.g[A1][0] = slot;
        c.g[V0][0] = Addr(slot as u32).lw(c);
        c.mult(0, V0, V0, V1);
        // The set record (0x58 bytes): +0x04 first bone index, +0x38 the
        // name records, +0x44 their count.
        let rec = sx((sets as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = rec;
        let count = lw(c, rec, 0x44);
        c.g[V0][0] = count;
        let names = lw(c, rec, 0x38);
        c.g[A0][0] = names;
        let found = if count as i64 > 0 {
            c.cycles += 1;
            c.g[S1][0] = names;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, table::STRCMP, 0x0031_0ca4)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    let first = lw(c, rec, 4);
                    break Some(sx((first as u32).wrapping_add(c.g[S0][0] as u32)));
                }
                let count = lw(c, rec, 0x44);
                c.cycles += 4;
                let i = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[S0][0] = i;
                let more = (i as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
                if !more {
                    break None;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0031_0c98);
                }
            }
        } else {
            None
        };
        c.g[V0][0] = match found {
            Some(index) => index,
            None => {
                c.cycles += 1;
                u64::MAX
            }
        };
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FPU and VU0 helpers.

/// The float constant at `gp - off` (`lwc1 f, -off(gp)`: `.sdata` literals).
#[inline(always)]
fn gp_float(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr((c.g[GP][0] as u32).wrapping_sub(off)).read_fpu(c)
}

/// `lwc1` of a field.
#[inline(always)]
fn lwc1(c: &mut Cpu, base: u64, off: u32) -> FpuF32 {
    Addr((base as u32).wrapping_add(off)).read_fpu(c)
}

/// `swc1` to a field.
#[inline(always)]
fn swc1(c: &mut Cpu, base: u64, off: u32, v: FpuF32) {
    Addr((base as u32).wrapping_add(off)).write_fpu(c, v)
}

/// `c.le.s a, b` with the FCC bit latched.
#[inline(always)]
fn c_le_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s a, b` with the FCC bit latched.
#[inline(always)]
fn c_eq_s(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.eq_s(b);
    abi::set_fcc(c, r);
    r
}

/// VU0 macro-mode helpers (the vector library idioms the rider code
/// inlines). They write the VF lanes, ACC lanes and Q the original leaves.
mod vu {
    use crate::ps2f::{vu0_scratch, Vec4, VuF32};
    use ssx3_ee_rt::Cpu;

    pub const X: u8 = 8;
    pub const XYZW: u8 = 0xf;

    /// A VF register.
    pub fn vf(c: &Cpu, reg: usize) -> Vec4 {
        let v = c.bus.plat.vf(reg);
        Vec4 {
            x: VuF32(v[0]),
            y: VuF32(v[1]),
            z: VuF32(v[2]),
            w: VuF32(v[3]),
        }
    }

    /// `lqc2 vf, addr`: load and leave the register set.
    pub fn lqc2(c: &mut Cpu, reg: usize, addr: u32) -> Vec4 {
        let v = crate::guest::Addr(addr).read_vec4(c);
        vu0_scratch::set_vf(c, reg, XYZW, v);
        v
    }

    /// Lane-wise `vmul`.
    pub fn mul(a: Vec4, b: Vec4) -> Vec4 {
        Vec4 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
            w: a.w * b.w,
        }
    }

    /// The four-lane dot product idiom with the operands in vf3/vf5:
    /// `vadd.x vf6, vf0, vf0w` (1.0), `vmul vf4, a, b`,
    /// `vadda.x ACC, vf4, vf4y`, `vmadda.x ACC, vf6, vf4z`,
    /// `vmadd.x vf4, vf6, vf4w`: a·b over x, y, z **and** w in vf4.x.
    /// Writes vf4, vf6.x and ACC.x; returns the vf4 image.
    pub fn dot4(c: &mut Cpu, a: Vec4, b: Vec4) -> Vec4 {
        let one = VuF32::ZERO + VuF32::ONE;
        vu0_scratch::set_vf(
            c,
            6,
            X,
            Vec4 {
                x: one,
                ..Vec4::default()
            },
        );
        let p = mul(a, b);
        let acc = p.x + p.y;
        let acc = VuF32::madd(acc, one, p.z);
        let sum = VuF32::madd(acc, one, p.w);
        vu0_scratch::set_acc(
            c,
            X,
            Vec4 {
                x: acc,
                ..Vec4::default()
            },
        );
        let vf4 = Vec4 { x: sum, ..p };
        vu0_scratch::set_vf(c, 4, XYZW, vf4);
        vf4
    }

    /// `vsqrt Q, ft; vwaitq`: Q = sqrt(x), available at once in macro mode.
    pub fn q_sqrt(c: &mut Cpu, x: VuF32) -> VuF32 {
        let q = VuF32::sqrt(x);
        vu0_scratch::set_q(c, q);
        q
    }

    /// `qmfc2 rt, vf`: the 128-bit GPR image of a VF register.
    pub fn qmfc2(v: Vec4) -> [u64; 2] {
        let b = v.to_bits();
        [b as u64, (b >> 64) as u64]
    }
}

// ---------------------------------------------------------------------------
// Slew targets.

/// `Rider` slews (`+0x1f0`, 12-byte `{cur, maxStep, target}` records).
mod slew {
    /// #0 steer.
    pub const STEER: u32 = 0x1f0;
    /// #3 brake.
    pub const BRAKE: u32 = 0x214;
    /// #4 crouch.
    pub const CROUCH: u32 = 0x220;
    /// #15 / #16 prewind spin / flip (`cur`).
    pub const PREWIND_SPIN: u32 = 0x2a4;
    pub const PREWIND_FLIP: u32 = 0x2b0;
    pub const CUR: u32 = 0;
    pub const MAX_STEP: u32 = 4;
    pub const TARGET: u32 = 8;
}

/// `Rider_SetSteerTarget(rider, steer /*f12*/)` (0x113e80): slew #0's
/// target is the steer axis, scaled down below the reference speed
/// (`gp-0x7c40`); its step grows with the distance to go (`|Δ| × k`,
/// clamped to `[lo, hi]`) and is smaller on surface types 2, 3 and 0xd.
pub fn rider_set_slew0_target(c: &mut Cpu) {
    let rider = c.g[A0][0];
    let mut steer = FpuF32(c.f[12]);
    c.cycles += 18;
    let ref_speed = gp_float(c, 0x7c40);
    let vel = vu::lqc2(c, 3, (rider as u32).wrapping_add(rider::VEL));
    let speed_sq = vu::dot4(c, vel, vel).x;
    let speed = vu::q_sqrt(c, speed_sq);
    c.g[V0][0] = sx(speed.0);
    c.g[AT][0] = 0x3f80_0000;
    let one = FpuF32::ONE;
    let scale = FpuF32(speed.0).div(ref_speed);
    if abi::c_lt_s(c, scale, one) {
        steer = steer * scale;
    }
    c.cycles += 11;
    let cur = lwc1(c, rider, slew::STEER + slew::CUR);
    let gain = gp_float(c, 0x7c3c);
    let lo = gp_float(c, 0x7c38);
    let hi = gp_float(c, 0x7c34);
    let dist = (steer - cur).abs() * gain;
    let mut step = lo;
    if c_le_s(c, lo, dist) {
        c.cycles += 1;
        step = dist.min(hi);
    }
    c.cycles += 5;
    let surface = lw(c, rider, rider::SURFACE_TYPE);
    c.g[V1][0] = surface;
    let soft_surface = (sx((surface as u32).wrapping_sub(2)) < 2) as u64;
    c.g[V0][0] = soft_surface;
    let mut scaled = step;
    let slow = if soft_surface != 0 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0xd;
        surface == 0xd
    };
    if slow {
        c.cycles += 2;
        scaled = scaled * gp_float(c, 0x7c30);
    }
    c.cycles += 5;
    let k = gp_float(c, 0x7c2c);
    swc1(c, rider, slew::STEER + slew::TARGET, steer);
    let max_step = scaled * k;
    swc1(c, rider, slew::STEER + slew::MAX_STEP, max_step);
    // Leftovers.
    c.f[0] = max_step.0;
    c.f[1] = scaled.0;
    c.f[2] = step.0;
    c.f[3] = hi.0;
    c.f[12] = steer.0;
    abi::ret(c);
}

/// `Rider_SetCrouchBrakeTargets(rider, crouch /*f12*/, brake /*f13*/)`
/// (0x113f88): crouch and brake exclude each other: while one is asked for
/// and the other is still non-zero, only the other's target is set to 0.
/// Otherwise slew #4 gets `crouch` with a step from the distance to go, and
/// slew #3 gets `brake`, signed by the lean side (powerslide), cleared when
/// already crouching or moving backward along the ground, with a step
/// `max(|Δ|, lo)` capped at 1, times k.
pub fn rider_set_slew3_and4_targets(c: &mut Cpu) {
    let rider = c.g[A0][0];
    let mut crouch = FpuF32(c.f[12]);
    let mut brake = FpuF32(c.f[13]);
    let zero = FpuF32::ZERO;
    c.f[1] = 0;
    c.cycles += 4;
    if abi::c_lt_s(c, zero, crouch) {
        c.cycles += 4;
        let cur = lwc1(c, rider, slew::BRAKE + slew::CUR);
        c.f[0] = cur.0;
        if !c_eq_s(c, cur, zero) {
            c.cycles += 4;
            // 0.04166 per tick at 60 Hz: a fixed per-tick step.
            let raw = gp_float(c, 0x7c20);
            let step = rate_step_60(c, raw);
            swc1(c, rider, slew::BRAKE + slew::TARGET, zero);
            swc1(c, rider, slew::BRAKE + slew::MAX_STEP, step);
            c.f[0] = step.0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 4;
    let crouch_cur = lwc1(c, rider, slew::CROUCH + slew::CUR);
    c.f[2] = crouch_cur.0;
    if abi::c_lt_s(c, zero, brake) {
        c.cycles += 3;
        if !c_eq_s(c, crouch_cur, zero) {
            c.cycles += 4;
            // 0.1 per tick at 60 Hz: a fixed per-tick step.
            let raw = gp_float(c, 0x7c1c);
            let step = rate_step_60(c, raw);
            swc1(c, rider, slew::CROUCH + slew::TARGET, zero);
            swc1(c, rider, slew::CROUCH + slew::MAX_STEP, step);
            c.f[0] = step.0;
            abi::ret(c);
            return;
        }
    }
    // Slew #4: crouch.
    c.cycles += 5;
    if abi::c_lt_s(c, zero, crouch) {
        c.cycles += 5;
        let cur = lwc1(c, rider, slew::BRAKE + slew::CUR);
        if !c_eq_s(c, cur, zero) {
            crouch = zero;
        }
        c.cycles += 1;
    }
    let delta = crouch - crouch_cur;
    c.cycles += 5;
    let threshold = gp_float(c, 0x7c18);
    let dist = delta.abs();
    // The crouch's step is `a · max(dist, 0.1)` per tick (a = 0.1000018): at
    // another rate the fraction is `1 - (1-a)^(60/rate)` and the floor follows
    // it (the game's floor is a·0.1). At 60 both entries are used unchanged.
    let raw = gp_float(c, 0x7c14);
    let a = smoothing_60(c, raw);
    let step = if c_le_s(c, threshold, dist) {
        c.cycles += 3;
        dist * a
    } else {
        c.cycles += 1;
        if crate::tick::rate(c) == 60 {
            gp_float(c, 0x7c10)
        } else {
            a * threshold
        }
    };
    c.f[1] = threshold.0;
    c.f[2] = dist.0;
    c.cycles += 6;
    c.f[1] = 0;
    swc1(c, rider, slew::CROUCH + slew::TARGET, crouch);
    swc1(c, rider, slew::CROUCH + slew::MAX_STEP, step);
    // Slew #3: brake; no brake while crouched or moving backward.
    let mut tail = 1;
    if abi::c_lt_s(c, zero, brake) {
        c.cycles += 5;
        let crouch_cur = lwc1(c, rider, slew::CROUCH + slew::CUR);
        c.f[0] = crouch_cur.0;
        if abi::c_lt_s(c, zero, crouch_cur) {
            brake = zero;
        } else {
            c.cycles += 13;
            let vel = vu::lqc2(c, 3, (rider as u32).wrapping_add(rider::VEL));
            let fwd = vu::lqc2(c, 5, (rider as u32).wrapping_add(rider::GROUND_FORWARD));
            let along = vu::dot4(c, vel, fwd);
            let image = vu::qmfc2(along);
            c.g[V0] = image;
            let along = FpuF32(image[0] as u32);
            c.f[0] = along.0;
            if abi::c_lt_s(c, along, zero) {
                c.cycles += 1;
                brake = zero;
            } else {
                tail = 0;
            }
        }
    }
    c.cycles += tail;
    let cur = lwc1(c, rider, slew::BRAKE + slew::CUR);
    c.cycles += 6;
    let lean_limit = gp_float(c, 0x7c0c);
    c.f[3] = lean_limit.0;
    let abs_cur = cur.abs();
    c.f[1] = abs_cur.0;
    // Powerslide: the brake takes the sign of the lean side.
    let mut flip = false;
    let mut checked = false;
    if c_le_s(c, abs_cur, lean_limit) {
        c.cycles += 6;
        let steer = lwc1(c, rider, slew::STEER + slew::CUR);
        c.f[1] = steer.0;
        c.f[0] = 0;
        if abi::c_lt_s(c, steer, zero) {
            flip = true;
            checked = true;
        }
    }
    if !checked {
        c.cycles += 5;
        let k = gp_float(c, 0x7c08);
        c.f[0] = k.0;
        if abi::c_lt_s(c, cur, k) {
            c.cycles += 5;
            let steer = lwc1(c, rider, slew::STEER + slew::CUR);
            c.f[0] = steer.0;
            if abi::c_lt_s(c, steer, lean_limit) {
                c.cycles += 1;
                flip = true;
            }
        }
    }
    if flip {
        brake = -brake;
        c.cycles += 1;
    }
    let delta = brake - cur;
    c.cycles += 8;
    let lo = gp_float(c, 0x7c04);
    let dist = delta.abs();
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = FpuF32::ONE.0;
    let mut amount = lo;
    if c_le_s(c, lo, dist) {
        amount = dist.min(FpuF32::ONE);
    }
    c.cycles += 5;
    // `clamp(|Δ|, 0.1, 1) · k` per tick: the fraction follows the rate.
    let raw = gp_float(c, 0x7c00);
    let k = smoothing_60(c, raw);
    swc1(c, rider, slew::BRAKE + slew::TARGET, brake);
    let step = amount * k;
    swc1(c, rider, slew::BRAKE + slew::MAX_STEP, step);
    c.f[0] = step.0;
    c.f[1] = amount.0;
    c.f[12] = crouch.0;
    c.f[13] = brake.0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Animation sequence updaters (called from `RiderAnim_UpdateSequence`).

/// The 0x40 frame of the sequence updaters: `s0` = the sequence (a2),
/// `s1` = the list (a1), `f20` = the time step (saved at +0x30).
fn open_seq_frame(c: &mut Cpu) -> Frame {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A2][0];
    frame.save_fpr(c, 20, 0x30);
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    frame
}

/// Epilogue of [`open_seq_frame`] (`+6`).
fn close_seq_frame(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
}

/// `RiderAnim_UpdateSeqType07(anim, list, seq, dt /*f12*/)` (0x104358):
/// a prewind wind-up sequence. Its track time follows the larger of the
/// prewind spin/flip slews (#15/#16) times the sequence's scale (+0x10);
/// the weight fades by `dt × k`; a finished sequence is removed from the
/// list. Returns whether it was removed.
pub fn rider_anim_update_seq_type07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = open_seq_frame(c);
        let (list, seq) = (c.g[S1][0], c.g[S0][0]);
        let dt = FpuF32(c.f[12]);
        c.f[20] = dt.0;
        let rider = lw(c, c.g[A0][0], anim::RIDER);
        c.g[V0][0] = rider;
        let flip = lwc1(c, rider, slew::PREWIND_FLIP).abs();
        let spin = lwc1(c, rider, slew::PREWIND_SPIN).abs();
        c.f[0] = flip.0;
        c.f[1] = spin.0;
        let mut wind = flip;
        if abi::c_lt_s(c, flip, spin) {
            c.cycles += 1;
            wind = spin;
        }
        c.f[2] = wind.0;
        c.cycles += 11;
        let scale = lwc1(c, seq, seq_field::TIME_SCALE);
        c.f[12] = (wind * scale).0;
        c.g[A0][0] = seq;
        c.g[A1][0] = 0;
        call_ptr(c, table::ANIM_SEQUENCE_SET_TRACK_TIME, 0x0010_43b4)?;
        let k = gp_float(c, 0x7f58);
        c.f[12] = (dt * k).0;
        c.g[A0][0] = seq;
        call_ptr(c, table::ANIM_SEQUENCE_UPDATE_WEIGHT_FADE, 0x0010_43c4)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = list;
        if faded {
            c.cycles += 3;
            c.g[A1][0] = seq;
            call_ptr(c, table::ANIM_SEQ_LIST_REMOVE, 0x0010_43dc)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        close_seq_frame(c, frame);
        Ok(())
    })
}

/// `RiderAnim_UpdateSeqType02(anim, list, seq, dt /*f12*/)` (0x1049c0): a
/// one-shot sequence: advance its track by `dt × k`; when the track has
/// ended (or else when the weight fade finished) remove it from the list.
/// Returns whether it was removed.
pub fn rider_anim_update_seq_type02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let k = gp_float(c, 0x7f3c);
        c.f[0] = k.0;
        let step = FpuF32(c.f[12]) * k;
        let frame = open_seq_frame(c);
        c.f[20] = step.0;
        let (list, seq) = (c.g[S1][0], c.g[S0][0]);
        c.g[A1][0] = 0;
        c.g[A0][0] = seq;
        c.f[12] = step.0;
        call_ptr(c, table::ANIM_SEQUENCE_ADVANCE_TRACK, 0x0010_49f4)?;
        let ended = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !ended {
            c.cycles += 3;
            c.f[12] = step.0;
            c.g[A0][0] = seq;
            call_ptr(c, table::ANIM_SEQUENCE_UPDATE_WEIGHT_FADE, 0x0010_4a08)?;
        }
        c.cycles += 2;
        let remove = c.g[V0][0] != 0;
        c.g[A0][0] = list;
        if remove {
            c.cycles += 4;
            c.g[A1][0] = seq;
            call_ptr(c, table::ANIM_SEQ_LIST_REMOVE, 0x0010_4a18)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        close_seq_frame(c, frame);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Turning around.

/// `Rider_TurnAroundIfBackward(rider)` (0x114cc0): when not braking and the
/// velocity points backward along the ground (`vel · groundForward < -k`),
/// turn the rider half way round its up axis (the orientation becomes
/// `(up, 0) ⊗ rot`, a 180° rotation), rebuild the world matrix and switch
/// stance. Returns whether it turned.
pub fn rider_turn_around_if_backward(c: &mut Cpu) {
    guest(c, |c| {
        use crate::ps2f::{vu0_scratch, Vec4, VuF32};
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.f[2] = 0;
        frame.save(c, S0, 0x60);
        let ra_at = frame.at(c, 0x50);
        c.sd(ra_at, c.g[RA][0]);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        let brake = lwc1(c, rider, slew::BRAKE + slew::CUR);
        c.f[0] = brake.0;
        let braking = !c_eq_s(c, brake, FpuF32::ZERO);
        c.g[V0][0] = 0;
        if !braking {
            c.cycles += 15;
            let k = gp_float(c, 0x7b98);
            let vel = vu::lqc2(c, 3, (rider as u32).wrapping_add(rider::VEL));
            let fwd = vu::lqc2(c, 5, (rider as u32).wrapping_add(rider::GROUND_FORWARD));
            let image = vu::qmfc2(vu::dot4(c, vel, fwd));
            c.g[V0] = image;
            let along = FpuF32(image[0] as u32);
            c.f[1] = along.0;
            c.f[0] = (-k).0;
            if abi::c_lt_s(c, -k, along) {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 32;
                // The up axis (world matrix row z) as a pure quaternion,
                // built through the stack as the original does.
                let up_row = (rider as u32).wrapping_add(rider::WORLD_UP);
                c.lq(V0, up_row);
                let sp = crate::call::sp(c);
                c.g[A0][0] = rider;
                c.swc1(2, sp + 0x1c);
                c.sq(V0, sp + 0x30);
                let up = Addr(sp + 0x30).read_vec4(c);
                c.f[0] = up.x.0;
                c.f[1] = up.y.0;
                c.f[2] = up.z.0;
                for base in [0x10, 0x20] {
                    Addr(sp + base).write_u32(c, up.x.0);
                    Addr(sp + base + 4).write_u32(c, up.y.0);
                    Addr(sp + base + 8).write_u32(c, up.z.0);
                }
                let a = vu::lqc2(c, 4, sp + 0x10);
                let b = vu::lqc2(c, 5, (rider as u32).wrapping_add(rider::ROT));
                // Quaternion product a ⊗ b (VU0 idiom: OPMULA/OPMSUB for the
                // cross product, MULA/MADDA for the scaled vectors, w from
                // the dot product).
                let prod = vu::mul(a, b);
                vu0_scratch::set_vf(c, 7, vu::XYZW, prod);
                let cross = Vec4 {
                    x: VuF32::msub(a.y * b.z, b.y, a.z),
                    y: VuF32::msub(a.z * b.x, b.z, a.x),
                    z: VuF32::msub(a.x * b.y, b.x, a.y),
                    w: VuF32::ZERO,
                };
                vu0_scratch::set_vf(c, 6, 0b1110, cross);
                let acc = Vec4 {
                    x: VuF32::madd(a.x * b.w, b.x, a.w),
                    y: VuF32::madd(a.y * b.w, b.y, a.w),
                    z: VuF32::madd(a.z * b.w, b.z, a.w),
                    w: VuF32::msub(prod.w - prod.x, VuF32::ONE, prod.y),
                };
                vu0_scratch::set_acc(c, vu::XYZW, acc);
                let q = Vec4 {
                    x: VuF32::madd(acc.x, cross.x, VuF32::ONE),
                    y: VuF32::madd(acc.y, cross.y, VuF32::ONE),
                    z: VuF32::madd(acc.z, cross.z, VuF32::ONE),
                    w: VuF32::msub(acc.w, VuF32::ONE, prod.z),
                };
                vu0_scratch::set_vf(c, 8, vu::XYZW, q);
                Addr(sp + 0x40).write_vec4(c, q);
                c.lq(V0, sp + 0x40);
                c.sq(V0, (rider as u32).wrapping_add(rider::ROT));
                c.sq(V0, sp);
                callee::RIDER_BUILD_WORLD_MATRIX.call(c, 0x0011_4d98)?;
                c.g[A0][0] = rider;
                callee::RIDER_TOGGLE_STANCE.call(c, 0x0011_4da0)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x60);
        let ra_at = frame.at(c, 0x50);
        c.g[RA][0] = c.ld(ra_at);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Boost.

/// `Rider_HandleBoostInput(rider, held, pressed)` (0x114130, proposed
/// `Rider_UpdateBoost`): a press is reported to the cheat-code detector.
/// While held with meter left: on the first frame latch `boostStrength`
/// from the meter (> k1: 1.0, > k2: 0.625, else 0.25) and start the boost
/// sound; then drain the meter by k3 per frame unless the power-up boost
/// runs, `boostMode` is 0 (unlimited) or `uberLevel` >= 10. Held with an
/// empty meter, a press plays the "no boost" sound. On release (or empty)
/// a running boost stops its sound and `boostStrength` is cleared.
pub fn rider_handle_boost_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save(c, S2, 0x10);
        let pressed = c.g[A2][0];
        c.g[S1][0] = pressed;
        frame.save_ra(c, 0);
        let held = c.g[A1][0];
        c.g[S2][0] = held;
        frame.save_fpr(c, 20, 0x40);
        if pressed != 0 {
            c.cycles += 4;
            let owner = lw(c, rider, rider::OWNER);
            c.g[A1][0] = 0;
            c.g[A0][0] = ptr_add(owner, base::CHEAT_DETECTOR);
            call_ptr(c, table::CHEAT_CODE_DETECTOR_POST_EVENT, 0x0011_4168)?;
        }
        let zero = FpuF32::ZERO;
        // `Some(())` when the boost is stopped/cleared (release path).
        let mut release = false;
        let mut tail = 1;
        c.cycles += 2;
        if held == 0 {
            release = true;
        } else {
            c.cycles += 5;
            let meter = lwc1(c, rider, rider::BOOST_METER);
            c.f[1] = meter.0;
            c.f[2] = 0;
            if abi::c_lt_s(c, zero, meter) {
                c.cycles += 5;
                let strength = lwc1(c, rider, rider::BOOST_STRENGTH);
                c.f[0] = strength.0;
                if c_eq_s(c, strength, zero) {
                    // Boost starts: latch its strength.
                    c.cycles += 6;
                    let full = gp_float(c, 0x7bfc);
                    let mut strength = FpuF32::ONE;
                    c.g[AT][0] = 0x3f80_0000;
                    if !abi::c_lt_s(c, full, meter) {
                        c.cycles += 6;
                        let low = gp_float(c, 0x7bf8);
                        let more = abi::c_lt_s(c, low, meter);
                        c.g[AT][0] = 0x3e80_0000;
                        strength = FpuF32(0x3e80_0000);
                        if more {
                            c.cycles += 2;
                            c.g[AT][0] = 0x3f20_0000;
                            strength = FpuF32(0x3f20_0000);
                        }
                    }
                    c.f[0] = strength.0;
                    c.cycles += 7;
                    swc1(c, rider, rider::BOOST_STRENGTH, strength);
                    call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_41d8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = pressed;
                    c.g[A1][0] = rider;
                    call_ptr(c, table::SSX_AUDIO_SYSTEM_START_BOOST_SOUND, 0x0011_41e8)?;
                }
                // Drain the meter.
                let free_boost = lwc1(c, rider, rider::POWER_UP_BOOST_TIMER);
                c.f[1] = free_boost.0;
                c.cycles += 5;
                c.f[0] = 0;
                tail = 0;
                if c_eq_s(c, free_boost, zero) {
                    c.cycles += 3;
                    let mode = lw(c, rider, rider::BOOST_MODE);
                    c.g[V0][0] = mode;
                    if mode != 0 {
                        c.cycles += 4;
                        let level = lw(c, rider, rider::UBER_LEVEL);
                        let limited = ((level as i64) < 10) as u64;
                        c.g[V0][0] = limited;
                        if limited != 0 {
                            c.cycles += 5;
                            let meter = lwc1(c, rider, rider::BOOST_METER);
                            let drain = gp_float(c, 0x7bf4);
                            // 0.0015068 per tick at 60 Hz (~11 s from full):
                            // the meter lasts the same wall-clock time at any
                            // rate (f1 keeps the per-tick amount, as at 60).
                            let drain = rate_step_60(c, drain);
                            c.f[1] = drain.0;
                            let left = meter - drain;
                            c.f[0] = left.0;
                            swc1(c, rider, rider::BOOST_METER, left);
                            tail = 1;
                        }
                    }
                }
            } else {
                c.cycles += 2;
                if pressed != 0 {
                    c.cycles += 6;
                    call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_4240)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = rider;
                    call_ptr(c, table::SSX_AUDIO_SYSTEM_NO_BOOST_SOUND, 0x0011_424c)?;
                }
                release = true;
            }
        }
        if release {
            let strength = lwc1(c, rider, rider::BOOST_STRENGTH);
            c.f[0] = strength.0;
            c.cycles += 5;
            c.f[20] = 0;
            if abi::c_lt_s(c, zero, strength) {
                c.cycles += 6;
                call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_426c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = rider;
                call_ptr(c, table::SSX_AUDIO_SYSTEM_STOP_BOOST_SOUND, 0x0011_4278)?;
            }
            swc1(c, rider, rider::BOOST_STRENGTH, zero);
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_AddBoost(rider, amount /*f12*/, source)` (0x10e098): add to the
/// boost meter, clamped to [0, 1]. Nothing with `boostMode` 3 (no boost) or
/// a zero amount; a positive amount also asks the back end for the
/// character's boost stat (interface 3, its result unused) and needs its
/// source bit in `boostSourceMask`. The meter sound follows the change.
/// Filling the meter with uber tricks allowed starts the uber time: HUD
/// message 5 when it wasn't running, `uberLevel` at least 1, and while the
/// level is below 10 the uber timer becomes at least 20 s. A negative
/// amount below level 10 ends the uber time.
pub fn rider_add_boost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = c.g[A0][0];
        c.g[S0][0] = rider;
        frame.save_fpr(c, 20, 0x30);
        let source = c.g[A1][0];
        c.g[S1][0] = source;
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x38);
        let mode = lw(c, rider, rider::BOOST_MODE);
        c.g[V1][0] = mode;
        let amount = FpuF32(c.f[12]);
        c.f[20] = amount.0;
        let zero = FpuF32::ZERO;
        // `None`: return at once; `Some(t)`: the uber timer to store.
        let mut uber_timer: Option<FpuF32> = None;
        let mut out = mode == 3;
        'body: {
            if out {
                break 'body;
            }
            c.cycles += 5;
            c.f[21] = 0;
            if c_eq_s(c, amount, zero) {
                break 'body;
            }
            c.cycles += 3;
            if abi::c_lt_s(c, zero, amount) {
                c.cycles += 13;
                call_ptr(c, table::BE_LIBRARY_GET_INSTANCE, 0x0010_e0f0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 3;
                call_ptr(c, table::BE_LIBRARY_GET_INTERFACE, 0x0010_e0fc)?;
                c.g[A1][0] = lw(c, rider, rider::INDEX);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = lw(c, rider, rider::STATS_INDEX);
                call_ptr(c, table::BE_CHAR_INT_GET_ATTRIB2_FACTOR, 0x0010_e10c)?;
                let mask = lw(c, rider, rider::BOOST_SOURCE_MASK);
                let allowed = source & mask;
                c.g[V0][0] = allowed;
                if allowed == 0 {
                    break 'body;
                }
            }
            // Add to the meter.
            c.cycles += 15;
            call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0010_e124)?;
            c.f[12] = lwc1(c, rider, rider::BOOST_METER).0;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = rider;
            c.f[13] = amount.0;
            call_ptr(c, table::SSX_AUDIO_SYSTEM_ON_BOOST_ADDED, 0x0010_e138)?;
            let sum = lwc1(c, rider, rider::BOOST_METER) + amount;
            c.f[0] = sum.0;
            c.g[AT][0] = 0x3f80_0000;
            let one = FpuF32::ONE;
            c.f[1] = one.0;
            let mut meter = zero;
            if c_le_s(c, zero, sum) {
                c.cycles += 1;
                meter = sum.min(one);
            }
            c.f[2] = meter.0;
            c.cycles += 4;
            let full = c_eq_s(c, meter, one);
            swc1(c, rider, rider::BOOST_METER, meter);
            let mut uber = false;
            if full {
                c.cycles += 3;
                if abi::c_lt_s(c, zero, amount) {
                    c.cycles += 3;
                    let flags = lw(c, rider, rider::UBER_FLAGS);
                    c.g[V0][0] = flags;
                    uber = flags != 0;
                }
            }
            if uber {
                // The meter just filled: uber time.
                c.cycles += 5;
                let timer = lwc1(c, rider, rider::UBER_TIMER);
                c.f[0] = timer.0;
                c.f[12] = 0;
                if c_eq_s(c, timer, zero) {
                    c.cycles += 3;
                    c.g[A0][0] = rider;
                    c.g[A1][0] = 5;
                    callee::RIDER_SET_HUD_MESSAGE.call(c, 0x0010_e1a4)?;
                }
                c.cycles += 3;
                let mut level = lw(c, rider, rider::UBER_LEVEL);
                if level as i64 > 0 {
                    level = sx((level as u32).wrapping_sub(1));
                } else {
                    c.cycles += 4;
                    sw(
                        c,
                        rider,
                        rider::UBER_LEVEL,
                        sx((level as u32).wrapping_add(1)),
                    );
                    level = sx((lw(c, rider, rider::UBER_LEVEL) as u32).wrapping_sub(1));
                }
                c.g[V0][0] = level;
                c.cycles += 3;
                // Levels 1..9 (`level - 1 < 9` unsigned).
                let below_ten = (level < 9) as u64;
                c.g[V0][0] = below_ten;
                if below_ten == 0 {
                    break 'body;
                }
                c.cycles += 5;
                let timer = lwc1(c, rider, rider::UBER_TIMER);
                c.g[AT][0] = 0x41a0_0000;
                let twenty = FpuF32(0x41a0_0000);
                c.f[1] = twenty.0;
                uber_timer = Some(timer.max(twenty));
            } else {
                c.cycles += 5;
                c.f[0] = 0;
                if !abi::c_lt_s(c, amount, zero) {
                    break 'body;
                }
                c.cycles += 4;
                let level = lw(c, rider, rider::UBER_LEVEL);
                let below_ten = ((level as i64) < 10) as u64;
                c.g[V0][0] = below_ten;
                if below_ten == 0 {
                    break 'body;
                }
                uber_timer = Some(zero);
            }
        }
        if let Some(t) = uber_timer {
            c.f[0] = t.0;
            c.cycles += 1;
            swc1(c, rider, rider::UBER_TIMER, t);
            out = true;
        }
        if out {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 6;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    //! Differential tests against the generated functions. Callees run for
    //! real (the game's image is loaded) on objects built so that their
    //! virtual calls land in the fakes of `difftest::fakes`, which log the
    //! registers at the call and unwind at a chosen call index. Branches
    //! are chosen by the seed so every path is taken.
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::*;
    use crate::difftest::{check, check_paths, elf_image, fresh, put_words, Rng, GAME_GP, RA};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::Cpu;

    const SEEDS: u32 = 300;

    /// `check`, plus how many seeds reached a fake and how many unwound
    /// (asserted against the minimums given).
    fn run(
        name: &str,
        gen: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        seeds: u32,
        min: (u32, u32),
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        check(gen, hand, seeds, setup);
        let (mut reached, mut unwound) = (0, 0);
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            reached += (c.bus.read32(LOG_COUNT) > 0) as u32;
            unwound += (c.pc != RA) as u32;
        }
        eprintln!("{name}: {seeds} seeds, {reached} reached a fake, {unwound} unwound");
        assert!(
            reached >= min.0 && unwound >= min.1,
            "{name}: reached {reached}, unwound {unwound}"
        );
    }

    /// The game image plus a back-end library whose interfaces are fakes
    /// (deep callees asking the back end for services stop at a fake).
    fn game_world(c: &mut Cpu, r: &mut Rng) {
        elf_image(c);
        for off in [0x1f04, 0x1f28] {
            let singleton = ptr(r);
            c.bus.write32(GAME_GP - off, singleton);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        for i in 0..8 {
            let lib = ptr(r);
            c.bus.write32(0x0053_08a8 + 4 * i, lib);
            let vt = vtable(c, r, &[8]);
            c.bus.write32(lib + 4, vt);
        }
    }

    /// A state id: mostly valid (`0..n`), sometimes out of range.
    fn state_id(r: &mut Rng, seed: u32, n: u32) -> u32 {
        match seed % (n + 2) {
            k if k < n => k,
            k if k == n => n + r.next_u32() % 8,
            _ => r.next_u32() | 0x8000_0000,
        }
    }

    /// A `RiderBase` in a0 (a1: an input record) whose state objects point
    /// back at an empty rider; the state word at `field` is `state`.
    fn rider_base(c: &mut Cpu, seed: u32, start: u32, field: u32, state: u32) {
        let mut r = machine(c, seed, 0x11, 3);
        c.pc = start;
        game_world(c, &mut r);
        let base = ptr(&mut r);
        c.g[A0][0] = base as u64;
        c.g[A1][0] = ptr(&mut r) as u64;
        c.bus.write32(base + field, state);
        let rider = ptr(&mut r);
        for vb in [
            0x18, 0x24, 0x70, 0xa0, 0x100, 0x1b0, 0x1c0, 0x1e4, 0x1f0, 0x200, 0x224, 0x288, 0x2a4,
            0x2b8, 0x340, 0x358, 0x364, 0x384, 0x398,
        ] {
            c.bus.write32(base + vb, rider);
        }
        c.bus.write32(rider + 0x77c, base);
    }

    /// Whether `f` returns within a few million cycles after `setup` (some
    /// real state handlers never finish on an empty rider, e.g. the air
    /// predictor's simulation). An event hook past the budget unwinds the
    /// host stack without a panic message.
    fn terminates(f: fn(&mut Cpu), setup: &dyn Fn(&mut Cpu)) -> bool {
        fn stop(_: &mut Cpu) -> bool {
            std::panic::resume_unwind(Box::new(()))
        }
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut c = fresh();
            setup(&mut c);
            c.hooks.event = stop;
            c.next_event = c.cycles + 3_000_000;
            f(&mut c);
        }))
        .is_ok()
    }

    #[test]
    fn state_dispatchers() {
        for (name, gen, hand, start, field, n) in [
            (
                "update_main_state",
                ssx3_ee::s00::f_00111408 as fn(&mut Cpu),
                super::rider_base_update_main_state as fn(&mut Cpu),
                0x0011_1408,
                0xde0,
                6,
            ),
            (
                "post_update_main_state",
                ssx3_ee::s00::f_001114a0,
                super::rider_base_post_update_main_state,
                0x0011_14a0,
                0xde0,
                6,
            ),
            (
                "handle_sub_state_input",
                ssx3_ee::s00::f_00111728,
                super::rider_base_handle_sub_state_input,
                0x0011_1728,
                0xde4,
                14,
            ),
        ] {
            // Every state id (and out-of-range ones); seeds whose handler
            // never finishes on the test rider fall back to state 0.
            let seeds = SEEDS * 2;
            let mut skipped = 0;
            let states: Vec<u32> = (0..seeds)
                .map(|seed| {
                    let mut r = Rng(seed as u64 * 0x9e37_79b9 + 1);
                    let st = state_id(&mut r, seed, n);
                    if terminates(gen, &|c| rider_base(c, seed, start, field, st)) {
                        st
                    } else {
                        skipped += 1;
                        0
                    }
                })
                .collect();
            eprintln!("{name}: {skipped} seeds fell back to state 0");
            run(name, gen, hand, seeds, (seeds / 5, 10), &|c, seed| {
                rider_base(c, seed, start, field, states[seed as usize])
            });
        }
    }

    #[test]
    fn jump_table_miss() {
        // A corrupted jump table entry leaves through `jump_out`.
        check(
            ssx3_ee::s00::f_00111408,
            super::rider_base_update_main_state,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12, 1);
                c.pc = 0x0011_1408;
                let base = ptr(&mut r);
                c.g[A0][0] = base as u64;
                c.bus.write32(base + 0xde0, seed % 6);
                put_words(
                    c,
                    0x0045_6b50,
                    &[[0x0011_1440, 0x0011_1444, 0x0011_1448, 0x0011_1450][(seed % 4) as usize]; 6],
                );
            },
        );
    }

    #[test]
    fn record_cheat_code_input() {
        check_paths(
            ssx3_ee::s09::f_002f6ac8,
            super::rider_base_record_cheat_code_input,
            SEEDS,
            &[7, 9, 10],
            &|c, seed| {
                let mut r = machine(c, seed, 0x13, 0);
                let d = ptr(&mut r);
                c.g[A0][0] = d as u64;
                c.bus.write8(d + 7, r.next_u32() as u8);
                c.g[A1][0] = match seed % 6 {
                    k @ 0..=2 => k as u64,
                    3 => (r.next_u32() % 100) as u64 + 3,
                    4 => (r.next_u32() as u64) << 32 | 1,
                    _ => (-(r.next_u32() as i64 % 100) - 1) as u64,
                };
            },
        );
    }

    #[test]
    fn hud_message_active() {
        check_events(
            ssx3_ee::s00::f_00117400,
            super::rider_metrix_is_hud_message_active,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14, 0);
                let m = ptr(&mut r);
                let mode = schedule(c, &mut r, seed, 0x0011_7400);
                c.g[A0][0] = m as u64;
                c.g[A1][0] = (r.next_u32() % 64) as u64;
                let n = if seed % 7 == 0 { 0 } else { r.next_u32() % 12 };
                let list = if seed % 11 == 0 { 0 } else { ptr(&mut r) };
                c.bus.write32(m + 0x1b0, list);
                c.bus.write32(
                    m + 0x1b4,
                    if seed % 13 == 0 {
                        (r.next_u32() | 0x8000_0000) & !0xff
                    } else {
                        n
                    },
                );
                c.bus.write32(m + 0x1b8, r.next_u32() & r.next_u32());
                c.bus.write32(m + 0x1bc, r.next_u32() & r.next_u32());
                c.bus
                    .write32(m + 0x1c0, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
                for i in 0..n {
                    let id = if r.next_u32() % 4 == 0 {
                        0x34
                    } else {
                        r.next_u32() % 64
                    };
                    c.bus.write32(list + 0x9c * i, id);
                }
                mode
            },
        );
    }

    #[test]
    fn reset_trick() {
        run(
            "reset_trick",
            ssx3_ee::s00::f_00117838,
            super::rider_metrix_reset_trick,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 0x15, 1);
                let m = ptr(&mut r);
                crate::difftest::fill(c, &mut r, m, 0x30);
                c.g[A0][0] = m as u64;
            },
        );
    }

    #[test]
    fn find_bone() {
        check_events(
            ssx3_ee::s10::f_00310c48,
            super::rider_model_find_bone,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16, 0);
                elf_image(c);
                let mode = schedule(c, &mut r, seed, 0x0031_0c48);
                let model = ptr(&mut r);
                let sets = ptr(&mut r);
                let offsets = ptr(&mut r);
                c.g[A0][0] = model as u64;
                let set = r.next_u32() % 4;
                c.g[A1][0] = set as u64 | (r.next_u32() as u64) << 32;
                c.bus.write32(model + 0xc, sets);
                c.bus.write32(model + 0x1c, offsets);
                c.bus.write32(offsets + 4 * set, set * 3);
                let rec = sets + set * 3 * 0x58;
                let n = if seed % 9 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 8
                };
                let names = ptr(&mut r);
                c.bus.write32(rec + 0x38, names);
                c.bus.write32(rec + 0x44, n);
                c.bus.write32(rec + 4, r.next_u32() % 40);
                let words = [
                    b"hips\0".as_slice(),
                    b"head\0",
                    b"handleft\0",
                    b"footright\0",
                    b"board_rootg\0",
                ];
                for i in 0..n {
                    let w = words[(r.next_u32() % 5) as usize];
                    for (k, b) in w.iter().enumerate() {
                        c.bus.write8(names + 0x50 * i + k as u32, *b);
                    }
                }
                let name = ptr(&mut r) + 0x8;
                let w = words[(r.next_u32() % 5) as usize];
                for (k, b) in w.iter().enumerate() {
                    c.bus.write8(name + k as u32, *b);
                }
                c.g[A2][0] = name as u64;
                mode
            },
        );
    }

    /// A rider with random slews, velocity, ground frame and surface type,
    /// and the game's `.sdata` constants.
    fn slew_rider(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        elf_image(c);
        let p = ptr(r);
        c.g[A0][0] = p as u64;
        for off in (0x1f0..0x230).step_by(4) {
            let v = match r.next_u32() % 4 {
                0 => 0,
                1 => r.edge_f32(),
                _ => r.f32(1.5),
            };
            c.bus.write32(p + off, v);
        }
        for i in 0..4 {
            c.bus.write32(
                p + 0x1e0 + 4 * i,
                if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(3000.0)
                },
            );
            c.bus.write32(p + 0x3a0 + 4 * i, r.f32(1.0));
        }
        c.bus.write32(
            p + 0x438,
            [0, 1, 2, 3, 0xd, 0xe, r.next_u32()][(r.next_u32() % 7) as usize],
        );
        p
    }

    fn axis(r: &mut Rng) -> u32 {
        match r.next_u32() % 5 {
            0 => 0,
            1 => r.edge_f32(),
            2 => 0x3f80_0000,
            _ => r.f32(1.2),
        }
    }

    #[test]
    fn set_slew0_target() {
        check(
            ssx3_ee::s00::f_00113e80,
            super::rider_set_slew0_target,
            2000,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17, 0);
                slew_rider(c, &mut r, seed);
                c.f[12] = axis(&mut r);
            },
        );
    }

    #[test]
    fn set_slew3_and4_targets() {
        let paths: std::collections::BTreeSet<u64> = (0..3000)
            .map(|seed| {
                let mut c = fresh();
                slew_setup(&mut c, seed);
                let before = c.cycles;
                ssx3_ee::s00::f_00113f88(&mut c);
                c.cycles - before
            })
            .collect();
        eprintln!("set_slew3_and4_targets paths: {paths:?}");
        check(
            ssx3_ee::s00::f_00113f88,
            super::rider_set_slew3_and4_targets,
            3000,
            &|c, seed| slew_setup(c, seed),
        );
    }

    /// `Rider_SetCrouchBrakeTargets`'s (0x113f88) slew steps are per tick: the
    /// approach fractions (`dist · a` for the crouch, `clamp(|Δ|, 0.1, 1) · k`
    /// for the brake) become `1 - (1-a)^(60/rate)`, and the two fixed "clear
    /// the other slew" steps `a · 60/rate`, so both slews move the same amount
    /// per second at any rate. At 60 the loaded entries are used unchanged.
    #[test]
    fn set_slew3_and4_targets_follows_the_rate() {
        use crate::tick;
        // The game's entries: crouch fraction, its floor (a·0.1), brake
        // fraction, 0.1.
        const A: u32 = 0x3dcc_cdc2; // 0.100001827
        const FLOOR: u32 = 0x3c23_d7cf; // 0.010000183
        const K: u32 = 0x3d2a_a635; // 0.0416624136
        const TENTH: u32 = 0x3dcc_cccd;
        const MINUS_TENTH: u32 = 0xbdcc_cccd;
        // The crouch (`+0x220`) and brake (`+0x214`) `maxStep` the function
        // writes for a crouch request, a brake request, and the two "clear the
        // other slew" cases.
        let steps =
            |hz: u32, crouch: f32, brake: f32, crouch_cur: f32, brake_cur: f32| -> (u32, u32) {
                let mut c = fresh();
                c.g[GP][0] = GAME_GP as u64;
                let p = 0x0080_0000u32;
                c.g[A0][0] = p as u64;
                c.bus.write32(p + 0x1f0, 0); // slew #0 (steer) cur
                c.bus.write32(p + 0x214, brake_cur.to_bits());
                c.bus.write32(p + 0x220, crouch_cur.to_bits());
                for (off, bits) in [
                    (0x7c20u32, K),
                    (0x7c1c, A),
                    (0x7c18, TENTH),
                    (0x7c14, A),
                    (0x7c10, FLOOR),
                    (0x7c0c, TENTH),
                    (0x7c08, MINUS_TENTH),
                    (0x7c04, TENTH),
                    (0x7c00, K),
                ] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), bits);
                }
                c.f[12] = crouch.to_bits();
                c.f[13] = brake.to_bits();
                tick::set_for_test(&mut c, hz);
                super::rider_set_slew3_and4_targets(&mut c);
                (c.bus.read32(p + 0x224), c.bus.read32(p + 0x218))
            };
        // 60 Hz: the entries as stored.
        assert_eq!(steps(60, 1.0, 0.0, 0.0, 0.0).0, A, "crouch step");
        assert_eq!(steps(60, 0.0, 1.0, 0.0, 0.0).1, K, "brake step");
        assert_eq!(steps(60, 1.0, 0.0, 0.0, 0.5).1, K, "clear brake");
        assert_eq!(steps(60, 0.0, 1.0, 0.5, 0.0).0, A, "clear crouch");
        for hz in [120u32, 240] {
            let frac = 60.0f32 / hz as f32;
            let a = 1.0f32 - (1.0f32 - f32::from_bits(A)).powf(frac);
            let k = 1.0f32 - (1.0f32 - f32::from_bits(K)).powf(frac);
            let crouch = f32::from_bits(steps(hz, 1.0, 0.0, 0.0, 0.0).0);
            assert!(
                (crouch - a).abs() < 1e-7,
                "{hz} Hz crouch {crouch}, want {a}"
            );
            let brake = f32::from_bits(steps(hz, 0.0, 1.0, 0.0, 0.0).1);
            assert!((brake - k).abs() < 1e-7, "{hz} Hz brake {brake}, want {k}");
            assert_eq!(
                steps(hz, 1.0, 0.0, 0.0, 0.5).1,
                (f32::from_bits(K) * frac).to_bits(),
                "{hz} Hz clear brake"
            );
            assert_eq!(
                steps(hz, 0.0, 1.0, 0.5, 0.0).0,
                (f32::from_bits(A) * frac).to_bits(),
                "{hz} Hz clear crouch"
            );
        }
    }

    fn slew_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x18, 0);
        let p = slew_rider(c, &mut r, seed);
        c.f[12] = axis(&mut r);
        c.f[13] = axis(&mut r);
        // Often one of the two slews already at zero.
        if seed % 3 == 0 {
            c.bus.write32(p + 0x214, 0);
        }
        if seed % 4 == 0 {
            c.bus.write32(p + 0x220, 0);
        }
    }

    /// An animation sequence with random times, rates and weights (no
    /// triggers), and a list (a1) that holds some other sequences: the
    /// removal walks it without finding the sequence.
    fn seq_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        elf_image(c);
        let seq = ptr(r);
        let list = ptr(r);
        c.g[A1][0] = list as u64;
        c.g[A2][0] = seq as u64;
        for off in [0x8, 0xc, 0x10, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
            let v = match r.next_u32() % 4 {
                0 => 0,
                1 => r.edge_f32(),
                _ => r.f32(3.0),
            };
            c.bus.write32(seq + off, v);
        }
        c.bus.write32(seq + 0x1c, r.next_u32() & 1);
        c.bus.write32(seq + 0xa0, r.next_u32() & 1);
        let mut at = list + 4;
        for _ in 0..seed % 3 {
            let other = ptr(r) + 0x8;
            c.bus.write32(at, other);
            at = other + 200;
        }
        c.f[12] = if seed % 4 == 0 {
            r.edge_f32()
        } else {
            r.f32(0.05)
        };
    }

    #[test]
    fn seq_type07() {
        run(
            "seq_type07",
            ssx3_ee::s00::f_00104358,
            super::rider_anim_update_seq_type07,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 0x19, 0);
                seq_setup(c, &mut r, seed);
                let anim = ptr(&mut r);
                let rider = ptr(&mut r);
                c.g[A0][0] = anim as u64;
                c.bus.write32(anim + 0x60, rider);
                c.bus.write32(rider + 0x2a4, r.edge_f32());
                c.bus.write32(rider + 0x2b0, r.edge_f32());
            },
        );
    }

    #[test]
    fn seq_type02() {
        run(
            "seq_type02",
            ssx3_ee::s00::f_001049c0,
            super::rider_anim_update_seq_type02,
            SEEDS,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a, 0);
                seq_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn turn_around_if_backward() {
        run(
            "turn_around",
            ssx3_ee::s00::f_00114cc0,
            super::rider_turn_around_if_backward,
            600,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b, 0);
                let p = slew_rider(c, &mut r, seed);
                if seed % 3 != 0 {
                    c.bus
                        .write32(p + 0x214, if seed % 7 == 0 { 0x8000_0000 } else { 0 });
                }
                for i in 0..4 {
                    c.bus.write32(p + 0x110 + 4 * i, r.f32(1e5));
                    c.bus.write32(p + 0x120 + 4 * i, r.f32(1.0));
                    c.bus.write32(
                        p + 0x1c0 + 4 * i,
                        if seed % 11 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(1.0)
                        },
                    );
                }
                let owner = ptr(&mut r);
                c.bus.write32(p + 0x77c, owner);
            },
        );
    }

    /// A rider with random boost state (not human, so the sound calls stay
    /// short) and an audio system instance.
    fn boost_rider(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        game_world(c, r);
        let p = ptr(r);
        c.g[A0][0] = p as u64;
        let owner = ptr(r);
        c.bus.write32(p + 0x77c, owner);
        let audio = ptr(r);
        c.bus.write32(GAME_GP + 0x410, audio);
        let pick = |r: &mut Rng, k: u32| match r.next_u32() % k {
            0 => 0,
            1 => r.edge_f32(),
            _ => r.f32(1.0),
        };
        for off in [0x2e8, 0x2ec, 0x2f0, 0x2f8, 0x2fc] {
            let v = pick(r, 3);
            c.bus.write32(p + off, v);
        }
        if seed % 3 == 0 {
            c.bus.write32(p + 0x2e8, 0);
        }
        c.bus.write32(
            p + 0x2f4,
            [0, 5, 9, 10, 11, r.next_u32()][(r.next_u32() % 6) as usize],
        );
        c.bus.write32(p + 0x304, r.next_u32() % 4);
        p
    }

    #[test]
    fn handle_boost_input() {
        run(
            "handle_boost_input",
            ssx3_ee::s00::f_00114130,
            super::rider_handle_boost_input,
            2000,
            (0, 0),
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c, 0);
                boost_rider(c, &mut r, seed);
                c.g[A1][0] = if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.g[A2][0] = if seed % 5 < 2 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }

    /// The boost meter drain (`Rider_HandleBoostInput`, 0x114130; the load and
    /// subtract at 0x114220/0x114224) is 0.0015068 **per tick**: a full meter
    /// must last the same wall-clock time at any rate, so a second of ticks
    /// drains the same amount at 60, 120 and 240 Hz.
    #[test]
    fn handle_boost_input_drain_follows_the_rate() {
        use crate::tick;
        // The game's drain at gp-0x7bf4 (0.00150679936632514).
        const DRAIN_60: u32 = 0x3ac5_7fcc;
        let drained = |hz: u32, ticks: u32| -> f32 {
            let mut c = fresh();
            let mut r = machine(&mut c, 0, 0x1c, 0);
            let p = boost_rider(&mut c, &mut r, 0);
            // A full meter, no power-up boost, boost mode 1, uber level 0.
            c.bus.write32(p + 0x2f8, 1.0f32.to_bits());
            c.bus.write32(p + 0x2e8, 0);
            c.bus.write32(p + 0x2f4, 0);
            c.bus.write32(p + 0x304, 1);
            c.bus.write32(GAME_GP.wrapping_sub(0x7bf4), DRAIN_60);
            c.g[A0][0] = p as u64;
            c.g[A1][0] = 1; // held
            c.g[A2][0] = 0; // not pressed
            tick::set_for_test(&mut c, hz);
            for _ in 0..ticks {
                super::rider_handle_boost_input(&mut c);
            }
            1.0 - f32::from_bits(c.bus.read32(p + 0x2f8))
        };
        // Half a second of ticks: 30 ticks of the 60 Hz drain.
        let at_60 = drained(60, 30);
        assert!((at_60 - 30.0 * 0.0015068).abs() < 1e-5, "{at_60}");
        for hz in [120u32, 240] {
            let at = drained(hz, 30 * hz / 60);
            assert!(
                (at - at_60).abs() < 1e-5,
                "{hz} Hz drained {at} in half a second, 60 Hz {at_60}"
            );
        }
    }

    #[test]
    fn add_boost() {
        run(
            "add_boost",
            ssx3_ee::s00::f_0010e098,
            super::rider_add_boost,
            2000,
            (100, 10),
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d, 4);
                let p = boost_rider(c, &mut r, seed);
                c.bus.write32(p + 0x86c, r.next_u32() % 6);
                c.bus.write32(p + 0xb34, r.next_u32() % 4);
                c.bus
                    .write32(p + 0xb28, if seed % 7 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(p + 0xb2c, r.next_u32() & 1);
                c.bus.write32(
                    p + 0x2f4,
                    [0, 0, 1, 5, 9, 10, 11, r.next_u32()][(r.next_u32() % 8) as usize],
                );
                if seed % 3 == 0 {
                    c.bus.write32(p + 0x2f0, 0);
                }
                c.bus.write32(
                    p + 0x2f8,
                    [0x3f80_0000, 0x3f7f_0000, r.f32(1.0)][(r.next_u32() % 3) as usize],
                );
                c.f[12] = match seed % 6 {
                    0 => 0,
                    1 => r.edge_f32(),
                    2 => r.f32(0.3) | 0x8000_0000,
                    _ => r.f32(1.0) & 0x7fff_ffff,
                };
                c.g[A1][0] = [1, 4, 0x10, r.next_u32() as u64][(r.next_u32() % 4) as usize];
            },
        );
    }
}
