//! Staging module oc592 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc592`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_11a0, intersect_info_ctor_4, "IntersectInfo_ctor_4"),
    (
        0x0036_1208,
        intersect_info_get_neg_normal,
        "IntersectInfo_GetNegNormal",
    ),
    (0x0036_1248, intersect_info_ctor_5, "IntersectInfo_ctor_5"),
    (0x0036_1300, intersect_info_ctor_6, "IntersectInfo_ctor_6"),
    (0x0036_1500, conveyor_node_ctor, "Conveyor_v01"),
    (0x0036_1530, counter_node_ctor, "Counter_v01"),
    (0x0036_1cd8, roller_modifier_v10, "RollerModifier_v10"),
    (
        0x0036_1e10,
        world_objects_global_ctors,
        "WorldObjects_GlobalCtors",
    ),
    (0x0036_1f40, intc_table_shutdown, "IntcTable_Shutdown"),
    (0x0036_2120, intc_handler_save_regs, "IntcHandler_SaveRegs"),
    (
        0x0036_2230,
        intc_handler_restore_regs,
        "IntcHandler_RestoreRegs",
    ),
    (
        0x003a_6cd8,
        world_get_object_section_data_2,
        "World_GetObjectSectionData_2",
    ),
    (0x003a_7010, stream_man_cancel, "cStreamMan_Cancel"),
    (
        0x003a_8230,
        world_cache_free_section_table,
        "cWorldCache_FreeSectionTable",
    ),
];

// Calls made with `call_ptr` (the original's `call_indirect`).

/// `cAnimFrame_init(this)`.
const CANIMFRAME_INIT_ADDR: u32 = 0x0035_13d0;
/// Base constructor of the conveyor and counter nodes.
const NODE_BASE_CTOR_ADDR: u32 = 0x0034_fbf0;
/// Called by the roller modifier's collide wrapper.
const ROLLER_CALL_ADDR: u32 = 0x0035_dde8;
/// Called by `cStreamMan_Cancel` with the stream's callback arguments.
const STREAM_CANCEL_CALL_ADDR: u32 = 0x003e_0e90;

// Direct calls (the original's `jal`).

const INTERSECT_RESET_MATRIX: Callee = Callee {
    addr: 0x0036_12c0,
    f: ee::s12::f_003612c0,
};
const WORLD_OBJECTS_STATIC_INIT: Callee = Callee {
    addr: 0x0035_fe10,
    f: ee::s12::f_0035fe10,
};
const INTC_REMOVE_VIF_VU_HANDLERS: Callee = Callee {
    addr: 0x0036_1e80,
    f: ee::s12::f_00361e80,
};
const VRAM_HEAP_FREE_BLOCK: Callee = Callee {
    addr: 0x0036_63d8,
    f: ee::s12::f_003663d8,
};
const WORLD_MEMMAN_FREE_SECTION_ALLOCATOR: Callee = Callee {
    addr: 0x003a_7b98,
    f: ee::s13::f_003a7b98,
};
const SECTION_TABLE_DTOR: Callee = Callee {
    addr: 0x003a_7d80,
    f: ee::s13::f_003a7d80,
};

/// The `cAnimFrame` vtable the constructors store at `this+0x84` (0x490af0),
/// and the `cMeshAnimFrame`/`cCamAnimFrame` variants' words (0x490ad0, 0x490ab0).
const ANIMFRAME_VTBL: u32 = 0x0049_0af0;
const MESHANIMFRAME_VTBL: u32 = 0x0049_0ad0;
const CAMANIMFRAME_VTBL: u32 = 0x0049_0ab0;

/// `IntersectInfo` (`cAnimFrame`-based) constructor (0x3611a0): stores the
/// vtable word at `this+0x84`, runs `cAnimFrame_init(this)`, returns `this`.
pub fn intersect_info_ctor_4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        abi::set_word(c, V0, ANIMFRAME_VTBL);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let vt = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x84).sw(c, vt);
        call_ptr(c, CANIMFRAME_INIT_ADDR, 0x0036_11c0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cAnimFrame_getAngles` (0x361208): writes the negated normal as a
/// quadword `(-f(+0x50), -f(+0x54), -f(+0x58), 0)` to the output at a1 (the
/// three floats go through the stack frame first, as the original does).
pub fn intersect_info_get_neg_normal(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x10);
    Addr(frame.at(c, 0xc)).sw(c, 0);
    let src = Addr(abi::arg(c, 0));
    let nx = -src.off(0x50).read_fpu(c);
    let ny = -src.off(0x54).read_fpu(c);
    let nz = -src.off(0x58).read_fpu(c);
    abi::set_fpr(c, 1, nx);
    abi::set_fpr(c, 2, ny);
    abi::set_fpr(c, 0, nz);
    Addr(frame.at(c, 0)).write_fpu(c, nx);
    Addr(frame.at(c, 4)).write_fpu(c, ny);
    Addr(frame.at(c, 8)).write_fpu(c, nz);
    frame.restore(c, V0, 0);
    let out = abi::arg(c, 1);
    c.sq(V0, out);
    frame.close(c);
}

/// `cMeshAnimFrame` constructor (0x361248): the `cAnimFrame` init, then the
/// mesh vtable word, then `IntersectInfo_ResetMatrix(this)`. Returns `this`.
pub fn intersect_info_ctor_5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        abi::set_word(c, V0, ANIMFRAME_VTBL);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let vt = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x84).sw(c, vt);
        call_ptr(c, CANIMFRAME_INIT_ADDR, 0x0036_1268)?;
        abi::set_word(c, V0, MESHANIMFRAME_VTBL);
        c.g[A0][0] = c.g[S0][0];
        let vt2 = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x84).sw(c, vt2);
        INTERSECT_RESET_MATRIX.call(c, 0x0036_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cCamAnimFrame` constructor (0x361300): the `cAnimFrame` init, then the
/// camera vtable word (left in v1) stored at `this+0x84`. Returns `this`.
pub fn intersect_info_ctor_6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        abi::set_word(c, V0, ANIMFRAME_VTBL);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let vt = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x84).sw(c, vt);
        call_ptr(c, CANIMFRAME_INIT_ADDR, 0x0036_1320)?;
        abi::set_word(c, V1, CAMANIMFRAME_VTBL);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        let vt2 = c.g[V1][0];
        Addr(c.g[S0][0] as u32).off(0x84).sw(c, vt2);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Shared shape of the conveyor and counter node constructors: store the
/// node's vtable word at `this+0xc` (`this` is kept in v1), then call the
/// base constructor with a0 as it came in.
fn node_ctor(c: &mut Cpu, vtbl: u32, ret_addr: u32) -> crate::call::Flow {
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    abi::set_word(c, V0, vtbl);
    let this = c.g[V1][0] as u32;
    let vt = c.g[V0][0];
    Addr(this).off(0xc).sw(c, vt);
    call_ptr(c, NODE_BASE_CTOR_ADDR, ret_addr)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cConveyorNode` constructor (0x361500).
pub fn conveyor_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        node_ctor(c, 0x0049_0570, 0x0036_151c)
    })
}

/// `cCounterNode` constructor (0x361530).
pub fn counter_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        node_ctor(c, 0x0049_03f0, 0x0036_154c)
    })
}

/// `cRollerModifier::collide` stub (0x361cd8): clears the word at +0x20 of
/// the object, then calls the function at 0x35dde8 with the object as a0.
pub fn roller_modifier_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let obj = c.g[V0][0] as u32;
        Addr(obj).off(0x20).sw(c, 0);
        call_ptr(c, ROLLER_CALL_ADDR, 0x0036_1cec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldObjects_GlobalCtors` (0x361e10): the static initialiser of the
/// world objects, `WorldObjects_StaticInit(1, 0xffff)`.
pub fn world_objects_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        WORLD_OBJECTS_STATIC_INIT.call(c, 0x0036_1e24)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `IntcTable_Shutdown` (0x361f40): removes the VIF/VU interrupt handlers.
pub fn intc_table_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        INTC_REMOVE_VIF_VU_HANDLERS.call(c, 0x0036_1f50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `IntcHandler_SaveRegs(u64 *buf)` (0x362120): stores the 32 FPU registers
/// f0..f31 as sign-extended doublewords at buf[0..32] (`mfc1` each).
pub fn intc_handler_save_regs(c: &mut Cpu) {
    c.cycles += 67;
    abi::set_gpr128(c, V0, 0);
    let buf = abi::arg(c, 0);
    for i in 0..32usize {
        let v = abi::mfc1(FpuF32(c.f[i]));
        abi::set_reg(c, V0, v);
        Addr(buf).off(8 * i as u32).sd(c, v);
    }
    abi::ret(c);
}

/// `IntcHandler_RestoreRegs(u64 *buf)` (0x362230): the reverse of
/// [`intc_handler_save_regs`]: loads buf[0..32] into f0..f31 (`mtc1`, the low
/// word of each doubleword).
pub fn intc_handler_restore_regs(c: &mut Cpu) {
    c.cycles += 67;
    abi::set_gpr128(c, V0, 0);
    let buf = abi::arg(c, 0);
    for i in 0..32usize {
        let v = Addr(buf).off(8 * i as u32).ld(c);
        abi::set_reg(c, V0, v);
        abi::set_fpr(c, i, FpuF32(v as u32));
    }
    abi::ret(c);
}

/// `VramHeap_FreeTextureBlocks(heap, index)` (0x366e30): the block at slot
/// `index` of the heap's table (at `heap+0x23e8`, 4 bytes per slot) is freed
/// with `VramHeap_FreeBlock` for its first and then its second sub-block, when
/// each sub-block's offset (`+0x30`, `+0x34`) is non-negative.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn vram_heap_free_texture_blocks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let heap = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(heap.wrapping_add(0x23e8)).lw(c);
        let slot = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = slot;
        c.g[S1][0] = Addr(slot as u32).lw(c);
        let blk = c.g[S1][0] as u32;
        c.g[A1][0] = Addr(blk.wrapping_add(0x30)).lw(c);
        if (c.g[A1][0] as i64) < 0 {
            let blk = c.g[S1][0] as u32;
            c.g[A1][0] = Addr(blk.wrapping_add(0x34)).lw(c);
        } else {
            c.cycles += 3;
            VRAM_HEAP_FREE_BLOCK.call(c, 0x0036_6e68)?;
            let blk = c.g[S1][0] as u32;
            c.g[A1][0] = Addr(blk.wrapping_add(0x34)).lw(c);
        }
        c.cycles += 2;
        if (c.g[A1][0] as i64) < 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            VRAM_HEAP_FREE_BLOCK.call(c, 0x0036_6e7c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `World_GetObjectSectionData_2(obj, ...)` (0x3a6cd8): follows
/// `obj->+0x88 -> ->+0xc -> ->+0x8` and returns the word found there.
pub fn world_get_object_section_data_2(c: &mut Cpu) {
    c.cycles += 4;
    let obj = abi::arg(c, 1);
    c.g[V0][0] = Addr(obj.wrapping_add(0x88)).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
    abi::ret(c);
}

/// `cStreamMan_Cancel(stream)` (0x3a7010): when the stream has a pending
/// request (`+0x90` non-zero), runs its cancel callback (0x3e0e90, with
/// `+0x88` and `+0x9c`), then marks the request done: `+0x90 = 0`, `+0x94 = -1`.
pub fn stream_man_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(s0.wrapping_add(0x90)).lw(c);
        if c.g[V0][0] != 0u64 {
            c.cycles += 7;
            c.g[A0][0] = Addr(s0.wrapping_add(0x88)).lw(c);
            c.g[A1][0] = Addr(s0.wrapping_add(0x9c)).lw(c);
            call_ptr(c, STREAM_CANCEL_CALL_ADDR, 0x003a_7038)?;
            c.g[V0][0] = u64::MAX;
            let s0 = c.g[S0][0] as u32;
            Addr(s0.wrapping_add(0x90)).sw(c, 0);
            let v = c.g[V0][0];
            Addr(s0.wrapping_add(0x94)).sw(c, v);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWorldCache_FreeSectionTable(cache, index)` (0x3a8230): frees the
/// section allocator of the cache (`+0xc`), then the section table at slot
/// `index` of the cache's table (`+0x8`, 4 bytes per slot, `index << 2`), if
/// there is one, and clears the word that slot points at.
pub fn world_cache_free_section_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((abi::arg(c, 1)) << 2);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        WORLD_MEMMAN_FREE_SECTION_ALLOCATOR.call(c, 0x003a_8250)?;
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(s0.wrapping_add(0x8)).lw(c);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[A0][0] == 0u64 {
            let s0 = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(s0.wrapping_add(0x8)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            SECTION_TABLE_DTOR.call(c, 0x003a_826c)?;
            let s0 = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(s0.wrapping_add(0x8)).lw(c);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        frame.restore(c, S1, 0x10);
        Addr(c.g[V0][0] as u32).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    #[test]
    fn intersect_info_ctor_4_matches() {
        check(
            ee::s12::f_003611a0__generated,
            intersect_info_ctor_4,
            150,
            &|c, seed| {
                let mut r = Rng(0x0036_11a0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0036_11a0;
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 3));
                c.bus
                    .write32(DATA + 0x100 * (seed % 3) + 0x84, r.next_u32());
            },
        );
    }

    #[test]
    fn intersect_info_get_neg_normal_matches() {
        check(
            ee::s12::f_00361208__generated,
            intersect_info_get_neg_normal,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_1208 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0036_1208;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                for off in [0x50, 0x54, 0x58] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
                if seed % 4 == 0 {
                    c.bus.write32(DATA + 0x54, 0x8000_0000);
                }
            },
        );
    }

    #[test]
    fn intersect_info_ctor_5_matches() {
        check(
            ee::s12::f_00361248__generated,
            intersect_info_ctor_5,
            150,
            &|c, seed| {
                let mut r = Rng(0x0036_1248 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0036_1248;
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 2));
                c.bus
                    .write32(DATA + 0x100 * (seed % 2) + 0x84, r.next_u32());
            },
        );
    }

    #[test]
    fn intersect_info_ctor_6_matches() {
        check(
            ee::s12::f_00361300__generated,
            intersect_info_ctor_6,
            150,
            &|c, seed| {
                let mut r = Rng(0x0036_1300 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0036_1300;
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 2));
                c.bus
                    .write32(DATA + 0x100 * (seed % 2) + 0x84, r.next_u32());
            },
        );
    }

    #[test]
    fn conveyor_node_ctor_matches() {
        check(ee::s12::f_00361500__generated, conveyor_node_ctor, 150, &|c, seed| {
            let mut r = Rng(0x0036_1500 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0036_1500;
            c.g[A0][0] = sx(DATA + 0x100 * (seed % 2));
        });
    }

    #[test]
    fn counter_node_ctor_matches() {
        check(ee::s12::f_00361530__generated, counter_node_ctor, 150, &|c, seed| {
            let mut r = Rng(0x0036_1530 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0036_1530;
            c.g[A0][0] = sx(DATA + 0x100 * (seed % 2));
        });
    }

    #[test]
    fn roller_modifier_v10_matches() {
        check(ee::s12::f_00361cd8__generated, roller_modifier_v10, 150, &|c, seed| {
            let mut r = Rng(0x0036_1cd8 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0036_1cd8;
            c.g[A0][0] = sx(DATA + 0x100 * (seed % 2));
            c.bus
                .write32(DATA + 0x100 * (seed % 2) + 0x20, r.next_u32());
        });
    }

    #[test]
    fn world_objects_global_ctors_matches() {
        check(
            ee::s12::f_00361e10__generated,
            world_objects_global_ctors,
            100,
            &|c, seed| {
                let mut r = Rng(0x0036_1e10 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0036_1e10;
            },
        );
    }

    #[test]
    fn intc_table_shutdown_matches() {
        check(ee::s12::f_00361f40__generated, intc_table_shutdown, 100, &|c, seed| {
            let mut r = Rng(0x0036_1f40 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0036_1f40;
        });
    }

    #[test]
    fn intc_handler_save_regs_matches() {
        check(
            ee::s12::f_00362120__generated,
            intc_handler_save_regs,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_2120 ^ seed as u64);
                scramble(c, &mut r);
                for i in 0..32 {
                    c.f[i] = r.next_u32();
                }
                c.pc = 0x0036_2120;
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn intc_handler_restore_regs_matches() {
        check(
            ee::s12::f_00362230__generated,
            intc_handler_restore_regs,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_2230 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 64);
                c.pc = 0x0036_2230;
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn world_get_object_section_data_2_matches() {
        check(
            ee::s13::f_003a6cd8__generated,
            world_get_object_section_data_2,
            150,
            &|c, seed| {
                let mut r = Rng(0x003a_6cd8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_6cd8;
                c.g[A1][0] = sx(DATA);
                c.bus.write32(DATA + 0x88, DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0xc, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x8, r.next_u32());
            },
        );
    }

    #[test]
    fn stream_man_cancel_matches() {
        check(ee::s13::f_003a7010__generated, stream_man_cancel, 150, &|c, seed| {
            let mut r = Rng(0x003a_7010 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x003a_7010;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(
                DATA + 0x90,
                if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
            );
            c.bus.write32(DATA + 0x88, DATA + 0x400);
            c.bus.write32(DATA + 0x9c, r.next_u32());
        });
    }

    #[test]
    fn world_cache_free_section_table_matches() {
        check(
            ee::s13::f_003a8230__generated,
            world_cache_free_section_table,
            150,
            &|c, seed| {
                let mut r = Rng(0x003a_8230 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_8230;
                let table = DATA + 0x800;
                let idx = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.bus.write32(DATA + 0xc, DATA + 0x400);
                c.bus.write32(DATA + 0x8, table);
                let entry = if seed % 2 == 0 { 0 } else { DATA + 0xa00 };
                c.bus.write32(table + 4 * idx, entry);
            },
        );
    }
}
