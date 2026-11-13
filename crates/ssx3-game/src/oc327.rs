//! Staging module oc327 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc327`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_8330, spch_find_bank_type, "SPCH_FindBankType"),
    (0x003d_9ac8, spch_sentence_append, "SPCH_SentenceAppend"),
    (
        0x0034_2e98,
        debounce_kill_restorable,
        "Debounce_KillRestorable",
    ),
    (
        0x002a_dc30,
        bank_monitor_find_free_timer,
        "BankMonitor_FindFreeTimer",
    ),
    (
        0x002f_b498,
        world_obj_factory_anim_teeter,
        "WorldObjFactory_AnimTeeter",
    ),
    (0x003b_47f8, eac_unpack, "EAC_Unpack"),
    (0x003a_7218, cstream_man_decompress, "cStreamMan_Decompress"),
    (0x003c_bb28, snd_dsp_resample_init, "SND_DspResample_Init"),
    (0x003d_9a40, spch_reset_sentence, "SPCH_ResetSentence"),
    (
        0x003d_81f0,
        spch_find_bank_user_data,
        "SPCH_FindBankUserData",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

/// `WorldObj_RunKillScript` (0x34fcc0).
pub(crate) const OC327_WORLD_OBJ_RUN_KILL_SCRIPT: Callee = Callee {
    addr: 0x0034_fcc0,
    f: ee::s11::WorldObj_RunKillScript,
};
/// `DeadNode_ctor` (0x3506d8).
pub(crate) const OC327_DEAD_NODE_CTOR: Callee = Callee {
    addr: 0x0035_06d8,
    f: ee::s11::DeadNode_ctor,
};
/// `RestoreNode_ctor` (0x350f60).
pub(crate) const OC327_RESTORE_NODE_CTOR: Callee = Callee {
    addr: 0x0035_0f60,
    f: ee::s11::RestoreNode_ctor,
};
/// `BankMonitor_TimerIsFree` (0x2ad930).
pub(crate) const OC327_BANK_TIMER_IS_FREE: Callee = Callee {
    addr: 0x002a_d930,
    f: ee::s08::BankMonitor_TimerIsFree,
};
/// `WorldObjFactory_AttachModifier` (0x2fae38, `checkActiveNode`).
pub(crate) const OC327_CHECK_ACTIVE_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};
/// `EAC_Unpack` (0x3b47f8).
pub(crate) const OC327_EAC_UNPACK: Callee = Callee {
    addr: 0x003b_47f8,
    f: ee::s13::EAC_Unpack,
};
/// `RefPack_Unpack` (0x3b4740).
pub(crate) const OC327_REFPACK_UNPACK: Callee = Callee {
    addr: 0x003b_4740,
    f: ee::s13::RefPack_Unpack,
};
/// `EA_MemAllocNamed` (0x317d70; the generator emitted `call_indirect`).
pub(crate) const OC327_EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
/// `MEM_copy` (0x3e6574; `call_indirect`).
pub(crate) const OC327_MEM_COPY: u32 = 0x003e_6574;
/// The generic animated-object constructor (0x3421a0; `call_indirect`).
pub(crate) const OC327_ANIM_OBJECT_CTOR_TEETER: u32 = 0x0034_21a0;

// ---------------------------------------------------------------------------
// SPCH bank table.

/// The 8-entry SPCH bank table (`{ void* data; u32 user; }` per entry).
pub(crate) const OC327_SPCH_BANK_TABLE: u32 = 0x0044_ffb8;
/// `SPCH_Sentence`'s global sentence state (`SPCH_SentenceGlobals`).
pub(crate) const OC327_SPCH_SENTENCE_GLOBALS: u32 = 0x0045_0700;

/// `s32 SPCH_FindBankType(const u8* key, u8* type)` (0x3d8330): scan the eight
/// bank-table entries for the one whose data block has `block[9] == key[2]`
/// and `block[8] == key[3]`; on a hit return 1 and store `block[9]` through
/// `type`, otherwise return 0. A leaf with a 0x10 stack frame for the matched
/// entry pointer.
pub fn spch_find_bank_type(c: &mut Cpu) {
    c.cycles += 8;
    let key = c.g[A0][0] as u32;
    let out = c.g[A1][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = 0x45_0000;
    c.g[A3][0] = sx(OC327_SPCH_BANK_TABLE);
    c.g[T3][0] = 0;
    let slot = c.g[SP][0] as u32;
    c.g[T2][0] = c.g[SP][0];
    c.g[T1][0] = 0;
    c.g[T0][0] = 0;
    let mut found = false;
    loop {
        c.cycles += 3;
        let block = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A2][0] = block;
        if block == 0 {
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 4;
            let want = Addr((block as u32).wrapping_add(9)).lbu(c);
            c.g[V1][0] = want;
            let have = Addr(key.wrapping_add(2)).lbu(c);
            c.g[V0][0] = have;
            if want != have {
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 4;
                let want = Addr((block as u32).wrapping_add(8)).lbu(c);
                c.g[V1][0] = want;
                let have = Addr(key.wrapping_add(3)).lbu(c);
                c.g[V0][0] = have;
                if want != have {
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    Addr(slot).sw(c, c.g[A3][0]);
                    c.g[T1][0] = 1;
                    found = true;
                }
            }
        }
        if found {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_8350);
        }
    }
    c.cycles += 2;
    let none = c.g[T1][0] == 0;
    c.g[V0][0] = Addr(slot).lw(c);
    if !none {
        c.cycles += 4;
        c.g[T3][0] = 1;
        let block = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = block;
        let kind = Addr((block as u32).wrapping_add(9)).lbu(c);
        c.g[A0][0] = kind;
        Addr(out).sw(c, kind);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T3][0];
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `s32 SPCH_FindBankUserData(const u8* key, u32* user)` (0x3d81f0): the same
/// bank-table scan as `SPCH_FindBankType`, but a hit stores the entry's user
/// word (`*(u32*)(entry + 4)`) through `user`.
pub fn spch_find_bank_user_data(c: &mut Cpu) {
    c.cycles += 8;
    let key = c.g[A0][0] as u32;
    let out = c.g[A1][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = 0x45_0000;
    c.g[A3][0] = sx(OC327_SPCH_BANK_TABLE);
    c.g[T3][0] = 0;
    let slot = c.g[SP][0] as u32;
    c.g[T2][0] = c.g[SP][0];
    c.g[T1][0] = 0;
    c.g[T0][0] = 0;
    let mut found = false;
    loop {
        c.cycles += 3;
        let block = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A2][0] = block;
        if block == 0 {
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 4;
            let want = Addr((block as u32).wrapping_add(9)).lbu(c);
            c.g[V1][0] = want;
            let have = Addr(key.wrapping_add(2)).lbu(c);
            c.g[V0][0] = have;
            if want != have {
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 4;
                let want = Addr((block as u32).wrapping_add(8)).lbu(c);
                c.g[V1][0] = want;
                let have = Addr(key.wrapping_add(3)).lbu(c);
                c.g[V0][0] = have;
                if want != have {
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    Addr(slot).sw(c, c.g[A3][0]);
                    c.g[T1][0] = 1;
                    found = true;
                }
            }
        }
        if found {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_8210);
        }
    }
    c.cycles += 2;
    let none = c.g[T1][0] == 0;
    c.g[V1][0] = Addr(slot).lw(c);
    if !none {
        c.cycles += 3;
        c.g[T3][0] = 1;
        let user = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
        c.g[V0][0] = user;
        Addr(out).sw(c, user);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T3][0];
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// SPCH sentences.

/// `void SPCH_SentenceAppend(SPCH_Node* h, s32 id, void* a2, const void* a3,
/// u32 a4, const u32* a5)` (0x3d9ac8): if the current sentence is free (-1) or
/// already `id`, take the next of the eight sentence slots (`f4++`); fill it
/// (20 words from `a5`, the q-table from `h`'s entries, `cnt = a3[4] >> 2`
/// records), mark it live and advance the q index.
pub fn spch_sentence_append(c: &mut Cpu) {
    let g = OC327_SPCH_SENTENCE_GLOBALS;
    c.cycles += 7;
    c.g[V0][0] = 0x45_0000;
    c.g[T7][0] = c.g[A0][0];
    c.g[T6][0] = c.g[V0][0];
    c.g[T4][0] = sx(g);
    c.g[V0][0] = Addr(g).lw(c);
    let busy = (c.g[V0][0] as i64) < 0;
    c.g[T5][0] = c.g[A2][0];
    let mut append = busy;
    if !busy {
        c.cycles += 2;
        append = c.g[V0][0] == c.g[A1][0];
    }
    if append {
        // `f4++`: the slot index, and no room past eight.
        c.cycles += 5;
        c.g[T3][0] = Addr(g.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(1));
        c.g[V1][0] = ((c.g[T3][0] as i64) < 8) as u64;
        let full = c.g[V1][0] == 0;
        Addr(g.wrapping_add(4)).sw(c, c.g[V0][0]);
        if !full {
            // Slot base: `g + slot * 0x5c`, fields at +0x18c/+0x190/+0x194.
            c.cycles += 12;
            c.g[V0][0] = 0x5c;
            Addr(g).sw(c, c.g[A1][0]);
            c.mult(0, V0, T3, V0);
            c.g[A2][0] = sx(g.wrapping_add(0x198));
            c.g[T2][0] = 0x13;
            c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[A0][0] = c.g[V1][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x18c)).sw(c, c.g[T5][0]);
            Addr((c.g[A0][0] as u32).wrapping_add(0x190)).sw(c, c.g[A3][0]);
            Addr((c.g[A1][0] as u32).wrapping_add(0x194)).sb(c, c.g[T0][0]);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            // 20 words of per-sentence data.
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(1));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(4));
                Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                let more = (c.g[T2][0] as i64) >= 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_9b30);
                }
            }
            // Live flag and the q records from the node's entries.
            c.cycles += 10;
            c.g[V0][0] = 0x5c;
            c.g[A0][0] = sx(g);
            c.mult(0, V0, T3, V0);
            c.g[V1][0] = 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Addr((c.g[V0][0] as u32).wrapping_add(0x195)).sb(c, c.g[V1][0]);
            c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(4)).lbu(c);
            c.g[A3][0] = sx((c.g[V0][0] as u32) >> 2);
            let empty = c.g[A3][0] == 0;
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
            if !empty {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = c.g[T7][0];
                c.g[A0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x62));
                c.g[T2][0] = c.g[A3][0];
                loop {
                    c.cycles += 12;
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lhu(c);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(1));
                    Addr((c.g[A2][0] as u32).wrapping_sub(4)).sh(c, c.g[V1][0]);
                    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(2)).lhu(c);
                    Addr((c.g[A2][0] as u32).wrapping_sub(2)).sh(c, c.g[V0][0]);
                    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(6)).lbu(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
                    Addr(c.g[A2][0] as u32).sh(c, c.g[V0][0]);
                    let more = c.g[T2][0] != 0;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(8));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_9b90);
                    }
                }
            }
            // The q index advances by the record count.
            c.cycles += 4;
            c.g[V1][0] = sx(g);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void SPCH_ResetSentence(void)` (0x3d9a40): reset the global sentence state
/// (current id -1, counters 0) and clear the eight 0x5c-byte sentence slots
/// and the 48 two-word q entries.
pub fn spch_reset_sentence(c: &mut Cpu) {
    let g = OC327_SPCH_SENTENCE_GLOBALS;
    c.cycles += 10;
    c.g[V0][0] = 0x45_0000;
    c.g[A0][0] = u64::MAX;
    c.g[V1][0] = sx(g);
    Addr(g).sw(c, c.g[A0][0]);
    c.g[A0][0] = c.g[V1][0];
    Addr(g.wrapping_add(4)).sw(c, 0);
    Addr(g.wrapping_add(8)).sw(c, 0);
    c.g[V1][0] = sx(g.wrapping_add(0x2e0));
    Addr(g.wrapping_add(0x18c)).sw(c, 0);
    loop {
        c.cycles += 8;
        let p = c.g[A0][0] as u32;
        Addr(p.wrapping_add(0x190)).sw(c, 0);
        Addr(p.wrapping_add(0x194)).sb(c, 0);
        Addr(p.wrapping_add(0x195)).sb(c, 0);
        Addr(p.wrapping_add(0x197)).sb(c, 0);
        c.g[A0][0] = sx(p.wrapping_add(0x5c));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x45_0000;
            c.g[A0][0] = u64::MAX;
            c.g[V0][0] = sx(g);
            c.g[V1][0] = 0x2f;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12));
            loop {
                c.cycles += 7;
                let p = c.g[V0][0] as u32;
                Addr(p.wrapping_sub(6)).sh(c, c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                Addr(p.wrapping_sub(4)).sh(c, c.g[A0][0]);
                Addr(p.wrapping_sub(2)).sh(c, 0);
                Addr(p).sh(c, 0);
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx(p.wrapping_add(8));
                if !more {
                    c.cycles += 2;
                    ret(c);
                    return;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_9aa0);
                }
            }
        }
        Addr((c.g[A0][0] as u32).wrapping_add(0x18c)).sw(c, 0);
        if event_due(c) {
            deliver_event_here(c, 0x003d_9a68);
        }
    }
}

// ---------------------------------------------------------------------------
// Debounce nodes.

/// The vtable-driven `OnKilled(this, 3)` call: `this+0xc` holds the vtable,
/// its slot +8 is `{ short delta; short; void (*fn)(); }` and the method is
/// called with `this + delta`. Charges `pre` cycles first (the caller charges
/// them inside its own branch).
fn debounce_kill_restorable_vt(c: &mut Cpu, ret_: u32, pre: u32) -> crate::call::Flow {
    c.cycles += pre as u64;
    let vt = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 3;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(8)).lh(c);
    c.g[V1][0] = Addr((vt as u32).wrapping_add(0xc)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[RA][0] = sx(ret_);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_)
}

/// `void Debounce_KillRestorable(Debounce* self, const void* check)`
/// (0x342e98): if the kill script lets it through and the debounce counter is
/// non-negative, mark the node killed (`OnKilled(this, 3)`) and replace it by
/// a dead node (mode 0) or a restore node (mode 3).
pub fn debounce_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_check = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        debounce_kill_restorable_body(c, frame, no_check)?;
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The body of `Debounce_KillRestorable`: each early exit restores `s0` (the
/// original restores it there and not in the epilogue) and returns.
fn debounce_kill_restorable_body(c: &mut Cpu, frame: Frame, no_check: bool) -> crate::call::Flow {
    if !no_check {
        c.cycles += 4;
        OC327_WORLD_OBJ_RUN_KILL_SCRIPT.call(c, 0x0034_2eb8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
            return Ok(());
        }
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x2c)).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        frame.restore(c, S0, 0x20);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x30)).lw(c);
    let mode = c.g[V1][0] != 0;
    c.g[V0][0] = 3;
    if !mode {
        // Mode 0: notify, then a dead node in place of this one.
        c.cycles += 2;
        let null_self = c.g[S0][0] == 0;
        c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
        if !null_self {
            debounce_kill_restorable_vt(c, 0x0034_2ef8, 6)?;
        }
        c.cycles += 11;
        c.g[A1][0] = 0x49_0000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = sx(0x0048_e738);
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0034_2f10);
        c.g[A3][0] = 0;
        call_ptr(c, OC327_EA_MEM_ALLOC_NAMED, 0x0034_2f10)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0034_2f1c);
        c.g[A1][0] = c.g[S1][0];
        OC327_DEAD_NODE_CTOR.call(c, 0x0034_2f1c)?;
        frame.restore(c, S0, 0x20);
        return Ok(());
    }
    c.cycles += 2;
    if c.g[V1][0] != c.g[V0][0] {
        // Any other mode: only the notification.
        c.cycles += 2;
        c.g[A1][0] = 3;
        let null_self = c.g[S0][0] == 0;
        if !null_self {
            debounce_kill_restorable_vt(c, 0x0034_2f94, 5)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        return Ok(());
    }
    // Mode 3: notify, then a restore node in place of this one.
    c.cycles += 2;
    let null_self = c.g[S0][0] == 0;
    c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
    if !null_self {
        debounce_kill_restorable_vt(c, 0x0034_2f4c, 6)?;
    }
    c.cycles += 11;
    c.g[A1][0] = 0x49_0000;
    c.g[A0][0] = 0x1c;
    c.g[A1][0] = sx(0x0048_e748);
    c.g[A2][0] = 0x2000_0000;
    c.g[RA][0] = sx(0x0034_2f64);
    c.g[A3][0] = 0;
    call_ptr(c, OC327_EA_MEM_ALLOC_NAMED, 0x0034_2f64)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[RA][0] = sx(0x0034_2f70);
    c.g[A1][0] = c.g[S1][0];
    OC327_RESTORE_NODE_CTOR.call(c, 0x0034_2f70)?;
    frame.restore(c, S0, 0x20);
    Ok(())
}

// ---------------------------------------------------------------------------
// Sound.

/// `s32 BankMonitor_FindFreeTimer(BankMonitor* self, s32 n)` (0x2adc30): the
/// index of the `n`-th free timer among the 48 at `self+0x98` (0x2c bytes
/// apart), or -1.
pub fn bank_monitor_find_free_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        let mut done = false;
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            OC327_BANK_TIMER_IS_FREE.call(c, 0x002a_dc58)?;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                if c.g[S2][0] != 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    done = true;
                }
            }
            if done {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 48) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_dc50);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// World object factory.

/// `void WorldObjFactory_AnimTeeter(Vec3* out, s32 n, const AnimEntry* e)`
/// (0x2fb498): build a teeter modifier template (initialised once into
/// 0x4fb400), overlay the `n` entries (`{ s32 idx; s32 val; u8 pad[4]; s32
/// type; }`), resolve the object id in the template, skip when the object is
/// already flagged, else create the modifier through the object's ctor, and
/// return the fixed global (0x4c9098) into `out`.
pub fn world_obj_factory_anim_teeter(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b400;
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(0x24d8)).lw(c);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A2][0] = 0x50_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !initialised {
            // First call: fill the global template.
            c.cycles += 26;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = Addr(TEMPLATE.wrapping_add(0x28)).lbu(c);
            c.g[V1][0] |= 0xffff;
            c.lwc1(3, gp.wrapping_sub(0x339c));
            c.g[A1][0] = 1;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 4);
            c.g[AT][0] = 0x4270_0000;
            c.mtc1(1, 0);
            c.lwc1(1, gp.wrapping_sub(0x3398));
            c.g[AT][0] = 0xffff_ffff_c120_0000;
            c.mtc1(1, 2);
            Addr(TEMPLATE).sw(c, c.g[V1][0]);
            Addr(TEMPLATE.wrapping_add(0x28)).sb(c, c.g[A0][0]);
            c.swc1(0, TEMPLATE.wrapping_add(0x0c));
            c.swc1(1, TEMPLATE.wrapping_add(0x10));
            c.swc1(2, TEMPLATE.wrapping_add(0x14));
            c.swc1(3, TEMPLATE.wrapping_add(0x18));
            c.swc1(4, TEMPLATE.wrapping_add(0x24));
            Addr(gp.wrapping_add(0x24d8)).sw(c, c.g[A1][0]);
            c.swc1(3, TEMPLATE.wrapping_add(0x04));
            Addr(TEMPLATE.wrapping_add(0x08)).sw(c, 0);
            Addr(TEMPLATE.wrapping_add(0x1c)).sw(c, 0);
            c.swc1(4, TEMPLATE.wrapping_add(0x20));
            Addr(c.g[SP][0] as u32).sb(c, c.g[A0][0]);
        }
        // Copy the template into the stack buffer (`MEM_copy`).
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c00));
        c.g[A2][0] = 0x2c;
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002f_b558);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, OC327_MEM_COPY, 0x002f_b558)?;
        let no_entries = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_entries {
            // Overlay the entries onto the buffer.
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x0044_5ed0);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let same = c.g[V1][0] == c.g[V0][0];
                if !same {
                    c.cycles += 2;
                }
                if same || c.g[V0][0] != c.g[A3][0] {
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                    c.cycles += 1;
                    Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, c.g[A0][0] as u32);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_b570);
                }
            }
        }
        // Resolve the template id to its object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let resolvable = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if !resolvable {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            // Charge of `s0 = obj`, after the `'b_2fb620` block: both the
            // resolved and the null-reference path pay it.
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }
        // Already active? Else create the modifier.
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            let none = c.g[A0][0] == 0;
            c.g[A1][0] = 4;
            let mut skip = false;
            if !none {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002f_b640);
                c.g[A2][0] = c.g[S1][0];
                OC327_CHECK_ACTIVE_NODE.call(c, 0x002f_b640)?;
                skip = c.g[V0][0] != 0;
            }
            if !skip {
                c.cycles += 11;
                c.g[A1][0] = 0x49_0000;
                c.g[A0][0] = 0x80;
                c.g[A1][0] = sx(0x0048_9678);
                c.g[A2][0] = 0x2000_0000;
                c.g[RA][0] = sx(0x002f_b660);
                c.g[A3][0] = 0;
                call_ptr(c, OC327_EA_MEM_ALLOC_NAMED, 0x002f_b660)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002f_b674);
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, OC327_ANIM_OBJECT_CTOR_TEETER, 0x002f_b674)?;
            }
        }
        // Return the fixed global vector into `out`.
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[A1][0] as u32);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[S2][0] as u32);
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Streaming / sound DSP.

/// `void* EAC_Unpack(...)` (0x3b47f8): a one-call wrapper over
/// `RefPack_Unpack` (the EA "refpack" decompressor).
pub fn eac_unpack(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(OC327_REFPACK_UNPACK),
        0x003b_4808,
    );
}

/// `void cStreamMan_Decompress(a0, a1, a2, a3)` (0x3a7218): decompress `a3`
/// through `EAC_Unpack`, returning its result.
pub fn cstream_man_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003a_7228);
        c.g[A0][0] = c.g[A3][0];
        OC327_EAC_UNPACK.call(c, 0x003a_7228)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SND_DspResample_Init(SND_DspResample* self)` (0x3cbb28): install the
/// resampler vtable (0x3cb540) and clear its counters and its four word
/// fields at +0x34..+0x28.
pub fn snd_dsp_resample_init(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x3d_0000;
    let self_ = c.g[A0][0] as u32;
    Addr(self_.wrapping_add(4)).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4ac0));
    Addr(self_.wrapping_add(0x20)).sw(c, 0);
    Addr(self_).sw(c, c.g[V0][0]);
    c.g[V1][0] = 3;
    Addr(self_.wrapping_add(0x24)).sh(c, 0);
    c.g[V0][0] = sx(self_.wrapping_add(0x34));
    Addr(self_.wrapping_add(0x26)).sh(c, 0);
    loop {
        c.cycles += 7;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_bb50);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, machine, vtable};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Once;

    // ------------------------------------------------------------ fake callees

    /// Control word of this module's gated fakes (in the machine's RAM, so it
    /// never leaks into another test even when tests share a process).
    pub(crate) const OC327_CTRL: u32 = DATA + 0x3e_0000;
    pub(crate) const OC327_MAGIC: u32 = 0x0c32_7f00;
    /// One forced result word per fake.
    pub(crate) const OC327_RET: u32 = DATA + 0x3e_0010;

    const RET_REFPACK: u32 = 0;
    const RET_RUN_SCRIPT: u32 = 1;
    const RET_ALLOC: u32 = 2;
    const RET_ATTACH: u32 = 3;
    const RET_BASE_CTOR: u32 = 4;
    const RET_WSCRIPT_CACHE: u32 = 5;

    /// A logging fake when this module's control word is set, otherwise
    /// whatever was installed before (chained, as `oc318` does).
    fn oc327_fake_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC327_CTRL) == OC327_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                // Sign-extended, as `abi::set_ret` leaves it: `-1` is a
                // meaningful result for the script lookup.
                c.g[V0][0] = sx(c.bus.read32(OC327_RET + 4 * ret_idx));
            }
            return;
        }
        let p = prev.load(Ordering::Relaxed);
        let f: fn(&mut Cpu) = if p == 0 {
            fallback
        } else {
            // SAFETY: `p` was read from the same slot (a `fn(&mut Cpu)`).
            unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) }
        };
        f(c)
    }

    macro_rules! oc327_hook {
        ($fake:ident, $prev:ident, $addr:expr, $ret:expr, $fallback:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc327_fake_gated(c, $addr, $ret, &$prev, $fallback);
            }
        };
    }

    oc327_hook!(
        oc327_fake_refpack,
        OC327_PREV_REFPACK,
        0x003b_4740,
        RET_REFPACK,
        ee::s13::f_003b4740__generated
    );
    oc327_hook!(
        oc327_fake_run_script,
        OC327_PREV_RUN_SCRIPT,
        0x0030_a598,
        RET_RUN_SCRIPT,
        ee::s10::f_0030a598__generated
    );
    oc327_hook!(
        oc327_fake_alloc,
        OC327_PREV_ALLOC,
        0x0031_7d70,
        RET_ALLOC,
        ee::s10::f_00317d70__generated
    );
    oc327_hook!(
        oc327_fake_wscript_cache,
        OC327_PREV_WSCRIPT_CACHE,
        0x003a_6b78,
        RET_WSCRIPT_CACHE,
        ee::s13::f_003a6b78__generated
    );
    oc327_hook!(
        oc327_fake_attach,
        OC327_PREV_ATTACH,
        0x002f_ae38,
        RET_ATTACH,
        ee::s10::f_002fae38__generated
    );
    oc327_hook!(
        oc327_fake_base_ctor,
        OC327_PREV_BASE_CTOR,
        0x0034_d9b0,
        RET_BASE_CTOR,
        ee::s11::f_0034d9b0__generated
    );

    fn oc327_install_refpack() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            OC327_PREV_REFPACK.store(ee::s13::OVERRIDE_003B4740 as usize, Ordering::Relaxed);
            ee::s13::OVERRIDE_003B4740 = oc327_fake_refpack;
        });
    }

    /// The allocator, shared by both create paths (installed once, chained).
    fn oc327_install_alloc() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            let cur = ee::s10::OVERRIDE_00317D70 as usize;
            if cur != oc327_fake_alloc as fn(&mut Cpu) as usize {
                OC327_PREV_ALLOC.store(cur, Ordering::Relaxed);
                ee::s10::OVERRIDE_00317D70 = oc327_fake_alloc;
            }
        });
    }

    /// The kill-script chain of `Debounce_KillRestorable` (the script runner,
    /// the allocator and the dead-node ctor) is faked.
    fn oc327_install_debounce() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            OC327_PREV_RUN_SCRIPT.store(ee::s10::OVERRIDE_0030A598 as usize, Ordering::Relaxed);
            ee::s10::OVERRIDE_0030A598 = oc327_fake_run_script;
            // `RestoreNode_ctor` has no slot of its own: its base constructor
            // (`WorldObj_ctor`) stops it from touching the world.
            OC327_PREV_DEAD_CTOR.store(ee::s11::OVERRIDE_003506D8 as usize, Ordering::Relaxed);
            ee::s11::OVERRIDE_003506D8 = oc327_fake_dead_ctor;
            OC327_PREV_WORLD_OBJ_CTOR.store(ee::s11::OVERRIDE_0034FB00 as usize, Ordering::Relaxed);
            ee::s11::OVERRIDE_0034FB00 = oc327_fake_world_obj_ctor;
            // `WScriptMan_RunKillScript`'s script lookup, so a refusing kill
            // script stops there.
            OC327_PREV_WSCRIPT_CACHE.store(ee::s13::OVERRIDE_003A6B78 as usize, Ordering::Relaxed);
            ee::s13::OVERRIDE_003A6B78 = oc327_fake_wscript_cache;
        });
        oc327_install_alloc();
    }

    /// `AnimTeeter`'s create path: the allocator, `checkActiveNode` and the
    /// animated object's base constructor are faked.
    fn oc327_install_teeter() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| unsafe {
            OC327_PREV_ATTACH.store(ee::s10::OVERRIDE_002FAE38 as usize, Ordering::Relaxed);
            ee::s10::OVERRIDE_002FAE38 = oc327_fake_attach;
            OC327_PREV_BASE_CTOR.store(ee::s11::OVERRIDE_0034D9B0 as usize, Ordering::Relaxed);
            ee::s11::OVERRIDE_0034D9B0 = oc327_fake_base_ctor;
        });
        oc327_install_alloc();
    }

    oc327_hook!(
        oc327_fake_dead_ctor,
        OC327_PREV_DEAD_CTOR,
        0x0035_06d8,
        RET_BASE_CTOR,
        ee::s11::f_003506d8__generated
    );
    oc327_hook!(
        oc327_fake_world_obj_ctor,
        OC327_PREV_WORLD_OBJ_CTOR,
        0x0034_fb00,
        RET_BASE_CTOR,
        ee::s11::f_0034fb00__generated
    );

    // ------------------------------------------------------------------ tests

    /// `SPCH_FindBankType` over a bank table whose single matching entry may
    /// miss on either compared byte; both `check` bytes of the key vary.
    #[test]
    fn spch_find_bank_type_matches() {
        check(
            ee::s14::f_003d8330__generated,
            spch_find_bank_type,
            200,
            &|c, seed| {
                let mut r = Rng(0x3d83_3000 ^ seed as u64);
                scramble(c, &mut r);
                let key = DATA + 0x200;
                let out = DATA + 0x300;
                c.g[A0][0] = sx(key);
                c.g[A1][0] = sx(out);
                let k2 = (seed.wrapping_mul(37) % 251 + 1) as u8;
                let k3 = (seed.wrapping_mul(11) % 249 + 3) as u8;
                c.bus.write8(key + 2, k2);
                c.bus.write8(key + 3, k3);
                c.bus.write32(out, r.next_u32());
                let hit = seed % 9;
                for i in 0..8u32 {
                    let e = OC327_SPCH_BANK_TABLE + i * 8;
                    if i == hit {
                        let block = DATA + 0x400 + i * 0x20;
                        c.bus.write32(e, block);
                        // 0: match; 1: the +9 byte differs; 2: the +8 byte differs.
                        let variant = seed % 3;
                        c.bus
                            .write8(block + 9, if variant == 1 { k2 ^ 0x5a } else { k2 });
                        c.bus
                            .write8(block + 8, if variant == 2 { k3 ^ 0x5a } else { k3 });
                    } else if seed % 2 == 0 {
                        c.bus.write32(e, 0);
                    } else {
                        let block = DATA + 0x600 + i * 0x20;
                        c.bus.write32(e, block);
                        c.bus.write8(block + 9, k2.wrapping_add(1));
                        c.bus.write8(block + 8, k3);
                    }
                }
            },
        );
    }

    /// `SPCH_FindBankUserData`: the same scan, storing the entry's user word.
    #[test]
    fn spch_find_bank_user_data_matches() {
        check(
            ee::s14::f_003d81f0__generated,
            spch_find_bank_user_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x3d81_f000 ^ seed as u64);
                scramble(c, &mut r);
                let key = DATA + 0x200;
                let out = DATA + 0x300;
                c.g[A0][0] = sx(key);
                c.g[A1][0] = sx(out);
                let k2 = (seed.wrapping_mul(37) % 251 + 1) as u8;
                let k3 = (seed.wrapping_mul(11) % 249 + 3) as u8;
                c.bus.write8(key + 2, k2);
                c.bus.write8(key + 3, k3);
                c.bus.write32(out, r.next_u32());
                let hit = seed % 9;
                for i in 0..8u32 {
                    let e = OC327_SPCH_BANK_TABLE + i * 8;
                    c.bus.write32(e + 4, r.next_u32());
                    if i == hit {
                        let block = DATA + 0x400 + i * 0x20;
                        c.bus.write32(e, block);
                        let variant = seed % 3;
                        c.bus
                            .write8(block + 9, if variant == 1 { k2 ^ 0x5a } else { k2 });
                        c.bus
                            .write8(block + 8, if variant == 2 { k3 ^ 0x5a } else { k3 });
                    } else if seed % 2 == 0 {
                        c.bus.write32(e, 0);
                    } else {
                        let block = DATA + 0x600 + i * 0x20;
                        c.bus.write32(e, block);
                        c.bus.write8(block + 9, k2.wrapping_add(1));
                        c.bus.write8(block + 8, k3);
                    }
                }
            },
        );
    }

    /// `SPCH_SentenceAppend`: free (`-1`) and matching ids append, another id
    /// returns; the slot counter 0..8 covers both the full and the append
    /// paths, and the record count 0..3 the empty q-table.
    #[test]
    fn spch_sentence_append_matches() {
        check(
            ee::s14::f_003d9ac8__generated,
            spch_sentence_append,
            300,
            &|c, seed| {
                let mut r = Rng(0x3d9a_c800 ^ seed as u64);
                scramble(c, &mut r);
                let g = OC327_SPCH_SENTENCE_GLOBALS;
                let h = DATA + 0x1000;
                let a2 = DATA + 0x2000;
                let a3 = DATA + 0x3000;
                let a5 = DATA + 0x4000;
                for i in 0..0x120u32 / 4 {
                    c.bus.write32(h + i * 4, r.next_u32());
                }
                for i in 0..20u32 {
                    c.bus.write32(a5 + i * 4, r.next_u32());
                }
                let cnt = (seed % 4) as u8;
                c.bus.write8(a3 + 4, cnt * 4);
                c.bus.write32(a3, r.next_u32());
                let id = 0x21 + (seed % 5);
                let f0 = match seed % 3 {
                    0 => 0xffff_ffff,
                    1 => id,
                    _ => id.wrapping_add(1),
                };
                c.bus.write32(g, f0);
                c.bus.write32(
                    g + 4,
                    match seed % 4 {
                        0 => 7,
                        1 => 8,
                        2 => 9,
                        _ => seed % 8,
                    },
                );
                c.bus.write32(g + 8, seed % 40);
                c.g[A0][0] = sx(h);
                // Sometimes junk in the id's upper half: the compare is 64-bit.
                c.g[A1][0] = if seed % 7 == 0 {
                    (r.next_u32() as u64) << 32 | id as u64
                } else {
                    sx(id)
                };
                c.g[A2][0] = sx(a2);
                c.g[A3][0] = sx(a3);
                c.g[T0][0] = (seed as u64) << 16 | 0x40;
                c.g[T1][0] = sx(a5);
            },
        );
    }

    /// `SPCH_ResetSentence` clears the whole sentence state (globals, the
    /// eight slots and the 48 q entries).
    #[test]
    fn spch_reset_sentence_matches() {
        check(
            ee::s14::f_003d9a40__generated,
            spch_reset_sentence,
            100,
            &|c, seed| {
                let mut r = Rng(0x3d9a_4000 ^ seed as u64);
                scramble(c, &mut r);
                for i in 0..(0x500u32 / 4) {
                    c.bus
                        .write32(OC327_SPCH_SENTENCE_GLOBALS + i * 4, r.next_u32());
                }
            },
        );
    }

    /// `SND_DspResample_Init` on a scrambled resampler object.
    #[test]
    fn snd_dsp_resample_init_matches() {
        check(
            ee::s14::f_003cbb28__generated,
            snd_dsp_resample_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x3cbb_2800 ^ seed as u64);
                scramble(c, &mut r);
                let obj = DATA + 0x100 + (seed % 4) * 0x40;
                c.g[A0][0] = sx(obj);
                for i in 0..0x40u32 / 4 {
                    c.bus.write32(obj + i * 4, r.next_u32());
                }
            },
        );
    }

    /// `BankMonitor_FindFreeTimer`: timers free on a stride, `n` taking 1..3
    /// (found), 0 and 48, and 200 (the whole table, -1).
    #[test]
    fn bank_monitor_find_free_timer_matches() {
        check(
            ee::s08::f_002adc30__generated,
            bank_monitor_find_free_timer,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_dc30 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let n = match seed % 4 {
                    0 => 0,
                    1 => 1 + (seed % 3),
                    2 => 200,
                    _ => 48,
                };
                c.g[A1][0] = sx(n);
                for i in 0..48u32 {
                    let t = this + 0x98 + i * 0x2c;
                    let free = (i + seed) % 5 == 0;
                    c.bus
                        .write32(t + 4, if free { 0xffff_ffff } else { (seed ^ i) | 1 });
                    c.bus.write32(t, r.next_u32());
                }
            },
        );
    }

    /// `EAC_Unpack`: a one-call wrapper (the decompressor is a fake).
    #[test]
    fn eac_unpack_matches() {
        oc327_install_refpack();
        covered(
            ee::s13::f_003b47f8__generated,
            eac_unpack,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003b_47f8, 1);
                c.pc = 0x003b_47f8;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_REFPACK, 0x1000 + seed);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }

    /// `cStreamMan_Decompress`: moves `a3` to `a0` and calls `EAC_Unpack`.
    #[test]
    fn cstream_man_decompress_matches() {
        oc327_install_refpack();
        covered(
            ee::s13::f_003a7218__generated,
            cstream_man_decompress,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_7218, 1);
                c.pc = 0x003a_7218;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_REFPACK, 0x2000 + seed);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }

    /// `Debounce_KillRestorable`: kill-script refusals, a negative counter, the
    /// three modes (0, 3, other), a null self and a missing parent; the
    /// script runner, allocator and ctors are fakes.
    #[test]
    fn debounce_kill_restorable_matches() {
        oc327_install_debounce();
        covered(
            ee::s11::f_00342e98__generated,
            debounce_kill_restorable,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_2e98, 4);
                c.pc = 0x0034_2e98;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(
                    OC327_RET + 4 * RET_RUN_SCRIPT,
                    if seed % 4 == 0 { 1 } else { 0 },
                );
                // The script lookup answers `-1` (no script): the runner then
                // skips its call.
                c.bus
                    .write32(OC327_RET + 4 * RET_WSCRIPT_CACHE, 0xffff_ffff);
                let block = DATA + 0x8_0000;
                c.bus.write32(OC327_RET + 4 * RET_ALLOC, block);
                c.bus.write32(OC327_RET + 4 * RET_BASE_CTOR, 0);
                let this = DATA + 0x100;
                let parent = DATA + 0x600;
                c.g[A0][0] = if seed % 11 == 0 { 0 } else { sx(this) };
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x500) };
                c.bus.write32(
                    this + 0x2c,
                    if seed % 5 == 0 { 0x8000_0000 } else { seed % 4 },
                );
                c.bus.write32(
                    this + 0x30,
                    match seed % 3 {
                        0 => 0,
                        1 => 3,
                        _ => 1 + (seed % 4) * 4,
                    },
                );
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(this + 0xc, vt);
                c.bus.write32(this + 0x18, parent);
                c.bus.write32(parent + 8, r.next_u32());
                // The block the allocator hands out; a restore node reads its
                // parent link.
                c.bus
                    .write32(block + 0x18, if seed % 7 == 0 { 0 } else { parent });
                c.bus.write32(block + 0xc, r.next_u32());
            },
        );
    }

    /// `WorldObjFactory_AnimTeeter`: the template init on the first call, the
    /// entry overlay (type match, type 2 as a float, plain words), both id
    /// paths, and the already-attached vs create branches (faked ctor).
    #[test]
    fn world_obj_factory_anim_teeter_matches() {
        oc327_install_teeter();
        covered(
            ee::s10::f_002fb498__generated,
            world_obj_factory_anim_teeter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_b498, 4);
                c.pc = 0x002f_b498;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_ATTACH, (seed % 2) as u32);
                c.bus.write32(OC327_RET + 4 * RET_ALLOC, DATA + 0x8_0000);
                c.bus.write32(OC327_RET + 4 * RET_BASE_CTOR, 0);
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                // The one-time template flag (skipped when set) and the two
                // `.lit4` constants the init copies.
                let fresh = seed % 3 == 0;
                c.bus.write32(gp + 0x24d8, if fresh { 0 } else { 1 });
                c.bus.write32(gp.wrapping_sub(0x339c), 0x3f19_999a); // 0.6f
                c.bus.write32(gp.wrapping_sub(0x3398), 0x3ca3_d70a); // 0.02f
                                                                     // The entry type table (0x445ed0).
                for i in 0..12u32 {
                    c.bus.write32(0x0044_5ed0 + i * 4, (i * 7 + seed) % 4);
                }
                let template = 0x004f_b400;
                let id = 0x21 + (seed % 7);
                let word = match seed % 3 {
                    0 => 0,
                    1 => 0x100,
                    _ => 0x1_0000,
                };
                if !fresh {
                    c.bus.write32(template, id);
                    for i in 1..12u32 {
                        c.bus.write32(template + i * 4, r.next_u32());
                    }
                }
                // id != -1: `(*(u32**)(s + 0x1c))[id >> 8] >> 8`, a reference.
                let holder = DATA + 0x1000;
                let slots = DATA + 0x1100;
                let array = DATA + 0x1200;
                let node = DATA + 0x1400;
                c.bus.write32(gp + 0x16c8, holder);
                c.bus.write32(holder, slots);
                c.bus.write32(slots + 8, array);
                c.bus.write32(array + (id & 0xff) * 4, node);
                c.bus.write32(node + 0x1c, DATA + 0x1500);
                c.bus.write32(DATA + 0x1500 + (id >> 8) * 4, word);
                // obj = (i << 2); its +0xc word is the node the factory probes.
                let obj = (word >> 8) << 2;
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                // id == -1: the world's "current object" at +0x290.
                c.bus.write32(gp + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1290, DATA + 0x1600);
                c.bus.write32(DATA + 0x160c, node);
                c.bus.write32(DATA + 0x1600 + 0x80, 0);
                // `0x3421a0` divides by these globals.
                c.bus.write32(gp + 0x2a74, DATA + 0x1800);
                c.bus.write32(DATA + 0x1810, 0x40_0000);
                for i in 0..4u32 {
                    // The intersect info the ctor reads (+8, +0x24, +4, +0xc..).
                    c.bus.write32(node + 8 + i * 4, r.next_u32());
                }
                // The entries: idx 0..11, val, type.
                let e = DATA + 0x3000;
                let n = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    _ => (seed % 6) as u32,
                };
                for i in 0..12u32 {
                    let ent = e + i * 16;
                    c.bus.write32(ent, i % 12);
                    c.bus.write32(ent + 4, r.next_u32());
                    c.bus.write32(ent + 0xc, (seed + i) % 3);
                }
                let out = DATA + 0x7000;
                for i in 0..3u32 {
                    c.bus.write32(out + i * 4, r.next_u32());
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(e);
                c.g[A3][0] = 0;
            },
        );
    }
}
