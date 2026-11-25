//! `src/luno/lunovm.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::fepopup::at;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::scripter::ssxscriptengine::oc428_at;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_24a8, luno_vm_pass_through, "LunoVM_PassThrough"),
    (
        0x0022_4c78,
        luno_vm_register_read_ptr,
        "LunoVMRegister_ReadPtr",
    ),
    (0x0022_4d00, luno_table_dtor, "LunoTable_dtor"),
    (0x0022_4da0, c_luno_table_ctor, "cLunoTable_ctor"),
    (0x0022_4f30, cluno_table_free_data, "cLunoTable_FreeData"),
    (0x0022_5b90, luno_vm_register_dtor, "LunoVMRegister_dtor"),
    (0x0022_5ca0, luno_vm_register_read, "LunoVMRegister_Read"),
    (
        0x0022_6600,
        luno_vm_register_set_raw,
        "LunoVMRegister_SetRaw",
    ),
    (0x0022_6610, luno_vm_register_as_int, "LunoVMRegister_AsInt"),
    (
        0x0022_6618,
        luno_vm_register_as_float,
        "LunoVMRegister_AsFloat",
    ),
    (0x0022_6768, c_luno_table_entry_dtor, "cLunoTableEntry_dtor"),
    (0x0022_24b8, luno_vm_call, "LunoVM_Call"),
    (0x0022_5068, cluno_table_assign, "cLunoTable_Assign"),
    (0x0022_57e0, c_luno_table_write, "cLunoTable_Write"),
    (0x0022_5be8, luno_vm_register_write, "LunoVMRegister_Write"),
    (
        0x0022_4c00,
        luno_vm_register_write_ptr,
        "LunoVMRegister_WritePtr",
    ),
    (0x0022_4df0, c_luno_table_dtor, "cLunoTable_dtor"),
    (0x0022_27d0, luno_vm_execute, "LunoVM_Execute"),
    (0x0022_5338, c_luno_table_set, "cLunoTable_Set"),
    (0x0022_4e50, cluno_table_alloc, "cLunoTable_Alloc"),
    (0x0022_5248, c_luno_table_find, "cLunoTable_Find"),
    (0x0022_6620, luno_vm_register_as_id, "LunoVMRegister_AsId"),
    (
        0x0022_5de8,
        luno_static_init_destroy,
        "LunoVM_StaticInitDestroy",
    ),
    (0x0022_67f0, luno_vm_global_ctors, "LunoVM_GlobalCtors"),
    (0x0022_6810, luno_vm_global_dtors, "LunoVM_GlobalDtors"),
    (0x0022_4f68, luno_table_clear, "cLunoTable_Clear"),
    (0x0022_50e0, oc681_luno_table_find_int, "cLunoTable_FindInt"),
    (0x0022_59a8, oc681_luno_table_read, "cLunoTable_Read"),
    (
        0x0022_6628,
        oc681_luno_vm_register_add,
        "LunoVMRegister_Add",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::C_LUNO_TABLE_ALLOC;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::LUNO_TABLE_DTOR as LUNO_TABLE_DTOR_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;

    pub(crate) use crate::game::fe::fepopup::callee::C00224C78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224DA0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224DF0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259A8;

    pub(crate) use crate::game::fe::fepopup::callee::C00225068;

    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
}

/// `void LunoVMRegister_dtor(cLunoValue *self, int flags)` (0x00225b90):
/// release a table reference when the value's type is 4, clear the type, and
/// free the value when `flags & 1`.
pub fn luno_vm_register_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.ldw(off(c.g[S0][0] as u32, 8));
        let is_table = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if is_table {
            c.cycles += 2;
            c.g[A1][0] = 2;
            callee::LUNO_TABLE_DTOR.call(c, 0x0022_5bbc)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let no_free = c.g[V0][0] == 0;
        c.stw(off(c.g[S0][0] as u32, 8), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_5bd0)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno table and crowd manager.

/// `cLunoTable_ctor(mem, a1)` 0x224da0: clear the table pointer, build the
/// table, return the object.
pub fn c_luno_table_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.sw(this as u32, 0);
        callee::C_LUNO_TABLE_ALLOC.call(c, 0x0022_4db8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno / movie / memory card managers.

/// `void cLunoTable_FreeData(cLunoTableRef* ref)` (0x224f30): destroy and clear
/// the table the reference holds.
pub fn cluno_table_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[A0][0] == 0 {
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::LUNO_TABLE_DTOR_2.call(c, 0x0022_4f54)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* LunoVMRegister_SetRaw(Register* r, void* value, void* tag)`
/// (0x226600): store the value and its tag in the register object.
pub fn luno_vm_register_set_raw(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    Addr(r.wrapping_add(0x4)).sw(c, c.g[A2][0]);
    let v = c.g[A1][0];
    Addr(r).sw(c, v);
    abi::ret(c);
}

/// `void* LunoVM_PassThrough(void* value)` (0x2224a8): the identity converter.
pub fn luno_vm_pass_through(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* LunoVMRegister_AsFloat(Register* r)` (0x226618): the float view of
/// a register (the same word).
pub fn luno_vm_register_as_float(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* LunoVMRegister_AsInt(Register* r)` (0x226610): the integer view of
/// a register (the same word).
pub fn luno_vm_register_as_int(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Luno VM.

/// `void LunoVMRegister_ReadPtr(LunoVMRegister* self, void* stream, int* out)`
/// (0x224c78): read a 32-bit base and a 32-bit offset from the stream and
/// publish `base + offset` at `out[0]` and the offset at `out[1]`.
pub fn luno_vm_register_read_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0022_4cb4);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0022_4cb4)?;
        c.g[V1][0] = Addr(r32(c, SP)).lw(c);
        c.g[A1][0] = c.g[SP][0] | 0x4;
        c.g[A2][0] = 0x4;
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V1)));
        Addr(r32(c, S2)).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0022_4cdc);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0022_4cdc)?;
        c.g[V0][0] = Addr(p(r32(c, SP), 4)).lw(c);
        frame.restore(c, S0, 0x40);
        Addr(p(r32(c, S2), 4)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cLunoTableEntry_dtor(cLunoTableEntry* self, int flags)` (0x226768):
/// release the owned register value, destroy the two embedded registers and
/// free the entry when `flags & 1`.
pub fn c_luno_table_entry_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0047_9890);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x1c)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0022_67ac);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0022_67ac)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[RA][0] = sx(0x0022_67b8);
        c.g[A1][0] = 0x2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0022_67b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0022_67c4);
        c.g[A1][0] = 0x2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0022_67c4)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0022_67d8);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_67d8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno table.

/// `void LunoTable_dtor(LunoTable* self, int flags)` (0x224d00): call the
/// destructor of every entry in the block at `+0x10` (0x20 bytes each, count at
/// block-0x10), free it, and delete the table when `flags & 1`.
pub fn luno_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
        let empty = c.g[V1][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xffff_fff0));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if c.g[V1][0] == c.g[S0][0] {
                c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                loop {
                    c.cycles += 9;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x1c));
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 8)).lh(c);
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                    c.g[V1][0] = sx(c.g[V1][0] as u32);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0022_4d58)?;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4d40);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0022_4d70)?; // EA_MemFree
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0022_4d84)?; // __builtin_delete
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_00225CA0(cLunoValue* self, sVEntry00224C00** stream, int base)`
/// (0x225ca0): read one Luno value from `stream` into `self`. A stream type of
/// 5 reads a pair, 4 a table, anything else is stored as the value type
/// after calling the stream's "read" slot; a previous table value at `self`
/// is destroyed first.
pub fn luno_vm_register_read(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_225dcc: {
        'b_225dc8: {
            'b_225d44: {
                c.cycles += 20;
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x40);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x30);
                c.g[A1][0] = sx(at(c, SP, 0x10));
                frame.save_ra(c, 0x20);
                c.g[S3][0] = c.g[A2][0];
                c.g[A2][0] = 0x4;
                c.g[V1][0] = lw(c, r32(c, S1));
                c.g[A0][0] = lh(c, at(c, V1, 0x10));
                c.g[V0][0] = lw(c, at(c, V1, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00225ce0);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00225ce0).is_err() {
                    return;
                }
                c.g[S2][0] = lw(c, at(c, SP, 0x10));
                c.g[V0][0] = 0x5;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[V0][0] = 0x4;
                if t {
                    break 'b_225d44;
                } else {
                    'b_225d28: {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        sw(c, r32(c, SP), 0u64);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00225d08);
                        sw(c, at(c, SP, 0x4), 0u64);
                        if callee::C00224C78.call(c, 0x00225d08).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at(c, S0, 0x8));
                        c.g[V0][0] = 0x4;
                        if c.g[V1][0] != c.g[V0][0] {
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d28;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00225d24);
                            c.g[A1][0] = 0x2;
                            if callee::C00224DF0.call(c, 0x00225d24).is_err() {
                                return;
                            }
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d28;
                        }
                    }
                    c.cycles += 7;
                    sw(c, r32(c, S0), 0u64);
                    ldl(c, 2, r32(c, SP));
                    sdl(c, 2, r32(c, S0));
                    let t = true;
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    if t {
                        break 'b_225dc8;
                    } else {
                        break 'b_225d44;
                    }
                }
            }
            'b_225dac: {
                c.cycles += 2;
                if c.g[S2][0] != c.g[V0][0] {
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    break 'b_225dac;
                } else {
                    'b_225d84: {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00225d58);
                        c.g[A1][0] = 0x20;
                        if callee::C00224DA0.call(c, 0x00225d58).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00225d68);
                        c.g[A0][0] = c.g[SP][0];
                        if callee::C002259A8.call(c, 0x00225d68).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, at(c, S0, 0x8));
                        if c.g[V0][0] != c.g[S2][0] {
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d84;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00225d80);
                            c.g[A1][0] = 0x2;
                            if callee::C00224DF0.call(c, 0x00225d80).is_err() {
                                return;
                            }
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d84;
                        }
                    }
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    sw(c, r32(c, S0), 0u64);
                    c.g[RA][0] = sx(0x00225d94);
                    c.g[A1][0] = c.g[SP][0];
                    if callee::C00225068.call(c, 0x00225d94).is_err() {
                        return;
                    }
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00225da4);
                    c.g[A1][0] = 0x2;
                    if callee::C00224DF0.call(c, 0x00225da4).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x60);
                    if t {
                        break 'b_225dcc;
                    } else {
                        break 'b_225dac;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x4;
            c.g[V0][0] = lw(c, r32(c, S1));
            c.g[A0][0] = lh(c, at(c, V0, 0x10));
            c.g[V1][0] = lw(c, at(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00225dc8);
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00225dc8).is_err() {
                return;
            }
            break 'b_225dc8;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x60);
        break 'b_225dcc;
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// LunoVM_Call (0x2224b8)

/// `operator new(size, name, flags, align)` (through the function table).
pub(crate) const LUNO_CALL_NEW_NAMED: u32 = 0x0031_7e30;

/// The allocation name (`D_00479780`).
pub(crate) const LUNO_CALL_NEW_NAME: u32 = 0x0047_9780;

/// The allocation flags.
pub(crate) const LUNO_CALL_NEW_FLAGS: u64 = 0x2000_0000;

/// The VM's default second register (`D_004C9098`, a `cLunoValue`).
pub(crate) const LUNO_CALL_DEFAULT_VALUE: u32 = 0x004c_9098;

/// `cLunoValue`: 12 bytes, `{value (8 bytes, a table pointer for type 4), type}`.
pub(crate) const LUNO_VALUE_SIZE: u64 = 0xc;

pub(crate) const LUNO_VALUE_TYPE: u32 = 0x8;

/// Value types: a table reference (counted) and an 8-byte pair.
pub(crate) const LUNO_TYPE_TABLE: u64 = 4;

pub(crate) const LUNO_TYPE_PAIR: u64 = 5;

pub(crate) const LUNO_CALL_TABLE_DTOR: Callee = Callee {
    addr: 0x0022_4df0,
    f: ee::s05::cLunoTable_dtor,
};

pub(crate) const LUNO_CALL_TABLE_ASSIGN: Callee = Callee {
    addr: 0x0022_5068,
    f: ee::s05::cLunoTable_Assign,
};

pub(crate) const LUNO_CALL_EXECUTE: Callee = Callee {
    addr: 0x0022_27d0,
    f: ee::s05::LunoVM_Execute,
};

/// `void LunoVM_Call(cLunoValue** out, LunoState* state, cLunoValue* table,
/// void* ctx)` (0x2224b8): run a Luno script function. Allocates the
/// register file (`n` values, `n` read from the script's function record at
/// `*state + *(*state + 4) + 8`, a 16-byte count header in front, every
/// value cleared), stores it in `*out`, sets register 0 to a reference to
/// `table` and register 1 to a copy of the VM default value (`D_004C9098`:
/// a table reference is counted, a type-5 pair copied as 8 bytes, anything
/// else as one word), each first releasing an old table reference, then
/// runs `LunoVM_Execute(out, state, ctx)`.
pub fn luno_vm_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = LUNO_VALUE_SIZE;
        for (reg, off) in [
            (S1, 0x70),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
            (S0, 0x80),
            (S2, 0x60),
        ] {
            frame.save(c, reg, off);
        }
        c.g[S3][0] = c.g[A2][0]; // table
        c.g[S4][0] = c.g[A0][0]; // out
        c.g[S6][0] = c.g[A1][0]; // state
        c.g[S7][0] = c.g[A3][0]; // ctx
        c.g[A2][0] = LUNO_CALL_NEW_FLAGS;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(LUNO_CALL_NEW_NAME);
        c.g[A3][0] = 0;
        c.g[S5][0] = 0x4d_0000;
        // n = the register count of the function record.
        let script = Addr(r32(c, S6)).lw(c);
        let code_off = Addr(script as u32).off(4).lw(c);
        c.g[A0][0] = code_off;
        let record = (script as u32).wrapping_add(code_off as u32);
        c.g[V0][0] = sx(record);
        c.g[S0][0] = Addr(record).off(8).lw(c);
        c.mult(0, V1, S0, V1); // v1 = n * 12
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x10));
        call_ptr(c, LUNO_CALL_NEW_NAMED, 0x0022_2524)?;

        // The count header, then clear every value.
        let block = r32(c, V0);
        c.g[S1][0] = sx(block.wrapping_add(0x10));
        let w_s0 = c.g[S0][0];
        Addr(block).sw(c, w_s0);
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        c.g[V0][0] = c.g[S1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
            loop {
                c.cycles += 7;
                let value = r32(c, V0);
                Addr(value).off(LUNO_VALUE_TYPE).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                Addr(value).sw(c, 0);
                let more = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = sx(value.wrapping_add(LUNO_VALUE_SIZE as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_2540);
                }
            }
        }

        // Register 0: release an old table reference, then reference `table`.
        c.cycles += 4;
        let regs = r32(c, S1);
        c.g[V0][0] = Addr(regs).off(LUNO_VALUE_TYPE).lw(c);
        c.g[S2][0] = LUNO_TYPE_TABLE;
        let w_s1 = c.g[S1][0];
        Addr(r32(c, S4)).sw(c, w_s1);
        if c.g[V0][0] == LUNO_TYPE_TABLE {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            LUNO_CALL_TABLE_DTOR.call(c, 0x0022_2578)?;
        }
        c.cycles += 11;
        c.g[A1][0] = c.g[S3][0];
        let regs = r32(c, S1);
        Addr(regs).off(LUNO_VALUE_TYPE).sw(c, 0);
        Addr(regs).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        LUNO_CALL_TABLE_ASSIGN.call(c, 0x0022_258c)?;
        c.g[S3][0] = sx(LUNO_CALL_DEFAULT_VALUE);
        c.g[S0][0] = Addr(r32(c, S4)).lw(c);
        let w_s2 = c.g[S2][0];
        Addr(r32(c, S1)).off(LUNO_VALUE_TYPE).sw(c, w_s2);

        // Register 1: release an old table reference.
        let regs = r32(c, S0);
        c.g[V0][0] = Addr(regs)
            .off(LUNO_VALUE_SIZE as u32 + LUNO_VALUE_TYPE)
            .lw(c);
        c.g[S1][0] = sx(regs.wrapping_add(LUNO_VALUE_SIZE as u32));
        if c.g[V0][0] == c.g[S2][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            LUNO_CALL_TABLE_DTOR.call(c, 0x0022_25b0)?;
        }

        // Register 1 = a copy of the default value.
        c.cycles += 5;
        let reg1 = r32(c, S1);
        Addr(reg1).off(LUNO_VALUE_TYPE).sw(c, 0);
        let regs = r32(c, S0);
        Addr(regs).off(0xc).sw(c, 0);
        let default = Addr(LUNO_CALL_DEFAULT_VALUE);
        c.g[V1][0] = default.off(LUNO_VALUE_TYPE).lw(c);
        c.g[V0][0] = LUNO_TYPE_PAIR;
        if c.g[V1][0] == c.g[S2][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            LUNO_CALL_TABLE_ASSIGN.call(c, 0x0022_25d0)?;
        } else {
            c.cycles += 2;
            let pair = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = default.lw(c);
            let regs = r32(c, S0);
            if pair {
                // `ldl/ldr` + `sdl/sdr`: an unaligned 8-byte copy.
                c.cycles += 7;
                c.g[A1][0] = sx(LUNO_CALL_DEFAULT_VALUE);
                c.g[V0][0] = default.ld(c);
                let lo = default.lw(c);
                let hi = default.off(4).lw(c);
                Addr(regs).off(0xc).sw(c, lo);
                Addr(regs).off(0x10).sw(c, hi);
            } else {
                c.cycles += 2;
                let w_v0 = c.g[V0][0];
                Addr(regs).off(0xc).sw(c, w_v0);
            }
        }
        c.g[V0][0] = default.off(LUNO_VALUE_TYPE).lw(c);

        c.cycles += 16;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = c.g[S7][0];
        let w_v0 = c.g[V0][0];
        Addr(r32(c, S1)).off(LUNO_VALUE_TYPE).sw(c, w_v0);
        LUNO_CALL_EXECUTE.call(c, 0x0022_2618)?;
        for (reg, off) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const LUNO_TYPE_PTR: u64 = 5;

pub(crate) const LUNO_STREAM_WRITE: u32 = 0x8;

/// `void LunoVMRegister_Write(cLunoValue* self, Stream* stream, int base)`
/// (0x225be8): save a Luno register: write its type tag (4 bytes), then a
/// pointer relative to `base` (`LunoVMRegister_WritePtr`), a table
/// (`cLunoTable_Write`) or the 4-byte payload. Frame 0x40 (s0-s2).
pub fn luno_vm_register_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        // stream->Write(&self->type, 4)
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(LUNO_VALUE_TYPE));
        c.g[A2][0] = 4;
        let vt = Addr(r32(c, S1)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE)).lh(c);
        let write = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE + 4)).lw(c);
        c.g[V0][0] = write;
        c.g[A0][0] = abi::addu(c.g[S1][0], delta);
        call_ptr(c, write as u32, 0x0022_5c24)?;
        let ty = Addr(r32(c, S0).wrapping_add(LUNO_VALUE_TYPE)).lw(c);
        c.g[V1][0] = ty;
        c.g[V0][0] = LUNO_TYPE_TABLE;
        if ty == LUNO_TYPE_PTR {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            Callee {
                addr: 0x0022_4c00,
                f: ee::s05::f_00224c00,
            }
            .call(c, 0x0022_5c44)?;
        } else {
            c.cycles += 2;
            if ty == LUNO_TYPE_TABLE {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                Callee {
                    addr: 0x0022_57e0,
                    f: ee::s05::f_002257e0,
                }
                .call(c, 0x0022_5c64)?;
            } else {
                // stream->Write(self, 4)
                let vt = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = vt;
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 4;
                let delta = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE)).lh(c);
                let write = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE + 4)).lw(c);
                c.g[V1][0] = write;
                c.g[A0][0] = abi::addu(c.g[S1][0], delta);
                call_ptr(c, write as u32, 0x0022_5c84)?;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cLunoTable_Assign (0x225068, lunovm.cpp)

/// cLunoTable field: the reference count.
pub(crate) const LTA_REFCOUNT: u32 = 0x8;

/// `cLunoTable_FreeData(cLunoTableRef*)`: destroys and frees the table.
pub(crate) const LTA_FREE_DATA: Callee = Callee {
    addr: 0x0022_4f30,
    f: ee::s05::cLunoTable_FreeData,
};

/// `cLunoTableRef* cLunoTable_Assign(cLunoTableRef* self, cLunoTableRef*
/// other)` (0x225068): reference assignment. When the tables differ, drops
/// the old reference (freeing the table at refcount 0), takes `other`'s
/// table and increments its refcount; returns `self`.
pub fn cluno_table_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if !same {
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                // Drop the old reference.
                c.cycles += 4;
                let old = c.g[V1][0] as u32;
                c.g[V0][0] =
                    sx((Addr(old.wrapping_add(LTA_REFCOUNT)).lw(c) as u32).wrapping_sub(1));
                let live = c.g[V0][0] != 0;
                Addr(old.wrapping_add(LTA_REFCOUNT)).sw(c, c.g[V0][0]);
                if !live {
                    c.cycles += 2;
                    LTA_FREE_DATA.call(c, 0x0022_50b0)?;
                }
                c.cycles += 1;
            }
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            // Take the new one.
            c.cycles += 5;
            let new = c.g[V1][0] as u32;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx((Addr(new.wrapping_add(LTA_REFCOUNT)).lw(c) as u32).wrapping_add(1));
            Addr(new.wrapping_add(LTA_REFCOUNT)).sw(c, c.g[V0][0]);
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cLunoTable_Write (0x2257e0)

pub(crate) const LUNO_REGISTER_WRITE: Callee = Callee {
    addr: 0x0022_5be8,
    f: ee::s05::LunoVMRegister_Write,
};

/// Table: highest bucket index (-1 without a hash part) and the node array.
pub(crate) const LUNO_TABLE_MODULUS: u32 = 0xc;

pub(crate) const LUNO_TABLE_NODES: u32 = 0x10;

/// Hash node (0x20 bytes): key register (+0), value register (+0xc, its
/// type tag at +0x14) and the chain link (+0x18).
pub(crate) const LUNO_NODE_VALUE: u32 = 0xc;

pub(crate) const LUNO_NODE_VALUE_TYPE: u32 = 0x14;

pub(crate) const LUNO_NODE_NEXT: u32 = 0x18;

pub(crate) const LUNO_NODE_SIZE: u32 = 0x20;

/// `r += 1` on a 32-bit counter register (`addiu r, r, 1`).
pub(crate) fn luno_table_inc(c: &mut Cpu, r: usize) {
    c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(1));
}

/// `stream->Write(data, n)` (vtable slot 8) returning to `ret`; `vt`/`m`
/// are the registers the original loads the vtable and method into.
pub(crate) fn luno_table_stream_write(
    c: &mut Cpu,
    vt: usize,
    m: usize,
    ret: u32,
) -> crate::call::Flow {
    let stream = r32(c, S2);
    c.g[vt][0] = Addr(stream).lw(c);
    let table = r32(c, vt);
    c.g[A0][0] = Addr(table).off(8).lh(c);
    c.g[m][0] = Addr(table).off(0xc).lw(c);
    let method = r32(c, m);
    c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void cLunoTable_Write(cLunoTableRef *self, Stream *stream, int base)`
/// (0x2257e0): save a Luno table: the table pointer (8 bytes), the number
/// of live entries (non-nil values over every bucket and its chain), then
/// each live entry's key and value register (`LunoVMRegister_Write`).
pub fn c_luno_table_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0]; // stream
        frame.save(c, S6, 0x20);
        c.g[S3][0] = c.g[A2][0]; // base
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 8;
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        // stream->Write(self->table, 8) (the a1 load sits before the method's).
        let stream = r32(c, S2);
        c.g[V1][0] = Addr(stream).lw(c);
        c.g[A1][0] = Addr(r32(c, S6)).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0022_582c)?;

        // Count the live entries into the local at sp+0 (a1; a3 = bucket,
        // t0 = modulus, a0 = node, a2 = next bucket).
        c.g[V1][0] = Addr(r32(c, S6)).lw(c);
        c.g[A3][0] = 0;
        let count_at = Addr(sp(c));
        count_at.sw(c, 0);
        let table = r32(c, V1);
        c.g[V0][0] = Addr(table).off(LUNO_TABLE_MODULUS).lw(c);
        c.g[A0][0] = Addr(table).off(LUNO_TABLE_NODES).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                if c.g[V0][0] != 0 {
                    luno_table_inc(c, A1);
                }
                c.cycles += 4;
                let node = r32(c, A0);
                c.g[V1][0] = Addr(node).off(LUNO_NODE_NEXT).lw(c);
                luno_table_inc(c, A3);
                c.g[A2][0] = sx(node.wrapping_add(LUNO_NODE_SIZE));
                if c.g[V1][0] != 0 {
                    // The chain (a0 counts, v1 walks).
                    c.cycles += 3;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                    loop {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            luno_table_inc(c, A0);
                        }
                        c.cycles += 3;
                        c.g[V1][0] = Addr(r32(c, V1)).off(LUNO_NODE_NEXT).lw(c);
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_5878);
                            }
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = c.g[A0][0];
                            break;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                c.g[A0][0] = c.g[A2][0];
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_5850);
                    }
                } else {
                    c.cycles += 1;
                    let count = c.g[A1][0];
                    Addr(sp(c)).sw(c, count);
                    break;
                }
            }
        }

        // stream->Write(&count, 4)
        c.cycles += 10;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        luno_table_stream_write(c, V0, V1, 0x0022_58bc)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        c.g[S5][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S6)).lw(c);
            let table = r32(c, V0);
            c.g[V1][0] = Addr(table).off(LUNO_TABLE_MODULUS).lw(c);
            c.g[S1][0] = Addr(table).off(LUNO_TABLE_NODES).lw(c);
            if (c.g[V1][0] as i64) >= 0 {
                // s1 = bucket node, s0 = chain node / value, s5 = bucket
                // index, s4 = next bucket.
                loop {
                    c.cycles += 3;
                    let node = r32(c, S1);
                    c.g[V0][0] = Addr(node).off(LUNO_NODE_VALUE_TYPE).lw(c);
                    c.g[S0][0] = sx(node.wrapping_add(LUNO_NODE_VALUE));
                    if c.g[V0][0] != 0 {
                        c.cycles += 11;
                        luno_table_write_entry(c, S1, S0, 0x0022_5900, 0x0022_5910)?;
                    }
                    c.cycles += 4;
                    let node = r32(c, S1);
                    c.g[S0][0] = Addr(node).off(LUNO_NODE_NEXT).lw(c);
                    luno_table_inc(c, S5);
                    c.g[S4][0] = sx(node.wrapping_add(LUNO_NODE_SIZE));
                    if c.g[S0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(r32(c, S0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                        loop {
                            c.cycles += 2;
                            let link = r32(c, S0);
                            c.g[S1][0] = sx(link.wrapping_add(LUNO_NODE_VALUE));
                            if c.g[V0][0] != 0 {
                                c.cycles += 11;
                                luno_table_write_entry(c, S0, S1, 0x0022_594c, 0x0022_595c)?;
                            }
                            c.cycles += 3;
                            c.g[S0][0] = Addr(r32(c, S0)).off(LUNO_NODE_NEXT).lw(c);
                            if c.g[S0][0] != 0 {
                                c.g[V0][0] = Addr(r32(c, S0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0022_5928);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, S6)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_TABLE_MODULUS).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    c.g[S1][0] = c.g[S4][0];
                    if c.g[V0][0] == 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_58d8);
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One live entry: `--count`, then `LunoVMRegister_Write(key)` and
/// `LunoVMRegister_Write(value)` (registers `key`/`value` hold them; `ret_*`
/// are the two return addresses).
pub(crate) fn luno_table_write_entry(
    c: &mut Cpu,
    key: usize,
    value: usize,
    ret_key: u32,
    ret_value: u32,
) -> crate::call::Flow {
    let count_at = Addr(sp(c));
    c.g[V0][0] = count_at.lw(c);
    c.g[A0][0] = c.g[key][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let count = c.g[V0][0];
    count_at.sw(c, count);
    LUNO_REGISTER_WRITE.call(c, ret_key)?;
    c.g[A0][0] = c.g[value][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    LUNO_REGISTER_WRITE.call(c, ret_value)
}

// ---------------------------------------------------------------------------
// cLunoTable_dtor (0x00224df0)

/// `cLunoTableRef::~cLunoTableRef(int flags)` (0x00224df0): drop one
/// reference to the shared Luno table (`self->table->refCount`, +0x8); the
/// last reference frees the table's data (`cLunoTable_FreeData(self)`).
/// `flags & 1` also deletes the reference itself (`operator delete`).
pub fn c_luno_table_dtor(c: &mut Cpu) {
    const TABLE_REFCOUNT: u32 = 0x8;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        let table = Addr(this).lw(c);
        c.g[V1][0] = table;
        let refs = sx((Addr(table as u32).off(TABLE_REFCOUNT).lw(c) as u32).wrapping_sub(1));
        c.g[V0][0] = refs;
        Addr(table as u32).off(TABLE_REFCOUNT).sw(c, refs);
        if refs == 0 {
            // Last reference: a0 is still `this`.
            c.cycles += 2;
            LUNO_TABLE_FREE_DATA.call(c, 0x0022_4e24)?;
        }
        c.cycles += 3;
        let delete_self = c.g[S1][0] & 1;
        c.g[V0][0] = delete_self;
        if delete_self != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_4e38)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cLunoTable_FreeData` (0x00224f30).
pub(crate) const LUNO_TABLE_FREE_DATA: Callee = Callee {
    addr: 0x0022_4f30,
    f: ee::s05::cLunoTable_FreeData,
};

// ---------------------------------------------------------------------------
// LunoVMRegister_WritePtr (0x224c00, lunovm.cpp)

/// The output stream's vtable slot `Write(this, const void* data, int n)`
/// (`{delta +8, fn +0xc}`).
pub(crate) const LVW_WRITE_SLOT: u32 = 0x8;

/// `void LunoVMRegister_WritePtr(LunoVMRegister* self, Stream* out, int base)`
/// (0x224c00): writes a pointer register as two words through the stream's
/// virtual `Write`: the pointer relative to `base`, then the register's
/// second word (+4). Frame 0x40 (the two words at 0 / 4, ra at 0x10, s1 /
/// s0 at 0x20 / 0x30).
pub fn luno_vm_register_write_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        let out = r32(c, S0);
        c.g[V1][0] = c.lw(out);
        c.g[V0][0] = c.lw(r32(c, S1));
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(LVW_WRITE_SLOT));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A2)));
        c.g[A3][0] = c.lw(vt.wrapping_add(LVW_WRITE_SLOT + 4));
        c.g[A0][0] = sx(out.wrapping_add(r32(c, A0)));
        c.sw(sp(c), c.g[V0][0]);
        c.g[A2][0] = 4;
        let method = r32(c, A3);
        call_ptr(c, method, 0x0022_4c40)?;
        let out = r32(c, S0);
        c.g[V1][0] = c.lw(out);
        c.g[A1][0] = c.g[SP][0] | 4;
        c.g[A3][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.g[A2][0] = 4;
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(LVW_WRITE_SLOT));
        c.g[V0][0] = c.lw(vt.wrapping_add(LVW_WRITE_SLOT + 4));
        c.g[A0][0] = sx(out.wrapping_add(r32(c, A0)));
        let method = r32(c, V0);
        c.sw(sp(c).wrapping_add(4), c.g[A3][0]);
        call_ptr(c, method, 0x0022_4c64)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LunoVM_Execute (0x2227d0): the Luno bytecode interpreter.
//
// A `cLunoVar` is 12 bytes: `{word0, word1, type}`. Types: 0 nil, 1 int,
// 2 float, 3 id (unsigned), 4 counted table reference (word0 = table, its
// refcount at +8), 5 an 8-byte pair (a function `{code, index}`). The VM's
// register file is `*vm` (`m_Regs`, allocated with a 16-byte count header).
// An instruction word is `op | A << 8 | B << 16 | C << 24`; jumps keep
// their target in the top 16 bits.
//
// Frame (0x140): the temporary `cLunoVar` at sp+0 (operator results are
// built there, then copied and released), the for-loop sum at sp+0x20,
// spills at sp+0x30..0x6c, and the locals: sp+0x70 the code, +0x74 the ops,
// +0x78 the op count, +0x7c the argument array (`cLunoArg`, 16 bytes:
// `{index, cLunoVar}`), +0x80 the argument depth.

/// `gLunoNil` (a nil `cLunoVar`) and the high half the code builds it from.
pub(crate) const LVX_NIL_HI: u64 = 0x004d_0000;

pub(crate) const LVX_NIL: u32 = 0x004c_9098;

/// The opcode jump table (`D_004797B0`, 43 entries).
pub(crate) const LVX_JUMP_TABLE: u32 = 0x0047_97b0;

/// `gLunoCFunctions` (C functions called by op 33).
pub(crate) const LVX_C_FUNCTIONS: u32 = 0x0044_1f38;

pub(crate) const LVX_OPERATOR_NEW: u32 = 0x0031_7e30;

pub(crate) const LVX_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;

pub(crate) const LVX_OPERATOR_DELETE: u32 = 0x0031_7e98;

pub(crate) const LVX_TABLE_DTOR: Callee = Callee {
    addr: 0x0022_4df0,
    f: ee::s05::cLunoTable_dtor,
};

pub(crate) const LVX_TABLE_ASSIGN: Callee = Callee {
    addr: 0x0022_5068,
    f: ee::s05::cLunoTable_Assign,
};

pub(crate) const LVX_TABLE_FIND: Callee = Callee {
    addr: 0x0022_5248,
    f: ee::s05::cLunoTable_Find,
};

pub(crate) const LVX_TABLE_SET: Callee = Callee {
    addr: 0x0022_5338,
    f: ee::s05::cLunoTable_Set,
};

pub(crate) const LVX_TABLE_CTOR: Callee = Callee {
    addr: 0x0022_4da0,
    f: ee::s05::cLunoTable_ctor,
};

pub(crate) const LVX_REG_DTOR: Callee = Callee {
    addr: 0x0022_5b90,
    f: ee::s05::LunoVMRegister_dtor,
};

pub(crate) const LVX_REG_SET_RAW: Callee = Callee {
    addr: 0x0022_6600,
    f: ee::s05::LunoVMRegister_SetRaw,
};

pub(crate) const LVX_REG_AS_INT: Callee = Callee {
    addr: 0x0022_6610,
    f: ee::s05::LunoVMRegister_AsInt,
};

pub(crate) const LVX_REG_AS_FLOAT: Callee = Callee {
    addr: 0x0022_6618,
    f: ee::s05::LunoVMRegister_AsFloat,
};

pub(crate) const LVX_REG_AS_ID: Callee = Callee {
    addr: 0x0022_6620,
    f: ee::s05::LunoVMRegister_AsId,
};

pub(crate) const LVX_REG_ADD: Callee = Callee {
    addr: 0x0022_6628,
    f: ee::s05::LunoVMRegister_Add,
};

pub(crate) const LVX_RETURN_ZERO: Callee = Callee {
    addr: 0x0022_24a0,
    f: ee::s05::LunoVM_ReturnZero,
};

pub(crate) const LVX_PASS_THROUGH: Callee = Callee {
    addr: 0x0022_24a8,
    f: ee::s05::LunoVM_PassThrough,
};

pub(crate) const LVX_CALL_WITH_ARG: Callee = Callee {
    addr: 0x0022_2648,
    f: ee::s05::LunoVM_CallWithArg,
};

/// Where an opcode handler continues: the loop test with `v1` = op count
/// still to load (0x224b2c), already loaded (0x224b30), or a return
/// (0x224b24: store the result type, jump past the end).
#[derive(Clone, Copy)]
pub(crate) enum LvxNext {
    Loop,
    Test,
    Ret,
}

pub(crate) type LvxStep = Result<LvxNext, Unwound>;

#[inline(always)]
pub(crate) fn lvx_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

#[inline(always)]
pub(crate) fn lvx_srl(c: &Cpu, r: usize, n: u32) -> u64 {
    sx((c.g[r][0] as u32) >> n)
}

#[inline(always)]
pub(crate) fn lvx_add(c: &Cpu, a: usize, b: usize) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32))
}

#[inline(always)]
pub(crate) fn lvx_addi(c: &Cpu, a: usize, k: u32) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(k))
}

/// `ldl rt, a+7; ldr rt, a`: an unaligned 8-byte load.
#[inline(always)]
pub(crate) fn lvx_ld8(c: &mut Cpu, rt: usize, a: u32) {
    c.ldl(rt, a.wrapping_add(7));
    c.ldr(rt, a);
}

/// `sdl rt, a+7; sdr rt, a`: an unaligned 8-byte store.
#[inline(always)]
pub(crate) fn lvx_sd8(c: &mut Cpu, rt: usize, a: u32) {
    c.sdl(rt, a.wrapping_add(7));
    c.sdr(rt, a);
}

/// `void cLunoVM::execute(cLunoFunc func, cLunoVar& result)` (0x2227d0):
/// `a0` = the VM (`*vm` = register file), `a1` = the function
/// (`{code, index}`), `a2` = the result. Allocates the function's argument
/// array (`NumArgs` of its record), runs its ops from `Start` to the end of
/// the code (or a return), then destroys the arguments and the register file.
pub fn luno_vm_execute(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S1, 0x110);
        frame.save(c, S5, 0xd0);
        frame.save(c, S7, 0xb0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S0, 0x120);
        frame.save(c, S2, 0x100);
        frame.save(c, S3, 0xf0);
        frame.save(c, S4, 0xe0);
        frame.save(c, S6, 0xc0);
        frame.save(c, FP, 0xa0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 20, 0x130);
        // code = func.Code; ops = code + 0x10; numOps = (FuncTable - 0x10) / 4;
        // the function record = code + FuncTable + index * 16.
        c.g[V0][0] = c.lw(lvx_at(c, A1, 0));
        c.sw(lvx_at(c, SP, 0x70), c.g[V0][0]);
        c.g[A0][0] = lvx_addi(c, V0, 0x10);
        c.g[V1][0] = c.lw(lvx_at(c, A1, 4));
        c.sw(lvx_at(c, SP, 0x7c), 0);
        c.sw(lvx_at(c, SP, 0x74), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, V1) << 4);
        c.g[A1][0] = c.lw(lvx_at(c, SP, 0x70));
        c.g[V0][0] = c.lw(lvx_at(c, V0, 4));
        c.sw(lvx_at(c, SP, 0x80), 0);
        c.g[S1][0] = lvx_add(c, A1, V0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x10));
        c.g[S1][0] = lvx_add(c, S1, V1);
        c.g[V0][0] = sx(r32(c, V0) >> 2);
        c.sw(lvx_at(c, SP, 0x78), c.g[V0][0]);
        c.g[S0][0] = c.lw(lvx_at(c, S1, 0xc));
        let no_args = (c.g[S0][0] as i64) <= 0;
        c.g[S7][0] = c.g[A2][0];
        if !no_args {
            // args = new cLunoArg[NumArgs] (count header, every value cleared).
            c.cycles += 12;
            c.g[A0][0] = sx(r32(c, S0) << 4);
            c.g[A1][0] = 0x48_0000;
            c.g[A2][0] = 0xffff_ffff_a000_0000;
            c.g[A0][0] = lvx_addi(c, A0, 0x10);
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x6870));
            c.g[A3][0] = 0;
            call_ptr(c, LVX_OPERATOR_NEW, 0x0022_2868)?;
            c.g[A2][0] = lvx_addi(c, V0, 0x10);
            c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
            c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
            let none = c.g[S0][0] == 0;
            c.g[V0][0] = c.g[A2][0];
            if !none {
                c.cycles += 1;
                c.g[A0][0] = u64::MAX;
                loop {
                    c.cycles += 7;
                    c.sw(lvx_at(c, V0, 0xc), 0);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.sw(lvx_at(c, V0, 4), 0);
                    let more = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = lvx_addi(c, V0, 0x10);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_2880);
                    }
                }
            }
            c.cycles += 1;
            c.sw(lvx_at(c, SP, 0x7c), c.g[A2][0]);
        }

        // pc = Start; while (pc < numOps) { op = ops[pc++]; switch ... }
        c.cycles += 5;
        c.g[S6][0] = c.lw(lvx_at(c, S1, 4));
        c.g[A2][0] = c.lw(lvx_at(c, SP, 0x78));
        c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.lw(lvx_at(c, SP, 0x7c));
        } else {
            c.cycles += 1;
            c.g[A3][0] = c.lw(lvx_at(c, SP, 0x74));
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, S6) << 2);
                c.g[V0][0] = lvx_add(c, V0, A3);
                c.g[S2][0] = c.lw(lvx_at(c, V0, 0));
                c.g[V1][0] = c.g[S2][0] & 0xff;
                c.g[V0][0] = (c.g[V1][0] < 0x2b) as u64;
                let known = c.g[V0][0] != 0;
                c.g[S6][0] = lvx_addi(c, S6, 1);
                let next = if known {
                    // jr through the opcode table.
                    c.cycles += 7;
                    let op = r32(c, V1);
                    c.g[V0][0] = 0x48_0000;
                    c.g[V1][0] = sx(op << 2);
                    c.g[V0][0] = sx(LVX_JUMP_TABLE);
                    c.g[V1][0] = lvx_add(c, V1, V0);
                    c.g[A0][0] = c.lw(lvx_at(c, V1, 0));
                    lvx_dispatch(c, op)?
                } else {
                    LvxNext::Loop
                };
                match next {
                    LvxNext::Ret => {
                        c.cycles += 3;
                        c.g[S6][0] = c.lw(lvx_at(c, SP, 0x78));
                        c.sw(lvx_at(c, S7, 8), c.g[V0][0]);
                        c.g[V1][0] = c.lw(lvx_at(c, SP, 0x78));
                    }
                    LvxNext::Loop => {
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(lvx_at(c, SP, 0x78));
                    }
                    LvxNext::Test => {}
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = c.lw(lvx_at(c, SP, 0x74));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_28b8);
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw(lvx_at(c, SP, 0x7c));
        }

        // delete[] args;
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw(lvx_at(c, A0, 0u32.wrapping_sub(0x10)));
            c.g[V0][0] = sx(r32(c, V0) << 4);
            c.g[S0][0] = lvx_add(c, A0, V0);
            let empty = c.g[A0][0] == c.g[S0][0];
            c.g[S1][0] = sx(r32(c, A0).wrapping_sub(0x10));
            if !empty {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x10));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 2;
                    c.g[A0][0] = lvx_addi(c, S0, 4);
                    LVX_REG_DTOR.call(c, 0x0022_4b6c)?;
                    c.g[A1][0] = c.lw(lvx_at(c, SP, 0x7c));
                    let more = c.g[A1][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x10));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4b60);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, LVX_OPERATOR_DELETE, 0x0022_4b80)?;
            c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
        }

        // delete[] m_Regs;
        c.cycles += 2;
        let no_regs = c.g[A1][0] == 0;
        c.g[V0][0] = 0xc;
        if !no_regs {
            c.cycles += 5;
            c.g[V1][0] = c.lw(lvx_at(c, A1, 0u32.wrapping_sub(0x10)));
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = lvx_add(c, A0, A1);
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    LVX_REG_DTOR.call(c, 0x0022_4bb4)?;
                    c.g[V0][0] = c.lw(lvx_at(c, S5, 0));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4ba8);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, LVX_OPERATOR_DELETE, 0x0022_4bcc)?;
        }

        c.cycles += 13;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore(c, S3, 0xf0);
        frame.restore(c, S4, 0xe0);
        frame.restore(c, S5, 0xd0);
        frame.restore(c, S6, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, FP, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

/// One opcode (`s2` = the instruction word, `s6` = the next pc).
pub(crate) fn lvx_dispatch(c: &mut Cpu, op: u32) -> LvxStep {
    match op {
        0 => {
            // pc = target
            c.cycles += 2;
            c.g[S6][0] = lvx_srl(c, S2, 16);
            Ok(LvxNext::Loop)
        }
        1 => lvx_op_jump_if(c, true),
        2 => lvx_op_jump_if(c, false),
        3 => lvx_op_eq(c, false),
        4 => lvx_op_eq(c, true),
        5 => lvx_op_ge_le(c, false),
        6 => lvx_op_gt_lt(c, false),
        7 => lvx_op_ge_le(c, true),
        8 => lvx_op_gt_lt(c, true),
        9 => lvx_op_or(c),
        10 => lvx_op_and(c),
        11 => lvx_op_move(c, true),
        12 => lvx_op_arith(c, LvxArith::Add),
        13 => lvx_op_arith(c, LvxArith::Sub),
        14 => lvx_op_arith(c, LvxArith::Mul),
        15 => lvx_op_div(c),
        16 => lvx_op_pow(c),
        17 => lvx_op_mod(c),
        18 => lvx_op_table_set(c),
        19 => lvx_op_move(c, false),
        20 => lvx_op_load_id(c),
        21 => lvx_op_load_int21(c),
        22 => lvx_op_load_int22(c),
        23 => lvx_op_load_float(c),
        24 => lvx_op_table_get(c),
        25 => lvx_op_new_table(c),
        27 => lvx_op_call(c),
        28 => lvx_op_table_set_next(c),
        29 => lvx_op_func(c),
        30 => lvx_op_nil(c),
        31 => lvx_op_return(c),
        32 => lvx_op_push_reg(c),
        33 => lvx_op_ccall(c),
        34 => lvx_op_not(c),
        35 => lvx_op_neg(c),
        36 => lvx_op_for(c),
        37 => lvx_op_push_word(c, 0x60, 0x0022_48c8, false),
        38 => lvx_op_push_float(c),
        39 => lvx_op_push_word(c, 0x68, 0x0022_49a8, true),
        40 => lvx_op_push_b(c),
        41 => lvx_op_push_b_float(c),
        42 => lvx_op_return_nil(c),
        // 26 (and anything else): nothing.
        _ => Ok(LvxNext::Loop),
    }
}

// ---------------------------------------------------------------------------
// Shared tails.

/// 0x224208: `m_Regs[A] = {v0, type v1}` (`s0` = the register).
pub(crate) fn lvx_store_word(c: &mut Cpu) -> LvxStep {
    c.cycles += 3;
    c.sw(lvx_at(c, S0, 8), c.g[V1][0]);
    c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// 0x224650: store the type word `v0` in `s0`, release the temporary at `a0`
/// (`LunoVMRegister_dtor(a0, 2)`).
pub(crate) fn lvx_release_temp(c: &mut Cpu) -> LvxStep {
    c.cycles += 4;
    c.sw(lvx_at(c, S0, 8), c.g[V0][0]);
    LVX_REG_DTOR.call(c, 0x0022_4658)?;
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x78));
    Ok(LvxNext::Test)
}

/// `m_Regs[idx] = temp` (the temporary at sp; `v1` = 12 on entry), then
/// release the temporary. The copy: clear the destination (releasing a table
/// reference), then a table reference is assigned (counted), a pair copied
/// as 8 bytes (through register `pair_reg`), anything else as one word.
pub(crate) fn lvx_assign_temp(
    c: &mut Cpu,
    idx: usize,
    pair_reg: usize,
    dtor_ret: u32,
    assign_ret: u32,
) -> LvxStep {
    c.cycles += 7;
    c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, idx, V1);
    c.g[S1][0] = 4;
    c.g[S0][0] = lvx_add(c, V1, A0);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[S1][0];
    c.g[S2][0] = c.g[SP][0];
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, dtor_ret)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S0, 8), 0);
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, SP, 8));
    let not_table = c.g[V1][0] != c.g[S1][0];
    c.g[V0][0] = 5;
    if !not_table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        LVX_TABLE_ASSIGN.call(c, assign_ret)?;
        c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    } else {
        c.cycles += 2;
        let not_pair = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(lvx_at(c, SP, 0));
        if not_pair {
            c.cycles += 1;
            c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
        } else {
            c.cycles += 5;
            let src = lvx_at(c, SP, 0);
            lvx_ld8(c, pair_reg, src);
            let dst = lvx_at(c, S0, 0);
            lvx_sd8(c, pair_reg, dst);
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 2;
    lvx_release_temp(c)
}

/// `m_Regs[idx] = (bool)v0` (`v1` = 12 on entry): spill the result, clear
/// the register (releasing a table reference), store it as an int.
pub(crate) fn lvx_store_bool(c: &mut Cpu, idx: usize, spill: u32, dtor_ret: u32) -> LvxStep {
    c.cycles += 8;
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, idx, V1);
    c.sw(lvx_at(c, SP, spill), c.g[V0][0]);
    c.g[A0][0] = 4;
    c.g[S0][0] = lvx_add(c, V1, A1);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    if c.g[V0][0] != c.g[A0][0] {
        c.sw(lvx_at(c, S0, 8), 0);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, dtor_ret)?;
        c.sw(lvx_at(c, S0, 8), 0);
    }
    c.cycles += 4;
    c.g[V1][0] = 1;
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V0][0] = c.lw(lvx_at(c, SP, spill));
    lvx_store_word(c)
}

/// The nil value into the temporary (`v0` = `0x4d0000`), through register
/// `pair_reg`; `word_reg` carries the type word.
pub(crate) fn lvx_temp_nil(c: &mut Cpu, base_reg: usize, pair_reg: usize, word_reg: usize) {
    c.g[base_reg][0] = sx(r32(c, V0).wrapping_sub(0x6f68));
    let src = lvx_at(c, base_reg, 0);
    lvx_ld8(c, pair_reg, src);
    c.g[word_reg][0] = c.lw(lvx_at(c, base_reg, 8));
    let dst = lvx_at(c, SP, 0);
    lvx_sd8(c, pair_reg, dst);
    c.sw(lvx_at(c, SP, 8), c.g[word_reg][0]);
}

// ---------------------------------------------------------------------------
// Control flow.

/// Ops 1/2: `if (m_Regs[A].isTrue()/isFalse()) pc = target;` (nil false,
/// int/id non-zero, float != 0, anything else true).
pub(crate) fn lvx_op_jump_if(c: &mut Cpu, when_true: bool) -> LvxStep {
    c.cycles += 9;
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[A0][0] = 0xc;
    c.g[V0][0] &= 0xff;
    c.g[V1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V0, V0, A0);
    c.g[A0][0] = lvx_add(c, V0, V1);
    c.g[V1][0] = c.lw(lvx_at(c, A0, 8));
    let nil = c.g[V1][0] == 0;
    c.g[V0][0] = if when_true { 0 } else { 1 };
    if !nil {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let word_test = if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw(lvx_at(c, A0, 0));
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let not_float = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 3;
            if !not_float {
                c.cycles += 6;
                let a = lvx_at(c, A0, 0);
                c.lwc1(1, a);
                c.mtc1(0, 0);
                c.c_eq_s(1, 0);
                let zero = c.fcc();
                c.g[V0][0] = 1;
                // true: jump when non-zero; false: when zero.
                if zero == when_true {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
                false
            } else {
                c.cycles += 2;
                let other = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = if when_true { 1 } else { 0 };
                if other {
                    false
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(lvx_at(c, A0, 0));
                    true
                }
            }
        };
        if word_test {
            if when_true {
                c.cycles += 2;
                c.g[V0][0] = (0 < c.g[V0][0]) as u64;
            } else {
                c.cycles += 1;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
    }
    c.cycles += 2;
    let stay = c.g[V0][0] == 0;
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x78));
    if stay {
        return Ok(LvxNext::Test);
    }
    c.cycles += 2;
    c.g[S6][0] = lvx_srl(c, S2, 16);
    Ok(LvxNext::Loop)
}

/// Op 27: `if (m_Regs[B].Type != nil) cLunoVM().call(m_Regs[B].castFunc(),
/// m_Regs[0], m_Regs[C], m_Regs[A]); else func_002224A0();`
pub(crate) fn lvx_op_call(c: &mut Cpu) -> LvxStep {
    c.cycles += 9;
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[S4][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, S3, V1, S4);
    c.g[V0][0] = lvx_add(c, S3, A1);
    c.g[V1][0] = c.lw(lvx_at(c, V0, 8));
    let nil = c.g[V1][0] == 0;
    c.g[S0][0] = lvx_srl(c, S2, 24);
    if nil {
        c.cycles += 4;
        LVX_RETURN_ZERO.call(c, 0x0022_3f50)?;
    } else {
        c.cycles += 17;
        c.g[S1][0] = lvx_addi(c, SP, 0x10);
        c.mult(0, V0, S0, S4);
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = lvx_add(c, V0, A1);
        LVX_PASS_THROUGH.call(c, 0x0022_3f18)?;
        c.g[T0][0] = lvx_srl(c, S2, 8);
        c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
        c.g[T0][0] &= 0xff;
        c.g[A0][0] = c.g[S1][0];
        c.mult(0, V0, T0, S4);
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = lvx_add(c, S3, A1);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = lvx_add(c, V0, A2);
        LVX_CALL_WITH_ARG.call(c, 0x0022_3f40)?;
    }
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x78));
    Ok(LvxNext::Test)
}

/// Op 31: `result = m_Regs[A]; pc = numOps;`
pub(crate) fn lvx_op_return(c: &mut Cpu) -> LvxStep {
    c.cycles += 9;
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[V1][0] = 0xc;
    c.g[V0][0] &= 0xff;
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V0, V0, V1);
    c.g[A0][0] = c.lw(lvx_at(c, S7, 8));
    c.g[S1][0] = 4;
    let not_table = c.g[A0][0] != c.g[S1][0];
    c.g[S0][0] = lvx_add(c, V0, A1);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4828)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S7, 8), 0);
    c.sw(lvx_at(c, S7, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V1][0] != c.g[S1][0];
    c.g[V0][0] = 5;
    if !not_table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = c.g[S0][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_4848)?;
        c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    } else {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(lvx_at(c, S0, 0));
            c.cycles += 1;
            c.sw(lvx_at(c, S7, 0), c.g[V0][0]);
        } else {
            c.cycles += 5;
            let src = lvx_at(c, S0, 0);
            lvx_ld8(c, A1, src);
            let dst = lvx_at(c, S7, 0);
            lvx_sd8(c, A1, dst);
        }
        c.cycles += 2;
        c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    }
    Ok(LvxNext::Ret)
}

/// Op 42: `result = gLunoNil; pc = numOps;`
pub(crate) fn lvx_op_return_nil(c: &mut Cpu) -> LvxStep {
    c.cycles += 5;
    c.g[S0][0] = LVX_NIL_HI;
    c.g[V0][0] = c.lw(lvx_at(c, S7, 8));
    c.g[S1][0] = 4;
    let not_table = c.g[V0][0] != c.g[S1][0];
    c.g[S2][0] = sx(LVX_NIL);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4ad0)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S7, 8), 0);
    c.sw(lvx_at(c, S7, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S2, 8));
    let not_table = c.g[V1][0] != c.g[S1][0];
    c.g[V0][0] = 5;
    if !not_table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = c.g[S2][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_4af0)?;
        c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    } else {
        c.cycles += 2;
        let not_pair = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(lvx_at(c, S0, 0u32.wrapping_sub(0x6f68)));
        if not_pair {
            c.cycles += 2;
            c.sw(lvx_at(c, S7, 0), c.g[V0][0]);
            c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
        } else {
            c.cycles += 7;
            c.g[A3][0] = sx(r32(c, S0).wrapping_sub(0x6f68));
            let src = lvx_at(c, A3, 0);
            lvx_ld8(c, A0, src);
            let dst = lvx_at(c, S7, 0);
            lvx_sd8(c, A0, dst);
            c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
        }
    }
    Ok(LvxNext::Ret)
}

// ---------------------------------------------------------------------------
// Comparisons and logic.

/// The operands of ops 3..8: `t1` = &m_Regs[B], `a1` = &m_Regs[C],
/// `v1` = B's type, `a2` = C's type, `a0` = A.
pub(crate) fn lvx_cmp_operands(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[A1][0] = 0xc;
    c.g[A0][0] = lvx_srl(c, S2, 24);
    c.g[V1][0] &= 0xff;
    c.mult(0, V1, V1, A1);
    c.mult(0, V0, A0, A1);
    c.g[A2][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A1][0] = lvx_add(c, V0, A2);
    c.g[T1][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[A2][0] = c.lw(lvx_at(c, A1, 8));
    c.g[V1][0] = c.lw(lvx_at(c, T1, 8));
    c.g[A0][0] = c.g[V0][0] & 0xff;
}

/// The type checks shared by the comparisons, after "types differ" and the
/// first `c3`: B float (v1 == 2) and C int goes to the float compare
/// (`false`), anything else to the int stages with `a2` = B's type and
/// `v0` = 1 (`true`).
pub(crate) fn lvx_cmp_to_int_stage(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[A2][0] = c.lw(lvx_at(c, T1, 8));
        c.cycles += 1;
        c.g[V0][0] = 1;
        return true;
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    if c.g[A2][0] != c.g[V0][0] {
        c.g[A2][0] = c.lw(lvx_at(c, T1, 8));
        return true;
    }
    false
}

/// Load B (float) and C (int, converted) for the float compare.
pub(crate) fn lvx_cmp_float_operands(c: &mut Cpu) {
    c.cycles += 7;
    let a = lvx_at(c, A1, 0);
    c.lwc1(1, a);
    c.cvt_s_w(1, 1);
    let b = lvx_at(c, T1, 0);
    c.lwc1(0, b);
}

/// Ops 3/4: `m_Regs[A] = m_Regs[B] == / != m_Regs[C]`.
pub(crate) fn lvx_op_eq(c: &mut Cpu, ne: bool) -> LvxStep {
    let (spill, dtor_ret) = if ne {
        (0x3c, 0x0022_2fe0)
    } else {
        (0x38, 0x0022_2ed0)
    };
    // 0: raw word compare (load both); 1: xor (operands loaded); 2: done.
    let mut stage;
    lvx_cmp_operands(c);
    if c.g[A2][0] == c.g[V1][0] {
        stage = 0;
    } else {
        if !lvx_cmp_to_int_stage(c) {
            lvx_cmp_float_operands(c);
            c.c_eq_s(0, 1);
            let eq = c.fcc();
            c.g[V1][0] = 0xc;
            if eq == ne {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            return lvx_store_bool(c, A0, spill, dtor_ret);
        }
        c.cycles += 2;
        let not_int = c.g[A2][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        stage = 3;
        if !not_int {
            c.cycles += 4;
            c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
            c.g[V0][0] = 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 5;
                let a = lvx_at(c, A1, 0);
                c.lwc1(0, a);
                c.cvt_w_s(1, 0);
                c.mfc1(V1, 1);
                c.g[V0][0] = c.lw(lvx_at(c, T1, 0));
                stage = 1;
            } else {
                c.g[V0][0] = 3;
            }
        }
        if stage == 3 {
            c.cycles += 2;
            let not_id = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            stage = 2;
            if !not_id {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                c.g[V0][0] = 1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0;
                } else {
                    stage = 0;
                }
            }
        }
    }
    if stage == 0 {
        c.cycles += 2;
        c.g[V0][0] = c.lw(lvx_at(c, T1, 0));
        c.g[V1][0] = c.lw(lvx_at(c, A1, 0));
        stage = 1;
    }
    if stage == 1 {
        c.cycles += 2;
        c.g[V0][0] ^= c.g[V1][0];
        c.g[V0][0] = if ne {
            (0 < c.g[V0][0]) as u64
        } else {
            (c.g[V0][0] < 1) as u64
        };
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_store_bool(c, A0, spill, dtor_ret)
}

/// Ops 5/7: `m_Regs[A] = m_Regs[B] >= / <= m_Regs[C]` (as `!(x < y)`, B in
/// register `x`, C in `y`).
pub(crate) fn lvx_op_ge_le(c: &mut Cpu, le: bool) -> LvxStep {
    let (x, y) = if le { (V1, V0) } else { (V0, V1) };
    let (spill, dtor_ret) = if le {
        (0x48, 0x0022_3348)
    } else {
        (0x40, 0x0022_3104)
    };
    // 0: signed compare; 1: done.
    let mut signed;
    lvx_cmp_operands(c);
    if c.g[A2][0] == c.g[V1][0] {
        c.cycles += 3;
        c.g[x][0] = c.lw(lvx_at(c, T1, 0));
        c.g[y][0] = c.lw(lvx_at(c, A1, 0));
        signed = true;
    } else {
        if !lvx_cmp_to_int_stage(c) {
            lvx_cmp_float_operands(c);
            if le {
                c.c_le_s(0, 1);
            } else {
                c.c_le_s(1, 0);
            }
            let holds = c.fcc();
            c.g[V1][0] = 0xc;
            if !holds {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            return lvx_store_bool(c, A0, spill, dtor_ret);
        }
        c.cycles += 2;
        let not_int = c.g[A2][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        signed = false;
        let mut mixed = false;
        if !not_int {
            c.cycles += 4;
            c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
            c.g[V0][0] = 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                let a = lvx_at(c, A1, 0);
                c.lwc1(0, a);
                c.g[x][0] = c.lw(lvx_at(c, T1, 0));
                c.cvt_w_s(1, 0);
                c.mfc1(y, 1);
                mixed = true;
                signed = true;
            } else {
                c.g[V0][0] = 3;
            }
        }
        if !mixed {
            c.cycles += 2;
            let not_id = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if !not_id {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                c.g[V0][0] = 1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[x][0] = c.lw(lvx_at(c, T1, 0));
                    c.g[y][0] = c.lw(lvx_at(c, A1, 0));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] ^= 1;
                }
            }
        }
    }
    if signed {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[V0][0] ^= 1;
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_store_bool(c, A0, spill, dtor_ret)
}

/// Ops 6/8: `m_Regs[A] = m_Regs[B] > / < m_Regs[C]` (B in `v1`, C in `v0`).
pub(crate) fn lvx_op_gt_lt(c: &mut Cpu, lt: bool) -> LvxStep {
    let (spill, dtor_ret) = if lt {
        (0x4c, 0x0022_3468)
    } else {
        (0x44, 0x0022_3224)
    };
    let slt = |c: &Cpu| -> u64 {
        let (b, cc) = (c.g[V1][0] as i64, c.g[V0][0] as i64);
        (if lt { b < cc } else { cc < b }) as u64
    };
    lvx_cmp_operands(c);
    if c.g[A2][0] == c.g[V1][0] {
        c.cycles += 4;
        c.g[V1][0] = c.lw(lvx_at(c, T1, 0));
        c.g[V0][0] = c.lw(lvx_at(c, A1, 0));
        c.g[V0][0] = slt(c);
    } else {
        if !lvx_cmp_to_int_stage(c) {
            lvx_cmp_float_operands(c);
            if lt {
                c.c_lt_s(0, 1);
            } else {
                c.c_lt_s(1, 0);
            }
            let holds = c.fcc();
            c.g[V1][0] = 0xc;
            if !holds {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            return lvx_store_bool(c, A0, spill, dtor_ret);
        }
        c.cycles += 2;
        let not_int = c.g[A2][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        let mut mixed = false;
        if !not_int {
            c.cycles += 4;
            c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
            c.g[V0][0] = 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 6;
                let a = lvx_at(c, A1, 0);
                c.lwc1(0, a);
                c.g[V1][0] = c.lw(lvx_at(c, T1, 0));
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] = slt(c);
                mixed = true;
            } else {
                c.g[V0][0] = 3;
            }
        }
        if !mixed {
            c.cycles += 2;
            let not_id = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if !not_id {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                c.g[V0][0] = 1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(lvx_at(c, T1, 0));
                    c.g[V0][0] = c.lw(lvx_at(c, A1, 0));
                    c.g[V0][0] = if lt {
                        (c.g[V1][0] < c.g[V0][0]) as u64
                    } else {
                        (c.g[V0][0] < c.g[V1][0]) as u64
                    };
                }
            }
        }
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_store_bool(c, A0, spill, dtor_ret)
}

/// The operands of ops 9/10: `t1` = &m_Regs[C], `v1` = &m_Regs[B],
/// `a0` = C's type, `a2` = A.
pub(crate) fn lvx_logic_operands(c: &mut Cpu) {
    c.g[A0][0] = 0xc;
    c.g[V0][0] = lvx_srl(c, S2, 24);
    c.mult(0, A2, V0, A0);
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[V1][0] &= 0xff;
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, A0);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[T1][0] = lvx_add(c, A2, A1);
    c.g[A0][0] = c.lw(lvx_at(c, T1, 8));
    c.g[A2][0] = c.g[V0][0] & 0xff;
    c.g[V1][0] = lvx_add(c, V1, A1);
}

/// Op 9: `m_Regs[A] = m_Regs[B] || m_Regs[C]`.
pub(crate) fn lvx_op_or(c: &mut Cpu) -> LvxStep {
    c.cycles += 13;
    lvx_logic_operands(c);
    let c_nil = c.g[A0][0] == 0;
    if c_nil {
        c.cycles += 3;
        c.g[V0][0] = c.lw(lvx_at(c, V1, 8));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(lvx_at(c, V1, 0));
            c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(lvx_at(c, V1, 8));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(lvx_at(c, V1, 0));
            c.cycles += 2;
            let non_zero = c.g[V0][0] != 0;
            c.g[V1][0] = 0;
            let mut one = non_zero;
            if !non_zero {
                c.cycles += 3;
                c.g[V0][0] = c.lw(lvx_at(c, T1, 0));
                let zero = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0];
                one = !zero;
            }
            if one {
                c.cycles += 2;
                c.g[V1][0] = 1;
                c.g[V0][0] = c.g[V1][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_store_bool(c, A2, 0x50, 0x0022_3528)
}

/// Op 10: `m_Regs[A] = m_Regs[B] && m_Regs[C]`.
pub(crate) fn lvx_op_and(c: &mut Cpu) -> LvxStep {
    c.cycles += 14;
    lvx_logic_operands(c);
    let c_nil = c.g[A0][0] == 0;
    c.g[V0][0] = 0;
    if !c_nil {
        c.cycles += 3;
        c.g[V0][0] = c.lw(lvx_at(c, V1, 8));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(lvx_at(c, V1, 0));
            c.cycles += 2;
            let zero = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !zero {
                c.cycles += 2;
                c.g[V0][0] = c.lw(lvx_at(c, T1, 0));
                c.g[V1][0] = (0 < c.g[V0][0]) as u64;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_store_bool(c, A2, 0x54, 0x0022_35cc)
}

/// Op 34: `m_Regs[A] = !m_Regs[B]` (int: `== 0`, float: `== 0.0`, else nil).
pub(crate) fn lvx_op_not(c: &mut Cpu) -> LvxStep {
    c.cycles += 10;
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[A0][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, A0);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[V1][0] = lvx_add(c, V1, A1);
    c.g[A0][0] = c.lw(lvx_at(c, V1, 8));
    let non_nil = c.g[A0][0] != 0;
    c.g[S1][0] = c.g[V0][0] & 0xff;
    if !non_nil {
        c.cycles += 10;
        c.g[V0][0] = LVX_NIL_HI;
        lvx_temp_nil(c, A2, V1, A0);
        c.g[V1][0] = 0xc;
    } else {
        c.cycles += 3;
        c.g[A1][0] = 1;
        let not_int = c.g[A0][0] != c.g[A1][0];
        c.g[V0][0] = 2;
        let as_int = if !not_int {
            c.cycles += 5;
            c.g[S0][0] = c.lw(lvx_at(c, V1, 0));
            c.g[A0][0] = c.g[SP][0];
            c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            true
        } else {
            c.cycles += 2;
            let not_float = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = LVX_NIL_HI;
            if !not_float {
                c.cycles += 6;
                let a = lvx_at(c, V1, 0);
                c.lwc1(1, a);
                c.mtc1(0, 0);
                c.c_eq_s(1, 0);
                let zero = c.fcc();
                c.g[S0][0] = 1;
                if !zero {
                    c.cycles += 1;
                    c.g[S0][0] = 0;
                }
                c.cycles += 2;
                c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
                c.g[A0][0] = c.g[SP][0];
                true
            } else {
                c.cycles += 7;
                lvx_temp_nil(c, A1, A3, V1);
                false
            }
        };
        if as_int {
            c.cycles += 4;
            LVX_REG_AS_INT.call(c, 0x0022_2bf0)?;
            c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
        }
        c.cycles += 1;
        c.g[V1][0] = 0xc;
    }
    lvx_assign_temp(c, S1, A2, 0x0022_2c40, 0x0022_2c60)
}

/// Op 35: `m_Regs[A] = -m_Regs[B]` (int or float, else nil).
pub(crate) fn lvx_op_neg(c: &mut Cpu) -> LvxStep {
    c.cycles += 10;
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[A0][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, A0);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[V1][0] = lvx_add(c, V1, A1);
    c.g[A1][0] = c.lw(lvx_at(c, V1, 8));
    let non_nil = c.g[A1][0] != 0;
    c.g[S1][0] = c.g[V0][0] & 0xff;
    if !non_nil {
        c.cycles += 10;
        c.g[V0][0] = LVX_NIL_HI;
        lvx_temp_nil(c, A3, A0, A1);
        c.g[V1][0] = 0xc;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let not_int = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = 2;
        if !not_int {
            c.cycles += 7;
            c.g[S0][0] = c.lw(lvx_at(c, V1, 0));
            c.g[A0][0] = c.g[SP][0];
            c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
            c.g[S0][0] = sx(0u32.wrapping_sub(r32(c, S0)));
            LVX_REG_AS_INT.call(c, 0x0022_2d08)?;
            c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
        } else {
            c.cycles += 2;
            let not_float = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = LVX_NIL_HI;
            if not_float {
                c.cycles += 7;
                lvx_temp_nil(c, A2, V1, A0);
            } else {
                c.cycles += 7;
                let a = lvx_at(c, V1, 0);
                c.lwc1(20, a);
                c.g[A0][0] = c.g[SP][0];
                c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
                c.neg_s(20, 20);
                LVX_REG_AS_FLOAT.call(c, 0x0022_2d2c)?;
                let d = lvx_at(c, V0, 0);
                c.swc1(20, d);
            }
        }
        c.cycles += 1;
        c.g[V1][0] = 0xc;
    }
    lvx_assign_temp(c, S1, A3, 0x0022_2d7c, 0x0022_2d9c)
}

// ---------------------------------------------------------------------------
// Moves and loads.

/// Ops 11/19: `m_Regs[A] = m_Regs[B]` (op 11's copy of a plain word takes two
/// cycles more).
pub(crate) fn lvx_op_move(c: &mut Cpu, op11: bool) -> LvxStep {
    c.cycles += 13;
    c.g[V1][0] = lvx_srl(c, S2, 8);
    c.g[A1][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[V0][0] = lvx_srl(c, S2, 16);
    c.mult(0, V1, V1, A1);
    c.g[V0][0] &= 0xff;
    c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, A2, V0, A1);
    c.g[S2][0] = 4;
    c.g[S1][0] = lvx_add(c, V1, A0);
    c.g[V0][0] = c.lw(lvx_at(c, S1, 8));
    let not_table = c.g[V0][0] != c.g[S2][0];
    c.g[S0][0] = lvx_add(c, A2, A0);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        let ret = if op11 { 0x0022_3620 } else { 0x0022_4100 };
        LVX_TABLE_DTOR.call(c, ret)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S1, 8), 0);
    c.sw(lvx_at(c, S1, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S0, 8));
    let table = c.g[V1][0] == c.g[S2][0];
    c.g[V0][0] = 5;
    if table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_4120)?;
        c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    } else {
        c.cycles += 2;
        let pair = c.g[V1][0] == c.g[V0][0];
        if pair {
            c.cycles += 5;
            let src = lvx_at(c, S0, 0);
            lvx_ld8(c, A3, src);
            let dst = lvx_at(c, S1, 0);
            lvx_sd8(c, A3, dst);
        } else {
            if op11 {
                c.cycles += 2;
            }
            c.g[V0][0] = c.lw(lvx_at(c, S0, 0));
            c.cycles += 1;
            c.sw(lvx_at(c, S1, 0), c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    }
    c.cycles += 2;
    c.sw(lvx_at(c, S1, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Ops 21/22/23: read the constant `ops[pc++]` into `v0` and spill it, `s0` =
/// &m_Regs[A], `v0` = its type; returns "not a table reference".
pub(crate) fn lvx_load_operand(c: &mut Cpu, ops_reg: usize, spill: u32) -> bool {
    c.cycles += 15;
    c.g[A0][0] = lvx_srl(c, S2, 8);
    c.g[ops_reg][0] = c.lw(lvx_at(c, SP, 0x74));
    c.g[V1][0] = sx(r32(c, S6) << 2);
    c.g[V0][0] = 0xc;
    c.g[A0][0] &= 0xff;
    c.g[V1][0] = lvx_add(c, V1, ops_reg);
    c.mult(0, A0, A0, V0);
    c.g[V0][0] = c.lw(lvx_at(c, V1, 0));
    c.g[A1][0] = 4;
    c.g[V1][0] = c.lw(lvx_at(c, S5, 0));
    c.sw(lvx_at(c, SP, spill), c.g[V0][0]);
    c.g[S0][0] = lvx_add(c, A0, V1);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[A1][0];
    c.g[S6][0] = lvx_addi(c, S6, 1);
    not_table
}

/// Op 23: `m_Regs[A] = (float)ops[pc++]`.
pub(crate) fn lvx_op_load_float(c: &mut Cpu) -> LvxStep {
    if !lvx_load_operand(c, A2, 0x30) {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_29e8)?;
    }
    c.cycles += 7;
    c.sw(lvx_at(c, S0, 8), 0);
    c.g[V0][0] = 2;
    c.sw(lvx_at(c, S0, 0), 0);
    let a = lvx_at(c, SP, 0x30);
    c.lwc1(0, a);
    c.sw(lvx_at(c, S0, 8), c.g[V0][0]);
    let d = lvx_at(c, S0, 0);
    c.swc1(0, d);
    Ok(LvxNext::Loop)
}

/// Op 22: `m_Regs[A] = (int)ops[pc++]`.
pub(crate) fn lvx_op_load_int22(c: &mut Cpu) -> LvxStep {
    if !lvx_load_operand(c, A3, 0x34) {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_2a4c)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S0, 8), 0);
    c.g[V1][0] = 1;
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V0][0] = c.lw(lvx_at(c, SP, 0x34));
    lvx_store_word(c)
}

/// Op 21: `m_Regs[A] = (int)ops[pc++]`.
pub(crate) fn lvx_op_load_int21(c: &mut Cpu) -> LvxStep {
    if !lvx_load_operand(c, A2, 0x5c) {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_41f8)?;
    }
    c.cycles += 4;
    c.sw(lvx_at(c, S0, 8), 0);
    c.g[V1][0] = 1;
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V0][0] = c.lw(lvx_at(c, SP, 0x5c));
    lvx_store_word(c)
}

/// Op 20: `m_Regs[A] = (unsigned)ops[pc++]` (type 3).
pub(crate) fn lvx_op_load_id(c: &mut Cpu) -> LvxStep {
    c.cycles += 15;
    c.g[A1][0] = c.lw(lvx_at(c, SP, 0x74));
    c.g[V1][0] = lvx_srl(c, S2, 8);
    c.g[A0][0] = 0xc;
    c.g[V0][0] = sx(r32(c, S6) << 2);
    c.g[V1][0] &= 0xff;
    c.g[V0][0] = lvx_add(c, V0, A1);
    c.mult(0, V1, V1, A0);
    c.g[A2][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A0][0] = c.lw(lvx_at(c, V0, 0));
    c.g[A1][0] = 4;
    c.sw(lvx_at(c, SP, 0x58), c.g[A0][0]);
    c.g[S0][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[A1][0];
    c.g[S6][0] = lvx_addi(c, S6, 1);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_419c)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S0, 8), 0);
    c.g[V1][0] = 3;
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V0][0] = c.lw(lvx_at(c, SP, 0x58));
    lvx_store_word(c)
}

/// Op 30: `m_Regs[A] = gLunoNil`.
pub(crate) fn lvx_op_nil(c: &mut Cpu) -> LvxStep {
    c.cycles += 11;
    c.g[V1][0] = lvx_srl(c, S2, 8);
    c.g[V0][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, V0);
    c.g[S1][0] = LVX_NIL_HI;
    c.g[S3][0] = 4;
    c.g[S0][0] = lvx_add(c, V1, A0);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[S3][0];
    c.g[S2][0] = sx(LVX_NIL);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_479c)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S0, 8), 0);
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S2, 8));
    let not_table = c.g[V1][0] != c.g[S3][0];
    c.g[V0][0] = 5;
    if !not_table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_47bc)?;
        c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    } else {
        c.cycles += 2;
        let not_pair = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(lvx_at(c, S1, 0u32.wrapping_sub(0x6f68)));
        if not_pair {
            c.cycles += 1;
            c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
            c.cycles += 1;
            c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
        } else {
            c.cycles += 7;
            c.g[A0][0] = sx(r32(c, S1).wrapping_sub(0x6f68));
            let src = lvx_at(c, A0, 0);
            lvx_ld8(c, A3, src);
            let dst = lvx_at(c, S0, 0);
            lvx_sd8(c, A3, dst);
            c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
        }
    }
    c.cycles += 2;
    c.sw(lvx_at(c, S0, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Op 29: `cLunoFunc f; f.init(code, ops[pc++]); m_Regs[A] = f;` (a pair).
pub(crate) fn lvx_op_func(c: &mut Cpu) -> LvxStep {
    c.cycles += 18;
    c.g[A3][0] = c.lw(lvx_at(c, SP, 0x74));
    c.g[V0][0] = sx(r32(c, S6) << 2);
    c.g[A1][0] = c.lw(lvx_at(c, SP, 0x70));
    c.g[A0][0] = c.g[SP][0];
    c.g[V0][0] = lvx_add(c, V0, A3);
    c.g[S6][0] = lvx_addi(c, S6, 1);
    c.g[A2][0] = c.lw(lvx_at(c, V0, 0));
    LVX_REG_SET_RAW.call(c, 0x0022_4234)?;
    c.g[V1][0] = lvx_srl(c, S2, 8);
    c.g[V0][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, V0);
    c.g[A1][0] = 4;
    c.g[S0][0] = lvx_add(c, V1, A0);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    if c.g[V0][0] != c.g[A1][0] {
        c.sw(lvx_at(c, S0, 8), 0);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4268)?;
        c.sw(lvx_at(c, S0, 8), 0);
    }
    c.cycles += 8;
    c.g[V0][0] = 5;
    c.sw(lvx_at(c, S0, 0), 0);
    let src = lvx_at(c, SP, 0);
    lvx_ld8(c, V1, src);
    let dst = lvx_at(c, S0, 0);
    lvx_sd8(c, V1, dst);
    c.sw(lvx_at(c, S0, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

// ---------------------------------------------------------------------------
// Arithmetic.

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum LvxArith {
    Add,
    Sub,
    Mul,
}

/// What an arithmetic op put in the temporary: a float or int result still
/// to store, nil, or (add of two ids) already stored.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum LvxResult {
    Float,
    Int,
    Nil,
    Stored,
}

/// Ops 12/13/14: `m_Regs[A] = lunoAdd/Sub/Mul(m_Regs[B], m_Regs[C])` (two
/// ints, two floats, float and int, int and float; add also adds two ids;
/// anything else nil). `s0` = &B, `a3` = &C, `a1` = B's type, `v1` = C's.
pub(crate) fn lvx_op_arith(c: &mut Cpu, k: LvxArith) -> LvxStep {
    let (float_ret, int_ret, dtor_ret, assign_ret) = match k {
        LvxArith::Add => (0x0022_3710, 0x0022_3754, 0x0022_37a4, 0x0022_37c4),
        LvxArith::Sub => (0x0022_38a0, 0x0022_38e4, 0x0022_3934, 0x0022_3954),
        LvxArith::Mul => (0x0022_3a30, 0x0022_3a74, 0x0022_3ac4, 0x0022_3ae4),
    };
    c.cycles += 14;
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[A1][0] = 0xc;
    c.g[V1][0] &= 0xff;
    c.g[A0][0] = lvx_srl(c, S2, 24);
    c.mult(0, V1, V1, A1);
    c.mult(0, V0, A0, A1);
    c.g[A2][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A3][0] = lvx_add(c, V0, A2);
    c.g[S0][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[V1][0] = c.lw(lvx_at(c, A3, 8));
    c.g[A1][0] = c.lw(lvx_at(c, S0, 8));
    let differ = c.g[V1][0] != c.g[A1][0];
    c.g[S1][0] = c.g[V0][0] & 0xff;
    let what = if !differ {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let not_float = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = 1;
        if !not_float {
            c.cycles += 5;
            let a = lvx_at(c, S0, 0);
            c.lwc1(20, a);
            c.g[A0][0] = c.g[SP][0];
            let b = lvx_at(c, A3, 0);
            c.lwc1(0, b);
            c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
            LvxResult::Float
        } else if k == LvxArith::Add {
            c.cycles += 2;
            let not_int = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 3;
            if !not_int {
                c.cycles += 4;
                c.g[S0][0] = c.lw(lvx_at(c, S0, 0));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = c.lw(lvx_at(c, A3, 0));
                LvxResult::Int
            } else {
                c.cycles += 2;
                let not_id = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = LVX_NIL_HI;
                if not_id {
                    LvxResult::Nil
                } else {
                    c.cycles += 8;
                    c.g[S0][0] = c.lw(lvx_at(c, S0, 0));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw(lvx_at(c, A3, 0));
                    c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
                    c.g[S0][0] = lvx_add(c, S0, V0);
                    LVX_REG_AS_ID.call(c, 0x0022_36d4)?;
                    c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
                    LvxResult::Stored
                }
            }
        } else {
            c.cycles += 2;
            let not_int = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = LVX_NIL_HI;
            if not_int {
                LvxResult::Nil
            } else {
                c.cycles += 4;
                c.g[S0][0] = c.lw(lvx_at(c, S0, 0));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = c.lw(lvx_at(c, A3, 0));
                LvxResult::Int
            }
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let to_int = if c.g[A1][0] != c.g[V0][0] {
            c.g[A1][0] = c.lw(lvx_at(c, S0, 8));
            c.cycles += 1;
            c.g[V0][0] = 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A1][0] = c.lw(lvx_at(c, S0, 8));
                true
            } else {
                false
            }
        };
        if !to_int {
            // float B, int C
            c.cycles += 5;
            let b = lvx_at(c, A3, 0);
            c.lwc1(0, b);
            c.cvt_s_w(0, 0);
            let a = lvx_at(c, S0, 0);
            c.lwc1(20, a);
            c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
            c.g[A0][0] = c.g[SP][0];
            LvxResult::Float
        } else {
            c.cycles += 2;
            let not_int = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = LVX_NIL_HI;
            if not_int {
                LvxResult::Nil
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A3, 8));
                c.g[V0][0] = 2;
                let not_float = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = LVX_NIL_HI;
                if not_float {
                    LvxResult::Nil
                } else {
                    // int B, float C (truncated)
                    c.cycles += 5;
                    let b = lvx_at(c, A3, 0);
                    c.lwc1(0, b);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(lvx_at(c, S0, 0));
                    c.cvt_w_s(1, 0);
                    c.mfc1(V0, 1);
                    LvxResult::Int
                }
            }
        }
    };
    match what {
        LvxResult::Float => {
            c.cycles += 4;
            match k {
                LvxArith::Add => c.add_s(20, 20, 0),
                LvxArith::Sub => c.sub_s(20, 20, 0),
                LvxArith::Mul => c.mul_s(20, 20, 0),
            }
            LVX_REG_AS_FLOAT.call(c, float_ret)?;
            let d = lvx_at(c, V0, 0);
            c.swc1(20, d);
        }
        LvxResult::Int => {
            c.cycles += 5;
            c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
            match k {
                LvxArith::Add => c.g[S0][0] = lvx_add(c, S0, V0),
                LvxArith::Sub => c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0))),
                LvxArith::Mul => c.mult(0, S0, S0, V0),
            }
            LVX_REG_AS_INT.call(c, int_ret)?;
            c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
        }
        LvxResult::Nil => {
            c.cycles += 7;
            lvx_temp_nil(c, A2, V1, A0);
        }
        LvxResult::Stored => {}
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_assign_temp(c, S1, A3, dtor_ret, assign_ret)
}

/// Op 15: `m_Regs[A] = lunoDiv(m_Regs[B], m_Regs[C])` (`s0` = &B, `a1` = &C,
/// `v1` = B's type, `a0` = C's).
pub(crate) fn lvx_op_div(c: &mut Cpu) -> LvxStep {
    c.cycles += 14;
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[A1][0] = 0xc;
    c.g[A0][0] = lvx_srl(c, S2, 24);
    c.g[V1][0] &= 0xff;
    c.mult(0, V1, V1, A1);
    c.mult(0, V0, A0, A1);
    c.g[A2][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A1][0] = lvx_add(c, V0, A2);
    c.g[S0][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[A0][0] = c.lw(lvx_at(c, A1, 8));
    c.g[V1][0] = c.lw(lvx_at(c, S0, 8));
    let differ = c.g[A0][0] != c.g[V1][0];
    c.g[S1][0] = c.g[V0][0] & 0xff;
    let what = if !differ {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let not_float = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 1;
        if !not_float {
            c.cycles += 5;
            let a = lvx_at(c, S0, 0);
            c.lwc1(20, a);
            c.g[A0][0] = c.g[SP][0];
            let b = lvx_at(c, A1, 0);
            c.lwc1(0, b);
            c.sw(lvx_at(c, SP, 8), c.g[V1][0]);
            LvxResult::Float
        } else {
            c.cycles += 2;
            let not_int = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = LVX_NIL_HI;
            if not_int {
                LvxResult::Nil
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(lvx_at(c, A1, 0));
                c.g[A0][0] = c.g[SP][0];
                c.g[S0][0] = c.lw(lvx_at(c, S0, 0));
                c.cycles += 3;
                c.div(0, S0, V0);
                c.sw(lvx_at(c, SP, 8), c.g[V1][0]);
                LvxResult::Int
            }
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let to_int = if c.g[V1][0] != c.g[V0][0] {
            c.g[A2][0] = c.lw(lvx_at(c, S0, 8));
            c.cycles += 1;
            c.g[V0][0] = 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A2][0] = c.lw(lvx_at(c, S0, 8));
                true
            } else {
                false
            }
        };
        if !to_int {
            c.cycles += 5;
            let b = lvx_at(c, A1, 0);
            c.lwc1(0, b);
            c.cvt_s_w(0, 0);
            let a = lvx_at(c, S0, 0);
            c.lwc1(20, a);
            c.sw(lvx_at(c, SP, 8), c.g[V1][0]);
            c.g[A0][0] = c.g[SP][0];
            LvxResult::Float
        } else {
            c.cycles += 2;
            let not_int = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = LVX_NIL_HI;
            if not_int {
                LvxResult::Nil
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                c.g[V0][0] = 2;
                let not_float = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = LVX_NIL_HI;
                if not_float {
                    LvxResult::Nil
                } else {
                    c.cycles += 8;
                    let b = lvx_at(c, A1, 0);
                    c.lwc1(0, b);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw(lvx_at(c, S0, 0));
                    c.cvt_w_s(1, 0);
                    c.mfc1(V0, 1);
                    c.div(0, S0, V0);
                    c.cycles += 1;
                    c.sw(lvx_at(c, SP, 8), c.g[A2][0]);
                    LvxResult::Int
                }
            }
        }
    };
    match what {
        LvxResult::Float => {
            c.cycles += 5;
            c.div_s(20, 20, 0);
            LVX_REG_AS_FLOAT.call(c, 0x0022_3bd4)?;
            let d = lvx_at(c, V0, 0);
            c.swc1(20, d);
        }
        LvxResult::Int => {
            c.cycles += 5;
            c.g[S0][0] = c.lo[0];
            LVX_REG_AS_INT.call(c, 0x0022_3c28)?;
            c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
        }
        LvxResult::Nil | LvxResult::Stored => {
            c.cycles += 7;
            lvx_temp_nil(c, A2, V1, A0);
        }
    }
    c.cycles += 1;
    c.g[V1][0] = 0xc;
    lvx_assign_temp(c, S1, A3, 0x0022_3c78, 0x0022_3c98)
}

/// Op 17: `m_Regs[A] = lunoMod(m_Regs[B], m_Regs[C])` (two ints, else nil).
pub(crate) fn lvx_op_mod(c: &mut Cpu) -> LvxStep {
    c.cycles += 15;
    c.g[A1][0] = 0xc;
    c.g[A0][0] = lvx_srl(c, S2, 24);
    c.mult(0, V0, A0, A1);
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[V1][0] &= 0xff;
    c.g[A2][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, A1);
    c.g[A0][0] = lvx_add(c, V0, A2);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[A1][0] = c.lw(lvx_at(c, A0, 8));
    c.g[S1][0] = c.g[V0][0] & 0xff;
    c.g[V1][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = c.lw(lvx_at(c, V1, 8));
    let differ = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = LVX_NIL_HI;
    if differ {
        c.cycles += 7;
        lvx_temp_nil(c, A1, A3, V1);
        c.cycles += 1;
        c.g[V1][0] = 0xc;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let not_int = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = LVX_NIL_HI;
        if not_int {
            c.cycles += 9;
            lvx_temp_nil(c, A2, V1, A0);
            c.g[V1][0] = 0xc;
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw(lvx_at(c, A0, 0));
            c.g[V1][0] = c.lw(lvx_at(c, V1, 0));
            c.g[A0][0] = c.g[SP][0];
            c.cycles += 6;
            c.div(0, V1, V0);
            c.g[S0][0] = c.hi[0];
            c.sw(lvx_at(c, SP, 8), c.g[A1][0]);
            LVX_REG_AS_INT.call(c, 0x0022_3d3c)?;
            c.sw(lvx_at(c, V0, 0), c.g[S0][0]);
            c.cycles += 1;
            c.g[V1][0] = 0xc;
        }
    }
    lvx_assign_temp(c, S1, A2, 0x0022_3db0, 0x0022_3dd0)
}

/// Op 16: `m_Regs[A] = lunoPow(m_Regs[B], m_Regs[C])` (always nil here).
pub(crate) fn lvx_op_pow(c: &mut Cpu) -> LvxStep {
    c.cycles += 22;
    c.g[A1][0] = 0xc;
    c.g[A0][0] = lvx_srl(c, S2, 24);
    c.mult(0, V0, A0, A1);
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[V1][0] &= 0xff;
    c.g[A2][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, V1, V1, A1);
    c.g[A0][0] = lvx_add(c, V0, A2);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[A1][0] = c.lw(lvx_at(c, A0, 8));
    c.g[A3][0] = c.g[V0][0] & 0xff;
    c.g[V1][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = c.lw(lvx_at(c, V1, 8));
    c.g[V0][0] = LVX_NIL_HI;
    lvx_temp_nil(c, A2, V1, A0);
    c.g[V1][0] = 0xc;
    lvx_assign_temp(c, A3, A3, 0x0022_3e88, 0x0022_3ea8)
}

// ---------------------------------------------------------------------------
// Tables.

/// Op 18: `m_Regs[A].castTable().set(m_Regs[B], m_Regs[C])`.
pub(crate) fn lvx_op_table_set(c: &mut Cpu) -> LvxStep {
    c.cycles += 16;
    c.g[V0][0] = 0xc;
    c.g[A2][0] = lvx_srl(c, S2, 24);
    c.mult(0, V1, A2, V0);
    c.g[A3][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A0][0] = lvx_srl(c, S2, 8);
    c.g[A1][0] = lvx_srl(c, S2, 16);
    c.g[A0][0] &= 0xff;
    c.g[A1][0] &= 0xff;
    c.g[A2][0] = lvx_add(c, V1, A3);
    c.mult(0, V1, A0, V0);
    c.g[A0][0] = lvx_add(c, V1, A3);
    c.mult(0, V1, A1, V0);
    c.g[A1][0] = lvx_add(c, V1, A3);
    LVX_TABLE_SET.call(c, 0x0022_4068)?;
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x78));
    Ok(LvxNext::Test)
}

/// Op 28: op 18, then `m_Regs[B].Data.Index++`.
pub(crate) fn lvx_op_table_set_next(c: &mut Cpu) -> LvxStep {
    c.cycles += 20;
    c.g[V0][0] = 0xc;
    c.g[A2][0] = lvx_srl(c, S2, 24);
    c.mult(0, V1, A2, V0);
    c.g[A1][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A0][0] = lvx_srl(c, S2, 8);
    c.g[S0][0] = lvx_srl(c, S2, 16);
    c.g[A0][0] &= 0xff;
    c.g[S0][0] &= 0xff;
    c.mult(0, S0, S0, V0);
    c.g[A2][0] = lvx_add(c, V1, A1);
    c.mult(0, V1, A0, V0);
    c.g[A0][0] = lvx_add(c, V1, A1);
    c.g[A1][0] = lvx_add(c, A1, S0);
    LVX_TABLE_SET.call(c, 0x0022_40a8)?;
    c.g[V1][0] = c.lw(lvx_at(c, S5, 0));
    c.g[S0][0] = lvx_add(c, S0, V1);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 0));
    c.g[V0][0] = lvx_addi(c, V0, 1);
    c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Op 24: `m_Regs[A] = m_Regs[B].castTable().get(m_Regs[C])`.
pub(crate) fn lvx_op_table_get(c: &mut Cpu) -> LvxStep {
    c.cycles += 20;
    c.g[A1][0] = lvx_srl(c, S2, 24);
    c.g[S1][0] = 0xc;
    c.mult(0, V0, A1, S1);
    c.g[V1][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A0][0] = lvx_srl(c, S2, 16);
    c.g[S0][0] = lvx_srl(c, S2, 8);
    c.g[A0][0] &= 0xff;
    c.g[S0][0] &= 0xff;
    c.g[S3][0] = 4;
    c.g[A1][0] = lvx_add(c, V0, V1);
    c.mult(0, V0, A0, S1);
    c.g[A0][0] = lvx_add(c, V0, V1);
    LVX_TABLE_FIND.call(c, 0x0022_4694)?;
    c.mult(0, S0, S0, S1);
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(lvx_at(c, S5, 0));
    c.g[S0][0] = lvx_add(c, S0, V0);
    c.g[V1][0] = c.lw(lvx_at(c, S0, 8));
    if c.g[V1][0] != c.g[S3][0] {
        c.sw(lvx_at(c, S0, 8), 0);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_46bc)?;
        c.sw(lvx_at(c, S0, 8), 0);
    }
    c.cycles += 4;
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S2, 8));
    let table = c.g[V1][0] == c.g[S3][0];
    c.g[V0][0] = 5;
    if table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_47bc)?;
        c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    } else {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(lvx_at(c, S2, 0));
            c.cycles += 1;
            c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
        } else {
            c.cycles += 5;
            let src = lvx_at(c, S2, 0);
            lvx_ld8(c, V1, src);
            let dst = lvx_at(c, S0, 0);
            lvx_sd8(c, V1, dst);
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    }
    c.cycles += 2;
    c.sw(lvx_at(c, S0, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Op 25: `m_Regs[A] = *lunoNewTable(new cLunoTab, 0x20)`.
pub(crate) fn lvx_op_new_table(c: &mut Cpu) -> LvxStep {
    c.cycles += 20;
    c.g[S0][0] = lvx_srl(c, S2, 8);
    c.g[A1][0] = 0x48_0000;
    c.g[S0][0] &= 0xff;
    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x6860));
    c.g[A0][0] = 4;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[S2][0] = 4;
    call_ptr(c, LVX_MEM_ALLOC_NAMED, 0x0022_4710)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x20;
    LVX_TABLE_CTOR.call(c, 0x0022_471c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V1][0] = c.lw(lvx_at(c, S5, 0));
    c.g[V0][0] = 0xc;
    c.mult(0, S0, S0, V0);
    c.g[S0][0] = lvx_add(c, S0, V1);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    if c.g[V0][0] != c.g[S2][0] {
        c.sw(lvx_at(c, S0, 8), 0);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4748)?;
        c.sw(lvx_at(c, S0, 8), 0);
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S1][0];
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[A0][0] = c.g[S0][0];
    LVX_TABLE_ASSIGN.call(c, 0x0022_475c)?;
    c.sw(lvx_at(c, S0, 8), c.g[S2][0]);
    Ok(LvxNext::Loop)
}

// ---------------------------------------------------------------------------
// Arguments and C calls.

/// Op 32: `args[argDepth++] = {A, m_Regs[B]}`.
pub(crate) fn lvx_op_push_reg(c: &mut Cpu) -> LvxStep {
    c.cycles += 17;
    c.g[V0][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[A1][0] = lvx_srl(c, S2, 16);
    c.g[A2][0] = c.lw(lvx_at(c, SP, 0x7c));
    c.g[A0][0] = lvx_srl(c, S2, 8);
    c.g[V1][0] = sx(r32(c, V0) << 4);
    c.g[A0][0] &= 0xff;
    c.g[V1][0] = lvx_add(c, V1, A2);
    c.g[V0][0] = 0xc;
    c.g[A1][0] &= 0xff;
    c.sw(lvx_at(c, V1, 0), c.g[A0][0]);
    c.mult(0, A1, A1, V0);
    c.g[S1][0] = lvx_addi(c, V1, 4);
    c.g[V0][0] = c.lw(lvx_at(c, S5, 0));
    c.g[S2][0] = 4;
    c.g[V1][0] = c.lw(lvx_at(c, S1, 8));
    let not_table = c.g[V1][0] != c.g[S2][0];
    c.g[S0][0] = lvx_add(c, A1, V0);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_2940)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S1, 8), 0);
    c.sw(lvx_at(c, S1, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V1][0] != c.g[S2][0];
    c.g[V0][0] = 5;
    if !not_table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_2960)?;
        c.g[A1][0] = c.lw(lvx_at(c, SP, 0x80));
    } else {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(lvx_at(c, S0, 0));
            c.cycles += 1;
            c.sw(lvx_at(c, S1, 0), c.g[V0][0]);
        } else {
            c.cycles += 5;
            let src = lvx_at(c, S0, 0);
            lvx_ld8(c, A3, src);
            let dst = lvx_at(c, S1, 0);
            lvx_sd8(c, A3, dst);
        }
        c.cycles += 1;
        c.g[A1][0] = c.lw(lvx_at(c, SP, 0x80));
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    c.g[A1][0] = lvx_addi(c, A1, 1);
    c.sw(lvx_at(c, SP, 0x80), c.g[A1][0]);
    c.sw(lvx_at(c, S1, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Ops 37..39: the argument slot `args[argDepth]` (`s1`, its value `s2`),
/// the constant `ops[pc++]` spilled; returns "not a table reference".
pub(crate) fn lvx_push_slot(c: &mut Cpu, spill: u32) -> bool {
    c.cycles += 17;
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x74));
    c.g[V0][0] = sx(r32(c, S6) << 2);
    c.g[A1][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[A0][0] = lvx_srl(c, S2, 8);
    c.g[V0][0] = lvx_add(c, V0, V1);
    c.g[A2][0] = c.lw(lvx_at(c, SP, 0x7c));
    c.g[V1][0] = sx(r32(c, A1) << 4);
    c.g[A0][0] &= 0xff;
    c.g[A1][0] = c.lw(lvx_at(c, V0, 0));
    c.g[S1][0] = lvx_add(c, V1, A2);
    c.g[S2][0] = lvx_addi(c, S1, 4);
    c.sw(lvx_at(c, SP, spill), c.g[A1][0]);
    c.g[V1][0] = 4;
    c.sw(lvx_at(c, S1, 0), c.g[A0][0]);
    c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    let not_table = c.g[V0][0] != c.g[V1][0];
    c.g[S6][0] = lvx_addi(c, S6, 1);
    not_table
}

/// Ops 37/39: `args[argDepth++] = {A, (int)ops[pc++]}`.
pub(crate) fn lvx_op_push_word(c: &mut Cpu, spill: u32, dtor_ret: u32, op39: bool) -> LvxStep {
    if !lvx_push_slot(c, spill) {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, dtor_ret)?;
    }
    c.cycles += if op39 { 5 } else { 6 };
    c.sw(lvx_at(c, S2, 8), 0);
    c.g[V1][0] = 1;
    c.sw(lvx_at(c, S1, 4), 0);
    c.g[A3][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[V0][0] = c.lw(lvx_at(c, SP, spill));
    c.cycles += 5;
    c.g[A3][0] = lvx_addi(c, A3, 1);
    c.sw(lvx_at(c, SP, 0x80), c.g[A3][0]);
    c.sw(lvx_at(c, S1, 4), c.g[V0][0]);
    c.sw(lvx_at(c, S2, 8), c.g[V1][0]);
    Ok(LvxNext::Loop)
}

/// Op 38: `args[argDepth++] = {A, (float)ops[pc++]}`.
pub(crate) fn lvx_op_push_float(c: &mut Cpu) -> LvxStep {
    if !lvx_push_slot(c, 0x64) {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4930)?;
    }
    c.cycles += 10;
    c.sw(lvx_at(c, S2, 8), 0);
    c.g[V0][0] = 2;
    c.sw(lvx_at(c, S1, 4), 0);
    c.g[A3][0] = c.lw(lvx_at(c, SP, 0x80));
    let a = lvx_at(c, SP, 0x64);
    c.lwc1(0, a);
    c.g[A3][0] = lvx_addi(c, A3, 1);
    c.sw(lvx_at(c, SP, 0x80), c.g[A3][0]);
    let d = lvx_at(c, S1, 4);
    c.swc1(0, d);
    c.sw(lvx_at(c, S2, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Op 40: `args[argDepth++] = {A, (int)B}`.
pub(crate) fn lvx_op_push_b(c: &mut Cpu) -> LvxStep {
    c.cycles += 14;
    c.g[V0][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[A0][0] = c.lw(lvx_at(c, SP, 0x7c));
    c.g[V1][0] = sx(r32(c, V0) << 4);
    c.g[V0][0] = lvx_srl(c, S2, 8);
    c.g[S1][0] = lvx_add(c, V1, A0);
    c.g[V0][0] &= 0xff;
    c.g[S0][0] = lvx_addi(c, S1, 4);
    c.sw(lvx_at(c, S1, 0), c.g[V0][0]);
    c.g[V1][0] = lvx_srl(c, S2, 16);
    c.g[V1][0] &= 0xff;
    c.g[A0][0] = 4;
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[A0][0];
    c.sw(lvx_at(c, SP, 0x10), c.g[V1][0]);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4a14)?;
    }
    c.cycles += 10;
    c.sw(lvx_at(c, S0, 8), 0);
    c.g[V1][0] = 1;
    c.sw(lvx_at(c, S1, 4), 0);
    c.g[A1][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[V0][0] = c.lw(lvx_at(c, SP, 0x10));
    c.g[A1][0] = lvx_addi(c, A1, 1);
    c.sw(lvx_at(c, SP, 0x80), c.g[A1][0]);
    c.sw(lvx_at(c, S1, 4), c.g[V0][0]);
    c.sw(lvx_at(c, S0, 8), c.g[V1][0]);
    Ok(LvxNext::Loop)
}

/// Op 41: `args[argDepth++] = {A, (float)B}`.
pub(crate) fn lvx_op_push_b_float(c: &mut Cpu) -> LvxStep {
    c.cycles += 16;
    c.g[A2][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[A0][0] = lvx_srl(c, S2, 16);
    c.g[A3][0] = c.lw(lvx_at(c, SP, 0x7c));
    c.g[V1][0] = lvx_srl(c, S2, 8);
    c.g[V0][0] = sx(r32(c, A2) << 4);
    c.g[V1][0] &= 0xff;
    c.g[S1][0] = lvx_add(c, V0, A3);
    c.g[A0][0] &= 0xff;
    c.sw(lvx_at(c, S1, 0), c.g[V1][0]);
    c.mtc1(A0, 0);
    c.cvt_s_w(0, 0);
    c.g[S0][0] = lvx_addi(c, S1, 4);
    c.g[V1][0] = 4;
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[V1][0];
    let s = lvx_at(c, SP, 0x6c);
    c.swc1(0, s);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_4a88)?;
    }
    c.cycles += 10;
    c.sw(lvx_at(c, S0, 8), 0);
    c.g[V0][0] = 2;
    c.sw(lvx_at(c, S1, 4), 0);
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x80));
    let a = lvx_at(c, SP, 0x6c);
    c.lwc1(0, a);
    c.g[V1][0] = lvx_addi(c, V1, 1);
    c.sw(lvx_at(c, SP, 0x80), c.g[V1][0]);
    let d = lvx_at(c, S1, 4);
    c.swc1(0, d);
    c.sw(lvx_at(c, S0, 8), c.g[V0][0]);
    Ok(LvxNext::Loop)
}

/// Op 33: `m_Regs[Ret] = gLunoCFunctions[CFunc](NumArgs,
/// &args[argDepth - NumArgs]); argDepth -= NumArgs;` (the result comes back
/// in the temporary at sp).
pub(crate) fn lvx_op_ccall(c: &mut Cpu) -> LvxStep {
    c.cycles += 26;
    c.g[V0][0] = lvx_srl(c, S2, 14);
    c.g[V1][0] = 0x44_0000;
    c.g[V1][0] = sx(LVX_C_FUNCTIONS);
    c.g[V0][0] &= 0x3fc;
    c.g[V0][0] = lvx_add(c, V0, V1);
    c.g[A3][0] = lvx_srl(c, S2, 24);
    c.g[V1][0] = c.lw(lvx_at(c, SP, 0x80));
    c.g[A1][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw(lvx_at(c, SP, 0x7c));
    c.g[S0][0] = lvx_srl(c, S2, 8);
    c.g[A2][0] = sx(r32(c, V1).wrapping_sub(r32(c, A1)));
    c.g[S0][0] &= 0xff;
    c.g[A2][0] = sx(r32(c, A2) << 4);
    c.g[V1][0] = c.lw(lvx_at(c, V0, 0));
    c.g[A2][0] = lvx_add(c, A0, A2);
    c.g[S4][0] = c.g[A3][0];
    c.g[A0][0] = c.g[SP][0];
    let target = r32(c, V1);
    c.g[S1][0] = 4;
    call_ptr(c, target, 0x0022_3fa4)?;
    c.g[V0][0] = 0xc;
    c.g[V1][0] = c.lw(lvx_at(c, S5, 0));
    c.mult(0, S0, S0, V0);
    c.g[S0][0] = lvx_add(c, S0, V1);
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[S1][0];
    c.g[S2][0] = c.g[SP][0];
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, 0x0022_3fcc)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S0, 8), 0);
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, SP, 8));
    let not_table = c.g[V1][0] != c.g[S1][0];
    c.g[V0][0] = 5;
    if !not_table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_3fec)?;
        c.g[V1][0] = c.lw(lvx_at(c, SP, 0x80));
    } else {
        c.cycles += 2;
        let not_pair = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(lvx_at(c, SP, 0));
        if not_pair {
            c.cycles += 1;
            c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
        } else {
            c.cycles += 5;
            let src = lvx_at(c, SP, 0);
            lvx_ld8(c, A1, src);
            let dst = lvx_at(c, S0, 0);
            lvx_sd8(c, A1, dst);
        }
        c.cycles += 1;
        c.g[V1][0] = c.lw(lvx_at(c, SP, 0x80));
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[SP][0];
    c.g[V0][0] = c.lw(lvx_at(c, S2, 8));
    c.g[A1][0] = 2;
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, S4)));
    c.sw(lvx_at(c, SP, 0x80), c.g[V1][0]);
    lvx_release_temp(c)
}

// ---------------------------------------------------------------------------
// The numeric for loop.

/// The A-vs-B test of op 36 (`up`: `A <= B`, else `A >= B`, as `!(x < y)`;
/// `a3` = &A, `a1` = &B, `v1` = A's type, `a0` = B's). Leaves the result in
/// `v0`.
pub(crate) fn lvx_for_compare(c: &mut Cpu, up: bool) {
    let (x, y) = if up { (V1, V0) } else { (V0, V1) };
    let signed;
    if c.g[A0][0] == c.g[V1][0] {
        c.cycles += 3;
        c.g[x][0] = c.lw(lvx_at(c, A3, 0));
        c.g[y][0] = c.lw(lvx_at(c, A1, 0));
        signed = true;
    } else {
        c.g[V0][0] = 2;
        c.cycles += 2;
        let to_int = if c.g[V1][0] != c.g[V0][0] {
            c.g[A0][0] = c.lw(lvx_at(c, A3, 8));
            c.cycles += 1;
            c.g[V0][0] = 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A0][0] = c.lw(lvx_at(c, A3, 8));
                true
            } else {
                false
            }
        };
        if !to_int {
            c.cycles += 6;
            let a = lvx_at(c, A1, 0);
            c.lwc1(1, a);
            c.cvt_s_w(1, 1);
            let b = lvx_at(c, A3, 0);
            c.lwc1(0, b);
            if up {
                c.c_le_s(0, 1);
            } else {
                c.c_le_s(1, 0);
            }
            if !c.fcc() {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            return;
        }
        c.cycles += 2;
        let not_int = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        let mut mixed = false;
        if !not_int {
            c.cycles += 4;
            c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
            c.g[V0][0] = 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                let a = lvx_at(c, A1, 0);
                c.lwc1(0, a);
                c.g[x][0] = c.lw(lvx_at(c, A3, 0));
                c.cvt_w_s(1, 0);
                c.mfc1(y, 1);
                mixed = true;
            } else {
                c.g[V0][0] = 3;
            }
        }
        signed = mixed;
        if !mixed {
            c.cycles += 2;
            let not_id = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if !not_id {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                c.g[V0][0] = 1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[x][0] = c.lw(lvx_at(c, A3, 0));
                    c.g[y][0] = c.lw(lvx_at(c, A1, 0));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] ^= 1;
                }
            }
        }
    }
    if signed {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[V0][0] ^= 1;
    }
}

/// Op 36: `target = ops[pc++]; if (step < 0 ? A >= B : A <= B) { A += C;
/// pc = target; }` (A the counter, B the limit, C the step).
pub(crate) fn lvx_op_for(c: &mut Cpu) -> LvxStep {
    c.cycles += 21;
    c.g[A3][0] = c.lw(lvx_at(c, SP, 0x74));
    c.g[V0][0] = sx(r32(c, S6) << 2);
    c.g[S3][0] = 2;
    c.g[A0][0] = c.g[SP][0];
    c.g[V0][0] = lvx_add(c, V0, A3);
    c.g[S0][0] = lvx_srl(c, S2, 24);
    c.g[FP][0] = c.lw(lvx_at(c, V0, 0));
    c.g[S1][0] = c.g[S0][0] & 0xff;
    c.sw(lvx_at(c, SP, 8), c.g[S3][0]);
    c.g[S4][0] = c.g[S0][0];
    // The temporary = cLunoVar(0.0f).
    LVX_REG_AS_FLOAT.call(c, 0x0022_42b8)?;
    c.g[S6][0] = lvx_addi(c, S6, 1);
    c.g[V1][0] = 0xc;
    c.sw(lvx_at(c, V0, 0), 0);
    c.mult(0, S1, S1, V1);
    c.g[V0][0] = c.lw(lvx_at(c, S5, 0));
    c.g[A0][0] = c.lw(lvx_at(c, SP, 8));
    c.g[S1][0] = lvx_add(c, S1, V0);
    c.g[V0][0] = c.lw(lvx_at(c, S1, 8));
    let differ = c.g[A0][0] != c.g[V0][0];
    c.g[A1][0] = c.g[SP][0];
    // s1 = step < 0.0f
    if !differ {
        c.cycles += 4;
        c.g[V1][0] = c.lw(lvx_at(c, S1, 0));
        c.g[V0][0] = c.lw(lvx_at(c, SP, 0));
        c.g[S1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    } else {
        c.cycles += 2;
        let to_int = if c.g[V0][0] != c.g[S3][0] {
            c.g[A0][0] = c.lw(lvx_at(c, S1, 8));
            c.cycles += 1;
            c.g[V0][0] = 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A0][0] = c.lw(lvx_at(c, S1, 8));
                true
            } else {
                false
            }
        };
        if !to_int {
            c.cycles += 7;
            let a = lvx_at(c, SP, 0);
            c.lwc1(1, a);
            c.cvt_s_w(1, 1);
            let b = lvx_at(c, S1, 0);
            c.lwc1(0, b);
            c.c_lt_s(0, 1);
            let neg = c.fcc();
            c.g[V1][0] = 1;
            if !neg {
                c.cycles += 1;
                c.g[V1][0] = 0;
            }
            c.cycles += 2;
            c.g[S1][0] = c.g[V1][0];
        } else {
            c.cycles += 2;
            let not_int = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = 3;
            let mut done = false;
            if !not_int {
                c.cycles += 4;
                c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                c.g[V0][0] = 2;
                let not_float = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 3;
                if !not_float {
                    c.cycles += 6;
                    let a = lvx_at(c, A1, 0);
                    c.lwc1(0, a);
                    c.g[V1][0] = c.lw(lvx_at(c, S1, 0));
                    c.cvt_w_s(1, 0);
                    c.mfc1(V0, 1);
                    c.g[S1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    done = true;
                }
            }
            if !done {
                c.cycles += 2;
                if c.g[A0][0] != c.g[V0][0] {
                    c.g[S1][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(lvx_at(c, A1, 8));
                    c.g[V0][0] = 1;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S1][0] = 0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(lvx_at(c, S1, 0));
                        c.g[V0][0] = c.lw(lvx_at(c, A1, 0));
                        c.g[S1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    }
                }
            }
        }
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 2;
    LVX_REG_DTOR.call(c, 0x0022_4390)?;
    let up = c.g[S1][0] == 0;
    c.g[V0][0] = lvx_srl(c, S2, 16);

    // a1 = &m_Regs[B], a3 = &m_Regs[A], t0 = m_Regs.
    c.cycles += 14;
    c.g[A1][0] = 0xc;
    c.g[A2][0] = lvx_srl(c, S2, 8);
    c.g[V0][0] &= 0xff;
    c.mult(0, A3, V0, A1);
    c.g[V1][0] = c.g[A2][0] & 0xff;
    c.mult(0, V1, V1, A1);
    c.g[A0][0] = c.lw(lvx_at(c, S5, 0));
    c.g[T0][0] = c.g[A0][0];
    c.g[A1][0] = lvx_add(c, A3, A0);
    c.g[A3][0] = lvx_add(c, V1, A0);
    c.g[A0][0] = c.lw(lvx_at(c, A1, 8));
    c.g[V1][0] = c.lw(lvx_at(c, A3, 8));
    lvx_for_compare(c, up);
    c.cycles += 2;
    let stop = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    if stop {
        return Ok(LvxNext::Loop);
    }

    // m_Regs[A] += m_Regs[C] (through the sum at sp+0x20); pc = target.
    let (add_ret, dtor_ret) = if up {
        (0x0022_45dc, 0x0022_45f4)
    } else {
        (0x0022_449c, 0x0022_44b4)
    };
    c.cycles += 12;
    c.g[A0][0] = lvx_addi(c, SP, 0x20);
    c.g[A2][0] = 0xc;
    c.g[S2][0] = 4;
    c.mult(0, V1, V0, A2);
    c.mult(0, V0, S4, A2);
    c.g[S0][0] = lvx_add(c, V1, T0);
    c.g[A2][0] = lvx_add(c, V0, T0);
    c.g[A1][0] = c.g[S0][0];
    LVX_REG_ADD.call(c, add_ret)?;
    c.g[V0][0] = c.lw(lvx_at(c, S0, 8));
    let not_table = c.g[V0][0] != c.g[S2][0];
    c.g[S1][0] = lvx_addi(c, SP, 0x20);
    if !not_table {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        LVX_TABLE_DTOR.call(c, dtor_ret)?;
    }
    c.cycles += 5;
    c.sw(lvx_at(c, S0, 8), 0);
    c.sw(lvx_at(c, S0, 0), 0);
    c.g[V1][0] = c.lw(lvx_at(c, S1, 8));
    let table = c.g[V1][0] == c.g[S2][0];
    c.g[V0][0] = 5;
    if table {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        LVX_TABLE_ASSIGN.call(c, 0x0022_4614)?;
        c.g[V0][0] = c.lw(lvx_at(c, S1, 8));
    } else {
        c.cycles += 2;
        let pair = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.lw(lvx_at(c, SP, 0x20));
        if pair {
            c.cycles += 6;
            let src = lvx_at(c, SP, 0x20);
            lvx_ld8(c, V1, src);
            let dst = lvx_at(c, S0, 0);
            lvx_sd8(c, V1, dst);
            c.g[V0][0] = c.lw(lvx_at(c, S1, 8));
        } else {
            c.cycles += if up { 1 } else { 2 };
            c.sw(lvx_at(c, S0, 0), c.g[V0][0]);
            c.cycles += 1;
            c.g[V0][0] = c.lw(lvx_at(c, S1, 8));
        }
    }
    c.cycles += 3;
    c.g[A0][0] = lvx_addi(c, SP, 0x20);
    c.g[A1][0] = 2;
    c.g[S6][0] = c.g[FP][0];
    lvx_release_temp(c)
}

// ---------------------------------------------------------------------------
// cLunoTable_Set (0x225338)

/// `void cLunoTable::set(cLunoTable* self, const cLunoValue* key, const
/// cLunoValue* val)` (0x225338): the Luno script hash table's insert/update
/// of `key -> val` (keys of type 0 are ignored): the bucket is `key.word0 %
/// modulus`; an empty bucket takes key and value in place, a matching key
/// is overwritten (releasing a replaced table value, copying a pair, else
/// the word; a value that becomes nil unlinks and deletes its node), and
/// otherwise a new 0x20-byte `cLunoTableEntry` node is allocated and linked
/// after the chain's last node. Written register for register (the original
/// has this exact unrolled shape; its blocks are kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_parens
)]
pub fn c_luno_table_set(c: &mut Cpu) {
    'b_2257c0: {
        'b_2257bc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let t = c.g[A3][0] == 0u64;
            c.g[S2][0] = c.g[A2][0];
            if t {
                break 'b_2257bc;
            } else {
                'b_225380: {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[T0][0] = c.g[V0][0];
                    c.divu(0, 2, 3);
                    if c.g[V1][0] == 0u64 {
                        break 'b_225380;
                    } else {
                        break 'b_225380;
                    }
                }
                'b_225554: {
                    'b_22554c: {
                        'b_225544: {
                            'b_22552c: {
                                'b_225524: {
                                    'b_225510: {
                                        'b_225448: {
                                            c.cycles += 7;
                                            c.g[A1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.hi[0];
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                                            c.g[S0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                break 'b_225448;
                                            } else {
                                                'b_2253bc: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[S3][0] = 0x4;
                                                    if c.g[V0][0] != c.g[S3][0] {
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8),
                                                            0u64,
                                                        );
                                                        break 'b_2253bc;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x002253b8);
                                                        c.g[A1][0] = 0x2;
                                                        c.pc = 0x00224df0;
                                                        ee::s05::cLunoTable_dtor(c);
                                                        if c.pc != 0x002253b8 {
                                                            return;
                                                        }
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8),
                                                            0u64,
                                                        );
                                                        break 'b_2253bc;
                                                    }
                                                }
                                                'b_225404: {
                                                    'b_2253e0: {
                                                        c.cycles += 4;
                                                        c.sw((c.g[S0][0] as u32), 0u64);
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        let t = c.g[V1][0] != c.g[S3][0];
                                                        c.g[V0][0] = 0x5;
                                                        if t {
                                                            break 'b_2253e0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x002253d8);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            c.pc = 0x00225068;
                                                            ee::s05::cLunoTable_Assign(c);
                                                            if c.pc != 0x002253d8 {
                                                                return;
                                                            }
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_225404;
                                                            } else {
                                                                break 'b_2253e0;
                                                            }
                                                        }
                                                    }
                                                    'b_225400: {
                                                        'b_2253fc: {
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != c.g[V0][0] {
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[S1][0] as u32));
                                                                break 'b_2253fc;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.ldl(
                                                                    2,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                c.ldr(2, (c.g[S1][0] as u32));
                                                                c.sdl(
                                                                    2,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                let t = true;
                                                                c.sdr(2, (c.g[S0][0] as u32));
                                                                if t {
                                                                    break 'b_225400;
                                                                } else {
                                                                    break 'b_2253fc;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                                                        break 'b_225400;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_225404;
                                                }
                                                'b_225424: {
                                                    c.cycles += 5;
                                                    c.g[S3][0] = 0x4;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                    c.g[S1][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[V0][0] != c.g[S3][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x8),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_225424;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x00225424);
                                                        c.g[A1][0] = 0x2;
                                                        c.pc = 0x00224df0;
                                                        ee::s05::cLunoTable_dtor(c);
                                                        if c.pc != 0x00225424 {
                                                            return;
                                                        }
                                                        break 'b_225424;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                let t = c.g[V1][0] == c.g[S3][0];
                                                c.g[V0][0] = 0x5;
                                                if t {
                                                    break 'b_225510;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];

                                                    if t {
                                                        break 'b_22552c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                        if t {
                                                            break 'b_225544;
                                                        } else {
                                                            break 'b_225448;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_2254dc: {
                                            'b_2254d4: {
                                                c.cycles += 2;
                                                if c.g[A3][0] == c.g[A0][0] {
                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                    break 'b_2254d4;
                                                } else {
                                                    'b_225484: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x2;
                                                        let t = c.g[A0][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_225484;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A3][0] != c.g[V0][0];

                                                            if t {
                                                                break 'b_225484;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.mtc1(8, 1);
                                                                c.cvt_s_w(1, 1);
                                                                c.lwc1(0, (c.g[S0][0] as u32));
                                                                c.c_eq_s(0, 1);
                                                                let t = c.fcc();

                                                                if t {
                                                                    break 'b_2254dc;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V0][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2254dc;
                                                                    } else {
                                                                        break 'b_225484;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2254b8: {
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_2254b8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = 0x2;
                                                            if c.g[V1][0] != c.g[V0][0] {
                                                                c.g[V0][0] = 0x3;
                                                                break 'b_2254b8;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.lwc1(0, (c.g[S1][0] as u32));
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[S0][0] as u32));
                                                                c.cvt_w_s(1, 0);
                                                                c.mfc1(3, 1);
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] ^ c.g[V1][0];
                                                                let t = true;
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0x1u64) as u64;
                                                                if t {
                                                                    break 'b_2254dc;
                                                                } else {
                                                                    break 'b_2254b8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_2254dc;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = 0x1;
                                                        if c.g[V1][0] != c.g[V0][0] {
                                                            c.g[V0][0] = 0u64;
                                                            break 'b_2254dc;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                            break 'b_2254d4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = c.g[V0][0] ^ c.g[T0][0];
                                            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                            break 'b_2254dc;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = 0x4;
                                        if t {
                                            break 'b_225554;
                                        } else {
                                            'b_2254fc: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                let t = c.g[V0][0] != c.g[S3][0];
                                                c.g[S1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                                if t {
                                                    break 'b_2254fc;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[RA][0] = sx(0x002254fc);
                                                    c.g[A1][0] = 0x2;
                                                    c.pc = 0x00224df0;
                                                    ee::s05::cLunoTable_dtor(c);
                                                    if c.pc != 0x002254fc {
                                                        return;
                                                    }
                                                    break 'b_2254fc;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] != c.g[S3][0];
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_225524;
                                            } else {
                                                break 'b_225510;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x0022551c);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.pc = 0x00225068;
                                    ee::s05::cLunoTable_Assign(c);
                                    if c.pc != 0x0022551c {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_22554c;
                                    } else {
                                        break 'b_225524;
                                    }
                                }
                                c.cycles += 2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                    break 'b_225544;
                                } else {
                                    break 'b_22552c;
                                }
                            }
                            c.cycles += 6;
                            c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                            c.ldr(2, (c.g[S2][0] as u32));
                            c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x13));
                            c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_22554c;
                            } else {
                                break 'b_225544;
                            }
                        }
                        c.cycles += 2;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_22554c;
                    }
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        break 'b_2257bc;
                    } else {
                        break 'b_225554;
                    }
                }
                'b_2256bc: {
                    c.cycles += 4;
                    c.g[S3][0] = c.g[S0][0];
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                    if c.g[S0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_2256bc;
                    } else {
                        c.cycles += 5;
                        c.g[A2][0] = 0x2;
                        c.g[A1][0] = 0x1;
                        c.g[S4][0] = 0x4;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        'l_225578: loop {
                            'b_225600: {
                                'b_2255f8: {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if c.g[V1][0] == c.g[A0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                        break 'b_2255f8;
                                    } else {
                                        'b_2255b8: {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] != c.g[A2][0];

                                            if t {
                                                break 'b_2255b8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[A1][0];

                                                if t {
                                                    break 'b_2255b8;
                                                } else {
                                                    c.cycles += 7;
                                                    c.mtc1(8, 1);
                                                    c.cvt_s_w(1, 1);
                                                    c.lwc1(0, (c.g[S0][0] as u32));
                                                    c.c_eq_s(0, 1);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_225600;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = 0u64;
                                                        if t {
                                                            break 'b_225600;
                                                        } else {
                                                            break 'b_2255b8;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_2255e4: {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] != c.g[A1][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_2255e4;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[A2][0];

                                                if t {
                                                    break 'b_2255e4;
                                                } else {
                                                    c.cycles += 7;
                                                    c.lwc1(0, (c.g[S1][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                    c.cvt_w_s(1, 0);
                                                    c.mfc1(3, 1);
                                                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                                    let t = true;
                                                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                                    if t {
                                                        break 'b_225600;
                                                    } else {
                                                        break 'b_2255e4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A0][0] != c.g[V0][0] {
                                            c.g[V0][0] = 0u64;
                                            break 'b_225600;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[A1][0] {
                                                c.g[V0][0] = 0u64;
                                                break 'b_225600;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                break 'b_2255f8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.g[V0][0] ^ c.g[T0][0];
                                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                break 'b_225600;
                            }
                            'b_2256ac: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0u64 {
                                    c.g[S3][0] = c.g[S0][0];
                                    break 'b_2256ac;
                                } else {
                                    'b_225620: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        let t = c.g[V0][0] != c.g[S4][0];
                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_225620;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x00225620);
                                            c.g[A1][0] = 0x2;
                                            c.pc = 0x00224df0;
                                            ee::s05::cLunoTable_dtor(c);
                                            if c.pc != 0x00225620 {
                                                return;
                                            }
                                            break 'b_225620;
                                        }
                                    }
                                    'b_225670: {
                                        'b_225648: {
                                            c.cycles += 5;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] != c.g[S4][0];
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_225648;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x00225640);
                                                c.g[A1][0] = c.g[S2][0];
                                                c.pc = 0x00225068;
                                                ee::s05::cLunoTable_Assign(c);
                                                if c.pc != 0x00225640 {
                                                    return;
                                                }
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_225670;
                                                } else {
                                                    break 'b_225648;
                                                }
                                            }
                                        }
                                        'b_225668: {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                break 'b_225668;
                                            } else {
                                                c.cycles += 6;
                                                c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                                c.ldr(2, (c.g[S2][0] as u32));
                                                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x13));
                                                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_225670;
                                                } else {
                                                    break 'b_225668;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        break 'b_225670;
                                    }
                                    c.cycles += 4;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    if c.g[V1][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        break 'b_2257c0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        let t = c.g[S0][0] == 0u64;
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                        if t {
                                            break 'b_2257bc;
                                        } else {
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                            c.g[A1][0] = 0x3;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x002256a4);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.pc = t;
                                            call_indirect(c, t);
                                            if c.pc != 0x002256a4 {
                                                return;
                                            }
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_2257c0;
                                            } else {
                                                break 'b_2256ac;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            if c.g[S0][0] != 0u64 {
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if event_due(c) {
                                    deliver_event_here(c, 0x00225578);
                                }
                                continue 'l_225578;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                break 'b_2256bc;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x480000;
                if t {
                    break 'b_2257bc;
                } else {
                    'b_225740: {
                        'b_225718: {
                            c.cycles += 16;
                            c.g[A0][0] = 0x20;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6780));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x002256d8);
                            c.g[A3][0] = 0u64;
                            c.pc = 0x00317d70;
                            call_indirect(c, 0x00317d70);
                            if c.pc != 0x002256d8 {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x480000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6770));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            c.g[V1][0] = 0x4;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.sw((c.g[S0][0] as u32), 0u64);
                            if t {
                                break 'b_225718;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00225710);
                                c.g[A1][0] = c.g[S1][0];
                                c.pc = 0x00225068;
                                ee::s05::cLunoTable_Assign(c);
                                if c.pc != 0x00225710 {
                                    return;
                                }
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_225740;
                                } else {
                                    break 'b_225718;
                                }
                            }
                        }
                        'b_22573c: {
                            'b_225738: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                if c.g[A0][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                    break 'b_225738;
                                } else {
                                    c.cycles += 5;
                                    c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, (c.g[S1][0] as u32));
                                    c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                                    let t = true;
                                    c.sdr(2, (c.g[S0][0] as u32));
                                    if t {
                                        break 'b_22573c;
                                    } else {
                                        break 'b_225738;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                            break 'b_22573c;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        break 'b_225740;
                    }
                    'b_225760: {
                        c.cycles += 5;
                        c.g[S4][0] = 0x4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V0][0] != c.g[S4][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        if t {
                            break 'b_225760;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00225760);
                            c.g[A1][0] = 0x2;
                            c.pc = 0x00224df0;
                            ee::s05::cLunoTable_dtor(c);
                            if c.pc != 0x00225760 {
                                return;
                            }
                            break 'b_225760;
                        }
                    }
                    'b_2257b0: {
                        'b_225784: {
                            c.cycles += 4;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            let t = c.g[V1][0] != c.g[S4][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            if t {
                                break 'b_225784;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x0022577c);
                                c.g[A1][0] = c.g[S2][0];
                                c.pc = 0x00225068;
                                ee::s05::cLunoTable_Assign(c);
                                if c.pc != 0x0022577c {
                                    return;
                                }
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_2257b0;
                                } else {
                                    break 'b_225784;
                                }
                            }
                        }
                        'b_2257a8: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x5;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                break 'b_2257a8;
                            } else {
                                c.cycles += 6;
                                c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                c.ldr(2, (c.g[S2][0] as u32));
                                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x13));
                                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_2257b0;
                                } else {
                                    break 'b_2257a8;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_2257b0;
                    }
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                    break 'b_2257bc;
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        break 'b_2257c0;
    }
    c.cycles += 7;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// cLunoTable_Alloc.

/// `void cLunoTable_Alloc(cLunoTableRef* ref, int n)` (0x224e50): allocates
/// a table object (0x14 bytes, "LunoTable": nodes, refCount, modulus) into
/// `ref->table`, then an `n`-entry node array (32 bytes each, preceded by a
/// count word) whose nodes are cleared and given their vtable word at +0x1c;
/// sets `modulus = n - 1` and `refCount = 1`.
pub fn cluno_table_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0xffff_ffff_a000_0000;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 0x14;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x67a0));
        // operator new (EA_MemAllocNamed).
        call_ptr(c, 0x0031_7d70, 0x0022_4e84)?;
        let table = c.g[V0][0] as u32;
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 5);
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] |= 0x10;
        c.g[A2][0] = 0xffff_ffff_a000_0000;
        c.g[S2][0] = sx(table.wrapping_add(0x10));
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x6790));
        Addr(table.wrapping_add(0x10)).sw(c, 0);
        Addr(table.wrapping_add(8)).sw(c, 0);
        c.g[A3][0] = 0;
        Addr(table.wrapping_add(0xc)).sw(c, 0);
        // operator new[] (operator_new_named).
        call_ptr(c, 0x0031_7e30, 0x0022_4eb4)?;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        Addr(c.g[V0][0] as u32).sw(c, c.g[S0][0]);
        c.g[V1][0] = c.g[A2][0];
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0048_0000;
            c.g[A1][0] = u64::MAX;
            c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x6770));
            loop {
                c.cycles += 9;
                let node = c.g[V1][0] as u32;
                Addr(node.wrapping_add(0x1c)).sw(c, c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                Addr(node.wrapping_add(8)).sw(c, 0);
                Addr(node).sw(c, 0);
                Addr(node.wrapping_add(0x14)).sw(c, 0);
                Addr(node.wrapping_add(0xc)).sw(c, 0);
                Addr(node.wrapping_add(0x18)).sw(c, 0);
                let more = c.g[A0][0] != c.g[A1][0];
                c.g[V1][0] = sx(node.wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_4ed8);
                }
            }
        }
        c.cycles += 13;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        Addr(c.g[S2][0] as u32).sw(c, c.g[A2][0]);
        c.g[A0][0] = 1;
        Addr((c.g[A1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        Addr((c.g[V0][0] as u32).wrapping_add(8)).sw(c, c.g[A0][0]);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const LUNO_NODE_KEY_TYPE: u32 = 0x8;

/// `cLunoValue* cLunoTable_Find(cLunoVM* self, cLunoKey* key)` (0x225248):
/// hash lookup. The bucket is `nodes[key.word0 % modulus]`; if its value is
/// non-nil, walk the chain for a node whose key matches (same type and
/// word; an int key matches a float node by value and a float key an int
/// node after truncation). Returns the node's value slot or `luno_gLunoNil`.
pub fn c_luno_table_find(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A2][0] = c.lw(c.g[A0][0] as u32);
    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = c.lw(oc428_at(c.g[A2][0], LUNO_TABLE_MODULUS));
    c.divu(0, A0, V0);
    'nil: {
        c.cycles += 7;
        c.g[A0][0] = c.lw(oc428_at(c.g[A2][0], LUNO_TABLE_NODES));
        c.g[V1][0] = c.hi[0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_VALUE_TYPE));
        let empty = c.g[V0][0] == 0;
        c.g[T0][0] = 2;
        if empty {
            break 'nil;
        }
        c.cycles += 5;
        c.g[A2][0] = c.lw(oc428_at(c.g[A1][0], LUNO_NODE_KEY_TYPE));
        c.g[A3][0] = 1;
        c.g[T1][0] = 3;
        c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_KEY_TYPE));
        'walk: loop {
            // `v0` = this node's key type, `a2` the key's, `a3` = 1, `t0` = 2,
            // `t1` = 3. The comparison leaves a result in `v0`: 1 = match.
            'matched: {
                'xor_words: {
                    'same_type: {
                        c.cycles += 2;
                        if c.g[A2][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            break 'same_type;
                        }
                        'int_key_float_node: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T0][0] {
                                break 'int_key_float_node;
                            }
                            c.cycles += 2;
                            if c.g[A2][0] != c.g[A3][0] {
                                break 'int_key_float_node;
                            }
                            // Float node, int key: compare as floats.
                            c.cycles += 7;
                            c.lwc1(1, c.g[A1][0] as u32);
                            c.cvt_s_w(1, 1);
                            c.lwc1(0, c.g[A0][0] as u32);
                            c.c_eq_s(0, 1);
                            let eq = c.fcc();
                            c.g[V0][0] = 1;
                            if eq {
                                break 'matched;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                            break 'matched;
                        }
                        'float_key_int_node: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[A3][0] {
                                break 'float_key_int_node;
                            }
                            c.cycles += 2;
                            if c.g[A2][0] != c.g[T0][0] {
                                break 'float_key_int_node;
                            }
                            // Int node, float key: truncate the key.
                            c.cycles += 5;
                            c.lwc1(0, c.g[A1][0] as u32);
                            c.cvt_w_s(1, 0);
                            c.mfc1(V1, 1);
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            break 'xor_words;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[T1][0] {
                            c.g[V0][0] = 0;
                            break 'matched;
                        }
                        c.cycles += 2;
                        if c.g[A2][0] != c.g[A3][0] {
                            c.g[V0][0] = 0;
                            break 'matched;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    }
                    // Same key type: compare the words.
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                }
                c.cycles += 2;
                c.g[V0][0] ^= c.g[V1][0];
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = sx(oc428_at(c.g[A0][0], LUNO_NODE_VALUE));
                ret(c);
                return;
            }
            c.g[A0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_NEXT));
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                break 'nil;
            }
            c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_KEY_TYPE));
            if event_due(c) {
                deliver_event_here(c, 0x0022_5290);
            }
            continue 'walk;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 0x4d_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
    ret(c);
}

/// `LunoVMRegister LunoVMRegister_AsId(id)` (0x226620): returns its argument.
pub fn luno_vm_register_as_id(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `void LunoVM_StaticInitDestroy(int initialize, int priority)` (0x225de8).
///
/// Only acts for `priority == 0xFFFF`. With `initialize != 0` it writes the
/// built-in LunoVM constant table: 56 `sV4` values (four floats each) at
/// `0x4c8d18..0x4c9088` (the reference's `new (D_004C8Dxx) sV4(...)` list),
/// plus the nil value's type and word. With `initialize == 0` it constructs
/// `luno_gLunoNil` with the LunoVM variable constructor at 0x225b90 (generated
/// name `LunoVMRegister_dtor`), `a1 = 2`.
///
/// The table part is the generated straight-line body copied by script (its
/// about 480 loads, stores and register moves, in generated order, so the
/// final scratch registers and the FPU loads match): the register bookkeeping
/// that the generated body leaves behind is part of the contract, and the
/// reference's `sV4` list does not model it. 486 cycles on that path.
pub fn luno_static_init_destroy(c: &mut Cpu) {
    'b_2265cc: {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V0][0] = 0xffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if t {
            break 'b_2265cc;
        }
        'b_2265bc: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x4d0000;
            if t {
                break 'b_2265bc;
            }
            c.cycles += 486;
            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5304));
            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5300));
            c.g[T6][0] = 0x4d0000;
            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x52fc));
            c.g[T7][0] = 0x4d0000;
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x52f8));
            c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x72b8));
            c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x72d8));
            c.g[S1][0] = 0x4d0000;
            c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x72c8));
            c.g[S2][0] = 0x4d0000;
            c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x72b8));
            c.g[S3][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x52f4));
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x7288));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x52f0));
            c.g[S5][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x52ec));
            c.g[S4][0] = 0x4d0000;
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[T3][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x52e8));
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x72e8));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x72d8));
            c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x72a8));
            c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x72c8));
            c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x7298));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x72a8));
            c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x7288));
            c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x7298));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x52e4));
            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x7278));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x52e0));
            c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7268));
            c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[T9][0] = 0x4d0000;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 6);
            c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x7258));
            c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x7278));
            c.g[S6][0] = 0x4d0000;
            c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x7268));
            c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x7228));
            c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x72e8));
            c.g[S2][0] = 0x4d0000;
            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x7208));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S3][0] = 0x4d0000;
            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x52dc));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x71f8));
            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x52d8));
            c.g[T8][0] = 0x4d0000;
            c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[FP][0] = 0x4d0000;
            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x52d4));
            c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x71e8));
            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x52d0));
            c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x7248));
            c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[S7][0] = 0x4d0000;
            c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[FP][0] = 0x4d0000;
            c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x71d8));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x7238));
            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x52cc));
            c.g[V1][0] = 0x4d0000;
            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x52c8));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7198));
            c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x7178));
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x52c4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7188));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7168));
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7158));
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x4d0000;
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7148));
            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = 0x4d0000;
            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x52c0));
            c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7118));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A3][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x52bc));
            c.g[A2][0] = 0x4d0000;
            c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7138));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x52b8));
            c.g[A2][0] = 0x4d0000;
            c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x7258));
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7108));
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x52b4));
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x70e8));
            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x52b0));
            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x52ac));
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x52a8));
            c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
            c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
            c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x7248));
            c.g[T2][0] = 0x4d0000;
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
            c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7088));
            c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x7238));
            c.g[T2][0] = 0x4d0000;
            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x52a4));
            c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x7228));
            c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x7218));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7218));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x52a0));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x7208));
            c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7068));
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x529c));
            c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7058));
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5298));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5294));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x71f8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x71e8));
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
            c.g[T5][0] = 0x4d0000;
            c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x7218));
            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
            c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6fd8));
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.g[T7][0] = 0x4d0000;
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
            c.g[S2][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6ff8));
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T7][0] = 0x4d0000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
            c.g[T5][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6fb8));
            c.g[S3][0] = 0x4d0000;
            c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6fa8));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x71a8));
            c.g[S0][0] = 0x4d0000;
            c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6f98));
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[S3][0] = 0x4d0000;
            c.swc1(1, (c.g[S3][0] as u32).wrapping_sub(0x7178));
            c.g[S0][0] = 0x4d0000;
            c.g[S3][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5290));
            c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x71d8));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6f78));
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x528c));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x71c8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x71c8));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5288));
            c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5284));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x71b8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x71b8));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5280));
            c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x527c));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x71a8));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5278));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x7198));
            c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x4d0000;
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5274));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x7188));
            c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x4d0000;
            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x71c8));
            c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V1][0] = 0x4d0000;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5270));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x7168));
            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x7158));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x71b8));
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x71a8));
            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
            c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
            c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x526c));
            c.g[FP][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5268));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x7148));
            c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[FP][0] = 0x4d0000;
            c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5264));
            c.g[A0][0] = c.g[V1][0];
            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x7138));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7128));
            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[FP][0];
            c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70f8));
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5260));
            c.g[A1][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x7128));
            c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x7118));
            c.g[A0][0] = 0x4d0000;
            c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x7108));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7128));
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x70f8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[FP][0] = 0x4d0000;
            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x525c));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x7078));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x70e8));
            c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5258));
            c.g[A2][0] = c.g[A0][0];
            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x70d8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x70d8));
            c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[A0][0];
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x70b8));
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.g[V0][0];
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5254));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x70c8));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5250));
            c.g[A2][0] = 0x4d0000;
            c.swc1(4, (c.g[S0][0] as u32).wrapping_sub(0x7098));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x524c));
            c.g[S0][0] = c.g[A2][0];
            c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x70a8));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5248));
            c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
            c.g[FP][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5244));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x7098));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x70c8));
            c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x70b8));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.g[V1][0];
            c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x70a8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7078));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A1][0] = 0x4d0000;
            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A2][0] = 0x4d0000;
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5240));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x523c));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x70c8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x7088));
            c.g[FP][0] = 0x4d0000;
            c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x70b8));
            c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5238));
            c.g[A3][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x7078));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70a8));
            c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = 0x4d0000;
            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5234));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7098));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x7068));
            c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = 0x4d0000;
            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7038));
            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5230));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x7058));
            c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = 0x4d0000;
            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x7048));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6fe8));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[FP][0] = 0x4d0000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V0][0] = c.g[FP][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7018));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[A3][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7048));
            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[T2][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x522c));
            c.g[A3][0] = 0x4d0000;
            c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = c.g[T2][0];
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5228));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7028));
            c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0x4d0000;
            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = 0x4d0000;
            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x7038));
            c.g[A0][0] = c.g[V1][0];
            c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x7028));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7008));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A3][0] = c.g[A2][0];
            c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x7018));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6fe8));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[T2][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x7008));
            c.g[S0][0] = 0x4d0000;
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[FP][0] = c.g[S0][0];
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5224));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6fc8));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x6ff8));
            c.g[V0][0] = 0x4d0000;
            c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7048));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5220));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x6fe8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7038));
            c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x521c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7028));
            c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x6fd8));
            c.g[A1][0] = 0x4d0000;
            c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7018));
            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A2][0] = 0x4d0000;
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5218));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7008));
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5214));
            c.g[T2][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x6fc8));
            c.swc1(6, (c.g[FP][0] as u32).wrapping_add(0x4));
            c.swc1(7, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A2][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[S2][0] = 0x4d0000;
            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5210));
            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x520c));
            c.g[FP][0] = 0x4d0000;
            c.swc1(2, (c.g[T7][0] as u32).wrapping_add(0xc));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6f68));
            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5208));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5204));
            c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6fb8));
            c.swc1(3, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.swc1(4, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5200));
            c.g[T5][0] = 0x4d0000;
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x51fc));
            c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x6fa8));
            c.swc1(5, (c.g[T7][0] as u32).wrapping_add(0x4));
            c.swc1(6, (c.g[T7][0] as u32).wrapping_add(0x8));
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x51f8));
            c.g[T7][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x51f4));
            c.g[S0][0] = c.g[T7][0];
            c.swc1(0, (c.g[T5][0] as u32).wrapping_sub(0x6f98));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6f88));
            c.swc1(7, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[T7][0] as u32).wrapping_sub(0x6f88));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x6f78));
            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
            c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x6f68), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
            break 'b_2265cc;
        }
        c.cycles += 4;
        c.g[A0][0] = 0x4d0000;
        c.g[A1][0] = 0x2;
        c.g[RA][0] = sx(0x002265cc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6f68));
        c.pc = 0x00225b90;
        ee::s05::f_00225b90(c);
        if c.pc != 0x002265cc {
            return;
        }
    }
    c.cycles += 12;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.pc = t;
    return;
}

/// `LunoVM_StaticInitDestroy(int, uint)` (0x225de8), called by the Luno
/// static constructor and destructor.
pub(crate) const OC513_LUNO_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0022_5de8,
    f: ssx3_ee::s05::f_00225de8,
};

/// `void luno::LunoVM_GlobalCtors(void)` (0x2267f0): `LunoVM_StaticInitDestroy(1, 0xffff)`.
pub fn luno_vm_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        OC513_LUNO_STATIC_INIT_DESTROY.call(c, 0x0022_6804)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void luno::LunoVM_GlobalDtors(void)` (0x226810): `LunoVM_StaticInitDestroy(0, 0xffff)`.
pub fn luno_vm_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        OC513_LUNO_STATIC_INIT_DESTROY.call(c, 0x0022_6824)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The word at `off(base)` for a base register value (32-bit address math).
#[inline(always)]
pub(crate) fn oc640_field(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `cLunoTable::emptyTable(ref)` (0x224f68): destroys the live entries of the
/// table in reverse order (each through its vtable's slot 1, with the entry
/// adjusted by the vtable's offset half-word), frees their block with the
/// count header in front of it, then allocates `Modulus + 1` nodes (a count
/// header and the array), zeroes them, gives each the table's vtable and stores
/// the array as the table's `Entries`. A `Modulus` of -1 keeps the old array.
pub fn luno_table_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = oc640_field(c.g[V0][0], 0x10).lw(c);
        c.g[S2][0] = 0x0048_0000;
        if c.g[V1][0] != 0 {
            // Destroy the live entries, last first.
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if c.g[V1][0] != c.g[S0][0] {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                loop {
                    c.cycles += 10;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = oc640_field(c.g[S0][0], 0x1c).lw(c);
                    c.g[A0][0] = oc640_field(c.g[V0][0], 0x8).lh(c);
                    c.g[V1][0] = oc640_field(c.g[V0][0], 0xc).lw(c);
                    let dtor = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, dtor, 0x0022_4fc0)?;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V1][0] = oc640_field(c.g[V0][0], 0x10).lw(c);
                    let more = c.g[V1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                    if !more {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4fa8);
                    }
                }
            } else {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            }
            c.cycles += 4;
            c.g[A0][0] = oc640_field(c.g[V0][0], 0x10).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0022_4fe0)?; // cMemMan_free(block)
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        }
        // Allocate and initialise the new node array.
        c.cycles += 14;
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6790));
        c.g[A2][0] = 0xffff_ffff_a000_0000;
        c.g[A3][0] = 0;
        c.g[S0][0] = oc640_field(c.g[V0][0], 0xc).lw(c);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 5);
        c.g[A0][0] = c.g[A0][0] | 0x10;
        call_ptr(c, 0x0031_7e30, 0x0022_5008)?; // operator new[] (count + 1 nodes)
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        let count = c.g[S1][0];
        Addr(c.g[V0][0] as u32).sw(c, count);
        c.g[V0][0] = u64::MAX;
        let keep_old = c.g[S0][0] == c.g[V0][0];
        c.g[V1][0] = c.g[A0][0];
        if !keep_old {
            c.cycles += 3;
            c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x6770)); // the node vtable
            c.g[A1][0] = u64::MAX;
            loop {
                c.cycles += 9;
                let vtable = c.g[V0][0];
                oc640_field(c.g[V1][0], 0x1c).sw(c, vtable);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                oc640_field(c.g[V1][0], 0x8).sw(c, 0);
                oc640_field(c.g[V1][0], 0x0).sw(c, 0);
                oc640_field(c.g[V1][0], 0x14).sw(c, 0);
                oc640_field(c.g[V1][0], 0xc).sw(c, 0);
                oc640_field(c.g[V1][0], 0x18).sw(c, 0);
                let more = c.g[S0][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_5028);
                }
            }
        }
        c.cycles += 7;
        let entries_slot = c.g[S2][0] as u32;
        let entries = c.g[A0][0];
        Addr(entries_slot).sw(c, entries);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LunoValue* cLunoTable_FindInt(cLunoTable* self, int key)` (0x2250e0): looks up
/// the integer `key` in the hash table: walks the chain at `key % modulus`,
/// comparing each node's index with an int-typed (3) script value, and returns
/// the node's value, or the nil value (0x4c9098) when absent.
pub fn oc681_luno_table_find_int(c: &mut Cpu) {
    guest(c, |c| {
        'b_22511c: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S3][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A2][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.divu(0, 19, 2);
            if c.g[V0][0] == 0u64 {
                break 'b_22511c;
            } else {
                break 'b_22511c;
            }
        }
        'b_22521c: {
            'b_225214: {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.hi[0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] == 0u64;
                c.g[S6][0] = 0x3;
                if t {
                    break 'b_225214;
                } else {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[SP][0];
                    c.g[S5][0] = 0x2;
                    c.g[S4][0] = 0x1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S6][0]);
                    'l_225148: loop {
                        'b_2251f8: {
                            'b_2251f4: {
                                'b_2251ec: {
                                    'b_225170: {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00225150);
                                        c.g[A0][0] = c.g[SP][0];
                                        Callee {
                                            addr: 0x00226620,
                                            f: ee::s05::LunoVMRegister_AsId,
                                        }
                                        .call(c, 0x00225150)?;
                                        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        let t = c.g[V1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_225170;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                            let t = true;
                                            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                            if t {
                                                break 'b_2251ec;
                                            } else {
                                                break 'b_225170;
                                            }
                                        }
                                    }
                                    'b_2251a8: {
                                        c.cycles += 2;
                                        if c.g[V0][0] != c.g[S5][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_2251a8;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[S4][0] {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_2251a8;
                                            } else {
                                                'b_2251a0: {
                                                    c.cycles += 7;
                                                    c.lwc1(1, c.g[SP][0] as u32);
                                                    c.cvt_s_w(1, 1);
                                                    c.lwc1(0, c.g[S1][0] as u32);
                                                    c.c_eq_s(0, 1);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_2251a0;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0u64;
                                                        break 'b_2251a0;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S0][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_2251f4;
                                                } else {
                                                    break 'b_2251a8;
                                                }
                                            }
                                        }
                                    }
                                    'b_2251d0: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[S4][0];

                                        if t {
                                            break 'b_2251d0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V0][0] != c.g[S5][0];

                                            if t {
                                                break 'b_2251d0;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, c.g[S2][0] as u32);
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(3, 1);
                                                let t = true;
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                if t {
                                                    break 'b_2251ec;
                                                } else {
                                                    break 'b_2251d0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[S6][0];
                                    c.g[S0][0] = 0u64;
                                    if t {
                                        break 'b_2251f4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        let t = c.g[V0][0] != c.g[S4][0];
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_2251f8;
                                        } else {
                                            c.cycles += 2;
                                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                            break 'b_2251ec;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                break 'b_2251f4;
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[SP][0];
                            break 'b_2251f8;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00225200);
                        c.g[A1][0] = 0x2;
                        Callee {
                            addr: 0x00225b90,
                            f: ee::s05::LunoVMRegister_dtor,
                        }
                        .call(c, 0x00225200)?;
                        let t = c.g[S0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_22521c;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            if c.g[S1][0] != 0u64 {
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S6][0]);
                                if event_due(c) {
                                    deliver_event_here(c, 0x00225148);
                                }
                                continue 'l_225148;
                            } else {
                                break 'b_225214;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x4d0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            break 'b_22521c;
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void cLunoTable_Read(cLunoTable* self, Stream* s, void* base)` (0x2259a8):
/// deserialises a replay-frame table: destroys the old node array (virtual
/// deleters, then free), reads the 8-byte header and the entry count from the
/// stream, allocates a new node array and `set`s every (key, value) read back.
pub fn oc681_luno_table_read(c: &mut Cpu) {
    guest(c, |c| {
        'b_225a40: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = 0x480000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S2][0] = c.g[A1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            let t = c.g[V1][0] == 0u64;
            c.g[S5][0] = c.g[A2][0];
            if t {
                break 'b_225a40;
            } else {
                'b_225a34: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if c.g[V1][0] == c.g[S0][0] {
                        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                        break 'b_225a34;
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                        'l_225a08: loop {
                            c.cycles += 10;
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00225a20);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00225a20)?;
                            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                            let t = c.g[V1][0] != c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00225a08);
                                }
                                continue 'l_225a08;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                                break 'b_225a34;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x00225a40);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x00317e98, 0x00225a40)?;
                break 'b_225a40;
            }
        }
        'b_225acc: {
            c.cycles += 22;
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A2][0] = 0x8;
            c.g[A1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00225a5c);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00225a5c)?;
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6790));
            c.g[A2][0] = 0xffffffffa0000000;
            c.g[A3][0] = 0u64;
            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[S1][0] as u32) << 5);
            c.g[RA][0] = sx(0x00225a84);
            c.g[A0][0] = c.g[A0][0] | 0x10;
            call_ptr(c, 0x00317e30, 0x00225a84)?;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
            c.g[V0][0] = 0xffffffffffffffff;
            let t = c.g[S0][0] == c.g[V0][0];
            c.g[V1][0] = c.g[A0][0];
            if t {
                break 'b_225acc;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x480000;
                c.g[A1][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6770));
                'l_225aa8: loop {
                    c.cycles += 9;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                    let t = c.g[S0][0] != c.g[A1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00225aa8);
                        }
                        continue 'l_225aa8;
                    } else {
                        break 'b_225acc;
                    }
                }
            }
        }
        'b_225b68: {
            c.cycles += 12;
            c.sw(c.g[S6][0] as u32, c.g[A0][0]);
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = 0x4;
            c.g[S1][0] = 0u64;
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00225af0);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00225af0)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_225b68;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.g[SP][0];
                'l_225b00: loop {
                    c.cycles += 26;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00225b18);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00225ca0,
                        f: ee::s05::LunoVMRegister_Read,
                    }
                    .call(c, 0x00225b18)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00225b30);
                    c.g[A2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x00225ca0,
                        f: ee::s05::LunoVMRegister_Read,
                    }
                    .call(c, 0x00225b30)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00225b40);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00225338,
                        f: ee::s05::cLunoTable_Set,
                    }
                    .call(c, 0x00225b40)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00225b4c);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x00225b90,
                        f: ee::s05::LunoVMRegister_dtor,
                    }
                    .call(c, 0x00225b4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00225b58);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x00225b90,
                        f: ee::s05::LunoVMRegister_dtor,
                    }
                    .call(c, 0x00225b58)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00225b00);
                        }
                        continue 'l_225b00;
                    } else {
                        break 'b_225b68;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `LunoVMRegister* LunoVMRegister_Add(out, lhs, rhs)` (0x226628): adds two
/// script values. Same types add (int type 1, float 2, index 3); float + int and
/// int + float mix; any other pair gives the nil value. Returns `out`.
pub fn oc681_luno_vm_register_add(c: &mut Cpu) {
    guest(c, |c| {
        'b_226748: {
            'b_22672c: {
                'b_22671c: {
                    'b_2266ec: {
                        'b_2266e8: {
                            'b_2266d8: {
                                'b_2266ac: {
                                    c.cycles += 9;
                                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                    let t = c.g[A3][0] != c.g[V1][0];
                                    c.g[S1][0] = c.g[A0][0];
                                    if t {
                                        break 'b_2266ac;
                                    } else {
                                        'b_226668: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_226668;
                                            } else {
                                                c.cycles += 4;
                                                c.lwc1(20, c.g[A1][0] as u32);
                                                c.lwc1(0, c.g[A2][0] as u32);
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x8),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_2266d8;
                                                } else {
                                                    break 'b_226668;
                                                }
                                            }
                                        }
                                        'b_226684: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_226684;
                                            } else {
                                                c.cycles += 5;
                                                c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x8),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_22671c;
                                                } else {
                                                    break 'b_226684;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x4d0000;
                                        if t {
                                            break 'b_22672c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                            c.g[RA][0] = sx(0x002266a4);
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            Callee {
                                                addr: 0x00226620,
                                                f: ee::s05::LunoVMRegister_AsId,
                                            }
                                            .call(c, 0x002266a4)?;
                                            let t = true;
                                            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                                            if t {
                                                break 'b_226748;
                                            } else {
                                                break 'b_2266ac;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                    break 'b_2266e8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    if c.g[A3][0] != c.g[V0][0] {
                                        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                        break 'b_2266ec;
                                    } else {
                                        c.cycles += 5;
                                        c.lwc1(0, c.g[A2][0] as u32);
                                        c.cvt_s_w(0, 0);
                                        c.lwc1(20, c.g[A1][0] as u32);
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_2266d8;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x002266e0);
                            c.add_s(20, 20, 0);
                            Callee {
                                addr: 0x00226618,
                                f: ee::s05::LunoVMRegister_AsFloat,
                            }
                            .call(c, 0x002266e0)?;
                            let t = true;
                            c.swc1(20, c.g[V0][0] as u32);
                            if t {
                                break 'b_226748;
                            } else {
                                break 'b_2266e8;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0x1;
                        break 'b_2266ec;
                    }
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        break 'b_22672c;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = 0x2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x4d0000;
                        if t {
                            break 'b_22672c;
                        } else {
                            c.cycles += 6;
                            c.lwc1(0, c.g[A2][0] as u32);
                            c.g[A0][0] = c.g[S1][0];
                            c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                            break 'b_22671c;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x00226724);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Callee {
                    addr: 0x00226610,
                    f: ee::s05::LunoVMRegister_AsInt,
                }
                .call(c, 0x00226724)?;
                let t = true;
                c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                if t {
                    break 'b_226748;
                } else {
                    break 'b_22672c;
                }
            }
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[A2][0] as u32);
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[S1][0] as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            break 'b_226748;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::bx::bxstring::tests::alloc;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_mutex;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_machine;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;

    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    #[test]
    fn luno_vm_register_dtor_matches() {
        check(
            ssx3_ee::s05::f_00225b90__generated,
            luno_vm_register_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x225b_90 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let selfp = DATA + 0x100;
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                c.bus
                    .write32(selfp + 8, if seed % 3 == 0 { 4 } else { r.next_u32() % 4 });
                let vt = DATA + 0x800;
                c.bus.write32(selfp, vt);
                c.bus.write32(vt + 8, 2 + r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn c_luno_table_ctor_matches() {
        check(
            ssx3_ee::s05::f_00224da0__generated,
            c_luno_table_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaf, 6);
                globals(c);
                alloc_mutex(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    // ------------------------------------------------ luno / movie / cards

    #[test]
    fn cluno_table_free_data_matches() {
        check(
            ee::s05::f_00224f30__generated,
            cluno_table_free_data,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x224f_30);
                let r = DATA + 0x400;
                zero(c, r, 0x40);
                c.g[A0][0] = sx(r);
                if seed % 2 != 0 {
                    let table = heap_block(c, 0x300);
                    zero(c, table, 0x300);
                    c.bus.write32(r, table);
                } else {
                    c.bus.write32(r, 0);
                }
            },
        );
    }

    #[test]
    fn luno_vm_register_set_raw_matches() {
        leaf(
            ssx3_ee::s05::f_00226600__generated,
            luno_vm_register_set_raw,
            17,
        );
    }

    #[test]
    fn luno_vm_pass_through_matches() {
        leaf(
            ssx3_ee::s05::f_002224a8__generated,
            luno_vm_pass_through,
            18,
        );
    }

    #[test]
    fn luno_vm_register_as_float_matches() {
        leaf(
            ssx3_ee::s05::f_00226618__generated,
            luno_vm_register_as_float,
            19,
        );
    }

    #[test]
    fn luno_vm_register_as_int_matches() {
        leaf(
            ssx3_ee::s05::f_00226610__generated,
            luno_vm_register_as_int,
            20,
        );
    }

    // ------------------------------------------------------------------
    // Luno VM.

    #[test]
    fn luno_vm_register_read_ptr_matches() {
        check(
            ssx3_ee::s05::f_00224c78__generated,
            luno_vm_register_read_ptr,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x224c78, 4);
                be_library(c);
                world(c);
                let reg = DATA + 0x100;
                let stream = DATA + 0x200;
                let out = DATA + 0x300;
                c.g[A0][0] = reg as u64;
                c.g[A1][0] = stream as u64;
                c.g[A2][0] = out as u64;
                let vt = DATA + 0x400;
                w(c, stream, vt);
                slot(c, vt, 0x10, 0, FAKE);
                slot(c, vt, 0x14, 0, FAKE + 0x10);
                w(c, out, r.next_u32());
                w(c, out + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_luno_table_entry_dtor_matches() {
        check(
            ssx3_ee::s05::f_00226768__generated,
            c_luno_table_entry_dtor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x226768, 4);
                be_library(c);
                world(c);
                let entry = DATA + 0x100;
                c.g[A0][0] = entry as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                if seed % 3 == 0 {
                    w(c, entry + 0x18, 0);
                } else {
                    let child = DATA + 0x400;
                    let vt = DATA + 0x800;
                    w(c, entry + 0x18, child);
                    w(c, child + 0x1c, vt);
                    slot(c, vt, 8, 0, FAKE);
                }
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------ luno

    #[test]
    fn luno_table_dtor_matches() {
        check(
            ssx3_ee::s05::f_00224d00__generated,
            luno_table_dtor,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0022_4d00);
                heap(c);
                let table = DATA + 0x100;
                clear(c, table, 0x100);
                let n = seed % 3;
                let block = alloc(c, 0x20 * (n + 1) + 0x10);
                let block = if block == 0 { DATA + 0x2000 } else { block };
                clear(c, block, 0x20 * (n + 1) + 0x10);
                w(c, block, n);
                let vtable = DATA + 0x3000;
                clear(c, vtable, 0x40);
                c.bus.write16(vtable + 8, 0);
                w(c, vtable + 0xc, crate::difftest::fakes::FAKE);
                for i in 0..n {
                    let e = block + 0x10 + 0x20 * (n - 1 - i);
                    w(c, e + 0x1c, vtable);
                }
                w(c, table + 0x10, block + 0x10);
                c.g[A0][0] = sx(table);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    pub(crate) const LUNO_VALUE: u32 = DATA + 0x1_0000;

    pub(crate) const LUNO_STREAM: u32 = DATA + 0x1_1000;

    #[test]
    fn luno_vm_register_read_matches() {
        check_all(
            ee::s05::f_00225ca0__generated,
            luno_vm_register_read,
            30,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x225c_a0, 8);
                c.g[A0][0] = sx(LUNO_VALUE);
                c.g[A1][0] = sx(LUNO_STREAM);
                c.g[A2][0] = sx(seed % 4);
                c.bus.write32(LUNO_VALUE, r.next_u32());
                c.bus.write32(LUNO_VALUE + 0x8, (seed % 6) as u32); // old type
                if seed % 2 == 0 {
                    let vt0 = vtable(c, &mut r, &[0x10, 0x14]);
                    c.bus.write32(LUNO_STREAM, vt0);
                }
            },
        );
    }

    use crate::difftest::fakes;

    /// The 16-pool-node classes of `pool_allocator` (block >= 16) filled so
    /// a value read past the register file looks like a counted table
    /// reference (type 4 at +4/+8, a table pointer at +0/+0xc of each 16
    /// bytes), which makes the "release the old reference" branches reachable.
    pub(crate) fn luno_call_fill_pools(c: &mut Cpu, table: u32) {
        let mut nodes = DATA + 0x24_000;
        for block in [8u32, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200] {
            if block >= 0x10 {
                for n in 0..16 {
                    let node = nodes + block * n;
                    for off in (4..block).step_by(4) {
                        let v = match off % 16 {
                            4 | 8 => 4,
                            _ => table,
                        };
                        c.bus.write32(node + off, v);
                    }
                }
            }
            nodes += block * 16;
        }
    }

    /// A script whose function record has `n` registers and no code (the
    /// interpreter returns at once and frees the register file), the
    /// allocator fixtures, and the default value of each type.
    #[test]
    fn luno_vm_call_matches() {
        use crate::difftest::check;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        check(
            ee::s05::f_002224b8__generated,
            luno_vm_call,
            120,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x2224_b8);
                let state = DATA + 0x1000;
                let script = DATA + 0x1100;
                let out = DATA + 0x1200;
                let table = DATA + 0x1800; // refcount at +8
                let table2 = DATA + 0x1900;
                let gc_table = DATA + 0x1a00; // what the pool fill points at
                c.bus.write32(table + 8, 100);
                c.bus.write32(table2 + 8, 100);
                c.bus.write32(gc_table + 8, 100);
                if seed % 2 == 0 {
                    luno_call_fill_pools(c, gc_table);
                }
                // state = {script, function 0}; script+4 = code offset 0x10, the
                // record at script+0x10: {_, start pc 0, n registers, 0 locals}.
                c.bus.write32(state, script);
                c.bus.write32(state + 4, 0);
                c.bus.write32(script + 4, 0x10);
                c.bus.write32(script + 0x14, 0);
                c.bus.write32(script + 0x18, seed % 4); // n = 0..3
                c.bus.write32(script + 0x1c, 0);
                // the table argument: a cLunoValue whose word 0 is the table.
                let arg = DATA + 0x1300;
                c.bus.write32(arg, table);
                // the default value: a table reference, a pair, or a plain word.
                let d = LUNO_CALL_DEFAULT_VALUE;
                let (word0, ty) = match seed / 4 % 3 {
                    0 => (table2, 4),
                    1 => (r.next_u32(), 5),
                    _ => (r.next_u32(), 2),
                };
                c.bus.write32(d, word0);
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, ty);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(state);
                c.g[A2][0] = sx(arg);
                c.g[A3][0] = sx(DATA + 0x1400);
                c.pc = 0x0022_24b8;
            },
        );
    }

    // (fakes imported above)

    // -- 0x00225be8 LunoVMRegister_Write ---------------------------------------

    /// `LunoVMRegister_Write` against the generated body: type tags 5
    /// (the real `LunoVMRegister_WritePtr`), 4 (the real `cLunoTable_Write`
    /// over a table with no hash part and no entries) and others (payload
    /// written directly). The stream's Write virtual is a fake (some calls
    /// unwind).
    #[test]
    fn luno_vm_register_write_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const VALUE: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        const TABLE: u32 = DATA + 0x20_0200;
        covered(
            ee::s05::f_00225be8__generated,
            luno_vm_register_write,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_5be8, 3);
                c.pc = 0x0022_5be8;
                let vt = vtable(c, &mut r, &[LUNO_STREAM_WRITE]);
                c.bus.write32(STREAM, vt);
                let ty = [4, 5, 4, 5, 0, 1, 2, 3, 6][(r.next_u32() % 9) as usize];
                c.bus.write32(VALUE + LUNO_VALUE_TYPE, ty);
                c.bus
                    .write32(VALUE, if ty == 4 { TABLE } else { r.next_u32() });
                c.bus.write32(VALUE + 4, r.next_u32());
                // Table: no hash part (+0xc = -1), so nothing to count or write.
                c.bus.write32(TABLE + 0xc, u32::MAX);
                c.bus.write32(TABLE + 0x10, TABLE + 0x40);
                c.g[A0][0] = sx(VALUE);
                c.g[A1][0] = sx(STREAM);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// cLunoTable_Assign: the same table, a null old reference, an old
    /// reference that stays alive (refcount > 1) and one whose last
    /// reference is dropped (a real zeroed heap table freed by the real
    /// cLunoTable_FreeData on the pool allocator / dlmalloc arena).
    #[test]
    fn cluno_table_assign_matches() {
        use crate::game::dirtysock::tagsunk::tests::zero;
        use crate::game::fe::festaterewards::tests::heap_block;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x400;
        const OTHER: u32 = DATA + 0x440;
        const OLD: u32 = DATA + 0x800;
        const NEW: u32 = DATA + 0x900;
        check(
            ee::s05::f_00225068__generated,
            cluno_table_assign,
            120,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x22_5068);
                c.pc = 0x0022_5068;
                zero(c, SELF, 0x80);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OTHER);
                c.bus.write32(NEW + LTA_REFCOUNT, r.next_u32() % 50);
                c.bus.write32(OTHER, NEW);
                match seed % 4 {
                    0 => c.bus.write32(SELF, NEW),
                    1 => c.bus.write32(SELF, 0),
                    2 => {
                        c.bus.write32(SELF, OLD);
                        c.bus.write32(OLD + LTA_REFCOUNT, 2 + r.next_u32() % 50);
                    }
                    _ => {
                        let table = heap_block(c, 0x300);
                        zero(c, table, 0x300);
                        c.bus.write32(table + LTA_REFCOUNT, 1);
                        c.bus.write32(SELF, table);
                    }
                }
            },
        );
    }

    // ------------------------------------------------------------ cLunoTable_Write

    /// Tables without a hash part, or with 1..4 buckets whose nodes and
    /// chains (0..3 links) hold nil and non-nil values; plain register
    /// types only (the real `LunoVMRegister_Write` writes them directly).
    /// The stream's Write is a fake (some calls unwind).
    #[test]
    fn c_luno_table_write_matches() {
        covered(
            ee::s05::f_002257e0__generated,
            c_luno_table_write,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_57e0, 8);
                c.pc = 0x0022_57e0;
                let this = DATA + 0x5_0000;
                let stream = DATA + 0x5_0100;
                let table = DATA + 0x5_0200;
                let nodes = DATA + 0x5_1000;
                let links = DATA + 0x5_2000;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.bus.write32(this, table);
                let modulus = [-1i32, 0, 1, 3][(seed % 4) as usize];
                c.bus.write32(table + LUNO_TABLE_MODULUS, modulus as u32);
                c.bus.write32(table + LUNO_TABLE_NODES, nodes);
                let types = [0u32, 0, 1, 2, 3, 6];
                let node = |c: &mut Cpu, r: &mut Rng, at: u32, next: u32| {
                    c.bus.write32(at, r.next_u32());
                    c.bus
                        .write32(at + 8, types[r.next_u32() as usize % types.len()]);
                    c.bus.write32(at + LUNO_NODE_VALUE, r.next_u32());
                    c.bus.write32(
                        at + LUNO_NODE_VALUE_TYPE,
                        types[r.next_u32() as usize % types.len()],
                    );
                    c.bus.write32(at + LUNO_NODE_NEXT, next);
                };
                let mut link = links;
                for b in 0..4u32 {
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 4 {
                        node(c, &mut r, link, next);
                        next = link;
                        link += LUNO_NODE_SIZE;
                    }
                    node(c, &mut r, nodes + LUNO_NODE_SIZE * b, next);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// cLunoTable_dtor: a heap-allocated reference to a heap-allocated table
    /// with no data. Refcount 1 (last reference: the table is freed) or more,
    /// and `flags & 1` (delete the reference) on or off.
    #[test]
    fn c_luno_table_dtor_matches() {
        check(
            ee::s05::f_00224df0__generated,
            c_luno_table_dtor,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x224df0, 0);
                be_library(c);
                world(c);
                heap(c);
                let this = alloc(c, 0x10);
                let table = alloc(c, 0x20);
                for o in (0..0x20).step_by(4) {
                    c.bus.write32(table + o, 0);
                }
                c.bus.write32(this, table);
                let refs = if seed % 2 == 0 {
                    1
                } else {
                    2 + r.next_u32() % 5
                };
                c.bus.write32(table + 0x8, refs);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ((r.next_u32() & !1) | (seed >> 1) & 1) as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0022_4df0;
            },
        );
    }

    /// LunoVMRegister_WritePtr: a register at DATA+0x200000, a stream whose vtable's
    /// Write slot is a fake (logs both calls' registers and the stacked
    /// words; some unwind).
    #[test]
    fn luno_vm_register_write_ptr_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(
            ee::s05::f_00224c00__generated,
            luno_vm_register_write_ptr,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x224c00, 2);
                c.pc = 0x0022_4c00;
                // (Away from the vtable's random spot in DATA..DATA+0xfffff.)
                let reg = DATA + 0x20_0000;
                let out = DATA + 0x20_0100;
                c.bus.write32(reg, r.next_u32());
                c.bus.write32(reg + 4, r.next_u32());
                let vt = vtable(c, &mut r, &[LVW_WRITE_SLOT]);
                c.bus.write32(out, vt);
                c.g[A0][0] = reg as u64;
                c.g[A1][0] = out as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::put_words;
    use crate::game::worldpainter::quadtree::tests::alloc_machine as alloc_machine_m2;
    use crate::game::worldpainter::quadtree::tests::run;

    /// The game's opcode table (`D_004797B0`).
    pub(crate) const LVX_T_JUMPS: [u32; 43] = [
        0x222b44, 0x222a60, 0x222ad0, 0x222dd4, 0x222ee4, 0x222ff4, 0x223118, 0x223238, 0x22335c,
        0x22347c, 0x22353c, 0x2235e0, 0x223644, 0x2237fc, 0x22398c, 0x223b1c, 0x223e08, 0x223cd0,
        0x224030, 0x2240c0, 0x224154, 0x2241b0, 0x222a04, 0x2229a0, 0x224660, 0x2246ec, 0x224b2c,
        0x223ee0, 0x224070, 0x224214, 0x224764, 0x2247f8, 0x2228f0, 0x223f58, 0x222b4c, 0x222c98,
        0x22428c, 0x224878, 0x2248e0, 0x224958, 0x2249d0, 0x224a3c, 0x224ab0,
    ];

    pub(crate) const LVX_T_VM: u32 = DATA + 0x2000;

    pub(crate) const LVX_T_FUNC: u32 = DATA + 0x2010;

    pub(crate) const LVX_T_RESULT: u32 = DATA + 0x2020;

    /// The counted table every type-4 value points at (refcount 1000: never
    /// freed).
    pub(crate) const LVX_T_TABLE: u32 = DATA + 0x2040;

    pub(crate) const LVX_T_CODE: u32 = DATA + 0x3000;

    pub(crate) const LVX_T_NREGS: u32 = 16;

    pub(crate) fn lvx_op(o: u32, a: u32, b: u32, cc: u32) -> u32 {
        o | a << 8 | b << 16 | cc << 24
    }

    /// A `cLunoVar` of type `ty` (a table reference points at the test table).
    pub(crate) fn lvx_t_value(c: &mut Cpu, r: &mut Rng, at: u32, ty: u32) {
        let w0 = if ty == 4 { LVX_T_TABLE } else { r.next_u32() };
        c.bus.write32(at, w0);
        c.bus.write32(at + 4, r.next_u32());
        c.bus.write32(at + 8, ty);
    }

    pub(crate) fn lvx_t_any_type(r: &mut Rng) -> u32 {
        r.next_u32() % 7
    }

    /// A random main function. Registers: R0 a table reference (the callee's
    /// globals), R1..R8 scratch destinations, R9 a small int key, R10 a new
    /// table (prologue), R11 a function (prologue: function 1), R12..R14 the
    /// for-loop counter, limit and step, R15 nil. Jumps go forward to
    /// statement starts only; argument pushes and their C call form one
    /// statement (never split by a jump), and a for loop (its loads, a body
    /// and the loop op) one too, so every loop starts from small constants.
    pub(crate) fn lvx_main_program(r: &mut Rng, nargs: u32, cfuncs: u32) -> Vec<u32> {
        let mut stmts: Vec<Vec<u32>> =
            vec![vec![lvx_op(25, 10, 0, 0)], vec![lvx_op(29, 11, 0, 0), 1]];
        // (statement, word, target in the high half (else the word), target
        // statement, word offset inside it)
        let mut fix: Vec<(usize, usize, bool, usize, usize)> = Vec::new();
        let mut left = 0u32;
        let n = 8 + r.next_u32() % 16;
        for _ in 0..n {
            let i = stmts.len();
            let dst = 1 + r.next_u32() % 8;
            let b = r.next_u32() % 16;
            let cc = r.next_u32() % 16;
            match r.next_u32() % 18 {
                0..=4 => stmts.push(vec![lvx_op(3 + r.next_u32() % 15, dst, b, cc)]),
                5 => {
                    let o = [34, 35, 11, 19][(r.next_u32() % 4) as usize];
                    stmts.push(vec![lvx_op(o, dst, b, 0)]);
                }
                6 => {
                    let o = 20 + r.next_u32() % 4;
                    let v = match o {
                        23 => r.edge_f32(),
                        _ if r.next_u32() % 2 == 0 => r.next_u32() % 8,
                        _ => r.next_u32(),
                    };
                    stmts.push(vec![lvx_op(o, dst, 0, 0), v]);
                }
                7 => stmts.push(vec![
                    lvx_op(22 - r.next_u32() % 2, 9, 0, 0),
                    r.next_u32() % 4,
                ]),
                8 => stmts.push(vec![lvx_op(30, dst, 0, 0)]),
                9 => {
                    let o = r.next_u32() % 3;
                    fix.push((i, 0, true, i + 1 + (r.next_u32() % 3) as usize, 0));
                    stmts.push(vec![lvx_op(o, b, 0, 0)]);
                }
                10 => stmts.push(vec![match r.next_u32() % 3 {
                    0 => lvx_op(18, 10, 9, b),
                    1 => lvx_op(24, dst, 10, 9),
                    _ => lvx_op(28, 10, 9, b),
                }]),
                11 => {
                    let room = nargs - left;
                    let pushes = r.next_u32() % (room.min(4) + 1);
                    let mut s = Vec::new();
                    for _ in 0..pushes {
                        let a = r.next_u32() % 256;
                        match r.next_u32() % 6 {
                            0 => s.push(lvx_op(32, a, r.next_u32() % 16, 0)),
                            1 => s.extend([lvx_op(37, a, 0, 0), r.next_u32()]),
                            2 => s.extend([lvx_op(38, a, 0, 0), r.edge_f32()]),
                            3 => s.extend([lvx_op(39, a, 0, 0), r.next_u32()]),
                            4 => s.push(lvx_op(40, a, r.next_u32() % 256, 0)),
                            _ => s.push(lvx_op(41, a, r.next_u32() % 256, 0)),
                        }
                    }
                    let pops = r.next_u32() % (pushes + 1);
                    left += pushes - pops;
                    s.push(lvx_op(33, dst, r.next_u32() % cfuncs, pops));
                    stmts.push(s);
                }
                12 => {
                    let f = [11, 11, 15][(r.next_u32() % 3) as usize];
                    stmts.push(vec![lvx_op(27, dst, f, cc)]);
                }
                13 => {
                    // R12 = start; R13 = limit; R14 = step; body; for.
                    let int_counter = r.next_u32() % 2 == 0;
                    let down = r.next_u32() % 2 == 0;
                    let start = (r.next_u32() % 4) as i32 - 1;
                    let limit = (r.next_u32() % 4) as i32 - 1;
                    let mut s = if int_counter {
                        vec![lvx_op(22, 12, 0, 0), start as u32]
                    } else {
                        vec![lvx_op(23, 12, 0, 0), (start as f32).to_bits()]
                    };
                    // Two floats compare as their bit patterns (signed ints),
                    // which inverts the order of negative floats: a float
                    // counter counting down past a negative float limit
                    // never stops. Float limits stay >= 0.
                    s.extend(match r.next_u32() % 5 {
                        0 | 1 => [lvx_op(22, 13, 0, 0), limit as u32],
                        2 | 3 => [lvx_op(23, 13, 0, 0), (limit.max(0) as f32).to_bits()],
                        _ => [lvx_op(20, 13, 0, 0), limit as u32],
                    });
                    let sign = if down { -1.0f32 } else { 1.0 };
                    s.extend(match r.next_u32() % 4 {
                        0 => [lvx_op(22, 14, 0, 0), sign as i32 as u32],
                        1 if int_counter => [lvx_op(23, 14, 0, 0), (1.5 * sign).to_bits()],
                        1 => [lvx_op(23, 14, 0, 0), (0.5 * sign).to_bits()],
                        2 => [lvx_op(20, 14, 0, 0), 1],
                        _ if int_counter => [lvx_op(22, 14, 0, 0), sign as i32 as u32],
                        _ => [lvx_op(23, 14, 0, 0), (0.5 * sign).to_bits()],
                    });
                    s.push(lvx_op(3 + r.next_u32() % 15, dst, b, cc));
                    s.extend([lvx_op(36, 12, 13, 14), 0]);
                    // The target word (8) = the body (word 6).
                    fix.push((i, 8, false, i, 6));
                    stmts.push(s);
                }
                14 => stmts.push(vec![(43 + r.next_u32() % 213) | r.next_u32() << 8]),
                15 => stmts.push(vec![lvx_op(26, dst, b, cc)]),
                _ => stmts.push(vec![lvx_op(3 + r.next_u32() % 15, dst, b, cc)]),
            }
        }
        match r.next_u32() % 3 {
            0 => stmts.push(vec![lvx_op(31, r.next_u32() % 16, 0, 0)]),
            1 => stmts.push(vec![lvx_op(42, 0, 0, 0)]),
            _ => {}
        }
        // The main function follows the callee's two words.
        let mut starts = Vec::new();
        let mut at = 2usize;
        for s in &stmts {
            starts.push(at);
            at += s.len();
        }
        starts.push(at);
        for (si, wi, high, t, off) in fix {
            let target = (starts[t.min(stmts.len())] + off) as u32;
            if high {
                stmts[si][wi] |= target << 16;
            } else {
                stmts[si][wi] = target;
            }
        }
        stmts.concat()
    }

    /// `LunoVM_Execute` on random scripts: function 1 (`R2 = R1 + R1; return
    /// R2`) is the callee of op 27, the main function (function 0) is
    /// [`lvx_main_program`] over registers of random types (and table
    /// references to a counted table), with a random number of argument
    /// slots (none in some seeds), an empty main function in some, and
    /// `gLunoNil` a table reference or pair in some (to reach the counted and
    /// pair copies of nil). The C functions: a fake and the real
    /// `LunoVMRegister_SetRaw` (only the fake when the temporary starts as a
    /// table reference). The register file comes from the real allocator,
    /// as `LunoVM_Call` allocates it.
    #[test]
    fn luno_vm_execute_matches() {
        check_lohi(
            ee::s05::f_002227d0__generated,
            luno_vm_execute,
            240,
            &|c, seed| {
                let mut r = alloc_machine_m2(c, seed, 0x2227_d0);
                put_words(c, LVX_JUMP_TABLE, &LVX_T_JUMPS);
                c.bus.write32(LVX_T_TABLE + 8, 1000);
                let nil_ty = [0, 0, 0, 0, 4, 5][(r.next_u32() % 6) as usize];
                lvx_t_value(c, &mut r, LVX_NIL, nil_ty);
                // The temporary at the interpreter's sp.
                let temp_ty = r.next_u32() % 6;
                lvx_t_value(c, &mut r, crate::difftest::SP - 0x140, temp_ty);
                let cfuncs = if temp_ty == 4 { 1 } else { 2 };
                put_words(c, LVX_C_FUNCTIONS, &[FAKE, 0x0022_6600]);

                // The register file: a count header, then the values.
                let size = (LVX_T_NREGS * 12 + 0x10) as u64;
                let block = run(
                    c,
                    LVX_OPERATOR_NEW,
                    ee::s10::f_00317e30,
                    &[size, 0x0047_9790, 0x2000_0000, 0],
                ) as u32;
                c.bus.write32(block, LVX_T_NREGS);
                let regs = block + 0x10;
                for i in 0..LVX_T_NREGS {
                    let at = regs + 12 * i;
                    match i {
                        0 => lvx_t_value(c, &mut r, at, 4),
                        9 => {
                            lvx_t_value(c, &mut r, at, 1);
                            c.bus.write32(at, r.next_u32() % 4);
                        }
                        15 => lvx_t_value(c, &mut r, at, 0),
                        _ => {
                            let ty = lvx_t_any_type(&mut r);
                            lvx_t_value(c, &mut r, at, ty);
                        }
                    }
                }
                c.bus.write32(LVX_T_VM, regs);

                // The code: header, ops, function records.
                let nargs = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 8
                };
                let mut words = vec![lvx_op(12, 2, 1, 1), lvx_op(31, 2, 0, 0)];
                words.extend(lvx_main_program(&mut r, nargs, cfuncs));
                let total = words.len() as u32;
                let ft = 0x10 + 4 * total;
                c.bus.write32(LVX_T_CODE + 4, ft);
                // (A temporary that starts as a table reference holds the code
                // pointer after op 29: a harmless "refcount" here.)
                c.bus.write32(LVX_T_CODE + 8, 1000);
                put_words(c, LVX_T_CODE + 0x10, &words);
                let start = if seed % 13 == 0 { total } else { 2 };
                put_words(
                    c,
                    LVX_T_CODE + ft,
                    &[0, start, LVX_T_NREGS, nargs, 0, 0, LVX_T_NREGS, 0],
                );
                c.bus.write32(LVX_T_FUNC, LVX_T_CODE);
                c.bus.write32(LVX_T_FUNC + 4, 0);
                let result_ty = lvx_t_any_type(&mut r);
                lvx_t_value(c, &mut r, LVX_T_RESULT, result_ty);

                c.g[A0][0] = sx(LVX_T_VM);
                c.g[A1][0] = sx(LVX_T_FUNC);
                c.g[A2][0] = sx(LVX_T_RESULT);
                c.pc = 0x0022_27d0;
            },
        );
    }

    pub(crate) use crate::game::seg::seg_1fbe38::tests::FK_MEM_ALLOC;

    pub(crate) use crate::game::ai::rider::tests::oc417_force;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc417_machine;

    // ----------------------------------------------------------------------
    // cLunoTable_Set

    /// A one to four bucket table with chains: keys of every type (0 is
    /// ignored), a missing value type (delete) or a value of each type,
    /// matching and non-matching chain keys, an empty or occupied bucket and
    /// the append of a new node (allocation, table-reference assignment and
    /// the node's destructor are faked).
    #[test]
    fn c_luno_table_set_matches() {
        const SELF: u32 = DATA + 0x1000;
        const TABLE: u32 = DATA + 0x1100;
        const NODES: u32 = DATA + 0x1200;
        const CHAIN: u32 = DATA + 0x1600;
        const KEY: u32 = DATA + 0x1040;
        const VAL: u32 = DATA + 0x1080;
        const NODE_VT: u32 = DATA + 0x1800;
        const NEW_NODE: u32 = DATA + 0x1a00;
        check(
            ee::s05::f_00225338__generated,
            c_luno_table_set,
            300,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_000c, 6);
                c.pc = 0x0022_5338;
                oc417_force(c, FK_MEM_ALLOC, NEW_NODE);
                let modulus = 1 + seed % 4;
                c.bus.write32(SELF, TABLE);
                c.bus.write32(TABLE + 0xc, modulus);
                c.bus.write32(TABLE + 0x10, NODES);
                c.bus.write16(NODE_VT + 8, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(NODE_VT + 0xc, fakes::FAKE);
                let key_type = if seed % 13 == 0 {
                    0
                } else {
                    1 + (seed / 2) % 5
                };
                let key_word = r.next_u32() % 16;
                c.bus.write32(KEY, key_word);
                c.bus.write32(KEY + 4, r.next_u32() % 4);
                c.bus.write32(KEY + 8, key_type);
                let val_type = (seed / 7) % 6;
                c.bus.write32(VAL, r.next_u32());
                c.bus.write32(VAL + 4, r.next_u32());
                c.bus.write32(VAL + 8, val_type);
                let bucket = NODES + 0x20 * (key_word % modulus);
                let occupied = (seed / 3) % 3 != 0;
                let same_key = (seed / 5) % 2 == 0;
                // Node layout: key (+0, type at +8), value (+0xc, type at +0x14),
                // next (+0x18), vtable (+0x1c).
                let put_node =
                    |c: &mut Cpu, at: u32, key_same: bool, vtype: u32, next: u32, r: &mut Rng| {
                        let word = if key_same {
                            key_word
                        } else {
                            key_word.wrapping_add(1 + r.next_u32() % 3)
                        };
                        c.bus.write32(at, word);
                        let key_hi = c.bus.read32(KEY + 4);
                        c.bus.write32(at + 4, key_hi);
                        c.bus.write32(at + 8, key_type);
                        c.bus.write32(at + 0xc, r.next_u32());
                        c.bus.write32(at + 0x10, r.next_u32());
                        c.bus.write32(at + 0x14, vtype);
                        c.bus.write32(at + 0x18, next);
                        c.bus.write32(at + 0x1c, NODE_VT);
                    };
                let chain_len = (seed / 11) % 3;
                let first_chain = if chain_len == 0 { 0 } else { CHAIN };
                let bucket_type = if occupied { 1 + r.next_u32() % 5 } else { 0 };
                put_node(c, bucket, same_key, bucket_type, first_chain, &mut r);
                for k in 0..chain_len {
                    let at = CHAIN + 0x20 * k;
                    let next = if k + 1 < chain_len { at + 0x20 } else { 0 };
                    let chain_same = (seed / 17 + k) % 2 == 0;
                    put_node(c, at, chain_same, 1 + r.next_u32() % 5, next, &mut r);
                }
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = KEY as u64;
                c.g[A2][0] = VAL as u64;
            },
        );
    }

    use crate::difftest::GAME_GP;

    // ------------------------------------------------------- cLunoTable_Alloc

    /// `cLunoTable_Alloc` against the real allocator (pool + arena): zero
    /// to six nodes.
    #[test]
    fn cluno_table_alloc_matches() {
        use crate::difftest::elf_image;
        check(
            ee::s05::f_00224e50__generated,
            cluno_table_alloc,
            14,
            &|c, seed| {
                machine(c, seed, 0x22_4e50, 0);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9_0000);
                c.bus.write32(GAME_GP - 0x848, DATA + 0xa_0000);
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.pc = 0x0022_4e50;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = [0u64, 1, 2, 3, 6][(seed % 5) as usize];
            },
        );
    }

    use crate::difftest::scramble;

    /// A 32-bit word of test data.
    pub(crate) fn oc428_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// One `cLunoTable_Find` fixture: a table of `m` buckets in `DATA +
    /// 0x1000`, each with a chain of up to 3 nodes; keys of every type pair
    /// (equal types, int key / float node, float key / int node, type 3,
    /// others), some matching in value, some not; some empty buckets.
    pub(crate) fn oc428_luno_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x225248 ^ (seed as u64) << 4);
        scramble(c, &mut r);
        let (vm, table, nodes, key) = (DATA + 0x100, DATA + 0x200, DATA + 0x1000, DATA + 0x300);
        let m = 1 + r.next_u32() % 4;
        c.g[A0][0] = sx(vm);
        c.g[A1][0] = sx(key);
        oc428_w(c, vm, table);
        oc428_w(c, table + LUNO_TABLE_MODULUS, m);
        oc428_w(c, table + LUNO_TABLE_NODES, nodes);
        let key_type = [1u32, 2, 3, 1, 2, 0, 5][(r.next_u32() % 7) as usize];
        let small = r.next_u32() % 6;
        let key_word = if key_type == 2 {
            (small as f32).to_bits().wrapping_add(r.next_u32() % 2)
        } else {
            small
        };
        oc428_w(c, key, key_word);
        oc428_w(c, key + 8, key_type);
        let bucket = key_word % m;
        // Chain: the bucket node, then up to two spare nodes.
        let mut node = nodes + bucket * 0x20;
        let len = 1 + r.next_u32() % 3;
        for i in 0..len {
            let node_type = [1u32, 2, 3, 1, 2, 0, 5][(r.next_u32() % 7) as usize];
            let word = match r.next_u32() % 4 {
                0 => small,
                1 => (small as f32).to_bits(),
                2 => key_word,
                _ => r.next_u32(),
            };
            let value_type = if seed % 9 == 0 && i == 0 {
                0
            } else {
                1 + r.next_u32() % 4
            };
            oc428_w(c, node, word);
            oc428_w(c, node + LUNO_NODE_KEY_TYPE, node_type);
            oc428_w(c, node + LUNO_NODE_VALUE, r.next_u32());
            oc428_w(c, node + LUNO_NODE_VALUE_TYPE, value_type);
            let next = nodes + (m + i) * 0x20;
            oc428_w(c, node + LUNO_NODE_NEXT, if i + 1 < len { next } else { 0 });
            node = next;
        }
    }

    /// `cLunoTable_Find` against the generated body.
    #[test]
    fn c_luno_table_find_matches() {
        check(
            ee::s05::f_00225248__generated,
            c_luno_table_find,
            600,
            &oc428_luno_setup,
        );
    }

    pub(crate) use crate::game::animation::gameanimmap::tests::oc438_leaf;

    #[test]
    fn luno_vm_register_as_id_matches() {
        oc438_leaf(
            ee::s05::f_00226620__generated,
            luno_vm_register_as_id,
            0x0022_6620,
        );
    }

    /// Both branches of `priority`, `initialize` and the nil construction
    /// (the real `LunoVMRegister_dtor` runs on the zeroed fixture).
    #[test]
    fn luno_static_init_destroy_matches() {
        check(
            ee::s05::f_00225de8__generated,
            luno_static_init_destroy,
            60,
            &|c, seed| {
                machine(c, seed, 0x225de8, 0);
                c.pc = 0x0022_5de8;
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0x1234 } else { 0xffff };
            },
        );
    }

    #[test]
    fn luno_vm_global_ctors_dtors_match() {
        for (gen, hand, salt) in [
            (
                ee::s05::f_002267f0__generated as fn(&mut Cpu),
                luno_vm_global_ctors as fn(&mut Cpu),
                0x67f0,
            ),
            (ee::s05::f_00226810__generated, luno_vm_global_dtors, 0x6810),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the fakes below: the magic word, then the flags, then the
    /// call counter.
    pub(crate) const OC640_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC640_MAGIC: u32 = 0x6f63_3634;

    /// The table's `ref`, the table, its entries (with the count in front)
    /// and their vtable.
    pub(crate) const OC640_REF: u32 = DATA + 0x1000;

    pub(crate) const OC640_TABLE: u32 = DATA + 0x2000;

    pub(crate) const OC640_ENTRIES: u32 = DATA + 0x3010;

    pub(crate) const OC640_VT: u32 = DATA + 0x4000;

    /// Where the gated allocator returns the new array (DATA+0x200000 and up
    /// is free of the fixtures).
    pub(crate) const OC640_ALLOC: u32 = DATA + 0x20_0000;

    pub(crate) static OC640_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC640_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// The gated `operator new[]` (0x317e30): logs the call and returns the
    /// fixture array when the gate is set.
    pub(crate) fn oc640_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC640_CTRL) != OC640_MAGIC {
            return fakes::fall_back(c, &OC640_PREV_ALLOC, ee::s10::f_00317e30__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e30, ra);
        if c.pc == ra {
            c.g[V0][0] = OC640_ALLOC as u64;
        }
    }

    /// The gated `cMemMan_free` (0x317e98): logs the call.
    pub(crate) fn oc640_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC640_CTRL) != OC640_MAGIC {
            return fakes::fall_back(c, &OC640_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
    }

    pub(crate) fn oc640_install_luno_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc640_fake_alloc,
            &OC640_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc640_fake_free,
            &OC640_PREV_FREE,
        );
    }

    /// `cLunoTable::emptyTable` against the generated body: a `Modulus` of -1
    /// (the old array is kept), a null `Entries` (nothing to destroy), zero
    /// to a few live entries (the vtable calls are fakes at a miss address),
    /// and the allocator and free fakes; the allocation is the fixture array.
    #[test]
    fn luno_table_clear_matches() {
        oc640_install_luno_fakes();
        fakes::covered(
            ee::s05::f_00224f68__generated,
            luno_table_clear,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x224f68, 6);
                c.bus.write32(OC640_CTRL, OC640_MAGIC);
                c.bus.write32(OC640_CTRL + 4, 0);
                let live = if seed % 5 == 0 { 0 } else { r.next_u32() % 4 };
                let modulus = if seed % 7 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 4
                };
                c.bus.write32(OC640_REF, OC640_TABLE);
                c.bus.write32(OC640_TABLE + 0xc, modulus);
                c.bus.write32(OC640_TABLE + 0x10, OC640_ENTRIES);
                if live == 0 && seed % 3 == 0 {
                    c.bus.write32(OC640_TABLE + 0x10, 0);
                }
                c.bus.write32(OC640_ENTRIES - 0x10, live);
                c.bus.write16(OC640_VT + 8, 0);
                c.bus.write32(OC640_VT + 0xc, fakes::FAKE + 0x20);
                for i in 0..live {
                    c.bus.write32(OC640_ENTRIES + 0x20 * i + 0x1c, OC640_VT);
                }
                c.g[A0][0] = sx(OC640_REF);
                c.pc = 0x0022_4f68;
            },
        );
    }
}
