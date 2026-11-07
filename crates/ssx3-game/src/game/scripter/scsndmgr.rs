//! `src/scripter/scsndmgr.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::be::be::vt_call;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_3450,
        script_sound_bank_man_ctor,
        "ScriptSoundBankMan_ctor",
    ),
    (
        0x0028_3518,
        script_sound_bank_man_dtor,
        "ScriptSoundBankMan_dtor",
    ),
    (
        0x0028_3580,
        script_sound_bank_man_add_bank,
        "ScriptSoundBankMan_AddBank",
    ),
    (
        0x0028_3610,
        script_sound_bank_man_remove_bank,
        "ScriptSoundBankMan_RemoveBank",
    ),
    (
        0x0028_3658,
        script_sound_bank_man_remove_all,
        "ScriptSoundBankMan_RemoveAll",
    ),
    (
        0x0028_36d8,
        script_sound_bank_man_find_bank,
        "ScriptSoundBankMan_FindBank",
    ),
    (
        0x0028_3720,
        script_sound_bank_man_free_entry,
        "ScriptSoundBankMan_FreeEntry",
    ),
    (
        0x0028_3760,
        script_sound_bank_man_find_free_entry,
        "ScriptSoundBankMan_FindFreeEntry",
    ),
    (0x0028_3798, script_movie_ctor, "ScriptMovie_ctor"),
    (0x0028_37c8, script_movie_dtor, "ScriptMovie_dtor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;

    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;

    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;

    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;

    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;

    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;

    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

// ---------------------------------------------------------------------------
// Script movie.

/// `ScriptMovie_ctor(ScriptMovie *self)` (0x283798): clear the movie handle and
/// state, and install the vtable `0x482228` at +0x18. Returns `self` in v0.
pub fn script_movie_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x0048_2228; // 0x480000 + 0x2228
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = sx(VTABLE);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(0x18).sw(c, sx(VTABLE));
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `void* ScriptSoundBankMan_FindFreeEntry(ScriptSoundBank* self)` (0x283760):
/// first entry (stride 0xc) whose data pointer is null, or 0.
pub fn script_sound_bank_man_find_free_entry(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    let empty = (c.g[A1][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let occupied = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !occupied {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0028_3770);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void ScriptSoundBankMan_FreeEntry(u32 unused, SoundBankEntry* e /*a1*/)`
/// (0x283720): unload `e->bank` (`+4`) through the sound bank manager, then
/// mark the entry free (`id = -1`, `data = 0`).
pub fn script_sound_bank_man_free_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0028_3734)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x118).lw(c);
        c.g[A1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK.call(c, 0x0028_3744)?;
        c.g[V0][0] = u64::MAX;
        c.sw(p(r32(c, S0), 8), 0);
        c.sw(r32(c, S0), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptSoundBankMan_RemoveBank(SoundBankMan* m, int id)` (0x283610):
/// drop the bank `id` from the list: its slot-2 method takes the entry.
pub fn script_sound_bank_man_remove_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK.call(c, 0x0028_3624)?;
        let entry = c.g[V0][0];
        c.g[A1][0] = entry;
        if entry != 0 {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x14));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0028_3644)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndEntry* ScriptSoundBankMan_FindBank(SndList* list, int id)` (0x2836d8):
/// scan `list->count` 0xc-byte entries for the live one with `id`; returns
/// it, or 0.
pub fn script_sound_bank_man_find_bank(c: &mut Cpu) {
    c.cycles += 3;
    let list = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[A2][0] = c.lw(list.wrapping_add(4));
    let none = (c.g[A2][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(list.wrapping_add(8));
        loop {
            c.cycles += 3;
            let data = c.lw(r32(c, A0).wrapping_add(8));
            c.g[V0][0] = data;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            let more;
            if data == 0 {
                c.cycles += 1;
                more = (c.g[V1][0] as i64) < (c.g[A2][0] as i64);
                c.g[V0][0] = more as u64;
            } else {
                c.cycles += 3;
                let entry_id = c.lw(r32(c, A0));
                c.g[V0][0] = entry_id;
                let hit = entry_id == id;
                more = (c.g[V1][0] as i64) < (c.g[A2][0] as i64);
                c.g[V0][0] = more as u64;
                if hit {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0];
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_36e8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `D_004A34DC`: the script-engine object table's live flag.
pub(crate) const SCRIPT_OBJ_TABLE_GP: i32 = 0x3ec;

/// The shared body of the two engine-object destructors below: install the
/// class vtable at `vt_off`, remove all children and, when bit 0 of `flags` is
/// set, delete the object. 13 (+2) + 5 cycles plus the callees.
pub(crate) fn script_obj_dtor(
    c: &mut Cpu,
    vt: u32,
    vt_off: u32,
    clear_global: bool,
    remove: Callee,
    remove_ret: u32,
    tail: u32,
) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(vt);
    c.sq(17, p(sp(c), 0x10));
    c.sq(16, p(sp(c), 0x20));
    c.g[S1][0] = c.g[A0][0];
    c.sd(p(sp(c), 0), c.g[RA][0]);
    c.g[S0][0] = c.g[A1][0];
    let self_ = r32(c, S1);
    c.sw(p(self_, vt_off), c.g[V0][0]);
    remove.call(c, remove_ret)?;
    c.g[S0][0] = c.g[S0][0] & 1;
    let keep = c.g[S0][0] == 0;
    if clear_global {
        c.sw(gp(c, SCRIPT_OBJ_TABLE_GP), 0);
    }
    if !keep {
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::OPERATOR_DELETE, remove_ret + 0x14)?;
    }
    c.cycles += tail as u64;
    frame.restore(c, 16, 0x20);
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void ScriptMovie_dtor(ScriptMovie* m, int flags)` (0x2837c8).
pub fn script_movie_dtor(c: &mut Cpu) {
    guest(c, |c| {
        script_obj_dtor(
            c,
            0x0048_2228,
            0x18,
            false,
            callee::SCRIPT_MOVIE_STOP,
            0x0028_37f0,
            4,
        )
    })
}

/// `ScriptSoundBankMan_dtor(ScriptSoundBankMan* self, int flags)` (0x283518):
/// install the vtable, remove all banks, free the name block and delete the
/// object when the low flag bit is set.
pub fn script_sound_bank_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0x0048_2240;
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        call_ptr(c, 0x0028_3658, 0x0028_3540)?;
        c.g[A0][0] = lw(c, p(this as u32, 8));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0028_3554)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0028_3568)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptSoundBankMan_RemoveAll(ScriptSoundBankMan* m)` (0x00283658):
/// for each of the `n` banks (array at `+8`, stride 0xc) call its `+0xc`
/// virtual destructor when present.
pub fn script_sound_bank_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V1][0] != 0 {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let vt = c.g[V0][0];
                    let obj = c.g[S0][0];
                    vt_call(c, vt, 0x10, obj, V1, 0x0028_36a8)?;
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_3680);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `int ScriptSoundBankMan_AddBank(SndList* self, int id, void* data, int b)`
/// (0x283580): reuse the bank entry for `id`, or take a free one, remember
/// the data and load the bank.
pub fn script_sound_bank_man_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        let self_ = abi::arg(c, 0);
        let id = abi::arg64(c, 1);
        let data = abi::arg64(c, 2);
        let b = abi::arg64(c, 3);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx(self_);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = id;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = data;
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[S4][0] = b;
        call_ptr(c, callee::SND_LIST_FIND_BANK, 0x0028_35b0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.lw(at(c, S0, 4));
        } else {
            c.cycles += 13;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::SND_LIST_FIND_FREE_ENTRY, 0x0028_35c4)?;
            c.g[S0][0] = c.g[V0][0];
            c.sw(r32(c, S0), c.g[S2][0]);
            c.sw(at(c, S0, 8), c.g[S3][0]);
            call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2, 0x0028_35d4)?;
            c.g[V1][0] = c.lw(at(c, V0, 0x118));
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = c.lw(at(c, S0, 4));
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = c.lw(r32(c, V1));
            call_ptr(c, callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY, 0x0028_35ec)?;
            c.g[V0][0] = c.lw(at(c, S0, 4));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptSoundBankMan::ctor(this, flags, count, base, tag)` (0x283450):
/// install the vtable, allocate the bank array and initialise each entry.
pub fn script_sound_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let this = abi::arg(c, 0);
        let flags = abi::arg(c, 1);
        let count = abi::arg(c, 2);
        let base = abi::arg(c, 3);
        let tag = abi::arg64(c, 4);
        let frame = Frame::open(c, 0x30);
        c.g[T0][0] = abi::arg64(c, 2);
        frame.save(c, 17, 0x10);
        c.g[V1][0] = abi::arg64(c, 1);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x20);
        c.g[A0][0] = 0xc;
        c.g[V0][0] = 0x0048_0000;
        c.mult(0, 0, 8, 4);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x2240));
        c.g[S0][0] = abi::arg64(c, 3);
        c.g[A1][0] = 0x0048_0000;
        c.g[A2][0] = c.g[V1][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x1fb0));
        Addr(this).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.lo[0];
        Addr(this).off(0xc).sw(c, c.g[V0][0]);
        c.g[A3][0] = 0;
        Addr(this).off(4).sw(c, c.g[T0][0]);
        callee::OPERATOR_NEW_NAMED.call(c, 0x0028_34a4)?;

        c.g[V1][0] = Addr(this).off(4).lw(c);
        c.g[A1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 3;
            c.g[A2][0] = sx(0xffff_ffff_u32);
            c.g[A0][0] = 0;
            loop {
                c.cycles += 15;
                c.g[V1][0] = Addr(this).off(8).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr(c.g[V1][0] as u32).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(this).off(8).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).off(4).sw(c, c.g[S0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(this).off(8).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).off(8).sw(c, 0);
                c.g[V0][0] = Addr(this).off(4).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_34c0);
                }
            }
        }

        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = (flags, count, base, tag);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tags2::tests::base;

    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn script_movie_ctor_matches() {
        check(
            ssx3_ee::s07::f_00283798,
            script_movie_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x1111_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..9 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn script_sound_bank_man_find_free_entry_matches() {
        check(
            ee::s07::f_00283760__generated,
            script_sound_bank_man_find_free_entry,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2837_60, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x40);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                let count = seed % 4;
                c.bus.write32(self_ + 4, count);
                c.bus.write32(self_ + 8, DATA + 0x600);
                for i in 0..4u32 {
                    let occupied = (i + seed) % 2 == 0;
                    c.bus.write32(DATA + 0x600 + i * 0xc + 8, occupied as u32);
                }
            },
        );
    }

    #[test]
    fn script_sound_bank_man_free_entry_matches() {
        check(
            ee::s07::f_00283720__generated,
            script_sound_bank_man_free_entry,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x283720, 0);
                let sys = DATA + 0x300;
                let banks = DATA + 0x400;
                let entry = DATA + 0x500;
                c.g[A1][0] = entry as u64;
                c.bus.write32(GAME_GP + 0x410, sys);
                c.bus.write32(sys + 0x118, banks);
                c.bus.write32(banks, DATA + 0x600);
                c.bus.write32(entry + 4, (seed % 16) as u32);
                c.bus.write32(entry, (seed % 9) as u32);
            },
        );
    }

    /// A one-entry sound bank list: `+4` count, `+8` entries (0xc bytes each),
    /// `+0xc` the manager vtable, entry 0 live.
    pub(crate) fn sound_bank_list(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) -> u32 {
        let list = DATA + 0x1000;
        let entries = DATA + 0x2000;
        let count = (seed % 4) as u32;
        c.bus.write32(list + 4, count);
        c.bus.write32(list + 8, entries);
        for i in 0..4u32 {
            let e = entries + 0xc * i;
            c.bus
                .write32(e, if i == 0 { 0 } else { 1 + r.next_u32() % 3 });
            c.bus
                .write32(e + 8, if i < count && i == 0 { 1 } else { 0 });
        }
        let vt = vtable(c, r, &[0x10]);
        c.bus.write32(list + 0xc, vt);
        list
    }

    #[test]
    fn script_sound_bank_man_remove_bank_matches() {
        covered(
            ee::s07::f_00283610__generated,
            script_sound_bank_man_remove_bank,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_3610, 1);
                let list = sound_bank_list(c, &mut r, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn script_sound_bank_man_find_bank_matches() {
        check(
            ee::s07::f_002836d8__generated,
            script_sound_bank_man_find_bank,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_36d8, 0);
                let list = sound_bank_list(c, &mut r, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    #[test]
    fn script_movie_dtor_matches() {
        check(
            ee::s07::f_002837c8__generated,
            script_movie_dtor,
            60,
            &|c, seed| {
                base(c, seed, 0x2837);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
            },
        );
    }

    // ------------------------------------------------------- bank-manager dtors

    #[test]
    fn script_sound_bank_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00283518__generated,
            script_sound_bank_man_dtor,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x283518);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x408, 0);
            },
        );
    }

    #[test]
    fn script_sound_bank_man_remove_all_matches() {
        check(
            ee::s07::f_00283658__generated,
            script_sound_bank_man_remove_all,
            80,
            &|c, seed| {
                let _ = base_2(c, seed, 0x2836, 3);
                let m = DATA;
                let n = (seed % 4) as u32;
                w(c, m + 4, n);
                w(c, m + 8, DATA + 0x1000);
                w(c, m + 0xc, DATA + 0x1800);
                slot(c, DATA + 0x1800, 0x10, FAKE);
                for i in 0..4u32 {
                    w(
                        c,
                        DATA + 0x1000 + 0xc * i + 8,
                        if (seed >> i) & 1 == 1 { 1 } else { 0 },
                    );
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn script_sound_bank_man_add_bank_matches() {
        check(
            ee::s07::f_00283580__generated,
            script_sound_bank_man_add_bank,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x2835, 0);
                env(c);
                let this = DATA + 0x400;
                let entries = DATA + 0x1000;
                let id = (seed % 3) as u32;
                if seed % 2 == 0 {
                    w32(c, this + 4, 1);
                    w32(c, this + 8, entries);
                    w32(c, entries, id);
                    w32(c, entries + 8, 1);
                } else {
                    w32(c, this + 4, 1);
                    w32(c, this + 8, entries);
                    w32(c, entries, id + 10);
                    w32(c, entries + 8, 0);
                }
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn script_sound_bank_man_ctor_matches() {
        check(
            ee::s07::f_00283450__generated,
            script_sound_bank_man_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2834, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { (seed % 5) as u64 };
                c.bus.write32(LOG_RESULTS, 0);
                let _ = &mut r;
            },
        );
    }
}
