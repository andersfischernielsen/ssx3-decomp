//! Staging wave 207 (oc256): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_e508, world_painter_get_weather18, "WorldPainter_GetWeather18"),
    (0x0037_4ae0, bezier_patch_cache_mark_used2, "BezierPatchCache_MarkUsed2"),
    (0x0034_c7f8, flag_manager_render, "cFlagManager_Render"),
    (0x002e_ee88, world_painter_get_light_glow20, "WorldPainter_GetLightGlow20"),
    (0x002e_e780, world_painter_get_weather60, "WorldPainter_GetWeather60"),
    (0x0015_23e8, be_test_char_profile_flag, "BE_TestCharProfileFlag"),
    (0x0036_5fa8, vram_heap_nodes_from_scratchpad, "VramHeap_NodesFromScratchpad"),
    (0x0010_4cf8, rider_anim_v04, "RiderAnim_v04"),
    (0x0012_d848, rider_wipeout_sub_update_recover, "RiderWipeoutSub_UpdateRecover"),
];

// ---------------------------------------------------------------------------
// World painter getters (lensfx.cpp / renderstateman.cpp).

/// The world painter's per-view table (`0x4fa370`, 0xf0 bytes per view).
const WP_TABLE_207: u32 = 0x004f_a370;
/// Stride of one view's entry in [`WP_TABLE_207`].
const WP_STRIDE_207: u64 = 0xf0;

/// The plain float getters of the 0x2ee*** family: `entry = table + index *
/// 0xf0`, `holder = entry->field`, `obj = *holder`, call the vtable slot
/// `vt_off` (`{delta, fn}`) with `obj + delta` and return the float its
/// result points at (in f0).
fn world_painter_float_getter_207(c: &mut Cpu, table_off: u32, vt_off: u32, ret_to: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = WP_STRIDE_207;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WP_TABLE_207);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(r32(c, A0)).off(table_off).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(vt_off).lh(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(vt_off + 4).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret_to)?;
        let p = r32(c, V0);
        c.lwc1(0, p);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather18(int index)` (0x2ee508): the weather
/// block's value (vtable slot 0x168 of the weather holder at `entry+0x20`)
/// faded by the view's weather fade at `entry+0x24`:
/// `*get() * (1.0 - fade)`. The entry pointer lives in s0 across the call.
pub fn world_painter_get_weather18(c: &mut Cpu) {
    const RET: u32 = 0x002e_e544;
    /// The view's weather holder.
    const WEATHER: u32 = 0x20;
    /// The view's weather fade (0 = full weather).
    const FADE: u32 = 0x24;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = WP_STRIDE_207;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x50_0000;
        c.mult(0, S0, A0, V1); // s0 = index * 0xf0
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WP_TABLE_207);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(r32(c, S0)).off(WEATHER).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        c.g[A1][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(0x168).lh(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x16c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, target, RET)?;
        let entry = r32(c, S0);
        c.lwc1(2, entry.wrapping_add(FADE));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0); // f0 = 1.0
        let p = r32(c, V0);
        c.lwc1(1, p);
        c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[2])).0;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.f[0] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetLightGlow20(int index)` (0x2eee88): the light
/// glow holder (`entry+0x8`), vtable slot 0x80.
pub fn world_painter_get_light_glow20(c: &mut Cpu) {
    world_painter_float_getter_207(c, 0x8, 0x80, 0x002e_eec0);
}

/// `float WorldPainter_GetWeather60(int index)` (0x2ee780): the weather
/// holder (`entry+0x20`), vtable slot 0x1b0.
pub fn world_painter_get_weather60(c: &mut Cpu) {
    world_painter_float_getter_207(c, 0x20, 0x1b0, 0x002e_e7b8);
}

// ---------------------------------------------------------------------------
// BezierPatchCache_MarkUsed2 (particle.cpp).

/// `void BezierPatchCache_MarkUsed2(cache, int count, Item* items, int lod,
/// int bit)` (0x374ae0): for each of the `count` items (stride 0x14) whose
/// patch index (`short` at +0xc) is not negative, OR `(1 << bit) | 0xc` into
/// the flags word (+4) of that patch's 12-byte record in the cache's list
/// for `lod` (`cache+0x18 + lod*4`).
pub fn bezier_patch_cache_mark_used2(c: &mut Cpu) {
    const HEAD: u32 = 0x0037_4b00;
    /// The per-LOD record lists.
    const LISTS: u32 = 0x18;
    /// An item's patch index (`short`, negative = none).
    const ITEM_PATCH: u32 = 0xc;
    const ITEM_SIZE: u32 = 0x14;
    /// A record's flags word.
    const REC_FLAGS: u32 = 4;
    c.cycles += 4;
    c.g[V0][0] = sx(1u32 << (r32(c, T0) & 31));
    let none = (c.g[A1][0] as i64) <= 0;
    c.g[T0][0] = c.g[V0][0] | 0xc;
    if !none {
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, A3) << 2);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(LISTS));
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[T1][0] = 0xc;
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, A2)).off(ITEM_PATCH).lh(c);
            let skip = (c.g[A0][0] as i64) < 0;
            c.mult(0, A0, A0, T1); // a0 = patch * 12
            if !skip {
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, A3)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = Addr(r32(c, V1)).off(REC_FLAGS).lw(c);
                c.g[V0][0] |= c.g[T0][0];
                c.sw(r32(c, V1).wrapping_add(REC_FLAGS), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
            let more = c.g[A1][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(ITEM_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// cFlagManager_Render (flagset.cpp).

/// `cFlagManager_GroupRender` (0x34b9b0).
const FLAG_MANAGER_GROUP_RENDER_207: Callee = Callee {
    addr: 0x0034_b9b0,
    f: ee::s11::cFlagManager_GroupRender,
};

/// `void cFlagManager_Render(cFlagManager* self)` (0x34c7f8): render each of
/// the 15 flag groups (`self+0x20`, 0x188 bytes each) that has models
/// (`group+0x5c` non-zero). s0 walks the groups, s1 counts 14 down to -1.
pub fn flag_manager_render(c: &mut Cpu) {
    const HEAD: u32 = 0x0034_c810;
    const RET: u32 = 0x0034_c824;
    const GROUPS: u32 = 0x20;
    const GROUP_SIZE: u32 = 0x188;
    /// A group's model count.
    const GROUP_MODELS: u32 = 0x5c;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(GROUPS));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xe;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).off(GROUP_MODELS).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                FLAG_MANAGER_GROUP_RENDER_207.call(c, RET)?;
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(GROUP_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE_TestCharProfileFlag (bedbprofile.cpp).

/// The three profile flag masks (`D_0045A7C0`), indexed by kind.
const CHAR_PROFILE_MASKS_207: u32 = 0x0045_a7c0;

/// `int BE_TestCharProfileFlag(CharProfile* self, int kind)` (0x1523e8):
/// whether the profile's flag word (`+0xacc`) has any bit of mask `kind`.
/// The masks are copied to the stack first (`ldl/ldr` + `lw`, then
/// `sdl/sdr` + `sw`), as the original's struct copy does.
pub fn be_test_char_profile_flag(c: &mut Cpu) {
    const FLAGS: u32 = 0xacc;
    c.cycles += 17;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x46_0000;
    c.g[T0][0] = sx(CHAR_PROFILE_MASKS_207);
    let first_two = Addr(CHAR_PROFILE_MASKS_207).read_u64_unaligned(c);
    c.g[V1][0] = first_two;
    c.g[A2][0] = Addr(CHAR_PROFILE_MASKS_207).off(8).lw(c);
    let copy = frame.at(c, 0);
    Addr(copy).write_u64_unaligned(c, first_two);
    c.sw(copy.wrapping_add(8), c.g[A2][0]);
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[A2][0] = sx(copy.wrapping_add(r32(c, A1)));
    c.g[V0][0] = Addr(r32(c, A0)).off(FLAGS).lw(c);
    c.g[V1][0] = Addr(r32(c, A2)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] & c.g[V1][0]) != 0) as u64;
    frame.close(c);
}

// ---------------------------------------------------------------------------
// VramHeap_NodesFromScratchpad (railmodifier.cpp).

/// `InvalidDCache` (libkernel; called through the table).
const INVALID_DCACHE_207: u32 = 0x0042_47d8;
/// `SprStream_Download` (0x38f738).
const SPR_STREAM_DOWNLOAD_207: Callee = Callee {
    addr: 0x0038_f738,
    f: ee::s12::SprStream_Download,
};

/// `void VramHeap_NodesFromScratchpad(VramHeap* self)` (0x365fa8): bring the
/// heap's node block back from the scratchpad: invalidate the data cache
/// over `self..self+0x1fef`, DMA 0x1ff0 bytes from the scratchpad stream to
/// `self` (flags 2: wait), then store `self` in the block's trailing link
/// word (`+0x1ff0`).
pub fn vram_heap_nodes_from_scratchpad(c: &mut Cpu) {
    const BLOCK: u32 = 0x1ff0;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(BLOCK - 1));
        call_ptr(c, INVALID_DCACHE_207, 0x0036_5fc0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = BLOCK as u64;
        c.g[A2][0] = 0x2;
        SPR_STREAM_DOWNLOAD_207.call(c, 0x0036_5fd0)?;
        let heap = r32(c, S0);
        c.sw(heap.wrapping_add(BLOCK), c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_v04 (seg_1218.cpp).

/// `AIAnimEventMap_PickAnim` (0x311710; called through the table).
const AI_ANIM_EVENT_MAP_PICK_ANIM_207: u32 = 0x0031_1710;

/// `RiderAnim_v04(RiderAnim* self, int event)` (0x104cf8): pick the
/// animation for `event` from the global event map (`gp+0xd0c`) with the
/// rider's filter flags: the board's (`self+0x64` → `+0xcd8`) if there is a
/// board, else the rider's (`self+0x60` → `+0x364`), else 0. Returns what
/// `AIAnimEventMap_PickAnim(map, event, flags)` returns.
pub fn rider_anim_v04(c: &mut Cpu) {
    const RET: u32 = 0x0010_4d28;
    const BOARD: u32 = 0x64;
    const BOARD_FLAGS: u32 = 0xcd8;
    const RIDER: u32 = 0x60;
    const RIDER_FLAGS: u32 = 0x364;
    /// `gp`-relative: the AI animation event map.
    const GP_EVENT_MAP: u32 = 0xd0c;
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).off(BOARD).lw(c);
        c.g[A2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A2][0] = Addr(r32(c, V0)).off(BOARD_FLAGS).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, A0)).off(RIDER).lw(c);
            if c.g[A0][0] != 0 {
                c.g[A2][0] = Addr(r32(c, A0)).off(RIDER_FLAGS).lw(c);
            }
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(r32(c, GP)).off(GP_EVENT_MAP).lw(c);
        call_ptr(c, AI_ANIM_EVENT_MAP_PICK_ANIM_207, RET)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdateRecover (ai.cpp).

/// `SSXAudioSystem_GetInstance` (0x28b180; through the table).
const SSX_AUDIO_GET_INSTANCE_207: u32 = 0x0028_b180;
/// `SSXAudioSystem_PlayLandingSound` (0x2948d0; through the table).
const SSX_AUDIO_PLAY_LANDING_SOUND_207: u32 = 0x0029_48d0;
/// `AnimController_GetLayerProgress` (0x312ab0; through the table).
const ANIM_CONTROLLER_GET_LAYER_PROGRESS_207: u32 = 0x0031_2ab0;
/// `AnimController_IsLayerAnimFinished` (0x312ae8; through the table).
const ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED_207: u32 = 0x0031_2ae8;
/// `AnimController_PlayAnim` (0x3128e8; through the table).
const ANIM_CONTROLLER_PLAY_ANIM_207: u32 = 0x0031_28e8;
const RIDER_SET_SUB_STATE_207: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::Rider_SetSubState,
};
const RIDER_SET_MAIN_STATE_207: Callee = Callee {
    addr: 0x0011_fe78,
    f: ee::s00::Rider_SetMainState,
};
const RIDER_METRIX_INTERRUPT_TRICK_207: Callee = Callee {
    addr: 0x0011_9e38,
    f: ee::s00::RiderMetrix_InterruptTrick,
};

/// `void RiderWipeoutSub_UpdateRecover(RiderWipeoutSub* self)` (0x12d848):
/// the wipeout's recovery phase.
///
/// - On the first update (`self+0x54` set): clear the flag, pass the length
///   of the impact vector (`self+0x60`, VU0 `sqrt` of the xyz dot product)
///   to the rider's vtable slot 0x88, then play the landing sound
///   (`SSXAudioSystem_PlayLandingSound(instance, rider, 0.0)`).
/// - A human rider (owner `+0x77c` → `+0x30` zero) feeds
///   `2 * (1 - progress of anim layer 2)` to vtable slot 0x90.
/// - When layer 2's animation has finished: a human rider plays anim 5 and
///   goes to sub-state 0 / main state 0; otherwise the trick is interrupted
///   (`RiderMetrix_InterruptTrick(metrix, +0x320 != +0x324, 0)`), anim 0x11f
///   plays and the rider goes to sub-state 5 / main state 1.
pub fn rider_wipeout_sub_update_recover(c: &mut Cpu) {
    /// The "recovery just started" flag.
    const STARTED: u32 = 0x54;
    /// The impact vector (xyz).
    const IMPACT: u32 = 0x60;
    const RIDER: u32 = 0x80;
    // Rider fields.
    const VTABLE: u32 = 0x6c0;
    const OWNER: u32 = 0x77c;
    const ANIM_CTL: u32 = 0x784;
    const METRIX: u32 = 0x790;
    /// Owner: non-zero for a computer rider.
    const OWNER_AI: u32 = 0x30;
    /// -1.0f (anim blend).
    const MINUS_ONE: u64 = 0xffff_ffff_bf80_0000;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2)).off(STARTED).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 24;
            let me = r32(c, S2);
            c.sw(me.wrapping_add(STARTED), 0);
            c.g[A1][0] = Addr(me).off(RIDER).lw(c);
            c.lqc2(3, me.wrapping_add(IMPACT));
            // |impact| (xyz) into f12.
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(V0, 22);
            c.mtc1(V0, 12);
            c.g[V0][0] = Addr(r32(c, A1)).off(VTABLE).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x88).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x8c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0012_d8b0)?;
            call_ptr(c, SSX_AUDIO_GET_INSTANCE_207, 0x0012_d8b8)?;
            c.g[A1][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
            c.mtc1(0, 12); // f12 = 0.0
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, SSX_AUDIO_PLAY_LANDING_SOUND_207, 0x0012_d8c8)?;
        }
        c.g[A2][0] = Addr(r32(c, S2)).off(RIDER).lw(c);

        // A human rider: feed the recovery progress to slot 0x90.
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A2)).off(OWNER).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(OWNER_AI).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 14;
            let rider = r32(c, A2);
            c.g[S1][0] = Addr(rider).off(VTABLE).lw(c);
            c.g[A1][0] = 0x2;
            c.g[A0][0] = Addr(rider).off(ANIM_CTL).lw(c);
            c.g[S0][0] = Addr(r32(c, S1)).off(0x90).lh(c);
            c.g[S0][0] = sx(rider.wrapping_add(r32(c, S0)));
            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_PROGRESS_207, 0x0012_d8f4)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 12); // f12 = 1.0
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr(r32(c, S1)).off(0x94).lw(c);
            c.f[12] = (FpuF32(c.f[12]) - FpuF32(c.f[0])).0;
            let target = r32(c, V0);
            c.f[12] = (FpuF32(c.f[12]) + FpuF32(c.f[12])).0;
            call_ptr(c, target, 0x0012_d910)?;
        }
        c.g[V0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);

        // Layer 2 finished: leave the wipeout.
        c.cycles += 5;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = Addr(r32(c, V0)).off(ANIM_CTL).lw(c);
        call_ptr(c, ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED_207, 0x0012_d920)?;
        let finished = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if finished {
            c.cycles += 5;
            c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(OWNER).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(OWNER_AI).lw(c);
            if c.g[V1][0] == 0 {
                c.cycles += 14;
                c.g[A0][0] = Addr(r32(c, A0)).off(ANIM_CTL).lw(c);
                c.g[A1][0] = 0x5;
                c.g[AT][0] = MINUS_ONE;
                c.mtc1(AT, 12);
                c.g[A2][0] = 0;
                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM_207, 0x0012_d954)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0;
                RIDER_SET_SUB_STATE_207.call(c, 0x0012_d960)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0;
                RIDER_SET_MAIN_STATE_207.call(c, 0x0012_d96c)?;
            } else {
                let rider = r32(c, A0);
                c.g[V0][0] = Addr(rider).off(0x324).lw(c);
                c.cycles += 20;
                c.g[A2][0] = 0;
                c.g[A1][0] = Addr(rider).off(0x320).lw(c);
                c.g[A0][0] = Addr(rider).off(METRIX).lw(c);
                c.g[A1][0] ^= c.g[V0][0];
                c.g[A1][0] = (c.g[A1][0] != 0) as u64;
                RIDER_METRIX_INTERRUPT_TRICK_207.call(c, 0x0012_d98c)?;
                c.g[V0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0x11f;
                c.g[AT][0] = MINUS_ONE;
                c.mtc1(AT, 12);
                c.g[A2][0] = 0;
                c.g[A0][0] = Addr(r32(c, V0)).off(ANIM_CTL).lw(c);
                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM_207, 0x0012_d9a8)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0x5;
                RIDER_SET_SUB_STATE_207.call(c, 0x0012_d9b4)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0x1;
                RIDER_SET_MAIN_STATE_207.call(c, 0x0012_d9c0)?;
            }
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{self, covered, FAKE};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};

    // ------------------------------------------------------- world painter

    /// A world-painter getter call for view `seed % 4`: the holder at
    /// `entry + table_off` → object → vtable whose slot `vt_off` is a fake
    /// (logs, sometimes unwinds); random floats at every 16-byte slot the
    /// fake's result pointer can land on.
    fn wp_getter_setup_207(c: &mut Cpu, seed: u32, entry_pc: u32, table_off: u32, vt_off: u32) -> Rng {
        let mut r = fakes::machine(c, seed, entry_pc as u64, 1);
        c.pc = entry_pc;
        for i in 0..0x4000u32 {
            let v = r.edge_f32();
            c.bus.write32(DATA + i * 0x10, v);
        }
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = WP_TABLE_207.wrapping_add(index * 0xf0);
        let holder = fakes::ptr(&mut r);
        let obj = fakes::ptr(&mut r);
        let vt = fakes::ptr(&mut r);
        c.bus.write32(entry.wrapping_add(table_off), holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj.wrapping_add(4), vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt.wrapping_add(vt_off), delta as u16);
        c.bus.write32(vt.wrapping_add(vt_off).wrapping_add(4), FAKE);
        r
    }

    /// WorldPainter_GetWeather18: also a random (edge-case) fade at
    /// `entry+0x24`.
    #[test]
    fn world_painter_get_weather18_matches() {
        covered(ee::s09::f_002ee508__generated, world_painter_get_weather18, 200, &|c, seed| {
            let mut r = wp_getter_setup_207(c, seed, 0x002e_e508, 0x20, 0x168);
            let entry = WP_TABLE_207 + (seed % 4) * 0xf0;
            let fade = r.edge_f32();
            c.bus.write32(entry + 0x24, fade);
        });
    }

    #[test]
    fn world_painter_get_light_glow20_matches() {
        covered(ee::s09::f_002eee88__generated, world_painter_get_light_glow20, 200, &|c, seed| {
            wp_getter_setup_207(c, seed, 0x002e_ee88, 0x8, 0x80);
        });
    }

    #[test]
    fn world_painter_get_weather60_matches() {
        covered(ee::s09::f_002ee780__generated, world_painter_get_weather60, 200, &|c, seed| {
            wp_getter_setup_207(c, seed, 0x002e_e780, 0x20, 0x1b0);
        });
    }

    // ------------------------------------------------ bezier patch cache

    /// BezierPatchCache_MarkUsed2: count 0, negative or 1..8; items whose
    /// patch index is negative or 0..15; LOD 0..3; a random bit number
    /// (only the low 5 bits count); random record flags.
    #[test]
    fn bezier_patch_cache_mark_used2_matches() {
        const CACHE: u32 = DATA;
        const ITEMS: u32 = DATA + 0x1000;
        const LISTS: u32 = DATA + 0x2000;
        check_lohi(ee::s12::f_00374ae0__generated, bezier_patch_cache_mark_used2, 300, &|c, seed| {
            let mut r = Rng(0x0037_4ae0 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x0037_4ae0;
            c.g[A0][0] = sx(CACHE);
            c.g[A1][0] = match seed % 5 {
                0 => 0,
                1 => sx((-(1 + (r.next_u32() % 3) as i32)) as u32),
                _ => (1 + r.next_u32() % 8) as u64,
            };
            c.g[A2][0] = sx(ITEMS);
            c.g[A3][0] = (seed % 4) as u64;
            c.g[T0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            for i in 0..4 {
                c.bus.write32(CACHE + 0x18 + 4 * i, LISTS + 0x100 * i);
            }
            for i in 0..8 {
                let patch = if r.next_u32() % 3 == 0 {
                    0x8000 | r.next_u32() as u16
                } else {
                    (r.next_u32() % 16) as u16
                };
                c.bus.write16(ITEMS + 0x14 * i + 0xc, patch);
            }
            fill(c, &mut r, LISTS, 0x200);
        });
    }

    // ------------------------------------------------------ flag manager

    /// cFlagManager_Render: each of the 15 groups with or without models;
    /// the real GroupRender runs and returns at once (no flag manager at
    /// `gp+0x1154` in empty RAM).
    #[test]
    fn flag_manager_render_matches() {
        check(ee::s11::f_0034c7f8__generated, flag_manager_render, 200, &|c, seed| {
            let mut r = Rng(0x0034_c7f8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0034_c7f8;
            c.g[A0][0] = sx(DATA);
            for i in 0..15u32 {
                let models = if seed % 7 == 0 || r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(DATA + 0x20 + i * 0x188 + 0x5c, models);
            }
        });
    }

    // --------------------------------------------------- profile flags

    /// BE_TestCharProfileFlag: kinds 0..2, random masks and flag words
    /// (sometimes zero, sometimes exactly the mask).
    #[test]
    fn be_test_char_profile_flag_matches() {
        check(ee::s01::f_001523e8__generated, be_test_char_profile_flag, 200, &|c, seed| {
            let mut r = Rng(0x0015_23e8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x0015_23e8;
            let kind = seed % 3;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = kind as u64;
            let masks = [1 << (r.next_u32() % 32), r.next_u32(), r.next_u32() & 0xff];
            for (i, m) in masks.iter().enumerate() {
                c.bus.write32(CHAR_PROFILE_MASKS_207 + 4 * i as u32, *m);
            }
            let flags = match seed % 4 {
                0 => 0,
                1 => masks[kind as usize],
                2 => !masks[kind as usize],
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0xacc, flags);
        });
    }

    // ------------------------------------------------------- vram heap

    /// VramHeap_NodesFromScratchpad: the heap at a random 16-byte aligned
    /// spot, the scratchpad stream's pointer (`gp+0x2a94`) at a random
    /// quadword that leaves room for the 0x1ff0-byte block, random RAM and
    /// scratchpad. The real InvalidDCache and SprStream_Download run.
    #[test]
    fn vram_heap_nodes_from_scratchpad_matches() {
        check(ee::s12::f_00365fa8__generated, vram_heap_nodes_from_scratchpad, 100, &|c, seed| {
            let mut r = Rng(0x0036_5fa8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0036_5fa8;
            fill(c, &mut r, DATA, 0x1000);
            for i in 0..0x1000 {
                c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
            }
            let spr = 0x7000_0000 + (r.next_u32() % 0x200) * 0x10;
            c.bus.write32(GAME_GP + 0x2a94, spr);
            c.g[A0][0] = sx(DATA + (r.next_u32() % 0x100) * 0x10);
        });
    }

    // ------------------------------------------------------- rider anim

    /// RiderAnim_v04: a board (its flags), no board but a rider (its
    /// flags), or neither (0); the real AIAnimEventMap_PickAnim takes its
    /// single-choice path (`entry+2 == 1`) for events 0..3.
    #[test]
    fn rider_anim_v04_matches() {
        const ANIM: u32 = DATA;
        const BOARD: u32 = DATA + 0x1000;
        const RIDER: u32 = DATA + 0x2000;
        const MAP: u32 = DATA + 0x3000;
        check(ee::s00::f_00104cf8__generated, rider_anim_v04, 200, &|c, seed| {
            let mut r = Rng(0x0010_4cf8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0010_4cf8;
            c.g[A0][0] = sx(ANIM);
            c.g[A1][0] = (seed % 4) as u64;
            let (board, rider) = match seed % 3 {
                0 => (BOARD, 0),
                1 => (0, RIDER),
                _ => (0, 0),
            };
            c.bus.write32(ANIM + 0x64, board);
            c.bus.write32(ANIM + 0x60, rider);
            let v = r.next_u32();
            c.bus.write32(BOARD + 0xcd8, v);
            let v = r.next_u32();
            c.bus.write32(RIDER + 0x364, v);
            c.bus.write32(GAME_GP + 0xd0c, MAP);
            for i in 0..4 {
                let pick = (r.next_u32() % 8) as u16;
                c.bus.write16(MAP + 4 * i, pick);
                c.bus.write16(MAP + 4 * i + 2, 1);
            }
            for k in 0..8 {
                let v = r.next_u32();
                c.bus.write32(0x0044_9960 + 12 * k, v);
            }
        });
    }

    // ------------------------------------------------------ rider wipeout

    /// RiderWipeoutSub_UpdateRecover: first update or not (random impact
    /// vector), human or computer rider, layer 2 finished or not, equal or
    /// different +0x320/+0x324. The rider's vtable slots (0x88, 0x90, and
    /// 0x48/0x28 used by the landing sound) and the anim controller's slot
    /// are fakes (forced result 0x207: `AnimController_PlayAnim`'s early
    /// out). `PlayLandingSound` takes its replay path: the world's game
    /// module says replay (`*(module+0x28) == 1`), the race man has no
    /// riders (`FindRider` → 0) and no views (`FindViewInRange` → -1). The
    /// trick metrix is idle (`+0x70` set: `InterruptTrick` makes no calls).
    /// Owner states as in the board-press test (main >= 6, sub 0xe..0x15).
    #[test]
    fn rider_wipeout_sub_update_recover_matches() {
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const OWNER: u32 = DATA + 0x2000;
        const VT: u32 = DATA + 0x3000;
        const CTL: u32 = DATA + 0x4000;
        const CTL_VT: u32 = DATA + 0x5000;
        const SEQS: u32 = DATA + 0x6000;
        const SEQ: u32 = DATA + 0x6100;
        const METRIX: u32 = DATA + 0x7000;
        const WORLD: u32 = DATA + 0x8000;
        const MODULE: u32 = DATA + 0x8100;
        const REPLAY: u32 = DATA + 0x8200;
        const RACE_MAN: u32 = DATA + 0x8300;
        check(ee::s00::f_0012d848__generated, rider_wipeout_sub_update_recover, 240, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0012_d848, 6);
            c.pc = 0x0012_d848;
            c.g[A0][0] = sx(SELF);
            let started = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(SELF + 0x54, started);
            for k in 0..4 {
                let v = r.f32(10.0);
                c.bus.write32(SELF + 0x60 + 4 * k, v);
            }
            c.bus.write32(SELF + 0x80, RIDER);
            c.bus.write32(RIDER + 0x6c0, VT);
            for (i, off) in [0x88u32, 0x90, 0x48, 0x28].into_iter().enumerate() {
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(VT + off, delta as u16);
                c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
            }
            c.bus.write32(RIDER + 0x77c, OWNER);
            let ai = if (seed / 3) % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(OWNER + 0x30, ai);
            c.bus.write32(OWNER + 0xde0, 6 + r.next_u32() % 4);
            c.bus.write32(OWNER + 0xde4, 0xe + r.next_u32() % 8);
            c.bus.write32(RIDER + 0x784, CTL);
            c.bus.write32(CTL + 0x50, SEQS);
            c.bus.write32(SEQS + 0x14, SEQ);
            let (t, len) = (r.f32(4.0), r.f32(8.0) | 0x3f80_0000);
            c.bus.write32(SEQ + 0x8, t);
            c.bus.write32(SEQ + 0x10, len & 0x7fff_ffff);
            let finished = if (seed / 6) % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(SEQ + 0xc0, finished);
            c.bus.write32(CTL + 0x58, CTL_VT);
            c.bus.write16(CTL_VT + 0x20, 0x10);
            c.bus.write32(CTL_VT + 0x24, FAKE + 0x40);
            for n in 0..8 {
                c.bus.write32(fakes::LOG_RESULTS + 4 * n, 0x207);
            }
            let a = r.next_u32();
            let b = if (seed / 12) % 2 == 0 { a } else { r.next_u32() };
            c.bus.write32(RIDER + 0x320, a);
            c.bus.write32(RIDER + 0x324, b);
            c.bus.write32(RIDER + 0x790, METRIX);
            c.bus.write32(METRIX + 0x70, 1);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
            c.bus.write32(WORLD + 0x84, MODULE);
            c.bus.write32(MODULE + 0x28, REPLAY);
            c.bus.write32(REPLAY, 1);
            c.bus.write32(MODULE + 0xc, RACE_MAN);
        });
    }
}
