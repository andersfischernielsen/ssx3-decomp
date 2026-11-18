//! `src/world/world.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::wscript::wscriptman::gp;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_6688, world_ctor, "World_ctor"),
    (0x003a_6740, world_dtor, "World_dtor"),
    (0x003a_67e0, world_on_game_enter, "World_OnGameEnter"),
    (
        0x003a_6b78,
        world_get_object_script_func,
        "World_GetObjectScriptFunc",
    ),
    (
        0x003a_6cc8,
        world_get_object_section_data,
        "World_GetObjectSectionData",
    ),
    (
        0x003a_6ce8,
        world_get_sphere_tree_at,
        "World_GetSphereTreeAt",
    ),
    (0x003a_6d00, world_get_table_entry, "World_GetTableEntry"),
    (
        0x003a_6d18,
        world_get_section_script_func,
        "World_GetSectionScriptFunc",
    ),
    (
        0x003a_6c68,
        world_get_section_mission_count,
        "World_GetSectionMissionCount",
    ),
    (
        0x003a_6c98,
        world_get_section_missions,
        "World_GetSectionMissions",
    ),
    (0x003a_6800, world_reset_sections, "World_ResetSections"),
    (0x003a_6ae0, world_load_2, "World_Load"),
    (
        0x003a_6ba8,
        world_get_section_shutdown_func_count,
        "World_GetSectionShutdownFuncCount",
    ),
    (
        0x003a_6bd8,
        world_get_section_shutdown_funcs,
        "World_GetSectionShutdownFuncs",
    ),
    (
        0x003a_6c08,
        world_get_section_process_count,
        "World_GetSectionProcessCount",
    ),
    (
        0x003a_6c38,
        world_get_section_processes,
        "World_GetSectionProcesses",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;

    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;

    pub(crate) use crate::game::object::debouncenode::callee::GET_OBJECT_SLOT;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;

    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
}

/// `int World_GetObjectScriptFunc(WScriptCache* cache, WorldObj* obj,
/// int flag)` (0x3a6b78): the object's script slot — the script table is at
/// `*(*(cache + 4) + 4)`, records are 12 bytes (`(unsigned char)
/// *(obj + 0x78)` indexes them), and the returned value is
/// `tWScriptCache_GetObjectSlot(table + id * 12, obj, flag)`.
pub fn world_get_object_script_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0xc;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x78).lbu(c);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        // v0 = id * 12 (record stride), in the delay slot.
        c.mult(0, 2, 2, 7);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::GET_OBJECT_SLOT.call(c, 0x003a_6b9c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SphereTree* World_GetSphereTreeAt(World* w, int index)` (0x3a6ce8): the
/// index-th sphere tree (0x140-byte records) of the world's tree array.
pub fn world_get_sphere_tree_at(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = 0x140;
    c.g[V0][0] = c.lw(selfp);
    c.mult(0, A1, A1, V1); // a1 = index * 0x140
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `void World_OnGameEnter(this)` (0x003a67e0): reset the world's section
/// state.
pub fn world_on_game_enter(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |_| {},
        callee::WORLD_RESET_SECTIONS,
        0x003a_67f0,
    )
}

/// `void* World_GetTableEntry(void** table, int index)` (0x3a6d00): entry
/// `index` of the pointer table.
pub fn world_get_table_entry(c: &mut Cpu) {
    c.cycles += 5;
    let table = abi::arg(c, 0);
    c.g[V0][0] = Addr(table).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let entry = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = entry;
    abi::ret(c);
}

/// `void* World_GetObjectSectionData(World* w, WorldObject* obj)`
/// (0x3a6cc8): the section data of the object's current model: `obj->+0x88`
/// → `[+0xc]` → `[+8]`.
pub fn world_get_object_section_data(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 1));
    c.g[V0][0] = obj.off(0x88).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
    let v = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
    c.g[V0][0] = v;
    abi::ret(c);
}

/// `void World_Load(World* self, char* name, u32 flags)` (0x3a6ae0): reset and
/// load the world cache, allocate the script cache from the section count and
/// release the name string.
pub fn world_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table_callee::BXSTRING_CTOR, 0x003a_6b08)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        callee::WORLD_CACHE_RESET.call(c, 0x003a_6b10)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        callee::WORLD_CACHE_LOAD.call(c, 0x003a_6b20)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        callee::STREAMER_GET_SECTION_COUNT.call(c, 0x003a_6b2c)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::WSCRIPT_CACHE_ALLOC.call(c, 0x003a_6b38)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        callee::STREAMER_GET_SECTION_COUNT.call(c, 0x003a_6b44)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table_callee::SECTION_TABLE_CTOR, 0x003a_6b50)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, table_callee::BXSTRING_DTOR, 0x003a_6b5c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World.

/// `void World_dtor(World* self, int flags)` (0x3a6740): stop the global game
/// module, destroy the world cache, script cache and painter, free the sphere
/// tree cache and delete the world when `flags & 1`.
pub fn world_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = lw_2(c, gp(c, 0x2a90));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = lw_2(c, p(c.g[A2][0] as u32, 0x10d8));
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x358)).lh(c);
        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x35c));
        c.g[V1][0] = sx(c.g[V1][0] as u32);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x003a_6770)?;
        c.g[A0][0] = lw_2(c, c.g[S0][0] as u32);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_7e38, 0x003a_6784)?; // cWorldCache_dtor
        }
        c.g[A0][0] = lw_2(c, p(c.g[S0][0] as u32, 4));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_c8f0, 0x003a_6798)?; // cWScriptCache_dtor
        }
        c.g[A0][0] = lw_2(c, p(c.g[S0][0] as u32, 8));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002b_b140, 0x003a_67ac)?; // cWorldPainterMan_dtor
        }
        c.cycles += 5;
        call_ptr(c, 0x0032_de20, 0x003a_67b4)?; // World_FreeSphereTreeCache
        c.g[V0][0] = c.g[S1][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        sw(c, gp(c, 0x16c8), 0);
        if !no_delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x003a_67c8)?; // __builtin_delete
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// World.

/// `cWorld* cWorld::cWorld()` (0x3a6688): construct the world singleton — the
/// object cache, the script cache, the trigger manager hook object and the
/// path network — and publish it at `gp_2+0x16c8`.
pub fn world_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = sx(0x0049_4d58);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x1c00;
        c.g[RA][0] = sx(0x003a_66b4);
        Addr(gp_2(c, 0x16c8)).sw(c, c.g[S0][0]);
        call_ptr(c, callee::MALLOC, 0x003a_66b4)?;

        c.g[RA][0] = sx(0x003a_66bc);
        c.g[A0][0] = c.g[V0][0];
        callee::WORLD_CACHE_CTOR.call(c, 0x003a_66bc)?;

        c.g[A1][0] = sx(0x0049_4d68);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(r(c, S0)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x003a_66d8);
        c.g[A0][0] = 0x10;
        call_ptr(c, callee::MALLOC, 0x003a_66d8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003a_66e4);
        c.g[A0][0] = c.g[V0][0];
        callee::WSCRIPT_CACHE_CTOR.call(c, 0x003a_66e4)?;

        c.g[A1][0] = sx(0x0049_4d78);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(p(r(c, S0), 4)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x003a_6700);
        c.g[A0][0] = 8;
        call_ptr(c, callee::MALLOC, 0x003a_6700)?;

        c.g[RA][0] = sx(0x003a_6708);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002b_b130, 0x003a_6708)?;

        Addr(p(r(c, S0), 0x10)).sw(c, 0);
        c.g[A0][0] = 0xa;
        Addr(p(r(c, S0), 0x14)).sw(c, 0);
        c.g[A1][0] = 4;
        Addr(p(r(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 2;
        c.g[RA][0] = sx(0x003a_6728);
        c.g[A3][0] = 5;
        call_ptr(c, 0x0032_dc10, 0x003a_6728)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World_GetSectionScriptFunc (0x3a6d18)
// ---------------------------------------------------------------------------

/// World: the section script-cache table (`{?, caches}`), 12-byte caches.
pub(crate) const WSSF_SCRIPT_TABLE: u32 = 0x4;

pub(crate) const WSSF_TABLE_CACHES: u32 = 0x4;

pub(crate) const WSSF_CACHE_SIZE: u32 = 0xc;

/// `ScriptFunc* World_GetSectionScriptFunc(World* self, int* handle)`
/// (0x3a6d18): the handle's low byte selects a section's script cache, the
/// rest the entry; returns `tWScriptCache_GetTableEntry(cache, handle >> 8)`.
pub fn world_get_section_script_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = WSSF_CACHE_SIZE as u64;
        frame.save_ra(c, 0);
        let handle = Addr(r32(c, A1)).lw(c) as u32;
        let table = Addr(r32(c, A0)).off(WSSF_SCRIPT_TABLE).lw(c);
        c.g[A2][0] = table;
        let offset = abi::mult(c, handle & 0xff, WSSF_CACHE_SIZE);
        let caches = Addr(table as u32).off(WSSF_TABLE_CACHES).lw(c);
        c.g[V0][0] = caches;
        c.g[A1][0] = sx(handle >> 8);
        c.g[A0][0] = sx((caches as u32).wrapping_add(offset));
        Callee {
            addr: 0x003a_dc50,
            f: ee::s13::f_003adc50,
        }
        .call(c, 0x003a_6d44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `tWScriptCache_GetMissions` (0x3add70).
pub(crate) const OC420_GET_MISSIONS: Callee = Callee {
    addr: 0x003a_dd70,
    f: ee::s13::f_003add70,
};

/// `tWScriptCache_GetMissionCount` (0x3add40).
pub(crate) const OC420_GET_MISSION_COUNT: Callee = Callee {
    addr: 0x003a_dd40,
    f: ee::s13::f_003add40,
};

/// Shared body of the two section-mission accessors: the world's script
/// cache table (`*(*(world + 4) + 4)`) holds 12-byte entries; call `callee`
/// on entry `i` (`a1 = i * 12`, `v0 = 12` stay behind as `mult` leaves them).
pub(crate) fn oc420_section_entry(c: &mut Cpu, callee: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        callee.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* World_GetSectionMissions(World* w, int section)` (0x3a6c98): the
/// mission list of one script-cache entry.
pub fn world_get_section_missions(c: &mut Cpu) {
    oc420_section_entry(c, OC420_GET_MISSIONS, 0x003a_6cb8);
}

/// `int World_GetSectionMissionCount(World* w, int section)` (0x3a6c68):
/// the mission count of one script-cache entry.
pub fn world_get_section_mission_count(c: &mut Cpu) {
    oc420_section_entry(c, OC420_GET_MISSION_COUNT, 0x003a_6c88);
}

/// `void World_ResetSections(World* self)` (0x3a6800): for every resident
/// streamed section, walk its sub-entries and mark the matching table entries
/// (`word8 = word8 & 0xffff0000 | (word8 >> 16) | 2`). Dispatcher-shaped like
/// the original: its loop heads 0x3a6828 and 0x3a6878 are resume entries
/// after a delivered event.
#[allow(unreachable_code)]
pub fn world_reset_sections(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003a6800 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = 0xffffffffffff0000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[S0][0] | 0xffff;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S2][0] = c.g[A0][0];
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003a68f8;
                        continue;
                    }
                    pc = 0x003a6828;
                    continue;
                }
                0x003a6828 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x003a9858,
                        f: ee::s13::cWorldCache_StreamerIsSectionResident,
                    }
                    .call(c, 0x003a6834)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003a68f4;
                        continue;
                    }
                    pc = 0x003a683c;
                    continue;
                }
                0x003a683c => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = 0x58;
                    c.g[V0][0] = 0xffffffffffffff00;
                    c.mult(0, 3, 17, 3);
                    c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0] & 0xff;
                    c.g[A3][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x26));
                    let t = (c.g[T1][0] as i64) <= 0;
                    c.g[S0][0] = c.g[S0][0] | c.g[A2][0];
                    if t {
                        pc = 0x003a68f4;
                        continue;
                    }
                    pc = 0x003a6870;
                    continue;
                }
                0x003a6870 => {
                    c.cycles += 2;
                    c.g[T3][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    pc = 0x003a6878;
                    continue;
                }
                0x003a6878 => {
                    c.cycles += 13;
                    c.g[V1][0] = c.g[S0][0] & 0xff;
                    c.g[A1][0] = sx((c.g[A3][0] as u32) << 8);
                    c.g[A0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = c.g[V1][0] | c.g[A1][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32) >> 8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[T0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x003a68cc;
                        continue;
                    }
                    pc = 0x003a68ac;
                    continue;
                }
                0x003a68ac => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003a68c8;
                        continue;
                    }
                    pc = 0x003a68c4;
                    continue;
                }
                0x003a68c4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    pc = 0x003a68c8;
                    continue;
                }
                0x003a68c8 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[A0][0];
                    pc = 0x003a68cc;
                    continue;
                }
                0x003a68cc => {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;

                    if t {
                        pc = 0x003a68ec;
                        continue;
                    }
                    pc = 0x003a68d4;
                    continue;
                }
                0x003a68d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                    c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003a68ec;
                    continue;
                }
                0x003a68ec => {
                    c.cycles += 2;

                    if c.g[T0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                        if event_due(c) && back_edge_unwinds(c, 0x003a6878) {
                            return Ok(());
                        }
                        pc = 0x003a6878;
                        continue;
                    }
                    pc = 0x003a68f4;
                    continue;
                }
                0x003a68f4 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.g[T2][0];
                    pc = 0x003a68f8;
                    continue;
                }
                0x003a68f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x003a9800,
                        f: ee::s13::cWorldCache_StreamerGetSectionCount,
                    }
                    .call(c, 0x003a6904)?;
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                        if event_due(c) && back_edge_unwinds(c, 0x003a6828) {
                            return Ok(());
                        }
                        pc = 0x003a6828;
                        continue;
                    }
                    pc = 0x003a6910;
                    continue;
                }
                0x003a6910 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003a6800, 0x003a6928);
                    return Ok(());
                }
            }
        }
        Ok(())
    })
}

/// `int World_GetSectionShutdownFuncs(World* self, int i)` (0x3a6bd8): the global scripts of
/// the world script cache's entry `i` (entries 0xc bytes each, at `+4` of the cache at
/// `self + 4`).
pub fn world_get_section_shutdown_funcs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0xc;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mult(0, 5, 5, 2);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        Callee {
            addr: 0x003adcb0,
            f: ee::s13::tWScriptCache_GetGlobals,
        }
        .call(c, 0x003a6bf8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `Callee` for a named generated function (direct call through its alias).
macro_rules! oc435_callee {
    ($addr:expr, $f:path) => {
        Callee { addr: $addr, f: $f }
    };
}
pub(crate) use oc435_callee;

/// Shared body of the three `World_GetSection*` wrappers (frame 0x10 and the
/// `mult` that scales the section index by the 12-byte entry size):
/// `callee(world->cache(+4)->sections(+4) + index * 12)`.
pub(crate) fn world_section_call(c: &mut Cpu, callee: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 5, 5, 2);
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        callee.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `World_GetSectionProcesses(world, section)` (0x3a6c38):
/// `tWScriptCache_GetProcesses(section entry)`.
pub fn world_get_section_processes(c: &mut Cpu) {
    world_section_call(
        c,
        oc435_callee!(0x003a_dd10, ee::s13::tWScriptCache_GetProcesses),
        0x003a_6c58,
    )
}

/// `World_GetSectionProcessCount(world, section)` (0x3a6c08):
/// `tWScriptCache_GetProcessCount(section entry)`.
pub fn world_get_section_process_count(c: &mut Cpu) {
    world_section_call(
        c,
        oc435_callee!(0x003a_dce0, ee::s13::tWScriptCache_GetProcessCount),
        0x003a_6c28,
    )
}

/// `World_GetSectionShutdownFuncCount(world, section)` (0x3a6ba8):
/// `tWScriptCache_GetGlobalCount(section entry)`.
pub fn world_get_section_shutdown_func_count(c: &mut Cpu) {
    world_section_call(
        c,
        oc435_callee!(0x003a_dc80, ee::s13::tWScriptCache_GetGlobalCount),
        0x003a_6bc8,
    )
}

/// `void World_Load(World *w, a1, a2)` (0x3a6ae0): resets and loads the world
/// cache (`w->+0`), sizes the script caches by the streamer's section count
/// (`cWScriptCache_Alloc(w->+4, n)`, 0x2bb1d0 on `w->+8`), bracketed by a
/// 0x3182d8 / 0x318260 pair on a stack record. Frame 0x50 (s0..s2).
pub fn world_load_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0031_82d8, 0x003a_6b08)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        oc435_callee!(0x003a_7f90, ee::s13::cWorldCache_Reset).call(c, 0x003a_6b10)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        oc435_callee!(0x003a_7fc0, ee::s13::cWorldCache_Load).call(c, 0x003a_6b20)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        oc435_callee!(0x003a_9800, ee::s13::cWorldCache_StreamerGetSectionCount)
            .call(c, 0x003a_6b2c)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        oc435_callee!(0x003a_c800, ee::s13::cWScriptCache_Alloc).call(c, 0x003a_6b38)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        oc435_callee!(0x003a_9800, ee::s13::cWorldCache_StreamerGetSectionCount)
            .call(c, 0x003a_6b44)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002b_b1d0, 0x003a_6b50)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_6b5c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::rng as rng_2;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;

    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::w;

    use ssx3_ee as ee;

    /// `wscript` fixture addresses.
    pub(crate) const WORLD: u32 = DATA + 0x200;

    pub(crate) const TABLE: u32 = DATA + 0x300;

    pub(crate) const SLOT1: u32 = DATA + 0x400;

    pub(crate) const SLOT2: u32 = DATA + 0x440;

    pub(crate) const BASE: u32 = DATA + 0x600;

    /// `*(man + 0x28c) -> cache -> +4 -> world -> +4 -> table`, with the
    /// object's 12-byte table record's type halfword.
    pub(crate) fn put_script_cache(c: &mut Cpu, man: u32, cache: u32, obj: u32, id: u8, kind: u16) {
        c.bus.write32(man + 0x28c, cache);
        c.bus.write32(cache + 4, WORLD);
        c.bus.write32(WORLD + 4, TABLE);
        c.bus.write8(obj + 0x78, id);
        c.bus.write16(TABLE + 12 * id as u32, kind);
    }

    /// `tWScriptCache_GetObjectSlot` fixture: `obj + 0x88 -> +8` indexes
    /// `BASE + 4 * flag`, through `slot + 8 -> +0x1c -> BASE`.
    pub(crate) fn put_object_slots(c: &mut Cpu, obj: u32, slot: u32, word1: u32, word3: u32) {
        c.bus.write32(obj + 0x88, SLOT1);
        c.bus.write32(SLOT1 + 8, 0);
        c.bus.write32(slot + 8, SLOT2);
        c.bus.write32(SLOT2 + 0x1c, BASE);
        c.bus.write32(BASE + 4, word1);
        c.bus.write32(BASE + 12, word3);
    }

    #[test]
    fn world_get_object_script_func_matches() {
        check(
            ee::s13::f_003a6b78__generated,
            world_get_object_script_func,
            SEEDS,
            &|c, seed| {
                let _r = base(c, seed, 0x3a6b);
                let (cache, obj) = (DATA, DATA + 0x140);
                let id = (seed % 3) as u8;
                put_script_cache(c, DATA + 0x700, cache, obj, id, 3);
                let slot = TABLE + 12 * id as u32;
                match seed % 4 {
                    0 => c.bus.write32(obj + 0x88, 0),
                    1 => {
                        c.bus.write32(obj + 0x88, SLOT1);
                        c.bus.write32(SLOT1 + 8, 0);
                    }
                    2 => {
                        c.bus.write32(obj + 0x88, SLOT1);
                        c.bus.write32(SLOT1 + 8, 0xffff_ffff);
                        c.bus.write32(slot + 8, SLOT2);
                        c.bus.write32(SLOT2 + 0x1c, BASE);
                    }
                    _ => put_object_slots(c, obj, slot, 0x1234, 0x4321),
                }
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = if seed % 2 == 0 { 1 } else { 3 };
            },
        );
    }

    #[test]
    fn world_get_sphere_tree_at_matches() {
        check_lohi(
            ee::s13::f_003a6ce8__generated,
            world_get_sphere_tree_at,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x003a_6ce8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, DATA + 0x2000);
                c.g[A1][0] = mixed(&mut r, seed, (seed % 8) as u32);
            },
        );
    }

    #[test]
    fn world_on_game_enter_matches() {
        check(
            ee::s13::f_003a67e0__generated,
            world_on_game_enter,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_67e0, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                match seed % 3 {
                    0 => {}
                    1 => {
                        c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                        c.bus.write32(DATA + 0x1000 + 0x10 + 4, 0);
                    }
                    _ => {
                        c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                        c.bus.write32(DATA + 0x1000 + 0x10 + 4, DATA + 0x3000);
                        c.bus.write32(DATA + 0x3000 + 8, r.next_u32() % 8);
                    }
                }
            },
        );
    }

    #[test]
    fn world_get_table_entry_matches() {
        check(
            ssx3_ee::s13::f_003a6d00__generated,
            world_get_table_entry,
            SEEDS_3,
            &|c, seed| {
                let mut r = rng_2(seed, 32);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(DATA, DATA + 0x800);
                for i in 0..0x400 {
                    c.bus.write32(DATA + 0x800 + i * 4, r.next_u32());
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() & 0x3ff) as u64;
            },
        );
    }

    #[test]
    fn world_get_object_section_data_matches() {
        check(
            ssx3_ee::s13::f_003a6cc8__generated,
            world_get_object_section_data,
            SEEDS_3,
            &|c, seed| {
                let mut r = rng_2(seed, 33);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(DATA + 0x88, DATA + 0x200);
                c.bus.write32(DATA + 0x20c, DATA + 0x300);
                c.bus.write32(DATA + 0x308, r.next_u32());
                c.g[A1][0] = DATA as u64;
            },
        );
    }

    // -------------------------------------------------------------- world

    #[test]
    fn world_dtor_matches() {
        check(
            ssx3_ee::s13::f_003a6740__generated,
            world_dtor,
            8,
            &|c, seed| {
                let _ = base_2(c, seed, 0x003a_6740);
                heap(c);
                pool(c);
                let world = DATA + 0x100;
                let mgr = DATA + 0x5000;
                let vtable = DATA + 0x6000;
                clear(c, world, 0x100);
                clear(c, mgr, 0x1200);
                clear(c, vtable, 0x200);
                clear(c, DATA + 0x2000, 0x400);
                clear(c, DATA + 0x3000, 0x400);
                clear(c, DATA + 0x4000, 0x400);
                if seed % 2 == 0 {
                    w(c, world, DATA + 0x2000);
                    w(c, world + 4, DATA + 0x3000);
                    w(c, world + 8, DATA + 0x4000);
                }
                w(c, GAME_GP.wrapping_add(0x2a90), mgr);
                w(c, mgr + 0x10d8, vtable);
                c.bus.write16(vtable + 0x358, 0);
                w(c, vtable + 0x35c, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                c.g[A0][0] = sx(world);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    // ------------------------------------------------------- world / triggers

    #[test]
    fn world_ctor_matches() {
        check(
            ee::s13::f_003a6688__generated,
            world_ctor,
            60,
            &|c, seed| {
                alloc_base(c, seed, 0x3a66, 5);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    use crate::difftest::fill;

    /// `World_GetSectionScriptFunc`: handles over 4 sections' caches (type 3
    /// with a function table, or another type: null) and 8 entries; the real
    /// tWScriptCache_GetTableEntry.
    #[test]
    fn world_get_section_script_func_matches() {
        use crate::difftest::check;
        const WORLD: u32 = DATA + 0x100;
        const HANDLE: u32 = DATA + 0x200;
        const TABLE: u32 = DATA + 0x300;
        const CACHES: u32 = DATA + 0x400;
        const HEADER: u32 = DATA + 0x600;
        const FUNCS: u32 = DATA + 0x800;
        check(
            ee::s13::f_003a6d18__generated,
            world_get_section_script_func,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_6d18, 0);
                c.pc = 0x003a_6d18;
                c.bus.write32(WORLD + WSSF_SCRIPT_TABLE, TABLE);
                c.bus.write32(TABLE + WSSF_TABLE_CACHES, CACHES);
                for i in 0..4 {
                    let cache = CACHES + WSSF_CACHE_SIZE * i;
                    fill(c, &mut r, cache, 3);
                    c.bus.write16(
                        cache,
                        if (seed + i) % 3 == 0 {
                            r.next_u32() as u16
                        } else {
                            3
                        },
                    );
                    c.bus.write32(cache + 8, HEADER);
                }
                c.bus.write32(HEADER + 0x3c, FUNCS);
                fill(c, &mut r, FUNCS, 8);
                let handle = (seed % 4) | (seed / 4 % 8) << 8;
                c.bus.write32(HANDLE, handle);
                c.g[A0][0] = sx(WORLD);
                c.g[A1][0] = sx(HANDLE);
            },
        );
    }

    use crate::difftest::Rng;

    /// Section accessors: entry `i` of the 12-byte table, read back through
    /// the real script-cache getters.
    pub(crate) fn oc420_section_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x3a6c_0000 ^ seed as u64);
        scramble(c, &mut r);
        let (world, mid, table) = (DATA, DATA + 0x100, DATA + 0x200);
        c.pc = entry;
        c.g[A0][0] = sx(world);
        c.g[A1][0] = [0u64, 1, 2, 5, 0xffff_ffff_0000_0003][(seed % 5) as usize];
        c.bus.write32(world + 4, mid);
        c.bus.write32(mid + 4, table);
        for i in 0..8 {
            c.bus.write32(table + 12 * i + 0x20, r.next_u32());
            c.bus.write32(table + 12 * i + 0x24, r.next_u32());
        }
    }

    #[test]
    fn world_get_section_missions_matches() {
        check_lohi(
            ee::s13::f_003a6c98__generated,
            world_get_section_missions,
            40,
            &|c, seed| oc420_section_fixture(c, seed, 0x003a_6c98),
        );
    }

    #[test]
    fn world_get_section_mission_count_matches() {
        check_lohi(
            ee::s13::f_003a6c68__generated,
            world_get_section_mission_count,
            40,
            &|c, seed| oc420_section_fixture(c, seed, 0x003a_6c68),
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::oc434_w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_force;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_run;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC433_FIX;

    /// `World_ResetSections` against the generated body: 0..3 sections (some
    /// resident), each with 0..3 sub-entries pointing at table entries that
    /// are sometimes missing. Dispatcher-shaped: entered at its start.
    #[test]
    fn world_reset_sections_matches() {
        oc433_run(
            ee::s13::f_003a6800__generated,
            world_reset_sections,
            0x003a_6800,
            7,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, a, b, tbl, secs) =
                    (f + 0x1000, f + 0x1100, f + 0x1200, f + 0x1300, f + 0x2000);
                oc433_w(c, this, a);
                oc433_w(c, a, b);
                oc433_w(c, a + 8, tbl);
                oc433_w(c, b + 0x14, secs);
                let count = seed % 4;
                let mut resident = Vec::new();
                for i in 0..3u32 {
                    let sub = (r.next_u32() % 4) as u16;
                    c.bus.write16(secs + 0x58 * i + 0x26, sub);
                    resident.push(((seed >> (2 + i)) & 1) as i32 | (i == 0) as i32);
                    let p = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        f + 0x3000 + 0x100 * i
                    };
                    oc433_w(c, tbl + 4 * i, p);
                    oc433_w(c, f + 0x3000 + 0x100 * i + 0x1c, f + 0x4000 + 0x100 * i);
                    for s in 0..3u32 {
                        let ptr = f + 0x5000 + 0x40 * (i * 3 + s);
                        let x = if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            (ptr >> 2) << 8
                        };
                        oc433_w(c, f + 0x4000 + 0x100 * i + 4 * s, x);
                        oc433_w(c, ptr + 8, r.next_u32());
                    }
                }
                oc433_force(c, 0x003a_9800, &[count as i32]);
                oc433_force(c, 0x003a_9858, &resident);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `World_GetSectionShutdownFuncs`: entry `i` of the script cache (the real
    /// `tWScriptCache_GetGlobals` decides).
    #[test]
    fn world_get_section_shutdown_funcs_matches() {
        check(
            ee::s13::f_003a6bd8__generated,
            world_get_section_shutdown_funcs,
            24,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x3a6bd8, 0, 0x003a_6bd8);
                let (this, cache, entries, data) =
                    (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x500);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 3) as u64;
                oc434_w(c, this + 4, cache);
                oc434_w(c, cache + 4, entries);
                for i in 0..3u32 {
                    let e = entries + 0xc * i;
                    c.bus
                        .write16(e, if (seed / 3 + i) % 3 == 0 { 2 } else { 3 });
                    oc434_w(
                        c,
                        e + 8,
                        if (seed / 5 + i) % 4 == 1 {
                            0
                        } else {
                            data + 0x20 * i
                        },
                    );
                    oc434_w(c, data + 0x20 * i + 0x14, r.next_u32());
                }
            },
        );
    }

    pub(crate) const SEEDS_4: u32 = 200;

    /// Fixture objects live above the area the fakes' default results point
    /// into (`DATA..DATA + 0x40000`).
    pub(crate) const BASE_2: u32 = DATA + 0x4_0000;

    pub(crate) fn w_2(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The `World_GetSection*` fixture: world -> cache -> entry table of 12
    /// byte entries (kind halfword, data pointer at `+8`), the data blocks
    /// random. The callees (the `tWScriptCache_*` leaves) run for real.
    pub(crate) fn world_section_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let (world, cache, table) = (BASE_2 + 0x100, BASE_2 + 0x200, BASE_2 + 0x300);
        w_2(c, world + 4, cache);
        w_2(c, cache + 4, table);
        for i in 0..4u32 {
            let entry = table + 12 * i;
            let data = BASE_2 + 0x1000 + 0x100 * i;
            c.bus
                .write16(entry, if (seed + i) % 5 == 4 { 1 } else { 3 });
            w_2(c, entry + 8, if (seed + i) % 7 == 3 { 0 } else { data });
            for k in 0..0x20u32 {
                w_2(c, data + 4 * k, r.next_u32());
            }
        }
        c.g[A0][0] = world as u64;
        c.g[A1][0] = (seed % 4) as u64;
    }

    /// `World_GetSectionProcesses`, `World_GetSectionProcessCount` and
    /// `World_GetSectionShutdownFuncCount`: the section's cache entry passed
    /// to the matching `tWScriptCache_*` getter.
    #[test]
    fn world_get_section_wrappers_match() {
        for (n, (gen, hand)) in [
            (
                ee::s13::f_003a6c38__generated as fn(&mut Cpu),
                world_get_section_processes as fn(&mut Cpu),
            ),
            (
                ee::s13::f_003a6c08__generated,
                world_get_section_process_count,
            ),
            (
                ee::s13::f_003a6ba8__generated,
                world_get_section_shutdown_func_count,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            check(gen, hand, SEEDS_4, &|c, seed| {
                world_section_setup(c, seed, 0x3a6000 + n as u64)
            });
        }
    }
}
