//! `src/animation/mdfarchive.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::be::beprepoststate::gp_lw;
pub(crate) use crate::game::fe::feasyncfile::gp_11 as gp;
pub(crate) use crate::game::fe::ovstates::at;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_4cb0, mdf_file_create_model, "MdfFile_CreateModel"),
    (
        0x0031_4d60,
        game_anim_map_compare_name_index,
        "GameAnimMap_CompareNameIndex",
    ),
    (0x0031_4d98, game_anim_map_ctor, "GameAnimMap_ctor"),
    (
        0x0031_4dd8,
        game_anim_map_build_name_index,
        "GameAnimMap_BuildNameIndex",
    ),
    (0x0031_4e88, game_anim_map_find_hash, "GameAnimMap_FindHash"),
    (
        0x0031_4ef0,
        game_anim_map_set_handle,
        "GameAnimMap_SetHandle",
    ),
    (
        0x0031_4f30,
        game_anim_map_register_bank,
        "GameAnimMap_RegisterBank",
    ),
    (
        0x0031_4fe8,
        game_anim_map_unregister_bank,
        "GameAnimMap_UnregisterBank",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;

    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
}

/// `void GameAnimMap_ctor(GameAnimMap *this)` (0x314d98): publish the map at
/// `gp+0xd8c`.
pub fn game_anim_map_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x205;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0xd8c)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

/// `void GameAnimMap_SetHandle(GameAnimMap* self, u32 hash, void* value)`
/// (0x314ef0): store `value` in the map entry for `hash` (when it exists).
pub fn game_anim_map_set_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        callee::GAME_ANIM_MAP_FIND_HASH.call(c, 0x0031_4f0c)?;
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = sx((r32(c, V0) as u32) << 2);
        if !missing {
            c.cycles += 2;
            c.g[V0][0] = add32(c.g[V0][0], c.g[S0][0]);
            c.sw(p(r32(c, V0), 0x1030), c.g[S1][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GameAnimMap_CompareNameIndex(int* a, int* b)` (0x314D60): the qsort
/// comparator over the name table at `gp+0xD88`, ordering by index.
pub fn game_anim_map_compare_name_index(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A3][0] = c.lw(c.g[A0][0] as u32);
    c.g[T0][0] = 1;
    c.g[A2][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = gp_lw(c, 0xd88);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A3][0] = sx(ea(c.g[A3][0], c.g[A0][0] as u32));
    c.g[A2][0] = sx(ea(c.g[A2][0], c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[T0][0];
    }
    abi::ret(c);
}

/// `int GameAnimMap_FindHash(GameAnimMap* self, int hash)` (0x314e88):
/// binary search of the 0x207-entry name order (`u32` indices at +0x818)
/// against the hashes at the object base; -1 when absent.
pub fn game_anim_map_find_hash(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0] as u32;
    c.g[T1][0] = sx(this.wrapping_add(0x818));
    c.g[T0][0] = 0;
    c.g[A3][0] = 0x206;
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    loop {
        c.cycles += 11;
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 31);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = lw(c, c.g[V0][0] as u32);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = lw(c, c.g[V1][0] as u32);
        if c.g[A1][0] == c.g[V1][0] {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] == 0 {
            c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 1;
            c.g[A3][0] = c.g[A2][0];
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        if !more {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_4e98);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Animation maps.

/// `void GameAnimMap_UnregisterBank(GameAnimMap* self, AnimBank* bank)`
/// (0x314fe8): clear every handle the bank registered, then forget the bank.
pub fn game_anim_map_unregister_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, S1), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 2)).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = 0;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            loop {
                c.cycles += 12;
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = Addr(gp(c, 0x2860)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                c.g[RA][0] = sx(0x0031_503c);
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x14));
                callee::GAME_ANIM_MAP_SET_HANDLE.call(c, 0x0031_503c)?;
                c.g[V1][0] = Addr(p(r32(c, S1), 4)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 2)).lh(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_5020);
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

// ---------------------------------------------------------------------------
// `GameAnimMap_BuildNameIndex` (0x314dd8): hash the 518 fixed animation names
// into `self->hash`, fill `self->order` with their indices, sort the order by
// hash and clear the map.
pub fn game_anim_map_build_name_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x490000;
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[S3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0u64;
        frame.save(c, S4, 0x10);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3040));
        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x818));
        frame.save_ra(c, 0);
        loop {
            c.cycles += 9;
            c.g[A0][0] = c.lw(c.g[S2][0] as u32);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00317670, 0x0031_4e1c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x818), c.g[S1][0]);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 518i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_4e10);
            }
        }
        c.cycles += 20;
        c.g[A3][0] = 0x310000;
        c.g[A0][0] = c.g[S4][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4d60));
        c.g[A1][0] = 0x206;
        c.g[A2][0] = 0x4;
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd88), c.g[S3][0]);
        call_ptr(c, 0x00418ef8, 0x0031_4e50)?;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1030));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A2][0] = 0x818;
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd88), 0u64);
        call_ptr(c, 0x00416210, 0x0031_4e64)?;
        c.lq(16, frame.at(c, 0x50));
        c.lq(17, frame.at(c, 0x40));
        c.lq(18, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x20));
        c.lq(20, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MdfFile_CreateModel(self)` (0x314cb0): name the model after the
/// MDF file, allocate and copy its 0x60-byte header.
pub fn mdf_file_create_model(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x60;
        frame.save(c, S0, 0x70);
        c.mult(0, 16, 5, 2);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0x40);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5a30));
        c.g[A2][0] = Addr(p(this as u32, 8)).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x0041_7828, 0x0031_4ce8)?;
        c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        c.g[A2][0] = 0x3000000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
        call_ptr(c, 0x0031_7e30, 0x0031_4d08)?;
        c.g[A1][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c);
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = 0x60;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x003e_6574, 0x0031_4d20)?;
        c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x14)).lw(c);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c.g[S2][0] as u32, 0xc)).lw(c);
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x003b_47f8, 0x0031_4d44)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Animation.

/// `void GameAnimMap_RegisterBank(GameAnimMap* self, int kind, void* bank)`
/// (0x314f30): for each of `bank->count` handles (word at `bank+8`, stride
/// 0x14) register `handle | kind << 8` at index `i`.
pub fn game_anim_map_register_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x80);
        c.sq(17, at(c.g[SP][0] as u32, 0x60));
        c.sq(18, at(c.g[SP][0] as u32, 0x50));
        c.g[S1][0] = 0;
        c.sq(21, at(c.g[SP][0] as u32, 0x20));
        c.g[S2][0] = c.g[A2][0];
        c.sq(16, at(c.g[SP][0] as u32, 0x70));
        c.sq(19, at(c.g[SP][0] as u32, 0x40));
        c.sq(20, at(c.g[SP][0] as u32, 0x30));
        c.sq(22, at(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(4).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = c.g[A0][0];
        if !empty {
            c.cycles += 3;
            c.g[S4][0] = c.g[A1][0] & 0xff;
            c.g[S6][0] = 0xffff_ffff_ffff_ff00;
            c.g[S3][0] = 0;
            loop {
                c.cycles += 18;
                c.g[S0][0] = c.g[S0][0] & c.g[S6][0];
                c.g[V1][0] = Addr(c.g[S2][0] as u32).off(8).lw(c);
                c.g[S0][0] = c.g[S0][0] | c.g[S4][0];
                c.g[A0][0] = sx((c.g[S1][0] as u32) << 8);
                c.g[V0][0] = c.g[S0][0] & 0xff;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0031_4fac);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0031_4ef0, 0x0031_4fac)?;
                c.g[V1][0] = Addr(c.g[S2][0] as u32).off(4).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4f78);
                }
            }
        }
        c.cycles += 10;
        c.lq(16, at(c.g[SP][0] as u32, 0x70));
        c.lq(17, at(c.g[SP][0] as u32, 0x60));
        c.lq(18, at(c.g[SP][0] as u32, 0x50));
        c.lq(19, at(c.g[SP][0] as u32, 0x40));
        c.lq(20, at(c.g[SP][0] as u32, 0x30));
        c.lq(21, at(c.g[SP][0] as u32, 0x20));
        c.lq(22, at(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x80);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::h;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_2;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn game_anim_map_ctor_matches() {
        check(
            ssx3_ee::s10::f_00314d98,
            game_anim_map_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP + 0xd8c, r.next_u32());
            },
        );
    }

    #[test]
    fn game_anim_map_set_handle_matches() {
        check(
            ee::s10::f_00314ef0__generated,
            game_anim_map_set_handle,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x314ef0, 0);
                let map = DATA + 0x200;
                c.g[A0][0] = map as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    1 + (seed % 5) as u64
                };
                c.g[A2][0] = (seed as u64) | 0x1000;
            },
        );
    }

    // ------------------------------------------------------- comparators

    #[test]
    fn game_anim_map_compare_name_index_matches() {
        check(
            ee::s10::f_00314d60__generated,
            game_anim_map_compare_name_index,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_4d60, 0);
                belib(c);
                let table = DATA + 0x2000;
                w(c, GAME_GP + 0xd88, table);
                for i in 0..16u32 {
                    w(c, table + 4 * i, r.next_u32() % 32);
                }
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x104) as u64;
                w(c, DATA + 0x100, r.next_u32() % 16);
                w(c, DATA + 0x104, r.next_u32() % 16);
            },
        );
    }

    #[test]
    fn game_anim_map_find_hash_matches() {
        check(
            ssx3_ee::s10::f_00314e88__generated,
            game_anim_map_find_hash,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x314e88);
                c.g[A0][0] = DATA as u64;
                // A sorted table of 0x207 hashes with the identity order.
                for i in 0..0x207u32 {
                    c.bus.write32(DATA + 4 * i, 100 + i * 10);
                    c.bus.write32(DATA + 0x818 + 4 * i, i);
                }
                let hash = if seed % 2 == 0 {
                    100 + (seed % 0x207) * 10
                } else {
                    r.next_u32()
                };
                c.g[A1][0] = hash as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Animation maps and camera triggers.

    #[test]
    fn game_anim_map_unregister_bank_matches() {
        check(
            ssx3_ee::s10::f_00314fe8__generated,
            game_anim_map_unregister_bank,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x314fe8, 3);
                be_library(c);
                world(c);
                let map = DATA + 0x100;
                let bank = DATA + 0x200;
                c.g[A0][0] = map as u64;
                c.g[A1][0] = bank as u64;
                w(c, GAME_GP.wrapping_add(0x2860), 0);
                let count = (r.next_u32() % 3) as i32;
                w(c, bank + 4, DATA + 0x300);
                h(c, DATA + 0x300 + 2, count as u16);
                w(c, bank + 8, DATA + 0x400);
                for i in 0..3 {
                    w(c, DATA + 0x400 + 0x14 * i, r.next_u32() & 0xffff);
                }
            },
        );
    }

    #[test]
    fn game_anim_map_build_name_index_matches() {
        check(
            ee::s10::f_00314dd8__generated,
            game_anim_map_build_name_index,
            12,
            &|c, seed| {
                let _r = base(c, seed, 0x314d);
                c.g[A0][0] = sx(DATA);
                let empty = DATA + 0x1000;
                for i in 0..0x206u32 {
                    c.bus.write32(0x0048_cfc0 + 4 * i, empty);
                }
                c.bus.write8(empty, 0);
            },
        );
    }

    #[test]
    fn mdf_file_create_model_matches() {
        check(
            ee::s10::f_00314cb0__generated,
            mdf_file_create_model,
            4,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0031_4cb0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = 1;
                w(c, OBJ + 8, 0x1000);
                w(c, OBJ + 0xc, 0x1000);
                w(c, 0x1074, (seed % 4) as u32);
                w(c, 0x1070, 0);
                w(c, 0x1000, 0);
            },
        );
    }

    // ------------------------------------------------------------ Animation.

    #[test]
    fn game_anim_map_register_bank_matches() {
        check(
            ee::s10::f_00314f30__generated,
            game_anim_map_register_bank,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_4f30, 0);
                let bank = DATA + 0x1000;
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(bank);
                let n = if seed % 6 == 0 { 0 } else { r.next_u32() % 4 };
                c.bus.write32(bank + 4, DATA + 0x1200);
                c.bus.write16(DATA + 0x1202, n as u16);
                c.bus.write32(bank + 8, DATA + 0x1400);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x1400 + i * 0x14, r.next_u32());
                }
            },
        );
    }
}
