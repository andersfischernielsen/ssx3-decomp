//! `src/be/beintsavefile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintreward::alloc_named;
pub(crate) use crate::game::be::beintreward::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::fe::festateload::glo;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_2638,
        be_save_int_get_instance,
        "BESaveInt_GetInstance",
    ),
    (
        0x0015_2688,
        be_save_int_get_save_buffer,
        "BESaveInt_GetSaveBuffer",
    ),
    (
        0x0015_26d0,
        besave_int_get_save_buffer_end,
        "BESaveInt_GetSaveBufferEnd",
    ),
    (
        0x0015_2700,
        be_save_int_free_save_buffer,
        "BESaveInt_FreeSaveBuffer",
    ),
    (
        0x0015_2728,
        besave_int_release_save_buffer,
        "BESaveInt_ReleaseSaveBuffer",
    ),
    (
        0x0015_2ba8,
        be_save_int_get_profile_save_size,
        "BESaveInt_GetProfileSaveSize",
    ),
    (0x0015_3050, be_save_int_destroy, "BESaveInt_Destroy"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

// ---------------------------------------------------------------------------
// Save singletons.

pub(crate) const SAVE_GP: u32 = 0x1ed4;

pub(crate) const SAVE_BUFFER_GP: u32 = 0x1ecc;

/// The separately freed save block (`gp - 0x1ed0`).
pub(crate) const SAVE_FREE_GP: u32 = 0x1ed0;

/// `BESaveInt_GetInstance()` (0x152638): the `BESaveInt` singleton, allocated
/// with `EA_MemAllocNamed(0x10, "BESaveInt", 0, 0)`.
pub fn be_save_int_get_instance(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(SAVE_GP);
        let inst = Addr(global).lw(c);
        c.g[V0][0] = inst;
        frame.save_ra(c, 0);
        if inst == 0 {
            c.cycles += 12;
            let block = alloc_named(c, 0x10, 0x0045_a7e0, 0x0015_2660)?;
            c.g[V1][0] = 0x0046_0000;
            Addr(block as u32 + 8).sw(c, 0);
            c.g[V1][0] = 0x0045_ad08;
            Addr(block as u32 + 0xc).sw(c, c.g[V1][0]);
            Addr(global).sw(c, block);
            c.sw(Addr((c.g[GP][0] as u32).wrapping_sub(0x1ec8)).0, 0);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(global).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BESaveInt_GetSaveBuffer()` (0x152688): the 0x80000-byte save buffer,
/// allocated (flags 0x100) on first use; sets the "buffer active" flag.
pub fn be_save_int_get_save_buffer(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let buffer = (c.g[GP][0] as u32).wrapping_sub(SAVE_BUFFER_GP);
        c.g[V0][0] = 1;
        c.g[V1][0] = Addr(buffer).lw(c);
        frame.save_ra(c, 0);
        let present = c.g[V1][0] != 0;
        c.sw(Addr((c.g[GP][0] as u32).wrapping_sub(0x1ec8)).0, c.g[V0][0]);
        if !present {
            c.cycles += 7;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x8_0000;
            c.g[A1][0] = 0x0045_a7f0;
            c.g[A2][0] = 0x100;
            c.g[RA][0] = 0x0015_26b8;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0015_26b8)?;
            c.sw(Addr(buffer).0, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(buffer).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BESaveInt_FreeSaveBuffer()` (0x152700): free the save buffer and clear
/// its global.
pub fn be_save_int_free_save_buffer(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let buffer = (c.g[GP][0] as u32).wrapping_sub(SAVE_FREE_GP);
        c.g[A0][0] = Addr(buffer).lw(c);
        let absent = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 2;
            c.g[RA][0] = 0x0015_2718;
            call_ptr(c, 0x0031_7e98, 0x0015_2718)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(Addr(buffer).0, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BESaveInt_Destroy()` (0x153050): delete the singleton and free the save
/// buffer.
pub fn be_save_int_destroy(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let global = (c.g[GP][0] as u32).wrapping_sub(SAVE_GP);
        c.g[A0][0] = Addr(global).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0015_3064)?;
        let buffer = (c.g[GP][0] as u32).wrapping_sub(SAVE_FREE_GP);
        c.g[A0][0] = Addr(buffer).lw(c);
        let absent = c.g[A0][0] == 0;
        c.sw(Addr(global).0, 0);
        if !absent {
            c.cycles += 2;
            c.g[RA][0] = 0x0015_3078;
            call_ptr(c, 0x0031_7e98, 0x0015_3078)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(Addr(buffer).0, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BESaveInt_GetProfileSaveSize(void)` (0x152ba8): the size in bytes of
/// the "save int" (profile settings) blob.
pub fn be_save_int_get_profile_save_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x9b61;
    abi::ret(c);
}

/// `int BESaveInt_GetSaveBufferEnd(void)` (0x1526d0): the save buffer base
/// (`D_004A1224`) plus the library's save-data size.
pub fn besave_int_get_save_buffer_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0015_26e0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_LIBRARY_GET_SAVE_DATA_SIZE.call(c, 0x0015_26e8)?;
        c.g[V1][0] = Addr(glo(c, g::SAVE_BUFFER)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.close(c);
        Ok(())
    });
}

/// `void BESaveInt_ReleaseSaveBuffer(void)` (0x152728): free the save buffer
/// (`D_004A1224`) and clear the buffer and its end.
pub fn besave_int_release_save_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(glo(c, g::SAVE_BUFFER)).lw(c);
        let buf = c.g[A0][0];
        frame.save_ra(c, 0);
        if buf != 0 {
            c.cycles += 2;
            call_ptr(c, callee::EA_MEM_FREE, 0x0015_2740)?;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(glo(c, g::SAVE_BUFFER)).sw(c, 0);
        Addr(glo(c, g::SAVE_BUFFER_END)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beintnetwork::tests::alloc_setup;
    pub(crate) use crate::game::be::beintnetwork::tests::MEM_MUTEX;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;

    #[test]
    fn be_save_score_match() {
        for (gen, hand, off, salt) in [
            (
                ssx3_ee::s01::f_00152638 as GuestFn,
                be_save_int_get_instance as GuestFn,
                0x1ed4u32,
                51u64,
            ),
            (
                ssx3_ee::s01::f_00152700,
                be_save_int_free_save_buffer,
                0x1ed0,
                52,
            ),
            (ssx3_ee::s01::f_00153050, be_save_int_destroy, 0x1ed4, 53),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 6);
                alloc_setup(c, &mut r);
                c.bus
                    .write32(GAME_GP - off, if seed % 2 == 0 { 0 } else { DATA + 0x400 });
                if seed % 3 == 0 {
                    c.bus.write32(GAME_GP - 0x1ed0, DATA + 0x800);
                }
            });
        }
        covered(
            ssx3_ee::s01::f_00152688,
            be_save_int_get_save_buffer,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 54, 6);
                alloc_setup(c, &mut r);
                c.bus.write32(
                    GAME_GP - 0x1ecc,
                    if seed % 2 == 0 { 0 } else { DATA + 0x400 },
                );
            },
        );
    }

    #[test]
    fn be_save_int_get_profile_save_size_matches() {
        check(
            ee::s01::f_00152ba8__generated,
            be_save_int_get_profile_save_size,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0015_2ba8, seed);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn besave_int_get_save_buffer_end_matches() {
        check(
            s01::f_001526d0__generated,
            besave_int_get_save_buffer_end,
            120,
            &|c, seed| {
                machine(c, seed, 0x1526d0, 2);
                globals(c);
                clear(c, DATA + 0x6_c000, 0x40);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1ecc),
                    0x0080_2000u32 + 0x40 * (seed & 7),
                );
                let _ = seed;
            },
        );
    }

    #[test]
    fn besave_int_release_save_buffer_matches() {
        check(
            s01::f_00152728__generated,
            besave_int_release_save_buffer,
            120,
            &|c, seed| {
                machine(c, seed, 0x152728, 1);
                clear(c, OBJ, 0x40);
                // A buffer to free, or none.
                let buf = if seed % 3 == 0 { 0 } else { OBJ };
                c.bus.write32(GAME_GP.wrapping_sub(0x1ecc), buf);
                c.bus.write32(GAME_GP.wrapping_sub(0x1ec8), 0x1234);
            },
        );
    }
}
