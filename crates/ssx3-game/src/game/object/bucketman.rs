//! `src/object/bucketman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_4b38, process_list_shutdown, "ProcessList_Shutdown"),
    (0x0035_4c08, process_list_add, "ProcessList_Add"),
    (0x0035_4e48, process_list_remove, "ProcessList_Remove"),
    (0x0035_4ea8, process_list_first, "ProcessList_First"),
    (0x0035_4ed0, process_list_last, "ProcessList_Last"),
    (0x0035_4f20, process_list_prev, "ProcessList_Prev"),
    (
        0x0035_4f70,
        process_list_pending_next,
        "ProcessList_PendingNext",
    ),
    (
        0x0035_5118,
        process_list_delete_priority,
        "ProcessList_DeletePriority",
    ),
    (
        0x0035_51a8,
        process_list_delete_section_objects,
        "ProcessList_DeleteSectionObjects",
    ),
    (
        0x0035_5260,
        process_list_set_sorted_insert,
        "ProcessList_SetSortedInsert",
    ),
    (0x0035_5280, mod_obj_ctor, "ModObj_ctor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2;
}

// ---------------------------------------------------------------------------
// Process list.

/// `void ProcessList_Add(ProcessList *list, void *entry, uint index)`
/// (0x354c08): insert `entry` into the list at `index`, on the used or free
/// side depending on the slot's flag.
pub fn process_list_add(c: &mut Cpu) {
    c.cycles += 8;
    let list = abi::arg(c, 0);
    let entry = c.g[A1][0];
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(list).off(4).lw(c);
    c.g[A2][0] = sx(abi::mult(c, c.g[A2][0] as u32, 0x44));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] &= 0x1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.cycles += 13;
        Addr(entry as u32).off(4).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        Addr(c.g[V1][0] as u32).off(8).sw(c, entry);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        Addr(entry as u32).off(8).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(list).off(4).lw(c);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(8).sw(c, entry);
    } else {
        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.cycles += 14;
        Addr(entry as u32).off(4).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        Addr(c.g[V1][0] as u32).off(8).sw(c, entry);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24));
        Addr(entry as u32).off(8).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(list).off(4).lw(c);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(0x28).sw(c, entry);
    }
    abi::ret(c);
}

/// `void ProcessList_Remove(void *entry)` (0x354e48): unlink an entry from its
/// doubly linked list.
pub fn process_list_remove(c: &mut Cpu) {
    c.cycles += 3;
    let entry = Addr(abi::arg(c, 1));
    c.g[V1][0] = entry.off(4).lw(c);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = entry.off(8).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            Addr(c.g[V0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[V1][0] = entry.off(8).lw(c);
            c.g[V0][0] = entry.off(4).lw(c);
            Addr(c.g[V0][0] as u32).off(8).sw(c, c.g[V1][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* ProcessList_First(ProcessList *list, uint index)` (0x354ea8): the
/// head entry of slot `index`, or null.
pub fn process_list_first(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x44));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[A1][0] = c.g[V0][0] ^ c.g[A1][0];
    if c.g[A1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void* ProcessList_Last(ProcessList *list, uint index)` (0x354ed0): the
/// tail entry of slot `index`, or null.
pub fn process_list_last(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x44));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = c.g[V0][0] ^ c.g[A1][0];
    if c.g[A1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void* ProcessList_Prev(ProcessList *list, void *entry, uint index)`
/// (0x354f20): the previous entry of `entry` in slot `index`, or null.
pub fn process_list_prev(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A2][0] = sx(abi::mult(c, c.g[A2][0] as u32, 0x44));
    c.g[V0][0] = Addr(abi::arg(c, 1)).off(8).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.g[V0][0] ^ c.g[V1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void* ProcessList_PendingNext(ProcessList *list, void *entry, uint index)`
/// (0x354f70): the pending-next entry of `entry` in slot `index`, or null.
pub fn process_list_pending_next(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A2][0] = sx(abi::mult(c, c.g[A2][0] as u32, 0x44));
    c.g[V0][0] = Addr(abi::arg(c, 1)).off(4).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = c.g[V0][0] ^ c.g[V1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void ProcessList_SetSortedInsert(ProcessList *list, uint index)`
/// (0x355260): mark slot `index` as sorted-insert.
pub fn process_list_set_sorted_insert(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x44));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] |= 0x1;
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void cBucketMan_purgeBucket(cBucketMan* self, int index)` (0x355118): for
/// every node the indexed process list yields, call the node's virtual
/// `purge(3)` (slot 1 of the vtable at `node + 0xc`).
pub fn process_list_delete_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = sx(self_);
        c.g[S2][0] = sx(index);
        call_ptr(c, callee::PROCESS_LIST_FIRST, 0x0035_513c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, callee::PROCESS_LIST_NEXT, 0x0035_515c)?;
                let done = c.g[S0][0] == 0;
                c.g[S1][0] = c.g[V0][0];
                if !done {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, S0, 0xc));
                    c.g[A1][0] = 3;
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0035_517c)?;
                }
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                c.g[A0][0] = c.g[S3][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0035_5150);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Process list.

/// `void ProcessList_DeleteSectionObjects(ProcessList* l, int section, int id)`
/// (0x3551a8): walk the list and delete every section object whose key matches
/// `id`.
pub fn process_list_delete_section_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            c.g[S2][0] = c.g[A1][0];
            let this = r(c, A0);
            c.g[S3][0] = sx(this);
            frame.save(c, S4, 0x10);
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save_ra(c, 0);
            c.g[S4][0] = c.g[A2][0];
            callee::PROCESS_LIST_FIRST_2.call(c, 0x0035_51d4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                loop {
                    'entry: {
                        c.cycles += 11;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0035_51f4);
                        c.g[A2][0] = c.g[S2][0];
                        callee::PROCESS_LIST_NEXT_2.call(c, 0x0035_51f4)?;
                        c.g[V1][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
                        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
                        let target = r(c, V0);
                        c.g[RA][0] = sx(0x0035_520c);
                        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                        call_ptr(c, target, 0x0035_520c)?;
                        if c.g[V0][0] != c.g[S4][0] {
                            c.g[S0][0] = c.g[S1][0];
                            break 'entry;
                        }
                        'del: {
                            c.cycles += 2;
                            let gone = c.g[S0][0] == 0;
                            c.g[A1][0] = 3;
                            if gone {
                                break 'del;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                            let target = r(c, V1);
                            c.g[RA][0] = sx(0x0035_5230);
                            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                            call_ptr(c, target, 0x0035_5230)?;
                            break 'del;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S1][0];
                        break 'entry;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        frame.restore(c, 16, 0x50);
                        break 'outer;
                    }
                    c.g[A0][0] = c.g[S3][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_51e8);
                    }
                }
            }
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ProcessList_Shutdown(ProcessList *self)` (0x354b38): delete every
/// priority bucket and free the pending game processes.
pub fn process_list_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        'b_354b7c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_354b7c;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                'l_354b60: loop {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00354b68);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00355118, 0x00354b68)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00354b60);
                        }
                        continue 'l_354b60;
                    } else {
                        break 'b_354b7c;
                    }
                }
            }
        }
        'b_354be8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0x44;
            if t {
                break 'b_354be8;
            } else {
                'b_354be0: {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
                    c.mult(0, 4, 3, 2);
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if c.g[A1][0] == c.g[S0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        break 'b_354be0;
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                        'l_354ba0: loop {
                            c.cycles += 15;
                            c.g[A1][0] = 0x2;
                            c.g[RA][0] = sx(0x00354bac);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
                            call_ptr(c, 0x003546c8, 0x00354bac)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                            c.g[RA][0] = sx(0x00354bb8);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003546c8, 0x00354bb8)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[RA][0] = sx(0x00354bc4);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003546c8, 0x00354bc4)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[RA][0] = sx(0x00354bd0);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003546c8, 0x00354bd0)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            let t = c.g[V0][0] != c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00354ba0);
                                }
                                continue 'l_354ba0;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_354be0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x00354be8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x00317e98, 0x00354be8)?;
                break 'b_354be8;
            }
        }
        c.cycles += 7;
        c.sw(c.g[S1][0] as u32, 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `ModObj* ModObj::ModObj(list, type, node)` (0x355280): the `WorldObj`
/// constructor, then the ModObj vtable (0x491028), scale 1.0 (+0x24), flag
/// bit 0 of the half-word at +0x12, and the fields +0x1c, +0x20, +0x28
/// cleared. Returns `this`.
pub fn mod_obj_ctor(c: &mut Cpu) {
    const WORLD_OBJ_CTOR: Callee = Callee {
        addr: 0x0034_fb00,
        f: ee::s11::WorldObj_ctor,
    };
    const VTABLE: u32 = 0x0049_1028;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        WORLD_OBJ_CTOR.call(c, 0x0035_5294)?;
        let me = c.g[S0][0] as u32;
        let flags = c.lhu(me.wrapping_add(0x12)) | 0x1;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = sx(VTABLE);
        c.sw(me.wrapping_add(0xc), sx(VTABLE));
        c.g[V1][0] = flags;
        c.swc1(0, me.wrapping_add(0x24));
        c.g[V0][0] = c.g[S0][0];
        c.sh(me.wrapping_add(0x12), flags);
        c.sw(me.wrapping_add(0x1c), 0);
        c.sw(me.wrapping_add(0x20), 0);
        c.sw(me.wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
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
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base;

    pub(crate) use crate::game::bx::bxstringctor::tests::run;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_2;

    use ssx3_ee as ee;

    #[test]
    fn process_list_add_matches() {
        check_lohi(
            ssx3_ee::s11::f_00354c08,
            process_list_add,
            200,
            &|c, seed| {
                let mut r = Rng(0x101f ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let base = DATA + 0x200;
                let slot = base + index * 0x44;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
                c.g[A2][0] = index as u64;
                c.bus.write32(DATA + 4, base);
                let flag = if seed % 2 == 0 { 0 } else { 1 };
                c.bus.write32(slot, flag);
                c.bus.write32(slot + 8, DATA + 0x300);
                c.bus.write32(slot + 0x28, DATA + 0x400);
            },
        );
    }

    #[test]
    fn process_list_remove_matches() {
        check(
            ssx3_ee::s11::f_00354e48,
            process_list_remove,
            200,
            &|c, seed| {
                let mut r = Rng(0x1020 ^ seed as u64);
                scramble(c, &mut r);
                let entry = DATA;
                let prev = DATA + 0x100;
                let next = DATA + 0x200;
                c.g[A1][0] = entry as u64;
                c.bus
                    .write32(entry + 4, if seed % 4 >= 2 { prev } else { 0 });
                c.bus
                    .write32(entry + 8, if seed % 2 == 0 { 0 } else { next });
                c.bus.write32(prev + 4, r.next_u32());
                c.bus.write32(next + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn process_list_first_matches() {
        check_lohi(
            ssx3_ee::s11::f_00354ea8,
            process_list_first,
            200,
            &|c, seed| {
                let mut r = Rng(0x1021 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let base = DATA + 0x200;
                let slot = base + index * 0x44;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.bus.write32(DATA + 4, base);
                let v = if seed % 3 == 0 {
                    slot + 0x14
                } else {
                    r.next_u32()
                };
                c.bus.write32(slot + 8, v);
            },
        );
    }

    #[test]
    fn process_list_last_matches() {
        check_lohi(
            ssx3_ee::s11::f_00354ed0,
            process_list_last,
            200,
            &|c, seed| {
                let mut r = Rng(0x1022 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let base = DATA + 0x200;
                let slot = base + index * 0x44;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.bus.write32(DATA + 4, base);
                let v = if seed % 3 == 0 {
                    slot + 4
                } else {
                    r.next_u32()
                };
                c.bus.write32(slot + 0x1c, v);
            },
        );
    }

    #[test]
    fn process_list_prev_matches() {
        check_lohi(
            ssx3_ee::s11::f_00354f20,
            process_list_prev,
            200,
            &|c, seed| {
                let mut r = Rng(0x1023 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let base = DATA + 0x200;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = index as u64;
                c.bus.write32(DATA + 4, base);
                let target = base + index * 0x44 + 4;
                let v = if seed % 3 == 0 { target } else { r.next_u32() };
                c.bus.write32(DATA + 0x100 + 8, v);
            },
        );
    }

    #[test]
    fn process_list_pending_next_matches() {
        check_lohi(
            ssx3_ee::s11::f_00354f70,
            process_list_pending_next,
            200,
            &|c, seed| {
                let mut r = Rng(0x1024 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let base = DATA + 0x200;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = index as u64;
                c.bus.write32(DATA + 4, base);
                let target = base + index * 0x44 + 0x34;
                let v = if seed % 3 == 0 { target } else { r.next_u32() };
                c.bus.write32(DATA + 0x100 + 4, v);
            },
        );
    }

    #[test]
    fn process_list_set_sorted_insert_matches() {
        check_lohi(
            ssx3_ee::s11::f_00355260,
            process_list_set_sorted_insert,
            200,
            &|c, seed| {
                let mut r = Rng(0x1025 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 4) as u32;
                let base = DATA + 0x200;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.bus.write32(DATA + 4, base);
                c.bus.write32(base + index * 0x44, r.next_u32());
            },
        );
    }

    #[test]
    fn process_list_delete_priority_matches() {
        check(
            ee::s11::f_00355118__generated,
            process_list_delete_priority,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3551, 0);
                env(c);
                let this = DATA + 0x400;
                let index = (seed % 2) as u32;
                let base = DATA + 0x2000;
                let desc = base + 0x44 * index;
                let sentinel = desc + 0x14;
                w32(c, this + 4, base);
                let vt = DATA + 0x3200;
                slot(c, vt, 8, EMPTY);
                let n0 = DATA + 0x3000;
                let n1 = DATA + 0x3100;
                w32(c, n0 + 0xc, vt);
                w32(c, n1 + 0xc, vt);
                match seed % 3 {
                    0 => w32(c, desc + 8, sentinel),
                    1 => {
                        w32(c, desc + 8, n0);
                        w32(c, n0 + 4, n1);
                        w32(c, n1 + 4, sentinel);
                    }
                    _ => {
                        w32(c, desc + 8, n0);
                        w32(c, n0 + 4, sentinel);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = index as u64;
            },
        );
    }

    // ------------------------------------------------------ world painter

    // --------------------------------------------------------- process list

    #[test]
    fn process_list_delete_section_objects_matches() {
        check(
            ee::s11::f_003551a8__generated,
            process_list_delete_section_objects,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x3551, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, DATA, 0);
            },
        );
    }

    #[test]
    fn process_list_shutdown_matches() {
        strict(
            ee::s11::f_00354b38__generated,
            process_list_shutdown,
            20,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x354b);
                let block = run(
                    c,
                    0x0031_7d70,
                    ee::s10::f_00317d70,
                    &[0x80, 0x0048_0c30, 0x2000_0000, 0],
                ) as u32;
                c.g[A0][0] = (block.wrapping_add(0x10)) as u64;
                for i in 0..0x20 {
                    c.bus.write32(block.wrapping_add(i * 4), 0);
                }
            },
        );
    }

    /// `ModObj_ctor`: the `WorldObj_ctor` fixture (its test in
    /// game/object/floatingnode.rs): process lists, the world -> focus
    /// manager chain, node flags with or without 0x100, kinds 6/0x10/0x16 or
    /// random; the WorldObj constructor runs for real.
    #[test]
    fn mod_obj_ctor_matches() {
        use crate::difftest::{check, fill};
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(
            ee::s11::f_00355280__generated,
            mod_obj_ctor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_5280, 0);
                c.pc = 0x0035_5280;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                fill(c, &mut r, SELF, 0x10);
                fill(c, &mut r, NODE, 0x10);
                let flags = r.next_u32();
                w(
                    c,
                    NODE + 0x8,
                    if seed % 4 == 0 {
                        flags | 0x100
                    } else {
                        flags & !0x100
                    },
                );
                let list = seed % 4;
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                for i in 0..4 {
                    w(c, PROCS + 0x44 * i, r.next_u32());
                    w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                    w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
                }
                let kind = match (seed / 4) % 6 {
                    0 => 6,
                    1 => 0x10,
                    2 => 0x16,
                    _ => r.next_u32() % 0x20,
                };
                fill(c, &mut r, 0x0044_aff0, 0x20);
                // World -> game -> race -> focus manager.
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, WORLD + 0x100);
                w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
                w(c, WORLD + 0x200 + 0xa4, FOCUS);
                w(
                    c,
                    FOCUS + 0xd4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                w(c, FOCUS + 0xd8, r.next_u32() % 8);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(kind);
                c.g[A3][0] = sx(NODE);
            },
        );
    }
}
