//! `src/fe/fereal.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::oc437_dtor_flag_delete;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_27a0, boot_movies_update, "ssx_BootMoviesUpdate"),
    (0x001a_0720, feui_custom_ctor, "FEUICustom_ctor"),
    (0x001a_07c8, feuicustom_v03, "FEUICustom_v03"),
    (
        0x001a_1cb8,
        fe_module_set_player_pad_mask,
        "FEModule_SetPlayerPadMask",
    ),
    (
        0x001a_1cd0,
        fe_module_get_player_pad_mask,
        "FEModule_GetPlayerPadMask",
    ),
    (0x001a_1d50, fe_module_dtor, "FEModule_dtor"),
    (0x001a_2128, femodule_try_exit, "FEModule_TryExit"),
    (0x001a_2190, fe_module_unload, "FEModule_Unload"),
    (
        0x001a_2bc0,
        femodule_unload_anim_data,
        "FEModule_UnloadAnimData",
    ),
    (0x001a_2e20, feui_sound_ctor, "FEUISound_ctor"),
    (0x001a_2f38, ovui_sound_ctor, "OVUISound_ctor"),
    (0x001a_2208, fe_module_render, "FEModule_Render"),
    (0x001d_cc78, feui_custom_dtor, "FEUICustom_dtor"),
    (0x001a_1ce8, fe_module_ctor, "FEModule_ctor"),
    (0x001a_1dc0, fe_module_enter, "FEModule_Enter"),
    (0x001a_2c40, cfe_real_fe_load_start_state, "FEModule_v08"),
    (0x001a_2f70, ovui_sound_v02, "OVUISound_v02"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_SOUND_BASE_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;

    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;

    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
}

/// `void FEModule_SetPlayerPadMask(FEModule* m, int slot, u8 mask)` (0x1a1cb8):
/// the per-player pad mask bytes at +8, indexed by the sign-extended `slot`.
pub fn fe_module_set_player_pad_mask(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let slot = abi::arg(c, 1) as u8 as i8 as i32;
    c.g[A1][0] = sx(slot as u32);
    c.g[A0][0] = sx(selfp.wrapping_add(slot as u32));
    let mask = c.g[A2][0];
    Addr(c.g[A0][0] as u32).off(8).sb(c, mask);
    abi::ret(c);
}

/// `u8 FEModule_GetPlayerPadMask(FEModule* m, int slot)` (0x1a1cd0): the
/// per-player pad mask byte at +8.
pub fn fe_module_get_player_pad_mask(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let slot = abi::arg(c, 1) as u8 as i8 as i32;
    c.g[A1][0] = sx(slot as u32);
    c.g[A0][0] = sx(selfp.wrapping_add(slot as u32));
    let addr = (c.g[A0][0] as u32).wrapping_add(8);
    c.g[V0][0] = c.lbu(addr);
    abi::ret(c);
}

/// `void* FEUISound_ctor(FEUISound* self)` (0x1a2e20): base UI-sound
/// constructor plus the derived vtable.
pub fn feui_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_SOUND_BASE_CTOR.call(c, 0x001a_2e34)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6cb8));
        let v1 = c.g[V1][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(4).sw(c, 0);
        Addr(s0 as u32).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* OVUISound_ctor(OVUISound* self)` (0x1a2f38): the overlay variant of
/// the UI-sound constructor.
pub fn ovui_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_SOUND_BASE_CTOR.call(c, 0x001a_2f4c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ce8));
        let v1 = c.g[V1][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(4).sw(c, 0);
        Addr(s0 as u32).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `addiu sp, sp, -n`.
#[inline(always)]
pub(crate) fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}

/// `addiu sp, sp, n; jr ra` (with `ra` already reloaded by the caller).
#[inline(always)]
pub(crate) fn close(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(n));
    abi::ret(c);
}

/// `int FEUICustom_v03(FEUIObject* self, int unused, int hash)` (0x1a07c8):
/// true (and the screen object's slot 24 invoked with `(1, 0)`) when `hash`
/// is the hash of the credits object name `D_0045FFF8`.
pub fn feuicustom_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.g[A2][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001a_07ec);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x8));
        call_ptr(c, 0x0031_7670, 0x001a_07ec)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            let no_obj = c.g[A3][0] == 0;
            c.g[A1][0] = 0x1;
            if !no_obj {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001a_0818);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_0818)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            c.g[V0][0] = 0u64;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `bool FEModule_TryExit(FEModule* self)` (0x1a2128): exit the current flow
/// state, stop the module's UI screens and free the module's flow state.
pub fn femodule_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001a_213c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0023_1cf0, 0x001a_213c)?;
        let can_exit = c.g[V0][0] != 0;
        c.g[V0][0] = 0xb0000;
        if can_exit {
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5ad4));
            if c.g[A0][0] == 0 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad8), 0u64);
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a_2164);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x0025_3418, 0x001a_2164)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad4), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad8), 0u64);
            }
            c.cycles += 5;
            c.g[A0][0] = sx(gp(c, 0x1d90));
            c.sw(gp(c, -0x1724), 0u64);
            c.g[RA][0] = sx(0x001a_217c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5adc), 0u64);
            call_ptr(c, 0x001a_06b0, 0x001a_217c)?;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `void FEModule_dtor(FEModule* self, int flags)` (0x1a1d50): tear down the
/// two net hooks then the base front-end module.
pub fn fe_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x46d7_c0;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0xb5abc;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(at(c, S0, 0xb5abc));
        Addr(at(c, S0, 0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0026_39b0, 0x001a_1d88)?;
        c.g[A1][0] = 2;
        c.g[A0][0] = sx(at(c, S0, 0xb5ab0));
        call_ptr(c, 0x0026_3678, 0x001a_1d9c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0019_57d0, 0x001a_1da8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_001A2190` (0x1a2190): `FEModule_Unload`. Unloads the module's
/// animation data, rider previews and bolt-file cache, then shuts the file
/// system down and clears the world's rider list (`world+0x80`, `+0x7c`)
/// through `0x28b180`/`0x286200`. Signature: `void unload(FEModule* this)`.
pub fn fe_module_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_r(c, A1, 0x10d8);
        c.g[A0][0] = lh_r(c, V0, 0x98);
        c.g[V1][0] = lw_r(c, V0, 0x9c);
        let target = low(c, V1);
        c.g[RA][0] = sx(0x001a_21b8);
        c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x001a_21b8)?;
        c.g[RA][0] = sx(0x001a_21c0);
        c.g[A0][0] = c.g[S0][0];
        callee::FE_UNLOAD_ANIM_DATA.call(c, 0x001a_21c0)?;
        c.g[RA][0] = sx(0x001a_21c8);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0xb0));
        callee::FE_UNLOAD_RIDER_PREVIEWS.call(c, 0x001a_21c8)?;
        c.g[RA][0] = sx(0x001a_21d0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0x1a70));
        callee::BOLT_FILE_CACHE_SHUTDOWN.call(c, 0x001a_21d0)?;
        c.g[A0][0] = 0x50_0000;
        c.g[RA][0] = sx(0x001a_21dc);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x47f8));
        call_ptr(c, 0x0038_adb0, 0x001a_21dc)?;
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x848));
        sw_r(c, V0, 0x80, 0);
        c.g[RA][0] = sx(0x001a_21ec);
        sw_r(c, V0, 0x7c, 0);
        call_ptr(c, 0x0028_b180, 0x001a_21ec)?;
        c.g[RA][0] = sx(0x001a_21f4);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_6200, 0x001a_21f4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_UnloadAnimData(self)` (0x1a2bc0): drop the first animation
/// bank: unregister the bank, free its channel caches and destroy the data
/// source.
pub fn femodule_unload_anim_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[S2][0] & 0xff;
        loop {
            c.cycles += 7;
            let banks = lw_at(c, gw(c, 0xd08));
            c.g[V0][0] = banks;
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = lw_at(c, c.g[V0][0] as u32);
            let none = c.g[S0][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if !none {
                c.cycles += 9;
                c.g[A0][0] = lw_at(c, gw(c, 0xd8c));
                call_ptr(c, callee::GAME_ANIM_MAP_UNREGISTER_BANK.addr, 0x001a_2c04)?;
                c.g[A0][0] = lw_at(c, gw(c, 0xd08));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.sw(c.g[V0][0] as u32, 0);
                call_ptr(c, callee::ANIM_DATA_BASE_FREE_CHAN_CACHES.addr, 0x001a_2c14)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 3;
                call_ptr(c, callee::ADF_DATA_SOURCE_DTOR.addr, 0x001a_2c20)?;
            }
            c.cycles += 2;
            let again = (c.g[S2][0] as i64) <= 0;
            c.g[V1][0] = c.g[S2][0] & 0xff;
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_2be0);
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* FEUICustom_ctor(self, owner)` (0x1a0720): build the custom UI
/// object and push it onto the state stack.
pub fn feui_custom_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0047_0000);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x27f0));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x50;
        Addr(r32(c, S1)).sw(c, c.g[A1][0]);
        c.g[A2][0] = 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, 0);
        c.g[A1][0] = sx(0x0046_0b90);
        c.g[A3][0] = 0;
        Addr(p(r32(c, S1), 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x001a0760)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e2a0, 0x001a0770)?;
        c.g[V1][0] = sx(0x0047_0000);
        Addr(p(r32(c, S0), 0x48)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4478));
        Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        let null = c.g[S0][0] == 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, c.g[S0][0]);
        if !null {
            c.cycles += 8;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
            let init = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, init, 0x001a079c)?;
            c.g[A0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
            c.g[A0][0] = sx(p(r32(c, A0), 0x18));
            call_ptr(c, 0x0039_f238, 0x001a07ac)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `FEModule_LoadAnimData` (0x001a2b00).
//
// `void cRealFE::loadCharAnimations()`: load the character-animation table
// file and install its (single) entry.

pub fn fe_module_load_anim_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1330));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = 0x64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x001a2b30);
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x003ded50, 0x001a2b30)?;
        c.g[S2][0] = 0x460000;
        c.g[S1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1730));
        c.g[A2][0] = 0u64;
        'l_1a2b40: loop {
            c.cycles += 20;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = 0x18;
            c.g[RA][0] = sx(0x001a2b50);
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1348));
            call_ptr(c, 0x00317d70, 0x001a2b50)?;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001a2b60);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00311250, 0x001a2b60)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd08));
            c.g[A0][0] = c.g[S0][0] & 0xff;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[RA][0] = sx(0x001a2b88);
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x00314f30, 0x001a2b88)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x001a2b40);
                }
                continue 'l_1a2b40;
            } else {
                c.cycles += 11;
                c.g[RA][0] = sx(0x001a2b98);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
                call_ptr(c, 0x003150b0, 0x001a2b98)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x001a2ba4);
                c.g[A1][0] = 0x64;
                call_ptr(c, 0x003dedc0, 0x001a2ba4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `ssx_BootMoviesUpdate` (0x1a27a0): the boot movies (EA logo, THX, the
/// intro), one at a time. The original lets X or Start skip only the intro:
/// it sets the "skippable" flag (`+0xb5ad8`) for the intro and clears it for
/// the logos when each starts. Here the flag is set while any of them plays
/// (`+0xb5ad4`, the player), so the game's own skip check (bindings 0x70 and
/// 0x7a) ends the logos too. Runs without movies (headless, the replays)
/// never see a player and are unchanged.
pub fn boot_movies_update(c: &mut Cpu) {
    let fe = r32(c, A0);
    if c.bus.read32(fe.wrapping_add(BOOT_PLAYER)) != 0 {
        c.bus.write32(fe.wrapping_add(BOOT_SKIPPABLE), 1);
    }
    ssx3_ee::s03::f_001a27a0__generated(c);
}

/// The front-end module's boot movie player and its "skippable" flag.
const BOOT_PLAYER: u32 = 0xb5ad4;
const BOOT_SKIPPABLE: u32 = 0xb5ad8;

// ------------------------------------------------------------ FEModule_Render

/// `gp`-relative `cPS2GraphicsMan* GrMan`.
pub(crate) const FE_MODULE_RENDER_GRMAN_GP: u32 = 0x854;

/// `gp`-relative `D_004A19CC`: draw a black backdrop instead of the module.
pub(crate) const FE_MODULE_RENDER_BACKDROP_GP: u32 = 0x1724;

/// `gp`-relative literal 0.02f (the audio update step).
pub(crate) const FE_MODULE_RENDER_AUDIO_STEP_GP: u32 = 0x5970;

/// GrMan: top of the graphics-state stack (`sFeGs*`, 0x14 bytes each).
pub(crate) const FE_MODULE_RENDER_GS_TOP: u32 = 0xe84;

/// GrMan: vtable (`{short delta; short; fn}` entries, 8 bytes).
pub(crate) const FE_MODULE_RENDER_VTABLE: u32 = 0x10d8;

/// FEModule: fade-out counter (`self + 0xb5adc`).
pub(crate) const FE_MODULE_RENDER_FADE: u32 = 0x5adc;

/// FEModule: the playing video (`self + 0xb5ad4`).
pub(crate) const FE_MODULE_RENDER_VIDEO: u32 = 0x5ad4;

/// The module's default graphics state `D_00501420`.
pub(crate) const FE_MODULE_RENDER_DEFAULT_GS: u32 = 0x0050_1420;

/// `rt = GrMan` (`lw rt, -0x854(gp)`).
pub(crate) fn fe_render_grman(c: &mut Cpu, rt: usize) {
    let gp = r32(c, GP);
    c.g[rt][0] = Addr(gp.wrapping_sub(FE_MODULE_RENDER_GRMAN_GP)).lw(c);
}

/// `vt = GrMan->vtable`, with GrMan in `this`.
pub(crate) fn fe_render_vt(c: &mut Cpu, this: usize, vt: usize) {
    let t = r32(c, this);
    c.g[vt][0] = Addr(t.wrapping_add(FE_MODULE_RENDER_VTABLE)).lw(c);
}

/// A GrMan virtual call through the vtable entry at `off`: `delta`/`func`
/// get the entry's halves, `a0 = this + delta`, `jalr func` back to `ret`.
pub(crate) fn fe_render_vcall(
    c: &mut Cpu,
    this: usize,
    vt: usize,
    delta: usize,
    func: usize,
    off: u32,
    ret: u32,
) -> Flow {
    let obj = c.g[this][0];
    let entry = r32(c, vt).wrapping_add(off);
    c.g[delta][0] = Addr(entry).lh(c);
    c.g[func][0] = Addr(entry.wrapping_add(4)).lw(c);
    let target = r32(c, func);
    c.g[A0][0] = crate::abi::addu(obj, c.g[delta][0]);
    call_ptr(c, target, ret)
}

/// Unsigned int to float as gcc emits it into `fd`: `cvt.s.w` of the value,
/// or of `(v >> 1) | (v & 1)` doubled when the sign bit is set.
pub(crate) fn fe_render_u2f(c: &mut Cpu, fd: usize) {
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] &= 0x1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(V0, fd);
        c.cvt_s_w(fd, fd);
        c.add_s(fd, fd, fd);
    } else {
        c.cycles += 4;
        c.mtc1(V0, fd);
        c.cvt_s_w(fd, fd);
    }
}

/// Copy the 0x14-byte graphics state at `src` to `dst` with the
/// unaligned `ldl/ldr` + `sdl/sdr` pairs, through `ra`/`rb`/`rw`.
pub(crate) fn fe_render_copy_gs(c: &mut Cpu, src: u32, dst: u32, ra: usize, rb: usize, rw: usize) {
    c.ldl(ra, src.wrapping_add(0x7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(0x8));
    c.g[rw][0] = Addr(src.wrapping_add(0x10)).lw(c);
    c.sdl(ra, dst.wrapping_add(0x7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(0x8));
    Addr(dst.wrapping_add(0x10)).sw(c, c.g[rw][0]);
}

/// `int FEModule_Render(FEModule* self)` (0x1a2208): the front end's frame
/// render. Nothing (0) while the graphics manager is not ready (vslot 17).
/// Counts the fade-out timer down; draws the fade (`AppModule_RenderFade`)
/// or the playing video and ends the frame (vslot 20) if either is active.
/// With `D_004A19CC` set it draws a black full-screen quad with an
/// orthographic view instead. Otherwise: camera (`self+0x10..0x18`) and view
/// matrix (`self+0x50`), the module's default graphics state pushed, the UI
/// state manager, the rider previews, the UI engine, the end of the frame and
/// the audio update (0.02 s). Returns 1.
pub fn fe_module_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x110);
        fe_render_grman(c, A1);
        frame.save(c, S1, 0xe0);
        frame.save(c, S0, 0xf0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        let this = r32(c, S1);
        // if (!GrMan->vslot17()) return 0;
        fe_render_vt(c, A1, V1);
        fe_render_vcall(c, A1, V1, A0, V0, 0x88, 0x001a_223c)?;
        let ready = c.g[V0][0] != 0;
        c.g[V0][0] = 0xb0000;
        if !ready {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return fe_render_epilogue(c, frame);
        }

        // Fade-out counter.
        c.cycles += 4;
        c.g[S0][0] = sx(this.wrapping_add(0xb0000));
        let hi = r32(c, S0);
        let fade = Addr(hi.wrapping_add(FE_MODULE_RENDER_FADE)).lw(c);
        c.g[V0][0] = sx((fade as u32).wrapping_sub(1));
        if fade != 0 {
            c.cycles += 1;
            Addr(hi.wrapping_add(FE_MODULE_RENDER_FADE)).sw(c, c.g[V0][0]);
        }

        // AppModule_RenderFade(self): end the frame.
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0023_1d60, 0x001a_2268)?;
        let fading = c.g[V0][0] != 0;
        fe_render_grman(c, A1);
        if fading {
            c.cycles += 7;
            fe_render_vt(c, A1, V0);
            fe_render_vcall(c, A1, V0, A0, V1, 0xa0, 0x001a_2284)?;
            c.g[V0][0] = 0x1;
            return fe_render_epilogue(c, frame);
        }

        // A playing video: VidEnginePlayer_Render(video, 0), end the frame.
        c.cycles += 3;
        let hi = r32(c, S0);
        c.g[A0][0] = Addr(hi.wrapping_add(FE_MODULE_RENDER_VIDEO)).lw(c);
        let video = c.g[A0][0] != 0;
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_sub(FE_MODULE_RENDER_BACKDROP_GP)).lw(c);
        if video {
            c.cycles += 10;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0025_3890, 0x001a_22a0)?;
            fe_render_grman(c, A1);
            fe_render_vt(c, A1, V0);
            fe_render_vcall(c, A1, V0, A0, V1, 0xa0, 0x001a_22b8)?;
            c.g[V0][0] = 0x1;
            return fe_render_epilogue(c, frame);
        }

        c.cycles += 2;
        let backdrop = c.g[V0][0] != 0;
        fe_render_grman(c, A1);
        if backdrop {
            fe_render_backdrop(c)?;
            c.g[V0][0] = 0x1;
            return fe_render_epilogue(c, frame);
        }

        // The module: begin the frame (vslots 21, 31), camera, view.
        c.cycles += 132;
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xa8, 0x001a_2584)?;
        fe_render_grman(c, A1);
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xf8, 0x001a_259c)?;
        fe_render_grman(c, A1);
        let this = r32(c, S1);
        c.lwc1(12, this.wrapping_add(0x10));
        c.lwc1(13, this.wrapping_add(0x14));
        c.lwc1(14, this.wrapping_add(0x18));
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xc8, 0x001a_25c0)?;
        fe_render_grman(c, A2);
        let this = r32(c, S1);
        c.g[A1][0] = sx(this.wrapping_add(0x50));
        fe_render_vt(c, A2, V0);
        fe_render_vcall(c, A2, V0, A0, V1, 0x110, 0x001a_25dc)?;

        // Push a copy of the top graphics state, then load the module's
        // default state and set its fields.
        fe_render_grman(c, A0);
        let grman = r32(c, A0);
        c.g[T2][0] = 0x500000;
        c.g[A2][0] = 0xffff_ffff_ffbf_0000 | 0xffff;
        c.g[T0][0] = 0xffff_ffff_fe7f_0000 | 0xffff;
        c.g[T3][0] = 0x800000;
        let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
        c.g[V0][0] = top;
        fe_render_copy_gs(c, top as u32, (top as u32).wrapping_add(0x14), T7, V1, A1);
        c.g[T4][0] = 0x300000;
        c.g[A3][0] = 0xffff_ffff_ffcf_0000 | 0xffff;
        c.g[T1][0] = 0xffff_ffff_fff0_0000 | 0xfff;
        let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
        fe_render_grman(c, V1);
        c.g[V0][0] = sx((top as u32).wrapping_add(0x14));
        c.g[T5][0] = 0xffff_ffff_ffff_fc1f;
        Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).sw(c, c.g[V0][0]);
        c.g[T6][0] = 0xffff_ffff_ffff_ff83;
        let grman_v1 = r32(c, V1);
        c.g[V0][0] = Addr(grman_v1.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
        c.g[A1][0] = sx(FE_MODULE_RENDER_DEFAULT_GS);
        let dst = r32(c, V0);
        fe_render_copy_gs(c, FE_MODULE_RENDER_DEFAULT_GS, dst, T7, V1, A0);
        fe_render_grman(c, A1);
        // g22 = 0; g23 = 1; g20 = 3; g12 = 0xd; alpha = 0x15; g2 = 5.
        let masks = [
            (V1, 0x4, A2, None),
            (A0, 0x4, T0, Some(T3)),
            (V1, 0x4, A3, Some(T4)),
            (A0, 0x4, T1, None),
            (V1, 0x8, T5, None),
            (A0, 0x4, T6, None),
        ];
        let ors = [0, 0, 0, 0xd000, 0x2a0, 0x14];
        for (&(reg, off, mask, or_reg), &or_imm) in masks.iter().zip(ors.iter()) {
            let gm = r32(c, A1);
            c.g[reg][0] = Addr(gm.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
            let gs = r32(c, reg);
            c.g[V0][0] = Addr(gs.wrapping_add(off)).lw(c);
            c.g[V0][0] &= c.g[mask][0];
            if let Some(o) = or_reg {
                c.g[V0][0] |= c.g[o][0];
            }
            c.g[V0][0] |= or_imm;
            Addr(gs.wrapping_add(off)).sw(c, c.g[V0][0]);
        }

        // UIEngine_RenderStateMgr(self->ui); the rider previews.
        let this = r32(c, S1);
        c.g[A0][0] = Addr(this.wrapping_add(0xc)).lw(c);
        call_ptr(c, 0x0039_8018, 0x001a_2700)?;
        let this = r32(c, S1);
        c.g[A0][0] = sx(this.wrapping_add(0xb0));
        Callee {
            addr: 0x001a_0598,
            f: ee::s03::FEModule_RenderRiderPreviews,
        }
        .call(c, 0x001a_2708)?;

        // Pop the state, end the frame (vslots 32, 22).
        fe_render_pop(c);
        fe_render_vcall(c, A0, A1, V1, V0, 0x100, 0x001a_272c)?;
        fe_render_grman(c, A1);
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xb0, 0x001a_2744)?;

        // UIEngine_Render(self->ui), vslot 20, the audio update.
        let this = r32(c, S1);
        c.g[A0][0] = Addr(this.wrapping_add(0xc)).lw(c);
        call_ptr(c, 0x0039_7df8, 0x001a_274c)?;
        fe_render_grman(c, A1);
        fe_render_vt(c, A1, V0);
        fe_render_vcall(c, A1, V0, A0, V1, 0xa0, 0x001a_2764)?;
        call_ptr(c, 0x0028_b180, 0x001a_276c)?;
        let gp = r32(c, GP);
        c.lwc1(12, gp.wrapping_sub(FE_MODULE_RENDER_AUDIO_STEP_GP));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_5bf8, 0x001a_277c)?;
        c.g[V0][0] = 0x1;
        fe_render_epilogue(c, frame)
    })
}

/// `GrMan->top--` with GrMan in `a0` and its vtable in `a1`.
pub(crate) fn fe_render_pop(c: &mut Cpu) {
    fe_render_grman(c, A0);
    let grman = r32(c, A0);
    let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    fe_render_vt(c, A0, A1);
    c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
    Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).sw(c, c.g[V0][0]);
}

/// The `D_004A19CC` path of [`fe_module_render`] (GrMan in `a1`): screen
/// size (vslots 10, 11), begin the frame, push the state, an orthographic
/// view of the screen with the identity matrix, alpha mode 9, a black
/// 4-vertex strip over the screen (vslot 71), end the frame, pop, vslot 20.
pub(crate) fn fe_render_backdrop(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    fe_render_vt(c, A1, V1);
    fe_render_vcall(c, A1, V1, A0, V0, 0x50, 0x001a_22dc)?;
    fe_render_grman(c, A1);
    fe_render_u2f(c, 21);
    fe_render_vt(c, A1, V1);
    c.cycles += 6;
    fe_render_vcall(c, A1, V1, A0, V0, 0x58, 0x001a_2320)?;
    fe_render_grman(c, A1);
    fe_render_u2f(c, 20);
    fe_render_vt(c, A1, V0);
    c.cycles += 57;
    fe_render_vcall(c, A1, V0, A0, V1, 0xa8, 0x001a_2364)?;
    fe_render_grman(c, A1);
    fe_render_vt(c, A1, V0);
    fe_render_vcall(c, A1, V0, A0, V1, 0xf8, 0x001a_237c)?;

    // Push: top[1] = top[0]; top++. Orthographic(0, 0, 0, 0, w, h, 0, 1).
    fe_render_grman(c, A0);
    let grman = r32(c, A0);
    c.g[A1][0] = 0;
    c.mtc1(0, 12);
    c.g[A2][0] = 0;
    let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    c.g[V0][0] = top;
    c.mov_s(15, 20);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 17);
    c.mov_s(14, 21);
    fe_render_copy_gs(c, top as u32, (top as u32).wrapping_add(0x14), V1, A3, T0);
    c.mov_s(16, 12);
    c.mov_s(13, 12);
    let top = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    fe_render_grman(c, A3);
    c.g[V0][0] = sx((top as u32).wrapping_add(0x14));
    Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).sw(c, c.g[V0][0]);
    fe_render_vt(c, A3, V1);
    fe_render_vcall(c, A3, V1, A0, V0, 0xd0, 0x001a_23f4)?;
    // SetViewMatrix(t4mat_Identity).
    fe_render_grman(c, A2);
    c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
    fe_render_vt(c, A2, V0);
    fe_render_vcall(c, A2, V0, A0, V1, 0x110, 0x001a_2414)?;

    // top->alpha = 9.
    fe_render_grman(c, A0);
    c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = 0x3;
    let grman = r32(c, A0);
    c.g[V1][0] = Addr(grman.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    let gs = r32(c, V1);
    c.g[V0][0] = Addr(gs.wrapping_add(0x8)).lw(c);
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= 0x120;
    Addr(gs.wrapping_add(0x8)).sw(c, c.g[V0][0]);
    // The compiler's empty countdown loop (a2 = 3 .. -1).
    loop {
        c.cycles += 7;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        let more = c.g[A2][0] != c.g[A3][0];
        c.g[A1][0] = 0x80;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001a_2438);
        }
    }

    // Four vertices: rgba (0, 0, 0, 0x80), q = 1.
    c.cycles += 5;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[V1][0] = c.g[SP][0];
    let s = sp(c);
    c.g[A0][0] = sx(s.wrapping_add(0xc0));
    Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A1][0]);
    loop {
        c.cycles += 8;
        let v = r32(c, V1);
        Addr(v.wrapping_add(0x10)).sw(c, 0);
        Addr(v.wrapping_add(0x14)).sw(c, 0);
        Addr(v.wrapping_add(0x18)).sw(c, 0);
        c.swc1(0, v.wrapping_add(0x8));
        c.g[V1][0] = sx(v.wrapping_add(0x30));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A1][0]);
        if event_due(c) {
            deliver_event_here(c, 0x001a_2468);
        }
    }

    // Positions (0,0), (0,w), (h,0)... built in the sp+0xc0 scratch vector.
    c.cycles += 58;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    let s = sp(c);
    let p = s.wrapping_add(0xc0);
    Addr(p).sw(c, 0);
    c.g[A1][0] = 0x4;
    Addr(p.wrapping_add(4)).sw(c, 0);
    c.g[A2][0] = c.g[SP][0];
    Addr(p.wrapping_add(8)).sw(c, 0);
    c.g[A3][0] = 0;
    c.swc1(0, p.wrapping_add(0xc));
    fe_render_grman(c, T0);
    c.lq(V0, p);
    Addr(p).sw(c, 0);
    c.swc1(21, p.wrapping_add(4));
    Addr(p.wrapping_add(8)).sw(c, 0);
    c.sq(V0, s.wrapping_add(0x20));
    c.lq(V1, p);
    Addr(p).sw(c, 0);
    Addr(p.wrapping_add(4)).sw(c, 0);
    c.swc1(20, p.wrapping_add(8));
    c.sq(V1, s.wrapping_add(0x50));
    c.lq(A0, p);
    c.swc1(21, p.wrapping_add(4));
    Addr(p).sw(c, 0);
    c.sq(A0, s.wrapping_add(0x80));
    c.lq(V0, p);
    c.sq(V0, s.wrapping_add(0xb0));
    // top->tex = -1.
    let gm = r32(c, T0);
    c.g[V1][0] = Addr(gm.wrapping_add(FE_MODULE_RENDER_GS_TOP)).lw(c);
    Addr(r32(c, V1).wrapping_add(0x10)).sh(c, c.g[T1][0]);
    // DrawStrip(4, v, 0).
    fe_render_grman(c, T0);
    fe_render_vt(c, T0, V0);
    fe_render_vcall(c, T0, V0, A0, V1, 0x238, 0x001a_2514)?;
    fe_render_grman(c, A1);
    fe_render_vt(c, A1, V0);
    fe_render_vcall(c, A1, V0, A0, V1, 0xb0, 0x001a_252c)?;
    fe_render_grman(c, A1);
    fe_render_vt(c, A1, V0);
    fe_render_vcall(c, A1, V0, A0, V1, 0x100, 0x001a_2544)?;
    fe_render_pop(c);
    fe_render_vcall(c, A0, A1, V1, V0, 0xa0, 0x001a_2568)
}

/// Restore s0/s1/ra/f20/f21 and return.
pub(crate) fn fe_render_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    frame.restore(c, S0, 0xf0);
    frame.restore(c, S1, 0xe0);
    frame.restore_ra(c, 0xd0);
    frame.restore_fpr(c, 21, 0x108);
    frame.restore_fpr(c, 20, 0x100);
    frame.close(c);
    Ok(())
}

/// `FEUICustom_dtor(this, flags)` (0x1dcc78): vtable at +4, delete if `flags & 1`.
pub fn feui_custom_dtor(c: &mut Cpu) {
    oc437_dtor_flag_delete(c, 0x47_0000, 0x22d8, 0x4, 0x001d_cc9c)
}

/// `void cRealFE::loadStartState(cRealFE* self)` (0x1a2c40, `FEModule_v08`).
///
/// Pushes the background front-end state, then, by `cRealFE_mStartState`
/// (`gp-0x172c`): 0 pushes the title state, 0x27 the lodge state (and clears
/// the start state), 0x34 the re-login state (closing the comm channel,
/// resetting voip/lobby and the two profiles, then clearing the start state);
/// any other value pushes nothing.
pub fn cfe_real_fe_load_start_state(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a2e10: {
            'b_1a2e0c: {
                'b_1a2e08: {
                    'b_1a2d5c: {
                        c.cycles += 30;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                        c.g[A1][0] = 0x460000;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = 0u64;
                        c.g[S1][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[A3][0] = 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1358));
                        c.g[A0][0] = 0x48;
                        call_ptr(c, 0x00317d70, 0x001a2c6c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039e2a0, 0x001a2c7c)?;
                        c.g[V1][0] = 0x470000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x43a8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        let t = c.g[V0][0] as u32;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        call_ptr(c, t, 0x001a2c98)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f238, 0x001a2ca8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x172c));
                        c.g[V0][0] = 0x34;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 53i64) as u64;
                        if t {
                            break 'b_1a2d5c;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_1a2e0c;
                            } else {
                                'b_1a2d18: {
                                    'b_1a2cd8: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V0][0] = 0x27;
                                        if t {
                                            break 'b_1a2cd8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1a2d18;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                if t {
                                                    break 'b_1a2e10;
                                                } else {
                                                    break 'b_1a2cd8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 16;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = 0x4c;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d70));
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0u64);
                                    call_ptr(c, 0x00317d70, 0x001a2cf4)?;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00194738, 0x001a2d00)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001a2d10)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1a2e0c;
                                    } else {
                                        break 'b_1a2d18;
                                    }
                                }
                                c.cycles += 17;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = 0x460000;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1188));
                                c.g[A0][0] = 0x58;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x001a2d38)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x001f3700, 0x001a2d44)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x001a2d54)?;
                                let t = true;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), 0u64);
                                if t {
                                    break 'b_1a2e08;
                                } else {
                                    break 'b_1a2d5c;
                                }
                            }
                        }
                    }
                    c.cycles += 43;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1370));
                    c.g[A0][0] = 0x6d4;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1718), 0u64);
                    call_ptr(c, 0x00317d70, 0x001a2d78)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001d5038, 0x001a2d84)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f400, 0x001a2d94)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c));
                    call_ptr(c, 0x00255cc8, 0x001a2da4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    call_ptr(c, 0x00266e88, 0x001a2dac)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                    call_ptr(c, 0x00266df8, 0x001a2db4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0025b6d8, 0x001a2dc0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x7;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x001a2dd0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014eca0, 0x001a2ddc)?;
                    call_ptr(c, 0x0014dc80, 0x001a2de4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014de28, 0x001a2df0)?;
                    call_ptr(c, 0x0014dc80, 0x001a2df8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014de28, 0x001a2e04)?;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), 0u64);
                    break 'b_1a2e08;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1a2e0c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1a2e10;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void FEModule_Enter(FEModule* self)` (0x1a1dc0): when the module was left
/// active (`gp-0x1738`) resets three BE interface slots and clears a 0x20-byte block, then
/// clears the module counters, calls the screen's activation methods and
/// `FEFlowState_Init`.
pub fn fe_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a1e2c: {
            c.cycles += 6;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1738));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_1a1e2c;
            } else {
                c.cycles += 21;
                c.g[RA][0] = sx(0x001a1de0);
                call_ptr(c, 0x0014dc80, 0x001a1de0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a1dec);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014de28, 0x001a1dec)?;
                c.g[RA][0] = sx(0x001a1df4);
                call_ptr(c, 0x0014dc80, 0x001a1df4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a1e00);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014de28, 0x001a1e00)?;
                c.g[RA][0] = sx(0x001a1e08);
                call_ptr(c, 0x0014dc80, 0x001a1e08)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a1e14);
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014de28, 0x001a1e14)?;
                c.g[A0][0] = 0x530000;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                c.g[RA][0] = sx(0x001a1e28);
                c.g[A2][0] = 0x20;
                call_ptr(c, 0x003e6448, 0x001a1e28)?;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1738), 0u64);
                break 'b_1a1e2c;
            }
        }
        c.cycles += 36;
        c.g[V0][0] = 0xb0000;
        c.g[A0][0] = 0u64;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5adc), 0u64);
        c.g[RA][0] = sx(0x001a1e44);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5ad8), 0u64);
        call_ptr(c, 0x0014de68, 0x001a1e44)?;
        c.g[RA][0] = sx(0x001a1e4c);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        call_ptr(c, 0x00369ff8, 0x001a1e4c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[RA][0] = sx(0x001a1e58);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
        call_ptr(c, 0x00195e58, 0x001a1e58)?;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x1234), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x14e8), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x12d8), 0u64);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x78));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001a1e8c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, t, 0x001a1e8c)?;
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1d90));
        c.g[RA][0] = sx(0x001a1e98);
        c.g[A1][0] = 0x56;
        Callee {
            addr: 0x001a0648,
            f: ee::s03::FEFlowState_Init,
        }
        .call(c, 0x001a1e98)?;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001a1eac);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001a1eac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cOVSound::playSound(cOVSound* self, int sel)` (0x1a2f70, `OVUISound_v02`).
///
/// Plays a front-end menu sound for `sel` 1..=6: a jump table (0x4613a0) picks
/// the case, which calls `SSXAudioSystem_GetInstance()` and then
/// `SSXAudioSystem_PlayFESound(audio, id)` with id 0xb, 0xc, 0xa, 0xd, 9, 9.
/// Any other `sel` does nothing. The resume entries (`pc != entry`) stay with
/// the generated `__entries`. A table entry equal to the function's own entry
/// (a restart) cannot occur in the game's constant table and is not handled.
pub fn ovui_sound_v02(c: &mut Cpu) {
    if c.pc != 0x001a_2f70 {
        return ee::s03::f_001a2f70__entries(c);
    }
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V1][0] < 6) as u64;
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !out_of_range {
            c.cycles += 7;
            c.g[V0][0] = 0x46_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            // (cycles of the case head, returns of its two calls, sound id,
            // falls into the common tail that charges one more cycle).
            let (cycles, ret_get, ret_play, id, via_tail) = match target {
                0x001a_2fa0 => (7, 0x001a_2fa8, 0x001a_2fb4, 0xb, false),
                0x001a_2fbc => (7, 0x001a_2fc4, 0x001a_2fd0, 0xc, false),
                0x001a_2fd8 => (7, 0x001a_2fe0, 0x001a_2fec, 0xa, false),
                0x001a_2ff4 => (7, 0x001a_2ffc, 0x001a_3008, 0xd, false),
                0x001a_3010 => (7, 0x001a_3018, 0x001a_3024, 0x9, false),
                0x001a_302c => (5, 0x001a_3034, 0x001a_3040, 0x9, true),
                _ => {
                    c.jump_out(target, 0x001a_2f70, 0x001a_304c);
                    return Ok(());
                }
            };
            c.cycles += cycles;
            call_ptr(c, 0x0028_b180, ret_get)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = id;
            call_ptr(c, 0x0029_4f78, ret_play)?;
            if via_tail {
                c.cycles += 1;
            }
            c.g[RA][0] = c.ld(sp(c));
        } else {
            c.cycles += 1;
            c.g[RA][0] = c.ld(sp(c));
        }
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// Offset of the first `FEModule` member constructed after the `cFE` base
/// (0xb5ab0), the sub-object at 0xb5abc, and the three words cleared last.
pub(crate) const FE_MODULE_CLEAR_BASE: u32 = 0xb_0000;

/// `FEModule* FEModule_ctor(FEModule* this)` (0x1a1ce8).
///
/// Runs the `cFE` base constructor, installs the vtable (0x46d7c0) at
/// `this`, constructs the sub-objects at `this+0xb5ab0` (0x263660) and
/// `this+0xb5abc` (0x263998), clears the words at `this+0xb5ad4/d8/dc` and
/// returns `this`.
pub fn fe_module_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001956f8, 0x001a_1cfc)?;
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = FE_MODULE_CLEAR_BASE as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2840));
        c.g[A0][0] |= 0x5ab0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        call_ptr(c, 0x0026_3660, 0x001a_1d18)?;
        c.g[A0][0] = FE_MODULE_CLEAR_BASE as u64;
        c.g[A0][0] |= 0x5abc;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x0026_3998, 0x001a_1d28)?;
        c.g[V1][0] = FE_MODULE_CLEAR_BASE as u64;
        c.g[V0][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore_ra(c, 0);
        let this_end = c.g[S0][0] as u32;
        c.sw(this_end.wrapping_add(0x5adc), 0);
        c.sw(this_end.wrapping_add(0x5ad4), 0);
        c.sw(this_end.wrapping_add(0x5ad8), 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc778_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc789_install_fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as RET;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn fe_module_set_player_pad_mask_matches() {
        check(
            ee::s03::f_001a1cb8__generated,
            fe_module_set_player_pad_mask,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001a_1cb8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100);
                let slot = (seed % 9) as i32 - 4;
                // Odd seeds pass a slot with the sign extension hidden in a wider
                // value, which the original re-sign-extends from the low byte.
                let raw = if seed % 2 == 0 { slot } else { 0x100 + slot };
                c.g[A1][0] = sx(raw as u32);
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn fe_module_get_player_pad_mask_matches() {
        check(
            ee::s03::f_001a1cd0__generated,
            fe_module_get_player_pad_mask,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x001a_1cd0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100);
                let slot = (seed % 9) as i32 - 4;
                c.g[A1][0] = sx(slot as u32);
                let at = (DATA + 0x100) as i32 + slot + 8;
                c.bus.write8(at as u32, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn feui_sound_ctor_matches() {
        check(
            ee::s03::f_001a2e20__generated,
            feui_sound_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a2e_20, 0);
                let obj = DATA + 0x400;
                fill(c, &mut r, obj, 0x10);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn ovui_sound_ctor_matches() {
        check(
            ee::s03::f_001a2f38__generated,
            ovui_sound_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a2f_38, 0);
                let obj = DATA + 0x400;
                fill(c, &mut r, obj, 0x10);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    /// Scrambled registers with a game-like `gp` (no fake call hooks: these
    /// tests only use callees that are already ported or reach no unknown
    /// code).
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x31_0102 ^ ((seed as u64) << 9) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// `str_hash(s)`: the ELF string hash, used to build the hash-keyed tests.
    pub(crate) fn str_hash(c: &mut Cpu, s: u32) -> u64 {
        c.g[A0][0] = sx(s);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    /// A two-word object at `obj` whose `+off` vtable slot calls the fake.
    pub(crate) fn fake_slot(c: &mut Cpu, r: &mut Rng, obj: u32, off: u32, slot: u32) {
        let vt = vtable(c, r, &[slot]);
        c.bus.write32(obj + off, vt);
    }

    #[test]
    fn feuicustom_v03_matches() {
        check(
            ee::s03::f_001a07c8__generated,
            feuicustom_v03,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a_07c8, 1);
                let hash = str_hash(c, 0x0045_fff8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.g[A2][0] = if seed % 2 == 0 { hash } else { hash ^ 1 };
                let obj = DATA + 0x200;
                fake_slot(c, &mut r, obj, 0x8, 0xc0);
                c.bus.write32(DATA + 0x10, obj);
            },
        );
    }

    #[test]
    fn femodule_try_exit_matches() {
        check(
            ee::s03::f_001a2128__generated,
            femodule_try_exit,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1a_2128);
                c.g[A0][0] = sx(DATA);
                // `AppModule_TryExit` compares a float at self+4 with the global
                // at gp-0x51b4, so both outcomes are reachable.
                let t = if seed % 2 == 0 { 0.5f32 } else { 1.5f32 };
                c.bus.write32(DATA + 0x4, t.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x51b4), 1.0f32.to_bits());
                c.bus.write32(
                    DATA + 0xb0000 + 0x5ad4,
                    if seed % 3 == 0 { 0 } else { DATA + 0x100 },
                );
                c.bus.write32(DATA + 0xb0000 + 0x5ad8, 0xdead);
                c.bus.write32(DATA + 0xb0000 + 0x5adc, 0xdead);
                c.bus.write32(GAME_GP.wrapping_sub(0x1724), 0xdead);
                c.bus.write32(GAME_GP + 0x1d90, 0xdead);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn fe_module_dtor_matches() {
        check(
            ee::s03::f_001a1d50__generated,
            fe_module_dtor,
            24,
            &|c, seed| {
                alloc_base(c, seed, 0x1a1d_50);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(GAME_GP - 0x854, 0);
                c.bus.write32(DATA + 0xb5aa0, 0);
            },
        );
    }

    #[test]
    fn fe_module_unload_matches() {
        check(
            ee::s03::f_001a2190__generated,
            fe_module_unload,
            16,
            &|c, seed| {
                machine(c, seed, 0x1a2190, 8);
                be_library(c);
                heap_init_2(c);
                world(c);
                let app = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x854), app);
                w(c, app + 0x10d8, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0x98, 0, FAKE);
                c.g[A0][0] = (DATA + 0x2_0000) as u64;
            },
        );
    }

    #[test]
    fn femodule_unload_anim_data_matches() {
        check(
            ee::s03::f_001a2bc0__generated,
            femodule_unload_anim_data,
            SEEDS_2,
            &|c, seed| {
                base_3(c, seed, 0x2bc0, 0);
                let map = DATA + 0x800;
                let banks = DATA + 0x900;
                c.bus.write32(GAME_GP + 0xd8c, map);
                c.bus.write32(GAME_GP + 0xd08, banks);
                let e = DATA + 0xa00;
                c.bus.write32(banks, if seed % 3 == 0 { 0 } else { e });
            },
        );
    }

    pub(crate) const SELF2: u32 = DATA + 0x19000;

    #[test]
    fn feui_custom_ctor_matches() {
        check(
            ee::s03::f_001a0720__generated,
            feui_custom_ctor,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1a0720, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, 0x0046_0b90, 0);
                w(c, OBJ2, if seed % 3 == 0 { 0 } else { SELF2 });
            },
        );
    }

    /// With no movie playing the wrapper is the original; while a logo plays
    /// it is marked skippable. (`+0xb5adc` set: the update returns at once,
    /// so only the flag differs.)
    #[test]
    fn boot_movies_update_makes_the_logos_skippable() {
        const FE: u32 = 0x0010_0000;
        for playing in [0u32, 0x0080_0000] {
            let run = |f: GuestFn| {
                let mut c = crate::difftest::fresh();
                c.pc = 0x001a_27a0;
                c.g[A0][0] = FE as u64;
                c.bus.write32(FE + 0xb5adc, 1);
                c.bus.write32(FE + BOOT_PLAYER, playing);
                f(&mut c);
                (c.pc, c.bus.read32(FE + BOOT_SKIPPABLE))
            };
            let (pc_g, flag_g) = run(ee::s03::f_001a27a0__generated);
            let (pc_h, flag_h) = run(boot_movies_update);
            assert_eq!(pc_g, pc_h);
            assert_eq!(flag_g, 0);
            assert_eq!(flag_h, (playing != 0) as u32, "player {playing:#x}");
        }
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::LOG_RESULTS;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's fakes (word `OC381_MAGIC`).
    pub(crate) const OC381_CTRL: u32 = DATA + 0x7_8000;

    pub(crate) const OC381_MAGIC: u32 = 0x0381_f00d;

    /// `OC381_CTRL + 4` = 1: the `AppModule_RenderFade` fake returns non-zero.
    pub(crate) const OC381_FADE_ON: u32 = OC381_CTRL + 4;

    /// `OC381_CTRL + 8` = 1: GrMan vslot 17 (ready) returns 0.
    pub(crate) const OC381_NOT_READY: u32 = OC381_CTRL + 8;

    pub(crate) const OC381_GRMAN: u32 = DATA + 0x1_0000;

    pub(crate) const OC381_VT: u32 = DATA + 0x2_0000;

    pub(crate) const OC381_TOP: u32 = DATA + 0x3_0000;

    pub(crate) const OC381_SELF: u32 = DATA + 0x4_0000;

    pub(crate) const OC381_VIDEO: u32 = DATA + 0x6_0000;

    /// The GrMan vtable entries FEModule_Render calls; entry i jumps to
    /// `FAKE + 0x10 * i`.
    pub(crate) const OC381_SLOTS: [u32; 12] = [
        0x50, 0x58, 0x88, 0xa0, 0xa8, 0xb0, 0xc8, 0xd0, 0xf8, 0x100, 0x110, 0x238,
    ];

    /// The fake behind vslot 17 (entry 0x88).
    pub(crate) const OC381_FAKE_READY: u32 = FAKE + 0x20;

    macro_rules! oc381_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC381_CTRL) != OC381_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if $addr == 0x0023_1d60u32 && c.bus.read32(OC381_FADE_ON) != 1 {
                    c.g[V0][0] = 0;
                }
            }
        };
    }

    oc381_gated!(
        oc381_fake_render_fade,
        OC381_PREV_RENDER_FADE,
        0x0023_1d60u32,
        ee::s06::f_00231d60__generated
    );

    oc381_gated!(
        oc381_fake_ui_state_mgr,
        OC381_PREV_UI_STATE_MGR,
        0x0039_8018u32,
        ee::s13::f_00398018__generated
    );

    oc381_gated!(
        oc381_fake_rider_previews,
        OC381_PREV_RIDER_PREVIEWS,
        0x001a_0598u32,
        ee::s03::f_001a0598__generated
    );

    oc381_gated!(
        oc381_fake_ui_render,
        OC381_PREV_UI_RENDER,
        0x0039_7df8u32,
        ee::s13::f_00397df8__generated
    );

    oc381_gated!(
        oc381_fake_audio_update,
        OC381_PREV_AUDIO_UPDATE,
        0x0028_5bf8u32,
        ee::s07::f_00285bf8__generated
    );

    oc381_gated!(
        oc381_fake_audio_instance,
        OC381_PREV_AUDIO_INSTANCE,
        0x0028_b180u32,
        ee::s07::f_0028b180__generated
    );

    pub(crate) fn oc381_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00231D60),
            oc381_fake_render_fade,
            &OC381_PREV_RENDER_FADE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00398018),
            oc381_fake_ui_state_mgr,
            &OC381_PREV_UI_STATE_MGR,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A0598),
            oc381_fake_rider_previews,
            &OC381_PREV_RIDER_PREVIEWS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397DF8),
            oc381_fake_ui_render,
            &OC381_PREV_UI_RENDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285BF8),
            oc381_fake_audio_update,
            &OC381_PREV_AUDIO_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc381_fake_audio_instance,
            &OC381_PREV_AUDIO_INSTANCE,
        );
    }

    /// GrMan's methods: logged fakes returning sign-extended results (the
    /// screen size can be "negative" for the unsigned-to-float path), and
    /// vslot 17 returns 0 on request.
    pub(crate) fn oc381_miss(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        c.g[V0][0] = if target == OC381_FAKE_READY && c.bus.read32(OC381_NOT_READY) == 1 {
            0
        } else {
            sx(c.g[V0][0] as u32)
        };
    }

    /// Paths by `seed % 8`: 0 not ready; 1 fading; 2 a video playing
    /// (`VidEnginePlayer_Render` runs for real on an idle player); 3 the
    /// black backdrop with screen sizes >= 2^31; 4 the backdrop with small
    /// sizes; 5-7 the module render. The fade counter is 0 or not.
    #[test]
    fn fe_module_render_matches() {
        oc381_install_fakes();
        covered(
            ee::s03::f_001a2208__generated,
            fe_module_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_2208, 14);
                c.hooks.miss = oc381_miss;
                c.pc = 0x001a_2208;
                c.bus.write32(OC381_CTRL, OC381_MAGIC);
                let path = seed % 8;
                c.bus.write32(OC381_NOT_READY, (path == 0) as u32);
                c.bus.write32(OC381_FADE_ON, (path == 1) as u32);

                c.bus
                    .write32(GAME_GP - FE_MODULE_RENDER_GRMAN_GP, OC381_GRMAN);
                c.bus
                    .write32(OC381_GRMAN + FE_MODULE_RENDER_VTABLE, OC381_VT);
                for (i, &off) in OC381_SLOTS.iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(OC381_VT + off, delta as u16);
                    c.bus.write32(OC381_VT + off + 4, FAKE + 0x10 * i as u32);
                }
                let top = OC381_TOP + 4 * (seed / 8 % 4);
                c.bus.write32(OC381_GRMAN + FE_MODULE_RENDER_GS_TOP, top);
                fill(c, &mut r, top, 16);
                fill(c, &mut r, FE_MODULE_RENDER_DEFAULT_GS, 5);

                let this = OC381_SELF;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x18);
                let fade = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(this + 0xb_0000 + FE_MODULE_RENDER_FADE, fade);
                let video = if path == 2 { OC381_VIDEO } else { 0 };
                c.bus
                    .write32(this + 0xb_0000 + FE_MODULE_RENDER_VIDEO, video);
                c.bus.write32(OC381_VIDEO + 0x38, 0);
                let backdrop = if path == 3 || path == 4 {
                    r.next_u32() | 1
                } else {
                    0
                };
                c.bus
                    .write32(GAME_GP - FE_MODULE_RENDER_BACKDROP_GP, backdrop);
                // Screen width/height: fake calls 2 and 3 on the backdrop path.
                if path == 3 {
                    c.bus.write32(LOG_RESULTS + 8, r.next_u32() | 0x8000_0000);
                    c.bus.write32(LOG_RESULTS + 12, r.next_u32() | 0x8000_0000);
                } else if path == 4 {
                    c.bus.write32(LOG_RESULTS + 8, 1 + r.next_u32() % 1024);
                    c.bus.write32(LOG_RESULTS + 12, 1 + r.next_u32() % 1024);
                }
            },
        );
    }

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc437_covered;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc437_dtor_flags;

    #[test]
    fn feui_custom_dtor_matches() {
        oc437_covered(
            ee::s04::f_001dcc78__generated,
            feui_custom_dtor,
            0x001d_cc78,
            1,
            &oc437_dtor_flags,
        );
    }

    use crate::difftest::put_words;
    pub(crate) use crate::game::fe::debugfe::tests::oc793_machine;

    pub(crate) use crate::game::fe::festatecharsetup::tests::oc787_arm;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC787_AUX;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC787_CTRL;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC787_MAGIC;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC787_OBJ;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc778_arm;

    pub(crate) use crate::game::sound::bankmonitor::tests::oc789_arm;

    /// `FEModule_v08` (`cRealFE::loadStartState`): the start state 0, 0x27,
    /// 0x34 (with the real `0x255cc8` reaching its vtable call or not), below
    /// and above the range, and the unwinds of its calls.
    #[test]
    fn cfe_real_fe_load_start_state_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s03::f_001a2c40__generated,
            cfe_real_fe_load_start_state,
            300,
            &|c, seed| {
                machine(c, seed, 0x778b, 14);
                oc778_arm(c);
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0xc, DATA + 0x3_0000);
                // The state vtable image the background state's ctor-call reads.
                c.bus.write16(0x0046_bc78, 0x20);
                c.bus.write32(0x0046_bc7c, fakes::FAKE + 0x10);
                let state = [0x34u32, 0, 0x27, 5, 0x40][(seed % 5) as usize];
                c.bus.write32(GAME_GP - 0x172c, state);
                // State 0x34: `0x255cc8(table, idx)` with the table entry set or not.
                let comm = DATA + 0x4_0000;
                let table = DATA + 0x5_0000;
                let obj = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let idx = (seed / 5) % 2;
                c.bus.write32(GAME_GP - 0x204, comm);
                c.bus.write32(comm + 0x6c, idx);
                c.bus.write32(GAME_GP - 0x238, table);
                c.bus
                    .write32(table + 4 * idx, if (seed / 10) % 2 == 0 { obj } else { 0 });
                c.bus.write32(obj, 1);
                c.bus.write32(obj + 8, vt);
                c.bus.write16(vt + 0x10, 0x20);
                c.bus.write32(vt + 0x14, fakes::FAKE + 0x20);
            },
        );
    }

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc787_mail(addr: u32) -> u32 {
        DATA + 0x32_1000 + (addr & 0xffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc787_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc787_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    macro_rules! oc787_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC787_CTRL) != OC787_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc787_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc787_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc787_fakes!(
        (
            oc787_fk_14dc80,
            OC787_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc787_fk_14dd58,
            OC787_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc787_fk_14eec8,
            OC787_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc787_fk_15e050,
            OC787_PREV_15E050,
            ee::s01::OVERRIDE_0015E050,
            ee::s01::f_0015e050__generated,
            0x0015_e050
        ),
        (
            oc787_fk_14de28,
            OC787_PREV_14DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc787_fk_14de68,
            OC787_PREV_14DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014_de68
        ),
        (
            oc787_fk_182ec0,
            OC787_PREV_182EC0,
            ee::s02::OVERRIDE_00182EC0,
            ee::s02::f_00182ec0__generated,
            0x0018_2ec0
        ),
        (
            oc787_fk_1831d0,
            OC787_PREV_1831D0,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0
        ),
        (
            oc787_fk_195e58,
            OC787_PREV_195E58,
            ee::s02::OVERRIDE_00195E58,
            ee::s02::f_00195e58__generated,
            0x0019_5e58
        ),
        (
            oc787_fk_1a0648,
            OC787_PREV_1A0648,
            ee::s03::OVERRIDE_001A0648,
            ee::s03::f_001a0648__generated,
            0x001a_0648
        ),
        (
            oc787_fk_287f00,
            OC787_PREV_287F00,
            ee::s07::OVERRIDE_00287F00,
            ee::s07::f_00287f00__generated,
            0x0028_7f00
        ),
        (
            oc787_fk_28b1d8,
            OC787_PREV_28B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8
        ),
        (
            oc787_fk_2948a0,
            OC787_PREV_2948A0,
            ee::s07::OVERRIDE_002948A0,
            ee::s07::f_002948a0__generated,
            0x0029_48a0
        ),
        (
            oc787_fk_296e20,
            OC787_PREV_296E20,
            ee::s08::OVERRIDE_00296E20,
            ee::s08::f_00296e20__generated,
            0x0029_6e20
        ),
        (
            oc787_fk_297438,
            OC787_PREV_297438,
            ee::s08::OVERRIDE_00297438,
            ee::s08::f_00297438__generated,
            0x0029_7438
        ),
        (
            oc787_fk_29b3c0,
            OC787_PREV_29B3C0,
            ee::s08::OVERRIDE_0029B3C0,
            ee::s08::f_0029b3c0__generated,
            0x0029_b3c0
        ),
        (
            oc787_fk_2ad5f0,
            OC787_PREV_2AD5F0,
            ee::s08::OVERRIDE_002AD5F0,
            ee::s08::f_002ad5f0__generated,
            0x002a_d5f0
        ),
        (
            oc787_fk_2ad848,
            OC787_PREV_2AD848,
            ee::s08::OVERRIDE_002AD848,
            ee::s08::f_002ad848__generated,
            0x002a_d848
        ),
        (
            oc787_fk_2ade88,
            OC787_PREV_2ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc787_fk_2b6900,
            OC787_PREV_2B6900,
            ee::s08::OVERRIDE_002B6900,
            ee::s08::f_002b6900__generated,
            0x002b_6900
        ),
        (
            oc787_fk_317670,
            OC787_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc787_fk_369ff8,
            OC787_PREV_369FF8,
            ee::s12::OVERRIDE_00369FF8,
            ee::s12::f_00369ff8__generated,
            0x0036_9ff8
        ),
        (
            oc787_fk_39d860,
            OC787_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc787_fk_397948,
            OC787_PREV_397948,
            ee::s13::OVERRIDE_00397948,
            ee::s13::f_00397948__generated,
            0x0039_7948
        ),
        (
            oc787_fk_3a0c70,
            OC787_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc787_fk_3d9a40,
            OC787_PREV_3D9A40,
            ee::s14::OVERRIDE_003D9A40,
            ee::s14::f_003d9a40__generated,
            0x003d_9a40
        ),
        (
            oc787_fk_3e6448,
            OC787_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    pub(crate) const OC787_VT: u32 = DATA + 0xe_0000;

    pub(crate) const OC787_VT2: u32 = DATA + 0xe_1000;

    /// A vtable-like record at `at` whose slot `off` is `{delta, fake method}`.
    pub(crate) fn oc787_vtable(c: &mut Cpu, at: u32, off: u32, delta: u16) {
        c.bus.write16(at + off, delta);
        c.bus.write32(at + off + 4, fakes::FAKE);
    }

    /// `FEModule_Enter`: the "left active" flag at `gp-0x1738` set or clear
    /// (the three interface resets and the 0x20-byte clear), two vtables whose
    /// methods are fake calls, and the unwinds of every call.
    #[test]
    fn fe_module_enter_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s03::f_001a1dc0__generated,
            fe_module_enter,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787e, 14);
                oc787_arm(c);
                c.pc = 0x001a_1dc0;
                c.g[A0][0] = sx(OC787_OBJ);
                c.bus.write32(
                    GAME_GP - 0x1738,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(GAME_GP - 0x854, OC787_AUX);
                c.bus.write32(GAME_GP - 0x848, OC787_AUX + 0x8000);
                c.bus.write32(OC787_AUX + 0x8000 + 0x8c, r.next_u32());
                c.bus.write32(OC787_AUX + 0x10d8, OC787_VT);
                oc787_vtable(c, OC787_VT, 0x78, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC787_OBJ, OC787_VT2);
                oc787_vtable(c, OC787_VT2, 0x40, (r.next_u32() & 0xfc) as u16);
            },
        );
    }

    /// `OVUISound_v02`: sel 0..=8 (out of range, the six cases), the jump
    /// table's targets, an unknown target (jump out), and unwinds of the
    /// `GetInstance` / `PlayFESound` calls. `PlayFESound` itself runs for real
    /// (its jump table is empty in the test RAM, so it jumps out to a fake).
    #[test]
    fn ovui_sound_v02_matches() {
        oc789_install_fakes();
        let table: [u32; 6] = [
            0x001a_2fa0,
            0x001a_2fbc,
            0x001a_2fd8,
            0x001a_2ff4,
            0x001a_3010,
            0x001a_302c,
        ];
        fakes::covered(
            ee::s03::f_001a2f70__generated,
            ovui_sound_v02,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x789a, 3);
                c.pc = 0x001a_2f70;
                oc789_arm(c);
                put_words(c, 0x0046_13a0, &table);
                let sel = seed % 9;
                c.g[A1][0] = if seed % 13 == 12 {
                    r.next_u32() as u64
                } else {
                    sx(sel)
                };
                if seed % 29 == 7 && (1..=6).contains(&sel) {
                    c.bus.write32(0x0046_13a0 + 4 * (sel - 1), 0x001a_2f90);
                }
            },
        );
    }

    /// `FEModule_ctor`: the real `cFE_cFE` and its helper run with logging
    /// fakes as callees; the two sub-object constructors are fakes; unwinds
    /// at random calls.
    #[test]
    fn fe_module_ctor_matches() {
        covered(
            ee::s03::f_001a1ce8__generated,
            fe_module_ctor,
            120,
            &|c, seed| {
                let _ = oc793_machine(c, seed, 0x001a_1ce8, 14);
                c.g[A0][0] = sx(DATA + 0x4_0000);
            },
        );
    }
}
