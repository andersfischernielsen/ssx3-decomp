//! `src/camera/trigger/cameratriggerlist.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use crate::abi::r32;
use crate::call::Flow;
pub(crate) use crate::game::fe::feasyncfile::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_c098, camera_trigger_man_destroy, "CameraTriggerMan_Destroy"),
    (0x0016_c120, camera_trigger_man_add_trigger, "CameraTriggerMan_AddTrigger"),
    (0x0016_c170, camera_trigger_man_load_file, "CameraTriggerMan_LoadFile"),
    (0x0016_c3d0, camera_trigger_man_read_header, "CameraTriggerMan_ReadHeader"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
}


/// `void CameraTriggerMan_AddTrigger(CameraTriggerMan *man, int value, int index)`
/// (0x16c120): append `value` at `index` (or at the running count when `index`
/// is negative and there is room).
pub fn camera_trigger_man_add_trigger(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let value = abi::arg64(c, 1);
    let index = c.g[A2][0];
    c.cycles += 2;
    if (index as i64) < 0 {
        c.cycles += 4;
        let count = man.off(0x4).lw(c);
        c.g[V1][0] = count;
        let room = (count as i64) < 120;
        c.g[V0][0] = room as u64;
        if room {
            let base = man.lw(c);
            c.g[V0][0] = base;
            c.cycles += 7;
            let slot = (count as u32).wrapping_shl(2).wrapping_add(base as u32);
            c.g[V1][0] = sx(slot);
            Addr(slot).sw(c, value);
            c.g[V0][0] = man.off(0x4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            man.off(0x4).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = man.lw(c);
    c.cycles += 3;
    c.g[V0][0] = sx((index as u32) << 2);
    let slot = (c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32);
    c.g[V0][0] = sx(slot);
    Addr(slot).sw(c, value);
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `EA_MemFree` (0x317e98).
pub(crate) const MEM_FREE: u32 = 0x0031_7e98;


// ---------------------------------------------------------------------------
// CameraTriggerMan_Destroy.

/// `void CameraTriggerMan_Destroy(this)` (0x16c098): free the trigger array,
/// then free the manager itself and clear the caller's pointer.
pub fn camera_trigger_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::CAMERA_TRIGGER_FREE_TRIGGERS.call(c, 0x0016_c0ac)?;
        c.g[A0][0] = Addr(this as u32).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, MEM_FREE, 0x0016_c0c0)?;
            c.sw(this as u32, 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cCameraTriggerMan_ReadHeader(cCameraTriggerMan* self, void* buffer)`
/// (0x16c3d0): read three words of a trigger list header.
pub fn camera_trigger_man_read_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // the serialised buffer
        c.g[S1][0] = c.g[A0][0]; // the header object
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x10));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c3f4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(4));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c400)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(8));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c40c)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Camera trigger file loading.

/// `int CameraTriggerMan_LoadFile(CameraTriggerMan* m, const char* name)`
/// (0x16c170): free the old trigger list, load and parse the named file,
/// returning 0 on success, 1 when the file is missing and 2 when the buffer
/// could not be allocated.
pub fn camera_trigger_man_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        'outer: {
            frame.save(c, S2, 0x20);
            frame.save(c, S3, 0x10);
            c.g[S2][0] = c.g[A1][0];
            let this = r(c, A0);
            c.g[S3][0] = sx(this);
            frame.save(c, S0, 0x40);
            frame.save(c, S1, 0x30);
            frame.save_ra(c, 0);
            c.g[RA][0] = sx(0x0016_c198);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7f38, 0x0016_c198)?;
            let missing = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if missing {
                break 'outer;
            }
            'mid: {
                c.cycles += 18;
                c.g[RA][0] = sx(0x0016_c1a8);
                c.g[A0][0] = c.g[S3][0];
                callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS.call(c, 0x0016_c1a8)?;
                c.g[RA][0] = sx(0x0016_c1b0);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0031_7f98, 0x0016_c1b0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx(0x0045_bf30);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0x100;
                c.g[RA][0] = sx(0x0016_c1cc);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0016_c1cc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0016_c1e0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003e_2470, 0x0016_c1e0)?;
                let ok = c.g[S1][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if ok {
                    break 'mid;
                }
                c.cycles += 2;
                c.g[V0][0] = 2;
                break 'outer;
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x0016_c1f8);
            c.g[A1][0] = c.g[S1][0];
            callee::CAMERA_TRIGGER_MAN_PARSE.call(c, 0x0016_c1f8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0016_c204);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e98, 0x0016_c204)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use super::ee;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;


    #[test]
    fn camera_trigger_man_add_trigger_matches() {
        check(ssx3_ee::s02::f_0016c120, camera_trigger_man_add_trigger, 300, &|c, seed| {
            let mut r = Rng(0x16c1 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 0xffff_ffff_ffff_ffff,
                1 => (r.next_u32() % 30) as u64,
                _ => (r.next_u32() % 120) as u64,
            };
            c.bus.write32(DATA, DATA + 0x400);
            c.bus.write32(DATA + 4, if seed % 5 == 0 { 200 } else { r.next_u32() % 100 });
        });
    }


    /// The cache pointer is small, so `MEM_IsPoolPointer` sees it inside the
    /// (zeroed) 8-byte pool and `MEM_PoolFree` is two stores — no heap walk.
    pub(crate) fn small_pointer(seed: u32) -> u32 {
        0x100 + (seed % 0x700)
    }


    // -----------------------------------------------------------------------
    // CameraTriggerMan_Destroy.

    /// The freed pointer is small, so `MEM_IsPoolPointer` sees it inside the
    /// (zeroed) 8-byte pool — `MEM_PoolFree` is two stores, never a heap walk.
    #[test]
    fn camera_trigger_man_destroy_matches() {
        check(ssx3_ee::s02::f_0016c098__generated, camera_trigger_man_destroy, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x16c0_98, 0);
            let this = DATA;
            c.g[A0][0] = this as u64;
            // An empty trigger list (count at `+4` is 0) and, for most seeds,
            // a small "pool" pointer to free.
            c.bus.write32(this + 0x4, 0);
            c.bus.write32(this + 0x0, if seed % 3 == 0 { 0 } else { small_pointer(seed) });
            c.bus.write32(this + 0x10, r.next_u32());
        });
    }


    #[test]
    fn camera_trigger_man_read_header_matches() {
        check(
            ee::s02::f_0016c3d0__generated,
            camera_trigger_man_read_header,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16c3_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let reader = DATA + 0x800;
                let bytes = DATA + 0x900;
                clear(c, this, 0x40);
                clear(c, reader, 0x20);
                for i in 0..0x40 {
                    c.bus.write32(bytes + 4 * i, r.next_u32());
                }
                c.bus.write32(reader, bytes);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(reader);
            },
        );
    }

    pub(crate) fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }


    #[test]
    fn camera_trigger_man_load_file_matches() {
        check(ee::s02::f_0016c170__generated, camera_trigger_man_load_file, 30, &|c, seed| {
            alloc_base(c, seed, 0x16c1, 4);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for (i, b) in b"triggers.bin\0".iter().enumerate() {
                wb(c, DATA + 0x800 + i as u32, *b);
            }
        });
    }
}
