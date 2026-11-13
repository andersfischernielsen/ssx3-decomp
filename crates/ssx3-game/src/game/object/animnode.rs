//! `src/object/animnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::be::beprepoststate::gp_rel;
pub(crate) use crate::game::visualfx::boardtrailfx::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_e600,
        anim_object_get_node_position,
        "AnimObject_GetNodePosition",
    ),
    (
        0x0034_eba0,
        anim_object_fire_end_script,
        "AnimObject_FireEndScript",
    ),
    (
        0x0034_ebe0,
        anim_object_is_time_in_window,
        "AnimObject_IsTimeInWindow",
    ),
    (0x0034_ee68, floating_v01, "Floating_v01"),
    (
        0x0034_e448,
        anim_object_alloc_node_info,
        "AnimObject_AllocNodeInfo",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;

    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
}

/// `void AnimObject_GetNodePosition(AnimObject *obj, int node, void *out,
/// void *arg)` (0x34e600): if `node` equals `obj->selected(+0x2c)`, copy the
/// node's 16-byte position (from `*(obj->model(+0x20))->+0x30` via a virtual
/// call) to `out` and to the stack; else copy the node's entry (at
/// `out = obj->model + node*0x40 + 0x40`, via a virtual call) to `out`.
pub fn anim_object_get_node_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x640);
        let obj = abi::arg(c, 0);
        let t0 = obj.wrapping_add(0x14);
        c.g[T0][0] = sx(t0);
        frame.save(c, S1, 0x620);
        frame.save(c, S0, 0x630);
        frame.save_ra(c, 0x610);
        let selected = Addr(obj).off(0x2c).lw(c);
        c.g[V0][0] = selected;
        let matches = abi::arg64(c, 1) == selected;
        c.g[S1][0] = abi::arg64(c, 3);
        if matches {
            c.cycles += 10;
            let vt = Addr(obj).off(0x20).lw(c);
            c.g[V1][0] = vt;
            c.g[A1][0] = abi::arg64(c, 2);
            let m = VSlot::read(c, vt, 0xe8);
            c.g[A0][0] = m.this(t0 as u64);
            c.g[V0][0] = m.func;
            call_ptr(c, m.func as u32, 0x0034_e638)?;
            c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sq(3, c.g[S1][0] as u32);
            c.sq(3, c.g[SP][0] as u32);
        } else {
            c.cycles += 14;
            let idx = abi::arg(c, 2);
            c.g[S0][0] = sx(idx << 6);
            let vt = Addr(obj).off(0x20).lw(c);
            c.g[A3][0] = vt;
            c.g[A1][0] = sx(abi::arg(c, 1).wrapping_add(0x10));
            c.g[V0][0] = 0x17;
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = u64::MAX;
            let m = VSlot::read(c, vt, 0x108);
            c.g[A0][0] = m.this(t0 as u64);
            c.g[V0][0] = m.func;
            call_ptr(c, m.func as u32, 0x0034_e670)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[SP][0] as u32));
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[S1][0] as u32);
            c.sq(2, c.g[SP][0] as u32);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore_ra(c, 0x610);
        frame.close(c);
        Ok(())
    })
}

/// `int AnimObject_FireEndScript(sAnimObject* obj)` (0x34eba0): fire the
/// animation end script of the link at `obj+0x14`, then report whether the
/// link's list head is the link itself (i.e. `list->head == link`).
pub fn anim_object_fire_end_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = abi::arg(c, 0);
        c.g[S0][0] = sx(obj);
        c.g[A0][0] = Addr(p(obj, 0x2c)).lw(c);
        frame.save_ra(c, 0);
        let link = p(obj, 0x14);
        c.g[S0][0] = sx(link);
        call_ptr(c, 0x002d_1ac8, 0x0034_ebbc)?;
        c.g[V1][0] = Addr(p(link, 0x18)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[V0][0] ^= c.g[S0][0];
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void Floating_v01(Floating* self, int flags)` (0x34ee68): the anim
/// node's destructor: install the vtable, run the subclass teardown, free
/// the owned block and the base object.
pub fn floating_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_f858; // D_0048F858
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        callee::FLOATING_V49.call(c, 0x0034_ee90)?;
        c.g[A0][0] = Addr(p(this, 0x74)).lw(c);
        call_ptr(c, table::BUILTIN_DELETE, 0x0034_ee98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::MOD_OBJ_DTOR.call(c, 0x0034_eea4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AnimObject_IsTimeInWindow(AnimObject* self, float t)` (0x34EBE0): is
/// `t` inside the object's [start, end) play window? A start of 1e10 means
/// "no window"; a window that ends before it starts wraps.
pub fn anim_object_is_time_in_window(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(0, gp_rel(c, -0x2aa0)); // the "no window" sentinel (1e10)
    c.lwc1(1, ea(c.g[A0][0], 8)); // start
    c.c_eq_s(1, 0);
    let no_window = c.fcc();
    c.g[V0][0] = 0;
    if no_window {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.lwc1(0, ea(c.g[A0][0], 0xc)); // end
    c.c_lt_s(1, 0);
    if !c.fcc() {
        // start >= end: the window wraps, so [end, start) is outside.
        c.cycles += 4;
        c.c_le_s(0, 12);
        c.g[V0][0] = 0;
        if c.fcc() {
            c.cycles += 4;
            c.c_lt_s(12, 1);
            c.g[V0][0] = 1;
            if c.fcc() {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // start < end: start <= t < end.
    c.cycles += 3;
    c.c_le_s(1, 12);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(12, 0);
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

pub(crate) mod aoani {

    /// AnimObject: its resource, the mesh count, the mesh instances, the
    /// per-node references.
    pub const RES: u32 = 0x2c;

    pub const MESH_COUNT: u32 = 0x40;

    pub const MESHES: u32 = 0x48;

    pub const REFS: u32 = 0x4c;

    /// Resource: the node definitions.
    pub const DEF: u32 = 0x80;

    /// Node definitions: count and the 16-byte entries (`+8` the mesh
    /// source, or 0).
    pub const DEF_COUNT: u32 = 0x4;

    pub const DEF_ENTRIES: u32 = 0x8;

    pub const ENTRY_SIZE: u32 = 0x10;

    pub const ENTRY_SRC: u32 = 0x8;

    /// A mesh instance (`IntersectInfo`, 0xd0 bytes) and its vtable field.
    pub const MESH_SIZE: u32 = 0xd0;

    pub const MESH_VTABLE: u32 = 0x84;

    /// Allocation names and the `IntersectInfo` vtables (base, then final).
    pub const MESH_NAME: u64 = 0x0048_e860;

    pub const REF_NAME: u64 = 0x0048_e870;

    pub const BASE_VTABLE: u64 = 0x0049_0af0;

    pub const VTABLE: u64 = 0x0049_0ad0;
}

pub(crate) const AOANI_CLEAR: Callee = Callee {
    addr: 0x0035_13d0,
    f: ee::s11::IntersectInfo_Clear,
};

pub(crate) const AOANI_SET_SOURCE: Callee = Callee {
    addr: 0x0035_1398,
    f: ee::s11::IntersectInfo_SetSource,
};

/// `void AnimObject_AllocNodeInfo(AnimObject* self)` (0x34e448): count the
/// node definitions that have a mesh source, allocate that many mesh
/// instances (`new IntersectInfo[n]`: a count word, then each one cleared
/// and given its identity matrix) and one 32-byte reference per node, then
/// point each mesh instance at its node's source.
pub fn anim_object_alloc_node_info(c: &mut Cpu) {
    use aoani::*;
    let frame = Frame { size: 0xb0 };
    guest(c, |c| {
        c.cycles += 19;
        Frame::open(c, 0xb0);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S0, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, S3);
        c.g[V0][0] = Addr(this).off(RES).lw(c);
        c.g[S4][0] = Addr(r32(c, V0)).off(DEF).lw(c);
        let def = r32(c, S4);
        c.g[S0][0] = Addr(def).off(DEF_ENTRIES).lw(c);
        Addr(this).off(MESH_COUNT).sw(c, 0);
        c.g[V0][0] = Addr(def).off(DEF_COUNT).lw(c);
        c.g[S1][0] = 0;
        // Count the nodes with a mesh source.
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 1;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0)).off(ENTRY_SRC).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    let n = Addr(this).off(MESH_COUNT).lw(c);
                    c.g[V0][0] = sx((n as u32).wrapping_add(1));
                    let n = c.g[V0][0];
                    Addr(this).off(MESH_COUNT).sw(c, n);
                }
                c.g[V0][0] = Addr(def).off(DEF_COUNT).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(ENTRY_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_e498);
                }
            }
        }
        // The mesh instances (the refs pointer's address at sp+0).
        c.cycles += 3;
        c.g[S1][0] = Addr(this).off(MESH_COUNT).lw(c);
        c.g[A0][0] = MESH_SIZE as u64;
        if c.g[S1][0] != 0 {
            c.cycles += 15;
            c.g[V0][0] = sx(this.wrapping_add(REFS));
            c.mult(0, A0, S1, A0);
            c.g[A1][0] = MESH_NAME;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            let refs_at = c.g[V0][0];
            Addr(sp(c)).sw(c, refs_at);
            c.g[FP][0] = sx(this.wrapping_add(MESHES));
            c.g[S2][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            call_ptr(c, 0x0031_7e30, 0x0034_e4fc)?; // operator_new_named
            c.g[S5][0] = sx(r32(c, V0).wrapping_add(0x10));
            let count = c.g[S1][0];
            Addr(r32(c, V0)).sw(c, count);
            c.g[S0][0] = c.g[S5][0];
            if c.g[S1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = 0x0049_0000;
                c.g[V1][0] = 0x0049_0000;
                c.g[S7][0] = BASE_VTABLE;
                c.g[S6][0] = VTABLE;
                c.g[S1][0] = u64::MAX;
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S0][0];
                    let mesh = r32(c, S0);
                    let base_vt = c.g[S7][0];
                    Addr(mesh).off(MESH_VTABLE).sw(c, base_vt);
                    AOANI_CLEAR.call(c, 0x0034_e52c)?;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                    let mesh = r32(c, S0);
                    let vt = c.g[S6][0];
                    Addr(mesh).off(MESH_VTABLE).sw(c, vt);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0036_12c0, 0x0034_e53c)?; // IntersectInfo_ResetMatrix
                    let more = c.g[S2][0] != c.g[S1][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(MESH_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_e520);
                    }
                }
            }
            c.cycles += 2;
            let meshes = c.g[S5][0];
            Addr(r32(c, FP)).sw(c, meshes);
        } else {
            c.cycles += 3;
            Addr(this).off(MESHES).sw(c, 0);
            c.g[V1][0] = sx(this.wrapping_add(REFS));
            let refs_at = c.g[V1][0];
            Addr(sp(c)).sw(c, refs_at);
        }
        // The node references (32 bytes each).
        c.cycles += 9;
        c.g[S0][0] = Addr(r32(c, S4)).off(DEF_COUNT).lw(c);
        c.g[A1][0] = REF_NAME;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx(r32(c, S0) << 5);
        call_ptr(c, 0x0031_7e30, 0x0034_e574)?; // operator_new_named
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 6;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[S1][0] = 0;
        let refs = c.g[V0][0];
        Addr(r32(c, V1)).sw(c, refs);
        let def = r32(c, S4);
        c.g[V0][0] = Addr(def).off(DEF_COUNT).lw(c);
        c.g[S0][0] = Addr(def).off(DEF_ENTRIES).lw(c);
        // Point each mesh instance at its node's source.
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 3;
                c.g[A1][0] = Addr(r32(c, S0)).off(ENTRY_SRC).lw(c);
                if c.g[A1][0] != 0 {
                    c.cycles += 5;
                    let meshes = Addr(r32(c, S3)).off(MESHES).lw(c);
                    c.g[A0][0] = sx((meshes as u32).wrapping_add(r32(c, S2)));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(MESH_SIZE));
                    AOANI_SET_SOURCE.call(c, 0x0034_e5bc)?;
                }
                c.g[V0][0] = Addr(r32(c, S4)).off(DEF_COUNT).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(ENTRY_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_e5a0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;

    use crate::difftest::fakes::*;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn anim_object_get_node_position_matches() {
        covered(
            ssx3_ee::s11::f_0034e600,
            anim_object_get_node_position,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 22, 2);
                let obj = ptr(&mut r);
                c.g[A0][0] = obj as u64;
                let node = r.next_u32();
                let selected = if r.next_u32() % 2 == 0 {
                    node
                } else {
                    r.next_u32()
                };
                c.bus.write32(obj + 0x2c, selected);
                c.g[A1][0] = node as u64;
                c.g[A2][0] = ptr(&mut r) as u64;
                c.g[A3][0] = ptr(&mut r) as u64;
                let vt = vtable(c, &mut r, &[0xe8, 0x108]);
                c.bus.write32(obj + 0x20, vt);
            },
        );
    }

    #[test]
    fn anim_object_fire_end_script_matches() {
        check(
            ee::s11::f_0034eba0__generated,
            anim_object_fire_end_script,
            200,
            &|c, seed| {
                machine(c, seed, 0x34eba0, 3);
                let obj = DATA;
                let list = DATA + 0x1000;
                let man = DATA + 0x2000;
                c.bus.write32(GAME_GP + 0xce8, man);
                c.bus.write32(man + 0x28c, 0);
                c.bus.write32(list + 0x78, 0);
                c.bus.write32(list + 0x88, 0);
                c.bus.write32(obj + 0x2c, list);
                // `list->head == link` decides the result.
                c.bus.write32(
                    list + 0xc,
                    if seed % 2 == 0 {
                        obj + 0x14
                    } else {
                        obj + 0x40
                    },
                );
                c.g[A0][0] = sx(obj);
            },
        );
    }

    /// A block from the test heap (`alloc_ready` must have run first).
    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn floating_v01_matches() {
        check(
            ee::s11::f_0034ee68__generated,
            floating_v01,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x34ee_68, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x30)
                };
                c.bus.write32(this + 0x74, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    pub(crate) fn fb(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn anim_object_is_time_in_window_matches() {
        check(
            ee::s11::f_0034ebe0__generated,
            anim_object_is_time_in_window,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_ebe0, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.edge_f32();
                w(c, GAME_GP - 0x2aa0, fb(1.0e10));
                let (start, end) = match seed % 4 {
                    0 => (1.0e10, 2.0),
                    1 => (1.0, 2.0),
                    2 => (2.0, 1.0),
                    _ => (-1.0, 1.0),
                };
                w(c, DATA + 8, fb(start));
                w(c, DATA + 0xc, fb(end));
            },
        );
    }

    use crate::difftest::put_vec;

    /// `AnimObject_AllocNodeInfo`: 0..6 node definitions, each with or
    /// without a mesh source (no meshes at all for some seeds); the real
    /// `operator_new_named` allocates from the test pools / heap, the real
    /// `IntersectInfo_Clear` / `ResetMatrix` / `SetSource` fill them.
    #[test]
    fn anim_object_alloc_node_info_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x20_0000;
        const RES: u32 = SELF + 0x100;
        const DEF: u32 = SELF + 0x200;
        const ENTRIES: u32 = SELF + 0x300;
        const SOURCES: u32 = SELF + 0x400;
        check(
            ee::s11::f_0034e448__generated,
            anim_object_alloc_node_info,
            200,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x34_e448);
                aoani_identity_matrix(c);
                c.pc = 0x0034_e448;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + 0x2c, RES);
                c.bus.write32(SELF + 0x40, r.next_u32());
                c.bus.write32(RES + 0x80, DEF);
                let count = seed % 7;
                c.bus.write32(DEF + 4, count);
                c.bus.write32(DEF + 8, ENTRIES);
                for i in 0..count {
                    let src = if seed % 5 == 0 || r.next_u32() % 3 == 0 {
                        0
                    } else {
                        SOURCES + 0x20 * i
                    };
                    c.bus.write32(ENTRIES + 0x10 * i + 8, src);
                    for k in 0..6 {
                        c.bus.write32(SOURCES + 0x20 * i + 4 * k, r.f32(100.0));
                    }
                }
            },
        );
    }

    /// The identity matrix `IntersectInfo_ResetMatrix` copies (0x4ff1a0).
    pub(crate) fn aoani_identity_matrix(c: &mut ssx3_ee_rt::Cpu) {
        for i in 0..4u32 {
            put_vec(c, 0x004f_f1a0 + 0x10 * i, {
                let mut v = [0u32; 4];
                v[i as usize] = 0x3f80_0000;
                v
            });
        }
    }
}
