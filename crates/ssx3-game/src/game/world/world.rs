//! `src/world/world.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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
}
