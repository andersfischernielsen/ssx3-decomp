//! `src/be/bedbreward.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw;
pub(crate) use crate::game::world::world::oc435_callee;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_ae00, rewards_file_unload, "RewardsFile_Unload"),
    (
        0x0015_c228,
        be_clear_option_state_5308b8,
        "BE_ClearOptionState5308b8",
    ),
    (
        0x0015_c830,
        be_pre_race_destroy_instance,
        "BEPreRace_DestroyInstance",
    ),
    (0x0015_c870, be_race_destroy, "BERace_Destroy"),
    (
        0x0015_c8b0,
        be_post_race_destroy_instance,
        "BEPostRace_DestroyInstance",
    ),
    (0x0015_c8f0, be_global_ctors, "BE_GlobalCtors"),
    (0x0015_c910, be_global_dtors, "BE_GlobalDtors"),
    (0x0015_c930, algorithm_list_ctor, "AlgorithmList_ctor"),
    (0x0015_c940, algorithm_list_dtor, "AlgorithmList_dtor"),
    (
        0x0031_84e8,
        c_bx_string_assign_cstr,
        "cBXString_assign_cstr",
    ),
    (0x0015_ad58, rewards_file_load_2, "RewardsFile_Load"),
    (0x0015_aec0, rewards_file_poll_load, "RewardsFile_PollLoad"),
    (0x0015_b028, be_static_init_destroy, "BE_StaticInitDestroy"),
    (
        0x0015_bc10,
        be_assign_global_string_1bc8,
        "BE_AssignGlobalString1bc8",
    ),
    (
        0x0015_bc30,
        be_copy_global_string_1bc8,
        "BE_CopyGlobalString1bc8",
    ),
    (
        0x0015_c188,
        be_set_name_entry_535617,
        "BE_SetNameEntry535617",
    ),
    (0x0015_c338, be_set_string_534d48, "BE_SetString534d48"),
    (0x0015_c368, be_set_string_534dc0, "BE_SetString534dc0"),
    (0x0015_c398, be_set_string_534e38, "BE_SetString534e38"),
    (
        0x0015_c738,
        be_score_int_destroy_instance,
        "BEScoreInt_DestroyInstance",
    ),
];

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {

    /// `operator delete(p)` (`__builtin_delete`).
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;

    /// `memset(dst, value, size)` (`EA_MemSet`).
    pub const EA_MEMSET: u32 = 0x003e_6448;

    /// `World_CreateDeadNode(world, position)`.
    pub const WORLD_CREATE_DEAD_NODE: u32 = 0x002d_1cc0;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;

    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;

    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;

    pub(crate) use crate::game::render::particle::callee::STRLEN;
}

/// `AlgorithmList* AlgorithmList_ctor(AlgorithmList *self)` (0x15c930):
/// clear the list head/tail and return `self`.
pub fn algorithm_list_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = a0;
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `__builtin_delete(0x317e50)`, cross-shard (through the function table).
pub(crate) const BUILTIN_DELETE_ADDR: u32 = 0x0031_7e50;

pub(crate) const ALGORITHM_LIST_CLEAR: Callee = Callee {
    addr: 0x0015_cb08,
    f: ee::s01::f_0015cb08,
};

/// `void AlgorithmList_dtor(AlgorithmList *list, uint flags)` (0x15c940):
/// clear the list, then delete it when `flags & 1`.
pub fn algorithm_list_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ALGORITHM_LIST_CLEAR.call(c, 0x0015_c95c)?;
        c.g[S0][0] &= 1;
        let delete_list = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_list {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0015_c970)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// More back-end singletons, character profiles and economy.

pub(crate) const BE_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0015_b028,
    f: ssx3_ee::s01::f_0015b028,
};

/// `void BE_GlobalCtors(void)` (0x15c8f0): `BE_StaticInitDestroy(1, 0xffff)`.
pub fn be_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        BE_STATIC_INIT_DESTROY.call(c, 0x0015_c904)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_GlobalDtors(void)` (0x15c910): `BE_StaticInitDestroy(0, 0xffff)`.
pub fn be_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        BE_STATIC_INIT_DESTROY.call(c, 0x0015_c924)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const BUILTIN_DELETE: u32 = 0x0031_7e50;

pub(crate) const PRE_RACE_GP: u32 = 0x1e98;

pub(crate) const RACE_GP: u32 = 0x1ec4;

pub(crate) const POST_RACE_GP: u32 = 0x1e9c;

/// `BEPreRace_DestroyInstance()` (0x15c830): as above for `gp - 0x1e98`.
pub fn be_pre_race_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, PRE_RACE_GP, 0x0015_c844);
}

/// `BERace_Destroy()` (0x15c870): as above for `gp - 0x1ec4`.
pub fn be_race_destroy(c: &mut Cpu) {
    destroy_instance(c, RACE_GP, 0x0015_c884);
}

/// `BEPostRace_DestroyInstance()` (0x15c8b0): as above for `gp - 0x1e9c`.
pub fn be_post_race_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, POST_RACE_GP, 0x0015_c8c4);
}

/// The shared body of the `*_DestroyInstance` family: load the singleton,
/// `__builtin_delete` it, then clear the global. `ret` is the return address
/// the original passes to the delete call.
pub(crate) fn destroy_instance(c: &mut Cpu, gp_off: u32, ret: u32) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(gp_off);
        c.g[A0][0] = Addr(global).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, ret)?;
        frame.restore_ra(c, 0);
        Addr(global).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_ClearOptionState5308b8(void)` (0x15c228): zero the 0x20-byte option
/// state at `0x5308b8`.
pub fn be_clear_option_state_5308b8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x0053_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
        c.g[A2][0] = 0x20;
        call_ptr(c, table_callee::EA_MEMSET, 0x0015_c244)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_assign_cstr(cBXString* self, const char* str)`
/// (0x3184e8): replace the contents with a C string.
pub fn c_bx_string_assign_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let len;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0031_850c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRLEN, 0x0031_850c)?;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        }
        c.cycles += 9;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = len;
        c.g[RA][0] = sx(0x0031_8528);
        callee::BXSTRING_ASSIGN_COPY.call(c, 0x0031_8528)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Reward database.

/// `void cBERewardDB_init(cBERewardDB* db, int use_file)` (0x15ad58): with a
/// bolt-file name given, acquire the bolt file for that rider; otherwise load
/// and parse the rewards file. Store the result in the database's first word.
pub fn rewards_file_load(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'end: {
            'named: {
                c.cycles += 7;
                frame.save(c, S2, 0x30);
                frame.save(c, S0, 0x20);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S1, 0x10);
                let t = c.g[A1][0] != 0;
                frame.save_ra(c, 0);
                if t {
                    break 'named;
                }
                c.cycles += 19;
                c.g[RA][0] = sx(0x0015_ad7c);
                c.g[A0][0] = lw(c, gp(c, -0x1e80));
                call_ptr(c, 0x0031_7f60, 0x0015_ad7c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(0x0045_ab78);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0015_ad98);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0015_ad98)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = lw(c, gp(c, -0x1e80));
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0015_adac);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003e_1bb8, 0x0015_adac)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0015_adb8);
                c.g[A0][0] = c.g[S2][0];
                callee::REWARDS_FILE_PARSE.call(c, 0x0015_adb8)?;
                c.g[V0][0] = sx(0xffff_ffff);
                break 'end;
            }
            c.cycles += 9;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            c.g[T0][0] = c.g[A1][0];
            c.g[A2][0] = lw(c, gp(c, -0x1e80));
            c.g[A1][0] = 0;
            c.g[A0][0] = lw(c, at(c.g[V0][0], 0x7c));
            c.g[A3][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[RA][0] = sx(0x0015_ade4);
            c.g[A0][0] = sx(at(c.g[A0][0], 0x1a70));
            callee::BOLT_FILE_ACQUIRE.call(c, 0x0015_ade4)?;
            break 'end;
        }
        c.cycles += 7;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `RewardsFile_Unload` (0x0015ae00).
//
// `void RewardsFile::Unload(RewardsFile* self)`: drop the open sound handle
// from the BE sound object, free the loaded data and clear the file record.

pub fn rewards_file_unload(c: &mut Cpu) {
    guest(c, |c| {
        'b_15ae44: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            let t = (c.g[A2][0] as i64) < 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_15ae44;
            } else {
                c.cycles += 9;
                c.g[A1][0] = 0u64;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
                c.g[RA][0] = sx(0x0015ae34);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0019dc20, 0x0015ae34)?;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0015ae44);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0019d4b8, 0x0015ae44)?;
                break 'b_15ae44;
            }
        }
        'b_15ae5c: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_15ae5c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0015ae58);
                call_ptr(c, 0x00317e98, 0x0015ae58)?;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_15ae5c;
            }
        }
        c.cycles += 24;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x38), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x44), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void RewardsFile_Load(RewardsFile **out, int a1)` (0x15ad58): with `a1 == 0`
/// reads the rewards file (size 0x317f60, buffer 0x317e30, load 0x3e1bb8) and
/// parses it (`RewardsFile_Parse(out, data)`), result -1; otherwise starts the
/// async bolt file acquire (0x19da10, world + 0x7c + 0x1a70); stores the
/// result in `*out`. Frame 0x40 (s0..s2).
pub fn rewards_file_load_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        let async_load = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if !async_load {
            c.cycles += 19;
            c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1e80)).lw(c);
            call_ptr(c, 0x0031_7f60, 0x0015_ad7c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0x46_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x5488));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0015_ad98)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1e80)).lw(c);
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x003e_1bb8, 0x0015_adac)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            oc435_callee!(0x0015_a818, ee::s01::RewardsFile_Parse).call(c, 0x0015_adb8)?;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 9;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            c.g[T0][0] = c.g[A1][0];
            c.g[A2][0] = Addr(r32(c, GP).wrapping_sub(0x1e80)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
            c.g[A3][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1a70));
            call_ptr(c, 0x0019_da10, 0x0015_ade4)?;
        }
        c.cycles += 7;
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The address `off(rs)` from the register's current value (the generated
/// code's `(c.g[rs][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn bsi_ea(c: &Cpu, rs: usize, off: i32) -> u32 {
    (c.g[rs][0] as u32).wrapping_add(off as u32)
}

/// `void BE_StaticInitDestroy(int param_1, uint param_2)` (0x15b028), the
/// static constructor and destructor of the back end (`BE_GlobalCtors` passes
/// `1, 0xffff`, `BE_GlobalDtors` passes `0, 0xffff`). Only `param_2 == 0xffff`
/// does anything: `param_1 != 0` writes the initial values of the statics
/// (the rewards and option tables, the character profile slots, the name
/// string and the bolt DB), `param_1 == 0` destroys the profile, rewards,
/// bolt and name-string singletons.
pub fn be_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        'b_15bacc: {
            c.g[V0][0] = 0xffff;
            frame.save(c, 16, 0x90);
            frame.save(c, 17, 0x80);
            frame.save(c, 18, 0x70);
            frame.save(c, 19, 0x60);
            frame.save(c, 20, 0x50);
            frame.save(c, 21, 0x40);
            frame.save(c, 22, 0x30);
            frame.save(c, 23, 0x20);
            frame.save(c, 30, 0x10);
            let t = c.g[A1][0] != c.g[V0][0];
            frame.save_ra(c, 0);
            if t {
                break 'b_15bacc;
            } else {
                'b_15ba2c: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[T5][0] = 0x4a0000;
                    if t {
                        break 'b_15ba2c;
                    } else {
                        // param_1 != 0: reset the statics to their initial values.
                        c.cycles += 511;
                        c.lwc1(9, bsi_ea(c, GP, -0x6de8));
                        c.lwc1(10, bsi_ea(c, GP, -0x6de4));
                        c.g[T6][0] = 0x4a0000;
                        c.lwc1(11, bsi_ea(c, GP, -0x6de0));
                        c.g[T7][0] = 0x4a0000;
                        c.lwc1(7, bsi_ea(c, GP, -0x6ddc));
                        c.g[V0][0] = sx(bsi_ea(c, T7, 0x6400));
                        c.swc1(9, bsi_ea(c, T5, 0x63e0));
                        c.g[S1][0] = 0x4a0000;
                        c.swc1(10, bsi_ea(c, T6, 0x63f0));
                        c.g[S2][0] = 0x4a0000;
                        c.swc1(11, bsi_ea(c, T7, 0x6400));
                        c.g[S3][0] = 0x4a0000;
                        c.lwc1(8, bsi_ea(c, GP, -0x6dd8));
                        c.g[A0][0] = sx(bsi_ea(c, S3, 0x6430));
                        c.lwc1(4, bsi_ea(c, GP, -0x6dd4));
                        c.g[S5][0] = 0x4a0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6dd0));
                        c.g[S4][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, V0, 0xc));
                        c.g[T3][0] = 0x4a0000;
                        c.lwc1(2, bsi_ea(c, GP, -0x6dcc));
                        c.g[A1][0] = sx(bsi_ea(c, S5, 0x63d0));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[A3][0] = sx(bsi_ea(c, T5, 0x63e0));
                        c.swc1(8, bsi_ea(c, S1, 0x6410));
                        c.g[T1][0] = sx(bsi_ea(c, T6, 0x63f0));
                        c.swc1(4, bsi_ea(c, S2, 0x6420));
                        c.g[V1][0] = sx(bsi_ea(c, S1, 0x6410));
                        c.swc1(5, bsi_ea(c, S3, 0x6430));
                        c.g[T0][0] = sx(bsi_ea(c, S2, 0x6420));
                        c.lwc1(3, bsi_ea(c, GP, -0x6dc8));
                        c.g[A2][0] = sx(bsi_ea(c, S4, 0x6440));
                        c.lwc1(1, bsi_ea(c, GP, -0x6dc4));
                        c.g[S1][0] = 0x4a0000;
                        c.swc1(2, bsi_ea(c, A0, 0xc));
                        c.g[S0][0] = sx(bsi_ea(c, S1, 0x64a0));
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 6);
                        c.g[S2][0] = 0x4a0000;
                        c.swc1(3, bsi_ea(c, S4, 0x6440));
                        c.g[S1][0] = sx(bsi_ea(c, S2, 0x64b0));
                        c.swc1(1, bsi_ea(c, T3, 0x6450));
                        c.g[S3][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S5, 0x63d0));
                        c.g[S2][0] = sx(bsi_ea(c, S3, 0x64c0));
                        c.swc1(6, bsi_ea(c, A1, 0xc));
                        c.g[S6][0] = 0x4a0000;
                        c.sw(bsi_ea(c, A3, 0xc), 0u64);
                        c.g[T4][0] = sx(bsi_ea(c, T3, 0x6450));
                        c.lwc1(17, bsi_ea(c, GP, -0x6dc0));
                        c.g[S3][0] = sx(bsi_ea(c, S6, 0x64d0));
                        c.lwc1(18, bsi_ea(c, GP, -0x6dbc));
                        c.g[FP][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T1, 0xc), 0u64);
                        c.g[T2][0] = sx(bsi_ea(c, FP, 0x6460));
                        c.lwc1(13, bsi_ea(c, GP, -0x6db8));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(14, bsi_ea(c, GP, -0x6db4));
                        c.g[T7][0] = sx(bsi_ea(c, S7, 0x6490));
                        c.swc1(17, bsi_ea(c, V0, 0x4));
                        c.g[T9][0] = 0x4a0000;
                        c.swc1(18, bsi_ea(c, V0, 0x8));
                        c.g[T5][0] = sx(bsi_ea(c, T9, 0x6470));
                        c.swc1(13, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(14, bsi_ea(c, V1, 0x8));
                        c.g[S5][0] = sx(bsi_ea(c, V0, 0x64e0));
                        c.sw(bsi_ea(c, V1, 0xc), 0u64);
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(15, bsi_ea(c, GP, -0x6db0));
                        c.g[S6][0] = sx(bsi_ea(c, V0, 0x64f0));
                        c.lwc1(16, bsi_ea(c, GP, -0x6dac));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T0, 0xc), 0u64);
                        c.g[V0][0] = sx(bsi_ea(c, V1, 0x6520));
                        c.lwc1(12, bsi_ea(c, GP, -0x6da8));
                        c.g[T8][0] = 0x4a0000;
                        c.swc1(15, bsi_ea(c, A0, 0x4));
                        c.g[T6][0] = sx(bsi_ea(c, T8, 0x6480));
                        c.swc1(16, bsi_ea(c, A0, 0x8));
                        c.swc1(12, bsi_ea(c, A2, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A2, 0xc));
                        c.g[S4][0] = sx(bsi_ea(c, A0, 0x6530));
                        c.swc1(0, bsi_ea(c, A1, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A1, 0x8));
                        c.g[V1][0] = sx(bsi_ea(c, A0, 0x6540));
                        c.swc1(0, bsi_ea(c, A3, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A3, 0x8));
                        c.g[T3][0] = sx(bsi_ea(c, A0, 0x6550));
                        c.swc1(0, bsi_ea(c, T1, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T1, 0x8));
                        c.g[A1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T0, 0x4));
                        c.g[A1][0] = sx(bsi_ea(c, A1, 0x1278));
                        c.swc1(0, bsi_ea(c, T0, 0x8));
                        c.lwc1(19, bsi_ea(c, GP, -0x6da4));
                        c.swc1(0, bsi_ea(c, A2, 0x8));
                        c.lwc1(2, bsi_ea(c, GP, -0x6da0));
                        c.g[A2][0] = sx(bsi_ea(c, A0, 0x6570));
                        c.swc1(19, bsi_ea(c, T4, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(1, bsi_ea(c, GP, -0x6d9c));
                        c.g[A3][0] = sx(bsi_ea(c, A0, 0x6580));
                        c.swc1(2, bsi_ea(c, FP, 0x6460));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(8, bsi_ea(c, GP, -0x6d98));
                        c.g[T0][0] = sx(bsi_ea(c, A0, 0x65d0));
                        c.swc1(1, bsi_ea(c, T2, 0xc));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(3, bsi_ea(c, GP, -0x6d94));
                        c.g[T1][0] = sx(bsi_ea(c, A0, 0x6630));
                        c.swc1(8, bsi_ea(c, T7, 0xc));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(4, bsi_ea(c, GP, -0x6d90));
                        c.g[FP][0] = 0xffffffffffff0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6d8c));
                        c.g[FP][0] = c.g[FP][0] | 0xff00;
                        c.swc1(3, bsi_ea(c, T4, 0x8));
                        c.sw(bsi_ea(c, T4, 0xc), 0u64);
                        c.swc1(4, bsi_ea(c, T2, 0x4));
                        c.g[T4][0] = sx(bsi_ea(c, A0, 0x6650));
                        c.swc1(5, bsi_ea(c, T2, 0x8));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T9, 0x6470));
                        c.swc1(0, bsi_ea(c, T5, 0xc));
                        c.swc1(0, bsi_ea(c, T8, 0x6480));
                        c.sw(bsi_ea(c, T6, 0xc), 0u64);
                        c.lwc1(7, bsi_ea(c, GP, -0x6d88));
                        c.swc1(0, bsi_ea(c, S7, 0x6490));
                        c.swc1(7, bsi_ea(c, T7, 0x8));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64a0));
                        c.swc1(0, bsi_ea(c, S0, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64b0));
                        c.swc1(0, bsi_ea(c, S1, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A0, 0x64c0));
                        c.g[T2][0] = sx(bsi_ea(c, S7, 0x6660));
                        c.sw(bsi_ea(c, S2, 0xc), 0u64);
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64d0));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T5, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T5, 0x8));
                        c.sw(bsi_ea(c, T6, 0x4), 0u64);
                        c.sw(bsi_ea(c, T6, 0x8), 0u64);
                        c.swc1(8, bsi_ea(c, T7, 0x4));
                        c.g[T6][0] = sx(bsi_ea(c, S7, 0x66f0));
                        c.swc1(0, bsi_ea(c, S0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S0, 0x8));
                        c.swc1(0, bsi_ea(c, S1, 0x4));
                        c.swc1(0, bsi_ea(c, S1, 0x8));
                        c.sw(bsi_ea(c, S2, 0x4), 0u64);
                        c.g[S1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S2, 0x8), 0u64);
                        c.g[S0][0] = sx(bsi_ea(c, S1, 0x66e0));
                        c.g[S2][0] = sx(bsi_ea(c, A0, 0x66d0));
                        c.sw(bsi_ea(c, S3, 0x4), 0u64);
                        c.sw(bsi_ea(c, S3, 0x8), 0u64);
                        c.g[A0][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S3, 0xc), 0u64);
                        c.g[A0][0] = sx(bsi_ea(c, A0, 0x6510));
                        c.lwc1(2, bsi_ea(c, GP, -0x6d84));
                        c.g[S1][0] = 0x4a0000;
                        c.swc1(6, bsi_ea(c, A0, 0xc));
                        c.g[S3][0] = 0x4a0000;
                        c.lwc1(1, bsi_ea(c, GP, -0x6d80));
                        c.g[T5][0] = sx(bsi_ea(c, S3, 0x6700));
                        c.swc1(2, bsi_ea(c, V0, 0xc));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(1, bsi_ea(c, S1, 0x6540));
                        c.g[T7][0] = sx(bsi_ea(c, A0, 0x6710));
                        c.swc1(0, bsi_ea(c, S7, 0x64e0));
                        c.g[S3][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S5, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64f0));
                        c.g[S1][0] = sx(bsi_ea(c, S3, 0x6720));
                        c.g[A0][0] = 0x4a0000;
                        c.g[S7][0] = 0x4a0000;
                        c.g[S3][0] = sx(bsi_ea(c, A0, 0x6740));
                        c.swc1(6, bsi_ea(c, S6, 0xc));
                        c.g[A0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S7, 0x6500));
                        c.g[A0][0] = sx(bsi_ea(c, A0, 0x6500));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(6, bsi_ea(c, A0, 0xc));
                        c.swc1(0, bsi_ea(c, S7, 0x6510));
                        c.g[A0][0] = sx(bsi_ea(c, GP, 0x1bc8));
                        c.lwc1(8, bsi_ea(c, GP, -0x6d7c));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(7, bsi_ea(c, GP, -0x6d78));
                        c.swc1(0, bsi_ea(c, S7, 0x6520));
                        c.swc1(8, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, V0, 0x8));
                        c.lwc1(5, bsi_ea(c, GP, -0x6d74));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x6530));
                        c.lwc1(4, bsi_ea(c, GP, -0x6d70));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(3, bsi_ea(c, GP, -0x6d6c));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6500));
                        c.swc1(5, bsi_ea(c, S4, 0x8));
                        c.sw(bsi_ea(c, S4, 0xc), 0u64);
                        c.swc1(4, bsi_ea(c, V1, 0x4));
                        c.swc1(3, bsi_ea(c, V1, 0x8));
                        c.sw(bsi_ea(c, V1, 0xc), 0u64);
                        c.g[V1][0] = 0x4a0000;
                        c.lwc1(2, bsi_ea(c, GP, -0x6d68));
                        c.swc1(0, bsi_ea(c, V1, 0x6550));
                        c.swc1(0, bsi_ea(c, T3, 0xc));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x6560));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6510));
                        c.swc1(0, bsi_ea(c, S5, 0x4));
                        c.swc1(0, bsi_ea(c, S5, 0x8));
                        c.swc1(6, bsi_ea(c, S6, 0x4));
                        c.swc1(0, bsi_ea(c, S6, 0x8));
                        c.swc1(6, bsi_ea(c, V0, 0x4));
                        c.g[S6][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6560));
                        c.swc1(6, bsi_ea(c, V1, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x65a0));
                        c.swc1(0, bsi_ea(c, S4, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T3, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T3, 0x8));
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.lwc1(1, bsi_ea(c, GP, -0x6d64));
                        c.swc1(2, bsi_ea(c, A3, 0xc));
                        c.lwc1(3, bsi_ea(c, GP, -0x6d60));
                        c.swc1(1, bsi_ea(c, S7, 0xc));
                        c.swc1(3, bsi_ea(c, S6, 0x8));
                        c.sw(bsi_ea(c, S6, 0xc), 0u64);
                        c.lwc1(4, bsi_ea(c, GP, -0x6d5c));
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6d58));
                        c.g[S7][0] = c.g[S6][0];
                        c.swc1(0, bsi_ea(c, V0, 0x6570));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x6590));
                        c.swc1(4, bsi_ea(c, A2, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(5, bsi_ea(c, A2, 0x8));
                        c.sw(bsi_ea(c, A2, 0xc), 0u64);
                        c.lwc1(6, bsi_ea(c, GP, -0x6d54));
                        c.lwc1(7, bsi_ea(c, GP, -0x6d50));
                        c.swc1(0, bsi_ea(c, V1, 0x6580));
                        c.swc1(6, bsi_ea(c, A3, 0x4));
                        c.g[V1][0] = c.g[V0][0];
                        c.swc1(7, bsi_ea(c, A3, 0x8));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x65a0));
                        c.lwc1(8, bsi_ea(c, GP, -0x6d4c));
                        c.swc1(0, bsi_ea(c, S6, 0x6590));
                        c.swc1(8, bsi_ea(c, S7, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0xc));
                        c.lwc1(9, bsi_ea(c, GP, -0x6d48));
                        c.g[S7][0] = c.g[S6][0];
                        c.swc1(0, bsi_ea(c, V0, 0x65a0));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x65b0));
                        c.swc1(9, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S6, 0x65b0));
                        c.g[V1][0] = c.g[V0][0];
                        c.swc1(0, bsi_ea(c, S7, 0xc));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x65c0));
                        c.swc1(0, bsi_ea(c, V0, 0x65c0));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(11, bsi_ea(c, GP, -0x6d44));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(10, bsi_ea(c, GP, -0x6d40));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6590));
                        c.swc1(0, bsi_ea(c, S6, 0x65d0));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(10, bsi_ea(c, T0, 0x4));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x65a0));
                        c.swc1(11, bsi_ea(c, T0, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T0, 0xc), 0u64);
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x65b0));
                        c.lwc1(12, bsi_ea(c, GP, -0x6d3c));
                        c.swc1(0, bsi_ea(c, S7, 0x65e0));
                        c.swc1(12, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x65c0));
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S6, 0x8));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x65e0));
                        c.swc1(0, bsi_ea(c, S7, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x4));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6640));
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(4, bsi_ea(c, GP, -0x6d38));
                        c.swc1(0, bsi_ea(c, V0, 0xc));
                        c.lwc1(3, bsi_ea(c, GP, -0x6d34));
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(4, bsi_ea(c, V1, 0x6620));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x65f0));
                        c.lwc1(2, bsi_ea(c, GP, -0x6d30));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(3, bsi_ea(c, T1, 0xc));
                        c.lwc1(1, bsi_ea(c, GP, -0x6d2c));
                        c.swc1(2, bsi_ea(c, S6, 0xc));
                        c.swc1(1, bsi_ea(c, T2, 0xc));
                        c.g[S6][0] = c.g[V1][0];
                        c.lwc1(5, bsi_ea(c, GP, -0x6d28));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6600));
                        c.swc1(0, bsi_ea(c, S7, 0x65f0));
                        c.swc1(5, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.sw(bsi_ea(c, V0, 0xc), 0u64);
                        c.swc1(0, bsi_ea(c, V1, 0x6600));
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S6, 0xc));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6610));
                        c.swc1(0, bsi_ea(c, S7, 0x6610));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, V0, 0xc), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6620));
                        c.sw(bsi_ea(c, V1, 0xc), 0u64);
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(6, bsi_ea(c, GP, -0x6d24));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(7, bsi_ea(c, GP, -0x6d20));
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S6, 0x6630));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6640));
                        c.swc1(6, bsi_ea(c, T1, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, T1, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(8, bsi_ea(c, GP, -0x6d1c));
                        c.swc1(0, bsi_ea(c, S7, 0x6640));
                        c.swc1(8, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(9, bsi_ea(c, GP, -0x6d18));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x6650));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x65f0));
                        c.swc1(9, bsi_ea(c, T4, 0x8));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T4, 0xc), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6600));
                        c.lwc1(10, bsi_ea(c, GP, -0x6d14));
                        c.swc1(0, bsi_ea(c, S6, 0x6660));
                        c.swc1(10, bsi_ea(c, T2, 0x4));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x6670));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6610));
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x4));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x6620));
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6640));
                        c.sw(bsi_ea(c, S6, 0x8), 0u64);
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S7, 0x4), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6670));
                        c.sw(bsi_ea(c, S7, 0x8), 0u64);
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6680));
                        c.swc1(0, bsi_ea(c, T4, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T2, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x66c0));
                        c.swc1(0, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(2, bsi_ea(c, GP, -0x6d10));
                        c.swc1(12, bsi_ea(c, S6, 0xc));
                        c.lwc1(1, bsi_ea(c, GP, -0x6d0c));
                        c.swc1(2, bsi_ea(c, S7, 0xc));
                        c.swc1(1, bsi_ea(c, S2, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0xc));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x6680));
                        c.g[S6][0] = c.g[V1][0];
                        c.swc1(0, bsi_ea(c, V1, 0x6690));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6690));
                        c.g[V0][0] = c.g[S7][0];
                        c.sw(bsi_ea(c, S6, 0xc), 0u64);
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x66a0));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x66a0));
                        c.sw(bsi_ea(c, V0, 0xc), 0u64);
                        c.g[S6][0] = c.g[V1][0];
                        c.swc1(0, bsi_ea(c, V1, 0x66b0));
                        c.g[S7][0] = 0x4a0000;
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x66b0));
                        c.lwc1(3, bsi_ea(c, GP, -0x6d08));
                        c.sw(bsi_ea(c, S6, 0xc), 0u64);
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S7, 0x66c0));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x66c0));
                        c.swc1(3, bsi_ea(c, V0, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.lwc1(4, bsi_ea(c, GP, -0x6d04));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x66d0));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(4, bsi_ea(c, S2, 0x8));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6d00));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6670));
                        c.swc1(0, bsi_ea(c, S6, 0x66e0));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(5, bsi_ea(c, S0, 0x8));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6680));
                        c.sw(bsi_ea(c, S0, 0xc), 0u64);
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(6, bsi_ea(c, GP, -0x6cfc));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6690));
                        c.lwc1(7, bsi_ea(c, GP, -0x6cf8));
                        c.swc1(0, bsi_ea(c, S7, 0x66f0));
                        c.swc1(6, bsi_ea(c, T6, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, T6, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x66a0));
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(12, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x66b0));
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S6, 0x8), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x66c0));
                        c.swc1(0, bsi_ea(c, S7, 0x4));
                        c.g[S6][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S7, 0x8), 0u64);
                        c.swc1(0, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.sw(bsi_ea(c, V0, 0x8), 0u64);
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.swc1(0, bsi_ea(c, S2, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S0, 0x4));
                        c.g[S2][0] = 0xffffffffffffffff;
                        c.sw(bsi_ea(c, T6, 0xc), 0u64);
                        c.lwc1(2, bsi_ea(c, GP, -0x6cf4));
                        c.sb(bsi_ea(c, GP, 0x1bc0), c.g[S2][0]);
                        c.lwc1(1, bsi_ea(c, GP, -0x6cf0));
                        c.swc1(2, bsi_ea(c, T7, 0xc));
                        c.g[V0][0] = c.g[S2][0] & 0xff;
                        c.swc1(1, bsi_ea(c, S1, 0xc));
                        c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                        c.lwc1(4, bsi_ea(c, GP, -0x6cec));
                        c.lwc1(3, bsi_ea(c, GP, -0x6ce8));
                        c.swc1(0, bsi_ea(c, S6, 0x6700));
                        c.swc1(4, bsi_ea(c, T5, 0x4));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(3, bsi_ea(c, T5, 0x8));
                        c.sw(bsi_ea(c, T5, 0xc), 0u64);
                        c.lwc1(8, bsi_ea(c, GP, -0x6ce4));
                        c.lwc1(5, bsi_ea(c, GP, -0x6ce0));
                        c.swc1(0, bsi_ea(c, S7, 0x6710));
                        c.swc1(8, bsi_ea(c, T7, 0x4));
                        c.swc1(5, bsi_ea(c, T7, 0x8));
                        c.lwc1(6, bsi_ea(c, GP, -0x6cdc));
                        c.lwc1(7, bsi_ea(c, GP, -0x6cd8));
                        c.swc1(0, bsi_ea(c, V1, 0x6720));
                        c.swc1(6, bsi_ea(c, S1, 0x4));
                        c.swc1(7, bsi_ea(c, S1, 0x8));
                        c.g[S1][0] = 0x4a0000;
                        c.g[S2][0] = c.g[S1][0];
                        c.swc1(0, bsi_ea(c, S1, 0x6730));
                        c.g[S2][0] = sx(bsi_ea(c, S2, 0x6730));
                        c.sw(bsi_ea(c, S2, 0xc), 0u64);
                        c.swc1(0, bsi_ea(c, S6, 0x6740));
                        c.swc1(0, bsi_ea(c, S3, 0x4));
                        c.sw(bsi_ea(c, S3, 0xc), 0u64);
                        c.sw(bsi_ea(c, GP, 0x1bc0), c.g[V0][0]);
                        c.swc1(0, bsi_ea(c, S2, 0x8));
                        c.sw(bsi_ea(c, S2, 0x4), 0u64);
                        c.sw(bsi_ea(c, S3, 0x8), 0u64);
                        call_ptr(c, 0x003182d8, 0x0015b83c)?;
                        c.g[A0][0] = 0x4a0000;
                        c.g[A0][0] = sx(bsi_ea(c, A0, 0x6750));
                        call_ptr(c, 0x0014c620, 0x0015b848)?;
                        c.g[V0][0] = 0x4a0000;
                        c.g[T2][0] = 0xffffffffffffffff;
                        c.g[T0][0] = 0xffffffffffffffff;
                        c.g[T3][0] = 0x9b50;
                        c.g[V1][0] = 0x2;
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6ca8));
                        // Character profiles: fill the slots with (0xffff, 0) halfword pairs.
                        'l_15b860: loop {
                            c.cycles += 4;
                            c.g[T1][0] = sx(bsi_ea(c, V1, -0x1));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V1][0] = 0x9;
                            'l_15b870: loop {
                                c.cycles += 4;
                                c.g[A2][0] = sx(bsi_ea(c, V1, -0x1));
                                c.g[A3][0] = sx(bsi_ea(c, A1, 0xf88));
                                c.g[V1][0] = sx(bsi_ea(c, A1, 0x290));
                                c.g[A0][0] = 0x20c;
                                'l_15b880: loop {
                                    c.cycles += 7;
                                    c.sh(bsi_ea(c, V1, 0), c.g[T2][0]);
                                    c.g[A0][0] = sx(bsi_ea(c, A0, -0x1));
                                    c.sh(bsi_ea(c, V1, 0x2), 0u64);
                                    let t = c.g[A0][0] != c.g[T0][0];
                                    c.g[V1][0] = sx(bsi_ea(c, V1, 0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0015b880);
                                        }
                                        continue 'l_15b880;
                                    } else {
                                        c.cycles += 4;
                                        c.sw(bsi_ea(c, A1, 0x288), 0u64);
                                        c.g[V1][0] = c.g[A2][0];
                                        let t = c.g[V1][0] != c.g[T0][0];
                                        c.g[A1][0] = c.g[A3][0];
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0015b870);
                                            }
                                            continue 'l_15b870;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V1][0] = c.g[T1][0];
                                            let t = c.g[V1][0] != c.g[T0][0];
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[T3][0] as u32));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0015b860);
                                                }
                                                continue 'l_15b860;
                                            } else {
                                                c.cycles += 84;
                                                c.g[A0][0] = 0x4c0000;
                                                c.g[A0][0] = sx(bsi_ea(c, A0, 0x3e98));
                                                call_ptr(c, 0x0015a778, 0x0015b8c4)?;
                                                c.g[A0][0] = 0x4c0000;
                                                c.g[A0][0] = sx(bsi_ea(c, A0, 0x3ef0));
                                                call_ptr(c, 0x0015a488, 0x0015b8d0)?;
                                                c.g[S3][0] = 0xffffffffffffffff;
                                                call_ptr(c, 0x0015c228, 0x0015b8d8)?;
                                                c.g[V0][0] = 0x4c0000;
                                                c.g[V1][0] = 0x2;
                                                c.g[A1][0] = 0x4;
                                                c.g[A2][0] = 0x5;
                                                c.g[S4][0] = sx(bsi_ea(c, V0, 0x4090));
                                                c.sb(bsi_ea(c, GP, 0x1bf0), c.g[V1][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c00), c.g[A1][0]);
                                                c.g[V0][0] = 0x1;
                                                c.sb(bsi_ea(c, GP, 0x1c08), c.g[A2][0]);
                                                c.g[A0][0] = 0x3;
                                                c.g[T1][0] = 0x7;
                                                c.g[T2][0] = 0x8;
                                                c.g[T3][0] = 0x9;
                                                c.g[T0][0] = 0xa;
                                                c.sb(bsi_ea(c, GP, 0x1be8), c.g[V0][0]);
                                                c.g[A3][0] = 0x6;
                                                c.sb(bsi_ea(c, GP, 0x1bf8), c.g[A0][0]);
                                                c.g[T7][0] = 0xb;
                                                c.sb(bsi_ea(c, GP, 0x1c18), c.g[T1][0]);
                                                c.g[T6][0] = 0xc;
                                                c.sb(bsi_ea(c, GP, 0x1c20), c.g[T2][0]);
                                                c.g[T1][0] = c.g[V0][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1c28), c.g[T3][0]);
                                                c.g[T2][0] = c.g[A1][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1c30), c.g[T0][0]);
                                                c.g[T3][0] = c.g[V1][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1c10), c.g[A3][0]);
                                                c.g[T0][0] = c.g[A2][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1be0), 0u64);
                                                c.g[T4][0] = c.g[A0][0] & 0xff;
                                                c.g[A2][0] = c.lbu(bsi_ea(c, GP, 0x1c18));
                                                c.g[A3][0] = c.g[A3][0] & 0xff;
                                                c.g[A1][0] = c.lbu(bsi_ea(c, GP, 0x1c20));
                                                c.g[T1][0] = c.g[T1][0] | 0x100;
                                                c.g[V1][0] = c.lbu(bsi_ea(c, GP, 0x1c28));
                                                c.g[T3][0] = c.g[T3][0] | 0x200;
                                                c.g[A0][0] = c.lbu(bsi_ea(c, GP, 0x1be0));
                                                c.g[T4][0] = c.g[T4][0] | 0x300;
                                                c.g[V0][0] = c.lbu(bsi_ea(c, GP, 0x1c30));
                                                c.g[V1][0] = c.g[V1][0] | 0x900;
                                                c.g[T2][0] = c.g[T2][0] | 0x400;
                                                c.g[T0][0] = c.g[T0][0] | 0x500;
                                                c.g[A3][0] = c.g[A3][0] | 0x600;
                                                c.g[A2][0] = c.g[A2][0] | 0x700;
                                                c.g[A1][0] = c.g[A1][0] | 0x800;
                                                c.sw(bsi_ea(c, GP, 0x1be0), c.g[A0][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c28), c.g[V1][0]);
                                                c.g[V0][0] = c.g[V0][0] | 0xa00;
                                                c.sw(bsi_ea(c, GP, 0x1be8), c.g[T1][0]);
                                                c.g[T5][0] = 0xd;
                                                c.sw(bsi_ea(c, GP, 0x1bf0), c.g[T3][0]);
                                                c.sw(bsi_ea(c, GP, 0x1bf8), c.g[T4][0]);
                                                c.g[S2][0] = 0xffffffffffffffff;
                                                c.sw(bsi_ea(c, GP, 0x1c00), c.g[T2][0]);
                                                c.g[S0][0] = sx(bsi_ea(c, S4, 0x290));
                                                c.sw(bsi_ea(c, GP, 0x1c08), c.g[T0][0]);
                                                c.g[S1][0] = 0x20c;
                                                c.sw(bsi_ea(c, GP, 0x1c10), c.g[A3][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c18), c.g[A2][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c20), c.g[A1][0]);
                                                c.sw(bsi_ea(c, GP, 0x1bdc), 0u64);
                                                c.sw(bsi_ea(c, GP, 0x1bd8), 0u64);
                                                c.sw(bsi_ea(c, GP, 0x1c30), c.g[V0][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c38), c.g[T7][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c40), c.g[T6][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c48), c.g[T5][0]);
                                                c.g[V0][0] = c.g[T7][0] & 0xff;
                                                c.g[V1][0] = c.g[T6][0] & 0xff;
                                                c.g[V0][0] = c.g[V0][0] | 0xb00;
                                                c.g[A0][0] = c.g[T5][0] & 0xff;
                                                c.g[V1][0] = c.g[V1][0] | 0xc00;
                                                c.sw(bsi_ea(c, GP, 0x1c38), c.g[V0][0]);
                                                c.g[A0][0] = c.g[A0][0] | 0xd00;
                                                c.sw(bsi_ea(c, GP, 0x1c40), c.g[V1][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c48), c.g[A0][0]);
                                                'l_15ba08: loop {
                                                    c.cycles += 7;
                                                    c.sh(bsi_ea(c, S0, 0), c.g[S3][0]);
                                                    c.g[S1][0] = sx(bsi_ea(c, S1, -0x1));
                                                    c.sh(bsi_ea(c, S0, 0x2), 0u64);
                                                    let t = c.g[S1][0] != c.g[S2][0];
                                                    c.g[S0][0] = sx(bsi_ea(c, S0, 0x4));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0015ba08);
                                                        }
                                                        continue 'l_15ba08;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.sw(bsi_ea(c, S4, 0x288), 0u64);
                                                        break 'b_15bacc;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // param_1 == 0: destroy the back end's singletons and the bolt DB.
                'b_15bab4: {
                    c.cycles += 14;
                    c.g[A0][0] = 0x4c0000;
                    c.g[A1][0] = 0x2;
                    c.g[S6][0] = sx(bsi_ea(c, GP, 0x1bc8));
                    c.g[A0][0] = sx(bsi_ea(c, A0, 0x4090));
                    call_ptr(c, 0x00151368, 0x0015ba40)?;
                    c.g[S5][0] = 0x4a0000;
                    c.g[A0][0] = 0x4c0000;
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx(bsi_ea(c, A0, 0x3e98));
                    call_ptr(c, 0x0015a7d0, 0x0015ba54)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = sx(bsi_ea(c, V0, 0x6ca8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x4c0000;
                    if t {
                        break 'b_15bab4;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx(bsi_ea(c, V0, 0x3e98));
                        let t = c.g[V1][0] == c.g[S1][0];
                        c.g[S4][0] = c.g[V1][0];
                        if t {
                            break 'b_15bab4;
                        } else {
                            c.cycles += 4;
                            c.g[S2][0] = 0xffffffffffff0000;
                            c.g[S2][0] = c.g[S2][0] | 0x64b0;
                            c.g[S3][0] = 0x9b50;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            'l_15ba80: loop {
                                'b_15baac: {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == 0u64;

                                    if t {
                                        break 'b_15baac;
                                    } else {
                                        c.cycles += 3;
                                        c.g[S0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        let t = c.g[S1][0] == c.g[S0][0];

                                        if t {
                                            break 'b_15baac;
                                        } else {
                                            c.cycles += 1;
                                            c.g[S0][0] = sx(bsi_ea(c, S0, -0xf88));
                                            'l_15ba98: loop {
                                                c.cycles += 5;
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x00151368, 0x0015baa4)?;
                                                if c.g[S1][0] != c.g[S0][0] {
                                                    c.g[S0][0] = sx(bsi_ea(c, S0, -0xf88));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0015ba98);
                                                    }
                                                    continue 'l_15ba98;
                                                } else {
                                                    break 'b_15baac;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] != c.g[S4][0];
                                c.g[S1][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0015ba80);
                                    }
                                    continue 'l_15ba80;
                                } else {
                                    break 'b_15bab4;
                                }
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = sx(bsi_ea(c, S5, 0x6750));
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014c658, 0x0015bac0)?;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x00318260, 0x0015bacc)?;
                break 'b_15bacc;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RewardsFile_PollLoad(RewardsFile *self)` (0x15aec0): when the file
/// is not parsed yet and the loader's handle is ready, parse it; the status
/// word becomes -1 once it is done.
pub fn rewards_file_poll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        let parsed = (c.g[A2][0] as i64) < 0;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if !parsed {
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0019_e238, 0x0015_aef4)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 12;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A1][0] = 0;
                call_ptr(c, 0x0019_e330, 0x0015_af08)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0015_a818, 0x0015_af14)?;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0019_dc20, 0x0015_af24)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                let done = c.g[V0][0];
                Addr(c.g[S1][0] as u32).sw(c, done);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_AssignGlobalString1bc8(void)` (0x15bc10): the global string at
/// `gp - 0x...1bc8` is passed to the string-assign helper.
pub fn be_assign_global_string_1bc8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1bc8));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_84e8, 0x0015_bc24)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_CopyGlobalString1bc8(void *dst)` (0x15bc30): copies the global
/// string at `gp + 0x1bc8` into `dst`, returns `dst`.
pub fn be_copy_global_string_1bc8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1bc8));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7fe8, 0x0015_bc48)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_SetNameEntry535617(int index, int a1, int a2)` (0x15c188): the
/// name slot `0x530000 + 0x5617 + index * 0x40`, through the setter.
pub fn be_set_name_entry_535617(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[A0][0] = 0x53_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 6);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5617));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0015_c1a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_SetString534d48(void)` (0x15c338): the global string at
/// `0x530000 + 0x4d48` through the string setter.
pub fn be_set_string_534d48(c: &mut Cpu) {
    set_global_string(c, 0x4d48, 0x0015_c34c);
}

/// `void BE_SetString534dc0(void)` (0x15c368): as `534d48`, at `0x4dc0`.
pub fn be_set_string_534dc0(c: &mut Cpu) {
    set_global_string(c, 0x4dc0, 0x0015_c37c);
}

/// `void BE_SetString534e38(void)` (0x15c398): as `534d48`, at `0x4e38`.
pub fn be_set_string_534e38(c: &mut Cpu) {
    set_global_string(c, 0x4e38, 0x0015_c3ac);
}

/// The shared body of the three `BE_SetString*` wrappers: `setter(0x530000 + off)`.
pub(crate) fn set_global_string(c: &mut Cpu, off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x53_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
        call_ptr(c, 0x0041_66f4, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BEScoreInt_DestroyInstance` (0x15c738), `cBEScoreInterface_purgeInterface`.
//
// `void BEScoreInt_DestroyInstance(void)`: `delete` the score interface
// singleton kept at `gp - 0x1ea8`, then clear that slot.

pub fn be_score_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let slot = (c.g[GP][0] as u32).wrapping_sub(0x1ea8);
        c.g[A0][0] = Addr(slot).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0015_c74c)?;
        frame.restore_ra(c, 0);
        Addr(slot).sw(c, 0);
        frame.close(c);
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
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::bx::bxstringctor::tests::game;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;

    #[test]
    fn algorithm_list_ctor_matches() {
        check(
            ssx3_ee::s01::f_0015c930,
            algorithm_list_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x15c9 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn algorithm_list_dtor_matches() {
        check(
            ee::s01::f_0015c940,
            algorithm_list_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xc940, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA, 0);
            },
        );
    }

    #[test]
    fn be_global_ctors_dtors_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_0015c8f0 as fn(&mut Cpu),
                be_global_ctors as fn(&mut Cpu),
                0xc8f0,
            ),
            (ssx3_ee::s01::f_0015c910, be_global_dtors, 0xc910),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }

    #[test]
    fn be_clear_option_state_5308b8_matches() {
        check(
            ee::s01::f_0015c228__generated,
            be_clear_option_state_5308b8,
            60,
            &|c, seed| {
                let mut r = game(c, seed, 0x15c228, 1);
                for i in 0..8u32 {
                    c.bus.write32(0x0053_08b8 + 4 * i, r.next_u32());
                }
            },
        );
    }

    pub(crate) fn call_setup(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const RET: u64 = 0x0012_3450;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        call_setup(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1);
    }

    #[test]
    fn c_bx_string_assign_cstr_matches() {
        check(
            ee::s10::f_003184e8__generated,
            c_bx_string_assign_cstr,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x3184e8, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                let data = DATA + 0x300;
                let text = DATA + 0x400;
                clear(c, this, 0x40);
                clear(c, data - 0x10, 0x40);
                clear(c, text, 0x40);
                c.bus.write32(this, data);
                c.bus.write32(data - 0xc, 1);
                c.bus.write32(data - 4, 0x40);
                c.bus.write8(text, b'h');
                c.bus.write8(text + 1, b'i');
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(text) };
            },
        );
    }

    #[test]
    fn rewards_file_unload_matches() {
        strict(
            ee::s01::f_0015ae00__generated,
            rewards_file_unload,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15ae, 8);
                let pool = DATA + 0x20_0000;
                c.bus.write32(GAME_GP + 0x2a70, pool);
                c.bus.write32(pool + 0x800, 0);
                c.bus.write32(GAME_GP + 0xdd8, 1);
                let this = DATA;
                c.bus.write32(this, 0xffff_ffff);
                c.bus
                    .write32(this + 4, if seed % 2 == 0 { 0 } else { pool + 0x100 });
                c.bus.write32(GAME_GP - 0x848, DATA + 0x1000);
                c.g[A0][0] = sx(this);
            },
        );
    }

    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;
    use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;

    /// The init path (`param_1 != 0`, `param_2 == 0xffff`) and the early exit
    /// (`param_2 != 0xffff`). The destroy path (`param_1 == 0`) is not
    /// compared: its outer loop steps the profile pointer down by 0x9b50 from
    /// 0x4ba348 and only stops at 0x4a6ca8, which it never hits on zeroed RAM.
    #[test]
    fn be_static_init_destroy_matches() {
        check(
            ssx3_ee::s01::f_0015b028__generated,
            be_static_init_destroy,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x15b028, 0);
                if seed % 4 == 3 {
                    c.g[A0][0] = (seed % 2) as u64;
                    c.g[A1][0] = 0x1234 + seed as u64;
                } else {
                    c.g[A0][0] = 1 + (seed % 3) as u64;
                    c.g[A1][0] = 0xffff;
                }
            },
        );
    }

    #[test]
    fn rewards_file_poll_load_matches() {
        check(
            ee::s01::f_0015aec0__generated,
            rewards_file_poll_load,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x15aec0, 0);
                // The loader's world: game at +0x7c, shape manager at +0x1a70.
                let world = DATA + 0x1_0000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, DATA + 0x2_0000);
                c.bus.write32(DATA + 0x2_0000 + 0x1a70, seed % 2);
                let file = DATA + 0x3_0000;
                c.bus
                    .write32(file, if seed % 3 == 0 { u32::MAX } else { seed % 4 });
                c.g[A0][0] = sx(file);
                c.pc = 0x0015_aec0;
            },
        );
    }

    #[test]
    fn be_assign_global_string_1bc8_matches() {
        check(
            ee::s01::f_0015bc10__generated,
            be_assign_global_string_1bc8,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x15bc10, 0);
                c.pc = 0x0015_bc10;
            },
        );
    }

    #[test]
    fn be_copy_global_string_1bc8_matches() {
        check(
            ee::s01::f_0015bc30__generated,
            be_copy_global_string_1bc8,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x15bc30, 0);
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 8));
                c.pc = 0x0015_bc30;
            },
        );
    }

    #[test]
    fn be_set_name_entry_535617_matches() {
        check(
            ee::s01::f_0015c188__generated,
            be_set_name_entry_535617,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x15c188, 0);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = sx(DATA + seed * 8);
                c.pc = 0x0015_c188;
            },
        );
    }

    #[test]
    fn be_set_string_534d48_matches() {
        check(
            ee::s01::f_0015c338__generated,
            be_set_string_534d48,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x15c338, 0);
                c.pc = 0x0015_c338;
            },
        );
    }

    #[test]
    fn be_set_string_534dc0_matches() {
        check(
            ee::s01::f_0015c368__generated,
            be_set_string_534dc0,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x15c368, 0);
                c.pc = 0x0015_c368;
            },
        );
    }

    #[test]
    fn be_set_string_534e38_matches() {
        check(
            ee::s01::f_0015c398__generated,
            be_set_string_534e38,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x15c398, 0);
                c.pc = 0x0015_c398;
            },
        );
    }

    #[test]
    fn be_score_int_destroy_instance_matches() {
        oc430_install_fakes();
        check(
            ee::s01::f_0015c738__generated,
            be_score_int_destroy_instance,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_c738, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0015_c738;
                let obj = if seed % 2 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(GAME_GP.wrapping_sub(0x1ea8), obj);
                c.bus.write32(DATA + 0x1000, r.next_u32());
            },
        );
    }
}
