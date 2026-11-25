//! Staging module oc660 (wave g40.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc660`.

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
    (0x0034_4e18, crowd_man3d_group_render, "CrowdMan3d_GroupRender"),
    (0x0034_8fa0, float_rail_ctor_2, "FloatRail_ctor_2"),
];

/// `base + off` as a 32-bit guest address (the `(reg as u32).wrapping_add`
/// the generated code spells out).
fn field_addr(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// The group's `(ref, +8)` slot array starts at `self + 0xc`.
const GROUP_REFS: u32 = 0xc;
/// Number of instances in the group (`< 2`: the single-draw path).
const GROUP_INSTANCES: u32 = 0x114;
/// The group's material block (`CrowdMan3d_GroupSetMeshFrame` target).
const GROUP_MATERIALS: u32 = 0x118;
/// The group's matrix block (passed to the draw call).
const GROUP_MATRICES: u32 = 0x1e8;
/// `gp + 0x2a90`: the crowd table whose `+0x10d8` holds the graphics object.
const GP_CROWD_TABLE: u32 = 0x2a90;
/// Offset of the graphics object inside the crowd table.
const CROWD_TABLE_GRAPHICS: u32 = 0x10d8;
/// `gp + 0x16c8`: the world object (`World_GetInstance` returns it).
const GP_WORLD: u32 = 0x16c8;

const CROWD_GROUP_SET_MESH_FRAME: Callee = Callee {
    addr: 0x0034_4730,
    f: ee::s11::f_00344730,
};
const FLEX_RAIL_CTOR_2: Callee = Callee {
    addr: 0x0034_7ea8,
    f: ee::s11::f_00347ea8,
};
const FLOAT_RAIL_UPDATE_MATS: Callee = Callee {
    addr: 0x0034_92a8,
    f: ee::s11::f_003492a8,
};
const FLOAT_RAIL_V50: Callee = Callee {
    addr: 0x0034_9110,
    f: ee::s11::f_00349110,
};
const FLEX_RAIL_REGISTER_SPACE_NODE: Callee = Callee {
    addr: 0x0034_8b90,
    f: ee::s11::f_00348b90,
};

/// `void CrowdMan3d_GroupRender(CrowdGroup* self)` (0x344e18): draw the
/// group's 32 `(ref, +8)` slots. A `ref` of -1 is empty. Each slot's model is
/// looked up through the world's two tables and drawn only when it is loaded.
/// A group with two or more instances first advances the per-instance mesh
/// frames (`CrowdMan3d_GroupSetMeshFrame`) and draws through the graphics
/// object's `+0x2f8/+0x2fc` slot with a factor of 1.0; a smaller group draws
/// through the `+0x300/+0x304` slot.
pub fn crowd_man3d_group_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = sx(field_addr(c.g[S1][0], GROUP_REFS));
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 0x1f;
        frame.save(c, S0, 0x60);
        c.g[S5][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = Addr(field_addr(c.g[GP][0], GP_CROWD_TABLE)).lw(c);
        loop {
            'slot: {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[V0][0] = !c.g[V0][0];
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = sx(field_addr(c.g[S2][0], 8));
                    break 'slot;
                }
                // The model: the world's first table by the low byte of
                // `ref`, then the second table by its next byte.
                'model: {
                    c.cycles += 11;
                    call_ptr(c, 0x002d_1bd8, 0x0034_4e68)?;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[A0][0] = Addr(field_addr(c.g[V1][0], 8)).lw(c);
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx(field_addr(c.g[V0][0], c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    if c.g[V1][0] == 0 {
                        c.cycles += 2;
                        c.g[S0][0] = 0;
                        break 'model;
                    }
                    c.g[A0][0] = Addr(field_addr(c.g[V1][0], 0x1c)).lw(c);
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx(field_addr(c.g[V0][0], c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    c.g[A1][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[A1][0];
                }
                // Loaded? The model's flag byte at +0x7d (0xff: empty) and
                // the world's type entry must match.
                'loaded: {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(field_addr(c.g[GP][0], GP_WORLD)).lw(c);
                    c.g[A0][0] = 0xff;
                    c.g[V1][0] = Addr(field_addr(c.g[S0][0], 0x7d)).lbu(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let empty_byte = c.g[V1][0] == c.g[A0][0];
                    c.g[V0][0] = 1;
                    if !empty_byte {
                        c.cycles += 7;
                        c.g[V0][0] = Addr(field_addr(c.g[S0][0], 0x7d)).lbu(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                        c.g[V0][0] = sx(field_addr(c.g[A1][0], c.g[V0][0] as u32));
                        c.g[V1][0] = Addr(field_addr(c.g[V0][0], 0x24)).lw(c);
                        c.g[V1][0] ^= 0x6;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = 0;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = Addr(field_addr(c.g[S0][0], 0x7e)).lh(c);
                            c.g[A0][0] = 0x18;
                            c.mult(0, 3, 2, 4);
                            c.g[V0][0] = sx(field_addr(c.g[V1][0], c.g[A1][0] as u32));
                            c.g[V1][0] = Addr(field_addr(c.g[V0][0], 0x3f0)).lw(c);
                            c.g[V1][0] ^= 0x3;
                            c.g[V0][0] = (c.g[V1][0] < 0x1) as u64;
                        }
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = sx(field_addr(c.g[S2][0], 8));
                    break 'slot;
                }
                'draw: {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(field_addr(c.g[S1][0], GROUP_INSTANCES)).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] =
                            Addr(field_addr(c.g[S3][0], CROWD_TABLE_GRAPHICS)).lw(c);
                        break 'draw;
                    }
                    // Two or more instances: advance this slot's mesh frame,
                    // then draw with the factor 1.0 (f12, from AT).
                    c.cycles += 19;
                    c.g[V0][0] = sx(field_addr(c.g[S1][0], c.g[S5][0] as u32));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = Addr(field_addr(c.g[V0][0], 0x10)).lw(c);
                    CROWD_GROUP_SET_MESH_FRAME.call(c, 0x0034_4f30)?;
                    c.g[V0][0] = Addr(field_addr(c.g[S3][0], CROWD_TABLE_GRAPHICS)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 12);
                    c.g[A2][0] = sx(field_addr(c.g[S1][0], GROUP_MATERIALS));
                    c.g[A0][0] = Addr(field_addr(c.g[V0][0], 0x2f8)).lh(c);
                    c.g[A3][0] = sx(field_addr(c.g[A1][0], 0x10));
                    c.g[V1][0] = Addr(field_addr(c.g[V0][0], 0x2fc)).lw(c);
                    c.g[T1][0] = 0;
                    c.g[T0][0] = Addr(field_addr(c.g[S1][0], GROUP_MATRICES)).lw(c);
                    c.g[A0][0] = sx(field_addr(c.g[S3][0], c.g[A0][0] as u32));
                    c.g[T2][0] = 0;
                    let draw_fn = c.g[V1][0] as u32;
                    call_ptr(c, draw_fn, 0x0034_4f64)?;
                    c.g[S2][0] = sx(field_addr(c.g[S2][0], 8));
                    break 'slot;
                }
                // One instance: draw through the second graphics slot.
                c.cycles += 9;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = Addr(field_addr(c.g[S1][0], GROUP_MATRICES)).lw(c);
                c.g[A3][0] = 0;
                c.g[A0][0] = Addr(field_addr(c.g[V1][0], 0x300)).lh(c);
                c.g[T0][0] = 0;
                c.g[V0][0] = Addr(field_addr(c.g[V1][0], 0x304)).lw(c);
                let draw_fn = c.g[V0][0] as u32;
                c.g[A0][0] = sx(field_addr(c.g[S3][0], c.g[A0][0] as u32));
                call_ptr(c, draw_fn, 0x0034_4f8c)?;
                c.g[S2][0] = sx(field_addr(c.g[S2][0], 8));
            }
            c.cycles += 3;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            let more = (c.g[S4][0] as i64) >= 0;
            c.g[S5][0] = sx(field_addr(c.g[S5][0], 8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_4e50);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* FloatRail_ctor_2(FloatRail* self, void* bucket, cBxReplayFrame*
/// stream)` (0x348fa0): the stream constructor. Runs the base constructor,
/// reads the node count from `self+0x20`, allocates `count * 0x50` bytes for
/// the nodes and stores the block at `self+0x70`. Each node's `+0x40` word is
/// set to -1, then the rail updates its matrices, its position and its space
/// hash. Returns `self`.
pub fn float_rail_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        FLEX_RAIL_CTOR_2.call(c, 0x0034_8fc4)?;
        c.g[S3][0] = sx(field_addr(c.g[S1][0], 0x70));
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx(field_addr(c.g[S1][0], 0x60));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd0));
        c.g[A2][0] = 0xc;
        let vtable = c.g[V1][0];
        Addr(field_addr(c.g[S1][0], 0xc)).sw(c, vtable);
        // The stream reader: `read(stream + off)` through the record's vtable.
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr(field_addr(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = Addr(field_addr(c.g[V0][0], 0x14)).lw(c);
        let read_fn = c.g[V1][0] as u32;
        c.g[A0][0] = sx(field_addr(c.g[S2][0], c.g[A0][0] as u32));
        call_ptr(c, read_fn, 0x0034_8ff0)?;
        c.g[S0][0] = Addr(field_addr(c.g[S1][0], 0x20)).lw(c);
        c.g[A0][0] = 0x50;
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x2000_0000;
        c.mult(0, 0, 16, 4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17e0));
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0034_9014)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !empty {
            c.cycles += 4;
            c.g[A1][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = u64::MAX;
            c.g[A1][0] |= 0xffff;
            loop {
                c.cycles += 7;
                let minus_one = c.g[A1][0];
                Addr(field_addr(c.g[V1][0], 0x40)).sw(c, minus_one);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                let more = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx(field_addr(c.g[V1][0], 0x50));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_9030);
                }
            }
        }
        c.cycles += 25;
        let block = c.g[V0][0];
        Addr(field_addr(c.g[S3][0], 0)).sw(c, block);
        c.g[V0][0] = 0x50;
        c.g[A2][0] = Addr(field_addr(c.g[S1][0], 0x20)).lw(c);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.mult(0, 0, 6, 2);
        c.g[A1][0] = Addr(field_addr(c.g[S1][0], 0x70)).lw(c);
        c.g[A0][0] = Addr(field_addr(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = Addr(field_addr(c.g[V1][0], 0x14)).lw(c);
        c.g[A2][0] = c.lo[0];
        let fill_fn = c.g[V0][0] as u32;
        c.g[A0][0] = sx(field_addr(c.g[S2][0], c.g[A0][0] as u32));
        call_ptr(c, fill_fn, 0x0034_9078)?;
        c.g[A0][0] = c.g[S1][0];
        FLOAT_RAIL_UPDATE_MATS.call(c, 0x0034_9080)?;
        c.g[A0][0] = c.g[S1][0];
        FLOAT_RAIL_V50.call(c, 0x0034_9088)?;
        c.g[A0][0] = c.g[S1][0];
        FLEX_RAIL_REGISTER_SPACE_NODE.call(c, 0x0034_9090)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// A game-globals address (`gp + off`) in the fixture.
    fn game_gp_at(off: u32) -> u32 {
        GAME_GP.wrapping_add(off)
    }

    /// Gate word for the rail fakes (written into the machine's own RAM).
    pub(crate) const OC660_CTRL: u32 = DATA + 0x2_0000;
    pub(crate) const OC660_MAGIC: u32 = 0x0660_0660;
    /// The node block `operator_new` hands back while the fakes are on.
    pub(crate) const OC660_ALLOC: u32 = DATA + 0x8000;

    pub(crate) static OC660_PREV_FLEX_CTOR_2: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC660_PREV_UPDATE_MATS: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC660_PREV_V50: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC660_PREV_SPACE_NODE: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC660_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    /// Gated `FlexRail_ctor_2`: returns at once.
    pub(crate) fn oc660_fake_flex_ctor_2(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_FLEX_CTOR_2, ee::s11::f_00347ea8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `FloatRail_updateMats`: returns at once.
    pub(crate) fn oc660_fake_update_mats(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_UPDATE_MATS, ee::s11::f_003492a8__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `FloatRail_v50`: returns at once.
    pub(crate) fn oc660_fake_v50(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_V50, ee::s11::f_00349110__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `FlexRail_RegisterSpaceNode`: returns at once.
    pub(crate) fn oc660_fake_space_node(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_SPACE_NODE, ee::s11::f_00348b90__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `operator_new_named`: hands back the fixture's node block.
    pub(crate) fn oc660_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC660_CTRL) != OC660_MAGIC {
            return fakes::fall_back(c, &OC660_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        c.g[V0][0] = OC660_ALLOC as u64;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc660_install() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00347EA8),
            oc660_fake_flex_ctor_2,
            &OC660_PREV_FLEX_CTOR_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003492A8),
            oc660_fake_update_mats,
            &OC660_PREV_UPDATE_MATS,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349110),
            oc660_fake_v50,
            &OC660_PREV_V50,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00348B90),
            oc660_fake_space_node,
            &OC660_PREV_SPACE_NODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc660_fake_new,
            &OC660_PREV_NEW,
        );
    }

    #[test]
    fn crowd_man3d_group_render_matches() {
        check(
            ee::s11::f_00344e18__generated,
            crowd_man3d_group_render,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x344e, 0);
                let this = DATA + 0x1000;
                let gobj = DATA + 0x5000;
                let world = DATA + 0x6000;
                let world_obj = DATA + 0x6100;
                let tab = DATA + 0x6200;
                let base2 = DATA + 0x14000;
                c.pc = 0x0034_4e18;
                c.g[A0][0] = this as u64;
                c.bus.write32(game_gp_at(GP_CROWD_TABLE), DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + CROWD_TABLE_GRAPHICS, gobj);
                // Graphics object: the two draw slots (delta, function).
                c.bus.write16(gobj + 0x2f8, 0x40);
                c.bus.write32(gobj + 0x2fc, FAKE);
                c.bus.write16(gobj + 0x300, 0x80);
                c.bus.write32(gobj + 0x304, FAKE + 0x10);
                // World: its object, the instance table and the model tables.
                c.bus.write32(game_gp_at(GP_WORLD), world);
                c.bus.write32(world, world_obj);
                c.bus.write32(world_obj + 8, tab);
                c.bus.write32(world_obj + 0x24, if seed % 2 == 0 { 6 } else { 7 });
                c.bus.write32(world_obj + 0x3f0, if seed % 3 == 0 { 0 } else { 3 });
                for k in 0..256u32 {
                    let e = DATA + 0x10000 + 0x40 * k;
                    c.bus.write32(tab + 4 * k, if k % 3 == 0 { 0 } else { e });
                    c.bus.write32(e + 0x1c, base2);
                    c.bus.write32(base2 + 4 * k, (k % 4) << 8);
                }
                if seed % 7 == 0 {
                    c.bus.write8(0x7d, 0xff);
                }
                c.bus.write32(this + 4, 0);
                c.bus.write32(this + 0x110, 0);
                c.bus.write32(this + 0x114, seed % 4);
                c.bus.write32(this + GROUP_MATRICES, DATA + 0x7000);
                for i in 0..32u32 {
                    let rf = if (i + seed) % 5 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() & 0xffff
                    };
                    c.bus.write32(this + GROUP_REFS + 8 * i, rf);
                    c.bus.write32(this + 0x10 + 8 * i, r.next_u32() & 0xff);
                }
            },
        );
    }

    #[test]
    fn float_rail_ctor_2_matches() {
        oc660_install();
        check(
            ee::s11::f_00348fa0__generated,
            float_rail_ctor_2,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x348f, 0);
                c.bus.write32(OC660_CTRL, OC660_MAGIC);
                let this = DATA + 0x1000;
                let stream = DATA + 0x2800;
                let record = DATA + 0x2900;
                c.pc = 0x0034_8fa0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = stream as u64;
                // The stream's record: its reader is a logged fake call.
                c.bus.write32(stream, record);
                c.bus.write16(record + 0x10, (4 * (seed % 8)) as u16);
                c.bus.write32(record + 0x14, FAKE + 0x40);
                // Node count: 0 (no block) to 4.
                c.bus.write32(this + 0x20, seed % 5);
            },
        );
    }
}
