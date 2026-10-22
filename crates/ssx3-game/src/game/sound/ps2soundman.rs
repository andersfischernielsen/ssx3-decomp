//! `src/sound/ps2soundman.cpp`: hand-written replacements (moved from the
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
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
pub(crate) use crate::game::be::belibrary::EA_MEM_FREE;
pub(crate) use crate::game::fe::festateriderbio::SOUND_MANAGER_REF_DTOR;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
];

/// Direct calls through the generated function (which dispatches into the
/// replacement when one is installed).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festateriderbio::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateriderbio::callee::SOUND_MANAGER_REF_DTOR;
}

pub(crate) const BUILTIN_DELETE: u32 = 0x0031_7e50;


/// The pointer members [`snd_play_params_dtor`] releases, in the original's
/// order (the first is `+0x20`, each block then re-loads the next one).
pub(crate) const SND_PARAM_PTRS: [u32; 14] = [
    0x20, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
];


/// `void SndPlayParams_dtor(SndPlayParams *p, int flags)` (0x2a97c0): free the
/// parameter's pointer members, then (per `flags`) the sound-manager reference
/// and the object itself.
pub fn snd_play_params_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x0048_0000;
        frame.save(c, S0, 0x40);
        c.g[A3][0] = sx(0x0048_0000u32.wrapping_add(0x3be0));
        frame.save(c, S1, 0x30);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = abi::arg64(c, 1);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let keep = c.g[S1][0] != 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A3][0]);
        let this = r32(c, S0);
        if !keep {
            // The non-deleting ctor-style vtable/offset fixup.
            c.cycles += 16;
            c.g[A2][0] = c.ld(0x0048_3be0);
            c.g[V0][0] = c.ld(0x0048_3be8);
            c.g[V1][0] = c.ld(0x0048_3bf0);
            c.g[A0][0] = c.ld(0x0048_3bf8);
            c.g[A1][0] = Addr(this).lw(c);
            let sp_now = sp(c);
            Addr(sp_now.wrapping_add(8)).sd(c, c.g[V0][0]);
            Addr(sp_now.wrapping_add(0x10)).sd(c, c.g[V1][0]);
            Addr(sp_now).sd(c, c.g[A2][0]);
            Addr(sp_now.wrapping_add(0x18)).sd(c, c.g[A0][0]);
            Addr(c.g[A1][0] as u32).off(4).sw(c, sp_now as u64);
            c.g[V1][0] = Addr(0x0048_3be8).lhu(c);
            c.g[V0][0] = Addr(this).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x88));
            c.g[V0][0] = sx(this.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            Addr(sp_now.wrapping_add(8)).sh(c, c.g[V1][0]);
        }

        // Free the pointer members: the first block also loads `+0x20`.
        c.cycles += 3;
        c.g[A0][0] = Addr(this).off(SND_PARAM_PTRS[0]).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x002a_9840)?;
        }
        for (i, &off) in SND_PARAM_PTRS.iter().enumerate().skip(1) {
            c.cycles += 2;
            c.g[A0][0] = Addr(this).off(off).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, EA_MEM_FREE, 0x002a_9840 + 0x14 * i as u32)?;
            }
        }
        c.g[V0][0] = c.g[S1][0] & 2;

        // The sound-manager reference (`flags & 2`), then the object itself
        // (`flags & 1`).
        c.cycles += 2;
        let no_ref = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_ref {
            c.cycles += 4;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[A1][0] = 0;
            callee::SOUND_MANAGER_REF_DTOR.call(c, 0x002a_995c)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }

        c.cycles += 2;
        let no_delete = c.g[V0][0] == 0;
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, BUILTIN_DELETE, 0x002a_9970)?;
        }

        c.cycles += 4;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}
