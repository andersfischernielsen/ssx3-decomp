//! Staging module oc539 (wave 231, list s2): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc539`.

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
///
/// Not listed (the differential test cannot fake their callees): the
/// snapshot readers `Replay_DeserializeSnapshot` (0x26d678, calls the
/// unslotted `cReplayFramePtr_loadBlock` 0x26eab8, which decompresses),
/// `Replay_RestoreSnapshot` (0x26db88), `WorldState_Restore` (0x26dbd0) and
/// `WorldState_Read` (0x26dbf0). The last one calls the unslotted
/// `cWScriptMan_readFromReplayFrame` (0x30bd20, a resumable dispatcher) and
/// `cReplay_restoreDeadBucket` (0x26e340); they are ported once those have
/// override slots.
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0026_c6e8, playback_man_init, "PlaybackMan_Init")];

/// The FindFile callee (`PlaybackMan_FindFile`, not overridden: it runs for real
/// in the tests, over the fixture's file table).
const CALLEE_PLAYBACK_FIND_FILE: Callee = Callee {
    addr: 0x0026_cfe8,
    f: ee::s07::f_0026cfe8,
};
/// `WorldStateStream_Reset` (called as a direct alias by the generated body).
const CALLEE_WORLD_STREAM_RESET: Callee = Callee {
    addr: 0x0026_d628,
    f: ee::s07::f_0026d628,
};
/// `ReplayStream_SetBuffer` (direct alias in the generated body).
const CALLEE_REPLAY_STREAM_SET_BUFFER: Callee = Callee {
    addr: 0x0026_e950,
    f: ee::s07::f_0026e950,
};

/// `void PlaybackMan_Init(PlaybackMan* self, const char* location)` (0x26c6e8).
/// Copies the location name, clears the save-frame counters, allocates the
/// save data (0x14000 bytes) and the current frame (0x1c22c bytes, version
/// 0xDBAC0101, buffer size 0x1c200), then builds the playback file table: one
/// entry per slot of `MaxPlaybackFiles` (id -1, no pack data), and one id per
/// mission of the mission interface.
pub fn playback_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let me = c.g[S1][0] as u32;

        // strcpy(self->Location, name)
        c.g[A0][0] = sx(me.wrapping_add(0x8));
        c.g[S0][0] = sx(me.wrapping_add(0x14));
        call_ptr(c, 0x0041_66f4, 0x0026_c710)?;

        // Save frame: none yet, AsyncFileNum = AsyncHandle = -1.
        c.g[V1][0] = u64::MAX;
        Addr(me.wrapping_add(0x10)).sw(c, 0);
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = 0x1_0000;
        let m1 = c.g[V1][0];
        Addr(me.wrapping_add(0x78)).sw(c, m1);
        c.g[A3][0] = 0;
        let m1 = c.g[V1][0];
        Addr(me.wrapping_add(0x74)).sw(c, m1);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1570));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.g[A0][0] | 0x4000;
        call_ptr(c, 0x0031_7e30, 0x0026_c73c)?;
        let save_data = c.g[V0][0];
        Addr(me.wrapping_add(0x4c)).sw(c, save_data);

        // WorldStateStream_Reset(save frame stream); ReplayStream_SetBuffer(stream, buffer, 0)
        c.g[A0][0] = c.g[S0][0];
        CALLEE_WORLD_STREAM_RESET.call(c, 0x0026_c748)?;
        c.g[A1][0] = Addr(me.wrapping_add(0x4c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        CALLEE_REPLAY_STREAM_SET_BUFFER.call(c, 0x0026_c758)?;

        // The current frame: cMemMan_alloc(0x1c22c), then MEM_fill to zero.
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = 0x1_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1580));
        c.g[A0][0] = c.g[A0][0] | 0xc22c;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_c774)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = 0x1_0000;
        let frame_ptr = c.g[S0][0];
        Addr(me.wrapping_add(0x4)).sw(c, frame_ptr);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[A2][0] | 0xc22c;
        call_ptr(c, 0x003e_6448, 0x0026_c790)?;

        // The playback file table: operator_new(MaxPlaybackFiles * 12).
        c.g[A1][0] = Addr(me.wrapping_add(0x54)).lw(c);
        c.g[A0][0] = 0xc;
        c.g[V1][0] = 0xffff_ffff_dbac_0000;
        c.g[V0][0] = 0x1_0000;
        c.mult(0, 0, 5, 4);
        c.g[V1][0] = c.g[V1][0] | 0x101;
        c.g[V0][0] = c.g[V0][0] | 0xc200;
        c.g[A1][0] = 0x48_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1590));
        let fv1 = c.g[V1][0];
        Addr(c.g[S0][0] as u32).sw(c, fv1);
        let fv0 = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x4)).sw(c, fv0);
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0026_c7cc)?;
        c.g[V1][0] = Addr(me.wrapping_add(0x54)).lw(c);
        c.g[A1][0] = 0;
        let no_files = (c.g[V1][0] as i64) <= 0;
        let files = c.g[V0][0];
        Addr(me.wrapping_add(0x58)).sw(c, files);
        if !no_files {
            c.cycles += 3;
            c.g[A2][0] = u64::MAX;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 14;
                c.g[V1][0] = Addr(me.wrapping_add(0x58)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let neg1 = c.g[A2][0];
                Addr(c.g[V1][0] as u32).sw(c, neg1);
                c.g[V0][0] = Addr(me.wrapping_add(0x58)).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).off(0x4).sw(c, 0);
                c.g[V1][0] = Addr(me.wrapping_add(0x58)).lw(c);
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr(c.g[V1][0] as u32).off(0x8).sw(c, 0);
                c.g[V0][0] = Addr(me.wrapping_add(0x54)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_c7e8);
                }
            }
        }

        // The mission interface: its mission count, then one id per mission.
        c.cycles += 10;
        c.g[S0][0] = 0;
        call_ptr(c, 0x0014_dc80, 0x0026_c828)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x0014_dd58, 0x0026_c834)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0015_4278, 0x0026_c840)?;
        let missions = c.g[V0][0];
        let no_missions = (missions as i64) <= 0;
        Addr(me.wrapping_add(0x50)).sw(c, missions);
        if !no_missions {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0015_4588, 0x0026_c85c)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = Addr(me.wrapping_add(0x58)).lw(c);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let id = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, id);
                c.g[V0][0] = Addr(me.wrapping_add(0x50)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_c850);
                }
            }
        }

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

/// `int PlaybackMan_StartFileLoad(PlaybackMan* self, int key)` (0x26c9f0).
/// Looks the key up in the file table. Unknown key: 0. Pack data already
/// present: 1. Otherwise starts the async load of the file's path into the
/// file's handle slot: 1 on success; on failure the handle is reset to -1
/// and the result is 0.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn playback_man_start_file_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S0][0] = c.g[A0][0];
        let me = c.g[S0][0] as u32;
        CALLEE_PLAYBACK_FIND_FILE.call(c, 0x0026_ca08)?;
        c.g[A1][0] = c.g[V0][0];
        'done: {
            if (c.g[A1][0] as i64) < 0 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = Addr(me.wrapping_add(0x58)).lw(c);
            c.mult(0, 2, 5, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 1;
                break 'done;
            }

            // PackData is zero: start the load. AsyncFileNum = idx, AsyncHandle = -1.
            c.cycles += 16;
            c.g[S1][0] = u64::MAX;
            let idx = c.g[A1][0];
            Addr(me.wrapping_add(0x74)).sw(c, idx);
            let minus1 = c.g[S1][0];
            Addr(me.wrapping_add(0x78)).sw(c, minus1);
            c.g[A2][0] = 0x4a_0000;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x324));
            c.g[A0][0] = c.g[SP][0];
            c.g[T1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3430));
            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x320));
            c.g[T0][0] = sx(me.wrapping_add(0x8));
            call_ptr(c, 0x0041_7828, 0x0026_ca58)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, 0x003d_f690, 0x0026_ca64)?;
            let handle = c.g[V0][0];
            Addr(me.wrapping_add(0x78)).sw(c, handle);
            if (handle as i64) < 0 {
                c.cycles += 1;
                let minus1 = c.g[S1][0];
                Addr(me.wrapping_add(0x78)).sw(c, minus1);
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for this module's callee fakes (test machines only): the fakes
    /// run their gate-off body unless the machine holds the magic.
    const OC539_GATE: u32 = DATA + 0x200000;
    const OC539_MAGIC: u32 = 0x0539_0001;
    /// The async load's result (the fake returns it, sign-extended).
    const OC539_ASYNC_RESULT: u32 = DATA + 0x200010;
    /// The mission count and the mission ids the interface fakes return.
    const OC539_MISSIONS: u32 = DATA + 0x200020;
    const OC539_IDS: u32 = DATA + 0x200040;
    /// Pointers the allocator fakes return (outside the table and the self object).
    const OC539_ALLOC_RET: u32 = DATA + 0x240000;
    const OC539_NEW_RET: u32 = DATA + 0x220000;
    const OC539_BE_RET: u32 = DATA + 0x250000;
    const OC539_IFACE_RET: u32 = DATA + 0x252000;

    /// A gated fake for an overridden callee: logs the call (`fakes::fake`),
    /// then sets the result in `$body`; when the gate is off it runs the body
    /// the slot held before.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $gen:path, $target:expr, |$c:ident| $body:block) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake($c: &mut Cpu) {
                if $c.bus.read32(OC539_GATE) != OC539_MAGIC {
                    return fakes::fall_back($c, &$prev, $gen);
                }
                let ra = $c.g[RA][0] as u32;
                fakes::fake($c, $target, ra);
                $body
            }
        };
    }

    gated_fake!(
        oc539_fake_strcpy,
        OC539_PREV_STRCPY,
        ee::s15::f_004166f4__generated,
        0x0041_66f4,
        |c| {}
    );
    gated_fake!(
        oc539_fake_stream_reset,
        OC539_PREV_STREAM_RESET,
        ee::s07::f_0026d628__generated,
        0x0026_d628,
        |c| {}
    );
    gated_fake!(
        oc539_fake_set_buffer,
        OC539_PREV_SET_BUFFER,
        ee::s07::f_0026e950__generated,
        0x0026_e950,
        |c| {}
    );
    gated_fake!(
        oc539_fake_alloc,
        OC539_PREV_ALLOC,
        ee::s10::f_00317d70__generated,
        0x0031_7d70,
        |c| {
            c.g[V0][0] = sx(OC539_ALLOC_RET);
        }
    );
    gated_fake!(
        oc539_fake_new,
        OC539_PREV_NEW,
        ee::s10::f_00317e30__generated,
        0x0031_7e30,
        |c| {
            c.g[V0][0] = sx(OC539_NEW_RET);
        }
    );
    gated_fake!(
        oc539_fake_mem_fill,
        OC539_PREV_MEM_FILL,
        ee::s14::f_003e6448__generated,
        0x003e_6448,
        |c| {}
    );
    gated_fake!(
        oc539_fake_get_be,
        OC539_PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        |c| {
            c.g[V0][0] = sx(OC539_BE_RET);
        }
    );
    gated_fake!(
        oc539_fake_get_iface,
        OC539_PREV_GET_IFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        |c| {
            c.g[V0][0] = sx(OC539_IFACE_RET);
        }
    );
    gated_fake!(
        oc539_fake_missions,
        OC539_PREV_MISSIONS,
        ee::s01::f_00154278__generated,
        0x0015_4278,
        |c| {
            c.g[V0][0] = sx(c.bus.read32(OC539_MISSIONS));
        }
    );
    gated_fake!(
        oc539_fake_sprintf,
        OC539_PREV_SPRINTF,
        ee::s15::f_00417828__generated,
        0x0041_7828,
        |c| {}
    );
    gated_fake!(
        oc539_fake_async,
        OC539_PREV_ASYNC,
        ee::s14::f_003df690__generated,
        0x003d_f690,
        |c| {
            c.g[V0][0] = sx(c.bus.read32(OC539_ASYNC_RESULT));
        }
    );

    /// The mission id fake reads the mission index from `a1`, which `fake`
    /// clobbers, so it is written out by hand.
    static OC539_PREV_MISSION_ID: AtomicUsize = AtomicUsize::new(0);
    fn oc539_fake_mission_id(c: &mut Cpu) {
        if c.bus.read32(OC539_GATE) != OC539_MAGIC {
            return fakes::fall_back(c, &OC539_PREV_MISSION_ID, ee::s01::f_00154588__generated);
        }
        let index = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0015_4588, ra);
        c.g[V0][0] = sx(c.bus.read32(OC539_IDS + 4 * index));
    }

    /// Hooks every fake of this module (each once per process).
    fn hook_all() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            oc539_fake_strcpy,
            &OC539_PREV_STRCPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D628),
            oc539_fake_stream_reset,
            &OC539_PREV_STREAM_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E950),
            oc539_fake_set_buffer,
            &OC539_PREV_SET_BUFFER,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc539_fake_alloc,
            &OC539_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc539_fake_new,
            &OC539_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            oc539_fake_mem_fill,
            &OC539_PREV_MEM_FILL,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc539_fake_get_be,
            &OC539_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc539_fake_get_iface,
            &OC539_PREV_GET_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00154278),
            oc539_fake_missions,
            &OC539_PREV_MISSIONS,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00154588),
            oc539_fake_mission_id,
            &OC539_PREV_MISSION_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc539_fake_sprintf,
            &OC539_PREV_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            oc539_fake_async,
            &OC539_PREV_ASYNC,
        );
    }

    /// Machine state with the fakes gated on and registers scrambled.
    fn gate_on(c: &mut Cpu, seed: u32, salt: u64) {
        fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC539_GATE, OC539_MAGIC);
    }

    #[test]
    fn playback_man_init_matches() {
        hook_all();
        check(ee::s07::f_0026c6e8__generated, playback_man_init, 200, &|c, seed| {
            gate_on(c, seed, 0x539_0001);
            let me = DATA + 0x1000;
            // MaxPlaybackFiles (0 covers the skipped table), and the missions.
            c.bus.write32(me + 0x54, seed % 4);
            c.bus.write32(OC539_MISSIONS, seed % 5);
            for i in 0..4 {
                c.bus.write32(OC539_IDS + 4 * i, 0x100 + i + seed % 7);
            }
            c.bus.write32(OC539_ASYNC_RESULT, 0);
            c.g[A0][0] = sx(me);
            c.g[A1][0] = sx(DATA + 0x1800);
        });
    }
}
