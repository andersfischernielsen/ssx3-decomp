//! Staging module oc410 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc410`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003a_bf20,
        c_ps2_chunk_resolve_relocate_mesh_dma,
        "cPS2ChunkResolve_RelocateMeshDma",
    ),
    (
        0x003a_bf50,
        c_ps2_chunk_resolve_relocate_mesh_dma_parts,
        "cPS2ChunkResolve_RelocateMeshDmaParts",
    ),
    (
        0x003a_bfc0,
        c_ps2_chunk_resolve_relocate_mesh_dma_part,
        "cPS2ChunkResolve_RelocateMeshDmaPart",
    ),
    (
        0x0016_9368,
        script_cam_controller_update,
        "ScriptCamController_Update",
    ),
    (
        0x0032_0e18,
        input_map_compile_entry,
        "InputMap_CompileEntry",
    ),
    (0x0035_7538, magnet_modifier_v21, "MagnetModifier_v21"),
    (
        0x0014_b988,
        beb_bolt_int_get_bolt_texture_name,
        "BEBoltInt_GetBoltTextureName",
    ),
    (
        0x0027_6bd8,
        script_list_get_play_status,
        "ScriptList_GetPlayStatus",
    ),
    (
        0x0027_2f28,
        in_seq_span_track_update,
        "InSeq_SpanTrackUpdate",
    ),
    (
        0x0017_12b0,
        camera_trigger_read_action,
        "CameraTrigger_ReadAction",
    ),
    (
        0x0032_50d8,
        input_parser_emit_unary_op,
        "InputParser_EmitUnaryOp",
    ),
    (
        0x0022_9820,
        crowd_man2d_bind_instance_frame,
        "CrowdMan2d_BindInstanceFrame",
    ),
    (
        0x0027_51a0,
        in_seq_instance_advance,
        "InSeq_InstanceAdvance",
    ),
    (
        0x0011_bbe8,
        rider_get_equipped_bolts,
        "Rider_GetEquippedBolts",
    ),
    (0x002f_c9c8, w_script_fn_create_flag, "WScriptFn_CreateFlag"),
    (0x0023_1fc0, load_indicator_render, "LoadIndicator_Render"),
    (
        0x0034_e798,
        anim_object_compute_node_velocities,
        "AnimObject_ComputeNodeVelocities",
    ),
];

// ---------------------------------------------------------------------------
// cPS2ChunkResolve: relocating mesh DMA lists.

/// `cPS2ChunkResolve::PatchDmaChain(ctx, chain, list, index)` (0x3ac048).
pub(crate) const CHUNK_PATCH_DMA_CHAIN: Callee = Callee {
    addr: 0x003a_c048,
    f: ee::s13::cPS2ChunkResolve_PatchDmaChain,
};

/// `cPS2ChunkResolve::RelocateMeshDmaParts(ctx, mesh)` (0x3abf50).
pub(crate) const CHUNK_RELOCATE_MESH_DMA_PARTS: Callee = Callee {
    addr: 0x003a_bf50,
    f: ee::s13::cPS2ChunkResolve_RelocateMeshDmaParts,
};

/// `cPS2ChunkResolve::RelocateMeshDmaPart(ctx, part, mesh)` (0x3abfc0).
pub(crate) const CHUNK_RELOCATE_MESH_DMA_PART: Callee = Callee {
    addr: 0x003a_bfc0,
    f: ee::s13::cPS2ChunkResolve_RelocateMeshDmaPart,
};

/// Mesh: relocation offset field (made absolute by `RelocateMeshDma`).
pub(crate) const CHUNK_MESH_RELOC: u32 = 0x24;
/// Mesh: number of parts, pointer to the part table (16-byte entries).
pub(crate) const CHUNK_MESH_PART_COUNT: u32 = 0x4;
pub(crate) const CHUNK_MESH_PART_TABLE: u32 = 0x8;
/// Part: number of DMA chains, pointer to the chain pointers.
pub(crate) const CHUNK_PART_CHAIN_COUNT: u32 = 0x1c;
pub(crate) const CHUNK_PART_CHAIN_LIST: u32 = 0x20;

/// `int cPS2ChunkResolve::RelocateMeshDma(ctx, mesh)` (0x3abf20): make the
/// mesh's `+0x24` field absolute (add the mesh address), relocate its parts,
/// return 1.
pub fn c_ps2_chunk_resolve_relocate_mesh_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(CHUNK_MESH_RELOC)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        let relocated = c.g[V0][0];
        Addr(r32(c, V1).wrapping_add(CHUNK_MESH_RELOC)).sw(c, relocated);
        CHUNK_RELOCATE_MESH_DMA_PARTS.call(c, 0x003a_bf3c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2ChunkResolve::RelocateMeshDmaParts(ctx, mesh)` (0x3abf50):
/// for each part entry with a non-null part pointer (+4), relocate it.
/// Returns 1.
pub fn c_ps2_chunk_resolve_relocate_mesh_dma_parts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(CHUNK_MESH_PART_COUNT)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = Addr(r32(c, S2).wrapping_add(CHUNK_MESH_PART_TABLE)).lw(c);
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = c.g[V0][0];
            loop {
                c.cycles += 3;
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(4)).lw(c);
                let null = c.g[A1][0] == 0;
                c.g[A0][0] = c.g[S3][0];
                if !null {
                    c.cycles += 2;
                    c.g[A2][0] = c.g[S2][0];
                    CHUNK_RELOCATE_MESH_DMA_PART.call(c, 0x003a_bf94)?;
                }
                c.cycles += 3;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_bf80);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2ChunkResolve::RelocateMeshDmaPart(ctx, part, mesh)` (0x3abfc0):
/// patch each of the part's DMA chains (`PatchDmaChain(ctx, chain, mesh,
/// index)`). Returns 1.
pub fn c_ps2_chunk_resolve_relocate_mesh_dma_part(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(CHUNK_PART_CHAIN_COUNT)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = Addr(r32(c, S2).wrapping_add(CHUNK_PART_CHAIN_LIST)).lw(c);
        if !empty {
            loop {
                c.cycles += 10;
                c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S3][0];
                CHUNK_PATCH_DMA_CHAIN.call(c, 0x003a_c00c)?;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(CHUNK_PART_CHAIN_COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_bff8);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptCamController_Update (0x169368)

/// `ScriptCameraAlgo::GetScriptObj(algo)` (0x1694b0).
pub(crate) const SCRIPT_CAMERA_ALGO_GET_SCRIPT_OBJ: Callee = Callee {
    addr: 0x0016_94b0,
    f: ee::s02::ScriptCameraAlgo_GetScriptObj,
};

/// Controller: pointer to the list head pointer (+8).
pub(crate) const SCRIPT_CAM_CTRL_LIST: u32 = 0x8;
/// Node: algo (+0), progress t (+4), blend weight (+0xc), next (+0x14).
#[cfg(test)]
pub(crate) const SCRIPT_CAM_NODE_ALGO: u32 = 0x0;
pub(crate) const SCRIPT_CAM_NODE_T: u32 = 0x4;
pub(crate) const SCRIPT_CAM_NODE_WEIGHT: u32 = 0xc;
pub(crate) const SCRIPT_CAM_NODE_NEXT: u32 = 0x14;

/// `void ScriptCamController::Update(this)` (0x169368): for every algo node
/// in the list, read its script object's progress (+0x18), store it as the
/// node's `t`, store the smoothstep `3p^2 - 2p^3` clamped to 1.0 as the
/// weight, then call the algo's vtable method at +0x2c (`this + delta(+0x28)`).
pub fn script_cam_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        let list = c.lw(r32(c, A0).wrapping_add(SCRIPT_CAM_CTRL_LIST));
        c.g[V0][0] = list;
        c.g[S0][0] = c.lw(list as u32);
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x4040_0000;
            c.mtc1(AT, 21);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 20);
            loop {
                c.cycles += 12;
                c.g[A0][0] = c.lw(r32(c, S0));
                SCRIPT_CAMERA_ALGO_GET_SCRIPT_OBJ.call(c, 0x0016_93a8)?;
                c.lwc1(2, r32(c, V0).wrapping_add(0x18));
                c.mul_s(1, 2, 2);
                c.add_s(0, 1, 1);
                c.mul_s(1, 1, 21);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 1, 0);
                c.c_lt_s(20, 1);
                let within = !c.fcc();
                c.swc1(2, r32(c, S0).wrapping_add(SCRIPT_CAM_NODE_T));
                if !within {
                    // The weight exceeds 1.0: clamp.
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 1);
                }
                c.cycles += 10;
                c.swc1(1, r32(c, S0).wrapping_add(SCRIPT_CAM_NODE_WEIGHT));
                c.g[A1][0] = c.lw(r32(c, S0));
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x28));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x2c));
                let method = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x0016_93f4)?;
                c.g[S0][0] = c.lw(r32(c, S0).wrapping_add(SCRIPT_CAM_NODE_NEXT));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0016_93a0);
                }
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputMap_CompileEntry (0x320e18)

/// `operator new(size, name, align, flags)` (0x317e30).
pub(crate) const OPERATOR_NEW_NAMED: Callee = Callee {
    addr: 0x0031_7e30,
    f: ee::s10::operator_new_named,
};
/// `EA_MemFree(ptr)` (0x317e98).
pub(crate) const EA_MEM_FREE_CALLEE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::EA_MemFree,
};
/// `cInputMapParser::compileStatement` (0x3245a8).
pub(crate) const INPUT_PARSER_COMPILE_IDENTIFIER: Callee = Callee {
    addr: 0x0032_45a8,
    f: ee::s10::InputParser_CompileIdentifier,
};
/// `EA_MemCopy(dst, src, bytes)` (0x3e6574; called through the table).
pub(crate) const EA_MEM_COPY_ADDR: u32 = 0x003e_6574;

/// InputMap fields: offsets table (+4), parser (+8), capacity (+0xc), used
/// (+0x10), code buffer (+0x14), all in words.
pub(crate) const INPUT_MAP_OFFSETS: u32 = 0x4;
pub(crate) const INPUT_MAP_PARSER: u32 = 0x8;
pub(crate) const INPUT_MAP_CAP: u32 = 0xc;
pub(crate) const INPUT_MAP_USED: u32 = 0x10;
pub(crate) const INPUT_MAP_CODE: u32 = 0x14;

/// `void InputMap::CompileEntry(this, idx, src)` (0x320e18): grow the opcode
/// buffer by 0x400 words when less than 0x100 remain, record the current
/// `used` as `offsets[idx]`, compile the statement at the end of the buffer
/// and add the words it produced to `used`.
pub fn input_map_compile_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CAP));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 256) as u64;
        let roomy = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A2][0];
        if !roomy {
            // Fewer than 0x100 words left: allocate a bigger buffer.
            c.cycles += 10;
            c.g[S2][0] = sx(r32(c, A0).wrapping_add(0x400));
            c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x22e0));
            c.g[A0][0] = sx(r32(c, S2) << 2);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            OPERATOR_NEW_NAMED.call(c, 0x0032_0e70)?;
            c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CODE));
            let no_old = c.g[A1][0] == 0;
            c.g[S1][0] = c.g[V0][0];
            let mut store_code = true;
            if !no_old {
                c.cycles += 7;
                c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CAP));
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = sx(r32(c, A2) << 2);
                call_ptr(c, EA_MEM_COPY_ADDR, 0x0032_0e8c)?;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CODE));
                if c.g[A0][0] == 0 {
                    let new_code = c.g[S1][0];
                    c.sw(r32(c, S0).wrapping_add(INPUT_MAP_CODE), new_code);
                    store_code = false;
                } else {
                    c.cycles += 2;
                    EA_MEM_FREE_CALLEE.call(c, 0x0032_0ea0)?;
                }
            }
            if store_code {
                c.cycles += 1;
                let new_code = c.g[S1][0];
                c.sw(r32(c, S0).wrapping_add(INPUT_MAP_CODE), new_code);
            }
            c.cycles += 1;
            let new_cap = c.g[S2][0];
            c.sw(r32(c, S0).wrapping_add(INPUT_MAP_CAP), new_cap);
        }
        c.cycles += 26;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_OFFSETS));
        c.g[V0][0] = sx(r32(c, S3) << 2);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        c.g[A1][0] = c.g[S4][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T0][0] = 5;
        let used = c.g[A0][0];
        c.sw(r32(c, V0), used);
        c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CAP));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_CODE));
        c.g[A2][0] = sx(r32(c, A3) << 2);
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_PARSER));
        c.g[A3][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
        c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        INPUT_PARSER_COMPILE_IDENTIFIER.call(c, 0x0032_0ee4)?;
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(INPUT_MAP_USED));
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.restore(c, S2, 0x30);
        let used = c.g[V1][0];
        c.sw(r32(c, S0).wrapping_add(INPUT_MAP_USED), used);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MagnetModifier_v21 (0x357538)

/// The initial-value quad the magnet modifier writes into both transform
/// rows of every contact it creates (`.rodata` at 0x4ff120).
#[cfg(test)]
pub(crate) const MAGNET_CONTACT_INIT_QUAD: u32 = 0x004f_f120;

/// `int cMagnetModifier::intersect(this, obj, contacts, ctx, flags)`
/// (0x357538): when `flags & 1`, copy the modifier's two vectors (+0x20,
/// +0x30; three floats each) to the stack and ask `obj`'s collision object
/// (vtable `+0x3c`, `this` adjusted by the halfword at `+0x38`) for its
/// contacts. Every contact (0x80 bytes, written into `contacts`) is then
/// initialised: both vector rows from the constant quad, state 1, and the
/// modifier's value at `this+0xa0` in `+0x50`. Returns the contact count.
pub fn magnet_modifier_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[T0][0] &= 1;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A2][0];
        let wanted = c.g[T0][0] != 0;
        c.g[T1][0] = 0;
        if wanted {
            c.cycles += 24;
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x50));
            c.g[T0][0] = c.g[A3][0];
            c.lwc1(5, r32(c, S0).wrapping_add(0x20));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(3, r32(c, S0).wrapping_add(0x24));
            c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
            c.lwc1(4, r32(c, S0).wrapping_add(0x28));
            c.lwc1(0, r32(c, S0).wrapping_add(0x30));
            c.lwc1(1, r32(c, S0).wrapping_add(0x34));
            c.lwc1(2, r32(c, S0).wrapping_add(0x38));
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x38));
            c.swc1(5, sp(c));
            c.swc1(3, sp(c).wrapping_add(0x4));
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.swc1(4, sp(c).wrapping_add(0x8));
            c.swc1(0, sp(c).wrapping_add(0x10));
            c.swc1(1, sp(c).wrapping_add(0x14));
            c.swc1(2, sp(c).wrapping_add(0x18));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x3c));
            let method = r32(c, V0);
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, method, 0x0035_75b4)?;
            c.g[T1][0] = c.g[V0][0];
            let none = (c.g[T1][0] as i64) <= 0;
            c.g[V0][0] = 0x0050_0000;
            if !none {
                c.cycles += 6;
                c.g[A2][0] = c.g[S1][0];
                c.g[T0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
                c.g[A3][0] = u64::MAX;
                c.g[T3][0] = 2;
                c.g[T2][0] = 1;
                c.g[A1][0] = c.g[T1][0];
                loop {
                    c.cycles += 27;
                    let contact = r32(c, A2);
                    let quad = r32(c, T0);
                    c.sw(contact.wrapping_add(0x50), 0);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    c.sw(contact.wrapping_add(0x5c), c.g[A3][0]);
                    c.sw(contact.wrapping_add(0x44), c.g[T3][0]);
                    c.sw(contact.wrapping_add(0x54), 0);
                    c.sw(contact.wrapping_add(0x58), 0);
                    c.sw(contact.wrapping_add(0x60), c.g[A3][0]);
                    c.sw(contact.wrapping_add(0x64), c.g[A3][0]);
                    c.sw(contact.wrapping_add(0x4c), c.g[A3][0]);
                    c.lq(2, quad);
                    c.sq(2, contact.wrapping_add(0x20));
                    c.lq(3, quad);
                    c.sw(contact.wrapping_add(0x68), 0);
                    c.sq(3, contact.wrapping_add(0x30));
                    c.sw(contact.wrapping_add(0x6c), 0);
                    c.sw(contact.wrapping_add(0x70), 0);
                    c.sw(contact.wrapping_add(0x44), c.g[T2][0]);
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xa0));
                    c.sw(contact.wrapping_add(0x5c), 0);
                    c.sw(contact.wrapping_add(0x50), c.g[V0][0]);
                    c.sw(contact.wrapping_add(0x48), 0);
                    c.lq(2, quad);
                    c.sq(2, contact.wrapping_add(0x20));
                    c.lq(3, quad);
                    c.sq(3, contact.wrapping_add(0x30));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(contact.wrapping_add(0x80));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_75d8);
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = c.g[T1][0];
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BEBoltInt_GetBoltTextureName (0x14b988)

/// `BELibrary_GetInstance` (0x0014dc80).
pub(crate) const BELIBRARY_GET_INSTANCE_CALL: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};
/// `BELibrary_GetInterface` (0x0014dd58).
pub(crate) const BELIBRARY_GET_INTERFACE_CALL: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};
/// `BE_GetCharProfileData` (0x0014ad28).
pub(crate) const BE_GET_CHAR_PROFILE_DATA_CALL: Callee = Callee {
    addr: 0x0014_ad28,
    f: ee::s01::BE_GetCharProfileData,
};
/// `Bolts_GetCharBoltCount` (0x0014d988).
pub(crate) const BOLTS_GET_CHAR_BOLT_COUNT_CALL: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};
/// `Bolts_GetCharBolts` (0x0014d998).
pub(crate) const BOLTS_GET_CHAR_BOLTS_CALL: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `int BEBoltInt::GetBoltTextureName(this, char, rider, const sBoltEntry* e0,
/// char* out)` (0x14b988): build the texture file name of a bolt. Copies
/// `e0->name` (+0x2c) to `out`; if it holds a `$` placeholder, fills the
/// placeholder characters from the names of the other bolts of the same
/// group (+3) that the rider has unlocked (profile flag 0x10). Returns 0
/// with an empty string when `e0` has no group (-1) or no name, else 1.
///
/// Mechanical port: the control flow keeps the generated block structure.
pub fn beb_bolt_int_get_bolt_texture_name(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        'b_14bb74: {
            'b_14b9e4: {
                'b_14b9d8: {
                    c.cycles += 17;
                    Frame::open(c, 0x90);
                    c.g[V0][0] = 0xffffffffffffffff;
                    frame.save(c, S0, 0x80);
                    frame.save(c, S1, 0x70);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S3, 0x50);
                    c.g[S1][0] = c.g[A2][0];
                    frame.save(c, S4, 0x40);
                    frame.save(c, S2, 0x60);
                    c.g[S4][0] = c.g[A3][0];
                    frame.save(c, S5, 0x30);
                    frame.save(c, S7, 0x10);
                    frame.save_ra(c, 0);
                    frame.save(c, S6, 0x20);
                    c.g[S6][0] = c.lb(r32(c, S4).wrapping_add(0x3));
                    let t = c.g[S6][0] == c.g[V0][0];
                    c.g[S3][0] = c.g[T0][0];
                    if t {
                        break 'b_14b9d8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(0x2c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_14b9e4;
                        } else {
                            break 'b_14b9d8;
                        }
                    }
                }
                c.cycles += 3;
                c.sb(r32(c, S3), 0u64);
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_14bb74;
                } else {
                    break 'b_14b9e4;
                }
            }
            c.cycles += 19;
            c.g[S7][0] = sx(r32(c, V0).wrapping_add(0x6750));
            BELIBRARY_GET_INSTANCE_CALL.call(c, 0x0014_b9ec)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x9;
            BELIBRARY_GET_INTERFACE_CALL.call(c, 0x0014_b9f8)?;
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            BE_GET_CHAR_PROFILE_DATA_CALL.call(c, 0x0014_ba08)?;
            c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(0x2c));
            c.g[A0][0] = c.g[S3][0];
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x0041_66f4, 0x0014_ba18)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x24;
            call_ptr(c, 0x0041_6404, 0x0014_ba24)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_14bb74;
            } else {
                'b_14bb70: {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[S1][0];
                    BOLTS_GET_CHAR_BOLT_COUNT_CALL.call(c, 0x0014_ba3c)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[S1][0];
                    BOLTS_GET_CHAR_BOLTS_CALL.call(c, 0x0014_ba4c)?;
                    c.g[T0][0] = c.g[V0][0];
                    let t = (c.g[S5][0] as i64) <= 0;
                    c.g[T3][0] = 0u64;
                    if t {
                        break 'b_14bb70;
                    } else {
                        c.cycles += 2;
                        c.g[T5][0] = 0x2e;
                        c.g[T4][0] = 0x24;
                        'l_14ba60: loop {
                            'b_14bb64: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb(r32(c, T0).wrapping_add(0x3));
                                if c.g[V0][0] != c.g[S6][0] {
                                    c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                    break 'b_14bb64;
                                } else {
                                    c.cycles += 3;
                                    c.g[T2][0] = c.lw(r32(c, T0).wrapping_add(0x2c));
                                    if c.g[T2][0] == 0u64 {
                                        c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                        break 'b_14bb64;
                                    } else {
                                        'b_14bb60: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lh(r32(c, T0).wrapping_add(0x4));
                                            c.g[V0][0] = c.lh(r32(c, S4).wrapping_add(0x4));
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = sx(r32(c, A0) << 1);
                                            if t {
                                                break 'b_14bb60;
                                            } else {
                                                'b_14baac: {
                                                    'b_14baa8: {
                                                        c.cycles += 5;
                                                        c.g[V1][0] =
                                                            c.lw(r32(c, S2).wrapping_add(0x288));
                                                        c.g[V0][0] =
                                                            sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                                        c.g[V0][0] = c.lh(r32(c, V0));
                                                        let t = (c.g[V0][0] as i64) < 0;
                                                        c.g[V0][0] = sx(r32(c, V0) << 2);
                                                        if t {
                                                            break 'b_14baa8;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                sx(r32(c, V0).wrapping_add(0x290));
                                                            let t = true;
                                                            c.g[V0][0] =
                                                                sx(r32(c, S2)
                                                                    .wrapping_add(r32(c, V0)));
                                                            if t {
                                                                break 'b_14baac;
                                                            } else {
                                                                break 'b_14baa8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V0][0] = 0u64;
                                                    break 'b_14baac;
                                                }
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lhu(r32(c, V0).wrapping_add(0x2));
                                                c.g[V0][0] = c.g[V0][0] & 0x10;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                                    break 'b_14bb64;
                                                } else {
                                                    'b_14bb14: {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lb(r32(c, S0));
                                                        c.g[A0][0] = c.g[S0][0];
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A2][0] =
                                                            sx(r32(c, S0).wrapping_sub(r32(c, S3)));
                                                        if t {
                                                            break 'b_14bb14;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == c.g[T5][0];

                                                            if t {
                                                                break 'b_14bb14;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A3][0] = 0x24;
                                                                c.g[T1][0] = 0x2e;
                                                                c.g[V0][0] = sx(r32(c, T2)
                                                                    .wrapping_add(r32(c, A2)));
                                                                'l_14bae0: loop {
                                                                    'b_14baf8: {
                                                                        c.cycles += 3;
                                                                        c.g[V1][0] =
                                                                            c.lb(r32(c, V0));
                                                                        let t = c.g[V1][0]
                                                                            == c.g[A3][0];
                                                                        c.g[A1][0] =
                                                                            c.lbu(r32(c, V0));
                                                                        if t {
                                                                            break 'b_14baf8;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] =
                                                                                c.lb(r32(c, A0));
                                                                            if c.g[V0][0]
                                                                                == c.g[A3][0]
                                                                            {
                                                                                c.sb(
                                                                                    r32(c, A0),
                                                                                    c.g[A1][0],
                                                                                );
                                                                                break 'b_14baf8;
                                                                            } else {
                                                                                break 'b_14baf8;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_14bb10: {
                                                                        c.cycles += 4;
                                                                        c.g[A0][0] = sx(r32(c, A0)
                                                                            .wrapping_add(0x1));
                                                                        c.g[V0][0] =
                                                                            c.lb(r32(c, A0));
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A2][0] = sx(r32(c, A2)
                                                                            .wrapping_add(0x1));
                                                                        if t {
                                                                            break 'b_14bb10;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V0][0]
                                                                                != c.g[T1][0];
                                                                            c.g[V0][0] =
                                                                                sx(r32(c, T2)
                                                                                    .wrapping_add(
                                                                                        r32(c, A2),
                                                                                    ));
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0014_bae0);
                                                                                }
                                                                                continue 'l_14bae0;
                                                                            } else {
                                                                                break 'b_14bb10;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c.lb(r32(c, S0));
                                                                    break 'b_14bb14;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_14bb58: {
                                                        c.cycles += 2;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] = c.lb(r32(c, S0));
                                                            break 'b_14bb58;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[T4][0] {
                                                                c.g[T3][0] =
                                                                    sx(r32(c, T3)
                                                                        .wrapping_add(0x1));
                                                                break 'b_14bb64;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[T5][0] {
                                                                    c.g[V0][0] = c.lb(r32(c, S0));
                                                                    break 'b_14bb58;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x24;
                                                                    c.g[V1][0] = 0x2e;
                                                                    c.g[S0][0] = sx(r32(c, S0)
                                                                        .wrapping_add(0x1));
                                                                    'l_14bb38: loop {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] =
                                                                            c.lb(r32(c, S0));
                                                                        let t = c.g[V0][0] == 0u64;

                                                                        if t {
                                                                            break 'b_14bb58;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[A0][0]
                                                                            {
                                                                                c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                                                                break 'b_14bb64;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[V1][0]
                                                                                {
                                                                                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1));
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x0014_bb38);
                                                                                    }
                                                                                    continue 'l_14bb38;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    c.g[V0][0] = c
                                                                                        .lb(r32(
                                                                                            c, S0,
                                                                                        ));
                                                                                    break 'b_14bb58;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != c.g[T4][0];
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_14bb74;
                                                    } else {
                                                        break 'b_14bb60;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                        break 'b_14bb64;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x38));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_ba60);
                                }
                                continue 'l_14ba60;
                            } else {
                                break 'b_14bb70;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_14bb74;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptList_GetPlayStatus (0x00276bd8)

/// `ScriptCmdQueue_At` (0x00283d70).
pub(crate) const SCRIPT_CMD_QUEUE_AT_CALL: Callee = Callee {
    addr: 0x0028_3d70,
    f: ee::s07::ScriptCmdQueue_At,
};
/// `ScriptEngine_IsProcessFinished` (0x00282838).
pub(crate) const SCRIPT_ENGINE_IS_PROCESS_FINISHED_CALL: Callee = Callee {
    addr: 0x0028_2838,
    f: ee::s07::ScriptEngine_IsProcessFinished,
};

/// `int ScriptList::GetPlayStatus(this)` (0x276bd8): the play state of a script list. 0 unless the state (+0xa4) is 3 (playing); 3 when +0xc4 is set (paused); otherwise, when +0xc8 is clear, look at the first queued command: if its flag 8 is set the list waits (2 with no wait object at +0xb0, or while the count at +0xb8 is above 1), else if the script process of its id (+0xc) has not finished the status is 1. The rest is 4 or, with a negative +0xbc, 5.
pub fn script_list_get_play_status(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        'b_276c98: {
            'b_276c94: {
                c.cycles += 9;
                Frame::open(c, 0x30);
                c.g[V1][0] = 0x3;
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xa4));
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_276c94;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_276c94;
                    } else {
                        'b_276c80: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc8));
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0xbc));
                                break 'b_276c80;
                            } else {
                                'b_276c7c: {
                                    'b_276c58: {
                                        c.cycles += 8;
                                        c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xc));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.g[S1][0];
                                        SCRIPT_CMD_QUEUE_AT_CALL.call(c, 0x0027_6c24)?;
                                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x8));
                                        c.g[V1][0] = c.g[V1][0] & 0x8;
                                        if c.g[V1][0] == 0u64 {
                                            c.g[A0][0] = c.g[S1][0];
                                            break 'b_276c58;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xb0));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_276c94;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xb8));
                                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_276c7c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    frame.restore(c, S0, 0x20);
                                                    if t {
                                                        break 'b_276c98;
                                                    } else {
                                                        break 'b_276c58;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    SCRIPT_CMD_QUEUE_AT_CALL.call(c, 0x0027_6c60)?;
                                    c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
                                    c.g[A0][0] = c.lw(r32(c, S0));
                                    SCRIPT_ENGINE_IS_PROCESS_FINISHED_CALL.call(c, 0x0027_6c6c)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0xbc));
                                        break 'b_276c80;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_276c94;
                                        } else {
                                            break 'b_276c7c;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0xbc));
                                break 'b_276c80;
                            }
                        }
                        c.cycles += 5;
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = 0x4;
                        c.g[V0][0] = 0x5;
                        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = c.g[A0][0];
                        }
                        break 'b_276c94;
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            break 'b_276c98;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_SpanTrackUpdate (0x00272f28)

/// `InSeq_SpanTrackActivateKey` (0x002730c0).
pub(crate) const IN_SEQ_SPAN_TRACK_ACTIVATE_KEY_CALL: Callee = Callee {
    addr: 0x0027_30c0,
    f: ee::s07::InSeq_SpanTrackActivateKey,
};
/// `InSeq_SpanTrackNextKey` (0x00273140).
pub(crate) const IN_SEQ_SPAN_TRACK_NEXT_KEY_CALL: Callee = Callee {
    addr: 0x0027_3140,
    f: ee::s07::InSeq_SpanTrackNextKey,
};

/// `int InSeq::SpanTrackUpdate(this, flags*, time)` (0x272f28): retire the expired cached events (`end <= time`: unlink, flag 8, decrement the count at +0x28), then activate and advance every key record whose time has come (flag 4) until the next one lies in the future. Returns 1 while events or a pending record remain, else 0. `time` is passed in f12.
pub fn in_seq_span_track_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        'b_272fb4: {
            c.cycles += 12;
            Frame::open(c, 0x40);
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = c.g[A0][0];
            frame.save_fpr(c, 20, 0x30);
            c.g[S1][0] = c.g[A1][0];
            frame.save_ra(c, 0);
            c.mov_s(20, 12);
            frame.save_fpr(c, 21, 0x38);
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x24));
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_272fb4;
            } else {
                'l_272f58: loop {
                    'b_272fac: {
                        'b_272fa8: {
                            c.cycles += 6;
                            c.lwc1(0, r32(c, V1).wrapping_add(0xc));
                            c.cvt_s_w(0, 0);
                            c.c_le_s(0, 20);
                            if !c.fcc() {
                                c.g[A0][0] = c.g[V1][0];
                                break 'b_272fa8;
                            } else {
                                'b_272f98: {
                                    'b_272f90: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw(r32(c, S1));
                                        c.g[V0][0] = c.g[V0][0] | 0x8;
                                        let t = c.g[A0][0] != 0u64;
                                        c.sw(r32(c, S1), c.g[V0][0]);
                                        if t {
                                            break 'b_272f90;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                                            c.g[V1][0] = c.g[V0][0];
                                            let t = true;
                                            c.sw(r32(c, S0).wrapping_add(0x24), c.g[V0][0]);
                                            if t {
                                                break 'b_272f98;
                                            } else {
                                                break 'b_272f90;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                                    c.sw(r32(c, A0).wrapping_add(0x18), c.g[V1][0]);
                                    break 'b_272f98;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x28));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                                let t = true;
                                c.sw(r32(c, S0).wrapping_add(0x28), c.g[V0][0]);
                                if t {
                                    break 'b_272fac;
                                } else {
                                    break 'b_272fa8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                        break 'b_272fac;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_2f58);
                        }
                        continue 'l_272f58;
                    } else {
                        break 'b_272fb4;
                    }
                }
            }
        }
        c.cycles += 3;
        c.mtc1(0, 21);
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        'l_272fc0: loop {
            'b_273010: {
                c.cycles += 2;
                if c.g[V1][0] == 0u64 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x24));
                    break 'b_273010;
                } else {
                    'b_27300c: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lhu(r32(c, V1).wrapping_add(0x4));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sub_s(0, 20, 0);
                        c.c_le_s(21, 0);
                        let t = !c.fcc();
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_27300c;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(r32(c, S1));
                            c.g[V0][0] = c.g[V0][0] | 0x4;
                            c.sw(r32(c, S1), c.g[V0][0]);
                            c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
                            IN_SEQ_SPAN_TRACK_ACTIVATE_KEY_CALL.call(c, 0x0027_2ffc)?;
                            c.g[A0][0] = c.g[S0][0];
                            IN_SEQ_SPAN_TRACK_NEXT_KEY_CALL.call(c, 0x0027_3004)?;
                            let t = true;
                            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0027_2fc0);
                                }
                                continue 'l_272fc0;
                            } else {
                                break 'b_27300c;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x24));
                    break 'b_273010;
                }
            }
            'b_273024: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_273024;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_273024;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x1;
                        break 'b_273024;
                    }
                }
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.restore_fpr(c, 21, 0x38);
            frame.restore_fpr(c, 20, 0x30);
            frame.close(c);
            return Ok(());
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CameraTrigger_ReadAction (0x001712b0)

/// `CameraTrigger_ReadActionType0` (0x001715a0).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE0_CALL: Callee = Callee {
    addr: 0x0017_15a0,
    f: ee::s02::CameraTrigger_ReadActionType0,
};
/// `CameraTrigger_ReadActionType1` (0x001714e8).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE1_CALL: Callee = Callee {
    addr: 0x0017_14e8,
    f: ee::s02::CameraTrigger_ReadActionType1,
};
/// `CameraTrigger_ReadActionType2` (0x001715f8).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE2_CALL: Callee = Callee {
    addr: 0x0017_15f8,
    f: ee::s02::CameraTrigger_ReadActionType2,
};
/// `CameraTrigger_ReadActionType3` (0x00171680).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE3_CALL: Callee = Callee {
    addr: 0x0017_1680,
    f: ee::s02::CameraTrigger_ReadActionType3,
};

/// `int CameraTrigger::ReadAction(reader, out**)` (0x1712b0): read the action type (`get_uint`), allocate the matching action object (switch 0x10 bytes, bounded 0x34, spline 0x20, none 8) with its default fields, let the type-specific reader fill it, store it in `*out` and return the bytes consumed. Unknown types read nothing.
pub fn camera_trigger_read_action(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        'b_1714c8: {
            'b_1714bc: {
                'b_171480: {
                    'b_171414: {
                        'b_171370: {
                            c.cycles += 16;
                            Frame::open(c, 0x70);
                            frame.save(c, S3, 0x30);
                            frame.save(c, S4, 0x20);
                            c.g[S3][0] = c.g[A0][0];
                            c.g[S4][0] = c.g[A1][0];
                            frame.save(c, S1, 0x50);
                            frame.save(c, S2, 0x40);
                            frame.save(c, S0, 0x60);
                            frame.save_ra(c, 0x10);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x002d_2558, 0x0017_12dc)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[S1][0] = c.lw(r32(c, SP));
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_171370;
                            } else {
                                'b_171320: {
                                    'b_171308: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_171308;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_171320;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x60);
                                                if t {
                                                    break 'b_1714c8;
                                                } else {
                                                    break 'b_171308;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_171414;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_171480;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            frame.restore(c, S0, 0x60);
                                            if t {
                                                break 'b_1714c8;
                                            } else {
                                                break 'b_171320;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0x10;
                                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f50));
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x0031_7d70, 0x0017_1338)?;
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 0);
                                c.g[V1][0] = 0x460000;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3d18));
                                c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                                c.g[A0][0] = c.g[S3][0];
                                c.swc1(0, r32(c, S0).wrapping_add(0xc));
                                c.g[A1][0] = c.g[S0][0];
                                c.sw(r32(c, S0), 0u64);
                                c.sw(r32(c, S0).wrapping_add(0x8), 0u64);
                                CAMERA_TRIGGER_READ_ACTION_TYPE0_CALL.call(c, 0x0017_1368)?;
                                let t = true;
                                c.sw(r32(c, S4), c.g[S0][0]);
                                if t {
                                    break 'b_1714bc;
                                } else {
                                    break 'b_171370;
                                }
                            }
                        }
                        c.cycles += 41;
                        c.g[A1][0] = 0x460000;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x34;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f38));
                        call_ptr(c, 0x0031_7d70, 0x0017_1388)?;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 3);
                        c.g[V1][0] = 0x460000;
                        c.lwc1(4, r32(c, GP).wrapping_sub(0x643c));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[AT][0] = 0x43c8_0000;
                        c.mtc1(1, 2);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3e30));
                        c.g[AT][0] = 0x41c8_0000;
                        c.mtc1(1, 1);
                        c.g[V0][0] = 0x500000;
                        c.g[AT][0] = 0x428c_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.g[S3][0];
                        c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                        c.g[A1][0] = c.g[S0][0];
                        c.sw(r32(c, S0), c.g[S1][0]);
                        c.swc1(3, r32(c, S0).wrapping_add(0xc));
                        c.swc1(4, r32(c, S0).wrapping_add(0x14));
                        c.swc1(2, r32(c, S0).wrapping_add(0x10));
                        c.swc1(1, r32(c, S0).wrapping_add(0x1c));
                        c.swc1(0, r32(c, S0).wrapping_add(0x18));
                        c.sw(r32(c, S0).wrapping_add(0x8), 0u64);
                        c.sw(r32(c, S0).wrapping_add(0x24), 0u64);
                        c.g[T0][0] = sx(r32(c, V0).wrapping_sub(0xf28));
                        c.ldl(3, r32(c, T0).wrapping_add(0x7));
                        c.ldr(3, r32(c, T0));
                        c.g[A2][0] = c.lw(r32(c, T0).wrapping_add(0x8));
                        c.sdl(3, r32(c, S0).wrapping_add(0x2f));
                        c.sdr(3, r32(c, S0).wrapping_add(0x28));
                        c.sw(r32(c, S0).wrapping_add(0x30), c.g[A2][0]);
                        c.sw(r32(c, S0).wrapping_add(0x20), 0u64);
                        CAMERA_TRIGGER_READ_ACTION_TYPE1_CALL.call(c, 0x0017_140c)?;
                        let t = true;
                        c.sw(r32(c, S4), c.g[S0][0]);
                        if t {
                            break 'b_1714bc;
                        } else {
                            break 'b_171414;
                        }
                    }
                    c.cycles += 27;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x20;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f20));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x0031_7d70, 0x0017_142c)?;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.g[V1][0] = 0x460000;
                    c.lwc1(2, r32(c, GP).wrapping_sub(0x6438));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[AT][0] = 0x4080_0000;
                    c.mtc1(1, 0);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3e48));
                    c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(r32(c, S0), c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.swc1(1, r32(c, S0).wrapping_add(0xc));
                    c.swc1(2, r32(c, S0).wrapping_add(0x10));
                    c.swc1(0, r32(c, S0).wrapping_add(0x18));
                    c.sw(r32(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r32(c, S0).wrapping_add(0x14), 0u64);
                    c.sw(r32(c, S0).wrapping_add(0x1c), 0u64);
                    CAMERA_TRIGGER_READ_ACTION_TYPE2_CALL.call(c, 0x0017_1478)?;
                    let t = true;
                    c.sw(r32(c, S4), c.g[S0][0]);
                    if t {
                        break 'b_1714bc;
                    } else {
                        break 'b_171480;
                    }
                }
                c.cycles += 15;
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = 0x8;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f08));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x0031_7d70, 0x0017_1498)?;
                c.g[V1][0] = 0x460000;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3d18));
                c.sw(r32(c, S0), c.g[S1][0]);
                c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                CAMERA_TRIGGER_READ_ACTION_TYPE3_CALL.call(c, 0x0017_14b8)?;
                c.sw(r32(c, S4), c.g[S0][0]);
                break 'b_1714bc;
            }
            c.cycles += 3;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.g[V0][0] = c.g[S2][0];
            frame.restore(c, S0, 0x60);
            break 'b_1714c8;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputParser_EmitUnaryOp (0x003250d8)

/// `InputParser_EmitNode` (0x00324678).
pub(crate) const INPUT_PARSER_EMIT_NODE_CALL: Callee = Callee {
    addr: 0x0032_4678,
    f: ee::s10::InputParser_EmitNode,
};
/// `InputParser_Error` (0x00321500).
pub(crate) const INPUT_PARSER_ERROR_CALL: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};

/// `int InputParser::EmitUnaryOp(this, node, dst*, code, cap, op, modifier)` (0x3250d8): compile the operand of a unary node, return its source register to the free-register stack, allocate a destination register when `*dst < 0`, then emit one opcode word (op, dst, src, modifier) into the buffer. Returns the number of words used (-1 after an error or when the buffer overflows).
pub fn input_parser_emit_unary_op(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        'b_32522c: {
            c.cycles += 21;
            Frame::open(c, 0x70);
            c.g[V0][0] = 0xffffffffffffffff;
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S2, 0x40);
            c.g[S1][0] = c.g[T0][0];
            frame.save(c, S3, 0x30);
            c.g[S2][0] = c.g[A3][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A2][0];
            frame.save_ra(c, 0x10);
            c.g[S4][0] = c.g[T1][0];
            c.g[T1][0] = c.g[T2][0];
            c.g[A1][0] = c.lw(r32(c, A1).wrapping_add(0x4));
            c.sw(r32(c, SP), c.g[V0][0]);
            c.g[A2][0] = c.g[SP][0];
            INPUT_PARSER_EMIT_NODE_CALL.call(c, 0x0032_5120)?;
            c.g[T0][0] = c.g[V0][0];
            let t = (c.g[T0][0] as i64) < 0;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_32522c;
            } else {
                'b_325164: {
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, T0) << 2);
                    c.g[A1][0] = c.lw(r32(c, SP));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = (c.g[A1][0] < 0x40u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(r32(c, T0)));
                    if t {
                        break 'b_325164;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = 0x10000;
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x9e8));
                        c.g[V0][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1));
                        c.sw(r32(c, V0).wrapping_add(0x8ec), c.g[A1][0]);
                        c.sw(r32(c, A0).wrapping_add(0x9e8), c.g[V1][0]);
                        break 'b_325164;
                    }
                }
                'b_325220: {
                    'b_3251ac: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, S3));
                        let t = (c.g[V0][0] as i64) >= 0;
                        c.g[V0][0] = 0x10000;
                        if t {
                            break 'b_3251ac;
                        } else {
                            'b_325194: {
                                c.cycles += 4;
                                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x9e8));
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                                if t {
                                    break 'b_325194;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = sx(r32(c, V0) << 2);
                                    c.sw(r32(c, A0).wrapping_add(0x9e8), c.g[V0][0]);
                                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x8ec));
                                    c.sw(r32(c, S3), c.g[V0][0]);
                                    break 'b_325194;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(r32(c, S3));
                            let t = (c.g[V0][0] as i64) >= 0;
                            c.g[A1][0] = 0x490000;
                            if t {
                                break 'b_3251ac;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S0][0];
                                let t = true;
                                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1dc0));
                                if t {
                                    break 'b_325220;
                                } else {
                                    break 'b_3251ac;
                                }
                            }
                        }
                    }
                    'b_325214: {
                        c.cycles += 2;
                        let t = (c.g[S1][0] as i64) <= 0;
                        c.g[V0][0] = 0xffffffffffffffc0;
                        if t {
                            break 'b_325214;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(r32(c, S2));
                            c.g[A0][0] = c.g[S4][0] & 0x3f;
                            c.g[A2][0] = c.lw(r32(c, SP));
                            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                            c.g[A1][0] = 0xfffffffffffff03f;
                            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                            c.g[A3][0] = 0xffffffffffc00000;
                            c.sw(r32(c, S2), c.g[V1][0]);
                            c.g[A2][0] = c.g[A2][0] & 0x3ff;
                            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                            c.g[A3][0] = c.g[A3][0] | 0xfff;
                            c.g[A0][0] = c.lw(r32(c, S3));
                            c.g[A2][0] = sx(r32(c, A2) << 12);
                            c.g[A1][0] = 0x3f0000;
                            c.g[V0][0] = sx(r32(c, T0).wrapping_add(0x1));
                            c.g[A0][0] = c.g[A0][0] & 0x3f;
                            c.g[A1][0] = c.g[A1][0] | 0xffff;
                            c.g[A0][0] = sx(r32(c, A0) << 6);
                            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                            c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                            c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
                            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                            let t = true;
                            c.sw(r32(c, S2), c.g[V1][0]);
                            if t {
                                break 'b_32522c;
                            } else {
                                break 'b_325214;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1dd8));
                    break 'b_325220;
                }
                c.cycles += 3;
                INPUT_PARSER_ERROR_CALL.call(c, 0x0032_5228)?;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_32522c;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdMan2d_BindInstanceFrame (0x00229820)

/// `void CrowdMan2d::BindInstanceFrame(this, handle, part, bone)` (0x229820): resolve the model of an instance `handle` and, if the part and bone indices are in range, store the 0x536690 constant as the pose entry of that bone.
pub fn crowd_man2d_bind_instance_frame(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        'b_229890: {
            'b_22986c: {
                c.cycles += 17;
                Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A2][0];
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A3][0];
                call_ptr(c, 0x002d_1bd8, 0x0022_9844)?;
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = c.g[S0][0] & 0xff;
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                if c.g[V1][0] != 0u64 {
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    break 'b_22986c;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_229890;
                    } else {
                        break 'b_22986c;
                    }
                }
            }
            'b_22988c: {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, S0) >> 8);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                if t {
                    break 'b_22988c;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_22988c;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
            break 'b_229890;
        }
        'b_2298f8: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            frame.restore(c, S0, 0x30);
            if t {
                break 'b_2298f8;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x80));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x94));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x8));
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[A3][0] = sx(r32(c, A0).wrapping_add(0x4));
                if t {
                    break 'b_2298f8;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, S1) << 4);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
                    c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = sx(r32(c, S2) << 2);
                    if t {
                        break 'b_2298f8;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = c.lw(r32(c, V0).wrapping_add(0x20));
                        c.g[A1][0] = 0x530000;
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x6690));
                        c.g[A0][0] = c.lw(r32(c, V1));
                        c.g[V0][0] = c.lh(r32(c, A0));
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
                        c.sw(r32(c, V0), c.g[A1][0]);
                        frame.restore(c, S0, 0x30);
                        break 'b_2298f8;
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_InstanceAdvance (0x002751a0)

/// `InSeq_ChannelAdvance` (0x00274100).
pub(crate) const IN_SEQ_CHANNEL_ADVANCE_CALL: Callee = Callee {
    addr: 0x0027_4100,
    f: ee::s07::InSeq_ChannelAdvance,
};
/// `InSeq_InstanceGetListener` (0x00274d70).
pub(crate) const IN_SEQ_INSTANCE_GET_LISTENER_CALL: Callee = Callee {
    addr: 0x0027_4d70,
    f: ee::s07::InSeq_InstanceGetListener,
};

/// `int InSeq::InstanceAdvance(this, time, prev, speed)` (0x2751a0): store the new time (f12, at +0x28), decide whether the play head jumped (speed above 1 or the integer part of the time changed), advance every channel (0x3c bytes each), and notify the listener (vtable +0x14) of the new time and speed. Returns 1 when any channel changed.
pub fn in_seq_instance_advance(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        'b_27522c: {
            'b_275228: {
                'b_2751f8: {
                    c.cycles += 20;
                    Frame::open(c, 0x80);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    frame.save_fpr(c, 21, 0x78);
                    c.mov_s(21, 14);
                    frame.save(c, S0, 0x60);
                    c.c_lt_s(0, 21);
                    frame.save(c, S4, 0x20);
                    frame.save(c, S5, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_fpr(c, 20, 0x70);
                    c.g[S5][0] = 0u64;
                    frame.save(c, S1, 0x50);
                    c.mov_s(20, 13);
                    frame.save(c, S2, 0x40);
                    c.g[S4][0] = 0u64;
                    frame.save(c, S3, 0x30);
                    frame.save_ra(c, 0);
                    let t = !c.fcc();
                    c.swc1(12, r32(c, S0).wrapping_add(0x28));
                    if t {
                        break 'b_2751f8;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[S4][0] = 0x1;
                        if t {
                            break 'b_275228;
                        } else {
                            break 'b_2751f8;
                        }
                    }
                }
                c.cycles += 10;
                c.cvt_w_s(0, 12);
                c.mfc1(2, 0);
                c.cvt_w_s(0, 20);
                c.mfc1(3, 0);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.c_eq_s(0, 12);
                if c.fcc() {
                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                    break 'b_27522c;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0] ^ c.g[V0][0];
                    c.g[S4][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_275228;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
            break 'b_27522c;
        }
        'b_275290: {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x1));
            let t = c.g[V0][0] == 0u64;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_275290;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0u64;
                'l_275240: loop {
                    'b_275278: {
                        'b_275274: {
                            c.cycles += 10;
                            c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x18));
                            c.g[S2][0] = 0u64;
                            c.lwc1(13, r32(c, S0).wrapping_add(0x28));
                            c.mov_s(12, 20);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                            c.mov_s(14, 21);
                            c.g[A1][0] = c.g[S4][0];
                            IN_SEQ_CHANNEL_ADVANCE_CALL.call(c, 0x0027_5260)?;
                            if c.g[V0][0] != 0u64 {
                                c.g[S2][0] = 0x1;
                                break 'b_275274;
                            } else {
                                c.cycles += 2;
                                if c.g[S5][0] == 0u64 {
                                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                                    break 'b_275278;
                                } else {
                                    c.cycles += 1;
                                    c.g[S2][0] = 0x1;
                                    break 'b_275274;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                        break 'b_275278;
                    }
                    c.cycles += 6;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1));
                    c.g[S5][0] = c.g[S2][0];
                    c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x3c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_5240);
                        }
                        continue 'l_275240;
                    } else {
                        break 'b_275290;
                    }
                }
            }
        }
        'b_2752c0: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            IN_SEQ_INSTANCE_GET_LISTENER_CALL.call(c, 0x0027_5298)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S0][0];
            if t {
                break 'b_2752c0;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw(r32(c, A2));
                c.lwc1(12, r32(c, A1).wrapping_add(0x28));
                c.mov_s(13, 21);
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x10));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0027_52c0)?;
                break 'b_2752c0;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S5][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_GetEquippedBolts (0x0011bbe8)

/// `int Rider::GetEquippedBolts(this, list**)` (0x11bbe8): fill `list` with the bolt entries (0x38 bytes) of the rider that count as equipped: all entries of the cheat character if there is one, else those whose profile flag 0x10 is set. Returns the count.
pub fn rider_get_equipped_bolts(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        'b_11bd34: {
            'b_11bca4: {
                c.cycles += 26;
                Frame::open(c, 0x80);
                frame.save(c, S3, 0x40);
                frame.save(c, S6, 0x10);
                c.g[S3][0] = c.g[A0][0];
                frame.save(c, S0, 0x70);
                c.g[S6][0] = c.g[A1][0];
                frame.save(c, S1, 0x60);
                frame.save(c, S2, 0x50);
                frame.save(c, S4, 0x30);
                frame.save(c, S5, 0x20);
                frame.save_ra(c, 0);
                c.g[S4][0] = 0u64;
                call_ptr(c, 0x0014_dc80, 0x0011_bc1c)?;
                c.g[A1][0] = 0x9;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x0011_bc28)?;
                c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x86c));
                c.g[S5][0] = c.g[V0][0];
                call_ptr(c, 0x0014_a080, 0x0011_bc34)?;
                c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x86c));
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0014_a0b0, 0x0011_bc40)?;
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_bdb8, 0x0011_bc48)?;
                let t = c.g[S0][0] == 0u64;
                c.g[S1][0] = c.g[V0][0];
                if t {
                    break 'b_11bca4;
                } else {
                    c.cycles += 10;
                    c.g[S2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0014_d998, 0x0011_bc60)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0014_d988, 0x0011_bc70)?;
                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[V0][0] = c.g[S4][0];
                        break 'b_11bd34;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S6][0];
                        'l_11bc80: loop {
                            c.cycles += 7;
                            c.sw(r32(c, A1), c.g[S0][0]);
                            c.g[S4][0] = sx(r32(c, S4).wrapping_add(0x1));
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
                            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x38));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0011_bc80);
                                }
                                continue 'l_11bc80;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = c.g[S4][0];
                                if t {
                                    break 'b_11bd34;
                                } else {
                                    break 'b_11bca4;
                                }
                            }
                        }
                    }
                }
            }
            'b_11bd30: {
                c.cycles += 13;
                c.g[A1][0] = c.lw(r32(c, S3).wrapping_add(0x86c));
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x0014_ad50, 0x0011_bcb0)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[S3][0] = c.g[V0][0];
                call_ptr(c, 0x0014_d988, 0x0011_bcc0)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0014_d998, 0x0011_bcd0)?;
                let t = (c.g[S0][0] as i64) <= 0;
                c.g[A0][0] = c.g[V0][0];
                if t {
                    break 'b_11bd30;
                } else {
                    c.cycles += 2;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S6][0];
                    'l_11bce0: loop {
                        'b_11bd0c: {
                            'b_11bd08: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lh(r32(c, A0).wrapping_add(0x4));
                                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x288));
                                c.g[V0][0] = sx(r32(c, V0) << 1);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                c.g[V0][0] = c.lh(r32(c, V0));
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[V0][0] = sx(r32(c, V0) << 2);
                                if t {
                                    break 'b_11bd08;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x290));
                                    let t = true;
                                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                                    if t {
                                        break 'b_11bd0c;
                                    } else {
                                        break 'b_11bd08;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0u64;
                            break 'b_11bd0c;
                        }
                        'b_11bd28: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lhu(r32(c, V0).wrapping_add(0x2));
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x1));
                            if t {
                                break 'b_11bd28;
                            } else {
                                c.cycles += 3;
                                c.sw(r32(c, A1), c.g[A0][0]);
                                c.g[S4][0] = sx(r32(c, S4).wrapping_add(0x1));
                                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4));
                                break 'b_11bd28;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[A2][0] != 0u64;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x38));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0011_bce0);
                            }
                            continue 'l_11bce0;
                        } else {
                            break 'b_11bd30;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
            break 'b_11bd34;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_CreateFlag (0x002fc9c8)

/// `WorldObjFactory_AttachModifier` (0x002fae38).
pub(crate) const WORLD_OBJ_FACTORY_ATTACH_MODIFIER_CALL: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};
/// `EA_MemAllocNamed` (0x00317d70).
pub(crate) const EA_MEM_ALLOC_NAMED_CALL: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// `WScriptFn::CreateFlag(entries, n)` (0x2fc9c8): build the 108-byte flag template (static, initialised once), apply the `n` (idx, type, value) overrides (floats converted when the slot is a float), resolve the target object (the world object of the id in the template, or the default one), and unless it already has an active node of kind 10, allocate a 0x1c-byte flag node on it. Returns a constant.
pub fn w_script_fn_create_flag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        'b_2fca98: {
            c.cycles += 13;
            Frame::open(c, 0xe0);
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0x2500));
            frame.save(c, S4, 0x90);
            frame.save(c, S0, 0xd0);
            c.g[S4][0] = c.g[A2][0];
            frame.save(c, S2, 0xb0);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S1, 0xc0);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S3, 0xa0);
            c.g[A2][0] = 0x500000;
            let t = c.g[V0][0] != 0u64;
            frame.save_ra(c, 0x80);
            if t {
                break 'b_2fca98;
            } else {
                c.cycles += 39;
                c.g[V0][0] = sx(r32(c, A2).wrapping_sub(0x4ac0));
                c.g[V1][0] = 0xffffffffffff0000;
                c.mtc1(0, 1);
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[A0][0] = 0x1;
                c.g[A1][0] = c.lbu(r32(c, V0).wrapping_add(0x68));
                c.g[AT][0] = 0x41c8_0000;
                c.mtc1(1, 2);
                c.sw(r32(c, A2).wrapping_sub(0x4ac0), c.g[V1][0]);
                c.sb(r32(c, V0).wrapping_add(0x68), c.g[A1][0]);
                c.swc1(2, r32(c, V0).wrapping_add(0x14));
                c.swc1(0, r32(c, V0).wrapping_add(0x3c));
                c.swc1(1, r32(c, V0).wrapping_add(0x60));
                c.sw(r32(c, GP).wrapping_add(0x2500), c.g[A0][0]);
                c.sw(r32(c, V0).wrapping_add(0x4), c.g[A0][0]);
                c.sw(r32(c, V0).wrapping_add(0x8), 0u64);
                c.sw(r32(c, V0).wrapping_add(0xc), 0u64);
                c.swc1(0, r32(c, V0).wrapping_add(0x10));
                c.swc1(0, r32(c, V0).wrapping_add(0x18));
                c.swc1(0, r32(c, V0).wrapping_add(0x1c));
                c.swc1(1, r32(c, V0).wrapping_add(0x20));
                c.swc1(0, r32(c, V0).wrapping_add(0x24));
                c.swc1(0, r32(c, V0).wrapping_add(0x28));
                c.swc1(1, r32(c, V0).wrapping_add(0x2c));
                c.swc1(0, r32(c, V0).wrapping_add(0x30));
                c.swc1(0, r32(c, V0).wrapping_add(0x34));
                c.swc1(1, r32(c, V0).wrapping_add(0x38));
                c.swc1(1, r32(c, V0).wrapping_add(0x40));
                c.swc1(1, r32(c, V0).wrapping_add(0x44));
                c.swc1(1, r32(c, V0).wrapping_add(0x48));
                c.swc1(1, r32(c, V0).wrapping_add(0x4c));
                c.swc1(1, r32(c, V0).wrapping_add(0x50));
                c.swc1(1, r32(c, V0).wrapping_add(0x54));
                c.swc1(1, r32(c, V0).wrapping_add(0x58));
                c.swc1(1, r32(c, V0).wrapping_add(0x5c));
                c.sw(r32(c, V0).wrapping_add(0x64), 0u64);
                c.sb(r32(c, SP), c.g[A1][0]);
                break 'b_2fca98;
            }
        }
        'b_2fcb20: {
            c.cycles += 11;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S1][0] = sx(r32(c, SP).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx(r32(c, A2).wrapping_sub(0x4ac0));
            c.g[A2][0] = 0x6c;
            c.sw(r32(c, SP).wrapping_add(0x10), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = 0x4d0000;
            call_ptr(c, 0x003e_6574, 0x002f_cabc)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S4][0];
            if t {
                break 'b_2fcb20;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S0][0];
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0x5fc0));
                c.g[A3][0] = 0x2;
                'l_2fcad8: loop {
                    'b_2fcb14: {
                        'b_2fcb10: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(r32(c, A2));
                            c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xc));
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                            c.g[V0][0] = c.lw(r32(c, V0));
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                                break 'b_2fcb10;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                                    break 'b_2fcb10;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, r32(c, A2).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, r32(c, A0));
                                    if t {
                                        break 'b_2fcb14;
                                    } else {
                                        break 'b_2fcb10;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(r32(c, A0), c.g[V0][0]);
                        break 'b_2fcb14;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f_cad8);
                        }
                        continue 'l_2fcad8;
                    } else {
                        break 'b_2fcb20;
                    }
                }
            }
        }
        'b_2fcb8c: {
            'b_2fcb3c: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(r32(c, S1));
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_add(0x16c8));
                if t {
                    break 'b_2fcb3c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0xce8));
                    let t = true;
                    c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0x290));
                    if t {
                        break 'b_2fcb8c;
                    } else {
                        break 'b_2fcb3c;
                    }
                }
            }
            'b_2fcb88: {
                'b_2fcb64: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                        break 'b_2fcb64;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_2fcb88;
                        } else {
                            break 'b_2fcb64;
                        }
                    }
                }
                'b_2fcb84: {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A2) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if t {
                        break 'b_2fcb84;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                        break 'b_2fcb84;
                    }
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
                break 'b_2fcb88;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
            break 'b_2fcb8c;
        }
        'b_2fcbdc: {
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;

            if t {
                break 'b_2fcbdc;
            } else {
                'b_2fcbb0: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0xa;
                    if t {
                        break 'b_2fcbb0;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.g[S1][0];
                        WORLD_OBJ_FACTORY_ATTACH_MODIFIER_CALL.call(c, 0x002f_cba8)?;
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_2fcbdc;
                        } else {
                            break 'b_2fcbb0;
                        }
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x3c08));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                EA_MEM_ALLOC_NAMED_CALL.call(c, 0x002f_cbc8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0034_add8, 0x002f_cbdc)?;
                break 'b_2fcbdc;
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx(r32(c, S3).wrapping_sub(0x6f68));
        c.ldl(2, r32(c, A1).wrapping_add(0x7));
        c.ldr(2, r32(c, A1));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x8));
        c.sdl(2, r32(c, S2).wrapping_add(0x7));
        c.sdr(2, r32(c, S2));
        c.sw(r32(c, S2).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LoadIndicator_Render (0x00231fc0)

/// `void LoadIndicator::Render(this)` (0x231fc0): draw the pulsing sprite of the loading indicator at the bottom-right of the 640x480 screen: save a render state, begin the 2D scene, set the sprite state and viewport, size the sprite with a sine of the time, draw it and restore.
pub fn load_indicator_render(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        'b_232180: {
            'b_232164: {
                c.cycles += 101;
                Frame::open(c, 0x90);
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x50);
                frame.save_ra(c, 0x40);
                frame.save_fpr(c, 21, 0x88);
                frame.save_fpr(c, 20, 0x80);
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0xe84));
                c.ldl(3, r32(c, V0).wrapping_add(0x7));
                c.ldr(3, r32(c, V0));
                c.ldl(4, r32(c, V0).wrapping_add(0xf));
                c.ldr(4, r32(c, V0).wrapping_add(0x8));
                c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x10));
                c.sdl(3, r32(c, V0).wrapping_add(0x1b));
                c.sdr(3, r32(c, V0).wrapping_add(0x14));
                c.sdl(4, r32(c, V0).wrapping_add(0x23));
                c.sdr(4, r32(c, V0).wrapping_add(0x1c));
                c.sw(r32(c, V0).wrapping_add(0x24), c.g[A2][0]);
                c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0xe84));
                c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
                c.sw(r32(c, A1).wrapping_add(0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0xa8));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xac));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_2034)?;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0xf8));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xfc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_204c)?;
                c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[A0][0] = 0x500000;
                c.g[A3][0] = 0xfffffffffe7f0000;
                c.g[T3][0] = 0x1000000;
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xe84));
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[T0][0] = 0xffffffffffcf0000;
                c.g[T2][0] = 0xfffffffffff00000;
                c.g[A2][0] = sx(r32(c, A0).wrapping_add(0x1420));
                c.ldl(9, r32(c, A2).wrapping_add(0x7));
                c.ldr(9, r32(c, A2));
                c.ldl(2, r32(c, A2).wrapping_add(0xf));
                c.ldr(2, r32(c, A2).wrapping_add(0x8));
                c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x10));
                c.sdl(9, r32(c, V1).wrapping_add(0x7));
                c.sdr(9, r32(c, V1));
                c.sdl(2, r32(c, V1).wrapping_add(0xf));
                c.sdr(2, r32(c, V1).wrapping_add(0x8));
                c.sw(r32(c, V1).wrapping_add(0x10), c.g[A1][0]);
                c.g[T0][0] = c.g[T0][0] | 0xffff;
                c.g[T2][0] = c.g[T2][0] | 0xfff;
                c.g[T1][0] = 0x10000;
                c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[T1][0] = c.g[T1][0] | 0x4000;
                c.g[T4][0] = 0xffffffffffffff83;
                c.g[A1][0] = 0x500000;
                c.g[A0][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xe60));
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                c.sw(r32(c, A0).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw(r32(c, V1).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw(r32(c, A0).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw(r32(c, V1).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(r32(c, A2).wrapping_add(0xe84));
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] | 0x3e0;
                c.sw(r32(c, A0).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x110));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x114));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_2128)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x10d8));
                c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(0xb8));
                c.g[S2][0] = sx(r32(c, V1).wrapping_add(0xbc));
                c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x50));
                c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x54));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, t, 0x0023_214c)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
                if t {
                    break 'b_232164;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    let t = true;
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
                    if t {
                        break 'b_232180;
                    } else {
                        break 'b_232164;
                    }
                }
            }
            c.cycles += 7;
            c.g[V1][0] = sx(r32(c, V0) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 20);
            c.cvt_s_w(20, 20);
            c.add_s(20, 20, 20);
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
            break 'b_232180;
        }
        'b_2321bc: {
            'b_2321a8: {
                c.cycles += 6;
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x58));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x5c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0023_2190)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx(r32(c, V0) >> 1);
                if t {
                    break 'b_2321a8;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 15);
                    c.cvt_s_w(15, 15);
                    let t = true;

                    if t {
                        break 'b_2321bc;
                    } else {
                        break 'b_2321a8;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 15);
            c.cvt_s_w(15, 15);
            c.add_s(15, 15, 15);
            break 'b_2321bc;
        }
        c.cycles += 90;
        c.mtc1(0, 12);
        c.mov_s(14, 20);
        c.g[V0][0] = c.lw(r32(c, S2));
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(20, 12);
        c.mov_s(13, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = 0x1;
        call_ptr(c, t, 0x0023_21e4)?;
        c.g[A3][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.mov_s(16, 20);
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 14);
        c.mov_s(17, 21);
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x10d8));
        c.mov_s(13, 20);
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 15);
        c.mov_s(12, 20);
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xd0));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xd4));
        c.g[A2][0] = 0u64;
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_2224)?;
        c.lwc1(0, r32(c, S0));
        c.lwc1(12, r32(c, GP).wrapping_sub(0x51a4));
        c.mul_s(12, 0, 12);
        call_ptr(c, 0x0031_c040, 0x0023_2234)?;
        c.add_s(0, 0, 21);
        c.g[AT][0] = 0x4110_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x4190_0000;
        c.mtc1(1, 3);
        c.g[T1][0] = sx(r32(c, S0).wrapping_add(0xc));
        c.lwc1(2, r32(c, GP).wrapping_sub(0x51a0));
        c.g[A1][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.mul_s(0, 0, 1);
        frame.save_fpr(c, 20, 0x18);
        c.lwc1(1, r32(c, GP).wrapping_sub(0x519c));
        c.g[A3][0] = sx(r32(c, SP).wrapping_add(0x20));
        frame.save_fpr(c, 21, 0x1c);
        c.g[T0][0] = sx(r32(c, SP).wrapping_add(0x30));
        frame.save_fpr(c, 3, 0x4);
        c.g[A2][0] = c.g[SP][0];
        frame.save_fpr(c, 0, 0);
        frame.save_fpr(c, 2, 0x10);
        frame.save_fpr(c, 1, 0x14);
        c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.g[T2][0] = c.lw(r32(c, V1).wrapping_add(0xa0));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xe84));
        c.sh(r32(c, V0).wrapping_add(0x10), c.g[T2][0]);
        frame.save_fpr(c, 20, 0x24);
        c.g[T2][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.lwc1(0, r32(c, S0).wrapping_add(0x4));
        frame.save_fpr(c, 21, 0x34);
        c.swc1(0, r32(c, S0).wrapping_add(0xc));
        frame.save_fpr(c, 20, 0x20);
        frame.save_fpr(c, 21, 0x30);
        c.g[V0][0] = c.lw(r32(c, T2).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x270));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x274));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_22c4)?;
        c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x100));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x104));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_22dc)?;
        c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0xb0));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xb4));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x0023_22f4)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        frame.restore(c, S0, 0x70);
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xe84));
        frame.restore(c, S1, 0x60);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        c.sw(r32(c, A0).wrapping_add(0xe84), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimObject_ComputeNodeVelocities (0x0034e798)

/// `IntersectInfo_EvalDerivative` (0x00351660).
pub(crate) const INTERSECT_INFO_EVAL_DERIVATIVE_CALL: Callee = Callee {
    addr: 0x0035_1660,
    f: ee::s11::IntersectInfo_EvalDerivative,
};

/// `void AnimObject::ComputeNodeVelocities(this)` (0x34e798): per active node, take the animation derivative (position and rotation rates, scaled by the anim speed and frame time), rotate by the node and bone matrices, add the parent bone's motion (lever arm cross product) and store the result in the per-bone delta table (+0x4c). Clears +0x10.
pub fn anim_object_compute_node_velocities(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x180 };
        'b_34e7f8: {
            c.cycles += 19;
            Frame::open(c, 0x180);
            frame.save(c, S0, 0x150);
            frame.save(c, S1, 0x140);
            frame.save(c, S2, 0x130);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S3, 0x120);
            frame.save(c, S4, 0x110);
            frame.save(c, S5, 0x100);
            frame.save(c, S6, 0xf0);
            frame.save(c, S7, 0xe0);
            frame.save(c, FP, 0xd0);
            frame.save_ra(c, 0xc0);
            frame.save_fpr(c, 22, 0x170);
            frame.save_fpr(c, 21, 0x168);
            frame.save_fpr(c, 20, 0x160);
            c.g[V0][0] = c.lhu(r32(c, S1).wrapping_add(0x26));
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x14));
            if t {
                break 'b_34e7f8;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x198));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x19c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0034_e7f8)?;
                break 'b_34e7f8;
            }
        }
        'b_34eb60: {
            c.cycles += 7;
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x18));
            c.g[S6][0] = 0u64;
            c.g[S4][0] = 0u64;
            c.g[S5][0] = c.lw(r32(c, V1).wrapping_add(0x80));
            c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(0x4));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.lw(r32(c, S5).wrapping_add(0x8));
            if t {
                break 'b_34eb60;
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0x500000;
                c.lwc1(22, r32(c, GP).wrapping_sub(0x2aa4));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 21);
                c.g[S7][0] = sx(r32(c, V0).wrapping_sub(0xee0));
                c.g[FP][0] = sx(r32(c, SP).wrapping_add(0x40));
                'l_34e830: loop {
                    'b_34eb4c: {
                        c.cycles += 12;
                        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                        c.g[S3][0] = sx(r32(c, S4) << 5);
                        c.lq(2, r32(c, S7));
                        c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                        c.sq(2, r32(c, A0));
                        c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                        c.lq(2, r32(c, S7));
                        c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                        c.sq(2, r32(c, V1).wrapping_add(0x10));
                        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(0x8));
                        let t = c.g[A0][0] == 0u64;
                        c.g[V0][0] = 0xd0;
                        if t {
                            break 'b_34eb4c;
                        } else {
                            'b_34eb28: {
                                c.cycles += 130;
                                c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x20));
                                c.mult(0, 2, 22, 2);
                                c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(0x48));
                                c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(0x1a8));
                                c.g[S6][0] = sx(r32(c, S6).wrapping_add(0x1));
                                c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x1ac));
                                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                let t = c.g[V1][0] as u32;
                                c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                                call_ptr(c, t, 0x0034_e884)?;
                                c.lwc1(12, r32(c, S1).wrapping_add(0x4));
                                c.mov_s(20, 0);
                                c.g[A1][0] = c.g[SP][0];
                                c.g[A2][0] = sx(r32(c, SP).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S0][0];
                                INTERSECT_INFO_EVAL_DERIVATIVE_CALL.call(c, 0x0034_e89c)?;
                                c.lwc1(2, r32(c, S0).wrapping_add(0x58));
                                c.g[A0][0] = sx(r32(c, SP).wrapping_add(0xb0));
                                c.lwc1(1, r32(c, S0).wrapping_add(0x50));
                                c.g[A1][0] = sx(r32(c, SP).wrapping_add(0xb4));
                                c.lwc1(0, r32(c, S0).wrapping_add(0x54));
                                c.neg_s(2, 2);
                                c.neg_s(1, 1);
                                c.sw(r32(c, SP).wrapping_add(0x3c), 0u64);
                                c.neg_s(0, 0);
                                c.g[V1][0] = c.lw(r32(c, GP).wrapping_add(0x2a74));
                                frame.save_fpr(c, 2, 0x38);
                                frame.save_fpr(c, 1, 0x30);
                                frame.save_fpr(c, 0, 0x34);
                                c.lq(2, r32(c, SP).wrapping_add(0x30));
                                c.sq(2, r32(c, SP).wrapping_add(0x20));
                                c.lwc1(0, r32(c, V1).wrapping_add(0x10));
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 20, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, r32(c, SP));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, r32(c, SP));
                                c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0x2a74));
                                c.lwc1(0, r32(c, V0).wrapping_add(0x10));
                                c.cvt_s_w(0, 0);
                                c.mul_s(20, 20, 0);
                                c.mul_s(20, 20, 22);
                                c.mfc1(2, 20);
                                c.lqc2(4, r32(c, SP).wrapping_add(0x10));
                                c.qmtc2(2, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, r32(c, SP).wrapping_add(0x10));
                                frame.restore_fpr(c, 2, 0x24);
                                frame.restore_fpr(c, 0, 0x20);
                                frame.restore_fpr(c, 1, 0x28);
                                c.mov_s(12, 2);
                                frame.save_fpr(c, 0, 0x30);
                                frame.save_fpr(c, 1, 0x38);
                                frame.save_fpr(c, 2, 0x34);
                                call_ptr(c, 0x0031_be50, 0x0034_e940)?;
                                frame.restore_fpr(c, 12, 0x38);
                                c.g[A1][0] = sx(r32(c, SP).wrapping_add(0xbc));
                                c.g[A0][0] = sx(r32(c, SP).wrapping_add(0xb8));
                                call_ptr(c, 0x0031_be50, 0x0034_e950)?;
                                frame.restore_fpr(c, 0, 0xb4);
                                c.g[A1][0] = sx(r32(c, S4) << 6);
                                frame.restore_fpr(c, 4, 0xbc);
                                c.neg_s(2, 0);
                                frame.restore_fpr(c, 1, 0xb8);
                                c.mul_s(0, 0, 4);
                                frame.restore_fpr(c, 3, 0xb0);
                                frame.save_fpr(c, 1, 0x50);
                                c.mul_s(2, 2, 1);
                                frame.save_fpr(c, 3, 0x48);
                                frame.save_fpr(c, 0, 0x40);
                                frame.save_fpr(c, 4, 0x54);
                                frame.save_fpr(c, 2, 0x44);
                                c.sw(r32(c, SP).wrapping_add(0x4c), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x58), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x5c), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x60), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x64), 0u64);
                                frame.save_fpr(c, 21, 0x68);
                                c.sw(r32(c, SP).wrapping_add(0x6c), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x70), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x74), 0u64);
                                c.sw(r32(c, SP).wrapping_add(0x78), 0u64);
                                frame.save_fpr(c, 21, 0x7c);
                                c.lqc2(8, r32(c, SP).wrapping_add(0x10));
                                c.lqc2(4, r32(c, FP));
                                c.lqc2(5, r32(c, FP).wrapping_add(0x10));
                                c.lqc2(6, r32(c, FP).wrapping_add(0x20));
                                c.lqc2(7, r32(c, FP).wrapping_add(0x30));
                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                c.sqc2(12, r32(c, SP).wrapping_add(0x80));
                                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x44));
                                c.lq(2, r32(c, SP).wrapping_add(0x80));
                                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                                c.sq(2, r32(c, SP).wrapping_add(0x10));
                                c.sq(2, r32(c, SP).wrapping_add(0x30));
                                c.lqc2(8, r32(c, SP));
                                c.lqc2(4, r32(c, V1));
                                c.lqc2(5, r32(c, V1).wrapping_add(0x10));
                                c.lqc2(6, r32(c, V1).wrapping_add(0x20));
                                c.lqc2(7, r32(c, V1).wrapping_add(0x30));
                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                c.sqc2(12, r32(c, SP).wrapping_add(0x30));
                                c.lq(2, r32(c, SP).wrapping_add(0x30));
                                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x44));
                                c.sq(2, r32(c, SP));
                                c.sq(2, r32(c, SP).wrapping_add(0x80));
                                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                                c.lqc2(8, r32(c, SP).wrapping_add(0x10));
                                c.lqc2(4, r32(c, V1));
                                c.lqc2(5, r32(c, V1).wrapping_add(0x10));
                                c.lqc2(6, r32(c, V1).wrapping_add(0x20));
                                c.lqc2(7, r32(c, V1).wrapping_add(0x30));
                                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                c.sqc2(12, r32(c, SP).wrapping_add(0x30));
                                c.lq(2, r32(c, SP).wrapping_add(0x30));
                                c.sq(2, r32(c, SP).wrapping_add(0x10));
                                c.sq(2, r32(c, SP).wrapping_add(0x80));
                                c.g[V1][0] = c.lw(r32(c, S2));
                                if (c.g[V1][0] as i64) < 0 {
                                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                                    break 'b_34eb28;
                                } else {
                                    c.cycles += 48;
                                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x44));
                                    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                                    c.lq(2, r32(c, V1).wrapping_add(0x30));
                                    c.sq(2, r32(c, SP).wrapping_add(0x30));
                                    c.g[V1][0] = c.lw(r32(c, S2));
                                    c.g[V1][0] = sx(r32(c, V1) << 6);
                                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                                    c.lq(2, r32(c, V1).wrapping_add(0x30));
                                    c.sq(2, r32(c, SP).wrapping_add(0x90));
                                    c.lqc2(3, r32(c, SP).wrapping_add(0x30));
                                    c.lqc2(4, r32(c, SP).wrapping_add(0x90));
                                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, r32(c, SP).wrapping_add(0xa0));
                                    c.lq(3, r32(c, SP).wrapping_add(0xa0));
                                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                                    c.sq(3, r32(c, SP).wrapping_add(0x80));
                                    c.g[V0][0] = c.lw(r32(c, S2));
                                    c.g[V0][0] = sx(r32(c, V0) << 5);
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                    c.lqc2(3, r32(c, SP).wrapping_add(0x10));
                                    c.lqc2(4, r32(c, V0).wrapping_add(0x10));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, r32(c, SP).wrapping_add(0x10));
                                    c.g[A0][0] = c.lw(r32(c, S2));
                                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                                    c.g[A0][0] = sx(r32(c, A0) << 5);
                                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                    c.lqc2(4, r32(c, SP).wrapping_add(0x80));
                                    c.lqc2(5, r32(c, V0).wrapping_add(0x10));
                                    c.vu0().opmula(4, 5);
                                    c.vu0().opmsub(6, 5, 4);
                                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                    c.sqc2(6, r32(c, SP).wrapping_add(0xa0));
                                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                                    c.lq(3, r32(c, SP).wrapping_add(0xa0));
                                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                    c.sq(3, r32(c, SP).wrapping_add(0x90));
                                    c.lqc2(3, r32(c, A0));
                                    c.lqc2(4, r32(c, SP).wrapping_add(0x90));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, r32(c, SP).wrapping_add(0xa0));
                                    c.lq(2, r32(c, SP).wrapping_add(0xa0));
                                    c.sq(2, r32(c, SP).wrapping_add(0x30));
                                    c.lqc2(3, r32(c, SP));
                                    c.lqc2(4, r32(c, SP).wrapping_add(0x30));
                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                    c.sqc2(5, r32(c, SP));
                                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                                    break 'b_34eb28;
                                }
                            }
                            c.cycles += 9;
                            c.lq(4, r32(c, SP));
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                            c.sq(4, r32(c, V0));
                            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
                            c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                            c.lqc2(3, r32(c, V1).wrapping_add(0x10));
                            c.lqc2(4, r32(c, SP).wrapping_add(0x10));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, r32(c, V1).wrapping_add(0x10));
                            break 'b_34eb4c;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(0x4));
                    c.g[S4][0] = sx(r32(c, S4).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_e830);
                        }
                        continue 'l_34e830;
                    } else {
                        break 'b_34eb60;
                    }
                }
            }
        }
        c.cycles += 16;
        c.sw(r32(c, S1).wrapping_add(0x10), 0u64);
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore(c, S5, 0x100);
        frame.restore(c, S6, 0xf0);
        frame.restore(c, S7, 0xe0);
        frame.restore(c, FP, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.restore_fpr(c, 22, 0x170);
        frame.restore_fpr(c, 21, 0x168);
        frame.restore_fpr(c, 20, 0x160);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};

    /// Gate word, call counter (+4), unwind index (+8), forced results
    /// (`+0x10 + 4*id`), float results (`+0x200 + 4*id`), out-parameter words
    /// (`+0x400 + 0x40*id`) and the call log (`+0x1000`, 0x40 bytes per call) of
    /// the fake callees. Not near `DATA+0x100000`.
    pub(crate) const OC410_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC410_MAGIC: u32 = 0x0c41_0a11;

    /// Result word of fake `id` (what its caller sees in `v0`).
    pub(crate) const fn oc410_result(id: u32) -> u32 {
        OC410_CTRL + 0x10 + 4 * id
    }
    /// Float result (`f0`) of fake `id`.
    pub(crate) const fn oc410_fresult(id: u32) -> u32 {
        OC410_CTRL + 0x200 + 4 * id
    }
    /// The words fake `id` stores through its out-parameters, in order
    /// (`outs`: register holding the pointer, byte offset, word count).
    pub(crate) const fn oc410_out(id: u32) -> u32 {
        OC410_CTRL + 0x400 + 0x40 * id
    }

    /// Fake callee `id`: logs its arguments and cycles, returns the forced
    /// result (or unwinds when its call index is the one in `CTRL+8`).
    pub(crate) fn oc410_fake(c: &mut Cpu, id: u32, outs: &[(usize, u32, u32)]) {
        let n = c.bus.read32(OC410_CTRL + 4);
        c.bus.write32(OC410_CTRL + 4, n.wrapping_add(1));
        let p = OC410_CTRL + 0x1000 + (n & 0x3f) * 0x40;
        c.bus.write32(p, id);
        for i in 0..4 {
            c.bus.write64(p + 8 + 8 * i as u32, c.g[A0 + i][0]);
        }
        c.bus.write32(p + 0x28, c.f[12]);
        c.bus.write32(p + 0x2c, c.f[13]);
        c.bus.write32(p + 0x30, c.g[RA][0] as u32);
        c.bus.write32(p + 0x34, c.g[SP][0] as u32);
        c.bus.write64(p + 0x38, c.cycles);
        let mut src = oc410_out(id);
        for &(reg, byte_off, words) in outs {
            let dst = (c.g[reg][0] as u32).wrapping_add(byte_off);
            for w in 0..words {
                let v = c.bus.read32(src);
                c.bus.write32(dst + 4 * w, v);
                src += 4;
            }
        }
        let forced = c.bus.read32(oc410_result(id));
        c.g[V0][0] = sx(forced);
        c.g[V1][0] = 0x5151_0000 + n as u64;
        c.f[0] = c.bus.read32(oc410_fresult(id));
        c.cycles += 5;
        if n == c.bus.read32(OC410_CTRL + 8) {
            c.pc = 0xdead_0000 | n << 4;
        } else {
            c.pc = c.g[RA][0] as u32;
        }
    }

    /// Defines the gated fake callees and `oc410_hook_fakes`, which puts each
    /// in its override slot (once per process; with the gate word clear a fake
    /// runs what the slot held before).
    macro_rules! oc410_fakes {
        ($(($name:ident, $id:expr, $out:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC410_CTRL) == OC410_MAGIC {
                        return oc410_fake(c, $id, $out);
                    }
                    fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen)
                }
            )*
            pub(crate) fn oc410_hook_fakes() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $name,
                        fakes::prev_cell($name),
                    );
                )*
            }
        };
    }

    oc410_fakes! {
        (fake_patch_dma_chain, 0, &[], s13, OVERRIDE_003AC048, f_003ac048__generated),
        (fake_new_named, 1, &[], s10, OVERRIDE_00317E30, f_00317e30__generated),
        (fake_mem_copy, 2, &[], s14, OVERRIDE_003E6574, f_003e6574__generated),
        (fake_mem_free, 3, &[], s10, OVERRIDE_00317E98, f_00317e98__generated),
        (fake_compile_identifier, 4, &[], s10, OVERRIDE_003245A8, f_003245a8__generated),
        (fake_get_script_obj, 5, &[], s02, OVERRIDE_001694B0, f_001694b0__generated),
        (fake_be_get_instance, 6, &[], s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (fake_be_get_interface, 7, &[], s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (fake_be_char_profile_data, 8, &[], s01, OVERRIDE_0014AD28, f_0014ad28__generated),
        (fake_bolts_char_bolt_count, 9, &[], s01, OVERRIDE_0014D988, f_0014d988__generated),
        (fake_bolts_char_bolts, 10, &[], s01, OVERRIDE_0014D998, f_0014d998__generated),
        (fake_get_uint, 11, &[(A1, 0, 1)], s09, OVERRIDE_002D2558, f_002d2558__generated),
        (fake_operator_new, 12, &[], s10, OVERRIDE_00317D70, f_00317d70__generated),
        (fake_cam_act_type0, 13, &[], s02, OVERRIDE_001715A0, f_001715a0__generated),
        (fake_cam_act_type1, 14, &[], s02, OVERRIDE_001714E8, f_001714e8__generated),
        (fake_cam_act_type2, 15, &[], s02, OVERRIDE_001715F8, f_001715f8__generated),
        (fake_cam_act_type3, 16, &[], s02, OVERRIDE_00171680, f_00171680__generated),
        (fake_script_cmd_queue_at, 17, &[], s07, OVERRIDE_00283D70, f_00283d70__generated),
        (fake_script_process_finished, 18, &[], s07, OVERRIDE_00282838, f_00282838__generated),
        (fake_span_activate_key, 19, &[], s07, OVERRIDE_002730C0, f_002730c0__generated),
        (fake_span_next_key, 20, &[(A0, 0x14, 1)], s07, OVERRIDE_00273140, f_00273140__generated),
        (fake_input_emit_node, 21, &[(A2, 0, 1)], s10, OVERRIDE_00324678, f_00324678__generated),
        (fake_input_error, 22, &[], s10, OVERRIDE_00321500, f_00321500__generated),
        (fake_get_listener, 24, &[], s07, OVERRIDE_00274D70, f_00274d70__generated),
        (fake_get_char_id, 25, &[], s01, OVERRIDE_0014A080, f_0014a080__generated),
        (fake_get_cheat_char_id, 26, &[], s01, OVERRIDE_0014A0B0, f_0014a0b0__generated),
        (fake_get_db, 27, &[], s01, OVERRIDE_0014BDB8, f_0014bdb8__generated),
        (fake_rider_data, 28, &[], s01, OVERRIDE_0014AD50, f_0014ad50__generated),
        (fake_model_lookup, 29, &[], s09, OVERRIDE_002D1BD8, f_002d1bd8__generated),
        (fake_attach_modifier, 30, &[], s10, OVERRIDE_002FAE38, f_002fae38__generated),
        (fake_flag_node_ctor, 31, &[], s11, OVERRIDE_0034ADD8, f_0034add8__generated),
        (fake_sine, 32, &[], s10, OVERRIDE_0031C040, f_0031c040__generated),
        (fake_get_deriv, 33, &[(A1, 0, 4), (A2, 0, 4)], s11, OVERRIDE_00351660, f_00351660__generated),
        (fake_sin_cos, 34, &[(A0, 0, 1), (A1, 0, 1)], s10, OVERRIDE_0031BE50, f_0031be50__generated),
    }

    /// Scrambled machine with a game gp, the fakes' gate open and (one seed in
    /// three) a fake call that unwinds (`calls`: how many fake calls to choose from).
    pub(crate) fn oc410_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC410_CTRL, OC410_MAGIC);
        let unwind = if calls > 0 && seed % 3 == 0 {
            r.next_u32() % calls
        } else {
            u32::MAX
        };
        c.bus.write32(OC410_CTRL + 8, unwind);
        oc410_hook_fakes();
        r
    }

    /// The mesh at `MESH`: parts at `PARTS`, items at `ITEMS`.
    const MESH: u32 = DATA + 0x1000;
    const PARTS: u32 = DATA + 0x2000;
    const ITEMS: u32 = DATA + 0x3000;

    fn chunk_resolve_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = oc410_machine(c, seed, salt, 0);
        c.pc = entry;
        c.g[A0][0] = (DATA + 0x500) as u64;
        c.g[A1][0] = MESH as u64;
        c.bus.write32(MESH + CHUNK_MESH_RELOC, r.next_u32());
        let parts = match seed % 7 {
            0 => 0,
            1 => 0xffff_fffe,
            n => n % 4,
        };
        c.bus.write32(MESH + CHUNK_MESH_PART_COUNT, parts);
        c.bus.write32(MESH + CHUNK_MESH_PART_TABLE, PARTS);
        for i in 0..4u32 {
            let item = ITEMS + i * 0x100;
            c.bus.write32(PARTS + 0x10 * i, r.next_u32());
            let null = (seed >> i) & 1 == 1 && i != 0;
            c.bus
                .write32(PARTS + 0x10 * i + 4, if null { 0 } else { item });
            let chains = match (seed + i) % 6 {
                0 => 0,
                1 => 0xffff_ffff,
                n => n % 4,
            };
            c.bus.write32(item + CHUNK_PART_CHAIN_COUNT, chains);
            c.bus.write32(item + CHUNK_PART_CHAIN_LIST, item + 0x40);
            for k in 0..4 {
                c.bus.write32(item + 0x40 + 4 * k, r.next_u32());
            }
        }
        c.g[A2][0] = (ITEMS + 0x20) as u64;
        // A `part` call from the test of `RelocateMeshDmaPart` itself.
        if entry == 0x003a_bfc0 {
            c.g[A1][0] = (ITEMS + 0x100 * (seed % 4)) as u64;
            c.g[A2][0] = MESH as u64;
        }
    }

    /// `RelocateMeshDma` makes `+0x24` absolute and relocates every part.
    #[test]
    fn c_ps2_chunk_resolve_relocate_mesh_dma_matches() {
        check(
            ee::s13::f_003abf20__generated,
            c_ps2_chunk_resolve_relocate_mesh_dma,
            300,
            &|c, seed| chunk_resolve_setup(c, seed, 0x003a_bf20, 0x003a_bf20),
        );
    }

    /// `RelocateMeshDmaParts` walks the part table, skipping null entries.
    #[test]
    fn c_ps2_chunk_resolve_relocate_mesh_dma_parts_matches() {
        check(
            ee::s13::f_003abf50__generated,
            c_ps2_chunk_resolve_relocate_mesh_dma_parts,
            300,
            &|c, seed| chunk_resolve_setup(c, seed, 0x003a_bf50, 0x003a_bf50),
        );
    }

    /// `RelocateMeshDmaPart` patches each DMA chain of one part.
    #[test]
    fn c_ps2_chunk_resolve_relocate_mesh_dma_part_matches() {
        check(
            ee::s13::f_003abfc0__generated,
            c_ps2_chunk_resolve_relocate_mesh_dma_part,
            300,
            &|c, seed| chunk_resolve_setup(c, seed, 0x003a_bfc0, 0x003a_bfc0),
        );
    }

    /// Controller `CTRL`, node list at `NODES`, algos at `ALGOS`.
    const SCAM_CTRL: u32 = DATA + 0x1000;
    const SCAM_NODES: u32 = DATA + 0x2000;
    const SCAM_ALGOS: u32 = DATA + 0x3000;
    const SCAM_SCRIPT_OBJ: u32 = DATA + 0x4000;

    /// `ScriptCamController_Update`: progress -> smoothstep weight, then the
    /// algo's virtual method for every node (some seeds unwind in one).
    #[test]
    fn script_cam_controller_update_matches() {
        check(
            ee::s02::f_00169368__generated,
            script_cam_controller_update,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0016_9368, 4);
                c.pc = 0x0016_9368;
                c.g[A0][0] = SCAM_CTRL as u64;
                c.bus
                    .write32(SCAM_CTRL + SCRIPT_CAM_CTRL_LIST, SCAM_CTRL + 0x100);
                let nodes = seed % 5;
                c.bus
                    .write32(SCAM_CTRL + 0x100, if nodes == 0 { 0 } else { SCAM_NODES });
                for i in 0..nodes {
                    let node = SCAM_NODES + 0x40 * i;
                    let algo = SCAM_ALGOS + 0x40 * i;
                    c.bus.write32(node + SCRIPT_CAM_NODE_ALGO, algo);
                    let next = if i + 1 < nodes { node + 0x40 } else { 0 };
                    c.bus.write32(node + SCRIPT_CAM_NODE_NEXT, next);
                    let vt = fakes::vtable(c, &mut r, &[0x28]);
                    c.bus.write32(algo + 0x10, vt);
                }
                c.bus.write32(oc410_result(5), SCAM_SCRIPT_OBJ);
                // Progress: inside 0..1 (no clamp) or anywhere (negative: clamp).
                let progress = if seed % 2 == 0 {
                    r.f32(1.0) & 0x3fff_ffff
                } else {
                    r.edge_f32()
                };
                c.bus.write32(SCAM_SCRIPT_OBJ + 0x18, progress);
            },
        );
    }

    const IMAP: u32 = DATA + 0x1000;

    /// `InputMap_CompileEntry`: grows the opcode buffer when fewer than 0x100
    /// words remain, then compiles the statement at the end.
    #[test]
    fn input_map_compile_entry_matches() {
        check(
            ee::s10::f_00320e18__generated,
            input_map_compile_entry,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0032_0e18, 0);
                c.pc = 0x0032_0e18;
                c.g[A0][0] = IMAP as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (DATA + 0x700) as u64;
                c.bus.write32(IMAP + INPUT_MAP_OFFSETS, DATA + 0x1800);
                c.bus.write32(IMAP + INPUT_MAP_PARSER, DATA + 0x1900);
                let used = r.next_u32() % 0x300;
                let cap = match seed % 4 {
                    0 => used + 0x100 + r.next_u32() % 0x200,
                    1 => used + r.next_u32() % 0x100,
                    2 => used.wrapping_sub(r.next_u32() % 8),
                    _ => used + 0xff,
                };
                c.bus.write32(IMAP + INPUT_MAP_CAP, cap);
                c.bus.write32(IMAP + INPUT_MAP_USED, used);
                c.bus.write32(
                    IMAP + INPUT_MAP_CODE,
                    if seed % 5 == 0 { 0 } else { DATA + 0x5000 },
                );
                c.bus.write32(oc410_result(1), DATA + 0x6000);
                c.bus.write32(oc410_result(4), r.next_u32() % 40);
            },
        );
    }

    const MAGNET_THIS: u32 = DATA + 0x1000;
    const MAGNET_OBJ: u32 = DATA + 0x2000;
    const MAGNET_OUT: u32 = DATA + 0x4000;

    /// `MagnetModifier_v21`: the contact query through the object's vtable and
    /// the initialisation of every returned contact.
    #[test]
    fn magnet_modifier_v21_matches() {
        check(ee::s11::f_00357538__generated, magnet_modifier_v21, 300, &|c, seed| {
            let mut r = oc410_machine(c, seed, 0x0035_7538, 1);
            c.pc = 0x0035_7538;
            c.g[A0][0] = MAGNET_THIS as u64;
            c.g[A1][0] = MAGNET_OBJ as u64;
            c.g[A2][0] = MAGNET_OUT as u64;
            c.g[A3][0] = r.next_u32() as u64;
            // Bit 0 of the fifth argument selects the contact query.
            c.g[T0][0] = if seed % 4 == 3 {
                r.next_u32() as u64 & !1
            } else {
                r.next_u32() as u64 | 1
            };
            for k in 0..0x30 {
                c.bus.write32(MAGNET_THIS + 0x20 + 4 * k, r.next_u32());
            }
            c.bus.write32(MAGNET_THIS + 0xa0, r.next_u32());
            let vt = fakes::vtable(c, &mut r, &[0x38]);
            c.bus.write32(MAGNET_OBJ + 0x50, vt);
            // The fake method returns 1..4 contacts; a real leaf returns 0.
            if seed % 5 == 0 {
                c.bus.write32(vt + 0x3c, 0x0014_06c0);
            } else {
                c.bus.write32(fakes::LOG_RESULTS, 1 + seed % 4);
            }
            let quad = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
            crate::difftest::put_vec(c, MAGNET_CONTACT_INIT_QUAD, quad);
        });
    }

    /// Test layout for `BEBoltInt_GetBoltTextureName`.
    const TEX_E0: u32 = DATA + 0x1000;
    const TEX_OUT: u32 = DATA + 0x7000;
    const TEX_BOLTS: u32 = DATA + 0x6000;
    const TEX_PROFILE: u32 = DATA + 0x9000;
    const TEX_KEYTAB: u32 = DATA + 0xa000;
    const TEX_STRINGS: u32 = DATA + 0xb000;

    /// A short name over the alphabet `a b $ .` (NUL-terminated) at `at`.
    fn tex_name(c: &mut Cpu, r: &mut Rng, at: u32, with_dollar: bool) {
        let len = 3 + r.next_u32() % 7;
        for i in 0..len {
            let ch = match r.next_u32() % 5 {
                0 => b'$',
                1 => b'.',
                2 => b'a',
                _ => b'b',
            };
            c.bus.write8(at + i, ch);
        }
        if with_dollar {
            c.bus.write8(at + r.next_u32() % len, b'$');
        }
        c.bus.write8(at + len, 0);
    }

    /// `GetBoltTextureName`: early outs (no group / no name / no `$`), then
    /// the `$` substitution over the rider's other unlocked bolts.
    #[test]
    fn beb_bolt_int_get_bolt_texture_name_matches() {
        check(
            ee::s01::f_0014b988__generated,
            beb_bolt_int_get_bolt_texture_name,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0014_b988, 0);
                c.pc = 0x0014_b988;
                c.g[A0][0] = (DATA + 0x500) as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = TEX_E0 as u64;
                c.g[T0][0] = TEX_OUT as u64;
                let group = match seed % 11 {
                    0 => 0xff,
                    n => n % 3,
                };
                c.bus.write8(TEX_E0 + 3, group as u8);
                let key = r.next_u32() % 6;
                c.bus.write16(TEX_E0 + 4, key as u16);
                let name = if seed % 13 == 1 { 0 } else { TEX_STRINGS };
                c.bus.write32(TEX_E0 + 0x2c, name);
                tex_name(c, &mut r, TEX_STRINGS, seed % 4 != 3);
                // The rider's other bolts.
                let count = 1 + seed % 5;
                for i in 0..count {
                    let e = TEX_BOLTS + 0x38 * i;
                    c.bus.write8(e + 3, (r.next_u32() % 3) as u8);
                    c.bus.write16(e + 4, (r.next_u32() % 6) as u16);
                    let s = TEX_STRINGS + 0x40 * (i + 1);
                    tex_name(c, &mut r, s, true);
                    c.bus
                        .write32(e + 0x2c, if r.next_u32() % 7 == 0 { 0 } else { s });
                }
                // The rider's profile: key -> entry index -> flags.
                c.bus.write32(TEX_PROFILE + 0x288, TEX_KEYTAB);
                for k in 0..6u32 {
                    let idx = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write16(TEX_KEYTAB + 2 * k, idx as u16);
                }
                for i in 0..4u32 {
                    c.bus.write16(
                        TEX_PROFILE + 0x290 + 4 * i + 2,
                        (r.next_u32() & 0x10 | r.next_u32() & 1) as u16,
                    );
                }
                c.bus.write32(oc410_result(8), TEX_PROFILE);
                c.bus.write32(oc410_result(9), count);
                c.bus.write32(oc410_result(10), TEX_BOLTS);
            },
        );
    }

    fn w32(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// `ScriptList_GetPlayStatus`: every state of the play status chain.
    #[test]
    fn script_list_get_play_status_matches() {
        const LIST: u32 = DATA + 0x1000;
        const CMD: u32 = DATA + 0x2000;
        check(
            ee::s07::f_00276bd8__generated,
            script_list_get_play_status,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0027_6bd8, 2);
                c.pc = 0x0027_6bd8;
                c.g[A0][0] = LIST as u64;
                w32(c, LIST, DATA + 0x600);
                w32(
                    c,
                    LIST + 0xa4,
                    if seed % 6 == 0 { r.next_u32() % 3 } else { 3 },
                );
                w32(c, LIST + 0xc4, (seed % 5 == 1) as u32);
                w32(c, LIST + 0xc8, (seed % 4 == 2) as u32);
                w32(c, LIST + 0xb0, (seed % 3 == 0) as u32);
                w32(c, LIST + 0xb8, r.next_u32() % 4);
                let bc = if r.next_u32() & 1 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                w32(c, LIST + 0xbc, bc);
                w32(c, oc410_result(17), CMD);
                w32(c, CMD + 8, if (seed / 3) % 2 == 0 { 8 } else { 0 });
                w32(c, CMD + 0xc, r.next_u32());
                w32(c, oc410_result(18), (seed / 7) % 2);
            },
        );
    }

    /// `InSeq_SpanTrackUpdate`: expired events are unlinked, due key records
    /// activated and advanced.
    #[test]
    fn in_seq_span_track_update_matches() {
        const THIS: u32 = DATA + 0x1000;
        const FLAGS: u32 = DATA + 0x500;
        const EVENTS: u32 = DATA + 0x2000;
        const REC0: u32 = DATA + 0x3000;
        const REC1: u32 = DATA + 0x3100;
        check(
            ee::s07::f_00272f28__generated,
            in_seq_span_track_update,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0027_2f28, 4);
                c.pc = 0x0027_2f28;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = FLAGS as u64;
                c.f[12] = ((r.next_u32() % 100) as f32 + 0.5).to_bits();
                w32(c, FLAGS, r.next_u32());
                let n = seed % 4;
                w32(c, THIS + 0x24, if n == 0 { 0 } else { EVENTS });
                w32(c, THIS + 0x28, r.next_u32() % 8);
                for i in 0..n {
                    let ev = EVENTS + 0x40 * i;
                    w32(c, ev + 0xc, r.next_u32() % 100);
                    w32(c, ev + 0x18, if i + 1 < n { ev + 0x40 } else { 0 });
                }
                w32(c, THIS + 0x14, if seed % 3 == 0 { 0 } else { REC0 });
                c.bus.write16(REC0 + 4, (r.next_u32() % 100) as u16);
                c.bus.write16(REC1 + 4, 65000);
                // The key reader moves on to a future record, or to none.
                w32(c, oc410_out(20), if seed % 2 == 0 { 0 } else { REC1 });
            },
        );
    }

    /// `CameraTrigger_ReadAction`: the four action kinds and an unknown one.
    #[test]
    fn camera_trigger_read_action_matches() {
        check(
            ee::s02::f_001712b0__generated,
            camera_trigger_read_action,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0017_12b0, 3);
                c.pc = 0x0017_12b0;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                let kind = [0, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
                w32(c, oc410_out(11), kind);
                w32(c, oc410_result(11), 1 + r.next_u32() % 100);
                w32(c, oc410_result(12), DATA + 0x3000);
                for id in 13..17 {
                    w32(c, oc410_result(id), r.next_u32() % 50);
                }
                w32(c, GAME_GP_REL_643C, r.edge_f32());
                w32(c, GAME_GP_REL_6438, r.edge_f32());
                for k in 0..3 {
                    w32(c, 0x004f_f0d8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    const GAME_GP_REL_643C: u32 = crate::difftest::GAME_GP.wrapping_sub(0x643c);
    const GAME_GP_REL_6438: u32 = crate::difftest::GAME_GP.wrapping_sub(0x6438);

    /// `InputParser_EmitUnaryOp`: operand compile, register bookkeeping and the
    /// error exits.
    #[test]
    fn input_parser_emit_unary_op_matches() {
        const PARSER: u32 = DATA + 0x2_0000;
        check(
            ee::s10::f_003250d8__generated,
            input_parser_emit_unary_op,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0032_50d8, 2);
                c.pc = 0x0032_50d8;
                c.g[A0][0] = PARSER as u64;
                c.g[A1][0] = (DATA + 0x600) as u64;
                c.g[A2][0] = (DATA + 0x700) as u64;
                c.g[A3][0] = (DATA + 0x800) as u64;
                let cap = [0xffff_ffffu32, 0, 1, 2, 5][(seed % 5) as usize];
                c.g[T0][0] = cap as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                w32(c, DATA + 0x604, DATA + 0x900);
                w32(
                    c,
                    DATA + 0x700,
                    if seed % 2 == 0 {
                        0xffff_fff0
                    } else {
                        r.next_u32() % 64
                    },
                );
                w32(c, PARSER + 0x1_09e8, r.next_u32() % 4);
                for k in 0..6 {
                    w32(c, PARSER + 0x1_08ec + 4 * k, r.next_u32() % 64);
                }
                let n = [0, 1, 2, 3, 0xffff_ffff][((seed / 5) % 5) as usize];
                w32(c, oc410_result(21), n);
                w32(
                    c,
                    oc410_out(21),
                    [5, 0x3f, 0x40, 0xffff_ffff][((seed / 3) % 4) as usize],
                );
            },
        );
    }

    /// `CrowdMan2d_BindInstanceFrame`: handle -> model -> part -> bone.
    #[test]
    fn crowd_man2d_bind_instance_frame_matches() {
        const TOP: u32 = DATA + 0x1000;
        const TABLE1: u32 = DATA + 0x1100;
        const SLOTS: u32 = DATA + 0x1200;
        const CELL: u32 = DATA + 0x2000;
        const TABLE2: u32 = DATA + 0x2100;
        const MODEL: u32 = DATA + 0x3000;
        const SKEL: u32 = DATA + 0x4000;
        const POSE: u32 = DATA + 0x4100;
        const PARTS: u32 = DATA + 0x4200;
        check(
            ee::s05::f_00229820__generated,
            crowd_man2d_bind_instance_frame,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0022_9820, 1);
                c.pc = 0x0022_9820;
                let lo = seed % 3;
                let hi = seed % 2;
                c.g[A1][0] = ((hi << 8) | lo) as u64;
                c.g[A2][0] = if seed % 11 == 0 {
                    u64::MAX
                } else {
                    (seed % 5) as u64
                };
                c.g[A3][0] = ((seed / 5) % 4) as u64;
                w32(c, oc410_result(29), TOP);
                w32(c, TOP, TABLE1);
                w32(c, TABLE1 + 8, SLOTS);
                for i in 0..3 {
                    w32(
                        c,
                        SLOTS + 4 * i,
                        if seed % 4 == 1 && i == lo { 0 } else { CELL },
                    );
                }
                w32(c, CELL + 0x1c, TABLE2);
                for i in 0..2 {
                    let x = if seed % 7 == 0 {
                        0
                    } else {
                        ((MODEL >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    w32(c, TABLE2 + 4 * i, x);
                }
                w32(c, MODEL + 0x80, SKEL);
                w32(c, MODEL + 0x94, POSE);
                w32(c, SKEL + 4, 3);
                w32(c, SKEL + 8, PARTS);
                for i in 0..3u32 {
                    let part = DATA + 0x4300 + 0x100 * i;
                    w32(c, PARTS + 0x10 * i + 4, part);
                    w32(c, part + 0x1c, 2 + i % 2);
                    w32(c, part + 0x20, part + 0x80);
                    for k in 0..3u32 {
                        let idx_at = DATA + 0x4800 + 0x20 * i + 4 * k;
                        w32(c, part + 0x80 + 4 * k, idx_at);
                        c.bus.write16(idx_at, (r.next_u32() % 4) as u16);
                    }
                }
            },
        );
    }

    /// `InSeq_InstanceAdvance`: jump decision, channel loop (real
    /// `InSeq_ChannelAdvance` on empty channels) and the listener call.
    #[test]
    fn in_seq_instance_advance_matches() {
        const THIS: u32 = DATA + 0x1000;
        const HDR: u32 = DATA + 0x1100;
        const TRACKS: u32 = DATA + 0x2000;
        const EMPTY_HDR: u32 = DATA + 0x5000;
        const LISTENER: u32 = DATA + 0x6000;
        check(
            ee::s07::f_002751a0__generated,
            in_seq_instance_advance,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0027_51a0, 2);
                c.pc = 0x0027_51a0;
                c.g[A0][0] = THIS as u64;
                let frac = |r: &mut Rng| if r.next_u32() & 1 == 0 { 0.0 } else { 0.5 };
                let time = (r.next_u32() % 50) as f32 + frac(&mut r);
                let prev = (r.next_u32() % 50) as f32 + frac(&mut r);
                c.f[12] = time.to_bits();
                c.f[13] = prev.to_bits();
                c.f[14] = [0.5f32, 1.0, 2.0][(seed % 3) as usize].to_bits();
                w32(c, THIS + 0x10, HDR);
                let channels = (seed / 3) % 4;
                c.bus.write8(HDR + 1, channels as u8);
                w32(c, THIS + 0x18, TRACKS);
                for i in 0..channels {
                    let ch = TRACKS + 0x3c * i;
                    w32(c, ch + 0x28, if r.next_u32() & 1 == 0 { 0 } else { 1 });
                    w32(c, ch + 0x1c, EMPTY_HDR);
                    w32(c, ch + 0x2c, 0);
                }
                let listener = if seed % 5 == 0 { 0 } else { LISTENER };
                w32(c, oc410_result(24), listener);
                let vt = fakes::vtable(c, &mut r, &[0x10]);
                w32(c, LISTENER, vt);
            },
        );
    }

    /// `Rider_GetEquippedBolts`: the cheat character path and the profile
    /// flag filter.
    #[test]
    fn rider_get_equipped_bolts_matches() {
        const RIDER: u32 = DATA + 0x1000;
        const LIST: u32 = DATA + 0x2000;
        const PROFILE: u32 = DATA + 0x3000;
        const KEYS: u32 = DATA + 0x3800;
        const BOLTS: u32 = DATA + 0x4000;
        check(
            ee::s00::f_0011bbe8__generated,
            rider_get_equipped_bolts,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0011_bbe8, 8);
                c.pc = 0x0011_bbe8;
                c.g[A0][0] = RIDER as u64;
                c.g[A1][0] = LIST as u64;
                w32(c, RIDER + 0x86c, r.next_u32() % 8);
                w32(
                    c,
                    oc410_result(26),
                    if seed % 3 == 0 {
                        1 + r.next_u32() % 5
                    } else {
                        0
                    },
                );
                w32(c, oc410_result(27), DATA + 0x700);
                w32(c, oc410_result(28), PROFILE);
                w32(c, oc410_result(10), BOLTS);
                w32(
                    c,
                    oc410_result(9),
                    if seed % 7 == 0 { 0 } else { 1 + seed % 4 },
                );
                for i in 0..4u32 {
                    c.bus
                        .write16(BOLTS + 0x38 * i + 4, (r.next_u32() % 6) as u16);
                }
                w32(c, PROFILE + 0x288, KEYS);
                for k in 0..6u32 {
                    let idx = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write16(KEYS + 2 * k, idx as u16);
                }
                for i in 0..4u32 {
                    c.bus
                        .write16(PROFILE + 0x290 + 4 * i + 2, (r.next_u32() & 0x10) as u16);
                }
            },
        );
    }

    /// `WScriptFn_CreateFlag`: static template init, the override loop (raw
    /// and float-converted), the object lookup and the node allocation.
    #[test]
    fn w_script_fn_create_flag_matches() {
        const ENTRIES: u32 = DATA + 0x1000;
        const WORLD_CELL: u32 = DATA + 0x2000;
        const WORLD_1: u32 = DATA + 0x2100;
        const WORLD_ARR: u32 = DATA + 0x2200;
        const SLOT_OBJ: u32 = DATA + 0x2300;
        const SLOT_TAB: u32 = DATA + 0x2400;
        const DEFAULT_CELL: u32 = DATA + 0x2500;
        const OBJ: u32 = DATA + 0x3000;
        const GP_FIRST_INIT: u32 = crate::difftest::GAME_GP + 0x2500;
        check(
            ee::s10::f_002fc9c8__generated,
            w_script_fn_create_flag,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x002f_c9c8, 5);
                c.pc = 0x002f_c9c8;
                c.g[A0][0] = (DATA + 0x500) as u64;
                let n = seed % 4;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = ENTRIES as u64;
                w32(c, GP_FIRST_INIT, (seed % 2 == 0) as u32);
                for idx in 0..9u32 {
                    w32(
                        c,
                        0x0044_5fc0 + 4 * idx,
                        if idx == 0 { 0 } else { r.next_u32() % 4 },
                    );
                }
                let (lo, hi) = (seed % 3, seed % 2);
                for i in 0..n {
                    let e = ENTRIES + 0x10 * i;
                    if i == 0 {
                        w32(c, e, 0);
                        w32(c, e + 4, (hi << 8) | lo);
                        w32(c, e + 0xc, 0);
                    } else {
                        w32(c, e, 1 + r.next_u32() % 8);
                        w32(c, e + 4, r.next_u32() % 1000);
                        w32(c, e + 0xc, r.next_u32() % 4);
                    }
                }
                w32(c, crate::difftest::GAME_GP + 0x16c8, WORLD_CELL);
                w32(c, WORLD_CELL, WORLD_1);
                w32(c, WORLD_1 + 8, WORLD_ARR);
                for i in 0..3 {
                    w32(
                        c,
                        WORLD_ARR + 4 * i,
                        if seed % 5 == 2 && i == lo {
                            0
                        } else {
                            SLOT_OBJ
                        },
                    );
                }
                w32(c, SLOT_OBJ + 0x1c, SLOT_TAB);
                for i in 0..2 {
                    let x = if seed % 9 == 4 {
                        0
                    } else {
                        ((OBJ >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    w32(c, SLOT_TAB + 4 * i, x);
                }
                w32(c, crate::difftest::GAME_GP + 0xce8, DEFAULT_CELL);
                w32(c, DEFAULT_CELL + 0x290, if seed % 6 == 5 { 0 } else { OBJ });
                w32(c, OBJ + 0xc, if seed % 4 == 3 { 0 } else { DATA + 0x3400 });
                w32(c, oc410_result(30), (seed % 3 == 0) as u32);
                w32(c, oc410_result(12), DATA + 0x3800);
                for k in 0..3 {
                    w32(c, 0x004c_9098 + 4 * k, r.next_u32());
                }
            },
        );
    }

    /// `LoadIndicator_Render`: the 2D state setup, nine virtual calls on the
    /// graphics manager and the pulsing size.
    #[test]
    fn load_indicator_render_matches() {
        const GRMAN: u32 = DATA + 0x1000;
        const STATES: u32 = DATA + 0x2000;
        const WORLD: u32 = DATA + 0x3000;
        const GP: u32 = crate::difftest::GAME_GP;
        check(
            ee::s06::f_00231fc0__generated,
            load_indicator_render,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0023_1fc0, 9);
                c.pc = 0x0023_1fc0;
                let this = DATA + 0x500;
                c.g[A0][0] = this as u64;
                w32(c, this, r.edge_f32());
                w32(c, this + 4, r.edge_f32());
                w32(c, this + 0xc, r.edge_f32());
                w32(c, GP.wrapping_sub(0x854), GRMAN);
                w32(c, GP.wrapping_sub(0x848), WORLD);
                w32(c, WORLD + 0xa0, r.next_u32());
                w32(c, GP.wrapping_sub(0x51a4), r.edge_f32());
                w32(c, GP.wrapping_sub(0x51a0), r.edge_f32());
                w32(c, GP.wrapping_sub(0x519c), r.edge_f32());
                w32(c, GRMAN + 0xe84, STATES);
                for k in 0..10 {
                    w32(c, STATES + 4 * k, r.next_u32());
                }
                for k in 0..5 {
                    w32(c, 0x0050_1420 + 4 * k, r.next_u32());
                }
                let vt = fakes::vtable(
                    c,
                    &mut r,
                    &[
                        0xa8, 0xf8, 0x110, 0x50, 0x58, 0xb8, 0xd0, 0x270, 0x100, 0xb0,
                    ],
                );
                w32(c, GRMAN + 0x10d8, vt);
                // Width and height come from two fakes that can return negatives.
                w32(c, vt + 0x54, 0x0014_dc80);
                w32(c, vt + 0x5c, 0x0014_dd58);
                let size = |r: &mut Rng, n: u32| {
                    if n % 3 == 0 {
                        0xffff_fff0
                    } else {
                        r.next_u32() % 700
                    }
                };
                w32(c, oc410_result(6), size(&mut r, seed));
                w32(c, oc410_result(7), size(&mut r, seed / 3));
                w32(c, oc410_fresult(32), r.edge_f32());
            },
        );
    }

    /// `AnimObject_ComputeNodeVelocities`: per-bone velocity from the
    /// animation derivative, with and without a parent bone.
    #[test]
    fn anim_object_compute_node_velocities_matches() {
        const THIS: u32 = DATA + 0x1000;
        const NODE_PTR: u32 = DATA + 0x1400;
        const INFO: u32 = DATA + 0x1500;
        const ENTRIES: u32 = DATA + 0x1600;
        const MATS: u32 = DATA + 0x4000;
        const NODES: u32 = DATA + 0x5000;
        const DELTAS: u32 = DATA + 0x6000;
        const APP: u32 = DATA + 0x7000;
        const GP: u32 = crate::difftest::GAME_GP;
        check(
            ee::s11::f_0034e798__generated,
            anim_object_compute_node_velocities,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0034_e798, 6);
                c.pc = 0x0034_e798;
                c.g[A0][0] = THIS as u64;
                c.bus.write16(THIS + 0x26, (seed % 2) as u16);
                w32(c, THIS + 4, r.edge_f32());
                // `this + 0x14` is the animation object (list node at +0x18).
                w32(c, THIS + 0x14 + 0x18, NODE_PTR);
                w32(c, NODE_PTR + 0x80, INFO);
                let bones = (seed / 2) % 4;
                w32(c, INFO + 4, bones);
                w32(c, INFO + 8, ENTRIES);
                for i in 0..3u32 {
                    let parent = if i == 0 || r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % i
                    };
                    w32(c, ENTRIES + 0x10 * i, parent);
                    w32(c, ENTRIES + 0x10 * i + 8, (r.next_u32() % 4 != 0) as u32);
                }
                w32(c, THIS + 0x44, MATS);
                w32(c, THIS + 0x48, NODES);
                w32(c, THIS + 0x4c, DELTAS);
                for k in 0..0x30 {
                    w32(c, MATS + 4 * k, r.edge_f32());
                    w32(c, DELTAS + 4 * k, r.edge_f32());
                }
                for k in 0..0x9c {
                    w32(c, NODES + 4 * k, r.edge_f32());
                }
                let vt = fakes::vtable(c, &mut r, &[0x198, 0x1a8]);
                w32(c, THIS + 0x20, vt);
                w32(c, GP.wrapping_add(0x2a74), APP);
                w32(c, APP + 0x10, r.next_u32() % 100);
                w32(c, GP.wrapping_sub(0x2aa4), r.edge_f32());
                crate::difftest::put_vec(
                    c,
                    0x004f_f120,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for k in 0..8 {
                    w32(c, oc410_out(33) + 4 * k, r.edge_f32());
                }
                w32(c, oc410_out(34), r.edge_f32());
                w32(c, oc410_out(34) + 4, r.edge_f32());
            },
        );
    }
}
