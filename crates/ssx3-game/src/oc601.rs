//! Staging module oc601 (wave 233, list s2): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc601`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_5320,
        spline_modifier_is_path_visible,
        "SplineModifier_IsPathVisible",
    ),
    (
        0x0034_5d80,
        dynamic_particle_modifier_ctor_2,
        "DynamicParticleModifier_ctor_2",
    ),
    (0x0034_6350, flex_bridge_ctor, "FlexBridge_ctor"),
    (0x0034_64e0, flex_bridge_ctor_2, "FlexBridge_ctor_2"),
    (0x0034_6568, flex_bridge_dtor, "FlexBridge_dtor"),
];

// gp-relative graphics manager and its world pointer.
const GFX_MANAGER_GP: u32 = 0x2a90;
const GFX_WORLD: u32 = 0x10d8;

// Callees, as the generated bodies reach them.
const PARTICLE_BASE_CTOR: Callee = Callee {
    addr: 0x0034_57c8,
    f: ee::s11::f_003457c8,
};
const EMITTER_CTOR: u32 = 0x0037_0b60;
const EMITTER_SETUP: u32 = 0x0037_0dc8;
const EMITTER_READ: u32 = 0x0037_1318;
const EMITTER_UPDATE: u32 = 0x0037_10d0;
const OPERATOR_NEW_NAMED: u32 = 0x0031_7d20;
const OPERATOR_DELETE: u32 = 0x0031_7e98;
const GET_NASH: u32 = 0x002d_1be0;
const SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;

const FLEX_WORLD_CTOR: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};
const FLEX_WORLD_CTOR_2: Callee = Callee {
    addr: 0x0034_fb90,
    f: ee::s11::f_0034fb90,
};
const FLEX_WORLD_DTOR: Callee = Callee {
    addr: 0x0034_fbf0,
    f: ee::s11::f_0034fbf0,
};
const FLEX_SPRINGS_CTOR: Callee = Callee {
    addr: 0x0035_3da8,
    f: ee::s11::f_00353da8,
};
const FLEX_SPRINGS_DTOR: Callee = Callee {
    addr: 0x0035_3db8,
    f: ee::s11::f_00353db8,
};
const FLEX_SPRINGS_LOAD: Callee = Callee {
    addr: 0x0035_3e80,
    f: ee::s11::f_00353e80,
};
const FLEX_SPRINGS_INIT: Callee = Callee {
    addr: 0x0035_3e08,
    f: ee::s11::f_00353e08,
};
const FLEX_BUILD_MESH: Callee = Callee {
    addr: 0x0034_66a0,
    f: ee::s11::f_003466a0,
};
const FLEX_INIT_SEGMENT_MIDPOINTS: Callee = Callee {
    addr: 0x0034_6d38,
    f: ee::s11::f_00346d38,
};
const FLEX_UPDATE_VERTS: Callee = Callee {
    addr: 0x0034_75d8,
    f: ee::s11::f_003475d8,
};

/// `SplineModifier_IsPathVisible` (0x345320), `cSpline_isVisible`: the path's
/// box is `self->get()`'s bounds; the graphics manager's cull test (vtable
/// slot 93) on the two box corners, and its view call (slot 43), decide the
/// result: `cull != 1`.
pub fn spline_modifier_is_path_visible(c: &mut Cpu) {
    guest(c, is_path_visible);
}

fn is_path_visible(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0xb0);
    let stk = sp(c);
    let slot = |off: u32| stk.wrapping_add(off);
    frame.save(c, S0, 0xa0);
    frame.save(c, S1, 0x90);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save_ra(c, 0x60);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x002d_1bd8, 0x0034_5340)?;

    // bounds[0] (a box pointer), the self word, and the flag-table entry.
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = c.g[S0][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let entry_empty = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[S0][0] as u32) >> 8);
    let listed = if entry_empty {
        false
    } else {
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x44)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        nonzero
    };
    if !listed {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }

    // The main path (also reached with v0 = 0 above): read the box corners
    // from v0 + 4.., and call the graphics manager.
    c.cycles += 42;
    let p = c.g[V0][0] as u32;
    c.lwc1(0, p.wrapping_add(4));
    c.g[S2][0] = sx(slot(0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 2);
    c.g[S3][0] = sx(slot(0x10));
    c.swc1(0, stk);
    c.g[A0][0] = c.g[S2][0];
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(GFX_MANAGER_GP)).lw(c);
    c.lwc1(0, p.wrapping_add(8));
    c.swc1(0, slot(4));
    c.lwc1(1, p.wrapping_add(0xc));
    c.swc1(2, slot(0xc));
    c.swc1(1, slot(8));
    c.lwc1(0, p.wrapping_add(0x10));
    c.swc1(0, slot(0x10));
    c.lwc1(1, p.wrapping_add(0x14));
    c.swc1(1, slot(0x14));
    c.lwc1(0, p.wrapping_add(0x18));
    c.swc1(2, slot(0x1c));
    c.swc1(0, slot(0x18));
    c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_add(GFX_WORLD)).lw(c);
    c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x2e8)).lh(c);
    c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x158)).lh(c);
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x15c)).lw(c);
    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
    let cull = c.g[V0][0] as u32;
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, cull, 0x0034_53f0)?;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x2ec)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = c.g[S2][0];
    let view = c.g[V0][0] as u32;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, view, 0x0034_5408)?;

    c.g[V0][0] ^= 1;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore_ra(c, 0x60);
    frame.close(c);
    Ok(())
}

/// `cDynamicParticleModifier::cDynamicParticleModifier` (0x345d80), the
/// replay-frame constructor: the base constructor, the emitter, the
/// `Mesh` block read from the stream (80 bytes); a live emitter gets its
/// particle data read from the stream (216 bytes, a fresh allocation), set
/// up and updated once; otherwise its particle data is null. The static
/// count goes up by one. Returns `self`.
pub fn dynamic_particle_modifier_ctor_2(c: &mut Cpu) {
    guest(c, dynamic_particle_ctor);
}

fn dynamic_particle_ctor(c: &mut Cpu) -> Flow {
    c.cycles += 29;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S1, 0x30);
    frame.save(c, S2, 0x20);
    frame.save(c, S0, 0x40);
    frame.save(c, S3, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[S2][0] = c.g[A1][0];
    PARTICLE_BASE_CTOR.call(c, 0x0034_5da4)?;

    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[V1][0] = 0x490000;
    c.g[A0][0] = c.g[S3][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1268));
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
    c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V1][0]);
    call_ptr(c, EMITTER_CTOR, 0x0034_5dc0)?;

    c.g[V1][0] = 0x490000;
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1370));
    c.g[A2][0] = 0x50;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x258), c.g[V1][0]);
    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
    let read = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, read, 0x0034_5de8)?;

    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x18)).lw(c);
    if c.g[V0][0] != 0 {
        c.sw((c.g[S1][0] as u32).wrapping_add(0x260), 0);
        c.cycles += 1;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xf0c)).lw(c);
    } else {
        c.cycles += 26;
        c.g[RA][0] = sx(0x0034_5dfc);
        c.g[A0][0] = 0xd8;
        call_ptr(c, OPERATOR_NEW_NAMED, 0x0034_5dfc)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x260), c.g[V0][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0xd8;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        let setread = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, setread, 0x0034_5e1c)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x260)).lw(c);
        call_ptr(c, EMITTER_SETUP, 0x0034_5e30)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, EMITTER_READ, 0x0034_5e3c)?;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x2b4c));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A3][0] = 1;
        call_ptr(c, EMITTER_UPDATE, 0x0034_5e54)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xf0c)).lw(c);
    }

    c.cycles += 10;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x40);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    c.sw((c.g[GP][0] as u32).wrapping_add(0xf0c), c.g[V1][0]);
    frame.close(c);
    Ok(())
}

/// `cFlexBridgeNode::cFlexBridgeNode` (0x346350), the level-data constructor:
/// the instance-node base (bucket, kind 5, the instance), the flex-bridge
/// vtable, the springs; `TextureId` 12 and `RailHeight` 100; the quad
/// of the instance's mesh gives the spring length (its first edge); the grid
/// is set up, the springs are initialised, the mesh built, the midpoints
/// and the vertices updated; the instance's flags get `|= 4` and `|= 0x40`
/// (cleared of bit 1 and bit 5 first). Returns `self`.
pub fn flex_bridge_ctor(c: &mut Cpu) {
    guest(c, flex_bridge_ctor_body);
}

fn flex_bridge_ctor_body(c: &mut Cpu) -> Flow {
    c.cycles += 24;
    let frame = Frame::open(c, 0xe0);
    let stk = sp(c);
    let slot = |off: u32| stk.wrapping_add(off);
    frame.save(c, S2, 0xb0);
    frame.save(c, S0, 0xd0);
    c.g[S2][0] = c.g[A3][0];
    c.g[S0][0] = c.g[A0][0];
    c.g[A3][0] = c.g[A2][0];
    frame.save(c, S1, 0xc0);
    frame.save_ra(c, 0xa0);
    c.g[A2][0] = 5;
    FLEX_WORLD_CTOR.call(c, 0x0034_6378)?;

    c.g[V1][0] = 0x490000;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x270));
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
    FLEX_SPRINGS_CTOR.call(c, 0x0034_638c)?;

    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(1, 0);
    c.g[V0][0] = 0xc;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V0][0]);
    c.g[T0][0] = sx(slot(0x30));
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x40));
    c.g[T1][0] = sx(slot(0x50));
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = 3;
    // The first counted loop (four iterations: the local array's initialiser).
    loop {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let again = c.g[V1][0] != c.g[A0][0];
        c.g[T2][0] = Addr((c.g[GP][0] as u32).wrapping_add(GFX_MANAGER_GP)).lw(c);
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_63b0);
        }
    }
    c.cycles += 5;
    c.g[V0][0] = 3;
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = 3;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    // The second counted loop.
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let again = c.g[V0][0] != c.g[V1][0];
        c.g[A2][0] = 0;
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_63e0);
        }
    }

    // The grid quad: GetMeshQuad of the graphics manager's world.
    c.cycles += 56;
    c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(GFX_WORLD)).lw(c);
    c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
    c.g[A3][0] = c.g[SP][0];
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x308)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30c)).lw(c);
    let get_quad = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, get_quad, 0x0034_6418)?;

    c.lwc1(4, slot(0));
    c.g[A1][0] = c.g[S2][0];
    c.lwc1(3, slot(4));
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(2, slot(0xc));
    c.lwc1(1, slot(0x10));
    c.sub_s(2, 2, 4);
    c.lwc1(0, slot(0x14));
    c.sub_s(1, 1, 3);
    c.lwc1(4, slot(8));
    c.mul_s(12, 2, 2);
    c.swc1(2, slot(0x90));
    c.sub_s(0, 0, 4);
    c.swc1(1, slot(0x94));
    c.mul_s(3, 1, 1);
    c.mul_s(2, 0, 0);
    c.swc1(0, slot(0x98));
    c.add_s(12, 12, 3);
    c.add_s(12, 12, 2);
    c.sqrt_s(12, 12);
    FLEX_SPRINGS_INIT.call(c, 0x0034_6470)?;

    c.g[A0][0] = c.g[S0][0];
    FLEX_BUILD_MESH.call(c, 0x0034_6478)?;
    c.g[A0][0] = c.g[S0][0];
    FLEX_INIT_SEGMENT_MIDPOINTS.call(c, 0x0034_6480)?;
    c.g[A0][0] = c.g[S0][0];
    FLEX_UPDATE_VERTS.call(c, 0x0034_6488)?;

    // The instance flags: bit 1 cleared and bit 2 set, bit 5 cleared and bit 6 set.
    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
    c.g[A0][0] = 0xffff_ffff_ffff_0000;
    c.g[A0][0] |= 0xfffd;
    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    c.g[A1][0] |= 0xffdf;
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0xc0);
    c.g[V1][0] &= c.g[A0][0];
    frame.restore(c, S2, 0xb0);
    c.g[V1][0] |= 4;
    frame.restore_ra(c, 0xa0);
    c.sw((c.g[A2][0] as u32).wrapping_add(8), c.g[V1][0]);
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
    frame.restore(c, S0, 0xd0);
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
    c.g[V1][0] &= c.g[A1][0];
    c.g[V1][0] |= 0x40;
    c.sw((c.g[A0][0] as u32).wrapping_add(8), c.g[V1][0]);
    frame.close(c);
    Ok(())
}

/// `cFlexBridgeNode::cFlexBridgeNode` (0x3464e0), the replay-frame
/// constructor: the instance-node base, the springs loaded from the frame
/// (`s1`'s vtable slot 2 reads 0x30 bytes into the node), the mesh built, the
/// vertices updated. Returns `self`.
pub fn flex_bridge_ctor_2(c: &mut Cpu) {
    guest(c, flex_bridge_ctor_2_body);
}

fn flex_bridge_ctor_2_body(c: &mut Cpu) -> Flow {
    c.cycles += 33;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A2][0];
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
    FLEX_WORLD_CTOR_2.call(c, 0x0034_6504)?;

    c.g[V1][0] = 0x490000;
    c.g[A1][0] = c.g[S1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x270));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
    FLEX_SPRINGS_LOAD.call(c, 0x0034_651c)?;

    c.g[A0][0] = c.g[S0][0];
    FLEX_BUILD_MESH.call(c, 0x0034_6524)?;

    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = 0x30;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
    let read = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, read, 0x0034_6540)?;

    c.g[A0][0] = c.g[S0][0];
    FLEX_UPDATE_VERTS.call(c, 0x0034_6548)?;

    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x20);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFlexBridgeNode::~cFlexBridgeNode` (0x346568): the grid's index and
/// normal blocks are freed (when set), the grid quad is released through
/// the graphics world's slot, the springs and the spatial hash are
/// released, the base destructor runs. Nothing is returned.
pub fn flex_bridge_dtor(c: &mut Cpu) {
    guest(c, flex_bridge_dtor_body);
}

fn flex_bridge_dtor_body(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x80);
    let stk = sp(c);
    let slot = |off: u32| stk.wrapping_add(off);
    c.g[V0][0] = 0x490000;
    frame.save(c, S2, 0x50);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x270));
    frame.save(c, S3, 0x40);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0x70);
    frame.save(c, S1, 0x60);
    frame.save_ra(c, 0x30);
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);

    // The index block (0x84).
    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x84)).lw(c);
    let t = c.g[A0][0] == 0;
    c.g[S3][0] = c.g[A1][0];
    if !t {
        c.cycles += 2;
        call_ptr(c, OPERATOR_DELETE, 0x0034_65a4)?;
    }

    // The normal block (0x80), then the position block (0x7c).
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x80)).lw(c);
    if c.g[A0][0] == 0 {
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x7c)).lw(c);
    } else {
        c.cycles += 3;
        call_ptr(c, OPERATOR_DELETE, 0x0034_65b8)?;
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x7c)).lw(c);
    }

    c.cycles += 2;
    let t = c.g[A0][0] == 0;
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(GFX_MANAGER_GP)).lw(c);
    if !t {
        c.cycles += 3;
        call_ptr(c, OPERATOR_DELETE, 0x0034_65cc)?;
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(GFX_MANAGER_GP)).lw(c);
    }

    // The grid quad goes back through the world's slot (0x268 / 0x26c).
    c.cycles += 52;
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x90));
    c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x78)).lw(c);
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(GFX_WORLD)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x268)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x26c)).lw(c);
    let release = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, release, 0x0034_65ec)?;

    // Copy the two 16-byte vectors through the stack (quadword moves).
    c.lq(V1, (c.g[S2][0] as u32).wrapping_add(0xa0));
    c.lq(V0, (c.g[S2][0] as u32).wrapping_add(0x90));
    c.sq(V1, slot(0x10));
    c.sq(V0, slot(0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 2);
    c.g[S0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x18)).lw(c);
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x60));
    c.swc1(0, slot(0x20));
    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x64));
    c.swc1(1, slot(0x24));
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x68));
    c.swc1(2, slot(0x2c));
    c.swc1(0, slot(0x28));
    c.lq(V0, slot(0x20));
    c.sq(V0, (c.g[S2][0] as u32).wrapping_add(0x90));
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
    c.swc1(0, slot(0x20));
    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x70));
    c.swc1(1, slot(0x24));
    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x74));
    c.swc1(2, slot(0x2c));
    c.swc1(0, slot(0x28));
    c.lq(V0, slot(0x20));
    c.g[RA][0] = sx(0x0034_6654);
    c.sq(V0, (c.g[S2][0] as u32).wrapping_add(0xa0));
    call_ptr(c, GET_NASH, 0x0034_6654)?;

    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S1][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.g[SP][0];
    c.g[A1][0] = 0;
    call_ptr(c, SPATIAL_HASH_UPDATE, 0x0034_666c)?;

    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 2;
    FLEX_SPRINGS_DTOR.call(c, 0x0034_6678)?;

    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S3][0];
    FLEX_WORLD_DTOR.call(c, 0x0034_6684)?;

    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture layout (inside the test data area, clear of the fakes' log).
    const PATH_OBJ: u32 = DATA + 0x1_0000;
    const PATH_BND: u32 = DATA + 0x1_0100;
    const PATH_P: u32 = DATA + 0x1_0200;
    const PATH_TABLE: u32 = DATA + 0x1_0300;
    const PATH_ENTRY: u32 = DATA + 0x1_0400;
    const PATH_BASE: u32 = DATA + 0x1_0500;
    const PATH_GFX: u32 = DATA + 0x1_1000;
    const PATH_WORLD: u32 = DATA + 0x1_2200;

    const DYN_SELF: u32 = DATA + 0x2_0000;
    const DYN_STREAM: u32 = DATA + 0x2_1000;
    const DYN_VT: u32 = DATA + 0x2_2000;

    const NODE: u32 = DATA + 0x3_0000;
    const DEF: u32 = DATA + 0x3_0400;
    const MESH: u32 = DATA + 0x3_0800;
    const MESH_VT: u32 = DATA + 0x3_0900;
    const WORLD: u32 = DATA + 0x3_0c00;
    const GFX: u32 = DATA + 0x3_2000;
    const P_INST: u32 = DATA + 0x3_4000;
    const T_DATA: u32 = DATA + 0x3_4800;
    const FRAME: u32 = DATA + 0x3_5000;
    const FRAME_VT: u32 = DATA + 0x3_5100;
    const IDX_BLOCK: u32 = DATA + 0x3_7000;

    /// Gate word and the bump pointer of the gated allocator fakes.
    const OC601_CTRL: u32 = DATA + 0xf_f000;
    const OC601_MAGIC: u32 = 0x4f43_3630;
    /// Bump-allocated blocks for the fake allocators.
    const OC601_HEAP: u32 = DATA + 0x20_0000;

    // Field offsets the tests write (the node, the instance record, the world).
    const NODE_INST: u32 = 0x18;
    const NODE_COUNT: u32 = 0x50;
    const DEF_MIN: u32 = 0x60;
    const WORLD_GRID_DELTA: u32 = 0x260;
    const WORLD_GRID_FN: u32 = 0x264;
    const WORLD_DEL_DELTA: u32 = 0x268;
    const WORLD_DEL_FN: u32 = 0x26c;
    const WORLD_QUAD_DELTA: u32 = 0x308;
    const WORLD_QUAD_FN: u32 = 0x30c;
    const MESH_VT_DELTA: u32 = 0x30;
    const MESH_VT_FN: u32 = 0x34;

    /// Gated fake: logs the call (`fakes::fake`) and returns to `ra`, or runs
    /// the body it replaced when the gate is off (the other tests' machines).
    macro_rules! gated {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC601_CTRL) != OC601_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    /// Gated allocator fake: logged, and returns a bump block of the machine.
    macro_rules! gated_alloc {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC601_CTRL) != OC601_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
                if c.pc == ra {
                    let bump = c.bus.read32(OC601_CTRL + 4);
                    c.bus.write32(OC601_CTRL + 4, bump + 0x400);
                    c.g[V0][0] = (OC601_HEAP + bump) as u64;
                }
            }
        };
    }

    gated!(
        oc601_fake_base_ctor,
        OC601_PREV_BASE_CTOR,
        0x0034_57c8,
        ee::s11::f_003457c8__generated
    );
    gated!(
        oc601_fake_emitter_ctor,
        OC601_PREV_EMITTER_CTOR,
        0x0037_0b60,
        ee::s12::f_00370b60__generated
    );
    gated!(
        oc601_fake_emitter_setup,
        OC601_PREV_EMITTER_SETUP,
        0x0037_0dc8,
        ee::s12::f_00370dc8__generated
    );
    gated!(
        oc601_fake_emitter_read,
        OC601_PREV_EMITTER_READ,
        0x0037_1318,
        ee::s12::f_00371318__generated
    );
    gated!(
        oc601_fake_emitter_update,
        OC601_PREV_EMITTER_UPDATE,
        0x0037_10d0,
        ee::s12::f_003710d0__generated
    );
    gated_alloc!(
        oc601_fake_alloc_named,
        OC601_PREV_ALLOC_NAMED,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    gated_alloc!(
        oc601_fake_new,
        OC601_PREV_NEW,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    gated!(
        oc601_fake_delete,
        OC601_PREV_DELETE,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );
    gated!(
        oc601_fake_spatial,
        OC601_PREV_SPATIAL,
        0x0032_91e0,
        ee::s10::f_003291e0__generated
    );
    gated!(
        oc601_fake_get_nash,
        OC601_PREV_GET_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated
    );
    gated!(
        oc601_fake_world_ctor,
        OC601_PREV_WORLD_CTOR,
        0x0034_fb00,
        ee::s11::f_0034fb00__generated
    );
    gated!(
        oc601_fake_world_ctor_2,
        OC601_PREV_WORLD_CTOR_2,
        0x0034_fb90,
        ee::s11::f_0034fb90__generated
    );
    gated!(
        oc601_fake_world_dtor,
        OC601_PREV_WORLD_DTOR,
        0x0034_fbf0,
        ee::s11::f_0034fbf0__generated
    );
    gated!(
        oc601_fake_springs_ctor,
        OC601_PREV_SPRINGS_CTOR,
        0x0035_3da8,
        ee::s11::f_00353da8__generated
    );
    gated!(
        oc601_fake_springs_dtor,
        OC601_PREV_SPRINGS_DTOR,
        0x0035_3db8,
        ee::s11::f_00353db8__generated
    );
    gated!(
        oc601_fake_springs_load,
        OC601_PREV_SPRINGS_LOAD,
        0x0035_3e80,
        ee::s11::f_00353e80__generated
    );
    gated!(
        oc601_fake_springs_init,
        OC601_PREV_SPRINGS_INIT,
        0x0035_3e08,
        ee::s11::f_00353e08__generated
    );
    gated!(
        oc601_fake_init_segments,
        OC601_PREV_INIT_SEGMENTS,
        0x0034_6d38,
        ee::s11::f_00346d38__generated
    );

    fn oc601_install() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003457C8),
            oc601_fake_base_ctor,
            &OC601_PREV_BASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00370B60),
            oc601_fake_emitter_ctor,
            &OC601_PREV_EMITTER_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00370DC8),
            oc601_fake_emitter_setup,
            &OC601_PREV_EMITTER_SETUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00371318),
            oc601_fake_emitter_read,
            &OC601_PREV_EMITTER_READ,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003710D0),
            oc601_fake_emitter_update,
            &OC601_PREV_EMITTER_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc601_fake_alloc_named,
            &OC601_PREV_ALLOC_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc601_fake_new,
            &OC601_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc601_fake_delete,
            &OC601_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003291E0),
            oc601_fake_spatial,
            &OC601_PREV_SPATIAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
            oc601_fake_get_nash,
            &OC601_PREV_GET_NASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc601_fake_world_ctor,
            &OC601_PREV_WORLD_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB90),
            oc601_fake_world_ctor_2,
            &OC601_PREV_WORLD_CTOR_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FBF0),
            oc601_fake_world_dtor,
            &OC601_PREV_WORLD_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353DA8),
            oc601_fake_springs_ctor,
            &OC601_PREV_SPRINGS_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353DB8),
            oc601_fake_springs_dtor,
            &OC601_PREV_SPRINGS_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353E80),
            oc601_fake_springs_load,
            &OC601_PREV_SPRINGS_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353E08),
            oc601_fake_springs_init,
            &OC601_PREV_SPRINGS_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00346D38),
            oc601_fake_init_segments,
            &OC601_PREV_INIT_SEGMENTS,
        );
    }

    /// A fresh machine with the gated fakes armed and the bump heap reset.
    fn oc601_base(c: &mut Cpu, seed: u32, salt: u64) {
        oc601_install();
        let _ = fakes::machine(c, seed, 0x0601_0000 ^ salt, 10);
        c.bus.write32(OC601_CTRL, OC601_MAGIC);
        c.bus.write32(OC601_CTRL + 4, 0);
    }

    /// The graphics manager, its world with the grid, quad and delete slots
    /// (fakes), the mesh the grid returns, and a mesh vertex fill (a fake).
    fn oc601_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP + GFX_MANAGER_GP, GFX);
        c.bus.write32(GFX + GFX_WORLD, WORLD);
        c.bus.write16(WORLD + WORLD_GRID_DELTA, 0);
        c.bus.write32(WORLD + WORLD_GRID_FN, fakes::FAKE + 0x40);
        c.bus.write16(WORLD + WORLD_QUAD_DELTA, 0);
        c.bus.write32(WORLD + WORLD_QUAD_FN, fakes::FAKE + 0x80);
        c.bus.write16(WORLD + WORLD_DEL_DELTA, 0);
        c.bus.write32(WORLD + WORLD_DEL_FN, fakes::FAKE + 0xc0);
        // Early fake calls return the mesh (CreateGridMesh is one of them).
        for i in 0..8 {
            c.bus.write32(LOG_RESULTS + 4 * i, MESH);
        }
        c.bus.write32(MESH + 4, MESH_VT);
        c.bus.write16(MESH_VT + MESH_VT_DELTA, 0);
        c.bus.write32(MESH_VT + MESH_VT_FN, fakes::FAKE + 0x100);
    }

    /// `cSpline_isVisible` cases: no table entry (seed % 3 == 0), an entry
    /// whose flag byte is clear (seed % 3 == 1), and a listed entry.
    fn path_setup(c: &mut Cpu, seed: u32) {
        oc601_base(c, seed, 0x3200);
        let mut r = Rng(0x3200 ^ seed as u64);
        let i = seed & 15;
        let j = (seed / 3) % 12;
        c.bus.write32(PATH_OBJ, (j << 8) | i);
        let entry = if seed % 3 == 0 { 0 } else { PATH_ENTRY };
        c.bus.write32(PATH_TABLE + 4 * i, entry);
        c.bus.write32(PATH_ENTRY + 0x44, PATH_BASE);
        let low = r.next_u32() & 0xff;
        let flags = if seed % 3 == 2 {
            ((1 + (seed / 3) % 5) << 8) | low
        } else {
            low
        };
        c.bus.write32(PATH_BASE + 4 * j, flags);
        c.bus.write32(GAME_GP + 0x16c8, PATH_BND);
        c.bus.write32(PATH_BND, PATH_P);
        c.bus.write32(PATH_P + 8, PATH_TABLE);
        c.bus.write32(GAME_GP + GFX_MANAGER_GP, PATH_GFX);
        c.bus.write32(PATH_GFX + GFX_WORLD, PATH_WORLD);
        c.bus.write16(PATH_WORLD + 0x2e8, (4 * (seed % 3)) as u16);
        c.bus.write16(PATH_WORLD + 0x158, 8);
        c.bus.write32(PATH_WORLD + 0x15c, fakes::FAKE + 0x40);
        c.bus.write32(PATH_WORLD + 0x2ec, fakes::FAKE + 0x80);
        c.g[A0][0] = PATH_OBJ as u64;
        c.pc = 0x0034_5320;
    }

    #[test]
    fn spline_modifier_is_path_visible_matches() {
        fakes::covered(
            ee::s11::f_00345320__generated,
            spline_modifier_is_path_visible,
            120,
            &|c, seed| path_setup(c, seed),
        );
    }

    /// The particle modifier's stream holds an 80-byte block read through
    /// the stream's vtable (slot 0x10); `seed % 2` chooses the live emitter
    /// (no allocation) or the fresh particle block (allocated).
    fn dyn_setup(c: &mut Cpu, seed: u32) {
        oc601_base(c, seed, 0x2000);
        c.bus.write32(DYN_STREAM, DYN_VT);
        c.bus.write16(DYN_VT + 0x10, 0x20);
        c.bus.write32(DYN_VT + 0x14, fakes::FAKE + 0x10);
        c.bus
            .write32(DYN_SELF + 0x18, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + 0xf0c, seed);
        c.g[A0][0] = DYN_SELF as u64;
        c.g[A1][0] = DYN_STREAM as u64;
        c.pc = 0x0034_5d80;
    }

    #[test]
    fn dynamic_particle_modifier_ctor_2_matches() {
        fakes::covered(
            ee::s11::f_00345d80__generated,
            dynamic_particle_modifier_ctor_2,
            120,
            &|c, seed| dyn_setup(c, seed),
        );
    }

    /// The bridge node with an empty grid (count 0, so the real build and
    /// update bodies take their short path), its instance record, the
    /// graphics manager, and the spring/mesh callees as fakes.
    fn ctor_setup(c: &mut Cpu, seed: u32) {
        oc601_base(c, seed, 0x3450);
        oc601_world(c);
        let mut r = Rng(0x3450 ^ seed as u64);
        c.bus.write32(NODE + NODE_INST, DEF);
        c.bus.write32(NODE + NODE_COUNT, 0);
        for k in 0..6u32 {
            c.bus.write32(DEF + DEF_MIN + 4 * k, r.f32(50.0));
        }
        c.g[A0][0] = NODE as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = P_INST as u64;
        c.g[A3][0] = T_DATA as u64;
        c.pc = 0x0034_6350;
    }

    #[test]
    fn flex_bridge_ctor_matches() {
        fakes::covered(
            ee::s11::f_00346350__generated,
            flex_bridge_ctor,
            120,
            &|c, seed| ctor_setup(c, seed),
        );
    }

    /// The replay-frame constructor: its vtable slot 0x14 reads the
    /// 0x30-byte block the node keeps.
    fn ctor_2_setup(c: &mut Cpu, seed: u32) {
        oc601_base(c, seed, 0x3464);
        oc601_world(c);
        c.bus.write32(NODE + NODE_INST, DEF);
        c.bus.write32(NODE + NODE_COUNT, 0);
        c.bus.write32(FRAME, FRAME_VT);
        c.bus.write16(FRAME_VT + 0x10, 0x20);
        c.bus.write32(FRAME_VT + 0x14, fakes::FAKE + 0x140);
        c.g[A0][0] = NODE as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = FRAME as u64;
        c.pc = 0x0034_64e0;
    }

    #[test]
    fn flex_bridge_ctor_2_matches() {
        fakes::covered(
            ee::s11::f_003464e0__generated,
            flex_bridge_ctor_2,
            120,
            &|c, seed| ctor_2_setup(c, seed),
        );
    }

    /// The node with its index and normal blocks set on odd seeds, its
    /// instance record, the graphics manager and world (release slot a fake).
    fn dtor_setup(c: &mut Cpu, seed: u32) {
        oc601_base(c, seed, 0x3468);
        oc601_world(c);
        let blocks = seed % 2 == 1;
        c.bus.write32(NODE + 0x18, DEF);
        c.bus.write32(NODE + 0x78, MESH);
        c.bus
            .write32(NODE + 0x7c, if blocks { IDX_BLOCK } else { 0 });
        c.bus.write32(
            NODE + 0x80,
            if seed % 4 == 1 { IDX_BLOCK + 0x100 } else { 0 },
        );
        c.bus.write32(
            NODE + 0x84,
            if seed % 3 == 0 { 0 } else { IDX_BLOCK + 0x200 },
        );
        c.g[A0][0] = NODE as u64;
        c.g[A1][0] = (seed % 5) as u64;
        c.pc = 0x0034_6568;
    }

    #[test]
    fn flex_bridge_dtor_matches() {
        fakes::covered(
            ee::s11::f_00346568__generated,
            flex_bridge_dtor,
            120,
            &|c, seed| dtor_setup(c, seed),
        );
    }
}
