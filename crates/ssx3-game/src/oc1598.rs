//! Staging module oc1598 (wave wave280): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1598`.

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
    0x002a_34d0,
    ssx_audio_system_pa_say20_c4,
    "SSXAudioSystem_PASay20C4",
)];

/// `SSXAudioSystem_PASay20C4` (0x002a34d0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_pa_say20_c4(c: &mut Cpu) {
    if c.pc != 0x002a34d0 {
        return ee::s08::f_002a34d0__generated(c);
    }
    guest(c, |c| {

'b_2a36e8: {
c.cycles += 14;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[A1][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x002a3500);
c.g[S3][0] = c.g[A3][0];
Callee { addr: 0x0029f128, f: ee::s08::SSXAudioSystem_IsPASpeechOn }.call(c, 0x002a3500)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002a3510);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00288ae0, 0x002a3510)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
'b_2a3588: {
c.cycles += 4;
c.g[RA][0] = sx(0x002a3520);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002a4248, f: ee::s08::SSXAudioSystem_IsRaceType4or5 }.call(c, 0x002a3520)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2a3588;
} else {
c.cycles += 2;
let t = c.g[S2][0] == 0u64;
c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c0));
if t {
break 'b_2a3588;
} else {
c.cycles += 22;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c0));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
c.g[S1][0] = 0x4a0000;
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x36e8));
c.g[S0][0] = c.lw((c.g[V0][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x002a3558);
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
call_ptr(c, t, 0x002a3558)?;
c.ldl(6, (c.g[S1][0] as u32).wrapping_add(0x7));
c.ldr(6, (c.g[S1][0] as u32));
c.g[A0][0] = c.g[S0][0];
c.g[T0][0] = c.g[V0][0];
c.g[A1][0] = 0x3e8;
c.g[A3][0] = 0u64;
c.g[T1][0] = 0u64;
c.g[T2][0] = 0u64;
c.g[RA][0] = sx(0x002a3580);
c.g[T3][0] = 0u64;
Callee { addr: 0x002adca0, f: ee::s08::BankMonitor_AddTimer }.call(c, 0x002a3580)?;
let t = true;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
break 'b_2a3588;
}
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x002a3590);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002a42c8, f: ee::s08::SSXAudioSystem_IsRaceType6to11 }.call(c, 0x002a3590)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
'b_2a35b0: {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
if t {
break 'b_2a35b0;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002a35a8);
c.g[A1][0] = 0u64;
Callee { addr: 0x002b0e28, f: ee::s08::SpeechSystem_IsInstanceReady }.call(c, 0x002a35a8)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_2a36e8;
} else {
break 'b_2a35b0;
}
}
}
'b_2a35ec: {
c.cycles += 2;
let t = c.g[S2][0] != 0u64;
c.g[S1][0] = 0x1;
if t {
break 'b_2a35ec;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002a35c0);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002a4078, f: ee::s08::SSXAudioSystem_IsTrackGroup0Event }.call(c, 0x002a35c0)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002a35d0);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002a4040, f: ee::s08::SSXAudioSystem_IsEventMode4 }.call(c, 0x002a35d0)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
c.cycles += 5;
c.g[A1][0] = c.g[S3][0];
c.g[RA][0] = sx(0x002a35e4);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00285d98, 0x002a35e4)?;
c.g[S1][0] = 0u64;
c.g[S2][0] = c.g[V0][0];
break 'b_2a35ec;
}
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x002a35f4);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00123168, 0x002a35f4)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
c.cycles += 11;
c.mtc1(0, 12);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
c.g[A1][0] = 0u64;
c.g[A2][0] = 0x20c4;
c.g[A3][0] = 0u64;
c.g[T0][0] = 0xb;
c.g[T1][0] = 0u64;
c.g[RA][0] = sx(0x002a3620);
c.g[T2][0] = 0u64;
Callee { addr: 0x002b1458, f: ee::s08::SpeechSystem_SetEventContext }.call(c, 0x002a3620)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_2a36e8;
} else {
'b_2a36b8: {
'b_2a365c: {
c.cycles += 8;
c.g[RA][0] = sx(0x002a3630);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0029f198, f: ee::s08::SSXAudioSystem_GetLanguageMask }.call(c, 0x002a3630)?;
c.g[S4][0] = c.g[V0][0];
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x002a3640);
c.g[A1][0] = c.g[S2][0];
Callee { addr: 0x002a1d48, f: ee::s08::SSXAudioSystem_GetRiderVoiceMask }.call(c, 0x002a3640)?;
let t = c.g[S1][0] == 0u64;
c.g[S3][0] = c.g[V0][0];
if t {
break 'b_2a365c;
} else {
c.cycles += 5;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x002a3654);
c.g[A1][0] = c.g[S2][0];
Callee { addr: 0x002a1e68, f: ee::s08::SSXAudioSystem_GetFinishSpeechMask }.call(c, 0x002a3654)?;
let t = true;
c.g[S0][0] = c.g[V0][0];
if t {
break 'b_2a36b8;
} else {
break 'b_2a365c;
}
}
}
'b_2a36b4: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xec));
c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x480000;
if t {
break 'b_2a36b4;
} else {
'b_2a36ac: {
'b_2a36a4: {
'b_2a369c: {
'b_2a3694: {
'b_2a368c: {
c.cycles += 6;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30f0));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x002a3684 => {
c.cycles += 2;
let t = true;
c.g[S0][0] = 0x1;
if t {
break 'b_2a36b8;
} else {
break 'b_2a368c;
}
}
0x002a368c => {
break 'b_2a368c;
}
0x002a3694 => {
break 'b_2a3694;
}
0x002a369c => {
break 'b_2a369c;
}
0x002a36a4 => {
break 'b_2a36a4;
}
0x002a36ac => {
break 'b_2a36ac;
}
_ => { c.jump_out(t, 0x002a34d0, 0x002a3708); return Ok(()); }
}
}
c.cycles += 2;
let t = true;
c.g[S0][0] = 0x2;
if t {
break 'b_2a36b8;
} else {
break 'b_2a3694;
}
}
c.cycles += 2;
let t = true;
c.g[S0][0] = 0x4;
if t {
break 'b_2a36b8;
} else {
break 'b_2a369c;
}
}
c.cycles += 2;
let t = true;
c.g[S0][0] = 0x8;
if t {
break 'b_2a36b8;
} else {
break 'b_2a36a4;
}
}
c.cycles += 2;
let t = true;
c.g[S0][0] = 0x10;
if t {
break 'b_2a36b8;
} else {
break 'b_2a36ac;
}
}
c.cycles += 2;
let t = true;
c.g[S0][0] = 0x200;
if t {
break 'b_2a36b8;
} else {
break 'b_2a36b4;
}
}
}
c.cycles += 1;
c.g[S0][0] = 0u64;
break 'b_2a36b8;
}
c.cycles += 12;
c.g[A0][0] = 0x1;
c.g[A1][0] = 0u64;
c.g[RA][0] = sx(0x002a36c8);
c.g[A2][0] = 0x20c4;
call_ptr(c, 0x003d8008, 0x002a36c8)?;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
c.g[A0][0] = c.g[V0][0];
c.g[A2][0] = c.g[S4][0];
c.g[A3][0] = c.g[S3][0];
c.g[T0][0] = c.g[S0][0];
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002a36e4);
c.g[A1][0] = 0x3;
call_ptr(c, t, 0x002a36e4)?;
c.g[V0][0] = 0x1;
break 'b_2a36e8;
}
}
}
}
}
}
c.cycles += 8;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::fakes::{self, fall_back, hook, prev_cell};
    use crate::difftest::{elf_image, scramble, Rng, DATA};
    use ssx3_ee_rt::cpu::reg::*;
    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;
    use std::ptr::addr_of_mut;

    /// Gate word: the callee fakes answer only when a test wrote the magic here.
    const OC1598_GATE: u32 = DATA + 0x21_0000;
    const OC1598_MAGIC: u32 = 0x1598_5a7e;
    /// Forced `v0` of callee `n`: a `(MAGIC, value)` word pair at `MAIL + 8n`.
    const OC1598_MAIL: u32 = DATA + 0x21_0100;

    // Fixture objects (all below the fakes' call log at DATA+0x100000).
    const OC1598_THIS: u32 = DATA + 0x4000;
    const OC1598_ARG_OBJ: u32 = DATA + 0x2000;
    const OC1598_RIDER_OBJ: u32 = DATA + 0x3000;
    const OC1598_PLAYER: u32 = DATA + 0x6000;
    const OC1598_BANK: u32 = DATA + 0x5000;

    /// A gated fake for the callee `$gen` (mailbox index `$idx`): logs the call
    /// like `fakes::fake` (every register is compared) and forces `v0` if the
    /// fixture asked for it.
    macro_rules! oc1598_fake {
        ($name:ident, $idx:expr, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1598_GATE) != OC1598_MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                let forced = c.bus.read32(OC1598_MAIL + 8 * $idx) == OC1598_MAGIC;
                let v0 = c.bus.read32(OC1598_MAIL + 8 * $idx + 4);
                fakes::fake(c, $addr, ra);
                if c.pc == ra && forced {
                    c.g[V0][0] = sx(v0);
                }
            }
        };
    }

    oc1598_fake!(oc1598_is_pa_speech_on, 0, 0x0029_f128, ee::s08::f_0029f128__generated);
    oc1598_fake!(oc1598_f_288ae0, 1, 0x0028_8ae0, ee::s07::f_00288ae0__generated);
    oc1598_fake!(oc1598_is_race_type_4or5, 2, 0x002a_4248, ee::s08::f_002a4248__generated);
    oc1598_fake!(oc1598_add_timer, 3, 0x002a_dca0, ee::s08::f_002adca0__generated);
    oc1598_fake!(oc1598_is_race_type_6to11, 4, 0x002a_42c8, ee::s08::f_002a42c8__generated);
    oc1598_fake!(oc1598_is_instance_ready, 5, 0x002b_0e28, ee::s08::f_002b0e28__generated);
    oc1598_fake!(oc1598_is_track_group0, 6, 0x002a_4078, ee::s08::f_002a4078__generated);
    oc1598_fake!(oc1598_is_event_mode4, 7, 0x002a_4040, ee::s08::f_002a4040__generated);
    oc1598_fake!(oc1598_f_285d98, 8, 0x0028_5d98, ee::s07::f_00285d98__generated);
    oc1598_fake!(oc1598_f_123168, 9, 0x0012_3168, ee::s00::f_00123168__generated);
    oc1598_fake!(oc1598_set_event_context, 10, 0x002b_1458, ee::s08::f_002b1458__generated);
    oc1598_fake!(oc1598_language_mask, 11, 0x0029_f198, ee::s08::f_0029f198__generated);
    oc1598_fake!(oc1598_rider_voice_mask, 12, 0x002a_1d48, ee::s08::f_002a1d48__generated);
    oc1598_fake!(oc1598_finish_speech_mask, 13, 0x002a_1e68, ee::s08::f_002a1e68__generated);
    oc1598_fake!(oc1598_f_3d8008, 14, 0x003d_8008, ee::s14::f_003d8008__generated);

    /// Hooks every callee fake into its override slot (each once per process).
    fn oc1598_install() {
        hook(addr_of_mut!(ee::s08::OVERRIDE_0029F128), oc1598_is_pa_speech_on, prev_cell(oc1598_is_pa_speech_on));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00288AE0), oc1598_f_288ae0, prev_cell(oc1598_f_288ae0));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002A4248), oc1598_is_race_type_4or5, prev_cell(oc1598_is_race_type_4or5));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002ADCA0), oc1598_add_timer, prev_cell(oc1598_add_timer));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002A42C8), oc1598_is_race_type_6to11, prev_cell(oc1598_is_race_type_6to11));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002B0E28), oc1598_is_instance_ready, prev_cell(oc1598_is_instance_ready));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002A4078), oc1598_is_track_group0, prev_cell(oc1598_is_track_group0));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002A4040), oc1598_is_event_mode4, prev_cell(oc1598_is_event_mode4));
        hook(addr_of_mut!(ee::s07::OVERRIDE_00285D98), oc1598_f_285d98, prev_cell(oc1598_f_285d98));
        hook(addr_of_mut!(ee::s00::OVERRIDE_00123168), oc1598_f_123168, prev_cell(oc1598_f_123168));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002B1458), oc1598_set_event_context, prev_cell(oc1598_set_event_context));
        hook(addr_of_mut!(ee::s08::OVERRIDE_0029F198), oc1598_language_mask, prev_cell(oc1598_language_mask));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002A1D48), oc1598_rider_voice_mask, prev_cell(oc1598_rider_voice_mask));
        hook(addr_of_mut!(ee::s08::OVERRIDE_002A1E68), oc1598_finish_speech_mask, prev_cell(oc1598_finish_speech_mask));
        hook(addr_of_mut!(ee::s14::OVERRIDE_003D8008), oc1598_f_3d8008, prev_cell(oc1598_f_3d8008));
    }

    fn oc1598_force(c: &mut Cpu, idx: u32, v0: u32) {
        c.bus.write32(OC1598_MAIL + 8 * idx, OC1598_MAGIC);
        c.bus.write32(OC1598_MAIL + 8 * idx + 4, v0);
    }

    /// `SSXAudioSystem::PASay20C4(this, rider, a2, a3)`: says the event-0x20c4
    /// speech line, after a chain of "is speech possible" checks (PA speech
    /// on, race type, event mode, speech instance ready ...). The seed picks
    /// the answer of each check (mostly the answer that lets the chain go
    /// on), whether `rider` is given, and the rider type (jump table at
    /// 0x4830f0, six entries and an out-of-range case).
    #[test]
    fn ssx_audio_system_pa_say20_c4_matches() {
        oc1598_install();
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = fakes::machine(c, seed, 0x002a_34d0, 20);
            elf_image(c);
            c.bus.write32(OC1598_GATE, OC1598_MAGIC);
            // Mostly the continuing answer; one in `k` the exiting one.
            let pick = |r: &mut Rng, cont: u32, other: u32, k: u32| {
                if r.next_u32() % k == 0 { other } else { cont }
            };
            let v = pick(&mut r, 1, 0, 8);
            oc1598_force(c, 0, v);
            let v = pick(&mut r, 0, 1, 8);
            oc1598_force(c, 1, v);
            let v = pick(&mut r, 0, 1, 2);
            oc1598_force(c, 2, v);
            let v = pick(&mut r, 0, 1, 4);
            oc1598_force(c, 4, v);
            let v = pick(&mut r, 1, 0, 8);
            oc1598_force(c, 5, v);
            let v = pick(&mut r, 1, 0, 8);
            oc1598_force(c, 6, v);
            let v = pick(&mut r, 0, 1, 8);
            oc1598_force(c, 7, v);
            oc1598_force(c, 8, OC1598_RIDER_OBJ);
            let v = pick(&mut r, 0, 1, 8);
            oc1598_force(c, 9, v);
            let v = pick(&mut r, 1, 0, 8);
            oc1598_force(c, 10, v);
            let rider_type = r.next_u32() % 8;
            c.bus.write32(OC1598_RIDER_OBJ + 0xec, rider_type);
            c.bus.write32(OC1598_ARG_OBJ + 0xec, rider_type);
            // The rider-given path (race type 4 or 5): player object at
            // +0x6c0 with an offset (+0x38), a function (+0x3c) and the
            // bank/monitor chain at this+0x118.
            c.bus.write32(OC1598_ARG_OBJ + 0x6c0, OC1598_PLAYER);
            c.bus.write16(OC1598_PLAYER + 0x38, (r.next_u32() % 0x100) as u16);
            c.bus.write32(OC1598_PLAYER + 0x3c, fakes::FAKE + 0x30);
            c.bus.write32(OC1598_THIS + 0x118, OC1598_BANK);
            c.bus.write32(OC1598_BANK, r.next_u32());
            // The final speech call goes through a function pointer at gp+0x173c.
            c.bus.write32(crate::difftest::GAME_GP + 0x173c, fakes::FAKE + 0x40);
            c.g[A0][0] = OC1598_THIS as u64;
            c.g[A1][0] = if r.next_u32() % 2 == 0 { 0 } else { OC1598_ARG_OBJ as u64 };
            // Entry pc: without it the generated body dispatches to its `__entries`
            // twin with a stale pc and returns at once (no call is ever made).
            c.pc = 0x002a_34d0;
        };
        fakes::check_reaching(
            ee::s08::f_002a34d0__generated,
            ssx_audio_system_pa_say20_c4,
            400,
            &[
                0x002a_3500, 0x002a_3510, 0x002a_3520, 0x002a_3558, 0x002a_3580, 0x002a_3590,
                0x002a_35a8, 0x002a_35c0, 0x002a_35d0, 0x002a_35e4, 0x002a_35f4, 0x002a_3620,
                0x002a_3630, 0x002a_3640, 0x002a_3654, 0x002a_36c8, 0x002a_36e4,
            ],
            &setup,
        );
    }
}
