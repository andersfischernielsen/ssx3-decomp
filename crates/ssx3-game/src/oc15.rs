//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_4c20, in_seq_instance_get_channel, "InSeq_InstanceGetChannel"),
    (0x0027_4d70, in_seq_instance_get_listener, "InSeq_InstanceGetListener"),
    (0x0027_5cd0, data_instance_man_free_info, "DataInstanceMan_FreeInfo"),
    (0x0027_5cf8, data_instance_man_alloc_info, "DataInstanceMan_AllocInfo"),
    (0x0027_7298, script_list_is_paused, "ScriptList_IsPaused"),
    (0x0027_7dd8, script_list_is_movie_entry, "ScriptList_IsMovieEntry"),
    (0x0027_7f08, script_bank_slot_clear, "ScriptBankSlot_Clear"),
    (0x0027_bd78, script_engine_pop_free_node, "ScriptEngine_PopFreeNode"),
    (0x0027_bd90, script_engine_push_free_node, "ScriptEngine_PushFreeNode"),
    (0x0027_d2f0, script_engine_get_binding_process, "ScriptEngine_GetBindingProcess"),
    (0x0027_d320, script_engine_get_script_obj, "ScriptEngine_GetScriptObj"),
    (0x0027_d540, script_engine_set_binding_filter, "ScriptEngine_SetBindingFilter"),
    (0x0027_d558, script_engine_clear_binding_filter, "ScriptEngine_ClearBindingFilter"),
    (0x0028_2150, script_engine_register_script, "ScriptEngine_RegisterScript"),
    (0x0028_2178, script_engine_unregister_script, "ScriptEngine_UnregisterScript"),
    (0x0028_2c88, script_engine_clear_process, "ScriptEngine_ClearProcess"),
    (0x0028_2cb0, script_engine_get_process, "ScriptEngine_GetProcess"),
    (0x0028_3200, script_obj_ctor, "ScriptObj_ctor"),
    (0x0028_3430, script_obj_activate, "ScriptObj_Activate"),
    (0x0028_3440, script_obj_deactivate, "ScriptObj_Deactivate"),
];

// ---------------------------------------------------------------------------
// Sequences.

/// `InSeqChannel *InSeq_InstanceGetChannel(InSeq *self, int index)` (0x274c20):
/// the `index`-th channel record (0x3c bytes each) whose array starts at
/// `self + 0x18`. Returns a pointer computed as `base + index*0x3c`.
pub fn in_seq_instance_get_channel(c: &mut Cpu) {
    c.cycles += 5;
    const CHANNELS: u32 = 0x18;
    const CHANNEL_SIZE: u32 = 0x3c;
    c.g[V1][0] = CHANNEL_SIZE as u64;
    let base = Addr(abi::arg(c, 0)).off(CHANNELS).lw(c);
    c.g[V0][0] = base;
    let offset = abi::mult(c, abi::arg(c, 1), CHANNEL_SIZE);
    c.g[A1][0] = sx(offset);
    c.g[V0][0] = sx((base as u32).wrapping_add(offset));
    abi::ret(c);
}

/// `InSeqListener *InSeq_InstanceGetListener(InSeq *self)` (0x274d70): the
/// listener cached at `self + 0x44` when set, otherwise the object reached
/// through `self + 0x8` (`*(self->+0x8)`).
pub fn in_seq_instance_get_listener(c: &mut Cpu) {
    c.cycles += 3;
    const CACHE: u32 = 0x44;
    const LINK: u32 = 0x8;
    let self_ = Addr(abi::arg(c, 0));
    let cached = self_.off(CACHE).lw(c);
    c.g[V0][0] = cached;
    if cached != 0 {
        c.cycles += 2;
    } else {
        c.cycles += 3;
        let link = self_.off(LINK).lw(c);
        c.g[V1][0] = link;
        c.g[V0][0] = Addr(link as u32).lw(c);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Data instance manager.

/// `void DataInstanceMan_FreeInfo(DataInstanceMan *man, DataInfo *info)`
/// (0x275cd0): push `info` onto the free list `man->free` (+0x8), clearing the
/// record's fields and threading the old head through `info + 0x14`.
pub fn data_instance_man_free_info(c: &mut Cpu) {
    c.cycles += 9;
    const FREE: u32 = 0x8;
    const NEXT: u32 = 0x14;
    let man = Addr(abi::arg(c, 0));
    let info = Addr(abi::arg(c, 1));
    info.off(0x8).sw(c, 0);
    info.off(0xc).sw(c, 0);
    info.off(0x10).sw(c, 0);
    info.sw(c, 0);
    info.off(0x4).sw(c, 0);
    let head = man.off(FREE).lw(c);
    c.g[V0][0] = head;
    info.off(NEXT).sw(c, head);
    man.off(FREE).sw(c, abi::arg64(c, 1));
    abi::ret(c);
}

/// `DataInfo *DataInstanceMan_AllocInfo(DataInstanceMan *man)` (0x275cf8):
/// pop the free list `man->free` (+0x8), returning the node and clearing its
/// next pointer (+0x14).
pub fn data_instance_man_alloc_info(c: &mut Cpu) {
    c.cycles += 6;
    const FREE: u32 = 0x8;
    const NEXT: u32 = 0x14;
    let man = Addr(abi::arg(c, 0));
    let info = man.off(FREE).lw(c);
    c.g[V1][0] = info;
    let next = Addr(info as u32).off(NEXT).lw(c);
    c.g[A1][0] = next;
    c.g[V0][0] = info;
    man.off(FREE).sw(c, next);
    Addr(info as u32).off(NEXT).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script lists.

/// `bool ScriptList_IsPaused(ScriptList *list)` (0x277298): true when the state
/// at +0xa4 is 3 and the pending count at +0xa8 is non-zero.
pub fn script_list_is_paused(c: &mut Cpu) {
    c.cycles += 4;
    const STATE: u32 = 0xa4;
    const PENDING: u32 = 0xa8;
    const PAUSED: u64 = 3;
    let list = Addr(abi::arg(c, 0));
    c.g[V1][0] = list.off(STATE).lw(c);
    c.g[V0][0] = PAUSED;
    let mut result = 0u64;
    if c.g[V1][0] == PAUSED {
        c.cycles += 2;
        let pending = list.off(PENDING).lw(c);
        c.g[V0][0] = pending;
        result = (0u64 < pending) as u64;
    }
    c.g[A1][0] = result;
    c.cycles += 2;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `bool ScriptList_IsMovieEntry(int *entry)` (0x277dd8): true when the signed
/// word at `entry` is at least 29 (i.e. not a movie entry when below 29).
pub fn script_list_is_movie_entry(c: &mut Cpu) {
    c.cycles += 4;
    let value = Addr(abi::arg(c, 1)).lw(c);
    c.g[V0][0] = ((value as i64) < 29) as u64;
    c.g[V0][0] ^= 0x1;
    abi::ret(c);
}

/// `void ScriptBankSlot_Clear(ScriptBankSlot *slot)` (0x277f08): reset a bank
/// slot: id -1 at +0x4, fields +0x0/+0x8/+0xc cleared.
pub fn script_bank_slot_clear(c: &mut Cpu) {
    c.cycles += 6;
    let slot = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    slot.off(0xc).sw(c, 0);
    slot.off(0x4).sw(c, c.g[V0][0]);
    slot.sw(c, 0);
    slot.off(0x8).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine: free-node list.

/// `ScriptNode *ScriptEngine_PopFreeNode(ScriptEngine *engine)` (0x27bd78):
/// pop `engine->freeNodes` (+0x548), returning the node and clearing its link
/// (+0x10).
pub fn script_engine_pop_free_node(c: &mut Cpu) {
    c.cycles += 6;
    const FREE: u32 = 0x548;
    const NEXT: u32 = 0x10;
    let engine = Addr(abi::arg(c, 0));
    let node = engine.off(FREE).lw(c);
    c.g[V1][0] = node;
    let next = Addr(node as u32).off(NEXT).lw(c);
    c.g[A1][0] = next;
    c.g[V0][0] = node;
    engine.off(FREE).sw(c, next);
    Addr(node as u32).off(NEXT).sw(c, 0);
    abi::ret(c);
}

/// `void ScriptEngine_PushFreeNode(ScriptEngine *engine, ScriptNode *node)`
/// (0x27bd90): push `node` onto `engine->freeNodes` (+0x548), setting its
/// fields to -1 and threading the old head through +0x10.
pub fn script_engine_push_free_node(c: &mut Cpu) {
    c.cycles += 9;
    const FREE: u32 = 0x548;
    const NEXT: u32 = 0x10;
    let engine = Addr(abi::arg(c, 0));
    let node = Addr(abi::arg(c, 1));
    let head = engine.off(FREE).lw(c);
    c.g[V1][0] = head;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    node.off(0xc).sw(c, c.g[V0][0]);
    node.off(NEXT).sw(c, head);
    node.sw(c, c.g[V0][0]);
    node.off(0x4).sw(c, c.g[V0][0]);
    node.off(0x8).sw(c, c.g[V0][0]);
    engine.off(FREE).sw(c, abi::arg64(c, 1));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine: bindings.

/// `ScriptProcess *ScriptEngine_GetBindingProcess(ScriptEngine *engine,
/// const Binding *binding)` (0x27d2f0): the process of the binding at
/// `engine->bindings (+0x50) + binding->index (+0x4)*0x14`, returned only when
/// its state (+0xc) is 1, else -1.
pub fn script_engine_get_binding_process(c: &mut Cpu) {
    c.cycles += 9;
    const BINDINGS: u32 = 0x50;
    const BINDING_SIZE: u32 = 0x14;
    const STATE: u32 = 0xc;
    const PROCESS: u32 = 0x8;
    let engine = abi::arg(c, 0);
    let index = Addr(abi::arg(c, 1)).off(0x4).lw(c);
    c.g[V0][0] = index;
    c.g[V1][0] = BINDING_SIZE as u64;
    c.g[A1][0] = 0x1;
    let offset = abi::mult(c, index as u32, BINDING_SIZE);
    let binding = engine.wrapping_add(offset).wrapping_add(BINDINGS);
    c.g[V0][0] = sx(binding);
    c.g[A0][0] = sx(binding);
    c.g[V1][0] = Addr(binding).off(STATE).lw(c);
    let mut result = 0xffff_ffff_ffff_ffffu64;
    if c.g[V1][0] == 0x1 {
        c.cycles += 1;
        result = Addr(binding).off(PROCESS).lw(c);
    }
    c.g[V0][0] = result;
    c.cycles += 2;
    abi::ret(c);
}

/// `ScriptObj *ScriptEngine_GetScriptObj(ScriptEngine *engine, int index)`
/// (0x27d320): the script object pointer at `engine + index*4`.
pub fn script_engine_get_script_obj(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    abi::ret(c);
}

/// `void ScriptEngine_SetBindingFilter(ScriptEngine *engine, const Binding
/// *binding, int filter)` (0x27d540): store `filter` at
/// `engine + binding->index (+0x4)*0x14 + 0x60`.
pub fn script_engine_set_binding_filter(c: &mut Cpu) {
    c.cycles += 6;
    const BINDING_SIZE: u32 = 0x14;
    const FILTER: u32 = 0x60;
    let engine = abi::arg(c, 0);
    let index = Addr(abi::arg(c, 1)).off(0x4).lw(c);
    c.g[V0][0] = index;
    c.g[V1][0] = BINDING_SIZE as u64;
    let offset = abi::mult(c, index as u32, BINDING_SIZE);
    c.g[A1][0] = sx(offset);
    let binding = engine.wrapping_add(offset);
    c.g[V0][0] = sx(binding);
    Addr(binding).off(FILTER).sw(c, abi::arg64(c, 2));
    abi::ret(c);
}

/// `void ScriptEngine_ClearBindingFilter(ScriptEngine *engine, const Binding
/// *binding)` (0x27d558): store -1 at
/// `engine + binding->index (+0x4)*0x14 + 0x60`.
pub fn script_engine_clear_binding_filter(c: &mut Cpu) {
    c.cycles += 7;
    const BINDING_SIZE: u32 = 0x14;
    const FILTER: u32 = 0x60;
    let engine = abi::arg(c, 0);
    let index = Addr(abi::arg(c, 1)).off(0x4).lw(c);
    c.g[V0][0] = index;
    c.g[V1][0] = BINDING_SIZE as u64;
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    let offset = abi::mult(c, index as u32, BINDING_SIZE);
    c.g[A2][0] = sx(offset);
    let binding = engine.wrapping_add(offset);
    c.g[V0][0] = sx(binding);
    Addr(binding).off(FILTER).sw(c, 0xffff_ffff_ffff_ffff);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine: registration table.

/// `bool ScriptEngine_RegisterScript(ScriptEngine *engine, ScriptObj *obj,
/// int index)` (0x282150): register `obj` in the table at `engine + 0x2b0`
/// (8-byte entries); false when the slot is already occupied.
pub fn script_engine_register_script(c: &mut Cpu) {
    c.cycles += 6;
    const TABLE: u32 = 0x2b0;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V1][0] = table;
    let index = abi::arg(c, 2);
    c.g[A2][0] = sx(index << 3);
    let slot = (table as u32).wrapping_add(index << 3);
    c.g[A2][0] = sx(slot);
    let existing = Addr(slot).lw(c);
    c.g[V0][0] = existing;
    c.g[V0][0] = 0x1;
    if existing == 0 {
        c.cycles += 2;
        Addr(slot).sw(c, abi::arg64(c, 1));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `ScriptObj *ScriptEngine_UnregisterScript(ScriptEngine *engine, int index)`
/// (0x282178): clear the table slot at `engine + 0x2b0 + index*8`, returning the
/// old pointer (0 when the slot was empty).
pub fn script_engine_unregister_script(c: &mut Cpu) {
    c.cycles += 6;
    const TABLE: u32 = 0x2b0;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let index = abi::arg(c, 1);
    c.g[A1][0] = sx(index << 3);
    let slot = (table as u32).wrapping_add(index << 3);
    c.g[A1][0] = sx(slot);
    let old = Addr(slot).lw(c);
    c.g[V0][0] = old;
    if old == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        Addr(slot).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine: process records.

/// `void ScriptEngine_ClearProcess(ScriptEngine *engine, ScriptProcess *proc)`
/// (0x282c88): reset a process record's fields (the engine argument is unused).
pub fn script_engine_clear_process(c: &mut Cpu) {
    c.cycles += 10;
    let proc = Addr(abi::arg(c, 1));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    proc.off(0xc).sw(c, 0);
    proc.off(0x14).sw(c, c.g[V0][0]);
    proc.sw(c, 0);
    proc.off(0x4).sw(c, 0);
    proc.off(0x20).sw(c, 0);
    proc.off(0x10).sh(c, 0);
    proc.off(0x12).sh(c, 0);
    proc.off(0x18).sw(c, 0);
    abi::ret(c);
}

/// `ScriptProcess *ScriptEngine_GetProcess(ScriptEngine *engine, int index)`
/// (0x282cb0): the process at `engine + 0x2c0 + index*0x24`, or null when its
/// +0x20 field is zero.
pub fn script_engine_get_process(c: &mut Cpu) {
    c.cycles += 7;
    const PROCESSES: u32 = 0x2c0;
    const PROCESS_SIZE: u32 = 0x24;
    const PRESENT: u32 = 0x20;
    let engine = abi::arg(c, 0);
    let offset = abi::mult(c, abi::arg(c, 1), PROCESS_SIZE).wrapping_add(PROCESSES);
    c.g[A1][0] = sx(offset);
    let proc = engine.wrapping_add(offset);
    c.g[V0][0] = sx(proc);
    c.g[V1][0] = Addr(proc).off(PRESENT).lw(c);
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script objects.

/// `ScriptObj *ScriptObj_ctor(ScriptObj *obj, int handle)` (0x283200):
/// initialise the object: `obj[0] = handle`, `obj[+0x4] = -1`, `obj[+0x8] = 0`,
/// `obj[+0xc] = vtable`.
pub fn script_obj_ctor(c: &mut Cpu) {
    c.cycles += 9;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x0048_0000;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x2558));
    obj.sw(c, abi::arg64(c, 1));
    obj.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = abi::arg64(c, 0);
    obj.off(0x4).sw(c, c.g[V1][0]);
    obj.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `void ScriptObj_Activate(ScriptObj *obj)` (0x283430): increment the
/// reference count at +0x8.
pub fn script_obj_activate(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let count = obj.off(0x8).lw(c);
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    obj.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void ScriptObj_Deactivate(ScriptObj *obj)` (0x283440): decrement the
/// reference count at +0x8.
pub fn script_obj_deactivate(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let count = obj.off(0x8).lw(c);
    c.g[V0][0] = sx((count as u32).wrapping_sub(1));
    obj.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA};

    #[test]
    fn in_seq_instance_get_channel_matches() {
        check_lohi(ssx3_ee::s07::f_00274c20, in_seq_instance_get_channel, 200, &|c, seed| {
            let mut r = Rng(0x274c_20 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 12) as u64;
            c.bus.write32(DATA + 0x18, r.next_u32());
        });
    }

    #[test]
    fn in_seq_instance_get_listener_matches() {
        check(ssx3_ee::s07::f_00274d70, in_seq_instance_get_listener, 200, &|c, seed| {
            let mut r = Rng(0x274d_70 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
            let link = DATA + 0x200;
            c.bus.write32(DATA + 0x8, link);
            c.bus.write32(link, r.next_u32());
        });
    }

    #[test]
    fn data_instance_man_free_info_matches() {
        check(ssx3_ee::s07::f_00275cd0, data_instance_man_free_info, 200, &|c, seed| {
            let mut r = Rng(0x275c_d0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x8, r.next_u32());
            for i in 0..6 {
                c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn data_instance_man_alloc_info_matches() {
        check(ssx3_ee::s07::f_00275cf8, data_instance_man_alloc_info, 200, &|c, seed| {
            let mut r = Rng(0x275c_f8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let head = DATA + 0x100;
            c.bus.write32(DATA + 0x8, if seed % 5 == 0 { 0 } else { head });
            c.bus.write32(head + 0x14, if seed % 4 == 0 { 0 } else { DATA + 0x140 });
        });
    }

    #[test]
    fn script_list_is_paused_matches() {
        check(ssx3_ee::s07::f_00277298, script_list_is_paused, 200, &|c, seed| {
            let mut r = Rng(0x2772_98 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0xa4, if seed % 3 == 0 { 3 } else { r.next_u32() });
            c.bus.write32(DATA + 0xa8, if seed % 4 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn script_list_is_movie_entry_matches() {
        check(ssx3_ee::s07::f_00277dd8, script_list_is_movie_entry, 200, &|c, seed| {
            let mut r = Rng(0x277d_d8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            let value = match seed % 4 {
                0 => 28,
                1 => 29,
                2 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA, value);
        });
    }

    #[test]
    fn script_bank_slot_clear_matches() {
        check(ssx3_ee::s07::f_00277f08, script_bank_slot_clear, 100, &|c, seed| {
            let mut r = Rng(0x277f_08 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn script_engine_pop_free_node_matches() {
        check(ssx3_ee::s07::f_0027bd78, script_engine_pop_free_node, 200, &|c, seed| {
            let mut r = Rng(0x27bd_78 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = DATA + 0x100;
            c.bus.write32(DATA + 0x548, if seed % 5 == 0 { 0 } else { node });
            c.bus.write32(node + 0x10, if seed % 4 == 0 { 0 } else { DATA + 0x140 });
        });
    }

    #[test]
    fn script_engine_push_free_node_matches() {
        check(ssx3_ee::s07::f_0027bd90, script_engine_push_free_node, 200, &|c, seed| {
            let mut r = Rng(0x27bd_90 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = DATA + 0x100;
            c.g[A1][0] = (node as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA + 0x548, r.next_u32());
            for i in 0..5 {
                c.bus.write32(node + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn script_engine_get_binding_process_matches() {
        check_lohi(ssx3_ee::s07::f_0027d2f0, script_engine_get_binding_process, 200, &|c, seed| {
            let mut r = Rng(0x27d2_f0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            let index = seed % 8;
            c.bus.write32(DATA + 0x400 + 4, index);
            let binding = DATA + 0x50 + index * 0x14;
            c.bus.write32(binding + 0xc, if seed % 3 == 0 { 1 } else { r.next_u32() });
            c.bus.write32(binding + 0x8, r.next_u32());
        });
    }

    #[test]
    fn script_engine_get_script_obj_matches() {
        check(ssx3_ee::s07::f_0027d320, script_engine_get_script_obj, 200, &|c, seed| {
            let mut r = Rng(0x27d3_20 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 16) as u64;
            c.g[A1][0] = index;
            c.bus.write32(DATA + index as u32 * 4, r.next_u32());
        });
    }

    #[test]
    fn script_engine_set_binding_filter_matches() {
        check_lohi(ssx3_ee::s07::f_0027d540, script_engine_set_binding_filter, 200, &|c, seed| {
            let mut r = Rng(0x27d5_40 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let index = seed % 8;
            c.bus.write32(DATA + 0x400 + 4, index);
            let binding = DATA + index * 0x14;
            c.bus.write32(binding + 0x60, r.next_u32());
        });
    }

    #[test]
    fn script_engine_clear_binding_filter_matches() {
        check_lohi(ssx3_ee::s07::f_0027d558, script_engine_clear_binding_filter, 200, &|c, seed| {
            let mut r = Rng(0x27d5_58 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            let index = seed % 8;
            c.bus.write32(DATA + 0x400 + 4, index);
            let binding = DATA + index * 0x14;
            c.bus.write32(binding + 0x60, r.next_u32());
        });
    }

    #[test]
    fn script_engine_register_script_matches() {
        check(ssx3_ee::s07::f_00282150, script_engine_register_script, 200, &|c, seed| {
            let mut r = Rng(0x2821_50 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let index = (seed % 4) as u64;
            c.g[A2][0] = index;
            let table = DATA + 0x800;
            c.bus.write32(DATA + 0x2b0, table);
            let slot = table + index as u32 * 8;
            c.bus.write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn script_engine_unregister_script_matches() {
        check(ssx3_ee::s07::f_00282178, script_engine_unregister_script, 200, &|c, seed| {
            let mut r = Rng(0x2821_78 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 4) as u64;
            c.g[A1][0] = index;
            let table = DATA + 0x800;
            c.bus.write32(DATA + 0x2b0, table);
            let slot = table + index as u32 * 8;
            c.bus.write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn script_engine_clear_process_matches() {
        check(ssx3_ee::s07::f_00282c88, script_engine_clear_process, 100, &|c, seed| {
            let mut r = Rng(0x282c_88 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..0x24 {
                c.bus.write8(DATA + 0x100 + i, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn script_engine_get_process_matches() {
        check_lohi(ssx3_ee::s07::f_00282cb0, script_engine_get_process, 200, &|c, seed| {
            let mut r = Rng(0x282c_b0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 4) as u64;
            c.g[A1][0] = index;
            let proc = DATA + 0x2c0 + index as u32 * 0x24;
            c.bus.write32(proc + 0x20, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn script_obj_ctor_matches() {
        check(ssx3_ee::s07::f_00283200, script_obj_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2832_00 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn script_obj_activate_matches() {
        check(ssx3_ee::s07::f_00283430, script_obj_activate, 100, &|c, seed| {
            let mut r = Rng(0x2834_30 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let value = match seed % 4 {
                0 => 0,
                1 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0x8, value);
        });
    }

    #[test]
    fn script_obj_deactivate_matches() {
        check(ssx3_ee::s07::f_00283440, script_obj_deactivate, 100, &|c, seed| {
            let mut r = Rng(0x2834_40 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let value = match seed % 4 {
                0 => 0,
                1 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0x8, value);
        });
    }
}
