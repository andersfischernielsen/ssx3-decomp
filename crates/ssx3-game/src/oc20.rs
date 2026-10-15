//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_04a8, mem_pool32_free, "MEM_Pool32Free"),
    (0x0032_04b8, mem_pool64_free, "MEM_Pool64Free"),
    (0x0032_04d0, mem_pool128_free, "MEM_Pool128Free"),
    (0x0032_04e8, mem_pool256_free, "MEM_Pool256Free"),
    (0x0032_0500, mem_pool512_free, "MEM_Pool512Free"),
    (0x0032_0b68, math_ceil_log2, "Math_CeilLog2"),
    (0x0032_1ad8, input_parser_pad_channel_operand, "InputParser_PadChannelOperand"),
    (0x0032_5250, input_vm_init, "InputVM_Init"),
    (0x0032_52e8, input_vm_get_reg_ptr, "InputVM_GetRegPtr"),
    (0x0032_5430, input_vm_read_builtin, "InputVM_ReadBuiltin"),
    (0x0032_6478, input_preproc_next_char, "InputPreproc_NextChar"),
    (0x0032_6c60, pad_queue_flush, "PadQueue_Flush"),
    (0x0032_7810, ai_fwd_diff_cache_ctor, "AIFwdDiffCache_ctor"),
    (0x0032_7ac0, ai_fwd_diff_cache_touch, "AIFwdDiffCache_Touch"),
    (0x0032_c5a8, sphere_tree_get_half_leaf_radius, "SphereTree_GetHalfLeafRadius"),
    (0x0034_1e48, anim_object_step_loop, "AnimObject_StepLoop"),
    (0x0034_1ec0, anim_object_step_ping_pong, "AnimObject_StepPingPong"),
    (0x0034_1f38, anim_object_step_clamp, "AnimObject_StepClamp"),
    (0x0034_2768, boost_modifier_ctor, "BoostModifier_ctor"),
    (0x0034_5700, modifier_list_push_front, "ModifierList_PushFront"),
    (0x0034_5760, modifier_list_unlink, "ModifierList_Unlink"),
    (0x0034_5798, particle_modifier_base_ctor, "ParticleModifier_BaseCtor"),
    (0x0034_af20, c_flag_manager_group_init_key, "cFlagManager_GroupInitKey"),
    (0x0034_d6f0, fence_add_impulse, "Fence_AddImpulse"),
    (0x0034_e320, anim_object_get_node_intersect_matrix, "AnimObject_GetNodeIntersectMatrix"),
    (0x0035_1170, instances_man_alloc, "InstancesMan_Alloc"),
    (0x0035_1260, instances_man_free, "InstancesMan_Free"),
    (0x0035_12c0, instances_man_resolve_handle, "InstancesMan_ResolveHandle"),
    (0x0035_1398, intersect_info_set_source, "IntersectInfo_SetSource"),
    (0x0035_1508, intersect_info_get_duration, "IntersectInfo_GetDuration"),
    (0x0035_2aa8, modifier_block_ctor, "ModifierBlock_ctor"),
    (0x0035_3ac0, sky_box_ctor, "SkyBox_ctor"),
    (0x0035_3da8, flex_bridge_springs_ctor, "FlexBridge_SpringsCtor"),
    (0x0035_4c08, process_list_add, "ProcessList_Add"),
    (0x0035_4e48, process_list_remove, "ProcessList_Remove"),
    (0x0035_4ea8, process_list_first, "ProcessList_First"),
    (0x0035_4ed0, process_list_last, "ProcessList_Last"),
    (0x0035_4f20, process_list_prev, "ProcessList_Prev"),
    (0x0035_4f70, process_list_pending_next, "ProcessList_PendingNext"),
    (0x0035_5260, process_list_set_sorted_insert, "ProcessList_SetSortedInsert"),
    (0x0035_81b8, multi_particle_add_instance, "MultiParticle_AddInstance"),
    (0x0035_9688, spline_modifier_set_speed, "SplineModifier_SetSpeed"),
    (0x0035_a550, multi_spline_modifier_set_speed, "MultiSplineModifier_SetSpeed"),
    (0x0035_b670, rail_modifier_list_push_front, "RailModifier_ListPushFront"),
    (0x0035_b6d0, rail_modifier_list_unlink, "RailModifier_ListUnlink"),
    (0x0035_f6e8, uv_scroll_modifier_ctor, "UVScrollModifier_ctor"),
    (0x0035_fb30, uv_scroll_modifier_set_param, "UVScrollModifier_SetParam"),
    (0x0036_7b60, tex_man_unlock_vram, "TexMan_UnlockVram"),
    (0x0036_8138, tex_man_emit_path3_window, "TexMan_EmitPath3Window"),
    (0x0036_9130, tex_man_set_tex1_filter, "TexMan_SetTex1Filter"),
];

/// `cvt.w.s`: the bits of the truncated float (same as the FPU op).
fn cvt_w_s(x: FpuF32) -> FpuF32 {
    FpuF32(float::to_fixed(x.0, 0))
}

/// A float constant read relative to `gp` (game globals / `.lit4`).
fn gp_float(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr((c.g[GP][0] as u32).wrapping_sub(off)).read_fpu(c)
}

// ---------------------------------------------------------------------------
// Memory-pool free lists: push a block onto the singly linked free list stored
// at a fixed offset inside the pool's base (the new head is written into the
// block's `next` word and into the pool head).

/// `void MEM_Pool32Free(void *pool, void *block)` (0x3204a8): push `block` on
/// the 32-byte pool's free list whose head lives at `pool + 0x6000`.
pub fn mem_pool32_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = Addr(abi::arg(c, 0));
    let block = c.g[A1][0];
    let head = pool.off(0x6000).lw(c);
    Addr(block as u32).sw(c, head);
    pool.off(0x6000).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool64Free(void *pool, void *block)` (0x3204b8): the 64-byte pool
/// is based at `pool + 0x30000`, head at `+0x2000`.
pub fn mem_pool64_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x30000));
    c.g[V1][0] = 0x30000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).off(0x2000).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).off(0x2000).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool128Free(void *pool, void *block)` (0x3204d0): 128-byte pool
/// based at `pool + 0x20000`, head at `+0x5800`.
pub fn mem_pool128_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x20000));
    c.g[V1][0] = 0x20000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).off(0x5800).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).off(0x5800).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool256Free(void *pool, void *block)` (0x3204e8): 256-byte pool
/// based at `pool + 0x20000`, head at its very start.
pub fn mem_pool256_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x20000));
    c.g[V1][0] = 0x20000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void MEM_Pool512Free(void *pool, void *block)` (0x320500): same layout as
/// the 256-byte pool.
pub fn mem_pool512_free(c: &mut Cpu) {
    c.cycles += 6;
    let base = sx(abi::arg(c, 0).wrapping_add(0x20000));
    c.g[V1][0] = 0x20000;
    c.g[A0][0] = base;
    let block = c.g[A1][0];
    let head = Addr(base as u32).lw(c);
    Addr(block as u32).sw(c, head);
    Addr(base as u32).sw(c, block);
    c.g[V0][0] = head;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Math.

/// `int Math_CeilLog2(uint x)` (0x320b68): bit-smear `x` to one less than the
/// next power of two and return the number of leading zeros of that
/// (i.e. the ceiling log2, 31 for 0/1).
pub fn math_ceil_log2(c: &mut Cpu) {
    c.cycles += 33;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V1][0] = 0xffff_ffff_ff00_0000;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = 0xffff_ffff_f000_0000;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 31) as u32);
    c.g[A3][0] = 0xffff_ffff_c000_0000;
    c.g[T0][0] = c.g[V0][0] & 0x10;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = sx((c.g[A0][0] as u32) << (c.g[T0][0] as u32 & 31));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 31) as u32);
    c.g[A0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = c.g[V1][0] & 0x8;
    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 31) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 31) as u32);
    c.g[V1][0] = c.g[A2][0] & 0x4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 31) as u32);
    c.g[V1][0] = c.g[A3][0] & 0x2;
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 31) as u32);
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.g[T0][0] ^ 0x1f;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Input system.

/// `int InputParser_PadChannelOperand(int player, int channel, int base)`
/// (0x321ad8): `base + player + channel * 6 + 0x60`.
pub fn input_parser_pad_channel_operand(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x6;
    let scaled = abi::mult(c, abi::arg(c, 1), 6);
    c.g[A0][0] = sx(scaled);
    let base = scaled.wrapping_add(abi::arg(c, 2));
    c.g[V0][0] = sx(base.wrapping_add(0x60));
    abi::ret(c);
}

/// `void InputVM_Init(InputVM *vm)` (0x325250): clear the first word and
/// return the object.
pub fn input_vm_init(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}

/// `float* InputVM_GetRegPtr(InputVM *vm, int index)` (0x3252e8): the register
/// slot at `vm + index*4 + 8`.
pub fn input_vm_get_reg_ptr(c: &mut Cpu) {
    c.cycles += 4;
    let off = abi::arg(c, 1).wrapping_shl(2).wrapping_add(8);
    c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(off));
    abi::ret(c);
}

/// `float InputVM_ReadBuiltin(InputVM *vm, int index)` (0x32525430): 0.0 for
/// index 0, otherwise the builtin float at `vm + index*4 + 0x108`.
pub fn input_vm_read_builtin(c: &mut Cpu) {
    c.cycles += 2;
    let index = abi::arg64(c, 1);
    let off = abi::arg(c, 1).wrapping_shl(2);
    c.g[V0][0] = sx(off);
    if index == 0 {
        c.cycles += 3;
        c.f[0] = 0;
    } else {
        c.cycles += 3;
        let ptr = abi::arg(c, 0).wrapping_add(off);
        c.g[V0][0] = sx(ptr);
        c.f[0] = Addr(ptr).off(0x108).read_fpu(c).0;
    }
    abi::ret(c);
}

/// `int InputPreproc_NextChar(InputPreproc *p, InputPreproc* /*cursor*/)`
/// (0x326478): advance the read cursor past the byte at the end of the
/// buffered range; store the byte (or -1) in the output word.
pub fn input_preproc_next_char(c: &mut Cpu) {
    c.cycles += 6;
    let out = Addr(abi::arg(c, 0));
    let cur = Addr(abi::arg(c, 1));
    let start = cur.lw(c);
    let len = cur.off(4).lw(c);
    let ptr = cur.off(8).lw(c);
    let end = sx((start as u32).wrapping_add(len as u32));
    c.g[V1][0] = len;
    c.g[A2][0] = ptr;
    c.g[V0][0] = end;
    if ptr != end {
        let byte = Addr(ptr as u32).lbu(c);
        c.cycles += 4;
        c.g[V0][0] = byte;
        let next = sx((ptr as u32).wrapping_add(1));
        c.g[V1][0] = next;
        out.off(0xc).sw(c, byte);
        cur.off(8).sw(c, next);
    } else {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        out.off(0xc).sw(c, u64::MAX);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pad queue.

/// `int PadQueue_Flush(PadQueue *q)` (0x326c60): wrap the write cursor at
/// `+0x2ee4` by 30 and return the remainder of that against the count at
/// `+0x2ee0`.
pub fn pad_queue_flush(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    let cursor = q.off(0x2ee4).lw(c);
    c.g[V1][0] = cursor;
    c.g[A1][0] = 0x1e;
    c.cycles += 12;
    c.g[T0][0] = 0x1e;
    let next = sx((cursor as u32).wrapping_add(0x1d));
    c.g[V0][0] = next;
    let count = q.off(0x2ee0).lw(c);
    c.g[A2][0] = count;
    let (_, rem) = abi::div(c, next as u32, 0x1e);
    let wrapped = sx((cursor as u32).wrapping_sub(count as u32).wrapping_add(0x1e));
    c.g[V1][0] = wrapped;
    c.g[A3][0] = sx(rem);
    let (_, rem2) = abi::div(c, wrapped as u32, 0x1e);
    q.off(0x2ee0).sw(c, sx(rem));
    c.g[V0][0] = sx(rem2);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Forward-difference cache.

/// `AIFwdDiffCache* AIFwdDiffCache_ctor(AIFwdDiffCache *c)` (0x327810): zero
/// the four head words.
pub fn ai_fwd_diff_cache_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(4).sw(c, 0);
    this.off(8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Forward-difference cache (continued).

/// `void AIFwdDiffCache_Touch(AIFwdDiffCache *cache, void *node)`
/// (0x327ac0): move `node` to the front of the bucket it currently sits in,
/// or unlink it first when it is not the bucket head.
pub fn ai_fwd_diff_cache_touch(c: &mut Cpu) {
    c.cycles += 4;
    let cache = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = cache as u64;
    let cur = Addr(cache).off(4).lw(c);
    c.g[V0][0] = cur;
    if cur != node as u64 {
        let next = Addr(cur as u32).off(8).lw(c);
        c.g[V0][0] = next;
        c.cycles += 2;
        if next != node as u64 {
            c.cycles += 16;
            let a = Addr(node).off(8).lw(c);
            c.g[V1][0] = a;
            let b = Addr(node).off(4).lw(c);
            c.g[V0][0] = b;
            Addr(a as u32).off(4).sw(c, b);
            let c1 = Addr(node).off(8).lw(c);
            c.g[V0][0] = c1;
            let d = Addr(node).off(4).lw(c);
            c.g[A0][0] = d;
            Addr(d as u32).off(8).sw(c, c1);
            let e = Addr(cache).off(4).lw(c);
            c.g[V1][0] = e;
            let f = Addr(e as u32).off(8).lw(c);
            c.g[V0][0] = f;
            Addr(node).off(8).sw(c, f);
            let g = Addr(cache).off(4).lw(c);
            c.g[V1][0] = g;
            Addr(node).off(4).sw(c, g);
            let h = Addr(cache).off(4).lw(c);
            c.g[V0][0] = h;
            let i = Addr(h as u32).off(8).lw(c);
            c.g[V1][0] = i;
            Addr(i as u32).off(4).sw(c, node as u64);
            let j = Addr(cache).off(4).lw(c);
            c.g[V0][0] = j;
            Addr(j as u32).off(8).sw(c, node as u64);
        }
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 3;
        let v = Addr(node).off(4).lw(c);
        c.g[V0][0] = v;
        Addr(cache).off(4).sw(c, v);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Sphere tree.

/// `float SphereTree_GetHalfLeafRadius(SphereTree *tree)` (0x32c5a8): half the
/// radius read through `tree->leaves[?]`, scaled by the value at `+0x90`.
pub fn sphere_tree_get_half_leaf_radius(c: &mut Cpu) {
    c.cycles += 13;
    let tree = Addr(abi::arg(c, 0));
    let leaf = tree.off(0x98).lw(c);
    c.g[V1][0] = leaf;
    c.g[A1][0] = 0xc;
    c.f[2] = tree.off(0x90).read_fpu(c).0;
    let a = Addr(leaf as u32).off(0xc).lw(c);
    let b = Addr(leaf as u32).off(0x20).lw(c);
    let scaled = abi::mult(c, a as u32, 0xc);
    c.g[V0][0] = sx(scaled);
    c.g[A0][0] = b;
    c.g[AT][0] = 0x3f00_0000;
    c.f[1] = 0x3f00_0000; // 0.5, as `lui 0x3f00` + `mtc1 $at, $f1`
    let entry = sx(scaled.wrapping_add(b as u32));
    c.g[V0][0] = entry;
    c.f[0] = Addr(entry as u32).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Anim objects: advance a parameter toward its loop bounds.

/// `void AnimObject_StepLoop(AnimObject *o)` (0x341e48): wrap the value at
/// `+0x1c` into `[+0x10, +0x14[, +0x18)`.
pub fn anim_object_step_loop(c: &mut Cpu) {
    let obj = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.f[0] = 0;
    c.f[1] = obj.off(0x10).read_fpu(c).0;
    let cc = FpuF32(c.f[0]).le(FpuF32(c.f[1]));
    abi::set_fcc(c, cc);
    c.f[0] = obj.off(0x1c).read_fpu(c).0;
    if !cc {
        c.cycles += 6;
        c.f[2] = obj.off(0x14).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[0]).lt(FpuF32(c.f[2]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if cc2 {
            c.cycles += 4;
            c.f[1] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
            c.f[0] = obj.off(0x18).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        }
    } else {
        c.cycles += 6;
        c.f[2] = obj.off(0x18).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if cc2 {
            c.cycles += 5;
            c.f[1] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
            c.f[0] = obj.off(0x14).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
            obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AnimObject_StepPingPong(AnimObject *o)` (0x341ec0): advance `+0x1c`
/// within `[+0x10, +0x14]`, reversing the sign of the rate at `+0x10` at a
/// bound.
pub fn anim_object_step_ping_pong(c: &mut Cpu) {
    let obj = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.f[0] = 0;
    c.f[1] = obj.off(0x10).read_fpu(c).0;
    let cc = FpuF32(c.f[0]).le(FpuF32(c.f[1]));
    abi::set_fcc(c, cc);
    c.f[0] = obj.off(0x1c).read_fpu(c).0;
    if cc {
        c.cycles += 6;
        c.f[2] = obj.off(0x18).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if !cc2 {
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 4;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
            c.f[1] = (-FpuF32(c.f[1])).0;
            c.f[0] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
        }
    } else {
        c.cycles += 6;
        c.f[2] = obj.off(0x14).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let cc2 = FpuF32(c.f[0]).lt(FpuF32(c.f[2]));
        abi::set_fcc(c, cc2);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
        obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
        if !cc2 {
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 3;
            c.f[0] = (FpuF32(c.f[2]) - FpuF32(c.f[0])).0;
            c.f[1] = (-FpuF32(c.f[1])).0;
            c.f[0] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
        }
    }
    c.cycles += 2;
    obj.off(0x10).write_fpu(c, FpuF32(c.f[1]));
    obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
    c.cycles += 2;
    abi::ret(c);
}

/// `void AnimObject_StepClamp(AnimObject *o)` (0x341f38): clamp `+0x1c` to the
/// bounds at `+0x10`/`+0x14` (or `+0x18`), flagging the hit in `+0x8`.
pub fn anim_object_step_clamp(c: &mut Cpu) {
    let obj = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.f[1] = 0;
    c.f[0] = obj.off(0x10).read_fpu(c).0;
    let cc = FpuF32(c.f[1]).le(FpuF32(c.f[0]));
    abi::set_fcc(c, cc);
    c.f[1] = obj.off(0x1c).read_fpu(c).0;
    if !cc {
        // min < 0 path (bounds +0x14)
        c.cycles += 5;
        c.f[0] = obj.off(0x14).read_fpu(c).0;
        let eq = FpuF32(c.f[1]).eq_s(FpuF32(c.f[0]));
        abi::set_fcc(c, eq);
        c.g[V0][0] = 0x1;
        if eq {
            c.cycles += 1;
            obj.off(8).sw(c, 1);
        }
        c.cycles += 6;
        c.f[0] = obj.off(0x1c).read_fpu(c).0;
        c.f[1] = obj.off(0x10).read_fpu(c).0;
        c.f[2] = obj.off(0x14).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let clt = FpuF32(c.f[0]).lt(FpuF32(c.f[2]));
        abi::set_fcc(c, clt);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
    } else {
        // min >= 0 path (bounds +0x18)
        c.cycles += 5;
        c.f[0] = obj.off(0x18).read_fpu(c).0;
        let eq = FpuF32(c.f[1]).eq_s(FpuF32(c.f[0]));
        abi::set_fcc(c, eq);
        c.g[V0][0] = 0x1;
        if eq {
            c.cycles += 1;
            obj.off(8).sw(c, 1);
        }
        c.cycles += 7;
        c.f[0] = obj.off(0x1c).read_fpu(c).0;
        c.f[1] = obj.off(0x10).read_fpu(c).0;
        c.f[2] = obj.off(0x18).read_fpu(c).0;
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        let clt = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
        abi::set_fcc(c, clt);
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[0]));
    }
    c.cycles += 2;
    let t = !c.fcc();
    obj.off(0x28).write_fpu(c, FpuF32(c.f[0]));
    if !t {
        c.cycles += 1;
        obj.off(0x1c).write_fpu(c, FpuF32(c.f[2]));
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Boost modifier.

/// `BoostModifier* BoostModifier_ctor(BoostModifier *m, void *desc, void *o)`
/// (0x342768): build a boost modifier from its descriptor.
pub fn boost_modifier_ctor(c: &mut Cpu) {
    c.cycles += 39;
    let this = Addr(abi::arg(c, 0));
    let desc = Addr(abi::arg(c, 1));
    let a2_in = c.g[A2][0];
    c.g[V1][0] = 0x490000;
    c.g[V0][0] = 0x1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x898));
    this.off(0x40).sw(c, a2_in);
    this.off(4).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    this.sw(c, c.g[V0][0]);
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = desc.off(4).lh(c);
    c.g[AT][0] = 0x42c80000;
    c.f[3] = c.g[AT][0] as u32;
    this.off(0x30).sw(c, c.g[V1][0]);
    c.g[V1][0] = c.g[V1][0] ^ 0x1;
    c.f[1] = Addr(c.g[A2][0] as u32).off(0x10).read_fpu(c).0;
    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
    c.f[0] = desc.off(8).read_fpu(c).0;
    Addr(c.g[SP][0] as u32).off(0xc).sw(c, 0);
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    c.f[1] = cvt_w_s(FpuF32(c.f[0])).0;
    c.g[A2][0] = sx(c.f[1]);
    this.off(0x28).sw(c, c.g[A2][0]);
    if c.g[V1][0] != 0 {
        c.g[A2][0] = 0;
    }
    c.f[0] = desc.off(0x14).read_fpu(c).0;
    c.f[1] = desc.off(0x18).read_fpu(c).0;
    c.f[2] = desc.off(0x1c).read_fpu(c).0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp + 4).write_fpu(c, FpuF32(c.f[1]));
    Addr(sp + 8).write_fpu(c, FpuF32(c.f[2]));
    let quad = Addr(sp).read_vec4(c);
    abi::set_gpr128(c, V1, quad.to_bits());
    this.off(0x10).write_vec4(c, quad);
    c.f[0] = desc.off(0x10).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[3])).0;
    this.off(0x20).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0xc).read_fpu(c).0;
    this.off(0x2c).sw(c, c.g[A2][0]);
    this.off(0x24).write_fpu(c, FpuF32(c.f[1]));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Modifier lists.

/// `void ModifierList_PushFront(ModifierList *list, Modifier *m)`
/// (0x345700): push `m` at the head.
pub fn modifier_list_push_front(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let m = c.g[A1][0];
    let head = list.lw(c);
    c.g[V0][0] = head;
    if head != 0 {
        Addr(head as u32).off(4).sw(c, m);
    }
    c.cycles += 5;
    Addr(m as u32).off(4).sw(c, list.0 as u64);
    c.g[V0][0] = list.lw(c);
    Addr(m as u32).sw(c, c.g[V0][0]);
    list.sw(c, m);
    abi::ret(c);
}

/// `void ModifierList_Unlink(Modifier *m)` (0x345760): unlink `m` from its
/// doubly linked list.
pub fn modifier_list_unlink(c: &mut Cpu) {
    c.cycles += 3;
    let m = Addr(abi::arg(c, 0));
    c.g[V1][0] = m.lw(c);
    if c.g[V1][0] == 0 {
        c.g[V1][0] = m.off(4).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = m.off(4).lw(c);
        Addr(c.g[V1][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[V1][0] = m.off(4).lw(c);
    }
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        m.off(4).sw(c, 0);
    } else {
        c.cycles += 3;
        c.g[V0][0] = m.lw(c);
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        m.off(4).sw(c, 0);
    }
    c.cycles += 2;
    m.sw(c, 0);
    abi::ret(c);
}

/// `ParticleModifier* ParticleModifier_BaseCtor(ParticleModifier *m, void *o,
/// ...)` (0x345798): initialise the base fields and bump the instance counter.
pub fn particle_modifier_base_ctor(c: &mut Cpu) {
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xf04)).lw(c);
    c.g[V0][0] = 0x490000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
    this.off(0xc).sw(c, c.g[A1][0]);
    this.off(8).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    this.sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(4).sw(c, 0);
    Addr((c.g[GP][0] as u32).wrapping_add(0xf04)).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void cFlagManager_GroupInitKey(cFlagManager_Group *g)` (0x34af20): clear
/// the group's key fields (state -1 at +0x58, +0x5c = 0).
pub fn c_flag_manager_group_init_key(c: &mut Cpu) {
    c.cycles += 6;
    let g = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    g.off(0x5c).sw(c, 0);
    g.off(0x58).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Fence.

/// `void Fence_AddImpulse(Fence *f, float impulse)` (0x34d6f0): add an impulse
/// to the fence's sway, clamping to 1.0, and damp the velocity.
pub fn fence_add_impulse(c: &mut Cpu) {
    let fence = Addr(abi::arg(c, 0));
    c.cycles += 9;
    c.f[0] = fence.off(0x30).read_fpu(c).0;
    c.f[1] = FpuF32(c.f[12]).abs().0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = 0x3f80_0000;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    let cc = FpuF32(c.f[2]).lt(FpuF32(c.f[0]));
    abi::set_fcc(c, cc);
    fence.off(0x30).write_fpu(c, FpuF32(c.f[0]));
    if cc {
        c.cycles += 1;
        fence.off(0x30).write_fpu(c, FpuF32(c.f[2]));
    }
    c.cycles += 5;
    c.f[1] = fence.off(0x34).read_fpu(c).0;
    c.f[0] = 0;
    let is_zero = FpuF32(c.f[1]).eq_s(FpuF32::ZERO);
    abi::set_fcc(c, is_zero);
    if is_zero {
        c.cycles += 4;
        let neg = FpuF32(c.f[12]).lt(FpuF32::ZERO);
        abi::set_fcc(c, neg);
        c.f[0] = fence.off(0x30).read_fpu(c).0;
        if neg {
            c.cycles += 3;
            c.f[1] = gp_float(c, 0x2ab4).0;
        } else {
            c.cycles += 2;
            c.f[1] = gp_float(c, 0x2aac).0;
        }
    } else {
        c.cycles += 4;
        let neg = FpuF32(c.f[1]).lt(FpuF32::ZERO);
        abi::set_fcc(c, neg);
        c.f[0] = fence.off(0x30).read_fpu(c).0;
        if neg {
            c.cycles += 3;
            c.f[1] = gp_float(c, 0x2ab0).0;
        } else {
            c.cycles += 2;
            c.f[1] = gp_float(c, 0x2aac).0;
        }
    }
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
    c.cycles += 2;
    fence.off(0x34).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Anim object node intersection matrix.

/// `void* AnimObject_GetNodeIntersectMatrix(AnimObject *o, int node)`
/// (0x34e320): the intersect matrix of `node`, or a default when there is no
/// node table.
pub fn anim_object_get_node_intersect_matrix(c: &mut Cpu) {
    c.cycles += 3;
    let node = abi::arg(c, 1);
    let table = Addr(abi::arg(c, 0)).off(0x48).lw(c);
    c.g[V1][0] = table;
    if table == 0 {
        c.g[V0][0] = 0x500000;
        c.cycles += 2;
        c.g[V0][0] = sx(0x500000u32.wrapping_sub(0xe60));
        abi::ret(c);
    } else {
        c.cycles += 5;
        c.g[V0][0] = 0xd0;
        let off = abi::mult(c, node, 0xd0);
        c.g[A0][0] = sx(off);
        let ptr = off.wrapping_add(table as u32);
        c.g[V0][0] = sx(ptr.wrapping_add(0x90));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Instance manager.

/// `void* InstancesMan_Alloc(InstancesMan *man)` (0x351170): pop a free
/// instance from `man->free` (`+8`), marking it used and rebuilding its
/// handle.
pub fn instances_man_alloc(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.g[A0][0];
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    if c.g[A3][0] == 0 {
        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
    }
    c.cycles += 16;
    c.g[A2][0] = 0xffff_ffff_ba2e_0000;
    c.g[A2][0] |= 0x8ba3;
    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    Addr(c.g[A3][0] as u32).off(0x78).sb(c, c.g[V0][0]);
    c.g[A0][0] |= 0x2000;
    c.g[V0][0] = c.g[A3][0];
    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(0x78).lbu(c);
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[A3][0] as u32).off(0x8).sw(c, c.g[A0][0]);
    c.g[V1][0] = sx(abi::mult(c, c.g[V1][0] as u32, c.g[A2][0] as u32));
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[A1][0] |= c.g[V1][0];
    Addr(c.g[A3][0] as u32).off(0x78).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void InstancesMan_Free(InstancesMan *man, void *inst)` (0x351260): push
/// `inst` on the free list.
pub fn instances_man_free(c: &mut Cpu) {
    c.cycles += 4;
    let man = Addr(abi::arg(c, 0));
    let inst = c.g[A1][0];
    c.g[V0][0] = man.off(8).lw(c);
    Addr(inst as u32).sw(c, c.g[V0][0]);
    man.off(8).sw(c, inst);
    abi::ret(c);
}

/// `void* InstancesMan_ResolveHandle(InstancesMan *man, uint handle)`
/// (0x3512c0): resolve a packed index/handle to an instance.
pub fn instances_man_resolve_handle(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    let matches = c.g[V0][0] == c.g[V1][0];
    c.g[V1][0] = sx((c.g[A1][0] as u32) >> 8);
    if !matches {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 21;
    c.g[V0][0] = 0xb0;
    c.g[V1][0] = sx(abi::mult(c, c.g[V1][0] as u32, 0xb0));
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lbu(c);
    c.g[A3][0] = 0xffff_ffff_ba2e_0000;
    c.g[A3][0] |= 0x8ba3;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x78).sb(c, c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = Addr(c.g[A0][0] as u32).off(0x78).lbu(c);
    c.g[A1][0] |= 0x2000;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x8).sw(c, c.g[A1][0]);
    c.g[V1][0] = sx(abi::mult(c, c.g[V1][0] as u32, c.g[A3][0] as u32));
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[A2][0] |= c.g[V1][0];
    Addr(c.g[A0][0] as u32).off(0x78).sw(c, c.g[A2][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Intersect info.

/// `void IntersectInfo_SetSource(IntersectInfo *info, const void *src)`
/// (0x351398): copy six floats from the source vector into `info`.
pub fn intersect_info_set_source(c: &mut Cpu) {
    c.cycles += 14;
    let info = Addr(abi::arg(c, 0));
    let src = Addr(abi::arg(c, 1));
    info.off(0x40).sw(c, c.g[A1][0]);
    c.f[0] = src.read_fpu(c).0;
    info.off(0x44).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = src.off(4).read_fpu(c).0;
    info.off(0x48).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = src.off(8).read_fpu(c).0;
    info.off(0x4c).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = src.off(0xc).read_fpu(c).0;
    info.off(0x50).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = src.off(0x10).read_fpu(c).0;
    info.off(0x54).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = src.off(0x14).read_fpu(c).0;
    info.off(0x58).write_fpu(c, FpuF32(c.f[1]));
    abi::ret(c);
}

/// `float IntersectInfo_GetDuration(IntersectInfo *info)` (0x351508): the
/// duration from the current action, or 0.0.
pub fn intersect_info_get_duration(c: &mut Cpu) {
    c.cycles += 5;
    let info = Addr(abi::arg(c, 0));
    let action = info.off(0x40).lw(c);
    c.f[0] = 0;
    let v0 = Addr(action as u32).off(0x20).lw(c);
    c.g[V0][0] = v0;
    let v1 = Addr(v0 as u32).lw(c);
    c.g[V1][0] = v1;
    if v0 != 0 && v1 != 0 {
        c.cycles += 5;
        let ptr = Addr(v1 as u32).off(4).lw(c);
        c.g[V0][0] = ptr;
        if ptr != 0 {
            c.f[0] = Addr(ptr as u32).off(0x10).read_fpu(c).0;
        }
    } else if v0 != 0 {
        c.cycles += 2;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Assorted constructors.

/// `ModifierBlock* ModifierBlock_ctor(ModifierBlock *b)` (0x352aa8): install
/// the vtable pointers and clear the block.
pub fn modifier_block_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let b = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = 0x490000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1340));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1200));
    b.off(0x18).sw(c, c.g[V0][0]);
    b.off(0x24).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    b.off(0x10).sw(c, 0);
    b.off(0x14).sw(c, 0);
    b.off(0x1c).sw(c, 0);
    b.off(0x20).sw(c, 0);
    b.sw(c, 0);
    b.off(4).sw(c, 0);
    b.off(8).sw(c, 0);
    b.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `SkyBox* SkyBox_ctor(SkyBox *s)` (0x353ac0): install the vtable pointer and
/// clear the sky box.
pub fn sky_box_ctor(c: &mut Cpu) {
    c.cycles += 10;
    let s = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xff8));
    s.off(0x10).sw(c, c.g[V1][0]);
    s.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    s.off(4).sw(c, 0);
    s.off(8).sw(c, 0);
    s.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `FlexBridge_Springs* FlexBridge_SpringsCtor(FlexBridge_Springs *s)`
/// (0x353da8): return the object with its spring count cleared.
pub fn flex_bridge_springs_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).off(0x24).sw(c, 0);
    abi::ret(c);
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

// ---------------------------------------------------------------------------
// Particles / modifiers.

/// `void MultiParticle_AddInstance(MultiParticle *p, void *inst)`
/// (0x3581b8): append `inst` to the instance array if there is room.
pub fn multi_particle_add_instance(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[A1][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SplineModifier_SetSpeed(SplineModifier *m, float speed)` (0x359688):
/// store `speed * <constant>` into `m + 0x48`.
pub fn spline_modifier_set_speed(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.f[0] = gp_float(c, 0x2a60).0;
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    this.off(0x48).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}

/// `void MultiSplineModifier_SetSpeed(MultiSplineModifier *m, float speed)`
/// (0x35a550): store `speed * <constant>` into `m + 0x14`.
pub fn multi_spline_modifier_set_speed(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.f[0] = gp_float(c, 0x2a28).0;
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    this.off(0x14).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}

/// `void RailModifier_ListPushFront(RailModifierList *list, void *m)`
/// (0x35b670): push `m` at the head.
pub fn rail_modifier_list_push_front(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let m = c.g[A1][0];
    let head = list.lw(c);
    c.g[V0][0] = head;
    if head != 0 {
        Addr(head as u32).off(4).sw(c, m);
    }
    c.cycles += 5;
    Addr(m as u32).off(4).sw(c, list.0 as u64);
    c.g[V0][0] = list.lw(c);
    Addr(m as u32).sw(c, c.g[V0][0]);
    list.sw(c, m);
    abi::ret(c);
}

/// `void RailModifier_ListUnlink(void *m)` (0x35b6d0): unlink `m` from its
/// doubly linked list.
pub fn rail_modifier_list_unlink(c: &mut Cpu) {
    c.cycles += 3;
    let m = Addr(abi::arg(c, 0));
    c.g[V1][0] = m.lw(c);
    if c.g[V1][0] == 0 {
        c.g[V1][0] = m.off(4).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = m.off(4).lw(c);
        Addr(c.g[V1][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[V1][0] = m.off(4).lw(c);
    }
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        m.off(4).sw(c, 0);
    } else {
        c.cycles += 3;
        c.g[V0][0] = m.lw(c);
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        m.off(4).sw(c, 0);
    }
    c.cycles += 2;
    m.sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UV scroll modifier.

/// `UVScrollModifier* UVScrollModifier_ctor(UVScrollModifier *m, void *desc)`
/// (0x35f6e8): build the modifier from its descriptor.
pub fn uv_scroll_modifier_ctor(c: &mut Cpu) {
    c.cycles += 40;
    let this = Addr(abi::arg(c, 0));
    let desc = Addr(abi::arg(c, 1));
    c.g[V0][0] = 0x1;
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    this.off(0x40).sw(c, c.g[V0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[AT][0] = 0x3f80_0000;
    c.f[2] = 0x3f80_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = desc.off(4).lw(c);
    this.sw(c, c.g[V1][0]);
    c.f[0] = desc.off(8).read_fpu(c).0;
    this.off(0x30).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0xc).read_fpu(c).0;
    this.off(0x34).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = desc.off(0x18).read_fpu(c).0;
    this.off(8).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0x1c).read_fpu(c).0;
    this.off(4).sw(c, 0);
    this.off(0xc).write_fpu(c, FpuF32(c.f[1]));
    this.off(0x10).sw(c, 0);
    c.f[1] = Addr(c.g[A2][0] as u32).off(0x10).read_fpu(c).0;
    c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
    c.f[0] = desc.off(0x20).read_fpu(c).0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).off(0xc).sw(c, 0);
    c.f[2] = FpuF32(c.f[2]).div(FpuF32(c.f[1])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
    this.off(0x14).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0x24).read_fpu(c).0;
    c.f[0] = desc.off(0x28).read_fpu(c).0;
    c.f[2] = desc.off(0x2c).read_fpu(c).0;
    Addr(sp).write_fpu(c, FpuF32(c.f[1]));
    Addr(sp + 4).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp + 8).write_fpu(c, FpuF32(c.f[2]));
    let quad = Addr(sp).read_vec4(c);
    abi::set_gpr128(c, V1, quad.to_bits());
    this.off(0x20).write_vec4(c, quad);
    c.f[0] = desc.off(0x10).read_fpu(c).0;
    this.off(0x38).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = desc.off(0x14).read_fpu(c).0;
    this.off(0x3c).write_fpu(c, FpuF32(c.f[1]));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

/// `void UVScrollModifier_SetParam(UVScrollModifier *m, int param, float v)`
/// (0x35fb30): store `v` into the field selected by `param`, clamped to the
/// parameter's range.
pub fn uv_scroll_modifier_set_param(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 4;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xc8));
    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xca));
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xcc));
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            // param 0xcc/0xcd: field 0x34/0x30, range [-4, 4]
            c.cycles += 5;
            c.g[AT][0] = 0xffff_ffff_c080_0000;
            c.f[0] = 0xc080_0000;
            let cc = FpuF32(c.f[0]).le(FpuF32(c.f[12]));
            abi::set_fcc(c, cc);
            if cc {
                c.cycles += 6;
                c.g[AT][0] = 0x4080_0000;
                c.f[0] = 0x4080_0000;
                let cc2 = FpuF32(c.f[12]).le(FpuF32(c.f[0]));
                abi::set_fcc(c, cc2);
                c.g[V0][0] = 0xcc;
                if cc2 {
                    c.cycles += 2;
                    if abi::arg(c, 1) != 0xcc {
                        this.off(0x34).write_fpu(c, FpuF32(c.f[12]));
                    } else {
                        c.cycles += 2;
                        this.off(0x30).write_fpu(c, FpuF32(c.f[12]));
                        abi::ret(c);
                        return;
                    }
                }
            }
        } else {
            // param 0xca/0xcb: field 0xc/0x8, range [0, 60]
            c.cycles += 4;
            c.f[0] = 0;
            let cc = FpuF32(c.f[0]).le(FpuF32(c.f[12]));
            abi::set_fcc(c, cc);
            if cc {
                c.cycles += 6;
                c.g[AT][0] = 0x4270_0000;
                c.f[0] = 0x4270_0000;
                let cc2 = FpuF32(c.f[12]).le(FpuF32(c.f[0]));
                abi::set_fcc(c, cc2);
                c.g[V0][0] = 0xca;
                if cc2 {
                    c.cycles += 2;
                    if abi::arg(c, 1) != 0xca {
                        c.cycles += 2;
                        this.off(0xc).write_fpu(c, FpuF32(c.f[12]));
                        abi::ret(c);
                        return;
                    }
                    c.cycles += 2;
                    this.off(8).write_fpu(c, FpuF32(c.f[12]));
                    abi::ret(c);
                    return;
                }
            }
        }
    } else {
        // param 0xc8/0xc9: field 0x3c/0x38, range [-1, 1]
        c.cycles += 5;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[0] = 0xbf80_0000;
        let cc = FpuF32(c.f[0]).le(FpuF32(c.f[12]));
        abi::set_fcc(c, cc);
        if cc {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            let cc2 = FpuF32(c.f[12]).le(FpuF32(c.f[0]));
            abi::set_fcc(c, cc2);
            c.g[V0][0] = 0xc9;
            if cc2 {
                c.cycles += 2;
                if abi::arg(c, 1) != 0xc9 {
                    c.cycles += 2;
                    this.off(0x3c).write_fpu(c, FpuF32(c.f[12]));
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
                this.off(0x38).write_fpu(c, FpuF32(c.f[12]));
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Texture manager.

/// `void TexMan_UnlockVram(TexMan *man, int index)` (0x367b60): mark the
/// texture's vram lock released.
pub fn tex_man_unlock_vram(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A3][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
    let t = c.g[V0][0] == c.g[V1][0];
    c.g[V1][0] = 0x1c;
    if t {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 13;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1f60));
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x1c));
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4350));
    c.g[A0][0] ^= 0x9;
    c.g[A3][0] = 0xffff_ffff_ffff_fffd;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1ff0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] &= c.g[A3][0];
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}

/// `void TexMan_EmitPath3Window(TexMan *man, void **cursor)` (0x368138):
/// append a path3 window packet and advance the cursor.
pub fn tex_man_emit_path3_window(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V0][0] = 0x450000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4e00));
    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V1][0] = 0x3000_0000;
    c.g[V1][0] |= 0x22;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] |= c.g[V1][0];
    c.sd(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0);
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void TexMan_SetTex1Filter(TexMan *man, int index, uint a2, uint a3,
/// uint t0, uint t1)` (0x369130): rewrite the tex1 filter word.
pub fn tex_man_set_tex1_filter(c: &mut Cpu) {
    c.cycles += 31;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = 0xffff_ffff_ffe7_0000;
    c.g[V1][0] |= 0xffff;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T0][0] &= 0x3;
    c.g[T2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[T0][0] = c.g[T0][0] << 19;
    c.g[T1][0] &= 0xfff;
    c.g[A0][0] = 0xffff_ffff_ffff_f000;
    c.g[A0][0] = c.g[A0][0] << 16;
    c.g[A0][0] |= 0xffff;
    c.g[A0][0] = c.g[A0][0] << 16;
    c.g[A0][0] |= 0xffff;
    c.g[V0][0] = c.ld((c.g[T2][0] as u32).wrapping_add(0x40));
    c.g[T1][0] = c.g[T1][0] << 32;
    c.g[A2][0] &= 0x7;
    c.g[A1][0] = 0xffff_ffff_ffff_fe3f;
    c.g[V0][0] &= c.g[V1][0];
    c.g[A2][0] = c.g[A2][0] << 6;
    c.g[V0][0] |= c.g[T0][0];
    c.g[A3][0] &= 0x1;
    c.g[V0][0] &= c.g[A0][0];
    c.g[V1][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V0][0] |= c.g[T1][0];
    c.g[A3][0] = c.g[A3][0] << 5;
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[A2][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A3][0];
    c.sd((c.g[T2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn mem_pool32_free_matches() {
        check(ssx3_ee::s10::f_003204a8, mem_pool32_free, 200, &|c, seed| {
            let mut r = Rng(0x1001 ^ seed as u64);
            scramble(c, &mut r);
            let pool = DATA + 0x100;
            let block = DATA + 0x200;
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(pool + 0x6000, r.next_u32());
        });
    }

    #[test]
    fn mem_pool64_free_matches() {
        check(ssx3_ee::s10::f_003204b8, mem_pool64_free, 200, &|c, seed| {
            let mut r = Rng(0x1002 ^ seed as u64);
            scramble(c, &mut r);
            let pool = DATA + 0x100;
            let block = DATA + 0x200;
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(pool + 0x30000 + 0x2000, r.next_u32());
        });
    }

    #[test]
    fn mem_pool128_free_matches() {
        check(ssx3_ee::s10::f_003204d0, mem_pool128_free, 200, &|c, seed| {
            let mut r = Rng(0x1003 ^ seed as u64);
            scramble(c, &mut r);
            let pool = DATA + 0x100;
            let block = DATA + 0x200;
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(pool + 0x20000 + 0x5800, r.next_u32());
        });
    }

    #[test]
    fn mem_pool256_free_matches() {
        check(ssx3_ee::s10::f_003204e8, mem_pool256_free, 200, &|c, seed| {
            let mut r = Rng(0x1004 ^ seed as u64);
            scramble(c, &mut r);
            let pool = DATA + 0x100;
            let block = DATA + 0x200;
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(pool + 0x20000, r.next_u32());
        });
    }

    #[test]
    fn mem_pool512_free_matches() {
        check(ssx3_ee::s10::f_00320500, mem_pool512_free, 200, &|c, seed| {
            let mut r = Rng(0x1005 ^ seed as u64);
            scramble(c, &mut r);
            let pool = DATA + 0x100;
            let block = DATA + 0x200;
            c.g[A0][0] = pool as u64;
            c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(pool + 0x20000, r.next_u32());
        });
    }

    #[test]
    fn math_ceil_log2_matches() {
        check(ssx3_ee::s10::f_00320b68, math_ceil_log2, 256, &|c, seed| {
            let mut r = Rng(0x1006 ^ seed as u64);
            scramble(c, &mut r);
            let x = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                3 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.g[A0][0] = x as u64;
        });
    }

    #[test]
    fn input_parser_pad_channel_operand_matches() {
        check_lohi(ssx3_ee::s10::f_00321ad8, input_parser_pad_channel_operand, 200, &|c, seed| {
            let mut r = Rng(0x1007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64);
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn input_vm_init_matches() {
        check(ssx3_ee::s10::f_00325250, input_vm_init, 200, &|c, seed| {
            let mut r = Rng(0x1008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn input_vm_get_reg_ptr_matches() {
        check(ssx3_ee::s10::f_003252e8, input_vm_get_reg_ptr, 200, &|c, seed| {
            let mut r = Rng(0x1009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 16) as u64;
        });
    }

    #[test]
    fn input_vm_read_builtin_matches() {
        check(ssx3_ee::s10::f_00325430, input_vm_read_builtin, 200, &|c, seed| {
            let mut r = Rng(0x100a ^ seed as u64);
            scramble(c, &mut r);
            let index = if seed % 3 == 0 { 0 } else { (seed % 16) as u32 };
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            let base = DATA + index * 4;
            c.bus.write32(base + 0x108, r.edge_f32());
        });
    }

    #[test]
    fn input_preproc_next_char_matches() {
        check(ssx3_ee::s10::f_00326478, input_preproc_next_char, 200, &|c, seed| {
            let mut r = Rng(0x100b ^ seed as u64);
            scramble(c, &mut r);
            let out = DATA;
            let st = DATA + 0x100;
            let start = DATA + 0x400;
            let len = (r.next_u32() % 16) as u32;
            c.g[A0][0] = out as u64;
            c.g[A1][0] = st as u64;
            c.bus.write32(st, start);
            c.bus.write32(st + 4, len);
            let ptr = if seed % 3 == 0 {
                start.wrapping_add(len) // cursor at end -> -1 branch
            } else {
                start // cursor inside -> byte branch
            };
            c.bus.write32(st + 8, ptr);
            c.bus.write8(ptr, (r.next_u32() & 0xff) as u8);
        });
    }

    #[test]
    fn pad_queue_flush_matches() {
        check_lohi(ssx3_ee::s10::f_00326c60, pad_queue_flush, 200, &|c, seed| {
            let mut r = Rng(0x100c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x2ee4, r.next_u32());
            c.bus.write32(DATA + 0x2ee0, r.next_u32());
        });
    }

    #[test]
    fn ai_fwd_diff_cache_ctor_matches() {
        check(ssx3_ee::s10::f_00327810, ai_fwd_diff_cache_ctor, 200, &|c, seed| {
            let mut r = Rng(0x100d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn ai_fwd_diff_cache_touch_matches() {
        check(ssx3_ee::s10::f_00327ac0, ai_fwd_diff_cache_touch, 200, &|c, seed| {
            let mut r = Rng(0x100e ^ seed as u64);
            scramble(c, &mut r);
            let node = DATA + 0x100;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = node as u64;
            let head = DATA + 0x200;
            let next = DATA + 0x300;
            match seed % 3 {
                0 => c.bus.write32(DATA + 4, node),
                1 => {
                    c.bus.write32(DATA + 4, head);
                    c.bus.write32(head + 8, node);
                }
                _ => {
                    c.bus.write32(DATA + 4, head);
                    c.bus.write32(head + 8, next);
                    c.bus.write32(node + 4, DATA + 0x400);
                    c.bus.write32(node + 8, DATA + 0x500);
                    c.bus.write32(next + 4, r.next_u32());
                }
            }
        });
    }

    #[test]
    fn sphere_tree_get_half_leaf_radius_matches() {
        check_lohi(ssx3_ee::s10::f_0032c5a8, sphere_tree_get_half_leaf_radius, 200, &|c, seed| {
            let mut r = Rng(0x100f ^ seed as u64);
            scramble(c, &mut r);
            let leaf = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x98, leaf);
            let idx = seed % 8;
            c.bus.write32(leaf + 0xc, idx);
            c.bus.write32(leaf + 0x20, DATA);
            c.bus.write32(DATA + 0x90, r.edge_f32());
            c.bus.write32(DATA + idx * 0xc, r.edge_f32());
        });
    }

    /// Setup for the three `AnimObject_Step*` functions: random edge floats on
    /// the parameter, bounds and rate fields.
    fn anim_step_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        for off in [0x10u32, 0x14, 0x18, 0x1c] {
            c.bus.write32(DATA + off, r.edge_f32());
        }
        c.bus.write32(DATA + 8, r.next_u32());
    }

    #[test]
    fn anim_object_step_loop_matches() {
        check(ssx3_ee::s11::f_00341e48, anim_object_step_loop, 300, &|c, seed| {
            anim_step_setup(c, seed, 0x341e48);
        });
    }

    #[test]
    fn anim_object_step_ping_pong_matches() {
        check(ssx3_ee::s11::f_00341ec0, anim_object_step_ping_pong, 300, &|c, seed| {
            anim_step_setup(c, seed, 0x341ec0);
        });
    }

    #[test]
    fn anim_object_step_clamp_matches() {
        check(ssx3_ee::s11::f_00341f38, anim_object_step_clamp, 300, &|c, seed| {
            anim_step_setup(c, seed, 0x341f38);
        });
    }

    #[test]
    fn boost_modifier_ctor_matches() {
        check(ssx3_ee::s11::f_00342768, boost_modifier_ctor, 200, &|c, seed| {
            let mut r = Rng(0x1010 ^ seed as u64);
            scramble(c, &mut r);
            let desc = DATA + 0x100;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = desc as u64;
            c.g[A2][0] = (DATA as u64 + 0x200) | ((r.next_u32() as u64) << 32);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 0x10, r.next_u32());
            c.bus.write16(desc + 4, (r.next_u32() & 0xfffe) as u16 | (seed as u16 & 1));
            for off in [8u32, 0xc, 0x10, 0x14, 0x18, 0x1c] {
                c.bus.write32(desc + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn modifier_list_push_front_matches() {
        check(ssx3_ee::s11::f_00345700, modifier_list_push_front, 200, &|c, seed| {
            let mut r = Rng(0x1011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
            let head = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(DATA, head);
        });
    }

    #[test]
    fn modifier_list_unlink_matches() {
        check(ssx3_ee::s11::f_00345760, modifier_list_unlink, 200, &|c, seed| {
            let mut r = Rng(0x1012 ^ seed as u64);
            scramble(c, &mut r);
            let m = DATA;
            let prev = DATA + 0x100;
            let next = DATA + 0x200;
            c.g[A0][0] = m as u64;
            c.bus.write32(m, if seed % 4 >= 2 { prev } else { 0 });
            c.bus.write32(m + 4, if seed % 2 == 0 { 0 } else { next });
            c.bus.write32(prev + 4, r.next_u32());
            c.bus.write32(next, r.next_u32());
        });
    }

    #[test]
    fn particle_modifier_base_ctor_matches() {
        check(ssx3_ee::s11::f_00345798, particle_modifier_base_ctor, 200, &|c, seed| {
            let mut r = Rng(0x1013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0xf04, r.next_u32());
        });
    }

    #[test]
    fn c_flag_manager_group_init_key_matches() {
        check(ssx3_ee::s11::f_0034af20, c_flag_manager_group_init_key, 200, &|c, seed| {
            let mut r = Rng(0x1014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x58, r.next_u32());
            c.bus.write32(DATA + 0x5c, r.next_u32());
        });
    }

    #[test]
    fn fence_add_impulse_matches() {
        check(ssx3_ee::s11::f_0034d6f0, fence_add_impulse, 300, &|c, seed| {
            let mut r = Rng(0x1015 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[GP][0] = GAME_GP as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(DATA + 0x30, r.edge_f32());
            c.bus.write32(DATA + 0x34, r.edge_f32());
            c.bus.write32(GAME_GP - 0x2ab4, r.edge_f32());
            c.bus.write32(GAME_GP - 0x2ab0, r.edge_f32());
            c.bus.write32(GAME_GP - 0x2aac, r.edge_f32());
        });
    }

    #[test]
    fn anim_object_get_node_intersect_matrix_matches() {
        check_lohi(ssx3_ee::s11::f_0034e320, anim_object_get_node_intersect_matrix, 200, &|c, seed| {
            let mut r = Rng(0x1016 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.bus.write32(DATA + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            c.bus.write32(DATA + 0x100 + 0x90, r.next_u32());
        });
    }

    #[test]
    fn instances_man_alloc_matches() {
        check_lohi(ssx3_ee::s11::f_00351170, instances_man_alloc, 200, &|c, seed| {
            let mut r = Rng(0x1017 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA, (r.next_u32() & 0xff) as u8);
            c.bus.write32(DATA + 0xc, r.next_u32());
            let slot = if seed % 3 == 0 { 0 } else { DATA + 0x100 };
            c.bus.write32(DATA + 8, slot);
            c.bus.write32(slot, r.next_u32());
            c.bus.write32(slot + 8, r.next_u32());
            c.bus.write8(slot + 0x78, (r.next_u32() & 0xff) as u8);
        });
    }

    #[test]
    fn instances_man_free_matches() {
        check(ssx3_ee::s11::f_00351260, instances_man_free, 200, &|c, seed| {
            let mut r = Rng(0x1018 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA + 8, r.next_u32());
        });
    }

    #[test]
    fn instances_man_resolve_handle_matches() {
        check_lohi(ssx3_ee::s11::f_003512c0, instances_man_resolve_handle, 200, &|c, seed| {
            let mut r = Rng(0x1019 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 4) as u32;
            let handle = r.next_u32() & 0xff;
            let stored = if seed % 3 == 0 { handle } else { (handle + 1) & 0xff };
            c.g[A1][0] = ((index << 8) | handle) as u64;
            c.bus.write32(DATA, stored);
            c.bus.write32(DATA + 0xc, DATA + 0x200);
            let p = DATA + 0x200 + index * 0xb0;
            c.bus.write32(p + 8, r.next_u32());
            c.bus.write8(p + 0x78, (r.next_u32() & 0xff) as u8);
        });
    }

    #[test]
    fn intersect_info_set_source_matches() {
        check(ssx3_ee::s11::f_00351398, intersect_info_set_source, 200, &|c, seed| {
            let mut r = Rng(0x101a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
            for i in 0..6u32 {
                c.bus.write32(DATA + 0x100 + i * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn intersect_info_get_duration_matches() {
        check(ssx3_ee::s11::f_00351508, intersect_info_get_duration, 200, &|c, seed| {
            let mut r = Rng(0x101b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let action = DATA + 0x100;
            c.bus.write32(DATA + 0x40, action);
            let (cur, target) = match seed % 4 {
                0 => (0, 0),
                1 => (DATA + 0x200, 0),
                2 => (DATA + 0x200, DATA + 0x300),
                _ => (DATA + 0x200, DATA + 0x300),
            };
            c.bus.write32(action + 0x20, cur);
            c.bus.write32(cur, target);
            let value = if seed % 4 == 2 { 0 } else { DATA + 0x400 };
            c.bus.write32(target + 4, value);
            c.bus.write32(DATA + 0x400 + 0x10, r.edge_f32());
        });
    }

    #[test]
    fn modifier_block_ctor_matches() {
        check(ssx3_ee::s11::f_00352aa8, modifier_block_ctor, 200, &|c, seed| {
            let mut r = Rng(0x101c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..8u32 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn sky_box_ctor_matches() {
        check(ssx3_ee::s11::f_00353ac0, sky_box_ctor, 200, &|c, seed| {
            let mut r = Rng(0x101d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..8u32 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn flex_bridge_springs_ctor_matches() {
        check(ssx3_ee::s11::f_00353da8, flex_bridge_springs_ctor, 200, &|c, seed| {
            let mut r = Rng(0x101e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x24, r.next_u32());
        });
    }

    #[test]
    fn process_list_add_matches() {
        check_lohi(ssx3_ee::s11::f_00354c08, process_list_add, 200, &|c, seed| {
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
        });
    }

    #[test]
    fn process_list_remove_matches() {
        check(ssx3_ee::s11::f_00354e48, process_list_remove, 200, &|c, seed| {
            let mut r = Rng(0x1020 ^ seed as u64);
            scramble(c, &mut r);
            let entry = DATA;
            let prev = DATA + 0x100;
            let next = DATA + 0x200;
            c.g[A1][0] = entry as u64;
            c.bus.write32(entry + 4, if seed % 4 >= 2 { prev } else { 0 });
            c.bus.write32(entry + 8, if seed % 2 == 0 { 0 } else { next });
            c.bus.write32(prev + 4, r.next_u32());
            c.bus.write32(next + 8, r.next_u32());
        });
    }

    #[test]
    fn process_list_first_matches() {
        check_lohi(ssx3_ee::s11::f_00354ea8, process_list_first, 200, &|c, seed| {
            let mut r = Rng(0x1021 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            let slot = base + index * 0x44;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let v = if seed % 3 == 0 { slot + 0x14 } else { r.next_u32() };
            c.bus.write32(slot + 8, v);
        });
    }

    #[test]
    fn process_list_last_matches() {
        check_lohi(ssx3_ee::s11::f_00354ed0, process_list_last, 200, &|c, seed| {
            let mut r = Rng(0x1022 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            let slot = base + index * 0x44;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let v = if seed % 3 == 0 { slot + 4 } else { r.next_u32() };
            c.bus.write32(slot + 0x1c, v);
        });
    }

    #[test]
    fn process_list_prev_matches() {
        check_lohi(ssx3_ee::s11::f_00354f20, process_list_prev, 200, &|c, seed| {
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
        });
    }

    #[test]
    fn process_list_pending_next_matches() {
        check_lohi(ssx3_ee::s11::f_00354f70, process_list_pending_next, 200, &|c, seed| {
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
        });
    }

    #[test]
    fn process_list_set_sorted_insert_matches() {
        check_lohi(ssx3_ee::s11::f_00355260, process_list_set_sorted_insert, 200, &|c, seed| {
            let mut r = Rng(0x1025 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            c.bus.write32(base + index * 0x44, r.next_u32());
        });
    }

    #[test]
    fn multi_particle_add_instance_matches() {
        check(ssx3_ee::s11::f_003581b8, multi_particle_add_instance, 200, &|c, seed| {
            let mut r = Rng(0x1026 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((DATA + 0x100) as u64) | ((r.next_u32() as u64) << 32);
            let cap = 4 + (seed % 4);
            let count = if seed % 3 == 0 { cap } else { seed % 4 };
            c.bus.write32(DATA, cap);
            c.bus.write32(DATA + 4, count);
            c.bus.write32(DATA + 0xc, DATA + 0x200);
        });
    }

    #[test]
    fn spline_modifier_set_speed_matches() {
        check(ssx3_ee::s11::f_00359688, spline_modifier_set_speed, 200, &|c, seed| {
            let mut r = Rng(0x1027 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[GP][0] = GAME_GP as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(GAME_GP - 0x2a60, r.edge_f32());
        });
    }

    #[test]
    fn multi_spline_modifier_set_speed_matches() {
        check(ssx3_ee::s11::f_0035a550, multi_spline_modifier_set_speed, 200, &|c, seed| {
            let mut r = Rng(0x1028 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[GP][0] = GAME_GP as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(GAME_GP - 0x2a28, r.edge_f32());
        });
    }

    #[test]
    fn rail_modifier_list_push_front_matches() {
        check(ssx3_ee::s11::f_0035b670, rail_modifier_list_push_front, 200, &|c, seed| {
            let mut r = Rng(0x1029 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((DATA + 0x100) as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
        });
    }

    #[test]
    fn rail_modifier_list_unlink_matches() {
        check(ssx3_ee::s11::f_0035b6d0, rail_modifier_list_unlink, 200, &|c, seed| {
            let mut r = Rng(0x102a ^ seed as u64);
            scramble(c, &mut r);
            let m = DATA;
            let prev = DATA + 0x100;
            let next = DATA + 0x200;
            c.g[A0][0] = m as u64;
            c.bus.write32(m, if seed % 4 >= 2 { prev } else { 0 });
            c.bus.write32(m + 4, if seed % 2 == 0 { 0 } else { next });
            c.bus.write32(prev + 4, r.next_u32());
            c.bus.write32(next, r.next_u32());
        });
    }

    #[test]
    fn uv_scroll_modifier_ctor_matches() {
        check(ssx3_ee::s12::f_0035f6e8, uv_scroll_modifier_ctor, 200, &|c, seed| {
            let mut r = Rng(0x102b ^ seed as u64);
            scramble(c, &mut r);
            let desc = DATA + 0x100;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = desc as u64;
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 0x10, r.next_u32());
            for off in [4u32, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
                c.bus.write32(desc + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn uv_scroll_modifier_set_param_matches() {
        check(ssx3_ee::s12::f_0035fb30, uv_scroll_modifier_set_param, 300, &|c, seed| {
            let mut r = Rng(0x102c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let p = match seed % 8 {
                0 => 0xc8,
                1 => 0xc9,
                2 => 0xca,
                3 => 0xcb,
                4 => 0xcc,
                5 => 0xcd,
                _ => 0xc0 + (seed % 4),
            };
            c.g[A1][0] = p as u64;
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn tex_man_unlock_vram_matches() {
        check_lohi(ssx3_ee::s12::f_00367b60, tex_man_unlock_vram, 200, &|c, seed| {
            let mut r = Rng(0x102d ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let p = DATA + 0x300;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + index * 4 + 8, p);
            c.bus.write32(p + 0x28, if seed % 3 == 0 { 0xffff_ffff } else { r.next_u32() });
            c.bus.write32(DATA + 0xc, if seed % 2 == 0 { 9 } else { r.next_u32() });
            let idx30 = seed % 4;
            c.bus.write32(DATA + 0x30, idx30);
            let v0 = DATA + if seed % 2 == 0 { 0x1f60 } else { 0x4350 };
            let q = DATA + 0x400;
            c.bus.write32(v0 + 0x1ff0, q);
            c.bus.write32(q + idx30 * 0x1c, r.next_u32());
        });
    }

    #[test]
    fn tex_man_emit_path3_window_matches() {
        check(ssx3_ee::s12::f_00368138, tex_man_emit_path3_window, 200, &|c, seed| {
            let mut r = Rng(0x102e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write64(DATA + 0x100, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
        });
    }

    #[test]
    fn tex_man_set_tex1_filter_matches() {
        check(ssx3_ee::s12::f_00369130, tex_man_set_tex1_filter, 200, &|c, seed| {
            let mut r = Rng(0x102f ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let t2 = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.bus.write32(DATA + index * 4 + 8, t2);
            c.bus.write64(t2 + 0x40, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
        });
    }
}
