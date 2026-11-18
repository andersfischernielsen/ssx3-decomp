//! `src/be/beintsavefile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintreward::alloc_named;
pub(crate) use crate::game::fe::festateload::glo;
use crate::guest::Addr;
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
}
