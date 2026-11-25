//! `src/be/beintsavefile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintreward::alloc_named;
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
    (
        0x0015_2de0,
        be_save_int_load_profile_save,
        "BESaveInt_LoadProfileSave",
    ),
    (
        0x0015_2948,
        be_save_int_get_system_save_size,
        "BESaveInt_GetSystemSaveSize",
    ),
    (
        0x0015_2758,
        be_save_int_build_system_save,
        "BESaveInt_BuildSystemSave",
    ),
    (
        0x0015_2950,
        be_save_int_build_profile_save,
        "BESaveInt_BuildProfileSave",
    ),
    (
        0x0015_2bb0,
        be_save_int_load_system_save,
        "BESaveInt_LoadSystemSave",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;

    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
}

/// `gp`-relative globals used here.
pub(crate) mod g {

    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
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

/// One `BE*Int_GetInstance()->onLoad()` of `BESaveInt_LoadProfileSave`: the
/// object returned by the call at `getter` has a vtable at `+0xc` whose
/// entry at `+0x10`/`+0x14` (`{delta, fn}`) is called with `this + delta`.
pub(crate) fn be_save_int_on_load(
    c: &mut Cpu,
    getter: u32,
    ret_getter: u32,
    ret_call: u32,
) -> Flow {
    call_ptr(c, getter, ret_getter)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let target = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_call)
}

/// `int BESaveInt_LoadProfileSave(BESaveInt* self, char* buf, int idx)`
/// (0x152de0): loads profile `idx` from a save buffer. The CRC16 (seed 0xfbea)
/// of the first `GetProfileSaveSize() - 4` bytes must equal the word stored
/// after them, else returns 0. Otherwise: remembers the ten per-sub-profile
/// `keep` words of the profile slot (stride 0xf88 from `0x4a6f30 + idx *
/// 0x9b50`), copies the 12-byte name (`strncpy` into `0x535b20 + idx * 0x1c`)
/// and the 0x9b40-byte profile block (an aligned `ld/sd` copy or an unaligned
/// `ldl/ldr` one, by the `(src | dst) & 7` test) from the buffer into
/// `0x4a6ca8 + idx * 0x9b50`, stores the flag byte at `0x535b31 + idx * 0x1c`,
/// calls the three interfaces' `onLoad`, and restores each sub-profile's `keep`
/// word and its equip flags (`BE_RestoreCharProfileEquipFlags`). Returns 1.
pub fn be_save_int_load_profile_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0015_2ba8, 0x0015_2e04)?; // BESaveInt_GetProfileSaveSize
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0xfbea;
        call_ptr(c, 0x003e_62d0, 0x0015_2e18)?; // crc16
        let sp0 = frame.at(c, 0x30);
        c.sw(sp0, 0);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.lwl(3, (c.g[S0][0] as u32).wrapping_add(3));
        c.lwr(3, c.g[S0][0] as u32);
        c.swl(3, sp0.wrapping_add(3));
        c.swr(3, sp0);
        c.g[V1][0] = c.lw(sp0);
        let mismatch = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !mismatch {
            c.cycles += 9;
            c.g[V1][0] = 0x9b50;
            c.g[S0][0] = 0x4a_0000;
            c.mult(0, V1, S2, V1);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x288));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 9;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            // keep[i] = profile[idx].sub[i].keep, i = 9..0 (into sp[0..9])
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf88));
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                let more = (c.g[A1][0] as i64) >= 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_2e60);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 17;
            c.g[V0][0] = 0x1c;
            c.g[A0][0] = 0x53_0000;
            c.mult(0, V0, S2, V0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b20));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0xc;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0041_6ce0, 0x0015_2ea0)?; // strncpy (name)
            c.g[V1][0] = 0x9b50;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6ca8));
            c.mult(0, V1, S2, V1);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = c.g[S1][0] | c.g[A0][0];
            c.g[V0][0] &= 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !aligned {
                c.cycles += 2;
                c.g[V0][0] = 0x9b40;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                loop {
                    c.cycles += 19;
                    let (a1, a0) = (c.g[A1][0] as u32, c.g[A0][0] as u32);
                    c.ldl(7, a1.wrapping_add(7));
                    c.ldr(7, a1);
                    c.ldl(8, a1.wrapping_add(0xf));
                    c.ldr(8, a1.wrapping_add(8));
                    c.ldl(3, a1.wrapping_add(0x17));
                    c.ldr(3, a1.wrapping_add(0x10));
                    c.ldl(6, a1.wrapping_add(0x1f));
                    c.ldr(6, a1.wrapping_add(0x18));
                    c.sdl(7, a0.wrapping_add(7));
                    c.sdr(7, a0);
                    c.sdl(8, a0.wrapping_add(0xf));
                    c.sdr(8, a0.wrapping_add(8));
                    c.sdl(3, a0.wrapping_add(0x17));
                    c.sdr(3, a0.wrapping_add(0x10));
                    c.sdl(6, a0.wrapping_add(0x1f));
                    c.sdr(6, a0.wrapping_add(0x18));
                    c.g[A1][0] = sx(a1.wrapping_add(0x20));
                    let more = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(a0.wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_2ec8);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x1c;
                    break;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x9b40;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                loop {
                    c.cycles += 11;
                    let (a1, a0) = (c.g[A1][0] as u32, c.g[A0][0] as u32);
                    c.g[A3][0] = c.ld(a1);
                    c.g[T0][0] = c.ld(a1.wrapping_add(8));
                    c.g[V1][0] = c.ld(a1.wrapping_add(0x10));
                    c.g[A2][0] = c.ld(a1.wrapping_add(0x18));
                    c.sd(a0, c.g[A3][0]);
                    c.sd(a0.wrapping_add(8), c.g[T0][0]);
                    c.sd(a0.wrapping_add(0x10), c.g[V1][0]);
                    c.sd(a0.wrapping_add(0x18), c.g[A2][0]);
                    c.g[A1][0] = sx(a1.wrapping_add(0x20));
                    let more = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(a0.wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_2f24);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0x1c;
                    break;
                }
            }
            // the last 16 bytes, then the flag byte
            c.cycles += 45;
            let (a1, a0) = (c.g[A1][0] as u32, c.g[A0][0] as u32);
            c.ldl(7, a1.wrapping_add(7));
            c.ldr(7, a1);
            c.ldl(8, a1.wrapping_add(0xf));
            c.ldr(8, a1.wrapping_add(8));
            c.sdl(7, a0.wrapping_add(7));
            c.sdr(7, a0);
            c.sdl(8, a0.wrapping_add(0xf));
            c.mult(0, V1, S2, V1);
            c.sdr(8, a0.wrapping_add(8));
            c.g[S0][0] = 0x9b50;
            c.g[V0][0] = 0x53_0000;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b31));
            c.g[A0][0] = c.lbu(c.g[S1][0] as u32);
            c.g[S3][0] = 9;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sb(c.g[V1][0] as u32, c.g[A0][0]);
            be_save_int_on_load(c, 0x0014_5970, 0x0015_2f9c, 0x0015_2fb0)?; // BENewPlayerInt
            be_save_int_on_load(c, 0x0014_7f78, 0x0015_2fb8, 0x0015_2fcc)?; // BECharInt
            be_save_int_on_load(c, 0x0014_f900, 0x0015_2fd4, 0x0015_2fe8)?; // BEBAGTInt
            c.mult(0, S0, S2, S0);
            c.g[V0][0] = 0x4a_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[S2][0] = c.g[SP][0];
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x288));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            loop {
                c.cycles += 9;
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf88));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0015_15b8, 0x0015_3024)?; // BE_RestoreCharProfileEquipFlags
                let more = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xf88));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_3008);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `int BESaveInt_GetSystemSaveSize(void)` (0x152948): 0xccc.
pub fn be_save_int_get_system_save_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xccc;
    ret(c);
}

/// `void* BESaveInt_BuildSystemSave(self)`: allocates the system save buffer, copies the option
/// DB (0x288 bytes), the high scores (0xa28 bytes), packs the controller bits of the six
/// players and appends the CRC16. Returns the buffer (also in `gp-0x1ed0`).
pub fn be_save_int_build_system_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_152858: {
            'b_152818: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[RA][0] = sx(0x00152770);
                c.g[S0][0] = c.g[A0][0];
                Callee {
                    addr: 0x00152948,
                    f: ee::s01::BESaveInt_GetSystemSaveSize,
                }
                .call(c, 0x00152770)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0015277c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00152700,
                    f: ee::s01::BESaveInt_FreeSaveBuffer,
                }
                .call(c, 0x0015277c)?;
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = c.g[S1][0];
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5810));
                c.g[RA][0] = sx(0x00152794);
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x00317e30, 0x00152794)?;
                c.g[A0][0] = 0x530000;
                c.g[S0][0] = c.g[V0][0];
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5898));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ed0), c.g[V0][0]);
                c.g[V1][0] = c.g[S0][0] | c.g[T0][0];
                c.g[V1][0] = c.g[V1][0] & 0x7;
                let t = c.g[V1][0] == 0u64;
                c.g[A3][0] = c.g[S0][0];
                if t {
                    break 'b_152818;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x280));
                    c.g[A1][0] = 0x530000;
                    'l_1527c4: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[T0][0] as u32);
                        c.ldl(4, (c.g[T0][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0x17));
                        c.ldr(9, (c.g[T0][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[T0][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[T0][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.sdr(3, c.g[A3][0] as u32);
                        c.sdl(4, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.sdr(4, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(9, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.sdr(9, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                        let t = c.g[T0][0] != c.g[A2][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001527c4);
                            }
                            continue 'l_1527c4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[S0][0] = c.g[V0][0];
                            if t {
                                break 'b_152858;
                            } else {
                                break 'b_152818;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
            c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x280));
            c.g[A1][0] = 0x530000;
            'l_152828: loop {
                c.cycles += 11;
                c.g[V1][0] = c.ld(c.g[T0][0] as u32);
                c.g[A0][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[T1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[T2][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A3][0] as u32, c.g[V1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                let t = c.g[T0][0] != c.g[A2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00152828);
                    }
                    continue 'l_152828;
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[V0][0];
                    break 'b_152858;
                }
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5c18));
        c.ldl(2, (c.g[T0][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[T0][0] as u32);
        c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.g[A0][0] = c.g[S0][0];
        c.sdr(2, c.g[A3][0] as u32);
        c.g[RA][0] = sx(0x00152878);
        c.g[A2][0] = 0xa28;
        call_ptr(c, 0x0041605c, 0x00152878)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa28));
        c.g[V0][0] = 0x530000;
        c.g[T1][0] = 0xfffffffffffffffe;
        c.g[T0][0] = 0xfffffffffffffe01;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
        c.g[A3][0] = 0x5;
        'l_152898: loop {
            c.cycles += 15;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[A0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x13));
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V1][0] = c.g[V1][0] & c.g[T1][0];
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
            c.sw(c.g[A1][0] as u32, c.g[V1][0]);
            let t = (c.g[A3][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00152898);
                }
                continue 'l_152898;
            } else {
                c.cycles += 28;
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[SP][0] as u32);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[S0][0] as u32);
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x17));
                c.g[A2][0] = 0xfbea;
                c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00152918);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x003e62d0, 0x00152918)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.lwl(9, (c.g[SP][0] as u32).wrapping_add(0x23));
                c.lwr(9, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swl(9, (c.g[S0][0] as u32).wrapping_add(0x3));
                c.swr(9, c.g[S0][0] as u32);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void* BESaveInt_BuildProfileSave(self, int idx)`: allocates the profile save buffer, copies
/// the user name, the player DB record (0x9b40 bytes) and one profile byte, lets the five
/// interfaces (new player, BAGT, mission, score, stat) write their parts through their
/// virtual `discard` and appends the CRC16. Returns the buffer.
pub fn be_save_int_build_profile_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_152a88: {
            'b_152a48: {
                c.cycles += 37;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x00152970);
                c.g[S2][0] = c.g[A1][0];
                Callee {
                    addr: 0x00152ba8,
                    f: ee::s01::BESaveInt_GetProfileSaveSize,
                }
                .call(c, 0x00152970)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0015297c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00152700,
                    f: ee::s01::BESaveInt_FreeSaveBuffer,
                }
                .call(c, 0x0015297c)?;
                c.g[A1][0] = 0x460000;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5810));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00152994);
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x00317e30, 0x00152994)?;
                c.g[V1][0] = 0x1c;
                c.g[A1][0] = 0x530000;
                c.mult(0, 3, 18, 3);
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b20));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0xc;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ed0), c.g[V0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[RA][0] = sx(0x001529c0);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, 0x00416ce0, 0x001529c0)?;
                c.g[A0][0] = 0x9b50;
                c.g[V1][0] = 0x4a0000;
                c.mult(0, 4, 18, 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.g[A2][0] | c.g[S0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[S0][0];
                if t {
                    break 'b_152a48;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = 0x9b40;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = 0x530000;
                    'l_1529f4: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[A2][0] as u32);
                        c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.sdr(3, c.g[A3][0] as u32);
                        c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[V0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001529f4);
                            }
                            continue 'l_1529f4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V1][0] = 0x1c;
                            if t {
                                break 'b_152a88;
                            } else {
                                break 'b_152a48;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 0x9b40;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = 0x530000;
            'l_152a58: loop {
                c.cycles += 11;
                c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                c.g[A1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A3][0] as u32, c.g[V1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00152a58);
                    }
                    continue 'l_152a58;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = 0x1c;
                    break 'b_152a88;
                }
            }
        }
        c.cycles += 71;
        c.g[V0][0] = 0x9b50;
        c.mult(0, 3, 18, 3);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[A2][0] as u32);
        c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
        c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[A3][0] as u32);
        c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b31));
        c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
        c.g[RA][0] = sx(0x00152ac8);
        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
        Callee {
            addr: 0x00145970,
            f: ee::s01::BENewPlayerInt_GetInstance,
        }
        .call(c, 0x00152ac8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152ae4);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00152ae4)?;
        c.g[RA][0] = sx(0x00152aec);
        Callee {
            addr: 0x0014f900,
            f: ee::s01::BEBAGTInt_GetInstance,
        }
        .call(c, 0x00152aec)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b04);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00152b04)?;
        c.g[RA][0] = sx(0x00152b0c);
        Callee {
            addr: 0x001532a0,
            f: ee::s01::BEMissionInt_GetInstance,
        }
        .call(c, 0x00152b0c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b24);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00152b24)?;
        c.g[RA][0] = sx(0x00152b2c);
        Callee {
            addr: 0x00154948,
            f: ee::s01::BEScoreInt_GetInstance,
        }
        .call(c, 0x00152b2c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b44);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00152b44)?;
        c.g[RA][0] = sx(0x00152b4c);
        Callee {
            addr: 0x00147f78,
            f: ee::s01::BECharInt_GetInstance,
        }
        .call(c, 0x00152b4c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00152b64);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00152b64)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00152b74);
        c.g[A2][0] = 0xfbea;
        call_ptr(c, 0x003e62d0, 0x00152b74)?;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1ed0));
        c.lwl(3, (c.g[SP][0] as u32).wrapping_add(0x3));
        c.lwr(3, c.g[SP][0] as u32);
        c.swl(3, (c.g[S0][0] as u32).wrapping_add(0x3));
        c.swr(3, c.g[S0][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int BESaveInt_LoadSystemSave(self, void* buf)`: validates the CRC16 of a system save and
/// loads it: option DB, high scores, controller bits, with the `onLoad` calls of the option,
/// score and new-player interfaces and the locale/display refresh. Returns 0 on a bad CRC,
/// else 1.
pub fn be_save_int_load_system_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_152dc8: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[RA][0] = sx(0x00152bcc);
            c.g[S1][0] = c.g[A1][0];
            Callee {
                addr: 0x00152948,
                f: ee::s01::BESaveInt_GetSystemSaveSize,
            }
            .call(c, 0x00152bcc)?;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00152be0);
            c.g[A2][0] = 0xfbea;
            call_ptr(c, 0x003e62d0, 0x00152be0)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.lwl(3, (c.g[S0][0] as u32).wrapping_add(0x3));
            c.lwr(3, c.g[S0][0] as u32);
            c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x23));
            c.swr(3, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0u64;
            if t {
                break 'b_152dc8;
            } else {
                'b_152cb8: {
                    'b_152c80: {
                        c.cycles += 7;
                        c.g[V0][0] = 0x530000;
                        c.g[S0][0] = c.g[S1][0];
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5898));
                        c.g[V0][0] = c.g[S0][0] | c.g[A2][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.g[S0][0];
                        if t {
                            break 'b_152c80;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                            c.g[S2][0] = 0x530000;
                            'l_152c2c: loop {
                                c.cycles += 19;
                                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[V1][0] as u32);
                                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                                c.sdr(7, c.g[A2][0] as u32);
                                c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0xf));
                                c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
                                c.sdr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x1f));
                                c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00152c2c);
                                    }
                                    continue 'l_152c2c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_152cb8;
                                    } else {
                                        break 'b_152c80;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x288));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                    c.g[S2][0] = 0x530000;
                    'l_152c8c: loop {
                        c.cycles += 11;
                        c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                        c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A2][0] as u32, c.g[A3][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00152c8c);
                            }
                            continue 'l_152c8c;
                        } else {
                            break 'b_152cb8;
                        }
                    }
                }
                c.cycles += 42;
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[V1][0] as u32);
                c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.g[S0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00152cd0);
                c.sdr(7, c.g[A2][0] as u32);
                Callee {
                    addr: 0x0014f250,
                    f: ee::s01::BEOptionInt_GetInstance,
                }
                .call(c, 0x00152cd0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00152ce8);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00152ce8)?;
                c.g[A2][0] = 0xa28;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5c18));
                c.g[RA][0] = sx(0x00152cfc);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa28));
                call_ptr(c, 0x0041605c, 0x00152cfc)?;
                c.g[RA][0] = sx(0x00152d04);
                Callee {
                    addr: 0x00154948,
                    f: ee::s01::BEScoreInt_GetInstance,
                }
                .call(c, 0x00152d04)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00152d1c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00152d1c)?;
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x7));
                c.ldr(5, c.g[S0][0] as u32);
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x17));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, c.g[SP][0] as u32);
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.g[V0][0] = 0x530000;
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = 0xfffffffffffffffd;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b20));
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0x5;
                'l_152d60: loop {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[A0][0] & 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sb((c.g[A1][0] as u32).wrapping_add(0x13), c.g[A0][0]);
                    let t = (c.g[A3][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00152d60);
                        }
                        continue 'l_152d60;
                    } else {
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x00152d9c);
                        Callee {
                            addr: 0x00145970,
                            f: ee::s01::BENewPlayerInt_GetInstance,
                        }
                        .call(c, 0x00152d9c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00152db4);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00152db4)?;
                        c.g[RA][0] = sx(0x00152dbc);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        call_ptr(c, 0x00228a78, 0x00152dbc)?;
                        c.g[RA][0] = sx(0x00152dc4);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        call_ptr(c, 0x00228c08, 0x00152dc4)?;
                        c.g[V0][0] = 0x1;
                        break 'b_152dc8;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beintnetwork::tests::alloc_setup;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;

    use ssx3_ee::s01;

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

    use crate::difftest::check_lohi;

    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::Rng;

    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;

    /// Fixture for `BESaveInt_LoadProfileSave`: a save buffer of 0x9c10 bytes
    /// at `DATA + 0x1000` (+4 on odd seeds: the unaligned copy loop), the CRC
    /// table zeroed (so the CRC is 0 and a zero stored word matches), forced
    /// fake results (call 0: the save size, calls 2/4/6: the interface
    /// objects, whose vtable slot `+0x10` points at fake methods).
    pub(crate) fn oc430_profile_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let buf = DATA + 0x1000 + if seed & 1 == 1 { 4 } else { 0 };
        fill(c, r, buf, 0x2700);
        for k in 0..0x80 {
            c.bus.write32(0x0045_1018 + 4 * k, 0);
        }
        c.bus.write32(
            buf + 0x9c00,
            if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
        );
        c.bus.write32(fakes::LOG_RESULTS, 0x9c04);
        for (k, n) in [2u32, 4, 6].into_iter().enumerate() {
            let obj = DATA + 0x2_0000 + 0x100 * k as u32;
            let vt = DATA + 0x3_0000 + 0x40 * k as u32;
            c.bus.write32(obj + 0xc, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + 0x14, fakes::FAKE + 0x10 * k as u32);
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, obj);
        }
        c.g[A1][0] = sx(buf);
        c.g[A2][0] = sx(r.next_u32() % 4);
    }

    /// `BESaveInt_LoadProfileSave`: matching and mismatching CRC words, four
    /// profile slots, aligned and unaligned block copies (LO/HI too); the
    /// real CRC runs, everything else is fake.
    #[test]
    fn be_save_int_load_profile_save_matches() {
        oc430_install_fakes();
        check_lohi(
            ee::s01::f_00152de0__generated,
            be_save_int_load_profile_save,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_2de0, 14);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0015_2de0;
                oc430_profile_fixture(c, &mut r, seed);
            },
        );
    }

    /// `BESaveInt_GetSystemSaveSize`: constant 0xccc.
    #[test]
    fn be_save_int_get_system_save_size_matches() {
        check(
            ee::s01::f_00152948__generated,
            be_save_int_get_system_save_size,
            20,
            &|c, seed| {
                let mut r = Rng(0x5a ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_2948;
            },
        );
    }

    pub(crate) use crate::game::be::beintecon::tests::oc676_instance;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (outside the fakes' call log).
    pub(crate) const OC676_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC676_MAGIC: u32 = 0x0c67_6c67;

    pub(crate) fn oc676_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC676_CTRL) == OC676_MAGIC
    }

    macro_rules! oc676_fake {
        ($name:ident, $addr:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if !oc676_gated(c) {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc676_fake!(
        oc676_fake_new_named,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    oc676_fake!(
        oc676_fake_memcpy,
        0x0041_605c,
        ee::s15::f_0041605c__generated
    );

    oc676_fake!(
        oc676_fake_strncpy,
        0x0041_6ce0,
        ee::s15::f_00416ce0__generated
    );

    oc676_fake!(
        oc676_fake_load_locale,
        0x0022_8a78,
        ee::s05::f_00228a78__generated
    );

    oc676_fake!(
        oc676_fake_apply_display,
        0x0022_8c08,
        ee::s05::f_00228c08__generated
    );

    /// Hooks the gated fakes of the save functions' callees (once per process).
    pub(crate) fn oc676_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc676_fake_new_named,
            fakes::prev_cell(oc676_fake_new_named),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0041605C),
            oc676_fake_memcpy,
            fakes::prev_cell(oc676_fake_memcpy),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416CE0),
            oc676_fake_strncpy,
            fakes::prev_cell(oc676_fake_strncpy),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00228A78),
            oc676_fake_load_locale,
            fakes::prev_cell(oc676_fake_load_locale),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00228C08),
            oc676_fake_apply_display,
            fakes::prev_cell(oc676_fake_apply_display),
        );
    }

    /// Opens the gate on this machine.
    pub(crate) fn oc676_gate(c: &mut Cpu) {
        oc676_install_fakes();
        c.bus.write32(OC676_CTRL, OC676_MAGIC);
    }

    /// The buffer the allocator fake hands out (alignment alternates).
    pub(crate) fn oc676_save_buffer(c: &mut Cpu, r: &mut Rng, seed: u32, words: u32) -> u32 {
        let buf = DATA + 0x1_0000 + (seed % 2) * 4;
        c.bus.write32(fakes::LOG_RESULTS, buf);
        fill(c, r, buf, words);
        c.bus.write32(GAME_GP - 0x1ed0, 0);
        buf
    }

    /// `BESaveInt_BuildSystemSave`: both copy forms (aligned and unaligned
    /// buffer); allocator and memcpy are gated fakes, the CRC is real.
    #[test]
    fn be_save_int_build_system_save_matches() {
        covered(
            ee::s01::f_00152758__generated,
            be_save_int_build_system_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x152758, 3);
                oc676_gate(c);
                oc676_save_buffer(c, &mut r, seed, 0x340);
                fill(c, &mut r, 0x0053_5898, 0xd2);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x00152758;
            },
        );
    }

    /// `BESaveInt_BuildProfileSave`: profiles 0..2, both copy forms; the five
    /// interfaces are singletons with a fake `discard`.
    #[test]
    fn be_save_int_build_profile_save_matches() {
        covered(
            ee::s01::f_00152950__generated,
            be_save_int_build_profile_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x152950, 7);
                oc676_gate(c);
                oc676_save_buffer(c, &mut r, seed, 0x2700);
                let idx = seed % 3;
                fill(c, &mut r, 0x004a_6ca8 + idx * 0x9b50, 0x26d0);
                fill(c, &mut r, 0x0053_5b20, 42);
                oc676_instance(c, 0x1f30, 0);
                oc676_instance(c, 0x1ee0, 1);
                oc676_instance(c, 0x1eb8, 2);
                oc676_instance(c, 0x1ea8, 3);
                oc676_instance(c, 0x1f2c, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                c.pc = 0x00152950;
            },
        );
    }

    /// CRC16 of `len` bytes at `buf` (`EA_Crc16` seed 0xfbea) on a scratch machine.
    pub(crate) fn oc676_crc16(c: &mut Cpu, buf: u32, len: u32) -> u32 {
        let mut t = crate::difftest::fresh();
        for i in 0..len {
            let b = c.bus.read8(buf + i);
            t.bus.write8(buf + i, b);
        }
        t.g[A0][0] = buf as u64;
        t.g[A1][0] = len as u64;
        t.g[A2][0] = 0xfbea;
        t.pc = 0x003e_62d0;
        ee::s14::f_003e62d0(&mut t);
        t.g[V0][0] as u32
    }

    /// `BESaveInt_LoadSystemSave`: a good CRC (loads: copy forms aligned and
    /// not, the three `onLoad` calls, the locale refresh) and a bad one.
    #[test]
    fn be_save_int_load_system_save_matches() {
        covered(
            ee::s01::f_00152bb0__generated,
            be_save_int_load_system_save,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x152bb0, 6);
                oc676_gate(c);
                let buf = DATA + 0x1_0000 + (seed % 2) * 4;
                fill(c, &mut r, buf, 0x333);
                let crc = oc676_crc16(c, buf, 0xcc8);
                let stored = if seed % 4 == 0 {
                    crc.wrapping_add(1)
                } else {
                    crc
                };
                c.bus.write32(buf + 0xcc8, stored);
                fill(c, &mut r, 0x0053_5b20, 42);
                oc676_instance(c, 0x1ee4, 0);
                oc676_instance(c, 0x1ea8, 1);
                oc676_instance(c, 0x1f30, 2);
                c.bus.write32(GAME_GP - 0x848, DATA + 0x200);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = buf as u64;
                c.pc = 0x00152bb0;
            },
        );
    }
}
